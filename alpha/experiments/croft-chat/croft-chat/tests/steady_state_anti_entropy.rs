//! Steady-state anti-entropy — §6.8.1's open half, at loopback grade.
//!
//! Earns/bounds: Part 2 §6.8.1 — steady-state anti-entropy (a live frame lost between
//! already-connected peers, no new join) is detected and repaired without a reconnect, and the
//! folds re-converge byte-identically. `Modeled` at loopback grade (whole-set range-compare stand-in;
//! the range-partitioned production construction stays open).
//!
//! Connect-time catch-up is already proven (a late joiner reaches an identical head on `NeighborUp`,
//! via a whole-log re-broadcast). This closes the other case: two peers are **already connected**, a
//! single live frame is lost to one of them, and *no new join occurs*. Because gossip carries no
//! per-recipient ack (§6.8), the peer that missed the frame gets **no live signal** — its
//! `Replicator` buffers nothing (there is no stranded successor). The gap is invisible to live
//! delivery and surfaces only by comparing what each peer holds. The repair ships **only the diff**
//! (not the whole log — that is the connect-time re-broadcast), so the two peers re-converge
//! byte-identically without a reconnect.
//!
//! Falsifies if: the lost frame leaves a live signal after all (defeating the premise), or the
//! anti-entropy diff fails to detect / repair the gap, or the repair does not re-converge the folds.

mod common;

use common::{pump_until_quiet, QueueBus};
use croft_chat::anti_entropy::{converged, missing_frames, range_summary};
use croft_chat::fingerprint::{fingerprint, fingerprint_lines};
use croft_chat::sync::Replicator;
use croft_chat::transport::Frame;
use social_graph_core::{GroupId, Identity, PrincipalId, Role, Session};

/// Fold `frames` into `session` through its persistent `Replicator` (per-device lamport order), the
/// same path a live frame takes. The replicator persists so a mid-chain frame delivered on its own
/// (an anti-entropy repair) folds against the applied-state the receiver already holds.
fn deliver(frames: Vec<Vec<u8>>, session: &Session, repl: &mut Replicator) {
    let mut bus = QueueBus::default();
    bus.inject(frames.into_iter().map(Frame).collect());
    pump_until_quiet(session, &mut bus, repl);
}

/// Ship `to` exactly the frames it lacks from `from` (the receiver-lacks diff) and fold them —
/// modelling both live delivery (each new frame as produced) and, later, the anti-entropy repair.
/// Shipping only the diff keeps the persistent replicator's buffer clean (no re-injected
/// already-applied frames), so `is_settled` stays an honest incompleteness signal.
fn sync_missing(from: &Session, to: &Session, group: &GroupId, repl: &mut Replicator) {
    deliver(missing_frames(from, to, group), to, repl);
}

#[tokio::test]
async fn a_frame_lost_between_connected_peers_is_repaired_without_reconnect() {
    croft_chat::init_tracing();

    let id_a = Identity::from_seed([0xA1; 32]);
    let id_b = Identity::from_seed([0xB2; 32]);
    let dir_a = tempfile::tempdir().expect("dir a");
    let dir_b = tempfile::tempdir().expect("dir b");
    let session_a = Session::open(&dir_a.path().join("a.redb"), &id_a).expect("open A");
    let session_b = Session::open(&dir_b.path().join("b.redb"), &id_b).expect("open B");
    session_a.trust_peer(id_b.device_id(), id_b.principal_id());
    session_b.trust_peer(id_a.device_id(), id_a.principal_id());

    let mut repl_a = Replicator::new();
    let mut repl_b = Replicator::new();

    // A founds the group and enrolls B; B learns the group and its own membership.
    let group: GroupId = session_a.create_group().await.expect("create_group");
    let b_principal = PrincipalId::new(id_b.principal_id().0);
    session_a.add_member(&group, b_principal, Role::Member).await.expect("add B");
    sync_missing(&session_a, &session_b, &group, &mut repl_b);
    assert!(
        session_b.get_group_summary(&group).is_ok_and(|s| s.members.iter().any(|m| m.principal == b_principal)),
        "B is a member after the initial exchange"
    );

    // Steady state: both post, both sides converge. Now they are connected and in sync.
    session_a.send_message(&group, "a-1", None).await.expect("a-1");
    session_b.send_message(&group, "b-1", None).await.expect("b-1");
    sync_missing(&session_a, &session_b, &group, &mut repl_b);
    sync_missing(&session_b, &session_a, &group, &mut repl_a);
    assert_eq!(fingerprint(&session_a, &group), fingerprint(&session_b, &group), "steady state converged");
    assert!(converged(&session_a, &session_b, &group), "peers hold the identical frame set");
    assert!(repl_b.is_settled(), "B is settled after steady-state convergence");

    // A live frame is LOST to B mid-session — the harness simply never delivers A's next frame, and
    // there is no new join, so nothing re-broadcasts and no successor strands in B's buffer.
    session_a.send_message(&group, "a-lost", None).await.expect("a-lost");
    // (deliberately not synced to B)

    // B has NO live signal it is behind: its replicator buffers nothing (no stranded successor,
    // gossip carries no per-recipient ack), yet the folds have diverged.
    assert!(repl_b.is_settled(), "B's replicator buffers nothing — the gap is invisible to live delivery");
    assert_ne!(
        fingerprint(&session_a, &group),
        fingerprint(&session_b, &group),
        "the lost frame diverged the folds, invisibly to live delivery"
    );

    // Anti-entropy DETECTS the gap by comparing range summaries (not by re-broadcasting).
    assert!(!converged(&session_a, &session_b, &group), "the summary compare detects the gap");
    let diff = missing_frames(&session_a, &session_b, &group);
    assert_eq!(diff.len(), 1, "exactly the lost frame is the diff — only the gap is shipped, not the log");
    assert!(
        diff.len() < session_a.export_group_log(&group).expect("export A").len(),
        "the repair ships the diff, not the whole retained log (unlike connect-time catch-up)"
    );

    // REPAIR: fold only the diff into B, with no reconnect and no whole-log re-broadcast.
    deliver(diff, &session_b, &mut repl_b);

    // The folds re-converge byte-identically.
    let fa = fingerprint(&session_a, &group);
    let fb = fingerprint(&session_b, &group);
    if fa != fb {
        for line in fingerprint_lines(&session_a, &group) {
            if !fingerprint_lines(&session_b, &group).contains(&line) {
                eprintln!("only on A: {line}");
            }
        }
        panic!("anti-entropy did not re-converge the folds");
    }
    assert!(converged(&session_a, &session_b, &group), "peers hold the identical frame set again");
    assert_eq!(
        range_summary(&session_a, &group).len(),
        range_summary(&session_b, &group).len(),
        "and the same number of frames"
    );

    eprintln!(
        "STEADY-STATE ANTI-ENTROPY: a frame lost between two connected peers left B settled-but-behind \
         (no live signal); a range-summary compare detected the 1-frame gap and a diff-only repair \
         re-converged the folds byte-identically, with no reconnect and no whole-log re-broadcast."
    );
}
