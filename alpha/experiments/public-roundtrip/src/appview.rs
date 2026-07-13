//! Minimal AppView: a SQLite index plus one XRPC-style read query over axum.
//! Its only job is to prove that records consumed off the public network are
//! servable from our own index.

use anyhow::{Context, Result};
use axum::{extract::Query, extract::State, response::Json, routing::get, Router};
use rusqlite::Connection;
use serde::Deserialize;
use serde_json::{json, Value};
use std::sync::Arc;

use crate::lexicon::{FeedPost, GET_THREAD_NSID, GET_TIMELINE_NSID};

pub struct Db;

impl Db {
    /// Open (creating if needed) the SQLite index and ensure the schema exists.
    pub fn open(path: &str) -> Result<Connection> {
        let conn = Connection::open(path).with_context(|| format!("opening sqlite at {path}"))?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS posts (
                uri        TEXT PRIMARY KEY,
                cid        TEXT NOT NULL,
                did        TEXT NOT NULL,
                collection TEXT NOT NULL,
                rkey       TEXT NOT NULL,
                text       TEXT NOT NULL,
                created_at TEXT NOT NULL,
                indexed_at TEXT NOT NULL,
                record_json TEXT NOT NULL,
                source     TEXT NOT NULL,
                root_uri   TEXT,
                parent_uri TEXT
            );
            CREATE TABLE IF NOT EXISTS meta (
                key   TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );",
        )?;
        // Best-effort migration for indexes created before reply support.
        let _ = conn.execute("ALTER TABLE posts ADD COLUMN root_uri TEXT", []);
        let _ = conn.execute("ALTER TABLE posts ADD COLUMN parent_uri TEXT", []);
        Ok(conn)
    }

    /// Persist a small key/value (used for the Jetstream resume cursor).
    pub fn set_meta(conn: &Connection, key: &str, value: &str) -> Result<()> {
        conn.execute(
            "INSERT OR REPLACE INTO meta (key, value) VALUES (?1, ?2)",
            rusqlite::params![key, value],
        )?;
        Ok(())
    }

    pub fn get_meta(conn: &Connection, key: &str) -> Option<String> {
        conn.query_row("SELECT value FROM meta WHERE key = ?1", [key], |r| r.get::<_, String>(0))
            .ok()
    }

    /// Remove a post by URI (Jetstream `delete` op). Returns whether a row went.
    pub fn delete_post(conn: &Connection, uri: &str) -> Result<bool> {
        let n = conn.execute("DELETE FROM posts WHERE uri = ?1", [uri])?;
        Ok(n > 0)
    }

    pub fn count_posts(conn: &Connection) -> Result<i64> {
        Ok(conn.query_row("SELECT COUNT(*) FROM posts", [], |r| r.get(0))?)
    }

    /// Assemble a thread: the root post plus every post sharing its root,
    /// chronologically, each tagged with its depth (hops to the root) and
    /// `parentUri` so a client can nest the replies.
    pub fn get_thread(conn: &Connection, root: &str) -> Result<Vec<Value>> {
        struct Row {
            uri: String,
            did: String,
            text: String,
            created_at: String,
            parent: Option<String>,
        }
        let mut stmt = conn.prepare(
            "SELECT uri, did, text, created_at, parent_uri FROM posts
             WHERE root_uri = ?1 OR uri = ?1 ORDER BY created_at ASC",
        )?;
        let rows: Vec<Row> = stmt
            .query_map([root], |r| {
                Ok(Row {
                    uri: r.get(0)?,
                    did: r.get(1)?,
                    text: r.get(2)?,
                    created_at: r.get(3)?,
                    parent: r.get(4)?,
                })
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;

        let parent_of: std::collections::HashMap<&str, Option<&str>> =
            rows.iter().map(|r| (r.uri.as_str(), r.parent.as_deref())).collect();
        let depth = |uri: &str| -> i64 {
            let mut d = 0;
            let mut cur = uri;
            while cur != root {
                match parent_of.get(cur).copied().flatten() {
                    Some(p) if p != cur => {
                        d += 1;
                        cur = p;
                        if d > 100 {
                            break;
                        }
                    }
                    _ => break,
                }
            }
            d
        };

        let mut out = Vec::new();
        for r in &rows {
            let mut obj = json!({
                "uri": r.uri,
                "did": r.did,
                "text": r.text,
                "createdAt": r.created_at,
                "depth": depth(&r.uri),
            });
            if let Some(p) = &r.parent {
                obj["parentUri"] = json!(p);
            }
            out.push(obj);
        }
        Ok(out)
    }

    /// Index one consumed record, validating it against the lexicon first.
    #[allow(clippy::too_many_arguments)]
    pub fn index_post(
        conn: &Connection,
        uri: &str,
        cid: &str,
        did: &str,
        collection: &str,
        rkey: &str,
        record: &Value,
        source: &str,
    ) -> Result<()> {
        let post: FeedPost = serde_json::from_value(record.clone())
            .context("record does not match feed.post shape")?;
        post.validate().context("record failed lexicon validation on ingest")?;
        // The timeline query orders by created_at as TEXT, so normalize to a
        // UTC RFC 3339 string ('Z') on ingest. Otherwise mixed UTC offsets would
        // sort lexicographically rather than chronologically.
        let created_at_utc = chrono::DateTime::parse_from_rfc3339(&post.created_at)
            .context("createdAt parse failed during normalization")?
            .with_timezone(&chrono::Utc)
            .to_rfc3339();
        let indexed_at = chrono::Utc::now().to_rfc3339();
        // Reply posts carry their thread root + parent; a non-reply is its own
        // root (root_uri = its own uri, parent_uri = NULL). This lets getThread
        // collect a whole thread by root and nest by parent.
        let (root_uri, parent_uri) = match &post.reply {
            Some(r) => (r.root.uri.clone(), Some(r.parent.uri.clone())),
            None => (uri.to_string(), None),
        };
        conn.execute(
            "INSERT OR REPLACE INTO posts
                (uri, cid, did, collection, rkey, text, created_at, indexed_at, record_json, source, root_uri, parent_uri)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
            rusqlite::params![
                uri,
                cid,
                did,
                collection,
                rkey,
                post.text,
                created_at_utc,
                indexed_at,
                record.to_string(),
                source,
                root_uri,
                parent_uri,
            ],
        )?;
        Ok(())
    }
}

#[derive(Clone)]
struct AppState {
    db_path: Arc<String>,
}

#[derive(Deserialize)]
struct TimelineParams {
    limit: Option<i64>,
}

/// `GET /xrpc/org.croftc.experiment.feed.getTimeline` — reverse-chronological.
async fn get_timeline(
    State(state): State<AppState>,
    Query(params): Query<TimelineParams>,
) -> Result<Json<Value>, (axum::http::StatusCode, String)> {
    let limit = params.limit.unwrap_or(50).clamp(1, 100);
    let db_path = state.db_path.clone();
    // rusqlite is synchronous; run it off the async worker thread.
    let posts = tokio::task::spawn_blocking(move || -> Result<Vec<Value>> {
        let conn = Db::open(&db_path)?;
        let mut stmt = conn.prepare(
            "SELECT uri, cid, did, text, created_at, indexed_at
             FROM posts ORDER BY created_at DESC LIMIT ?1",
        )?;
        let rows = stmt.query_map([limit], |row| {
            Ok(json!({
                "uri": row.get::<_, String>(0)?,
                "cid": row.get::<_, String>(1)?,
                "did": row.get::<_, String>(2)?,
                "text": row.get::<_, String>(3)?,
                "createdAt": row.get::<_, String>(4)?,
                "indexedAt": row.get::<_, String>(5)?,
            }))
        })?;
        let mut out = Vec::new();
        for r in rows {
            out.push(r?);
        }
        Ok(out)
    })
    .await
    .map_err(|e| {
        // Log detail server-side; return a generic message so internal details
        // (e.g. the SQLite path) are not leaked to HTTP clients.
        eprintln!("appview timeline task error: {e}");
        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "internal error".to_string())
    })?
    .map_err(|e| {
        eprintln!("appview timeline query error: {e}");
        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "internal error".to_string())
    })?;

    Ok(Json(json!({ "posts": posts })))
}

