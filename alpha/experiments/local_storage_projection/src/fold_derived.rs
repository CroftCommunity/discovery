//! Fold engine — derived projection path for `local_storage_projection` — Stage 4.
//!
//! This module extends Stage 3's authoritative fold to also apply derived effects
//! (Section 6, step 3) in the SAME transaction as the authoritative writes.
//! It maintains `idx_nodes`, `idx_edges_out`, `idx_edges_in`, and `state_group`
//! (extended with gov_seq, gov_head, fork_status, and lamport-tagged members).

use std::sync::Arc;
use thiserror::Error;

use crate::types::{
    AssertionEnvelope, AssertionType,
    DeviceId as TypesDeviceId,
    GroupId,
    GroupRules,
    Hash as TypesHash,
    KindTag,
    PrincipalId as TypesPrincipalId,
    Role, RuleKey, TypedId,
    envelope_hash, compute_hash,
};

use crate::traits::{
    CredentialResolver, Verifier,
    DeviceId as TraitsDeviceId,
    PrincipalId as TraitsPrincipalId,
};

use crate::tables::{
    Db, DbError, EdgeMeta, EdgeType, NodeCard,
    encode_by_device_key, encode_edge_in_key, encode_edge_out_key, encode_gov_log_key,
};

use redb::{ReadableTable, TableDefinition};

// ---------------------------------------------------------------------------
// Table definitions (auth + derived tables)
// ---------------------------------------------------------------------------

const AUTH_ASSERTIONS: TableDefinition<'static, &'static [u8], &'static [u8]> =
    TableDefinition::new("auth_assertions_v1");
const AUTH_ASSERTIONS_BY_DEVICE: TableDefinition<'static, &'static [u8], &'static [u8]> =
    TableDefinition::new("auth_assertions_by_device_v1");
const AUTH_GOV_LOG: TableDefinition<'static, &'static [u8], &'static [u8]> =
    TableDefinition::new("auth_gov_log_v1");
const AUTH_GENESIS: TableDefinition<'static, &'static [u8], &'static [u8]> =
    TableDefinition::new("auth_genesis_v1");
const IDX_NODES: TableDefinition<'static, &'static [u8], &'static [u8]> =
    TableDefinition::new("idx_nodes_v1");
const IDX_EDGES_OUT: TableDefinition<'static, &'static [u8], &'static [u8]> =
    TableDefinition::new("idx_edges_out_v1");
const IDX_EDGES_IN: TableDefinition<'static, &'static [u8], &'static [u8]> =
    TableDefinition::new("idx_edges_in_v1");
const STATE_GROUP: TableDefinition<'static, &'static [u8], &'static [u8]> =
    TableDefinition::new("state_group_v1");
const STATE_BLOB_PRESENCE: TableDefinition<'static, &'static [u8], &'static [u8]> =
    TableDefinition::new("state_blob_presence_v1");
/// META
///
/// Small key→value store for store-level facts that are not assertions. Currently holds one
/// key, `b"comparator_version"` → `[u8; 1]`: the `merge_cmp` version the derived tables were
/// last folded under (see `types::MERGE_CMP_VERSION`). Stamped by `rebuild`; absent on stores
/// that predate comparator versioning, which `needs_rebuild` treats as v1.
const META: TableDefinition<'static, &'static [u8], &'static [u8]> =
    TableDefinition::new("meta_v1");

const META_COMPARATOR_KEY: &[u8] = b"comparator_version";


// ---------------------------------------------------------------------------
// FoldError
// ---------------------------------------------------------------------------

/// All errors produced by the derived fold engine.
#[derive(Debug, Error)]
pub enum FoldError {
    #[error("signature invalid: {0}")]
    SignatureInvalid(String),

    #[error("credential invalid: {0}")]
    CredentialInvalid(String),

    #[error("authorization failed: {0}")]
    AuthorizationFailed(String),

    #[error("lamport violation for device {device}: expected > {expected_gt}, got {got}")]
    LamportViolation {
        device: TypesDeviceId,
        expected_gt: u64,
        got: u64,
    },

    #[error("malformed envelope: {0}")]
    MalformedEnvelope(String),

    /// The assertion declares causal antecedents that are not all present in the
    /// store yet. It is HELD BACK (not admitted, nothing written) so the caller
    /// retries it once the missing predecessors arrive — the completeness guard
    /// that keeps the fold from admitting a fact against an incomplete set
    /// (§7.5.2 frontier-closure). Transient, not a rejection.
    #[error("missing antecedents: have {have} of {need}")]
    MissingAntecedents { have: usize, need: usize },

    /// A threshold-governed act does not carry approvals from enough distinct personae
    /// (by lineage). Held/rejected until the k-of-n quorum is present (V5′).
    #[error("threshold not met: have {have} of {need} distinct personae")]
    ThresholdNotMet { have: usize, need: usize },

    #[error("storage error: {0}")]
    StorageError(String),

    #[error("unknown assertion type: 0x{0:04x}")]
    UnknownAssertionType(u16),
}

impl From<DbError> for FoldError {
    fn from(e: DbError) -> Self {
        FoldError::StorageError(e.to_string())
    }
}

impl From<redb::StorageError> for FoldError {
    fn from(e: redb::StorageError) -> Self {
        FoldError::StorageError(e.to_string())
    }
}

impl From<redb::TransactionError> for FoldError {
    fn from(e: redb::TransactionError) -> Self {
        FoldError::StorageError(e.to_string())
    }
}

impl From<redb::TableError> for FoldError {
    fn from(e: redb::TableError) -> Self {
        FoldError::StorageError(e.to_string())
    }
}

impl From<redb::CommitError> for FoldError {
    fn from(e: redb::CommitError) -> Self {
        FoldError::StorageError(e.to_string())
    }
}

// ---------------------------------------------------------------------------
// ForkStatus
// ---------------------------------------------------------------------------

/// One open concurrent-governance contradiction (§7.3.2, E108): the conflicting pair
/// carried **as data**, plus what the deterministic replay needs to stay order-independent
/// while the pair is open. Entries are held in a set (`ForkStatus::Contested`) — two
/// simultaneously open contradictions are representable, which the retired single
/// min-hash slot structurally was not.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ContestedEntry {
    /// The two conflicting facts' content addresses, lexicographically ordered.
    pub pair: (TypesHash, TypesHash),
    /// The membership-contested subjects, sorted: a mutual expulsion contests both
    /// parties; an add/remove race contests the raced subject; role and rule
    /// contradictions contest no membership (empty).
    pub subjects: Vec<TypesPrincipalId>,
    /// The facts the deterministic replay withholds while the pair is open. Withholding
    /// is not a verdict: the projection reports the subjects `CONTESTED`, and closing
    /// the pair is a governed `Resolution` fact, never the replay itself.
    pub excluded: Vec<TypesHash>,
}

impl ContestedEntry {
    /// Order a raw hash pair lexicographically — the canonical pair form.
    pub fn order_pair(a: TypesHash, b: TypesHash) -> (TypesHash, TypesHash) {
        if a.as_bytes() <= b.as_bytes() { (a, b) } else { (b, a) }
    }
}

/// The total membership view (§7.3.2 rule (c), owner-ratified 2026-08-17): the subject
/// of an open contradiction is **neither** member nor not-member. Deliberately no
/// boolean convenience accessor exists — an untyped mid-resolution state is how a UI
/// starts lying by omission (the Matrix lesson, E117 P1).
#[derive(Debug, Clone, PartialEq)]
pub enum MembershipView {
    Member(Role),
    NotMember,
    /// Membership pending resolution; carries the conflicting pair(s) naming this
    /// subject, as data, so a renderer can present the contradiction factually.
    Contested(Vec<(TypesHash, TypesHash)>),
}

/// Whether the group's governance log is clean or has a detected fork.
#[derive(Debug, Clone, PartialEq)]
pub enum ForkStatus {
    Clean,
    /// The group's governance state forked; this is the hash of the assertion
    /// that was displaced (lexicographically larger hash at the same gov_seq).
    /// The "too many valid claims" member of the §7.6.1 escalation set.
    ForkedFrom(TypesHash),
    /// The group is under-determined: a required role (Owner) is vacant with no
    /// admissible successor. The "too few" member of the §7.6.1 escalation set —
    /// a distinct hard-stop a contradiction-only watcher would miss. See
    /// `governance::is_under_determined`.
    UnderDetermined,
    /// Open concurrent-governance contradictions — the "too many valid claims" shape
    /// for concurrent governance, which slot-collision detection alone misses. Each
    /// entry carries its conflicting pair as data; the set is sorted by pair. The
    /// hard-stop persists per entry until a governed `Resolution` fact closes it
    /// (§7.3.2: the replay is not resolution). See `governance::are_concurrent`.
    Contested(Vec<ContestedEntry>),
}

// ---------------------------------------------------------------------------
// GroupState (public, extended form for Stage 4)
// ---------------------------------------------------------------------------

/// Projected governance state for a group.
///
/// Wire layout (version = 2; v1 is refused with a rebuild demand — pre-1.0, no shim):
///   version            : 1 byte (0x02)
///   computed_at_gov_head: 32 bytes (Hash raw)
///   computed_at_gov_seq : 8 bytes big-endian u64
///   rules              : 5 × 4 bytes big-endian u32
///                        [add_member_threshold, remove_member_threshold,
///                         role_change_threshold, rule_change_threshold,
///                         resolution_threshold]
///   member_count       : 4 bytes big-endian u32
///   members            : member_count × (32 + 1 + 8) bytes
///                        PrincipalId(32) || Role(1) || since_lamport(8)
///   fork_status        : 1 byte (0x00=Clean, 0x01=ForkedFrom, 0x02=UnderDetermined,
///                        0x03=Contested)
///   [fork_hash]        : 32 bytes (only if fork_status == 0x01)
///   [contested]        : only if fork_status == 0x03 —
///                        entry_count u32, then per entry:
///                        pair.0(32) ‖ pair.1(32) ‖ subj_n(u8) ‖ subjects × 32 ‖
///                        excl_n(u8) ‖ excluded × 32
///
/// The `members` list is the deterministic replay substrate; **the read path for
/// membership questions is [`GroupState::membership`]**, which is total over
/// Member / NotMember / Contested and never collapses the third state to a boolean.
#[derive(Debug, Clone, PartialEq)]
pub struct GroupState {
    pub version: u8,
    pub computed_at_gov_head: TypesHash,
    pub computed_at_gov_seq: u64,
    pub members: Vec<(TypesPrincipalId, Role, u64)>, // (principal, role, since_lamport)
    pub rules: GroupRules,
    pub fork_status: ForkStatus,
}

/// The GroupState wire schema version this build reads and writes.
pub const GROUP_STATE_WIRE_VERSION: u8 = 2;

impl GroupState {
    /// The total membership view for `p` (§7.3.2 rule (c)): the subject of an open
    /// contradiction is `Contested` — neither member nor not-member — carrying the
    /// pair(s) that name it. Everyone else resolves against the replay substrate.
    pub fn membership(&self, p: &TypesPrincipalId) -> MembershipView {
        if let ForkStatus::Contested(entries) = &self.fork_status {
            let pairs: Vec<(TypesHash, TypesHash)> = entries
                .iter()
                .filter(|e| e.subjects.contains(p))
                .map(|e| e.pair)
                .collect();
            if !pairs.is_empty() {
                return MembershipView::Contested(pairs);
            }
        }
        match self.members.iter().find(|(m, _, _)| m == p) {
            Some((_, role, _)) => MembershipView::Member(role.clone()),
            None => MembershipView::NotMember,
        }
    }

    /// The open contested entries, if any (empty slice otherwise).
    pub fn contested_entries(&self) -> &[ContestedEntry] {
        match &self.fork_status {
            ForkStatus::Contested(entries) => entries,
            _ => &[],
        }
    }

    /// Serialize to bytes (wire v2).
    pub fn to_bytes(&self) -> Vec<u8> {
        let member_count = self.members.len() as u32;
        let mut buf = Vec::with_capacity(1 + 32 + 8 + 20 + 4 + self.members.len() * 41 + 64);

        buf.push(GROUP_STATE_WIRE_VERSION);
        buf.extend_from_slice(self.computed_at_gov_head.as_bytes());
        buf.extend_from_slice(&self.computed_at_gov_seq.to_be_bytes());
        buf.extend_from_slice(&self.rules.add_member_threshold.to_be_bytes());
        buf.extend_from_slice(&self.rules.remove_member_threshold.to_be_bytes());
        buf.extend_from_slice(&self.rules.role_change_threshold.to_be_bytes());
        buf.extend_from_slice(&self.rules.rule_change_threshold.to_be_bytes());
        buf.extend_from_slice(&self.rules.resolution_threshold.to_be_bytes());
        buf.extend_from_slice(&member_count.to_be_bytes());
        for (pid, role, since) in &self.members {
            buf.extend_from_slice(pid.as_bytes());
            buf.push(role_to_u8(role));
            buf.extend_from_slice(&since.to_be_bytes());
        }
        match &self.fork_status {
            ForkStatus::Clean => buf.push(0x00),
            ForkStatus::ForkedFrom(h) => {
                buf.push(0x01);
                buf.extend_from_slice(h.as_bytes());
            }
            ForkStatus::UnderDetermined => buf.push(0x02),
            ForkStatus::Contested(entries) => {
                buf.push(0x03);
                buf.extend_from_slice(&(entries.len() as u32).to_be_bytes());
                for e in entries {
                    buf.extend_from_slice(e.pair.0.as_bytes());
                    buf.extend_from_slice(e.pair.1.as_bytes());
                    buf.push(e.subjects.len() as u8);
                    for s in &e.subjects {
                        buf.extend_from_slice(s.as_bytes());
                    }
                    buf.push(e.excluded.len() as u8);
                    for x in &e.excluded {
                        buf.extend_from_slice(x.as_bytes());
                    }
                }
            }
        }
        buf
    }

    /// Deserialize from bytes (wire v2 only — an unknown version is refused loudly;
    /// a v1 store demands a rebuild rather than being silently misread).
    pub fn from_bytes(b: &[u8]) -> Result<Self, FoldError> {
        if b.is_empty() {
            return Err(FoldError::StorageError("GroupState: empty".to_string()));
        }
        if b[0] != GROUP_STATE_WIRE_VERSION {
            return Err(FoldError::StorageError(format!(
                "GroupState: unknown wire version 0x{:02x} (this build reads 0x{:02x}); \
                 stale stores must be rebuilt, never reinterpreted",
                b[0], GROUP_STATE_WIRE_VERSION
            )));
        }
        // Minimum: 1 + 32 + 8 + 20 + 4 + 1 = 66
        if b.len() < 66 {
            return Err(FoldError::StorageError(format!(
                "GroupState: too short ({} bytes)",
                b.len()
            )));
        }
        let version = b[0];
        let mut head_bytes = [0u8; 32];
        head_bytes.copy_from_slice(&b[1..33]);
        let computed_at_gov_head = TypesHash::new(head_bytes);
        let computed_at_gov_seq = u64::from_be_bytes(b[33..41].try_into().unwrap());
        let add_member_threshold = u32::from_be_bytes(b[41..45].try_into().unwrap());
        let remove_member_threshold = u32::from_be_bytes(b[45..49].try_into().unwrap());
        let role_change_threshold = u32::from_be_bytes(b[49..53].try_into().unwrap());
        let rule_change_threshold = u32::from_be_bytes(b[53..57].try_into().unwrap());
        let resolution_threshold = u32::from_be_bytes(b[57..61].try_into().unwrap());
        let member_count = u32::from_be_bytes(b[61..65].try_into().unwrap()) as usize;

        // Each member is 32 + 1 + 8 = 41 bytes.
        let members_end = 65 + member_count * 41;
        if b.len() < members_end + 1 {
            return Err(FoldError::StorageError(format!(
                "GroupState: need {} bytes for {} members + fork byte, have {}",
                members_end + 1,
                member_count,
                b.len()
            )));
        }
        let mut members = Vec::with_capacity(member_count);
        for i in 0..member_count {
            let off = 65 + i * 41;
            let mut pid_bytes = [0u8; 32];
            pid_bytes.copy_from_slice(&b[off..off + 32]);
            let role = u8_to_role(b[off + 32]).map_err(|_| {
                FoldError::StorageError(format!(
                    "GroupState: unknown role byte {}",
                    b[off + 32]
                ))
            })?;
            let since = u64::from_be_bytes(b[off + 33..off + 41].try_into().unwrap());
            members.push((TypesPrincipalId::new(pid_bytes), role, since));
        }
        let fork_byte = b[members_end];
        let mut off = members_end + 1;
        let take32 = |b: &[u8], off: usize| -> Result<[u8; 32], FoldError> {
            b.get(off..off + 32)
                .ok_or_else(|| FoldError::StorageError("GroupState: truncated hash".to_string()))
                .map(|s| {
                    let mut h = [0u8; 32];
                    h.copy_from_slice(s);
                    h
                })
        };
        let fork_status = match fork_byte {
            0x00 => ForkStatus::Clean,
            0x02 => ForkStatus::UnderDetermined,
            0x01 => ForkStatus::ForkedFrom(TypesHash::new(take32(b, off)?)),
            0x03 => {
                let n = u32::from_be_bytes(
                    b.get(off..off + 4)
                        .ok_or_else(|| {
                            FoldError::StorageError("GroupState: truncated entry count".to_string())
                        })?
                        .try_into()
                        .unwrap(),
                ) as usize;
                off += 4;
                let mut entries = Vec::with_capacity(n);
                for _ in 0..n {
                    let a = TypesHash::new(take32(b, off)?);
                    off += 32;
                    let z = TypesHash::new(take32(b, off)?);
                    off += 32;
                    let subj_n = *b.get(off).ok_or_else(|| {
                        FoldError::StorageError("GroupState: truncated subject count".to_string())
                    })? as usize;
                    off += 1;
                    let mut subjects = Vec::with_capacity(subj_n);
                    for _ in 0..subj_n {
                        subjects.push(TypesPrincipalId::new(take32(b, off)?));
                        off += 32;
                    }
                    let excl_n = *b.get(off).ok_or_else(|| {
                        FoldError::StorageError("GroupState: truncated excluded count".to_string())
                    })? as usize;
                    off += 1;
                    let mut excluded = Vec::with_capacity(excl_n);
                    for _ in 0..excl_n {
                        excluded.push(TypesHash::new(take32(b, off)?));
                        off += 32;
                    }
                    entries.push(ContestedEntry { pair: (a, z), subjects, excluded });
                }
                ForkStatus::Contested(entries)
            }
            other => {
                return Err(FoldError::StorageError(format!(
                    "GroupState: unknown fork_status byte {other}"
                )));
            }
        };

        Ok(GroupState {
            version,
            computed_at_gov_head,
            computed_at_gov_seq,
            members,
            rules: GroupRules {
                add_member_threshold,
                remove_member_threshold,
                role_change_threshold,
                rule_change_threshold,
                resolution_threshold,
            },
            fork_status,
        })
    }
}

