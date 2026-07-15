//! Language-neutral vector schemas.
//!
//! These are the on-disk JSON shapes. Byte arrays are hex-encoded strings so
//! the vectors are portable to a non-Rust implementation. Every file carries the
//! spec section it exercises and an `expect` discriminant (`accept` /
//! `reject:<reason>`) per the conformance-suite layout.
//!
//! The structs are *only* a serialization boundary — they hold no logic. The
//! emitter fills them from the real API; the runner reads them back and re-feeds
//! the inputs through the same API.

use serde::{Deserialize, Serialize};

/// Common header on every vector file: the spec section it exercises.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Header {
    /// Spec section, e.g. "CROFT-PROTOCOL.md §2".
    pub spec_section: String,
    /// Conformance-suite category number (1..=9).
    pub category: u32,
    /// Human note on what the file proves.
    pub note: String,
}

// --- Category 1: derivations -------------------------------------------------

/// How a derivation's expected output is reproduced from its input.
///
/// * `Structural` — `sha256(input_bytes)` (the untagged `GenesisId::from_bytes`
///   content-hash anchor). `input_hex` holds the raw canonical bytes.
/// * `LineageGenesis` / `GroupGenesis` / `GroupTopic` — the CROFT-PROTOCOL §2
///   *tagged* wire-identity derivations: `sha256(tag ‖ id_string)`. Here the
///   `input_id` is the UTF-8 id string the real `lineage_core::ids::*` function
///   takes; the runner re-derives by calling that exact function (it does NOT
///   re-hash `input_hex`), so the tag bytes are proven by the real code.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DerivationKind {
    /// `sha256(input bytes)` — the structural content-hash anchor.
    Structural,
    /// `lineage_core::ids::lineage_genesis(input_id)` — `sha256("croft-lineage-genesis:" ‖ id)`.
    LineageGenesis,
    /// `lineage_core::ids::group_genesis(input_id)` — `sha256("croft-group-genesis:" ‖ id)`.
    GroupGenesis,
    /// `lineage_core::ids::group_topic(input_id)` — `sha256("croft-group-topic:" ‖ id)`.
    GroupTopic,
}

/// One derivation vector: a labelled input pre-image → its 32-byte hash output.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DerivationVector {
    /// What this derivation is (human label).
    pub kind: String,
    /// Which real derivation function reproduces `expected_hex`. The runner
    /// dispatches on this so it re-derives through the canonical API rather than
    /// re-hashing the recorded pre-image.
    pub derivation: DerivationKind,
    /// For `Structural`: the exact `canonical_bytes` pre-image (hex). For the
    /// tagged kinds: the hash pre-image `tag ‖ id` (hex), recorded for portability
    /// so a foreign impl can reproduce it without knowing the tag separately.
    pub input_hex: String,
    /// For tagged kinds only: the UTF-8 id string the real function is called
    /// with (e.g. "lin-a", "grp-1"). Empty for `Structural`.
    #[serde(default)]
    pub input_id: String,
    /// The expected 32-byte output (hex).
    pub expected_hex: String,
    /// `accept` — the derivation must reproduce `expected_hex`.
    #[serde(default = "accept_default")]
    pub expect: String,
}

fn accept_default() -> String {
    "accept".into()
}

/// The cat-1 file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DerivationsFile {
    /// Spec-section header.
    pub header: Header,
    /// The derivation vectors.
    pub vectors: Vec<DerivationVector>,
}

// --- Category 2: signed pre-images ------------------------------------------

/// A signing vector: (branch, seq, author, payload) → signing_bytes + sig + key.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SigningVector {
    /// 32-byte branch genesis id (hex).
    pub branch_hex: String,
    /// Position of the message on the branch.
    pub seq: u64,
    /// Author DID (UTF-8 string).
    pub author: String,
    /// The seed the real `SigningIdentity::from_seed` used. Recorded so the
    /// runner can rebuild the *real* key via the public API and verify through
    /// `VerifyingIdentity::verify`. A foreign implementation ignores this and
    /// verifies the recorded `signature_hex` against `verifying_key_hex` with
    /// its own Ed25519 — both must agree.
    pub author_seed: u64,
    /// Payload bytes (hex).
    pub payload_hex: String,
    /// The expected canonical signing pre-image (`"msg-v1" ‖ …`) (hex).
    pub signing_bytes_hex: String,
    /// The author's Ed25519 verifying key (32 bytes, hex).
    pub verifying_key_hex: String,
    /// A known-good detached signature over `signing_bytes` (64 bytes, hex).
    pub signature_hex: String,
    /// `accept` for the good vector; `reject:bad-signature` for a tampered one.
    pub expect: String,
}

