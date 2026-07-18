//! Auditable reach (RUN-18 B6; PUBLICATIONS.md §4 "Reach" row, GROUPS.md A.4).
//!
//! In the open tier a roster is a pure projection of public records, so a
//! subscriber count is never anything a serving node merely asserts: any
//! auditor folding the same records re-derives it exactly. This module is the
//! auditor's side — an INDEPENDENT fold (it shares no state with any DS; it
//! calls the same public fold the DS does, over the same public records, which
//! is the point) and a verdict for a claimed count.

use crate::fold::FoldState;
use crate::source::SourceEvent;

/// The auditor's verdict on a served subscriber count.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CountVerdict {
    /// The folded records support exactly the asserted count.
    Supported,
    /// The asserted count has no folded records behind it; `derived` is the
    /// number the records actually support.
    Unsupported {
        /// The count as asserted by the serving node.
        asserted: usize,
        /// The count an independent fold derives from the records.
        derived: usize,
    },
}

/// Independently re-derive the current roster count of `scope` from the
/// public records alone.
///
/// # Panics
/// Never in practice: the fold errors only on structurally undecodable
/// records, which the harness sources do not produce; an undecodable stream
/// would be a test failure, not a silent zero.
#[must_use]
pub fn derive_roster_count(events: &[SourceEvent], scope: &str) -> usize {
    FoldState::run(events)
        .expect("auditor fold over well-formed records")
        .roster_members(scope)
        .len()
}

/// Audit an asserted subscriber count against the records.
#[must_use]
pub fn audit_count(events: &[SourceEvent], scope: &str, asserted: usize) -> CountVerdict {
    let derived = derive_roster_count(events, scope);
    if derived == asserted {
        CountVerdict::Supported
    } else {
        CountVerdict::Unsupported { asserted, derived }
    }
}
