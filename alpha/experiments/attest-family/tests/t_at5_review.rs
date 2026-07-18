//! Part 5 — EXP-AT5: unilateral-with-notice mode (reviews). T-AT5.*.
//!
//! Reviews must not require subject countersignature — a business would
//! countersign only praise; that failure is the point. Integrity comes from
//! provenance structure: signed authorship, scope, antecedent grades, subject
//! notice, and signed reply as a peer object.

mod common;

use attest_family::fixtures::*;
use attest_family::fold::{Marker, ReviewStatus};
use attest_family::query::FreshnessDial;
use attest_family::types::*;
use common::*;

fn d(y: u16, m: u8, day: u8) -> DateClaim {
    DateClaim::new(y, m, day)
}

/// BIZ1 (controller P3), one review by P1a. V4 churn (RUN-ATTEST-04, named):
/// under the graded resolvability default the reviewer P1a opts OPEN (a
/// deliberate policy act) so third-party viewers still traverse the review —
/// the fixture posture for a public reviewer.
fn fixture() -> (World, Vec<Envelope>, ObjectId) {
    let w = World::new();
    let decl = w.p3.emit(vec![], thing_decl(w.biz1, ThingKind::Business, w.p3.id));
    let rv = w.p1a.emit(
        vec![],
        review(SubjectRef::Thing(w.biz1), "plumbing", "showed up late, fixed it well", d(2026, 6, 1), None),
    );
    let open = w.p1a.emit(vec![], policy(w.p1a.id, PolicyRule::AllowAll, None));
    let rv_id = rv.object_id();
    (w, vec![decl, rv, open], rv_id)
}

// ---------------------------------------------------------------------------
// T-AT5.1 — a review stands with only the author's signature
// ---------------------------------------------------------------------------

#[test]
fn review_stands_without_countersign() {
    let (_, envs, rv_id) = fixture();
    let state = log_from(&envs).fold();
    let view = state.review(&rv_id).expect("review folded");
    assert_eq!(view.status, ReviewStatus::Standing, "unilateral_notice needs no countersign");
    assert_eq!(view.replies, Vec::<ObjectId>::new(), "no reply exists yet — and none is required");
}

// ---------------------------------------------------------------------------
// T-AT5.2 — folding a review deterministically emits a notice fact
// ---------------------------------------------------------------------------

#[test]
fn subject_notice_fact_emitted() {
    let (w, envs, rv_id) = fixture();
    let state = log_from(&envs).fold();

    let notices = state.notices();
    assert_eq!(notices.len(), 1, "exactly one notice per standing review");
    assert_eq!(notices[0].review, rv_id);
    assert_eq!(notices[0].subject, SubjectRef::Thing(w.biz1));
    assert_eq!(
        notices[0].addressed_to,
        Some(w.p3.id),
        "the notice is addressed to the subject (the thing's declared controller)"
    );

    // Deterministic: every arrival permutation yields the identical notice set.
    for perm in permutations(&envs) {
        let s = log_from(&perm).fold();
        assert_eq!(s.notices(), notices, "notice emission must be deterministic");
    }
}

// ---------------------------------------------------------------------------
// T-AT5.3 — the signed reply is a peer object; review bytes unchanged
// ---------------------------------------------------------------------------

#[test]
fn signed_reply_is_peer_object() {
    let (w, envs, rv_id) = fixture();
    let mut log = log_from(&envs);
    let rv_bytes = log.object_bytes(&rv_id).unwrap().to_vec();

    // The subject's controller replies; a non-controller's "reply" does not attach.
    let rp = w.p3.emit(vec![rv_id], reply(rv_id, "we were stuck in traffic — sorry", d(2026, 6, 2)));
    let rogue = w.p2.emit(vec![rv_id], reply(rv_id, "unrelated party", d(2026, 6, 3)));
    log.append(rp.clone()).unwrap();
    log.append(rogue.clone()).unwrap();

    let state = log.fold();
    let view = state.review(&rv_id).unwrap();
    assert_eq!(view.replies, vec![rp.object_id()], "the subject's signed reply attaches; a rogue reply does not");
    // The review's bytes are unchanged by the reply (peer object, not mutation).
    assert_eq!(log.object_bytes(&rv_id).unwrap(), &rv_bytes[..]);

    // The corroboration structure returns both (review entry carries the reply).
    let resp = state.corroboration(
        &w.p2.id,
        &SubjectRef::Thing(w.biz1),
        &Scope::new("plumbing"),
        &FreshnessDial { stale_after_days: 3650 },
        d(2026, 7, 18),
    );
    assert_eq!(resp.entries.len(), 1);
    assert_eq!(resp.entries[0].attestation, rv_id);
    assert_eq!(resp.entries[0].replies, vec![rp.object_id()]);
}

// ---------------------------------------------------------------------------
// T-AT5.4 — no suppression path exists (negative API-surface test)
// ---------------------------------------------------------------------------

