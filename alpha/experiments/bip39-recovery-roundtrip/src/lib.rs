//! BIP39 paper-recovery round-trip — the Tier-1 lock (RUN-08 Part 1C).
//!
//! Serves the recovery-anchor's **Tier-1 lock** (the *mechanism*, not the trust tier):
//! `open-threads.md §2` (total-device-loss recovery, direction confirmed 2026-07-07 —
//! "build the lock now") and `EXPERIMENT-BACKLOG.md §6g` (the BIP39 round-trip, sketched
//! as "recoveryKey ↔ 24-word mnemonic (KAT-verified) then secretbox-wrap the masterKey").
//!
//! It proves the lock exists and round-trips bit-exact; **who may open it is I9** (the
//! identity/key-recovery trust tier — the owner's open call). This crate contains no
//! share-splitting, no release predicate, no threshold anything (RUN-08 firewall).
//!
//! Two primitives, both over the same 32-byte key material:
//!   1. `recoveryKey (32 B)` ⇄ 24-word BIP39 English mnemonic (bit-exact round-trip,
//!      checksum-validated on the way back).
//!   2. `masterKey` secretbox-wrapped under the `recoveryKey` (XSalsa20-Poly1305 via
//!      `dryoc`, the same vetted secretbox `altdrive-core` uses), unwrapping bit-exact;
//!      a wrong key or a tampered blob fails cleanly (authentication error, never a
//!      silently-wrong plaintext).
//!
//! Experiment-grade: the crate choices here (`bip39`, `dryoc`) are pinned exactly for a
//! reproducible spike, and are **not** a `[gates-release]` decision. The release-final
//! choice of mnemonic library, KDF, and AEAD is deferred to the recovery-anchor prototype
//! (Part 2 §7.3.9 `[to be determined by the recovery-anchor prototype]`).

use bip39::Mnemonic;
use dryoc::classic::crypto_secretbox::{
    crypto_secretbox_easy, crypto_secretbox_open_easy, Key, Nonce,
};
use dryoc::constants::{CRYPTO_SECRETBOX_MACBYTES, CRYPTO_SECRETBOX_NONCEBYTES};
use dryoc::rng::copy_randombytes;
use zeroize::{Zeroize, ZeroizeOnDrop};

/// A 256-bit key. `recoveryKey` and `masterKey` are both this width; a 32-byte
/// (256-bit) entropy is exactly a 24-word BIP39 mnemonic.
pub const KEY_BYTES: usize = 32;

/// Bytes a wrap adds on top of the plaintext: a fresh 24-byte nonce prefix plus the
/// 16-byte Poly1305 tag inside the secretbox ciphertext.
pub const WRAP_OVERHEAD: usize = CRYPTO_SECRETBOX_NONCEBYTES + CRYPTO_SECRETBOX_MACBYTES;

/// The 32-byte recovery key — the root the paper mnemonic reconstructs. Zeroized on drop;
/// deliberately carries no `Debug` (a `Debug` impl would be a way to log the secret).
#[derive(Clone, PartialEq, Eq, Zeroize, ZeroizeOnDrop)]
pub struct RecoveryKey([u8; KEY_BYTES]);

impl RecoveryKey {
    /// Wrap raw key bytes. The caller owns the entropy source; this spike does not
    /// generate keys (it proves the round-trip over a caller-supplied key).
    pub fn from_bytes(bytes: [u8; KEY_BYTES]) -> Self {
        Self(bytes)
    }

    /// Borrow the raw bytes (for wrapping / comparison in tests).
    pub fn as_bytes(&self) -> &[u8; KEY_BYTES] {
        &self.0
    }
}

/// The 32-byte master key protected under the recovery key. Same secret discipline as
/// [`RecoveryKey`].
#[derive(Clone, PartialEq, Eq, Zeroize, ZeroizeOnDrop)]
pub struct MasterKey([u8; KEY_BYTES]);

impl MasterKey {
    pub fn from_bytes(bytes: [u8; KEY_BYTES]) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; KEY_BYTES] {
        &self.0
    }
}

/// Failure modes, each distinct so a caller (and a test) can tell a checksum rejection
/// from a wrong-key unwrap. No variant carries secret bytes.
#[derive(Debug, PartialEq, Eq)]
pub enum RecoveryError {
    /// The mnemonic failed BIP39 validation (bad checksum, unknown word, wrong word
    /// count, …). Carries the human string of the underlying `bip39::Error`.
    Bip39(String),
    /// The mnemonic parsed but did not decode to a 32-byte (24-word) recovery key.
    NotTwentyFourWords { entropy_bytes: usize },
    /// The wrapped blob was shorter than a nonce + tag; it cannot be a valid wrap.
    MalformedWrap { len: usize },
    /// The secretbox authentication failed: a wrong key, or the ciphertext/nonce was
    /// tampered with. The plaintext is never returned in this case.
    Unwrap,
}

