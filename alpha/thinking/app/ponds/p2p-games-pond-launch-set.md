# Killer Games for the P2P Social Games Pond

author: research session

scope: candidate hunt for games to port or wrap into an iroh-backed, Bluesky-fronted P2P games pond

date: 2026-06-21

---

## How to read this

Every candidate carries four flags: license, extractive risk, onboarding weight, and transport tempo.

The buckets are **port** (adapt open-source code, swap the transport), **wrap** (run a self-contained web game largely as-is), and **build-fresh** (faster to write than to port).

One correction up front about the porting recipe, because it changes several calls below.

---

## Transport reality check (read before the list)

Your recipe assumes any WebRTC-data-channel game is a porting candidate. That is true, but with a wrinkle worth internalizing, because a Delta Chat core dev (flub) said it plainly in the webxdc-x-WebRTC thread.

The webxdc `joinRealtimeChannel()` API is a **broadcast / gossip channel**, not a point-to-point data channel. Every peer receives every message, there is no per-message metadata, and it is **ephemeral**: a peer who joins late receives nothing sent before they arrived. It is built on iroh-gossip, which sits on iroh QUIC, which sits on IP.

The Quake III port (WofWca/quake3.xdc) works by mocking the browser WebRTC API on top of that broadcast channel and **hard-coding the signaling handshake responses based on whether the instance is server or client**. flub's point: building IP-like point-to-point routing on top of a broadcast channel is "a little bit odd," and the cleaner move is to expose direct iroh QUIC channels and polyfill WebRTC DataChannels against those.

Why this matters for you: you are building your own iroh integration, not riding Delta Chat's. So you are not stuck with the broadcast-only constraint. You can expose **both** a broadcast channel (for lobby, presence, challenge fan-out) and a direct QUIC stream per peer pair (for the actual game). That makes you a better porting target than webxdc itself for any 1v1 WebRTC game, and it sidesteps the late-joiner problem for two-player matches.

Practical tempo guidance:

- **Turn-based** (chess, Connect Four, word games): trivially fine over relayed iroh. Latency is invisible when you are waiting for a human to move.

- **Slow-real-time** (drawing-and-guessing, co-op timing puzzles): fine. Tens of milliseconds of jitter does not hurt.

- **Twitch-real-time** (platform fighters, shooters): works on a direct hole-punched connection, degrades on relay fallback. Acceptable as a "look what's possible" demo, wrong as the onboarding hook.

---

## The recommended launch set (ranked)

Ranked by hook quality: instantly legible, socially native, low onboarding weight, clean license, zero extractive risk. The bangers and kid picks are marked.

### 1. Connect Four — BANGER, KID-FRIENDLY

what: the classic four-in-a-row on a 7x6 grid.

why it fits: this is the single best hook on the list. Graspable in literally one sentence, genuinely fun for a five-year-old and a fifty-year-old, and the board state is tiny. A settled win is unambiguous and trivial to attest to a leaderboard.

license: reference impl ArcaneCircle/c4 is **MIT**. The rules are unprotectable, so even a clean-room rebuild has no license exposure.

bucket: **build-fresh**. The whole game is a 42-cell grid and a win-check. Writing it in your Rust core is faster than porting anyone's JS, and it becomes your reference implementation of the whole challenge-to-outcome loop.

transport: turn-based. Perfect fit.

risks: none. This is your "hello world" pond game.

### 2. Reversi / Othello — BANGER

what: disc-flipping territory game on an 8x8 board.

why it's a banger: deeper than it looks, swingy, fast to play, and adults who think they are bad at "thinky" games still enjoy it because a single move can flip the whole board. Strong replayability in a group chat.

license: ArcaneCircle/Othello is **GPL-3.0**. Rules are unprotectable.

bucket: **build-fresh**. Simple board, well-known flip logic. Same reasoning as Connect Four.

transport: turn-based. Perfect fit.

risks: none.

### 3. A drawing-and-guessing game (skribbl-style) — BANGER

what: one player draws a prompt, others race to guess it in a shared canvas with a timer.

