//! Functional core for the Phase 0 feed.
//!
//! `(state, intent) -> (state, effects)`. Pure: no network, no clock, no
//! storage, no screen, no async. The core describes effects as data; the shell
//! performs them and feeds results back as intents. See `design-philosophy.md`
//! and `BUILD-SPEC.md`.

pub mod effect;
pub mod intent;
pub mod model;
pub mod project;
pub mod update;
pub mod view;

pub use effect::Effect;
pub use intent::Intent;
pub use model::{Cursor, FeedStatus, Model};
pub use project::project;
pub use update::update;
pub use view::{Avatar, FeedView, Footer, PostCard, RetryAffordance};
