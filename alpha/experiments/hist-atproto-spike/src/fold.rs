//! B4 — the fold: repo/firehose/rkey delivery order MUST NOT influence the
//! folded state; convergence ordering is the predecessor chain, only ever.
//!
//! This is the ordering rider (lib.rs; GROUPS.md v2 A.7) as executable code.
//! The structural half lives in [`crate::delivery`] (the cursor cannot be
//! read by this module — enforced by privacy, no accessor, no `Ord`); the
//! behavioral half is this fold: buffer-until-chained per subspace, so any
//! delivery permutation of the same entry set folds to the same state.

use crate::delivery::Delivered;
use crate::envelope::{Digest, Envelope, GENESIS_MARKER};
use std::collections::BTreeMap;

/// Converged per-subspace chains. Equality/digest is over chain content in
/// chain order — never over arrival order.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FoldState {
    /// subspace → predecessor-linked chain, genesis-first.
    chains: BTreeMap<Digest, Vec<Envelope>>,
    /// Entries whose predecessor has not yet arrived (keyed by predecessor).
    pending: BTreeMap<Digest, Vec<Envelope>>,
}

impl FoldState {
    pub fn chains(&self) -> &BTreeMap<Digest, Vec<Envelope>> {
        &self.chains
    }

    pub fn pending_count(&self) -> usize {
        self.pending.values().map(Vec::len).sum()
    }

    /// State digest: blake3 over the canonical bytes of every chained
    /// envelope, subspaces in sorted order, each chain in chain order.
    pub fn digest(&self) -> Digest {
        let mut h = blake3::Hasher::new();
        for (subspace, chain) in &self.chains {
            h.update(b"chain");
            h.update(subspace);
            for e in chain {
                h.update(&e.canonical_bytes());
            }
        }
        *h.finalize().as_bytes()
    }
}

/// Fold a delivery sequence into converged chain state: buffer-until-chained
/// per subspace, ordering by the in-payload predecessor chain only. (The B4
/// red run captured a deliberately order-sensitive fold — append-in-arrival-
/// order — producing permutation-dependent state before this form went
/// green.)
pub fn fold(deliveries: impl IntoIterator<Item = Delivered>) -> FoldState {
    let mut state = FoldState::default();
    for d in deliveries {
        let env = d.env; // the cursor is structurally unreadable here (delivery.rs)
        let chain = state.chains.entry(env.subspace).or_default();
        let head = chain.last().map(|e| e.entry_digest).unwrap_or(GENESIS_MARKER);
        if env.predecessor == head {
            chain.push(env);
            drain_chained(chain, &mut state.pending);
        } else {
            state.pending.entry(env.predecessor).or_default().push(env);
        }
    }
    state
}

/// Extend `chain` from `pending` as far as links allow (buffer-until-chained
/// helper).
fn drain_chained(
    chain: &mut Vec<Envelope>,
    pending: &mut BTreeMap<Digest, Vec<Envelope>>,
) {
    loop {
        let head = chain.last().map(|e| e.entry_digest).unwrap_or(GENESIS_MARKER);
        let Some(mut nexts) = pending.remove(&head) else {
            return;
        };
        // Single-writer per subspace (§H): at most one legitimate successor.
        // Deterministic pick + re-queue of any extras keeps the fold total.
        nexts.sort();
        let next = nexts.remove(0);
        if !nexts.is_empty() {
            pending.insert(head, nexts);
        }
        chain.push(next);
    }
}
