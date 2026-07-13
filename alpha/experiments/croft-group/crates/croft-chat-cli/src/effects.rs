//! Performing effects. This is the shell's job and the **only** place the
//! `Transport` port is held and called (DECISION 1). The real iroh adapter
//! will plug in behind the same port (L5).

use group_core::Effect;

use crate::transport::{Frame, Topic, Transport};

/// Perform one effect against the transport. Unlike the feed pond's
/// `perform_effect -> Intent`, these effects are fire-and-forget: the port's
/// `subscribe`/`publish` return nothing, and `FrameDropped` is a log — so there
/// is no follow-up intent. Inbound arrives only via `pump`/`drain`, never as an
/// effect follow-up.
///
/// The `topic` is supplied by the shell: the core's effects are topic-free, so
/// the core stays transport-agnostic (DECISION 1).
pub fn perform_effect<T: Transport>(effect: &Effect, transport: &mut T, topic: &Topic) {
    match effect {
        Effect::Subscribe => transport.subscribe(topic),
        Effect::Publish { bytes } => transport.publish(topic, Frame::new(bytes.clone())),
        // The I/O side of the core's drop signal: the core (pure) can't log, so
        // it emits FrameDropped and the shell writes it to stderr. Fires only on
        // hostile/corrupt input; carries no payload content (see WireError).
        Effect::FrameDropped { reason } => {
            eprintln!("croft-chat: dropped malformed frame: {reason}");
        }
    }
}
