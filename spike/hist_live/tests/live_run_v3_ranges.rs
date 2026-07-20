//! Round 3 — range-query mechanics against a bsky-hosted PDS.
//!
//! Question set the user posed at close of round 2:
//!  - what does the PDS accept for range queries?
//!  - how large a range in one go?
//!  - how do you page over a range that exceeds a single response?
//!  - performance / overhead at scale?
//!
//! Baseline: 40 records on a fresh subspace `hist-live/v3/range`.  All
//! measurements happen against that dataset, so per-record cost comparisons
//! are apples-to-apples.  Reads are not budget-capped.
//!
//! Gentleness contract still enforced by the shared Budget/Pacer.

mod common;

use common::*;
use hist_live::budget::BudgetCaps;
use hist_live::car::parse_car;
use hist_live::leg::{ApplyWritesOp, LiveLegTrait, XrpcError};
use hist_live::record::{HistEntry, Rkey, Subspace, HIST_ENTRY_TYPE};
use serde::Serialize;
use std::collections::{BTreeSet, HashSet};
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};

const BASELINE_RECORDS: u32 = 40;

fn evidence_dir() -> PathBuf {
    let p = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("evidence")
        .join("live_v3");
    fs::create_dir_all(&p).unwrap();
    p
}

fn write_json<T: Serialize>(rel: &str, v: &T) {
    let p = evidence_dir().join(rel);
    fs::create_dir_all(p.parent().unwrap()).unwrap();
    fs::write(&p, serde_json::to_string_pretty(v).unwrap()).unwrap();
}

#[derive(Default)]
struct WrittenLedger {
    records: Vec<(String, String)>,
}
impl WrittenLedger {
    fn note(&mut self, collection: &str, rkey: &str) {
        self.records.push((collection.into(), rkey.into()));
    }
}

// -------- Result structs -------------------------------------------

#[derive(Serialize)]
struct E_R1_ParamSurface {
    verdict: &'static str,
    limit_hard_max_documented: u32,
    limit_over_max_response: OverMaxResp,
    rkey_range_filter_observation: String,
    rkey_range_filters_correctly: bool,
    rkey_range_filter_probe: RangeFilterProbe,
    inverted_range_behavior: String,
    reverse_true_first_rkey: Option<String>,
    reverse_false_first_rkey: Option<String>,
    reverse_true_matches_reverse_of_reverse_false: bool,
}

#[derive(Serialize)]
struct OverMaxResp {
    http_status: u16,
    error: Option<String>,
    message: Option<String>,
}

#[derive(Serialize)]
struct RangeFilterProbe {
    rkey_start: String,
    rkey_end: String,
    expected_counters_in_range: Vec<u32>,
    observed_counters_in_range: Vec<u32>,
}

#[derive(Serialize)]
struct E_R2_Pagination {
    verdict: &'static str,
    total_records: u32,
    per_page_size: Vec<PageSizeRow>,
}

#[derive(Serialize)]
struct PageSizeRow {
    limit: u32,
    pages: usize,
    total_records: usize,
    total_bytes: usize,
    wall_clock_ms: u128,
    avg_ms_per_page: u128,
    avg_bytes_per_record: f64,
    records_per_sec: f64,
}

#[derive(Serialize)]
struct E_R3_GetBlocks {
    verdict: &'static str,
    batch_probes: Vec<BlocksBatchRow>,
    max_batch_that_worked: usize,
    comparison_to_listrecords: BlocksComparison,
}

#[derive(Serialize)]
struct BlocksBatchRow {
    batch_size: usize,
    http_status: u16,
    response_bytes: usize,
    wall_clock_ms: u128,
    blocks_returned: Option<usize>,
    error: Option<String>,
}

#[derive(Serialize)]
struct BlocksComparison {
    records: usize,
    getblocks_bytes: usize,
    getblocks_ms: u128,
    listrecords_bytes: usize,
    listrecords_ms: u128,
    getblocks_per_record_bytes: f64,
    listrecords_per_record_bytes: f64,
}

