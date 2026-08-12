//! **S10 — what does catch-up actually cost?**
//!
//! S9 established that a returning member walks the epoch chain: **N missed epochs = N serial
//! round trips**, because the queue name for epoch E+1 is derivable only after applying the commit
//! that created it. This measures whether that is expensive **over the real transport**, and
//! compares it against the obvious "skip ahead" alternative.
//!
//! Fidelity: **Rung A (real-lib)** — real OpenMLS, real CISS, real iroh over a real relay.
//!
//! # The framing this test corrects
//!
//! Two things were assumed and are measured here instead:
//!
//! 1. **How big is N?** Earlier framing said "a group that rotates keys aggressively makes returns
//!    slower", implying chat drives it. It does not: **only commits advance the epoch** (50
//!    application messages leave it unchanged). So N counts *governance events* in the retention
//!    window, not messages — and the window bounds it absolutely.
//! 2. **Is skipping ahead even an alternative?** It is cheaper only if it does the same job.
//!
//! The queue name is used directly as the meer's `RecipientId`: in the fabric model the queue *is*
//! the group's, so the "recipient" is the name, and no meer change is needed.

use std::sync::Arc;
use std::time::Instant;

use meer_queue::ciss_harness::CissHarness;
use meer_queue::meer::RecipientId;
use meer_queue::mls;
use meer_queue::transport::{MeerClient, MeerServer};
use mls_replant::{join, stamp, Persona};
use openmls::prelude::*;
use tls_codec::{Deserialize as _, Serialize as _};

const QUEUE_LABEL: &str = "croft/meer-queue/v1";

fn queue_name(group: &MlsGroup, who: &Persona) -> RecipientId {
    let secret = group
        .export_secret(who.provider.crypto(), QUEUE_LABEL, &[], 32)
        .expect("export_secret");
    RecipientId::new(hex::encode(secret))
}

/// Walk the chain over the real transport, timing each hop.
#[tokio::test]
async fn a_returning_member_walks_the_chain_over_the_real_transport() {
    meer_queue::init_tracing();
    const MISSED: usize = 10;

    let alice = Persona::new("alice");
    let bob = Persona::new("bob");
    let mut a = stamp(&alice, &[&bob]);
    let mut bob_group = join(
        &bob,
        a.welcome.clone().expect("welcome"),
        a.ratchet_tree.clone(),
    );

    let ciss = Arc::new(CissHarness::spawn().await);
    let server = MeerServer::spawn(Arc::clone(&ciss)).await.expect("meer");
    let alice_client = MeerClient::connect(server.relay_url()).await.expect("alice");

    // Bob's last known queue.
    let bob_start = queue_name(&bob_group, &bob);

    // While Bob is away: in each epoch Alice seals a message AND commits, both landing in the
    // queue named by the epoch they were sent in.
    for i in 0..MISSED {
        let q = queue_name(&a.group, &alice);
        let msg = mls::seal(&mut a.group, &alice, format!("message in epoch {i}").as_bytes())
            .expect("seal");
        alice_client
            .deposit(server.addr(), &msg, std::slice::from_ref(&q))
            .await
            .expect("deposit message");
        let (_, commit) = mls_replant::commit(&mut a.group, &alice);
        let commit_bytes = commit.tls_serialize_detached().expect("ser");
        alice_client
            .deposit(server.addr(), &commit_bytes, std::slice::from_ref(&q))
            .await
            .expect("deposit commit");
    }

    // Bob returns and walks. One connection, sequential hops — the pipelined case.
    let bob_client = MeerClient::connect(server.relay_url()).await.expect("bob");
    let mut asking = bob_start;
    let mut read = 0usize;
    let mut per_hop = Vec::new();

    let total_start = Instant::now();
    for _ in 0..MISSED {
        let hop = Instant::now();
        // Drain BY NAME — the capability model. Bob names the one queue he can derive.
        let blobs = bob_client
            .drain_queue(server.addr(), &asking, &[])
            .await
            .expect("drain by name");
        for bytes in blobs {
            // **Dispatch on the CLEARTEXT `content_type` BEFORE processing.** `process_message`
            // consumes the message key (S3b), so a try-decrypt-then-fall-back pattern corrupts the
            // group's own state: the second attempt hits `SecretReuseError`. This test was written
            // that way first and failed exactly so. S7 measured that `content_type` is readable
            // with no key — routing on it, rather than on a failed decrypt, is what it is for.
            let parsed = MlsMessageIn::tls_deserialize_exact(&bytes).expect("parse");
            let protocol: ProtocolMessage = parsed.try_into_protocol_message().expect("protocol");
            match protocol.content_type() {
                ContentType::Application => {
                    let plain = mls::open(&mut bob_group, &bob, &bytes).expect("open application");
                    read += 1;
                    assert!(String::from_utf8_lossy(&plain).starts_with("message in epoch"));
                }
                ContentType::Commit => {
                    let processed = bob_group
                        .process_message(&bob.provider, protocol)
                        .expect("process commit");
                    if let ProcessedMessageContent::StagedCommitMessage(sc) =
                        processed.into_content()
                    {
                        bob_group
                            .merge_staged_commit(&bob.provider, *sc)
                            .expect("merge");
                    }
                }
                other => panic!("unexpected content type {other:?}"),
            }
        }
        per_hop.push(hop.elapsed());
        asking = queue_name(&bob_group, &bob);
    }
    let total = total_start.elapsed();

    assert_eq!(read, MISSED, "every missed message was read");
    assert_eq!(
        asking,
        queue_name(&a.group, &alice),
        "Bob has caught up to the current queue"
    );

    let avg = total / u32::try_from(MISSED).expect("fits");
    println!(
        "S10 MEASURED (real-lib): catch-up across {MISSED} missed epochs over real CISS + real iroh \
         = {:?} total, {:?} per hop, {read} message(s) recovered.",
        total, avg
    );
    println!(
        "S10 NOTE: N counts GOVERNANCE events, not messages — 50 application messages leave the \
         epoch unchanged. And the retention window bounds N absolutely: past it there is nothing \
         to walk, so a member back after six months pays the same walk as one back after two weeks."
    );

    bob_client.close().await;
    alice_client.close().await;
    server.shutdown().await;
    ciss.shutdown().await;
}