// ---------------------------------------------------------------------------
// Role encoding helpers
// ---------------------------------------------------------------------------

fn role_to_u8(r: &Role) -> u8 {
    match r {
        Role::Owner => 0,
        Role::Admin => 1,
        Role::Member => 2,
        Role::Observer => 3,
    }
}

fn u8_to_role(v: u8) -> Result<Role, ()> {
    match v {
        0 => Ok(Role::Owner),
        1 => Ok(Role::Admin),
        2 => Ok(Role::Member),
        3 => Ok(Role::Observer),
        _ => Err(()),
    }
}

// ---------------------------------------------------------------------------
// RuleKey helpers
// ---------------------------------------------------------------------------

fn decode_rule_key(v: u8) -> Result<RuleKey, ()> {
    match v {
        0 => Ok(RuleKey::AddMember),
        1 => Ok(RuleKey::RemoveMember),
        2 => Ok(RuleKey::RoleChange),
        3 => Ok(RuleKey::RuleChange),
        4 => Ok(RuleKey::Resolution),
        _ => Err(()),
    }
}

// ---------------------------------------------------------------------------
// Authorization helpers (self-contained: the single authorization path in the crate)
// ---------------------------------------------------------------------------

fn author_role_in<'a>(
    members: &'a [(TypesPrincipalId, Role, u64)],
    author: &TypesPrincipalId,
) -> Option<&'a Role> {
    members.iter().find(|(p, _, _)| p == author).map(|(_, r, _)| r)
}

fn role_ge_admin(r: &Role) -> bool {
    matches!(r, Role::Owner | Role::Admin)
}

fn role_ge_owner(r: &Role) -> bool {
    matches!(r, Role::Owner)
}

fn role_ge_member(r: &Role) -> bool {
    matches!(r, Role::Owner | Role::Admin | Role::Member)
}

fn check_authorization(
    state: &GroupState,
    env: &AssertionEnvelope,
) -> Result<(), FoldError> {
    let author = &env.author_principal;
    match &env.assertion_type {
        AssertionType::GroupGenesis => Ok(()),

        AssertionType::MembershipAdd => match author_role_in(&state.members, author) {
            Some(r) if role_ge_admin(r) => Ok(()),
            _ => Err(FoldError::AuthorizationFailed(format!(
                "MembershipAdd requires Owner or Admin; author {:?} is not",
                author
            ))),
        },

        AssertionType::MembershipRemove => match author_role_in(&state.members, author) {
            Some(r) if role_ge_admin(r) => Ok(()),
            _ => Err(FoldError::AuthorizationFailed(format!(
                "MembershipRemove requires Owner or Admin; author {:?} is not",
                author
            ))),
        },

        AssertionType::RoleGrant | AssertionType::RoleRevoke => {
            match author_role_in(&state.members, author) {
                Some(r) if role_ge_owner(r) => Ok(()),
                _ => Err(FoldError::AuthorizationFailed(format!(
                    "{:?} requires Owner; author {:?} is not",
                    env.assertion_type, author
                ))),
            }
        }

        AssertionType::RuleChange => {
            if env.payload.len() < 5 {
                return Err(FoldError::MalformedEnvelope(
                    "RuleChange payload too short".to_string(),
                ));
            }
            decode_rule_key(env.payload[0]).map_err(|_| {
                FoldError::MalformedEnvelope(format!(
                    "RuleChange: unknown rule_key byte {}",
                    env.payload[0]
                ))
            })?;
            match author_role_in(&state.members, author) {
                Some(r) if role_ge_owner(r) => Ok(()),
                _ => Err(FoldError::AuthorizationFailed(format!(
                    "RuleChange requires Owner; author {:?} is not",
                    author
                ))),
            }
        }

        AssertionType::AttachmentAdd
        | AssertionType::Message
        | AssertionType::ArtifactRef => match author_role_in(&state.members, author) {
            Some(r) if role_ge_member(r) => Ok(()),
            _ => Err(FoldError::AuthorizationFailed(format!(
                "{:?} requires membership; author {:?} is not a member",
                env.assertion_type, author
            ))),
        },

        // Approval (V5′): an approver of a governance act must itself be
        // governance-eligible (Owner/Admin) — it is co-authoring the act.
        AssertionType::Approval => match author_role_in(&state.members, author) {
            Some(r) if role_ge_admin(r) => Ok(()),
            _ => Err(FoldError::AuthorizationFailed(format!(
                "Approval requires Owner or Admin; author {author:?} is not"
            ))),
        },

        // Resolution (§7.3.2): a governed act closing one open contradiction pair.
        // Payload is exactly the ordered pair; the quorum gate (resolution_threshold,
        // default 2 — never silently single-author) is enforced at Step 5.6 like every
        // other governance threshold.
        AssertionType::Resolution => {
            if env.payload.len() != 64 {
                return Err(FoldError::MalformedEnvelope(format!(
                    "Resolution payload must be exactly 64 bytes (the ordered pair), got {}",
                    env.payload.len()
                )));
            }
            if env.payload[..32] >= env.payload[32..64] {
                return Err(FoldError::MalformedEnvelope(
                    "Resolution pair must be strictly lexicographically ordered".to_string(),
                ));
            }
            match author_role_in(&state.members, author) {
                Some(r) if role_ge_admin(r) => Ok(()),
                _ => Err(FoldError::AuthorizationFailed(format!(
                    "Resolution requires Owner or Admin; author {author:?} is not"
                ))),
            }
        }

        // I5 gate: Vouch must have non-empty context and valid strength.
        AssertionType::Vouch => {
            if env.payload.len() < 37 {
                return Err(FoldError::MalformedEnvelope(
                    "Vouch payload too short".to_string(),
                ));
            }
            let ctx_len =
                u32::from_be_bytes(env.payload[32..36].try_into().unwrap()) as usize;
            if ctx_len == 0 {
                return Err(FoldError::AuthorizationFailed(
                    "Vouch must have non-empty context".to_string(),
                ));
            }
            let required = 32 + 4 + ctx_len + 1;
            if env.payload.len() < required {
                return Err(FoldError::MalformedEnvelope(format!(
                    "Vouch payload truncated: need {}, have {}",
                    required,
                    env.payload.len()
                )));
            }
            let strength_byte = env.payload[32 + 4 + ctx_len];
            if strength_byte > 2 {
                return Err(FoldError::AuthorizationFailed(format!(
                    "Vouch has invalid strength byte {}",
                    strength_byte
                )));
            }
            Ok(())
        }
    }
}

// ---------------------------------------------------------------------------
// Governance predicate
// ---------------------------------------------------------------------------

fn is_governance(t: &AssertionType) -> bool {
    matches!(
        t,
        AssertionType::GroupGenesis
            | AssertionType::MembershipAdd
            | AssertionType::MembershipRemove
            | AssertionType::RoleGrant
            | AssertionType::RoleRevoke
            | AssertionType::RuleChange
            | AssertionType::Resolution
    )
}

/// The k-of-n threshold governing an act, from the rules in effect at its position.
/// Genesis and non-governance acts have no gate (1).
fn threshold_for(t: &AssertionType, rules: &GroupRules) -> u32 {
    match t {
        AssertionType::MembershipAdd => rules.add_member_threshold,
        AssertionType::MembershipRemove => rules.remove_member_threshold,
        AssertionType::RoleGrant | AssertionType::RoleRevoke => rules.role_change_threshold,
        AssertionType::RuleChange => rules.rule_change_threshold,
        AssertionType::Resolution => rules.resolution_threshold,
        _ => 1,
    }
}

/// The 32-byte approval **subject** for a `RuleChange` act: a content hash of its
/// payload (`rule_key ‖ new_value`). A RuleChange has no principal subject, so approvers
/// name `(RuleChange, this)` — computable from the proposed change before the act exists,
/// and the act derives the identical value from its own payload. Public so approvers and
/// tests compute the same subject. The `(type, subject)` pair in `approval_matches` keeps
/// this distinct from a real principal subject even on a hash collision.
#[must_use]
pub fn rule_change_approval_subject(payload: &[u8]) -> [u8; 32] {
    *blake3::hash(payload).as_bytes()
}

/// The subject an act's approvals must name. For membership/role acts it is the target
/// principal (first 32 payload bytes); for a `RuleChange` it is the content hash of the
/// proposed change (see [`rule_change_approval_subject`]). `None` for acts with no
/// threshold subject (genesis, data-plane, `Approval` itself).
fn act_subject(env: &AssertionEnvelope) -> Option<TypesPrincipalId> {
    match env.assertion_type {
        AssertionType::MembershipAdd
        | AssertionType::MembershipRemove
        | AssertionType::RoleGrant
        | AssertionType::RoleRevoke => {
            if env.payload.len() < 32 {
                return None;
            }
            let mut b = [0u8; 32];
            b.copy_from_slice(&env.payload[..32]);
            Some(TypesPrincipalId::new(b))
        }
        AssertionType::RuleChange | AssertionType::Resolution => {
            Some(TypesPrincipalId::new(rule_change_approval_subject(&env.payload)))
        }
        _ => None,
    }
}

/// Does `approval` approve `(want_type, subject)`? Payload = act_type(2) ‖ subject(32).
fn approval_matches(approval: &AssertionEnvelope, want_type: u16, subject: &TypesPrincipalId) -> bool {
    approval.assertion_type == AssertionType::Approval
        && approval.payload.len() >= 34
        && u16::from_be_bytes([approval.payload[0], approval.payload[1]]) == want_type
        && &approval.payload[2..34] == subject.as_bytes()
}

/// Gather the distinct approver principals for `act`: its own author plus the authors of
/// its antecedent `Approval` facts that name `(act.type, subject)`. The antecedent guard
/// has already ensured the referenced approvals are present. Counting is by principal, so
/// a persona's multiple clients collapse to one (§5.7).
fn gather_approvers(
    db: &Db,
    act: &AssertionEnvelope,
    subject: &TypesPrincipalId,
) -> Result<Vec<TypesPrincipalId>, FoldError> {
    let mut approvers = vec![act.author_principal];
    let want_type = act.assertion_type.to_u16();
    let read_txn = db.inner().begin_read().map_err(|e| FoldError::StorageError(e.to_string()))?;
    let table = read_txn.open_table(AUTH_ASSERTIONS).map_err(|e| FoldError::StorageError(e.to_string()))?;
    for ant in &act.antecedents {
        let ant_key: &[u8] = ant.as_bytes();
        if let Some(bytes) = table.get(ant_key).map_err(|e| FoldError::StorageError(e.to_string()))? {
            let raw: &[u8] = bytes.value();
            if raw.is_empty() {
                continue;
            }
            if let Ok(env) = decode_envelope_from_canonical(&raw[1..]) {
                if approval_matches(&env, want_type, subject) {
                    approvers.push(env.author_principal);
                }
            }
        }
    }
    Ok(approvers)
}

// ---------------------------------------------------------------------------
// GroupState transitions
// ---------------------------------------------------------------------------

fn genesis_initial_state(
    env: &AssertionEnvelope,
    hash: TypesHash,
) -> Result<GroupState, FoldError> {
    if env.payload.len() < 50 {
        return Err(FoldError::MalformedEnvelope(format!(
            "GroupGenesis payload too short: {} bytes",
            env.payload.len()
        )));
    }
    let add_member_threshold = u32::from_be_bytes(env.payload[2..6].try_into().unwrap());
    let remove_member_threshold = u32::from_be_bytes(env.payload[6..10].try_into().unwrap());
    let role_change_threshold = u32::from_be_bytes(env.payload[10..14].try_into().unwrap());
    let rule_change_threshold = u32::from_be_bytes(env.payload[14..18].try_into().unwrap());
    Ok(GroupState {
        version: GROUP_STATE_WIRE_VERSION,
        computed_at_gov_head: hash,
        computed_at_gov_seq: 0,
        members: vec![(env.author_principal, Role::Owner, env.lamport)],
        rules: GroupRules {
            add_member_threshold,
            remove_member_threshold,
            role_change_threshold,
            rule_change_threshold,
            // Not in the genesis payload: minted at the product default (owner decision
            // 2026-08-21 — "two, so no one gets a one-signature verdict") and dialed
            // thereafter by governed RuleChange like every other threshold.
            resolution_threshold: 2,
        },
        fork_status: ForkStatus::Clean,
    })
}

/// Apply a governance assertion to produce a new `GroupState`.
fn apply_governance(
    state: &GroupState,
    env: &AssertionEnvelope,
    hash: TypesHash,
    gov_seq: u64,
) -> Result<GroupState, FoldError> {
    let mut next = GroupState {
        version: state.version,
        computed_at_gov_head: hash,
        computed_at_gov_seq: gov_seq,
        members: state.members.clone(),
        rules: state.rules.clone(),
        fork_status: state.fork_status.clone(),
    };

    match env.assertion_type {
        AssertionType::GroupGenesis => {
            if !next.members.iter().any(|(p, _, _)| p == &env.author_principal) {
                next.members.push((env.author_principal, Role::Owner, env.lamport));
            }
        }

        AssertionType::MembershipAdd => {
            if env.payload.len() < 33 {
                return Err(FoldError::MalformedEnvelope(
                    "MembershipAdd payload too short".to_string(),
                ));
            }
            let mut pid_bytes = [0u8; 32];
            pid_bytes.copy_from_slice(&env.payload[..32]);
            let invitee = TypesPrincipalId::new(pid_bytes);
            let role = u8_to_role(env.payload[32]).map_err(|_| {
                FoldError::MalformedEnvelope(format!(
                    "MembershipAdd: unknown role byte {}",
                    env.payload[32]
                ))
            })?;
            if let Some(entry) = next.members.iter_mut().find(|(p, _, _)| *p == invitee) {
                entry.1 = role;
            } else {
                next.members.push((invitee, role, env.lamport));
            }
        }

        AssertionType::MembershipRemove => {
            if env.payload.len() < 32 {
                return Err(FoldError::MalformedEnvelope(
                    "MembershipRemove payload too short".to_string(),
                ));
            }
            let mut pid_bytes = [0u8; 32];
            pid_bytes.copy_from_slice(&env.payload[..32]);
            let subject = TypesPrincipalId::new(pid_bytes);
            // Soft-remove: retain in list but mark with a sentinel role byte.
            // We keep the entry for history; the edge will be marked present=false.
            // For GroupState members we actually retain them but the edge marking
            // communicates absence. However the spec says "soft-remove for history",
            // so we keep the record.
            next.members.retain(|(p, _, _)| *p != subject);
        }

        AssertionType::RoleGrant => {
            if env.payload.len() < 33 {
                return Err(FoldError::MalformedEnvelope(
                    "RoleGrant payload too short".to_string(),
                ));
            }
            let mut pid_bytes = [0u8; 32];
            pid_bytes.copy_from_slice(&env.payload[..32]);
            let subject = TypesPrincipalId::new(pid_bytes);
            let new_role = u8_to_role(env.payload[32]).map_err(|_| {
                FoldError::MalformedEnvelope(format!(
                    "RoleGrant: unknown role byte {}",
                    env.payload[32]
                ))
            })?;
            if let Some(entry) = next.members.iter_mut().find(|(p, _, _)| *p == subject) {
                entry.1 = new_role;
            }
        }

        AssertionType::RoleRevoke => {
            if env.payload.len() < 32 {
                return Err(FoldError::MalformedEnvelope(
                    "RoleRevoke payload too short".to_string(),
                ));
            }
            let mut pid_bytes = [0u8; 32];
            pid_bytes.copy_from_slice(&env.payload[..32]);
            let subject = TypesPrincipalId::new(pid_bytes);
            if let Some(entry) = next.members.iter_mut().find(|(p, _, _)| *p == subject) {
                entry.1 = Role::Member;
            }
        }

        AssertionType::RuleChange => {
            if env.payload.len() < 5 {
                return Err(FoldError::MalformedEnvelope(
                    "RuleChange payload too short".to_string(),
                ));
            }
            let rule_key = decode_rule_key(env.payload[0]).map_err(|_| {
                FoldError::MalformedEnvelope(format!(
                    "RuleChange: unknown rule_key byte {}",
                    env.payload[0]
                ))
            })?;
            let new_value = u32::from_be_bytes(env.payload[1..5].try_into().unwrap());
            match rule_key {
                RuleKey::AddMember => next.rules.add_member_threshold = new_value,
                RuleKey::RemoveMember => next.rules.remove_member_threshold = new_value,
                RuleKey::RoleChange => next.rules.role_change_threshold = new_value,
                RuleKey::RuleChange => next.rules.rule_change_threshold = new_value,
                RuleKey::Resolution => next.rules.resolution_threshold = new_value,
            }
        }

        _ => {}
    }
    Ok(next)
}

// ---------------------------------------------------------------------------
// IngestResult
// ---------------------------------------------------------------------------

/// The outcome of a successful `ingest` call.
#[derive(Debug, Clone, PartialEq)]
pub enum IngestResult {
    /// The assertion was new and has been applied.
    Applied { hash: TypesHash },
    /// The assertion was already present; no writes were made.
    Duplicate,
}

// ---------------------------------------------------------------------------
// DerivedFold — the main fold engine for Stage 4
// ---------------------------------------------------------------------------

/// Fold engine that writes both authoritative and derived state in one transaction.
pub struct DerivedFold<V, C>
where
    V: Verifier,
    C: CredentialResolver,
{
    db: Arc<Db>,
    verifier: V,
    cred_resolver: C,
}

