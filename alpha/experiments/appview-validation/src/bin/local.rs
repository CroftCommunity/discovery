//! Local proof harness for the four items the network phases left unproven:
//!   L1. custom-lexicon COMPREHENSION (typed, validated views — not raw storage)
//!   L2. PDS BACKFILL (bounded historical replay, not live tail)
//!   L3. MODERATION / LABELS (label-based filtering of the served view)
//!   L4. SCALE (batched indexing throughput vs row-by-row, and what it implies)
//!
//! Everything here is LOCAL and deterministic — no network, no credentials. It
//! reuses the same `Index` and the same `RecordSource` seam as the live phases:
//! a `FixtureSource` stands in for Jetstream, proving the machinery is genuinely
//! source-agnostic (swap the source, keep the pipeline).

use std::time::{Duration, Instant};

use anyhow::Result;
use serde_json::{json, Value};

use appview_validation::index::{Index, IndexStats};
use appview_validation::lexicon::{self, comprehend};
use appview_validation::record_source::{Action, ParseOutcome, RecordEvent, RecordSource};

/// A local, deterministic `RecordSource` — the "private stack behind the trait".
struct FixtureSource {
    events: Vec<RecordEvent>,
}

impl RecordSource for FixtureSource {
    async fn run(&mut self, mut on_outcome: impl FnMut(ParseOutcome) -> bool) -> Result<()> {
        for ev in &self.events {
            let raw = json!({
                "did": ev.did,
                "kind": "commit",
                "time_us": ev.cursor,
                "commit": {
                    "operation": ev.action.as_str(),
                    "collection": ev.collection,
                    "rkey": ev.rkey,
                    "record": ev.record,
                    "cid": ev.cid,
                },
            });
            if !on_outcome(ParseOutcome::Commit { event: ev.clone(), raw }) {
                break;
            }
        }
        Ok(())
    }
}

fn mk(did: &str, collection: &str, rkey: &str, record: Value, cursor: i64) -> RecordEvent {
    RecordEvent {
        action: Action::Create,
        did: did.to_string(),
        collection: collection.to_string(),
        rkey: rkey.to_string(),
        record: Some(record),
        cid: Some(format!("bafyfixture{rkey}")),
        cursor,
    }
}

