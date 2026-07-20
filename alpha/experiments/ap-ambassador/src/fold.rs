//! The interval fold — P3.
//!
//! Follow opens an interval; Undo Follow closes it; a re-Follow opens a
//! second. The serve-gate honors the interval at the causal position (a
//! request positioned inside a closed interval is refused; inside an open
//! one, served). Ordering is by antecedents and the fold, never by
//! received-at metadata — the covert-clock red test (T-AP3.4) permutes
//! delivery order and asserts the roster is identical.

use std::collections::BTreeMap;

use crate::records::ReceiptRecord;
use crate::types::*;

/// An interval `[open, close?]` — `open` is the ReceiptId of the Follow
/// receipt; `close` is `Some(receipt_id_of_undo)` if an Undo Follow has
/// closed it, or `None` if still open.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Interval {
    pub open: ReceiptId,
    pub close: Option<ReceiptId>,
}

/// The rolling roster the fold derives: per (actor → target) pair, the list
/// of intervals in causal order (oldest first).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FollowerRoster {
    inner: BTreeMap<(ActorId, String), Vec<Interval>>,
}

impl FollowerRoster {
    /// Intervals for a specific (actor → target) pair, oldest first.
    pub fn intervals(&self, actor: &ActorId, target: &str) -> &[Interval] {
        static EMPTY: Vec<Interval> = Vec::new();
        self.inner
            .get(&(actor.clone(), target.to_string()))
            .map(|v| v.as_slice())
            .unwrap_or(EMPTY.as_slice())
    }

    /// The full set of (actor → target) pairs the fold has ever seen. Ordering
    /// is BTreeMap order — deterministic, not delivery-order.
    pub fn pairs(&self) -> impl Iterator<Item = (&ActorId, &str)> {
        self.inner
            .keys()
            .map(|(a, t)| (a, t.as_str()))
    }

    /// Serve-gate: is (actor → target) currently being followed, at the
    /// causal position AFTER the fold has consumed everything given? An
    /// open (unclosed) interval means yes.
    ///
    /// Well-formed input has at most one open interval per pair (a second
    /// Follow that isn't preceded by an Undo of the first is legitimate —
    /// see T-AP3.3 — because a party may re-Follow only after Undoing).
    /// The check is `any close-less`, not `.last().close.is_none()`, so
    /// the answer does not depend on the intervals' emit-order under sort.
    pub fn is_currently_following(&self, actor: &ActorId, target: &str) -> bool {
        self.intervals(actor, target)
            .iter()
            .any(|i| i.close.is_none())
    }

    /// Number of open intervals for (actor → target) — should be 0 or 1
    /// under well-formed input.
    pub fn open_count(&self, actor: &ActorId, target: &str) -> usize {
        self.intervals(actor, target)
            .iter()
            .filter(|i| i.close.is_none())
            .count()
    }
}

/// Fold a set of receipt records into a follower roster. Deterministic:
/// same set, same roster, regardless of iteration order (T-AP3.4 covert-clock
/// red test).
///
/// **The set** — the fold ingests the set, not a sequence. Any callable
/// input (Vec, HashSet, iterator in random order) folds to the same result.
///
/// Ordering within the fold is by (`ReceiptId`), NOT by any received-at
/// metadata (there is no such field in the record on purpose — receipts
/// carry no wall-clock).
///
/// Follow/Undo pairing: an Undo Follow's `undoes` field names the ReceiptId
/// of the Follow it closes. If a matching open interval is found, it is
/// closed; otherwise the Undo is a no-op (an Undo without a matching open
/// Follow does not open a new interval and does not create a "phantom"
/// close — silence is silence).
pub fn fold_roster<'a, I>(records: I) -> FollowerRoster
where
    I: IntoIterator<Item = &'a ReceiptRecord>,
{
    // Bucket by ReceiptId — that's the deterministic order.
    let mut by_id: BTreeMap<[u8; 32], &ReceiptRecord> = BTreeMap::new();
    for r in records {
        by_id.insert(r.receipt_id().0, r);
    }

    // Two passes: first, open every Follow's interval. Second, close by
    // Undo, matching on the `undoes` field.
    let mut roster = FollowerRoster::default();

    // Track (Follow ReceiptId) → (actor, target) index for close lookups.
    let mut follow_index: BTreeMap<[u8; 32], (ActorId, String)> = BTreeMap::new();

    // Pass 1 — Follow opens.
    for (id_bytes, r) in &by_id {
        if r.kind == ActivityKind::Follow {
            let key = (r.actor.clone(), r.object.clone());
            let list = roster.inner.entry(key.clone()).or_default();
            list.push(Interval {
                open: ReceiptId(*id_bytes),
                close: None,
            });
            follow_index.insert(*id_bytes, key);
        }
    }

    // Pass 2 — UndoFollow closes.
    for (undo_id, r) in &by_id {
        if r.kind != ActivityKind::UndoFollow {
            continue;
        }
        let Some(target_follow_id) = r.undoes else {
            // Malformed Undo (no target) — ignore.
            continue;
        };
        let Some(key) = follow_index.get(&target_follow_id.0) else {
            // Undo pointing at a Follow we never saw — no-op.
            continue;
        };
        if let Some(list) = roster.inner.get_mut(key) {
            // Close the specific interval opened by `target_follow_id`.
            for iv in list.iter_mut() {
                if iv.open == target_follow_id && iv.close.is_none() {
                    iv.close = Some(ReceiptId(*undo_id));
                    break;
                }
            }
        }
    }

    // Sort each pair's interval list by its open ReceiptId for
    // determinism-of-output (order-of-arrival cannot influence).
    for list in roster.inner.values_mut() {
        list.sort_by_key(|iv| iv.open.0);
    }

    roster
}
