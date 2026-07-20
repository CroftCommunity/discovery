//! The §G reconciliation envelope — the five content-free fields a history
//! store sees in clear (inside the G-hist transport), plus fixture chains.
//!
//! history-durability.md §G: `subspace_id` (hashed, never plain),
//! `predecessor_digest` (or a genesis/checkpoint marker), `entry_digest` (the
//! reconciliation identity and the sealed blob's address), `counter`
//! (per-subspace, monotonic, single-writer), `size_hint` (padded byte length).
//! Explicitly excluded, and deliberately unrepresentable here: the Willow
//! path, wall-clock time, Meadowcap capabilities, the raw subspace id.
//!
//! The canonical encoding is dag-cbor over a single-char-keyed map through the
//! proven serde_ipld_dagcbor path — a spike-local canonical form, NOT a wire
//! pin (`[gates-release]` untouched).

use ipld_core::ipld::Ipld;
use std::collections::BTreeMap;

pub const DIGEST_LEN: usize = 32;
pub type Digest = [u8; DIGEST_LEN];

/// The `predecessor_digest` value that marks a chain head with no predecessor
/// (genesis). §G: "the content digest of this entry's predecessor in its
/// chain, or a genesis or checkpoint marker."
pub const GENESIS_MARKER: Digest = [0u8; DIGEST_LEN];

/// The §G reconciliation envelope. Content-free by construction: nothing in
/// this struct can hold a path, a wall-clock instant, or a capability.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Envelope {
    /// Hashed device-subspace id (§G / Willow-e2e hash-not-expose guidance).
    pub subspace: Digest,
    /// Digest of the predecessor entry, or [`GENESIS_MARKER`].
    pub predecessor: Digest,
    /// Content digest of this entry = address of its sealed blob.
    pub entry_digest: Digest,
    /// Per-subspace logical counter (monotonic, single-writer per §H).
    pub counter: u64,
    /// Padded byte length of the sealed blob (padded so true length does not
    /// leak — §G).
    pub size_hint: u64,
}

fn b(x: &Digest) -> Ipld {
    Ipld::Bytes(x.to_vec())
}

impl Envelope {
    /// Spike-local canonical bytes: dag-cbor of a map with fixed single-char
    /// keys (equal-length keys, so BTreeMap order and dag-cbor canonical
    /// order agree). Not a wire pin.
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut m = BTreeMap::new();
        m.insert("c".to_string(), Ipld::Integer(self.counter as i128));
        m.insert("e".to_string(), b(&self.entry_digest));
        m.insert("p".to_string(), b(&self.predecessor));
        m.insert("s".to_string(), b(&self.subspace));
        m.insert("z".to_string(), Ipld::Integer(self.size_hint as i128));
        serde_ipld_dagcbor::to_vec(&Ipld::Map(m)).expect("dag-cbor encode of pure map cannot fail")
    }

    /// Decode [`Self::canonical_bytes`]. Rejects anything malformed rather
    /// than repairing it (the lexicon-spec invalid-data posture, matchup §5-6).
    pub fn from_canonical(raw: &[u8]) -> Result<Self, String> {
        let v: Ipld = serde_ipld_dagcbor::from_slice(raw).map_err(|e| e.to_string())?;
        let Ipld::Map(m) = v else {
            return Err("envelope: not a map".into());
        };
        let digest = |k: &str| -> Result<Digest, String> {
            match m.get(k) {
                Some(Ipld::Bytes(v)) if v.len() == DIGEST_LEN => {
                    let mut d = [0u8; DIGEST_LEN];
                    d.copy_from_slice(v);
                    Ok(d)
                }
                _ => Err(format!("envelope: bad or missing field {k:?}")),
            }
        };
        let int = |k: &str| -> Result<u64, String> {
            match m.get(k) {
                Some(Ipld::Integer(i)) if *i >= 0 && *i <= u64::MAX as i128 => Ok(*i as u64),
                _ => Err(format!("envelope: bad or missing field {k:?}")),
            }
        };
        if m.len() != 5 {
            return Err(format!("envelope: expected 5 fields, got {}", m.len()));
        }
        Ok(Self {
            subspace: digest("s")?,
            predecessor: digest("p")?,
            entry_digest: digest("e")?,
            counter: int("c")?,
            size_hint: int("z")?,
        })
    }
}

/// Fixture: a hashed subspace id derived from a label (the §G hash-not-expose
/// rule applied to fixture data).
pub fn fixture_subspace(label: &str) -> Digest {
    *blake3::hash(format!("hist-spike-subspace:{label}").as_bytes()).as_bytes()
}

/// Fixture: deterministic pseudo-sealed blob bytes for (subspace, counter).
/// Stands in for a G-sealed content blob — opaque bytes, no structure.
pub fn fixture_blob(subspace: &Digest, counter: u64) -> Vec<u8> {
    let mut h = blake3::Hasher::new();
    h.update(b"hist-spike-blob");
    h.update(subspace);
    h.update(&counter.to_le_bytes());
    let seed = h.finalize();
    // 64 bytes of opaque fixture ciphertext-shaped content.
    let mut out = seed.as_bytes().to_vec();
    out.extend_from_slice(blake3::hash(seed.as_bytes()).as_bytes());
    out
}

/// Fixture: a well-linked chain of `n` (envelope, sealed-blob) pairs for one
/// subspace, counters `0..n`, predecessor-linked, genesis-marked at 0.
pub fn fixture_chain(label: &str, n: u64) -> Vec<(Envelope, Vec<u8>)> {
    let subspace = fixture_subspace(label);
    let mut out = Vec::new();
    let mut prev = GENESIS_MARKER;
    for counter in 0..n {
        let blob = fixture_blob(&subspace, counter);
        let entry_digest = *blake3::hash(&blob).as_bytes();
        let env = Envelope {
            subspace,
            predecessor: prev,
            entry_digest,
            counter,
            // Fixture padding: next multiple of 64 (a padded length, §G).
            size_hint: blob.len().div_ceil(64) as u64 * 64,
        };
        prev = entry_digest;
        out.push((env, blob));
    }
    out
}
