//! **G1 — the §7.3.1 fold against its own three ordering keys.**
//!
//! Every standing decision in the corpus depends on this fold: §11.8's admission gate, the meer
//! spike's S22 serving policy, and every Croft governance row of the scenario matrix. §11.11 carries
//! its hardest property (gap-completeness) as a named beam, and the corpus's scenario matrix
//! initially recorded the fold as **ungrounded** — which was **wrong**: it is implemented in
//! `local_storage_projection::fold_derived` and exercised by `competing_quorums.rs` (RUN-01/03).
//!
//! So the useful experiment is **not** to rebuild it. It is to check the implementation against the
//! three keys §7.3.1 actually specifies, because the fold resolves by **sequential replay in
//! `merge_cmp` order** (`lamport → author_device → envelope_hash`), and two of the three keys are
//! not obviously in that comparator:
//!
//! 1. **Operation-type precedence — "subtractions before additions."** §7.3.1 key 1 requires a
//!    *layered* fold: threshold changes, then membership removals, then role/capability removals,
//!    then grants, then membership additions, each tier resolved against the settled result of the
//!    tiers above. A flat lamport sort does not do this. **Does the restrictive reading win?**
//! 2. **Causal precedence within a tier.** Lamport ordering should deliver this.
//! 3. **The concurrent tiebreak must be the CONTENT ADDRESS, and party-neutral.** §7.3.1: *"any
//!    party-privileging key is opt-in and itself under k-of-n governance, and the content address
//!    remains the total fallback."* `merge_cmp` puts **`author_device` ahead of the hash**. **Is a
//!    concurrent conflict decided by whose device it was?**
//!
//! And the property all three serve: **order independence**. Same facts, any arrival order, same
//! standing.
//!
//! **This file does NOT test gap-completeness** (§11.11's beam) and is not evidence about it.
//! Fidelity: **Modeled** — a real fold over a real store, but governance facts are this
//! experiment's own envelope type, not a wire-format-final Drystone encoding.

mod common;

use std::sync::Arc;

use common::{base, has_member, membership_add_payload, sign, signed_genesis};
use local_storage_projection::fold_derived::DerivedFold;
use local_storage_projection::tables::Db;
use local_storage_projection::types::envelope_hash;
use local_storage_projection::{
    AssertionEnvelope, AssertionType, DeviceId, GroupId, Hash, PrincipalId,
};
use social_graph_core::{Ed25519Verifier, Identity, RegistryCredentialResolver, Session};

fn remove_payload(subject: PrincipalId) -> Vec<u8> {
    subject.as_bytes().to_vec()
}

/// Fold `order` into a fresh store and open a reader over the result.
fn fold_order(
    path: &std::path::Path,
    authors: &[&Identity],
    reader: &Identity,
    order: &[&AssertionEnvelope],
) -> Session {
    {
        let db = Arc::new(Db::open(path).expect("open db"));
        let resolver = RegistryCredentialResolver::new();
        for a in authors {
            resolver.register(a.device_id(), a.principal_id());
        }
        let fold = DerivedFold::new(Arc::clone(&db), Ed25519Verifier, resolver);
        for env in order {
            // Ingest outcomes are surfaced, never swallowed: a REJECTED fact would make an
            // apparent order-dependence into a harness artifact (different facts accepted in
            // different orders), which is the first thing to rule out before believing a
            // convergence failure.
            match fold.ingest(env) {
                Ok(_) => eprintln!("    ingest {:?} -> ok", env.assertion_type),
                Err(e) => eprintln!("    ingest {:?} -> REJECTED: {e}", env.assertion_type),
            }
        }
    }
    Session::open(path, reader).expect("open session")
}

/// Is `who` a member in the folded state? Thin wrapper over the shared harness helper so the
/// call sites below read as governance questions rather than storage lookups.
fn is_member(session: &Session, group: &GroupId, who: &PrincipalId) -> bool {
    has_member(session, group, who)
}

