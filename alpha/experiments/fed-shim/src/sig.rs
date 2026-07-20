//! HTTP-signature emit — produce the byte-shape a real Mastodon emits
//! for a signed inbox POST.
//!
//! Fidelity source: `tests/specimens/mastodon-http-signature-header.md`.
//! Signing string covers `(request-target) host date digest`;
//! `Digest: SHA-256=<b64>` over raw body; `Signature: keyId=…,algorithm=
//! "rsa-sha256",headers=…,signature=…`.

use base64::{engine::general_purpose::STANDARD as B64, Engine as _};
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha20Rng;
use rsa::pkcs1v15::SigningKey;
use rsa::signature::{RandomizedSigner, SignatureEncoding};
use sha2::{Digest, Sha256};

use crate::actor::ShimActor;

/// A signed inbox POST — headers and body ready to be handed to an HTTP
/// client. The shim itself does NOT open sockets (`FED-SHIM.md §3`).
#[derive(Debug, Clone)]
pub struct SignedInboxPost {
    pub method: String,
    pub path: String,
    pub host: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

/// Build a signed inbox POST bound for `peer_inbox_url` (an absolute URL
/// like `https://bob.example/users/bob/inbox`).
///
/// Deterministic given (actor, peer_inbox_url, date, body). We seed the
/// RSA-blinding RNG from `blake3(actor_key_id ‖ peer_inbox_url ‖ date ‖
/// body)` — so the same inputs produce byte-identical output. Real
/// deployments would use OS randomness for signature blinding; this
/// determinism is a shim-specific claim (§1 row 9) that makes tests
/// reproducible.
pub fn build_inbox_post(
    actor: &ShimActor,
    peer_inbox_url: &str,
    date: &str,
    body: Vec<u8>,
) -> SignedInboxPost {
    // Split the peer inbox URL into host + path.
    let (host, path) = parse_url_host_path(peer_inbox_url);

    // Digest header first (covered by the signing string).
    let digest_b64 = B64.encode(Sha256::digest(&body).as_slice());
    let digest_header = format!("SHA-256={}", digest_b64);

    // Canonical signing string per draft-cavage:
    //   (request-target): post <path>
    //   host: <host>
    //   date: <date>
    //   digest: <digest-header-value>
    let signing_string = format!(
        "(request-target): post {path}\nhost: {host}\ndate: {date}\ndigest: {digest}",
        path = path,
        host = host,
        date = date,
        digest = digest_header,
    );

    // Deterministic RNG for the RSA-PSS-blinding step.
    let mut seed_hasher = blake3::Hasher::new();
    seed_hasher.update(actor.key_id.as_bytes());
    seed_hasher.update(peer_inbox_url.as_bytes());
    seed_hasher.update(date.as_bytes());
    seed_hasher.update(&body);
    let seed = *seed_hasher.finalize().as_bytes();
    let mut rng = ChaCha20Rng::from_seed(seed);

    let signing_key = SigningKey::<Sha256>::new(actor.private.clone());
    let signature = signing_key.sign_with_rng(&mut rng, signing_string.as_bytes());
    let sig_b64 = B64.encode(signature.to_bytes());

    // Specimen key-order for the Signature header: keyId, algorithm,
    // headers, signature.
    let sig_header = format!(
        r#"keyId="{key_id}",algorithm="rsa-sha256",headers="(request-target) host date digest",signature="{sig}""#,
        key_id = actor.key_id,
        sig = sig_b64,
    );

    SignedInboxPost {
        method: "POST".into(),
        path: path.to_string(),
        host: host.to_string(),
        headers: vec![
            ("host".into(), host.to_string()),
            ("date".into(), date.to_string()),
            ("digest".into(), digest_header),
            ("signature".into(), sig_header),
        ],
        body,
    }
}

/// Split `https://bob.example/users/bob/inbox` → ("bob.example",
/// "/users/bob/inbox"). Minimal — the shim does not URL-parse beyond
/// what draft-cavage's signing-string cares about.
fn parse_url_host_path(url: &str) -> (&str, &str) {
    // Strip scheme.
    let after_scheme = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))
        .unwrap_or(url);
    match after_scheme.find('/') {
        Some(slash) => (&after_scheme[..slash], &after_scheme[slash..]),
        None => (after_scheme, "/"),
    }
}
