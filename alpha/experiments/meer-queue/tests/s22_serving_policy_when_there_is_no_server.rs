//! **S22 — the readmission serving policy, when there is no server to put it in.**
//!
//! S19/S20 established that the readmission gate's effective location is **the moment a `GroupInfo`
//! is served**, not the moment of re-entry — because re-entry is self-admission and there is nothing
//! to deny. S20 also established that the dial's position 1 was **a label, not a thing**: no
//! component in the corpus resolves standing, so the delivery design was sitting at position 0 while
//! describing itself as stronger.
//!
//! > **CORRECTED 2026-08-16 (owner), before this file was committed.** It was first written as
//! > `s22_position_one_standing_checked_server.rs`, gating at a **history-convergence node**. **Part 1
//! > §2.4 forbids that reading**: *"a Group MUST NOT structurally depend on any single persona's
//! > presence to act"*, the no-helper path MUST stay real, a meer is optional and everything else is
//! > distributed. **There is no chokepoint.** Every member holds live group state and can export a
//! > `GroupInfo`, so the policy is one **every peer applies**, and the last test here measures what
//! > that costs.
//!
//! This builds the smallest real version of position 1 and measures whether it does what the dial
//! claims. Four questions:
//!
//! 1. **Does refusing to serve actually stop a banned lineage?** S18 measured that a removed member
//!    re-seats herself *given* a `GroupInfo`. If serving is the gate, withholding must be the
//!    difference.
//! 2. **Does the graceful path survive the gate?** A dormant member in good standing must still get
//!    the immediate self-service return §11.7 promises. A gate that blocks both is not position 1;
//!    it is position 3 with extra steps.
//! 3. **Where is the residual?** A peer can hold current *group* state with a stale *standing* view,
//!    because the two propagate on different chains (§11.8).
//! 4. **How wide is that, given every member is a serving peer?** This is the question the
//!    server framing hid, and it is the one that decides which dial position is actually robust.
//!
//! Fidelity: **Rung A (real-lib)** for every MLS operation. The standing chain is a
//! `SPEC-DELTA[groupinfo-serving-standing-stub]` — see the module docs; §7.3.1's real fold is not
//! reimplemented here, and this test must not be read as evidence about it.

use meer_queue::groupinfo_policy::{RefusalReason, ServeDecision, ServePolicy, ServingPeer};
use meer_queue::mls;
use mls_replant::{join, Persona};
use openmls::messages::group_info::VerifiableGroupInfo;
use openmls::prelude::*;
use tls_codec::{Deserialize as _, Serialize as _};

fn group_config() -> MlsGroupCreateConfig {
    MlsGroupCreateConfig::builder()
        .ciphersuite(mls_replant::CS)
        .use_ratchet_tree_extension(true)
        .build()
}

/// The lineage identifier a standing chain resolves against. In the spike this is the credential's
/// serialized identity; in the real system it is the persona lineage of §11.8.
fn lineage_of(p: &Persona) -> Vec<u8> {
    p.cwk.credential.serialized_content().to_vec()
}

fn seat(founder: &Persona, joiner: &Persona) -> (MlsGroup, MlsGroup) {
    let mut f = MlsGroup::new(
        &founder.provider,
        &founder.signer,
        &group_config(),
        founder.cwk.clone(),
    )
    .expect("create");
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
        _ => panic!("welcome"),
    };
    (f, join(joiner, welcome, tree))
}

/// Everything a history-convergence node would hold about the live group: a current `GroupInfo`
/// **with** the tree and one **without**, so the two-artifact gate (S18/S19) is expressible.
fn snapshot(group: &mut MlsGroup, who: &Persona) -> (Vec<u8>, Vec<u8>) {
    let with_tree = group
        .export_group_info(who.provider.crypto(), &who.signer, true)
        .expect("gi+tree")
        .tls_serialize_detached()
        .expect("ser");
    let bare = group
        .export_group_info(who.provider.crypto(), &who.signer, false)
        .expect("gi bare")
        .tls_serialize_detached()
        .expect("ser");
    (with_tree, bare)
}

fn as_group_info(bytes: &[u8]) -> VerifiableGroupInfo {
    match MlsMessageIn::tls_deserialize_exact(bytes)
        .expect("de")
        .extract()
    {
        MlsMessageBodyIn::GroupInfo(gi) => gi,
        _ => panic!("expected GroupInfo"),
    }
}

/// Try to re-enter with what the server handed over. `Ok` means she is in.
fn try_re_enter(who: &Persona, served: &[u8]) -> Result<MlsGroup, String> {
    MlsGroup::external_commit_builder()
        .with_config(MlsGroupJoinConfig::default())
        .build_group(&who.provider, as_group_info(served), who.cwk.clone())
        .map_err(|e| e.to_string())?
        .load_psks(who.provider.storage())
        .map_err(|e| e.to_string())?
        .build(
            who.provider.rand(),
            who.provider.crypto(),
            &who.signer,
            |_| true,
        )
        .map_err(|e| e.to_string())?
        .finalize(&who.provider)
        .map(|(g, _bundle)| g)
        .map_err(|e| e.to_string())
}