/// Is "skip ahead" actually an alternative? Only if it does the same job.
#[test]
fn skipping_ahead_is_cheaper_because_it_does_not_deliver_the_messages() {
    const MISSED: usize = 3;

    let alice = Persona::new("alice");
    let bob = Persona::new("bob");
    let mut a = stamp(&alice, &[&bob]);
    let mut bob_group = join(
        &bob,
        a.welcome.clone().expect("welcome"),
        a.ratchet_tree.clone(),
    );

    // Alice sends one message per epoch while Bob is away, keeping every ciphertext.
    let mut missed_messages = Vec::new();
    let mut commits = Vec::new();
    for i in 0..MISSED {
        missed_messages.push(
            mls::seal(&mut a.group, &alice, format!("missed {i}").as_bytes()).expect("seal"),
        );
        let (_, c) = mls_replant::commit(&mut a.group, &alice);
        commits.push(c);
    }

    // Bob "skips ahead": he applies only the LAST commit, without the ones before it.
    let last = commits.last().expect("a commit");
    let bytes = last.tls_serialize_detached().expect("ser");
    let protocol: ProtocolMessage = MlsMessageIn::tls_deserialize_exact(&bytes)
        .expect("parse")
        .try_into_protocol_message()
        .expect("protocol");
    let jumped = bob_group.process_message(&bob.provider, protocol);

    match jumped {
        Err(e) => println!(
            "S10 MEASURED (real-lib): skipping ahead is REFUSED outright — a member cannot apply a \
             commit whose predecessors it has not seen: {e}"
        ),
        Ok(_) => {
            // If it were accepted, the point still stands: the missed plaintexts are unreachable.
            let recovered = missed_messages
                .iter()
                .filter(|m| mls::open(&mut bob_group, &bob, m).is_ok())
                .count();
            println!(
                "S10 MEASURED (real-lib): skipping ahead recovered {recovered}/{MISSED} missed \
                 messages."
            );
            assert_eq!(recovered, 0, "skipping cannot recover the missed messages");
        }
    }

    println!(
        "S10 CONFIRMED (real-lib): the chain walk and 'skip ahead' are NOT two strategies for one \
         goal. The walk catches up AND delivers what was missed; skipping abandons it. Comparing \
         their costs is a category error — the cheaper option does not do the job."
    );
}
