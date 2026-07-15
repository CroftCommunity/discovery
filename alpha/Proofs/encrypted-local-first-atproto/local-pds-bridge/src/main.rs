//! Phase 5: the live flip, on loopback. A real local PDS (`pds.rs`) speaking the
//! atproto XRPC wire protocol; a real publishing client (`publisher.rs`) doing
//! createSession + createRecord over HTTP; a real WebSocket firehose consumer;
//! and the byte-identical AppView (indexer/server/views) from Phases 3a/3b
//! serving a hydrated timeline.
//!
//! Why local: the environment's egress allowlist blocks bsky.social /
//! plc.directory / Jetstream hosts (and the npm/Docker registries needed to run
//! the official PDS), so the live network can't be reached from here. A local
//! PDS proves the live *mechanics* end to end over real sockets without egress.
//! Pointing `publisher`/the firehose consumer at a live host (once allowlisted +
//! given real creds) is the only remaining change.

#[allow(dead_code)]
mod content_id;
#[allow(dead_code)]
mod indexer; // byte-identical to local-appview/src/indexer.rs
#[allow(dead_code)]
mod jetstream;
#[allow(dead_code)]
mod lexicon;
mod pds;
mod publisher;
#[allow(dead_code)]
mod record;
mod server; // byte-identical to local-appview/src/server.rs
mod source;
mod views;

use std::sync::{Arc, Mutex};
use std::time::Duration;

use futures_util::StreamExt;
use indexer::Indexer;
use jetstream::JetstreamSource;
use lexicon::Lexicon;
use record::{Post, Reaction, StrongRef, POST_NSID, REACTION_NSID};
use serde_json::{json, Value};
use source::RecordSource;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

fn section(t: &str) {
    println!("\n=== {t} ===");
}

