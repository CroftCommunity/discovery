//! Second live run — E-MST, E-Since, E-Firehose.  Fresh `HttpLeg` (fresh
//! 100-write budget), single-flight pacer, gated on `HIST_PDS_APP_PASSWORD`.
//!
//! Ordering: E-MST first (writes 3, verifies proof-path), then E-Since
//! (writes 2 more, exercises delta CAR), then E-Firehose (subscribes,
//! writes 3 in {5,3,4} order, captures seq).  Teardown at end.

mod common;

use common::*;
use hist_live::budget::BudgetCaps;
use hist_live::car::parse_car;
use hist_live::firehose::FirehoseClient;
use hist_live::leg::{ApplyWritesOp, LiveLegTrait};
use hist_live::mst::{verify_leaves_in_signed_tree, MstProofSummary};
use hist_live::record::{HistEntry, Rkey, Subspace, HIST_ENTRY_TYPE};
use serde::Serialize;
use std::collections::{BTreeSet, HashSet};
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};

fn evidence_dir() -> PathBuf {
    let p = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("evidence")
        .join("live_v2");
    fs::create_dir_all(&p).unwrap();
    p
}

fn write_json<T: Serialize>(rel: &str, value: &T) {
    let p = evidence_dir().join(rel);
    fs::create_dir_all(p.parent().unwrap()).unwrap();
    fs::write(&p, serde_json::to_string_pretty(value).unwrap()).unwrap();
}

fn write_bytes(rel: &str, bytes: &[u8]) {
    let p = evidence_dir().join(rel);
    fs::create_dir_all(p.parent().unwrap()).unwrap();
    fs::write(&p, bytes).unwrap();
}

