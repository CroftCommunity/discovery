//! The receipt record itself — envelope, evidence body, blinded commitment.
//!
//! AP-V2 (record composition): a receipt carries the full AP activity JSON,
//! the HTTP-signature headers as received, and the actor public key pinned
//! at verification time. The envelope wire form is a canonical dag-cbor map;
//! envelope identity is `H(envelope canonical bytes)` (BLAKE3). Blinded
//! form is posture-conditional: record carries commitment + body_hash;
//! body sits in the store.

use crate::types::*;

/// The evidence body — everything the ambassador holds about a single
/// received activity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceBody {
    pub raw_body: Vec<u8>,
    pub headers: Vec<(String, String)>,
    pub actor_key_spki_der: Vec<u8>,
    pub actor_key_id: KeyId,
    pub method: String,
    pub path: String,
}

impl EvidenceBody {
    pub fn body_hash(&self) -> [u8; 32] {
        unimplemented!("P2 GREEN: BLAKE3 of canonical encoding of the evidence body")
    }
}

/// Blinded-form salt (fixture stand-in this run, AP-V2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Salt(pub [u8; 32]);

/// Commitment: `BLAKE3(salt || body_hash)`.
pub fn commitment(_salt: &Salt, _body_hash: &[u8; 32]) -> [u8; 32] {
    unimplemented!("P2 GREEN: commitment = BLAKE3(salt || body_hash)")
}

/// The receipt record's canonical envelope — MINTED by the ambassador (the
/// observer side), NOT a fact from the actor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReceiptRecord {
    pub kind: ActivityKind,
    pub actor: ActorId,
    pub object: String,
    pub activity_id: String,
    pub undoes: Option<ReceiptId>,
    pub state: ReceiptState,
    pub commitment: [u8; 32],
    pub body_hash: [u8; 32],
    pub attestation_marker: String,
}

impl ReceiptRecord {
    pub fn receipt_id(&self) -> ReceiptId {
        unimplemented!("P2 GREEN: BLAKE3 of canonical encoding of the receipt record")
    }
    pub const GATEWAY_MARKER: &'static str = "ap_ambassador_receipt";
}
