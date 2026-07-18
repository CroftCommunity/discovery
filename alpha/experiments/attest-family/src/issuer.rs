//! RUN-ATTEST-02/04 — the co-op issuer: mint ceremony, retained state, and
//! the V5/V6 issuer-transparency model (RUN-ATTEST-04 Part B).
//!
//! **Publication (V5)**: per epoch, the issuer publishes ONE signed tree head
//! (CT/RFC-9162 shape): a Merkle root over KEYED commitments in canonical
//! order by commitment value, the leaf count, the superseded-set root, and
//! the governance-era anchor. Nothing per-credential exists on the public
//! surface (T-A4.6); the per-credential public receipt pile of RUN-ATTEST-02
//! and its unordered-set fold are REMOVED (test scaffolding and
//! issuer-internal structures are code, and code changes by commit — this is
//! not a Drystone-tier lineage violation).
//!
//! **Confirmation (V5)**: holder-STAPLED inclusion proofs. At mint the holder
//! receives its commitment + an issuer-signed binding; after the covering
//! head publishes, the holder fetches its proof over the holder channel
//! ([`IssuerState::holder_proof`] — self-authenticating by knowledge of the
//! commitment, which T-A4.8 keeps unguessable). Verification is the pure
//! function [`verify_staple`]: the verifier NEVER contacts the co-op — the
//! T-PA6.3 status-check endpoint and its (verifier, subject) capture leak are
//! DELETED (T-A4.11).
//!
//! **Revocation (V5)**: the per-epoch superseded set, published beside each
//! head with its root inside the signed portion. A superseded credential
//! cannot staple a fresh-head proof (T-A4.10). Freshness requirements remain
//! app policy, fail-closed as always ([`CheckDial`]) — restated, not
//! re-derived.
//!
//! **Eras (V6 + era-reissue)**: issuer operational time IS governance time —
//! the co-op is literally a Drystone group, and its governance lineage is the
//! era spine. The commitment key is derived per era (`key_e = KDF(coop_secret,
//! era_anchor_e)` — a MODELED choice, flagged owner-revisitable, not an OC:
//! rotation rides era rolls by construction and a leaked key exposes exactly
//! one era). Head cadence is the seed-rule shape — epoch roll OR N facts, the
//! N mirrored from the reused R7 register (T-A4.9) — and NO wall-clock input
//! exists anywhere in this pipeline.
//!
//! The issuer-state type stays deliberately narrow (T-PA6.1): closed enums,
//! fixed-size arrays, counters, and maps/sets of those; no `String`, no free
//! bytes, no `PersonaId` anywhere in retained state. Holder linkage lives
//! only at the named [`SeamBoundary`].

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
/// member is never public (T-PA3.3). V8 note: the fee attaches to the vetting
/// event and the mint act — era-reissues never touch this seam (T-A4.14).
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
/// The runtime half: re-deriving from the same seed yields the same mint
/// nonces, and the issuer state deterministically refuses a reused nonce.
pub struct MintEntropy {
    pub(crate) seed: [u8; 32],
}

impl MintEntropy {
    pub fn from_seed(seed: [u8; 32]) -> Self {
        MintEntropy { seed }
    }
}

/// Deterministic mint refusals (T-PA2.2, T-PA3.2). Checked in this order:
/// dial, then nonce reuse; a refused mint leaves state unchanged.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MintRefusal {
    /// The member is at the governed max-anchors dial.
    DialExceeded,
    /// A derived mint nonce was already used — shared derivation state.
    NonceReused,
}

/// Deterministic reissue refusals (T-A4.14): the request must be the
/// HOLDER'S OWN act, name the credential it cites, and target the current
/// era. A refused reissue leaves state unchanged.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReissueRefusal {
    /// The request envelope's author is not the credential's subject —
    /// reissue is unilateral and holder-signed, or it is nothing.
    NotHolderSigned,
    /// The request payload is not a reissue request naming this credential.
    WrongCredential,
    /// The request's era anchor is not the issuer's current era.
    WrongEra,
    /// The cited credential is not in this issuer's assertion lineage.
    UnknownCredential,
    /// The derived mint nonce was already used.
    NonceReused,
}

