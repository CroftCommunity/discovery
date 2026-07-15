//! Phase 2 experiments E2.6–E2.8 + fold/unfold (I7, I8, I9).
//!
//! History-as-navigable-tree: fresh-genesis inherits both parent logs as
//! read-only ancestry without reordering (E2.6/I7); consensual backfill of an
//! entitled branch verifies and imports while a forged branch is rejected
//! (E2.7/I8/I3); reconcile yields distinct navigable branches, never a merged
//! scroll (E2.8); folding is lossless and inert (I9).

use std::collections::BTreeMap;

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

/// A verifier backed by a known directory of public keys.
fn verifier(dir: &BTreeMap<Did, VerifyingIdentity>) -> impl Fn(&Did, &[u8], &Sig) -> bool + '_ {
    move |d: &Did, msg: &[u8], sig: &Sig| dir.get(d).is_some_and(|v| v.verify(msg, sig))
}

/// I9 — folding hides a branch from the daily view losslessly and inertly.
#[test]
fn i9_fold_is_lossless_and_inert() {
    let mut b = BranchHistory::new(g(b"branch"));
    let alice = SigningIdentity::from_seed(did("alice"), 1);
    b.append(&alice, b"one");
    b.append(&alice, b"two");

    b.fold();
    assert!(b.is_folded());
    assert_eq!(b.visible().len(), 0, "folded branch is inert (no visible messages)");
    assert_eq!(b.len(), 2, "but nothing is deleted");

    b.unfold();
    assert_eq!(b.visible().len(), 2, "unfolding restores full context");
}

/// E2.6 — a fresh-genesis merge inherits both parent logs as read-only
/// ancestry; no message is reordered (I7).
#[test]
fn e2_6_fresh_genesis_inherits_logs_unreordered() {
    let alice = SigningIdentity::from_seed(did("alice"), 1);
    let bob = SigningIdentity::from_seed(did("bob"), 1);
    let (ga, gb, gc) = (g(b"parentA"), g(b"parentB"), g(b"freshC"));

    // Two parent branches each accumulate their own ordered history.
    let mut parent_a = BranchHistory::new(ga);
    parent_a.append(&alice, b"A1");
    parent_a.append(&alice, b"A2");
    let mut parent_b = BranchHistory::new(gb);
    parent_b.append(&bob, b"B1");
    parent_b.append(&bob, b"B2");

    // Fresh genesis descends from both parents.
    let mut lineage = Lineage::new();
    lineage.add_root(ga, [did("alice")]);
    lineage.add_root(gb, [did("bob")]);
    lineage.recombine(ga, gb, gc, [did("alice"), did("bob")]);

    let mut dir = BTreeMap::new();
    dir.insert(did("alice"), alice.verifying());
    dir.insert(did("bob"), bob.verifying());

    let mut store = HistoryStore::new();
    store.branch_mut(gc); // the live branch
    store.backfill_import(&parent_a, gc, &lineage, verifier(&dir)).unwrap();
    store.backfill_import(&parent_b, gc, &lineage, verifier(&dir)).unwrap();

    // Both parent logs are present, intact and unreordered.
    let payloads: Vec<&[u8]> = store
        .branch(ga)
        .unwrap()
        .messages()
        .iter()
        .map(|m| m.payload.as_slice())
        .collect();
    assert_eq!(payloads, vec![b"A1".as_slice(), b"A2".as_slice()]);
    assert_eq!(store.branch(gb).unwrap().len(), 2);
}

/// E2.7 — an entitled branch (shared genesis, valid signatures) imports; a
/// forged branch (broken signature) and a foreign-genesis branch are rejected
/// (I8, I3).
#[test]
fn e2_7_backfill_verifies_and_rejects_forgeries() {
    let alice = SigningIdentity::from_seed(did("alice"), 1);
    let (ga, gb, mine) = (g(b"branchA"), g(b"branchB"), g(b"mine"));

    let mut dir = BTreeMap::new();
    dir.insert(did("alice"), alice.verifying());

    // Lineage: branchA shares a root with `mine`; branchB is on a foreign root.
    let mut lineage = Lineage::new();
    lineage.add_root(ga, [did("alice")]);
    lineage.fork(ga, mine, [did("alice")]);
    lineage.add_root(gb, [did("alice")]); // unrelated root

    // Entitled, well-formed donor -> imports.
    let mut donor = BranchHistory::new(ga);
    donor.append(&alice, b"hello");
    donor.append(&alice, b"world");
    let mut store = HistoryStore::new();
    assert!(store.backfill_import(&donor, mine, &lineage, verifier(&dir)).is_ok());

    // Foreign genesis -> rejected by entitlement check.
    let mut foreign = BranchHistory::new(gb);
    foreign.append(&alice, b"x");
    let mut store2 = HistoryStore::new();
    assert_eq!(
        store2.backfill_import(&foreign, mine, &lineage, verifier(&dir)),
        Err(BackfillError::ForeignGenesis)
    );

    // Tampered payload (signature no longer matches the changed bytes) ->
    // rejected. We keep the original signature but alter the payload.
    let mut tampered = BranchHistory::new(ga);
    let good = donor.messages()[0].clone();
    tampered.push_raw(Message { payload: b"TAMPERED".to_vec(), ..good });
    let mut store3 = HistoryStore::new();
    assert!(matches!(
        store3.backfill_import(&tampered, mine, &lineage, verifier(&dir)),
        Err(BackfillError::BadSignature { .. })
    ));
}

/// E2.8 — reconcile produces distinct navigable branches, NOT a merged scroll.
/// Guards against regressing into timestamp interleaving ("six tapes").
#[test]
fn e2_8_no_timestamp_interleave() {
    let alice = SigningIdentity::from_seed(did("alice"), 1);
    let bob = SigningIdentity::from_seed(did("bob"), 1);
    let (ga, gb, mine) = (g(b"A"), g(b"B"), g(b"mine"));

    let mut lineage = Lineage::new();
    lineage.add_root(g(b"root"), [did("alice")]);
    lineage.fork(g(b"root"), ga, [did("alice")]);
    lineage.fork(g(b"root"), gb, [did("bob")]);
    lineage.fork(g(b"root"), mine, [did("alice")]);

    let mut dir = BTreeMap::new();
    dir.insert(did("alice"), alice.verifying());
    dir.insert(did("bob"), bob.verifying());

    let mut a = BranchHistory::new(ga);
    a.append(&alice, b"a-only");
    let mut b = BranchHistory::new(gb);
    b.append(&bob, b"b-only");

    let mut store = HistoryStore::new();
    store.backfill_import(&a, mine, &lineage, verifier(&dir)).unwrap();
    store.backfill_import(&b, mine, &lineage, verifier(&dir)).unwrap();

    // Two distinct branches, each holding only its own messages — no interleave.
    assert_eq!(store.branch_count(), 2);
    assert_eq!(store.branch(ga).unwrap().messages()[0].payload, b"a-only");
    assert_eq!(store.branch(gb).unwrap().messages()[0].payload, b"b-only");
    // There is no API that returns a single merged-by-timestamp timeline.
}
