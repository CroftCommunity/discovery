//! B3 — a mock responder with the `com.atproto.repo.listRecords` shape, and
//! the member-side consumer that names chain gaps by bounding digests.
//!
//! The responder mirrors the *verified* lexicon surface (matchup row 3,
//! §5-4): `cursor` + `limit` + rkey-sorted pages, **no server-side range
//! bounds** (the fetched lexicon carries none), and — rbsr-construction
//! req. 3 — **no session state**: every call is answered from the sorted map
//! and the cursor value alone.
//!
//! The consumer renders history-durability.md §I: a gap is *named by its two
//! bounding digests* (the last held entry's digest and the next served
//! entry's digest), which is exactly the span request both parties can
//! resolve. Counter arithmetic locates a break; digests name it.

use crate::envelope::{Digest, Envelope, GENESIS_MARKER};
use crate::rkey::{entry_rkey, rkey_prefix};
use std::collections::BTreeMap;

/// A stateless mock PDS: rkey-sorted records, cursor-paged reads.
#[derive(Debug, Default)]
pub struct MockPds {
    records: BTreeMap<String, Envelope>,
}

/// One `listRecords`-shaped page.
#[derive(Debug, Clone)]
pub struct Page {
    pub items: Vec<(String, Envelope)>,
    /// Present iff more records follow (the XRPC cursor convention:
    /// "continuing until the cursor is not included any longer").
    pub cursor: Option<String>,
}

impl MockPds {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, env: Envelope) {
        self.records.insert(entry_rkey(&env.subspace, env.counter), env);
    }

    pub fn remove(&mut self, subspace: &Digest, counter: u64) -> Option<Envelope> {
        self.records.remove(&entry_rkey(subspace, counter))
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Stateless cursored read over one subspace's rkey range (the client
    /// filters by prefix — the lexicon offers no server-side bound). The
    /// cursor is the last-served rkey; the server holds no session.
    pub fn list_records(&self, subspace: &Digest, cursor: Option<&str>, limit: usize) -> Page {
        let prefix = rkey_prefix(subspace);
        let mut items: Vec<(String, Envelope)> = self
            .records
            .iter()
            .filter(|(k, _)| k.starts_with(&prefix))
            .filter(|(k, _)| cursor.is_none_or(|c| k.as_str() > c))
            .take(limit + 1)
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        let more = items.len() > limit;
        items.truncate(limit);
        let cursor = if more {
            items.last().map(|(k, _)| k.clone())
        } else {
            None
        };
        Page { items, cursor }
    }
}

/// The §I gap, named by its bounding digests.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChainCheck {
    /// The served span is predecessor-linked and counter-contiguous.
    Complete(Vec<Envelope>),
    /// A gap: the missing span is bounded by (digest of the last entry held
    /// before the break, digest of the first entry served after it).
    Gap {
        subspace: Digest,
        after_digest: Digest,
        before_digest: Digest,
    },
}

/// Member-side consumer: pull every page for `subspace`, verify the chain,
/// name any gap by its bounding digests. (The B3 red run captured the naive
/// last-page consumer — cursor-to-exhaustion, no linkage check — calling a
/// gapped chain complete before this verifying form went green.)
pub fn assemble_chain(pds: &MockPds, subspace: &Digest, page_limit: usize) -> ChainCheck {
    let mut out = Vec::new();
    let mut cursor: Option<String> = None;
    loop {
        let page = pds.list_records(subspace, cursor.as_deref(), page_limit);
        out.extend(page.items.into_iter().map(|(_, e)| e));
        match page.cursor {
            Some(c) => cursor = Some(c),
            None => return verify_span(subspace, &out),
        }
    }
}

/// Chain verification shared with B5/B6: given rkey-ordered envelopes of one
/// subspace, confirm genesis anchoring, counter contiguity, and predecessor
/// linkage; name the first break by bounding digests.
pub fn verify_span(subspace: &Digest, span: &[Envelope]) -> ChainCheck {
    let mut prev: Option<&Envelope> = None;
    for e in span {
        let ok = match prev {
            None => e.predecessor == GENESIS_MARKER && e.counter == 0,
            Some(p) => e.predecessor == p.entry_digest && e.counter == p.counter + 1,
        };
        if !ok {
            return ChainCheck::Gap {
                subspace: *subspace,
                after_digest: prev.map(|p| p.entry_digest).unwrap_or(GENESIS_MARKER),
                before_digest: e.entry_digest,
            };
        }
        prev = Some(e);
    }
    ChainCheck::Complete(span.to_vec())
}
