//! B2 — the rkey encoding: `<hashed-subspace-prefix>-<zero-padded counter>`.
//!
//! Matchup row 2: MST keys are byte strings sorted lexically (repository
//! spec), record keys allow `A-Za-z0-9` + `.-_:~`, 1–512 chars, ≤80 practice
//! (record-key spec). A fixed-width counter makes the repo tree itself the
//! sorted (subspace, counter) reconciliation index.
//!
//! **The padding width is a byte choice, `[gates-release]`-adjacent, NOT
//! pinned here.** The spike-local width is 20 decimal digits (the u64
//! ceiling: `u64::MAX` = 18446744073709551615, 20 digits), giving
//! 16 hex chars + `-` + 20 digits = 37 chars, under the 80-char practice
//! bound. The subspace prefix is the first 8 bytes of the hashed subspace id
//! rendered lowercase hex — also a spike-local choice (a 64-bit hash prefix;
//! collision odds are negligible at spike scale and the full id rides in the
//! record body, which stays authoritative).
//!
//! The ordering rider applies (lib.rs): rkey sort is a delivery artifact.
//! B2 proves the *index* property (lexicographic ≡ (subspace, counter)),
//! never an ordering-for-fold property — that is B4's rejection.

use crate::envelope::Digest;

/// The rkey prefix for a subspace: first 8 bytes of the hashed id, lowercase
/// hex (16 chars, alphanumeric — inside the record-key alphabet).
pub fn rkey_prefix(subspace: &Digest) -> String {
    hex::encode(&subspace[..8])
}

/// The rkey for (subspace, counter): fixed-width zero-padded counter, so
/// lexicographic rkey order equals the (subspace, counter) total order.
/// (The B2 red run captured the unpadded form failing at the 9→10 boundary
/// before this padded form went green; width is spike-local, see module doc.)
pub fn entry_rkey(subspace: &Digest, counter: u64) -> String {
    format!("{}-{:020}", rkey_prefix(subspace), counter)
}
