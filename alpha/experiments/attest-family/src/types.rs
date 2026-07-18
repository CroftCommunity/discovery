//! The attestation family's vocabulary as types.
//!
//! One family, two axes: **subject type** ([`SubjectRef`]: persona | thing) and
//! **consent mode** ([`ConsentMode`]). Everything else is shared machinery.
//!
//! Two deliberate absences are load-bearing:
//! - **No numeric trust/score/rating/rank field exists on any public type**
//!   (T-AT0.2 compile-boundary invariant; see the doc-tests on
//!   [`crate::query::CorroborationStructure`]).
//! - **No wall-clock value participates in ordering.** Wall-clock appears only
//!   inside payloads as asserted claims ([`DateClaim`] — an issuer's "sighted on
//!   2026-07-17"), never in the envelope, never in fold order (T-AT0.4).

use crate::canonical;
use ipld_core::ipld::Ipld;

// ---------------------------------------------------------------------------
// Identifiers
// ---------------------------------------------------------------------------

/// A persona: one holder-controlled keypair, named by its Ed25519 verifying-key
/// bytes. The holder↔persona linkage exists ONLY in fixture bookkeeping — no
/// payload type in this crate has a field that could carry a holder identifier
/// (T-AT4.3's correlation-resistance floor is a type-level fact first).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PersonaId(pub [u8; 32]);

/// A thing subject (business, product, work), named by a 32-byte identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ThingId(pub [u8; 32]);

/// Content address of a folded object: BLAKE3 of `canonical_bytes_with_sig`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ObjectId(pub [u8; 32]);

impl std::fmt::Display for ObjectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for b in &self.0[..8] {
            write!(f, "{b:02x}")?;
        }
        Ok(())
    }
}

/// The subject axis: an attestation is about a persona or about a thing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SubjectRef {
    Persona(PersonaId),
    Thing(ThingId),
}

/// The consent axis. `Mutual` = co-signed edge (the friend case);
/// `UnilateralNotice` = subject notified, signed reply allowed, no countersign
/// required (the review case); `UnilateralPrivate` = note to self —
/// OWNER-CALL: OC-4 DECIDED (V3, 2026-07-18, owner-confirmed in chat):
/// deferred from v1; when it ships, it ships as a private-substrate artifact
/// (an MLS-group-of-one) under the private tier's own logic, never as a
/// fourth public consent mode. Zero tests remains the deliberate statement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConsentMode {
    Mutual,
    UnilateralNotice,
    UnilateralPrivate,
}

impl ConsentMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            ConsentMode::Mutual => "mutual",
            ConsentMode::UnilateralNotice => "unilateral_notice",
            ConsentMode::UnilateralPrivate => "unilateral_private",
        }
    }
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "mutual" => Some(ConsentMode::Mutual),
            "unilateral_notice" => Some(ConsentMode::UnilateralNotice),
            "unilateral_private" => Some(ConsentMode::UnilateralPrivate),
            _ => None,
        }
    }
}

/// A named scope ("would hire as contractor"). Scope match is EXACT string
/// equality — adjacent scopes never bleed (T-AT3.2). A scope is a label on a
/// claim, not a category tree; no protocol semantics attach to its text.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Scope(pub String);

impl Scope {
    pub fn new(s: &str) -> Self {
        Scope(s.to_string())
    }
}

// ---------------------------------------------------------------------------
// Antecedent kinds — the closed qualifying class (V1, RUN-ATTEST-03)
// ---------------------------------------------------------------------------

