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

// ============================================================================================
// The range-**partitioned** production form (RUN-12 Part 3b) — §6.8.1's open residual.
//
// The whole-set `missing_frames` above compares two peers' full `(device, lamport)` summaries and
// ships the diff in one exchange — O(n) to compare even when the divergence is tiny. Per the RUN-12
// Part 3a brief (`beta/impl/drystone-design/rbsr-construction.md`, recommendation §D), the production
// form for §6.8.1's *linear* `(device, lamport)` key space is a **Negentropy-style one-dimensional
// recursive range reconciler**: fingerprint a range, and where two peers' fingerprints disagree,
// split into count-balanced sub-ranges and recurse into only the disagreeing ones, shipping actual
// records (`IdList`) once a sub-range is small. Bandwidth is proportional to the divergence and the
// round count is O(log n), not O(n).
//
// Experiment-grade and loopback: the fingerprint below is a **test-only** additive monoid over the
// `(device, lamport)` keys — NOT the `[gates-release]` wire fingerprint (Appendix B), which stays
// unpinned. No new dependency: the key-id mixing and 256-bit addition are inline. Real-transport
// loss stays the X1 residual; this proves the partitioned *repair*, at loopback, like the whole-set
// form it generalizes (the whole-set `missing_frames` is the degenerate single-`IdList`-bucket case).
// ============================================================================================

/// A range's **fingerprint**: a Negentropy-style additive monoid — the sum (mod 2^256) of the
/// per-key ids in the range, plus the record count. Two ranges holding the identical key set share a
/// fingerprint; the sum composes over adjacent ranges (an associative, commutative combine), which is
/// what lets an agreeing range terminate with no records shipped. TEST-ONLY: this is not a wire
/// fingerprint (the `[gates-release]` encoding, Appendix B, is unpinned).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct RangeFingerprint {
    /// Sum mod 2^256 of the per-key ids in the range (big-endian).
    pub sum: [u8; 32],
    /// The number of records in the range.
    pub count: u64,
}

impl RangeFingerprint {
    /// The neutral element of the monoid (the empty range).
    #[must_use]
    pub fn neutral() -> Self {
        Self::default()
    }

    /// Combine two range fingerprints (associative and commutative) — the tree-friendly property that
    /// lets a range's fingerprint be assembled from its sub-ranges'.
    #[must_use]
    pub fn combine(self, other: Self) -> Self {
        Self { sum: add256(self.sum, other.sum), count: self.count + other.count }
    }
}

type Key = ([u8; 32], u64);

