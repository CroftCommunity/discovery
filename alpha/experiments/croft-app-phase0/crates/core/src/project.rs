//! The projection: `Model -> FeedView`. Pure. The single seam between the
//! native Bluesky shape and the shared, display-ready chrome.

use crate::model::{FeedStatus, Model};
use crate::view::{Avatar, FeedView, Footer, PostCard, RetryAffordance};
use bluesky::types::Post;

pub fn project(model: &Model) -> FeedView {
    match &model.status {
        // Pre-load and cold load both read as "loading"; neither has posts.
        FeedStatus::Idle | FeedStatus::LoadingCold => FeedView::Loading,

        FeedStatus::Loaded { cursor } => {
            if model.posts.is_empty() {
                // P5: loaded-and-empty is its own state, not an error.
                FeedView::Empty {
                    message: "Nothing here yet.".to_string(),
                }
            } else {
                // P1 (None -> EndReached) / P2 (Some -> MoreAvailable).
                let footer = match cursor {
                    Some(_) => Footer::MoreAvailable,
                    None => Footer::EndReached,
                };
                FeedView::Feed {
                    posts: project_posts(&model.posts),
                    footer,
                }
            }
        }

        // Loading the next page: show the posts we have, with a footer that
        // says a page is on the way (distinct from "scroll to load more").
        FeedStatus::LoadingMore { .. } => FeedView::Feed {
            posts: project_posts(&model.posts),
            footer: Footer::LoadingMore,
        },

        // P6: cold failure -> full error view with a retry affordance.
        FeedStatus::ErrorCold { reason } => FeedView::Error {
            reason: reason.clone(),
            retry: RetryAffordance,
        },

        // P7: append failure -> the normal feed plus an inline-error footer,
        // never the full error view.
        FeedStatus::ErrorWhileAppended { reason, .. } => FeedView::Feed {
            posts: project_posts(&model.posts),
            footer: Footer::InlineError {
                reason: reason.clone(),
                retry: RetryAffordance,
            },
        },
    }
}

fn project_posts(posts: &[Post]) -> Vec<PostCard> {
    posts.iter().map(project_post).collect()
}

fn project_post(post: &Post) -> PostCard {
    PostCard {
        // The native id (URI) carried through as an opaque address for pinning.
        id: post.uri.clone(),
        // Fall back to the handle when the native shape carries no display name.
        author_display_name: post
            .author
            .display_name
            .clone()
            .unwrap_or_else(|| post.author.handle.clone()),
        handle: post.author.handle.clone(),
        body: post.record.text.clone(),
        // DECISION 3: absolute timestamp from the post's own carried value;
        // relative time is deferred because it would require the clock to enter
        // the projection. The core does not read the clock.
        timestamp: post.record.created_at.clone(),
        avatar: match &post.author.avatar {
            Some(url) => Avatar::Url(url.clone()),
            None => Avatar::NoAvatar,
        },
    }
}
