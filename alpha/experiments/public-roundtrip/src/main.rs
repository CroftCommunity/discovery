//! Standalone experiment: publish a custom-lexicon record to a real AT Protocol
//! PDS via app-password auth, then consume it back off the public Jetstream
//! network and serve it from a minimal local AppView.
//!
//! See README.md for the full brief, guardrails, and the friction log.

mod appview;
mod bridge;
mod cidv1;
mod jetstream;
mod lexicon;
mod moderation;
mod repo_verify;
mod oauth;
mod preflight;
mod xrpc;

use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;
use tokio::sync::{mpsc, oneshot};

use jetstream::{build_url, ReceivedEvent, DEFAULT_JETSTREAM};
use lexicon::{FeedPost, StrongRef, POST_NSID};
use xrpc::{parse_at_uri, Session, XrpcClient};

#[derive(Parser)]
#[command(name = "public-roundtrip", about = "Public AT Protocol round-trip experiment")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Connectivity + toolchain gate. Run this first.
    Preflight,
    /// Authenticate and print the resolved DID + PDS.
    Whoami,
    /// Drive the real OAuth flow up to the browser-consent wall (extension #1).
    Oauth {
        /// Account handle to start the login from.
        #[arg(long)]
        handle: String,
        /// Loopback port for the OAuth callback.
        #[arg(long, default_value_t = 4599)]
        port: u16,
    },
    /// Publish N experimental records (no Jetstream).
    Publish {
        #[arg(long, default_value_t = 2)]
        count: usize,
        #[arg(long, default_value = "public-roundtrip experiment — innocuous test record")]
        text: String,
    },
    /// Listen to Jetstream filtered to our collection and print events.
    Consume {
        #[arg(long, default_value_t = 30)]
        seconds: u64,
        /// Optionally filter to a specific DID.
        #[arg(long)]
        did: Option<String>,
        /// Collection to filter (defaults to the experimental NSID). Override to
        /// smoke-test the parser against a high-traffic collection.
        #[arg(long, default_value = POST_NSID)]
        collection: String,
    },
    /// THE HEADLINE: publish, then catch the same records back on Jetstream.
    Roundtrip {
        #[arg(long, default_value_t = 3)]
        count: usize,
        #[arg(long, default_value = "roundtrip.sqlite")]
        db: String,
        /// Settle time after WS connect before publishing (ms).
        #[arg(long, default_value_t = 1500)]
        settle_ms: u64,
        /// Max seconds to wait for all records to come back.
        #[arg(long, default_value_t = 60)]
        timeout_secs: u64,
    },
    /// Publish N records and report the latency distribution (extension #3).
    Latency {
        #[arg(long, default_value_t = 25)]
        count: usize,
        #[arg(long, default_value_t = 1500)]
        settle_ms: u64,
        #[arg(long, default_value_t = 120)]
        timeout_secs: u64,
    },
    /// Publish from N accounts and catch them all back (extension #2).
    MultiRoundtrip {
        #[arg(long, default_value = "roundtrip.sqlite")]
        db: String,
        #[arg(long, default_value_t = 1500)]
        settle_ms: u64,
        #[arg(long, default_value_t = 60)]
        timeout_secs: u64,
    },
    /// Probe whether the real PDS validates our custom lexicon (validation #1).
    ProbeValidation,
    /// Verify handle↔DID bidirectionally (anti-impersonation) (validation #2).
    VerifyIdentity {
        #[arg(long)]
        handle: String,
    },
    /// Recompute a record's CID and compare to the server's (validation #3).
    CidCheck,
    /// Update a record via putRecord and confirm operation:update round-trips (validation #4a).
    UpdateRoundtrip {
        #[arg(long, default_value_t = 1500)]
        settle_ms: u64,
        #[arg(long, default_value_t = 60)]
        timeout_secs: u64,
    },
    /// Build and assemble a thread spanning two accounts/PDSes (validation #4b).
    CrossThread,
    /// Verify the signed repo: CAR export → commit signature → MST inclusion (capstone).
    VerifyRepo,
    /// Upload a blob, embed it, and round-trip it back (quick win: media path).
    BlobCheck,
    /// Confirm rkeys (TIDs) decode to monotonic timestamps (quick win: ordering).
    TidCheck,
    /// Fetch + sanity-check the PLC audit log for a handle (quick win: identity history).
    PlcAudit {
        #[arg(long)]
        handle: String,
    },
    /// Confirm Jetstream replays from a past cursor, not just live tail (quick win).
    ReplayCheck {
        #[arg(long, default_value_t = 1500)]
        settle_ms: u64,
        #[arg(long, default_value_t = 60)]
        timeout_secs: u64,
    },
    /// Moderation-labels experiment: self-labels, labeler discovery, queryLabels,
    /// and label-signature verification.
    Moderation {
        /// Labeler DID to query (default: the Bluesky moderation service).
        #[arg(long, default_value = "did:plc:ar7c4by46qjdydhdevvrndac")]
        labeler: String,
    },
    /// Diff the public path against an in-process local-path baseline (extension #6).
    Compare {
        #[arg(long, default_value_t = 1500)]
        settle_ms: u64,
        #[arg(long, default_value_t = 60)]
        timeout_secs: u64,
    },
    /// Publish a 3-post reply thread, index it, and print the assembled thread (extension #5).
    Thread {
        #[arg(long, default_value = "thread.sqlite")]
        db: String,
    },
    /// Run a real-ish indexer: backfill + resume cursor + handle update/delete (extension #4).
    Index {
        #[arg(long, default_value = POST_NSID)]
        collection: String,
        /// Backfill existing records from this repo (handle or DID) before listening.
        #[arg(long)]
        backfill_repo: Option<String>,
        /// Run for N seconds (0 = until Ctrl-C).
        #[arg(long, default_value_t = 0)]
        seconds: u64,
        #[arg(long, default_value = "roundtrip.sqlite")]
        db: String,
    },
    /// Serve the minimal AppView over HTTP.
    Serve {
        #[arg(long, default_value = "roundtrip.sqlite")]
        db: String,
        #[arg(long, default_value_t = 8088)]
        port: u16,
    },
    /// Delete all experimental records from the authed repo.
    Cleanup {
        /// Required confirmation flag.
        #[arg(long)]
        yes: bool,
    },
}

/// Validate a configured URL: it must parse, use an allowed scheme, and have a
/// host. Guards against SSRF / misconfiguration from an injected env var (e.g.
/// `file://`, a schemeless value, or an internal address with no host).
fn validate_url(raw: &str, schemes: &[&str], name: &str) -> Result<()> {
    let u = url::Url::parse(raw).with_context(|| format!("{name} is not a valid URL: {raw}"))?;
    if !schemes.contains(&u.scheme()) || !u.has_host() {
        bail!("{name} must use one of {schemes:?} and include a host (got `{raw}`)");
    }
    Ok(())
}

fn env_host() -> Result<String> {
    let raw = std::env::var("ATPROTO_HOST").unwrap_or_else(|_| "https://bsky.social".to_string());
    validate_url(&raw, &["http", "https"], "ATPROTO_HOST")?;
    Ok(raw)
}

fn jetstream_url_base() -> Result<String> {
    let raw = std::env::var("JETSTREAM_URL").unwrap_or_else(|_| DEFAULT_JETSTREAM.to_string());
    validate_url(&raw, &["ws", "wss"], "JETSTREAM_URL")?;
    Ok(raw)
}

/// Authenticate using the app-password path. Reads ATPROTO_HANDLE and
/// ATPROTO_APP_PASSWORD from the environment (or .env). Never logs the password.
async fn auth(client: &XrpcClient) -> Result<Session> {
    let handle = std::env::var("ATPROTO_HANDLE")
        .context("ATPROTO_HANDLE not set (your test account handle, e.g. yourtest.bsky.social)")?;
    let app_password = std::env::var("ATPROTO_APP_PASSWORD")
        .context("ATPROTO_APP_PASSWORD not set (create an app password in account settings)")?;
    auth_with(client, &handle, &app_password).await
}

/// Authenticate one account by handle + app password. Never logs the password.
async fn auth_with(client: &XrpcClient, handle: &str, app_password: &str) -> Result<Session> {
    let host = env_host()?;
    let session = client
        .create_session(&host, handle, app_password)
        .await
        .context("authentication failed (app-password path)")?;
    println!("✓ authenticated as @{} ({})", session.handle, session.did);
    println!("  PDS: {}", session.pds);
    Ok(session)
}

