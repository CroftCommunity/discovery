//! Governance: a forward-only, signed log of membership operations evaluated
//! against immutable genesis rules.
//!
//! This is the "governance tree" of the two-tree model (thesis §1.1): where
//! "did this operation meet its threshold" is decided, grounded in signed data
//! alone. It is deliberately separate from the MLS epoch chain.
//!
//! Invariants exercised here:
//! * **I1 — genesis immutability.** The admin set and per-op thresholds are
//!   fixed at genesis and there is *no* op kind that can change them. Adding a
//!   member never confers standing (admin authority), so the "who decides who
//!   decides" regress is grounded at the root.
//! * **I2 — threshold soundness.** An op is enacted iff it carries signatures
//!   meeting the genesis threshold for its kind, from admins with standing in
//!   the current epoch. No under-threshold op is ever applied.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::ids::{Did, GenesisId};
use crate::keys::{Sig, SigningIdentity, VerifyingIdentity};

/// The membership operations the governance log can carry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum OpKind {
    Add,
    Remove,
    Leave,
    Dissolve,
    Fork,
    Recombine,
}

impl OpKind {
    fn tag(self) -> u8 {
        match self {
            OpKind::Add => 0,
            OpKind::Remove => 1,
            OpKind::Leave => 2,
            OpKind::Dissolve => 3,
            OpKind::Fork => 4,
            OpKind::Recombine => 5,
        }
    }
}

/// Immutable rules fixed at genesis. There is intentionally no operation that
/// mutates this structure — that is invariant I1, enforced structurally.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GenesisRules {
    /// The admin set: the only DIDs whose signatures count toward thresholds.
    /// Fixed forever at genesis; membership changes never alter it.
    pub admins: BTreeSet<Did>,
    /// Signatures required per op kind. Missing kinds default to 1.
    pub thresholds: BTreeMap<OpKind, u32>,
}

impl GenesisRules {
    pub fn threshold(&self, kind: OpKind) -> u32 {
        self.thresholds.get(&kind).copied().unwrap_or(1)
    }

    fn canonical_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(b"rules-v1");
        for admin in &self.admins {
            buf.extend_from_slice(b"\x00admin\x00");
            buf.extend_from_slice(admin.0.as_bytes());
        }
        // BTreeMap iterates in sorted key order -> canonical.
        for (kind, n) in &self.thresholds {
            buf.push(kind.tag());
            buf.extend_from_slice(&n.to_le_bytes());
        }
        buf
    }
}

/// The founding state of a lineage: immutable rules + founders, anchored by a
/// content-derived genesis id.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Genesis {
    pub rules: GenesisRules,
    pub founders: BTreeSet<Did>,
    pub id: GenesisId,
}

impl Genesis {
    pub fn new(rules: GenesisRules, founders: BTreeSet<Did>) -> Self {
        let mut h = Sha256::new();
        h.update(b"genesis-v1");
        h.update(rules.canonical_bytes());
        for f in &founders {
            h.update(b"\x00founder\x00");
            h.update(f.0.as_bytes());
        }
        let mut id = [0u8; 32];
        id.copy_from_slice(&h.finalize());
        Self {
            rules,
            founders,
            id: GenesisId(id),
        }
    }
}

/// The signable body of a governance op. Forms a hash chain via `prev`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpBody {
    /// Anchors this op to a specific lineage (I3 standing depends on it).
    pub genesis: GenesisId,
    /// Position in the chain (0-based).
    pub seq: u64,
    /// Hash of the previous op, or the genesis id for `seq == 0`.
    pub prev: [u8; 32],
    pub kind: OpKind,
    /// The member the op concerns (None for Dissolve).
    pub subject: Option<Did>,
}

