//! EXP-4 (RUN-01) — two fold open items on the §7.6.1 contradiction surface.
//!
//!  A. **Two competing RuleChange quorums** (characterization / refutation). Two Owners
//!     concurrently push conflicting RuleChanges on the *same* rule, each carrying a
//!     valid k-of-n quorum. §7.6 says an irreducible concurrent conflict with no causal
//!     order to decide it must HARD-STOP, not be silently auto-resolved. This test folds
//!     the pair in both orders and reports what the fold actually does. It is a *probe*:
//!     the fix (a RuleChange competing-quorum contradiction predicate) is a design
//!     decision (which competing-quorum shapes escalate), deliberately NOT made here —
//!     see the backlog. The test pins the CURRENT behavior so a future fix has a RED.
//!
//!  B. **Contradicted-group byte-head naming** (corroboration). When the fold DOES
//!     hard-stop a contradiction (mutual expulsion, already built), the surfaced status
//!     `contradiction:{hash}` must name the group by a deterministic, order-independent
//!     **byte-head**: the lexicographically-smaller hash of the conflicting pair
//!     (`min_hash`). This pins that the byte-head is exactly `min(H(F), H(G))` and is
//!     identical across arrival orders.

mod common;

use std::sync::Arc;

use common::{
    approval_payload, base, genesis_payload, membership_add_payload, rule_change_payload,
    rule_change_subject as rc_subject, sign,
};
use local_storage_projection::fold_derived::DerivedFold;
use local_storage_projection::tables::Db;
use local_storage_projection::types::envelope_hash;
use local_storage_projection::{
    AssertionEnvelope, AssertionType, DeviceId, GroupId, Hash, PrincipalId,
};
use social_graph_core::{Ed25519Verifier, Identity, RegistryCredentialResolver, Session};

fn remove_payload(subject: PrincipalId) -> Vec<u8> {
    subject.as_bytes().to_vec()
}

/// Fold `order` into a fresh store directly through the fold, then open a reader.
fn fold_order(
    path: &std::path::Path,
    authors: &[&Identity],
    reader: &Identity,
    order: &[&AssertionEnvelope],
) -> Session {
    {
        let db = Arc::new(Db::open(path).expect("open db"));
        let resolver = RegistryCredentialResolver::new();
        for a in authors {
            resolver.register(a.device_id(), a.principal_id());
        }
        let fold = DerivedFold::new(Arc::clone(&db), Ed25519Verifier, resolver);
        for env in order {
            let outcome = fold.ingest(env);
            eprintln!(
                "  ingest {:?} -> {:?}",
                env.assertion_type,
                outcome.map(|_| "ok").map_err(|e| e.to_string())
            );
        }
    }
    Session::open(path, reader).expect("open session")
}

/// The lexicographically-smaller of two hashes — the canonical order-independent
/// byte-head naming a conflicting pair (mirrors `fold_derived::min_hash`).
fn min_hash(a: Hash, b: Hash) -> Hash {
    if a.as_bytes() <= b.as_bytes() {
        a
    } else {
        b
    }
}

// ---------------------------------------------------------------------------
// A. Two competing RuleChange quorums — characterization / refutation
// ---------------------------------------------------------------------------

