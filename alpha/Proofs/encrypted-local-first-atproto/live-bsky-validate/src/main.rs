//! Live validation against real Bluesky test accounts.
//!
//! Validates, against the REAL atproto network, the claims the local phases
//! proved on a stand-in PDS:
//!   * a real PDS accepts our custom-lexicon records via createRecord;
//!   * the PDS-assigned CID matches our locally-computed CIDv1 (CID parity);
//!   * the record reads back and re-validates against our lexicon;
//!   * (optional) the record appears on the real Jetstream firehose;
//!   * cleanup deletes the records afterward (the firehose already saw them).
//!
//! Identity binding (Phase 8/11) is NOT validated here: it needs the DID's
//! verification key, which bsky-hosted accounts don't expose to an app password.
//!
//! Requires (set as env vars; do NOT paste secrets in chat):
//!   ATP_IDENTIFIER, ATP_APP_PASSWORD            (account 1 — author)
//!   ATP_IDENTIFIER_2, ATP_APP_PASSWORD_2        (account 2 — reactor)
//!   PDS_HOST          (optional, default https://bsky.social)
//!   JETSTREAM_HOST    (optional, default wss://jetstream2.us-east.bsky.network; "off" to skip)
//!   KEEP_RECORDS=1    (optional, skip cleanup)
//! And the environment's egress allowlist must include bsky.social,
//! plc.directory, and the Jetstream host.

#[allow(dead_code)]
mod content_id;
#[allow(dead_code)]
mod jetstream;
#[allow(dead_code)]
mod lexicon;
mod client;
#[allow(dead_code)]
mod record;

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use futures_util::StreamExt;
use jetstream::JetstreamEvent;
use lexicon::Lexicon;
use record::{Post, Reaction, StrongRef, POST_NSID, REACTION_NSID};
use reqwest::StatusCode;
use serde_json::Value;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

