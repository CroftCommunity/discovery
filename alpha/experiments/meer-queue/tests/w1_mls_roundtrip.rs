//! **Phase 2 wiring test** — the seal is real OpenMLS, end to end.
//!
//! Every claim in this spike that touches confidentiality or byte-identity runs against the
//! real library. The methodology names the forbidden shortcut explicitly (XOR-as-MLS), so
//! this test's job is to prove the seal is genuine before anything is built on top of it.
//!
//! What it pins, and why:
//!
//! - **Round trip.** Alice seals, Bob opens, plaintext matches. The baseline M1 depends on.
//! - **The bytes are real MLS framing.** `wire_format() == PrivateMessage`, not merely
//!   "the plaintext isn't in there" — that weaker assertion is satisfied by *any*
//!   transformation, including a placeholder cipher. This is the assertion that makes the
//!   test specific to a genuine seal (Pass 3, mutation resistance).
//! - **`open` returns `Err`, never panics, for a message a group cannot decrypt.** S7 (Carol
//!   carries and learns nothing) must *record the named error*, which is impossible if the
//!   failure path unwinds. Pinned here so the signature cannot regress to a panic.
//! - **The printed version banner matches the actual pins.** The methodology requires every
//!   result to print exact resolved versions; a banner that silently drifts from `Cargo.toml`
//!   would be worse than none.

use meer_queue::mls;
use mls_replant::{join, stamp, Persona};
use openmls::prelude::*;
use tls_codec::Deserialize as _;

#[test]
fn a_sealed_message_round_trips_through_the_real_library() {
    let alice = Persona::new("alice");
    let bob = Persona::new("bob");
    let mut stamped = stamp(&alice, &[&bob]);
    let mut bob_group = join(
        &bob,
        stamped.welcome.clone().expect("welcome"),
        stamped.ratchet_tree.clone(),
    );

    let plaintext = b"the conversation stays alive while you sleep";
    let sealed = mls::seal(&mut stamped.group, &alice, plaintext).expect("seal");

    // Genuine MLS framing, not merely "not the plaintext".
    let parsed = MlsMessageIn::tls_deserialize_exact(&sealed).expect("sealed bytes parse as MLS");
    assert_eq!(
        parsed.wire_format(),
        WireFormat::PrivateMessage,
        "the seal must produce a real MLS PrivateMessage"
    );
    assert!(
        !sealed.windows(plaintext.len()).any(|w| w == plaintext),
        "plaintext must not appear in the sealed bytes"
    );

    let opened = mls::open(&mut bob_group, &bob, &sealed).expect("open");
    assert_eq!(opened, plaintext, "Bob must recover exactly what Alice sealed");
}

#[test]
fn a_group_that_cannot_decrypt_gets_an_error_not_a_panic() {
    // Carol's group is a different group entirely; Alice's message means nothing to it.
    // S7 needs to *record* what the library says here, so this must be a value, not an unwind.
    let alice = Persona::new("alice");
    let bob = Persona::new("bob");
    let mut stamped = stamp(&alice, &[&bob]);
    let sealed = mls::seal(&mut stamped.group, &alice, b"not for carol").expect("seal");

    let carol = Persona::new("carol");
    let dave = Persona::new("dave");
    let mut carols = stamp(&carol, &[&dave]);

    let result = mls::open(&mut carols.group, &carol, &sealed);
    let err = result.expect_err("a foreign group must not decrypt this message");
    assert!(
        !format!("{err}").is_empty(),
        "the error must be reportable — S7 records it verbatim"
    );
}

#[test]
fn the_version_banner_matches_the_actual_pins() {
    let banner = mls::resolved_versions();
    for pin in [
        "openmls =0.8.1",
        "openmls_rust_crypto =0.5.1",
        "openmls_basic_credential =0.5.0",
        "openmls_traits =0.5.0",
    ] {
        assert!(
            banner.contains(pin),
            "banner must report {pin}; got:\n{banner}"
        );
    }

    // The guard that matters: the banner is hand-written, so prove it still agrees with the
    // manifest rather than trusting that nobody bumped a pin without updating the string.
    let manifest = include_str!("../Cargo.toml");
    for (crate_name, version) in [
        ("openmls", "=0.8.1"),
        ("openmls_rust_crypto", "=0.5.1"),
        ("openmls_basic_credential", "=0.5.0"),
        ("openmls_traits", "=0.5.0"),
    ] {
        let line = manifest
            .lines()
            .find(|l| l.trim_start().starts_with(&format!("{crate_name} =")))
            .unwrap_or_else(|| panic!("{crate_name} not pinned in Cargo.toml"));
        assert!(
            line.contains(version),
            "Cargo.toml pins {crate_name} differently from the banner: {line}"
        );
    }
}
