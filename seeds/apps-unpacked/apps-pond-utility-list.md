# Apps Pond: Useful and Cool P2P Things to Bundle

author: research session

scope: the same treatment as the games list, applied to non-game apps. High-utility or just-cool things that work P2P in small groups, with "have you seen this?" energy.

date: 2026-06-21

ranking axis: not raw utility. The energy. A thing ranks high when the no-server-no-account property creates *visible* delight or removes *visible* friction. The wow is the disappearance of infrastructure you assumed was required.

---

## What creates the energy (the selection filter)

A utility earns a spot when it triggers one of three reactions:

**"Wait, that needed an account and a server, and now it just... doesn't?"** Expense splitting, scheduling, file transfer. Things people currently sign up for, here happening in a chat with nobody's cloud involved.

**"It's alive."** Real-time co-presence: a shared cursor moving, a whiteboard filling in as someone draws, a live reaction landing. P2P realtime makes a small group feel *present* in a way async tools don't.

**"This is mine and only ours."** A shared album, a family recipe box, a private group feed. The delight is intimacy by construction: the data is on your devices, not in a company's warehouse, and that's felt, not just claimed.

If a utility triggers none of these, it's just a worse version of a cloud app and doesn't belong in the bundle.

---

## The transport advantage you have here (and it's bigger than for games)

For games, your edge over webxdc was direct point-to-point channels. For utilities the edge is larger, because **iroh ships three production-ready protocols that are basically a local-first backend you don't have to write:**

**iroh-blobs**: content-addressed, BLAKE3-verified file/blob transfer with resumable downloads. This is a finished file-sync engine. Anything involving "move a file or a photo between people" is mostly already built.

**iroh-docs**: an eventually-consistent key-value store with a built-in sync protocol. This is shared mutable state across peers without a central database, which is the hard part of every collaborative utility. Lists, boards, trackers, notes all sit naturally on this.

**iroh-gossip**: pub/sub over the swarm, tuned for mobile churn. This is your presence/broadcast layer for the "it's alive" category.

So the build calculus shifts. For games you mostly build-fresh. For utilities, the smart default is **build-fresh on top of iroh-docs / iroh-blobs**, because the storage-and-sync substrate is the part that's normally expensive and it's already done. The CRDT-versus-last-write-wins decision is the main design choice, and the webxdc reference apps already demonstrate both paths (Automerge for conflict-heavy, plain CRUD for simple).

The same three inclusion pathways still apply: **build-fresh** (on iroh primitives), **wrap** (a webxdc-native app via your API shim), **port** (a raw-WebRTC or local-first web app via transport swap).

---

## The ranked list

### Tier S: maximum "have you seen this?"

**1. Split the check / shared expenses (Splitwise, minus the account)**

energy: the strongest "wait, no account?" reaction available. Splitwise is the canonical "we all begrudgingly signed up for this" app. Doing it in a friend group's pond, with running balances synced peer-to-peer and nobody's financial data on a server, is genuinely startling. Trip groups, roommates, dinners.

inclusion: **build-fresh** on iroh-docs (the running ledger is exactly a synced key-value state). Hidden-info is a non-issue; everyone sees the shared ledger by design.

players: 2 to ~10. tempo: async, perfect fit. wrinkle: none. This is pure upside.

note: this is the killer utility. If the games hook gets people in, this is the one that makes them keep the app installed.

**2. Find-a-time / group scheduling (when2meet, minus the link and the account)**

energy: "you didn't make me create a poll on a website?" Everyone fills a shared availability grid, the overlap lights up live. The single most universally-needed group-coordination tool, with the most friction normally attached.

inclusion: **build-fresh** on iroh-docs, or wrap a webxdc poll/scheduling app via the shim. The poll primitive already exists in the webxdc catalog as a starting point.

players: 2 to many. tempo: async with live-overlap-update. wrinkle: none.

**3. Secure file / photo drop, device-to-device (sendme, but in your app)**

energy: "it's transferring directly and it's that fast?" No upload, no WeTransfer link, no size limit, no cloud. iroh's own flagship demo is exactly this, so it's essentially free to you, and it saturates multi-gigabit links with resumable, integrity-checked transfers.

inclusion: **build-fresh** on iroh-blobs (this is what iroh-blobs *is*). sendme is the reference implementation and is interoperable.

players: 2 (or fan-out to a small group). tempo: real-time transfer, relay-tolerant. wrinkle: none, and it doubles as a "send to my own other device" utility.

note: nearly zero build cost because it's the substrate's home turf. Include it early just to demonstrate the "no cloud, still works" thesis in the most literal way.

**4. Shared whiteboard / co-drawing (the "it's alive" anchor)**

energy: watching a friend's strokes appear in real time over a private P2P channel is the most visceral "it's alive" moment in the whole bundle. Doubles as brainstorming, doodling, teaching, kids' fun.

inclusion: **build-fresh** on iroh-gossip for live strokes (the webxdc "draw" app is a reference), or wrap it. Strokes are broadcast-shaped, so even the simpler transport handles it.

