# iroh app-pond building blocks: games, file drop, media, on-device AI

`Status: cairn layer (Layer 3, the open field). Register: survey / homage. Resolution: library — the
external building blocks the games and media pads are assembled from, each carried with the reason it
matters (what it proves, what Croft reuses, ports, or rejects). The product design that consumes these
blocks lives in `croft/product-the-garden-of-ponds.md` and is out of scope here. External facts carry
verification flags inline; iroh version facts cite the FACTCHECK source of truth and are not re-verified
here. The realtime-media ceiling, once carried as suspect, is now corroborated against shipping n0 work
(callme, iroh-live, moq); residual flags are the volatile on-device-AI facts and the wrappable catalog's
license notes, which want a final glance at bundle time.`

## Overview

The product treats an activity or game as a *pad* — a small, sandboxed, permission-scoped unit that runs
in the shell peer-to-peer, with iroh QUIC as the transport rather than browser WebRTC. Games are the named
cold-start hook: a warm, free, no-account thing two people can do together is the reason a first pair ever
opens the app. This document registers the external prior art the pads are built among — the shipping
games and frameworks that already run over iroh, the file-drop references, the realtime-media proof that
grounds the calls pond, the on-device-AI options behind the optional assistant, and the one anti-pattern
the whole "free perpetual ping" stance rebukes. It carries, for each block, the load-bearing *why*: what
it proves, and whether Croft would reuse it, port it, wrap it, or set it aside.

The load-bearing conclusion is that the games and media pads are a recombination of parts that already
exist and already run over iroh — direct-QUIC twitch play, rollback netcode, virtual-LAN tunnelling, and
a proven Opus audio floor are all demonstrated in the wild — so the pad layer is an assembly problem, not
an invention.

## Charter: what this document covers

- **In scope:** the external building blocks for the games/media pads, each with the reason it matters and
  the reuse / port / wrap / reject call.
- **Out of scope (and where it lives):** the product design that consumes these blocks — the garden thesis,
  the three inclusion pathways (build-fresh / wrap / port), the deep-link resolver, the fair-reveal
  primitive, the pad security bar, and the on-device assistant's invariants — all live in
  `croft/product-the-garden-of-ponds.md` and are not reproduced here. iroh's own transport mechanics live
  in the impl transport notes.
- **Boundary call:** this is the "what real things do the pads build among, and what does each prove"
  register. The *how* of assembling them into a product is the product doc's job; here we carry only the
  external references and the reason each is credited.

## Games — the named cold-start hook

The closest single prior art is a framework that already ships the exact stack a Croft game pad would want.

- **libmarathon / Marathon** (sunbeam-stdio) — an offline-first multiplayer framework on **Bevy + iroh +
  iroh-gossip + CRDTs**, with a 3D cube-sim demo on macOS/iOS. It is the closest existing "iroh-gossip +
  CRDT game" prior art: the exact substrate combination the pad layer is betting on, already assembled and
  running. Relationship: learn↔, build-on. `[verified: web 2026-06-22 — crates.io/crates/libmarathon]`
- **ascii-royale** (Chad Fowler) — a 16-player terminal ASCII battle-royale in Rust over iroh; the host
  prints a ticket and there are no servers. Why it matters: it proves **direct-QUIC twitch play** with a
  ticket-based join and zero infrastructure — the shape of a Croft game pad's session bootstrap. Relationship:
  homage, learn↔. `[verified: github chad/ascii-royale]`
- **iroh-lan** (rustonbsd) — a Hamachi-like encrypted virtual-LAN over iroh, turning legacy LAN-only games
  (Minecraft Java, StarCraft, CS 1.6) and **emulator netplay** (Snes9x / RetroArch) into internet play with
  no port-forwarding. Why it matters: it proves the **"tunnel localhost over iroh"** pattern, which is a
  whole category of legacy games reachable without touching their code. Relationship: learn↔, build-on.
  `[verified: github rustonbsd/iroh-lan]`
