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

    println!(
        "G1 MEASURED (modeled): a genuinely concurrent MembershipRemove(m) and MembershipAdd(m) — \
         two authorized devices, equal lamport, same observed frontier. Ingested remove-then-add the \
         subject is {}; ingested add-then-remove the subject is {}.",
        if remove_first { "A MEMBER" } else { "NOT a member" },
        if add_first { "A MEMBER" } else { "NOT a member" },
    );

    assert_eq!(
        remove_first, add_first,
        "ORDER INDEPENDENCE: both arrival orders must resolve identically"
    );

    if remove_first {
        println!(
            "G1 FINDING (modeled): the fold **converged** (both orders agree) but resolved to \
             **MEMBER** — the ADDITION won. §7.3.1 key 1 requires the opposite: *\"subtractions \
             before additions… this biases every intermediate state toward the more restrictive \
             reading (the fail-safe direction)\"*, with membership removals in a strictly higher tier \
             than membership additions. **The layered operation-type fold is not implemented: \
             resolution is a flat sequential replay in `merge_cmp` order, so the later-sorted fact \
             wins whatever its type.**"
        );
        println!(
            "G1 CONSEQUENCE — and it touches the ban work directly: key 1 IS the fail-safe direction, \
             so its absence fails OPEN in exactly the case readmission cares about. A concurrent \
             remove/add on one subject should settle to removed; measured, it settles to whichever \
             the comparator places later. **This is the governance-layer instance of the same shape \
             S22 found at the delivery layer — a restrictive rule that is not actually applied fails \
             open.** The fix (a tiered fold) is a design change, deliberately NOT made here."
        );
    } else {
        println!(
            "G1 CONFIRMED (modeled): resolved to **NOT a member** in both arrival orders — the \
             restrictive reading won, as §7.3.1 key 1 requires. Whether that is the tier mechanism or \
             this pair's comparator order needs a second shape to separate; recorded as \
             consistent-with-spec, not as proof of the tier."
        );
    }
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
