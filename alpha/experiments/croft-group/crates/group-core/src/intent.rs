//! Intents: small descriptions of what happened or what the user wants.
//! They come *in* across the boundary; they are not function calls into logic.

/// What happened or what the user wants. Fed into [`crate::update::update`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Intent {
    /// The user joined the group (subscribe to its topic).
    JoinGroup,
    /// The user sent a message with this body.
    SendMessage {
        /// The message body to send.
        text: String,
    },
    /// An opaque inbound frame was drained from the transport by the shell and
    /// handed back to the core to decode. The core owns the wire format; the
    /// transport never inspects these bytes.
    FrameReceived {
        /// The opaque frame payload.
        bytes: Vec<u8>,
    },
}
