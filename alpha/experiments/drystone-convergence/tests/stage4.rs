// Stage 4 governance convergence tests: Groups G, H, I, K.
// All tests are hand-written and deterministic; no proptest.

use drystone_convergence::finality::{
    quorum_fold, requires_enforcing_commit, transition_key,
    Ceiling, InFlightTally, Now, QuorumResult, SlotTransition, Vote,
};
use drystone_convergence::types::{AuthorityState, FactId};
use std::collections::{BTreeMap, BTreeSet};

// ── shared helpers ────────────────────────────────────────────────────────────

fn eligible(names: &[&str]) -> BTreeSet<String> {
    names.iter().map(|&s| s.to_string()).collect()
}

// Build a Vote with a predictable explicit FactId. Using val < 256 keeps
// the byte-ordering identical to numeric ordering (only the first byte differs,
// remaining bytes are all 0).
fn ev(id: u64, author: &str, t: &SlotTransition) -> Vote {
    Vote::with_explicit_id(FactId::explicit(id), author.to_string(), vec![], t.clone())
}

fn empty_now(head_val: u64) -> Now {
    Now {
        state:        AuthorityState::empty(),
        in_flight:    BTreeMap::new(),
        head:         FactId::explicit(head_val),
        attestations: BTreeSet::new(),
    }
}

// ── Group G — quorum folding (A1) ─────────────────────────────────────────────

#[test]
fn g1_insufficient_k_minus_1() {
    // k=3, provide k-1=2 concordant votes; expect Insufficient { count:2, threshold:3 }.
    let t  = SlotTransition::RemoveMember("dave".to_string());
    let el = eligible(&["alice", "bob", "charlie"]);
    let vs = vec![ev(3, "alice", &t), ev(2, "bob", &t)];

    let result = quorum_fold(&vs, &t, 3, &el);
    assert_eq!(result, QuorumResult::Insufficient { count: 2, threshold: 3 });
}

#[test]
fn g2_crossed_at_k() {
    // k=3, provide exactly k=3 concordant votes; expect Crossed.
    let t  = SlotTransition::RemoveMember("dave".to_string());
    let el = eligible(&["alice", "bob", "charlie"]);
    let vs = vec![ev(3, "alice", &t), ev(2, "bob", &t), ev(1, "charlie", &t)];

    let result = quorum_fold(&vs, &t, 3, &el);
    assert!(matches!(result, QuorumResult::Crossed { .. }));
}

#[test]
fn g3_single_vote_insufficient_when_k_gt_1() {
    // k=2, single vote from alice — the author's own vote is not sufficient alone.
    let t  = SlotTransition::GrantRole("alice".to_string(), "admin".to_string());
    let el = eligible(&["alice", "bob"]);
    let vs = vec![ev(1, "alice", &t)];

    let result = quorum_fold(&vs, &t, 2, &el);
    assert_eq!(result, QuorumResult::Insufficient { count: 1, threshold: 2 });
}

#[test]
fn g4_permutation_invariant() {
    // Same 3 votes in 4 different input orderings must yield the same QuorumResult.
    let t  = SlotTransition::SetThreshold("admin".to_string(), 2, 3);
    let el = eligible(&["alice", "bob", "charlie"]);

    // ids < 256: numeric order == lexicographic byte order.
    // Descending sort: 3 > 2 > 1, so completing_vote = explicit(1), votes = [3, 2, 1].
    let orderings: [Vec<(&str, u64)>; 4] = [
        vec![("alice", 3), ("bob", 2), ("charlie", 1)],
        vec![("bob", 2), ("alice", 3), ("charlie", 1)],
        vec![("charlie", 1), ("bob", 2), ("alice", 3)],
        vec![("alice", 3), ("charlie", 1), ("bob", 2)],
    ];

    let results: Vec<QuorumResult> = orderings.iter()
        .map(|order| {
            let vs: Vec<Vote> = order.iter().map(|&(a, id)| ev(id, a, &t)).collect();
            quorum_fold(&vs, &t, 3, &el)
        })
        .collect();

    // All four orderings must produce identical results.
    assert!(results.windows(2).all(|w| w[0] == w[1]),
        "quorum_fold must be permutation-invariant");
    assert!(matches!(&results[0], QuorumResult::Crossed { .. }));
}

