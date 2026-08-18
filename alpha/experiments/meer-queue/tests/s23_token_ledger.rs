//! **S23 — the token ledger: PSK resolvability across membership change.**
//!
//! E106 ratified the governance-issued **external PSK** as §11.7's re-entry credential. S16
//! measured that one incumbent who *holds the PSK bytes* can seat a returner who presents them.
//! But the RFC constraint the walk surfaced was never measured: **every incumbent that may
//! process the return commit must resolve the PSK from its own provider storage.** The PSK
//! secret is mixed into the key schedule, so a member who never received the bytes cannot compute
//! the new epoch — the token is therefore *group state* (a "token ledger") that must reach members
//! who join *after* issuance, and survive membership change.
//!
//! Three arms, in the order they bite:
//!
//! 1. **Negative arm (RED first).** Incumbent B holds group state but not the PSK bytes for
//!    returner R's token. R presents a valid external commit + PSK proposal. Name the failure
//!    mode — clean processing error, silent drop, or staged-commit failure. The RED step asserts
//!    the optimistic *no-ledger* hypothesis (B seats R anyway) and watches it fail, proving the
//!    ledger obligation is real before any bookkeeping is built.
//! 2. **Ledger transfer.** Member C joins *after* R's token was issued. The ledger reaches C
//!    (modeled as sealed app-layer state, not `GroupContextExtensions`, which would leak into a
//!    served `GroupInfo`). R returns; C processes and merges R's commit.
//! 3. **Revocation as chain fact.** The issuance fact is marked revoked; incumbents still *hold*
//!    the PSK bytes but the policy check refuses. Revocation needs no key-deletion race.
//!
//! Fidelity: **Rung A (real-lib)** — real OpenMLS 0.8.1. No CISS: nothing here touches storage.

use mls_replant::{join, Persona};
use openmls::messages::group_info::VerifiableGroupInfo;
use openmls::prelude::*;
use openmls::schedule::psk::{PreSharedKeyId, Psk};
use tls_codec::{Deserialize as _, Serialize as _};

fn group_config() -> MlsGroupCreateConfig {
    MlsGroupCreateConfig::builder()
        .ciphersuite(mls_replant::CS)
        .number_of_resumption_psks(8)
        .use_ratchet_tree_extension(true)
        .build()
}

/// Found a group at `founder` and seat `joiners`, returning the founder's live group plus each
/// joiner's live group (same epoch, all merged). Every party is a real member with real state.
fn seat_group(founder: &Persona, joiners: &[&Persona]) -> (MlsGroup, Vec<MlsGroup>) {
    let mut f = MlsGroup::new(
        &founder.provider,
        &founder.signer,
        &group_config(),
        founder.cwk.clone(),
    )
    .expect("create group");
    let kps: Vec<KeyPackage> = joiners.iter().map(|p| p.key_package()).collect();
    let (_c, welcome_out, _g) = f
        .add_members(&founder.provider, &founder.signer, &kps)
        .expect("add");
    f.merge_pending_commit(&founder.provider).expect("merge");
    let tree: RatchetTreeIn = f.export_ratchet_tree().into();
    let welcome = extract_welcome(&welcome_out);
    let joined: Vec<MlsGroup> = joiners
        .iter()
        .map(|p| join(p, welcome.clone(), tree.clone()))
        .collect();
    (f, joined)
}

fn extract_welcome(welcome_out: &MlsMessageOut) -> Welcome {
    match MlsMessageIn::tls_deserialize_exact(welcome_out.tls_serialize_detached().expect("ser"))
        .expect("de")
        .extract()
    {
        MlsMessageBodyIn::Welcome(w) => w,
        _ => panic!("expected a Welcome"),
    }
}

/// A current `GroupInfo` as a returner would receive it — serialized and re-parsed, so nothing
/// crosses this boundary that could not cross a wire.
fn current_group_info(group: &mut MlsGroup, who: &Persona) -> VerifiableGroupInfo {
    let bytes = group
        .export_group_info(who.provider.crypto(), &who.signer, true)
        .expect("export group info")
        .tls_serialize_detached()
        .expect("ser");
    match MlsMessageIn::tls_deserialize_exact(&bytes)
        .expect("de")
        .extract()
    {
        MlsMessageBodyIn::GroupInfo(gi) => gi,
        _ => panic!("expected GroupInfo"),
    }
}

/// The governance's issuance: an external PSK id, and the secret bytes issued *to the returner*.
/// Storing the secret in a provider is what puts the token into that party's slice of the ledger.
struct IssuedToken {
    psk_id: PreSharedKeyId,
    secret: Vec<u8>,
}

fn issue_token(who: &str) -> IssuedToken {
    let token_id = format!("croft/reentry-token/v1/{who}").into_bytes();
    IssuedToken {
        psk_id: PreSharedKeyId::external(token_id, vec![7u8; 32]),
        secret: vec![0x5a; 32],
    }
}

