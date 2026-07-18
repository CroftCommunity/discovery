//! P1 — Envelope, identity, and record shapes (component).
//!
//! RUN-16 §A.5 made executable at the byte level. Red-first: these acceptance
//! criteria are committed as failing tests before `canonical`/`envelope`/
//! `identity` exist.
//!
//! Goldens (component grade — golden vectors, not live-wire predictions, so
//! guardrail-3 prediction-first does not apply; byte-stability is asserted
//! structurally and a pinned digest anchors regression):
//!
//! - canonical encoding byte-stability: two independent encodes of the same
//!   value are byte-identical; encode→decode→re-encode is a fixpoint.
//! - signature verify / reject.
//! - context-binding rejection: the same payload replayed into a DIFFERENT
//!   scope or DIFFERENT antecedents MUST fail verification.
//! - IDENTITY goldens: H(envelope) identical across two encodes; the same
//!   envelope arriving twice dedups to one identity via seen(H); the same
//!   payload from the same author under different antecedents yields TWO
//!   identities; a re-signed copy changes the hash (the signature is inside the
//!   hashed bytes → two different authors cannot share an identity).

use std::collections::HashSet;
use tier_proof::envelope::{Envelope, SignedBody};
use tier_proof::identity::Signer;

fn body(scope: &str, author: &str, antecedents: &[&str], payload: &[u8]) -> SignedBody {
    SignedBody {
        scope: scope.to_string(),
        author: author.to_string(),
        antecedents: antecedents.iter().map(|s| (*s).to_string()).collect(),
        payload: payload.to_vec(),
    }
}

#[test]
fn canonical_encoding_is_byte_stable() {
    let s = Signer::from_seed([7u8; 32]);
    let env = Envelope::seal(
        body("scope:alpha", &s.did(), &["aa", "bb"], b"hello"),
        &s,
    )
    .expect("seal");

    let a = tier_proof::canonical::to_canonical(&env).expect("encode a");
    let b = tier_proof::canonical::to_canonical(&env).expect("encode b");
    assert_eq!(a, b, "two independent encodes must be byte-identical");

    // encode -> decode -> re-encode is a fixpoint.
    let decoded: Envelope = ciborium::from_reader(a.as_slice()).expect("decode");
    let c = tier_proof::canonical::to_canonical(&decoded).expect("re-encode");
    assert_eq!(a, c, "round-trip must be a canonical fixpoint");
}

#[test]
fn canonical_key_ordering_is_deterministic_golden() {
    // A pinned digest of the canonical bytes of a fixed envelope. Anchors the
    // canonical map-key ordering (RFC 8949 §4.2.1 length-then-bytewise) so a
    // silent encoder change is caught. Value pinned at first green.
    let s = Signer::from_seed([1u8; 32]);
    let env = Envelope::seal(body("scope:golden", &s.did(), &["deadbeef"], b"g"), &s)
        .expect("seal");
    let bytes = tier_proof::canonical::to_canonical(&env).expect("encode");
    // The DID and signature are deterministic from the seed, so the whole
    // envelope — and its digest — is reproducible.
    assert_eq!(
        env.identity_hex(),
        "2707a13a9de09334a43c7668fbcea3d44b172dcbfc7aff7465b9c88861993d37",
        "identity digest must match the pinned golden"
    );
    assert!(!bytes.is_empty());
}

#[test]
fn signature_verifies_and_rejects() {
    let s = Signer::from_seed([2u8; 32]);
    let env = Envelope::seal(body("scope:s", &s.did(), &[], b"payload"), &s).expect("seal");
    assert!(env.verify().is_ok(), "a well-formed envelope verifies");

    // Tamper the payload: signature no longer covers these bytes.
    let mut tampered = env.clone();
    tampered.body.payload = b"payloax".to_vec();
    assert!(tampered.verify().is_err(), "tampered payload must be rejected");
}

#[test]
fn context_binding_rejects_scope_and_antecedent_replay() {
    let s = Signer::from_seed([3u8; 32]);
    let env = Envelope::seal(body("scope:home", &s.did(), &["p1"], b"m"), &s).expect("seal");

    // Replay the exact signed payload into a DIFFERENT scope.
    let mut moved_scope = env.clone();
    moved_scope.body.scope = "scope:other".to_string();
    assert!(
        moved_scope.verify().is_err(),
        "replay into a different scope must fail (context binding)"
    );

    // Replay under DIFFERENT antecedents (a different causal position).
    let mut moved_ante = env.clone();
    moved_ante.body.antecedents = vec!["p2".to_string()];
    assert!(
        moved_ante.verify().is_err(),
        "replay under different antecedents must fail (context binding)"
    );
}

#[test]
fn identity_is_stable_and_dedups() {
    let s = Signer::from_seed([4u8; 32]);
    let env = Envelope::seal(body("scope:d", &s.did(), &["x"], b"once"), &s).expect("seal");

    // H(envelope) identical across two independent encodes.
    assert_eq!(env.identity(), env.identity(), "identity is a pure function");

    // The same envelope arriving twice (two delivery paths) dedups to one.
    let path_a = env.clone();
    let path_b = env.clone();
    let mut seen: HashSet<[u8; 32]> = HashSet::new();
    seen.insert(path_a.identity());
    let inserted_second = seen.insert(path_b.identity());
    assert!(!inserted_second, "second delivery path dedups via seen(H)");
    assert_eq!(seen.len(), 1, "one identity after two deliveries");
}

#[test]
fn same_payload_different_antecedents_are_two_identities() {
    let s = Signer::from_seed([5u8; 32]);
    let first = Envelope::seal(body("scope:m", &s.did(), &["a"], b"same"), &s).expect("seal");
    let second = Envelope::seal(body("scope:m", &s.did(), &["a", "b"], b"same"), &s).expect("seal");
    assert_ne!(
        first.identity(),
        second.identity(),
        "same author + same payload under different antecedents are two messages"
    );
}

#[test]
fn resigned_copy_changes_the_hash() {
    // The signature is inside the hashed bytes: two envelopes with an identical
    // body but different signature bytes have different identities. This is why
    // two different authors cannot share an identity.
    let a = Signer::from_seed([6u8; 32]);
    let b = Signer::from_seed([9u8; 32]);
    let shared = body("scope:h", &a.did(), &["z"], b"payload");

    let signed_a = Envelope::seal(shared.clone(), &a).expect("seal a");
    // Re-sign the *same body bytes* with a different key (verify would fail, but
    // the identity must still differ because the sig is hashed).
    let mut signed_b = signed_a.clone();
    signed_b.sig = b.sign_body(&shared);
    assert_ne!(
        signed_a.identity(),
        signed_b.identity(),
        "a re-signed copy changes the identity hash"
    );
}
