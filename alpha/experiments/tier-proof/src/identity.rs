//! Identity: signing keys and `did:key` encoding.
//!
//! The live model runs on `did:plc` + secp256k1 repo-commit signature chains
//! (RUN-14 proved that path). Here — with the live leg BLOCKED on credentials —
//! authors are `did:key` identities over ed25519, which need **no network to
//! resolve** and so serve as the multi-party stand-in behind the same resolver
//! interface ([`crate::delegation::DidResolver`]). Keys are seeded
//! deterministically so every test vector is reproducible.
//!
//! `did:key` encoding: `did:key:z` + base58btc(multicodec `0xed01` ‖ 32-byte
//! ed25519 public key). This is the same multibase/multicodec shape a real
//! DID-document `verificationMethod.publicKeyMultibase` carries, so the
//! encoding under test is genuine even though resolution is local.

use ed25519_dalek::{Signature, Signer as _, SigningKey, Verifier as _, VerifyingKey};

use crate::envelope::SignedBody;

/// The multicodec prefix for an ed25519 public key (varint `0xed01`).
const MULTICODEC_ED25519: [u8; 2] = [0xed, 0x01];

/// Errors decoding or resolving an identity.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum IdError {
    /// The string is not a `did:key:z…` identifier.
    #[error("not a did:key: {0}")]
    NotDidKey(String),
    /// The multibase/base58 body did not decode.
    #[error("bad multibase encoding")]
    BadMultibase,
    /// The decoded bytes were not a 0xed01-prefixed 32-byte ed25519 key.
    #[error("not an ed25519 verification key")]
    NotEd25519,
}

/// A signing identity (holds the private key). Deterministic from a seed.
pub struct Signer {
    key: SigningKey,
    did: String,
}

impl Signer {
    /// Construct a signer from a 32-byte seed (deterministic, reproducible).
    #[must_use]
    pub fn from_seed(seed: [u8; 32]) -> Self {
        let key = SigningKey::from_bytes(&seed);
        let did = did_key_from_verifying(&key.verifying_key());
        Self { key, did }
    }

    /// This identity's `did:key` string.
    #[must_use]
    pub fn did(&self) -> String {
        self.did.clone()
    }

    /// The public verifying key.
    #[must_use]
    pub fn verifying_key(&self) -> VerifyingKey {
        self.key.verifying_key()
    }

    /// Sign arbitrary bytes (64-byte ed25519 signature).
    #[must_use]
    pub fn sign_bytes(&self, msg: &[u8]) -> Vec<u8> {
        self.key.sign(msg).to_bytes().to_vec()
    }

    /// Sign the canonical bytes of a [`SignedBody`] — the envelope signature.
    ///
    /// # Panics
    /// Never in practice: the body is plain owned data that always encodes.
    #[must_use]
    pub fn sign_body(&self, body: &SignedBody) -> Vec<u8> {
        let bytes = crate::canonical::to_canonical(body)
            .expect("a SignedBody of owned scalars always encodes");
        self.sign_bytes(&bytes)
    }
}

/// Encode an ed25519 verifying key as a `did:key` string.
#[must_use]
pub fn did_key_from_verifying(vk: &VerifyingKey) -> String {
    let mut buf = Vec::with_capacity(34);
    buf.extend_from_slice(&MULTICODEC_ED25519);
    buf.extend_from_slice(vk.as_bytes());
    format!("did:key:z{}", bs58::encode(buf).into_string())
}

/// Recover the ed25519 verifying key from a `did:key` string.
///
/// # Errors
/// Returns [`IdError`] if the string is not a `did:key:z…` over a 0xed01
/// ed25519 key.
pub fn verifying_from_did_key(did: &str) -> Result<VerifyingKey, IdError> {
    let rest = did
        .strip_prefix("did:key:z")
        .ok_or_else(|| IdError::NotDidKey(did.to_string()))?;
    let bytes = bs58::decode(rest)
        .into_vec()
        .map_err(|_| IdError::BadMultibase)?;
    if bytes.len() != 34 || bytes[0..2] != MULTICODEC_ED25519 {
        return Err(IdError::NotEd25519);
    }
    let arr: [u8; 32] = bytes[2..].try_into().map_err(|_| IdError::NotEd25519)?;
    VerifyingKey::from_bytes(&arr).map_err(|_| IdError::NotEd25519)
}

/// Verify a 64-byte signature over `msg` against a `did:key` author.
///
/// # Errors
/// Returns [`IdError`] if the DID cannot be decoded. Returns `Ok(false)` for a
/// well-formed DID whose key rejects the signature.
pub fn verify_by_did(did: &str, msg: &[u8], sig: &[u8]) -> Result<bool, IdError> {
    let vk = verifying_from_did_key(did)?;
    let sig: [u8; 64] = match sig.try_into() {
        Ok(s) => s,
        Err(_) => return Ok(false),
    };
    Ok(vk.verify(msg, &Signature::from_bytes(&sig)).is_ok())
}
