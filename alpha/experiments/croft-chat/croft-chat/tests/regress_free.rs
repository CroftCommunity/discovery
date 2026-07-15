//! V3′ — regress-free: an unadmitted grant authorizes nothing (Battery 6, Rung B).
//!
//! Earns/bounds: Part 2 §7.3 — authority is a fold over admitted state: an unadmitted grant authorizes nothing.
//!
//! The authority-side complement to G1, and the line that makes G1 precise.
//! Authority is checked against the partial state the fold has already built, so
//! a fact whose authorizing grant is not (yet) admitted authorizes nothing.
//!
//! Two shapes:
//!   (a) present-but-insufficient — a plain Member authors a MembershipAdd. On a
//!       node with the COMPLETE set, it is still rejected: role is checked against
//!       accumulated state, not assumed.
//!   (b) missing grant — the grant that would make an author an Admin is dropped.
//!       The dependent fact is rejected on the node missing the grant, leaving that
//!       node with a strict-subset (stale-but-honest, P-Local-Truth) view.
//!
//! This is one of two complementary completeness mechanisms. The *authority* check
//! here (role evaluated against accumulated state) catches a missing
//! authority-conferring fact — the dependent action is rejected outright. The
//! *antecedent* guard added for G1 catches a missing ordinary-state fact — the
//! dependent fact is held back. Together they mean a node missing anything a later
//! fact depends on never folds it in: authority gaps reject, state gaps hold, and
//! neither produces the silent divergence G1 originally exposed.

mod common;

use common::{base, drive, frame, genesis_payload, has_member, membership_add_payload, sign};
use local_storage_projection::types::envelope_hash;
use local_storage_projection::{AssertionType, DeviceId, GroupId, PrincipalId};
use social_graph_core::Identity;

#[tokio::test]
async fn unadmitted_grant_authorizes_nothing() {
    let dir = tempfile::tempdir().expect("tempdir");
    let group = GroupId::new([0x55; 32]);

    let id_o = Identity::from_seed([0x10; 32]); // owner
    let id_a2 = Identity::from_seed([0x11; 32]); // granted Admin
    let id_m = Identity::from_seed([0x12; 32]); // plain Member
    let a2_principal = PrincipalId::new(id_a2.principal_id().0);
    let m_principal = PrincipalId::new(id_m.principal_id().0);
    let x_principal = PrincipalId::new([0x33; 32]);
    let y_principal = PrincipalId::new([0x44; 32]);
    let o_device = DeviceId::new(id_o.device_id().0);

    let genesis = sign(
        &id_o,
        base(&id_o, group, AssertionType::GroupGenesis, 1, vec![], genesis_payload(&o_device)),
    );
    // α: O grants A2 Admin. γ: O adds M as a plain Member.
    let alpha = sign(
        &id_o,
        base(&id_o, group, AssertionType::MembershipAdd, 2, vec![], membership_add_payload(a2_principal, 1)),
    );
    let gamma = sign(
        &id_o,
        base(&id_o, group, AssertionType::MembershipAdd, 3, vec![], membership_add_payload(m_principal, 2)),
    );
    // β: Admin A2 adds X (should succeed). δ: Member M adds Y (should be rejected).
    let beta = sign(
        &id_a2,
        base(&id_a2, group, AssertionType::MembershipAdd, 1, vec![envelope_hash(&alpha)], membership_add_payload(x_principal, 2)),
    );
    let delta = sign(
        &id_m,
        base(&id_m, group, AssertionType::MembershipAdd, 1, vec![envelope_hash(&gamma)], membership_add_payload(y_principal, 2)),
    );

    let authors = [&id_o, &id_a2, &id_m];

    // Shape (a): the COMPLETE set. Everything delivered; δ still rejected because
    // M is only a Member — authority is checked, not assumed.
    let sess_a = drive(
        &dir.path().join("a.redb"),
        &id_o,
        &authors,
        vec![
            frame(&genesis),
            frame(&alpha),
            frame(&gamma),
            frame(&beta),
            frame(&delta),
        ],
    );
    assert!(has_member(&sess_a, &group, &x_principal), "Admin A2's add of X is admitted");
    assert!(
        !has_member(&sess_a, &group, &y_principal),
        "Member M's add of Y is rejected even with the complete set — regress-free: \
         a role that was never granted authorizes nothing"
    );

    // Shape (b): drop the grant α. A2 is not an Admin on B, so β (add X) is rejected.
    // B ends a strict subset of A — stale but honest, not a confident divergence.
    let sess_b = drive(
        &dir.path().join("b.redb"),
        &id_o,
        &authors,
        vec![frame(&beta), frame(&genesis)], // α dropped
    );
    assert!(
        !has_member(&sess_b, &group, &x_principal),
        "with the grant α dropped, A2's add of X is rejected — the missing fact was \
         authority-conferring, so unlike G1 the dependent fact does NOT slip through"
    );
    let sum_b = sess_b.get_group_summary(&group).expect("summary B");
    assert_eq!(sum_b.fork_status, "clean", "B under-authorizes (stale-but-honest), no false fork");

    eprintln!(
        "V3′ RESULT (corroboration): an unadmitted grant authorizes nothing — a plain \
         Member's add is rejected against the complete set, and a dropped grant makes \
         the dependent add fail rather than diverge. This is the authority completeness \
         mechanism (role check); the G1 antecedent guard is its state-side complement."
    );
}