#[tokio::test]
async fn two_competing_rulechange_quorums() {
    let dir = tempfile::tempdir().expect("tempdir");
    let group = GroupId::new([0xC0; 32]);

    // Two Owners (O1, O2) so two DISTINCT quorums can each author a RuleChange
    // (RuleChange requires an Owner author). A, B are the second approver of each.
    let id_o1 = Identity::from_seed([0xA0; 32]);
    let id_o2 = Identity::from_seed([0xA1; 32]);
    let id_a = Identity::from_seed([0xA2; 32]);
    let id_b = Identity::from_seed([0xA3; 32]);
    let o1_device = DeviceId::new(id_o1.device_id().0);
    let o2_principal = PrincipalId::new(id_o2.principal_id().0);
    let a_principal = PrincipalId::new(id_a.principal_id().0);
    let b_principal = PrincipalId::new(id_b.principal_id().0);

    // Setup: genesis (O1 Owner), add O2 as Owner (role 0), add A and B as Admins,
    // raise rule_change_threshold 1 -> 2 so each competing change needs a 2-quorum.
    let genesis = sign(&id_o1, base(&id_o1, group, AssertionType::GroupGenesis, 1, vec![], genesis_payload(&o1_device)));
    let add_o2 = sign(&id_o1, base(&id_o1, group, AssertionType::MembershipAdd, 2, vec![], membership_add_payload(o2_principal, 0)));
    let add_a = sign(&id_o1, base(&id_o1, group, AssertionType::MembershipAdd, 3, vec![], membership_add_payload(a_principal, 1)));
    let add_b = sign(&id_o1, base(&id_o1, group, AssertionType::MembershipAdd, 4, vec![], membership_add_payload(b_principal, 1)));
    let raise = sign(&id_o1, base(&id_o1, group, AssertionType::RuleChange, 5, vec![], rule_change_payload(3, 2)));
    let raise_h = envelope_hash(&raise);

    // Two CONCURRENT, CONFLICTING changes to the SAME rule (add_member_threshold,
    // key 0): quorum-1 (O1 + A) sets it to 5; quorum-2 (O2 + B) sets it to 9.
    // Each references only `raise` + its own approval — neither references the other,
    // so they are causally concurrent.
    let payload5 = rule_change_payload(0, 5);
    let payload9 = rule_change_payload(0, 9);
    let appr_a = sign(&id_a, base(&id_a, group, AssertionType::Approval, 1, vec![raise_h], approval_payload(AssertionType::RuleChange, rc_subject(&payload5))));
    let appr_b = sign(&id_b, base(&id_b, group, AssertionType::Approval, 1, vec![raise_h], approval_payload(AssertionType::RuleChange, rc_subject(&payload9))));
    let change5 = sign(&id_o1, base(&id_o1, group, AssertionType::RuleChange, 6, vec![raise_h, envelope_hash(&appr_a)], payload5));
    let change9 = sign(&id_o2, base(&id_o2, group, AssertionType::RuleChange, 1, vec![raise_h, envelope_hash(&appr_b)], payload9));

    let authors = [&id_o1, &id_o2, &id_a, &id_b];
    let setup: Vec<&AssertionEnvelope> = vec![&genesis, &add_o2, &add_a, &add_b, &raise, &appr_a, &appr_b];

    // Order 1: change5 folded before change9.
    let order1: Vec<&AssertionEnvelope> = setup.iter().copied().chain([&change5, &change9]).collect();
    let sess1 = fold_order(&dir.path().join("q1.redb"), &authors, &id_o1, &order1);
    // Order 2: change9 folded before change5.
    let order2: Vec<&AssertionEnvelope> = setup.iter().copied().chain([&change9, &change5]).collect();
    let sess2 = fold_order(&dir.path().join("q2.redb"), &authors, &id_o1, &order2);

    let s1 = sess1.get_group_summary(&group).expect("summary 1");
    let s2 = sess2.get_group_summary(&group).expect("summary 2");
    eprintln!(
        "COMPETING-QUORUMS: order1 -> add_member_threshold={} fork={:?}; order2 -> add_member_threshold={} fork={:?}",
        s1.rules.add_member_threshold, s1.fork_status, s2.rules.add_member_threshold, s2.fork_status
    );

    let order_dependent = s1.rules.add_member_threshold != s2.rules.add_member_threshold;
    let hard_stopped = s1.fork_status.starts_with("contradiction");
    eprintln!("COMPETING-QUORUMS: order_dependent={order_dependent} hard_stopped={hard_stopped}");

    // FIXED (RUN-03 Phase B): the fold now carries a competing-RuleChange contradiction
    // predicate (§7.6.1, F8 — `detect_competing_rulechange`). Two concurrent admitted
    // RuleChanges on the same rule with differing values HARD-STOP, surfaced identically to
    // mutual expulsion as `contradiction:{byte-head}` where the byte-head is the
    // order-independent `min(H(F), H(G))`. Neither contested change is applied, so the rule
    // keeps its pre-conflict value — no verdict. Both fold orders yield the IDENTICAL
    // contradiction status and the IDENTICAL effective rules (order-independence restored).
    // Register: alpha/experiments/SPEC-DIVERGENCE-REGISTER.md (`competing-quorum-autoresolve`,
    // now Reconciled).
    let expected_head = min_hash(envelope_hash(&change5), envelope_hash(&change9));
    let expected_status = format!("contradiction:{expected_head}");

    assert_eq!(
        s1.fork_status, expected_status,
        "competing RuleChange quorums must hard-stop, byte-head min(H(F),H(G)) (order 1)"
    );
    assert_eq!(
        s2.fork_status, expected_status,
        "the contradiction byte-head must be identical under the opposite arrival order (order 2)"
    );
    assert_eq!(
        s1.rules.add_member_threshold, s2.rules.add_member_threshold,
        "both fold orders must leave identical effective rules (order-independent)"
    );
    // Pre-conflict value retained: neither contested change (5 or 9) applied; the rule
    // stays at the genesis default (1), the state before either RuleChange — no verdict.
    assert_eq!(
        s1.rules.add_member_threshold, 1,
        "a hard-stopped competing RuleChange must leave the rule at its pre-conflict value"
    );
    assert!(
        s1.rules.add_member_threshold != 5 && s1.rules.add_member_threshold != 9,
        "neither contested new_value may silently win"
    );
}

