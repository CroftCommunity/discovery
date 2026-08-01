# Graze forum — subjective-consensus karma + Reddit-parity mechanics (Gemini dialogue)

date: 2026-07 (pasted 2026-07-31)

source: Gemini design dialogue (single conversation), pasted into the session

status: **preserved-condensed (cleaned-paste, content-faithful — PLAYBOOK §4).** No canonical export
exists. UI render chrome stripped (the italic action lines like "Evaluated naming variants…", citation
badges "GitHub"/"apple"/"PTKD + 4", "Show more", and the "Gemini is AI and can make mistakes" footer).
User turns reproduced close to verbatim; Gemini turns reproduced faithfully to substance (the long
third-party-app feature-research turns are condensed; the design-mechanics and formulas are preserved).

fact-check discipline: this is a **Gemini** dialogue → per the source-of-truth
`atproto-atmospheric-web-iroh-mobile-FACTCHECK.md`, treat all atproto/iroh facts as **dialogue-sourced,
`[UNVERIFIED]`** until aligned. Notable: Jetstream/firehose "Merkle search trees" here is in the
**atproto** repo context (atproto repos are MSTs served in CAR files) — atproto-correct, distinct from
the FACTCHECK's iroh-docs correction (iroh-docs = range-based set reconciliation + LWW, not MST). The
named atproto job-board projects (atwork.place, Sifa, place.atwork.listing lexicon, H3 geo-indexing) are
**dialogue-sourced, pending verification** — see ECOSYSTEM.

covers: job boards on atproto; ungameable karma without a trusted PDS (AppView aggregation, custom
downvote lexicon, Sybil resistance); **graph-weighted subjective consensus** (the social-tree thesis
made concrete); subjective Hot/Rising/Controversial; decentralized labeler moderation; graph-amplified
awards / zero-cut tipping; community forking; offline archives; local-first via Jetstream (no mandatory
AppView, two equal modes); "better than Reddit" feature set; third-party-app feature research
(RES/Apollo/RIF/Slide/Infinity/RedReader/Geddit); titleless-post handling; Pixel Pals PWA feasibility.

---

### User

Are there job boards running on atproto now?

### Gemini `[dialogue-sourced, UNVERIFIED]`

