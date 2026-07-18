//! `ds-client` — RUN-19 P4: native WebTransport client for the blind DS.
//!
//! RED stub: every operation reports [`ClientError::Unbuilt`].

#![warn(missing_docs)]

/// Why a DS client operation failed.
#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    /// RED: not yet implemented.
    #[error("unbuilt: P4 red")]
    Unbuilt,
    /// The QUIC/WebTransport session could not be established (includes the
    /// wrong-cert-hash refusal — the dev-trust negative case).
    #[error("connect: {0}")]
    Connect(String),
    /// The server refused the request (the one flat refusal).
    #[error("refused")]
    Refused,
    /// Protocol-level failure.
    #[error("proto: {0}")]
    Proto(String),
}

/// A WebTransport session to the blind DS.
pub struct DsClient;

impl DsClient {
    /// Connect to `url` trusting exactly the pinned SHA-256 cert hash (hex) —
    /// the browser `serverCertificateHashes` dev-trust mechanism.
    ///
    /// # Errors
    /// [`ClientError::Unbuilt`] in RED.
    pub async fn connect(_url: &str, _cert_hash_hex: &str) -> Result<Self, ClientError> {
        Err(ClientError::Unbuilt)
    }

    /// Admit `member` to the group's offer roster.
    ///
    /// # Errors
    /// [`ClientError::Unbuilt`] in RED.
    pub async fn roster_add(&self, _group: &str, _member: &str) -> Result<(), ClientError> {
        Err(ClientError::Unbuilt)
    }

    /// Remove `member` from the offer roster.
    ///
    /// # Errors
    /// [`ClientError::Unbuilt`] in RED.
    pub async fn roster_remove(&self, _group: &str, _member: &str) -> Result<(), ClientError> {
        Err(ClientError::Unbuilt)
    }

    /// Store one opaque blob at `(group, seq)` as `member`.
    ///
    /// # Errors
    /// [`ClientError::Unbuilt`] in RED.
    pub async fn put(
        &self,
        _group: &str,
        _seq: u64,
        _member: &str,
        _blob: &[u8],
    ) -> Result<(), ClientError> {
        Err(ClientError::Unbuilt)
    }

    /// Fetch every offered blob of `group` from `from_seq`, as `member`.
    ///
    /// # Errors
    /// [`ClientError::Unbuilt`] in RED.
    pub async fn fetch(
        &self,
        _group: &str,
        _from_seq: u64,
        _member: &str,
    ) -> Result<Vec<(u64, Vec<u8>)>, ClientError> {
        Err(ClientError::Unbuilt)
    }
}
