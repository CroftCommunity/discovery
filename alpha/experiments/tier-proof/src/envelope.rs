//! The message envelope (RUN-16 §A.2, §A.5).
//!
//! An envelope is a signature over **scope id + author + antecedents + payload**
//! (the [`SignedBody`]), plus that signature. Identity is `H(envelope)` =
//! sha256 of the canonical bytes of the *whole* envelope, signature included —
//! so a re-signed copy is a different identity, and two different authors can
//! never share one. The signed body binds the message to its scope and causal
//! position: replaying the same payload into a different scope or under
//! different antecedents does not verify.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::identity::{verify_by_did, Signer};

/// The bytes a signature covers: scope id + author + antecedents + payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignedBody {
    /// The scope (backplane) this message belongs to.
    pub scope: String,
    /// The author's `did:key` (live: `did:plc`).
    pub author: String,
    /// Identity hashes of the antecedent envelopes (the causal position).
    pub antecedents: Vec<String>,
    /// The opaque application payload.
    #[serde(with = "serde_bytes")]
    pub payload: Vec<u8>,
}

/// A signed message envelope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Envelope {
    /// The signed body.
    pub body: SignedBody,
    /// The 64-byte ed25519 signature over `canonical(body)`.
    #[serde(with = "serde_bytes")]
    pub sig: Vec<u8>,
}

/// Why sealing or verifying an envelope failed.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum EnvError {
    /// The signed body did not encode canonically.
    #[error("canonical encode: {0}")]
    Encode(String),
    /// The author DID was malformed.
    #[error("identity: {0}")]
    Identity(#[from] crate::identity::IdError),
    /// The signature did not verify against the author's key over the body.
    #[error("signature does not verify (author, scope, antecedents, or payload)")]
    BadSignature,
}

impl Envelope {
    /// Seal a [`SignedBody`] with `signer` (the author). The signer's `did`
    /// should equal `body.author`; callers that mismatch produce an envelope
    /// that will not [`verify`](Self::verify).
    ///
    /// # Errors
    /// Returns [`EnvError::Encode`] if the body cannot be canonically encoded.
    pub fn seal(body: SignedBody, signer: &Signer) -> Result<Self, EnvError> {
        let bytes =
            crate::canonical::to_canonical(&body).map_err(|e| EnvError::Encode(e.to_string()))?;
        let sig = signer.sign_bytes(&bytes);
        Ok(Self { body, sig })
    }

    /// Verify the signature against the author DID over the canonical body.
    /// This is where context binding lives: any change to scope, author,
    /// antecedents, or payload invalidates the signature.
    ///
    /// # Errors
    /// Returns [`EnvError`] if the author DID is malformed or the signature
    /// does not verify.
    pub fn verify(&self) -> Result<(), EnvError> {
        let bytes = crate::canonical::to_canonical(&self.body)
            .map_err(|e| EnvError::Encode(e.to_string()))?;
        if verify_by_did(&self.body.author, &bytes, &self.sig)? {
            Ok(())
        } else {
            Err(EnvError::BadSignature)
        }
    }

    /// `H(envelope)` — sha256 over the canonical bytes of the whole envelope
    /// (signature included). This is the message identity.
    ///
    /// # Panics
    /// Never in practice: an `Envelope` of owned scalars always encodes.
    #[must_use]
    pub fn identity(&self) -> [u8; 32] {
        let bytes = crate::canonical::to_canonical(self)
            .expect("an Envelope of owned scalars always encodes");
        let mut h = Sha256::new();
        h.update(&bytes);
        h.finalize().into()
    }

    /// `H(envelope)` as lowercase hex.
    #[must_use]
    pub fn identity_hex(&self) -> String {
        hex::encode(self.identity())
    }
}
