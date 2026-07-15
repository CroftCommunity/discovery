//! EXP-H1 (backlog §2b, RUN-07) — horizon-manifest determinism, cross-package.
//!
//! Mirrors the competing-quorums harness shape (two members, both arrival orders,
//! folded through the real `DerivedFold`) and asserts that the pure fold-side
//! `horizon_manifest` is **byte-identical** across members and across arrival orders
//! for the two ways a contradiction can now arise: mutual expulsion and a competing
//! quorum-met RuleChange. Also pins the two cadence terms of the horizon trigger and
//! the negative properties (a resolved/absent contradiction is absent; an open one
//! never ages out).
//!
//! This earns the *manifest determinism* claim ONLY. There is no wire format here:
//! `manifest_bytes` below is a TEST-ONLY serialization for the byte-identity
//! assertion, explicitly NOT the `[gates-release]` horizon-manifest encoding
//! (Part 2 §7.6.9, which stays `Design`).

mod common;

use std::sync::Arc;

use common::{
    approval_payload, base, genesis_payload, membership_add_payload, rule_change_payload,
    rule_change_subject as rc_subject, sign,
};
use local_storage_projection::fold_derived::DerivedFold;
use local_storage_projection::horizon::{
    horizon_manifest, HorizonCadence, HorizonEvent, HorizonManifest,
};
use local_storage_projection::tables::Db;
use local_storage_projection::types::envelope_hash;
use local_storage_projection::{
    AssertionEnvelope, AssertionType, DeviceId, GroupId, PrincipalId,
};
use social_graph_core::{Ed25519Verifier, Identity, RegistryCredentialResolver};

fn remove_payload(subject: PrincipalId) -> Vec<u8> {
    subject.as_bytes().to_vec()
}

/// Fold `order` into a fresh store through the real `DerivedFold`, then read back the
/// derived `GroupState` and return it. The direct-fold counterpart used by the
/// competing-quorums harness, extended to expose the folded state for the manifest.
fn fold_and_read(
    path: &std::path::Path,
    authors: &[&Identity],
    group: &GroupId,
    order: &[&AssertionEnvelope],
) -> local_storage_projection::fold_derived::GroupState {
    let db = Arc::new(Db::open(path).expect("open db"));
    let resolver = RegistryCredentialResolver::new();
    for a in authors {
        resolver.register(a.device_id(), a.principal_id());
    }
    let fold = DerivedFold::new(Arc::clone(&db), Ed25519Verifier, resolver);
    for env in order {
        let _ = fold.ingest(env);
    }
    fold.read_group_state(group)
        .expect("read state")
        .expect("group state present after folding")
}

/// TEST-ONLY serialization used purely to compare two manifests byte-for-byte. This is
/// NOT the `[gates-release]` horizon-manifest wire encoding (Part 2 §7.6.9); it exists
/// only to turn the manifest into a comparable byte string inside this test.
fn manifest_bytes(m: &HorizonManifest) -> Vec<u8> {
    let mut b = Vec::new();
    b.extend_from_slice(m.frontier_head.as_bytes());
    b.extend_from_slice(&(m.open_contradictions.len() as u32).to_be_bytes());
    for h in &m.open_contradictions {
        b.extend_from_slice(h.as_bytes());
    }
    b
}