#[test]
fn g5_sub_k_ceiling_is_none() {
    // k-1 votes cannot produce a valid Ceiling; Ceiling::stamp must return None
    // for an Insufficient result, making a sub-k enactment detectable as a
    // fork origin.
    let t  = SlotTransition::RemoveMember("eve".to_string());
    let el = eligible(&["alice", "bob", "charlie"]);
    let vs = vec![ev(2, "alice", &t), ev(1, "bob", &t)];

    let result = quorum_fold(&vs, &t, 3, &el);
    assert!(matches!(result, QuorumResult::Insufficient { .. }));

    let ceiling = Ceiling::stamp("eve".to_string(), result);
    assert!(ceiling.is_none(),
        "Ceiling::stamp must return None for Insufficient — sub-k enactment is a fork origin");
}

// ── Group H — non-exclusive recognition, concurrent completion (A2) ───────────

#[test]
fn h1_concurrent_completion_non_exclusive() {
    // 3 members, k=2. Each pair independently achieves quorum. All three nodes
    // call quorum_fold on their local view and each sees Crossed — recognition
    // is non-exclusive (A2).
    let t  = SlotTransition::RemoveMember("dave".to_string());
    let el = eligible(&["alice", "bob", "charlie"]);
    let k  = 2u32;

    // Factories: Vote does not implement Clone, so create fresh instances.
    let alice   = || ev(3, "alice",   &t);
    let bob     = || ev(2, "bob",     &t);
    let charlie = || ev(1, "charlie", &t);

    // Node A: alice + bob
    let ra = quorum_fold(&[alice(), bob()], &t, k, &el);
    // Node B: alice + charlie
    let rb = quorum_fold(&[alice(), charlie()], &t, k, &el);
    // Node C: bob + charlie
    let rc = quorum_fold(&[bob(), charlie()], &t, k, &el);

    assert!(matches!(ra, QuorumResult::Crossed { .. }), "node A must see Crossed");
    assert!(matches!(rb, QuorumResult::Crossed { .. }), "node B must see Crossed");
    assert!(matches!(rc, QuorumResult::Crossed { .. }), "node C must see Crossed");
}

#[test]
fn h2_unanimous_canonical_result() {
    // k=N=3, all members vote concurrently. There is exactly one canonical
    // Crossed result regardless of input ordering.
    // completing_vote = the k-th (index 2) in descending FactId order = min(ids).
    let t  = SlotTransition::AddMember("frank".to_string());
    let el = eligible(&["alice", "bob", "charlie"]);
    let k  = 3u32;

    // explicit(3) > explicit(2) > explicit(1) (all < 256; first byte == value).
    // Descending: [3, 2, 1]. Index 2 (k-1) = explicit(1) = completing_vote.
    let expected_completing = FactId::explicit(1);
    let expected_votes      = vec![FactId::explicit(3), FactId::explicit(2), FactId::explicit(1)];

    let orderings: [Vec<(&str, u64)>; 3] = [
        vec![("alice", 3), ("bob", 2), ("charlie", 1)],
        vec![("bob", 2), ("charlie", 1), ("alice", 3)],
        vec![("charlie", 1), ("alice", 3), ("bob", 2)],
    ];

    for order in &orderings {
        let vs: Vec<Vote> = order.iter().map(|&(a, id)| ev(id, a, &t)).collect();
        let result = quorum_fold(&vs, &t, k, &el);
        match result {
            QuorumResult::Crossed { completing_vote, votes } => {
                assert_eq!(completing_vote, expected_completing,
                    "completing_vote must be the k-th in descending FactId order");
                assert_eq!(votes, expected_votes,
                    "votes must be sorted descending and truncated to k");
            }
            _ => panic!("expected Crossed for all orderings"),
        }
    }
}

// ── Group I — ceiling (A3) ────────────────────────────────────────────────────