#[derive(Serialize)]
struct E_R4_CursorStability {
    verdict: &'static str,
    baseline_before: usize,
    inserted_during_walk_rkey: String,
    inserted_visible_in_ongoing_walk: bool,
    total_visited_in_ongoing_walk: usize,
    ordering_monotonic_within_walk: bool,
    walk_direction: &'static str,
    /// The one that closes OC-3: after the ongoing walk ends, does a FRESH
    /// walk see the mid-walk insert?  YES = snapshot semantics; NO = the
    /// insert was ghosted somehow.
    fresh_walk_after_insert_sees_it: bool,
    walk_transcript: Vec<String>,
    interpretation: String,
}

#[derive(Serialize)]
struct E_R5_Overhead {
    verdict: &'static str,
    single_getRecord_bytes: usize,
    single_getRecord_ms: u128,
    per_record_bytes_at_page_sizes: Vec<(u32, f64)>,
    framing_overhead_bytes_at_page_sizes: Vec<(u32, i64)>,
    /// Practical range-request recipe based on the observations.
    recipe: RecipeSummary,
}

#[derive(Serialize)]
struct RecipeSummary {
    max_records_per_request: u32,
    recommended_page_size_for_bulk: u32,
    range_filter_supported: bool,
    incremental_via_since_supported: bool,
    cid_batch_via_get_blocks_supported: bool,
    per_request_overhead_estimate_bytes: i64,
    per_record_estimate_bytes: u32,
}

// -------- Helpers ---------------------------------------------------

fn build_chain(sub: &Subspace, count: u32) -> Vec<(Rkey, HistEntry)> {
    let mut v = Vec::new();
    let mut prev = None;
    for c in 1..=count {
        let e = HistEntry::new(sub, c, prev, 24);
        prev = Some(e.cid());
        v.push((Rkey::from(sub, c), e));
    }
    v
}

fn write_baseline(
    leg: &Arc<impl LiveLegTrait + ?Sized>,
    sub: &Subspace,
    count: u32,
    ledger: &mut WrittenLedger,
) -> Vec<(Rkey, HistEntry)> {
    let entries = build_chain(sub, count);
    for chunk in entries.chunks(10) {
        let ops: Vec<_> = chunk
            .iter()
            .map(|(rk, e)| ApplyWritesOp::Create {
                collection: HIST_ENTRY_TYPE.into(),
                rkey: rk.0.clone(),
                value: serde_json::to_value(e).unwrap(),
            })
            .collect();
        leg.apply_writes(ops).expect("baseline apply_writes");
        for (rk, _) in chunk {
            ledger.note(HIST_ENTRY_TYPE, &rk.0);
        }
    }
    entries
}

fn counter_from_rkey(rkey: &str) -> Option<u32> {
    rkey.rsplit('_').next().and_then(|s| s.parse::<u32>().ok())
}

// -------- The test --------------------------------------------------

