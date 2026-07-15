//! CRDT layer — automerge 0.7 (generic plumbing carried over).
//!
//! This phase bootstraps a member from a full `save()` snapshot (no incremental
//! sync is exercised here), so only the new/load/snapshot primitives are used;
//! the incremental helpers from earlier phases are intentionally omitted.

use automerge::AutoCommit;

/// A fresh, empty document.
pub fn new_doc() -> AutoCommit {
    AutoCommit::new()
}

/// Load a document from a `save()` snapshot.
pub fn load(snapshot: &[u8]) -> AutoCommit {
    AutoCommit::load(snapshot).expect("failed to load automerge snapshot")
}

/// Take a full snapshot suitable for bootstrapping a new member.
pub fn snapshot(doc: &mut AutoCommit) -> Vec<u8> {
    doc.save()
}
