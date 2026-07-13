#![warn(missing_docs)]
//! `croft-chat-cli` — a TDD-built, production-grade driver for Croft group-chat scenarios.
//!
//! The design is hexagonal: scenario logic drives peers through the [`transport::Transport`]
//! port. The first adapter, [`transport::InProcBus`], delivers frames deterministically in a
//! single process so scenarios replay identically (the regression / conformity bed). A real
//! iroh-gossip adapter will follow behind the same port, so one scenario script runs against
//! either transport.

pub mod effects;
pub mod render;
pub mod runtime;
pub mod transport;

/// The fixed group topic this happy-path slice uses. A shell-side constant —
/// the core's effects are topic-free; the shell supplies the topic when
/// performing them (DECISION 1). A hardcoded placeholder; real topic derivation
/// is a later phase.
pub const GROUP_TOPIC: &str = "croft-group-demo";
