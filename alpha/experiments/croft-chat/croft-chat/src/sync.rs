//! `Replicator` — the convergence layer between a [`Session`] and a [`Transport`].
//!
//! The fold enforces **per-device strict lamport monotonicity**, so a device's
//! own chain must be applied in lamport order — applying a later assertion first
//! permanently rejects the earlier one. A transport, however, delivers frames in
//! arbitrary order. The `Replicator` bridges the two: it buffers drained frames
//! by `(device, lamport)` and applies each device's **contiguous** chain
//! (1, 2, 3, …) in order, holding back any frame whose predecessor has not yet
//! arrived. Re-running is safe — already-applied frames are idempotent.

use std::collections::{BTreeMap, HashMap};

use social_graph_core::{assertion_order_key, GroupId, Session, SessionError};

use crate::transport::{Frame, Topic, Transport};

/// Drives replication of a session's groups over a transport.
#[derive(Default)]
pub struct Replicator {
    /// device → (lamport → frame bytes) not yet applied.
    pending: HashMap<[u8; 32], BTreeMap<u64, Vec<u8>>>,
    /// device → highest contiguous lamport already applied (0 = none).
    applied: HashMap<[u8; 32], u64>,
}

impl Replicator {
    /// A fresh replicator with nothing buffered.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// How many received-but-not-yet-applied frames are buffered.
    ///
    /// This is the node's honest incompleteness signal. A frame stays buffered
    /// when it cannot be applied yet — either its per-device predecessor has not
    /// arrived, or (for a governance fact) the fold is holding it back until a
    /// declared antecedent arrives (the G1 completeness guard). A settled node
    /// buffers nothing; a non-zero count means "I have seen facts I cannot yet
    /// fold — I am catching up," which is exactly what lets a lagging node show
    /// it is behind instead of presenting a stale view as current.
    #[must_use]
    pub fn pending_len(&self) -> usize {
        self.pending.values().map(std::collections::BTreeMap::len).sum()
    }

    /// True when nothing is buffered — the node has folded everything it received.
    #[must_use]
    pub fn is_settled(&self) -> bool {
        self.pending_len() == 0
    }

    /// Publish every frame of `group` on `topic` (idempotent — peers dedup).
    ///
    /// # Errors
    /// Propagates a [`SessionError`] from the export.
    pub fn publish_group(
        &self,
        session: &Session,
        transport: &mut impl Transport,
        topic: &Topic,
        group: &GroupId,
    ) -> Result<(), SessionError> {
        for bytes in session.export_group_log(group)? {
            transport.publish(topic, Frame(bytes));
        }
        Ok(())
    }

    /// Drain the transport and apply foreign frames in per-device lamport order.
    ///
    /// Returns the number of assertions newly applied this call. Frames whose
    /// per-device predecessor has not arrived stay buffered for a later pump.
    pub fn pump(&mut self, session: &Session, transport: &mut impl Transport) -> usize {
        for Frame(bytes) in transport.drain() {
            match assertion_order_key(&bytes) {
                Some((device, lamport)) => {
                    self.pending.entry(device).or_default().insert(lamport, bytes);
                }
                None => tracing::warn!("replicator: undecodable frame dropped"),
            }
        }

        let mut newly_applied = 0usize;
        let devices: Vec<[u8; 32]> = self.pending.keys().copied().collect();
        for device in devices {
            let mut next = self.applied.get(&device).copied().unwrap_or(0) + 1;
            loop {
                let Some(chain) = self.pending.get(&device) else {
                    break;
                };
                let Some(bytes) = chain.get(&next).cloned() else {
                    break; // gap: predecessor of `next` not yet here
                };
                match session.apply_remote(&bytes) {
                    Ok(_) => {
                        self.applied.insert(device, next);
                        if let Some(chain) = self.pending.get_mut(&device) {
                            chain.remove(&next);
                        }
                        newly_applied += 1;
                        next += 1;
                    }
                    Err(e) => {
                        // Should not happen for an in-order chain; surface it.
                        tracing::warn!(lamport = next, "replicator: apply_remote failed: {e}");
                        break;
                    }
                }
            }
        }
        newly_applied
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replicator_starts_empty() {
        let r = Replicator::new();
        assert!(r.pending.is_empty());
        assert!(r.applied.is_empty());
    }
}
