//! Verification — k-of-n thresholds are enforced on **RuleChange** too, over distinct
//! personae (Battery 6 / V5′, Rung B — the RuleChange facet).
//!
//! Earns/bounds: Part 2 §7.2 R7 — the k-of-n threshold is enforced on RuleChange via the content-bound approval subject.
//!
//! This reconciles the `rulechange-quorum` spec-delta: RuleChange threshold enforcement
//! was previously deferred (a RuleChange has no principal subject, so Step 5.6 skipped it
//! and the fold fell back to an Owner-role proxy). The fold now gives a RuleChange a
//! *content-hash* subject (`rule_change_approval_subject(payload)`), so approvers name
//! `(RuleChange, H(payload))` exactly as they name `(MembershipAdd, principal)`. Step 5.6
//! then counts distinct approver personae by lineage — the same path as membership.
//!
//! Three cases, all with rule_change_threshold raised to 2:
//!   1. sole Owner, no approvals → the amendment is rejected; the target rule is unchanged.
//!   2. Owner + a second admin's approval → admitted (2 distinct personae); rule changes.
//!   3. Owner + Owner's OWN approval → rejected (still 1 distinct persona).

mod common;

use common::{approval_payload, base, genesis_payload, ingest_seq, membership_add_payload, rule_change_payload, rule_change_subject as rc_subject, sign};
use local_storage_projection::types::envelope_hash;
use local_storage_projection::{AssertionType, DeviceId, GroupId, Hash, PrincipalId};
use social_graph_core::Identity;

#[tokio::test]
async fn single_signer_fails_rulechange_two_of_n() {
    let dir = tempfile::tempdir().expect("tempdir");
    let group = GroupId::new([0x41; 32]);
    let id_o = Identity::from_seed([0xE0; 32]);
    let o_device = DeviceId::new(id_o.device_id().0);

    let genesis = sign(&id_o, base(&id_o, group, AssertionType::GroupGenesis, 1, vec![], genesis_payload(&o_device)));
    // Raise rule_change_threshold 1 → 2 (allowed: threshold was 1, so O alone can).
    let raise = sign(&id_o, base(&id_o, group, AssertionType::RuleChange, 2, vec![], rule_change_payload(3, 2)));
    // Now attempt to raise add_member_threshold 1 → 5, sole Owner, NO approvals.
    let amend = sign(&id_o, base(&id_o, group, AssertionType::RuleChange, 3, vec![], rule_change_payload(0, 5)));

    let sess = ingest_seq(&dir.path().join("t.redb"), &[&id_o], &id_o, &[&genesis, &raise, &amend]);
    let sum = sess.get_group_summary(&group).expect("summary");
    assert_eq!(sum.rules.rule_change_threshold, 2, "rule_change_threshold raised to 2");
    assert_eq!(
        sum.rules.add_member_threshold, 1,
        "amendment must be REJECTED — one signer cannot meet a 2-of-n RuleChange (V5′ enforced)"
    );
    eprintln!("RULECHANGE single-signer: add_member_threshold still 1 — 2-of-n enforced.");
}

#[tokio::test]
async fn two_distinct_personae_meet_rulechange_two_of_n() {
    let dir = tempfile::tempdir().expect("tempdir");
    let group = GroupId::new([0x42; 32]);
    let id_o = Identity::from_seed([0xE1; 32]);
    let id_a2 = Identity::from_seed([0xE2; 32]);
    let a2 = PrincipalId::new(id_a2.principal_id().0);
    let o_device = DeviceId::new(id_o.device_id().0);

    let amend_payload = rule_change_payload(0, 5); // raise add_member_threshold → 5
    let genesis = sign(&id_o, base(&id_o, group, AssertionType::GroupGenesis, 1, vec![], genesis_payload(&o_device)));
    let add_a2 = sign(&id_o, base(&id_o, group, AssertionType::MembershipAdd, 2, vec![], membership_add_payload(a2, 1))); // Admin
    let raise = sign(&id_o, base(&id_o, group, AssertionType::RuleChange, 3, vec![], rule_change_payload(3, 2)));
    // A2 approves the amendment (naming its content-hash subject); O submits it referencing that approval.
    let approval = sign(&id_a2, base(&id_a2, group, AssertionType::Approval, 1, vec![], approval_payload(AssertionType::RuleChange, rc_subject(&amend_payload))));
    let ant: Vec<Hash> = vec![envelope_hash(&approval)];
    let amend = sign(&id_o, base(&id_o, group, AssertionType::RuleChange, 4, ant, amend_payload));

    let sess = ingest_seq(&dir.path().join("t.redb"), &[&id_o, &id_a2], &id_o, &[&genesis, &add_a2, &raise, &approval, &amend]);
    let sum = sess.get_group_summary(&group).expect("summary");
    assert_eq!(
        sum.rules.add_member_threshold, 5,
        "amendment admitted — author O + approver A2 are two distinct personae, meeting the 2-of-n"
    );
    eprintln!("RULECHANGE two-personae: add_member_threshold = 5 — quorum of two distinct personae met.");
}

