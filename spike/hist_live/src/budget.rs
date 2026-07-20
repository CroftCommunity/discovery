//! Gentleness contract: write budget, blob budget, single-flight pacing.
//!
//! Budget is a counter, not a warning — exceeding a cap is a run-failure
//! surfaced as an `Err(BudgetExceeded)` from the leg. Pacing serializes calls
//! at ≤1 rps sustained; it does NOT sleep on the caller (tests can override to
//! zero-delay to keep CI fast).

use std::sync::{Mutex, MutexGuard};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy)]
pub struct BudgetCaps {
    /// Total record writes (creates + updates + deletes count as writes for
    /// the applyWrites contract; retries count too).
    pub writes: usize,
    /// Blob uploads (each MUST be ≤64 KB per the brief).
    pub blobs: usize,
    /// Max blob body length in bytes.
    pub max_blob_bytes: usize,
    /// Reads are metered but not budget-capped; the field exists so the ledger
    /// is complete for the results doc.
    pub reads_cap: Option<usize>,
}

impl BudgetCaps {
    pub const GENTLE: BudgetCaps = BudgetCaps {
        writes: 100,
        blobs: 3,
        max_blob_bytes: 64 * 1024,
        reads_cap: None,
    };
    pub const UNCAPPED: BudgetCaps = BudgetCaps {
        writes: usize::MAX,
        blobs: usize::MAX,
        max_blob_bytes: usize::MAX,
        reads_cap: None,
    };
}

#[derive(Debug, Default, Clone, Copy)]
pub struct BudgetLedger {
    pub writes: usize,
    pub reads: usize,
    pub blobs: usize,
    pub write_calls: usize,
    pub read_calls: usize,
    pub blob_calls: usize,
    pub rate_limit_signals: usize,
}

impl BudgetLedger {
    pub fn is_within(&self, caps: &BudgetCaps) -> bool {
        self.writes <= caps.writes && self.blobs <= caps.blobs
    }
}

#[derive(Debug)]
pub enum BudgetError {
    WritesExceeded { requested: usize, cap: usize },
    BlobsExceeded { requested: usize, cap: usize },
    BlobTooLarge { size: usize, cap: usize },
}

impl std::fmt::Display for BudgetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BudgetError::WritesExceeded { requested, cap } => {
                write!(f, "budget: writes exceeded ({} > {})", requested, cap)
            }
            BudgetError::BlobsExceeded { requested, cap } => {
                write!(f, "budget: blobs exceeded ({} > {})", requested, cap)
            }
            BudgetError::BlobTooLarge { size, cap } => {
                write!(f, "budget: blob too large ({} > {})", size, cap)
            }
        }
    }
}

impl std::error::Error for BudgetError {}

pub struct Budget {
    caps: BudgetCaps,
    ledger: Mutex<BudgetLedger>,
}

impl std::fmt::Debug for Budget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Budget")
            .field("caps", &self.caps)
            .field("ledger", &*self.ledger.lock().unwrap())
            .finish()
    }
}

impl Budget {
    pub fn new(caps: BudgetCaps) -> Self {
        Budget {
            caps,
            ledger: Mutex::new(BudgetLedger::default()),
        }
    }

    pub fn caps(&self) -> BudgetCaps {
        self.caps
    }

    pub fn snapshot(&self) -> BudgetLedger {
        *self.ledger.lock().unwrap()
    }

    /// Charge `n` writes and increment the write-call counter by one.  Returns
    /// `Err(BudgetError)` if adding `n` would cross the cap; the counter is
    /// only advanced on success.
    pub fn charge_writes(&self, n: usize) -> Result<(), BudgetError> {
        let mut l = self.ledger.lock().unwrap();
        let would = l.writes + n;
        if would > self.caps.writes {
            return Err(BudgetError::WritesExceeded {
                requested: would,
                cap: self.caps.writes,
            });
        }
        l.writes = would;
        l.write_calls += 1;
        Ok(())
    }

