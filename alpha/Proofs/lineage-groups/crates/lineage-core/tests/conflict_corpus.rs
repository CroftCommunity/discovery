//! Merge/split corpus widening (thinking/merge-split-corpus.md §4): the conflict
//! reasons beyond the original C1.
//!
//! - C7  dissolve-vs-continue → hard-stop (newly added detector reason).
//! - C8  diamond recombine: standing/shares_lineage hold over a multi-parent DAG.
//! - C9  governance equivocation is detected + attributed (exercises the existing
//!       `detect_equivocation` from A2.2).
//! - C10 ban-evasion: a removed member's new device cannot silently re-confer
//!       standing — re-admission requires a threshold-meeting Add.

use std::collections::{BTreeMap, BTreeSet};

use lineage_core::conflict::{detect, ConflictReason, Resolution};
use lineage_core::dag::Lineage;
use lineage_core::gov::{
    detect_equivocation, sign_op, Directory, Genesis, GenesisRules, GroupState, OpKind, RejectReason,
};
use lineage_core::ids::{Did, GenesisId};
use lineage_core::keys::SigningIdentity;

fn did(s: &str) -> Did {
    Did::new(s)
}

fn world() -> (SigningIdentity, SigningIdentity, Directory, Genesis) {
    let alice = SigningIdentity::from_seed(did("alice"), 1);
    let bob = SigningIdentity::from_seed(did("bob"), 1);
    let carol = SigningIdentity::from_seed(did("carol"), 1);
    let admins: BTreeSet<Did> = [did("alice"), did("bob")].into_iter().collect();
    let mut thresholds = BTreeMap::new();
    thresholds.insert(OpKind::Remove, 2);
    thresholds.insert(OpKind::Add, 2);
    let rules = GenesisRules { admins, thresholds };
    let founders: BTreeSet<Did> =
        [did("alice"), did("bob"), did("carol")].into_iter().collect();
    let genesis = Genesis::new(rules, founders);
    let mut dir = Directory::new();
    for v in [&alice, &bob, &carol] {
        dir.insert(v.verifying());
    }
    (alice, bob, dir, genesis)
}

#[test]
fn c7_dissolve_vs_continue_hard_stops() {
    let (alice, bob, dir, genesis) = world();
    let mut dissolved = GroupState::new(genesis.clone());
    let alive = GroupState::new(genesis);

    // Left side dissolves the group (Dissolve defaults to threshold 1; sign with both).
    let op = sign_op(&dissolved, OpKind::Dissolve, None, &[&alice, &bob]);
    dissolved.apply(op, &dir).unwrap();
    assert!(dissolved.dissolved);

    match detect(&dissolved, &alive) {
        Resolution::HardStop(reasons) => {
            assert!(reasons.contains(&ConflictReason::DissolvedThenContinued))
        }
        Resolution::Heal => panic!("dissolve-vs-continue must hard-stop, not heal"),
    }
}

#[test]
fn c8_diamond_recombine_preserves_standing_and_shared_lineage() {
    // Two roots r1, r2; a child c recombines from both (a diamond / two-parent
    // node). A member of either root must have standing on the recombined branch,
    // and the recombined branch shares lineage with both roots.
    let mut lin = Lineage::new();
    let r1 = GenesisId::from_bytes(b"root-1");
    let r2 = GenesisId::from_bytes(b"root-2");
    let c = GenesisId::from_bytes(b"recombined-child");
    lin.add_root(r1, [did("alice")]);
    lin.add_root(r2, [did("bob")]);
    lin.recombine(r1, r2, c, [did("carol")]);

    assert!(lin.standing(&did("alice"), c), "a member of root-1 has standing on the diamond child");
    assert!(lin.standing(&did("bob"), c), "a member of root-2 has standing on the diamond child");
    assert!(lin.shares_lineage(c, r1) && lin.shares_lineage(c, r2));
    // An outsider who was never a member of any ancestor has no standing.
    assert!(!lin.standing(&did("mallory"), c));
}

#[test]
fn c9_equivocation_is_detected_and_attributed() {
    // Alice signs two DIFFERENT ops at the same chain position (seq 0): the
    // classic governance fork. detect_equivocation attributes it to her.
    let (alice, bob, dir, genesis) = world();
    let state = GroupState::new(genesis);

    let op_a = sign_op(&state, OpKind::Remove, Some(did("carol")), &[&alice, &bob]);
    let op_b = sign_op(&state, OpKind::Add, Some(did("dave")), &[&alice, &bob]);

    let eqs = detect_equivocation(&op_a, &op_b, &dir);
    assert!(!eqs.is_empty(), "two conflicting ops at the same seq must be flagged");
    let culprits: BTreeSet<&Did> = eqs.iter().map(|e| &e.culprit).collect();
    assert!(culprits.contains(&did("alice")) && culprits.contains(&did("bob")));
}

#[test]
fn c10_ban_evasion_new_device_cannot_self_confer_standing() {
    // Carol is removed. A "new device" for carol (carol.alt) tries to act and to
    // be re-added. It is neither an admin nor a member, so it cannot vote, and
    // re-admission requires a threshold-meeting Add by the real admins.
    let (alice, bob, mut dir, genesis) = world();
    let mut state = GroupState::new(genesis);

    let remove = sign_op(&state, OpKind::Remove, Some(did("carol")), &[&alice, &bob]);
    state.apply(remove, &dir).unwrap();
    assert!(!state.members.contains(&did("carol")));

    let carol_alt = SigningIdentity::from_seed(did("carol.alt"), 9);
    dir.insert(carol_alt.verifying());

    // The new device cannot author governance (not an admin).
    let self_readmit = sign_op(&state, OpKind::Add, Some(did("carol.alt")), &[&carol_alt]);
    assert_eq!(
        state.validate(&self_readmit, &dir),
        Err(RejectReason::SignerLacksStanding(did("carol.alt"))),
        "a removed member's new device cannot vote itself back in"
    );

    // Re-admission is possible ONLY via a threshold-meeting Add by the real admins.
    let proper_add = sign_op(&state, OpKind::Add, Some(did("carol.alt")), &[&alice, &bob]);
    assert!(state.apply(proper_add, &dir).is_ok(), "admins may choose to re-admit (audited, explicit)");
}
