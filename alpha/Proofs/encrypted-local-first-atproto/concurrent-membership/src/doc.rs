//! CRDT layer — automerge 0.7.
//!
//! Chat is an Automerge document with a `messages` list, manipulated through the
//! AutoCommit API. New members bootstrap from a `save()` snapshot; existing
//! members catch up via incremental `get_changes` / `apply_changes`.
//!
//! 0.7-specific API points exercised here (vs. the older 0.6.x line):
//!   * `get_changes(&[ChangeHash]) -> Vec<Change>` returns *owned* changes
//!     (0.6.x returned borrowed `Vec<&Change>` needing `.cloned()`).
//!   * completeness is probed via `get_heads()` / `get_missing_deps()`.
//!
//! NOTE on a brief assumption that did not hold for automerge 0.7.4: on
//! `AutoCommit`, `get_missing_deps`, `get_changes`, and `get_heads` all take
//! `&mut self` (they flush the open auto-transaction first), not `&self`. The
//! brief stated `get_missing_deps` takes `&self` in 0.7 — that applies to the
//! lower-level `Automerge` type, not `AutoCommit`.

use automerge::transaction::Transactable;
use automerge::{AutoCommit, Change, ChangeHash, ObjType, ReadDoc, ROOT};

/// A fresh, empty chat document.
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

/// Append a chat message, creating the `messages` list on first use.
pub fn append_message(doc: &mut AutoCommit, text: &str) {
    let list = match doc.get(ROOT, "messages").expect("read failed") {
        Some((_, id)) => id,
        None => doc
            .put_object(ROOT, "messages", ObjType::List)
            .expect("failed to create messages list"),
    };
    let idx = doc.length(&list);
    doc.insert(&list, idx, text).expect("failed to insert message");
    doc.commit();
}

/// Read the `messages` list as plain strings.
pub fn read_messages(doc: &AutoCommit) -> Vec<String> {
    let Some((_, list)) = doc.get(ROOT, "messages").expect("read failed") else {
        return Vec::new();
    };
    let len = doc.length(&list);
    (0..len)
        .map(|i| {
            let (value, _) = doc
                .get(&list, i)
                .expect("read failed")
                .expect("missing list element");
            value.into_string().expect("message was not a string")
        })
        .collect()
}

/// The document's current heads (used as a peer's "have" set for sync).
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

/// Whether the document is causally complete.
///
/// Determined via missing-deps, NOT by checking whether `messages` is empty:
/// an incomplete (dependency-waiting) document and a legitimately empty one both
/// read as an empty list, so emptiness is not a completeness signal.
pub fn is_complete(doc: &mut AutoCommit) -> bool {
    doc.get_missing_deps(&[]).is_empty()
}
