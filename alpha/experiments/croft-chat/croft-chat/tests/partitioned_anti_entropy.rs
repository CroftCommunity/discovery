//! §6.8.1 — the range-**partitioned** steady-state anti-entropy construction (RUN-12 Part 3b).
//!
//! Earns/bounds: Part 2 §6.8.1 — the production form of steady-state anti-entropy, per the RUN-12
//! Part 3a brief (`beta/impl/drystone-design/rbsr-construction.md`, recommended construction §D): a
//! Negentropy-style one-dimensional recursive range reconciler over the `(device, lamport)` key
//! space, replacing the RUN-09 whole-set `missing_frames` diff. `Modeled` at loopback grade — the
//! fingerprint is test-only (the `[gates-release]` wire fingerprint stays unpinned), and real-transport
//! loss is the X1 residual, exactly as for the whole-set form it generalizes.
//!
//! The RED-able assertion set (brief §F), landed here:
//!   1. **diff-only equivalence** — the partitioned reconciler ships exactly the whole-set diff for a
//!      single-frame gap, and the folds re-converge (no regression of the RUN-09 result);
//!   2. **fingerprint composition** — a range fingerprint equals the monoid-combination of its
//!      sub-ranges', and two peers holding the identical set produce the identical fingerprint;
//!   3. **bandwidth proportional to divergence** — records shipped equals the divergence, not the set;
//!   4. **scale — O(log)-ish rounds** — a large divergent range repaired in a round count bounded by
//!      ~log_B(N), strictly fewer than the whole-set O(N) exchange (asserted as a round count, not
//!      wall-clock);
//!   5. **convergence predicate holds after repair**.

mod common;

use common::{pump_until_quiet, QueueBus};
use croft_chat::anti_entropy::{
    converged, frame_fingerprint, missing_frames, reconcile_partitioned, RangeFingerprint,
};
use croft_chat::fingerprint::fingerprint;
use croft_chat::sync::Replicator;
use croft_chat::transport::Frame;
use social_graph_core::{GroupId, Identity, PrincipalId, Role, Session};

/// Fold `frames` into `session` through its persistent `Replicator` (per-device lamport order).
fn deliver(frames: Vec<Vec<u8>>, session: &Session, repl: &mut Replicator) {
    let mut bus = QueueBus::default();
    bus.inject(frames.into_iter().map(Frame).collect());
    pump_until_quiet(session, &mut bus, repl);
}

/// Ship `to` exactly the frames it lacks from `from` (the whole-set diff) and fold them — used to
/// bring the two peers into steady state before the divergence is introduced.
fn sync_missing(from: &Session, to: &Session, group: &GroupId, repl: &mut Replicator) {
    deliver(missing_frames(from, to, group), to, repl);
}