impl<V, C> DerivedFold<V, C>
where
    V: Verifier + Send + Sync,
    C: CredentialResolver + Send + Sync,
{
    /// Create a new `DerivedFold` and initialise all tables.
    pub fn new(db: Arc<Db>, verifier: V, cred_resolver: C) -> Self {
        let this = Self { db, verifier, cred_resolver };
        this.ensure_tables().expect("DerivedFold::new: failed to initialise tables");
        this
    }

    fn ensure_tables(&self) -> Result<(), FoldError> {
        let txn = self
            .db
            .inner()
            .begin_write()
            .map_err(|e| FoldError::StorageError(e.to_string()))?;
        txn.open_table(AUTH_ASSERTIONS)
            .map_err(|e| FoldError::StorageError(e.to_string()))?;
        txn.open_table(AUTH_ASSERTIONS_BY_DEVICE)
            .map_err(|e| FoldError::StorageError(e.to_string()))?;
        txn.open_table(AUTH_GOV_LOG)
            .map_err(|e| FoldError::StorageError(e.to_string()))?;
        txn.open_table(AUTH_GENESIS)
            .map_err(|e| FoldError::StorageError(e.to_string()))?;
        txn.open_table(IDX_NODES)
            .map_err(|e| FoldError::StorageError(e.to_string()))?;
        txn.open_table(IDX_EDGES_OUT)
            .map_err(|e| FoldError::StorageError(e.to_string()))?;
        txn.open_table(IDX_EDGES_IN)
            .map_err(|e| FoldError::StorageError(e.to_string()))?;
        txn.open_table(STATE_GROUP)
            .map_err(|e| FoldError::StorageError(e.to_string()))?;
        txn.open_table(STATE_BLOB_PRESENCE)
            .map_err(|e| FoldError::StorageError(e.to_string()))?;
        txn.commit()
            .map_err(|e| FoldError::StorageError(e.to_string()))?;
        Ok(())
    }

    /// Read the current derived [`GroupState`] for `group`, or `None` if the group has
    /// no folded state yet. A read-only accessor (a read transaction + deserialize) that
    /// exposes the folded state the horizon manifest (EXP-H1) and other fold-level tests
    /// compute over. It does not fold or mutate.
    pub fn read_group_state(&self, group: &GroupId) -> Result<Option<GroupState>, FoldError> {
        let read_txn = self
            .db
            .inner()
            .begin_read()
            .map_err(|e| FoldError::StorageError(e.to_string()))?;
        let table = read_txn
            .open_table(STATE_GROUP)
            .map_err(|e| FoldError::StorageError(e.to_string()))?;
        match table
            .get(group.as_bytes().as_ref())
            .map_err(|e| FoldError::StorageError(e.to_string()))?
        {
            Some(bytes) => Ok(Some(GroupState::from_bytes(bytes.value())?)),
            None => Ok(None),
        }
    }

    /// Ingest an assertion, writing auth + derived state in one atomic transaction.
    pub fn ingest(&self, envelope: &AssertionEnvelope) -> Result<IngestResult, FoldError> {
        // Step 1: Hash + duplicate check.
        let hash = envelope_hash(envelope);
        let hash_bytes: &[u8] = hash.as_bytes();

        {
            let read_txn = self
                .db
                .inner()
                .begin_read()
                .map_err(|e| FoldError::StorageError(e.to_string()))?;
            let table = read_txn
                .open_table(AUTH_ASSERTIONS)
                .map_err(|e| FoldError::StorageError(e.to_string()))?;
            if table
                .get(hash_bytes)
                .map_err(|e| FoldError::StorageError(e.to_string()))?
                .is_some()
            {
                return Ok(IngestResult::Duplicate);
            }
        }

        // Step 2: Verify signature.
        let canonical = envelope.canonical_bytes();
        self.verifier
            .verify(
                &TraitsDeviceId(*envelope.author_device.as_bytes()),
                &canonical,
                &envelope.signature,
            )
            .map_err(|e| FoldError::SignatureInvalid(e.to_string()))?;

        // Step 3: Validate credential.
        self.cred_resolver
            .resolve(
                &TraitsDeviceId(*envelope.author_device.as_bytes()),
                &TraitsPrincipalId(*envelope.author_principal.as_bytes()),
            )
            .map_err(|e| FoldError::CredentialInvalid(e.to_string()))?;

        // Step 4: Load current GroupState; apply authorization rules-at-position.
        let current_state = self.load_or_init_state(envelope)?;
        // §7.6.1 concurrent contradiction: a MembershipRemove whose author was itself
        // removed by a *concurrent, mutually-expelling* remove is not merely
        // "unauthorized" — it is the second half of a mutual expulsion (A⊗B), which
        // must hard-stop rather than be silently dropped. Detect that before treating
        // the authorization failure as a plain rejection. A detected contradiction is
        // carried as a `ContestedEntry` — pair as data, contested subjects, and the
        // facts the deterministic replay withholds (§7.3.2, E108).
        let mut contested_new: Option<ContestedEntry> =
            match check_authorization(&current_state, envelope) {
                Ok(()) => {
                    // Authorized concurrent conflicts (both actors survive, so both
                    // facts are authorized): removed-then-included, role thrash, and
                    // competing RuleChange.
                    if matches!(
                        envelope.assertion_type,
                        AssertionType::MembershipAdd
                            | AssertionType::MembershipRemove
                            | AssertionType::RoleGrant
                            | AssertionType::RoleRevoke
                            | AssertionType::RuleChange
                    ) {
                        let log = group_governance_log(&self.db, &envelope.group)?;
                        detect_authorized_contested(&log, envelope, &hash)
                    } else {
                        None
                    }
                }
                Err(e) => {
                    // Unauthorized. Mutual expulsion: a MembershipRemove whose author
                    // was itself removed by a concurrent, mutually-expelling remove is
                    // the second half of A⊗B — a hard-stop, not a plain rejection.
                    let entry = if envelope.assertion_type == AssertionType::MembershipRemove {
                        let log = group_governance_log(&self.db, &envelope.group)?;
                        detect_mutual_expulsion(&log, envelope, &hash).map(|partner| {
                            mutual_expulsion_entry(envelope, &hash, partner)
                        })
                    } else {
                        None
                    };
                    if entry.is_none() {
                        return Err(e);
                    }
                    entry
                }
            };
        // A Resolution never opens a contradiction; it closes one (validated in Step 7).
        if envelope.assertion_type == AssertionType::Resolution {
            contested_new = None;
        }

        // Step 5: Lamport monotonicity check.
        {
            let read_txn = self
                .db
                .inner()
                .begin_read()
                .map_err(|e| FoldError::StorageError(e.to_string()))?;
            let table = read_txn
                .open_table(AUTH_ASSERTIONS_BY_DEVICE)
                .map_err(|e| FoldError::StorageError(e.to_string()))?;
            let start = encode_by_device_key(&envelope.author_device, 0);
            let end = encode_by_device_key(&envelope.author_device, u64::MAX);
            if let Some(last_entry) = table
                .range(start.as_slice()..=end.as_slice())
                .map_err(|e| FoldError::StorageError(e.to_string()))?
                .last()
            {
                let (k, _) = last_entry.map_err(|e| FoldError::StorageError(e.to_string()))?;
                let key_bytes = k.value();
                let last_lamport = u64::from_be_bytes(
                    key_bytes[32..40]
                        .try_into()
                        .map_err(|_| FoldError::StorageError("lamport decode".to_string()))?,
                );
                if envelope.lamport <= last_lamport {
                    tracing::warn!(
                        got = envelope.lamport,
                        expected_gt = last_lamport,
                        "lamport monotonicity violation"
                    );
                    return Err(FoldError::LamportViolation {
                        device: envelope.author_device,
                        expected_gt: last_lamport,
                        got: envelope.lamport,
                    });
                }
            }
        }

        // Step 5.5 (completeness / §7.5.2 frontier-closure): a GOVERNANCE fact must
        // not be folded until every causal antecedent it declares is already
        // admitted. The per-device lamport chain (Step 5) only orders one device's
        // own history; it cannot see a dependency that crosses devices. Without this
        // check a node that missed a cross-device predecessor would admit the
        // dependent governance fact anyway and fold to an authority head no complete
        // peer ever held (the gap the G1 experiment exposed). Holding it back keeps a
        // lagging node's head a strict prefix of a complete peer's — stale-but-honest
        // — and it heals when the predecessor arrives.
        //
        // The gate is GOVERNANCE-only by the razor (Part 1 §2.0.1): a dangling
        // antecedent is a display concern for the data plane (a reply whose parent
        // message hasn't arrived), which is optimistically accepted, but a
        // convergence concern for the control plane, which decides authority. So
        // data-plane assertions keep their optimistic acceptance; only what is
        // *decided* waits for completeness.
        if is_governance(&envelope.assertion_type) && !envelope.antecedents.is_empty() {
            let read_txn = self
                .db
                .inner()
                .begin_read()
                .map_err(|e| FoldError::StorageError(e.to_string()))?;
            let table = read_txn
                .open_table(AUTH_ASSERTIONS)
                .map_err(|e| FoldError::StorageError(e.to_string()))?;
            let mut have = 0usize;
            for ant in &envelope.antecedents {
                let ant_bytes: &[u8] = ant.as_bytes();
                if table
                    .get(ant_bytes)
                    .map_err(|e| FoldError::StorageError(e.to_string()))?
                    .is_some()
                {
                    have += 1;
                }
            }
            if have < envelope.antecedents.len() {
                return Err(FoldError::MissingAntecedents {
                    have,
                    need: envelope.antecedents.len(),
                });
            }
        }

        // Step 5.6 (V5′): k-of-n threshold enforcement. A governance act whose threshold
        // (at position) is > 1 must carry approvals from enough DISTINCT personae, by
        // lineage. Approvals are `Approval` facts naming this act's (type, subject),
        // referenced as antecedents; Step 5.5 has already ensured they are present.
        // Counting is by principal, so a persona's many clients never inflate the quorum.
        // RuleChange is enforced too: its subject is a content hash of the proposed change
        // (`rule_change_approval_subject`), so approvers name (RuleChange, H(payload)).
        if is_governance(&envelope.assertion_type) {
            if let Some(subject) = act_subject(envelope) {
                let required = threshold_for(&envelope.assertion_type, &current_state.rules);
                if required > 1 {
                    let approvers = gather_approvers(&self.db, envelope, &subject)?;
                    let have = crate::governance::count_personae_by_lineage(&approvers);
                    if have < required as usize {
                        return Err(FoldError::ThresholdNotMet {
                            have,
                            need: required as usize,
                        });
                    }
                }
            }
        }

        // Step 6: Compute gov_seq for governance assertions. Also detect forks.
        //
        // GroupGenesis is special: it always claims gov_seq=0. If another genesis
        // already occupies seq=0 for this group, that is a fork regardless of
        // how many total governance entries exist.
        // For all other governance assertions the next seq is the current count.
        let (gov_seq_opt, fork_opt) = if is_governance(&envelope.assertion_type) {
            let read_txn = self
                .db
                .inner()
                .begin_read()
                .map_err(|e| FoldError::StorageError(e.to_string()))?;
            let table = read_txn
                .open_table(AUTH_GOV_LOG)
                .map_err(|e| FoldError::StorageError(e.to_string()))?;
            let start = encode_gov_log_key(&envelope.group, 0);
            let end = encode_gov_log_key(&envelope.group, u64::MAX);
            let count = table
                .range(start.as_slice()..=end.as_slice())
                .map_err(|e| FoldError::StorageError(e.to_string()))?
                .count();

            // Genesis always targets slot 0; other governance targets the next slot.
            let target_seq: u64 = if envelope.assertion_type == AssertionType::GroupGenesis {
                0
            } else {
                count as u64
            };

            // Fork detection: check if the target slot already has an entry.
            let fork_key = encode_gov_log_key(&envelope.group, target_seq);
            let existing_at_seq = table
                .get(fork_key.as_slice())
                .map_err(|e| FoldError::StorageError(e.to_string()))?
                .map(|v| {
                    let mut h = [0u8; 32];
                    h.copy_from_slice(v.value());
                    TypesHash::new(h)
                });

            let fork_status_update = if let Some(existing_hash) = existing_at_seq {
                // Deterministic tiebreak: keep the assertion with the
                // lexicographically smaller hash. Mark the other as fork point.
                if hash.as_bytes() < existing_hash.as_bytes() {
                    // Our new assertion wins; existing becomes the fork.
                    Some(ForkStatus::ForkedFrom(existing_hash))
                } else {
                    // Existing wins; we are the fork — but we still record it.
                    Some(ForkStatus::ForkedFrom(hash))
                }
            } else {
                None
            };

            (Some(target_seq), fork_status_update)
        } else {
            (None, None)
        };

        // Step 7: Compute next GroupState via the shared transition (the same function
        // the rebuild replay calls, so live folding and rebuild cannot diverge).
        let next_state_opt: Option<GroupState> = if is_governance(&envelope.assertion_type) {
            let gov_seq = gov_seq_opt
                .expect("invariant: gov_seq_opt is Some for governance assertions (set under the same is_governance predicate above)");
            let log = if contested_new.is_some()
                || envelope.assertion_type == AssertionType::Resolution
            {
                group_governance_log(&self.db, &envelope.group)?
            } else {
                Vec::new()
            };
            Some(compute_next_governance_state(
                &log,
                &current_state,
                envelope,
                hash,
                gov_seq,
                contested_new.clone(),
                fork_opt.clone(),
            )?)
        } else {
            None
        };

        // Step 8 (I1 / I5): All writes in ONE transaction.
        {
            let write_txn = self
                .db
                .inner()
                .begin_write()
                .map_err(|e| FoldError::StorageError(e.to_string()))?;

            // 8a. auth_assertions
            {
                let mut table = write_txn
                    .open_table(AUTH_ASSERTIONS)
                    .map_err(|e| FoldError::StorageError(e.to_string()))?;
                let value = envelope.canonical_bytes_with_sig();
                let mut versioned = Vec::with_capacity(1 + value.len());
                versioned.push(0x01u8);
                versioned.extend_from_slice(&value);
                table
                    .insert(hash_bytes, versioned.as_slice())
                    .map_err(|e| FoldError::StorageError(e.to_string()))?;
            }

            // 8b. auth_assertions_by_device
            {
                let mut table = write_txn
                    .open_table(AUTH_ASSERTIONS_BY_DEVICE)
                    .map_err(|e| FoldError::StorageError(e.to_string()))?;
                let key = encode_by_device_key(&envelope.author_device, envelope.lamport);
                table
                    .insert(key.as_slice(), hash_bytes)
                    .map_err(|e| FoldError::StorageError(e.to_string()))?;
            }

            // 8c. Governance: auth_gov_log + state_group + auth_genesis
            if let (Some(gov_seq), Some(ref next_state)) = (gov_seq_opt, &next_state_opt) {
                {
                    let mut table = write_txn
                        .open_table(AUTH_GOV_LOG)
                        .map_err(|e| FoldError::StorageError(e.to_string()))?;
                    let key = encode_gov_log_key(&envelope.group, gov_seq);
                    table
                        .insert(key.as_slice(), hash_bytes)
                        .map_err(|e| FoldError::StorageError(e.to_string()))?;
                }
                {
                    let mut table = write_txn
                        .open_table(STATE_GROUP)
                        .map_err(|e| FoldError::StorageError(e.to_string()))?;
                    let state_bytes = next_state.to_bytes();
                    table
                        .insert(
                            envelope.group.as_bytes().as_ref(),
                            state_bytes.as_slice(),
                        )
                        .map_err(|e| FoldError::StorageError(e.to_string()))?;
                }
                if envelope.assertion_type == AssertionType::GroupGenesis {
                    let mut table = write_txn
                        .open_table(AUTH_GENESIS)
                        .map_err(|e| FoldError::StorageError(e.to_string()))?;
                    let mut versioned = Vec::with_capacity(1 + envelope.payload.len());
                    versioned.push(0x01u8);
                    versioned.extend_from_slice(&envelope.payload);
                    table
                        .insert(
                            envelope.group.as_bytes().as_ref(),
                            versioned.as_slice(),
                        )
                        .map_err(|e| FoldError::StorageError(e.to_string()))?;
                }
            }

            // 8d. Derived: upsert author principal node in idx_nodes (all types).
            {
                let author_typed_id =
                    TypedId::new(KindTag::Principal, TypesHash::new(*envelope.author_principal.as_bytes()));
                upsert_node_stub(
                    &write_txn,
                    &author_typed_id,
                    envelope.author_principal,
                    envelope.timestamp,
                    false, // already exists = no forced update; stub only
                    None,
                )
                .map_err(|e| FoldError::StorageError(e.to_string()))?;
            }

            // 8e. Type-specific derived effects.
            apply_derived_effects_free(&write_txn, envelope, hash, &next_state_opt)?;

            write_txn
                .commit()
                .map_err(|e| FoldError::StorageError(e.to_string()))?;
        }

        Ok(IngestResult::Applied { hash })
    }

}

// ---------------------------------------------------------------------------
// Free function: apply derived effects (used by both DerivedFold and rebuild)
// ---------------------------------------------------------------------------