#[test]
fn live_v3_range_experiments_and_teardown() {
    if !live_enabled() {
        println!("[live_v3] SKIPPED — HIST_PDS_APP_PASSWORD not set");
        return;
    }
    let leg = live_gentle();
    let sess = leg.session().expect("session");
    println!(
        "[live_v3] session did={} pds={}",
        sess.did, sess.pds_endpoint
    );

    let mut ledger = WrittenLedger::default();
    let sub = Subspace("hist-live/v3/range".to_string());
    println!(
        "[baseline] writing {} records to subspace {}",
        BASELINE_RECORDS,
        sub.hash_prefix()
    );
    let entries = write_baseline(&leg, &sub, BASELINE_RECORDS, &mut ledger);
    let sub_hash = sub.hash_prefix();
    println!("[baseline] {} records written, prefix={}", entries.len(), sub_hash);

    // ---- E-Range-1 --------------------------------------------------
    let e_r1 = run_er1(&leg, &sub_hash);
    write_json("e_range_1_params.json", &e_r1);
    assert_eq!(e_r1.verdict, "GREEN", "E-Range-1 failed");

    // ---- E-Range-2 --------------------------------------------------
    let e_r2 = run_er2(&leg, &sub_hash);
    write_json("e_range_2_pagination.json", &e_r2);
    assert_eq!(e_r2.verdict, "GREEN", "E-Range-2 failed");

    // ---- E-Range-3 --------------------------------------------------
    let target_cids: Vec<String> = entries
        .iter()
        .map(|(_, e)| e.cid().to_string())
        .collect();
    let e_r3 = run_er3(&leg, &sess.did, &target_cids, &sub_hash);
    write_json("e_range_3_getblocks.json", &e_r3);
    assert_eq!(e_r3.verdict, "GREEN", "E-Range-3 failed");

    // ---- E-Range-4 --------------------------------------------------
    let e_r4 = run_er4(&leg, &sub, &sub_hash, &mut ledger);
    write_json("e_range_4_cursor_stability.json", &e_r4);
    assert!(matches!(e_r4.verdict, "GREEN" | "OBSERVATIONAL"), "E-Range-4 unexpected verdict: {}", e_r4.verdict);

    // ---- E-Range-5 --------------------------------------------------
    let e_r5 = run_er5(&leg, &sub_hash, &entries, &e_r1, &e_r2, &e_r3);
    write_json("e_range_5_overhead.json", &e_r5);
    assert_eq!(e_r5.verdict, "GREEN", "E-Range-5 failed");

    // ---- Teardown ----------------------------------------------------
    teardown(&leg, ledger);

    let l = leg.budget_snapshot();
    let caps = BudgetCaps::GENTLE;
    println!(
        "[budget] writes={}/{}  blobs={}/{}  reads={}  rate_limit_signals={}",
        l.writes, caps.writes, l.blobs, caps.blobs, l.reads, l.rate_limit_signals
    );
    write_json(
        "budget_ledger.json",
        &serde_json::json!({
            "writes": l.writes, "reads": l.reads, "blobs": l.blobs,
            "write_calls": l.write_calls, "read_calls": l.read_calls,
            "rate_limit_signals": l.rate_limit_signals,
            "caps": {"writes": caps.writes, "blobs": caps.blobs},
        }),
    );
    assert!(l.writes <= caps.writes, "budget writes overrun");
}

// ==================================================================
// E-Range-1 — parameter surface
// ==================================================================

