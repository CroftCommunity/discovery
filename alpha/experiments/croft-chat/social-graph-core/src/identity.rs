//! Identity: an ed25519 keypair plus the substrate id derivations.
//!
//! A device's [`DeviceId`] *is* its ed25519 verifying-key bytes, which makes the
//! [`crate::crypto::Ed25519Verifier`] stateless (it reconstructs the public key
//! straight from the id). The [`PrincipalId`] is derived deterministically from
//! the device key for this single-device-per-principal demo.

use ed25519_dalek::SigningKey;
use local_storage_projection::traits::{DeviceId, PrincipalId};
use sha2::{Digest, Sha256};

/// Domain-separation tag for principal-id derivation. Bumping this changes every
/// derived principal id, so it is versioned.
const PRINCIPAL_DOMAIN: &[u8] = b"croft-principal-v1";

/// A signing identity: one ed25519 keypair.
///
/// Holds secret key material (`SigningKey`), so it is deliberately **not**
/// `Debug`/`Clone`/`Serialize` — the secret never leaves this type except as a
/// [`crate::crypto::Ed25519Signer`] (which the substrate consumes per call). The
/// underlying `SigningKey` zeroizes on drop.
pub struct Identity {
    signing_key: SigningKey,
}

impl Identity {
    /// Create an identity from operating-system entropy.
    ///
    /// # Errors
    /// Returns the underlying `getrandom` error if the OS RNG is unavailable.
    pub fn generate() -> Result<Self, getrandom::Error> {
        let mut seed = [0u8; 32];
        getrandom::getrandom(&mut seed)?;
        let identity = Self::from_seed(seed);
        seed.fill(0); // do not leave the seed lying on the stack
        Ok(identity)
    }

    /// Create a deterministic identity from a 32-byte seed.
    ///
    /// Used for repeatable demos and tests; a real deployment uses
    /// [`Identity::generate`].
    #[must_use]
    pub fn from_seed(seed: [u8; 32]) -> Self {
        Self {
            signing_key: SigningKey::from_bytes(&seed),
        }
    }

    /// The device id: the ed25519 verifying-key bytes.
    #[must_use]
    pub fn device_id(&self) -> DeviceId {
        DeviceId(self.signing_key.verifying_key().to_bytes())
    }

    /// The principal id: `SHA-256(PRINCIPAL_DOMAIN || device_pubkey)`.
    ///
    /// Deterministic for a given device key (single-device principal for this
    /// demo); the [`crate::Session`] registers the (device, principal) pair with
    /// the credential resolver.
    #[must_use]
    pub fn principal_id(&self) -> PrincipalId {
        let mut hasher = Sha256::new();
        hasher.update(PRINCIPAL_DOMAIN);
        hasher.update(self.signing_key.verifying_key().to_bytes());
        let digest = hasher.finalize();
        let mut out = [0u8; 32];
        out.copy_from_slice(&digest);
        PrincipalId(out)
    }

    /// Borrow the signing key to construct a signer adapter.
    pub(crate) fn signing_key(&self) -> &SigningKey {
        &self.signing_key
    }
}

#[cfg(test)]
mod tests {
    use super::Identity;

    #[test]
    fn from_seed_is_deterministic() {
        let a = Identity::from_seed([7u8; 32]);
        let b = Identity::from_seed([7u8; 32]);
        assert_eq!(a.device_id(), b.device_id(), "same seed -> same device id");
        assert_eq!(
            a.principal_id(),
            b.principal_id(),
            "same seed -> same principal id"
        );
    }

    #[test]
    fn distinct_seeds_give_distinct_ids() {
        let a = Identity::from_seed([1u8; 32]);
        let b = Identity::from_seed([2u8; 32]);
        assert_ne!(a.device_id(), b.device_id());
        assert_ne!(a.principal_id(), b.principal_id());
    }

    #[test]
    fn principal_differs_from_device() {
        // The principal id is a domain-separated hash, never equal to the raw
        // device key bytes.
        let id = Identity::from_seed([9u8; 32]);
        assert_ne!(id.device_id().0, id.principal_id().0);
    }

    #[test]
    fn generate_produces_distinct_identities() {
        let a = Identity::generate().expect("os rng");
        let b = Identity::generate().expect("os rng");
        assert_ne!(a.device_id(), b.device_id(), "fresh entropy each time");
    }
}
