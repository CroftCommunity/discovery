//! RUN-ATTEST-02 — the co-op issuer: mint ceremony, retained state, blinded
//! commitment lineage, and the status-check protocol.
//!
//! The issuer-state type is deliberately narrow (T-PA6.1): it can hold its own
//! signed assertions (by credential hash), process bookkeeping (epochs,
//! used salts), blinded commitments, and the payment-bookkeeping stand-in —
//! the latter explicitly typed as [`SeamBoundary`], the ONE named place where
//! holder linkage may live, so it cannot silently spread. Substrate (ID
//! numbers, card numbers) remains unrepresentable, inheriting T-AT6.1: every
//! field is a closed enum, fixed-size array, or map of those. No `String`, no
//! free-form bytes, no `PersonaId` anywhere in retained state.
//!
//! Public lineage (T-PA1.3, narrowest option per OC-1): per-epoch UNORDERED
//! sets of blinded commitments `H(credential-id ‖ fresh salt)` — never subject
//! persona identifiers. Verification of a credential needs only the issuer's
//! signature on the credential itself; no public registry lookup exists.
//!
//! Status checks (T-PA6.3): OCSP-shaped, read-side solicitation. A verifier
//! submits a credential hash; the issuer answers current/superseded/unknown,
//! signed, deterministically, from its own assertion lineage. Staleness of an
//! unanswered check is presentation, never verdict: whether to require a
//! fresh answer (and fail closed without one) is the verifier app's dial
//! ([`CheckDial`]), not protocol.

use std::collections::{BTreeMap, BTreeSet};

use ipld_core::ipld::Ipld;

use crate::fixtures::PersonaFixture;
use crate::types::*;

// ---------------------------------------------------------------------------
// The seam
// ---------------------------------------------------------------------------

/// An opaque member handle used ONLY inside [`SeamBoundary`]. It is derived
/// from fixture bookkeeping and is deliberately NOT a [`PersonaId`]: the seam
/// may know "this member paid for N anchors" without any persona linkage being
/// representable in retained state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MemberRef(pub [u8; 32]);

/// The payment-bookkeeping stand-in (RUN-ATTEST-02 §3 / T-PA6.1): the KNOWN
/// linkage point, named in the type system. It records per-member anchor
/// counts for fee/dial enforcement and NOTHING else; it has no serialization
/// and no public accessor that returns its contents — the anchor count of a
/// member is never public (T-PA3.3).
#[derive(Debug, Default)]
pub struct SeamBoundary {
    ledger: BTreeMap<MemberRef, u32>,
}

impl SeamBoundary {
    /// Anchors recorded for `member` so far. `pub(crate)` on purpose: the seam
    /// is readable by the mint path's dial check, never by public queries.
    pub(crate) fn anchors_of(&self, member: &MemberRef) -> u32 {
        self.ledger.get(member).copied().unwrap_or(0)
    }

    pub(crate) fn record_anchor(&mut self, member: MemberRef) {
        *self.ledger.entry(member).or_insert(0) += 1;
    }
}

// ---------------------------------------------------------------------------
// Mint entropy — single-use by type
// ---------------------------------------------------------------------------

/// Per-mint entropy. Deliberately neither `Clone` nor `Copy`: a mint consumes
/// it by value, so the same entropy value cannot back two mints — the
/// compile-boundary half of T-PA2.2:
///
/// ```compile_fail
/// use attest_family::issuer::MintEntropy;
/// let e = MintEntropy::from_seed([1; 32]);
/// let first = e;
/// let second = e; // moved — one mint's entropy cannot feed a second mint
/// ```
///
/// The runtime half: re-deriving from the same seed yields the same salts,
/// and the issuer state deterministically refuses a reused salt.
pub struct MintEntropy {
    pub(crate) seed: [u8; 32],
}

impl MintEntropy {
    pub fn from_seed(seed: [u8; 32]) -> Self {
        MintEntropy { seed }
    }
}