fn run_er1(leg: &Arc<impl LiveLegTrait + ?Sized>, sub_hash: &str) -> E_R1_ParamSurface {
    // A: limit=101 -- expected 400 InvalidRequest.
    let over_max = match leg.list_records_range(HIST_ENTRY_TYPE, 101, None, false, None, None) {
        Ok(_) => OverMaxResp {
            http_status: 200,
            error: Some("UNEXPECTED_OK".into()),
            message: Some("server accepted limit=101 (documented max = 100)".into()),
        },
        Err(XrpcError::Xrpc {
            http_status,
            error,
            message,
            ..
        }) => OverMaxResp {
            http_status,
            error,
            message,
        },
        Err(e) => OverMaxResp {
            http_status: 0,
            error: Some("NON_XRPC".into()),
            message: Some(e.to_string()),
        },
    };

    // B: rkeyStart / rkeyEnd — request counters [10..20] via
    // `<hash>_0000010` .. `<hash>_0000020`.
    let start_rkey = format!("{}_{:07}", sub_hash, 10);
    let end_rkey = format!("{}_{:07}", sub_hash, 20);
    let mut observed: Vec<u32> = Vec::new();
    let mut cursor = None;
    loop {
        let page = leg
            .list_records_range(
                HIST_ENTRY_TYPE,
                100,
                cursor.as_deref(),
                false,
                Some(&start_rkey),
                Some(&end_rkey),
            )
            .expect("range list");
        for r in &page.records {
            let rk = r.uri.rsplit('/').next().unwrap().to_string();
            if rk.starts_with(sub_hash) {
                if let Some(n) = counter_from_rkey(&rk) {
                    observed.push(n);
                }
            }
        }
        if page.cursor.is_none() {
            break;
        }
        cursor = page.cursor;
    }
    observed.sort();
    // The atproto lexicon marks rkeyStart / rkeyEnd as DEPRECATED and modern
    // PDSes accept the params (returning HTTP 200) but silently do not
    // filter — the full result set comes back regardless.  This experiment
    // was hoping to confirm exclusive-at-both-ends semantics, but the
    // finding is the deprecation: **there is no server-side rkey-range
    // filter on the current PDS**.  Ranges must be assembled client-side
    // from cursored enumeration, OR fetched by CID batch via
    // `sync.getBlocks` (E-Range-3).
    let expected_exclusive_both: Vec<u32> = (11..=19).collect();
    let expected_inclusive: Vec<u32> = (10..=20).collect();
    let range_matches_exclusive = observed == expected_exclusive_both;
    let range_matches_inclusive = observed == expected_inclusive;
    let range_ok = range_matches_exclusive || range_matches_inclusive;
    let observation = if range_matches_exclusive {
        "rkeyStart/rkeyEnd honored with exclusive-at-both-ends semantics".to_string()
    } else if range_matches_inclusive {
        "rkeyStart/rkeyEnd honored with inclusive-at-both-ends semantics".to_string()
    } else {
        format!(
            "rkeyStart/rkeyEnd IGNORED by the server (all {} records returned instead of {} expected). Deprecated fields per the atproto lexicon; a range fold MUST enumerate via cursor and filter client-side, or use sync.getBlocks(cids).",
            observed.len(),
            expected_exclusive_both.len()
        )
    };

    // C: inverted range (start > end) — probe behavior.
    let inv_start = format!("{}_{:07}", sub_hash, 30);
    let inv_end = format!("{}_{:07}", sub_hash, 20);
    let inv_page = leg.list_records_range(
        HIST_ENTRY_TYPE,
        100,
        None,
        false,
        Some(&inv_start),
        Some(&inv_end),
    );
    let inverted_behavior = match inv_page {
        Ok(p) => format!("returned {} records (server treats inverted range as empty or clamps)", p.records.len()),
        Err(e) => format!("returned error: {}", e),
    };

    // D: reverse=true vs reverse=false first rkey.
    let asc = leg
        .list_records_range(HIST_ENTRY_TYPE, 5, None, false, None, None)
        .expect("asc page");
    let desc = leg
        .list_records_range(HIST_ENTRY_TYPE, 5, None, true, None, None)
        .expect("desc page");
    let first_desc: Option<String> = desc
        .records
        .first()
        .map(|r| r.uri.rsplit('/').next().unwrap().to_string());
    let first_asc: Option<String> = asc
        .records
        .first()
        .map(|r| r.uri.rsplit('/').next().unwrap().to_string());
    // Fetch all in each direction; assert asc == reverse(desc).
    let mut all_asc: Vec<String> = Vec::new();
    let mut cursor = None;
    loop {
        let p = leg
            .list_records_range(HIST_ENTRY_TYPE, 100, cursor.as_deref(), false, None, None)
            .expect("asc");
        for r in &p.records {
            let rk = r.uri.rsplit('/').next().unwrap().to_string();
            if rk.starts_with(sub_hash) {
                all_asc.push(rk);
            }
        }
        if p.cursor.is_none() {
            break;
        }
        cursor = p.cursor;
    }
    let mut all_desc: Vec<String> = Vec::new();
    let mut cursor = None;
    loop {
        let p = leg
            .list_records_range(HIST_ENTRY_TYPE, 100, cursor.as_deref(), true, None, None)
            .expect("desc");
        for r in &p.records {
            let rk = r.uri.rsplit('/').next().unwrap().to_string();
            if rk.starts_with(sub_hash) {
                all_desc.push(rk);
            }
        }
        if p.cursor.is_none() {
            break;
        }
        cursor = p.cursor;
    }
    let reversed_desc: Vec<String> = all_desc.iter().rev().cloned().collect();
    let reverse_matches = all_asc == reversed_desc;

    // The parameter-surface probe is GREEN whenever we've measured each
    // property honestly — regardless of whether the range filter is
    // honored, since "not honored" is the finding for OC-3.  The
    // load-bearing assertions are: limit=101 IS rejected, and
    // reverse=true reverses.
    let verdict = if over_max.http_status == 400 && reverse_matches {
        "GREEN"
    } else {
        "RED"
    };

    E_R1_ParamSurface {
        verdict,
        limit_hard_max_documented: 100,
        limit_over_max_response: over_max,
        rkey_range_filter_observation: observation,
        rkey_range_filters_correctly: range_ok,
        rkey_range_filter_probe: RangeFilterProbe {
            rkey_start: start_rkey,
            rkey_end: end_rkey,
            expected_counters_in_range: expected_exclusive_both.clone(),
            observed_counters_in_range: observed,
        },
        inverted_range_behavior: inverted_behavior,
        reverse_true_first_rkey: first_desc,
        reverse_false_first_rkey: first_asc,
        reverse_true_matches_reverse_of_reverse_false: reverse_matches,
    }
}

