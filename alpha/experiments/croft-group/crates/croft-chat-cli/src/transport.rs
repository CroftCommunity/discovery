//! The [`Transport`] port and its in-process, deterministic adapter ([`InProcBus`]).
//!
//! Faithful to the relay design: the transport routes opaque [`Frame`]s by [`Topic`] and
//! never inspects payload bytes.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// An opaque group topic — the routing key. The transport routes by `Topic` alone and
/// MUST NOT inspect frame payloads (blind-broker faithfulness).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Topic(String);

impl Topic {
    /// Construct a topic from its string form.
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }
}

/// An opaque frame. The transport carries the bytes verbatim; it never reads them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frame(Vec<u8>);

impl Frame {
    /// Wrap bytes as a frame.
    #[must_use]
    pub fn new(bytes: impl Into<Vec<u8>>) -> Self {
        Self(bytes.into())
    }

    /// Borrow the frame's opaque bytes.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.0
    }
}

/// A peer's view of a transport: subscribe to topics, publish opaque frames, and drain the
/// frames received on subscribed topics since the last drain.
pub trait Transport {
    /// Subscribe this peer to `topic`. Idempotent.
    fn subscribe(&mut self, topic: &Topic);

    /// Publish `frame` to every *other* peer subscribed to `topic` (no self-echo).
    fn publish(&mut self, topic: &Topic, frame: Frame);

    /// Remove and return the frames received on subscribed topics since the last drain,
    /// in delivery order.
    fn drain(&mut self) -> Vec<Frame>;
}

/// A deterministic, in-process message bus shared by the peers in one scenario.
#[derive(Default)]
pub struct InProcBus {
    inner: Rc<RefCell<BusState>>,
}

#[derive(Default)]
struct BusState {
    next_id: u64,
    /// Which peers are subscribed to each topic, in subscription order.
    subscriptions: HashMap<Topic, Vec<PeerId>>,
    /// Frames awaiting each peer's next drain, in delivery order.
    inboxes: HashMap<PeerId, Vec<Frame>>,
}

/// An opaque per-peer identity on an [`InProcBus`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PeerId(u64);

impl InProcBus {
    /// Create an empty bus.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Attach a new peer to the bus, returning its handle.
    #[must_use]
    pub fn attach(&self) -> InProcPeer {
        let id = {
            let mut state = self.inner.borrow_mut();
            let id = PeerId(state.next_id);
            state.next_id += 1;
            id
        };
        InProcPeer {
            id,
            bus: Rc::clone(&self.inner),
        }
    }
}

/// A peer handle on an [`InProcBus`].
pub struct InProcPeer {
    id: PeerId,
    bus: Rc<RefCell<BusState>>,
}

impl Transport for InProcPeer {
    fn subscribe(&mut self, topic: &Topic) {
        let mut state = self.bus.borrow_mut();
        let subscribers = state.subscriptions.entry(topic.clone()).or_default();
        if !subscribers.contains(&self.id) {
            subscribers.push(self.id);
        }
    }

    fn publish(&mut self, topic: &Topic, frame: Frame) {
        let mut state = self.bus.borrow_mut();
        // Collect targets first (ending the immutable borrow) so we can then mutate inboxes.
        let targets: Vec<PeerId> = state
            .subscriptions
            .get(topic)
            .into_iter()
            .flatten()
            .copied()
            .filter(|peer| *peer != self.id) // no self-echo
            .collect();
        for target in targets {
            state.inboxes.entry(target).or_default().push(frame.clone());
        }
    }

    fn drain(&mut self) -> Vec<Frame> {
        let mut state = self.bus.borrow_mut();
        state
            .inboxes
            .get_mut(&self.id)
            .map(std::mem::take)
            .unwrap_or_default()
    }
}
