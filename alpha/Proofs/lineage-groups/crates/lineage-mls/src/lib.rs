//! `lineage-mls` — a thin wrapper over [`openmls`] (RFC 9420).
//!
//! Per the plan (§2) this crate isolates *every* openmls assumption so that a
//! wrong assumption fails one crate rather than the whole thesis. It exposes
//! exactly the primitives the reconnect model depends on:
//!
//! * create a group bound to a DID-bearing credential,
//! * add / remove members with an epoch rekey,
//! * **external-commit join** — the "re-key the other side into the survivor
//!   epoch" primitive (Phase 1 gate, E1.2),
//! * **fresh genesis** — mint a brand-new group and re-add everyone (the
//!   "mint a third" path, E1.3),
//! * export the epoch exporter secret so two views can be proven identical.
//!
//! Messages always cross a real `tls_codec` serialize/deserialize boundary, so
//! these wrappers double as proof that the artifacts survive the wire — which
//! is what Phase 3's real transport will carry.
//!
//! ## Honesty boundary (recorded as a Phase 1 finding)
//! openmls 0.8.1 exposes the `ReInitProposal` primitive but **no** high-level
//! group method that enacts a reinit while binding the new group to the old as
//! continuation. The thesis' "reinit / fresh-genesis" is therefore implemented
//! here as *new group + re-add members* ([`Device::fresh_genesis`]); the
//! lineage binding ("both prior logs inherited as read-only ancestry") lives in
//! the Phase 2 governance layer, not inside MLS. We compose MLS with that
//! layer; we do not extend MLS.

use lineage_core::keys::{Sig, SigningIdentity, VerifyingIdentity};
use lineage_core::Did;
use openmls::prelude::{tls_codec::*, *};
use openmls::treesync::RatchetTree;
use openmls_basic_credential::SignatureKeyPair;
use openmls_rust_crypto::OpenMlsRustCrypto;

/// The ciphersuite used throughout the experiment. X25519 + AES-128-GCM +
/// SHA-256 + Ed25519 — the mandatory-to-implement suite, broadly supported.
pub const CIPHERSUITE: Ciphersuite =
    Ciphersuite::MLS_128_DHKEMX25519_AES128GCM_SHA256_Ed25519;

/// Re-exported openmls types that cross this crate's API (so dependents need
/// not depend on openmls directly).
pub use openmls::prelude::{KeyPackage, LeafNodeIndex};

/// Label used for the exported per-epoch secret we compare across views.
const EPOCH_PROOF_LABEL: &str = "lineage-epoch-proof";

#[derive(Debug, thiserror::Error)]
pub enum MlsError {
    #[error("no active group on this device")]
    NoGroup,
    #[error("openmls operation failed: {0}")]
    Op(String),
    #[error("wire (de)serialization failed: {0}")]
    Wire(String),
    #[error("unexpected message body: {0}")]
    UnexpectedBody(String),
}

type Result<T> = std::result::Result<T, MlsError>;

fn op<E: std::fmt::Debug>(e: E) -> MlsError {
    MlsError::Op(format!("{e:?}"))
}

/// A signed claim that a device belongs to a lineage (T1, multi-device §10.1).
///
/// The device key is *independent* (logical binding, the decided model); the
/// lineage-root key signs the canonical `(lineage_id, device_did)` bytes, and
/// that signature is what makes the claim unforgeable. The claim is encoded into
/// the MLS leaf's `BasicCredential` identity (see [`Device::new_with_lineage`]),
/// so any other member can read it off the leaf and verify it from signed data
/// alone — the prerequisite for the presentation fold (E2.9) and lineage-counted
/// thresholds (E2.10).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineageClaim {
    lineage_id: Did,
    device_did: Did,
    lineage_sig: Sig,
}

/// Magic prefix distinguishing a lineage-claim identity from a plain DID identity
/// (so `leaf_index_of` can transparently handle both shapes).
const LINEAGE_CLAIM_MAGIC: &[u8; 4] = b"LCL1";

/// Domain-separated bytes the lineage root signs. Length-prefixed so the two
/// fields can never be confused by concatenation.
fn lineage_signing_bytes(lineage_id: &Did, device_did: &Did) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(b"croft-lineage-claim-v1");
    let lid = lineage_id.0.as_bytes();
    let did = device_did.0.as_bytes();
    out.extend_from_slice(&(lid.len() as u32).to_le_bytes());
    out.extend_from_slice(lid);
    out.extend_from_slice(&(did.len() as u32).to_le_bytes());
    out.extend_from_slice(did);
    out
}

