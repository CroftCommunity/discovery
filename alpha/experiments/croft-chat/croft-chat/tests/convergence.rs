//! P7 — headless convergence proof (Milestone A gate).
//!
//! Two principals, two redb stores, one shared-directory transport that
//! deliberately scrambles delivery order. They create a group, exchange
//! membership, interleave messages from both sides, and must derive
//! byte-identical state (invariant I5) — without any UI and without a network.
//! This is the wiring test for the whole substrate→session→transport stack.

use croft_chat::fingerprint::{fingerprint, fingerprint_lines};
use croft_chat::shared_dir::SharedDirBus;
use croft_chat::sync::Replicator;
use croft_chat::transport::{Topic, Transport};
use social_graph_core::{Identity, PrincipalId, Role, Session, TimelineWindow};

/// Run publish+pump on both sides until two consecutive rounds apply nothing
/// new (or the bound is hit). Returns true if it quiesced.
fn converge(
    topic: &Topic,
    group: &social_graph_core::GroupId,
    a: (&Session, &mut SharedDirBus, &mut Replicator),
    b: (&Session, &mut SharedDirBus, &mut Replicator),
) -> bool {
    let (sa, bus_a, repl_a) = a;
    let (sb, bus_b, repl_b) = b;
    let mut quiet = 0;
    for _ in 0..50 {
        repl_a.publish_group(sa, bus_a, topic, group).expect("A publish");
        repl_b.publish_group(sb, bus_b, topic, group).expect("B publish");
        let applied = repl_a.pump(sa, bus_a) + repl_b.pump(sb, bus_b);
        if applied == 0 {
            quiet += 1;
            if quiet >= 2 {
                return true;
            }
        } else {
            quiet = 0;
        }
    }
    false
}

#[tokio::test]
async fn two_nodes_converge_regardless_of_arrival_order() {
    croft_chat::init_tracing();

    let bus_root = tempfile::tempdir().expect("bus root");
    let topic = Topic("drystone/demo".to_string());

    let id_a = Identity::from_seed([0xA1; 32]);
    let id_b = Identity::from_seed([0xB2; 32]);

    let dir_a = tempfile::tempdir().expect("dir a");
    let dir_b = tempfile::tempdir().expect("dir b");
    let session_a = Session::open(&dir_a.path().join("a.redb"), &id_a).expect("open A");
    let session_b = Session::open(&dir_b.path().join("b.redb"), &id_b).expect("open B");

    // Each node must verify + resolve the other's authorship.
    session_a.trust_peer(id_b.device_id(), id_b.principal_id());
    session_b.trust_peer(id_a.device_id(), id_a.principal_id());

    let mut bus_a = SharedDirBus::new(bus_root.path(), "A");
    let mut bus_b = SharedDirBus::new(bus_root.path(), "B");
    bus_a.subscribe(&topic);
    bus_b.subscribe(&topic);
    let mut repl_a = Replicator::new();
    let mut repl_b = Replicator::new();

    // A creates the group and enrolls B.
    let group = session_a.create_group().await.expect("create_group");
    let b_principal = PrincipalId::new(id_b.principal_id().0);
    session_a
        .add_member(&group, b_principal, Role::Member)
        .await
        .expect("add_member B");

    // Phase 1: replicate so B learns the group and its own membership.
    assert!(
        converge(
            &topic,
            &group,
            (&session_a, &mut bus_a, &mut repl_a),
            (&session_b, &mut bus_b, &mut repl_b),
        ),
        "phase 1 must quiesce"
    );
    let b_summary = session_b.get_group_summary(&group).expect("B sees group");
    assert!(
        b_summary.members.iter().any(|m| m.principal == b_principal),
        "B is a member on B after replication"
    );

    // Both sides post, interleaved. (B is now authorized to send.)
    session_a.send_message(&group, "a-1", None).await.expect("a-1");
    session_b.send_message(&group, "b-1", None).await.expect("b-1");
    session_a.send_message(&group, "a-2", None).await.expect("a-2");
    session_b.send_message(&group, "b-2", None).await.expect("b-2");

    // Phase 2: converge the interleaved messages.
    assert!(
        converge(
            &topic,
            &group,
            (&session_a, &mut bus_a, &mut repl_a),
            (&session_b, &mut bus_b, &mut repl_b),
        ),
        "phase 2 must quiesce"
    );

    // The headline claim: byte-identical derived state on both nodes.
    let fa = fingerprint(&session_a, &group);
    let fb = fingerprint(&session_b, &group);
    if fa != fb {
        let la = fingerprint_lines(&session_a, &group);
        let lb = fingerprint_lines(&session_b, &group);
        eprintln!("--- divergence: A has {} lines, B has {} ---", la.len(), lb.len());
        for line in &la {
            if !lb.contains(line) {
                eprintln!("only on A: {line}");
            }
        }
        for line in &lb {
            if !la.contains(line) {
                eprintln!("only on B: {line}");
            }
        }
        panic!("nodes diverged");
    }

    // Sanity: all four messages converged on both nodes.
    let count_messages = |s: &Session| {
        s.get_timeline(&group, TimelineWindow::LastN(usize::MAX))
            .expect("timeline")
            .entries
            .iter()
            .filter(|e| s.get_message(&e.hash).is_some())
            .count()
    };
    assert_eq!(count_messages(&session_a), 4, "A has all 4 messages");
    assert_eq!(count_messages(&session_b), 4, "B has all 4 messages");
}