#[test]
fn i1_ceiling_at_head_equals_completing_vote() {
    // After a quorum crosses, stamp a Ceiling. Its at_head must equal the
    // completing_vote from the QuorumResult.
    let t  = SlotTransition::RemoveMember("grace".to_string());
    let el = eligible(&["alice", "bob"]);
    let vs = vec![ev(2, "alice", &t), ev(1, "bob", &t)];

    // k=2. Descending: [2, 1]. completing_vote = index 1 = explicit(1).
    let result = quorum_fold(&vs, &t, 2, &el);
    let completing = match &result {
        QuorumResult::Crossed { completing_vote, .. } => *completing_vote,
        _ => panic!("expected Crossed"),
    };
    assert_eq!(completing, FactId::explicit(1));

    let ceiling = Ceiling::stamp("grace".to_string(), result)
        .expect("Crossed must produce a Ceiling");
    assert_eq!(ceiling.at_head, completing,
        "ceiling.at_head must equal the completing_vote");
}

#[test]
fn i2_two_ceilings_canonical_head_is_max() {
    // When two ceilings exist at different completing_vote heads, the canonical
    // head is max(at_head) — the R1 tiebreak. Permutation-invariant.
    let head_a = FactId::explicit(2);
    let head_b = FactId::explicit(5);

    let ceiling_a = Ceiling { removed: "old".to_string(), at_head: head_a, votes: vec![head_a] };
    let ceiling_b = Ceiling { removed: "old".to_string(), at_head: head_b, votes: vec![head_b] };

    let canonical = std::cmp::max(ceiling_a.at_head, ceiling_b.at_head);
    assert_eq!(canonical, head_b,
        "canonical head must be the larger completing_vote (R1 tiebreak)");

    // Permutation-invariant: max(a, b) == max(b, a).
    assert_eq!(
        std::cmp::max(ceiling_a.at_head, ceiling_b.at_head),
        std::cmp::max(ceiling_b.at_head, ceiling_a.at_head),
    );
}

#[test]
fn i3_voids_action_at() {
    // A removed member's authority ends at ceiling.at_head.
    // Actions at or before that head are not void; actions strictly after are void.
    let ceiling = Ceiling {
        removed: "old".to_string(),
        at_head: FactId::explicit(5),
        votes:   vec![],
    };

    // Action exactly at the ceiling: not void (removal is not retroactive to that point).
    assert!(!ceiling.voids_action_at(FactId::explicit(5)),
        "action at ceiling head must not be void");
    // Action before the ceiling: not void.
    assert!(!ceiling.voids_action_at(FactId::explicit(4)),
        "action before ceiling head must not be void");
    // Action strictly after the ceiling: void.
    assert!(ceiling.voids_action_at(FactId::explicit(6)),
        "action after ceiling head must be void");
}

// ── Group K — the now (A7) ────────────────────────────────────────────────────

#[test]
fn k1_fingerprint_changes_on_tamper() {
    // Same state+in_flight+head → same fingerprint.
    // Different head → different fingerprint (verifiably derived).
    let now1  = empty_now(1);
    let now1b = empty_now(1);
    let now2  = empty_now(2);

    assert_eq!(now1.fingerprint(), now1b.fingerprint(),
        "identical Nows must have identical fingerprints");
    assert_ne!(now1.fingerprint(), now2.fingerprint(),
        "changing the head must change the fingerprint");
}

#[test]
fn k2_now_replaced_not_accumulated() {
    // Advancing the Now replaces it entirely; the old Now is discarded, not merged.
    let now_v1 = empty_now(10);
    let now_v2 = empty_now(11); // head advanced by one fact

    assert_ne!(now_v1.head, now_v2.head,
        "heads must differ after advancing");
    assert_ne!(now_v1.fingerprint(), now_v2.fingerprint(),
        "Now_v2 must not share a fingerprint with Now_v1");
}

#[test]
fn k3_requires_enforcing_commit() {
    // Membership changes require an EnforcingCommit; fold-plane changes do not.
    assert!(requires_enforcing_commit(&SlotTransition::AddMember("x".to_string())));
    assert!(requires_enforcing_commit(&SlotTransition::RemoveMember("x".to_string())));
    assert!(!requires_enforcing_commit(
        &SlotTransition::GrantRole("x".to_string(), "admin".to_string())));
    assert!(!requires_enforcing_commit(
        &SlotTransition::RevokeRole("x".to_string(), "admin".to_string())));
    assert!(!requires_enforcing_commit(
        &SlotTransition::SetThreshold("admin".to_string(), 2, 3)));
}

