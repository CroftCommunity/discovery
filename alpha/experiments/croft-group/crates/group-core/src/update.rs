//! The update function: `(Model, Intent) -> (Model, Vec<Effect>)`.
//!
//! Pure and synchronous. No async, no I/O, no clock. This signature is
//! load-bearing for DECISION 1: an awaited port call could not return from it,
//! which is exactly why the port lives in the shell, not here.

use crate::effect::Effect;
use crate::intent::Intent;
use crate::model::{ChatMessage, Model};
use crate::wire;

/// The local peer's sender handle. A hardcoded PLACEHOLDER for this slice —
/// real identity (DID / lineage) replaces it in L1. The `Intent::SendMessage`
/// carries no sender and `Model` holds no identity, so the core stamps every
/// outgoing message with this constant. (The advisory's "alice/bob" two-identity
/// flavor is an L1 concern; it is not reachable through the current signatures.)
const LOCAL_SENDER: &str = "alice";

/// Apply one intent to the model, returning the next model and the effects the
/// shell should perform. Pure and exhaustive over every [`Intent`]. Takes
/// `intent` by value (the functional-core `(Model, Intent) -> (Model, Vec<Effect>)`
/// contract); the send/receive arms consume its payload.
#[must_use]
pub fn update(model: Model, intent: Intent) -> (Model, Vec<Effect>) {
    match intent {
        // Joining a fresh group requests a subscribe; re-joining is a no-op
        // (the transport's subscribe is idempotent, but we also avoid a
        // redundant effect).
        Intent::JoinGroup => {
            if model.joined {
                (model, vec![])
            } else {
                (
                    Model {
                        joined: true,
                        ..model
                    },
                    vec![Effect::Subscribe],
                )
            }
        }
        // Sending optimistically appends our own message (the transport does
        // not self-echo, so there is no double-append) and asks the shell to
        // publish its wire bytes.
        Intent::SendMessage { text } => {
            let Model {
                mut messages,
                joined,
            } = model;
            let message = ChatMessage {
                sender: LOCAL_SENDER.to_string(),
                text,
            };
            let bytes = wire::serialize(&message);
            messages.push(message);
            (Model { messages, joined }, vec![Effect::Publish { bytes }])
        }

        // A frame drained from the transport. A valid one appends silently; a
        // malformed one (hostile/corrupt sender) is dropped without changing
        // state, surfacing an observable FrameDropped — never a panic.
        Intent::FrameReceived { bytes } => match wire::deserialize(&bytes) {
            Ok(message) => {
                let Model {
                    mut messages,
                    joined,
                } = model;
                messages.push(message);
                (Model { messages, joined }, vec![])
            }
            Err(error) => (
                model,
                vec![Effect::FrameDropped {
                    reason: error.to_string(),
                }],
            ),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{ChatMessage, Model};

    #[test]
    fn joining_a_fresh_group_marks_joined_and_subscribes() {
        let (model, effects) = update(Model::new(), Intent::JoinGroup);
        assert!(model.joined, "joining marks the model joined");
        assert_eq!(
            effects,
            vec![Effect::Subscribe],
            "joining a fresh group requests exactly one subscribe"
        );
    }

    #[test]
    fn rejoining_an_already_joined_group_is_a_noop() {
        let (joined, _) = update(Model::new(), Intent::JoinGroup);
        let (again, effects) = update(joined.clone(), Intent::JoinGroup);
        assert_eq!(again, joined, "re-joining does not change the model");
        assert!(
            effects.is_empty(),
            "re-joining an already-joined group emits no effect"
        );
    }

    fn joined_model() -> Model {
        update(Model::new(), Intent::JoinGroup).0
    }

    #[test]
    fn sending_appends_own_message_and_emits_one_publish_that_round_trips() {
        let (model, effects) = update(
            joined_model(),
            Intent::SendMessage {
                text: "hi".to_string(),
            },
        );
        assert_eq!(
            model.messages.len(),
            1,
            "the sender optimistically appends its own message (the port does not self-echo)"
        );
        assert_eq!(model.messages[0].text, "hi");
        assert_eq!(effects.len(), 1, "sending emits exactly one effect");
        match &effects[0] {
            Effect::Publish { bytes } => assert_eq!(
                crate::wire::deserialize(bytes),
                Ok(model.messages[0].clone()),
                "the published bytes deserialize back to the appended message"
            ),
            other => panic!("expected a Publish effect, got {other:?}"),
        }
    }

    #[test]
    fn receiving_a_valid_frame_appends_it_with_no_effect() {
        let remote = ChatMessage {
            sender: "bob".to_string(),
            text: "hello".to_string(),
        };
        let bytes = crate::wire::serialize(&remote);
        let (model, effects) = update(joined_model(), Intent::FrameReceived { bytes });
        assert_eq!(
            model.messages,
            vec![remote],
            "a valid inbound frame is appended verbatim"
        );
        assert!(
            effects.is_empty(),
            "a valid inbound frame is silent — no effect, so a stray effect fails the test"
        );
    }

    #[test]
    fn malformed_frames_are_dropped_observably_and_change_nothing() {
        // The corrupt/hostile-sender case: a peer can put any bytes on the wire.
        // Each must leave state byte-identical and surface exactly one observable
        // FrameDropped — never panic, never append.
        let before = joined_model();
        for junk in [b"".to_vec(), b"{".to_vec(), vec![0xff, 0xff]] {
            let (model, effects) = update(
                before.clone(),
                Intent::FrameReceived {
                    bytes: junk.clone(),
                },
            );
            assert_eq!(
                model, before,
                "a malformed frame leaves the model byte-identical (junk = {junk:?})"
            );
            assert_eq!(
                effects.len(),
                1,
                "a dropped frame surfaces exactly one effect (junk = {junk:?})"
            );
            match &effects[0] {
                Effect::FrameDropped { reason } => {
                    assert!(!reason.is_empty(), "the drop carries a non-empty reason");
                }
                other => panic!("expected FrameDropped, got {other:?}"),
            }
        }
    }
}
