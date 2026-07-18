//! The fold: replay an ordered event stream into a catalogue + roster.
//!
//! The fold is the model's reconstruction engine. It ingests [`SourceEvent`]s in
//! causal order, VERIFIES every envelope before folding (invalid envelopes are
//! dropped, never trusted), and emits:
//!
//! - a **catalogue** — one entry per scope with BOTH policy fields (§A.2);
//! - a **roster** as **membership intervals** (§A.3) — for each (scope, member)
//!   the set of `[grant, cut)` half-open intervals; a member is *currently* on
//!   the roster iff they hold an open-ended interval.
//!
//! Because positions are the stream index, the same event stream always folds
//! to the same intervals — so a rebuild from backfill+tail is byte-identical to
//! the indexed fold ([`FoldState::canonical_digest`]). That equality is the
//! reconstructability proof (§A.5, P2) and the archive-rebuild proof (P4).

use std::collections::{BTreeMap, BTreeSet};

use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::envelope::Envelope;
use crate::records::{self, MembershipPolicy, Record, WritePolicy};
use crate::source::SourceEvent;

/// One member's half-open membership intervals `[start, end)`.
pub type IntervalSet = Vec<(u64, Option<u64>)>;
/// scope → member → interval set (the roster snapshot shape).
type ScopeIntervals = BTreeMap<String, BTreeMap<String, IntervalSet>>;
/// scope → member → roles.
type ScopeRoles = BTreeMap<String, BTreeMap<String, Vec<String>>>;

/// A catalogue entry: a scope and its policy fields.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CatalogueEntry {
    /// The scope id.
    pub scope: String,
    /// The genesis author DID — the "single writer" under `WritePolicy::Single`.
    pub owner: String,
    /// Human title.
    pub title: String,
    /// Who may post.
    pub write_policy: WritePolicy,
    /// How one joins.
    pub membership_policy: MembershipPolicy,
    /// Steward DIDs (gated tier).
    pub steward_set: Vec<String>,
    /// Co-sign threshold (gated tier).
    pub threshold: u32,
    /// If superseded (P7), the successor (supersession record) identity.
    pub superseded_by: Option<String>,
    /// If superseded (P7), the predecessor genesis identity (the lineage link).
    pub predecessor: Option<String>,
    /// If superseded (P7), the causal position of the policy change.
    pub transition_at: Option<u64>,
}

/// Why folding failed (structural; invalid envelopes are dropped, not errors).
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum FoldError {
    /// An envelope payload was not a decodable record.
    #[error("undecodable record at position {0}: {1}")]
    Undecodable(u64, String),
}

/// The folded state.
#[derive(Debug, Clone, Default)]
pub struct FoldState {
    /// Scope catalogue (ordered by scope id).
    pub catalogue: BTreeMap<String, CatalogueEntry>,
    /// (scope, did) → membership intervals `[start, end)` (end `None` = open).
    intervals: BTreeMap<(String, String), IntervalSet>,
    /// (scope, did) → the causal position of the revocation cut, if any.
    cuts: BTreeMap<(String, String), u64>,
    /// (scope, did) → set of roles currently held.
    roles: BTreeMap<(String, String), Vec<String>>,
    /// envelope identity → (author, record), to resolve deletes.
    by_id: BTreeMap<String, (String, Record)>,
    /// envelope identity → causal position (stream index).
    positions: BTreeMap<String, u64>,
    /// request identity → (scope, subject) — the pending-request ledger.
    requests: BTreeMap<String, (String, String)>,
    /// request identity → set of distinct steward DIDs who have co-signed a
    /// grant answering it.
    cosigns: BTreeMap<String, BTreeSet<String>>,
    /// request identities that reached the co-sign threshold.
    granted: BTreeSet<String>,
    /// scope → genesis envelope identity (the chain anchor, RUN-18 B1).
    genesis_ids: BTreeMap<String, String>,
    /// (scope, author) → the author's newest chained envelope identity, for
    /// write-restricted scopes (RUN-18 B1; GROUPS.md A.2 reception paragraph).
    chain_heads: BTreeMap<(String, String), String>,
    /// dropped (failed-verification) envelope count — observability.
    dropped: u64,
}

