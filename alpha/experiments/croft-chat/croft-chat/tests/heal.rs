//! G2 — a held-back fact heals monotonically when its predecessor arrives
//! (Battery 5, Rung B).
//!
//! Earns/bounds: Part 2 §7.3.2 — a held-back fact heals monotonically when its referenced predecessor arrives.
//!
//! The companion to G1's guard. G1 shows a fact whose cross-device antecedent is
//! missing is held back (not admitted); G2 shows that when the missing predecessor
//! finally arrives, the held fact is admitted and the node advances to the complete
//! head, never reverting anything already accepted — the monotonic-fold posture
//! (Part 1 §2.2) that structurally rules out the Matrix state-reset class
//! (CVE-2025-49090). A lagging node under-authorizes; it never mis-authorizes.
//!
//! Same cross-device scenario as G1 (owner O: genesis, α add A2 Admin, R RuleChange
//! 1→7; admin A2: β add X, antecedents=[hash(R)]). Node B is driven in two phases:
//!   phase 1 — receives {genesis, α, β} but not R: β is HELD (its antecedent R is
//!             absent), so X is absent and the threshold is at the pre-R value 1;
//!   phase 2 — R arrives late: β's antecedent is now present, β is admitted, X
//!             appears and the threshold advances to 7 — a strict prefix catching up.

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
async fn late_heal_is_monotonic() {
    let dir = tempfile::tempdir().expect("tempdir");
    let group = GroupId::new([0x88; 32]);

    let id_o = Identity::from_seed([0x20; 32]);
    let id_a2 = Identity::from_seed([0x21; 32]);
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

    // Node B, with the pipeline held open across two delivery phases.
    let session = Session::open(&dir.path().join("b.redb"), &id_o).expect("open B");
    session.trust_peer(id_o.device_id(), id_o.principal_id());
    session.trust_peer(id_a2.device_id(), id_a2.principal_id());
    let mut bus = QueueBus::default();
    bus.subscribe(&Topic("drystone/completeness".to_string()));
    let mut repl = Replicator::new();

    // Phase 1: R withheld. β is held back (its antecedent R is absent).
    bus.inject(vec![frame(&beta), frame(&genesis), frame(&alpha)]);
    pump_until_quiet(&session, &mut bus, &mut repl);
    let sum1 = session.get_group_summary(&group).expect("summary phase 1");
    assert_eq!(sum1.rules.remove_member_threshold, 1, "phase 1: behind (R withheld)");
    assert!(
        !has_member(&session, &group, &x_principal),
        "phase 1: X absent — β is held pending its missing antecedent R (the G1 guard)"
    );
    assert!(has_member(&session, &group, &a2_principal), "phase 1: α applied (no gap): A2 present");
    assert_eq!(sum1.fork_status, "clean", "phase 1: a clean prefix, not a divergence");

    // Phase 2: R arrives late — β's antecedent is now present.
    bus.inject(vec![frame(&r)]);
    pump_until_quiet(&session, &mut bus, &mut repl);
    let sum2 = session.get_group_summary(&group).expect("summary phase 2");

    // HEAL: R applied AND the previously-held β now admitted, both in one catch-up.
    assert_eq!(sum2.rules.remove_member_threshold, 7, "phase 2: healed to the complete value");
    assert!(
        has_member(&session, &group, &x_principal),
        "phase 2: X now present — the held β was admitted once its antecedent arrived"
    );
    assert!(has_member(&session, &group, &a2_principal), "phase 2: A2 still present (no reversion)");
    assert_eq!(sum2.fork_status, "clean", "phase 2: still clean");

    eprintln!(
        "G2 RESULT (corroboration): with R withheld, B holds β back (X absent, threshold \
         1 — a clean prefix); when R arrives late, both R and the held β are admitted, B \
         advances to threshold 7 with X present, and nothing already accepted reverts. \
         Held facts heal monotonically once their predecessor lands."
    );
}