// ---------------------------------------------------------------------------
// Determinism — mutual expulsion (the §2.5 canonical residue)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn horizon_manifest_identical_across_orders_mutual_expulsion() {
    let dir = tempfile::tempdir().expect("tempdir");
    let group = GroupId::new([0xE0; 32]);

    let id_o = Identity::from_seed([0x10; 32]);
    let id_a = Identity::from_seed([0x11; 32]);
    let id_b = Identity::from_seed([0x12; 32]);
    let a_principal = PrincipalId::new(id_a.principal_id().0);
    let b_principal = PrincipalId::new(id_b.principal_id().0);
    let o_device = DeviceId::new(id_o.device_id().0);

    let genesis = sign(&id_o, base(&id_o, group, AssertionType::GroupGenesis, 1, vec![], genesis_payload(&o_device)));
    let add_a = sign(&id_o, base(&id_o, group, AssertionType::MembershipAdd, 2, vec![], membership_add_payload(a_principal, 1)));
    let add_b = sign(&id_o, base(&id_o, group, AssertionType::MembershipAdd, 3, vec![], membership_add_payload(b_principal, 1)));
    // Mutual expulsion: A removes B, B removes A, concurrently.
    let a_removes_b = sign(&id_a, base(&id_a, group, AssertionType::MembershipRemove, 1, vec![envelope_hash(&add_b)], remove_payload(b_principal)));
    let b_removes_a = sign(&id_b, base(&id_b, group, AssertionType::MembershipRemove, 1, vec![envelope_hash(&add_a)], remove_payload(a_principal)));

    let authors = [&id_o, &id_a, &id_b];
    let setup: Vec<&AssertionEnvelope> = vec![&genesis, &add_a, &add_b];

    // Member 1 folds A-removes-B first; member 2 folds B-removes-A first.
    let order1: Vec<&AssertionEnvelope> = setup.iter().copied().chain([&a_removes_b, &b_removes_a]).collect();
    let order2: Vec<&AssertionEnvelope> = setup.iter().copied().chain([&b_removes_a, &a_removes_b]).collect();
    let s1 = fold_and_read(&dir.path().join("m1.redb"), &authors, &group, &order1);
    let s2 = fold_and_read(&dir.path().join("m2.redb"), &authors, &group, &order2);

    let m1 = horizon_manifest(&s1);
    let m2 = horizon_manifest(&s2);
    eprintln!("H1 mutual-expulsion: order1 open={:?}; order2 open={:?}", m1.open_contradictions, m2.open_contradictions);

    assert_eq!(m1.open_contradictions.len(), 1, "mutual expulsion is one open contradiction");
    assert_eq!(
        manifest_bytes(&m1), manifest_bytes(&m2),
        "horizon manifest must be byte-identical across members and arrival orders (mutual expulsion)"
    );
}

// ---------------------------------------------------------------------------
// Determinism — competing quorum-met RuleChange
// ---------------------------------------------------------------------------

#[tokio::test]
async fn horizon_manifest_identical_across_orders_competing_rulechange() {
    let dir = tempfile::tempdir().expect("tempdir");
    let group = GroupId::new([0xE1; 32]);

    let id_o1 = Identity::from_seed([0x20; 32]);
    let id_o2 = Identity::from_seed([0x21; 32]);
    let id_a = Identity::from_seed([0x22; 32]);
    let id_b = Identity::from_seed([0x23; 32]);
    let o1_device = DeviceId::new(id_o1.device_id().0);
    let o2_principal = PrincipalId::new(id_o2.principal_id().0);
    let a_principal = PrincipalId::new(id_a.principal_id().0);
    let b_principal = PrincipalId::new(id_b.principal_id().0);

    let genesis = sign(&id_o1, base(&id_o1, group, AssertionType::GroupGenesis, 1, vec![], genesis_payload(&o1_device)));
    let add_o2 = sign(&id_o1, base(&id_o1, group, AssertionType::MembershipAdd, 2, vec![], membership_add_payload(o2_principal, 0)));
    let add_a = sign(&id_o1, base(&id_o1, group, AssertionType::MembershipAdd, 3, vec![], membership_add_payload(a_principal, 1)));
    let add_b = sign(&id_o1, base(&id_o1, group, AssertionType::MembershipAdd, 4, vec![], membership_add_payload(b_principal, 1)));
    let raise = sign(&id_o1, base(&id_o1, group, AssertionType::RuleChange, 5, vec![], rule_change_payload(3, 2)));
    let raise_h = envelope_hash(&raise);

    let payload5 = rule_change_payload(0, 5);
    let payload9 = rule_change_payload(0, 9);
    let appr_a = sign(&id_a, base(&id_a, group, AssertionType::Approval, 1, vec![raise_h], approval_payload(AssertionType::RuleChange, rc_subject(&payload5))));
    let appr_b = sign(&id_b, base(&id_b, group, AssertionType::Approval, 1, vec![raise_h], approval_payload(AssertionType::RuleChange, rc_subject(&payload9))));
    let change5 = sign(&id_o1, base(&id_o1, group, AssertionType::RuleChange, 6, vec![raise_h, envelope_hash(&appr_a)], payload5));
    let change9 = sign(&id_o2, base(&id_o2, group, AssertionType::RuleChange, 1, vec![raise_h, envelope_hash(&appr_b)], payload9));

    let authors = [&id_o1, &id_o2, &id_a, &id_b];
    let setup: Vec<&AssertionEnvelope> = vec![&genesis, &add_o2, &add_a, &add_b, &raise, &appr_a, &appr_b];

    let order1: Vec<&AssertionEnvelope> = setup.iter().copied().chain([&change5, &change9]).collect();
    let order2: Vec<&AssertionEnvelope> = setup.iter().copied().chain([&change9, &change5]).collect();
    let s1 = fold_and_read(&dir.path().join("r1.redb"), &authors, &group, &order1);
    let s2 = fold_and_read(&dir.path().join("r2.redb"), &authors, &group, &order2);

    let m1 = horizon_manifest(&s1);
    let m2 = horizon_manifest(&s2);
    eprintln!("H1 competing-rulechange: order1 open={:?}; order2 open={:?}", m1.open_contradictions, m2.open_contradictions);

    assert_eq!(m1.open_contradictions.len(), 1, "a competing quorum-met RuleChange is one open contradiction");
    assert_eq!(
        manifest_bytes(&m1), manifest_bytes(&m2),
        "horizon manifest must be byte-identical across members and arrival orders (competing RuleChange)"
    );
}

