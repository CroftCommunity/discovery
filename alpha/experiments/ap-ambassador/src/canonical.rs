//! Canonical dag-cbor encoding for the ambassador's records and evidence
//! bodies — the SAME §4.6 dag-cbor path that `attest-family/src/canonical.rs`
//! uses (`serde_ipld_dagcbor` over sorted `Ipld::Map`s with single-character
//! keys so lexicographic and length-first key orders coincide).
//!
//! The path is reused, never re-implemented.

use crate::records::*;

/// Encode an evidence body to canonical dag-cbor. The result is the input
/// to `EvidenceBody::body_hash`.
pub fn encode_evidence_body(_body: &EvidenceBody) -> Vec<u8> {
    unimplemented!("P2 GREEN: encode_evidence_body — reuse attest-family §4.6 dag-cbor path")
}

/// Encode a receipt record to canonical dag-cbor. The result is the input to
/// `ReceiptRecord::receipt_id`.
pub fn encode_receipt(_r: &ReceiptRecord) -> Vec<u8> {
    unimplemented!("P2 GREEN: encode_receipt — reuse attest-family §4.6 dag-cbor path")
}