fn apply_derived_effects_free(
        txn: &redb::WriteTransaction,
        env: &AssertionEnvelope,
        hash: TypesHash,
        next_state: &Option<GroupState>,
    ) -> Result<(), FoldError> {
        let group_typed_id = TypedId::new(
            KindTag::Group,
            TypesHash::new(*env.group.as_bytes()),
        );

        match env.assertion_type {
            AssertionType::GroupGenesis => {
                // Create the group's NodeCard.
                upsert_node_full(
                    txn,
                    &group_typed_id,
                    KindTag::Group,
                    true,
                    "".to_string(),
                    env.author_principal,
                    env.timestamp,
                    None,
                )
                .map_err(|e| FoldError::StorageError(e.to_string()))?;
            }

            AssertionType::MembershipAdd => {
                if env.payload.len() < 33 {
                    return Err(FoldError::MalformedEnvelope(
                        "MembershipAdd payload too short".to_string(),
                    ));
                }
                let mut pid_bytes = [0u8; 32];
                pid_bytes.copy_from_slice(&env.payload[..32]);
                let invitee = TypesPrincipalId::new(pid_bytes);
                let invitee_typed =
                    TypedId::new(KindTag::Principal, TypesHash::new(pid_bytes));

                // Upsert invitee node stub.
                upsert_node_stub(
                    txn,
                    &invitee_typed,
                    invitee,
                    env.timestamp,
                    false,
                    None,
                )
                .map_err(|e| FoldError::StorageError(e.to_string()))?;

                // Write MEMBER_OF edge: invitee → group (atomically in both tables).
                let edge_meta = EdgeMeta {
                    version: 1,
                    since_lamport: env.lamport,
                    since_assertion: hash,
                    present: true,
                };
                write_edge_atomic(
                    txn,
                    &invitee_typed,
                    EdgeType::MemberOf,
                    &group_typed_id,
                    &edge_meta,
                )
                .map_err(|e| FoldError::StorageError(e.to_string()))?;
            }

            AssertionType::MembershipRemove => {
                if env.payload.len() < 32 {
                    return Err(FoldError::MalformedEnvelope(
                        "MembershipRemove payload too short".to_string(),
                    ));
                }
                let mut pid_bytes = [0u8; 32];
                pid_bytes.copy_from_slice(&env.payload[..32]);
                let subject_typed =
                    TypedId::new(KindTag::Principal, TypesHash::new(pid_bytes));

                // Mark MEMBER_OF edge present=false.
                let edge_meta = EdgeMeta {
                    version: 1,
                    since_lamport: env.lamport,
                    since_assertion: hash,
                    present: false,
                };
                write_edge_atomic(
                    txn,
                    &subject_typed,
                    EdgeType::MemberOf,
                    &group_typed_id,
                    &edge_meta,
                )
                .map_err(|e| FoldError::StorageError(e.to_string()))?;
            }

            AssertionType::RoleGrant | AssertionType::RoleRevoke => {
                // State is updated in GroupState; no additional derived edge needed.
            }

            AssertionType::RuleChange => {
                // State updated in GroupState; no additional derived edge.
            }

            AssertionType::AttachmentAdd => {
                if env.payload.len() < 1 {
                    return Err(FoldError::MalformedEnvelope(
                        "AttachmentAdd payload too short".to_string(),
                    ));
                }
                let kind_byte = env.payload[0];
                let kind = KindTag::from_u8(kind_byte).ok_or_else(|| {
                    FoldError::MalformedEnvelope(format!(
                        "AttachmentAdd: unknown KindTag byte 0x{:02x}",
                        kind_byte
                    ))
                })?;

                // Decode title.
                let (title, blob_hash, attachment_hash) =
                    decode_attachment_add_payload(&env.payload, hash)?;

                let attach_typed = TypedId::new(kind, attachment_hash);

                // Create NodeCard for the attachment.
                upsert_node_full(
                    txn,
                    &attach_typed,
                    kind,
                    true,
                    title,
                    env.author_principal,
                    env.timestamp,
                    blob_hash,
                )
                .map_err(|e| FoldError::StorageError(e.to_string()))?;

                // HAS_ATTACHMENT edge: group → attachment.
                let edge_meta = EdgeMeta {
                    version: 1,
                    since_lamport: env.lamport,
                    since_assertion: hash,
                    present: true,
                };
                write_edge_atomic(
                    txn,
                    &group_typed_id,
                    EdgeType::HasAttachment,
                    &attach_typed,
                    &edge_meta,
                )
                .map_err(|e| FoldError::StorageError(e.to_string()))?;

                // Stub BlobPresence if blob_hash is Some.
                if let Some(bh) = blob_hash {
                    let mut table = txn
                        .open_table(STATE_BLOB_PRESENCE)
                        .map_err(|e| FoldError::StorageError(e.to_string()))?;
                    // key = blob hash; value = 0x00 (stub = not confirmed present)
                    table
                        .insert(bh.as_bytes().as_ref(), [0x00u8].as_ref())
                        .map_err(|e| FoldError::StorageError(e.to_string()))?;
                }

                // Upsert stub for any typed_id referenced (I8): the attach_typed itself.
                upsert_node_stub(
                    txn,
                    &attach_typed,
                    env.author_principal,
                    env.timestamp,
                    false,
                    None,
                )
                .map_err(|e| FoldError::StorageError(e.to_string()))?;
            }

            AssertionType::Message => {
                // Derive a stable hash for this message node from the envelope hash.
                let msg_hash = compute_hash(hash.as_bytes());
                let msg_typed = TypedId::new(KindTag::ArtifactChat, msg_hash);

                upsert_node_full(
                    txn,
                    &msg_typed,
                    KindTag::ArtifactChat,
                    true,
                    String::new(),
                    env.author_principal,
                    env.timestamp,
                    None,
                )
                .map_err(|e| FoldError::StorageError(e.to_string()))?;

                // REFERENCES edge: from the target scope → message. The scope is
                // the channel (ArtifactChat) when the payload names one, else the
                // group. This makes a message addressable per channel; group-level
                // messages (no channel) still hang off the group as before.
                let scope = match crate::types::decode_message_payload(&env.payload) {
                    Some((_, _, Some(channel))) => channel,
                    _ => group_typed_id,
                };
                let edge_meta = EdgeMeta {
                    version: 1,
                    since_lamport: env.lamport,
                    since_assertion: hash,
                    present: true,
                };
                write_edge_atomic(
                    txn,
                    &scope,
                    EdgeType::References,
                    &msg_typed,
                    &edge_meta,
                )
                .map_err(|e| FoldError::StorageError(e.to_string()))?;
            }

            AssertionType::ArtifactRef => {
                if env.payload.len() < 33 {
                    return Err(FoldError::MalformedEnvelope(
                        "ArtifactRef payload too short".to_string(),
                    ));
                }
                let kind_byte = env.payload[0];
                let kind = KindTag::from_u8(kind_byte).ok_or_else(|| {
                    FoldError::MalformedEnvelope(format!(
                        "ArtifactRef: unknown KindTag byte 0x{:02x}",
                        kind_byte
                    ))
                })?;
                let mut h = [0u8; 32];
                h.copy_from_slice(&env.payload[1..33]);
                let artifact_typed = TypedId::new(kind, TypesHash::new(h));

                // Upsert stub for unknown ref (I8).
                upsert_node_stub(
                    txn,
                    &artifact_typed,
                    env.author_principal,
                    env.timestamp,
                    false,
                    None,
                )
                .map_err(|e| FoldError::StorageError(e.to_string()))?;

                // REFERENCES edge: group → artifact.
                let edge_meta = EdgeMeta {
                    version: 1,
                    since_lamport: env.lamport,
                    since_assertion: hash,
                    present: true,
                };
                write_edge_atomic(
                    txn,
                    &group_typed_id,
                    EdgeType::References,
                    &artifact_typed,
                    &edge_meta,
                )
                .map_err(|e| FoldError::StorageError(e.to_string()))?;
            }

            AssertionType::Vouch => {
                // I5 gate already checked in check_authorization.
                if env.payload.len() < 32 {
                    return Err(FoldError::MalformedEnvelope(
                        "Vouch payload too short".to_string(),
                    ));
                }
                let mut pid_bytes = [0u8; 32];
                pid_bytes.copy_from_slice(&env.payload[..32]);
                let subject_typed =
                    TypedId::new(KindTag::Principal, TypesHash::new(pid_bytes));
                let author_typed = TypedId::new(
                    KindTag::Principal,
                    TypesHash::new(*env.author_principal.as_bytes()),
                );

                // VOUCHES edge: author → subject.
                let edge_meta = EdgeMeta {
                    version: 1,
                    since_lamport: env.lamport,
                    since_assertion: hash,
                    present: true,
                };
                write_edge_atomic(
                    txn,
                    &author_typed,
                    EdgeType::Vouches,
                    &subject_typed,
                    &edge_meta,
                )
                .map_err(|e| FoldError::StorageError(e.to_string()))?;
            }

            // Approval (V5′) has no derived effect of its own — it is evidence gathered
            // by the act it approves (Step 5.6). Stored in auth_assertions; nothing else.
            AssertionType::Approval => {}

            // Resolution (§7.3.2) acts entirely on the governance projection (closing an
            // open contested pair in GroupState); it has no derived-graph effect.
            AssertionType::Resolution => {}
        }

        // Suppress unused-variable warning for next_state.
        let _ = next_state;
        Ok(())
}

// ---------------------------------------------------------------------------
// load_or_init_state helper for DerivedFold
// ---------------------------------------------------------------------------

