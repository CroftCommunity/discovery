//! AR-3 — backfill DoS resistance: a hostile donor cannot make the victim do
//! unbounded work, and rejected branches accumulate no state.
//!
//! The substantive DoS property is that rejection cost does not scale with the
//! attacker's payload: `backfill_import` checks `shares_lineage` BEFORE verifying
//! any message, so a foreign branch — however large — is rejected with ZERO
//! signature verifications; and a forged branch on a shared lineage is rejected
//! at the FIRST bad message, not after processing the whole thing. Correctness of
//! rejection (unauthorized author, non-contiguous) is green-real in
//! `backfill_adversarial`; this adds the cost bound.
//!
//! Transport note: the gossip/transport layer also caps per-message size
//! (iroh-gossip), so an oversized single payload is bounded below this layer; a
//! full cross-host flood measurement on the 3.8G box (node-3) is a follow-on.

use std::collections::BTreeMap;
use std::sync::atomic::{AtomicUsize, Ordering};

use lineage_core::dag::Lineage;
use lineage_core::ids::{Did, GenesisId};
use lineage_core::keys::{Sig, SigningIdentity, VerifyingIdentity};
use lineage_history::{BackfillError, BranchHistory, HistoryStore, Message};

fn did(s: &str) -> Did {
    Did::new(s)
}
fn g(seed: &[u8]) -> GenesisId {
    GenesisId::from_bytes(seed)
}

#[test]
fn ar3_huge_foreign_branch_rejected_with_zero_crypto() {
    // My lineage.
    let mut lineage = Lineage::new();
    let mine = g(b"my-branch");
    let myroot = g(b"my-root");
    lineage.add_root(myroot, [did("me")]);
    lineage.fork(myroot, mine, [did("me")]);

    // A hostile branch on a DIFFERENT lineage (no shared root), made enormous.
    let foreign = g(b"attacker-branch");
    let mut flood = BranchHistory::new(foreign);
    for seq in 0..10_000u64 {
        flood.push_raw(Message {
            author: did("attacker"),
            seq,
            branch: foreign,
            payload: vec![0u8; 256], // bulk
            sig: Sig([0u8; 64]),     // junk — must never be checked
        });
    }

    // A verifier that PANICS if invoked — proves no message was verified.
    let verify = |_d: &Did, _m: &[u8], _s: &Sig| -> bool {
        panic!("backfill must reject a foreign-lineage branch BEFORE any crypto");
    };

    let mut store = HistoryStore::new();
    let result = store.backfill_import(&flood, mine, &lineage, verify);
    assert_eq!(
        result,
        Err(BackfillError::ForeignGenesis),
        "a branch from an unshared lineage is rejected on the genesis boundary"
    );
    assert_eq!(store.branch_count(), 0, "rejected flood accumulates no state");
}

#[test]
fn ar3_forged_branch_on_shared_lineage_rejected_at_first_defect() {
    // Shared lineage; alice held standing.
    let alice = SigningIdentity::from_seed(did("alice"), 1);
    let mut dir: BTreeMap<Did, VerifyingIdentity> = BTreeMap::new();
    dir.insert(did("alice"), alice.verifying());

    let gb = g(b"branch");
    let mine = g(b"mine");
    let mut lineage = Lineage::new();
    lineage.add_root(gb, [did("alice")]);
    lineage.fork(gb, mine, [did("alice")]);

    // A long branch where message 0 has a BAD signature; the rest are well-formed.
    let mut forged = BranchHistory::new(gb);
    // msg 0: bad sig.
    forged.push_raw(Message {
        author: did("alice"),
        seq: 0,
        branch: gb,
        payload: b"tampered".to_vec(),
        sig: Sig([7u8; 64]),
    });
    for seq in 1..5_000u64 {
        let p = format!("m{seq}");
        let b = Message::signing_bytes(gb, seq, &did("alice"), p.as_bytes());
        forged.push_raw(Message { author: did("alice"), seq, branch: gb, payload: p.into_bytes(), sig: alice.sign(&b) });
    }

    // Count verifications; assert we stop at the first defect (bounded work).
    let calls = AtomicUsize::new(0);
    let verify = |d: &Did, m: &[u8], s: &Sig| {
        calls.fetch_add(1, Ordering::SeqCst);
        dir.get(d).is_some_and(|v| v.verify(m, s))
    };

    let mut store = HistoryStore::new();
    let result = store.backfill_import(&forged, mine, &lineage, verify);
    assert!(
        matches!(result, Err(BackfillError::BadSignature { seq: 0, .. })),
        "the forged first message is rejected"
    );
    assert_eq!(
        calls.load(Ordering::SeqCst),
        1,
        "rejection cost is bounded by the first defect, not the 5000-message payload"
    );
    assert_eq!(store.branch_count(), 0);
}

#[test]
fn ar3_repeated_rejections_do_not_accumulate_state() {
    let mut lineage = Lineage::new();
    let mine = g(b"my-branch");
    let myroot = g(b"my-root");
    lineage.add_root(myroot, [did("me")]);
    lineage.fork(myroot, mine, [did("me")]);

    let verify = |_d: &Did, _m: &[u8], _s: &Sig| true;
    let mut store = HistoryStore::new();
    for i in 0..1_000u64 {
        let foreign = g(format!("attacker-{i}").as_bytes());
        let mut b = BranchHistory::new(foreign);
        b.push_raw(Message { author: did("attacker"), seq: 0, branch: foreign, payload: vec![1, 2, 3], sig: Sig([0u8; 64]) });
        let _ = store.backfill_import(&b, mine, &lineage, verify);
    }
    assert_eq!(store.branch_count(), 0, "a thousand rejected foreign branches leave no residue");
}
