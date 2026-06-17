# Real-time media over iroh — WebRTC media engine on our transport (the end-state)

date: 2026-06-16
status: thinking (design). Problem / Approach / Reasoning, with challenges → proposed solutions →
test cases. The end-state for voice/video/stage; deliberately deferred behind the messaging spine but
specified here so the deferral is "integrate an engine," not "invent an architecture."

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
   no retransmit and no head-of-line blocking. [UNVERIFIED: iroh's datagram API surface + whether/how
   QUIC CC applies to datagrams in iroh's stack — confirm first.]
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
candidate; its production maturity for audio+video is the gating unknown. libwebrtc is mature but owns
its own ICE/DTLS transport and fights substitution.

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
TC-INT3 decides A1 vs A2 empirically.

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
  TC-ENG1/2  audio over iroh datagrams (2-peer)  ── MILESTONE 1: "is it physically viable?"
     │            (also the C1 substrate)
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

- iroh datagram API + its CC behavior is **[UNVERIFIED]** — TC-ENG0/TC-CC1 resolve it; everything
  downstream assumes datagrams behave as unreliable/uncontrolled-enough.
- str0m video maturity **[UNVERIFIED]** — audio-first hedges it.
- DAVE's exact SFrame/MLS internals **[UNVERIFIED]** — we implement to the pattern, not their code.
- Browser reach is handled by **Mode B** above (str0m-as-bridge at the meer + SFrame via Insertable
  Streams); the open risk is str0m's real-browser interop fidelity and the latency of the two-regime
  hop — TC-INT1/2 resolve it.
- E0-NAT hole-punch (gated on public ingress) is a prerequisite for the mesh/direct-media path; until
  it's open, all media is meer-relayed (the louder-metadata case).
