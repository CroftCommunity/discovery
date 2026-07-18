//! The seal interface — RED stub.
//!
//! Mirrors the croft-group `group-seal::Sealer` surface exactly (found /
//! enroll / key_package / invite / accept_welcome / seal / open /
//! apply_control / remove_member / epoch / epoch_secret), so the P1 test
//! source is identical across RED and GREEN; only this module's wiring
//! changes. Every operation currently reports [`SealError::Unbuilt`].

/// A chat message — the `group-core` frame shape (sender + text). In GREEN
/// this becomes a re-export of `group_core::ChatMessage`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatMessage {
    /// Who sent the message.
    pub sender: String,
    /// The message body.
    pub text: String,
}

/// Build a [`ChatMessage`] (test convenience, stable across RED/GREEN).
#[must_use]
pub fn msg(sender: &str, text: &str) -> ChatMessage {
    ChatMessage {
        sender: sender.to_string(),
        text: text.to_string(),
    }
}

/// A per-epoch exported group secret (equality is the "same group state"
/// assertion). GREEN re-exports `group_seal::EpochSecret`.
#[derive(Clone, PartialEq, Eq)]
pub struct EpochSecret(Vec<u8>);

impl EpochSecret {
    /// The raw secret bytes, for equality/length assertions in tests.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

/// A key package handle (GREEN: `lineage_mls::KeyPackage`).
pub struct KeyPackage;

/// Why a seal operation failed.
#[derive(Debug, thiserror::Error)]
pub enum SealError {
    /// RED: the MLS stack is not yet wired into the wasm module.
    #[error("unbuilt: P1 red — the MLS stack is not yet in the wasm module")]
    Unbuilt,
}

/// The ciphersuite the stack seals under, as a stable name for the P1
/// prediction pin (PRED-CS). GREEN formats `lineage_mls::CIPHERSUITE`.
#[must_use]
pub fn ciphersuite_name() -> String {
    String::new()
}

/// One member's sealing view (GREEN: `group_seal::Sealer`).
pub struct Sealer;

impl Sealer {
    /// Found a fresh sealed group (genesis) under a DID-bearing credential.
    ///
    /// # Errors
    /// [`SealError::Unbuilt`] in RED.
    pub fn found(_did: &str) -> Result<Self, SealError> {
        Err(SealError::Unbuilt)
    }

    /// Enroll a prospective member (credential, no group yet).
    ///
    /// # Errors
    /// [`SealError::Unbuilt`] in RED.
    pub fn enroll(_did: &str) -> Result<Self, SealError> {
        Err(SealError::Unbuilt)
    }

    /// This member's key package, so a group member can add it.
    ///
    /// # Errors
    /// [`SealError::Unbuilt`] in RED.
    pub fn key_package(&self) -> Result<KeyPackage, SealError> {
        Err(SealError::Unbuilt)
    }

    /// Add members by key package → `(commit, welcome)` bytes.
    ///
    /// # Errors
    /// [`SealError::Unbuilt`] in RED.
    pub fn invite(&mut self, _kps: &[KeyPackage]) -> Result<(Vec<u8>, Vec<u8>), SealError> {
        Err(SealError::Unbuilt)
    }

    /// Join a group from a Welcome (newcomer side).
    ///
    /// # Errors
    /// [`SealError::Unbuilt`] in RED.
    pub fn accept_welcome(&mut self, _welcome: &[u8]) -> Result<(), SealError> {
        Err(SealError::Unbuilt)
    }

    /// Seal a chat message as MLS application ciphertext.
    ///
    /// # Errors
    /// [`SealError::Unbuilt`] in RED.
    pub fn seal(&mut self, _message: &ChatMessage) -> Result<Vec<u8>, SealError> {
        Err(SealError::Unbuilt)
    }

    /// Open a sealed frame back to a chat message.
    ///
    /// # Errors
    /// [`SealError::Unbuilt`] in RED.
    pub fn open(&mut self, _sealed: &[u8]) -> Result<ChatMessage, SealError> {
        Err(SealError::Unbuilt)
    }

    /// Process an inbound MLS control message (a commit), advancing the epoch.
    ///
    /// # Errors
    /// [`SealError::Unbuilt`] in RED.
    pub fn apply_control(&mut self, _control: &[u8]) -> Result<(), SealError> {
        Err(SealError::Unbuilt)
    }

    /// Re-key an already-governed member out by DID → the commit to fan out.
    ///
    /// # Errors
    /// [`SealError::Unbuilt`] in RED.
    pub fn remove_member(&mut self, _did: &str) -> Result<Vec<u8>, SealError> {
        Err(SealError::Unbuilt)
    }

    /// This member's exported per-epoch secret.
    ///
    /// # Errors
    /// [`SealError::Unbuilt`] in RED.
    pub fn epoch_secret(&self) -> Result<EpochSecret, SealError> {
        Err(SealError::Unbuilt)
    }

    /// This member's current epoch number.
    ///
    /// # Errors
    /// [`SealError::Unbuilt`] in RED.
    pub fn epoch(&self) -> Result<u64, SealError> {
        Err(SealError::Unbuilt)
    }
}
