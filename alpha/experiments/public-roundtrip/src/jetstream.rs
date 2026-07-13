//! Jetstream consumer: connect to a public Jetstream endpoint, filtered
//! server-side to our experimental collection (and optionally our test DID), and
//! surface the raw commit events. We intentionally parse the *raw* JSON shape
//! Jetstream emits — that shape (field by field) is a primary thing the
//! experiment reports back and compares against any assumed `RecordEvent` struct.

use anyhow::{Context, Result};
use futures_util::StreamExt;
use serde::Deserialize;
use serde_json::Value;
use tokio::sync::mpsc;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

pub const DEFAULT_JETSTREAM: &str = "wss://jetstream2.us-east.bsky.network/subscribe";

/// A raw Jetstream event envelope. Optional fields are modelled as `Option` so
/// we can *observe* which are present/absent rather than assume them.
#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)] // `time_us`/`kind` are modelled to observe the raw event shape.
pub struct JetstreamEvent {
    pub did: String,
    #[serde(rename = "time_us")]
    pub time_us: u64,
    pub kind: String,
    pub commit: Option<Commit>,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)] // `rev` is modelled to observe the raw event shape.
pub struct Commit {
    pub rev: Option<String>,
    pub operation: String, // "create" | "update" | "delete"
    pub collection: String,
    pub rkey: String,
    pub cid: Option<String>,        // absent on delete
    pub record: Option<Value>,      // absent on delete
}

/// What the consumer hands back to the caller, with local receive timestamps so
/// we can measure observed propagation latency. `recv_instant` is monotonic
/// (immune to wall-clock skew) and captured at parse time; `recv_unix_ms` is the
/// wall-clock equivalent for human-readable logging.
#[derive(Debug, Clone)]
pub struct ReceivedEvent {
    pub recv_unix_ms: u128,
    pub recv_instant: std::time::Instant,
    pub raw: Value,
    pub event: JetstreamEvent,
}

/// Build the filtered subscribe URL, percent-encoding query values (DIDs
/// contain `:`, a reserved character). Uses the `url` crate rather than string
/// concatenation so reserved characters are escaped correctly.
pub fn build_url(base: &str, collection: &str, dids: &[String]) -> String {
    let mut url = url::Url::parse(base).expect("invalid Jetstream base URL");
    {
        let mut qp = url.query_pairs_mut();
        qp.append_pair("wantedCollections", collection);
        for did in dids {
            qp.append_pair("wantedDids", did);
        }
    }
    url.to_string()
}

/// Like `build_url`, plus an optional resume `cursor` (a `time_us` value).
/// Jetstream replays commits at/after the cursor, so a restarted consumer
/// misses nothing in the gap.
pub fn build_url_cursor(base: &str, collection: &str, dids: &[String], cursor: Option<u64>) -> String {
    let mut url = url::Url::parse(base).expect("invalid Jetstream base URL");
    {
        let mut qp = url.query_pairs_mut();
        qp.append_pair("wantedCollections", collection);
        for did in dids {
            qp.append_pair("wantedDids", did);
        }
        if let Some(c) = cursor {
            qp.append_pair("cursor", &c.to_string());
        }
    }
    url.to_string()
}

/// Connect and stream filtered events into `tx`. Sends `()` on `ready_tx` once
/// the socket is open. Runs until `tx` is dropped or the socket closes.
pub async fn consume(
    url: String,
    tx: mpsc::Sender<ReceivedEvent>,
    ready_tx: Option<tokio::sync::oneshot::Sender<()>>,
) -> Result<()> {
    let (ws, _resp) = connect_async(&url).await.context("connecting to Jetstream")?;
    if let Some(rt) = ready_tx {
        let _ = rt.send(());
    }
    let (_write, mut read) = ws.split();
    while let Some(msg) = read.next().await {
        let msg = msg.context("reading Jetstream frame")?;
        let text = match msg {
            Message::Text(t) => t.to_string(),
            Message::Binary(b) => String::from_utf8_lossy(&b).to_string(),
            Message::Close(_) => break,
            _ => continue,
        };
        let raw: Value = match serde_json::from_str(&text) {
            Ok(v) => v,
            Err(_) => continue,
        };
        let event: JetstreamEvent = match serde_json::from_value(raw.clone()) {
            Ok(e) => e,
            Err(_) => continue,
        };
        let recv_instant = std::time::Instant::now();
        let recv_unix_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        if tx.send(ReceivedEvent { recv_unix_ms, recv_instant, raw, event }).await.is_err() {
            break; // receiver gone
        }
    }
    Ok(())
}