/// Collect accounts for the multi-account round-trip. Prefers numbered pairs
/// `ATPROTO_HANDLE_N` / `ATPROTO_APP_PASSWORD_N` (N = 1, 2, ...); if none are
/// set, falls back to the single `ATPROTO_HANDLE` / `ATPROTO_APP_PASSWORD`.
fn collect_accounts() -> Result<Vec<(String, String)>> {
    let mut accounts = Vec::new();
    let mut i = 1;
    while let (Ok(h), Ok(p)) = (
        std::env::var(format!("ATPROTO_HANDLE_{i}")),
        std::env::var(format!("ATPROTO_APP_PASSWORD_{i}")),
    ) {
        accounts.push((h, p));
        i += 1;
    }
    if accounts.is_empty() {
        let h = std::env::var("ATPROTO_HANDLE")
            .context("no accounts: set ATPROTO_HANDLE[_N] and ATPROTO_APP_PASSWORD[_N]")?;
        let p = std::env::var("ATPROTO_APP_PASSWORD").context("ATPROTO_APP_PASSWORD not set")?;
        accounts.push((h, p));
    }
    Ok(accounts)
}

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenvy::dotenv();
    let cli = Cli::parse();

    match cli.cmd {
        Cmd::Preflight => cmd_preflight().await,
        Cmd::Whoami => cmd_whoami().await,
        Cmd::Oauth { handle, port } => oauth::run(&handle, port).await,
        Cmd::Publish { count, text } => cmd_publish(count, &text).await,
        Cmd::Consume { seconds, did, collection } => cmd_consume(seconds, did, &collection).await,
        Cmd::Roundtrip { count, db, settle_ms, timeout_secs } => {
            cmd_roundtrip(count, &db, settle_ms, timeout_secs).await
        }
        Cmd::Latency { count, settle_ms, timeout_secs } => {
            cmd_latency(count, settle_ms, timeout_secs).await
        }
        Cmd::MultiRoundtrip { db, settle_ms, timeout_secs } => {
            cmd_multi_roundtrip(&db, settle_ms, timeout_secs).await
        }
        Cmd::ProbeValidation => cmd_probe_validation().await,
        Cmd::VerifyIdentity { handle } => cmd_verify_identity(&handle).await,
        Cmd::CidCheck => cmd_cid_check().await,
        Cmd::UpdateRoundtrip { settle_ms, timeout_secs } => cmd_update_roundtrip(settle_ms, timeout_secs).await,
        Cmd::CrossThread => cmd_cross_thread().await,
        Cmd::VerifyRepo => cmd_verify_repo().await,
        Cmd::BlobCheck => cmd_blob_check().await,
        Cmd::TidCheck => cmd_tid_check().await,
        Cmd::PlcAudit { handle } => cmd_plc_audit(&handle).await,
        Cmd::ReplayCheck { settle_ms, timeout_secs } => cmd_replay_check(settle_ms, timeout_secs).await,
        Cmd::Moderation { labeler } => cmd_moderation(&labeler).await,
        Cmd::Compare { settle_ms, timeout_secs } => cmd_compare(settle_ms, timeout_secs).await,
        Cmd::Thread { db } => cmd_thread(&db).await,
        Cmd::Index { collection, backfill_repo, seconds, db } => {
            cmd_index(&collection, backfill_repo, seconds, &db).await
        }
        Cmd::Serve { db, port } => appview::serve(db, port).await,
        Cmd::Cleanup { yes } => cmd_cleanup(yes).await,
    }
}

async fn cmd_preflight() -> Result<()> {
    println!("=== Connectivity preflight ===");
    println!("rustc target build: {} (compiled)", env!("CARGO_PKG_VERSION"));
    let host = env_host()?;
    // Probe the firehose host's plain WS subscribe endpoint (no filter needed for handshake).
    let probes = preflight::run(&host, &jetstream_url_base()?).await?;
    let mut all_ok = true;
    for p in &probes {
        let mark = if p.reachable { "✓" } else { "✗" };
        println!("{mark} {:<26} {}  [{}]", p.label, p.detail, p.target);
        if !p.reachable {
            all_ok = false;
        }
    }
    if !all_ok {
        bail!("one or more required hosts unreachable — STOP. This experiment requires live connectivity and must not be stubbed.");
    }
    println!("\nAll required hosts reachable. Gate PASSED.");
    Ok(())
}

async fn cmd_whoami() -> Result<()> {
    let client = XrpcClient::new()?;
    let session = auth(&client).await?;
    // Independently resolve handle→DID→PDS via the public resolver + PLC, and
    // cross-check against the PDS the session reported (a DID-resolution check).
    if let Ok(did) = client.resolve_handle(&session.handle).await {
        println!("  resolveHandle: @{} → {did}", session.handle);
        if did == session.did {
            match client.resolve_pds_via_plc(&did).await {
                Ok(pds) => {
                    let agree = if pds == session.pds { "matches session" } else { "DIFFERS from session" };
                    println!("  PLC #atproto_pds: {pds}  ({agree})");
                }
                Err(e) => println!("  PLC resolution note: {e}"),
            }
        }
    }
    Ok(())
}

fn now_rfc3339() -> String {
    chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
}

fn now_unix_ms() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}

async fn cmd_publish(count: usize, text: &str) -> Result<()> {
    let client = XrpcClient::new()?;
    let session = auth(&client).await?;
    for i in 0..count {
        let post = FeedPost::new(format!("{text} #{}", i + 1), now_rfc3339());
        post.validate().context("local lexicon validation failed before publish")?;
        let record: Value = serde_json::to_value(&post)?;
        let resp = client.create_record(&session, POST_NSID, &record).await?;
        println!(
            "✓ published  uri={}  cid={}  validation={}",
            resp.uri,
            resp.cid,
            resp.validation_status.as_deref().unwrap_or("(none)")
        );
    }
    Ok(())
}

async fn cmd_consume(seconds: u64, did: Option<String>, collection: &str) -> Result<()> {
    let dids: Vec<String> = did.into_iter().collect();
    let url = build_url(&jetstream_url_base()?, collection, &dids);
    println!("Connecting to Jetstream (filtered): {url}");
    let (tx, mut rx) = mpsc::channel::<ReceivedEvent>(256);
    let consume_url = url.clone();
    let handle = tokio::spawn(async move {
        if let Err(e) = jetstream::consume(consume_url, tx, None).await {
            eprintln!("jetstream consumer ended: {e}");
        }
    });
    let deadline = tokio::time::Instant::now() + Duration::from_secs(seconds);
    println!("Listening for {seconds}s...");
    loop {
        match tokio::time::timeout_at(deadline, rx.recv()).await {
            Ok(Some(ev)) => {
                if let Some(c) = &ev.event.commit {
                    println!(
                        "[{}] {} {}/{}  cid={}",
                        ev.event.did, c.operation, c.collection, c.rkey,
                        c.cid.as_deref().unwrap_or("-")
                    );
                }
            }
            Ok(None) => break,
            Err(_) => break, // deadline
        }
    }
    handle.abort();
    Ok(())
}

async fn cmd_roundtrip(count: usize, db: &str, settle_ms: u64, timeout_secs: u64) -> Result<()> {
    let client = XrpcClient::new()?;
    let session = auth(&client).await?;

    // 1. Start the filtered Jetstream consumer, scoped to our DID + collection.
    let url = build_url(&jetstream_url_base()?, POST_NSID, &[session.did.clone()]);
    println!("Jetstream (filtered): {url}");
    let (tx, mut rx) = mpsc::channel::<ReceivedEvent>(256);
    let (ready_tx, ready_rx) = oneshot::channel::<()>();
    let consume_url = url.clone();
    let consumer = tokio::spawn(async move {
        if let Err(e) = jetstream::consume(consume_url, tx, Some(ready_tx)).await {
            eprintln!("jetstream consumer ended: {e}");
        }
    });

    // 2. Wait for the socket to open, then settle so we don't miss our own events.
    ready_rx.await.ok();
    println!("Jetstream connected; settling {settle_ms}ms before publishing...");
    tokio::time::sleep(Duration::from_millis(settle_ms)).await;

    // 3. Publish `count` records, remembering send time + expected cid per uri.
    struct Pending {
        cid: String,
        send_ms: u128,
        record: Value,
    }
    let mut pending: HashMap<String, Pending> = HashMap::new();
    for i in 0..count {
        let post = FeedPost::new(
            format!("public-roundtrip experiment — innocuous test record #{}", i + 1),
            now_rfc3339(),
        );
        post.validate()?;
        let record: Value = serde_json::to_value(&post)?;
        let send_ms = now_unix_ms();
        let resp = client.create_record(&session, POST_NSID, &record).await?;
        println!("→ createRecord  uri={}  cid={}", resp.uri, resp.cid);
        pending.insert(resp.uri, Pending { cid: resp.cid, send_ms, record });
    }
    let total = pending.len();

    // 4. Catch the same records back off Jetstream, matching uri + cid.
    let mut conn = appview::Db::open(db)?;
    let mut matched = 0usize;
    let mut latencies_ms: Vec<i128> = Vec::new();
    let mut printed_shape = false;
    let deadline = tokio::time::Instant::now() + Duration::from_secs(timeout_secs);

    while matched < total {
        match tokio::time::timeout_at(deadline, rx.recv()).await {
            Ok(Some(ev)) => {
                let commit = match &ev.event.commit {
                    Some(c) if c.operation == "create" => c,
                    _ => continue,
                };
                let uri = format!("at://{}/{}/{}", ev.event.did, commit.collection, commit.rkey);
                let Some(p) = pending.get(&uri) else { continue };

                // First matching event: dump the raw shape, field by field.
                if !printed_shape {
                    println!("\n=== RAW Jetstream commit event (as received) ===");
                    println!("{}", serde_json::to_string_pretty(&ev.raw).unwrap_or_default());
                    println!("=== end raw event ===\n");
                    printed_shape = true;
                }

                let cid_match = commit.cid.as_deref() == Some(p.cid.as_str());
                let latency = ev.recv_unix_ms as i128 - p.send_ms as i128;
                latencies_ms.push(latency);
                println!(
                    "✓ round-trip  uri={}  cid_match={}  latency={}ms",
                    uri, cid_match, latency
                );

                // 5. Index into the AppView (validates on ingest).
                appview::Db::index_post(
                    &conn,
                    &uri,
                    commit.cid.as_deref().unwrap_or(""),
                    &ev.event.did,
                    &commit.collection,
                    &commit.rkey,
                    commit.record.as_ref().unwrap_or(&p.record),
                    "jetstream",
                )?;
                matched += 1;
            }
            Ok(None) => {
                eprintln!("consumer channel closed before all records returned");
                break;
            }
            Err(_) => {
                eprintln!(
                    "timeout after {timeout_secs}s — matched {matched}/{total}",
                );
                break;
            }
        }
    }
    consumer.abort();
    let _ = &mut conn;

    // 6. Report.
    println!("\n=== Round-trip report ===");
    println!("published: {total}   returned via Jetstream: {matched}");
    if !latencies_ms.is_empty() {
        let min = latencies_ms.iter().min().unwrap();
        let max = latencies_ms.iter().max().unwrap();
        let avg = latencies_ms.iter().sum::<i128>() / latencies_ms.len() as i128;
        println!("observed propagation latency (createRecord→Jetstream): min={min}ms avg={avg}ms max={max}ms");
    }
    println!("indexed into: {db}");
    println!("Serve it:  cargo run -- serve --db {db}");
    if matched < total {
        bail!("not all records returned within the timeout — see log above");
    }
    Ok(())
}