/// What the PERSONA receives from a mint ceremony: its vetting fact, its
/// single-predicate credentials, and — new in V5 — one HOLDER-HELD binding
/// per credential (commitment + issuer binding signature, the staple seed).
/// Bindings are holder-held, never published; what the ISSUER publishes is
/// exactly the signed tree heads.
#[derive(Debug, Clone)]
pub struct MintOutput {
    pub vetting: Envelope,
    pub credentials: Vec<Envelope>,
    pub bindings: Vec<HolderBinding>,
}

/// What the holder receives from an era-reissue (T-A4.14): the fresh
/// credential (chaining the ORIGINAL vetting — no new vetting antecedent) and
/// its holder-held binding under the new era key.
#[derive(Debug, Clone)]
pub struct ReissueOutput {
    pub credential: Envelope,
    pub binding: HolderBinding,
}

// ---------------------------------------------------------------------------
// Keyed commitments, Merkle trees, and staples (V5)
// ---------------------------------------------------------------------------

/// The genesis era anchor — the stand-in for the governance-lineage fact that
/// opened the co-op's first era (declared stand-in; real anchors are envelope
/// hashes from the co-op's own Drystone governance lineage, T-A4.9).
pub fn genesis_era() -> [u8; 32] {
    *blake3::hash(b"attest-era:genesis").as_bytes()
}

fn leaf_hash(commitment: &[u8; 32]) -> [u8; 32] {
    let mut h = blake3::Hasher::new();
    h.update(&[0x00]);
    h.update(commitment);
    *h.finalize().as_bytes()
}

fn node_hash(left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
    let mut h = blake3::Hasher::new();
    h.update(&[0x01]);
    h.update(left);
    h.update(right);
    *h.finalize().as_bytes()
}

fn empty_root() -> [u8; 32] {
    *blake3::hash(b"attest-tree:empty").as_bytes()
}

/// Merkle root over leaves in canonical order (byte-ascending commitment
/// values — mint order is structurally absent, T-A4.7). Odd nodes promote.
fn merkle_root(leaves: &[[u8; 32]]) -> [u8; 32] {
    if leaves.is_empty() {
        return empty_root();
    }
    let mut level: Vec<[u8; 32]> = leaves.iter().map(leaf_hash).collect();
    while level.len() > 1 {
        level = level
            .chunks(2)
            .map(|pair| if pair.len() == 2 { node_hash(&pair[0], &pair[1]) } else { pair[0] })
            .collect();
    }
    level[0]
}

/// Inclusion path for `index`: (sibling-is-right, sibling hash) per level.
fn merkle_proof(leaves: &[[u8; 32]], index: usize) -> Vec<(bool, [u8; 32])> {
    let mut proof = Vec::new();
    let mut level: Vec<[u8; 32]> = leaves.iter().map(leaf_hash).collect();
    let mut i = index;
    while level.len() > 1 {
        let sibling = if i.is_multiple_of(2) { i + 1 } else { i - 1 };
        if sibling < level.len() {
            proof.push((sibling > i, level[sibling]));
        }
        level = level
            .chunks(2)
            .map(|pair| if pair.len() == 2 { node_hash(&pair[0], &pair[1]) } else { pair[0] })
            .collect();
        i /= 2;
    }
    proof
}

/// Pure inclusion check: walk the path from the commitment's leaf hash to the
/// root. Takes nothing but bytes — no issuer, no state, no network.
pub fn verify_inclusion(root: &[u8; 32], commitment: &[u8; 32], proof: &[(bool, [u8; 32])]) -> bool {
    let mut acc = leaf_hash(commitment);
    for (sibling_is_right, sibling) in proof {
        acc = if *sibling_is_right { node_hash(&acc, sibling) } else { node_hash(sibling, &acc) };
    }
    acc == *root
}

/// Canonical digest of a superseded set: dag-cbor of the byte-ascending list.
fn superseded_digest(set: &BTreeSet<[u8; 32]>) -> [u8; 32] {
    let list = Ipld::List(set.iter().map(|c| Ipld::Bytes(c.to_vec())).collect());
    let bytes = serde_ipld_dagcbor::to_vec(&list).expect("pure value encode cannot fail");
    *blake3::hash(&bytes).as_bytes()
}

