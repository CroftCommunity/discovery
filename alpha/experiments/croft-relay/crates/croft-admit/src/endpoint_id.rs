//! The connecting endpoint's identity.
//!
//! An `EndpointId` is a 32-byte ed25519 public key. The relay authenticates
//! this key cryptographically during attach (it is the key that terminates the
//! connection), so by the time our admission code sees it, the id is *proven*,
//! not asserted. That is the hinge the whole design turns on: the token's `sub`
//! claim is compared against this authenticated id, so a stolen token cannot be
//! replayed from a different endpoint (see `token::verify`).
//!
//! On the wire the relay presents it hex-encoded in the `X-Iroh-Endpoint-Id`
//! header (Phase 1 HTTP hook) or we read it from the authenticated attach
//! (Phase 2 embed). Both forms decode through here.

use std::fmt;

/// A 32-byte ed25519 public key identifying an iroh endpoint.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct EndpointId([u8; 32]);

/// Why a hex string failed to parse as an `EndpointId`.
#[derive(Debug, PartialEq, Eq)]
pub enum IdError {
    /// Not 64 hex chars / 32 bytes once decoded.
    BadLength(usize),
    /// Non-hex characters.
    NotHex,
}

impl EndpointId {
    /// Construct from raw 32 bytes (e.g. the relay's authenticated attach key).
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        EndpointId(bytes)
    }

    /// Parse the hex wire form (`X-Iroh-Endpoint-Id`). Rejects anything that is
    /// not exactly 32 bytes of hex — a malformed header is a hard deny, never a
    /// best-effort guess.
    pub fn from_hex(s: &str) -> Result<Self, IdError> {
        let raw = hex::decode(s).map_err(|_| IdError::NotHex)?;
        let arr: [u8; 32] = raw
            .as_slice()
            .try_into()
            .map_err(|_| IdError::BadLength(raw.len()))?;
        Ok(EndpointId(arr))
    }

    /// Lowercase hex, the canonical form we mint into token `sub` claims so the
    /// comparison in `token::verify` is a plain string equality.
    pub fn to_hex(&self) -> String {
        hex::encode(self.0)
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl fmt::Display for EndpointId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_hex())
    }
}

impl fmt::Debug for EndpointId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EndpointId({})", self.to_hex())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_roundtrips_and_accessors_hold() {
        let mut raw = [0u8; 32];
        raw[0] = 0xde;
        raw[31] = 0xad;
        let id = EndpointId::from_bytes(raw);
        let hex = id.to_hex();

        // Wire form is exactly the lowercase hex of the 32 bytes.
        assert_eq!(hex.len(), 64);
        assert!(hex.starts_with("de"));
        assert!(hex.ends_with("ad"));
        assert_eq!(EndpointId::from_hex(&hex), Ok(id));
        assert_eq!(id.as_bytes(), &raw);

        // Display is the hex; Debug wraps it. (Pins the formatters so a
        // no-op fmt mutant is caught.)
        assert_eq!(format!("{id}"), hex);
        assert_eq!(format!("{id:?}"), format!("EndpointId({hex})"));
    }

    #[test]
    fn bad_hex_is_rejected_not_guessed() {
        assert_eq!(EndpointId::from_hex("zz"), Err(IdError::NotHex));
        assert_eq!(EndpointId::from_hex("dead"), Err(IdError::BadLength(2)));
    }
}
