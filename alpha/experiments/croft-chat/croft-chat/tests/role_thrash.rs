//! Verification — role thrash hard-stops, order-independently (Battery 6 / §7.6.1,
//! Rung B).
//!
//! The third concurrent-conflict shape, structurally identical to removed-then-included
//! but on ROLE rather than membership. Owner O grants X→Admin while owner O2 revokes X
//! (→Member), concurrently. Both are authorized (both owners), so — like removed-then-
//! included — this began as a silent, order-dependent last-writer-wins on X's role
//! ([grant, revoke] → Member; [revoke, grant] → Admin, both clean). §7.6.1 says "what
//! role?" has no causal answer here and must hard-stop.
//!
//! The fold detects a concurrent grant/revoke race on one subject and resolves it by
//! reverting X to its pre-thrash (base) role — no verdict on the contested change — and
//! flags Contradiction with the canonical pair label. Both orders reach the same role +
//! status. If it regresses to last-writer-wins, the refutation is back.

mod common;

use std::sync::Arc;

use common::{base, genesis_payload, membership_add_payload, sign};
use local_storage_projection::fold_derived::DerivedFold;
use local_storage_projection::tables::Db;
use local_storage_projection::types::envelope_hash;
use local_storage_projection::{AssertionEnvelope, AssertionType, DeviceId, GroupId, PrincipalId, Role};
use social_graph_core::{Ed25519Verifier, Identity, RegistryCredentialResolver, Session};

fn role_grant_payload(subject: PrincipalId, role_byte: u8) -> Vec<u8> {
    let mut p = subject.as_bytes().to_vec();
    p.push(role_byte);
    p
}
fn role_revoke_payload(subject: PrincipalId) -> Vec<u8> {
    subject.as_bytes().to_vec()
}

fn ingest_in_order(
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
            let _ = fold.ingest(env);
        }
    }
    Session::open(path, reader).expect("open session")
}

#[tokio::test]
async fn role_thrash_hard_stops_order_independently() {
    let dir = tempfile::tempdir().expect("tempdir");
    let group = GroupId::new([0xF0; 32]);

    let id_o = Identity::from_seed([0xC0; 32]);
    let id_o2 = Identity::from_seed([0xC1; 32]);
    let o2_principal = PrincipalId::new(id_o2.principal_id().0);
    let x_principal = PrincipalId::new([0xE5; 32]);
    let o_device = DeviceId::new(id_o.device_id().0);

    let genesis = sign(&id_o, base(&id_o, group, AssertionType::GroupGenesis, 1, vec![], genesis_payload(&o_device)));
    let add_o2 = sign(&id_o, base(&id_o, group, AssertionType::MembershipAdd, 2, vec![], membership_add_payload(o2_principal, 0))); // Owner
    let add_x = sign(&id_o, base(&id_o, group, AssertionType::MembershipAdd, 3, vec![], membership_add_payload(x_principal, 2))); // Member
    // Concurrent role race on X: O grants X→Admin, O2 revokes X.
    let grant_x = sign(&id_o, base(&id_o, group, AssertionType::RoleGrant, 4, vec![envelope_hash(&add_x)], role_grant_payload(x_principal, 1)));
    let revoke_x = sign(&id_o2, base(&id_o2, group, AssertionType::RoleRevoke, 1, vec![envelope_hash(&add_x)], role_revoke_payload(x_principal)));

    let authors = [&id_o, &id_o2];
    let setup = [&genesis, &add_o2, &add_x];

    let order1: Vec<&AssertionEnvelope> = setup.iter().copied().chain([&grant_x, &revoke_x]).collect();
    let order2: Vec<&AssertionEnvelope> = setup.iter().copied().chain([&revoke_x, &grant_x]).collect();
    let sess1 = ingest_in_order(&dir.path().join("o1.redb"), &authors, &id_o, &order1);
    let sess2 = ingest_in_order(&dir.path().join("o2.redb"), &authors, &id_o, &order2);

    let s1 = sess1.get_group_summary(&group).expect("summary 1");
    let s2 = sess2.get_group_summary(&group).expect("summary 2");
    let role1 = s1.members.iter().find(|m| m.principal == x_principal).map(|m| m.role.clone());
    let role2 = s2.members.iter().find(|m| m.principal == x_principal).map(|m| m.role.clone());
    eprintln!(
        "ROLE-THRASH: order1(grant,revoke) -> X_role={role1:?} fork={:?}; order2(revoke,grant) -> X_role={role2:?} fork={:?}",
        s1.fork_status, s2.fork_status
    );

    // Fixed behaviour: surfaced as a contradiction, X at its base role, order-independent.
    assert!(s1.fork_status.starts_with("contradiction"), "order 1 hard-stops, got {}", s1.fork_status);
    assert!(s2.fork_status.starts_with("contradiction"), "order 2 hard-stops, got {}", s2.fork_status);
    assert_eq!(role1, Some(Role::Member), "order 1 reverts X to base role (no verdict on the contested change)");
    assert_eq!(role2, Some(Role::Member), "order 2 reverts X to base role");
    assert_eq!(role1, role2, "X's role no longer depends on arrival order");
    assert_eq!(s1.fork_status, s2.fork_status, "both orders surface the same canonical contradiction status");

    eprintln!(
        "ROLE-THRASH RESULT (fix verified): a concurrent grant/revoke race on one subject \
         now hard-stops as a contradiction with X reverted to its base role in both orders \
         — no order-dependent last-writer verdict. The detect/resolve machinery covers the \
         third §7.6.1 concurrent shape."
    );
}

