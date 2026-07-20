//! EXP-LEX-03 — the clean-room verifier for the CID-First Attestation Spec.
//!
//! Acceptance criteria (red-first):
//!   * INTEROP — our verifier accepts records signed by the REFERENCE impl
//!     (crates.io atproto-attestation 0.14.5), inline and remote;
//!   * ROUND-TRIP — our builder+verifier agree, inline and remote;
//!   * ADVERSARIAL — mutated payload, cross-repo replay, foreign-key swap, high-S,
//!     tampered proof each fail for the STATED reason;
//!   * DEMO — an organizer-signed attendance attestation over a REAL public RSVP
//!     record verifies.
//!
//! The interop cases are the strongest evidence a second implementation can
//! bring: two independent codebases agreeing on the same bytes.

mod common;
use common::{load, reference_public_did_key, MockResolver};

use lexicon_community::attest::{
    build_inline, build_remote, verify_inline, verify_record, verify_remote, VerifyError, VerifyOpts,
    Verified,
};
use lexicon_community::didkey::Curve;
use lexicon_community::sign::SignKey;

// --- INTEROP: verify the reference implementation's real output ---------------

#[test]
fn interop_reference_inline_verifies() {
    let mut fixture = load("golden/interop_inline_p256.json");
    // The reference CLI omits the public `key` (a finding — AMBIGUITIES A-2b);
    // we supply the did:key derived from its published signing key.
    let key = reference_public_did_key();
    let mut entry = fixture["signatures"][0].as_object().unwrap().clone();
    entry.insert("key".into(), serde_json::Value::String(key.clone()));
    let entry = serde_json::Value::Object(entry);

    // No issuer in the reference metadata → verify with the lax (naive) posture.
    let resolver = MockResolver::default();
    let v = verify_inline(&fixture, &entry, "did:plc:testrepo123", &resolver, &VerifyOpts::lax())
        .expect("our verifier accepts the reference impl's inline signature");
    match v {
        Verified::Inline { cid, .. } => {
            assert_eq!(cid, "bafyreihdsqaggs62r6qycdbp456kqu36bob62dy3gpmlrazeh5xzafhhne")
        }
        _ => panic!("expected inline"),
    }
    let _ = &mut fixture;
}

#[test]
fn interop_reference_inline_wrong_repo_fails() {
    // Replay prevention: the reference signature is bound to did:plc:testrepo123.
    let fixture = load("golden/interop_inline_p256.json");
    let key = reference_public_did_key();
    let mut entry = fixture["signatures"][0].as_object().unwrap().clone();
    entry.insert("key".into(), serde_json::Value::String(key));
    let entry = serde_json::Value::Object(entry);
    let resolver = MockResolver::default();
    let err = verify_inline(&fixture, &entry, "did:plc:someoneelse", &resolver, &VerifyOpts::lax())
        .expect_err("a different repository DID must break the CID");
    assert!(matches!(err, VerifyError::CidMismatch { .. }), "got {err:?}");
}

#[test]
fn interop_reference_remote_verifies() {
    let attested = load("golden/interop_remote_attested.json");
    let proof = load("golden/interop_remote_proof.json");
    let entry = attested["signatures"][0].clone();
    let uri = entry["uri"].as_str().unwrap().to_string();
    let resolver = MockResolver::default().with_record(&uri, proof);
    // The strongRef binds to the SOURCE repo the proof was computed against.
    let v = verify_remote(&attested, &entry, "did:plc:sourcerepo", &resolver)
        .expect("our verifier accepts the reference impl's remote attestation");
    assert!(matches!(v, Verified::Remote { .. }));
}

// --- ROUND-TRIP: our builder ↔ our verifier -----------------------------------

fn organizer() -> SignKey {
    SignKey::from_seed(Curve::P256, b"RUN-LEX-01/organizer")
}

#[test]
fn self_inline_round_trips_strict() {
    let record = load("golden/event_valid.json");
    let signer = organizer();
    let issuer = "did:plc:organizer";
    let meta = serde_json::json!({"$type":"community.lexicon.attest.attendance","issuer":issuer,"purpose":"organizer-signed"});
    let signed = build_inline(&record, &signer, &meta, "did:plc:eventrepo").unwrap();

    let resolver = MockResolver::default().with_key(issuer, &signer.did_key());
    let entry = signed["signatures"][0].clone();
    let v = verify_inline(&signed, &entry, "did:plc:eventrepo", &resolver, &VerifyOpts::default())
        .expect("strict verify (issuer binding + low-S) accepts our own attestation");
    assert!(matches!(v, Verified::Inline { .. }));
}

