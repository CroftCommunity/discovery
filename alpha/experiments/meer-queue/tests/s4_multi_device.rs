//! **S4 — multi-device and deliver-once.**
//!
//! Claim under test, from `meer-as-custodian-queue.md` §"Cursors and delivery":
//!
//! > **Deliver-once is correct, not a compromise.** §6.6.5 guarantees that if any one of a
//! > persona's enrolled devices receives a message, every enrolled device eventually sees it,
//! > so the device-Group is the fan-out and the meer must not duplicate it. Prune on ack.
//!
//! The doc's own dial: *"deliver-once when a device group is present, race across enrolled
//! devices when it is not — a detectable condition, not a preference."*
//!
//! **This scenario runs the without-device-group arm, and expects the laptop to starve.** That
//! falsification is the deliverable: it makes concrete a dependency the doc currently asserts.
//!
//! Fidelity:
//! - **without a device group: Rung A (real-lib)** — real group, real transport, real prune.
//! - **with a device group: NOT TESTED.** §6.6.5 fan-out is not built, and standing something in
//!   for it would be standing in for the exact mechanism the claim is about — the methodology's
//!   canonical forbidden move. Reasoned in prose in `TEST-LOG.md`; a Rung-A follow-up is filed
//!   in `ROADMAP_TODO.md` and the question stays open in the residue.

use std::sync::Arc;

use meer_queue::ciss_harness::CissHarness;
use meer_queue::mls;
use meer_queue::transport::{MeerClient, MeerServer};
use mls_replant::{join, stamp, Persona};

#[tokio::test]
async fn without_a_device_group_deliver_once_starves_the_second_device() {
    meer_queue::init_tracing();

    // One persona — Bob — with two devices. In MLS terms both hold the same group state here;
    // what differs is that each device is a separate endpoint with its own queue.
    let alice = Persona::new("alice");
    let bob = Persona::new("bob");
    let mut alice_group = stamp(&alice, &[&bob]);
    let mut bob_phone_state = join(
        &bob,
        alice_group.welcome.clone().expect("welcome"),
        alice_group.ratchet_tree.clone(),
    );

    let ciss = Arc::new(CissHarness::spawn().await);
    let server = MeerServer::spawn(Arc::clone(&ciss)).await.expect("meer");
    let alice_client = MeerClient::connect(server.relay_url()).await.expect("alice");

    let phone = MeerClient::connect(server.relay_url()).await.expect("phone");
    let laptop = MeerClient::connect(server.relay_url()).await.expect("laptop");

    // Alice addresses Bob-the-persona. With no device group, the sender has one thing it can
    // do: name the device(s) it knows. Here it names the phone — the device it has seen.
    let plaintext = b"does the laptop ever see this?";
    let sealed = mls::seal(&mut alice_group.group, &alice, plaintext).expect("seal");
    let digest = alice_client
        .deposit(server.addr(), &sealed, &[phone.recipient_id()])
        .await
        .expect("deposit");

    // The phone drains and acks — the ordinary, correct path for that device.
    let phone_got = phone.drain(server.addr(), &[]).await.expect("phone drain");
    assert_eq!(phone_got.len(), 1, "the phone receives the message");
    assert_eq!(
        mls::open(&mut bob_phone_state, &bob, &phone_got[0]).expect("phone decrypts"),
        plaintext
    );
    phone.ack(server.addr(), &[digest]).await.expect("phone ack");

    // The laptop drains its own queue. Under drain-scoped-by-identity, it has none.
    let laptop_got = laptop.drain(server.addr(), &[]).await.expect("laptop drain");

    assert!(
        laptop_got.is_empty(),
        "EXPECTED FALSIFICATION: with no device group, the laptop starves — it received {} \
         message(s), which would mean the meer duplicated the fan-out",
        laptop_got.len()
    );

    // And the meer has nothing left to give: prune-on-ack removed the only reference.
    let phone_again = phone.drain(server.addr(), &[]).await.expect("phone re-drain");
    assert!(phone_again.is_empty(), "acked and pruned");

    println!(
        "S4 FALSIFIED-AS-EXPECTED (real-lib, without-device-group arm): the phone received and \
         acked; the laptop drained its own queue and got 0 messages. Naive deliver-once starves \
         a second enrolled device. The compensating mechanism (§6.6.5 device-group fan-out) is \
         NOT BUILT and is NOT TESTED here. [{}]",
        mls::resolved_versions()
    );

    laptop.close().await;
    phone.close().await;
    alice_client.close().await;
    server.shutdown().await;
    ciss.shutdown().await;
}

#[tokio::test]
async fn naming_both_devices_costs_one_deposit_and_two_queue_entries() {
    // The alternative the doc calls "race across enrolled devices": the sender names every
    // device it knows. Measured here so the cost of the fallback is a number, not a guess —
    // it is what the dial actually costs when no device group is present.
    meer_queue::init_tracing();

    let alice = Persona::new("alice");
    let bob = Persona::new("bob");
    let mut alice_group = stamp(&alice, &[&bob]);

    let ciss = Arc::new(CissHarness::spawn().await);
    let server = MeerServer::spawn(Arc::clone(&ciss)).await.expect("meer");
    let alice_client = MeerClient::connect(server.relay_url()).await.expect("alice");
    let phone = MeerClient::connect(server.relay_url()).await.expect("phone");
    let laptop = MeerClient::connect(server.relay_url()).await.expect("laptop");

    let sealed = mls::seal(&mut alice_group.group, &alice, b"named to both devices").expect("seal");

    let before = ciss.put_count();
    alice_client
        .deposit(
            server.addr(),
            &sealed,
            &[phone.recipient_id(), laptop.recipient_id()],
        )
        .await
        .expect("deposit to both");
    let deposits = ciss.put_count() - before;

    let phone_got = phone.drain(server.addr(), &[]).await.expect("phone");
    let laptop_got = laptop.drain(server.addr(), &[]).await.expect("laptop");
    assert_eq!(phone_got.len(), 1, "phone gets it");
    assert_eq!(laptop_got.len(), 1, "laptop gets it too");
    assert_eq!(phone_got[0], laptop_got[0], "the same bytes");

    println!(
        "S4 MEASURED (real-lib): racing across 2 enrolled devices costs {} deposit(s), {} stored \
         object(s), and 2 queue entries. The race is cheap at the meer — the cost is that each \
         device must ack independently, so pruning is per-device rather than per-message.",
        deposits,
        ciss.blob_files().len()
    );

    laptop.close().await;
    phone.close().await;
    alice_client.close().await;
    server.shutdown().await;
    ciss.shutdown().await;
}
