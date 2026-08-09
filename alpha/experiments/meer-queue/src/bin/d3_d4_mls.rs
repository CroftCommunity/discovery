//! **D3 + D4 probe** — real OpenMLS application messages, and the re-frame question.
//!
//! D4: does `mls-replant` compose, and does `create_message` accept its persona state?
//!     Disposition: `promote` — becomes `src/mls.rs` in Phase 2.
//!
//! D3: can an `MlsMessageIn` be re-serialized, and **does a re-frame actually change the
//!     bytes**? This is M2's negative arm. The plan is explicit that the API shape must be
//!     read, not remembered — and that if openmls offers no such path, that absence is
//!     itself the result.
//!     Disposition: `promote` — becomes the negative arm in Phase 6.

use mls_replant::{join, stamp, Persona};
use openmls::prelude::*;
use tls_codec::{Deserialize as _, Serialize as _};

fn sha256_hex(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let mut h = Sha256::new();
    h.update(bytes);
    hex::encode(h.finalize())
}

fn main() {
    println!("=== D4: real OpenMLS application message via mls-replant personae ===");

    let alice = Persona::new("alice");
    let bob = Persona::new("bob");

    // D4 (a): group construction reuses the Rung-A ancestor.
    let mut stamped = stamp(&alice, &[&bob]);
    println!("group stamped: members = {}", stamped.member_count);

    // D4 (b): Bob joins from the real Welcome + ratchet tree.
    let mut bob_group = join(
        &bob,
        stamped.welcome.clone().expect("welcome"),
        stamped.ratchet_tree.clone(),
    );
    println!("bob joined:    epoch = {}", bob_group.epoch().as_u64());

    // D4 (c): the question the plan flagged — does `create_message` accept
    // `persona.provider` / `persona.signer` directly? (Fields are `pub`.)
    let plaintext = b"the conversation stays alive while you sleep";
    let sealed: MlsMessageOut = stamped
        .group
        .create_message(&alice.provider, &alice.signer, plaintext)
        .expect("create_message");
    let sealed_bytes = sealed.tls_serialize_detached().expect("ser");
    println!(
        "sealed:        {} bytes, digest {}",
        sealed_bytes.len(),
        &sha256_hex(&sealed_bytes)[..16]
    );

    // Is it really a PrivateMessage on the wire, and is the plaintext absent?
    let parsed = MlsMessageIn::tls_deserialize_exact(&sealed_bytes).expect("de");
    println!("wire_format:   {:?}", parsed.wire_format());
    let leaks = sealed_bytes
        .windows(plaintext.len())
        .any(|w| w == plaintext);
    println!("plaintext in ciphertext: {leaks}");

    // D4 (d): Bob opens it with the real library.
    let protocol: ProtocolMessage = MlsMessageIn::tls_deserialize_exact(&sealed_bytes)
        .expect("de")
        .try_into_protocol_message()
        .expect("protocol message");
    let processed = bob_group
        .process_message(&bob.provider, protocol)
        .expect("process_message");
    match processed.into_content() {
        ProcessedMessageContent::ApplicationMessage(app) => {
            let got = app.into_bytes();
            println!(
                "bob decrypted: {:?} — matches = {}",
                String::from_utf8_lossy(&got),
                got == plaintext
            );
        }
        other => println!("UNEXPECTED content: {other:?}"),
    }

    println!("\n=== D3: the re-frame — is it reachable, and does it change the bytes? ===");
    reframe_probe(&sealed_bytes);
}

/// D3 proper. In a **default build** of openmls 0.8.1 the conversions that would let a
/// forwarder decode-and-re-encode a message are compiled out:
///
/// ```text
/// // The following two `From` implementations break abstraction layers and MUST
/// // NOT be made available outside of tests or "test-utils".
/// #[cfg(any(feature = "test-utils", test))]
/// impl From<MlsMessageIn> for MlsMessageOut { ... }
/// ```
/// (`src/framing/message_out.rs:195-211`; the same gate and comment on
/// `From<PrivateMessageIn> for PrivateMessage`, `src/framing/private_message_in.rs:263-277`.)
///
/// So this probe reports which paths compile under the feature set we actually built with.
#[cfg(feature = "reframe")]
fn reframe_probe(sealed_bytes: &[u8]) {
    let msg_in = MlsMessageIn::tls_deserialize_exact(sealed_bytes).expect("de");
    let round_tripped: MlsMessageOut = msg_in.into();
    let re_encoded = round_tripped.tls_serialize_detached().expect("re-ser");
    println!("re-encode reachable: YES (openmls `test-utils` enabled)");
    println!("  application message:");
    println!("    original   {} bytes  {}", sealed_bytes.len(), sha256_hex(sealed_bytes));
    println!("    re-encoded {} bytes  {}", re_encoded.len(), sha256_hex(&re_encoded));
    println!("    BYTES IDENTICAL: {}", re_encoded == sealed_bytes);

    // Does the result generalise beyond PrivateMessage? A commit is the other object
    // the meer would carry, and it is framed differently.
    let carol = Persona::new("carol");
    let dave = Persona::new("dave");
    let mut s = stamp(&carol, &[&dave]);
    let (_, commit_msg) = mls_replant::commit(&mut s.group, &carol);
    let commit_bytes = commit_msg.tls_serialize_detached().expect("ser commit");
    let commit_rt: MlsMessageOut = MlsMessageIn::tls_deserialize_exact(&commit_bytes)
        .expect("de commit")
        .into();
    let commit_re = commit_rt.tls_serialize_detached().expect("re-ser commit");
    println!("  commit ({:?}):", MlsMessageIn::tls_deserialize_exact(&commit_bytes).unwrap().wire_format());
    println!("    original   {} bytes  {}", commit_bytes.len(), sha256_hex(&commit_bytes));
    println!("    re-encoded {} bytes  {}", commit_re.len(), sha256_hex(&commit_re));
    println!("    BYTES IDENTICAL: {}", commit_re == commit_bytes);

    // (Welcome not probed here: `mls_replant::Stamp` keeps the *extracted* `Welcome`, not
    // its `MlsMessageOut` wire form, and openmls has no `From<Welcome> for MlsMessageOut`.
    // Phase 11 constructs Welcomes directly and can re-check there if it matters.)
}

#[cfg(not(feature = "reframe"))]
fn reframe_probe(_sealed_bytes: &[u8]) {
    println!("re-encode reachable: NO — `From<MlsMessageIn> for MlsMessageOut` and");
    println!("  `From<PrivateMessageIn> for PrivateMessage` are both");
    println!("  #[cfg(any(feature = \"test-utils\", test))] in openmls 0.8.1, each carrying");
    println!("  the comment: \"breaks abstraction layers and MUST NOT be made available");
    println!("  outside of tests or test-utils\".");
    println!("  => In a default build the forwarder CANNOT re-frame. Re-run with");
    println!("     --features reframe to measure whether a re-encode changes the bytes.");
}
