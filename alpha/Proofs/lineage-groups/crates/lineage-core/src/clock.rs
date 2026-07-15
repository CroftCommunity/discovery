//! Deterministic logical clock.
//!
//! Phase 0 requires that every scenario be reproducible. We never call
//! `SystemTime::now()` in logic crates; instead time is a Lamport clock that
//! is ticked explicitly and merged on message receipt. This gives a total
//! order that is identical across runs for the same scripted scenario.

use serde::{Deserialize, Serialize};

/// A Lamport timestamp. Monotonic per-clock; comparable across clocks only as
/// a partial order (ties broken elsewhere, e.g. by device id).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Lamport(pub u64);

/// A per-device Lamport clock.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LamportClock {
    counter: u64,
}

impl LamportClock {
    pub fn new() -> Self {
        Self { counter: 0 }
    }

    /// Advance for a local event and return the new timestamp.
    ///
    /// Panics on `u64` overflow rather than wrapping: a wrapped counter would
    /// silently violate the monotonic-ordering guarantee the clock exists to
    /// provide. Overflow is unreachable in practice (u64 events).
    pub fn tick(&mut self) -> Lamport {
        self.counter = self
            .counter
            .checked_add(1)
            .expect("Lamport clock overflow in tick()");
        Lamport(self.counter)
    }

    /// Merge a remote timestamp (on receive): clock = max(local, remote) + 1.
    ///
    /// Panics on overflow, for the same reason as [`tick`](Self::tick).
    pub fn observe(&mut self, remote: Lamport) -> Lamport {
        self.counter = self
            .counter
            .max(remote.0)
            .checked_add(1)
            .expect("Lamport clock overflow in observe()");
        Lamport(self.counter)
    }

    /// Current value without advancing.
    pub fn now(&self) -> Lamport {
        Lamport(self.counter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tick_is_monotonic() {
        let mut c = LamportClock::new();
        assert_eq!(c.tick(), Lamport(1));
        assert_eq!(c.tick(), Lamport(2));
        assert_eq!(c.now(), Lamport(2));
    }

    #[test]
    fn observe_takes_max_plus_one() {
        let mut c = LamportClock::new();
        c.tick(); // 1
        assert_eq!(c.observe(Lamport(5)), Lamport(6));
        assert_eq!(c.observe(Lamport(2)), Lamport(7));
    }
}