// ---------------------------------------------------------------------------
// Trigger — both cadence terms fire at the same fact position on both members
// ---------------------------------------------------------------------------

#[tokio::test]
async fn horizon_trigger_both_modes_fire_at_same_position() {
    // N seeded in the test, standing in for the genesis rule.
    const N: u64 = 3;

    // Fact-count term: two independent cadences fed the identical fact stream fire a
    // boundary at exactly the same positions (every Nth fact).
    let mut m1 = HorizonCadence::new(N);
    let mut m2 = HorizonCadence::new(N);
    let mut b1 = Vec::new();
    let mut b2 = Vec::new();
    for i in 1..=(3 * N) {
        b1.push((i, m1.observe(HorizonEvent::Fact)));
        b2.push((i, m2.observe(HorizonEvent::Fact)));
    }
    assert_eq!(b1, b2, "both members fire the fact-count boundary at identical positions");
    let boundaries: Vec<u64> = b1.iter().filter(|(_, fired)| *fired).map(|(i, _)| *i).collect();
    assert_eq!(boundaries, vec![N, 2 * N, 3 * N], "a boundary fires at every Nth fact, counter reset each time");

    // Epoch-roll term: fires a boundary immediately and resets the fact counter, on
    // both members identically.
    let mut e1 = HorizonCadence::new(N);
    let mut e2 = HorizonCadence::new(N);
    assert!(!e1.observe(HorizonEvent::Fact));
    assert!(!e2.observe(HorizonEvent::Fact));
    assert!(e1.observe(HorizonEvent::EpochRoll), "epoch roll fires a boundary");
    assert!(e2.observe(HorizonEvent::EpochRoll), "epoch roll fires a boundary");
    assert_eq!(e1.facts_since_boundary(), 0, "epoch roll reset the fact counter (member 1)");
    assert_eq!(e2.facts_since_boundary(), 0, "epoch roll reset the fact counter (member 2)");
    // After the roll the fact counter starts fresh: the next boundary is N facts later.
    let mut fired_at = None;
    for i in 1..=N {
        if e1.observe(HorizonEvent::Fact) {
            fired_at = Some(i);
        }
    }
    assert_eq!(fired_at, Some(N), "the post-roll fact-count boundary is N facts after the roll");
}

