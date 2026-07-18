//! Part 1 — EXP-AT1: the co-signed edge (mutual mode). T-AT1.*.
//!
//! An edge is a co-signed op in the R7 shape (approvals are antecedents):
//! Alice's half and Bob's half each reference the same canonical shared core;
//! the edge exists iff both halves match on core hash. The per-side label is
//! side-local.

mod common;

use attest_family::fixtures::*;
use attest_family::fold::{EdgeStatus, Marker};
use attest_family::query::FreshnessDial;
use attest_family::types::*;
use common::*;

fn d(y: u16, m: u8, day: u8) -> DateClaim {
    DateClaim::new(y, m, day)
}

/// The standard established edge: ceremony session co-signed by both, then
/// two halves over the same core. Returns (world, ceremony facts, halves,
/// core hash).
fn established_edge() -> (World, Vec<Envelope>, [u8; 32]) {
    let w = World::new();
    let session = [0x51; 16];
    let cer_a = w.p1a.emit(vec![], ceremony_fact(session, w.p1a.id, w.p2.id, d(2026, 7, 17)));
    let cer_b = w.p2.emit(vec![], ceremony_fact(session, w.p1a.id, w.p2.id, d(2026, 7, 17)));
    let core = edge_core(w.p1a.id, w.p2.id, [0x10; 16], vec![cer_a.object_id(), cer_b.object_id()]);
    let core_hash = core.core_hash();
    let half_a = w.p1a.emit(
        vec![cer_a.object_id(), cer_b.object_id()],
        edge_half(core.clone(), "friend from school"),
    );
    let half_b = w.p2.emit(
        vec![cer_a.object_id(), cer_b.object_id()],
        edge_half(core, "roommate's sister"),
    );
    (w, vec![cer_a, cer_b, half_a, half_b], core_hash)
}

// ---------------------------------------------------------------------------
// T-AT1.1 — a single half folds to pending; pending is never partial
// ---------------------------------------------------------------------------

#[test]
fn half_is_not_an_edge() {
    let (w, corpus, core_hash) = established_edge();
    // Only Alice's half arrives (plus the ceremony facts).
    let log = log_from(&corpus[..3]);
    let state = log.fold();

    assert!(state.edges().is_empty(), "a single half must never surface as an edge");
    assert_eq!(state.pending_halves().len(), 1, "the lone half folds to pending");
    assert_eq!(state.pending_halves()[0].core_hash, core_hash);
    assert!(state.edge_by_core(&core_hash).is_none(), "no query surfaces a pending half as an edge");
    // The edge-list disclosure surface is empty too — pending is never partial.
    assert!(state.edge_list(&w.p1a.id, &w.p2.id).is_empty());
    assert!(state.edge_list(&w.p1a.id, &w.p1a.id).is_empty());
}

// ---------------------------------------------------------------------------
// T-AT1.2 — both halves referencing the same core hash → edge exists
// ---------------------------------------------------------------------------

#[test]
fn edge_iff_matching_core_hash() {
    let (w, corpus, core_hash) = established_edge();
    let state = log_from(&corpus).fold();

    assert_eq!(state.edges().len(), 1, "matching halves must fold to exactly one edge");
    let edge = state.edge_by_core(&core_hash).expect("edge exists");
    assert!(matches!(edge.status, EdgeStatus::Established));
    let mut parts = edge.participants();
    parts.sort();
    let mut expect = [w.p1a.id, w.p2.id];
    expect.sort();
    assert_eq!(parts, expect);
    assert!(state.pending_halves().is_empty(), "matched halves are no longer pending");
}

// ---------------------------------------------------------------------------
// T-AT1.3 — a tampered core never edges; no error-verdict, just the facts
// ---------------------------------------------------------------------------

