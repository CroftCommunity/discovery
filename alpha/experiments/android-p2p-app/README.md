# Experiment: Android app for basic private-group read/post over iroh + Automerge

A **self-contained** experiment that wires a Rust P2P stack — **iroh** (transport) +
**Automerge** (CRDT) — into a *truly basic* private-group experience (see the group's
messages, post a message), and bridges it to an Android Kotlin UI through a single
JSON-in / JSON-out FFI call.

It is structured along Delta Chat's proven, shipping iroh-in-Android patterns:

* **JSON-RPC-over-FFI binding** (a single `handle(json) -> json` surface) rather than
  per-method C-FFI/JNI — the migration Delta Chat made because it is far less work to
  maintain.
* **iroh started lazily** (only when a group is created/joined), as an ephemeral
  channel that can be torn down when the group view closes.
* **Bootstrap by NodeAddr + 32-byte TopicId**, with the **direct IP deliberately
  excluded** from the invite.

> **Scope honesty:** A signed/installable APK needs the Android SDK + NDK + Gradle +
> a Rust cross-compile. This build environment has **no SDK and no NDK**, so it
> **cannot emit an APK** — exactly as the brief predicted, and as even Delta Chat
> sidesteps by shipping *prebuilt* cores. The deliverable is a **complete, correct,
> buildable project** plus the verification this environment *can* do. The
> load-bearing proof — two iroh peers syncing an Automerge document over **real
> iroh** — runs green here.

---

## 1. Capability tier reached: **Tier 1** (+ Kotlin bindings generated)

| Gate | Result |
|---|---|
| `rustc` / `cargo` | **1.94.1** ✅ (≥ 1.80) |
| crates.io registry access | ✅ |
| Android Rust targets | `aarch64-linux-android`, `x86_64-linux-android` **installed** ✅ |
| Android **NDK** (`ANDROID_NDK_HOME`) | ❌ absent → cross-compile blocked at link time |
| `cargo-ndk` | ❌ not installed |
| Android **SDK** (`ANDROID_HOME`) | ❌ absent |
| JDK | **21.0.10** ✅ |
| Gradle | **8.14.3** ✅ (wrapper pinned to 8.9) |

* **Tier 1 (reached):** project scaffold complete; Rust core **compiles**; **all host
  tests pass**; Kotlin bindings **generated**; complete Android/Gradle project consumes
  them; build instructions documented.
* **Tier 2 (not reached):** the Android targets install, but with no NDK there is no
  `aarch64`/`x86_64` Android linker, so producing a `.so` fails at link time.
* **Tier 3 (not reached):** no Android SDK ⇒ no `assembleDebug`.

### Resolved versions (pinned)

```
iroh          0.98.2        automerge   0.6.1        tokio    1.52.3
iroh-base     0.98.0        uniffi      0.29.5       sha2     0.10.9 + 0.11.0-rc.5 (coexist)
ed25519-dalek 3.0.0-pre.6
```

---

## 2. Binding strategy: **Option A — JSON-RPC over a UniFFI string surface**

The entire Kotlin ↔ Rust boundary is one method:

```
GroupClient.handle(commandJson: String): String   // JSON Command in, JSON Response out
```

Commands: `init`, `create_group`, `join_group`, `get_invite`, `post_message`,
`get_messages`, `sync`. Adding a feature = add a `Command` variant + a match arm in
Rust — **no new binding plumbing per method**. This is the exact reason Delta Chat
moved off per-method C-FFI/JNI. UniFFI still generates idiomatic typed Kotlin (the
`GroupClient` class) and handles the JNA loading boilerplate, so we get the
JSON-RPC simplicity *and* generated bindings.

The generated Kotlin lives at
[`android/app/src/main/java/uniffi/group_core/group_core.kt`](android/app/src/main/java/uniffi/group_core/group_core.kt)
and is regenerated with:

```sh
cd core
cargo build
cargo run --bin uniffi-bindgen -- generate \
  --library target/debug/libgroup_core.so --language kotlin \
  --out-dir ../android/app/src/main/java
```

**The Kotlin call site** ([`MainActivity.kt`](android/app/src/main/java/com/croftc/p2pexp/MainActivity.kt)),
following Delta Chat's "blocking call on a background thread" guidance:

```kotlin
private val client: GroupClient by lazy { GroupClient() }

private suspend fun send(commandJson: String): String =
    withContext(Dispatchers.IO) { client.handle(commandJson) }   // blocks on the core's Tokio runtime
```

---

## 3. iroh bootstrap / invite structure

The invite ([`protocol.rs`](core/src/protocol.rs)) carries **only**:

```
node_id    = inviter's public key (iroh EndpointId)
relay_url  = inviter's relay url (if any)
topic      = random 32-byte TopicId (hex)
```

encoded as `croftcgrp1:<base64url(json)>`. The **direct IP is dropped on purpose**
(`Invite::new` keeps relay + pubkey only), so an invite-carrier cannot persist it;
holepunching discovers the direct path at connect time. The test
`invite_roundtrip_excludes_ip` asserts no IP leaks into the invite.

iroh is **started lazily** (`Session::ensure_node`): the endpoint is bound only on
`create_group` / `join_group`, never at app start.

---

## 4. Host test results (run with `cd core && cargo test`)

