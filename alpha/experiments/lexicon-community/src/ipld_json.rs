//! JSON ⇄ IPLD with atproto conventions — the lossless bridge.
//!
//! atproto records travel as "IPLD-in-JSON": a CID link is `{"$link": "bafy…"}`
//! and a byte string is `{"$bytes": "<base64>"}`. To recompute a record's CID we
//! must map those back to real `Ipld::Link` / `Ipld::Bytes` before DAG-CBOR
//! encoding — otherwise they serialize as plain maps and the CID diverges.
//!
//! Canonical-order note (proved empirically in EXP-LEX-02): `serde_ipld_dagcbor`
//! serializing an `Ipld::Map` (a `BTreeMap`) already emits DAG-CBOR's
//! length-first-then-bytewise key order, byte-identical to what real PDS records
//! carry. We therefore do NOT re-sort keys here; we only translate node kinds.

use std::collections::BTreeMap;

use base64::Engine;
use ipld_core::cid::Cid;
use ipld_core::ipld::Ipld;

/// Errors from the JSON→IPLD translation (malformed `$link`/`$bytes`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConvError(pub String);

impl std::fmt::Display for ConvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ipld/json: {}", self.0)
    }
}
impl std::error::Error for ConvError {}

fn b64() -> base64::engine::GeneralPurpose {
    // Standard alphabet, padded — the spec's `$bytes` transport. We decode
    // permissively (accept no-pad) but that is handled by decode_permissive.
    base64::engine::general_purpose::STANDARD
}

fn decode_b64_permissive(s: &str) -> Result<Vec<u8>, ConvError> {
    // The spec: "$bytes … standard Base64 … Decoders should accept both padded
    // and unpadded input." Try padded, then no-pad.
    if let Ok(v) = b64().decode(s) {
        return Ok(v);
    }
    base64::engine::general_purpose::STANDARD_NO_PAD
        .decode(s)
        .map_err(|e| ConvError(format!("$bytes base64: {e}")))
}

/// Translate a `serde_json::Value` (IPLD-in-JSON) into an `Ipld` node.
pub fn json_to_ipld(v: &serde_json::Value) -> Result<Ipld, ConvError> {
    use serde_json::Value as J;
    Ok(match v {
        J::Null => Ipld::Null,
        J::Bool(b) => Ipld::Bool(*b),
        J::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ipld::Integer(i as i128)
            } else if let Some(u) = n.as_u64() {
                Ipld::Integer(u as i128)
            } else {
                // DAG-CBOR permits floats; atproto records avoid them, but be faithful.
                Ipld::Float(n.as_f64().ok_or_else(|| ConvError("bad number".into()))?)
            }
        }
        J::String(s) => Ipld::String(s.clone()),
        J::Array(a) => {
            let mut out = Vec::with_capacity(a.len());
            for e in a {
                out.push(json_to_ipld(e)?);
            }
            Ipld::List(out)
        }
        J::Object(o) => {
            // atproto IPLD-in-JSON: a one-key object with $link or $bytes is a leaf.
            if o.len() == 1 {
                if let Some(J::String(link)) = o.get("$link") {
                    let cid = Cid::try_from(link.as_str())
                        .map_err(|e| ConvError(format!("$link cid: {e}")))?;
                    return Ok(Ipld::Link(cid));
                }
                if let Some(J::String(b)) = o.get("$bytes") {
                    return Ok(Ipld::Bytes(decode_b64_permissive(b)?));
                }
            }
            let mut m = BTreeMap::new();
            for (k, val) in o {
                m.insert(k.clone(), json_to_ipld(val)?);
            }
            Ipld::Map(m)
        }
    })
}

/// Translate an `Ipld` node back to `serde_json::Value` (IPLD-in-JSON).
/// The inverse of [`json_to_ipld`]; `$link`/`$bytes` re-emerge as one-key objects.
pub fn ipld_to_json(v: &Ipld) -> serde_json::Value {
    use serde_json::Value as J;
    match v {
        Ipld::Null => J::Null,
        Ipld::Bool(b) => J::Bool(*b),
        Ipld::Integer(i) => J::Number(serde_json::Number::from(*i as i64)),
        Ipld::Float(f) => serde_json::Number::from_f64(*f).map(J::Number).unwrap_or(J::Null),
        Ipld::String(s) => J::String(s.clone()),
        Ipld::Bytes(b) => {
            let mut m = serde_json::Map::new();
            m.insert("$bytes".into(), J::String(b64().encode(b)));
            J::Object(m)
        }
        Ipld::List(l) => J::Array(l.iter().map(ipld_to_json).collect()),
        Ipld::Link(c) => {
            let mut m = serde_json::Map::new();
            m.insert("$link".into(), J::String(c.to_string()));
            J::Object(m)
        }
        Ipld::Map(m) => {
            let mut o = serde_json::Map::new();
            for (k, val) in m {
                o.insert(k.clone(), ipld_to_json(val));
            }
            J::Object(o)
        }
    }
}
