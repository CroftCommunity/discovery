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
    pub fn to_ipld(&self) -> Ipld {
        unimplemented!("RUN-ATTEST-02: status response serialization pending")
    }

    pub fn to_canonical_bytes(&self) -> Vec<u8> {
        unimplemented!("RUN-ATTEST-02: status response serialization pending")
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
    pub fn to_ipld(&self) -> Ipld {
        unimplemented!("RUN-ATTEST-02: epoch record serialization pending")
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
        unimplemented!("RUN-ATTEST-02: issuer state pending ({dial_max_anchors})")
    }

    /// Feed the governed dial value from the folded R7 register (T-PA3.2 —
    /// the dial is a rule under the EXISTING quorum machinery; this state
    /// only mirrors the folded value, it never governs it).
    pub fn set_dial(&mut self, max_anchors: u32) {
        unimplemented!("RUN-ATTEST-02: dial bridge pending ({max_anchors})")
    }

    pub fn dial(&self) -> u32 {
        self.dial_max_anchors
    }

    /// Close the open epoch: sign its unordered commitment set + declared
    /// total, append the record to public lineage, open the next epoch.
    pub fn close_epoch(&mut self, issuer: &PersonaFixture) -> EpochRecord {
        let _ = issuer;
        unimplemented!("RUN-ATTEST-02: epoch close pending")
    }

    /// The issuer's ENTIRE public lineage, canonical bytes (T-PA1.3): closed
    /// epoch records only — commitments, never identities.
    pub fn lineage_bytes(&self) -> Vec<u8> {
        unimplemented!("RUN-ATTEST-02: public lineage pending")
    }

    /// The status-check protocol (T-PA6.3): deterministic signed answer from
    /// the issuer's own assertion lineage, for exactly the queried hash.
    pub fn status_check(&self, issuer: &PersonaFixture, credential_hash: [u8; 32]) -> StatusResponse {
        let _ = (issuer, credential_hash);
        unimplemented!("RUN-ATTEST-02: status check pending")
    }

    /// Supersede one of this issuer's credentials: signs a
    /// `credential_supersede` assertion (the persona's published object stays
    /// byte-intact in every log; T-PA6.4) and moves the assertion head.
    pub fn supersede(&mut self, issuer: &PersonaFixture, credential: &Envelope) -> Envelope {
        let _ = (issuer, credential);
        unimplemented!("RUN-ATTEST-02: supersede pending")
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
pub fn mint(
    state: &mut IssuerState,
    issuer: &PersonaFixture,
    member: MemberRef,
    subject: &PersonaFixture,
    kinds: &[PredicateKind],
    performed_on: DateClaim,
    entropy: MintEntropy,
) -> Result<MintOutput, MintRefusal> {
    let _ = (state, issuer, member, subject, kinds, performed_on, entropy);
    unimplemented!("RUN-ATTEST-02: mint ceremony pending")
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
    let _ = (bytes, issuer);
    unimplemented!("RUN-ATTEST-02: credential verification pending")
}

/// Verify a status response's signature against the issuer key.
pub fn verify_status_response(resp: &StatusResponse, issuer: &PersonaId) -> bool {
    let _ = (resp, issuer);
    unimplemented!("RUN-ATTEST-02: status response verification pending")
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
    let _ = (credential_bytes, issuer, status, dial);
    unimplemented!("RUN-ATTEST-02: verifier acceptance pending")
}

/// The covenant audit over PUBLIC lineage bytes (T-PA6.2): every epoch record
/// decodes, its signature verifies, every commitment is well-formed (32
/// bytes), the declared total matches the set, epochs are contiguous from 1 —
/// while resolving ZERO persona identities (the report is totals only).
pub fn audit_lineage(bytes: &[u8], issuer: &PersonaId) -> Result<AuditReport, AuditFailure> {
    let _ = (bytes, issuer);
    unimplemented!("RUN-ATTEST-02: lineage audit pending")
}
