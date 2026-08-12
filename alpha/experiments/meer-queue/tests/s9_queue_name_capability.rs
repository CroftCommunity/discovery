//! **S9 — the queue name as a capability, and what catch-up costs.**
//!
//! Follows the 2026-08-12 reshape: the meer is a **swarm participant** holding a *group's*
//! traffic (Part 2 §5.4), not an addressed node holding a person's. That raises the question the
//! addressed model never had to answer: **what entitles you to drain a group's queue?**
//!
//! The proposal under test: derive the **queue name** from the MLS group's exporter secret. Every
//! member derives the same name; a non-member cannot; and the meer — holding no key — cannot
//! either, so it needs **nothing in advance**. It stores under an opaque name handed to it.
//!
//! Fidelity: **Rung A (real-lib)** — real `MlsGroup::export_secret`, real commits, real epochs.
//!
//! # What this measures, and why the second one matters most
//!
//! 1. **Derivation** — members agree, non-members diverge, and the name rotates per epoch.
//! 2. **Catch-up cost.** The name is epoch-bound, so a member offline across N commits cannot
//!    name the newest queue. It works only because of an ordering property worth confirming
//!    rather than assuming: *the commit that moves the group from epoch E to E+1 is sent during
//!    epoch E*, so it sits in the queue you **can** already name. Catch-up is therefore a serial
//!    chain — drain E, process, derive E+1, drain E+1 — at **one round trip per missed epoch.**
//!
//! If that ordering did not hold, the whole design would deadlock: you would need the newest
//! secret to fetch the commits that produce it.

use meer_queue::mls;
use mls_replant::{join, stamp, Persona};
use openmls::prelude::*;

/// Domain-separated label, so this export can never be confused with another use of the
/// group's exporter secret.
const QUEUE_LABEL: &str = "croft/meer-queue/v1";
const NAME_LEN: usize = 32;

/// The queue name a member derives for its group's *current* epoch.
fn queue_name(group: &MlsGroup, who: &Persona) -> String {
    let secret = group
        .export_secret(who.provider.crypto(), QUEUE_LABEL, &[], NAME_LEN)
        .expect("export_secret");
    hex::encode(secret)
}

#[test]
fn members_agree_on_the_queue_name_and_non_members_cannot_derive_it() {
    let alice = Persona::new("alice");
    let bob = Persona::new("bob");
    let mut stamped = stamp(&alice, &[&bob]);
    let bob_group = join(
        &bob,
        stamped.welcome.clone().expect("welcome"),
        stamped.ratchet_tree.clone(),
    );

    let from_alice = queue_name(&stamped.group, &alice);
    let from_bob = queue_name(&bob_group, &bob);
    assert_eq!(
        from_alice, from_bob,
        "every member of a group must derive the SAME queue name"
    );
    assert_eq!(from_alice.len(), NAME_LEN * 2, "32 bytes, hex");

    // A different group derives a different name. This is what a non-member is: someone whose
    // group state cannot produce this value.
    let carol = Persona::new("carol");
    let dave = Persona::new("dave");
    let carols = stamp(&carol, &[&dave]);
    assert_ne!(
        queue_name(&carols.group, &carol),
        from_alice,
        "a non-member must not derive the group's queue name"
    );

    println!(
        "S9 CONFIRMED (real-lib): members agree on the queue name ({}…); a foreign group derives a \
         different one. The meer needs NO prior knowledge — it stores under an opaque name it \
         cannot compute. [{}]",
        &from_alice[..16],
        mls::resolved_versions()
    );
}

#[test]
fn the_queue_name_rotates_with_the_epoch() {
    let alice = Persona::new("alice");
    let bob = Persona::new("bob");
    let mut stamped = stamp(&alice, &[&bob]);

    let before = queue_name(&stamped.group, &alice);
    let epoch_before = stamped.group.epoch().as_u64();

    let (_, _commit) = mls_replant::commit(&mut stamped.group, &alice);
    let after = queue_name(&stamped.group, &alice);

    assert_ne!(
        before, after,
        "advancing the epoch must change the queue name — this is what makes rotation free"
    );
    println!(
        "S9 CONFIRMED (real-lib): the queue name rotates with the epoch ({} -> {}): {}… -> {}…. \
         Nobody tells the meer; it is simply a new queue.",
        epoch_before,
        stamped.group.epoch().as_u64(),
        &before[..12],
        &after[..12]
    );
}