// ---------------------------------------------------------------------------
// A (negative cases) — the predicate must NOT over-trip
// ---------------------------------------------------------------------------

/// Two concurrent RuleChanges on the SAME rule with the SAME new_value are **concordant**,
/// not a contradiction: there is nothing to escalate because both name the identical
/// outcome. The fold applies the value and stays `Clean`, identically in both orders.
#[tokio::test]
async fn concurrent_same_value_rulechanges_are_concordant() {
    let dir = tempfile::tempdir().expect("tempdir");
    let group = GroupId::new([0xC2; 32]);

    let id_o1 = Identity::from_seed([0xD0; 32]);
    let id_o2 = Identity::from_seed([0xD1; 32]);
    let id_a = Identity::from_seed([0xD2; 32]);
    let o1_device = DeviceId::new(id_o1.device_id().0);
    let o2_principal = PrincipalId::new(id_o2.principal_id().0);
    let a_principal = PrincipalId::new(id_a.principal_id().0);

    // rule_change_threshold stays at its default of 1, so each Owner can author a
    // RuleChange alone; `add_a` is a shared antecedent that makes concurrency provable.
    let genesis = sign(&id_o1, base(&id_o1, group, AssertionType::GroupGenesis, 1, vec![], genesis_payload(&o1_device)));
    let add_o2 = sign(&id_o1, base(&id_o1, group, AssertionType::MembershipAdd, 2, vec![], membership_add_payload(o2_principal, 0)));
    let add_a = sign(&id_o1, base(&id_o1, group, AssertionType::MembershipAdd, 3, vec![], membership_add_payload(a_principal, 1)));
    let add_a_h = envelope_hash(&add_a);

    // Both set add_member_threshold (key 0) to the SAME value 5; neither references the
    // other, so they are causally concurrent, but concordant on the outcome.
    let change_a = sign(&id_o1, base(&id_o1, group, AssertionType::RuleChange, 4, vec![add_a_h], rule_change_payload(0, 5)));
    let change_b = sign(&id_o2, base(&id_o2, group, AssertionType::RuleChange, 1, vec![add_a_h], rule_change_payload(0, 5)));

    let authors = [&id_o1, &id_o2, &id_a];
    let setup: Vec<&AssertionEnvelope> = vec![&genesis, &add_o2, &add_a];

    let order1: Vec<&AssertionEnvelope> = setup.iter().copied().chain([&change_a, &change_b]).collect();
    let sess1 = fold_order(&dir.path().join("c1.redb"), &authors, &id_o1, &order1);
    let order2: Vec<&AssertionEnvelope> = setup.iter().copied().chain([&change_b, &change_a]).collect();
    let sess2 = fold_order(&dir.path().join("c2.redb"), &authors, &id_o1, &order2);

    let s1 = sess1.get_group_summary(&group).expect("summary 1");
    let s2 = sess2.get_group_summary(&group).expect("summary 2");
    eprintln!(
        "CONCORDANT: order1 -> add_member_threshold={} fork={:?}; order2 -> add_member_threshold={} fork={:?}",
        s1.rules.add_member_threshold, s1.fork_status, s2.rules.add_member_threshold, s2.fork_status
    );

    assert!(
        !s1.fork_status.starts_with("contradiction") && !s2.fork_status.starts_with("contradiction"),
        "same-rule same-value concurrent RuleChanges must NOT hard-stop (concordant)"
    );
    assert_eq!(s1.rules.add_member_threshold, 5, "the concordant value applies (order 1)");
    assert_eq!(s2.rules.add_member_threshold, 5, "the concordant value applies (order 2)");
}

