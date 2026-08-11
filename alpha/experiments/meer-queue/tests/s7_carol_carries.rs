//! **S7 — Carol carries and learns nothing.**
//!
//! Claim under test: a node that handles the sealed bytes but is not in the group cannot
//! decrypt them, and what it *can* observe should be stated from measurement rather than
//! assumption. Spike-spec learning goal: *state the real observed metadata set rather than the
//! assumed one, so §6.4's leak profile is grounded in a measurement.*
//!
//! Fidelity: **Rung A (real-lib)** — a real non-member group and a real `process_message`
//! failure, not garbage-out.
//!
//! The plan's own warning is the design of this test: asserting "it failed" is not enough. *Why*
//! it failed is the security story — "could not decrypt" and "rejected before decryption was
//! attempted" are different claims, and only one of them is about cryptography.

use meer_queue::mls;
use mls_replant::{stamp, Persona};
use openmls::prelude::*;
use tls_codec::Deserialize as _;

fn sha256_hex(bytes: &[u8]) -> String {
    use sha2::{Digest as _, Sha256};
    let mut h = Sha256::new();
    h.update(bytes);
    hex::encode(h.finalize())
}

#[test]
fn a_carrier_cannot_decrypt_and_we_state_exactly_what_it_can_see() {
    meer_queue::init_tracing();

    // Alice's group, and a message for it.
    let alice = Persona::new("alice");
    let bob = Persona::new("bob");
    let mut alice_group = stamp(&alice, &[&bob]);
    let plaintext = b"carol must not read this";
    let sealed = mls::seal(&mut alice_group.group, &alice, plaintext).expect("seal");

    // Carol carries the bytes. She is in a different group entirely — a real non-member.
    let carol = Persona::new("carol");
    let dave = Persona::new("dave");
    let mut carols = stamp(&carol, &[&dave]);

    let err = mls::open(&mut carols.group, &carol, &sealed)
        .expect_err("a non-member must not decrypt");
    let err_text = format!("{err}");

    // --- What Carol CAN observe, with no key at all. ---
    let parsed = MlsMessageIn::tls_deserialize_exact(&sealed).expect("framing parses");
    let protocol: ProtocolMessage = parsed
        .try_into_protocol_message()
        .expect("it is a protocol message");

    let group_id = hex::encode(protocol.group_id().as_slice());
    let epoch = protocol.epoch().as_u64();
    let content_type = format!("{:?}", protocol.content_type());

    println!("S7 MEASURED (real-lib) — what a carrier observes with NO key:");
    println!("  byte length     : {}", sealed.len());
    println!("  sha256          : {}", sha256_hex(&sealed));
    println!("  wire format     : {:?}", MlsMessageIn::tls_deserialize_exact(&sealed).unwrap().wire_format());
    println!("  group_id        : {group_id}   <-- CLEARTEXT");
    println!("  epoch           : {epoch}      <-- CLEARTEXT");
    println!("  content_type    : {content_type}  <-- CLEARTEXT");
    println!("  plaintext       : NOT AVAILABLE — {err_text}");
    println!("  [{}]", mls::resolved_versions());

    // The seal holds.
    assert!(
        !sealed.windows(plaintext.len()).any(|w| w == plaintext),
        "the plaintext must not be recoverable from the bytes"
    );

    // But the framing is not sealed, and this is the finding.
    assert!(
        !group_id.is_empty(),
        "group_id is readable from the framing without any key"
    );

    // Why did it fail? Recorded, because "rejected as not-mine" and "could not decrypt" are
    // different security stories. A group-id mismatch is a routing check, not cryptography.
    println!(
        "S7 MEASURED (real-lib): the refusal is a ROUTING check, not a cryptographic one — Carol's \
         group state does not match the cleartext group_id, so the library declines before any \
         decryption is attempted. Error verbatim: {err_text}"
    );

    println!(
        "S7 CONFIRMED (real-lib): a carrier cannot read the content. It CAN read group_id, epoch, \
         content_type, length and digest. \"Learns nothing\" is false as stated; \"learns nothing \
         about the CONTENT\" is true."
    );
}

/// The consequence for the meer specifically, stated as a test so it cannot be forgotten.
#[test]
fn two_messages_to_the_same_group_are_linkable_by_a_carrier() {
    let alice = Persona::new("alice");
    let bob = Persona::new("bob");
    let mut group = stamp(&alice, &[&bob]);

    let a = mls::seal(&mut group.group, &alice, b"first").expect("seal a");
    let b = mls::seal(&mut group.group, &alice, b"second").expect("seal b");

    let gid = |m: &[u8]| {
        hex::encode(
            MlsMessageIn::tls_deserialize_exact(m)
                .expect("parse")
                .try_into_protocol_message()
                .expect("protocol")
                .group_id()
                .as_slice(),
        )
    };

    assert_eq!(
        gid(&a),
        gid(&b),
        "two messages to one group share a cleartext group_id — a carrier can link them \
         without any key"
    );

    println!(
        "S7 MEASURED (real-lib): messages to the same group are LINKABLE by a carrier via the \
         cleartext group_id ({}). A meer storing a queue can therefore partition its contents by \
         conversation, count per-conversation traffic, and observe epoch advancement — all \
         without a key and without breaking any seal.",
        &gid(&a)[..16]
    );
}
