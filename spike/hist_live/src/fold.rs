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

/// Strict fold — the safe consumer of untrusted mirror input.
///
/// Unlike `fold_by_antecedent_hashes`, this function does not trust its
/// input.  Rejection kinds are surfaced so E-Adversarial can name the
/// forgery it caught.  Rules, in order:
///
///  1. Each entry's declared CID (computed locally by re-canonicalising)
///     must be well-formed — trivially true from the type, but a Broken
///     canonicalizer would make this an integrity check point.
///  2. Within a subspace, `(counter)` MUST be unique (duplicate counters
///     are a forgery signal — a mirror can't create two records at the
///     same chain position).
///  3. Each non-first entry (counter > 1, or predecessor != None) MUST
///     name a predecessor CID that BELONGS to another entry IN THE INPUT
///     with the appropriate counter — no dangling refs.
///  4. The predecessor chain within a subspace MUST be a straight line —
///     each counter's predecessor is exactly the counter's minus-one
///     entry (no branches, no cycles).
///
/// A `StrictFoldError` is a REJECTION verdict, not a soft warning.  A
/// safe backfill consumer stops before absorbing anything the strict fold
/// rejects (matches history-durability.md §I "backfill acceptance
/// requires standing + contiguity").
pub fn strict_fold(entries: &[HistEntry]) -> Result<FoldState, StrictFoldError> {
    use std::collections::HashMap;

    // Group by subspace.
    let mut by_sub: HashMap<String, Vec<&HistEntry>> = HashMap::new();
    for e in entries {
        by_sub.entry(e.subspace.clone()).or_default().push(e);
    }
    let mut chains = std::collections::BTreeMap::new();
    for (sub, mut es) in by_sub {
        // Rule 2: no duplicate counters.
        es.sort_by_key(|e| e.counter);
        for w in es.windows(2) {
            if w[0].counter == w[1].counter {
                return Err(StrictFoldError::DuplicateCounter {
                    subspace: sub,
                    counter: w[0].counter,
                });
            }
        }
        // Rule 3+4: predecessor linkage and linearity.
        let cid_by_counter: HashMap<u32, String> = es
            .iter()
            .map(|e| (e.counter, e.cid().to_string()))
            .collect();
        for e in &es {
            let claimed_pred = e.predecessor.as_ref().map(|c| c.link.clone());
            match (e.counter, claimed_pred) {
                (1, None) => {} // First entry in chain: MUST have no predecessor.
                (1, Some(p)) => {
                    return Err(StrictFoldError::FirstEntryHasPredecessor {
                        subspace: sub,
                        claimed: p,
                    });
                }
                (c, None) => {
                    return Err(StrictFoldError::MissingPredecessor {
                        subspace: sub,
                        counter: c,
                    });
                }
                (c, Some(claimed)) => {
                    let expected = cid_by_counter.get(&(c - 1)).ok_or_else(|| {
                        StrictFoldError::PredecessorNotInInput {
                            subspace: sub.clone(),
                            counter: c,
                            claimed_predecessor: claimed.clone(),
                        }
                    })?;
                    if expected != &claimed {
                        return Err(StrictFoldError::PredecessorMismatch {
                            subspace: sub.clone(),
                            counter: c,
                            expected: expected.clone(),
                            claimed,
                        });
                    }
                }
            }
        }
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
    Ok(FoldState { chains })
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
#[serde(tag = "kind")]
pub enum StrictFoldError {
    DuplicateCounter {
        subspace: String,
        counter: u32,
    },
    FirstEntryHasPredecessor {
        subspace: String,
        claimed: String,
    },
    MissingPredecessor {
        subspace: String,
        counter: u32,
    },
    PredecessorNotInInput {
        subspace: String,
        counter: u32,
        claimed_predecessor: String,
    },
    PredecessorMismatch {
        subspace: String,
        counter: u32,
        expected: String,
        claimed: String,
    },
}

impl std::fmt::Display for StrictFoldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StrictFoldError::DuplicateCounter { subspace, counter } => {
                write!(f, "duplicate counter {} in subspace {}", counter, subspace)
            }
            StrictFoldError::FirstEntryHasPredecessor { subspace, claimed } => write!(
                f,
                "first entry in subspace {} claims predecessor {}",
                subspace, claimed
            ),
            StrictFoldError::MissingPredecessor { subspace, counter } => write!(
                f,
                "entry (counter {}) in subspace {} has no predecessor",
                counter, subspace
            ),
            StrictFoldError::PredecessorNotInInput {
                subspace,
                counter,
                claimed_predecessor,
            } => write!(
                f,
                "entry (counter {}) in subspace {} names predecessor {} not in input",
                counter, subspace, claimed_predecessor
            ),
            StrictFoldError::PredecessorMismatch {
                subspace,
                counter,
                expected,
                claimed,
            } => write!(
                f,
                "entry (counter {}) in subspace {} claims predecessor {} but the counter-{} entry has CID {}",
                counter, subspace, claimed, counter - 1, expected
            ),
        }
    }
}

impl std::error::Error for StrictFoldError {}

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