/// ONE signed tree head — the issuer's ENTIRE per-epoch publication (V5,
/// T-A4.6): Merkle root over the era's keyed commitments (canonical order by
/// commitment value), leaf count, superseded-set root, era anchor reference,
/// issuer signature. The superseded set itself is published beside the head
/// (its root is inside the signed portion); it is commitments only — keyed,
/// never per-credential-identifiable (T-A4.8).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreeHead {
    pub epoch_no: u64,
    pub era_anchor: [u8; 32],
    pub leaf_count: u64,
    pub root: [u8; 32],
    pub superseded_root: [u8; 32],
    pub superseded: BTreeSet<[u8; 32]>,
    pub signature: Vec<u8>,
}

impl TreeHead {
    /// The signed portion: {a: era anchor, e: epoch, n: leaf count, r: root,
    /// u: superseded root} — everything the head asserts.
    fn signing_ipld(
        epoch_no: u64,
        era_anchor: &[u8; 32],
        leaf_count: u64,
        root: &[u8; 32],
        superseded_root: &[u8; 32],
    ) -> Ipld {
        let mut m = BTreeMap::new();
        m.insert("a".to_string(), Ipld::Bytes(era_anchor.to_vec()));
        m.insert("e".to_string(), Ipld::Integer(epoch_no as i128));
        m.insert("n".to_string(), Ipld::Integer(leaf_count as i128));
        m.insert("r".to_string(), Ipld::Bytes(root.to_vec()));
        m.insert("u".to_string(), Ipld::Bytes(superseded_root.to_vec()));
        Ipld::Map(m)
    }

    pub(crate) fn signing_bytes(
        epoch_no: u64,
        era_anchor: &[u8; 32],
        leaf_count: u64,
        root: &[u8; 32],
        superseded_root: &[u8; 32],
    ) -> Vec<u8> {
        serde_ipld_dagcbor::to_vec(&Self::signing_ipld(
            epoch_no,
            era_anchor,
            leaf_count,
            root,
            superseded_root,
        ))
        .expect("pure map encode cannot fail")
    }

    pub fn to_ipld(&self) -> Ipld {
        let Ipld::Map(mut m) = Self::signing_ipld(
            self.epoch_no,
            &self.era_anchor,
            self.leaf_count,
            &self.root,
            &self.superseded_root,
        ) else {
            unreachable!("signing form is a map")
        };
        m.insert(
            "d".to_string(),
            Ipld::List(self.superseded.iter().map(|c| Ipld::Bytes(c.to_vec())).collect()),
        );
        m.insert("g".to_string(), Ipld::Bytes(self.signature.clone()));
        Ipld::Map(m)
    }

    pub fn to_canonical_bytes(&self) -> Vec<u8> {
        serde_ipld_dagcbor::to_vec(&self.to_ipld()).expect("pure map encode cannot fail")
    }
}

/// The holder-held mint receipt for ONE credential (never published): the
/// keyed commitment and the issuer's binding signature over
/// {credential hash ↔ commitment}. This is what makes a staple verifiable
/// without the era key: the binding asserts the pairing; the proof asserts
/// tree membership; the verifier checks both from bytes alone.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HolderBinding {
    pub credential_hash: [u8; 32],
    pub commitment: [u8; 32],
    pub signature: Vec<u8>,
}

impl HolderBinding {
    fn signing_ipld(credential_hash: &[u8; 32], commitment: &[u8; 32]) -> Ipld {
        let mut m = BTreeMap::new();
        m.insert("c".to_string(), Ipld::Bytes(commitment.to_vec()));
        m.insert("h".to_string(), Ipld::Bytes(credential_hash.to_vec()));
        Ipld::Map(m)
    }

    pub(crate) fn signing_bytes(credential_hash: &[u8; 32], commitment: &[u8; 32]) -> Vec<u8> {
        serde_ipld_dagcbor::to_vec(&Self::signing_ipld(credential_hash, commitment))
            .expect("pure map encode cannot fail")
    }

    pub fn to_ipld(&self) -> Ipld {
        let Ipld::Map(mut m) = Self::signing_ipld(&self.credential_hash, &self.commitment) else {
            unreachable!("signing form is a map")
        };
        m.insert("g".to_string(), Ipld::Bytes(self.signature.clone()));
        Ipld::Map(m)
    }

    pub fn to_canonical_bytes(&self) -> Vec<u8> {
        serde_ipld_dagcbor::to_vec(&self.to_ipld()).expect("pure map encode cannot fail")
    }
}