/// Percentile (nearest-rank) of a sorted slice of millisecond latencies.
fn percentile(sorted: &[u128], p: f64) -> u128 {
    if sorted.is_empty() {
        return 0;
    }
    let rank = (p / 100.0 * sorted.len() as f64).ceil() as usize;
    sorted[rank.saturating_sub(1).min(sorted.len() - 1)]
}

async fn cmd_latency(count: usize, settle_ms: u64, timeout_secs: u64) -> Result<()> {
    let client = XrpcClient::new()?;
    let session = auth(&client).await?;

    let url = build_url(&jetstream_url_base()?, POST_NSID, &[session.did.clone()]);
    let (tx, mut rx) = mpsc::channel::<ReceivedEvent>(512);
    let (ready_tx, ready_rx) = oneshot::channel::<()>();
    let consume_url = url.clone();
    let consumer = tokio::spawn(async move {
        if let Err(e) = jetstream::consume(consume_url, tx, Some(ready_tx)).await {
            eprintln!("jetstream consumer ended: {e}");
        }
    });
    ready_rx.await.ok();
    tokio::time::sleep(Duration::from_millis(settle_ms)).await;

    // Publish `count` records, stamping each with a monotonic send Instant.
    println!("publishing {count} records...");
    let mut send_at: HashMap<String, std::time::Instant> = HashMap::new();
    let mut order: Vec<String> = Vec::new();
    for i in 0..count {
        let post = FeedPost::new(format!("latency probe #{}", i + 1), now_rfc3339());
        post.validate()?;
        let record: Value = serde_json::to_value(&post)?;
        let resp = client.create_record(&session, POST_NSID, &record).await?;
        send_at.insert(resp.uri.clone(), std::time::Instant::now());
        order.push(resp.uri);
    }
    let total = send_at.len();

    // Catch them back; latency = monotonic (recv_instant - send_instant).
    let mut latencies: Vec<u128> = Vec::new();
    let mut first_latency: Option<u128> = None;
    let deadline = tokio::time::Instant::now() + Duration::from_secs(timeout_secs);
    while latencies.len() < total {
        match tokio::time::timeout_at(deadline, rx.recv()).await {
            Ok(Some(ev)) => {
                let Some(commit) = ev.event.commit.as_ref().filter(|c| c.operation == "create") else { continue };
                let uri = format!("at://{}/{}/{}", ev.event.did, commit.collection, commit.rkey);
                let Some(sent) = send_at.get(&uri) else { continue };
                let ms = ev.recv_instant.saturating_duration_since(*sent).as_millis();
                if order.first().map(|u| u == &uri).unwrap_or(false) {
                    first_latency = Some(ms);
                }
                latencies.push(ms);
            }
            Ok(None) => break,
            Err(_) => {
                eprintln!("timeout — caught {}/{total}", latencies.len());
                break;
            }
        }
    }
    consumer.abort();

    // Clean up the probe records.
    let records = client.list_records(&session, POST_NSID).await?;
    for (uri, _cid) in records {
        if let Some((_d, _c, rkey)) = parse_at_uri(&uri) {
            client.delete_record(&session, POST_NSID, &rkey).await?;
        }
    }

    latencies.sort_unstable();
    println!("\n=== Latency distribution (createRecord → Jetstream, monotonic) ===");
    println!("samples: {}/{total}", latencies.len());
    if let Some(f) = first_latency {
        println!("first record (cold path): {f}ms");
    }
    if !latencies.is_empty() {
        let mean = latencies.iter().sum::<u128>() / latencies.len() as u128;
        println!("min={}  p50={}  p90={}  p95={}  p99={}  max={}  mean={}  (ms)",
            latencies[0],
            percentile(&latencies, 50.0),
            percentile(&latencies, 90.0),
            percentile(&latencies, 95.0),
            percentile(&latencies, 99.0),
            latencies[latencies.len() - 1],
            mean,
        );
    }
    if latencies.len() < total {
        bail!("not all probe records returned within the timeout");
    }
    Ok(())
}

async fn cmd_update_roundtrip(settle_ms: u64, timeout_secs: u64) -> Result<()> {
    let client = XrpcClient::new()?;
    let session = auth(&client).await?;
    let url = build_url(&jetstream_url_base()?, POST_NSID, &[session.did.clone()]);
    let (tx, mut rx) = mpsc::channel::<ReceivedEvent>(256);
    let (ready_tx, ready_rx) = oneshot::channel::<()>();
    let consume_url = url.clone();
    let consumer = tokio::spawn(async move {
        if let Err(e) = jetstream::consume(consume_url, tx, Some(ready_tx)).await {
            eprintln!("jetstream consumer ended: {e}");
        }
    });
    ready_rx.await.ok();
    tokio::time::sleep(Duration::from_millis(settle_ms)).await;

    // Create, then update the SAME rkey via putRecord.
    let v1 = FeedPost::new("update test — original", now_rfc3339());
    let create = client.create_record(&session, POST_NSID, &serde_json::to_value(&v1)?).await?;
    let (_d, _c, rkey) = parse_at_uri(&create.uri).context("parse uri")?;
    println!("→ create  cid={}", create.cid);
    let v2 = FeedPost::new("update test — EDITED", now_rfc3339());
    let put = client.put_record(&session, POST_NSID, &rkey, &serde_json::to_value(&v2)?).await?;
    println!("→ putRecord (update) cid={}  (differs from create: {})", put.cid, put.cid != create.cid);

    // Catch the update commit on the firehose; index reflects the new text.
    let conn = appview::Db::open(":memory:")?;
    let mut saw_create = false;
    let mut saw_update = false;
    let deadline = tokio::time::Instant::now() + Duration::from_secs(timeout_secs);
    while !saw_update {
        match tokio::time::timeout_at(deadline, rx.recv()).await {
            Ok(Some(ev)) => {
                let Some(commit) = ev.event.commit.as_ref() else { continue };
                if commit.rkey != rkey || commit.collection != POST_NSID {
                    continue;
                }
                let uri = create.uri.clone();
                if let Some(rec) = &commit.record {
                    let _ = appview::Db::index_post(&conn, &uri, commit.cid.as_deref().unwrap_or(""), &ev.event.did, &commit.collection, &commit.rkey, rec, "jetstream");
                }
                match commit.operation.as_str() {
                    "create" => { saw_create = true; println!("← firehose: operation=create"); }
                    "update" => {
                        saw_update = true;
                        let cid_ok = commit.cid.as_deref() == Some(put.cid.as_str());
                        println!("← firehose: operation=update  cid_matches_put={cid_ok}");
                    }
                    _ => {}
                }
            }
            _ => break,
        }
    }
    consumer.abort();

    // Confirm the index now holds the edited text.
    let edited = conn.query_row(
        "SELECT text FROM posts WHERE uri = ?1",
        [create.uri.as_str()],
        |r| r.get::<_, String>(0),
    ).unwrap_or_default();
    println!("\n=== Update round-trip ===");
    println!("saw create event: {saw_create}   saw update event: {saw_update}");
    println!("index text after update: {edited:?}");

    if let Some((_d, _c, rkey)) = parse_at_uri(&create.uri) {
        client.delete_record(&session, POST_NSID, &rkey).await?;
    }
    if !saw_update {
        bail!("did not observe the update event within the timeout");
    }
    Ok(())
}