/// The status of a membership request under the fold.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestStatus {
    /// No such request is known in this scope.
    Unknown,
    /// Recorded but not yet granted to threshold — and silence is not a verdict.
    Pending,
    /// Granted (reached the co-sign threshold).
    Granted,
}

/// Why a message was not admitted (the causal membership check).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdmitReject {
    /// The signature did not verify.
    BadSignature,
    /// The payload was not a decodable message record.
    NotAMessage,
    /// The author held no membership interval covering the message's causal
    /// position (never a member, or the position is at/after the revocation cut).
    OutsideMembership,
}

impl FoldState {
    /// Fold an ordered event stream. Positions are the stream index.
    ///
    /// # Errors
    /// Returns [`FoldError`] only for a structurally undecodable record; a
    /// signature failure is a silent drop (validate-before-fold), counted in
    /// [`dropped`](FoldState::dropped_count).
    pub fn run(events: &[SourceEvent]) -> Result<Self, FoldError> {
        let mut st = FoldState::default();
        for (i, ev) in events.iter().enumerate() {
            let pos = i as u64;
            match ev {
                SourceEvent::Put(env) => {
                    if env.verify().is_err() {
                        st.dropped += 1;
                        continue;
                    }
                    let rec = records::decode(env).map_err(|e| FoldError::Undecodable(pos, e))?;
                    let id = env.identity_hex();
                    st.by_id
                        .insert(id.clone(), (env.body.author.clone(), rec.clone()));
                    st.positions.insert(id, pos);
                    st.apply(env, &rec, pos);
                }
                SourceEvent::Delete { author, target } => {
                    st.apply_delete(author, target, pos);
                }
            }
        }
        Ok(st)
    }

