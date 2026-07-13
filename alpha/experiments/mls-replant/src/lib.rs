//! The **re-plant** primitive against a real MLS library (openmls 0.8), for Battery 7.
//!
//! The whole fork/heal/re-key story reduces to one operation: read the member set from
//! the governance chain, **stamp a fresh MLS group over it**, atomically repoint the
//! conversation (§7.6.2). This crate implements the stamp against openmls and exposes the
//! measurements and invariants the E12 experiments probe. The governance chain itself is
//! out of scope here (that is Rung B, the local_storage_projection fold); the member set
//! is modelled as a list of personae.

use openmls::prelude::*;
use openmls_basic_credential::SignatureKeyPair;
use openmls_rust_crypto::OpenMlsRustCrypto;
use tls_codec::{Deserialize as _, Serialize as _};

/// Re-exported so tests can hold a `Vec<MlsGroup>` (the per-member copies M1's round-robin drives).
pub use openmls::prelude::MlsGroup;

/// The ciphersuite the spike used.
pub const CS: Ciphersuite = Ciphersuite::MLS_128_DHKEMX25519_AES128GCM_SHA256_Ed25519;

/// A persona with its own crypto provider, signer, and credential — the unit a member set
/// is made of. Each holds its own private state, as separate devices would.
pub struct Persona {
    pub provider: OpenMlsRustCrypto,
    pub signer: SignatureKeyPair,
    pub cwk: CredentialWithKey,
    /// Stable public identity (the signature public key bytes) for membership comparison.
    pub id: Vec<u8>,
}

impl Persona {
    /// A fresh persona named `name`.
    #[must_use]
    pub fn new(name: &str) -> Self {
        let provider = OpenMlsRustCrypto::default();
        let signer = SignatureKeyPair::new(CS.signature_algorithm()).expect("signer");
        signer.store(provider.storage()).expect("store signer");
        let credential = BasicCredential::new(name.as_bytes().to_vec());
        let cwk = CredentialWithKey {
            credential: credential.into(),
            signature_key: signer.public().into(),
        };
        let id = signer.public().to_vec();
        Self { provider, signer, cwk, id }
    }

    /// A fresh `KeyPackage` (public) this persona can be added with.
    #[must_use]
    pub fn key_package(&self) -> KeyPackage {
        KeyPackage::builder()
            .build(CS, &self.provider, &self.signer, self.cwk.clone())
            .expect("key package")
            .key_package()
            .clone()
    }

    /// A **last-resort** `KeyPackage`: marked reusable, so it can seat this persona at a
    /// re-plant boundary even when a fresh package is not fetchable — the availability
    /// backstop E12.6 tests. Reusing it means the persona's leaf key is not refreshed.
    #[must_use]
    pub fn last_resort_key_package(&self) -> KeyPackage {
        KeyPackage::builder()
            .mark_as_last_resort()
            .build(CS, &self.provider, &self.signer, self.cwk.clone())
            .expect("last-resort key package")
            .key_package()
            .clone()
    }
}

/// The result of a stamp: the planter's group plus the sizes the liveness window is tuned
/// against (§11.11 / E12.1).
pub struct Stamp {
    pub group: MlsGroup,
    /// Serialized Commit (the add-all commit that seats the member set). 0 for a lone stamp.
    pub commit_bytes: usize,
    /// Serialized Welcome (carries the encrypted group secrets to every added member). 0 for
    /// a lone stamp.
    pub welcome_bytes: usize,
    /// Number of members seated (planter included).
    pub member_count: usize,
    /// The Welcome to seat the added members (`None` for a lone re-plant).
    pub welcome: Option<Welcome>,
    /// The ratchet tree, needed by joiners.
    pub ratchet_tree: RatchetTreeIn,
}

/// **Stamp a fresh MLS group over a member set**: `planter` creates the group and adds
/// `others` (their `KeyPackage`s) in one commit, then merges. This is the re-plant's core
/// operation — the planter is itself a member of the resulting group, so the membership is
/// exactly `{planter} ∪ others`, regardless of who planted.
///
/// # Panics
/// On any openmls error (crypto, serialization) — these are test-time invariants.
#[must_use]
pub fn stamp(planter: &Persona, others: &[&Persona]) -> Stamp {
    let kps: Vec<KeyPackage> = others.iter().map(|p| p.key_package()).collect();
    stamp_kps(planter, kps)
}

/// Like [`stamp`], but seats the given explicit `KeyPackage`s (e.g. a last-resort package,
/// or a pre-fetched one) rather than freshly generating them — the boundary E12.6 exercises.
///
/// # Panics
/// On any openmls error.
#[must_use]
pub fn stamp_kps(planter: &Persona, kps: Vec<KeyPackage>) -> Stamp {
    let mut group = MlsGroup::new(
        &planter.provider,
        &planter.signer,
        &MlsGroupCreateConfig::default(),
        planter.cwk.clone(),
    )
    .expect("create group");

    let (welcome, commit_bytes, welcome_bytes) = if kps.is_empty() {
        (None, 0, 0)
    } else {
        let (commit, welcome, _gi) =
            group.add_members(&planter.provider, &planter.signer, &kps).expect("add_members");
        group.merge_pending_commit(&planter.provider).expect("merge");
        let commit_bytes = commit.tls_serialize_detached().expect("ser commit").len();
        let welcome_bytes = welcome.tls_serialize_detached().expect("ser welcome").len();
        (Some(extract_welcome(&welcome)), commit_bytes, welcome_bytes)
    };

    Stamp {
        member_count: group.members().count(),
        commit_bytes,
        welcome_bytes,
        welcome,
        ratchet_tree: group.export_ratchet_tree().into(),
        group,
    }
}

