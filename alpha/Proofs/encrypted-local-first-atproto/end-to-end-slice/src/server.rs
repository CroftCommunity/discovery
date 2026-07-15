//! XRPC-style read API over HTTP (axum). Endpoints live at `/xrpc/<nsid>` and
//! return JSON matching the read lexicons' `output` schemas. No knowledge of the
//! private stack — it only queries the indexer.

use std::sync::{Arc, Mutex};

use axum::{
    extract::{Query, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::indexer::Indexer;

/// Shared indexer handle. `Mutex` makes the `!Sync` SQLite connection usable as
/// axum state across worker threads.
pub type Shared = Arc<Mutex<Indexer>>;

pub fn router(state: Shared) -> Router {
    Router::new()
        .route("/xrpc/org.croftc.experiment.feed.getTimeline", get(get_timeline))
        .route("/xrpc/org.croftc.experiment.feed.getPostThread", get(get_post_thread))
        .with_state(state)
}

#[derive(Deserialize)]
struct TimelineParams {
    limit: Option<u32>,
}

async fn get_timeline(State(state): State<Shared>, Query(p): Query<TimelineParams>) -> Json<Value> {
    let idx = state.lock().unwrap();
    let feed = idx.timeline(p.limit.unwrap_or(50).clamp(1, 100));
    Json(json!({ "feed": feed }))
}

#[derive(Deserialize)]
struct ThreadParams {
    uri: String,
}

async fn get_post_thread(
    State(state): State<Shared>,
    Query(p): Query<ThreadParams>,
) -> Result<Json<Value>, StatusCode> {
    let idx = state.lock().unwrap();
    match idx.thread(&p.uri) {
        Some(view) => Ok(Json(json!({ "thread": view }))),
        None => Err(StatusCode::NOT_FOUND),
    }
}
