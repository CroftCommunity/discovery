//! AEAD encryption layer.
//!
//! Durable payloads are encrypted by *us*, keyed by the per-epoch content key
//! derived from the MLS exporter secret (see `mls.rs`). We deliberately do NOT
//! use MLS application-message encryption for stored data — MLS only manages
//! the rotating group key; this layer makes the payload opaque to the
//! transport/storage layer.
//!
//! ChaCha20-Poly1305 with a fresh random 96-bit nonce per message. The nonce is
//! stored alongside the ciphertext as `nonce(12) || ciphertext`.

use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Key, Nonce,
};
use rand::RngCore;

const NONCE_LEN: usize = 12;

/// Encrypt `plaintext` under a 32-byte content key. Output is `nonce || ct`.
pub fn encrypt(content_key: &[u8; 32], plaintext: &[u8]) -> Vec<u8> {
    let cipher = ChaCha20Poly1305::new(Key::from_slice(content_key));
    let mut nonce_bytes = [0u8; NONCE_LEN];
    rand::rngs::OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);
    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .expect("AEAD encryption failed");
    let mut out = Vec::with_capacity(NONCE_LEN + ciphertext.len());
    out.extend_from_slice(&nonce_bytes);
    out.extend_from_slice(&ciphertext);
    out
}

/// Decrypt a `nonce || ct` blob under a 32-byte content key.
pub fn decrypt(content_key: &[u8; 32], blob: &[u8]) -> Result<Vec<u8>, String> {
    if blob.len() < NONCE_LEN {
        return Err("ciphertext too short to contain a nonce".into());
    }
    let (nonce_bytes, ciphertext) = blob.split_at(NONCE_LEN);
    let cipher = ChaCha20Poly1305::new(Key::from_slice(content_key));
    let nonce = Nonce::from_slice(nonce_bytes);
    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| "AEAD decryption/authentication failed".to_string())
}