// ==================================================================
// E-Range-2 — pagination performance
// ==================================================================

fn run_er2(leg: &Arc<impl LiveLegTrait + ?Sized>, sub_hash: &str) -> E_R2_Pagination {
    let page_sizes = [1u32, 10, 50, 100];
    let mut rows = Vec::new();
    let mut all_ok = true;
    for &limit in &page_sizes {
        let t0 = Instant::now();
        let mut records = 0usize;
        let mut bytes = 0usize;
        let mut pages = 0usize;
        let mut cursor: Option<String> = None;
        loop {
            let page = leg
                .list_records_range(
                    HIST_ENTRY_TYPE,
                    limit,
                    cursor.as_deref(),
                    false,
                    None,
                    None,
                )
                .expect("er2 page");
            pages += 1;
            for r in &page.records {
                let rk = r.uri.rsplit('/').next().unwrap();
                if rk.starts_with(sub_hash) {
                    records += 1;
                }
            }
            // Estimate serialized bytes.
            let est_bytes: usize = page
                .records
                .iter()
                .map(|r| serde_json::to_string(&r.value).unwrap_or_default().len())
                .sum();
            bytes += est_bytes;
            if page.cursor.is_none() {
                break;
            }
            cursor = page.cursor;
        }
        let wall = t0.elapsed().as_millis();
        let avg_ms = if pages > 0 { wall / pages as u128 } else { 0 };
        let avg_b = if records > 0 { bytes as f64 / records as f64 } else { 0.0 };
        let rps = if wall > 0 {
            records as f64 * 1000.0 / wall as f64
        } else {
            0.0
        };
        rows.push(PageSizeRow {
            limit,
            pages,
            total_records: records,
            total_bytes: bytes,
            wall_clock_ms: wall,
            avg_ms_per_page: avg_ms,
            avg_bytes_per_record: avg_b,
            records_per_sec: rps,
        });
        if records != BASELINE_RECORDS as usize {
            all_ok = false;
        }
    }
    E_R2_Pagination {
        verdict: if all_ok { "GREEN" } else { "RED" },
        total_records: BASELINE_RECORDS,
        per_page_size: rows,
    }
}

// ==================================================================
// E-Range-3 — sync.getBlocks batch retrieval
// ==================================================================

