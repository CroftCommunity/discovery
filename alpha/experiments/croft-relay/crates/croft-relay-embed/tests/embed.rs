//! The embed adapter over a real `iroh_relay::server::ClientRequest`.
//!
//! Proves the Phase-2 hinge end to end against actual iroh types: a token bound
//! to the connection's cryptographically-authenticated endpoint id admits; the
//! same token presented from any other endpoint is refused. `ClientRequest::new`
//! (public in iroh-relay 1.0.3) lets us build an authenticated request without
//! standing up a relay.

use std::sync::Arc;

use http::header::AUTHORIZATION;
use iroh_base::SecretKey;
use iroh_relay::http::ProtocolVersion;
use iroh_relay::server::{Access, AccessControl, ClientRequest};
use ring::signature::{Ed25519KeyPair, KeyPair};

use croft_admit::endpoint_id::EndpointId;
use croft_admit::registry::Registry;
use croft_admit::tier::{bucket_for, RateBucket, Tier};
use croft_admit::token::{TokenIssuer, TokenVerifier};
use croft_relay_embed::{EmbedDecision, RegistryAccess, TokenAccess};

const ISS: &str = "https://admit.croft.ing";
const TTL: u64 = 900;
const LEEWAY: u64 = 30;
const NOW: u64 = 1_000_000;

struct Keys {
    pkcs8: Vec<u8>,
    public_raw: Vec<u8>,
}
fn keys() -> Keys {
    let rng = ring::rand::SystemRandom::new();
    let pkcs8 = Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
    let kp = Ed25519KeyPair::from_pkcs8(pkcs8.as_ref()).unwrap();
    Keys {
        pkcs8: pkcs8.as_ref().to_vec(),
        public_raw: kp.public_key().as_ref().to_vec(),
    }
}

/// A relay-authenticated `SecretKey` from a fixed seed (deterministic id).
fn secret(seed: u8) -> SecretKey {
    SecretKey::from_bytes(&[seed; 32])
}

/// croft-admit's view of an iroh endpoint (same 32 public-key bytes).
fn admit_id(sk: &SecretKey) -> EndpointId {
    EndpointId::from_bytes(*sk.public().as_bytes())
}

/// Build an authenticated `ClientRequest` for `sk`, optionally carrying `token`
/// as an `Authorization: Bearer` header — exactly how the iroh client presents
/// it on native targets.
fn request(sk: &SecretKey, token: Option<&str>) -> ClientRequest {
    let mut b = http::Request::builder().uri("https://relay.croft.ing/");
    if let Some(t) = token {
        b = b.header(AUTHORIZATION, format!("Bearer {t}"));
    }
    let parts = b.body(()).unwrap().into_parts().0;
    ClientRequest::new(sk.public(), ProtocolVersion::V2, parts)
}

fn access(keys: &Keys) -> TokenAccess {
    TokenAccess::new(TokenVerifier::new(&keys.public_raw, ISS, LEEWAY))
}
fn issuer(keys: &Keys) -> TokenIssuer {
    TokenIssuer::new(&keys.pkcs8, ISS, TTL)
}

#[test]
fn valid_token_bound_to_the_connection_admits_with_tier_and_bucket() {
    let k = keys();
    let sk = secret(1);
    let tok = issuer(&k).mint(&admit_id(&sk), Tier::Broker, NOW);

    let d = access(&k).decide(&request(&sk, Some(&tok)), NOW);

    assert_eq!(
        d,
        EmbedDecision::Admit {
            tier: Tier::Broker,
            bucket: RateBucket::UNLIMITED,
        }
    );
    assert_eq!(d.to_access(), Access::Allow);
}

#[test]
fn coordination_token_admits_with_the_capped_bucket() {
    let k = keys();
    let sk = secret(2);
    let tok = issuer(&k).mint(&admit_id(&sk), Tier::Coordination, NOW);

    let d = access(&k).decide(&request(&sk, Some(&tok)), NOW);

    assert_eq!(
        d,
        EmbedDecision::Admit {
            tier: Tier::Coordination,
            bucket: bucket_for(Tier::Coordination),
        }
    );
}

#[test]
fn token_for_another_endpoint_is_refused_on_this_connection() {
    // The anti-replay hinge over real iroh types: mint for endpoint A, present
    // it on a connection the relay authenticated as endpoint B -> deny.
    let k = keys();
    let a = secret(1);
    let b = secret(2);
    let tok_for_a = issuer(&k).mint(&admit_id(&a), Tier::Broker, NOW);

    let d = access(&k).decide(&request(&b, Some(&tok_for_a)), NOW);

    assert!(!d.is_admit());
    assert!(matches!(d.to_access(), Access::Deny { .. }));
}

#[test]
fn no_token_denies() {
    let k = keys();
    let d = access(&k).decide(&request(&secret(1), None), NOW);
    assert_eq!(d, EmbedDecision::DenyNoToken);
    assert!(matches!(d.to_access(), Access::Deny { .. }));
}

#[test]
fn expired_token_denies() {
    let k = keys();
    let sk = secret(1);
    let tok = issuer(&k).mint(&admit_id(&sk), Tier::Broker, NOW);

    let later = NOW + TTL + LEEWAY + 1;
    let d = access(&k).decide(&request(&sk, Some(&tok)), later);

    assert!(!d.is_admit());
    assert!(matches!(d.to_access(), Access::Deny { .. }));
}

#[tokio::test]
async fn on_connect_uses_the_real_clock_and_admits_a_fresh_token() {
    // Exercises the AccessControl trait impl + the wall-clock edge, not just
    // the injected-clock `decide`.
    let k = keys();
    let sk = secret(7);
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let tok = issuer(&k).mint(&admit_id(&sk), Tier::Broker, now);

    let access = access(&k);
    let granted = access.on_connect(&request(&sk, Some(&tok))).await;

    assert!(matches!(granted, Access::Allow));
}

#[tokio::test]
async fn registry_access_admits_only_enrolled_endpoints() {
    let reg = Arc::new(Registry::new());
    let enrolled = secret(1);
    reg.bind(
        admit_id(&enrolled),
        croft_admit::Did::parse("did:plc:alice").unwrap(),
    );
    let access = RegistryAccess::new(reg);

    assert!(matches!(
        access.on_connect(&request(&enrolled, None)).await,
        Access::Allow
    ));
    assert!(matches!(
        access.on_connect(&request(&secret(2), None)).await,
        Access::Deny { .. }
    ));
}