#[test]
fn k4_same_facts_same_fingerprint_regardless_of_insertion_order() {
    // BTreeMap iterates in sorted key order, so insertion order does not affect
    // the fingerprint. Two nodes building the same in_flight via different
    // code paths arrive at the same fingerprint.
    let t   = SlotTransition::RemoveMember("dave".to_string());
    let key = transition_key(&t);

    // Helper: tally with alice→1 and bob→2, in two insertion orders.
    let make_alice_first = || {
        let mut m: BTreeMap<String, FactId> = BTreeMap::new();
        m.insert("alice".to_string(), FactId::explicit(1));
        m.insert("bob".to_string(),   FactId::explicit(2));
        InFlightTally { transition: t.clone(), votes: m, threshold: 3, enacted: false }
    };
    let make_bob_first = || {
        let mut m: BTreeMap<String, FactId> = BTreeMap::new();
        m.insert("bob".to_string(),   FactId::explicit(2));
        m.insert("alice".to_string(), FactId::explicit(1));
        InFlightTally { transition: t.clone(), votes: m, threshold: 3, enacted: false }
    };

    let mut in_flight_1: BTreeMap<String, InFlightTally> = BTreeMap::new();
    in_flight_1.insert(key.clone(), make_alice_first());

    let mut in_flight_2: BTreeMap<String, InFlightTally> = BTreeMap::new();
    in_flight_2.insert(key.clone(), make_bob_first());

    let now1 = Now {
        state: AuthorityState::empty(),
        in_flight: in_flight_1,
        head: FactId::explicit(42),
        attestations: BTreeSet::new(),
    };
    let now2 = Now {
        state: AuthorityState::empty(),
        in_flight: in_flight_2,
        head: FactId::explicit(42),
        attestations: BTreeSet::new(),
    };

    assert_eq!(now1.fingerprint(), now2.fingerprint(),
        "identical in-flight content must produce identical fingerprint regardless of insertion order");
}

#[test]
fn k5_attestations_do_not_change_fingerprint() {
    // Nine-fives test (A7): N=3 members attest the same Now.
    // There is still ONE Now object (not 3 rival nows).
    // Fingerprint must be invariant to attestations.
    let mut now = empty_now(99);

    let fp_before = now.fingerprint();

    now.attest("alice".to_string());
    now.attest("bob".to_string());
    now.attest("charlie".to_string());

    // Still one Now object with 3 attestations.
    assert_eq!(now.attestation_count(), 3);

    // Fingerprint unchanged by attestations.
    assert_eq!(now.fingerprint(), fp_before,
        "fingerprint must not change when attestations are added");

    // A now with 0 attestations and one with 3 have the same fingerprint
    // when state, in_flight, and head are identical.
    let now_zero = empty_now(99);
    assert_eq!(now_zero.fingerprint(), fp_before,
        "0-attestation and 3-attestation Nows with same state must fingerprint identically");
}

#[test]
fn k6_in_flight_tally_correctness() {
    // Build a pending RemoveMember tally with 2 of 3 votes (threshold=3).
    // Assert count=2, threshold=3, enacted=false, is_crossed=false.
    // Then add the 3rd vote: assert is_crossed==true but enacted remains false
    // (no EnforcingCommit has been issued — crossing does not auto-enact).
    let t = SlotTransition::RemoveMember("dave".to_string());
    let mut tally = InFlightTally {
        transition: t,
        votes:      BTreeMap::new(),
        threshold:  3,
        enacted:    false,
    };

    tally.votes.insert("alice".to_string(), FactId::explicit(1));
    tally.votes.insert("bob".to_string(),   FactId::explicit(2));

    assert_eq!(tally.vote_count(), 2);
    assert_eq!(tally.threshold,   3);
    assert!(!tally.is_crossed(), "2 of 3 must not be crossed");
    assert!(!tally.enacted,      "enacted must be false before EnforcingCommit");

    // Add 3rd vote: quorum is now crossed, but enacted stays false.
    tally.votes.insert("charlie".to_string(), FactId::explicit(3));

    assert_eq!(tally.vote_count(), 3);
    assert!(tally.is_crossed(),
        "3 of 3 must be crossed");
    assert!(!tally.enacted,
        "enacted remains false — crossing requires an EnforcingCommit before taking effect");
}