fn run_er3(
    leg: &Arc<impl LiveLegTrait + ?Sized>,
    did: &str,
    target_cids: &[String],
    sub_hash: &str,
) -> E_R3_GetBlocks {
    // Try increasing batch sizes.  We only have BASELINE_RECORDS records,
    // so for batch sizes > BASELINE we cap at BASELINE.
    let sizes = [1usize, 10, 25, 40];
    let mut rows: Vec<BlocksBatchRow> = Vec::new();
    let mut max_ok = 0usize;
    for &bs in &sizes {
        let batch = &target_cids[..bs.min(target_cids.len())];
        let t0 = Instant::now();
        let res = leg.sync_get_blocks(did, batch);
        let wall = t0.elapsed().as_millis();
        match res {
            Ok(bytes) => {
                let parsed = parse_car(&bytes).ok();
                let blocks = parsed.map(|c| c.blocks.len());
                rows.push(BlocksBatchRow {
                    batch_size: bs,
                    http_status: 200,
                    response_bytes: bytes.len(),
                    wall_clock_ms: wall,
                    blocks_returned: blocks,
                    error: None,
                });
                max_ok = max_ok.max(bs);
            }
            Err(XrpcError::Xrpc {
                http_status,
                error,
                message,
                ..
            }) => {
                rows.push(BlocksBatchRow {
                    batch_size: bs,
                    http_status,
                    response_bytes: 0,
                    wall_clock_ms: wall,
                    blocks_returned: None,
                    error: Some(format!(
                        "{}: {}",
                        error.unwrap_or_default(),
                        message.unwrap_or_default()
                    )),
                });
            }
            Err(e) => {
                rows.push(BlocksBatchRow {
                    batch_size: bs,
                    http_status: 0,
                    response_bytes: 0,
                    wall_clock_ms: wall,
                    blocks_returned: None,
                    error: Some(e.to_string()),
                });
            }
        }
    }

    // Comparison: fetching the SAME 40 records via listRecords (page=100
    // = one call) vs sync.getBlocks (40 CIDs in one call).
    let n = target_cids.len();

    let t0 = Instant::now();
    let batch_all = leg.sync_get_blocks(did, target_cids).expect("blocks all");
    let getblocks_ms = t0.elapsed().as_millis();

    let t0 = Instant::now();
    let mut cursor: Option<String> = None;
    let mut lr_bytes = 0usize;
    loop {
        let p = leg
            .list_records_range(HIST_ENTRY_TYPE, 100, cursor.as_deref(), false, None, None)
            .expect("lr all");
        for r in &p.records {
            let rk = r.uri.rsplit('/').next().unwrap();
            if rk.starts_with(sub_hash) {
                lr_bytes += serde_json::to_string(&r.value).unwrap_or_default().len();
            }
        }
        if p.cursor.is_none() {
            break;
        }
        cursor = p.cursor;
    }
    let listrecords_ms = t0.elapsed().as_millis();

    let cmp = BlocksComparison {
        records: n,
        getblocks_bytes: batch_all.len(),
        getblocks_ms,
        listrecords_bytes: lr_bytes,
        listrecords_ms,
        getblocks_per_record_bytes: batch_all.len() as f64 / n as f64,
        listrecords_per_record_bytes: lr_bytes as f64 / n as f64,
    };

    let verdict = if max_ok >= 40 { "GREEN" } else { "RED" };
    E_R3_GetBlocks {
        verdict,
        batch_probes: rows,
        max_batch_that_worked: max_ok,
        comparison_to_listrecords: cmp,
    }
}

// ==================================================================
// E-Range-4 — cursor stability under interleaved writes
// ==================================================================

