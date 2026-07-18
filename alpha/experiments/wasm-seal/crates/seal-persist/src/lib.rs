//! `seal-persist` — RUN-19 P3: state at rest, resumption, eviction honesty.
//!
//! **Provenance**: the group mechanics are copied from
//! `alpha/Proofs/lineage-groups/crates/lineage-mls` (`Device`, plain-DID
//! shape only) and the frame handling from
//! `alpha/experiments/croft-group/crates/group-seal` (`Sealer`), because both
//! deliberately hide the openmls provider and therefore expose no
//! state-at-rest surface (FND-R19-2, filed as a croft-group backlog gap, not
//! fought here). The one extension is [`PersistSealer::snapshot`] /
//! [`PersistSealer::restore`]: the member's entire MLS state (the provider's
//! storage map + signer + group identity) serialized and sealed as an
//! **AES-128-GCM** blob via the provider's own AEAD, so what rests on disk —
//! or in IndexedDB/OPFS in the browser mapping — is ciphertext under a
//! host-held key.

#![warn(missing_docs)]

use group_core::ChatMessage;
use lineage_core::Did;
use openmls::prelude::{tls_codec::Deserialize as _, *};
use openmls_basic_credential::SignatureKeyPair;
use openmls_rust_crypto::OpenMlsRustCrypto;
use openmls_traits::crypto::OpenMlsCrypto;
use openmls_traits::random::OpenMlsRand;
use openmls_traits::types::AeadType;
use serde::{Deserialize, Serialize};

/// The MTI ciphersuite (the lineage-groups/croft-group pin).
const CIPHERSUITE: Ciphersuite = Ciphersuite::MLS_128_DHKEMX25519_AES128GCM_SHA256_Ed25519;

/// Domain separation for the at-rest AEAD.
const AT_REST_AAD: &[u8] = b"run19-seal-persist-state-v1";
/// AES-GCM nonce length.
const NONCE_LEN: usize = 12;

/// Why a persist-capable seal operation failed.
#[derive(Debug, thiserror::Error)]
pub enum PersistError {
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

fn mls_err<E: std::fmt::Debug>(e: E) -> PersistError {
    PersistError::Mls(format!("{e:?}"))
}

fn blob_err<E: std::fmt::Debug>(e: E) -> PersistError {
    PersistError::Blob(format!("{e:?}"))
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
    fn put(&mut self, blob: &[u8]) -> Result<(), PersistError> {
        std::fs::write(&self.path, blob).map_err(blob_err)
    }
    fn get(&self) -> Result<Option<Vec<u8>>, PersistError> {
        match std::fs::read(&self.path) {
            Ok(bytes) => Ok(Some(bytes)),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(blob_err(e)),
        }
    }
    fn destroy(&mut self) -> Result<(), PersistError> {
        match std::fs::remove_file(&self.path) {
            Ok(()) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(blob_err(e)),
        }
    }
}

/// The serialized (pre-encryption) state: the provider's whole storage map,
/// the signer's public key (the private half lives IN the storage map, where
/// `SignatureKeyPair::store` put it), the DID, and the group id.
#[derive(Serialize, Deserialize)]
struct StateAtRest {
    values: Vec<(Vec<u8>, Vec<u8>)>,
    signer_pub: Vec<u8>,
    did: String,
    group_id: Option<Vec<u8>>,
}

/// A persistence-capable member: the same seal surface as
/// `group_seal::Sealer`, plus [`PersistSealer::snapshot`] /
/// [`PersistSealer::restore`].
pub struct PersistSealer {
    did: Did,
    provider: OpenMlsRustCrypto,
    signer: SignatureKeyPair,
    credential: CredentialWithKey,
    group: Option<MlsGroup>,
}

impl PersistSealer {
    fn new_inner(did: &str) -> Result<Self, PersistError> {
        let provider = OpenMlsRustCrypto::default();
        let signer =
            SignatureKeyPair::new(CIPHERSUITE.signature_algorithm()).map_err(mls_err)?;
        signer.store(provider.storage()).map_err(mls_err)?;
        let did = Did::new(did);
        let credential = BasicCredential::new(did.credential_identity());
        let credential = CredentialWithKey {
            credential: credential.into(),
            signature_key: signer.public().into(),
        };
        Ok(Self {
            did,
            provider,
            signer,
            credential,
            group: None,
        })
    }

    /// Found a fresh sealed group (genesis).
    ///
    /// # Errors
    /// [`PersistError::Mls`] if credential or group creation fails.
    pub fn found(did: &str) -> Result<Self, PersistError> {
        let mut this = Self::new_inner(did)?;
        let group = MlsGroup::builder()
            .ciphersuite(CIPHERSUITE)
            .use_ratchet_tree_extension(true)
            .build(&this.provider, &this.signer, this.credential.clone())
            .map_err(mls_err)?;
        this.group = Some(group);
        Ok(this)
    }

    /// Enroll a prospective member (joins later from a Welcome).
    ///
    /// # Errors
    /// [`PersistError::Mls`] if credential creation fails.
    pub fn enroll(did: &str) -> Result<Self, PersistError> {
        Self::new_inner(did)
    }