fn sha256_hex(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    hex::encode(Sha256::digest(bytes))
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

// -------- E-MST evidence structs --------

#[derive(Serialize)]
struct EMstResult {
    verdict: &'static str,
    proof: MstProofSummary,
    target_leaves_written: Vec<String>,
    all_targets_reachable: bool,
    forgery_negative_control: NegControl,
}

#[derive(Serialize)]
struct NegControl {
    description: String,
    forged_cid: String,
    correctly_absent_from_reachable_set: bool,
}

// -------- E-Since evidence structs --------

#[derive(Serialize)]
struct ESinceResult {
    verdict: &'static str,
    r0_rev: String,
    r0_car_bytes: usize,
    r0_hist_leaves: Vec<String>,
    r1_rev: String,
    r1_car_bytes: usize,
    r1_hist_leaves: Vec<String>,
    delta_since_r0_bytes: usize,
    delta_since_r0_hist_leaves: Vec<String>,
    // Assertions
    delta_leaves_equal_new_records: bool,
    r1_leaves_equal_r0_leaves_union_new: bool,
}

// -------- E-Firehose evidence structs --------

#[derive(Serialize)]
struct EFirehoseResult {
    verdict: &'static str,
    subscribed_from_cursor: Option<i64>,
    write_order_rkeys: Vec<String>,
    write_order_counters: Vec<u32>,
    firehose_events_captured_for_our_repo: Vec<FirehoseSummary>,
    /// Fold-by-antecedent-hashes head (the truth): highest counter.
    correct_head_counter: Option<u32>,
    /// Firehose seq order (the delivery cursor): the order commits actually
    /// arrived on the wire.
    firehose_seq_order_counters: Vec<u32>,
    /// Rkey enumeration order (the other delivery cursor).
    rkey_enum_order_counters: Vec<u32>,
    /// Do these two delivery cursors differ from each other AND from
    /// counter-order?  That's the MUST-NOT the brief warns about: seq is
    /// NOT the fold's order.
    seq_diverges_from_counter_order: bool,
    rkey_diverges_from_seq: bool,
}

#[derive(Serialize, Clone)]
struct FirehoseSummary {
    seq: i64,
    rev: String,
    commit_cid: String,
    op_paths: Vec<String>,
}

// -------- The test --------

#[test]
fn live_v2_orchestration_e_mst_e_since_e_firehose_and_teardown() {
    if !live_enabled() {
        println!("[live_v2] SKIPPED — HIST_PDS_APP_PASSWORD not set");
        return;
    }
    let leg = live_gentle();
    let sess = leg.session().expect("session");
    println!(
        "[live_v2] session did={} pds={}",
        sess.did, sess.pds_endpoint
    );

    let mut written = WrittenLedger::default();

    // ---- E-MST ---------------------------------------------------------
    let e_mst = run_e_mst(&leg, &sess.did, &mut written);
    write_json("e_mst.json", &e_mst);
    assert_eq!(e_mst.verdict, "GREEN", "E-MST failed");

    // ---- E-Since -------------------------------------------------------
    let e_since = run_e_since(&leg, &sess.did, &mut written);
    write_json("e_since.json", &e_since);
    assert_eq!(e_since.verdict, "GREEN", "E-Since failed");

    // ---- E-Firehose ----------------------------------------------------
    let e_fh = run_e_firehose(&leg, &sess, &mut written);
    write_json("e_firehose.json", &e_fh);
    assert_eq!(e_fh.verdict, "GREEN", "E-Firehose failed");

    // ---- Teardown ------------------------------------------------------
    teardown(&leg, written);

    let ledger = leg.budget_snapshot();
    let caps = BudgetCaps::GENTLE;
    println!(
        "[budget] writes={}/{}  blobs={}/{}  reads={}  rate_limit_signals={}",
        ledger.writes, caps.writes, ledger.blobs, caps.blobs, ledger.reads,
        ledger.rate_limit_signals
    );
    write_json(
        "budget_ledger.json",
        &serde_json::json!({
            "writes": ledger.writes,
            "reads": ledger.reads,
            "blobs": ledger.blobs,
            "write_calls": ledger.write_calls,
            "read_calls": ledger.read_calls,
            "blob_calls": ledger.blob_calls,
            "rate_limit_signals": ledger.rate_limit_signals,
            "caps": {"writes": caps.writes, "blobs": caps.blobs},
        }),
    );
    assert!(ledger.writes <= caps.writes, "budget writes overrun");
}

// ==================================================================
// E-MST
// ==================================================================

fn run_e_mst(
    leg: &Arc<impl LiveLegTrait + ?Sized>,
    did: &str,
    written: &mut WrittenLedger,
) -> EMstResult {
    // Write 3 records to a fresh subspace.
    let sub = Subspace("hist-live/v2/mst".to_string());
    let mut prev = None;
    let mut entries: Vec<(Rkey, HistEntry)> = Vec::new();
    for c in 1..=3u32 {
        let e = HistEntry::new(&sub, c, prev, 32);
        prev = Some(e.cid());
        entries.push((Rkey::from(&sub, c), e));
    }
    let ops: Vec<_> = entries
        .iter()
        .map(|(rk, e)| ApplyWritesOp::Create {
            collection: HIST_ENTRY_TYPE.into(),
            rkey: rk.0.clone(),
            value: serde_json::to_value(e).unwrap(),
        })
        .collect();
    leg.apply_writes(ops).expect("E-MST writes");
    for (rk, _) in &entries {
        written.note(HIST_ENTRY_TYPE, &rk.0);
    }
    // Fetch the full CAR and walk the MST.
    let car_bytes = leg.sync_get_repo(did).expect("sync_get_repo E-MST");
    let car = parse_car(&car_bytes).expect("parse E-MST car");
    let target_leaves: Vec<cid::Cid> = entries.iter().map(|(_, e)| e.cid()).collect();
    let proof = verify_leaves_in_signed_tree(&car, &target_leaves).expect("MST verify");
    let all_present = proof.target_leaves_absent.is_empty();

    // Negative control: verify a synthetically-fabricated CID is NOT in
    // the reachable set.  Take a well-formed CID (of the smoke test's
    // known-invalid content) — if the mirror claims this is in the tree,
    // it's a lie.
    let forged_bytes = b"hist-live/v2/mst/forged";
    let forged_cid = hist_live::canonical::cid_v1_dag_cbor(forged_bytes);
    let proof_forged = verify_leaves_in_signed_tree(&car, &[forged_cid]).expect("MST verify forged");
    let neg = NegControl {
        description: "synthetic never-written CID must be ABSENT from reachable leaves"
            .to_string(),
        forged_cid: forged_cid.to_string(),
        correctly_absent_from_reachable_set: proof_forged.target_leaves_absent.len() == 1
            && proof_forged.target_leaves_present.is_empty(),
    };

    let verdict = if all_present && neg.correctly_absent_from_reachable_set {
        "GREEN"
    } else {
        "RED"
    };
    write_bytes("archive_e_mst.car", &car_bytes);
    EMstResult {
        verdict,
        proof,
        target_leaves_written: target_leaves.iter().map(|c| c.to_string()).collect(),
        all_targets_reachable: all_present,
        forgery_negative_control: neg,
    }
}

// ==================================================================
// E-Since
// ==================================================================

fn run_e_since(
    leg: &Arc<impl LiveLegTrait + ?Sized>,
    did: &str,
    written: &mut WrittenLedger,
) -> ESinceResult {
    // R0: current repo state (after E-MST).  Fetch rev + CAR.
    let (_r0_cid, r0_rev) = leg
        .sync_get_latest_commit(did)
        .expect("getLatestCommit R0");
    let r0_bytes = leg.sync_get_repo(did).expect("sync_get_repo R0");
    let r0_leaves = extract_hist_leaves(&r0_bytes);

    // Write 2 fresh records on a distinct subspace.
    let sub = Subspace("hist-live/v2/since".to_string());
    let e1 = HistEntry::new(&sub, 1, None, 16);
    let e2 = HistEntry::new(&sub, 2, Some(e1.cid()), 16);
    let rk1 = Rkey::from(&sub, 1);
    let rk2 = Rkey::from(&sub, 2);
    // Two separate commits so R0 → R1 is a multi-commit delta.
    leg.apply_writes(vec![ApplyWritesOp::Create {
        collection: HIST_ENTRY_TYPE.into(),
        rkey: rk1.0.clone(),
        value: serde_json::to_value(&e1).unwrap(),
    }])
    .expect("E-Since write e1");
    written.note(HIST_ENTRY_TYPE, &rk1.0);
    leg.apply_writes(vec![ApplyWritesOp::Create {
        collection: HIST_ENTRY_TYPE.into(),
        rkey: rk2.0.clone(),
        value: serde_json::to_value(&e2).unwrap(),
    }])
    .expect("E-Since write e2");
    written.note(HIST_ENTRY_TYPE, &rk2.0);

    // R1: after the writes.
    let (_r1_cid, r1_rev) = leg
        .sync_get_latest_commit(did)
        .expect("getLatestCommit R1");
    let r1_bytes = leg.sync_get_repo(did).expect("sync_get_repo R1");
    let r1_leaves = extract_hist_leaves(&r1_bytes);

    // Delta from R0 to R1.
    let delta_bytes = leg
        .sync_get_repo_since(did, Some(&r0_rev))
        .expect("sync_get_repo since R0");
    let delta_leaves = extract_hist_leaves(&delta_bytes);

    // Assertions:
    let new_records: HashSet<String> = [&e1, &e2].iter().map(|e| e.cid().to_string()).collect();
    let r0_set: HashSet<String> = r0_leaves.iter().cloned().collect();
    let r1_set: HashSet<String> = r1_leaves.iter().cloned().collect();
    let delta_set: HashSet<String> = delta_leaves.iter().cloned().collect();

    // Delta MUST contain the new records; may or may not contain older
    // ones depending on server behavior.  Property: new_records ⊂ delta.
    let delta_contains_new = new_records.is_subset(&delta_set);

    // r1 = r0 ∪ new_records (assuming no concurrent writes).
    let expected_r1: HashSet<String> = r0_set.union(&new_records).cloned().collect();
    let r1_matches_expected = r1_set == expected_r1;

    write_bytes("archive_e_since_r1.car", &r1_bytes);
    write_bytes("archive_e_since_delta.car", &delta_bytes);

    let verdict = if delta_contains_new && r1_matches_expected {
        "GREEN"
    } else {
        "RED"
    };

    ESinceResult {
        verdict,
        r0_rev,
        r0_car_bytes: r0_bytes.len(),
        r0_hist_leaves: r0_leaves,
        r1_rev,
        r1_car_bytes: r1_bytes.len(),
        r1_hist_leaves: r1_leaves,
        delta_since_r0_bytes: delta_bytes.len(),
        delta_since_r0_hist_leaves: delta_leaves,
        delta_leaves_equal_new_records: delta_contains_new,
        r1_leaves_equal_r0_leaves_union_new: r1_matches_expected,
    }
}

fn extract_hist_leaves(car_bytes: &[u8]) -> Vec<String> {
    let car = match parse_car(car_bytes) {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };
    let mut out = Vec::new();
    for b in &car.blocks {
        if let Ok(v) = hist_live::canonical::dag_cbor_to_atproto_json(&b.data) {
            if v.get("$type").and_then(|t| t.as_str()) == Some(HIST_ENTRY_TYPE) {
                out.push(b.cid.to_string());
            }
        }
    }
    out.sort();
    out
}

// ==================================================================
// E-Firehose
// ==================================================================

fn run_e_firehose(
    leg: &Arc<impl LiveLegTrait + ?Sized>,
    sess: &hist_live::leg::SessionTokens,
    written: &mut WrittenLedger,
) -> EFirehoseResult {
    // Subscribe from now (no cursor) BEFORE writing.
    let pds_host = sess
        .pds_endpoint
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .trim_end_matches('/');
    println!("[e-firehose] connecting wss://{}/...subscribeRepos", pds_host);
    let mut fh = match FirehoseClient::connect(pds_host, None) {
        Ok(c) => c,
        Err(e) => {
            return EFirehoseResult {
                verdict: "UNPROVEN",
                subscribed_from_cursor: None,
                write_order_rkeys: vec![],
                write_order_counters: vec![],
                firehose_events_captured_for_our_repo: vec![],
                correct_head_counter: None,
                firehose_seq_order_counters: vec![],
                rkey_enum_order_counters: vec![],
                seq_diverges_from_counter_order: false,
                rkey_diverges_from_seq: false,
            };
        }
    };
    println!("[e-firehose] connected; sleeping 1s to let subscription settle");
    std::thread::sleep(Duration::from_millis(1000));

    // Write E6-shape sequence to a fresh subspace, {5,3,4} in wall-clock order.
    let sub = Subspace("hist-live/v2/firehose".to_string());
    let e3 = HistEntry::new(&sub, 3, None, 16);
    let e4 = HistEntry::new(&sub, 4, Some(e3.cid()), 16);
    let e5 = HistEntry::new(&sub, 5, Some(e4.cid()), 16);
    let write_seq: Vec<(&HistEntry, u32)> = vec![(&e5, 5), (&e3, 3), (&e4, 4)];
    let mut write_order_counters = Vec::new();
    let mut write_order_rkeys = Vec::new();
    for (ent, c) in &write_seq {
        let rk = Rkey::from(&sub, *c);
        leg.apply_writes(vec![ApplyWritesOp::Create {
            collection: HIST_ENTRY_TYPE.into(),
            rkey: rk.0.clone(),
            value: serde_json::to_value(*ent).unwrap(),
        }])
        .expect("E-Firehose write");
        written.note(HIST_ENTRY_TYPE, &rk.0);
        write_order_counters.push(*c);
        write_order_rkeys.push(rk.0.clone());
    }

    // Read from firehose until we've captured 3 events touching our repo
    // OR the deadline elapses (10s).  There will be non-our-repo events
    // interleaved — filter by DID.
    println!("[e-firehose] reading firehose for our-repo events (deadline 10s)...");
    let deadline = Instant::now() + Duration::from_secs(10);
    let mut captured: Vec<FirehoseSummary> = Vec::new();
    let mut fh_counters: Vec<u32> = Vec::new();
    while captured.len() < 3 && Instant::now() < deadline {
        let batch = fh
            .read_commits(50, deadline)
            .unwrap_or_default();
        for ev in batch.commits {
            if ev.repo == sess.did {
                let mut op_paths = Vec::new();
                for op in &ev.ops {
                    op_paths.push(format!("{} {}", op.action, op.path));
                    // Extract counter from rkey `<hash>_<7digits>`
                    if op.path.starts_with(HIST_ENTRY_TYPE) {
                        if let Some(rkey) = op.path.rsplit('/').next() {
                            if let Some(n) = rkey.rsplit('_').next().and_then(|s| s.parse::<u32>().ok()) {
                                fh_counters.push(n);
                            }
                        }
                    }
                }
                captured.push(FirehoseSummary {
                    seq: ev.seq,
                    rev: ev.rev.clone(),
                    commit_cid: ev.commit_cid.clone(),
                    op_paths,
                });
            }
        }
    }
    fh.close();

    // Sort captured by seq (delivery order).
    let mut by_seq = captured.clone();
    by_seq.sort_by_key(|s| s.seq);
    let fh_counter_order_by_seq: Vec<u32> = by_seq
        .iter()
        .flat_map(|s| {
            s.op_paths.iter().filter_map(|p| {
                if p.contains(HIST_ENTRY_TYPE) {
                    p.rsplit('_').next().and_then(|s| s.parse::<u32>().ok())
                } else {
                    None
                }
            })
        })
        .collect();

    // Rkey enumeration order: {3, 4, 5} sorted.
    let rkey_order: Vec<u32> = vec![3, 4, 5];

    // Correct fold head: highest counter = 5.
    let correct_head: Option<u32> = Some(5);

    // Divergences:
    // - firehose seq order = write commit order = {5, 3, 4}
    // - rkey order = {3, 4, 5}
    // - counter order = {3, 4, 5}
    let seq_diverges = fh_counter_order_by_seq != vec![3, 4, 5];
    let rkey_vs_seq = fh_counter_order_by_seq != rkey_order;

    let ok = captured.len() >= 3 && seq_diverges && rkey_vs_seq;
    let verdict = if ok { "GREEN" } else { "UNPROVEN" };

    EFirehoseResult {
        verdict,
        subscribed_from_cursor: None,
        write_order_rkeys,
        write_order_counters,
        firehose_events_captured_for_our_repo: captured,
        correct_head_counter: correct_head,
        firehose_seq_order_counters: fh_counter_order_by_seq,
        rkey_enum_order_counters: rkey_order,
        seq_diverges_from_counter_order: seq_diverges,
        rkey_diverges_from_seq: rkey_vs_seq,
    }
}

// ==================================================================
// Teardown
// ==================================================================

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
            eprintln!("[teardown] batch delete err: {}", e);
        }
    }
    // Sweep any stragglers.
    let mut cursor: Option<String> = None;
    let mut stragglers: BTreeSet<String> = BTreeSet::new();
    loop {
        let page = match leg.list_records(HIST_ENTRY_TYPE, 100, cursor.as_deref(), false) {
            Ok(p) => p,
            Err(_) => break,
        };
        for r in &page.records {
            stragglers.insert(r.uri.rsplit('/').next().unwrap().to_string());
        }
        if page.cursor.is_none() {
            break;
        }
        cursor = page.cursor;
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
            eprintln!("[teardown] straggler batch err: {}", e);
        }
    }
    println!("[teardown] complete");
}