- **godot-iroh** — a Godot Asset-Library extension that swaps Godot's default multiplayer socket for an
  iroh endpoint (connect by Node ID, no port-forwarding). Why it matters: it proves an established game
  engine's networking layer drops onto iroh cleanly, so an engine-built pad is not a rewrite. Relationship:
  build-on. `[verified: web 2026-06-22]`
- **GGRS + matchbox** (gschup / community) — rollback netcode in Rust plus WebRTC signaling. Why it matters:
  it is the **architecture-aligned rollback stack** for the Rust core (more so than the JS alternatives),
  the load-bearing dependency for twitch-tier games. Relationship: build-on. `[license wants a final glance
  at bundle time — the netcode dependency ships in the bundle.]`
- **netplayjs** (rameshvarun) — rollback netcode plus WebRTC, where host-authoritative state doubles as a
  hidden-info dealer. Why it matters: a JS reference for rollback whose signaling could be swapped for iroh;
  carried as a substrate-port candidate rather than a direct dependency. Relationship: port substrate.
  `[confirm license at bundle time.]`
- **Curvytron** (Curve Fever / Achtung die Kurve) — an **MIT** fork base for the standout 2–8-player twitch
  game. Why it matters: a clean-licensed base for the marquee twitch pad. Relationship: port / wrap.
  `[MIT — verify at bundle.]`
- **boardgame.io** (nicolodavis) — an **MIT** turn-based engine with a transport abstraction, in JS/React.
  Why it matters: strong turn-based patterns to learn from, though its JS/React shape is in architecture-fit
  tension with the Rust core, so it is a pattern source, not a dependency. Relationship: learn↔ (turn-based
  patterns). `[MIT.]`
- **webxdc game catalog** (adbenitez / ArcaneCircle) — a catalog of small games (chess, the "wonster" word
  puzzle, and many more) wrappable through a webxdc-compatible shim rather than one port per game. Why it
  matters: it turns an entire catalog into candidate pads for the cost of one compatibility layer.
  Relationship: wrap (via webxdc-compat shim), homage, learn↔.
  `[LICENSE TRAP — MIXED LICENSES: several titles are GPL-3.0, and the chess piece art is CC-BY-SA-3.0
  (flagged trap); the "wonster" puzzle is MIT. Licenses and assets are inherited per-title on wrap and the
  surfaced set must be hand-picked; final license glance required at bundle time.]`

## File transfer — the device-to-device drop pad

- **sendme** (n0) — the iroh-blobs file-transfer reference: the near-free device-to-device drop pad, built
  directly on the blob primitive. Why it matters: the reference implementation for the drop pad, from the
  transport authors themselves. Relationship: build-on. `[iroh-blobs is a companion crate; version facts
  cite the FACTCHECK source of truth.]`
- **DataBeam** (vinay-winai / schollz / n0) — a desktop GUI uniting **croc** (schollz; code-phrase P2P file
  transfer) convenience with **sendme** (iroh) speed and resumability. Why it matters: a
  convenience-meets-iroh exemplar showing what a friendly drop-pad UX over iroh looks like. Relationship:
  learn↔. `[verified: croc + sendme both real; project live.]`

## Security model — CSP alone does not contain a webview

- **Cure53 webxdc security audit** (Cure53 / OpenTechFund) — the audit that established the blunt lesson
  every wrapped or ported pad inherits: a Content-Security-Policy alone does **not** contain a webview.
  WebRTC connections and DNS-prefetch were both found to be able to exfiltrate data from a webview that CSP
  believed it had isolated (the FILL500 finding). Why it matters — and the one genuinely-owned action item:
  **disable webview WebRTC** in pads. Croft's transport is iroh QUIC, not browser WebRTC, so a pad never
  needs the webview's WebRTC at all; disabling it closes the exfiltration hole at zero functional cost. The
  DNS-prefetch and speculative-connection paths in Chromium-family webviews need an equivalent check at the
  embedding layer. Relationship: learn↔ (security model). The product-layer commitments that follow from
  this audit (context-boundary isolation, per-match pseudonyms) are the product doc's pad security bar, not
  reproduced here.