fn section(t: &str) {
    println!("\n=== {t} ===");
}
fn pass(r: &mut Vec<(&'static str, bool)>, name: &'static str, ok: bool) {
    println!("  -> {}: {}", name, if ok { "PASS" } else { "FAIL" });
    r.push((name, ok));
}
fn env(k: &str) -> Option<String> {
    std::env::var(k).ok().filter(|v| !v.is_empty())
}

#[tokio::main]
async fn main() {
    let base = env("PDS_HOST").unwrap_or_else(|| "https://bsky.social".into());
    let jetstream = env("JETSTREAM_HOST").unwrap_or_else(|| "wss://jetstream2.us-east.bsky.network".into());

    let (id1, pw1, id2, pw2) = match (env("ATP_IDENTIFIER"), env("ATP_APP_PASSWORD"), env("ATP_IDENTIFIER_2"), env("ATP_APP_PASSWORD_2")) {
        (Some(a), Some(b), Some(c), Some(d)) => (a, b, c, d),
        _ => {
            println!("live-bsky-validate is READY but needs configuration:");
            println!("  1. Allowlist egress: bsky.social, plc.directory, {} host", jetstream);
            println!("  2. Set env vars (do not paste secrets in chat):");
            println!("       ATP_IDENTIFIER / ATP_APP_PASSWORD       (author account)");
            println!("       ATP_IDENTIFIER_2 / ATP_APP_PASSWORD_2   (reactor account)");
            println!("  Optional: PDS_HOST (default https://bsky.social), JETSTREAM_HOST, KEEP_RECORDS=1");
            println!("\nThen: cargo run -p live-bsky-validate");
            return;
        }
    };

    let client = reqwest::Client::new();
    let mut results: Vec<(&'static str, bool)> = Vec::new();

    // ------------------------------------------------------------------
    section("STEP 1: createSession for both accounts (real app-password auth)");
    // ------------------------------------------------------------------
    let author = match client::create_session(&client, &base, &id1, &pw1).await {
        Ok(s) => s,
        Err(e) => { println!("FAILED to authenticate author: {e}"); std::process::exit(1); }
    };
    let reactor = match client::create_session(&client, &base, &id2, &pw2).await {
        Ok(s) => s,
        Err(e) => { println!("FAILED to authenticate reactor: {e}"); std::process::exit(1); }
    };
    println!("author : {} -> {}", author.handle, author.did);
    println!("reactor: {} -> {}", reactor.handle, reactor.did);
    pass(&mut results, "1. Both test accounts authenticated against the live PDS", !author.did.is_empty() && !reactor.did.is_empty());

    let post_lex = Lexicon::load(lexicon::POST_LEXICON);

    // Jetstream backfill cursor: a few seconds before we publish.
    let cursor_us = (SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros() as u64).saturating_sub(10_000_000);

    // ------------------------------------------------------------------
    section("STEP 2: Author publishes a custom-lexicon post to the real PDS");
    // ------------------------------------------------------------------
    let post_rkey = record::new_tid();
    let post = Post::new("hello from an encrypted-local-first experiment 👋 (test record)", Some(vec!["en".into()]));
    let post_val = serde_json::to_value(&post).unwrap();
    let local_cid = content_id::record_cid(&post_val);
    let (st, body) = client::create_record(&client, &base, &author.access_jwt, &author.did, POST_NSID, &post_rkey, &post_val).await;
    println!("createRecord status = {st}");
    println!("response = {}", serde_json::to_string_pretty(&body).unwrap());
    let pds_uri = body["uri"].as_str().unwrap_or_default().to_string();
    let pds_cid = body["cid"].as_str().unwrap_or_default().to_string();
    let accepted = st == StatusCode::OK;
    pass(&mut results, "2. Live PDS accepted a custom-NSID lexicon record", accepted);

    // ------------------------------------------------------------------
    section("STEP 3: CID parity — PDS-assigned CID vs. locally-computed CIDv1");
    // ------------------------------------------------------------------
    println!("local  CID = {local_cid}");
    println!("PDS    CID = {pds_cid}");
    let parity = accepted && !pds_cid.is_empty() && pds_cid == local_cid;
    if accepted && !parity {
        println!("  NOTE: mismatch is itself a finding — likely DAG-CBOR canonicalization differences");
        println!("  (field ordering, integer/float encoding) between our encoder and the PDS.");
    }
    pass(&mut results, "3. CID parity with the real PDS", parity);

    // ------------------------------------------------------------------
    section("STEP 4: Reactor publishes a reaction (strongRef -> the post)");
    // ------------------------------------------------------------------
    let mut reaction_ok = false;
    let reaction_rkey = record::new_tid();
    if accepted {
        let subject = StrongRef { uri: pds_uri.clone(), cid: pds_cid.clone() };
        let reaction = Reaction::new(subject, "🎉");
        let reaction_val = serde_json::to_value(&reaction).unwrap();
        let (rst, rbody) = client::create_record(&client, &base, &reactor.access_jwt, &reactor.did, REACTION_NSID, &reaction_rkey, &reaction_val).await;
        println!("reaction createRecord status = {rst}");
        println!("reaction uri = {}", rbody["uri"].as_str().unwrap_or("-"));
        reaction_ok = rst == StatusCode::OK;
    } else {
        println!("  skipped (post was not accepted)");
    }
    pass(&mut results, "4. Reactor's strongRef reaction accepted by the live PDS", reaction_ok);

    // ------------------------------------------------------------------
    section("STEP 5: Read the post back from the PDS + re-validate against lexicon");
    // ------------------------------------------------------------------
    let mut readback_ok = false;
    if accepted {
        let (gst, gbody) = client::get_record(&client, &base, &author.did, POST_NSID, &post_rkey).await;
        println!("getRecord status = {gst}");
        let value = gbody.get("value").cloned().unwrap_or(Value::Null);
        let valid = post_lex.validate(&value).is_ok();
        let text_ok = value["text"] == post_val["text"];
        println!("re-validates against our lexicon? {valid}; text matches? {text_ok}");
        readback_ok = gst == StatusCode::OK && valid && text_ok;
    }
    pass(&mut results, "5. Round-trip readback re-validates against our lexicon", readback_ok);

    // ------------------------------------------------------------------
    section("STEP 6: (optional) Observe our records on the real Jetstream firehose");
    // ------------------------------------------------------------------
    if jetstream.eq_ignore_ascii_case("off") {
        println!("  skipped (JETSTREAM_HOST=off)");
    } else if accepted {
        let collections = vec![POST_NSID.to_string(), REACTION_NSID.to_string()];
        let dids = vec![author.did.clone(), reactor.did.clone()];
        let want = if reaction_ok { vec![post_rkey.clone(), reaction_rkey.clone()] } else { vec![post_rkey.clone()] };
        let found = observe_firehose(&jetstream, &collections, &dids, &want, cursor_us, Duration::from_secs(20)).await;
        println!("  observed {}/{} of our records on the firehose", found.len(), want.len());
        // best-effort: recorded, but does not gate the overall result
        results.push(("6. Records observed on the real Jetstream firehose (best-effort)", !found.is_empty()));
    }

    // ------------------------------------------------------------------
    section("CLEANUP");
    // ------------------------------------------------------------------
    if env("KEEP_RECORDS").is_none() {
        if accepted {
            let s = client::delete_record(&client, &base, &author.access_jwt, &author.did, POST_NSID, &post_rkey).await;
            println!("  deleted author post: {s}");
        }
        if reaction_ok {
            let s = client::delete_record(&client, &base, &reactor.access_jwt, &reactor.did, REACTION_NSID, &reaction_rkey).await;
            println!("  deleted reactor reaction: {s}");
        }
        println!("  (note: the firehose already emitted these; deletion tidies the repos, not the relay history)");
    } else {
        println!("  KEEP_RECORDS set; leaving records in place");
    }

    // ------------------------------------------------------------------
    section("SUMMARY");
    // ------------------------------------------------------------------
    let mut all = true;
    for (name, ok) in &results {
        println!("[{}] {}", if *ok { "PASS" } else { "FAIL" }, name);
        all &= *ok;
    }
    println!("\nOVERALL: {}", if all { "PASS — the data model interoperates with the live atproto network" } else { "see failures above" });

    section("VERSION REPORT");
    println!("rustc {} | reqwest {} | tokio-tungstenite {} | cid {} | serde_ipld_dagcbor {}",
        env!("SLICE_RUSTC_VERSION"), env!("SLICE_VER_REQWEST"), env!("SLICE_VER_TUNGSTENITE"), env!("SLICE_VER_CID"), env!("SLICE_VER_DAGCBOR"));

    if !all {
        std::process::exit(1);
    }
}

/// Connect to a real Jetstream instance (filtered to our collections + DIDs,
/// backfilled from `cursor_us`) and watch for our published rkeys. Best-effort.
async fn observe_firehose(
    host: &str,
    collections: &[String],
    dids: &[String],
    want_rkeys: &[String],
    cursor_us: u64,
    timeout: Duration,
) -> Vec<String> {
    let mut url = format!("{host}/subscribe?cursor={cursor_us}");
    for c in collections {
        url.push_str(&format!("&wantedCollections={c}"));
    }
    for d in dids {
        url.push_str(&format!("&wantedDids={d}"));
    }
    let (mut ws, _) = match connect_async(&url).await {
        Ok(x) => x,
        Err(e) => {
            println!("  firehose connect failed: {e}");
            return Vec::new();
        }
    };
    let mut found: Vec<String> = Vec::new();
    let deadline = tokio::time::Instant::now() + timeout;
    while found.len() < want_rkeys.len() {
        let remaining = deadline.saturating_duration_since(tokio::time::Instant::now());
        if remaining.is_zero() {
            break;
        }
        match tokio::time::timeout(remaining, ws.next()).await {
            Ok(Some(Ok(Message::Text(t)))) => {
                if let Ok(ev) = serde_json::from_str::<JetstreamEvent>(&t) {
                    if let Some(c) = ev.commit {
                        if want_rkeys.contains(&c.rkey) && !found.contains(&c.rkey) {
                            println!("  firehose saw {}/{}/{}", ev.did, c.collection, c.rkey);
                            found.push(c.rkey);
                        }
                    }
                }
            }
            Ok(Some(Ok(_))) => {}
            _ => break,
        }
    }
    found
}