/// What a holder presents to a verifier alongside their credential: the
/// commitment, the issuer's binding signature, and the inclusion proof
/// against a published head. Composed holder-side; verified by the pure
/// [`verify_staple`] — no issuer round-trip exists (T-A4.11).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Staple {
    pub commitment: [u8; 32],
    pub binding_sig: Vec<u8>,
    pub proof: Vec<(bool, [u8; 32])>,
}

/// A verifier app's freshness posture (T-PA6.4 discipline, restated for the
/// staple model — not re-derived): the protocol never renders a verdict from
/// silence; requiring a staple against a fresh head (and failing closed
/// without one) is app policy, expressed as the caller's dial.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckDial {
    SignatureOnly,
    RequireFreshStaple,
}

// ---------------------------------------------------------------------------
// What a covenant audit may learn (T-A4.13)
// ---------------------------------------------------------------------------

/// Totals and shape validity ONLY. No persona identity is resolvable from a
/// head lineage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AuditReport {
    pub heads_audited: u64,
    pub total_commitments: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuditFailure {
    Undecodable,
    BadSignature { epoch_no: u64 },
    MalformedSuperseded { epoch_no: u64 },
    SupersededRootMismatch { epoch_no: u64 },
    LeafCountRegressed { epoch_no: u64 },
    NonContiguousEpochs,
    EraReused,
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
/// credential hash), process bookkeeping (epoch counter, used nonces, era
/// anchor, cadence), the era's keyed commitments, the published heads, and
/// the [`SeamBoundary`]. Nothing else is representable. The confidential
/// commitment secret is a fixed-size key, never serialized.
#[derive(Debug)]
pub struct IssuerState {
    pub(crate) epoch_no: u64,
    pub(crate) era_anchor: [u8; 32],
    pub(crate) coop_secret: [u8; 32],
    pub(crate) facts_since_head: u32,
    pub(crate) era_leaves: BTreeSet<[u8; 32]>,
    pub(crate) published_leaves: Vec<[u8; 32]>,
    pub(crate) era_superseded: BTreeSet<[u8; 32]>,
    pub(crate) heads: Vec<TreeHead>,
    pub(crate) used_nonces: BTreeSet<[u8; 16]>,
    pub(crate) commitment_of: BTreeMap<[u8; 32], [u8; 32]>,
    pub(crate) assertions: BTreeMap<[u8; 32], AssertionHead>,
    pub(crate) seam: SeamBoundary,
    pub(crate) dial_max_anchors: u32,
    pub(crate) cadence_n: u32,
}

impl IssuerState {
    pub fn new(dial_max_anchors: u32) -> Self {
        IssuerState {
            epoch_no: 1,
            era_anchor: genesis_era(),
            // Declared stand-in: a fixture confidential secret. The era key
            // is derived, never stored: key_e = KDF(coop_secret, era_anchor_e)
            // — MODELED choice, flagged owner-revisitable (not an OC).
            coop_secret: *blake3::hash(b"attest-coop-secret:fixture").as_bytes(),
            facts_since_head: 0,
            era_leaves: BTreeSet::new(),
            published_leaves: Vec::new(),
            era_superseded: BTreeSet::new(),
            heads: Vec::new(),
            used_nonces: BTreeSet::new(),
            commitment_of: BTreeMap::new(),
            assertions: BTreeMap::new(),
            seam: SeamBoundary::default(),
            dial_max_anchors,
            cadence_n: u32::MAX,
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

    /// Feed the governed head-cadence value from the folded R7 register
    /// (T-A4.9, same mirroring pattern as `set_dial`): a head publishes when
    /// N facts accumulate — OR on an explicit epoch roll. The seed-rule
    /// shape; no timer exists anywhere.
    pub fn set_cadence(&mut self, n_facts: u32) {
        self.cadence_n = n_facts;
    }

    pub fn cadence(&self) -> u32 {
        self.cadence_n
    }

    /// The current governance-era anchor (an era fact, not a status).
    pub fn era(&self) -> [u8; 32] {
        self.era_anchor
    }

    /// key_e = KDF(coop_secret, era_anchor_e). Modeled as a keyed BLAKE3;
    /// flagged owner-revisitable. Rotation rides era rolls by construction —
    /// a leaked key exposes exactly one era.
    fn era_key(&self) -> [u8; 32] {
        *blake3::keyed_hash(&self.coop_secret, &self.era_anchor).as_bytes()
    }

    fn commit(&self, credential_hash: &[u8; 32]) -> [u8; 32] {
        *blake3::keyed_hash(&self.era_key(), credential_hash).as_bytes()
    }

    /// Close the open epoch: publish ONE signed tree head over the era's
    /// commitments (canonical order by commitment value) + the era's
    /// superseded set. The explicit "epoch roll" half of the cadence rule.
    pub fn close_epoch(&mut self, issuer: &PersonaFixture) -> TreeHead {
        self.published_leaves = self.era_leaves.iter().copied().collect();
        let root = merkle_root(&self.published_leaves);
        let superseded = self.era_superseded.clone();
        let superseded_root = superseded_digest(&superseded);
        let leaf_count = self.published_leaves.len() as u64;
        let signature = issuer.sign_bytes(&TreeHead::signing_bytes(
            self.epoch_no,
            &self.era_anchor,
            leaf_count,
            &root,
            &superseded_root,
        ));
        let head = TreeHead {
            epoch_no: self.epoch_no,
            era_anchor: self.era_anchor,
            leaf_count,
            root,
            superseded_root,
            superseded,
            signature,
        };
        self.heads.push(head.clone());
        self.epoch_no += 1;
        self.facts_since_head = 0;
        head
    }

    /// Roll into a new governance era (V6/era-reissue): the era anchor is a
    /// governance-lineage fact — key rotation, tree-head epoching, and
    /// register changes are era'd governance facts of the co-op's own
    /// Drystone group. Publishes the old era's final head, then resets the
    /// era-scoped tree, superseded set, and (by derivation) the commitment
    /// key.
    pub fn roll_era(&mut self, issuer: &PersonaFixture, new_anchor: [u8; 32]) -> TreeHead {
        let closing = self.close_epoch(issuer);
        self.era_anchor = new_anchor;
        self.era_leaves.clear();
        self.published_leaves.clear();
        self.era_superseded.clear();
        closing
    }

    /// The issuer's ENTIRE public surface, canonical bytes (T-A4.6): the
    /// signed tree heads (each with its superseded set) — nothing
    /// per-credential, no identities, no mint order.
    pub fn lineage_bytes(&self) -> Vec<u8> {
        let list = Ipld::List(self.heads.iter().map(|h| h.to_ipld()).collect());
        serde_ipld_dagcbor::to_vec(&list).expect("pure value encode cannot fail")
    }

    /// The HOLDER channel (not a verifier surface, T-A4.11): fetch the
    /// inclusion proof for a commitment against the last published head.
    /// Takes the commitment only — a value known to nobody but the holder
    /// (via its binding) and the issuer (T-A4.8); no subject, no persona, no
    /// verifier query shape exists here.
    pub fn holder_proof(&self, commitment: &[u8; 32]) -> Option<Vec<(bool, [u8; 32])>> {
        let index = self.published_leaves.iter().position(|l| l == commitment)?;
        Some(merkle_proof(&self.published_leaves, index))
    }

    /// Holder convenience: compose the full staple for a binding.
    pub fn holder_staple(&self, binding: &HolderBinding) -> Option<Staple> {
        Some(Staple {
            commitment: binding.commitment,
            binding_sig: binding.signature.clone(),
            proof: self.holder_proof(&binding.commitment)?,
        })
    }

    /// Supersede one of this issuer's credentials: signs a
    /// `credential_supersede` assertion (the persona's published object stays
    /// byte-intact in every log; T-PA6.4), moves the assertion head, and
    /// enters the commitment into the era's superseded set (published with
    /// the NEXT head — T-A4.10). A commitment from an already-closed era has
    /// no live tree to leave; its supersession is visible in the credential's
    /// own supersede lineage.
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
        if let Some(commitment) = self.commitment_of.get(&id.0) {
            if self.era_leaves.contains(commitment) {
                self.era_superseded.insert(*commitment);
            }
        }
        env
    }
}

// ---------------------------------------------------------------------------
// The mint ceremony harness (RUN-ATTEST-02 §3, V5 commitments)
// ---------------------------------------------------------------------------

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

/// Mint one persona's single-predicate credentials. Performs the
/// vetting-event stand-in, derives everything independently per persona from
/// single-use entropy (fresh vetting nonce, fresh per-credential mint
/// nonces), routes retained effects through `state` (dial check at the seam,
/// nonce-reuse refusal, keyed commitment into the era tree, assertion heads,
/// holder bindings), and returns what the PERSONA holds. If the mint's facts
/// reach the governed cadence, the head publishes inside the mint (the
/// "N facts" half of the cadence rule — no timer).
///
/// Envelope lamports are the OPEN EPOCH NUMBER, not a per-mint counter:
/// within an epoch, mint order is not representable in any public object
/// (F-PA-3; the tree's canonical-by-commitment order is T-A4.7's guarantee).
pub fn mint(
    state: &mut IssuerState,
    issuer: &PersonaFixture,
    member: MemberRef,
    subject: &PersonaFixture,
    kinds: &[PredicateKind],
    performed_on: DateClaim,
    entropy: MintEntropy,
) -> Result<MintOutput, MintRefusal> {
    // Refusals are deterministic and ordered — dial, then nonce reuse — and a
    // refused mint changes NOTHING (all checks precede all writes).
    if state.seam.anchors_of(&member) + 1 > state.dial_max_anchors {
        return Err(MintRefusal::DialExceeded);
    }
    let nonces: Vec<[u8; 16]> =
        (0..kinds.len()).map(|k| first16(derive(&entropy.seed, b"nonce", k as u8))).collect();
    if nonces.iter().any(|n| state.used_nonces.contains(n)) {
        return Err(MintRefusal::NonceReused);
    }

    // The vetting-event stand-in: one per mint call, per persona, fresh
    // nonce. Lamport is the OPEN EPOCH NUMBER.
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
    // each citing the vetting fact as antecedent, each carrying the current
    // era anchor as an era fact (V6).
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
                mint_nonce: nonces[k],
                era: state.era_anchor,
                supersedes: None,
            }),
        );
        credentials.push(env);
    }

    // Retained effects, routed through the issuer state (never around it):
    // the seam records the anchor (the fee attaches HERE — mint act +
    // vetting event, V8), nonces are consumed, each credential's KEYED
    // commitment enters the era tree, the assertion head opens as current,
    // and the holder receives its binding.
    state.seam.record_anchor(member);
    let mut bindings: Vec<HolderBinding> = Vec::new();
    for (env, nonce) in credentials.iter().zip(&nonces) {
        state.used_nonces.insert(*nonce);
        let id = env.object_id().0;
        let commitment = state.commit(&id);
        state.era_leaves.insert(commitment);
        state.commitment_of.insert(id, commitment);
        state.assertions.insert(id, AssertionHead { superseded_by: None });
        bindings.push(HolderBinding {
            credential_hash: id,
            commitment,
            signature: issuer.sign_bytes(&HolderBinding::signing_bytes(&id, &commitment)),
        });
        state.facts_since_head += 1;
    }
    if state.facts_since_head >= state.cadence_n {
        state.close_epoch(issuer);
    }
    Ok(MintOutput { vetting, credentials, bindings })
}

