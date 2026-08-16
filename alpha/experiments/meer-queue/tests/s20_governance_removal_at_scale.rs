//! **S20 — a governance ban at group scale: is the excluded member actually excluded?**
//!
//! The owner's methodological challenge (2026-08-16), which is the right one to raise:
//!
//! > *I worry our testing right now is "roll epoch and equally include all existing users including
//! > the ban prospect", which would definitely fail the test. We need a way to say 10 people, 1 is
//! > banned by legit group governance, epoch roll, only include non-banned folks in new group.*
//!
//! S19 used a two-person group and a `remove_members` commit, which **is** a genuine MLS removal —
//! but a two-person group cannot distinguish "the removal excluded her" from "the group is now one
//! person and trivially disagrees with her". **At N = 10 the distinction is visible:** if the roll
//! genuinely excludes one member, the other **nine must all agree** on the new key material while
//! the tenth is alone in disagreeing. That is the test this file runs.
//!
//! It then measures the thing the exclusion does *not* settle: **when the admission gate fires, and
//! what the propagation window actually costs.**
//!
//! Fidelity: **Rung A (real-lib)** — real OpenMLS.

use meer_queue::mls;
use mls_replant::{join, Persona};
use openmls::messages::group_info::VerifiableGroupInfo;
use openmls::prelude::*;
use tls_codec::{Deserialize as _, Serialize as _};

const QUEUE_LABEL: &str = "croft/meer-queue/v1";
const N: usize = 10;

/// The queue name, or `None` if the group object can no longer derive one.
///
/// **Fallible on purpose.** A member who processes the commit that evicts them has their group
/// object marked inactive by OpenMLS, and every later use — including `export_secret` — fails with
/// `UseAfterEviction`. That is a measurement, not an inconvenience, so it is surfaced rather than
/// unwrapped.
fn queue_name_opt(group: &MlsGroup, who: &Persona) -> Option<String> {
    group
        .export_secret(who.provider.crypto(), QUEUE_LABEL, &[], 32)
        .ok()
        .map(hex::encode)
}

fn queue_name(group: &MlsGroup, who: &Persona) -> String {
    queue_name_opt(group, who).expect("this member must still be able to derive")
}

fn group_config() -> MlsGroupCreateConfig {
    MlsGroupCreateConfig::builder()
        .ciphersuite(mls_replant::CS)
        .use_ratchet_tree_extension(true)
        .build()
}