/// grant-vs-grant sub-case: two concurrent grants of the subject to *different* roles.
/// Same shape as grant/revoke — last-writer-wins on the role, order-dependent — and the
/// same resolution (revert to base role + Contradiction). The generalized detector keys
/// on the two facts' *resulting roles* differing, so grant-vs-grant to the same role, and
/// revoke-vs-revoke, stay benign.
#[tokio::test]
async fn role_thrash_grant_vs_grant_hard_stops() {
    let dir = tempfile::tempdir().expect("tempdir");
    let group = GroupId::new([0xF1; 32]);

    let id_o = Identity::from_seed([0xD0; 32]);
    let id_o2 = Identity::from_seed([0xD1; 32]);
    let o2_principal = PrincipalId::new(id_o2.principal_id().0);
    let x_principal = PrincipalId::new([0xE6; 32]);
    let o_device = DeviceId::new(id_o.device_id().0);

    let genesis = sign(&id_o, base(&id_o, group, AssertionType::GroupGenesis, 1, vec![], genesis_payload(&o_device)));
    let add_o2 = sign(&id_o, base(&id_o, group, AssertionType::MembershipAdd, 2, vec![], membership_add_payload(o2_principal, 0)));
    let add_x = sign(&id_o, base(&id_o, group, AssertionType::MembershipAdd, 3, vec![], membership_add_payload(x_principal, 2)));
    // Concurrent grants of X to DIFFERENT roles: O → Admin(1), O2 → Owner(0).
    let grant_admin = sign(&id_o, base(&id_o, group, AssertionType::RoleGrant, 4, vec![envelope_hash(&add_x)], role_grant_payload(x_principal, 1)));
    let grant_owner = sign(&id_o2, base(&id_o2, group, AssertionType::RoleGrant, 1, vec![envelope_hash(&add_x)], role_grant_payload(x_principal, 0)));

    let authors = [&id_o, &id_o2];
    let setup = [&genesis, &add_o2, &add_x];
    let order1: Vec<&AssertionEnvelope> = setup.iter().copied().chain([&grant_admin, &grant_owner]).collect();
    let order2: Vec<&AssertionEnvelope> = setup.iter().copied().chain([&grant_owner, &grant_admin]).collect();
    let sess1 = ingest_in_order(&dir.path().join("g1.redb"), &authors, &id_o, &order1);
    let sess2 = ingest_in_order(&dir.path().join("g2.redb"), &authors, &id_o, &order2);

    let s1 = sess1.get_group_summary(&group).expect("summary 1");
    let s2 = sess2.get_group_summary(&group).expect("summary 2");
    let role1 = s1.members.iter().find(|m| m.principal == x_principal).map(|m| m.role.clone());
    let role2 = s2.members.iter().find(|m| m.principal == x_principal).map(|m| m.role.clone());
    eprintln!(
        "ROLE-THRASH (grant/grant): order1 -> X_role={role1:?} fork={:?}; order2 -> X_role={role2:?} fork={:?}",
        s1.fork_status, s2.fork_status
    );

    assert!(s1.fork_status.starts_with("contradiction"), "order 1 hard-stops, got {}", s1.fork_status);
    assert!(s2.fork_status.starts_with("contradiction"), "order 2 hard-stops, got {}", s2.fork_status);
    assert_eq!(role1, Some(Role::Member), "order 1 reverts X to base role");
    assert_eq!(role2, Some(Role::Member), "order 2 reverts X to base role");
    assert_eq!(s1.fork_status, s2.fork_status, "same canonical contradiction status");

    eprintln!("ROLE-THRASH (grant/grant) RESULT (fix verified): two concurrent grants to different roles hard-stop, order-independent.");
}

