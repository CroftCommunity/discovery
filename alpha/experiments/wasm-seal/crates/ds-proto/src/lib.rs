//! `ds-proto` — RUN-19 P4: the content-blind DS wire protocol.
//!
//! One **bidirectional stream per request** (the WebTransport idiom a browser
//! client would use): `u32-LE length || JSON header`, then
//! `u32-LE length || payload bytes` — both directions. The payload is opaque
//! to this crate and to the server: ciphertext in, ciphertext out.
//!
//! RED stub: the codec reports [`ProtoError::Unbuilt`].

#![warn(missing_docs)]

use serde::{Deserialize, Serialize};
use tokio::io::{AsyncRead, AsyncWrite};

/// A request header (the payload, if any, follows as the second frame).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum Request {
    /// Store one opaque blob at `(group, seq)` as `member`.
    Put {
        /// Scope id (opaque to the DS).
        group: String,
        /// The writer's delivery cursor for this blob — a DS-assigned-shape
        /// ordinal, never an order claim (covert-clock rule).
        seq: u64,
        /// The caller's member id (see the module-level auth note).
        member: String,
    },
    /// Offer every stored blob of `group` with `seq >= from_seq` to `member`.
    Fetch {
        /// Scope id.
        group: String,
        /// Lowest delivery cursor wanted.
        from_seq: u64,
        /// The caller's member id.
        member: String,
    },
    /// Admit `member` to the group's offer roster (creates the group when
    /// first seen). Roster-admin authentication is the RUN-14 EXP-A
    /// service-auth seam — NOT rebuilt here (README, named non-goal).
    RosterAdd {
        /// Scope id.
        group: String,
        /// Member to admit.
        member: String,
    },
    /// Remove `member` from the offer roster (offering stops; nothing about
    /// already-held ciphertext or keys changes — offering vs reading).
    RosterRemove {
        /// Scope id.
        group: String,
        /// Member to remove.
        member: String,
    },
}

/// A response header (blob payloads, if any, follow as one length-prefixed
/// frame each, in `seqs` order).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    /// Whether the request was served.
    pub ok: bool,
    /// On refusal: ALWAYS the identical flat string (no existence, length,
    /// or membership leak — the EXP-B one-refusal rule).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Delivery cursors of the offered blobs (fetch only).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub seqs: Vec<u64>,
}

/// The one flat refusal string (compared byte-for-byte in the tests).
pub const REFUSED: &str = "refused";

/// Why a protocol read/write failed.
#[derive(Debug, thiserror::Error)]
pub enum ProtoError {
    /// RED: not yet implemented.
    #[error("unbuilt: P4 red")]
    Unbuilt,
    /// I/O failure on the stream.
    #[error("io: {0}")]
    Io(String),
    /// The header was not valid JSON / valid shape.
    #[error("codec: {0}")]
    Codec(String),
    /// A frame exceeded the sanity bound.
    #[error("frame too large: {0} bytes")]
    TooLarge(u64),
}

/// Write one `u32-LE length || bytes` frame.
///
/// # Errors
/// [`ProtoError::Unbuilt`] in RED.
pub async fn write_frame<W: AsyncWrite + Unpin>(
    _w: &mut W,
    _bytes: &[u8],
) -> Result<(), ProtoError> {
    Err(ProtoError::Unbuilt)
}

/// Read one `u32-LE length || bytes` frame (bounded).
///
/// # Errors
/// [`ProtoError::Unbuilt`] in RED.
pub async fn read_frame<R: AsyncRead + Unpin>(_r: &mut R) -> Result<Vec<u8>, ProtoError> {
    Err(ProtoError::Unbuilt)
}

/// Serialize + write a JSON header frame.
///
/// # Errors
/// [`ProtoError::Unbuilt`] in RED.
pub async fn write_json<W: AsyncWrite + Unpin, T: Serialize>(
    _w: &mut W,
    _value: &T,
) -> Result<(), ProtoError> {
    Err(ProtoError::Unbuilt)
}

/// Read + deserialize a JSON header frame.
///
/// # Errors
/// [`ProtoError::Unbuilt`] in RED.
pub async fn read_json<R: AsyncRead + Unpin, T: for<'de> Deserialize<'de>>(
    _r: &mut R,
) -> Result<T, ProtoError> {
    Err(ProtoError::Unbuilt)
}