/// (1) diff-only equivalence + (5) convergence after repair: a single lost frame is localized and
/// shipped by the partitioned reconciler exactly as the whole-set form would, and the folds
/// re-converge.
#[tokio::test]
async fn partitioned_repairs_a_single_gap_like_the_whole_set_form() {
    croft_chat::init_tracing();
    let id_a = Identity::from_seed([0xA1; 32]);
    let id_b = Identity::from_seed([0xB2; 32]);
    let dir_a = tempfile::tempdir().expect("dir a");
    let dir_b = tempfile::tempdir().expect("dir b");
    let session_a = Session::open(&dir_a.path().join("a.redb"), &id_a).expect("open A");
    let session_b = Session::open(&dir_b.path().join("b.redb"), &id_b).expect("open B");
    session_a.trust_peer(id_b.device_id(), id_b.principal_id());
    session_b.trust_peer(id_a.device_id(), id_a.principal_id());
    let mut repl_b = Replicator::new();

    let group: GroupId = session_a.create_group().await.expect("create_group");
    let b_principal = PrincipalId::new(id_b.principal_id().0);
    session_a.add_member(&group, b_principal, Role::Member).await.expect("add B");
    for i in 0..8 {
        session_a.send_message(&group, &format!("m-{i}"), None).await.expect("send");
    }
    sync_missing(&session_a, &session_b, &group, &mut repl_b);
    assert!(converged(&session_a, &session_b, &group), "steady state before the gap");

    // A live frame is lost to B (never delivered), diverging the folds invisibly.
    session_a.send_message(&group, "lost", None).await.expect("lost");
    assert!(!converged(&session_a, &session_b, &group), "the lost frame diverged the peers");

    // The partitioned reconciler localizes and ships exactly the whole-set diff.
    let whole_set = missing_frames(&session_a, &session_b, &group);
    let r = reconcile_partitioned(&session_a, &session_b, &group, 4, 2);
    assert_eq!(r.to_b.len(), 1, "exactly the one lost frame is shipped");
    assert_eq!(r.shipped, 1, "bandwidth == the divergence (one record)");
    assert_eq!(r.to_a.len(), 0, "B has nothing A lacks");
    let mut ws: Vec<Vec<u8>> = whole_set.clone();
    let mut rb: Vec<Vec<u8>> = r.to_b.clone();
    ws.sort();
    rb.sort();
    assert_eq!(rb, ws, "the partitioned diff equals the whole-set diff (no regression)");

    // Repair and re-converge.
    deliver(r.to_b, &session_b, &mut repl_b);
    assert_eq!(fingerprint(&session_a, &group), fingerprint(&session_b, &group), "folds re-converge");
    assert!(converged(&session_a, &session_b, &group), "convergence predicate holds after repair");

    eprintln!("PARTITIONED RBSR (1,5): single-frame gap localized, 1 record shipped, folds re-converged; diff == whole-set diff.");
}

/// (2) fingerprint composition: a range fingerprint equals the combination of its sub-ranges', and
/// two peers holding the identical set produce the identical fingerprint.
#[tokio::test]
async fn fingerprint_is_a_composing_monoid_and_agrees_on_equal_sets() {
    croft_chat::init_tracing();
    let id_a = Identity::from_seed([0xA3; 32]);
    let id_b = Identity::from_seed([0xB4; 32]);
    let dir_a = tempfile::tempdir().expect("dir a");
    let dir_b = tempfile::tempdir().expect("dir b");
    let session_a = Session::open(&dir_a.path().join("a.redb"), &id_a).expect("open A");
    let session_b = Session::open(&dir_b.path().join("b.redb"), &id_b).expect("open B");
    session_a.trust_peer(id_b.device_id(), id_b.principal_id());
    session_b.trust_peer(id_a.device_id(), id_a.principal_id());
    let mut repl_b = Replicator::new();

    let group: GroupId = session_a.create_group().await.expect("create_group");
    let b_principal = PrincipalId::new(id_b.principal_id().0);
    session_a.add_member(&group, b_principal, Role::Member).await.expect("add B");
    for i in 0..10 {
        session_a.send_message(&group, &format!("c-{i}"), None).await.expect("send");
    }
    sync_missing(&session_a, &session_b, &group, &mut repl_b);

    let frames = session_a.export_group_log(&group).expect("export");
    let whole = frame_fingerprint(&frames);

    // Composition: any partition of the set combines back to the whole fingerprint. Split several ways.
    for split in [1, 3, frames.len() / 2, frames.len() - 1] {
        let (h1, h2) = frames.split_at(split.min(frames.len()));
        let combined = frame_fingerprint(h1).combine(frame_fingerprint(h2));
        assert_eq!(combined, whole, "fingerprint composes over the partition at {split}");
    }
    // Commutative: combining the halves in the other order is identical.
    let (h1, h2) = frames.split_at(frames.len() / 2);
    assert_eq!(
        frame_fingerprint(h1).combine(frame_fingerprint(h2)),
        frame_fingerprint(h2).combine(frame_fingerprint(h1)),
        "the combine is commutative"
    );
    assert_eq!(RangeFingerprint::neutral().combine(whole), whole, "neutral is the identity");

    // Equal sets ⇒ equal fingerprint (this is what lets an agreeing range terminate).
    let b_frames = session_b.export_group_log(&group).expect("export B");
    assert_eq!(frame_fingerprint(&b_frames), whole, "two peers with the identical set agree");
    assert_eq!(whole.count, frames.len() as u64, "the count term equals the record count");

    eprintln!("PARTITIONED RBSR (2): fingerprint is a composing, commutative monoid; equal sets agree.");
}

