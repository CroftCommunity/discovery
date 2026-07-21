//! Client-side sealing for the card spike.
//!
//! This crate is the **decrypt capability**. It lives with the key holder (in the card use case the
//! key travels in the URL fragment, never to a server). The content-blind service in `card-service`
//! MUST NOT depend on this crate, which is what makes "the service cannot read what it stores" a
//! compile-time fact rather than a convention. See the workspace README.
//!
//! Operates on raw bytes; key custody is the caller's. ChaCha20-Poly1305 AEAD.

use chacha20poly1305::aead::{Aead, KeyInit};
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};

/// Seal `plaintext` under a 32-byte key and 12-byte nonce, returning ciphertext (with auth tag).
///
/// # Errors
/// Returns [`SealError::Seal`] if the AEAD encrypt fails (only on pathological input).
pub fn seal(key: &[u8; 32], nonce: &[u8; 12], plaintext: &[u8]) -> Result<Vec<u8>, SealError> {
    let cipher = ChaCha20Poly1305::new(Key::from_slice(key));
    cipher
        .encrypt(Nonce::from_slice(nonce), plaintext)
        .map_err(|_| SealError::Seal)
}

/// Open `ciphertext` under a 32-byte key and 12-byte nonce.
///
/// # Errors
/// Returns [`SealError::Open`] if the key or nonce is wrong or the ciphertext was tampered with
/// (the AEAD tag check fails).
pub fn open(key: &[u8; 32], nonce: &[u8; 12], ciphertext: &[u8]) -> Result<Vec<u8>, SealError> {
    let cipher = ChaCha20Poly1305::new(Key::from_slice(key));
    cipher
        .decrypt(Nonce::from_slice(nonce), ciphertext)
        .map_err(|_| SealError::Open)
}

/// A sealing/opening failure. Deliberately opaque: `Open` never says *why* (bad key vs tamper).
#[derive(Debug, PartialEq, Eq)]
pub enum SealError {
    /// Encryption failed.
    Seal,
    /// Decryption failed (wrong key/nonce, or tampered ciphertext).
    Open,
}

impl std::fmt::Display for SealError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SealError::Seal => write!(f, "seal failed"),
            SealError::Open => write!(f, "open failed (bad key/nonce or tampered ciphertext)"),
        }
    }
}

impl std::error::Error for SealError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seal_open_round_trips() {
        let key = [7u8; 32];
        let nonce = [3u8; 12];
        let pt = b"happy birthday, Carol";
        let ct = seal(&key, &nonce, pt).expect("seal");
        assert_ne!(&ct[..], &pt[..], "ciphertext must not equal plaintext");
        let back = open(&key, &nonce, &ct).expect("open");
        assert_eq!(back, pt);
    }

    #[test]
    fn wrong_key_fails_to_open() {
        let ct = seal(&[7u8; 32], &[3u8; 12], b"secret").expect("seal");
        assert_eq!(open(&[8u8; 32], &[3u8; 12], &ct), Err(SealError::Open));
    }

    #[test]
    fn tampered_ciphertext_fails_to_open() {
        let key = [7u8; 32];
        let nonce = [3u8; 12];
        let mut ct = seal(&key, &nonce, b"secret").expect("seal");
        ct[0] ^= 0xff;
        assert_eq!(open(&key, &nonce, &ct), Err(SealError::Open));
    }
}
