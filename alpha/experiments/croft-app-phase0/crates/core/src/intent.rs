//! Intents: small descriptions of what happened or what the user wants.
//! They come *in* across the boundary; they are not function calls into logic.

use crate::model::Cursor;
use bluesky::types::Post;

#[derive(Debug, Clone, PartialEq)]
pub enum Intent {
    /// The user opened the feed.
    OpenFeed,
    /// The user scrolled to the end (a request to load more).
    FeedReachedEnd,
    /// A successful fetch result coming back in.
    FeedPageLoaded {
        posts: Vec<Post>,
        next_cursor: Option<Cursor>,
    },
    /// A failed fetch result coming back in (DECISION 5: reason is a string).
    FeedLoadFailed { reason: String },
    /// The user asked to retry after an error.
    RetryRequested,
}