/// (3) bandwidth proportional to divergence + (4) O(log)-ish rounds for a large divergent range,
/// asserted as a round count (not wall-clock), strictly fewer than the whole-set O(N) exchange.
#[tokio::test]
async fn large_divergent_range_repairs_in_log_rounds() {
    croft_chat::init_tracing();
    let id_a = Identity::from_seed([0xA5; 32]);
    let id_b = Identity::from_seed([0xB6; 32]);
    let dir_a = tempfile::tempdir().expect("dir a");
    let dir_b = tempfile::tempdir().expect("dir b");
    let session_a = Session::open(&dir_a.path().join("a.redb"), &id_a).expect("open A");
    let session_b = Session::open(&dir_b.path().join("b.redb"), &id_b).expect("open B");
    session_a.trust_peer(id_b.device_id(), id_b.principal_id());
    session_b.trust_peer(id_a.device_id(), id_a.principal_id());
    let mut repl_b = Replicator::new();

    // A large shared prefix, then a contiguous divergent block B never receives.
    const SHARED: usize = 60;
    const DIVERGENCE: usize = 8;
    const BRANCHING: usize = 4;

    let group: GroupId = session_a.create_group().await.expect("create_group");
    let b_principal = PrincipalId::new(id_b.principal_id().0);
    session_a.add_member(&group, b_principal, Role::Member).await.expect("add B");
    for i in 0..SHARED {
        session_a.send_message(&group, &format!("s-{i}"), None).await.expect("send");
    }
    sync_missing(&session_a, &session_b, &group, &mut repl_b);
    assert!(converged(&session_a, &session_b, &group), "the shared prefix is in sync");

    // The divergent block: authored by A, never delivered to B.
    for i in 0..DIVERGENCE {
        session_a.send_message(&group, &format!("d-{i}"), None).await.expect("send");
    }
    let total = session_a.export_group_log(&group).expect("export").len();

    let r = reconcile_partitioned(&session_a, &session_b, &group, BRANCHING, 4);

    // (3) bandwidth == divergence, not the whole set.
    assert_eq!(r.shipped, DIVERGENCE, "shipped exactly the divergence, not the {total}-record set");
    assert_eq!(r.to_b.len(), DIVERGENCE, "the diff is exactly the divergent block");

    // (4) round count is O(log_B(total)), and far below the whole-set O(N) exchange.
    let log_bound = (total as f64).log(BRANCHING as f64).ceil() as usize + 3;
    assert!(
        r.rounds <= log_bound,
        "rounds {} within the ~log_{}({}) bound {}",
        r.rounds, BRANCHING, total, log_bound
    );
    assert!(r.rounds < total, "rounds {} strictly below the whole-set O(N)={} exchange", r.rounds, total);

    // (5) repair re-converges.
    deliver(r.to_b, &session_b, &mut repl_b);
    assert!(converged(&session_a, &session_b, &group), "the partitioned repair re-converged the peers");

    eprintln!(
        "PARTITIONED RBSR (3,4,5): a {DIVERGENCE}-record divergence in a {total}-record set repaired in \
         {} rounds (<= ~log_{BRANCHING}({total})+3), shipping only the {DIVERGENCE} divergent records; re-converged.",
        r.rounds
    );
}
