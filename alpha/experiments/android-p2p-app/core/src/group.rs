//! The group document: an Automerge CRDT holding a list of chat messages.
//!
//! ## Why the "shared genesis" rule matters
//!
//! Automerge identifies a nested object (like our `messages` list) by the
//! operation that *created* it (actor id + sequence number). If two peers each
//! call `put_object(ROOT, "messages", List)` on their own fresh document, they
//! create *two different* list objects that happen to share the key `"messages"`.
//! Merging those documents does **not** union their items; one list wins and the
//! other's messages are lost under a conflict.
//!
//! The canonical fix (and what real Automerge apps do) is a *shared genesis*: the
//! group creator builds the document with the empty `messages` list, and everyone
//! else starts from an **empty** document and obtains the list by merging the
//! creator's document in. After that first merge every peer is appending to the
//! *same* list object, so merges union cleanly and order-independently.

use anyhow::{Context, Result};
use automerge::transaction::Transactable;
use automerge::{AutoCommit, ObjId, ObjType, ReadDoc, ScalarValue, Value, ROOT};
use serde::{Deserialize, Serialize};

const MESSAGES_KEY: &str = "messages";

/// A single chat message as surfaced to the UI.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Message {
    pub author: String,
    pub text: String,
    /// Unix milliseconds, stored in the CRDT as an Automerge integer.
    pub ts: i64,
}

/// Wrapper around an Automerge document for one group.
pub struct GroupDoc {
    doc: AutoCommit,
}

impl GroupDoc {
    /// Create the document **as the group creator** (the shared genesis): an empty
    /// `messages` list under the document root.
    pub fn new_group() -> Result<Self> {
        let mut doc = AutoCommit::new();
        doc.put_object(ROOT, MESSAGES_KEY, ObjType::List)
            .context("create messages list")?;
        Ok(Self { doc })
    }

    /// Create an **empty** document for a joiner. It has no `messages` list yet;
    /// the joiner obtains it by merging the creator's document (its first sync).
    pub fn new_empty() -> Self {
        Self {
            doc: AutoCommit::new(),
        }
    }

    /// Load a document from a full snapshot produced by [`GroupDoc::snapshot`].
    pub fn load(bytes: &[u8]) -> Result<Self> {
        let doc = AutoCommit::load(bytes).context("load automerge snapshot")?;
        Ok(Self { doc })
    }

    /// Locate the shared `messages` list object id, if the document has one yet.
    fn messages_id(&self) -> Option<ObjId> {
        match self.doc.get(ROOT, MESSAGES_KEY) {
            Ok(Some((Value::Object(ObjType::List), id))) => Some(id),
            _ => None,
        }
    }

    /// Append a message authored locally. Fails if the `messages` list does not
    /// exist yet (i.e. a joiner who has not completed their first sync).
    pub fn post(&mut self, author: &str, text: &str, ts: i64) -> Result<()> {
        let messages = self
            .messages_id()
            .context("group not initialized yet (no messages list — sync first)")?;
        let index = self.doc.length(&messages);
        let msg = self
            .doc
            .insert_object(&messages, index, ObjType::Map)
            .context("insert message map")?;
        self.doc.put(&msg, "author", author.to_string())?;
        self.doc.put(&msg, "text", text.to_string())?;
        self.doc.put(&msg, "ts", ts)?;
        Ok(())
    }

    /// Read all messages in list order (oldest first).
    pub fn messages(&self) -> Vec<Message> {
        let Some(messages) = self.messages_id() else {
            return Vec::new();
        };
        let len = self.doc.length(&messages);
        let mut out = Vec::with_capacity(len);
        for i in 0..len {
            let Ok(Some((Value::Object(ObjType::Map), msg))) = self.doc.get(&messages, i) else {
                continue;
            };
            out.push(Message {
                author: self.str_field(&msg, "author"),
                text: self.str_field(&msg, "text"),
                ts: self.int_field(&msg, "ts"),
            });
        }
        out
    }

    fn str_field(&self, obj: &ObjId, key: &str) -> String {
        match self.doc.get(obj, key) {
            Ok(Some((Value::Scalar(s), _))) => match s.as_ref() {
                ScalarValue::Str(text) => text.to_string(),
                other => other.to_string(),
            },
            _ => String::new(),
        }
    }

    fn int_field(&self, obj: &ObjId, key: &str) -> i64 {
        match self.doc.get(obj, key) {
            Ok(Some((Value::Scalar(s), _))) => match s.as_ref() {
                ScalarValue::Int(v) => *v,
                ScalarValue::Uint(v) => *v as i64,
                ScalarValue::Timestamp(v) => *v,
                _ => 0,
            },
            _ => 0,
        }
    }

    /// Full binary snapshot of the document, suitable for sending to a peer or
    /// persisting. (For this basic experiment we exchange full snapshots; Automerge
    /// also supports incremental changes and the sync protocol for efficiency.)
    pub fn snapshot(&mut self) -> Vec<u8> {
        self.doc.save()
    }

    /// Merge a peer's snapshot into ours. This is the CRDT join: idempotent and
    /// independent of the order peers' changes arrive in.
    pub fn merge_snapshot(&mut self, bytes: &[u8]) -> Result<()> {
        let mut other = AutoCommit::load(bytes).context("load peer snapshot")?;
        self.doc.merge(&mut other).context("merge peer snapshot")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn post_and_read_roundtrip() {
        let mut g = GroupDoc::new_group().unwrap();
        g.post("alice", "hello", 1).unwrap();
        g.post("alice", "world", 2).unwrap();
        let msgs = g.messages();
        assert_eq!(msgs.len(), 2);
        assert_eq!(msgs[0].text, "hello");
        assert_eq!(msgs[1].author, "alice");
        assert_eq!(msgs[1].ts, 2);
    }

    #[test]
    fn joiner_starts_empty_then_obtains_list_by_merge() {
        // Creator builds the shared genesis and posts.
        let mut host = GroupDoc::new_group().unwrap();
        host.post("alice", "from host", 1).unwrap();

        // Joiner cannot post before syncing.
        let mut joiner = GroupDoc::new_empty();
        assert!(joiner.post("bob", "too early", 1).is_err());

        // First sync: joiner merges the host snapshot, gaining the shared list.
        joiner.merge_snapshot(&host.snapshot()).unwrap();
        joiner.post("bob", "from joiner", 2).unwrap();

        // Host merges the joiner's snapshot back.
        host.merge_snapshot(&joiner.snapshot()).unwrap();

        // Both converge to the union, in a consistent order.
        let h = host.messages();
        let j = joiner.messages();
        assert_eq!(h, j, "documents must converge after bidirectional merge");
        assert_eq!(h.len(), 2);
        let texts: Vec<_> = h.iter().map(|m| m.text.as_str()).collect();
        assert!(texts.contains(&"from host"));
        assert!(texts.contains(&"from joiner"));
    }
}
