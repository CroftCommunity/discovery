//! E12.7 (keystone) — the MLS re-plant stamps *exactly* the member set the governance fold
//! derives, at every step of a sequence of authorized changes.
//!
//! This is the Rung-B claim the whole re-plant story rests on: the crypto membership is a
//! *function of* the governance chain, never an independent source of truth. Genesis seats the
//! founder; each authorized add extends the derived set; and at every step the fresh stamp seats
//! precisely those principals — no more (a stray seat would be an over-broad group), no fewer (a
//! missing seat would lock a governed member out of the keys).
//!
//! Falsifies if: the stamped principal set ever differs from the fold's derived member set.

use replant_continuity::{restamp, stamped_principals, Chain, Roster};

/// The invariant under test: whoever the fold derives is exactly whom the stamp seats.
fn assert_stamp_tracks(chain: &Chain, roster: &Roster) {
    let derived = chain.derived_members();
    let stamp = restamp(roster, &derived).expect("non-empty group stamps");
    let stamped = stamped_principals(roster, &stamp);
    assert_eq!(
        derived, stamped,
        "the re-plant must stamp exactly the fold's derived member set — derived {} members, \
         stamped {}",
        derived.len(),
        stamped.len()
    );
}

#[test]
fn stamp_tracks_the_fold_across_authorized_adds() {
    let roster = Roster::of(5);
    let mut chain = Chain::new(&roster);

    // Genesis: member 0 founds the group with all thresholds = 1 (a lone Owner authorizes each
    // change). The fold seats the founder as Owner.
    chain.genesis(&roster.members[0], [1, 1, 1, 1]).expect("genesis");
    assert_eq!(chain.derived_members().len(), 1, "genesis seats exactly the founder");
    assert_stamp_tracks(&chain, &roster);

    // Each authorized add extends the derived set; the stamp must track it every time.
    // Roles: 2 = Member, 1 = Admin.
    let owner = &roster.members[0];
    chain.add(owner, &roster.members[1], 2).expect("add m1");
    assert_stamp_tracks(&chain, &roster);

    chain.add(owner, &roster.members[2], 2).expect("add m2");
    assert_stamp_tracks(&chain, &roster);

    chain.add(owner, &roster.members[3], 2).expect("add m3");
    assert_stamp_tracks(&chain, &roster);

    chain.add(owner, &roster.members[4], 1).expect("add m4 as admin");
    assert_stamp_tracks(&chain, &roster);

    assert_eq!(chain.derived_members().len(), 5, "all five members are now governed and seated");

    eprintln!(
        "E12.7 RESULT (corroboration): across genesis + four authorized adds, the MLS re-plant \
         stamped exactly the fold's derived member set at every step — the crypto membership is a \
         function of the governance chain, never an independent authority. Rung B holds for the \
         happy path."
    );
}
