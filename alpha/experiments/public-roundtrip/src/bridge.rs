//! Extension #6: the source-agnostic boundary an AppView ingests, plus the seam
//! where experiment 3a's private/local path plugs in.
//!
//! The `compare` command revealed that the public (Jetstream) event and the
//! local path's assumed `RecordEvent` have *different shapes*. The honest
//! resolution is a single normalized event that both sources map into. This
//! module is that boundary: `NormalizedEvent::from_jetstream` is the public
//! mapping; the `LocalPath` trait is where 3a's real private path implements the
//! same mapping. `InProcessLocalPath` is a baseline stand-in until 3a lands here.

use crate::jetstream::JetstreamEvent;
use crate::lexicon::POST_NSID;
use anyhow::Result;
use serde::Serialize;
use serde_json::Value;
use std::time::{Duration, Instant};

/// The single shape an AppView ingest consumes, regardless of source. Both the
/// public path and the local path normalize *into* this.
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

impl NormalizedEvent {
    /// PUBLIC-path boundary: map a raw Jetstream commit event into the normalized
    /// shape. Note we must *construct* the AT-URI — Jetstream does not supply one
    /// — and lift `collection/rkey/cid/record/operation` out of the nested
    /// `commit` object. This is exactly the normalization #6 showed is required.
    pub fn from_jetstream(ev: &JetstreamEvent) -> Option<Self> {
        let c = ev.commit.as_ref()?;
        Some(NormalizedEvent {
            uri: format!("at://{}/{}/{}", ev.did, c.collection, c.rkey),
            did: ev.did.clone(),
            collection: c.collection.clone(),
            rkey: c.rkey.clone(),
            cid: c.cid.clone(),
            record: c.record.clone(),
            operation: c.operation.clone(),
            source: "public:jetstream".into(),
        })
    }

    /// The sorted set of JSON keys this event exposes — used to prove that two
    /// sources normalized to the *same* shape.
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

/// The seam where experiment 3a's private/local path plugs in. A real
/// implementation writes `record` through the private path and returns the
/// normalized event its ingest emits, plus the observed write→read latency.
///
/// To complete the true cross-experiment diff, implement this against 3a's
/// private path and hand it to `compare` in place of `InProcessLocalPath`.
pub trait LocalPath {
    fn roundtrip(&self, did: &str, record: &Value) -> Result<(NormalizedEvent, Duration)>;
}

/// Baseline stand-in: a direct in-process write→read with no network hop.
pub struct InProcessLocalPath;

impl LocalPath for InProcessLocalPath {
    fn roundtrip(&self, did: &str, record: &Value) -> Result<(NormalizedEvent, Duration)> {
        let micros = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_micros();
        let rkey = format!("local-{micros}");
        let t0 = Instant::now();
        // Direct write→read: emit the normalized event immediately.
        let ev = NormalizedEvent {
            uri: format!("at://{did}/{POST_NSID}/{rkey}"),
            did: did.to_string(),
            collection: POST_NSID.into(),
            rkey,
            cid: None, // a real local path supplies/derives its own content id
            record: Some(record.clone()),
            operation: "create".into(),
            source: "local:in-process".into(),
        };
        let latency = t0.elapsed();
        Ok((ev, latency))
    }
}
