//! Publish-loop validation (the deferred "publish phase").
//!
//! Authenticates to a real PDS, publishes records, then confirms each one arrives
//! back through our OWN Jetstream indexer — closing the full
//!   write -> PDS -> firehose -> Jetstream -> our AppView index
//! loop against the live public network.
//!
//! SECURITY: credentials come ONLY from environment variables
//! (BSKY_IDENTIFIER, BSKY_PASSWORD). They are never hardcoded, never printed,
//! never written to disk, and never committed. Use a throwaway/test account and
//! rotate the password afterward. The access JWT is held in memory only.
//!
//! Run:  BSKY_IDENTIFIER='you@example.com' BSKY_PASSWORD='app-password' \
//!         cargo run --bin publish

use std::cell::RefCell;
use std::time::Duration;

use anyhow::{anyhow, Context, Result};
use serde_json::{json, Value};

use appview_validation::index::{Index, IndexStats};
use appview_validation::jetstream::JetstreamSource;
use appview_validation::record_source::{ParseOutcome, RecordSource};

const PDS: &str = "https://bsky.social";

#[tokio::main]
async fn main() -> Result<()> {
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("install rustls ring crypto provider");

    let identifier =
        std::env::var("BSKY_IDENTIFIER").context("set BSKY_IDENTIFIER (handle or email)")?;
    let password =
        std::env::var("BSKY_PASSWORD").context("set BSKY_PASSWORD (app password recommended)")?;

    let http = reqwest::Client::builder()
        .user_agent("appview-validation-experiment/0.1")
        .build()?;

    // ───────────────── STEP P1: createSession ─────────────────
    println!("\n############ STEP P1: com.atproto.server.createSession ############");
    let sess: Value = http
        .post(format!("{PDS}/xrpc/com.atproto.server.createSession"))
        .json(&json!({ "identifier": identifier, "password": password }))
        .send()
        .await?
        .error_for_status()
        .context("createSession failed (check identifier / password)")?
        .json()
        .await?;
    let did = sess["did"]
        .as_str()
        .ok_or_else(|| anyhow!("no did in session response"))?
        .to_string();
    let handle = sess["handle"].as_str().unwrap_or("<unknown>").to_string();
    let jwt = sess["accessJwt"]
        .as_str()
        .ok_or_else(|| anyhow!("no accessJwt in session response"))?
        .to_string();
    println!("  authenticated as @{handle}");
    println!("  did = {did}");
    println!("  accessJwt acquired ({} chars; value not printed)", jwt.len());

    let mut index = Index::open("publish.sqlite")?;
    let mut stats = IndexStats::default();

    // ───────────────── STEP P2/P3: post -> firehose -> index ─────────────────
    println!("\n############ STEP P2: createRecord (app.bsky.feed.post) ############");
    let now = chrono::Utc::now();
    let marker = format!("appview-validation loop test {}", now.timestamp_micros());
    let post_uri = create_record(
        &http,
        &jwt,
        &did,
        "app.bsky.feed.post",
        json!({
            "$type": "app.bsky.feed.post",
            "text": marker,
            "createdAt": now.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            "langs": ["en"],
        }),
    )
    .await?;
    let post_rkey = rkey_of(&post_uri);
    println!("  published: {post_uri}");
    println!("  marker   : {marker}");

    println!("\n############ STEP P3: confirm post round-trips via Jetstream ############");
    let found = watch_for_rkey(
        &did,
        "app.bsky.feed.post",
        &post_rkey,
        now.timestamp_micros(),
        &mut index,
        &mut stats,
    )
    .await?;
    if found {
        println!("  LOOP CLOSED — the post we just wrote was observed on the live");
        println!("  firehose and indexed by our own AppView.");
    } else {
        println!("  WARNING: post not observed within the watch window.");
    }

    // ───────────────── STEP P4 (stretch): custom lexicon ─────────────────
    println!("\n############ STEP P4 (stretch): custom-lexicon record on the network ############");
    let custom_nsid = "org.owasp.validation.note";
    let now2 = chrono::Utc::now();
    let custom_uri = create_record(
        &http,
        &jwt,
        &did,
        custom_nsid,
        json!({
            "$type": custom_nsid,
            "note": "custom lexicon flow test from the appview-validation experiment",
            "createdAt": now2.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            "n": 2112,
        }),
    )
    .await?;
    let custom_rkey = rkey_of(&custom_uri);
    println!("  published custom record: {custom_uri}");
    let found_custom = watch_for_rkey(
        &did,
        custom_nsid,
        &custom_rkey,
        now2.timestamp_micros(),
        &mut index,
        &mut stats,
    )
    .await?;
    if found_custom {
        println!("  CUSTOM LEXICON FLOWED — Jetstream is collection-agnostic; a custom");
        println!("  NSID propagates on the firehose with no pre-registration.");
    } else {
        println!("  WARNING: custom record not observed within the watch window.");
    }

    println!(
        "\n  index now holds {} of our own published rows.",
        index.row_count()?
    );

    // ───────────────── STEP P5: cleanup ─────────────────
    println!("\n############ STEP P5: cleanup (deleteRecord) ############");
    delete_record(&http, &jwt, &did, "app.bsky.feed.post", &post_rkey).await?;
    delete_record(&http, &jwt, &did, custom_nsid, &custom_rkey).await?;
    println!("  deleted both published records from the test account.");

    println!("\n############ DONE (publish loop) ############");
    Ok(())
}

