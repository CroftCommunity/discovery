//! Part 0 — floor invariants (RUN-ATTEST-01 §4, T-AT0.*).
//!
//! These four invariants hold across the whole crate for the rest of the run:
//! canonical round-trip, no-score-anywhere, supersede-preserves-prior, and
//! ordering-ignores-wallclock.

mod common;

use attest_family::fixtures::*;
use attest_family::query::FreshnessDial;
use attest_family::types::*;
use attest_family::{canonical, fold::EdgeStatus, fold::VouchStatus};
use common::*;

fn d(y: u16, m: u8, day: u8) -> DateClaim {
    DateClaim::new(y, m, day)
}

/// One signed envelope of EVERY attestation kind, from the standard world.
fn one_of_each(w: &World) -> Vec<Envelope> {
    let session = [0x5E; 16];
    let cer_a = w.p1a.emit(vec![], ceremony_fact(session, w.p1a.id, w.p2.id, d(2026, 7, 17)));
    let cer_b = w.p2.emit(vec![], ceremony_fact(session, w.p1a.id, w.p2.id, d(2026, 7, 17)));
    let core = edge_core(
        w.p1a.id,
        w.p2.id,
        [0xED; 16],
        vec![cer_a.object_id(), cer_b.object_id()],
    );
    let core_hash = core.core_hash();
    let half_a = w.p1a.emit(
        vec![cer_a.object_id(), cer_b.object_id()],
        edge_half(core.clone(), "friend from school"),
    );
    let half_b = w.p2.emit(
        vec![cer_a.object_id(), cer_b.object_id()],
        edge_half(core, "roommate's sister"),
    );
    let dissolve = w.p1a.emit(
        vec![half_a.object_id(), half_b.object_id()],
        edge_dissolve(core_hash, vec![half_a.object_id(), half_b.object_id()]),
    );
    let tx = w.p2.emit(
        vec![],
        transaction_fact(w.p2.id, SubjectRef::Thing(w.biz1), d(2026, 7, 1)),
    );
    let decl = w.p3.emit(vec![], thing_decl(w.biz1, ThingKind::Business, w.p3.id));
    let v = w.p1a.emit(
        vec![half_a.object_id(), half_b.object_id()],
        vouch(w.p2.id, "would hire as contractor", "solid work", core_hash, d(2026, 7, 17), None),
    );
    let vw = w.p1a.emit(vec![v.object_id()], vouch_withdraw(v.object_id()));
    let rv = w.p2.emit(
        vec![tx.object_id()],
        review(SubjectRef::Thing(w.biz1), "plumbing", "fixed the leak", d(2026, 7, 2), None),
    );
    let rp = w.p3.emit(vec![rv.object_id()], reply(rv.object_id(), "thanks", d(2026, 7, 3)));
    let pr = w.coop.emit(
        vec![],
        predicate(
            PredicateKind::Over18,
            w.p2.id,
            coop_process(MethodKind::DocumentSighted, d(2026, 7, 17)),
            None,
        ),
    );
    let pol = w.p2.emit(vec![], policy(w.p2.id, PolicyRule::AllowOnly(vec![w.p1a.id]), None));
    vec![cer_a, cer_b, half_a, half_b, dissolve, tx, decl, v, vw, rv, rp, pr, pol]
}

// ---------------------------------------------------------------------------
// T-AT0.1 — every attestation kind round-trips byte-identically
// ---------------------------------------------------------------------------

#[test]
fn canonical_roundtrip() {
    let w = World::new();
    let corpus = one_of_each(&w);
    // All eleven payload kinds are present in the corpus.
    let kinds: std::collections::BTreeSet<&str> =
        corpus.iter().map(|e| e.payload.kind_str()).collect();
    assert_eq!(
        kinds.len(),
        11,
        "the corpus must cover every attestation kind, got {kinds:?}"
    );

    for env in &corpus {
        let bytes = env.canonical_bytes_with_sig();
        let decoded = canonical::decode_envelope(&bytes)
            .unwrap_or_else(|e| panic!("{}: decode failed: {e}", env.payload.kind_str()));
        assert_eq!(&decoded, env, "{}: decode must reproduce the envelope", env.payload.kind_str());
        let re = decoded.canonical_bytes_with_sig();
        assert_eq!(
            re,
            bytes,
            "{}: encode(decode(bytes)) must be byte-identical",
            env.payload.kind_str()
        );
        assert_eq!(decoded.object_id(), env.object_id());
    }
}

// ---------------------------------------------------------------------------
// T-AT0.2 — no numeric trust/score/rating/rank field exists on any public type
// ---------------------------------------------------------------------------

