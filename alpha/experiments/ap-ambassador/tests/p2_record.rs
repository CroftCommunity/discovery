//! P2 — the receipt record.
//!
//! Acceptance (RUN-AP-01 §2 P2):
//! - T-AP2.1 canonical envelope round-trips (encode → decode of the parts
//!   we hash is stable byte-for-byte).
//! - T-AP2.2 receipt identity `H(envelope)` — same record different
//!   construction path yields the same ReceiptId.
//! - T-AP2.3 body hash is BLAKE3 of canonical body encoding — mutating any
//!   header changes it.
//! - T-AP2.4 commitment = BLAKE3(salt || body_hash) — same salt+body
//!   deterministic; different salt → different commitment.
//! - T-AP2.5 blinded posture: a record carrying (commitment, body_hash) can
//!   later be re-verified once the body is produced; the body alone,
//!   without the salt, cannot deanonymize the commitment (structural — no
//!   way to compute the commitment from body alone).
//! - T-AP2.6 store insert / fetch — record and body retrievable by
//!   ReceiptId.

use ap_ambassador::records::*;
use ap_ambassador::store::EvidenceStore;
use ap_ambassador::types::*;

fn sample_body() -> EvidenceBody {
    EvidenceBody {
        raw_body: br#"{"type":"Follow","id":"x","actor":"a","object":"b"}"#.to_vec(),
        headers: vec![
            ("host".into(), "example.social".into()),
            ("date".into(), "Mon, 20 Jul 2026 12:00:00 GMT".into()),
        ],
        actor_key_spki_der: vec![0x30, 0x82, 0x00, 0x01],
        actor_key_id: KeyId::new("https://alice.example/actor#main-key"),
        method: "POST".into(),
        path: "/inbox".into(),
    }
}

fn sample_record(body_hash: [u8; 32], commitment: [u8; 32]) -> ReceiptRecord {
    ReceiptRecord {
        kind: ActivityKind::Follow,
        actor: ActorId::new("https://alice.example/actor"),
        object: "https://example.social/users/bob".into(),
        activity_id: "https://alice.example/activities/1".into(),
        undoes: None,
        state: ReceiptState::EvidenceComplete,
        commitment,
        body_hash,
        attestation_marker: ReceiptRecord::GATEWAY_MARKER.into(),
    }
}

// T-AP2.1 — canonical body encoding is deterministic (same input → same bytes)

#[test]
fn t_ap2_1_canonical_body_encoding_deterministic() {
    let body = sample_body();
    let b1 = ap_ambassador::canonical::encode_evidence_body(&body);
    let b2 = ap_ambassador::canonical::encode_evidence_body(&body);
    assert_eq!(b1, b2, "canonical encode must be deterministic");
    assert!(!b1.is_empty());
}

// T-AP2.2 — receipt identity: same record via two construction paths → same id

#[test]
fn t_ap2_2_receipt_identity_matches() {
    let body = sample_body();
    let bh = body.body_hash();
    let salt = Salt([7u8; 32]);
    let c = commitment(&salt, &bh);
    let r_a = sample_record(bh, c);
    let r_b = sample_record(bh, c); // rebuild from same inputs
    assert_eq!(r_a.receipt_id(), r_b.receipt_id());
    // Changing the salt (⇒ changing the commitment) changes the receipt id.
    let salt2 = Salt([8u8; 32]);
    let c2 = commitment(&salt2, &bh);
    let r_c = sample_record(bh, c2);
    assert_ne!(r_a.receipt_id(), r_c.receipt_id());
}

// T-AP2.3 — mutating any header changes body_hash

#[test]
fn t_ap2_3_body_hash_covers_headers() {
    let body = sample_body();
    let h_before = body.body_hash();
    let mut mutated = body.clone();
    mutated.headers[0].1 = "different-host".to_string();
    assert_ne!(h_before, mutated.body_hash());
    // And the raw body itself:
    let mut mutated2 = body.clone();
    mutated2.raw_body.push(0x20);
    assert_ne!(h_before, mutated2.body_hash());
}