    pub fn charge_blob(&self, size: usize) -> Result<(), BudgetError> {
        if size > self.caps.max_blob_bytes {
            return Err(BudgetError::BlobTooLarge {
                size,
                cap: self.caps.max_blob_bytes,
            });
        }
        let mut l = self.ledger.lock().unwrap();
        let would = l.blobs + 1;
        if would > self.caps.blobs {
            return Err(BudgetError::BlobsExceeded {
                requested: would,
                cap: self.caps.blobs,
            });
        }
        l.blobs = would;
        l.blob_calls += 1;
        Ok(())
    }

    pub fn charge_read(&self) {
        let mut l = self.ledger.lock().unwrap();
        l.reads += 1;
        l.read_calls += 1;
    }

    pub fn note_rate_limit_signal(&self) {
        self.ledger.lock().unwrap().rate_limit_signals += 1;
    }

    pub fn assert_within(&self) -> Result<(), BudgetError> {
        let l = self.snapshot();
        if l.writes > self.caps.writes {
            Err(BudgetError::WritesExceeded {
                requested: l.writes,
                cap: self.caps.writes,
            })
        } else if l.blobs > self.caps.blobs {
            Err(BudgetError::BlobsExceeded {
                requested: l.blobs,
                cap: self.caps.blobs,
            })
        } else {
            Ok(())
        }
    }
}

/// Single-flight pacer: at most one caller in flight, and consecutive calls
/// are separated by ≥ `min_interval`.
pub struct Pacer {
    min_interval: Duration,
    state: Mutex<PacerState>,
}

impl std::fmt::Debug for Pacer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Pacer")
            .field("min_interval", &self.min_interval)
            .finish()
    }
}

struct PacerState {
    last_release: Option<Instant>,
    in_flight: bool,
}

impl Pacer {
    /// 1 request/second sustained.
    pub fn one_rps() -> Self {
        Pacer::new(Duration::from_millis(1000))
    }

    /// Zero-delay — for tests.
    pub fn zero() -> Self {
        Pacer::new(Duration::from_millis(0))
    }

    pub fn new(min_interval: Duration) -> Self {
        Pacer {
            min_interval,
            state: Mutex::new(PacerState {
                last_release: None,
                in_flight: false,
            }),
        }
    }

    pub fn min_interval(&self) -> Duration {
        self.min_interval
    }

    /// Acquire the pacer, sleeping until the next slot is available.  Returns
    /// a guard that releases on drop and stamps the release time.
    pub fn acquire(&self) -> PacerGuard<'_> {
        // Wait our turn (single-flight).
        loop {
            let mut s = self.state.lock().unwrap();
            if !s.in_flight {
                let wait = match s.last_release {
                    None => Duration::from_millis(0),
                    Some(t) => {
                        let elapsed = t.elapsed();
                        if elapsed >= self.min_interval {
                            Duration::from_millis(0)
                        } else {
                            self.min_interval - elapsed
                        }
                    }
                };
                s.in_flight = true;
                drop(s);
                if !wait.is_zero() {
                    std::thread::sleep(wait);
                }
                return PacerGuard { pacer: self };
            }
            drop(s);
            std::thread::sleep(Duration::from_millis(2));
        }
    }

    pub(crate) fn release(&self) {
        let mut s = self.state.lock().unwrap();
        s.in_flight = false;
        s.last_release = Some(Instant::now());
    }
}

pub struct PacerGuard<'a> {
    pacer: &'a Pacer,
}

impl<'a> Drop for PacerGuard<'a> {
    fn drop(&mut self) {
        self.pacer.release();
    }
}

// Assert single-flight from tests without racing on `in_flight`.
impl Pacer {
    pub fn debug_in_flight(&self) -> bool {
        self.state.lock().unwrap().in_flight
    }
    fn _guard(&self) -> MutexGuard<'_, PacerState> {
        self.state.lock().unwrap()
    }
}
