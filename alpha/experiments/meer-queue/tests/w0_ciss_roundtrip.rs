//! **Phase 1 wiring test** — the spike's storage boundary is the real CISS server, reached
//! over real loopback HTTP.
//!
//! This is a wiring test, not a unit test: it drives `CissHarness` end to end through
//! `App::router()` into the real blobstore, so a green run proves the call chain is live —
//! not merely that a helper compiles.
//!
//! What it pins, and why each assertion is here:
//!
//! - **Round trip.** Bytes PUT come back byte-identical when GET by content address. This is
//!   the foundation M2's digest chain stands on.
//! - **The cap at BOTH edges.** Exactly `MAX_OBJECT_BYTES` is accepted; one byte more is
//!   refused. Testing only the over-cap side is a single-point assertion that an off-by-one
//!   in the comparison would survive (Pass 3, mutation resistance).
//! - **The refusal is HTTP 413**, from axum's `DefaultBodyLimit` — *not* CISS's
//!   `ObjectTooLarge`. Phase 0's D2 probe established that the request never reaches the
//!   blobstore, so the blobstore's own check is a second line of defence this path never
//!   exercises. Asserting a CISS error here would assert something that cannot happen.
//! - **Dedup is CISS's, not ours.** Identical bytes yield one content address AND one file on
//!   disk. S2's fan-out claim later needs a source independent of our own bookkeeping; this
//!   is where that source is established.

use meer_queue::ciss_harness::{CissHarness, MAX_OBJECT_BYTES};

#[tokio::test]
async fn objects_round_trip_through_the_real_ciss_boundary() {
    let harness = CissHarness::spawn().await;
    let alice = harness.identity("alice");

    let payload = b"the conversation stays alive while you sleep";
    let put = harness.put_object(&alice, "greeting", payload).await;
    assert_eq!(put.status, 200, "PUT should succeed: {}", put.body_text());
    let cid = put.cid().expect("PUT returns a content address");

    let got = harness.get_object(&alice, &cid).await;
    assert_eq!(got.status, 200, "GET should succeed");
    assert_eq!(
        got.body, payload,
        "bytes must survive the content-address round trip unchanged"
    );

    harness.shutdown().await;
}

#[tokio::test]
async fn the_two_mib_cap_is_enforced_at_both_edges() {
    let harness = CissHarness::spawn().await;
    let alice = harness.identity("alice");

    // At the boundary: accepted.
    let at_cap = harness
        .put_object(&alice, "at-cap", &vec![0u8; MAX_OBJECT_BYTES])
        .await;
    assert_eq!(
        at_cap.status, 200,
        "an object of exactly MAX_OBJECT_BYTES must be accepted"
    );

    // One byte over: refused, and refused at the HTTP boundary (Phase 0 D2).
    let over_cap = harness
        .put_object(&alice, "over-cap", &vec![0u8; MAX_OBJECT_BYTES + 1])
        .await;
    assert_eq!(
        over_cap.status, 413,
        "one byte over the cap must be refused with 413 (axum DefaultBodyLimit), \
         got {} / {}",
        over_cap.status,
        over_cap.body_text()
    );

    harness.shutdown().await;
}

#[tokio::test]
async fn identical_bytes_are_stored_once() {
    let harness = CissHarness::spawn().await;
    let alice = harness.identity("alice");

    let bytes = b"identical payload";
    let first = harness.put_object(&alice, "copy-a", bytes).await;
    let second = harness.put_object(&alice, "copy-b", bytes).await;
    assert_eq!(first.status, 200);
    assert_eq!(second.status, 200);
    assert_eq!(
        first.cid().expect("cid"),
        second.cid().expect("cid"),
        "content addressing must give identical bytes one address"
    );

    // Independent of our bookkeeping: what actually landed on disk.
    let files = harness.blob_files();
    assert_eq!(
        files.len(),
        1,
        "two PUTs of identical bytes must leave ONE blob on disk, found: {files:?}"
    );

    // And distinct bytes are not collapsed — without this, an implementation that always
    // stored exactly one object would pass the assertion above.
    let other = harness.put_object(&alice, "different", b"a different payload").await;
    assert_eq!(other.status, 200);
    assert_eq!(
        harness.blob_files().len(),
        2,
        "distinct bytes must be stored distinctly"
    );

    harness.shutdown().await;
}
