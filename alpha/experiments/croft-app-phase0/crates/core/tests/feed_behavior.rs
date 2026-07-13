//! Behavior acceptance tests (BUILD-SPEC §4), groups A–D. Given/when/then against
//! `update`. No mocks, deterministic. The milestone is not done until all pass.

use app_core::{update, Cursor, Effect, FeedStatus, Intent, Model};
use bluesky::types::{Author, Post, PostRecord};

// --- helpers ---

fn post(id: &str) -> Post {
    Post {
        uri: format!("at://did:plc:example/app.bsky.feed.post/{id}"),
        cid: format!("cid-{id}"),
        author: Author {
            did: "did:plc:example".to_string(),
            handle: "alice.bsky.social".to_string(),
            display_name: Some("Alice".to_string()),
            avatar: None,
        },
        record: PostRecord {
            text: format!("post {id}"),
            created_at: "2026-01-01T00:00:00.000Z".to_string(),
        },
        indexed_at: "2026-01-01T00:00:00.000Z".to_string(),
    }
}

fn cur(s: &str) -> Cursor {
    Cursor(s.to_string())
}

fn loaded(cursor: Option<Cursor>, posts: Vec<Post>) -> Model {
    Model {
        posts,
        status: FeedStatus::Loaded { cursor },
    }
}

// --- Group A: opening ---

#[test]
fn a1_idle_open_starts_cold_load() {
    let (model, effects) = update(Model::new(), Intent::OpenFeed);
    assert_eq!(model.status, FeedStatus::LoadingCold);
    assert!(model.posts.is_empty());
    assert_eq!(effects, vec![Effect::FetchFeed { cursor: None }]);
}

#[test]
fn a2_open_when_loaded_is_noop() {
    let start = loaded(Some(cur("c1")), vec![post("1")]);
    let (model, effects) = update(start.clone(), Intent::OpenFeed);
    assert_eq!(model, start);
    assert!(effects.is_empty());
}

#[test]
fn a3_open_when_loading_cold_is_noop() {
    let start = Model {
        posts: vec![],
        status: FeedStatus::LoadingCold,
    };
    let (model, effects) = update(start.clone(), Intent::OpenFeed);
    assert_eq!(model, start);
    assert!(effects.is_empty());
}

// --- Group B: first page arriving ---

#[test]
fn b1_cold_load_resolves_to_loaded_with_cursor() {
    let start = Model {
        posts: vec![],
        status: FeedStatus::LoadingCold,
    };
    let page = vec![post("1"), post("2")];
    let (model, effects) = update(
        start,
        Intent::FeedPageLoaded {
            posts: page.clone(),
            next_cursor: Some(cur("next")),
        },
    );
    assert_eq!(
        model.status,
        FeedStatus::Loaded {
            cursor: Some(cur("next"))
        }
    );
    assert_eq!(model.posts, page);
    assert!(effects.is_empty());
}

#[test]
fn b2_cold_load_empty_is_loaded_not_error() {
    let start = Model {
        posts: vec![],
        status: FeedStatus::LoadingCold,
    };
    let (model, effects) = update(
        start,
        Intent::FeedPageLoaded {
            posts: vec![],
            next_cursor: None,
        },
    );
    assert_eq!(model.status, FeedStatus::Loaded { cursor: None });
    assert!(model.posts.is_empty());
    assert!(effects.is_empty());
}

#[test]
fn b3_cold_load_failure_is_error_cold_no_retry() {
    let start = Model {
        posts: vec![],
        status: FeedStatus::LoadingCold,
    };
    let (model, effects) = update(
        start,
        Intent::FeedLoadFailed {
            reason: "boom".to_string(),
        },
    );
    assert_eq!(
        model.status,
        FeedStatus::ErrorCold {
            reason: "boom".to_string()
        }
    );
    assert!(model.posts.is_empty());
    assert!(effects.is_empty());
}

// --- Group C: load-more ---

#[test]
fn c1_reached_end_with_cursor_loads_more() {
    let start = loaded(Some(cur("c1")), vec![post("1")]);
    let (model, effects) = update(start, Intent::FeedReachedEnd);
    assert_eq!(
        model.status,
        FeedStatus::LoadingMore { cursor: cur("c1") }
    );
    assert_eq!(
        effects,
        vec![Effect::FetchFeed {
            cursor: Some(cur("c1"))
        }]
    );
}

