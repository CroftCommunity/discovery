//! The blinded roster (RUN-16 P6, blind half).
//!
//! A members-only salt is steward state. The roster is published as a set of
//! `hash(salt ‖ DID)` commitments — public, but opaque: linking a known DID to
//! the roster requires the salt, so an outsider with the whole public chain
//! cannot tell who is on it. A member, holding their DID and the salt (their
//! individual attestation), recomputes their own commitment and proves their
//! membership. Rotating the salt republishes every commitment and retires the
//! old ones; removing a member and rotating gives forward-blindness — the past
//! member, holding only the old salt, cannot read the post-removal roster.

use sha2::{Digest, Sha256};

/// A commitment: `sha256(salt ‖ DID)`. Domain-separated from the message-leaf
/// hash so a commitment can never be confused with an envelope identity.
#[must_use]
pub fn commit(did: &str, salt: &[u8]) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(b"tier-proof/blinded-roster/v1");
    h.update((salt.len() as u64).to_be_bytes());
    h.update(salt);
    h.update(did.as_bytes());
    h.finalize().into()
}

/// Does `did` under `salt` appear among `commitments`? True only for a caller
/// holding the real salt (a member); an outsider without it cannot match.
#[must_use]
pub fn links_with_salt(commitments: &[[u8; 32]], did: &str, salt: &[u8]) -> bool {
    let c = commit(did, salt);
    commitments.contains(&c)
}

/// A published blinded roster: only the commitments are public.
#[derive(Debug, Clone)]
pub struct BlindedRoster {
    commitments: Vec<[u8; 32]>,
}

impl BlindedRoster {
    /// Publish commitments for `dids` under `salt`.
    #[must_use]
    pub fn publish(dids: &[String], salt: &[u8]) -> Self {
        Self {
            commitments: dids.iter().map(|d| commit(d, salt)).collect(),
        }
    }

    /// The public commitments (all an outsider ever sees).
    #[must_use]
    pub fn commitments(&self) -> &[[u8; 32]] {
        &self.commitments
    }

    /// Rotate the salt (and optionally the member set), republishing every
    /// commitment. The returned roster shares no commitment with the old one.
    #[must_use]
    pub fn rotate(&self, dids: &[String], new_salt: &[u8]) -> Self {
        Self::publish(dids, new_salt)
    }
}