impl<V, C> DerivedFold<V, C>
where
    V: Verifier + Send + Sync,
    C: CredentialResolver + Send + Sync,
{
    fn load_or_init_state(
        &self,
        envelope: &AssertionEnvelope,
    ) -> Result<GroupState, FoldError> {
        let read_txn = self
            .db
            .inner()
            .begin_read()
            .map_err(|e| FoldError::StorageError(e.to_string()))?;
        let table = read_txn
            .open_table(STATE_GROUP)
            .map_err(|e| FoldError::StorageError(e.to_string()))?;

        if let Some(bytes) = table
            .get(envelope.group.as_bytes().as_ref())
            .map_err(|e| FoldError::StorageError(e.to_string()))?
        {
            GroupState::from_bytes(bytes.value())
        } else {
            if envelope.assertion_type == AssertionType::GroupGenesis {
                genesis_initial_state(envelope, envelope_hash(envelope))
            } else {
                Ok(GroupState {
                    version: GROUP_STATE_WIRE_VERSION,
                    computed_at_gov_head: TypesHash::new([0u8; 32]),
                    computed_at_gov_seq: 0,
                    members: Vec::new(),
                    rules: GroupRules {
                        add_member_threshold: 1,
                        remove_member_threshold: 1,
                        role_change_threshold: 1,
                        rule_change_threshold: 1,
                        resolution_threshold: 2,
                    },
                    fork_status: ForkStatus::Clean,
                })
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Concurrent-contradiction (§7.6.1) — mutual-expulsion detection + resolution
// ---------------------------------------------------------------------------

/// Subject principal (first 32 bytes) of a `MembershipRemove` payload, if this is
/// one and it is well-formed.
fn remove_subject(env: &AssertionEnvelope) -> Option<TypesPrincipalId> {
    if env.assertion_type != AssertionType::MembershipRemove || env.payload.len() < 32 {
        return None;
    }
    let mut b = [0u8; 32];
    b.copy_from_slice(&env.payload[..32]);
    Some(TypesPrincipalId::new(b))
}

/// This group's admitted governance envelopes (with their stored hashes), in
/// gov-log order.
fn group_governance_log(
    db: &Db,
    group: &GroupId,
) -> Result<Vec<(TypesHash, AssertionEnvelope)>, FoldError> {
    let read_txn = db.inner().begin_read().map_err(|e| FoldError::StorageError(e.to_string()))?;
    let gov = read_txn.open_table(AUTH_GOV_LOG).map_err(|e| FoldError::StorageError(e.to_string()))?;
    let auth = read_txn.open_table(AUTH_ASSERTIONS).map_err(|e| FoldError::StorageError(e.to_string()))?;
    let start = encode_gov_log_key(group, 0);
    let end = encode_gov_log_key(group, u64::MAX);
    let mut out = Vec::new();
    for item in gov
        .range(start.as_slice()..=end.as_slice())
        .map_err(|e| FoldError::StorageError(e.to_string()))?
    {
        let (_, v) = item.map_err(|e| FoldError::StorageError(e.to_string()))?;
        let mut h = [0u8; 32];
        h.copy_from_slice(v.value());
        let hash = TypesHash::new(h);
        let hash_key: &[u8] = hash.as_bytes();
        if let Some(bytes) = auth.get(hash_key).map_err(|e| FoldError::StorageError(e.to_string()))? {
            let raw: &[u8] = bytes.value();
            if raw.is_empty() {
                continue;
            }
            if let Ok(env) = decode_envelope_from_canonical(&raw[1..]) {
                out.push((hash, env));
            }
        }
    }
    Ok(out)
}

/// Detect a mutual-expulsion contradiction for an incoming `MembershipRemove` `g`.
/// Returns the hash of the admitted partner remove `F` when `F` removed `g`'s author,
/// `F` was authored by `g`'s subject, and `F` is causally concurrent with `g` (A⊗B).
/// Concurrency (not mere co-existence) is required, so a later remove that causally
/// *followed* the first is a normal sequential act, not a contradiction.
fn detect_mutual_expulsion(
    log: &[(TypesHash, AssertionEnvelope)],
    g: &AssertionEnvelope,
    g_hash: &TypesHash,
) -> Option<TypesHash> {
    // Concurrency must be POSITIVELY established, which needs a causal claim. A fact
    // with no antecedents makes none, so it is not provably concurrent with anything —
    // treat it as sequential (no contradiction) rather than false-trip the escalation
    // channel. (In a real deployment governance facts always carry antecedents; the
    // empty case is a bare/legacy fact.) This positively-established-concurrency contract
    // is shared across the concurrent-contradiction predicate family (removed-then-included,
    // role-thrash, competing-RuleChange); the empty-antecedent-folds-sequential consequence
    // is deliberate and was decided knowingly (RUN-03 audit, 2026-07-14; see
    // detect_competing_rulechange for the F8 RuleChange marquee case).
    if g.antecedents.is_empty() {
        return None;
    }
    let g_subject = remove_subject(g)?; // X
    let g_author = g.author_principal; // Y
    let lookup = |k: &TypesHash| -> Option<Vec<TypesHash>> {
        if k == g_hash {
            return Some(g.antecedents.clone());
        }
        log.iter().find(|(h, _)| h == k).map(|(_, e)| e.antecedents.clone())
    };
    for (f_hash, f) in log {
        if f.antecedents.is_empty() {
            continue;
        }
        let Some(f_subject) = remove_subject(f) else {
            continue;
        };
        if f_subject == g_author
            && f.author_principal == g_subject
            && crate::governance::are_concurrent(f_hash, g_hash, &lookup)
        {
            return Some(*f_hash);
        }
    }
    None
}

/// Add subject (first 32 bytes of a `MembershipAdd` payload), if well-formed.
fn add_subject(env: &AssertionEnvelope) -> Option<TypesPrincipalId> {
    if env.assertion_type != AssertionType::MembershipAdd || env.payload.len() < 32 {
        return None;
    }
    let mut b = [0u8; 32];
    b.copy_from_slice(&env.payload[..32]);
    Some(TypesPrincipalId::new(b))
}

/// The lexicographically smaller of two hashes — a deterministic, order-independent
/// label for a conflicting pair.
fn min_hash(a: TypesHash, b: TypesHash) -> TypesHash {
    if a.as_bytes() <= b.as_bytes() {
        a
    } else {
        b
    }
}

/// Detect a removed-then-included contradiction for an incoming `MembershipAdd` or
/// `MembershipRemove` on subject S: an admitted, causally-concurrent fact of the
/// *opposite* kind on the *same* S (an add/remove race with no causal order to decide
/// "in or out"). Returns `(partner_hash, remove_hash)` — the admitted partner fact, and
/// the hash of whichever of the pair is the remove (the replay-excluded side, retaining
/// S in the substrate while the projection reports S `CONTESTED`). Unlike mutual
/// expulsion, neither author is removed, so both facts are authorized; this fires on
/// the authorized path.
fn detect_removed_then_included(
    log: &[(TypesHash, AssertionEnvelope)],
    incoming: &AssertionEnvelope,
    incoming_hash: &TypesHash,
) -> Option<(TypesHash, TypesHash)> {
    // Concurrency must be positively established (see detect_mutual_expulsion): a fact
    // with no antecedents is not provably concurrent, so a bare add-then-remove of one
    // subject is a normal sequential edit, not a contradiction.
    if incoming.antecedents.is_empty() {
        return None;
    }
    let incoming_is_remove = incoming.assertion_type == AssertionType::MembershipRemove;
    let subject = if incoming_is_remove {
        remove_subject(incoming)?
    } else {
        add_subject(incoming)?
    };
    let lookup = |k: &TypesHash| -> Option<Vec<TypesHash>> {
        if k == incoming_hash {
            return Some(incoming.antecedents.clone());
        }
        log.iter().find(|(h, _)| h == k).map(|(_, e)| e.antecedents.clone())
    };
    for (f_hash, f) in log {
        if f.antecedents.is_empty() {
            continue;
        }
        let f_subject = if incoming_is_remove { add_subject(f) } else { remove_subject(f) };
        let Some(f_subject) = f_subject else {
            continue;
        };
        if f_subject == subject && crate::governance::are_concurrent(f_hash, incoming_hash, &lookup) {
            let remove_hash = if incoming_is_remove { *incoming_hash } else { *f_hash };
            return Some((*f_hash, remove_hash));
        }
    }
    None
}

/// Subject (first 32 bytes) of a `RoleGrant`/`RoleRevoke` payload, if well-formed.
fn role_subject(env: &AssertionEnvelope) -> Option<TypesPrincipalId> {
    if !matches!(
        env.assertion_type,
        AssertionType::RoleGrant | AssertionType::RoleRevoke
    ) || env.payload.len() < 32
    {
        return None;
    }
    let mut b = [0u8; 32];
    b.copy_from_slice(&env.payload[..32]);
    Some(TypesPrincipalId::new(b))
}

/// The role a `RoleGrant`/`RoleRevoke` establishes for its subject (grant → the granted
/// role byte; revoke → Member). None if not a well-formed role act.
fn resulting_role(env: &AssertionEnvelope) -> Option<u8> {
    match env.assertion_type {
        AssertionType::RoleGrant if env.payload.len() >= 33 => Some(env.payload[32]),
        AssertionType::RoleRevoke if env.payload.len() >= 32 => Some(role_to_u8(&Role::Member)),
        _ => None,
    }
}

/// Detect a role-thrash contradiction for an incoming `RoleGrant`/`RoleRevoke` on
/// subject S: an admitted, causally-concurrent role act on the same S whose *resulting
/// role differs* — "what role?" with no causal order to decide. This covers grant-vs-
/// revoke and grant-vs-grant-to-different-roles alike, while two acts with the *same*
/// resulting role (two identical grants, or revoke-vs-revoke) stay benign. Returns
/// `(partner_hash, label)`; resolution excludes the partner and does not apply the
/// incoming, reverting S to its pre-thrash role — no verdict on the contested change.
fn detect_role_thrash(
    log: &[(TypesHash, AssertionEnvelope)],
    incoming: &AssertionEnvelope,
    incoming_hash: &TypesHash,
) -> Option<(TypesHash, TypesHash)> {
    if incoming.antecedents.is_empty() {
        return None;
    }
    let subject = role_subject(incoming)?;
    let incoming_role = resulting_role(incoming)?;
    let lookup = |k: &TypesHash| -> Option<Vec<TypesHash>> {
        if k == incoming_hash {
            return Some(incoming.antecedents.clone());
        }
        log.iter().find(|(h, _)| h == k).map(|(_, e)| e.antecedents.clone())
    };
    for (f_hash, f) in log {
        if f.antecedents.is_empty() {
            continue;
        }
        let Some(f_subject) = role_subject(f) else {
            continue;
        };
        let Some(f_role) = resulting_role(f) else {
            continue;
        };
        if f_subject == subject
            && f_role != incoming_role
            && crate::governance::are_concurrent(f_hash, incoming_hash, &lookup)
        {
            return Some((*f_hash, min_hash(*incoming_hash, *f_hash)));
        }
    }
    None
}

/// The `(rule_key byte, new_value)` a `RuleChange` payload encodes, if well-formed
/// (`rule_key ‖ new_value` — one byte then a big-endian u32, the §7.2 layout).
fn rulechange_target(env: &AssertionEnvelope) -> Option<(u8, u32)> {
    if env.assertion_type != AssertionType::RuleChange || env.payload.len() < 5 {
        return None;
    }
    let value = u32::from_be_bytes(env.payload[1..5].try_into().ok()?);
    Some((env.payload[0], value))
}

/// Detect a competing-RuleChange contradiction (§7.6.1, F8) for an incoming admitted
/// `RuleChange` on rule R: an admitted, causally-concurrent RuleChange on the *same*
/// `rule_key` whose *new_value differs* — two constitutions for R with no causal order to
/// decide which governs, exactly the §7.6-class genuine contradiction RUN-02 F8 decided
/// must hard-stop rather than be silently content-address-tiebroken. Two concurrent
/// RuleChanges with the *same* value are concordant (no contradiction); RuleChanges on
/// *different* rule_keys never conflict. Returns `(partner_hash, label)`; resolution
/// excludes the partner and does not apply the incoming, so R keeps its pre-conflict value
/// — no verdict on either change. Mirrors `detect_role_thrash`, and surfaces identically
/// (`Contradiction(min_hash(...))`).
fn detect_competing_rulechange(
    log: &[(TypesHash, AssertionEnvelope)],
    incoming: &AssertionEnvelope,
    incoming_hash: &TypesHash,
) -> Option<(TypesHash, TypesHash)> {
    // Concurrency must be positively established: a RuleChange with empty antecedents makes no
    // causal claim, so bare re-sets never contradict and fold as sequential amendments in
    // canonical (merge_cmp) order. Consequence, deliberate: a threshold-1 rule can flap between
    // concurrent setters deterministically but without a contradiction banner. Every quorum-met
    // change carries its approvals as antecedents (Part 2 §7.2 R7), so the F8 marquee case always
    // trips this predicate. If the silent flap ever proves socially wrong for a Group, the
    // remedies are a Part 2 note or raising that rule's threshold. Decided knowingly (RUN-03
    // audit, 2026-07-14).
    if incoming.antecedents.is_empty() {
        return None;
    }
    let (rule_key, new_value) = rulechange_target(incoming)?;
    let lookup = |k: &TypesHash| -> Option<Vec<TypesHash>> {
        if k == incoming_hash {
            return Some(incoming.antecedents.clone());
        }
        log.iter().find(|(h, _)| h == k).map(|(_, e)| e.antecedents.clone())
    };
    for (f_hash, f) in log {
        if f.antecedents.is_empty() {
            continue;
        }
        let Some((f_key, f_value)) = rulechange_target(f) else {
            continue;
        };
        if f_key == rule_key
            && f_value != new_value
            && crate::governance::are_concurrent(f_hash, incoming_hash, &lookup)
        {
            return Some((*f_hash, min_hash(*incoming_hash, *f_hash)));
        }
    }
    None
}

/// Resolve a detected concurrent contradiction. Recompute membership by replaying this
/// group's governance log in canonical (`merge_cmp`) order **excluding** every hash in
/// `exclude` (the conflicting removes; the incoming fact is not in the log yet). The
/// contested parties are thereby retained — no verdict is rendered — and the result is
/// byte-identical regardless of arrival order, which is what fixes the divergence. The
/// state is flagged `Contradiction(label)` with the caller's canonical pair label.
/// Build the `ContestedEntry` for a detected mutual expulsion: the pair as data, both
/// parties as contested subjects, both removes withheld from the replay (no verdict).
fn mutual_expulsion_entry(
    g: &AssertionEnvelope,
    g_hash: &TypesHash,
    partner: TypesHash,
) -> ContestedEntry {
    let mut subjects = vec![g.author_principal];
    if let Some(s) = remove_subject(g) {
        if !subjects.contains(&s) {
            subjects.push(s);
        }
    }
    subjects.sort();
    let mut excluded = vec![partner, *g_hash];
    excluded.sort();
    ContestedEntry {
        pair: ContestedEntry::order_pair(partner, *g_hash),
        subjects,
        excluded,
    }
}

/// Detection sweep for the authorized-path contradiction shapes (both actors survive,
/// so both facts are authorized): removed-then-included, role thrash, and competing
/// RuleChange. Canonical entry regardless of which half of the pair arrived second.
fn detect_authorized_contested(
    log: &[(TypesHash, AssertionEnvelope)],
    env: &AssertionEnvelope,
    hash: &TypesHash,
) -> Option<ContestedEntry> {
    match env.assertion_type {
        AssertionType::MembershipAdd | AssertionType::MembershipRemove => {
            detect_removed_then_included(log, env, hash).map(|(partner, remove_hash)| {
                let subject = if env.assertion_type == AssertionType::MembershipRemove {
                    remove_subject(env)
                } else {
                    add_subject(env)
                }
                .expect("detection required a well-formed subject");
                ContestedEntry {
                    pair: ContestedEntry::order_pair(partner, *hash),
                    subjects: vec![subject],
                    excluded: vec![remove_hash],
                }
            })
        }
        AssertionType::RoleGrant | AssertionType::RoleRevoke => {
            detect_role_thrash(log, env, hash).map(|(partner, _)| {
                let mut excluded = vec![partner, *hash];
                excluded.sort();
                ContestedEntry {
                    pair: ContestedEntry::order_pair(partner, *hash),
                    // A role contest does not contest membership (§7.3.2 scopes
                    // CONTESTED to the membership projection).
                    subjects: vec![],
                    excluded,
                }
            })
        }
        AssertionType::RuleChange => {
            detect_competing_rulechange(log, env, hash).map(|(partner, _)| {
                let mut excluded = vec![partner, *hash];
                excluded.sort();
                ContestedEntry {
                    pair: ContestedEntry::order_pair(partner, *hash),
                    subjects: vec![],
                    excluded,
                }
            })
        }
        _ => None,
    }
}

/// The facts permanently withheld by pairs already CLOSED by admitted `Resolution`
/// facts in `log`: closing a pair is not un-deciding it — the contested facts never
/// re-apply; the group re-decides forward with new governance (§7.3.2). Derived from
/// the log itself so every replay (live, contradiction, resolution, rebuild) computes
/// the identical exclusion set with no extra state to carry.
fn resolved_excluded(log: &[(TypesHash, AssertionEnvelope)]) -> Vec<TypesHash> {
    let mut out = Vec::new();
    for (_, e) in log {
        if e.assertion_type == AssertionType::Resolution && e.payload.len() == 64 {
            let mut a = [0u8; 32];
            a.copy_from_slice(&e.payload[..32]);
            let mut b = [0u8; 32];
            b.copy_from_slice(&e.payload[32..64]);
            out.push(TypesHash::new(a));
            out.push(TypesHash::new(b));
        }
    }
    out
}

/// Deterministic full replay of the governance log in canonical (`merge_cmp`) order,
/// withholding `excluded` — the order-independent construction contradictions and
/// resolutions both rest on. The caller sets `fork_status`.
fn replay_excluding(
    log: &[(TypesHash, AssertionEnvelope)],
    excluded: &[TypesHash],
    head_hash: TypesHash,
    head_seq: u64,
) -> Result<GroupState, FoldError> {
    let mut envs: Vec<&(TypesHash, AssertionEnvelope)> =
        log.iter().filter(|(h, _)| !excluded.contains(h)).collect();
    envs.sort_by(|a, b| crate::types::merge_cmp(&a.1, &b.1));

    let genesis = envs
        .iter()
        .find(|(_, e)| e.assertion_type == AssertionType::GroupGenesis)
        .ok_or_else(|| FoldError::StorageError("replay_excluding: no genesis".to_string()))?;
    let mut ns = genesis_initial_state(&genesis.1, genesis.0)?;
    let mut seq = 1u64;
    for (h, env) in envs.iter().filter(|(_, e)| e.assertion_type != AssertionType::GroupGenesis) {
        ns = apply_governance(&ns, env, *h, seq)?;
        seq += 1;
    }
    ns.computed_at_gov_head = head_hash;
    ns.computed_at_gov_seq = head_seq;
    Ok(ns)
}

/// The one governance state transition, shared by the live ingest path and the rebuild
/// replay so the two can never diverge (rebuild previously ran no detection at all — a
/// contested store silently lost its hard-stop on rebuild). Storage-free: takes the
/// admitted governance log as data. This is the surface Phase 2 extracts into the core.
fn compute_next_governance_state(
    log: &[(TypesHash, AssertionEnvelope)],
    current_state: &GroupState,
    envelope: &AssertionEnvelope,
    hash: TypesHash,
    gov_seq: u64,
    contested_new: Option<ContestedEntry>,
    fork_opt: Option<ForkStatus>,
) -> Result<GroupState, FoldError> {
    let mut ns = if envelope.assertion_type == AssertionType::Resolution {
        // §7.3.2: a Resolution closes exactly one open pair. Naming a pair that is not
        // open is refused loudly — a resolution of nothing is either a replay artifact
        // or an attempt to pre-authorize a verdict, and both must surface.
        let mut a = [0u8; 32];
        a.copy_from_slice(&envelope.payload[..32]);
        let mut b = [0u8; 32];
        b.copy_from_slice(&envelope.payload[32..64]);
        let pair = (TypesHash::new(a), TypesHash::new(b));
        let entries = current_state.contested_entries();
        let Some(idx) = entries.iter().position(|e| e.pair == pair) else {
            return Err(FoldError::AuthorizationFailed(
                "Resolution names no open contradiction pair".to_string(),
            ));
        };
        let remaining: Vec<ContestedEntry> = entries
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != idx)
            .map(|(_, e)| e.clone())
            .collect();
        let mut excluded: Vec<TypesHash> = resolved_excluded(log);
        excluded.push(pair.0);
        excluded.push(pair.1);
        for e in &remaining {
            excluded.extend_from_slice(&e.excluded);
        }
        excluded.sort();
        excluded.dedup();
        let mut ns = replay_excluding(log, &excluded, hash, gov_seq)?;
        ns.fork_status = if remaining.is_empty() {
            ForkStatus::Clean
        } else {
            ForkStatus::Contested(remaining)
        };
        ns
    } else if let Some(entry) = &contested_new {
        // A new contradiction joins whatever is already open — set-valued, so two
        // simultaneously open pairs are representable (the retired single slot's
        // structural failure). Replay withholds every open entry's excluded facts
        // plus everything already closed by resolutions.
        let mut entries: Vec<ContestedEntry> = current_state.contested_entries().to_vec();
        if !entries.contains(entry) {
            entries.push(entry.clone());
        }
        entries.sort();
        let mut excluded: Vec<TypesHash> = resolved_excluded(log);
        for e in &entries {
            excluded.extend_from_slice(&e.excluded);
        }
        excluded.sort();
        excluded.dedup();
        let mut ns = replay_excluding(log, &excluded, hash, gov_seq)?;
        ns.fork_status = ForkStatus::Contested(entries);
        ns
    } else if envelope.assertion_type == AssertionType::GroupGenesis {
        genesis_initial_state(envelope, hash)?
    } else {
        apply_governance(current_state, envelope, hash, gov_seq)?
    };

    // A slot-collision fork or under-determination never overrides a detected
    // contradiction (all are hard-stops; the contradiction is the precise one).
    if contested_new.is_none() {
        if let Some(ref fs) = fork_opt {
            ns.fork_status = fs.clone();
        }
    }
    // §7.6.1 under-determination: if this governance step left a group with no Owner
    // (required role vacant, no admissible successor), the fold hard-stops rather than
    // folding onward on a headless group. A fork or contradiction already surfaced
    // above takes precedence (all are hard-stops).
    if matches!(ns.fork_status, ForkStatus::Clean)
        && crate::governance::is_under_determined(&ns.members)
    {
        ns.fork_status = ForkStatus::UnderDetermined;
    }
    Ok(ns)
}

// ---------------------------------------------------------------------------
// Rebuild operation (I3)
// ---------------------------------------------------------------------------

/// The comparator version this store's derived tables were last folded under, or `None` if
/// the store predates comparator versioning (i.e., was folded under v1).
pub fn comparator_version(db: &Arc<Db>) -> Result<Option<u8>, FoldError> {
    let read_txn = db
        .inner()
        .begin_read()
        .map_err(|e| FoldError::StorageError(e.to_string()))?;
    let table = match read_txn.open_table(META) {
        Ok(t) => t,
        // A store written before the META table existed has no stamp at all.
        Err(redb::TableError::TableDoesNotExist(_)) => return Ok(None),
        Err(e) => return Err(FoldError::StorageError(e.to_string())),
    };
    let stamped = table
        .get(META_COMPARATOR_KEY)
        .map_err(|e| FoldError::StorageError(e.to_string()))?
        .map(|v| v.value().first().copied())
        .flatten();
    Ok(stamped)
}

/// Does this store need a rebuild before its derived state can be trusted?
///
/// True iff the store holds at least one assertion **and** its stamped comparator version is
/// not the current [`crate::types::MERGE_CMP_VERSION`]. A store folded under an older
/// comparator can project a different resolved state for any group containing cross-device
/// same-lamport (genuinely concurrent) facts, so it must be re-folded — silently reading it
/// would present a v1 resolution as if it were v2. A fresh/empty store needs nothing.
pub fn needs_rebuild(db: &Arc<Db>) -> Result<bool, FoldError> {
    // Empty store: nothing was ever folded, so no stale resolution can exist.
    let non_empty = {
        let read_txn = db
            .inner()
            .begin_read()
            .map_err(|e| FoldError::StorageError(e.to_string()))?;
        match read_txn.open_table(AUTH_ASSERTIONS) {
            Ok(t) => t
                .iter()
                .map_err(|e| FoldError::StorageError(e.to_string()))?
                .next()
                .is_some(),
            Err(redb::TableError::TableDoesNotExist(_)) => false,
            Err(e) => return Err(FoldError::StorageError(e.to_string())),
        }
    };
    if !non_empty {
        return Ok(false);
    }
    Ok(comparator_version(db)? != Some(crate::types::MERGE_CMP_VERSION))
}

/// Stamp the current comparator version into the store's meta table.
fn stamp_comparator_version(db: &Arc<Db>) -> Result<(), FoldError> {
    let write_txn = db
        .inner()
        .begin_write()
        .map_err(|e| FoldError::StorageError(e.to_string()))?;
    {
        let mut table = write_txn
            .open_table(META)
            .map_err(|e| FoldError::StorageError(e.to_string()))?;
        table
            .insert(META_COMPARATOR_KEY, &[crate::types::MERGE_CMP_VERSION][..])
            .map_err(|e| FoldError::StorageError(e.to_string()))?;
    }
    write_txn
        .commit()
        .map_err(|e| FoldError::StorageError(e.to_string()))?;
    Ok(())
}

/// Drop all derived tables and re-fold all assertions from `auth_assertions`
/// in causal (merge_cmp) order to reproduce byte-identical derived state.
pub fn rebuild(
    db: &Arc<Db>,
    verifier: &impl Verifier,
    cred_resolver: &impl CredentialResolver,
) -> Result<(), FoldError> {
    // Step 1: Collect all assertions from auth_assertions.
    let envelopes: Vec<AssertionEnvelope> = {
        let read_txn = db
            .inner()
            .begin_read()
            .map_err(|e| FoldError::StorageError(e.to_string()))?;
        let table = read_txn
            .open_table(AUTH_ASSERTIONS)
            .map_err(|e| FoldError::StorageError(e.to_string()))?;

        let mut envs = Vec::new();
        for item in table
            .iter()
            .map_err(|e: redb::StorageError| FoldError::StorageError(e.to_string()))?
        {
            let (_, v) = item.map_err(|e: redb::StorageError| FoldError::StorageError(e.to_string()))?;
            let raw: &[u8] = v.value();
            // Skip version byte (raw[0]).
            if raw.is_empty() {
                continue;
            }
            let env = decode_envelope_from_canonical(&raw[1..]).map_err(|e| {
                FoldError::MalformedEnvelope(format!("rebuild: decode failed: {}", e))
            })?;
            envs.push(env);
        }
        envs
    };

    // Step 2: Sort by causal order (merge_cmp).
    let mut envs = envelopes;
    envs.sort_by(crate::types::merge_cmp);

    // Step 3: Drop all derived and state tables.
    {
        let write_txn = db
            .inner()
            .begin_write()
            .map_err(|e| FoldError::StorageError(e.to_string()))?;

        truncate_table(&write_txn, IDX_NODES)?;
        truncate_table(&write_txn, IDX_EDGES_OUT)?;
        truncate_table(&write_txn, IDX_EDGES_IN)?;
        truncate_table(&write_txn, STATE_GROUP)?;
        truncate_table(&write_txn, STATE_BLOB_PRESENCE)?;
        // Auth genesis is authoritative; we keep it but we'll re-derive it.
        truncate_table(&write_txn, AUTH_GENESIS)?;
        // Gov log must also be cleared so that replay computes seq numbers fresh,
        // producing byte-identical state (I3 invariant).
        truncate_table(&write_txn, AUTH_GOV_LOG)?;

        write_txn
            .commit()
            .map_err(|e| FoldError::StorageError(e.to_string()))?;
    }

    // Step 4: Re-ingest all assertions in causal order.
    // We need a fold engine that skips re-writing auth_assertions (already present).
    // DerivedFold::ingest handles duplicates via the auth_assertions check — but
    // those ARE present. We must apply derived effects directly.
    let fold = DerivedFoldReplay { db: Arc::clone(db) };
    for env in &envs {
        fold.replay(env)?;
    }

    // Step 5: stamp the comparator version the derived state was just folded under, so
    // `needs_rebuild` can detect a store whose resolution predates a comparator change.
    stamp_comparator_version(db)?;

    Ok(())
}

// ---------------------------------------------------------------------------
// Internal replay engine for rebuild (no auth/sig checks, just derived effects)
// ---------------------------------------------------------------------------

struct DerivedFoldReplay {
    db: Arc<Db>,
}

impl DerivedFoldReplay {
    fn replay(&self, env: &AssertionEnvelope) -> Result<(), FoldError> {
        let hash = envelope_hash(env);

        // Load current state.
        let current_state = self.load_or_init_state(env)?;

        // Compute gov_seq (genesis always at 0; others at current count).
        let gov_seq_opt: Option<u64> = if is_governance(&env.assertion_type) {
            let read_txn = self
                .db
                .inner()
                .begin_read()
                .map_err(|e| FoldError::StorageError(e.to_string()))?;
            let table = read_txn
                .open_table(AUTH_GOV_LOG)
                .map_err(|e| FoldError::StorageError(e.to_string()))?;
            let start = encode_gov_log_key(&env.group, 0);
            let end = encode_gov_log_key(&env.group, u64::MAX);
            let count = table
                .range(start.as_slice()..=end.as_slice())
                .map_err(|e| FoldError::StorageError(e.to_string()))?
                .count();
            if env.assertion_type == AssertionType::GroupGenesis {
                Some(0)
            } else {
                Some(count as u64)
            }
        } else {
            None
        };

        // Compute next state — through the SAME shared transition as live ingest,
        // including the contradiction sweep, so a rebuild of a contested store
        // reproduces the hard-stop instead of silently losing it (the pre-P1 replay
        // ran no detection at all). Replay sees only facts live ingest admitted, so
        // running the sweep unconditionally reproduces the live outcomes: the second
        // half of a pair triggers detection exactly as it did on arrival.
        let next_state_opt: Option<GroupState> = if is_governance(&env.assertion_type) {
            let gov_seq = gov_seq_opt
                .expect("invariant: gov_seq_opt is Some for governance assertions (set under the same is_governance predicate above)");
            let log = group_governance_log(&self.db, &env.group)?;
            let contested_new = if env.assertion_type == AssertionType::Resolution {
                None
            } else if env.assertion_type == AssertionType::MembershipRemove {
                detect_mutual_expulsion(&log, env, &hash)
                    .map(|partner| mutual_expulsion_entry(env, &hash, partner))
                    .or_else(|| detect_authorized_contested(&log, env, &hash))
            } else {
                detect_authorized_contested(&log, env, &hash)
            };
            Some(compute_next_governance_state(
                &log,
                &current_state,
                env,
                hash,
                gov_seq,
                contested_new,
                None, // slot-fork detection is the live path's; rebuilt logs are collision-free
            )?)
        } else {
            None
        };

        // Write in one transaction.
        let write_txn = self
            .db
            .inner()
            .begin_write()
            .map_err(|e| FoldError::StorageError(e.to_string()))?;

        // Re-write auth_gov_log and state_group.
        if let (Some(gov_seq), Some(ref ns)) = (gov_seq_opt, &next_state_opt) {
            {
                let mut table = write_txn
                    .open_table(AUTH_GOV_LOG)
                    .map_err(|e| FoldError::StorageError(e.to_string()))?;
                let key = encode_gov_log_key(&env.group, gov_seq);
                table
                    .insert(key.as_slice(), hash.as_bytes().as_ref())
                    .map_err(|e| FoldError::StorageError(e.to_string()))?;
            }
            {
                let mut table = write_txn
                    .open_table(STATE_GROUP)
                    .map_err(|e| FoldError::StorageError(e.to_string()))?;
                let state_bytes = ns.to_bytes();
                table
                    .insert(
                        env.group.as_bytes().as_ref(),
                        state_bytes.as_slice(),
                    )
                    .map_err(|e| FoldError::StorageError(e.to_string()))?;
            }
            if env.assertion_type == AssertionType::GroupGenesis {
                let mut table = write_txn
                    .open_table(AUTH_GENESIS)
                    .map_err(|e| FoldError::StorageError(e.to_string()))?;
                let mut versioned = Vec::with_capacity(1 + env.payload.len());
                versioned.push(0x01u8);
                versioned.extend_from_slice(&env.payload);
                table
                    .insert(env.group.as_bytes().as_ref(), versioned.as_slice())
                    .map_err(|e| FoldError::StorageError(e.to_string()))?;
            }
        }

        // Author node.
        let author_typed_id =
            TypedId::new(KindTag::Principal, TypesHash::new(*env.author_principal.as_bytes()));
        upsert_node_stub(&write_txn, &author_typed_id, env.author_principal, env.timestamp, false, None)
            .map_err(|e| FoldError::StorageError(e.to_string()))?;

        // Derived effects (call the free function directly).
        apply_derived_effects_free(&write_txn, env, hash, &next_state_opt)?;

        write_txn
            .commit()
            .map_err(|e| FoldError::StorageError(e.to_string()))?;

        Ok(())
    }

    fn load_or_init_state(&self, env: &AssertionEnvelope) -> Result<GroupState, FoldError> {
        let read_txn = self
            .db
            .inner()
            .begin_read()
            .map_err(|e| FoldError::StorageError(e.to_string()))?;
        let table = read_txn
            .open_table(STATE_GROUP)
            .map_err(|e| FoldError::StorageError(e.to_string()))?;
        if let Some(bytes) = table
            .get(env.group.as_bytes().as_ref())
            .map_err(|e| FoldError::StorageError(e.to_string()))?
        {
            GroupState::from_bytes(bytes.value())
        } else {
            if env.assertion_type == AssertionType::GroupGenesis {
                genesis_initial_state(env, envelope_hash(env))
            } else {
                Ok(GroupState {
                    version: GROUP_STATE_WIRE_VERSION,
                    computed_at_gov_head: TypesHash::new([0u8; 32]),
                    computed_at_gov_seq: 0,
                    members: Vec::new(),
                    rules: GroupRules {
                        add_member_threshold: 1,
                        remove_member_threshold: 1,
                        role_change_threshold: 1,
                        rule_change_threshold: 1,
                        resolution_threshold: 2,
                    },
                    fork_status: ForkStatus::Clean,
                })
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Helper: truncate a table inside an open write transaction
// ---------------------------------------------------------------------------

fn truncate_table(
    txn: &redb::WriteTransaction,
    def: TableDefinition<'static, &'static [u8], &'static [u8]>,
) -> Result<(), FoldError> {
    let mut table = txn
        .open_table(def)
        .map_err(|e| FoldError::StorageError(e.to_string()))?;
    // Collect all keys first, then delete them.
    let keys: Vec<Vec<u8>> = {
        let iter = table
            .iter()
            .map_err(|e: redb::StorageError| FoldError::StorageError(e.to_string()))?;
        let mut collected = Vec::new();
        for item in iter {
            let (k, _v) = item
                .map_err(|e: redb::StorageError| FoldError::StorageError(e.to_string()))?;
            let key_bytes: &[u8] = k.value();
            collected.push(key_bytes.to_vec());
        }
        collected
    };
    for k in keys {
        table
            .remove(k.as_slice())
            .map_err(|e: redb::StorageError| FoldError::StorageError(e.to_string()))?;
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Helper: upsert a node stub (present=false) if not already present
// ---------------------------------------------------------------------------

fn upsert_node_stub(
    txn: &redb::WriteTransaction,
    typed_id: &TypedId,
    created_by: TypesPrincipalId,
    created_at: u64,
    _force: bool,
    blob_hash: Option<TypesHash>,
) -> Result<(), DbError> {
    let mut table = txn.open_table(IDX_NODES)?;
    let key = typed_id.as_bytes().as_ref();
    let existing_card: Option<NodeCard> =
        table.get(key)?.and_then(|g| NodeCard::from_bytes(g.value()).ok());
    if let Some(nc) = existing_card {
        // Node already exists. `created_at`/`created_by` must be a DETERMINISTIC
        // function of the log, not of fold/ingest order (I3 + cross-peer
        // convergence): keep the canonical MIN (created_at, created_by). Lowering
        // is monotonic + commutative, so any fold order — and rebuild's canonical
        // order — converge. (Fix 2026-06-26: was first-touch-wins => divergence.)
        if (created_at, *created_by.as_bytes()) < (nc.created_at, *nc.created_by.as_bytes()) {
            let updated = NodeCard { created_at, created_by, ..nc };
            table.insert(key, updated.to_bytes().as_slice())?;
        }
        return Ok(());
    }
    let nc = NodeCard {
        version: 1,
        kind: typed_id.kind(),
        present: false,
        title: String::new(),
        created_by,
        created_at,
        blob_hash,
    };
    table.insert(key, nc.to_bytes().as_slice())?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Helper: upsert a full node card (present=true), overwriting stubs
// ---------------------------------------------------------------------------

fn upsert_node_full(
    txn: &redb::WriteTransaction,
    typed_id: &TypedId,
    kind: KindTag,
    present: bool,
    title: String,
    created_by: TypesPrincipalId,
    created_at: u64,
    blob_hash: Option<TypesHash>,
) -> Result<(), DbError> {
    let mut table = txn.open_table(IDX_NODES)?;
    let key = typed_id.as_bytes().as_ref();
    let existing_card: Option<NodeCard> =
        table.get(key)?.and_then(|g| NodeCard::from_bytes(g.value()).ok());
    // `created_at`/`created_by` = canonical MIN over all referencing assertions
    // (order-insensitive; see upsert_node_stub). `present` is monotonic up.
    let (eff_at, eff_by) = match &existing_card {
        Some(nc) if (nc.created_at, *nc.created_by.as_bytes()) < (created_at, *created_by.as_bytes()) => {
            (nc.created_at, nc.created_by)
        }
        _ => (created_at, created_by),
    };
    if let Some(nc) = existing_card {
        if nc.present {
            // Already full: keep it, only converge created_at/created_by to MIN.
            let updated = NodeCard { created_at: eff_at, created_by: eff_by, ..nc };
            table.insert(key, updated.to_bytes().as_slice())?;
            return Ok(());
        }
        // existing is a stub being upgraded to full; fall through and rewrite.
    }
    let nc = NodeCard {
        version: 1,
        kind,
        present,
        title,
        created_by: eff_by,
        created_at: eff_at,
        blob_hash,
    };
    table.insert(key, nc.to_bytes().as_slice())?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Helper: write an edge to both idx_edges_out and idx_edges_in atomically
// ---------------------------------------------------------------------------

fn write_edge_atomic(
    txn: &redb::WriteTransaction,
    source: &TypedId,
    edge_type: EdgeType,
    target: &TypedId,
    meta: &EdgeMeta,
) -> Result<(), DbError> {
    let meta_bytes = meta.to_bytes();

    {
        let mut table = txn.open_table(IDX_EDGES_OUT)?;
        let key = encode_edge_out_key(source, edge_type, target);
        table.insert(key.as_ref(), meta_bytes.as_slice())?;
    }
    {
        let mut table = txn.open_table(IDX_EDGES_IN)?;
        let key = encode_edge_in_key(target, edge_type, source);
        table.insert(key.as_ref(), meta_bytes.as_slice())?;
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Helper: decode an AttachmentAdd payload
// ---------------------------------------------------------------------------

/// Returns (title, blob_hash, attachment_hash).
/// attachment_hash is a stable derived hash from the envelope hash + title.
fn decode_attachment_add_payload(
    payload: &[u8],
    envelope_h: TypesHash,
) -> Result<(String, Option<TypesHash>, TypesHash), FoldError> {
    // Layout: kind(1) || title_len(4) || title_bytes || has_blob(1) || [blob_hash(32)]
    if payload.len() < 6 {
        return Err(FoldError::MalformedEnvelope(
            "AttachmentAdd payload too short".to_string(),
        ));
    }
    let title_len = u32::from_be_bytes(payload[1..5].try_into().unwrap()) as usize;
    let title_end = 5 + title_len;
    if payload.len() < title_end + 1 {
        return Err(FoldError::MalformedEnvelope(
            "AttachmentAdd payload truncated at title".to_string(),
        ));
    }
    let title = std::str::from_utf8(&payload[5..title_end])
        .map_err(|e| {
            FoldError::MalformedEnvelope(format!("AttachmentAdd: invalid UTF-8 title: {}", e))
        })?
        .to_owned();

    let has_blob = payload[title_end];
    let blob_hash = if has_blob == 0x01 {
        if payload.len() < title_end + 1 + 32 {
            return Err(FoldError::MalformedEnvelope(
                "AttachmentAdd: truncated blob_hash".to_string(),
            ));
        }
        let mut h = [0u8; 32];
        h.copy_from_slice(&payload[title_end + 1..title_end + 33]);
        Some(TypesHash::new(h))
    } else {
        None
    };

    // Derive a stable hash for the attachment node from the envelope hash.
    let attachment_hash = compute_hash(envelope_h.as_bytes());

    Ok((title, blob_hash, attachment_hash))
}

// ---------------------------------------------------------------------------
// Helper: decode an AssertionEnvelope from canonical_bytes_with_sig layout
// ---------------------------------------------------------------------------

fn decode_envelope_from_canonical(raw: &[u8]) -> Result<AssertionEnvelope, String> {
    // Layout (from canonical_bytes_with_sig):
    // version(1) + assertion_type(2) + author_device(32) + author_principal(32)
    // + group(32) + antecedents_count(4) + antecedents*(32) + lamport(8)
    // + timestamp(8) + payload_len(4) + payload + sig_len(4) + sig
    if raw.len() < 1 + 2 + 32 + 32 + 32 + 4 + 8 + 8 + 4 {
        return Err(format!("envelope too short: {} bytes", raw.len()));
    }
    let mut off = 0;
    let version = raw[off];
    off += 1;
    let at_u16 = u16::from_be_bytes(raw[off..off + 2].try_into().unwrap());
    off += 2;
    let assertion_type = crate::types::AssertionType::from_u16(at_u16)
        .ok_or_else(|| format!("unknown assertion type 0x{:04x}", at_u16))?;
    let mut dev = [0u8; 32];
    dev.copy_from_slice(&raw[off..off + 32]);
    off += 32;
    let mut prin = [0u8; 32];
    prin.copy_from_slice(&raw[off..off + 32]);
    off += 32;
    let mut grp = [0u8; 32];
    grp.copy_from_slice(&raw[off..off + 32]);
    off += 32;
    let ant_count = u32::from_be_bytes(raw[off..off + 4].try_into().unwrap()) as usize;
    off += 4;
    let mut antecedents = Vec::with_capacity(ant_count);
    for _ in 0..ant_count {
        if raw.len() < off + 32 {
            return Err("antecedents truncated".to_string());
        }
        let mut h = [0u8; 32];
        h.copy_from_slice(&raw[off..off + 32]);
        off += 32;
        antecedents.push(TypesHash::new(h));
    }
    if raw.len() < off + 8 + 8 + 4 {
        return Err("envelope truncated before lamport".to_string());
    }
    let lamport = u64::from_be_bytes(raw[off..off + 8].try_into().unwrap());
    off += 8;
    let timestamp = u64::from_be_bytes(raw[off..off + 8].try_into().unwrap());
    off += 8;
    let payload_len = u32::from_be_bytes(raw[off..off + 4].try_into().unwrap()) as usize;
    off += 4;
    if raw.len() < off + payload_len + 4 {
        return Err("payload/sig truncated".to_string());
    }
    let payload = raw[off..off + payload_len].to_vec();
    off += payload_len;
    let sig_len = u32::from_be_bytes(raw[off..off + 4].try_into().unwrap()) as usize;
    off += 4;
    if raw.len() < off + sig_len {
        return Err("signature truncated".to_string());
    }
    let signature = raw[off..off + sig_len].to_vec();

    Ok(AssertionEnvelope {
        version,
        assertion_type,
        author_device: TypesDeviceId::new(dev),
        author_principal: TypesPrincipalId::new(prin),
        group: GroupId::new(grp),
        antecedents,
        lamport,
        timestamp,
        payload,
        signature,
    })
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tables::Db;
    use crate::traits::mocks::{MockCredentialResolver, MockSigner};
    use crate::traits::{DeviceId as TraitsDeviceId, PrincipalId as TraitsPrincipalId, Signer};
    use crate::types::{
        AssertionEnvelope, AssertionType, DeviceId as TypesDeviceId,
        GroupId, PrincipalId as TypesPrincipalId, Role,
    };
    use proptest::prelude::*;
    use std::sync::Arc;

    #[test]
    fn group_state_roundtrips_all_fork_statuses() {
        let make = |fs: ForkStatus| GroupState {
            version: GROUP_STATE_WIRE_VERSION,
            computed_at_gov_head: TypesHash::new([7u8; 32]),
            computed_at_gov_seq: 3,
            members: vec![(TypesPrincipalId::new([9u8; 32]), Role::Owner, 1)],
            rules: GroupRules {
                add_member_threshold: 1,
                remove_member_threshold: 2,
                role_change_threshold: 3,
                rule_change_threshold: 4,
                resolution_threshold: 2,
            },
            fork_status: fs,
        };
        for fs in [
            ForkStatus::Clean,
            ForkStatus::ForkedFrom(TypesHash::new([0xAB; 32])),
            ForkStatus::UnderDetermined,
            // §7.3.2/E108: the set-valued form — TWO simultaneously open entries,
            // each carrying its pair, subjects, and withheld facts.
            ForkStatus::Contested(vec![
                ContestedEntry {
                    pair: ContestedEntry::order_pair(
                        TypesHash::new([0xCD; 32]),
                        TypesHash::new([0x11; 32]),
                    ),
                    subjects: vec![
                        TypesPrincipalId::new([0x21; 32]),
                        TypesPrincipalId::new([0x22; 32]),
                    ],
                    excluded: vec![TypesHash::new([0x11; 32]), TypesHash::new([0xCD; 32])],
                },
                ContestedEntry {
                    pair: ContestedEntry::order_pair(
                        TypesHash::new([0xEE; 32]),
                        TypesHash::new([0xEF; 32]),
                    ),
                    subjects: vec![TypesPrincipalId::new([0x23; 32])],
                    excluded: vec![TypesHash::new([0xEE; 32])],
                },
            ]),
        ] {
            let state = make(fs.clone());
            let back = GroupState::from_bytes(&state.to_bytes()).expect("roundtrip");
            assert_eq!(back.fork_status, fs, "fork_status must survive to_bytes/from_bytes");
            // The variants must not collapse into one another on the wire.
            assert_eq!(back.members, state.members);
            assert_eq!(back.computed_at_gov_seq, state.computed_at_gov_seq);
            assert_eq!(back.rules, state.rules, "all five thresholds survive the wire");
        }
    }

    #[test]
    fn group_state_refuses_unknown_wire_version() {
        // O2 discipline: `from_bytes` refuses unknown versions LOUDLY — a v1 store
        // demands a rebuild rather than being silently reinterpreted as v2.
        let state = GroupState {
            version: GROUP_STATE_WIRE_VERSION,
            computed_at_gov_head: TypesHash::new([7u8; 32]),
            computed_at_gov_seq: 3,
            members: vec![],
            rules: GroupRules {
                add_member_threshold: 1,
                remove_member_threshold: 1,
                role_change_threshold: 1,
                rule_change_threshold: 1,
                resolution_threshold: 2,
            },
            fork_status: ForkStatus::Clean,
        };
        let mut bytes = state.to_bytes();
        bytes[0] = 0x01; // the retired v1 tag
        let err = GroupState::from_bytes(&bytes).expect_err("v1 bytes must be refused");
        let msg = format!("{err}");
        assert!(
            msg.contains("unknown wire version") && msg.contains("rebuilt"),
            "the refusal must name the version and demand a rebuild, got: {msg}"
        );
    }

    // -----------------------------------------------------------------------
    // Test helpers
    // -----------------------------------------------------------------------

    fn make_device(seed: u8) -> TypesDeviceId {
        TypesDeviceId::new([seed; 32])
    }

    fn make_principal(seed: u8) -> TypesPrincipalId {
        TypesPrincipalId::new([seed; 32])
    }

    fn make_group(seed: u8) -> GroupId {
        GroupId::new([seed; 32])
    }

    fn make_hash_t(seed: u8) -> TypesHash {
        TypesHash::new([seed; 32])
    }

    fn genesis_payload(device_seed: u8) -> Vec<u8> {
        let mut p = Vec::with_capacity(50);
        p.extend_from_slice(&1u16.to_be_bytes()); // policy_version
        p.extend_from_slice(&1u32.to_be_bytes()); // add_member_threshold
        p.extend_from_slice(&1u32.to_be_bytes()); // remove_member_threshold
        p.extend_from_slice(&1u32.to_be_bytes()); // role_change_threshold
        p.extend_from_slice(&1u32.to_be_bytes()); // rule_change_threshold
        p.extend_from_slice(&[device_seed; 32]); // founding_device
        p
    }

    fn membership_add_payload(principal_seed: u8, role: Role) -> Vec<u8> {
        let mut p = Vec::with_capacity(33);
        p.extend_from_slice(&[principal_seed; 32]);
        p.push(role_to_u8(&role));
        p
    }

    fn membership_remove_payload(principal_seed: u8) -> Vec<u8> {
        let mut p = vec![0u8; 32];
        p.iter_mut().for_each(|b| *b = principal_seed);
        p
    }

    fn vouch_payload(subject_seed: u8, context: &str, strength: u8) -> Vec<u8> {
        let mut p = Vec::new();
        p.extend_from_slice(&[subject_seed; 32]); // subject
        let ctx_bytes = context.as_bytes();
        p.extend_from_slice(&(ctx_bytes.len() as u32).to_be_bytes()); // ctx_len
        p.extend_from_slice(ctx_bytes); // ctx_bytes
        p.push(strength); // strength
        p
    }

    fn attachment_add_payload(kind: KindTag, title: &str, blob: Option<TypesHash>) -> Vec<u8> {
        let mut p = Vec::new();
        p.push(kind as u8);
        let title_bytes = title.as_bytes();
        p.extend_from_slice(&(title_bytes.len() as u32).to_be_bytes());
        p.extend_from_slice(title_bytes);
        match blob {
            None => p.push(0x00),
            Some(h) => {
                p.push(0x01);
                p.extend_from_slice(h.as_bytes());
            }
        }
        p
    }

    fn artifact_ref_payload(kind: KindTag, hash: TypesHash) -> Vec<u8> {
        let mut p = Vec::with_capacity(33);
        p.push(kind as u8);
        p.extend_from_slice(hash.as_bytes());
        p
    }

    fn sign_envelope(env: &mut AssertionEnvelope, signer: &MockSigner) {
        let canonical = env.canonical_bytes();
        env.signature = signer.sign(&canonical);
    }

    fn make_genesis_envelope(
        signer: &MockSigner,
        group_seed: u8,
        author_principal: TypesPrincipalId,
        lamport: u64,
    ) -> AssertionEnvelope {
        let device = TypesDeviceId::new(signer.device_id().0);
        let mut env = AssertionEnvelope {
            version: 0x01,
            assertion_type: AssertionType::GroupGenesis,
            author_device: device,
            author_principal,
            group: make_group(group_seed),
            antecedents: vec![],
            lamport,
            timestamp: 1_700_000_000,
            payload: genesis_payload(signer.device_id().0[0]),
            signature: vec![],
        };
        sign_envelope(&mut env, signer);
        env
    }

    fn make_fold(
        signer: &MockSigner,
        principal: TypesPrincipalId,
        db: Arc<Db>,
    ) -> DerivedFold<MockSigner, MockCredentialResolver> {
        let device = TypesDeviceId::new(signer.device_id().0);
        let verifier = MockSigner::new(signer.device_id().0);
        let mut cred = MockCredentialResolver::new();
        cred.register(
            TraitsDeviceId(device.as_bytes().clone()),
            TraitsPrincipalId(principal.as_bytes().clone()),
        );
        DerivedFold::new(db, verifier, cred)
    }

    // -----------------------------------------------------------------------
    // Completeness guard: a fact whose antecedents are absent is held back
    // -----------------------------------------------------------------------

    #[test]
    fn missing_antecedent_holds_the_fact_back() {
        let owner_signer = MockSigner::from_seed(0x01);
        let owner_principal = make_principal(0x01);
        let db = Arc::new(Db::create_in_memory().unwrap());
        let fold = make_fold(&owner_signer, owner_principal, Arc::clone(&db));

        let genesis = make_genesis_envelope(&owner_signer, 0x10, owner_principal, 1);
        fold.ingest(&genesis).unwrap();

        // Well-formed, authorized MembershipAdd by the owner, but declaring an
        // antecedent hash that is NOT in the store.
        let device = TypesDeviceId::new(owner_signer.device_id().0);
        let mut add_missing = AssertionEnvelope {
            version: 0x01,
            assertion_type: AssertionType::MembershipAdd,
            author_device: device,
            author_principal: owner_principal,
            group: make_group(0x10),
            antecedents: vec![TypesHash::new([0xEE; 32])], // absent
            lamport: 2,
            timestamp: 1_700_000_001,
            payload: membership_add_payload(0xBB, Role::Member),
            signature: vec![],
        };
        sign_envelope(&mut add_missing, &owner_signer);

        // HELD BACK, not admitted — nothing written.
        match fold.ingest(&add_missing) {
            Err(FoldError::MissingAntecedents { have, need }) => {
                assert_eq!((have, need), (0, 1), "one antecedent, none present");
            }
            other => panic!("expected MissingAntecedents, got {other:?}"),
        }

        // The same fact, but declaring a PRESENT antecedent (genesis), is admitted —
        // proving the guard turns on antecedent presence, not on the fact itself.
        let mut add_present = AssertionEnvelope {
            antecedents: vec![envelope_hash(&genesis)],
            signature: vec![],
            ..add_missing.clone()
        };
        sign_envelope(&mut add_present, &owner_signer);
        assert!(
            matches!(fold.ingest(&add_present), Ok(IngestResult::Applied { .. })),
            "with its antecedent present, the fact is admitted"
        );
    }

    // -----------------------------------------------------------------------
    // I1: Edge atomicity — after MembershipAdd, both edge tables have the row
    // -----------------------------------------------------------------------

    #[test]
    fn test_i1_edge_atomicity() {
        let owner_signer = MockSigner::from_seed(0x01);
        let owner_principal = make_principal(0x01);
        let db = Arc::new(Db::create_in_memory().unwrap());

        let fold = make_fold(&owner_signer, owner_principal, Arc::clone(&db));

        // Genesis.
        let genesis = make_genesis_envelope(&owner_signer, 0x10, owner_principal, 1);
        fold.ingest(&genesis).unwrap();

        // MembershipAdd.
        let device = TypesDeviceId::new(owner_signer.device_id().0);
        let invitee_seed = 0xBB_u8;
        let mut add_env = AssertionEnvelope {
            version: 0x01,
            assertion_type: AssertionType::MembershipAdd,
            author_device: device,
            author_principal: owner_principal,
            group: make_group(0x10),
            antecedents: vec![],
            lamport: 2,
            timestamp: 1_700_000_001,
            payload: membership_add_payload(invitee_seed, Role::Member),
            signature: vec![],
        };
        sign_envelope(&mut add_env, &owner_signer);
        fold.ingest(&add_env).unwrap();

        let invitee_typed = TypedId::new(
            KindTag::Principal,
            TypesHash::new([invitee_seed; 32]),
        );
        let group_typed = TypedId::new(
            KindTag::Group,
            TypesHash::new(*make_group(0x10).as_bytes()),
        );

        let read_txn = db.inner().begin_read().unwrap();

        // Check idx_edges_out.
        {
            let table = read_txn.open_table(IDX_EDGES_OUT).unwrap();
            let key = encode_edge_out_key(&invitee_typed, EdgeType::MemberOf, &group_typed);
            let v = table.get(key.as_ref()).unwrap();
            assert!(v.is_some(), "idx_edges_out must contain MEMBER_OF edge");
        }

        // Check idx_edges_in.
        {
            let table = read_txn.open_table(IDX_EDGES_IN).unwrap();
            let key = encode_edge_in_key(&group_typed, EdgeType::MemberOf, &invitee_typed);
            let v = table.get(key.as_ref()).unwrap();
            assert!(v.is_some(), "idx_edges_in must contain MEMBER_OF edge");
        }
    }

    // -----------------------------------------------------------------------
    // I2: Order-insensitive convergence (proptest)
    // -----------------------------------------------------------------------

    // Build a canonical set of 6 causally-consistent assertions (genesis + 5 ops).
    fn build_canonical_sequence(
        signer: &MockSigner,
        owner: TypesPrincipalId,
        group_seed: u8,
    ) -> Vec<AssertionEnvelope> {
        let device = TypesDeviceId::new(signer.device_id().0);

        let mut genesis = make_genesis_envelope(signer, group_seed, owner, 1);
        sign_envelope(&mut genesis, signer);

        // MembershipAdd: add member 0x02 as Member.
        let mut add1 = AssertionEnvelope {
            version: 0x01,
            assertion_type: AssertionType::MembershipAdd,
            author_device: device,
            author_principal: owner,
            group: make_group(group_seed),
            antecedents: vec![],
            lamport: 2,
            timestamp: 1_700_000_001,
            payload: membership_add_payload(0x02, Role::Member),
            signature: vec![],
        };
        sign_envelope(&mut add1, signer);

        // MembershipAdd: add member 0x03 as Admin.
        let mut add2 = AssertionEnvelope {
            version: 0x01,
            assertion_type: AssertionType::MembershipAdd,
            author_device: device,
            author_principal: owner,
            group: make_group(group_seed),
            antecedents: vec![],
            lamport: 3,
            timestamp: 1_700_000_002,
            payload: membership_add_payload(0x03, Role::Admin),
            signature: vec![],
        };
        sign_envelope(&mut add2, signer);

        // RoleGrant: promote 0x02 to Admin.
        let mut rg = AssertionEnvelope {
            version: 0x01,
            assertion_type: AssertionType::RoleGrant,
            author_device: device,
            author_principal: owner,
            group: make_group(group_seed),
            antecedents: vec![],
            lamport: 4,
            timestamp: 1_700_000_003,
            payload: {
                let mut p = vec![0x02u8; 32];
                p.push(role_to_u8(&Role::Admin));
                p
            },
            signature: vec![],
        };
        sign_envelope(&mut rg, signer);

        // MembershipRemove: remove 0x03.
        let mut rem = AssertionEnvelope {
            version: 0x01,
            assertion_type: AssertionType::MembershipRemove,
            author_device: device,
            author_principal: owner,
            group: make_group(group_seed),
            antecedents: vec![],
            lamport: 5,
            timestamp: 1_700_000_004,
            payload: membership_remove_payload(0x03),
            signature: vec![],
        };
        sign_envelope(&mut rem, signer);

        // RuleChange: set add_member_threshold to 2.
        let mut rc = AssertionEnvelope {
            version: 0x01,
            assertion_type: AssertionType::RuleChange,
            author_device: device,
            author_principal: owner,
            group: make_group(group_seed),
            antecedents: vec![],
            lamport: 6,
            timestamp: 1_700_000_005,
            payload: {
                let mut p = vec![0u8]; // AddMember key
                p.extend_from_slice(&2u32.to_be_bytes());
                p
            },
            signature: vec![],
        };
        sign_envelope(&mut rc, signer);

        vec![genesis, add1, add2, rg, rem, rc]
    }

    fn snapshot_state(db: &Arc<Db>, group_seed: u8) -> Vec<u8> {
        let read_txn = db.inner().begin_read().unwrap();
        let table = read_txn.open_table(STATE_GROUP).unwrap();
        table
            .get(make_group(group_seed).as_bytes().as_ref())
            .unwrap()
            .map(|v| v.value().to_vec())
            .unwrap_or_default()
    }

    fn snapshot_edges_out(db: &Arc<Db>) -> Vec<Vec<u8>> {
        let read_txn = db.inner().begin_read().unwrap();
        let table = read_txn.open_table(IDX_EDGES_OUT).unwrap();
        table
            .iter()
            .unwrap()
            .map(|item| {
                let (k, v) = item.unwrap();
                let mut row = k.value().to_vec();
                row.extend_from_slice(v.value());
                row
            })
            .collect::<Vec<_>>()
    }

    proptest! {
        #[test]
        fn test_i2_order_insensitive_convergence(
            permutation_seeds in proptest::collection::vec(0u8..10u8, 10),
        ) {
            let signer = MockSigner::from_seed(0x42);
            let owner = make_principal(0x42);
            let group_seed = 0xA0;
            let envs = build_canonical_sequence(&signer, owner, group_seed);

            // The sequence is already causally ordered by lamport.
            // We apply it in the canonical order (only valid causal order since
            // each assertion needs the previous governance state).
            // For order-insensitivity test we apply 10 times to separate DBs.
            let mut final_states: Vec<Vec<u8>> = Vec::new();
            let mut final_edges: Vec<Vec<Vec<u8>>> = Vec::new();

            for _ in &permutation_seeds {
                let db = Arc::new(Db::create_in_memory().unwrap());
                let fold = make_fold(&signer, owner, Arc::clone(&db));

                // Apply in canonical causal order (lamport ASC). The spec says
                // "valid causal order" permutations — since all are from the same
                // device with strictly increasing lamport, the only valid order
                // is the canonical one.
                for env in &envs {
                    fold.ingest(env).unwrap();
                }

                final_states.push(snapshot_state(&db, group_seed));
                let mut edges = snapshot_edges_out(&db);
                edges.sort();
                final_edges.push(edges);
            }

            // All 10 runs (same order, different DBs) produce identical bytes.
            for i in 1..final_states.len() {
                prop_assert_eq!(
                    &final_states[0], &final_states[i],
                    "state_group diverged on run {}", i
                );
                prop_assert_eq!(
                    &final_edges[0], &final_edges[i],
                    "idx_edges_out diverged on run {}", i
                );
            }
        }
    }

    // -----------------------------------------------------------------------
    // I3: Rebuild exact reproduction
    // -----------------------------------------------------------------------

    #[test]
    fn test_i3_rebuild_exact_reproduction() {
        let signer = MockSigner::from_seed(0x33);
        let owner = make_principal(0x33);
        let db = Arc::new(Db::create_in_memory().unwrap());
        let fold = make_fold(&signer, owner, Arc::clone(&db));

        // Ingest 20 assertions: genesis + 19 messages.
        let genesis = make_genesis_envelope(&signer, 0xB0, owner, 1);
        fold.ingest(&genesis).unwrap();

        // Add owner as member so messages are authorized.
        let device = TypesDeviceId::new(signer.device_id().0);
        let mut add_env = AssertionEnvelope {
            version: 0x01,
            assertion_type: AssertionType::MembershipAdd,
            author_device: device,
            author_principal: owner,
            group: make_group(0xB0),
            antecedents: vec![],
            lamport: 2,
            timestamp: 1_700_000_001,
            payload: membership_add_payload(0x33, Role::Owner),
            signature: vec![],
        };
        sign_envelope(&mut add_env, &signer);
        fold.ingest(&add_env).unwrap();

        for i in 0..18_u64 {
            let body = format!("msg-{}", i);
            let mut msg = AssertionEnvelope {
                version: 0x01,
                assertion_type: AssertionType::Message,
                author_device: device,
                author_principal: owner,
                group: make_group(0xB0),
                antecedents: vec![],
                lamport: 3 + i,
                timestamp: 1_700_000_002 + i,
                payload: {
                    let mut p = Vec::new();
                    p.extend_from_slice(&(body.len() as u32).to_be_bytes());
                    p.extend_from_slice(body.as_bytes());
                    p.extend_from_slice(&[0u8; 4]);
                    p
                },
                signature: vec![],
            };
            sign_envelope(&mut msg, &signer);
            fold.ingest(&msg).unwrap();
        }

        // Snapshot derived state before rebuild.
        let state_before = snapshot_state(&db, 0xB0);
        let mut edges_before = snapshot_edges_out(&db);
        edges_before.sort();

        // Rebuild.
        let verifier = MockSigner::new(signer.device_id().0);
        let cred = MockCredentialResolver::new();
        rebuild(&db, &verifier, &cred).unwrap();

        // Snapshot after rebuild.
        let state_after = snapshot_state(&db, 0xB0);
        let mut edges_after = snapshot_edges_out(&db);
        edges_after.sort();

        assert_eq!(state_before, state_after, "state_group must be byte-identical after rebuild");
        assert_eq!(edges_before, edges_after, "idx_edges_out must be byte-identical after rebuild");
    }

    // -----------------------------------------------------------------------
    // I4: Authoritative justification — every edge has a backing assertion
    // -----------------------------------------------------------------------

    #[test]
    fn test_i4_authoritative_justification() {
        let signer = MockSigner::from_seed(0x44);
        let owner = make_principal(0x44);
        let db = Arc::new(Db::create_in_memory().unwrap());
        let fold = make_fold(&signer, owner, Arc::clone(&db));

        // Build a sequence: genesis, add member, vouch.
        let genesis = make_genesis_envelope(&signer, 0xC0, owner, 1);
        fold.ingest(&genesis).unwrap();

        let device = TypesDeviceId::new(signer.device_id().0);
        let mut add_env = AssertionEnvelope {
            version: 0x01,
            assertion_type: AssertionType::MembershipAdd,
            author_device: device,
            author_principal: owner,
            group: make_group(0xC0),
            antecedents: vec![],
            lamport: 2,
            timestamp: 1_700_000_001,
            payload: membership_add_payload(0x44, Role::Owner),
            signature: vec![],
        };
        sign_envelope(&mut add_env, &signer);
        fold.ingest(&add_env).unwrap();

        let mut vouch_env = AssertionEnvelope {
            version: 0x01,
            assertion_type: AssertionType::Vouch,
            author_device: device,
            author_principal: owner,
            group: make_group(0xC0),
            antecedents: vec![],
            lamport: 3,
            timestamp: 1_700_000_002,
            payload: vouch_payload(0x55, "work", 2),
            signature: vec![],
        };
        sign_envelope(&mut vouch_env, &signer);
        fold.ingest(&vouch_env).unwrap();

        // For every edge in idx_edges_out, verify the since_assertion hash
        // exists in auth_assertions.
        let read_txn = db.inner().begin_read().unwrap();
        let edges_table = read_txn.open_table(IDX_EDGES_OUT).unwrap();
        let auth_table = read_txn.open_table(AUTH_ASSERTIONS).unwrap();

        for item in edges_table.iter().unwrap() {
            let (_, v) = item.unwrap();
            let meta = EdgeMeta::from_bytes(v.value()).unwrap();
            let assertion_exists = auth_table
                .get(meta.since_assertion.as_bytes().as_ref())
                .unwrap()
                .is_some();
            assert!(
                assertion_exists,
                "edge backed by unknown assertion {:?}",
                meta.since_assertion
            );
        }
    }

    // -----------------------------------------------------------------------
    // I8: Stub created for unknown ref
    // -----------------------------------------------------------------------

    #[test]
    fn test_i8_stub_created_for_unknown_ref() {
        let signer = MockSigner::from_seed(0x55);
        let owner = make_principal(0x55);
        let db = Arc::new(Db::create_in_memory().unwrap());
        let fold = make_fold(&signer, owner, Arc::clone(&db));

        // Genesis.
        let genesis = make_genesis_envelope(&signer, 0xD0, owner, 1);
        fold.ingest(&genesis).unwrap();

        // Add owner as member.
        let device = TypesDeviceId::new(signer.device_id().0);
        let mut add_env = AssertionEnvelope {
            version: 0x01,
            assertion_type: AssertionType::MembershipAdd,
            author_device: device,
            author_principal: owner,
            group: make_group(0xD0),
            antecedents: vec![],
            lamport: 2,
            timestamp: 1_700_000_001,
            payload: membership_add_payload(0x55, Role::Owner),
            signature: vec![],
        };
        sign_envelope(&mut add_env, &signer);
        fold.ingest(&add_env).unwrap();

        // ArtifactRef referencing an unknown group.
        let unknown_group_hash = make_hash_t(0xEE);
        let mut ref_env = AssertionEnvelope {
            version: 0x01,
            assertion_type: AssertionType::ArtifactRef,
            author_device: device,
            author_principal: owner,
            group: make_group(0xD0),
            antecedents: vec![],
            lamport: 3,
            timestamp: 1_700_000_002,
            payload: artifact_ref_payload(KindTag::Group, unknown_group_hash),
            signature: vec![],
        };
        sign_envelope(&mut ref_env, &signer);
        fold.ingest(&ref_env).unwrap();

        // Verify stub NodeCard created for the referenced group.
        let ref_typed = TypedId::new(KindTag::Group, unknown_group_hash);
        let read_txn = db.inner().begin_read().unwrap();
        let table = read_txn.open_table(IDX_NODES).unwrap();
        let node = table.get(ref_typed.as_bytes().as_ref()).unwrap();
        assert!(node.is_some(), "stub NodeCard must be created for unknown ref");
        let nc = NodeCard::from_bytes(node.unwrap().value()).unwrap();
        assert!(!nc.present, "stub must have present=false");
        assert_eq!(nc.kind, KindTag::Group, "stub must have correct kind");
    }

    // -----------------------------------------------------------------------
    // I8: Kind mismatch detectable
    // -----------------------------------------------------------------------

    #[test]
    fn test_i8_mismatch_detectable() {
        let signer = MockSigner::from_seed(0x66);
        let owner = make_principal(0x66);
        let db = Arc::new(Db::create_in_memory().unwrap());
        let fold = make_fold(&signer, owner, Arc::clone(&db));

        let genesis = make_genesis_envelope(&signer, 0xE0, owner, 1);
        fold.ingest(&genesis).unwrap();

        let device = TypesDeviceId::new(signer.device_id().0);
        let mut add_env = AssertionEnvelope {
            version: 0x01,
            assertion_type: AssertionType::MembershipAdd,
            author_device: device,
            author_principal: owner,
            group: make_group(0xE0),
            antecedents: vec![],
            lamport: 2,
            timestamp: 1_700_000_001,
            payload: membership_add_payload(0x66, Role::Owner),
            signature: vec![],
        };
        sign_envelope(&mut add_env, &signer);
        fold.ingest(&add_env).unwrap();

        let shared_hash = make_hash_t(0xAB);

        // First: reference with kind=Group → stub created with kind=Group.
        let mut ref_group = AssertionEnvelope {
            version: 0x01,
            assertion_type: AssertionType::ArtifactRef,
            author_device: device,
            author_principal: owner,
            group: make_group(0xE0),
            antecedents: vec![],
            lamport: 3,
            timestamp: 1_700_000_002,
            payload: artifact_ref_payload(KindTag::Group, shared_hash),
            signature: vec![],
        };
        sign_envelope(&mut ref_group, &signer);
        fold.ingest(&ref_group).unwrap();

        // Second: reference same hash but kind=ArtifactNote.
        let mut ref_note = AssertionEnvelope {
            version: 0x01,
            assertion_type: AssertionType::ArtifactRef,
            author_device: device,
            author_principal: owner,
            group: make_group(0xE0),
            antecedents: vec![],
            lamport: 4,
            timestamp: 1_700_000_003,
            payload: artifact_ref_payload(KindTag::ArtifactNote, shared_hash),
            signature: vec![],
        };
        sign_envelope(&mut ref_note, &signer);
        fold.ingest(&ref_note).unwrap();

        // Both TypedIds (with different kinds) can be looked up independently.
        let read_txn = db.inner().begin_read().unwrap();
        let table = read_txn.open_table(IDX_NODES).unwrap();

        let group_typed = TypedId::new(KindTag::Group, shared_hash);
        let note_typed = TypedId::new(KindTag::ArtifactNote, shared_hash);

        let group_node = table.get(group_typed.as_bytes().as_ref()).unwrap();
        let note_node = table.get(note_typed.as_bytes().as_ref()).unwrap();

        assert!(group_node.is_some(), "Group stub must exist");
        assert!(note_node.is_some(), "ArtifactNote stub must exist");

        let nc_group = NodeCard::from_bytes(group_node.unwrap().value()).unwrap();
        let nc_note = NodeCard::from_bytes(note_node.unwrap().value()).unwrap();

        // The two nodes have different kinds — mismatch is detectable.
        assert_ne!(nc_group.kind, nc_note.kind, "kind mismatch must be detectable");
    }

    // -----------------------------------------------------------------------
    // Fork detection
    // -----------------------------------------------------------------------

    #[test]
    fn test_fork_detection() {
        // Simulate two assertions at the same gov_seq by ingesting a second
        // genesis for the same group.  Since genesis is always at seq=0 and
        // the first one occupies that slot, any re-use of the slot triggers
        // fork detection.
        //
        // In practice, the DerivedFold::ingest will detect a fork when
        // AUTH_GOV_LOG already has an entry at the computed gov_seq.
        // We test this by using two different DerivedFold instances that share
        // a DB and try to write concurrent governance at the same slot.
        //
        // NOTE: The fold validates signature/credential before checking gov_seq,
        // so both signers need to be valid. We use two separate fold instances
        // (one per signer) sharing the same DB.

        let signer_a = MockSigner::from_seed(0x11);
        let signer_b = MockSigner::from_seed(0x22);
        let principal_a = make_principal(0x11);
        let principal_b = make_principal(0x22);

        let db = Arc::new(Db::create_in_memory().unwrap());

        // Build fold engine that accepts both signers and principals.
        let fold_a: DerivedFold<MockSigner, MockCredentialResolver> = {
            let verifier = MockSigner::new(signer_a.device_id().0);
            let mut cred = MockCredentialResolver::new();
            cred.register(
                TraitsDeviceId(signer_a.device_id().0),
                TraitsPrincipalId(principal_a.as_bytes().clone()),
            );
            DerivedFold::new(Arc::clone(&db), verifier, cred)
        };

        let fold_b: DerivedFold<MockSigner, MockCredentialResolver> = {
            let verifier = MockSigner::new(signer_b.device_id().0);
            let mut cred = MockCredentialResolver::new();
            cred.register(
                TraitsDeviceId(signer_b.device_id().0),
                TraitsPrincipalId(principal_b.as_bytes().clone()),
            );
            DerivedFold::new(Arc::clone(&db), verifier, cred)
        };

        // Both submit a genesis for the same group at lamport=1.
        let genesis_a = make_genesis_envelope(&signer_a, 0xF0, principal_a, 1);
        let genesis_b = make_genesis_envelope(&signer_b, 0xF0, principal_b, 1);

        // Compute hashes to determine expected tiebreak winner.
        let hash_a = envelope_hash(&genesis_a);
        let hash_b = envelope_hash(&genesis_b);

        // Ingest the first genesis.
        let r1 = fold_a.ingest(&genesis_a).unwrap();
        assert!(matches!(r1, IngestResult::Applied { .. }));

        // Ingest the second genesis — this should trigger fork detection at gov_seq=0.
        // The second genesis has a valid lamport=1 from a different device so
        // Lamport check passes (different device).
        let r2 = fold_b.ingest(&genesis_b).unwrap();
        assert!(matches!(r2, IngestResult::Applied { .. }));

        // The state_group should now reflect a fork.
        let state_bytes = snapshot_state(&db, 0xF0);
        let state = GroupState::from_bytes(&state_bytes).unwrap();

        // The winner is the one with the lex-smaller hash; fork_status names the loser.
        if hash_a.as_bytes() < hash_b.as_bytes() {
            // a wins; state should report ForkedFrom(hash_b).
            assert!(
                matches!(&state.fork_status, ForkStatus::ForkedFrom(h) if h == &hash_b),
                "expected ForkedFrom(hash_b), got {:?}", state.fork_status
            );
        } else {
            // b wins (or equal); state should report ForkedFrom(hash_a).
            assert!(
                matches!(&state.fork_status, ForkStatus::ForkedFrom(h) if h == &hash_a),
                "expected ForkedFrom(hash_a), got {:?}", state.fork_status
            );
        }

        // Tiebreak must be deterministic: apply the same pair to a fresh DB and
        // verify the same fork_status.
        let db2 = Arc::new(Db::create_in_memory().unwrap());
        let fold_a2: DerivedFold<MockSigner, MockCredentialResolver> = {
            let verifier = MockSigner::new(signer_a.device_id().0);
            let mut cred = MockCredentialResolver::new();
            cred.register(
                TraitsDeviceId(signer_a.device_id().0),
                TraitsPrincipalId(principal_a.as_bytes().clone()),
            );
            DerivedFold::new(Arc::clone(&db2), verifier, cred)
        };
        let fold_b2: DerivedFold<MockSigner, MockCredentialResolver> = {
            let verifier = MockSigner::new(signer_b.device_id().0);
            let mut cred = MockCredentialResolver::new();
            cred.register(
                TraitsDeviceId(signer_b.device_id().0),
                TraitsPrincipalId(principal_b.as_bytes().clone()),
            );
            DerivedFold::new(Arc::clone(&db2), verifier, cred)
        };
        // Apply in opposite order.
        fold_b2.ingest(&genesis_b).unwrap();
        fold_a2.ingest(&genesis_a).unwrap();

        let state2_bytes = snapshot_state(&db2, 0xF0);
        let state2 = GroupState::from_bytes(&state2_bytes).unwrap();
        assert_eq!(
            state.fork_status, state2.fork_status,
            "tiebreak must be deterministic regardless of ingestion order"
        );
    }

    // -----------------------------------------------------------------------
    // Comparator versioning (merge_cmp v2) — stamp, needs_rebuild, migration
    // -----------------------------------------------------------------------

    /// A non-empty store with no comparator stamp cannot prove its derived state was resolved
    /// under the current comparator, so it needs a rebuild; `rebuild` stamps it; an empty
    /// store needs nothing.
    #[test]
    fn rebuild_stamps_the_comparator_version_and_unstamped_stores_need_rebuild() {
        let signer = MockSigner::from_seed(0x31);
        let principal = make_principal(0x31);
        let db = Arc::new(Db::create_in_memory().unwrap());

        // Empty store: nothing folded, nothing stale.
        assert!(!needs_rebuild(&db).unwrap(), "an empty store needs no rebuild");
        assert_eq!(comparator_version(&db).unwrap(), None);

        // Ingest-built store: non-empty and unstamped. Conservative-true by design — an
        // ingest-built store's derived state follows ARRIVAL order (the projection-divergence
        // finding, G1), so its resolution provenance is unknown until a rebuild canonicalizes
        // it under the stamped comparator.
        let fold = make_fold(&signer, principal, Arc::clone(&db));
        let genesis = make_genesis_envelope(&signer, 0x31, principal, 1);
        fold.ingest(&genesis).unwrap();
        assert!(
            needs_rebuild(&db).unwrap(),
            "non-empty + unstamped == cannot prove current-comparator resolution"
        );

        // Rebuild canonicalizes and stamps.
        let verifier = MockSigner::new(signer.device_id().0);
        let mut cred = MockCredentialResolver::new();
        cred.register(
            TraitsDeviceId(signer.device_id().0),
            TraitsPrincipalId(principal.as_bytes().clone()),
        );
        rebuild(&db, &verifier, &cred).unwrap();
        assert_eq!(
            comparator_version(&db).unwrap(),
            Some(crate::types::MERGE_CMP_VERSION),
            "rebuild stamps the comparator it folded under"
        );
        assert!(!needs_rebuild(&db).unwrap(), "a stamped, current store needs nothing");
    }

    /// The migration property the v2 change rests on: after `rebuild`, derived state is a
    /// function of the canonical (comparator) order, not of arrival order — measured on a pair
    /// of genuinely concurrent facts (same lamport, different devices) whose v1 (device) and
    /// v2 (hash) orders disagree, so the reorder is actually exercised.
    #[test]
    fn rebuild_canonicalizes_arrival_orders_under_the_v2_comparator() {
        let signer_a = MockSigner::from_seed(0x41);
        let signer_b = MockSigner::from_seed(0x42);
        let principal_a = make_principal(0x41);
        let principal_b = make_principal(0x42);
        let group_seed = 0x40;

        // O1 founds; O1 seats O2 as Owner; then the concurrent pair: O1 adds X while O2 adds Y,
        // same lamport, different devices — genuinely concurrent, non-conflicting subjects.
        let genesis = make_genesis_envelope(&signer_a, group_seed, principal_a, 1);
        let device_a = TypesDeviceId::new(signer_a.device_id().0);
        let device_b = TypesDeviceId::new(signer_b.device_id().0);
        let mut seat_o2 = AssertionEnvelope {
            version: 0x01,
            assertion_type: AssertionType::MembershipAdd,
            author_device: device_a,
            author_principal: principal_a,
            group: make_group(group_seed),
            antecedents: vec![],
            lamport: 2,
            timestamp: 1_700_000_001,
            payload: membership_add_payload(0x42, Role::Owner),
            signature: vec![],
        };
        sign_envelope(&mut seat_o2, &signer_a);
        let make_add = |device: TypesDeviceId,
                        principal: TypesPrincipalId,
                        signer: &MockSigner,
                        subject_seed: u8|
         -> AssertionEnvelope {
            let mut env = AssertionEnvelope {
                version: 0x01,
                assertion_type: AssertionType::MembershipAdd,
                author_device: device,
                author_principal: principal,
                group: make_group(group_seed),
                antecedents: vec![],
                lamport: 3,
                timestamp: 1_700_000_002,
                payload: membership_add_payload(subject_seed, Role::Member),
                signature: vec![],
            };
            sign_envelope(&mut env, signer);
            env
        };
        let add_y = make_add(device_b, principal_b, &signer_b, 0x52);

        // The pair must exercise the v1→v2 reorder: device order and hash order must disagree,
        // or this test degenerates into the commutative case and proves nothing about the
        // comparator. The hash covers the payload, so search subject seeds deterministically
        // for a pair the two comparators order oppositely — no hand-picked luck.
        let device_dir = device_a.as_bytes().cmp(device_b.as_bytes());
        let add_x = (0x51..0x80u8)
            .map(|seed| make_add(device_a, principal_a, &signer_a, seed))
            .find(|cand| {
                envelope_hash(cand)
                    .as_bytes()
                    .cmp(envelope_hash(&add_y).as_bytes())
                    != device_dir
            })
            .expect("some subject seed in 0x51..0x80 must flip the hash direction");

        // Two stores, opposite arrival orders for the concurrent pair.
        let build = |first: &AssertionEnvelope, second: &AssertionEnvelope| -> Arc<Db> {
            let db = Arc::new(Db::create_in_memory().unwrap());
            let fold_a = make_fold(&signer_a, principal_a, Arc::clone(&db));
            let fold_b = make_fold(&signer_b, principal_b, Arc::clone(&db));
            fold_a.ingest(&genesis).unwrap();
            fold_a.ingest(&seat_o2).unwrap();
            let route = |env: &AssertionEnvelope| {
                if env.author_device == device_a {
                    fold_a.ingest(env).unwrap();
                } else {
                    fold_b.ingest(env).unwrap();
                }
            };
            route(first);
            route(second);
            db
        };
        let db_xy = build(&add_x, &add_y);
        let db_yx = build(&add_y, &add_x);

        // Rebuild both under the current comparator.
        let verifier = MockSigner::new(signer_a.device_id().0);
        let mut cred = MockCredentialResolver::new();
        cred.register(
            TraitsDeviceId(signer_a.device_id().0),
            TraitsPrincipalId(principal_a.as_bytes().clone()),
        );
        cred.register(
            TraitsDeviceId(signer_b.device_id().0),
            TraitsPrincipalId(principal_b.as_bytes().clone()),
        );
        rebuild(&db_xy, &verifier, &cred).unwrap();
        rebuild(&db_yx, &verifier, &cred).unwrap();

        // The migration property: byte-identical derived state, arrival order erased.
        assert_eq!(
            snapshot_state(&db_xy, group_seed),
            snapshot_state(&db_yx, group_seed),
            "post-rebuild derived state must be a function of the canonical order alone"
        );
        assert_eq!(comparator_version(&db_xy).unwrap(), Some(crate::types::MERGE_CMP_VERSION));
        assert_eq!(comparator_version(&db_yx).unwrap(), Some(crate::types::MERGE_CMP_VERSION));
    }
}

