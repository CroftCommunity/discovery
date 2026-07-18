//! P3 — state at rest, resumption, and eviction honesty (native half; the
//! wasm host-kill drill is `node/resume.mjs`, driven by `make p3-resume`).

use group_core::ChatMessage;
use seal_persist::{BlobStore, FileStore, PersistSealer};

fn msg(sender: &str, text: &str) -> ChatMessage {
    ChatMessage {
        sender: sender.to_string(),
        text: text.to_string(),
    }
}

const KEY: [u8; 16] = *b"run19-at-rest-k!";

/// Setup: alice founds, bob joins; returns both in the same epoch.
fn founded_pair() -> (PersistSealer, PersistSealer) {
    let mut alice = PersistSealer::found("did:example:alice").expect("found");
    let mut bob = PersistSealer::enroll("did:example:bob").expect("enroll");
    let kp = bob.key_package().expect("kp");
    let (_c, welcome) = alice.invite(&kp).expect("invite");
    bob.accept_welcome(&welcome).expect("welcome");
    (alice, bob)
}

/// Resumption: state through the blob and back mid-conversation; the
/// restored member decrypts the NEXT message and — after an epoch roll it
/// only hears about post-restore — the next epoch's traffic too.
#[test]
fn snapshot_restore_resumes_across_epoch_roll() {
    let (mut alice, mut bob) = founded_pair();
    assert_eq!(
        bob.open(&alice.seal(&msg("alice", "pre")).expect("seal"))
            .expect("open")
            .text,
        "pre"
    );

    // Bob's state goes to rest; the in-memory member is dropped (the kill).
    let blob = bob.snapshot(&KEY).expect("snapshot");
    drop(bob);

    // While bob is dead: another message AND an epoch roll (carol added).
    let missed = alice.seal(&msg("alice", "while you were down")).expect("seal");
    let mut carol = PersistSealer::enroll("did:example:carol").expect("enroll");
    let kp_c = carol.key_package().expect("kp");
    let (commit, welcome_c) = alice.invite(&kp_c).expect("invite");
    carol.accept_welcome(&welcome_c).expect("welcome");
    let post_roll = alice.seal(&msg("alice", "next epoch says hi")).expect("seal");

    // Reload from the blob: catches up on the message, folds the commit,
    // and decrypts the next epoch's traffic.
    let mut bob2 = PersistSealer::restore(&KEY, &blob).expect("restore");
    assert_eq!(bob2.open(&missed).expect("open missed").text, "while you were down");
    bob2.apply_control(&commit).expect("fold the roll");
    assert_eq!(bob2.open(&post_roll).expect("open post-roll").text, "next epoch says hi");
    assert_eq!(bob2.epoch().expect("e"), alice.epoch().expect("e"));
    assert_eq!(
        bob2.epoch_secret().expect("s"),
        alice.epoch_secret().expect("s")
    );
}

/// The blob is encrypted at rest: no plaintext identity or state markers in
/// the bytes, and the wrong key does not open it.
#[test]
fn blob_is_opaque_and_key_bound() {
    let (_alice, bob) = founded_pair();
    let blob = bob.snapshot(&KEY).expect("snapshot");

    // The serialized state would carry the DID and openmls storage-key labels
    // in clear; the encrypted blob must not.
    for marker in [b"did:example:bob".as_slice(), b"MlsGroup".as_slice(), b"Epoch".as_slice()] {
        assert!(
            !blob.windows(marker.len()).any(|w| w == marker),
            "at-rest blob leaks plaintext marker {:?}",
            String::from_utf8_lossy(marker)
        );
    }

    let wrong = *b"wrong-key-000000";
    assert!(PersistSealer::restore(&wrong, &blob).is_err());
}

/// The storage trait's file backing round-trips and destroys
/// (`SPEC-DELTA[run19-storage-shim]` — the in-environment stand-in for
/// IndexedDB/OPFS + WebCrypto key wrapping).
#[test]
fn file_store_roundtrip_and_destroy() {
    let dir = tempfile::tempdir().expect("tmpdir");
    let mut store = FileStore::new(dir.path().join("bob.state"));
    assert!(store.get().expect("empty get").is_none());
    store.put(b"ciphertext blob").expect("put");
    assert_eq!(store.get().expect("get").as_deref(), Some(b"ciphertext blob".as_slice()));
    store.destroy().expect("destroy");
    assert!(store.get().expect("post-destroy get").is_none());
}

/// EVICTION drill: destroy the blob entirely — the member cannot
/// self-restore (forward secrecy is not overridden by any recovery path
/// here), and re-entry is a FRESH add via Welcome, blind to the gap.
#[test]
fn eviction_destroyed_blob_means_rejoin_blind_to_gap() {
    let (mut alice, bob) = founded_pair();

    // Bob's device state is at rest, then DESTROYED.
    let dir = tempfile::tempdir().expect("tmpdir");
    let mut store = FileStore::new(dir.path().join("bob.state"));
    store.put(&bob.snapshot(&KEY).expect("snapshot")).expect("put");
    drop(bob);
    store.destroy().expect("destroy");

    // Nothing to restore from: the documented recovery path does not exist.
    assert!(store.get().expect("get").is_none());
    assert!(PersistSealer::restore(&KEY, &[]).is_err());

    // Traffic sealed during the gap.
    let gap_msg = alice.seal(&msg("alice", "sealed into the gap")).expect("seal");

    // Re-entry: a FRESH enrollment + Welcome (a new leaf, not a resurrection).
    let mut bob2 = PersistSealer::enroll("did:example:bob").expect("enroll");
    let kp = bob2.key_package().expect("kp");
    let (_c, welcome) = alice.invite(&kp).expect("re-invite");
    bob2.accept_welcome(&welcome).expect("welcome");

    // The re-joined member reads NEW traffic…
    let now = alice.seal(&msg("alice", "welcome back")).expect("seal");
    assert_eq!(bob2.open(&now).expect("open").text, "welcome back");
    // …but is BLIND to the gap: the missed ciphertext does not open.
    assert!(
        bob2.open(&gap_msg).is_err(),
        "re-joined member read into the gap — forward secrecy overridden"
    );
}