fn group_info(group: &mut MlsGroup, who: &Persona) -> VerifiableGroupInfo {
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

/// Apply `commit` at `group` as `who`. Returns whether it applied.
fn apply(group: &mut MlsGroup, who: &Persona, commit: &[u8]) -> bool {
    let protocol: ProtocolMessage = match MlsMessageIn::tls_deserialize_exact(commit)
        .expect("parse")
        .try_into_protocol_message()
    {
        Ok(p) => p,
        Err(_) => return false,
    };
    match group.process_message(&who.provider, protocol) {
        Ok(processed) => match processed.into_content() {
            ProcessedMessageContent::StagedCommitMessage(sc) => {
                group.merge_staged_commit(&who.provider, *sc).is_ok()
            }
            _ => false,
        },
        Err(_) => false,
    }
}

/// Found a group of `N`: a founder plus `N - 1` others, all seated from one commit.
fn found_group_of_n() -> (Vec<Persona>, Vec<MlsGroup>) {
    let founder = Persona::new("m0");
    let others: Vec<Persona> = (1..N).map(|i| Persona::new(&format!("m{i}"))).collect();

    let mut f = MlsGroup::new(
        &founder.provider,
        &founder.signer,
        &group_config(),
        founder.cwk.clone(),
    )
    .expect("create group");
    let kps: Vec<KeyPackage> = others.iter().map(Persona::key_package).collect();
    let (_c, welcome_out, _g) = f
        .add_members(&founder.provider, &founder.signer, &kps)
        .expect("add all");
    f.merge_pending_commit(&founder.provider).expect("merge");

    let tree: RatchetTreeIn = f.export_ratchet_tree().into();
    let welcome_bytes = welcome_out.tls_serialize_detached().expect("ser");

    let mut personas = vec![founder];
    let mut groups = vec![f];
    for p in others {
        let welcome = match MlsMessageIn::tls_deserialize_exact(&welcome_bytes)
            .expect("de")
            .extract()
        {
            MlsMessageBodyIn::Welcome(w) => w,
            _ => panic!("expected a Welcome"),
        };
        groups.push(join(&p, welcome, tree.clone()));
        personas.push(p);
    }
    assert_eq!(groups.len(), N);
    (personas, groups)
}

/// **The owner's scenario, literally.** Ten people, one banned by governance, one epoch roll, and
/// only the nine in the new key material.
#[test]
fn a_governance_ban_at_n_ten_excludes_exactly_one_and_the_other_nine_agree() {
    let (personas, mut groups) = found_group_of_n();

    // Everyone starts genuinely current: all ten derive the SAME queue name. Without this the
    // later disagreement would prove nothing.
    let start: Vec<String> = groups
        .iter()
        .zip(&personas)
        .map(|(g, p)| queue_name(g, p))
        .collect();
    assert!(
        start.windows(2).all(|w| w[0] == w[1]),
        "all {N} members must agree BEFORE the ban, or the test is measuring noise"
    );

    // --- Governance bans m7. The enactment is a removal commit: an epoch roll that re-keys the
    //     path so the banned leaf is not in the new key material. ---
    const BANNED: usize = 7;
    let banned_leaf = groups[BANNED].own_leaf_index();
    let (removal, _w, _g) = groups[0]
        .remove_members(&personas[0].provider, &personas[0].signer, &[banned_leaf])
        .expect("governance removal");
    groups[0]
        .merge_pending_commit(&personas[0].provider)
        .expect("merge");
    let removal_wire = removal.tls_serialize_detached().expect("ser");

    // Every OTHER member applies it. The banned member deliberately does not yet — her own
    // handling of it is measured separately below, because it turns out to be a distinct state.
    let mut applied_by = Vec::new();
    for i in 1..N {
        if i != BANNED && apply(&mut groups[i], &personas[i], &removal_wire) {
            applied_by.push(i);
        }
    }

    // --- The claim under test: NINE agree, ONE is alone. ---
    let after: Vec<String> = groups
        .iter()
        .zip(&personas)
        .map(|(g, p)| queue_name(g, p))
        .collect();
    let survivors: Vec<usize> = (0..N).filter(|i| *i != BANNED).collect();
    let survivor_key = &after[survivors[0]];

    for &i in &survivors {
        assert_eq!(
            &after[i], survivor_key,
            "member m{i} must share the post-ban key material with every other survivor"
        );
    }
    assert_ne!(
        &after[BANNED], survivor_key,
        "the BANNED member must NOT hold the new key material"
    );
    assert_eq!(
        after[BANNED], start[BANNED],
        "and she is stranded on the pre-ban key, having derived nothing new"
    );
    assert_eq!(
        groups[0].members().count(),
        N - 1,
        "the roster lost exactly one"
    );

    // --- And the exclusion is cryptographic, not bookkeeping. ---
    // Advance the banned member's own stale branch so both sides sit at the SAME epoch number;
    // only then does a read failure mean "cannot derive the key" rather than "wrong count".
    let after_ban = mls::seal(&mut groups[0], &personas[0], b"said after the ban").expect("seal");
    mls_replant::commit(&mut groups[BANNED], &personas[BANNED]);
    assert_eq!(
        groups[0].epoch(),
        groups[BANNED].epoch(),
        "same epoch NUMBER — this is now a key test, not a counter test"
    );
    let refused = mls::open(&mut groups[BANNED], &personas[BANNED], &after_ban)
        .expect_err("the banned member MUST NOT read post-ban traffic");

    println!(
        "S20 CONFIRMED (real-lib): **the methodological worry is answered — the roll genuinely \
         excludes.** At N = {N}, all {N} members agreed on the queue name before the ban; after one \
         governance removal commit, the **nine survivors all derive the SAME new key material** and \
         the **banned member derives none of it** — she is stranded on the pre-ban key, unchanged. \
         The roster went to {} members. This is not 'roll and include everyone': exactly one leaf \
         was excluded from the re-keyed path, and it is the intended one. The exclusion is \
         cryptographic, measured at the strong grade — with both sides advanced to the same epoch \
         NUMBER she is refused on the KEY: `{refused}`. [{}]",
        groups[0].members().count(),
        mls::resolved_versions()
    );
    println!(
        "S20 NOTE: {} of the {} other members applied the removal commit and all agree. The banned \
         member is entitled to process it too — it is addressed to the epoch she still holds — and \
         doing so is what tells her she was banned. **That is the ceiling: she reads up to and \
         including her own banning, and nothing after it.** What that costs her is measured \
         separately below.",
        applied_by.len(),
        N - 2
    );
}

/// **And what does it cost her to ACCEPT the evidence of her own ban?**
///
/// Split into its own test with a clean sequence: a member can process a given commit once, and the
/// strong-grade test above deliberately forks her branch first, which would make the removal commit
/// inapplicable for reasons that have nothing to do with eviction.
#[test]
fn processing_her_own_removal_leaves_a_dead_group_object_not_a_stale_one() {
    let (personas, mut groups) = found_group_of_n();
    const BANNED: usize = 7;

    let before = queue_name(&groups[BANNED], &personas[BANNED]);
    let banned_leaf = groups[BANNED].own_leaf_index();
    let (removal, _w, _g) = groups[0]
        .remove_members(&personas[0].provider, &personas[0].signer, &[banned_leaf])
        .expect("governance removal");
    groups[0]
        .merge_pending_commit(&personas[0].provider)
        .expect("merge");
    let removal_wire = removal.tls_serialize_detached().expect("ser");

    // She processes the very commit that removes her — nothing else has happened to her state.
    let applied = apply(&mut groups[BANNED], &personas[BANNED], &removal_wire);
    let after = queue_name_opt(&groups[BANNED], &personas[BANNED]);

    assert!(
        applied,
        "she can process the commit that removes her — it is addressed to the epoch she holds"
    );
    assert!(
        after.is_none(),
        "and afterwards her group object must be unusable, not merely stale (was {before:.16}...)"
    );

    println!(
        "S20 MEASURED (real-lib): accepting the evidence of her own ban leaves her **worse off than \
         ignoring it, and in a qualitatively different state.** She processed the removal commit \
         successfully — that is how she LEARNS she was banned — and afterwards her group object \
         cannot derive a queue name at all: OpenMLS marks an evicted member's group INACTIVE and \
         fails every later use with `UseAfterEviction`. **So there are three distinct post-ban \
         states, not two:** (1) she never sees the removal — live object, stale key, excluded from \
         everything forward; (2) she processes it — **dead object, no key derivation at all**; (3) \
         she rebuilds from a GroupInfo — fresh object, current keys. **Nothing carries from (1) or \
         (2) into (3)**, which is the mechanical reason the ban's key-layer work cannot influence \
         the re-entry path at all."
    );
}

/// **So what is the admission gate, and when does it fire?**
///
/// The exclusion above is total. What it does not settle is re-entry — and the shape of that is not
/// "she requests key material and someone grants it". This measures what it actually is.
#[test]
fn re_entry_is_self_admission_not_a_request_and_the_window_is_a_lagging_member() {
    let (personas, mut groups) = found_group_of_n();
    const BANNED: usize = 7;

    let banned_leaf = groups[BANNED].own_leaf_index();
    let (removal, _w, _g) = groups[0]
        .remove_members(&personas[0].provider, &personas[0].signer, &[banned_leaf])
        .expect("removal");
    groups[0]
        .merge_pending_commit(&personas[0].provider)
        .expect("merge");
    let removal_wire = removal.tls_serialize_detached().expect("ser");

    // **m5 lags.** Everyone else syncs the ban; m5 has not yet. This is §11.8's eventually-
    // consistent propagation, made concrete as one member behind by one commit.
    const LAGGING: usize = 5;
    for i in 1..N {
        if i != LAGGING && i != BANNED {
            apply(&mut groups[i], &personas[i], &removal_wire);
        }
    }
    assert_eq!(
        queue_name(&groups[LAGGING], &personas[LAGGING]),
        queue_name(&groups[BANNED], &personas[BANNED]),
        "the lagging member is still on the PRE-ban key, same as the banned member"
    );

    // --- The banned member does NOT ask anyone for anything. ---
    // She obtains a GroupInfo from the lagging member and unilaterally constructs a commit that
    // seats her. There is no request, and therefore nothing for a member to deny.
    let gi = group_info(&mut groups[LAGGING], &personas[LAGGING]);
    let (_hers, bundle) = MlsGroup::external_commit_builder()
        .with_config(MlsGroupJoinConfig::default())
        .build_group(&personas[BANNED].provider, gi, personas[BANNED].cwk.clone())
        .expect("she builds group state herself — nobody is consulted")
        .load_psks(personas[BANNED].provider.storage())
        .expect("no psks")
        .build(
            personas[BANNED].provider.rand(),
            personas[BANNED].provider.crypto(),
            &personas[BANNED].signer,
            |_| true,
        )
        .expect("build")
        .finalize(&personas[BANNED].provider)
        .expect("finalize");
    let re_entry = bundle.commit().tls_serialize_detached().expect("ser");

    // --- The gate fires at MERGE, and only for members who can see the ban. ---
    let lagging_admitted = apply(&mut groups[LAGGING], &personas[LAGGING], &re_entry);
    let current_admitted = apply(&mut groups[0], &personas[0], &re_entry);

    assert!(
        lagging_admitted,
        "the member who has not synced the ban admits her — this is the propagation window"
    );
    assert!(
        !current_admitted,
        "a member already past the ban cannot even apply it: her commit is built on a superseded \
         epoch"
    );

    println!(
        "S20 MEASURED (real-lib): **re-entry is SELF-ADMISSION, not a request.** The banned member \
         asked nobody for key material: she took a `GroupInfo` from a member who had not yet synced \
         the ban, derived the current epoch's init_secret from the `external_pub` published inside \
         it, and constructed a commit seating herself. **There is no request, so there is nothing \
         for a member to deny** — which is why the gate cannot be a permission prompt."
    );
    println!(
        "S20 CONFIRMED (real-lib): **and the window is exactly the lagging member.** Her re-entry \
         commit was admitted by the member who had not synced the ban, and was NOT applicable at a \
         member already past it — the commit is built on an epoch that member has superseded, so it \
         is refused without any policy being consulted. **So the exposure window is not 'anyone can \
         let her back in'. It is precisely: the set of members whose view predates the ban.** That \
         is the same eventually-consistent propagation §11.8 already owns, and it means the gate has \
         TWO enforcement points, not one: (1) **who is served a GroupInfo** — a member who has \
         synced the ban must refuse to serve one to a banned lineage, which is the cheap and \
         effective control; and (2) **merge-time policy** at members who are current, which is a \
         backstop rather than the primary defence."
    );
    println!(
        "S20 CONSEQUENCE: this sharpens the timing question. **The gate is not at the moment of the \
         ban and not at the moment of re-entry — it is at the moment a GroupInfo is SERVED.** The \
         ban's enactment (the removal commit) is instantaneous and total for key material; the ban's \
         *enforcement* against re-entry is only as fast as the ban reaches whoever is willing to \
         hand out a GroupInfo. **This is the concrete form of E105/E107 being one decision**, and it \
         argues the GroupInfo server must resolve standing at head (§11.8) before serving, not \
         merely relay."
    );
}
