//! THE wiring test (Phase 7, the make-or-break for option C here): drive
//! `group-core` through the existing `Transport` port and prove a message sent
//! by one peer reaches another's model over the in-proc bus. If this cannot be
//! made to pass, the architecture is wrong — not just a component.

use croft_chat_cli::runtime::{apply, pump};
use croft_chat_cli::transport::{InProcBus, Topic};
use croft_chat_cli::GROUP_TOPIC;
use group_core::{Intent, Model};

#[test]
fn a_message_sent_by_one_peer_reaches_another_over_the_transport() {
    let bus = InProcBus::new();
    let mut alice = bus.attach();
    let mut bob = bus.attach();
    let topic = Topic::new(GROUP_TOPIC);

    // Both peers join (each subscribes to the topic).
    let mut alice_model = apply(Model::new(), Intent::JoinGroup, &mut alice, &topic);
    let mut bob_model = apply(Model::new(), Intent::JoinGroup, &mut bob, &topic);
    assert!(alice_model.joined, "alice joined");
    assert!(bob_model.joined, "bob joined");

    // Alice sends.
    alice_model = apply(
        alice_model,
        Intent::SendMessage {
            text: "hi".to_string(),
        },
        &mut alice,
        &topic,
    );

    // Checkpoint: alice has her own message BEFORE bob pumps (update fired and
    // the optimistic append happened). Localizes a failure to the send path.
    assert_eq!(
        alice_model.messages.len(),
        1,
        "alice optimistically appended her own message"
    );
    assert_eq!(alice_model.messages[0].text, "hi");

    // Bob pumps: drains the bus, feeds each frame back as FrameReceived.
    bob_model = pump(bob_model, &mut bob, &topic);

    // The gate: bob received exactly what alice sent, over the real port. A
    // dropped/garbled frame would leave bob empty (and eprintln a FrameDropped),
    // so a non-empty, matching message also proves the happy path dropped
    // nothing.
    assert_eq!(
        bob_model.messages.len(),
        1,
        "bob received exactly one message over the bus"
    );
    assert_eq!(
        bob_model.messages[0], alice_model.messages[0],
        "bob's received message matches the one alice sent (sender + text)"
    );
}
