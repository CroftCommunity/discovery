// Included via `mod common;` by several test binaries; each compiles the module
// independently, so helpers used by only some tests read as dead code in others.
#![allow(dead_code)]

//! Shared scaffolding for the Battery 5 completeness experiments (G1/G2/V3′).
//!
//! Lets a test hand-craft signed assertions (including RuleChange / cross-device
//! chains the `Session` API does not expose), deliver a chosen subset of them
//! through the REAL `Replicator` + `Session` + fold, and read back the derived
//! governance state — so a dropped or duplicated frame is a first-class knob.
//!
//! This scaffolding builds signed assertions directly and lets a test drop / duplicate /
//! reorder frames — the *adversarial* delivery a well-behaved emit API must refuse to
//! produce, so it can only be hand-built. Well-formed governance facts (RuleChange /
//! Approval) are now emittable through the real `Session` API (see
//! `rulechange_quorum_via_api.rs`); this module is for refutation-style and focused
//! fold-level tests, not an API gap. (Reconciled: SPEC-DIVERGENCE-REGISTER.md.)

use std::sync::Arc;

use croft_chat::sync::Replicator;
use croft_chat::transport::{Frame, Topic, Transport};
use local_storage_projection::fold_derived::{rule_change_approval_subject, DerivedFold};
use local_storage_projection::tables::Db;
use local_storage_projection::traits::Signer as _;
use local_storage_projection::{AssertionEnvelope, AssertionType, DeviceId, GroupId, Hash, PrincipalId};
use social_graph_core::{Ed25519Signer, Ed25519Verifier, Identity, RegistryCredentialResolver, Session};

/// A transport whose inbox the test preloads directly, so it controls exactly
/// which frames each node sees. Nodes here only receive; `publish` is unused.
#[derive(Default)]
pub struct QueueBus {
    inbox: Vec<Frame>,
}

impl QueueBus {
    pub fn inject(&mut self, frames: Vec<Frame>) {
        self.inbox.extend(frames);
    }
}

impl Transport for QueueBus {
    fn subscribe(&mut self, _topic: &Topic) {}
    fn publish(&mut self, _topic: &Topic, _frame: Frame) {}
    fn drain(&mut self) -> Vec<Frame> {
        std::mem::take(&mut self.inbox)
    }
}

/// The wire frame the Replicator/Session consume: a version byte prepended to
/// `canonical_bytes_with_sig()` (mirrors what the fold stores and `export_group_log`
/// emits).
#[must_use]
pub fn frame(env: &AssertionEnvelope) -> Frame {
    let mut b = Vec::with_capacity(1 + env.payload.len() + 200);
    b.push(0x01);
    b.extend_from_slice(&env.canonical_bytes_with_sig());
    Frame(b)
}

#[must_use]
pub fn sign(identity: &Identity, mut env: AssertionEnvelope) -> AssertionEnvelope {
    let canonical = env.canonical_bytes();
    env.signature = Ed25519Signer::new(identity).sign(&canonical);
    env
}

#[must_use]
pub fn base(
    identity: &Identity,
    group: GroupId,
    ty: AssertionType,
    lamport: u64,
    antecedents: Vec<Hash>,
    payload: Vec<u8>,
) -> AssertionEnvelope {
    AssertionEnvelope {
        version: 0x01,
        assertion_type: ty,
        author_device: DeviceId::new(identity.device_id().0),
        author_principal: PrincipalId::new(identity.principal_id().0),
        group,
        antecedents,
        lamport,
        timestamp: 1_700_000_000 + lamport,
        payload,
        signature: vec![],
    }
}

/// Genesis payload matching `create_group`: policy_version + four thresholds
/// (all 1, so the Owner alone satisfies every gate) + the founding device.
#[must_use]
pub fn genesis_payload(device: &DeviceId) -> Vec<u8> {
    let mut p = Vec::with_capacity(50);
    p.extend_from_slice(&1u16.to_be_bytes());
    for _ in 0..4 {
        p.extend_from_slice(&1u32.to_be_bytes());
    }
    p.extend_from_slice(device.as_bytes());
    p
}

/// A signed `GroupGenesis` envelope for `identity` over `group` — the founding fact most
/// governance tests start from (thresholds all 1; see [`genesis_payload`]).
#[must_use]
pub fn signed_genesis(identity: &Identity, group: GroupId, lamport: u64) -> AssertionEnvelope {
    let device = DeviceId::new(identity.device_id().0);
    sign(
        identity,
        base(identity, group, AssertionType::GroupGenesis, lamport, vec![], genesis_payload(&device)),
    )
}

/// MembershipAdd payload: principal(32) ‖ role(1). Roles: 0 Owner, 1 Admin, 2 Member.
#[must_use]
pub fn membership_add_payload(subject: PrincipalId, role_byte: u8) -> Vec<u8> {
    let mut p = Vec::with_capacity(33);
    p.extend_from_slice(subject.as_bytes());
    p.push(role_byte);
    p
}

