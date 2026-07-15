//! Jetstream wire format + a `RecordSource` over it.
//!
//! Jetstream (Bluesky's JSON firehose) emits one JSON object per event. We model
//! the real shape and map each commit into the shared `RecordEvent`. The
//! indexer/server never see these wire types — only `RecordEvent`.
//!
//! Real Jetstream commit event:
//! ```jsonc
//! { "did": "did:plc:…", "time_us": 1727…, "kind": "commit",
//!   "commit": { "rev": "3k…", "operation": "create|update|delete",
//!               "collection": "<nsid>", "rkey": "<rkey>",
//!               "record": { … }, "cid": "bafy…" } }
//! ```
//! Non-commit events (`"kind": "identity"`, `"kind": "account"`) carry no
//! `commit` and are skipped. `time_us` doubles as the replay cursor.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::source::{Action, RecordEvent, RecordSource};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JetstreamEvent {
    pub did: String,
    pub time_us: u64,
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit: Option<Commit>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Commit {
    pub rev: String,
    pub operation: String,
    pub collection: String,
    pub rkey: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cid: Option<String>,
}

/// A `RecordSource` reading the Jetstream wire format from an NDJSON feed.
///
/// In 3a this was `LocalStackSource`. In real 3b the `feed` would be frames from
/// a live Jetstream WebSocket; here it is a recorded/synthetic NDJSON log of our
/// own records in the exact wire shape. The mapping logic — kind filtering,
/// collection filtering, operation→action, cursor tracking — is identical
/// either way.
pub struct JetstreamSource {
    feed: Vec<String>,
    collections: Vec<String>,
    cursor: u64,
}

impl JetstreamSource {
    /// Start from the beginning of the feed.
    pub fn new(feed: Vec<String>, collections: Vec<String>) -> Self {
        Self { feed, collections, cursor: 0 }
    }

    /// Resume from a saved cursor (`time_us`); events at or before it are skipped.
    pub fn from_cursor(feed: Vec<String>, collections: Vec<String>, cursor: u64) -> Self {
        Self { feed, collections, cursor }
    }

    /// The cursor after the last processed event (persist this to resume).
    pub fn cursor(&self) -> u64 {
        self.cursor
    }
}

impl RecordSource for JetstreamSource {
    fn events(&mut self) -> Vec<RecordEvent> {
        let mut out = Vec::new();
        for line in &self.feed {
            let ev: JetstreamEvent = match serde_json::from_str(line) {
                Ok(e) => e,
                Err(_) => continue, // tolerate malformed frames, like a real consumer
            };
            // Cursor: only events strictly after the cursor (resumability).
            if ev.time_us <= self.cursor {
                continue;
            }
            // Skip non-commit events (identity/account).
            let Some(commit) = ev.kind.eq("commit").then_some(()).and(ev.commit.clone()) else {
                self.cursor = ev.time_us;
                continue;
            };
            // Only the collections this AppView indexes.
            if !self.collections.contains(&commit.collection) {
                self.cursor = ev.time_us;
                continue;
            }
            let action = match commit.operation.as_str() {
                "create" => Action::Create,
                "update" => Action::Update,
                "delete" => Action::Delete,
                _ => {
                    self.cursor = ev.time_us;
                    continue;
                }
            };
            out.push(RecordEvent {
                action,
                did: ev.did,
                collection: commit.collection,
                rkey: commit.rkey,
                cid: commit.cid,
                record: commit.record.unwrap_or(Value::Null),
                observed_at: (ev.time_us / 1000) as i64,
            });
            self.cursor = ev.time_us;
        }
        out
    }
}

// ---------------------------------------------------------------------------
// Synthetic feed generator — produces the exact wire format from our records.
// (Stands in for frames off a live Jetstream socket; see module docs.)
// ---------------------------------------------------------------------------

/// Build one NDJSON commit-event line. `record`/`cid` are omitted for deletes.
pub fn commit_line(
    did: &str,
    time_us: u64,
    operation: &str,
    collection: &str,
    rkey: &str,
    record: Option<&Value>,
    cid: Option<&str>,
) -> String {
    let ev = JetstreamEvent {
        did: did.to_string(),
        time_us,
        kind: "commit".to_string(),
        commit: Some(Commit {
            rev: crate::record::new_tid(),
            operation: operation.to_string(),
            collection: collection.to_string(),
            rkey: rkey.to_string(),
            record: record.cloned(),
            cid: cid.map(str::to_string),
        }),
    };
    serde_json::to_string(&ev).unwrap()
}

/// Build a non-commit (`identity`) event line, which the source must skip.
pub fn identity_line(did: &str, time_us: u64) -> String {
    let ev = JetstreamEvent {
        did: did.to_string(),
        time_us,
        kind: "identity".to_string(),
        commit: None,
    };
    serde_json::to_string(&ev).unwrap()
}
