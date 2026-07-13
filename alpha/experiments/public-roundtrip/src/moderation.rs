//! Moderation-labels experiment helpers: reconstruct the canonical bytes a
//! labeler signed for a label, so we can verify label signatures with the same
//! machinery as repo commit signatures (`repo_verify::verify_signature`).
//!
//! A label is signed over the DAG-CBOR of the label object *without* its `sig`
//! field. Note a key subtlety we discovered empirically: in the label lexicon
//! (`com.atproto.label.defs#label`) the `cid` field is a **string**, not a CBOR
//! CID link (tag 42) — unlike CIDs inside the repo MST. So it is encoded as a
//! plain string here; treating it as a link makes cid-bearing labels fail to
//! verify while cid-less (account-level) labels still pass.

use anyhow::{bail, Context, Result};
use base64::Engine;
use ipld_core::ipld::Ipld;
use serde_json::Value;
use std::collections::BTreeMap;

/// Canonical DAG-CBOR of a label minus `sig` — the bytes the labeler signed.
pub fn label_unsigned_dagcbor(label: &Value) -> Result<Vec<u8>> {
    let obj = label.as_object().context("label is not a JSON object")?;
    let mut map: BTreeMap<String, Ipld> = BTreeMap::new();
    for (k, v) in obj {
        if k == "sig" {
            continue;
        }
        map.insert(k.clone(), json_to_ipld(v)?);
    }
    serde_ipld_dagcbor::to_vec(&Ipld::Map(map)).context("encode unsigned label")
}

/// Extract a label's signature bytes. queryLabels renders CBOR bytes as
/// `{"$bytes":"<base64>"}` (DAG-JSON); older shapes may use a bare string.
pub fn label_sig_bytes(label: &Value) -> Result<Vec<u8>> {
    let sig = label.get("sig").context("label has no sig")?;
    let b64 = if let Some(s) = sig.as_str() {
        s.to_string()
    } else if let Some(s) = sig.get("$bytes").and_then(|v| v.as_str()) {
        s.to_string()
    } else {
        bail!("unrecognized sig encoding");
    };
    // DAG-JSON $bytes is standard base64, no padding.
    base64::engine::general_purpose::STANDARD_NO_PAD
        .decode(b64.trim_end_matches('='))
        .context("base64-decode sig")
}

fn json_to_ipld(v: &Value) -> Result<Ipld> {
    Ok(match v {
        Value::Null => Ipld::Null,
        Value::Bool(b) => Ipld::Bool(*b),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ipld::Integer(i as i128)
            } else if let Some(u) = n.as_u64() {
                Ipld::Integer(u as i128)
            } else {
                Ipld::Float(n.as_f64().unwrap_or(0.0))
            }
        }
        Value::String(s) => Ipld::String(s.clone()),
        Value::Array(a) => Ipld::List(a.iter().map(json_to_ipld).collect::<Result<_>>()?),
        Value::Object(o) => {
            let mut m = BTreeMap::new();
            for (k, val) in o {
                m.insert(k.clone(), json_to_ipld(val)?);
            }
            Ipld::Map(m)
        }
    })
}
