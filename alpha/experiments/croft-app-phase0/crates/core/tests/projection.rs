//! Projection acceptance tests (BUILD-SPEC §5), P1–P7. `project(&Model)` is pure;
//! view models must be renderable as plain text (the CLI proves this).

use app_core::{
    project, Avatar, Cursor, FeedStatus, FeedView, Footer, Model, PostCard, RetryAffordance,
};
use bluesky::types::{Author, Post, PostRecord};

fn cur(s: &str) -> Cursor {
    Cursor(s.to_string())
}

fn post_full(handle: &str, display: Option<&str>, text: &str, ts: &str, avatar: Option<&str>) -> Post {
    Post {
        uri: "at://did:plc:example/app.bsky.feed.post/x".to_string(),
        cid: "cid-x".to_string(),
        author: Author {
            did: "did:plc:example".to_string(),
            handle: handle.to_string(),
            display_name: display.map(str::to_string),
            avatar: avatar.map(str::to_string),
        },
        record: PostRecord {
            text: text.to_string(),
            created_at: ts.to_string(),
        },
        indexed_at: ts.to_string(),
    }
}

fn p(n: u32) -> Post {
    post_full(
        "alice.bsky.social",
        Some("Alice"),
        &format!("post {n}"),
        "2026-01-01T00:00:00.000Z",
        None,
    )
}

#[test]
fn p1_loaded_end_reached_lists_cards_in_order() {
    let model = Model {
        posts: vec![p(1), p(2), p(3)],
        status: FeedStatus::Loaded { cursor: None },
    };
    match project(&model) {
        FeedView::Feed { posts, footer } => {
            assert_eq!(posts.len(), 3);
            assert_eq!(posts[0].body, "post 1");
            assert_eq!(posts[1].body, "post 2");
            assert_eq!(posts[2].body, "post 3");
            assert_eq!(footer, Footer::EndReached);
        }
        other => panic!("expected Feed, got {other:?}"),
    }
}

#[test]
fn p2_loaded_with_cursor_shows_more_available() {
    let model = Model {
        posts: vec![p(1)],
        status: FeedStatus::Loaded {
            cursor: Some(cur("c1")),
        },
    };
    match project(&model) {
        FeedView::Feed { footer, .. } => assert_eq!(footer, Footer::MoreAvailable),
        other => panic!("expected Feed, got {other:?}"),
    }
}

#[test]
fn p3_post_card_has_display_ready_fields() {
    // With display name and avatar present.
    let with_avatar = Model {
        posts: vec![post_full(
            "alice.bsky.social",
            Some("Alice"),
            "hello world",
            "2026-03-04T12:00:00.000Z",
            Some("https://cdn.example/avatar.jpg"),
        )],
        status: FeedStatus::Loaded { cursor: None },
    };
    let card = first_card(project(&with_avatar));
    assert_eq!(card.author_display_name, "Alice");
    assert_eq!(card.handle, "alice.bsky.social");
    assert_eq!(card.body, "hello world");
    // Absolute timestamp derived from the post's own data (DECISION 3).
    assert_eq!(card.timestamp, "2026-03-04T12:00:00.000Z");
    assert_eq!(
        card.avatar,
        Avatar::Url("https://cdn.example/avatar.jpg".to_string())
    );

    // Missing avatar -> explicit NoAvatar, never a null for the renderer.
    let no_avatar = Model {
        posts: vec![post_full(
            "bob.bsky.social",
            None,
            "hi",
            "2026-03-04T12:00:00.000Z",
            None,
        )],
        status: FeedStatus::Loaded { cursor: None },
    };
    let card = first_card(project(&no_avatar));
    assert_eq!(card.avatar, Avatar::NoAvatar);
    // No display name -> fall back to the handle.
    assert_eq!(card.author_display_name, "bob.bsky.social");
}

#[test]
fn p2b_loading_more_shows_loading_more_footer_over_posts() {
    let model = Model {
        posts: vec![p(1), p(2)],
        status: FeedStatus::LoadingMore { cursor: cur("c1") },
    };
    match project(&model) {
        FeedView::Feed { posts, footer } => {
            assert_eq!(posts.len(), 2);
            assert_eq!(footer, Footer::LoadingMore);
        }
        other => panic!("expected Feed, got {other:?}"),
    }
}

#[test]
fn p4_loading_cold_is_a_loading_view() {
    let model = Model {
        posts: vec![],
        status: FeedStatus::LoadingCold,
    };
    assert_eq!(project(&model), FeedView::Loading);
}

#[test]
fn p5_loaded_empty_is_empty_view_with_message() {
    let model = Model {
        posts: vec![],
        status: FeedStatus::Loaded { cursor: None },
    };
    match project(&model) {
        FeedView::Empty { message } => assert!(!message.is_empty()),
        other => panic!("expected Empty, got {other:?}"),
    }
}

#[test]
fn p6_error_cold_is_full_error_with_retry() {
    let model = Model {
        posts: vec![],
        status: FeedStatus::ErrorCold {
            reason: "offline".to_string(),
        },
    };
    match project(&model) {
        FeedView::Error { reason, retry } => {
            assert_eq!(reason, "offline");
            assert_eq!(retry, RetryAffordance);
        }
        other => panic!("expected Error, got {other:?}"),
    }
}

#[test]
fn p7_error_while_appended_is_feed_with_inline_error() {
    let model = Model {
        posts: vec![p(1), p(2)],
        status: FeedStatus::ErrorWhileAppended {
            reason: "timeout".to_string(),
            cursor: cur("c1"),
        },
    };
    match project(&model) {
        FeedView::Feed { posts, footer } => {
            assert_eq!(posts.len(), 2);
            match footer {
                Footer::InlineError { reason, retry } => {
                    assert_eq!(reason, "timeout");
                    assert_eq!(retry, RetryAffordance);
                }
                other => panic!("expected InlineError footer, got {other:?}"),
            }
        }
        other => panic!("expected Feed, got {other:?}"),
    }
}

fn first_card(view: FeedView) -> PostCard {
    match view {
        FeedView::Feed { posts, .. } => posts.into_iter().next().expect("at least one card"),
        other => panic!("expected Feed, got {other:?}"),
    }
}
