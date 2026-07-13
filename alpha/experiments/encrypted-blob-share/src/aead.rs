//! Real AEAD helpers built on `chacha20poly1305` (ChaCha20-Poly1305).
//!
//! Design principle (see crate root): plaintext media never enters the blob
//! store or the wire. We encrypt *before* the bytes reach the content-addressed
//! store, so the store and transport only ever see ciphertext.
//!
//! Framing decision: the blob store holds the *pure ciphertext* (ciphertext+tag)
//! and nothing else, so the BLAKE3 content hash is exactly `BLAKE3(ciphertext)`.
//! The AEAD nonce is NOT stored in the blob — it rides in the CRDT
//! attachment-reference record instead (the nonce is not secret). This makes the
//! "information needed to recover plaintext travels in the reference, not the
//! store" property concrete. The AEAD key itself is the shared MLS epoch content
//! key and is NEVER written anywhere — each member already holds it via MLS.

use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Key, Nonce,
};
use rand::RngCore;

/// Length of the ChaCha20-Poly1305 key, in bytes. Matches the MLS exporter
/// output length we request.
pub const KEY_LEN: usize = 32;
/// Length of the ChaCha20-Poly1305 nonce, in bytes.
pub const NONCE_LEN: usize = 12;

/// Encrypt `plaintext` under `key` with a fresh random 12-byte nonce.
///
/// Returns `(nonce, ciphertext)`. The ciphertext includes the Poly1305 tag.
pub fn encrypt(key: &[u8; KEY_LEN], plaintext: &[u8]) -> anyhow::Result<([u8; NONCE_LEN], Vec<u8>)> {
    let cipher = ChaCha20Poly1305::new(Key::from_slice(key));
    let mut nonce_bytes = [0u8; NONCE_LEN];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let ciphertext = cipher
        .encrypt(Nonce::from_slice(&nonce_bytes), plaintext)
        .map_err(|e| anyhow::anyhow!("AEAD encrypt failed: {e}"))?;
    Ok((nonce_bytes, ciphertext))
}

/// Decrypt `ciphertext` under `key` using `nonce`. Authentication failure
/// (wrong key, wrong nonce, or tampered bytes) returns an error.
pub fn decrypt(
    key: &[u8; KEY_LEN],
    nonce: &[u8; NONCE_LEN],
    ciphertext: &[u8],
) -> anyhow::Result<Vec<u8>> {
    let cipher = ChaCha20Poly1305::new(Key::from_slice(key));
    cipher
        .decrypt(Nonce::from_slice(nonce), ciphertext)
        .map_err(|e| anyhow::anyhow!("AEAD decrypt/authentication failed: {e}"))
}
