//! P5 — Device-key delegation (LIVE; live DID-doc resolution BLOCKED here).
//!
//! An account delegates a device signing key by publishing an attestation
//! record; the verifier accepts an envelope signed by the device key by
//! resolving the account's DID document (the delegating identity) and confirming
//! a live, non-deleted attestation; deleting the attestation rejects the next
//! device envelope; the key cache is invalidated by the firehose delete EVENT,
//! not a TTL.
//!
//! Live grade needs a real `did:plc` document resolved over the PLC directory
//! (RUN-14 proved that path); without egress the account resolves as a `did:key`
//! (which needs no network — genuine resolution) behind the SAME
//! [`DidResolver`] interface a PLC resolver implements —
//! `SPEC-DELTA[run17-did-resolver | declared-stand-in]`.
//!
//! Prediction-first (guardrail 3) — constants BEFORE the (blocked) live call:
//!
//! - P5-1: the delegated device key is a `did:key` whose multibase body begins
//!   `z6Mk` (ed25519 = multicodec `0xed01`), the same `publicKeyMultibase`
//!   shape a DID-document `verificationMethod` carries.
//! - P5-2: `did_key_from_verifying` → `verifying_from_did_key` round-trips (the
//!   DID-doc key encoding is reversible and canonical).
//! - P5-3: deleting the attestation rejects the next device envelope
//!   (revocation-by-deletion), and the flip is caused by the delete EVENT.

use tier_proof::delegation::{DelegReject, DelegationVerifier, DidKeyResolver, DidResolver};
use tier_proof::identity::{did_key_from_verifying, verifying_from_did_key, Signer};
use tier_proof::records::{self, Record};
use tier_proof::source::{MemSource, RecordSource};

const SCOPE: &str = "scope:devices";

fn account() -> Signer {
    Signer::from_seed([50u8; 32])
}
fn device() -> Signer {
    Signer::from_seed([51u8; 32])
}

#[test]
fn did_doc_key_encoding_prediction_holds() {
    // P5-1 + P5-2 at component grade.
    let dev = device();
    assert!(dev.did().starts_with("did:key:z6Mk"), "ed25519 did:key begins z6Mk (P5-1)");
    let vk = verifying_from_did_key(&dev.did()).expect("decode");
    assert_eq!(did_key_from_verifying(&vk), dev.did(), "encoding round-trips (P5-2)");
}

/// Build a firehose with an account delegating a device key via an attestation.
fn source_with_attestation() -> (MemSource, String) {
    let mut src = MemSource::new();
    let att = src.put_record(
        &account(),
        Record::DeviceAttestation {
            scope: SCOPE.to_string(),
            device_did: device().did(),
        },
    );
    (src, att)
}

/// An envelope signed by the DEVICE key (the account is not in the loop per-msg).
fn device_envelope(text: &str) -> tier_proof::envelope::Envelope {
    records::seal(&device(), vec![], &Record::Message { scope: SCOPE.to_string(), text: text.to_string() })
}

#[test]
fn verifier_accepts_device_envelope_under_a_live_attestation() {
    let (src, _att) = source_with_attestation();
    let verifier = DelegationVerifier::from_events(DidKeyResolver, &src.all());

    let env = device_envelope("from my phone");
    assert!(
        verifier.accepts_device_envelope(&env, &account().did()).is_ok(),
        "device envelope accepted while the attestation is live"
    );
}

#[test]
fn envelope_from_undelegated_key_is_rejected() {
    let (src, _att) = source_with_attestation();
    let verifier = DelegationVerifier::from_events(DidKeyResolver, &src.all());

    // A different, undelegated device.
    let rogue = Signer::from_seed([77u8; 32]);
    let env = records::seal(&rogue, vec![], &Record::Message { scope: SCOPE.to_string(), text: "rogue".into() });
    assert_eq!(
        verifier.accepts_device_envelope(&env, &account().did()),
        Err(DelegReject::NoActiveAttestation),
    );
}

#[test]
fn deleting_the_attestation_rejects_the_next_device_envelope() {
    // P5-3: revocation-by-deletion, event-driven (no TTL).
    let (mut src, att) = source_with_attestation();

    // Before the delete: accepted, and repeated calls are stable (no time input).
    let before = DelegationVerifier::from_events(DidKeyResolver, &src.all());
    let env = device_envelope("still me");
    assert!(before.accepts_device_envelope(&env, &account().did()).is_ok());
    assert!(before.accepts_device_envelope(&env, &account().did()).is_ok(), "stable: not TTL-driven");

    // Delete the attestation on the firehose.
    src.delete(&account(), &att);
    let after = DelegationVerifier::from_events(DidKeyResolver, &src.all());
    assert_eq!(
        after.accepts_device_envelope(&env, &account().did()),
        Err(DelegReject::NoActiveAttestation),
        "the delete event invalidates the delegation"
    );
}

#[test]
fn cache_invalidation_is_event_driven_not_ttl() {
    // Applying the delete event is what flips the verdict — asserted by driving
    // the verifier event-by-event with no clock in the loop.
    let (src, att) = source_with_attestation();
    let mut verifier = DelegationVerifier::new(DidKeyResolver);
    for ev in src.all() {
        verifier.apply_event(&ev);
    }
    let env = device_envelope("hello");
    assert!(verifier.accepts_device_envelope(&env, &account().did()).is_ok());

    // Feed only a delete event — the sole cause of invalidation.
    verifier.apply_event(&tier_proof::source::SourceEvent::Delete {
        author: account().did(),
        target: att,
    });
    assert_eq!(
        verifier.accepts_device_envelope(&env, &account().did()),
        Err(DelegReject::NoActiveAttestation),
    );
}

#[test]
fn account_that_does_not_resolve_is_rejected() {
    // The resolver interface is exercised: an account DID the resolver cannot
    // resolve is refused (a real PLC resolver would 404; the did:key resolver
    // rejects a malformed DID).
    let (src, _att) = source_with_attestation();
    let verifier = DelegationVerifier::from_events(DidKeyResolver, &src.all());
    assert!(verifier.resolver().resolve_key("did:key:not-a-key").is_none());
}
