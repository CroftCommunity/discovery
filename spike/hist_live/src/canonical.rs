//! Canonical dag-cbor encoding for atproto records.
//!
//! Two things distinguish atproto's on-the-wire encoding from a naive
//! dag-cbor round-trip of a `serde_json::Value`, and E1's smoke run
//! demonstrated both live:
//!
//!   1. **Map-key ordering.** IPLD DAG-CBOR
//!      (https://ipld.io/specs/codecs/dag-cbor/) canonicalises map keys as
//!      (length, then bytewise-lex) — verified live in E1 against a
//!      bsky-hosted PDS's stored bytes.  A downstream user who is tempted to
//!      switch to `serde_ipld_dagcbor` should first spot-check: this crate
//!      (as of 0.6) emitted the same length-first ordering the wire expects.
//!      The problem this module fixes is orthogonal — it is the extended-JSON
//!      translation below.
//!   2. **`$bytes` / `$link` conventions.** When a JSON value uses atproto's
//!      "extended JSON" markers `{"$bytes": "<b64>"}` or `{"$link": "<cid>"}`,
//!      the atproto encoder translates them into native CBOR bytes / CID-link
//!      values.  A naive `serde_ipld_dagcbor::to_vec` over a `serde_json::Value`
//!      emits them as literal maps.
//!
//! This module implements the atproto data model directly:
//!  - `json_to_ipld`  translates the extended-JSON form to a canonical
//!                    `ipld_core::ipld::Ipld` tree
//!  - `encode_ipld`   emits canonical dag-cbor with bytewise-lex map ordering
//!  - `canonical_dag_cbor(value)` is the public entrypoint: takes any
//!    `serde::Serialize`, goes via `serde_json::Value`, applies both rules,
//!    and returns bytes.
//!
//! Verified live in E1: bytes and CID match the PDS's stored form exactly.

use cid::Cid;
use ipld_core::ipld::Ipld;
use multihash::Multihash;
use sha2::{Digest, Sha256};

pub const CODEC_DAG_CBOR: u64 = 0x71;
pub const CODE_SHA2_256: u64 = 0x12;

/// Canonicalize a serde-serializable value to atproto-canonical dag-cbor.
pub fn canonical_dag_cbor<T: serde::Serialize>(value: &T) -> Vec<u8> {
    let json = serde_json::to_value(value).expect("value → json");
    let ipld = json_to_ipld(&json);
    let mut buf = Vec::new();
    encode_ipld(&ipld, &mut buf);
    buf
}

pub fn cid_v1_dag_cbor(bytes: &[u8]) -> Cid {
    let digest = Sha256::digest(bytes);
    let mh = Multihash::<64>::wrap(CODE_SHA2_256, digest.as_ref()).expect("mh wrap");
    Cid::new_v1(CODEC_DAG_CBOR, mh)
}

pub type CidString = String;

pub fn cid_to_string(c: &Cid) -> CidString {
    c.to_string()
}

/// Inverse of `json_to_ipld`: translate Ipld back into atproto extended-JSON.
///  - `Ipld::Bytes(b)` → `{"$bytes": "<base64>"}`  (standard, padded)
///  - `Ipld::Link(cid)` → `{"$link": "<cid.to_string()>"}`
pub fn ipld_to_json(ipld: &Ipld) -> serde_json::Value {
    use serde_json::Value as J;
    match ipld {
        Ipld::Null => J::Null,
        Ipld::Bool(b) => J::Bool(*b),
        Ipld::Integer(i) => J::Number(serde_json::Number::from_i128(*i).unwrap_or_else(|| {
            serde_json::Number::from(0)
        })),
        Ipld::Float(f) => serde_json::Number::from_f64(*f)
            .map(J::Number)
            .unwrap_or(J::Null),
        Ipld::String(s) => J::String(s.clone()),
        Ipld::Bytes(b) => {
            use base64::Engine as _;
            let mut m = serde_json::Map::new();
            m.insert(
                "$bytes".into(),
                J::String(base64::engine::general_purpose::STANDARD.encode(b)),
            );
            J::Object(m)
        }
        Ipld::List(l) => J::Array(l.iter().map(ipld_to_json).collect()),
        Ipld::Map(m) => {
            let mut out = serde_json::Map::new();
            for (k, v) in m {
                out.insert(k.clone(), ipld_to_json(v));
            }
            J::Object(out)
        }
        Ipld::Link(cid) => {
            let mut m = serde_json::Map::new();
            m.insert("$link".into(), J::String(cid.to_string()));
            J::Object(m)
        }
    }
}

/// Decode a dag-cbor block into atproto extended JSON: first as `Ipld`, then
/// mapped through `ipld_to_json`.  Correct for records with CID links and
/// raw bytes.
pub fn dag_cbor_to_atproto_json(bytes: &[u8]) -> Result<serde_json::Value, String> {
    let ipld: Ipld = serde_ipld_dagcbor::from_slice(bytes).map_err(|e| e.to_string())?;
    Ok(ipld_to_json(&ipld))
}

