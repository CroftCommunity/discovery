//! Verification — a RuleChange k-of-n quorum is driven **end-to-end through the real
//! `Session` emit API** (not hand-crafted envelopes), across two live sessions that
//! replicate. This reconciles the emit half of the `handcrafted-assertions` spec-delta:
//! `Session` can now emit `RuleChange` and `Approval`, so a well-formed governance flow
//! no longer needs hand-built assertions.
//!
//! Earns/bounds: Part 2 §7.2 R7 — a RuleChange k-of-n quorum driven end-to-end through the real Session emit API.
//!
//! Flow: O creates the group, adds A2 (Admin), and raises rule_change_threshold to 2.
//! A lone amendment by O is refused. A2 approves the specific change; O submits it
//! referencing that approval; the quorum of two distinct personae admits it, and A2
//! converges. Everything is emitted via `Session::{propose_rule_change, approve_rule_change,
//! add_member, create_group}` and moved over the real replication path.

mod common;

use common::replicate;
use local_storage_projection::types::{PrincipalId, Role};
use social_graph_core::{Identity, Session};

#[tokio::test]
async fn rulechange_quorum_end_to_end_via_session_api() {
    let dir_o = tempfile::tempdir().expect("dir o");
    let dir_a2 = tempfile::tempdir().expect("dir a2");
    let id_o = Identity::from_seed([0x51; 32]);
    let id_a2 = Identity::from_seed([0x52; 32]);
    let a2 = PrincipalId::new(id_a2.principal_id().0);

    let sess_o = Session::open(&dir_o.path().join("o.redb"), &id_o).expect("open O");
    let sess_a2 = Session::open(&dir_a2.path().join("a2.redb"), &id_a2).expect("open A2");
    sess_o.trust_peer(id_a2.device_id(), id_a2.principal_id());
    sess_a2.trust_peer(id_o.device_id(), id_o.principal_id());

    // O founds the group, enrolls A2 as Admin, and raises rule_change_threshold 1 → 2
    // (allowed while the threshold is still 1). rule_key 3 == RuleChange.
    let group = sess_o.create_group().await.expect("create group");
    sess_o.add_member(&group, a2, Role::Admin).await.expect("add A2");
    sess_o.propose_rule_change(&group, 3, 2, vec![]).await.expect("raise rule_change_threshold");

    // A2 learns the group + threshold, then approves the amendment (→5); O references the
    // approval and submits — a quorum of two distinct personae. (Success first, so O's
    // device chain stays contiguous for replication; the lone-refusal check trails.)
    replicate(&sess_o, &sess_a2, &group);
    let approval = sess_a2.approve_rule_change(&group, 0, 5).await.expect("A2 approves");
    replicate(&sess_a2, &sess_o, &group);
    sess_o
        .propose_rule_change(&group, 0, 5, vec![approval])
        .await
        .expect("amendment admitted at quorum");

    assert_eq!(
        sess_o.get_group_summary(&group).expect("summary").rules.add_member_threshold, 5,
        "amendment applied — O + A2 quorum via the real Session emit API"
    );

    // It converges on A2.
    replicate(&sess_o, &sess_a2, &group);
    assert_eq!(
        sess_a2.get_group_summary(&group).expect("summary").rules.add_member_threshold, 5,
        "A2 converges to the amended rule"
    );

    // Trailing refutation: a lone amendment (remove_member_threshold → 7, no approvals)
    // is refused now that rule_change_threshold is 2. (Last op, so its consumed lamport
    // needs no successor.)
    let lone = sess_o.propose_rule_change(&group, 1, 7, vec![]).await;
    assert!(lone.is_err(), "lone owner cannot meet the 2-of-n RuleChange: {lone:?}");
    assert_eq!(
        sess_o.get_group_summary(&group).expect("summary").rules.remove_member_threshold, 1,
        "refused amendment left the rule unchanged"
    );
    eprintln!("RULECHANGE quorum via Session API: add_member_threshold 1→5 at O+A2 quorum, converged; lone amend refused.");
}
