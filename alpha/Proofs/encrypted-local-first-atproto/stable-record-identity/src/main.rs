//! Phase 9: stable cross-boundary record identity.
//!
//! Phase 6 found that publishing reassigned rkeys, so a record's identity
//! changed group->public and every strongRef needed a lookup-table rewrite. The
//! fix: **pin the rkey at creation** and publish with that exact rkey. Then the
//! public URI is a *pure authority rewrite* of the group URI (swap the DID, keep
//! collection+rkey) — no lookup table — and the binding from Phase 8 makes the
//! authority swap verifiable. Plus: idempotent re-publish, and edits via
//! putRecord that keep the URI stable while the CID changes.

#[allow(dead_code)]
mod content_id;
#[allow(dead_code)]
mod lexicon;
mod pds;
mod publisher;
#[allow(dead_code)]
mod record;

use record::{Post, POST_NSID};
use reqwest::StatusCode;
use serde_json::json;

fn section(t: &str) {
    println!("\n=== {t} ===");
}
fn pass(r: &mut Vec<(&'static str, bool)>, name: &'static str, ok: bool) {
    println!("  -> {}: {}", name, if ok { "PASS" } else { "FAIL" });
    r.push((name, ok));
}

/// The cross-boundary mapping: public URI = group URI with the authority (DID)
/// swapped. A PURE FUNCTION when the rkey is pinned — no lookup table.
fn public_uri(group_uri: &str, group_did: &str, pds_did: &str) -> String {
    group_uri.replacen(group_did, pds_did, 1)
}

#[tokio::main]
async fn main() {
    let mut results: Vec<(&'static str, bool)> = Vec::new();

    let pds = pds::Pds::new();
    let l = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = l.local_addr().unwrap();
    let app = pds::router(pds.clone());
    tokio::spawn(async move { axum::serve(l, app).await.unwrap(); });
    let base = format!("http://{addr}");
    let client = reqwest::Client::new();

    let (pds_did, token) = publisher::create_session(&client, &base, "alice.test", "pw").await;
    // The group-side identity (did:key) the record was authored under privately.
    let group_did = "did:key:zalicegroupkey";

    // ------------------------------------------------------------------
    section("STEP 1: Pin the rkey at creation; publish with that exact rkey");
    // ------------------------------------------------------------------
    let rkey = record::new_tid(); // minted ONCE, at authoring time, in the group
    let group_uri = format!("at://{group_did}/{POST_NSID}/{rkey}");
    let post = Post::new("stable identity across the boundary", None);
    let post_val = serde_json::to_value(&post).unwrap();
    let (st, body) = publisher::create_record(&client, &base, &token, &pds_did, POST_NSID, &rkey, &post_val).await;
    let published_uri = body["uri"].as_str().unwrap_or("").to_string();
    let published_cid = body["cid"].as_str().unwrap_or("").to_string();
    println!("group rkey   = {rkey}");
    println!("group uri    = {group_uri}");
    println!("published uri= {published_uri}  ({st})");
    let rkey_preserved = published_uri.ends_with(&format!("/{POST_NSID}/{rkey}"));
    pass(&mut results, "1. PDS honored the caller's rkey (identity pinned)", st == StatusCode::OK && rkey_preserved);

    // ------------------------------------------------------------------
    section("STEP 2: Public URI is a PURE authority rewrite of the group URI");
    // ------------------------------------------------------------------
    let computed = public_uri(&group_uri, group_did, &pds_did);
    println!("pure-function mapping (swap DID): {computed}");
    println!("actual published uri:            {published_uri}");
    let pure_ok = computed == published_uri;
    println!("  match (no lookup table needed)? {pure_ok}");
    pass(&mut results, "2. Cross-boundary URI mapping is pure (authority swap)", pure_ok);

    // ------------------------------------------------------------------
    section("STEP 3: Re-publishing the identical record is idempotent");
    // ------------------------------------------------------------------
    let (st2, body2) = publisher::create_record(&client, &base, &token, &pds_did, POST_NSID, &rkey, &post_val).await;
    println!("  status={st2} idempotent={} uri={}", body2["idempotent"], body2["uri"]);
    let idem_ok = st2 == StatusCode::OK
        && body2["idempotent"] == json!(true)
        && body2["cid"] == json!(published_cid);
    pass(&mut results, "3. Idempotent re-create (same rkey+content): no duplicate", idem_ok);

    // ------------------------------------------------------------------
    section("STEP 4: A conflicting create (same rkey, different content) is rejected");
    // ------------------------------------------------------------------
    let edited = Post::new("edited body — different content", None);
    let edited_val = serde_json::to_value(&edited).unwrap();
    let (st3, body3) = publisher::create_record(&client, &base, &token, &pds_did, POST_NSID, &rkey, &edited_val).await;
    println!("  status={st3} body={body3}");
    pass(&mut results, "4. create at existing rkey w/ new content -> 409 (must putRecord)", st3 == StatusCode::CONFLICT);

    // ------------------------------------------------------------------
    section("STEP 5: Edit via putRecord — URI stable, CID changes");
    // ------------------------------------------------------------------
    let (st4, body4) = publisher::put_record(&client, &base, &token, &pds_did, POST_NSID, &rkey, &edited_val).await;
    let edited_uri = body4["uri"].as_str().unwrap_or("").to_string();
    let edited_cid = body4["cid"].as_str().unwrap_or("").to_string();
    let readback = publisher::get_record(&client, &base, &pds_did, POST_NSID, &rkey).await;
    println!("  putRecord status={st4}");
    println!("  uri before={published_uri}");
    println!("  uri after ={edited_uri}  (stable? {})", edited_uri == published_uri);
    println!("  cid before={published_cid}");
    println!("  cid after ={edited_cid}  (changed? {})", edited_cid != published_cid);
    println!("  readback value.text = {}", readback["value"]["text"]);
    let edit_ok = st4 == StatusCode::OK
        && edited_uri == published_uri
        && edited_cid != published_cid
        && readback["value"]["text"] == json!("edited body — different content");
    pass(&mut results, "5. putRecord edit: identity (URI) stable, content (CID) updated", edit_ok);

    // ------------------------------------------------------------------
    section("SUMMARY");
    // ------------------------------------------------------------------
    let mut all = true;
    for (name, ok) in &results {
        println!("[{}] {}", if *ok { "PASS" } else { "FAIL" }, name);
        all &= *ok;
    }
    println!("\nOVERALL: {}", if all { "PASS — record identity is stable across the boundary and across edits" } else { "FAIL" });

    section("ISSUES SURFACED / RESOLVED");
    println!("  RESOLVED: pinning the rkey at creation makes the public URI a pure authority");
    println!("    rewrite of the group URI — the Phase 6 lookup table for strongRef rewriting is");
    println!("    no longer needed; combined with the Phase 8 binding the swap is verifiable.");
    println!("  1. The committer must mint the rkey once (a TID at authoring time) and reuse it on");
    println!("     publish; two independent publishes of 'the same' record must agree on the rkey or");
    println!("     they create two records (no content-addressed dedup of rkeys).");
    println!("  2. Edits require putRecord (upsert); createRecord is create-only and 409s on conflict.");
    println!("     Clients must choose the right verb. Real PDSes also enforce per-collection rkey");
    println!("     rules (e.g. key: tid vs literal).");
    println!("  3. rkey collisions across authors are namespaced by DID (the repo), so no global");
    println!("     collision — but within one repo a TID clock must stay monotonic.");

    section("VERSION REPORT");
    println!("rustc {} | axum {} | tokio {} | reqwest {} | cid {} | serde_json {}",
        env!("SLICE_RUSTC_VERSION"), env!("SLICE_VER_AXUM"), env!("SLICE_VER_TOKIO"), env!("SLICE_VER_REQWEST"), env!("SLICE_VER_CID"), env!("SLICE_VER_SERDE_JSON"));

    if !all {
        std::process::exit(1);
    }
}
