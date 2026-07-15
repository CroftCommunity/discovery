//! Phase 2.6 — authority lifetime & governed override (follow-on experiments).
//!
//! Two probes the earlier suites did not cover:
//! * **A2.4 (departed-admin authority)** — does a genesis admin who has been
//!   removed from the group still govern? The thesis claims I2 counts only
//!   signatures "from admins with standing in the current epoch"; this falsifies
//!   the *old* behavior (genesis-admin-forever) and pins the corrected one.
//! * **A2.5 (quorum override)** — a hard-stop conflict can be resolved, but only
//!   by an explicit, threshold-meeting *signed* decision; below threshold (or
//!   with no decision) the hard-stop stands. The algorithm never picks a winner.
//!
//! Run: `cargo test -p lineage-core --test authority`.

use std::collections::{BTreeMap, BTreeSet};

use lineage_core::conflict::{
    detect, quorum_override, Decision, OverrideOutcome, Resolution,
};
use lineage_core::gov::{
    sign_op, Directory, Genesis, GenesisRules, GroupState, OpKind, RejectReason,
};
use lineage_core::ids::Did;
use lineage_core::keys::SigningIdentity;

fn did(s: &str) -> Did {
    Did::new(s)
}

/// Admins {alice, bob}; founders {alice, bob, carol}; Remove needs 2, Add 1.
fn world() -> (SigningIdentity, SigningIdentity, Directory, Genesis) {
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
    (alice, bob, dir, genesis)
}

/// A2.4 — an admin who is *removed from the group* loses governance authority.
/// A genesis admin is necessary but not sufficient: standing is per-epoch, so a
/// departed admin's signature no longer counts. (Closes the gap where the code
/// counted any genesis admin regardless of current membership, contradicting
/// the documented I2.)
#[test]
fn a2_4_departed_admin_loses_authority() {
    let (alice, bob, dir, genesis) = world();
    let mut state = GroupState::new(genesis);

    // While still a member, bob co-signs his own removal — valid (he has
    // standing at the moment the op is evaluated, before it is applied).
    let remove_bob = sign_op(&state, OpKind::Remove, Some(did("bob")), &[&alice, &bob]);
    state.apply(remove_bob, &dir).unwrap();
    assert!(!state.members.contains(&did("bob")));

    // Now bob is a departed admin. He is still in the immutable genesis admin
    // set, but no longer a current member, so a fresh op he signs is rejected
    // as a departed admin — not merely "not an admin".
    let add_dave = sign_op(&state, OpKind::Add, Some(did("dave")), &[&bob]);
    assert_eq!(
        state.validate(&add_dave, &dir).unwrap_err(),
        RejectReason::DepartedAdmin(did("bob")),
    );

    // His signature also fails to *count toward a threshold*: alice alone can
    // still Add (threshold 1), but alice+bob cannot reach Remove's threshold 2,
    // because bob's signature is no longer valid standing.
    assert!(state.meets_threshold(
        &sign_op(&state, OpKind::Add, Some(did("dave")), &[&alice]),
        &dir
    ));
    let remove_carol = sign_op(&state, OpKind::Remove, Some(did("carol")), &[&alice, &bob]);
    assert_eq!(
        state.valid_admin_sigs(&remove_carol, &dir),
        1,
        "only alice's signature retains standing"
    );
    assert!(!state.meets_threshold(&remove_carol, &dir));

    // A still-present admin (alice) is of course unaffected.
    let add_dave_alice = sign_op(&state, OpKind::Add, Some(did("dave")), &[&alice]);
    assert!(state.validate(&add_dave_alice, &dir).is_ok());
}

/// A2.5 — a hard-stop conflict is resolved only by an explicit quorum decision
/// that meets threshold. No decision, or an under-threshold one, leaves the
/// hard-stop standing. The override never re-admits anyone on its own.
#[test]
fn a2_5_quorum_override_requires_explicit_threshold_decision() {
    let (alice, bob, dir, genesis) = world();

    // Left boots carol (2 sigs -> valid); right still includes her.
    let mut left = GroupState::new(genesis.clone());
    left.apply(
        sign_op(&left, OpKind::Remove, Some(did("carol")), &[&alice, &bob]),
        &dir,
    )
    .unwrap();
    let right = GroupState::new(genesis.clone());

    let Resolution::HardStop(reasons) = detect(&left, &right) else {
        panic!("expected a hard-stop");
    };

    // (a) No decision offered -> the hard-stop stands, unchanged.
    let none: BTreeMap<Did, Decision> = BTreeMap::new();
    assert_eq!(
        quorum_override(&reasons, &none, &left, &dir),
        OverrideOutcome::Unresolved(reasons.clone()),
    );

    // (b) An under-threshold decision (ConfirmRemoval signed by alice only,
    // Remove needs 2) is NOT authorized -> still unresolved.
    let mut weak = BTreeMap::new();
    weak.insert(
        did("carol"),
        Decision::ConfirmRemoval(sign_op(&left, OpKind::Remove, Some(did("carol")), &[&alice])),
    );
    assert_eq!(
        quorum_override(&reasons, &weak, &left, &dir),
        OverrideOutcome::Unresolved(reasons.clone()),
    );

    // (c) A threshold-meeting ConfirmRemoval (alice+bob) authorizes the
    // resolution: the quorum explicitly confirmed the boot.
    let mut confirm = BTreeMap::new();
    confirm.insert(
        did("carol"),
        Decision::ConfirmRemoval(sign_op(&left, OpKind::Remove, Some(did("carol")), &[&alice, &bob])),
    );
    assert_eq!(
        quorum_override(&reasons, &confirm, &left, &dir),
        OverrideOutcome::Resolved(vec![(did("carol"), OpKind::Remove)]),
    );

    // (d) Re-admission is also possible, but only via an explicit signed Add
    // meeting threshold (Add needs 1). Never automatic.
    let mut readmit = BTreeMap::new();
    readmit.insert(
        did("carol"),
        Decision::Readmit(sign_op(&left, OpKind::Add, Some(did("carol")), &[&alice])),
    );
    assert_eq!(
        quorum_override(&reasons, &readmit, &left, &dir),
        OverrideOutcome::Resolved(vec![(did("carol"), OpKind::Add)]),
    );

    // (e) A decision for the wrong subject does not authorize the conflict.
    let mut wrong = BTreeMap::new();
    wrong.insert(
        did("carol"),
        Decision::Readmit(sign_op(&left, OpKind::Add, Some(did("dave")), &[&alice])),
    );
    assert_eq!(
        quorum_override(&reasons, &wrong, &left, &dir),
        OverrideOutcome::Unresolved(reasons.clone()),
    );

    // The override mutated nothing: carol is still booted on the left.
    assert!(!left.members.contains(&did("carol")));
}
