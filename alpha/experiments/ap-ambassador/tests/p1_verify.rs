//! P1 — verification core + golden fixtures.
//!
//! Acceptance (RUN-AP-01 §2 P1):
//! - T-AP1.1 happy path: a Mastodon-shaped Follow signed by a fixture actor
//!   verifies; the returned VerifiedActivity carries the pinned actor SPKI
//!   bytes and the parsed activity.
//! - T-AP1.2 KeyResolutionFailed — resolver returns None for the keyId.
//! - T-AP1.3 DigestMismatch — body bytes mutated after signing.
//! - T-AP1.4 SignatureMismatch — signature bytes flipped one bit.
//! - T-AP1.5 MalformedActivity — truncated / non-JSON body.
//! - T-AP1.6 no-collapse: the four tamper fixtures produce four DISTINCT
//!   variants — no variant collapses into another.

use ap_ambassador::fixtures::*;
use ap_ambassador::types::*;
use ap_ambassador::verify::*;

const HOST: &str = "example.social";
const PATH: &str = "/users/bob/inbox";
const DATE: &str = "Mon, 20 Jul 2026 12:00:00 GMT";

fn alice() -> FixtureActor {
    FixtureActor::generate(
        "alice-seed-v1",
        "https://alice.example/actor#main-key",
        "https://alice.example/actor",
    )
}

fn resolver_for(actor: &FixtureActor) -> FixtureKeyResolver {
    FixtureKeyResolver::from_actor(actor)
}

// T-AP1.1 — happy path

#[test]
fn t_ap1_1_happy_follow_verifies() {
    let alice = alice();
    let bob_url = "https://example.social/users/bob";
    let body = follow_json(&alice.actor_url, bob_url, "https://alice.example/activities/1");
    let req = build_signed_post(&alice, HOST, PATH, DATE, body);
    let resolver = resolver_for(&alice);
    let verified = verify_ap_http_signature(&req, &resolver).expect("happy path verifies");
    assert_eq!(verified.actor_key_id, alice.key_id);
    assert_eq!(verified.actor_key_spki_der, alice.spki_der);
    assert_eq!(verified.activity.kind, ActivityKind::Follow);
    assert_eq!(verified.activity.actor, alice.actor_url);
    assert_eq!(verified.activity.object, bob_url);
}

// T-AP1.2 — KeyResolutionFailed

#[test]
fn t_ap1_2_key_resolution_failed() {
    let alice = alice();
    let body = follow_json(
        &alice.actor_url,
        "https://example.social/users/bob",
        "https://alice.example/activities/1",
    );
    let req = build_signed_post(&alice, HOST, PATH, DATE, body);
    // Resolver knows a DIFFERENT actor's key — Alice's keyId won't resolve.
    let mallory = FixtureActor::generate(
        "mallory-seed-v1",
        "https://mallory.example/actor#main-key",
        "https://mallory.example/actor",
    );
    let resolver = resolver_for(&mallory);
    let err = verify_ap_http_signature(&req, &resolver).expect_err("must fail");
    assert_eq!(err, VerifyError::KeyResolutionFailed);
}

// T-AP1.3 — DigestMismatch

#[test]
fn t_ap1_3_digest_mismatch() {
    let alice = alice();
    let body = follow_json(
        &alice.actor_url,
        "https://example.social/users/bob",
        "https://alice.example/activities/1",
    );
    let mut req = build_signed_post(&alice, HOST, PATH, DATE, body);
    // Mutate the BODY after signing — Digest header no longer matches.
    let idx = req.body.len() - 2;
    req.body[idx] ^= 0x01;
    let resolver = resolver_for(&alice);
    let err = verify_ap_http_signature(&req, &resolver).expect_err("must fail");
    assert_eq!(err, VerifyError::DigestMismatch);
}

// T-AP1.4 — SignatureMismatch

#[test]
fn t_ap1_4_signature_mismatch() {
    let alice = alice();
    let body = follow_json(
        &alice.actor_url,
        "https://example.social/users/bob",
        "https://alice.example/activities/1",
    );
    let mut req = build_signed_post(&alice, HOST, PATH, DATE, body);
    // Mutate the SIGNATURE (last chunk of the Signature header) — keep the
    // digest header intact so we exercise the RSA verify failure path, not
    // the digest check.
    let sig_hdr = req.headers.iter_mut().find(|(k, _)| k == "signature").unwrap();
    // Flip a base64 character in the middle of the signature; find the
    // signature="…" segment and mutate one char inside its quotes.
    let s = &mut sig_hdr.1;
    let sig_marker = "signature=\"";
    let start = s.find(sig_marker).unwrap() + sig_marker.len();
    // Grab a char inside the base64; swap it for a different valid base64 char.
    let mut chars: Vec<char> = s.chars().collect();
    let target = start + 10; // safely inside the base64 blob
    chars[target] = if chars[target] == 'A' { 'B' } else { 'A' };
    *s = chars.into_iter().collect();
    let resolver = resolver_for(&alice);
    let err = verify_ap_http_signature(&req, &resolver).expect_err("must fail");
    assert_eq!(err, VerifyError::SignatureMismatch);
}