Yes. Custom Lexicons let job listings and professional profiles live on the network. **atwork.place** is
a decentralized job board built natively on the protocol: listings use a custom lexicon record
(`place.atwork.listing`), stored in the company/user's PDS and tied to their DID; profile record
extensions let users set a `#hiring`/`#forhire` status singleton. It recently began integration with
**Sifa** (a professional-identity/workspace project). Protocol advantages highlighted: data ownership
(listings/resumes owned by the creator's DID, readable by any AppView); privacy-friendly location via
**H3 hexagonal geo-indexing** (proximity queries without exposing exact addresses). Younger than
LinkedIn, but functional and expanding.

### User

I am making a reddit like forum with a bluesky backend, all user content is from their own PDS so how
do we do karma or reputation points since it's all user gameable? Count replies to their posts? We
could still do likes bc those are from other users. I am not entirely sure how the karma system works
on reddit. Help me think through it.

### Gemini

The core challenge of decentralized reputation: if users control their PDS they can manipulate any
self-reported metric, so you cannot trust what a user's PDS says about their reputation — you must
calculate it from authenticated interactions by *other* users.

*How Reddit karma works:* essentially upvotes − downvotes, with (a) **diminishing returns** (a viral
post's karma is heavily sublinear — 50k net upvotes might yield ~7k karma), (b) **vote fuzzing**
(randomized displayed counts to defeat bots), (c) **Sybil detection** (centralized voting-ring
nullification).

*Architecting it on atproto* — rely on your Indexer (AppView), custom lexicons, and Sybil resistance:
1. **AppView aggregation is the source of truth.** The PDS just hosts content + outgoing interactions
   (the likes *given*). Your AppView consumes the firehose and tallies valid vote records from other
   DIDs pointing at the user's post URIs. If a user edits their PDS to claim a million karma, the
   AppView ignores it.
2. **Custom lexicon for voting.** Bluesky natively supports likes (`app.bsky.feed.like`). For distinct
   up/down arrows you'd define e.g. `com.graze.feed.vote` (voter's PDS record: target URI + direction
   ±1).
3. **Sybil resistance** (the hard part — anyone can spin up DIDs): **follow-graph weighting** (count
   votes only from accounts in a healthy social graph — e.g. ≥ N followers or followed by trusted
   accounts), **account age/activity cool-down** (new DIDs don't affect scores until proven),
   **atproto labelers** (ignore votes from DIDs flagged by trusted labelers).
4. **Replies ≠ reputation.** A heavily-replied post may be controversial/wrong ("getting ratio'd").
   Explicit cryptographic upvotes from external DIDs measure approval more accurately than reply count.

### User

We could also weight karma more from tenured users maybe. I am going to piggy back on likes for
upvotes and store down votes as a custom lexicon. [Sybil resistance] is good and the graph or tree as a
weighting component fits in with my thesis.

### Gemini

Piggybacking native `app.bsky.feed.like` for upvotes + a custom downvote lexicon (e.g.
`com.graze.feed.downvote`) is elegant: **interoperability** (upvotes read as ordinary likes across the
network, so Graze posts gain traction on standard Bluesky) + **containment** (downvotes stay local to
Graze, not injected as negative interactions into clients that can't contextualize them).

Graph-weighted reputation is the gold standard for decentralized Sybil resistance: it aligns with
node-based trust algorithms (**EigenTrust / PageRank**) where a vote's power derives from the voter's
existing reputation. A bot farm forms an isolated cluster with no inbound follows from the tenured
"trunk," so the AppView can mathematically reduce its combined weight to near zero. **Tenure
multipliers** (by DID age / first-valid-interaction date) make gaming temporally expensive.

### User

Maybe even weighting activity of their followers and the diversity of their interactions — expensive to
do if you get too fancy, and anything is gameable, but one of the tenets of my work here is anchoring
the online experience in our own social tree, so the closer someone is to me the more I would want
their weight to matter, or at least be the starting point.

### Gemini (the shift to subjective consensus)

What you're describing is a shift from **Global Consensus** (Reddit) to **Subjective Consensus**. If
karma is weighted by how close the voter is to the viewer, a post's score is no longer a static number
— what you see as highly-upvoted may look different to someone else because your social trees don't
overlap. Very resilient to manipulation, but naive real-time graph traversal per post/vote/user will
melt the database. How to make it tractable:

1. **Cache degrees of separation** (constrain depth, cache periodically): Depth 0 = self; **Depth 1**
   (inner ring, people you follow) → highest multiplier (e.g. 1.5–2×); **Depth 2** (their follows) →
   standard 1×; **Depth 3+** (the wild west) → fractional (0.1×) or filtered by global Sybil rules. A
   background cron updates these caches for active users rather than computing at read-time.
2. **"Diversity of interactions" cheaply** — proxies rather than true disjoint-subgraph math:
   **PDS distribution** (are upvotes spread across 50 PDSes or 90% from one self-hosted PDS? penalize
   localized clusters), **age distribution** (mix of old + new accounts vs a swarm minted last
   Tuesday), **cross-labeler validation** (voters subscribing to wildly different labelers implies an
   organic crowd, not a coordinated ring).
3. **Two-score UI** — because subjective scores can confuse users comparing notes: show a composite —
   **Network Score** (baseline global reputation with Sybil resistance) + **Tree Score** (how the
   viewer's own graph voted). Grounding the heaviest weight in Depth 1/2 neutralizes a bot farm's
   ability to touch the front page: 100k upvotes from accounts nobody in your tree follows get filtered
   before they reach your experience.

### User

Can you think on other mechanisms reddit uses and how to translate them to a subjective consensus and
atproto reality?

### Gemini

1. **Sorting algorithms.** Replace raw global variables with graph-weighted ones.
   - *Subjective "Hot":* Reddit's Hot ≈ `H = log10(max(1, |U−D|)) + T/45000` (T in seconds, 12-hour-ish
     periods). In Graze, `U`/`D` become **graph-weighted counts** — three Depth-1 upvotes can rocket a
     low-global-velocity post to the top of *your* Hot.
   - *Subjective "Rising":* Reddit's Rising ≈ Top over a short recent window. Graph-weighted → **inbound
     velocity within the tree** (a post suddenly accumulating Depth-2 upvotes, signaling an incoming
     wave before it reaches your direct follows).
   - *Subjective "Controversial":* Reddit ≈ `C = (U+D)^(min(U,D)/max(U,D))`. Subjectively it becomes
     **a stark split between your trusted graph and the global network** — your inner ring downvotes
     what the broader network upvotes → surfaced as contested-for-you.
2. **Moderation = decentralized labelers.** You can't delete a user's post from their PDS. A Graze
   community runs its own **Ozone labeler**; it flags a DID or post URI; the AppView drops it from the
   community feed UI (data stays on the PDS, loses its megaphone). **Subjective AutoMod / Crowd
   Control:** users subscribe to personalized labelers; if an account's reputation drops below the
   viewer's threshold (e.g. repeatedly downvoted by Depth-1), the AppView auto-collapses its comments —
   threshold dictated by the viewer, not an admin.
3. **Awards / Gold → graph-amplified highlights** via a `com.graze.feed.award` lexicon with friction
   (e.g. one award token per week of authentic participation). A random DID's award = small icon; a
   **Depth-1** award = massive visual highlight ("a trusted connection spent their scarce weekly token
   on this").
4. **Shadowbanning = the natural byproduct.** In strict subjective consensus, a bot farm has zero
   inbound follows from a normal user's trunk → its subjective weight is precisely zero. You don't
   build a ban system; they scream into a mathematically isolated void by default.

### User

I think we offer both modes so users can understand the difference actually. And can I use the default
labelers to filter bot and spam content already? And one of the ethos here is this is a PWA/SPA that can
operate without a custom AppView — this is local first, an AppView is a convenience and a helper but not
a necessity. You could do relational-tree subjective reddit with no AppView by using follows and
followers, or you could have the global view — two different but equal modes. And the global view even
works without [an AppView] on Jetstream.

### Gemini

Offering a **Global Consensus** / **Subjective Consensus** toggle demonstrates the protocol's power
(users see their graph filter noise vs the raw firehose). Executing the local-first vision on existing
infrastructure:

1. **Default labelers:** yes — Bluesky's default moderation ecosystem already ships global labels for
   inauthentic behavior (bot/spam label values). Configure your agent (`@atproto/api`) to subscribe to
   the default moderation service or send the `atproto-accept-labelers` header with established labeler
   DIDs; when your SPA pulls posts/votes it also receives labels and drops flagged data client-side
   before rendering or scoring.
2. **Local-first via Jetstream.** Direct firehose consumption is hard for a client (CBOR/CAR parsing,
   the repo Merkle search trees, volume). **Jetstream** sits between Relay and client and translates
   to a lightweight, filtered **JSON WebSocket** stream. Two modes in a pure client-side SPA:
   - **Mode A — Global (Jetstream-powered):** subscribe to your collections (`app.bsky.feed.post`,
     `app.bsky.feed.like`, `com.graze.feed.downvote`), tally in local state / IndexedDB → raw global
     karma with no backend aggregator.
   - **Mode B — Subjective (client-side graphing):** on login, query `app.bsky.graph.getFollows` to
     establish Depth 1; as votes stream via Jetstream, check the voter's DID against the local follow
     list and multiply the weight (or ignore beyond your tree). The "server" is the user's device
     processing the open web.

### User

Can you think of things we can do on this model that are better than reddit? Things that would delight
users that reddit refuses to or cannot because it's monetizing them?

### Gemini

1. **Stackable "choose your own" moderation (BYO-Mods).** Reddit moderation is feudal (a rogue power-mod
   or corporate can wipe a 10-year community). On Ozone, moderation is detached from hosting — subscribe
   to up to ~20 independent labelers simultaneously (a journalist "fact-check" labeler + a community
   "anti-toxicity" one). If a mod team goes rogue you unsubscribe and switch; posts stay intact on PDSes.
2. **Algorithmic sovereignty (the slider UI).** Reddit's Home/Popular are black boxes tuned for
   engagement. atproto supports custom feed generators → give users literal sliders (Depth-1 friends vs
   global; recency vs all-time; niche-topic focus). A tuned formula exports as a shareable custom feed.
3. **Zero-cut P2P tipping (anti-Reddit-Gold).** Reddit takes a large cut on cash-outs. DIDs are
   user-owned → attach **WebLN (Lightning), Solana wallets, or Stripe links** to profile metadata; a
   "Tip" button sends value directly, **Graze takes 0%**, changing the incentive structure for
   high-quality posts.
4. **Community forking & portability.** A Graze "field" (subreddit) is just an indexing tag on posts
   living on user PDSes → forking doesn't move data, just a new frontend filter / custom feed
   aggregating the same historical posts under new rules. Communities evolve without being destroyed by
   fragmentation.
5. **True offline archives & scraper-friendly design.** Local-first SPA/PWA + IndexedDB → "Sync
   f/retrocomputing" before a flight caches the full community incl. comment trees; read/draft/upvote
   offline; flush drafted interactions to the PDS on reconnect.

### User

This heavily mirrors my own thinking — Community Forking & Portability. And love [the tipping]. And yes
good one — [the offline caching].

### Gemini

How they map cleanly for the frontend: **frictionless forking** (`f/gardening` is a metadata tag on a
PDS record, not a server-locked destination — forking is a new frontend filter, old posts aren't
lost); **profile-metadata tipping** (`app.bsky.actor.profile` already handles links; scan for `lnurl` /
Solana / Stripe formats and render a native "Tip" button); **IndexedDB syncing** (Jetstream/API data is
already structured JSON → push a thread's state into IndexedDB via e.g. localForage in ms; a sync queue
holds offline upvotes/replies and flushes on reconnect).

### User

Can you research what Reddit Enhancement Suite and other apps that were around when the reddit API was
open had that users love and miss?

### Gemini (condensed to substance)

**RES (desktop power-user):** custom colored user tags + auto-highlight of OP/friends/mods;
comprehensive customizable keyboard navigation; a customizable subreddit-shortcut bar. **Apollo (iOS
gold standard):** GIF scrubbing; smart thread collapsing + auto-hide AutoMod stickies; account-age
highlighting; deep gesture customization; infinite-scroll toggle (anti-addiction). **Android (Sync /
Boost / RIF):** color-coded nested-comment lines; auto-hide read posts; dynamic layouts / masonry
gallery per subreddit; true offline subreddit caching. Because Graze is atproto + IndexedDB, offline
caching, custom tagging (local state), and per-community layout prefs are highly achievable and signal
"built for former third-party-app users."

### User

Those are great, can you do some research and give me a top 10 things reddit users miss or wish they had?

### Gemini (condensed to substance)

1. A functional, non-intrusive video player (the official TikTok-style player was slow/unreliable).
2. Cross-device state syncing (read posts, inboxes, filtered-subreddit lists).
3. Robust mobile moderation tools (modmail, mod queue).
4. True unfiltered feeds (the lost `r/all` vs algorithmic `r/popular`).
5. Smart image/gallery handling (swipe multi-image; share screenshots with sub+author credit).
6. Accessibility / screen-reader support (Apollo, Dystopia).
7. Customizable NSFW toggle (on at work, off at home).
8. App config backups / settings export (Boost).
9. Instant notifications + "watch sub" (Sync Pro; official push delayed 10–30 min).
10. A fast, snappy, ad-free interface (RIF/Apollo/Sync/Boost).

### User

Tell me more about Apollo and features that users loved and if the code is opensource.

### Gemini (condensed to substance)

**Apollo** (Christian Selig, iOS, launched Oct 2017, ~5M lifetime installs, ad-free; shut down
2023-06-30 after Reddit's API pricing — an estimated ~$20M/yr to keep running). Loved: customizable
gesture navigation; the Jump Bar (rapid subreddit switching); a powerful media viewer; color-coded
comment lines; smart toggles (auto-hide AutoMod stickies, highlight new accounts); deep per-subreddit
theming; Markdown editor; custom app icons; **Pixel Pals** (2022 — Tamagotchi-style pets on the Dynamic
Island). **Open source?** The iOS client is **not** open source (Swift/Obj-C proprietary). The **server
backend** (push notifications + subreddit watchers) *was* published to GitHub just before shutdown, to
prove Apollo respected rate limits and only made authenticated requests (rebutting Reddit admins'
scraping claims).

### User

What about RIF, same exercise.

### Gemini (condensed to substance)

**RIF ("reddit is fun" → "rif is fun for Reddit," Andrew Shu / TalkLittle, Android)** — an early,
utilitarian power-user client; shut down summer 2023 over API pricing (it had a 2012 trademark-license
revenue-share with Reddit, which Reddit terminated in 2016). Loved: information density (text-focused,
uncluttered); speed (hardware-accelerated); dual-pane tablet mode (early pioneer); true high-contrast
dark mode (pre-system dark mode); efficient navigation (left-edge-slide sidebar; quick hot/new/
controversial switching). **Open source?** Versions 1.0–1.3 were GPLv3 (archived on Shu's GitHub); the
2.0 (Mar 2012) rewrite was closed-source, reusing no GPL code — so the modern app millions used until
2023 was **not** open source.

### User

Were any of the popular 3rd party apps open source?

### Gemini (condensed to substance)

Yes: **Infinity for Reddit** (Android Java + iOS Swift/SwiftUI; ad-free; "Lazy mode" hands-free
auto-scroll; post-API subscription for logged-in API access but codebase stays open; anonymous mode
with local save/hide/read-tracking). **RedReader** (Android; free/open; advanced cache management,
two-column tablet mode, swipe actions, AMOLED "ultra black"; accessibility/screen-reader focus; GitHub +
F-Droid). **Geddit** (Android; open, read-only; bypasses the official API by pulling Reddit's public
JSON; GPL-3.0; Vue/CSS/JS). **Slide for Reddit (legacy)** (GPLv3; offline caching + deep color
customization; codebase public and heavily studied).

### User

What is slide for reddit, how was it unique?

### Gemini (condensed to substance)

**Slide** — open-source, ad-free, tracker-free (began as a high-school Android project on the Java
Reddit API Wrapper; later rewritten native Swift for iOS). Unique: **true offline** (autocaching
favorite subreddits); deep per-subreddit theming (thousands of combinations); a data-saving mode
(lower-quality images); advanced content filtering (subreddits/titles/flairs/users); a
community-built/community-coded open-source model; rich inline content (tables, code blocks, Imgur
albums, Gfycat, XKCD) + a full mod suite (modmail, mod log); multi-column tablet UI.

### User

Talk me through representing bluesky posts and reddit style posts when they don't have titles.

### Gemini

The classic protocol-bridging challenge: `app.bsky.feed.post` is microblogging — `text`, timestamps,
optional embeds, **no title field**; Reddit is Title (required) + optional Body.

1. **Posts authored inside Graze.** The **Markdown convention (recommended):** don't invent a lexicon
   other clients can't read — use standard `app.bsky.feed.post`; when a Graze user types Title + Body,
   combine into one text string with the title as a Markdown header on the first line, a double
   line-break, then the body (`# Title\n\nBody`). Graze's parser splits the first `#` line as the
   Reddit-style title and renders the rest as body; standard Bluesky users just see a well-formatted
   post.
2. **Standard Bluesky posts (external content).** **Smart text extraction:** Title = first complete
   sentence (stop at first `.`/`!`/`?`), or first newline, or hard 80-char cap — whichever comes first;
   remainder = Body. **Media/link fallback:** `app.bsky.embed.external` link previews carry an
   Open-Graph `title` — use the embed's title as the Graze title and the user's text as the body.
3. **The "titleless card" UI.** If a Bluesky post is very short (< ~150 chars, no embeds), don't split —
   drop the bold header and render full text at medium size (mirrors how modern Reddit renders short/
   image-only posts), blending microblog and forum idioms.

### User

How hard would this be to do in a PWA/SPA? [re: Pixel Pals — Tamagotchi-style virtual pets]

### Gemini (condensed to substance)

Highly feasible, moderately difficult. Three pillars: **rendering** (CSS sprite sheets with
`steps()`, or `<canvas>` for complex interactions), **state management** (a state machine on a timer —
hunger/happiness/energy/age degrade; a `requestAnimationFrame`/`setInterval` loop; persist last-close
timestamp in localStorage/IndexedDB and retroactively apply decay so the pet "lived" while closed),
**PWA capabilities** (Service Workers to cache sprites/sounds offline; Web Push for hunger
notifications; a Web App Manifest for installability). **The catch — PWA vs native limits:** Dynamic
Island / notch residency = impossible (PWAs are confined to the tab/installed window); lock-screen /
Live Activities = impossible; background execution = limited (PWAs freeze when backgrounded → simulate
elapsed time on reopen). Cross-platform is excellent (one codebase). Building the pet logic/animation/
feeding in-window is a straightforward, fun front-end project; it only gets hard if the pet must break
out of the browser window into the OS UI.

---

## Pointers (where this was distilled)

- Subjective-consensus karma + graph-weighting + the two-mode (Global/Subjective, Jetstream-first,
  no-mandatory-AppView) design → `thinking/app/` (the social-tree thesis made concrete; bears on the
  E80↔E62 architecture fork, COHESION §63).
- "Better than Reddit" features (BYO-mods, algorithmic sliders, zero-cut tipping, forking, offline) +
  third-party-app feature research → `thinking/app/` + `research/` (extends
  `reddit-aggregation-ux-slashdot-baseline-2026-07.md`).
- New ecosystem rows (atwork.place, Sifa, the third-party Reddit clients as design prior art) →
  `ECOSYSTEM.md` (flag dialogue-sourced / pending-verification).
