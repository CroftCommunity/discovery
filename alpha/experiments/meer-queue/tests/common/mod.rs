//! Shared Rung-A MLS helpers for the E112 token-reentry / admission-fact experiments
//! (S24, C4). Real OpenMLS 0.8.1 throughout; the external-PSK + external-commit patterns are
//! the ones S16 measured.

#![allow(dead_code)]

use mls_replant::{join, Persona};
use openmls::messages::group_info::VerifiableGroupInfo;
use openmls::prelude::*;
use openmls::schedule::psk::{PreSharedKeyId, Psk};
use tls_codec::{Deserialize as _, Serialize as _};

pub fn group_config() -> MlsGroupCreateConfig {
    MlsGroupCreateConfig::builder()
        .ciphersuite(mls_replant::CS)
        .number_of_resumption_psks(8)
        .use_ratchet_tree_extension(true)
        .build()
}

/// Found a group at `founder` and seat `joiners`, returning the founder's group + each joiner's.
pub fn seat_group(founder: &Persona, joiners: &[&Persona]) -> (MlsGroup, Vec<MlsGroup>) {
    let mut f = MlsGroup::new(&founder.provider, &founder.signer, &group_config(), founder.cwk.clone())
        .expect("create group");
    let kps: Vec<KeyPackage> = joiners.iter().map(|p| p.key_package()).collect();
    let (_c, welcome_out, _g) = f
        .add_members(&founder.provider, &founder.signer, &kps)
        .expect("add");
    f.merge_pending_commit(&founder.provider).expect("merge");
    let tree: RatchetTreeIn = f.export_ratchet_tree().into();
    let welcome = extract_welcome(&welcome_out);
    let joined = joiners.iter().map(|p| join(p, welcome.clone(), tree.clone())).collect();
    (f, joined)
}

pub fn extract_welcome(welcome_out: &MlsMessageOut) -> Welcome {
    match MlsMessageIn::tls_deserialize_exact(welcome_out.tls_serialize_detached().expect("ser"))
        .expect("de")
        .extract()
    {
        MlsMessageBodyIn::Welcome(w) => w,
        _ => panic!("expected a Welcome"),
    }
}

/// A current `GroupInfo` as a returner would receive it — serialized and re-parsed.
pub fn current_group_info(group: &mut MlsGroup, who: &Persona) -> VerifiableGroupInfo {
    let bytes = group
        .export_group_info(who.provider.crypto(), &who.signer, true)
        .expect("export group info")
        .tls_serialize_detached()
        .expect("ser");
    match MlsMessageIn::tls_deserialize_exact(&bytes).expect("de").extract() {
        MlsMessageBodyIn::GroupInfo(gi) => gi,
        _ => panic!("expected GroupInfo"),
    }
}

/// A governance-issued external PSK, and the bytes issued to the returner.
pub struct IssuedToken {
    pub psk_id: PreSharedKeyId,
    pub secret: Vec<u8>,
    pub token_id: Vec<u8>,
}

pub fn issue_token(who: &str) -> IssuedToken {
    let token_id = format!("croft/reentry-token/v1/{who}").into_bytes();
    IssuedToken {
        psk_id: PreSharedKeyId::external(token_id.clone(), vec![7u8; 32]),
        secret: vec![0x5a; 32],
        token_id,
    }
}

impl IssuedToken {
    pub fn deposit_with(&self, holder: &Persona) {
        self.psk_id.store(&holder.provider, &self.secret).expect("store token secret");
    }

    /// Build the returner's external commit carrying this token as a PSK proposal, riding `aad`.
    /// Returns the commit message (what an incumbent processes).
    pub fn returner_commit_with_aad(
        &self,
        returner: &Persona,
        gi: VerifiableGroupInfo,
        aad: &[u8],
    ) -> MlsMessageOut {
        let (_g, bundle) = MlsGroup::external_commit_builder()
            .with_config(MlsGroupJoinConfig::default())
            .with_aad(aad.to_vec())
            .build_group(&returner.provider, gi, returner.cwk.clone())
            .expect("build group from GroupInfo")
            .add_psk_proposal(PreSharedKeyProposal::new(self.psk_id.clone()))
            .load_psks(returner.provider.storage())
            .expect("the returner holds their own token")
            .build(returner.provider.rand(), returner.provider.crypto(), &returner.signer, |_| true)
            .expect("build external commit")
            .finalize(&returner.provider)
            .expect("finalize");
        bundle.commit().clone()
    }

