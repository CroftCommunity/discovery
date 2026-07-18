//! RUN-ATTEST-03 Part A — settlement riders for the first three owner-call
//! verdicts of the 2026-07-18 walk. T-A3.1–T-A3.3 (V1: the closed antecedent
//! class), T-A3.5–T-A3.6 (V2: marker precision + absence-not-tombstone).
//! T-A3.4 (the governed register) lives in `t_a3_register.rs` with the
//! substrate dev-deps.
//!
//! V1 (AT OC-2 → option B): a vouch requires a qualifying antecedent from a
//! CLOSED class — co-signed edge, transaction attestation, or ceremony fact —
//! not an edge specifically. These are shapes of one provenance mechanism;
//! what varies is the kind of trust bound — bidirectional (edge) or
//! unidirectional (transaction, ceremony) — and both are valid.
//!
//! V2 (AT OC-3 ratified + tier clarification): persist-with-marker stands,
//! the marker is kind-specific (`edge_superseded`), and Drystone-tier
//! withdrawal is supersede + ABSENCE from corroboration — no tombstone.
//! Authoritative-layer removal is the ATProto tier's job (the Part B brief).

mod common;

use attest_family::fixtures::*;
use attest_family::fold::{Marker, VouchStatus};
use attest_family::query::FreshnessDial;
use attest_family::types::*;
use common::*;
use ipld_core::ipld::Ipld;

fn d(y: u16, m: u8, day: u8) -> DateClaim {
    DateClaim::new(y, m, day)
}

fn dial() -> FreshnessDial {
    FreshnessDial { stale_after_days: 3650 }
}

fn as_of() -> DateClaim {
    d(2026, 7, 18)
}

/// Established P1a–P2 edge (remote grade), same shape as t_at2's fixture.
fn edge_fixture() -> (World, Vec<Envelope>, [u8; 32]) {
    let w = World::new();
    let core = edge_core(w.p1a.id, w.p2.id, [0xA3; 16], vec![]);
    let core_hash = core.core_hash();
    let half_a = w.p1a.emit(vec![], edge_half(core.clone(), "client"));
    let half_b = w.p2.emit(vec![], edge_half(core, "client"));
    (w, vec![half_a, half_b], core_hash)
}

// ---------------------------------------------------------------------------
// T-A3.1 — THE red for Part A.1: the plumber case, end to end
// ---------------------------------------------------------------------------

/// P1a hired P2 (a plumber who is a persona, not a listed business) and paid —
/// the fixture transaction fact. No edge exists between them and none ever
/// will; a one-off trade is not a mutual relationship. Under the pre-change
/// fold this vouch was PENDING (that failing state is the captured red);
/// under V1 it stands with grade `transaction_backed`.
#[test]
fn plumber_case_red() {
    let w = World::new();
    let tx = w.p1a.emit(
        vec![],
        transaction_fact(w.p1a.id, SubjectRef::Persona(w.p2.id), d(2026, 6, 20)),
    );
    let v = w.p1a.emit(
        vec![tx.object_id()],
        vouch_edge_free(w.p2.id, "would hire as plumber", "fixed the leak, fair price", d(2026, 7, 1), None),
    );
    let state = log_from(&[tx.clone(), v.clone()]).fold();

    let view = state.vouch(&v.object_id()).expect("vouch folds");
    assert_eq!(
        view.status,
        VouchStatus::Standing,
        "V1: a transaction-backed, edge-free vouch STANDS (pre-change: pending — the red)"
    );
    assert_eq!(view.base_edge, None, "genuinely edge-free");
    assert_eq!(view.antecedent_kinds, vec![AntecedentKind::Transaction]);
    assert_eq!(view.grades, vec![Grade::TransactionBacked]);
    assert_eq!(view.grade, Grade::TransactionBacked, "legacy single grade agrees");

    // End to end: the vouch corroborates for a third viewer, carrying the
    // kind-derived grade set.
    let resp = state.corroboration(
        &w.p3.id,
        &SubjectRef::Persona(w.p2.id),
        &Scope::new("would hire as plumber"),
        &dial(),
        as_of(),
    );
    assert_eq!(resp.entries.len(), 1, "the plumber vouch reaches the corroboration structure");
    assert_eq!(resp.entries[0].attestation, v.object_id());
    assert_eq!(resp.entries[0].grades, vec![Grade::TransactionBacked]);

    // And the withdraw path works the same as for edge-backed vouches — the
    // vouch supersedes independently of its antecedent (T-AT2.2 discipline).
    let mut log = log_from(&[tx, v.clone()]);
    let wd = w.p1a.emit(vec![v.object_id()], vouch_withdraw(v.object_id()));
    log.append(wd.clone()).unwrap();
    let after = log.fold();
    assert_eq!(
        after.vouch(&v.object_id()).unwrap().status,
        VouchStatus::Withdrawn { by: wd.object_id() }
    );
}

