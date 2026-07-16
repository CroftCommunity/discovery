//! Verification — the three governance acts the fold has always understood but no
//! client could issue — **`MembershipRemove`, `RoleGrant`, `RoleRevoke`** — are now
//! driven **end-to-end through the real `Session` emit API**, mirroring the proven
//! `rulechange_quorum_via_api.rs` shape (propose → approvals across sessions →
//! enacting act at quorum → the fold enforces it). Closes the RUN-02-era
//! Session-emit residual (SPEC-DIVERGENCE-REGISTER.md): the fold's
//! `apply_governance` remove/role arms and the generic `approve` command already
//! exist; this is the emit half.
//!
//! Earns/bounds: Part 2 §7.2 R7 — the same content-bound k-of-n counting and the
//! same co-signed-op antecedent pattern as RuleChange, now for the membership and
//! role acts. Fold/API level only: no MLS re-key linkage (that is `mls-replant`
//! territory, L2b+/app), no authority-model change (the existing role gate and
//! quorum as-is), no trust tier.
//!
//! Each test raises the relevant threshold to 2 (a real k-of-n), collects a
//! distinct-persona approval across sessions, enacts, and asserts the fold's
//! consequence: a removed persona's later facts are rejected; a granted role lets
//! the fold authorize an act the persona could not do before; a revoked role
//! withdraws that authorization. The result converges identically on a second
//! session (arrival-order independent at this grade; the reversed-order fold-level
//! independence for these act kinds is covered by `removed_then_included.rs` and
//! `role_thrash.rs`).

mod common;

use common::{has_member, replicate};
use local_storage_projection::types::{PrincipalId, Role};
use social_graph_core::{Identity, Session};

fn role_of(sess: &Session, group: &local_storage_projection::types::GroupId, who: &PrincipalId) -> Option<Role> {
    sess.get_group_summary(group)
        .expect("summary")
        .members
        .into_iter()
        .find(|m| &m.principal == who)
        .map(|m| m.role)
}

/// MembershipRemove at a 2-of-n quorum via the Session API; the removed persona's
/// later fact is rejected by a member who has folded the removal.
#[tokio::test]
async fn membership_remove_quorum_via_session_api() {
    let dir_o = tempfile::tempdir().expect("dir o");
    let dir_a2 = tempfile::tempdir().expect("dir a2");
    let dir_m = tempfile::tempdir().expect("dir m");
    let id_o = Identity::from_seed([0x11; 32]);
    let id_a2 = Identity::from_seed([0x12; 32]);
    let id_m = Identity::from_seed([0x13; 32]);
    let a2 = PrincipalId::new(id_a2.principal_id().0);
    let m = PrincipalId::new(id_m.principal_id().0);

    let sess_o = Session::open(&dir_o.path().join("o.redb"), &id_o).expect("open O");
    let sess_a2 = Session::open(&dir_a2.path().join("a2.redb"), &id_a2).expect("open A2");
    let sess_m = Session::open(&dir_m.path().join("m.redb"), &id_m).expect("open M");
    for s in [&sess_o, &sess_a2, &sess_m] {
        s.trust_peer(id_o.device_id(), id_o.principal_id());
        s.trust_peer(id_a2.device_id(), id_a2.principal_id());
        s.trust_peer(id_m.device_id(), id_m.principal_id());
    }

    // O founds, enrols A2 (Admin) and M (Member), and raises remove_member_threshold
    // 1 -> 2 (rule_key 1) while it is still 1.
    let group = sess_o.create_group().await.expect("create group");
    sess_o.add_member(&group, a2, Role::Admin).await.expect("add A2");
    sess_o.add_member(&group, m, Role::Member).await.expect("add M");
    sess_o.propose_rule_change(&group, 1, 2, vec![]).await.expect("raise remove_member_threshold");

    // A2 and M learn the group (M learns it is a Member, before any removal).
    replicate(&sess_o, &sess_a2, &group);
    replicate(&sess_o, &sess_m, &group);

    // A2 approves removing M; O enacts referencing the approval — a quorum of two.
    let approval = sess_a2.approve_remove_member(&group, m).await.expect("A2 approves remove M");
    replicate(&sess_a2, &sess_o, &group);
    sess_o.propose_remove_member(&group, m, vec![approval]).await.expect("remove M at quorum");

    assert!(!has_member(&sess_o, &group, &m), "M removed at O+A2 quorum");
    assert!(has_member(&sess_o, &group, &a2), "A2 retained");

    // Converges on A2 (identical across arrival orders at this grade).
    replicate(&sess_o, &sess_a2, &group);
    assert!(!has_member(&sess_a2, &group, &m), "A2 converges to the removal");

    // The removed persona authors a later fact from its stale local view; a member
    // who has folded the removal rejects it (removed => no authorizing role).
    let msg = sess_m.send_message(&group, "after my removal", None).await.expect("M posts locally");
    assert!(sess_m.get_message(&msg).is_some(), "M has its own post locally");
    replicate(&sess_m, &sess_o, &group);
    assert!(sess_o.get_message(&msg).is_none(), "O rejects the removed persona's later fact");

    eprintln!("REMOVE quorum via Session API: M removed at O+A2 quorum, converged; post-removal fact rejected.");
}