/// Deterministic mint refusals (T-PA2.2, T-PA3.2). Checked in this order:
/// dial, then salt reuse; a refused mint leaves state unchanged.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MintRefusal {
    /// The member is at the governed max-anchors dial.
    DialExceeded,
    /// A derived commitment salt was already used — shared derivation state.
    SaltReused,
}

/// What the PERSONA publishes after a mint ceremony (RUN-ATTEST-02 §3): its
/// vetting fact and its single-predicate credentials. What the ISSUER retains
/// or publishes is exactly the [`IssuerState`] surface — the harness routes
/// through it, never around it.
#[derive(Debug, Clone)]
pub struct MintOutput {
    pub vetting: Envelope,
    pub credentials: Vec<Envelope>,
}

// ---------------------------------------------------------------------------
// Status protocol
// ---------------------------------------------------------------------------

/// The three deterministic answers of a status check.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CredStanding {
    Current,
    Superseded,
    /// The hash is not in this issuer's assertion lineage.
    Unknown,
}

impl CredStanding {
    pub fn as_str(&self) -> &'static str {
        match self {
            CredStanding::Current => "current",
            CredStanding::Superseded => "superseded",
            CredStanding::Unknown => "unknown",
        }
    }
}

/// The status-check response: EXACTLY the queried hash (echo), the standing,
/// and the issuer's signature — nothing else (T-PA2.3: the field set is the
/// whole disclosure; no counts, no neighbors, no timestamps).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusResponse {
    pub credential_hash: [u8; 32],
    pub standing: CredStanding,
    pub signature: Vec<u8>,
}

impl StatusResponse {
    /// The signed portion: {h: hash, s: standing} — what the issuer's
    /// signature covers, and everything the response asserts.
    fn signing_ipld(credential_hash: &[u8; 32], standing: CredStanding) -> Ipld {
        let mut m = BTreeMap::new();
        m.insert("h".to_string(), Ipld::Bytes(credential_hash.to_vec()));
        m.insert("s".to_string(), Ipld::String(standing.as_str().to_string()));
        Ipld::Map(m)
    }

    pub(crate) fn signing_bytes(credential_hash: &[u8; 32], standing: CredStanding) -> Vec<u8> {
        serde_ipld_dagcbor::to_vec(&Self::signing_ipld(credential_hash, standing))
            .expect("pure map encode cannot fail")
    }

    pub fn to_ipld(&self) -> Ipld {
        let Ipld::Map(mut m) = Self::signing_ipld(&self.credential_hash, self.standing) else {
            unreachable!("signing form is a map")
        };
        m.insert("g".to_string(), Ipld::Bytes(self.signature.clone()));
        Ipld::Map(m)
    }

    pub fn to_canonical_bytes(&self) -> Vec<u8> {
        serde_ipld_dagcbor::to_vec(&self.to_ipld()).expect("pure map encode cannot fail")
    }
}

/// A verifier app's freshness posture (T-PA6.3): protocol never renders a
/// verdict from silence; requiring a fresh `current` answer (and failing
/// closed without one) is app policy, expressed here as the caller's dial.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckDial {
    SignatureOnly,
    RequireCurrentStatus,
}

// ---------------------------------------------------------------------------
// Public lineage
// ---------------------------------------------------------------------------

/// One closed epoch of the issuer's public lineage: an UNORDERED set of
/// blinded commitments (T-PA1.4 — within an epoch there is no order, so
/// lineage adjacency cannot correlate siblings minted in one ceremony
/// session), the declared issuance total, and the issuer's signature.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EpochRecord {
    pub epoch_no: u64,
    pub commitments: BTreeSet<[u8; 32]>,
    pub declared_total: u64,
    pub signature: Vec<u8>,
}