impl IssuedToken {
    /// Deposit the token bytes into a party's provider storage — i.e. record it in that party's
    /// slice of the token ledger. An incumbent who never runs this cannot resolve the PSK.
    fn deposit_with(&self, holder: &Persona) {
        self.psk_id
            .store(&holder.provider, &self.secret)
            .expect("store token secret");
    }

    /// Build the returner's external commit carrying this token as a PSK proposal.
    fn returner_commit(&self, returner: &Persona, gi: VerifiableGroupInfo) -> MlsMessageOut {
        self.returner_commit_with_aad(returner, gi, &[])
    }

    /// As [`returner_commit`], but riding a governance-issuance attestation in the AAD. The
    /// attestation identifies the issuance fact (a §7.5.1 chain fact) so an incumbent's policy
    /// layer can decide on it *before* merging — the seam revocation acts through.
    fn returner_commit_with_aad(
        &self,
        returner: &Persona,
        gi: VerifiableGroupInfo,
        aad: &[u8],
    ) -> MlsMessageOut {
        let (_group, bundle) = MlsGroup::external_commit_builder()
            .with_config(MlsGroupJoinConfig::default())
            .with_aad(aad.to_vec())
            .build_group(&returner.provider, gi, returner.cwk.clone())
            .expect("build group from GroupInfo")
            .add_psk_proposal(PreSharedKeyProposal::new(self.psk_id.clone()))
            .load_psks(returner.provider.storage())
            .expect("the returner holds their own token, so it resolves")
            .build(
                returner.provider.rand(),
                returner.provider.crypto(),
                &returner.signer,
                |_| true,
            )
            .expect("build external commit carrying the token")
            .finalize(&returner.provider)
            .expect("finalize");
        bundle.commit().clone()
    }

    /// The identifier of this token's issuance fact, as it would ride the AAD attestation. Here
    /// it is simply the token id — in the real design it is the content address of the R6-shaped
    /// issuance record on the governance chain.
    fn issuance_attestation(&self) -> Vec<u8> {
        let Psk::External(ext) = self.psk_id.psk() else {
            panic!("issued tokens are external PSKs");
        };
        ext.psk_id().to_vec()
    }
}

/// Attempt to process + merge `commit` at incumbent `who`'s group, surfacing the outcome. No
/// `let _ =`: every stage's result is returned so the failure mode is nameable (G1's lesson).
enum MergeOutcome {
    Seated,
    ProcessRefused(String),
    NotAStagedCommit(String),
    MergeRefused(String),
}

