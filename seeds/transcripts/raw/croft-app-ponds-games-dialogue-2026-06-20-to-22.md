# Croft app — ponds & pads / games deep-dive dialogue (2026-06-20 to 2026-06-22)

> **Provenance / fidelity note (PLAYBOOK §4).** This is a **cleaned reproduction of a chat the user
> pasted in** (a claude.ai session), not a byte-pristine export — same status as the companion
> `croft-app-design-dialogue-2026-06-20-to-22.md`. No canonical export was available, so this is the
> best-available raw: UI **render chrome** removed (tool-activity lines — "Searched the web",
> "Fetched: …", "Ran a command…", "Cloned…"; artifact-card footers "Document · MD"; "Show more";
> date/time dividers; "Presented N files"), dialogue wording preserved as faithfully as the rendered
> paste allowed. **Content-faithful, not byte-exact.** Volatile facts are the model's
> session-time claims; the **verified** ones were checked in-session against live sources (iroh-docs
> model, app licenses via GitHub mirror, on-device-LLM platform APIs, deferred-deep-link platform
> state) and carry their verdicts in the artifacts.
>
> **The eight artifacts this dialogue produced are frozen verbatim** at `seeds/apps-unpacked/`
> (p2p-games-pond-launch-set, webxdc-security-and-competitive-games, games-pond-authoritative-list,
> apps-pond-utility-list, build-shape-pass, on-device-llm-feasibility, build-order,
> fair-reveal-primitive-spec). Working copies + an index live at `thinking/app/ponds/`. This
> transcript is the reasoning behind them. It is the **run** of the
> `seeds/generated-prompts/games-pond-research-prompt.md` and a continuation of the app-design
> dialogue.

---

## Topic

A deep dive into what fills the Croft "garden": the **games pond** and **utility/ritual pads**, how
to build them on the iroh substrate, the webxdc security model, fair-outcome cryptography, the
ponds/pads UX (complexity management + deep-linking), an optional on-device-LLM navigator, and the
non-extraction/cooperative economic frame that makes it sustainable. Output: 8 reference artifacts.

---

## Dialogue

**USER:** [continuing from the app-design dialogue; pasted the games-pond research prompt] Run the
games hunt — find existing P2P/webxdc games and port/wrap/build-fresh candidates for a P2P social
games pond; at least 3 adult "bangers," a strong kid-friendly subset; transport-fit and license per
candidate.

**ASSISTANT:** [Verified the ecosystem against live sources rather than memory.] Findings reshaped
the recommendation beyond a flat list. **Transport-recipe correction:** the webxdc
`joinRealtimeChannel()` API is a **broadcast/gossip** channel, not point-to-point, and is
**ephemeral** (late joiners get nothing); the Quake III port works by hard-coding signaling
responses by server/client role on top of that broadcast layer (a Delta Chat core dev, flub, called
it "a little bit odd"). Since you control your own iroh integration, you can expose **direct QUIC
streams per peer pair** alongside a broadcast channel — making you a *better* WebRTC port target than
webxdc itself and removing the late-joiner problem for 1v1. **Build-fresh wins more than the recipe
implies:** perfect-information classics are simpler to write in your Rust core than to graft a
transport swap onto someone's GPL-3.0 JS, and building fresh dodges two real license traps confirmed
in-session (several ArcaneCircle reference repos are GPL-3.0; the chess repo's piece art is
CC-BY-SA 3.0 — copyleft on the *art*). **Launch set leads with cryptographically boring games on
purpose:** Connect Four, Reversi, Checkers, Go, chess are perfect-information, so divergent state is
self-detecting and there's no hidden-info wrinkle. The daily word puzzle (ArcaneCircle/wonster, MIT)
is the cleanest fit to the Bluesky-outcome-record hook (async, seedable, share-then-play-the-same-
seed). Hanabi is the standout co-op banger and sidesteps leaderboard attestation entirely.
*Inclusion is three pathways: build-fresh (abstract classics), wrap (webxdc-native), port
(raw-WebRTC).* Open items flagged: confirm netplayjs and GGRS/matchbox licenses; most webxdc apps
live on Codeberg (blocked from the sandbox) so the canonical xdcget index was read via GitHub
mirrors. → produced **p2p-games-pond-launch-set.md**.

