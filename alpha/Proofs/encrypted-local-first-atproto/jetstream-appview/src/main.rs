//! Phase 3b (in-discipline): feed the Phase 3a AppView from the REAL Jetstream
//! wire format instead of the local stack — proving the ingest swap is additive.
//!
//! What is real here: the Jetstream commit-event JSON shape, the
//! `JetstreamSource: RecordSource` mapping (kind/collection filtering,
//! operation→action, cursor/resume, create/update/delete), genuine atproto
//! CIDv1 (DAG-CBOR/SHA-256) content ids, and a byte-identical `indexer.rs` /
//! `server.rs` / `views.rs` carried from 3a.
//!
//! What is still stubbed (the only remaining 3b work, flagged in the README):
//! the live network — publishing to a real PDS via OAuth and connecting to a
//! live Jetstream WebSocket. The feed here is a synthetic NDJSON log of our own
//! records in the exact wire shape, so no outbound network or credentials are
//! used. Pointing the socket at a live endpoint is the final flip.

mod content_id;
// indexer/record are carried verbatim; not every helper (rebuild, cid_for) is
// exercised in this phase. allow(dead_code) keeps the files byte-identical.
#[allow(dead_code)]
mod indexer; // byte-identical to local-appview/src/indexer.rs
mod jetstream;
mod lexicon;
#[allow(dead_code)]
mod record;
mod server; // byte-identical to local-appview/src/server.rs
mod source;
mod views;

use std::sync::{Arc, Mutex};

use indexer::Indexer;
use jetstream::JetstreamSource;
use lexicon::Lexicon;
use record::{Post, Reaction, StrongRef, POST_NSID, REACTION_NSID};
use serde_json::{json, Value};
use source::RecordSource;

fn section(t: &str) {
    println!("\n=== {t} ===");
}