async fn create_record(
    http: &reqwest::Client,
    jwt: &str,
    did: &str,
    collection: &str,
    record: Value,
) -> Result<String> {
    let resp: Value = http
        .post(format!("{PDS}/xrpc/com.atproto.repo.createRecord"))
        .bearer_auth(jwt)
        .json(&json!({ "repo": did, "collection": collection, "record": record }))
        .send()
        .await?
        .error_for_status()
        .context("createRecord failed")?
        .json()
        .await?;
    resp["uri"]
        .as_str()
        .map(str::to_string)
        .ok_or_else(|| anyhow!("no uri in createRecord response: {resp}"))
}

async fn delete_record(
    http: &reqwest::Client,
    jwt: &str,
    did: &str,
    collection: &str,
    rkey: &str,
) -> Result<()> {
    http.post(format!("{PDS}/xrpc/com.atproto.repo.deleteRecord"))
        .bearer_auth(jwt)
        .json(&json!({ "repo": did, "collection": collection, "rkey": rkey }))
        .send()
        .await?
        .error_for_status()
        .context("deleteRecord failed")?;
    Ok(())
}

fn rkey_of(at_uri: &str) -> String {
    at_uri.rsplit('/').next().unwrap_or("").to_string()
}

/// Subscribe to Jetstream filtered to our DID + collection, replaying from just
/// before we published (so we cannot miss our own event), and return true as soon
/// as the given rkey is observed (early-stopping the source).
async fn watch_for_rkey(
    did: &str,
    collection: &str,
    rkey: &str,
    published_us: i64,
    index: &mut Index,
    stats: &mut IndexStats,
) -> Result<bool> {
    // Replay from ~10s before publish to cover propagation + connect latency.
    let cursor = published_us - 10_000_000;
    let mut src = JetstreamSource::new(&[collection], 10_000, Duration::from_secs(30))
        .with_did(did)
        .from_cursor(cursor);
    println!("    watching {} for rkey {rkey} (cursor replay)…", src.url());

    let found = RefCell::new(false);
    src.run(|outcome| {
        if let ParseOutcome::Commit { event, .. } = outcome {
            let _ = index.apply(&event, stats);
            if event.rkey == rkey {
                *found.borrow_mut() = true;
                return false; // early stop — we saw our own record
            }
        }
        true
    })
    .await?;
    Ok(found.into_inner())
}
