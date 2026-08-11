//! **M2 — byte-identical forwarding, and the negative case.** The second must-pass claim.
//!
//! Claim under test: the meer stores and forwards sealed bytes **unchanged**
//! (Part 2 §6.6.2, the `MUST` on byte-identical storage).
//!
//! Fidelity: **Rung A (real-lib)** throughout.
//!
//! # The negative arm was falsified before it was written
//!
//! The spike spec hypothesised that a re-framed copy — decode and re-encode without changing
//! semantic content — is *detectably different at the byte level*. Phase 0's D3 probe showed
//! it is **not**: a re-encode is byte-identical, because TLS-codec serialization is canonical.
//! So this file does not assert what the spec asked for. It asserts what is true, and records
//! the falsification.
//!
//! What survives is a **stronger** result than the spec claimed. Three things hold:
//!
//! 1. In a default build the conversion does not exist — openmls gates
//!    `From<MlsMessageIn> for MlsMessageOut` and `From<PrivateMessageIn> for PrivateMessage`
//!    behind `test-utils`, each with the comment *"break abstraction layers and MUST NOT be
//!    made available outside of tests"*. A forwarder cannot re-frame even if it wanted to.
//! 2. Forced open (`--features reframe`), the re-encode is byte-identical — so a re-frame is
//!    not a way to produce a different-but-valid copy either.
//! 3. The operation that *would* break the seal is **re-sealing**, and that needs a key the
//!    meer does not have.
//!
//! The `MUST` therefore has teeth for a simpler reason than the spec gave: a blind forwarder
//! has no route to a semantically-equivalent-but-byte-different copy at all.

use std::sync::Arc;

use meer_queue::ciss_harness::CissHarness;
use meer_queue::mls;
use meer_queue::transport::{MeerClient, MeerServer};
use mls_replant::{join, stamp, Persona};

fn sha256_hex(bytes: &[u8]) -> String {
    use sha2::{Digest as _, Sha256};
    let mut h = Sha256::new();
    h.update(bytes);
    hex::encode(h.finalize())
}

#[tokio::test]
async fn the_digest_is_stable_across_store_and_serve() {
    meer_queue::init_tracing();

    let alice = Persona::new("alice");
    let bob = Persona::new("bob");
    let mut alice_group = stamp(&alice, &[&bob]);
    let mut bob_group = join(
        &bob,
        alice_group.welcome.clone().expect("welcome"),
        alice_group.ratchet_tree.clone(),
    );

    let ciss = Arc::new(CissHarness::spawn().await);
    let server = MeerServer::spawn(Arc::clone(&ciss)).await.expect("meer");
    let alice_client = MeerClient::connect(server.relay_url()).await.expect("alice");
    let bob_client = MeerClient::connect(server.relay_url()).await.expect("bob");

    // (1) at production.
    let sealed = mls::seal(&mut alice_group.group, &alice, b"unchanged in transit").expect("seal");
    let at_production = sha256_hex(&sealed);

    // (2) after the meer's PUT. CISS's content address IS the sha256 of the stored bytes, so
    // the address it returns is directly comparable to the digest we computed.
    let stored = alice_client
        .deposit(server.addr(), &sealed, &[bob_client.recipient_id()])
        .await
        .expect("deposit");
    assert_eq!(
        stored.to_string(),
        at_production,
        "CISS's content address must equal the digest of what we handed it"
    );

    // (3) after CISS's re-verify-on-read, and (4) at Bob's receive, before any decode.
    let drained = bob_client.drain(server.addr(), &[]).await.expect("drain");
    assert_eq!(drained.len(), 1);
    let at_receive = sha256_hex(&drained[0]);
    assert_eq!(
        at_receive, at_production,
        "the digest must be stable across store + serve"
    );
    assert_eq!(drained[0], sealed, "and the bytes themselves must be equal");

    // Only now decode — the digest chain above never touched the MLS layer.
    let opened = mls::open(&mut bob_group, &bob, &drained[0]).expect("open");
    assert_eq!(opened, b"unchanged in transit");

    println!(
        "M2 positive arm CONFIRMED (real-lib): digest {} stable at production, after PUT, \
         after CISS re-verify-on-read, and at receive. [{}]",
        &at_production[..16],
        mls::resolved_versions()
    );

    bob_client.close().await;
    alice_client.close().await;
    server.shutdown().await;
    ciss.shutdown().await;
}

