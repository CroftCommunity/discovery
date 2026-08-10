//! **Phase 3 wiring test** — the meer's four core operations, end to end through real CISS.
//!
//! Drives: accept a publish → `PUT` the blob **once** → append an entry to each recipient's
//! queue → serve a drain by have/want diff → ack and prune. (The fifth operation, sweep and
//! watermark, is deliberately not built yet; S5 drives it in Phase 9.)
//!
//! The happy path alone is a single-point assertion on branching set logic, so the edges are
//! here too — an empty queue, an unknown digest in the have-set, and a repeated drain. Those
//! are the boundaries a one-line mutation to the diff would otherwise survive.

use meer_queue::ciss_harness::CissHarness;
use meer_queue::meer::{Meer, RecipientId};

/// Sealed bytes stand in for a real MLS message here **only** in the sense that this test is
/// about the queue, not the seal — the meer never inspects them either way. M1 and M2 drive
/// the same path with genuine OpenMLS output.
const MSG_A: &[u8] = b"sealed message A";
const MSG_B: &[u8] = b"sealed message B";

fn bob() -> RecipientId {
    RecipientId::new("bob")
}
fn carol() -> RecipientId {
    RecipientId::new("carol")
}

#[tokio::test]
async fn one_publish_to_many_recipients_stores_the_blob_once() {
    let ciss = CissHarness::spawn().await;
    let mut meer = Meer::new(&ciss);

    let digest = meer
        .publish(MSG_A, &[bob(), carol()])
        .await
        .expect("publish");

    // Two distinct claims, and only one of them is about the meer:
    //   - CISS dedups at rest, so ONE file lands on disk. True however many times we PUT.
    //   - the MEER deposits once, so only ONE PUT crosses the boundary. This is the claim.
    // A surviving mutant taught us the difference: PUTting per-recipient still leaves one
    // file, but it is N x the metered transit for one delivered message.
    assert_eq!(
        ciss.put_count(),
        1,
        "a message to two recipients must be DEPOSITED once, not once per recipient"
    );
    assert_eq!(
        ciss.blob_files().len(),
        1,
        "and CISS must store it once (content-addressed)"
    );
    assert_eq!(meer.queue_len(&bob()), 1, "bob gets an entry");
    assert_eq!(meer.queue_len(&carol()), 1, "carol gets an entry");

    // Both queues reference the same stored object.
    assert!(meer.wants(&bob(), &[]).contains(&digest));
    assert!(meer.wants(&carol(), &[]).contains(&digest));

    ciss.shutdown().await;
}

#[tokio::test]
async fn a_drain_returns_only_what_the_recipient_lacks() {
    let ciss = CissHarness::spawn().await;
    let mut meer = Meer::new(&ciss);

    let a = meer.publish(MSG_A, &[bob()]).await.expect("publish a");
    let b = meer.publish(MSG_B, &[bob()]).await.expect("publish b");

    // Empty have-set: everything is wanted.
    let first = meer.drain(&bob(), &[]).await.expect("first drain");
    assert_eq!(first.len(), 2, "an empty have-set wants both messages");
    assert!(first.contains(&MSG_A.to_vec()) && first.contains(&MSG_B.to_vec()));

    // Ack one; it is pruned and no longer served.
    meer.ack(&bob(), std::slice::from_ref(&a));
    assert_eq!(meer.queue_len(&bob()), 1, "an acked entry is pruned");

    let second = meer.drain(&bob(), &[]).await.expect("second drain");
    assert_eq!(second, vec![MSG_B.to_vec()], "only the un-acked message remains");

    // And stating what you already hold suppresses it even before an ack.
    let third = meer.drain(&bob(), &[b]).await.expect("third drain");
    assert!(third.is_empty(), "a digest in the have-set is not re-sent");

    ciss.shutdown().await;
}

#[tokio::test]
async fn draining_an_empty_queue_is_empty_not_an_error() {
    let ciss = CissHarness::spawn().await;
    let meer = Meer::new(&ciss);

    let drained = meer
        .drain(&bob(), &[])
        .await
        .expect("draining an empty queue must succeed, not error");
    assert!(drained.is_empty());
    assert_eq!(meer.queue_len(&bob()), 0);

    ciss.shutdown().await;
}

#[tokio::test]
async fn an_unknown_digest_in_the_have_set_is_ignored() {
    let ciss = CissHarness::spawn().await;
    let mut meer = Meer::new(&ciss);
    let a = meer.publish(MSG_A, &[bob()]).await.expect("publish");

    // A digest the queue never held: must neither crash nor be echoed back.
    let bogus = meer_queue::meer::Digest::new("f".repeat(64));
    let drained = meer
        .drain(&bob(), std::slice::from_ref(&bogus))
        .await
        .expect("an unknown have-digest must be tolerated");
    assert_eq!(
        drained,
        vec![MSG_A.to_vec()],
        "an unknown have-digest must not suppress a real entry"
    );
    assert!(!meer.wants(&bob(), &[bogus]).is_empty());
    assert_eq!(meer.wants(&bob(), &[a]).len(), 0);

    ciss.shutdown().await;
}

#[tokio::test]
async fn draining_twice_without_a_publish_is_idempotent() {
    let ciss = CissHarness::spawn().await;
    let mut meer = Meer::new(&ciss);
    meer.publish(MSG_A, &[bob()]).await.expect("publish");

    let first = meer.drain(&bob(), &[]).await.expect("first");
    let second = meer.drain(&bob(), &[]).await.expect("second");
    assert_eq!(
        first, second,
        "a drain must not consume; only an ack prunes"
    );
    assert_eq!(meer.queue_len(&bob()), 1);

    ciss.shutdown().await;
}

#[tokio::test]
async fn entries_record_the_day_they_were_deposited() {
    // The watermark story (S5, Phase 9) needs to know when mail arrived. The clock is
    // CISS's own `SimClock` — deterministic, day-granular, no wall-clock reads.
    let ciss = CissHarness::spawn().await;
    let mut meer = Meer::new(&ciss);

    meer.publish(MSG_A, &[bob()]).await.expect("day 0");
    meer.advance_days(3);
    meer.publish(MSG_B, &[bob()]).await.expect("day 3");

    let days = meer.deposit_days(&bob());
    assert_eq!(
        days,
        vec![0, 3],
        "each entry records the day it was deposited"
    );

    ciss.shutdown().await;
}

#[tokio::test]
async fn the_same_object_queued_twice_for_one_recipient_is_one_entry() {
    // The queue is keyed by content address, so re-depositing the same sealed bytes for the
    // same recipient is a no-op rather than a duplicate. This is what makes S3's dual delivery
    // (carried live AND drained) free rather than a special case — but it is queue behaviour,
    // so it is pinned here where the queue is built, not left for S3 to discover.
    let ciss = CissHarness::spawn().await;
    let mut meer = Meer::new(&ciss);

    let first = meer.publish(MSG_A, &[bob()]).await.expect("first");
    let second = meer.publish(MSG_A, &[bob()]).await.expect("second");

    assert_eq!(first, second, "identical bytes get one content address");
    assert_eq!(meer.queue_len(&bob()), 1, "and one queue entry, not two");
    assert_eq!(ciss.blob_files().len(), 1, "and one stored object");

    ciss.shutdown().await;
}
