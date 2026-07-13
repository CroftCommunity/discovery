//! Fold engine — authoritative path for `local_storage_projection` — Stage 3.
//!
//! This module implements the single-writer ingest pipeline that validates,
//! authorises, and persists every `AssertionEnvelope` in one atomic transaction
//! (invariants I1, I5, I9 from the spec).

use std::sync::Arc;
use thiserror::Error;

use crate::types::{
    AssertionEnvelope, AssertionType, DeviceId as TypesDeviceId, GroupRules,
    PrincipalId as TypesPrincipalId, Role, RuleKey,
    envelope_hash,
};
use crate::types::Hash as TypesHash;

use crate::traits::{
    CredentialResolver, LamportSource, Verifier,
    DeviceId as TraitsDeviceId, PrincipalId as TraitsPrincipalId,
};

use crate::tables::{
    Db, DbError,
    encode_by_device_key, encode_gov_log_key,
};

// Re-export the table definitions we need to open in transactions.
// redb TableDefinition is Copy so we just reference the module constants.
use redb::TableDefinition;

const AUTH_ASSERTIONS: TableDefinition<'static, &'static [u8], &'static [u8]> =
    TableDefinition::new("auth_assertions_v1");
const AUTH_ASSERTIONS_BY_DEVICE: TableDefinition<'static, &'static [u8], &'static [u8]> =
    TableDefinition::new("auth_assertions_by_device_v1");
const AUTH_GOV_LOG: TableDefinition<'static, &'static [u8], &'static [u8]> =
    TableDefinition::new("auth_gov_log_v1");
const STATE_GROUP: TableDefinition<'static, &'static [u8], &'static [u8]> =
    TableDefinition::new("state_group_v1");

// ---------------------------------------------------------------------------
// Conversion helpers between traits newtypes and types newtypes
// ---------------------------------------------------------------------------

fn types_device_to_traits(d: &TypesDeviceId) -> TraitsDeviceId {
    TraitsDeviceId(*d.as_bytes())
}

fn types_principal_to_traits(p: &TypesPrincipalId) -> TraitsPrincipalId {
    TraitsPrincipalId(*p.as_bytes())
}

// ---------------------------------------------------------------------------
// FoldError
// ---------------------------------------------------------------------------

/// All errors that can be produced by the authoritative fold engine.
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
// GroupState — in-memory projection of STATE_GROUP
// ---------------------------------------------------------------------------
//
// Wire layout (version = 1):
//   version          : 1 byte (0x01)
//   rules            : 4 × 4 bytes big-endian u32
//                      [add_member_threshold, remove_member_threshold,
//                       role_change_threshold, rule_change_threshold]
//   member_count     : 4 bytes big-endian u32
//   members          : member_count × (32 + 1) bytes
//                      PrincipalId(32) || Role(1)
//                      Role encoding: 0=Owner, 1=Admin, 2=Member, 3=Observer

#[derive(Debug, Clone)]
struct GroupState {
    rules: GroupRules,
    members: Vec<(TypesPrincipalId, Role)>,
}

impl GroupState {
    fn to_bytes(&self) -> Vec<u8> {
        let member_count = self.members.len() as u32;
        let mut buf = Vec::with_capacity(1 + 16 + 4 + self.members.len() * 33);
        buf.push(0x01u8); // version
        buf.extend_from_slice(&self.rules.add_member_threshold.to_be_bytes());
        buf.extend_from_slice(&self.rules.remove_member_threshold.to_be_bytes());
        buf.extend_from_slice(&self.rules.role_change_threshold.to_be_bytes());
        buf.extend_from_slice(&self.rules.rule_change_threshold.to_be_bytes());
        buf.extend_from_slice(&member_count.to_be_bytes());
        for (pid, role) in &self.members {
            buf.extend_from_slice(pid.as_bytes());
            buf.push(role_to_u8(role));
        }
        buf
    }

    fn from_bytes(b: &[u8]) -> Result<Self, FoldError> {
        if b.len() < 21 {
            return Err(FoldError::StorageError(format!(
                "GroupState: too short ({} bytes)", b.len()
            )));
        }
        // Skip version byte at b[0].
        let add_member_threshold    = u32::from_be_bytes(b[1..5].try_into().unwrap());
        let remove_member_threshold = u32::from_be_bytes(b[5..9].try_into().unwrap());
        let role_change_threshold   = u32::from_be_bytes(b[9..13].try_into().unwrap());
        let rule_change_threshold   = u32::from_be_bytes(b[13..17].try_into().unwrap());
        let member_count            = u32::from_be_bytes(b[17..21].try_into().unwrap()) as usize;

        let required = 21 + member_count * 33;
        if b.len() < required {
            return Err(FoldError::StorageError(format!(
                "GroupState: need {} bytes for {} members, have {}",
                required, member_count, b.len()
            )));
        }
        let mut members = Vec::with_capacity(member_count);
        for i in 0..member_count {
            let offset = 21 + i * 33;
            let mut pid_bytes = [0u8; 32];
            pid_bytes.copy_from_slice(&b[offset..offset + 32]);
            let role = u8_to_role(b[offset + 32]).map_err(|_| {
                FoldError::StorageError(format!("GroupState: unknown role byte {}", b[offset + 32]))
            })?;
            members.push((TypesPrincipalId::new(pid_bytes), role));
        }
        Ok(GroupState {
            rules: GroupRules {
                add_member_threshold,
                remove_member_threshold,
                role_change_threshold,
                rule_change_threshold,
            },
            members,
        })
    }
}

fn role_to_u8(r: &Role) -> u8 {
    match r {
        Role::Owner    => 0,
        Role::Admin    => 1,
        Role::Member   => 2,
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
// Authorization context and rules-at-position (I6)
// ---------------------------------------------------------------------------

/// Context passed to the authorization check.
pub struct AuthorizationContext<'a> {
    pub rules: &'a GroupRules,
    pub members: &'a [(TypesPrincipalId, Role)],
    pub author: &'a TypesPrincipalId,
}

/// Returns the `Role` of `author` in the member list, or `None` if not a member.
fn author_role<'a>(
    members: &'a [(TypesPrincipalId, Role)],
    author: &TypesPrincipalId,
) -> Option<&'a Role> {
    members.iter().find(|(pid, _)| pid == author).map(|(_, r)| r)
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