impl LineageClaim {
    /// Sign a claim binding `device_did` into the lineage rooted at `root`.
    pub fn sign(root: &SigningIdentity, device_did: &Did) -> Self {
        let lineage_id = root.did().clone();
        let lineage_sig = root.sign(&lineage_signing_bytes(&lineage_id, device_did));
        Self {
            lineage_id,
            device_did: device_did.clone(),
            lineage_sig,
        }
    }

    /// Reconstruct a claim from its parts (e.g. after reading it off a leaf).
    pub fn new(lineage_id: Did, device_did: Did, lineage_sig: Sig) -> Self {
        Self {
            lineage_id,
            device_did,
            lineage_sig,
        }
    }

    /// The lineage this claim asserts membership in.
    pub fn lineage_id(&self) -> &Did {
        &self.lineage_id
    }

    /// The device the claim is bound to.
    pub fn device_did(&self) -> &Did {
        &self.device_did
    }

    /// Verify the claim against the lineage root's public key. Returns true only
    /// if the root key's DID matches the asserted lineage id AND the signature is
    /// valid over the canonical bytes — so a claim naming a lineage it was not
    /// signed by is rejected (forgery resistance).
    pub fn verify(&self, root_vk: &VerifyingIdentity) -> bool {
        root_vk.did() == &self.lineage_id
            && root_vk.verify(
                &lineage_signing_bytes(&self.lineage_id, &self.device_did),
                &self.lineage_sig,
            )
    }

    /// Encode into MLS-leaf credential identity bytes (magic-prefixed, length-
    /// delimited). Crosses the real `tls_codec` wire inside the credential.
    fn encode(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(LINEAGE_CLAIM_MAGIC);
        let lid = self.lineage_id.0.as_bytes();
        let did = self.device_did.0.as_bytes();
        out.extend_from_slice(&(lid.len() as u32).to_le_bytes());
        out.extend_from_slice(lid);
        out.extend_from_slice(&(did.len() as u32).to_le_bytes());
        out.extend_from_slice(did);
        out.extend_from_slice(&self.lineage_sig.0);
        out
    }

    /// Decode from credential identity bytes. Returns `None` for anything that is
    /// not a lineage claim (e.g. a plain-DID `Device::new` identity).
    fn decode(bytes: &[u8]) -> Option<Self> {
        let rest = bytes.strip_prefix(LINEAGE_CLAIM_MAGIC.as_slice())?;
        let (lid, rest) = take_lp(rest)?;
        let (did, rest) = take_lp(rest)?;
        if rest.len() != 64 {
            return None;
        }
        let mut sig = [0u8; 64];
        sig.copy_from_slice(rest);
        Some(Self {
            lineage_id: Did(String::from_utf8(lid).ok()?),
            device_did: Did(String::from_utf8(did).ok()?),
            lineage_sig: Sig(sig),
        })
    }
}

/// Read a u32-LE length-prefixed byte field, returning (field, remainder).
fn take_lp(bytes: &[u8]) -> Option<(Vec<u8>, &[u8])> {
    if bytes.len() < 4 {
        return None;
    }
    let (len_bytes, rest) = bytes.split_at(4);
    let len = u32::from_le_bytes(len_bytes.try_into().ok()?) as usize;
    if rest.len() < len {
        return None;
    }
    let (field, rest) = rest.split_at(len);
    Some((field.to_vec(), rest))
}

/// What `Device::recv` produced after processing one message.
#[derive(Debug)]
pub enum Received {
    /// An application message: the decrypted plaintext.
    Application(Vec<u8>),
    /// A commit was processed and merged; the device's epoch advanced.
    CommitMerged,
}

/// A participant + its local MLS state. In Phase 1 one `Device` is one member
/// view; the DID is what later lets several devices share an identity.
pub struct Device {
    pub did: Did,
    provider: OpenMlsRustCrypto,
    signer: SignatureKeyPair,
    credential: CredentialWithKey,
    group: Option<MlsGroup>,
}