## On-device AI — the optional assistant, detect-first / accelerate-never-gate

The assistant is an accelerant onto the deep-link resolver, never a gate; its product invariants live in
the product doc. The building blocks here are the on-device models it could ride, and the reason each is
a detect-first option rather than an assumed presence.

- **Apple Foundation Models framework** (Apple) — an on-device ~3B model with **guided generation**
  (`@Generable`) that constrains output to a real catalog, which is the anti-hallucination property that
  matters: the model is trusted only to rank intent onto links that already resolve, never to emit a link
  that does not. Why it matters and why it is bounded: strong on capable Apple devices (macOS wants
  M-series) and needs a **Swift↔Rust bridge** to reach the core. Relationship: build-on (optional
  assistant; macOS/Apple target). `[vendor-documented as of mid-2026; specific model sizes and device lists
  move monthly and are re-checked at build time.]`
- **Google Gemini Nano** (Google; AICore + ML Kit GenAI) — an on-device model with strong privacy isolation
  but a steep device cliff (roughly flagship-only) and weaker structured output than the Apple path's guided
  generation. Why it matters: the Android-side option, carried fallback-heavy because of the device cliff.
  Relationship: build-on (optional assistant; Android target). `[vendor-documented; device coverage
  volatile, re-checked at build time.]`

Frame for both: **detect availability first, accelerate but never gate.** When the model is absent the
assistant simply is not offered — no dead ends, no degraded-but-broken state — and every path it offers is
reachable without it.

## Realtime media — the calls pond

This section grounds the media claim the beta spec makes. The spec asserts that realtime media works over
iroh; the in-the-wild references that ground that assertion are below. The floor (conversational audio) is
corroborated by a shipping n0 project (`callme`); the ceiling — broadcast media and browser reach — is
**also corroborated by shipping n0 work**, correcting an earlier pass that carried it as suspect.

- **callme / iroh-roq** (n0) — the proven audio floor: peer-to-peer audio over iroh with **no WebRTC**,
  using **iroh-roq** datagrams plus **Opus** encoding (and `cpal` for capture/playback). Why it matters:
  this is the corroborated external proof that a real Opus audio call runs over iroh transport directly,
  which is what a calls pad's floor needs. Relationship: build-on (the audio floor).
  `[corroborated: n0's callme uses iroh-roq + Opus; the audio floor is the reliable part of this section.]`

The ceiling — group/broadcast media and browser reach — is **more corroborated than an earlier pass
recorded**, and the correction is load-bearing: the broadcast path is a *second proven QUIC media protocol*,
not a hypothesis. Media over iroh is two protocols for two use cases (the same "type at creation" shape as
the interaction tiers): **RoQ** (RTP-over-QUIC) for symmetric, lowest-latency calls — the callme floor above
— and **MoQ** (Media-over-QUIC) for one-to-many broadcast. The RoQ/MoQ transport mechanics, the meer's
blind broadcast role, and the str0m fold's test plan live in the impl transport notes; here they are the
building blocks the calls/stage pads assemble among.

- **iroh-live / MoQ** (n0; moq-rs) — the group/one-to-many broadcast ceiling. n0's `iroh-live` carries
  h264 + Opus over iroh via **moq-rs**, with GStreamer/ffmpeg ingest and a room-ticket connect; the upstream
  **moq** (moq-dev) has first-class iroh support (`iroh://` URL schemes, P2P-by-default with **optional relay
  bridging to browsers via WebTransport over HTTP/3**). MoQ is **pub/sub and lazy** — a broadcaster publishes
  named Tracks, viewers subscribe, and nothing is encoded or sent until a subscriber asks — which is the
  battery/compute/privacy win and the media instance of "nothing to fan out if nobody is watching."
  Corroborating adoption signal: the **Rave** watch-party app ships iroh + MoQ for video, chosen after
  evaluating libp2p and WebRTC first. Relationship: build-on (the broadcast ceiling). `[corroborated: web
  2026-06 — n0-computer/iroh-live, moq-dev/moq, iroh.computer/solutions. The specific per-relay connection
  ceilings Rave's marketing cites are UNVERIFIED — trust measured numbers, not the order-of-magnitude.]`