#[test]
fn no_suppression_path_exists() {
    // (a) Enumerate the crate's public operations (fold + query surface) and
    // pin them to the exact allowlist. Anything new must be reviewed against
    // this invariant before it can land.
    let allowlist: std::collections::BTreeSet<&str> = [
        // log
        "new", "append", "object_bytes", "envelope", "len", "is_empty", "fold",
        // state accessors
        "fold_order", "edges", "edge_by_core", "pending_halves", "vouches", "vouch",
        "reviews", "review", "predicates", "notices", "policy_head", "policy_lineage",
        "resolvable", "edge_list",
        // RUN-ATTEST-02 (reviewed): read-only credential accessors — return
        // fold views only; cannot remove, hide, or demote anything.
        "credentials", "credential",
        // RUN-ATTEST-03 (reviewed): the V1 antecedent-register surface.
        // `fold_with_register` mirrors the R7-governed qualifying-kind
        // register into a fold; it can change STANDING (qualification), never
        // remove, hide, or demote a stored object — lineage and views remain,
        // and the register moves only by content-bound quorum, not by any
        // subject's unilateral act. `full`/`from_mask`/`allows` are pure
        // constructors/readers on the mirror type.
        "fold_with_register", "full", "from_mask", "allows",
        // views
        "participants", "side", "as_str",
        // query
        "corroboration", "mutual_connection_count", "edge_list_ipld", "to_ipld",
        "to_canonical_bytes", "predicate_presentation",
    ]
    .into_iter()
    .collect();

    let mut seen = Vec::new();
    for file in ["src/fold.rs", "src/query.rs"] {
        let src = crate_source(file);
        for (line_no, line) in code_lines(&src) {
            let t = line.trim();
            if let Some(rest) = t.strip_prefix("pub fn ") {
                let name: String = rest
                    .chars()
                    .take_while(|c| c.is_alphanumeric() || *c == '_')
                    .collect();
                assert!(
                    allowlist.contains(name.as_str()),
                    "{file}:{line_no}: unreviewed public operation `{name}` — check it cannot suppress"
                );
                seen.push(name);
            }
        }
        // No operation is even NAMED like suppression.
        for (line_no, line) in code_lines(&src) {
            let l = line.to_lowercase();
            for bad in ["fn delete", "fn remove", "fn hide", "fn suppress", "fn demote", "fn take_down", "fn redact"] {
                assert!(!l.contains(bad), "{file}:{line_no}: suppression-shaped operation: {line}");
            }
        }
    }
    assert!(seen.contains(&"corroboration".to_string()), "scan must actually see the query surface");

    // (b) Behaviorally: everything the subject can do — reply, and set their
    // own persona's resolvability policy — leaves the review in a third
    // viewer's corroboration structure.
    let (w, envs, rv_id) = fixture();
    let mut log = log_from(&envs);
    let rp = w.p3.emit(vec![rv_id], reply(rv_id, "our side of it", d(2026, 6, 2)));
    log.append(rp).unwrap();
    // The subject's controller locks their OWN persona down completely.
    let pol = w.p3.emit(vec![], policy(w.p3.id, PolicyRule::AllowOnly(vec![]), None));
    log.append(pol).unwrap();

    let state = log.fold();
    let resp = state.corroboration(
        &w.p2.id, // a third viewer
        &SubjectRef::Thing(w.biz1),
        &Scope::new("plumbing"),
        &FreshnessDial { stale_after_days: 3650 },
        d(2026, 7, 18),
    );
    assert_eq!(
        resp.entries.iter().filter(|e| e.attestation == rv_id).count(),
        1,
        "no subject power can remove, hide, or demote the review from a third viewer's structure"
    );
}

// ---------------------------------------------------------------------------
// T-AT5.5 — freshness is presentation; never dropped, never down-ranked
// ---------------------------------------------------------------------------

#[test]
fn freshness_is_presentation() {
    let w = World::new();
    let decl = w.p3.emit(vec![], thing_decl(w.biz1, ThingKind::Business, w.p3.id));
    let old = w.p1a.emit(
        vec![],
        review(SubjectRef::Thing(w.biz1), "plumbing", "years ago: fine", d(2023, 1, 10), None),
    );
    let fresh = w.p2.emit(
        vec![],
        review(SubjectRef::Thing(w.biz1), "plumbing", "last week: fine", d(2026, 7, 10), None),
    );
    // V4 churn (RUN-ATTEST-04, named): both reviewers opt OPEN so the
    // stranger viewer P3 traverses them under the graded default.
    let open_a = w.p1a.emit(vec![], policy(w.p1a.id, PolicyRule::AllowAll, None));
    let open_b = w.p2.emit(vec![], policy(w.p2.id, PolicyRule::AllowAll, None));
    let state = log_from(&[decl, old.clone(), fresh.clone(), open_a, open_b]).fold();

    let dial = FreshnessDial { stale_after_days: 90 };
    let resp = state.corroboration(
        &w.p3.id,
        &SubjectRef::Thing(w.biz1),
        &Scope::new("plumbing"),
        &dial,
        d(2026, 7, 18),
    );

    // Both entries are PRESENT — the dial never drops.
    assert_eq!(resp.entries.len(), 2, "an old review is never dropped (no verdict by timeout)");
    let old_entry = resp.entries.iter().find(|e| e.attestation == old.object_id()).unwrap();
    let fresh_entry = resp.entries.iter().find(|e| e.attestation == fresh.object_id()).unwrap();
    assert_eq!(old_entry.markers, vec![Marker::Stale], "old gains staleness presentation metadata");
    assert!(fresh_entry.markers.is_empty());

    // And no down-ranking: the order is canonical-hash order regardless of age.
    let ids: Vec<[u8; 32]> = resp.entries.iter().map(|e| e.attestation.0).collect();
    let mut sorted = ids.clone();
    sorted.sort();
    assert_eq!(ids, sorted, "entry order is canonical-hash order, not freshness order");

    // A wider dial marks nothing; entries and order are unchanged.
    let resp_wide = state.corroboration(
        &w.p3.id,
        &SubjectRef::Thing(w.biz1),
        &Scope::new("plumbing"),
        &FreshnessDial { stale_after_days: 36500 },
        d(2026, 7, 18),
    );
    assert!(resp_wide.entries.iter().all(|e| e.markers.is_empty()));
    let ids_wide: Vec<[u8; 32]> = resp_wide.entries.iter().map(|e| e.attestation.0).collect();
    assert_eq!(ids, ids_wide, "the dial changes presentation only, never membership or order");
}
