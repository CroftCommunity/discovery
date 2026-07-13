//! The wire format: `ChatMessage` <-> opaque bytes. The core authors messages,
//! so it owns their serialization (the chat analog of the feed pond's
//! `bluesky/wire.rs`). Version-tagged (`{ "v": 1, "sender", "text" }`) so L1/L2
//! fields can extend the frame without breaking older readers.

use serde::{Deserialize, Serialize};

use crate::model::ChatMessage;

/// The wire version this build emits and accepts. Bumped when the frame shape
/// changes incompatibly; older fields stay, new readers tolerate new ones.
const WIRE_VERSION: u8 = 1;

/// The on-the-wire frame. A version tag plus the message fields. Kept private:
/// the domain type (`ChatMessage`) does not know about serialization or
/// versioning — that is the wire's concern (honest seam).
#[derive(Serialize, Deserialize)]
struct FrameV1 {
    v: u8,
    sender: String,
    text: String,
}

/// Why an inbound frame could not be decoded. Carries no payload content: a
/// frame can hold user data, so errors (which the shell logs) report only the
/// parse position or version, never input bytes.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum WireError {
    /// The bytes were not a valid framed JSON object. Reports only the parse
    /// position, never the offending content.
    #[error("malformed frame (parse error at line {line}, column {column})")]
    Malformed {
        /// 1-based line of the parse error.
        line: usize,
        /// 1-based column of the parse error.
        column: usize,
    },
    /// The frame parsed but declared a wire version this build does not support.
    #[error("unsupported wire version: {0}")]
    UnsupportedVersion(u8),
}

/// Serialize a message to its opaque wire bytes. Infallible in practice:
/// `FrameV1` is a fixed struct of a `u8` and owned `String`s, which
/// `serde_json` cannot fail to serialize.
///
/// # Panics
/// Never in practice. The internal `expect` guards an invariant — if
/// `serde_json` ever failed to serialize this fixed, key-less-map-free struct
/// it would mean a broken serde, which is an unrecoverable bug, not a runtime
/// condition the caller can handle.
#[must_use]
pub fn serialize(message: &ChatMessage) -> Vec<u8> {
    let frame = FrameV1 {
        v: WIRE_VERSION,
        sender: message.sender.clone(),
        text: message.text.clone(),
    };
    serde_json::to_vec(&frame)
        .expect("serializing a fixed FrameV1 of u8 + owned Strings is infallible")
}

/// Decode opaque wire bytes back into a message. Returns a typed [`WireError`]
/// on malformed input or an unsupported version — never panics, so a hostile
/// or corrupt frame cannot take the receiver down (see Phase 5).
///
/// # Errors
/// - [`WireError::Malformed`] if the bytes are not a valid framed JSON object.
/// - [`WireError::UnsupportedVersion`] if the frame's `v` is not supported.
pub fn deserialize(bytes: &[u8]) -> Result<ChatMessage, WireError> {
    let frame: FrameV1 = serde_json::from_slice(bytes).map_err(|e| WireError::Malformed {
        line: e.line(),
        column: e.column(),
    })?;
    if frame.v != WIRE_VERSION {
        return Err(WireError::UnsupportedVersion(frame.v));
    }
    Ok(ChatMessage {
        sender: frame.sender,
        text: frame.text,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::ChatMessage;

    #[test]
    fn chat_message_round_trips_through_bytes() {
        let msg = ChatMessage {
            sender: "alice".to_string(),
            text: "hi bob".to_string(),
        };
        let bytes = serialize(&msg);
        assert_eq!(
            deserialize(&bytes),
            Ok(msg),
            "a serialized message deserializes back to itself"
        );
    }

    #[test]
    fn malformed_bytes_fail_as_a_typed_error_not_a_panic() {
        let result = deserialize(b"not-json");
        assert!(
            matches!(result, Err(WireError::Malformed { .. })),
            "garbage bytes yield a typed Malformed error, got {result:?}"
        );
    }
}