    fn apply(&mut self, env: &Envelope, rec: &Record, pos: u64) {
        let author = env.body.author.as_str();
        match rec {
            Record::Genesis(g) => {
                self.genesis_ids
                    .insert(g.scope.clone(), env.identity_hex());
                self.catalogue.insert(
                    g.scope.clone(),
                    CatalogueEntry {
                        scope: g.scope.clone(),
                        owner: author.to_string(),
                        title: g.title.clone(),
                        write_policy: g.write_policy,
                        membership_policy: g.membership_policy,
                        steward_set: g.steward_set.clone(),
                        threshold: g.threshold,
                        superseded_by: None,
                        predecessor: None,
                        transition_at: None,
                    },
                );
            }
            Record::SelfRegistration { scope } => {
                // Open tier: one signature (the author's) admits the author.
                if matches!(
                    self.catalogue.get(scope).map(|c| c.membership_policy),
                    Some(MembershipPolicy::Open)
                ) {
                    self.open(scope, author, pos);
                }
            }
            Record::Request { scope } => {
                // The member's half of the two-sided fact. Record it pending;
                // silence never turns it into a verdict.
                self.requests
                    .insert(env.identity_hex(), (scope.clone(), author.to_string()));
            }
            Record::Grant { scope, subject } => {
                // The steward's half. A grant counts only if (a) its author is a
                // steward of the scope and (b) it cites, among its antecedents, a
                // recorded request from `subject` for this scope. Membership opens
                // when distinct steward co-signs reach the scope threshold — at
                // the causal position of the threshold-reaching grant.
                let Some(cat) = self.catalogue.get(scope) else {
                    return;
                };
                if !cat.steward_set.iter().any(|s| s == author) {
                    return; // not a steward — a self-grant does not admit.
                }
                let threshold = cat.threshold.max(1);
                // Find the answered request among the antecedents.
                let request_id = env.body.antecedents.iter().find(|a| {
                    matches!(self.requests.get(*a), Some((rs, rq)) if rs == scope && rq == subject)
                });
                let Some(request_id) = request_id.cloned() else {
                    return; // grant without a matching request antecedent — inert.
                };
                let signers = self.cosigns.entry(request_id.clone()).or_default();
                signers.insert(author.to_string());
                if signers.len() as u32 >= threshold && !self.granted.contains(&request_id) {
                    self.granted.insert(request_id);
                    self.open(scope, subject, pos);
                }
            }
            Record::Leave { scope } => {
                self.close(scope, author, pos);
            }
            Record::Revocation { scope, subject } => {
                self.close(scope, subject, pos);
                self.cuts.insert((scope.clone(), subject.clone()), pos);
            }
            Record::RoleGrant {
                scope,
                subject,
                role,
            } => {
                let entry = self
                    .roles
                    .entry((scope.clone(), subject.clone()))
                    .or_default();
                if !entry.contains(role) {
                    entry.push(role.clone());
                }
            }
            Record::RoleRevoke {
                scope,
                subject,
                role,
            } => {
                if let Some(entry) = self.roles.get_mut(&(scope.clone(), subject.clone())) {
                    entry.retain(|r| r != role);
                }
            }
            Record::Supersession {
                scope,
                predecessor,
                write_policy,
                membership_policy,
            } => {
                // Tier transition as re-plant (P7): the catalogue keeps one
                // continuous identity; the policy fields change at this position.
                // A supersession is governed — only the scope owner may re-plant,
                // and its lineage must name the predecessor genesis.
                if let Some(entry) = self.catalogue.get_mut(scope) {
                    let governed = entry.owner == author;
                    // The lineage must name the predecessor genesis: it is a
                    // known folded record and cited among the antecedents.
                    let lineage_ok = self.positions.contains_key(predecessor)
                        && env.body.antecedents.iter().any(|a| a == predecessor);
                    if governed && lineage_ok {
                        entry.write_policy = *write_policy;
                        entry.membership_policy = *membership_policy;
                        entry.superseded_by = Some(env.identity_hex());
                        entry.predecessor = Some(predecessor.clone());
                        entry.transition_at = Some(pos);
                    }
                }
            }
            // Message chain heads (RUN-18 B1): in a write-restricted scope a
            // folded message advances the author's chain head IF it is chained
            // (first antecedent = the expected link); an unchained message —
            // which validate-before-relay would have dropped — never moves the
            // head. Messages never mutate the roster fold.
            Record::Message { scope, .. } => {
                let restricted = matches!(
                    self.catalogue.get(scope).map(|c| c.write_policy),
                    Some(WritePolicy::Single)
                );
                if restricted {
                    let expected = self
                        .chain_head(scope, author)
                        .or_else(|| self.genesis_id(scope));
                    if env.body.antecedents.first() == expected.as_ref() {
                        self.chain_heads
                            .insert((scope.clone(), author.to_string()), env.identity_hex());
                    }
                }
            }
            // Device attestations are handled by the delegation verifier (P5).
            Record::DeviceAttestation { .. } => {}
        }
    }

    fn apply_delete(&mut self, author: &str, target: &str, pos: u64) {
        if let Some((orig_author, rec)) = self.by_id.get(target).cloned() {
            // A member may delete their own membership-establishing record
            // (self-leave). Deleting someone else's record has no roster effect.
            if orig_author == author {
                match rec {
                    Record::SelfRegistration { scope }
                    | Record::Request { scope }
                    | Record::Grant { scope, .. } => self.close(&scope, author, pos),
                    _ => {}
                }
            }
        }
    }

    fn open(&mut self, scope: &str, did: &str, pos: u64) {
        let iv = self
            .intervals
            .entry((scope.to_string(), did.to_string()))
            .or_default();
        if iv.last().map(|(_, end)| end.is_some()).unwrap_or(true) {
            iv.push((pos, None));
        }
    }

    fn close(&mut self, scope: &str, did: &str, pos: u64) {
        if let Some(iv) = self
            .intervals
            .get_mut(&(scope.to_string(), did.to_string()))
        {
            if let Some(last) = iv.last_mut() {
                if last.1.is_none() {
                    last.1 = Some(pos);
                }
            }
        }
    }

