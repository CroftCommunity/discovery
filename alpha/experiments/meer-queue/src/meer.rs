//! The meer: a blind store-and-forward node.
//!
//! **This module deliberately has no OpenMLS dependency.** It takes `&[u8]` in and hands
//! `Vec<u8>` out, and it never parses what it carries. That is the structural form of M2's
//! positive arm: the meer cannot re-frame a message because it cannot *name* the type. A
//! future edit that introduced a re-frame would first have to add an openmls dependency here,
//! which is a visible, reviewable act rather than a silent regression. Phase 6 asserts this
//! mechanically with `cargo tree`.
//!
//! Four of the five operations live here. The fifth — sweep expired, leave a watermark —
//! arrives in Phase 9 with the S5 test that drives it, rather than sitting untested until then.
//!
//! SPEC-DELTA[meer-spike-namespace | stand-in]: no CISS custodian-chain mode exists yet, so
//! the meer owns **one** CISS namespace and per-recipient queues are slots within it. The spec
//! target is per-DID queues in the recipient's own namespace under a revocable custodial
//! grant. This changes *who signs*, not the delivery shape under test — but it is also why
//! S6's "it never left home" is only partly testable in this spike.
//! — Register: `alpha/experiments/SPEC-DIVERGENCE-REGISTER.md`
//!
//! SPEC-DELTA[meer-spike-kind-gate | absent]: chain kinds and the queue-only custodial-write
//! gate do not exist; **nothing enforces them here**. The spike does not test the gate and
//! must not be read as evidence about it.
//! — Register: `alpha/experiments/SPEC-DIVERGENCE-REGISTER.md`

use std::collections::HashMap;

use ciss::clock::SimClock;

use crate::ciss_harness::{CissHarness, Identity};
use crate::queue::Queue;

pub use crate::queue::{Digest, Entry, RecipientId};

/// What can go wrong depositing or serving. The meer never invents a reason — a storage
/// failure is reported with the status and body CISS actually returned.
#[derive(Debug, thiserror::Error)]
pub enum MeerError {
    /// CISS refused the object on `PUT`.
    #[error("CISS refused the deposit: HTTP {status} — {body}")]
    Deposit { status: u16, body: String },
    /// CISS refused or failed to serve an object the queue references.
    #[error("CISS could not serve {digest}: HTTP {status} — {body}")]
    Fetch {
        digest: String,
        status: u16,
        body: String,
    },
}

/// A blind mailbox over CISS.
///
/// Holds no group state, no ordering, and no key. Its only durable state is, per recipient, a
/// list of content addresses — and even the bytes those name live in CISS, not here.
pub struct Meer<'a> {
    ciss: &'a CissHarness,
    /// The meer's own CISS namespace (the `meer-spike-namespace` stand-in).
    namespace: Identity,
    queues: HashMap<RecipientId, Queue>,
    clock: SimClock,
}

impl<'a> Meer<'a> {
    /// A meer backed by `ciss`, owning its own namespace within it.
    #[must_use]
    pub fn new(ciss: &'a CissHarness) -> Self {
        Self {
            namespace: ciss.identity("meer"),
            ciss,
            queues: HashMap::new(),
            clock: SimClock::new(),
        }
    }

    /// **Operation 1 + 2 + 3.** Accept a publish: store the sealed bytes in CISS **once**
    /// (content-addressed, so a message to fifty recipients is one object), then append an
    /// entry to each recipient's queue.
    ///
    /// The bytes are opaque. The meer does not parse, validate, order, or attribute them.
    ///
    /// # Errors
    /// [`MeerError::Deposit`] if CISS refuses the object — e.g. over `MAX_OBJECT_BYTES`,
    /// which is refused at the HTTP boundary with 413.
    pub async fn publish(
        &mut self,
        sealed: &[u8],
        recipients: &[RecipientId],
    ) -> Result<Digest, MeerError> {
        let outcome = self
            .ciss
            .put_object(&self.namespace, "queued", sealed)
            .await;
        if outcome.status != 200 {
            return Err(MeerError::Deposit {
                status: outcome.status,
                body: outcome.body_text(),
            });
        }
        let digest = Digest::new(outcome.cid().ok_or_else(|| MeerError::Deposit {
            status: outcome.status,
            body: format!("no content address in response: {}", outcome.body_text()),
        })?);

        let day = self.clock.now();
        for who in recipients {
            self.queues
                .entry(who.clone())
                .or_default()
                .append(digest.clone(), day);
        }
        tracing::debug!(
            digest = %digest,
            bytes = sealed.len(),
            recipients = recipients.len(),
            "deposit"
        );
        Ok(digest)
    }

    /// **Operation 4.** Serve a drain: the recipient states what it holds, and gets back the
    /// bytes for everything it lacks.
    ///
    /// A drain does **not** consume. Only [`Self::ack`] prunes, which is what makes a repeated
    /// drain idempotent and lets a recipient fail mid-transfer without losing mail.
    ///
    /// # Errors
    /// [`MeerError::Fetch`] if CISS cannot serve an object the queue references.
    pub async fn drain(
        &self,
        who: &RecipientId,
        have: &[Digest],
    ) -> Result<Vec<Vec<u8>>, MeerError> {
        let wanted = self.wants(who, have);
        tracing::debug!(recipient = %who, want = wanted.len(), "drain");
        let mut out = Vec::with_capacity(wanted.len());
        for digest in wanted {
            let got = self.ciss.get_object(&self.namespace, digest.as_str()).await;
            if got.status != 200 {
                return Err(MeerError::Fetch {
                    digest: digest.to_string(),
                    status: got.status,
                    body: got.body_text(),
                });
            }
            out.push(got.body);
        }
        Ok(out)
    }

    /// The want-set for `who` given what it `have`s — the diff, without fetching.
    #[must_use]
    pub fn wants(&self, who: &RecipientId, have: &[Digest]) -> Vec<Digest> {
        self.queues
            .get(who)
            .map(|q| q.want(have))
            .unwrap_or_default()
    }

    /// Acknowledge delivery: prune the acked entries from `who`'s queue.
    pub fn ack(&mut self, who: &RecipientId, acked: &[Digest]) {
        if let Some(q) = self.queues.get_mut(who) {
            q.ack(acked);
            tracing::debug!(recipient = %who, acked = acked.len(), remaining = q.len(), "ack");
        }
    }

    /// How many entries `who`'s queue holds.
    #[must_use]
    pub fn queue_len(&self, who: &RecipientId) -> usize {
        self.queues.get(who).map_or(0, Queue::len)
    }

    /// The deposit day of each entry in `who`'s queue, in order. Read by S5's retention window.
    #[must_use]
    pub fn deposit_days(&self, who: &RecipientId) -> Vec<u64> {
        self.queues
            .get(who)
            .map(|q| q.entries().iter().map(Entry::deposited_day).collect())
            .unwrap_or_default()
    }

    /// Advance the meer's clock by `days` (the `meer-spike-clock` stand-in).
    pub fn advance_days(&mut self, days: u64) {
        self.clock.advance_days(days);
    }
}