/// The **closed class** of antecedents that can stand a vouch up (OWNER-CALL
/// OC-2 DECIDED (V1, 2026-07-18, owner-confirmed in chat): a vouch requires a
/// qualifying antecedent from this class — not an edge specifically). These
/// are shapes of one provenance mechanism; what varies is the kind of trust
/// bound — bidirectional (`co_signed_edge`) or unidirectional (`transaction`,
/// `ceremony`) — and both are valid.
///
/// The class is closed at the compile boundary (the T-AT6.1 style): there is
/// no string escape hatch, so an antecedent kind outside the enum is
/// unrepresentable — adding a kind is a source change. Which kinds *qualify*
/// at fold time is additionally a governed register on the reused R7
/// machinery ([`crate::fold::AntecedentRegister`]): widening the class later
/// is a quorum act with lineage, not a code edit.
///
/// ```compile_fail
/// use attest_family::types::AntecedentKind;
/// // No string escape hatch exists — a kind outside the enum is a type
/// // error, not a data value.
/// let k: AntecedentKind = "notarized_selfie".into();
/// ```
///
/// ```compile_fail
/// use attest_family::types::AntecedentKind;
/// // The enum is closed: no such variant exists.
/// let k = AntecedentKind::Hearsay;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AntecedentKind {
    /// A co-signed edge (both halves folded) — the bidirectional bind.
    CoSignedEdge,
    /// A transaction attestation naming author as payer, subject as payee.
    Transaction,
    /// A co-presence ceremony fact naming exactly {author, subject}.
    Ceremony,
}

impl AntecedentKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            AntecedentKind::CoSignedEdge => "co_signed_edge",
            AntecedentKind::Transaction => "transaction",
            AntecedentKind::Ceremony => "ceremony",
        }
    }
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "co_signed_edge" => Some(AntecedentKind::CoSignedEdge),
            "transaction" => Some(AntecedentKind::Transaction),
            "ceremony" => Some(AntecedentKind::Ceremony),
            _ => None,
        }
    }

    /// The grade a qualifying antecedent of this kind contributes (T-A3.3:
    /// kind set ↔ grade set, exact). Grade remains metadata-only; this
    /// mapping lives here so the fold's grade lines stay assignment-only.
    pub fn grade(&self) -> Grade {
        match self {
            AntecedentKind::CoSignedEdge => Grade::EdgeBacked,
            AntecedentKind::Transaction => Grade::TransactionBacked,
            AntecedentKind::Ceremony => Grade::CeremonyBacked,
        }
    }
}

// ---------------------------------------------------------------------------
// Provenance grade — metadata, never an input
// ---------------------------------------------------------------------------

/// Provenance grade: how an attestation's ceremony/antecedent structure was
/// formed. Grade is **provenance metadata only** — it is set by the fold and
/// consumed by serialization/display, and by nothing else. It deliberately
/// implements neither `PartialOrd` nor `Ord`, so no code path can compare,
/// rank, or fold on it — a compile-boundary fact (T-AT1.7, T-AT2.4):
///
/// ```compile_fail
/// use attest_family::types::Grade;
/// // Grades cannot be compared — grade is metadata, never an ordering input.
/// let _ = Grade::InPerson > Grade::Remote;
/// ```
///
/// ```compile_fail
/// use attest_family::types::Grade;
/// let mut v = [Grade::Remote, Grade::InPerson];
/// v.sort(); // no Ord — grade cannot participate in any ordering
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grade {
    /// Both edge personas co-signed a shared co-presence session fact.
    InPerson,
    /// No co-presence ceremony backs the attestation.
    Remote,
    /// The attestation cites a transaction attestation as antecedent.
    TransactionBacked,
    /// V1 (RUN-ATTEST-03): the vouch is backed by a qualifying co-signed
    /// edge antecedent.
    EdgeBacked,
    /// V1 (RUN-ATTEST-03): the vouch is backed by a qualifying ceremony-fact
    /// antecedent.
    CeremonyBacked,
}

impl Grade {
    pub fn as_str(&self) -> &'static str {
        match self {
            Grade::InPerson => "in_person",
            Grade::Remote => "remote",
            Grade::TransactionBacked => "transaction_backed",
            Grade::EdgeBacked => "edge_backed",
            Grade::CeremonyBacked => "ceremony_backed",
        }
    }
}

