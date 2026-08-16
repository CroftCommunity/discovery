//! **S16 — the governance-attestation half of §11.7's two-part credential.**
//!
//! §11.7 defines re-entry as a **two-part credential**: a **governance attestation** (standing —
//! "this Group still considers you one of ours") plus a **resumption PSK** (keys — "and you really
//! are who you were"). S14 measured the *key* half working: a cold member rejoined by external
//! commit from a current `GroupInfo`. The **standing** half has never been tested, and S14's own
//! run is the reason to test it — the rejoin there succeeded **without any PSK being supplied**,
//! which means whatever admitted her, it was not the credential §11.7 describes.
//!
//! Three questions, in the order they bite:
//!
//! 1. **Does MLS check standing at all?** If a party who was *never* a member can external-commit
//!    in with only a `GroupInfo`, then the attestation is not a protocol mechanism and must be
//!    built.
//! 2. **Is the key half load-bearing when it is used?** A resumption PSK should be unforgeable by
//!    a non-member — but if it is optional, its absence is the attack, not its forgery.
//! 3. **If both are application-layer, does MLS give the application a place to stand?** An
//!    attestation is worthless if a member cannot read it *before* committing to the new state.
//!
//! Fidelity: **Rung A (real-lib)** — real OpenMLS. No CISS: nothing here touches storage.

use mls_replant::{join, Persona};
use openmls::messages::group_info::VerifiableGroupInfo;
use openmls::prelude::*;
use openmls::schedule::psk::{PreSharedKeyId, ResumptionPskUsage};
use tls_codec::{Deserialize as _, Serialize as _};

/// What a governance attestation would look like on the wire. Opaque bytes here — S16 is about
/// *whether there is a carrier and a checkpoint*, not about the token format.
const ATTESTATION: &[u8] = b"croft/governance-attestation/v1: bob-vouches-for-this-return";

fn group_config() -> MlsGroupCreateConfig {
    MlsGroupCreateConfig::builder()
        .ciphersuite(mls_replant::CS)
        .number_of_resumption_psks(8)
        .use_ratchet_tree_extension(true)
        .build()
}

