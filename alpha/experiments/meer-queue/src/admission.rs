//! **The admission fact (E112, PRELIMINARY).**
//!
//! DECISION-2 amended: *recognition is the merge, **and the merge deposits the admission fact.***
//! Every external-commit admission deposits a governance fact, or it does not happen. The **merging
//! member** mints an R6-shaped acceptance record (§7.5.1 — an acceptance record *is* a governance
//! fact) stating "merged lineage L's `NewMemberCommit` [content address], redeeming token T, at my
//! frontier F," chained into the acceptor's acceptance chain. The returner mints nothing (a
//! non-member, and at head possibly banned, authors no chain fact).
//!
//! **Comparator placement (owner talk-through, 2026-08-17; discharges §11.11 item 3).** The
//! admission fact is typed as an **acceptance/event record that opens a membership span — never a
//! slot-competing membership addition.** It records the mechanical execution of the charter's
//! pre-agreed merge rule, not a governance decision about the subject's standing, so it does not
//! enter the §7.3.1 tier contest. Identity is the *event* — the commit's content address (§7.3.4
//! sign-the-state) — so per-acceptor facts about one admission **corroborate**, they never rival.
//!
//! This module is the application-plane logic; the MLS merge itself is real openmls, driven by the
//! S24/C4 harnesses. Fidelity of this module's own claims: **Modeled** (the issuance ledger and
//! acceptance chain are in-memory stand-ins for governance-chain state).

use std::collections::HashMap;

use sha2::{Digest, Sha256};

/// The content address of a `NewMemberCommit` — the admission *event*'s identity. Per-acceptor
/// admission facts naming the same address corroborate one event (§7.3.4), never rival it.
#[must_use]
pub fn content_address(commit_wire: &[u8]) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(b"croft/admission-event/v1");
    h.update(commit_wire);
    h.finalize().into()
}

/// A governance issuance fact: the chain record that a token was issued to a lineage. Holding the
/// PSK *bytes* is not this — the bytes are key material, the fact is governance. Severing the two
/// is the point of S24 refusal arm (d).
#[derive(Debug, Clone)]
pub struct IssuanceFact {
    /// The lineage the token was issued to (the only lineage it may admit).
    pub lineage: Vec<u8>,
    /// Revoked issuance facts still exist on the chain; they just no longer admit.
    pub revoked: bool,
}

/// The governance ledger of issuance facts, keyed by token id. An incumbent consults its own copy.
#[derive(Debug, Default, Clone)]
pub struct IssuanceLedger {
    facts: HashMap<Vec<u8>, IssuanceFact>,
}

impl IssuanceLedger {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Record that `token_id` was issued to `lineage`.
    pub fn issue(&mut self, token_id: Vec<u8>, lineage: Vec<u8>) {
        self.facts.insert(token_id, IssuanceFact { lineage, revoked: false });
    }

    /// Mark an existing issuance fact revoked (the bytes are untouched — no key-deletion race).
    pub fn revoke(&mut self, token_id: &[u8]) {
        if let Some(f) = self.facts.get_mut(token_id) {
            f.revoked = true;
        }
    }

    #[must_use]
    pub fn get(&self, token_id: &[u8]) -> Option<&IssuanceFact> {
        self.facts.get(token_id)
    }
}

/// An R6-shaped acceptance record: the admission fact. **Opens a membership span**; it is never a
/// slot-competing membership addition (see module docs).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmissionFact {
    /// The admission event's identity: the content address of the merged `NewMemberCommit`.
    pub event: [u8; 32],
    /// The lineage admitted (the span this fact opens is that lineage's).
    pub merged_lineage: Vec<u8>,
    /// The token redeemed.
    pub redeemed_token: Vec<u8>,
    /// The acceptor (merging member) — the author of this per-acceptor fact.
    pub acceptor: Vec<u8>,
    /// The acceptor's frontier F at merge — its frontier commitment (§7.5.1 classifies acceptors
    /// concurrently-stale by this).
    pub acceptor_frontier: u64,
}

/// Why a `NewMemberCommit` merge is refused before it can seat anyone. Refusing here is the
/// merge-rule clause: *a merge that would not emit its admission fact does not happen.*
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum MergeRefusal {
    /// The PSK bytes resolve, but there is **no issuance fact** for this token — a forged /
    /// out-of-band ledger entry. Severs fact-from-bytes (S24 arm d). The admission fact cannot
    /// cite an issuance that does not exist, so the merge is refused.
    #[error("no issuance fact for the presented token: fact cannot be minted")]
    NoIssuanceFact,
    /// The issuance fact exists but is revoked at head.
    #[error("issuance fact is revoked")]
    Revoked,
    /// The token was issued to a different lineage than the joiner's credential resolves to (the
    /// credential half of the §11.7 check).
    #[error("token issued to a different lineage than the joiner presents")]
    LineageMismatch,
}

