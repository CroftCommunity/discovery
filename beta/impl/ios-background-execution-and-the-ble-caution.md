# iOS background execution, and the BLE caution that forces the design

`Status: impl layer (mobile-background execution model). Register: platform mechanics + a load-bearing
negative result. Resolution: library — the concrete iOS wake-hook taxonomy mapped to iroh wake-pulses,
plus the BLE-scavenger negative result that forces the meer-anchored delivery design. iOS platform facts
cite the FACTCHECK source of truth and are not re-verified here; iroh version facts likewise. Complements
the delivery layer, which owns the push-notify node and APNs parity; this document adds the full wake-hook
taxonomy and the negative result that sits underneath it.`

## Overview

On a server, peer-to-peer transport is *deterministic*: a process holds a QUIC socket open indefinitely,
routes are stable, and a peer is reachable because it is always listening. iOS forbids that shape. The OS
does not let an app hold a live network connection in the background; it suspends the process and reclaims
the socket. So on iOS the transport stack cannot be a resident listener. It must be *opportunistic*: it
lives like a *scavenger*, treating a small set of sanctioned OS lifecycle events as *wake-pulses* — brief,
budgeted windows in which it can boot, grab or pass data, and tear down before the window closes.

This document does two things. First, it names the concrete wake-hooks iOS actually provides and maps each
to the iroh work it can carry, so the mobile transport design rests on the real platform surface rather
than on an idealized one. Second, it carries the load-bearing negative result that shapes the whole
delivery design: spontaneous, off-grid, device-to-device meshing over Bluetooth Low Energy is
*aspirational*, not a foundation you can build on. That negative result is *why* the delivery layer anchors
on an always-on rendezvous (the meer / superpeer) and treats device-to-device sync as a bonus. The
resolution is only trustworthy if the reason travels with it, so this document keeps the reason.

## Charter: what this document covers

- **In scope:** the iOS background-execution model (the wake-hook taxonomy), the Rust-core / Swift-shell
  bridge that consumes those hooks, and the BLE negative result together with the design consequence it
  forces.
- **Out of scope (and where it lives):** the push-notify node's rights properties (blind, revocable,
  byte-free), the APNs / FCM payload and throttle facts, and the parity-with-a-normal-app stance — these
  are the delivery layer's, in `delivery-layer/01-delivery-architecture.md` (§2.4 mobile wake, §4 the
  presence plane, §5 the push-notify role) and summarized in `delivery-layer/00-session-summary.md`. This
  document does not re-specify them; it supplies the platform mechanics that sit beneath them.
- **Boundary call:** the delivery layer answers "who wakes a sleeping phone, and with what rights" (the
  push-notify node, at parity with a normal messaging app). This document answers "what wake-hooks does
  iOS actually expose, what can each carry, and why can we not lean on off-grid BLE" — the platform
  substrate under that presence design, plus the negative result that makes an always-on anchor
  non-optional. Push is one actuator among the hooks below; it is specified there, referenced here.

## 1. The opportunistic model, and why iOS forces it

The controlling constraint: an iOS app cannot hold a background socket open. The OS suspends the process
shortly after it leaves the foreground and does not guarantee it any continuous execution. Everything
below follows from that single fact. Rather than fight for a connection the OS will sever, the transport
core treats each sanctioned system event as an explicit network *wake-pulse*, and structures all
background work as short stateless bursts.

The runtime pattern that falls out of this:

```
[ iOS system event (wake-pulse) ]
            │
            ▼  (OS relaunches / resumes the app container)
[ Swift shell: AppDelegate / task handler ]
            │
            ▼  (calls into the Rust core over UniFFI)
[ Rust / iroh core: ephemeral Endpoint ]
            │
            ▼  one stateless write/read across the swarm
[ signal task completion to iOS ]  ──▶  (process suspended again)
```

The shell catches the OS event; the core binds an ephemeral iroh `Endpoint`, does one stateless exchange
(push a small signed frame, or pull a bounded delta), and signals completion well before the budget
expires. This is the shape production apps already use to bridge a heavy decentralized core into iOS's
strict runtime, Delta Chat (Rust core, Swift shell, push-into-background-fetch) being the clearest
reference. `[FACTCHECK source of truth — Delta Chat architecture verified, not re-verified here.]`

## 2. The wake-hook taxonomy: what each hook buys an iroh mesh

iOS exposes a small, fixed set of background wake-hooks. Each maps to a distinct kind of iroh work. The
four load-bearing ones, with the platform facts that bound them:

