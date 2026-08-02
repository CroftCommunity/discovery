//! Phase 3 core: tier claim -> per-connection rate bucket.
//!
//! One credential, one primitive, three product dials. The `tier` claim carried
//! in the token becomes an enforcement decision at admission: what rate bucket
//! this connection's relayed bytes are metered against.
//!
//!   - **Coordination** — admitted, but metered hard enough that holepunch
//!     coordination (a small burst of disco frames) succeeds while *sustained
//!     relayed media is starved*. Content-based splitting is impossible by
//!     design: the relay cannot tell a disco frame from app data (it is all
//!     encrypted), so volume is the only honest lever. See ADR-0004.
//!   - **Broker** — generous or absent limits; the relay will carry media.
//!
//! `bucket_for` is the whole mapping. The embedding layer applies the returned
//! bucket to the connection (Phase 3 integration, deferred here — see the
//! experiment README for why the live relay could not be stood up in this
//! environment).
//!
//! ## Calibration is NOT done here
//!
//! The coordination numbers below are a PLACEHOLDER, not a measured value. The
//! plan's Phase 3 requires the bucket be sized from an instrumented holepunch
//! exchange (measure bytes of a successful coordination, set the bucket with
//! headroom above that and far below usable media bitrate). That measurement
//! needs the live relay + two endpoints, which this environment cannot run
//! (github clone blocked; no multi-process holepunch harness). The constant is
//! therefore marked `SPEC-DELTA` and must be re-derived before any deployment.

use serde::{Deserialize, Serialize};

/// Admission tier, as it appears in the token `tier` claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Tier {
    /// Holepunch-coordination only; relayed media starved.
    Coordination,
    /// Full broker; relayed media permitted.
    Broker,
}

/// A per-connection receive-rate bucket, mirroring iroh-relay's
/// `[limits.client.rx]` shape (`bytes_per_second`, `max_burst_bytes`). `None`
/// on a field means "unset" -> fall through to the relay's global default /
/// unlimited, which is what the broker tier wants.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RateBucket {
    pub bytes_per_second: Option<u64>,
    pub max_burst_bytes: Option<u64>,
}

impl RateBucket {
    /// An explicitly unlimited bucket (broker tier).
    pub const UNLIMITED: RateBucket = RateBucket {
        bytes_per_second: None,
        max_burst_bytes: None,
    };
}

// SPEC-DELTA(phase-3-calibration): placeholder coordination-bucket sizes. NOT
// measured. Must be re-derived from an instrumented holepunch exchange before
// deployment (headroom above measured coordination bytes, far below media
// bitrate). Tracked in EXPERIMENT-BACKLOG + ADR-0004.
//
// Rationale for the placeholder magnitudes: a holepunch disco exchange is a
// handful of small frames over a second or two; a few KiB/s with a small burst
// allowance clears that comfortably while sitting one to two orders of
// magnitude below any usable audio/video bitrate (tens of KB/s and up).
const COORDINATION_BYTES_PER_SECOND: u64 = 4 * 1024;
const COORDINATION_MAX_BURST_BYTES: u64 = 16 * 1024;

/// Map a tier to the per-connection bucket the embedding layer will enforce.
pub fn bucket_for(tier: Tier) -> RateBucket {
    match tier {
        Tier::Coordination => RateBucket {
            bytes_per_second: Some(COORDINATION_BYTES_PER_SECOND),
            max_burst_bytes: Some(COORDINATION_MAX_BURST_BYTES),
        },
        Tier::Broker => RateBucket::UNLIMITED,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Pins the exact placeholder magnitudes. This is the regression guard the
    // Phase-3 calibration will edit: when the measured numbers replace the
    // placeholders, update this test alongside the constants. Until then it
    // keeps the arithmetic honest (a nonzero, exact cap).
    #[test]
    fn coordination_bucket_has_the_calibrated_placeholder_values() {
        let b = bucket_for(Tier::Coordination);
        assert_eq!(b.bytes_per_second, Some(4 * 1024));
        assert_eq!(b.max_burst_bytes, Some(16 * 1024));
    }
}
