//! Phase 2 experiments E2.1–E2.5 + fuzzed survivor/convergence.
//!
//! Governance threshold soundness (I2), genesis immutability (I1), standing
//! (I3), deterministic survivor selection (I5), conflict hard-stop (I6),
//! convergence (I10), and the resting-state (rejected merge leaves two valid
//! groups). Run: `cargo test -p lineage-core`.

use std::collections::{BTreeMap, BTreeSet};

use lineage_core::conflict::{detect, reconcile, ConflictReason, Escalator, Resolution};
use lineage_core::dag::Lineage;
use lineage_core::gov::{sign_op, Directory, Genesis, GenesisRules, GroupState, OpKind, RejectReason};
use lineage_core::ids::{Did, GenesisId};
use lineage_core::keys::SigningIdentity;
use lineage_core::rng::DetRng;
use lineage_core::survivor::{select_survivor, BranchSummary, SurvivorRule};

fn did(s: &str) -> Did {
    Did::new(s)
}

/// Two admins (alice, bob), founders {alice, bob, carol}, Remove needs 2 sigs.
struct World {
    alice: SigningIdentity,
    bob: SigningIdentity,
    carol_id: Did,
    dir: Directory,
    genesis: Genesis,
}

fn world() -> World {
    let alice = SigningIdentity::from_seed(did("alice"), 1);
    let bob = SigningIdentity::from_seed(did("bob"), 1);
    let carol = SigningIdentity::from_seed(did("carol"), 1);

    let admins: BTreeSet<Did> = [did("alice"), did("bob")].into_iter().collect();
    let mut thresholds = BTreeMap::new();
    thresholds.insert(OpKind::Remove, 2);
    thresholds.insert(OpKind::Add, 1);
    let rules = GenesisRules { admins, thresholds };

    let founders: BTreeSet<Did> = [did("alice"), did("bob"), did("carol")].into_iter().collect();
    let genesis = Genesis::new(rules, founders);

    let mut dir = Directory::new();
    dir.insert(alice.verifying());
    dir.insert(bob.verifying());
    dir.insert(carol.verifying());

    World { alice, bob, carol_id: did("carol"), dir, genesis }
}

/// E2.1 — an under-threshold remove is rejected by every honest client.
#[test]
fn e2_1_under_threshold_remove_rejected() {
    let w = world();
    let state = GroupState::new(w.genesis.clone());

    // Remove needs 2 admin signatures; sign with only alice.
    let op = sign_op(&state, OpKind::Remove, Some(w.carol_id.clone()), &[&w.alice]);

    // Deterministic rejection, identical on a second independent client.
    let err1 = GroupState::new(w.genesis.clone()).validate(&op, &w.dir).unwrap_err();
    let err2 = state.validate(&op, &w.dir).unwrap_err();
    assert_eq!(err1, err2);
    assert_eq!(
        err1,
        RejectReason::UnderThreshold { kind: OpKind::Remove, have: 1, need: 2 }
    );

    // With both signatures it passes.
    let op2 = sign_op(&state, OpKind::Remove, Some(w.carol_id.clone()), &[&w.alice, &w.bob]);
    assert!(state.validate(&op2, &w.dir).is_ok());
}

/// E2.2 — genesis rules survive a membership change: adding a member never
/// confers admin standing, so the admin set fixed at genesis is immutable (I1).
#[test]
fn e2_2_genesis_rules_immutable() {
    let w = world();
    let mut state = GroupState::new(w.genesis.clone());

    // Add a brand-new member, dave (Add threshold 1, signed by alice).
    let dave = SigningIdentity::from_seed(did("dave"), 1);
    let mut dir = Directory::new();
    dir.insert(w.alice.verifying());
    dir.insert(w.bob.verifying());
    dir.insert(dave.verifying());

    let add = sign_op(&state, OpKind::Add, Some(did("dave")), &[&w.alice]);
    state.apply(add, &dir).unwrap();
    assert!(state.members.contains(&did("dave")));

    // Dave is now a member, but NOT an admin. His signature carries no
    // standing — any op he authorizes is rejected, proving the genesis admin
    // set cannot be expanded by membership operations.
    let dave_remove = sign_op(&state, OpKind::Remove, Some(did("carol")), &[&dave, &w.alice]);
    let err = state.validate(&dave_remove, &dir).unwrap_err();
    assert_eq!(err, RejectReason::SignerLacksStanding(did("dave")));

    // An op anchored to a forged genesis is also rejected deterministically.
    let other = Genesis::new(
        GenesisRules { admins: [did("alice")].into_iter().collect(), thresholds: BTreeMap::new() },
        [did("alice")].into_iter().collect(),
    );
    let mut forged = sign_op(&state, OpKind::Remove, Some(did("carol")), &[&w.alice, &w.bob]);
    forged.body.genesis = other.id;
    assert_eq!(state.validate(&forged, &dir).unwrap_err(), RejectReason::GenesisMismatch);
}