#[test]
fn self_remote_round_trips() {
    let record = load("golden/event_valid.json");
    let meta = serde_json::json!({"$type":"community.lexicon.attest.attendance","issuer":"did:plc:coop","purpose":"attend"});
    let (proof, attested) =
        build_remote(&record, &meta, "did:plc:eventrepo", "did:plc:coop", "3demo123").unwrap();
    let entry = attested["signatures"][0].clone();
    let uri = entry["uri"].as_str().unwrap().to_string();
    let resolver = MockResolver::default().with_record(&uri, proof);
    verify_remote(&attested, &entry, "did:plc:eventrepo", &resolver).expect("remote round-trip");
}

// --- ADVERSARIAL --------------------------------------------------------------

#[test]
fn mutated_payload_fails() {
    let record = load("golden/event_valid.json");
    let signer = organizer();
    let meta = serde_json::json!({"$type":"x","issuer":"did:plc:organizer"});
    let mut signed = build_inline(&record, &signer, &meta, "did:plc:eventrepo").unwrap();
    // Flip the event name AFTER signing.
    signed["name"] = serde_json::Value::String("TAMPERED".into());
    let entry = signed["signatures"][0].clone();
    let resolver = MockResolver::default();
    let err = verify_inline(&signed, &entry, "did:plc:eventrepo", &resolver, &VerifyOpts::lax())
        .expect_err("a mutated payload must fail");
    assert!(matches!(err, VerifyError::CidMismatch { .. }), "got {err:?}");
}

#[test]
fn foreign_key_swap_lax_accepts_but_strict_rejects() {
    // THE finding (A-1). The attacker recomputes the (public) CID and re-signs it
    // with THEIR key, swapping both `key` and `signature`. Because `key` is not
    // covered by the signature, the naive/lax verifier ACCEPTS the forgery —
    // proving inline attestations authenticate a key, not an issuer. The strict
    // posture (issuer↔key DID-document binding) is the fix.
    let record = load("golden/event_valid.json");
    let issuer = "did:plc:organizer";
    let honest = organizer();
    let meta = serde_json::json!({"$type":"x","issuer":issuer});
    let signed = build_inline(&record, &honest, &meta, "did:plc:eventrepo").unwrap();

    // Attacker re-signs the same CID with their own key.
    let attacker = SignKey::from_seed(Curve::P256, b"attacker");
    let cid = signed["signatures"][0]["cid"].as_str().unwrap();
    let cid_parsed = lexicon_community::cidfirst::RecordCid::parse(cid).unwrap();
    let asig = attacker.sign_cid(&cid_parsed);
    use base64::Engine as _;
    let mut entry = signed["signatures"][0].as_object().unwrap().clone();
    entry.insert("key".into(), serde_json::Value::String(attacker.did_key()));
    entry.insert(
        "signature".into(),
        serde_json::json!({"$bytes": base64::engine::general_purpose::STANDARD.encode(&asig)}),
    );
    let entry = serde_json::Value::Object(entry);

    // Lax: forgery ACCEPTED (the gap).
    let lax = MockResolver::default();
    verify_inline(&signed, &entry, "did:plc:eventrepo", &lax, &VerifyOpts::lax())
        .expect("lax verifier is fooled by the key swap — this IS the vulnerability");

    // Strict: attacker key is not authorized by the issuer's DID document.
    let strict = MockResolver::default().with_key(issuer, &honest.did_key());
    let err = verify_inline(&signed, &entry, "did:plc:eventrepo", &strict, &VerifyOpts::default())
        .expect_err("strict issuer-binding rejects the foreign key");
    assert!(matches!(err, VerifyError::IssuerBindingAbsent { .. }), "got {err:?}");
}