impl OpBody {
    /// Canonical bytes that are signed and hashed. Hand-rolled for a stable
    /// byte layout independent of any serializer's framing.
    pub fn signing_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(b"op-v1");
        buf.extend_from_slice(&self.genesis.0);
        buf.extend_from_slice(&self.seq.to_le_bytes());
        buf.extend_from_slice(&self.prev);
        buf.push(self.kind.tag());
        match &self.subject {
            Some(d) => {
                buf.push(1);
                buf.extend_from_slice(d.0.as_bytes());
            }
            None => buf.push(0),
        }
        buf
    }

    /// The op's id: hash of its signing bytes. Used as the next op's `prev`.
    pub fn id(&self) -> [u8; 32] {
        let mut h = Sha256::new();
        h.update(self.signing_bytes());
        let mut out = [0u8; 32];
        out.copy_from_slice(&h.finalize());
        out
    }
}

/// An op plus the signatures gathered for it. Serializable so it can travel as
/// an opaque payload over a transport (Phase 3) without the relay learning its
/// contents.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedOp {
    pub body: OpBody,
    pub sigs: BTreeMap<Did, Sig>,
}

/// Build an op that chains onto `state`'s head and collect signatures from
/// `signers`. A convenience for tests and the simulator; mirrors what a client
/// would assemble before broadcasting.
pub fn sign_op(
    state: &GroupState,
    kind: OpKind,
    subject: Option<Did>,
    signers: &[&SigningIdentity],
) -> SignedOp {
    let body = OpBody {
        genesis: state.genesis.id,
        seq: state.log.len() as u64,
        prev: state.head,
        kind,
        subject,
    };
    let bytes = body.signing_bytes();
    let mut sigs = BTreeMap::new();
    for signer in signers {
        sigs.insert(signer.did().clone(), signer.sign(&bytes));
    }
    SignedOp { body, sigs }
}

/// Why an op was rejected. Every honest client rejects for the same reason
/// deterministically.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum RejectReason {
    #[error("op anchored to a different genesis")]
    GenesisMismatch,
    #[error("op does not chain onto the current head (seq/prev mismatch)")]
    BrokenChain,
    #[error("signer {} is not an admin with standing", .0.redacted())]
    SignerLacksStanding(Did),
    #[error("admin {} no longer has standing in this epoch (removed or departed)", .0.redacted())]
    DepartedAdmin(Did),
    #[error("signature from {} failed verification", .0.redacted())]
    BadSignature(Did),
    #[error("under threshold for {kind:?}: have {have}, need {need}")]
    UnderThreshold { kind: OpKind, have: u32, need: u32 },
    #[error("operation references {} who is not a current member", .0.redacted())]
    NotAMember(Did),
    #[error("missing subject for an op kind that requires one")]
    MissingSubject,
    #[error("group is dissolved")]
    Dissolved,
}

/// Maps DIDs to their public verifying identities. Runtime data (public keys),
/// distinct from the immutable genesis rules.
#[derive(Default)]
pub struct Directory {
    map: BTreeMap<Did, VerifyingIdentity>,
}

impl Directory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, v: VerifyingIdentity) {
        self.map.insert(v.did().clone(), v);
    }

    fn verify(&self, did: &Did, msg: &[u8], sig: &Sig) -> bool {
        self.map.get(did).is_some_and(|v| v.verify(msg, sig))
    }
}

/// The evaluated state of one lineage branch: its genesis, current members,
/// and the validated op log.
#[derive(Debug, Clone)]
pub struct GroupState {
    pub genesis: Genesis,
    pub members: BTreeSet<Did>,
    pub log: Vec<SignedOp>,
    pub head: [u8; 32],
    pub dissolved: bool,
}

impl GroupState {
    pub fn new(genesis: Genesis) -> Self {
        let members = genesis.founders.clone();
        let head = genesis.id.0;
        Self {
            genesis,
            members,
            log: Vec::new(),
            head,
            dissolved: false,
        }
    }

