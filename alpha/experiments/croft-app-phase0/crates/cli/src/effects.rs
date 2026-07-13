//! Performing effects. This is the shell's job and the only place the port is
//! held and called (DECISION 1). The native HTTP client would live here too;
//! Phase 0 uses the fake.

use app_core::{Cursor, Effect, Intent};
use bluesky::port::BlueskyPort;

/// Perform one effect against the port and translate the outcome into the
/// follow-up intent the core expects. The core never does this translation.
pub async fn perform_effect<P: BlueskyPort>(effect: &Effect, port: &P) -> Intent {
    match effect {
        Effect::FetchFeed { cursor } => {
            // Map the core's opaque Cursor down to the protocol string the port
            // speaks, and map the result back up into an intent.
            let token = cursor.as_ref().map(|c| c.0.clone());
            match port.fetch_timeline(token).await {
                Ok(page) => Intent::FeedPageLoaded {
                    posts: page.posts,
                    next_cursor: page.next_cursor.map(Cursor),
                },
                Err(reason) => Intent::FeedLoadFailed { reason },
            }
        }
    }
}