/// **The load-bearing one.** Does catch-up across missed epochs actually chain, or deadlock?
#[test]
fn catching_up_across_missed_epochs_chains_one_round_trip_at_a_time() {
    const MISSED: usize = 5;

    let alice = Persona::new("alice");
    let bob = Persona::new("bob");
    let mut stamped = stamp(&alice, &[&bob]);
    let mut bob_group = join(
        &bob,
        stamped.welcome.clone().expect("welcome"),
        stamped.ratchet_tree.clone(),
    );

    // The name Bob knows when he goes offline.
    let bob_name_at_departure = queue_name(&bob_group, &bob);
    assert_eq!(bob_name_at_departure, queue_name(&stamped.group, &alice));

    // While Bob is away, Alice commits repeatedly. Each commit is SENT during the epoch it
    // closes, so it belongs to that epoch's queue — the ordering the whole design rests on.
    let mut wire: Vec<(String, MlsMessageOut)> = Vec::new();
    for _ in 0..MISSED {
        let name_now = queue_name(&stamped.group, &alice); // the queue this commit lands in
        let (_, commit) = mls_replant::commit(&mut stamped.group, &alice);
        wire.push((name_now, commit));
    }

    let alice_final = queue_name(&stamped.group, &alice);
    assert_ne!(
        alice_final, bob_name_at_departure,
        "the group has moved on; Bob cannot name the current queue"
    );

    // Bob returns. He can name exactly one queue: the one he left on. He must chain forward.
    let mut round_trips = 0usize;
    let mut asking_for = bob_name_at_departure.clone();
    for (queue_of, commit) in &wire {
        round_trips += 1;
        assert_eq!(
            &asking_for, queue_of,
            "round trip {round_trips}: Bob must be able to NAME the queue holding the next commit \
             — if this fails the design deadlocks (you would need the newest secret to fetch the \
             commits that produce it)"
        );
        // He drains it, applies the commit, and only now can derive the next name.
        mls_replant::apply_commit(&mut bob_group, &bob, commit);
        asking_for = queue_name(&bob_group, &bob);
    }

    assert_eq!(round_trips, MISSED, "one round trip per missed epoch");
    assert_eq!(
        asking_for, alice_final,
        "after chaining, Bob names the current queue and is caught up"
    );

    println!(
        "S9 MEASURED (real-lib): catch-up across {MISSED} missed epochs took {round_trips} serial \
         round trip(s) — ONE PER MISSED EPOCH, in order. It does not deadlock, and the reason is an \
         ordering property: the commit closing epoch E is sent DURING epoch E, so it sits in the \
         queue the returning member can already name. Cost: a member returning after N commits \
         pays N sequential fetches before it can read anything newer."
    );
}

/// The honest limit: an opaque name is access control, not privacy.
#[test]
fn the_opaque_name_does_not_hide_the_group_from_the_meer() {
    use tls_codec::Deserialize as _;

    let alice = Persona::new("alice");
    let bob = Persona::new("bob");
    let mut stamped = stamp(&alice, &[&bob]);

    let sealed = mls::seal(&mut stamped.group, &alice, b"routed by an opaque name").expect("seal");
    let name = queue_name(&stamped.group, &alice);

    // The meer files under the opaque name…
    assert_eq!(name.len(), NAME_LEN * 2);
    // …but `group_id` is still readable from the payload with no key at all (S7).
    let gid = hex::encode(
        MlsMessageIn::tls_deserialize_exact(&sealed)
            .expect("parse")
            .try_into_protocol_message()
            .expect("protocol")
            .group_id()
            .as_slice(),
    );
    assert!(!gid.is_empty());
    assert_ne!(gid, name, "they are different values, and both are visible");

    println!(
        "S9 MEASURED (real-lib): the queue name ({}…) is NOT the group_id ({}…). The name rotates \
         per epoch and is unguessable; `group_id` is cleartext in the envelope and stable for the \
         group's life. So the name buys ACCESS CONTROL, not unlinkability — the meer can still \
         link across epochs via the payload. Closing that is E96 (nested sealing) — and note the \
         queue name is what makes nested sealing POSSIBLE, since it gives the meer something to \
         route on that is not the MLS framing.",
        &name[..12],
        &gid[..12]
    );
}
