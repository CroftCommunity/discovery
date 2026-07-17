//! A disposable SQLite index — the query-optimized projection an AppView serves.
//!
//! PROVENANCE (copied, not imported): the schema and upsert/serve shape are the
//! minimal subset of `alpha/experiments/appview-validation/src/index.rs` and
//! `server.rs` that EXP-C needs. The point of EXP-C is that this ONE code path
//! serves a public-source row and a helper-fed (private-source) row identically —
//! the source-agnostic claim, now live on the private side.

use anyhow::Result;
use rusqlite::Connection;

use crate::bridge::NormalizedEvent;

/// One search hit as the serve layer returns it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchHit {
    pub uri: String,
    pub text: String,
    pub source: String,
}

pub struct Index {
    conn: Connection,
}

impl Index {
    /// A fresh in-memory index (disposable projection).
    pub fn open_in_memory() -> Result<Index> {
        let conn = Connection::open_in_memory()?;
        conn.execute_batch(
            r#"
            CREATE TABLE records (
                uri         TEXT PRIMARY KEY,
                did         TEXT NOT NULL,
                collection  TEXT NOT NULL,
                rkey        TEXT NOT NULL,
                record_json TEXT,
                text        TEXT,
                source      TEXT NOT NULL
            );
            CREATE INDEX idx_text ON records(text);
            "#,
        )?;
        Ok(Index { conn })
    }

    /// Apply one normalized event — the SINGLE ingest path for every source.
    pub fn apply(&self, ev: &NormalizedEvent) -> Result<()> {
        let record_json = ev.record.as_ref().map(std::string::ToString::to_string);
        self.conn.execute(
            r#"
            INSERT INTO records (uri, did, collection, rkey, record_json, text, source)
            VALUES (?1,?2,?3,?4,?5,?6,?7)
            ON CONFLICT(uri) DO UPDATE SET
                record_json = excluded.record_json,
                text        = excluded.text,
                source      = excluded.source
            "#,
            rusqlite::params![
                ev.uri,
                ev.did,
                ev.collection,
                ev.rkey,
                record_json,
                ev.text(),
                ev.source,
            ],
        )?;
        Ok(())
    }

    /// The serve layer: full-text-ish search over indexed content — the same
    /// query for public and helper-fed rows.
    pub fn search(&self, term: &str) -> Result<Vec<SearchHit>> {
        let pattern = format!("%{term}%");
        let mut stmt = self.conn.prepare(
            "SELECT uri, COALESCE(text,''), source FROM records
             WHERE text LIKE ?1 ORDER BY uri",
        )?;
        let hits = stmt
            .query_map([pattern], |r| {
                Ok(SearchHit {
                    uri: r.get(0)?,
                    text: r.get(1)?,
                    source: r.get(2)?,
                })
            })?
            .filter_map(std::result::Result::ok)
            .collect();
        Ok(hits)
    }

    pub fn count(&self) -> Result<i64> {
        Ok(self
            .conn
            .query_row("SELECT COUNT(*) FROM records", [], |r| r.get(0))?)
    }
}
