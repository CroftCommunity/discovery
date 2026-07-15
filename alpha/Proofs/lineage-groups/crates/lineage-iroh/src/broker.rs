//! A blind relay/queue/snapshot broker (invariant probed by E3.4).
//!
//! The broker carries commits and rekeys when peers are not co-present and
//! stores epoch snapshots for recovery — but it is *blind*: this module
//! imports **nothing** from `lineage-mls`, `lineage-core`'s governance, or
//! `lineage-history`. It only ever handles opaque ciphertext addressed to a
//! `NodeId` on a `GroupTopic`. It therefore *cannot* observe plaintext or group
//! membership semantics; blindness is structural, not promised.
//!
//! (As in the field — cf. Signal sealed sender — a relay still observes
//! routing metadata: addresses, topic ids, timing and volume. That residual is
//! acknowledged, not eliminated.)

use std::collections::{BTreeMap, VecDeque};

use crate::transport::{EnvKind, Envelope, GroupTopic, NodeId};

/// An immutable record of exactly what the broker saw, for auditing E3.4.
#[derive(Clone)]
pub struct Observed {
    pub to: NodeId,
    pub topic: GroupTopic,
    pub kind: EnvKind,
    pub bytes_len: usize,
    ciphertext: Vec<u8>,
}

#[derive(Default)]
pub struct BlindBroker {
    /// Keyed by (recipient, topic) so a node that is a member of several groups
    /// never receives another group's envelopes in the same drain.
    queues: BTreeMap<(NodeId, GroupTopic), VecDeque<Envelope>>,
    /// Opaque per-topic snapshot (e.g. a serialized group-info for recovery).
    snapshots: BTreeMap<GroupTopic, Vec<u8>>,
    /// Nodes currently cut off; their mail is held (broker carries it) until heal.
    offline: BTreeMap<NodeId, ()>,
    observed: Vec<Observed>,
}

impl BlindBroker {
    pub fn new() -> Self {
        Self::default()
    }

    /// Accept an opaque envelope for relay. Recorded for the blindness audit.
    pub fn relay(&mut self, env: Envelope) {
        self.observed.push(Observed {
            to: env.to.clone(),
            topic: env.topic,
            kind: env.kind,
            bytes_len: env.ciphertext.len(),
            ciphertext: env.ciphertext.clone(),
        });
        self.queues
            .entry((env.to.clone(), env.topic))
            .or_default()
            .push_back(env);
    }

    /// Cut a node off / restore it. While offline, its queued mail is retained
    /// (the broker carries it) and not delivered.
    pub fn set_offline(&mut self, who: &NodeId, offline: bool) {
        if offline {
            self.offline.insert(who.clone(), ());
        } else {
            self.offline.remove(who);
        }
    }

    /// Deliver all mail queued for `to` on `topic` (empty if `to` is offline).
    pub fn drain_topic(&mut self, to: &NodeId, topic: GroupTopic) -> Vec<Envelope> {
        if self.offline.contains_key(to) {
            return Vec::new();
        }
        self.queues
            .get_mut(&(to.clone(), topic))
            .map(|q| q.drain(..).collect())
            .unwrap_or_default()
    }

    // --- opaque snapshot store (for recovery, E3.3) -------------------------

    pub fn put_snapshot(&mut self, topic: GroupTopic, bytes: Vec<u8>) {
        self.snapshots.insert(topic, bytes);
    }

    pub fn get_snapshot(&self, topic: GroupTopic) -> Option<Vec<u8>> {
        self.snapshots.get(&topic).cloned()
    }

    // --- blindness audit (E3.4) --------------------------------------------

    /// Everything the broker observed: routing metadata + ciphertext lengths.
    pub fn audit(&self) -> &[Observed] {
        &self.observed
    }

    /// True iff none of the bytes the broker handled contain any of the given
    /// plaintexts as a substring — i.e. the broker never saw cleartext. (MLS
    /// encrypts payloads, so plaintext must not appear in any observed buffer,
    /// including the snapshots, which are group-info, not messages.)
    pub fn never_saw_plaintext(&self, plaintexts: &[&[u8]]) -> bool {
        let contains = |hay: &[u8], needle: &[u8]| {
            !needle.is_empty() && hay.windows(needle.len()).any(|w| w == needle)
        };
        let in_observed = self
            .observed
            .iter()
            .any(|o| plaintexts.iter().any(|p| contains(&o.ciphertext, p)));
        let in_snapshots = self
            .snapshots
            .values()
            .any(|s| plaintexts.iter().any(|p| contains(s, p)));
        !in_observed && !in_snapshots
    }
}