    /// Validate an op against the current state and genesis rules without
    /// mutating anything. This is the heart of I1/I2.
    pub fn validate(&self, op: &SignedOp, dir: &Directory) -> Result<(), RejectReason> {
        if self.dissolved {
            return Err(RejectReason::Dissolved);
        }
        // Anchored to this lineage (I3).
        if op.body.genesis != self.genesis.id {
            return Err(RejectReason::GenesisMismatch);
        }
        // Chains onto the current head — forward-only, no rewrite.
        if op.body.seq != self.log.len() as u64 || op.body.prev != self.head {
            return Err(RejectReason::BrokenChain);
        }

        // Count signatures from admins with standing in the *current epoch*.
        // Standing is two-fold (I1 + I2): the signer must be in the immutable
        // genesis admin set (I1: never the mutable membership), AND must still
        // be a current member of this branch. A genesis admin who has been
        // removed or has left forfeits governance authority even though the
        // immutable admin *set* still names them — otherwise a compromised,
        // then-evicted, admin key would govern forever.
        let signing_bytes = op.body.signing_bytes();
        let mut valid: u32 = 0;
        for (did, sig) in &op.sigs {
            if !self.genesis.rules.admins.contains(did) {
                return Err(RejectReason::SignerLacksStanding(did.clone()));
            }
            if !self.members.contains(did) {
                return Err(RejectReason::DepartedAdmin(did.clone()));
            }
            if !dir.verify(did, &signing_bytes, sig) {
                return Err(RejectReason::BadSignature(did.clone()));
            }
            valid += 1;
        }

        let need = self.genesis.rules.threshold(op.body.kind);
        if valid < need {
            return Err(RejectReason::UnderThreshold {
                kind: op.body.kind,
                have: valid,
                need,
            });
        }

        // Semantic checks per kind.
        match op.body.kind {
            OpKind::Remove | OpKind::Leave => {
                let subject = op.body.subject.as_ref().ok_or(RejectReason::MissingSubject)?;
                if !self.members.contains(subject) {
                    return Err(RejectReason::NotAMember(subject.clone()));
                }
            }
            OpKind::Add => {
                op.body.subject.as_ref().ok_or(RejectReason::MissingSubject)?;
            }
            OpKind::Dissolve | OpKind::Fork | OpKind::Recombine => {}
        }
        Ok(())
    }

    /// Validate then apply an op, advancing the head and membership.
    pub fn apply(&mut self, op: SignedOp, dir: &Directory) -> Result<(), RejectReason> {
        self.validate(&op, dir)?;
        match op.body.kind {
            OpKind::Add => {
                self.members.insert(op.body.subject.clone().unwrap());
            }
            OpKind::Remove | OpKind::Leave => {
                self.members.remove(op.body.subject.as_ref().unwrap());
            }
            OpKind::Dissolve => {
                self.dissolved = true;
            }
            OpKind::Fork | OpKind::Recombine => { /* tracked in the DAG layer */ }
        }
        self.head = op.body.id();
        self.log.push(op);
        Ok(())
    }

    /// Count valid signatures on `op` from admins **with standing in this
    /// epoch** (genesis admin AND current member), without checking that the op
    /// chains onto the head. Shared by `validate` and the conflict-layer quorum
    /// override, which evaluates a decision's authority independent of where it
    /// would sit in the chain.
    pub fn valid_admin_sigs(&self, op: &SignedOp, dir: &Directory) -> u32 {
        let bytes = op.body.signing_bytes();
        op.sigs
            .iter()
            .filter(|(did, sig)| {
                self.genesis.rules.admins.contains(did)
                    && self.members.contains(did)
                    && dir.verify(did, &bytes, sig)
            })
            .count() as u32
    }

    /// Does `op` carry enough valid admin-with-standing signatures to meet the
    /// genesis threshold for its kind? (Threshold check only — not chaining.)
    pub fn meets_threshold(&self, op: &SignedOp, dir: &Directory) -> bool {
        self.valid_admin_sigs(op, dir) >= self.genesis.rules.threshold(op.body.kind)
    }

    /// Count distinct **lineages** among valid admin-with-standing signers (E2.10).
    ///
    /// `lineage_of` maps a signer DID to its lineage id (computed by every client
    /// from the T1 leaf credential). A signer absent from the map counts as its
    /// own lineage (a single-device admin — preserves legacy behaviour). Two
    /// signatures from devices of the *same* lineage count once, so a person
    /// cannot manufacture a social quorum from multiple of their own devices.
    pub fn valid_admin_lineages(
        &self,
        op: &SignedOp,
        dir: &Directory,
        lineage_of: &BTreeMap<Did, Did>,
    ) -> u32 {
        let bytes = op.body.signing_bytes();
        let mut lineages = BTreeSet::new();
        for (did, sig) in &op.sigs {
            if self.genesis.rules.admins.contains(did)
                && self.members.contains(did)
                && dir.verify(did, &bytes, sig)
            {
                let lineage = lineage_of.get(did).cloned().unwrap_or_else(|| did.clone());
                lineages.insert(lineage);
            }
        }
        lineages.len() as u32
    }

