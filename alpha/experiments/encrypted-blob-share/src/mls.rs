//! Minimal-but-real MLS layer on top of `openmls` 0.8.
//!
//! Responsibilities:
//!   * create Ed25519 BasicCredential members,
//!   * run the standard add-proposal -> commit -> Welcome join flow,
//!   * advance epochs on membership change,
//!   * derive a per-epoch 32-byte *content key* via the MLS exporter.
//!
//! The exporter key — NOT MLS application-message encryption — is what encrypts
//! the blob. Every member that shares group state derives byte-identical key
//! material from `export_secret` with the same label/context.

use anyhow::Context;
use openmls::prelude::*;
use openmls_basic_credential::SignatureKeyPair;
use openmls_rust_crypto::OpenMlsRustCrypto;
use openmls_traits::OpenMlsProvider;
// The TLS codec traits provide `tls_serialize_detached` / `tls_deserialize_exact`.
use openmls::prelude::tls_codec::{Deserialize as _, Serialize as _};

/// The ciphersuite mandated by the brief.
pub const CIPHERSUITE: Ciphersuite =
    Ciphersuite::MLS_128_DHKEMX25519_AES128GCM_SHA256_Ed25519;

/// Exporter label/context used to derive the blob content key. Both must match
/// on every member for the derived keys to agree.
pub const EXPORTER_LABEL: &str = "encrypted-blob-share/content-key";
pub const EXPORTER_CONTEXT: &[u8] = b"attachments-v1";

/// A single group member: its crypto provider, signer, and credential.
pub struct Member {
    pub name: String,
    pub provider: OpenMlsRustCrypto,
    pub signer: SignatureKeyPair,
    pub credential: CredentialWithKey,
}

impl Member {
    /// Create a fresh member with an Ed25519 BasicCredential.
    pub fn new(name: &str) -> anyhow::Result<Self> {
        let provider = OpenMlsRustCrypto::default();
        let signer = SignatureKeyPair::new(CIPHERSUITE.signature_algorithm())
            .context("generate signature key pair")?;
        signer
            .store(provider.storage())
            .map_err(|e| anyhow::anyhow!("store signer: {e:?}"))?;
        let credential = BasicCredential::new(name.as_bytes().to_vec());
        let credential = CredentialWithKey {
            credential: credential.into(),
            signature_key: signer.to_public_vec().into(),
        };
        Ok(Self {
            name: name.to_string(),
            provider,
            signer,
            credential,
        })
    }

    /// Publish a fresh KeyPackage so another member can add us to a group.
    pub fn key_package(&self) -> anyhow::Result<KeyPackage> {
        let bundle = KeyPackage::builder()
            .build(CIPHERSUITE, &self.provider, &self.signer, self.credential.clone())
            .context("build key package")?;
        Ok(bundle.key_package().clone())
    }

    /// Derive the 32-byte per-epoch content key for a group we belong to.
    pub fn content_key(&self, group: &MlsGroup) -> anyhow::Result<[u8; 32]> {
        let secret = group
            .export_secret(self.provider.crypto(), EXPORTER_LABEL, EXPORTER_CONTEXT, 32)
            .map_err(|e| anyhow::anyhow!("export_secret: {e:?}"))?;
        let mut key = [0u8; 32];
        key.copy_from_slice(&secret);
        Ok(key)
    }
}

/// Create a new single-member group owned by `owner`.
pub fn create_group(owner: &Member) -> anyhow::Result<MlsGroup> {
    let config = MlsGroupCreateConfig::builder()
        .ciphersuite(CIPHERSUITE)
        // Carry the ratchet tree inside the Welcome's GroupInfo so a joiner can
        // build its tree without an out-of-band tree delivery.
        .use_ratchet_tree_extension(true)
        .build();
    let group = MlsGroup::new(
        &owner.provider,
        &owner.signer,
        &config,
        owner.credential.clone(),
    )
    .context("create MLS group")?;
    Ok(group)
}

/// Result of adding a member: the serialized commit (for existing members to
/// process) and the serialized Welcome (for the new member to join from).
pub struct AddArtifacts {
    pub commit: Vec<u8>,
    pub welcome: Vec<u8>,
}

/// `adder` adds `new_member` to `group`, merging the commit locally so the
/// adder advances to the new epoch. Returns the wire-serialized commit/welcome.
pub fn add_member(
    group: &mut MlsGroup,
    adder: &Member,
    new_member_kp: KeyPackage,
) -> anyhow::Result<AddArtifacts> {
    let (commit_out, welcome_out, _group_info) = group
        .add_members(&adder.provider, &adder.signer, &[new_member_kp])
        .context("add_members")?;
    group
        .merge_pending_commit(&adder.provider)
        .context("merge pending commit (adder)")?;

    // Production path: serialize to the wire as MlsMessageOut, the receiver
    // deserializes as MlsMessageIn. We avoid the test-only `into_welcome`
    // helpers by going through the codec.
    let commit = commit_out
        .tls_serialize_detached()
        .context("serialize commit")?;
    let welcome = welcome_out
        .tls_serialize_detached()
        .context("serialize welcome")?;
    Ok(AddArtifacts { commit, welcome })
}

/// A member that already belongs to the group processes a commit produced by
/// someone else, advancing to the new epoch.
pub fn process_commit(group: &mut MlsGroup, member: &Member, commit: &[u8]) -> anyhow::Result<()> {
    let msg_in = MlsMessageIn::tls_deserialize_exact(commit).context("deserialize commit")?;
    let protocol_msg = msg_in
        .try_into_protocol_message()
        .context("commit was not a protocol message")?;
    let processed = group
        .process_message(&member.provider, protocol_msg)
        .map_err(|e| anyhow::anyhow!("process_message: {e:?}"))?;
    match processed.into_content() {
        ProcessedMessageContent::StagedCommitMessage(staged) => {
            group
                .merge_staged_commit(&member.provider, *staged)
                .context("merge staged commit")?;
            Ok(())
        }
        _ => anyhow::bail!("expected a staged commit message"),
    }
}

/// `joiner` joins a group from a serialized Welcome message.
pub fn join_from_welcome(joiner: &Member, welcome: &[u8]) -> anyhow::Result<MlsGroup> {
    let msg_in = MlsMessageIn::tls_deserialize_exact(welcome).context("deserialize welcome")?;
    // `into_welcome()` is `#[cfg(test-utils)]`-gated, so use the production path:
    // extract the message body and match the Welcome variant.
    let welcome = match msg_in.extract() {
        MlsMessageBodyIn::Welcome(w) => w,
        _ => anyhow::bail!("message was not a Welcome"),
    };
    let config = MlsGroupJoinConfig::builder().build();
    let staged = StagedWelcome::new_from_welcome(&joiner.provider, &config, welcome, None)
        .context("stage welcome")?;
    let group = staged
        .into_group(&joiner.provider)
        .context("join group from welcome")?;
    Ok(group)
}
