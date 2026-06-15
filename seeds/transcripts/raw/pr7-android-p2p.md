# Raw transcript — PR #7 android-p2p-app (verbatim)

source: https://github.com/croftc/SecurityPolicy/pull/7 · pasted 2026-06-15 · verbatim

---

Used 3 tools
Check runs unchanged (Secrets ✓, Licenses ✗ ignored). The review-comments response is now large (59k chars) where it was empty before — something may have been posted. Let me inspect it.
Ran — Parse saved review-comments file

Status on PR #7:
Checks: Cycode Secrets ✓ · Cycode Licenses Compliance ✗ (ignored, per your call). The 13 Cycode bot comments are all the MPL-2.0 / LGPL license flags we already decided to ignore (uniffi*, attohttpc, jna, etc.).
New: CodeRabbit posted 9 code-review comments. Triaged:
Legit & small (worth fixing):
🟠 Reset self.peer = None in CreateGroup — otherwise a session that previously joined another group can merge the new group's doc into the stale peer (cross-group leak). Real bug.
🟠 Don't expect() on the poisoned mutex at the FFI boundary — return a JSON error instead of crashing Kotlin.
🟠 Bound invite size before base64/JSON decode (JoinGroup) — avoid large-alloc/OOM on hostile input.
🟠 Add timeouts around iroh connect/accept/stream waits — avoid indefinite hangs.
🟠 Close GroupClient on Activity teardown — native/session leak on config changes.
🟡 Build the init payload with JSONObject so Build.MODEL is escaped safely.
Noise / not actionable: comments on gradlew and the UniFFI RustBuffer layout (both generated files, not hand-written); and a "critical" note that accept_and_exchange isn't wired into the live session loop — that's the documented deferred live-accept path, not a bug.

These are all genuine improvements, but this is a throwaway experiment, so whether to polish it is your call:
Asked — CodeRabbit fixes (×3, one stray duplicate)

**User chose Apply all 6.** Let me implement them.
Edited lib.rs +19 -2
Now the invite size bound in protocol.rs:
Edited protocol.rs +11 -0
Now add timeouts to the iroh network ops in net.rs:
Edited net.rs +39 -28
Now the two Android fixes in MainActivity.kt:
Edited MainActivity.kt +10 -2
Now rebuild and re-run the tests to confirm the Rust changes are correct:
Ran — Rebuild and run tests after fixes
Type-inference + trait-bound errors. Let me see the full list:
Ran — List all compile errors
Root cause: Incoming is IntoFuture, not Future (the rest cascade from it). Wrap it in an async block:
Edited a file, ran a command
All tests green again, including the load-bearing iroh sync. The public FFI surface is unchanged, so the generated bindings stay valid. Committing and pushing:
Pushed claude/delta-chat-android-iroh-16o28h

Done. Applied all 6 CodeRabbit fixes (commit 506e632, pushed to PR #7):
Correctness: reset peer on CreateGroup (prevents cross-group merges)
Hardening: no-panic on poisoned mutex at FFI boundary; 15s timeouts on all iroh network waits + relay online(); bounded invite size before decode
Android: JSONObject for the init payload; GroupClient.close() in onDestroy
All host tests still pass — including the two-peer Automerge sync over real iroh. Posted one summary comment on the PR rather than spamming the 6 threads.
PR #7 state: Secrets ✓ · Licenses Compliance ✗ (ignored, per your call).

[stray duplicate "Apply all 6" question noted as already complete]

**status of experiment, what would it take to get to apk from here to test? needs documenting**

Let me verify one thing that materially affects whether the APK will actually load the native lib — the library name the generated bindings expect — so the doc is accurate, not guesswork.
Ran — Find native library name expected by bindings
Confirmed: the bindings load the native lib as group_core (JNA → libgroup_core.so), which is exactly what the cdylib produces — so no renaming is needed, and there's a startup checksum guard tying the .so to the bindings. That makes the doc accurate. Writing it now:
Created PATH_TO_APK.md +156 -0
Now add a pointer from the README so it's discoverable:
Edited a file, ran a command
Documented and pushed (commit 915c284).

Experiment status: Solid at Tier 1, verified. The load-bearing claim holds: two peers sync an Automerge doc over real iroh (cargo test green, all 5 tests). Rust core compiles, Kotlin bindings generated/committed, full Gradle project, CodeRabbit's 6 fixes applied. Only thing standing between here and an installable APK is toolchain availability (no NDK/SDK in this sandbox) — not code.

What it takes to get to a testable APK (now in PATH_TO_APK.md):
1. Install toolchain (~30–60 min, one-time): Android SDK Platform 35 + Build-Tools 35, NDK r27c (r27+ for Android 15's 16 KB pages), cargo install cargo-ndk. (Rust targets already installed.)
2. Tier 2 — cross-compile the .so: cargo ndk -t arm64-v8a -t x86_64 --platform 24 -o ../android/app/src/main/jniLibs build --release. Bindings load group_core → libgroup_core.so, no rename needed.
3. Tier 3 — assemble + install: set sdk.dir in local.properties, ./gradlew assembleDebug, adb install.
4. Test on two devices: A creates group → Show invite → paste into B → Join → post/read converges (the live relay-path version of the host test).
The doc also flags the real gotchas: the UniFFI startup checksum ties the .so to the bindings (regenerate together if the API changes); the relay path needs outbound internet to n0 infra; and the Toast-based invite is awkward to copy across emulators (suggested a one-line Log.d tweak for testing).
PR #7: 4 commits, Secrets ✓ / Licenses ✗ (ignored).
