//! Standalone experiment: a minimal AT Protocol AppView, validated against the
//! REAL public atproto network via Jetstream.
//!
//! Pipeline: ingest (real Jetstream WS) -> index (disposable SQLite) -> serve
//! (XRPC-shaped HTTP queries). The primary deliverable is LEARNING: the
//! field-by-field comparison of my predicted event shape against reality, the
//! observed event rate, and an honest friction log. See README.md.

use std::time::Duration;

use serde_json::Value;

use appview_validation::index::{Index, IndexStats};
use appview_validation::jetstream::{self, JetstreamSource};
use appview_validation::record_source::{Action, ParseOutcome, RecordEvent, RecordSource};
use appview_validation::report::{self, Findings};
use appview_validation::server;

const DB_PATH: &str = "appview.sqlite";

/// Resolved versions of the load-bearing crates (full pin lives in Cargo.lock).
/// Printed so the run is self-describing — the atproto Rust ecosystem moves fast.
const VERSION_BANNER: &str = "\
  rustc            1.94.1
  edition          2024
  tokio            1.52.x
  tokio-tungstenite 0.29.x  (rustls-tls-native-roots) -- WS transport / ingest
  rustls           0.23.x   (ring provider, installed at startup)
  rusqlite         0.32.x   (bundled SQLite 0.30.x via libsqlite3-sys)
  axum             0.8.x    -- XRPC HTTP serving
  serde_json       1.0.x
  (no dedicated Jetstream crate: events parsed from raw JSON on purpose; see README)";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // FRICTION (logged): rustls 0.23 refuses to pick a crypto provider when the
    // feature flags don't unambiguously select one (tokio-tungstenite is built
    // with default-features off here). Must install one explicitly before any TLS.
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("install rustls ring crypto provider");

    println!("\n############ STEP 1: versions + connectivity ############");
    println!("{VERSION_BANNER}");
    println!(
        "  Jetstream host   {} (verified reachable in preflight; \
         handshake confirmed below)",
        jetstream::DEFAULT_HOST
    );

    let mut index = Index::open(DB_PATH)?;
    let mut findings = Findings::default();
    let mut stats = IndexStats::default();

    // ───────────────────── STEP 2: live ingest of real posts ─────────────────
    println!("\n############ STEP 2: live ingest — app.bsky.feed.post ############");
    let mut first_sample: Option<(Value, RecordEvent)> = None;

    let mut posts = JetstreamSource::new(
        &["app.bsky.feed.post"],
        200,
        Duration::from_secs(60),
    );
    posts
        .run(|outcome| {
            findings.frames += 1;
            match outcome {
                ParseOutcome::Commit { event, raw } => {
                    findings.commits += 1;
                    match event.action {
                        Action::Create => findings.creates_seen += 1,
                        Action::Update => findings.updates_seen += 1,
                        Action::Delete => findings.deletes_seen += 1,
                    }
                    if first_sample.is_none() {
                        dump_first_event(&raw, &event);
                        first_sample = Some((raw, event.clone()));
                    }
                    // Lenient: a single bad record must not abort the run.
                    if let Err(e) = index.apply(&event, &mut stats) {
                        println!("    [index skipped {} : {e}]", event.at_uri());
                    }
                }
                ParseOutcome::NonCommit { kind, .. } => {
                    *findings.non_commit_kinds.entry(kind).or_default() += 1;
                }
                ParseOutcome::Malformed { error, raw_text } => {
                    findings.malformed += 1;
                    if findings.malformed <= 5 {
                        println!("    [malformed frame: {error} | raw: {raw_text}]");
                    }
                }
            }
            true // run the full window
        })
        .await?;

    // Headline reality check, using the first real commit event.
    if let Some((raw, ev)) = &first_sample {
        report::print_shape_report(raw, ev);
    } else {
        println!("\n  (no commit events captured — cannot run shape report)");
    }

    // ───────────────────── STEP 3: index stats ──────────────────────────────
    println!("\n############ STEP 3: index (disposable SQLite projection) ############");
    println!("  rows indexed (posts) : {}", index.count_for("app.bsky.feed.post")?);
    println!(
        "  create/update/delete : {}/{}/{}  (deletes that removed a row: {})",
        stats.created, stats.updated, stats.deletes_seen, stats.deleted_rows
    );
    if stats.deletes_seen == 0 {
        println!(
            "  note: no delete observed in this window — delete handling is wired \
             (DELETE WHERE uri) but unexercised this run."
        );
    }

    // ───────────────────── STEP 3b: short likes capture (for hydration) ─────
    println!("\n############ STEP 3b: short capture — app.bsky.feed.like ############");
    let mut likes = JetstreamSource::new(
        &["app.bsky.feed.like"],
        300,
        Duration::from_secs(20),
    );
    likes
        .run(|outcome| {
            findings.frames += 1;
            match outcome {
                ParseOutcome::Commit { event, .. } => {
                    findings.commits += 1;
                    match event.action {
                        Action::Create => findings.creates_seen += 1,
                        Action::Update => findings.updates_seen += 1,
                        Action::Delete => findings.deletes_seen += 1,
                    }
                    let _ = index.apply(&event, &mut stats);
                }
                ParseOutcome::NonCommit { kind, .. } => {
                    *findings.non_commit_kinds.entry(kind).or_default() += 1;
                }
                ParseOutcome::Malformed { .. } => findings.malformed += 1,
            }
            true // run the full window
        })
        .await?;
    println!("  rows indexed (likes) : {}", index.count_for("app.bsky.feed.like")?);
    let total_rows = index.row_count()?;
    println!("  total rows in index  : {total_rows}");

    report::print_findings(&findings);

    // Release the writer connection before the server opens read connections.
    drop(index);

    // ───────────────────── STEP 4 + 5: serve & query ────────────────────────
    println!("\n############ STEP 4: serve XRPC over the index (axum) ############");
    let app = server::router(DB_PATH)?;
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
    let addr = listener.local_addr()?;
    println!("  serving on http://{addr}/xrpc/...");
    tokio::spawn(async move {
        let _ = axum::serve(listener, app).await;
    });
    // Tiny grace period for the server task to come up.
    tokio::time::sleep(Duration::from_millis(150)).await;

    let addr = addr.to_string();

    println!("\n############ STEP 5: real HTTP GETs against the running server ############");
    let path = "/xrpc/com.example.getRecentPosts?limit=5";
    println!("  GET {path}");
    print_json(&server::http_get(&addr, path).await?);

    // ───────────────────── STEP 6: hydration proof ──────────────────────────
    println!("\n############ STEP 6: hydration proof (aggregation across records) ############");
    let path = "/xrpc/com.example.getLikeCountsBySubject?limit=5";
    println!("  GET {path}");
    println!("  (each row joins MANY like records into one view row — not an echo");
    println!("   of any single stored record; that is hydration.)");
    print_json(&server::http_get(&addr, path).await?);

    // ───────────────────── STEP 7: rebuild / disposability ──────────────────
    println!("\n############ STEP 7: disposable-projection proof ############");
    println!(
        "  The index file `{DB_PATH}` is recreated from scratch on every run \
         (Index::open deletes it first)."
    );
    println!(
        "  It currently holds {total_rows} rows projected from the live network. \
         Authoritative copies live in each author's PDS; deleting `{DB_PATH}` and \
         re-running ingest fully rebuilds the view."
    );

    println!("\n############ DONE ############");
    Ok(())
}

/// Dump the first real commit event in full raw form, beside its normalized shape.
fn dump_first_event(raw: &Value, ev: &RecordEvent) {
    println!("\n  ---- FIRST REAL COMMIT EVENT (raw, full) ----");
    println!(
        "{}",
        serde_json::to_string_pretty(raw).unwrap_or_else(|_| raw.to_string())
    );
    println!("\n  ---- normalized RecordEvent (my model) ----");
    println!("    action     : {}", ev.action.as_str());
    println!("    did        : {}", ev.did);
    println!("    collection : {}", ev.collection);
    println!("    rkey       : {}", ev.rkey);
    println!("    cid        : {:?}", ev.cid);
    println!("    cursor     : {} (time_us)", ev.cursor);
    println!("    at_uri     : {}", ev.at_uri());
    println!(
        "    record     : {} bytes of JSON (stored whole)",
        ev.record.as_ref().map(|r| r.to_string().len()).unwrap_or(0)
    );
}

fn print_json(body: &str) {
    match serde_json::from_str::<Value>(body) {
        Ok(v) => println!("{}", serde_json::to_string_pretty(&v).unwrap()),
        Err(_) => println!("{body}"),
    }
}