why it's a banger: this is the genre that actually keeps group chats alive. It is social, funny, forgiving of skill, and scales from two players to a whole family thread. It is the strongest "people keep coming back" candidate on the list.

license: skribbl.io itself is proprietary, so do not port it. Several open-source clones exist, but most are server-authoritative (Node + WebSocket) and would need real surgery. The drawing-sync and word-list logic is simple enough to rebuild.

bucket: **build-fresh**, leaning on your own realtime channel for stroke sync. Strokes are a perfect fit for a broadcast channel: every peer wants every stroke, and late strokes do not matter once the round ends.

transport: slow-real-time. Good fit. Stroke updates tolerate jitter.

risks: word-list curation is the only real work. Ship a hand-picked, kid-safe list. No extractive risk in a rebuild.

design note: this is the one case where the **broadcast** nature of the webxdc-style channel is a feature, not a constraint. It is also a clean showcase of "private by design," since the canvas never leaves the peer group.

### 4. Hanabi — BANGER, CO-OP

what: a cooperative card game where you can see everyone's hand except your own, and you spend limited hint tokens to help each other play cards in order.

why it's a banger: it is a beloved, award-winning design (Antoine Bauza) with deep adult appeal, and being co-op it sidesteps the entire leaderboard-attestation problem. The "win together against the game" framing is also a values showcase: it is the opposite of engagement-bait competition.

license: do not reuse the boxed game's name or art. Hanabi-Live (Hanabi-Live/hanabi-live) is the gold-standard open implementation, but it is a Go server plus PostgreSQL plus a TypeScript client, so it is a reference for *logic*, not a drop-in port. Multiple MIT-ish React/Socket.io clones exist (e.g. andrwmillr/hanabi) but are server-bound.

bucket: **build-fresh** (logic), styled and renamed (e.g. "Fireworks") to stay clear of trademark.

transport: turn-based. Perfect fit.

risks and the wrinkle: Hanabi has an **inverted hidden-information** model. You can see others' hands but not your own. Over pure P2P this is actually easier than normal hidden-info, because each peer can be the authority for the hands it can see and simply never transmit a peer's own cards to them. No commit-reveal needed if you trust the dealer; a host-authoritative model (see netplayjs below) handles it cleanly. Flag for a future trustless version.

### 5. Go — BANGER (for the right audience)

what: the ancient territory game, on 9x9 for quick games or 19x19 for serious ones.

why it's a banger: bottomless depth, and a 9x9 game is short enough to fit a chat session. This is your "principled and serious" flagship for the subset of users who want it.

license: rules unprotectable. BUGOUT (an open-source play-Go-with-friends-over-web project) exists as prior art and reference. SGF (the standard Go record format) is a natural fit for your outcome lexicon record.

bucket: **build-fresh** for the board and rules; the hard part is scoring (territory counting and life/death), which you can defer by using manual stone-removal agreement at game end, exactly as casual Go servers do.

transport: turn-based. Perfect fit.

risks: scoring complexity is the only real cost. Start with 9x9 and agreement-based scoring.

### 6. Checkers / Draughts — KID-FRIENDLY

what: the diagonal-capture classic.

why it fits: universally known, simpler than chess, great for kids and casual adults. Rounds out the abstract-board set without overlapping Connect Four or Reversi.

license: rules unprotectable. Many MIT implementations exist.

bucket: **build-fresh**.

transport: turn-based. Perfect fit.

risks: none. Decide up front whether to enforce forced-capture rules; casual players often prefer them off.

### 7. Chess — completeness pick

what: chess, with two players and optional spectators.

why it's here and not higher: chess is table stakes, but it is *not* a great first-impression hook, because it signals "commitment" and intimidates casual users. Include it for credibility, not as the tutorial.

license: ArcaneCircle/chess is **GPL-3.0** and is a genuinely good 2-player-plus-observers webxdc implementation. Important asset caveat: its piece images are the Cburnett set under **CC-BY-SA 3.0**, which is a copyleft-on-art obligation. If you reuse the art you must attribute and share-alike; if you rebuild, pick CC0 or your own pieces.