- **Browser reach splits by media type** — broadcast (MoQ) reaches browsers via **WebTransport over HTTP/3**
  through a moq-relay, needing no WebRTC engine and likely the *easier* browser path; conversational (RoQ)
  browser reach is the harder WebRTC-bridge path below. `[corroborated: web 2026-06.]`
- **WebRTC-over-iroh (the str0m fold)** — the conversational path's route to browser reach and to a mature
  adaptive engine at scale: keep WebRTC's media engine (a **sans-IO** engine such as **str0m** — the
  codecs/jitter/FEC/echo-cancellation nobody should rebuild) and carry its packets over iroh datagrams, with
  relay fallback when direct paths fail. str0m's production record is **strongest as a server-side SFU** and
  thinnest in exactly the P2P-ICE agent iroh bypasses, so the maturity worry is narrower than it looks. A
  characterized direction with two genuine unknowns — datagram congestion-control interaction and str0m's
  **video** maturity (audio-first hedges the latter). Relationship: build-on / port substrate.
  `[str0m server-SFU use corroborated; video maturity UNVERIFIED — audio-first.]`

One embellishment from an earlier pass is corrected on the record: group media is **not** a "gossiped
lateral chunk-sharing mesh." The confirmed model is **per-track QUIC streams, P2P-by-default over iroh with
relay bridging**; iroh-gossip is the membership/signalling bus, not a chunk-distribution mesh.

## The anti-pattern — Bond Touch

**Bond Touch** (and similar "thinking-of-you" bracelets) built an entire business, account system, and
cloud relay around what is functionally ~50 bytes: a partner-to-partner "I'm thinking of you" ping. Why it
is carried: it is the negative example the free, perpetual, no-account "thinking-of-you" ping directly
rebukes. The whole value was a tiny peer-to-peer signal, and wrapping it in accounts, servers, and a
subscription is exactly the extraction the pad layer refuses — a peer-to-peer ping over iroh needs none of
it. Relationship: learn↔ (negative example).

## What this establishes (and does not)

Establishes that the games and media pads are an assembly of parts that already exist and already run over
iroh: direct-QUIC twitch play (ascii-royale), the exact Bevy + iroh + gossip + CRDT stack (libmarathon),
virtual-LAN tunnelling for legacy games (iroh-lan), engine integration (godot-iroh), rollback netcode
architecture-aligned with the Rust core (GGRS + matchbox), a wrappable open game catalog (webxdc), a
device-drop reference (sendme / DataBeam), a corroborated Opus audio floor over iroh (callme / iroh-roq),
and a corroborated MoQ broadcast path over iroh (iroh-live / moq-rs, with Rave as the adoption signal). It
carries the security lesson that grounds every wrapped or ported pad (the Cure53 audit → disable webview
WebRTC), the two on-device-AI options behind the optional assistant framed detect-first, and the Bond Touch
anti-pattern the free perpetual ping rebukes.

Does **not** design the product that consumes these blocks — the garden thesis, the build-fresh / wrap /
port inclusion pathways, the deep-link resolver, the fair-reveal primitive, the pad security bar, and the
assistant invariants all live in `croft/product-the-garden-of-ponds.md`. Does **not** re-document iroh's
transport mechanics (impl transport notes) or re-verify iroh version facts (FACTCHECK source of truth).
Does **not** reproduce the RoQ/MoQ transport mechanics or the str0m fold's test plan (impl transport notes);
the residual media unknowns it does **not** resolve are str0m's **video** maturity, real-browser interop
fidelity, and Rave's specific per-relay ceiling numbers — the audio floor (callme / RoQ), the MoQ broadcast
path (iroh-live), and browser-via-WebTransport are corroborated, correcting an earlier pass that carried the
ceiling as suspect. Does **not** clear the license traps in the wrappable catalog (mixed GPL-3.0, chess art
CC-BY-SA-3.0) or the netcode dependencies, which want a final license glance at bundle time.
