//! `seal-persist` — RUN-19 P3: state at rest, resumption, eviction honesty.
//!
//! RED stub: the interface exists (the same seal surface as
//! `group-seal::Sealer`, plus `snapshot`/`restore` and the [`BlobStore`]
//! trait); every operation reports [`PersistError::Unbuilt`].

#![warn(missing_docs)]

use group_core::ChatMessage;

/// Why a persist-capable seal operation failed.
#[derive(Debug, thiserror::Error)]
pub enum PersistError {
    /// RED: not yet implemented.
    #[error("unbuilt: P3 red")]
    Unbuilt,
    /// The underlying MLS operation failed.
    #[error("mls: {0}")]
    Mls(String),
    /// The blob could not be decrypted or decoded (wrong key, corrupt, or
    /// destroyed state — the eviction drill's expected failure).
    #[error("blob: {0}")]
    Blob(String),
    /// A received message was the wrong kind (application vs control).
    #[error("unexpected message kind")]
    UnexpectedKind,
    /// The named member is not in the group.
    #[error("no such member: {0}")]
    NoSuchMember(String),
}

/// The at-rest backing for one member's encrypted state blob.
///
/// `SPEC-DELTA[run19-storage-shim | declared-stand-in]`: in this environment
/// the impls are a file (native) and the Node host's filesystem (wasm drill),
/// standing in for the browser's IndexedDB/OPFS with a WebCrypto-wrapped
/// at-rest key — Register: `alpha/experiments/SPEC-DIVERGENCE-REGISTER.md`.
pub trait BlobStore {
    /// Persist the encrypted blob (overwrites).
    ///
    /// # Errors
    /// [`PersistError::Blob`] if the backing cannot be written.
    fn put(&mut self, blob: &[u8]) -> Result<(), PersistError>;
    /// Fetch the encrypted blob, if any state is at rest.
    ///
    /// # Errors
    /// [`PersistError::Blob`] if the backing cannot be read.
    fn get(&self) -> Result<Option<Vec<u8>>, PersistError>;
    /// Destroy the at-rest state entirely (the eviction drill).
    ///
    /// # Errors
    /// [`PersistError::Blob`] if the backing cannot be cleared.
    fn destroy(&mut self) -> Result<(), PersistError>;
}

/// A file-backed [`BlobStore`] (the native in-environment backing).
pub struct FileStore {
    path: std::path::PathBuf,
}

impl FileStore {
    /// A store at `path` (need not exist yet).
    #[must_use]
    pub fn new(path: impl Into<std::path::PathBuf>) -> Self {
        Self { path: path.into() }
    }
}

impl BlobStore for FileStore {
    fn put(&mut self, _blob: &[u8]) -> Result<(), PersistError> {
        Err(PersistError::Unbuilt)
    }
    fn get(&self) -> Result<Option<Vec<u8>>, PersistError> {
        Err(PersistError::Unbuilt)
    }
    fn destroy(&mut self) -> Result<(), PersistError> {
        Err(PersistError::Unbuilt)
    }
}

/// A persistence-capable member: the same seal surface as
/// `group_seal::Sealer`, plus [`PersistSealer::snapshot`] /
/// [`PersistSealer::restore`].
pub struct PersistSealer;

impl PersistSealer {
    /// Found a fresh sealed group (genesis).
    ///
    /// # Errors
    /// [`PersistError::Unbuilt`] in RED.
    pub fn found(_did: &str) -> Result<Self, PersistError> {
        Err(PersistError::Unbuilt)
    }

    /// Enroll a prospective member.
    ///
    /// # Errors
    /// [`PersistError::Unbuilt`] in RED.
    pub fn enroll(_did: &str) -> Result<Self, PersistError> {
        Err(PersistError::Unbuilt)
    }

    /// This member's key package as wire bytes.
    ///
    /// # Errors
    /// [`PersistError::Unbuilt`] in RED.
    pub fn key_package(&self) -> Result<Vec<u8>, PersistError> {
        Err(PersistError::Unbuilt)
    }

    /// Add one member by wire key package → `(commit, welcome)`.
    ///
    /// # Errors
    /// [`PersistError::Unbuilt`] in RED.
    pub fn invite(&mut self, _kp_bytes: &[u8]) -> Result<(Vec<u8>, Vec<u8>), PersistError> {
        Err(PersistError::Unbuilt)
    }

    /// Join a group from a Welcome.
    ///
    /// # Errors
    /// [`PersistError::Unbuilt`] in RED.
    pub fn accept_welcome(&mut self, _welcome: &[u8]) -> Result<(), PersistError> {
        Err(PersistError::Unbuilt)
    }

    /// Seal a chat message.
    ///
    /// # Errors
    /// [`PersistError::Unbuilt`] in RED.
    pub fn seal(&mut self, _message: &ChatMessage) -> Result<Vec<u8>, PersistError> {
        Err(PersistError::Unbuilt)
    }

    /// Open a sealed frame.
    ///
    /// # Errors
    /// [`PersistError::Unbuilt`] in RED.
    pub fn open(&mut self, _sealed: &[u8]) -> Result<ChatMessage, PersistError> {
        Err(PersistError::Unbuilt)
    }

    /// Apply an inbound commit.
    ///
    /// # Errors
    /// [`PersistError::Unbuilt`] in RED.
    pub fn apply_control(&mut self, _control: &[u8]) -> Result<(), PersistError> {
        Err(PersistError::Unbuilt)
    }

    /// Remove a member by DID → the commit.
    ///
    /// # Errors
    /// [`PersistError::Unbuilt`] in RED.
    pub fn remove_member(&mut self, _did: &str) -> Result<Vec<u8>, PersistError> {
        Err(PersistError::Unbuilt)
    }

    /// Current epoch.
    ///
    /// # Errors
    /// [`PersistError::Unbuilt`] in RED.
    pub fn epoch(&self) -> Result<u64, PersistError> {
        Err(PersistError::Unbuilt)
    }

    /// Exported per-epoch secret bytes (test comparator).
    ///
    /// # Errors
    /// [`PersistError::Unbuilt`] in RED.
    pub fn epoch_secret(&self) -> Result<Vec<u8>, PersistError> {
        Err(PersistError::Unbuilt)
    }

    /// Serialize this member's entire MLS state as an AES-128-GCM
    /// encrypted-at-rest blob under `key` (16 bytes).
    ///
    /// # Errors
    /// [`PersistError::Unbuilt`] in RED.
    pub fn snapshot(&self, _key: &[u8; 16]) -> Result<Vec<u8>, PersistError> {
        Err(PersistError::Unbuilt)
    }

    /// Rebuild a member from an encrypted blob. The ONLY way back without a
    /// fresh Welcome — and it requires both the blob and the key.
    ///
    /// # Errors
    /// [`PersistError::Unbuilt`] in RED.
    pub fn restore(_key: &[u8; 16], _blob: &[u8]) -> Result<Self, PersistError> {
        Err(PersistError::Unbuilt)
    }
}