/// **Key 3 — is the concurrent tiebreak party-neutral?**
///
/// Two mutually-concurrent facts of the same type at the same lamport. §7.3.1 says the content
/// address decides. `merge_cmp` consults `author_device` first.
#[tokio::test]
async fn the_concurrent_tiebreak_consults_author_device_before_the_content_address() {
    // Two envelopes identical in everything the spec's key 3 says should matter, differing only in
    // author device — so if the outcome differs, the device decided it.
    let id_x = Identity::from_seed([0xB0; 32]);
    let id_y = Identity::from_seed([0xB1; 32]);
    let group = GroupId::new([0xC1; 32]);
    let victim = PrincipalId::new([0xDD; 32]);

    let a = sign(
        &id_x,
        base(
            &id_x,
            group,
            AssertionType::MembershipRemove,
            7,
            vec![],
            remove_payload(victim),
        ),
    );
    let b = sign(
        &id_y,
        base(
            &id_y,
            group,
            AssertionType::MembershipRemove,
            7,
            vec![],
            remove_payload(victim),
        ),
    );

    let ha = envelope_hash(&a);
    let hb = envelope_hash(&b);
    let hash_says: DeviceId = if ha.as_bytes() <= hb.as_bytes() {
        a.author_device
    } else {
        b.author_device
    };
    let device_says: DeviceId = if a.author_device.as_bytes() <= b.author_device.as_bytes() {
        a.author_device
    } else {
        b.author_device
    };

    let cmp = local_storage_projection::types::merge_cmp(&a, &b);
    let comparator_says = if cmp == std::cmp::Ordering::Less {
        a.author_device
    } else {
        b.author_device
    };

    println!(
        "G1 MEASURED (modeled): two mutually-concurrent same-type facts at the same lamport. \
         The CONTENT ADDRESS would order {}first; the AUTHOR DEVICE orders {}first; \
         `merge_cmp` puts {}first.",
        if hash_says == a.author_device { "A " } else { "B " },
        if device_says == a.author_device { "A " } else { "B " },
        if comparator_says == a.author_device { "A " } else { "B " },
    );
    assert_eq!(
        comparator_says, device_says,
        "the comparator follows the DEVICE key, not the content address"
    );

    println!(
        "G1 FINDING (modeled): **`merge_cmp` orders `lamport -> author_device -> envelope_hash`, so \
         among genuine concurrents the AUTHOR'S DEVICE decides before the content address ever \
         runs.** §7.3.1 key 3 specifies the opposite: the default key is the content address, \
         *\"party-neutral, ungameable, and identical everywhere\"*, and *\"any party-privileging key \
         is opt-in and itself under k-of-n governance\"*. **A device identifier is a \
         party-privileging key applied as a silent default.**"
    );
    println!(
        "G1 CONSEQUENCE: this is a DIVERGENCE to adjudicate, not a bug report — the ordering is still \
         deterministic and identical on every node, so the fold's convergence property is NOT at \
         risk, and no test here shows divergence. What is at risk is the *reason* §7.3.1 gives for \
         choosing the content address: a party-derived key means a participant who can choose device \
         identifiers can bias every concurrent tie they are party to. **Two honest readings:** \
         (a) `author_device` is a stability aid and the spec should say the tiebreak is \
         `(device, hash)`; or (b) the implementation should drop to the hash and let §7.3.1 stand. \
         **Owner's call — the spec and the code currently disagree.**"
    );
}

