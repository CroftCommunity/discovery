//! Part 1C (RUN-08) — the Tier-1 paper-recovery lock, KAT-verified round-trip.
//!
//! Earns the recovery-anchor's **Tier-1 lock mechanism** (`open-threads.md §2` "build the
//! lock now"; `EXPERIMENT-BACKLOG.md §6g`; Part 2 §7.3.9 recovery-anchor prototype). Green
//! here proves four things and nothing about the *trust* tier (who may open the lock — I9):
//!
//!  A. **recoveryKey ⇄ 24-word mnemonic is bit-exact.** A 32-byte key encodes to a 24-word
//!     BIP39 English mnemonic and decodes back byte-for-byte identical.
//!  B. **The standard BIP39 English-wordlist KATs pass**, both directions, on the canonical
//!     256-bit Trezor vectors (all-zero, 0x7f, 0x80, all-ones entropy).
//!  C. **Checksum-failure negatives are rejected, never silently accepted** — a corrupted
//!     word, a transposed pair, an out-of-wordlist word, and a wrong word count each fail
//!     validation rather than decoding to a wrong key.
//!  D. **masterKey secretbox-wrapped under the recoveryKey unwraps bit-exact, and a wrong
//!     key (or tampered blob) fails cleanly** — an authentication error, never a
//!     silently-wrong plaintext.
//!
//! Experiment-grade; the crate/version choices are a spike pin, not a `[gates-release]`
//! decision (see the crate-root doc and README).

use bip39_recovery_roundtrip::{
    mnemonic_to_recovery_key, recovery_key_to_mnemonic, unwrap_master_key, wrap_master_key,
    MasterKey, RecoveryError, RecoveryKey, KEY_BYTES, WRAP_OVERHEAD,
};

/// 24 words: `word` repeated 23 times, then `last` (the checksum word).
fn repeated(word: &str, last: &str) -> String {
    let mut w = vec![word; 23];
    w.push(last);
    w.join(" ")
}

/// Canonical 256-bit BIP39 English test vectors (Trezor `vectors.json`, 24-word entries).
/// Each is `(entropy, expected 24-word mnemonic)`. The repeated-word vectors are built
/// programmatically to rule out a transcription miscount; only the checksum word (`art`,
/// `title`, `bunker`, `vote`) is asserted literally — which is exactly what the KAT pins.
fn kats() -> Vec<([u8; KEY_BYTES], String)> {
    let block_7f = "legal winner thank year wave sausage worth useful";
    let mnemonic_7f =
        format!("{block_7f} {block_7f} legal winner thank year wave sausage worth title");
    let block_80 = "letter advice cage absurd amount doctor acoustic avoid";
    let mnemonic_80 =
        format!("{block_80} {block_80} letter advice cage absurd amount doctor acoustic bless");

    vec![
        ([0x00u8; KEY_BYTES], repeated("abandon", "art")),
        ([0x7fu8; KEY_BYTES], mnemonic_7f),
        ([0x80u8; KEY_BYTES], mnemonic_80),
        ([0xffu8; KEY_BYTES], repeated("zoo", "vote")),
    ]
}

// ---- A. recoveryKey ⇄ mnemonic round-trip is bit-exact -----------------------------------

#[test]
fn recovery_key_round_trips_bit_exact() {
    // A deterministic, non-trivial key (no RNG needed): 0,1,2,…,31.
    let mut bytes = [0u8; KEY_BYTES];
    for (i, b) in bytes.iter_mut().enumerate() {
        *b = i as u8;
    }
    let key = RecoveryKey::from_bytes(bytes);

    let mnemonic = recovery_key_to_mnemonic(&key);
    assert_eq!(
        mnemonic.split_whitespace().count(),
        24,
        "a 32-byte key must encode to exactly 24 words"
    );

    let recovered = mnemonic_to_recovery_key(&mnemonic).expect("valid mnemonic must decode");
    assert_eq!(
        recovered.as_bytes(),
        key.as_bytes(),
        "recoveryKey → mnemonic → recoveryKey must be byte-for-byte identical"
    );
}

// ---- B. the standard BIP39 English KATs pass, both directions -----------------------------

#[test]
fn bip39_english_kats_pass_both_directions() {
    for (entropy, expected_mnemonic) in kats() {
        let key = RecoveryKey::from_bytes(entropy);

        // Forward: entropy → mnemonic equals the known-answer string.
        let got = recovery_key_to_mnemonic(&key);
        assert_eq!(
            got, expected_mnemonic,
            "KAT forward mismatch for entropy {entropy:02x?}"
        );

        // Backward: the known-answer mnemonic decodes to the exact entropy (checksum ok).
        let back = mnemonic_to_recovery_key(&expected_mnemonic)
            .expect("KAT mnemonic must pass checksum validation");
        assert_eq!(
            back.as_bytes(),
            &entropy,
            "KAT backward mismatch for entropy {entropy:02x?}"
        );
    }
}

// ---- C. checksum-failure negatives are rejected, never silently accepted ------------------

#[test]
fn corrupted_word_is_rejected() {
    // The all-zero vector is "abandon"×23 + "art". Swapping the checksum word "art" for
    // "abandon" (24×abandon) is a valid-wordlist phrase with the WRONG checksum.
    let corrupted = vec!["abandon"; 24].join(" ");
    let err = match mnemonic_to_recovery_key(&corrupted) {
        Ok(_) => panic!("bad checksum must be rejected"),
        Err(e) => e,
    };
    match err {
        RecoveryError::Bip39(msg) => assert!(
            msg.contains("checksum"),
            "expected a checksum rejection, got: {msg}"
        ),
        other => panic!("expected Bip39 checksum error, got {other:?}"),
    }
}

