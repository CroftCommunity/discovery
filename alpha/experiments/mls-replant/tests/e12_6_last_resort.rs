//! E12.6 — KeyPackage availability is bounded by the last-resort package: the swap never
//! blocks (Battery 7, Rung A).
//!
//! Earns/bounds: Part 2 §7.6.3 — KeyPackage availability is bounded by the last-resort package: the re-plant swap never blocks.
//!
//! When a member's fresh KeyPackage is not fetchable at boundary time, the last-resort
//! package (marked reusable) seats them anyway, so the swap does not block on an unreachable
//! member — it trades that member's key refresh for availability until they republish. So a
//! member seated via a *reused* last-resort package does NOT rotate their leaf key (the E12.5
//! exception), while a member seated via a fresh package does.
//!
//! Falsifies if: seating via last-resort fails (the swap blocks), or a last-resort-seated
//! member's leaf key rotates (it should not, being reused), or a fresh-package member's leaf
//! key fails to rotate.

use std::collections::HashMap;

use mls_replant::{leaf_keys, stamp_kps, Persona};

#[test]
fn last_resort_package_seats_without_blocking_and_does_not_rotate() {
    let planter = Persona::new("planter");
    let lr = Persona::new("last-resort-member"); // fresh package "unavailable" at boundary
    let fresh = Persona::new("fresh-member");

    // lr publishes ONE last-resort package; it is reused across both boundaries.
    let lr_kp = lr.last_resort_key_package();

    // Boundary A: seat lr via its last-resort package (its fresh package is unreachable) and
    // fresh via a fresh package.
    let a = stamp_kps(&planter, vec![lr_kp.clone(), fresh.key_package()]);
    // Boundary B: a fresh re-plant. lr is again seated via the SAME last-resort package;
    // fresh via a NEW fresh package.
    let b = stamp_kps(&planter, vec![lr_kp.clone(), fresh.key_package()]);

    // The swap never blocked: both boundaries seated all three (planter + lr + fresh).
    assert_eq!(a.member_count, 3, "boundary A seated everyone via last-resort");
    assert_eq!(b.member_count, 3, "boundary B seated everyone via the reused last-resort");

    let enc_a: HashMap<Vec<u8>, Vec<u8>> = leaf_keys(&a.group).into_iter().collect();
    let enc_b: HashMap<Vec<u8>, Vec<u8>> = leaf_keys(&b.group).into_iter().collect();

    // The last-resort member did NOT rotate (reused package) — the E12.5 exception.
    assert_eq!(
        enc_a.get(&lr.id), enc_b.get(&lr.id),
        "a member seated via a REUSED last-resort package must keep the same leaf key (no \
         rotation) — availability traded for refresh"
    );
    // The fresh-package member DID rotate — confirming the exception is exactly and only the
    // last-resort case.
    assert_ne!(
        enc_a.get(&fresh.id), enc_b.get(&fresh.id),
        "a member seated via a fresh package must rotate — the last-resort exception is not \
         leaking to fresh members"
    );

    eprintln!(
        "E12.6 RESULT (corroboration): the last-resort package seated the member at both \
         boundaries (the swap never blocked), and — being reused — its leaf key did NOT \
         rotate, while the fresh-package member's did. Availability is bounded by last-resort; \
         the only cost is that member's deferred refresh."
    );
}