// ---------------------------------------------------------------------------
// T-A3.2 — the antecedent class is closed (compile-boundary + fold)
// ---------------------------------------------------------------------------

/// The class is closed at TWO boundaries: the compile boundary (no string
/// escape hatch — the compile_fail doc-tests on `types::AntecedentKind`), and
/// the fold (zero qualifying antecedents → pending, never standing — T-AT2.1's
/// discipline restated; that test is renamed `vouch_requires_qualifying_antecedent`).
#[test]
fn antecedent_class_is_closed() {
    // (a) Vocabulary boundary: exactly the three decided kinds exist; an
    // unknown kind is not decodable into the class.
    for (kind, s) in [
        (AntecedentKind::CoSignedEdge, "co_signed_edge"),
        (AntecedentKind::Transaction, "transaction"),
        (AntecedentKind::Ceremony, "ceremony"),
    ] {
        assert_eq!(kind.as_str(), s);
        assert_eq!(AntecedentKind::from_str(s), Some(kind));
    }
    assert_eq!(AntecedentKind::from_str("notarized_selfie"), None);
    assert_eq!(AntecedentKind::from_str("edge"), None, "no aliases either");

    // (b) Source boundary: the enum region carries unit variants only — no
    // variant payload, no String, nothing that could smuggle an open kind.
    let src = crate_source("src/types.rs");
    let start = src.find("pub enum AntecedentKind").expect("closed enum exists");
    let end = start + src[start..].find('}').expect("enum region closes");
    for (line_no, line) in code_lines(&src[start..end]) {
        assert!(
            !line.contains('(') && !line.contains("String") && !line.contains("Vec<"),
            "AntecedentKind region line {line_no} is not a bare unit variant: {line}"
        );
    }

    // (c) Fold boundary: zero qualifying antecedents → pending, in every
    // near-miss shape.
    let w = World::new();
    // No antecedents at all.
    let bare = w.p1a.emit(
        vec![],
        vouch_edge_free(w.p2.id, "plumber", "nice person", d(2026, 7, 1), None),
    );
    let s1 = log_from(std::slice::from_ref(&bare)).fold();
    assert_eq!(s1.vouch(&bare.object_id()).unwrap().status, VouchStatus::Pending);

    // A transaction whose PAYER is not the vouch author does not qualify —
    // somebody else's purchase is not this author's trust bind.
    let tx_other = w.p3.emit(
        vec![],
        transaction_fact(w.p3.id, SubjectRef::Persona(w.p2.id), d(2026, 6, 1)),
    );
    let v_other_payer = w.p1a.emit(
        vec![tx_other.object_id()],
        vouch_edge_free(w.p2.id, "plumber", "heard good things", d(2026, 7, 1), None),
    );
    let s2 = log_from(&[tx_other, v_other_payer.clone()]).fold();
    let view2 = s2.vouch(&v_other_payer.object_id()).unwrap();
    assert_eq!(view2.status, VouchStatus::Pending);
    assert_eq!(view2.antecedent_kinds, Vec::<AntecedentKind>::new());

    // A transaction with a different PAYEE does not qualify — paying a third
    // party binds nothing about this subject.
    let tx_wrong_payee = w.p1a.emit(
        vec![],
        transaction_fact(w.p1a.id, SubjectRef::Persona(w.p3.id), d(2026, 6, 1)),
    );
    let v_wrong_payee = w.p1a.emit(
        vec![tx_wrong_payee.object_id()],
        vouch_edge_free(w.p2.id, "plumber", "solid", d(2026, 7, 1), None),
    );
    let s3 = log_from(&[tx_wrong_payee, v_wrong_payee.clone()]).fold();
    assert_eq!(s3.vouch(&v_wrong_payee.object_id()).unwrap().status, VouchStatus::Pending);

    // A ceremony fact naming a DIFFERENT pair does not qualify.
    let cer_wrong = w.p1a.emit(
        vec![],
        ceremony_fact([0x99; 16], w.p1a.id, w.p3.id, d(2026, 6, 15)),
    );
    let v_wrong_pair = w.p1a.emit(
        vec![cer_wrong.object_id()],
        vouch_edge_free(w.p2.id, "plumber", "met once", d(2026, 7, 1), None),
    );
    let s4 = log_from(&[cer_wrong, v_wrong_pair.clone()]).fold();
    assert_eq!(s4.vouch(&v_wrong_pair.object_id()).unwrap().status, VouchStatus::Pending);

    // A qualifying ceremony fact DOES stand the vouch up (the third kind of
    // the class, proven qualifying so (c) is not vacuous).
    let cer = w.p1a.emit(
        vec![],
        ceremony_fact([0x9A; 16], w.p1a.id, w.p2.id, d(2026, 6, 15)),
    );
    let v_cer = w.p1a.emit(
        vec![cer.object_id()],
        vouch_edge_free(w.p2.id, "plumber", "watched them work", d(2026, 7, 1), None),
    );
    let s5 = log_from(&[cer, v_cer.clone()]).fold();
    let view5 = s5.vouch(&v_cer.object_id()).unwrap();
    assert_eq!(view5.status, VouchStatus::Standing);
    assert_eq!(view5.antecedent_kinds, vec![AntecedentKind::Ceremony]);
}

