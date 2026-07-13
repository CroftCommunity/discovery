//! The reality check: diff my predicted `RecordEvent` shape against a real frame.
//!
//! This is the headline learning output. Given the first real commit event (raw
//! JSON) and the `RecordEvent` my parser produced from it, report which
//! predictions held, which fields were present-but-unmodeled, and which expected
//! fields were absent or live somewhere other than predicted.

use std::collections::BTreeMap;

use serde_json::Value;

use crate::record_source::RecordEvent;

/// Aggregate counts collected across the whole ingest window.
#[derive(Default)]
pub struct Findings {
    pub frames: usize,
    pub commits: usize,
    pub malformed: usize,
    /// kind -> count, for non-commit frames (tests prediction P4).
    pub non_commit_kinds: BTreeMap<String, usize>,
    pub deletes_seen: usize,
    pub creates_seen: usize,
    pub updates_seen: usize,
}

/// Enumerate the *envelope* keys of a commit frame (top level + inside `commit`),
/// deliberately NOT descending into `commit.record` (that subtree is stored whole
/// and is intentionally opaque to the envelope model).
fn envelope_paths(raw: &Value) -> Vec<String> {
    let mut paths = Vec::new();
    if let Some(obj) = raw.as_object() {
        for (k, v) in obj {
            paths.push(k.clone());
            if k == "commit" {
                if let Some(c) = v.as_object() {
                    for (kk, _) in c {
                        paths.push(format!("commit.{kk}"));
                    }
                }
            }
        }
    }
    paths
}

/// The set of envelope paths my parser actually reads.
fn consumed_paths() -> Vec<&'static str> {
    vec![
        "kind",
        "did",
        "time_us",
        "commit",
        "commit.operation",
        "commit.collection",
        "commit.rkey",
        "commit.record",
        "commit.cid",
    ]
}

pub fn print_shape_report(raw: &Value, ev: &RecordEvent) {
    println!("\n================ HYPOTHESIS vs REALITY (field-by-field) ================");
    println!("First real commit event, raw envelope keys vs my predicted model.\n");

    let present = envelope_paths(raw);
    let consumed = consumed_paths();

    println!("-- Envelope keys present in the REAL event:");
    for p in &present {
        let modeled = consumed.contains(&p.as_str());
        println!("     {} {}", if modeled { "[modeled]    " } else { "[UNMODELED]  " }, p);
    }

    println!("\n-- Present-but-UNMODELED (real fields my RecordEvent ignores):");
    let mut any_unmodeled = false;
    for p in &present {
        if !consumed.contains(&p.as_str()) {
            any_unmodeled = true;
            let sample = path_value(raw, p)
                .map(short_value)
                .unwrap_or_default();
            println!("     • {p} = {sample}");
        }
    }
    if !any_unmodeled {
        println!("     (none)");
    }

    println!("\n-- Prediction-by-prediction verdicts:");

    // P2: cid at top level next to did?
    let top_cid = raw.get("cid").is_some();
    let commit_cid = raw.get("commit").and_then(|c| c.get("cid")).is_some();
    println!(
        "     P2  cid at top level next to did? -> {}. Top-level cid present: {}. \
         Real location: {}.",
        if top_cid { "HELD" } else { "WRONG" },
        top_cid,
        if commit_cid { "commit.cid" } else { "<not found>" },
    );

    // P3: time_us and units.
    match raw.get("time_us").and_then(Value::as_i64) {
        Some(t) => {
            let as_secs = t / 1_000_000;
            let plausible = (1_500_000_000..2_500_000_000).contains(&as_secs);
            println!(
                "     P3  cursor field `time_us` present? -> HELD ({t}). Treating as \
                 microseconds-since-epoch gives unix seconds {as_secs} ({}).",
                if plausible {
                    "plausible current date -> units = MICROSECONDS"
                } else {
                    "implausible -> units are NOT microseconds"
                }
            );
        }
        None => println!("     P3  cursor field `time_us` present? -> WRONG (absent)"),
    }

    // P4: handled by aggregate non-commit kinds, printed in print_findings.
    println!(
        "     P4  every frame is kind=commit? -> see aggregate kinds below \
         (non-commit kinds observed are a violation)."
    );

    // P5: delete shape — noted in aggregate (need a real delete to confirm).
    println!(
        "     P5  delete carries no record/cid? -> confirmed at runtime if a delete \
         was observed (see index stats: deletes removed rows)."
    );

    // P6: record.createdAt ISO string.
    let created = ev
        .record
        .as_ref()
        .and_then(|r| r.get("createdAt"))
        .and_then(Value::as_str);
    match created {
        Some(s) => println!(
            "     P6  record.createdAt is an ISO string? -> HELD (\"{s}\")."
        ),
        None => println!(
            "     P6  record.createdAt is an ISO string? -> not present on this record."
        ),
    }

    // Also surface a couple of structural realities worth knowing.
    let rev = raw.get("commit").and_then(|c| c.get("rev")).and_then(Value::as_str);
    if let Some(rev) = rev {
        println!(
            "\n-- Note: `commit.rev` = \"{rev}\" — the repo revision (TID). Unmodeled; \
             a real AppView uses it for ordering/dedup within a repo."
        );
    }
    println!("=======================================================================");
}

pub fn print_findings(f: &Findings) {
    println!("\n---------------- aggregate ingest findings ----------------");
    println!("  frames received : {}", f.frames);
    println!("  commit events   : {}", f.commits);
    println!(
        "    creates={}  updates={}  deletes={}",
        f.creates_seen, f.updates_seen, f.deletes_seen
    );
    println!("  malformed frames: {}", f.malformed);
    if f.non_commit_kinds.is_empty() {
        println!("  non-commit kinds: (none observed) -> prediction P4 HELD for this window");
    } else {
        println!("  non-commit kinds (prediction P4 VIOLATED — these exist):");
        for (k, n) in &f.non_commit_kinds {
            println!("     • kind={k}: {n}");
        }
    }
    println!("-----------------------------------------------------------");
}

fn path_value<'a>(raw: &'a Value, path: &str) -> Option<&'a Value> {
    let mut cur = raw;
    for seg in path.split('.') {
        cur = cur.get(seg)?;
    }
    Some(cur)
}

fn short_value(v: &Value) -> String {
    let s = match v {
        Value::String(s) => format!("\"{s}\""),
        other => other.to_string(),
    };
    if s.len() > 80 {
        format!("{}…", &s[..80])
    } else {
        s
    }
}