/// Translate JSON to canonical Ipld, honoring atproto's extended forms:
///   - `{"$bytes": "<base64>"}` → `Ipld::Bytes(bytes)`
///   - `{"$link":  "<cid>"}`    → `Ipld::Link(cid)`
///   - `null`                   → `Ipld::Null`
///   - numbers: exact integers → `Ipld::Integer`, otherwise `Ipld::Float`
pub fn json_to_ipld(v: &serde_json::Value) -> Ipld {
    use serde_json::Value as J;
    match v {
        J::Null => Ipld::Null,
        J::Bool(b) => Ipld::Bool(*b),
        J::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ipld::Integer(i as i128)
            } else if let Some(u) = n.as_u64() {
                Ipld::Integer(u as i128)
            } else {
                Ipld::Float(n.as_f64().unwrap_or(0.0))
            }
        }
        J::String(s) => Ipld::String(s.clone()),
        J::Array(a) => Ipld::List(a.iter().map(json_to_ipld).collect()),
        J::Object(m) => {
            // Atproto extension: single-key objects with $bytes / $link.
            if m.len() == 1 {
                if let Some(J::String(b64)) = m.get("$bytes") {
                    let bytes = decode_atproto_base64(b64).unwrap_or_default();
                    return Ipld::Bytes(bytes);
                }
                if let Some(J::String(cs)) = m.get("$link") {
                    if let Ok(c) = cs.parse::<Cid>() {
                        return Ipld::Link(c);
                    }
                    // Fall through; if link fails to parse, keep as map.
                }
            }
            let mut out = std::collections::BTreeMap::new();
            for (k, v) in m {
                out.insert(k.clone(), json_to_ipld(v));
            }
            Ipld::Map(out)
        }
    }
}

/// The atproto `$bytes` marker uses standard base64 (with or without
/// padding).  Support both.
fn decode_atproto_base64(s: &str) -> Result<Vec<u8>, base64::DecodeError> {
    use base64::Engine as _;
    let cleaned: String = s.chars().filter(|c| !c.is_whitespace()).collect();
    // Try padded first, then unpadded.
    if let Ok(b) = base64::engine::general_purpose::STANDARD.decode(&cleaned) {
        return Ok(b);
    }
    base64::engine::general_purpose::STANDARD_NO_PAD.decode(&cleaned)
}

// ---- Canonical dag-cbor encoder ----

pub fn encode_ipld(v: &Ipld, out: &mut Vec<u8>) {
    match v {
        Ipld::Null => out.push(0xf6),
        Ipld::Bool(false) => out.push(0xf4),
        Ipld::Bool(true) => out.push(0xf5),
        Ipld::Integer(i) => encode_int(*i, out),
        Ipld::Float(f) => encode_f64(*f, out),
        Ipld::String(s) => encode_head(3, s.len() as u64, out).and_then(|_| {
            out.extend_from_slice(s.as_bytes());
            Some(())
        })
        .unwrap_or(()),
        Ipld::Bytes(b) => {
            encode_head(2, b.len() as u64, out);
            out.extend_from_slice(b);
        }
        Ipld::List(l) => {
            encode_head(4, l.len() as u64, out);
            for item in l {
                encode_ipld(item, out);
            }
        }
        Ipld::Map(m) => {
            // Sort keys by (length, then bytewise-lex).  This is the DAG-CBOR
            // canonical form (https://ipld.io/specs/codecs/dag-cbor/) that
            // atproto uses on the wire — verified live in E1 by byte-for-byte
            // equality against a bsky-hosted PDS's stored CAR block.
            let mut keys: Vec<&String> = m.keys().collect();
            keys.sort_by(|a, b| {
                a.len()
                    .cmp(&b.len())
                    .then_with(|| a.as_bytes().cmp(b.as_bytes()))
            });
            encode_head(5, keys.len() as u64, out);
            for k in keys {
                // Encode the key as a text string.
                encode_head(3, k.len() as u64, out);
                out.extend_from_slice(k.as_bytes());
                encode_ipld(m.get(k).unwrap(), out);
            }
        }
        Ipld::Link(cid) => {
            // CBOR tag 42 = CID link.  Content is a byte string of the
            // multibase-identity-prefixed CID bytes (leading 0x00 + cid.to_bytes()).
            encode_tag(42, out);
            let mut cid_bytes = Vec::with_capacity(cid.to_bytes().len() + 1);
            cid_bytes.push(0x00);
            cid_bytes.extend_from_slice(&cid.to_bytes());
            encode_head(2, cid_bytes.len() as u64, out);
            out.extend_from_slice(&cid_bytes);
        }
    }
}

fn encode_head(major: u8, len: u64, out: &mut Vec<u8>) -> Option<()> {
    // Emit the shortest CBOR head for `len` under the given major type (0..7).
    let m = major << 5;
    if len < 24 {
        out.push(m | (len as u8));
    } else if len <= 0xff {
        out.push(m | 24);
        out.push(len as u8);
    } else if len <= 0xffff {
        out.push(m | 25);
        out.extend_from_slice(&(len as u16).to_be_bytes());
    } else if len <= 0xffff_ffff {
        out.push(m | 26);
        out.extend_from_slice(&(len as u32).to_be_bytes());
    } else {
        out.push(m | 27);
        out.extend_from_slice(&len.to_be_bytes());
    }
    Some(())
}

fn encode_int(i: i128, out: &mut Vec<u8>) {
    if i >= 0 {
        encode_head(0, i as u64, out);
    } else {
        // Negative integer: encoded as -1 - n, with major type 1.
        let n = (-1 - i) as u64;
        encode_head(1, n, out);
    }
}

fn encode_f64(f: f64, out: &mut Vec<u8>) {
    // Atproto dag-cbor forbids NaN / infinities.  For finite floats we emit
    // f64 exactly (no shrinkage).  This spike doesn't put floats in its
    // envelopes so this branch is exercised only when the record author
    // (mistakenly) supplies one.
    out.push(0xfb); // major 7, additional 27 = 8-byte float
    out.extend_from_slice(&f.to_be_bytes());
}

fn encode_tag(tag: u64, out: &mut Vec<u8>) {
    encode_head(6, tag, out);
}
