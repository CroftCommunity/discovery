//! `seal-wire` — RUN-19: KeyPackage across the process/build boundary.
//!
//! RED stub: both directions report [`WireError::Unbuilt`].

#![warn(missing_docs)]

use lineage_mls::KeyPackage;

/// Why a wire (de)serialization failed.
#[derive(Debug, thiserror::Error)]
pub enum WireError {
    /// RED: not yet implemented.
    #[error("unbuilt: P2 red")]
    Unbuilt,
    /// The bytes were not a valid MLS message carrying a key package.
    #[error("wire: {0}")]
    Wire(String),
}

/// Serialize a key package for the wire (an `MlsMessage` framing, the same
/// `tls_codec` boundary every other artifact in the stack crosses).
///
/// # Errors
/// [`WireError::Unbuilt`] in RED.
pub fn key_package_to_bytes(_kp: &KeyPackage) -> Result<Vec<u8>, WireError> {
    Err(WireError::Unbuilt)
}

/// Deserialize and validate a key package received over the wire.
///
/// # Errors
/// [`WireError::Unbuilt`] in RED.
pub fn key_package_from_bytes(_bytes: &[u8]) -> Result<KeyPackage, WireError> {
    Err(WireError::Unbuilt)
}