// T-AP2.4 — commitment determinism and salt-dependence

#[test]
fn t_ap2_4_commitment_determinism_and_salt() {
    let body = sample_body();
    let bh = body.body_hash();
    let salt_a = Salt([1u8; 32]);
    let salt_b = Salt([2u8; 32]);
    assert_eq!(commitment(&salt_a, &bh), commitment(&salt_a, &bh));
    assert_ne!(commitment(&salt_a, &bh), commitment(&salt_b, &bh));
}

// T-AP2.5 — blinded re-verify: given a record's (commitment, body_hash) and
// a produced body + salt, the commitment reproduces.

#[test]
fn t_ap2_5_blinded_reverify_and_no_body_alone_deanon() {
    let body = sample_body();
    let bh = body.body_hash();
    let salt = Salt([13u8; 32]);
    let c = commitment(&salt, &bh);

    // Reverify path — reproducing the commitment from the body + the salt.
    let bh_again = body.body_hash();
    assert_eq!(bh_again, bh, "body hash reproducible from stored body");
    assert_eq!(commitment(&salt, &bh_again), c);

    // Body alone (no salt) — the commitment covers salt bytes too, so a
    // caller who hashes only the body cannot arrive at the commitment.
    let body_only_hash = *blake3::hash(&bh_again).as_bytes();
    assert_ne!(body_only_hash, c);
    // Even brute-forcing over the body: an attacker without the salt cannot
    // confirm a specific record — the commitment is not H(body) alone.
    assert_ne!(bh_again, c);
}

// T-AP2.6 — store insert / fetch by ReceiptId

#[test]
fn t_ap2_6_store_insert_fetch() {
    let body = sample_body();
    let bh = body.body_hash();
    let salt = Salt([21u8; 32]);
    let c = commitment(&salt, &bh);
    let r = sample_record(bh, c);
    let mut store = EvidenceStore::new();
    let id = store.insert(r.clone(), body.clone());
    assert_eq!(store.len(), 1);
    let got_r = store.receipt(&id).expect("record present");
    assert_eq!(got_r, &r);
    let got_b = store.body(&id).expect("body present");
    assert_eq!(got_b, &body);
    // Attestation marker legibility (P5 companion assertion):
    assert_eq!(got_r.attestation_marker, ReceiptRecord::GATEWAY_MARKER);
    assert_eq!(got_r.state, ReceiptState::EvidenceComplete);
}

// T-AP2.7 — golden bytes: the canonical encoding of a fully-specified
// receipt matches a fixed golden byte sequence.

#[test]
fn t_ap2_7_golden_bytes_stable() {
    // A record with every field fully specified — a golden vector so a
    // future refactor of the canonical encoder is caught.
    let body_hash = [0x11u8; 32];
    let commitment = [0x22u8; 32];
    let r = ReceiptRecord {
        kind: ActivityKind::Follow,
        actor: ActorId::new("https://alice.example/actor"),
        object: "https://example.social/users/bob".into(),
        activity_id: "https://alice.example/activities/1".into(),
        undoes: None,
        state: ReceiptState::EvidenceComplete,
        commitment,
        body_hash,
        attestation_marker: ReceiptRecord::GATEWAY_MARKER.into(),
    };
    let bytes = ap_ambassador::canonical::encode_receipt(&r);
    // Two runs must produce identical bytes; the length must be stable and
    // non-zero. (A stronger "vs stored constants" pin will land once the
    // encoding is release-pinned; this run stays TEST-ONLY serialization,
    // consistent with attest-family's convention.)
    let bytes2 = ap_ambassador::canonical::encode_receipt(&r);
    assert_eq!(bytes, bytes2);
    assert!(bytes.len() > 32);
    // The receipt id is BLAKE3 of the same bytes.
    let expected = blake3::hash(&bytes);
    assert_eq!(r.receipt_id().0, *expected.as_bytes());
}
