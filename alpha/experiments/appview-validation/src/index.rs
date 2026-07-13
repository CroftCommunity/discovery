//! The index: a disposable SQLite projection of network records.
//!
//! The authoritative copy of every record lives in its author's PDS. This table
//! is a query-optimized view we can wipe and rebuild by re-running ingest.

use anyhow::Result;
use rusqlite::Connection;

use crate::record_source::{Action, RecordEvent};

pub struct Index {
    conn: Connection,
}

/// Outcome of indexing a single event, for reporting.
#[derive(Debug, Default, Clone, Copy)]
pub struct IndexStats {
    pub created: usize,
    pub updated: usize,
    pub deleted_rows: usize,
    pub deletes_seen: usize,
}

impl Index {
    /// Open a fresh, disposable index file (recreated each run).
    pub fn open(path: &str) -> Result<Index> {
        let _ = std::fs::remove_file(path); // disposable projection — start clean
        let conn = Connection::open(path)?;
        conn.execute_batch(
            r#"
            CREATE TABLE records (
                uri          TEXT PRIMARY KEY,
                did          TEXT NOT NULL,
                collection   TEXT NOT NULL,
                rkey         TEXT NOT NULL,
                record_json  TEXT,
                created_at   TEXT,
                cid          TEXT,
                cursor       INTEGER,
                text         TEXT,   -- app.bsky.feed.post body text
                reply_parent TEXT,   -- post: reply.parent.uri
                subject_uri  TEXT    -- app.bsky.feed.like: subject.uri
            );
            CREATE INDEX idx_collection ON records(collection);
            CREATE INDEX idx_subject ON records(subject_uri);
            CREATE INDEX idx_cursor ON records(cursor);

            -- Moderation: labels applied to records by some labeler `src`.
            CREATE TABLE labels (
                uri TEXT NOT NULL,
                val TEXT NOT NULL,   -- e.g. 'spam', '!hide'
                src TEXT NOT NULL    -- labeler identity
            );
            CREATE INDEX idx_label_uri ON labels(uri);

            -- Consumer checkpoint: the last processed Jetstream cursor (time_us),
            -- so a restarted consumer can resume instead of losing its place.
            CREATE TABLE cursor_state (
                name    TEXT PRIMARY KEY,
                time_us INTEGER NOT NULL
            );
            "#,
        )?;
        Ok(Index { conn })
    }

    /// Apply one event. Lenient: extraction failures are tolerated (logged by caller).
    pub fn apply(&mut self, ev: &RecordEvent, stats: &mut IndexStats) -> Result<()> {
        match ev.action {
            Action::Delete => {
                stats.deletes_seen += 1;
                stats.deleted_rows += delete_row(&self.conn, &ev.at_uri())?;
            }
            Action::Create | Action::Update => {
                upsert_row(&self.conn, ev)?;
                match ev.action {
                    Action::Create => stats.created += 1,
                    Action::Update => stats.updated += 1,
                    _ => {}
                }
            }
        }
        Ok(())
    }

    /// Bulk-apply creates inside a single transaction. Proves the scale lesson:
    /// batching amortizes the per-commit fsync that throttles row-by-row inserts.
    pub fn apply_batch(&mut self, events: &[RecordEvent]) -> Result<usize> {
        let tx = self.conn.transaction()?;
        for ev in events {
            upsert_row(&tx, ev)?;
        }
        tx.commit()?;
        Ok(events.len())
    }

    /// Attach a moderation label to a record (from labeler `src`).
    pub fn add_label(&self, uri: &str, val: &str, src: &str) -> Result<()> {
        self.conn.execute(
            "INSERT INTO labels (uri, val, src) VALUES (?1, ?2, ?3)",
            rusqlite::params![uri, val, src],
        )?;
        Ok(())
    }

    /// Recent posts, optionally moderated: when `hide_labeled` is true, rows
    /// carrying a hide/spam label are excluded (a basic moderation filter).
    pub fn recent_posts(
        &self,
        hide_labeled: bool,
        limit: i64,
    ) -> Result<Vec<(String, String)>> {
        let sql = if hide_labeled {
            "SELECT r.uri, COALESCE(r.text, '')
             FROM records r
             WHERE r.collection = 'app.bsky.feed.post'
               AND r.uri NOT IN (SELECT uri FROM labels WHERE val IN ('spam','!hide'))
             ORDER BY r.cursor DESC LIMIT ?1"
        } else {
            "SELECT r.uri, COALESCE(r.text, '')
             FROM records r
             WHERE r.collection = 'app.bsky.feed.post'
             ORDER BY r.cursor DESC LIMIT ?1"
        };
        let mut stmt = self.conn.prepare(sql)?;
        let rows = stmt
            .query_map([limit], |row| Ok((row.get(0)?, row.get(1)?)))?
            .filter_map(std::result::Result::ok)
            .collect();
        Ok(rows)
    }

