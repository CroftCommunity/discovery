//! Parse the committed (real, recorded) fixtures through the same `wire`
//! parser the real adapter uses. No network. Satisfies the M6 DoD: "adapter
//! parsing is tested against fixtures."
//!
//! Run with the parsing surface enabled, e.g.:
//!   cargo test -p bluesky --features adapter
#![cfg(feature = "serde")]

use bluesky::wire::parse_timeline;
use std::fs;
use std::path::PathBuf;

fn fixture(name: &str) -> String {
    let path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "fixtures", name].iter().collect();
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn page_1_parses_into_native_posts_with_a_cursor() {
    let page = parse_timeline(&fixture("timeline_page_1.json")).expect("page 1 parses");
    assert!(!page.posts.is_empty(), "page 1 should carry posts");
    assert!(
        page.next_cursor.is_some(),
        "page 1 should advertise more (a next cursor)"
    );
    // Native fields survived parsing.
    let first = &page.posts[0];
    assert!(!first.author.handle.is_empty());
    assert!(!first.uri.is_empty());
    assert!(!first.record.created_at.is_empty());
}

#[test]
fn page_2_parses_and_follows_page_1() {
    let p1 = parse_timeline(&fixture("timeline_page_1.json")).unwrap();
    let p2 = parse_timeline(&fixture("timeline_page_2.json")).expect("page 2 parses");
    assert!(!p2.posts.is_empty());
    // The two pages are distinct recordings (different leading post).
    assert_ne!(p1.posts[0].uri, p2.posts[0].uri);
}

#[test]
fn empty_fixture_parses_to_no_posts_and_no_cursor() {
    let page = parse_timeline(&fixture("timeline_empty.json")).expect("empty parses");
    assert!(page.posts.is_empty());
    assert!(page.next_cursor.is_none());
}

#[test]
fn malformed_json_errors_without_leaking_content() {
    let payload = "{ not valid json ";
    let err = parse_timeline(payload).unwrap_err();
    assert!(err.contains("malformed timeline JSON"));
    // Position only; the raw payload must never appear in the error.
    assert!(err.contains("line"));
    assert!(!err.contains(payload));
}