/// Era-reissue (V5/V6 refinement, T-A4.14): a holder-signed request — ONLY
/// the holder signs — citing their existing credential and the current era
/// anchor. The issuer supersedes the old credential, CHAINS THE ORIGINAL
/// VETTING EVENT (no new vetting antecedent — V8's free-reissue structural
/// pin: the seam is untouched, no dial check runs, no anchor is recorded),
/// and mints a fresh commitment under the new era key.
pub fn reissue(
    state: &mut IssuerState,
    issuer: &PersonaFixture,
    request: &Envelope,
    old_credential: &Envelope,
    entropy: MintEntropy,
) -> Result<ReissueOutput, ReissueRefusal> {
    let Payload::ReissueRequest(rr) = &request.payload else {
        return Err(ReissueRefusal::WrongCredential);
    };
    let old_id = old_credential.object_id();
    if rr.credential != old_id {
        return Err(ReissueRefusal::WrongCredential);
    }
    let Payload::Credential(old) = &old_credential.payload else {
        return Err(ReissueRefusal::WrongCredential);
    };
    if request.author != old.subject {
        return Err(ReissueRefusal::NotHolderSigned);
    }
    if rr.era_anchor != state.era_anchor {
        return Err(ReissueRefusal::WrongEra);
    }
    if !state.assertions.contains_key(&old_id.0) {
        return Err(ReissueRefusal::UnknownCredential);
    }
    let nonce = first16(derive(&entropy.seed, b"nonce", 0));
    if state.used_nonces.contains(&nonce) {
        return Err(ReissueRefusal::NonceReused);
    }

    // The fresh credential: same predicate/subject/process (the ORIGINAL
    // vetting stands behind it — its antecedents are the old credential's
    // vetting refs plus the holder's request), superseding the old, era'd
    // under the current anchor.
    let mut antecedents = old_credential.antecedents.clone();
    antecedents.push(request.object_id());
    let env = issuer.sign(
        state.epoch_no,
        antecedents,
        Payload::Credential(crate::types::Credential {
            predicate: old.predicate,
            subject: old.subject,
            process: old.process,
            mint_nonce: nonce,
            era: state.era_anchor,
            supersedes: Some(old_id),
        }),
    );

    // Retained effects: NO seam write, NO dial check (era-reissues are free —
    // structurally, not by policy). The old credential's assertion head moves
    // and its commitment (if in the live era) enters the superseded set; the
    // fresh keyed commitment enters the era tree.
    state.used_nonces.insert(nonce);
    let new_id = env.object_id().0;
    let commitment = state.commit(&new_id);
    state.era_leaves.insert(commitment);
    state.commitment_of.insert(new_id, commitment);
    state.assertions.insert(new_id, AssertionHead { superseded_by: None });
    if let Some(head) = state.assertions.get_mut(&old_id.0) {
        head.superseded_by = Some(new_id);
    }
    if let Some(old_commitment) = state.commitment_of.get(&old_id.0) {
        if state.era_leaves.contains(old_commitment) {
            state.era_superseded.insert(*old_commitment);
        }
    }
    state.facts_since_head += 1;
    if state.facts_since_head >= state.cadence_n {
        state.close_epoch(issuer);
    }
    let binding = HolderBinding {
        credential_hash: new_id,
        commitment,
        signature: issuer.sign_bytes(&HolderBinding::signing_bytes(&new_id, &commitment)),
    };
    Ok(ReissueOutput { credential: env, binding })
}