**USER:** Leaderboard is solvable let's not sweat it too much. And talk me through the webxdc security
model and look for more competitive esp 2 player to 4 player games.

**ASSISTANT:** **The webxdc security model** is defense-in-depth around one promise: the app is a
static zip in a network-isolated webview with **zero internet access**, and the only IO is handing
messages to the host messenger, which relays them E2E-encrypted through an audited Rust core. Layers:
bundled-only packaging (manifest, no CDN), blocked external links and fetch/XHR, CSP + webview
config, per-app storage isolation, and the tiny send/receive/realtime API as the sole channel. **The
instructive part (a Cure53 audit, OpenTechFund-funded):** **CSP alone does not contain a webview** —
`RTCPeerConnection` ignores CSP entirely and could exfiltrate data, plus a DNS-prefetch leak where
the official disable knob didn't work; five high-severity findings incl. two full CSP bypasses. Their
WebRTC fix is the blunt "FILL500" hack (instantiate 500 peer connections to hit Chromium's per-process
cap). **The clean irony for you:** since iroh QUIC is your transport you don't need the webview's
WebRTC at all, so disabling it is pure upside and closes the exact hole that cost Delta Chat months —
the one security action item genuinely yours (you wrap your own Tauri webview). Two design points
fall out: keep the games pond and social panels as **hard-separate webview contexts** (a compromised
game can't reach DID keys/session tokens), and hand games an **ephemeral per-match pseudonym** by
default (a game that reads a stable identity can correlate players across sessions). **Competitive
2-4p widening, by tempo:** turn-based (Connect Four, Reversi, Checkers, Chess, Go, Backgammon + new
Dominoes, Dots and Boxes, Ultimate Tic-Tac-Toe — perfect-info ones are self-detecting, only card/tile
need a dealer); slow-real-time (drawing-and-guessing, bingo, word races — jitter-tolerant); twitch
(standout: **Curve Fever / Achtung die Kurve**, two-button, 2-8 players, Curvytron is MIT; + Pong,
Bomberman, platform fighters). Enablers: **netplayjs** (rollback + WebRTC, swap signaling for iroh)
and, more architecture-aligned, **GGRS + matchbox** (rollback in Rust). → produced
**webxdc-security-and-competitive-games.md**.

**USER:** One of the cornerstones here is since P2P has no server load and this is a user-first
non-extractive app, we can feed user attention for good and collab and fun. No one is looking to make
a buck off of this — ever. What does that enable that can't be easily matched in terms of experience?
The one exception is a public or private leaderboard and we can post to Bluesky for it for free at a
minimum.

**ASSISTANT:** The question isn't "what features does P2P enable," it's "what experiences are
structurally impossible for anyone carrying server cost or an extraction mandate." A moat that comes
from *not having things*: **No latency tax on generosity** — every server-backed app meters
generosity against COGS (free-tier ceilings, retention windows, "upgrade to keep your history"); you
have no per-user marginal cost, so the always-first-to-be-capped things (unlimited history, unlimited
groups, no "3 free X this month") are free, permanently, with no asterisk — and the asterisk-free-ness
*is* the moat, because a competitor can match any one as a loss-leader but not all of them forever
against their P&L. **The app can be genuinely done** — software that costs nothing to run can stop
changing; finished-and-stable is nearly extinct because it generates no recurring revenue, and you can
promise it. **Attention has no adversary** — every funded app has a part of the org whose job is to
capture more attention than freely given; you don't have the gradient, so the pro-attention moves
(natural stopping points, "go to bed," games that end) that are career-limiting elsewhere are just
fine here. **Privacy true by construction** — not "we respect it," but "we couldn't violate it; the
data never reaches us and there is no us to subpoena" — a physics, not a policy; the competitor's own
infrastructure (logs, warehouse, an ads SDK someone added years ago) betrays them. **Trust compounds
instead of eroding** — no monetization phase is coming, so no enshittification arc. The honest
counter: server-backed apps win on anything needing a neutral third party (stranger matchmaking,
server-witness anti-cheat, async-offline play, scale spectating, trusted-dealer hidden-info) — so the
moat is specifically the friend-to-friend, present-together, nothing-at-stake-but-fun zone, and the
composable "different ponds" model keeps you from sprawling into the zones where a server wins. The
leaderboard exception is the thesis working: reach for atproto's already-public commons (a custom
lexicon record) instead of building private infrastructure. *The experiences you can't match aren't
features — they're promises with no expiry date.* One open question flagged: the sustainability model
that isn't extraction (donations / foundation / tiny co-op dues / your time as gift) is what turns "no
one makes a buck" from a vulnerability into a credible forever-promise.