    /// This member's key package as wire bytes.
    ///
    /// # Errors
    /// [`PersistError::Mls`] if building or serializing fails.
    pub fn key_package(&self) -> Result<Vec<u8>, PersistError> {
        let bundle = KeyPackage::builder()
            .build(
                CIPHERSUITE,
                &self.provider,
                &self.signer,
                self.credential.clone(),
            )
            .map_err(mls_err)?;
        seal_wire::key_package_to_bytes(bundle.key_package()).map_err(mls_err)
    }

    fn group_ref(&self) -> Result<&MlsGroup, PersistError> {
        self.group
            .as_ref()
            .ok_or_else(|| PersistError::Mls("no active group".into()))
    }

    /// Add one member by wire key package → `(commit, welcome)`.
    ///
    /// # Errors
    /// [`PersistError::Mls`] if validation or the add commit fails.
    pub fn invite(&mut self, kp_bytes: &[u8]) -> Result<(Vec<u8>, Vec<u8>), PersistError> {
        let kp = seal_wire::key_package_from_bytes(kp_bytes).map_err(mls_err)?;
        let provider = &self.provider;
        let signer = &self.signer;
        let group = self.group.as_mut().ok_or_else(|| PersistError::Mls("no active group".into()))?;
        let (commit, welcome, _gi) = group
            .add_members(provider, signer, &[kp])
            .map_err(mls_err)?;
        group.merge_pending_commit(provider).map_err(mls_err)?;
        use openmls::prelude::tls_codec::Serialize as _;
        Ok((
            commit.tls_serialize_detached().map_err(mls_err)?,
            welcome.tls_serialize_detached().map_err(mls_err)?,
        ))
    }

    /// Join a group from a Welcome (ratchet tree rides the extension).
    ///
    /// # Errors
    /// [`PersistError::Mls`] if the Welcome cannot be processed.
    pub fn accept_welcome(&mut self, welcome_bytes: &[u8]) -> Result<(), PersistError> {
        let welcome = match MlsMessageIn::tls_deserialize_exact(welcome_bytes)
            .map_err(mls_err)?
            .extract()
        {
            MlsMessageBodyIn::Welcome(w) => w,
            other => {
                return Err(PersistError::Mls(format!("expected welcome, got {other:?}")))
            }
        };
        let staged = StagedWelcome::new_from_welcome(
            &self.provider,
            &MlsGroupJoinConfig::default(),
            welcome,
            None,
        )
        .map_err(mls_err)?;
        self.group = Some(staged.into_group(&self.provider).map_err(mls_err)?);
        Ok(())
    }

    /// Seal a chat message as MLS application ciphertext.
    ///
    /// # Errors
    /// [`PersistError::Mls`] if encryption fails.
    pub fn seal(&mut self, message: &ChatMessage) -> Result<Vec<u8>, PersistError> {
        let frame = group_core::serialize(message);
        let provider = &self.provider;
        let signer = &self.signer;
        let group = self.group.as_mut().ok_or_else(|| PersistError::Mls("no active group".into()))?;
        let out = group
            .create_message(provider, signer, &frame)
            .map_err(mls_err)?;
        use openmls::prelude::tls_codec::Serialize as _;
        out.tls_serialize_detached().map_err(mls_err)
    }

    fn recv(&mut self, bytes: &[u8]) -> Result<ProcessedMessageContent, PersistError> {
        let protocol_msg: ProtocolMessage = MlsMessageIn::tls_deserialize_exact(bytes)
            .map_err(mls_err)?
            .try_into_protocol_message()
            .map_err(mls_err)?;
        let provider = &self.provider;
        let group = self.group.as_mut().ok_or_else(|| PersistError::Mls("no active group".into()))?;
        let processed = group.process_message(provider, protocol_msg).map_err(mls_err)?;
        Ok(processed.into_content())
    }

    /// Open a sealed frame back to a chat message.
    ///
    /// # Errors
    /// [`PersistError::Mls`] on decryption failure (no-key peer, evicted
    /// epoch); [`PersistError::UnexpectedKind`] for a control message;
    /// [`PersistError::Blob`] never.
    pub fn open(&mut self, sealed: &[u8]) -> Result<ChatMessage, PersistError> {
        match self.recv(sealed)? {
            ProcessedMessageContent::ApplicationMessage(app) => {
                group_core::deserialize(&app.into_bytes())
                    .map_err(|e| PersistError::Mls(format!("frame: {e:?}")))
            }
            _ => Err(PersistError::UnexpectedKind),
        }
    }

    /// Apply an inbound commit, advancing the epoch.
    ///
    /// # Errors
    /// [`PersistError::Mls`] if processing fails;
    /// [`PersistError::UnexpectedKind`] for application data.
    pub fn apply_control(&mut self, control: &[u8]) -> Result<(), PersistError> {
        match self.recv(control)? {
            ProcessedMessageContent::StagedCommitMessage(staged) => {
                let provider = &self.provider;
                let group = self.group.as_mut().ok_or_else(|| PersistError::Mls("no active group".into()))?;
                group.merge_staged_commit(provider, *staged).map_err(mls_err)
            }
            _ => Err(PersistError::UnexpectedKind),
        }
    }

