//! §2e follow-on (RUN-11) — the subspace-derivation half, `Design → green-real`.
//!
//! Part 3's Design model (`seam.rs`) modeled the persona → subspace fold from
//! lineage names. This test moves the **subspace-derivation half** of E.1 to
//! `green-real` by reusing the `Verified` lineage fold
//! (`lineage-mls::Device::fold_by_lineage`; the RUN-08 `fold_matches` result)
//! on **real openmls 0.8.1 leaves**: a persona's several device leaves fold to
//! ONE subspace identity, computed from the real leaf credentials every member
//! holds. This is the "derive a per-persona `SubspaceId` by folding a persona's
//! multiple MLS leaves to one lineage identity" step, against real crypto.
//!
//! Scope kept: the `SubspaceId` **byte encoding** stays `[gates-release]`
//! (Appendix B / E.1) — this earns the grade for the *fold*, not the wire form —
//! and no trust tier is decided (I9). No production code is added; this reuses a
//! `Verified` primitive as green-real evidence.

use lineage_core::ids::Did;
use lineage_mls::{Device, LineageClaim};

// The green-real E.1 fold: a persona's device leaves collapse to one subspace.
#[test]
fn a_personas_device_leaves_fold_to_one_subspace_over_real_openmls() {
    // Two personae, each a key-lineage; alice runs two devices, bob one.
    let alice = Did::new("lineage-alice");
    let bob = Did::new("lineage-bob");
    // A lineage claim binds each device leaf to its persona lineage (T1).
    let claim = |lineage: &Did, seed: u64, device: &str| {
        let root = lineage_core::keys::SigningIdentity::from_seed(lineage.clone(), seed);
        LineageClaim::sign(&root, &Did::new(device))
    };

    let mut founder =
        Device::new_with_lineage(Did::new("alice.laptop"), claim(&alice, 20, "alice.laptop"))
            .expect("alice.laptop device");
    founder.create_group().expect("genesis group");
    let phone = Device::new_with_lineage(Did::new("alice.phone"), claim(&alice, 20, "alice.phone"))
        .expect("alice.phone device");
    let bob_dev = Device::new_with_lineage(Did::new("bob.laptop"), claim(&bob, 21, "bob.laptop"))
        .expect("bob.laptop device");
    founder
        .add(&[
            phone.key_package().expect("alice.phone kp"),
            bob_dev.key_package().expect("bob kp"),
        ])
        .expect("add the second device + bob");

    // Fold the real leaf credentials by lineage. Three device leaves, but two
    // personae => TWO subspaces: alice's two leaves collapse to one.
    let subspaces = founder
        .fold_by_lineage()
        .expect("the group folds by lineage");
    assert_eq!(
        subspaces.len(),
        2,
        "two personae fold to two subspaces (not three device leaves)",
    );
    assert_eq!(
        subspaces
            .get("lineage:lineage-alice")
            .map(Vec::len)
            .unwrap_or_default(),
        2,
        "a persona's two real MLS device leaves fold to one subspace identity (E.1, green-real)",
    );
    assert_eq!(
        subspaces
            .get("lineage:lineage-bob")
            .map(Vec::len)
            .unwrap_or_default(),
        1,
        "a single-device persona is its own subspace",
    );

    // The fold is deterministic: recomputed from the same leaf credentials, every
    // member derives the identical subspace partition (the prerequisite for a
    // consistent SubspaceId across members — the byte encoding stays [gates-release]).
    let again = founder.fold_by_lineage().expect("re-fold");
    assert_eq!(subspaces, again, "the subspace fold is deterministic");
}
