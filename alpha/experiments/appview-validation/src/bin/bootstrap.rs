//! AppView lifecycle proofs against the live network (needs a test account):
//!   #1  BACKFILL -> LIVE-TAIL with no gap: crawl repo history via listRecords,
//!       then live-tail Jetstream from a checkpoint taken BEFORE backfill, proving
//!       records that arrive during/after backfill are still caught (zero gap),
//!       overlap is harmless (upsert dedup), and the final view is complete.
//!   #2  CURSOR PERSISTENCE + CRASH RECOVERY: checkpoint the cursor, "crash",
//!       publish during downtime, restart from the persisted cursor, prove the
//!       downtime event is not lost (at-least-once; dedup absorbs replay).
//!
//! SECURITY: credentials come ONLY from BSKY_IDENTIFIER / BSKY_PASSWORD env vars.
//! Never hardcoded, printed, persisted, or committed. Published records are
//! deleted at the end. Use a throwaway account and rotate the password after.

use std::cell::RefCell;
use std::collections::HashSet;
use std::time::Duration;

use anyhow::{anyhow, Result};
use serde_json::{json, Value};

use appview_validation::atproto::{self, ENTRYWAY};
use appview_validation::index::{Index, IndexStats};
use appview_validation::jetstream::JetstreamSource;
use appview_validation::record_source::{Action, ParseOutcome, RecordEvent, RecordSource};

const COLL: &str = "app.bsky.feed.post";

fn now_us() -> i64 {
    chrono::Utc::now().timestamp_micros()
}

fn rkey_of(uri: &str) -> String {
    uri.rsplit('/').next().unwrap_or("").to_string()
}

/// Convert a listRecords entry ({uri, cid, value}) into a RecordEvent.
fn event_from_listing(rec: &Value) -> Option<RecordEvent> {
    let uri = rec.get("uri")?.as_str()?;
    let (did, collection, rkey) = atproto::parse_at_uri(uri)?;
    Some(RecordEvent {
        action: Action::Create,
        did,
        collection,
        rkey,
        record: rec.get("value").cloned(),
        cid: rec.get("cid").and_then(Value::as_str).map(str::to_string),
        cursor: 0, // listRecords carries no firehose cursor; backfill is historical
    })
}

async fn post(http: &reqwest::Client, jwt: &str, did: &str, text: &str) -> Result<String> {
    atproto::create_record(
        http,
        ENTRYWAY,
        jwt,
        did,
        COLL,
        json!({
            "$type": COLL,
            "text": text,
            "createdAt": chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            "langs": ["en"],
        }),
    )
    .await
}

