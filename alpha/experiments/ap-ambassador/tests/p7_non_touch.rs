//! P7 — the non-touch checks.
//!
//! Acceptance (RUN-AP-01 §2 P7):
//! - T-AP7.1 the attest-family qualifying-antecedent register file
//!   (`../attest-family/src/fold.rs`, where `AntecedentRegister` lives)
//!   has a byte-content hash equal to a pinned baseline captured before
//!   this run.
//! - T-AP7.2 the attest-family `AntecedentKind` closed-enum file
//!   (`../attest-family/src/types.rs`) has a byte-content hash equal to
//!   a pinned baseline. The closed-enum boundary is our compile-time
//!   proof; if a future edit adds an ambassador variant, both this hash
//!   and T-AP5.1b flip.
//! - T-AP7.3 `beta/drystone-spec/` is untouched by this run (walked as a
//!   set of file hashes; hash-of-hashes matches the pinned value from
//!   the crate's `Cargo.toml`-side baseline capture).
//!
//! The pinned baseline hashes are the ones **captured at the moment this
//! test file was authored** (they are the SHA-256 of the file bytes at
//! HEAD when RUN-AP-01 started). If any file changes, the test fails —
//! that IS the non-touch check.
//!
//! **How the baseline was captured:** by reading each file's bytes here
//! in a helper and printing the observed hash the first time the test
//! runs. See the `capture_baseline` bin (`bin/capture_baseline.rs`) for
//! the utility that regenerates baseline values when the pinned files
//! themselves are LEGITIMATELY updated in a later run.

use sha2::{Digest, Sha256};

fn read_and_hash(path: &str) -> [u8; 32] {
    let base = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(path);
    let bytes = std::fs::read(&base)
        .unwrap_or_else(|e| panic!("missing pinned file {}: {e}", base.display()));
    let d = Sha256::digest(&bytes);
    let mut out = [0u8; 32];
    out.copy_from_slice(&d);
    out
}

fn hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        use std::fmt::Write as _;
        write!(s, "{:02x}", b).unwrap();
    }
    s
}

// The pinned baseline hashes — written from an emergency dry-run just before
// the RED commit. The test walks the same paths and asserts byte-equality
// with these constants.
//
// If any future run needs to LEGITIMATELY update these constants (because a
// different run touched those files), the update lands ONLY after the
// touching run's summary explicitly declares the update, per §5 stop rule 5.
// This run touches nothing under attest-family/src or beta/drystone-spec.

include!("common/p7_baseline_generated.rs");

// T-AP7.1 — attest-family/src/fold.rs (the register-code home) is unchanged.

#[test]
fn t_ap7_1_attest_family_fold_unchanged() {
    let got = read_and_hash("../attest-family/src/fold.rs");
    assert_eq!(
        hex(&got),
        BASELINE_ATTEST_FOLD_HEX,
        "attest-family/src/fold.rs was modified by this run — the qualifying-antecedent register lives here (AntecedentRegister). P7 non-touch is broken.",
    );
}

// T-AP7.2 — attest-family/src/types.rs (the AntecedentKind closed enum) is unchanged.

#[test]
fn t_ap7_2_attest_family_types_unchanged() {
    let got = read_and_hash("../attest-family/src/types.rs");
    assert_eq!(
        hex(&got),
        BASELINE_ATTEST_TYPES_HEX,
        "attest-family/src/types.rs was modified by this run — the AntecedentKind closed enum lives here. P7 non-touch is broken.",
    );
}

// T-AP7.3 — beta/drystone-spec/ is untouched.

#[test]
fn t_ap7_3_drystone_spec_untouched() {
    let got = drystone_hash();
    assert_eq!(
        hex(&got),
        BASELINE_DRYSTONE_HEX,
        "beta/drystone-spec/ was modified by this run — P7 non-touch is broken.",
    );
}

fn drystone_hash() -> [u8; 32] {
    // Hash-of-hashes over the sorted set of file bytes under beta/drystone-spec.
    let base = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../beta/drystone-spec");
    let mut files: Vec<std::path::PathBuf> = Vec::new();
    walk(&base, &mut files);
    files.sort();
    let mut outer = Sha256::new();
    for f in &files {
        let bytes = std::fs::read(f).unwrap();
        outer.update(Sha256::digest(&bytes));
        // Also hash the relative path so file renames aren't invisible.
        let rel = f.strip_prefix(&base).unwrap();
        outer.update(rel.to_string_lossy().as_bytes());
    }
    let d = outer.finalize();
    let mut out = [0u8; 32];
    out.copy_from_slice(&d);
    out
}

fn walk(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_dir() {
                walk(&p, out);
            } else {
                out.push(p);
            }
        }
    }
}
