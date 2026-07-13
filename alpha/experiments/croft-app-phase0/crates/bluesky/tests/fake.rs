//! The fake serves the committed fixtures deterministically across its modes,
//! with no network. Satisfies the M4 DoD. Run with:
//!   cargo test -p bluesky --features fake
#![cfg(feature = "fake")]

use bluesky::fake::{default_fixtures_dir, FakeBluesky, FakeMode};
use bluesky::port::BlueskyPort;

// The fake's futures are always immediately ready, so a poll-once driver is
// enough to test it without an async runtime.
fn now<F: std::future::Future>(fut: F) -> F::Output {
    use std::ptr;
    use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
    static VT: RawWakerVTable =
        RawWakerVTable::new(|_| RawWaker::new(ptr::null(), &VT), |_| {}, |_| {}, |_| {});
    let waker = unsafe { Waker::from_raw(RawWaker::new(ptr::null(), &VT)) };
    let mut cx = Context::from_waker(&waker);
    let mut fut = std::pin::pin!(fut);
    match fut.as_mut().poll(&mut cx) {
        Poll::Ready(v) => v,
        Poll::Pending => panic!("fake future pended unexpectedly"),
    }
}

fn fake(mode: FakeMode) -> FakeBluesky {
    FakeBluesky::new(default_fixtures_dir()).with_mode(mode)
}

#[test]
fn normal_mode_serves_page_1_then_page_2_by_cursor() {
    let f = fake(FakeMode::Normal);

    let p1 = now(f.fetch_timeline(None)).expect("page 1");
    assert!(!p1.posts.is_empty());
    let cursor = p1.next_cursor.clone().expect("page 1 has a next cursor");

    let p2 = now(f.fetch_timeline(Some(cursor))).expect("page 2");
    assert!(!p2.posts.is_empty());
    assert_ne!(p1.posts[0].uri, p2.posts[0].uri);

    // Deterministic: same input, same output.
    let p1_again = now(f.fetch_timeline(None)).unwrap();
    assert_eq!(p1.posts, p1_again.posts);
}

#[test]
fn an_unexpected_cursor_is_rejected() {
    let f = fake(FakeMode::Normal);
    let err = now(f.fetch_timeline(Some("not-the-real-cursor".to_string()))).unwrap_err();
    assert!(err.contains("unexpected cursor"));
    // The bogus token itself is not echoed (length fingerprint only).
    assert!(!err.contains("not-the-real-cursor"));
}

#[test]
fn empty_mode_serves_an_empty_feed() {
    let page = now(fake(FakeMode::Empty).fetch_timeline(None)).expect("empty");
    assert!(page.posts.is_empty());
    assert!(page.next_cursor.is_none());
}

#[test]
fn error_mode_always_fails() {
    let err = now(fake(FakeMode::Error).fetch_timeline(None)).unwrap_err();
    assert!(err.contains("simulated fetch failure"));
}