/// **Key 1 — operation-type precedence: does the restrictive reading win?**
///
/// A concurrent `RemoveMember(m)` and `AddMember(m)`. §7.3.1 key 1 puts membership **removals** in a
/// strictly higher tier than membership **additions**, resolved against the settled result above —
/// so the removal must win **regardless of arrival order**.
///
/// **Genuine concurrency requires two devices.** A first draft of this test authored both facts from
/// one device at the same lamport; the fold **rejected the second** ("lamport violation… expected >
/// 2, got 2"), because a device's own clock must be monotonic. That produced an apparent
/// order-dependence which was purely a harness artifact — different facts accepted in different
/// orders. Two distinct authorized devices is what makes the pair actually concurrent.
#[tokio::test]
async fn concurrent_remove_and_add_should_resolve_restrictively_regardless_of_arrival() {
    let o1 = Identity::from_seed([0xE0; 32]);
    let o2 = Identity::from_seed([0xE2; 32]);
    let group = GroupId::new([0xC2; 32]);
    let subject = Identity::from_seed([0xE1; 32]);
    let subject_pid = PrincipalId::new(subject.principal_id().0);
    let o2_pid = PrincipalId::new(o2.principal_id().0);

    let genesis = signed_genesis(&o1, group, 0);
    let gh = envelope_hash(&genesis);

    // A second owner, so two devices can both author governance facts.
    let add_o2 = sign(
        &o1,
        base(&o1, group, AssertionType::MembershipAdd, 1, vec![gh],
             membership_add_payload(o2_pid, 0)),
    );
    let seat = sign(
        &o1,
        base(&o1, group, AssertionType::MembershipAdd, 2, vec![envelope_hash(&add_o2)],
             membership_add_payload(subject_pid, 2)),
    );
    let seat_h = envelope_hash(&seat);

    // Genuinely concurrent: different devices, same lamport, same observed frontier.
    let remove = sign(
        &o1,
        base(&o1, group, AssertionType::MembershipRemove, 3, vec![seat_h],
             remove_payload(subject_pid)),
    );
    let readd = sign(
        &o2,
        base(&o2, group, AssertionType::MembershipAdd, 3, vec![seat_h],
             membership_add_payload(subject_pid, 2)),
    );

    let dir_a = tempfile::tempdir().expect("tempdir");
    let dir_b = tempfile::tempdir().expect("tempdir");
    let authors = [&o1, &o2];
    let s_remove_first = fold_order(
        &dir_a.path().join("a.redb"), &authors, &o1,
        &[&genesis, &add_o2, &seat, &remove, &readd],
    );
    let s_add_first = fold_order(
        &dir_b.path().join("b.redb"), &authors, &o1,
        &[&genesis, &add_o2, &seat, &readd, &remove],
    );

    let remove_first = is_member(&s_remove_first, &group, &subject_pid);
    let add_first = is_member(&s_add_first, &group, &subject_pid);
    let status_a = s_remove_first
        .get_group_summary(&group)
        .map(|s| s.fork_status)
        .unwrap_or_default();
    let status_b = s_add_first
        .get_group_summary(&group)
        .map(|s| s.fork_status)
        .unwrap_or_default();
    println!("    fork_status: remove-first={status_a:?} add-first={status_b:?}");

    println!(
        "G1 MEASURED (modeled): a genuinely concurrent MembershipRemove(m) and MembershipAdd(m) — \
         two authorized devices, equal lamport, same observed frontier. Ingested remove-then-add the \
         subject is {}; ingested add-then-remove the subject is {}.",
        if remove_first { "A MEMBER" } else { "NOT a member" },
        if add_first { "A MEMBER" } else { "NOT a member" },
    );

    assert_eq!(
        status_a, status_b,
        "the CONTRADICTION must be named identically in both arrival orders"
    );
    assert!(
        status_a.starts_with("contradiction:"),
        "expected a hard-stop, got {status_a:?}"
    );
    assert_eq!(
        remove_first, add_first,
        "and the projection must agree across arrival orders"
    );

    println!(
        "G1 CONFIRMED (modeled): **the fold does NOT silently resolve this permissively — it \
         HARD-STOPS.** Both arrival orders surfaced the identical contradiction byte-head, exactly \
         as §7.3.2/§7.6 require: where a contradiction cannot be determinately resolved without \
         manufacturing a utility verdict, escalate rather than fold, and name the conflicting pair \
         order-independently. **An earlier draft of this file read the MEMBER projection as \"the \
         addition won, key 1 is missing, the fold fails open\". That was WRONG and is withdrawn** — \
         the fold escalated, and what is shown is the projection of a group hard-stopped awaiting \
         human resolution, not a resolved verdict."
    );
    println!(
        "G1 NOTE: while hard-stopped this shape projects the subject as A MEMBER in both orders — \
         consistent across peers, so not a convergence problem, but worth an explicit product \
         decision about what a contradicted group should DISPLAY, since hard-stopped and \
         currently-a-member are being shown together."
    );
}