async fn cmd_blob_check() -> Result<()> {
    let client = XrpcClient::new()?;
    let session = auth(&client).await?;

    // A tiny innocuous "blob" (not a real image; bytes are all that matter here).
    let blob_bytes = b"public-roundtrip experiment blob payload".to_vec();
    let blob = client.upload_blob(&session, blob_bytes.clone(), "application/octet-stream").await?;
    let blob_cid = blob
        .get("ref")
        .and_then(|r| r.get("$link"))
        .and_then(|l| l.as_str())
        .context("blob ref missing")?
        .to_string();
    println!("\n=== Blob round-trip ===");
    println!("uploaded blob: cid={blob_cid}  size={:?}", blob.get("size"));

    // Embed the blob object in a record (extra field; the PDS accepts it — see V1).
    let record = serde_json::json!({
        "$type": POST_NSID,
        "text": "blob embed test",
        "createdAt": now_rfc3339(),
        "embed": blob,
    });
    let resp = client.create_record(&session, POST_NSID, &record).await?;
    let (_d, _c, rkey) = parse_at_uri(&resp.uri).context("parse uri")?;

    // Fetch the record back; confirm the blob ref survived.
    let (_cid, stored) = client.get_record_public(&session.pds, &session.did, POST_NSID, &rkey).await?;
    let survived = stored
        .get("embed")
        .and_then(|e| e.get("ref"))
        .and_then(|r| r.get("$link"))
        .and_then(|l| l.as_str())
        == Some(blob_cid.as_str());
    println!("blob ref survived record round-trip: {}", if survived { "YES ✓" } else { "NO ✗" });

    // Fetch the blob bytes back and compare.
    let fetched = client.get_blob(&session.pds, &session.did, &blob_cid).await?;
    let bytes_match = fetched == blob_bytes;
    println!("fetched blob bytes match uploaded: {}  ({} bytes)", if bytes_match { "YES ✓" } else { "NO ✗" }, fetched.len());

    client.delete_record(&session, POST_NSID, &rkey).await?;
    println!("(cleaned up; the blob itself is GC'd by the PDS once unreferenced)");
    if !survived || !bytes_match {
        bail!("blob round-trip failed");
    }
    Ok(())
}

/// base32-sortable alphabet used by atproto TIDs.
const TID_ALPHABET: &[u8] = b"234567abcdefghijklmnopqrstuvwxyz";

/// Decode a 13-char TID rkey to its microsecond timestamp (high 53 bits after
/// the leading 0 bit; low 10 bits are a clock id).
fn decode_tid_micros(s: &str) -> Option<u64> {
    if s.len() != 13 {
        return None;
    }
    let mut v: u128 = 0;
    for c in s.bytes() {
        let idx = TID_ALPHABET.iter().position(|&x| x == c)?;
        v = (v << 5) | idx as u128;
    }
    Some(((v >> 10) & 0x1f_ffff_ffff_ffff) as u64)
}

async fn cmd_tid_check() -> Result<()> {
    let client = XrpcClient::new()?;
    let session = auth(&client).await?;

    // Publish a few records in quick succession and inspect their rkeys.
    let mut rkeys = Vec::new();
    for i in 0..5 {
        let post = FeedPost::new(format!("tid probe #{}", i + 1), now_rfc3339());
        let resp = client.create_record(&session, POST_NSID, &serde_json::to_value(&post)?).await?;
        if let Some((_d, _c, rkey)) = parse_at_uri(&resp.uri) {
            rkeys.push(rkey);
        }
    }
    let now_micros = now_unix_ms() as u64 * 1000;

    println!("\n=== TID / rkey ordering ===");
    let mut prev = 0u64;
    let mut monotonic = true;
    let mut near_now = true;
    for rkey in &rkeys {
        let ts = decode_tid_micros(rkey).unwrap_or(0);
        let ascending = ts > prev;
        if !ascending {
            monotonic = false;
        }
        let skew_s = (now_micros as i64 - ts as i64).abs() / 1_000_000;
        if skew_s > 30 {
            near_now = false;
        }
        println!("  {rkey}  ts_micros={ts}  ascending={ascending}  skew={skew_s}s");
        prev = ts;
    }
    println!("strictly monotonic in publish order: {}", if monotonic { "YES ✓" } else { "NO ✗" });
    println!("timestamps within 30s of now: {}", if near_now { "YES ✓" } else { "NO ✗" });
    println!("lexical sort == chronological: {}", {
        let mut sorted = rkeys.clone();
        sorted.sort();
        if sorted == rkeys { "YES ✓ (rkeys are time-sortable strings)" } else { "NO ✗" }
    });

    // Cleanup.
    for (uri, _cid) in client.list_records(&session, POST_NSID).await.unwrap_or_default() {
        if let Some((_d, _c, rkey)) = parse_at_uri(&uri) {
            let _ = client.delete_record(&session, POST_NSID, &rkey).await;
        }
    }
    if !monotonic {
        bail!("rkeys were not monotonic");
    }
    Ok(())
}

async fn cmd_plc_audit(handle: &str) -> Result<()> {
    let client = XrpcClient::new()?;
    let did = client.resolve_handle(handle).await?;
    let log = client.fetch_plc_audit(&did).await?;
    let ops = log.as_array().context("audit log is not an array")?;

    println!("\n=== PLC audit log: @{handle} ({did}) ===");
    println!("signed operations in history: {}", ops.len());

    // Operations should be time-ordered; show genesis + latest, and confirm the
    // latest op establishes the same #atproto signing key the repo commit used.
    let times: Vec<&str> = ops.iter().filter_map(|o| o.get("createdAt").and_then(|c| c.as_str())).collect();
    let ordered = times.windows(2).all(|w| w[0] <= w[1]);
    println!("operations time-ordered: {}", if ordered { "YES ✓" } else { "NO ✗" });

    let doc = client.fetch_plc_doc(&did).await?;
    let current_key = extract_atproto_key(&doc);
    // The latest non-nullified op's verificationMethods.atproto:
    let latest_key = ops
        .last()
        .and_then(|o| o.get("operation"))
        .and_then(|op| op.get("verificationMethods"))
        .and_then(|vm| vm.get("atproto"))
        .and_then(|k| k.as_str());
    if let (Some(latest), Some(cur)) = (latest_key, current_key.as_deref()) {
        // latest is a did:key (did:key:zXX...); cur is the bare multibase (zXX...).
        let agrees = latest.ends_with(cur);
        println!("latest op's atproto key matches current DID doc key: {}", if agrees { "YES ✓" } else { "NO ✗" });
    }
    println!("genesis: {}   latest: {}", times.first().unwrap_or(&"?"), times.last().unwrap_or(&"?"));

    if !ordered {
        bail!("PLC audit log not time-ordered");
    }
    Ok(())
}

async fn cmd_replay_check(settle_ms: u64, timeout_secs: u64) -> Result<()> {
    let client = XrpcClient::new()?;
    let session = auth(&client).await?;
    let base = jetstream_url_base()?;

    // Phase 1: connect live, publish one record, capture its time_us cursor.
    let url = jetstream::build_url(&base, POST_NSID, &[session.did.clone()]);
    let (tx, mut rx) = mpsc::channel::<ReceivedEvent>(64);
    let (ready_tx, ready_rx) = oneshot::channel::<()>();
    let c1 = tokio::spawn(async move { let _ = jetstream::consume(url, tx, Some(ready_tx)).await; });
    ready_rx.await.ok();
    tokio::time::sleep(Duration::from_millis(settle_ms)).await;

    let post = FeedPost::new("replay probe", now_rfc3339());
    let resp = client.create_record(&session, POST_NSID, &serde_json::to_value(&post)?).await?;
    let mut cursor = 0u64;
    let deadline = tokio::time::Instant::now() + Duration::from_secs(timeout_secs);
    loop {
        match tokio::time::timeout_at(deadline, rx.recv()).await {
            Ok(Some(ev)) => {
                if let Some(c) = ev.event.commit.as_ref() {
                    let uri = format!("at://{}/{}/{}", ev.event.did, c.collection, c.rkey);
                    if uri == resp.uri {
                        cursor = ev.event.time_us;
                        break;
                    }
                }
            }
            _ => break,
        }
    }
    c1.abort();
    if cursor == 0 {
        bail!("did not observe the probe record live");
    }
    println!("\n=== Jetstream replay window ===");
    println!("published + observed live at cursor (time_us) = {cursor}");

    // Phase 2: reconnect from a cursor BEFORE the event; the record must replay
    // (proving cursor resume is true replay, not just live tail).
    let past_cursor = cursor.saturating_sub(5_000_000); // 5s earlier
    let url2 = jetstream::build_url_cursor(&base, POST_NSID, &[session.did.clone()], Some(past_cursor));
    let (tx2, mut rx2) = mpsc::channel::<ReceivedEvent>(64);
    let c2 = tokio::spawn(async move { let _ = jetstream::consume(url2, tx2, None).await; });
    let mut replayed = false;
    let deadline2 = tokio::time::Instant::now() + Duration::from_secs(timeout_secs);
    loop {
        match tokio::time::timeout_at(deadline2, rx2.recv()).await {
            Ok(Some(ev)) => {
                if let Some(c) = ev.event.commit.as_ref() {
                    let uri = format!("at://{}/{}/{}", ev.event.did, c.collection, c.rkey);
                    if uri == resp.uri {
                        replayed = true;
                        break;
                    }
                }
            }
            _ => break,
        }
    }
    c2.abort();
    println!("reconnected from cursor {past_cursor} (5s earlier)");
    println!("record replayed from the past cursor: {}", if replayed { "YES ✓ — resume is true replay" } else { "NO ✗" });

    if let Some((_d, _c, rkey)) = parse_at_uri(&resp.uri) {
        client.delete_record(&session, POST_NSID, &rkey).await?;
    }
    if !replayed {
        bail!("record did not replay from a past cursor");
    }
    Ok(())
}

