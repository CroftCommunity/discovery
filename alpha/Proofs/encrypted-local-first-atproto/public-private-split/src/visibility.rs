//! Visibility policy for the public/private split.
//!
//! Visibility is *client-side metadata*, deliberately NOT a field on the record:
//! the lexicon record types are closed schemas (a `visibility` property would
//! make the record invalid and, worse, would itself be published). So the author
//! tracks per-record visibility out of band, keyed by the record's private
//! AT-URI. The mirror policy reads this to decide what crosses the boundary.

use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Visibility {
    /// Group-only; encrypted; never leaves the private space.
    Private,
    /// Eligible to be mirrored into the public atproto space (cleartext).
    Public,
}

/// Per-record visibility decisions, keyed by the record's private AT-URI.
#[derive(Default)]
pub struct MirrorPolicy {
    decisions: HashMap<String, Visibility>,
}

impl MirrorPolicy {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&mut self, uri: &str, v: Visibility) {
        self.decisions.insert(uri.to_string(), v);
    }

    /// Default-deny: anything not explicitly marked public stays private. This
    /// is the safe default for a privacy boundary — a missing decision must
    /// never leak.
    pub fn visibility(&self, uri: &str) -> Visibility {
        self.decisions.get(uri).copied().unwrap_or(Visibility::Private)
    }
}
