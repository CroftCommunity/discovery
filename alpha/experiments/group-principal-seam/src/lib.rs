//! `group-principal-seam` — the §2e seam spike (RUN-11 Part 3).
//!
//! A `Design`-grade model of the group-principal seam the RUN-10 brief
//! (`beta/impl/drystone-design/group-principal-seam.md`) recommended: the Group
//! principal is a Meadowcap **communal namespace** identified by the genesis
//! hash `H(tag ‖ group_id)`; each persona is a **self-authorizing subspace**
//! (write authority is the persona's own signature, no grant); read is the
//! fold-gated asset key (resolved elsewhere, not modeled here); and authority to
//! issue capabilities is a **folded Group Role**. Because Meadowcap has no native
//! revocation, an authority change never overwrites a held capability
//! in-place — the governance fold advances an **authority generation** and
//! capabilities are **re-issued** to the folded set, so a stale capability is
//! *superseded*, never mutated.
//!
//! ## Scope (the firewall / scope wall)
//! Every binding here is `Design`-grade. No trust tier is decided: who-may-remove
//! is an *input* governance fact (`Governance::remove`), never chosen by this
//! crate (I9). No wire/byte encoding is pinned: the [`GroupPrincipal`] and
//! [`SubspaceId`] content-addresses use BLAKE3 over a **test-only** byte layout,
//! explicitly NOT the `[gates-release]` canonical encoding (Appendix B / E.1). No
//! MLS internals are touched — the leaf→lineage subspace fold is *modeled* from
//! lineage names (the fold itself is inherited, §4.5).

#![warn(missing_docs)]

use std::collections::{BTreeMap, BTreeSet};

/// The Group principal: a Meadowcap communal namespace identified by the genesis
/// hash `H(tag ‖ group_id)`. There is no whole-namespace secret (F1); this is a
/// public identifier, stable across all churn.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupPrincipal {
    namespace_id: [u8; 32],
}

impl GroupPrincipal {
    /// Derive the communal-namespace id as `H(tag ‖ group_id)`. Deterministic and
    /// stable: the same `(tag, group_id)` always yields the same id, and it never
    /// rotates under membership change. The byte layout is test-only, not the
    /// `[gates-release]` canonical encoding.
    #[must_use]
    pub fn of(tag: &str, group_id: &str) -> Self {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"drystone-group-principal:");
        hasher.update(tag.as_bytes());
        hasher.update(b"\x00");
        hasher.update(group_id.as_bytes());
        Self {
            namespace_id: *hasher.finalize().as_bytes(),
        }
    }

    /// The raw namespace-id bytes (for binding a capability to its Group).
    #[must_use]
    pub fn namespace_id(&self) -> &[u8] {
        &self.namespace_id
    }
}

/// A persona's subspace identity — its lineage identity, folded from the
/// persona's several device leaves to one identity (E.1). Content-addressed with
/// BLAKE3 over a test-only layout (the canonical `SubspaceId` encoding is
/// `[gates-release]`).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SubspaceId {
    id: [u8; 32],
}

impl SubspaceId {
    /// Fold a persona's device leaves (in any order) plus its lineage name to one
    /// subspace identity. The fold is order-independent (leaves are sorted), the
    /// prerequisite for every member computing the same subspace id.
    #[must_use]
    pub fn from_leaves(leaves: &[&str], lineage: &str) -> Self {
        let mut sorted: Vec<&str> = leaves.to_vec();
        sorted.sort_unstable();
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"drystone-subspace:");
        hasher.update(lineage.as_bytes());
        for leaf in sorted {
            hasher.update(b"\x00");
            hasher.update(leaf.as_bytes());
        }
        Self {
            id: *hasher.finalize().as_bytes(),
        }
    }

    /// The persona's own write key (modeled): a communal subspace's write
    /// authority *is* ownership of its subspace key pair, so the own key is
    /// derived from the subspace id itself.
    #[must_use]
    pub fn own_key(&self) -> AuthorKey {
        AuthorKey { key: self.id }
    }
}

/// A modeled write-authority key. In a communal namespace, authority derives
/// from ownership of the subspace key pair; this stands in for that key pair.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorKey {
    key: [u8; 32],
}

/// A write into a subspace, carrying the author's key. Self-authorizing iff the
/// author key is the subspace's own key (communal, per-subspace authority).
#[derive(Debug, Clone)]
pub struct Write {
    subspace: SubspaceId,
    author: AuthorKey,
    #[allow(dead_code)]
    payload: Vec<u8>,
}

impl Write {
    /// Author a write into `subspace` signed by `author`.
    #[must_use]
    pub fn authored(subspace: &SubspaceId, author: &AuthorKey, payload: &[u8]) -> Self {
        Self {
            subspace: subspace.clone(),
            author: author.clone(),
            payload: payload.to_vec(),
        }
    }

