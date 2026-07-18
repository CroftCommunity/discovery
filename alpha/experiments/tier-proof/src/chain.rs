//! Reader-side chain detection (RUN-18 B2–B4; GROUPS.md A.2, the reception
//! paragraph).
//!
//! In a write-restricted scope the author's envelopes chain: each carries the
//! author's previous envelope as its FIRST antecedent, the first anchoring to
//! the scope genesis (enforced producer-side by [`crate::relay`], B1). This
//! module is the SUBSCRIBER side: from a held set of envelopes ALONE — no
//! counts, no positions, no serving node's word — [`detect`] reconstructs the
//! chain's shape, names every gap as a known omission, and states the honest
//! completeness claim: complete **up to the newest held envelope**, never
//! "current" (the withheld tail is undetectable until anything newer arrives
//! by any path — the completeness-ahead doctrine).

use std::collections::{BTreeMap, BTreeSet};

use crate::envelope::Envelope;
use crate::records::{self, Record};
use crate::source::SourceEvent;

/// What the chain's shape says about a held set of envelopes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChainReport {
    /// Held identities no other held envelope links to — the tops of the held
    /// segments. One head = one unbroken held segment; more = gaps between.
    /// Sorted by identity: order AMONG multiple heads is unknowable from the
    /// chain alone, and the report does not pretend otherwise.
    pub heads: Vec<String>,
    /// Identities the chain references that are neither held nor the genesis
    /// anchor — every gap, each a KNOWN omission, named.
    pub missing: Vec<String>,
    /// Whether some held segment reaches the genesis anchor.
    pub anchored: bool,
}

impl ChainReport {
    /// The newest held envelope — defined only when the held set forms a
    /// single segment (one head). With multiple segments no "newest" is
    /// claimable from the chain alone.
    #[must_use]
    pub fn newest_held(&self) -> Option<&str> {
        match self.heads.as_slice() {
            [only] => Some(only),
            _ => None,
        }
    }

    /// Provably complete up to the newest held envelope: one segment, no
    /// missing links, anchored at genesis. This is the STRONGEST claim the
    /// detector ever makes — nothing here claims currency (B3).
    #[must_use]
    pub fn complete_up_to_newest(&self) -> bool {
        self.heads.len() == 1 && self.missing.is_empty() && self.anchored
    }

    /// The detector's claim, rendered. Its strongest form is
    /// `complete as of <newest held>` — scoped, by construction, to what is
    /// held; there is no wording for full currency because the detector can
    /// never know it (the withheld-tail limit, completeness-ahead doctrine).
    #[must_use]
    pub fn claim(&self) -> String {
        if self.complete_up_to_newest() {
            format!("complete as of {}", self.heads[0])
        } else if self.missing.is_empty() && !self.anchored {
            "incomplete: not anchored to genesis".to_string()
        } else {
            format!(
                "incomplete: {} known omission(s): {}",
                self.missing.len(),
                self.missing.join(", ")
            )
        }
    }
}

/// Reconstruct the chain's shape for `author`'s message envelopes in `scope`
/// from `held` alone. Envelopes failing signature verification, foreign to the
/// scope/author, or non-messages are ignored (a detector never trusts bytes it
/// cannot verify); duplicates collapse by `H(envelope)`.
#[must_use]
pub fn detect(genesis_id: &str, scope: &str, author: &str, held: &[Envelope]) -> ChainReport {
    let mut by_id: BTreeMap<String, &Envelope> = BTreeMap::new();
    for env in held {
        if env.body.scope == scope
            && env.body.author == author
            && env.verify().is_ok()
            && matches!(records::decode(env), Ok(Record::Message { .. }))
        {
            by_id.insert(env.identity_hex(), env);
        }
    }
    let link = |env: &&Envelope| env.body.antecedents.first().cloned();

    let referenced: BTreeSet<String> = by_id.values().filter_map(link).collect();
    let heads: Vec<String> = by_id
        .keys()
        .filter(|id| !referenced.contains(*id))
        .cloned()
        .collect();

    let mut missing: Vec<String> = Vec::new();
    let mut anchored = false;
    for env in by_id.values() {
        match link(env) {
            Some(l) if l == genesis_id => anchored = true,
            Some(l) if !by_id.contains_key(&l) => missing.push(l),
            _ => {}
        }
    }
    missing.sort();
    missing.dedup();

    ChainReport {
        heads,
        missing,
        anchored,
    }
}

/// What a reader can say about an identity's existence (RUN-18 B5; the
/// tamper-evident-history delta, PUBLICATIONS.md §4). The three-way
/// distinction lives in the absent-content variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Existence {
    /// The reader holds the envelope; nothing is absent.
    Held,
    /// No held chain references the identity: it never existed, as far as any
    /// chain can attest.
    NeverExisted,
    /// The chain references the identity AND its deletion is verifiable at
    /// source (an authenticated delete by the chain author). Content gone,
    /// existence provable — retraction is possible, but never silent.
    Retracted,
    /// The chain references the identity, no reached source offers it, and
    /// deletion cannot be shown. Withheld — from this reader, at this moment.
    WithheldFromMe,
}

/// Classify an identity's existence for `author`'s chain in `scope`, from the
/// reader's `held` set plus the `source_view` the reader checked for
/// authenticated deletions (the author's repo event stream; at component
/// grade the harness's delete events stand in for the signed repo tree —
/// `SPEC-DELTA[run18-retraction-local | stand-in]`).
#[must_use]
pub fn classify_existence(
    scope: &str,
    author: &str,
    id: &str,
    held: &[Envelope],
    source_view: &[SourceEvent],
) -> Existence {
    let report = detect("", scope, author, held);
    if held.iter().any(|e| e.identity_hex() == id) {
        return Existence::Held;
    }
    if !report.missing.iter().any(|m| m == id) {
        return Existence::NeverExisted;
    }
    let deleted_at_source = source_view.iter().any(|ev| {
        matches!(ev, SourceEvent::Delete { author: a, target } if a == author && target == id)
    });
    if deleted_at_source {
        Existence::Retracted
    } else {
        Existence::WithheldFromMe
    }
}

/// The vanilla current-state check: is the record identified by `id` present
/// in the repo's CURRENT state (put and not since deleted)? This is what the
/// bare substrate proves — tamper-FREE but memoryless: a retracted record and
/// a never-existed one are the same absence here (the contrast B5 asserts).
#[must_use]
pub fn vanilla_present(events: &[SourceEvent], id: &str) -> bool {
    let mut present = false;
    for ev in events {
        match ev {
            SourceEvent::Put(env) if env.identity_hex() == id => present = true,
            SourceEvent::Delete { target, .. } if target == id => present = false,
            _ => {}
        }
    }
    present
}