/// Boundary: two concurrent grants to the SAME role commute (same resulting role), so
/// they must NOT trip the hard-stop — the false-trip guard for the generalized
/// resulting-role predicate.
#[tokio::test]
async fn benign_concurrent_same_role_grants_stay_clean() {
    let dir = tempfile::tempdir().expect("tempdir");
    let group = GroupId::new([0xF2; 32]);

    let id_o = Identity::from_seed([0xD4; 32]);
    let id_o2 = Identity::from_seed([0xD5; 32]);
    let o2_principal = PrincipalId::new(id_o2.principal_id().0);
    let x_principal = PrincipalId::new([0xE7; 32]);
    let o_device = DeviceId::new(id_o.device_id().0);

    let genesis = sign(&id_o, base(&id_o, group, AssertionType::GroupGenesis, 1, vec![], genesis_payload(&o_device)));
    let add_o2 = sign(&id_o, base(&id_o, group, AssertionType::MembershipAdd, 2, vec![], membership_add_payload(o2_principal, 0)));
    let add_x = sign(&id_o, base(&id_o, group, AssertionType::MembershipAdd, 3, vec![], membership_add_payload(x_principal, 2)));
    // Both grant X → Admin(1) — same resulting role, concurrent.
    let grant1 = sign(&id_o, base(&id_o, group, AssertionType::RoleGrant, 4, vec![envelope_hash(&add_x)], role_grant_payload(x_principal, 1)));
    let grant2 = sign(&id_o2, base(&id_o2, group, AssertionType::RoleGrant, 1, vec![envelope_hash(&add_x)], role_grant_payload(x_principal, 1)));

    let authors = [&id_o, &id_o2];
    let setup = [&genesis, &add_o2, &add_x];
    let order1: Vec<&AssertionEnvelope> = setup.iter().copied().chain([&grant1, &grant2]).collect();
    let order2: Vec<&AssertionEnvelope> = setup.iter().copied().chain([&grant2, &grant1]).collect();
    let sess1 = ingest_in_order(&dir.path().join("s1.redb"), &authors, &id_o, &order1);
    let sess2 = ingest_in_order(&dir.path().join("s2.redb"), &authors, &id_o, &order2);

    let s1 = sess1.get_group_summary(&group).expect("summary 1");
    let s2 = sess2.get_group_summary(&group).expect("summary 2");
    let role1 = s1.members.iter().find(|m| m.principal == x_principal).map(|m| m.role.clone());
    let role2 = s2.members.iter().find(|m| m.principal == x_principal).map(|m| m.role.clone());

    assert_eq!(s1.fork_status, "clean", "same-role grants must not trip (order 1)");
    assert_eq!(s2.fork_status, "clean", "same-role grants must not trip (order 2)");
    assert_eq!(role1, Some(Role::Admin), "X is Admin (order 1)");
    assert_eq!(role2, Some(Role::Admin), "X is Admin (order 2)");
    assert_eq!(role1, role2, "converge");

    eprintln!("SAME-ROLE-GRANTS RESULT (boundary): two concurrent grants to the same role converge to Admin, clean — no false trip.");
}
