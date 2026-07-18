// Included via `mod common;` by several test binaries; each compiles the module
// independently, so helpers used by only some tests read as dead code in others.
#![allow(dead_code)]

//! Shared scaffolding for the RUN-ATTEST-01 experiments.

use attest_family::fold::AttestLog;
use attest_family::types::Envelope;
use ipld_core::ipld::Ipld;

/// Append a corpus in the given order; panics on signature rejection (fixture
/// corpora are always well-signed).
pub fn log_from(envs: &[Envelope]) -> AttestLog {
    let mut log = AttestLog::new();
    for e in envs {
        log.append(e.clone()).expect("fixture envelope must append");
    }
    log
}

/// Collect every byte-string leaf in an Ipld tree (the leakage surface for
/// T-AT3.5 / T-AT4.3).
pub fn ipld_byte_leaves(v: &Ipld, out: &mut Vec<Vec<u8>>) {
    match v {
        Ipld::Bytes(b) => out.push(b.clone()),
        Ipld::List(l) => {
            for x in l {
                ipld_byte_leaves(x, out);
            }
        }
        Ipld::Map(m) => {
            for x in m.values() {
                ipld_byte_leaves(x, out);
            }
        }
        _ => {}
    }
}

/// Collect every (path, integer/float) numeric leaf in an Ipld tree — the
/// no-scalar walker for T-AT0.2 / T-AT3.1. The path is the chain of map keys.
pub fn ipld_numeric_leaves(v: &Ipld, path: &str, out: &mut Vec<(String, String)>) {
    match v {
        Ipld::Integer(i) => out.push((path.to_string(), format!("int:{i}"))),
        Ipld::Float(f) => out.push((path.to_string(), format!("float:{f}"))),
        Ipld::List(l) => {
            for x in l {
                ipld_numeric_leaves(x, path, out);
            }
        }
        Ipld::Map(m) => {
            for (k, x) in m {
                let sub = if path.is_empty() { k.clone() } else { format!("{path}.{k}") };
                ipld_numeric_leaves(x, &sub, out);
            }
        }
        _ => {}
    }
}

/// Does `hay` contain `needle` as a contiguous subslice?
pub fn contains_subslice(hay: &[u8], needle: &[u8]) -> bool {
    if needle.is_empty() || hay.len() < needle.len() {
        return false;
    }
    hay.windows(needle.len()).any(|w| w == needle)
}

/// Read a crate source file (path relative to the crate root).
pub fn crate_source(rel: &str) -> String {
    let root = env!("CARGO_MANIFEST_DIR");
    std::fs::read_to_string(format!("{root}/{rel}"))
        .unwrap_or_else(|e| panic!("read {rel}: {e}"))
}

/// Strip comment lines (`//`, `//!`, `///`) and blank lines, keeping code lines
/// for source-scan invariants.
pub fn code_lines(src: &str) -> Vec<(usize, String)> {
    src.lines()
        .enumerate()
        .filter_map(|(i, l)| {
            let t = l.trim();
            if t.is_empty() || t.starts_with("//") {
                None
            } else {
                Some((i + 1, l.to_string()))
            }
        })
        .collect()
}