fn pass(results: &mut Vec<(&'static str, bool)>, name: &'static str, ok: bool) {
    println!("  -> {}: {}", name, if ok { "PASS" } else { "FAIL" });
    results.push((name, ok));
}

fn pct(s: &str) -> String {
    let mut out = String::new();
    for b in s.bytes() {
        if b.is_ascii_alphanumeric() || matches!(b, b'-' | b'.' | b'_' | b'~') {
            out.push(b as char);
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}

#[tokio::main]
async fn main() {
    let mut results: Vec<(&'static str, bool)> = Vec::new();
    let collections = vec![POST_NSID.to_string(), REACTION_NSID.to_string()];

    // Synthetic authors (real DIDs arrive via identity/OAuth in live 3b).
    let alice_did = "did:plc:alice000000000000000000";
    let bob_did = "did:plc:bob0000000000000000000000";

    // ------------------------------------------------------------------
    section("STEP 0: Author records + build a real-wire Jetstream feed");
    // ------------------------------------------------------------------
    let p1_rkey = record::new_tid();
    let post1 = Post::new("gm from the firehose 🔥", Some(vec!["en".into()]));
    let post1_val = serde_json::to_value(&post1).unwrap();
    let post1_cid = content_id::record_cid(&post1_val);

    let p2_rkey = record::new_tid();
    let post2 = Post::new("first draft", None);
    let post2_val = serde_json::to_value(&post2).unwrap();
    let post2_cid = content_id::record_cid(&post2_val);

    let r1_rkey = record::new_tid();
    let reaction = Reaction::new(
        StrongRef { uri: record::at_uri(alice_did, POST_NSID, &p1_rkey), cid: post1_cid.clone() },
        "🎉",
    );
    let reaction_val = serde_json::to_value(&reaction).unwrap();
    let reaction_cid = content_id::record_cid(&reaction_val);

    // Batch 1 — initial commits, plus an identity event that must be skipped.
    let mut t = 1_727_000_000_000_000u64; // microseconds
    let mut next = || {
        t += 1_000;
        t
    };
    let batch1 = vec![
        jetstream::commit_line(alice_did, next(), "create", POST_NSID, &p1_rkey, Some(&post1_val), Some(&post1_cid)),
        jetstream::commit_line(alice_did, next(), "create", POST_NSID, &p2_rkey, Some(&post2_val), Some(&post2_cid)),
        jetstream::identity_line(bob_did, next()),
        jetstream::commit_line(bob_did, next(), "create", REACTION_NSID, &r1_rkey, Some(&reaction_val), Some(&reaction_cid)),
    ];
    println!("Sample wire event (a create commit):\n{}", {
        let v: Value = serde_json::from_str(&batch1[0]).unwrap();
        serde_json::to_string_pretty(&v).unwrap()
    });
    let cid_real = content_id::is_atproto_cid(&post1_cid);
    println!("post1 cid = {post1_cid}\n  is a real atproto CIDv1 (dag-cbor)? {cid_real}");
    pass(&mut results, "0. Records encoded as real Jetstream events with real CIDs", cid_real);

    // ------------------------------------------------------------------
    section("STEP 1: Ingest via JetstreamSource -> identical indexer");
    // ------------------------------------------------------------------
    let indexer = Indexer::open_in_memory();
    let mut src = JetstreamSource::new(batch1.clone(), collections.clone());
    let events = src.events();
    for ev in &events {
        println!("  {:>6}  {}  cid={}", ev.action.as_str(), ev.uri(), ev.cid.as_deref().unwrap_or("-"));
    }
    let mut indexed = 0;
    for ev in &events {
        match indexer.apply(ev) {
            Ok(_) => indexed += 1,
            Err(e) => println!("  rejected: {e}"),
        }
    }
    let cursor1 = src.cursor();
    println!("  emitted {} events (identity skipped), indexed {indexed}, row count {}, cursor={cursor1}",
        events.len(), indexer.count());
    pass(&mut results, "1. JetstreamSource maps commits; identity skipped; indexed",
        events.len() == 3 && indexed == 3 && indexer.count() == 3);

    // ------------------------------------------------------------------
    section("STEP 2: Serve + query (same axum server) + hydration assertion");
    // ------------------------------------------------------------------
    let state: server::Shared = Arc::new(Mutex::new(indexer));
    let app = server::router(state.clone());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let bound = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    let base = format!("http://{bound}");
    let client = reqwest::Client::new();

    let post1_uri = record::at_uri(alice_did, POST_NSID, &p1_rkey);
    let timeline: Value = client
        .get(format!("{base}/xrpc/org.croftc.experiment.feed.getTimeline?limit=10"))
        .send().await.unwrap().json().await.unwrap();
    println!("getTimeline ->\n{}", serde_json::to_string_pretty(&timeline).unwrap());

    let timeline_lex = Lexicon::load(lexicon::GET_TIMELINE_LEXICON);
    let output_ok = timeline_lex.validate_output(&timeline).is_ok();
    let feed = timeline["feed"].as_array().cloned().unwrap_or_default();
    let hydrated = feed.iter().find(|p| p["uri"] == json!(post1_uri)).cloned().unwrap_or(Value::Null);
    let join_ok = hydrated["reactions"].as_array().map_or(false, |r| {
        r.len() == 1 && r[0]["emoji"] == json!("🎉") && r[0]["reactor"] == json!(bob_did)
    }) && hydrated["author"] == json!(alice_did);
    println!("  output matches getTimeline lexicon? {output_ok}; reaction hydrated onto post1? {join_ok}");
    pass(&mut results, "2. Same server serves hydrated, lexicon-valid timeline",
        output_ok && join_ok && feed.len() == 2);

    // ------------------------------------------------------------------
    section("STEP 3: Resume from cursor — apply update + delete");
    // ------------------------------------------------------------------
    // Batch 2 arrives later: Alice edits post2; Bob removes his reaction.
    let post2_edited = Post {
        type_: POST_NSID.to_string(),
        text: "edited: shipping it 🚀".to_string(),
        created_at: post2.created_at.clone(),
        langs: None,
    };
    let post2_edited_val = serde_json::to_value(&post2_edited).unwrap();
    let post2_edited_cid = content_id::record_cid(&post2_edited_val);
    let mut full_feed = batch1.clone();
    full_feed.push(jetstream::commit_line(alice_did, next(), "update", POST_NSID, &p2_rkey, Some(&post2_edited_val), Some(&post2_edited_cid)));
    full_feed.push(jetstream::commit_line(bob_did, next(), "delete", REACTION_NSID, &r1_rkey, None, None));

    // Resume: feed the FULL log but from cursor1, so batch-1 events are skipped.
    let mut src2 = JetstreamSource::from_cursor(full_feed, collections.clone(), cursor1);
    let events2 = src2.events();
    println!("  resumed at cursor {cursor1}; new events: {}", events2.len());
    {
        let idx = state.lock().unwrap();
        for ev in &events2 {
            let _ = idx.apply(ev);
        }
    }
    let timeline2: Value = client
        .get(format!("{base}/xrpc/org.croftc.experiment.feed.getTimeline?limit=10"))
        .send().await.unwrap().json().await.unwrap();
    let thread2: Value = client
        .get(format!("{base}/xrpc/org.croftc.experiment.feed.getPostThread?uri={}", pct(&post1_uri)))
        .send().await.unwrap().json().await.unwrap();

    let post2_uri = record::at_uri(alice_did, POST_NSID, &p2_rkey);
    let post2_view = timeline2["feed"].as_array().unwrap().iter()
        .find(|p| p["uri"] == json!(post2_uri)).cloned().unwrap_or(Value::Null);
    let update_applied = post2_view["text"] == json!("edited: shipping it 🚀");
    let delete_applied = thread2["thread"]["reactions"].as_array().map_or(false, |r| r.is_empty());
    // The post-thread response still conforms to its read lexicon after edits.
    let thread_lex = Lexicon::load(lexicon::GET_POST_THREAD_LEXICON);
    let thread_output_ok = thread_lex.validate_output(&thread2).is_ok();
    println!("  resume processed only batch-2 ({} events); post2 updated? {update_applied}; reaction deleted? {delete_applied}; thread output valid? {thread_output_ok}",
        events2.len());
    pass(&mut results, "3. Cursor resume applies update + delete correctly",
        events2.len() == 2 && update_applied && delete_applied && thread_output_ok && src2.cursor() > cursor1);

    // ------------------------------------------------------------------
    section("STEP 4: Swap proof — indexer/server are byte-identical to 3a");
    // ------------------------------------------------------------------
    let indexer_same = include_str!("indexer.rs") == include_str!("../../local-appview/src/indexer.rs");
    let server_same = include_str!("server.rs") == include_str!("../../local-appview/src/server.rs");
    let views_same = include_str!("views.rs") == include_str!("../../local-appview/src/views.rs");
    println!("  indexer.rs identical to 3a? {indexer_same}");
    println!("  server.rs  identical to 3a? {server_same}");
    println!("  views.rs   identical to 3a? {views_same}");
    pass(&mut results, "4. Ingest swap required zero indexer/server/views changes",
        indexer_same && server_same && views_same);

    // ------------------------------------------------------------------
    section("SUMMARY");
    // ------------------------------------------------------------------
    let mut all = true;
    for (name, ok) in &results {
        println!("[{}] {}", if *ok { "PASS" } else { "FAIL" }, name);
        all &= *ok;
    }
    println!("\nOVERALL: {}", if all { "PASS — AppView fed from the real Jetstream wire format; swap is additive" } else { "FAIL" });

    section("VERSION REPORT");
    println!("rustc               : {}", env!("SLICE_RUSTC_VERSION"));
    println!("rusqlite            : {}", env!("SLICE_VER_RUSQLITE"));
    println!("axum                : {}", env!("SLICE_VER_AXUM"));
    println!("tokio               : {}", env!("SLICE_VER_TOKIO"));
    println!("reqwest             : {}", env!("SLICE_VER_REQWEST"));
    println!("cid                 : {}", env!("SLICE_VER_CID"));
    println!("serde_ipld_dagcbor  : {}", env!("SLICE_VER_DAGCBOR"));
    println!("multihash-codetable : {}", env!("SLICE_VER_MULTIHASH"));
    println!("serde_json          : {}", env!("SLICE_VER_SERDE_JSON"));
    println!("(prior phases unchanged: automerge 0.7.4 / openmls 0.8.1 / chacha20poly1305 0.10.1)");

    if !all {
        std::process::exit(1);
    }
}
