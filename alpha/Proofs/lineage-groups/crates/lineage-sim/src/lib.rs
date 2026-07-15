//! `lineage-sim` — in-process transport + partition simulator and the
//! scenario/invariant harness (Phase 0).
//!
//! A [`Scenario`] owns a set of named [`Device`]s, a deterministic logical
//! clock and a seeded RNG, and a simple in-memory message bus that can be
//! delivered in FIFO order or deterministically shuffled (to fuzz orderings in
//! Phase 2). Every experiment runs against this harness so results are
//! reproducible from a single seed.

use std::collections::BTreeMap;

use lineage_core::{DetRng, Did, Lamport, LamportClock};
use lineage_mls::{Device, MlsError, Received};

/// Errors surfaced by the harness itself (as distinct from MLS errors, which
/// are wrapped transparently).
#[derive(Debug, thiserror::Error)]
pub enum SimError {
    #[error("duplicate device name: {0}")]
    DuplicateDevice(String),
    #[error("message routed to unknown device: {0}")]
    UnknownDevice(String),
    #[error(transparent)]
    Mls(#[from] MlsError),
}

/// A queued ciphertext addressed to a device, stamped with logical time.
#[derive(Debug, Clone)]
pub struct Envelope {
    pub to: String,
    pub bytes: Vec<u8>,
    pub at: Lamport,
}

/// A reproducible scenario: named devices, a logical clock, a seeded RNG, and
/// an in-process bus.
pub struct Scenario {
    pub clock: LamportClock,
    pub rng: DetRng,
    devices: BTreeMap<String, Device>,
    bus: Vec<Envelope>,
}

impl Scenario {
    pub fn new(seed: u64) -> Self {
        Self {
            clock: LamportClock::new(),
            rng: DetRng::from_seed(seed),
            devices: BTreeMap::new(),
            bus: Vec::new(),
        }
    }

    /// Register a device under `name`, bound to `did`. Rejects a duplicate
    /// `name` rather than silently overwriting live group state.
    pub fn add_device(&mut self, name: &str, did: &str) -> Result<(), SimError> {
        if self.devices.contains_key(name) {
            return Err(SimError::DuplicateDevice(name.to_string()));
        }
        let dev = Device::new(Did::new(did))?;
        self.devices.insert(name.to_string(), dev);
        Ok(())
    }

    /// Accessor for a registered device. Panics on an unknown `name`: these are
    /// compile-time test identifiers, so a miss is a programmer error (the same
    /// contract as indexing). Message *routing* by contrast is fallible — see
    /// [`deliver_all`](Self::deliver_all).
    pub fn device(&self, name: &str) -> &Device {
        self.devices
            .get(name)
            .unwrap_or_else(|| panic!("unknown device: {name}"))
    }

    pub fn device_mut(&mut self, name: &str) -> &mut Device {
        self.devices
            .get_mut(name)
            .unwrap_or_else(|| panic!("unknown device: {name}"))
    }

    /// Enqueue a ciphertext for `to`, advancing the logical clock.
    pub fn enqueue(&mut self, to: &str, bytes: Vec<u8>) {
        let at = self.clock.tick();
        self.bus.push(Envelope { to: to.to_string(), bytes, at });
    }

    /// Deliver every queued message in FIFO (deterministic) order, applying it
    /// to the addressed device. Returns the decrypted application payloads.
    pub fn deliver_all(&mut self) -> Result<Vec<(String, Vec<u8>)>, SimError> {
        self.drain_with_order(false)
    }

    /// Like [`deliver_all`] but shuffles delivery order with the seeded RNG —
    /// used in Phase 2 to fuzz reorderings while staying reproducible.
    pub fn deliver_all_shuffled(&mut self) -> Result<Vec<(String, Vec<u8>)>, SimError> {
        self.drain_with_order(true)
    }

    fn drain_with_order(&mut self, shuffle: bool) -> Result<Vec<(String, Vec<u8>)>, SimError> {
        let mut pending = std::mem::take(&mut self.bus);
        if shuffle {
            self.rng.shuffle(&mut pending);
        }
        let mut out = Vec::new();
        for env in pending {
            self.clock.observe(env.at);
            // Routing target is data, not a compile-time name — fail softly.
            let dev = self
                .devices
                .get_mut(&env.to)
                .ok_or_else(|| SimError::UnknownDevice(env.to.clone()))?;
            if let Received::Application(bytes) = dev.recv(&env.bytes)? {
                out.push((env.to, bytes));
            }
        }
        Ok(out)
    }

    // --- invariant helpers --------------------------------------------------

    /// I4 / I10 helper: assert every named device derives the *same* epoch
    /// proof secret (i.e. they share one live epoch). Panics otherwise.
    pub fn assert_converged(&self, names: &[&str]) {
        let mut iter = names.iter();
        let first = iter.next().expect("need at least one device");
        let secret = self
            .device(first)
            .epoch_proof()
            .expect("first device has no group");
        for name in iter {
            let other = self
                .device(name)
                .epoch_proof()
                .expect("device has no group");
            assert_eq!(
                secret, other,
                "convergence failed: {first} and {name} are in different epochs"
            );
        }
    }

    /// A serializable digest of the *logical* state (clock + per-device epoch
    /// and member count). Two runs of the same scenario with the same seed
    /// must produce an identical digest — this is the Phase 0 reproducibility
    /// assertion (logical layer; see lineage-core::rng honesty boundary).
    pub fn logical_digest(&self) -> String {
        let mut parts = vec![format!("clock={}", self.clock.now().0)];
        for (name, dev) in &self.devices {
            let epoch = dev.epoch().map(|e| e as i64).unwrap_or(-1);
            let members = dev.member_count().map(|m| m as i64).unwrap_or(-1);
            parts.push(format!("{name}:e={epoch}:m={members}"));
        }
        parts.join("|")
    }
}
