# Real-time media over iroh — WebRTC media engine on our transport (the end-state)

date: 2026-06-16
status: thinking (design). Problem / Approach / Reasoning, with challenges → proposed solutions →
test cases. The end-state for voice/video/stage; deliberately deferred behind the messaging spine but
specified here so the deferral is "integrate an engine," not "invent an architecture."

**Feasibility at a glance (2026-06-16):** the *floor* (P2P audio over iroh, no WebRTC) is **proven
externally** by n0's callme (iroh-roq datagrams + Opus + cpal). The *ceiling* (group video + browser +
stage, blind, at scale) is a clear path of "proven shape + known integration" with **two genuine
technical unknowns** (datagram congestion-control interaction; str0m video maturity) and **two gated
items** (NAT hole-punch ingress for direct media; the meer binary E8/E9). Metadata cost is a known,
accepted property, not a risk. See "Feasibility vs unknown" at the bottom.

## Problem

Voice/video/stage is the one Discord capability our messaging spine does not touch, and the naive
read ("a separate real-time subsystem, defer it") hides that **three of its four layers reuse things
we have already proven**. We need the end-state written down so the deferred work is scoped correctly
and so its genuinely-hard parts have testable proposals rather than hand-waves. The seed is in
`seeds/transcripts/raw/p2p-architecture-origin-dialogue.md` (WebRTC DataChannels + the signaling
caveat + "browsers can't do raw QUIC"), the CSR idea in `SOVEREIGN-COMMONS-DOSSIER.md` (eliminate
WebRTC's stateful signaling server), and `research/discord-matrix-groupchat.md` ("iroh could carry
P2P media later"). This doc is their synthesis.

## Approach — keep WebRTC's media engine, throw away WebRTC's transport

iroh already *is* the transport WebRTC bolts on awkwardly. The fold cleaves the stack where each side
is strongest:

```
  ┌─────────────────────────────────────────────┐
  │ MEDIA ENGINE — Opus/AV1, jitter/NetEq, echo   │  reuse (the one piece nobody should rebuild)
  │ cancel, FEC, bandwidth estimation             │
  ├─────────────────────────────────────────────┤
  │ RTP/RTCP packetization                        │
  ├─────────────────────────────────────────────┤
  │ SFrame, keyed by the MLS group epoch          │  E2EE THROUGH the blind SFU-meer (the DAVE shape)
  ├─────────────────────────────────────────────┤
  │ iroh QUIC datagrams                           │  PROVEN: NAT traversal, relay fallback, Ed25519
  │ (replaces ICE + STUN + TURN + DTLS + signaling)│  identity, hop encryption
  └─────────────────────────────────────────────┘
```

- **Media engine → reuse**, via a *sans-IO* engine (str0m) so we own the transport rather than fork libwebrtc.
- **Transport → iroh**, proven (MD-G1 NAT path, E0–E7 relay/placement). The `EndpointAddr` + gossip
  topic is the rendezvous — the CSR "signaling without a signaling server" idea realized.
- **Keying → MLS**, proven (MD-G4 fold, MD-G5 revocation, faithful-path standing). SFrame keyed off the
  MLS epoch; rekey on membership change. Discord's DAVE validates the whole right column.
- **Forwarding → the blind meer**, characterized (E3.4 blind broker, E5 hosting, E0–E7 opaque forward).

### Topologies, and where each fits us vs Discord

| topology | who forwards | scales to | us | Discord |
|---|---|---|---|---|
| **full mesh** | everyone → everyone | ~2–5 | interactive tier — direct iroh when co-present | **none** (always server-routed) — our differentiator |
| **SFU** | one node forwards each stream (no decode) | dozens–hundreds | the **blind SFU-meer** — E0–E7 opaque-forward shape | **their architecture** (DAVE = blind SFU since 2024) |
| **MCU** | server decodes + mixes (plaintext) | many | **drop** — breaks the blind broker | not used by Discord either |

Capability set ≈ Discord's SFU **+** a mesh mode Discord lacks **−** MCU. The decisive difference is
*who runs the SFU*: Discord's is centralized and could un-blind; ours is a co-op/peer-run blind meer,
with a pure-P2P mesh fallback that needs no infrastructure at all.

### Prior art: n0's `callme` already proves the floor — audio over iroh, no WebRTC

Verified 2026-06-16: n0's own **`callme`** is a peer-to-peer audio-call app that uses **`iroh-roq`
(RTP-over-QUIC)** to carry **Opus**-encoded audio, with **`cpal`** for cross-platform audio I/O and
**optional echo cancellation** — and **no WebRTC at all**. It's experimental (v0.1.x, 2025) with CLI +
egui GUI on desktop and Android. This is, in effect, a working instance of our **Milestone 1**
(2-peer audio over iroh): the "is audio over iroh even viable?" probe is *already answered* by a
shipping n0 demo, and `iroh-roq` is the RTP-carriage seam. **CONFIRMED from `iroh-roq` source
(2026-06-16):** it supports **both** RoQ modes — `send_flow.rs::send_rtp()` calls
`conn.send_datagram(...)` (the **unreliable QUIC-DATAGRAM** media path, with a VarInt flow-ID prepended
for muxing), and `new_send_stream()` via `open_uni()` for the reliable path. callme carries Opus over
the **datagram flow** — exactly the media-appropriate, no-retransmit mode, and proof iroh exposes QUIC
datagrams. This resolves the *transport-primitive* half of C1 (the congestion-control interaction is
still to test).

### This gives us *three* engine lines, not one

`callme` reframes C2 (the str0m question) — it's no longer "str0m or nothing":

| line | what it is | strength | cost |
|---|---|---|---|
| **L1 — RoQ-direct (callme-style)** | `iroh-roq` + Opus + cpal, hand-rolled jitter/adaptation | **proven viable** by callme; simplest stack; pure P2P | you build jitter/FEC/congestion/echo yourself (or borrow callme's); audio-first; no browser |
| **L2 — str0m media engine** | sans-IO WebRTC media over iroh | mature jitter/FEC/congestion + video + browser interop (Mode B) | integration complexity; str0m's weak spot is P2P ICE (which we bypass) |
| **L3 — webrtc-rs `rtc` (sans-IO)** | the new (2026) sans-IO core + v0.20 wrapper | broader codec lineage; runtime-agnostic; second mature-lineage option | alpha today (production = old v0.17.x callback API) |

The pragmatic path: **start from L1 (callme) for the audio MVP and mesh/small calls** — it's the
shortest route to a working voice channel and it's already de-risked — and reach for **L2/L3 only when
we need what RoQ-direct lacks**: a mature adaptive media engine at scale, video, and browser interop
via the SFU-meer. `iroh-roq` is the common RTP seam, so L1→L2/L3 is an evolution, not a rewrite.

### Media interaction *types* — by use case, like group types (RoQ vs MoQ)

The lines above are all the *conversational* path. But media has the same "type at creation, not a
mode toggle" shape as the messaging **interaction tiers** (`interaction-tiers.md`), and it lands on
**two different QUIC media protocols** chosen by use case — confirmed by n0's own apps (see
`research/iroh-realtime-media-references.md`):

| media type | protocol | shape | topology | reference | maps to tier |
|---|---|---|---|---|---|
| **Conversational** (call) | **RoQ** (RTP-over-QUIC) | symmetric, lowest latency | mesh (≤~5) → blind SFU-meer | **`callme`** | interactive |
| **Broadcast** (stage/watch-party/livestream) | **MoQ** (Media-over-QUIC) | **pub/sub Tracks**, lazy, internet-scale, sub-250 ms | one-to-many fan-out via a **MoQ relay** | **`iroh-live`** (moq-rs + GStreamer) | broadcast |
| *(quiet-large video — mostly idle small group)* | either, **lazy** | encode only when watched | mesh/SFU | — | quiet-large |

The decisive properties:
- **MoQ is pub/sub and lazy.** A broadcaster publishes named **Tracks** (video / audio / metadata as
  separate tracks); viewers **subscribe**; and **nothing is encoded or sent until a subscriber asks**.
  That lazy property is the *media instance of the interaction-tiers philosophy* — "nothing to fan out
  if nobody is watching" — and it's the battery/compute/privacy win (no idle broadcast).
- **The meer gains a broadcast role.** A **MoQ relay** forwards Tracks it needn't decode → **blind**,
  the media analog of the message blind broker, and it costs nothing until a subscriber appears. So
  the meer has *three* blind roles: message broker, conversational SFU (RoQ), and broadcast relay
  (MoQ). Detailed in `thinking/meer-superpeer-design.md`.
- **Type is chosen at creation, by need**, exactly like interactive-vs-broadcast rooms: a "call" is a
  RoQ object, a "stage/channel-with-an-audience" is a MoQ object. Live in-place conversion is a
  create-new-and-redirect, not a mutation (same rule as the messaging tiers).
- **Browser reach splits by type** (verified — `research/iroh-realtime-media-references.md`): broadcast
  (MoQ) reaches browsers via **WebTransport over HTTP/3** (`h3+iroh://`) through a **moq-relay** — no
  str0m needed, likely the *easier* browser path; conversational (RoQ) browser reach is the
  str0m-WebRTC + SFrame-Insertable-Streams bridge (Mode B). `moq` itself has first-class iroh support
  (`iroh://` / `moqt+iroh://` URL schemes, P2P-by-default + relay bridging), so the broadcast stack is
  mostly assembly, not build.

So "voice/video/stage" is not one feature — it's **conversational (RoQ) + broadcast (MoQ)**, two media
types under the same iroh transport + MLS keying + blind-meer forwarding, each matched to a need.

---

## Challenges → proposed solutions → test cases

The four hard parts, each with a concrete proposal and test cases that **reuse harnesses we already
built** wherever possible (the E6 `tc netem` rig, the openmls/faithful-path machinery, the
E3.4/AR-4 metadata method).

### C1 — Congestion control across iroh datagrams

**Challenge.** A media engine runs its own bandwidth estimator (GCC/TWCC-style) that needs loss + RTT
+ arrival-jitter feedback and assumes it controls the wire. iroh QUIC has *its own* congestion
control. Stacked, they fight: QUIC pacing/buffering can hide loss from the media estimator (it
overshoots), or reliable-stream retransmit injects latency the engine never expects. Media is
loss-tolerant and latency-intolerant — the opposite of QUIC's default reliable-stream behavior.

**Proposed solution.**
1. Carry media over **QUIC datagrams** (unreliable, unordered) — never reliable streams — so there is
   no retransmit and no head-of-line blocking. **CONFIRMED available:** `iroh-roq` already does exactly
   this (`send_flow.rs::send_rtp` → `conn.send_datagram`), and callme ships on it. The primitive is
   proven; what remains to test is **how/whether QUIC congestion control + pacing applies to iroh
   datagrams** and whether it fights the media estimator (TC-CC1/2 below).
2. Make the **media engine's estimator authoritative for media bitrate**, and feed it feedback derived
   from the datagram path: loss = gaps in a per-stream sequence number we add; RTT = iroh's own
   path-RTT estimate exposed via the API; jitter = arrival timestamps. Treat iroh's datagram pipe as a
   "dumb" measured channel, not a second controller.
3. Keep media datagrams and bulk/reliable transfers on **separate flows** so the media flow is never
   starved or HOL-blocked by a file transfer on the same connection.

**Test cases (extend the E6 tc-netem rig directly):**
- **TC-CC1** — 2-peer Opus audio over iroh datagrams across the boxes; sweep `tc netem` loss/delay/
  bandwidth-cap (the E6 conditions). Pass: the engine's estimator converges to the netem-imposed
  bandwidth and audio stays intelligible (proxy metric: packet-loss-concealment events + jitter-buffer
  depth stay bounded). Run a raw-UDP baseline alongside to isolate iroh-CC interference.
- **TC-CC2** — bandwidth-ramp: start unconstrained, drop the cap mid-call (live netem change); measure
  time-to-adapt and overshoot. This is the "do the two controllers fight?" detector — if QUIC hides the
  loss, adaptation lags.
- **TC-CC3** — contention: media datagrams + a bulk reliable iroh transfer on the same connection;
  verify the media flow is not starved (datagram/stream isolation holds).

### C2 — Media-engine maturity (str0m or alternative)

**Challenge.** "Fold over iroh" requires a sans-IO engine (we own the socket). str0m is the Rust
candidate; its production maturity is the gating unknown. libwebrtc is mature but owns its own ICE/DTLS
transport and fights substitution.

*Verified (see `research/str0m-production-readiness.md`, 2026-06-16):* str0m's production track record
is **strongest as a server-side SFU** (Lookback's actual use) and **thinnest in its P2P ICE agent**
(the maintainers say so). This is favorable for us: Mode B (the SFU-meer) is exactly the tested path,
and Mode A bypasses str0m's ICE entirely (iroh is the transport) — so the maturity worry is narrower
than "is str0m ready" in the abstract. webrtc-rs is moving sans-IO (v0.20.0) and is the hedge. [CONFIRM
the rust-libp2p→str0m migration's current state; it adopted str0m from webrtc-rs ~2023.]

**Proposed solution.**
1. Define a thin **`MediaEngine` seam** (feed RTP in / get RTP out / report bandwidth estimate / set
   target bitrate / expose loss+RTT hooks) so the engine is swappable and the rest of the stack does
   not hard-wire str0m.
2. **Audio-first.** Prove the whole pipeline on Opus audio before committing to video; video (VP8 →
   AV1) is a later milestone gated on engine capability, not a day-one requirement.
3. Fallback ladder if str0m is insufficient: (a) a minimal hand-rolled Opus + jitter-buffer path for
   audio-only MVP; (b) wrap libwebrtc behind the seam with a custom transport shim (harder, last resort).

**Test cases:**
- **TC-ENG0** — *API audit (paper, gates the rest):* does str0m expose the bandwidth-estimation/TWCC
  feedback hooks C1 needs, and packet-in/packet-out without owning the socket? If no, the seam's
  fallback ladder triggers before any integration spend.
- **TC-ENG1** — str0m audio loopback on one host: mic → str0m → packets *we* carry → str0m → speaker;
  measure added latency and confirm we own the packets.
- **TC-ENG2** — TC-ENG1 between two boxes carried over iroh datagrams (this is also TC-CC1's substrate)
  — the actual integration proof and the **first real milestone**.
- **TC-ENG3** — video feasibility (VP8 then AV1): does str0m packetize/depacketize + simulcast? Pass/
  fail on capability, not quality.

### C3 — SFrame-over-MLS keying (per-sender, the SFU-meer never holds a key)

**Challenge.** Media frames need E2EE that survives the SFU-meer hop. SFrame encrypts the media
*payload* (leaving the RTP header + an SFrame header readable, so the SFU can still do layer selection)
with a key derived from the MLS group; each sender uses its own key. On membership change the group
rekeys. Media is loss-tolerant, so the keying **must not require contiguity** (unlike the message
hash-chain, which does). DAVE proves the pattern [UNVERIFIED on DAVE's exact SFrame/MLS internals].

**Proposed solution.**
1. Derive a **per-sender SFrame base key** from the MLS group's **exporter secret** + the sender's leaf
   index; the SFrame header carries `(key-id, frame-counter)`. Reuse the proven MLS epoch machinery —
   the MD-G4/G5 fold/revocation events are exactly the rekey triggers.
2. **Header-only SFU.** The SFU-meer forwards/selects on RTP + SFrame headers and never the payload —
   design every SFU decision (simulcast layer pick, active-speaker routing) to need headers only.
3. **Loss-tolerant counters.** Per-sender monotonic frame counter + a sliding replay window; decrypt
   surviving frames out of order, reject replays, never require a contiguous chain.

**Test cases (reuse the openmls + faithful-path machinery):**
- **TC-KEY1** — derive per-sender SFrame keys from a real MLS group; verify two senders get distinct
  keys and a **non-member cannot derive any key** (ties to the proven standing check — the media analog
  of the faithful-path `UnauthorizedAuthor` result).
- **TC-KEY2** — encrypt/decrypt a frame stream with 10% loss + intra-window reorder; verify receivers
  decrypt surviving frames and reject replays (loss-tolerant, not contiguity-requiring).
- **TC-KEY3** — membership change mid-stream → epoch advance → SFrame rekey; verify a **revoked sender's
  subsequent frames can't be decrypted** by remaining members (media MD-G5) while frames received
  pre-revocation stay decryptable (history-not-clawed-back analog).
- **TC-KEY4** — blind-SFU check: feed the SFU SFrame ciphertext; verify it can do layer selection from
  headers alone and **cannot recover any plaintext** (the blind property; extends E3.4 + AR-4 to media).

### C4 — Metadata cost (continuous media is the loudest AR-4 signal)

**Challenge.** A forwarding meer sees the call's `EndpointId` set (= membership), per-stream packet
timing, bitrate, and duration — near-perfect "who's talking to whom, when, how much" traffic analysis,
*even fully content-blind*. Continuous media is far louder than bursty messaging. Mesh avoids the meer
seeing it but exposes peer IPs to each other.

**Proposed solution (a dial, each setting measurable):**
1. **Prefer mesh for small calls** (direct, hole-punched) so no forwarding node observes the pattern —
   accepting peer-IP exposure. Ties to E0-NAT hole-punch (currently gated on public ingress).
2. For SFU-meer calls, accept that the meer learns call membership + timing, and make that an
   **explicit, consented property of choosing a meer** (vs mesh). Optional **constant-bitrate / padding**
   to defeat bitrate/active-speaker inference, at a measured bandwidth cost. Onion/multi-hop is
   latency-prohibitive for real-time — out of scope.
3. **Compartmentalize:** run the SFU-meer separate from the messaging-meer so the media traffic-analysis
   surface is not correlated with message metadata.

**Test cases (extend the AR-4 / E3.4 method to media):**
- **TC-META1** — instrument an SFU-meer during a call; enumerate exactly what it observes (membership
  set, per-stream timing, bitrate, duration). Output: the **AR-4-for-media bound** (the media parallel
  of the messaging metadata-leak characterization). Measurement, not pass/fail.
- **TC-META2** — constant-bitrate/padding on: measure the bandwidth cost and whether speaking-vs-silent
  is still distinguishable from the padded stream (does CBR actually hide the active speaker?).
- **TC-META3** — mesh-vs-SFU leak comparison: confirm a hole-punched mesh call gives the meer **zero**
  observability, quantify the peer-IP-exposure trade, and state the dial honestly.

---

## str0m & WebRTC interoperability — two modes from one library

The reason to choose str0m specifically is that its two properties — **sans-IO** (does no network I/O;
you drive it with received bytes + a clock and it returns packets-to-send) and a **complete WebRTC
protocol stack** (ICE, DTLS, RTP/RTCP, SRTP, data channels) — unlock two complementary interop modes
that no single alternative gives cleanly. libwebrtc has the protocol stack but owns its own sockets/
ICE (fights mode A); a bare codec/jitter library gives neither; webrtc-rs is more libwebrtc-shaped
(owns more I/O); Pion is sans-IO-ish but Go. str0m's sans-IO + full-stack combo is the enabler.
[UNVERIFIED: str0m's production maturity, and exactly how much of its media path is usable without
driving a full ICE/DTLS session — TC-INT/TC-ENG0 resolve this.]

### Mode A — native ↔ native: iroh is the wire, str0m is the media engine (the "fold")

Use str0m for the **media pipeline** (RTP/RTCP, jitter, packetization, and SRTP-or-SFrame) and carry
its packets over **iroh datagrams**. iroh already provides what str0m's transport layer would
(identity via Ed25519, NAT traversal, relay fallback, hop encryption), so its ICE/DTLS are redundant.
This is the clean native-to-native path — no browser, no signaling server.

There is a **real sub-decision here** worth a test, because str0m is organized around a full WebRTC
session (it expects SDP/ICE/DTLS to establish SRTP keys):
- **A1 — media-layer-only:** drive only str0m's RTP/media machinery and supply keys ourselves
  (SFrame from MLS), bypassing its ICE/DTLS entirely. Cleanest, but may sit below str0m's intended
  public API.
- **A2 — full session tunneled:** run str0m's complete WebRTC session *inside* a single iroh datagram
  flow (iroh as one "candidate"/pipe). Simpler to wire, but pays redundant DTLS encryption + carries
  ICE/DTLS machinery we don't need.
TC-INT3 decides A1 vs A2 empirically — but the production-readiness verification
(`research/str0m-production-readiness.md`) already **biases toward A1**, because A2 would drag in
str0m's least-tested area (its P2P ICE agent), the exact thing A1 + iroh route around.

### Mode B — browser ↔ overlay: str0m as a real WebRTC endpoint at the meer (the bridge)

Browsers cannot do iroh QUIC (the origin dialogue's hard constraint), but they *do* speak standard
WebRTC. So a **meer runs str0m as a genuine WebRTC peer**: the browser connects to it with ordinary
WebRTC (SDP offer/answer, ICE, DTLS-SRTP), and on its other side that meer speaks **iroh to native
peers**. str0m is the **bilingual bridge** — WebRTC on the browser-facing leg, iroh on the native-
facing leg. The SFU-meer naturally becomes this gateway because it is already the always-on forwarding
point: browser legs arrive over WebRTC, native legs over iroh, and it forwards opaque frames between
them. This is the *only* way browsers join the overlay at all, and str0m's full protocol stack is what
makes the meer a valid WebRTC endpoint without a separate libwebrtc dependency.

### SFrame keeps the bridge blind

In Mode B the browser↔meer leg is DTLS-SRTP, which **terminates at the meer** — so DTLS alone would let
the meer read the media. To keep the meer blind across the bridge, the browser applies **SFrame
end-to-end via the WebRTC Encoded Transform / Insertable Streams API**: JS encrypts each encoded frame
with the MLS-derived SFrame key *before* handing it to the browser's WebRTC stack, and decrypts after
receive. The meer then terminates DTLS (hop) but still cannot read the media (SFrame e2e) — exactly the
two-layer posture native peers get (iroh hop + SFrame e2e). This is the same construction Discord's
DAVE uses for browser E2EE [UNVERIFIED on DAVE's exact browser path, but the Encoded-Transform + SFrame
approach is the standard one].

### Interop test cases

- **TC-INT1** — a real browser establishes a WebRTC call to a str0m endpoint (the meer); audio flows
  both ways. Proves str0m is a valid WebRTC peer (Mode B baseline).
- **TC-INT2** — SFrame-through-the-bridge: the browser applies SFrame via Insertable Streams keyed from
  a (modelled) MLS group; verify the meer terminates DTLS yet **cannot recover plaintext** (blind across
  the bridge — the browser analog of TC-KEY4), and a native iroh peer on the far side decrypts it.
- **TC-INT3** — Mode A sub-decision: stand up str0m media between two native peers over iroh datagrams
  with **A1 (media-only, no ICE/DTLS)** vs **A2 (full session tunneled)**; compare wiring complexity,
  added latency, and redundant-encryption cost. Picks the native path.

---

## Phased validation sequence

```
  TC-ENG0 (str0m API audit, paper)              ── gates everything
     │
     ▼
  TC-ENG1/2  audio over iroh (2-peer)            ── MILESTONE 1: LARGELY ANSWERED by n0's callme
     │            (also the C1 substrate)             (iroh-roq + Opus + cpal). Our M1 = reproduce/
     │                                                extend callme and measure it under the netem rig.
     ▼
  TC-CC1/2/3  congestion vs the E6 netem rig     ── MILESTONE 2: "does it survive real networks?"
     │
     ▼
  TC-INT3  Mode A native fold (A1 vs A2)         ── picks the native transport wiring
     │
     ▼
  TC-KEY1–4  SFrame-over-MLS, blind SFU          ── MILESTONE 3: "is it E2EE through a blind meer?"
     │
     ▼
  TC-META1–3  the media metadata bound           ── MILESTONE 4: "what does the meer learn?"
     │
     ▼
  TC-INT1/2  browser ↔ str0m-meer + SFrame bridge ── MILESTONE 5 (browser reach): "can browsers join, blindly?"
     │
     ▼
  video (VP8 → AV1), stage = broadcast-tier fan-out  ── deferred until 1–4 are green
```

Milestone 1 (2-peer audio over iroh datagrams across the NAT path) is the cheap, isolated probe — the
media analog of how MD-G1 proved gossip-over-NAT before anything was built on it. It can be stood up
while the boxes are available and tells us whether the whole direction is viable before any engine or
keying investment.

## Reasoning

- **Why borrow the engine, build everything else:** the media engine (codecs/jitter/FEC/congestion) is
  the only part that is both genuinely hard *and* not differentiating. Transport, keying, and
  forwarding are where our proven work lives and where the non-extractive guarantee is made — so we
  own those and rent the engine.
- **Why sans-IO is load-bearing:** "fold over iroh" is only feasible if the engine doesn't own the
  socket. That single property (str0m's, not libwebrtc's) is what turns this from a fork into an
  integration. TC-ENG0 gates on exactly it.
- **Why the keying reuses MLS unchanged conceptually:** a media frame's right to be decrypted is the
  same membership/standing question as a message's right to be applied. SFrame-over-MLS makes media
  revocation = message revocation (MD-G5), and a non-member's media as undecryptable as a non-member's
  message is unauthorized (faithful path). The only twist is loss-tolerance (no contiguity requirement).
- **Why metadata is the honest ceiling:** no transport elegance removes the fact that a forwarding node
  in a continuous-media call sees the conversation graph. The dial (mesh ↔ blind-SFU ↔ padded) is a
  consented trade, and naming it is the non-extractive move — the same posture as `failed-op-response.md`
  (the leak is disclosed and chosen, not hidden).

## Open edges

- iroh datagram **primitive is CONFIRMED** (iroh-roq `send_datagram`, callme ships on it); what's still
  open is its **CC behavior** — TC-CC1/2 resolve whether QUIC pacing/CC fights the media estimator.
- str0m video maturity **[UNVERIFIED]** — audio-first hedges it.
- DAVE's exact SFrame/MLS internals **[UNVERIFIED]** — we implement to the pattern, not their code.
- Browser reach is handled by **Mode B** above (str0m-as-bridge at the meer + SFrame via Insertable
  Streams); the open risk is str0m's real-browser interop fidelity and the latency of the two-regime
  hop — TC-INT1/2 resolve it.
- E0-NAT hole-punch (gated on public ingress) is a prerequisite for the mesh/direct-media path; until
  it's open, all media is meer-relayed (the louder-metadata case).

## Next experiments (priority order — what each de-risks, what it reuses)

The smallest sequence that converts unknowns into evidence. Each reuses an existing asset where noted.
**Baked into the testing round** as **E10–E12** in `experiments/iroh/RELAY-PLACEMENT-LAB-SPEC.md §4a**
(E10 = step 1 RoQ-under-netem; E11 = MoQ broadcast lazy fan-out; E12 = blind media-meer SFrame/MLS);
the meer build itself is `thinking/meer-superpeer-design.md` (phases P0–P6).

1. **Reproduce + measure callme under netem (audio MVP).** Build/run callme (or a minimal RoQ-direct
   clone) between two of the AWS boxes and the NAT Mac; drive it through the **E6 `tc netem` rig**
   (delay/loss/bandwidth-cap). *De-risks:* C1 congestion control (the one live technical unknown) and
   confirms audio quality holds across our real NAT fabric. *Reuses:* callme, the E6 harness, the box
   fabric. **Highest priority — it attacks the only genuine technical unknown with the most-proven
   component.**
2. **TC-ENG0 — engine API audit (paper).** Confirm str0m (and the new webrtc-rs `rtc`) expose the
   packet-in/out + bandwidth-estimation hooks we need without owning the socket. *De-risks:* C2 engine
   choice; gates any engine integration spend. *Reuses:* nothing — desk research.
3. **Blind SFU-meer forwarding (3+ peers).** Stand up a meer that forwards opaque RTP datagrams among
   3 peers (no decode); measure the active-passthrough throughput/CPU wall (E0's media-rate ceiling).
   *De-risks:* the meer as a media forwarder + its real cost. *Reuses:* the relay-lab E0/E5 cgroup +
   metrics machinery.
4. **TC-KEY1–4 — SFrame-over-MLS.** Per-sender keys from a real MLS group; loss-tolerant decrypt;
   media revocation = MD-G5; blind-SFU can't read payload. *De-risks:* C3 keying. *Reuses:* the
   openmls / faithful-path machinery already in `Proofs/lineage-groups`.
5. **TC-META1–3 — the media metadata bound.** Instrument the SFU-meer; produce the AR-4-for-media
   characterization; measure padding/mesh mitigations. *De-risks:* nothing technical (it's the honest
   ceiling) — produces the threat-model bound. *Reuses:* the AR-4 / E3.4 method.
6. **TC-INT1–3 — browser bridge + Mode-A wiring.** Real browser ↔ str0m-meer; SFrame-through-bridge
   stays blind; A1-vs-A2 native decision (biased to A1). *De-risks:* browser reach. *Gated soft on*
   step 2.

Cross-cutting prerequisite for *direct* (mesh) media rather than meer-relayed: **open E0-NAT
hole-punch ingress** (3343/3478) — currently gated. Until then steps 1/3 run meer-relayed.

## Feasibility vs unknown — where each piece sits (2026-06-16)

```
  PROVEN ──────────── FEASIBLE (known work) ──────── OPEN (real unknown) ──── GATED (blocked)
  │                   │                              │                        │
  P2P audio/iroh      blind SFU-meer forwarding      datagram congestion       NAT hole-punch for
  (callme: iroh-roq   (= E0–E7 opaque-forward +      control vs media          DIRECT media
   datagrams+Opus     str0m server-SFU @ Lookback)   estimator (TC-CC1/2)      (E0-NAT, ingress
   +cpal)                                                                       closed)
  │                   SFrame-over-MLS keying         str0m VIDEO maturity      the meer BINARY
  QUIC datagrams      (DAVE pattern + our proven      (audio-first hedges)      (E8/E9 unbuilt)
  exposed by iroh      MLS fold/revocation/standing)
  │                   browser bridge Mode B
  iroh NAT/relay       (str0m WebRTC + SFrame via
  (MD-G1/E0–E7)        Insertable Streams)
                      audio media engine
                       (callme floor + str0m/rtc)
```

- **Metadata cost is NOT on this scale** — it's a known, unavoidable property (a forwarding meer sees
  the call graph). Feasible to build; the only "measurement" is how much mesh/padding mitigates it.
- **Net:** the floor is proven, the ceiling is mostly "known work," and the project's technical risk
  concentrates in exactly **two cells** (datagram CC; video engine) plus **two gated** items
  (hole-punch ingress; the meer binary). Step 1 above attacks the first unknown directly.