// ---------------------------------------------------------------------------
// T-A3.3 — grade derives from kind: kind set ↔ grade set, exact
// ---------------------------------------------------------------------------

#[test]
fn grade_derives_from_kind() {
    let (w, corpus, core_hash) = edge_fixture();
    let tx = w.p1a.emit(
        vec![],
        transaction_fact(w.p1a.id, SubjectRef::Persona(w.p2.id), d(2026, 6, 20)),
    );
    let cer = w.p1a.emit(
        vec![],
        ceremony_fact([0x9B; 16], w.p1a.id, w.p2.id, d(2026, 6, 21)),
    );

    // A multi-antecedent vouch: edge + transaction + ceremony, all qualifying.
    let v_all = w.p1a.emit(
        vec![corpus[0].object_id(), corpus[1].object_id(), tx.object_id(), cer.object_id()],
        vouch(w.p2.id, "would hire as contractor", "every bind at once", core_hash, d(2026, 7, 1), None),
    );
    // Single-kind vouches for each class member.
    let v_edge = w.p1a.emit(
        vec![corpus[0].object_id(), corpus[1].object_id()],
        vouch(w.p2.id, "would trust with keys", "neighbor", core_hash, d(2026, 7, 2), None),
    );
    let v_tx = w.p1a.emit(
        vec![tx.object_id()],
        vouch_edge_free(w.p2.id, "would hire as plumber", "paid twice", d(2026, 7, 3), None),
    );
    let v_cer = w.p1a.emit(
        vec![cer.object_id()],
        vouch_edge_free(w.p2.id, "met in person", "shared a session", d(2026, 7, 4), None),
    );

    let mut envs = corpus.clone();
    envs.extend([tx, cer, v_all.clone(), v_edge.clone(), v_tx.clone(), v_cer.clone()]);
    let state = log_from(&envs).fold();

    // The multi-antecedent vouch carries the SET, in declaration order.
    let all = state.vouch(&v_all.object_id()).unwrap();
    assert_eq!(all.status, VouchStatus::Standing);
    assert_eq!(
        all.antecedent_kinds,
        vec![AntecedentKind::CoSignedEdge, AntecedentKind::Transaction, AntecedentKind::Ceremony]
    );
    assert_eq!(
        all.grades,
        vec![Grade::EdgeBacked, Grade::TransactionBacked, Grade::CeremonyBacked]
    );

    // Kind set ↔ grade set is EXACT: one grade per kind, by the closed map.
    for id in [&v_all, &v_edge, &v_tx, &v_cer] {
        let view = state.vouch(&id.object_id()).unwrap();
        let derived: Vec<Grade> = view.antecedent_kinds.iter().map(|k| k.grade()).collect();
        assert_eq!(view.grades, derived, "grade set must be exactly the kind set's image");
    }
    assert_eq!(state.vouch(&v_edge.object_id()).unwrap().grades, vec![Grade::EdgeBacked]);
    assert_eq!(state.vouch(&v_tx.object_id()).unwrap().grades, vec![Grade::TransactionBacked]);
    assert_eq!(state.vouch(&v_cer.object_id()).unwrap().grades, vec![Grade::CeremonyBacked]);

    // Grades remain metadata-only — the no-consumption scan, re-asserted
    // (T-AT1.7's rule over the modules that touch grades).
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

    // Serialization carries the set as strings; the numeric-leaf walk stays
    // date-only (no new scalar snuck in with the set).
    let resp = state.corroboration(
        &w.p3.id,
        &SubjectRef::Persona(w.p2.id),
        &Scope::new("would hire as contractor"),
        &dial(),
        as_of(),
    );
    assert_eq!(resp.entries.len(), 1);
    let Ipld::Map(entry) = resp.entries[0].to_ipld() else { panic!("entry serializes as map") };
    assert_eq!(
        entry.get("j"),
        Some(&Ipld::List(vec![
            Ipld::String("edge_backed".into()),
            Ipld::String("transaction_backed".into()),
            Ipld::String("ceremony_backed".into()),
        ]))
    );
    let mut numerics = Vec::new();
    ipld_numeric_leaves(&resp.to_ipld(), "", &mut numerics);
    for (path, val) in &numerics {
        let leaf_key = path.rsplit('.').next().unwrap_or(path);
        assert!(
            matches!(leaf_key, "d" | "m" | "y"),
            "numeric leaf outside a date claim: {path} = {val}"
        );
    }
}

