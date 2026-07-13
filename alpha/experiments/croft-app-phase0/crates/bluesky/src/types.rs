//! Native Bluesky shapes. Deliberately NOT normalized into a cross-ecosystem
//! type (honest seams: there is no shared post model). This mirrors the relevant
//! slice of atproto's `app.bsky.feed.defs#postView`. It is the type the core
//! imports for its model (DECISION 2).
//!
//! serde derives are gated behind the `serde` feature so the core (which depends
//! on this crate with default features off) pulls in no serialization surface.

/// A post as Bluesky presents it. Phase 0 carries only what the projection needs;
/// grow the shape by use case.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Post {
    pub uri: String,
    pub cid: String,
    pub author: Author,
    pub record: PostRecord,
    #[cfg_attr(feature = "serde", serde(rename = "indexedAt"))]
    pub indexed_at: String,
}

/// The post author (atproto `profileViewBasic`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Author {
    pub did: String,
    pub handle: String,
    #[cfg_attr(feature = "serde", serde(rename = "displayName"))]
    pub display_name: Option<String>,
    pub avatar: Option<String>,
}

/// The post record (atproto `app.bsky.feed.post`). Phase 0 needs text and the
/// creation time only.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PostRecord {
    pub text: String,
    #[cfg_attr(feature = "serde", serde(rename = "createdAt"))]
    pub created_at: String,
}
