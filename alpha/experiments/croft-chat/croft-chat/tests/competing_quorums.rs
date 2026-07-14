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

    // CHARACTERIZATION of current behavior (see assertion values set from the observed
    // run). This documents whether the fold hard-stops (spec-faithful) or auto-resolves
    // order-dependently (a refutation — the §7.6.1 gap for competing RuleChange quorums).
    let order_dependent = s1.rules.add_member_threshold != s2.rules.add_member_threshold;
    let hard_stopped = s1.fork_status.starts_with("contradiction");
    eprintln!("COMPETING-QUORUMS: order_dependent={order_dependent} hard_stopped={hard_stopped}");

    // SPEC-DELTA[competing-quorum-autoresolve | weakened-assertion]: this test asserts the
    // CURRENT order-dependent auto-resolution of two competing RuleChange quorums, which is
    // strictly WEAKER than §7.6's required hard-stop. The fold has no competing-RuleChange
    // contradiction predicate (only membership/role races escalate). The fix is a design
    // decision (which competing-quorum shapes escalate) — see the backlog A1 finding row.
    // Register: alpha/experiments/SPEC-DIVERGENCE-REGISTER.md
    //
    // If a future fix adds the predicate, `hard_stopped` flips true and this RED assertion
    // fires, forcing the test to be updated to assert the fixed hard-stop behavior.
    assert!(
        order_dependent && !hard_stopped,
        "REFUTATION PIN: two competing RuleChange quorums currently auto-resolve \
         order-dependently with NO contradiction flag (order1={}, order2={}, \
         fork1={}, fork2={}). If this fails, the §7.6.1 competing-quorum gap has been \
         addressed — update this test to assert the hard-stop.",
        s1.rules.add_member_threshold, s2.rules.add_member_threshold, s1.fork_status, s2.fork_status
    );
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