    pub fn returner_commit(&self, returner: &Persona, gi: VerifiableGroupInfo) -> MlsMessageOut {
        self.returner_commit_with_aad(returner, gi, &[])
    }

    /// As `returner_commit_with_aad`, but also returns the returner's own live group (so a test can
    /// read messages the returner could decrypt during its span).
    pub fn returner_join(
        &self,
        returner: &Persona,
        gi: VerifiableGroupInfo,
        aad: &[u8],
    ) -> (MlsMessageOut, MlsGroup) {
        let (group, bundle) = MlsGroup::external_commit_builder()
            .with_config(MlsGroupJoinConfig::default())
            .with_aad(aad.to_vec())
            .build_group(&returner.provider, gi, returner.cwk.clone())
            .expect("build group from GroupInfo")
            .add_psk_proposal(PreSharedKeyProposal::new(self.psk_id.clone()))
            .load_psks(returner.provider.storage())
            .expect("the returner holds their own token")
            .build(returner.provider.rand(), returner.provider.crypto(), &returner.signer, |_| true)
            .expect("build external commit")
            .finalize(&returner.provider)
            .expect("finalize");
        (bundle.commit().clone(), group)
    }

    /// The issuance-fact id this token names in its AAD attestation (here, the token id).
    #[must_use]
    pub fn issuance_attestation(&self) -> Vec<u8> {
        let Psk::External(ext) = self.psk_id.psk() else { panic!("external psk") };
        ext.psk_id().to_vec()
    }
}

/// The outcome of processing + merging a commit at an incumbent — every stage's result surfaced.
pub enum MergeOutcome {
    Seated,
    ProcessRefused(String),
    NotAStagedCommit(String),
    MergeRefused(String),
}

pub fn describe(o: &MergeOutcome) -> String {
    match o {
        MergeOutcome::Seated => "seated".into(),
        MergeOutcome::ProcessRefused(e) => format!("process refused: {e}"),
        MergeOutcome::NotAStagedCommit(c) => format!("not a staged commit: {c}"),
        MergeOutcome::MergeRefused(e) => format!("merge refused: {e}"),
    }
}

pub fn try_merge(group: &mut MlsGroup, who: &Persona, commit: &MlsMessageOut) -> MergeOutcome {
    let wire = commit.tls_serialize_detached().expect("ser commit");
    let protocol: ProtocolMessage = match MlsMessageIn::tls_deserialize_exact(&wire)
        .expect("parse")
        .try_into_protocol_message()
    {
        Ok(p) => p,
        Err(e) => return MergeOutcome::NotAStagedCommit(e.to_string()),
    };
    let processed = match group.process_message(&who.provider, protocol) {
        Ok(p) => p,
        Err(e) => return MergeOutcome::ProcessRefused(e.to_string()),
    };
    let staged = match processed.into_content() {
        ProcessedMessageContent::StagedCommitMessage(sc) => sc,
        other => return MergeOutcome::NotAStagedCommit(format!("{other:?}")),
    };
    match group.merge_staged_commit(&who.provider, *staged) {
        Ok(()) => MergeOutcome::Seated,
        Err(e) => MergeOutcome::MergeRefused(e.to_string()),
    }
}

/// The wire bytes of a commit — the input to a content address.
#[must_use]
pub fn commit_wire(commit: &MlsMessageOut) -> Vec<u8> {
    commit.tls_serialize_detached().expect("ser commit")
}

/// A serialized `GroupInfo`, with or without the ratchet tree bundled — the two forms a serving
/// peer holds (bare proves current state; tree-bundled additionally admits).
#[must_use]
pub fn current_group_info_bytes(group: &mut MlsGroup, who: &Persona, with_tree: bool) -> Vec<u8> {
    group
        .export_group_info(who.provider.crypto(), &who.signer, with_tree)
        .expect("export group info")
        .tls_serialize_detached()
        .expect("ser")
}
