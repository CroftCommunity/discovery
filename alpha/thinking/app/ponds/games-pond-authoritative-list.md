# Games Pond: Authoritative List

author: research session

scope: the definitive launch candidate list, organized by inclusion pathway, ranked by fun

date: 2026-06-21

ranking axis: fun in *this* context, which means friend-to-friend, present-together, instant-start, group-chat-alive fun. Not abstract game depth. A drawing game outranks chess here on purpose.

---

## The three inclusion pathways (this determines effort, not fun)

There are exactly three ways a game enters the pond, and which one applies depends on what the source game already speaks.

**Build-fresh.** Write it in your Rust core. Correct for any game where the rules are simpler than an integration would be, where you want a clean license, and where you want the outcome-record-to-Bluesky loop to be native rather than bolted on. Covers nearly every abstract classic and the drawing game. Trademark note: build-fresh lets you sidestep name traps. Use "Four in a Row" not Connect Four, "Reversi" not Othello, generic names for anything Hasbro or Mattel owns. The mechanics are free; the names and boards are not.

**Wrap (via a webxdc-API shim).** This is the highest-leverage move on the whole project. You already intend to borrow the webxdc realtime-channel API shape. If you implement a webxdc-compatible API surface (`sendUpdate`, `setUpdateListener`, `joinRealtimeChannel`) backed by your iroh transport, then the *entire* ArcaneCircle catalog becomes wrappable for the cost of one compatibility layer, not one effort per game. A dozen working games fall out of a single shim. The catch: you inherit each app's license (the catalog is a mix of MIT, GPL-3.0, BSD, AGPL) and its assets (the chess set is CC-BY-SA 3.0), and quality varies, so you still hand-pick.

**Port (via WebRTC-to-iroh transport swap).** For non-webxdc games that speak raw WebRTC. You polyfill the browser WebRTC API onto a direct iroh QUIC stream and mock the signaling handshake, exactly the Quake-port recipe, except you have direct point-to-point channels available so you are a *better* host than webxdc's broadcast-only layer. This is the path for the real-time arena games and the netcode-framework games.

The decision rule: webxdc-native game then wrap, raw-WebRTC game then port, everything else build-fresh.

---

## The ranked list

### Tier S: keeps a group chat alive

**1. Drawing-and-guessing (skribbl-style)**

fun: the single best group-chat-keeper in existence. Funny, social, forgiving of skill, scales from 3 to a whole family thread. The highest-ceiling social fun on the list.

inclusion: **build-fresh**. skribbl.io is proprietary and the open clones are server-bound. Stroke-sync and a curated word list is light work, and strokes fit a broadcast channel perfectly.

license: clean (rebuild). players: 3 to 8. tempo: slow-real-time, jitter-tolerant.

note: not your cold-open hook (needs 3+ people and a round to warm up). It is your retention engine.

**2. Curve Fever / Achtung die Kurve**

fun: chaotic real-time crowd fun, two-button controls, the screaming-at-your-phone-together energy. Famous precisely for being learnable in one sentence.

inclusion: **port** (Curvytron, MIT, as a fork base; development is halted so treat it as source, not a live dependency) or build-fresh (logic is just constant-speed turning plus trail collision; the netcode is the hard part).

license: MIT via Curvytron. players: 2 to 8. tempo: twitch-real-time, wants a direct hole-punched iroh connection, degrades on relay.

note: best fun-to-weight ratio in the twitch tier. The one real-time game worth launching rather than demoing.

**3. Four in a Row (Connect Four)**

fun: the perfect hook. Instant, cross-generational, genuinely fun, fast rounds, "best of five" energy. Graspable in one sentence by a five-year-old or a grandparent.

inclusion: **build-fresh** (ArcaneCircle/c4 is MIT if you prefer to wrap, but this is your "hello world" of the whole challenge-to-outcome loop, so build it).

license: clean (rebuild). players: 2. tempo: turn-based, perfect fit.

note: **this is your cold-open hook.** Highest fun-per-second-of-onboarding of anything here.

### Tier A: genuinely great, high replay

**4. Reversi (Othello)**

