# Raw transcript — Iroh real-time media in the wild, the Rave/Apple trap, scaled use cases (Gemini)

source: design-exploration dialogue, pasted 2026-06-22 · **cleaned-paste, content-faithful — NOT a
byte-pristine export** (PLAYBOOK §4).

> **Provenance & verification (READ FIRST).** AI-assistant (**Gemini**) dialogue; user flags Gemini as
> sometimes unreliable. Fact-checked 2026-06-22 — see
> **`iroh-realtime-media-rave-trap-scaled-usecases-dialogue-2026-06-22-FACTCHECK.md`**. Gemini's failure
> mode: fabricated/overstated specifics (numbers, repo names, quotes, "X is built on iroh") around a
> real skeleton. Treat scale figures, repo names, and the Ari-Lotter quote as suspect until confirmed.
>
> **HEAVY OVERLAP — cite, don't redo.** The corpus already covers most of this, more rigorously:
> the **Rave trap** is fully treated in `thinking/abuse-resistance-and-the-rave-trap.md` (verified
> 2026-06-16: iroh+MoQ, Aug-2025 removal, 5-country May-2026 antitrust suits, SharePlay motive, the
> porn/piracy/CSAM + unmoderated-public-rooms reason, the "worked too well both ways" framing, and
> Croft's Signal-shaped answer). **Real-time media over iroh** is treated in
> `thinking/realtime-media-over-iroh.md` (callme = iroh-roq + Opus + cpal as the proven floor; str0m
> ceiling). Nous Research (distributed LLM training) + Paycode (POS) are already CONFIRMED in
> `atproto-atmospheric-web-iroh-mobile-FACTCHECK.md` (the iroh source-of-truth; iroh 1.0.0). Net-new
> here = specific media references (irl / frando-GStreamer / moq-dev) + scaled-case detail (Psyche/
> DisTrO, p2proxy, Drift, the "secure security camera" blog). UI citation chips + image captions
> stripped per §4. Do not copy as working code.

---

**[user]** can you find use cases in the wild of iroh being used for real time video or audio as
references?

**[assistant]** Iroh evolved from rewriting IPFS components into a modular P2P networking toolkit in
Rust; built entirely on QUIC, it's a compelling alternative to WebRTC for low-latency real-time audio/
video. References "in the wild":

1. **Rave: watch-parties / video streaming at global scale.** A massive consumer media-sharing app —
   millions sync up to watch videos together in real time. Stack: evaluated libp2p and WebRTC before
   migrating to Iroh using **Media over QUIC (MoQ)**. Why iroh: mobile networks are volatile; WebRTC's
   media stack + CPU overhead were less efficient than iroh's QUIC stream multiplexing; graceful
   degradation under packet loss + seamless NAT traversal. Scale: **>600,000 concurrent connections per
   relay**; **5 global self-hosted relay locations**, 24/7.
2. **Low-latency P2P game streaming.** n0.computer highlighted interactive game-streaming (host desktop
   → client mobile, self-hosted-cloud-gaming-like) with strict latency bounds; bidirectional
   high-framerate video + real-time controller inputs over raw QUIC streams; sub-frame delivery, E2EE
   by default.
3. **The official iroh GStreamer & MoQ reference.** iroh core dev **frando** released a blueprint for
   raw real-time A/V on iroh: Rust + Iroh + **GStreamer**; sender captures mic (→ **OPUS**) + camera
   (→ **VP8**), wraps them in an Iroh RTP sync channel; receiver pipes raw streams into GStreamer's
   **decodebin3**. The magic: **poll-based streaming** — streams are lazily created; the host won't
   waste battery/compute encoding video until a viewer connects and requests the stream.

Why teams choose iroh over WebRTC for media:

| Feature | WebRTC | Iroh (MoQ) |
|---|---|---|
| Connection setup | Complex external signaling + heavy STUN/TURN | Native NAT traversal + encrypted fallback relays in one library |
| Network adaptation | Stutters/drops on cellular handoffs | QUIC — handles packet loss without tearing down |
| Resource footprint | Heavy baseline CPU; hard on IoT | Lightweight Rust; scales down to ESP32 / Raspberry Pi |

**[user]** any more examples? do broad research

**[assistant]** 1. **iroh-live & the `irl` ecosystem** (by n0.computer) — flagship repo for low-latency
bidirectional media. The **`irl` CLI**: `irl publish` grabs camera+mic, spins up an Iroh node, prints a
shareable ticket/QR; a peer `irl play <ticket>` streams end-to-end with sub-second latency. The
cross-platform call client **`callme`**: CLI (`callme-cli`) + GUI (`callme-egui`, Desktop + Android),
using a real-time protocol handler (**`iroh-roq`**) to swap Opus-encoded live audio over raw QUIC.
2. **Production Android bidirectional video-calling reference** (inside iroh-live testing): Android
Camera2/CameraX capture → frames into Rust via a JNI bridge; Rust uses moq-media + rusty-codecs to use
the phone's **MediaCodec** (H.264 video, Opus audio); receive side uses **zero-copy EGL AHardwareBuffer**
imports to render the live video grid on the GPU.
3. **`moq-dev/moq` native P2P mesh** — open-source IETF Media-over-QUIC; natively targets CDN scale
(Cloudflare) but integrates Iroh for decentralized P2P transport. A broadcast ingested via OBS/FFmpeg
is distributed across a localized mesh of viewer nodes, gossiped/discovered via Iroh, peers sharing
chunks laterally without overloading the origin.
4. **Decentralized secure smart-home cameras** — the iroh blog "**What if your security camera was
secure?**" (IoT). Traditional cameras (Ring/Nest) upload to centralized cloud (privacy + bandwidth
cost). Iroh solution: embed Iroh on a low-power SBC/microcontroller; the camera is a closed Iroh
endpoint; **poll-based delivery** — near-zero bandwidth/CPU when idle; opening the app "dials" the
camera's public key, punches the home NAT, establishes an E2EE MoQ tunnel, streams directly device→
screen.

