//! Client-side authorship for the card spike.
//!
//! An ed25519 signature over the *ciphertext* binds "this exact opaque contribution was produced by
//! the holder of key K". Because the signed message is the ciphertext, verification needs no
//! decryption key: it is **content-blind-safe**, so a reader or even the content-blind service can
//! check authorship without ever reading the message. This is the opt-in capability the no-login /
//! bearer path lacks (see the CAP-4 probe): without a signature, any link holder can append content
//! under any claimed name.

use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};

/// An author's signing capability (the secret half). Deterministic from a 32-byte seed for the spike.
pub struct Author {
    key: SigningKey,
}

impl Author {
    /// Build an author from a 32-byte seed.
    #[must_use]
    pub fn from_seed(seed: &[u8; 32]) -> Self {
        Self { key: SigningKey::from_bytes(seed) }
    }

    /// The author's public key (what a verifier needs).
    #[must_use]
    pub fn public_key(&self) -> [u8; 32] {
        self.key.verifying_key().to_bytes()
    }

    /// Sign a message (the ciphertext).
    #[must_use]
    pub fn sign(&self, message: &[u8]) -> [u8; 64] {
        self.key.sign(message).to_bytes()
    }
}

/// Verify `signature` over `message` against `public_key`. Content-blind-safe: `message` is the
/// ciphertext, so no decryption key is required to verify authorship. Returns `false` on any bad
/// input rather than panicking.
#[must_use]
pub fn verify(public_key: &[u8; 32], message: &[u8], signature: &[u8; 64]) -> bool {
    let Ok(vk) = VerifyingKey::from_bytes(public_key) else {
        return false;
    };
    vk.verify_strict(message, &Signature::from_bytes(signature)).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sign_verify_round_trips() {
        let author = Author::from_seed(&[5u8; 32]);
        let msg = b"opaque-ciphertext-bytes";
        let sig = author.sign(msg);
        assert!(verify(&author.public_key(), msg, &sig));
    }

    #[test]
    fn forged_or_tampered_is_rejected() {
        let alice = Author::from_seed(&[1u8; 32]);
        let mallory = Author::from_seed(&[2u8; 32]);
        let msg = b"opaque-ciphertext";
        let alice_sig = alice.sign(msg);

        // Mallory's signature does not verify under Alice's key.
        assert!(!verify(&alice.public_key(), msg, &mallory.sign(msg)));
        // A tampered message does not verify under Alice's genuine signature.
        assert!(!verify(&alice.public_key(), b"opaque-ciphertes!", &alice_sig));
    }
}
