//! Boundary — benign concurrent removes converge and stay clean (Battery 6 / §7.6.1,
//! Rung B).
//!
//! The companion to the mutual-expulsion refutation, and the false-trip guard any fix
//! must respect. Concurrency is *necessary but not sufficient* for a §7.6.1
//! contradiction: two admins concurrently removing *different* members commute, so
//! they must converge to one head and stay `clean`. Flagging them would erode the
//! escalation channel §7.5.2/§7.6 says must not be widened.
//!
//! This sharpens the mutual-expulsion finding: the problem there is not concurrency
//! per se (this converges fine) but *conflicting* concurrency (mutual expulsion),
//! which is undetected. It also pins the boundary in advance — when the reconcile
//! hard-stop lands, this test must still pass unchanged.
//!
//! Scenario: O owner; O adds A, B as Admin and C, D as Member. A removes C, B removes
//! D — concurrent, disjoint subjects. Fed to two folds in opposite orders.

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
async fn benign_concurrent_removes_converge_and_stay_clean() {
    let dir = tempfile::tempdir().expect("tempdir");
    let group = GroupId::new([0xCC; 32]);

    let id_o = Identity::from_seed([0xA0; 32]);
    let id_a = Identity::from_seed([0xA1; 32]);
    let id_b = Identity::from_seed([0xA2; 32]);
    let c_principal = PrincipalId::new([0xC0; 32]);
    let d_principal = PrincipalId::new([0xD0; 32]);
    let a_principal = PrincipalId::new(id_a.principal_id().0);
    let b_principal = PrincipalId::new(id_b.principal_id().0);
    let o_device = DeviceId::new(id_o.device_id().0);

    let genesis = sign(
        &id_o,
        base(&id_o, group, AssertionType::GroupGenesis, 1, vec![], genesis_payload(&o_device)),
    );
    let add_a = sign(&id_o, base(&id_o, group, AssertionType::MembershipAdd, 2, vec![], membership_add_payload(a_principal, 1)));
    let add_b = sign(&id_o, base(&id_o, group, AssertionType::MembershipAdd, 3, vec![], membership_add_payload(b_principal, 1)));
    let add_c = sign(&id_o, base(&id_o, group, AssertionType::MembershipAdd, 4, vec![], membership_add_payload(c_principal, 2)));
    let add_d = sign(&id_o, base(&id_o, group, AssertionType::MembershipAdd, 5, vec![], membership_add_payload(d_principal, 2)));
    // Concurrent, DISJOINT: A removes C, B removes D.
    let a_removes_c = sign(&id_a, base(&id_a, group, AssertionType::MembershipRemove, 1, vec![envelope_hash(&add_c)], remove_payload(c_principal)));
    let b_removes_d = sign(&id_b, base(&id_b, group, AssertionType::MembershipRemove, 1, vec![envelope_hash(&add_d)], remove_payload(d_principal)));

    let authors = [&id_o, &id_a, &id_b];
    let setup = [&genesis, &add_a, &add_b, &add_c, &add_d];

    let order1: Vec<&AssertionEnvelope> = setup.iter().copied().chain([&a_removes_c, &b_removes_d]).collect();
    let order2: Vec<&AssertionEnvelope> = setup.iter().copied().chain([&b_removes_d, &a_removes_c]).collect();
    let sess1 = ingest_in_order(&dir.path().join("o1.redb"), &authors, &id_o, &order1);
    let sess2 = ingest_in_order(&dir.path().join("o2.redb"), &authors, &id_o, &order2);

    let s1 = sess1.get_group_summary(&group).expect("summary 1");
    let s2 = sess2.get_group_summary(&group).expect("summary 2");

    // Both orders: clean, and converge to the same membership {O, A, B}.
    assert_eq!(s1.fork_status, "clean", "benign concurrent removes must not trip a hard-stop (order 1)");
    assert_eq!(s2.fork_status, "clean", "benign concurrent removes must not trip a hard-stop (order 2)");
    let members = |s: &social_graph_core::GroupSummaryView| {
        let mut v: Vec<[u8; 32]> = s.members.iter().map(|m| *m.principal.as_bytes()).collect();
        v.sort_unstable();
        v
    };
    assert_eq!(members(&s1), members(&s2), "the two orders converge to identical membership");
    assert!(!s1.members.iter().any(|m| m.principal == c_principal || m.principal == d_principal), "C and D removed");
    assert!(
        s1.members.iter().any(|m| m.principal == a_principal) && s1.members.iter().any(|m| m.principal == b_principal),
        "A and B remain"
    );

    eprintln!(
        "BENIGN-CONCURRENT RESULT (corroboration): two admins concurrently removing \
         DIFFERENT members converge to identical membership {{O, A, B}} in both orders, \
         both clean. The problem is conflicting concurrency (mutual expulsion), not \
         concurrency itself — and this is the boundary the reconcile fix must preserve."
    );
}
