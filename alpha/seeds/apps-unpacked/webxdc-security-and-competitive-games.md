# Webxdc Security Model + Competitive Games (2-4P)

author: research session

scope: companion to the launch-set doc, covering (1) the webxdc security model and what it means for your own iroh integration, and (2) a widened competitive 2-to-4-player candidate set

date: 2026-06-21

note: leaderboard attestation is treated as solved per your call, so it is not re-litigated here.

---

## Part 1: The webxdc security model

### The one-sentence version

A webxdc app is a zip of static HTML/CSS/JS that runs in a **network-isolated webview** with **no internet access at all**, and the only way instances talk to each other is by handing messages to the host messenger, which relays them end-to-end encrypted. The app never sees the network, never sees a server, and cannot phone home, because there is no "home" it is allowed to reach.

That is the whole pitch, and it is genuinely strong. But the *enforcement* of that promise is where the interesting and instructive parts live, and they bear directly on your build because you are writing your own webview host rather than inheriting Delta Chat's.

### How the sandbox is constructed (the layers)

The isolation is defense-in-depth, not a single switch:

1. **Packaging.** The app is a `.xdc`, which is a plain zip. A root `manifest.toml` carries `name` and an optional `source_code_url`. A root `icon.png`/`icon.jpg` is the icon. Everything the app uses must be bundled inside; nothing loads from a CDN.

2. **No remote loading by construction.** The messenger unpacks the zip and serves it to a webview from local files only. External `href` links are blocked, `XMLHttpRequest` and `fetch` to the network do not work, and `src=` to non-bundled resources does not resolve.

3. **CSP plus webview configuration.** Standard Content-Security-Policy directives plus webview-level settings are used to choke off network access and inline-resource loading.

4. **Storage isolation.** Each app instance gets its own isolated `localStorage`, `sessionStorage`, and `indexedDB`, walled off from every other app. (This is why the dev tool runs each instance on its own origin/port.)

5. **The messaging API is the only IO.** Apps get a tiny API: send a state update, receive state updates, plus the newer `joinRealtimeChannel()` ephemeral channel. State updates are persisted and gossiped to the chat; realtime data is ephemeral and peer-to-peer. Both go through the host's audited core, not the network stack.

The headline guarantee the project states: not even the app developer can track or read app state, because the app cannot contact anyone outside the chat.

### The instructive part: CSP alone does NOT contain a webview

This is the single most important security lesson in the ecosystem, and it is exactly the kind of thing that bites a from-scratch integration.

In early 2023, a contributor (WofWca, the same person behind the Quake port) discovered that `RTCPeerConnection` (WebRTC) is **not constrained by CSP or the standard webview network-isolation options**. A sandboxed webxdc app could open a WebRTC connection and exfiltrate data, fully defeating the privacy promise. Then an OpenTechFund-funded Cure53 audit found a *second* class of leak: Chromium's **DNS-prefetching** could be abused to exfiltrate data via DNS queries, and the official "disable dns-prefetch" knob did not actually work. The audit logged five high-severity issues in total, including two full CSP bypasses (one via `webxdc.js` injection, one via PDF embed) and a dev-tools exfiltration path.

The fixes are worth knowing because they are so blunt:

- **WebRTC on Chromium** is killed with a hack they named FILL500: instantiate 500 `RTCPeerConnection` objects at startup and never close them, hitting Chromium's hard-coded per-process cap of 500 so that no further connection can ever be created. On WebKit/iOS the fix is cleaner: delete `RTCPeerConnection` from the JS namespace entirely. FILL500 can add 3 to 5 seconds of startup delay on old phones, and they shipped it anyway because they consider the privacy promise more fundamental.

- **DNS-prefetch and other network paths** on Desktop (Electron) are handled by blocking DNS in the renderer process outright, allowing only an explicit mapbox exception for opt-in location streaming.

The takeaway, in their own words: platforms that serve web code must trust their entire JS supply chain, because **CSP does not prevent leakage**. The W3C did finally adopt a `webrtc 'block'` CSP directive in 2022, but browsers had not implemented it at the time of the audit, which is why the FILL500 hack still exists.

### What this means for YOUR architecture

You are not using Delta Chat's webview host. You are wrapping your own (Tauri, which uses the platform webview: WKWebView on Apple, WebView2/Chromium on Windows, WebKitGTK on Linux). So you inherit the *problem* but not their *fixes*. Concretely:

- **The WebRTC exfiltration hole is yours to close.** If your games pond runs untrusted or semi-trusted game code in a Tauri webview, that code can potentially open a `RTCPeerConnection` and leak. You need an equivalent mitigation per platform. Tauri does give you a real advantage here: you can inject an initialization script before app code runs, and you control the webview config, so the "delete `RTCPeerConnection` from the namespace" approach (the WebKit fix) is available to you on at least the Apple side. Verify per-platform, because WebView2 is Chromium and may need the FILL500-style treatment or a newer `webrtc 'block'` CSP if the embedded Chromium is recent enough to support it.

- **DNS-prefetch and speculative connections** are a Chromium-family concern (WebView2, WebKitGTK to a lesser degree). Audit whether your webview does speculative DNS and whether you can disable it at the embedding layer.

