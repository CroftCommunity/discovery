//! Hydrated view objects served by the AppView. These match the `#postView` /
//! `#reactionView` definitions in the read lexicons (getTimeline / getPostThread).
//! Pure data + serde; no knowledge of storage or the private stack.

use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct ReactionView {
    pub emoji: String,
    /// DID of the reactor (author of the reaction record).
    pub reactor: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct PostView {
    pub uri: String,
    /// DID of the post author.
    pub author: String,
    pub text: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// Reactions hydrated onto this post (the join the AppView performs).
    pub reactions: Vec<ReactionView>,
}