    /// Currently-active members of `scope` (those holding an open interval),
    /// sorted.
    #[must_use]
    pub fn roster_members(&self, scope: &str) -> Vec<String> {
        let mut out: Vec<String> = self
            .intervals
            .iter()
            .filter(|((s, _), iv)| {
                s == scope && iv.last().map(|(_, e)| e.is_none()).unwrap_or(false)
            })
            .map(|((_, d), _)| d.clone())
            .collect();
        out.sort();
        out.dedup();
        out
    }

    /// The membership intervals for a (scope, member) — the interval set (§A.3).
    #[must_use]
    pub fn member_intervals(&self, scope: &str, did: &str) -> Vec<(u64, Option<u64>)> {
        self.intervals
            .get(&(scope.to_string(), did.to_string()))
            .cloned()
            .unwrap_or_default()
    }

    /// The revocation-cut position for a (scope, member), if revoked.
    #[must_use]
    pub fn cut_position(&self, scope: &str, did: &str) -> Option<u64> {
        self.cuts
            .get(&(scope.to_string(), did.to_string()))
            .copied()
    }

    /// The causal (stream) position at which an envelope identity was folded.
    #[must_use]
    pub fn position_of(&self, id: &str) -> Option<u64> {
        self.positions.get(id).copied()
    }

    /// The genesis envelope identity of a scope — the anchor the first chained
    /// envelope of a write-restricted scope must reference (RUN-18 B1).
    #[must_use]
    pub fn genesis_id(&self, scope: &str) -> Option<String> {
        self.genesis_ids.get(scope).cloned()
    }

    /// The author's current chain head in a write-restricted scope: the
    /// identity of their newest chained envelope, if any (RUN-18 B1).
    #[must_use]
    pub fn chain_head(&self, scope: &str, did: &str) -> Option<String> {
        self.chain_heads
            .get(&(scope.to_string(), did.to_string()))
            .cloned()
    }

    /// The status of a request by its identity (§A.3: silence is not a verdict —
    /// an unanswered request stays [`RequestStatus::Pending`] at every fold
    /// point; no timeout path can turn it into a verdict).
    #[must_use]
    pub fn request_status(&self, _scope: &str, request_id: &str) -> RequestStatus {
        if self.granted.contains(request_id) {
            RequestStatus::Granted
        } else if self.requests.contains_key(request_id) {
            RequestStatus::Pending
        } else {
            RequestStatus::Unknown
        }
    }

    /// Decide whether a message envelope is admitted: signature valid, and the
    /// author held membership at the message's causal position — where causal
    /// position is the greatest stream position among the message's antecedents
    /// (a message with no resolvable antecedent is treated as "at the current
    /// tip", so a revoked author cannot smuggle one in after the cut).
    ///
    /// This is "roster-at-position": a message from a revoked member with
    /// antecedents before the cut is admitted; one at/after the cut is not.
    ///
    /// # Errors
    /// Returns the specific [`AdmitReject`] on refusal.
    pub fn admit_message(&self, env: &Envelope) -> Result<(), AdmitReject> {
        if env.verify().is_err() {
            return Err(AdmitReject::BadSignature);
        }
        let rec = records::decode(env).map_err(|_| AdmitReject::NotAMessage)?;
        let scope = match &rec {
            Record::Message { scope, .. } => scope.clone(),
            _ => return Err(AdmitReject::NotAMessage),
        };
        let causal_pos = env
            .body
            .antecedents
            .iter()
            .filter_map(|a| self.positions.get(a).copied())
            .max()
            .unwrap_or(u64::MAX);
        let intervals = self.member_intervals(&scope, &env.body.author);
        let held = intervals
            .iter()
            .any(|(start, end)| causal_pos >= *start && end.is_none_or(|e| causal_pos < e));
        if held {
            Ok(())
        } else {
            Err(AdmitReject::OutsideMembership)
        }
    }

    /// Roles currently held by a (scope, member).
    #[must_use]
    pub fn roles_of(&self, scope: &str, did: &str) -> Vec<String> {
        self.roles
            .get(&(scope.to_string(), did.to_string()))
            .cloned()
            .unwrap_or_default()
    }