    /// Does `op` meet its genesis threshold counting **distinct lineages** rather
    /// than distinct DIDs (E2.10)? This is the social-threshold check that
    /// resists own-device quorum manufacture.
    pub fn meets_threshold_by_lineage(
        &self,
        op: &SignedOp,
        dir: &Directory,
        lineage_of: &BTreeMap<Did, Did>,
    ) -> bool {
        self.valid_admin_lineages(op, dir, lineage_of) >= self.genesis.rules.threshold(op.body.kind)
    }

    /// E2.14 — the same-lineage / cross-lineage asymmetry for an op targeting a
    /// device under `subject_lineage`. If **every** valid admin signer shares the
    /// subject's lineage, a single signature suffices (a person managing their
    /// own devices — the shared lineage *is* the authorization). Otherwise the
    /// full genesis threshold for the op kind applies, counted by lineage
    /// (a cross-lineage action on a device is an ordinary social decision).
    pub fn device_op_meets_threshold(
        &self,
        op: &SignedOp,
        dir: &Directory,
        lineage_of: &BTreeMap<Did, Did>,
        subject_lineage: &Did,
    ) -> bool {
        let bytes = op.body.signing_bytes();
        let valid_signers: Vec<&Did> = op
            .sigs
            .iter()
            .filter(|(d, s)| {
                self.genesis.rules.admins.contains(*d)
                    && self.members.contains(*d)
                    && dir.verify(d, &bytes, s)
            })
            .map(|(d, _)| d)
            .collect();
        if valid_signers.is_empty() {
            return false;
        }
        let all_same_lineage = valid_signers
            .iter()
            .all(|d| lineage_of.get(*d) == Some(subject_lineage));
        if all_same_lineage {
            // Same-lineage self-management: one valid signature is enough.
            true
        } else {
            // Cross-lineage: pay the full social threshold (lineage-counted).
            self.meets_threshold_by_lineage(op, dir, lineage_of)
        }
    }
}

/// E2.13 — the set of member DIDs sharing `lineage` (per the leaf→lineage map),
/// i.e. every device leaf of one person. `leave-all-under-lineage` removes this
/// whole set; `leave-this-leaf` removes a single DID. The two are distinct ops.
pub fn devices_of_lineage(
    members: &BTreeSet<Did>,
    lineage_of: &BTreeMap<Did, Did>,
    lineage: &Did,
) -> BTreeSet<Did> {
    members
        .iter()
        .filter(|d| lineage_of.get(*d) == Some(lineage))
        .cloned()
        .collect()
}

/// A threshold-signed compaction checkpoint (T3 / the real-crypto form of the
/// model's F-group). It summarizes a branch's log up to `head` at `up_to_seq` so
/// history can be rolled up. Its legitimacy comes from a *threshold of admin
/// lineages* signing it — never a single authority (the broker). Bound to a
/// specific head, so it cannot span an open fork.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Checkpoint {
    pub genesis: GenesisId,
    pub up_to_seq: u64,
    pub head: [u8; 32],
    pub sigs: BTreeMap<Did, Sig>,
}

impl Checkpoint {
    /// Canonical bytes the signers sign over.
    pub fn signing_bytes(&self) -> Vec<u8> {
        let mut b = Vec::new();
        b.extend_from_slice(b"checkpoint-v1");
        b.extend_from_slice(&self.genesis.0);
        b.extend_from_slice(&self.up_to_seq.to_le_bytes());
        b.extend_from_slice(&self.head);
        b
    }

