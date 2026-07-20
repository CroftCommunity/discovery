//! E-Adversarial — the strict fold rejects tampered input.
//!
//! Purely offline; no live budget consumed.  Four tamperings, each producing
//! a specific `StrictFoldError` variant so the rejection reason is named
//! (rather than a "something's wrong" catch-all).  Property: for every
//! tampering, `strict_fold` returns `Err(<expected variant>)`.  Sanity: the
//! well-formed input yields `Ok(_)`.

mod common;

use hist_live::fold::{strict_fold, StrictFoldError};
use hist_live::record::{CidLink, HistEntry, Subspace};

fn build_chain(sub: &Subspace, count: u32) -> Vec<HistEntry> {
    let mut v = Vec::new();
    let mut prev = None;
    for c in 1..=count {
        let e = HistEntry::new(sub, c, prev, 16);
        prev = Some(e.cid());
        v.push(e);
    }
    v
}

#[test]
fn strict_fold_accepts_well_formed_chain() {
    let sub = Subspace("hist-live/adv/happy".to_string());
    let chain = build_chain(&sub, 5);
    let state = strict_fold(&chain).expect("well-formed chain accepted");
    let cs = state.chains.get(&sub.hash_prefix()).unwrap();
    assert_eq!(cs.head.as_ref().map(|(c, _)| *c), Some(5));
    assert!(cs.gaps.is_empty());
}

#[test]
fn strict_fold_rejects_duplicate_counter() {
    let sub = Subspace("hist-live/adv/dup".to_string());
    let mut chain = build_chain(&sub, 3);
    // Add a second entry with counter=2 (different content → different CID).
    let mut dup = HistEntry::new(&sub, 2, None, 32);
    dup.predecessor = chain[0].predecessor.clone();
    dup.size_hint = 999;
    chain.push(dup);
    let err = strict_fold(&chain).expect_err("duplicate MUST reject");
    assert!(matches!(
        err,
        StrictFoldError::DuplicateCounter { counter: 2, .. }
    ), "expected DuplicateCounter, got {:?}", err);
}

#[test]
fn strict_fold_rejects_first_entry_with_predecessor() {
    let sub = Subspace("hist-live/adv/first-has-pred".to_string());
    let mut chain = build_chain(&sub, 3);
    // Tamper: make chain[0] (counter=1) claim a predecessor.
    chain[0].predecessor = Some(CidLink {
        link: "bafyreiaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(),
    });
    let err = strict_fold(&chain).expect_err("first-has-pred MUST reject");
    assert!(matches!(
        err,
        StrictFoldError::FirstEntryHasPredecessor { .. }
    ), "got {:?}", err);
}

#[test]
fn strict_fold_rejects_missing_predecessor() {
    let sub = Subspace("hist-live/adv/missing-pred".to_string());
    let mut chain = build_chain(&sub, 3);
    // Tamper: strip counter=2's predecessor.
    chain[1].predecessor = None;
    let err = strict_fold(&chain).expect_err("missing-pred MUST reject");
    assert!(matches!(
        err,
        StrictFoldError::MissingPredecessor { counter: 2, .. }
    ), "got {:?}", err);
}

#[test]
fn strict_fold_rejects_predecessor_mismatch() {
    // The classic "reordered signed entries" forgery: counter=3 claims a
    // predecessor CID that isn't counter=2's CID.  Even if counter=2 IS
    // in the input, this is caught.
    let sub = Subspace("hist-live/adv/pred-mismatch".to_string());
    let mut chain = build_chain(&sub, 3);
    // Point counter=3 at the CID of the FIRST entry (skipping over 2).
    let wrong_pred_cid = chain[0].cid().to_string();
    chain[2].predecessor = Some(CidLink {
        link: wrong_pred_cid,
    });
    let err = strict_fold(&chain).expect_err("pred-mismatch MUST reject");
    assert!(matches!(
        err,
        StrictFoldError::PredecessorMismatch { counter: 3, .. }
    ), "got {:?}", err);
}

#[test]
fn strict_fold_rejects_predecessor_not_in_input() {
    // Backfill acceptance requires standing PLUS contiguity — a partial
    // set (counter=3 present but 2 absent) MUST be rejected.
    let sub = Subspace("hist-live/adv/orphan".to_string());
    let mut chain = build_chain(&sub, 3);
    chain.remove(1); // drop counter=2, keep 1 and 3
    let err = strict_fold(&chain).expect_err("orphan MUST reject");
    // Because counter=2 is absent from `cid_by_counter`, we hit the
    // PredecessorNotInInput branch (the map lookup fails).
    assert!(matches!(
        err,
        StrictFoldError::PredecessorNotInInput { counter: 3, .. }
    ), "got {:?}", err);
}

// The signature-alone-is-not-sufficient property from history-durability.md
// §I: two illegitimate branches — a stranger's perfectly-signed "history"
// and a tampered ordering of genuinely-signed entries — must both be caught.
// The tampered-ordering case is `predecessor_mismatch` above; the strict
// fold catches it via CID chain even when every entry is individually
// well-formed.
#[test]
fn strict_fold_shape_matches_history_durability_backfill_admission() {
    // Positive control: ordinary chain accepted.
    let sub = Subspace("hist-live/adv/shape".to_string());
    let chain = build_chain(&sub, 4);
    assert!(strict_fold(&chain).is_ok());

    // Negative: shuffled input order (write order != chain order) — but
    // predecessor links are correct.  This MUST be accepted (order of
    // presentation is irrelevant; content-bound chain is what matters).
    let mut shuffled = chain.clone();
    shuffled.reverse();
    assert!(
        strict_fold(&shuffled).is_ok(),
        "shuffled-but-correct chain MUST be accepted (order of presentation is irrelevant)"
    );
}

