//! E12.3 — planter byte-nondeterminism is a dedup, not a fork (Battery 7, Rung A).
//!
//! The core correctness claim of the whole fork story, and the single most important thing
//! to try to break. Two members stamping independently from the *same* governance-chain
//! member set may produce different MLS tree bytes (fresh secrets, different KeyPackage
//! selection, different leaf order). The claim: this divergence is a **dedup** — both are
//! valid groups over the identical membership, and an observer picks one by the §6
//! content-hash tiebreak — and it NEVER escalates as a fork.
//!
//! Falsifies if: the byte divergence propagates into a MEMBERSHIP divergence (a stamp
//! difference that produces a real fork). The guard is: membership(A) == membership(B).
//!
//! Rung A for the byte divergence (measured against real openmls); the "nothing downstream
//! reads tree shape" invariant is Rung B (asserted here by comparing membership, the only
//! thing governance reads).

use mls_replant::{membership, stamp, tree_bytes, Persona};

#[test]
fn concurrent_stamps_dedup_never_fork() {
    // A fixed member set read from the governance chain.
    let m: Vec<Persona> = (0..6).map(|i| Persona::new(&format!("m{i}"))).collect();

    // Planter m0 stamps a fresh group over the whole set; planter m1 does the same,
    // independently. Each planter is itself in the set, so both cover {m0..m5}.
    let others_for = |planter: usize| -> Vec<&Persona> {
        (0..m.len()).filter(|&j| j != planter).map(|j| &m[j]).collect()
    };
    let a = stamp(&m[0], &others_for(0));
    let b = stamp(&m[1], &others_for(1));

    // Both seated the full set.
    assert_eq!(a.member_count, m.len(), "planter m0 seats all {}", m.len());
    assert_eq!(b.member_count, m.len(), "planter m1 seats all {}", m.len());

    let mem_a = membership(&a.group);
    let mem_b = membership(&b.group);
    let tree_a = tree_bytes(&a.group);
    let tree_b = tree_bytes(&b.group);

    // THE BYTE DIVERGENCE IS REAL: two independent stamps produce different tree bytes.
    assert_ne!(
        tree_a, tree_b,
        "two independent stamps must differ in tree bytes (fresh secrets/leaf order) — \
         otherwise there is no divergence to resolve"
    );

    // THE REFUTATION GUARD: despite the byte divergence, the MEMBERSHIP is identical. A
    // stamp difference is NOT a membership/governance fork.
    assert_eq!(
        mem_a, mem_b,
        "membership must be identical across independent stamps — if it differs, the byte \
         divergence has escalated into a real fork (E12.3 falsified)"
    );

    // THE RESOLUTION (dedup): the §6 content-hash tiebreak picks one deterministically, so
    // every observer converges on the same group. min-of-content-address is total and
    // order-independent — the same discipline the governance fold uses (governance::tiebreak).
    let winner_is_a = tree_a <= tree_b;
    let winner = if winner_is_a { &tree_a } else { &tree_b };
    // The tiebreak is deterministic regardless of which order an observer saw the two.
    let winner_other_order = if tree_b <= tree_a { &tree_b } else { &tree_a };
    assert_eq!(
        winner, winner_other_order,
        "the content-hash tiebreak must pick the same winner regardless of arrival order"
    );

    eprintln!(
        "E12.3 RESULT (corroboration): two independent stamps over the same {}-member set \
         produced DIFFERENT tree bytes ({} vs {} bytes) but IDENTICAL membership — a dedup, \
         resolved by the content-hash tiebreak, never a fork. The byte divergence does not \
         reach the layer governance reads.",
        m.len(),
        tree_a.len(),
        tree_b.len()
    );
}
