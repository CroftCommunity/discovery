//! Serve: an XRPC-shaped read API over the SQLite index, with axum.
//!
//! Two queries. One straightforward read, one genuine hydration (aggregation
//! across many records into a view that is not an echo of any single stored row):
//!
//!   GET /xrpc/com.example.getRecentPosts?limit=N
//!        -> most recent indexed posts, hydrated (author DID, text, createdAt,
//!           reply-parent if any).
//!
//!   GET /xrpc/com.example.getLikeCountsBySubject?limit=N
//!        -> like records GROUPed BY their subject URI, with counts: a view row
//!           assembled by joining many like records together. This is hydration —
//!           the response row exists in no single stored record.

use std::sync::{Arc, Mutex};

use axum::{
    extract::{Query, State},
    routing::get,
    Json, Router,
};
use rusqlite::Connection;
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Mutex<Connection>>,
}

#[derive(Deserialize)]
pub struct LimitParam {
    limit: Option<i64>,
}

fn clamp_limit(l: Option<i64>) -> i64 {
    l.unwrap_or(10).clamp(1, 100)
}

pub fn router(db_path: &str) -> anyhow::Result<Router> {
    // Read-only connection for serving.
    let conn = Connection::open(db_path)?;
    let state = AppState {
        db: Arc::new(Mutex::new(conn)),
    };
    Ok(Router::new()
        .route("/xrpc/com.example.getRecentPosts", get(get_recent_posts))
        .route(
            "/xrpc/com.example.getLikeCountsBySubject",
            get(get_like_counts_by_subject),
        )
        .with_state(state))
}

async fn get_recent_posts(
    State(st): State<AppState>,
    Query(p): Query<LimitParam>,
) -> Json<Value> {
    let limit = clamp_limit(p.limit);
    let conn = st.db.lock().unwrap();
    let mut stmt = conn
        .prepare(
            "SELECT uri, did, text, created_at, reply_parent
             FROM records
             WHERE collection = 'app.bsky.feed.post'
             ORDER BY cursor DESC
             LIMIT ?1",
        )
        .unwrap();
    let rows = stmt
        .query_map([limit], |row| {
            let reply_parent: Option<String> = row.get(4)?;
            Ok(json!({
                "uri": row.get::<_, String>(0)?,
                "author": row.get::<_, String>(1)?,
                "text": row.get::<_, Option<String>>(2)?,
                "createdAt": row.get::<_, Option<String>>(3)?,
                "replyTo": reply_parent,
            }))
        })
        .unwrap()
        .filter_map(Result::ok)
        .collect::<Vec<_>>();
    Json(json!({ "posts": rows, "count": rows.len() }))
}

async fn get_like_counts_by_subject(
    State(st): State<AppState>,
    Query(p): Query<LimitParam>,
) -> Json<Value> {
    let limit = clamp_limit(p.limit);
    let conn = st.db.lock().unwrap();
    let mut stmt = conn
        .prepare(
            "SELECT subject_uri, COUNT(*) AS likes
             FROM records
             WHERE collection = 'app.bsky.feed.like' AND subject_uri IS NOT NULL
             GROUP BY subject_uri
             ORDER BY likes DESC
             LIMIT ?1",
        )
        .unwrap();
    let rows = stmt
        .query_map([limit], |row| {
            Ok(json!({
                "subject": row.get::<_, String>(0)?,
                "likeCount": row.get::<_, i64>(1)?,
            }))
        })
        .unwrap()
        .filter_map(Result::ok)
        .collect::<Vec<_>>();
    Json(json!({ "subjects": rows, "count": rows.len() }))
}

/// Minimal raw HTTP/1.1 GET against our own localhost server, so the demo issues
/// *real* network requests without pulling in a heavyweight HTTP client. Sends
/// `Connection: close` and reads to EOF, then splits headers from the JSON body.
pub async fn http_get(addr: &str, path: &str) -> anyhow::Result<String> {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpStream;

    let mut stream = TcpStream::connect(addr).await?;
    let req = format!(
        "GET {path} HTTP/1.1\r\nHost: {addr}\r\nConnection: close\r\nAccept: application/json\r\n\r\n"
    );
    stream.write_all(req.as_bytes()).await?;
    let mut buf = Vec::new();
    stream.read_to_end(&mut buf).await?;
    let raw = String::from_utf8_lossy(&buf);
    let body = raw
        .split_once("\r\n\r\n")
        .map(|(_, b)| b.to_string())
        .unwrap_or_else(|| raw.to_string());
    Ok(body)
}
