//! Phase 3a: a source-agnostic local AppView over the encrypted sync stack.
//!
//! An AppView does three things: ingest records from an event source, index them
//! into a disposable query store, and serve a read API. The whole point of this
//! phase is the **source-agnostic** boundary: the indexer and server consume
//! records only through the `RecordSource` trait and never touch Automerge, MLS,
//! encryption, or iroh. We feed it from a `LocalStackSource` over our own
//! decrypted group document. Phase 3b swaps in a `JetstreamSource` — same
//! `RecordEvent`, no change to indexer or server.
//!
//! Module roles:
//!   BEHIND the boundary (private stack): address, crypto, doc, groupdoc, mls,
//!     record, lexicon, store, and `source::LocalStackSource`.
//!   THE boundary: `source::{RecordSource, RecordEvent}`.
//!   IN FRONT (source-agnostic): indexer (SQLite), server (axum XRPC), views.

mod address;
mod crypto;
mod doc;
mod groupdoc;
mod indexer;
mod lexicon;
mod mls;
mod record;
mod server;
mod source;
mod store;
mod views;

use std::sync::{Arc, Mutex};

use address::Address;
use indexer::Indexer;
use lexicon::Lexicon;
use record::{Post, Reaction, StrongRef, POST_NSID, REACTION_NSID};
use serde_json::{json, Value};
use source::{Action, LocalStackSource, RecordEvent, RecordSource};
use store::BlobStore;

const NAMESPACE: &[u8] = b"org.croftc.experiment.group/demo";

fn did_of(m: &mls::Member) -> String {
    format!("did:key:z{}", hex::encode(&m.identity()[..16]))
}

fn section(t: &str) {
    println!("\n=== {t} ===");
}

/// Percent-encode a query-parameter value (RFC 3986 unreserved set kept as-is).
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

