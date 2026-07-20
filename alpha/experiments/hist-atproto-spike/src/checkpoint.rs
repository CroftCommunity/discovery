//! B7 — the checkpoint prune GATE: deleting index records below a checkpoint
//! is permitted only when the checkpoint marker record is present and
//! verifiable; prune-without-checkpoint is rejected.
//!
//! **§L's checkpoint construction remains OPEN.** This module tests the
//! *gate*, not the construction: `commitment_well_formed` is a placeholder
//! predicate over a fixture commitment shape, standing where the corroborated
//! §L commitment will stand once pinned. Nothing here closes, narrows, or
//! prejudges that open item — the mirror of `ing.croft.hist.checkpoint`'s
//! sketch note (HIST-ATPROTO-MATCHUP.md §3).

use crate::envelope::{Digest, Envelope};
use std::collections::BTreeMap;

/// The `ing.croft.hist.checkpoint` record shape (sketch mirror).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckpointRecord {
    pub subspace: Digest,
    /// Entries with `counter < below_counter` are prunable once this record
    /// verifies.
    pub below_counter: u64,
    /// Digest of the last pruned entry — the anchor reverse validation
    /// terminates at (§L).
    pub prefix_head: Digest,
    /// Placeholder for the §L corroborated commitment (construction OPEN).
    pub commitment: Vec<u8>,
}

/// The placeholder commitment the fixture mints — what the §L construction
/// will replace. Fixture shape: a keyed hash over (subspace, boundary, head).
pub fn fixture_commitment(subspace: &Digest, below_counter: u64, prefix_head: &Digest) -> Vec<u8> {
    let mut h = blake3::Hasher::new();
    h.update(b"hist-spike-checkpoint-placeholder (SECTION-L CONSTRUCTION OPEN)");
    h.update(subspace);
    h.update(&below_counter.to_le_bytes());
    h.update(prefix_head);
    h.finalize().as_bytes().to_vec()
}

/// Gate-level verification of the placeholder commitment shape. Tests the
/// GATE (a prune must present a verifiable marker); never the construction.
pub fn commitment_well_formed(cp: &CheckpointRecord) -> bool {
    cp.commitment == fixture_commitment(&cp.subspace, cp.below_counter, &cp.prefix_head)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PruneError {
    /// Prune attempted with no checkpoint marker present.
    NoCheckpoint,
    /// Marker present but for a different subspace.
    WrongSubspace,
    /// Marker's `prefix_head` does not match the store's entry at the
    /// boundary — the anchor would not verify.
    BoundaryMismatch,
    /// Marker's commitment fails gate-level verification.
    BadCommitment,
}

/// A minimal per-subspace envelope index (counter-keyed), standing in for
/// the store's record set.
#[derive(Debug, Default)]
pub struct EnvelopeIndex {
    entries: BTreeMap<(Digest, u64), Envelope>,
}

impl EnvelopeIndex {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, env: Envelope) {
        self.entries.insert((env.subspace, env.counter), env);
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn get(&self, subspace: &Digest, counter: u64) -> Option<&Envelope> {
        self.entries.get(&(*subspace, counter))
    }

    pub fn count_below(&self, subspace: &Digest, below: u64) -> usize {
        self.entries
            .range((*subspace, 0)..(*subspace, below))
            .count()
    }

    /// Prune every entry of `subspace` with counter < the checkpoint's
    /// boundary. Permitted ONLY when a present, verifiable checkpoint marker
    /// covers the prune — the B7 gate. (The red run captured the ungated
    /// form pruning unconditionally before this went green.) The gate order:
    /// presence, subspace, coverage + anchor match, commitment shape — and
    /// nothing is deleted unless every check passes.
    pub fn prune_below(
        &mut self,
        subspace: &Digest,
        below_counter: u64,
        checkpoint: Option<&CheckpointRecord>,
    ) -> Result<usize, PruneError> {
        let cp = checkpoint.ok_or(PruneError::NoCheckpoint)?;
        if &cp.subspace != subspace {
            return Err(PruneError::WrongSubspace);
        }
        // The marker must cover the requested prune, and its anchor must be
        // the store's own entry at the boundary (reverse validation will
        // terminate exactly there, §L).
        if cp.below_counter < below_counter {
            return Err(PruneError::BoundaryMismatch);
        }
        let anchor_ok = self
            .get(subspace, cp.below_counter - 1)
            .is_some_and(|e| e.entry_digest == cp.prefix_head);
        if !anchor_ok {
            return Err(PruneError::BoundaryMismatch);
        }
        if !commitment_well_formed(cp) {
            return Err(PruneError::BadCommitment);
        }
        let victims: Vec<(Digest, u64)> = self
            .entries
            .range((*subspace, 0)..(*subspace, below_counter))
            .map(|(k, _)| *k)
            .collect();
        for k in &victims {
            self.entries.remove(k);
        }
        Ok(victims.len())
    }
}