players: 2 to ~8. tempo: slow-real-time, jitter-tolerant. wrinkle: none.

### Tier A: high utility, real delight

**5. Shared photo album (no cloud upload, ever)**

energy: "our trip photos, ours, not in anyone's cloud." A pooled album where everyone contributes and the originals live on participants' devices, synced peer-to-peer. The privacy property is felt strongly for personal photos specifically.

inclusion: **build-fresh** on iroh-blobs (content-addressed photos) plus iroh-docs (the album index). Real but not large work given the substrate.

players: 2 to ~15. tempo: async. wrinkle: storage asymmetry (who keeps the originals); worth a design note on replication.

**6. Collaborative list / checklist (packing, shopping, todo)**

energy: lower wow, very high keep-rate. The everyday tool a group actually reaches for weekly. The delight is that it just stays in sync with no app, no account.

inclusion: **wrap** (the webxdc "checklist" app uses Automerge CRDT and is a clean reference) or build-fresh on iroh-docs. Both conflict-resolution strategies are already demonstrated in the catalog (Automerge vs the plain-CRUD "Corkboard").

players: 2 to many. tempo: async. wrinkle: concurrent edits, solved by CRDT.

**7. Decision tools: ranked-choice / dot-voting / consensus**

energy: "we just decided fairly in ten seconds." Polls are table stakes; the interesting version is *fair* group decision: ranked-choice for picking a restaurant or a movie, dot-voting for prioritizing, simple consensus check. Resonates hard with the cooperative-governance ethic, and is a natural feeder into your commons-governance thinking.

inclusion: **build-fresh** on iroh-docs. The webxdc poll app is the trivial-case starting point.

players: 2 to many. tempo: async, with a fair-tally wrinkle (a tamper-evident tally or commit-reveal for secret ballots, same crypto pattern as game dice).

note: this is the one category that doubles as infrastructure for Croft / cooperative governance, not just a group toy.

**8. Collaborative notes / shared doc (real-time co-editing)**

energy: "a shared doc with no Google account." Live multi-cursor editing of a note or outline, peer-to-peer.