| Wake-hook | Window / trigger | iroh *wake-pulse* it carries |
|---|---|---|
| **Significant Location Change (SLC)** | Fires on cell-tower change / substantial movement | Spatial peer re-discovery: on entering a new venue, scan the local link and re-establish nearby routing, drop a state-sync frame, sleep |
| **BGAppRefresh** | ~30 s predictive window, scheduled by on-device learning ahead of likely app opens | Proactive delta-sync: pull recent updates so history is fresh before the user opens the app |
| **BGProcessing** | Minutes of execution, but only while charging, on unmetered Wi-Fi, and idle | Heavy work: large blob pulls, cryptographic chain verification, local DB compaction, and acting as a temporary relay for the swarm |
| **Silent push + Notification Service Extension (NSE)** | ~30 s to process a payload; out-of-band trigger | The wake actuator for a stationary device that no other hook will rouse — the hand-off point to the delivery layer's push-notify node |

Platform facts bounding the table, all citing the FACTCHECK source of truth and not re-verified here:

- **SLC** wakes the app on tower change or substantial movement. `[FACTCHECK: CONFIRMED.]`
- **BGAppRefresh** gives roughly a 30-second predictive window, scheduled by on-device learning.
  `[FACTCHECK: CONFIRMED.]`
- **BGProcessing** grants minutes, gated on charging + unmetered Wi-Fi + idle. `[FACTCHECK: CONFIRMED.]`
- **Silent push and the NSE** are distinct but both offer roughly a 30-second processing window; silent
  pushes are rate-limited by the OS. The NSE memory cap is **24 MB** (iOS 14+) — not the larger figure the
  originating dialogue asserted. `[FACTCHECK: PARTLY — silent-push vs NSE were conflated in the source;
  both ~30 s; silent pushes rate-limited; NSE cap is 24 MB, source of truth, not re-verified.]`
- Region monitoring (geofence) is a near relative of SLC: a short (~10 s) wake on a boundary crossing,
  with a dwell threshold near 20 s and a cap of 20 regions. `[FACTCHECK: PARTLY — the ~10 s is right, the
  dwell was understated in the source.]`

Two supporting platform facts the background core depends on:

- The correct file-protection class is **`NSFileProtectionCompleteUntilFirstUserAuthentication`**, which
  lets the background core read and write its own database while the phone is locked, once it has been
  unlocked at least once since boot. `[FACTCHECK: CONFIRMED.]`
- The 24 MB NSE cap is a hard ceiling on how heavy the background entry path can be. It forces the core to
  stream to disk and pull chunks lazily on the silent-push path, rather than materializing large state in
  memory. `[FACTCHECK: 24 MB cap, source of truth, not re-verified.]`

## 3. The Rust-core / Swift-shell bridge