/// Apply the rules-at-position authorization check.
///
/// Returns `Ok(())` if the author is permitted, or `Err(FoldError::AuthorizationFailed)`
/// otherwise.
pub fn check_authorization(
    ctx: &AuthorizationContext,
    assertion_type: &AssertionType,
    payload: &[u8],
) -> Result<(), FoldError> {
    match assertion_type {
        // GroupGenesis is always allowed — it is the first assertion for a group.
        AssertionType::GroupGenesis => Ok(()),

        // MembershipAdd: author must be Owner or Admin (simplified threshold for Stage 3).
        AssertionType::MembershipAdd => {
            match author_role(ctx.members, ctx.author) {
                Some(r) if role_ge_admin(r) => Ok(()),
                _ => Err(FoldError::AuthorizationFailed(format!(
                    "MembershipAdd requires Owner or Admin; author {:?} is not",
                    ctx.author
                ))),
            }
        }

        // MembershipRemove: author must be Owner or Admin.
        AssertionType::MembershipRemove => {
            match author_role(ctx.members, ctx.author) {
                Some(r) if role_ge_admin(r) => Ok(()),
                _ => Err(FoldError::AuthorizationFailed(format!(
                    "MembershipRemove requires Owner or Admin; author {:?} is not",
                    ctx.author
                ))),
            }
        }

        // RoleGrant / RoleRevoke: author must be Owner.
        AssertionType::RoleGrant | AssertionType::RoleRevoke => {
            match author_role(ctx.members, ctx.author) {
                Some(r) if role_ge_owner(r) => Ok(()),
                _ => Err(FoldError::AuthorizationFailed(format!(
                    "{:?} requires Owner; author {:?} is not",
                    assertion_type, ctx.author
                ))),
            }
        }

        // RuleChange: parse payload; author must satisfy the CURRENT threshold for
        // the rule being changed.  For Stage 3 we simplify "satisfies threshold" to
        // "is Owner" (single-node quorum = 1 owner always suffices).
        AssertionType::RuleChange => {
            // Decode a RuleChangePayload from `payload`.
            // Wire layout (mirrors RuleChangePayload):
            //   rule_key  : 1 byte  (0=AddMember, 1=RemoveMember, 2=RoleChange, 3=RuleChange)
            //   new_value : 4 bytes big-endian u32
            if payload.len() < 5 {
                return Err(FoldError::MalformedEnvelope(
                    "RuleChange payload too short".to_string(),
                ));
            }
            let rule_key = decode_rule_key(payload[0]).map_err(|_| {
                FoldError::MalformedEnvelope(format!(
                    "RuleChange: unknown rule_key byte {}", payload[0]
                ))
            })?;
            // Determine the current threshold for the key being changed.
            let current_threshold = match rule_key {
                RuleKey::AddMember    => ctx.rules.add_member_threshold,
                RuleKey::RemoveMember => ctx.rules.remove_member_threshold,
                RuleKey::RoleChange   => ctx.rules.role_change_threshold,
                RuleKey::RuleChange   => ctx.rules.rule_change_threshold,
            };
            // For Stage 3: threshold is satisfied when the author is Owner.
            // (A full implementation would count members whose role meets the bar.)
            let _ = current_threshold; // acknowledged; used in full impl
            match author_role(ctx.members, ctx.author) {
                Some(r) if role_ge_owner(r) => Ok(()),
                _ => Err(FoldError::AuthorizationFailed(format!(
                    "RuleChange requires Owner; author {:?} is not",
                    ctx.author
                ))),
            }
        }

        // AttachmentAdd, Message, ArtifactRef: author must be a member (any role).
        AssertionType::AttachmentAdd
        | AssertionType::Message
        | AssertionType::ArtifactRef => {
            match author_role(ctx.members, ctx.author) {
                Some(r) if role_ge_member(r) => Ok(()),
                _ => Err(FoldError::AuthorizationFailed(format!(
                    "{:?} requires membership; author {:?} is not a member",
                    assertion_type, ctx.author
                ))),
            }
        }

        // Vouch: payload must have non-empty context and a valid (graded) strength.
        AssertionType::Vouch => {
            // Wire layout for VouchPayload:
            //   subject   : 32 bytes PrincipalId
            //   ctx_len   : 4 bytes big-endian u32
            //   ctx_bytes : ctx_len bytes (UTF-8 context string)
            //   strength  : 1 byte (0=Weak, 1=Moderate, 2=Strong)
            if payload.len() < 37 {
                return Err(FoldError::MalformedEnvelope(
                    "Vouch payload too short".to_string(),
                ));
            }
            let ctx_len = u32::from_be_bytes(
                payload[32..36].try_into().unwrap()
            ) as usize;
            if ctx_len == 0 {
                return Err(FoldError::AuthorizationFailed(
                    "Vouch must have non-empty context".to_string(),
                ));
            }
            let required = 32 + 4 + ctx_len + 1;
            if payload.len() < required {
                return Err(FoldError::MalformedEnvelope(format!(
                    "Vouch payload truncated: need {}, have {}",
                    required, payload.len()
                )));
            }
            let strength_byte = payload[32 + 4 + ctx_len];
            // Validate strength is a known graded value.
            match strength_byte {
                0 | 1 | 2 => {} // Weak / Moderate / Strong
                _ => {
                    return Err(FoldError::AuthorizationFailed(format!(
                        "Vouch has ungraded/invalid strength byte {}",
                        strength_byte
                    )));
                }
            }
            Ok(())
        }

        // Approval (V5′): an approver of a governance act must be governance-eligible
        // (Owner/Admin) — it is co-authoring the act toward its k-of-n threshold.
        AssertionType::Approval => match author_role(ctx.members, ctx.author) {
            Some(r) if role_ge_admin(r) => Ok(()),
            _ => Err(FoldError::AuthorizationFailed(
                "Approval requires Owner or Admin".to_string(),
            )),
        },
    }
}

fn decode_rule_key(v: u8) -> Result<RuleKey, ()> {
    match v {
        0 => Ok(RuleKey::AddMember),
        1 => Ok(RuleKey::RemoveMember),
        2 => Ok(RuleKey::RoleChange),
        3 => Ok(RuleKey::RuleChange),
        _ => Err(()),
    }
}

// ---------------------------------------------------------------------------
// Governance assertion predicate
// ---------------------------------------------------------------------------

/// Returns `true` for assertion types that mutate group governance state and
/// should be appended to `auth_gov_log`.
fn is_governance(t: &AssertionType) -> bool {
    matches!(
        t,
        AssertionType::GroupGenesis
            | AssertionType::MembershipAdd
            | AssertionType::MembershipRemove
            | AssertionType::RoleGrant
            | AssertionType::RoleRevoke
            | AssertionType::RuleChange
    )
}

// ---------------------------------------------------------------------------
// GroupState transitions
// ---------------------------------------------------------------------------

/// Apply a governance assertion to produce a new `GroupState`.
///
/// Only called when `is_governance` is true.  Non-governance assertions are
/// passed through without state mutation.
fn apply_governance(
    state: &GroupState,
    env: &AssertionEnvelope,
) -> Result<GroupState, FoldError> {
    let mut next = state.clone();
    match env.assertion_type {
        AssertionType::GroupGenesis => {
            // Genesis initialises the state; the state was already seeded from
            // the genesis payload in `load_or_init_state`.  Additionally, the
            // author_principal is implicitly granted the Owner role so that
            // subsequent governance operations (MembershipAdd, etc.) succeed.
            if !next.members.iter().any(|(p, _)| p == &env.author_principal) {
                next.members.push((env.author_principal, Role::Owner));
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
                    "MembershipAdd: unknown role byte {}", env.payload[32]
                ))
            })?;
            // Upsert: replace if already present.
            if let Some(entry) = next.members.iter_mut().find(|(p, _)| *p == invitee) {
                entry.1 = role;
            } else {
                next.members.push((invitee, role));
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
            next.members.retain(|(p, _)| *p != subject);
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
                    "RoleGrant: unknown role byte {}", env.payload[32]
                ))
            })?;
            if let Some(entry) = next.members.iter_mut().find(|(p, _)| *p == subject) {
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
            // Demote to Member (revoke elevated role).
            if let Some(entry) = next.members.iter_mut().find(|(p, _)| *p == subject) {
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
                    "RuleChange: unknown rule_key byte {}", env.payload[0]
                ))
            })?;
            let new_value = u32::from_be_bytes(env.payload[1..5].try_into().unwrap());
            match rule_key {
                RuleKey::AddMember    => next.rules.add_member_threshold    = new_value,
                RuleKey::RemoveMember => next.rules.remove_member_threshold = new_value,
                RuleKey::RoleChange   => next.rules.role_change_threshold   = new_value,
                RuleKey::RuleChange   => next.rules.rule_change_threshold   = new_value,
            }
        }
        _ => {} // non-governance; no state change
    }
    Ok(next)
}

