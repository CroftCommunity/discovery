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

use std::collections::BTreeMap;

use serde::Serialize;
use sha2::{Digest, Sha256};

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
    /// If superseded (P7), the successor genesis identity.
    pub superseded_by: Option<String>,
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
    /// dropped (failed-verification) envelope count — observability.
    dropped: u64,
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
                    st.by_id
                        .insert(env.identity_hex(), (env.body.author.clone(), rec.clone()));
                    st.apply(&env.body.author, &rec, pos);
                }
                SourceEvent::Delete { author, target } => {
                    st.apply_delete(author, target, pos);
                }
            }
        }
        Ok(st)
    }

    fn apply(&mut self, author: &str, rec: &Record, pos: u64) {
        match rec {
            Record::Genesis(g) => {
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
            Record::Grant { scope, subject } => {
                // Gated tier: authority + the request antecedent are checked by
                // the caller-side grant validator (P4); the roster fold opens
                // the interval from the grant's causal position.
                self.open(scope, subject, pos);
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
                predecessor: _,
                write_policy,
                membership_policy,
            } => {
                // Tier transition as re-plant (P7): the catalogue keeps one
                // continuous identity; the policy fields change at this position.
                if let Some(entry) = self.catalogue.get_mut(scope) {
                    entry.write_policy = *write_policy;
                    entry.membership_policy = *membership_policy;
                }
            }
            // Requests are tracked by the caller-side gated-tier logic (P4);
            // device attestations by the delegation verifier (P5); messages by
            // the write-policy relay/serve (P3). None mutate the roster fold.
            Record::Request { .. } | Record::DeviceAttestation { .. } | Record::Message { .. } => {}
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