- **There is a clean irony to lean on.** Your transport is iroh QUIC, not browser WebRTC. So for *your own* game networking you do not need or want the webview's WebRTC at all, which means disabling WebRTC in the webview is pure upside: it closes the exfiltration hole and costs you nothing, because your P2P goes through the Rust core exactly the way Delta Chat routes through its audited core. This is a genuinely nice fit between your stack and the threat model.

- **Supply chain is the real attack surface.** Because the sandbox is the only thing standing between a malicious bundled dependency and your users, your vetting story matters as much as the sandbox. This dovetails with how you already think about provenance: a hand-picked launch set, reproducible builds where possible, and treating every bundled npm/crate dependency in a game as untrusted code that the sandbox must contain. The webxdc folks explicitly note they would prefer source-built apps (F-Droid style) over downloading prebuilt `.xdc` releases, for exactly this reason.

- **Your "different ponds, honest seams" model is a security boundary, not just a UX one.** The games pond runs third-party game code; your social panels run your own code against ATProto/ActivityPub. Keeping those as separate webview contexts (separate origins, separate capabilities) means a compromised game cannot reach into the social session's tokens or DID keys. Worth making that seam a hard process/context boundary, not just a UI tab.

### Threat-model summary table

| Threat | webxdc's mitigation | Your situation |
|---|---|---|
| App phones home via fetch/XHR | CSP + webview network isolation | Same approach; verify in Tauri per platform |
| App exfiltrates via WebRTC | FILL500 (Chromium) / delete RTCPeerConnection (WebKit) | You must replicate; but you can just disable WebRTC since iroh is your transport |
| App exfiltrates via DNS-prefetch | Block DNS in renderer (Electron) | Chromium-family webviews need an equivalent check |
| Malicious dependency in bundle | Sandbox containment + curation | Curation is yours; lean on reproducible builds |
| Cross-app/cross-pond data theft | Per-app storage + origin isolation | Enforce a hard context boundary between games pond and social panels |
| App spoofs another user's identity | Noted as XDC-06 info issue (selfAddr spoofable) | In your model, identity comes from your DID layer, not a self-reported field, which is stronger |

That last row is a real advantage of your design. webxdc's `selfAddr` was spoofable in-app; your cryptographic-identity layer (did:webvh anchored, platform DIDs via `alsoKnownAs`) gives a game a verifiable identity assertion instead of a self-reported string, if you choose to expose it. Be careful about *how much* you expose, though: a game that can read a player's stable DID is a game that can correlate players across sessions, which is a mild deanonymization vector. Consider handing games an ephemeral per-match pseudonym by default and only exposing durable identity on explicit user action.

---

## Part 2: Competitive games, widened (2 to 4 players)

Organized by tempo, because tempo is the binding constraint over a relayed P2P transport. Within each, the strongest competitive picks first. License and extractive flags carried throughout. Reminder from the transport analysis: you can expose a direct iroh QUIC stream per peer pair, so you are a *better* host for real-time WebRTC ports than webxdc itself, which only has a broadcast channel.

### Turn-based competitive (perfect fit, cryptographically simple)

These are the robust core. All perfect-information unless noted, so divergent state is self-detecting and there is no hidden-info wrinkle.

- **Connect Four** (2P). ArcaneCircle/c4, **MIT**. Build-fresh. The best hook on the whole list; covered in the launch doc.

- **Reversi / Othello** (2P). ArcaneCircle/Othello, **GPL-3.0**. Build-fresh. Swingy, deep, fast. A genuine banger.

- **Checkers / Draughts** (2P). Many MIT impls; rules unprotectable. Build-fresh. Decide forced-capture up front.

- **Chess** (2P + spectators). ArcaneCircle/chess, **GPL-3.0**, art CC-BY-SA 3.0. Credibility pick, not the hook. Watch the art copyleft.

- **Go** (2P). 9x9 for speed, 19x19 for depth. BUGOUT as reference. Build-fresh; defer scoring to manual agreement.

- **Backgammon** (2P). Rules unprotectable. Build-fresh. Carries a **dice-fairness wrinkle**: use a commit-reveal coin-flip (both peers hash a seed, reveal, XOR) for provably fair dice. Small and worth doing.

- **Dominoes** (2 to 4P). Rules unprotectable; open implementations exist but most are server-bound. Build-fresh. The draw pile is hidden information, so it needs the trusted-dealer pattern. Good four-player option.

- **Ultimate Tic-Tac-Toe** (2P). Nokse22/ultimate-tic-tac-toe and others; genuinely strategic unlike plain tic-tac-toe. Build-fresh, trivial. A surprisingly good quick-play competitive filler.

- **Dots and Boxes** (2 to 4P). Rules unprotectable. Build-fresh, trivial state. Scales to 4 players cleanly and has real tactical depth. Underrated pick.

- **Four-player chess** (4P). board-game/chess-variant projects exist; niche but a real draw for the chess crowd. Higher build cost; defer.

Hidden-info note for this tier: only the card/tile games (Dominoes, and any trick-taker) need the dealer pattern. Everything board-based is perfect-information and clean.