/// RuleChange payload: rule_key(1) ‖ new_value(4, BE). rule_key 0 == AddMember,
/// 1 == RemoveMember, 2 == RoleChange, 3 == RuleChange.
#[must_use]
pub fn rule_change_payload(rule_key: u8, new_value: u32) -> Vec<u8> {
    let mut p = Vec::with_capacity(5);
    p.push(rule_key);
    p.extend_from_slice(&new_value.to_be_bytes());
    p
}

/// Approval payload: approved act_type (2 bytes BE) ‖ subject (32 bytes). For membership
/// / role acts the subject is the target principal; for a RuleChange it is the change's
/// content hash — see [`rule_change_subject`].
#[must_use]
pub fn approval_payload(act_type: AssertionType, subject: PrincipalId) -> Vec<u8> {
    let mut p = act_type.to_u16().to_be_bytes().to_vec();
    p.extend_from_slice(subject.as_bytes());
    p
}

/// The approval subject a `RuleChange` with this payload must be approved under (the
/// content hash of the proposed change). Mirrors the fold's `act_subject` so a test's
/// approvals name exactly what the fold will look for.
#[must_use]
pub fn rule_change_subject(payload: &[u8]) -> PrincipalId {
    PrincipalId::new(rule_change_approval_subject(payload))
}

/// Pump the real Replicator until it quiesces (two consecutive rounds applying
/// nothing new, or a 50-round bound). Frames rejected for now (e.g. arrived
/// before their authority) stay in the Replicator's buffer and are retried.
pub fn pump_until_quiet(session: &Session, bus: &mut QueueBus, repl: &mut Replicator) {
    let mut quiet = 0;
    for _ in 0..50 {
        if repl.pump(session, bus) == 0 {
            quiet += 1;
            if quiet >= 2 {
                break;
            }
        } else {
            quiet = 0;
        }
    }
}

/// Open a session that trusts `authors`, inject `frames`, and pump to quiescence.
#[must_use]
pub fn drive(
    path: &std::path::Path,
    reader: &Identity,
    authors: &[&Identity],
    frames: Vec<Frame>,
) -> Session {
    let session = Session::open(path, reader).expect("open session");
    for a in authors {
        session.trust_peer(a.device_id(), a.principal_id());
    }
    let mut bus = QueueBus::default();
    bus.subscribe(&Topic("drystone/completeness".to_string()));
    bus.inject(frames);
    let mut repl = Replicator::new();
    pump_until_quiet(&session, &mut bus, &mut repl);
    session
}

/// Replicate `group`'s log from `from` into `to` by exporting its frames and pumping
/// them through the real `Replicator` (which buffers for per-device lamport order).
/// The end-to-end counterpart used to converge two live `Session`s in a test.
pub fn replicate(from: &Session, to: &Session, group: &GroupId) {
    let frames: Vec<Frame> = from
        .export_group_log(group)
        .expect("export group log")
        .into_iter()
        .map(Frame)
        .collect();
    let mut bus = QueueBus::default();
    bus.inject(frames);
    let mut repl = Replicator::new();
    pump_until_quiet(to, &mut bus, &mut repl);
}

/// True if `principal` is a member in the group's derived summary.
#[must_use]
pub fn has_member(session: &Session, group: &GroupId, principal: &PrincipalId) -> bool {
    session
        .get_group_summary(group)
        .map(|s| s.members.iter().any(|m| &m.principal == principal))
        .unwrap_or(false)
}

/// Ingest a sequence of pre-signed envelopes straight through a fresh `DerivedFold`
/// (registering every author's credential first), then reopen the store as a `Session`
/// for the `reader` to read back. The direct-ingest counterpart to [`drive`] — for
/// governance-fact tests that don't need the transport/Replicator path. Ingest outcomes
/// are logged to stderr so a rejection is visible under `--nocapture`.
#[must_use]
pub fn ingest_seq(
    path: &std::path::Path,
    authors: &[&Identity],
    reader: &Identity,
    envs: &[&AssertionEnvelope],
) -> Session {
    {
        let db = Arc::new(Db::open(path).expect("open db"));
        let resolver = RegistryCredentialResolver::new();
        for a in authors {
            resolver.register(a.device_id(), a.principal_id());
        }
        let fold = DerivedFold::new(Arc::clone(&db), Ed25519Verifier, resolver);
        for env in envs {
            let outcome = fold.ingest(env);
            eprintln!(
                "  ingest {:?} -> {:?}",
                env.assertion_type,
                outcome.map(|_| "ok").map_err(|e| e.to_string())
            );
        }
    }
    Session::open(path, reader).expect("open session")
}
