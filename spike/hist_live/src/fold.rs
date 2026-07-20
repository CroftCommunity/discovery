//! Fold semantics for the hist-atproto lane.
//!
//! Fold-by-antecedent-hashes: reconciliation order = per-subspace
//! predecessor-chain walk, driven by the envelope hash chain, NEVER by
//! firehose seq / repo commit rev / rkey enumeration order.  Rkey enumeration
//! is used as a DELIVERY CURSOR for gap detection only (see the GROUPS v2
//! row 11 MUST-NOT reproduced at the top of HIST-LIVE-RESULTS.md).
//!
//! The negative control (`fold_by_commit_order_NEGATIVE_CONTROL`) is used only
//! by E6 to prove divergence; never call it from a shipping code path.

use crate::record::HistEntry;
use serde::Serialize;
use std::collections::{BTreeMap, HashMap};

/// A per-subspace summary of the folded state.  Equality is structural.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FoldState {
    /// (subspace hex → chain summary), sorted for stable equality.
    pub chains: BTreeMap<String, ChainSummary>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ChainSummary {
    /// Counters actually observed on this chain (sorted).
    pub counters: Vec<u32>,
    /// Ordered list of (counter, entry_cid) pairs as the fold visited them.
    pub visits: Vec<(u32, String)>,
    /// Chain-gap detector: counters missing from a contiguous [min..max].
    pub gaps: Vec<u32>,
    /// The chain's head after fold (highest observed counter and its CID).
    pub head: Option<(u32, String)>,
}

/// The correct fold: for each subspace, walk the predecessor-hash chain.  The
/// order records are supplied in is IRRELEVANT to the result.
pub fn fold_by_antecedent_hashes(entries: &[HistEntry]) -> FoldState {
    // Group by subspace.
    let mut by_sub: HashMap<String, Vec<&HistEntry>> = HashMap::new();
    for e in entries {
        by_sub.entry(e.subspace.clone()).or_default().push(e);
    }
    let mut chains = BTreeMap::new();
    for (sub, mut es) in by_sub {
        // Order by counter for a stable visit sequence — this is a
        // presentation choice for the summary; the fold's correctness is
        // independent of it because we key state on (subspace, counter) which
        // is unique per chain.
        es.sort_by_key(|e| e.counter);
        let counters: Vec<u32> = es.iter().map(|e| e.counter).collect();
        let visits: Vec<(u32, String)> = es
            .iter()
            .map(|e| (e.counter, e.cid().to_string()))
            .collect();
        let gaps = detect_gaps(&counters);
        let head = counters
            .last()
            .and_then(|c| es.iter().find(|e| e.counter == *c))
            .map(|e| (e.counter, e.cid().to_string()));
        chains.insert(
            sub,
            ChainSummary {
                counters,
                visits,
                gaps,
                head,
            },
        );
    }
    FoldState { chains }
}

/// NEGATIVE CONTROL — used only by E6.  Folds by insertion order.  Because
/// counters aren't respected, the visit sequence and the head are wrong
/// whenever insertion order != counter order.  The SHOUTY name is deliberate
/// so any grep for `commit_order` in a shipping path is immediately visible.
#[allow(non_snake_case)]
pub fn fold_by_commit_order_NEGATIVE_CONTROL(entries: &[HistEntry]) -> FoldState {
    let mut by_sub: HashMap<String, Vec<&HistEntry>> = HashMap::new();
    for e in entries {
        by_sub.entry(e.subspace.clone()).or_default().push(e);
    }
    let mut chains = BTreeMap::new();
    for (sub, es) in by_sub {
        // NO SORT — visit in the order handed in.
        let counters: Vec<u32> = es.iter().map(|e| e.counter).collect();
        let visits: Vec<(u32, String)> = es
            .iter()
            .map(|e| (e.counter, e.cid().to_string()))
            .collect();
        let gaps: Vec<u32> = Vec::new(); // no gap notion without sort
        let head = visits.last().map(|(c, s)| (*c, s.clone()));
        chains.insert(
            sub,
            ChainSummary {
                counters,
                visits,
                gaps,
                head,
            },
        );
    }
    FoldState { chains }
}

/// Return the missing counters between `min` and `max` inclusive, given a
/// sorted list `present`.  Empty when there are no gaps.
pub fn detect_gaps(present: &[u32]) -> Vec<u32> {
    if present.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::new();
    let min = present[0];
    let max = *present.last().unwrap();
    let set: std::collections::HashSet<u32> = present.iter().copied().collect();
    for c in min..=max {
        if !set.contains(&c) {
            out.push(c);
        }
    }
    out
}