#[test]
fn c2_page_loaded_appends_after_existing() {
    let start = Model {
        posts: vec![post("1"), post("2")],
        status: FeedStatus::LoadingMore { cursor: cur("c1") },
    };
    let (model, effects) = update(
        start,
        Intent::FeedPageLoaded {
            posts: vec![post("3"), post("4")],
            next_cursor: Some(cur("c2")),
        },
    );
    assert_eq!(
        model.status,
        FeedStatus::Loaded {
            cursor: Some(cur("c2"))
        }
    );
    assert_eq!(
        model.posts,
        vec![post("1"), post("2"), post("3"), post("4")]
    );
    assert!(effects.is_empty());
}

#[test]
fn c3_reached_end_at_true_end_is_noop() {
    let start = loaded(None, vec![post("1")]);
    let (model, effects) = update(start.clone(), Intent::FeedReachedEnd);
    assert_eq!(model, start);
    assert!(effects.is_empty());
}

#[test]
fn c4_reached_end_while_loading_more_is_noop() {
    let start = Model {
        posts: vec![post("1")],
        status: FeedStatus::LoadingMore { cursor: cur("c1") },
    };
    let (model, effects) = update(start.clone(), Intent::FeedReachedEnd);
    assert_eq!(model, start);
    assert!(effects.is_empty());
}

#[test]
fn c5_load_more_failure_preserves_posts_and_cursor() {
    let start = Model {
        posts: vec![post("1"), post("2")],
        status: FeedStatus::LoadingMore { cursor: cur("c1") },
    };
    let (model, effects) = update(
        start,
        Intent::FeedLoadFailed {
            reason: "net".to_string(),
        },
    );
    assert_eq!(
        model.status,
        FeedStatus::ErrorWhileAppended {
            reason: "net".to_string(),
            cursor: cur("c1"),
        }
    );
    assert_eq!(model.posts, vec![post("1"), post("2")]);
    assert!(effects.is_empty());
}

// --- Group D: retry ---

#[test]
fn d1_retry_cold_error_reloads_from_none() {
    let start = Model {
        posts: vec![],
        status: FeedStatus::ErrorCold {
            reason: "x".to_string(),
        },
    };
    let (model, effects) = update(start, Intent::RetryRequested);
    assert_eq!(model.status, FeedStatus::LoadingCold);
    assert_eq!(effects, vec![Effect::FetchFeed { cursor: None }]);
}

#[test]
fn d2_retry_append_error_resumes_from_variant_cursor() {
    let start = Model {
        posts: vec![post("1")],
        status: FeedStatus::ErrorWhileAppended {
            reason: "x".to_string(),
            cursor: cur("c1"),
        },
    };
    let (model, effects) = update(start, Intent::RetryRequested);
    assert_eq!(
        model.status,
        FeedStatus::LoadingMore { cursor: cur("c1") }
    );
    assert_eq!(
        effects,
        vec![Effect::FetchFeed {
            cursor: Some(cur("c1"))
        }]
    );
    assert_eq!(model.posts, vec![post("1")]);
}

// --- 0c: the cursor-invariant debug assertion ---

// A transition that produces a cursor-mandatory state with an empty (junk)
// cursor must trip the debug assertion. This is the mechanical guard that a
// future state addition can't quietly violate the invariant. Debug builds only
// (release strips debug_assert!), so the test is gated to match.
#[cfg(debug_assertions)]
#[test]
#[should_panic(expected = "cursor invariant")]
fn zz_0c_empty_cursor_trips_the_invariant_assertion() {
    let start = Model {
        posts: vec![],
        status: FeedStatus::LoadingCold,
    };
    // A page that comes back advertising an empty-string cursor as "more
    // available" would build Loaded { cursor: Some("") } — illegal.
    let _ = update(
        start,
        Intent::FeedPageLoaded {
            posts: vec![post("1")],
            next_cursor: Some(Cursor(String::new())),
        },
    );
}

