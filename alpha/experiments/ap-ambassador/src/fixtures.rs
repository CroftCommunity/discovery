//! Fixtures — deterministic-seed RSA keypairs, an in-test resolver, canned
//! AP activities, signed request builders. Fixtures BEFORE features
//! (RUN-AP-01 §5 stop rule 2). No wall-clock entropy — `rand_chacha` is
//! seeded from `blake3(seed_id)`, so every fixture actor is byte-identical
//! across runs.
//!
//! The module is compiled into the crate proper (not cfg-test-gated) so
//! that path-dep consumers can build them if the outbound-delivery run
//! ever needs the same fixture shapes; no runtime code path uses these
//! builders. Every RSA key generation call takes an explicit seed —
//! nothing here reads a clock, an OS entropy source, or a global RNG.

use std::collections::BTreeMap;

use base64::{engine::general_purpose::STANDARD as B64, Engine as _};
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha20Rng;
use rsa::pkcs1v15::SigningKey;
use rsa::pkcs8::EncodePublicKey;
use rsa::signature::{RandomizedSigner, SignatureEncoding};
use rsa::{RsaPrivateKey, RsaPublicKey};
use sha2::{Digest, Sha256};

use crate::types::*;
use crate::verify::KeyResolver;

/// A fixture actor keypair — the RSA private key (for signing) and the
/// SPKI DER of the public key (for the resolver to hand to verify).
pub struct FixtureActor {
    pub key_id: KeyId,
    pub actor_url: ActorId,
    pub private: RsaPrivateKey,
    pub spki_der: Vec<u8>,
}

impl FixtureActor {
    /// Deterministic build: same `seed_id` string always produces the same
    /// keypair. Uses ChaCha20 seeded from a fixed 32-byte seed hashed from
    /// `seed_id`, so no wall-clock entropy is used.
    ///
    /// Key size is 1024 bits — deliberately small so tests are fast; RSA-1024
    /// is not deployment-grade but is fine for signature-scheme fixtures.
    pub fn generate(seed_id: &str, key_id: &str, actor_url: &str) -> Self {
        let mut rng = ChaCha20Rng::from_seed(*blake3::hash(seed_id.as_bytes()).as_bytes());
        let private = RsaPrivateKey::new(&mut rng, 1024)
            .expect("deterministic RSA-1024 key generation");
        let public = RsaPublicKey::from(&private);
        let spki = public
            .to_public_key_der()
            .expect("SPKI DER encoding");
        FixtureActor {
            key_id: KeyId::new(key_id),
            actor_url: ActorId::new(actor_url),
            private,
            spki_der: spki.as_bytes().to_vec(),
        }
    }
}

/// A fixture key resolver — a plain `keyId → SPKI DER` map. Same shape as
/// the runtime resolver trait; the fetch impl is out of scope this run
/// (AP-OC-6 territory).
#[derive(Default)]
pub struct FixtureKeyResolver {
    map: BTreeMap<String, Vec<u8>>,
}

impl FixtureKeyResolver {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, key_id: &KeyId, spki_der: &[u8]) {
        self.map.insert(key_id.0.clone(), spki_der.to_vec());
    }

    pub fn from_actor(actor: &FixtureActor) -> Self {
        let mut r = Self::new();
        r.insert(&actor.key_id, &actor.spki_der);
        r
    }
}

impl KeyResolver for FixtureKeyResolver {
    fn resolve(&self, key_id: &KeyId) -> Option<Vec<u8>> {
        self.map.get(&key_id.0).cloned()
    }
}

/// Build a Mastodon-shaped Follow activity JSON body targeting `object_url`.
pub fn follow_json(actor: &ActorId, object_url: &str, activity_id: &str) -> Vec<u8> {
    let json = format!(
        r#"{{"type":"Follow","id":"{aid}","actor":"{actor}","object":"{obj}"}}"#,
        aid = activity_id,
        actor = actor.0,
        obj = object_url,
    );
    json.into_bytes()
}

/// Build a Mastodon-shaped Undo Follow activity JSON body.
pub fn undo_follow_json(
    actor: &ActorId,
    inner_follow_id: &str,
    inner_object_url: &str,
    activity_id: &str,
) -> Vec<u8> {
    let json = format!(
        r#"{{"type":"Undo","id":"{aid}","actor":"{actor}","object":{{"type":"Follow","id":"{fid}","actor":"{actor}","object":"{obj}"}}}}"#,
        aid = activity_id,
        actor = actor.0,
        fid = inner_follow_id,
        obj = inner_object_url,
    );
    json.into_bytes()
}

/// Build a Mastodon-shaped Delete activity JSON body (Delete of an actor).
pub fn delete_actor_json(actor: &ActorId, activity_id: &str) -> Vec<u8> {
    let json = format!(
        r#"{{"type":"Delete","id":"{aid}","actor":"{actor}","object":"{obj}"}}"#,
        aid = activity_id,
        actor = actor.0,
        obj = actor.0,
    );
    json.into_bytes()
}

/// Compute the canonical `Digest` header value from a body: `SHA-256=<b64>`.
pub fn digest_header(body: &[u8]) -> String {
    let d = Sha256::digest(body);
    format!("SHA-256={}", B64.encode(d.as_slice()))
}

/// Build a Mastodon-style signed POST request. The signature covers the
/// standard header list `(request-target) host date digest`.
pub fn build_signed_post(
    actor: &FixtureActor,
    host: &str,
    path: &str,
    date: &str,
    body: Vec<u8>,
) -> crate::types::SignedRequest {
    let digest = digest_header(&body);
    let signing_string = format!(
        "(request-target): post {path}\nhost: {host}\ndate: {date}\ndigest: {digest}",
    );

    let signing_key = SigningKey::<Sha256>::new(actor.private.clone());
    let mut rng = ChaCha20Rng::from_seed([7u8; 32]);
    let signature = signing_key.sign_with_rng(&mut rng, signing_string.as_bytes());
    let sig_b64 = B64.encode(signature.to_bytes());

    let sig_header = format!(
        r#"keyId="{}",algorithm="rsa-sha256",headers="(request-target) host date digest",signature="{}""#,
        actor.key_id.0, sig_b64,
    );
    crate::types::SignedRequest {
        method: "POST".into(),
        path: path.into(),
        headers: vec![
            ("host".into(), host.into()),
            ("date".into(), date.into()),
            ("digest".into(), digest),
            ("signature".into(), sig_header),
        ],
        body,
    }
}

/// Fixture ap-origin proof for the P6 dual-proof binding. Produced by the
/// AP actor's private key over an activity body that names both the DID
/// and the antecedent-receipt id hex.
pub fn make_ap_origin_proof(
    actor: &FixtureActor,
    did_url: &str,
    antecedent_hex: &str,
) -> crate::binding::ApOriginProof {
    let activity = format!(
        r#"{{"type":"Consent","actor":"{actor}","did":"{did}","antecedent":"{ant}"}}"#,
        actor = actor.actor_url.0,
        did = did_url,
        ant = antecedent_hex,
    )
    .into_bytes();

    let signing_key = SigningKey::<Sha256>::new(actor.private.clone());
    let mut rng = ChaCha20Rng::from_seed([9u8; 32]);
    let signature = signing_key.sign_with_rng(&mut rng, &activity);
    let sig_b64 = B64.encode(signature.to_bytes());
    crate::binding::ApOriginProof {
        key_id: actor.key_id.clone(),
        activity_body: activity,
        signature_b64: sig_b64,
        actor_key_spki_der: actor.spki_der.clone(),
    }
}