impl Device {
    /// Create a fresh device with a basic credential bound to `did`.
    pub fn new(did: Did) -> Result<Self> {
        let provider = OpenMlsRustCrypto::default();
        let signer =
            SignatureKeyPair::new(CIPHERSUITE.signature_algorithm()).map_err(op)?;
        signer.store(provider.storage()).map_err(op)?;

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

    /// Create a device under a lineage (T1). The MLS leaf carries the signed
    /// [`LineageClaim`] in its `BasicCredential` identity, so other members can
    /// fold devices and count thresholds by lineage from signed data alone. The
    /// device's signing key is independent of the lineage root key (logical
    /// binding — multi-device.md §10.1).
    pub fn new_with_lineage(device_did: Did, claim: LineageClaim) -> Result<Self> {
        let provider = OpenMlsRustCrypto::default();
        let signer =
            SignatureKeyPair::new(CIPHERSUITE.signature_algorithm()).map_err(op)?;
        signer.store(provider.storage()).map_err(op)?;

        let credential = BasicCredential::new(claim.encode());
        let credential = CredentialWithKey {
            credential: credential.into(),
            signature_key: signer.public().into(),
        };

        Ok(Self {
            did: device_did,
            provider,
            signer,
            credential,
            group: None,
        })
    }

    /// Read the [`LineageClaim`] carried on the leaf at `index`, if any. Returns
    /// `None` for a plain-DID member or an absent leaf. This is the leaf→lineage
    /// mapping every client computes (member-list fold, lineage-counted thresholds).
    pub fn lineage_claim_of(&self, index: LeafNodeIndex) -> Option<LineageClaim> {
        let group = self.group.as_ref()?;
        for m in group.members() {
            if m.index == index {
                let basic = BasicCredential::try_from(m.credential.clone()).ok()?;
                return LineageClaim::decode(basic.identity());
            }
        }
        None
    }

    /// Fold the group's leaves into actors by lineage (E2.9 / corpus C4). Leaves
    /// carrying the same lineage id collapse to one actor; a plain-DID leaf is its
    /// own actor. Computed only from the leaf credentials every client holds, so
    /// all clients fold identically — the prerequisite for a consistent member
    /// list and for lineage-counted thresholds (E2.10). Returns actor-key → the
    /// leaf indices folded under it.
    pub fn fold_by_lineage(&self) -> Option<std::collections::BTreeMap<String, Vec<LeafNodeIndex>>> {
        let group = self.group.as_ref()?;
        let mut actors: std::collections::BTreeMap<String, Vec<LeafNodeIndex>> =
            std::collections::BTreeMap::new();
        for m in group.members() {
            let basic = BasicCredential::try_from(m.credential.clone()).ok()?;
            let key = match LineageClaim::decode(basic.identity()) {
                Some(claim) => format!("lineage:{}", claim.lineage_id().0),
                None => format!("did:{}", String::from_utf8_lossy(basic.identity())),
            };
            actors.entry(key).or_default().push(m.index);
        }
        Some(actors)
    }

    /// Produce (and persist) a key package so another device can add us.
    pub fn key_package(&self) -> Result<KeyPackage> {
        let bundle = KeyPackage::builder()
            .build(
                CIPHERSUITE,
                &self.provider,
                &self.signer,
                self.credential.clone(),
            )
            .map_err(op)?;
        Ok(bundle.key_package().clone())
    }

    /// Become the founder of a new group (genesis).
    pub fn create_group(&mut self) -> Result<()> {
        let group = MlsGroup::builder()
            .ciphersuite(CIPHERSUITE)
            // Embed the ratchet tree in welcomes/group-info so newcomers can
            // join without an out-of-band tree (the in-process bus carries
            // only the welcome). Realistic for a group of this scale.
            .use_ratchet_tree_extension(true)
            .build(&self.provider, &self.signer, self.credential.clone())
            .map_err(op)?;
        self.group = Some(group);
        Ok(())
    }

    fn group_ref(&self) -> Result<&MlsGroup> {
        self.group.as_ref().ok_or(MlsError::NoGroup)
    }

    /// Add members. Returns `(commit, welcome)` to fan out to existing members
    /// and to the newcomers respectively. The local epoch is advanced.
    pub fn add(&mut self, key_packages: &[KeyPackage]) -> Result<(Vec<u8>, Vec<u8>)> {
        let provider = &self.provider;
        let signer = &self.signer;
        let group = self.group.as_mut().ok_or(MlsError::NoGroup)?;
        let (commit, welcome, _group_info) =
            group.add_members(provider, signer, key_packages).map_err(op)?;
        group.merge_pending_commit(provider).map_err(op)?;
        Ok((wire(&commit)?, wire(&welcome)?))
    }

    /// Remove members by leaf index. Returns the commit (and optional welcome)
    /// to fan out. Local epoch advances.
    pub fn remove(&mut self, members: &[LeafNodeIndex]) -> Result<(Vec<u8>, Option<Vec<u8>>)> {
        let provider = &self.provider;
        let signer = &self.signer;
        let group = self.group.as_mut().ok_or(MlsError::NoGroup)?;
        let (commit, welcome, _gi) = group.remove_members(provider, signer, members).map_err(op)?;
        group.merge_pending_commit(provider).map_err(op)?;
        let welcome = match welcome {
            Some(w) => Some(wire(&w)?),
            None => None,
        };
        Ok((wire(&commit)?, welcome))
    }

    /// Stage (but do NOT merge) a removal — models a commit produced while
    /// peers are offline (E1.4). Returns the commit bytes to queue.
    pub fn stage_remove(&mut self, members: &[LeafNodeIndex]) -> Result<Vec<u8>> {
        let provider = &self.provider;
        let signer = &self.signer;
        let group = self.group.as_mut().ok_or(MlsError::NoGroup)?;
        let (commit, _welcome, _gi) =
            group.remove_members(provider, signer, members).map_err(op)?;
        wire(&commit)
    }

    /// Merge a previously-staged own commit (companion to [`stage_remove`]).
    pub fn merge_own_pending(&mut self) -> Result<()> {
        let provider = &self.provider;
        let group = self.group.as_mut().ok_or(MlsError::NoGroup)?;
        group.merge_pending_commit(provider).map_err(op)
    }

    /// Join a group from a welcome message (the newcomer side of `add`).
    pub fn join_from_welcome(&mut self, welcome_bytes: &[u8], ratchet_tree: Option<&[u8]>) -> Result<()> {
        let welcome = match from_wire(welcome_bytes)?.extract() {
            MlsMessageBodyIn::Welcome(w) => w,
            other => {
                return Err(MlsError::UnexpectedBody(format!("expected welcome, got {other:?}")))
            }
        };

        let rt = match ratchet_tree {
            Some(bytes) => Some(deser_ratchet_tree(bytes)?),
            None => None,
        };

        let staged = StagedWelcome::new_from_welcome(
            &self.provider,
            &MlsGroupJoinConfig::default(),
            welcome,
            rt,
        )
        .map_err(op)?;
        let group = staged.into_group(&self.provider).map_err(op)?;
        self.group = Some(group);
        Ok(())
    }

    /// Export this group's verifiable group info + ratchet tree so another
    /// device can external-commit join. The "survivor publishes its epoch".
    pub fn publish_group_info(&self) -> Result<(Vec<u8>, Vec<u8>)> {
        let group = self.group_ref()?;
        let gi = group
            .export_group_info(self.provider.crypto(), &self.signer, false)
            .map_err(op)?;
        let rt = group.export_ratchet_tree();
        Ok((wire(&gi)?, ser_ratchet_tree(&rt)?))
    }

    /// **The survivor re-key primitive (E1.2).** Join an existing epoch with no
    /// prior welcome, using the target's published group info + tree. Returns
    /// the external commit to hand back to the existing members so they admit
    /// us. Sets our local group to the joined epoch.
    pub fn external_commit_join(
        &mut self,
        group_info_bytes: &[u8],
        ratchet_tree_bytes: &[u8],
    ) -> Result<Vec<u8>> {
        let vgi = match from_wire(group_info_bytes)?.extract() {
            MlsMessageBodyIn::GroupInfo(gi) => gi,
            other => {
                return Err(MlsError::UnexpectedBody(format!(
                    "expected group info, got {other:?}"
                )))
            }
        };
        let rt = deser_ratchet_tree(ratchet_tree_bytes)?;

        let (group, bundle) = MlsGroup::external_commit_builder()
            .with_ratchet_tree(rt)
            .with_config(MlsGroupJoinConfig::default())
            .build_group(&self.provider, vgi, self.credential.clone())
            .map_err(op)?
            .load_psks(self.provider.storage())
            .map_err(op)?
            .build(
                self.provider.rand(),
                self.provider.crypto(),
                &self.signer,
                |_| true,
            )
            .map_err(op)?
            .finalize(&self.provider)
            .map_err(op)?;

        let commit = wire(bundle.commit())?;
        self.group = Some(group);
        Ok(commit)
    }

    /// **The "mint a third" path (E1.3).** Found a brand-new group and re-add
    /// the supplied key packages. The two prior epochs are not referenced by
    /// MLS at all — they are simply abandoned (lineage binding is Phase 2).
    /// Returns `(commit, welcome)` for the re-added members.
    pub fn fresh_genesis(&mut self, members: &[KeyPackage]) -> Result<(Vec<u8>, Vec<u8>)> {
        self.create_group()?;
        self.add(members)
    }

    /// Encrypt an application message.
    pub fn send(&mut self, plaintext: &[u8]) -> Result<Vec<u8>> {
        let provider = &self.provider;
        let signer = &self.signer;
        let group = self.group.as_mut().ok_or(MlsError::NoGroup)?;
        let out = group.create_message(provider, signer, plaintext).map_err(op)?;
        wire(&out)
    }

    /// Process an incoming message: decrypt application data, or merge a commit.
    pub fn recv(&mut self, bytes: &[u8]) -> Result<Received> {
        let protocol_msg: ProtocolMessage = from_wire(bytes)?
            .try_into_protocol_message()
            .map_err(|e| MlsError::UnexpectedBody(format!("{e:?}")))?;
        let provider = &self.provider;
        let group = self.group.as_mut().ok_or(MlsError::NoGroup)?;
        let processed = group.process_message(provider, protocol_msg).map_err(op)?;
        match processed.into_content() {
            ProcessedMessageContent::ApplicationMessage(app) => {
                Ok(Received::Application(app.into_bytes()))
            }
            ProcessedMessageContent::StagedCommitMessage(staged) => {
                group.merge_staged_commit(provider, *staged).map_err(op)?;
                Ok(Received::CommitMerged)
            }
            other => Err(MlsError::UnexpectedBody(format!("{other:?}"))),
        }
    }

    /// Current epoch number, for assertions.
    pub fn epoch(&self) -> Result<u64> {
        Ok(self.group_ref()?.epoch().as_u64())
    }

    /// Number of members currently in the group.
    pub fn member_count(&self) -> Result<usize> {
        Ok(self.group_ref()?.members().count())
    }

    /// The exported per-epoch proof secret. Two views in the same epoch of the
    /// same group derive identical bytes; this is how we prove "both sides
    /// compute the same group secret" (I4 / E1.2).
    pub fn epoch_proof(&self) -> Result<Vec<u8>> {
        let group = self.group_ref()?;
        group
            .export_secret(self.provider.crypto(), EPOCH_PROOF_LABEL, b"", 32)
            .map_err(op)
    }

    /// Find the leaf index of the member identified by `did` (so removals can
    /// target a DID rather than a raw index). Handles both shapes: a lineage-claim
    /// leaf matches on the claim's `device_did`; a plain-DID leaf matches on the
    /// raw credential identity.
    pub fn leaf_index_of(&self, did: &Did) -> Result<Option<LeafNodeIndex>> {
        let want = did.credential_identity();
        for m in self.group_ref()?.members() {
            if let Ok(basic) = BasicCredential::try_from(m.credential.clone()) {
                let id = basic.identity();
                match LineageClaim::decode(id) {
                    Some(claim) if claim.device_did() == did => return Ok(Some(m.index)),
                    Some(_) => continue,
                    None if id == want.as_slice() => return Ok(Some(m.index)),
                    None => continue,
                }
            }
        }
        Ok(None)
    }
}

// --- wire helpers: every artifact crosses a real tls_codec boundary ---------

fn wire(msg: &MlsMessageOut) -> Result<Vec<u8>> {
    msg.tls_serialize_detached()
        .map_err(|e| MlsError::Wire(format!("{e:?}")))
}

fn from_wire(bytes: &[u8]) -> Result<MlsMessageIn> {
    // `tls_deserialize_exact` rejects trailing bytes after a valid message
    // prefix; plain `tls_deserialize` leaves them in the buffer. Since these
    // bytes arrive over the (untrusted) wire, we require exact consumption.
    MlsMessageIn::tls_deserialize_exact(bytes).map_err(|e| MlsError::Wire(format!("{e:?}")))
}

fn ser_ratchet_tree(rt: &RatchetTree) -> Result<Vec<u8>> {
    rt.tls_serialize_detached()
        .map_err(|e| MlsError::Wire(format!("{e:?}")))
}

fn deser_ratchet_tree(bytes: &[u8]) -> Result<RatchetTreeIn> {
    RatchetTreeIn::tls_deserialize_exact(bytes).map_err(|e| MlsError::Wire(format!("{e:?}")))
}
