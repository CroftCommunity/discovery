//! Wire parsing shared by every backend that speaks the real protocol (the fake
//! reading fixtures, and the real adapter reading the network). Gated behind
//! `serde` so the core never compiles it.
//!
//! The shape is `app.bsky.feed.getTimeline` / `getAuthorFeed`:
//! `{ "feed": [ { "post": <PostView> } … ], "cursor": "…"? }`. Both endpoints
//! return the same `FeedViewPost` shape; Phase 0 parses it into native posts.

use crate::port::TimelinePage;
use crate::types::Post;

#[derive(serde::Deserialize)]
struct TimelineResponse {
    feed: Vec<FeedItem>,
    cursor: Option<String>,
}

#[derive(serde::Deserialize)]
struct FeedItem {
    post: Post,
}

#[derive(serde::Deserialize)]
struct PostsResponse {
    posts: Vec<Post>,
}

/// Parse an `app.bsky.feed.getPosts` response (`{ "posts": [PostView] }`) into
/// native posts. Used to hydrate a pin from its address (BUILD-SPEC Phase 2
/// M2.4). An empty list means the target is gone (degraded pin).
pub fn parse_posts(raw: &str) -> Result<Vec<Post>, String> {
    let resp: PostsResponse = serde_json::from_str(raw).map_err(|e| {
        format!(
            "malformed posts JSON (parse error at line {}, column {})",
            e.line(),
            e.column()
        )
    })?;
    Ok(resp.posts)
}

/// Parse a real timeline/feed response body into a native page.
pub fn parse_timeline(raw: &str) -> Result<TimelinePage, String> {
    // Report only the parse position, never the error's rendered content: a
    // feed response can carry user data, so we keep payload snippets out of
    // error strings/logs.
    let resp: TimelineResponse = serde_json::from_str(raw).map_err(|e| {
        format!(
            "malformed timeline JSON (parse error at line {}, column {})",
            e.line(),
            e.column()
        )
    })?;
    Ok(TimelinePage {
        posts: resp.feed.into_iter().map(|item| item.post).collect(),
        next_cursor: resp.cursor,
    })
}