/// A wall-clock value **as an asserted claim inside a payload** (an issuer's
/// "sighted on 2026-07-17"). It is signed content like any other claim; it
/// never influences fold ordering or conflict outcomes (T-AT0.4). The only
/// consumer outside serialization is the freshness *presentation* dial
/// (T-AT5.5), which marks entries stale — it never drops or reorders them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateClaim {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

impl DateClaim {
    pub fn new(year: u16, month: u8, day: u8) -> Self {
        DateClaim { year, month, day }
    }

    /// Coarse day-count for the freshness *presentation* dial only (T-AT5.5).
    /// Never used in fold order or conflict resolution.
    pub fn approx_days(&self) -> i64 {
        self.year as i64 * 365 + self.month as i64 * 30 + self.day as i64
    }
}

// ---------------------------------------------------------------------------
// Payloads — the attestation kinds
// ---------------------------------------------------------------------------

/// The canonical **shared core** of a mutual edge: both persona ids, an edge
/// nonce (the edge id), the consent mode, and the ceremony fact references.
/// Both halves must reference the same core hash for the edge to exist
/// (T-AT1.2/1.3). The per-side label is NOT part of the core (T-AT1.4).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EdgeCore {
    /// The two participants, in canonical (byte-ascending) order.
    pub persona_a: PersonaId,
    pub persona_b: PersonaId,
    /// The edge id: a nonce agreed by the pair (fixture-supplied here).
    pub edge_nonce: [u8; 16],
    /// Always `Mutual` for a well-formed edge; carried explicitly so a
    /// tampered mode changes the core hash (T-AT1.3).
    pub consent: ConsentMode,
    /// Ceremony fact object ids (co-presence session facts), sorted.
    pub ceremony: Vec<ObjectId>,
}

impl EdgeCore {
    /// BLAKE3 of the core's canonical dag-cbor bytes — the edge's identity.
    pub fn core_hash(&self) -> [u8; 32] {
        *blake3::hash(&canonical::encode_edge_core(self)).as_bytes()
    }

    pub fn participants(&self) -> [PersonaId; 2] {
        [self.persona_a, self.persona_b]
    }
}

/// One side's half of a mutual edge. A half is not an edge (T-AT1.1): alone it
/// folds to pending, and pending is never partial.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EdgeHalf {
    pub core: EdgeCore,
    /// Side-local label ("friend from school" vs "roommate's sister") — free
    /// text, outside the core hash, may differ per side (T-AT1.4).
    pub label: String,
}

/// Ends (supersedes) an edge. The prior halves stay in lineage, bytes
/// unchanged (T-AT0.3, T-AT1.5). Nothing is revoked in place.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EdgeDissolve {
    /// The edge being ended, by core hash.
    pub core_hash: [u8; 32],
    /// The half object ids being superseded (lineage pointers).
    pub supersedes: Vec<ObjectId>,
}

/// A co-presence session fact: one participant's signed statement of a shared
/// session. Grade `in_person` requires one such fact from EACH participant
/// naming the same session (T-AT1.7).
///
/// A ceremony fact is a completed historical event: it has no "ended" state,
/// no supersede pointer, and no payload kind targets it — so a
/// superseded-antecedent marker is **unrepresentable** for ceremony-backed
/// vouches (T-A3.5, V2), not merely unset:
///
/// ```compile_fail
/// use attest_family::types::*;
/// let c = CeremonyFact {
///     session: [0; 16],
///     participants: [PersonaId([0; 32]), PersonaId([1; 32])],
///     sighted_on: DateClaim::new(2026, 7, 17),
///     supersedes: None, // no such field exists — a session happened or it didn't
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CeremonyFact {
    pub session: [u8; 16],
    pub participants: [PersonaId; 2],
    /// Asserted claim, not an ordering input.
    pub sighted_on: DateClaim,
}