/// **Question 1 — refusing to serve is what stops a banned lineage.**
#[test]
fn a_standing_checked_server_refuses_a_banned_lineage_and_that_refusal_is_the_gate() {
    let alice = Persona::new("alice");
    let carol = Persona::new("carol");
    let (mut alices, carols) = seat(&alice, &carol);

    // Governance bans Carol and the group enacts it.
    let carol_leaf = carols.own_leaf_index();
    alices
        .remove_members(&alice.provider, &alice.signer, &[carol_leaf])
        .expect("remove");
    alices.merge_pending_commit(&alice.provider).expect("merge");

    let (with_tree, bare) = snapshot(&mut alices, &alice);
    let mut peer = ServingPeer::new(ServePolicy::StandingChecked, with_tree, bare);
    peer.ban_at_head(lineage_of(&carol));

    // She asks. The server resolves her lineage against the chain at head.
    let decision = peer.serve(&lineage_of(&carol), true);
    let reason = match decision {
        ServeDecision::Refused(r) => r,
        ServeDecision::Served { .. } => panic!("a banned lineage must not be served"),
    };
    assert!(matches!(reason, RefusalReason::BannedAtHead { .. }));

    // **And this is the whole difference from S18.** With nothing served, she has no path at all.
    // The control: hand her the same artifact the server withheld and she is straight back in.
    let (control_with_tree, _) = snapshot(&mut alices, &alice);
    assert!(
        try_re_enter(&carol, &control_with_tree).is_ok(),
        "the control arm must succeed, or the refusal proves nothing"
    );

    println!(
        "S22 CONFIRMED (real-lib): a standing-checked server **refused** a banned lineage — \
         `{reason}` — and the refusal is the entire gate. The control arm proves it: handed the same \
         `GroupInfo` the server withheld, she re-enters immediately. **So S18's \"a removal is only \
         as durable as GroupInfo distribution\" is not a defect to fix elsewhere — it is a \
         specification of where the fix goes.** Position 1 is a real position, not a label. [{}]",
        mls::resolved_versions()
    );
}

/// **Question 2 — the graceful path must survive the gate, or this is position 3.**
#[test]
fn a_dormant_member_in_good_standing_is_served_and_returns_immediately() {
    let alice = Persona::new("alice");
    let boreas = Persona::new("boreas");
    let (mut alices, boreass) = seat(&alice, &boreas);

    // Boreas goes dormant and is migrated to cold — the SAME operation as a ban (§11.6/§11.8),
    // which is exactly why the server, not the key layer, has to tell them apart.
    let leaf = boreass.own_leaf_index();
    alices
        .remove_members(&alice.provider, &alice.signer, &[leaf])
        .expect("migrate to cold");
    alices.merge_pending_commit(&alice.provider).expect("merge");
    for _ in 0..3 {
        mls_replant::commit(&mut alices, &alice);
    }

    let (with_tree, bare) = snapshot(&mut alices, &alice);
    let mut peer = ServingPeer::new(ServePolicy::StandingChecked, with_tree, bare);
    // Somebody else is banned; Boreas is not. The chain is non-empty, so this is a real check.
    peer.ban_at_head(b"some-other-banned-lineage".to_vec());

    let served = match peer.serve(&lineage_of(&boreas), true) {
        ServeDecision::Served { artifact, .. } => artifact,
        ServeDecision::Refused(r) => panic!("standing intact must be served, got {r}"),
    };
    let rejoined = try_re_enter(&boreas, &served).expect("he returns");

    println!(
        "S22 CONFIRMED (real-lib): the **same server, same chain, same epoch** that refused a banned \
         lineage **served** a dormant member in good standing, and he re-entered at epoch {} by his \
         own external commit — no Welcome, no active member's help, immediate. **Position 1 \
         preserves §11.7's self-service return for the case that actually happens (dormancy) while \
         closing it for the case governance decided against.** The key layer cannot make this \
         distinction (§11.6/§11.8 use the identical removal); the standing chain can, and this is \
         the component that consults it.",
        rejoined.epoch().as_u64()
    );
}