impl EpochRecord {
    /// The signed portion: {c: sorted commitment list, e: epoch, t: total}.
    /// `BTreeSet` iteration IS byte-ascending order — the canonical form of
    /// an unordered set, computed from commitment values alone (T-PA1.4).
    fn signing_ipld(epoch_no: u64, commitments: &BTreeSet<[u8; 32]>, declared_total: u64) -> Ipld {
        let mut m = BTreeMap::new();
        m.insert(
            "c".to_string(),
            Ipld::List(commitments.iter().map(|c| Ipld::Bytes(c.to_vec())).collect()),
        );
        m.insert("e".to_string(), Ipld::Integer(epoch_no as i128));
        m.insert("t".to_string(), Ipld::Integer(declared_total as i128));
        Ipld::Map(m)
    }

    pub(crate) fn signing_bytes(
        epoch_no: u64,
        commitments: &BTreeSet<[u8; 32]>,
        declared_total: u64,
    ) -> Vec<u8> {
        serde_ipld_dagcbor::to_vec(&Self::signing_ipld(epoch_no, commitments, declared_total))
            .expect("pure map encode cannot fail")
    }

    pub fn to_ipld(&self) -> Ipld {
        let Ipld::Map(mut m) =
            Self::signing_ipld(self.epoch_no, &self.commitments, self.declared_total)
        else {
            unreachable!("signing form is a map")
        };
        m.insert("g".to_string(), Ipld::Bytes(self.signature.clone()));
        Ipld::Map(m)
    }
}

/// What a covenant audit may learn (T-PA6.2): totals and shape validity ONLY.
/// No persona identity is resolvable from a commitment lineage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AuditReport {
    pub epochs_audited: u64,
    pub total_commitments: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuditFailure {
    Undecodable,
    BadSignature { epoch_no: u64 },
    TotalMismatch { epoch_no: u64 },
    MalformedCommitment { epoch_no: u64 },
    NonContiguousEpochs,
}

impl std::fmt::Display for AuditFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

// ---------------------------------------------------------------------------
// Issuer state
// ---------------------------------------------------------------------------

/// A head in the issuer's own assertion lineage: issued, possibly superseded.
/// Keyed by credential hash — retained state holds NO persona identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct AssertionHead {
    pub(crate) superseded_by: Option<[u8; 32]>,
}

/// The issuer's retained state (T-PA6.1): its own signed assertions (by
/// credential hash), process bookkeeping (epoch counter, used salts), blinded
/// commitments, and the [`SeamBoundary`]. Nothing else is representable.
#[derive(Debug)]
pub struct IssuerState {
    pub(crate) epoch_no: u64,
    pub(crate) open_commitments: BTreeSet<[u8; 32]>,
    pub(crate) closed: Vec<EpochRecord>,
    pub(crate) used_salts: BTreeSet<[u8; 32]>,
    pub(crate) assertions: BTreeMap<[u8; 32], AssertionHead>,
    pub(crate) seam: SeamBoundary,
    pub(crate) dial_max_anchors: u32,
}

impl IssuerState {
    pub fn new(dial_max_anchors: u32) -> Self {
        IssuerState {
            epoch_no: 1,
            open_commitments: BTreeSet::new(),
            closed: Vec::new(),
            used_salts: BTreeSet::new(),
            assertions: BTreeMap::new(),
            seam: SeamBoundary::default(),
            dial_max_anchors,
        }
    }

    /// Feed the governed dial value from the folded R7 register (T-PA3.2 —
    /// the dial is a rule under the EXISTING quorum machinery; this state
    /// only mirrors the folded value, it never governs it).
    pub fn set_dial(&mut self, max_anchors: u32) {
        self.dial_max_anchors = max_anchors;
    }

    pub fn dial(&self) -> u32 {
        self.dial_max_anchors
    }