Architecture (modular blocks, not a monolith):
```
+-------------------------------------------------------+
|                 Your Application Layer                |
|     (Captures Camera/Mic -> Encodes to H.264/Opus)    |
+-------------------------------------------------------+
                           |
                           v
+-------------------------------------------------------+
|      Protocol Layer (e.g., moq-transport / roq)       |
|     (Frames media chunks, assigns packet priority)    |
+-------------------------------------------------------+
                           |
                           v
+-------------------------------------------------------+
|                Iroh Transport Layer                   |
|   (Handles E2EE, NAT hole-punching, & Relay fallback) |
+-------------------------------------------------------+
```

**[user]** why did apple remove rave from it's appstore?

**[assistant]** Apple removed the co-viewing app **Rave** from the App Store in **August 2025**,
triggering a legal battle. In **May 2026**, Rave filed antitrust lawsuits against Apple across **five
countries (U.S., Canada, Brazil, the Netherlands, Russia)**. Two very different stories:
1. **Apple's perspective: safety/privacy/content moderation** — "repeated guideline violations":
   severe lack of content moderation (unmoderated public chat/viewing rooms → spam/scams/bots); adult
   + pirated content (and CSAM/minor-exposure concerns); security/malware flags — Apple revoked Rave's
   **macOS developer certificate** (Macs blocked it w/ a malware warning); Google, Microsoft, Kaspersky,
   BitDefender also flagged/pulled it at various points.
2. **Rave's perspective: anti-competitive pretext** — the **SharePlay threat** (Rave is fully
   cross-platform — iOS/Android/Mac/Windows watch together — threatening Apple's closed ecosystem);
   vague justifications (Apple hid behind "dishonest or fraudulent activity" + "vague concerns about
   content moderation"); aggressive lockouts (disabling "**Sign in with Apple**" locked **11.4 million
   users** out of years-old accounts).
Current status: Rave launched **SaveRave.com** and rolled out an AI-driven moderation system (**a-eye**)
to prove it can keep users safe. But given Apple's zero-tolerance toward unmoderated adult content +
piracy, Rave remains unavailable on iOS/macOS while continuing on Android/Windows.

**[user]** so it worked /too well/

**[assistant]** The ultimate double-edged sword of high-performance decentralized P2P. (1) Competitive:
it shattered ecosystem lock-in — SharePlay only works if everyone is on Apple; Rave let iPhone+Android+
Windows sit in the same room with synced sub-second video, bypassing the Apple tax + walled garden,
out-performing Apple's native feature cross-platform. (2) Architectural: ultra-efficient direct pipelines
are natively designed to route around central choke points. Traditional apps funnel media through a
cloud server where content can be inspected/filtered; P2P/QUIC prioritizes direct encrypted delivery.
Because the architecture was so good at letting data move without friction, proactive content moderation
became a massive hurdle — it became a Wild West of pirated movies + unmoderated rooms, giving Apple the
ammunition to drop the hammer under "user safety." The design dilemma: how to build a high-performance
decentralized media protocol without inheriting the platform liabilities of an unmoderated network?

**[user]** yes, how not to fall into the same trap

**[assistant]** Solve the paradox: leverage a decentralized privacy-first stack while satisfying
centralized risk-averse App Store reviewers (else Guideline 1.2 UGC ban). Strategies:
1. **Decouple the control plane from the media plane.** Never do room discovery / text chat / link
   sharing purely P2P; run auth + room metadata + link filtering + reports through a central control
   server (WebSockets/HTTPS) that approves the connection + exchanges Iroh tickets, while Iroh/MoQ
   streams raw A/V E2EE. Run automated moderation APIs (Hive AI / AWS Content Moderator) on text + URLs
   before they reach users.