**USER:** Ok give me our best authoritative games list by inclusion criteria (port etc) and ranked by
fun.

**ASSISTANT:** [Synthesized the consolidated, ranked list.] Inclusion is three pathways, not a flat
tag, and **wrap is nearly free at scale**: since you already plan to borrow the webxdc realtime-channel
API shape, a webxdc-compatible shim over your iroh transport makes the *entire* ArcaneCircle catalog
wrappable for the cost of one compatibility layer (the catch: you inherit mixed licenses/assets, so
hand-pick). Fun ranking — **Tier S** (keeps a chat alive): drawing-and-guessing, Curve Fever, Four in
a Row (the cold-open hook, highest fun-per-second-of-onboarding). **Tier A:** Reversi, Hanabi (the
co-op showcase), Dots and Boxes (the underrated pick), daily word puzzle (the Bluesky loop in
miniature). **Tier B:** Backgammon, Go, Dominoes, Checkers, Ultimate Tic-Tac-Toe. **Tier C**
(credibility/kid-filler/demos): Chess (a masterpiece but signals commitment and intimidates a cold
audience — credibility, not the tutorial), memory, bingo, pong, snake, 2048, Bomberman, platform
fighter. **Build-only-five cut:** Four in a Row, drawing-and-guessing, daily word puzzle, Reversi,
Curve Fever (spans all three pathways, all three tempos, 2-player through whole-group). Open: confirm
netplayjs + GGRS/matchbox licenses. → produced **games-pond-authoritative-list.md**.

**USER:** I frame this as: durable software needs maintenance, like a car, A/C, health or a
relationship, and that's fair but substantially different from extractive software which needs
attention and likely ever-increasing, which leads to dark patterns and enshittification. And yes a
premise here is that care providers deserve to be valued and that value is material but the economics
can be cooperative.

**ASSISTANT:** The load-bearing move is separating two kinds of ongoing cost that get deliberately
conflated. **Maintenance** is cost paid to keep a thing *at rest* — tracks entropy and a changing
environment, bounded by the world not by appetite (the car doesn't need more maintenance because you
want it to); roughly flat, occasionally spiky, indexed to things outside anyone's commercial interest.
**Attention-capture** is cost paid to keep a thing *escalating* — it conflates itself with maintenance
to look legitimate ("we have to keep shipping") but the target is a rising engagement number that
can't sit still because the model compounds, so the work is intrinsically ever-increasing, and once
the honest levers are exhausted the only ones left are dark. **Enshittification isn't a villain turn —
it's the terminal state of a cost structure indexed to appetite rather than entropy; it arrives on
schedule.** On valuing care: "labor of love freely given" is the framing that *looks* principled and
then collapses into burnout (slow-motion abandonment), so valuing the maintainer materially is a
*requirement* of keeping the forever-promise, not a compromise of the ethic. The cooperative makes it
work because **the funder and the beneficiary are the same body, so the incentive to escalate never
forms** (member dues vs investor growth-pressure — Ostrom: commons upkeep is solvable when those who
depend on the resource govern and fund it, unsolvable when an extractive outsider captures
governance). And the architecture reinforces it: P2P/local-first means costs genuinely don't balloon
with success, so the economics and the architecture make the same forever-promise from two directions.
Honest caveats: the maintenance curve isn't guaranteed flat (support/security/platform load grows with
scale — but a co-op's dues scale with the base generating the load, where a free-no-model project
quietly overloads the one maintainer until it dies). The one-line charter version: *durable software
needs care like anything you keep, bounded by entropy; extractive software needs attention, unbounded
by appetite, which is why it ends in dark patterns. We choose the first, pay maintainers materially
because unpaid care is deferred abandonment, and make it sustainable cooperatively because a commons
governed and funded by the people who depend on it has no one inside whose interest is escalation.*

