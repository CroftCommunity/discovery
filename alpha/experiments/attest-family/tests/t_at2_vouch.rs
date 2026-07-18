//! Part 2 — EXP-AT2: scoped vouches layered on the edge. T-AT2.*.
//!
//! A vouch is a separate, later, unilateral claim by one edge participant
//! about the other, in a named scope, citing the base edge as antecedent.
//! Vouches supersede independently of the edge.

mod common;

use attest_family::fixtures::*;
use attest_family::fold::{Marker, VouchStatus};
use attest_family::types::*;
use common::*;

fn d(y: u16, m: u8, day: u8) -> DateClaim {
    DateClaim::new(y, m, day)
}

/// Established P1a–P2 edge (remote grade — no ceremony, keeps Part 2 focused).
fn edge_fixture() -> (World, Vec<Envelope>, [u8; 32]) {
    let w = World::new();
    let core = edge_core(w.p1a.id, w.p2.id, [0x20; 16], vec![]);
    let core_hash = core.core_hash();
    let half_a = w.p1a.emit(vec![], edge_half(core.clone(), "colleague"));
    let half_b = w.p2.emit(vec![], edge_half(core, "colleague"));
    (w, vec![half_a, half_b], core_hash)
}

// ---------------------------------------------------------------------------
// T-AT2.1 — a vouch without a qualifying antecedent folds to pending
// ---------------------------------------------------------------------------

// OWNER-CALL: OC-2 DECIDED (V1, 2026-07-18, owner-confirmed in chat) — option
// B: the qualifying antecedent comes from the closed class (co-signed edge,
// transaction, ceremony), not an edge specifically; the transaction-backed
// edge-free case is proven end-to-end in T-A3.1. This test keeps T-AT2.1's
// discipline: zero qualifying antecedents → pending, never standing.
#[test]
fn vouch_requires_qualifying_antecedent() {
    let (w, corpus, core_hash) = edge_fixture();

    // A vouch citing a base edge that resolves: stands.
    let good = w.p1a.emit(
        vec![corpus[0].object_id(), corpus[1].object_id()],
        vouch(w.p2.id, "would hire as contractor", "solid", core_hash, d(2026, 7, 17), None),
    );
    let mut envs = corpus.clone();
    envs.push(good.clone());
    let state = log_from(&envs).fold();
    assert_eq!(state.vouch(&good.object_id()).unwrap().status, VouchStatus::Standing);

    // The same vouch WITHOUT the halves in the log: pending, not standing.
    let lone_state = log_from(std::slice::from_ref(&good)).fold();
    assert_eq!(
        lone_state.vouch(&good.object_id()).unwrap().status,
        VouchStatus::Pending,
        "no resolvable base edge → pending, never a standing vouch"
    );

    // A vouch naming a base edge that never existed: pending forever.
    let phantom = w.p1a.emit(
        vec![],
        vouch(w.p2.id, "contractor", "solid", [0xFF; 32], d(2026, 7, 17), None),
    );
    let mut envs2 = corpus.clone();
    envs2.push(phantom.clone());
    let state2 = log_from(&envs2).fold();
    assert_eq!(state2.vouch(&phantom.object_id()).unwrap().status, VouchStatus::Pending);

    // A vouch by a NON-participant citing a real edge: pending too — only an
    // edge participant can layer a vouch on it.
    let outsider = w.p3.emit(
        vec![corpus[0].object_id(), corpus[1].object_id()],
        vouch(w.p2.id, "contractor", "hearsay", core_hash, d(2026, 7, 17), None),
    );
    let mut envs3 = corpus.clone();
    envs3.push(outsider.clone());
    let state3 = log_from(&envs3).fold();
    assert_eq!(state3.vouch(&outsider.object_id()).unwrap().status, VouchStatus::Pending);
}

// ---------------------------------------------------------------------------
// T-AT2.2 — vouch supersede is independent of the edge
// ---------------------------------------------------------------------------

#[test]
fn vouch_supersede_independent() {
    let (w, corpus, core_hash) = edge_fixture();
    let v1 = w.p1a.emit(
        vec![corpus[0].object_id(), corpus[1].object_id()],
        vouch(w.p2.id, "would hire as contractor", "any job", core_hash, d(2026, 7, 1), None),
    );
    let mut envs = corpus.clone();
    envs.push(v1.clone());
    let mut log = log_from(&envs);
    let edges_before = log.fold().edges().to_vec();

    // Narrow the vouch (supersede), then withdraw the narrowed one.
    let v2 = w.p1a.emit(
        vec![v1.object_id()],
        vouch(
            w.p2.id,
            "would hire as contractor",
            "small jobs only",
            core_hash,
            d(2026, 7, 10),
            Some(v1.object_id()),
        ),
    );
    log.append(v2.clone()).unwrap();
    let wd = w.p1a.emit(vec![v2.object_id()], vouch_withdraw(v2.object_id()));
    log.append(wd.clone()).unwrap();

    let state = log.fold();
    // The vouch lineage moved: v1 superseded by v2, v2 withdrawn by wd.
    assert_eq!(
        state.vouch(&v1.object_id()).unwrap().status,
        VouchStatus::Superseded { by: v2.object_id() }
    );
    assert_eq!(
        state.vouch(&v2.object_id()).unwrap().status,
        VouchStatus::Withdrawn { by: wd.object_id() }
    );
    assert_eq!(state.vouch(&v1.object_id()).unwrap().lineage, vec![
        v1.object_id(),
        v2.object_id(),
        wd.object_id()
    ]);

    // The EDGE fold is byte-identical before and after the vouch lineage moved.
    let edges_after = state.edges().to_vec();
    assert_eq!(edges_before, edges_after, "vouch supersede must not touch the edge fold");
}

