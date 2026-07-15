# BIP39 paper-recovery round-trip — the Tier-1 lock (RUN-08 Part 1C)

Serves: Part 2 §7.3.9 (the recovery-anchor's Tier-1 *lock* mechanism — recoveryKey ⇄ 24-word
BIP39 mnemonic, then secretbox-wrap the masterKey) — earns/bounds: `Verified` at
experiment-grade (real crypto, in-process; not a `[gates-release]` crate choice) — register:
none — landed: RUN-08.

`RUN-08 Part 1C, 2026-07-15. Rust 1.94.1. Standalone spike (own lockfile = own pin):
bip39 =2.2.2, dryoc =0.8.0, zeroize =1.9.0.`

## What this is

The **cheapest first step** of the total-device-loss recovery model, sketched in
`EXPERIMENT-BACKLOG.md §6g` and confirmed as a build direction in
`beta/drystone-spec/open-threads.md §2` ("build the lock now"). Recovery splits into two
separable tiers:

- **Tier 1 — the lock** (a *mechanism*: sealed key material, reconstructable from a paper
  artifact). Buildable now. **This spike.**
- **Tier 2 — the trust** (who and when a release is legitimate). The genuinely-unsolved-in-general
  social problem — the identity/key-recovery trust tier, MASTER-INDEX **I9**, the owner's open
  call. **Untouched here.**

This spike proves the lock exists and round-trips bit-exact. It says nothing about who may open
it. Per the RUN-08 firewall it contains **no share-splitting, no release predicate, no threshold
anything** — just the two-primitive lock.

## The two primitives

1. **recoveryKey (32 B) ⇄ 24-word BIP39 English mnemonic.** A 256-bit key is exactly a 24-word
   mnemonic. `recovery_key_to_mnemonic` encodes; `mnemonic_to_recovery_key` decodes and validates
   the BIP39 checksum on the way in.
2. **masterKey secretbox-wrapped under the recoveryKey.** `wrap_master_key` seals the 32-byte
   masterKey with XSalsa20-Poly1305 (`dryoc` — the same vetted secretbox `altdrive-core` uses),
   `nonce(24) ‖ tag(16) ‖ sealed(32)`; `unwrap_master_key` recovers it. A wrong key or a tampered
   blob fails with an authentication error — the plaintext is never surfaced on failure.

Secret material (`RecoveryKey`, `MasterKey`) is `Zeroize` + `ZeroizeOnDrop` and carries no
`Debug` impl, per the `altdrive-core` secret-handling discipline.

## Run

```sh
cd alpha/experiments/bip39-recovery-roundtrip
cargo test          # 11 tests
cargo clippy --all-targets
```

## Results (RUN-08)

**PASS — 11/11 green, clippy clean.**

- **A. Round-trip bit-exact.** `recovery_key_round_trips_bit_exact`: a 32-byte key → 24-word
  mnemonic → 32-byte key, byte-for-byte identical.
- **B. Standard BIP39 English KATs, both directions.** `bip39_english_kats_pass_both_directions`
  over the four canonical 256-bit Trezor vectors:
  | entropy | 24th (checksum) word |
  |---|---|
  | `00…00` | `art` |
  | `7f…7f` | `title` |
  | `80…80` | `bless` |
  | `ff…ff` | `vote` |
  Forward (entropy → the known-answer mnemonic) and backward (mnemonic → the exact entropy, i.e.
  the checksum validates) both hold. *(The KAT earned its keep: an initial transcription of the
  `80…80` checksum word as `bunker` was caught by the forward assertion and corrected to the
  official `bless`.)*
- **C. Checksum-failure negatives rejected — never silently accepted.** Four negatives, each an
  `Err`, none decoding to a wrong key:
  - `corrupted_word_is_rejected` — `abandon`×24 (the checksum word swapped) → `InvalidChecksum`.
  - `transposed_pair_is_rejected` — the `7f…7f` vector with its first two words swapped → rejected
    (both words remain in the wordlist, so only the checksum can catch it — and it does).
  - `out_of_wordlist_word_is_rejected` — a non-wordlist word → `UnknownWord`.
  - `wrong_word_count_is_rejected` — 23 words → `BadWordCount`.
- **D. secretbox wrap/unwrap + clean failure.** `master_key_wrap_unwrap_bit_exact` (round-trips),
  `wrap_uses_fresh_nonce_but_recovers_identically` (two wraps differ, both unwrap), plus the
  clean-failure trio: `wrong_key_unwrap_fails_cleanly`, `tampered_ciphertext_fails_cleanly`,
  `malformed_wrap_is_rejected` — each an `Err`, never a silently-wrong plaintext.

## Scope discipline (what this does NOT do)

- **No trust tier.** No decision on who may trigger a release, no threshold shares, no quorum, no
  revoke-authority. That is I9 — the owner's open call (RUN-08 firewall).
- **Experiment-grade crate choice.** `bip39` / `dryoc` are pinned exact for a reproducible spike;
  they are **not** a `[gates-release]` decision. The release-final mnemonic library, memory-hard
  KDF, and AEAD are deferred to the recovery-anchor prototype (Part 2 §7.3.9's
  `[to be determined by the recovery-anchor prototype]`).
- **No key generation / no KDF.** The spike operates on caller-supplied 32-byte keys; it does not
  derive `recoveryKey` from a passphrase (that KDF is the recovery-anchor prototype's job) and does
  not source entropy.
- **In-process only.** No wire, no persistence, no networking.