inclusion: **wrap** (the webxdc "editor" uses Yjs for collaborative WYSIWYG, and WofWca's webxdc-yjs-provider syncs Yjs over the transport) or build-fresh. Yjs is the mature CRDT for text specifically.

players: 2 to ~8. tempo: real-time, jitter-tolerant. wrinkle: text-merge, solved by Yjs.

**9. Watch-together / listen-together sync (playback controller)**

energy: "it's synced across both our screens." A shared transport bar: play, pause, seek, all peers in lockstep, for a video or audio everyone already has locally. The co-experience feeling is strong.

inclusion: **build-fresh** on iroh-gossip (you sync *control events and a clock*, not the media itself, which keeps it light and legal). Port candidates exist in the watch-party space but most assume a server.

players: 2 to ~6. tempo: real-time control sync, relay-tolerant. wrinkle: the media must be locally present on each device (you sync timing, not bytes); flag this clearly or pair it with the file-drop utility.

### Tier B: useful or cool, more niche

**10. Shared map with pins**

energy: "drop a pin for where we're meeting / our trip spots," collaboratively, privately. Pleasant and practical for planning.

inclusion: **build-fresh** on iroh-docs. Map *tiles* are the catch: offline/embedded tiles avoid a network dependency, otherwise a tile server creeps the server back in. Worth a design note.

players: 2 to many. tempo: async. wrinkle: map-tile sourcing.

**11. Group retro / brainstorm board (mad-sad-glad, affinity mapping)**

energy: "we ran a real retro with no SaaS." Sticky notes on a shared board, group them, vote. Workplace-adjacent but also genuinely useful for any planning group.

inclusion: **build-fresh** on iroh-docs (sticky notes are CRUD state; the "Corkboard" reference app is literally this pattern). Pairs with the dot-voting tool.

players: 2 to ~12. tempo: async-to-slow-real-time. wrinkle: none.

**12. Planning poker / estimation**

energy: "simultaneous reveal, no spreadsheet." Everyone picks a card, all reveal at once. Small, satisfying, and the simultaneous-reveal mechanic is a nice showcase of fair commit-reveal.

inclusion: **build-fresh** on iroh-gossip plus a commit-reveal step for the simultaneous reveal. players: 2 to ~10. tempo: near-real-time. wrinkle: the reveal fairness is the whole point, and it's the same crypto you'd build for game dice.

**13. Shared recipe box / family wiki / bookmark collection**

energy: "our stuff, curated by us, living nowhere else." A small shared knowledge base for a household or friend group.

inclusion: **build-fresh** on iroh-docs, or wrap the webxdc memos/notes apps. players: 2 to ~15. tempo: async. wrinkle: none.

**14. Private group feed / scrapbook (a tiny social pond)**

energy: "a social feed that's just us, no algorithm, no ads, no reach." A chronological shared wall for a family or friend group.

inclusion: **wrap** (ArcaneCircle/pixelsocial is exactly this, a serverless pixelated social network for a group chat) or build-fresh. players: 2 to ~30. tempo: async. wrinkle: none, and it's a natural bridge to your ATProto/ActivityPub social-panel thinking.

**15. Countdown / shared timer / ready-check**

energy: small but "it's alive" on a micro scale, everyone sees the same clock tick, or a synchronized go-signal. Useful for coordinating anything timed (workouts, study sprints, "everyone hit start").

inclusion: **build-fresh** on iroh-gossip. players: 2 to many. tempo: real-time. wrinkle: clock-skew handling.

### Tier C: micro-utilities and clever toys

**16. Fair randomizer / decider** (who pays, who goes first, draw straws). Build-fresh with commit-reveal so nobody can rig it. The fairness *is* the cool part. Tiny.

**17. Cross-device clipboard / quick-share** (push a link or snippet to your own other device). Build-fresh on iroh-blobs or a tiny doc. High personal utility.

**18. Shared countdown calendar / "this day" memory** (a group advent-style or anniversary tracker). Build-fresh on iroh-docs.

**19. Guestbook / question-of-the-day** (a daily prompt the group answers, lightweight async ritual). Build-fresh. Pairs with the daily-word-puzzle hook energetically.

**20. Live reactions / applause** (ephemeral emoji rain everyone sees during a shared moment). Build-fresh on iroh-gossip. Pure "it's alive" garnish, near-zero cost, makes other apps feel social.

---

## The webxdc non-game catalog (wrap-ready references)

These exist today and are the fastest path via your API shim, plus they're good architecture references even if you build fresh. Licenses are mixed across the catalog (MIT / GPL-3.0 / Unlicense), confirm per repo before bundling, and Codeberg was unreachable from the sandbox so re-check the Codeberg-hosted ones directly.

- **poll** (one question, up to 5 answers) - the decision-tool starting point.

- **checklist** (Automerge CRDT) and **Corkboard** (plain CRUD) - the two canonical local-first patterns, side by side.

- **editor** (Yjs collaborative WYSIWYG) plus **webxdc-yjs-provider** - real-time co-editing.

- **draw** (shared drawing board) - the whiteboard reference.

- **memos** (shared notes in a group) - notes/recipe/wiki starting point.

- **filesync / filesend** (split-and-send files) - though iroh-blobs is the better substrate for you.

- **pixelsocial** (serverless group social network) - the private-feed reference.

- **calc** (Ironcalc spreadsheet as webxdc) - a surprisingly capable collaborative spreadsheet, if a "shared sheet" ever fits.

- **timetracking** - a time tracker, niche but real.

Helper libs worth knowing: **highscores** (MIT, score persistence, ties to your leaderboard), and the **webxdc-yjs-provider** and Automerge integrations for CRDT state.

---

## If you bundle only five (utilities)

The set that proves the thesis outside games and maximizes "have you seen this?":

1. **Split the check** (build-fresh / iroh-docs) - the killer utility, the keep-installed reason.

2. **Find-a-time** (build-fresh / iroh-docs) - the most universally needed coordination tool.

3. **Secure file/photo drop** (build-fresh / iroh-blobs) - nearly free, the most literal proof of "no cloud, still works."

4. **Shared whiteboard** (build-fresh / iroh-gossip) - the "it's alive" anchor.

5. **Ranked-choice decision** (build-fresh / iroh-docs) - useful, fair, and doubles as cooperative-governance infrastructure for Croft.

That set spans all three iroh primitives (docs, blobs, gossip), all three "have you seen this?" triggers (no-account, it's-alive, ours-only), and async through real-time. None has a serious fairness or hidden-info wrinkle except the decision tool, whose fairness mechanism is the same commit-reveal you'd build anyway.

---

## The through-line

The games hook gets people in the door by being fun. The utilities are what make them *stay*, because a group that splits its dinner bills and shares its trip photos and schedules its plans in your app has moved its actual coordination there, with no account, no server, no data extraction, and no reason for any of that to ever change. That's the cooperative-maintenance promise made concrete: software that does real work, owned by the people who use it, with no one inside it whose interest is escalation.

### Verification notes

iroh 1.0 shipped June 15 2026 with iroh-blobs, iroh-gossip, and iroh-docs as production protocols, and official Rust/Python/Node/Swift/Kotlin bindings; sendme and dumbpipe are the canonical reference tools (confirmed via iroh.computer and launch coverage). The webxdc non-game apps (checklist/Automerge, Corkboard, editor/Yjs, poll, draw, memos, pixelsocial, calc/Ironcalc) were confirmed via the webxdc GitHub org and Codeberg listings; licenses vary and several live only on Codeberg, so confirm each directly before bundling. The CRDT building blocks (Automerge, Yjs, webxdc-yjs-provider) are real and current. Map-tile sourcing and watch-together media-locality are the two flagged design constraints that can quietly reintroduce a server if handled carelessly.
