//! A focused PDS for the record-identity question. Same XRPC shape as Phase 5,
//! but the create/edit semantics are the point here:
//!   * `createRecord` honors a caller-supplied `rkey` (so identity is pinned at
//!     creation, not reassigned by the server);
//!   * `createRecord` is **idempotent** for an identical (rkey, content) repeat,
//!     and **rejects** a same-rkey/different-content create (use putRecord);
//!   * `putRecord` **upserts** — same rkey, new content -> same URI, new CID.
//! In-memory map store (no firehose/SQLite needed for this question).

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use axum::extract::{Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::lexicon::Lexicon;
use crate::record::{POST_NSID, REACTION_NSID};

#[derive(Default)]
struct State_ {
    sessions: BTreeMap<String, String>,                 // token -> did
    records: BTreeMap<(String, String, String), (Value, String)>, // (did,coll,rkey) -> (record, cid)
}

#[derive(Clone)]
pub struct Pds {
    inner: Arc<Mutex<State_>>,
    post_lex: Arc<Lexicon>,
    reaction_lex: Arc<Lexicon>,
}

impl Pds {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(State_::default())),
            post_lex: Arc::new(Lexicon::load(crate::lexicon::POST_LEXICON)),
            reaction_lex: Arc::new(Lexicon::load(crate::lexicon::REACTION_LEXICON)),
        }
    }
    fn lex(&self, collection: &str) -> Option<&Lexicon> {
        if collection == POST_NSID {
            Some(&self.post_lex)
        } else if collection == REACTION_NSID {
            Some(&self.reaction_lex)
        } else {
            None
        }
    }
}

pub fn router(pds: Pds) -> Router {
    Router::new()
        .route("/xrpc/com.atproto.server.createSession", post(create_session))
        .route("/xrpc/com.atproto.repo.createRecord", post(create_record))
        .route("/xrpc/com.atproto.repo.putRecord", post(put_record))
        .route("/xrpc/com.atproto.repo.getRecord", get(get_record))
        .with_state(pds)
}

fn did_for(identifier: &str) -> String {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    identifier.hash(&mut h);
    format!("did:plc:{:016x}", h.finish())
}

fn bearer(h: &HeaderMap) -> Option<String> {
    h.get(axum::http::header::AUTHORIZATION)?.to_str().ok()?.strip_prefix("Bearer ").map(str::to_string)
}

#[derive(Deserialize)]
struct SessionReq {
    identifier: String,
    #[allow(dead_code)]
    password: String,
}

async fn create_session(State(pds): State<Pds>, Json(req): Json<SessionReq>) -> Json<Value> {
    let did = did_for(&req.identifier);
    let token = format!("tok-{}", did_for(&format!("{}-session", req.identifier)));
    pds.inner.lock().unwrap().sessions.insert(token.clone(), did.clone());
    Json(json!({ "did": did, "handle": req.identifier, "accessJwt": token }))
}

#[derive(Deserialize)]
struct WriteReq {
    repo: String,
    collection: String,
    rkey: Option<String>,
    record: Value,
}

fn auth(pds: &Pds, headers: &HeaderMap, repo: &str) -> Result<(), (StatusCode, String)> {
    let token = bearer(headers).ok_or((StatusCode::UNAUTHORIZED, "missing bearer".into()))?;
    let did = pds.inner.lock().unwrap().sessions.get(&token).cloned();
    match did {
        Some(d) if d == repo => Ok(()),
        Some(_) => Err((StatusCode::FORBIDDEN, "repo != session did".into())),
        None => Err((StatusCode::UNAUTHORIZED, "invalid session".into())),
    }
}

async fn create_record(
    State(pds): State<Pds>,
    headers: HeaderMap,
    Json(req): Json<WriteReq>,
) -> Result<Json<Value>, (StatusCode, String)> {
    auth(&pds, &headers, &req.repo)?;
    let lex = pds.lex(&req.collection).ok_or((StatusCode::BAD_REQUEST, "unknown collection".into()))?;
    lex.validate(&req.record).map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    // Identity is pinned at creation: honor the caller's rkey (only mint one if absent).
    let rkey = req.rkey.clone().unwrap_or_else(crate::record::new_tid);
    let cid = crate::content_id::record_cid(&req.record);
    let uri = format!("at://{}/{}/{}", req.repo, req.collection, rkey);
    let key = (req.repo.clone(), req.collection.clone(), rkey.clone());

    let mut st = pds.inner.lock().unwrap();
    if let Some((_, existing_cid)) = st.records.get(&key) {
        if *existing_cid == cid {
            // Idempotent: identical (rkey, content) -> no duplicate, same result.
            return Ok(Json(json!({ "uri": uri, "cid": cid, "idempotent": true })));
        }
        // Same rkey, different content -> a create must NOT clobber. Edits use putRecord.
        return Err((StatusCode::CONFLICT, "record already exists at rkey; use putRecord to edit".into()));
    }
    st.records.insert(key, (req.record, cid.clone()));
    Ok(Json(json!({ "uri": uri, "cid": cid, "idempotent": false })))
}

async fn put_record(
    State(pds): State<Pds>,
    headers: HeaderMap,
    Json(req): Json<WriteReq>,
) -> Result<Json<Value>, (StatusCode, String)> {
    auth(&pds, &headers, &req.repo)?;
    let lex = pds.lex(&req.collection).ok_or((StatusCode::BAD_REQUEST, "unknown collection".into()))?;
    lex.validate(&req.record).map_err(|e| (StatusCode::BAD_REQUEST, e))?;
    let rkey = req.rkey.clone().ok_or((StatusCode::BAD_REQUEST, "putRecord requires rkey".into()))?;
    let cid = crate::content_id::record_cid(&req.record);
    let uri = format!("at://{}/{}/{}", req.repo, req.collection, rkey);
    // Upsert: same URI, (possibly) new CID.
    pds.inner.lock().unwrap().records.insert((req.repo.clone(), req.collection.clone(), rkey), (req.record, cid.clone()));
    Ok(Json(json!({ "uri": uri, "cid": cid })))
}

#[derive(Deserialize)]
struct GetReq {
    repo: String,
    collection: String,
    rkey: String,
}

async fn get_record(State(pds): State<Pds>, Query(q): Query<GetReq>) -> Result<Json<Value>, StatusCode> {
    let st = pds.inner.lock().unwrap();
    match st.records.get(&(q.repo.clone(), q.collection.clone(), q.rkey.clone())) {
        Some((record, cid)) => Ok(Json(json!({
            "uri": format!("at://{}/{}/{}", q.repo, q.collection, q.rkey),
            "cid": cid,
            "value": record,
        }))),
        None => Err(StatusCode::NOT_FOUND),
    }
}
