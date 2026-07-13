//! The Bluesky port: the narrow trait the *shell* depends on (DECISION 1).
//!
//! The core never sees this trait. The shell holds an implementation, calls it
//! to perform a `FetchFeed` effect, and turns the result into a `FeedPageLoaded`
//! or `FeedLoadFailed` intent.

use crate::types::Post;
use std::future::Future;

/// One page of the timeline: the native posts plus the cursor for the next page
/// (`None` = the true end). The cursor is a plain protocol string; the shell maps
/// it to/from the core's `Cursor`, so this crate stays ignorant of the core.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimelinePage {
    pub posts: Vec<Post>,
    pub next_cursor: Option<String>,
}

/// Fetch a page of the timeline given an optional cursor. Async because the
/// shell side performs I/O. Phase 0 has exactly this one capability.
pub trait BlueskyPort {
    fn fetch_timeline(
        &self,
        cursor: Option<String>,
    ) -> impl Future<Output = Result<TimelinePage, String>>;
}