/// The verified-purchase analog — a fixture fact standing in for a payment
/// rail (declared stand-in, §3). Citing one as antecedent yields grade
/// `transaction_backed` (T-AT2.4).
///
/// A completed transaction has no "ended" state (T-A3.5, V2): no supersede
/// pointer, no dissolve analog, no payload kind that targets it — so a
/// superseded-antecedent marker is **unrepresentable** for transaction-backed
/// vouches, not merely unset:
///
/// ```compile_fail
/// use attest_family::types::*;
/// let t = TransactionFact {
///     payer: PersonaId([0; 32]),
///     payee: SubjectRef::Persona(PersonaId([1; 32])),
///     occurred_on: DateClaim::new(2026, 6, 20),
///     supersedes: None, // no such field exists — a purchase happened or it didn't
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransactionFact {
    pub payer: PersonaId,
    pub payee: SubjectRef,
    /// Asserted claim, not an ordering input.
    pub occurred_on: DateClaim,
}

/// Declares a thing subject (business, product, work) and its controlling
/// persona — the persona that may sign replies to reviews of the thing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThingDecl {
    pub thing: ThingId,
    pub kind: ThingKind,
    pub controller: PersonaId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThingKind {
    Business,
    Product,
    Work,
}

impl ThingKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            ThingKind::Business => "business",
            ThingKind::Product => "product",
            ThingKind::Work => "work",
        }
    }
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "business" => Some(ThingKind::Business),
            "product" => Some(ThingKind::Product),
            "work" => Some(ThingKind::Work),
            _ => None,
        }
    }
}

/// A scoped vouch: a separate, later, unilateral claim by one persona about
/// another, in a named scope, standing on at least one resolvable qualifying
/// antecedent from the closed [`AntecedentKind`] class — a co-signed edge, a
/// transaction attestation, or a ceremony fact. Vouches supersede
/// independently of their antecedents (T-AT2.2).
///
/// OWNER-CALL: OC-2 DECIDED (V1, 2026-07-18, owner-confirmed in chat) —
/// option B: the qualifying antecedent comes from the closed class, not an
/// edge specifically. A vouch with zero qualifying antecedents folds to
/// pending, never to a standing vouch (T-A3.2, restating T-AT2.1's
/// discipline).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vouch {
    pub subject: PersonaId,
    pub scope: Scope,
    /// Asserted claim text — the protocol never evaluates it (utility is
    /// computed by humans at the edges, never by the protocol).
    pub statement: String,
    /// The base edge, by core hash, when the vouch is layered on a co-signed
    /// edge. `None` for an edge-free vouch standing on a transaction or
    /// ceremony antecedent (V1). When present, the envelope's antecedents
    /// must resolve this edge's halves for the edge kind to qualify.
    pub base_edge: Option<[u8; 32]>,
    /// Asserted claim, not an ordering input.
    pub made_on: DateClaim,
    /// Narrowing/replacement lineage pointer.
    pub supersedes: Option<ObjectId>,
}

/// Withdraws a vouch by superseding it. The withdrawn vouch's bytes persist in
/// lineage (T-AT0.3); nothing is deleted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VouchWithdraw {
    pub supersedes: ObjectId,
}

/// A review: `unilateral_notice` mode. Stands with only the author's signature
/// (T-AT5.1) — a business would countersign only praise, so integrity comes
/// from provenance structure, not subject consent. Folding one deterministically
/// emits a notice fact addressed to the subject (T-AT5.2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Review {
    pub subject: SubjectRef,
    pub scope: Scope,
    pub statement: String,
    pub consent: ConsentMode,
    /// Asserted claim, not an ordering input; feeds only the freshness
    /// *presentation* dial (T-AT5.5).
    pub made_on: DateClaim,
    pub supersedes: Option<ObjectId>,
}

/// The subject's signed reply — a peer object attached to the review. The
/// review's bytes are unchanged by a reply (T-AT5.3); the corroboration
/// structure returns both.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reply {
    pub review: ObjectId,
    pub statement: String,
    pub made_on: DateClaim,
}

