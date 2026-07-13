//! The payload-blind transport port.
//!
//! Lifted from `croft-group`'s `croft-chat-cli` (DECISION 1: the port lives in
//! the shell; the core emits effect data). The port carries opaque `Frame`s on
//! named `Topic`s and knows nothing about assertions — so the local shared-dir
//! adapter (P6) and the iroh-gossip adapter (P16) are drop-in substitutes.

/// A pub/sub topic. Peers in one conversation share a topic string.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Topic(pub String);

/// An opaque wire frame (e.g. a serialized assertion). The transport never
/// inspects it. `Ord` is provided for deterministic set-comparison in tests.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame(pub Vec<u8>);

/// A pub/sub transport between peers.
pub trait Transport {
    /// Subscribe this peer to `topic`. Idempotent.
    fn subscribe(&mut self, topic: &Topic);

    /// Publish `frame` to every *other* peer subscribed to `topic` (no
    /// self-echo).
    fn publish(&mut self, topic: &Topic, frame: Frame);

    /// Remove and return the frames received on subscribed topics since the last
    /// drain. Order is not guaranteed — callers that need ordering impose it.
    fn drain(&mut self) -> Vec<Frame>;
}