#[test]
fn high_s_signature_rejected() {
    // Malleability: negate S to make the (still valid) signature high-S; the
    // strict verifier rejects it per spec §6.
    let record = load("golden/event_valid.json");
    let signer = organizer();
    let meta = serde_json::json!({"$type":"x","issuer":"did:plc:organizer"});
    let signed = build_inline(&record, &signer, &meta, "did:plc:eventrepo").unwrap();
    let mut entry = signed["signatures"][0].as_object().unwrap().clone();

    use base64::Engine as _;
    let sig_b64 = entry["signature"]["$bytes"].as_str().unwrap();
    let sig_bytes = base64::engine::general_purpose::STANDARD.decode(sig_b64).unwrap();
    let sig = p256::ecdsa::Signature::from_slice(&sig_bytes).unwrap();
    let (r, s) = sig.split_scalars();
    let high = p256::ecdsa::Signature::from_scalars(*r.as_ref(), -*s.as_ref()).unwrap();
    entry.insert(
        "signature".into(),
        serde_json::json!({"$bytes": base64::engine::general_purpose::STANDARD.encode(high.to_bytes())}),
    );
    let entry = serde_json::Value::Object(entry);

    let resolver = MockResolver::default().with_key("did:plc:organizer", &signer.did_key());
    let err = verify_inline(&signed, &entry, "did:plc:eventrepo", &resolver, &VerifyOpts::default())
        .expect_err("high-S rejected");
    assert!(matches!(err, VerifyError::HighS), "got {err:?}");
}

#[test]
fn remote_proof_cid_tamper_fails() {
    let record = load("golden/event_valid.json");
    let meta = serde_json::json!({"$type":"a","issuer":"did:plc:coop"});
    let (proof, attested) =
        build_remote(&record, &meta, "did:plc:eventrepo", "did:plc:coop", "3x").unwrap();
    let entry = attested["signatures"][0].clone();
    let uri = entry["uri"].as_str().unwrap().to_string();
    // Tamper: the strongRef cid no longer matches the proof record's plain CID.
    let mut bad = entry.as_object().unwrap().clone();
    bad.insert(
        "cid".into(),
        serde_json::Value::String("bafyreiaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".into()),
    );
    let bad = serde_json::Value::Object(bad);
    let resolver = MockResolver::default().with_record(&uri, proof);
    let err = verify_remote(&attested, &bad, "did:plc:eventrepo", &resolver)
        .expect_err("tampered strongRef cid");
    assert!(matches!(err, VerifyError::ProofCidMismatch { .. }), "got {err:?}");
}

#[test]
fn remote_replay_other_source_repo_fails() {
    let record = load("golden/event_valid.json");
    let meta = serde_json::json!({"$type":"a","issuer":"did:plc:coop"});
    let (proof, attested) =
        build_remote(&record, &meta, "did:plc:eventrepo", "did:plc:coop", "3y").unwrap();
    let entry = attested["signatures"][0].clone();
    let uri = entry["uri"].as_str().unwrap().to_string();
    let resolver = MockResolver::default().with_record(&uri, proof);
    // Verify against the WRONG source repo — the attestation CID won't match.
    let err = verify_remote(&attested, &entry, "did:plc:otherrepo", &resolver)
        .expect_err("remote replay to another repo");
    assert!(matches!(err, VerifyError::ProofBindingMismatch { .. }), "got {err:?}");
}

// --- DEMO ---------------------------------------------------------------------

#[test]
fn demo_attendance_attestation_over_real_rsvp() {
    // Organizer signs an attendance attestation over a REAL public RSVP record
    // (captured from pds.cauda.cloud). Verified by the clean-room verifier.
    let rsvp = load("recorded/rsvp.getRecord.json");
    let record = rsvp["value"].clone();
    let organizer = organizer();
    let issuer = "did:plc:organizer";
    let repo = "did:plc:cbkjy5n7bk3ax2wplmtjofq2"; // the RSVP author's repo
    let meta = serde_json::json!({
        "$type":"community.lexicon.attest.attendance",
        "issuer":issuer,
        "purpose":"organizer-confirmed-attendance"
    });
    let signed = build_inline(&record, &organizer, &meta, repo).unwrap();
    let resolver = MockResolver::default().with_key(issuer, &organizer.did_key());
    let out = verify_record(&signed, repo, &resolver, &VerifyOpts::default())
        .expect("organizer-signed attendance over a real RSVP verifies");
    assert_eq!(out.len(), 1);
}