/// An issuer predicate ("over_18") about a persona: predicate, subject,
/// process-provenance metadata. The issuer is the envelope author. The
/// substrate (ID number, card number) is **unrepresentable**: every field is a
/// closed enum or fixed-shape claim — there is no field capable of carrying
/// substrate (T-AT6.1). Attempting it does not compile:
///
/// ```compile_fail
/// use attest_family::types::*;
/// let p = Predicate {
///     predicate: PredicateKind::Over18,
///     subject: PersonaId([0; 32]),
///     process: ProcessProvenance {
///         method: MethodKind::DocumentSighted,
///         performed_on: DateClaim::new(2026, 7, 17),
///         role: IssuerRole::CoopIssuer,
///     },
///     supersedes: None,
///     id_number: "042-68-4425".to_string(), // no such field exists
/// };
/// ```
///
/// ```compile_fail
/// use attest_family::types::*;
/// // The method is a closed enum — free-form process text (where substrate
/// // could hide) is a type error, not a convention.
/// let m: MethodKind = "saw drivers license #D1234567".into();
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Predicate {
    pub predicate: PredicateKind,
    pub subject: PersonaId,
    pub process: ProcessProvenance,
    /// Refresh lineage (e.g. re-verified phone) — supersede, never expiry
    /// (T-AT6.3).
    pub supersedes: Option<ObjectId>,
}

/// Closed set of predicates the co-op issuer asserts. Adding a predicate is a
/// vocabulary change (a governed act), not a data change.
///
/// `VettedHolder` (RUN-ATTEST-02) is the reality anchor: "a vetted human
/// stands behind this persona." It is NOT proof of unique personhood — one
/// human may hold several anchor personas, and no operation answers whether
/// two personas share a holder (T-PA5.3, the pin now unqualified).
/// `sole_anchor(context)` is REJECTED (V7, 2026-07-18): uniqueness is
/// group-local membership vetting under local authority — governance counts
/// member handles; personas sign — never a portable credential. Portable
/// proof-of-personhood is the escalation the design refuses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PredicateKind {
    Over18,
    PhoneVerified,
    PaymentVerified,
    VettedHolder,
}

impl PredicateKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            PredicateKind::Over18 => "over_18",
            PredicateKind::PhoneVerified => "phone_verified",
            PredicateKind::PaymentVerified => "payment_verified",
            PredicateKind::VettedHolder => "vetted_holder",
        }
    }
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "over_18" => Some(PredicateKind::Over18),
            "phone_verified" => Some(PredicateKind::PhoneVerified),
            "payment_verified" => Some(PredicateKind::PaymentVerified),
            "vetted_holder" => Some(PredicateKind::VettedHolder),
            _ => None,
        }
    }
}

/// Process-provenance metadata: how the issuer verified, when (as a claim),
/// in what role. Every serialization of a predicate carries this — no code
/// path yields a bare "over_18: true" (T-AT6.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProcessProvenance {
    pub method: MethodKind,
    pub performed_on: DateClaim,
    pub role: IssuerRole,
}

/// Closed set of verification methods (no free-form field where substrate
/// could hide).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MethodKind {
    DocumentSighted,
    SmsRoundTrip,
    CardAuthorization,
}

impl MethodKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            MethodKind::DocumentSighted => "document_sighted",
            MethodKind::SmsRoundTrip => "sms_round_trip",
            MethodKind::CardAuthorization => "card_authorization",
        }
    }
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "document_sighted" => Some(MethodKind::DocumentSighted),
            "sms_round_trip" => Some(MethodKind::SmsRoundTrip),
            "card_authorization" => Some(MethodKind::CardAuthorization),
            _ => None,
        }
    }
}

/// The issuer's role marker (the co-op issuer stand-in, §3).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IssuerRole {
    CoopIssuer,
}

impl IssuerRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            IssuerRole::CoopIssuer => "coop_issuer",
        }
    }
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "coop_issuer" => Some(IssuerRole::CoopIssuer),
            _ => None,
        }
    }
}

// ---------------------------------------------------------------------------
// RUN-ATTEST-02 — anchor-persona payloads
// ---------------------------------------------------------------------------

