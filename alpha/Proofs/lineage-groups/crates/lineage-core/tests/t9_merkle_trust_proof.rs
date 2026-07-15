//! T9 — offline transitive trust via Merkle proofs (links I3 provenance / I8
//! backfill verifiability; the dossier's trust-graph primitive).
//!
//! A party publishes a Merkle root over a set of signed trust assertions (e.g.
//! "these DIDs have standing", "these facts hold"). Anyone can then verify that a
//! specific assertion is in the set with a compact inclusion proof — **offline**,
//! against the root alone, with no live authority to query. Tampering (a forged
//! leaf or a doctored path) fails. This is the building block for transitive
//! trust that doesn't reintroduce a trusted online party.

use lineage_core::merkle;

fn leaves() -> Vec<merkle::Hash> {
    // Stand-in trust assertions; in practice each is a signed statement's bytes.
    ["alice vouches bob", "bob in family", "carol admin", "dave guest", "erin neighbor"]
        .iter()
        .map(|s| merkle::leaf_hash(s.as_bytes()))
        .collect()
}

#[test]
fn t9_inclusion_proof_verifies_offline_against_the_root() {
    let ls = leaves();
    let root = merkle::root(&ls);
    // Prove "carol admin" (index 2) is in the published set.
    let leaf = merkle::leaf_hash(b"carol admin");
    let proof = merkle::prove(&ls, 2).expect("index in range");
    assert!(
        merkle::verify(leaf, &proof, root),
        "an inclusion proof verifies against the root with no authority queried"
    );
}

#[test]
fn t9_every_leaf_has_a_valid_proof() {
    let ls = leaves();
    let root = merkle::root(&ls);
    for (i, &leaf) in ls.iter().enumerate() {
        let proof = merkle::prove(&ls, i).unwrap();
        assert!(merkle::verify(leaf, &proof, root), "leaf {i} must prove");
    }
}

#[test]
fn t9_tampered_leaf_or_path_is_rejected() {
    let ls = leaves();
    let root = merkle::root(&ls);
    let proof = merkle::prove(&ls, 2).unwrap();

    // A leaf NOT in the set (forged assertion) does not verify with carol's proof.
    let forged = merkle::leaf_hash(b"mallory admin");
    assert!(!merkle::verify(forged, &proof, root), "a forged assertion is not in the set");

    // A doctored proof (flip a sibling byte) fails.
    let mut bad = proof.clone();
    if let Some(first) = bad.step_mut(0) {
        first.sibling[0] ^= 0xFF;
    }
    let real_leaf = merkle::leaf_hash(b"carol admin");
    assert!(!merkle::verify(real_leaf, &bad, root), "a doctored path fails");

    // Verifying against the WRONG root fails.
    let other_root = merkle::root(&[merkle::leaf_hash(b"different set")]);
    assert!(!merkle::verify(real_leaf, &proof, other_root), "wrong root fails");
}

#[test]
fn t9_root_is_deterministic_and_order_sensitive() {
    let a = merkle::root(&leaves());
    let b = merkle::root(&leaves());
    assert_eq!(a, b, "same set → same root (deterministic)");
    // Order matters (a set commitment must fix order, or define a canonical sort).
    let mut reordered = leaves();
    reordered.swap(0, 1);
    assert_ne!(merkle::root(&reordered), a, "reordering changes the root");
}
