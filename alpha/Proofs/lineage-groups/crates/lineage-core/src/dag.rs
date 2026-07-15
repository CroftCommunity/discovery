//! The lineage DAG: genesis-anchored branches connected by fork and recombine
//! edges, with standing queries decided from structure alone (invariant I3).
//!
//! "Knowing reliably where a conversation branched from" is the security and
//! the social-legibility primitive at once. Standing — "were you ever party to
//! a group on this lineage" — is answered here without trusting any party's
//! assertion: it follows from shared genesis ancestry plus recorded membership.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use crate::ids::{Did, GenesisId};

/// One node in the lineage: a branch identified by its own genesis, its
/// parent branches (0 for a root, 1 for a fork, 2 for a recombine /
/// fresh-genesis), and everyone who ever held standing on it.
#[derive(Debug, Clone)]
pub struct BranchNode {
    pub id: GenesisId,
    pub parents: Vec<GenesisId>,
    pub ever_members: BTreeSet<Did>,
}

/// The whole lineage forest/DAG.
#[derive(Debug, Default)]
pub struct Lineage {
    nodes: BTreeMap<GenesisId, BranchNode>,
}

impl Lineage {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a root branch (a genesis with no parents).
    pub fn add_root(&mut self, id: GenesisId, members: impl IntoIterator<Item = Did>) {
        self.nodes.insert(
            id,
            BranchNode {
                id,
                parents: Vec::new(),
                ever_members: members.into_iter().collect(),
            },
        );
    }

    /// Record a fork: `child` branches from `parent`.
    pub fn fork(&mut self, parent: GenesisId, child: GenesisId, members: impl IntoIterator<Item = Did>) {
        self.nodes.insert(
            child,
            BranchNode {
                id: child,
                parents: vec![parent],
                ever_members: members.into_iter().collect(),
            },
        );
    }

    /// Record a recombine / fresh-genesis: `child` descends from two parents
    /// (the "sixteenth-great-grandparent" path inherits both as ancestry).
    pub fn recombine(
        &mut self,
        a: GenesisId,
        b: GenesisId,
        child: GenesisId,
        members: impl IntoIterator<Item = Did>,
    ) {
        self.nodes.insert(
            child,
            BranchNode {
                id: child,
                parents: vec![a, b],
                ever_members: members.into_iter().collect(),
            },
        );
    }

    /// Note that `did` held standing on `branch` (e.g. after an Add).
    pub fn record_member(&mut self, branch: GenesisId, did: Did) {
        if let Some(node) = self.nodes.get_mut(&branch) {
            node.ever_members.insert(did);
        }
    }

    /// All ancestors of `branch` (including itself), walking every parent edge.
    fn ancestors(&self, branch: GenesisId) -> BTreeSet<GenesisId> {
        let mut seen = BTreeSet::new();
        let mut queue = VecDeque::from([branch]);
        while let Some(id) = queue.pop_front() {
            if !seen.insert(id) {
                continue;
            }
            if let Some(node) = self.nodes.get(&id) {
                for p in &node.parents {
                    queue.push_back(*p);
                }
            }
        }
        seen
    }

    /// The root genesis ids reachable from `branch`.
    pub fn roots(&self, branch: GenesisId) -> BTreeSet<GenesisId> {
        self.ancestors(branch)
            .into_iter()
            .filter(|id| self.nodes.get(id).is_some_and(|n| n.parents.is_empty()))
            .collect()
    }

    /// Do two branches share genesis ancestry? This is the privacy boundary
    /// for reconciliation/backfill (thesis §1): you may only be offered history
    /// from a lineage you share a root with.
    pub fn shares_lineage(&self, a: GenesisId, b: GenesisId) -> bool {
        let ra = self.roots(a);
        ra.intersection(&self.roots(b)).next().is_some()
    }

    /// Does `actor` have standing on `branch`'s lineage? True iff the actor was
    /// ever a member of any branch sharing a root with `branch`. Decided from
    /// signed/recorded data, never from the actor's assertion (I3).
    pub fn standing(&self, actor: &Did, branch: GenesisId) -> bool {
        let target_roots = self.roots(branch);
        self.nodes.values().any(|node| {
            node.ever_members.contains(actor)
                && self
                    .roots(node.id)
                    .intersection(&target_roots)
                    .next()
                    .is_some()
        })
    }
}
