use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use sha2::{Sha256, Digest};

pub type MemberId = String;
pub type Role     = String;
pub type AuthorId = String;

/// A 32-byte identifier for a governance fact.
///
/// In production, FactId is a SHA-256 content hash. The test harness also
/// allows explicit synthetic ids (see `FactId::explicit`) for Property B,
/// which requires a causally-later fact with a smaller id — impossible if the
/// id is always derived from content. R1's correctness must hold for *any* id
/// assignment, which is precisely what makes explicit test ids legitimate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FactId(pub [u8; 32]);

impl FactId {
    /// Construct a synthetic FactId from a u64. Used only in tests.
    pub fn explicit(val: u64) -> Self {
        let mut bytes = [0u8; 32];
        bytes[..8].copy_from_slice(&val.to_le_bytes());
        FactId(bytes)
    }
}

impl fmt::Display for FactId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for b in &self.0[..8] { write!(f, "{:02x}", b)?; }
        Ok(())
    }
}

/// The payload of a governance fact.
///
/// Slot mapping:
/// | Payload             | Slot                  |
/// |---------------------|-----------------------|
/// | AddMember(m)        | member:m              |
/// | RemoveMember(m)     | member:m (+ revokes role:m:* at its causal position, R2) |
/// | GrantRole(m,r)      | role:m:r              |
/// | RevokeRole(m,r)     | role:m:r              |
/// | SetThreshold(r,k,n) | threshold:r           |
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FactPayload {
    AddMember(MemberId),
    RemoveMember(MemberId),
    GrantRole(MemberId, Role),
    RevokeRole(MemberId, Role),
    /// k-of-n threshold for `role`.
    SetThreshold(Role, u32, u32),
}

/// An immutable governance fact.
///
/// Facts form a DAG via `predecessors`: fact B listing A's id in its
/// predecessors means B was authored after A was observed. The fold derives
/// the causal (happens-before) order from this DAG. A referenced predecessor
/// absent from the fold set is a detected gap (R3).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Fact {
    pub id:           FactId,
    pub author:       AuthorId,
    pub counter:      u64,
    /// The frontier of facts the author had observed when authoring this fact.
    /// Required for R1 (causal order) and R3 (gap detection).
    pub predecessors: Vec<FactId>,
    pub payload:      FactPayload,
}

impl Fact {
    /// Construct a fact; FactId = SHA-256(author, counter, sorted predecessors, payload).
    pub fn new(author: AuthorId, counter: u64, predecessors: Vec<FactId>, payload: FactPayload) -> Self {
        let id = compute_fact_id(&author, counter, &predecessors, &payload);
        Fact { id, author, counter, predecessors, payload }
    }

    /// Construct a fact with an explicit synthetic FactId.
    ///
    /// Used in tests for Property B: R1 requires causal precedence to hold for
    /// any id assignment, so explicit non-hash ids directly exercise that guarantee.
    /// Never use in production; production always uses `new`.
    pub fn with_explicit_id(
        id: FactId,
        author: AuthorId,
        counter: u64,
        predecessors: Vec<FactId>,
        payload: FactPayload,
    ) -> Self {
        Fact { id, author, counter, predecessors, payload }
    }
}

fn compute_fact_id(author: &str, counter: u64, predecessors: &[FactId], payload: &FactPayload) -> FactId {
    let mut h = Sha256::new();
    h.update(b"drystone-fact-v1\x00");
    hash_str_h(&mut h, author);
    h.update(counter.to_le_bytes());
    let mut sorted = predecessors.to_vec();
    sorted.sort_unstable();
    h.update((sorted.len() as u64).to_le_bytes());
    for d in &sorted { h.update(d.0); }
    hash_payload_h(&mut h, payload);
    FactId(h.finalize().into())
}

pub(crate) fn hash_str_h(h: &mut Sha256, s: &str) {
    h.update((s.len() as u64).to_le_bytes());
    h.update(s.as_bytes());
}

fn hash_payload_h(h: &mut Sha256, p: &FactPayload) {
    match p {
        FactPayload::AddMember(m)          => { h.update([0]); hash_str_h(h, m); }
        FactPayload::RemoveMember(m)       => { h.update([1]); hash_str_h(h, m); }
        FactPayload::GrantRole(m, r)       => { h.update([2]); hash_str_h(h, m); hash_str_h(h, r); }
        FactPayload::RevokeRole(m, r)      => { h.update([3]); hash_str_h(h, m); hash_str_h(h, r); }
        FactPayload::SetThreshold(r, k, n) => {
            h.update([4]);
            hash_str_h(h, r);
            h.update(k.to_le_bytes());
            h.update(n.to_le_bytes());
        }
    }
}

/// The folded governance authority state.
///
/// `effective_roles` is the R2 projection: (m, r) is here iff role:m:r
/// resolved to granted AND member:m resolved to member. Computed once on the
/// final resolved slots, never incrementally.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorityState {
    pub members:         BTreeSet<MemberId>,
    pub effective_roles: BTreeSet<(MemberId, Role)>,
    pub thresholds:      BTreeMap<Role, (u32, u32)>,
}

impl AuthorityState {
    pub fn empty() -> Self {
        AuthorityState {
            members:         BTreeSet::new(),
            effective_roles: BTreeSet::new(),
            thresholds:      BTreeMap::new(),
        }
    }

    /// Canonical SHA-256 fingerprint. Two states are identical iff fingerprints match.
    pub fn fingerprint(&self) -> [u8; 32] {
        let mut h = Sha256::new();
        h.update(b"drystone-state-v2\x00");

        h.update((self.members.len() as u64).to_le_bytes());
        for m in &self.members { hash_str_h(&mut h, m); }

        h.update((self.effective_roles.len() as u64).to_le_bytes());
        for (m, r) in &self.effective_roles {
            hash_str_h(&mut h, m);
            hash_str_h(&mut h, r);
        }

        h.update((self.thresholds.len() as u64).to_le_bytes());
        for (r, (k, n)) in &self.thresholds {
            hash_str_h(&mut h, r);
            h.update(k.to_le_bytes());
            h.update(n.to_le_bytes());
        }

        h.finalize().into()
    }
}