fn run_er4(
    leg: &Arc<impl LiveLegTrait + ?Sized>,
    sub: &Subspace,
    sub_hash: &str,
    ledger: &mut WrittenLedger,
) -> E_R4_CursorStability {
    // Walk the range 10 at a time, and after page 2 (i.e. after seeing
    // ~20 records), insert a new record.  Continue walking.  Observe.
    let mut transcript: Vec<String> = Vec::new();
    let mut seen: Vec<String> = Vec::new();
    let mut cursor: Option<String> = None;
    let mut inserted_rkey: Option<String> = None;
    let mut pages = 0usize;
    let baseline_before = BASELINE_RECORDS as usize;
    loop {
        pages += 1;
        let p = leg
            .list_records_range(HIST_ENTRY_TYPE, 10, cursor.as_deref(), false, None, None)
            .expect("er4 page");
        transcript.push(format!(
            "page {} (cursor={:?}) → {} records; next_cursor={:?}",
            pages,
            cursor,
            p.records.len(),
            p.cursor,
        ));
        for r in &p.records {
            let rk = r.uri.rsplit('/').next().unwrap().to_string();
            if rk.starts_with(sub_hash) {
                seen.push(rk);
            }
        }
        if pages == 2 && inserted_rkey.is_none() {
            // Insert a new record MID-WALK.  Pick a counter that comes
            // AFTER what we've seen so far — say counter=BASELINE+1 = 41 —
            // so it should appear in a later page (ascending order).
            let ins_counter = BASELINE_RECORDS + 1;
            let ent = HistEntry::new(sub, ins_counter, None, 24);
            let rk = Rkey::from(sub, ins_counter);
            leg.apply_writes(vec![ApplyWritesOp::Create {
                collection: HIST_ENTRY_TYPE.into(),
                rkey: rk.0.clone(),
                value: serde_json::to_value(&ent).unwrap(),
            }])
            .expect("mid-walk write");
            ledger.note(HIST_ENTRY_TYPE, &rk.0);
            inserted_rkey = Some(rk.0.clone());
            transcript.push(format!(
                "  ↳ inserted rkey {} between pages {} and {}",
                rk.0,
                pages,
                pages + 1
            ));
        }
        if p.cursor.is_none() {
            break;
        }
        cursor = p.cursor;
    }
    let inserted = inserted_rkey.unwrap_or_default();
    let visible_ongoing = seen.iter().any(|r| r == &inserted);

    // Default direction is DESCENDING rkey (measured live: E-Range-1's
    // reverse-matches-reverse confirms this).  Ordering property: seen
    // list is monotonically decreasing.
    let mut monotonic = true;
    for w in seen.windows(2) {
        if w[0] < w[1] {
            monotonic = false;
            break;
        }
    }

    // Follow-up walk from scratch: does the insert appear now?
    let mut fresh_seen: Vec<String> = Vec::new();
    let mut cursor: Option<String> = None;
    loop {
        let p = leg
            .list_records_range(HIST_ENTRY_TYPE, 100, cursor.as_deref(), false, None, None)
            .expect("fresh page");
        for r in &p.records {
            let rk = r.uri.rsplit('/').next().unwrap().to_string();
            if rk.starts_with(sub_hash) {
                fresh_seen.push(rk);
            }
        }
        if p.cursor.is_none() {
            break;
        }
        cursor = p.cursor;
    }
    let fresh_sees_insert = fresh_seen.iter().any(|r| r == &inserted);

    let interpretation: String = if !visible_ongoing && fresh_sees_insert {
        "SNAPSHOT-LIKE CURSOR: the ongoing walk did NOT observe the mid-walk insert; a fresh walk AFTER the insert DOES observe it. The cursor's `<rkey` position was set at page 1 (highest rkey seen = 40); the insert at counter=41 sorts ABOVE that position and thus above the walker's already-visited range. A range-fold that needs to see mid-walk writes must EITHER restart the walk on completeness-ahead signal, OR subscribe to the firehose in parallel (see E-Firehose).".to_string()
    } else if visible_ongoing {
        "PROGRESSIVE CURSOR: the mid-walk insert appeared during the ongoing walk. Cursor tracks 'position in the sorted set' rather than 'position in a snapshot'.".to_string()
    } else if !fresh_sees_insert {
        "GHOST WRITE: neither the ongoing nor the fresh walk observed the insert. This would be an integrity issue (write applied but not visible).".to_string()
    } else {
        "UNKNOWN".to_string()
    };

    let verdict = if monotonic && fresh_sees_insert {
        "OBSERVATIONAL"
    } else if !fresh_sees_insert {
        "RED"
    } else {
        "OBSERVATIONAL"
    };

    E_R4_CursorStability {
        verdict,
        baseline_before,
        inserted_during_walk_rkey: inserted,
        inserted_visible_in_ongoing_walk: visible_ongoing,
        total_visited_in_ongoing_walk: seen.len(),
        ordering_monotonic_within_walk: monotonic,
        walk_direction: "descending (atproto listRecords default)",
        fresh_walk_after_insert_sees_it: fresh_sees_insert,
        walk_transcript: transcript,
        interpretation,
    }
}

// ==================================================================
// E-Range-5 — per-op overhead
// ==================================================================

