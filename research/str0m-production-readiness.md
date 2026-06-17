# str0m production readiness — verification for Croft's real-time media path

author: verified by user (Chase), captured here

date: 2026-06-16

status: research note. Verifies the production-maturity claim that gates challenge **C2** in
`discovery/thinking/realtime-media-over-iroh.md`. Production-use claims go stale or get overstated, so
this separates **confirmed production use** from cases that can't be fully stood behind, and flags what
to re-confirm.

---

## Why this note exists

The real-time-media design leans on str0m as the sans-IO media engine ("keep WebRTC's media engine,
throw away WebRTC's transport"). That only holds if str0m is actually production-grade *in the parts we
use*. The headline finding: **str0m's production track record is strongest exactly where we'd lean on
it (server-side SFU) and thinnest exactly where we'd otherwise have worried (peer-to-peer ICE) — and
our architecture already bypasses the weak part.** Details below.

## Confirmed

- **Lookback — the origin and primary production user.** str0m was created by Martin Algesten of
  Lookback, and they run it in production as a **server-side SFU**, not peer-to-peer. The team is
  explicit that the **SFU path is the heavily tested and developed part** (their actual use case), and
  — directly relevant to us — that the **peer-to-peer areas, mainly the ICE agent, have not received as
  much attention or testing.**
- **rust-libp2p adopted str0m for its WebRTC transport** (switching from webrtc-rs). The maintainers'
  reasoning: webrtc-rs is a very heavy dependency that bloats the tree, and its async-callback design
  is not idiomatic Rust; str0m already supported data channels in tests, which made the replacement
  viable. A meaningful signal given libp2p's reach. **[CONFIRM current state — the switch issue dates
  to 2023; verify the migration actually landed and its present status.]**

## Caveats on the broader "who uses it" question

- The maintainers **actively solicit production reports** (asking people building SFUs/MCUs to file an
  issue and tell them). That phrasing implies there is **no large public roster** of named production
  deployments — so beyond Lookback and the libp2p integration, do not claim a long list.
- The shipped **example code carries an explicit warning**: the chat/SFU example demonstrates the API
  but is **not intended for production or heavy load.** So the *building blocks* are production-grade per
  Lookback's own use, but the *reference examples* are not drop-in — budget real integration work.

## Relevant moving target: webrtc-rs is going sans-IO too

webrtc-rs is moving toward a **sans-IO architecture in its v0.20.0 work**, with a runtime abstraction
meant to fix the exact callback/Arc-cloning problems that drove libp2p toward str0m. So the
architectural gap that made str0m the obvious pick **may narrow over time.** For now, webrtc-rs's own
guidance is to stay on the **v0.17.x branch for production.** Keep webrtc-rs on the radar as a hedge,
not as today's choice.

## Implication for Croft — favorable, and it sharpens a sub-decision

This maps onto our two interop modes (`realtime-media-over-iroh.md` §"str0m & WebRTC interoperability")
better than expected:

- **Mode B (browser bridge — meer as a server-side SFU)** is *exactly Lookback's tested use case.* The
  meer terminating browser WebRTC and forwarding is server-SFU-shaped, str0m's strongest, most-exercised
  path. This is reassuring for the part we were least sure about.
- **Mode A (native fold — iroh is the wire)** *bypasses str0m's weakest part.* str0m's under-tested area
  is its **P2P ICE agent** — and in Mode A we don't use str0m's ICE at all; **iroh provides NAT
  traversal/identity/hop-encryption.** So the verification is a direct argument for the **A1
  (media-layer-only, bypass str0m ICE/DTLS)** branch of the A1-vs-A2 sub-decision over **A2 (full str0m
  session tunneled)**, because A2 would drag in exactly the ICE machinery Lookback says is least tested.
- **Net:** we lean on str0m where it is strong (server SFU) and route around it where it is weak (full
  P2P ICE), using iroh. That does not eliminate risk — anywhere we *do* touch str0m's ICE (e.g. the
  browser-facing leg of the SFU-meer) we exercise a less-hardened path and should budget for it — but it
  means the worry in C2 is smaller than "is str0m production-ready" in the abstract.

## To confirm / open

- **[CONFIRM]** current state of the rust-libp2p → str0m migration (issue is 2023).
- **[CONFIRM]** whether the strong/weak boundary is precisely *server-side/ICE-lite (tested)* vs *full
  P2P ICE hole-punching (under-tested)* — this determines how much the browser-facing SFU leg (which
  does terminate browser ICE) sits on the tested vs untested side.
- **[TRACK]** webrtc-rs v0.20.0 sans-IO progress as the hedge option.
- These feed **TC-ENG0** (the str0m API audit that gates the media work) and **TC-INT3** (the A1-vs-A2
  native-transport decision), which this note now biases toward A1.

## Sources

User-conducted verification, 2026-06-16: str0m repository/README and maintainer (Martin Algesten /
Lookback) statements on SFU-vs-P2P testing focus; rust-libp2p issue proposing the webrtc-rs → str0m
switch (data-channel support, dependency-weight/callback rationale; ~2023); str0m maintainers'
call-for-production-reports and the example-code "not for production" warning; webrtc-rs v0.20.0
sans-IO roadmap and v0.17.x production-branch guidance. [Secondary/maintainer sources; specific items
flagged [CONFIRM] above were noted by the user as worth re-checking before relying on them.]
