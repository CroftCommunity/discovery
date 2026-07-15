//! E2.10 — thresholds count lineages, not leaves (TEST-PLAN T2 gate).
//!
//! The one that "bites silently if wrong": a person must not be able to
//! manufacture a social quorum from multiple of their own devices. Standing
//! still counts *signatures by admin DID* for the basic soundness checks
//! (I2/E2.1), but a social threshold must count distinct *lineages* among the
//! valid admin signers. Rests on T1 (the leaf carries a verifiable lineage id).
//!
//! This test demonstrates the gap directly: the by-DID count (the old
//! `meets_threshold`) WRONGLY accepts one person's two devices; the
//! lineage-aware count rejects it.

use std::collections::{BTreeMap, BTreeSet};

use lineage_core::gov::{sign_op, Directory, Genesis, GenesisRules, GroupState, OpKind};
use lineage_core::ids::Did;
use lineage_core::keys::SigningIdentity;

fn did(s: &str) -> Did {
    Did::new(s)
}

/// alice has two devices (phone, laptop), both admins; bob is a separate admin;
/// carol is a plain member to be removed. Remove needs 2.
struct World {
    alice_phone: SigningIdentity,
    alice_laptop: SigningIdentity,
    bob: SigningIdentity,
    dir: Directory,
    genesis: Genesis,
    lineage_of: BTreeMap<Did, Did>,
}

fn world() -> World {
    let alice_phone = SigningIdentity::from_seed(did("alice.phone"), 1);
    let alice_laptop = SigningIdentity::from_seed(did("alice.laptop"), 1);
    let bob = SigningIdentity::from_seed(did("bob"), 1);
    let carol = SigningIdentity::from_seed(did("carol"), 1);

    let admins: BTreeSet<Did> =
        [did("alice.phone"), did("alice.laptop"), did("bob")].into_iter().collect();
    let mut thresholds = BTreeMap::new();
    thresholds.insert(OpKind::Remove, 2);
    let rules = GenesisRules { admins, thresholds };

    let founders: BTreeSet<Did> =
        [did("alice.phone"), did("alice.laptop"), did("bob"), did("carol")]
            .into_iter()
            .collect();
    let genesis = Genesis::new(rules, founders);

    let mut dir = Directory::new();
    for v in [&alice_phone, &alice_laptop, &bob, &carol] {
        dir.insert(v.verifying());
    }

    // The leaf→lineage mapping every client computes from the T1 leaf credential:
    // alice's two devices share lineage "alice"; bob is his own lineage.
    let lineage_of: BTreeMap<Did, Did> = [
        (did("alice.phone"), did("alice")),
        (did("alice.laptop"), did("alice")),
        (did("bob"), did("bob")),
    ]
    .into_iter()
    .collect();

    World { alice_phone, alice_laptop, bob, dir, genesis, lineage_of }
}

#[test]
fn e2_10_two_own_devices_cannot_manufacture_a_quorum() {
    let w = world();
    let state = GroupState::new(w.genesis.clone());

    // A remove signed by ALICE'S TWO DEVICES only (one person).
    let op = sign_op(
        &state,
        OpKind::Remove,
        Some(did("carol")),
        &[&w.alice_phone, &w.alice_laptop],
    );

    // By raw DID count this WRONGLY meets the threshold of 2 (two signatures)...
    assert!(
        state.meets_threshold(&op, &w.dir),
        "by-DID count sees two signatures (this is exactly the unsafe behaviour)"
    );

    // ...but counting LINEAGES, alice's two devices are one lineage → below 2.
    assert!(
        !state.meets_threshold_by_lineage(&op, &w.dir, &w.lineage_of),
        "two devices of one lineage must NOT manufacture a quorum (E2.10 gate)"
    );
}

#[test]
fn e2_10_two_distinct_lineages_do_meet_threshold() {
    let w = world();
    let state = GroupState::new(w.genesis.clone());

    // One device of alice + bob = two distinct lineages.
    let op = sign_op(
        &state,
        OpKind::Remove,
        Some(did("carol")),
        &[&w.alice_phone, &w.bob],
    );

    assert!(
        state.meets_threshold_by_lineage(&op, &w.dir, &w.lineage_of),
        "two distinct lineages legitimately meet the threshold"
    );
}

#[test]
fn e2_10_signer_absent_from_map_counts_as_own_lineage() {
    // A single-device admin not present in the lineage map counts as itself,
    // so existing single-device behaviour is preserved (no regression to E2.1).
    let w = world();
    let state = GroupState::new(w.genesis.clone());
    let empty: BTreeMap<Did, Did> = BTreeMap::new();

    let op = sign_op(
        &state,
        OpKind::Remove,
        Some(did("carol")),
        &[&w.alice_phone, &w.bob],
    );
    assert!(
        state.meets_threshold_by_lineage(&op, &w.dir, &empty),
        "with no lineage map, each admin DID is its own lineage (legacy behaviour)"
    );
}
