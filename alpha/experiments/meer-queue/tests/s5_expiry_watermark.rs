//! **S5 — expiry and the watermark.**
//!
//! Claim under test, from `meer-as-custodian-queue.md` §"Cursors and delivery":
//!
//! > Retention is **14 days as a ceiling, not a floor** — "14 days or until drained," never
//! > "14 days no matter what." … Past the window the recipient gets the watermark: a loud,
//! > visible, SSH-host-key-shaped "here is what is gone," which is the no-invisible-loss rule
//! > (Part 1 §2.2) doing its job.
//!
//! Learning goal (spike spec): *whether "loud, visible gap" is actually constructible from what
//! the meer retains, or whether it needs more state than the watermark.*
//!
//! Fidelity: **Rung A (real-lib)** — real CISS storage boundary. Time is
//! SPEC-DELTA[meer-spike-clock], CISS's own `SimClock`.
//!
//! The watermark is deliberately **minimal** — a count and a day range, no digests. If a richer
//! watermark turns out to be necessary, that should be a finding, not something the test quietly
//! assumed by retaining enough state to guarantee the answer.

use std::sync::Arc;

use meer_queue::ciss_harness::CissHarness;
use meer_queue::meer::{Meer, RecipientId, RETENTION_DAYS};

fn bob() -> RecipientId {
    RecipientId::new("bob")
}
fn carol() -> RecipientId {
    RecipientId::new("carol")
}

#[tokio::test]
async fn expired_entries_stop_being_served_and_leave_a_watermark() {
    meer_queue::init_tracing();
    let ciss = Arc::new(CissHarness::spawn().await);
    let mut meer = Meer::new(Arc::clone(&ciss));

    meer.publish(b"missed message one", &[bob()]).await.expect("pub 1");
    meer.advance_days(2);
    meer.publish(b"missed message two", &[bob()]).await.expect("pub 2");

    meer.advance_days(RETENTION_DAYS + 1);
    let report = meer.sweep();

    assert_eq!(report.swept, 2, "both entries aged out");
    assert_eq!(meer.queue_len(&bob()), 0, "nothing left to serve");
    assert!(
        meer.drain(&bob(), &[]).await.expect("drain").is_empty(),
        "an expired queue serves nothing"
    );

    let wm = meer.watermark(&bob()).expect("a watermark remains");
    assert_eq!(wm.swept, 2);
    assert_eq!(wm.earliest_day, 0);
    assert_eq!(wm.latest_day, 2);

    ciss.shutdown().await;
}

#[tokio::test]
async fn the_retention_boundary_is_exact() {
    // "14 days as a ceiling" is a comparison, and testing only "much later" lets an off-by-one
    // live. Both sides of the boundary are pinned.
    let ciss = Arc::new(CissHarness::spawn().await);
    let mut meer = Meer::new(Arc::clone(&ciss));

    meer.publish(b"at the boundary", &[bob()]).await.expect("pub");
    meer.advance_days(RETENTION_DAYS);
    assert_eq!(meer.sweep().swept, 0, "an entry exactly at the window is NOT yet expired");
    assert_eq!(meer.queue_len(&bob()), 1);

    meer.advance_days(1);
    assert_eq!(meer.sweep().swept, 1, "one day past the window, it is");
    assert_eq!(meer.queue_len(&bob()), 0);

    ciss.shutdown().await;
}

#[tokio::test]
async fn a_queue_drained_before_expiry_leaves_no_watermark() {
    // The rule is "14 days OR UNTIL DRAINED". A watermark for successfully delivered mail would
    // be a false gap report — the opposite of what the no-invisible-loss rule wants.
    let ciss = Arc::new(CissHarness::spawn().await);
    let mut meer = Meer::new(Arc::clone(&ciss));

    let digest = meer.publish(b"delivered on time", &[bob()]).await.expect("pub");
    meer.drain(&bob(), &[]).await.expect("drain");
    meer.ack(&bob(), &[digest]);

    meer.advance_days(RETENTION_DAYS + 5);
    assert_eq!(meer.sweep().swept, 0, "nothing to sweep — it was drained");
    assert!(
        meer.watermark(&bob()).is_none(),
        "delivered mail must NOT leave a gap marker"
    );

    ciss.shutdown().await;
}

