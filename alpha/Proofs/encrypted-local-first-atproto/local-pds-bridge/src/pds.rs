//! A minimal local atproto PDS, speaking the real XRPC wire protocol over real
//! HTTP/WebSocket on loopback. Built because the official Node/Docker PDS can't
//! be installed here (npm/registry egress is allowlist-blocked), and a local PDS
//! sidesteps the egress block entirely while still proving the *live mechanics*.
//!
//! Implements exactly what the live flip exercises:
//!   * POST /xrpc/com.atproto.server.createSession  (identifier+password -> token+did)
//!   * POST /xrpc/com.atproto.repo.createRecord      (Bearer auth; validates; stores; emits firehose)
//!   * GET  /jetstream/subscribe                     (WebSocket firehose, Jetstream-shape JSON)
//!
//! Faithful where it matters (real auth handshake, real createRecord with
//! server-assigned rkey + real CIDv1, lexicon validation on write, a real
//! firehose), simplified where it doesn't: the access token is an opaque session
//! id (not a signed JWT — we are both ends), and the firehose emits the Jetstream
//! JSON shape directly rather than the CBOR `com.atproto.sync.subscribeRepos`
//! frames a real PDS sends (Jetstream is the public JSON translation of that).

use std::collections::{BTreeMap, HashMap};
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};

use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::Response;
use axum::routing::{get, post};
use axum::{Json, Router};
use rand::RngCore;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::lexicon::Lexicon;
use crate::record::{self, POST_NSID, REACTION_NSID};

#[derive(Default)]
pub struct PdsState {
    sessions: HashMap<String, String>, // accessJwt(token) -> did
    records: BTreeMap<(String, String, String), Value>, // (did, collection, rkey) -> record
    firehose: Vec<String>, // commit events as Jetstream-shape JSON lines
    seq: u64,
}

#[derive(Clone)]
pub struct Pds {
    state: Arc<Mutex<PdsState>>,
    post_lex: Arc<Lexicon>,
    reaction_lex: Arc<Lexicon>,
}

pub fn router(pds: Pds) -> Router {
    Router::new()
        .route("/xrpc/com.atproto.server.createSession", post(create_session))
        .route("/xrpc/com.atproto.repo.createRecord", post(create_record))
        .route("/jetstream/subscribe", get(subscribe))
        .with_state(pds)
}

impl Pds {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(PdsState::default())),
            post_lex: Arc::new(Lexicon::load(crate::lexicon::POST_LEXICON)),
            reaction_lex: Arc::new(Lexicon::load(crate::lexicon::REACTION_LEXICON)),
        }
    }
}

fn did_for(identifier: &str) -> String {
    let mut h = std::collections::hash_map::DefaultHasher::new();
    identifier.hash(&mut h);
    format!("did:plc:{:016x}", h.finish())
}

fn random_token() -> String {
    let mut b = [0u8; 16];
    rand::rngs::OsRng.fill_bytes(&mut b);
    hex::encode(b)
}

fn bearer(headers: &HeaderMap) -> Option<String> {
    headers
        .get(axum::http::header::AUTHORIZATION)?
        .to_str()
        .ok()?
        .strip_prefix("Bearer ")
        .map(str::to_string)
}

#[derive(Deserialize)]
struct SessionReq {
    identifier: String,
    password: String,
}

async fn create_session(State(pds): State<Pds>, Json(req): Json<SessionReq>) -> Result<Json<Value>, StatusCode> {
    // If real account creds are configured, enforce them; otherwise this local
    // test PDS auto-provisions the account (it is our own throwaway server).
    if let (Ok(id), Ok(pw)) = (std::env::var("ATP_IDENTIFIER"), std::env::var("ATP_APP_PASSWORD")) {
        if req.identifier != id || req.password != pw {
            return Err(StatusCode::UNAUTHORIZED);
        }
    }
    let did = did_for(&req.identifier);
    let token = random_token();
    pds.state.lock().unwrap().sessions.insert(token.clone(), did.clone());
    Ok(Json(json!({ "did": did, "handle": req.identifier, "accessJwt": token })))
}

#[derive(Deserialize)]
struct CreateReq {
    repo: String,
    collection: String,
    #[serde(default)]
    rkey: Option<String>,
    record: Value,
}

async fn create_record(
    State(pds): State<Pds>,
    headers: HeaderMap,
    Json(req): Json<CreateReq>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let token = bearer(&headers).ok_or((StatusCode::UNAUTHORIZED, "missing bearer".into()))?;
    let did = pds
        .state
        .lock()
        .unwrap()
        .sessions
        .get(&token)
        .cloned()
        .ok_or((StatusCode::UNAUTHORIZED, "invalid session".into()))?;
    // A repo may only be written by its own authenticated owner.
    if req.repo != did {
        return Err((StatusCode::FORBIDDEN, "repo does not match session did".into()));
    }
    // Validate the record against its lexicon before accepting it (a real PDS does).
    let lex: &Lexicon = if req.collection == POST_NSID {
        &pds.post_lex
    } else if req.collection == REACTION_NSID {
        &pds.reaction_lex
    } else {
        return Err((StatusCode::BAD_REQUEST, format!("unknown collection {}", req.collection)));
    };
    lex.validate(&req.record).map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    let rkey = req.rkey.clone().unwrap_or_else(record::new_tid);
    let cid = crate::content_id::record_cid(&req.record);
    let uri = format!("at://{}/{}/{}", req.repo, req.collection, rkey);

    let mut st = pds.state.lock().unwrap();
    st.records.insert((did.clone(), req.collection.clone(), rkey.clone()), req.record.clone());
    st.seq += 1;
    let time_us = 1_727_000_000_000_000 + st.seq * 1_000;
    let event = crate::jetstream::commit_line(&did, time_us, "create", &req.collection, &rkey, Some(&req.record), Some(&cid));
    st.firehose.push(event);

    Ok(Json(json!({ "uri": uri, "cid": cid })))
}

async fn subscribe(State(pds): State<Pds>, ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(move |socket| handle_firehose(socket, pds))
}

async fn handle_firehose(mut socket: WebSocket, pds: Pds) {
    // Backfill: replay the buffered commit log (like subscribeRepos cursor=0),
    // then hold the socket open for live events until the client disconnects.
    let backlog = { pds.state.lock().unwrap().firehose.clone() };
    for line in backlog {
        if socket.send(Message::Text(line.into())).await.is_err() {
            return;
        }
    }
    while let Some(Ok(msg)) = socket.recv().await {
        if matches!(msg, Message::Close(_)) {
            break;
        }
    }
}
