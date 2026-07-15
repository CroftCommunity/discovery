//! E12.4 — a fresh stamp resets the re-key drift accumulated through removals
//! (Battery 7, Rung A).
//!
//! Earns/bounds: Part 2 §7.6.2 (and §7.9.3) — a fresh stamp resets the re-key drift accumulated through removals — byte-size proxy (register e12.4-byteproxy).
//!
//! An MLS tree evolved through removals accumulates blank (unmerged) leaves; the re-key cost
//! drifts from O(log n) toward O(n) as the tree fragments, and the serialized tree carries
//! those blanks. A fresh stamp draws a pristine tree with no blanks, so the drift is reset
//! to baseline. Measured with the byte-size proxy the plan permits.
//!
//! Falsifies if: the fresh stamp does not restore a pristine (smaller, blank-free) tree
//! relative to the evolved one.

use mls_replant::{remove_by_ids, stamp, tree_bytes, Persona};

#[test]
fn fresh_stamp_resets_drift_from_removals() {
    let n = 8usize;
    let m: Vec<Persona> = (0..n).map(|i| Persona::new(&format!("m{i}"))).collect();
    let others0: Vec<&Persona> = m[1..].iter().collect();

    // Plant a group of 8, then evolve it: remove the ODD-indexed members (m1,m3,m5,m7).
    // Interior removals leave blanks *between* live leaves, which cannot be truncated the
    // way trailing blanks are — this is the drift.
    let mut s = stamp(&m[0], &others0);
    assert_eq!(s.member_count, n);
    let removed_ids: Vec<Vec<u8>> = (1..n).step_by(2).map(|i| m[i].id.clone()).collect();
    remove_by_ids(&mut s.group, &m[0], &removed_ids);
    let live_after = s.group.members().count();
    let evolved_tree = tree_bytes(&s.group).len();

    // Re-plant: a fresh stamp over just the survivors {m0,m2,m4,m6}.
    let survivors: Vec<&Persona> = vec![&m[2], &m[4], &m[6]];
    let fresh = stamp(&m[0], &survivors);
    let fresh_tree = tree_bytes(&fresh.group).len();

    assert_eq!(fresh.member_count, 4, "fresh stamp seats exactly the 4 survivors");
    assert_eq!(live_after, 4, "the evolved tree holds 4 live members (4 removed)");

    // DRIFT RESET: the evolved tree (blanks from the removals) is larger than the pristine
    // fresh tree over the same live members.
    assert!(
        fresh_tree < evolved_tree,
        "a fresh stamp must restore a compact, blank-free tree: evolved={evolved_tree} B, \
         fresh={fresh_tree} B"
    );

    eprintln!(
        "E12.4 RESULT (corroboration, direction): after removing 4 of 8 interior members, the \
         evolved tree carried the drift ({evolved_tree} B, blanks retained); a fresh stamp \
         over the 4 survivors reset it to a pristine {fresh_tree} B tree. NOTE: the byte-size \
         proxy UNDERSTATES the effect — openmls serializes blanks compactly, so the small \
         delta here belies the real O(log n)→O(n) drift in re-key *path* length (which a \
         resolution/blank count, not surfaced by openmls 0.8, would show directly). The \
         direction — fresh < evolved — corroborates the reset; the magnitude is a floor."
    );
}
