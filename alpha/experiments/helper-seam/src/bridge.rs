//! The source-agnostic ingest boundary — an AppView consumes the SAME normalized
//! shape regardless of where a record came from.
//!
//! PROVENANCE (copied, not imported): the `NormalizedEvent` / `LocalPath` boundary
//! is copied from `alpha/experiments/public-roundtrip/src/bridge.rs` (extension
//! #6). The experiments convention is self-containment; a copied ~100-line
//! boundary with provenance beats a cross-experiment build coupling. This copy
//! ADDS the `from_group_message` constructor — the designed private side #6 left
//! as a `LocalPath` seam ("where experiment 3a's private/local path plugs in").
//! RUN-14 EXP-C is that private side, now live over real MLS.

use group_core::ChatMessage;
use serde::Serialize;
use serde_json::Value;

/// The single shape an AppView ingest consumes, regardless of source. Both the
/// public path and the private/helper path normalize *into* this. (Copied from
/// public-roundtrip; field-for-field identical so a row's origin is invisible to
/// the index.)
#[derive(Debug, Clone, Serialize)]
pub struct NormalizedEvent {
    pub uri: String,
    pub did: String,
    pub collection: String,
    pub rkey: String,
    pub cid: Option<String>,
    pub record: Option<Value>,
    pub operation: String,
    pub source: String,
}

/// The experiment-local collection for group posts (mirrors app.bsky.feed.post's
/// role: a text-bearing content record).
pub const GROUP_POST_NSID: &str = "app.stellin.groupPost";

impl NormalizedEvent {
    /// PRIVATE-path boundary (the new half): map a decrypted group `ChatMessage`
    /// into the normalized shape. The helper constructs the AT-URI from the group
    /// coordinates exactly as the public path constructs it from repo coordinates.
    pub fn from_group_message(
        group_id: &str,
        seq: i64,
        sender_did: &str,
        msg: &ChatMessage,
    ) -> Self {
        let rkey = format!("{group_id}-{seq}");
        NormalizedEvent {
            uri: format!("at://{sender_did}/{GROUP_POST_NSID}/{rkey}"),
            did: sender_did.to_string(),
            collection: GROUP_POST_NSID.to_string(),
            rkey,
            cid: None, // a real private path derives its own content id
            record: Some(serde_json::json!({ "text": msg.text, "sender": msg.sender })),
            operation: "create".to_string(),
            source: "private:group-helper".to_string(),
        }
    }

    /// PUBLIC-path constructor (stand-in for `from_jetstream`, whose input type
    /// lives in another crate): a public text post normalized into the same shape.
    pub fn public_post(did: &str, rkey: &str, text: &str) -> Self {
        NormalizedEvent {
            uri: format!("at://{did}/{GROUP_POST_NSID}/{rkey}"),
            did: did.to_string(),
            collection: GROUP_POST_NSID.to_string(),
            rkey: rkey.to_string(),
            cid: None,
            record: Some(serde_json::json!({ "text": text })),
            operation: "create".to_string(),
            source: "public:jetstream".to_string(),
        }
    }

    /// The text body, for indexing/search.
    pub fn text(&self) -> Option<String> {
        self.record
            .as_ref()?
            .get("text")
            .and_then(Value::as_str)
            .map(str::to_string)
    }

    /// The sorted set of JSON keys this event exposes — used to prove two sources
    /// normalized to the *same* shape (copied from public-roundtrip).
    pub fn key_set(&self) -> Vec<String> {
        match serde_json::to_value(self) {
            Ok(Value::Object(m)) => {
                let mut k: Vec<String> = m.keys().cloned().collect();
                k.sort();
                k
            }
            _ => Vec::new(),
        }
    }
}
