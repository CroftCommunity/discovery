//! The shell runtime: `apply` (one intent through the core, performing its
//! effects) and `pump` (the explicit synchronous tick that drains inbound
//! frames back into the core). This is the chat analog of the feed pond's
//! `drive`, with one structural difference — see `apply`.

use group_core::{update, Intent, Model};

use crate::effects::perform_effect;
use crate::transport::{Topic, Transport};

/// Apply one intent: run the core's `update` once, then perform each returned
/// effect against the transport. A **single pass**, not a drive-to-quiescence
/// loop — unlike the feed pond's `drive`, no effect here yields a follow-up
/// intent (subscribe/publish are fire-and-forget; inbound arrives via `pump`).
#[must_use]
pub fn apply<T: Transport>(
    model: Model,
    intent: Intent,
    transport: &mut T,
    topic: &Topic,
) -> Model {
    let (next, effects) = update(model, intent);
    for effect in &effects {
        perform_effect(effect, transport, topic);
    }
    next
}

/// The explicit synchronous tick: drain every frame the transport has received
/// and feed each back into the core as a `FrameReceived` intent. The shell does
/// the draining (it is I/O); the core decodes. The real iroh adapter (L5)
/// supplies the real event loop that calls this.
#[must_use]
pub fn pump<T: Transport>(model: Model, transport: &mut T, topic: &Topic) -> Model {
    transport.drain().into_iter().fold(model, |model, frame| {
        apply(
            model,
            Intent::FrameReceived {
                bytes: frame.bytes().to_vec(),
            },
            transport,
            topic,
        )
    })
}
