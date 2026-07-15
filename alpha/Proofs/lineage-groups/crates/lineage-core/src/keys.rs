//! Deterministic Ed25519 identities for governance signing.
//!
//! Governance ops and history messages are signed; "did this op meet its
//! threshold" is decided from signed data alone (the security/standing
//! invariant). Ed25519 signatures are deterministic (RFC 8032), so a signed op
//! is bit-reproducible for a fixed key + message — which keeps the whole
//! governance layer reproducible (unlike the MLS key material, see
//! `rng` honesty boundary).
//!
//! Keys here are derived deterministically from a seed so scenarios are
//! repeatable; a real deployment would use hardware/OS entropy.

use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sha2::{Digest, Sha256};

use crate::ids::Did;

/// A 64-byte detached signature. Serialized as hex (serde's blanket array impls
/// stop at length 32, so we (de)serialize through a hex string).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sig(pub [u8; 64]);

impl Sig {
    pub fn to_hex(&self) -> String {
        hex::encode(self.0)
    }
}

impl Serialize for Sig {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&self.to_hex())
    }
}

impl<'de> Deserialize<'de> for Sig {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let hexed = String::deserialize(d)?;
        let bytes = hex::decode(&hexed).map_err(serde::de::Error::custom)?;
        let arr: [u8; 64] = bytes
            .try_into()
            .map_err(|_| serde::de::Error::custom("signature must be 64 bytes"))?;
        Ok(Sig(arr))
    }
}

/// A private signing identity. Holds the secret key; never serialized.
pub struct SigningIdentity {
    did: Did,
    key: SigningKey,
}

impl SigningIdentity {
    /// Derive a deterministic identity for `did` from `seed`. Same (did, seed)
    /// always yields the same key — essential for reproducible scenarios.
    pub fn from_seed(did: Did, seed: u64) -> Self {
        let mut h = Sha256::new();
        h.update(b"lineage-signing-key-v1");
        h.update(did.0.as_bytes());
        h.update(seed.to_le_bytes());
        let digest = h.finalize();
        let mut secret = [0u8; 32];
        secret.copy_from_slice(&digest);
        Self {
            did,
            key: SigningKey::from_bytes(&secret),
        }
    }

    pub fn did(&self) -> &Did {
        &self.did
    }

    /// The public half, safe to share and embed in the lineage.
    pub fn verifying(&self) -> VerifyingIdentity {
        VerifyingIdentity {
            did: self.did.clone(),
            key: self.key.verifying_key(),
        }
    }

    pub fn sign(&self, msg: &[u8]) -> Sig {
        Sig(self.key.sign(msg).to_bytes())
    }
}

/// A public verifying identity. Cheap to clone, serializable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerifyingIdentity {
    did: Did,
    key: VerifyingKey,
}

impl VerifyingIdentity {
    pub fn did(&self) -> &Did {
        &self.did
    }

    /// The raw 32-byte Ed25519 public key. Exposed so a language-neutral
    /// conformance vector can carry the portable key bytes a second
    /// implementation verifies against (it is public material; no secrecy
    /// concern).
    pub fn to_bytes(&self) -> [u8; 32] {
        self.key.to_bytes()
    }

    /// Reconstruct a verifying identity from a DID and the raw 32-byte Ed25519
    /// public key. The inverse of [`to_bytes`](Self::to_bytes): a conformance
    /// runner (or any peer) rebuilds the public key from the portable bytes and
    /// verifies recorded signatures against it. Returns `None` if the bytes are
    /// not a valid Ed25519 point.
    pub fn from_bytes(did: Did, bytes: &[u8; 32]) -> Option<Self> {
        VerifyingKey::from_bytes(bytes).ok().map(|key| Self { did, key })
    }

    /// Verify `sig` over `msg`. Returns false on any malformed signature or
    /// verification failure (never panics on attacker-controlled input).
    pub fn verify(&self, msg: &[u8], sig: &Sig) -> bool {
        let signature = Signature::from_bytes(&sig.0);
        self.key.verify(msg, &signature).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn did(s: &str) -> Did {
        Did::new(s)
    }

    #[test]
    fn signatures_are_deterministic_and_verify() {
        let id = SigningIdentity::from_seed(did("alice"), 1);
        let v = id.verifying();
        let a = id.sign(b"hello");
        let b = id.sign(b"hello");
        assert_eq!(a, b, "Ed25519 signing must be deterministic");
        assert!(v.verify(b"hello", &a));
        assert!(!v.verify(b"tampered", &a));
    }

    #[test]
    fn verifying_key_bytes_are_stable_and_distinct() {
        let a = SigningIdentity::from_seed(did("alice"), 1).verifying();
        let a2 = SigningIdentity::from_seed(did("alice"), 1).verifying();
        let b = SigningIdentity::from_seed(did("bob"), 1).verifying();
        assert_eq!(a.to_bytes().len(), 32);
        assert_eq!(a.to_bytes(), a2.to_bytes(), "same (did,seed) -> same public key");
        assert_ne!(a.to_bytes(), b.to_bytes(), "different did -> different key");
    }

    #[test]
    fn same_seed_same_identity() {
        let a = SigningIdentity::from_seed(did("bob"), 7).verifying();
        let b = SigningIdentity::from_seed(did("bob"), 7).verifying();
        assert_eq!(a, b);
        let c = SigningIdentity::from_seed(did("bob"), 8).verifying();
        assert_ne!(a, c);
    }

    #[test]
    fn verifying_key_round_trips_through_bytes_and_verifies() {
        let id = SigningIdentity::from_seed(did("alice"), 1);
        let original = id.verifying();
        let rebuilt = VerifyingIdentity::from_bytes(did("alice"), &original.to_bytes())
            .expect("valid Ed25519 key bytes round-trip");
        assert_eq!(rebuilt, original);
        // A signature made by the real key verifies against the rebuilt key.
        let sig = id.sign(b"revoke erin");
        assert!(rebuilt.verify(b"revoke erin", &sig));
        assert!(!rebuilt.verify(b"revoke bob", &sig));
    }

    #[test]
    fn from_bytes_against_wrong_key_does_not_verify() {
        // Reconstruct from one signer's bytes; a signature from a *different*
        // signer must not verify against it (the security-relevant property of
        // carrying portable key bytes in a conformance vector).
        let alice = SigningIdentity::from_seed(did("alice"), 1);
        let bob = SigningIdentity::from_seed(did("bob"), 1);
        let alice_pub =
            VerifyingIdentity::from_bytes(did("alice"), &alice.verifying().to_bytes())
                .expect("valid key bytes");
        let bob_sig = bob.sign(b"revoke erin");
        assert!(!alice_pub.verify(b"revoke erin", &bob_sig));
        assert!(alice_pub.verify(b"revoke erin", &alice.sign(b"revoke erin")));
    }

    #[test]
    fn other_identity_cannot_forge() {
        let alice = SigningIdentity::from_seed(did("alice"), 1);
        let mallory = SigningIdentity::from_seed(did("mallory"), 1);
        let sig = mallory.sign(b"transfer");
        // Alice's verifying key must reject Mallory's signature.
        assert!(!alice.verifying().verify(b"transfer", &sig));
    }
}
