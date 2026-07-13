//! The ingest seam: a `RecordSource` yields a stream of normalized `RecordEvent`s.
//!
//! ─────────────────────────────────────────────────────────────────────────────
//!  THE HYPOTHESIS
//! ─────────────────────────────────────────────────────────────────────────────
//! `RecordEvent` below is my *prediction* of what a real atproto Jetstream commit
//! event carries. I am writing it the way a developer would design it from reading
//! docs/tutorials, BEFORE the field-by-field reality check. Part of this experiment
//! is checking that prediction against actual events off the live public network
//! and reporting every divergence (see `report.rs`).
//!
//! My explicit predictions about a Jetstream commit event:
//!   P1. It carries: an action (create/update/delete), the repo DID, the
//!       collection NSID, the rkey, the record body (JSON), the record CID, and a
//!       cursor/timestamp.
//!   P2. `cid` sits at the event top level, next to `did`.            [I expect this]
//!   P3. The cursor is a field called `time_us`; I am unsure of its units.
//!   P4. Every event on a commit stream is a "commit"; I do not model other kinds.
//!   P5. A `delete` event carries no record body and no cid.
//!   P6. The record body's timestamp lives at `record.createdAt` as an ISO string.
//!
//! The report flags, against real events: which predictions held, which fields were
//! present-but-unmodeled, and which expected fields were absent / differently named.

use serde_json::Value;

/// Operation carried by a commit event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Create,
    Update,
    Delete,
}

impl Action {
    pub fn parse(s: &str) -> Option<Action> {
        match s {
            "create" => Some(Action::Create),
            "update" => Some(Action::Update),
            "delete" => Some(Action::Delete),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Action::Create => "create",
            Action::Update => "update",
            Action::Delete => "delete",
        }
    }
}

/// My normalized, predicted shape of a record event.
#[derive(Debug, Clone)]
pub struct RecordEvent {
    pub action: Action,
    pub did: String,
    pub collection: String,
    pub rkey: String,
    /// Record body as JSON. `None` for deletes (prediction P5).
    pub record: Option<Value>,
    /// Record CID. `None` for deletes (prediction P5).
    pub cid: Option<String>,
    /// The Jetstream cursor (`time_us`), prediction P3.
    pub cursor: i64,
}

impl RecordEvent {
    /// Build the AT-URI: at://<did>/<collection>/<rkey>.
    pub fn at_uri(&self) -> String {
        format!("at://{}/{}/{}", self.did, self.collection, self.rkey)
    }
}

/// A bounded source of normalized record events.
///
/// Implemented by `JetstreamSource` (the only impl). Behind a trait on purpose:
/// the seam is good architecture AND a direct test — does a real network event
/// populate the same structure a local/stub source would?
pub trait RecordSource {
    /// Run the source until its bound (event count or time window) is reached,
    /// invoking `on_outcome` for *every* raw frame (including non-commit /
    /// malformed), so the caller can do the reality check and collect findings.
    /// The callback returns `false` to request an early stop (e.g. the publish
    /// loop saw the exact record it was waiting for).
    fn run(
        &mut self,
        on_outcome: impl FnMut(ParseOutcome) -> bool,
    ) -> impl std::future::Future<Output = anyhow::Result<()>>;
}

/// What a single raw Jetstream frame turned into when run through my predicted parser.
pub enum ParseOutcome {
    /// A commit event that matched my model. Carries the normalized event plus the
    /// raw JSON so the report can diff predicted-vs-real field shapes.
    Commit { event: RecordEvent, raw: Value },
    /// A frame whose `kind` was not `commit` (prediction P4 says these shouldn't
    /// happen on a commit stream — a finding if they do).
    NonCommit { kind: String },
    /// A frame that did not parse into my model at all. Logged, never fatal.
    Malformed { error: String, raw_text: String },
}

/// Attempt to parse a raw Jetstream frame into my predicted `RecordEvent`.
///
/// This is deliberately written against my *prediction*, then it records what it
/// actually found. It is lenient: anything unexpected is surfaced, never panics.
pub fn parse_frame(text: &str) -> ParseOutcome {
    let raw: Value = match serde_json::from_str(text) {
        Ok(v) => v,
        Err(e) => {
            return ParseOutcome::Malformed {
                error: format!("not JSON: {e}"),
                raw_text: text.chars().take(500).collect(),
            };
        }
    };

    let kind = raw.get("kind").and_then(Value::as_str).unwrap_or("<missing>");
    if kind != "commit" {
        // Prediction P4 violated if we ever get here.
        return ParseOutcome::NonCommit { kind: kind.to_string() };
    }

    // Pull the cursor. Prediction P3: `time_us` at top level.
    let cursor = raw.get("time_us").and_then(Value::as_i64).unwrap_or(-1);
    let did = raw.get("did").and_then(Value::as_str).unwrap_or("").to_string();

    // Prediction P2: I expect `cid` at the top level next to `did`. The parser
    // *first looks where I predicted*, then falls back to where it really is, so
    // the report can tell whether my prediction held.
    let top_level_cid = raw.get("cid").and_then(Value::as_str).map(str::to_string);

    let commit = match raw.get("commit") {
        Some(c) => c,
        None => {
            return ParseOutcome::Malformed {
                error: "kind=commit but no `commit` object".into(),
                raw_text: text.chars().take(500).collect(),
            };
        }
    };

    let action = commit
        .get("operation")
        .and_then(Value::as_str)
        .and_then(Action::parse);
    let action = match action {
        Some(a) => a,
        None => {
            return ParseOutcome::Malformed {
                error: format!(
                    "unknown/absent operation: {:?}",
                    commit.get("operation")
                ),
                raw_text: text.chars().take(500).collect(),
            };
        }
    };

    let collection = commit
        .get("collection")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string();
    let rkey = commit
        .get("rkey")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string();
    let record = commit.get("record").cloned();
    // Where the cid REALLY is (inside commit) — falls back to my predicted spot.
    let cid = commit
        .get("cid")
        .and_then(Value::as_str)
        .map(str::to_string)
        .or(top_level_cid);

    let event = RecordEvent {
        action,
        did,
        collection,
        rkey,
        record,
        cid,
        cursor,
    };
    ParseOutcome::Commit { event, raw }
}
