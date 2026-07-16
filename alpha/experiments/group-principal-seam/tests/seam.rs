//! §2e — the group-principal seam spike (RUN-11 Part 3).
//!
//! Executes the shaped backlog §2e assertions against the group-principal seam
//! brief (`beta/impl/drystone-design/group-principal-seam.md`, RUN-10 Part 2):
//! the Group principal is a Meadowcap **communal namespace** identified by the
//! genesis hash; personae are **self-authorizing subspaces**; capability
//! issuance is **downstream of the folded Group Role** and is **re-issued (never
//! revoked-in-place)** across a fold-driven authority change — asserting a
//! revoked persona's stale capability fails and a re-issued one succeeds,
//! deterministically on both members.
//!
//! Scope wall (Part 3 rule 3): every binding here is `Design`-grade experiment
//! code. No trust tier is decided (who-may-revoke is an *input* governance fact,
//! not chosen here — I9), no wire/byte encoding is pinned (the `SubspaceId` and
//! genesis-id encodings stay `[gates-release]`, test-only serialization), and no
//! MLS internals are touched. The row's aspiration to move the subspace-mapping
//! half `Design → green-real` against real crypto EXCEEDS this wall and is
//! FINDING-stopped (see the RUN-11 summary); the Design-grade seam model lands.

use group_principal_seam::{Governance, GroupPrincipal, SubspaceId, Write};

// Assertion 1 — the Group principal IS a communal namespace identified by the
// genesis hash, stable across churn.
#[test]
fn group_principal_is_the_genesis_hash_stable_across_churn() {
    let group = GroupPrincipal::of("drystone-group", "acme-coop");
    // Deterministic: same (tag, group_id) => same namespace id.
    assert_eq!(
        group,
        GroupPrincipal::of("drystone-group", "acme-coop"),
        "the communal-namespace id is H(tag || group_id), deterministic",
    );
    // A different group is a different namespace.
    assert_ne!(
        group,
        GroupPrincipal::of("drystone-group", "other-coop"),
        "a different group_id is a different namespace",
    );
    // Stable across churn: the namespace id does not rotate when membership
    // changes (there is no whole-namespace secret to rotate — F1).
    let before = group.namespace_id().to_vec();
    let mut gov = Governance::genesis(&group, &[sub("alice"), sub("bob")]);
    gov.remove(&sub("alice"));
    assert_eq!(
        group.namespace_id(),
        before.as_slice(),
        "the communal namespace id is unchanged by a membership change",
    );
}

// Assertion 2 — a persona is a self-authorizing subspace: a write into your own
// subspace is authorized by your own signature alone; another key is rejected.
#[test]
fn persona_subspace_is_self_authorizing() {
    let alice = sub("alice");
    // A write into alice's subspace, signed by alice's own key, is authorized
    // with no Group Role grant (communal: authority is per-subspace ownership).
    let own = Write::authored(&alice, &alice.own_key(), b"hello");
    assert!(
        own.is_self_authorized(),
        "a persona writing its own subspace with its own key is self-authorizing",
    );
    // A write into alice's subspace signed by bob's key is not self-authorizing.
    let forged = Write::authored(&alice, &sub("bob").own_key(), b"forged");
    assert!(
        !forged.is_self_authorized(),
        "a write into another persona's subspace under the wrong key is rejected",
    );
}

// Assertion 3 — capability issuance is downstream of the folded Group Role.
#[test]
fn capability_issuance_is_downstream_of_the_folded_group_role() {
    let group = GroupPrincipal::of("drystone-group", "acme-coop");
    // alice holds the folded Group Role at genesis; carol is a plain member.
    let mut gov = Governance::genesis(&group, &[sub("alice"), sub("carol")]);
    gov.grant_role(&sub("alice"));

    assert!(
        gov.issue(&sub("alice")).is_some(),
        "a Role-holder can issue a capability (issuance is downstream of the fold)",
    );
    assert!(
        gov.issue(&sub("carol")).is_none(),
        "a member without the folded Group Role cannot issue a capability",
    );
}