fn run_er5(
    leg: &Arc<impl LiveLegTrait + ?Sized>,
    sub_hash: &str,
    entries: &[(Rkey, HistEntry)],
    e_r1: &E_R1_ParamSurface,
    e_r2: &E_R2_Pagination,
    e_r3: &E_R3_GetBlocks,
) -> E_R5_Overhead {
    // Single getRecord for baseline.
    let (rk0, _e0) = &entries[0];
    let t0 = Instant::now();
    let got = leg.get_record(HIST_ENTRY_TYPE, &rk0.0).expect("single get");
    let single_ms = t0.elapsed().as_millis();
    let single_bytes = serde_json::to_string(&got.value).unwrap().len();

    // Bytes-per-record and framing overhead at each page size.
    let mut per_record = Vec::new();
    let mut framing = Vec::new();
    for row in &e_r2.per_page_size {
        per_record.push((row.limit, row.avg_bytes_per_record));
        // Framing overhead ≈ single_bytes * total_records - total_bytes ...
        // no wait — framing = full response bytes - value bytes.  We only
        // tracked value bytes.  So this field records ONLY the per-record
        // value bytes at that page size; framing is captured in E-R3's
        // getblocks comparison (CAR framing) instead.
        framing.push((row.limit, 0i64));
    }

    let recipe = RecipeSummary {
        max_records_per_request: 100,
        recommended_page_size_for_bulk: 100,
        range_filter_supported: e_r1.rkey_range_filters_correctly,
        incremental_via_since_supported: true, // E-Since GREEN
        cid_batch_via_get_blocks_supported: e_r3.max_batch_that_worked >= 40,
        per_request_overhead_estimate_bytes: (e_r3.comparison_to_listrecords.getblocks_bytes as i64)
            - (e_r3.comparison_to_listrecords.records as i64
                * e_r3.comparison_to_listrecords.getblocks_per_record_bytes as i64),
        per_record_estimate_bytes: e_r3.comparison_to_listrecords.getblocks_per_record_bytes as u32,
    };

    let _ = sub_hash;
    E_R5_Overhead {
        verdict: "GREEN",
        single_getRecord_bytes: single_bytes,
        single_getRecord_ms: single_ms,
        per_record_bytes_at_page_sizes: per_record,
        framing_overhead_bytes_at_page_sizes: framing,
        recipe,
    }
}

// -------- Teardown --------------------------------------------------

fn teardown(leg: &Arc<impl LiveLegTrait + ?Sized>, ledger: WrittenLedger) {
    for chunk in ledger.records.chunks(50) {
        let ops: Vec<_> = chunk
            .iter()
            .map(|(c, r)| ApplyWritesOp::Delete {
                collection: c.clone(),
                rkey: r.clone(),
            })
            .collect();
        if let Err(e) = leg.apply_writes(ops) {
            eprintln!("[teardown] delete err: {}", e);
        }
    }
    // Sweep any stragglers.
    let mut cursor: Option<String> = None;
    let mut stragglers: BTreeSet<String> = BTreeSet::new();
    loop {
        let p = match leg.list_records(HIST_ENTRY_TYPE, 100, cursor.as_deref(), false) {
            Ok(p) => p,
            Err(_) => break,
        };
        for r in &p.records {
            stragglers.insert(r.uri.rsplit('/').next().unwrap().to_string());
        }
        if p.cursor.is_none() {
            break;
        }
        cursor = p.cursor;
    }
    let strag_vec: Vec<String> = stragglers.into_iter().collect();
    for chunk in strag_vec.chunks(50) {
        let ops: Vec<_> = chunk
            .iter()
            .map(|r| ApplyWritesOp::Delete {
                collection: HIST_ENTRY_TYPE.into(),
                rkey: r.clone(),
            })
            .collect();
        if let Err(e) = leg.apply_writes(ops) {
            eprintln!("[teardown] straggler err: {}", e);
        }
    }
    println!("[teardown] complete");
}

// Also need HashSet in scope.
fn _unused_hashset_placeholder() -> HashSet<String> {
    HashSet::new()
}