bucket: **port** if you want the observer model fast (study the chess repo's turn-state-over-updates pattern), or **build-fresh** on a library like a Rust move-gen crate. The chess repo does *not* use the realtime channel; it syncs turn state through ordinary update messages, which is a good model for your Bluesky-record-per-move idea.

transport: turn-based. Perfect fit.

risks: GPL-3.0 on the reference code means a port inherits copyleft. The CC-BY-SA art is the subtler trap.

### 8. A daily word puzzle (Wordle-style) — KID-FRIENDLY-ish, async banger

what: a once-a-day guess-the-word puzzle, shared and compared with friends.

why it fits: the daily-puzzle-plus-share loop is *exactly* your hook thesis in miniature. You see a friend's score on a post, you tap, you play the same puzzle, your result posts back. It is the most native fit to the Bluesky-outcome-record idea of anything on this list.

license: ArcaneCircle/wonster ("a daily word puzzle webxdc game about befriending monsters") is **MIT** and is purpose-built for this. trosel/webxdc-daily-challenge-game-template is a template for exactly this genre.

bucket: **wrap** the wonster app, or **build-fresh** from the daily-challenge template. Either is light.

transport: effectively async. The "match" is two people playing the same seed independently, then comparing. No live transport needed at all, which makes it the lowest-onboarding-weight option here.

risks: none. Note this is *async-competitive*, not live-P2P, so it exercises a different (and easier) path than the rest. That is a feature for a launch set: it proves the social loop without needing a live connection at all.

---

## Kid-friendly subset (the values showcase)

These are safe by construction: friend-to-friend P2P only, no stranger chat, no ads, no accounts, no dark patterns. Quality, not just safety.

- **Connect Four** — the anchor. Simple, genuinely fun cross-generationally.

- **Checkers** — universally known, gentle skill curve.

- **Memory / concentration** — flip pairs of cards. stackwatermelon/memory-blocks-webxdc exists (Memory Blocks, as seen in Lumosity). Trivial to build fresh. Naturally co-op or solo-competitive, no fairness wrinkle.

- **Snake / Tron-style** (single-screen) — ArcaneCircle/snake is **GPL-3.0**. Build-fresh is easy. Wholesome, instantly legible.

- **A drawing game with a kid word-list** — the skribbl-style game above, shipped with a curated child-safe prompt set, is one of the best kid experiences you can offer.

- **2048 (co-op or score-compare)** — ArcaneCircle/2048 exists; rules trivial. Good solo-with-shared-score filler.

Design point for the whole subset: because these are friend-to-friend by construction, the "no chat with strangers" property is not a moderation feature you build, it is a consequence of the architecture. That is worth saying out loud in your marketing, because it is true and rare.

---

## Fuller candidate inventory by category

### Classic 2-player abstract / board

| game | repo | license | bucket | tempo |
|---|---|---|---|---|
| Connect Four | ArcaneCircle/c4 | MIT | build-fresh | turn |
| Reversi/Othello | ArcaneCircle/Othello | GPL-3.0 | build-fresh | turn |
| Chess | ArcaneCircle/chess | GPL-3.0 (art CC-BY-SA) | port or build | turn |
| Checkers | many | varies / unprotectable | build-fresh | turn |
| Go | BUGOUT (reference) | open | build-fresh | turn |
| Connect-4 variant | loic-fejoz/connect4-variant | unstated | reference | turn |

Backgammon and Nine Men's Morris are natural additions in the same build-fresh vein. Backgammon adds a dice-fairness wrinkle (see design wrinkles below).

### Word and party

- **Daily word puzzle**: ArcaneCircle/wonster (**MIT**), trosel daily-challenge template. Best-in-class fit. Async.

- **Drawing-and-guessing**: rebuild; skribbl.io is proprietary, open clones are server-bound. Slow-real-time.

- **Word/letter-tile games**: be careful here. Acbn-Nick/webxdc-scramble and kl3mousse/street-sweeper-13 exist but are unlicensed / self-described "AI-slop," and the obvious tile-game targets (Scrabble, Boggle, Codenames) are **trademarked with protected board/art**. Build generic versions with original naming and your own word lists. The *mechanics* are free; the *names and boards* are not.

### Card games (and the hidden-information question)

- **Hanabi** (co-op, inverted hidden-info): see banger entry. Hanabi-Live (logic reference), andrwmillr/hanabi (clone). Build-fresh + rename.

- **Hearts / Spades / general trick-takers**: classic, well-understood, but every one has the hidden-hand problem (see wrinkles). Build-fresh with a dealer model.

- **UNO-likes**: trademark trap on the name and card design. A generic "crazy eights" rebuild is clean and kid-friendly.

Hidden information over pure P2P is the recurring wrinkle. Covered below.

### Co-op (fits the ethic, dodges leaderboard attestation)

- **Hanabi** is the flagship.

- **Co-op drawing / shared-canvas** "make something together" toys (no winner) are pure, wholesome, and trivially fits a broadcast channel. ArcaneCircle has a draw-board precedent.

- **Co-op word puzzles** (solve the same daily together, shared hint budget) are a nice twist on the daily-puzzle hook.

Co-op is underweighted in most game catalogs and overweighted toward your values. Lean into it.

### The existing webxdc game catalog (inventory)

The serious maintainer is **ArcaneCircle** (the developer adbenitez, a Delta Chat contributor). Most apps are mirrored on GitHub; the canonical curated index is the `xdcget.ini` file in the **Codeberg** `webxdc/xdcget` repo, which feeds the store bots and webxdc.org/apps.

Worth surfacing (maintained, real):

- chess (GPL-3.0, 2 players + observers, actively released)

- c4 / Connect Four (MIT)

- Othello (GPL-3.0)

- tictactoe (Unlicense, with multi-game matchmaking and spectating)

- wonster daily word puzzle (MIT)

- 2048, snake, sudoku, bejeweled, crashdown, StackUp, breakout71, Dino, whack-a-ninja (mixed MIT / GPL-3.0 / BSD / AGPL)

Hobby-grade or skip:

- webxdc-scramble (unlicensed, self-described AI-slop)

- street-sweeper-13 (NOASSERTION license)

- various single-author one-offs with no license file

Useful infrastructure from the same ecosystem (not games, but you will want them):

- **webxdc/highscores** (MIT): a library for score-based webxdc games. Directly relevant to your leaderboard idea.

- **WofWca/webxdc-yjs-provider** (now on Codeberg): syncs app state via Yjs CRDT over the webxdc transport. Relevant if you want conflict-free shared state instead of hand-rolled sync.

- **webxdc-download-polyfill**: fixes file-export/download links inside a webview. You will likely need an equivalent.

### Broader WebRTC browser multiplayer (the porting recipe opens this up)

- **netplayjs** (rameshvarun/netplayjs, ~557 stars, TypeScript): the most important find in this category. A prototyping framework that gives you **rollback netcode + WebRTC** with a host-authoritative state model, and it abstracts the netcode from the transport. It uses a matchmaking server purely for signaling, which is the exact swap-point your recipe targets: replace its signaling/transport with your iroh channel and you inherit rollback for free. License needs confirming on the repo (the org publishes under permissive terms; verify before bundling). The host-authoritative state correction it does is also a ready-made **dealer model** for hidden-info games.

- **boardgame.io** (boardgameio/boardgame.io, **MIT**): a turn-based game engine where you write moves as pure state-transition functions and it handles multiplayer via a pluggable transport. Structurally ideal to swap iroh into, and it ships debugging/replay/bot tooling. The tension: it is **JS/React**, which fights your Rust-core / Leptos plan. Use it as a *logic and architecture reference*, or as a fast-path prototype, rather than the production substrate, unless you are willing to run a React panel for the games pond specifically.

- **internaut/p2p-bomberman** (custom BSD-3): a real WebRTC-over-PeerJS 4-player Bomberman. Good study specimen for the recipe; dated, and Bomberman tempo is twitchy.

- **The Longest Yard / ioquake3 / HumbleNet** (the Quake lineage): **GPL-2.0** engine. This is the anti-pattern hook (multi-hundred-MB data file, twitch tempo) and the copyleft is load-bearing. Keep it as a ceiling demo only, exactly as you already concluded.

---

## Cross-cutting design wrinkles

### 1. Hidden information over pure P2P

Pure peer-to-peer has no neutral server to hold secret state, so any game with hidden cards or hidden roles needs one of:

- **Trusted dealer**: one peer (the challenger, or netplayjs's host) holds the deck and only sends each player what they may see. Simple, fast, good enough for friend-to-friend play where nobody is incentivized to cheat. This is the right default for a *friends* product.

- **Commit-reveal cryptography**: each peer commits to a shuffled deck via hashes, then reveals, so neither can cheat without detection. This is mental-poker territory. Correct for a trustless future version, overkill for launch.

Mapping to candidates:

- Hanabi: easiest case. The secret is *your own* hand, which everyone else can see. Each peer is the authority for hands it can see and simply withholds a peer's own cards from them. Trusted-dealer is invisible here.

- Trick-takers (Hearts, Spades): standard hidden hands. Trusted-dealer at launch, commit-reveal later.

- Backgammon / any dice game: not hidden info, but **dice fairness** is the same class of problem. A commit-reveal coin-flip (both peers hash a seed, reveal, XOR) gives provably fair dice cheaply. Worth doing even at launch because it is small.

- Connect Four, Reversi, Chess, Checkers, Go: **perfect information**. No wrinkle at all. Both peers see the whole board, so a divergent state is immediately detectable. This is another reason to launch on the abstract-board set: it is the cryptographically boring, robust core.

### 2. Tempo / transport fit (summary)

- Everything in the recommended launch set is **turn-based or slow-real-time**, which is deliberate. All of it fits relayed iroh comfortably.

- The daily-word-puzzle is **async**, needing no live connection, the lightest path of all.

- Reserve **twitch-real-time** (the Quake lineage, Bomberman, platform fighters) for "look what's possible," never for onboarding.

### 3. Wrap vs port vs build-fresh (the meta-call)

For this pond specifically, **build-fresh wins more often than the recipe suggests**, for three reasons.

First, the abstract classics are genuinely trivial to implement, and a clean Rust-core implementation matches your architecture better than grafting a transport swap onto someone's JS.

Second, building fresh sidesteps both license inheritance (several reference repos are GPL-3.0) and asset-copyleft traps (the chess pieces are CC-BY-SA).

Third, your outcome-record-on-Bluesky and leaderboard logic wants to be *native* to your core, not bolted onto a foreign game loop.

**Port** when the value is in nontrivial, correct game logic you do not want to rewrite (a strong chess move generator) or in a netcode framework (netplayjs's rollback).

**Wrap** for the genuinely self-contained, already-good webxdc apps where the only glue needed is your transport and your outcome hook (the daily word puzzle is the cleanest wrap).

**Build-fresh** for everything where the rules are simpler than the integration: every perfect-information board game, the drawing game, the memory game.

---

## Bottom line

Launch on the **perfect-information abstract set plus the daily word puzzle plus a drawing game**: Connect Four, Reversi, Checkers, the daily puzzle, and a skribbl-style canvas, with Hanabi and Go as the "we have depth too" follow-ons and chess for credibility.

That set is fun, instantly legible, cryptographically robust, license-clean if you build fresh, and zero-extraction by construction. It demonstrates the composable model by being it, which is the whole point of the hook.

The one thing to confirm before any bundling: re-check live license files on netplayjs and any specific webxdc repo you actually pull, because licenses and repo state move, and a few of the reference repos carry GPL-3.0 or unstated licenses.

---

## Source and access notes

Most webxdc apps live on **Codeberg**, which was not reachable from this session's sandbox network, so the canonical `xdcget.ini` index was read indirectly via search and the GitHub mirrors rather than cloned directly. If you want a fully authoritative inventory, pull `webxdc/xdcget` from Codeberg yourself and read `xdcget.ini`. The GitHub `ArcaneCircle/*` mirrors and licenses were confirmed via the GitHub API and repo pages. The realtime-channel mechanics and the WebRTC-polyfill caveat were confirmed against the Delta Chat docs, the `peer_channels` Rust source, and the webxdc-x-WebRTC forum thread.