    /// Remove a member by DID (plain-DID leaves only) → the commit.
    ///
    /// # Errors
    /// [`PersistError::NoSuchMember`] if absent; [`PersistError::Mls`] on
    /// commit failure.
    pub fn remove_member(&mut self, did: &str) -> Result<Vec<u8>, PersistError> {
        let want = Did::new(did).credential_identity();
        let target = self
            .group_ref()?
            .members()
            .find_map(|m| {
                BasicCredential::try_from(m.credential.clone())
                    .ok()
                    .filter(|c| c.identity() == want.as_slice())
                    .map(|_| m.index)
            })
            .ok_or_else(|| PersistError::NoSuchMember(did.to_string()))?;
        let provider = &self.provider;
        let signer = &self.signer;
        let group = self.group.as_mut().ok_or_else(|| PersistError::Mls("no active group".into()))?;
        let (commit, _welcome, _gi) = group
            .remove_members(provider, signer, &[target])
            .map_err(mls_err)?;
        group.merge_pending_commit(provider).map_err(mls_err)?;
        use openmls::prelude::tls_codec::Serialize as _;
        commit.tls_serialize_detached().map_err(mls_err)
    }

    /// Current epoch.
    ///
    /// # Errors
    /// [`PersistError::Mls`] if there is no active group.
    pub fn epoch(&self) -> Result<u64, PersistError> {
        Ok(self.group_ref()?.epoch().as_u64())
    }

    /// Exported per-epoch secret bytes (the I4 comparator; test exposure).
    ///
    /// # Errors
    /// [`PersistError::Mls`] if the export fails.
    pub fn epoch_secret(&self) -> Result<Vec<u8>, PersistError> {
        self.group_ref()?
            .export_secret(self.provider.crypto(), "lineage-epoch-proof", b"", 32)
            .map_err(mls_err)
    }

    /// Serialize this member's entire MLS state as an AES-128-GCM
    /// encrypted-at-rest blob under `key`: `nonce(12) || ciphertext`, AAD
    /// domain-separated. What rests is ciphertext only.
    ///
    /// # Errors
    /// [`PersistError::Blob`] on serialization/encryption failure.
    pub fn snapshot(&self, key: &[u8; 16]) -> Result<Vec<u8>, PersistError> {
        let values = self
            .provider
            .storage()
            .values
            .read()
            .map_err(|_| PersistError::Blob("storage lock poisoned".into()))?
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        let state = StateAtRest {
            values,
            signer_pub: self.signer.public().to_vec(),
            did: self.did.0.clone(),
            group_id: self
                .group
                .as_ref()
                .map(|g| g.group_id().as_slice().to_vec()),
        };
        let plain = serde_json::to_vec(&state).map_err(blob_err)?;
        let nonce = self
            .provider
            .rand()
            .random_vec(NONCE_LEN)
            .map_err(blob_err)?;
        let ct = self
            .provider
            .crypto()
            .aead_encrypt(AeadType::Aes128Gcm, key, &plain, &nonce, AT_REST_AAD)
            .map_err(blob_err)?;
        let mut blob = nonce;
        blob.extend_from_slice(&ct);
        Ok(blob)
    }

    /// Rebuild a member from an encrypted blob. The ONLY way back without a
    /// fresh Welcome — and it requires both the blob and the key.
    ///
    /// # Errors
    /// [`PersistError::Blob`] if the blob is missing/corrupt or the key is
    /// wrong (the eviction drill's expected failure).
    pub fn restore(key: &[u8; 16], blob: &[u8]) -> Result<Self, PersistError> {
        if blob.len() <= NONCE_LEN {
            return Err(PersistError::Blob("blob too short".into()));
        }
        let (nonce, ct) = blob.split_at(NONCE_LEN);
        let provider = OpenMlsRustCrypto::default();
        let plain = provider
            .crypto()
            .aead_decrypt(AeadType::Aes128Gcm, key, ct, nonce, AT_REST_AAD)
            .map_err(|_| PersistError::Blob("decryption failed (wrong key or corrupt)".into()))?;
        let state: StateAtRest = serde_json::from_slice(&plain).map_err(blob_err)?;

        {
            let mut values = provider
                .storage()
                .values
                .write()
                .map_err(|_| PersistError::Blob("storage lock poisoned".into()))?;
            *values = state.values.into_iter().collect();
        }
        let signer = SignatureKeyPair::read(
            provider.storage(),
            &state.signer_pub,
            CIPHERSUITE.signature_algorithm(),
        )
        .ok_or_else(|| PersistError::Blob("signer not in restored storage".into()))?;
        let did = Did::new(&state.did);
        let credential = BasicCredential::new(did.credential_identity());
        let credential = CredentialWithKey {
            credential: credential.into(),
            signature_key: signer.public().into(),
        };
        let group = match state.group_id {
            Some(gid) => MlsGroup::load(provider.storage(), &GroupId::from_slice(&gid))
                .map_err(blob_err)?,
            None => None,
        };
        Ok(Self {
            did,
            provider,
            signer,
            credential,
            group,
        })
    }
}