/// **Question 3 — the residual, located precisely.** A server that has not synced the ban serves.
#[test]
fn a_server_lagging_behind_the_ban_serves_and_that_is_the_entire_residual() {
    let alice = Persona::new("alice");
    let carol = Persona::new("carol");
    let (mut alices, carols) = seat(&alice, &carol);

    let carol_leaf = carols.own_leaf_index();
    alices
        .remove_members(&alice.provider, &alice.signer, &[carol_leaf])
        .expect("remove");
    alices.merge_pending_commit(&alice.provider).expect("merge");
    let (with_tree, bare) = snapshot(&mut alices, &alice);

    // **The lagging server.** It holds current GROUP state but a stale STANDING view — which is the
    // realistic failure, because the two propagate on different paths (§11.8: the governance chain
    // is separate from the epoch chain).
    let mut lagging = ServingPeer::new(ServePolicy::StandingChecked, with_tree.clone(), bare.clone());
    let served = match lagging.serve(&lineage_of(&carol), true) {
        ServeDecision::Served { artifact, .. } => artifact,
        ServeDecision::Refused(r) => panic!("a server that has not synced the ban cannot refuse: {r}"),
    };
    assert!(try_re_enter(&carol, &served).is_ok(), "so she gets back in");

    // Then it syncs, and closes.
    lagging.ban_at_head(lineage_of(&carol));
    assert!(matches!(
        lagging.serve(&lineage_of(&carol), true),
        ServeDecision::Refused(_)
    ));

    println!(
        "S22 MEASURED (real-lib): a server holding **current group state** but a **stale standing \
         view** served the banned lineage, and she re-entered. After syncing the ban the same server \
         refuses. **This is position 1's residual in miniature:** exposure is not 'she \
         found a hole' but 'the ban had not reached this peer yet'. The next test measures how wide \
         that is when every member is such a peer."
    );
    println!(
        "S22 CONSEQUENCE: a peer current on the EPOCH chain and stale on the GOVERNANCE chain is \
         precisely the hole, so **a peer SHOULD refuse to serve when its standing view is older than \
         its group view by more than a stated bound** — recorded as a design consequence, not a \
         measured requirement. **What this does NOT do is shrink the residual to a watchable tier.** \
         An earlier draft of this file claimed exactly that, on the reading that serving happens at a \
         history-convergence node. Part 1 §2.4 forbids that dependency, so **every member is one of \
         these peers** — see the last test in this file."
    );
}

/// **The second gate, composed with the first** (S18/S19): the tree is withheld by default.
#[test]
fn the_ratchet_tree_is_a_second_gate_the_server_controls_independently() {
    let alice = Persona::new("alice");
    let boreas = Persona::new("boreas");
    let (mut alices, boreass) = seat(&alice, &boreas);
    let leaf = boreass.own_leaf_index();
    alices
        .remove_members(&alice.provider, &alice.signer, &[leaf])
        .expect("remove");
    alices.merge_pending_commit(&alice.provider).expect("merge");

    let (with_tree, bare) = snapshot(&mut alices, &alice);
    let peer = ServingPeer::new(ServePolicy::StandingChecked, with_tree, bare);

    // Standing intact, but he did not ask for the tree — or the server declined to include it.
    let bare_served = match peer.serve(&lineage_of(&boreas), false) {
        ServeDecision::Served { artifact, .. } => artifact,
        ServeDecision::Refused(r) => panic!("standing intact: {r}"),
    };
    let refused = try_re_enter(&boreas, &bare_served)
        .expect_err("a bare GroupInfo must not be enough to re-enter");
    assert!(
        refused.contains("ratchet tree") || refused.contains("tree"),
        "the refusal must be the missing tree: {refused}"
    );

    // Same server, same standing, tree included → in.
    let full = match peer.serve(&lineage_of(&boreas), true) {
        ServeDecision::Served { artifact, .. } => artifact,
        ServeDecision::Refused(r) => panic!("{r}"),
    };
    assert!(try_re_enter(&boreas, &full).is_ok());

    println!(
        "S22 CONFIRMED (real-lib): the **ratchet tree is a second, independently-controlled gate at \
         the same server**. Same requester, same standing, same epoch: served bare he is refused \
         (`{refused}`); served with the tree he is in. **So a server has two dials, not one** — WHO \
         it serves (standing) and WHAT it releases (tree). The bare form is still useful: it proves \
         current group state for corroboration (§7.4.2) without admitting its holder, which is the \
         one thing S19 said a `GroupInfo` alone cannot do. **Withholding the tree recovers exactly \
         that property.**"
    );
}