/// Compile-boundary assertion in the EXP-B style, two prongs:
/// (a) source scan: no code line in the crate introduces a score-like name;
/// (b) serialization walk: a populated corroboration response contains no
///     numeric leaf outside date claims (dates are asserted claims, and the
///     mutual-count disclosure's single cardinality is its own type).
/// The compile_fail doc-tests on `types::Grade` pin the no-comparison half.
#[test]
fn no_score_field_exists() {
    // (a) Source scan over every crate source file, code lines only.
    let banned = ["score", "rating", "rank", "trust", "weight", "reputation"];
    for file in ["src/lib.rs", "src/types.rs", "src/canonical.rs", "src/fixtures.rs", "src/fold.rs", "src/query.rs"] {
        let src = crate_source(file);
        for (line_no, line) in code_lines(&src) {
            let lower = line.to_lowercase();
            for b in &banned {
                assert!(
                    !lower.contains(b),
                    "{file}:{line_no}: banned score-like token `{b}` in code line: {line}"
                );
            }
        }
    }

    // (b) A populated response serializes with no numeric leaf outside date
    // claims. (Entries carry grade as a STRING, ids as bytes.)
    let w = World::new();
    let corpus = one_of_each(&w);
    let state = log_from(&corpus).fold();
    // Viewer P1a: the fixture reviewer P2's policy allows P1a, so the review
    // traverses and the response is genuinely populated.
    let resp = state.corroboration(
        &w.p1a.id,
        &SubjectRef::Thing(w.biz1),
        &Scope::new("plumbing"),
        &FreshnessDial { stale_after_days: 3650 },
        DateClaim::new(2026, 7, 18),
    );
    assert!(!resp.entries.is_empty(), "fixture review must corroborate");
    let mut numerics = Vec::new();
    ipld_numeric_leaves(&resp.to_ipld(), "", &mut numerics);
    for (path, val) in &numerics {
        let leaf_key = path.rsplit('.').next().unwrap_or(path);
        assert!(
            matches!(leaf_key, "d" | "m" | "y"),
            "numeric leaf outside a date claim in corroboration response: {path} = {val}"
        );
    }
}

// ---------------------------------------------------------------------------
// T-AT0.3 — supersede preserves the prior object's bytes
// ---------------------------------------------------------------------------

#[test]
fn supersede_preserves_prior() {
    let w = World::new();
    let core = edge_core(w.p1a.id, w.p2.id, [0x03; 16], vec![]);
    let core_hash = core.core_hash();
    let half_a = w.p1a.emit(vec![], edge_half(core.clone(), "a"));
    let half_b = w.p2.emit(vec![], edge_half(core, "b"));
    let v = w.p1a.emit(
        vec![half_a.object_id(), half_b.object_id()],
        vouch(w.p2.id, "contractor", "good", core_hash, DateClaim::new(2026, 7, 1), None),
    );

    let mut log = log_from(&[half_a.clone(), half_b.clone(), v.clone()]);
    let half_a_bytes = log.object_bytes(&half_a.object_id()).expect("half a stored").to_vec();
    let vouch_bytes = log.object_bytes(&v.object_id()).expect("vouch stored").to_vec();

    // Supersede the vouch (narrowed restatement), then the edge (dissolve).
    let v2 = w.p1a.emit(
        vec![v.object_id()],
        vouch(
            w.p2.id,
            "contractor",
            "good, small jobs only",
            core_hash,
            DateClaim::new(2026, 7, 10),
            Some(v.object_id()),
        ),
    );
    log.append(v2.clone()).expect("append supersede");
    let dis = w.p2.emit(
        vec![half_a.object_id(), half_b.object_id()],
        edge_dissolve(core_hash, vec![half_a.object_id(), half_b.object_id()]),
    );
    log.append(dis).expect("append dissolve");

    // The prior objects' bytes are retrievable UNCHANGED.
    assert_eq!(
        log.object_bytes(&half_a.object_id()).expect("half a still stored"),
        &half_a_bytes[..],
        "superseded half's bytes must be unchanged"
    );
    assert_eq!(
        log.object_bytes(&v.object_id()).expect("vouch still stored"),
        &vouch_bytes[..],
        "superseded vouch's bytes must be unchanged"
    );
    // And they are the canonical bytes they were born with.
    assert_eq!(half_a_bytes, half_a.canonical_bytes_with_sig());
    assert_eq!(vouch_bytes, v.canonical_bytes_with_sig());

    // The fold reflects supersession (status), it never deletes (lineage).
    let state = log.fold();
    let edge = state.edge_by_core(&core_hash).expect("edge in state");
    assert!(matches!(edge.status, EdgeStatus::Superseded { .. }));
    let v1_view = state.vouch(&v.object_id()).expect("prior vouch still in state");
    assert!(matches!(v1_view.status, VouchStatus::Superseded { .. }));
    assert_eq!(v1_view.lineage.last(), Some(&v2.object_id()), "lineage points forward");
}

// ---------------------------------------------------------------------------
// T-AT0.4 — ordering ignores wall-clock
// ---------------------------------------------------------------------------

