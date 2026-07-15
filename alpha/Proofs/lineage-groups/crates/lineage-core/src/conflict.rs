//! Conflict detection for reconnect (invariant I6).
//!
//! Under partition, two branches that share a genesis can both make validly-
//! signed but contradictory membership decisions. The correct resolution is
//! never to algorithmically pick a winner — that is a losing battle against
//! social complexity, explicitly out of scope (thesis §1.4). Instead:
//!
//! * no conflict  -> heal silently;
//! * genuine conflict (one side booted someone the other still includes)
//!   -> hard-stop and escalate to a human via a callback.
//!
//! The detector only *classifies*; the escalation hook is where a human (or a
//! quorum-approved override) decides. Conflict is promoted from a failure mode
//! to a feature: the two groups simply remain two groups, each with intact
//! lineage.

use std::collections::{BTreeMap, BTreeSet};

use crate::gov::{Directory, GroupState, OpKind, SignedOp};
use crate::ids::Did;

/// A specific contradiction found between two reconnecting branches.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConflictReason {
    /// One side removed this member; the other still includes them.
    RemovedThenIncluded(Did),
    /// One side dissolved the group; the other kept operating (corpus C7). The
    /// group cannot be both gone and alive — never auto-resolved.
    DissolvedThenContinued,
}

/// The outcome of reconnect analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Resolution {
    /// No contradiction — safe to heal silently.
    Heal,
    /// Genuine membership conflict; must escalate, never auto-resolve.
    HardStop(Vec<ConflictReason>),
}

/// Receives conflicts for human (or quorum) adjudication. Implementations must
/// not silently re-admit removed members.
pub trait Escalator {
    fn on_conflict(&mut self, reason: &ConflictReason);
}

/// Members explicitly *removed* (booted) on `state`. A consensual `Leave` is
/// not a contradiction and is intentionally excluded.
fn booted(state: &GroupState) -> BTreeSet<Did> {
    state
        .log
        .iter()
        .filter(|op| op.body.kind == OpKind::Remove)
        .filter_map(|op| op.body.subject.clone())
        .collect()
}

/// Classify a reconnect between two branch states. Deterministic: both honest
/// clients compute the same `Resolution` (reasons are sorted).
pub fn detect(left: &GroupState, right: &GroupState) -> Resolution {
    let mut reasons = BTreeSet::new();
    for did in booted(left) {
        if right.members.contains(&did) {
            reasons.insert(ConflictReason::RemovedThenIncluded(did));
        }
    }
    for did in booted(right) {
        if left.members.contains(&did) {
            reasons.insert(ConflictReason::RemovedThenIncluded(did));
        }
    }
    // C7: one side dissolved the group, the other kept it alive — a contradiction
    // that must escalate, never silently pick "gone" or "alive".
    if left.dissolved != right.dissolved {
        reasons.insert(ConflictReason::DissolvedThenContinued);
    }
    if reasons.is_empty() {
        Resolution::Heal
    } else {
        Resolution::HardStop(reasons.into_iter().collect())
    }
}

/// Detect, and on a hard-stop route every reason to `esc`. Returns the
/// resolution unchanged — it never re-admits or excludes anyone itself.
pub fn reconcile<E: Escalator>(left: &GroupState, right: &GroupState, esc: &mut E) -> Resolution {
    let resolution = detect(left, right);
    if let Resolution::HardStop(reasons) = &resolution {
        for r in reasons {
            esc.on_conflict(r);
        }
    }
    resolution
}

// --- quorum override (experiment #5) ---------------------------------------
//
// `detect`/`reconcile` only ever *classify* and escalate; they never change
// membership. The escalation hook is where a human or an admin quorum decides.
// This is the structured form of that decision: a quorum may override a
// hard-stop, but **only** by producing an explicit, threshold-meeting *signed*
// governance op for each contested member. Below threshold — or with no
// decision at all — the hard-stop stands. The algorithm still never picks a
// winner; it only checks that the humans signed for one.

/// An admin quorum's explicit decision for one contested member. This is what
/// the escalation hook yields; it is never synthesized by the detector.
#[derive(Debug, Clone)]
pub enum Decision {
    /// Re-admit the contested member via an explicit, signed `Add`.
    Readmit(SignedOp),
    /// Confirm the removal stands via an explicit, signed `Remove` (the branch
    /// that still included the member adopts the boot).
    ConfirmRemoval(SignedOp),
}

impl Decision {
    fn op(&self) -> &SignedOp {
        match self {
            Decision::Readmit(op) | Decision::ConfirmRemoval(op) => op,
        }
    }
    fn expected_kind(&self) -> OpKind {
        match self {
            Decision::Readmit(_) => OpKind::Add,
            Decision::ConfirmRemoval(_) => OpKind::Remove,
        }
    }
}

/// The result of applying a quorum override to a set of conflicts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OverrideOutcome {
    /// Every conflict was resolved by an authorized, threshold-meeting decision.
    /// Carries the per-member governed outcome (the kind the quorum signed for).
    Resolved(Vec<(Did, OpKind)>),
    /// One or more conflicts lacked an authorized decision; the hard-stop stands
    /// for exactly those reasons (no partial, silent resolution).
    Unresolved(Vec<ConflictReason>),
}

/// Authorize (or refuse) a quorum override of a hard-stop. For each conflict
/// reason there must be a `Decision` whose signed op (a) names the contested
/// member as its subject, (b) is of the matching kind (Readmit→Add,
/// ConfirmRemoval→Remove), and (c) **meets the genesis threshold** when its
/// signatures are counted against `group`'s current admins-with-standing.
///
/// Returns [`OverrideOutcome::Resolved`] only if *every* reason is so
/// authorized; otherwise [`OverrideOutcome::Unresolved`] with the reasons that
/// were not. It is all-or-nothing and it never mutates membership — applying
/// the decision is an ordinary governance `apply`, deliberately out of scope.
pub fn quorum_override(
    reasons: &[ConflictReason],
    decisions: &BTreeMap<Did, Decision>,
    group: &GroupState,
    dir: &Directory,
) -> OverrideOutcome {
    let mut resolved = Vec::new();
    let mut unresolved = Vec::new();
    for reason in reasons {
        // C7 (dissolve-vs-continue) has no per-member decision shape; the
        // per-member quorum override cannot clear it, so the hard-stop stands.
        let did = match reason {
            ConflictReason::RemovedThenIncluded(did) => did,
            ConflictReason::DissolvedThenContinued => {
                unresolved.push(reason.clone());
                continue;
            }
        };
        match decisions.get(did) {
            Some(decision)
                if decision.op().body.subject.as_ref() == Some(did)
                    && decision.op().body.kind == decision.expected_kind()
                    && group.meets_threshold(decision.op(), dir) =>
            {
                resolved.push((did.clone(), decision.expected_kind()));
            }
            _ => unresolved.push(reason.clone()),
        }
    }
    if unresolved.is_empty() {
        OverrideOutcome::Resolved(resolved)
    } else {
        OverrideOutcome::Unresolved(unresolved)
    }
}
