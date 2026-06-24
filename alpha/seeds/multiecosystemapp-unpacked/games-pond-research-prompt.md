# Research Prompt: Killer Games for a P2P Social Games Pond

Copy this into a fresh session to run the games hunt as its own deep dive. It is
self-contained: it carries the context needed so the new session does not depend
on the originating one.

---

## Context you need

I am building a cross-platform, composable social client. One planned component
(a "pond") is direct peer-to-peer social games. The relevant architecture:

- Live gameplay runs peer-to-peer over **iroh** (a Rust P2P networking library;
1.0 as of June 2026; relay-fallback with hole-punching; mutually authenticated,
end-to-end encrypted connections dialed by public key). The game session is
ephemeral and private; no moves are persisted.

- The **social layer rides on Bluesky / atproto**: a challenge is a normal public
post or a custom lexicon record, which also carries the iroh rendezvous info
(topic + ticket) so peers connect without slow discovery. Optionally, a settled
**outcome** is written as a custom lexicon record (public, attributable, but not
feed-rendering) to drive a public leaderboard.

- The UI is one **web app** (Rust core compiled to WASM, Leptos UI), wrapped by
Tauri for desktop/mobile, also servable as a website/PWA. So games must run as
**web technology** (HTML/CSS/JS/WASM) inside a webview.

- Prior art and homage: **Delta Chat's webxdc** mini-apps do almost exactly this
(small web apps shared in a chat, realtime collaboration over iroh via a
`joinRealtimeChannel()` API). I intend to credit them and likely borrow the
webxdc realtime-channel API shape and porting pattern.

- The **porting recipe** I want to exploit (from the webxdc Quake III port):
take a WebRTC-data-channel multiplayer browser game, polyfill the browser WebRTC
API to use an iroh-backed realtime channel as the transport, mock the
signaling/WebSocket handshake with hard-coded server/client roles, and strip
other direct-internet dependencies. This means **any WebRTC-based browser
multiplayer game is a porting candidate**, not just webxdc-native ones.

## My values (these constrain what counts as a good candidate)

- **Not extractive.** No tracking, no ads, no accounts-as-data-grab, no
engagement-optimized dark patterns. Games that phone home or harvest data are
disqualified. The no-extractive-use-case angle is itself a selling point.

- **User-respecting, low mental load.** Instant start, works first time for a
non-technical user (think "grandma and gen alpha"). Kid-friendly options are
explicitly wanted and valued.

- **Vetted by default.** I will hand-pick a small launch set, not expose an open
store. Quality, onboarding smoothness, and the non-extractive guarantee all
matter, not just "does it run."

- **Open-source / permissively licensed strongly preferred**, because I need to
port or wrap and redistribute, and because it fits a credit-and-commons ethic.
License is a first-class filter.

## The hook thesis (why this matters)

The games-plus-Bluesky loop is meant to be an onboarding hook that teaches the
composable model by being it: see a challenge on a normal Bluesky post, tap,
instantly play a friend peer-to-peer, result posts back. So launch games must be
**instantly legible and socially native**, graspable in seconds, not
impressive-but-heavy. (A multiplayer Quake port is the wrong hook: great ceiling
demo, terrible first impression.)

---

## What I want from this session

Find me **killer-app game candidates** to port or wrap into this pond. Be
comprehensive and concrete: name actual games, actual repos, actual licenses
where findable, and assess portability against the constraints above. Use search
liberally; this is a research task, not a from-memory task. Verify current state
(repos move, licenses change, projects die).

### Specific deliverables

1. **A launch set recommendation.** A small curated set (say 6 to 10) that
together cover the bases below. For each: what it is, why it fits, license, repo
if open source, rough port/wrap difficulty, and any extractive/onboarding risks.

2. **At least 3 "bangers" adults genuinely love.** Not just classics-for-
completeness. Games with real adult appeal and replayability that work in a
turn-based or near-real-time P2P web context. Think the things people actually
keep a group chat alive to play. Argue why each is a banger, not just that it
exists.

3. **A strong kid-friendly subset.** Genuinely good for children: simple,
safe, no chat-with-strangers risk by design (these are friend-to-friend P2P
games), no dark patterns, ideally educational-adjacent or wholesome. This subset
is a values showcase, so quality matters as much as the adult set.

4. **Port vs wrap assessment per candidate.** Distinguish:
   - **Port**: an open-source game whose code I adapt (e.g. swap WebRTC transport
   for the iroh realtime channel per the recipe above).
   - **Wrap**: an existing self-contained web game I can run largely as-is inside
   the webview with thin glue.
   - **Build-fresh**: simpler to write from scratch than to port (true for many
   trivial classics).
   Flag which bucket each candidate is in and why.

5. **The transport-fit analysis.** P2P games have a tempo constraint: turn-based
and slow-real-time fit relayed/iroh transport comfortably; twitch-real-time may
struggle (latency, relay routing for browser peers). For each candidate, note
whether its tempo fits a relay-tolerant P2P transport.

### Categories to cover in the hunt

- Classic 2-player abstract/board games (chess, checkers, go, connect-four,
backgammon, reversi, etc.): which have clean open-source web implementations?

- Word and party games (think the genres that make group chats fun): which exist
as portable web games with permissive licenses?

- Card games (including ones with hidden information, which raises a design
question worth flagging: hidden-info games over pure P2P need either a trusted
dealer or commit-reveal cryptography; note which candidates have this wrinkle).

- Co-op games (not just competitive): co-op fits the friendly, non-extractive
ethic well and sidesteps the leaderboard-attestation problem entirely.

- Kid-friendly / educational games specifically.

- The existing **webxdc game catalog**: inventory what is actually there and
which are worth surfacing vs which are hobby-grade.

- The broader **WebRTC browser multiplayer game** space, since the porting recipe
opens it up. Find notable open-source ones.

### Things to explicitly assess and flag

- **License** for every candidate (MIT/Apache/GPL/proprietary/unclear). GPL is
not disqualifying but changes how I can combine it; flag it.

- **Extractive risk**: does it phone home, require accounts, show ads, track?
Disqualify or flag.

- **Onboarding weight**: instant-start vs heavy-download (the Quake port needs a
multi-hundred-MB data file; that is the anti-pattern for a hook).

- **Hidden-information / fairness wrinkles**: games needing a trusted referee or
commit-reveal, since pure P2P has no server witness.

- **Maintenance/liveness** of the source project.

### Output format

Lead with the recommended launch set as a ranked, scannable list (the bangers and
kid-friendly subset clearly marked). Then the fuller candidate inventory by
category. Then a short section on the cross-cutting design wrinkles (hidden-info
games, tempo/transport fit, the wrap-vs-port-vs-build call). Keep licenses and
extractive-risk flags visible per candidate. Prefer concrete named games and real
repos over abstract categories. Search to verify; do not rely on memory for
current repo/license state.

---

## One meta-note for the session

The deeper goal is a hook that is genuinely fun *and* demonstrates that
software can be social, private, and non-extractive at once. A game that is fun
but extractive defeats the purpose; a game that is principled but boring does not
hook anyone. I need both. Hold that bar.