/// RoleGrant at a 2-of-n quorum via the Session API; the granted role lets the fold
/// authorize an Admin-only act the persona could not perform as a Member.
#[tokio::test]
async fn role_grant_quorum_via_session_api() {
    let dir_o = tempfile::tempdir().expect("dir o");
    let dir_a2 = tempfile::tempdir().expect("dir a2");
    let dir_m = tempfile::tempdir().expect("dir m");
    let id_o = Identity::from_seed([0x21; 32]);
    let id_a2 = Identity::from_seed([0x22; 32]);
    let id_m = Identity::from_seed([0x23; 32]);
    let id_n = Identity::from_seed([0x24; 32]);
    let a2 = PrincipalId::new(id_a2.principal_id().0);
    let m = PrincipalId::new(id_m.principal_id().0);
    let n = PrincipalId::new(id_n.principal_id().0);

    let sess_o = Session::open(&dir_o.path().join("o.redb"), &id_o).expect("open O");
    let sess_a2 = Session::open(&dir_a2.path().join("a2.redb"), &id_a2).expect("open A2");
    let sess_m = Session::open(&dir_m.path().join("m.redb"), &id_m).expect("open M");
    for s in [&sess_o, &sess_a2, &sess_m] {
        s.trust_peer(id_o.device_id(), id_o.principal_id());
        s.trust_peer(id_a2.device_id(), id_a2.principal_id());
        s.trust_peer(id_m.device_id(), id_m.principal_id());
        s.trust_peer(id_n.device_id(), id_n.principal_id());
    }

    // O founds, enrols A2 (Admin), M (Member), N (Member), raises role_change_threshold
    // 1 -> 2 (rule_key 2).
    let group = sess_o.create_group().await.expect("create group");
    sess_o.add_member(&group, a2, Role::Admin).await.expect("add A2");
    sess_o.add_member(&group, m, Role::Member).await.expect("add M");
    sess_o.add_member(&group, n, Role::Member).await.expect("add N");
    sess_o.propose_rule_change(&group, 2, 2, vec![]).await.expect("raise role_change_threshold");

    replicate(&sess_o, &sess_a2, &group);
    replicate(&sess_o, &sess_m, &group);
    assert_eq!(role_of(&sess_o, &group, &m), Some(Role::Member), "M starts a Member");

    // A2 approves granting M -> Admin; O enacts at quorum.
    let approval = sess_a2.approve_role_grant(&group, m).await.expect("A2 approves grant M");
    replicate(&sess_a2, &sess_o, &group);
    sess_o.propose_role_grant(&group, m, Role::Admin, vec![approval]).await.expect("grant M Admin at quorum");
    assert_eq!(role_of(&sess_o, &group, &m), Some(Role::Admin), "M is now Admin");

    // M learns the grant; as an Admin it can now remove a member (an act a Member is
    // not authorized for) — remove_member_threshold is still 1, so M acts alone.
    replicate(&sess_o, &sess_m, &group);
    sess_m.propose_remove_member(&group, n, vec![]).await.expect("M-as-Admin removes N");
    assert!(!has_member(&sess_m, &group, &n), "the granted role authorized M to remove N");

    // Converges on O: M is Admin, N is gone.
    replicate(&sess_m, &sess_o, &group);
    assert_eq!(role_of(&sess_o, &group, &m), Some(Role::Admin), "O converges: M Admin");
    assert!(!has_member(&sess_o, &group, &n), "O converges: N removed by the newly-granted Admin");

    eprintln!("ROLE-GRANT quorum via Session API: M Member->Admin at O+A2 quorum; granted role authorized a remove.");
}