#[test]
fn transposed_pair_is_rejected() {
    // Take a valid vector and swap its first two words. Both words remain in the wordlist,
    // so the only way this can be caught is the checksum — and it MUST be caught.
    let (entropy, mnemonic) = kats()[1].clone(); // the 0x7f vector
    let mut words: Vec<&str> = mnemonic.split_whitespace().collect();
    words.swap(0, 1);
    let transposed = words.join(" ");
    assert_ne!(transposed, mnemonic, "the swap must actually change the phrase");

    let result = mnemonic_to_recovery_key(&transposed);
    assert!(
        result.is_err(),
        "a transposed pair must be rejected, not silently accepted"
    );
    // And it certainly must not decode back to the original key.
    if let Ok(k) = result {
        assert_ne!(k.as_bytes(), &entropy);
    }
}

#[test]
fn out_of_wordlist_word_is_rejected() {
    let (_entropy, mnemonic) = kats()[0].clone();
    let mut words: Vec<&str> = mnemonic.split_whitespace().collect();
    words[5] = "zzzz"; // not a BIP39 English word
    let bad = words.join(" ");
    let err = match mnemonic_to_recovery_key(&bad) {
        Ok(_) => panic!("unknown word must be rejected"),
        Err(e) => e,
    };
    assert!(matches!(err, RecoveryError::Bip39(_)));
}

#[test]
fn wrong_word_count_is_rejected() {
    let (_entropy, mnemonic) = kats()[0].clone();
    let mut words: Vec<&str> = mnemonic.split_whitespace().collect();
    words.pop(); // 23 words — not a valid BIP39 length
    let short = words.join(" ");
    let err = match mnemonic_to_recovery_key(&short) {
        Ok(_) => panic!("a 23-word phrase must be rejected"),
        Err(e) => e,
    };
    assert!(matches!(err, RecoveryError::Bip39(_)));
}

// ---- D. masterKey secretbox-wrap / unwrap, and wrong-key failure --------------------------

#[test]
fn master_key_wrap_unwrap_bit_exact() {
    let recovery = RecoveryKey::from_bytes([0xA5u8; KEY_BYTES]);
    let master = MasterKey::from_bytes({
        let mut b = [0u8; KEY_BYTES];
        for (i, x) in b.iter_mut().enumerate() {
            *x = (200 - i) as u8;
        }
        b
    });

    let wrapped = wrap_master_key(&recovery, &master);
    assert_eq!(
        wrapped.len(),
        WRAP_OVERHEAD + KEY_BYTES,
        "wrap layout = nonce(24) + tag(16) + sealed key(32)"
    );

    let unwrapped = unwrap_master_key(&recovery, &wrapped).expect("correct key must unwrap");
    assert_eq!(
        unwrapped.as_bytes(),
        master.as_bytes(),
        "masterKey must unwrap byte-for-byte identical"
    );
}

#[test]
fn wrap_uses_fresh_nonce_but_recovers_identically() {
    let recovery = RecoveryKey::from_bytes([0x11u8; KEY_BYTES]);
    let master = MasterKey::from_bytes([0x22u8; KEY_BYTES]);

    let a = wrap_master_key(&recovery, &master);
    let b = wrap_master_key(&recovery, &master);
    assert_ne!(a, b, "a fresh random nonce must make each wrap distinct");

    assert_eq!(
        unwrap_master_key(&recovery, &a).unwrap().as_bytes(),
        master.as_bytes()
    );
    assert_eq!(
        unwrap_master_key(&recovery, &b).unwrap().as_bytes(),
        master.as_bytes()
    );
}

#[test]
fn wrong_key_unwrap_fails_cleanly() {
    let right = RecoveryKey::from_bytes([0x01u8; KEY_BYTES]);
    let wrong = RecoveryKey::from_bytes([0x02u8; KEY_BYTES]);
    let master = MasterKey::from_bytes([0x42u8; KEY_BYTES]);

    let wrapped = wrap_master_key(&right, &master);
    let err = match unwrap_master_key(&wrong, &wrapped) {
        Ok(_) => panic!("wrong key must not unwrap"),
        Err(e) => e,
    };
    assert_eq!(
        err,
        RecoveryError::Unwrap,
        "a wrong key must fail authentication cleanly, never return a plaintext"
    );
}

#[test]
fn tampered_ciphertext_fails_cleanly() {
    let recovery = RecoveryKey::from_bytes([0x7Eu8; KEY_BYTES]);
    let master = MasterKey::from_bytes([0x55u8; KEY_BYTES]);

    let mut wrapped = wrap_master_key(&recovery, &master);
    let last = wrapped.len() - 1;
    wrapped[last] ^= 0x01; // flip one bit of the ciphertext

    let err = match unwrap_master_key(&recovery, &wrapped) {
        Ok(_) => panic!("tamper must be caught"),
        Err(e) => e,
    };
    assert_eq!(err, RecoveryError::Unwrap);
}

#[test]
fn malformed_wrap_is_rejected() {
    let recovery = RecoveryKey::from_bytes([0x09u8; KEY_BYTES]);
    let too_short = vec![0u8; WRAP_OVERHEAD - 1];
    let err = match unwrap_master_key(&recovery, &too_short) {
        Ok(_) => panic!("too-short blob must be rejected"),
        Err(e) => e,
    };
    assert!(matches!(err, RecoveryError::MalformedWrap { .. }));
}
