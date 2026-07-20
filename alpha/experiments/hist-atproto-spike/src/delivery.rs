//! Delivery cursors — structurally unreadable by the fold.
//!
//! The ordering rider (lib.rs; GROUPS.md v2 A.7): firehose `seq`, commit
//! order, and rkey sort are delivery cursors, never order. This module makes
//! the MUST-NOT structural: [`DeliveryCursor`]'s value is private to this
//! module, exposes no accessor, and implements neither `Ord` nor
//! `PartialOrd`, so code outside this module — the fold included — **cannot
//! compile** a read of delivery position. The only consumer-facing surface of
//! a [`Delivered`] item is its envelope.
//!
//! ```compile_fail
//! // The fold (or anyone) trying to read delivery position does not compile:
//! // the cursor's field is private and it has no accessor and no ordering.
//! let d: hist_atproto_spike::delivery::Delivered = unimplemented!();
//! let _ = d.cursor.0;
//! ```
//!
//! ```compile_fail
//! // Neither can delivery items be compared by cursor:
//! fn cmp(a: &hist_atproto_spike::delivery::DeliveryCursor,
//!        b: &hist_atproto_spike::delivery::DeliveryCursor) -> std::cmp::Ordering {
//!     a.cmp(b)
//! }
//! ```

use crate::envelope::Envelope;

/// An opaque delivery position (firehose seq / commit order / page order —
/// any of the three artifacts). Deliberately: private value, no accessor,
/// no `Ord`/`PartialOrd`.
#[derive(Debug, Clone)]
pub struct DeliveryCursor {
    _seq: u64,
}

/// One delivered envelope, stamped with the cursor the transport assigned.
#[derive(Debug, Clone)]
pub struct Delivered {
    pub cursor: DeliveryCursor,
    pub env: Envelope,
}

/// Stamps delivery cursors in arrival order — the mock transport's sequencer
/// (what a firehose or a page iterator would do).
#[derive(Debug, Default)]
pub struct Deliverer {
    next: u64,
}

impl Deliverer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn deliver(&mut self, env: Envelope) -> Delivered {
        let d = Delivered {
            cursor: DeliveryCursor { _seq: self.next },
            env,
        };
        self.next += 1;
        d
    }
}