/// The cat-2 file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SigningFile {
    /// Spec-section header.
    pub header: Header,
    /// The good vector (must verify).
    pub good: SigningVector,
    /// A one-bit-flipped variant of the good signature (must reject).
    pub tampered: SigningVector,
}

// --- Categories 3+4: fold-by-lineage / thresholds count lineages ------------

/// One admin signer in a fold vector: a device DID and the lineage it belongs to.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FoldSigner {
    /// The device/admin DID.
    pub did: String,
    /// The lineage this device folds into (the actor).
    pub lineage: String,
}

/// A fold/threshold vector. A set of admin signers (each a device under some
/// lineage) signs one Remove op; the expected counts are by-DID vs by-lineage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FoldVector {
    /// What the vector demonstrates.
    pub label: String,
    /// The signers on the op (device DIDs + their lineage).
    pub signers: Vec<FoldSigner>,
    /// The op kind being authorized ("Remove" or "Add").
    pub op_kind: String,
    /// The genesis threshold for that kind.
    pub threshold: u32,
    /// Expected count of distinct DIDs that validly signed (the *unsafe* count).
    pub expected_by_did: u32,
    /// Expected count of distinct lineages (the *safe* count, E2.10).
    pub expected_by_lineage: u32,
    /// `accept` if the lineage-counted quorum meets threshold; otherwise
    /// `reject:under-threshold-by-lineage`.
    pub expect: String,
}

/// The cat-3+4 file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FoldFile {
    /// Spec-section header.
    pub header: Header,
    /// The fold/threshold vectors.
    pub vectors: Vec<FoldVector>,
}

// --- Category 5: revocation mechanics ---------------------------------------

/// A single message in a donor branch (for the backfill standing checks).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistMsg {
    /// The message author's DID.
    pub author: String,
    /// Position on the branch.
    pub seq: u64,
    /// Payload bytes (hex).
    pub payload_hex: String,
}

/// A revocation-mechanics vector. A branch's history is offered for backfill;
/// after the author is revoked (loses standing) the post-revoke branch MUST be
/// rejected, while pre-revoke history MUST remain importable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RevocationVector {
    /// What the vector demonstrates.
    pub label: String,
    /// The branch genesis (hex).
    pub branch_hex: String,
    /// Whether the author has standing on this branch at import time.
    pub author_has_standing: bool,
    /// The messages on the donor branch.
    pub messages: Vec<HistMsg>,
    /// `accept` if backfill should succeed; `reject:unauthorized-author` if the
    /// author lacks standing (post-revocation).
    pub expect: String,
}

/// The cat-5 file. Carries a marker recording that the threshold-AUTHORITY
/// sub-case is deferred (design row, blocked on Workstream C).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RevocationFile {
    /// Spec-section header.
    pub header: Header,
    /// The revocation-mechanics vectors.
    pub vectors: Vec<RevocationVector>,
    /// Why the revoke-authority threshold vectors are not present.
    pub deferred_authority_note: String,
}

// --- Category 5b: revoke-AUTHORITY threshold (real signature + lineage count) -

/// One admin in a revoke-authority vector: the admin DID, its lineage, and the
/// raw 32-byte Ed25519 verifying key (hex). The verifying key is public material
/// a foreign implementation uses to verify the recorded `(did, sig)` pairs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorityAdmin {
    /// The admin DID (a member of the genesis admin set).
    pub did: String,
    /// The lineage this admin's device folds into (the actor).
    pub lineage: String,
    /// The admin's Ed25519 verifying key (32 bytes, hex).
    pub verifying_key_hex: String,
}

/// One `(did, signature)` pair gathered on the revoke op (signature 64 bytes, hex).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthoritySig {
    /// The signer DID.
    pub did: String,
    /// The signer's detached Ed25519 signature over `signing_bytes` (hex).
    pub signature_hex: String,
}

