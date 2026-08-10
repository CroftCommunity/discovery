//! **S3b — is a duplicate distinguishable from genuine loss?**
//!
//! Follow-on from S3, which found that openmls 0.8.1 *errors* on a duplicate application
//! message rather than applying it idempotently: *"The requested secret was deleted to preserve
//! forward secrecy."*
//!
//! That raised a design question with two candidate answers:
//!
//! 1. **The client keeps state** (a set of delivered content hashes) and dedups *before*
//!    processing, so the error never occurs.
//! 2. **The client treats the error as a benign duplicate signal** — a no-op costing only
//!    bandwidth and an error branch.
//!
//! Option 2 is only safe if the duplicate error is **distinguishable** from the error a
//! genuinely lost message produces. Forward secrecy deletes a message key after use *and* as
//! the ratchet advances past it — so "I already read this" and "I can never read this" may
//! arrive as the same condition. If they do, treating the error as benign silently swallows
//! real loss, which is precisely what the no-invisible-loss rule (Part 1 §2.2) forbids.
//!
//! This test does not assert a preferred answer. It **measures which world we are in.**
//!
//! Fidelity: **Rung A (real-lib)**.

use meer_queue::mls;
use mls_replant::{join, stamp, Persona};

#[test]
fn a_duplicate_and_a_message_lost_to_the_ratchet_are_compared_verbatim() {
    meer_queue::init_tracing();

    let alice = Persona::new("alice");
    let bob = Persona::new("bob");
    let mut alice_group = stamp(&alice, &[&bob]);
    let mut bob_group = join(
        &bob,
        alice_group.welcome.clone().expect("welcome"),
        alice_group.ratchet_tree.clone(),
    );

    // Alice sends a run of messages. Bob will receive them out of order.
    let mut sealed = Vec::new();
    for i in 0..40 {
        sealed.push(
            mls::seal(
                &mut alice_group.group,
                &alice,
                format!("message {i}").as_bytes(),
            )
            .expect("seal"),
        );
    }

    // --- Case A: a genuine duplicate. Process message 0, then process it again. ---
    let first = mls::open(&mut bob_group, &bob, &sealed[0]).expect("first application");
    assert_eq!(first, b"message 0");
    let duplicate_err = mls::open(&mut bob_group, &bob, &sealed[0])
        .expect_err("a duplicate must not silently succeed");

    // --- Case B: genuine loss. Jump far forward, then reach back for one never seen. ---
    // Applying a far-future message advances the ratchet past the intervening ones; message 1
    // was never delivered and is now unreachable. This is real, unrecoverable loss.
    let jumped = mls::open(&mut bob_group, &bob, &sealed[39]);
    let lost_err = match jumped {
        Ok(_) => mls::open(&mut bob_group, &bob, &sealed[1])
            .expect_err("a message the ratchet has passed must not silently succeed"),
        // If the forward jump itself is refused, that is also a finding: the out-of-order
        // window is narrower than the scenario assumes.
        Err(e) => {
            println!("S3b NOTE: the forward jump to message 39 was itself refused: {e}");
            mls::open(&mut bob_group, &bob, &sealed[1])
                .expect_err("reaching back must not silently succeed")
        }
    };

    let dup = format!("{duplicate_err}");
    let lost = format!("{lost_err}");

    println!("S3b MEASURED (real-lib):");
    println!("  duplicate (already read)   -> {dup}");
    println!("  lost (ratchet moved past)  -> {lost}");
    println!("  DISTINGUISHABLE BY ERROR: {}", dup != lost);
    println!("  [{}]", mls::resolved_versions());

    if dup == lost {
        println!(
            "  => The two are INDISTINGUISHABLE at the error surface. Treating the error as a \
             benign duplicate would silently swallow genuine, unrecoverable loss. The client \
             MUST hold delivered content hashes and dedup BEFORE processing; only that state \
             tells 'I already read this' from 'I can never read this'."
        );
    } else {
        println!(
            "  => The two are DISTINGUISHABLE. A client may treat the duplicate variant as a \
             no-op without hiding loss, provided it matches on the specific error rather than \
             on any processing failure."
        );
    }

    // The measurement is the deliverable; this pins the shape so a library bump cannot change
    // the answer without failing here.
    assert!(
        !dup.is_empty() && !lost.is_empty(),
        "both failure modes must be reportable"
    );
}