// ---------------------------------------------------------------------------
// T-A3.5 — only edges supersede (V2: the marker is kind-specific)
// ---------------------------------------------------------------------------

/// A completed transaction has no "ended" state; neither has a co-presence
/// session that happened. The superseded-antecedent marker is therefore
/// UNREPRESENTABLE for transaction- and ceremony-backed vouches — the
/// compile_fail doc-tests on `types::TransactionFact` / `types::CeremonyFact`
/// pin that no supersede field exists, and this test pins that no fold path
/// can attach the marker from anything but a folded edge's supersession.
#[test]
fn only_edges_supersede() {
    let (w, corpus, core_hash) = edge_fixture();
    let tx = w.p1a.emit(
        vec![],
        transaction_fact(w.p1a.id, SubjectRef::Persona(w.p2.id), d(2026, 6, 20)),
    );
    let cer = w.p1a.emit(
        vec![],
        ceremony_fact([0x9C; 16], w.p1a.id, w.p2.id, d(2026, 6, 21)),
    );
    let v_tx = w.p1a.emit(
        vec![tx.object_id()],
        vouch_edge_free(w.p2.id, "plumber", "paid", d(2026, 7, 1), None),
    );
    let v_cer = w.p1a.emit(
        vec![cer.object_id()],
        vouch_edge_free(w.p2.id, "met in person", "shared a session", d(2026, 7, 2), None),
    );
    let v_edge = w.p1a.emit(
        vec![corpus[0].object_id(), corpus[1].object_id()],
        vouch(w.p2.id, "would trust with keys", "neighbor", core_hash, d(2026, 7, 3), None),
    );

    let mut envs = corpus.clone();
    envs.extend([tx.clone(), cer.clone(), v_tx.clone(), v_cer.clone(), v_edge.clone()]);
    let mut log = log_from(&envs);

    // Adversarial attempts to "end" the unidirectional antecedents: an
    // edge-dissolve naming the transaction's object id, and one naming the
    // ceremony's, as if they were edge core hashes. Plus the REAL edge's
    // dissolve (which is legitimate — for the edge-backed vouch only).
    let fake_dis_tx = w.p1a.emit(vec![], edge_dissolve(tx.object_id().0, vec![tx.object_id()]));
    let fake_dis_cer = w.p1a.emit(vec![], edge_dissolve(cer.object_id().0, vec![cer.object_id()]));
    let real_dis = w.p2.emit(
        vec![corpus[0].object_id(), corpus[1].object_id()],
        edge_dissolve(core_hash, vec![corpus[0].object_id(), corpus[1].object_id()]),
    );
    log.append(fake_dis_tx).unwrap();
    log.append(fake_dis_cer).unwrap();
    log.append(real_dis).unwrap();

    let state = log.fold();
    // The tx- and ceremony-backed vouches: still standing, ZERO markers —
    // there is nothing whose supersession could mark them.
    for id in [&v_tx, &v_cer] {
        let view = state.vouch(&id.object_id()).unwrap();
        assert_eq!(view.status, VouchStatus::Standing);
        assert_eq!(
            view.markers,
            Vec::<Marker>::new(),
            "a completed transaction/ceremony has no ended state — no marker is representable"
        );
    }
    // The edge-backed vouch: marked (kind-specificity contrast) — the marker
    // names exactly what ended, the EDGE.
    assert_eq!(state.vouch(&v_edge.object_id()).unwrap().markers, vec![Marker::EdgeSuperseded]);

    // Unrepresentable, not merely unset: the fold has exactly ONE attach site
    // for the marker, and it is guarded by the folded EDGE's supersession —
    // no code path reaches the marker from a transaction or ceremony fact.
    let src = crate_source("src/fold.rs");
    let lines = code_lines(&src);
    let push_sites: Vec<usize> = lines
        .iter()
        .enumerate()
        .filter(|(_, (_, l))| l.contains("push(Marker::EdgeSuperseded)"))
        .map(|(i, _)| i)
        .collect();
    assert_eq!(push_sites.len(), 1, "exactly one marker attach site exists");
    assert!(
        lines[push_sites[0] - 1].1.contains("base_superseded"),
        "the single attach site is guarded by the base EDGE's supersession"
    );
}