// ---------------------------------------------------------------------------
// Negative — absent contradiction absent; open contradiction never ages out
// ---------------------------------------------------------------------------

#[tokio::test]
async fn horizon_manifest_negatives() {
    let dir = tempfile::tempdir().expect("tempdir");

    // (a) A clean group (no contradiction) has an empty open-contradiction set.
    let group_clean = GroupId::new([0xE2; 32]);
    let id_o = Identity::from_seed([0x30; 32]);
    let id_a = Identity::from_seed([0x31; 32]);
    let a_principal = PrincipalId::new(id_a.principal_id().0);
    let o_device = DeviceId::new(id_o.device_id().0);
    let genesis = sign(&id_o, base(&id_o, group_clean, AssertionType::GroupGenesis, 1, vec![], genesis_payload(&o_device)));
    let add_a = sign(&id_o, base(&id_o, group_clean, AssertionType::MembershipAdd, 2, vec![], membership_add_payload(a_principal, 1)));
    let authors = [&id_o, &id_a];
    let clean_order: Vec<&AssertionEnvelope> = vec![&genesis, &add_a];
    let s_clean = fold_and_read(&dir.path().join("clean.redb"), &authors, &group_clean, &clean_order);
    let m_clean = horizon_manifest(&s_clean);
    assert!(
        m_clean.open_contradictions.is_empty(),
        "a resolved/absent contradiction must not appear in the manifest"
    );

    // (b) An OPEN contradiction persists across horizon boundaries — decay is a
    // presentation concern, not a truth concern. The manifest is a pure function of the
    // folded state, so rolling any number of horizons (the cadence firing) does not age
    // the descriptor out: the byte-identical descriptor is still present.
    let group_c = GroupId::new([0xE3; 32]);
    let id_b = Identity::from_seed([0x32; 32]);
    let b_principal = PrincipalId::new(id_b.principal_id().0);
    let genesis_c = sign(&id_o, base(&id_o, group_c, AssertionType::GroupGenesis, 1, vec![], genesis_payload(&o_device)));
    let add_a_c = sign(&id_o, base(&id_o, group_c, AssertionType::MembershipAdd, 2, vec![], membership_add_payload(a_principal, 1)));
    let add_b_c = sign(&id_o, base(&id_o, group_c, AssertionType::MembershipAdd, 3, vec![], membership_add_payload(b_principal, 1)));
    let a_rm_b = sign(&id_a, base(&id_a, group_c, AssertionType::MembershipRemove, 1, vec![envelope_hash(&add_b_c)], remove_payload(b_principal)));
    let b_rm_a = sign(&id_b, base(&id_b, group_c, AssertionType::MembershipRemove, 1, vec![envelope_hash(&add_a_c)], remove_payload(a_principal)));
    let authors_c = [&id_o, &id_a, &id_b];
    let order_c: Vec<&AssertionEnvelope> = vec![&genesis_c, &add_a_c, &add_b_c, &a_rm_b, &b_rm_a];
    let s_c = fold_and_read(&dir.path().join("c.redb"), &authors_c, &group_c, &order_c);

    let m_before = horizon_manifest(&s_c);
    assert_eq!(m_before.open_contradictions.len(), 1, "the contradiction is open");
    let descriptor = m_before.open_contradictions[0];

    // Roll several horizon boundaries; the folded state (hence the manifest) is unchanged.
    let mut cadence = HorizonCadence::new(2);
    let mut rolls = 0;
    for _ in 0..10 {
        if cadence.observe(HorizonEvent::Fact) {
            rolls += 1;
        }
    }
    assert!(rolls >= 1, "at least one horizon boundary rolled");
    let m_after = horizon_manifest(&s_c);
    assert_eq!(
        manifest_bytes(&m_before), manifest_bytes(&m_after),
        "an open contradiction never ages out of the manifest across horizon boundaries"
    );
    assert!(
        m_after.open_contradictions.contains(&descriptor),
        "the open-contradiction descriptor persists across horizons"
    );
}