### Slow-real-time competitive (good fit, jitter-tolerant)

The social heart of the set. Tens of milliseconds of latency are invisible here.

- **Drawing-and-guessing (skribbl-style)** (3 to 8P, competitive scoring). Rebuild; skribbl.io is proprietary and open clones are server-bound. The single best group-chat-keeper on the list. Strokes fit a broadcast channel perfectly. Slow-real-time.

- **Draw Battle / competitive team drawing** (teams). Same genre, team-scored variant. Same build approach.

- **Bingo / number-draw races** (2 to many). Trivial to build, competitive, kid-and-grandparent friendly. The "house" RNG needs a fair-draw mechanism (commit-reveal or a designated dealer), same class as dice.

- **Typing/word races** (2 to 4P). Race to type or find words on a shared seed. Build-fresh; effectively a real-time variant of your daily-word hook.

### Twitch-real-time competitive (works on direct connection, degrades on relay)

Reserve these for "look what's possible," not onboarding. They need a direct hole-punched iroh connection to feel good; on relay fallback they will get sluggish. The netcode toolkits below are the key enablers.

- **Curve Fever / Achtung die Kurve / Zatacka** (2 to 8P). The standout real-time arena pick. Two-button controls (turn left, turn right), instantly legible, genuinely thrilling with friends, scales to a crowd. **Curvytron** is the networked multiplayer version and is **MIT** (development halted, so treat as a fork base, not a live dependency). Other impls: achtungkurve.com (GPL-3.0), an Elm "Classic" (AGPL-3.0), various C/C++/Lua remakes. This is the best fun-to-onboarding-weight ratio in the twitch tier; the game is famous precisely for being learnable in one sentence. Port or build-fresh; the logic is simple (constant-speed turning, trail collision), the hard part is netcode, which is where the toolkits come in.

- **Pong / air hockey** (2P). Trivial, universally legible, a fine twitch demo. Many impls but most are WebSocket/server. Build-fresh on a netcode lib. Tempo is forgiving-ish because the action is one-dimensional.

- **Bomberman-style** (2 to 4P). internaut/p2p-bomberman is a real WebRTC-over-PeerJS 4-player implementation under a custom BSD-3 license; dated. Good study specimen for the porting recipe. Twitchy tempo.

- **Platform fighter / physics brawler** (2 to 4P). netplayjs ships fighter/physics demos. Genuinely fun, but the highest tempo sensitivity here. Demo-tier.

#### The netcode enablers (this is what makes the twitch tier feasible)

- **netplayjs** (rameshvarun/netplayjs, ~557 stars, TypeScript). Rollback netcode plus WebRTC, with a host-authoritative state-correction model. It uses a matchmaking server only for signaling, which is the exact swap point: replace its signaling/transport with your iroh channel and you inherit rollback for free. Its host-authoritative correction also doubles as a ready-made dealer for hidden-info games. Confirm the repo license before bundling.

- **GGRS / matchbox** (Rust). If you want rollback *in your Rust core* rather than in JS, GGRS is a pure-Rust GGPO-style rollback library and matchbox is its WebRTC-ish P2P signaling layer. This is the more architecture-aligned path for an all-Rust core, and worth a look precisely because it lets the netcode live where your game logic already wants to be. Verify current crates and licenses.

### Quick-reference: competitive picks by player count

| Players | Best picks |
|---|---|
| Strictly 2P | Connect Four, Reversi, Checkers, Chess, Go, Backgammon, Pong, Ultimate Tic-Tac-Toe |
| 2 to 4P | Dominoes, Dots and Boxes, Bomberman, platform fighter |
| 3 to 8P (party) | Drawing-and-guessing, Curve Fever, Bingo, word races |

---

## Net recommendation change

The security review does not change the launch set, it *reinforces* it: the perfect-information turn-based core is not only the easiest to build and license-cleanest, it is also the one with no fairness-wrinkle attack surface, which keeps your first impression robust.

For competitive breadth, the two additions worth prioritizing are **Curve Fever** (the real-time crowd-pleaser, MIT via Curvytron as a fork base) and **Dots and Boxes** plus **Dominoes** (cheap, deep, 2-to-4-player coverage). The fighting/shooter tier stays a "look what's possible" demo gated behind a direct iroh connection.

The one security action item that is genuinely yours and not inherited: **disable WebRTC in your Tauri webview per platform**, which closes the same exfiltration hole that cost Delta Chat months, and costs you nothing because iroh is already your transport.

### Sources and verification notes

The security model is drawn from the webxdc spec pages (format, messenger, compatibility), the ArcaneChat/Delta Chat core `webxdc.md`, and most importantly the Delta Chat write-up of the Cure53 audit (the FILL500 hack, DNS-prefetch exfiltration, the five high-severity findings, and the "CSP does not prevent leakage" conclusion). Game repos and licenses were confirmed via GitHub pages and the osgameclones/Arch listings; Codeberg remained unreachable from this sandbox, so any Codeberg-hosted variant should be re-confirmed directly. Confirm netplayjs and GGRS/matchbox licenses against their repos before bundling, since those are the load-bearing netcode dependencies.