/// The vetting-event stand-in (RUN-ATTEST-02 §3): the issuer's signed
/// statement that a vetting ceremony stood behind THIS persona. Emitted once
/// per mint call, per persona, with a fresh nonce — never shared across
/// sibling personas (independent derivation, T-PA2.2). A credential without a
/// resolvable vetting antecedent folds to pending, never standing (T-PA3.1).
///
/// The organizational fact that one human's vetting backed several personas
/// lives ONLY in issuer retained state at the named `SeamBoundary` — it is not
/// representable here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VettingFact {
    pub subject: PersonaId,
    /// Fresh per mint call (derived from single-use mint entropy).
    pub vetting_nonce: [u8; 16],
    /// Asserted claim, not an ordering input.
    pub performed_on: DateClaim,
    pub role: IssuerRole,
}

/// A single-predicate credential (RUN-ATTEST-02 §3): the anchor-persona mint
/// unit. Exactly one predicate per credential — bundles exist only as
/// presentation-side composition (T-PA4.4). Like [`Predicate`], the substrate
/// is unrepresentable (closed enums and fixed-shape claims only) and the
/// issuer IS the envelope author. Unlike [`Predicate`], a credential:
/// - requires a vetting-event antecedent to stand (T-PA3.1);
/// - carries a fresh per-mint nonce so no two credentials share derivation
///   state (T-PA2.1, T-PA2.2).
///
/// Deliberate absences (T-PA1.1): no ordinal, no `primary`, no rank, no
/// sequence field — nothing in any public object designates one of a
/// holder's personas as first or preferred. An observer's total knowledge is
/// "this persona carries the predicate; that one does not."
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Credential {
    pub predicate: PredicateKind,
    pub subject: PersonaId,
    pub process: ProcessProvenance,
    /// Fresh per mint (single-use entropy; reuse is refused — T-PA2.2).
    pub mint_nonce: [u8; 16],
    /// V5/V6 (RUN-ATTEST-04): the governance-era anchor this credential was
    /// minted or reissued under — issuer operational time IS governance time,
    /// so membership is era-graded, "meaningful but factual". An era fact,
    /// never a status: old-era credentials never expire, and silence carries
    /// no penalty (T-A4.16).
    pub era: [u8; 32],
    /// Refresh lineage — supersede, never expiry (T-AT6.3 discipline).
    pub supersedes: Option<ObjectId>,
}

/// V5/V6 era-reissue (RUN-ATTEST-04): a HOLDER-signed request to reissue an
/// existing credential under a new governance era. Unilateral — only the
/// holder's signature exists on it (the issuer refuses a request whose author
/// is not the credential's subject); it cites the existing credential and the
/// new era anchor. The issuer's reissue chains the ORIGINAL vetting event —
/// no new vetting antecedent exists, which is also V8's free-reissue
/// structural pin (T-A4.14). Voluntary: not requesting one changes nothing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReissueRequest {
    /// The holder's existing credential, by content address.
    pub credential: ObjectId,
    /// The governance-lineage fact opening the era being reissued into.
    pub era_anchor: [u8; 32],
}

/// The issuer's supersede marker for a credential (revocation-equivalent,
/// without replacement). The superseded credential's bytes persist in lineage
/// unchanged (T-AT0.3); a status check answers `superseded` (T-PA6.4).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CredentialSupersede {
    pub supersedes: ObjectId,
}

/// A persona's resolvability policy: who may resolve this persona in query
/// traversal. Governed by the NAMED party — the envelope author must be the
/// persona itself (T-AT4.1); an edge holder's disclosure choices can never
/// grant resolution of the far end. Policy changes are superseding facts with
/// lineage, not mutations (T-AT4.2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvabilityPolicy {
    pub persona: PersonaId,
    pub rule: PolicyRule,
    pub supersedes: Option<ObjectId>,
}

/// Stand-in policy language (§3: the resolvability policy is an in-memory
/// table): allow everyone, or allow an explicit viewer list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyRule {
    AllowAll,
    AllowOnly(Vec<PersonaId>),
}

