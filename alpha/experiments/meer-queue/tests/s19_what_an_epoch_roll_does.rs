//! **S19 — what does an epoch roll actually do, and what does it actually prevent?**
//!
//! The working model behind most of the planning, stated by the owner (2026-08-16):
//!
//! > *An epoch roll is the literal changing of the group encryption material, so a user left out of
//! > an epoch roll would have no way in cryptographically.*
//!
//! **The first half of that is exactly right, and this file confirms it.** The second half is where
//! S18's result comes from, and the difference is not a subtlety — it is two different doors:
//!
//! - **Derivation.** Each commit mixes new path entropy with the prior epoch's `init_secret` to
//!   produce the next epoch's secrets. A removed member holds neither the new path entropy nor a
//!   leaf on the re-keyed path, so it **cannot compute epoch N+1 from anything it has.** That is
//!   forward secrecy and post-compromise security doing their job.
//! - **External join.** RFC 9420's external commit does **not** derive the new epoch from prior
//!   state at all. The joiner performs a KEM against the **`external_pub`** key published in the
//!   `GroupInfo` to obtain the current epoch's `init_secret`, then commits itself in. **Prior
//!   membership contributes nothing to this path — and so losing it prevents nothing.**
//!
//! So "locked out" is true of **reading forward** and false of **getting back in**, because getting
//! back in was never a derivation in the first place. This file measures each half separately so the
//! two are not conflated again.
//!
//! Fidelity: **Rung A (real-lib)** — real OpenMLS.

use meer_queue::mls;
use mls_replant::{join, Persona};
use openmls::messages::group_info::VerifiableGroupInfo;
use openmls::prelude::*;
use tls_codec::{Deserialize as _, Serialize as _};

const QUEUE_LABEL: &str = "croft/meer-queue/v1";

fn queue_name(group: &MlsGroup, who: &Persona) -> String {
    hex::encode(
        group
            .export_secret(who.provider.crypto(), QUEUE_LABEL, &[], 32)
            .expect("export_secret"),
    )
}

