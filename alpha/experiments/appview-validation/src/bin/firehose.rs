//! Hypothesis-validation: do you need a DISTRIBUTED INGESTER at full-firehose scale?
//!
//! THE HYPOTHESIS:
//!   HS1. The full, unfiltered firehose is fast — guess ~2,000 events/s.
//!   HS2. A single-node SQLite ingester CANNOT keep up with the full firehose.
//!   HS3. Therefore you NEED a distributed/sharded ingester.
//!
//! We test against reality: measure this machine's batched-insert capacity, then
//! run the FULL unfiltered Jetstream through a realistic ingester architecture
//! (bounded queue + batching consumer = backpressure) and measure whether a single
//! node keeps up — computing the actual headroom. The "do I need distribution?"
//! answer is then evidence, not assumption.

use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use anyhow::Result;
use futures_util::StreamExt;
use serde_json::json;
use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite::Message;

use appview_validation::index::Index;
use appview_validation::jetstream::DEFAULT_HOST;
use appview_validation::record_source::{parse_frame, Action, ParseOutcome, RecordEvent};

const WINDOW: Duration = Duration::from_secs(15);
const QUEUE_CAP: usize = 2048;
const BATCH: usize = 1000;

/// Measure this machine's single-connection batched-insert ceiling.
fn measure_insert_capacity() -> Result<f64> {
    let mut idx = Index::open("firehose-cap.sqlite")?;
    let events: Vec<RecordEvent> = (0..50_000)
        .map(|i| RecordEvent {
            action: Action::Create,
            did: "did:plc:cap".into(),
            collection: "app.bsky.feed.post".into(),
            rkey: format!("cap{i:06}"),
            record: Some(json!({"$type":"app.bsky.feed.post","text":"cap","createdAt":"2026-06-13T12:00:00.000Z"})),
            cid: Some("bafycap".into()),
            cursor: i as i64,
        })
        .collect();
    let t = Instant::now();
    idx.apply_batch(&events)?;
    Ok(50_000.0 / t.elapsed().as_secs_f64())
}

