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
