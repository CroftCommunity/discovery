//! View models: ready-to-render, presentation-agnostic data.
//!
//! These must render equally to a terminal or a DOM. No view model may contain
//! a protocol type or anything that assumes a specific renderer. (The CLI is the
//! proof: if it cannot render one of these as text, the view model is wrong.)

/// What the feed looks like right now, ready to paint.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FeedView {
    /// First load in flight; distinct from empty.
    Loading,
    /// Loaded successfully but there is nothing to show.
    Empty { message: String },
    /// One or more posts, with a footer describing what is below them.
    Feed { posts: Vec<PostCard>, footer: Footer },
    /// A full-screen error (cold failure), with a retry affordance.
    Error {
        reason: String,
        retry: RetryAffordance,
    },
}

/// A single post, with every field display-ready. No protocol type appears here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PostCard {
    /// The item's stable native id (e.g. its at:// URI). An opaque string, not
    /// a protocol type — it is what the shell records when pinning the item.
    pub id: String,
    pub author_display_name: String,
    pub handle: String,
    pub body: String,
    /// Absolute timestamp string from the post's own data (DECISION 3).
    pub timestamp: String,
    pub avatar: Avatar,
}

/// Avatar is explicit: a URL or an explicit absence. Never a null for the
/// renderer to puzzle over.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Avatar {
    Url(String),
    NoAvatar,
}

/// What sits below the posts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Footer {
    /// More can be loaded.
    MoreAvailable,
    /// A next page is in flight (posts already shown above).
    LoadingMore,
    /// The true end of the feed.
    EndReached,
    /// A load-more failed, but the existing posts are intact. Inline, not a
    /// full error view (P7).
    InlineError {
        reason: String,
        retry: RetryAffordance,
    },
}

/// Marker that a retry control should be offered. Carries no behavior; the shell
/// wires the actual intent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RetryAffordance;
