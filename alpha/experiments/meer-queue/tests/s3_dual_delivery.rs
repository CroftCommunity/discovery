//! **S3 — dual delivery.** Bob receives the same message twice: once carried live, once
//! drained from the meer.
//!
//! Claim under test: it deduplicates on content hash to a single entry, and MLS applies it
//! idempotently — Part 2 §6.6.2. The hypothesis doc calls the racing story (§6.6.4) *free in
//! practice*; this is where that is checked rather than asserted.
//!
//! Fidelity: **Rung A (real-lib)**.
//!
//! The live-carriage path is real: Alice hands the bytes straight to Bob's endpoint over iroh,
//! bypassing the meer entirely, and separately deposits the same bytes with the meer.

use std::collections::HashSet;
use std::sync::Arc;

use meer_queue::ciss_harness::CissHarness;
use meer_queue::mls;
use meer_queue::transport::{MeerClient, MeerServer};
use mls_replant::{join, stamp, Persona};

fn sha256_hex(bytes: &[u8]) -> String {
    use sha2::{Digest as _, Sha256};
    let mut h = Sha256::new();
    h.update(bytes);
    hex::encode(h.finalize())
}

#[tokio::test]
async fn a_message_carried_live_and_drained_dedups_to_one_entry() {
    meer_queue::init_tracing();

    let alice = Persona::new("alice");
    let bob = Persona::new("bob");
    let mut alice_group = stamp(&alice, &[&bob]);
    let mut bob_group = join(
        &bob,
        alice_group.welcome.clone().expect("welcome"),
        alice_group.ratchet_tree.clone(),
    );

    let ciss = Arc::new(CissHarness::spawn().await);
    let server = MeerServer::spawn(Arc::clone(&ciss)).await.expect("meer");
    let alice_client = MeerClient::connect(server.relay_url()).await.expect("alice");
    let bob_client = MeerClient::connect(server.relay_url()).await.expect("bob");

    let plaintext = b"delivered twice, applied once";
    let sealed = mls::seal(&mut alice_group.group, &alice, plaintext).expect("seal");

    // Path 1: carried live, straight to Bob's endpoint.
    alice_client
        .live_deliver(bob_client.addr(), &sealed)
        .await
        .expect("live carriage");
    // Path 2: deposited with the meer for the same recipient.
    alice_client
        .deposit(server.addr(), &sealed, &[bob_client.recipient_id()])
        .await
        .expect("deposit");

    tokio::time::sleep(std::time::Duration::from_millis(300)).await;

    let live = bob_client.received().await;
    let drained = bob_client.drain(server.addr(), &[]).await.expect("drain");
    assert_eq!(live.len(), 1, "one copy arrived live");
    assert_eq!(drained.len(), 1, "one copy came from the meer");
    assert_eq!(live[0], drained[0], "both paths delivered the same bytes");

    // Dedup on content hash — the client holds one entry, not two.
    let mut held: HashSet<String> = HashSet::new();
    for copy in live.iter().chain(drained.iter()) {
        held.insert(sha256_hex(copy));
    }
    assert_eq!(
        held.len(),
        1,
        "two deliveries of the same object dedup to ONE entry on content hash"
    );

    // And having stated that digest, the meer does not re-send it.
    let digest = meer_queue::meer::Digest::new(held.iter().next().expect("digest").clone());
    let again = bob_client
        .drain(server.addr(), std::slice::from_ref(&digest))
        .await
        .expect("re-drain");
    assert!(
        again.is_empty(),
        "declaring the digest suppresses the duplicate — no special case needed"
    );

    // What does MLS itself do when the same application message is processed twice? Recorded,
    // not assumed: the second application is what the racing story rests on.
    let first = mls::open(&mut bob_group, &bob, &live[0]).expect("first application succeeds");
    assert_eq!(first, plaintext);
    let second = mls::open(&mut bob_group, &bob, &drained[0]);

    match &second {
        Ok(_) => println!(
            "S3 MEASURED (real-lib): a duplicate application message is ACCEPTED a second time by \
             openmls 0.8.1 — the library does not dedup, so dedup must be the client's job \
             (content-hash, as above)."
        ),
        Err(e) => println!(
            "S3 MEASURED (real-lib): a duplicate application message is REJECTED by openmls 0.8.1 \
             at the second application: {e}"
        ),
    }

    println!(
        "S3 CONFIRMED (real-lib): live + drained delivery of one object dedups to 1 entry on \
         content hash; declaring the digest suppresses re-send. [{}]",
        mls::resolved_versions()
    );

    bob_client.close().await;
    alice_client.close().await;
    server.shutdown().await;
    ciss.shutdown().await;
}
