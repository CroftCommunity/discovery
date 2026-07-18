//! Canonical (deterministic) CBOR encoding.
//!
//! The model signs and hashes over bytes, so those bytes must be reproducible:
//! two independent encodes of the same value are byte-identical, and encode →
//! decode → re-encode is a fixpoint. We achieve this by serialising to a
//! [`ciborium::Value`], recursively sorting every map's entries by the RFC 8949
//! §4.2.1 deterministic rule (encoded-key **length first, then bytewise**), and
//! serialising the sorted tree.
//!
//! This is genuine deterministic CBOR key ordering — not the declaration-order
//! default ciborium emits — so the "dag-cbor canonical" claim holds at the byte
//! level. (Full IPLD DAG-CBOR additionally forbids indefinite-length items and
//! pins float encoding; our records use none of those, so the two agree here.)

use ciborium::value::Value;
use serde::Serialize;

/// Why canonical encoding failed.
#[derive(Debug, thiserror::Error)]
pub enum CanonError {
    /// The value could not be represented as a `ciborium::Value`.
    #[error("serialize to value: {0}")]
    ToValue(String),
    /// The canonical tree could not be written to bytes.
    #[error("write canonical bytes: {0}")]
    Write(String),
}

/// Encode `value` to canonical CBOR bytes.
///
/// # Errors
/// Returns [`CanonError`] if `value` cannot be represented as CBOR or the
/// canonical tree cannot be serialised.
pub fn to_canonical<T: Serialize>(value: &T) -> Result<Vec<u8>, CanonError> {
    let v = Value::serialized(value).map_err(|e| CanonError::ToValue(e.to_string()))?;
    let v = canonicalize(v)?;
    let mut out = Vec::new();
    ciborium::into_writer(&v, &mut out).map_err(|e| CanonError::Write(e.to_string()))?;
    Ok(out)
}

/// Recursively rewrite `v` so every map is in RFC 8949 §4.2.1 deterministic key
/// order. Arrays keep their order (order is semantic); scalars are unchanged.
fn canonicalize(v: Value) -> Result<Value, CanonError> {
    match v {
        Value::Map(entries) => {
            // Canonicalise children first, then sort by the encoded key bytes.
            let mut keyed: Vec<(Vec<u8>, Value, Value)> = Vec::with_capacity(entries.len());
            for (k, val) in entries {
                let k = canonicalize(k)?;
                let val = canonicalize(val)?;
                let mut kb = Vec::new();
                ciborium::into_writer(&k, &mut kb)
                    .map_err(|e| CanonError::Write(e.to_string()))?;
                keyed.push((kb, k, val));
            }
            // Length first, then bytewise (the "deterministic encoding" rule).
            keyed.sort_by(|a, b| a.0.len().cmp(&b.0.len()).then_with(|| a.0.cmp(&b.0)));
            Ok(Value::Map(keyed.into_iter().map(|(_, k, val)| (k, val)).collect()))
        }
        Value::Array(items) => {
            let mut out = Vec::with_capacity(items.len());
            for it in items {
                out.push(canonicalize(it)?);
            }
            Ok(Value::Array(out))
        }
        other => Ok(other),
    }
}
