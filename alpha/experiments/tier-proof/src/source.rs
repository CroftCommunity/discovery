//! Record sources: the interface the fold ingests, and the live/stand-in split.
//!
//! The live source is a real Jetstream firehose (RUN-14 proved this path); it
//! needs `ATP_TEST_*` credentials + egress. Absent those, [`live_open_tier_leg`]
//! reports **BLOCKED** (never fabricates), and [`MemSource`] — an ordered,
//! in-memory event log — stands in behind the SAME [`RecordSource`] interface
//! (`SPEC-DELTA[run17-live-source | declared-stand-in]`). Causal order is the
//! vector order; a real firehose supplies the same ordering via commit `seq`.

use crate::envelope::Envelope;
use crate::identity::Signer;
use crate::records::{self, Record};

/// One firehose event: a record `create` or a record `delete`.
#[derive(Debug, Clone)]
pub enum SourceEvent {
    /// A `create`/`update` commit carrying a signed envelope.
    Put(Envelope),
    /// A `delete` commit: `author` deletes the record whose envelope identity
    /// is `target` (hex). No record body, mirroring the live `delete` frame.
    Delete {
        /// The DID that issued the delete.
        author: String,
        /// The `H(envelope)` (hex) of the deleted record.
        target: String,
    },
}

/// A source of ordered record events (live firehose or the in-memory stand-in).
pub trait RecordSource {
    /// All events in causal (commit-seq) order.
    fn all(&self) -> Vec<SourceEvent>;
}

/// The outcome of the live open-tier leg.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LiveLeg {
    /// The leg did not run; `reason` names what blocked it.
    Blocked {
        /// Why the live leg was not run (e.g. missing credentials).
        reason: String,
    },
    /// The leg ran live; `detail` records what was written/observed.
    Ran {
        /// A human note about the live run.
        detail: String,
    },
}

/// Probe the live open-tier leg without pretending. Returns [`LiveLeg::Blocked`]
/// when the `ATP_TEST_*` credentials are absent (guardrail 4: BLOCKED beats
/// pretended). The actual live write lives in `bin/open_live` and only runs
/// when credentials are present.
#[must_use]
pub fn live_open_tier_leg() -> LiveLeg {
    let handle = std::env::var("ATP_TEST_HANDLE")
        .ok()
        .filter(|s| !s.is_empty());
    let pass = std::env::var("ATP_TEST_PASSWORD")
        .ok()
        .filter(|s| !s.is_empty());
    match (handle, pass) {
        (Some(_), Some(_)) => LiveLeg::Ran {
            detail: "ATP_TEST credentials present; run `cargo run --bin open_live`".to_string(),
        },
        _ => LiveLeg::Blocked {
            reason: "ATP_TEST_HANDLE/ATP_TEST_PASSWORD not set: live open-tier leg not run \
                     (guardrail 4: BLOCKED beats pretended); stand-in MemSource used"
                .to_string(),
        },
    }
}

/// An ordered in-memory event log standing in for a live firehose.
#[derive(Debug, Default, Clone)]
pub struct MemSource {
    events: Vec<SourceEvent>,
    tips: Vec<String>,
}

impl MemSource {
    /// A new, empty source.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Append a record authored by `signer`, linked causally to the current
    /// tips. Returns the record's `H(envelope)` (hex).
    pub fn put_record(&mut self, signer: &Signer, record: Record) -> String {
        let antecedents = self.tips.clone();
        self.put_record_with_antecedents(signer, antecedents, record)
    }

    /// Append a record with explicit antecedents (used where a test constructs a
    /// specific causal position — e.g. a grant citing a request hash).
    pub fn put_record_with_antecedents(
        &mut self,
        signer: &Signer,
        antecedents: Vec<String>,
        record: Record,
    ) -> String {
        let env = records::seal(signer, antecedents, &record);
        let id = env.identity_hex();
        self.events.push(SourceEvent::Put(env));
        self.tips = vec![id.clone()];
        id
    }

    /// Append a `delete` of the record identified by `target`, issued by
    /// `signer` (a leave-by-deletion).
    pub fn delete(&mut self, signer: &Signer, target: &str) {
        self.events.push(SourceEvent::Delete {
            author: signer.did(),
            target: target.to_string(),
        });
        self.tips = vec![target.to_string()];
    }

    /// Split the log into a backfill prefix and a tail suffix, preserving order.
    /// Concatenating them reproduces the full causal stream (the reconstruction
    /// input).
    #[must_use]
    pub fn split_backfill_tail(&self) -> (Vec<SourceEvent>, Vec<SourceEvent>) {
        let n = self.events.len();
        let cut = n / 2;
        (self.events[..cut].to_vec(), self.events[cut..].to_vec())
    }
}

impl RecordSource for MemSource {
    fn all(&self) -> Vec<SourceEvent> {
        self.events.clone()
    }
}