**USER:** Ok now that we've got some good P2P-backed open games thinking let's do a pass with the same
basic thinking for apps outside of games. High utility, even fun, in a P2P sense in smallish groups.
What's useful or even just cool we could bundle? We are looking for "have you seen this?" energy.

**ASSISTANT:** [Grounded against the actual webxdc non-game catalog + iroh primitives.] The key
unlock: **iroh 1.0 ships three production protocols that are effectively a local-first backend you
don't have to write** — `iroh-blobs` (content-addressed, BLAKE3-verified file transfer with resume;
`sendme` is the ready demo), `iroh-docs` (eventually-consistent key-value store with built-in sync),
`iroh-gossip` (pub/sub tuned for mobile churn). So for utilities the smart default is **build-fresh on
top of these** (the storage-and-sync layer, normally the expensive part, is done). The selection
filter is not raw utility but one of three reactions: **"wait, that needed an account and a server?"**
/ **"it's alive"** / **"this is ours only."** If none, it's just a worse cloud app — out. **Tier S:**
split-the-check (Splitwise minus the account — the killer utility), find-a-time (when2meet minus the
link), device-to-device file/photo drop (sendme in your app), shared whiteboard (the "it's alive"
anchor). **Tier A:** shared photo album, collaborative checklist, fair decision tools
(ranked-choice/dot-voting), collaborative notes, watch-together sync. **Tier B/C:** shared map, retro
board, planning poker, recipe box, private group feed, micro-toys. Two server-creep constraints: map
tiles (use embedded/offline tiles) and watch-together (sync the **clock and control events**, not the
media bytes — the file must already be local on each device). The decision/voting tool doubles as
**cooperative-governance infrastructure** for Croft. → produced **apps-pond-utility-list.md**.

**USER:** Yes to both [verify licenses + iroh-docs model], and a local real-time voting app is
actually fascinating too. One thing I worry about here is complexity to the user. I think ponds or
groups of apps by utility and the ability to pin an app from a pond at a top layer and even to link
to the app or share the app in a way that jumps the user to the one thing expected.

**ASSISTANT:** [Verified licenses via the GitHub mirror + the iroh-docs data model.] **Licenses:**
editor = MIT; checklist / corkboard / poll = Unlicense; one early example MPL-2.0 — all permissive, no
copyleft, no attribution-art trap (confirm each LICENSE at bundle time; Codeberg was unreachable to
clone). **Split-the-check schema** hinges on two verified iroh-docs facts: the namespace secret is
*shared* write-capability (not per-user), and conflict resolution is **last-write-wins per
(author, key)**. So never store a running balance as one mutable key (the last writer clobbers it) —
use **event-sourcing**: one document per group, an append-only log of immutable expense/void/settle
events each authored and keyed by a ULID (keys never collide), the roster as per-member owned keys,
balances folded client-side in integer minor units; corrections are appends, not edits (auditable, and
it fits the cooperative-transparency ethic). Flagged unknown: iroh-docs exposes a single namespace
write token, so revoking a departed member means rotating to a new document (fine for a dinner group;
into the threat model). **Real-time voting is the standout** — the rare utility where *watching it
happen* is the product. Two modes mapping onto iroh's two layers: open live-tally on gossip (the
theatrical one — bars moving, a late vote flipping the lead) and secret-ballot via **commit-reveal**
(hash, lock, reveal, verify → a provably fair secret ballot with no server). Crucially it's the **same
fair-reveal primitive** you need for dice and the fair randomizer — build once, reuse. **On
complexity (the central product risk):** don't present apps, present **ponds as rooms with activities
you do inside them.** Three layers each with one job: ponds (the only thing most people navigate),
activities grouped by **verb not app identity** (Decide / Track / Make / Play / Share — twenty apps
become five legible verbs), and a user-**pinned** top layer for the two-or-three things they use
(pinning is the pressure valve: deep catalog, simple default surface). The deep-link you described is
the same mechanism as the Bluesky game hook: **build one resolver + one catalog manifest** serving
games, utilities, social identically, shareable at three depths (whole pond / an activity / a single
instance like "this specific vote"). → produced **build-shape-pass.md**.

