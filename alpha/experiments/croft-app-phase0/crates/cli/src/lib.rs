//! The Phase 0 imperative shell, as a library so its pieces are testable. The
//! `pond` binary (`main.rs`) is a thin wrapper over these modules.

pub mod effects;
pub mod executor;
pub mod render;
pub mod runtime;