impl core::fmt::Display for RecoveryError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            RecoveryError::Bip39(e) => write!(f, "BIP39 validation failed: {e}"),
            RecoveryError::NotTwentyFourWords { entropy_bytes } => write!(
                f,
                "mnemonic decoded to {entropy_bytes} bytes of entropy, not the 32 a \
                 recovery key needs (a 24-word mnemonic)"
            ),
            RecoveryError::MalformedWrap { len } => write!(
                f,
                "wrapped blob is {len} bytes, shorter than the {WRAP_OVERHEAD}-byte nonce+tag overhead"
            ),
            RecoveryError::Unwrap => write!(
                f,
                "secretbox authentication failed (wrong key or tampered ciphertext)"
            ),
        }
    }
}

impl std::error::Error for RecoveryError {}

/// `recoveryKey (32 B)` → 24-word BIP39 English mnemonic.
///
/// Infallible for a 32-byte key: 256 bits is a valid BIP39 entropy length, so
/// `Mnemonic::from_entropy` cannot reject it.
pub fn recovery_key_to_mnemonic(key: &RecoveryKey) -> String {
    Mnemonic::from_entropy(key.as_bytes())
        .expect("32 bytes is a valid BIP39 entropy length")
        .to_string()
}

/// 24-word mnemonic → `recoveryKey`, validating the BIP39 checksum on the way in.
///
/// A corrupted word, a transposed pair, an out-of-wordlist word, or a wrong word count
/// all fail here rather than silently decoding to a wrong key.
pub fn mnemonic_to_recovery_key(phrase: &str) -> Result<RecoveryKey, RecoveryError> {
    let mnemonic = Mnemonic::parse(phrase).map_err(|e| RecoveryError::Bip39(e.to_string()))?;
    let mut entropy = mnemonic.to_entropy();
    let out = <[u8; KEY_BYTES]>::try_from(entropy.as_slice())
        .map(RecoveryKey::from_bytes)
        .map_err(|_| RecoveryError::NotTwentyFourWords {
            entropy_bytes: entropy.len(),
        });
    entropy.zeroize();
    out
}

/// secretbox-wrap `masterKey` under `recoveryKey`. Output layout: `nonce (24 B) ‖
/// ciphertext (tag 16 B ‖ sealed 32 B)`, i.e. `WRAP_OVERHEAD + KEY_BYTES` bytes. A fresh
/// random nonce is drawn per call.
pub fn wrap_master_key(recovery: &RecoveryKey, master: &MasterKey) -> Vec<u8> {
    let mut key: Key = *recovery.as_bytes();
    let mut nonce: Nonce = [0u8; CRYPTO_SECRETBOX_NONCEBYTES];
    copy_randombytes(&mut nonce);

    let mut ciphertext = vec![0u8; KEY_BYTES + CRYPTO_SECRETBOX_MACBYTES];
    crypto_secretbox_easy(&mut ciphertext, master.as_bytes(), &nonce, &key)
        .expect("secretbox encrypt of a fixed-size key cannot fail");

    let mut out = Vec::with_capacity(CRYPTO_SECRETBOX_NONCEBYTES + ciphertext.len());
    out.extend_from_slice(&nonce);
    out.extend_from_slice(&ciphertext);

    key.zeroize();
    out
}

/// Unwrap a blob produced by [`wrap_master_key`]. A wrong key or any tamper fails cleanly
/// with [`RecoveryError::Unwrap`]; the plaintext is never surfaced on failure.
pub fn unwrap_master_key(
    recovery: &RecoveryKey,
    wrapped: &[u8],
) -> Result<MasterKey, RecoveryError> {
    if wrapped.len() < WRAP_OVERHEAD {
        return Err(RecoveryError::MalformedWrap { len: wrapped.len() });
    }
    let (nonce_bytes, ciphertext) = wrapped.split_at(CRYPTO_SECRETBOX_NONCEBYTES);
    let nonce: Nonce = nonce_bytes
        .try_into()
        .expect("split_at guarantees a 24-byte prefix");
    let mut key: Key = *recovery.as_bytes();

    let mut plaintext = vec![0u8; ciphertext.len() - CRYPTO_SECRETBOX_MACBYTES];
    let opened = crypto_secretbox_open_easy(&mut plaintext, ciphertext, &nonce, &key)
        .map_err(|_| RecoveryError::Unwrap);
    key.zeroize();
    opened?;

    let out = <[u8; KEY_BYTES]>::try_from(plaintext.as_slice())
        .map(MasterKey::from_bytes)
        .map_err(|_| RecoveryError::NotTwentyFourWords {
            entropy_bytes: plaintext.len(),
        });
    plaintext.zeroize();
    out
}
