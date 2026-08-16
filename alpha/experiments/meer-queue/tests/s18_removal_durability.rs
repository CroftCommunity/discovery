//! **S18 — how durable is a removal?**
//!
//! S16 measured that a party who was *never* a member can join a live group by external commit,
//! holding only a current `GroupInfo`. A **deliberately removed** member is the same mechanism
//! pointed at the case where getting it wrong hurts a real person: the group that removed someone
//! after a death, a divorce, or a falling-out needs "we removed them" to mean something.
//!
//! The owner's framing, which this test is built to check rather than to illustrate:
//!
//! > *Being in the group is a multi-tiered constraint, and who you key your responses for is the
//! > truest sense of your group. You can't be forced to do it.*
//!
//! So there are two questions, and the second matters more than the first:
//!
//! 1. **Can a removed member re-seat themselves?** (The alarming question.)
//! 2. **Does refusing to key for them actually hold?** (The one that decides whether it matters.)
//!
//! And one mitigation worth measuring rather than assuming: the ratchet-tree extension. With it
//! **off**, a `GroupInfo` alone may not be enough.
//!
//! Fidelity: **Rung A (real-lib)** — real OpenMLS, real CISS for the delivery-layer half.

use std::sync::Arc;

use meer_queue::ciss_harness::CissHarness;
use meer_queue::meer::{Meer, RecipientId};
use meer_queue::mls;
use mls_replant::{join, Persona};
use openmls::messages::group_info::VerifiableGroupInfo;
use openmls::prelude::*;
use tls_codec::{Deserialize as _, Serialize as _};

const QUEUE_LABEL: &str = "croft/meer-queue/v1";

fn queue_name(group: &MlsGroup, who: &Persona) -> RecipientId {
    RecipientId::new(hex::encode(
        group
            .export_secret(who.provider.crypto(), QUEUE_LABEL, &[], 32)
            .expect("export_secret"),
    ))
}

fn group_config(tree_ext: bool) -> MlsGroupCreateConfig {
    MlsGroupCreateConfig::builder()
        .ciphersuite(mls_replant::CS)
        .use_ratchet_tree_extension(tree_ext)
        .build()
}

/// Found a group of three: `founder` seats `a` and `b`.
fn seat_two(
    founder: &Persona,
    a: &Persona,
    b: &Persona,
    tree_ext: bool,
) -> (MlsGroup, MlsGroup, MlsGroup) {
    let mut f = MlsGroup::new(
        &founder.provider,
        &founder.signer,
        &group_config(tree_ext),
        founder.cwk.clone(),
    )
    .expect("create group");
    let (_c, welcome_out, _g) = f
        .add_members(
            &founder.provider,
            &founder.signer,
            &[a.key_package(), b.key_package()],
        )
        .expect("add both");
    f.merge_pending_commit(&founder.provider).expect("merge");
    let tree: RatchetTreeIn = f.export_ratchet_tree().into();
    let bytes = welcome_out.tls_serialize_detached().expect("ser");
    let extract = || match MlsMessageIn::tls_deserialize_exact(&bytes)
        .expect("de")
        .extract()
    {
        MlsMessageBodyIn::Welcome(w) => w,
        _ => panic!("expected a Welcome"),
    };
    let ga = join(a, extract(), tree.clone());
    let gb = join(b, extract(), tree);
    (f, ga, gb)
}

fn current_group_info(group: &mut MlsGroup, who: &Persona) -> VerifiableGroupInfo {
    group_info(group, who, true)
}