#[test]
fn core_mismatch_never_edges() {
    let w = World::new();
    let core_a = edge_core(w.p1a.id, w.p2.id, [0x13; 16], vec![]);
    // Bob's side carries a TAMPERED core: same pair, same nonce, but consent
    // mode flipped to unilateral_notice. Built by hand because the canonical
    // builder refuses to make a non-mutual edge core.
    let mut core_b = core_a.clone();
    core_b.consent = ConsentMode::UnilateralNotice;
    assert_ne!(core_a.core_hash(), core_b.core_hash(), "tamper must change the core hash");

    let half_a = w.p1a.emit(vec![], edge_half(core_a.clone(), "friend"));
    let half_b = w.p2.emit(vec![], edge_half(core_b, "friend"));

    // Both halves APPEND cleanly — the log records facts, it does not verdict.
    let state = log_from(&[half_a, half_b]).fold();

    assert!(state.edges().is_empty(), "mismatched cores must never fold to an edge");
    assert_eq!(state.pending_halves().len(), 2, "two pendings, forever — the fold states the facts");
    assert!(state.edge_by_core(&core_a.core_hash()).is_none());
}

// ---------------------------------------------------------------------------
// T-AT1.4 — labels are side-local
// ---------------------------------------------------------------------------

#[test]
fn labels_are_side_local() {
    let (w, corpus, core_hash) = established_edge();
    let state = log_from(&corpus).fold();
    let edge = state.edge_by_core(&core_hash).expect("edge exists despite differing labels");

    let side_a = edge.side(&w.p1a.id).expect("side a");
    let side_b = edge.side(&w.p2.id).expect("side b");
    assert_eq!(side_a.label, "friend from school", "each side's fold shows its own label");
    assert_eq!(side_b.label, "roommate's sister");
    assert_ne!(side_a.label, side_b.label, "fixture must actually differ");

    // The label is OUTSIDE the core: rebuilding either half with any other
    // label yields the same core hash.
    let relabeled = EdgeHalf {
        core: match &corpus[2].payload {
            Payload::EdgeHalf(h) => h.core.clone(),
            _ => unreachable!(),
        },
        label: "completely different".into(),
    };
    assert_eq!(relabeled.core.core_hash(), core_hash);
}

// ---------------------------------------------------------------------------
// T-AT1.5 — superseding an edge leaves the old edge in lineage
// ---------------------------------------------------------------------------

#[test]
fn edge_supersede_lineage() {
    let (w, corpus, core_hash) = established_edge();
    let half_a_id = corpus[2].object_id();
    let half_b_id = corpus[3].object_id();
    let mut log = log_from(&corpus);
    let prior_bytes = log.object_bytes(&half_a_id).unwrap().to_vec();

    let dis = w.p2.emit(
        vec![half_a_id, half_b_id],
        edge_dissolve(core_hash, vec![half_a_id, half_b_id]),
    );
    let dis_id = log.append(dis).expect("append dissolve");

    let state = log.fold();
    let edge = state.edge_by_core(&core_hash).expect("edge stays in lineage");
    assert_eq!(edge.status, EdgeStatus::Superseded { by: dis_id }, "current view shows superseded");
    // T-AT0.3 invariant holds here too.
    assert_eq!(log.object_bytes(&half_a_id).unwrap(), &prior_bytes[..]);
}

// ---------------------------------------------------------------------------
// T-AT1.6 — property: all arrival permutations reach identical fold state
// ---------------------------------------------------------------------------

#[test]
fn order_independent_fold() {
    let (w, corpus, core_hash) = established_edge();
    let half_a_id = corpus[2].object_id();
    let half_b_id = corpus[3].object_id();
    let dis = w.p1a.emit(
        vec![half_a_id, half_b_id],
        edge_dissolve(core_hash, vec![half_a_id, half_b_id]),
    );

    // Permute {half A, half B, supersede} (ceremony facts lead, fixed) — the
    // spec's set — and additionally permute the full five-object corpus.
    let mut arrivals = vec![corpus[2].clone(), corpus[3].clone(), dis.clone()];
    arrivals.extend_from_slice(&corpus[..2]);

    let mut projections = std::collections::BTreeSet::new();
    let all = permutations(&arrivals);
    assert_eq!(all.len(), 120, "5 arrivals → 120 permutations");
    for perm in &all {
        let state = log_from(perm).fold();
        let edge = state.edge_by_core(&core_hash).expect("edge exists in every order");
        let proj = format!(
            "{:?}|{:?}|{}|{}",
            edge.status,
            edge.grade,
            state.pending_halves().len(),
            state
                .fold_order()
                .iter()
                .map(|o| format!("{o}"))
                .collect::<Vec<_>>()
                .join(",")
        );
        projections.insert(proj);
    }
    assert_eq!(
        projections.len(),
        1,
        "every arrival permutation must fold to the identical state: {projections:?}"
    );
}

