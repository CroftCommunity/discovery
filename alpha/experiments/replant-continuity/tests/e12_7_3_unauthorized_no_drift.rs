//! E12.7 — an **unauthorized** membership change does not drift the crypto membership. The fold
//! rejects it at `ingest`, the derived set is unchanged, and the fresh stamp therefore never
//! seats the would-be member.
//!
//! This is the adversarial complement to the happy path: it proves the fold — not the MLS layer —
//! is the sole authority on membership. A member who is not authorized to add (here: a principal
//! that was never seated, so has no role) cannot cause a persona to be seated by the re-plant,
//! because the re-plant only ever stamps what the fold *derived*, and the fold refused the change.
//!
//! Falsifies if: the unauthorized add is accepted (`ingest` returns `Ok`), or the derived set
//! changes, or the interloper's principal appears in the stamp.

use replant_continuity::{restamp, stamped_principals, Chain, Roster};

#[test]
fn unauthorized_add_is_rejected_and_never_seats() {
    let roster = Roster::of(4);
    let mut chain = Chain::new(&roster);
    let owner = &roster.members[0];

    chain.genesis(owner, [1, 1, 1, 1]).expect("genesis");
    chain.add(owner, &roster.members[1], 2).expect("add m1 (authorized)");

    let baseline = chain.derived_members();
    assert_eq!(baseline.len(), 2, "owner + m1");

    // m3 is NOT a member (never added) and therefore has no role. It attempts to add m2. The fold
    // must reject this at ingest — a non-member cannot grant membership.
    let interloper = &roster.members[3];
    let victim_target = &roster.members[2];
    let result = chain.add(interloper, victim_target, 2);
    assert!(
        result.is_err(),
        "the fold must REJECT an add authored by a non-member — got {result:?}"
    );

    // The derived set is unchanged: neither the interloper nor its target was seated.
    let after = chain.derived_members();
    assert_eq!(after, baseline, "an unauthorized add leaves the derived member set unchanged");
    assert!(!after.contains(&victim_target.principal), "the unauthorized target is not seated");
    assert!(!after.contains(&interloper.principal), "the interloper did not seat itself");

    // The stamp tracks the (unchanged) derived set — the interloper's push never reaches crypto.
    let stamp = restamp(&roster, &after).expect("stamp");
    let stamped = stamped_principals(&roster, &stamp);
    assert_eq!(after, stamped, "the re-plant stamps exactly the fold's (unchanged) set");
    assert!(
        !stamped.contains(&victim_target.principal) && !stamped.contains(&interloper.principal),
        "no unauthorized principal is seated in the fresh stamp"
    );

    eprintln!(
        "E12.7 RESULT (corroboration): an unauthorized add was rejected by the fold at ingest; the \
         derived member set did not move, and the fresh stamp seated no unauthorized principal. \
         The governance fold — not the MLS layer — is the sole authority on membership."
    );
}
