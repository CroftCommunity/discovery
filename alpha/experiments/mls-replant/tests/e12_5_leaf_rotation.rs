//! E12.5 — a fresh stamp is a group-wide leaf-key rotation (Battery 7, Rung A).
//!
//! Earns/bounds: Part 2 §7.6.2 (and §8.1) — a fresh stamp is a group-wide leaf-key rotation (a free group-wide re-key).
//!
//! Because each fresh group draws a fresh KeyPackage per member, the re-plant rotates every
//! member's leaf (HPKE) encryption key at once — a free group-wide re-key. The stable
//! signature key (the persona's identity) is preserved, so it is a *re-key*, not a change of
//! who is in the group. (The last-resort exception — a member seated on a reused last-resort
//! package does not rotate until they republish — is E12.6, deferred.)
//!
//! Falsifies if: any member's leaf encryption key fails to rotate across a fresh stamp, or a
//! member's identity (signature key) changes.

use std::collections::HashMap;

use mls_replant::{leaf_keys, stamp, Persona};

#[test]
fn fresh_stamp_rotates_every_leaf_key() {
    let m: Vec<Persona> = (0..6).map(|i| Persona::new(&format!("m{i}"))).collect();
    let others_for = |planter: usize| -> Vec<&Persona> {
        (0..m.len()).filter(|&j| j != planter).map(|j| &m[j]).collect()
    };

    // First plant, then an independent re-plant over the SAME member set.
    let a = stamp(&m[0], &others_for(0));
    let b = stamp(&m[0], &others_for(0));

    // Map identity (signature key) → leaf encryption key, per stamp.
    let enc_a: HashMap<Vec<u8>, Vec<u8>> = leaf_keys(&a.group).into_iter().collect();
    let enc_b: HashMap<Vec<u8>, Vec<u8>> = leaf_keys(&b.group).into_iter().collect();

    // Same identities in both (a re-key, not a membership change).
    let mut ids_a: Vec<&Vec<u8>> = enc_a.keys().collect();
    let mut ids_b: Vec<&Vec<u8>> = enc_b.keys().collect();
    ids_a.sort();
    ids_b.sort();
    assert_eq!(ids_a, ids_b, "the same personae (signature keys) are in both stamps");

    // Every member's leaf encryption key rotated.
    let mut rotated = 0usize;
    for (id, enc_before) in &enc_a {
        let enc_after = enc_b.get(id).expect("same identity present");
        assert_ne!(
            enc_before, enc_after,
            "member's leaf encryption key must rotate across a fresh stamp"
        );
        rotated += 1;
    }
    assert_eq!(rotated, m.len(), "every one of the {} members rotated", m.len());

    eprintln!(
        "E12.5 RESULT (corroboration): a fresh stamp rotated the leaf encryption key of all \
         {} members at once, while preserving every persona's identity (signature key) — a \
         group-wide re-key for free.",
        m.len()
    );
}