/// The meer must not be able to name what it carries.
///
/// **This is a source-level lint, not a toolchain guarantee, and the difference matters.** The
/// plan called for a `cargo tree` assertion that `meer.rs` reaches no openmls crate — but
/// `cargo tree` resolves *crates*, and `meer.rs` is a module in a crate that also contains
/// `mls.rs`. Crate-granularity tooling cannot see a module-granularity boundary, so the plan's
/// mechanism does not exist as specified.
///
/// What would make this a real guarantee is splitting the meer into its own crate with no
/// openmls dependency in its manifest. That is recorded as a recommendation for the meer lane's
/// Phase 2 (the real gateway service), where the meer is a separate process anyway. For a
/// spike, the lint plus the module's own imports is proportionate — and saying which one this
/// is beats implying the stronger one.
#[test]
fn the_meer_module_cannot_name_an_mls_type() {
    for (path, src) in [
        ("src/meer.rs", include_str!("../src/meer.rs")),
        ("src/queue.rs", include_str!("../src/queue.rs")),
    ] {
        for line in src.lines() {
            let code = line.split("//").next().unwrap_or("");
            for forbidden in ["openmls", "mls_replant", "crate::mls", "MlsMessage"] {
                assert!(
                    !code.contains(forbidden),
                    "{path} references `{forbidden}` in code: {line}\n\
                     The meer must not be able to name what it carries — that is the structural \
                     form of M2's positive arm."
                );
            }
        }
    }
}

/// Forced open, a re-frame is byte-identical. Run with `--features reframe`.
#[cfg(feature = "reframe")]
#[test]
fn a_forced_reframe_is_byte_identical_falsifying_the_spec_hypothesis() {
    use openmls::prelude::*;
    use tls_codec::Deserialize as _;

    let alice = Persona::new("alice");
    let bob = Persona::new("bob");
    let mut group = stamp(&alice, &[&bob]);
    let mut bob_group = join(
        &bob,
        group.welcome.clone().expect("welcome"),
        group.ratchet_tree.clone(),
    );

    let sealed = mls::seal(&mut group.group, &alice, b"re-frame me").expect("seal");
    let reframed = mls::reframe(&sealed).expect("reframe");

    assert_eq!(
        sha256_hex(&reframed),
        sha256_hex(&sealed),
        "SPEC HYPOTHESIS FALSIFIED: a re-framed copy is byte-IDENTICAL, not detectably different"
    );

    // And because it is the same bytes, Bob processes it normally. There is no rejection to
    // record, because there is nothing to reject.
    let opened = mls::open(&mut bob_group, &bob, &reframed).expect("open the re-framed copy");
    assert_eq!(opened, b"re-frame me");

    // The transformation genuinely ran: the round trip really did decode to an MLS type.
    let parsed = MlsMessageIn::tls_deserialize_exact(&reframed).expect("parses");
    assert_eq!(parsed.wire_format(), WireFormat::PrivateMessage);

    println!(
        "M2 negative arm FALSIFIED-AS-SPECIFIED (real-lib): a forced decode/re-encode is \
         byte-identical ({}), so a re-framed copy is NOT detectably different. The MUST stands; \
         its stated rationale does not — the hazard is re-SEALING, which needs a key the meer \
         lacks. [{}]",
        &sha256_hex(&sealed)[..16],
        mls::resolved_versions()
    );
}

/// In a default build the re-frame does not exist at all.
#[cfg(not(feature = "reframe"))]
#[test]
fn the_reframe_path_is_absent_from_a_default_build() {
    // Constant by construction — this test only compiles when the feature is off, which is the
    // assertion. Stated as a compile-time fact rather than a runtime one clippy will call vacuous.
    const _: () = assert!(!cfg!(feature = "reframe"));
    // The conversions this would need are `#[cfg(any(feature = \"test-utils\", test))]` in
    // openmls 0.8.1 (framing/message_out.rs:195-211, framing/private_message_in.rs:263-277),
    // each carrying: "break abstraction layers and MUST NOT be made available outside of tests".
    println!(
        "M2 structural (real-lib): re-frame unreachable in a default build — openmls gates both \
         conversions behind `test-utils` with an explicit MUST NOT. [{}]",
        mls::resolved_versions()
    );
}
