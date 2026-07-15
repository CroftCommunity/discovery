//! Group key management layer — openmls (MLS / RFC 9420).
//!
//! MLS manages a rotating per-epoch group key. We do NOT use MLS
//! application-message encryption for stored data. Instead, once the group is
//! established at a given epoch, every member derives a 32-byte **content key**
//! from MLS's *exporter secret* (`export_secret`) with a fixed label. Members in
//! the same group state derive the *same* content key; when membership changes,
//! the epoch advances and the derived key rotates.
//!
//! The single committer for membership changes is assumed (Alice). Concurrent
//! membership commits would require deterministic tiebreak / fork resolution,
//! which is out of scope for this slice.

use openmls::prelude::tls_codec::*;
use openmls::prelude::*;
use openmls_basic_credential::SignatureKeyPair;
use openmls_rust_crypto::OpenMlsRustCrypto;
use openmls_traits::types::SignatureScheme;
use openmls_traits::OpenMlsProvider;

/// Default Ed25519 ciphersuite required by the brief.
pub const CIPHERSUITE: Ciphersuite =
    Ciphersuite::MLS_128_DHKEMX25519_AES128GCM_SHA256_Ed25519;

/// Fixed exporter label + context binding the derived key to this application.
pub const EXPORTER_LABEL: &str = "encrypted-sync-slice/content-key";
pub const EXPORTER_CONTEXT: &[u8] = b"chat/doc";

/// A group member: its own crypto provider, MLS credential, and signer.
pub struct Member {
    /// Human-readable label, retained for diagnostics.
    #[allow(dead_code)]
    pub name: String,
    pub provider: OpenMlsRustCrypto,
    pub credential: CredentialWithKey,
    pub signer: SignatureKeyPair,
}

impl Member {
    pub fn new(name: &str) -> Self {
        let provider = OpenMlsRustCrypto::default();
        let signature_algorithm: SignatureScheme = CIPHERSUITE.signature_algorithm();

        let credential = BasicCredential::new(name.as_bytes().to_vec());
        let signer = SignatureKeyPair::new(signature_algorithm)
            .expect("failed to generate signature keypair");
        signer
            .store(provider.storage())
            .expect("failed to store signature keypair");

        let credential = CredentialWithKey {
            credential: credential.into(),
            signature_key: signer.to_public_vec().into(),
        };

        Self {
            name: name.to_string(),
            provider,
            credential,
            signer,
        }
    }

    /// The member's Ed25519 signature public key. Used directly as the
    /// `subspace` component of the 4-tuple address, so addressing identity and
    /// MLS credential identity are the *same* bytes.
    pub fn identity(&self) -> Vec<u8> {
        self.signer.to_public_vec()
    }

    /// Produce a fresh KeyPackage this member can be invited with.
    pub fn key_package(&self) -> KeyPackageBundle {
        KeyPackage::builder()
            .build(CIPHERSUITE, &self.provider, &self.signer, self.credential.clone())
            .expect("failed to build key package")
    }

    /// Derive this member's 32-byte per-epoch content key from the MLS exporter.
    pub fn content_key(&self, group: &MlsGroup) -> [u8; 32] {
        let secret = group
            .export_secret(self.provider.crypto(), EXPORTER_LABEL, EXPORTER_CONTEXT, 32)
            .expect("failed to export secret");
        let mut key = [0u8; 32];
        key.copy_from_slice(&secret);
        key
    }
}

/// Create a new MLS group owned by `founder` (epoch 0).
pub fn create_group(founder: &Member) -> MlsGroup {
    let config = MlsGroupCreateConfig::builder()
        .ciphersuite(CIPHERSUITE)
        .use_ratchet_tree_extension(true)
        .build();
    MlsGroup::new(
        &founder.provider,
        &founder.signer,
        &config,
        founder.credential.clone(),
    )
    .expect("failed to create MLS group")
}

/// `committer` adds `new_member` to `group`, advancing the epoch, and returns
/// the serialized Welcome bytes the new member uses to join (as they would
/// travel over the wire). Assumes a single committer.
pub fn add_member(
    group: &mut MlsGroup,
    committer: &Member,
    new_member: &Member,
) -> Vec<u8> {
    let key_package = new_member.key_package();
    let (_commit, welcome, _group_info) = group
        .add_members(
            &committer.provider,
            &committer.signer,
            core::slice::from_ref(key_package.key_package()),
        )
        .expect("failed to add member");
    group
        .merge_pending_commit(&committer.provider)
        .expect("failed to merge commit");
    welcome.to_bytes().expect("failed to serialize Welcome")
}

/// `member` joins a group from serialized Welcome bytes.
pub fn join_from_welcome(member: &Member, welcome_bytes: &[u8]) -> MlsGroup {
    let config = MlsGroupJoinConfig::builder()
        .use_ratchet_tree_extension(true)
        .build();
    // Deserialize as an incoming message and extract the Welcome body. Both
    // `into_welcome` helpers are test-utils gated, so we use the ungated
    // `extract()` -> `MlsMessageBodyIn::Welcome` path.
    let message = MlsMessageIn::tls_deserialize_exact(welcome_bytes)
        .expect("failed to deserialize Welcome message");
    let welcome = match message.extract() {
        MlsMessageBodyIn::Welcome(welcome) => welcome,
        _ => panic!("expected the message to be a Welcome"),
    };
    StagedWelcome::new_from_welcome(&member.provider, &config, welcome, None)
        .expect("failed to stage Welcome")
        .into_group(&member.provider)
        .expect("failed to join group from Welcome")
}
