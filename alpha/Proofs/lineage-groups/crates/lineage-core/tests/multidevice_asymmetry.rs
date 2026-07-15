//! E2.13 (leave-one vs leave-all) + E2.14 (same-lineage 1-sig vs cross-lineage
//! full threshold). The deliberate asymmetry that lets your devices self-organize
//! while keeping cross-lineage action on a device a normal social decision.

use std::collections::{BTreeMap, BTreeSet};

use lineage_core::gov::{
    devices_of_lineage, sign_op, Directory, Genesis, GenesisRules, GroupState, OpKind,
};
use lineage_core::ids::Did;
use lineage_core::keys::SigningIdentity;

fn did(s: &str) -> Did {
    Did::new(s)
}

/// alice: phone+laptop (lineage "alice"); bob, carol: own lineages. All admins.
/// Remove threshold = 2.
struct W {
    alice_phone: SigningIdentity,
    bob: SigningIdentity,
    carol: SigningIdentity,
    dir: Directory,
    genesis: Genesis,
    lineage_of: BTreeMap<Did, Did>,
}

fn w() -> W {
    let alice_phone = SigningIdentity::from_seed(did("alice.phone"), 1);
    let alice_laptop = SigningIdentity::from_seed(did("alice.laptop"), 1);
    let bob = SigningIdentity::from_seed(did("bob"), 1);
    let carol = SigningIdentity::from_seed(did("carol"), 1);

    let admins: BTreeSet<Did> =
        [did("alice.phone"), did("alice.laptop"), did("bob"), did("carol")]
            .into_iter()
            .collect();
    let mut thresholds = BTreeMap::new();
    thresholds.insert(OpKind::Remove, 2);
    let rules = GenesisRules { admins: admins.clone(), thresholds };
    let genesis = Genesis::new(rules, admins);

    let mut dir = Directory::new();
    for v in [&alice_phone, &alice_laptop, &bob, &carol] {
        dir.insert(v.verifying());
    }
    let lineage_of: BTreeMap<Did, Did> = [
        (did("alice.phone"), did("alice")),
        (did("alice.laptop"), did("alice")),
        (did("bob"), did("bob")),
        (did("carol"), did("carol")),
    ]
    .into_iter()
    .collect();

    W { alice_phone, bob, carol, dir, genesis, lineage_of }
}

#[test]
fn e2_14_same_lineage_one_signature_suffices() {
    let w = w();
    let state = GroupState::new(w.genesis.clone());

    // Remove alice's laptop, authored by alice's phone (same lineage). One sig OK.
    let op = sign_op(&state, OpKind::Remove, Some(did("alice.laptop")), &[&w.alice_phone]);
    assert!(
        state.device_op_meets_threshold(&op, &w.dir, &w.lineage_of, &did("alice")),
        "your own device removing your other device needs one signature"
    );
}

#[test]
fn e2_14_cross_lineage_pays_full_threshold() {
    let w = w();
    let state = GroupState::new(w.genesis.clone());

    // bob alone (cross-lineage to alice) tries to remove alice's laptop: 1 lineage
    // < threshold 2 → not enough.
    let solo = sign_op(&state, OpKind::Remove, Some(did("alice.laptop")), &[&w.bob]);
    assert!(
        !state.device_op_meets_threshold(&solo, &w.dir, &w.lineage_of, &did("alice")),
        "a single outsider cannot remove someone else's device below threshold"
    );

    // bob + carol (two distinct outside lineages) meet the full threshold.
    let pair = sign_op(&state, OpKind::Remove, Some(did("alice.laptop")), &[&w.bob, &w.carol]);
    assert!(
        state.device_op_meets_threshold(&pair, &w.dir, &w.lineage_of, &did("alice")),
        "two distinct outside lineages meet the full cross-lineage threshold"
    );
}

#[test]
fn e2_13_leave_one_vs_leave_all_are_distinct() {
    let w = w();
    let members = GroupState::new(w.genesis).members;

    // leave-this-leaf: a single device DID.
    let leave_one: BTreeSet<Did> = [did("alice.laptop")].into_iter().collect();

    // leave-all-under-lineage: every device of the "alice" lineage.
    let leave_all = devices_of_lineage(&members, &w.lineage_of, &did("alice"));

    assert_eq!(
        leave_all,
        [did("alice.phone"), did("alice.laptop")].into_iter().collect::<BTreeSet<_>>(),
        "leave-all drops every device of the person"
    );
    assert_ne!(leave_one, leave_all, "the two ops are distinct");
    assert!(leave_all.is_superset(&leave_one), "leave-all subsumes leave-one for that lineage");
    // bob/carol are untouched by either.
    assert!(!leave_all.contains(&did("bob")) && !leave_all.contains(&did("carol")));
}
