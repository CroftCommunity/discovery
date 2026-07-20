//! MST reachability proof — the leaf-CID set reachable from the signed
//! commit's `data` root, walking every value link (`v`) and subtree link
//! (`t` / `l`) in every MST inner node.
//!
//! This does NOT parse the MST semantically (no key reconstruction, no
//! ordering verification) — it only verifies **containment**: "was this
//! leaf CID reachable from the tree root that the signed commit points
//! to?".  That's the security property E-MST asserts: a tampering mirror
//! that returns a valid leaf-block for a record NOT in the tree fails,
//! because the leaf isn't reachable from the signed root.
//!
//! Every block traversed is checked for CID integrity: `cid_v1_dag_cbor(&bytes)
//! == declared_cid`.  A mirror can't rebadge a block without breaking this.

use crate::canonical::cid_v1_dag_cbor;
use crate::car::Car;
use cid::Cid;
use ipld_core::ipld::Ipld;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct MstReachability {
    /// Total blocks visited by the walk (excluding leaves, which are just
    /// values we recorded).
    pub inner_blocks_visited: usize,
    /// CIDs that appeared as MST leaf `v` values (record content links).
    pub leaves_reachable: Vec<String>,
    /// Any structural warning surfaced during the walk (e.g., a link points
    /// to a block missing from the CAR).
    pub warnings: Vec<String>,
}

#[derive(Debug)]
pub enum MstError {
    RootBlockMissing(Cid),
    BlockCidMismatch { declared: Cid, computed: Cid },
    InnerNodeDecode(String),
    NoDataFieldInCommit,
}

impl std::fmt::Display for MstError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MstError::RootBlockMissing(c) => write!(f, "MST root {} missing from CAR", c),
            MstError::BlockCidMismatch { declared, computed } => write!(
                f,
                "MST block CID mismatch: declared {}, computed from bytes {}",
                declared, computed
            ),
            MstError::InnerNodeDecode(s) => write!(f, "MST inner-node decode: {}", s),
            MstError::NoDataFieldInCommit => write!(f, "commit has no `data` field"),
        }
    }
}

impl std::error::Error for MstError {}

/// Extract the `data` field CID from a commit block.  The commit is
/// `{data: Link, did, prev, rev, sig, version}`.
pub fn extract_commit_data_root(commit_bytes: &[u8]) -> Result<Cid, MstError> {
    let ipld: Ipld = serde_ipld_dagcbor::from_slice(commit_bytes)
        .map_err(|e| MstError::InnerNodeDecode(e.to_string()))?;
    let map = match ipld {
        Ipld::Map(m) => m,
        _ => return Err(MstError::NoDataFieldInCommit),
    };
    match map.get("data") {
        Some(Ipld::Link(c)) => Ok(*c),
        _ => Err(MstError::NoDataFieldInCommit),
    }
}

/// Walk the MST from `root`, collecting the set of leaf CIDs reachable via
/// `v` links, and validating CID integrity at each block.  Subtree links
/// (`t` per-entry and `l` left-of-first-entry) are followed recursively.
pub fn walk_reachable_leaves(car: &Car, root: Cid) -> Result<MstReachability, MstError> {
    let mut visited: HashSet<Cid> = HashSet::new();
    let mut leaves: HashSet<Cid> = HashSet::new();
    let mut warnings: Vec<String> = Vec::new();
    let mut inner = 0usize;
    let mut stack = vec![root];
    while let Some(cid) = stack.pop() {
        if !visited.insert(cid) {
            continue;
        }
        let bytes = match car.by_cid.get(&cid) {
            Some(b) => b,
            None => {
                warnings.push(format!("subtree link {} missing from CAR", cid));
                continue;
            }
        };
        let computed = cid_v1_dag_cbor(bytes);
        if computed != cid {
            return Err(MstError::BlockCidMismatch {
                declared: cid,
                computed,
            });
        }
        inner += 1;
        let ipld: Ipld = serde_ipld_dagcbor::from_slice(bytes)
            .map_err(|e| MstError::InnerNodeDecode(e.to_string()))?;
        let map = match ipld {
            Ipld::Map(m) => m,
            _ => {
                warnings.push(format!("MST block {} is not a map", cid));
                continue;
            }
        };
        // `l` — left subtree (optional).
        if let Some(Ipld::Link(child)) = map.get("l") {
            stack.push(*child);
        }
        // `e` — entries array.  Each entry: {k, p, v, t?}.
        if let Some(Ipld::List(entries)) = map.get("e") {
            for entry in entries {
                if let Ipld::Map(em) = entry {
                    if let Some(Ipld::Link(leaf)) = em.get("v") {
                        leaves.insert(*leaf);
                    }
                    if let Some(Ipld::Link(subtree)) = em.get("t") {
                        stack.push(*subtree);
                    }
                }
            }
        }
    }
    Ok(MstReachability {
        inner_blocks_visited: inner,
        leaves_reachable: leaves.iter().map(|c| c.to_string()).collect(),
        warnings,
    })
}

/// Convenience: given a full-repo CAR, verify that `target_leaves` are all
/// reachable from the signed commit's `data` root.
pub fn verify_leaves_in_signed_tree(
    car: &Car,
    target_leaves: &[Cid],
) -> Result<MstProofSummary, MstError> {
    let root = car.roots.first().copied().ok_or(MstError::NoDataFieldInCommit)?;
    let commit_bytes = car
        .by_cid
        .get(&root)
        .ok_or(MstError::RootBlockMissing(root))?
        .clone();
    // Verify the commit block's CID integrity too.
    let computed = cid_v1_dag_cbor(&commit_bytes);
    if computed != root {
        return Err(MstError::BlockCidMismatch {
            declared: root,
            computed,
        });
    }
    let data_root = extract_commit_data_root(&commit_bytes)?;
    let reach = walk_reachable_leaves(car, data_root)?;
    let reachable_set: HashSet<String> =
        reach.leaves_reachable.iter().cloned().collect();
    let mut absent = Vec::new();
    let mut present = Vec::new();
    for l in target_leaves {
        if reachable_set.contains(&l.to_string()) {
            present.push(l.to_string());
        } else {
            absent.push(l.to_string());
        }
    }
    Ok(MstProofSummary {
        commit_root_cid: root.to_string(),
        data_root_cid: data_root.to_string(),
        inner_blocks_visited: reach.inner_blocks_visited,
        total_reachable_leaves: reach.leaves_reachable.len(),
        target_leaves_present: present,
        target_leaves_absent: absent,
        walk_warnings: reach.warnings,
    })
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct MstProofSummary {
    pub commit_root_cid: String,
    pub data_root_cid: String,
    pub inner_blocks_visited: usize,
    pub total_reachable_leaves: usize,
    pub target_leaves_present: Vec<String>,
    pub target_leaves_absent: Vec<String>,
    pub walk_warnings: Vec<String>,
}
