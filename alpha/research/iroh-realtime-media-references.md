# iroh real-time media — references and the RoQ-vs-MoQ split

author: captured by Claude from user-supplied leads + web verification

date: 2026-06-16

status: research note. Reference deployments and the two QUIC media protocols that bear on
`thinking/realtime-media-over-iroh.md`. Confirmed-vs-unverified separated, because production/scale
claims overstate easily (cf. the str0m/libp2p correction in `str0m-production-readiness.md`).

## The headline: two QUIC media protocols, two interaction types

There is not one "media over iroh" — there are **two protocols for two use cases**, and they map onto
the messaging interaction tiers:

- **RoQ — RTP over QUIC.** Conversational. RTP frames over QUIC (datagram flow for media). Lowest
  latency; symmetric; point-to-point or SFU. **n0's `callme` uses RoQ** (`iroh-roq`). A latency
  comparison found **RoQ achieves the best latency** of WebRTC/MoQ/RoQ. → voice/video *calls*.
- **MoQ — Media over QUIC.** Broadcast/streaming. A **pub/sub** model (Kafka/MQTT-shaped, adapted for
  media): a broadcaster publishes named **Tracks** (separate video/audio/metadata tracks), viewers
  **subscribe**; relay-based, internet-scale, **sub-250 ms**, one cheap QUIC stream per frame, and
  **lazy** — *streams are only created when someone subscribes; the host does not encode/transmit until
  a viewer connects* (the battery/compute/privacy win). **n0's `iroh-live` and the Rave app use MoQ.**
  → stage / watch-party / livestream / town-hall. An IETF effort with Cloudflare/Fastly/Wowza backing.

This is the protocol-level basis for "types of media interaction akin to types of groups."

## Confirmed (web-verified 2026-06-16)

- **n0 `iroh-live`** — "media livestreaming over iroh." Uses **moq-rs** (Media over QUIC in Rust) to
  carry audio/video over iroh connections; supports **h264 + Opus via ffmpeg**, hardware-accelerated
  encode where available, **GStreamer + ffmpeg** ingest, and a room-ticket connect for video+audio
  chat. This is the canonical GStreamer/MoQ-on-iroh reference (the "frando blueprint" the lead
  referred to). Repo: `github.com/n0-computer/iroh-live`.
- **n0 `callme`** — P2P audio over **`iroh-roq`** (RTP-over-QUIC, datagram flow) + Opus + cpal, no
  WebRTC; experimental. (Detail + source read in `realtime-media-over-iroh.md`.)
- **Rave uses iroh + MoQ** for video streaming "to reach millions of devices," after evaluating
  **libp2p and WebRTC** first — confirmed on iroh's own solutions page (`iroh.computer/solutions/video`).
  Cited reasons: WebRTC's complex media stack + CPU overhead vs QUIC's stream multiplexing; graceful
  degradation on lossy mobile networks; built-in NAT traversal + encrypted relay fallback; lazy
  per-subscriber streams; you control who can see a stream (no third-party server in the middle).
- **iroh is QUIC-native** and positioned explicitly as a WebRTC alternative for low-latency media; the
  same NAT-traversal/relay machinery we proved (E0–E7) is what these media stacks ride on.

## [UNVERIFIED] — flagged, do not rely on without confirmation

- **"600,000 concurrent connections per relay" and "5 global self-hosted relay locations" for Rave.**
  iroh's page says "millions of devices," but I did **not** confirm the specific 600k/relay or 5-region
  figures from a primary source. Treat as marketing-order-of-magnitude, not a measured ceiling — our
  own E-series (E0 memory wall ≈130k idle conns on a 4 GiB slice) is the number we trust. The 600k
  figure, if real, likely reflects fatter relays and/or active-stream (not idle) accounting; confirm
  before quoting.
- **"Low-latency P2P game streaming" production use** (bidirectional video + controller input,
  sub-frame). Plausible and on-thesis (QUIC streams, e2e encryption) but I found no primary
  confirmation in this pass; treat as illustrative.
- **"scales down to ESP32 / Raspberry Pi."** Pi is plausible; ESP32 (a microcontroller) running a full
  iroh/QUIC media path is a strong claim — unconfirmed.

## Why teams cite iroh/QUIC over WebRTC for media (the recurring argument)

- **Connection setup:** WebRTC needs external signaling + STUN/TURN coordination; iroh has native NAT
  traversal + encrypted relay fallback in one library (the CSR "signaling without a signaling server"
  idea, realized).
- **Network adaptation:** QUIC degrades gracefully under loss / cellular handoff instead of tearing
  down; WebRTC can stutter/drop aggressively.
- **Footprint:** a lightweight Rust QUIC path vs WebRTC's heavy baseline CPU.
These mirror our own reasons for the "fold WebRTC's transport into iroh" stance — and MoQ/RoQ mean we
may not need WebRTC's *engine* either for many cases (iroh-live/callme prove it).

## Implications for Croft (feeds the design + the meer doc)

1. **Adopt the two-protocol split as media interaction types** (see `realtime-media-over-iroh.md`
   §"Media interaction types"): RoQ for conversational, MoQ for broadcast. Chosen at creation by use
   case, exactly like the messaging tiers.
