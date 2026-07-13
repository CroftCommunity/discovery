//! Effects: what the core asks the shell to do, expressed as data.
//! The core emits these and returns; it never performs them and never holds or
//! calls the `Transport` port (DECISION 1 — the port lives in the shell).

/// What the core asks the shell to do. Topic-free by design: the core stays
/// transport-agnostic and the shell supplies the fixed `Topic` (Phase 7). A
/// `FrameDropped` diagnostic variant is added in Phase 5 when the receive arm
/// needs it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Effect {
    /// Subscribe to the group's topic (idempotent at the transport).
    Subscribe,
    /// Publish this opaque frame payload to other subscribers.
    Publish {
        /// The serialized message bytes to put on the wire.
        bytes: Vec<u8>,
    },
    /// An inbound frame could not be decoded and was dropped. The core is pure
    /// and cannot do I/O, so it surfaces the drop as effect data; the shell
    /// logs it (Phase 7). This is the observability seam that makes a
    /// hostile/corrupt frame visible without taking the receiver down.
    FrameDropped {
        /// Why the frame was dropped (carries no payload content — see
        /// `WireError`), suitable for the shell to log.
        reason: String,
    },
}
