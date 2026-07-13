//! Incompleteness is observable: a held-back peer knows it is behind
//! (Battery 5 follow-up, Rung B).
//!
//! The G1 guard makes a lagging node *correct* (it never folds a governance fact
//! whose antecedent it lacks). This closes the honesty half: the node can also
//! *tell* it is behind, instead of presenting its stale-but-valid prefix as if it
//! were current. `Replicator::pending_len` counts received-but-unfoldable frames —
//! nonzero means "I have seen facts I cannot yet apply; I am catching up."
//!
//! Same cross-device scenario as G1. With R withheld, β (which declares R as its
//! antecedent) is held, so the node is NOT settled — pending_len > 0. When R
//! arrives, β folds and the node settles — pending_len == 0.

mod common;

use common::{
    base, frame, genesis_payload, has_member, membership_add_payload, pump_until_quiet,
    rule_change_payload, sign, QueueBus,
};
use croft_chat::sync::Replicator;
use croft_chat::transport::{Topic, Transport};
use local_storage_projection::types::envelope_hash;
use local_storage_projection::{AssertionType, DeviceId, GroupId, PrincipalId};
use social_graph_core::{Identity, Session};

#[tokio::test]
async fn a_held_back_peer_reports_it_is_incomplete() {
    let dir = tempfile::tempdir().expect("tempdir");
    let group = GroupId::new([0xBE; 32]);

    let id_o = Identity::from_seed([0x80; 32]);
    let id_a2 = Identity::from_seed([0x81; 32]);
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

    let session = Session::open(&dir.path().join("b.redb"), &id_o).expect("open");
    session.trust_peer(id_o.device_id(), id_o.principal_id());
    session.trust_peer(id_a2.device_id(), id_a2.principal_id());
    let mut bus = QueueBus::default();
    bus.subscribe(&Topic("drystone/completeness".to_string()));
    let mut repl = Replicator::new();

    // Phase 1: R withheld. β is held → the node is NOT settled and says so.
    bus.inject(vec![frame(&beta), frame(&genesis), frame(&alpha)]);
    pump_until_quiet(&session, &mut bus, &mut repl);
    assert!(!has_member(&session, &group, &x_principal), "β held: X absent");
    assert!(
        repl.pending_len() >= 1,
        "the node must report it is holding a fact (pending_len > 0), not look settled"
    );
    assert!(!repl.is_settled(), "not settled while a fact is held");

    // Phase 2: R arrives → β folds → the node settles.
    bus.inject(vec![frame(&r)]);
    pump_until_quiet(&session, &mut bus, &mut repl);
    assert!(has_member(&session, &group, &x_principal), "β folded once R arrived: X present");
    assert_eq!(repl.pending_len(), 0, "nothing held after the antecedent arrived");
    assert!(repl.is_settled(), "settled once caught up");

    eprintln!(
        "PENDING-SIGNAL RESULT (corroboration): while its antecedent R was withheld, \
         node B held β and reported pending_len>0 (not settled) — it can show it is \
         catching up rather than present a stale prefix as current; once R arrived, β \
         folded and the node settled (pending_len==0)."
    );
}