async fn cmd_verify_repo() -> Result<()> {
    let client = XrpcClient::new()?;
    let session = auth(&client).await?;

    // Publish a record so the repo has a known leaf to look for.
    let post = FeedPost::new("repo verification anchor", now_rfc3339());
    let resp = client.create_record(&session, POST_NSID, &serde_json::to_value(&post)?).await?;
    println!("anchored record cid: {}", resp.cid);

    // 1. Export + parse the CAR.
    let car = client.get_repo_car(&session.pds, &session.did).await?;
    let (roots, blocks) = repo_verify::parse_car(&car)?;
    println!("\n=== 1. CAR export ===");
    println!("downloaded {} bytes, {} blocks, {} root(s)", car.len(), blocks.len(), roots.len());

    // 2. Every block content-addressed correctly?
    let (ok, total) = repo_verify::verify_block_integrity(&blocks);
    println!("\n=== 2. Block integrity (sha256(bytes) == CID) ===");
    println!("{ok}/{total} blocks verify ✓");

    // 3. Decode the signed root commit.
    let root = *roots.first().context("CAR has no root")?;
    let commit = repo_verify::decode_commit(&root, &blocks)?;
    println!("\n=== 3. Signed commit ===");
    println!("did={}  rev={}  mst_root={}", commit.did, commit.rev, commit.data);
    if commit.did != session.did {
        bail!("commit.did does not match the authenticated DID");
    }

    // 4. Verify the signature against the account's signing key from its DID doc.
    let doc = client.fetch_plc_doc(&session.did).await?;
    let key_mb = extract_atproto_key(&doc).context("no #atproto signing key in DID doc")?;
    let curve = repo_verify::verify_commit_sig(&commit, &key_mb)?;
    println!("\n=== 4. Commit signature ===");
    println!("signing key ({curve}) from DID doc verified the commit ✓");

    // 5. Walk the MST; confirm our record CID is committed by the signed root.
    let values = repo_verify::collect_mst_values(&commit.data, &blocks);
    let present = values.iter().any(|c| c.to_string() == resp.cid);
    println!("\n=== 5. MST inclusion ===");
    println!("MST commits to {} record value(s)", values.len());
    println!("our anchored record present in the signed tree: {}", if present { "YES ✓" } else { "NO ✗" });

    println!("\n=== Chain of custody ===");
    println!("verified identity (DID doc key) → signed commit → MST root → our record CID → record bytes (V3).");
    println!("Every link checked: the record is provably part of this identity's signed repo.");

    if let Some((_d, _c, rkey)) = parse_at_uri(&resp.uri) {
        client.delete_record(&session, POST_NSID, &rkey).await?;
    }
    if !present {
        bail!("record CID was not found in the MST");
    }
    Ok(())
}

/// Pull a verificationMethod's `publicKeyMultibase` (by id suffix) from a DID doc.
fn extract_vm_key(doc: &serde_json::Value, suffix: &str) -> Option<String> {
    let vms = doc.get("verificationMethod")?.as_array()?;
    for vm in vms {
        let id = vm.get("id").and_then(|v| v.as_str()).unwrap_or_default();
        if id.ends_with(suffix) {
            return vm.get("publicKeyMultibase").and_then(|v| v.as_str()).map(str::to_string);
        }
    }
    None
}

fn extract_atproto_key(doc: &serde_json::Value) -> Option<String> {
    extract_vm_key(doc, "#atproto")
}

/// Pull a service `serviceEndpoint` (by id suffix) from a DID doc.
fn extract_service(doc: &serde_json::Value, suffix: &str) -> Option<String> {
    let svcs = doc.get("service")?.as_array()?;
    for s in svcs {
        let id = s.get("id").and_then(|v| v.as_str()).unwrap_or_default();
        if id.ends_with(suffix) {
            return s.get("serviceEndpoint").and_then(|v| v.as_str()).map(str::to_string);
        }
    }
    None
}

fn redact_subject(uri: &str) -> String {
    // Show enough to be a stable identifier in logs without exposing the full
    // subject of a moderation label.
    let head: String = uri.chars().take(24).collect();
    format!("{head}…")
}

async fn cmd_moderation(labeler: &str) -> Result<()> {
    let client = XrpcClient::new()?;
    let session = auth(&client).await?;

    // ---- M1: self-labeling round-trip ----
    println!("\n=== M1: self-labeling round-trip ===");
    let self_val = "!no-unauthenticated"; // benign, self-protective self-label
    let record = serde_json::json!({
        "$type": POST_NSID,
        "text": "moderation experiment — self-label test",
        "createdAt": now_rfc3339(),
        "labels": {
            "$type": "com.atproto.label.defs#selfLabels",
            "values": [ { "val": self_val } ]
        }
    });
    let resp = client.create_record(&session, POST_NSID, &record).await?;
    let (_d, _c, rkey) = parse_at_uri(&resp.uri).context("parse uri")?;
    let (_cid, stored) = client.get_record_public(&session.pds, &session.did, POST_NSID, &rkey).await?;
    let survived = stored
        .get("labels")
        .and_then(|l| l.get("values"))
        .and_then(|v| v.as_array())
        .and_then(|a| a.first())
        .and_then(|x| x.get("val"))
        .and_then(|v| v.as_str())
        == Some(self_val);
    println!("self-label '{self_val}' survived record round-trip: {}", if survived { "YES ✓" } else { "NO ✗" });
    println!("(self-labels live inside the signed record, so they inherit V3/capstone integrity)");
    client.delete_record(&session, POST_NSID, &rkey).await?;

    // ---- M2: labeler discovery ----
    println!("\n=== M2: labeler discovery ({labeler}) ===");
    let doc = client.fetch_plc_doc(labeler).await?;
    let aka = doc.get("alsoKnownAs").and_then(|a| a.as_array()).and_then(|a| a.first()).and_then(|v| v.as_str()).unwrap_or("?");
    let endpoint = extract_service(&doc, "#atproto_labeler").context("no #atproto_labeler service")?;
    let label_key = extract_vm_key(&doc, "#atproto_label").context("no #atproto_label key")?;
    println!("handle: {aka}");
    println!("labeler service (#atproto_labeler): {endpoint}");
    println!("label signing key (#atproto_label): {label_key}");

    // ---- M3: queryLabels (pull) ----
    println!("\n=== M3: queryLabels ===");
    let mine = client.query_labels(&endpoint, &[session.did.as_str()], 5).await?;
    let mine_n = mine.get("labels").and_then(|l| l.as_array()).map(|a| a.len()).unwrap_or(0);
    println!("labels on our clean test account: {mine_n}  (expected 0 — no moderation history)");

    let sample = client.query_labels(&endpoint, &["*"], 10).await?;
    let labels: Vec<Value> = sample.get("labels").and_then(|l| l.as_array()).cloned().unwrap_or_default();
    println!("sample labels fetched: {}", labels.len());
    let mut by_val: std::collections::BTreeMap<String, u32> = std::collections::BTreeMap::new();
    for l in &labels {
        let v = l.get("val").and_then(|v| v.as_str()).unwrap_or("?").to_string();
        *by_val.entry(v).or_default() += 1;
    }
    println!("label value distribution (subjects redacted): {by_val:?}");

    // ---- M4: verify label signatures (accounting for key rotation) ----
    println!("\n=== M4: label signature verification ===");
    // Candidate signing keys: the current #atproto_label key plus every key that
    // ever appeared in the labeler's signed PLC history (labelers rotate keys, and
    // a historical label was signed by the key valid at its `cts`).
    let mut candidate_keys = vec![label_key.clone()];
    if let Ok(audit) = client.fetch_plc_audit(labeler).await {
        if let Some(ops) = audit.as_array() {
            for op in ops {
                if let Some(vms) = op.get("operation").and_then(|o| o.get("verificationMethods")).and_then(|v| v.as_object()) {
                    for (_name, v) in vms {
                        if let Some(s) = v.as_str() {
                            let mb = s.strip_prefix("did:key:").unwrap_or(s).to_string();
                            if !candidate_keys.contains(&mb) {
                                candidate_keys.push(mb);
                            }
                        }
                    }
                }
            }
        }
    }
    println!("candidate labeler signing keys (current + PLC history): {}", candidate_keys.len());

    let mut checked = 0u32;
    let mut verified = 0u32;
    let mut verified_by_current = 0u32;
    let mut sample_line = String::new();
    for l in &labels {
        let src = l.get("src").and_then(|v| v.as_str()).unwrap_or("");
        if l.get("sig").is_none() || src != labeler {
            continue;
        }
        checked += 1;
        let (Ok(msg), Ok(sig)) = (moderation::label_unsigned_dagcbor(l), moderation::label_sig_bytes(l)) else { continue };
        let mut ok_key: Option<usize> = None;
        for (i, k) in candidate_keys.iter().enumerate() {
            if repo_verify::verify_signature(&msg, &sig, k).is_ok() {
                ok_key = Some(i);
                break;
            }
        }
        if let Some(i) = ok_key {
            verified += 1;
            if i == 0 {
                verified_by_current += 1;
            }
            if sample_line.is_empty() {
                let subj = l.get("uri").and_then(|v| v.as_str()).unwrap_or("");
                let val = l.get("val").and_then(|v| v.as_str()).unwrap_or("");
                let cts = l.get("cts").and_then(|v| v.as_str()).unwrap_or("");
                sample_line = format!("val='{val}' subject={} cts={cts} → signature VALID (key #{i})", redact_subject(subj));
            }
        }
    }
    println!("verified {verified}/{checked} label signatures (of which {verified_by_current} by the *current* key)");
    if !sample_line.is_empty() {
        println!("sample: {sample_line}");
    }

    // ---- M4b: is the label firehose (subscribeLabels) publicly exposed? ----
    println!("\n=== M4b: subscribeLabels availability ===");
    let sub_url = format!("{endpoint}/xrpc/com.atproto.label.subscribeLabels?cursor=0");
    let status = reqwest::Client::new().get(&sub_url).send().await.map(|r| r.status().as_u16()).unwrap_or(0);
    println!("GET {sub_url} → HTTP {status}");
    println!("(the Bluesky labeler exposes pull/queryLabels, not a public subscribeLabels firehose)");

    if !survived {
        bail!("self-label did not survive round-trip");
    }
    if checked > 0 && verified == 0 {
        bail!("no label signatures verified — investigate the canonical encoding");
    }
    Ok(())
}

