//! The real adapter (M6): fetches a live feed and parses it through the same
//! `wire::parse_timeline` the fake uses, behind the same `BlueskyPort`. Swapping
//! fake <-> real changes nothing in the core or the shell loop.
//!
//! Phase 0 reads the public AppView's `getAuthorFeed`, which returns the exact
//! `FeedViewPost` shape `getTimeline` does, so it needs no credentials. (Real
//! `getTimeline` is per-user and requires auth; that is a later concern. The
//! port hides the choice, keeping it reversible — philosophy §10.)
//!
//! The HTTPS GET is performed by shelling out to the system `curl` rather than
//! linking a Rust TLS stack. Two reasons: (1) every Rust TLS backend pulled in a
//! transitive crate a former employer's license policy flags as non-permissive, and (2)
//! `curl` is exactly the kind of "plain native client" the shell is meant to use
//! to perform effects (philosophy §6). It is blocking, so `fetch_timeline`'s
//! future completes on the first poll and the shell's minimal executor drives it.

use crate::port::{BlueskyPort, TimelinePage};
use crate::wire::parse_timeline;
use std::process::Command;
use std::time::Duration;

const DEFAULT_APPVIEW: &str = "https://public.api.bsky.app";
const DEFAULT_ACTOR: &str = "bsky.app";

/// A real, network-backed Bluesky source.
pub struct RealBluesky {
    appview: String,
    actor: String,
    page_limit: u32,
    timeout: Duration,
}

impl RealBluesky {
    /// Defaults to the public AppView and a well-known actor.
    pub fn new() -> Self {
        RealBluesky {
            appview: DEFAULT_APPVIEW.to_string(),
            actor: DEFAULT_ACTOR.to_string(),
            page_limit: 25,
            timeout: Duration::from_secs(15),
        }
    }

    /// Point at a specific actor's feed (the "timeline" to read in Phase 0).
    pub fn with_actor(mut self, actor: impl Into<String>) -> Self {
        self.actor = actor.into();
        self
    }
}

impl Default for RealBluesky {
    fn default() -> Self {
        RealBluesky::new()
    }
}

impl BlueskyPort for RealBluesky {
    async fn fetch_timeline(&self, cursor: Option<String>) -> Result<TimelinePage, String> {
        // Build the query with percent-encoded values so an actor/cursor can't
        // inject extra parameters. The URL begins with the fixed https base, so
        // it can never be read by curl as an option.
        let mut url = format!(
            "{}/xrpc/app.bsky.feed.getAuthorFeed?actor={}&limit={}",
            self.appview,
            encode(&self.actor),
            self.page_limit,
        );
        if let Some(c) = cursor.as_deref() {
            url.push_str("&cursor=");
            url.push_str(&encode(c));
        }

        let secs = self.timeout.as_secs().max(1).to_string();
        let output = Command::new("curl")
            .arg("--silent")
            .arg("--show-error")
            .arg("--fail") // non-2xx -> non-zero exit
            .arg("--max-time")
            .arg(&secs)
            .arg("--") // no arg after this is treated as an option
            .arg(&url)
            .output()
            .map_err(|_| {
                "could not run `curl` (is it installed and on PATH?)".to_string()
            })?;

        if !output.status.success() {
            // Report the exit status only; do not echo curl's stderr, which can
            // contain the full URL.
            return Err(match output.status.code() {
                Some(code) => format!("feed request failed (curl exit {code})"),
                None => "feed request failed (curl terminated by signal)".to_string(),
            });
        }

        let body = String::from_utf8(output.stdout)
            .map_err(|_| "feed response was not valid UTF-8".to_string())?;
        parse_timeline(&body)
    }
}

/// Minimal percent-encoding for a query-parameter value: keep the unreserved
/// set, encode everything else. Enough to carry handles and opaque cursors
/// safely without a url crate.
fn encode(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for &b in value.as_bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char)
            }
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}
