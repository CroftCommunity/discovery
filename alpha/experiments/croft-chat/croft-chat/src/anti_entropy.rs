//! **Steady-state anti-entropy** — §6.8.1's open half, at loopback grade.
//!
//! Connect-time catch-up is already demonstrated over real iroh-gossip (a late joiner reaches an
//! identical head on `NeighborUp`), but via a **whole-retained-log re-broadcast** — the coarse push
//! cousin of RBSR. The gap this module closes is the other case §6.8.1 names: a live frame lost
//! **between already-connected peers**, with *no new join*, so no `NeighborUp` fires and no
//! re-broadcast is triggered. Because gossip has no per-recipient ack (§6.8), the peer that missed
//! the frame gets **no live signal** it is behind — its `Replicator` buffers nothing (there is no
//! stranded successor waiting on the lost frame). The only thing that surfaces the gap is comparing
//! what each peer *holds*.
//!
//! This is **range-based set reconciliation in its simplest whole-set form**: each peer's held set
//! is summarised by the `(device, lamport)` coordinates it carries (the key space RBSR reconciles
//! over), the two summaries are compared, and **only the diff** is shipped — never the whole log.
//! The range-*partitioned* production construction (Willow 3d-range versus Negentropy) is still the
//! open §5 / Appendix B choice; this is the loopback stand-in that proves the steady-state repair
//! itself, not that construction. No wire format is pinned here.

use std::collections::BTreeSet;

use social_graph_core::{assertion_order_key, GroupId, Session};

/// A **range summary** of what `session` holds for `group`: the set of `(device, lamport)` keys of
/// the frames it carries — the coordinates anti-entropy reconciles over. Comparing two peers'
/// summaries yields the diff without shipping any record. Per-device lamport is strictly monotonic,
/// so a `(device, lamport)` key uniquely names a frame.
#[must_use]
pub fn range_summary(session: &Session, group: &GroupId) -> BTreeSet<([u8; 32], u64)> {
    session
        .export_group_log(group)
        .unwrap_or_default()
        .iter()
        .filter_map(|bytes| assertion_order_key(bytes))
        .collect()
}

/// The **steady-state anti-entropy diff**: the frames `to` is missing relative to `from`, found by
/// comparing range summaries (the RBSR move — compare summaries, then ship only the difference).
/// The result is exactly the lost frames, never the whole log, so this is distinct from the
/// connect-time whole-retained-log re-broadcast. Empty iff the two peers already hold the same set.
///
/// The caller folds the returned frames through the ordinary [`crate::sync::Replicator`] — a repaired
/// frame is accepted on its own author signature and its fold into the derived state, exactly as a
/// live frame is, so this adds a repair *path*, not a second trust rule.
#[must_use]
pub fn missing_frames(from: &Session, to: &Session, group: &GroupId) -> Vec<Vec<u8>> {
    let have = range_summary(to, group);
    from.export_group_log(group)
        .unwrap_or_default()
        .into_iter()
        .filter(|bytes| assertion_order_key(bytes).is_some_and(|k| !have.contains(&k)))
        .collect()
}

/// Whether `from` and `to` hold the identical frame set for `group` — the anti-entropy convergence
/// predicate. A repair is needed exactly when this is false.
#[must_use]
pub fn converged(from: &Session, to: &Session, group: &GroupId) -> bool {
    range_summary(from, group) == range_summary(to, group)
}
