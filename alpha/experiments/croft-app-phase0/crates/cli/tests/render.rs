//! Render-coverage tests: every view state must render to non-empty text. This
//! is the Phase 0 text-mode stand-in for the per-state snapshot rule
//! (philosophy §8) — an undesigned state shows up as a failing/empty render.

use app_core::{Avatar, FeedView, Footer, PostCard, RetryAffordance};
use pond_cli::render::render;

fn card() -> PostCard {
    PostCard {
        id: "at://did:plc:example/app.bsky.feed.post/1".to_string(),
        author_display_name: "Alice".to_string(),
        handle: "alice.bsky.social".to_string(),
        body: "hello".to_string(),
        timestamp: "2026-01-01T00:00:00.000Z".to_string(),
        avatar: Avatar::NoAvatar,
    }
}

fn states() -> Vec<FeedView> {
    vec![
        FeedView::Loading,
        FeedView::Empty {
            message: "Nothing here yet.".to_string(),
        },
        FeedView::Feed {
            posts: vec![card()],
            footer: Footer::MoreAvailable,
        },
        FeedView::Feed {
            posts: vec![card()],
            footer: Footer::EndReached,
        },
        FeedView::Feed {
            posts: vec![card()],
            footer: Footer::LoadingMore,
        },
        FeedView::Feed {
            posts: vec![card()],
            footer: Footer::InlineError {
                reason: "timeout".to_string(),
                retry: RetryAffordance,
            },
        },
        FeedView::Error {
            reason: "offline".to_string(),
            retry: RetryAffordance,
        },
    ]
}

#[test]
fn every_view_state_renders_to_nonempty_text() {
    for state in states() {
        let text = render(&state);
        assert!(!text.trim().is_empty(), "empty render for {state:?}");
    }
}

#[test]
fn avatar_present_and_absent_both_render() {
    let with = FeedView::Feed {
        posts: vec![PostCard {
            avatar: Avatar::Url("https://cdn.example/a.jpg".to_string()),
            ..card()
        }],
        footer: Footer::EndReached,
    };
    assert!(render(&with).contains("https://cdn.example/a.jpg"));

    let without = FeedView::Feed {
        posts: vec![card()],
        footer: Footer::EndReached,
    };
    assert!(render(&without).contains("no avatar"));
}
