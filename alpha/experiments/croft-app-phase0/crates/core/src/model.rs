//! The model: loaded posts (native shape, DECISION 2) plus the load lifecycle.

use bluesky::types::Post;

/// An opaque pagination token. The core never interprets it; it carries it from
/// a `FeedPageLoaded` intent into the status and back out in a `FetchFeed`
/// effect. (The protocol's cursor is a string; the shell maps to/from it.)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cursor(pub String);

/// The load lifecycle. One axis, six variants (philosophy §9 / BUILD-SPEC §3).
///
/// Cursor-bearing states carry the cursor in the variant (DECISION 4), so a
/// recovery state with nothing to retry from is unrepresentable. The invariant
/// is proven in BUILD-SPEC §3a; preserve it if you add states.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FeedStatus {
    /// Never opened. No posts, no cursor.
    Idle,
    /// First load in flight, no posts yet, no cursor.
    LoadingCold,
    /// Posts present, nothing in flight. `Some` = more available; `None` = true
    /// end. This is the only meaningful `Option<Cursor>` in the status.
    Loaded { cursor: Option<Cursor> },
    /// Posts present, a next-page load in flight from this cursor.
    LoadingMore { cursor: Cursor },
    /// First load failed, no posts. Carries a reason (DECISION 5).
    ErrorCold { reason: String },
    /// A load-more failed but posts are still present. Carries a reason and the
    /// preserved cursor so retry can always resume (DECISION 4).
    ErrorWhileAppended { reason: String, cursor: Cursor },
}

/// The whole core state: the native posts and where we are in the lifecycle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Model {
    pub posts: Vec<Post>,
    pub status: FeedStatus,
}

impl Model {
    /// A fresh, never-opened model.
    pub fn new() -> Self {
        Model {
            posts: Vec::new(),
            status: FeedStatus::Idle,
        }
    }
}

impl Default for Model {
    fn default() -> Self {
        Model::new()
    }
}
