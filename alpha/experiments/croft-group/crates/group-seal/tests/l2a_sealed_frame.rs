//! L2a — the MLS-sealed happy-path frame (mechanism half of croft-group L2).
//!
//! Executes the six RED-able assertions shaped in `CROFT-GROUP-L2-READINESS.md`
//! §4 (backlog §3 L2a). Each is a failing test a build turns green, against real
//! openmls 0.8.1 at loopback grade, reusing the proven crates — `lineage-mls`
//! (the openmls `Device` wrapper `mls-welcome-over-iroh` reuses for the Welcome
//! and `mls-replant`/`replant-continuity` reuse for the re-plant re-key), the
//! pure `group-core` (depended ON, never the reverse), and the
//! `bip39-recovery-roundtrip` Zeroize/no-`Debug` secret pattern.
//!
//! Scope wall (firewall): these cover R1–R7 (seal, key/epoch state, Welcome key
//! distribution, membership-half re-key, credential identity, Zeroize,
//! governance-derived membership). They touch NO revocation-authority knob
//! (R10, I9), NO recovery-trust tier (R8-tier, I9), and NO resolution-ACL / fork
//! projection (R9, the parked croft-group L3). Assertion 6 guards that wall.

use group_core::{update, Effect, Intent, Model};
use group_seal::{EpochSecret, Sealer};
use zeroize::ZeroizeOnDrop;

/// Found a group and add two joiners over a real MLS Welcome; return
/// (founder, alice, bob), all in the same epoch. Reuses the add/Welcome path.
fn founded_trio() -> (Sealer, Sealer, Sealer) {
    let mut founder = Sealer::found("founder").expect("found genesis group");
    let mut alice = Sealer::enroll("alice").expect("enroll alice");
    let mut bob = Sealer::enroll("bob").expect("enroll bob");
    let kps = [
        alice.key_package().expect("alice kp"),
        bob.key_package().expect("bob kp"),
    ];
    let (_commit, welcome) = founder.invite(&kps).expect("invite alice+bob");
    alice.accept_welcome(&welcome).expect("alice joins");
    bob.accept_welcome(&welcome).expect("bob joins");
    (founder, alice, bob)
}

fn msg(text: &str) -> group_core::ChatMessage {
    group_core::ChatMessage {
        sender: "founder".to_string(),
        text: text.to_string(),
    }
}

// Assertion 1 — Seal != plaintext, and round-trips through real ciphertext.
#[test]
fn seal_is_ciphertext_and_round_trips() {
    let (mut founder, mut alice, _bob) = founded_trio();
    let message = msg("hi");
    let sealed = founder.seal(&message).expect("seal");

    // The sealed frame is real MLS ciphertext: the plaintext body never appears.
    assert!(
        !sealed.windows(2).any(|w| w == b"hi"),
        "the sealed frame must not contain the plaintext body 'hi'"
    );
    // And a keyed peer opens it back to the identical ChatMessage.
    assert_eq!(
        alice.open(&sealed).expect("alice opens the sealed frame"),
        message,
        "the sealed frame round-trips to the identical message on a keyed peer"
    );
}

// Assertion 2 — No key => observable drop, no panic; the core Model is unchanged.
#[test]
fn no_key_peer_drops_observably_without_panic() {
    let (mut founder, _alice, _bob) = founded_trio();
    let sealed = founder.seal(&msg("secret")).expect("seal");

    // A peer that never received a Welcome cannot open it — an observable typed
    // error, never a panic.
    let mut outsider = Sealer::enroll("outsider").expect("enroll outsider");
    assert!(
        outsider.open(&sealed).is_err(),
        "a no-key peer's open fails observably, not by panic"
    );

    // At the shell boundary the undecryptable frame is dropped via group-core's
    // existing hostile-input discipline: exactly one FrameDropped, Model unchanged.
    let before = Model::new();
    let (after, effects) = update(before.clone(), Intent::FrameReceived { bytes: sealed });
    assert_eq!(
        after, before,
        "a dropped frame leaves the core Model byte-identical"
    );
    assert_eq!(effects.len(), 1, "the drop surfaces exactly one effect");
    assert!(
        matches!(&effects[0], Effect::FrameDropped { reason } if !reason.is_empty()),
        "the drop is an observable FrameDropped with a non-empty reason"
    );
}

