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

/// Every `.rs` file under `src/`, crate-relative (RUN-ATTEST-02 scans the
/// WHOLE source tree, so a new module cannot dodge the invariants by not
/// being on a hardcoded list).
pub fn crate_src_files() -> Vec<String> {
    let root = env!("CARGO_MANIFEST_DIR");
    let mut out: Vec<String> = std::fs::read_dir(format!("{root}/src"))
        .expect("read src dir")
        .filter_map(|e| {
            let name = e.expect("dir entry").file_name().into_string().expect("utf8 name");
            name.ends_with(".rs").then(|| format!("src/{name}"))
        })
        .collect();
    out.sort();
    out
}

// ---------------------------------------------------------------------------
// RUN-ATTEST-02 adversary sweep — extends the T-AT4.3 surface (same leaf
// walkers, same subslice checks; nothing forked).
// ---------------------------------------------------------------------------

use attest_family::fold::{AttestState, CredentialStatus};
use attest_family::query::{edge_list_ipld, FreshnessDial};
use attest_family::types::{DateClaim, PersonaId, Scope, SubjectRef};

/// The public surface of ONE persona as a third-party `viewer` can sweep it:
/// the persona's disclosed edge list, its corroboration structures for the
/// given scopes, its folded credential views, and its published envelopes
/// (credentials + vetting facts), in a deterministic order. Byte-level
/// equality of two sweeps means byte-level equality of everything that viewer
/// can obtain about the persona from the fold surface.
pub fn persona_surface_ipld(
    state: &AttestState,
    of: &PersonaId,
    viewer: &PersonaId,
    scopes: &[&str],
    published: &[attest_family::types::Envelope],
) -> Ipld {
    let dial = FreshnessDial { stale_after_days: 3650 };
    let as_of = DateClaim::new(2026, 7, 18);
    let mut sections: Vec<Ipld> = Vec::new();
    sections.push(edge_list_ipld(&state.edge_list(of, viewer)));
    for scope in scopes {
        let resp = state.corroboration(viewer, &SubjectRef::Persona(*of), &Scope::new(scope), &dial, as_of);
        sections.push(resp.to_ipld());
    }
    let mut cred_views: Vec<&attest_family::fold::CredentialView> =
        state.credentials().iter().filter(|c| &c.subject == of).collect();
    cred_views.sort_by(|a, b| a.object.0.cmp(&b.object.0));
    sections.push(Ipld::List(cred_views.iter().map(|c| c.to_ipld()).collect()));
    let mut published_bytes: Vec<Vec<u8>> =
        published.iter().map(|e| e.canonical_bytes_with_sig()).collect();
    published_bytes.sort();
    sections.push(Ipld::List(published_bytes.into_iter().map(Ipld::Bytes).collect()));
    Ipld::List(sections)
}

pub fn persona_surface_bytes(
    state: &AttestState,
    of: &PersonaId,
    viewer: &PersonaId,
    scopes: &[&str],
    published: &[attest_family::types::Envelope],
) -> Vec<u8> {
    serde_ipld_dagcbor::to_vec(&persona_surface_ipld(state, of, viewer, scopes, published))
        .expect("pure value encode cannot fail")
}

/// Structural mask (RUN-ATTEST-02): render an Ipld tree with every byte-leaf
/// of 16+ bytes replaced by a stable occurrence-order token (`b0`, `b1`, …;
/// the SAME value gets the SAME token, so the equality pattern of identifiers
/// survives while their values vanish). Two masked forms being identical
/// means the structures differ only in identifier values — the shape carries
/// no information about which persona/world produced it.
pub fn masked_form(v: &Ipld) -> String {
    let mut names: std::collections::BTreeMap<Vec<u8>, usize> = std::collections::BTreeMap::new();
    let mut out = String::new();
    masked_walk(v, &mut names, &mut out);
    out
}

fn masked_walk(v: &Ipld, names: &mut std::collections::BTreeMap<Vec<u8>, usize>, out: &mut String) {
    match v {
        Ipld::Bytes(b) if b.len() >= 16 => {
            let next = names.len();
            let n = *names.entry(b.clone()).or_insert(next);
            out.push_str(&format!("b{n}"));
        }
        Ipld::Bytes(b) => out.push_str(&format!("raw{b:02x?}")),
        Ipld::String(s) => out.push_str(&format!("s({s})")),
        Ipld::Integer(i) => out.push_str(&format!("i({i})")),
        Ipld::Bool(b) => out.push_str(&format!("t({b})")),
        Ipld::Null => out.push_str("null"),
        Ipld::Float(f) => out.push_str(&format!("f({f})")),
        Ipld::List(l) => {
            out.push('[');
            for x in l {
                masked_walk(x, names, out);
                out.push(',');
            }
            out.push(']');
        }
        Ipld::Map(m) => {
            out.push('{');
            for (k, x) in m {
                out.push_str(k);
                out.push(':');
                masked_walk(x, names, out);
                out.push(',');
            }
            out.push('}');
        }
        Ipld::Link(_) => out.push_str("link"),
    }
}

/// Convenience: is this persona's credential (by object id) standing?
pub fn credential_standing(state: &AttestState, id: &attest_family::types::ObjectId) -> bool {
    state
        .credential(id)
        .map(|c| c.status == CredentialStatus::Standing)
        .unwrap_or(false)
}
