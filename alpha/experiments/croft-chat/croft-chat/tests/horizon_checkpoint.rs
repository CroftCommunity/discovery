//! EXP-H2 (RUN-12 Part 6) — the horizon checkpoint as a foldable **fact**.
//!
//! Earns/bounds: Part 2 §7.6.9 / §7.3.3 — a horizon checkpoint recorded and folded as a
//! co-signable **fact**. EXP-H1 (RUN-07) proved the manifest computes identically; this
//! lands the fact form. Experiment-grade, test-only serialization, no wire pinning (the
//! manifest encoding stays `[gates-release]`; the §7.6.9 worked example stays `Design`).
//!
//! §7.3.3 semantics unchanged: a co-signature is corroboration of an independent
//! identical fold, never a substitute. Each member records only the manifest its OWN
//! fold produced, so a member whose fold does not match records nothing toward another's
//! manifest — no false corroboration.
//!
//! Mirrors the `horizon_manifest.rs` harness (two members, both arrival orders, folded
//! through the real `DerivedFold`), lifted to the fact layer. Assertions:
//!   1. the corroboration count for a `(frontier, manifest)` pair folds deterministically
//!      and identically across members and arrival orders;
//!   2. a member whose fold does NOT match records nothing (no false corroboration);
//!   3. an open contradiction persists in the manifest across successive checkpoints
//!      (the H1 decay-is-presentation assertion, now at the fact layer).

mod common;

use std::sync::Arc;

use common::{
    approval_payload, base, genesis_payload, membership_add_payload, rule_change_payload,
    rule_change_subject as rc_subject, sign,
};
use local_storage_projection::fold_derived::{DerivedFold, GroupState};
use local_storage_projection::horizon::HorizonCadence;
use local_storage_projection::horizon::HorizonEvent;
use local_storage_projection::horizon_checkpoint::{
    corroboration_count, manifest_bytes, CheckpointFact,
};
use local_storage_projection::tables::Db;
use local_storage_projection::types::envelope_hash;
use local_storage_projection::{AssertionEnvelope, AssertionType, DeviceId, GroupId, PrincipalId};
use social_graph_core::{Ed25519Verifier, Identity, RegistryCredentialResolver};

fn remove_payload(subject: PrincipalId) -> Vec<u8> {
    subject.as_bytes().to_vec()
}