/// Drive a FixtureSource through the real index pipeline (one-by-one apply).
async fn ingest(index: &mut Index, stats: &mut IndexStats, events: Vec<RecordEvent>) -> Result<()> {
    let mut src = FixtureSource { events };
    // Borrow split: collect events to apply, then apply outside the closure isn't
    // needed — apply directly in the callback like the live phases do.
    src.run(|outcome| {
        if let ParseOutcome::Commit { event, .. } = outcome {
            let _ = index.apply(&event, stats);
        }
        true
    })
    .await
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut index = Index::open("local.sqlite")?;
    let mut stats = IndexStats::default();

    // ───────────────── L1: custom-lexicon COMPREHENSION ─────────────────
    println!("\n############ L1: custom-lexicon comprehension (typed views) ############");
    let nsid = lexicon::NSID;
    let now = "2026-06-13T12:00:00.000Z";
    let notes = vec![
        mk("did:plc:fix", nsid, "ok1", json!({"$type": nsid, "note": "valid note", "n": 7, "createdAt": now}), 1),
        mk("did:plc:fix", nsid, "ok2", json!({"$type": nsid, "note": "another", "n": 42, "createdAt": now}), 2),
        mk("did:plc:fix", nsid, "bad1", json!({"$type": nsid, "note": "missing n", "createdAt": now}), 3),
        mk("did:plc:fix", nsid, "bad2", json!({"$type": nsid, "note": "wrong type", "n": "NaN", "createdAt": now}), 4),
        mk("did:plc:fix", nsid, "bad3", json!({"$type": "app.bsky.feed.post", "text": "wrong type entirely", "createdAt": now}), 5),
    ];
    ingest(&mut index, &mut stats, notes).await?;
    println!("  stored {} raw records under {nsid}; now COMPREHENDING (typed + validated):", index.count_for(nsid)?);
    let (mut ok, mut rejected) = (0, 0);
    for (uri, json_str) in index.records_json(nsid)? {
        let rkey = uri.rsplit('/').next().unwrap_or("");
        match comprehend(&json_str) {
            Ok(note) => {
                ok += 1;
                println!("    ✓ [{rkey}] typed: note={:?} n={} createdAt={}", note.note, note.n, note.created_at);
            }
            Err(why) => {
                rejected += 1;
                println!("    ✗ [{rkey}] REJECTED: {why}");
            }
        }
    }
    println!("  => comprehension result: {ok} valid typed views, {rejected} rejected as non-conforming.");
    println!("     (raw storage accepted all 5; comprehension is what enforces the schema.)");

    // ───────────────── L2: PDS BACKFILL ─────────────────
    println!("\n############ L2: PDS backfill (bounded historical replay) ############");
    // A simulated repo page: older posts (small cursors), as listRecords would return.
    let mut backfill = Vec::new();
    for i in 0..25 {
        let rkey = format!("hist{i:03}");
        backfill.push(mk(
            "did:plc:hist",
            "app.bsky.feed.post",
            &rkey,
            json!({"$type": "app.bsky.feed.post", "text": format!("historical post #{i}"), "createdAt": now}),
            1_000 + i, // historical cursors, all "in the past"
        ));
    }
    let before = index.count_for("app.bsky.feed.post")?;
    ingest(&mut index, &mut stats, backfill).await?;
    let after = index.count_for("app.bsky.feed.post")?;
    println!("  backfilled {} historical posts via the SAME pipeline (FixtureSource swapped in for Jetstream).", after - before);
    println!("  posts in index now: {after}");
    println!("  => backfill = a bounded, non-live source feeding the same RecordSource seam.");
    println!("     Real-world equivalent: com.atproto.repo.listRecords paginated by cursor");
    println!("     (or a CAR repo export) instead of the live firehose tail.");

    // ───────────────── L3: MODERATION / LABELS ─────────────────
    println!("\n############ L3: moderation / labels ############");
    index.add_label("at://did:plc:hist/app.bsky.feed.post/hist000", "spam", "did:plc:labeler")?;
    index.add_label("at://did:plc:hist/app.bsky.feed.post/hist001", "!hide", "did:plc:labeler")?;
    let unmoderated = index.recent_posts(false, 100)?;
    let moderated = index.recent_posts(true, 100)?;
    println!("  applied 2 labels (spam, !hide) from a labeler.");
    println!("  served view WITHOUT moderation: {} posts", unmoderated.len());
    println!("  served view WITH moderation   : {} posts ({} hidden)", moderated.len(), unmoderated.len() - moderated.len());
    println!("  => the moderated XRPC view excludes labeled records; labels are a side");
    println!("     table joined at read time, so re-labeling needs no re-index.");

    // ───────────────── L4: SCALE ─────────────────
    println!("\n############ L4: scale (batched vs row-by-row throughput) ############");
    let n_rowwise = 500usize;
    let n_batch = 50_000usize;

    // Row-by-row, autocommit (one fsync per insert) — the naive path.
    let mut idx_rw = Index::open("local-scale-rw.sqlite")?;
    let mut s = IndexStats::default();
    let rw_events = gen_posts("did:plc:rw", n_rowwise);
    let t = Instant::now();
    for ev in &rw_events {
        idx_rw.apply(ev, &mut s)?;
    }
    let rw = t.elapsed();
    println!("  row-by-row (autocommit): {n_rowwise} rows in {rw:?} = {:.0} rows/s", rate(n_rowwise, rw));

    // Batched in a single transaction — the path a real ingester needs.
    let mut idx_b = Index::open("local-scale-batch.sqlite")?;
    let batch_events = gen_posts("did:plc:batch", n_batch);
    let t = Instant::now();
    idx_b.apply_batch(&batch_events)?;
    let b = t.elapsed();
    println!("  batched (1 txn)        : {n_batch} rows in {b:?} = {:.0} rows/s", rate(n_batch, b));
    let speedup = rate(n_batch, b) / rate(n_rowwise, rw).max(1.0);
    println!("  => batching is ~{speedup:.0}x faster per row. The live firehose ran ~185 ev/s");
    println!("     (likes) earlier; at full-network scale an ingester MUST batch writes and");
    println!("     absorb bursts (buffer/backpressure) rather than fsync per event.");

    println!("\n############ DONE (local proof) ############");
    Ok(())
}

fn gen_posts(did: &str, n: usize) -> Vec<RecordEvent> {
    (0..n)
        .map(|i| {
            mk(
                did,
                "app.bsky.feed.post",
                &format!("scale{i:06}"),
                json!({"$type": "app.bsky.feed.post", "text": format!("scale post {i}"), "createdAt": "2026-06-13T12:00:00.000Z"}),
                i as i64,
            )
        })
        .collect()
}

fn rate(n: usize, d: Duration) -> f64 {
    n as f64 / d.as_secs_f64().max(1e-9)
}