// ---------------------------------------------------------------------------
// T-A3.6 — withdrawn is ABSENT, not tombstoned (V2's Drystone-tier boundary)
// ---------------------------------------------------------------------------

/// An author-superseded (withdrawn) vouch is ABSENT from every corroboration
/// structure: no tombstone field, no count, no "something was here." Lineage
/// retains the objects (T-AT0.3) — absence is a presentation/query guarantee.
/// That is exactly the V2 tier boundary: authoritative-layer removal is the
/// ATProto tier's job (author deletes the record from their own PDS, the
/// Part B brief); Drystone-tier withdrawal is supersede + absence.
///
/// RED (refutation-pin style): a `withdrawn: bool` tombstone field was staged
/// on `CorroborationEntry` (withdrawn vouches included, flagged) — the
/// masked-equality, trace, and source-scan assertions below all failed
/// against it; the staging was deleted at green.
#[test]
fn withdrawn_is_absent_not_tombstoned() {
    // Two worlds, identical except world A's P1a vouch is made-then-withdrawn
    // and world B's never existed. P3's vouch exists in both.
    let build = |with_withdrawn: bool| -> (World, attest_family::fold::AttestLog, Option<ObjectId>) {
        let w = World::new();
        let core_a = edge_core(w.p1a.id, w.p2.id, [0xA6; 16], vec![]);
        let core_b = edge_core(w.p3.id, w.p2.id, [0xA7; 16], vec![]);
        let ha1 = w.p1a.emit(vec![], edge_half(core_a.clone(), "friend"));
        let ha2 = w.p2.emit(vec![], edge_half(core_a.clone(), "friend"));
        let hb1 = w.p3.emit(vec![], edge_half(core_b.clone(), "cousin"));
        let hb2 = w.p2.emit(vec![], edge_half(core_b.clone(), "cousin"));
        let v3 = w.p3.emit(
            vec![hb1.object_id(), hb2.object_id()],
            vouch(w.p2.id, "would hire as contractor", "fixed my roof", core_b.core_hash(), d(2026, 7, 2), None),
        );
        let mut envs = vec![ha1.clone(), ha2, hb1, hb2, v3];
        let mut withdrawn_id = None;
        if with_withdrawn {
            let va = w.p1a.emit(
                vec![ha1.object_id()],
                vouch(w.p2.id, "would hire as contractor", "rebuilt my porch", core_a.core_hash(), d(2026, 7, 1), None),
            );
            let wd = w.p1a.emit(vec![va.object_id()], vouch_withdraw(va.object_id()));
            withdrawn_id = Some(va.object_id());
            envs.push(va);
            envs.push(wd);
        }
        (w, log_from(&envs), withdrawn_id)
    };

    let (wa, log_a, withdrawn) = build(true);
    let (wb, log_b, _) = build(false);
    let withdrawn = withdrawn.unwrap();
    let state_a = log_a.fold();
    let state_b = log_b.fold();

    let scope = Scope::new("would hire as contractor");
    let resp_a = state_a.corroboration(&wa.p2.id, &SubjectRef::Persona(wa.p2.id), &scope, &dial(), as_of());
    let resp_b = state_b.corroboration(&wb.p2.id, &SubjectRef::Persona(wb.p2.id), &scope, &dial(), as_of());

    // Masked-structural equality: the withdrawn world's structure is
    // indistinguishable from the never-was world's — no count, no slot, no
    // "something was here."
    assert_eq!(
        masked_form(&resp_a.to_ipld()),
        masked_form(&resp_b.to_ipld()),
        "a withdrawn vouch must leave NO structural residue in corroboration"
    );
    assert_eq!(resp_a.entries.len(), 1, "only P3's standing vouch remains");

    // No trace of the withdrawn object anywhere in the serialized response.
    let bytes = resp_a.to_canonical_bytes();
    assert!(!contains_subslice(&bytes, &withdrawn.0), "no withdrawn-object id trace");
    for needle in [b"withdrawn".as_slice(), b"tombstone".as_slice()] {
        assert!(!contains_subslice(&bytes, needle), "no tombstone vocabulary in the response");
    }

    // Source boundary: the response types carry no tombstone-shaped field.
    let src = crate_source("src/query.rs");
    for (line_no, line) in code_lines(&src) {
        let l = line.to_lowercase();
        for bad in ["withdrawn", "tombstone", "redacted", "deleted"] {
            assert!(
                !l.contains(bad),
                "src/query.rs:{line_no}: tombstone-shaped surface in the query module: {line}"
            );
        }
    }

    // The review half: an author-superseded (amended) review is absent AS AN
    // ENTRY — its replacement stands alone. (The replacement's lineage
    // pointers are supersede ancestry, T-AT0.3's retention — deliberately
    // visible; absence applies to the superseded object's own entry.)
    let w = World::new();
    let decl = w.p3.emit(vec![], thing_decl(w.biz1, ThingKind::Business, w.p3.id));
    let r1 = w.p1a.emit(
        vec![],
        review(SubjectRef::Thing(w.biz1), "plumbing", "first take", d(2026, 6, 1), None),
    );
    let r2 = w.p1a.emit(
        vec![r1.object_id()],
        review(SubjectRef::Thing(w.biz1), "plumbing", "amended take", d(2026, 6, 5), Some(r1.object_id())),
    );
    let s = log_from(&[decl, r1.clone(), r2.clone()]).fold();
    let resp = s.corroboration(&w.p2.id, &SubjectRef::Thing(w.biz1), &Scope::new("plumbing"), &dial(), as_of());
    let ids: Vec<ObjectId> = resp.entries.iter().map(|e| e.attestation).collect();
    assert_eq!(ids, vec![r2.object_id()], "only the amendment stands as an entry");

    // The V2 boundary, stated as assertions: lineage RETAINS the withdrawn
    // object (bytes unchanged, fold status Withdrawn) — absence is a
    // corroboration/presentation guarantee, not deletion. The authoritative
    // claw-back (author removing the canonical copy from their own PDS) is
    // the public/ATProto tier's mechanism, out of this crate's scope.
    assert!(log_a.object_bytes(&withdrawn).is_some(), "lineage retains the bytes (T-AT0.3)");
    assert!(matches!(
        state_a.vouch(&withdrawn).unwrap().status,
        VouchStatus::Withdrawn { .. }
    ));
}