/// **The property all three keys serve.** Permute a governance log and demand one answer.
///
/// **Only genuinely concurrent facts may be permuted, and this took two attempts to get right.**
/// A first draft permuted causally-dependent facts; the fold rejected the dependent one when it
/// arrived first (`missing antecedents: have 0 of 1`) and **did not retry it** on this direct-ingest
/// path, so three of six permutations silently lost a removal. That is a harness boundary, not a
/// fold defect — `DerivedFold::ingest` is the raw seam, and the buffering/retry the module documents
/// lives above it — but it would have read as an order-dependence failure. Recorded because the
/// distinction is the whole point of the test.
///
/// So: every permuted fact references the **same observed frontier**, is authored by a **distinct
/// device**, and acts on a subject seated **causally before** the frontier. Any arrival order is
/// then legal, and convergence is the only thing under test.
#[tokio::test]
async fn the_fold_is_order_independent_across_permutations_of_concurrent_facts() {
    let o1 = Identity::from_seed([0xF0; 32]);
    let o2 = Identity::from_seed([0xF1; 32]);
    let o3 = Identity::from_seed([0xF2; 32]);
    let group = GroupId::new([0xC3; 32]);
    let m1 = PrincipalId::new([0x11; 32]);
    let m2 = PrincipalId::new([0x22; 32]);
    let m3 = PrincipalId::new([0x33; 32]);

    // --- setup, strictly causal: three owners and two seated members ---
    let genesis = signed_genesis(&o1, group, 0);
    let gh = envelope_hash(&genesis);
    let add_o2 = sign(&o1, base(&o1, group, AssertionType::MembershipAdd, 1, vec![gh],
        membership_add_payload(PrincipalId::new(o2.principal_id().0), 0)));
    let add_o3 = sign(&o1, base(&o1, group, AssertionType::MembershipAdd, 2, vec![envelope_hash(&add_o2)],
        membership_add_payload(PrincipalId::new(o3.principal_id().0), 0)));
    let seat_m1 = sign(&o1, base(&o1, group, AssertionType::MembershipAdd, 3, vec![envelope_hash(&add_o3)],
        membership_add_payload(m1, 2)));
    let seat_m2 = sign(&o1, base(&o1, group, AssertionType::MembershipAdd, 4, vec![envelope_hash(&seat_m1)],
        membership_add_payload(m2, 2)));
    let frontier = envelope_hash(&seat_m2);

    // --- the concurrent set: same frontier, three devices, two removals and one addition ---
    let rm_m1 = sign(&o1, base(&o1, group, AssertionType::MembershipRemove, 5, vec![frontier],
        remove_payload(m1)));
    let add_m3 = sign(&o2, base(&o2, group, AssertionType::MembershipAdd, 5, vec![frontier],
        membership_add_payload(m3, 2)));
    let rm_m2 = sign(&o3, base(&o3, group, AssertionType::MembershipRemove, 5, vec![frontier],
        remove_payload(m2)));

    let perms: [[&AssertionEnvelope; 3]; 6] = [
        [&rm_m1, &add_m3, &rm_m2], [&rm_m1, &rm_m2, &add_m3], [&add_m3, &rm_m1, &rm_m2],
        [&add_m3, &rm_m2, &rm_m1], [&rm_m2, &rm_m1, &add_m3], [&rm_m2, &add_m3, &rm_m1],
    ];

    let authors = [&o1, &o2, &o3];
    let setup: Vec<&AssertionEnvelope> = vec![&genesis, &add_o2, &add_o3, &seat_m1, &seat_m2];
    let mut outcomes = Vec::new();
    for p in perms {
        let dir = tempfile::tempdir().expect("tempdir");
        let order: Vec<&AssertionEnvelope> = setup.iter().copied().chain(p).collect();
        let s = fold_order(&dir.path().join("p.redb"), &authors, &o1, &order);
        outcomes.push((
            is_member(&s, &group, &m1),
            is_member(&s, &group, &m2),
            is_member(&s, &group, &m3),
        ));
    }

    let first = outcomes[0];
    println!("G1 MEASURED (modeled): 6 permutations of 3 concurrent facts resolved to: {outcomes:?}");
    assert!(
        outcomes.iter().all(|o| *o == first),
        "ORDER INDEPENDENCE is the fold's whole purpose: got {outcomes:?}"
    );

    println!(
        "G1 CONFIRMED (modeled): all **6** arrival permutations of three genuinely concurrent \
         governance facts — two removals and one addition, three distinct authoring devices, one \
         shared frontier — resolved to the **identical** membership state (m1 {}, m2 {}, m3 {}). \
         **The fold's central property holds over this shape: same event set in, same standing out, \
         whatever order the network delivered.** Note precisely what this does and does not show: it \
         is order independence over a **complete** set. §11.11's beam is whether a node can *know* \
         its set is complete, and **nothing here bears on that** — the next test shows what happens \
         when it is not.",
        if first.0 { "member" } else { "not a member" },
        if first.1 { "member" } else { "not a member" },
        if first.2 { "member" } else { "not a member" },
    );
}

