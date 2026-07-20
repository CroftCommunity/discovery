//! `ing.croft.hist.entry` — the synthetic envelope this spike writes.
//!
//! Shape mirrors the reconciliation envelope of
//! beta/impl/drystone-design/history-durability.md §G, transposed to a public
//! atproto record. Content is a padded synthetic byte pattern — nothing
//! personal, nothing operational, tagged with a note so a human who stumbles
//! on the record understands it. Everything here is a fixture.

use crate::canonical::{canonical_dag_cbor, cid_v1_dag_cbor};
use cid::Cid;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const HIST_ENTRY_TYPE: &str = "ing.croft.hist.entry";

/// A subspace name; the atproto side uses the hashed prefix as an rkey prefix.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Subspace(pub String);

impl Subspace {
    /// 4-byte SHA-256 prefix, hex-lower. Chosen small so the rkey is short and
    /// human-eyeballable at debugging time; collisions are irrelevant at this
    /// scale.
    pub fn hash_prefix(&self) -> String {
        let d = Sha256::digest(self.0.as_bytes());
        hex::encode(&d[..4])
    }
}

/// An rkey has the shape `<subspace-hash>_<7-digit-counter>` — lexicographic
/// enumeration therefore visits (subspace, counter) in sort order.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rkey(pub String);

impl Rkey {
    pub fn from(subspace: &Subspace, counter: u32) -> Self {
        Rkey(format!("{}_{:07}", subspace.hash_prefix(), counter))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Predecessor link: a CID (as string) or null for the chain's first entry.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum Predecessor {
    None,
    Cid(CidLink),
}

/// dag-cbor / dag-json link — serializes as `{"$link": "bafy..."}`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CidLink {
    #[serde(rename = "$link")]
    pub link: String,
}

/// dag-cbor / dag-json bytes — serializes as `{"$bytes": "<b64>"}`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DagBytes {
    #[serde(rename = "$bytes")]
    pub b64: String,
}

impl DagBytes {
    pub fn from_bytes(b: &[u8]) -> Self {
        use base64::Engine as _;
        DagBytes {
            b64: base64::engine::general_purpose::STANDARD_NO_PAD.encode(b),
        }
    }
}

/// The record as it lands in the PDS.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HistEntry {
    #[serde(rename = "$type")]
    pub type_: String,
    /// Hex-lower 8-char subspace hash (matches the rkey prefix).
    pub subspace: String,
    /// Monotonic counter within the subspace.
    pub counter: u32,
    /// Predecessor entry CID, or `null` for the chain's first record.  The
    /// field is always PRESENT (never elided) so the chain-gap detector's shape
    /// is uniform for every entry.
    pub predecessor: Option<CidLink>,
    /// Padded content size; the store's meer-level exposure.
    pub size_hint: u32,
    /// Synthetic sealed-blob-shaped payload — deterministic pattern so replays
    /// match, not zero (a probe against zero-detection heuristics on any relay).
    pub content: DagBytes,
    /// One-line human-readable stamp; mandatory so anyone stumbling on the
    /// record understands what it is and why it exists.
    pub note: String,
}

impl HistEntry {
    /// Build a synthetic entry with a deterministic content pattern.
    pub fn new(
        subspace: &Subspace,
        counter: u32,
        predecessor: Option<Cid>,
        size_hint: u32,
    ) -> Self {
        let mut content = Vec::with_capacity(size_hint as usize);
        let seed = format!("{}:{}", subspace.0, counter);
        let mut h = Sha256::new();
        h.update(seed.as_bytes());
        let mut block = h.finalize_reset().to_vec();
        while content.len() < size_hint as usize {
            content.extend_from_slice(&block);
            h.update(&block);
            block = h.finalize_reset().to_vec();
        }
        content.truncate(size_hint as usize);

        HistEntry {
            type_: HIST_ENTRY_TYPE.to_string(),
            subspace: subspace.hash_prefix(),
            counter,
            predecessor: predecessor.map(|c| CidLink { link: c.to_string() }),
            size_hint,
            content: DagBytes::from_bytes(&content),
            note: NOTE.to_string(),
        }
    }

    pub fn canonical_bytes(&self) -> Vec<u8> {
        canonical_dag_cbor(self)
    }

    pub fn cid(&self) -> Cid {
        cid_v1_dag_cbor(&self.canonical_bytes())
    }
}

const NOTE: &str =
    "RUN-HIST-02 rev B synthetic fixture (ing.croft.hist.entry). Non-personal, non-operational; deleted at run teardown.";
