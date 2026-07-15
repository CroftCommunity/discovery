//! The ingest abstraction — the load-bearing design seam of this phase.
//!
//! The indexer and server consume only `RecordEvent`s through `RecordSource`
//! and contain ZERO references to Automerge, MLS, encryption, or iroh. The
//! entire private stack sits behind this trait. In Phase 3b a
//! `JetstreamSource: RecordSource` populates the *same* `RecordEvent` from real
//! firehose commits — an additive ingest impl, not a redesign.
//!
//! ## `RecordEvent` vs. a real Jetstream commit event
//!
//! Jetstream emits, per commit, roughly:
//! ```jsonc
//! { "did": "...", "time_us": 172..., "kind": "commit",
//!   "commit": { "rev": "3k...", "operation": "create|update|delete",
//!               "collection": "<nsid>", "rkey": "<rkey>",
//!               "record": { ... }, "cid": "bafy..." } }
//! ```
//! `RecordEvent` mirrors the *essential* fields: `did`, `observed_at` (≈
//! `time_us`), `action` (≈ `operation`), `collection`, `rkey`, `record`, `cid`.
//!
//! Fields a real Jetstream event has that the local source does NOT — exactly
//! the surface Phase 3b must fill:
//!   * `rev`            — the repo revision/commit id (we have no MST/rev).
//!   * `cid`            — a real CIDv1 over DAG-CBOR (we synthesize a BLAKE3
//!                        stand-in, prefixed `b3-`).
//!   * cursor/sequence  — Jetstream's replayable cursor (`time_us`); our one-shot
//!                        replay has no resumable position.
//!   * `kind`           — Jetstream also emits `identity`/`account` events; we
//!                        only ever produce record commits.
//!   * time precision   — `time_us` is microseconds; we use millis.

use serde_json::Value;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::groupdoc;
use automerge::AutoCommit;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    Create,
    // Update/Delete are part of the Jetstream-parity surface and are handled by
    // the indexer; the local one-shot replay only ever emits Create. A live
    // source (3b) produces all three.
    #[allow(dead_code)]
    Update,
    #[allow(dead_code)]
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

/// A normalized record-level event, shaped like a Jetstream commit (see module
/// docs). The indexer and server only ever see this struct.
#[derive(Clone, Debug)]
pub struct RecordEvent {
    pub action: Action,
    /// Author DID — the authority of the AT-URI and the repo owner.
    pub did: String,
    /// Collection NSID.
    pub collection: String,
    /// Record key.
    pub rkey: String,
    /// Content id (a `b3-…` BLAKE3 stand-in here; a real CIDv1 in 3b).
    /// Carried for Jetstream parity; the indexer does not key on it yet.
    #[allow(dead_code)]
    pub cid: Option<String>,
    /// Record body JSON (`Value::Null` for deletes).
    pub record: Value,
    /// Observation time, millis since epoch (≈ Jetstream `time_us`).
    /// Carried for Jetstream parity; not yet used as a cursor.
    #[allow(dead_code)]
    pub observed_at: i64,
}

impl RecordEvent {
    /// `at://<did>/<collection>/<rkey>`.
    pub fn uri(&self) -> String {
        format!("at://{}/{}/{}", self.did, self.collection, self.rkey)
    }
}

/// An event source the AppView ingests from. Source-agnostic by construction.
pub trait RecordSource {
    /// Yield the source's events. This phase replays current state one-shot;
    /// a streaming source (real Jetstream) would yield a live, unbounded stream
    /// — the trait shape accommodates both (3b would return an iterator/stream).
    fn events(&mut self) -> Vec<RecordEvent>;
}

/// A `RecordSource` backed by our private encrypted-sync stack: it walks a
/// decrypted Automerge group document and replays every record as a Create.
///
/// NOTE: this is the *only* place the AppView touches the stack. One-shot
/// replay of current state (documented choice over incremental trickle) is the
/// first cut; a live source would emit on each applied Automerge change.
pub struct LocalStackSource<'a> {
    doc: &'a AutoCommit,
}

impl<'a> LocalStackSource<'a> {
    pub fn new(doc: &'a AutoCommit) -> Self {
        Self { doc }
    }
}

impl RecordSource for LocalStackSource<'_> {
    fn events(&mut self) -> Vec<RecordEvent> {
        let now = now_millis();
        groupdoc::list_all(self.doc)
            .into_iter()
            .map(|(did, collection, rkey, json)| {
                let record: Value = serde_json::from_str(&json).unwrap_or(Value::Null);
                let cid = Some(format!("b3-{}", blake3::hash(json.as_bytes()).to_hex()));
                RecordEvent {
                    action: Action::Create,
                    did,
                    collection,
                    rkey,
                    cid,
                    record,
                    observed_at: now,
                }
            })
            .collect()
    }
}

pub fn now_millis() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}