/// The acceptor's acceptance chain: the append-only record of admission facts it has minted.
#[derive(Debug, Default, Clone)]
pub struct AcceptanceChain {
    facts: Vec<AdmissionFact>,
}

impl AcceptanceChain {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    fn append(&mut self, fact: AdmissionFact) {
        self.facts.push(fact);
    }

    /// Facts this acceptor has minted about `event` (its content address).
    #[must_use]
    pub fn facts_for(&self, event: &[u8; 32]) -> Vec<&AdmissionFact> {
        self.facts.iter().filter(|f| &f.event == event).collect()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.facts.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.facts.is_empty()
    }
}

/// Evaluate whether an admission fact can be minted for this merge, and return the fact to emit.
///
/// This is the merge-rule clause in one place: an admission is refused unless the fact can be
/// minted — i.e. there is an unrevoked issuance fact whose lineage matches the joiner's. The PSK
/// bytes resolving (real MLS) is necessary but **not sufficient**: the governance fact must exist.
///
/// # Errors
/// [`MergeRefusal::NoIssuanceFact`], [`MergeRefusal::Revoked`], or [`MergeRefusal::LineageMismatch`].
pub fn evaluate_admission(
    ledger: &IssuanceLedger,
    acceptor: &[u8],
    acceptor_frontier: u64,
    token_id: &[u8],
    joiner_lineage: &[u8],
    commit_wire: &[u8],
) -> Result<AdmissionFact, MergeRefusal> {
    let fact = ledger.get(token_id).ok_or(MergeRefusal::NoIssuanceFact)?;
    if fact.revoked {
        return Err(MergeRefusal::Revoked);
    }
    if fact.lineage != joiner_lineage {
        return Err(MergeRefusal::LineageMismatch);
    }
    Ok(AdmissionFact {
        event: content_address(commit_wire),
        merged_lineage: joiner_lineage.to_vec(),
        redeemed_token: token_id.to_vec(),
        acceptor: acceptor.to_vec(),
        acceptor_frontier,
    })
}

/// Mint-or-refuse: evaluate, and on success append the fact to `chain` and hand it back so the
/// caller performs the MLS merge. On refusal, nothing is appended — the span never opens.
///
/// **The merge-rule clause made operational:** a caller that merges only when this returns `Ok`
/// cannot seat a joiner without also depositing the admission fact.
///
/// # Errors
/// Propagates [`evaluate_admission`]'s refusals.
pub fn mint_or_refuse(
    ledger: &IssuanceLedger,
    chain: &mut AcceptanceChain,
    acceptor: &[u8],
    acceptor_frontier: u64,
    token_id: &[u8],
    joiner_lineage: &[u8],
    commit_wire: &[u8],
) -> Result<AdmissionFact, MergeRefusal> {
    let fact = evaluate_admission(
        ledger,
        acceptor,
        acceptor_frontier,
        token_id,
        joiner_lineage,
        commit_wire,
    )?;
    chain.append(fact.clone());
    Ok(fact)
}

// ==================================================================================================
// Effective-membership projection: standing read OVER spans (E112, the comparator placement).
//
// The admission fact OPENS a membership span; it is never a slot-competing membership addition. So
// the projection reads STANDING (§7.3.1 decisions) over SPANS (admission/removal events): a span
// says "this lineage was admitted in this window"; standing says "is it banned at head." A ban and
// an admission fact are an enactment record vs a decision — they never form a contradiction PAIR,
// and fold order-independently to "excluded, span recorded." Only two rival standing DECISIONS
// (a ban vs a readmission quorum) contest the slot and hard-stop → CONTESTED.
// ==================================================================================================

/// A governance DECISION on the standing slot (these compete; an admission fact does not).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StandingDecision {
    /// The lineage is banned.
    Ban,
    /// A readmission **quorum** — a governance decision to readmit. Unlike an admission fact, this
    /// competes on the standing slot.
    ReadmitQuorum,
}

/// A standing-slot decision about one lineage.
#[derive(Debug, Clone)]
pub struct StandingEvent {
    pub lineage: Vec<u8>,
    pub decision: StandingDecision,
}

