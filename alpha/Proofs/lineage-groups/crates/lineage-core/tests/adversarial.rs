//! Phase 2.5 — adversarial governance experiments.
//!
//! These go after the invariants harder than the E2 suite: order-independent
//! convergence under fuzzed delivery (I10/I5), and attributable detection of an
//! admin that equivocates (the "fork-detecting" claim). Run:
//! `cargo test -p lineage-core --test adversarial`.

use std::collections::{BTreeMap, BTreeSet};

use lineage_core::gov::{
    detect_equivocation, sign_op, Directory, Genesis, GenesisRules, GroupState, OpKind, SignedOp,
};
use lineage_core::ids::Did;
use lineage_core::keys::SigningIdentity;
use lineage_core::rng::DetRng;

fn did(s: &str) -> Did {
    Did::new(s)
}

/// A2.1 (fuzz of I10) — a single branch's forward-only signed log converges to
/// the *same* state on every replica regardless of delivery order. A replica
/// buffers out-of-order ops and applies any whose `prev`/`seq` match its head;
/// after quiescence it must match the canonical in-order application exactly.
#[test]
fn a2_1_convergence_under_fuzzed_delivery() {
    let admin = SigningIdentity::from_seed(did("alice"), 1);
    let mut dir = Directory::new();
    dir.insert(admin.verifying());

    let admins: BTreeSet<Did> = [did("alice")].into_iter().collect();
    let mut thresholds = BTreeMap::new();
    thresholds.insert(OpKind::Add, 1);
    let rules = GenesisRules { admins, thresholds };
    let genesis = Genesis::new(rules, [did("alice")].into_iter().collect());

    for seed in 0..300u64 {
        let mut rng = DetRng::from_seed(seed);
        let k = 1 + (rng.next_u64() % 12) as usize;

        // Canonical chain: apply k adds strictly in order.
        let mut canonical = GroupState::new(genesis.clone());
        let mut ops: Vec<SignedOp> = Vec::new();
        for i in 0..k {
            let op = sign_op(
                &canonical,
                OpKind::Add,
                Some(did(&format!("m{i}"))),
                &[&admin],
            );
            canonical.apply(op.clone(), &dir).unwrap();
            ops.push(op);
        }

        // Several replicas each receive the ops in a different shuffled order,
        // buffering until each op chains onto their head.
        for replica_seed in 0..4u64 {
            let mut rng2 = DetRng::from_seed(seed.wrapping_mul(31).wrapping_add(replica_seed));
            let mut shuffled = ops.clone();
            rng2.shuffle(&mut shuffled);

            let mut replica = GroupState::new(genesis.clone());
            loop {
                let mut progressed = false;
                for op in &shuffled {
                    if op.body.seq == replica.log.len() as u64 && op.body.prev == replica.head {
                        replica.apply(op.clone(), &dir).unwrap();
                        progressed = true;
                    }
                }
                if !progressed {
                    break;
                }
            }

            assert_eq!(replica.log.len(), k, "seed {seed}: replica did not converge");
            assert_eq!(replica.head, canonical.head, "seed {seed}: head diverged");
            assert_eq!(replica.members, canonical.members, "seed {seed}: members diverged");
        }
    }
}

/// A2.2 (equivocation) — an admin who signs two conflicting ops at the same
/// chain position is detected and attributed; a non-conflict (identical op, or
/// ops at different positions) is not flagged.
#[test]
fn a2_2_admin_equivocation_is_detected_and_attributed() {
    let alice = SigningIdentity::from_seed(did("alice"), 1);
    let bob = SigningIdentity::from_seed(did("bob"), 1);
    let mut dir = Directory::new();
    dir.insert(alice.verifying());
    dir.insert(bob.verifying());

    let admins: BTreeSet<Did> = [did("alice"), did("bob")].into_iter().collect();
    let mut thresholds = BTreeMap::new();
    thresholds.insert(OpKind::Remove, 2);
    let rules = GenesisRules { admins, thresholds };
    let genesis = Genesis::new(
        rules,
        [did("alice"), did("bob"), did("carol"), did("dave")]
            .into_iter()
            .collect(),
    );
    let state = GroupState::new(genesis);

    // Two DIFFERENT ops at seq 0, both signed by alice and bob.
    let boot_carol = sign_op(&state, OpKind::Remove, Some(did("carol")), &[&alice, &bob]);
    let boot_dave = sign_op(&state, OpKind::Remove, Some(did("dave")), &[&alice, &bob]);

    let eqs = detect_equivocation(&boot_carol, &boot_dave, &dir);
    let culprits: BTreeSet<Did> = eqs.iter().map(|e| e.culprit.clone()).collect();
    assert_eq!(
        culprits,
        [did("alice"), did("bob")].into_iter().collect(),
        "both double-signers must be attributed"
    );
    // Same position, and a genuine conflict (distinct op ids).
    assert!(eqs.iter().all(|e| e.seq == 0 && e.op_lo != e.op_hi));

    // Detection is symmetric.
    assert_eq!(
        detect_equivocation(&boot_carol, &boot_dave, &dir),
        detect_equivocation(&boot_dave, &boot_carol, &dir)
    );

    // The same op against itself is NOT an equivocation.
    assert!(detect_equivocation(&boot_carol, &boot_carol, &dir).is_empty());

    // A forged "equivocation" where the second signature is invalid (bob did
    // not actually sign the conflicting op) attributes only the real signer.
    let dave_only_alice = sign_op(&state, OpKind::Remove, Some(did("dave")), &[&alice]);
    let eqs2 = detect_equivocation(&boot_carol, &dave_only_alice, &dir);
    let culprits2: BTreeSet<Did> = eqs2.iter().map(|e| e.culprit.clone()).collect();
    assert_eq!(culprits2, [did("alice")].into_iter().collect());
}
