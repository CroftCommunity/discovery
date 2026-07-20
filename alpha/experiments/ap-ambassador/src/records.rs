//! The receipt record itself — envelope, evidence body, blinded commitment.
//!
//! AP-V2 (record composition): a receipt carries
//!   - the full AP activity JSON as received,
//!   - the HTTP-signature headers as received,
//!   - the actor public key pinned at verification time.
//!
//! The envelope wire form is a canonical dag-cbor map (§A.5 / §4.6 path from
//! attest-family, single-character keys so both key-order conventions
//! coincide). Envelope identity is `H(envelope canonical bytes)` (BLAKE3).
//!
//! **Blinded form** (posture-conditional): the record can carry only
//! `commitment` + `H(evidence body)`; the body itself sits in the ambassador
//! `store`, produced at the tier the roster is visible at. A blinded record
//! re-verifies once the body is produced from the store (P2 T-AP2.5), and
//! the body alone deanonymizes nothing without the salt (P2 T-AP2.6).

use crate::types::*;

/// The evidence body — everything the ambassador holds about a single
/// received activity. The `raw_body` is the exact bytes the sender's HTTP
/// signature covered (a re-verification pin — you must re-verify against
/// exactly what was signed, not a re-serialized copy).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceBody {
    /// Raw AP activity JSON, as received.
    pub raw_body: Vec<u8>,
    /// HTTP-signature-relevant headers as received, in arrival order:
    /// `(request-target)`, `host`, `date`, `digest`, `signature`, plus any
    /// others the signature covered. Stored as name/value pairs (names
    /// lower-cased at receipt time; the raw arrival forms nothing new that
    /// the signature covered because the signature signs the canonical
    /// signing string).
    pub headers: Vec<(String, String)>,
    /// The actor public key pinned at verification time — SPKI DER bytes.
    /// Storing the bytes rather than the parsed form is deliberate: the
    /// evidence is byte-faithful to what verify accepted.
    pub actor_key_spki_der: Vec<u8>,
    /// The keyId string that selected `actor_key_spki_der` at verify time.
    pub actor_key_id: KeyId,
    /// Which HTTP verb + path the signed request used (so the receipt is
    /// self-contained for re-verification).
    pub method: String,
    pub path: String,
}

impl EvidenceBody {
    /// BLAKE3 of the canonical dag-cbor encoding of the body — the value that
    /// the record's `body_hash` field pins.
    pub fn body_hash(&self) -> [u8; 32] {
        *blake3::hash(&crate::canonical::encode_evidence_body(self)).as_bytes()
    }
}

/// The blinded-form salt — a fixture 32 bytes this run (AP-V2 declared
/// stand-in). Real deployments would derive per-record.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Salt(pub [u8; 32]);

/// The commitment: `BLAKE3(salt || body_hash)` — publishable at the blinded
/// tier without revealing the body. Same value re-computes from the body
/// once produced; the salt keeps a body-alone attacker from confirming a
/// specific record by re-hashing the body.
pub fn commitment(salt: &Salt, body_hash: &[u8; 32]) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();
    hasher.update(&salt.0);
    hasher.update(body_hash);
    *hasher.finalize().as_bytes()
}

/// The receipt record's canonical envelope. This is the value the
/// ambassador MINTs (signed by the ambassador's own key — the observer
/// side); it is NOT a fact from the actor. The ambassador role is the
/// author.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReceiptRecord {
    /// The activity kind this receipt attests reception of.
    pub kind: ActivityKind,
    /// The AP actor URL that authored the activity.
    pub actor: ActorId,
    /// For Follow/UndoFollow: the followed object (target).
    /// For Delete: the deleted actor/object URL.
    pub object: String,
    /// The AP activity's own `id` field.
    pub activity_id: String,
    /// For UndoFollow: the receipt this Undo closes (`Some(receipt_id_of_the_Follow)`);
    /// for Follow / Delete: `None`.
    pub undoes: Option<ReceiptId>,
    /// State marker — evidence-complete or attested-redacted (AP-V2/V3).
    pub state: ReceiptState,
    /// Commitment (blinded posture) — `BLAKE3(salt || body_hash)`.
    pub commitment: [u8; 32],
    /// The body hash — pinned in-envelope so a blinded record still names
    /// which body it commits to when the body is later produced.
    pub body_hash: [u8; 32],
    /// The ambassador's declared "gateway attestation" tag — a static
    /// marker in the envelope so a fold consumer can see "this is
    /// gateway-attested, not two-sided" without decoding further (P5
    /// legibility). The value is the constant `"ap_ambassador_receipt"`;
    /// this is NOT an `AntecedentKind` (P5 permanent-red).
    pub attestation_marker: String,
}

impl ReceiptRecord {
    /// BLAKE3 of the canonical dag-cbor bytes of the record — the
    /// `ReceiptId`.
    pub fn receipt_id(&self) -> ReceiptId {
        ReceiptId(*blake3::hash(&crate::canonical::encode_receipt(self)).as_bytes())
    }

    /// The static gateway-attestation marker. Constant across all receipts;
    /// used by P5 as legibility, never by the R7 fold as classification.
    pub const GATEWAY_MARKER: &'static str = "ap_ambassador_receipt";
}
