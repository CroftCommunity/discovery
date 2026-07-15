//! Content-addressed blob store — the transport + storage seam.
//!
//! THIS IS THE ONE DELIBERATELY-STUBBED LAYER. The build brief permits
//! simulating the transport with a direct in-process store and explicitly
//! flags `iroh`/`iroh-blobs` as a high-churn API and the least-risky seam.
//!
//! What is real here: content addressing by BLAKE3 digest — the *same*
//! primitive `iroh-blobs` uses to address blobs. A blob's key is the hash of
//! its bytes, so storing the same ciphertext twice is idempotent and a reader
//! who knows the hash can fetch the exact bytes. What is stubbed: the QUIC
//! transport between two endpoints. Both peers share one `BlobStore` and
//! "fetch over the network" is a direct map lookup.
//!
//! Resolvable real versions (confirmed via `cargo`, not linked into this
//! binary to avoid the async-runtime weight and API churn of a first cut):
//!   iroh = 0.98.2, iroh-blobs = 0.102.0
//!
//! Swapping this for the real thing means implementing the same
//! `put`/`get`-by-`Hash` shape against `iroh_blobs` and routing fetches over an
//! `iroh::Endpoint`; nothing above this layer (addressing, crypto, MLS,
//! Automerge) would change.

use std::collections::HashMap;

/// A BLAKE3 content hash, hex-encoded for display. Mirrors an iroh blob `Hash`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BlobHash(pub String);

impl std::fmt::Display for BlobHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0[..16.min(self.0.len())])
    }
}

/// In-process content-addressed blob store shared by all simulated peers.
#[derive(Default)]
pub struct BlobStore {
    /// content hash -> blob bytes
    blobs: HashMap<String, Vec<u8>>,
    /// 4-tuple storage key -> content hash (the "naming" / mutable pointer layer)
    pointers: HashMap<String, BlobHash>,
}

impl BlobStore {
    pub fn new() -> Self {
        Self::default()
    }

    /// Store bytes content-addressed by their BLAKE3 hash; returns the hash.
    pub fn put(&mut self, bytes: &[u8]) -> BlobHash {
        let hash = BlobHash(blake3::hash(bytes).to_hex().to_string());
        self.blobs.insert(hash.0.clone(), bytes.to_vec());
        hash
    }

    /// Fetch bytes by content hash (the "download over transport" step).
    pub fn get(&self, hash: &BlobHash) -> Option<&[u8]> {
        self.blobs.get(&hash.0).map(|v| v.as_slice())
    }

    /// Publish that a 4-tuple storage key currently points at a content hash.
    pub fn set_pointer(&mut self, storage_key: &str, hash: BlobHash) {
        self.pointers.insert(storage_key.to_string(), hash);
    }

    /// Resolve a 4-tuple storage key to the content it currently points at.
    pub fn resolve(&self, storage_key: &str) -> Option<&[u8]> {
        let hash = self.pointers.get(storage_key)?;
        self.get(hash)
    }
}
