//! T3 — real threshold-signed compaction checkpoint (closes ledger dep #3; the
//! real-crypto form of the model's F-group / F2).
//!
//! A checkpoint is a signed summary of a branch's log up to a head, used for
//! roll-up/compaction so history doesn't grow unbounded (the SSB trap). The
//! security question (F2, the "referee leak"): is the checkpoint signed by a
//! THRESHOLD of admins, or secretly by the superpeer/broker (a single authority)?
//! If a single-authority checkpoint were accepted, the broker would be a de-facto
//! ordering/finality authority — exactly the dirty secret. These prove:
//!   - a threshold of admin LINEAGES must sign (lineage-counted, per E2.10);
//!   - a single authority / below-threshold / non-admin checkpoint is rejected;
//!   - the checkpointed head must match the real log (no tampered summary);
//!   - a checkpoint is bound to a specific head, so forked branches cannot share
//!     one — a checkpoint cannot span an open fork.

use std::collections::{BTreeMap, BTreeSet};

use lineage_core::gov::{
    sign_op, verify_checkpoint, Checkpoint, CheckpointError, Directory, Genesis, GenesisRules,
    GroupState, OpKind,
};
use lineage_core::ids::Did;
use lineage_core::keys::SigningIdentity;

fn did(s: &str) -> Did {
    Did::new(s)
}

/// admins {alice, bob, carol} (each its own lineage); members + admins; a couple
/// of ops applied so the log has a non-genesis head. Checkpoint threshold = 2.
struct W {
    alice: SigningIdentity,
    bob: SigningIdentity,
    alice2: SigningIdentity, // alice's second device, same lineage
    broker: SigningIdentity, // a non-admin "superpeer"
    dir: Directory,
    genesis: Genesis,
    lineage_of: BTreeMap<Did, Did>,
}

fn w() -> W {
    let alice = SigningIdentity::from_seed(did("alice.dev1"), 1);
    let alice2 = SigningIdentity::from_seed(did("alice.dev2"), 1);
    let bob = SigningIdentity::from_seed(did("bob"), 1);
    let carol = SigningIdentity::from_seed(did("carol"), 1);
    let broker = SigningIdentity::from_seed(did("broker"), 1);

    let admins: BTreeSet<Did> =
        [did("alice.dev1"), did("alice.dev2"), did("bob"), did("carol")].into_iter().collect();
    let mut thresholds = BTreeMap::new();
    thresholds.insert(OpKind::Add, 1);
    let rules = GenesisRules { admins: admins.clone(), thresholds };
    let genesis = Genesis::new(rules, admins);

    let mut dir = Directory::new();
    for v in [&alice, &alice2, &bob, &carol, &broker] {
        dir.insert(v.verifying());
    }
    let lineage_of: BTreeMap<Did, Did> = [
        (did("alice.dev1"), did("alice")),
        (did("alice.dev2"), did("alice")),
        (did("bob"), did("bob")),
        (did("carol"), did("carol")),
    ]
    .into_iter()
    .collect();

    W { alice, bob, alice2, broker, dir, genesis, lineage_of }
}

/// Advance the log by one Add so the head is non-genesis.
fn advanced(w: &W) -> GroupState {
    let mut st = GroupState::new(w.genesis.clone());
    let op = sign_op(&st, OpKind::Add, Some(did("newbie")), &[&w.alice]);
    st.apply(op, &w.dir).unwrap();
    st
}

#[test]
fn t3_threshold_signed_checkpoint_verifies() {
    let w = w();
    let st = advanced(&w);
    let cp = Checkpoint::sign(&st, &[&w.alice, &w.bob]); // two distinct lineages
    assert!(
        verify_checkpoint(&st, &cp, &w.dir, &w.lineage_of, 2).is_ok(),
        "a checkpoint signed by a threshold of admin lineages, on the real head, verifies"
    );
}

#[test]
fn t3_single_authority_checkpoint_is_rejected() {
    // F2 / the referee leak: a checkpoint signed only by the broker (a single
    // non-admin authority) must be rejected — the broker is NOT a finality authority.
    let w = w();
    let st = advanced(&w);
    let broker_cp = Checkpoint::sign(&st, &[&w.broker]);
    assert!(
        matches!(verify_checkpoint(&st, &broker_cp, &w.dir, &w.lineage_of, 2), Err(CheckpointError::UnderThreshold { .. })),
        "a broker-signed checkpoint carries zero admin lineages → under threshold, rejected"
    );

    // Below threshold: a single admin lineage (alice) is < 2.
    let alice_cp = Checkpoint::sign(&st, &[&w.alice]);
    assert!(matches!(
        verify_checkpoint(&st, &alice_cp, &w.dir, &w.lineage_of, 2),
        Err(CheckpointError::UnderThreshold { have: 1, need: 2 })
    ));

    // Own-device padding (E2.10 interaction): alice's two devices are one lineage.
    let alice_two_devices = Checkpoint::sign(&st, &[&w.alice, &w.alice2]);
    assert!(
        matches!(verify_checkpoint(&st, &alice_two_devices, &w.dir, &w.lineage_of, 2), Err(CheckpointError::UnderThreshold { have: 1, need: 2 })),
        "two of alice's devices count as one lineage — cannot self-finalize a checkpoint"
    );
}

#[test]
fn t3_checkpoint_head_must_match_the_real_log() {
    let w = w();
    let st = advanced(&w);
    let mut cp = Checkpoint::sign(&st, &[&w.alice, &w.bob]);
    cp.head = [0xAB; 32]; // claim a head the log never had
    assert!(matches!(
        verify_checkpoint(&st, &cp, &w.dir, &w.lineage_of, 2),
        Err(CheckpointError::HeadMismatch)
    ));
}

#[test]
fn t3_checkpoint_cannot_span_an_open_fork() {
    // Two branches fork from the same genesis with divergent heads. A checkpoint
    // valid for branch-L is bound to L's head, so it fails to verify against
    // branch-R — a single checkpoint cannot cover an unreconciled fork.
    let w = w();
    let mut left = GroupState::new(w.genesis.clone());
    let mut right = GroupState::new(w.genesis.clone());
    left.apply(sign_op(&left, OpKind::Add, Some(did("L")), &[&w.alice]), &w.dir).unwrap();
    right.apply(sign_op(&right, OpKind::Add, Some(did("R")), &[&w.alice]), &w.dir).unwrap();
    assert_ne!(left.head, right.head, "the forked branches have distinct heads");

    let cp_left = Checkpoint::sign(&left, &[&w.alice, &w.bob]);
    assert!(verify_checkpoint(&left, &cp_left, &w.dir, &w.lineage_of, 2).is_ok());
    assert!(
        matches!(verify_checkpoint(&right, &cp_left, &w.dir, &w.lineage_of, 2), Err(CheckpointError::HeadMismatch)),
        "a checkpoint is bound to one branch's head — it cannot span the open fork"
    );
}