fun: swingy and deep but fast. The "I thought I was winning" full-board reversal is a real thrill. Adults who think they hate thinky games still enjoy it.

inclusion: **build-fresh** (ArcaneCircle/Othello exists but is GPL-3.0; rebuild for a clean license).

license: clean (rebuild). players: 2. tempo: turn-based, perfect fit.

**5. Hanabi (rename to "Fireworks")**

fun: the co-op banger. Beloved, tense, "we win together" instead of beating each other. Deeply replayable for the group that clicks with it.

inclusion: **build-fresh** (logic only; Hanabi-Live is a Go-plus-Postgres server, useful as a reference, not a port). Rename and restyle to stay clear of trademark.

license: clean (rebuild). players: 2 to 5. tempo: turn-based.

note: slightly higher mental load, so not a cold hook, but the values showcase of the set. Hidden-info is the *easy* inverted kind (you see everyone's hand but your own), so each peer is authority for the hands it can see and simply withholds a peer's own cards. No commit-reveal needed.

**6. Dots and Boxes**

fun: sneaky-deep. Looks like a kids' doodle, plays like a tactics puzzle once the chain-reaction endgame kicks in. Scales 2 to 4 cleanly. The most underrated pick on the list.

inclusion: **build-fresh** (trivial state, rules unprotectable).

license: clean (rebuild). players: 2 to 4. tempo: turn-based, perfect fit.

**7. Daily word puzzle (Wordle-style)**

fun: the share-and-compare loop. Not live fun, but enormous stickiness, and it *is* your hook thesis in miniature: see a friend's score on a post, tap, play the same seed, your result posts back.

inclusion: **wrap** (ArcaneCircle/wonster, MIT, is purpose-built and genuinely self-contained) or build-fresh from a daily-challenge template.

license: MIT. players: 1 vs 1 async (compare same seed). tempo: async, needs no live connection at all, the lightest path here.

note: the cleanest possible demonstration of the Bluesky-outcome-record idea, and the only candidate that proves the social loop with zero live networking.

### Tier B: great, but more niche or slower-burn

**8. Backgammon**

fun: fast, lucky-but-skillful, great for two people who play a lot. The doubling cube adds real tension.

inclusion: **build-fresh**. Carries a dice-fairness wrinkle: commit-reveal coin-flip (both peers hash a seed, reveal, XOR) gives provably fair dice cheaply. Small, worth doing.

license: clean (rebuild). players: 2. tempo: turn-based.

**9. Go (9x9 and 19x19)**

fun: bottomless, but for the right audience only. 9x9 is the accessible on-ramp and fits a chat session; 19x19 is the serious-player flagship.

inclusion: **build-fresh** (BUGOUT as reference). Defer territory scoring to manual stone-removal agreement, as casual servers do. SGF is a natural fit for the outcome record.

license: clean (rebuild). players: 2. tempo: turn-based.

**10. Dominoes**

fun: social, good 2-to-4-player coverage, a pleasant slower-burn.

inclusion: **build-fresh** (logic). The draw pile is hidden info, so it needs the trusted-dealer pattern.

license: clean (rebuild). players: 2 to 4. tempo: turn-based.

**11. Checkers / Draughts**

fun: universally known, gentle skill curve, great for kids and casual adults. Lower ceiling than the games above it.

inclusion: **build-fresh**. Decide forced-capture up front.

license: clean (rebuild). players: 2. tempo: turn-based, perfect fit.

**12. Ultimate Tic-Tac-Toe**

fun: surprisingly strategic, a great quick-play filler, unlike plain tic-tac-toe which is solved and dull.

inclusion: **build-fresh** (trivial).

license: clean (rebuild). players: 2. tempo: turn-based.

### Tier C: completeness, kid filler, and "look what's possible"

**13. Chess**

fun: a masterpiece, but an intimidating cold hook that signals commitment. Include for credibility, not as the tutorial.

inclusion: **wrap** (ArcaneCircle/chess is a good 2-players-plus-observers implementation, but GPL-3.0 and its piece art is CC-BY-SA 3.0) or build-fresh on a Rust move-gen crate.

license: GPL-3.0 + CC-BY-SA art if wrapped; clean if built. players: 2 + spectators. tempo: turn-based.

**14. Memory / concentration**

fun: wholesome kid filler, no fairness wrinkle.

inclusion: **build-fresh** (trivial) or wrap an existing webxdc memory game.

license: clean. players: 2 to 4. tempo: turn-based.

**15. Bingo / number-draw races**

fun: kid-and-grandparent friendly, light, communal.

inclusion: **build-fresh**. The draw RNG needs a fair-draw mechanism (commit-reveal or designated dealer), same class as dice.

license: clean. players: 2 to many. tempo: slow-real-time.

**16. Pong / air hockey**

fun: trivial twitch fun, fine quick demo. One-dimensional action makes it relatively latency-forgiving.

inclusion: **build-fresh** on a netcode lib. players: 2. tempo: twitch-real-time.

**17. Snake (single-screen competitive)**

fun: legible, light, mildly competitive.

inclusion: **wrap** (ArcaneCircle/snake, GPL-3.0) or build-fresh. players: 2. tempo: slow-to-twitch.

**18. 2048 (score-compare)**

fun: solo-with-shared-score filler.

inclusion: **wrap** (ArcaneCircle/2048). players: 1 vs 1 async. tempo: async.

**19. Bomberman-style**

fun: real fun, but twitchy and the available codebase is dated.

inclusion: **port** (internaut/p2p-bomberman, custom BSD-3, a good study specimen for the recipe). players: 2 to 4. tempo: twitch-real-time. Demo-tier.

**20. Platform fighter / physics brawler**

fun: high ceiling, but the most tempo-sensitive thing on the list.

inclusion: **port** (netplayjs ships fighter/physics demos). players: 2 to 4. tempo: twitch-real-time. Demo only.

---

## The netcode enablers (for the twitch tier)

These are not games, they are what make the port-tier feasible.

**netplayjs** (rameshvarun, ~557 stars, TypeScript): rollback netcode plus WebRTC, host-authoritative state correction. Swap its signaling for your iroh channel and inherit rollback. The host-authoritative model doubles as a dealer for hidden-info games. Confirm the repo license before bundling.

**GGRS + matchbox** (Rust): pure-Rust GGPO-style rollback (GGRS) plus P2P signaling (matchbox). The more architecture-aligned path for an all-Rust core, since the netcode lives where your game logic already wants to be. Verify current crates and licenses.

**boardgame.io** (MIT): a turn-based engine with a pluggable transport, ideal to swap iroh into and shipped with replay/debug tooling. The tension is that it is JS/React, which fights your Rust-core plan, so use it as a logic-and-architecture reference rather than the production substrate.

---

## If you build only five

The minimum set that proves the thesis and is genuinely fun:

1. **Four in a Row** (build-fresh) - the cold-open hook and your reference implementation of the whole loop.

2. **Drawing-and-guessing** (build-fresh) - the retention engine.

3. **Daily word puzzle** (wrap wonster) - the cleanest Bluesky-outcome-record demo, zero live networking.

4. **Reversi** (build-fresh) - depth without intimidation, the "one more game" pick.

5. **Curve Fever** (port Curvytron) - proves real-time P2P works and is the crowd-pleaser.

That set spans all three inclusion pathways, all three tempos, perfect-information and hidden-info and async, and 2-player through whole-group. It is fun, license-clean, and demonstrates the composable private model by being it.

---

## Verification notes

ArcaneCircle licenses (c4 MIT, Othello GPL-3.0, chess GPL-3.0 with CC-BY-SA art, wonster MIT, snake GPL-3.0, tictactoe Unlicense) confirmed via the GitHub API and repo pages. Curvytron MIT and the Achtung-clone licenses confirmed via osgameclones. netplayjs and GGRS/matchbox licenses still need a live check against their repos before bundling, since those are the load-bearing netcode dependencies. Codeberg was unreachable from the research sandbox, so any Codeberg-hosted variant should be re-confirmed directly. Trademark caution on names (Connect Four, Othello, Scrabble, UNO, Codenames and similar) stands regardless of code license: build-fresh with generic names.