The pattern above needs a language boundary: a portable Rust/iroh core that owns all cryptographic and
protocol logic, and a thin Swift shell that owns the iOS lifecycle and calls into the core when a wake-hook
fires. The two are bridged over **UniFFI** (Mozilla's cross-language Rust binding generator), giving the
Swift shell a type-safe surface onto the core. `[FACTCHECK: UniFFI is Mozilla's cross-language Rust
binding tooling — CONFIRMED, source of truth.]`

This is now first-party ground rather than a hand-rolled FFI: iroh ships **first-party Swift bindings**
(`iroh-ffi`), released with **iroh core 1.0 in mid-2026**. `[FACTCHECK: CONFIRMED — first-party Swift
bindings shipped with iroh 1.0, mid-June 2026; iroh version facts cite the FACTCHECK source of truth and
are not re-verified here.]` So the bridge the design relies on is real and supported, not aspirational.

The division of labor is strict: the shell holds no protocol state and makes no trust decisions; it only
translates OS lifecycle events into calls, and reports completion back to iOS. All sealing, verification,
and convergence live in the core, so the same core serves every platform and the iOS-specific surface stays
small.

## 4. The BLE-scavenger caution: the load-bearing negative result

There is a romantic story that off-grid meshing invites: *two strangers' locked phones pass on the street,
auto-wake over Bluetooth Low Energy, and silently sync.* It is worth stating plainly that this story is
**shakier than it sounds, and cannot be a foundation.** This is the *BLE-scavenger caution*, and it is a
negative result, not a caveat.

Two facts establish it, both citing the FACTCHECK source of truth:

- **CoreBluetooth State Restoration does not wake the app on discovering a brand-new advertiser.** It
  relaunches the app only for *established or pending* connections and subscriptions. The specific "an
  encrypted service UUID nearby wakes a terminated app" claim — the mechanism the whole passing-strangers
  story rests on — is refuted as described. `[FACTCHECK: REFUTED as described — restoration covers
  established/pending connections and subscriptions only, NOT new-advertiser discovery; source of truth,
  not re-verified.]`
- **The OS kills a backgrounded P2P node within seconds.** Berty — a zero-server peer-to-peer messenger
  that explicitly built proximity transports over BLE and Multipeer Connectivity, and is the clearest
  reference for attempting exactly this — reports in its own engineering writing that the OS tears the
  background P2P node down within seconds of backgrounding. Reliable wake-and-sync while locked is,
  by Berty's own account, an unsolved constraint rather than a shipped feature. `[FACTCHECK: verified —
  Berty's own blog; source of truth, not re-verified.]`

A second, independent leg reinforces the caution from the transport side. Even setting the iOS wake problem
aside, BLE-over-iroh is not part of core iroh: it exists only as a **community transport crate**
(`iroh-ble-transport` / `blew`), and the one public demonstration of it is unencrypted. So the device-to-device
BLE substrate is itself unofficial and immature — the caution does not rest on the platform quirk alone. The
usable path leans on `unstable-custom-transports` (QUIC over any sufficiently large datagram link) as the
mechanism, but the specific BLE binding that a passing-strangers mesh would need is not first-party ground.
`[FACTCHECK: PARTLY — BLE is a community crate, not core iroh; source of truth, not re-verified.]`

What survives is narrow but real: the *opportunistic-wake* model of §2 is usable — SLC, BGAppRefresh,
BGProcessing, silent push, and **established-connection** BLE restoration all work as budgeted wake-pulses.
What does not survive is *spontaneous* off-grid meshing: an app cannot count on being woken by a peer it
has never met, and cannot hold the connection long enough to be a dependable mesh member while backgrounded.
Device-to-device BLE sync is therefore a bonus that sometimes fires, never a backbone.

## 5. Why the caution forces the meer-anchored design

The negative result is not a dead end; it is the reason the delivery design has the shape it does. If two
backgrounded phones cannot be relied on to find and wake each other, then the dependable rendezvous must be
a node that is *not* subject to the mobile watchdog — an always-on anchor. That anchor is the meer /
superpeer: a blind store-and-forward node that holds sealed bytes for an offline recipient and is reachable
whenever the recipient's phone next gets a wake-pulse, together with the push-notify node that fires the
content-free wake in the first place.

This is the direct line from the negative result to the delivery layer's structure, and it is why that
layer has a push-notify node at all:

- Because a sleeping phone cannot be reached by a peer, **something off the mobile-watchdog path must poke
  it.** That is the push-notify node (delivery layer §4–§5): it learns only "this endpoint has mail, wake
  it," fires a content-free wake, and holds no sealed bytes. The silent-push + NSE hook of §2 is the iOS
  actuator it drives.
- Because a sleeping phone cannot hold a message for a peer either, **durability must live on an always-on
  node**, not on the participants' phones alone. That is the meer (delivery layer's durability plane).
- Because the wake channel is throttled and unreliable by nature, **the design treats it as an optimization
  over polling**, never a dependency — the phone always catches up on next foreground by polling the meer.

So the platform mechanics here and the delivery layer's roles are two ends of one design: the wake-hook
taxonomy is the set of moments an iOS device can act, and the meer-plus-push arrangement is what makes those
brief moments sufficient. The BLE caution is what rules out the alternative — a pure device-to-device mesh
with no anchor — and thereby forces the anchor to exist.

## What this establishes (and does not)

Establishes the concrete iOS background-execution model: that the platform forbids a resident background
socket, so the transport core must be opportunistic, waking on a fixed taxonomy of hooks (SLC,
BGAppRefresh, BGProcessing, silent push + NSE, with geofence as a near relative) and doing short stateless
bursts across an ephemeral iroh `Endpoint` via a Rust-core / Swift-shell bridge over UniFFI, on first-party
iroh Swift bindings shipped with iroh 1.0. It establishes the BLE-scavenger negative result — that
CoreBluetooth restoration does not wake on a new advertiser, that a backgrounded P2P node is killed
within seconds, and that BLE-over-iroh is a community crate rather than core, with its one public demo
unencrypted — and shows that this negative result is *why* the delivery design anchors on an always-on
meer with a content-free push actuator rather than a pure device-to-device mesh.

Does **not** re-specify the push-notify node, the APNs / FCM payload and throttle facts, or the
parity-with-a-normal-app stance — those belong to the delivery layer (`delivery-layer/01-delivery-architecture.md`
§2.4, §4, §5; `delivery-layer/00-session-summary.md`) and are referenced, not repeated. Does **not**
re-verify the iOS platform facts or the iroh version facts, which cite the FACTCHECK source of truth. Does
**not** decide what off-grid or background behavior Croft ultimately promises on iOS — that product line is
an open thread — nor claim that established-connection BLE restoration extends to any spontaneous-meshing
use; the caution bounds exactly how far the usable wake model reaches.