fn pass(results: &mut Vec<(&'static str, bool)>, name: &'static str, ok: bool) {
    println!("  -> {}: {}", name, if ok { "PASS" } else { "FAIL" });
    results.push((name, ok));
}

#[tokio::main]
async fn main() {
    let mut results: Vec<(&'static str, bool)> = Vec::new();
    let collections = vec![POST_NSID.to_string(), REACTION_NSID.to_string()];

    // ------------------------------------------------------------------
    section("STEP 1: Start a local PDS (real XRPC over loopback)");
    // ------------------------------------------------------------------
    let pds = pds::Pds::new();
    let pds_listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let pds_addr = pds_listener.local_addr().unwrap();
    let pds_app = pds::router(pds.clone());
    tokio::spawn(async move {
        axum::serve(pds_listener, pds_app).await.unwrap();
    });
    let base = format!("http://{pds_addr}");
    println!("PDS listening at {base}");
    pass(&mut results, "1. Local PDS up on loopback", true);

    // ------------------------------------------------------------------
    section("STEP 2: Publish records via real createSession + createRecord");
    // ------------------------------------------------------------------
    let client = reqwest::Client::new();
    // Credentials: env-overridable; this local PDS auto-provisions accounts.
    let alice_id = std::env::var("ATP_IDENTIFIER").unwrap_or_else(|_| "alice.test".into());
    let alice_pw = std::env::var("ATP_APP_PASSWORD").unwrap_or_else(|_| "test-app-password".into());
    let (alice_did, alice_token) = publisher::create_session(&client, &base, &alice_id, &alice_pw).await;
    let (bob_did, bob_token) = publisher::create_session(&client, &base, "bob.test", "test-app-password").await;
    println!("session: {alice_id} -> {alice_did} (token {}…)", &alice_token[..8]);
    println!("session: bob.test  -> {bob_did}");

    // Alice publishes two posts.
    let post1 = Post::new("gm — first post on the local PDS 🌅", Some(vec!["en".into()]));
    let post1_val = serde_json::to_value(&post1).unwrap();
    let (uri1, cid1) = publisher::create_record(&client, &base, &alice_token, &alice_did, POST_NSID, &post1_val).await;
    let post2 = Post::new("local-first all the way down", None);
    let (uri2, _cid2) = publisher::create_record(&client, &base, &alice_token, &alice_did, POST_NSID, &serde_json::to_value(&post2).unwrap()).await;

    // Bob reacts to Alice's first post (strongRef uses the PDS-assigned cid).
    let reaction = Reaction::new(StrongRef { uri: uri1.clone(), cid: cid1.clone() }, "🔥");
    let (uri3, _cid3) = publisher::create_record(&client, &base, &bob_token, &bob_did, REACTION_NSID, &serde_json::to_value(&reaction).unwrap()).await;
    println!("created:\n  {uri1}\n    cid {cid1}\n  {uri2}\n  {uri3}");

    // CID parity: the PDS-assigned CID matches a locally recomputed real CIDv1.
    let local_cid1 = content_id::record_cid(&post1_val);
    let cid_parity = local_cid1 == cid1 && content_id::is_atproto_cid(&cid1);
    println!("PDS cid == locally recomputed real CIDv1? {cid_parity}");
    pass(&mut results, "2. createSession + createRecord over real HTTP; real CID parity", cid_parity);

    // ------------------------------------------------------------------
    section("STEP 3: Consume the PDS firehose over a real WebSocket");
    // ------------------------------------------------------------------
    let ws_url = format!("ws://{pds_addr}/jetstream/subscribe");
    let (mut ws, _resp) = connect_async(&ws_url).await.expect("ws connect failed");
    let mut lines: Vec<String> = Vec::new();
    loop {
        match tokio::time::timeout(Duration::from_millis(700), ws.next()).await {
            Ok(Some(Ok(Message::Text(t)))) => lines.push(t.to_string()),
            Ok(Some(Ok(Message::Close(_)))) | Ok(None) => break,
            Ok(Some(Ok(_))) => {}
            Ok(Some(Err(_))) | Err(_) => break, // error or idle timeout: caught up
        }
    }
    println!("firehose delivered {} commit frames over the WebSocket", lines.len());
    // Map with the byte-identical JetstreamSource from Phase 3b.
    let mut src = JetstreamSource::new(lines, collections.clone());
    let events = src.events();
    for ev in &events {
        println!("  {:>6}  {}", ev.action.as_str(), ev.uri());
    }
    pass(&mut results, "3. Live WebSocket firehose -> 3 RecordEvents", events.len() == 3);

    // ------------------------------------------------------------------
    section("STEP 4: Index (verbatim) + serve hydrated timeline");
    // ------------------------------------------------------------------
    let indexer = Indexer::open_in_memory();
    for ev in &events {
        let _ = indexer.apply(ev);
    }
    let state: server::Shared = Arc::new(Mutex::new(indexer));
    let app = server::router(state.clone());
    let av_listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let av_addr = av_listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(av_listener, app).await.unwrap();
    });
    let timeline: Value = client
        .get(format!("http://{av_addr}/xrpc/org.croftc.experiment.feed.getTimeline?limit=10"))
        .send().await.unwrap().json().await.unwrap();
    println!("AppView getTimeline ->\n{}", serde_json::to_string_pretty(&timeline).unwrap());

    let timeline_lex = Lexicon::load(lexicon::GET_TIMELINE_LEXICON);
    let output_ok = timeline_lex.validate_output(&timeline).is_ok();
    let feed = timeline["feed"].as_array().cloned().unwrap_or_default();
    let hydrated = feed.iter().find(|p| p["uri"] == json!(uri1)).cloned().unwrap_or(Value::Null);
    let join_ok = hydrated["reactions"].as_array().map_or(false, |r| {
        r.len() == 1 && r[0]["emoji"] == json!("🔥") && r[0]["reactor"] == json!(bob_did)
    }) && hydrated["author"] == json!(alice_did);
    println!("  output matches read lexicon? {output_ok}; reaction (by Bob) hydrated onto Alice's post? {join_ok}");
    pass(&mut results, "4. End-to-end: PDS -> firehose -> AppView serves hydrated view", output_ok && join_ok && feed.len() == 2);

    // ------------------------------------------------------------------
    section("SUMMARY");
    // ------------------------------------------------------------------
    let mut all = true;
    for (name, ok) in &results {
        println!("[{}] {}", if *ok { "PASS" } else { "FAIL" }, name);
        all &= *ok;
    }
    println!("\nOVERALL: {}", if all { "PASS — live flip mechanics proven end to end on a local PDS over real sockets" } else { "FAIL" });

    section("VERSION REPORT");
    println!("rustc               : {}", env!("SLICE_RUSTC_VERSION"));
    println!("axum                : {}", env!("SLICE_VER_AXUM"));
    println!("tokio               : {}", env!("SLICE_VER_TOKIO"));
    println!("reqwest             : {}", env!("SLICE_VER_REQWEST"));
    println!("tokio-tungstenite   : {}", env!("SLICE_VER_TUNGSTENITE"));
    println!("rusqlite            : {}", env!("SLICE_VER_RUSQLITE"));
    println!("cid                 : {}", env!("SLICE_VER_CID"));
    println!("serde_json          : {}", env!("SLICE_VER_SERDE_JSON"));

    if !all {
        std::process::exit(1);
    }
}
