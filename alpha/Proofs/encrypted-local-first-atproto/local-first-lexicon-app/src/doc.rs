//! CRDT layer — automerge 0.7 (generic plumbing carried over from the slice).
//!
//! Application-specific structure (the atproto-style repo: collection -> rkey ->
//! record) lives in `repo.rs`; this module only provides the
//! snapshot/incremental/completeness primitives the sync path needs.
//!
//! 0.7 API note: on `AutoCommit`, `get_changes`, `get_heads`, and
//! `get_missing_deps` take `&mut self` (they flush the open auto-transaction);
//! `get_changes` returns *owned* `Vec<Change>`.

use automerge::{AutoCommit, Change, ChangeHash};

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

/// The document's current heads (a peer's "have" set for incremental sync).
pub fn heads(doc: &mut AutoCommit) -> Vec<ChangeHash> {
    doc.get_heads()
}

/// Owned changes since `have_deps` (0.7 returns owned `Vec<Change>`).
pub fn changes_since(doc: &mut AutoCommit, have_deps: &[ChangeHash]) -> Vec<Change> {
    doc.get_changes(have_deps)
}

/// Apply incremental changes from another peer.
pub fn apply(doc: &mut AutoCommit, changes: Vec<Change>) {
    doc.apply_changes(changes).expect("failed to apply changes");
}

/// Whether the document is causally complete — probed via missing-deps, NOT by
/// checking whether any collection is empty (an incomplete, dependency-waiting
/// document and a legitimately empty one are indistinguishable by content).
pub fn is_complete(doc: &mut AutoCommit) -> bool {
    doc.get_missing_deps(&[]).is_empty()
}

/// Length-prefixed (u32 BE) concatenation of each change's raw bytes, for
/// encrypting + storing a batch of incremental changes as one payload.
pub fn serialize_changes(changes: &[Change]) -> Vec<u8> {
    let mut out = Vec::new();
    for c in changes {
        let raw = c.raw_bytes();
        out.extend_from_slice(&(raw.len() as u32).to_be_bytes());
        out.extend_from_slice(raw);
    }
    out
}

/// Reverse of `serialize_changes`.
pub fn deserialize_changes(bytes: &[u8]) -> Vec<Change> {
    let mut out = Vec::new();
    let mut i = 0;
    while i + 4 <= bytes.len() {
        let len = u32::from_be_bytes(bytes[i..i + 4].try_into().unwrap()) as usize;
        i += 4;
        out.push(Change::from_bytes(bytes[i..i + len].to_vec()).expect("bad change bytes"));
        i += len;
    }
    out
}
