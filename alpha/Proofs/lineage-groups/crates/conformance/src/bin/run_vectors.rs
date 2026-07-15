//! `run-vectors` — load every emitted vector file and re-prove it against the
//! real `lineage-core` / `lineage-history` public API. Prints a per-category
//! PASS/FAIL summary and exits nonzero on any mismatch.
//!
//! This is the green check: it never trusts a recorded value as the answer; it
//! recomputes the value and diffs. A faked or drifted vector fails here.

use std::path::{Path, PathBuf};
use std::process::exit;

use conformance::model::{
    AdversarialFile, AuthorityFile, DerivationsFile, FoldFile, Manifest, ReconcileFile,
    RevocationFile, SigningFile, TsVerdictFile,
};
use conformance::runner::{
    check_adversarial, check_derivation, check_fold, check_reconcile, check_revocation,
    check_revocation_authority, check_signing, check_ts_experiment, CheckOutcome,
};

/// A category's running tally.
struct Tally {
    name: String,
    pass: usize,
    fail: usize,
    failures: Vec<String>,
}

impl Tally {
    fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            pass: 0,
            fail: 0,
            failures: Vec::new(),
        }
    }

    fn record(&mut self, label: &str, outcome: CheckOutcome) {
        match outcome {
            CheckOutcome::Pass => self.pass += 1,
            CheckOutcome::Fail(reason) => {
                self.fail += 1;
                self.failures.push(format!("{label}: {reason}"));
            }
        }
    }
}

fn main() {
    let root = vectors_root();
    if !root.exists() {
        eprintln!(
            "no vectors at {} — run `cargo run -p conformance --bin emit-vectors` first",
            root.display()
        );
        exit(2);
    }

    let mut tallies: Vec<Tally> = Vec::new();

    // cat 1
    let mut t = Tally::new("cat1 derivations");
    let df: DerivationsFile = load(&root.join("derivations.json"));
    for v in &df.vectors {
        t.record(&v.kind, check_derivation(v));
    }
    tallies.push(t);

    // cat 2
    let mut t = Tally::new("cat2 signing");
    let sf: SigningFile = load(&root.join("signing.json"));
    t.record("good", check_signing(&sf.good));
    t.record("tampered (must-reject)", check_signing(&sf.tampered));
    tallies.push(t);

    // cat 3+4
    let mut t = Tally::new("cat3+4 fold/thresholds");
    let ff: FoldFile = load(&root.join("fold.json"));
    for v in &ff.vectors {
        t.record(&v.label, check_fold(v));
    }
    tallies.push(t);

    // cat 5
    let mut t = Tally::new("cat5 revocation (mechanics)");
    let rf: RevocationFile = load(&root.join("revocation.json"));
    for v in &rf.vectors {
        t.record(&v.label, check_revocation(v));
    }
    tallies.push(t);

    // cat 5b
    let mut t = Tally::new("cat5b revoke-authority");
    let af: AuthorityFile = load(&root.join("revocation-authority.json"));
    for v in &af.vectors {
        t.record(&v.label, check_revocation_authority(v));
    }
    tallies.push(t);

    // cat 6
    let mut t = Tally::new("cat6 reconcile C1..C10");
    let recon_dir = root.join("reconcile");
    let mut files = list_json(&recon_dir);
    files.sort();
    for f in &files {
        let rcf: ReconcileFile = load(f);
        t.record(&rcf.vector.id, check_reconcile(&rcf.vector));
    }
    tallies.push(t);

    // cat 7
    let mut t = Tally::new("cat7 adversarial AR-1..AR-6");
    let avf: AdversarialFile = load(&root.join("adversarial.json"));
    for v in &avf.vectors {
        t.record(&format!("{} {}", v.ar, v.label), check_adversarial(v));
    }
    tallies.push(t);

    // cat 8 (visibility V1..V9 + S2) — TS-authoritative; Rust validates structure.
    let vis_path = root.join("visibility").join("visibility.json");
    if vis_path.exists() {
        let mut t = Tally::new("cat8 visibility (TS-authoritative)");
        let vf: TsVerdictFile = load(&vis_path);
        for e in &vf.experiments {
            t.record(&e.name, check_ts_experiment(e));
        }
        tallies.push(t);
    }

    // cat 9 (freshness E2.16) — TS-authoritative; Rust validates structure.
    let fresh_path = root.join("freshness.json");
    if fresh_path.exists() {
        let mut t = Tally::new("cat9 freshness E2.16 (TS-authoritative)");
        let ff: TsVerdictFile = load(&fresh_path);
        for e in &ff.experiments {
            t.record(&e.name, check_ts_experiment(e));
        }
        tallies.push(t);
    }

    // Manifest integrity: re-hash each listed file and confirm it matches.
    let mut t = Tally::new("manifest integrity");
    let manifest: Manifest = load(&root.join("MANIFEST.json"));
    for entry in &manifest.entries {
        let path = root.parent().expect("conformance root").join(&entry.file);
        match std::fs::read(&path) {
            Ok(bytes) => {
                use sha2::{Digest, Sha256};
                let mut h = Sha256::new();
                h.update(&bytes);
                let actual = hex::encode(h.finalize());
                if actual == entry.sha256_hex {
                    t.record(&entry.file, CheckOutcome::Pass);
                } else {
                    t.record(
                        &entry.file,
                        CheckOutcome::Fail(format!(
                            "sha256 {actual} != manifest {}",
                            entry.sha256_hex
                        )),
                    );
                }
            }
            Err(e) => t.record(&entry.file, CheckOutcome::Fail(format!("read failed: {e}"))),
        }
    }
    tallies.push(t);

    // --- summary -------------------------------------------------------------
    println!("\nconformance runner — per-category summary");
    println!("=========================================");
    let mut total_pass = 0;
    let mut total_fail = 0;
    for t in &tallies {
        let status = if t.fail == 0 { "PASS" } else { "FAIL" };
        println!("[{status}] {:<32} {} pass, {} fail", t.name, t.pass, t.fail);
        for f in &t.failures {
            println!("        - {f}");
        }
        total_pass += t.pass;
        total_fail += t.fail;
    }
    println!("-----------------------------------------");
    println!("TOTAL: {total_pass} pass, {total_fail} fail");

    if total_fail > 0 {
        exit(1);
    }
}

/// `conformance/vectors/` under the workspace root.
fn vectors_root() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    Path::new(manifest_dir)
        .parent()
        .and_then(Path::parent)
        .expect("workspace root")
        .join("conformance")
        .join("vectors")
}

/// Load + deserialize a vector file, exiting on any error (a missing or
/// malformed file is a hard failure, not a skipped check).
fn load<T: serde::de::DeserializeOwned>(path: &Path) -> T {
    let data = std::fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("read {}: {e}", path.display());
        exit(2);
    });
    serde_json::from_str(&data).unwrap_or_else(|e| {
        eprintln!("parse {}: {e}", path.display());
        exit(2);
    })
}

/// List `*.json` files in a directory (non-recursive).
fn list_json(dir: &Path) -> Vec<PathBuf> {
    std::fs::read_dir(dir)
        .map(|rd| {
            rd.filter_map(Result::ok)
                .map(|e| e.path())
                .filter(|p| p.extension().is_some_and(|x| x == "json"))
                .collect()
        })
        .unwrap_or_default()
}