/// **Position 0, for contrast** — the same server, ungated, is what the delivery design had.
#[test]
fn the_open_policy_serves_a_banned_lineage_which_is_what_position_zero_means() {
    let alice = Persona::new("alice");
    let carol = Persona::new("carol");
    let (mut alices, carols) = seat(&alice, &carol);
    let carol_leaf = carols.own_leaf_index();
    alices
        .remove_members(&alice.provider, &alice.signer, &[carol_leaf])
        .expect("remove");
    alices.merge_pending_commit(&alice.provider).expect("merge");

    let (with_tree, bare) = snapshot(&mut alices, &alice);
    let mut open_peer = ServingPeer::new(ServePolicy::Open, with_tree, bare);
    open_peer.ban_at_head(lineage_of(&carol)); // the chain KNOWS; the policy does not consult it

    let served = match open_peer.serve(&lineage_of(&carol), true) {
        ServeDecision::Served { artifact, .. } => artifact,
        ServeDecision::Refused(_) => panic!("an open policy refuses nobody"),
    };
    assert!(try_re_enter(&carol, &served).is_ok());

    println!(
        "S22 MEASURED (real-lib): under an **open** policy the server served a lineage it KNEW to be \
         banned — the standing chain was populated and simply not consulted — and she re-entered. \
         **This is the honest picture of position 0**, and it is what a `GroupInfo` server built for \
         §11.7's graceful return looks like before anyone gates it. The difference between position \
         0 and position 1 is one policy check at one component, and this file is the measurement of \
         what that check buys."
    );
}

/// **Question 4 — the one the server framing hid.**
///
/// Part 1 §2.4 forbids a Group depending on any single persona's presence to act, so there is no
/// serving tier to watch. **Every member holds live group state and can export a `GroupInfo`.** So
/// a negative check ("refuse if I know she is banned") is only as good as the *least* synced member.
#[test]
fn every_member_is_a_serving_peer_so_a_negative_check_is_only_as_good_as_the_least_synced() {
    const N: usize = 10;
    let alice = Persona::new("alice");
    let carol = Persona::new("carol");
    let (mut alices, carols) = seat(&alice, &carol);

    let carol_leaf = carols.own_leaf_index();
    alices
        .remove_members(&alice.provider, &alice.signer, &[carol_leaf])
        .expect("remove");
    alices.merge_pending_commit(&alice.provider).expect("merge");
    let (with_tree, bare) = snapshot(&mut alices, &alice);

    // Ten members, all holding the same current group state — because they do; that is what being
    // a member means. The ban has reached nine of them.
    let mut membership: Vec<ServingPeer> = (0..N)
        .map(|_| ServingPeer::new(ServePolicy::StandingChecked, with_tree.clone(), bare.clone()))
        .collect();
    const UNSYNCED: usize = 6;
    for (i, peer) in membership.iter_mut().enumerate() {
        if i != UNSYNCED {
            peer.ban_at_head(lineage_of(&carol));
        }
    }

    // She asks each in turn. She needs exactly one yes.
    let mut refusals = 0;
    let mut got: Option<Vec<u8>> = None;
    for peer in &membership {
        match peer.serve(&lineage_of(&carol), true) {
            ServeDecision::Refused(_) => refusals += 1,
            ServeDecision::Served { artifact, .. } => got = got.or(Some(artifact)),
        }
    }
    assert_eq!(refusals, N - 1, "nine of ten correctly refused");
    let served = got.expect("but one did not, and that is all she needed");
    assert!(try_re_enter(&carol, &served).is_ok(), "she is back in");

    // Position 2, same membership, same stale peer: the check is POSITIVE, so staleness fails closed.
    let vouched: Vec<ServingPeer> = (0..N)
        .map(|_| {
            ServingPeer::new(
                ServePolicy::Vouched(std::collections::HashSet::new()),
                with_tree.clone(),
                bare.clone(),
            )
        })
        .collect();
    let all_refused = vouched
        .iter()
        .all(|p| matches!(p.serve(&lineage_of(&carol), true), ServeDecision::Refused(_)));
    assert!(all_refused, "no peer holds a token for her, so every one refuses");

    println!(
        "S22 MEASURED (real-lib): with {N} members all holding current group state, **nine refused \
         correctly and one stale peer served** — and she needed exactly one yes. **A negative check \
         is only as good as the LEAST synced member**, and Part 1 §2.4 guarantees there is no serving \
         tier to shrink that set to. **The earlier claim that position 1 makes the residual \
         'a small enumerable set a community can watch' is WITHDRAWN — it was an artifact of the \
         server framing, and the server does not exist.**"
    );
    println!(
        "S22 CONSEQUENCE — and it inverts the earlier recommendation: under the SAME staleness, \
         **position 2 refused at every peer**, because a positive check needs the verifier to hold a \
         token rather than to have heard about a ban. **A negative check fails OPEN on a stale peer; \
         a positive check fails CLOSED.** In an architecture with a chokepoint, position 1 is the \
         cheap right answer. **In this architecture — no chokepoint, by principle — position 1's \
         guarantee degrades to the worst-synced member, and position 2 is the one that actually \
         holds.** Position 1 remains valuable as the DEFAULT-CASE path (dormancy, where failing open \
         is the desired behaviour); it is not adequate as the ban defence."
    );
}