2. **The meer gets a broadcast role:** a **MoQ relay** (forwards Tracks it needn't decode → blind) is
   the media analog of the message blind broker, and the lazy property means it costs nothing until a
   subscriber appears — perfectly aligned with the interaction-tiers "nothing to fan out if nobody is
   watching" philosophy. (See `thinking/meer-superpeer-design.md`.)
3. **We can lean on `iroh-live` / `callme` as references**, not build the media path from scratch —
   the same posture as the messaging spine reusing proven primitives.

## Sources

- [Video Streaming with MoQ — iroh solutions](https://www.iroh.computer/solutions/video) (Rave, MoQ, lazy streams, NAT traversal)
- [n0-computer/iroh-live](https://github.com/n0-computer/iroh-live) (MoQ + GStreamer/ffmpeg, h264/Opus)
- [moq.dev](https://moq.dev/) and [moq-dev/moq](https://github.com/moq-dev/moq) (Media over QUIC standard/impl)
- [Cloudflare: MoQ — refactoring the internet's real-time media stack](https://blog.cloudflare.com/moq/)
- [Fastly: Media over QUIC — scale and low latency](https://www.fastly.com/blog/media-over-quic-can-streaming-finally-have-both-scale-and-low-latency)
- [QUIC-based vs WebRTC remote-rendering comparison (arXiv 2505.22132)](https://arxiv.org/html/2505.22132v1) (RoQ best latency; WebRTC/MoQ/RoQ over Wi-Fi/5G)
- [MWM: Rave watch-party app](https://mwm.ai/apps/app/929775122)

---

## Addendum (2026-06-16) — second reference batch, verified + corrected

A second batch of leads, run through the same confirmed-vs-flagged filter.

### Confirmed

- **`iroh-live` internals.** Real structure (n0-computer/iroh-live, WIP): capture via **firewheel**
  (audio), **nokwha** (camera), **xcap** (screen); encode/decode **h264 + Opus via ffmpeg**, hardware-
  accelerated where supported; **`iroh-moq`** adapters run **moq-lite** sessions over iroh; **
  `web-transport-rs`** implements web-transport traits for iroh connections. Ships **`rooms`** (prints
  a room ticket → copy to another device → video+audio chat), **`publish`**, and **`watch`** examples.
  This is the canonical real-time-media-on-iroh reference.
- **`moq` (moq-dev) has first-class iroh integration — strong, concrete.** `moq-native`/`moq-relay`
  optionally connect via iroh (off by default, `iroh` feature). URL schemes:
  **`iroh://<ENDPOINT_ID>`** and **`moql+iroh://`** (moq-lite over raw QUIC), **`moqt+iroh://`** (IETF
  MoQ over raw QUIC), **`h3+iroh://…`** (WebTransport over HTTP/3). Connections are **P2P by default**
  with **optional relay bridging to browsers via WebTransport**; each video rendition + audio track is
  an independent QUIC stream. iroh is framed as "simpler and more powerful than WebRTC data channels."
- **Secure-camera / IoT / embedded is a real iroh thrust.** Blog **"What if your security camera was
  secure?"** (2026-03-11) and **"Running iroh on an ESP32"** (2026-03-24) are real posts; there are
  `/solutions/iot` and `/solutions/rave` pages. The **poll-based** pattern (idle device uses ~no
  bandwidth; your phone *dials the camera's public key*, NAT-punches, opens an E2EE MoQ tunnel, streams
  directly) is genuine and on-thesis — and it's the **media instance of "pull from your home node"**
  from `seeds/transcripts/raw/p2p-architecture-origin-dialogue.md`, tied to the lazy + freshness
  philosophy. So **ESP32-scale is corroborated** (a dedicated post), upgrading the earlier flag.

### Flagged / corrected (claims that overstate or invent)

- **`callme` is NOT a sub-project of `iroh-live`** — they are separate repos (`n0-computer/callme` vs
  `n0-computer/iroh-live`). The batch conflated them.
- **`irl` CLI name unconfirmed** — the verified interface is the `rooms`/`publish`/`watch` *examples*,
  not an `irl publish`/`irl play` binary. Capability is real; the command name may be invented.
- **`rusty-codecs` appears invented** — the real Android crate is **`mediacodec`** (Rust bindings to
  Android MediaCodec). **`moq-media`** unconfirmed as a crate. The **zero-copy EGL `AHardwareBuffer`**
  rendering detail is plausible technique but unconfirmed for these repos — treat as illustrative.
- **"Gossiped lateral chunk-sharing P2P mesh"** for moq is embellishment; the confirmed model is
  **per-track QUIC streams, P2P-by-default over iroh + relay bridging**, not gossip-shared chunks.
- (Still flagged from batch 1: Rave's "600k conns/relay, 5 regions" specific numbers — unverified;
  trust our own E-series ceilings.)

### Design refinement this batch forces: **browser reach splits by media type**

`moq`'s `h3+iroh://` (WebTransport over HTTP/3, relay-bridged) is a **cleaner browser path for
*broadcast* media than str0m**. So the browser-bridge story is type-specific:
- **Conversational (RoQ):** browser ↔ str0m-meer over classic WebRTC + SFrame via Insertable Streams
  (Mode B in `realtime-media-over-iroh.md`).
- **Broadcast (MoQ):** browser subscribes via **WebTransport** through a **moq-relay** (the meer's MoQ
  role) — no str0m needed on the broadcast leg. This likely makes broadcast browser reach the *easier*
  of the two.

### Addendum sources

- [n0-computer/iroh-live](https://github.com/n0-computer/iroh-live) (firewheel/nokwha/xcap, iroh-moq, web-transport-rs, rooms/publish/watch)
- [moq-dev/moq README](https://github.com/moq-dev/moq/blob/main/README.md) (iroh feature, iroh:// / moql+iroh:// / moqt+iroh:// / h3+iroh:// URL schemes, P2P + WebTransport bridging)
- [iroh blog](https://www.iroh.computer/blog) — "What if your security camera was secure?" (2026-03-11), "Running iroh on an ESP32" (2026-03-24); [/solutions/iot], [/solutions/rave]
- [mediacodec crate](https://crates.io/crates/mediacodec) (the real Android MediaCodec Rust bindings; "rusty-codecs" not found)