// Assertion 3 — Welcome distributes the read key (epoch_secret_match), end to end.
#[test]
fn welcome_distributes_the_read_key() {
    let mut founder = Sealer::found("founder").expect("found");
    let mut joiner = Sealer::enroll("joiner").expect("enroll");
    let (_commit, welcome) = founder
        .invite(&[joiner.key_package().expect("kp")])
        .expect("invite");
    joiner.accept_welcome(&welcome).expect("join from welcome");

    // The joiner derives the committer's exporter secret for the joined epoch.
    // Compared with `==` (not assert_eq!) so EpochSecret keeps its no-`Debug`
    // property — a Debug impl would be a way to log the secret.
    assert!(
        founder.epoch_secret().expect("founder secret")
            == joiner.epoch_secret().expect("joiner secret"),
        "join = key distribution: both derive the identical epoch secret",
    );
    // And can then decrypt a message sealed in that epoch.
    let sealed = founder.seal(&msg("welcome works")).expect("seal");
    assert_eq!(
        joiner.open(&sealed).expect("joiner opens"),
        msg("welcome works"),
        "the joiner decrypts a message sealed in the epoch the Welcome distributed",
    );
}

// Assertion 4 — Governed removal re-keys the departed reader out (PCS).
#[test]
fn governed_removal_re_keys_the_departed_reader_out() {
    let (mut founder, mut alice, mut bob) = founded_trio();

    // An authorized removal driven by the departed member's governed identity
    // (the same shape replant-continuity's e12_7_2_removal_propagates drives).
    let commit = founder.remove_member("alice").expect("remove alice");
    bob.apply_control(&commit)
        .expect("bob applies the removal commit");

    let sealed = founder
        .seal(&msg("after removal"))
        .expect("seal post-removal");
    // The retained member still reads; the removed member cannot (forward read
    // access lost — survivor-epoch re-key / PCS).
    assert_eq!(
        bob.open(&sealed).expect("retained member reads"),
        msg("after removal"),
        "a retained member decrypts the post-removal message",
    );
    assert!(
        alice.open(&sealed).is_err(),
        "the removed member cannot decrypt the post-removal message (PCS)",
    );
}

// Assertion 5 — Secrets never leak the pure core.
#[test]
fn secrets_stay_out_of_the_pure_core() {
    // The exporter/epoch secret newtype is Zeroize-on-drop (compile-time bound).
    fn assert_zeroize_on_drop<T: ZeroizeOnDrop>() {}
    assert_zeroize_on_drop::<EpochSecret>();

    // group-core carries no crypto/transport dependency — it stays WASM-clean,
    // so no secret bytes can enter an Effect or a WireError there.
    let core_manifest = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../group-core/Cargo.toml"
    ))
    .expect("read group-core manifest");
    for forbidden in ["openmls", "lineage-mls", "lineage-core", "zeroize", "iroh"] {
        assert!(
            !core_manifest.contains(forbidden),
            "group-core must not depend on {forbidden} (it stays pure and secret-free)",
        );
    }
}

// Assertion 6 — Firewall guard: no authority / projection surface in the API.
#[test]
fn firewall_guard_no_authority_or_projection_api() {
    let src = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"))
        .expect("read lib.rs");
    // Scan only the public API declarations, so the firewall prose in doc
    // comments (which legitimately names what is excluded) does not trip the guard.
    for line in src.lines() {
        let t = line.trim_start();
        if t.starts_with("pub fn ") || t.starts_with("pub struct ") || t.starts_with("pub enum ") {
            let lower = t.to_lowercase();
            for forbidden in [
                "revoke",
                "revocation",
                "authority",
                "quorum",
                "cosign",
                "co_sign",
                "recover",
                "recovery",
                "who_may",
                "trust_tier",
                "projection",
            ] {
                assert!(
                    !lower.contains(forbidden),
                    "L2a's public API must expose no `{forbidden}` surface (firewall: I9 / the parked resolution-ACL) — offending: {t}",
                );
            }
        }
    }
}