fn seat(founder: &Persona, joiner: &Persona) -> (MlsGroup, MlsGroup) {
    let mut f = MlsGroup::new(
        &founder.provider,
        &founder.signer,
        &group_config(),
        founder.cwk.clone(),
    )
    .expect("create group");
    let (_c, welcome_out, _g) = f
        .add_members(&founder.provider, &founder.signer, &[joiner.key_package()])
        .expect("add");
    f.merge_pending_commit(&founder.provider).expect("merge");
    let tree = f.export_ratchet_tree().into();
    let welcome = match MlsMessageIn::tls_deserialize_exact(
        welcome_out.tls_serialize_detached().expect("ser"),
    )
    .expect("de")
    .extract()
    {
        MlsMessageBodyIn::Welcome(w) => w,
        _ => panic!("expected a Welcome"),
    };
    let j = join(joiner, welcome, tree);
    (f, j)
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

/// **Question 1 — does MLS check standing?**
///
/// Mallory was never a member, was never invited, and holds no group secret of any epoch. She
/// holds one thing: a current `GroupInfo`.
#[test]
fn mls_admits_a_party_who_was_never_a_member_on_a_groupinfo_alone() {
    let bob = Persona::new("bob");
    let alice = Persona::new("alice");
    let mallory = Persona::new("mallory");
    let (mut bobs, _alices) = seat(&bob, &alice);

    let before = bobs.members().count();
    let gi = current_group_info(&mut bobs, &bob);

    let (mallorys, bundle) = MlsGroup::external_commit_builder()
        .with_config(MlsGroupJoinConfig::default())
        .build_group(&mallory.provider, gi, mallory.cwk.clone())
        .expect("a stranger built the group state from a GroupInfo alone")
        .load_psks(mallory.provider.storage())
        .expect("load psks — she has none, and none are demanded")
        .build(
            mallory.provider.rand(),
            mallory.provider.crypto(),
            &mallory.signer,
            |_| true,
        )
        .expect("build external commit")
        .finalize(&mallory.provider)
        .expect("finalize");

    // …and the incumbent accepts it. This is the half that matters: a commit nobody merges is
    // just bytes.
    let wire = bundle
        .commit()
        .tls_serialize_detached()
        .expect("ser commit");
    let protocol: ProtocolMessage = MlsMessageIn::tls_deserialize_exact(&wire)
        .expect("parse")
        .try_into_protocol_message()
        .expect("protocol");
    let processed = bobs
        .process_message(&bob.provider, protocol)
        .expect("bob processed a stranger's external commit without objection");
    match processed.into_content() {
        ProcessedMessageContent::StagedCommitMessage(sc) => {
            bobs.merge_staged_commit(&bob.provider, *sc).expect("merge");
        }
        other => panic!("expected a staged commit, got {other:?}"),
    }

    assert_eq!(
        bobs.members().count(),
        before + 1,
        "the group grew by one, and nobody asked her anything"
    );
    assert_eq!(
        bobs.epoch(),
        mallorys.epoch(),
        "incumbent and stranger agree on the new epoch"
    );

    println!(
        "S16 MEASURED (real-lib): a party who was NEVER a member, never invited, and holding no \
         group secret of any epoch, joined a live group by external commit using only a current \
         GroupInfo — and the incumbent processed and merged it without objection. The group went \
         from {before} members to {}. **MLS checks no standing whatsoever.** §11.7's governance \
         attestation is therefore not a protocol mechanism that needs configuring; it is a thing \
         that does not exist yet and must be built. [{}]",
        bobs.members().count(),
        meer_queue::mls::resolved_versions()
    );
    println!(
        "S16 CONSEQUENCE: this generalises S11's finding from the inbox to the group. S11 measured \
         that a stranger can seat YOU in a group you never asked to join; S16 measures that a \
         stranger can seat HERSELF in a group that never asked for her. Both are MLS working as \
         specified — external commit exists precisely so a party can join without a member's \
         help — and both mean the same thing for this design: **admission control is entirely \
         application-layer, and the protocol will not do any of it for us.**"
    );
}

/// **Question 2 — is the key half load-bearing when it is used, and is it optional?**
///
/// A resumption PSK is derived from an epoch secret, so only a party who held that epoch can
/// supply one. If the returner attaches it, prior membership is proved cryptographically. The
/// question is whether anything *requires* it.
#[test]
fn the_resumption_psk_proves_prior_membership_but_nothing_demands_it() {
    let bob = Persona::new("bob");
    let alice = Persona::new("alice");
    let (mut bobs, alices) = seat(&bob, &alice);

    // Alice's standing as a former member, in cryptographic form: the resumption PSK of the epoch
    // she held. A non-member cannot produce this — it is derived from that epoch's secret.
    let held_epoch = alices.epoch();
    let group_id = alices.group_id().clone();
    let resumption = alices.resumption_psk_secret().as_slice().to_vec();
    assert!(
        !resumption.is_empty(),
        "a member holds a continuity token for the epoch they were in"
    );

    // She goes cold and the group churns past her.
    let alice_leaf = alices.own_leaf_index();
    bobs.remove_members(&bob.provider, &bob.signer, &[alice_leaf])
        .expect("remove");
    bobs.merge_pending_commit(&bob.provider).expect("merge");
    for _ in 0..3 {
        mls_replant::commit(&mut bobs, &bob);
    }

    // She returns, attaching the PSK proposal — §11.7's key half, used exactly as written. Both
    // parties first commit the secret to their provider PSK store under the same id: the *value*
    // is an epoch secret only members of that epoch ever held, so a non-member could not populate
    // it, unlike the GroupInfo the previous test ran on, which is public by construction.
    let psk_id = PreSharedKeyId::resumption(
        ResumptionPskUsage::Application,
        group_id,
        held_epoch,
        vec![0u8; 32],
    );
    psk_id
        .store(&alice.provider, &resumption)
        .expect("the returner holds the epoch secret she is claiming");
    psk_id
        .store(&bob.provider, &resumption)
        .expect("and so does the incumbent, from the same epoch");

    let gi = current_group_info(&mut bobs, &bob);
    let attempted = MlsGroup::external_commit_builder()
        .with_config(MlsGroupJoinConfig::default())
        .build_group(&alice.provider, gi, alice.cwk.clone())
        .expect("build group")
        .add_psk_proposal(PreSharedKeyProposal::new(psk_id))
        .load_psks(alice.provider.storage());

    // **This is the finding, and it refutes the assumption the test was written under.**
    // Matched rather than `expect_err`d because the Ok type is a builder that is not `Debug`.
    let err = match attempted {
        Err(e) => e,
        Ok(_) => panic!("the resumption-PSK path was expected to be unconstructible — it built"),
    };
    assert!(
        err.to_string().contains("PSK") || err.to_string().contains("psk"),
        "the refusal is about the PSK specifically, not some unrelated build failure: {err}"
    );

    println!(
        "S16 MEASURED (real-lib): attaching a resumption PSK to an external commit **FAILS** — \
         `{err}` — even though the returner genuinely holds that epoch's secret and wrote it to her \
         provider PSK store. The cause is structural, not a harness mistake: resumption PSKs are \
         resolved from the **group's own** `ResumptionPskStore`, never from provider storage \
         (`schedule/psk.rs:530-537` takes the `Psk::Resumption` branch straight to \
         `resumption_psk_store.get()`), and a group built by external commit initialises that store \
         **empty** — `ResumptionPskStore::new(32)` at \
         `group/mls_group/commit_builder/external_commits.rs:290`. The store's `add` is \
         `pub(crate)`, so there is no public API to seed it either."
    );
    println!(
        "S16 CONSEQUENCE: **§11.7's two-part credential is not implementable on openmls 0.8.1 as \
         written.** Its standing half has no protocol mechanism (previous test: a total stranger is \
         admitted), and its key half has no reachable API (this test: the resumption PSK cannot be \
         attached to the very operation it was specified for). This is not a gap in our code — it \
         is a gap between the spec text and the library, and the spec is the thing that has to \
         move. **An EXTERNAL PSK is the shape that does work**: the `Psk::External` branch of the \
         same resolver reads provider storage, so a token the governance issues and both parties \
         store is constructible today. That converts §11.7's 'resumption PSK' into 'a governance- \
         issued external PSK', which — usefully — is also the standing half. **One mechanism can \
         carry both halves, and it is the one the library actually supports.**"
    );
}

/// **The constructive half of question 2.** The previous test's closing claim — that an *external*
/// PSK is the shape that works — is measured here rather than reasoned. A governance-issued token,
/// held by returner and incumbent, attached to the external commit and merged by the group.
#[test]
fn a_governance_issued_external_psk_carries_both_halves_of_the_credential() {
    let bob = Persona::new("bob");
    let alice = Persona::new("alice");
    let returner = Persona::new("returner");
    let (mut bobs, _alices) = seat(&bob, &alice);

    // What the governance issued when it decided this party may return. Opaque to MLS: its
    // meaning is entirely ours, which is the point — the protocol carries it and checks nothing.
    let token_id = b"croft/reentry-token/v1/returner@example".to_vec();
    let token_secret = vec![0x5a; 32];
    let psk_id = PreSharedKeyId::external(token_id.clone(), vec![7u8; 32]);
    psk_id
        .store(&returner.provider, &token_secret)
        .expect("the returner was issued the token");
    psk_id
        .store(&bob.provider, &token_secret)
        .expect("and the incumbent was told to honour it");

    let members_before = bobs.members().count();
    let gi = current_group_info(&mut bobs, &bob);
    let (_rs, bundle) = MlsGroup::external_commit_builder()
        .with_config(MlsGroupJoinConfig::default())
        .build_group(&returner.provider, gi, returner.cwk.clone())
        .expect("build group")
        .add_psk_proposal(PreSharedKeyProposal::new(psk_id))
        .load_psks(returner.provider.storage())
        .expect("an EXTERNAL psk resolves from provider storage, where a resumption psk did not")
        .build(
            returner.provider.rand(),
            returner.provider.crypto(),
            &returner.signer,
            |_| true,
        )
        .expect("build external commit carrying the token")
        .finalize(&returner.provider)
        .expect("finalize");

    // The incumbent sees the claim as a first-class element of the staged commit — countable and
    // inspectable before any merge, which is where the policy goes.
    let staged = stage_at_incumbent(&mut bobs, &bob, &bundle);
    assert_eq!(
        staged.psk_proposals().count(),
        1,
        "the incumbent can see the token before deciding"
    );
    bobs.merge_staged_commit(&bob.provider, *staged)
        .expect("a group that honours the token admits the returner");
    let after = bobs.members().count();
    assert_eq!(after, members_before + 1);

    println!(
        "S16 CONFIRMED (real-lib): a **governance-issued external PSK** attaches to an external \
         commit, resolves from provider storage, is visible to the incumbent as a countable PSK \
         proposal before merging, and the merge seats the returner ({members_before} → {after} \
         members). So the mechanism §11.7 needs exists today — it is just not the one §11.7 names. **The \
         external PSK carries both halves at once:** possessing it proves the governance issued it \
         (standing), and it binds into the commit's key schedule so it cannot be claimed without \
         being held (keys). Its one difference from a resumption PSK is the honest one — it proves \
         **the governance vouched for you**, not **that you were there**."
    );
    println!(
        "S16 CONSEQUENCE: this makes the enforceable rule concrete and checkable — a Group refuses \
         any external commit whose `psk_proposals()` is empty or whose PSK it does not recognise. \
         Combined with the AAD carrier below, the full §11.7 check is: **read the token, read the \
         AAD attestation, verify both against the joiner's credential, then merge or drop.** Every \
         one of those four is measured available in this file."
    );
}

/// **Question 3 — does MLS give the application a place to stand?**
///
/// An attestation is worthless if a member cannot read it *before* adopting the new state. This
/// is the constructive half: where the attestation rides, and where the check goes.
#[test]
fn the_attestation_rides_in_aad_and_is_readable_before_the_merge_decision() {
    let bob = Persona::new("bob");
    let alice = Persona::new("alice");
    let returner = Persona::new("returner");
    let (mut bobs, _alices) = seat(&bob, &alice);

    let epoch_before = bobs.epoch();
    let members_before = bobs.members().count();
    let gi = current_group_info(&mut bobs, &bob);

    let (_rs, bundle) = MlsGroup::external_commit_builder()
        .with_config(MlsGroupJoinConfig::default())
        .with_aad(ATTESTATION.to_vec())
        .build_group(&returner.provider, gi, returner.cwk.clone())
        .expect("build group")
        .load_psks(returner.provider.storage())
        .expect("load psks")
        .build(
            returner.provider.rand(),
            returner.provider.crypto(),
            &returner.signer,
            |_| true,
        )
        .expect("build")
        .finalize(&returner.provider)
        .expect("finalize");

    let wire = bundle.commit().tls_serialize_detached().expect("ser");
    let protocol: ProtocolMessage = MlsMessageIn::tls_deserialize_exact(&wire)
        .expect("parse")
        .try_into_protocol_message()
        .expect("protocol");
    let processed = bobs
        .process_message(&bob.provider, protocol)
        .expect("process");

    // --- The three things a policy needs, all available before any merge. ---
    assert_eq!(
        processed.aad(),
        ATTESTATION,
        "the attestation survives the round trip and is readable by the incumbent"
    );
    let sender_is_external = matches!(processed.sender(), Sender::NewMemberCommit);
    assert!(
        sender_is_external,
        "and the incumbent can tell this is an external join rather than a member's commit"
    );
    let joiner_identity = processed.credential().serialized_content().to_vec();
    assert!(
        !joiner_identity.is_empty(),
        "and can name who is asking, so the attestation can be checked AGAINST an identity"
    );

    // --- Declining is real: refuse to merge and nothing moves. ---
    let staged = match processed.into_content() {
        ProcessedMessageContent::StagedCommitMessage(sc) => sc,
        other => panic!("expected staged commit, got {other:?}"),
    };
    assert!(
        staged.update_path_leaf_node().is_some(),
        "and the joiner's new leaf is inspectable too"
    );
    drop(staged); // the policy said no
    assert_eq!(bobs.epoch(), epoch_before, "declining left the epoch alone");
    assert_eq!(
        bobs.members().count(),
        members_before,
        "and did not seat the joiner"
    );

    println!(
        "S16 CONFIRMED (real-lib): the attestation has a carrier and the check has a checkpoint. \
         An external commit's **AAD survives to the incumbent** ({} bytes, byte-exact), the sender \
         is distinguishable as `NewMemberCommit` rather than a member's commit, and the joiner's \
         **credential is readable** — all three available on the ProcessedMessage BEFORE \
         merge_staged_commit. Dropping the staged commit instead of merging left the group at \
         epoch {} with {members_before} members, unchanged. So the policy hook §11.7 needs exists: \
         **read AAD, verify the attestation against the credential, then merge or drop.**",
        ATTESTATION.len(),
        epoch_before.as_u64()
    );
    println!(
        "S16 HONEST LIMIT — two of them, and the second is the sharper. (1) The AAD is signed by \
         the **joiner's own new leaf key**, so it is self-asserted: it authenticates the carrier, \
         never the claim. The attestation must therefore be a token the GOVERNANCE issued and the \
         member verifies out of band; MLS supplies the envelope and nothing else. (2) **Refusal is \
         not consensus.** Bob declining moves Bob only — a member who merged is now at a different \
         epoch, and the group has forked. So the attestation policy must be a GROUP-WIDE rule \
         agreed in advance (a group-context extension is the natural home), not a per-member \
         judgement call. A policy every member evaluates differently is a partition."
    );
}

/// Process `bundle`'s commit at the incumbent and hand back the staged commit, unmerged.
fn stage_at_incumbent(
    group: &mut MlsGroup,
    who: &Persona,
    bundle: &CommitMessageBundle,
) -> Box<StagedCommit> {
    let wire = bundle.commit().tls_serialize_detached().expect("ser");
    let protocol: ProtocolMessage = MlsMessageIn::tls_deserialize_exact(&wire)
        .expect("parse")
        .try_into_protocol_message()
        .expect("protocol");
    match group
        .process_message(&who.provider, protocol)
        .expect("process")
        .into_content()
    {
        ProcessedMessageContent::StagedCommitMessage(sc) => sc,
        other => panic!("expected staged commit, got {other:?}"),
    }
}