    /// Close the open epoch: sign its unordered commitment set + declared
    /// total, append the record to public lineage, open the next epoch.
    pub fn close_epoch(&mut self, issuer: &PersonaFixture) -> EpochRecord {
        let commitments = std::mem::take(&mut self.open_commitments);
        let declared_total = commitments.len() as u64;
        let signature =
            issuer.sign_bytes(&EpochRecord::signing_bytes(self.epoch_no, &commitments, declared_total));
        let record = EpochRecord { epoch_no: self.epoch_no, commitments, declared_total, signature };
        self.closed.push(record.clone());
        self.epoch_no += 1;
        record
    }

    /// The issuer's ENTIRE public lineage, canonical bytes (T-PA1.3): closed
    /// epoch records only — commitments, never identities.
    pub fn lineage_bytes(&self) -> Vec<u8> {
        let list = Ipld::List(self.closed.iter().map(|r| r.to_ipld()).collect());
        serde_ipld_dagcbor::to_vec(&list).expect("pure value encode cannot fail")
    }

    /// The status-check protocol (T-PA6.3): deterministic signed answer from
    /// the issuer's own assertion lineage, for exactly the queried hash.
    pub fn status_check(&self, issuer: &PersonaFixture, credential_hash: [u8; 32]) -> StatusResponse {
        let standing = match self.assertions.get(&credential_hash) {
            None => CredStanding::Unknown,
            Some(head) => match head.superseded_by {
                None => CredStanding::Current,
                Some(_) => CredStanding::Superseded,
            },
        };
        let signature = issuer.sign_bytes(&StatusResponse::signing_bytes(&credential_hash, standing));
        StatusResponse { credential_hash, standing, signature }
    }

    /// Supersede one of this issuer's credentials: signs a
    /// `credential_supersede` assertion (the persona's published object stays
    /// byte-intact in every log; T-PA6.4) and moves the assertion head.
    pub fn supersede(&mut self, issuer: &PersonaFixture, credential: &Envelope) -> Envelope {
        let id = credential.object_id();
        let head = self
            .assertions
            .get_mut(&id.0)
            .expect("supersede targets a credential this issuer asserted");
        let env = issuer.sign(
            self.epoch_no,
            vec![id],
            Payload::CredentialSupersede(crate::types::CredentialSupersede { supersedes: id }),
        );
        head.superseded_by = Some(env.object_id().0);
        env
    }
}

// ---------------------------------------------------------------------------
// The mint ceremony harness (RUN-ATTEST-02 §3)
// ---------------------------------------------------------------------------

/// Mint one persona's single-predicate credentials. Performs the
/// vetting-event stand-in, derives everything independently per persona from
/// single-use entropy (fresh vetting nonce, fresh per-credential mint nonces
/// and commitment salts — no shared state across calls), routes retained
/// effects through `state` (dial check at the seam, salt-reuse refusal,
/// commitment into the open epoch, assertion heads), and returns what the
/// PERSONA publishes.
///
/// Envelope lamports are the OPEN EPOCH NUMBER, not a per-mint counter: within
/// an epoch, mint order is not representable in any public object (T-PA1.4).
/// Domain-separated derivation from the single-use mint entropy.
fn derive(seed: &[u8; 32], tag: &[u8], k: u8) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();
    hasher.update(seed);
    hasher.update(tag);
    hasher.update(&[k]);
    *hasher.finalize().as_bytes()
}

fn first16(v: [u8; 32]) -> [u8; 16] {
    let mut out = [0u8; 16];
    out.copy_from_slice(&v[..16]);
    out
}

