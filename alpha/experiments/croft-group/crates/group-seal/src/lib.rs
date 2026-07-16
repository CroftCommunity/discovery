//! `group-seal` — croft-group **L2a**: the MLS-sealed happy-path frame.
//!
//! The *mechanism* half of croft-group Layer 2 (MLS / encryption). It turns the
//! pure `group-core` plaintext frame into an MLS application-message ciphertext,
//! obtains the read key over a real Welcome, holds the epoch/key state here (a
//! sibling of the pure core, never inside it), and re-keys on a
//! governance-derived membership change — all over **reused** proven crates,
//! never re-implemented: the openmls `Device` wrapper `lineage-mls` (the same
//! primitive `mls-welcome-over-iroh` reuses for Welcome key distribution and
//! `mls-replant`/`replant-continuity` reuse for the re-plant re-key). See
//! `CROFT-GROUP-L2-READINESS.md` §3–4 and backlog §3 L2a.
//!
//! ## Architecture (DECISION 1/4)
//! This crate depends **on** `group-core`; the core never depends on this one,
//! so `group-core` gains no crypto/transport dependency and stays WASM-clean.
//! All key/epoch/exporter material lives here in [`EpochSecret`], a
//! Zeroize-on-drop, no-`Debug` newtype (the `bip39-recovery-roundtrip` pattern),
//! and never enters an `Effect` or a `WireError`.
//!
//! ## Firewall (I9 + the parked resolution-ACL / croft-group L3)
//! L2a covers R1–R7: seal/unseal, key/epoch state, Welcome distribution,
//! membership-half re-key, credential identity, `Zeroize`, and
//! governance-derived membership. It deliberately exposes **no** who-may-revoke
//! knob, **no** co-sign-versus-vote ordering, and **no** recovery-trust-tier
//! selector — those are the *authority* half (R8-tier / R10, gated on I9) and
//! the *projection* half (R9, the parked resolution-ACL). The removal below is
//! the *mechanical* re-key of an already-governed member set, not an authority
//! decision (the `l2a_sealed_frame.rs` firewall guard asserts the wall holds).

#![warn(missing_docs)]

use group_core::ChatMessage;
use lineage_core::ids::Did;
use lineage_mls::{Device, KeyPackage, MlsError, Received};
use zeroize::{Zeroize, ZeroizeOnDrop};

/// A per-epoch exported group secret. Two members in the same epoch of the same
/// group derive equal bytes, which is how "the Welcome distributed the read key"
/// is asserted, so it is `PartialEq` (a constant-time compare is not claimed at
/// this experiment grade). Zeroize-on-drop and deliberately **no** `Debug` — a
/// `Debug` impl would be a way to log the secret (the `bip39-recovery-roundtrip`
/// discipline).
#[derive(Clone, PartialEq, Eq, Zeroize, ZeroizeOnDrop)]
pub struct EpochSecret(Vec<u8>);

impl EpochSecret {
    /// The raw secret bytes, for equality assertions. The secret never leaves
    /// this crate in production; this accessor exists for the L2a tests.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

/// Why a seal / open / membership operation failed.
#[derive(Debug, thiserror::Error)]
pub enum SealError {
    /// The underlying MLS operation failed (wraps `lineage-mls`), reported as a
    /// string so this crate's error surface does not re-export openmls types.
    #[error("mls: {0}")]
    Mls(String),
    /// A frame opened to plaintext that was not a valid `group-core` frame.
    #[error("frame decode: {0}")]
    Frame(String),
    /// A received message was an MLS control message where an application
    /// message was expected, or vice versa.
    #[error("unexpected message kind (application vs control)")]
    UnexpectedKind,
    /// The named member is not in the group (nothing to re-key out).
    #[error("no such member: {0}")]
    NoSuchMember(String),
}

impl From<MlsError> for SealError {
    fn from(e: MlsError) -> Self {
        // Stringified so this crate's error surface does not re-export openmls
        // types (the honesty boundary: L2a wraps the proven crate, not openmls).
        SealError::Mls(e.to_string())
    }
}

/// One member's L2 sealing view: a reused openmls [`Device`] holding the
/// epoch/key state, outside the pure core.
pub struct Sealer {
    device: Device,
}

impl Sealer {
    /// Found a fresh sealed group (genesis) under a DID-bearing credential.
    ///
    /// # Errors
    /// [`SealError::Mls`] if openmls fails to create the credential or group.
    pub fn found(did: &str) -> Result<Self, SealError> {
        let mut device = Device::new(Did::new(did)).map_err(SealError::from)?;
        device.create_group().map_err(SealError::from)?;
        Ok(Self { device })
    }

    /// Enroll a prospective member: a device with a credential but no group yet
    /// (it joins later from a Welcome).
    ///
    /// # Errors
    /// [`SealError::Mls`] if openmls fails to create the credential.
    pub fn enroll(did: &str) -> Result<Self, SealError> {
        let device = Device::new(Did::new(did)).map_err(SealError::from)?;
        Ok(Self { device })
    }

