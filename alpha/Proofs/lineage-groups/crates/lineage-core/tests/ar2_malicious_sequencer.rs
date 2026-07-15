//! AR-2 — malicious sequencer / blind broker: censorship + reorder resistance.
//!
//! The superpeer/broker is BLIND (sees opaque signed ops, no keys). Its powers
//! are drop, reorder, duplicate, delay, and inject — never forge. This proves
//! the active-attack complement to A3 ("capability not a right") and A2.1
//! (order-independence): a blind sequencer cannot (a) change the deterministic
//! converged state by reordering/duplicating, (b) silently stall a peer — a
//! dropped op leaves the peer at a visibly-behind head, never a false "current",
//! or (c) manufacture a membership change by injecting (forged ops are rejected).
//!
//! This is the sim proof; the cross-machine broker (tamper rejected,
//! through-broker verdict == peer verdict) is green-real-multimachine in A3.

use std::collections::{BTreeMap, BTreeSet};

use lineage_core::gov::{sign_op, Directory, Genesis, GenesisRules, GroupState, OpKind, RejectReason, SignedOp};
use lineage_core::ids::Did;
use lineage_core::keys::SigningIdentity;
use lineage_core::rng::DetRng;

fn did(s: &str) -> Did {
    Did::new(s)
}

/// Apply a delivery stream with buffering (out-of-order ops wait until they
/// chain). Mirrors A2.1's honest replica. Returns the converged state.
fn apply_stream(genesis: &Genesis, dir: &Directory, stream: &[SignedOp]) -> GroupState {
    let mut st = GroupState::new(genesis.clone());
    loop {
        let mut progressed = false;
        for op in stream {
            if op.body.seq == st.log.len() as u64 && op.body.prev == st.head {
                // A duplicate of an already-applied op no longer matches (seq has
                // advanced), so it is simply skipped — idempotent.
                st.apply(op.clone(), dir).unwrap();
                progressed = true;
            }
        }
        if !progressed {
            break;
        }
    }
    st
}

fn world() -> (SigningIdentity, Directory, Genesis) {
    let admin = SigningIdentity::from_seed(did("alice"), 1);
    let mut dir = Directory::new();
    dir.insert(admin.verifying());
    let admins: BTreeSet<Did> = [did("alice")].into_iter().collect();
    let mut thresholds = BTreeMap::new();
    thresholds.insert(OpKind::Add, 1);
    let rules = GenesisRules { admins, thresholds };
    let genesis = Genesis::new(rules, [did("alice")].into_iter().collect());
    (admin, dir, genesis)
}

/// Build the canonical k-op chain and return (canonical_state, ops).
fn canonical_chain(admin: &SigningIdentity, dir: &Directory, genesis: &Genesis, k: usize) -> (GroupState, Vec<SignedOp>) {
    let mut st = GroupState::new(genesis.clone());
    let mut ops = Vec::new();
    for i in 0..k {
        let op = sign_op(&st, OpKind::Add, Some(did(&format!("m{i}"))), &[admin]);
        st.apply(op.clone(), dir).unwrap();
        ops.push(op);
    }
    (st, ops)
}

#[test]
fn ar2_reorder_and_duplicate_cannot_change_the_converged_state() {
    let (admin, dir, genesis) = world();
    for seed in 0..200u64 {
        let mut rng = DetRng::from_seed(seed);
        let k = 2 + (rng.next_u64() % 10) as usize;
        let (canonical, ops) = canonical_chain(&admin, &dir, &genesis, k);

        // Malicious broker: shuffle AND duplicate every op (adversarial reorder +
        // replay). Forging is impossible (no keys), so this is its full power here.
        let mut stream = ops.clone();
        stream.extend(ops.clone()); // duplicate everything
        rng.shuffle(&mut stream);

        let got = apply_stream(&genesis, &dir, &stream);
        assert_eq!(got.head, canonical.head, "seed {seed}: reorder/duplicate changed the head");
        assert_eq!(got.members, canonical.members, "seed {seed}: members diverged");
        assert_eq!(got.log.len(), k, "seed {seed}: duplicates were not idempotent");
    }
}

#[test]
fn ar2_dropped_op_leaves_a_visibly_behind_head_not_a_false_current() {
    let (admin, dir, genesis) = world();
    let k = 6;
    let (canonical, ops) = canonical_chain(&admin, &dir, &genesis, k);

    // Broker drops the op at seq 3. The deprived peer can apply 0..3 and then is
    // stuck at the gap — it CANNOT reach the canonical head.
    let mut withheld = ops.clone();
    withheld.remove(3);

    let deprived = apply_stream(&genesis, &dir, &withheld);
    assert_eq!(deprived.log.len(), 3, "peer applies the strict prefix before the gap");
    assert_ne!(deprived.head, canonical.head, "the deprived peer's head differs from current");
    // Staleness is detectable: the deprived head is a known earlier head, not a
    // false 'current'. A peer comparing heads with a complete peer sees it is behind.
    assert_eq!(deprived.head, ops[2].body.id(), "peer sits at a real earlier head (visibly stale)");
}

#[test]
fn ar2_injected_forged_op_cannot_manufacture_membership() {
    let (admin, dir, genesis) = world();
    let mut dir = dir;
    let (state, _ops) = canonical_chain(&admin, &dir, &genesis, 1);

    // The broker injects an op signed by a NON-admin it controls (mallory).
    let mallory = SigningIdentity::from_seed(did("mallory"), 9);
    dir.insert(mallory.verifying());
    let forged = sign_op(&state, OpKind::Add, Some(did("mallory")), &[&mallory]);
    assert_eq!(
        state.validate(&forged, &dir),
        Err(RejectReason::SignerLacksStanding(did("mallory"))),
        "a blind broker cannot manufacture membership — its injected op has no admin standing"
    );

    // Even an op claiming an admin's identity but not actually signed by them is
    // rejected (bad signature) — the broker holds no keys.
    let mut spoofed = sign_op(&state, OpKind::Add, Some(did("evil")), &[&mallory]);
    // Relabel mallory's signature as if it were alice's (the broker's best forgery).
    let sig = spoofed.sigs.remove(&did("mallory")).unwrap();
    spoofed.sigs.insert(did("alice"), sig);
    assert_eq!(
        state.validate(&spoofed, &dir),
        Err(RejectReason::BadSignature(did("alice"))),
        "a signature not actually produced by the admin is rejected"
    );
}