/// Every attestation kind in the family. One enum, one shared envelope, one
/// fold — the "one attestation family" claim as a type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Payload {
    EdgeHalf(EdgeHalf),
    EdgeDissolve(EdgeDissolve),
    CeremonyFact(CeremonyFact),
    TransactionFact(TransactionFact),
    ThingDecl(ThingDecl),
    Vouch(Vouch),
    VouchWithdraw(VouchWithdraw),
    Review(Review),
    Reply(Reply),
    Predicate(Predicate),
    ResolvabilityPolicy(ResolvabilityPolicy),
    VettingFact(VettingFact),
    Credential(Credential),
    CredentialSupersede(CredentialSupersede),
    ReissueRequest(ReissueRequest),
}

impl Payload {
    pub fn kind_str(&self) -> &'static str {
        match self {
            Payload::EdgeHalf(_) => "edge_half",
            Payload::EdgeDissolve(_) => "edge_dissolve",
            Payload::CeremonyFact(_) => "ceremony_fact",
            Payload::TransactionFact(_) => "transaction_fact",
            Payload::ThingDecl(_) => "thing_decl",
            Payload::Vouch(_) => "vouch",
            Payload::VouchWithdraw(_) => "vouch_withdraw",
            Payload::Review(_) => "review",
            Payload::Reply(_) => "reply",
            Payload::Predicate(_) => "predicate",
            Payload::ResolvabilityPolicy(_) => "resolvability_policy",
            Payload::VettingFact(_) => "vetting_fact",
            Payload::Credential(_) => "credential",
            Payload::CredentialSupersede(_) => "credential_supersede",
            Payload::ReissueRequest(_) => "reissue_request",
        }
    }
}

// ---------------------------------------------------------------------------
// Envelope
// ---------------------------------------------------------------------------

/// The signed container for every attestation. Note what is ABSENT: there is
/// no envelope timestamp. Ordering inputs are cryptographic/logical only —
/// (lamport, author bytes, object id). Wall-clock lives inside payloads as
/// claims (T-AT0.4).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Envelope {
    /// Wire version; always 1 for this generation.
    pub version: u8,
    /// The authoring persona (Ed25519 verifying key).
    pub author: PersonaId,
    /// Logical clock — per-author monotonic counter, a cryptographically
    /// committed (signed) logical value, not wall-clock.
    pub lamport: u64,
    /// Object ids this attestation causally cites (approvals-as-antecedents,
    /// the R7 shape).
    pub antecedents: Vec<ObjectId>,
    pub payload: Payload,
    /// Detached Ed25519 signature over `canonical_bytes()`.
    pub signature: Vec<u8>,
}

impl Envelope {
    /// Canonical dag-cbor bytes of everything except the signature.
    pub fn canonical_bytes(&self) -> Vec<u8> {
        canonical::encode_envelope(self, false)
    }

    /// Canonical dag-cbor bytes including the signature — the stored/wire form.
    pub fn canonical_bytes_with_sig(&self) -> Vec<u8> {
        canonical::encode_envelope(self, true)
    }

    /// BLAKE3 of `canonical_bytes_with_sig` — the object's content address.
    pub fn object_id(&self) -> ObjectId {
        ObjectId(*blake3::hash(&self.canonical_bytes_with_sig()).as_bytes())
    }
}

/// Fold-order comparison: lamport ASC → author bytes ASC → object id ASC.
/// Mirrors the substrate's `merge_cmp` (logical + cryptographic inputs only —
/// no wall-clock anywhere in this function, T-AT0.4).
pub fn fold_cmp(a: &Envelope, b: &Envelope) -> std::cmp::Ordering {
    a.lamport
        .cmp(&b.lamport)
        .then_with(|| a.author.0.cmp(&b.author.0))
        .then_with(|| a.object_id().0.cmp(&b.object_id().0))
}

/// Ipld leaf helpers shared by canonical + tests.
pub fn ipld_bytes(b: &[u8]) -> Ipld {
    Ipld::Bytes(b.to_vec())
}
