//! Signing material — only to MINT fixtures (golden inline/remote records).
//!
//! Deterministic keys from a seed so the corpus is reproducible: the secret
//! scalar is `SHA-256(seed)` (truncated/extended per curve). Signing is RFC-6979
//! deterministic and low-S normalized, matching the spec and the reference impl
//! (empirically confirmed: EXP-LEX-03 interop probe). The clean-room VERIFIER in
//! `attest.rs` never depends on this module.

use ecdsa::signature::Signer;
use sha2::{Digest, Sha256};

use crate::cidfirst::RecordCid;
use crate::didkey::{Curve, PubKey};

/// A signing key over one of the three blessed curves.
pub enum SignKey {
    K256(k256::ecdsa::SigningKey),
    P256(p256::ecdsa::SigningKey),
    P384(p384::ecdsa::SigningKey),
}

impl SignKey {
    /// Deterministic key from a seed (reproducible fixtures).
    pub fn from_seed(curve: Curve, seed: &[u8]) -> Self {
        match curve {
            Curve::K256 => {
                let s: [u8; 32] = Sha256::digest(seed).into();
                SignKey::K256(k256::ecdsa::SigningKey::from_slice(&s).expect("valid k256 scalar"))
            }
            Curve::P256 => {
                let s: [u8; 32] = Sha256::digest(seed).into();
                SignKey::P256(p256::ecdsa::SigningKey::from_slice(&s).expect("valid p256 scalar"))
            }
            Curve::P384 => {
                // 48 bytes: SHA-256(seed) ‖ SHA-256(seed‖0x01).
                let a: [u8; 32] = Sha256::digest(seed).into();
                let mut s2in = seed.to_vec();
                s2in.push(0x01);
                let b: [u8; 32] = Sha256::digest(&s2in).into();
                let mut scalar = [0u8; 48];
                scalar[..32].copy_from_slice(&a);
                scalar[32..].copy_from_slice(&b[..16]);
                SignKey::P384(p384::ecdsa::SigningKey::from_slice(&scalar).expect("valid p384 scalar"))
            }
        }
    }

    pub fn public(&self) -> PubKey {
        match self {
            SignKey::K256(sk) => PubKey::K256(*sk.verifying_key()),
            SignKey::P256(sk) => PubKey::P256(*sk.verifying_key()),
            SignKey::P384(sk) => PubKey::P384(*sk.verifying_key()),
        }
    }

    pub fn did_key(&self) -> String {
        self.public().to_did_key()
    }

    /// Sign the 36-byte binary CID (the spec's "CID bytes"; interop-confirmed),
    /// returning raw `r‖s`, low-S normalized.
    pub fn sign_cid(&self, cid: &RecordCid) -> Vec<u8> {
        let msg = cid.to_bytes();
        match self {
            SignKey::K256(sk) => {
                let sig: k256::ecdsa::Signature = sk.sign(&msg);
                let sig = sig.normalize_s().unwrap_or(sig);
                sig.to_bytes().to_vec()
            }
            SignKey::P256(sk) => {
                let sig: p256::ecdsa::Signature = sk.sign(&msg);
                let sig = sig.normalize_s().unwrap_or(sig);
                sig.to_bytes().to_vec()
            }
            SignKey::P384(sk) => {
                let sig: p384::ecdsa::Signature = sk.sign(&msg);
                let sig = sig.normalize_s().unwrap_or(sig);
                sig.to_bytes().to_vec()
            }
        }
    }
}