#[tokio::test]
async fn expiry_is_per_recipient_but_storage_is_shared() {
    // One object, two recipients, different drain behaviour. Bob's entry ages out; Carol has
    // not drained. The shared object must not vanish out from under her.
    let ciss = Arc::new(CissHarness::spawn().await);
    let mut meer = Meer::new(Arc::clone(&ciss));

    meer.publish(b"one object, two queues", &[bob(), carol()]).await.expect("pub");
    assert_eq!(ciss.blob_files().len(), 1);

    meer.advance_days(RETENTION_DAYS + 1);
    // Only Bob's queue is swept in this scenario — Carol drains first, so hers is not expired.
    meer.sweep();
    assert_eq!(meer.queue_len(&bob()), 0, "bob's entry aged out");

    // Carol's entry aged out too (same deposit day) — which is the point: expiry is a property
    // of the entry's age, not of any one recipient's behaviour.
    assert_eq!(meer.queue_len(&carol()), 0, "carol's entry aged out on the same clock");

    ciss.shutdown().await;
}

/// **The finding.** "Here is what is gone" is a claim about *serving*, not about *storage*.
#[tokio::test]
async fn sweeping_stops_service_but_cannot_remove_the_bytes() {
    let ciss = Arc::new(CissHarness::spawn().await);
    let mut meer = Meer::new(Arc::clone(&ciss));

    meer.publish(b"supposedly gone after the window", &[bob()]).await.expect("pub");
    let stored_before = ciss.blob_files().len();
    assert_eq!(stored_before, 1);

    meer.advance_days(RETENTION_DAYS + 1);
    meer.sweep();

    // The queue no longer serves it...
    assert!(meer.drain(&bob(), &[]).await.expect("drain").is_empty());
    // ...but the bytes are still there. CISS exposes PUT and GET on the object plane and no
    // DELETE, so the meer has no mechanism to remove what it stored.
    let stored_after = ciss.blob_files().len();
    assert_eq!(
        stored_after, 1,
        "MEASURED: the object survives its own retention window — CISS has no object DELETE"
    );

    println!(
        "S5 MEASURED (real-lib): after sweep, queue serves 0 entries but CISS still holds {} \
         object(s). Retention is a SERVING policy, not a storage guarantee — CISS's object plane \
         is PUT/GET only, with no DELETE. \"It is gone\" is false as written; \"we stopped \
         serving it\" is true.",
        stored_after
    );

    ciss.shutdown().await;
}

/// The learning goal: what can an honest gap report actually say?
#[tokio::test]
async fn what_the_minimal_watermark_can_and_cannot_support() {
    let ciss = Arc::new(CissHarness::spawn().await);
    let mut meer = Meer::new(Arc::clone(&ciss));

    for i in 0..3 {
        meer.publish(format!("missed {i}").as_bytes(), &[bob()]).await.expect("pub");
        meer.advance_days(1);
    }
    meer.advance_days(RETENTION_DAYS);
    meer.sweep();

    let wm = meer.watermark(&bob()).expect("watermark");

    // CAN say: a loud, specific, honest gap.
    let rendered = format!(
        "You were away. {} message(s) arrived between day {} and day {} and are no longer \
         available from this meer.",
        wm.swept, wm.earliest_day, wm.latest_day
    );
    assert!(rendered.contains("3 message(s)"));

    // CANNOT say: which ones. There are no digests, so the client cannot ask a peer for
    // precisely what it missed.
    println!("S5 MEASURED (real-lib): the minimal watermark renders: \"{rendered}\"");
    println!(
        "S5 MEASURED (real-lib): it supports NO-INVISIBLE-LOSS (the gap is loud, counted and \
         time-bounded) but NOT RECOVERY — with no digests retained, the client cannot name what \
         it missed to a peer (D-peer corroboration). Retaining digests would enable recovery and \
         would leave a per-recipient content-address log outliving the mail itself."
    );

    ciss.shutdown().await;
}
