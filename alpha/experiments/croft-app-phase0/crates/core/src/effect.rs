//! Effects: what the core asks the shell to do, expressed as data.
//! The core emits these and returns; it never performs them and never calls
//! the port to fulfill them (DECISION 1).

use crate::model::Cursor;

#[derive(Debug, Clone, PartialEq)]
pub enum Effect {
    /// Fetch a page of the feed. `None` cursor = the first (cold) page.
    FetchFeed { cursor: Option<Cursor> },
}
