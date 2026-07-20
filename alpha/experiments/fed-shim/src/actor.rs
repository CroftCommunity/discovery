//! ShimActor — identity + RSA keypair with wire-fidelity to a Mastodon
//! Person actor.

use rsa::{RsaPrivateKey, RsaPublicKey};

/// A shim actor: everything needed to sign outgoing requests, verify
/// incoming requests targeted at this actor, and serve a byte-fidelity
/// actor-document JSON-LD.
pub struct ShimActor {
    pub actor_url: String,
    pub preferred_username: String,
    pub inbox_url: String,
    pub outbox_url: String,
    pub key_id: String,
    pub private: RsaPrivateKey,
    pub public: RsaPublicKey,
    /// Public key in SPKI PEM format (the shape actor documents carry).
    pub public_key_pem: String,
    /// Public key in SPKI DER bytes (the shape ap-ambassador's verify
    /// path consumes — matches its `KeyResolver` interface).
    pub public_key_spki_der: Vec<u8>,
}

impl ShimActor {
    /// Deterministic build: same `seed_id` string always produces the
    /// same keypair. Uses ChaCha20 seeded from `blake3(seed_id)`, so no
    /// wall-clock entropy is used. RSA-1024 for test speed (declared
    /// stand-in — `FED-SHIM.md §6`).
    pub fn generate(
        _seed_id: &str,
        _actor_url: &str,
        _preferred_username: &str,
    ) -> Self {
        unimplemented!("fed-shim GREEN: RSA-1024 keypair + PEM serialization")
    }
}

/// The bytes of the shim actor's JSON-LD document (`FED-SHIM.md §1 row 6`,
/// specimen `mastodon-actor-doc-observed-shape.md`). Byte-fidelity to the
/// Mastodon shape.
pub fn actor_document(_actor: &ShimActor) -> Vec<u8> {
    unimplemented!("fed-shim GREEN: emit actor-document JSON-LD, key-order and byte-shape per specimen")
}