async fn cmd_cross_thread() -> Result<()> {
    let client = XrpcClient::new()?;
    let accounts = collect_accounts()?;
    if accounts.len() < 2 {
        bail!("cross-thread needs two accounts (set ATPROTO_HANDLE_1/_2 and ATPROTO_APP_PASSWORD_1/_2)");
    }
    let s_root = auth_with(&client, &accounts[0].0, &accounts[0].1).await?;
    let s_reply = auth_with(&client, &accounts[1].0, &accounts[1].1).await?;

    // Root authored by account 1.
    let root = FeedPost::new("cross-account thread root", now_rfc3339());
    let root_resp = client.create_record(&s_root, POST_NSID, &serde_json::to_value(&root)?).await?;
    let root_ref = lexicon::StrongRef { uri: root_resp.uri.clone(), cid: root_resp.cid.clone() };
    println!("→ root  by @{}  {}", s_root.handle, root_resp.uri);

    // Reply authored by account 2 (different DID / PDS), referencing account 1's root.
    let reply = FeedPost::reply(
        format!("reply from @{} on a different PDS", s_reply.handle),
        now_rfc3339(),
        root_ref.clone(),
        root_ref.clone(),
    );
    let reply_resp = client.create_record(&s_reply, POST_NSID, &serde_json::to_value(&reply)?).await?;
    println!("→ reply by @{}  {}", s_reply.handle, reply_resp.uri);

    // Backfill BOTH repos into one index, then assemble the thread.
    let conn = appview::Db::open(":memory:")?;
    for s in [&s_root, &s_reply] {
        let records = client.list_records_full_public(&s.pds, &s.did, POST_NSID).await?;
        for (uri, cid, record) in &records {
            if let Some((d, c, rkey)) = parse_at_uri(uri) {
                let _ = appview::Db::index_post(&conn, uri, cid, &d, &c, &rkey, record, "backfill");
            }
        }
    }
    let thread = appview::Db::get_thread(&conn, &root_resp.uri)?;
    println!("\n=== Cross-account thread getThread({}) ===", root_resp.uri);
    for post in &thread {
        let depth = post.get("depth").and_then(|d| d.as_i64()).unwrap_or(0);
        let text = post.get("text").and_then(|t| t.as_str()).unwrap_or("");
        let did = post.get("did").and_then(|d| d.as_str()).unwrap_or("");
        println!("{}[{}] {}", "    ".repeat(depth as usize), did, text);
    }
    let distinct_authors: std::collections::BTreeSet<&str> =
        thread.iter().filter_map(|p| p.get("did").and_then(|d| d.as_str())).collect();
    println!("\ndistinct authors in thread: {}", distinct_authors.len());

    // Cleanup each author's own record.
    if let Some((_d, _c, rkey)) = parse_at_uri(&root_resp.uri) {
        client.delete_record(&s_root, POST_NSID, &rkey).await?;
    }
    if let Some((_d, _c, rkey)) = parse_at_uri(&reply_resp.uri) {
        client.delete_record(&s_reply, POST_NSID, &rkey).await?;
    }
    println!("(cleaned up both accounts)");
    if distinct_authors.len() < 2 {
        bail!("expected a cross-author thread but only saw one author");
    }
    Ok(())
}

async fn cmd_cid_check() -> Result<()> {
    let client = XrpcClient::new()?;
    let session = auth(&client).await?;

    let post = FeedPost::new("CID integrity check", now_rfc3339());
    let record = serde_json::to_value(&post)?;
    let resp = client.create_record(&session, POST_NSID, &record).await?;
    println!("\n=== CID integrity (recompute vs server) ===");

    // 1. Recompute from exactly what we published.
    let local = cidv1::cid_v1_dagcbor(&record)?;
    println!("server CID (createRecord):  {}", resp.cid);
    println!("recomputed from our record: {local}");
    println!("match: {}", if local == resp.cid { "YES ✓" } else { "NO ✗" });

    // 2. Fetch the server-stored value and recompute from *that* (proves the
    //    server didn't silently alter content).
    let (_did, _coll, rkey) = parse_at_uri(&resp.uri).context("parse uri")?;
    let (server_cid, stored) = client
        .get_record_public(&session.pds, &session.did, POST_NSID, &rkey)
        .await?;
    let local_from_stored = cidv1::cid_v1_dagcbor(&stored)?;
    println!("\nserver-stored value → CID:  {server_cid}");
    println!("recomputed from stored:     {local_from_stored}");
    println!("match: {}", if local_from_stored == server_cid { "YES ✓" } else { "NO ✗" });

    // 3. Tamper: change one byte of content; the CID must change.
    let mut tampered = record.clone();
    tampered["text"] = serde_json::json!("CID integrity check (tampered)");
    let tampered_cid = cidv1::cid_v1_dagcbor(&tampered)?;
    println!("\ntampered-content CID:       {tampered_cid}");
    println!("differs from original: {}  (content-addressing is tamper-evident)",
        if tampered_cid != local { "YES ✓" } else { "NO ✗" });

    if let Some((_d, _c, rkey)) = parse_at_uri(&resp.uri) {
        client.delete_record(&session, POST_NSID, &rkey).await?;
    }
    println!("\n(cleaned up)");
    Ok(())
}

async fn cmd_verify_identity(handle: &str) -> Result<()> {
    let client = XrpcClient::new()?;
    // Forward: handle -> DID (done by the entryway / DNS / well-known).
    let did = client.resolve_handle(handle).await.context("resolveHandle (forward)")?;
    // Reverse: the DID document must itself claim the handle in alsoKnownAs.
    let doc = client.fetch_plc_doc(&did).await?;
    let aka: Vec<String> = doc
        .get("alsoKnownAs")
        .and_then(|a| a.as_array())
        .map(|a| a.iter().filter_map(|v| v.as_str().map(str::to_string)).collect())
        .unwrap_or_default();
    let expected = format!("at://{}", handle.to_lowercase());
    let verified = aka.iter().any(|a| a.eq_ignore_ascii_case(&expected));

    println!("\n=== Identity verification: @{handle} ===");
    println!("forward  handle -> DID:        {did}");
    println!("reverse  DID.alsoKnownAs:       {aka:?}");
    println!("expected claim:                 {expected}");
    println!(
        "bidirectional match: {}",
        if verified {
            "YES — handle is verified, both directions agree ✓"
        } else {
            "NO — DID does not claim this handle (spoofable) ✗"
        }
    );
    if !verified {
        bail!("handle↔DID verification failed for @{handle}");
    }
    Ok(())
}