pub fn mint(
    state: &mut IssuerState,
    issuer: &PersonaFixture,
    member: MemberRef,
    subject: &PersonaFixture,
    kinds: &[PredicateKind],
    performed_on: DateClaim,
    entropy: MintEntropy,
) -> Result<MintOutput, MintRefusal> {
    // Refusals are deterministic and ordered — dial, then salt reuse — and a
    // refused mint changes NOTHING (all checks precede all writes).
    if state.seam.anchors_of(&member) + 1 > state.dial_max_anchors {
        return Err(MintRefusal::DialExceeded);
    }
    let salts: Vec<[u8; 32]> =
        (0..kinds.len()).map(|k| derive(&entropy.seed, b"salt", k as u8)).collect();
    if salts.iter().any(|s| state.used_salts.contains(s)) {
        return Err(MintRefusal::SaltReused);
    }

    // The vetting-event stand-in: one per mint call, per persona, fresh
    // nonce. Lamport is the OPEN EPOCH NUMBER (mint order is unrepresentable
    // within an epoch).
    let epoch = state.epoch_no;
    let vetting = issuer.sign(
        epoch,
        vec![],
        Payload::VettingFact(crate::types::VettingFact {
            subject: subject.id,
            vetting_nonce: first16(derive(&entropy.seed, b"vet", 0)),
            performed_on,
            role: IssuerRole::CoopIssuer,
        }),
    );

    // One single-predicate credential per kind, each independently derived,
    // each citing the vetting fact as antecedent.
    let mut credentials: Vec<Envelope> = Vec::new();
    for (k, kind) in kinds.iter().enumerate() {
        let env = issuer.sign(
            epoch,
            vec![vetting.object_id()],
            Payload::Credential(crate::types::Credential {
                predicate: *kind,
                subject: subject.id,
                process: ProcessProvenance {
                    method: match kind {
                        PredicateKind::PhoneVerified => MethodKind::SmsRoundTrip,
                        PredicateKind::PaymentVerified => MethodKind::CardAuthorization,
                        _ => MethodKind::DocumentSighted,
                    },
                    performed_on,
                    role: IssuerRole::CoopIssuer,
                },
                mint_nonce: first16(derive(&entropy.seed, b"nonce", k as u8)),
                supersedes: None,
            }),
        );
        credentials.push(env);
    }

    // Retained effects, routed through the issuer state (never around it):
    // the seam records the anchor, salts are consumed, each credential's
    // BLINDED commitment H(id ‖ salt) enters the open epoch, and the
    // assertion head opens as current.
    state.seam.record_anchor(member);
    for (env, salt) in credentials.iter().zip(&salts) {
        state.used_salts.insert(*salt);
        let mut hasher = blake3::Hasher::new();
        hasher.update(&env.object_id().0);
        hasher.update(salt);
        state.open_commitments.insert(*hasher.finalize().as_bytes());
        state.assertions.insert(env.object_id().0, AssertionHead { superseded_by: None });
    }
    Ok(MintOutput { vetting, credentials })
}

// ---------------------------------------------------------------------------
// Verifier side — no registry anywhere
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CredentialVerifyError {
    Undecodable,
    NotACredential,
    WrongIssuer,
    BadSignature,
}

/// Verify a published credential from NOTHING but its bytes and the expected
/// issuer key (T-PA1.3): decode, check the author is the issuer, verify the
/// signature. There is deliberately no registry parameter and no lookup.
pub fn verify_credential(bytes: &[u8], issuer: &PersonaId) -> Result<Envelope, CredentialVerifyError> {
    let env = crate::canonical::decode_envelope(bytes)
        .map_err(|_| CredentialVerifyError::Undecodable)?;
    if !matches!(env.payload, Payload::Credential(_)) {
        return Err(CredentialVerifyError::NotACredential);
    }
    if &env.author != issuer {
        return Err(CredentialVerifyError::WrongIssuer);
    }
    crate::fold::verify_signature(&env).map_err(|_| CredentialVerifyError::BadSignature)?;
    Ok(env)
}

fn verify_detached(bytes: &[u8], signature: &[u8], key: &PersonaId) -> bool {
    use ed25519_dalek::{Signature, Verifier as _, VerifyingKey};
    let Ok(vk) = VerifyingKey::from_bytes(&key.0) else { return false };
    let Ok(sig_bytes) = <[u8; 64]>::try_from(signature.to_vec()) else { return false };
    vk.verify(bytes, &Signature::from_bytes(&sig_bytes)).is_ok()
}