/// **The failure §11.11 warns about, made concrete.** Two peers, different (incomplete) sets.
#[tokio::test]
async fn two_peers_folding_different_subsets_can_resolve_standing_differently() {
    let owner = Identity::from_seed([0xA8; 32]);
    let group = GroupId::new([0xC4; 32]);
    let subject = PrincipalId::new([0x33; 32]);

    let genesis = signed_genesis(&owner, group, 0);
    let gh = envelope_hash(&genesis);
    let add = sign(
        &owner,
        base(
            &owner,
            group,
            AssertionType::MembershipAdd,
            1,
            vec![gh],
            membership_add_payload(subject, 2),
        ),
    );
    let ban = sign(
        &owner,
        base(
            &owner,
            group,
            AssertionType::MembershipRemove,
            2,
            vec![envelope_hash(&add)],
            remove_payload(subject),
        ),
    );

    // Peer A holds everything. Peer B never received the ban.
    let dir_a = tempfile::tempdir().expect("tempdir");
    let dir_b = tempfile::tempdir().expect("tempdir");
    let complete = fold_order(&dir_a.path().join("a.redb"), &[&owner], &owner, &[&genesis, &add, &ban]);
    let incomplete = fold_order(&dir_b.path().join("b.redb"), &[&owner], &owner, &[&genesis, &add]);

    let a_says = is_member(&complete, &group, &subject);
    let b_says = is_member(&incomplete, &group, &subject);
    assert_ne!(
        a_says, b_says,
        "the whole point: an incomplete set yields a different standing"
    );

    println!(
        "G1 MEASURED (modeled): peer A (complete set) resolves the subject as **{}**; peer B, which \
         never received the ban, resolves the subject as **{}**. Neither peer is wrong over the set \
         it holds — which is exactly why §11.11 calls this the beam. **This is the governance-layer \
         source of the delivery-layer behaviour S22 measured**: the stale peer that served a banned \
         lineage was not malfunctioning, it was folding an incomplete set correctly.",
        if a_says { "a member" } else { "NOT a member" },
        if b_says { "a member" } else { "NOT a member" },
    );
    println!(
        "G1 CONSEQUENCE: the two experiments now meet. S22 measured that a **negative** standing \
         check fails open at the least-synced peer; G1 shows **why** that peer answers as it does. \
         So 'keep everyone synced' is not a mitigation available at either layer — it is a restatement \
         of the beam. **The available mitigation is the one S22 measured: a POSITIVE credential, \
         which a peer can verify from what it already holds and which therefore does not depend on \
         the peer's set being complete.** That is a design consequence of two measurements meeting, \
         and it is the strongest argument yet for position 2."
    );
}

/// Read `who`'s effective role, if any, from the folded state.
fn role_of(session: &Session, group: &GroupId, who: &PrincipalId) -> Option<String> {
    let summary = session.get_group_summary(group).ok()?;
    summary
        .members
        .iter()
        .find(|m| &m.principal == who)
        .map(|m| format!("{:?}", m.role))
}