/// Build the same corpus with a caller-chosen set of payload date claims.
/// Returns the envelopes in fixture order (the index is the object's name for
/// cross-corpus comparison).
fn corpus_with_dates(dates: &[DateClaim; 4]) -> (World, Vec<Envelope>) {
    let w = World::new();
    let session = [0x44; 16];
    let cer_a = w.p1a.emit(vec![], ceremony_fact(session, w.p1a.id, w.p2.id, dates[0]));
    let cer_b = w.p2.emit(vec![], ceremony_fact(session, w.p1a.id, w.p2.id, dates[1]));
    let core = edge_core(w.p1a.id, w.p2.id, [0x40; 16], vec![cer_a.object_id(), cer_b.object_id()]);
    let core_hash = core.core_hash();
    let half_a = w.p1a.emit(
        vec![cer_a.object_id(), cer_b.object_id()],
        edge_half(core.clone(), "friend"),
    );
    let half_b = w.p2.emit(
        vec![cer_a.object_id(), cer_b.object_id()],
        edge_half(core, "neighbor"),
    );
    let v = w.p1a.emit(
        vec![half_a.object_id(), half_b.object_id()],
        vouch(w.p2.id, "contractor", "solid", core_hash, dates[2], None),
    );
    let decl = w.p3.emit(vec![], thing_decl(w.biz1, ThingKind::Business, w.p3.id));
    let rv = w.p2.emit(
        vec![],
        review(SubjectRef::Thing(w.biz1), "plumbing", "fine", dates[3], None),
    );
    (w, vec![cer_a, cer_b, half_a, half_b, v, decl, rv])
}

/// A hash-erased projection of the folded state: every object id is replaced
/// by its fixture index, so two corpora that differ only in payload
/// wall-clock claims (and therefore in every hash) can be compared
/// structurally.
fn erased_projection(corpus: &[Envelope], state: &attest_family::fold::AttestState) -> String {
    let index_of = |id: &ObjectId| -> String {
        corpus
            .iter()
            .position(|e| &e.object_id() == id)
            .map(|i| i.to_string())
            .unwrap_or_else(|| "?".to_string())
    };
    let mut out = String::new();
    out.push_str("order:");
    for id in state.fold_order() {
        out.push_str(&format!(" {}", index_of(id)));
    }
    out.push_str("\nedges:");
    for e in state.edges() {
        out.push_str(&format!(
            " [{}+{} grade={} status={}]",
            index_of(&e.sides[0].half),
            index_of(&e.sides[1].half),
            e.grade.as_str(),
            match &e.status {
                EdgeStatus::Established => "established".to_string(),
                EdgeStatus::Superseded { by } => format!("superseded-by-{}", index_of(by)),
            }
        ));
    }
    out.push_str("\npending:");
    for p in state.pending_halves() {
        out.push_str(&format!(" {}", index_of(&p.object)));
    }
    out.push_str("\nvouches:");
    for v in state.vouches() {
        out.push_str(&format!(
            " [{} {:?} {}]",
            index_of(&v.object),
            v.status,
            v.scope.0
        ));
    }
    out.push_str("\nreviews:");
    for r in state.reviews() {
        out.push_str(&format!(" [{} {:?} {}]", index_of(&r.object), r.status, r.scope.0));
    }
    out.push_str("\nnotices:");
    for n in state.notices() {
        out.push_str(&format!(" [{}]", index_of(&n.review)));
    }
    out
}

#[test]
fn ordering_ignores_wallclock() {
    // Corpus A and corpus B: identical except every payload wall-clock claim
    // is shifted (+41 days) and permuted between objects.
    let dates_a = [d(2026, 7, 17), d(2026, 7, 17), d(2026, 7, 1), d(2026, 6, 2)];
    let dates_b = [d(2026, 6, 2), d(2026, 8, 27), d(2026, 7, 17), d(2026, 9, 9)];
    let (_, corpus_a) = corpus_with_dates(&dates_a);
    let (_, corpus_b) = corpus_with_dates(&dates_b);

    // The corpora genuinely differ byte-wise (the claims moved)...
    assert_ne!(
        corpus_a[0].canonical_bytes_with_sig(),
        corpus_b[0].canonical_bytes_with_sig(),
        "fixture must actually vary the wall-clock claims"
    );

    // ...but fold order and folded state are identical under hash erasure.
    let proj_a = erased_projection(&corpus_a, &log_from(&corpus_a).fold());
    let proj_b = erased_projection(&corpus_b, &log_from(&corpus_b).fold());
    assert_eq!(
        proj_a, proj_b,
        "fold order and state must not depend on wall-clock payload claims"
    );

    // And within one corpus, arrival order doesn't matter either (set-fold):
    // a reversed append order folds to the same projection.
    let mut reversed = corpus_a.clone();
    reversed.reverse();
    let proj_rev = erased_projection(&corpus_a, &log_from(&reversed).fold());
    assert_eq!(proj_a, proj_rev, "arrival order must not influence the fold");
}