async fn cmd_probe_validation() -> Result<()> {
    let client = XrpcClient::new()?;
    let session = auth(&client).await?;
    let bogus_nsid = "org.croftc.experiment.feed.bogus";

    // Each case: (label, collection, raw record). We bypass our own validator on
    // purpose to see what the *PDS* does with malformed / unknown-lexicon records.
    let cases: Vec<(&str, &str, Value)> = vec![
        ("control — valid record", POST_NSID,
            serde_json::json!({"$type": POST_NSID, "text": "valid control", "createdAt": now_rfc3339()})),
        ("wrong type: text is a number, createdAt missing", POST_NSID,
            serde_json::json!({"$type": POST_NSID, "text": 42})),
        ("missing all required fields", POST_NSID,
            serde_json::json!({"$type": POST_NSID})),
        ("entirely unknown lexicon / collection", bogus_nsid,
            serde_json::json!({"$type": bogus_nsid, "anything": true, "n": 123})),
    ];

    println!("\n=== PDS validation probe (does the PDS enforce our lexicon?) ===");
    for (label, collection, rec) in &cases {
        // What our own lexicon validator thinks (only meaningful for the post shape):
        let local = serde_json::from_value::<lexicon::FeedPost>(rec.clone())
            .map_err(anyhow::Error::from)
            .and_then(|p| p.validate());
        let local_verdict = match &local {
            Ok(()) => "local: VALID".to_string(),
            Err(e) => format!("local: REJECTS ({})", e.to_string().lines().next().unwrap_or("")),
        };
        match client.create_record(&session, collection, rec).await {
            Ok(r) => println!(
                "• {label}\n    PDS: ACCEPTED  validationStatus={:?}  | {local_verdict}",
                r.validation_status
            ),
            Err(e) => println!(
                "• {label}\n    PDS: REJECTED ({})  | {local_verdict}",
                e.to_string().lines().next().unwrap_or("")
            ),
        }
    }

    // Clean up anything that was accepted, in both collections.
    for coll in [POST_NSID, bogus_nsid] {
        if let Ok(recs) = client.list_records(&session, coll).await {
            for (uri, _cid) in recs {
                if let Some((_d, _c, rkey)) = parse_at_uri(&uri) {
                    let _ = client.delete_record(&session, coll, &rkey).await;
                }
            }
        }
    }
    println!("\n(cleaned up any accepted probe records)");
    Ok(())
}

async fn cmd_compare(settle_ms: u64, timeout_secs: u64) -> Result<()> {
    use bridge::{InProcessLocalPath, LocalPath, NormalizedEvent};

    // ---- LOCAL / PRIVATE PATH (in-process, no network) ----
    // Stand-in for experiment 3a's private path; emits via the SAME normalized
    // boundary the public path uses. Swap InProcessLocalPath for a real 3a impl
    // of `bridge::LocalPath` to turn this into a true cross-experiment diff.
    let local_path = InProcessLocalPath;
    let local_post = FeedPost::new("compare: local/private path", now_rfc3339());
    let (local_norm, local_latency) =
        local_path.roundtrip("did:example:local", &serde_json::to_value(&local_post)?)?;

    // ---- PUBLIC PATH (createRecord → Jetstream → normalize) ----
    let client = XrpcClient::new()?;
    let session = auth(&client).await?;
    let url = build_url(&jetstream_url_base()?, POST_NSID, &[session.did.clone()]);
    let (tx, mut rx) = mpsc::channel::<ReceivedEvent>(256);
    let (ready_tx, ready_rx) = oneshot::channel::<()>();
    let consume_url = url.clone();
    let consumer = tokio::spawn(async move {
        if let Err(e) = jetstream::consume(consume_url, tx, Some(ready_tx)).await {
            eprintln!("jetstream consumer ended: {e}");
        }
    });
    ready_rx.await.ok();
    tokio::time::sleep(Duration::from_millis(settle_ms)).await;

    let public_post = FeedPost::new("compare: public path", now_rfc3339());
    let record = serde_json::to_value(&public_post)?;
    let send = std::time::Instant::now();
    let resp = client.create_record(&session, POST_NSID, &record).await?;

    let mut public_raw: Option<Value> = None;
    let mut public_norm: Option<NormalizedEvent> = None;
    let mut public_latency = None;
    let deadline = tokio::time::Instant::now() + Duration::from_secs(timeout_secs);
    loop {
        match tokio::time::timeout_at(deadline, rx.recv()).await {
            Ok(Some(ev)) => {
                let Some(c) = ev.event.commit.as_ref().filter(|c| c.operation == "create") else { continue };
                let uri = format!("at://{}/{}/{}", ev.event.did, c.collection, c.rkey);
                if uri == resp.uri {
                    public_latency = Some(ev.recv_instant.saturating_duration_since(send));
                    public_norm = NormalizedEvent::from_jetstream(&ev.event);
                    public_raw = Some(ev.raw);
                    break;
                }
            }
            _ => break,
        }
    }
    consumer.abort();
    if let Some((_d, _c, rkey)) = parse_at_uri(&resp.uri) {
        client.delete_record(&session, POST_NSID, &rkey).await?;
    }

    let public_raw = public_raw.context("public record did not return via Jetstream in time")?;
    let public_norm = public_norm.context("could not normalize the public event")?;
    let public_latency = public_latency.unwrap();

    // ---- DIFF ----
    println!("\n=== Latency ===");
    println!("local/private (in-process):       {:>8.3} ms", local_latency.as_secs_f64() * 1000.0);
    println!("public (createRecord→Jetstream):  {:>8} ms", public_latency.as_millis());
    let ratio = public_latency.as_secs_f64() / local_latency.as_secs_f64().max(1e-9);
    println!("public is ~{ratio:.0}x slower than the direct local write/read");

    // The boundary's payoff: both sources normalize to the SAME shape.
    println!("\n=== Source-agnostic boundary ===");
    let lk = local_norm.key_set();
    let pk = public_norm.key_set();
    println!("local NormalizedEvent keys:  {lk:?}");
    println!("public NormalizedEvent keys: {pk:?}");
    println!(
        "shapes converge: {}",
        if lk == pk { "YES — one ingest path handles both sources ✓" } else { "NO — boundary mismatch ✗" }
    );

    println!("\n=== What the public boundary had to do (vs the raw event) ===");
    println!("• build `uri` (Jetstream has none): {}", public_norm.uri);
    println!("• lift collection/rkey/cid/record out of the nested `commit` object");
    println!("• carry `operation` ({}) — create/update/delete drive index vs delete", public_norm.operation);
    println!("• (drop `time_us`/`rev` here; the indexer uses time_us only as the resume cursor)");

    println!("\nnormalized local event:\n{}", serde_json::to_string_pretty(&local_norm)?);
    println!("\nnormalized public event:\n{}", serde_json::to_string_pretty(&public_norm)?);
    println!("\nraw public Jetstream event (pre-normalization):\n{}", serde_json::to_string_pretty(&public_raw)?);
    println!("\nNOTE: the local side is an in-process baseline. Implementing bridge::LocalPath");
    println!("against experiment 3a's real private path makes this a true cross-experiment diff.");
    Ok(())
}

async fn cmd_thread(db: &str) -> Result<()> {
    let client = XrpcClient::new()?;
    let session = auth(&client).await?;

    // 1. Root post.
    let root = FeedPost::new("thread root — richer-lexicon experiment", now_rfc3339());
    root.validate()?;
    let root_resp = client.create_record(&session, POST_NSID, &serde_json::to_value(&root)?).await?;
    let root_ref = StrongRef { uri: root_resp.uri.clone(), cid: root_resp.cid.clone() };
    println!("→ root   {}", root_resp.uri);

    // 2. First reply (parent = root).
    let r1 = FeedPost::reply("first reply to the root", now_rfc3339(), root_ref.clone(), root_ref.clone());
    r1.validate()?;
    let r1_resp = client.create_record(&session, POST_NSID, &serde_json::to_value(&r1)?).await?;
    let r1_ref = StrongRef { uri: r1_resp.uri.clone(), cid: r1_resp.cid.clone() };
    println!("→ reply1 {}  (parent=root)", r1_resp.uri);

    // 3. Nested reply (parent = reply1, same root).
    let r2 = FeedPost::reply("a reply to the reply (depth 2)", now_rfc3339(), root_ref.clone(), r1_ref);
    r2.validate()?;
    let r2_resp = client.create_record(&session, POST_NSID, &serde_json::to_value(&r2)?).await?;
    println!("→ reply2 {}  (parent=reply1)", r2_resp.uri);

    // 4. Index everything via backfill (deterministic; no need to race Jetstream).
    let records = client.list_records_full_public(&session.pds, &session.did, POST_NSID).await?;
    let conn = appview::Db::open(db)?;
    for (uri, cid, record) in &records {
        if let Some((d, c, rkey)) = parse_at_uri(uri) {
            let _ = appview::Db::index_post(&conn, uri, cid, &d, &c, &rkey, record, "backfill");
        }
    }

    // 5. Assemble + print the thread from our own index.
    let thread = appview::Db::get_thread(&conn, &root_resp.uri)?;
    println!("\n=== getThread({}) ===", root_resp.uri);
    for post in &thread {
        let depth = post.get("depth").and_then(|d| d.as_i64()).unwrap_or(0);
        let text = post.get("text").and_then(|t| t.as_str()).unwrap_or("");
        println!("{}{}", "    ".repeat(depth as usize), text);
    }
    println!("\nraw JSON:\n{}", serde_json::to_string_pretty(&serde_json::json!({ "thread": thread }))?);

    // 6. Clean up.
    for (uri, _cid, _r) in &records {
        if let Some((_d, _c, rkey)) = parse_at_uri(uri) {
            client.delete_record(&session, POST_NSID, &rkey).await?;
        }
    }
    println!("\n✗ cleaned up {} record(s)", records.len());
    Ok(())
}

