//! G3 — duplicate and reordered delivery is idempotent (Battery 5, Rung B).
//!
//! Earns/bounds: Part 2 §6.6.4 (and §6 content-addressing) — content-addressed dedup makes duplicate and reordered delivery idempotent.
//!
//! Completes the delivery-adversary set: G1/G2 cover *loss*; G3 covers *duplication
//! and reorder*. A gossip medium redelivers and reorders freely, so the claim is
//! that content-addressed dedup (§6) makes redelivery a no-op — a node that receives
//! every frame several times, out of order, converges to exactly the head of a node
//! that received each once, in order.
//!
//! Same cross-device set as G1 (owner O: genesis, α add A2 Admin, R RuleChange 1→7;
//! admin A2: β add X). One node gets the clean single delivery; the other gets every
//! frame three times, scrambled. Their fingerprints must match, and X must be a
//! member exactly once.
//!
//! Falsifies if: the duplicate/reordered node reaches a different fingerprint, or a
//! duplicate is applied twice (X appearing more than once, or a shifted head).

mod common;

use common::{
    base, drive, frame, genesis_payload, membership_add_payload, rule_change_payload, sign,
};
use croft_chat::fingerprint::fingerprint;
use local_storage_projection::types::envelope_hash;
use local_storage_projection::{AssertionType, DeviceId, GroupId, PrincipalId};
use social_graph_core::Identity;

#[tokio::test]
async fn duplicate_and_reordered_delivery_is_idempotent() {
    let dir = tempfile::tempdir().expect("tempdir");
    let group = GroupId::new([0x99; 32]);

    let id_o = Identity::from_seed([0x30; 32]);
    let id_a2 = Identity::from_seed([0x31; 32]);
    let a2_principal = PrincipalId::new(id_a2.principal_id().0);
    let x_principal = PrincipalId::new([0x33; 32]);
    let o_device = DeviceId::new(id_o.device_id().0);

    let genesis = sign(
        &id_o,
        base(&id_o, group, AssertionType::GroupGenesis, 1, vec![], genesis_payload(&o_device)),
    );
    let alpha = sign(
        &id_o,
        base(&id_o, group, AssertionType::MembershipAdd, 2, vec![], membership_add_payload(a2_principal, 1)),
    );
    let r = sign(
        &id_o,
        base(&id_o, group, AssertionType::RuleChange, 3, vec![], rule_change_payload(1, 7)),
    );
    let beta = sign(
        &id_a2,
        base(&id_a2, group, AssertionType::MembershipAdd, 1, vec![envelope_hash(&r)], membership_add_payload(x_principal, 2)),
    );

    let authors = [&id_o, &id_a2];

    // Clean node: each frame once, in causal order.
    let sess_clean = drive(
        &dir.path().join("clean.redb"),
        &id_o,
        &authors,
        vec![frame(&genesis), frame(&alpha), frame(&r), frame(&beta)],
    );

    // Adversarial node: every frame three times, scrambled.
    let noisy = vec![
        frame(&beta),
        frame(&genesis),
        frame(&r),
        frame(&beta),
        frame(&alpha),
        frame(&genesis),
        frame(&r),
        frame(&alpha),
        frame(&beta),
        frame(&genesis),
        frame(&alpha),
        frame(&r),
    ];
    let sess_dup = drive(&dir.path().join("dup.redb"), &id_o, &authors, noisy);

    // Same head.
    let fp_clean = fingerprint(&sess_clean, &group);
    let fp_dup = fingerprint(&sess_dup, &group);
    assert_eq!(
        fp_dup, fp_clean,
        "duplicate + reordered delivery must converge to the clean head"
    );

    // X applied exactly once (not duplicated into the member set).
    let sum = sess_dup.get_group_summary(&group).expect("summary");
    let x_count = sum.members.iter().filter(|m| m.principal == x_principal).count();
    assert_eq!(x_count, 1, "X must be a member exactly once, got {x_count}");
    assert_eq!(sum.rules.remove_member_threshold, 7, "R applied once");

    eprintln!(
        "G3 RESULT (corroboration): a node fed every frame 3× and scrambled reaches \
         the identical fingerprint to clean single delivery; X is a member exactly \
         once. Content-addressed dedup makes redelivery and reorder no-ops."
    );
}