/// A `GroupInfo`, with the ratchet tree included or not.
///
/// **The `with_ratchet_tree` flag is independent of the group's `use_ratchet_tree_extension`
/// config** — a member can bundle the tree into an exported `GroupInfo` regardless. Getting this
/// wrong is how the first version of the mitigation test below measured nothing: it turned the
/// extension off in the group config and then handed the joiner the tree anyway.
fn group_info(group: &mut MlsGroup, who: &Persona, with_tree: bool) -> VerifiableGroupInfo {
    let bytes = group
        .export_group_info(who.provider.crypto(), &who.signer, with_tree)
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

/// Remove `victim`'s leaf from `group`, as `who`, and merge.
fn remove(group: &mut MlsGroup, who: &Persona, victim_leaf: LeafNodeIndex) {
    group
        .remove_members(&who.provider, &who.signer, &[victim_leaf])
        .expect("remove");
    group.merge_pending_commit(&who.provider).expect("merge");
}

/// **Question 1 — can a removed member walk back in?**
#[test]
fn a_removed_member_can_re_seat_themselves_with_a_current_groupinfo() {
    let alice = Persona::new("alice");
    let bob = Persona::new("bob");
    let carol = Persona::new("carol");
    let (mut alices, _bobs, carols) = seat_two(&alice, &bob, &carol, true);

    let carol_leaf = carols.own_leaf_index();
    let epoch_before_removal = alices.epoch().as_u64();
    remove(&mut alices, &alice, carol_leaf);
    let after_removal = alices.members().count();
    assert_eq!(after_removal, 2, "the group deliberately removed Carol");
    assert!(
        !alices.members().any(|m| m.index == carol_leaf),
        "and her leaf is gone from the group Alice holds"
    );

    // Carol obtains a current GroupInfo — the whole question is what that alone buys her.
    let gi = current_group_info(&mut alices, &alice);

    let rebuilt = MlsGroup::external_commit_builder()
        .with_config(MlsGroupJoinConfig::default())
        .build_group(&carol.provider, gi, carol.cwk.clone());

    let (carols_again, bundle) = match rebuilt {
        Ok(builder) => builder
            .load_psks(carol.provider.storage())
            .expect("load psks — she supplies none")
            .build(
                carol.provider.rand(),
                carol.provider.crypto(),
                &carol.signer,
                |_| true,
            )
            .expect("build external commit")
            .finalize(&carol.provider)
            .expect("finalize"),
        Err(e) => {
            println!(
                "S18 MEASURED (real-lib): a removed member was REFUSED at the external-commit \
                 boundary: {e}. Removal is durable against a GroupInfo alone."
            );
            return;
        }
    };

    // And Alice — who performed the removal — processes it without the library objecting.
    let wire = bundle.commit().tls_serialize_detached().expect("ser");
    let protocol: ProtocolMessage = MlsMessageIn::tls_deserialize_exact(&wire)
        .expect("parse")
        .try_into_protocol_message()
        .expect("protocol");
    let processed = alices
        .process_message(&alice.provider, protocol)
        .expect("process");
    assert!(
        matches!(processed.sender(), Sender::NewMemberCommit),
        "at least it is visibly an external join and not an ordinary commit"
    );
    match processed.into_content() {
        ProcessedMessageContent::StagedCommitMessage(sc) => {
            alices
                .merge_staged_commit(&alice.provider, *sc)
                .expect("merge");
        }
        other => panic!("expected staged commit, got {other:?}"),
    }

    assert_eq!(
        alices.members().count(),
        after_removal + 1,
        "the removed member is seated again"
    );
    assert_eq!(alices.epoch(), carols_again.epoch());

    println!(
        "S18 MEASURED (real-lib): **a deliberately removed member RE-SEATED herself** using a \
         current GroupInfo alone — no Welcome, no invitation, no PSK, and no member acting on her \
         behalf. Removed at epoch {epoch_before_removal} → {after_removal} members; back at epoch \
         {} → {} members. **A removal is exactly as durable as GroupInfo distribution, and not one \
         bit more.** [{}]",
        alices.epoch().as_u64(),
        alices.members().count(),
        mls::resolved_versions()
    );
    println!(
        "S18 CONSEQUENCE: this is the governance side door, measured. But note what it required and \
         what it did NOT require. It required a **current** GroupInfo — a document produced on \
         demand by someone still inside. It did not require anyone's consent. So the control is \
         entirely on the **distribution** of GroupInfo, which is the same lever E105 (the \
         readmission channel) is built on. **The re-entry channel and the removal side door are the \
         same mechanism.** Designing one designs the other, and they cannot be given different \
         answers."
    );
}

/// **Question 1b — what does "walked back in" actually MEAN?**
///
/// "Re-seated" is ambiguous in the way that matters most. Two very different readings:
///
/// - **Weak:** she can still emit messages from her old lineage, which current members may or may
///   not be able to read. Annoying, largely inert.
/// - **Strong:** she is a full member **at the current epoch** — she encrypts to everyone, everyone
///   decrypts her, and she decrypts them.
///
/// And one more that decides how bad the strong reading is: **does she get the history she missed?**
#[test]
fn the_re_seated_member_is_a_full_current_member_but_gets_no_history() {
    let alice = Persona::new("alice");
    let bob = Persona::new("bob");
    let carol = Persona::new("carol");
    let (mut alices, mut bobs, carols) = seat_two(&alice, &bob, &carol, true);

    // Remove Carol; Bob applies it too, so both remaining members agree.
    let carol_leaf = carols.own_leaf_index();
    let (removal, _w, _g) = alices
        .remove_members(&alice.provider, &alice.signer, &[carol_leaf])
        .expect("remove");
    alices.merge_pending_commit(&alice.provider).expect("merge");
    let removal_wire = removal.tls_serialize_detached().expect("ser");
    let protocol: ProtocolMessage = MlsMessageIn::tls_deserialize_exact(&removal_wire)
        .expect("parse")
        .try_into_protocol_message()
        .expect("protocol");
    match bobs
        .process_message(&bob.provider, protocol)
        .expect("bob processes removal")
        .into_content()
    {
        ProcessedMessageContent::StagedCommitMessage(sc) => {
            bobs.merge_staged_commit(&bob.provider, *sc).expect("merge")
        }
        other => panic!("unexpected {other:?}"),
    }

    // --- Traffic while she is OUT. This is the history she must not recover. ---
    let while_she_was_out =
        mls::seal(&mut alices, &alice, b"said while carol was removed").expect("seal");

    // --- She re-seats herself. ---
    let gi = current_group_info(&mut alices, &alice);
    let (mut carols_again, bundle) = MlsGroup::external_commit_builder()
        .with_config(MlsGroupJoinConfig::default())
        .build_group(&carol.provider, gi, carol.cwk.clone())
        .expect("build group")
        .load_psks(carol.provider.storage())
        .expect("load psks")
        .build(
            carol.provider.rand(),
            carol.provider.crypto(),
            &carol.signer,
            |_| true,
        )
        .expect("build")
        .finalize(&carol.provider)
        .expect("finalize");

    // Both remaining members accept it, so this is the "group did not object" case.
    let wire = bundle.commit().tls_serialize_detached().expect("ser");
    for (group, who) in [(&mut alices, &alice), (&mut bobs, &bob)] {
        let protocol: ProtocolMessage = MlsMessageIn::tls_deserialize_exact(&wire)
            .expect("parse")
            .try_into_protocol_message()
            .expect("protocol");
        match group
            .process_message(&who.provider, protocol)
            .expect("process")
            .into_content()
        {
            ProcessedMessageContent::StagedCommitMessage(sc) => group
                .merge_staged_commit(&who.provider, *sc)
                .expect("merge"),
            other => panic!("unexpected {other:?}"),
        }
    }

    // --- (a) Can she ENCRYPT at the current epoch, readable by the members? ---
    let hers = mls::seal(&mut carols_again, &carol, b"i am back and current").expect("seal");
    let alice_reads = mls::open(&mut alices, &alice, &hers).expect("alice reads carol");
    assert_eq!(alice_reads, b"i am back and current");

    // --- (b) Can she READ what a member sends now? ---
    let theirs = mls::seal(&mut bobs, &bob, b"sent after she returned").expect("seal");
    let carol_reads = mls::open(&mut carols_again, &carol, &theirs).expect("carol reads bob");
    assert_eq!(carol_reads, b"sent after she returned");

    // --- (c) Can she read what was said WHILE SHE WAS OUT? ---
    let history = mls::open(&mut carols_again, &carol, &while_she_was_out);
    let history_err = history.expect_err("the gap must NOT be recoverable");

    assert_eq!(alices.epoch(), carols_again.epoch());
    assert_eq!(alices.epoch(), bobs.epoch());

    println!(
        "S18 MEASURED (real-lib): **the strong reading is the correct one.** A re-seated member is a \
         FULL MEMBER AT THE CURRENT EPOCH, not a ghost emitting from a stale lineage: (a) she sealed \
         a message and a member read it in cleartext; (b) she read a member's message sealed after \
         her return; (c) all three sit at the same epoch. **This is not 'she can still shout \
         through the door' — she is in the room, keyed to everyone, from the moment the commit \
         merges.**"
    );
    println!(
        "S18 CONFIRMED (real-lib): **but she recovers NO history.** A message sealed while she was \
         removed is refused after re-entry — `{history_err}`. **Stated precisely, because the error \
         matters:** this is an EPOCH/ORDERING rejection, not an exercised decryption failure, and \
         it cannot be made into one — every pre-re-entry message is by construction at an older \
         epoch than the one her external commit created, so the ordering check always fires first. \
         The underlying reason she could not decrypt regardless is structural: an external commit \
         derives the NEW epoch's secrets and nothing earlier, so she never held the key that sealed \
         it. That is the same boundary S13 found for a first-time joiner. **Exposure is therefore \
         strictly forward-looking — re-entry costs everything said from that moment on and nothing \
         said before it** — which is a materially smaller harm than 'a removed party recovered the \
         conversation', and it is the honest thing to tell a user."
    );
}

/// **Question 2 — and the one that decides whether question 1 matters.**
///
/// The owner's claim: *who you key your responses for is the truest sense of your group, and you
/// can't be forced to do it.* Measured here, at both layers.
#[tokio::test]
async fn a_member_who_declines_keeps_the_returner_out_of_everything_it_sends() {
    let alice = Persona::new("alice");
    let bob = Persona::new("bob");
    let carol = Persona::new("carol");
    let (mut alices, mut bobs, carols) = seat_two(&alice, &bob, &carol, true);

    let ciss = Arc::new(CissHarness::spawn().await);
    let mut meer = Meer::new(Arc::clone(&ciss));

    // The group removes Carol; both remaining members apply it.
    let carol_leaf = carols.own_leaf_index();
    let (removal, _w, _g) = alices
        .remove_members(&alice.provider, &alice.signer, &[carol_leaf])
        .expect("remove");
    alices.merge_pending_commit(&alice.provider).expect("merge");
    let removal_wire = removal.tls_serialize_detached().expect("ser");
    let protocol: ProtocolMessage = MlsMessageIn::tls_deserialize_exact(&removal_wire)
        .expect("parse")
        .try_into_protocol_message()
        .expect("protocol");
    match bobs
        .process_message(&bob.provider, protocol)
        .expect("process removal")
        .into_content()
    {
        ProcessedMessageContent::StagedCommitMessage(sc) => {
            bobs.merge_staged_commit(&bob.provider, *sc).expect("merge")
        }
        other => panic!("expected staged commit, got {other:?}"),
    }

    // Carol re-seats herself off a GroupInfo.
    let gi = current_group_info(&mut alices, &alice);
    let (mut carols_again, bundle) = MlsGroup::external_commit_builder()
        .with_config(MlsGroupJoinConfig::default())
        .build_group(&carol.provider, gi, carol.cwk.clone())
        .expect("build group")
        .load_psks(carol.provider.storage())
        .expect("load psks")
        .build(
            carol.provider.rand(),
            carol.provider.crypto(),
            &carol.signer,
            |_| true,
        )
        .expect("build")
        .finalize(&carol.provider)
        .expect("finalize");

    let wire = bundle.commit().tls_serialize_detached().expect("ser");
    let as_protocol = || -> ProtocolMessage {
        MlsMessageIn::tls_deserialize_exact(&wire)
            .expect("parse")
            .try_into_protocol_message()
            .expect("protocol")
    };

    // --- Alice ACCEPTS. Bob DECLINES — he inspects and drops the staged commit. ---
    match alices
        .process_message(&alice.provider, as_protocol())
        .expect("alice processes")
        .into_content()
    {
        ProcessedMessageContent::StagedCommitMessage(sc) => alices
            .merge_staged_commit(&alice.provider, *sc)
            .expect("alice merges"),
        other => panic!("unexpected {other:?}"),
    }

    let bobs_verdict = bobs.process_message(&bob.provider, as_protocol());
    let bob_epoch_before = bobs.epoch();
    match bobs_verdict {
        Ok(processed) => {
            // Bob's policy says no. He drops it rather than merging.
            let staged = match processed.into_content() {
                ProcessedMessageContent::StagedCommitMessage(sc) => sc,
                other => panic!("unexpected {other:?}"),
            };
            drop(staged);
        }
        Err(e) => println!("S18 note: Bob's library refused outright rather than staging: {e}"),
    }
    assert_eq!(
        bobs.epoch(),
        bob_epoch_before,
        "declining left Bob exactly where he was"
    );

    // --- Layer 1: Bob's messages are not keyed for Carol, and she cannot read them. ---
    //
    // Bob first commits once on his own branch, so both sides sit at the SAME epoch NUMBER while
    // holding divergent secrets. Without this the refusal would surface as an epoch/generation
    // check — real, but a bookkeeping rejection rather than an exercised decryption failure, which
    // is the same weakness S7's negative had. This makes the fork prove itself cryptographically.
    mls_replant::commit(&mut bobs, &bob);
    assert_eq!(
        bobs.epoch(),
        carols_again.epoch(),
        "same epoch number, different branches — now a decryption test, not an ordering one"
    );
    let bobs_message = mls::seal(&mut bobs, &bob, b"said among those who stayed").expect("seal");
    let carol_attempt = mls::open(&mut carols_again, &carol, &bobs_message);
    let read_error = carol_attempt.expect_err("Carol must not be able to read Bob's traffic");

    // --- Layer 2: she cannot even FIND it. Bob's queue name is not one she can derive. ---
    let bobs_queue = queue_name(&bobs, &bob);
    let carols_queue = queue_name(&carols_again, &carol);
    assert_ne!(
        bobs_queue, carols_queue,
        "Bob's queue and Carol's are different names entirely"
    );
    meer.publish(&bobs_message, std::slice::from_ref(&bobs_queue))
        .await
        .expect("bob deposits");
    assert!(
        meer.drain(&carols_queue, &[])
            .await
            .expect("carol drains")
            .is_empty(),
        "Carol's drain returns nothing — Bob's mail is at an address she cannot name"
    );

    // --- And Alice, who accepted, is now on the other side of the fork from Bob. ---
    //
    // Compared by derived secret rather than by epoch number: after Bob's own commit both sit at
    // the same epoch NUMBER, which is precisely why a number is the wrong thing to compare. Two
    // branches can agree on the count and share nothing.
    assert_eq!(
        alices.epoch(),
        bobs.epoch(),
        "the fork is invisible in the epoch counter — both advanced once"
    );
    assert_ne!(
        queue_name(&alices, &alice),
        queue_name(&bobs, &bob),
        "but they derive different secrets: Alice and Bob are on different branches"
    );

    println!(
        "S18 CONFIRMED (real-lib): **refusal holds, at two independent layers.** Bob declined to \
         merge Carol's external commit and stayed at his epoch. (1) **Keys:** a message Bob sealed \
         afterwards is unreadable to Carol — `{read_error}`. (2) **Addressing:** Bob's queue name \
         is not one Carol can derive, so his mail sits at an address she cannot even ask for; her \
         drain returns empty. **The owner's framing is literally true and mechanically enforced: \
         who you key for IS your group, and nobody can force you to key for someone.** Neither \
         layer needs the meer's cooperation, and neither needs Carol's."
    );
    println!(
        "S18 CONSEQUENCE — and this is the real design problem: **the cost of refusing is a fork, \
         and the fork is INVISIBLE in the epoch counter.** Alice accepted and Bob declined; after \
         each advanced once they sit at the SAME epoch number while deriving DIFFERENT secrets and \
         different queue names. So a client cannot detect this by comparing epochs — the only \
         symptom is that peers stop being able to read each other. Carol did not split the group; \
         the DISAGREEMENT about Carol split the group. The mechanism is sound and the **UX is the \
         whole problem**: members must reach the same answer in advance, without a negotiation \
         round — which is what makes the readmission rule a **group-context policy** rather than a \
         per-member prompt. **A dialog box asking each member 'allow Carol back?' is a partition \
         generator**, and one that hides its own damage."
    );

    ciss.shutdown().await;
}

/// **The mitigation, measured rather than assumed.** A `GroupInfo` is self-sufficient only if it
/// carries the ratchet tree. Withhold the tree and the door should narrow from "anyone with a
/// GroupInfo" to "anyone with a GroupInfo **and** the tree".
#[test]
fn a_groupinfo_without_the_ratchet_tree_is_not_enough_to_re_seat() {
    let alice = Persona::new("alice");
    let bob = Persona::new("bob");
    let carol = Persona::new("carol");
    let (mut alices, _bobs, carols) = seat_two(&alice, &bob, &carol, false);

    let carol_leaf = carols.own_leaf_index();
    remove(&mut alices, &alice, carol_leaf);

    // A GroupInfo with NO ratchet tree bundled — the actual dial, as opposed to the group's
    // `use_ratchet_tree_extension` config, which does not control what an exporter includes.
    let bare = group_info(&mut alices, &alice, false);
    let without_tree = MlsGroup::external_commit_builder()
        .with_config(MlsGroupJoinConfig::default())
        .build_group(&carol.provider, bare, carol.cwk.clone());

    let refusal = match without_tree {
        Err(e) => e,
        Ok(_) => panic!(
            "a GroupInfo carrying no ratchet tree was expected to be insufficient — it was not"
        ),
    };

    // …and the same GroupInfo, with the tree bundled, lets her straight back in. Same removal,
    // same member, same epoch: the ONLY variable is whether the tree travelled.
    let with_tree = group_info(&mut alices, &alice, true);
    let admitted = MlsGroup::external_commit_builder()
        .with_config(MlsGroupJoinConfig::default())
        .build_group(&carol.provider, with_tree, carol.cwk.clone());
    assert!(
        admitted.is_ok(),
        "the control arm must succeed, or the comparison proves nothing"
    );

    println!(
        "S18 CONFIRMED (real-lib): a removed member holding a `GroupInfo` **without** the ratchet \
         tree is refused — `{refusal}` — while the SAME member, SAME removal and SAME epoch is \
         admitted the moment the tree is bundled. **The admission surface is the ratchet tree, not \
         the GroupInfo.** So there is a genuine governance dial: withhold the tree and re-entry \
         needs two separately-distributed artifacts instead of one."
    );
    println!(
        "S18 CONSEQUENCE: the dial is cheap and points the same way as an argument we already had. \
         S8 measured that the tree extension roughly DOUBLES `Welcome` size (330 vs 152 bytes per \
         member) and is first to cross the 2 MiB object cap, at N ≈ 6,350. **Bandwidth and \
         governance want the same thing here** — ship the tree deliberately and narrowly rather \
         than bundling it by default. Note the flag is independent of the group's \
         `use_ratchet_tree_extension` config: an exporter chooses per call, so this has to be \
         enforced at whatever serves GroupInfo (E105), not in the group's configuration."
    );
}
