//! **S6 — revocation and re-point.** Bob points at a second meer and stops using the first.
//!
//! Claim under test — the design's strongest story, from `meer-as-custodian-queue.md`:
//!
//! > Per-DID queues do not satisfy that guard, they **retire** it: there is nothing to port
//! > because it never left home. Portability is a promise that a helper will give your data
//! > back; ownership is not having handed it over.
//!
//! Spike-spec learning goal: *the "it never left home" claim is the design's strongest story;
//! this is where it either holds or reveals a hidden dependency on the incumbent.*
//!
//! # Read the verdict carefully — this scenario is bounded by a stand-in
//!
//! SPEC-DELTA[meer-spike-namespace] means mail lives in **the meer's** CISS namespace, not the
//! recipient's. So this test can show that **re-pointing loses no mail and migrates nothing** —
//! the mechanism — but it **cannot** show the thing that makes the claim interesting, which is
//! that the mail was in *Bob's* namespace all along and the meer only ever held a revocable
//! grant to write there.
//!
//! Stated plainly: **S6 passes here for a weaker reason than the design claims.** Under the
//! stand-in, "nothing to migrate" is true because each meer's mail is independent, not because
//! Bob owned it. The strong form needs custodian mode (meer lane Phase 1) and is untestable
//! until then. Marking this CONFIRMED without that sentence would overstate it.

use std::sync::Arc;

use meer_queue::ciss_harness::CissHarness;
use meer_queue::mls;
use meer_queue::transport::{MeerClient, MeerServer};
use mls_replant::{join, stamp, Persona};

#[tokio::test]
async fn re_pointing_to_a_second_meer_loses_no_mail_and_migrates_nothing() {
    meer_queue::init_tracing();

    let alice = Persona::new("alice");
    let bob = Persona::new("bob");
    let mut alice_group = stamp(&alice, &[&bob]);
    let mut bob_group = join(
        &bob,
        alice_group.welcome.clone().expect("welcome"),
        alice_group.ratchet_tree.clone(),
    );

    // Two independent meers, each over its own CISS instance — two different providers, as a
    // real re-point would be.
    let ciss_a = Arc::new(CissHarness::spawn().await);
    let meer_a = MeerServer::spawn(Arc::clone(&ciss_a)).await.expect("meer A");
    let ciss_b = Arc::new(CissHarness::spawn().await);
    let meer_b = MeerServer::spawn(Arc::clone(&ciss_b)).await.expect("meer B");

    // Bob keeps one identity across the move — the same secret key, so the same queue address.
    let bob_secret = iroh::SecretKey::generate();
    let bob_on_a = MeerClient::connect_with_key(meer_a.relay_url(), Some(bob_secret.clone()))
        .await
        .expect("bob on A");
    let alice_on_a = MeerClient::connect(meer_a.relay_url()).await.expect("alice on A");

    // --- Era 1: Alice deposits with meer A. ---
    let msg1 = mls::seal(&mut alice_group.group, &alice, b"era one, via meer A").expect("seal 1");
    let d1 = alice_on_a
        .deposit(meer_a.addr(), &msg1, &[bob_on_a.recipient_id()])
        .await
        .expect("deposit A");

    // --- Bob re-points: he enrols with B and stops using A. Nothing is migrated. ---
    let bob_on_b = MeerClient::connect_with_key(meer_b.relay_url(), Some(bob_secret))
        .await
        .expect("bob on B");
    assert_eq!(
        bob_on_b.recipient_id(),
        bob_on_a.recipient_id(),
        "Bob's queue address is his identity, so it is the same at both meers"
    );
    let alice_on_b = MeerClient::connect(meer_b.relay_url()).await.expect("alice on B");

    // --- Era 2: Alice deposits with meer B. ---
    let msg2 = mls::seal(&mut alice_group.group, &alice, b"era two, via meer B").expect("seal 2");
    let d2 = alice_on_b
        .deposit(meer_b.addr(), &msg2, &[bob_on_b.recipient_id()])
        .await
        .expect("deposit B");

    // --- Nothing was lost across the move. Era-1 mail is still at A; era-2 mail is at B. ---
    let from_a = bob_on_a.drain(meer_a.addr(), &[]).await.expect("drain A");
    let from_b = bob_on_b.drain(meer_b.addr(), &[]).await.expect("drain B");
    assert_eq!(from_a, vec![msg1.clone()], "era-1 mail survived the re-point");
    assert_eq!(from_b, vec![msg2.clone()], "era-2 mail arrived at the new meer");

    // Both decrypt against the same group — the move is invisible to the seal.
    assert_eq!(
        mls::open(&mut bob_group, &bob, &from_a[0]).expect("open 1"),
        b"era one, via meer A"
    );
    assert_eq!(
        mls::open(&mut bob_group, &bob, &from_b[0]).expect("open 2"),
        b"era two, via meer B"
    );

    // Nothing was migrated: B never held era-1 mail, and A never learned about era 2.
    assert_eq!(
        ciss_b.blob_files().len(),
        1,
        "meer B stores only what was deposited with it — no migration"
    );
    assert_eq!(
        ciss_a.blob_files().len(),
        1,
        "meer A keeps only its own era — it was not asked to hand anything over"
    );

    bob_on_a.ack(meer_a.addr(), &[d1]).await.expect("ack A");
    bob_on_b.ack(meer_b.addr(), &[d2]).await.expect("ack B");

    println!(
        "S6 CONFIRMED-WITH-STAND-IN (real-lib): re-pointing lost no mail and migrated nothing — \
         each meer served only its own era, and Bob's queue address survived the move because it \
         is his identity. [{}]",
        mls::resolved_versions()
    );
    println!(
        "S6 LIMIT (what this does NOT show): under SPEC-DELTA[meer-spike-namespace] the mail sits \
         in each MEER's CISS namespace, not Bob's. So \"nothing to migrate\" holds here because \
         the two meers are independent, NOT because Bob owned the bytes. The design's actual \
         claim — that mail never left home and the meer held only a revocable grant to write into \
         Bob's own namespace — requires custodian mode and is UNTESTED. A stronger S6 belongs in \
         meer lane Phase 1."
    );

    bob_on_b.close().await;
    bob_on_a.close().await;
    alice_on_b.close().await;
    alice_on_a.close().await;
    meer_b.shutdown().await;
    meer_a.shutdown().await;
    ciss_b.shutdown().await;
    ciss_a.shutdown().await;
}