// Assertion 4 — re-issue, never revoke-in-place, across a fold-driven authority
// change: the revoked persona's stale capability fails and a re-issued one succeeds.
#[test]
fn re_issue_not_revoke_in_place_across_a_fold_driven_authority_change() {
    let group = GroupPrincipal::of("drystone-group", "acme-coop");
    let (alice, bob) = (sub("alice"), sub("bob"));
    let mut gov = Governance::genesis(&group, &[alice.clone(), bob.clone()]);
    gov.grant_role(&alice);
    gov.grant_role(&bob);

    let cap_alice = gov.issue(&alice).expect("alice issues at gen 0");
    let cap_bob_v1 = gov.issue(&bob).expect("bob issues at gen 0");
    assert!(
        gov.capability_valid(&cap_alice),
        "cap valid before the change"
    );
    assert!(
        gov.capability_valid(&cap_bob_v1),
        "cap valid before the change"
    );

    // A governance-fold removal of alice advances the authority generation. This
    // is the ONLY revocation path (Meadowcap has no native revocation, and the
    // owned-namespace overwrite trick is structurally unavailable communally).
    gov.remove(&alice);

    // (a) The revoked persona's stale capability fails (subspace out of the set).
    assert!(
        !gov.capability_valid(&cap_alice),
        "the revoked persona's stale capability fails on the folded set",
    );
    // (b) A surviving member's pre-change capability is superseded, not overwritten.
    assert!(
        !gov.capability_valid(&cap_bob_v1),
        "the pre-change capability is superseded by the authority-generation advance",
    );
    assert_eq!(
        cap_bob_v1.issued_at_generation(),
        0,
        "re-issue, not revoke-in-place: the old capability object is unchanged (no overwrite)",
    );
    // (c) The re-issued capability at the new generation succeeds.
    let cap_bob_v2 = gov
        .issue(&bob)
        .expect("bob is re-issued at the new generation");
    assert!(
        gov.capability_valid(&cap_bob_v2),
        "the re-issued capability succeeds after the fold-driven authority change",
    );
    assert!(
        cap_bob_v2.issued_at_generation() > cap_bob_v1.issued_at_generation(),
        "the re-issue advances the generation (governance clock, never wall-clock)",
    );
}

// Assertion 5 — deterministic on both members: two members folding the same
// governance chain agree on every verdict, order-independently.
#[test]
fn deterministic_on_both_members_order_independent() {
    let group = GroupPrincipal::of("drystone-group", "acme-coop");
    let (alice, bob, carol) = (sub("alice"), sub("bob"), sub("carol"));
    let founders = [alice.clone(), bob.clone(), carol.clone()];

    // Member M1 removes alice then bob; member M2 removes bob then alice. Two
    // concurrent, independent removals fold order-independently.
    let mut m1 = Governance::genesis(&group, &founders);
    m1.grant_role(&carol);
    m1.remove(&alice);
    m1.remove(&bob);

    let mut m2 = Governance::genesis(&group, &founders);
    m2.grant_role(&carol);
    m2.remove(&bob);
    m2.remove(&alice);

    // Both members reach the same folded set and the same generation.
    assert_eq!(
        m1.members(),
        m2.members(),
        "both members fold to the identical surviving set regardless of removal order",
    );
    // And agree on capability validity: carol (survivor, Role-holder) can re-issue
    // a capability both accept; a capability minted against the pre-removal state
    // is rejected by both.
    let cap_carol_m1 = m1.issue(&carol).expect("carol re-issues on m1");
    assert!(
        m1.capability_valid(&cap_carol_m1) && m2.capability_valid(&cap_carol_m1),
        "both members accept the survivor's re-issued capability",
    );
    let stale = Governance::genesis(&group, &founders).issue_unchecked(&alice);
    assert!(
        !m1.capability_valid(&stale) && !m2.capability_valid(&stale),
        "both members reject the removed persona's stale capability",
    );
}

fn sub(name: &str) -> SubspaceId {
    // A persona's subspace id is folded from its device leaves to one lineage
    // identity (E.1); modeled here from the lineage name (the byte encoding is
    // [gates-release], not pinned).
    SubspaceId::from_leaves(&[&format!("{name}.laptop"), &format!("{name}.phone")], name)
}