// T-AP1.5 — MalformedActivity

#[test]
fn t_ap1_5_malformed_activity_truncated_json() {
    let alice = alice();
    // A body that is not a JSON object at all. The digest and signature will
    // still be applied over these bytes by build_signed_post, so we pass the
    // signature stage — but the AP parse fails.
    let bad = b"not-json".to_vec();
    let req = build_signed_post(&alice, HOST, PATH, DATE, bad);
    let resolver = resolver_for(&alice);
    let err = verify_ap_http_signature(&req, &resolver).expect_err("must fail");
    match err {
        VerifyError::MalformedActivity(_) => {}
        other => panic!("expected MalformedActivity, got {other:?}"),
    }
}

// T-AP1.6 — no-collapse: the four tamper fixtures produce four distinct variants.

#[test]
fn t_ap1_6_no_variant_collapses() {
    use std::collections::BTreeSet;

    let alice = alice();

    // (a) key resolution failure
    let body1 = follow_json(
        &alice.actor_url,
        "https://example.social/users/bob",
        "https://alice.example/activities/x",
    );
    let req1 = build_signed_post(&alice, HOST, PATH, DATE, body1);
    let mallory = FixtureActor::generate(
        "mallory-seed-v2",
        "https://mallory.example/actor#main-key",
        "https://mallory.example/actor",
    );
    let e1 = verify_ap_http_signature(&req1, &resolver_for(&mallory)).unwrap_err();

    // (b) digest mismatch
    let body2 = follow_json(
        &alice.actor_url,
        "https://example.social/users/bob",
        "https://alice.example/activities/x2",
    );
    let mut req2 = build_signed_post(&alice, HOST, PATH, DATE, body2);
    req2.body[0] ^= 0xff;
    let e2 = verify_ap_http_signature(&req2, &resolver_for(&alice)).unwrap_err();

    // (c) signature mismatch
    let body3 = follow_json(
        &alice.actor_url,
        "https://example.social/users/bob",
        "https://alice.example/activities/x3",
    );
    let mut req3 = build_signed_post(&alice, HOST, PATH, DATE, body3);
    let sig_hdr = req3.headers.iter_mut().find(|(k, _)| k == "signature").unwrap();
    let s = &mut sig_hdr.1;
    let sig_marker = "signature=\"";
    let start = s.find(sig_marker).unwrap() + sig_marker.len();
    let mut chars: Vec<char> = s.chars().collect();
    chars[start + 10] = if chars[start + 10] == 'A' { 'B' } else { 'A' };
    *s = chars.into_iter().collect();
    let e3 = verify_ap_http_signature(&req3, &resolver_for(&alice)).unwrap_err();

    // (d) malformed
    let bad = b"not-json".to_vec();
    let req4 = build_signed_post(&alice, HOST, PATH, DATE, bad);
    let e4 = verify_ap_http_signature(&req4, &resolver_for(&alice)).unwrap_err();

    // Discriminant-only, ignoring MalformedActivity's payload string.
    fn tag(e: &VerifyError) -> u8 {
        match e {
            VerifyError::SignatureMismatch => 1,
            VerifyError::KeyResolutionFailed => 2,
            VerifyError::DigestMismatch => 3,
            VerifyError::MalformedActivity(_) => 4,
            VerifyError::EvidenceRedacted => 5,
        }
    }
    let s: BTreeSet<u8> = [tag(&e1), tag(&e2), tag(&e3), tag(&e4)].into_iter().collect();
    assert_eq!(s.len(), 4, "each tamper class must map to a DISTINCT variant");
    assert!(s.contains(&2));
    assert!(s.contains(&3));
    assert!(s.contains(&1));
    assert!(s.contains(&4));
}

// T-AP1.7 — an UndoFollow parses; carries a `undoes` field with the inner Follow id.

#[test]
fn t_ap1_7_undo_follow_parses() {
    let alice = alice();
    let body = undo_follow_json(
        &alice.actor_url,
        "https://alice.example/activities/1",
        "https://example.social/users/bob",
        "https://alice.example/activities/2",
    );
    let req = build_signed_post(&alice, HOST, PATH, DATE, body);
    let resolver = resolver_for(&alice);
    let verified = verify_ap_http_signature(&req, &resolver).expect("Undo verifies");
    assert_eq!(verified.activity.kind, ActivityKind::UndoFollow);
    assert_eq!(
        verified.activity.undoes.as_deref(),
        Some("https://alice.example/activities/1"),
    );
}
