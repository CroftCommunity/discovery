//! Canonical dag-cbor encoding for the ambassador's records and evidence
//! bodies — the SAME §4.6 dag-cbor path that `attest-family/src/canonical.rs`
//! uses (`serde_ipld_dagcbor` over sorted `Ipld::Map`s with single-character
//! keys so lexicographic and length-first key orders coincide).
//!
//! The path is reused, never re-implemented. The concrete type shapes differ
//! (attest-family encodes edges/vouches/etc; here we encode ambassador
//! receipts and evidence bodies), but the encoder set — `serde_ipld_dagcbor`
//! + `ipld-core` — is the same and the discipline is the same.

use std::collections::BTreeMap;

use ipld_core::ipld::Ipld;

use crate::records::*;
use crate::types::*;

fn map(pairs: Vec<(&str, Ipld)>) -> Ipld {
    let mut m = BTreeMap::new();
    for (k, v) in pairs {
        m.insert(k.to_string(), v);
    }
    Ipld::Map(m)
}

fn bytes(b: &[u8]) -> Ipld {
    Ipld::Bytes(b.to_vec())
}

fn s(v: &str) -> Ipld {
    Ipld::String(v.to_string())
}

fn header_list(headers: &[(String, String)]) -> Ipld {
    Ipld::List(
        headers
            .iter()
            .map(|(k, v)| {
                Ipld::List(vec![s(k), s(v)])
            })
            .collect(),
    )
}

/// Encode an evidence body to canonical dag-cbor. The result is the input
/// to `EvidenceBody::body_hash`.
pub fn encode_evidence_body(body: &EvidenceBody) -> Vec<u8> {
    // single-char keys, sorted by BTreeMap:
    //   b = raw AP JSON bytes
    //   h = headers (ordered list of [name, value] lists)
    //   i = key id (string)
    //   k = actor key SPKI DER bytes
    //   m = HTTP method (string)
    //   p = HTTP path (string)
    let v = map(vec![
        ("b", bytes(&body.raw_body)),
        ("h", header_list(&body.headers)),
        ("i", s(&body.actor_key_id.0)),
        ("k", bytes(&body.actor_key_spki_der)),
        ("m", s(&body.method)),
        ("p", s(&body.path)),
    ]);
    serde_ipld_dagcbor::to_vec(&v).expect("dag-cbor encode of pure map cannot fail")
}

/// Encode a receipt record to canonical dag-cbor. The result is the input to
/// `ReceiptRecord::receipt_id`.
pub fn encode_receipt(r: &ReceiptRecord) -> Vec<u8> {
    // single-char keys, sorted by BTreeMap:
    //   a = actor URL
    //   c = commitment (32 bytes)
    //   d = body hash (32 bytes) — d for "digest of body"
    //   e = attestation marker (static string; legibility only)
    //   i = activity_id
    //   k = kind ("Follow" | "Undo" | "Delete")
    //   o = object URL
    //   s = state ("evidence-complete" | "attested-redacted")
    //   u = undoes (32 bytes) — present only for UndoFollow
    let mut pairs = vec![
        ("a", s(&r.actor.0)),
        ("c", bytes(&r.commitment)),
        ("d", bytes(&r.body_hash)),
        ("e", s(&r.attestation_marker)),
        ("i", s(&r.activity_id)),
        ("k", s(r.kind.as_str())),
        ("o", s(&r.object)),
        ("s", s(r.state.as_str())),
    ];
    if let Some(u) = &r.undoes {
        pairs.push(("u", bytes(&u.0)));
    }
    serde_ipld_dagcbor::to_vec(&map(pairs)).expect("dag-cbor encode of pure map cannot fail")
}
