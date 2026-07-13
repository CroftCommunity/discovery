//! Verification — k-of-n governance thresholds are enforced over distinct personae
//! (Battery 6 / V5′, Rung B).
//!
//! This began as a refutation: thresholds were decorative (a single Owner satisfied a
//! 2-of-n). The fold now enforces them (V5′). A threshold-k governance act must carry
//! approvals from k DISTINCT personae by lineage: `Approval` facts naming the act's
//! (type, subject), referenced as the act's antecedents. The antecedent guard holds the
//! act until the approvals are present; then Step 5.6 counts distinct approver principals
//! (author + approvals) and admits the act only at quorum.
//!
//! Three cases, all bootstrapped to add_member_threshold = 2:
//!   1. sole Owner, no approvals → rejected (1 < 2), X absent.
//!   2. Owner + a second admin's approval → admitted (2 distinct), X present.
//!   3. Owner + Owner's OWN approval → rejected (still 1 distinct persona) — a persona
//!      cannot reach quorum by approving its own act. (The pure by-device lineage guard
//!      is unit-tested in governance::thresholds_count_personae_by_lineage_never_clients.)

mod common;

use common::{approval_payload, base, genesis_payload, ingest_seq, membership_add_payload, rule_change_payload, sign};
use local_storage_projection::types::envelope_hash;
use local_storage_projection::{AssertionType, DeviceId, GroupId, Hash, PrincipalId};
use social_graph_core::Identity;

#[tokio::test]
async fn single_signer_fails_two_of_n() {
    let dir = tempfile::tempdir().expect("tempdir");
    let group = GroupId::new([0x31; 32]);
    let id_o = Identity::from_seed([0xF0; 32]);
    let o_device = DeviceId::new(id_o.device_id().0);
    let x = PrincipalId::new([0xAA; 32]);

    let genesis = sign(&id_o, base(&id_o, group, AssertionType::GroupGenesis, 1, vec![], genesis_payload(&o_device)));
    // Raise add_member_threshold 1 → 2 (rule_change_threshold is 1, so O alone can).
    let raise = sign(&id_o, base(&id_o, group, AssertionType::RuleChange, 2, vec![], rule_change_payload(0, 2)));
    // Sole Owner adds X with no approvals.
    let add_x = sign(&id_o, base(&id_o, group, AssertionType::MembershipAdd, 3, vec![], membership_add_payload(x, 2)));

    let sess = ingest_seq(&dir.path().join("t.redb"), &[&id_o], &id_o, &[&genesis, &raise, &add_x]);
    let sum = sess.get_group_summary(&group).expect("summary");
    assert_eq!(sum.rules.add_member_threshold, 2, "threshold raised to 2");
    assert!(
        !sum.members.iter().any(|m| m.principal == x),
        "X must NOT be admitted — one signer cannot meet a 2-of-n add (V5′ enforced)"
    );
    eprintln!("THRESHOLD single-signer: X absent — 2-of-n enforced.");
}

#[tokio::test]
async fn two_distinct_personae_meet_two_of_n() {
    let dir = tempfile::tempdir().expect("tempdir");
    let group = GroupId::new([0x32; 32]);
    let id_o = Identity::from_seed([0xF1; 32]);
    let id_a2 = Identity::from_seed([0xF2; 32]);
    let a2 = PrincipalId::new(id_a2.principal_id().0);
    let o_device = DeviceId::new(id_o.device_id().0);
    let x = PrincipalId::new([0xAB; 32]);

    let genesis = sign(&id_o, base(&id_o, group, AssertionType::GroupGenesis, 1, vec![], genesis_payload(&o_device)));
    let add_a2 = sign(&id_o, base(&id_o, group, AssertionType::MembershipAdd, 2, vec![], membership_add_payload(a2, 1))); // Admin, thresh still 1
    let raise = sign(&id_o, base(&id_o, group, AssertionType::RuleChange, 3, vec![], rule_change_payload(0, 2)));
    // A2 approves adding X; O submits the add referencing that approval.
    let approval = sign(&id_a2, base(&id_a2, group, AssertionType::Approval, 1, vec![], approval_payload(AssertionType::MembershipAdd, x)));
    let ant: Vec<Hash> = vec![envelope_hash(&approval)];
    let add_x = sign(&id_o, base(&id_o, group, AssertionType::MembershipAdd, 4, ant, membership_add_payload(x, 2)));

    let sess = ingest_seq(&dir.path().join("t.redb"), &[&id_o, &id_a2], &id_o, &[&genesis, &add_a2, &raise, &approval, &add_x]);
    let sum = sess.get_group_summary(&group).expect("summary");
    assert!(
        sum.members.iter().any(|m| m.principal == x),
        "X admitted — author O + approver A2 are two distinct personae, meeting the 2-of-n"
    );
    eprintln!("THRESHOLD two-personae: X present — quorum of two distinct personae met.");
}

#[tokio::test]
async fn self_approval_does_not_reach_quorum() {
    let dir = tempfile::tempdir().expect("tempdir");
    let group = GroupId::new([0x33; 32]);
    let id_o = Identity::from_seed([0xF3; 32]);
    let o_device = DeviceId::new(id_o.device_id().0);
    let x = PrincipalId::new([0xAC; 32]);

    let genesis = sign(&id_o, base(&id_o, group, AssertionType::GroupGenesis, 1, vec![], genesis_payload(&o_device)));
    let raise = sign(&id_o, base(&id_o, group, AssertionType::RuleChange, 2, vec![], rule_change_payload(0, 2)));
    // O approves its OWN add — same persona, must not count as a second.
    let self_approval = sign(&id_o, base(&id_o, group, AssertionType::Approval, 3, vec![], approval_payload(AssertionType::MembershipAdd, x)));
    let ant: Vec<Hash> = vec![envelope_hash(&self_approval)];
    let add_x = sign(&id_o, base(&id_o, group, AssertionType::MembershipAdd, 4, ant, membership_add_payload(x, 2)));

    let sess = ingest_seq(&dir.path().join("t.redb"), &[&id_o], &id_o, &[&genesis, &raise, &self_approval, &add_x]);
    let sum = sess.get_group_summary(&group).expect("summary");
    assert!(
        !sum.members.iter().any(|m| m.principal == x),
        "X must NOT be admitted — a persona approving its own act is still one persona"
    );
    eprintln!("THRESHOLD self-approval: X absent — one persona cannot self-quorum a 2-of-n.");
}