// ---------------------------------------------------------------------------
// T-AT2.3 — edge supersede marks dependent vouches; never auto-withdraws
// ---------------------------------------------------------------------------

// OWNER-CALL: OC-3 DECIDED (V2, 2026-07-18, owner-confirmed in chat) —
// persist-with-marker RATIFIED, with the marker made kind-specific
// (`edge_superseded`, T-A3.5) and the tier boundary clarified: Drystone-tier
// withdrawal is supersede + absence (T-A3.6); authoritative-layer claw-back
// is the ATProto tier's mechanism (the Part B brief). No wall-clock anywhere;
// the private review/remediation mechanism is parked separately.
#[test]
fn edge_supersede_marks_vouches() {
    let (w, corpus, core_hash) = edge_fixture();
    let v = w.p1a.emit(
        vec![corpus[0].object_id(), corpus[1].object_id()],
        vouch(w.p2.id, "would hire as contractor", "solid", core_hash, d(2026, 7, 1), None),
    );
    let mut envs = corpus.clone();
    envs.push(v.clone());
    let mut log = log_from(&envs);
    let v_bytes = log.object_bytes(&v.object_id()).unwrap().to_vec();

    // Before the dissolve: standing, no marker.
    let before = log.fold();
    assert_eq!(before.vouch(&v.object_id()).unwrap().status, VouchStatus::Standing);
    assert!(before.vouch(&v.object_id()).unwrap().markers.is_empty());

    // Supersede the base edge.
    let dis = w.p2.emit(
        vec![corpus[0].object_id(), corpus[1].object_id()],
        edge_dissolve(core_hash, vec![corpus[0].object_id(), corpus[1].object_id()]),
    );
    log.append(dis).unwrap();

    let after = log.fold();
    let view = after.vouch(&v.object_id()).unwrap();
    // Intact object, unchanged bytes, NOT withdrawn — no verdict by side effect.
    assert_eq!(log.object_bytes(&v.object_id()).unwrap(), &v_bytes[..]);
    assert_eq!(
        view.status,
        VouchStatus::Standing,
        "a vouch is never auto-withdrawn by its edge's supersession"
    );
    // The fold gains presentation metadata instead.
    assert_eq!(view.markers, vec![Marker::EdgeSuperseded]);
}

// ---------------------------------------------------------------------------
// T-AT2.4 — a transaction antecedent raises the grade (metadata only)
// ---------------------------------------------------------------------------

#[test]
fn transaction_antecedent_raises_grade() {
    let (w, corpus, core_hash) = edge_fixture();
    // P1a transacted with P2 (fixture fact — the verified-purchase analog).
    let tx = w.p1a.emit(
        vec![],
        transaction_fact(w.p1a.id, SubjectRef::Persona(w.p2.id), d(2026, 6, 20)),
    );
    let v_tx = w.p1a.emit(
        vec![corpus[0].object_id(), corpus[1].object_id(), tx.object_id()],
        vouch(w.p2.id, "would hire as contractor", "paid twice", core_hash, d(2026, 7, 1), None),
    );
    let v_plain = w.p1a.emit(
        vec![corpus[0].object_id(), corpus[1].object_id()],
        vouch(w.p2.id, "would trust with keys", "neighbor", core_hash, d(2026, 7, 2), None),
    );
    let mut envs = corpus.clone();
    envs.extend([tx.clone(), v_tx.clone(), v_plain.clone()]);
    let state = log_from(&envs).fold();

    assert_eq!(
        state.vouch(&v_tx.object_id()).unwrap().grade,
        Grade::TransactionBacked,
        "citing a transaction attestation as antecedent carries grade transaction_backed"
    );
    assert_eq!(
        state.vouch(&v_plain.object_id()).unwrap().grade,
        Grade::Remote,
        "without a transaction antecedent the vouch inherits the base edge's grade"
    );
    // Both stand — grade is metadata, not a gate (same rule as T-AT1.7).
    assert_eq!(state.vouch(&v_tx.object_id()).unwrap().status, VouchStatus::Standing);
    assert_eq!(state.vouch(&v_plain.object_id()).unwrap().status, VouchStatus::Standing);

    // A review citing a transaction is graded the same way.
    let decl = w.p3.emit(vec![], thing_decl(w.biz1, ThingKind::Business, w.p3.id));
    let tx2 = w.p2.emit(
        vec![],
        transaction_fact(w.p2.id, SubjectRef::Thing(w.biz1), d(2026, 6, 25)),
    );
    let rv = w.p2.emit(
        vec![tx2.object_id()],
        review(SubjectRef::Thing(w.biz1), "plumbing", "fixed it", d(2026, 7, 3), None),
    );
    let state2 = log_from(&[decl, tx2, rv.clone()]).fold();
    assert_eq!(state2.review(&rv.object_id()).unwrap().grade, Grade::TransactionBacked);
}
