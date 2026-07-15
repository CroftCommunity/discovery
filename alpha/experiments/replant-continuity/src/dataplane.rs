//! **B1 — the dataplane hash structure (experiment-grade).** The message-continuity half of the
//! §7.6.2 re-plant: an in-flight conversation survives the atomic repoint with **no loss and no
//! duplication**. The membership half (whom the stamp seats) lives in `lib.rs`; this is the
//! *content* thread the PSK does not carry — the dataplane history the re-plant repoints.
//!
//! Serves: Part 2 §7.6.2 (the message-continuity half of re-plant) + Appendix B / B1 (the
//! dataplane hash structures) — earns/bounds: §7.6.2 message half `Modeled` at loopback grade
//! (a reasoning-complete model over content-addressed records; no real transport, no wire
//! pinning) — register: none — landed: RUN-09 Part 3.
//!
//! A **record** is a data-plane entry: content + a causal link (its antecedent's content id) + a
//! governance-generation stamp (§7.6, "each data-plane entry MUST carry a governance-generation
//! stamp"). Its identity is the BLAKE3 hash of a **test-only** canonical serialization — NOT the
//! `[gates-release]` wire encoding, which is deliberately not pinned here.
//!
//! A **history** is the hash structure records fold into (§6.6.3: "trust a record only on its own
//! signature and its fold into the hash structure"). It is content-addressed (so a record is
//! present at most once), causally linked (so a dropped predecessor leaves a detectable gap), and
//! carries a **digest** that is a function of the causally-ordered content set and therefore
//! byte-identical across arrival orders. The digest uses a test-only serialization: it exists so
//! two independently-arrived histories can be compared for equality, not to pin a wire format.

use std::collections::BTreeMap;

/// Domain-separation tags for the test-only serializations (kept distinct so a record digest can
/// never collide with a history digest).
const RECORD_TAG: &[u8] = b"drystone-dataplane-record-v0-testonly";
const HISTORY_TAG: &[u8] = b"drystone-dataplane-history-v0-testonly";

/// A data-plane entry: the content thread the re-plant carries. `antecedent` is the content id of
/// the causal predecessor (`None` for the conversation root), so the records form a causal chain
/// (or DAG) independent of arrival order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Record {
    /// Authoring principal (governance identity).
    pub author: [u8; 32],
    /// The governance-generation stamp the author held (§7.6, line "generation stamp").
    pub gen_stamp: u64,
    /// Content id of the causal predecessor; `None` iff this is a conversation root.
    pub antecedent: Option<[u8; 32]>,
    /// The (already-sealed, opaque to this layer) payload bytes.
    pub body: Vec<u8>,
}

impl Record {
    /// The record's content id: BLAKE3 over a test-only canonical serialization. Deterministic and
    /// collision-resistant over the fields, so equal records share an id and a mutated field yields
    /// a different id (which is how a dup is recognised and a tampered record is rejected).
    #[must_use]
    pub fn id(&self) -> [u8; 32] {
        let mut h = blake3::Hasher::new();
        h.update(RECORD_TAG);
        h.update(&self.author);
        h.update(&self.gen_stamp.to_be_bytes());
        match &self.antecedent {
            Some(a) => {
                h.update(&[1u8]);
                h.update(a);
            }
            None => {
                h.update(&[0u8]);
            }
        };
        h.update(&(self.body.len() as u64).to_be_bytes());
        h.update(&self.body);
        *h.finalize().as_bytes()
    }
}

/// The outcome of folding one record into a [`History`] — the detection surface the negative case
/// (d) rests on. A duplicate or a gap is **reported**, never silently absorbed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Fold {
    /// A new record; folded in.
    New,
    /// The record's content id is already present — a duplicate, detected and not re-applied.
    Duplicate,
    /// The record's antecedent is absent — a causal gap; the record is held, not folded.
    Gap { missing: [u8; 32] },
}

/// A summary of delivering a stream of records to quiescence: how many folded, how many were
/// detected as duplicates, and which antecedents never arrived (an unrepaired **drop**).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DeliveryReport {
    pub applied: usize,
    pub duplicates: usize,
    /// Antecedent ids still missing after quiescence — each is a record that could not fold because
    /// its causal predecessor was dropped. Empty iff the stream was complete.
    pub unresolved_gaps: Vec<[u8; 32]>,
}

impl DeliveryReport {
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.unresolved_gaps.is_empty()
    }
}