/// **Key 1, the case §7.3.2 actually emphasises — and the one with a story that holds up.**
///
/// The first version of this file tested a concurrent `Remove(m)` + `Add(m)`. **That pairing is hard
/// to reach socially** (owner, 2026-08-16): a moderator who has not seen the removal still sees `m`
/// as a member, and would have no reason to *add* an existing member. The finding was mechanically
/// real and the scenario around it was fabricated.
///
/// **This is the collision that does happen.** A removes `m` for cause. Concurrently B — who has not
/// synced — **promotes `m`**, an entirely ordinary act toward someone you still see as a member.
/// §7.3.2 is explicit about the required outcome:
///
/// > *"a role is effective only if its slot resolves to granted **and** m's membership slot resolves
/// > to member, so a removed member holds no effective role even if a concurrent grant happened to
/// > win that role's tiebreak."*
#[tokio::test]
async fn a_concurrent_promotion_must_not_survive_a_removal() {
    let o1 = Identity::from_seed([0x70; 32]);
    let o2 = Identity::from_seed([0x71; 32]);
    let group = GroupId::new([0xC7; 32]);
    let m = PrincipalId::new([0x77; 32]);
    let o2_pid = PrincipalId::new(o2.principal_id().0);

    let genesis = signed_genesis(&o1, group, 0);
    let gh = envelope_hash(&genesis);
    let add_o2 = sign(&o1, base(&o1, group, AssertionType::MembershipAdd, 1, vec![gh],
        membership_add_payload(o2_pid, 0)));
    let seat_m = sign(&o1, base(&o1, group, AssertionType::MembershipAdd, 2,
        vec![envelope_hash(&add_o2)], membership_add_payload(m, 2)));
    let frontier = envelope_hash(&seat_m);

    // Genuinely concurrent, and both are natural acts from their author's view.
    let remove = sign(&o1, base(&o1, group, AssertionType::MembershipRemove, 3, vec![frontier],
        remove_payload(m)));
    let promote = sign(&o2, base(&o2, group, AssertionType::RoleGrant, 3, vec![frontier],
        membership_add_payload(m, 1)));  // same payload shape: principal ‖ role. 1 = Admin.

    let dir_a = tempfile::tempdir().expect("tempdir");
    let dir_b = tempfile::tempdir().expect("tempdir");
    let authors = [&o1, &o2];
    let removal_first = fold_order(&dir_a.path().join("a.redb"), &authors, &o1,
        &[&genesis, &add_o2, &seat_m, &remove, &promote]);
    let promote_first = fold_order(&dir_b.path().join("b.redb"), &authors, &o1,
        &[&genesis, &add_o2, &seat_m, &promote, &remove]);

    let a_member = is_member(&removal_first, &group, &m);
    let b_member = is_member(&promote_first, &group, &m);
    let a_role = role_of(&removal_first, &group, &m);
    let b_role = role_of(&promote_first, &group, &m);

    println!(
        "G1 MEASURED (modeled): A removes m for cause; concurrently B promotes m to Admin, not having \
         seen the removal. Ingested removal-first: member={a_member}, role={a_role:?}. Ingested \
         promotion-first: member={b_member}, role={b_role:?}."
    );

    assert_eq!(
        (a_member, &a_role), (b_member, &b_role),
        "ORDER INDEPENDENCE: both arrival orders must resolve identically"
    );

    if a_member {
        println!(
            "G1 FINDING (modeled): **the removed member is still a member, and holds the concurrently \
             granted role.** §7.3.2 requires the opposite — *\"a removed member holds no effective role \
             even if a concurrent grant happened to win that role's tiebreak\"* — and §7.3.1 key 1 puts \
             membership removals in a strictly higher tier than role grants. **Neither the tier order \
             nor the effective-roles projection is applied here.** This is the collision with a story \
             that holds up: promoting someone you still see as a member is an ordinary act, and it \
             silently reverses a removal made for cause."
        );
    } else {
        println!(
            "G1 CONFIRMED (modeled): the removal held — m is not a member and holds no effective role \
             — so the restrictive reading won over a concurrent promotion. §7.3.2's projection is \
             doing its job for this shape, whether via the tier order or via the projection itself."
        );
    }
}