/// Two concurrent RuleChanges on DIFFERENT rule_keys never conflict: they touch disjoint
/// rules and commute. Both apply, the fold stays `Clean`, identically in both orders.
#[tokio::test]
async fn concurrent_disjoint_rulekey_rulechanges_do_not_conflict() {
    let dir = tempfile::tempdir().expect("tempdir");
    let group = GroupId::new([0xC3; 32]);

    let id_o1 = Identity::from_seed([0xE0; 32]);
    let id_o2 = Identity::from_seed([0xE1; 32]);
    let id_a = Identity::from_seed([0xE2; 32]);
    let o1_device = DeviceId::new(id_o1.device_id().0);
    let o2_principal = PrincipalId::new(id_o2.principal_id().0);
    let a_principal = PrincipalId::new(id_a.principal_id().0);

    let genesis = sign(&id_o1, base(&id_o1, group, AssertionType::GroupGenesis, 1, vec![], genesis_payload(&o1_device)));
    let add_o2 = sign(&id_o1, base(&id_o1, group, AssertionType::MembershipAdd, 2, vec![], membership_add_payload(o2_principal, 0)));
    let add_a = sign(&id_o1, base(&id_o1, group, AssertionType::MembershipAdd, 3, vec![], membership_add_payload(a_principal, 1)));
    let add_a_h = envelope_hash(&add_a);

    // Different rules: add_member_threshold (key 0) → 5, remove_member_threshold (key 1) → 7.
    let change_a = sign(&id_o1, base(&id_o1, group, AssertionType::RuleChange, 4, vec![add_a_h], rule_change_payload(0, 5)));
    let change_b = sign(&id_o2, base(&id_o2, group, AssertionType::RuleChange, 1, vec![add_a_h], rule_change_payload(1, 7)));

    let authors = [&id_o1, &id_o2, &id_a];
    let setup: Vec<&AssertionEnvelope> = vec![&genesis, &add_o2, &add_a];

    let order1: Vec<&AssertionEnvelope> = setup.iter().copied().chain([&change_a, &change_b]).collect();
    let sess1 = fold_order(&dir.path().join("d1.redb"), &authors, &id_o1, &order1);
    let order2: Vec<&AssertionEnvelope> = setup.iter().copied().chain([&change_b, &change_a]).collect();
    let sess2 = fold_order(&dir.path().join("d2.redb"), &authors, &id_o1, &order2);

    let s1 = sess1.get_group_summary(&group).expect("summary 1");
    let s2 = sess2.get_group_summary(&group).expect("summary 2");
    eprintln!(
        "DISJOINT: order1 -> add={} remove={} fork={:?}; order2 -> add={} remove={} fork={:?}",
        s1.rules.add_member_threshold, s1.rules.remove_member_threshold, s1.fork_status,
        s2.rules.add_member_threshold, s2.rules.remove_member_threshold, s2.fork_status
    );

    assert!(
        !s1.fork_status.starts_with("contradiction") && !s2.fork_status.starts_with("contradiction"),
        "concurrent RuleChanges on different rule_keys must NOT hard-stop (disjoint)"
    );
    assert_eq!(s1.rules.add_member_threshold, 5, "add_member_threshold applied (order 1)");
    assert_eq!(s1.rules.remove_member_threshold, 7, "remove_member_threshold applied (order 1)");
    assert_eq!(s2.rules.add_member_threshold, 5, "add_member_threshold applied (order 2)");
    assert_eq!(s2.rules.remove_member_threshold, 7, "remove_member_threshold applied (order 2)");
}

