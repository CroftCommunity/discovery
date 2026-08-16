//! The **outer seal**: E96's nested double-sealing, over a whole MLS envelope.
//!
//! S7 measured that `group_id`, `epoch` and `content_type` sit *beside* the ciphertext in RFC
//! 9420's framing and are readable with no key, so a carrier can bucket a conversation across
//! every queue-name rotation. E96 filed that and was parked on one objection: **an outer seal
//! hides the routing metadata, so how does the carrier route?**
//!
//! The two-target design retired that objection. The queue name comes from
//! `export_secret("croft/meer-queue/v1")`, which was never inside the envelope — so the carrier's
//! routing handle survives a seal that covers the envelope completely. This module is that seal.
//!
//! **Nothing here is stood in.** The AEAD is the group ciphersuite's own, reached through the
//! provider's crypto, and the key is a real MLS exporter output. A placeholder cipher here would
//! be the methodology's canonical forbidden move, since every claim S17 makes about what a carrier
//! can see runs through this code.
//!
//! ## The wrapping rule, which is the whole subtlety
//!
//! An object must be wrapped with the key of the **epoch whose queue carries it**. For ordinary
//! traffic that is simply the current epoch. For the **commit that closes an epoch** it is the
//! epoch being closed, *not* the one being opened — a returning member arrives at hop N holding
//! epoch N, and must be able to open the thing that carries her to N+1. Wrap that commit at N+1
//! and the walk deadlocks at the first hop: she cannot open the object that would let her derive
//! the key to open it.
//!
//! OpenMLS exports secrets for the **current epoch only**, so this is not a mistake the API can
//! prevent. That is why [`OuterKey`] is a first-class value a caller derives *before* committing
//! and holds across the commit, rather than something [`wrap`] reaches for implicitly.

use openmls::prelude::*;
use openmls_traits::crypto::OpenMlsCrypto;
use openmls_traits::random::OpenMlsRand;
use openmls_traits::types::AeadType;
use openmls_traits::OpenMlsProvider;
use zeroize::{Zeroize, ZeroizeOnDrop};

use mls_replant::Persona;

/// The exporter label for the outer seal. Distinct from the queue-name label so that a party who
/// somehow learned one derives nothing about the other — they are independent exporter outputs.
pub const OUTER_SEAL_LABEL: &str = "croft/meer-outer-seal/v1";

/// What can go wrong wrapping or unwrapping.
#[derive(Debug, thiserror::Error)]
pub enum OuterSealError {
    /// The group refused to export the outer-seal secret.
    #[error("could not derive the outer-seal key: {0}")]
    Derive(String),
    /// Randomness for the nonce was unavailable.
    #[error("could not draw a nonce: {0}")]
    Nonce(String),
    /// The AEAD refused to encrypt.
    #[error("outer seal failed: {0}")]
    Seal(String),
    /// The object is shorter than a nonce, so it cannot be one of ours.
    #[error("outer-sealed object is truncated: {len} bytes, need more than {nonce_len}")]
    Truncated { len: usize, nonce_len: usize },
    /// The AEAD refused to decrypt — **the access-control case**. A non-member lands here.
    #[error("outer seal did not open: {0}")]
    Open(String),
}

/// One epoch's outer-seal key.
///
/// A named value rather than a `Vec<u8>` because *which epoch it belongs to* is the load-bearing
/// fact (see the wrapping rule above), and because it is secret material: it zeroizes on drop and
/// deliberately implements no `Debug`, so it cannot reach a log line by accident.
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct OuterKey {
    key: Vec<u8>,
    #[zeroize(skip)]
    aead: AeadType,
    #[zeroize(skip)]
    nonce_len: usize,
}

/// Derive the outer-seal key for the epoch `group` is at **right now**.
///
/// Hold the result across a commit to wrap the object that closes this epoch.
///
/// # Errors
/// [`OuterSealError::Derive`] if the library refuses the export.
pub fn outer_key(group: &MlsGroup, who: &Persona) -> Result<OuterKey, OuterSealError> {
    let cs = group.ciphersuite();
    let key = group
        .export_secret(
            who.provider.crypto(),
            OUTER_SEAL_LABEL,
            &[],
            cs.aead_key_length(),
        )
        .map_err(|e| OuterSealError::Derive(e.to_string()))?;
    Ok(OuterKey {
        key,
        aead: cs.aead_algorithm(),
        nonce_len: cs.aead_nonce_length(),
    })
}

/// Wrap `inner` under `key`, returning `nonce || ciphertext`.
///
/// The nonce travels in the clear ahead of the ciphertext — it is not secret, and prefixing it
/// keeps the object a single opaque blob to the carrier, which is the property under test.
///
/// # Errors
/// [`OuterSealError::Nonce`] or [`OuterSealError::Seal`].
pub fn wrap_with(
    key: &OuterKey,
    provider: &impl OpenMlsProvider,
    inner: &[u8],
) -> Result<Vec<u8>, OuterSealError> {
    let nonce = provider
        .rand()
        .random_vec(key.nonce_len)
        .map_err(|e| OuterSealError::Nonce(e.to_string()))?;
    let ciphertext = provider
        .crypto()
        .aead_encrypt(key.aead, &key.key, inner, &nonce, &[])
        .map_err(|e| OuterSealError::Seal(e.to_string()))?;
    let mut out = nonce;
    out.extend_from_slice(&ciphertext);
    Ok(out)
}

/// Open an object wrapped under `key`.
///
/// # Errors
/// [`OuterSealError::Truncated`] if it is too short to carry a nonce, or
/// [`OuterSealError::Open`] if the AEAD tag does not verify — which is what a non-member gets.
pub fn unwrap_with(
    key: &OuterKey,
    provider: &impl OpenMlsProvider,
    wrapped: &[u8],
) -> Result<Vec<u8>, OuterSealError> {
    if wrapped.len() <= key.nonce_len {
        return Err(OuterSealError::Truncated {
            len: wrapped.len(),
            nonce_len: key.nonce_len,
        });
    }
    let (nonce, ciphertext) = wrapped.split_at(key.nonce_len);
    provider
        .crypto()
        .aead_decrypt(key.aead, &key.key, ciphertext, nonce, &[])
        .map_err(|e| OuterSealError::Open(e.to_string()))
}

/// Wrap `inner` at the epoch `group` is at now — the ordinary-traffic case.
///
/// # Errors
/// As [`outer_key`] and [`wrap_with`].
pub fn wrap(group: &MlsGroup, who: &Persona, inner: &[u8]) -> Result<Vec<u8>, OuterSealError> {
    let key = outer_key(group, who)?;
    wrap_with(&key, &who.provider, inner)
}

/// Open an object wrapped at the epoch `group` is at now.
///
/// # Errors
/// As [`outer_key`] and [`unwrap_with`].
pub fn unwrap(group: &MlsGroup, who: &Persona, wrapped: &[u8]) -> Result<Vec<u8>, OuterSealError> {
    let key = outer_key(group, who)?;
    unwrap_with(&key, &who.provider, wrapped)
}