/// RoleRevoke at a 2-of-n quorum via the Session API; the revoked role withdraws the
/// authorization the persona had, so an act it could perform as Admin is now rejected.
#[tokio::test]
async fn role_revoke_quorum_via_session_api() {
    let dir_o = tempfile::tempdir().expect("dir o");
    let dir_a2 = tempfile::tempdir().expect("dir a2");
    let dir_p = tempfile::tempdir().expect("dir p");
    let id_o = Identity::from_seed([0x31; 32]);
    let id_a2 = Identity::from_seed([0x32; 32]);
    let id_p = Identity::from_seed([0x33; 32]);
    let id_n = Identity::from_seed([0x34; 32]);
    let a2 = PrincipalId::new(id_a2.principal_id().0);
    let p = PrincipalId::new(id_p.principal_id().0);
    let n = PrincipalId::new(id_n.principal_id().0);

    let sess_o = Session::open(&dir_o.path().join("o.redb"), &id_o).expect("open O");
    let sess_a2 = Session::open(&dir_a2.path().join("a2.redb"), &id_a2).expect("open A2");
    let sess_p = Session::open(&dir_p.path().join("p.redb"), &id_p).expect("open P");
    for s in [&sess_o, &sess_a2, &sess_p] {
        s.trust_peer(id_o.device_id(), id_o.principal_id());
        s.trust_peer(id_a2.device_id(), id_a2.principal_id());
        s.trust_peer(id_p.device_id(), id_p.principal_id());
        s.trust_peer(id_n.device_id(), id_n.principal_id());
    }

    // O founds, enrols A2 (Admin), P (Admin), N (Member), raises role_change_threshold 1 -> 2.
    let group = sess_o.create_group().await.expect("create group");
    sess_o.add_member(&group, a2, Role::Admin).await.expect("add A2");
    sess_o.add_member(&group, p, Role::Admin).await.expect("add P");
    sess_o.add_member(&group, n, Role::Member).await.expect("add N");
    sess_o.propose_rule_change(&group, 2, 2, vec![]).await.expect("raise role_change_threshold");

    replicate(&sess_o, &sess_a2, &group);
    replicate(&sess_o, &sess_p, &group);
    assert_eq!(role_of(&sess_o, &group, &p), Some(Role::Admin), "P starts an Admin");

    // A2 approves revoking P's role; O enacts at quorum. Revoke demotes P to Member.
    let approval = sess_a2.approve_role_revoke(&group, p).await.expect("A2 approves revoke P");
    replicate(&sess_a2, &sess_o, &group);
    sess_o.propose_role_revoke(&group, p, vec![approval]).await.expect("revoke P at quorum");
    assert_eq!(role_of(&sess_o, &group, &p), Some(Role::Member), "P demoted to Member");

    // P learns the revoke; as a Member it can no longer remove N (Admin-only act).
    replicate(&sess_o, &sess_p, &group);
    let denied = sess_p.propose_remove_member(&group, n, vec![]).await;
    assert!(denied.is_err(), "the revoked role withdrew authorization: {denied:?}");
    assert!(has_member(&sess_o, &group, &n), "N was never removed");

    // Converges: a second session agrees P is a Member (identical across orders).
    replicate(&sess_o, &sess_a2, &group);
    assert_eq!(role_of(&sess_a2, &group, &p), Some(Role::Member), "A2 converges: P Member");

    eprintln!("ROLE-REVOKE quorum via Session API: P Admin->Member at O+A2 quorum; demoted persona's remove rejected.");
}
