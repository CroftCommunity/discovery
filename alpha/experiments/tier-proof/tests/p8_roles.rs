//! P8 (component matrix) — delivery roles reconciliation + interval backfill.
//!
//! The A.7 rehearsal, component half. The process half (three co-hosted
//! processes over local sockets, failure isolation) is `p8_processes.rs`.
//!
//! - dedup by `H(envelope)` across transports: an envelope injected ONLY via the
//!   swarm path and another ONLY via the DS path both appear exactly ONCE in the
//!   converged set;
//! - interval backfill: a requester proving membership for `[J, R)` receives
//!   exactly the envelopes with causal positions in that interval; is refused
//!   pre-`J` history; after revocation is refused new material (the refusal is
//!   OFFERING-side — the node offers nothing outside the proven interval, so it
//!   retains nothing to "unsee");
//! - a non-member's backfill request is refused outright;
//! - a SEALED scope: the convergence node's store is ciphertext-only, the
//!   interval rule gates OFFERING, and decryption is bounded by harness-held
//!   keys (offered vs decryptable).

use tier_proof::identity::Signer;
use tier_proof::records::{self, Record};
use tier_proof::roles::{self, EnvelopeStore, OfferReject};

const SCOPE: &str = "scope:backplane";

fn stored(signer: &Signer, pos: u64, text: &str) -> (tier_proof::envelope::Envelope, u64) {
    let e = records::seal(signer, vec![], &Record::Message { scope: SCOPE.into(), text: text.into() });
    (e, pos)
}

#[test]
fn converged_set_dedups_by_envelope_hash_across_transports() {
    let a = Signer::from_seed([70u8; 32]);
    let (only_swarm, _) = stored(&a, 0, "via swarm");
    let (only_ds, _) = stored(&a, 1, "via ds");
    let (both, _) = stored(&a, 2, "via both paths");

    // The DS store and the swarm store each hold a different exclusive envelope,
    // plus one they both received (two delivery paths).
    let mut ds = EnvelopeStore::new();
    ds.insert(only_ds.clone(), 1);
    ds.insert(both.clone(), 2);

    let mut swarm = EnvelopeStore::new();
    swarm.insert(only_swarm.clone(), 0);
    swarm.insert(both.clone(), 2);

    let converged = roles::converge(&[&ds, &swarm]);
    let ids: Vec<String> = converged.iter().map(|e| e.identity_hex()).collect();
    assert_eq!(converged.len(), 3, "exactly three distinct envelopes");
    for e in [&only_swarm, &only_ds, &both] {
        assert_eq!(ids.iter().filter(|id| **id == e.identity_hex()).count(), 1, "each appears exactly once");
    }
}

/// A store spanning positions 0..5 for a member whose interval is [2, 5) (joined
/// at 2, revoked at 5).
fn interval_store() -> EnvelopeStore {
    let a = Signer::from_seed([71u8; 32]);
    let mut store = EnvelopeStore::new();
    for pos in 0..7u64 {
        let (e, p) = stored(&a, pos, &format!("msg{pos}"));
        store.insert(e, p);
    }
    store
}

#[test]
fn interval_backfill_offers_exactly_the_proven_window() {
    let store = interval_store();
    let member_intervals = vec![(2u64, Some(5u64))]; // joined at 2, revoked at 5

    let offered = roles::offer_interval(&store, &member_intervals, (2, 5)).expect("offer");
    let positions: Vec<u64> = offered.iter().map(|(_, p)| *p).collect();
    assert_eq!(positions, vec![2, 3, 4], "exactly the [J,R) window, no pre-J, no post-cut");
}

#[test]
fn pre_join_history_is_refused() {
    let store = interval_store();
    let member_intervals = vec![(2u64, Some(5u64))];
    // Asking for [0,5) exceeds the proven interval — not covered.
    assert_eq!(
        roles::offer_interval(&store, &member_intervals, (0, 5)),
        Err(OfferReject::NotProven),
        "a window reaching before the join is refused (offering-side)",
    );
}

#[test]
fn post_revocation_new_material_is_refused() {
    let store = interval_store();
    let member_intervals = vec![(2u64, Some(5u64))];
    // Asking for [2,7) reaches past the cut at 5 — not covered.
    assert_eq!(
        roles::offer_interval(&store, &member_intervals, (2, 7)),
        Err(OfferReject::NotProven),
        "material after the revocation cut is never offered",
    );
}

#[test]
fn non_member_backfill_is_refused_outright() {
    let store = interval_store();
    assert_eq!(
        roles::offer_interval(&store, &[], (0, 7)),
        Err(OfferReject::NotAMember),
        "a non-member (no proven interval) is refused outright",
    );
}

#[test]
fn sealed_scope_store_is_ciphertext_only_offered_vs_decryptable() {
    // The harness holds the key; the convergence node never does.
    let key = b"harness-held-seal-key-0123456789";
    let a = Signer::from_seed([72u8; 32]);

    let mut sealed = roles::SealedStore::new();
    for pos in 0..5u64 {
        let (e, p) = stored(&a, pos, &format!("secret{pos}"));
        let plaintext = tier_proof::canonical::to_canonical(&e).expect("encode");
        sealed.insert(roles::seal(key, &plaintext), p);
    }

    // Interval gates OFFERING even though the node cannot read the content.
    let member_intervals = vec![(1u64, Some(4u64))];
    let offered = roles::offer_sealed_interval(&sealed, &member_intervals, (1, 4)).expect("offer");
    let positions: Vec<u64> = offered.iter().map(|(_, p)| *p).collect();
    assert_eq!(positions, vec![1, 2, 3], "offered = the proven window (metadata only)");

    // Offered ciphertext is decryptable ONLY with the harness key.
    let (cipher, _pos) = &offered[0];
    let opened = roles::open(key, cipher);
    let reencoded: tier_proof::envelope::Envelope = ciborium::from_reader(opened.as_slice()).expect("decode");
    assert!(reencoded.verify().is_ok(), "harness key opens the offered ciphertext");

    // A wrong key does not recover the plaintext (the node, keyless, cannot read).
    let wrong = roles::open(b"the-wrong-key-aaaaaaaaaaaaaaaaaaa", cipher);
    assert_ne!(wrong, opened, "without the key the store's bytes are opaque");
}
