//! Stub-only property tests — these run in CI without any live env.
//!
//! Each corresponds to an assertion the live orchestration reuses; when a
//! property test goes red, the live suite is guaranteed to be already broken.

mod common;

use common::*;
use hist_live::budget::{Budget, BudgetCaps, BudgetError, Pacer};
use hist_live::canonical::{canonical_dag_cbor, cid_v1_dag_cbor};
use hist_live::fold::{
    detect_gaps, fold_by_antecedent_hashes,
    fold_by_commit_order_NEGATIVE_CONTROL,
};
use hist_live::leg::{ApplyWritesOp, LiveLegTrait};
use hist_live::record::{HistEntry, Rkey, Subspace, HIST_ENTRY_TYPE};

// --- E1 properties (stub) ---

#[test]
fn e1_canonical_dag_cbor_is_deterministic() {
    let sub = Subspace("hist-live/props/e1".to_string());
    let e = HistEntry::new(&sub, 1, None, 32);
    let a = canonical_dag_cbor(&e);
    let b = canonical_dag_cbor(&e);
    assert_eq!(a, b);
    assert_eq!(cid_v1_dag_cbor(&a), e.cid());
}

#[test]
fn e1_stub_getrecord_matches_local_cid() {
    let leg = stub_gentle();
    let sub = Subspace("hist-live/props/e1s".to_string());
    let entry = HistEntry::new(&sub, 1, None, 32);
    let rk = Rkey::from(&sub, 1);
    let ops = vec![ApplyWritesOp::Create {
        collection: HIST_ENTRY_TYPE.into(),
        rkey: rk.0.clone(),
        value: serde_json::to_value(&entry).unwrap(),
    }];
    leg.apply_writes(ops).unwrap();
    let got = leg.get_record(HIST_ENTRY_TYPE, &rk.0).unwrap();
    assert_eq!(got.cid, entry.cid().to_string());
}

// --- E2 properties: rkey sort order is (subspace-hash, counter) ---

#[test]
fn e2_rkey_ordering_is_stable_across_shuffles() {
    use rand_shuffle::shuffle;
    let sub_a = Subspace("hist-live/props/e2a".to_string());
    let sub_b = Subspace("hist-live/props/e2b".to_string());
    let mut all: Vec<Rkey> = (1..=18u32)
        .flat_map(|c| vec![Rkey::from(&sub_a, c), Rkey::from(&sub_b, c)])
        .collect();
    let mut sorted = all.clone();
    sorted.sort();
    // Property: for 5 different shuffles, sorting recovers the same order.
    for seed in 0..5u64 {
        shuffle(&mut all, seed);
        let mut copy = all.clone();
        copy.sort();
        assert_eq!(copy, sorted, "shuffle seed {} did not sort back to canonical", seed);
    }
}

#[test]
fn e2_chain_gap_detector_finds_missing_counter() {
    // Present: 1,2,4,5 — expected gap: 3.
    assert_eq!(detect_gaps(&[1, 2, 4, 5]), vec![3]);
    assert_eq!(detect_gaps(&[1, 2, 3, 4, 5]), vec![] as Vec<u32>);
    assert_eq!(detect_gaps(&[]), vec![] as Vec<u32>);
    assert_eq!(detect_gaps(&[10]), vec![] as Vec<u32>);
    assert_eq!(detect_gaps(&[1, 5]), vec![2, 3, 4]);
}

// --- Budget / pacer harness (E1..E8 all depend on this) ---

#[test]
fn budget_caps_deny_over_write_cap() {
    let b = Budget::new(BudgetCaps {
        writes: 5,
        blobs: 1,
        max_blob_bytes: 64,
        reads_cap: None,
    });
    assert!(b.charge_writes(3).is_ok());
    assert!(b.charge_writes(2).is_ok());
    match b.charge_writes(1) {
        Err(BudgetError::WritesExceeded { requested, cap }) => {
            assert_eq!(requested, 6);
            assert_eq!(cap, 5);
        }
        other => panic!("expected WritesExceeded, got {:?}", other),
    }
    // Ledger did NOT advance on the denied call.
    assert_eq!(b.snapshot().writes, 5);
}

#[test]
fn budget_caps_deny_over_blob_cap_and_oversize() {
    let b = Budget::new(BudgetCaps {
        writes: 100,
        blobs: 1,
        max_blob_bytes: 64,
        reads_cap: None,
    });
    assert!(b.charge_blob(64).is_ok());
    assert!(matches!(
        b.charge_blob(64),
        Err(BudgetError::BlobsExceeded { .. })
    ));
    let b2 = Budget::new(BudgetCaps {
        writes: 100,
        blobs: 3,
        max_blob_bytes: 64,
        reads_cap: None,
    });
    assert!(matches!(
        b2.charge_blob(65),
        Err(BudgetError::BlobTooLarge { .. })
    ));
}

#[test]
fn pacer_serializes_calls_and_holds_min_interval() {
    let pacer = Pacer::new(std::time::Duration::from_millis(50));
    let t0 = std::time::Instant::now();
    {
        let _g = pacer.acquire();
    }
    {
        let _g = pacer.acquire();
    }
    let elapsed = t0.elapsed();
    assert!(
        elapsed >= std::time::Duration::from_millis(50),
        "pacer did not enforce 50ms gap: {:?}",
        elapsed
    );
}

