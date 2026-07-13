//! P18 — two-node convergence over real iroh-gossip (Milestone C gate).
//!
//! The same property as P7 (order-insensitive convergence of two principals'
//! interleaved messages), but carried over the **real iroh-gossip transport**
//! instead of the shared-directory stand-in. Two endpoints run in one process
//! here (the same code path the cross-host run on the secroute boxes uses — see
//! `RUN.md`); the gate is the byte-identical fingerprint on both stores after a
//! cross-send.
//!
//! Gated behind `iroh-it` so default `cargo test` never binds a socket. Run with:
//!   cargo test -p croft-chat --features iroh-it --test iroh_convergence -- --nocapture

#![cfg(feature = "iroh-it")]

use std::time::Duration;

use croft_chat::fingerprint::{fingerprint, fingerprint_lines};
use croft_chat::iroh_bus::{IrohGossipBus, RelayChoice};
use croft_chat::sync::Replicator;
use croft_chat::transport::Topic;
use social_graph_core::{GroupId, Identity, PrincipalId, Role, Session, TimelineWindow};

/// A node's replication trio.
type Node<'a> = (&'a Session, &'a mut IrohGossipBus, &'a mut Replicator);

/// Publish both sides' group logs and pump, with a delay for gossip delivery,
/// until both fingerprints match or the bound is hit.
async fn converge_over_iroh(topic: &Topic, group: &GroupId, a: Node<'_>, b: Node<'_>) -> bool {
    let (session_a, bus_a, repl_a) = a;
    let (session_b, bus_b, repl_b) = b;
    for _ in 0..120 {
        repl_a.publish_group(session_a, bus_a, topic, group).ok();
        repl_b.publish_group(session_b, bus_b, topic, group).ok();
        repl_a.pump(session_a, bus_a);
        repl_b.pump(session_b, bus_b);
        if fingerprint(session_a, group) == fingerprint(session_b, group)
            && !fingerprint(session_a, group).is_empty()
        {
            return true;
        }
        tokio::time::sleep(Duration::from_millis(250)).await;
    }
    false
}

#[tokio::test]
async fn two_nodes_converge_over_iroh_gossip() {
    croft_chat::init_tracing();
    let topic = Topic("drystone/iroh-demo".to_string());

    let id_a = Identity::from_seed([0xA1; 32]);
    let id_b = Identity::from_seed([0xB2; 32]);

    let dir_a = tempfile::tempdir().expect("dir a");
    let dir_b = tempfile::tempdir().expect("dir b");
    let session_a = Session::open(&dir_a.path().join("a.redb"), &id_a).expect("open A");
    let session_b = Session::open(&dir_b.path().join("b.redb"), &id_b).expect("open B");
    session_a.trust_peer(id_b.device_id(), id_b.principal_id());
    session_b.trust_peer(id_a.device_id(), id_a.principal_id());

    // A binds first; B bootstraps from A's address (the cross-host recipe writes
    // these to JSON and exchanges them out-of-band).
    // SPEC-DELTA[hermetic-gossip | test-hermeticization]: LocalDirect exercises
    // loopback gossip only — NOT the relay/holepunch path a real deployment uses
    // (relay_mode = "n0"). That path is unreproducible where Internet UDP is
    // blocked; it is covered by X1 (needs the boxes). Register: alpha/SPEC-DIVERGENCE-REGISTER.md
    // Direct-only over loopback: hermetic (no relay / Internet dependency), so
    // the proof runs anywhere — a sandbox, CI, or a dev box — not only where the
    // n0 relays are reachable.
    let mut bus_a = IrohGossipBus::connect(&topic, vec![], RelayChoice::LocalDirect)
        .await
        .expect("bus a");
    let a_addr = bus_a.endpoint_addr();
    let mut bus_b = IrohGossipBus::connect(&topic, vec![a_addr], RelayChoice::LocalDirect)
        .await
        .expect("bus b");
    // Let the swarm form.
    tokio::time::sleep(Duration::from_secs(2)).await;

    let mut repl_a = Replicator::new();
    let mut repl_b = Replicator::new();

    // A creates the group and enrolls B.
    let group = session_a.create_group().await.expect("create_group");
    let b_principal = PrincipalId::new(id_b.principal_id().0);
    session_a
        .add_member(&group, b_principal, Role::Member)
        .await
        .expect("add_member");

    // Phase 1: B learns the group + its membership over gossip.
    assert!(
        converge_over_iroh(
            &topic,
            &group,
            (&session_a, &mut bus_a, &mut repl_a),
            (&session_b, &mut bus_b, &mut repl_b),
        )
        .await,
        "phase 1 (membership) must converge over iroh"
    );
    assert!(
        session_b
            .get_group_summary(&group)
            .map(|s| s.members.iter().any(|m| m.principal == b_principal))
            .unwrap_or(false),
        "B sees itself as a member after gossip"
    );

    // Both sides post, interleaved.
    session_a.send_message(&group, "a-1", None).await.expect("a-1");
    session_b.send_message(&group, "b-1", None).await.expect("b-1");
    session_a.send_message(&group, "a-2", None).await.expect("a-2");
    session_b.send_message(&group, "b-2", None).await.expect("b-2");

    // Phase 2: the four messages converge.
    let converged = converge_over_iroh(
        &topic,
        &group,
        (&session_a, &mut bus_a, &mut repl_a),
        (&session_b, &mut bus_b, &mut repl_b),
    )
    .await;

    let fa = fingerprint(&session_a, &group);
    let fb = fingerprint(&session_b, &group);
    if !converged || fa != fb {
        for line in fingerprint_lines(&session_a, &group) {
            if !fingerprint_lines(&session_b, &group).contains(&line) {
                eprintln!("only on A: {line}");
            }
        }
        for line in fingerprint_lines(&session_b, &group) {
            if !fingerprint_lines(&session_a, &group).contains(&line) {
                eprintln!("only on B: {line}");
            }
        }
        panic!("nodes did not converge over iroh");
    }

    let count = |s: &Session| {
        s.get_timeline(&group, TimelineWindow::LastN(usize::MAX))
            .map(|t| {
                t.entries
                    .iter()
                    .filter(|e| s.get_message(&e.hash).is_some())
                    .count()
            })
            .unwrap_or(0)
    };
    assert_eq!(count(&session_a), 4, "A has all four messages");
    assert_eq!(count(&session_b), 4, "B has all four messages");
}
