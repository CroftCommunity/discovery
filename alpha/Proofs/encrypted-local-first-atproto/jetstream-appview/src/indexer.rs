//! The indexer — a disposable SQLite projection of records arriving via
//! `RecordSource`. It validates each record against its lexicon on ingest, then
//! stores the full JSON plus a few extracted columns for querying. It contains
//! ZERO references to Automerge, MLS, encryption, or iroh — only `RecordEvent`,
//! the lexicon validator, and view structs.
//!
//! SQLite chosen via `rusqlite` (bundled) for a single-file, zero-setup,
//! rebuildable store — the standard AppView pattern (source of truth lives
//! elsewhere; the index is regenerable).

use rusqlite::{params, Connection};
use serde_json::Value;

use crate::lexicon::{self, Lexicon};
use crate::source::{Action, RecordEvent};
use crate::views::{PostView, ReactionView};

const SCHEMA: &str = "CREATE TABLE IF NOT EXISTS records (
    uri        TEXT PRIMARY KEY,
    did        TEXT NOT NULL,
    collection TEXT NOT NULL,
    rkey       TEXT NOT NULL,
    json       TEXT NOT NULL,
    created_at TEXT,
    subject    TEXT,
    emoji      TEXT
);";

pub struct Indexer {
    conn: Connection,
    post_lex: Lexicon,
    reaction_lex: Lexicon,
}

impl Indexer {
    pub fn open_in_memory() -> Self {
        let conn = Connection::open_in_memory().expect("open sqlite");
        conn.execute_batch(SCHEMA).expect("create schema");
        Self {
            conn,
            post_lex: Lexicon::load(lexicon::POST_LEXICON),
            reaction_lex: Lexicon::load(lexicon::REACTION_LEXICON),
        }
    }

    fn post_nsid(&self) -> &str {
        self.post_lex.id()
    }
    fn reaction_nsid(&self) -> &str {
        self.reaction_lex.id()
    }

    /// Drop and recreate the table — the index is a disposable projection,
    /// rebuildable by replaying the source.
    pub fn rebuild(&self) {
        self.conn.execute_batch("DROP TABLE IF EXISTS records;").expect("drop");
        self.conn.execute_batch(SCHEMA).expect("recreate");
    }

    /// Ingest one event. `Ok(true)` if a row changed; `Err(reason)` if the
    /// record is rejected (schema-invalid or an unknown collection).
    pub fn apply(&self, ev: &RecordEvent) -> Result<bool, String> {
        let uri = ev.uri();
        match ev.action {
            Action::Delete => {
                let n = self
                    .conn
                    .execute("DELETE FROM records WHERE uri = ?1", params![uri])
                    .map_err(|e| e.to_string())?;
                Ok(n > 0)
            }
            Action::Create | Action::Update => {
                // Enforce the lexicon on the way in — an AppView indexes only
                // schema-conformant records.
                let lex = self.lexicon_for(&ev.collection)?;
                lex.validate(&ev.record)
                    .map_err(|e| format!("rejected {uri}: {e}"))?;

                let created_at = ev.record["createdAt"].as_str().unwrap_or_default().to_string();
                let (subject, emoji) = if ev.collection == self.reaction_nsid() {
                    (
                        ev.record["subject"]["uri"].as_str().map(str::to_string),
                        ev.record["emoji"].as_str().map(str::to_string),
                    )
                } else {
                    (None, None)
                };
                let json = serde_json::to_string(&ev.record).unwrap();
                self.conn
                    .execute(
                        "INSERT OR REPLACE INTO records
                         (uri, did, collection, rkey, json, created_at, subject, emoji)
                         VALUES (?1,?2,?3,?4,?5,?6,?7,?8)",
                        params![uri, ev.did, ev.collection, ev.rkey, json, created_at, subject, emoji],
                    )
                    .map_err(|e| e.to_string())?;
                Ok(true)
            }
        }
    }

    fn lexicon_for(&self, collection: &str) -> Result<&Lexicon, String> {
        if collection == self.post_nsid() {
            Ok(&self.post_lex)
        } else if collection == self.reaction_nsid() {
            Ok(&self.reaction_lex)
        } else {
            Err(format!("unknown collection {collection}"))
        }
    }

    pub fn count(&self) -> i64 {
        self.conn
            .query_row("SELECT COUNT(*) FROM records", [], |r| r.get(0))
            .unwrap_or(0)
    }

    /// Reactions hydrated for a post URI (emoji + reactor DID), in rkey order.
    fn reactions_for(&self, post_uri: &str) -> Vec<ReactionView> {
        let mut stmt = self
            .conn
            .prepare("SELECT emoji, did FROM records WHERE collection = ?1 AND subject = ?2 ORDER BY rkey")
            .unwrap();
        let rows = stmt
            .query_map(params![self.reaction_nsid(), post_uri], |r| {
                Ok(ReactionView {
                    emoji: r.get::<_, Option<String>>(0)?.unwrap_or_default(),
                    reactor: r.get(1)?,
                })
            })
            .unwrap();
        rows.filter_map(Result::ok).collect()
    }

    fn row_to_postview(&self, uri: String, did: String, json: String, created_at: String) -> PostView {
        let v: Value = serde_json::from_str(&json).unwrap_or(Value::Null);
        let text = v["text"].as_str().unwrap_or_default().to_string();
        let reactions = self.reactions_for(&uri);
        PostView { uri, author: did, text, created_at, reactions }
    }

    /// Timeline: posts in reverse-chronological order (rkey/TID desc), each
    /// hydrated with its reactions. This join is the core AppView behavior.
    pub fn timeline(&self, limit: u32) -> Vec<PostView> {
        let mut stmt = self
            .conn
            .prepare("SELECT uri, did, json, created_at FROM records WHERE collection = ?1 ORDER BY rkey DESC LIMIT ?2")
            .unwrap();
        let rows = stmt
            .query_map(params![self.post_nsid(), limit], |r| {
                Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?))
            })
            .unwrap();
        rows.filter_map(Result::ok)
            .map(|(u, d, j, c)| self.row_to_postview(u, d, j, c))
            .collect()
    }

    /// A single post (by URI) hydrated with its reactions.
    pub fn thread(&self, uri: &str) -> Option<PostView> {
        let row = self
            .conn
            .query_row(
                "SELECT uri, did, json, created_at FROM records WHERE uri = ?1 AND collection = ?2",
                params![uri, self.post_nsid()],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)),
            )
            .ok()?;
        Some(self.row_to_postview(row.0, row.1, row.2, row.3))
    }
}
