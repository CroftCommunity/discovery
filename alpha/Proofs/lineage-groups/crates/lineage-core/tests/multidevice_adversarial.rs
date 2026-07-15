//! Tier-A batch: governance-level multi-device + adversarial cases that build on
//! the existing `gov`/`conflict` machinery (no new op-kinds needed).
//!
//! - E2.15  self-removal ordering: a leaf authors its own removal while it has
//!          standing; afterward it cannot author.
//! - AR-1   Sybil / fresh-lineage: minting fresh identities never reaches a
//!          threshold without being an authorized admin.
//! - AR-6   replay / double-count: a DID cannot be counted twice; a replayed op
//!          does not re-enact.
//! - C3     concurrent identical remove heals (no false hard-stop).

use std::collections::{BTreeMap, BTreeSet};

use lineage_core::conflict::{detect, Resolution};
use lineage_core::gov::{sign_op, Directory, Genesis, GenesisRules, GroupState, OpKind, RejectReason};
use lineage_core::ids::Did;
use lineage_core::keys::SigningIdentity;

fn did(s: &str) -> Did {
    Did::new(s)
}

/// admins {alice, bob}; founders {alice, bob, carol}; Remove needs 2, others 1.
fn world() -> (SigningIdentity, SigningIdentity, SigningIdentity, Directory, Genesis) {
    let alice = SigningIdentity::from_seed(did("alice"), 1);
    let bob = SigningIdentity::from_seed(did("bob"), 1);
    let carol = SigningIdentity::from_seed(did("carol"), 1);

    let admins: BTreeSet<Did> = [did("alice"), did("bob")].into_iter().collect();
    let mut thresholds = BTreeMap::new();
    thresholds.insert(OpKind::Remove, 2);
    let rules = GenesisRules { admins, thresholds };
    let founders: BTreeSet<Did> =
        [did("alice"), did("bob"), did("carol")].into_iter().collect();
    let genesis = Genesis::new(rules, founders);

    let mut dir = Directory::new();
    for v in [&alice, &bob, &carol] {
        dir.insert(v.verifying());
    }
    (alice, bob, carol, dir, genesis)
}

#[test]
fn e2_15_leaf_can_author_own_removal_then_loses_standing() {
    let (alice, _bob, _carol, dir, genesis) = world();
    let mut state = GroupState::new(genesis);

    // Alice leaves while she still has standing (Leave defaults to threshold 1).
    let leave = sign_op(&state, OpKind::Leave, Some(did("alice")), &[&alice]);
    state.apply(leave, &dir).expect("a leaf may author its own departure while a member");
    assert!(!state.members.contains(&did("alice")), "alice is out after her own leave");

    // After departing, Alice can no longer author governance (DepartedAdmin).
    let after = sign_op(&state, OpKind::Add, Some(did("dave")), &[&alice]);
    assert_eq!(
        state.apply(after, &dir),
        Err(RejectReason::DepartedAdmin(did("alice"))),
        "a departed leaf cannot author an op after dropping its standing"
    );
}

#[test]
fn ar_1_sybil_fresh_identities_never_reach_threshold() {
    let (_alice, _bob, _carol, mut dir, genesis) = world();
    let state = GroupState::new(genesis);

    // Mint a crowd of fresh identities (a Sybil) and have them sign a remove.
    let sybils: Vec<SigningIdentity> =
        (0..10).map(|i| SigningIdentity::from_seed(did(&format!("sybil{i}")), 1)).collect();
    for s in &sybils {
        dir.insert(s.verifying());
    }
    let signers: Vec<&SigningIdentity> = sybils.iter().collect();
    let op = sign_op(&state, OpKind::Remove, Some(did("carol")), &signers);

    // Not in the immutable admin set → rejected outright, never counted.
    assert!(
        matches!(state.validate(&op, &dir), Err(RejectReason::SignerLacksStanding(_))),
        "fresh (non-admin) identities have no standing — a Sybil cannot vote"
    );
    // And by-lineage counting confirms zero authority regardless of headcount.
    let lineage_of: BTreeMap<Did, Did> =
        sybils.iter().map(|s| (s.did().clone(), did("sybil-lineage"))).collect();
    assert_eq!(
        state.valid_admin_lineages(&op, &dir, &lineage_of),
        0,
        "no admit-authority accrues to fresh lineages"
    );
}

#[test]
fn ar_6_double_count_is_structurally_prevented() {
    let (alice, _bob, _carol, dir, genesis) = world();
    let state = GroupState::new(genesis);

    // Try to count alice twice toward the Remove threshold of 2.
    let op = sign_op(&state, OpKind::Remove, Some(did("carol")), &[&alice, &alice]);
    assert_eq!(op.sigs.len(), 1, "sigs are keyed by DID — one signer cannot appear twice");
    assert!(
        !state.meets_threshold(&op, &dir),
        "alice signing twice still counts once → below threshold 2"
    );
}

#[test]
fn ar_6_replayed_op_does_not_reenact() {
    let (alice, bob, _carol, dir, genesis) = world();
    let mut state = GroupState::new(genesis);

    let remove = sign_op(&state, OpKind::Remove, Some(did("carol")), &[&alice, &bob]);
    state.apply(remove.clone(), &dir).expect("two admins meet the Remove threshold");
    assert!(!state.members.contains(&did("carol")));

    // Replaying the same op (seq 0, prev = genesis) against the advanced head fails.
    assert_eq!(
        state.apply(remove, &dir),
        Err(RejectReason::BrokenChain),
        "a replayed op does not chain onto the new head → not re-enacted"
    );
}

#[test]
fn c3_concurrent_identical_remove_heals() {
    // Two partitioned branches both boot carol (agreement, not contradiction).
    let (alice, bob, _carol, dir, genesis) = world();
    let mut left = GroupState::new(genesis.clone());
    let mut right = GroupState::new(genesis);

    let l = sign_op(&left, OpKind::Remove, Some(did("carol")), &[&alice, &bob]);
    left.apply(l, &dir).unwrap();
    let r = sign_op(&right, OpKind::Remove, Some(did("carol")), &[&alice, &bob]);
    right.apply(r, &dir).unwrap();

    assert_eq!(
        detect(&left, &right),
        Resolution::Heal,
        "both sides agreeing to remove the same member is a heal, not a hard-stop"
    );
}
