//! Behavior of the [`Transport`] port, exercised through the in-process adapter's public API.

use croft_chat_cli::transport::{Frame, InProcBus, Topic, Transport};

/// A frame published to a topic reaches the other peers subscribed to that topic, and no
/// one else: peers on a different topic see nothing, and the sender gets no self-echo.
#[test]
fn frame_reaches_only_other_subscribers_of_its_topic() {
    let bus = InProcBus::new();
    let mut alice = bus.attach();
    let mut bob = bus.attach();
    let mut carol = bus.attach();

    let group = Topic::new("group-g");
    let other = Topic::new("other");
    alice.subscribe(&group);
    bob.subscribe(&group);
    carol.subscribe(&other);

    alice.publish(&group, Frame::new(b"hello".to_vec()));

    // Bob shares the topic → receives it exactly once.
    assert_eq!(bob.drain(), vec![Frame::new(b"hello".to_vec())]);
    // Carol is on a different topic → receives nothing.
    assert_eq!(carol.drain(), Vec::<Frame>::new());
    // The sender does not receive its own publish.
    assert_eq!(alice.drain(), Vec::<Frame>::new());
}

/// Draining is consuming: frames are returned once, then the inbox is empty.
#[test]
fn drain_consumes_received_frames() {
    let bus = InProcBus::new();
    let mut alice = bus.attach();
    let mut bob = bus.attach();

    let group = Topic::new("group-g");
    alice.subscribe(&group);
    bob.subscribe(&group);

    alice.publish(&group, Frame::new(b"one".to_vec()));
    assert_eq!(bob.drain(), vec![Frame::new(b"one".to_vec())]);
    // Second drain with nothing new → empty.
    assert_eq!(bob.drain(), Vec::<Frame>::new());
}