/// A revoke-authority vector. A Remove (revoke) `SignedOp` is presented along
/// with the admin set, the lineage map, and the genesis threshold. A conforming
/// implementation re-runs the real `meets_threshold_by_lineage` (verify each
/// signature against the admin's key, count distinct admin LINEAGES, compare to
/// threshold) and the verdict must match `expect`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorityVector {
    /// What the vector demonstrates.
    pub label: String,
    /// The genesis admin set + their verifying keys + lineages.
    pub admins: Vec<AuthorityAdmin>,
    /// Extra non-admin signers' verifying keys (for the non-admin-signer case).
    /// Each is a member who may sign but is NOT in the admin set.
    pub non_admin_signers: Vec<AuthorityAdmin>,
    /// The op kind being authorized (always "Remove" here).
    pub op_kind: String,
    /// The DID being revoked (the op subject).
    pub subject: String,
    /// The exact canonical op signing pre-image (`"op-v1" ‖ …`) (hex).
    pub signing_bytes_hex: String,
    /// The `(did, sig)` pairs gathered on the op.
    pub sigs: Vec<AuthoritySig>,
    /// The signer DID -> lineage DID map (the E2.10 lineage-counted input).
    pub lineage_of: Vec<(String, String)>,
    /// The genesis threshold for the op kind.
    pub threshold: u32,
    /// `accept` if the lineage-counted admin quorum meets threshold; otherwise
    /// `reject:<reason>` (under_threshold | non_admin_signer | one_lineage_multi_device).
    pub expect: String,
}

/// The cat-5b file. Carries the real revoke-authority signature + lineage-counted
/// threshold vectors (green-real `gov::meets_threshold_by_lineage`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorityFile {
    /// Spec-section header.
    pub header: Header,
    /// The revoke-authority vectors.
    pub vectors: Vec<AuthorityVector>,
}

// --- Category 6: reconcile corpus C1..C10 -----------------------------------

/// A membership op on a branch (for building reconcile inputs).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReconcileOp {
    /// "Add" | "Remove" | "Dissolve".
    pub kind: String,
    /// The subject DID (None for Dissolve, encoded as empty string).
    pub subject: String,
    /// The signer DIDs.
    pub signers: Vec<String>,
}

/// One side of a reconcile (one partitioned branch's op log).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReconcileBranch {
    /// A human label for the branch.
    pub label: String,
    /// The signed membership ops this branch applied while partitioned.
    pub ops: Vec<ReconcileOp>,
}

/// A reconcile vector: two (or more) partitioned branches → a verdict.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReconcileVector {
    /// Corpus id, e.g. "C1".
    pub id: String,
    /// What the scenario demonstrates.
    pub label: String,
    /// The partitioned branches that reconnect.
    pub branches: Vec<ReconcileBranch>,
    /// Expected verdict: "converge" | "hard-stop" | "re-formation".
    pub verdict: String,
    /// For hard-stop: the contested member DIDs (sorted). Empty otherwise.
    pub contested: Vec<String>,
}

/// A cat-6 corpus file (one per Cn).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReconcileFile {
    /// Spec-section header.
    pub header: Header,
    /// The single reconcile vector this file carries.
    pub vector: ReconcileVector,
}

// --- Category 7: adversarial AR-1..AR-6 -------------------------------------

/// The kind of adversarial check a cat-7 vector exercises. Each variant maps to
/// a real `lineage-core` / `lineage-history` API the runner re-runs; the verdict
/// is never hand-set. The string carried in `expect` is the human-facing
/// accept/reject/bound discriminant; this enum tells the runner *how* to derive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdversarialKind {
    /// AR-1 Sybil: fresh non-admin identities sign a Remove. The op MUST be
    /// rejected (`SignerLacksStanding`) and the by-lineage admin count MUST be 0.
    SybilNoStanding,
    /// AR-2 malicious sequencer (reorder + replay): a shuffled+duplicated op
    /// stream MUST converge to the same head/members as the in-order chain.
    SequencerReorderConverges,
    /// AR-2 malicious sequencer (drop): dropping op at `drop_seq` leaves the peer
    /// at a real earlier head — visibly behind, never a false current.
    SequencerDropVisiblyBehind,
    /// AR-2 malicious sequencer (inject): an injected op signed by a non-admin
    /// MUST be rejected (`SignerLacksStanding`).
    SequencerInjectRejected,
    /// AR-6 double-count: one DID signing twice still counts once → below
    /// threshold. Sigs are keyed by DID, so the bundle MUST NOT meet threshold.
    ReplayDoubleCountPrevented,
    /// AR-6 replay: an already-applied op replayed against the advanced head
    /// MUST be rejected (`BrokenChain`) — it does not re-enact.
    ReplayDoesNotReenact,
    /// AR-3 backfill DoS: a foreign-lineage branch MUST be rejected on the
    /// genesis boundary BEFORE any signature verification (zero-crypto reject).
    BackfillForeignZeroCrypto,
    /// AR-3 backfill DoS: a forged branch on a shared lineage MUST be rejected at
    /// the FIRST bad message; rejection cost is bounded by the first defect.
    BackfillFirstDefectBounded,
}

