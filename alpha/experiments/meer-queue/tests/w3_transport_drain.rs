//! **Phase 4 wiring test** — deposit and drain over a real iroh connection, homed on a real
//! loopback relay.
//!
//! The positive half proves the path works. The **negative half is the point**: a third
//! endpoint draining the same meer gets nothing, because the queue a drain serves is chosen
//! from the caller's authenticated `EndpointId` and not from anything the caller says.
//!
//! Note what is *absent* from the wire protocol: a drain request carries **no recipient
//! field**. There is nothing for a caller to claim. That is the difference between a scope
//! that is enforced and one that is merely checked — Mallory cannot ask for Bob's mail
//! incorrectly, because there is no way to ask for it at all.
//!
//! SPEC-DELTA[meer-spike-drain-auth | stand-in]: the spec target is CISS **account identity**.
//! `EndpointId` comes free off the authenticated QUIC connection and carries the same shape
//! for this test; multi-device-per-account auth is not exercised.
//! — Register: `alpha/experiments/SPEC-DIVERGENCE-REGISTER.md`

use std::sync::Arc;

use meer_queue::ciss_harness::CissHarness;
use meer_queue::transport::{MeerClient, MeerServer};

const SEALED: &[u8] = b"sealed bytes that must cross the wire unchanged";

#[tokio::test]
async fn a_deposit_crosses_the_wire_and_only_its_recipient_can_drain_it() {
    let ciss = Arc::new(CissHarness::spawn().await);
    let server = MeerServer::spawn(Arc::clone(&ciss))
        .await
        .expect("meer server");

    let alice = MeerClient::connect(server.relay_url()).await.expect("alice");
    let bob = MeerClient::connect(server.relay_url()).await.expect("bob");
    let mallory = MeerClient::connect(server.relay_url()).await.expect("mallory");

    // Alice deposits for Bob, naming him by his endpoint id.
    let digest = alice
        .deposit(server.addr(), SEALED, &[bob.recipient_id()])
        .await
        .expect("deposit");

    // Bob drains: he gets the bytes, unchanged.
    let drained = bob.drain(server.addr(), &[]).await.expect("bob drains");
    assert_eq!(drained.len(), 1, "Bob has one message waiting");
    assert_eq!(
        drained[0], SEALED,
        "bytes must cross the wire unchanged in both directions"
    );

    // Mallory drains the same meer and gets nothing. She never had a way to name Bob.
    let stolen = mallory
        .drain(server.addr(), &[])
        .await
        .expect("mallory's drain succeeds but is empty");
    assert!(
        stolen.is_empty(),
        "a drain must serve only the caller's own queue, got {} messages",
        stolen.len()
    );

    // Bob acks; the entry is pruned and a second drain is empty.
    bob.ack(server.addr(), &[digest]).await.expect("ack");
    let after = bob.drain(server.addr(), &[]).await.expect("bob re-drains");
    assert!(after.is_empty(), "an acked message is not served again");

    server.shutdown().await;
    ciss.shutdown().await;
}

#[tokio::test]
async fn the_have_set_crosses_the_wire_and_suppresses_what_the_caller_holds() {
    let ciss = Arc::new(CissHarness::spawn().await);
    let server = MeerServer::spawn(Arc::clone(&ciss))
        .await
        .expect("meer server");
    let alice = MeerClient::connect(server.relay_url()).await.expect("alice");
    let bob = MeerClient::connect(server.relay_url()).await.expect("bob");

    let first = alice
        .deposit(server.addr(), b"message one", &[bob.recipient_id()])
        .await
        .expect("deposit one");
    alice
        .deposit(server.addr(), b"message two", &[bob.recipient_id()])
        .await
        .expect("deposit two");

    // Stating what he already holds suppresses exactly that message, over the wire.
    let wanted = bob
        .drain(server.addr(), std::slice::from_ref(&first))
        .await
        .expect("drain with a have-set");
    assert_eq!(wanted, vec![b"message two".to_vec()]);

    server.shutdown().await;
    ciss.shutdown().await;
}
