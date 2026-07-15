//! Phase 6: the full end-to-end slice in ONE program. Chains every prior phase:
//!
//!   encrypted MLS group (Phase 1) + lexicon records (Phase 2)
//!     -> public/private mirror with redaction (Phase 4)
//!     -> publish public records to a real local PDS (Phase 5)
//!     -> firehose over a real WebSocket -> AppView (Phases 3a/3b)
//!     -> hydrated, lexicon-valid public timeline
//!
//! The point: see the model end to end, and surface the integration issues the
//! isolated phases couldn't (AT-URI identity change at the boundary, group<->PDS
//! identity mapping, CID consistency, end-to-end non-leakage). Fully offline
//! (local PDS on loopback), since the live network is egress-blocked here.

mod bridge;
#[allow(dead_code)]
mod content_id;
mod crypto;
mod doc;
mod groupdoc;
#[allow(dead_code)]
mod indexer;
#[allow(dead_code)]
mod jetstream;
#[allow(dead_code)]
mod lexicon;
mod mls;
mod pds;
mod publisher;
#[allow(dead_code)]
mod record;
mod server;
#[allow(dead_code)]
mod source;
mod views;
mod visibility;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use bridge::Identity;
use futures_util::StreamExt;
use indexer::Indexer;
use jetstream::JetstreamSource;
use lexicon::Lexicon;
use record::{Post, Reaction, StrongRef, POST_NSID, REACTION_NSID};
use serde_json::{json, Value};
use source::RecordSource;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use visibility::{MirrorPolicy, Visibility};

const SECRET: &str = "internal: Q3 budget is 50000 USD";

/// Group-side identity (MLS-derived) — distinct from the PDS account DID.
fn group_did(m: &mls::Member) -> String {
    format!("did:key:z{}", hex::encode(&m.identity()[..16]))
}

