//! Verification — mutual expulsion hard-stops as a contradiction, order-independently
//! (Battery 6 / §7.6.1, Rung B).
//!
//! §2.5 names the *canonical* irreducible residue: A expels B while B expels A at
//! equal standing. There is no fact about who should remain, so §7.6.1 says this
//! "too many valid claims" contradiction must hard-stop for the humans (a fork), not
//! be resolved by any tiebreak. This test began as a refutation: the fold silently
//! auto-resolved it to an *order-dependent* survivor (order `[A⊗B, B⊗A]` → `{O,A}`;
//! order `[B⊗A, A⊗B]` → `{O,B}`; both `clean`) — a silent I5 violation, because
//! collision detection was genesis-only (concurrent non-genesis facts get sequential
//! slots) and authorization-at-position made it first-fold-wins.
//!
//! The fold now detects it. When a `MembershipRemove` whose author was removed by a
//! *concurrent, mutually-expelling* remove is folded, the fold recomputes membership
//! by replaying the log in canonical order excluding the partner — retaining BOTH
//! contested parties (no verdict) — and flags `Contradiction`. This test verifies the
//! closed behaviour: two folds fed the same five facts in opposite orders both reach
//! `{O, A, B}` + contradiction, identically. If it regresses to an order-dependent
//! survivor, the refutation is back.

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

/// Ingest `order` into a fresh store via the fold directly (explicit application
/// order; a remove whose author is already removed is rejected and ignored).
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
            let _ = fold.ingest(env); // authorized facts apply; unauthorized removes drop
        }
    } // db + fold dropped, releasing the redb lock before Session::open
    Session::open(path, reader).expect("open session")
}

#[tokio::test]
async fn mutual_expulsion_hard_stops_order_independently() {
    let dir = tempfile::tempdir().expect("tempdir");
    let group = GroupId::new([0xEE; 32]);

    let id_o = Identity::from_seed([0x90; 32]);
    let id_a = Identity::from_seed([0x91; 32]);
    let id_b = Identity::from_seed([0x92; 32]);
    let a_principal = PrincipalId::new(id_a.principal_id().0);
    let b_principal = PrincipalId::new(id_b.principal_id().0);
    let o_device = DeviceId::new(id_o.device_id().0);

    let genesis = sign(
        &id_o,
        base(&id_o, group, AssertionType::GroupGenesis, 1, vec![], genesis_payload(&o_device)),
    );
    let add_a = sign(
        &id_o,
        base(&id_o, group, AssertionType::MembershipAdd, 2, vec![], membership_add_payload(a_principal, 1)),
    );
    let add_b = sign(
        &id_o,
        base(&id_o, group, AssertionType::MembershipAdd, 3, vec![], membership_add_payload(b_principal, 1)),
    );
    let a_removes_b = sign(
        &id_a,
        base(&id_a, group, AssertionType::MembershipRemove, 1, vec![envelope_hash(&add_b)], remove_payload(b_principal)),
    );
    let b_removes_a = sign(
        &id_b,
        base(&id_b, group, AssertionType::MembershipRemove, 1, vec![envelope_hash(&add_a)], remove_payload(a_principal)),
    );

    let authors = [&id_o, &id_a, &id_b];
    let setup = [&genesis, &add_a, &add_b];

    // Order 1: A's remove folded first.
    let order1: Vec<&AssertionEnvelope> =
        setup.iter().copied().chain([&a_removes_b, &b_removes_a]).collect();
    let sess1 = ingest_in_order(&dir.path().join("o1.redb"), &authors, &id_o, &order1);

    // Order 2: B's remove folded first.
    let order2: Vec<&AssertionEnvelope> =
        setup.iter().copied().chain([&b_removes_a, &a_removes_b]).collect();
    let sess2 = ingest_in_order(&dir.path().join("o2.redb"), &authors, &id_o, &order2);

    let s1 = sess1.get_group_summary(&group).expect("summary 1");
    let s2 = sess2.get_group_summary(&group).expect("summary 2");
    let a1 = s1.members.iter().any(|m| m.principal == a_principal);
    let b1 = s1.members.iter().any(|m| m.principal == b_principal);
    let a2 = s2.members.iter().any(|m| m.principal == a_principal);
    let b2 = s2.members.iter().any(|m| m.principal == b_principal);
    eprintln!(
        "MUTUAL-EXPULSION: order1 -> A={a1} B={b1} fork={:?}; order2 -> A={a2} B={b2} fork={:?}",
        s1.fork_status, s2.fork_status
    );

    // Fixed behaviour (1): the contradiction is SURFACED as a hard-stop, not silently
    // auto-resolved. Both orders reach a Contradiction status.
    assert!(
        s1.fork_status.starts_with("contradiction"),
        "order 1 must hard-stop as a contradiction, got {}",
        s1.fork_status
    );
    assert!(
        s2.fork_status.starts_with("contradiction"),
        "order 2 must hard-stop as a contradiction, got {}",
        s2.fork_status
    );

    // Fixed behaviour (2): no verdict — BOTH contested parties are retained — and the
    // outcome is now ORDER-INDEPENDENT (the two orders reach identical membership).
    assert!(a1 && b1, "order 1 retains both A and B (no verdict rendered)");
    assert!(a2 && b2, "order 2 retains both A and B (no verdict rendered)");
    assert_eq!(
        (a1, b1),
        (a2, b2),
        "both arrival orders reach identical membership — the order-dependent \
         divergence is gone"
    );
    assert_eq!(
        s1.fork_status, s2.fork_status,
        "both orders surface the same canonical contradiction status"
    );

    eprintln!(
        "MUTUAL-EXPULSION RESULT (fix verified): both arrival orders hard-stop as a \
         contradiction with both A and B retained ({{O,A,B}}) — no order-dependent \
         verdict, no silent resolution. The §7.6.1 contradiction hard-stop now fires \
         for concurrent governance conflict, not only genesis collisions."
    );
}