**USER:** I like this with a "thinking of you" type notice — I've seen bracelets that flash a light as
a thinking-of-you, could do it direct and real time for free perpetually. Countdown / shared timer /
ready-check energy: small but "it's alive" on a micro scale. And Rave exists for shared watch and has
lots of appetite issues — we'd want to defer it and keep it relatively private and small scale. And a
digital guestbook is a fascinating use case — could intertwine photos and other things. Also invites
with basic meetup reminders and details.

**ASSISTANT:** These cluster around **ritual and presence** (vs utility — the bill-splitter is a tool
you reach for; these make a small group *feel* like a group over time), worth their own pond. **The
thinking-of-you ping** is the purest expression of the thesis: the bracelets (Bond Touch) built a
business, an account, and a cloud relay around what is **fifty bytes** over the wire — you deliver the
identical felt experience as a near-empty gossip packet, free and undiscontinuable. Keep it
**non-verbal and pressure-free** (no read receipts; a small fixed vocabulary, not free text — free
text turns it back into messaging); real-time-first (lands now or not, like a knock — more intimate
and simpler), persisted-pings a later toggle. **The guestbook is connective tissue** — the one app
that *accumulates*, which is how a group acquires shared memory; naturally multimodal (notes, photos
via blobs, a drawing, a saved game outcome), so the present-tense apps deposit durable traces into it.
**The question-of-the-day** is the engine that keeps it alive **without engagement-bait — the critical
design choice is the absence of a streak** (an invitation, never a Skinner box; that one decision is
the whole ethic in miniature). Append-only with redaction, not true delete (void-don't-mutate).
**Invites with local reminders** point outward to gathering and close the loop (find-a-time settles
*when*, the invite reminds, the gathering fills the guestbook); honest caveats — reminders fire as
**local notifications per device** (no push server; verify mobile background limits), store **absolute
time** (zones), and a rendered map reintroduces tiles. **On Rave/deferral** — codify why as a reusable
filter: Rave is watch-together *plus the appetite curve* (discovery surface, stranger rooms, the
moderation burden strangers create, scale infra, the engagement metrics that justify it) — none of
which is inherent to watching with a friend; it's inherent to **scale + strangers**. So keep co-
experience apps **small-group and friends-only** and the appetite problems don't form. Group these as
a **"Presence & Ritual" pond** (a Feel/Together verb): the ping, guestbook, invites, daily prompt,
live reactions — shared substrate (gossip for live/ephemeral, docs+blobs for accreting), shared
no-coercion ethic. One axis to decide per-app: **persist vs evaporate** (ping evaporates; guestbook
accretes; invite is between).

**USER:** How feasible is this at an app and ecosystem layer? And agreed [deep-link resolver as one
mechanism]. What else can we think of for this pond? And a "Would You Rather" for a group would be
fun. And maybe travel games or apps.

**ASSISTANT:** **Feasibility — app layer:** feasible and mostly verified, not hoped-for (iroh 1.0 real
with the three protocols + Swift/Kotlin bindings; webxdc proves the sandbox-app pattern in production;
Tauri-wraps-Rust-core is well-trodden; the apps are small once the substrate exists). The honest
app-layer risk isn't any single app, it's the **integration surface** (iroh + the webxdc shim + the
deep-link resolver + per-pond context boundaries + the Tauri webview cohering into something a
non-technical person finds simple) — each piece feasible, the seam-work is where the months go, so
**sequence ruthlessly. Ecosystem layer (harder):** the **network-effect bootstrap** is brutal for a
friends-only P2P tool (worthless until your people are on it; no public-square discovery by design) —
so the **deep-link is your entire acquisition model**, which makes the resolver the single
most-strategically-important component (tier-zero, build first). **Relay infrastructure is a quiet
asterisk on "serverless"** — iroh hole-punches ~90-95%, the rest falls back to relays (n0's by
default), free at small scale but a third-party dependency you'd self-host if it grew — name it in the
charter honestly. The **webxdc-shim bet** carries ecosystem risk (the API is experimental, the
ecosystem is a few grant-funded people) — so build-fresh what you want to guarantee, wrap what's
nice-to-have. And the **sustainability answer** must be sketched before people trust the guestbook with
sentiment, because lost sentiment is real harm. **More for the pond** (mostly reusing voting or
commit-reveal, so cheap): Would You Rather (open-tally voting in a party hat), This-or-That/hot-takes,
Two Truths and a Lie (commit-reveal), rose-thorn-bud check-in (the most meaningful), gratitude
exchange, countdown-to-event, ambient status-glance, collaborative playlist (share **references**, not
audio). **Travel** is the strong recombination angle — not a new app but a **"Trip" pond preset**
(pre-pinned: split-check, photo album, find-a-time, invite/itinerary, map-of-pins, guestbook) that
lands the **offline** story hard (a group abroad coordinating P2P over local Wi-Fi, no roaming-
dependent cloud — iroh connects same-network devices directly). Plus **travel/car games** (20
Questions, geography chain, I-spy, road-trip bingo) — build-fresh, turn-based, kid-friendly; the car
game's form factor *is* your design brief. Caution: keep travel to the preset + classic games; reach
for live maps/bookings and you've rebuilt a server-dependent product.

**USER:** Ok feasible I meant deep linking and surfacing app ponds and apps based on intake channel
and deep link, making it so users get right to where they expect but still can orient from there.

**ASSISTANT:** Feasibility splits by **who resolves the link**, because the intake channel determines
how much you know at landing. **Case 1 (has app, in pond):** fully feasible — internal route over
state you hold. **Case 2 (has app, not in pond):** feasible — the link carries a **join capability**
(the iroh-docs/gossip ticket), and the decision is what it authorizes and for how long (a link is a
**bearer token** — single-use/short-lived/approval-gated for anything sensitive). **Case 3 (no
app):** feasible to make *good*, impossible to make *seamless* — you cannot auto-install from a link
on modern mobile, so the path is link → landing page → store install → app restores context, and that
last clause (**deferred deep linking**) is the hard, failure-prone, growth-critical piece. The
commercial answer (Branch/AppsFlyer/Adjust) works by **fingerprinting**, exactly the tracking you
refuse — so the privacy-respecting path is platform-native deferred links where they work + a
**one-more-tap claim-code** fallback ("we don't fingerprint you across an install, so paste your
invite once" — a feature, not an apology). **"Land exactly there but orientable"** is a solved UI
pattern: land in the specific thing, framed by thin persistent chrome (pond name, sibling activities,
"here's the rest") — the deep-link **sets the depth** but all three nav layers stay reachable; it's a
*starting position* in a space, not a tunnel. **Channel-adaptive surfacing** is feasible and a genuine
differentiator if kept to ~3 classes (direct / public / proximity), each setting two things: how much
orientation chrome and **what capability the link carries** (a public Bluesky-origin link grants only
"request to join," a direct-share link a real ticket — the channel gates capability, a UX+security
alignment). Don't over-engineer beyond a few classes.

**USER:** Yes [check App Clips / Instant Apps]. And a sidebar — Android, iOS, OSX and even Chrome and
Firefox have a low-power local LLM now; could we leverage that as ecosystem navigation and orient?
"Hi, welcome to Croft, you're jumping into a game. This app has a small local-first assistant here to
help." The goal is not to have to navigate menus if you don't want — semantic app navigation, most
bang for buck. Users can say "any travel games?" and get internally usable links and help. I know
it's a bit cliché and often done poorly but in this case I think it might make the difference if it
stays out of the way and can be very easily disabled entirely.

**ASSISTANT:** **Deferred-link verified state (not great for Case 3):** Android **Instant Apps ended
Dec 2025**; **Firebase Dynamic Links shut down Aug 2025**; **App Clips survive (iOS-only)** for
proximity (QR/NFC). iOS treats install→app context as a privacy boundary *on purpose* — Universal
Links open warm if installed but offer zero deferred utility; bridging requires fingerprinting (the
MMP path you refuse). So **the seamless private Case 3 is not achievable in 2026** — warm opens are
solid (Universal/App Links), App Clips help proximity, and the **claim-code one-more-tap is the
principled answer**, framed as refusing tracking. **On the on-device LLM — not a sidebar, possibly the
answer to the complexity problem:** most assistants are bad because they're a worse interface to a
solved problem, bolted on, sending data to a server. Yours inverts every part: the real problem is
**semantic discovery over a growing composable catalog** (genuinely hard for menus, ideal for an LLM:
fuzzy intent in, structured pointers out); it's the **only assistant whose privacy story matches the
product** (on-device is the only values-coherent option; a cloud one would be the single place the
no-extraction promise leaks); and it **resolves the deep-link/nav tension** — it turns natural language
into an **internal deep link** ("any travel games?" → catalog map → the same pond/app/instance links
your resolver already handles), so it's a **language front-end to the resolver you're building
anyway**, adding no new plumbing and degrading gracefully (off → menus still work). Scope it narrow
(intent → ranked catalog links + a one-line orientation; **constrain output to real catalog links so
it can't hallucinate an app**); peer to the menus never a gate; one-tap permanent disable; absent by
default where hardware can't do it. The welcome register you sketched is right *because* it's modest.

**USER:** Yes dig deeper across Android, iOS, OSX and even Chrome and Firefox here to understand
what's deeply feasible now.

**ASSISTANT:** [Verified per-platform on-device LLM state.] **Apple — excellent, nearly turnkey:** the
**Foundation Models framework** (iOS/iPadOS/macOS 26) exposes the ~3B on-device model in a few lines,
free/offline/no-key, scoped by Apple to exactly your task; the standout is **guided generation**
(`@Generable` + `.anyOf([...])`) which **constrains output to your real catalog at the token level so
it cannot hallucinate** — solving the biggest risk, in the API, for free; floor ~iPhone 15 Pro / 16+,
M-series. **Android — works on flagships, steep cliff:** Gemini Nano via AICore + ML Kit GenAI, strong
privacy isolation, but **structured output is weaker** (alpha Prompt API, coerce-by-prompt), inference
foreground-only/battery-quota'd, and the device gate is brutal (newest Nano needs ~12GB RAM →
roughly Pixel 10 / Galaxy S26 / a few others ≈ 1-3% of active Android; the **fallback is the majority
path**). **Chrome — desktop-only, flag-gated bonus** (Prompt API runs Gemini Nano via WASM/WebGPU;
**not on Chrome for Android or iOS**; Stable ~late-2026/early-2027; ~22GB free disk; no browser
indicator that prompts stay local). **Firefox / mobile-web — assume nothing** (no Prompt-API
equivalent; universal fallback is a bundled WASM model — Qwen-0.5/1.5B, Llama-3.2-1B via wllama/WebLLM
— at a few-hundred-MB-to-~GB download + WebGPU). **The architecture this forces is the one you wanted:
detect-and-degrade optional accelerant over a navigation system complete without it**, three engine
tiers (platform model → bundled WASM → no-assistant) behind one interface, every output validated
against the real catalog. → produced **on-device-llm-feasibility.md**.

**USER:** iOS is not in our first build list so no, Android and OSX are.

**ASSISTANT:** That splits your best and weakest cases across your two targets. **macOS** carries the
same Foundation Models framework incl. `@Generable` guided generation (the good part is fully
available) — but the gate is **M-series Apple-silicon only** (Intel Macs get the no-assistant path),
and since your core is Rust-in-Tauri you need a **Swift-to-Rust bridge** to call it (small, particular
to your non-Swift stack, scope it early). **Android is now your hard floor** and the weaker target: the
device cliff means **the common case is the fallback case** (most early Android users hit weaker Nano
or no model), structured output must be **prompt-coerced + hard-validated** (no `@Generable`
guarantee), foreground-only. So **let Android's constraints set the assistant's floor**: build the
plain navigation to be **complete and excellent on its own** (on Android it's the only nav for many),
make catalog-constraint validation **strict and engine-agnostic** (native on macOS, post-validated on
Android), and **decide consciously** whether the conversational layer on no-model Android devices is
worth a bundled-model download or whether those users are well-served by menus alone.

**USER:** No let's pause on it, it's a great-to-have, but can it be a requirement? Let's go back to the
ecosystem of local and peer apps from before, give me the overall breakdown of that research and
analysis.

**ASSISTANT:** Right — the coverage research confirms the assistant **can't be a requirement** (you'd
gate a core experience on hardware a large share of users lack). Park it. [Delivered the consolidated
breakdown — the thesis; the iroh-1.0 substrate (gossip/docs/blobs) + the relay asterisk; the three
inclusion pathways; the games pond ranked by fun; the utilities pond by "have you seen this?" energy;
the presence-and-ritual pond as the project's heart; the cross-cutting wrinkles (hidden-info →
dealer-or-commit-reveal; tempo → transport fit; persist-vs-evaporate; voting promoted toward flagship);
the recurring event-sourced data model; the ponds/verbs/pins UX + deep-link-as-growth + the
deferred-link reality; the maintenance-vs-attention economic frame; and the two make-or-break items:
the deep-link resolver (tier-zero) and the sustainability model.]

**USER:** Both pls [the consolidated build-order doc + the fair-reveal primitive spec].

**ASSISTANT:** [Wrote both as artifacts.] **build-order.md** — six phases on three rules (growth path
and core UX are the same component so it's first; prove one pond end-to-end before going wide; sequence
by leverage not excitement): **Phase 0** (foundation, nothing user-facing — iroh integration layer,
deep-link resolver + catalog manifest, the pond context boundary with WebRTC disabled, the shared
event-sourced store ledger+guestbook reuse); **Phase 1** (first pond end-to-end — Four in a Row as
cold-open hook + reference loop, the fair-reveal primitive, a second game exercising it, the verb/pin
UI shell; gate: a non-technical person lands from a link into a ready game, plays P2P, records the
result, orients, all without the assistant); **Phase 2** (killer utility — split-the-check reusing the
Phase 0 store, local real-time voting reusing fair-reveal); **Phase 3** (the heart — thinking-of-you
ping, guestbook + question-of-the-day, invites, cheap ritual games); **Phase 4** (breadth, now cheap —
the webxdc shim = a dozen games for one layer, remaining classics, port-tier real-time, remaining
utilities, pond presets); **Phase 5** (great-to-have, explicitly last/optional — the on-device
assistant as a front-end to the resolver, can slip indefinitely). Two parallel gates: the resolver is
tier-zero; the sustainability model must be sketched before Phase 3 (the guestbook creates a custodial
obligation the moment a family trusts it with memory). **fair-reveal-primitive-spec.md** — the one
shared module giving fairness with no server: standard **commit-reveal** (publish
`H(value ‖ nonce)` — binding-but-hiding — then reveal `(value, nonce)`, everyone verifies + computes
locally), closing both peeking and changing. Three things flagged hard: the **nonce is load-bearing
for low-entropy values** like votes (else an attacker hashes all possible votes); the **abort case**
(commit-then-vanish) is the real hazard, handled by a **reveal deadline** (relative timeout, not
wall-clock, for skew) not crypto; it maps onto gossip (live rounds) + docs/Bluesky-record (durable
outcome) and reuses iroh's per-entry author signatures for attribution. Boundaries stated honestly:
handles **discrete hidden-then-revealed moments** (votes, dice, Two Truths and a Lie), **not**
continuous hidden state (an ongoing dealt deck → trusted-dealer for friends, or mental-poker for
trustless); doesn't solve collusion/sybil (that's pond membership, upstream). The leverage: building it
in Phase 1 makes voting + several later games "call the primitive and compute." Both docs flag
build-time items to confirm (platform CSPRNG, BLAKE3 encoding, mobile background-notification limits).

**USER:** give me all artifacts produced in this chat.

**ASSISTANT:** [Listed the eight: p2p-games-pond-launch-set, webxdc-security-and-competitive-games,
games-pond-authoritative-list, apps-pond-utility-list, build-shape-pass, on-device-llm-feasibility,
build-order, fair-reveal-primitive-spec.] Noted that a fair amount of thinking stayed in conversation
rather than becoming a file — the moat analysis (what non-extraction structurally enables), the
durable-maintenance-vs-extractive-attention framing + cooperative economics, the Presence & Ritual
pond design, and the channel-adaptive deep-link breakdown. (Those are preserved in this transcript.)

---

## End of dialogue

The eight artifacts are the durable output (frozen at `seeds/apps-unpacked/`, working at
`thinking/app/ponds/`). The conversational-only threads — the **moat from not-having-things**, the
**maintenance-vs-attention + cooperative** economic frame, the **Presence & Ritual pond**, and the
**channel-adaptive deep-link** feasibility — are preserved here as the reasoning record, and the
load-bearing ones are carried into `thinking/open-considerations.md` and `ROADMAP_TODO.md`.
