//! `JetstreamSource`: a real consumer of the live public atproto Jetstream.
//!
//! Connects over WebSocket (wss) to a public Jetstream instance, server-side
//! filtered to one or more collection NSIDs, and yields normalized events via the
//! `RecordSource` trait. Bounded by a time window AND/OR a max commit count so the
//! run terminates cleanly.

use std::time::{Duration, Instant};

use anyhow::Context;
use futures_util::StreamExt;
use tokio_tungstenite::tungstenite::Message;

use crate::record_source::{parse_frame, ParseOutcome, RecordSource};

/// Public Jetstream host verified reachable in the connectivity preflight.
pub const DEFAULT_HOST: &str = "jetstream2.us-east.bsky.network";

pub struct JetstreamSource {
    pub host: String,
    pub wanted_collections: Vec<String>,
    /// Optional server-side DID filter (`wantedDids`). Empty = all repos.
    pub wanted_dids: Vec<String>,
    /// Optional replay cursor (`time_us`). Jetstream replays from this point then
    /// tails live — lets a consumer catch an event emitted just before connecting.
    pub cursor: Option<i64>,
    /// Stop after this many *commit* events (events that matched the model).
    pub max_commits: usize,
    /// Stop after this wall-clock window regardless of count.
    pub window: Duration,
}

impl JetstreamSource {
    pub fn new(
        collections: &[&str],
        max_commits: usize,
        window: Duration,
    ) -> Self {
        JetstreamSource {
            host: DEFAULT_HOST.to_string(),
            wanted_collections: collections.iter().map(|s| s.to_string()).collect(),
            wanted_dids: Vec::new(),
            cursor: None,
            max_commits,
            window,
        }
    }

    /// Server-side filter to a single repo DID — keeps volume tiny when watching
    /// for one account's own writes (used by the publish loop test).
    pub fn with_did(mut self, did: &str) -> Self {
        self.wanted_dids = vec![did.to_string()];
        self
    }

    /// Replay from a `time_us` cursor (then tail live).
    pub fn from_cursor(mut self, time_us: i64) -> Self {
        self.cursor = Some(time_us);
        self
    }

    /// The exact wss URL, with server-side collection and DID filtering.
    pub fn url(&self) -> String {
        let mut url = format!("wss://{}/subscribe", self.host);
        let mut first = true;
        for c in &self.wanted_collections {
            url.push(if first { '?' } else { '&' });
            url.push_str("wantedCollections=");
            url.push_str(c);
            first = false;
        }
        for d in &self.wanted_dids {
            url.push(if first { '?' } else { '&' });
            url.push_str("wantedDids=");
            url.push_str(d);
            first = false;
        }
        if let Some(c) = self.cursor {
            url.push(if first { '?' } else { '&' });
            url.push_str("cursor=");
            url.push_str(&c.to_string());
        }
        url
    }
}

impl RecordSource for JetstreamSource {
    async fn run(
        &mut self,
        mut on_outcome: impl FnMut(ParseOutcome) -> bool,
    ) -> anyhow::Result<()> {
        let url = self.url();
        println!("    connecting to: {url}");

        let (mut ws, response) = tokio_tungstenite::connect_async(&url)
            .await
            .with_context(|| format!("WebSocket connect_async to {url}"))?;
        println!(
            "    handshake OK: HTTP {} (Switching Protocols expected)",
            response.status().as_u16()
        );

        let start = Instant::now();
        let deadline = start + self.window;
        let mut commits = 0usize;

        loop {
            if commits >= self.max_commits {
                println!("    reached max_commits={}", self.max_commits);
                break;
            }
            let remaining = match deadline.checked_duration_since(Instant::now()) {
                Some(r) if !r.is_zero() => r,
                _ => {
                    println!("    reached time window={:?}", self.window);
                    break;
                }
            };

            let msg = match tokio::time::timeout(remaining, ws.next()).await {
                Err(_) => {
                    println!("    reached time window={:?}", self.window);
                    break;
                }
                Ok(None) => {
                    println!("    stream closed by server");
                    break;
                }
                Ok(Some(Ok(m))) => m,
                Ok(Some(Err(e))) => {
                    // Surface, don't crash — connection friction is a finding.
                    println!("    websocket error (continuing): {e}");
                    continue;
                }
            };

            let text = match msg {
                Message::Text(t) => t.as_str().to_string(),
                // We don't request compression, so we don't expect Binary; if it
                // shows up, try to treat it as UTF-8 text rather than crash.
                Message::Binary(b) => match String::from_utf8(b.to_vec()) {
                    Ok(s) => s,
                    Err(_) => {
                        on_outcome(ParseOutcome::Malformed {
                            error: "binary frame, not UTF-8 (compression?)".into(),
                            raw_text: String::new(),
                        });
                        continue;
                    }
                },
                Message::Ping(_) | Message::Pong(_) | Message::Frame(_) => continue,
                Message::Close(_) => {
                    println!("    server sent Close");
                    break;
                }
            };

            let outcome = parse_frame(&text);
            if matches!(outcome, ParseOutcome::Commit { .. }) {
                commits += 1;
            }
            if !on_outcome(outcome) {
                println!("    early stop requested by consumer");
                break;
            }
        }

        let elapsed = start.elapsed().as_secs_f64();
        println!(
            "    ingest window done: {commits} commits in {elapsed:.1}s ({:.1} commits/s)",
            commits as f64 / elapsed.max(0.001)
        );
        let _ = ws.close(None).await;
        Ok(())
    }
}