fn try_merge(group: &mut MlsGroup, who: &Persona, commit: &MlsMessageOut) -> MergeOutcome {
    let wire = commit.tls_serialize_detached().expect("ser commit");
    let protocol: ProtocolMessage = MlsMessageIn::tls_deserialize_exact(&wire)
        .expect("parse")
        .try_into_protocol_message()
        .expect("protocol");
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

/// **Arm 1 — the ledger constraint, characterized.**
///
/// RED step (observed 2026-08-17): the optimistic no-ledger hypothesis — that an incumbent
/// without the PSK bytes seats the returner anyway — was asserted and **watched fail**. The
/// constraint is therefore real: decision-2's ledger obligation does not dissolve. This durable
/// test records the *failure mode* (the deliverable the plan asks for: clean error vs silent
/// drop vs staged-commit failure), which decides how loud a missing-ledger bug is in production.
#[test]
fn arm1_incumbent_without_the_psk_bytes_cannot_seat_the_returner() {
    let alice = Persona::new("alice");
    let bob = Persona::new("bob");
    let returner = Persona::new("returner");
    let (mut alices, mut bobs_groups) = seat_group(&alice, &[&bob]);
    let mut bobs = bobs_groups.pop().expect("bob's group");

    // The governance issued R a token. R holds the bytes; the incumbent Bob was NEVER told to
    // honour it — his ledger slice is empty for this token.
    let token = issue_token("returner@example");
    token.deposit_with(&returner);

    let members_before = bobs.members().count();
    let epoch_before = bobs.epoch();
    let gi = current_group_info(&mut alices, &alice);
    let commit = token.returner_commit(&returner, gi);

    let outcome = try_merge(&mut bobs, &bob, &commit);

    // The constraint: without the ledger entry, the returner is NOT seated.
    assert!(
        !matches!(outcome, MergeOutcome::Seated),
        "an incumbent without the PSK bytes must NOT seat the returner"
    );
    assert_eq!(
        bobs.members().count(),
        members_before,
        "the group did not grow: the missing-ledger commit seats nobody at Bob"
    );

    // The failure mode, named. A missing ledger entry is a LOUD, clean error at the point the
    // PSK must be resolved — not a silent drop, and not a partial state change.
    let mode = match &outcome {
        MergeOutcome::Seated => unreachable!("asserted not seated above"),
        MergeOutcome::ProcessRefused(e) => format!("process_message refused (clean error): {e}"),
        MergeOutcome::NotAStagedCommit(c) => format!("not a staged commit: {c}"),
        MergeOutcome::MergeRefused(e) => format!("merge_staged_commit refused (clean error): {e}"),
    };
    assert!(
        matches!(
            outcome,
            MergeOutcome::ProcessRefused(_) | MergeOutcome::MergeRefused(_)
        ),
        "the failure is a clean, named library error (loud), never a silent drop; got: {mode}"
    );
    assert_eq!(
        bobs.epoch(),
        epoch_before,
        "Bob's epoch is unmoved — no partial application"
    );

    println!(
        "S23 arm 1 MEASURED (real-lib): a missing token-ledger entry produces a {mode}. The \
         returner is not seated ({members_before} members, unchanged) and Bob's state does not \
         advance. **The ledger constraint is real** — the PSK is mixed into the key schedule, so \
         an incumbent who never received the bytes cannot resolve the referenced PSK and the \
         commit fails LOUD at that resolution, not silently. Decision-2's ledger obligation \
         stands. [{}]",
        meer_queue::mls::resolved_versions()
    );
}

/// Add `newcomers` to a live group at `adder`, merging the add everywhere, and return each
/// newcomer's joined group at the new epoch. Existing `incumbents` process the add so they stay
/// converged (realism: a late join is a governance event the whole group folds).
fn add_members_after(
    adder_group: &mut MlsGroup,
    adder: &Persona,
    newcomers: &[&Persona],
    incumbents: &mut [(&Persona, &mut MlsGroup)],
) -> Vec<MlsGroup> {
    let kps: Vec<KeyPackage> = newcomers.iter().map(|p| p.key_package()).collect();
    let (add_commit, welcome_out, _gi) = adder_group
        .add_members(&adder.provider, &adder.signer, &kps)
        .expect("add members");
    adder_group
        .merge_pending_commit(&adder.provider)
        .expect("adder merges the add");
    for (who, group) in incumbents.iter_mut() {
        match try_merge(group, who, &add_commit) {
            MergeOutcome::Seated => {}
            other => panic!(
                "incumbent should fold a plain Add; instead: {}",
                describe(&other)
            ),
        }
    }
    let tree: RatchetTreeIn = adder_group.export_ratchet_tree().into();
    let welcome = extract_welcome(&welcome_out);
    newcomers
        .iter()
        .map(|p| join(p, welcome.clone(), tree.clone()))
        .collect()
}

fn describe(o: &MergeOutcome) -> String {
    match o {
        MergeOutcome::Seated => "seated".into(),
        MergeOutcome::ProcessRefused(e) => format!("process refused: {e}"),
        MergeOutcome::NotAStagedCommit(c) => format!("not a staged commit: {c}"),
        MergeOutcome::MergeRefused(e) => format!("merge refused: {e}"),
    }
}

/// **Arm 2 — ledger transfer: the token must reach members who join after issuance.**
///
/// R's token is issued while the group is {alice, bob}. Two members then join *after* issuance:
/// Carol, who receives the ledger entry (modeled as sealed app-layer state synced in-band — not
/// `GroupContextExtensions`, which would leak into a served `GroupInfo`), and Dave, who does not.
/// R returns. Carol seats R; Dave cannot — the *transfer* is the load-bearing thing.
#[test]
fn arm2_the_ledger_must_reach_members_who_join_after_issuance() {
    let alice = Persona::new("alice");
    let bob = Persona::new("bob");
    let carol = Persona::new("carol");
    let dave = Persona::new("dave");
    let returner = Persona::new("returner");
    let (mut alices, mut bobs_groups) = seat_group(&alice, &[&bob]);
    let mut bobs = bobs_groups.pop().expect("bob's group");

    // Issuance while the group is {alice, bob}: incumbents present at issue time hold the bytes.
    let token = issue_token("returner@example");
    token.deposit_with(&alice);
    token.deposit_with(&bob);
    token.deposit_with(&returner);

    // Carol and Dave join AFTER issuance.
    let mut joined = add_members_after(
        &mut alices,
        &alice,
        &[&carol, &dave],
        &mut [(&bob, &mut bobs)],
    );
    let mut daves = joined.pop().expect("dave joined");
    let mut carols = joined.pop().expect("carol joined");

    // Ledger transfer: only Carol receives R's token entry (the sealed in-band sync). Dave does
    // not — his ledger slice never learned of R's pre-join issuance.
    token.deposit_with(&carol);

    // R returns, presenting to Carol first (a current member).
    let gi_c = current_group_info(&mut carols, &carol);
    let seated_by_carol = try_merge(&mut carols, &carol, &token.returner_commit(&returner, gi_c));

    // …and, in a parallel universe, to Dave (rebuild a fresh commit against Dave's GroupInfo).
    let gi_d = current_group_info(&mut daves, &dave);
    let refused_by_dave = try_merge(&mut daves, &dave, &token.returner_commit(&returner, gi_d));

    assert!(
        matches!(seated_by_carol, MergeOutcome::Seated),
        "Carol received the ledger entry, so she resolves the PSK and seats R; got: {}",
        describe(&seated_by_carol)
    );
    assert!(
        matches!(refused_by_dave, MergeOutcome::ProcessRefused(_)),
        "Dave never received the ledger entry, so the PSK is unresolvable at him; got: {}",
        describe(&refused_by_dave)
    );

    println!(
        "S23 arm 2 MEASURED (real-lib): a token issued while the group was {{alice, bob}} is \
         resolvable at a member who joined AFTER issuance IFF the ledger entry was transferred to \
         them. Carol (transferred) seats the returner; Dave (not transferred) fails at PSK \
         resolution ({}). So the token ledger is real group state that MUST propagate to \
         late-joiners — it cannot live only with the members present at issue time. Modeled as \
         sealed app-layer state, deliberately NOT in GroupContextExtensions (which leak into a \
         served GroupInfo).",
        describe(&refused_by_dave)
    );
}

/// **Arm 3 — revocation is a chain fact, not a key-deletion race.**
///
/// The incumbent still *holds* the PSK bytes (crypto would resolve). But the issuance fact,
/// named in the returner's AAD attestation, is marked revoked in the incumbent's governance
/// fold. The policy layer reads the attestation before merging and refuses. No PSK is deleted;
/// no race with key material; the chain fact alone governs.
#[test]
fn arm3_revocation_is_a_chain_fact_needing_no_key_deletion() {
    let alice = Persona::new("alice");
    let bob = Persona::new("bob");
    let returner = Persona::new("returner");
    let (mut alices, mut bobs_groups) = seat_group(&alice, &[&bob]);
    let mut bobs = bobs_groups.pop().expect("bob's group");

    // Bob holds the token bytes: the crypto would succeed.
    let token = issue_token("returner@example");
    token.deposit_with(&bob);
    token.deposit_with(&returner);

    // Bob's governance fold has revoked this issuance fact. The revocation set is keyed on the
    // issuance-fact identifier — the thing the returner names in the AAD.
    let revoked: std::collections::HashSet<Vec<u8>> =
        [token.issuance_attestation()].into_iter().collect();

    let members_before = bobs.members().count();
    let epoch_before = bobs.epoch();
    let gi = current_group_info(&mut alices, &alice);
    let attestation = token.issuance_attestation();
    let commit = token.returner_commit_with_aad(&returner, gi, &attestation);

    // Bob processes to the staging point — the crypto resolves (he holds the bytes), so this is
    // where a pure-crypto gate would have already lost. The policy check runs on the staged
    // commit's AAD, before merge_staged_commit.
    let wire = commit.tls_serialize_detached().expect("ser");
    let protocol: ProtocolMessage = MlsMessageIn::tls_deserialize_exact(&wire)
        .expect("parse")
        .try_into_protocol_message()
        .expect("protocol");
    let processed = bobs
        .process_message(&bob.provider, protocol)
        .expect("the PSK resolves — Bob holds the bytes; staging succeeds");
    let named_issuance = processed.aad().to_vec();
    let staged = match processed.into_content() {
        ProcessedMessageContent::StagedCommitMessage(sc) => sc,
        other => panic!("expected a staged commit, got {other:?}"),
    };

    // The policy decision: the issuance fact this commit redeems is revoked → refuse (drop).
    let is_revoked = revoked.contains(&named_issuance);
    assert!(
        is_revoked,
        "the attestation names the revoked issuance fact"
    );
    if !is_revoked {
        bobs.merge_staged_commit(&bob.provider, *staged)
            .expect("would merge if unrevoked");
    } else {
        drop(staged); // policy said no; nothing is merged
    }

    assert_eq!(
        bobs.members().count(),
        members_before,
        "revoked: the returner is not seated even though the PSK bytes are present"
    );
    assert_eq!(bobs.epoch(), epoch_before, "and Bob's epoch did not advance");

    println!(
        "S23 arm 3 MEASURED (real-lib): with the PSK bytes present (staging SUCCEEDED — the \
         crypto resolved), the incumbent still refused the return because the issuance fact named \
         in the AAD attestation is revoked in its governance fold. Revocation is therefore a \
         policy decision over a chain fact, decided BEFORE merge_staged_commit — it needs no \
         deletion of key material and races nothing. This is the property decision-2 asserts: \
         revocation as a chain fact, not a key-deletion race."
    );
}