    /// Count of envelopes dropped for failing verification.
    #[must_use]
    pub fn dropped_count(&self) -> u64 {
        self.dropped
    }

    /// A byte-stable digest of the catalogue + interval roster. Two folds of the
    /// same event stream (e.g. an indexed fold and a backfill+tail rebuild)
    /// produce the same digest iff they are structurally identical.
    ///
    /// # Panics
    /// Never in practice: the snapshot is owned scalar data.
    #[must_use]
    pub fn canonical_digest(&self) -> String {
        #[derive(Serialize)]
        struct Snapshot<'a> {
            catalogue: &'a BTreeMap<String, CatalogueEntry>,
            intervals: ScopeIntervals,
            roles: ScopeRoles,
        }
        let mut intervals: ScopeIntervals = BTreeMap::new();
        for ((scope, did), iv) in &self.intervals {
            intervals
                .entry(scope.clone())
                .or_default()
                .insert(did.clone(), iv.clone());
        }
        let mut roles: ScopeRoles = BTreeMap::new();
        for ((scope, did), rs) in &self.roles {
            roles
                .entry(scope.clone())
                .or_default()
                .insert(did.clone(), rs.clone());
        }
        let snap = Snapshot {
            catalogue: &self.catalogue,
            intervals,
            roles,
        };
        let bytes =
            crate::canonical::to_canonical(&snap).expect("snapshot of owned data always encodes");
        hex::encode(Sha256::digest(&bytes))
    }
}

/// Convenience alias so tests read `Fold::run`.
pub type Fold = FoldState;

/// The plain-language, DR-style transition banner for a scope, if it has been
/// superseded (P7). Returns `None` for a scope that has not re-planted. The
/// wording is a default template — owner-editable — but its PRESENCE at a
/// transition is the asserted invariant.
#[must_use]
pub fn transition_banner(entry: &CatalogueEntry) -> Option<String> {
    entry.superseded_by.as_ref().map(|_| {
        format!(
            "“{}” has adopted a new membership process. Everything posted before \
             the change stays exactly as it was, and nobody's past membership is \
             affected. From here on, joining works differently — see the scope's \
             notes for what changed. (This message is editable by the scope's \
             stewards.)",
            entry.title
        )
    })
}

/// Write folded ops to an archive (the "state table"): the canonical bytes of
/// the ordered event stream. Persisting the events — not a derived index — is
/// what lets a second folder re-verify every signature on rebuild.
///
/// # Panics
/// Never in practice: `SourceEvent`s of owned data always encode.
#[must_use]
pub fn archive(events: &[SourceEvent]) -> Vec<u8> {
    crate::canonical::to_canonical(&events.to_vec()).expect("events of owned data always encode")
}

/// Rebuild a [`FoldState`] from an archive alone. Signatures are re-verified by
/// [`FoldState::run`] — the archive is verifiable, not trusted.
///
/// # Errors
/// Returns a string on undecodable archive bytes or a structurally invalid
/// record.
pub fn rebuild_from_archive(bytes: &[u8]) -> Result<FoldState, String> {
    let events: Vec<SourceEvent> =
        ciborium::from_reader(bytes).map_err(|e| format!("decode archive: {e}"))?;
    FoldState::run(&events).map_err(|e| e.to_string())
}

/// Test helper: flip a byte in the payload of the first `Put` envelope in an
/// archive, so its signature no longer verifies. Used to prove a rebuild drops
/// a tampered archive entry rather than trusting it.
///
/// # Panics
/// Panics if the archive does not decode or has no `Put` with a payload — it is
/// a test-only helper fed well-formed archives.
#[must_use]
pub fn tamper_first_put_for_test(bytes: &[u8]) -> Vec<u8> {
    let mut events: Vec<SourceEvent> = ciborium::from_reader(bytes).expect("test archive decodes");
    for ev in &mut events {
        if let SourceEvent::Put(env) = ev {
            if let Some(b) = env.body.payload.first_mut() {
                *b ^= 0xff;
                break;
            }
        }
    }
    crate::canonical::to_canonical(&events).expect("events re-encode")
}