// ---------------------------------------------------------------------------
// Verifier side — pure functions; the verifier never contacts the issuer
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

/// Verify a published tree head: issuer signature over the signed portion,
/// and the published superseded set matching its signed root. Pure.
pub fn verify_tree_head(head: &TreeHead, issuer: &PersonaId) -> bool {
    superseded_digest(&head.superseded) == head.superseded_root
        && verify_detached(
            &TreeHead::signing_bytes(
                head.epoch_no,
                &head.era_anchor,
                head.leaf_count,
                &head.root,
                &head.superseded_root,
            ),
            &head.signature,
            issuer,
        )
}

/// THE verification (V5): `verify(head, proof, credential) -> bool`, a pure
/// function over bytes the HOLDER staples — the verifier never contacts the
/// co-op (T-A4.11, T-A4.12). Checks: the credential verifies from its bytes
/// and the issuer key; the head verifies; the issuer's binding ties this
/// credential to this commitment; the inclusion proof reaches the head's
/// root; and the commitment is not in the head's superseded set (T-A4.10).
pub fn verify_staple(
    credential_bytes: &[u8],
    issuer: &PersonaId,
    head: &TreeHead,
    staple: &Staple,
) -> bool {
    let Ok(env) = verify_credential(credential_bytes, issuer) else { return false };
    verify_tree_head(head, issuer)
        && verify_detached(
            &HolderBinding::signing_bytes(&env.object_id().0, &staple.commitment),
            &staple.binding_sig,
            issuer,
        )
        && verify_inclusion(&head.root, &staple.commitment, &staple.proof)
        && !head.superseded.contains(&staple.commitment)
}

