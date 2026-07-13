//! Functional core for the Croft group/messaging pond.
//!
//! `(model, intent) -> (model, effects)`. Pure: no network, no clock, no
//! storage, no screen, no async. The core describes effects as data; the shell
//! performs them and feeds results back as intents. Mirrors the feed pond's
//! `app-core` (`experiments/croft-app-phase0/crates/core`). The `Transport`
//! port lives in the shell, never here (DECISION 1).
#![warn(missing_docs)]

pub mod effect;
pub mod intent;
pub mod model;
pub mod project;
pub mod update;
pub mod view;
pub mod wire;

pub use effect::Effect;
pub use intent::Intent;
pub use model::{ChatMessage, Model};
pub use project::project;
pub use update::update;
pub use view::{MessageLine, ViewModel};
pub use wire::{deserialize, serialize, WireError};
