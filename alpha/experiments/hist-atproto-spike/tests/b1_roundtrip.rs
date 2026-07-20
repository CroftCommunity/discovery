//! B1 — envelope ↔ record lossless round-trip (RUN-HIST-01).
//!
//! Serves history-durability.md §G via HIST-ATPROTO-MATCHUP.md rows 1–2: the
//! §G envelope maps to an `ing.croft.hist.entry`-shaped record and back with
//! byte-level fidelity over the canonical encoding (atproto_map precedent —
//! payload-level mapping, pure, no network). RED-able: a field dropped or
//! reordered by the mapping is caught by round-trip equality over canonical
//! bytes — the staged red drops `sizeHint`.
//!
//! `OWNER-CALL: HS OC-2 pending` — the record carries BOTH `entryDigest`
//! (in-house blake3) and the blessed-format blob CID; their fusion is the
//! owner call, and this test proves both survive the trip un-fused.

use hist_atproto_spike::envelope::{fixture_chain, Envelope};
use hist_atproto_spike::record::{blob_cid, from_record, record_bytes, to_record};

#[test]
fn envelope_record_roundtrip_is_lossless_bytewise() {
    for (env, blob) in fixture_chain("b1-chain", 5) {
        let record = to_record(&env, &blob);
        let (back, cid) = from_record(&record).expect("well-formed record maps back");

        // Byte-level fidelity over the canonical encoding: the §G envelope
        // survives the trip exactly.
        assert_eq!(
            env.canonical_bytes(),
            back.canonical_bytes(),
            "envelope canonical bytes must survive the record round-trip \
             (a dropped or defaulted field lands here)"
        );
        assert_eq!(env, back, "structural equality backs the byte equality");

        // The blob reference is the blessed-shaped CID of the sealed bytes —
        // carried beside entryDigest, never fused (HS OC-2 pending).
        assert_eq!(cid, blob_cid(&blob), "blob ref is the CID of the sealed bytes");
        assert_ne!(
            cid.hash().digest(),
            env.entry_digest.as_slice(),
            "spike carries entry_digest and blob CID as distinct identities \
             (their fusion is HS OC-2, not this crate's call)"
        );

        // Record-level determinism: re-mapping yields byte-identical records.
        assert_eq!(
            record_bytes(&record),
            record_bytes(&to_record(&back, &blob)),
            "record bytes are deterministic across a round-trip"
        );
    }
}

#[test]
fn malformed_records_are_rejected_whole_never_repaired() {
    let (env, blob) = &fixture_chain("b1-bad", 1)[0];
    let good = to_record(env, blob);

    // Wrong $type → rejected.
    let ipld_core::ipld::Ipld::Map(mut m) = good.clone() else {
        panic!("record is a map")
    };
    m.insert(
        "$type".to_string(),
        ipld_core::ipld::Ipld::String("ing.croft.other".into()),
    );
    assert!(from_record(&ipld_core::ipld::Ipld::Map(m)).is_err());

    // Truncated digest → rejected (the lexicon invalid-data posture:
    // entirely invalid, no repair, no partial processing).
    let ipld_core::ipld::Ipld::Map(mut m) = good.clone() else {
        panic!("record is a map")
    };
    m.insert(
        "entryDigest".to_string(),
        ipld_core::ipld::Ipld::Bytes(vec![0u8; 31]),
    );
    assert!(from_record(&ipld_core::ipld::Ipld::Map(m)).is_err());
}

#[test]
fn envelope_canonical_bytes_roundtrip() {
    let (env, _) = &fixture_chain("b1-canon", 1)[0];
    let bytes = env.canonical_bytes();
    let back = Envelope::from_canonical(&bytes).expect("canonical bytes decode");
    assert_eq!(env, &back);
}
