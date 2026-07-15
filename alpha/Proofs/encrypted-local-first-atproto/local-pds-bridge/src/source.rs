//! The ingest contract — IDENTICAL to Phase 3a's `source::{Action, RecordEvent,
//! RecordSource}`, so the byte-identical `indexer.rs` / `server.rs` compile
//! against it unchanged. Phase 3a's source impl was `LocalStackSource` (over the
//! encrypted CRDT doc); this phase's impl is `JetstreamSource` (over the real
//! Jetstream wire format) in `jetstream.rs`. Same contract, different source —
//! that is the whole point.

use serde_json::Value;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    Create,
    Update,
    Delete,
}

impl Action {
    pub fn as_str(self) -> &'static str {
        match self {
            Action::Create => "create",
            Action::Update => "update",
            Action::Delete => "delete",
        }
    }
}

/// A normalized record-level event (see `jetstream.rs` for the Jetstream wire
/// shape it is mapped from). The indexer and server only ever see this struct.
#[derive(Clone, Debug)]
pub struct RecordEvent {
    pub action: Action,
    pub did: String,
    pub collection: String,
    pub rkey: String,
    /// Real CIDv1 (DAG-CBOR) here; carried for parity, not yet keyed on by the indexer.
    #[allow(dead_code)]
    pub cid: Option<String>,
    pub record: Value,
    /// Derived from Jetstream `time_us`; carried for parity / future cursoring.
    #[allow(dead_code)]
    pub observed_at: i64,
}

impl RecordEvent {
    pub fn uri(&self) -> String {
        format!("at://{}/{}/{}", self.did, self.collection, self.rkey)
    }
}

/// An event source the AppView ingests from. Source-agnostic by construction.
pub trait RecordSource {
    fn events(&mut self) -> Vec<RecordEvent>;
}