/// **The B1 dataplane hash structure.** A content-addressed, causally-linked record set. Records
/// are keyed by content id, so the structure is a set (no record appears twice) whose
/// causally-ordered digest is independent of the order records arrived in.
#[derive(Clone, Default)]
pub struct History {
    by_id: BTreeMap<[u8; 32], Record>,
}

impl History {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Fold one record. Returns [`Fold::Duplicate`] if its content id is already present,
    /// [`Fold::Gap`] if its antecedent is absent (the record is *not* folded), else [`Fold::New`].
    pub fn fold(&mut self, r: Record) -> Fold {
        let id = r.id();
        if self.by_id.contains_key(&id) {
            return Fold::Duplicate;
        }
        if let Some(a) = r.antecedent {
            if !self.by_id.contains_key(&a) {
                return Fold::Gap { missing: a };
            }
        }
        self.by_id.insert(id, r);
        Fold::New
    }

    /// Deliver a stream of records, buffering causally-early arrivals and retrying until quiescent
    /// — the loopback stand-in for the out-of-order delivery a real transport produces. A duplicate
    /// is counted and dropped; a record whose antecedent never arrives is reported as an unresolved
    /// gap. Convergence is arrival-order-independent: any permutation of a complete stream yields
    /// the same folded set.
    pub fn deliver_all<I: IntoIterator<Item = Record>>(&mut self, records: I) -> DeliveryReport {
        let mut pending: Vec<Record> = records.into_iter().collect();
        let mut report = DeliveryReport::default();
        loop {
            let mut progressed = false;
            let mut still_pending: Vec<Record> = Vec::new();
            for r in std::mem::take(&mut pending) {
                match self.fold(r.clone()) {
                    Fold::New => {
                        report.applied += 1;
                        progressed = true;
                    }
                    Fold::Duplicate => {
                        report.duplicates += 1;
                        progressed = true;
                    }
                    Fold::Gap { .. } => still_pending.push(r),
                }
            }
            pending = still_pending;
            if !progressed || pending.is_empty() {
                break;
            }
        }
        report.unresolved_gaps = pending.iter().filter_map(|r| r.antecedent).collect();
        report
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.by_id.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.by_id.is_empty()
    }

    #[must_use]
    pub fn contains(&self, id: &[u8; 32]) -> bool {
        self.by_id.contains_key(id)
    }

    /// The records in **deterministic causal order**: a stable topological sort (Kahn's algorithm
    /// with the ready set ordered by `(gen_stamp, id)`), so an antecedent always precedes its
    /// dependents and the order is a pure function of the set — never of arrival order.
    #[must_use]
    pub fn ordered(&self) -> Vec<&Record> {
        let mut emitted: BTreeMap<[u8; 32], ()> = BTreeMap::new();
        let mut out: Vec<&Record> = Vec::with_capacity(self.by_id.len());
        // Records not yet emitted, revisited until fixpoint. The ready predicate (antecedent None
        // or already emitted) plus the (gen_stamp, id) ordering makes each pass deterministic.
        loop {
            let mut ready: Vec<(&[u8; 32], &Record)> = self
                .by_id
                .iter()
                .filter(|(id, _)| !emitted.contains_key(*id))
                .filter(|(_, r)| match &r.antecedent {
                    None => true,
                    Some(a) => emitted.contains_key(a) || !self.by_id.contains_key(a),
                })
                .collect();
            if ready.is_empty() {
                break;
            }
            ready.sort_by(|(ida, ra), (idb, rb)| {
                ra.gen_stamp.cmp(&rb.gen_stamp).then_with(|| ida.cmp(idb))
            });
            for (id, r) in ready {
                emitted.insert(*id, ());
                out.push(r);
            }
        }
        out
    }

    /// A digest that is a **function of the causally-ordered content set**: BLAKE3 folded over the
    /// ordered record ids. Byte-identical across arrival orders (because [`ordered`](Self::ordered)
    /// is), and different whenever a record is added, dropped, or altered. Test-only serialization —
    /// it exists to compare two histories, not to pin a wire format.
    #[must_use]
    pub fn digest(&self) -> [u8; 32] {
        let mut h = blake3::Hasher::new();
        h.update(HISTORY_TAG);
        h.update(&(self.by_id.len() as u64).to_be_bytes());
        for r in self.ordered() {
            h.update(&r.id());
        }
        *h.finalize().as_bytes()
    }
}
