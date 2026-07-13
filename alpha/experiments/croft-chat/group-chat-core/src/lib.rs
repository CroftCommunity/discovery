//! Chat tenant — one implementation attached to the social-graph substrate.
//!
//! Pure core: `Intent` / `Effect` / `update` / `project` / view models for the
//! two-pane TUI. Depends on `social-graph-core` for substrate types and never
//! reaches around it into redb.
//!
//! Phases populate this crate: P8 (model + update), P9 (project + view),
//! P14 (channel selection state).
#![warn(missing_docs)]

pub mod model;
pub mod project;
pub mod update;
pub mod view;

pub use model::{Effect, GroupRef, Intent, MessageLine, Model, Snapshot};
pub use project::project;
pub use update::update;
pub use view::{
    ChannelNode, ChatView, GraphTreeView, GroupNode, TimelineLineView, TimelineView, TreeRow,
};