// ---------------------------------------------------------------------------
// B. Contradicted-group byte-head naming — corroboration
// ---------------------------------------------------------------------------

#[tokio::test]
async fn contradicted_group_byte_head_is_min_hash_order_independent() {
    let dir = tempfile::tempdir().expect("tempdir");
    let group = GroupId::new([0xC1; 32]);

    let id_o = Identity::from_seed([0xB0; 32]);
    let id_a = Identity::from_seed([0xB1; 32]);
    let id_b = Identity::from_seed([0xB2; 32]);
    let a_principal = PrincipalId::new(id_a.principal_id().0);
    let b_principal = PrincipalId::new(id_b.principal_id().0);
    let o_device = DeviceId::new(id_o.device_id().0);

    let genesis = sign(&id_o, base(&id_o, group, AssertionType::GroupGenesis, 1, vec![], genesis_payload(&o_device)));
    let add_a = sign(&id_o, base(&id_o, group, AssertionType::MembershipAdd, 2, vec![], membership_add_payload(a_principal, 1)));
    let add_b = sign(&id_o, base(&id_o, group, AssertionType::MembershipAdd, 3, vec![], membership_add_payload(b_principal, 1)));
    // Mutual expulsion (the §2.5 canonical residue): A removes B, B removes A, concurrently.
    let a_removes_b = sign(&id_a, base(&id_a, group, AssertionType::MembershipRemove, 1, vec![envelope_hash(&add_b)], remove_payload(b_principal)));
    let b_removes_a = sign(&id_b, base(&id_b, group, AssertionType::MembershipRemove, 1, vec![envelope_hash(&add_a)], remove_payload(a_principal)));

    // The specified byte-head: the lexicographically-smaller of the two remove hashes.
    let expected_head = min_hash(envelope_hash(&a_removes_b), envelope_hash(&b_removes_a));
    let expected_status = format!("contradiction:{expected_head}");

    let authors = [&id_o, &id_a, &id_b];
    let setup: Vec<&AssertionEnvelope> = vec![&genesis, &add_a, &add_b];

    let order1: Vec<&AssertionEnvelope> = setup.iter().copied().chain([&a_removes_b, &b_removes_a]).collect();
    let sess1 = fold_order(&dir.path().join("h1.redb"), &authors, &id_o, &order1);
    let order2: Vec<&AssertionEnvelope> = setup.iter().copied().chain([&b_removes_a, &a_removes_b]).collect();
    let sess2 = fold_order(&dir.path().join("h2.redb"), &authors, &id_o, &order2);

    let s1 = sess1.get_group_summary(&group).expect("summary 1");
    let s2 = sess2.get_group_summary(&group).expect("summary 2");
    eprintln!(
        "BYTE-HEAD: expected={expected_status}; order1={}; order2={}",
        s1.fork_status, s2.fork_status
    );

    assert_eq!(
        s1.fork_status, expected_status,
        "the contradicted group's byte-head must be min(H(F), H(G)) exactly (order 1)"
    );
    assert_eq!(
        s2.fork_status, expected_status,
        "the byte-head must be identical under the opposite arrival order (order 2)"
    );
}
