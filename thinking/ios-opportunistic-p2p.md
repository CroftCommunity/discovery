# iOS opportunistic P2P over Iroh: event-driven keepalive, swarm relay, off-grid transports

date: 2026-06-22

status: exploratory thinking, distilled from a fact-checked Gemini dialogue
(`seeds/transcripts/raw/atproto-atmospheric-web-iroh-mobile-dialogue.md` +
`...-FACTCHECK.md`). Only CONFIRMED/PARTLY claims carried here, flagged inline. Relates to
`thinking/multi-device.md`, `thinking/meer-superpeer-design.md`,
`thinking/realtime-media-over-iroh.md`, and the [[croft-relay-lab-2026-06-16]] findings.

---

## The core insight: opportunistic, not deterministic

On a server, P2P is deterministic — persistent socket, static routes. On iOS it must be
**opportunistic**: you cannot hold a QUIC socket open in the background, so the network stack
lives like a scavenger, treating OS lifecycle events as wake-pulses to grab/pass data inside a
short window, then tear down. The design pattern: **Swift shell catches the OS event → calls a
Rust/Iroh core via FFI (UniFFI) → ephemeral `Endpoint`, do one stateless write/read across the
swarm → signal completion before the window closes.** This is exactly how Delta Chat (Rust core
`deltachat-core-rust`, Swift iOS, push→background-fetch) and Berty (Go/libp2p core, RN UI)
operate [CONFIRMED].

## The real iOS wake-hooks (and what each buys an Iroh mesh)

- **Significant Location Change (SLC)** — wakes on tower change / substantial movement → spatial
  peer re-discovery [CONFIRMED].
- **Region monitoring (geofence)** — short (~10 s) wake on boundary crossing; caveat: ~20 s dwell
  threshold, max 20 regions [PARTLY — the ~10 s is right, dwell understated].
- **BGAppRefreshTask** — ~30 s predictive refresh window via on-device learning → proactive
  delta-sync before the user opens the app [CONFIRMED].
- **BGProcessingTask** — minutes of execution when charging + Wi-Fi + idle → heavy work (blob
  pulls, chain verification, DB compaction, acting as a temporary relay) [CONFIRMED].
- **Silent push + Notification Service Extension** — ~30 s to process a payload (the OOB trigger
  for a keepalive when the device is stationary) [PARTLY — silent-push vs NSE conflated; both
  ~30 s; silent pushes are rate-limited; NSE memory cap is **24 MB**, not 30–50 MB].
- **CoreBluetooth State Restoration** — relaunch on BLE events, **but only for established/pending
  connections, NOT on discovering a brand-new advertiser** [REFUTED as described]. This is the
  load-bearing correction (see below).

`NSFileProtectionCompleteUntilFirstUserAuthentication` is the right file-protection class so the
background core can read/write its DB while the phone is locked-after-first-unlock [CONFIRMED].

## Iroh as the core (verified primitives)

- **Ephemeral keepalive to an HA peer**: bind an `Endpoint`, dial by **EndpointId** (Ed25519
  public key), push a tiny signed frame (CIDs/state), tear down. Iroh's **relays** (ex-"DERP")
  give NAT fallback; **QUIC-multipath** lets a session survive Wi-Fi↔cellular [CONFIRMED].
  ⚠️ The dialogue's Rust uses `connect_to_peer` — **wrong**; the real 1.0 API is
  `endpoint.connect(addr, &[u8] alpn)` (see `IROH-1.0.0-API-VERIFIED.md`).
- **Swarm relay-through-the-chain**: ping one awake peer; **iroh-gossip** (HyParView + Plumtree)
  ripples the frame through the group, routing around sleeping nodes [CONFIRMED — matches our
  `altdrive-spike-gossip`]. Trackerless discovery via a shared Topic ID is plausible.
- **Off-grid transports**: `unstable-custom-transports` (iroh 0.97+, QUIC over any ≥1,200-byte
  datagram link) is real [CONFIRMED]. **BLE** exists as a **community** crate
  (`mcginty/iroh-ble-transport` + `blew`; `BlewChat` demo is **unencrypted**), GATT→L2CAP upgrade
  with QUIC riding the link. Tor (`n0-computer/iroh-tor`) and Nym (`iroh-nym`) give metadata
  privacy [PARTLY — BLE is community not core; `iroh-webrtc-transport` likely doesn't exist].
- **First-party Swift bindings** (`iroh-ffi`) shipped with iroh 1.0 mid-June 2026 [CONFIRMED] —
  the FFI bridge is real, not aspirational.
- **Clock drift**: don't trust mobile wall-clocks for ordering; use logical clocks /
  iroh-docs causal sync. ⚠️ iroh-docs uses **range-based set reconciliation + LWW**, NOT Merkle
  Search Trees (the dialogue conflated AT Proto's MST) [REFUTED as described].
- **Music over Iroh**: stream from an HA peer via **iroh-blobs** (BLAKE3 chunks) piped into a
  decoder; QUIC avoids HoL blocking, session migrates on roam [CONFIRMED — Aster is a real
  local-first P2P-music prototype]. Consistent with `thinking/realtime-media-over-iroh.md`.

## The load-bearing caution

The romantic "**two strangers' locked phones pass on the street, auto-wake over BLE, and silently
sync**" story is **shakier than the dialogue implies**. CoreBluetooth restoration does *not*
relaunch on new-advertiser discovery, and **Berty's own engineering blog says the OS kills the
P2P node within seconds of backgrounding** [verified]. The *opportunistic-wake* model is real and
usable (SLC, BGAppRefresh, BGProcessing, established-connection BLE restoration, silent push), but
**spontaneous off-grid scavenger meshing should be treated as aspirational/unproven**, not a
given. For Croft this argues for the **meer/superpeer** as the dependable always-on rendezvous
(the relay lab's co-location thesis), with opportunistic device-to-device sync as a bonus, not the
backbone.

## Open edges

- Background BLE meshing reliability on iOS — needs a real spike before any design leans on it.
- The HA-peer-vs-meer choice maps onto `thinking/meer-superpeer-design.md`; "ping one awake peer →
  gossip" is the same shape as the relay-lab rendezvous, which is **proven** (E3: 12/12, 30/30
  converged) where ad-hoc BLE meshing is not.
- Reconnect-storm handshake CPU and the cold-boot memory cap (24 MB NSE) bound how heavy the
  background core can be — keep background entry paths lean (stream to disk, lazy chunks).
