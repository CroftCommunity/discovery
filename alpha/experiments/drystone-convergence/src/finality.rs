// Stage 4: quorum voting, ceiling, in-flight tally, now snapshot.
// NOTE: The Epoch/EnforcingCommit model below is NOT a real MLS epoch or key
// schedule. It is a reference placeholder that models the invariant that
// membership changes (AddMember, RemoveMember) require an out-of-band commit
// step before taking effect in the cryptographic group state. Fold-plane-only
// changes (GrantRole, RevokeRole, SetThreshold) do not require such a commit.

use std::collections::{BTreeMap, BTreeSet};
use sha2::{Sha256, Digest};
use crate::types::{AuthorId, AuthorityState, FactId, MemberId, Role, hash_str_h};

// ─────────────────────────────────────────────────────────────────────────────
// SlotTransition
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SlotTransition {
    RemoveMember(MemberId),
    AddMember(MemberId),
    GrantRole(MemberId, Role),
    RevokeRole(MemberId, Role),
    SetThreshold(Role, u32, u32),
}

// ─────────────────────────────────────────────────────────────────────────────
// transition_key — canonical string key for a SlotTransition
// ─────────────────────────────────────────────────────────────────────────────

pub fn transition_key(t: &SlotTransition) -> String {
    match t {
        SlotTransition::RemoveMember(m)       => format!("rm:{}", m),
        SlotTransition::AddMember(m)          => format!("add:{}", m),
        SlotTransition::GrantRole(m, r)       => format!("grant:{}:{}", m, r),
        SlotTransition::RevokeRole(m, r)      => format!("revoke:{}:{}", m, r),
        SlotTransition::SetThreshold(r, k, n) => format!("thresh:{}:{}:{}", r, k, n),
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Vote — a signed fact for a transition (separate from the Fact type)
// ─────────────────────────────────────────────────────────────────────────────

pub struct Vote {
    pub id:             FactId,
    pub author:         AuthorId,
    pub predecessors:   Vec<FactId>,
    pub for_transition: SlotTransition,
}

impl Vote {
    // Compute FactId via SHA-256("drystone-vote-v1\0" + author + counter +
    // sorted_predecessors + transition).
    pub fn new(
        author:       AuthorId,
        counter:      u64,
        predecessors: Vec<FactId>,
        transition:   SlotTransition,
    ) -> Self {
        let id = compute_vote_id(&author, counter, &predecessors, &transition);
        Vote { id, author, predecessors, for_transition: transition }
    }

    // Construct with an explicit synthetic FactId. Used in tests to supply
    // predetermined ids so the sort order and completing_vote are predictable.
    pub fn with_explicit_id(
        id:           FactId,
        author:       AuthorId,
        predecessors: Vec<FactId>,
        transition:   SlotTransition,
    ) -> Self {
        Vote { id, author, predecessors, for_transition: transition }
    }
}

fn compute_vote_id(
    author:       &str,
    counter:      u64,
    predecessors: &[FactId],
    transition:   &SlotTransition,
) -> FactId {
    let mut h = Sha256::new();
    h.update(b"drystone-vote-v1\x00");
    hash_str_h(&mut h, author);
    h.update(counter.to_le_bytes());
    let mut sorted = predecessors.to_vec();
    sorted.sort_unstable();
    h.update((sorted.len() as u64).to_le_bytes());
    for d in &sorted { h.update(d.0); }
    hash_transition_h(&mut h, transition);
    FactId(h.finalize().into())
}

fn hash_transition_h(h: &mut Sha256, t: &SlotTransition) {
    match t {
        SlotTransition::RemoveMember(m) => {
            h.update([0u8]);
            hash_str_h(h, m);
        }
        SlotTransition::AddMember(m) => {
            h.update([1u8]);
            hash_str_h(h, m);
        }
        SlotTransition::GrantRole(m, r) => {
            h.update([2u8]);
            hash_str_h(h, m);
            hash_str_h(h, r);
        }
        SlotTransition::RevokeRole(m, r) => {
            h.update([3u8]);
            hash_str_h(h, m);
            hash_str_h(h, r);
        }
        SlotTransition::SetThreshold(r, k, n) => {
            h.update([4u8]);
            hash_str_h(h, r);
            h.update(k.to_le_bytes());
            h.update(n.to_le_bytes());
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// QuorumResult
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuorumResult {
    Insufficient { count: u32, threshold: u32 },
    Crossed { completing_vote: FactId, votes: Vec<FactId> },
}

// ─────────────────────────────────────────────────────────────────────────────
// quorum_fold
// ─────────────────────────────────────────────────────────────────────────────

pub fn quorum_fold(
    votes:            &[Vote],
    transition:       &SlotTransition,
    threshold:        u32,
    eligible_members: &BTreeSet<MemberId>,
) -> QuorumResult {
    // Collect concordant votes from eligible members only.
    // Deduplicate by author: keep the highest FactId per author.
    let mut best: BTreeMap<String, FactId> = BTreeMap::new();
    for vote in votes {
        if &vote.for_transition == transition && eligible_members.contains(&vote.author) {
            let entry = best.entry(vote.author.clone()).or_insert(vote.id);
            if vote.id > *entry {
                *entry = vote.id;
            }
        }
    }

    let count = best.len() as u32;
    if count < threshold {
        return QuorumResult::Insufficient { count, threshold };
    }

    // Sort all winning ids descending; truncate to threshold.
    // completing_vote = the k-th entry (index threshold-1): the last id
    // needed to reach the threshold (canonically the minimum of the quorum).
    let mut all_ids: Vec<FactId> = best.into_values().collect();
    all_ids.sort_unstable_by(|a, b| b.cmp(a));
    all_ids.truncate(threshold as usize);
    let completing_vote = all_ids[(threshold as usize) - 1];

    QuorumResult::Crossed { completing_vote, votes: all_ids }
}

// ─────────────────────────────────────────────────────────────────────────────
// Ceiling — governance head at which a removed member's authority ends
// ─────────────────────────────────────────────────────────────────────────────

pub struct Ceiling {
    pub removed: MemberId,
    pub at_head: FactId,
    pub votes:   Vec<FactId>,
}

impl Ceiling {
    // Returns Some only when the quorum result is Crossed; None for Insufficient.
    // A sub-k enactment cannot produce a valid Ceiling, making it detectable
    // as a fork origin.
    pub fn stamp(removed: MemberId, result: QuorumResult) -> Option<Self> {
        match result {
            QuorumResult::Crossed { completing_vote, votes } =>
                Some(Ceiling { removed, at_head: completing_vote, votes }),
            QuorumResult::Insufficient { .. } => None,
        }
    }

    // Returns true iff the action occurred strictly after the removal ceiling:
    // the removed member had no authority at the time of that action.
    pub fn voids_action_at(&self, action_head: FactId) -> bool {
        action_head > self.at_head
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// InFlightTally
// ─────────────────────────────────────────────────────────────────────────────

pub struct InFlightTally {
    pub transition: SlotTransition,
    pub votes:      BTreeMap<AuthorId, FactId>,
    pub threshold:  u32,
    pub enacted:    bool,
}

impl InFlightTally {
    pub fn vote_count(&self) -> u32 {
        self.votes.len() as u32
    }

    pub fn is_crossed(&self) -> bool {
        self.vote_count() >= self.threshold
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Now — materialized current state
// ─────────────────────────────────────────────────────────────────────────────

pub struct Now {
    pub state:        AuthorityState,
    pub in_flight:    BTreeMap<String, InFlightTally>,
    pub head:         FactId,
    pub attestations: BTreeSet<AuthorId>,
}

impl Now {
    // Attestations are intentionally excluded: they are per-node metadata and
    // must not affect the convergence identity of a Now. Two nodes with the
    // same state, in_flight, and head produce identical fingerprints regardless
    // of how many attestations each has accumulated.
    pub fn fingerprint(&self) -> [u8; 32] {
        let mut h = Sha256::new();
        h.update(b"drystone-now-v1\x00");
        h.update(self.state.fingerprint());
        h.update(self.head.0);
        // BTreeMap iterates in sorted key order — deterministic regardless of
        // insertion order.
        h.update((self.in_flight.len() as u64).to_le_bytes());
        for (key, tally) in &self.in_flight {
            hash_str_h(&mut h, key);
            // tally.votes is also a BTreeMap, sorted by author.
            h.update((tally.votes.len() as u64).to_le_bytes());
            for (author, fact_id) in &tally.votes {
                hash_str_h(&mut h, author);
                h.update(fact_id.0);
            }
            h.update(tally.threshold.to_le_bytes());
            h.update([tally.enacted as u8]);
        }
        h.finalize().into()
    }

    pub fn attest(&mut self, member: AuthorId) {
        self.attestations.insert(member);
    }

    pub fn attestation_count(&self) -> usize {
        self.attestations.len()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Reference epoch model
// NOTE: NOT a real MLS epoch or key schedule. Epoch and EnforcingCommit are
// reference placeholders. The invariant they model: AddMember/RemoveMember
// transitions require an EnforcingCommit before they take effect
// cryptographically. GrantRole, RevokeRole, and SetThreshold are fold-plane
// changes that do not require such a commit.
// ─────────────────────────────────────────────────────────────────────────────

pub struct Epoch {
    pub id:          u64,
    pub state_after: AuthorityState,
}

pub struct EnforcingCommit {
    pub id:             FactId,
    pub for_transition: SlotTransition,
    pub epoch_id:       u64,
}

pub fn requires_enforcing_commit(transition: &SlotTransition) -> bool {
    matches!(
        transition,
        SlotTransition::AddMember(_) | SlotTransition::RemoveMember(_)
    )
}