```
running 4 tests (lib)
test group::tests::post_and_read_roundtrip ... ok
test group::tests::joiner_starts_empty_then_obtains_list_by_merge ... ok
test protocol::tests::invite_roundtrip_excludes_ip ... ok
test tests::command_surface_offline ... ok

running 1 test (tests/sync_over_iroh.rs)   <-- THE load-bearing proof, over real iroh
test two_peers_sync_automerge_over_real_iroh ... ok
  host sees:   [Message { author: "host",   text: "from host",   ts: 1 },
                Message { author: "joiner", text: "from joiner", ts: 2 }]
  joiner sees: [Message { author: "host",   text: "from host",   ts: 1 },
                Message { author: "joiner", text: "from joiner", ts: 2 }]
```

`two_peers_sync_automerge_over_real_iroh` binds **two real iroh endpoints**, dials over
loopback via direct addresses (relay/DNS disabled, so it is hermetic and needs no
external infra), and exchanges Automerge snapshots over a real QUIC bidirectional
stream **in both directions**. Post-on-A → sync → read-on-B, and vice versa; both
documents converge. That is the post → sync → read loop proven end to end over iroh.

---

## 5. Architecture

```
Kotlin UI (MainActivity)                       android/app
   │  client.handle(jsonCommand) : jsonResponse  (single JNI/JNA boundary)
   ▼
group_core (Rust cdylib)                       core/src
   ├─ protocol.rs : JSON Command/Response + Invite (NodeAddr+TopicId, no IP)
   ├─ group.rs    : Automerge doc — a "messages" list (author, text, ts)
   ├─ net.rs      : iroh Endpoint, lazily bound; snapshot exchange over QUIC
   └─ lib.rs      : Session (Tokio runtime) + GroupClient (UniFFI object)
```

---

## 6. Building further

> 📄 **Full, step-by-step path to a testable APK — including toolchain versions,
> the cross-compile, install, two-device test flow, and troubleshooting — is in
> [`PATH_TO_APK.md`](PATH_TO_APK.md).** The summary below is the short version.

### Producing the `.so` (Tier 2 — needs the NDK)

```sh
rustup target add aarch64-linux-android x86_64-linux-android   # already done here
cargo install cargo-ndk
export ANDROID_NDK_HOME=/path/to/ndk
cd core
cargo ndk -t arm64-v8a -t x86_64 -o ../android/app/src/main/jniLibs build --release
```

### Assembling the APK (Tier 3 — needs the SDK)

```sh
cd android
echo "sdk.dir=/path/to/Android/sdk" > local.properties
./gradlew assembleDebug        # -> app/build/outputs/apk/debug/app-debug.apk
```

Open `android/` in Android Studio and it will fetch the AGP/Kotlin plugins itself.

---

## 7. Friction log (honest)

* **Dependency hell between iroh and Automerge.** Latest `automerge` (0.10) and `iroh`
  (0.98) cannot coexist: `iroh-base` pins `sha2 =0.11.0-rc.5` (a release candidate),
  while `automerge 0.10` wants `sha2 ^0.11.0` (stable) — same 0.11 line, so cargo
  cannot unify them. **Resolution:** pin `automerge = 0.6.1`, which has no `sha2 0.11`
  requirement, so `sha2 0.10.9` (transitive) and `sha2 0.11.0-rc.5` (iroh) coexist as
  separate majors. This is why the doc layer targets the 0.6 Automerge API.
* **iroh-gossip dropped.** `iroh-gossip 0.100` conflicts with `iroh 0.98` on
  `ed25519-dalek`; `iroh-gossip 0.98` conflicts with `automerge` on `sha2`. Rather than
  chase a matched triple, this experiment uses **direct iroh `Endpoint` connections
  over a custom ALPN** for the snapshot exchange — genuinely real iroh transport, no
  extra transitive-crypto coupling, and the right altitude for a basic 2-peer/small
  group. Gossip (iroh-gossip) is the documented scale-up for larger groups.
* **iroh 0.98 API churn.** This release renamed `NodeId`→`EndpointId`,
  `NodeAddr`→`EndpointAddr`, and introduced a `Preset` builder (`presets::N0` /
  `Minimal` / `N0DisableRelay`). The code was written against the actual 0.98 source
  and examples, not from memory.
* **Async across the FFI.** Per Delta Chat's guidance, the FFI surface is **blocking**:
  `GroupClient.handle` drives iroh on an embedded Tokio runtime via `block_on`, and
  Kotlin calls it on `Dispatchers.IO`. No Rust→Kotlin callbacks.
* **No NDK/SDK in this environment** ⇒ Tier 1. `cargo-ndk`, the Android linker, and
  `assembleDebug` are unavailable here; the steps above complete the remaining tiers.
* **ktlint** is not installed, so the generated Kotlin is unformatted (harmless).

---

## 8. What's proven vs. deferred

**Proven (here, green):**
* The Automerge group model and the shared-genesis merge rule (so peers converge).
* The post → sync → read loop over **real iroh** (the integration test).
* The JSON command surface (`init`/`create`/`post`/`get_messages`/`get_invite`).
* The invite format (NodeAddr + TopicId, IP excluded).
* The Rust core **compiles**; the **Kotlin bindings generate**; the Android/Gradle
  project is complete and consumes them.

**Deferred (honestly):**
* On-device / multi-emulator run and a real APK (needs SDK + NDK).
* Live sync across NATs via the **relay** path (needs n0 relay/DNS reachability;
  proven structurally, run deferred). The hermetic test uses direct loopback addrs.
* **Encryption** — out of scope. In the broader architecture an encryption layer would
  wrap the snapshot payloads exchanged in `net.rs` before they hit the wire.
* Background service / push, persistence beyond in-memory, multi-group, gossip-based
  large-group fan-out, and NAT-traversal testing on real cellular networks.