/// A verifier's acceptance decision. Silence is never a verdict: with
/// `SignatureOnly` a missing staple is acceptable BY THAT APP'S CHOICE; with
/// `RequireFreshStaple` the app fails closed without a verifying staple
/// against the head IT considers fresh. Fail-closed is restated here, not
/// re-derived — the T-PA6.4 discipline unchanged under the staple model.
pub fn verifier_accepts(
    credential_bytes: &[u8],
    issuer: &PersonaId,
    stapled: Option<(&TreeHead, &Staple)>,
    dial: CheckDial,
) -> bool {
    if verify_credential(credential_bytes, issuer).is_err() {
        return false;
    }
    match stapled {
        Some((head, staple)) => verify_staple(credential_bytes, issuer, head, staple),
        None => dial == CheckDial::SignatureOnly,
    }
}

/// The covenant audit over the PUBLIC surface (T-A4.13): every head decodes,
/// its signature verifies, its superseded set is well-formed and matches its
/// signed root, epochs are contiguous from 1, leaf counts never regress
/// within an era, and era references are contiguous (an era never reappears
/// after being left) — while resolving ZERO persona identities (the report is
/// totals only: heads audited + total commitments summed per era).
pub fn audit_heads(bytes: &[u8], issuer: &PersonaId) -> Result<AuditReport, AuditFailure> {
    let v: Ipld = serde_ipld_dagcbor::from_slice(bytes).map_err(|_| AuditFailure::Undecodable)?;
    let Ipld::List(records) = v else { return Err(AuditFailure::Undecodable) };

    let mut expected_epoch: u64 = 1;
    let mut heads_audited: u64 = 0;
    let mut total_commitments: u64 = 0;
    let mut seen_eras: Vec<[u8; 32]> = Vec::new();
    let mut era_last_count: u64 = 0;
    for record in records {
        let Ipld::Map(m) = record else { return Err(AuditFailure::Undecodable) };
        let get_b32 = |k: &str| -> Result<[u8; 32], AuditFailure> {
            match m.get(k) {
                Some(Ipld::Bytes(b)) if b.len() == 32 => {
                    Ok(<[u8; 32]>::try_from(b.as_slice()).expect("checked length"))
                }
                _ => Err(AuditFailure::Undecodable),
            }
        };
        let get_u64 = |k: &str| -> Result<u64, AuditFailure> {
            match m.get(k) {
                Some(Ipld::Integer(i)) => u64::try_from(*i).map_err(|_| AuditFailure::Undecodable),
                _ => Err(AuditFailure::Undecodable),
            }
        };
        let epoch_no = get_u64("e")?;
        let era_anchor = get_b32("a")?;
        let leaf_count = get_u64("n")?;
        let root = get_b32("r")?;
        let superseded_root = get_b32("u")?;
        let Some(Ipld::Bytes(sig)) = m.get("g") else { return Err(AuditFailure::Undecodable) };
        let Some(Ipld::List(d)) = m.get("d") else { return Err(AuditFailure::Undecodable) };

        // The signature covers exactly the signed portion.
        if !verify_detached(
            &TreeHead::signing_bytes(epoch_no, &era_anchor, leaf_count, &root, &superseded_root),
            sig,
            issuer,
        ) {
            return Err(AuditFailure::BadSignature { epoch_no });
        }

        // The published superseded set is well-formed (32-byte commitments)
        // and matches the signed root.
        let mut superseded: BTreeSet<[u8; 32]> = BTreeSet::new();
        for c in d {
            let Ipld::Bytes(b) = c else {
                return Err(AuditFailure::MalformedSuperseded { epoch_no });
            };
            let Ok(arr) = <[u8; 32]>::try_from(b.as_slice()) else {
                return Err(AuditFailure::MalformedSuperseded { epoch_no });
            };
            superseded.insert(arr);
        }
        if superseded_digest(&superseded) != superseded_root {
            return Err(AuditFailure::SupersededRootMismatch { epoch_no });
        }

        // Epochs contiguous from 1.
        if epoch_no != expected_epoch {
            return Err(AuditFailure::NonContiguousEpochs);
        }
        expected_epoch += 1;

        // Era references contiguous; per-era leaf counts sum via the last
        // count per era (counts are era-cumulative and never regress).
        match seen_eras.last() {
            Some(current) if *current == era_anchor => {
                if leaf_count < era_last_count {
                    return Err(AuditFailure::LeafCountRegressed { epoch_no });
                }
            }
            _ => {
                if seen_eras.contains(&era_anchor) {
                    return Err(AuditFailure::EraReused);
                }
                total_commitments += era_last_count;
                seen_eras.push(era_anchor);
            }
        }
        era_last_count = leaf_count;
        heads_audited += 1;
    }
    total_commitments += era_last_count;
    Ok(AuditReport { heads_audited, total_commitments })
}