    /// Build a checkpoint at `state`'s current head and collect `signers`' sigs.
    pub fn sign(state: &GroupState, signers: &[&SigningIdentity]) -> Checkpoint {
        let mut cp = Checkpoint {
            genesis: state.genesis.id,
            up_to_seq: state.log.len() as u64,
            head: state.head,
            sigs: BTreeMap::new(),
        };
        let bytes = cp.signing_bytes();
        for s in signers {
            cp.sigs.insert(s.did().clone(), s.sign(&bytes));
        }
        cp
    }
}

/// Why a checkpoint was rejected.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckpointError {
    #[error("checkpoint anchored to a different genesis")]
    GenesisMismatch,
    #[error("checkpoint head/seq does not match the branch log (tampered or wrong branch)")]
    HeadMismatch,
    #[error("under threshold: have {have} admin lineages, need {need}")]
    UnderThreshold { have: u32, need: u32 },
}

/// Verify a checkpoint against the branch `state`. It must anchor to this genesis,
/// match the real head/seq (no tampered summary; binds it to one branch so it
/// cannot span a fork), and carry signatures from at least `threshold` distinct
/// admin LINEAGES with standing (lineage-counted, per E2.10 — a single authority
/// or own-device padding cannot finalize it).
pub fn verify_checkpoint(
    state: &GroupState,
    cp: &Checkpoint,
    dir: &Directory,
    lineage_of: &BTreeMap<Did, Did>,
    threshold: u32,
) -> Result<(), CheckpointError> {
    if cp.genesis != state.genesis.id {
        return Err(CheckpointError::GenesisMismatch);
    }
    if cp.up_to_seq != state.log.len() as u64 || cp.head != state.head {
        return Err(CheckpointError::HeadMismatch);
    }
    let bytes = cp.signing_bytes();
    let mut lineages = BTreeSet::new();
    for (d, s) in &cp.sigs {
        if state.genesis.rules.admins.contains(d)
            && state.members.contains(d)
            && dir.verify(d, &bytes, s)
        {
            let lin = lineage_of.get(d).cloned().unwrap_or_else(|| d.clone());
            lineages.insert(lin);
        }
    }
    let have = lineages.len() as u32;
    if have < threshold {
        return Err(CheckpointError::UnderThreshold { have, need: threshold });
    }
    Ok(())
}

/// An attributable governance equivocation: an admin signed two *conflicting*
/// operations at the same chain position. This is the signal a "fork-detecting,
/// attributable" governance tree must surface — it is never a silent heal.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Equivocation {
    /// The DID that double-signed.
    pub culprit: Did,
    pub genesis: GenesisId,
    pub seq: u64,
    /// The two conflicting op ids, sorted for a deterministic report.
    pub op_lo: [u8; 32],
    pub op_hi: [u8; 32],
}

/// Detect equivocation between two signed ops: same lineage and same `seq` but
/// different op ids, with one or more DIDs that *validly signed both*. Returns
/// one [`Equivocation`] per culprit (sorted, deterministic). Empty when the ops
/// are identical, at different positions, on different lineages, or share no
/// common valid signer.
pub fn detect_equivocation(a: &SignedOp, b: &SignedOp, dir: &Directory) -> Vec<Equivocation> {
    if a.body.genesis != b.body.genesis || a.body.seq != b.body.seq {
        return Vec::new();
    }
    let (id_a, id_b) = (a.body.id(), b.body.id());
    if id_a == id_b {
        return Vec::new(); // same op, no conflict
    }
    let (op_lo, op_hi) = if id_a <= id_b { (id_a, id_b) } else { (id_b, id_a) };
    let bytes_a = a.body.signing_bytes();
    let bytes_b = b.body.signing_bytes();
    let mut out = BTreeSet::new();
    for (did, sig_a) in &a.sigs {
        if let Some(sig_b) = b.sigs.get(did) {
            // Culprit must have validly signed *both* conflicting ops.
            if dir.verify(did, &bytes_a, sig_a) && dir.verify(did, &bytes_b, sig_b) {
                out.insert(Equivocation {
                    culprit: did.clone(),
                    genesis: a.body.genesis,
                    seq: a.body.seq,
                    op_lo,
                    op_hi,
                });
            }
        }
    }
    out.into_iter().collect()
}