#[tokio::test]
async fn approval_for_a_different_change_does_not_count() {
    // An approval names a SPECIFIC proposed change by its content hash. An approval for
    // change X must not satisfy the quorum of a different change Y — otherwise the subject
    // would not bind approvals to the act they approve. (This is the case that pins
    // `rule_change_approval_subject` to be injective across payloads: a constant/degenerate
    // subject would let this through.)
    let dir = tempfile::tempdir().expect("tempdir");
    let group = GroupId::new([0x44; 32]);
    let id_o = Identity::from_seed([0xE4; 32]);
    let id_a2 = Identity::from_seed([0xE5; 32]);
    let a2 = PrincipalId::new(id_a2.principal_id().0);
    let o_device = DeviceId::new(id_o.device_id().0);

    let approved_payload = rule_change_payload(0, 5); // A2 approves raising add_member_threshold → 5
    let actual_payload = rule_change_payload(0, 9); // …but O submits a DIFFERENT change (→ 9)
    let genesis = sign(&id_o, base(&id_o, group, AssertionType::GroupGenesis, 1, vec![], genesis_payload(&o_device)));
    let add_a2 = sign(&id_o, base(&id_o, group, AssertionType::MembershipAdd, 2, vec![], membership_add_payload(a2, 1)));
    let raise = sign(&id_o, base(&id_o, group, AssertionType::RuleChange, 3, vec![], rule_change_payload(3, 2)));
    let approval = sign(&id_a2, base(&id_a2, group, AssertionType::Approval, 1, vec![], approval_payload(AssertionType::RuleChange, rc_subject(&approved_payload))));
    let ant: Vec<Hash> = vec![envelope_hash(&approval)];
    let amend = sign(&id_o, base(&id_o, group, AssertionType::RuleChange, 4, ant, actual_payload));

    let sess = ingest_seq(&dir.path().join("t.redb"), &[&id_o, &id_a2], &id_o, &[&genesis, &add_a2, &raise, &approval, &amend]);
    let sum = sess.get_group_summary(&group).expect("summary");
    assert_eq!(
        sum.rules.add_member_threshold, 1,
        "amendment must be REJECTED — A2's approval named a different change (→5), not this one (→9)"
    );
    eprintln!("RULECHANGE mismatched-approval: add_member_threshold still 1 — approvals bind to their change.");
}

#[tokio::test]
async fn self_approval_does_not_reach_rulechange_quorum() {
    let dir = tempfile::tempdir().expect("tempdir");
    let group = GroupId::new([0x43; 32]);
    let id_o = Identity::from_seed([0xE3; 32]);
    let o_device = DeviceId::new(id_o.device_id().0);

    let amend_payload = rule_change_payload(0, 5);
    let genesis = sign(&id_o, base(&id_o, group, AssertionType::GroupGenesis, 1, vec![], genesis_payload(&o_device)));
    let raise = sign(&id_o, base(&id_o, group, AssertionType::RuleChange, 2, vec![], rule_change_payload(3, 2)));
    // O approves its OWN amendment — same persona, must not count as a second.
    let self_approval = sign(&id_o, base(&id_o, group, AssertionType::Approval, 3, vec![], approval_payload(AssertionType::RuleChange, rc_subject(&amend_payload))));
    let ant: Vec<Hash> = vec![envelope_hash(&self_approval)];
    let amend = sign(&id_o, base(&id_o, group, AssertionType::RuleChange, 4, ant, amend_payload));

    let sess = ingest_seq(&dir.path().join("t.redb"), &[&id_o], &id_o, &[&genesis, &raise, &self_approval, &amend]);
    let sum = sess.get_group_summary(&group).expect("summary");
    assert_eq!(
        sum.rules.add_member_threshold, 1,
        "amendment must be REJECTED — a persona approving its own act is still one persona"
    );
    eprintln!("RULECHANGE self-approval: add_member_threshold still 1 — one persona cannot self-quorum.");
}
