//! Device-key delegation (RUN-16 §A.5, P5).
//!
//! An account delegates a device signing key by publishing a
//! [`Record::DeviceAttestation`]. A verifier accepts an envelope signed by the
//! device key iff (a) the delegating account's DID resolves, (b) a live,
//! non-deleted attestation binds the account to that device key, and (c) the
//! envelope signature verifies against the device key. Deleting the attestation
//! on the firehose invalidates the delegation — the cache is driven by the
//! delete EVENT, never by a wall-clock TTL.
//!
//! The [`DidResolver`] is the seam. Live grade resolves a real `did:plc`
//! document over the PLC directory (RUN-14's path); here [`DidKeyResolver`]
//! resolves `did:key` locally behind the SAME interface — genuine resolution
//! for `did:key`, a stand-in only for the `did:plc` account case
//! (`SPEC-DELTA[run17-did-resolver | declared-stand-in]`).

use std::collections::{BTreeMap, BTreeSet};

use ed25519_dalek::VerifyingKey;

use crate::envelope::Envelope;
use crate::identity::verifying_from_did_key;
use crate::records::{self, Record};
use crate::source::SourceEvent;

/// The DID-resolution seam: recover a DID's signing verification key.
///
/// A live implementation fetches the DID document (PLC directory for `did:plc`)
/// and returns its `#atproto` verification key. [`DidKeyResolver`] resolves
/// `did:key` with no network.
pub trait DidResolver {
    /// The signing key for `did`, or `None` if the DID does not resolve.
    fn resolve_key(&self, did: &str) -> Option<VerifyingKey>;
}

/// A local `did:key` resolver (no network) — genuine resolution for `did:key`.
#[derive(Debug, Clone, Copy, Default)]
pub struct DidKeyResolver;

impl DidResolver for DidKeyResolver {
    fn resolve_key(&self, did: &str) -> Option<VerifyingKey> {
        verifying_from_did_key(did).ok()
    }
}

/// Why a device envelope was refused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DelegReject {
    /// The delegating account DID did not resolve.
    AccountUnresolvable,
    /// No live attestation binds this account to the signing device key.
    NoActiveAttestation,
    /// The envelope signature did not verify against the device key.
    BadSignature,
}

/// A delegation verifier holding an event-driven attestation cache. There is no
/// TTL field: the cache changes ONLY when [`apply_event`](Self::apply_event)
/// folds a firehose create/delete.
pub struct DelegationVerifier<R: DidResolver> {
    resolver: R,
    /// Active (account, device) delegations.
    active: BTreeSet<(String, String)>,
    /// Attestation identity → (account, device), to resolve deletes.
    by_id: BTreeMap<String, (String, String)>,
}

impl<R: DidResolver> DelegationVerifier<R> {
    /// A new verifier with an empty cache.
    pub fn new(resolver: R) -> Self {
        Self {
            resolver,
            active: BTreeSet::new(),
            by_id: BTreeMap::new(),
        }
    }

    /// Build a verifier by folding a firehose event stream.
    pub fn from_events(resolver: R, events: &[SourceEvent]) -> Self {
        let mut v = Self::new(resolver);
        for ev in events {
            v.apply_event(ev);
        }
        v
    }

    /// Fold one firehose event into the attestation cache (event-driven).
    pub fn apply_event(&mut self, ev: &SourceEvent) {
        match ev {
            SourceEvent::Put(env) => {
                if env.verify().is_err() {
                    return;
                }
                if let Ok(Record::DeviceAttestation { device_did, .. }) = records::decode(env) {
                    let key = (env.body.author.clone(), device_did.clone());
                    self.active.insert(key.clone());
                    self.by_id.insert(env.identity_hex(), key);
                }
            }
            SourceEvent::Delete { author, target } => {
                if let Some(key) = self.by_id.get(target).cloned() {
                    // Only the delegating account may retract its attestation.
                    if key.0 == *author {
                        self.active.remove(&key);
                    }
                }
            }
        }
    }

    /// Accept `env` (signed by a device key) as authorised for `account_did`.
    ///
    /// # Errors
    /// Returns the specific [`DelegReject`] on refusal.
    pub fn accepts_device_envelope(
        &self,
        env: &Envelope,
        account_did: &str,
    ) -> Result<(), DelegReject> {
        // (a) the delegating account must resolve.
        if self.resolver.resolve_key(account_did).is_none() {
            return Err(DelegReject::AccountUnresolvable);
        }
        // (b) a live attestation must bind the account to the signing device.
        let device_did = env.body.author.clone();
        if !self.active.contains(&(account_did.to_string(), device_did)) {
            return Err(DelegReject::NoActiveAttestation);
        }
        // (c) the envelope must verify against the device key.
        if env.verify().is_err() {
            return Err(DelegReject::BadSignature);
        }
        Ok(())
    }

    /// Borrow the resolver (to exercise the seam directly in tests).
    pub fn resolver(&self) -> &R {
        &self.resolver
    }
}