/// One event in an ingest stream: either an admission fact (span-opening, non-competing) or a
/// standing-slot decision (competing).
#[derive(Debug, Clone)]
pub enum Event {
    Admission(AdmissionFact),
    Standing(StandingEvent),
}

/// The effective standing of a lineage at head.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Standing {
    /// A span is open and standing is good — an effective member.
    Member,
    /// A ban is decided at head. Standing read over the span excludes it, even though the span was
    /// real (the window happened).
    Excluded,
    /// Two rival standing DECISIONS (ban vs readmission quorum) — a genuine contradiction,
    /// hard-stop, order-independent.
    Contested,
    /// No span opened for this lineage.
    Absent,
}

/// The projected effective membership: per-lineage span presence + standing classification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Projection {
    /// Sorted `(lineage, span_open, standing_byte)` — canonical, so `to_bytes` is arrival-order
    /// independent by construction.
    rows: Vec<(Vec<u8>, bool, u8)>,
}

fn standing_byte(s: Standing) -> u8 {
    match s {
        Standing::Member => 1,
        Standing::Excluded => 2,
        Standing::Contested => 3,
        Standing::Absent => 0,
    }
}

impl Projection {
    /// Standing of a lineage (Absent if it never appeared).
    #[must_use]
    pub fn standing_of(&self, lineage: &[u8]) -> Standing {
        match self.rows.iter().find(|(l, _, _)| l == lineage) {
            Some((_, _, 1)) => Standing::Member,
            Some((_, _, 2)) => Standing::Excluded,
            Some((_, _, 3)) => Standing::Contested,
            _ => Standing::Absent,
        }
    }

    /// Whether a span is recorded for a lineage (the window was real, even if now excluded).
    #[must_use]
    pub fn span_recorded(&self, lineage: &[u8]) -> bool {
        self.rows.iter().any(|(l, open, _)| l == lineage && *open)
    }

    /// Any contested lineage in the projection.
    #[must_use]
    pub fn any_contested(&self) -> bool {
        self.rows.iter().any(|(_, _, s)| *s == 3)
    }

    /// Canonical bytes — identical for the same fact SET regardless of arrival order.
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut b = Vec::new();
        for (lin, open, st) in &self.rows {
            b.extend_from_slice(&(lin.len() as u32).to_be_bytes());
            b.extend_from_slice(lin);
            b.push(u8::from(*open));
            b.push(*st);
        }
        b
    }
}

/// Project effective membership from an ingest stream. **Order-independent by construction**: the
/// stream is collected into sets before classification, so `project(&[a, b])` and
/// `project(&[b, a])` are byte-identical. An admission fact opens a span but is *not* a standing
/// decision — this is the load-bearing typing (the comparator placement).
#[must_use]
pub fn project(events: &[Event]) -> Projection {
    use std::collections::{BTreeMap, BTreeSet};

    let mut spans: BTreeSet<Vec<u8>> = BTreeSet::new();
    let mut decisions: BTreeMap<Vec<u8>, BTreeSet<u8>> = BTreeMap::new(); // lineage -> {decision}

    for ev in events {
        match ev {
            Event::Admission(f) => {
                spans.insert(f.merged_lineage.clone());
            }
            Event::Standing(s) => {
                let code = match s.decision {
                    StandingDecision::Ban => 0u8,
                    StandingDecision::ReadmitQuorum => 1u8,
                };
                decisions.entry(s.lineage.clone()).or_default().insert(code);
            }
        }
    }

    let mut lineages: BTreeSet<Vec<u8>> = BTreeSet::new();
    lineages.extend(spans.iter().cloned());
    lineages.extend(decisions.keys().cloned());

    let mut rows = Vec::new();
    for lin in lineages {
        let span_open = spans.contains(&lin);
        let empty = BTreeSet::new();
        let d = decisions.get(&lin).unwrap_or(&empty);
        let banned = d.contains(&0);
        let readmit = d.contains(&1);
        // Standing read over the span:
        let standing = if banned && readmit {
            Standing::Contested // two rival DECISIONS on the slot — hard-stop
        } else if banned {
            Standing::Excluded // a ban beats the span; the admission fact never contests it
        } else if span_open || readmit {
            Standing::Member
        } else {
            Standing::Absent
        };
        rows.push((lin, span_open, standing_byte(standing)));
    }
    Projection { rows }
}