// ---------------------------------------------------------------------------
// T-AT1.7 — ceremony grade: co-presence → in_person, else remote
// ---------------------------------------------------------------------------

#[test]
fn ceremony_grade_in_person() {
    // With a co-presence session fact signed by BOTH in the same session.
    let (_, corpus, core_hash) = established_edge();
    let state = log_from(&corpus).fold();
    assert_eq!(
        state.edge_by_core(&core_hash).unwrap().grade,
        Grade::InPerson,
        "both-signed same-session ceremony yields in_person"
    );

    // Without it (no ceremony facts): grade remote.
    let w2 = World::new();
    let core = edge_core(w2.p1a.id, w2.p2.id, [0x17; 16], vec![]);
    let ch = core.core_hash();
    let ha = w2.p1a.emit(vec![], edge_half(core.clone(), "x"));
    let hb = w2.p2.emit(vec![], edge_half(core, "y"));
    let state2 = log_from(&[ha, hb]).fold();
    assert_eq!(state2.edge_by_core(&ch).unwrap().grade, Grade::Remote);

    // One-sided ceremony (only Alice signed a session fact): still remote —
    // co-presence needs both.
    let w3 = World::new();
    let session = [0x77; 16];
    let cer_a = w3.p1a.emit(vec![], ceremony_fact(session, w3.p1a.id, w3.p2.id, d(2026, 7, 17)));
    let core3 = edge_core(w3.p1a.id, w3.p2.id, [0x18; 16], vec![cer_a.object_id()]);
    let ch3 = core3.core_hash();
    let ha3 = w3.p1a.emit(vec![cer_a.object_id()], edge_half(core3.clone(), "x"));
    let hb3 = w3.p2.emit(vec![cer_a.object_id()], edge_half(core3, "y"));
    let state3 = log_from(&[cer_a, ha3, hb3]).fold();
    assert_eq!(state3.edge_by_core(&ch3).unwrap().grade, Grade::Remote);

    // Grade is provenance metadata, never a comparison/ordering input: the
    // compile_fail doc-tests on `types::Grade` pin the no-comparison boundary;
    // here we pin that crate code consumes grade only at assignment and
    // serialization (`as_str`) — no branch, comparison, sort, or filter.
    for file in ["src/fold.rs", "src/query.rs", "src/canonical.rs"] {
        let src = crate_source(file);
        for (line_no, line) in code_lines(&src) {
            if !line.to_lowercase().contains("grade") {
                continue;
            }
            let l = line.to_lowercase();
            for forbidden in ["if ", "match ", ".cmp", "sort", "filter", ".min", ".max"] {
                assert!(
                    !l.contains(forbidden),
                    "{file}:{line_no}: grade may not feed `{forbidden}` — grade is metadata only: {line}"
                );
            }
        }
    }
}

// ---------------------------------------------------------------------------
// (floor) markers exist as presentation vocabulary — referenced by Part 2/5
// ---------------------------------------------------------------------------

#[test]
fn marker_vocabulary_is_presentation_only() {
    assert_eq!(Marker::AntecedentSuperseded.as_str(), "antecedent_superseded");
    assert_eq!(Marker::Stale.as_str(), "stale");
    // A freshness dial exists and is a presentation parameter, not state.
    let _ = FreshnessDial { stale_after_days: 90 };
}
