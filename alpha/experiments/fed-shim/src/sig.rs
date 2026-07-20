//! HTTP-signature emit — produce the byte-shape a real Mastodon emits
//! for a signed inbox POST.
//!
//! Fidelity source: `tests/specimens/mastodon-http-signature-header.md`.
//! Signing string covers `(request-target) host date digest`;
//! `Digest: SHA-256=<b64>` over raw body; `Signature: keyId=…,algorithm=
//! "rsa-sha256",headers=…,signature=…`.
//!
//! The VERIFY direction is delegated to `ap-ambassador::verify` unchanged
//! (dev-dep path). This module produces the SEND direction only.

use crate::actor::ShimActor;

/// A signed inbox POST — headers and body ready to be handed to an HTTP
/// client. The shim itself does NOT open sockets (`FED-SHIM.md §3`).
#[derive(Debug, Clone)]
pub struct SignedInboxPost {
    pub method: String,
    pub path: String,
    pub host: String,
    /// Ordered header pairs — arrival order matters for the signing
    /// string (`(request-target) host date digest` is the covered order,
    /// per specimen).
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

/// Build a signed inbox POST bound for `peer_inbox_url` (an absolute URL
/// like `https://bob.example/users/bob/inbox`).
///
/// Deterministic given (actor, peer_inbox_url, date, body). `FED-SHIM.md
/// §1 row 9` — the shim exposes this determinism explicitly so tests
/// are reproducible; a real deployment would use a wall-clock Date.
pub fn build_inbox_post(
    _actor: &ShimActor,
    _peer_inbox_url: &str,
    _date: &str,
    _body: Vec<u8>,
) -> SignedInboxPost {
    unimplemented!("fed-shim GREEN: sign HTTP request per Mastodon shape, produce ready-to-POST bytes")
}
