//! Phase 2: the signed-token deny matrix.
//!
//! Every row of the plan's Phase 2 TDD matrix is a test here. This is the
//! surface the mutation-testing policy says must have no surviving mutant, so
//! each gate is exercised independently: change one fact, watch one deny.

mod common;
use common::*;

use croft_admit::tier::Tier;
use croft_admit::token::{TokenError, TokenIssuer, TokenVerifier, VerifiedClaims};

const ISS: &str = "https://admit.croft.ing";
const TTL: u64 = 900; // 15 min
const LEEWAY: u64 = 30; // clock-skew tolerance
const NOW: u64 = 1_000_000; // fixed "now" for determinism

fn issuer(km: &KeyMaterial) -> TokenIssuer {
    TokenIssuer::new(&km.pkcs8_der, ISS, TTL)
}
fn verifier(km: &KeyMaterial) -> TokenVerifier {
    TokenVerifier::new(&km.public_raw, ISS, LEEWAY)
}

#[test]
fn valid_token_matching_id_admits_with_tier() {
    let km = generate_keypair();
    let ep = endpoint_from_seed(1);
    let tok = issuer(&km).mint(&ep, Tier::Broker, NOW);

    let got = verifier(&km).verify(&tok, &ep, NOW);

    assert_eq!(
        got,
        Ok(VerifiedClaims {
            endpoint: ep,
            tier: Tier::Broker
        })
    );
}

#[test]
fn coordination_tier_survives_the_round_trip() {
    // The tier claim must arrive intact; Phase 3 maps it to a bucket.
    let km = generate_keypair();
    let ep = endpoint_from_seed(2);
    let tok = issuer(&km).mint(&ep, Tier::Coordination, NOW);

    let got = verifier(&km).verify(&tok, &ep, NOW).unwrap();

    assert_eq!(got.tier, Tier::Coordination);
}

#[test]
fn valid_token_mismatched_id_denies_replay() {
    // The anti-replay hinge: a perfectly valid token presented from a different
    // (but also relay-authenticated) endpoint must be refused.
    let km = generate_keypair();
    let subject = endpoint_from_seed(1);
    let attacker = endpoint_from_seed(2);
    let tok = issuer(&km).mint(&subject, Tier::Broker, NOW);

    let got = verifier(&km).verify(&tok, &attacker, NOW);

    assert_eq!(got, Err(TokenError::IdMismatch));
}

#[test]
fn expired_token_denies() {
    let km = generate_keypair();
    let ep = endpoint_from_seed(1);
    let tok = issuer(&km).mint(&ep, Tier::Broker, NOW);

    // well past exp + leeway
    let later = NOW + TTL + LEEWAY + 1;
    assert_eq!(
        verifier(&km).verify(&tok, &ep, later),
        Err(TokenError::Expired)
    );
}

#[test]
fn token_valid_exactly_at_expiry_plus_leeway_boundary() {
    // now == exp + leeway is still valid; one second later is not. Pins the
    // boundary so a mutant that flips `>` to `>=` is caught.
    let km = generate_keypair();
    let ep = endpoint_from_seed(1);
    let tok = issuer(&km).mint(&ep, Tier::Broker, NOW);
    let exp = NOW + TTL;

    assert!(verifier(&km).verify(&tok, &ep, exp + LEEWAY).is_ok());
    assert_eq!(
        verifier(&km).verify(&tok, &ep, exp + LEEWAY + 1),
        Err(TokenError::Expired)
    );
}

#[test]
fn token_from_the_future_beyond_leeway_denies() {
    let km = generate_keypair();
    let ep = endpoint_from_seed(1);
    // Issued at a future NOW; verifier's clock is behind by more than leeway.
    let tok = issuer(&km).mint(&ep, Tier::Broker, NOW);
    let earlier = NOW - LEEWAY - 1;

    assert_eq!(
        verifier(&km).verify(&tok, &ep, earlier),
        Err(TokenError::NotYetValid)
    );
}

#[test]
fn token_within_negative_leeway_still_admits() {
    // Verifier's clock behind issuer by exactly leeway -> still fine.
    let km = generate_keypair();
    let ep = endpoint_from_seed(1);
    let tok = issuer(&km).mint(&ep, Tier::Broker, NOW);

    assert!(verifier(&km).verify(&tok, &ep, NOW - LEEWAY).is_ok());
}

#[test]
fn wrong_issuer_key_denies() {
    // Token signed by one key, verified with another public key -> signature
    // failure. Stolen-token-without-the-key is worthless.
    let signer = generate_keypair();
    let other = generate_keypair();
    let ep = endpoint_from_seed(1);
    let tok = issuer(&signer).mint(&ep, Tier::Broker, NOW);

    let got = TokenVerifier::new(&other.public_raw, ISS, LEEWAY).verify(&tok, &ep, NOW);

    assert_eq!(got, Err(TokenError::SignatureOrMalformed));
}

#[test]
fn wrong_issuer_claim_denies() {
    // Correctly signed by our key, but `iss` is not the issuer we trust.
    let km = generate_keypair();
    let ep = endpoint_from_seed(1);
    let tok =
        TokenIssuer::new(&km.pkcs8_der, "https://evil.example", TTL).mint(&ep, Tier::Broker, NOW);

    let got = verifier(&km).verify(&tok, &ep, NOW);

    assert_eq!(got, Err(TokenError::WrongIssuer));
}

#[test]
fn malformed_token_denies() {
    let km = generate_keypair();
    let ep = endpoint_from_seed(1);
    let v = verifier(&km);

    for junk in ["", "not.a.jwt", "a.b", "....", "aaaa.bbbb.cccc"] {
        assert_eq!(
            v.verify(junk, &ep, NOW),
            Err(TokenError::SignatureOrMalformed),
            "junk {junk:?} must deny"
        );
    }
}

#[test]
fn tampered_payload_denies() {
    // Flip a byte in the payload segment; signature must fail.
    let km = generate_keypair();
    let ep = endpoint_from_seed(1);
    let tok = issuer(&km).mint(&ep, Tier::Broker, NOW);

    let mut parts: Vec<&str> = tok.split('.').collect();
    // Corrupt the middle (claims) segment deterministically.
    let mut payload = parts[1].to_string();
    let last = payload.pop().unwrap();
    payload.push(if last == 'A' { 'B' } else { 'A' });
    parts[1] = &payload;
    let tampered = parts.join(".");

    assert_eq!(
        verifier(&km).verify(&tampered, &ep, NOW),
        Err(TokenError::SignatureOrMalformed)
    );
}

#[test]
fn replay_within_expiry_by_legit_endpoint_admits() {
    // Documented and intentional: tokens are capabilities, not nonces. The same
    // valid token used twice by its own endpoint, both within expiry, admits
    // both times.
    let km = generate_keypair();
    let ep = endpoint_from_seed(1);
    let tok = issuer(&km).mint(&ep, Tier::Broker, NOW);
    let v = verifier(&km);

    assert!(v.verify(&tok, &ep, NOW).is_ok());
    assert!(v.verify(&tok, &ep, NOW + 60).is_ok());
}