    /// This member's key package, so a group member can add it.
    ///
    /// # Errors
    /// [`SealError::Mls`] if openmls fails to build the key package.
    pub fn key_package(&self) -> Result<KeyPackage, SealError> {
        self.device.key_package().map_err(SealError::from)
    }

    /// Add members by their key packages, returning `(commit, welcome)` — the
    /// commit for existing members, the Welcome for the newcomers. Reuses
    /// `Device::add` (the add/Welcome path `mls-welcome-over-iroh` proves).
    ///
    /// # Errors
    /// [`SealError::Mls`] if the add commit fails.
    pub fn invite(&mut self, key_packages: &[KeyPackage]) -> Result<(Vec<u8>, Vec<u8>), SealError> {
        self.device.add(key_packages).map_err(SealError::from)
    }

    /// Join a group from a Welcome (the newcomer side). The ratchet tree rides
    /// in the Welcome's extension, so no out-of-band tree is needed. Reuses
    /// `Device::join_from_welcome`.
    ///
    /// # Errors
    /// [`SealError::Mls`] if the Welcome cannot be processed.
    pub fn accept_welcome(&mut self, welcome: &[u8]) -> Result<(), SealError> {
        self.device
            .join_from_welcome(welcome, None)
            .map_err(SealError::from)
    }

    /// Seal a chat message as MLS application ciphertext (R1: `SendMessage`
    /// seals). The plaintext is the `group-core` wire frame; the output is the
    /// opaque payload the shell publishes.
    ///
    /// # Errors
    /// [`SealError::Mls`] if encryption fails.
    pub fn seal(&mut self, message: &ChatMessage) -> Result<Vec<u8>, SealError> {
        let frame = group_core::serialize(message);
        self.device.send(&frame).map_err(SealError::from)
    }

    /// Open a sealed frame back to a chat message (R1: `FrameReceived` opens). A
    /// control message (a membership commit) is not a frame — route it to
    /// [`Sealer::apply_control`] instead.
    ///
    /// # Errors
    /// - [`SealError::Mls`] if decryption fails (e.g. a no-key peer, or a frame
    ///   from an epoch this member has been re-keyed out of).
    /// - [`SealError::UnexpectedKind`] if the message was a control message.
    /// - [`SealError::Frame`] if the decrypted bytes are not a valid frame.
    pub fn open(&mut self, sealed: &[u8]) -> Result<ChatMessage, SealError> {
        match self.device.recv(sealed).map_err(SealError::from)? {
            Received::Application(plaintext) => {
                group_core::deserialize(&plaintext).map_err(|e| SealError::Frame(e.to_string()))
            }
            Received::CommitMerged => Err(SealError::UnexpectedKind),
        }
    }

    /// Process an inbound MLS control message (a membership commit), advancing
    /// this member's epoch.
    ///
    /// # Errors
    /// - [`SealError::Mls`] if the commit cannot be processed.
    /// - [`SealError::UnexpectedKind`] if the message was application data.
    pub fn apply_control(&mut self, control: &[u8]) -> Result<(), SealError> {
        match self.device.recv(control).map_err(SealError::from)? {
            Received::CommitMerged => Ok(()),
            Received::Application(_) => Err(SealError::UnexpectedKind),
        }
    }

    /// Re-key an already-governed member out by DID, returning the commit to fan
    /// out (R4/R7). This is the **mechanical** re-plant re-key of the
    /// fold-derived member set — the departed member's identity is the input;
    /// no who-may-revoke authority is decided here (that is R10 / I9, firewalled
    /// out). Reuses `Device::remove`, the primitive
    /// `replant-continuity::e12_7_2_removal_propagates` drives.
    ///
    /// # Errors
    /// - [`SealError::NoSuchMember`] if `did` is not a current member.
    /// - [`SealError::Mls`] if the removal commit fails.
    pub fn remove_member(&mut self, did: &str) -> Result<Vec<u8>, SealError> {
        let target = self
            .device
            .leaf_index_of(&Did::new(did))
            .map_err(SealError::from)?
            .ok_or_else(|| SealError::NoSuchMember(did.to_string()))?;
        let (commit, _welcome) = self.device.remove(&[target]).map_err(SealError::from)?;
        Ok(commit)
    }

    /// This member's exported per-epoch secret, wrapped in the Zeroize newtype.
    ///
    /// # Errors
    /// [`SealError::Mls`] if the export fails.
    pub fn epoch_secret(&self) -> Result<EpochSecret, SealError> {
        Ok(EpochSecret(
            self.device.epoch_proof().map_err(SealError::from)?,
        ))
    }

    /// This member's current epoch number.
    ///
    /// # Errors
    /// [`SealError::Mls`] if there is no active group.
    pub fn epoch(&self) -> Result<u64, SealError> {
        self.device.epoch().map_err(SealError::from)
    }
}
