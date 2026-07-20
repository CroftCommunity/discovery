//! B1 — the §G envelope ↔ `ing.croft.hist.entry` record mapping (atproto_map
//! precedent: pure, no network, lossless at the payload level).
//!
//! The record shape mirrors `lexicons/ing.croft.hist.entry.json`: the five §G
//! fields plus the mandatory blob reference (matchup row 1: a PDS-resident
//! blob REQUIRES a referencing record — that requirement is this record).
//!
//! `OWNER-CALL: HS OC-2 pending` — reconciliation identity. The spike carries
//! BOTH the in-house `entry_digest` (blake3, the committed suite) and a
//! blessed-format blob CID (raw 0x55 + sha-256, data-model spec, matchup
//! §5-10) rather than fusing them; whether `entry_digest` ≡ blob CID is the
//! owner call, and either answer couples to a `[gates-release]` hash choice.

use crate::envelope::{Digest, Envelope, DIGEST_LEN};
use ipld_core::cid::multihash::Multihash;
use ipld_core::cid::Cid;
use ipld_core::ipld::Ipld;
use sha2::{Digest as _, Sha256};
use std::collections::BTreeMap;

pub const ENTRY_TYPE: &str = "ing.croft.hist.entry";
const RAW_CODEC: u64 = 0x55;
const SHA2_256: u64 = 0x12;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapError(pub String);

/// Blessed-format CID for a sealed blob: CIDv1, `raw` codec, sha-256
/// (data-model spec — matchup §5-10). Spike-local realization; HS OC-2 open.
pub fn blob_cid(blob: &[u8]) -> Cid {
    let digest = Sha256::digest(blob);
    let mh = Multihash::<64>::wrap(SHA2_256, &digest).expect("sha-256 digest fits");
    Cid::new_v1(RAW_CODEC, mh)
}

/// Envelope + sealed blob → the `ing.croft.hist.entry` record shape.
pub fn to_record(env: &Envelope, blob: &[u8]) -> Ipld {
    let mut blob_node = BTreeMap::new();
    blob_node.insert("$type".to_string(), Ipld::String("blob".to_string()));
    blob_node.insert("ref".to_string(), Ipld::Link(blob_cid(blob)));
    blob_node.insert(
        "mimeType".to_string(),
        Ipld::String("application/octet-stream".to_string()),
    );
    blob_node.insert("size".to_string(), Ipld::Integer(blob.len() as i128));

    let mut m = BTreeMap::new();
    m.insert("$type".to_string(), Ipld::String(ENTRY_TYPE.to_string()));
    m.insert("subspace".to_string(), Ipld::Bytes(env.subspace.to_vec()));
    m.insert(
        "predecessor".to_string(),
        Ipld::Bytes(env.predecessor.to_vec()),
    );
    m.insert(
        "entryDigest".to_string(),
        Ipld::Bytes(env.entry_digest.to_vec()),
    );
    m.insert("counter".to_string(), Ipld::Integer(env.counter as i128));
    m.insert("sizeHint".to_string(), Ipld::Integer(env.size_hint as i128));
    m.insert("blob".to_string(), Ipld::Map(blob_node));
    Ipld::Map(m)
}

/// Deterministic record bytes (spike-local dag-cbor; not a wire pin).
pub fn record_bytes(record: &Ipld) -> Vec<u8> {
    serde_ipld_dagcbor::to_vec(record).expect("dag-cbor encode of pure map cannot fail")
}

/// Record shape → (envelope, blob CID). Rejects malformed records whole
/// (lexicon invalid-data posture); never repairs.
pub fn from_record(record: &Ipld) -> Result<(Envelope, Cid), MapError> {
    let Ipld::Map(m) = record else {
        return Err(MapError("record: not a map".into()));
    };
    match m.get("$type") {
        Some(Ipld::String(t)) if t == ENTRY_TYPE => {}
        _ => return Err(MapError(format!("record: $type is not {ENTRY_TYPE:?}"))),
    }
    let digest = |k: &str| -> Result<Digest, MapError> {
        match m.get(k) {
            Some(Ipld::Bytes(v)) if v.len() == DIGEST_LEN => {
                let mut d = [0u8; DIGEST_LEN];
                d.copy_from_slice(v);
                Ok(d)
            }
            _ => Err(MapError(format!("record: bad or missing field {k:?}"))),
        }
    };
    let int = |k: &str| -> Result<u64, MapError> {
        match m.get(k) {
            Some(Ipld::Integer(i)) if *i >= 0 && *i <= u64::MAX as i128 => Ok(*i as u64),
            _ => Err(MapError(format!("record: bad or missing field {k:?}"))),
        }
    };
    let cid = match m.get("blob") {
        Some(Ipld::Map(bm)) => match bm.get("ref") {
            Some(Ipld::Link(c)) => *c,
            _ => return Err(MapError("record: blob node missing ref link".into())),
        },
        _ => return Err(MapError("record: missing blob node".into())),
    };
    Ok((
        Envelope {
            subspace: digest("subspace")?,
            predecessor: digest("predecessor")?,
            entry_digest: digest("entryDigest")?,
            counter: int("counter")?,
            size_hint: int("sizeHint")?,
        },
        cid,
    ))
}