/// An adversarial vector (cat 7). Holds a scenario the runner re-runs against the
/// real API to derive a must-reject / must-converge / must-bound verdict.
///
/// The fields are a superset across the AR kinds; only those relevant to `kind`
/// are populated (the runner reads exactly the ones it needs and ignores the
/// rest). This keeps one portable JSON shape for the whole adversarial category.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdversarialVector {
    /// The AR id this vector belongs to, e.g. "AR-1".
    pub ar: String,
    /// What the vector demonstrates.
    pub label: String,
    /// Which real-API check the runner runs.
    pub kind: AdversarialKind,
    /// Admin DIDs in the genesis (governance authority). For sequencer cases the
    /// sole admin authors the canonical chain.
    #[serde(default)]
    pub admins: Vec<String>,
    /// Founding members of the genesis (admins are a subset).
    #[serde(default)]
    pub founders: Vec<String>,
    /// The op kind under test ("Add" | "Remove").
    #[serde(default)]
    pub op_kind: String,
    /// The op subject DID (the member added/removed), if any.
    #[serde(default)]
    pub subject: String,
    /// Signer DIDs on the op (for Sybil / double-count / inject cases).
    #[serde(default)]
    pub signers: Vec<String>,
    /// The genesis threshold for `op_kind`.
    #[serde(default)]
    pub threshold: u32,
    /// For sequencer cases: how many sequential Adds the canonical chain has.
    #[serde(default)]
    pub chain_len: u32,
    /// For the drop case: the seq the malicious sequencer withholds.
    #[serde(default)]
    pub drop_seq: u32,
    /// For backfill cases: how many messages the hostile donor branch carries.
    #[serde(default)]
    pub donor_msgs: u32,
    /// The human-facing verdict discriminant. `accept`/`converge` for the bound
    /// cases that must hold; `reject:<reason>` for must-reject cases.
    pub expect: String,
}

/// The cat-7 file: the adversarial corpus the Rust suite can derive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdversarialFile {
    /// Spec-section header.
    pub header: Header,
    /// The derivable adversarial vectors (AR-1, AR-2, AR-3, AR-6).
    pub vectors: Vec<AdversarialVector>,
    /// AR cases NOT emitted as Rust crypto vectors, with the reason each is a
    /// characterization/bound the conformance crate cannot derive deterministically.
    pub not_emitted: Vec<String>,
}

// --- Categories 8 + 9: TS-authoritative visibility / freshness ---------------

/// One invariant verdict carried over from the authoritative TS model (an
/// `ExperimentResult.invariants[]` entry). The Rust runner validates STRUCTURE
/// only — it does not re-prove the TS logic.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TsInvariant {
    /// The INV-* invariant name.
    pub invariant: String,
    /// Whether the TS model reported it passing.
    pub passed: bool,
}

/// One TS experiment's verdict (e.g. "V1", "S2a", "E2.16b"), language-neutral.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TsExperiment {
    /// The experiment name as printed by the TS model.
    pub name: String,
    /// The invariants the experiment asserts and their pass/fail.
    pub invariants: Vec<TsInvariant>,
    /// True iff every invariant passed (the experiment's overall verdict).
    pub all_passed: bool,
}

/// A cat-8 (visibility V1..V9 + S2) or cat-9 (freshness E2.16) file. The
/// AUTHORITATIVE runner for these categories is the TS model in
/// `Proofs/lineage-group-model`; this file is a language-neutral snapshot of its
/// verdicts. The Rust runner only validates structure + manifest hash.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TsVerdictFile {
    /// Spec-section header.
    pub header: Header,
    /// Which runner is authoritative for these vectors (always the TS model).
    pub authoritative_runner: String,
    /// How this snapshot was produced (the exact TS command).
    pub provenance: String,
    /// The experiment verdicts captured from the TS model.
    pub experiments: Vec<TsExperiment>,
}

// --- MANIFEST ----------------------------------------------------------------

/// One entry in the manifest: a vector file and its content hash.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestEntry {
    /// Path relative to the `conformance/` root.
    pub file: String,
    /// SHA-256 of the file's exact bytes (hex).
    pub sha256_hex: String,
}

/// The suite manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Manifest {
    /// Conformance-suite version.
    pub suite_version: String,
    /// The spec version/date this suite targets.
    pub targets_spec: String,
    /// Hashes of every emitted vector file.
    pub entries: Vec<ManifestEntry>,
    /// Categories present in this emission.
    pub categories_present: Vec<String>,
    /// Categories deliberately not emitted in this pass (with reasons).
    pub not_yet_emitted: Vec<String>,
}
