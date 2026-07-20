//! HTTP-signature verification for inbound ActivityPub activities.
//!
//! Mastodon-shaped signature (draft-cavage-http-signatures): the sender
//! signs a canonical string built from `(request-target)`, `host`, `date`,
//! `digest`; algorithm = `rsa-sha256`; digest = `SHA-256=<base64>` of the raw
//! body. Distinct error variants (P1 T-AP1.5, no collapse):
//!   - `SignatureMismatch`   — key resolves, digest ok, but signature invalid
//!   - `KeyResolutionFailed` — no key for keyId
//!   - `DigestMismatch`      — Digest header ≠ SHA-256(body)
//!   - `MalformedActivity`   — parse / structural failure
//!   - `EvidenceRedacted`    — attempted verify on a redacted record

use crate::types::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerifyError {
    SignatureMismatch,
    KeyResolutionFailed,
    DigestMismatch,
    MalformedActivity(String),
    EvidenceRedacted,
}

impl std::fmt::Display for VerifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VerifyError::SignatureMismatch => write!(f, "signature does not verify"),
            VerifyError::KeyResolutionFailed => write!(f, "key resolution failed"),
            VerifyError::DigestMismatch => write!(f, "digest header does not match body"),
            VerifyError::MalformedActivity(m) => write!(f, "malformed activity: {}", m),
            VerifyError::EvidenceRedacted => write!(f, "evidence redacted"),
        }
    }
}
impl std::error::Error for VerifyError {}

pub trait KeyResolver {
    fn resolve(&self, key_id: &KeyId) -> Option<Vec<u8>>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignatureHeader {
    pub key_id: KeyId,
    pub algorithm: String,
    pub headers: Vec<String>,
    pub signature_b64: String,
}

#[derive(Debug, Clone)]
pub struct VerifiedActivity {
    pub activity: InboundActivity,
    pub actor_key_spki_der: Vec<u8>,
    pub actor_key_id: KeyId,
}

pub fn parse_signature_header(_value: &str) -> Result<SignatureHeader, VerifyError> {
    unimplemented!("P1 GREEN: parse Mastodon-shaped Signature header")
}

pub fn build_signing_string(
    _req: &SignedRequest,
    _covered: &[String],
) -> Result<String, VerifyError> {
    unimplemented!("P1 GREEN: canonical signing string over covered headers")
}

pub fn parse_ap_activity(_raw_body: &[u8]) -> Result<InboundActivity, VerifyError> {
    unimplemented!("P1 GREEN: parse AP JSON body into InboundActivity")
}

pub fn verify_ap_http_signature<R: KeyResolver>(
    _req: &SignedRequest,
    _resolver: &R,
) -> Result<VerifiedActivity, VerifyError> {
    unimplemented!("P1 GREEN: verify HTTP signature (RSA-SHA256 over canonical string)")
}
