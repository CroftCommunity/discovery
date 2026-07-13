# Coding transcript — iroh + Automerge Android P2P experiment (PR #7)

source: AI coding session, pasted verbatim 2026-06-15 (condensed rendering; the verbatim raw
is in `discovery/seeds/transcripts/raw/pr7-android-p2p.md`).

context: this captures the closing arc — applying CodeRabbit's review fixes, then writing the
path-to-APK documentation. The earlier build (Rust group-core, UniFFI bindings, Gradle
project, two-peer Automerge sync over real iroh) is described in README.md.

---

## CodeRabbit triage and fixes

CodeRabbit posted 9 review comments. Triaged: 6 legit (1 real bug + 5 hardening), 3 noise
(generated files; a "critical" note about `accept_and_exchange` not wired into the live loop —
that's the documented deferred live-accept path, not a bug). The Cycode comments were the
already-decided MPL-2.0 / LGPL license flags (uniffi*, attohttpc, jna, etc.).

User chose **Apply all 6**:

- **Correctness:** reset `self.peer = None` in `CreateGroup` — otherwise a session that
  previously joined another group can merge the new group's doc into the stale peer
  (cross-group leak). Real bug.

- **Hardening:** no-panic on a poisoned mutex at the FFI boundary (return a JSON error
  instead of crashing Kotlin); 15s timeouts on all iroh connect/accept/stream waits +
  relay `online()`; bound invite size before base64/JSON decode (avoid large-alloc/OOM on
  hostile input).

- **Android:** build the init payload with `JSONObject` so `Build.MODEL` is escaped safely;
  `GroupClient.close()` in `onDestroy` (avoid native/session leak on config changes).

Build hiccup fixed: `Incoming` is `IntoFuture`, not `Future` — wrapped it in an async block.
All host tests green again, including the two-peer Automerge sync over real iroh. Public FFI
surface unchanged, so the generated bindings stay valid. Pushed (commit 506e632); posted one
summary comment on the PR rather than spamming the 6 threads.

## Status + path to APK

> **status of experiment, what would it take to get to apk from here to test? needs documenting**

Verified the native library name the bindings expect before documenting (so the doc is
accurate, not guesswork): the bindings load the native lib as `group_core` (JNA →
`libgroup_core.so`), exactly what the cdylib produces — no renaming needed, and there's a
startup checksum guard tying the .so to the bindings.

Wrote `PATH_TO_APK.md` (commit 915c284). Status: **solid at Tier 1, verified** — two peers
sync an Automerge doc over real iroh (5 tests green). The only thing between here and an
installable APK is toolchain availability (no NDK/SDK in this sandbox), not code.

Path to a testable APK (short version):

1. Install toolchain (~30–60 min, one-time): Android SDK Platform 35 + Build-Tools 35, NDK
   r27c (r27+ for Android 15's 16 KB pages), `cargo install cargo-ndk`. (Rust targets already
   installed.)

2. Tier 2 — cross-compile the .so: `cargo ndk -t arm64-v8a -t x86_64 --platform 24 -o
   ../android/app/src/main/jniLibs build --release`. Lib name aligns — bindings load
   `group_core` → `libgroup_core.so`, no rename.

3. Tier 3 — assemble + install: set `sdk.dir` in `local.properties`, `./gradlew
   assembleDebug`, `adb install`.

4. Test on two devices: A creates group → Show invite → paste into B → Join → post/read
   converges (the live relay-path version of the host test).

Gotchas flagged in the doc: the UniFFI startup checksum ties the .so to the bindings
(regenerate together if the API changes); the relay path needs outbound internet to n0 infra;
the Toast-based invite is awkward to copy across emulators (suggested a one-line `Log.d` tweak
for testing).