/// Fold `order` into a fresh store through the real `DerivedFold`, then read back the
/// derived `GroupState`. The `horizon_manifest.rs` helper, reused for the fact layer.
fn fold_and_read(
    path: &std::path::Path,
    authors: &[&Identity],
    group: &GroupId,
    order: &[&AssertionEnvelope],
) -> GroupState {
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

/// The competing quorum-met RuleChange contradiction, built as in `horizon_manifest.rs`.
/// Returns the ordered setup plus the two concurrent changes so a caller can fold either
/// arrival order, and the members' identities.
struct Contradiction {
    authors: Vec<Identity>,
    setup: Vec<AssertionEnvelope>,
    change5: AssertionEnvelope,
    change9: AssertionEnvelope,
    group: GroupId,
}

fn competing_rulechange(group_seed: u8) -> Contradiction {
    let group = GroupId::new([group_seed; 32]);
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

    Contradiction {
        authors: vec![id_o1, id_o2, id_a, id_b],
        setup: vec![genesis, add_o2, add_a, add_b, raise, appr_a, appr_b],
        change5,
        change9,
        group,
    }
}

/// (1) The corroboration count folds deterministically and identically across members and
/// arrival orders.
#[tokio::test]
async fn corroboration_count_is_deterministic_across_members_and_orders() {
    let dir = tempfile::tempdir().expect("tempdir");
    let c = competing_rulechange(0xC0);
    let authors: Vec<&Identity> = c.authors.iter().collect();
    let setup: Vec<&AssertionEnvelope> = c.setup.iter().collect();

    // Member 1 folds change5 first; member 2 folds change9 first (opposite arrival orders).
    let order1: Vec<&AssertionEnvelope> = setup.iter().copied().chain([&c.change5, &c.change9]).collect();
    let order2: Vec<&AssertionEnvelope> = setup.iter().copied().chain([&c.change9, &c.change5]).collect();
    let s1 = fold_and_read(&dir.path().join("m1.redb"), &authors, &c.group, &order1);
    let s2 = fold_and_read(&dir.path().join("m2.redb"), &authors, &c.group, &order2);

    // Each member records a checkpoint fact from its OWN fold (never copies a peer's).
    let m1_principal = c.authors[2].principal_id().0; // a
    let m2_principal = c.authors[3].principal_id().0; // b
    let cp1 = CheckpointFact::record(m1_principal, &s1);
    let cp2 = CheckpointFact::record(m2_principal, &s2);

    // The two independent folds produced the byte-identical manifest (EXP-H1), so the
    // co-signing fact names the same digests.
    assert_eq!(
        manifest_bytes(&cp1.manifest), manifest_bytes(&cp2.manifest),
        "two members' independent folds (opposite orders) produced the identical manifest"
    );

    // Corroboration count is 2, and it is order-independent over the facts themselves.
    let target = cp1.manifest.clone();
    assert_eq!(corroboration_count(&[cp1.clone(), cp2.clone()], &target), 2, "two distinct members corroborate");
    assert_eq!(
        corroboration_count(&[cp2.clone(), cp1.clone()], &target), 2,
        "the corroboration fold is order-independent over the facts"
    );

    // A member's several clients collapse to one lineage: the same signer twice is one.
    let cp1_again = CheckpointFact::record(m1_principal, &s1);
    assert_eq!(
        corroboration_count(&[cp1.clone(), cp1_again, cp2.clone()], &target), 2,
        "a signer recorded twice counts once (distinct-lineage union, §7.3.4)"
    );

    eprintln!("EXP-H2 (1): corroboration count = 2 for the (frontier, manifest) pair, identical across members and arrival orders.");
}

/// (2) A member whose fold does NOT match records nothing toward the target manifest — no
/// false corroboration.
#[tokio::test]
async fn a_non_matching_fold_does_not_falsely_corroborate() {
    let dir = tempfile::tempdir().expect("tempdir");
    let c = competing_rulechange(0xC1);
    let authors: Vec<&Identity> = c.authors.iter().collect();
    let setup: Vec<&AssertionEnvelope> = c.setup.iter().collect();

    // Members 1 and 2 fold the full contradiction (both changes) → the contradiction manifest M.
    let order1: Vec<&AssertionEnvelope> = setup.iter().copied().chain([&c.change5, &c.change9]).collect();
    let order2: Vec<&AssertionEnvelope> = setup.iter().copied().chain([&c.change9, &c.change5]).collect();
    let s1 = fold_and_read(&dir.path().join("m1.redb"), &authors, &c.group, &order1);
    let s2 = fold_and_read(&dir.path().join("m2.redb"), &authors, &c.group, &order2);

    // Member 3 folds only ONE of the competing changes → no contradiction, a different
    // frontier: its own fold produced a DIFFERENT manifest.
    let order3: Vec<&AssertionEnvelope> = setup.iter().copied().chain([&c.change5]).collect();
    let s3 = fold_and_read(&dir.path().join("m3.redb"), &authors, &c.group, &order3);

    let cp1 = CheckpointFact::record(c.authors[2].principal_id().0, &s1);
    let cp2 = CheckpointFact::record(c.authors[3].principal_id().0, &s2);
    let cp3 = CheckpointFact::record(c.authors[0].principal_id().0, &s3);

    let m = cp1.manifest.clone();
    assert_ne!(
        manifest_bytes(&cp3.manifest), manifest_bytes(&m),
        "member 3's fold produced a different manifest (it saw no contradiction)"
    );

    // M3 does not corroborate M: the target's count stays 2 even with cp3 present.
    let all = [cp1.clone(), cp2.clone(), cp3.clone()];
    assert_eq!(
        corroboration_count(&all, &m), 2,
        "the non-matching member contributes nothing to M — no false corroboration"
    );
    // Member 3 truthfully corroborates only its OWN manifest.
    assert_eq!(corroboration_count(&all, &cp3.manifest), 1, "member 3 corroborates only what it folded");

    eprintln!("EXP-H2 (2): a member whose fold did not match added nothing to the target's corroboration (stayed 2).");
}

/// (3) An open contradiction persists in the manifest across successive checkpoints — the
/// H1 decay-is-presentation assertion, now at the fact layer.
#[tokio::test]
async fn open_contradiction_persists_across_successive_checkpoints() {
    let dir = tempfile::tempdir().expect("tempdir");

    // Mutual expulsion → one open contradiction.
    let group = GroupId::new([0xC2; 32]);
    let id_o = Identity::from_seed([0x30; 32]);
    let id_a = Identity::from_seed([0x31; 32]);
    let id_b = Identity::from_seed([0x32; 32]);
    let a_principal = PrincipalId::new(id_a.principal_id().0);
    let b_principal = PrincipalId::new(id_b.principal_id().0);
    let o_device = DeviceId::new(id_o.device_id().0);
    let genesis = sign(&id_o, base(&id_o, group, AssertionType::GroupGenesis, 1, vec![], genesis_payload(&o_device)));
    let add_a = sign(&id_o, base(&id_o, group, AssertionType::MembershipAdd, 2, vec![], membership_add_payload(a_principal, 1)));
    let add_b = sign(&id_o, base(&id_o, group, AssertionType::MembershipAdd, 3, vec![], membership_add_payload(b_principal, 1)));
    let a_rm_b = sign(&id_a, base(&id_a, group, AssertionType::MembershipRemove, 1, vec![envelope_hash(&add_b)], remove_payload(b_principal)));
    let b_rm_a = sign(&id_b, base(&id_b, group, AssertionType::MembershipRemove, 1, vec![envelope_hash(&add_a)], remove_payload(a_principal)));
    let authors = [&id_o, &id_a, &id_b];
    let order: Vec<&AssertionEnvelope> = vec![&genesis, &add_a, &add_b, &a_rm_b, &b_rm_a];
    let state = fold_and_read(&dir.path().join("c.redb"), &authors, &group, &order);

    let signer_a = id_a.principal_id().0;
    let signer_b = id_b.principal_id().0;

    // Checkpoint at the first cadence boundary.
    let cp_boundary1 = CheckpointFact::record(signer_a, &state);
    assert_eq!(cp_boundary1.manifest.open_contradictions.len(), 1, "the contradiction is open at boundary 1");
    let descriptor = cp_boundary1.manifest.open_contradictions[0];

    // Roll a horizon boundary (the cadence firing). The folded state is unchanged — the
    // contradiction never ages out (decay is presentation, not truth).
    let mut cadence = HorizonCadence::new(2);
    let mut boundaries = 0;
    for _ in 0..4 {
        if cadence.observe(HorizonEvent::Fact) {
            boundaries += 1;
        }
    }
    assert!(boundaries >= 1, "at least one horizon boundary rolled");

    // Checkpoint at the next boundary — same open contradiction, byte-identical manifest.
    let cp_boundary2 = CheckpointFact::record(signer_a, &state);
    assert_eq!(
        manifest_bytes(&cp_boundary1.manifest), manifest_bytes(&cp_boundary2.manifest),
        "the open contradiction persists byte-identically across successive checkpoints"
    );
    assert!(
        cp_boundary2.manifest.open_contradictions.contains(&descriptor),
        "the open-contradiction descriptor is still present at boundary 2"
    );

    // A co-signer folding the identical state corroborates the SAME open-contradiction pair
    // at each boundary — corroboration of the persisting contradiction, not of its decay.
    let cosign2 = CheckpointFact::record(signer_b, &state);
    let facts = [cp_boundary2.clone(), cosign2];
    assert_eq!(
        corroboration_count(&facts, &cp_boundary2.manifest), 2,
        "two members corroborate the still-open contradiction at the later checkpoint"
    );

    eprintln!("EXP-H2 (3): the open contradiction persisted byte-identically across successive checkpoints and stayed corroborated — decay is presentation, at the fact layer.");
}