#[derive(Deserialize)]
struct ThreadParams {
    uri: String,
}

/// `GET /xrpc/org.croftc.experiment.feed.getThread?uri=<root>` — threaded view.
async fn get_thread(
    State(state): State<AppState>,
    Query(params): Query<ThreadParams>,
) -> Result<Json<Value>, (axum::http::StatusCode, String)> {
    let db_path = state.db_path.clone();
    let thread = tokio::task::spawn_blocking(move || -> Result<Vec<Value>> {
        let conn = Db::open(&db_path)?;
        Db::get_thread(&conn, &params.uri)
    })
    .await
    .map_err(|e| {
        eprintln!("appview thread task error: {e}");
        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "internal error".to_string())
    })?
    .map_err(|e| {
        eprintln!("appview thread query error: {e}");
        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "internal error".to_string())
    })?;
    Ok(Json(json!({ "thread": thread })))
}

/// Serve the AppView on 127.0.0.1:`port`. Loopback only — this is an experiment.
pub async fn serve(db_path: String, port: u16) -> Result<()> {
    let state = AppState { db_path: Arc::new(db_path) };
    let timeline_route = format!("/xrpc/{GET_TIMELINE_NSID}");
    let thread_route = format!("/xrpc/{GET_THREAD_NSID}");
    let app = Router::new()
        .route(&timeline_route, get(get_timeline))
        .route(&thread_route, get(get_thread))
        .with_state(state);
    let addr = format!("127.0.0.1:{port}");
    println!("AppView listening on http://{addr}");
    println!("  timeline: {timeline_route}");
    println!("  thread:   {thread_route}?uri=<root>");
    let listener = tokio::net::TcpListener::bind(&addr).await.with_context(|| format!("binding {addr}"))?;
    axum::serve(listener, app).await.context("axum serve")?;
    Ok(())
}