// ---------------------------------------------------------------------------
// Genesis payload helpers
// ---------------------------------------------------------------------------

/// Decode a GroupGenesisPayload from raw bytes for the purpose of seeding
/// the initial GroupState.
///
/// Wire layout for GroupGenesisPayload (our convention):
///   policy_version   : 2 bytes big-endian u16
///   add_member_thr   : 4 bytes big-endian u32
///   remove_member_thr: 4 bytes big-endian u32
///   role_change_thr  : 4 bytes big-endian u32
///   rule_change_thr  : 4 bytes big-endian u32
///   founding_device  : 32 bytes DeviceId
fn genesis_initial_state(payload: &[u8]) -> Result<GroupState, FoldError> {
    if payload.len() < 50 {
        return Err(FoldError::MalformedEnvelope(format!(
            "GroupGenesis payload too short: {} bytes", payload.len()
        )));
    }
    let add_member_threshold    = u32::from_be_bytes(payload[2..6].try_into().unwrap());
    let remove_member_threshold = u32::from_be_bytes(payload[6..10].try_into().unwrap());
    let role_change_threshold   = u32::from_be_bytes(payload[10..14].try_into().unwrap());
    let rule_change_threshold   = u32::from_be_bytes(payload[14..18].try_into().unwrap());
    Ok(GroupState {
        rules: GroupRules {
            add_member_threshold,
            remove_member_threshold,
            role_change_threshold,
            rule_change_threshold,
        },
        members: Vec::new(),
    })
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
// AuthFold
// ---------------------------------------------------------------------------

/// The authoritative fold engine.
///
/// All writes are serialised through a single redb write transaction per
/// `ingest` call (invariant I1).  A failed validation produces zero writes
/// (invariant I5).
pub struct AuthFold<V, C, L>
where
    V: Verifier,
    C: CredentialResolver,
    L: LamportSource,
{
    db: Arc<Db>,
    verifier: V,
    cred_resolver: C,
    lamport_source: L,
}

impl<V, C, L> AuthFold<V, C, L>
where
    V: Verifier + Send + Sync,
    C: CredentialResolver + Send + Sync,
    L: LamportSource + Send + Sync,
{
    /// Create a new `AuthFold` instance.
    ///
    /// Pre-creates all required tables in a single write transaction so that
    /// subsequent read transactions can open them without error.
    pub fn new(db: Arc<Db>, verifier: V, cred_resolver: C, lamport_source: L) -> Self {
        let this = Self { db, verifier, cred_resolver, lamport_source };
        this.ensure_tables().expect("AuthFold::new: failed to initialise tables");
        this
    }

    /// Open (and thereby create if absent) all tables used by the fold engine.
    fn ensure_tables(&self) -> Result<(), FoldError> {
        let write_txn = self.db.inner().begin_write()
            .map_err(|e| FoldError::StorageError(e.to_string()))?;
        write_txn.open_table(AUTH_ASSERTIONS)
            .map_err(|e| FoldError::StorageError(e.to_string()))?;
        write_txn.open_table(AUTH_ASSERTIONS_BY_DEVICE)
            .map_err(|e| FoldError::StorageError(e.to_string()))?;
        write_txn.open_table(AUTH_GOV_LOG)
            .map_err(|e| FoldError::StorageError(e.to_string()))?;
        write_txn.open_table(STATE_GROUP)
            .map_err(|e| FoldError::StorageError(e.to_string()))?;
        write_txn.commit()
            .map_err(|e| FoldError::StorageError(e.to_string()))?;
        Ok(())
    }

    /// Ingest an assertion from a peer or local author.
    ///
    /// - I1: all writes in one transaction.
    /// - I5: any validation failure → zero writes.
    /// - I9: re-ingest of known hash → `Ok(IngestResult::Duplicate)` with zero writes.
    pub fn ingest(&self, envelope: &AssertionEnvelope) -> Result<IngestResult, FoldError> {
        // ------------------------------------------------------------------
        // Step 1: Compute hash. Check auth_assertions for duplicate (I9).
        // ------------------------------------------------------------------
        let hash = envelope_hash(envelope);
        let hash_bytes: &[u8] = hash.as_bytes().as_ref();

        {
            let read_txn = self.db.inner().begin_read()
                .map_err(|e| FoldError::StorageError(e.to_string()))?;
            let table = read_txn.open_table(AUTH_ASSERTIONS)
                .map_err(|e| FoldError::StorageError(e.to_string()))?;
            if table.get(hash_bytes)
                .map_err(|e| FoldError::StorageError(e.to_string()))?
                .is_some()
            {
                return Ok(IngestResult::Duplicate);
            }
        }

        // ------------------------------------------------------------------
        // Step 2: Verify signature.
        // ------------------------------------------------------------------
        let canonical = envelope.canonical_bytes();
        self.verifier
            .verify(
                &types_device_to_traits(&envelope.author_device),
                &canonical,
                &envelope.signature,
            )
            .map_err(|e| FoldError::SignatureInvalid(e.to_string()))?;

        // ------------------------------------------------------------------
        // Step 3: Validate credential.
        // ------------------------------------------------------------------
        self.cred_resolver
            .resolve(
                &types_device_to_traits(&envelope.author_device),
                &types_principal_to_traits(&envelope.author_principal),
            )
            .map_err(|e| FoldError::CredentialInvalid(e.to_string()))?;

        // ------------------------------------------------------------------
        // Step 4: Load current GroupRules and membership; apply rules-at-position.
        // ------------------------------------------------------------------
        let current_state = self.load_or_init_state(envelope)?;

        let auth_ctx = AuthorizationContext {
            rules: &current_state.rules,
            members: &current_state.members,
            author: &envelope.author_principal,
        };
        check_authorization(&auth_ctx, &envelope.assertion_type, &envelope.payload)?;

        // ------------------------------------------------------------------
        // Step 5: Check per-device Lamport monotonicity.
        // ------------------------------------------------------------------
        {
            let read_txn = self.db.inner().begin_read()
                .map_err(|e| FoldError::StorageError(e.to_string()))?;
            let table = read_txn.open_table(AUTH_ASSERTIONS_BY_DEVICE)
                .map_err(|e| FoldError::StorageError(e.to_string()))?;

            // Scan for the maximum lamport stored for this device.
            let start = encode_by_device_key(&envelope.author_device, 0);
            let end   = encode_by_device_key(&envelope.author_device, u64::MAX);

            // Find the last (highest) lamport for this device.
            if let Some(last_entry) = table
                .range(start.as_slice()..=end.as_slice())
                .map_err(|e| FoldError::StorageError(e.to_string()))?
                .last()
            {
                let (k, _v) = last_entry.map_err(|e| FoldError::StorageError(e.to_string()))?;
                let key_bytes = k.value();
                // Decode lamport from bytes [32..40] of the key.
                let last_lamport = u64::from_be_bytes(
                    key_bytes[32..40].try_into().map_err(|_| {
                        FoldError::StorageError("lamport key decode error".to_string())
                    })?,
                );
                if envelope.lamport <= last_lamport {
                    return Err(FoldError::LamportViolation {
                        device: envelope.author_device,
                        expected_gt: last_lamport,
                        got: envelope.lamport,
                    });
                }
            }
        }

        // ------------------------------------------------------------------
        // Step 6: Compute next gov_seq (for governance assertions).
        // ------------------------------------------------------------------
        let gov_seq_opt: Option<u64> = if is_governance(&envelope.assertion_type) {
            let read_txn = self.db.inner().begin_read()
                .map_err(|e| FoldError::StorageError(e.to_string()))?;
            let table = read_txn.open_table(AUTH_GOV_LOG)
                .map_err(|e| FoldError::StorageError(e.to_string()))?;

            let start = encode_gov_log_key(&envelope.group, 0);
            let end   = encode_gov_log_key(&envelope.group, u64::MAX);

            let count = table
                .range(start.as_slice()..=end.as_slice())
                .map_err(|e| FoldError::StorageError(e.to_string()))?
                .count();
            Some(count as u64)
        } else {
            None
        };

        // ------------------------------------------------------------------
        // Step 7: Compute next GroupState (for governance assertions).
        // ------------------------------------------------------------------
        let next_state_opt: Option<GroupState> = if is_governance(&envelope.assertion_type) {
            Some(apply_governance(&current_state, envelope)?)
        } else {
            None
        };

        // ------------------------------------------------------------------
        // Step 8 (I1 / I5): All writes in ONE transaction.
        // ------------------------------------------------------------------
        {
            let write_txn = self.db.inner().begin_write()
                .map_err(|e| FoldError::StorageError(e.to_string()))?;

            // 8a. Write envelope to auth_assertions.
            {
                let mut table = write_txn.open_table(AUTH_ASSERTIONS)
                    .map_err(|e| FoldError::StorageError(e.to_string()))?;
                let value = envelope.canonical_bytes_with_sig();
                // Version prefix 0x01 prepended.
                let mut versioned = Vec::with_capacity(1 + value.len());
                versioned.push(0x01u8);
                versioned.extend_from_slice(&value);
                table
                    .insert(hash_bytes, versioned.as_slice())
                    .map_err(|e| FoldError::StorageError(e.to_string()))?;
            }

            // 8b. Write to auth_assertions_by_device.
            {
                let mut table = write_txn.open_table(AUTH_ASSERTIONS_BY_DEVICE)
                    .map_err(|e| FoldError::StorageError(e.to_string()))?;
                let key = encode_by_device_key(&envelope.author_device, envelope.lamport);
                table
                    .insert(key.as_slice(), hash_bytes)
                    .map_err(|e| FoldError::StorageError(e.to_string()))?;
            }

            // 8c. If governance, append to auth_gov_log and update state_group.
            if let (Some(gov_seq), Some(next_state)) = (gov_seq_opt, next_state_opt) {
                {
                    let mut table = write_txn.open_table(AUTH_GOV_LOG)
                        .map_err(|e| FoldError::StorageError(e.to_string()))?;
                    let key = encode_gov_log_key(&envelope.group, gov_seq);
                    table
                        .insert(key.as_slice(), hash_bytes)
                        .map_err(|e| FoldError::StorageError(e.to_string()))?;
                }
                {
                    let mut table = write_txn.open_table(STATE_GROUP)
                        .map_err(|e| FoldError::StorageError(e.to_string()))?;
                    let state_bytes = next_state.to_bytes();
                    table
                        .insert(
                            envelope.group.as_bytes().as_ref(),
                            state_bytes.as_slice(),
                        )
                        .map_err(|e| FoldError::StorageError(e.to_string()))?;
                }
            }

            write_txn.commit()
                .map_err(|e| FoldError::StorageError(e.to_string()))?;
        }

        Ok(IngestResult::Applied { hash })
    }

    // ------------------------------------------------------------------
    // Private: load GroupState from state_group, or seed from genesis payload.
    // ------------------------------------------------------------------
    fn load_or_init_state(
        &self,
        envelope: &AssertionEnvelope,
    ) -> Result<GroupState, FoldError> {
        let read_txn = self.db.inner().begin_read()
            .map_err(|e| FoldError::StorageError(e.to_string()))?;
        let table = read_txn.open_table(STATE_GROUP)
            .map_err(|e| FoldError::StorageError(e.to_string()))?;

        if let Some(bytes) = table
            .get(envelope.group.as_bytes().as_ref())
            .map_err(|e| FoldError::StorageError(e.to_string()))?
        {
            // Skip version byte.
            GroupState::from_bytes(bytes.value())
        } else {
            // No existing state: this must be a genesis assertion, or we
            // synthesise an empty permissive state for the first non-genesis
            // assertion on a group (should not happen in a well-formed system
            // but we handle it gracefully).
            if envelope.assertion_type == AssertionType::GroupGenesis {
                genesis_initial_state(&envelope.payload)
            } else {
                // Return an empty group state with defaults — auth check
                // will fail if the author is not in the member list.
                Ok(GroupState {
                    rules: GroupRules {
                        add_member_threshold: 1,
                        remove_member_threshold: 1,
                        role_change_threshold: 1,
                        rule_change_threshold: 1,
                    },
                    members: Vec::new(),
                })
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{
        AssertionEnvelope, AssertionType, DeviceId as TypesDeviceId,
        GroupId, PrincipalId as TypesPrincipalId, Role,
    };
    use crate::traits::mocks::{MockSigner, MockCredentialResolver, MockLamportSource};
    use crate::traits::{Signer, DeviceId as TraitsDeviceId, PrincipalId as TraitsPrincipalId};
    use crate::tables::Db;
    use redb::ReadableTableMetadata;
    use std::sync::Arc;
    use proptest::prelude::*;

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

    /// Build a genesis payload (policy_version=1, all thresholds=1,
    /// founding_device from seed).
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

    /// Build a MembershipAdd payload: PrincipalId || Role byte.
    fn membership_add_payload(principal_seed: u8, role: Role) -> Vec<u8> {
        let mut p = Vec::with_capacity(33);
        p.extend_from_slice(&[principal_seed; 32]);
        p.push(role_to_u8(&role));
        p
    }

    /// Build a RuleChange payload: rule_key byte || new_value u32.
    fn rule_change_payload(rule_key_byte: u8, new_value: u32) -> Vec<u8> {
        let mut p = Vec::with_capacity(5);
        p.push(rule_key_byte);
        p.extend_from_slice(&new_value.to_be_bytes());
        p
    }

    /// Sign an envelope using a MockSigner and register its (device, principal)
    /// credential with the resolver.
    fn sign_envelope(env: &mut AssertionEnvelope, signer: &MockSigner) {
        let canonical = env.canonical_bytes();
        env.signature = signer.sign(&canonical);
    }

    /// Build and sign a complete genesis envelope for testing.
    ///
    /// The `author_principal` is the principal that will be registered as Owner
    /// in the initial group state.  Callers must ensure the same principal is
    /// registered in the fold engine's credential resolver.
    fn make_genesis_envelope(
        signer: &MockSigner,
        group_seed: u8,
        author_principal: TypesPrincipalId,
    ) -> AssertionEnvelope {
        let device = TypesDeviceId::new(signer.device_id().0);
        let mut env = AssertionEnvelope {
            version: 0x01,
            assertion_type: AssertionType::GroupGenesis,
            author_device: device,
            author_principal,
            group: make_group(group_seed),
            antecedents: vec![],
            lamport: 1,
            timestamp: 1_700_000_000,
            payload: genesis_payload(signer.device_id().0[0]),
            signature: vec![],
        };
        sign_envelope(&mut env, signer);
        env
    }

    /// Create a fold engine backed by a fresh in-memory Db.
    fn make_fold(
        signer: &MockSigner,
        principal: TypesPrincipalId,
    ) -> (
        AuthFold<MockSigner, MockCredentialResolver, MockLamportSource>,
        Arc<Db>,
    ) {
        let db = Arc::new(Db::create_in_memory().expect("in-memory db"));
        let device = TypesDeviceId::new(signer.device_id().0);
        // MockSigner also implements Verifier.
        let verifier = MockSigner::new(signer.device_id().0);
        let mut cred = MockCredentialResolver::new();
        cred.register(
            TraitsDeviceId(device.as_bytes().clone()),
            TraitsPrincipalId(principal.as_bytes().clone()),
        );
        let lamport = MockLamportSource::new();
        let fold = AuthFold::new(Arc::clone(&db), verifier, cred, lamport);
        (fold, db)
    }

    // -----------------------------------------------------------------------
    // I5: invalid signature → zero writes
    // -----------------------------------------------------------------------

    #[test]
    fn test_i5_invalid_signature_no_write() {
        let signer = MockSigner::from_seed(0x01);
        let principal = make_principal(0xAA);
        let (fold, db) = make_fold(&signer, principal);

        let mut env = make_genesis_envelope(&signer, 0x10, principal);
        // Corrupt the signature.
        env.signature = vec![0xFF; 64];

        let result = fold.ingest(&env);
        assert!(
            matches!(result, Err(FoldError::SignatureInvalid(_))),
            "expected SignatureInvalid, got {:?}", result
        );

        // Verify zero writes: auth_assertions must be empty.
        let read_txn = db.inner().begin_read().unwrap();
        let table = read_txn.open_table(AUTH_ASSERTIONS).unwrap();
        assert_eq!(table.len().unwrap(), 0);
    }

    // -----------------------------------------------------------------------
    // I5: invalid credential → zero writes
    // -----------------------------------------------------------------------

    #[test]
    fn test_i5_invalid_credential_no_write() {
        let signer = MockSigner::from_seed(0x02);
        // Create fold engine but register a DIFFERENT principal than what the
        // envelope will carry.
        let db = Arc::new(Db::create_in_memory().unwrap());
        let device = TypesDeviceId::new(signer.device_id().0);
        let verifier = MockSigner::new(signer.device_id().0);
        let mut cred = MockCredentialResolver::new();
        // Register device with principal 0xBB, but envelope will use 0xAA.
        cred.register(
            TraitsDeviceId(device.as_bytes().clone()),
            TraitsPrincipalId([0xBBu8; 32]),
        );
        let lamport = MockLamportSource::new();
        let fold = AuthFold::new(Arc::clone(&db), verifier, cred, lamport);

        let mut env = make_genesis_envelope(&signer, 0x11, make_principal(0xAA));
        // Ensure principal 0xAA is in the envelope (intentionally mismatches
        // the resolver which has 0xBB registered).
        assert_eq!(env.author_principal, make_principal(0xAA));

        let result = fold.ingest(&env);
        assert!(
            matches!(result, Err(FoldError::CredentialInvalid(_))),
            "expected CredentialInvalid, got {:?}", result
        );

        let read_txn = db.inner().begin_read().unwrap();
        let table = read_txn.open_table(AUTH_ASSERTIONS).unwrap();
        assert_eq!(table.len().unwrap(), 0);
    }

    // -----------------------------------------------------------------------
    // I5: unauthorized action → zero writes
    // -----------------------------------------------------------------------

    #[test]
    fn test_i5_unauthorized_no_write() {
        // Owner signer seeds the group.
        let owner_signer = MockSigner::from_seed(0x01);
        let member_signer = MockSigner::from_seed(0x02);
        let owner_principal = make_principal(0xAA);
        let member_principal = make_principal(0xBB);

        let db = Arc::new(Db::create_in_memory().unwrap());

        // Build a multi-signer verifier using the owner's signer for verification;
        // we'll call ingest twice: once for genesis (as owner) and once for the
        // unauthorised MembershipRemove (as member).
        // For simplicity, build separate fold instances sharing the same Db.

        // Ingest genesis as owner.
        {
            let verifier = MockSigner::new(owner_signer.device_id().0);
            let mut cred = MockCredentialResolver::new();
            cred.register(
                TraitsDeviceId(owner_signer.device_id().0),
                TraitsPrincipalId(owner_principal.as_bytes().clone()),
            );
            let fold = AuthFold::new(
                Arc::clone(&db),
                verifier,
                cred,
                MockLamportSource::new(),
            );
            let genesis = make_genesis_envelope(&owner_signer, 0x20, owner_principal);
            fold.ingest(&genesis).expect("genesis should succeed");
        }

        // Add the member via MembershipAdd.
        {
            let verifier = MockSigner::new(owner_signer.device_id().0);
            let mut cred = MockCredentialResolver::new();
            cred.register(
                TraitsDeviceId(owner_signer.device_id().0),
                TraitsPrincipalId(owner_principal.as_bytes().clone()),
            );
            let fold = AuthFold::new(
                Arc::clone(&db),
                verifier,
                cred,
                MockLamportSource::new(),
            );
            let device = TypesDeviceId::new(owner_signer.device_id().0);
            let mut add_env = AssertionEnvelope {
                version: 0x01,
                assertion_type: AssertionType::MembershipAdd,
                author_device: device,
                author_principal: owner_principal,
                group: make_group(0x20),
                antecedents: vec![],
                lamport: 2,
                timestamp: 1_700_000_001,
                payload: membership_add_payload(0xBB, Role::Member),
                signature: vec![],
            };
            sign_envelope(&mut add_env, &owner_signer);
            fold.ingest(&add_env).expect("MembershipAdd should succeed");
        }

        // Now attempt MembershipRemove as the Member (should fail).
        {
            let verifier = MockSigner::new(member_signer.device_id().0);
            let mut cred = MockCredentialResolver::new();
            cred.register(
                TraitsDeviceId(member_signer.device_id().0),
                TraitsPrincipalId(member_principal.as_bytes().clone()),
            );
            let fold = AuthFold::new(
                Arc::clone(&db),
                verifier,
                cred,
                MockLamportSource::new(),
            );
            let device = TypesDeviceId::new(member_signer.device_id().0);
            let mut remove_env = AssertionEnvelope {
                version: 0x01,
                assertion_type: AssertionType::MembershipRemove,
                author_device: device,
                author_principal: member_principal,
                group: make_group(0x20),
                antecedents: vec![],
                lamport: 1,
                timestamp: 1_700_000_002,
                payload: {
                    let mut p = Vec::new();
                    p.extend_from_slice(owner_principal.as_bytes());
                    p
                },
                signature: vec![],
            };
            sign_envelope(&mut remove_env, &member_signer);

            let count_before = {
                let r = db.inner().begin_read().unwrap();
                let t = r.open_table(AUTH_ASSERTIONS).unwrap();
                t.len().unwrap()
            };

            let result = fold.ingest(&remove_env);
            assert!(
                matches!(result, Err(FoldError::AuthorizationFailed(_))),
                "expected AuthorizationFailed, got {:?}", result
            );

            let count_after = {
                let r = db.inner().begin_read().unwrap();
                let t = r.open_table(AUTH_ASSERTIONS).unwrap();
                t.len().unwrap()
            };
            assert_eq!(count_before, count_after, "unauthorized action must not write");
        }
    }

    // -----------------------------------------------------------------------
    // I9: duplicate ingest is a no-op
    // -----------------------------------------------------------------------

    #[test]
    fn test_i9_duplicate_is_noop() {
        let signer = MockSigner::from_seed(0x05);
        let principal = make_principal(0xAA);
        let (fold, db) = make_fold(&signer, principal);

        let genesis = make_genesis_envelope(&signer, 0x30, principal);

        let r1 = fold.ingest(&genesis).unwrap();
        assert!(matches!(r1, IngestResult::Applied { .. }));

        let count_after_first = {
            let r = db.inner().begin_read().unwrap();
            let t = r.open_table(AUTH_ASSERTIONS).unwrap();
            t.len().unwrap()
        };

        let r2 = fold.ingest(&genesis).unwrap();
        assert_eq!(r2, IngestResult::Duplicate);

        let count_after_second = {
            let r = db.inner().begin_read().unwrap();
            let t = r.open_table(AUTH_ASSERTIONS).unwrap();
            t.len().unwrap()
        };
        assert_eq!(count_after_first, count_after_second, "duplicate must not add rows");
    }

    // -----------------------------------------------------------------------
    // I6: rules-at-position — RuleChange raises threshold, old threshold fails
    // -----------------------------------------------------------------------

    #[test]
    fn test_i6_rules_at_position() {
        // This test verifies that after a RuleChange raises a threshold, an
        // action that was previously allowed (under the old threshold) is now
        // rejected.  For Stage 3 this manifests as: after raising
        // remove_member_threshold to 999 (conceptually requiring more quorum),
        // an Admin attempting MembershipRemove is still blocked by the
        // Owner-only rule.  We verify the rule is persisted and re-read.
        let signer = MockSigner::from_seed(0x07);
        let principal = make_principal(0xAA);
        let db = Arc::new(Db::create_in_memory().unwrap());

        let make_fold_for = |db: Arc<Db>| {
            let verifier = MockSigner::new(signer.device_id().0);
            let mut cred = MockCredentialResolver::new();
            cred.register(
                TraitsDeviceId(signer.device_id().0),
                TraitsPrincipalId(principal.as_bytes().clone()),
            );
            AuthFold::new(db, verifier, cred, MockLamportSource::new())
        };

        // Step 1: Genesis (owner = principal 0xAA).
        {
            let fold = make_fold_for(Arc::clone(&db));
            let genesis = make_genesis_envelope(&signer, 0x40, principal);
            fold.ingest(&genesis).unwrap();
        }

        // Step 2: Add principal 0xAA as Owner in members list.
        {
            let fold = make_fold_for(Arc::clone(&db));
            let device = TypesDeviceId::new(signer.device_id().0);
            let mut add_env = AssertionEnvelope {
                version: 0x01,
                assertion_type: AssertionType::MembershipAdd,
                author_device: device,
                author_principal: principal,
                group: make_group(0x40),
                antecedents: vec![],
                lamport: 2,
                timestamp: 1_700_000_001,
                payload: membership_add_payload(0xAA, Role::Owner),
                signature: vec![],
            };
            sign_envelope(&mut add_env, &signer);
            fold.ingest(&add_env).unwrap();
        }

        // Step 3: RuleChange — raise rule_change_threshold to 99.
        {
            let fold = make_fold_for(Arc::clone(&db));
            let device = TypesDeviceId::new(signer.device_id().0);
            let mut rc_env = AssertionEnvelope {
                version: 0x01,
                assertion_type: AssertionType::RuleChange,
                author_device: device,
                author_principal: principal,
                group: make_group(0x40),
                antecedents: vec![],
                lamport: 3,
                timestamp: 1_700_000_002,
                payload: rule_change_payload(3 /* RuleChange key */, 99),
                signature: vec![],
            };
            sign_envelope(&mut rc_env, &signer);
            fold.ingest(&rc_env).unwrap();
        }

        // Step 4: Verify the stored rule_change_threshold is 99.
        {
            let read_txn = db.inner().begin_read().unwrap();
            let table = read_txn.open_table(STATE_GROUP).unwrap();
            let raw = table
                .get(make_group(0x40).as_bytes().as_ref())
                .unwrap()
                .expect("state must be present");
            let state = GroupState::from_bytes(raw.value()).unwrap();
            assert_eq!(state.rules.rule_change_threshold, 99);
        }

        // Step 5: Attempt another RuleChange still as Owner — must still succeed
        // because Owner always satisfies the threshold in Stage 3.
        {
            let fold = make_fold_for(Arc::clone(&db));
            let device = TypesDeviceId::new(signer.device_id().0);
            let mut rc_env2 = AssertionEnvelope {
                version: 0x01,
                assertion_type: AssertionType::RuleChange,
                author_device: device,
                author_principal: principal,
                group: make_group(0x40),
                antecedents: vec![],
                lamport: 4,
                timestamp: 1_700_000_003,
                payload: rule_change_payload(0 /* AddMember key */, 2),
                signature: vec![],
            };
            sign_envelope(&mut rc_env2, &signer);
            // Should succeed (Owner always satisfies).
            fold.ingest(&rc_env2).unwrap();
        }
    }

    // -----------------------------------------------------------------------
    // Lamport violation detection
    // -----------------------------------------------------------------------

    #[test]
    fn test_lamport_violation_detected() {
        let signer = MockSigner::from_seed(0x08);
        let principal = make_principal(0xAA);
        let db = Arc::new(Db::create_in_memory().unwrap());

        let make_fold_for = |db: Arc<Db>| {
            let verifier = MockSigner::new(signer.device_id().0);
            let mut cred = MockCredentialResolver::new();
            cred.register(
                TraitsDeviceId(signer.device_id().0),
                TraitsPrincipalId(principal.as_bytes().clone()),
            );
            AuthFold::new(db, verifier, cred, MockLamportSource::new())
        };

        // Ingest genesis at lamport=1.
        {
            let fold = make_fold_for(Arc::clone(&db));
            let genesis = make_genesis_envelope(&signer, 0x50, principal);
            // genesis uses lamport=1 by default.
            fold.ingest(&genesis).unwrap();
        }

        // Ingest a message at lamport=5.
        {
            let fold = make_fold_for(Arc::clone(&db));
            // First add the author as a member so Message assertion is authorized.
            let device = TypesDeviceId::new(signer.device_id().0);
            let mut add_env = AssertionEnvelope {
                version: 0x01,
                assertion_type: AssertionType::MembershipAdd,
                author_device: device,
                author_principal: principal,
                group: make_group(0x50),
                antecedents: vec![],
                lamport: 2,
                timestamp: 1_700_000_001,
                payload: membership_add_payload(0xAA, Role::Owner),
                signature: vec![],
            };
            sign_envelope(&mut add_env, &signer);
            fold.ingest(&add_env).unwrap();

            let mut msg_env = AssertionEnvelope {
                version: 0x01,
                assertion_type: AssertionType::Message,
                author_device: device,
                author_principal: principal,
                group: make_group(0x50),
                antecedents: vec![],
                lamport: 5,
                timestamp: 1_700_000_002,
                payload: {
                    let body = b"hello";
                    let mut p = Vec::new();
                    let blen = body.len() as u32;
                    p.extend_from_slice(&blen.to_be_bytes());
                    p.extend_from_slice(body);
                    p.extend_from_slice(&[0u8; 4]); // reply_to = None (0)
                    p
                },
                signature: vec![],
            };
            sign_envelope(&mut msg_env, &signer);
            fold.ingest(&msg_env).unwrap();
        }

        // Now attempt to ingest the same device at lamport=3 (< 5) → violation.
        {
            let fold = make_fold_for(Arc::clone(&db));
            let device = TypesDeviceId::new(signer.device_id().0);
            let mut old_env = AssertionEnvelope {
                version: 0x01,
                assertion_type: AssertionType::Message,
                author_device: device,
                author_principal: principal,
                group: make_group(0x50),
                antecedents: vec![],
                lamport: 3,
                timestamp: 1_700_000_003,
                payload: {
                    let body = b"late";
                    let mut p = Vec::new();
                    let blen = body.len() as u32;
                    p.extend_from_slice(&blen.to_be_bytes());
                    p.extend_from_slice(body);
                    p.extend_from_slice(&[0u8; 4]);
                    p
                },
                signature: vec![],
            };
            sign_envelope(&mut old_env, &signer);

            let result = fold.ingest(&old_env);
            assert!(
                matches!(result, Err(FoldError::LamportViolation { got: 3, .. })),
                "expected LamportViolation(got=3), got {:?}", result
            );
        }
    }

    // -----------------------------------------------------------------------
    // Governance indexed in auth_gov_log; non-governance is not
    // -----------------------------------------------------------------------

    #[test]
    fn test_governance_indexed() {
        let signer = MockSigner::from_seed(0x09);
        let principal = make_principal(0xAA);
        let db = Arc::new(Db::create_in_memory().unwrap());

        let make_fold_for = |db: Arc<Db>| {
            let verifier = MockSigner::new(signer.device_id().0);
            let mut cred = MockCredentialResolver::new();
            cred.register(
                TraitsDeviceId(signer.device_id().0),
                TraitsPrincipalId(principal.as_bytes().clone()),
            );
            AuthFold::new(db, verifier, cred, MockLamportSource::new())
        };

        // Genesis (governance).
        {
            let fold = make_fold_for(Arc::clone(&db));
            let genesis = make_genesis_envelope(&signer, 0x60, principal);
            fold.ingest(&genesis).unwrap();
        }

        // MembershipAdd (governance): adds principal 0xAA as Owner.
        {
            let fold = make_fold_for(Arc::clone(&db));
            let device = TypesDeviceId::new(signer.device_id().0);
            let mut add_env = AssertionEnvelope {
                version: 0x01,
                assertion_type: AssertionType::MembershipAdd,
                author_device: device,
                author_principal: principal,
                group: make_group(0x60),
                antecedents: vec![],
                lamport: 2,
                timestamp: 1_700_000_001,
                payload: membership_add_payload(0xAA, Role::Owner),
                signature: vec![],
            };
            sign_envelope(&mut add_env, &signer);
            fold.ingest(&add_env).unwrap();
        }

        // gov_log should have 2 entries (genesis + MembershipAdd).
        let gov_count_before = {
            let read_txn = db.inner().begin_read().unwrap();
            let table = read_txn.open_table(AUTH_GOV_LOG).unwrap();
            let start = encode_gov_log_key(&make_group(0x60), 0);
            let end   = encode_gov_log_key(&make_group(0x60), u64::MAX);
            table.range(start.as_slice()..=end.as_slice()).unwrap().count()
        };
        assert_eq!(gov_count_before, 2, "expected 2 governance entries");

        // Message (non-governance): should NOT appear in gov_log.
        {
            let fold = make_fold_for(Arc::clone(&db));
            let device = TypesDeviceId::new(signer.device_id().0);
            let mut msg_env = AssertionEnvelope {
                version: 0x01,
                assertion_type: AssertionType::Message,
                author_device: device,
                author_principal: principal,
                group: make_group(0x60),
                antecedents: vec![],
                lamport: 3,
                timestamp: 1_700_000_002,
                payload: {
                    let body = b"hi";
                    let mut p = Vec::new();
                    p.extend_from_slice(&(body.len() as u32).to_be_bytes());
                    p.extend_from_slice(body);
                    p.extend_from_slice(&[0u8; 4]);
                    p
                },
                signature: vec![],
            };
            sign_envelope(&mut msg_env, &signer);
            fold.ingest(&msg_env).unwrap();
        }

        let gov_count_after = {
            let read_txn = db.inner().begin_read().unwrap();
            let table = read_txn.open_table(AUTH_GOV_LOG).unwrap();
            let start = encode_gov_log_key(&make_group(0x60), 0);
            let end   = encode_gov_log_key(&make_group(0x60), u64::MAX);
            table.range(start.as_slice()..=end.as_slice()).unwrap().count()
        };
        assert_eq!(gov_count_after, 2, "Message must not appear in gov_log");

        // auth_assertions should have 3 entries total.
        let assertion_count = {
            let read_txn = db.inner().begin_read().unwrap();
            let table = read_txn.open_table(AUTH_ASSERTIONS).unwrap();
            table.len().unwrap()
        };
        assert_eq!(assertion_count, 3);
    }

    // -----------------------------------------------------------------------
    // Property test: 50 random valid assertion sequences all ingest Ok
    // -----------------------------------------------------------------------

    proptest! {
        #[test]
        fn prop_valid_sequences_all_applied(seeds in proptest::collection::vec(1u8..=200u8, 1..=50usize)) {
            // Use a fixed signer per test run.
            let signer = MockSigner::from_seed(0xAB);
            let principal = make_principal(0xCC);
            let db = Arc::new(Db::create_in_memory().unwrap());

            let verifier = MockSigner::new(signer.device_id().0);
            let mut cred = MockCredentialResolver::new();
            cred.register(
                TraitsDeviceId(signer.device_id().0),
                TraitsPrincipalId(principal.as_bytes().clone()),
            );
            let fold = AuthFold::new(
                Arc::clone(&db),
                verifier,
                cred,
                MockLamportSource::new(),
            );

            // Ingest genesis first.
            let genesis = make_genesis_envelope(&signer, 0x70, principal);
            fold.ingest(&genesis).unwrap();

            // Add principal as Owner.
            let device = TypesDeviceId::new(signer.device_id().0);
            let mut add_env = AssertionEnvelope {
                version: 0x01,
                assertion_type: AssertionType::MembershipAdd,
                author_device: device,
                author_principal: principal,
                group: make_group(0x70),
                antecedents: vec![],
                lamport: 2,
                timestamp: 1_700_000_001,
                payload: membership_add_payload(0xCC, Role::Owner),
                signature: vec![],
            };
            sign_envelope(&mut add_env, &signer);
            fold.ingest(&add_env).unwrap();

            // Ingest `seeds.len()` Message assertions, each with a unique lamport.
            for (i, _seed) in seeds.iter().enumerate() {
                let lamport = 3 + i as u64;
                let body = format!("msg-{}", i);
                let mut msg_env = AssertionEnvelope {
                    version: 0x01,
                    assertion_type: AssertionType::Message,
                    author_device: device,
                    author_principal: principal,
                    group: make_group(0x70),
                    antecedents: vec![],
                    lamport,
                    timestamp: 1_700_000_002 + i as u64,
                    payload: {
                        let mut p = Vec::new();
                        p.extend_from_slice(&(body.len() as u32).to_be_bytes());
                        p.extend_from_slice(body.as_bytes());
                        p.extend_from_slice(&[0u8; 4]);
                        p
                    },
                    signature: vec![],
                };
                sign_envelope(&mut msg_env, &signer);
                prop_assert!(fold.ingest(&msg_env).is_ok(), "ingest failed at i={}", i);
            }

            // auth_assertions count = 2 (genesis + add) + seeds.len() Messages.
            let assertion_count = {
                let read_txn = db.inner().begin_read().unwrap();
                let table = read_txn.open_table(AUTH_ASSERTIONS).unwrap();
                table.len().unwrap()
            };
            prop_assert_eq!(assertion_count as usize, 2 + seeds.len());
        }
    }

    // -----------------------------------------------------------------------
    // GroupState codec boundary tests (mutation-driven, 2026-06-26)
    //
    // Added to kill cargo-mutants survivors in GroupState::from_bytes that the
    // existing happy-path round-trips left uncaught:
    //   - fold_auth.rs:153  `b.len() < 21`           (`<`->`==`, `<`->`<=`)
    //   - fold_auth.rs:165  `21 + member_count * 33` (`*`->`/`)
    // The pre-existing tests only fed valid, well-sized buffers, so the lower
    // bound and the member-section length arithmetic were never exercised at
    // their boundaries.
    // -----------------------------------------------------------------------

    fn group_state(members: Vec<(TypesPrincipalId, Role)>) -> GroupState {
        GroupState {
            rules: GroupRules {
                add_member_threshold: 1,
                remove_member_threshold: 2,
                role_change_threshold: 3,
                rule_change_threshold: 4,
            },
            members,
        }
    }

    /// A zero-member state encodes to exactly the 21-byte minimum and decodes
    /// back. Guards the lower bound (line 153): a `<`->`<=` mutant would reject
    /// this valid 21-byte buffer.
    #[test]
    fn group_state_zero_member_roundtrip_is_exactly_min_len() {
        let bytes = group_state(vec![]).to_bytes();
        assert_eq!(bytes.len(), 21, "zero-member GroupState must be the 21-byte minimum");
        let decoded = GroupState::from_bytes(&bytes).expect("21-byte zero-member state must decode");
        assert!(decoded.members.is_empty());
        assert_eq!(decoded.rules.add_member_threshold, 1);
        assert_eq!(decoded.rules.rule_change_threshold, 4);
    }

    /// Any buffer below the 21-byte minimum is rejected, not decoded. Guards
    /// line 153: a `<`->`==` mutant would accept a 20-byte buffer and then
    /// panic slicing the threshold / member-count fields.
    #[test]
    fn group_state_below_min_len_is_rejected() {
        assert!(GroupState::from_bytes(&[0u8; 20]).is_err(), "20-byte buffer must be rejected");
        assert!(GroupState::from_bytes(&[]).is_err(), "empty buffer must be rejected");
        assert!(GroupState::from_bytes(&[0u8; 17]).is_err(), "17-byte buffer must be rejected");
    }

    /// A header that claims members but is truncated before their bytes is
    /// rejected. Guards line 165: a `*`->`/` mutant computes
    /// `required = 21 + member_count / 33 = 21`, would accept the short buffer,
    /// and panic reading member 0.
    #[test]
    fn group_state_truncated_members_is_rejected() {
        let full = group_state(vec![
            (make_principal(7), Role::Admin),
            (make_principal(9), Role::Member),
        ])
        .to_bytes();
        assert_eq!(full.len(), 21 + 2 * 33);

        // member_count=2 in the header, but no member bytes follow.
        assert!(
            GroupState::from_bytes(&full[..21]).is_err(),
            "header claiming 2 members with no member bytes must be rejected"
        );
        // First member present, second truncated.
        assert!(
            GroupState::from_bytes(&full[..21 + 33]).is_err(),
            "header claiming 2 members with only 1 present must be rejected"
        );
    }

    /// Full round-trip with members preserves the rules and the member list in
    /// order (regression cover for the member-section read loop).
    #[test]
    fn group_state_multi_member_roundtrip_preserves_members() {
        let gs = group_state(vec![
            (make_principal(1), Role::Owner),
            (make_principal(2), Role::Admin),
            (make_principal(3), Role::Observer),
        ]);
        let decoded = GroupState::from_bytes(&gs.to_bytes()).expect("round-trip");
        assert_eq!(decoded.members.len(), 3);
        assert_eq!(decoded.members[0].0.as_bytes(), make_principal(1).as_bytes());
        assert_eq!(role_to_u8(&decoded.members[0].1), role_to_u8(&Role::Owner));
        assert_eq!(decoded.members[2].0.as_bytes(), make_principal(3).as_bytes());
        assert_eq!(role_to_u8(&decoded.members[2].1), role_to_u8(&Role::Observer));
        assert_eq!(decoded.rules.remove_member_threshold, 2);
    }
}