#[test]
fn stub_apply_writes_charges_budget_and_denies_at_cap() {
    let leg = stub_gentle();
    // Gentle cap = 100 writes.  Try to submit 101 in one op — MUST reject.
    let sub = Subspace("hist-live/props/cap".to_string());
    let ops: Vec<_> = (1..=101u32)
        .map(|c| ApplyWritesOp::Create {
            collection: HIST_ENTRY_TYPE.into(),
            rkey: Rkey::from(&sub, c).0,
            value: serde_json::to_value(&HistEntry::new(&sub, c, None, 8)).unwrap(),
        })
        .collect();
    let err = leg.apply_writes(ops).unwrap_err();
    assert!(matches!(
        err,
        hist_live::leg::XrpcError::Budget(hist_live::budget::BudgetError::WritesExceeded { .. })
    ));
    // And the ledger did not advance a single write.
    assert_eq!(leg.budget_snapshot().writes, 0);
}

// --- E5: mid-chain delete surfaces as a chain-gap ---

#[test]
fn e5_deleting_mid_chain_surfaces_gap_on_stub() {
    let leg = stub_gentle();
    let sub = Subspace("hist-live/props/e5".to_string());
    let mut prev = None;
    let mut entries: Vec<(Rkey, HistEntry)> = Vec::new();
    for c in 1..=5u32 {
        let e = HistEntry::new(&sub, c, prev, 8);
        prev = Some(e.cid());
        entries.push((Rkey::from(&sub, c), e));
    }
    let ops: Vec<_> = entries
        .iter()
        .map(|(rk, e)| ApplyWritesOp::Create {
            collection: HIST_ENTRY_TYPE.into(),
            rkey: rk.0.clone(),
            value: serde_json::to_value(e).unwrap(),
        })
        .collect();
    leg.apply_writes(ops).unwrap();
    // Delete counter 3.
    leg.apply_writes(vec![ApplyWritesOp::Delete {
        collection: HIST_ENTRY_TYPE.into(),
        rkey: entries[2].0 .0.clone(),
    }])
    .unwrap();
    // List and detect.
    let mut all: Vec<HistEntry> = Vec::new();
    let mut cursor = None;
    loop {
        let page = leg
            .list_records(HIST_ENTRY_TYPE, 10, cursor.as_deref(), false)
            .unwrap();
        for r in &page.records {
            let e: HistEntry = serde_json::from_value(r.value.clone()).unwrap();
            all.push(e);
        }
        if page.cursor.is_none() {
            break;
        }
        cursor = page.cursor;
    }
    let state = fold_by_antecedent_hashes(&all);
    let sub_state = state
        .chains
        .get(&Subspace("hist-live/props/e5".to_string()).hash_prefix())
        .unwrap();
    assert_eq!(sub_state.gaps, vec![3u32], "expected gap at 3");
}

// --- E6: fold-by-antecedent vs fold-by-commit-order divergence ---

#[test]
fn e6_negative_control_diverges_from_correct_fold_when_write_order_shuffled() {
    let sub = Subspace("hist-live/props/e6".to_string());
    // Build entries 1..=6 in counter order, then hand them to the folds in
    // {5, 3, 4, 1, 6, 2} order.
    let mut prev = None;
    let mut entries: Vec<HistEntry> = Vec::new();
    for c in 1..=6u32 {
        let e = HistEntry::new(&sub, c, prev, 8);
        prev = Some(e.cid());
        entries.push(e);
    }
    let shuffled_indices = [4usize, 2, 3, 0, 5, 1]; // → counters 5,3,4,1,6,2
    let shuffled: Vec<HistEntry> = shuffled_indices.iter().map(|&i| entries[i].clone()).collect();

    let correct = fold_by_antecedent_hashes(&shuffled);
    let canary = fold_by_commit_order_NEGATIVE_CONTROL(&shuffled);

    // Correct fold: head is counter=6.
    let correct_chain = correct
        .chains
        .get(&sub.hash_prefix())
        .expect("chain present");
    assert_eq!(correct_chain.head.as_ref().map(|(c, _)| *c), Some(6));
    // Canary: head is the LAST insertion, which is counter=2 (not 6).
    let canary_chain = canary.chains.get(&sub.hash_prefix()).expect("chain present");
    assert_eq!(canary_chain.head.as_ref().map(|(c, _)| *c), Some(2));
    assert_ne!(correct, canary, "negative control MUST diverge");
}

// --- Minimal xorshift shuffle so tests don't pull rand ---

mod rand_shuffle {
    pub fn shuffle<T>(v: &mut [T], seed: u64) {
        let mut s = if seed == 0 { 0x9e3779b97f4a7c15 } else { seed };
        for i in (1..v.len()).rev() {
            s ^= s << 13;
            s ^= s >> 7;
            s ^= s << 17;
            let j = (s as usize) % (i + 1);
            v.swap(i, j);
        }
    }
}
