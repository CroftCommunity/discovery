//! One recipient's mailbox: a list of content addresses, and the have/want diff over it.
//!
//! SPEC-DELTA[meer-spike-clock | test-scaffold]: entries are stamped with a day from CISS's
//! own [`SimClock`] rather than wall time, so S5 can age a queue past its retention window
//! deterministically. Chosen over a spike-local clock seam because the substrate already had
//! one, built for exactly this ("no wall-clock reads", day granularity — the granularity a
//! 14-day window works in). — Register: `alpha/experiments/SPEC-DIVERGENCE-REGISTER.md`
//!
//! **There is no stored offset on either side.** The recipient states the digests it holds and
//! the queue answers with the difference. Content-addressing is what removes the cursor: the
//! cursor and the gap-detector are the same object. That is why a drain is idempotent and why
//! only an ack prunes.
//!
//! The expiry/watermark half deliberately does not live here yet — S5 drives it in Phase 9.

use std::collections::HashSet;

/// A content address: the sha256 of the sealed bytes, as CISS returns it.
///
/// A newtype rather than a bare `String` because "the address of these exact bytes" and
/// "some string" are different things, and the whole design rests on the distinction.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Digest(String);

impl Digest {
    /// Wrap a content address.
    #[must_use]
    pub fn new(hex: impl Into<String>) -> Self {
        Self(hex.into())
    }

    /// The address as hex.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Digest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// Who a queue belongs to.
///
/// A label in Phase 3. From Phase 4 it carries the caller's iroh `EndpointId` —
/// SPEC-DELTA[meer-spike-drain-auth | stand-in], tagged at its site in `transport.rs`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecipientId(String);

impl RecipientId {
    /// Name a recipient.
    #[must_use]
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// The underlying label.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for RecipientId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// One queued item: what to fetch, and when it arrived.
#[derive(Debug, Clone)]
pub struct Entry {
    digest: Digest,
    deposited_day: u64,
}

impl Entry {
    /// The content address this entry points at.
    #[must_use]
    pub fn digest(&self) -> &Digest {
        &self.digest
    }

    /// The day this entry was deposited, on the meer's clock. Read by S5's retention window.
    #[must_use]
    pub fn deposited_day(&self) -> u64 {
        self.deposited_day
    }
}

/// One recipient's mailbox.
#[derive(Debug, Default)]
pub struct Queue {
    entries: Vec<Entry>,
}

impl Queue {
    /// An empty queue.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add `digest` to the queue, stamped with `day`.
    ///
    /// Idempotent on digest: the same object queued twice for the same recipient is one entry,
    /// which is what makes S3's dual delivery (live + drained) free rather than a special case.
    pub fn append(&mut self, digest: Digest, day: u64) {
        if self.entries.iter().any(|e| e.digest == digest) {
            return;
        }
        self.entries.push(Entry {
            digest,
            deposited_day: day,
        });
    }

    /// The digests this queue holds that `have` does not — the want-set.
    ///
    /// A digest in `have` that the queue never held is simply absent from the difference; it
    /// is not an error and is never echoed back.
    #[must_use]
    pub fn want(&self, have: &[Digest]) -> Vec<Digest> {
        let held: HashSet<&Digest> = have.iter().collect();
        self.entries
            .iter()
            .map(|e| &e.digest)
            .filter(|d| !held.contains(*d))
            .cloned()
            .collect()
    }

    /// Drop the acknowledged digests. This is the only thing that prunes — a drain does not
    /// consume, so draining twice returns the same set.
    pub fn ack(&mut self, acked: &[Digest]) {
        let acked: HashSet<&Digest> = acked.iter().collect();
        self.entries.retain(|e| !acked.contains(&e.digest));
    }

    /// How many entries are held.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the queue is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// The entries, in deposit order.
    #[must_use]
    pub fn entries(&self) -> &[Entry] {
        &self.entries
    }
}
