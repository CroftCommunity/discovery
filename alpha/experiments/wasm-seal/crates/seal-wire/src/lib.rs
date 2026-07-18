//! `seal-wire` — RUN-19: KeyPackage across the process/build boundary.
//!
//! The one artifact the P2 cross-build goldens exchange that `group-seal`
//! holds as a typed object rather than bytes. Framed as an `MlsMessage` —
//! the same `tls_codec` boundary every other artifact in the stack crosses —
//! and **validated** on the way in (signature, ciphersuite, lifetime), never
//! trusted raw.

#![warn(missing_docs)]

use lineage_mls::KeyPackage;
use openmls::prelude::tls_codec::{Deserialize as _, Serialize as _};
use openmls::prelude::{MlsMessageBodyIn, MlsMessageIn, MlsMessageOut, ProtocolVersion};
use openmls_rust_crypto::OpenMlsRustCrypto;
use openmls_traits::OpenMlsProvider as _;

/// Why a wire (de)serialization failed.
#[derive(Debug, thiserror::Error)]
pub enum WireError {
    /// The bytes were not a valid MLS message carrying a valid key package.
    #[error("wire: {0}")]
    Wire(String),
}

fn wire_err<E: std::fmt::Debug>(e: E) -> WireError {
    WireError::Wire(format!("{e:?}"))
}

/// Serialize a key package for the wire (an `MlsMessage` framing).
///
/// # Errors
/// [`WireError::Wire`] if tls serialization fails.
pub fn key_package_to_bytes(kp: &KeyPackage) -> Result<Vec<u8>, WireError> {
    MlsMessageOut::from(kp.clone())
        .tls_serialize_detached()
        .map_err(wire_err)
}

/// Deserialize and **validate** a key package received over the wire.
///
/// # Errors
/// [`WireError::Wire`] if the bytes are not an MLS message, are not a key
/// package, or the key package fails validation.
pub fn key_package_from_bytes(bytes: &[u8]) -> Result<KeyPackage, WireError> {
    let msg = MlsMessageIn::tls_deserialize_exact(bytes).map_err(wire_err)?;
    match msg.extract() {
        MlsMessageBodyIn::KeyPackage(kp_in) => {
            let provider = OpenMlsRustCrypto::default();
            kp_in
                .validate(provider.crypto(), ProtocolVersion::Mls10)
                .map_err(wire_err)
        }
        other => Err(WireError::Wire(format!(
            "expected key package, got {other:?}"
        ))),
    }
}