    /// Whether this write is self-authorized: the author holds the subspace's own
    /// key. No Group Role grant is consulted — authority is per-subspace ownership.
    #[must_use]
    pub fn is_self_authorized(&self) -> bool {
        self.author == self.subspace.own_key()
    }
}

/// A capability issued downstream of the folded Group Role, bound to its Group's
/// namespace and stamped with the governance generation at which it was issued.
/// Revocation is by supersession (an authority-generation advance), never by
/// mutating this object.
#[derive(Debug, Clone)]
pub struct Capability {
    namespace_id: [u8; 32],
    subspace: SubspaceId,
    issued_at_generation: u64,
}

impl Capability {
    /// The governance generation this capability was issued at. A later
    /// authority-generation advance supersedes it (R4's window is in generations,
    /// never wall-clock).
    #[must_use]
    pub fn issued_at_generation(&self) -> u64 {
        self.issued_at_generation
    }
}

/// The folded governance state of one member's view: the surviving member set,
/// which subspaces hold the Group Role, the governance generation counter, and
/// the per-subspace authority generation (the re-issue point). Two members
/// folding the same facts reach the identical state.
#[derive(Debug, Clone)]
pub struct Governance {
    namespace_id: [u8; 32],
    members: BTreeSet<SubspaceId>,
    roles: BTreeSet<SubspaceId>,
    generation: u64,
    authority_generation: BTreeMap<SubspaceId, u64>,
}

impl Governance {
    /// Genesis: the founding personae are the initial members, at generation 0,
    /// each with authority generation 0. Roles are granted separately.
    #[must_use]
    pub fn genesis(group: &GroupPrincipal, founders: &[SubspaceId]) -> Self {
        let members: BTreeSet<SubspaceId> = founders.iter().cloned().collect();
        let authority_generation = members.iter().map(|s| (s.clone(), 0)).collect();
        Self {
            namespace_id: group.namespace_id,
            members,
            roles: BTreeSet::new(),
            generation: 0,
            authority_generation,
        }
    }

    /// Grant the folded Group Role to a member (governance fact).
    pub fn grant_role(&mut self, subspace: &SubspaceId) {
        if self.members.contains(subspace) {
            self.roles.insert(subspace.clone());
        }
    }

    /// The current surviving member set.
    #[must_use]
    pub fn members(&self) -> &BTreeSet<SubspaceId> {
        &self.members
    }

    /// The current governance generation.
    #[must_use]
    pub fn generation(&self) -> u64 {
        self.generation
    }

    /// Remove a member as a governance fact — the ONLY revocation path (Meadowcap
    /// has no native revocation, and the owned-namespace overwrite trick is
    /// structurally unavailable to a communal principal). The authority
    /// generation advances and every surviving member's re-issue point moves to
    /// it, so pre-change capabilities are superseded, never overwritten. Whether
    /// this removal is *authorized* is a separate trust-tier question (I9) not
    /// decided here — this models the enactment of an already-decided fact.
    pub fn remove(&mut self, subject: &SubspaceId) {
        self.generation += 1;
        self.members.remove(subject);
        self.roles.remove(subject);
        self.authority_generation.remove(subject);
        for member in &self.members {
            self.authority_generation
                .insert(member.clone(), self.generation);
        }
    }

    /// Issue a capability to `subspace` at the current generation — but only if
    /// it holds the folded Group Role (issuance is downstream of the fold).
    #[must_use]
    pub fn issue(&self, subspace: &SubspaceId) -> Option<Capability> {
        if self.members.contains(subspace) && self.roles.contains(subspace) {
            Some(self.mint(subspace))
        } else {
            None
        }
    }

    /// Mint a capability without the Role check — for exercising a stale
    /// capability minted against a prior state.
    #[must_use]
    pub fn issue_unchecked(&self, subspace: &SubspaceId) -> Capability {
        self.mint(subspace)
    }

    fn mint(&self, subspace: &SubspaceId) -> Capability {
        Capability {
            namespace_id: self.namespace_id,
            subspace: subspace.clone(),
            issued_at_generation: self.generation,
        }
    }

    /// A capability is valid iff it is bound to this Group, its subspace is still
    /// in the folded member set, and it was issued at or after that subspace's
    /// current authority generation (so a superseded pre-change capability fails).
    #[must_use]
    pub fn capability_valid(&self, cap: &Capability) -> bool {
        cap.namespace_id == self.namespace_id
            && self.members.contains(&cap.subspace)
            && self
                .authority_generation
                .get(&cap.subspace)
                .is_some_and(|&floor| cap.issued_at_generation >= floor)
    }
}
