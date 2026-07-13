//! E12.7 — an authorized **removal** propagates through the re-plant: the removed principal is
//! gone from the fold's derived set *and* absent from the fresh stamp's crypto membership.
//!
//! This is the load-bearing half of "removal is real". In MLS a removal must actually re-key the
//! group so the removed member cannot read on; the re-plant achieves this by stamping a fresh
//! group over the *post-removal* derived set. If the removed principal still seated, they would
//! retain a live leaf — the removal would be governance theatre. This test forbids that.
//!
//! Falsifies if: after an authorized remove, the removed principal is still in the derived set, or
//! still seated in the re-stamp, or the surviving set does not match between fold and stamp.

use replant_continuity::{restamp, stamped_principals, Chain, Roster};

#[test]
fn authorized_removal_drops_the_member_from_both_fold_and_stamp() {
    let roster = Roster::of(4);
    let mut chain = Chain::new(&roster);
    let owner = &roster.members[0];

    chain.genesis(owner, [1, 1, 1, 1]).expect("genesis");
    chain.add(owner, &roster.members[1], 2).expect("add m1");
    chain.add(owner, &roster.members[2], 2).expect("add m2");
    chain.add(owner, &roster.members[3], 2).expect("add m3");

    // Sanity: all four seated, fold and stamp agree.
    let before = chain.derived_members();
    assert_eq!(before.len(), 4, "four members before removal");
    let stamp_before = restamp(&roster, &before).expect("stamp");
    assert_eq!(before, stamped_principals(&roster, &stamp_before), "fold==stamp before removal");
    assert!(
        before.contains(&roster.members[2].principal),
        "m2 is a member before removal"
    );

    // Owner removes m2 (authorized: Owner + remove_threshold=1).
    chain.remove(owner, &roster.members[2]).expect("remove m2");

    let after = chain.derived_members();
    assert_eq!(after.len(), 3, "three members after removal");
    assert!(
        !after.contains(&roster.members[2].principal),
        "the fold dropped m2 from the derived set"
    );

    // The fresh stamp over the post-removal set must NOT seat m2, and must match the fold exactly.
    let stamp_after = restamp(&roster, &after).expect("stamp after removal");
    let stamped_after = stamped_principals(&roster, &stamp_after);
    assert_eq!(after, stamped_after, "the re-plant stamps exactly the surviving set");
    assert!(
        !stamped_after.contains(&roster.members[2].principal),
        "the removed member is NOT seated in the fresh stamp — removal re-keys them out"
    );

    eprintln!(
        "E12.7 RESULT (corroboration): an authorized removal propagated through the re-plant — m2 \
         left the fold's derived set and was absent from the fresh stamp's crypto membership. \
         Removal is real: the re-plant re-keys the departed member out, it is not governance \
         theatre."
    );
}
