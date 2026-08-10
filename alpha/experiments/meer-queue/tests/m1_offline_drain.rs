//! **M1 — an offline member drains and decrypts.** The first must-pass claim.
//!
//! Claim under test: a member offline during a message's live window recovers it from the
//! meer and decrypts it, with the meer never holding a key
//! (`meer-as-custodian-queue.md` §"What the meer does"; Part 2 §6.6.2).
//!
//! Fidelity: **Rung A (real-lib)** — real OpenMLS group and seal, real CISS storage boundary,
//! real iroh transport over a real loopback relay. Nothing about the seal is stood in.
//!
//! "Offline" is a **genuine endpoint teardown**, not a flag the meer reads, and the test proves
//! that *discriminatingly*: Bob is dialed while up (must succeed) and again after teardown
//! (must fail). An earlier version asserted only the second half — and a mutation that removed
//! the teardown entirely still passed, because nothing listened on Bob's endpoint in either
//! world, so the dial failed both times. The assertion was equally true of the correct state
//! and the broken one. Both halves are now required.
//!
//! Bob returns on the **same secret key**, so his `EndpointId` — and therefore his queue — is
//! unchanged across the absence.

use std::sync::Arc;
use std::time::Duration;

use meer_queue::ciss_harness::CissHarness;
use meer_queue::mls;
use meer_queue::transport::{MeerClient, MeerServer};
use mls_replant::{join, stamp, Persona};

/// Reachability of `peer` from `from`, bounded so a down peer fails fast.
async fn reachable(from: &MeerClient, peer: iroh::EndpointAddr) -> bool {
    matches!(
        tokio::time::timeout(Duration::from_secs(3), from.probe(peer)).await,
        Ok(Ok(()))
    )
}

#[tokio::test]
async fn an_offline_member_drains_and_decrypts_while_the_meer_holds_no_group_key() {
    meer_queue::init_tracing();

    // --- A real two-member MLS group. ---
    let alice = Persona::new("alice");
    let bob = Persona::new("bob");
    let mut alice_group = stamp(&alice, &[&bob]);
    let mut bob_group = join(
        &bob,
        alice_group.welcome.clone().expect("welcome"),
        alice_group.ratchet_tree.clone(),
    );

    // --- A meer over real CISS, reachable over a real relay. ---
    let ciss = Arc::new(CissHarness::spawn().await);
    let server = MeerServer::spawn(Arc::clone(&ciss)).await.expect("meer");
    let alice_client = MeerClient::connect(server.relay_url())
        .await
        .expect("alice endpoint");

    // Bob binds. His secret key is kept, so the endpoint he returns on is the one he left.
    let bob_secret = iroh::SecretKey::generate();
    let bob_client = MeerClient::connect_with_key(server.relay_url(), Some(bob_secret.clone()))
        .await
        .expect("bob endpoint");
    let bob_id = bob_client.recipient_id();
    let bob_addr = bob_client.addr();

    // Both halves are load-bearing: "up" must differ from "down", or the observation is empty.
    assert!(
        reachable(&alice_client, bob_addr.clone()).await,
        "precondition: Bob must be reachable BEFORE the teardown, otherwise the \
         after-teardown check proves nothing"
    );

    bob_client.close().await;

    assert!(
        !reachable(&alice_client, bob_addr.clone()).await,
        "Bob must be genuinely unreachable during the send window, not merely marked absent"
    );

    // --- Alice seals and publishes while Bob is down. ---
    let plaintext = b"the conversation stays alive while you sleep";
    let sealed = mls::seal(&mut alice_group.group, &alice, plaintext).expect("seal");
    let digest = alice_client
        .deposit(server.addr(), &sealed, std::slice::from_ref(&bob_id))
        .await
        .expect("deposit while bob is offline");

    // --- Bob comes back on the same key, drains, and decrypts. ---
    let bob_back = MeerClient::connect_with_key(server.relay_url(), Some(bob_secret))
        .await
        .expect("bob reconnects");
    assert_eq!(
        bob_back.recipient_id(),
        bob_id,
        "Bob must return to the same queue he left"
    );
    assert!(
        bob_back.received().await.is_empty(),
        "nothing was carried to Bob live — the meer is the only path this message took"
    );

    let drained = bob_back.drain(server.addr(), &[]).await.expect("drain");
    assert_eq!(drained.len(), 1, "exactly the message he missed");
    assert_eq!(
        drained[0], sealed,
        "the meer must return the sealed bytes unchanged"
    );

    let opened = mls::open(&mut bob_group, &bob, &drained[0]).expect("real process_message");
    assert_eq!(
        opened, plaintext,
        "Bob recovers the plaintext through the real library"
    );

    bob_back.ack(server.addr(), &[digest]).await.expect("ack");

    // --- The meer held no group key at any point. ---
    let keys = server.key_inventory().await;
    assert_eq!(
        keys.group_keys, 0,
        "the meer must never hold MLS/group key material"
    );
    // Stated rather than hidden: it does hold ONE credential — its own CISS namespace key.
    // Blind to content is not the same as credential-less, and a bare zero would overstate it.
    assert_eq!(
        keys.storage_credentials, 1,
        "the meer holds exactly its own storage credential"
    );

    println!(
        "M1 CONFIRMED (real-lib): offline member drained {} blob(s) and decrypted; \
         meer group keys held = {}, storage credentials = {}. [{}]",
        drained.len(),
        keys.group_keys,
        keys.storage_credentials,
        mls::resolved_versions()
    );

    bob_back.close().await;
    alice_client.close().await;
    server.shutdown().await;
    ciss.shutdown().await;
}