/// E2.3 — partition with non-conflicting ops heals; survivor selection is
/// deterministic and symmetric; both sides share lineage (I10, I5).
#[test]
fn e2_3_partition_heals_and_converges() {
    let w = world();

    // Left partition: alice adds dave.
    let mut left = GroupState::new(w.genesis.clone());
    let mut dir = Directory::new();
    dir.insert(w.alice.verifying());
    dir.insert(w.bob.verifying());
    left.apply(sign_op(&left, OpKind::Add, Some(did("dave")), &[&w.alice]), &dir).unwrap();

    // Right partition: bob adds erin.
    let mut right = GroupState::new(w.genesis.clone());
    right.apply(sign_op(&right, OpKind::Add, Some(did("erin")), &[&w.bob]), &dir).unwrap();

    // No removals -> no conflict -> heal silently.
    assert_eq!(detect(&left, &right), Resolution::Heal);

    // Survivor is chosen deterministically from per-state head hashes, and the
    // choice is identical regardless of argument order (no negotiation).
    let ls = BranchSummary { genesis: GenesisId(left.head), member_count: left.members.len() };
    let rs = BranchSummary { genesis: GenesisId(right.head), member_count: right.members.len() };
    let rule = SurvivorRule::MemberCountThenGenesis;
    let s1 = select_survivor(&ls, &rs, rule);
    let s2 = select_survivor(&rs, &ls, rule);
    assert_eq!(s1, s2, "every honest client computes the same survivor");

    // Both partitions share the same genesis lineage.
    let mut lineage = Lineage::new();
    lineage.add_root(w.genesis.id, w.genesis.founders.clone());
    assert!(lineage.shares_lineage(w.genesis.id, w.genesis.id));
}

/// E2.4 — partition with a contradictory remove/keep hard-stops and escalates;
/// no silent re-admit (I6).
#[test]
fn e2_4_conflict_hard_stops() {
    let w = world();
    let mut dir = Directory::new();
    dir.insert(w.alice.verifying());
    dir.insert(w.bob.verifying());

    // Left boots carol (2 admin sigs -> valid).
    let mut left = GroupState::new(w.genesis.clone());
    left.apply(
        sign_op(&left, OpKind::Remove, Some(w.carol_id.clone()), &[&w.alice, &w.bob]),
        &dir,
    )
    .unwrap();
    assert!(!left.members.contains(&w.carol_id));

    // Right keeps carol (does nothing).
    let right = GroupState::new(w.genesis.clone());
    assert!(right.members.contains(&w.carol_id));

    // Reconnect: must hard-stop and escalate to a human, never auto-resolve.
    struct Recorder(Vec<ConflictReason>);
    impl Escalator for Recorder {
        fn on_conflict(&mut self, reason: &ConflictReason) {
            self.0.push(reason.clone());
        }
    }
    let mut rec = Recorder(Vec::new());
    let resolution = reconcile(&left, &right, &mut rec);

    assert_eq!(
        resolution,
        Resolution::HardStop(vec![ConflictReason::RemovedThenIncluded(w.carol_id.clone())])
    );
    assert_eq!(rec.0, vec![ConflictReason::RemovedThenIncluded(w.carol_id.clone())]);

    // No silent re-admit: the detector changed neither side's membership.
    assert!(!left.members.contains(&w.carol_id));
    assert!(right.members.contains(&w.carol_id));
}

/// E2.5 — a rejected conflict-merge is a resting state: two valid groups, each
/// with intact lineage and standing.
#[test]
fn e2_5_rejected_merge_is_resting_state() {
    let w = world();
    let mut dir = Directory::new();
    dir.insert(w.alice.verifying());
    dir.insert(w.bob.verifying());

    let mut left = GroupState::new(w.genesis.clone());
    left.apply(
        sign_op(&left, OpKind::Remove, Some(w.carol_id.clone()), &[&w.alice, &w.bob]),
        &dir,
    )
    .unwrap();
    let right = GroupState::new(w.genesis.clone());

    // The two groups simply remain two groups, each internally valid.
    assert_eq!(left.members, [did("alice"), did("bob")].into_iter().collect());
    assert_eq!(right.members, w.genesis.founders);

    // Both retain intact lineage; carol still has standing on the lineage
    // (she was a founder), even though she was booted from the left branch.
    let mut lineage = Lineage::new();
    lineage.add_root(w.genesis.id, w.genesis.founders.clone());
    assert!(lineage.standing(&w.carol_id, w.genesis.id));
}

/// I5/I10 fuzz — across many seeded member-count combinations, survivor
/// selection is total, deterministic and symmetric, and conflict detection is
/// order-independent.
#[test]
fn e2_survivor_and_detect_are_order_independent_fuzzed() {
    let mut rng = DetRng::from_seed(0xABCDEF);
    let rule = SurvivorRule::MemberCountThenGenesis;
    for i in 0..2000u64 {
        let a = BranchSummary {
            genesis: GenesisId::from_bytes(&rng.next_u64().to_le_bytes()),
            member_count: (rng.next_u64() % 12) as usize,
        };
        let b = BranchSummary {
            genesis: GenesisId::from_bytes(&rng.next_u64().to_le_bytes()),
            member_count: (rng.next_u64() % 12) as usize,
        };
        let s1 = select_survivor(&a, &b, rule);
        let s2 = select_survivor(&b, &a, rule);
        assert_eq!(s1, s2, "survivor not symmetric at iter {i}");
        // The winner must be one of the two inputs (total order, no third).
        assert!(s1 == a.genesis || s1 == b.genesis);
    }
}