```
+--------------------------------------------------------+
| Central Control Server (WebSockets/HTTPS)              |
| -> Handles Auth, Room Metadata, Link Filtering, Reports|
+--------------------------------------------------------+
                           |  (Approves Connection &
                           |   Exchanges Iroh Tickets)
                           v
+--------------------------------------------------------+
| Iroh / MoQ Media Plane (Direct P2P / Relays)           |
| -> Streams Raw Audio/Video Encrypted End-to-End        |
+--------------------------------------------------------+
```
2. **Eliminate "public discovery" by default.** Treat the app as a private utility, not a social
   network; lean on iroh's capability-based security — connections require an explicit invite token /
   cryptographic ticket / scanned QR shared out-of-band. No "Public Rooms" tab → far less liability +
   fewer reviewer flags.
3. **Hardcode the big three UGC requirements.** Block button (terminate the Iroh QUIC stream
   `conn.close()` + blackhole that Peer ID locally); report button (take a temporary localized 1-second
   buffer snapshot of the media canvas, upload to the central server only on Report — works even with
   E2EE); centralized kill-switches (the signaling server can revoke session tokens, preventing a node
   from registering).
4. **Limit the media ingest pipeline.** No raw unrestricted in-app browser; strict URL allowlists; for
   co-watching use official sandboxed IFrames/SDKs (YouTube/Twitch); enforce CORS + domain verification
   server-side before broadcasting a track namespace.
5. **Edge-based AI moderation.** Shift compute to the client (CoreML / MediaPipe) to sample the local
   camera/canvas every few seconds; if explicit content is detected, cut the Iroh broadcast locally +
   alert the backend.
The golden rule: **App stores don't care if your transport is decentralized; they care if your
governance is.** Use iroh to move high-bandwidth bytes with zero latency; keep a centralized hand on
identity + compliance.

**[user]** any other interesting, unexpected, or scaled iroh use cases worth learning from?

**[assistant]** Because iroh replaces IP/DNS with cryptographic public keys + UDP hole-punching:
1. **Global distributed AI training (the Psyche Network & Nous Research).** Nous Research deployed
   **DisTrO** (Distributed Training Over-the-Internet) on the **Psyche Network**, using Iroh as
   transport. Architecture: a decentralized swarm where everyday nodes (gaming PCs, independent AWS/GCP/
   Azure instances) pool idle GPU to cooperatively train massive models — runs up to **40-billion
   parameters / 20-trillion tokens**. How iroh is used: **iroh-gossip** for high-frequency state sync/
   coordination across thousands of nodes; **iroh-blobs** bundles raw gradient updates into
   content-addressed binary files peers share laterally. Because DisTrO aggressively compresses
   inter-GPU data, latency is no longer the bottleneck — Ari Lotter (Principal Engineer, Nous): *"Doubling
   the network speed halves our compute budget."*
2. **Serverless Point-of-Sale payments & PCI compliance.** Iroh baked into PoS hardware on lightweight
   embedded systems; a portable card reader dials the register tablet's Iroh public key; devices
   discover over Bluetooth/LAN/Wi-Fi, bypassing firewalls; QUIC/TLS 1.3 E2EE + cryptographic auth →
   easier PCI compliance (no intermediary cloud interception point).
3. **Zero-config web infrastructure (`iroh-ssh` & `p2proxy`).** **iroh-ssh** — SSH into a remote machine
   with no IP / no inbound port, just the target's public key. **p2proxy** — a P2P reverse proxy: a local
   daemon bridges a TCP port over an Iroh QUIC tunnel, so you reach a home server behind double-NAT from
   anywhere by mapping it to a local loopback. (Replaces mesh VPNs like Tailscale / dynamic-DNS hacks.)
4. **Cross-platform local data transfer (`Drift`).** Open-source cross-platform file transfer on Iroh;
   zero-config any-OS sharing via iroh-blobs (fully resumable, verified byte-streams — close the laptop
   mid-transfer of a 10 GB file and it resumes when devices reconnect, no cloud).

Architectural comparison:

| Use Case | Core Module | Traffic Pattern | Network Priority |
|---|---|---|---|
| Distributed AI training | iroh-gossip & iroh-blobs | Many-to-many swarm | Maximize raw bandwidth saturation |
| Payment terminals | Raw `iroh::Endpoint` | 1:1 direct local/mesh | Strict cryptographic identity verification |
| DevOps (p2proxy) | QUIC stream multiplexing | 1:1 tunneled stream | Bypassing corporate firewalls |
| File sharing (Drift) | iroh-blobs | 1:1 verified chunks | Resumability + data integrity |

Key insight: every scaled deployment treats the network as a **flat plane** — shifting trust/routing to
cryptographic keys rather than volatile IP config makes the local router / global distance secondary.