fn section(t: &str) {
    println!("\n=== {t} ===");
}
fn pass(r: &mut Vec<(&'static str, bool)>, name: &'static str, ok: bool) {
    println!("  -> {}: {}", name, if ok { "PASS" } else { "FAIL" });
    r.push((name, ok));
}

#[tokio::main]
async fn main() {
    let mut results: Vec<(&'static str, bool)> = Vec::new();
    let mut issues: Vec<String> = Vec::new();
    let collections = vec![POST_NSID.to_string(), REACTION_NSID.to_string()];

    // ==================================================================
    section("STEP 1: Encrypted private group (MLS + CRDT) with mixed visibility");
    // ==================================================================
    let alice = mls::Member::new("Alice");
    let bob = mls::Member::new("Bob");
    let mut ag = mls::create_group(&alice);
    let welcome = mls::add_member(&mut ag, &alice, &bob);
    let bg = mls::join_from_welcome(&bob, &welcome);
    let (key_a, key_b) = (alice.content_key(&ag), bob.content_key(&bg));
    let (gdid_a, gdid_b) = (group_did(&alice), group_did(&bob));

    let mut policy = MirrorPolicy::new();

    // Alice authors her posts.
    let mut doc_a = doc::new_doc();
    let a_pub = record::new_tid();
    let a_pub_post = Post::new("Announcing our public launch 🚀", Some(vec!["en".into()]));
    let a_pub_val = serde_json::to_value(&a_pub_post).unwrap();
    groupdoc::put_record(&mut doc_a, &gdid_a, POST_NSID, &a_pub, &serde_json::to_string(&a_pub_post).unwrap());
    policy.set(&record::at_uri(&gdid_a, POST_NSID, &a_pub), Visibility::Public);

    let a_pub2 = record::new_tid();
    let a_pub2_post = Post::new("local-first, lexicon-shaped, end-to-end", None);
    groupdoc::put_record(&mut doc_a, &gdid_a, POST_NSID, &a_pub2, &serde_json::to_string(&a_pub2_post).unwrap());
    policy.set(&record::at_uri(&gdid_a, POST_NSID, &a_pub2), Visibility::Public);

    let a_priv = record::new_tid();
    let a_priv_post = Post::new(SECRET, None);
    let a_priv_val = serde_json::to_value(&a_priv_post).unwrap();
    groupdoc::put_record(&mut doc_a, &gdid_a, POST_NSID, &a_priv, &serde_json::to_string(&a_priv_post).unwrap());
    policy.set(&record::at_uri(&gdid_a, POST_NSID, &a_priv), Visibility::Private);

    // Encrypt + sync to Bob (real crypto path); confirm secret not in ciphertext.
    let snapshot = doc::snapshot(&mut doc_a);
    let ciphertext = crypto::encrypt(&key_a, &snapshot);
    let ct_leaks = String::from_utf8_lossy(&ciphertext).contains(SECRET);
    let mut doc_b = doc::load(&crypto::decrypt(&key_b, &ciphertext).expect("bob decrypts"));

    // Bob reacts: one to the public post (fine), one to the private post (leaky).
    let r_pub = record::new_tid();
    let r_pub_reaction = Reaction::new(
        StrongRef { uri: record::at_uri(&gdid_a, POST_NSID, &a_pub), cid: content_id::record_cid(&a_pub_val) },
        "🎉",
    );
    groupdoc::put_record(&mut doc_b, &gdid_b, REACTION_NSID, &r_pub, &serde_json::to_string(&r_pub_reaction).unwrap());
    policy.set(&record::at_uri(&gdid_b, REACTION_NSID, &r_pub), Visibility::Public);

    let r_priv = record::new_tid();
    let r_priv_reaction = Reaction::new(
        StrongRef { uri: record::at_uri(&gdid_a, POST_NSID, &a_priv), cid: content_id::record_cid(&a_priv_val) },
        "👀",
    );
    groupdoc::put_record(&mut doc_b, &gdid_b, REACTION_NSID, &r_priv, &serde_json::to_string(&r_priv_reaction).unwrap());
    policy.set(&record::at_uri(&gdid_b, REACTION_NSID, &r_priv), Visibility::Public);

    let n = groupdoc::list_all(&doc_b).len();
    println!("group doc: {n} records (3 posts + 2 reactions), encrypted at rest; ct leaks secret? {ct_leaks}; keys match? {}", key_a == key_b);
    pass(&mut results, "1. Encrypted group synced; secret not in ciphertext", n == 5 && key_a == key_b && !ct_leaks);

    // ==================================================================
    section("STEP 2: Start local PDS; map group identities -> PDS accounts");
    // ==================================================================
    let pds = pds::Pds::new();
    let pl = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let pds_addr = pl.local_addr().unwrap();
    let app = pds::router(pds.clone());
    tokio::spawn(async move { axum::serve(pl, app).await.unwrap(); });
    let base = format!("http://{pds_addr}");
    let client = reqwest::Client::new();

    let (adid, atok) = publisher::create_session(&client, &base, "alice.test", "pw").await;
    let (bdid, btok) = publisher::create_session(&client, &base, "bob.test", "pw").await;
    let mut identities: HashMap<String, Identity> = HashMap::new();
    identities.insert(gdid_a.clone(), Identity { pds_did: adid.clone(), token: atok });
    identities.insert(gdid_b.clone(), Identity { pds_did: bdid.clone(), token: btok });
    println!("identity map (issue surfaced — group id != PDS DID, needs a verifiable binding):");
    println!("  {gdid_a}  ->  {adid}");
    println!("  {gdid_b}  ->  {bdid}");
    issues.push("Group identity (MLS-derived did:key) and PDS account DID (did:plc) are distinct identifiers for the same principal; production needs a verifiable binding (signed linkage) so an AppView can trust 'this DID is this group member'.".into());
    pass(&mut results, "2. PDS up; every group author mapped to a PDS account", identities.len() == 2);

    // ==================================================================
    section("STEP 3: Mirror-publish public records to the PDS (boundary crossing)");
    // ==================================================================
    let mut log = Vec::new();
    let (stats, uri_map) = bridge::mirror_publish(&client, &base, &doc_b, &policy, &identities, &mut log).await;
    for l in &log {
        println!("  {l}");
    }
    println!("  {stats:?}");
    issues.push("Crossing the boundary changes a record's AT-URI (at://<groupDid>/.. -> at://<pdsDid>/<pdsRkey>); every strongRef (reaction subject) had to be REWRITTEN to the published URI/CID or it would dangle.".into());
    pass(&mut results, "3. Published 2 posts + 1 reaction; 1 private kept; 1 leaky redacted",
        stats.published_posts == 2 && stats.published_reactions == 1 && stats.kept_private == 1 && stats.redacted_refs == 1);

    // CID consistency: the group-side CID equals the PDS-assigned CID (parity).
    let a_pub_group_uri = record::at_uri(&gdid_a, POST_NSID, &a_pub);
    let (_pub_uri, pub_cid) = uri_map.get(&a_pub_group_uri).cloned().unwrap_or_default();
    let cid_parity = pub_cid == content_id::record_cid(&a_pub_val);
    println!("  CID parity (group-side == PDS-assigned for the public post)? {cid_parity}");
    pass(&mut results, "3b. CID parity across the boundary", cid_parity);

    // ==================================================================
    section("STEP 4: Firehose (WebSocket) -> AppView -> hydrated public timeline");
    // ==================================================================
    let (mut ws, _) = connect_async(format!("ws://{pds_addr}/jetstream/subscribe")).await.unwrap();
    let mut lines: Vec<String> = Vec::new();
    loop {
        match tokio::time::timeout(Duration::from_millis(700), ws.next()).await {
            Ok(Some(Ok(Message::Text(t)))) => lines.push(t.to_string()),
            _ => break,
        }
    }
    let firehose_leaks = lines.iter().any(|l| l.contains(SECRET));
    let mut src = JetstreamSource::new(lines, collections.clone());
    let events = src.events();
    let indexer = Indexer::open_in_memory();
    for ev in &events {
        let _ = indexer.apply(ev);
    }
    let state: server::Shared = Arc::new(Mutex::new(indexer));
    let avapp = server::router(state.clone());
    let al = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let av = al.local_addr().unwrap();
    tokio::spawn(async move { axum::serve(al, avapp).await.unwrap(); });
    let timeline: Value = client
        .get(format!("http://{av}/xrpc/org.croftc.experiment.feed.getTimeline?limit=10"))
        .send().await.unwrap().json().await.unwrap();
    println!("AppView getTimeline ->\n{}", serde_json::to_string_pretty(&timeline).unwrap());

    let timeline_lex = Lexicon::load(lexicon::GET_TIMELINE_LEXICON);
    let out_ok = timeline_lex.validate_output(&timeline).is_ok();
    let feed = timeline["feed"].as_array().cloned().unwrap_or_default();
    let pub_post_pub_uri = uri_map.get(&a_pub_group_uri).map(|(u, _)| u.clone()).unwrap_or_default();
    let hydrated = feed.iter().find(|p| p["uri"] == json!(pub_post_pub_uri)).cloned().unwrap_or(Value::Null);
    let join_ok = hydrated["reactions"].as_array().map_or(false, |r| {
        r.len() == 1 && r[0]["emoji"] == json!("🎉") && r[0]["reactor"] == json!(bdid)
    }) && hydrated["author"] == json!(adid);
    println!("  timeline output valid? {out_ok}; 2 public posts? {}; reaction hydrated w/ rewritten ref? {join_ok}", feed.len() == 2);
    pass(&mut results, "4. AppView serves 2 public posts; reaction hydrated via rewritten ref",
        out_ok && feed.len() == 2 && join_ok);

    // ==================================================================
    section("STEP 5: End-to-end non-leakage + identity continuity (keystone)");
    // ==================================================================
    let timeline_str = timeline.to_string();
    let priv_uri = record::at_uri(&gdid_a, POST_NSID, &a_priv);
    let leaks_secret = firehose_leaks || timeline_str.contains(SECRET);
    let leaks_priv_uri = timeline_str.contains(&priv_uri) || timeline_str.contains(&a_priv);
    let priv_absent = !feed.iter().any(|p| p["text"] == json!(SECRET));
    println!("  secret leaked downstream (firehose/timeline)? {leaks_secret}");
    println!("  private post URI/rkey present in timeline? {leaks_priv_uri}");
    println!("  private post absent from public timeline? {priv_absent}");
    println!("  identity continuity: author={adid} reactor={bdid} (both PDS DIDs)");
    pass(&mut results, "5. No private data crosses to the public read path; identity continuous",
        !leaks_secret && !leaks_priv_uri && priv_absent);

    // ==================================================================
    section("SUMMARY");
    // ==================================================================
    let mut all = true;
    for (name, ok) in &results {
        println!("[{}] {}", if *ok { "PASS" } else { "FAIL" }, name);
        all &= *ok;
    }
    println!("\nOVERALL: {}", if all { "PASS — full model proven end to end: encrypted group -> split -> PDS -> firehose -> AppView" } else { "FAIL" });

    section("ISSUES SURFACED (for the model, to address before production)");
    for (i, issue) in issues.iter().enumerate() {
        println!("  {}. {issue}", i + 1);
    }
    println!("  3. rkey changes on publish (PDS-assigned), so a record's identity is not stable group->public; clients must track the mapping or pin rkeys at creation.");
    println!("  4. Single committer for membership; concurrent membership commits + CRDT fork resolution are still unaddressed (deferred since Phase 1).");
    println!("  5. Transport (iroh) is still stubbed; the public side runs over real sockets, the private side does not yet.");

    section("VERSION REPORT");
    println!("rustc {} | automerge {} | openmls {} | chacha20poly1305 {}", env!("SLICE_RUSTC_VERSION"), env!("SLICE_VER_AUTOMERGE"), env!("SLICE_VER_OPENMLS"), env!("SLICE_VER_CHACHA"));
    println!("axum {} | tokio {} | reqwest {} | tokio-tungstenite {} | rusqlite {}", env!("SLICE_VER_AXUM"), env!("SLICE_VER_TOKIO"), env!("SLICE_VER_REQWEST"), env!("SLICE_VER_TUNGSTENITE"), env!("SLICE_VER_RUSQLITE"));
    println!("cid {} | serde_ipld_dagcbor {} | serde_json {}", env!("SLICE_VER_CID"), env!("SLICE_VER_DAGCBOR"), env!("SLICE_VER_SERDE_JSON"));

    if !all {
        std::process::exit(1);
    }
}