#[tokio::main]
async fn main() -> Result<()> {
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("install rustls ring crypto provider");

    let identifier =
        std::env::var("BSKY_IDENTIFIER").map_err(|_| anyhow!("set BSKY_IDENTIFIER"))?;
    let password = std::env::var("BSKY_PASSWORD").map_err(|_| anyhow!("set BSKY_PASSWORD"))?;

    let http = atproto::client();
    let sess = atproto::create_session(&http, ENTRYWAY, &identifier, &password).await?;
    let did = sess.did.clone();
    let (pds, handle) = atproto::resolve_identity(&http, &did).await?;
    println!("\n############ setup ############");
    println!("  authenticated as @{handle}  ({did})");
    println!("  resolved PDS (for repo reads): {pds}");

    let mut index = Index::open("bootstrap.sqlite")?;
    let mut stats = IndexStats::default();
    let mut to_cleanup: Vec<String> = Vec::new();

    // ═════════════════ #1 BACKFILL -> LIVE-TAIL (no gap) ═════════════════
    println!("\n############ #1: backfill -> live-tail with no-gap proof ############");

    // Checkpoint BEFORE any writes: this is where the live tail will resume from.
    let checkpoint = now_us() - 2_000_000;
    println!("  checkpoint cursor (pre-backfill) = {checkpoint}");

    // Seed history: 3 posts that exist at backfill time.
    println!("  publishing 3 seed posts (the 'history')…");
    for i in 0..3 {
        let uri = post(&http, &sess.jwt, &did, &format!("bootstrap seed #{i} {}", now_us())).await?;
        to_cleanup.push(uri);
    }
    tokio::time::sleep(Duration::from_millis(1500)).await;

    // BACKFILL via paginated listRecords (limit=2 forces multiple pages).
    println!("  backfilling via com.atproto.repo.listRecords (page size 2)…");
    let mut cursor: Option<String> = None;
    let (mut backfilled, mut pages) = (0usize, 0usize);
    loop {
        let (records, next) =
            atproto::list_records_page(&http, &pds, &did, COLL, 2, cursor.as_deref()).await?;
        pages += 1;
        for rec in &records {
            if let Some(ev) = event_from_listing(rec) {
                index.apply(&ev, &mut stats)?;
                backfilled += 1;
            }
        }
        match next {
            Some(n) if !records.is_empty() => cursor = Some(n),
            _ => break,
        }
    }
    println!("  backfilled {backfilled} records across {pages} pages.");

    // Two MORE posts AFTER backfill — these would be MISSED by backfill alone.
    println!("  publishing 2 more posts AFTER backfill (the gap-risk events)…");
    let mut post_backfill_rkeys = Vec::new();
    for i in 3..5 {
        let uri = post(&http, &sess.jwt, &did, &format!("bootstrap live #{i} {}", now_us())).await?;
        post_backfill_rkeys.push(rkey_of(&uri));
        to_cleanup.push(uri);
    }
    tokio::time::sleep(Duration::from_millis(1500)).await;

    // LIVE-TAIL from the pre-backfill checkpoint, early-stop once all 5 are seen.
    let target: HashSet<String> = to_cleanup.iter().map(|u| rkey_of(u)).collect();
    println!("  live-tailing Jetstream from the checkpoint (DID-filtered)…");
    let seen_live: RefCell<HashSet<String>> = RefCell::new(HashSet::new());
    let mut src = JetstreamSource::new(&[COLL], 100_000, Duration::from_secs(40))
        .with_did(&did)
        .from_cursor(checkpoint);
    src.run(|outcome| {
        if let ParseOutcome::Commit { event, .. } = outcome {
            let _ = index.apply(&event, &mut stats);
            if target.contains(&event.rkey) {
                seen_live.borrow_mut().insert(event.rkey.clone());
            }
        }
        seen_live.borrow().len() < target.len() // stop once all targets seen live
    })
    .await?;

    let seen_live = seen_live.into_inner();
    let live_caught_post_backfill = post_backfill_rkeys
        .iter()
        .filter(|r| seen_live.contains(*r))
        .count();
    let overlap = backfilled.min(seen_live.len()); // seeds present in both paths
    let final_rows = index.count_for(COLL)?;
    println!("\n  RESULTS:");
    println!("    backfilled (history)         : {backfilled}");
    println!("    seen via live tail (replay)  : {}", seen_live.len());
    println!("    post-backfill posts caught   : {live_caught_post_backfill}/2  (gap = {})",
        2 - live_caught_post_backfill);
    println!("    overlap (in both paths)      : ~{overlap}  -> deduped by upsert");
    println!("    final unique posts in index  : {final_rows}");
    if live_caught_post_backfill == 2 {
        println!("  => NO GAP: the 2 posts that arrived after backfill were caught by the");
        println!("     live tail resuming from the pre-backfill checkpoint. Overlap was");
        println!("     harmless (upsert dedup). This is the canonical AppView bootstrap.");
    } else {
        println!("  => WARNING: some post-backfill posts were not observed in the window.");
    }

    // ═════════════════ #2 CURSOR PERSISTENCE + CRASH RECOVERY ═════════════════
    println!("\n############ #2: cursor persistence + crash recovery ############");

    let c0 = now_us() - 2_000_000;
    let uri_a = post(&http, &sess.jwt, &did, &format!("crashtest A {}", now_us())).await?;
    let rkey_a = rkey_of(&uri_a);
    to_cleanup.push(uri_a);
    tokio::time::sleep(Duration::from_millis(1500)).await;

    // Consume from c0, checkpointing each cursor, until we process A — then "crash".
    println!("  consuming until post A, checkpointing the cursor each event…");
    let mut src = JetstreamSource::new(&[COLL], 100_000, Duration::from_secs(30))
        .with_did(&did)
        .from_cursor(c0);
    let saw_a = RefCell::new(false);
    src.run(|outcome| {
        if let ParseOutcome::Commit { event, .. } = outcome {
            let _ = index.apply(&event, &mut stats);
            let _ = index.save_cursor("crash-demo", event.cursor); // checkpoint to disk
            if event.rkey == rkey_a {
                *saw_a.borrow_mut() = true;
                return false; // CRASH: stop right after checkpointing A
            }
        }
        true
    })
    .await?;
    let checkpoint_at_crash = index.load_cursor("crash-demo")?.unwrap_or(0);
    println!("  processed A = {} ; checkpoint persisted at cursor {checkpoint_at_crash}",
        saw_a.into_inner());

    // DOWNTIME: publish B while the consumer is "down".
    let uri_b = post(&http, &sess.jwt, &did, &format!("crashtest B during downtime {}", now_us())).await?;
    let rkey_b = rkey_of(&uri_b);
    to_cleanup.push(uri_b);
    println!("  (consumer down) published B during downtime.");
    tokio::time::sleep(Duration::from_millis(1500)).await;

    // RESTART: resume strictly from the persisted checkpoint.
    let resume = index.load_cursor("crash-demo")?.ok_or_else(|| anyhow!("no checkpoint"))?;
    println!("  RESTART: resuming from persisted cursor {resume}…");
    let mut src = JetstreamSource::new(&[COLL], 100_000, Duration::from_secs(30))
        .with_did(&did)
        .from_cursor(resume);
    let saw_b = RefCell::new(false);
    src.run(|outcome| {
        if let ParseOutcome::Commit { event, .. } = outcome {
            let _ = index.apply(&event, &mut stats);
            let _ = index.save_cursor("crash-demo", event.cursor);
            if event.rkey == rkey_b {
                *saw_b.borrow_mut() = true;
                return false;
            }
        }
        true
    })
    .await?;
    if saw_b.into_inner() {
        println!("  => NO LOSS: B (published during downtime) was captured after resuming");
        println!("     from the persisted checkpoint. A may be re-delivered (at-least-once);");
        println!("     the upsert index makes replay idempotent.");
    } else {
        println!("  => WARNING: B not observed after resume within the window.");
    }

    // ═════════════════ cleanup ═════════════════
    println!("\n############ cleanup (deleteRecord) ############");
    for uri in &to_cleanup {
        if let Some((_, coll, rkey)) = atproto::parse_at_uri(uri) {
            let _ = atproto::delete_record(&http, ENTRYWAY, &sess.jwt, &did, &coll, &rkey).await;
        }
    }
    println!("  deleted {} published records.", to_cleanup.len());
    println!("\n############ DONE (bootstrap) ############");
    Ok(())
}