fn pass(results: &mut Vec<(&'static str, bool)>, name: &'static str, ok: bool) {
    println!("  -> {}: {}", name, if ok { "PASS" } else { "FAIL" });
    results.push((name, ok));
}

#[tokio::main]
async fn main() {
    let mut results: Vec<(&'static str, bool)> = Vec::new();

    // ------------------------------------------------------------------
    section("STEP 1: Stack setup (reused, proven) — sync an encrypted group doc");
    // ------------------------------------------------------------------
    let alice = mls::Member::new("Alice");
    let bob = mls::Member::new("Bob");
    let mut alice_group = mls::create_group(&alice);
    let welcome = mls::add_member(&mut alice_group, &alice, &bob);
    let bob_group = mls::join_from_welcome(&bob, &welcome);
    let key_alice = alice.content_key(&alice_group);
    let key_bob = bob.content_key(&bob_group);
    let (alice_did, bob_did) = (did_of(&alice), did_of(&bob));
    println!("Alice DID {alice_did}\nBob   DID {bob_did}\nepoch keys match? {}", key_alice == key_bob);

    // Alice authors two posts into her sub-repo.
    let mut doc_alice = doc::new_doc();
    let p1_rkey = record::new_tid();
    let post1 = Post::new("gm, group ☀️", Some(vec!["en".into()]));
    let post1_json = serde_json::to_string(&post1).unwrap();
    groupdoc::put_record(&mut doc_alice, &alice_did, POST_NSID, &p1_rkey, &post1_json);

    let p2_rkey = record::new_tid();
    let post2 = Post::new("local-first feels different", None);
    groupdoc::put_record(&mut doc_alice, &alice_did, POST_NSID, &p2_rkey, &serde_json::to_string(&post2).unwrap());

    // Encrypt a snapshot; Bob bootstraps it (real crypto path).
    let mut store = BlobStore::new();
    let snap = doc::snapshot(&mut doc_alice);
    let ct = crypto::encrypt(&key_alice, &snap);
    let addr = Address::new(NAMESPACE, alice.identity(), b"/group/doc".to_vec(), 1);
    let h = store.put(&ct);
    store.set_pointer(&addr.storage_key(), h);
    let fetched = store.resolve(&addr.storage_key()).unwrap().to_vec();
    let mut doc_bob = doc::load(&crypto::decrypt(&key_bob, &fetched).unwrap());

    // Bob reacts to post1 in his sub-repo.
    let r1_rkey = record::new_tid();
    let subject = StrongRef {
        uri: record::at_uri(&alice_did, POST_NSID, &p1_rkey),
        cid: record::cid_for(&post1_json),
    };
    let reaction = Reaction::new(subject, "🔥");
    groupdoc::put_record(&mut doc_bob, &bob_did, REACTION_NSID, &r1_rkey, &serde_json::to_string(&reaction).unwrap());

    let all = groupdoc::list_all(&doc_bob);
    println!("Bob's decrypted group doc holds {} records (2 posts + 1 reaction)", all.len());
    pass(&mut results, "1. Encrypted group doc synced with posts + reaction", all.len() == 3 && key_alice == key_bob);

    // ------------------------------------------------------------------
    section("STEP 2: Ingest through the source-agnostic boundary -> SQLite");
    // ------------------------------------------------------------------
    let mut src = LocalStackSource::new(&doc_bob);
    let events = src.events();
    for ev in &events {
        println!("  event: {:>6}  {}  ({})", ev.action.as_str(), ev.uri(), ev.collection);
    }
    let indexer = Indexer::open_in_memory();
    let mut indexed = 0;
    for ev in &events {
        match indexer.apply(ev) {
            Ok(_) => indexed += 1,
            Err(e) => println!("  indexer rejected: {e}"),
        }
    }
    // Inject a malformed record (missing required createdAt) to show rejection.
    let bad = RecordEvent {
        action: Action::Create,
        did: alice_did.clone(),
        collection: POST_NSID.to_string(),
        rkey: "badrecord".to_string(),
        cid: None,
        record: json!({"$type": POST_NSID, "text": "no createdAt field"}),
        observed_at: source::now_millis(),
    };
    let rejected = indexer.apply(&bad);
    println!("  malformed record result: {rejected:?}");
    let count = indexer.count();
    println!("  indexed {indexed} valid records; row count = {count}");
    pass(&mut results, "2. Ingest validates + indexes; rejects malformed", indexed == 3 && rejected.is_err() && count == 3);

    // ------------------------------------------------------------------
    section("STEP 3: Serve + query (real HTTP XRPC)");
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

    let timeline: Value = client
        .get(format!("{base}/xrpc/org.croftc.experiment.feed.getTimeline?limit=10"))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    println!("GET /xrpc/...getTimeline ->\n{}", serde_json::to_string_pretty(&timeline).unwrap());

    let post1_uri = record::at_uri(&alice_did, POST_NSID, &p1_rkey);
    let thread: Value = client
        .get(format!("{base}/xrpc/org.croftc.experiment.feed.getPostThread?uri={}", pct(&post1_uri)))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    println!("GET /xrpc/...getPostThread?uri={post1_uri} ->\n{}", serde_json::to_string_pretty(&thread).unwrap());

    let served_ok = timeline["feed"].as_array().map_or(false, |f| f.len() == 2)
        && thread["thread"]["uri"] == json!(post1_uri);
    pass(&mut results, "3. XRPC server returns timeline + thread", served_ok);

    // ------------------------------------------------------------------
    section("STEP 4: Hydration assertion (the keystone)");
    // ------------------------------------------------------------------
    let timeline_lex = Lexicon::load(lexicon::GET_TIMELINE_LEXICON);
    let thread_lex = Lexicon::load(lexicon::GET_POST_THREAD_LEXICON);
    let timeline_valid = timeline_lex.validate_output(&timeline);
    let thread_valid = thread_lex.validate_output(&thread);
    println!("  timeline output matches getTimeline lexicon? {timeline_valid:?}");
    println!("  thread   output matches getPostThread lexicon? {thread_valid:?}");

    // The post that was reacted to must carry its reaction (a real join, not a
    // flat echo): reactions non-empty and reactor == Bob's DID.
    let feed = timeline["feed"].as_array().cloned().unwrap_or_default();
    let hydrated = feed.iter().find(|p| p["uri"] == json!(post1_uri)).cloned().unwrap_or(Value::Null);
    println!("  fully-hydrated post-with-reactions view:\n{}", serde_json::to_string_pretty(&hydrated).unwrap());
    let reactions = hydrated["reactions"].as_array().cloned().unwrap_or_default();
    let join_ok = reactions.len() == 1
        && reactions[0]["emoji"] == json!("🔥")
        && reactions[0]["reactor"] == json!(bob_did)
        && hydrated["author"] == json!(alice_did);
    pass(
        &mut results,
        "4. Hydration: timeline joins posts+reactions; output matches lexicon",
        timeline_valid.is_ok() && thread_valid.is_ok() && join_ok,
    );

    // ------------------------------------------------------------------
    section("STEP 5: Rebuild proof (index is a disposable projection)");
    // ------------------------------------------------------------------
    {
        let idx = state.lock().unwrap();
        idx.rebuild();
        for ev in &events {
            let _ = idx.apply(ev);
        }
        println!("  wiped + replayed from source; row count = {}", idx.count());
    }
    let timeline2: Value = client
        .get(format!("{base}/xrpc/org.croftc.experiment.feed.getTimeline?limit=10"))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    let rebuild_ok = timeline2 == timeline;
    println!("  query results identical after rebuild? {rebuild_ok}");
    pass(&mut results, "5. Rebuild from source yields identical query results", rebuild_ok);

    // ------------------------------------------------------------------
    section("SUMMARY");
    // ------------------------------------------------------------------
    let mut all = true;
    for (name, ok) in &results {
        println!("[{}] {}", if *ok { "PASS" } else { "FAIL" }, name);
        all &= *ok;
    }
    println!("\nOVERALL: {}", if all { "PASS — source-agnostic AppView composes with the encrypted stack" } else { "FAIL" });

    section("VERSION REPORT");
    println!("rustc               : {}", env!("SLICE_RUSTC_VERSION"));
    println!("automerge           : {}", env!("SLICE_VER_AUTOMERGE"));
    println!("openmls             : {}", env!("SLICE_VER_OPENMLS"));
    println!("chacha20poly1305    : {}", env!("SLICE_VER_CHACHA"));
    println!("rusqlite            : {}", env!("SLICE_VER_RUSQLITE"));
    println!("axum                : {}", env!("SLICE_VER_AXUM"));
    println!("tokio               : {}", env!("SLICE_VER_TOKIO"));
    println!("reqwest             : {}", env!("SLICE_VER_REQWEST"));
    println!("serde_json          : {}", env!("SLICE_VER_SERDE_JSON"));
    println!("iroh (resolvable, NOT linked — transport stubbed): 0.98.2 / iroh-blobs 0.102.0");

    if !all {
        std::process::exit(1);
    }
}
