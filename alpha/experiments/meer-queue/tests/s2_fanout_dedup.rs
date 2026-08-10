//! **S2 — fan-out and dedup.** One message, five recipients.
//!
//! Claim under test: the blob is stored **once** in CISS (content-addressed) and referenced
//! five times — `meer-as-custodian-queue.md` §"What the meer does": *"`PUT` the blob once to
//! CISS (content-addressed, so a message to fifty recipients is stored once)"*.
//!
//! Fidelity: **Rung A (real-lib)** — real OpenMLS seal, real CISS storage boundary.
//!
//! Two things are measured separately, because Phase 3 established they are different claims
//! that the obvious observable cannot tell apart:
//!
//! - **deposits** — did the *meer* store once? Counted at the boundary it crosses.
//! - **stored objects** — did *CISS* dedup? Counted on disk.
//!
//! A meer that deposited per-recipient would leave one file either way, because the store is
//! content-addressed. Storage dedups; transit does not.

use std::sync::Arc;

use meer_queue::ciss_harness::CissHarness;
use meer_queue::meer::RecipientId;
use meer_queue::mls;
use meer_queue::transport::{MeerClient, MeerServer};
use mls_replant::{stamp, Persona};

#[tokio::test]
async fn one_message_to_five_recipients_is_deposited_once_and_stored_once() {
    meer_queue::init_tracing();

    let alice = Persona::new("alice");
    let members: Vec<Persona> = (0..5).map(|i| Persona::new(&format!("m{i}"))).collect();
    let mut group = stamp(&alice, &members.iter().collect::<Vec<_>>());
    let sealed = mls::seal(&mut group.group, &alice, b"one message, five recipients").expect("seal");

    let ciss = Arc::new(CissHarness::spawn().await);
    let server = MeerServer::spawn(Arc::clone(&ciss)).await.expect("meer");
    let alice_client = MeerClient::connect(server.relay_url()).await.expect("alice");

    let recipients: Vec<RecipientId> = (0..5)
        .map(|i| RecipientId::new(format!("recipient-{i}")))
        .collect();

    let before = ciss.put_count();
    alice_client
        .deposit(server.addr(), &sealed, &recipients)
        .await
        .expect("deposit");
    let deposits = ciss.put_count() - before;

    assert_eq!(
        deposits, 1,
        "five recipients must cost ONE deposit — this is the meer's claim, and transit is \
         metered even where storage dedups"
    );
    assert_eq!(
        ciss.blob_files().len(),
        1,
        "and CISS must store it once — a separate claim, about the store"
    );

    // The naive alternative, measured rather than asserted: deposit per recipient and see what
    // it actually costs. Storage is unchanged (content-addressed); transit is 5x.
    let naive_before = ciss.put_count();
    for _ in &recipients {
        alice_client
            .deposit(server.addr(), &sealed, std::slice::from_ref(&recipients[0]))
            .await
            .expect("naive deposit");
    }
    let naive_deposits = ciss.put_count() - naive_before;
    let files_after_naive = ciss.blob_files().len();

    println!(
        "S2 MEASURED (real-lib): fan-out to 5 = {} deposit(s), {} stored object(s), {} sealed bytes. \
         Naive per-recipient = {} deposit(s), still {} stored object(s). \
         Dedup saves TRANSIT ({}x), not at-rest storage. [{}]",
        deposits,
        1,
        sealed.len(),
        naive_deposits,
        files_after_naive,
        naive_deposits,
        mls::resolved_versions()
    );

    alice_client.close().await;
    server.shutdown().await;
    ciss.shutdown().await;
}

#[tokio::test]
async fn dedup_is_scoped_to_a_namespace_which_bounds_the_fan_out_claim() {
    // The hypothesis doc's dedup claim is stated unconditionally. It is not unconditional: CISS
    // lays objects out as `blocks/{did}/{cid}`, so identical bytes in two namespaces are two
    // stored objects. Under the spike's meer-owned single namespace
    // (SPEC-DELTA[meer-spike-namespace]) the claim holds. Under the design's *stated default* —
    // per-DID queues in each recipient's own namespace — it would not.
    //
    // This is measured here rather than reasoned about, because it decides whether "stored once"
    // survives the move to custodian mode.
    let ciss = Arc::new(CissHarness::spawn().await);
    let alice = ciss.identity("alice");
    let bob = ciss.identity("bob");
    let bytes = b"identical bytes, two owners";

    let a = ciss.put_object(&alice, "msg", bytes).await;
    let b = ciss.put_object(&bob, "msg", bytes).await;
    assert_eq!(a.status, 200);
    assert_eq!(b.status, 200);
    assert_eq!(
        a.cid(),
        b.cid(),
        "the content address is the same — it is a function of the bytes"
    );

    let files = ciss.blob_files();
    println!(
        "S2 MEASURED (real-lib): identical bytes stored under 2 namespaces = {} object(s) on disk \
         ({:?}). Dedup is per-namespace.",
        files.len(),
        files
    );
    assert_eq!(
        files.len(),
        2,
        "CISS namespaces each hold their own copy — dedup does not cross a namespace boundary"
    );

    ciss.shutdown().await;
}
