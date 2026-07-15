//! `croft-chat` — the TUI shell that demonstrates the Drystone protocol.
//!
//! The shell owns the ports (`Transport`, storage path, identity) and drives
//! the `group-chat-core` tenant against the `social-graph-core` substrate. This
//! library exposes the pieces the binary and the integration tests share; the
//! binary (`main.rs`) is a thin entry point over it.
//!
//! Phases populate this crate: P6 (transport port + shared-dir adapter),
//! P7 (convergence proof + fingerprint helper), P10/P11/P15 (TUI),
//! P12 (persistence), P16 (iroh adapter), P17 (topology config).
#![warn(missing_docs)]

pub mod anti_entropy;
pub mod app;
pub mod config;
pub mod fingerprint;
#[cfg(feature = "iroh-it")]
pub mod iroh_bus;
pub mod input;
pub mod shared_dir;
pub mod sync;
pub mod transport;
pub mod ui;

use tracing_subscriber::EnvFilter;

/// Install a process-global `tracing` subscriber driven by `RUST_LOG`.
///
/// Idempotent across the process: a second call is a no-op (the global
/// subscriber can only be set once), which lets both `main` and test harnesses
/// call it freely. Defaults to `info` when `RUST_LOG` is unset.
pub fn init_tracing() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    // `try_init` returns Err if a global subscriber is already installed; that
    // is the idempotent case we deliberately ignore.
    let _ = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(std::io::stderr)
        .try_init();
}

#[cfg(test)]
mod tests {
    use super::init_tracing;

    #[test]
    fn init_tracing_is_idempotent() {
        // The global subscriber can only be installed once; the helper must
        // tolerate repeated calls (main + test harnesses both call it) without
        // panicking. A second call hitting the already-installed path is the
        // case being pinned.
        init_tracing();
        init_tracing();
    }
}
