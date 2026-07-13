//! Verification — removed-then-included hard-stops, order-independently
//! (Battery 6 / §7.6.1, Rung B).
//!
//! The second §7.6.1 conflict shape, and a *different* mechanism from mutual
//! expulsion. There, each remove struck the other's author, so the second was dropped
//! at authorization. Here both actors survive: admin A removes X while admin B
//! (re-)includes X, concurrently. Neither author is removed, so both facts are
//! authorized — and this began as a refutation: the final presence of X was decided
//! last-writer-wins ([remove, add] → X present; [add, remove] → X absent), both
//! `clean`. A silent, order-dependent divergence on the same subject.
//!
//! The fold now detects it on the authorized path: an add/remove race on one subject
//! with the two facts causally concurrent. It resolves inclusively (replay excluding
//! the remove → X retained, no verdict) and flags `Contradiction` with the canonical
//! pair label. This test verifies the closed behaviour: both arrival orders retain X
//! and surface the same contradiction status. If it regresses to last-writer-wins,
//! the refutation is back. (Inclusive resolution is the conservative default — do not
//! drop a member on a contested basis; the humans decide via fork.)

mod common;

use std::sync::Arc;

use common::{base, genesis_payload, membership_add_payload, sign};
use local_storage_projection::fold_derived::DerivedFold;
use local_storage_projection::tables::Db;
use local_storage_projection::types::envelope_hash;
use local_storage_projection::{AssertionEnvelope, AssertionType, DeviceId, GroupId, PrincipalId};
use social_graph_core::{Ed25519Verifier, Identity, RegistryCredentialResolver, Session};

fn remove_payload(subject: PrincipalId) -> Vec<u8> {
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
async fn removed_then_included_hard_stops_order_independently() {
    let dir = tempfile::tempdir().expect("tempdir");
    let group = GroupId::new([0xDD; 32]);

    let id_o = Identity::from_seed([0xB0; 32]);
    let id_a = Identity::from_seed([0xB1; 32]);
    let id_b = Identity::from_seed([0xB2; 32]);
    let a_principal = PrincipalId::new(id_a.principal_id().0);
    let b_principal = PrincipalId::new(id_b.principal_id().0);
    let x_principal = PrincipalId::new([0xE0; 32]);
    let o_device = DeviceId::new(id_o.device_id().0);

    let genesis = sign(&id_o, base(&id_o, group, AssertionType::GroupGenesis, 1, vec![], genesis_payload(&o_device)));
    let add_a = sign(&id_o, base(&id_o, group, AssertionType::MembershipAdd, 2, vec![], membership_add_payload(a_principal, 1)));
    let add_b = sign(&id_o, base(&id_o, group, AssertionType::MembershipAdd, 3, vec![], membership_add_payload(b_principal, 1)));
    let add_x = sign(&id_o, base(&id_o, group, AssertionType::MembershipAdd, 4, vec![], membership_add_payload(x_principal, 2)));
    // Concurrent race on X: A removes X, B re-includes X. Both authors survive.
    let a_removes_x = sign(&id_a, base(&id_a, group, AssertionType::MembershipRemove, 1, vec![envelope_hash(&add_x)], remove_payload(x_principal)));
    let b_adds_x = sign(&id_b, base(&id_b, group, AssertionType::MembershipAdd, 1, vec![envelope_hash(&add_x)], membership_add_payload(x_principal, 2)));

    let authors = [&id_o, &id_a, &id_b];
    let setup = [&genesis, &add_a, &add_b, &add_x];

    let order1: Vec<&AssertionEnvelope> = setup.iter().copied().chain([&a_removes_x, &b_adds_x]).collect();
    let order2: Vec<&AssertionEnvelope> = setup.iter().copied().chain([&b_adds_x, &a_removes_x]).collect();
    let sess1 = ingest_in_order(&dir.path().join("o1.redb"), &authors, &id_o, &order1);
    let sess2 = ingest_in_order(&dir.path().join("o2.redb"), &authors, &id_o, &order2);

    let s1 = sess1.get_group_summary(&group).expect("summary 1");
    let s2 = sess2.get_group_summary(&group).expect("summary 2");
    let x1 = s1.members.iter().any(|m| m.principal == x_principal);
    let x2 = s2.members.iter().any(|m| m.principal == x_principal);
    eprintln!(
        "REMOVED-THEN-INCLUDED: order1(remove,add) -> X={x1} fork={:?}; order2(add,remove) -> X={x2} fork={:?}",
        s1.fork_status, s2.fork_status
    );

    // Fixed behaviour: the add/remove race is SURFACED as a contradiction and resolved
    // inclusively (X retained — no verdict), order-independently.
    assert!(s1.fork_status.starts_with("contradiction"), "order 1 hard-stops, got {}", s1.fork_status);
    assert!(s2.fork_status.starts_with("contradiction"), "order 2 hard-stops, got {}", s2.fork_status);
    assert!(x1 && x2, "both orders retain X (inclusive resolution, no verdict)");
    assert_eq!(x1, x2, "X's presence no longer depends on arrival order");
    assert_eq!(s1.fork_status, s2.fork_status, "both orders surface the same canonical contradiction status");

    eprintln!(
        "REMOVED-THEN-INCLUDED RESULT (fix verified): a concurrent add/remove race on the \
         same subject now hard-stops as a contradiction with X retained in both orders — \
         no order-dependent last-writer verdict, no silent resolution. The detect/resolve \
         machinery extends to this second §7.6.1 shape."
    );
}