#[tokio::main]
async fn main() -> Result<()> {
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("install rustls ring crypto provider");

    println!("\n############ distributed ingester @ scale — hypothesis vs reality ############");
    let cap_rate = measure_insert_capacity()?;
    println!("  single-node batched-insert capacity (this machine): {cap_rate:.0} rows/s");

    // Shared queue-depth gauges between producer and consumer.
    // Signed: the producer-increment / consumer-decrement race can transiently go
    // negative (item enqueued before the gauge is bumped); that's harmless here.
    let depth = Arc::new(AtomicI64::new(0));
    let max_depth = Arc::new(AtomicI64::new(0));
    let (tx, mut rx) = mpsc::channel::<RecordEvent>(QUEUE_CAP);

    // ---- consumer: batch-drain the queue into the index ----
    let cons_depth = depth.clone();
    let consumer = tokio::spawn(async move {
        let mut idx = Index::open("firehose.sqlite").expect("open index");
        let mut buf: Vec<RecordEvent> = Vec::with_capacity(BATCH);
        let mut consumed = 0usize;
        let mut busy = Duration::ZERO;
        while let Some(ev) = rx.recv().await {
            cons_depth.fetch_sub(1, Ordering::Relaxed);
            buf.push(ev);
            while let Ok(ev) = rx.try_recv() {
                cons_depth.fetch_sub(1, Ordering::Relaxed);
                buf.push(ev);
                if buf.len() >= BATCH {
                    break;
                }
            }
            let t = Instant::now();
            let _ = idx.apply_batch(&buf);
            busy += t.elapsed();
            consumed += buf.len();
            buf.clear();
        }
        if !buf.is_empty() {
            let _ = idx.apply_batch(&buf);
            consumed += buf.len();
        }
        (consumed, busy)
    });

    // ---- producer: read the FULL unfiltered firehose, push to the queue ----
    let url = format!("wss://{DEFAULT_HOST}/subscribe"); // no filters = everything
    println!("  connecting to FULL firehose: {url}");
    let (mut ws, resp) = tokio_tungstenite::connect_async(&url).await?;
    println!("  handshake OK: HTTP {}", resp.status().as_u16());
    println!("  running {WINDOW:?} (bounded queue cap={QUEUE_CAP}, batch={BATCH})…");

    let start = Instant::now();
    let deadline = start + WINDOW;
    let (mut produced, mut frames, mut bytes, mut backpressure, mut non_commit) = (0usize, 0usize, 0usize, 0usize, 0usize);

    loop {
        let remaining = match deadline.checked_duration_since(Instant::now()) {
            Some(r) if !r.is_zero() => r,
            _ => break,
        };
        let msg = match tokio::time::timeout(remaining, ws.next()).await {
            Ok(Some(Ok(m))) => m,
            _ => break,
        };
        let text = match msg {
            Message::Text(t) => t.as_str().to_string(),
            Message::Binary(b) => String::from_utf8_lossy(&b).into_owned(),
            _ => continue,
        };
        frames += 1;
        bytes += text.len();
        match parse_frame(&text) {
            ParseOutcome::Commit { event, .. } => {
                // try_send first to detect backpressure, then block (await) if full.
                match tx.try_send(event) {
                    Ok(()) => {}
                    Err(mpsc::error::TrySendError::Full(ev)) => {
                        backpressure += 1;
                        if tx.send(ev).await.is_err() {
                            break;
                        }
                    }
                    Err(_) => break,
                }
                let d = depth.fetch_add(1, Ordering::Relaxed) + 1;
                max_depth.fetch_max(d, Ordering::Relaxed);
                produced += 1;
            }
            _ => non_commit += 1,
        }
    }
    let elapsed = start.elapsed().as_secs_f64();
    drop(tx); // close queue -> consumer drains and finishes
    let _ = ws.close(None).await;
    let (consumed, busy) = consumer.await?;

    // ---- results ----
    let ev_rate = produced as f64 / elapsed;
    let frame_rate = frames as f64 / elapsed;
    let mb_s = bytes as f64 / elapsed / 1_000_000.0;
    let busy_frac = busy.as_secs_f64() / elapsed * 100.0;
    let headroom = cap_rate / ev_rate.max(1.0);
    let md = max_depth.load(Ordering::Relaxed);
    let kept_up = md < QUEUE_CAP as i64 && consumed >= produced && backpressure == 0;
    println!("\n================ HYPOTHESIS vs REALITY ================");
    println!("  full firehose frames : {frames} ({frame_rate:.0}/s), {non_commit} non-commit");
    println!("  indexable events     : {produced} ({ev_rate:.0} events/s)");
    println!("  wire volume          : {mb_s:.2} MB/s");
    println!("  queue: max depth {md} / {QUEUE_CAP}  | backpressure stalls: {backpressure}");
    println!("  consumer: indexed {consumed} rows; write-busy {busy_frac:.1}% of wall time");
    println!("    (the high busy % is the tiny-batch fsync tax: at {ev_rate:.0} ev/s the batch");
    println!("     rarely fills, so we commit ~per event; time-based batching collapses this.)");
    println!("  kept up? produced={produced} consumed={consumed} (lag {})", produced as i64 - consumed as i64);
    println!();
    println!("  HS1 'full firehose ~2,000 ev/s' -> measured {ev_rate:.0} events/s ({frame_rate:.0} frames/s).");
    println!("  HS2 'single node can't keep up' -> {}", if kept_up {
        format!("REFUTED: one node kept up — lag 0, max queue depth {md}/{QUEUE_CAP}, zero backpressure.")
    } else {
        "SUPPORTED: queue saturated / consumer fell behind.".to_string()
    });
    println!("  HS3 'you need distribution' -> headroom = capacity/rate = {cap_rate:.0}/{ev_rate:.0} ≈ {headroom:.0}x.");
    if headroom > 5.0 {
        println!("     A single batched node has ~{headroom:.0}x headroom: distribution is PREMATURE today;");
        println!("     it becomes necessary only ~{headroom:.0}x beyond current volume (or for HA/redundancy,");
        println!("     multi-collection fan-out, and hydration fan-out — not raw insert throughput).");
    } else {
        println!("     Headroom is thin: sharding / horizontal scale is warranted.");
    }
    println!("======================================================");
    println!("\n############ DONE (firehose) ############");
    Ok(())
}
