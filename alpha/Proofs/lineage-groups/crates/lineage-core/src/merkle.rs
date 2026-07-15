//! Offline transitive trust via Merkle proofs (T9; links I3 provenance / I8
//! backfill verifiability).
//!
//! A party commits to a set of trust assertions with a single Merkle [`root`].
//! Anyone holding the root can later verify that a specific assertion is in the
//! set with a compact inclusion [`Proof`] — **offline**, with no authority to
//! query. Domain-separated leaf/node hashing (RFC 6962-style) blocks the
//! second-preimage attack where an internal node is passed off as a leaf.

use sha2::{Digest, Sha256};

/// A 32-byte hash.
pub type Hash = [u8; 32];

/// Hash a leaf's data (domain-separated from internal nodes).
pub fn leaf_hash(data: &[u8]) -> Hash {
    let mut h = Sha256::new();
    h.update([0x00]); // leaf domain tag
    h.update(data);
    h.finalize().into()
}

/// Hash an internal node from its two children.
fn node_hash(left: &Hash, right: &Hash) -> Hash {
    let mut h = Sha256::new();
    h.update([0x01]); // node domain tag
    h.update(left);
    h.update(right);
    h.finalize().into()
}

/// The Merkle root over `leaves` (already leaf-hashed). An odd node at any level
/// is promoted by hashing it with itself, fixing the tree shape. Empty input
/// yields the all-zero hash.
pub fn root(leaves: &[Hash]) -> Hash {
    if leaves.is_empty() {
        return [0u8; 32];
    }
    let mut level = leaves.to_vec();
    while level.len() > 1 {
        let mut next = Vec::with_capacity(level.len().div_ceil(2));
        let mut i = 0;
        while i < level.len() {
            let l = level[i];
            let r = if i + 1 < level.len() { level[i + 1] } else { level[i] };
            next.push(node_hash(&l, &r));
            i += 2;
        }
        level = next;
    }
    level[0]
}

/// One step of an inclusion path: a sibling hash and whether it sits on the right.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Step {
    pub sibling: Hash,
    pub sibling_is_right: bool,
}

/// A compact inclusion proof: the sibling hashes from leaf to root.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Proof {
    pub siblings: Vec<Step>,
}

impl Proof {
    /// Public accessor used by tests to tamper with a step (see T9).
    pub fn step_mut(&mut self, i: usize) -> Option<&mut Step> {
        self.siblings.get_mut(i)
    }
}

/// Produce an inclusion proof for the leaf at `index`, or `None` if out of range.
pub fn prove(leaves: &[Hash], index: usize) -> Option<Proof> {
    if index >= leaves.len() {
        return None;
    }
    let mut level = leaves.to_vec();
    let mut idx = index;
    let mut siblings = Vec::new();
    while level.len() > 1 {
        let sib_index = if idx % 2 == 0 { idx + 1 } else { idx - 1 };
        let sibling = if sib_index < level.len() { level[sib_index] } else { level[idx] };
        siblings.push(Step { sibling, sibling_is_right: idx % 2 == 0 });

        let mut next = Vec::with_capacity(level.len().div_ceil(2));
        let mut i = 0;
        while i < level.len() {
            let l = level[i];
            let r = if i + 1 < level.len() { level[i + 1] } else { level[i] };
            next.push(node_hash(&l, &r));
            i += 2;
        }
        idx /= 2;
        level = next;
    }
    Some(Proof { siblings })
}

/// Verify that `leaf` is included under `root` via `proof`. Pure and offline.
pub fn verify(leaf: Hash, proof: &Proof, root_hash: Hash) -> bool {
    let mut acc = leaf;
    for step in &proof.siblings {
        acc = if step.sibling_is_right {
            node_hash(&acc, &step.sibling)
        } else {
            node_hash(&step.sibling, &acc)
        };
    }
    acc == root_hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_leaf_root_is_itself() {
        let l = leaf_hash(b"only");
        assert_eq!(root(&[l]), l);
        // A degenerate proof (no siblings) verifies a single-leaf tree.
        assert!(verify(l, &prove(&[l], 0).unwrap(), root(&[l])));
    }

    #[test]
    fn odd_leaf_count_still_proves() {
        let ls: Vec<Hash> = (0..5u8).map(|i| leaf_hash(&[i])).collect();
        let r = root(&ls);
        for (i, &leaf) in ls.iter().enumerate() {
            assert!(verify(leaf, &prove(&ls, i).unwrap(), r));
        }
        assert!(prove(&ls, 5).is_none());
    }
}