    /// Top subjects by like count (trending), from indexed `app.bsky.feed.like` rows.
    pub fn top_liked_subjects(&self, limit: i64) -> Result<Vec<(String, i64)>> {
        let mut stmt = self.conn.prepare(
            "SELECT subject_uri, COUNT(*) AS likes
             FROM records
             WHERE collection = 'app.bsky.feed.like' AND subject_uri IS NOT NULL
             GROUP BY subject_uri
             ORDER BY likes DESC, subject_uri
             LIMIT ?1",
        )?;
        let rows = stmt
            .query_map([limit], |row| Ok((row.get(0)?, row.get(1)?)))?
            .filter_map(std::result::Result::ok)
            .collect();
        Ok(rows)
    }

    /// Stored raw record JSON for every row of a collection (for typed comprehension).
    pub fn records_json(&self, collection: &str) -> Result<Vec<(String, String)>> {
        let mut stmt = self.conn.prepare(
            "SELECT uri, COALESCE(record_json, '') FROM records WHERE collection = ?1",
        )?;
        let rows = stmt
            .query_map([collection], |row| Ok((row.get(0)?, row.get(1)?)))?
            .filter_map(std::result::Result::ok)
            .collect();
        Ok(rows)
    }

    /// Checkpoint the last processed cursor (`time_us`) under a name.
    pub fn save_cursor(&self, name: &str, time_us: i64) -> Result<()> {
        self.conn.execute(
            "INSERT INTO cursor_state (name, time_us) VALUES (?1, ?2)
             ON CONFLICT(name) DO UPDATE SET time_us = excluded.time_us",
            rusqlite::params![name, time_us],
        )?;
        Ok(())
    }

    /// Load a previously checkpointed cursor, if any.
    pub fn load_cursor(&self, name: &str) -> Result<Option<i64>> {
        Ok(self
            .conn
            .query_row(
                "SELECT time_us FROM cursor_state WHERE name = ?1",
                [name],
                |r| r.get(0),
            )
            .ok())
    }

    pub fn row_count(&self) -> Result<i64> {
        Ok(self
            .conn
            .query_row("SELECT COUNT(*) FROM records", [], |r| r.get(0))?)
    }

    pub fn count_for(&self, collection: &str) -> Result<i64> {
        Ok(self.conn.query_row(
            "SELECT COUNT(*) FROM records WHERE collection = ?1",
            [collection],
            |r| r.get(0),
        )?)
    }
}

/// Upsert one record. Takes anything that derefs to a `Connection` (a borrowed
/// connection or a `Transaction`), so single and batched inserts share the logic.
fn upsert_row(conn: &Connection, ev: &RecordEvent) -> Result<()> {
    let record = ev.record.as_ref();
    let get_str = |path: &[&str]| -> Option<String> {
        let mut cur = record?;
        for p in path {
            cur = cur.get(p)?;
        }
        cur.as_str().map(str::to_string)
    };
    let created_at = get_str(&["createdAt"]);
    let text = get_str(&["text"]);
    let reply_parent = get_str(&["reply", "parent", "uri"]);
    let subject_uri = get_str(&["subject", "uri"]);
    let record_json = record.map(|r| r.to_string());

    conn.execute(
        r#"
        INSERT INTO records
            (uri, did, collection, rkey, record_json, created_at, cid,
             cursor, text, reply_parent, subject_uri)
        VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11)
        ON CONFLICT(uri) DO UPDATE SET
            record_json = excluded.record_json,
            created_at  = excluded.created_at,
            cid         = excluded.cid,
            cursor      = excluded.cursor,
            text        = excluded.text,
            reply_parent= excluded.reply_parent,
            subject_uri = excluded.subject_uri
        "#,
        rusqlite::params![
            ev.at_uri(),
            ev.did,
            ev.collection,
            ev.rkey,
            record_json,
            created_at,
            ev.cid,
            ev.cursor,
            text,
            reply_parent,
            subject_uri,
        ],
    )?;
    Ok(())
}

fn delete_row(conn: &Connection, uri: &str) -> Result<usize> {
    Ok(conn.execute("DELETE FROM records WHERE uri = ?1", [uri])?)
}
