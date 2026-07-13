# Path from here to a testable APK

This experiment currently builds and verifies to **Tier 1** in a sandbox with **no
Android SDK and no NDK**. This document is the concrete, ordered path to a **signed
debug APK you can install and exercise on two devices** — i.e. Tier 1 → Tier 2 → Tier 3.

> TL;DR effort: ~30–60 min, almost all of it one-time SDK/NDK install + first
> dependency downloads. The actual build is a couple of commands.

---

## 0. Where it stands now (Tier 1, proven)

| Piece | State |
|---|---|
| Rust core (`core/`) | Compiles on host; **all host tests pass**, incl. two-peer Automerge sync over **real iroh** (`cargo test`). |
| Android Rust targets | `aarch64-linux-android`, `x86_64-linux-android` **installed**. |
| Kotlin bindings | **Generated & committed** (`android/app/src/main/java/uniffi/group_core/group_core.kt`). |
| Android/Gradle project | Complete, with wrapper (`./gradlew`). |
| **Missing for an APK** | Android **SDK**, Android **NDK**, `cargo-ndk`, and therefore the cross-compiled `.so`. |

The native library the bindings load is **`group_core`** (JNA resolves this to
`libgroup_core.so`), which is exactly what the `core` crate's `cdylib` produces — so
**no renaming is needed**. UniFFI also embeds a startup **checksum** tying the `.so` to
the generated bindings; if the exported API changes you must regenerate the bindings
(see step 2c) or the app will throw at load time.

---

## 1. Install the toolchain (one time)

Pick versions known to work with this project (AGP 8.7, Kotlin 2.0, Gradle 8.9):

| Tool | Version | Notes |
|---|---|---|
| JDK | **17+** (21 is fine) | AGP 8.7 requires JDK 17+. |
| Android SDK Platform | **API 35** | matches `compileSdk = 35`. |
| Android Build-Tools | **35.x** | |
| Android **NDK** | **r27c (27.x)** recommended | r26+ works; **r27+ gives 16 KB page-size alignment** required by Android 15+ devices. |
| `cargo-ndk` | latest (`cargo install cargo-ndk`) | drives the cross-compile + linker. |
| Rust targets | `aarch64-linux-android`, `x86_64-linux-android` | already installed here. |

Easiest route: install **Android Studio**, then in *SDK Manager* tick **SDK Platform 35**,
**Build-Tools 35**, **NDK (Side by side) 27.x**, and **CMake**. Then export:

```sh
export ANDROID_HOME="$HOME/Android/Sdk"            # or wherever the SDK lives
export ANDROID_NDK_HOME="$ANDROID_HOME/ndk/27.x.y" # the installed NDK
```

---

## 2. Tier 2 — produce the native `.so`

### 2a. Confirm the Rust prerequisites (already true in this repo)

```sh
rustup target add aarch64-linux-android x86_64-linux-android
cargo install cargo-ndk
```

### 2b. Cross-compile into the app's `jniLibs`

```sh
cd experiments/android-p2p-app/core
cargo ndk \
  -t arm64-v8a \          # -> aarch64-linux-android  (real devices)
  -t x86_64 \             # -> x86_64-linux-android    (emulator)
  --platform 24 \         # min API, matches app minSdk = 24
  -o ../android/app/src/main/jniLibs \
  build --release
```

Produces:

```
android/app/src/main/jniLibs/arm64-v8a/libgroup_core.so
android/app/src/main/jniLibs/x86_64/libgroup_core.so
```

> **Size note:** build **`--release`**. A debug `cdylib` of this dependency graph is
> ~300 MB; release is far smaller and strips fine. (`abiFilters` in
> `app/build.gradle.kts` is already limited to these two ABIs.)

### 2c. (Only if you changed the exported Rust API) regenerate the bindings

```sh
cargo build   # host build, produces target/debug/libgroup_core.so
cargo run --bin uniffi-bindgen -- generate \
  --library target/debug/libgroup_core.so --language kotlin \
  --out-dir ../android/app/src/main/java
```

The committed bindings are current for the existing `GroupClient { new, new_local,
handle }` surface, so you can skip this unless you add/rename exported items.

---

## 3. Tier 3 — assemble and install the APK

```sh
cd experiments/android-p2p-app/android
echo "sdk.dir=$ANDROID_HOME" > local.properties
./gradlew assembleDebug
# -> app/build/outputs/apk/debug/app-debug.apk
adb install -r app/build/outputs/apk/debug/app-debug.apk
```

(First run downloads the AGP/Kotlin plugins and the JNA `aar` from Google/Maven —
needs network.) Or just open `android/` in Android Studio and press **Run**.

---

## 4. Exercise the app on two devices/emulators

The shipped UI uses the **production** `GroupClient()` (n0 relay + discovery), so peers
reach each other across NATs using only the relay url + public key from the invite.

1. Launch **two** emulators/devices and install the APK on both.
2. Each device auto-creates its own group on launch and sets an author name.
3. On **device A**: tap **Show invite** → an invite string `croftcgrp1:…` appears.
4. Copy it to **device B** (see ergonomics note below), paste into the invite field,
   tap **Join group**.
5. Post messages on either device; the other should converge (post → sync → read).

**Verifies live what the host test proves hermetically:** the iroh transport + Automerge
merge loop, now over the real relay path on real Android.

---

## 5. Gotchas / troubleshooting

* **`UnsatisfiedLinkError: libgroup_core.so not found`** — the `.so` wasn't placed in
  `jniLibs/<abi>/`, or the device ABI isn't in `abiFilters`. Re-run step 2b for the
  needed ABI (e.g. add `-t armeabi-v7a` → `armv7-linux-androideabi` for old 32-bit
  devices, and `rustup target add armv7-linux-androideabi`).
* **UniFFI checksum/`API checksum mismatch` at startup** — the `.so` and the generated
  Kotlin are from different API versions. Rebuild the `.so` and rerun step 2c together.
* **No sync between peers** — the relay path needs outbound internet to reach n0's relay
  + pkarr DNS. On a locked-down network it won't connect. The hermetic host test sidesteps
  this with direct loopback addrs; on-device same-LAN direct-dialing (no relay) would
  require putting a direct addr in the invite, which we deliberately do not do.
* **16 KB page-size crash on Android 15** — use NDK **r27+** (already recommended).
* **Invite is hard to copy from the Toast** — for easier multi-device testing, log it:
  add `Log.d("p2p", invite)` next to the Toast in `MainActivity` and read it with
  `adb logcat -s p2p`. (Minor test-ergonomics tweak, not shipped.)

---

## 6. What an APK does *not* prove (still deferred)

A working APK validates packaging + on-device load + the live relay sync. It does **not**
add: message encryption (noted in `net.rs` where it would wrap payloads), a background
service / push, persistence beyond in-memory, multi-group, gossip-based large-group
fan-out, or NAT-traversal testing on real cellular networks. Those remain out of scope
for this experiment.