fn group_config() -> MlsGroupCreateConfig {
    MlsGroupCreateConfig::builder()
        .ciphersuite(mls_replant::CS)
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

/// **Half 1 — the working model, confirmed.** An epoch roll really does lock a removed member out
/// of everything derived forward.
#[test]
fn an_epoch_roll_locks_a_removed_member_out_of_everything_derived_forward() {
    let alice = Persona::new("alice");
    let carol = Persona::new("carol");
    let (mut alices, mut carols) = seat(&alice, &carol);

    // While she is in, she is fully current: same queue name, reads Alice's traffic.
    assert_eq!(queue_name(&alices, &alice), queue_name(&carols, &carol));
    let while_in = mls::seal(&mut alices, &alice, b"while carol was a member").expect("seal");
    assert_eq!(
        mls::open(&mut carols, &carol, &while_in).expect("reads while in"),
        b"while carol was a member"
    );

    // --- The epoch roll: Alice removes her. ---
    let carol_leaf = carols.own_leaf_index();
    let (removal, _w, _g) = alices
        .remove_members(&alice.provider, &alice.signer, &[carol_leaf])
        .expect("remove");
    alices.merge_pending_commit(&alice.provider).expect("merge");

    // What does the removed member see if handed the very commit that removed her?
    let removal_wire = removal.tls_serialize_detached().expect("ser");
    let protocol: ProtocolMessage = MlsMessageIn::tls_deserialize_exact(&removal_wire)
        .expect("parse")
        .try_into_protocol_message()
        .expect("protocol");
    let saw_own_removal = carols.process_message(&carol.provider, protocol);
    let removal_observation = match saw_own_removal {
        Ok(_) => "she CAN process the commit that removes her (it is addressed to the epoch she \
                  still holds), learning that she was removed"
            .to_string(),
        Err(e) => format!("the library refused to let her process it: {e}"),
    };

    // --- Everything after the roll is beyond her, three ways. ---
    //
    // Measured at the strongest available grade. A removed member is simply BEHIND, so the naive
    // form of this test gets `Message epoch differs from the group's epoch` — a bookkeeping
    // rejection that would hold even for a member who merely lagged. To make the refusal
    // genuinely cryptographic, Carol advances her own stale view by one commit first, so both
    // sides sit at the SAME epoch NUMBER on different branches. Only then does a failure to read
    // mean "she cannot derive the key" rather than "she is at the wrong count".
    let after = mls::seal(&mut alices, &alice, b"said after the roll").expect("seal");
    let behind_err =
        mls::open(&mut carols, &carol, &after).expect_err("she cannot read post-roll traffic");

    mls_replant::commit(&mut carols, &carol);
    assert_eq!(
        alices.epoch(),
        carols.epoch(),
        "same epoch NUMBER now — this is a key test, not a counter test"
    );
    let read_attempt = mls::open(&mut carols, &carol, &after);
    let read_err = read_attempt.expect_err("she MUST NOT be able to read post-roll traffic");

    assert_ne!(
        queue_name(&alices, &alice),
        queue_name(&carols, &carol),
        "she cannot derive the new queue name either — she cannot even FIND the mail"
    );

    println!(
        "S19 CONFIRMED (real-lib): **the working model is correct.** An epoch roll changes the group's \
         key material, and a removed member is locked out of everything derived forward: she cannot \
         read a message sealed after the roll and cannot derive the new queue name. Measured at two \
         grades, because the weaker one would not have proved it: while merely behind she is \
         refused on a COUNTER check (`{behind_err}`), which a lagging member would also hit; after \
         advancing her own stale branch to the SAME epoch number she is refused on the KEY — \
         `{read_err}`. Each commit mixes NEW path entropy with the prior epoch's secret, and she \
         holds neither the entropy nor a leaf on the re-keyed path. **Nothing she possesses \
         computes the new epoch.** [{}]",
        mls::resolved_versions()
    );
    println!("S19 NOTE (real-lib): on being handed her own removal commit — {removal_observation}");
}

/// **Half 2 — and why that does not make her unable to get back in.**
///
/// The external-join path does not derive the new epoch from prior state, so **holding no prior
/// state costs nothing.** Measured the sharpest available way: a returner using a **completely
/// fresh** provider — no stored group state, no history, nothing carried over from her membership.
#[test]
fn external_join_derives_from_the_published_key_not_from_prior_membership() {
    let alice = Persona::new("alice");
    let carol = Persona::new("carol");
    let (mut alices, carols) = seat(&alice, &carol);

    let carol_leaf = carols.own_leaf_index();
    alices
        .remove_members(&alice.provider, &alice.signer, &[carol_leaf])
        .expect("remove");
    alices.merge_pending_commit(&alice.provider).expect("merge");
    for _ in 0..3 {
        mls_replant::commit(&mut alices, &alice);
    }
    let members_before = alices.members().count();

    // **A brand-new persona: fresh provider, fresh store, zero prior group state.** If prior
    // membership were load-bearing for the re-entry path, this could not work at all.
    let stateless = Persona::new("carol-on-a-new-device");
    let gi = group_info(&mut alices, &alice, true);

    let (rejoined, bundle) = MlsGroup::external_commit_builder()
        .with_config(MlsGroupJoinConfig::default())
        .build_group(&stateless.provider, gi, stateless.cwk.clone())
        .expect("built current group state from the GroupInfo's published key alone")
        .load_psks(stateless.provider.storage())
        .expect("no psks — none are consulted on this path")
        .build(
            stateless.provider.rand(),
            stateless.provider.crypto(),
            &stateless.signer,
            |_| true,
        )
        .expect("build external commit")
        .finalize(&stateless.provider)
        .expect("finalize");

    let wire = bundle.commit().tls_serialize_detached().expect("ser");
    let protocol: ProtocolMessage = MlsMessageIn::tls_deserialize_exact(&wire)
        .expect("parse")
        .try_into_protocol_message()
        .expect("protocol");
    match alices
        .process_message(&alice.provider, protocol)
        .expect("process")
        .into_content()
    {
        ProcessedMessageContent::StagedCommitMessage(sc) => alices
            .merge_staged_commit(&alice.provider, *sc)
            .expect("merge"),
        other => panic!("unexpected {other:?}"),
    }
    assert_eq!(alices.members().count(), members_before + 1);
    assert_eq!(alices.epoch(), rejoined.epoch());

    println!(
        "S19 MEASURED (real-lib): a party with a **completely fresh provider — no stored group \
         state, no prior epoch secrets, nothing carried over** — joined the live group and the \
         incumbent merged it ({members_before} → {} members). **Prior membership contributes \
         NOTHING to the external-join path**, which is why losing it prevents nothing. The joiner \
         does a KEM against the `external_pub` key published IN THE GROUPINFO to obtain the current \
         epoch's init_secret, then commits itself in. **This is a second door, not a bypass of the \
         first one:** the epoch roll's lock is on DERIVATION, and this path never derives.",
        alices.members().count()
    );
    println!(
        "S19 CONSEQUENCE: the two facts are consistent and both are true — (1) an epoch roll DOES \
         cryptographically exclude a removed member from everything forward, and (2) it does NOT \
         prevent re-entry. **Exclusion is passive-reading exclusion; re-entry is an active \
         protocol operation gated on a published key, not on anything the roll destroyed.** Any \
         planning that reads (1) as implying (2) is reading a guarantee the protocol does not make \
         — which is precisely why Part 2 §11.8 puts ban enforcement in an application-layer \
         admission gate rather than relying on the key layer."
    );
}

/// **The dial, checked at its root.** If `external_pub` is what admits an external joiner, can a
/// member simply export a `GroupInfo` without it?
#[test]
fn there_is_no_way_to_export_a_groupinfo_without_the_external_join_key() {
    let alice = Persona::new("alice");
    let carol = Persona::new("carol");
    let (mut alices, _carols) = seat(&alice, &carol);
    let stranger = Persona::new("stranger");

    // The only knob `export_group_info` offers is the ratchet tree. Both settings are exported and
    // handed to a stranger; the question is whether either withholds the external-join key.
    for with_tree in [true, false] {
        let gi = group_info(&mut alices, &alice, with_tree);
        let attempt = MlsGroup::external_commit_builder()
            .with_config(MlsGroupJoinConfig::default())
            .build_group(&stranger.provider, gi, stranger.cwk.clone());

        match (with_tree, attempt) {
            (true, Ok(_)) => {}
            (true, Err(e)) => panic!("with the tree bundled a stranger should get in, got {e}"),
            (false, Err(e)) => {
                // The refusal must be about the TREE, never about the external-join key — that is
                // the whole point of this test.
                let msg = e.to_string();
                assert!(
                    msg.contains("ratchet tree") || msg.contains("tree"),
                    "the refusal must be the missing TREE, not a missing external_pub: {msg}"
                );
            }
            (false, Ok(_)) => panic!("a tree-less GroupInfo should not suffice"),
        }
    }

    println!(
        "S19 CONFIRMED (real-lib): **there is no export flag that withholds the external-join key.** \
         `export_group_info` takes exactly one option — `with_ratchet_tree` — and with the tree \
         bundled a stranger gets in, while without it the refusal is specifically about the missing \
         **tree**, never about a missing `external_pub`. (`export_group_info_with_additional_\
         extensions` documents that it *errors* if a `RatchetTreeExtension` or `ExternalPubExtension` \
         is supplied directly, so neither can be hand-managed either.) **Every GroupInfo a member \
         can produce carries the external-join key.**"
    );
    println!(
        "S19 CONSEQUENCE: this closes the question S18 left open about WHICH dial to reach for. \
         There is no such thing as a 'safe' GroupInfo — one that proves current group state without \
         also admitting its holder. So the admission surface cannot be narrowed by exporting a \
         weaker GroupInfo; it can only be narrowed by **withholding the ratchet tree** (S18) and by \
         **controlling who is handed a GroupInfo at all**. Both are policies at the serving node, \
         which is why E105's `GroupInfo` channel and E107's removal durability are one decision."
    );
}
