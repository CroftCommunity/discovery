//! Inbox POST acceptance — the receive side.
//!
//! The shim's own `accept_inbox_post` is a thin adapter that hands the
//! wire bytes to `ap-ambassador::verify::verify_ap_http_signature`
//! UNCHANGED. See `FED-SHIM.md §1 row 7` for the accepted-kinds set
//! (Follow / Undo Follow / Delete only; every other kind returns
//! `NotAShim`).
//!
//! `ap-ambassador::verify` is a dev-dep — kept out of the shim's lib
//! deps so the shim stays usable in worlds that don't want the
//! ambassador (`FED-SHIM.md §5 Kinship`). The end-to-end round-trip
//! test at `tests/shim_ambassador_roundtrip.rs` uses both together.

/// The kinds fed-shim accepts (§1 row 7). Everything else 501s.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AcceptedKind {
    Follow,
    UndoFollow,
    Delete,
}

impl AcceptedKind {
    pub fn from_type_string(s: &str) -> Option<AcceptedKind> {
        match s {
            "Follow" => Some(AcceptedKind::Follow),
            "Undo" => Some(AcceptedKind::UndoFollow),
            "Delete" => Some(AcceptedKind::Delete),
            _ => None,
        }
    }
}

/// The error variants the shim returns from inbox acceptance. Distinct
/// from `ap_ambassador::verify::VerifyError` because a shim caller may
/// not want to pull the ambassador into its dep tree — the shim's
/// error is standalone.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShimAcceptError {
    /// The activity type is not in the shim's accepted set. Named
    /// distinctly from a verify error because it is a shim-scope
    /// rejection (§3 firm non-goal).
    NotAShim(String),
    /// Wire verification failed (signature, digest, key resolution,
    /// malformed activity). The upstream detail lives in the
    /// ap-ambassador verify surface; this shim error is a categorical
    /// wrapper.
    WireVerifyFailed(String),
}

impl std::fmt::Display for ShimAcceptError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShimAcceptError::NotAShim(t) => write!(f, "activity type '{}' is not in the shim's accepted set (see FED-SHIM.md §1 row 7 + §3)", t),
            ShimAcceptError::WireVerifyFailed(m) => write!(f, "wire verify failed: {}", m),
        }
    }
}
impl std::error::Error for ShimAcceptError {}
