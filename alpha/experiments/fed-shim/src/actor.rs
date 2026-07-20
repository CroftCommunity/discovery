//! ShimActor — identity + RSA keypair with wire-fidelity to a Mastodon
//! Person actor.

use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha20Rng;
use rsa::pkcs8::{EncodePublicKey, LineEnding};
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
    pub fn generate(seed_id: &str, actor_url: &str, preferred_username: &str) -> Self {
        let mut rng = ChaCha20Rng::from_seed(*blake3::hash(seed_id.as_bytes()).as_bytes());
        let private = RsaPrivateKey::new(&mut rng, 1024)
            .expect("deterministic RSA-1024 key generation");
        let public = RsaPublicKey::from(&private);
        let der = public
            .to_public_key_der()
            .expect("SPKI DER encoding");
        // PEM in the exact shape actor documents carry: standard SPKI,
        // "-----BEGIN PUBLIC KEY-----" delimiters, LF line endings.
        let pem = public
            .to_public_key_pem(LineEnding::LF)
            .expect("SPKI PEM encoding");
        ShimActor {
            actor_url: actor_url.to_string(),
            preferred_username: preferred_username.to_string(),
            inbox_url: format!("{}/inbox", actor_url),
            outbox_url: format!("{}/outbox", actor_url),
            key_id: format!("{}#main-key", actor_url),
            private,
            public,
            public_key_pem: pem,
            public_key_spki_der: der.as_bytes().to_vec(),
        }
    }
}

/// The bytes of the shim actor's JSON-LD document (`FED-SHIM.md §1 row 6`,
/// specimen `mastodon-actor-doc-observed-shape.md`). Byte-fidelity to the
/// Mastodon shape: single-line compact JSON, key-order
/// `@context, id, type, preferredUsername, inbox, outbox, publicKey`;
/// inner `publicKey` order `id, owner, publicKeyPem`. PEM newlines are
/// JSON-escaped as `\n` INSIDE the string; no trailing newline on the
/// overall body.
pub fn actor_document(actor: &ShimActor) -> Vec<u8> {
    // Escape PEM's real newlines as JSON `\n` sequences (i.e. the two
    // bytes `\`, `n`), matching Mastodon's emit.
    let pem_escaped = actor.public_key_pem.replace('\n', "\\n");
    let s = format!(
        concat!(
            r#"{{"@context":["https://www.w3.org/ns/activitystreams","https://w3id.org/security/v1"],"#,
            r#""id":"{actor}","type":"Person","preferredUsername":"{user}","#,
            r#""inbox":"{inbox}","outbox":"{outbox}","#,
            r#""publicKey":{{"id":"{key_id}","owner":"{actor}","publicKeyPem":"{pem}"}}}}"#,
        ),
        actor = actor.actor_url,
        user = actor.preferred_username,
        inbox = actor.inbox_url,
        outbox = actor.outbox_url,
        key_id = actor.key_id,
        pem = pem_escaped,
    );
    s.into_bytes()
}
