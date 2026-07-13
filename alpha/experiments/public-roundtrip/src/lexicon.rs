//! The experimental record lexicon `org.croftc.experiment.feed.post` plus a
//! targeted validator. We deliberately validate this *one* record type by hand
//! rather than pulling a general Lexicon engine: the experiment only needs to
//! prove a custom-NSID record round-trips through a real PDS.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

/// The collection NSID for our experimental post record. The `org.croftc.experiment.*`
/// namespace makes it unambiguous that these are non-production records.
pub const POST_NSID: &str = "org.croftc.experiment.feed.post";

/// The read query NSID served by the minimal AppView.
pub const GET_TIMELINE_NSID: &str = "org.croftc.experiment.feed.getTimeline";

/// The threaded-read query NSID served by the minimal AppView.
pub const GET_THREAD_NSID: &str = "org.croftc.experiment.feed.getThread";

/// A content-addressed reference to another record (`uri` + `cid`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrongRef {
    pub uri: String,
    pub cid: String,
}

/// Links a reply to its thread `root` and immediate `parent`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyRef {
    pub root: StrongRef,
    pub parent: StrongRef,
}

/// An `org.croftc.experiment.feed.post` record body.
///
/// `$type` is mandatory on AT Protocol records, so we serialize it explicitly.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedPost {
    #[serde(rename = "$type")]
    pub r#type: String,
    pub text: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// Present only when this post is a reply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply: Option<ReplyRef>,
}

impl FeedPost {
    pub fn new(text: impl Into<String>, created_at: impl Into<String>) -> Self {
        FeedPost {
            r#type: POST_NSID.to_string(),
            text: text.into(),
            created_at: created_at.into(),
            reply: None,
        }
    }

    /// Build a reply post linking the thread `root` and immediate `parent`.
    pub fn reply(
        text: impl Into<String>,
        created_at: impl Into<String>,
        root: StrongRef,
        parent: StrongRef,
    ) -> Self {
        FeedPost {
            r#type: POST_NSID.to_string(),
            text: text.into(),
            created_at: created_at.into(),
            reply: Some(ReplyRef { root, parent }),
        }
    }

    /// Validate against the lexicon constraints before publishing or on ingest.
    /// Mirrors the JSON lexicon in `lexicons/org.croftc.experiment.feed.post.json`.
    pub fn validate(&self) -> Result<()> {
        if self.r#type != POST_NSID {
            bail!("$type must be exactly `{POST_NSID}`, got `{}`", self.r#type);
        }
        // text: required, string, maxLength 3000 (bytes), maxGraphemes 300.
        if self.text.is_empty() {
            bail!("text is required and must be non-empty");
        }
        if self.text.len() > 3000 {
            bail!("text exceeds maxLength 3000 bytes (was {})", self.text.len());
        }
        // Grapheme counting is approximated by char count here; sufficient for the
        // ASCII test content this experiment publishes.
        let graphemes = self.text.chars().count();
        if graphemes > 300 {
            bail!("text exceeds maxGraphemes 300 (was {graphemes})");
        }
        // createdAt: required, datetime (RFC 3339).
        if chrono::DateTime::parse_from_rfc3339(&self.created_at).is_err() {
            bail!("createdAt must be an RFC 3339 datetime, got `{}`", self.created_at);
        }
        // reply: optional, but if present both refs must be well-formed.
        if let Some(reply) = &self.reply {
            for (label, r) in [("reply.root", &reply.root), ("reply.parent", &reply.parent)] {
                if !r.uri.starts_with("at://") {
                    bail!("{label}.uri must be an at:// URI, got `{}`", r.uri);
                }
                if r.cid.is_empty() {
                    bail!("{label}.cid must be non-empty");
                }
            }
        }
        Ok(())
    }
}