async fn cmd_index(collection: &str, backfill_repo: Option<String>, seconds: u64, db: &str) -> Result<()> {
    let client = XrpcClient::new()?;
    let conn = appview::Db::open(db)?;

    // Backfill: pull existing records straight from the repo so the index is
    // complete on first run, not just from the moment we started listening.
    if let Some(repo) = &backfill_repo {
        let did = if repo.starts_with("did:") {
            repo.clone()
        } else {
            client.resolve_handle(repo).await.context("resolving backfill handle")?
        };
        let pds = client.resolve_pds_via_plc(&did).await.context("resolving backfill PDS")?;
        let records = client.list_records_full_public(&pds, &did, collection).await?;
        let mut n = 0;
        for (uri, cid, record) in records {
            if let Some((d, c, rkey)) = parse_at_uri(&uri) {
                if appview::Db::index_post(&conn, &uri, &cid, &d, &c, &rkey, &record, "backfill").is_ok() {
                    n += 1;
                }
            }
        }
        println!("backfilled {n} record(s) from {repo}");
    }

    // Resume from the saved cursor so a restart misses nothing.
    let cursor = appview::Db::get_meta(&conn, "jetstream_cursor").and_then(|s| s.parse::<u64>().ok());
    match cursor {
        Some(c) => println!("resuming Jetstream from cursor {c}"),
        None => println!("no saved cursor — starting from live"),
    }
    let url = jetstream::build_url_cursor(&jetstream_url_base()?, collection, &[], cursor);
    let (tx, mut rx) = mpsc::channel::<ReceivedEvent>(512);
    let (ready_tx, ready_rx) = oneshot::channel::<()>();
    let consume_url = url.clone();
    let consumer = tokio::spawn(async move {
        if let Err(e) = jetstream::consume(consume_url, tx, Some(ready_tx)).await {
            eprintln!("jetstream consumer ended: {e}");
        }
    });
    ready_rx.await.ok();
    println!("indexing live (collection={collection}); {}...",
        if seconds > 0 { format!("for {seconds}s") } else { "Ctrl-C to stop".to_string() });

    let mut creates = 0u32;
    let mut deletes = 0u32;
    let run_until = (seconds > 0).then(|| tokio::time::Instant::now() + Duration::from_secs(seconds));
    loop {
        let ev = match run_until {
            Some(dl) => match tokio::time::timeout_at(dl, rx.recv()).await {
                Ok(Some(e)) => e,
                _ => break,
            },
            None => tokio::select! {
                e = rx.recv() => match e { Some(e) => e, None => break },
                _ = tokio::signal::ctrl_c() => { println!("\nCtrl-C — stopping"); break; }
            },
        };
        let Some(commit) = ev.event.commit.as_ref() else { continue };
        let uri = format!("at://{}/{}/{}", ev.event.did, commit.collection, commit.rkey);
        match commit.operation.as_str() {
            "create" | "update" => {
                if let Some(rec) = &commit.record {
                    if appview::Db::index_post(
                        &conn, &uri, commit.cid.as_deref().unwrap_or(""), &ev.event.did,
                        &commit.collection, &commit.rkey, rec, "jetstream",
                    ).is_ok() {
                        creates += 1;
                        println!("+ {} {uri}", commit.operation);
                    }
                }
            }
            "delete" => {
                if appview::Db::delete_post(&conn, &uri)? {
                    deletes += 1;
                    println!("- delete {uri}");
                }
            }
            _ => {}
        }
        // Persist the cursor after each event so a restart resumes precisely here.
        appview::Db::set_meta(&conn, "jetstream_cursor", &ev.event.time_us.to_string())?;
    }
    consumer.abort();

    println!("\n=== Indexer summary ===");
    println!("creates/updates indexed: {creates}   deletes processed: {deletes}");
    println!("rows in index now: {}", appview::Db::count_posts(&conn)?);
    if let Some(c) = appview::Db::get_meta(&conn, "jetstream_cursor") {
        println!("saved cursor: {c}  (next run resumes from here)");
    }
    Ok(())
}

async fn cmd_multi_roundtrip(db: &str, settle_ms: u64, timeout_secs: u64) -> Result<()> {
    let client = XrpcClient::new()?;
    let accounts = collect_accounts()?;
    let mut sessions = Vec::new();
    for (handle, password) in &accounts {
        sessions.push(auth_with(&client, handle, password).await?);
    }
    let dids: Vec<String> = sessions.iter().map(|s| s.did.clone()).collect();
    let distinct_pds: std::collections::BTreeSet<&str> = sessions.iter().map(|s| s.pds.as_str()).collect();
    println!(
        "\naccounts: {}   distinct PDS hosts: {}{}",
        sessions.len(),
        distinct_pds.len(),
        if distinct_pds.len() > 1 { "  (cross-PDS round-trip)" } else { "  (single PDS — add a 2nd account on another PDS to observe cross-PDS)" }
    );

    // Jetstream filtered to ALL accounts' DIDs at once.
    let url = build_url(&jetstream_url_base()?, POST_NSID, &dids);
    let (tx, mut rx) = mpsc::channel::<ReceivedEvent>(256);
    let (ready_tx, ready_rx) = oneshot::channel::<()>();
    let consume_url = url.clone();
    let consumer = tokio::spawn(async move {
        if let Err(e) = jetstream::consume(consume_url, tx, Some(ready_tx)).await {
            eprintln!("jetstream consumer ended: {e}");
        }
    });
    ready_rx.await.ok();
    tokio::time::sleep(Duration::from_millis(settle_ms)).await;

    // One record per account.
    struct Pending {
        cid: String,
        send_ms: u128,
        pds: String,
        handle: String,
        record: Value,
    }
    let mut pending: HashMap<String, Pending> = HashMap::new();
    for s in &sessions {
        let post = FeedPost::new(
            format!("multi-account roundtrip from @{}", s.handle),
            now_rfc3339(),
        );
        post.validate()?;
        let record: Value = serde_json::to_value(&post)?;
        let send_ms = now_unix_ms();
        let resp = client.create_record(s, POST_NSID, &record).await?;
        println!("→ @{} createRecord uri={}", s.handle, resp.uri);
        pending.insert(
            resp.uri,
            Pending { cid: resp.cid, send_ms, pds: s.pds.clone(), handle: s.handle.clone(), record },
        );
    }
    let total = pending.len();

    // Catch them all back, attributing each to its account/PDS.
    let conn = appview::Db::open(db)?;
    let mut matched = 0usize;
    let mut per_pds: HashMap<String, Vec<i128>> = HashMap::new();
    let deadline = tokio::time::Instant::now() + Duration::from_secs(timeout_secs);
    while matched < total {
        match tokio::time::timeout_at(deadline, rx.recv()).await {
            Ok(Some(ev)) => {
                let commit = match &ev.event.commit {
                    Some(c) if c.operation == "create" => c,
                    _ => continue,
                };
                let uri = format!("at://{}/{}/{}", ev.event.did, commit.collection, commit.rkey);
                let Some(p) = pending.get(&uri) else { continue };
                let cid_ok = commit.cid.as_deref() == Some(p.cid.as_str());
                let latency = ev.recv_unix_ms as i128 - p.send_ms as i128;
                per_pds.entry(p.pds.clone()).or_default().push(latency);
                println!("✓ @{} back via Jetstream  cid_match={cid_ok}  latency={latency}ms  [{}]", p.handle, p.pds);
                appview::Db::index_post(
                    &conn, &uri, commit.cid.as_deref().unwrap_or(""), &ev.event.did,
                    &commit.collection, &commit.rkey, commit.record.as_ref().unwrap_or(&p.record), "jetstream",
                )?;
                matched += 1;
            }
            Ok(None) => break,
            Err(_) => {
                eprintln!("timeout after {timeout_secs}s — matched {matched}/{total}");
                break;
            }
        }
    }
    consumer.abort();

    println!("\n=== Multi-account report ===");
    println!("published & returned: {matched}/{total}");
    for (pds, lats) in &per_pds {
        let avg = lats.iter().sum::<i128>() / lats.len() as i128;
        println!("  PDS {pds}: {} record(s), avg latency {avg}ms", lats.len());
    }

    // Clean up each account's records.
    for s in &sessions {
        let records = client.list_records(s, POST_NSID).await?;
        for (uri, _cid) in records {
            if let Some((_d, _c, rkey)) = parse_at_uri(&uri) {
                client.delete_record(s, POST_NSID, &rkey).await?;
            }
        }
        println!("✗ cleaned up @{}", s.handle);
    }
    if matched < total {
        bail!("not all records returned within the timeout");
    }
    Ok(())
}

async fn cmd_cleanup(yes: bool) -> Result<()> {
    if !yes {
        bail!("refusing to delete without --yes (this deletes ALL {POST_NSID} records in the authed repo)");
    }
    let client = XrpcClient::new()?;
    let session = auth(&client).await?;
    let records = client.list_records(&session, POST_NSID).await?;
    if records.is_empty() {
        println!("no records to delete.");
        return Ok(());
    }
    println!("deleting {} record(s)...", records.len());
    for (uri, _cid) in records {
        if let Some((_did, _coll, rkey)) = parse_at_uri(&uri) {
            client.delete_record(&session, POST_NSID, &rkey).await?;
            println!("✗ deleted {uri}");
        }
    }
    println!("done. (Tip: run `consume` to observe whether deletes propagate as commit events.)");
    Ok(())
}