/// SplitMix64 — a dep-free finaliser used only to spread `(device, lamport)` coordinates into a
/// well-separated 256-bit id for the additive fingerprint. Not cryptographic; the fill is
/// self-verifying regardless (§6.8.1), so the fingerprint's job is efficiency, not integrity.
fn splitmix64(x: u64) -> u64 {
    let mut z = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// A 256-bit id for a `(device, lamport)` key: four lanes seeded from the device chunks mixed with
/// the lamport, so distinct coordinates map to well-separated ids (a same-device, different-lamport
/// pair does not collide).
fn key_id(key: &Key) -> [u8; 32] {
    let (dev, lam) = key;
    let mut out = [0u8; 32];
    for i in 0..4 {
        let lane = u64::from_be_bytes(dev[i * 8..i * 8 + 8].try_into().unwrap());
        let mixed = splitmix64(lane ^ splitmix64(lam.wrapping_add(i as u64)));
        out[i * 8..i * 8 + 8].copy_from_slice(&mixed.to_be_bytes());
    }
    out
}

/// 256-bit big-endian addition (wrapping mod 2^256) — the fingerprint's additive combine.
fn add256(mut acc: [u8; 32], x: [u8; 32]) -> [u8; 32] {
    let mut carry = 0u16;
    for i in (0..32).rev() {
        let s = u16::from(acc[i]) + u16::from(x[i]) + carry;
        acc[i] = s as u8;
        carry = s >> 8;
    }
    acc
}

/// The additive fingerprint over a slice of raw frames (deriving each frame's `(device, lamport)`
/// key). Exposed so a caller can check the tree-friendly composition property: the fingerprint of a
/// set equals the monoid-combination of any partition of it, and two peers holding the identical set
/// produce the identical fingerprint.
#[must_use]
pub fn frame_fingerprint(frames: &[Vec<u8>]) -> RangeFingerprint {
    let mut fp = RangeFingerprint::neutral();
    for f in frames {
        if let Some(k) = assertion_order_key(f) {
            fp = fp.combine(RangeFingerprint { sum: key_id(&k), count: 1 });
        }
    }
    fp
}

/// The fingerprint of a range of `(key, frame)` records.
#[must_use]
fn fingerprint_of(kv: &[(Key, Vec<u8>)]) -> RangeFingerprint {
    let mut fp = RangeFingerprint::neutral();
    for (k, _) in kv {
        fp = fp.combine(RangeFingerprint { sum: key_id(k), count: 1 });
    }
    fp
}

/// A peer's held records for `group`, as `(key, frame)` pairs sorted by `(device, lamport)`.
fn sorted_kv(session: &Session, group: &GroupId) -> Vec<(Key, Vec<u8>)> {
    let mut v: Vec<(Key, Vec<u8>)> = session
        .export_group_log(group)
        .unwrap_or_default()
        .into_iter()
        .filter_map(|bytes| assertion_order_key(&bytes).map(|k| (k, bytes)))
        .collect();
    v.sort_by(|(ka, _), (kb, _)| ka.cmp(kb));
    v
}

/// The outcome of a partitioned reconciliation: the frames each peer must ship the other, the number
/// of range-fingerprint exchange **rounds** (the partition-tree depth — each level is one round-trip),
/// and the total records **shipped** as `IdList`s (proportional to the divergence, not the set size).
#[derive(Clone, Debug, Default)]
pub struct Reconciliation {
    /// Frames `from` holds that `to` lacks (to be folded into `to`).
    pub to_b: Vec<Vec<u8>>,
    /// Frames `to` holds that `from` lacks.
    pub to_a: Vec<Vec<u8>>,
    /// Range-fingerprint exchange rounds (partition-tree depth). O(log n) for a localized divergence.
    pub rounds: usize,
    /// Total records shipped as `IdList`s — equals the divergence, never the whole set.
    pub shipped: usize,
}

/// Reconcile `from` and `to` over `group` with the range-partitioned construction: `branching`
/// count-balanced sub-ranges per split, shipping a range's records once both peers hold at most
/// `id_list_max` in it. Returns the diff to fold plus the round/shipped counts. The diff is identical
/// to [`missing_frames`] (this generalizes the whole-set form), but localized in O(log n) rounds.
#[must_use]
pub fn reconcile_partitioned(
    from: &Session,
    to: &Session,
    group: &GroupId,
    branching: usize,
    id_list_max: usize,
) -> Reconciliation {
    let a = sorted_kv(from, group);
    let b = sorted_kv(to, group);
    let mut out = Reconciliation::default();
    recurse(&a, &b, branching.max(2), id_list_max.max(1), 1, &mut out);
    out
}

/// One reconciliation step over a `(device, lamport)` sub-range: exchange fingerprints (this level is
/// one round); if they agree, stop; if both sides are small, ship the symmetric difference as an
/// `IdList`; else split into `branching` count-balanced sub-ranges over the union key space and recurse.
fn recurse(
    a: &[(Key, Vec<u8>)],
    b: &[(Key, Vec<u8>)],
    branching: usize,
    id_list_max: usize,
    depth: usize,
    out: &mut Reconciliation,
) {
    if a.is_empty() && b.is_empty() {
        return;
    }
    out.rounds = out.rounds.max(depth);

    // Fingerprints agree ⇒ the ranges hold the identical set; terminate, ship nothing.
    if fingerprint_of(a) == fingerprint_of(b) {
        return;
    }

    // Small enough on both sides ⇒ ship the symmetric difference directly (an IdList exchange).
    if a.len() <= id_list_max && b.len() <= id_list_max {
        let aset: BTreeSet<Key> = a.iter().map(|(k, _)| *k).collect();
        let bset: BTreeSet<Key> = b.iter().map(|(k, _)| *k).collect();
        for (k, f) in a {
            if !bset.contains(k) {
                out.to_b.push(f.clone());
                out.shipped += 1;
            }
        }
        for (k, f) in b {
            if !aset.contains(k) {
                out.to_a.push(f.clone());
                out.shipped += 1;
            }
        }
        return;
    }

    // Split the union key space into `branching` count-balanced buckets and recurse into each.
    let mut union: Vec<Key> = a.iter().map(|(k, _)| *k).chain(b.iter().map(|(k, _)| *k)).collect();
    union.sort();
    union.dedup();
    let chunk = union.len().div_ceil(branching);
    let mut start = 0;
    while start < union.len() {
        let end = (start + chunk).min(union.len());
        let lo = union[start];
        let hi = union.get(end).copied();
        let in_range = |k: &Key| *k >= lo && hi.is_none_or(|h| *k < h);
        let a_sub: Vec<(Key, Vec<u8>)> = a.iter().filter(|(k, _)| in_range(k)).cloned().collect();
        let b_sub: Vec<(Key, Vec<u8>)> = b.iter().filter(|(k, _)| in_range(k)).cloned().collect();
        recurse(&a_sub, &b_sub, branching, id_list_max, depth + 1, out);
        start = end;
    }
}