/// The membership of a group as a sorted set of stable member identities (signature public
/// keys). Two groups over the same member set have the same membership, whatever their tree
/// bytes — this is what tells a benign byte-divergence (dedup) from a real fork (E12.3).
#[must_use]
pub fn membership(group: &MlsGroup) -> Vec<Vec<u8>> {
    let mut ids: Vec<Vec<u8>> = group.members().map(|m| m.signature_key.as_slice().to_vec()).collect();
    ids.sort();
    ids
}

/// The exported ratchet-tree bytes — the group's *tree shape*, and its content address for
/// the §6 tiebreak. Two independent stamps over the same member set differ here (fresh
/// secrets, leaf order), the byte-nondeterminism E12.3 says must resolve as a dedup.
#[must_use]
pub fn tree_bytes(group: &MlsGroup) -> Vec<u8> {
    group.export_ratchet_tree().tls_serialize_detached().expect("ser tree")
}

/// The per-member leaf signature keys (sorted with the member id), so E12.5 can check that a
/// fresh stamp rotated every leaf.
#[must_use]
pub fn leaf_keys(group: &MlsGroup) -> Vec<(Vec<u8>, Vec<u8>)> {
    let mut v: Vec<(Vec<u8>, Vec<u8>)> = group
        .members()
        .map(|m| (m.signature_key.as_slice().to_vec(), m.encryption_key.as_slice().to_vec()))
        .collect();
    v.sort();
    v
}

/// Remove the members whose identities (signature keys) are in `ids` from `group`,
/// authored by `planter`, and merge. Used to evolve a tree through removals so E12.4 can
/// measure the drift a fresh stamp resets. Removed leaves are blanked, not repacked.
///
/// # Panics
/// On any openmls error.
pub fn remove_by_ids(group: &mut MlsGroup, planter: &Persona, ids: &[Vec<u8>]) {
    let indices: Vec<LeafNodeIndex> = group
        .members()
        .filter(|m| ids.contains(&m.signature_key.as_slice().to_vec()))
        .map(|m| m.index)
        .collect();
    group
        .remove_members(&planter.provider, &planter.signer, &indices)
        .expect("remove_members");
    group.merge_pending_commit(&planter.provider).expect("merge remove");
}

/// The steady-state **per-commit** cost: `member` performs one self-update (rotates its own
/// leaf) on `group` and merges; returns the serialized Commit size. Unlike the O(N) stamp
/// (E12.1), an MLS commit only updates the path from the committer's leaf to the root, so
/// this is O(log N) — the number that says whether hot-N stays comfortable (§11.11 / M1).
///
/// # Panics
/// On any openmls error.
pub fn self_update_commit_bytes(group: &mut MlsGroup, member: &Persona) -> usize {
    let bundle = group
        .self_update(&member.provider, &member.signer, LeafNodeParameters::default())
        .expect("self_update");
    let bytes = bundle.commit().tls_serialize_detached().expect("ser commit").len();
    group.merge_pending_commit(&member.provider).expect("merge self_update");
    bytes
}

/// A member joins the group from a Welcome + ratchet tree (processes the Welcome into its own
/// provider). Needed so members other than the planter can commit, populating the tree (M1).
///
/// # Panics
/// On any openmls error.
#[must_use]
pub fn join(persona: &Persona, welcome: Welcome, ratchet_tree: RatchetTreeIn) -> MlsGroup {
    StagedWelcome::new_from_welcome(
        &persona.provider,
        &MlsGroupJoinConfig::default(),
        welcome,
        Some(ratchet_tree),
    )
    .expect("staged welcome")
    .into_group(&persona.provider)
    .expect("into_group")
}

/// `member` self-updates `group` and merges; returns `(commit_bytes, commit_message)`. The
/// message must be applied to every other member's copy via [`apply_commit`] to keep them in
/// sync (and, over a round-robin, to populate the tree interior).
///
/// # Panics
/// On any openmls error.
pub fn commit(group: &mut MlsGroup, member: &Persona) -> (usize, MlsMessageOut) {
    let bundle = group
        .self_update(&member.provider, &member.signer, LeafNodeParameters::default())
        .expect("self_update");
    let bytes = bundle.commit().tls_serialize_detached().expect("ser commit").len();
    let msg = bundle.commit().clone();
    group.merge_pending_commit(&member.provider).expect("merge");
    (bytes, msg)
}

/// Apply a peer's commit message to `group` (process + merge the staged commit).
///
/// # Panics
/// On any openmls error, or if the message is not a commit.
pub fn apply_commit(group: &mut MlsGroup, member: &Persona, commit_msg: &MlsMessageOut) {
    let protocol = to_protocol(commit_msg);
    let processed = group.process_message(&member.provider, protocol).expect("process_message");
    match processed.into_content() {
        ProcessedMessageContent::StagedCommitMessage(sc) => {
            group.merge_staged_commit(&member.provider, *sc).expect("merge staged commit");
        }
        _ => panic!("expected a staged commit"),
    }
}

fn to_protocol(m: &MlsMessageOut) -> ProtocolMessage {
    let bytes = m.tls_serialize_detached().expect("ser");
    MlsMessageIn::tls_deserialize_exact(&bytes)
        .expect("de")
        .try_into_protocol_message()
        .expect("protocol message")
}

fn extract_welcome(welcome_out: &MlsMessageOut) -> Welcome {
    let bytes = welcome_out.tls_serialize_detached().expect("ser welcome");
    let msg_in = MlsMessageIn::tls_deserialize_exact(&bytes).expect("de welcome");
    match msg_in.extract() {
        MlsMessageBodyIn::Welcome(w) => w,
        _ => panic!("expected a Welcome"),
    }
}