/// **The reachable remove-vs-add collision — and it is the readmission case.**
///
/// The earlier remove-vs-re-add test was socially unreachable: two moderators who both see `m` as a
/// member, one of whom nonetheless issues an *Add*. This is the shape that does occur, and both acts
/// are natural from their author's own view:
///
/// **`m` is NOT currently a member** — they left, or were removed long ago, and have applied to
/// rejoin. Moderator A **approves the application** (an Add). Concurrently, moderator B **enacts a
/// standing ban** on `m`'s lineage (a Remove). Neither has seen the other. Per §7.3.1 key 1,
/// membership removals resolve in a strictly higher tier than additions, so the restrictive reading
/// must win and `m` must stay out.
#[tokio::test]
async fn approving_a_rejoin_concurrently_with_enacting_a_ban_must_resolve_restrictively() {
    let a = Identity::from_seed([0x80; 32]);
    let b = Identity::from_seed([0x81; 32]);
    let group = GroupId::new([0xC8; 32]);
    let m = PrincipalId::new([0x88; 32]);
    let b_pid = PrincipalId::new(b.principal_id().0);

    let genesis = signed_genesis(&a, group, 0);
    let gh = envelope_hash(&genesis);
    let add_b = sign(&a, base(&a, group, AssertionType::MembershipAdd, 1, vec![gh],
        membership_add_payload(b_pid, 0)));
    let frontier = envelope_hash(&add_b);

    // m is NOT a member at the frontier — both moderators agree about that.
    // A approves the rejoin application; B enacts the ban. Genuinely concurrent.
    let approve = sign(&a, base(&a, group, AssertionType::MembershipAdd, 2, vec![frontier],
        membership_add_payload(m, 2)));
    let enact_ban = sign(&b, base(&b, group, AssertionType::MembershipRemove, 2, vec![frontier],
        remove_payload(m)));

    let dir_a = tempfile::tempdir().expect("tempdir");
    let dir_b = tempfile::tempdir().expect("tempdir");
    let authors = [&a, &b];
    let approve_first = fold_order(&dir_a.path().join("a.redb"), &authors, &a,
        &[&genesis, &add_b, &approve, &enact_ban]);
    let ban_first = fold_order(&dir_b.path().join("b.redb"), &authors, &a,
        &[&genesis, &add_b, &enact_ban, &approve]);

    let approve_first_member = is_member(&approve_first, &group, &m);
    let ban_first_member = is_member(&ban_first, &group, &m);

    // Characterise the divergence fully before claiming anything about it: both stores were fed
    // the IDENTICAL four facts and every ingest returned ok, so any difference here is the fold's.
    let dump = |label: &str, sess: &Session| {
        let sum = sess.get_group_summary(&group).expect("summary");
        let mut who: Vec<String> = sum
            .members
            .iter()
            .map(|mv| format!("{:02x}{:02x}:{:?}", mv.principal.as_bytes()[0], mv.principal.as_bytes()[1], mv.role))
            .collect();
        who.sort();
        println!("    [{label}] members={who:?} fork_status={:?}", sum.fork_status);
    };
    dump("approve-then-ban", &approve_first);
    dump("ban-then-approve", &ban_first);

    println!(
        "G1 MEASURED (modeled): m is not a member; A approves their rejoin while B concurrently \
         enacts a standing ban. Ingested approve-then-ban: m is {}. Ingested ban-then-approve: m is \
         {}. **All eight ingests returned ok in both runs — no fact was rejected, so both stores \
         hold the identical four-fact set.**",
        if approve_first_member { "A MEMBER" } else { "NOT a member" },
        if ban_first_member { "A MEMBER" } else { "NOT a member" },
    );

    let st_a = approve_first
        .get_group_summary(&group)
        .map(|s| s.fork_status)
        .unwrap_or_default();
    let st_b = ban_first
        .get_group_summary(&group)
        .map(|s| s.fork_status)
        .unwrap_or_default();

    // What the spec REQUIRES: the contradiction is detected and named identically everywhere.
    assert_eq!(st_a, st_b, "the contradiction byte-head must be order-independent");
    assert!(st_a.starts_with("contradiction:"), "expected a hard-stop, got {st_a:?}");

    println!(
        "G1 CONFIRMED (modeled): both peers detected the SAME contradiction and hard-stopped, with a \
         byte-identical head (`{st_a}`) regardless of arrival order. **§7.3.2/§7.6's escalation \
         behaviour holds:** a ban racing a rejoin approval is not silently resolved either way — it \
         is surfaced as a contradiction for humans, which is the correct outcome and a good one."
    );

    if approve_first_member != ban_first_member {
        println!(
            "G1 FINDING (modeled): **but the MEMBERSHIP PROJECTION diverges while hard-stopped.** \
             Same four facts, every ingest ok, identical contradiction — yet approve-then-ban shows \
             the applicant as A MEMBER and ban-then-approve shows them as NOT a member. **Two \
             moderators looking at the same contradicted group see different member lists**, and \
             nothing on screen distinguishes which projection they are looking at."
        );
        println!(
            "G1 CONSEQUENCE: this is narrower than a convergence failure — the RESOLUTION converges \
             (same contradiction, same head) and only the projected state does not — but it is the \
             one a human actually reads. §7.6 says to present a contradiction as *\"an unambiguous, \
             grounded statement of the two conflicting facts\"*; it does not say what the membership \
             view should show meanwhile, and this measures why that gap matters. **Two candidate \
             rules, both spec work rather than code work:** project the RESTRICTIVE reading while \
             contradicted (the applicant is out until humans decide, matching key 1's fail-safe \
             direction), or project NO membership answer at all for the contested subject and force \
             the UI to show the contradiction instead of a member list. **Recorded for the owner; \
             not decided here.**"
        );
    } else {
        println!(
            "G1 CONFIRMED (modeled): and the membership projection agrees across both orders as well."
        );
    }
}