/// Verify a status response's signature against the issuer key.
pub fn verify_status_response(resp: &StatusResponse, issuer: &PersonaId) -> bool {
    verify_detached(
        &StatusResponse::signing_bytes(&resp.credential_hash, resp.standing),
        &resp.signature,
        issuer,
    )
}

/// A verifier's acceptance decision (T-PA6.4). Silence is never a verdict:
/// with `SignatureOnly` a missing status answer is acceptable BY THAT APP'S
/// CHOICE; with `RequireCurrentStatus` the app fails closed without a fresh
/// `current`. A provided `superseded` answer refuses in either posture.
pub fn verifier_accepts(
    credential_bytes: &[u8],
    issuer: &PersonaId,
    status: Option<&StatusResponse>,
    dial: CheckDial,
) -> bool {
    let Ok(env) = verify_credential(credential_bytes, issuer) else { return false };
    match status {
        Some(resp) => {
            resp.credential_hash == env.object_id().0
                && verify_status_response(resp, issuer)
                && resp.standing == CredStanding::Current
        }
        None => dial == CheckDial::SignatureOnly,
    }
}

/// The covenant audit over PUBLIC lineage bytes (T-PA6.2): every epoch record
/// decodes, its signature verifies, every commitment is well-formed (32
/// bytes), the declared total matches the set, epochs are contiguous from 1 —
/// while resolving ZERO persona identities (the report is totals only).
pub fn audit_lineage(bytes: &[u8], issuer: &PersonaId) -> Result<AuditReport, AuditFailure> {
    let v: Ipld = serde_ipld_dagcbor::from_slice(bytes).map_err(|_| AuditFailure::Undecodable)?;
    let Ipld::List(records) = v else { return Err(AuditFailure::Undecodable) };

    let mut expected_epoch: u64 = 1;
    let mut total_commitments: u64 = 0;
    let mut epochs_audited: u64 = 0;
    for record in records {
        let Ipld::Map(mut m) = record else { return Err(AuditFailure::Undecodable) };
        let Some(Ipld::Integer(e)) = m.get("e").cloned() else {
            return Err(AuditFailure::Undecodable);
        };
        let epoch_no = u64::try_from(e).map_err(|_| AuditFailure::Undecodable)?;
        let Some(Ipld::Bytes(sig)) = m.remove("g") else { return Err(AuditFailure::Undecodable) };

        // The signature covers the record minus its own `g` field — the
        // exact bytes the issuer signed at epoch close.
        let unsigned =
            serde_ipld_dagcbor::to_vec(&Ipld::Map(m.clone())).map_err(|_| AuditFailure::Undecodable)?;
        if !verify_detached(&unsigned, &sig, issuer) {
            return Err(AuditFailure::BadSignature { epoch_no });
        }

        // Every commitment has the well-formed shape: exactly 32 bytes.
        let Some(Ipld::List(commitments)) = m.get("c") else {
            return Err(AuditFailure::Undecodable);
        };
        for c in commitments {
            let Ipld::Bytes(b) = c else { return Err(AuditFailure::MalformedCommitment { epoch_no }) };
            if b.len() != 32 {
                return Err(AuditFailure::MalformedCommitment { epoch_no });
            }
        }

        // The declared issuance total matches the commitment set.
        let Some(Ipld::Integer(t)) = m.get("t") else { return Err(AuditFailure::Undecodable) };
        let declared = u64::try_from(*t).map_err(|_| AuditFailure::Undecodable)?;
        if declared != commitments.len() as u64 {
            return Err(AuditFailure::TotalMismatch { epoch_no });
        }

        // The lineage is intact: epochs contiguous from 1.
        if epoch_no != expected_epoch {
            return Err(AuditFailure::NonContiguousEpochs);
        }
        expected_epoch += 1;
        epochs_audited += 1;
        total_commitments += declared;
    }
    Ok(AuditReport { epochs_audited, total_commitments })
}
