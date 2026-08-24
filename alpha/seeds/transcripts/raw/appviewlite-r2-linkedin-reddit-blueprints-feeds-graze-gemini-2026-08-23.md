# Raw: AppViewLite integrations · Bluesky origin retold · R2 pricing · LinkedIn/Reddit UI blueprints · Bluesky feeds mechanics · Graze Social (Gemini, pasted 2026-08-23)

**Preservation status: preserved-condensed (cleaned-paste, content-faithful — NOT byte-pristine) —
PLAYBOOK §4.** Source: **Gemini** ("Gemini is AI and can make mistakes." footers throughout),
multiple sessions of unknown dates, pasted 2026-08-23. **Threads 6 and 9 demonstrably predate the
2026-08-21 Graze→Forage rename** (they treat "Graze" as the live project name); the parenthetical
annotations after several footers — **"(stellin)"**, **"(forage)"**, *"(interesting almos tall
still work after graze to forage rename)"*, *"(led to graze rename)"* — are **owner commentary
added at paste time**, reproduced verbatim where they occurred; they bind threads to projects and
are the most corpus-valuable lines in the paste. The AppViewLite Q&A pair appeared **twice
verbatim** in the paste → reproduced once (the BookHive-2026-08-09 pattern). Two threads respond
to **screenshots not in the paste** (the Treksky feed view "image.png", and The 'Gram feed). No
secrets.

**Fact posture.**

- **AppViewLite (thread 1):** the base row is verified in ECOSYSTEM §5f (2026-06-22). The
  multi-protocol detail here (Mastodon/Nostr/RSS/Tumblr/imageboards **read-only**, no
  write-bridge, per its own feature table; mechanism guesses about REST/WebSocket fetching) is
  dialogue-sourced `[UNVERIFIED]` — plausible but Gemini's "grounded answer" cites no code.
- **Bluesky origin story (thread 2):** a retelling the corpus already holds — the narrative
  (Dorsey 2019 → Graber 2021 autonomy → PBC spin-out → Musk 2022 → X) is recorded as
  dialogue-sourced `[UNVERIFIED]` under **E52** and the 2026-07-23 origin raw. This version adds
  nothing verified; **cite E52's posture, don't launder this telling.** The owner's own prompt
  framing ("that autonomy turned into a real product… that's why we are able to build serious
  applications at real 40M user accessible scale without reinventing the wheel") is the useful
  part — it's the owner's articulation of why atproto is the substrate.
- **R2 pricing (thread 3):** point-in-time vendor pricing, `[UNVERIFIED]` and volatile — recheck
  against cloudflare.com before any budget rests on it.
- **LinkedIn/Reddit blueprints (threads 4–5):** UI/UX abstraction, not factual claims; the legal
  framing ("mechanics aren't protected, trade dress and nomenclature are") is a model's
  layperson gloss, **not legal advice**.
- **Feeds mechanics (thread 8):** consistent with the corpus's verified feed-generator material
  (E62/§5i); the lists-vs-feeds distinction (lists = PDS records assembled client-side from
  standard queries; feeds = third-party generator services) matches known architecture. Tool
  specifics (SkyFeed regex blocks, blueskyfeedcreator options) `[UNVERIFIED]`.
- **Graze Social (thread 9):** dialogue-sourced `[UNVERIFIED]` (PBC status, See-Less 25-click
  global filter, sponsored-feed monetization) — registered in ECOSYSTEM §5m with that caveat.

**What this is.** A grab-bag of Gemini sessions the owner batch-pasted, whose real value is
provenance: the **LinkedIn-shape blueprint feeding stellin**, the **Reddit-shape blueprint
feeding Forage**, the **tagline round that survives the rename**, and the **Graze Social thread
the owner marks as what "led to graze rename"** — the documented trigger for Graze→Forage
(E73/E122; `alpha/research/forage-name-clearance-2026-08.md` is the clearance record). Plus: R2
price points for the co-op-economics thread (E132), feed/list mechanics, and two book-ecosystem
answers adding **Inventaire**.

---

## Thread 1 — AppViewLite: Mastodon / Nostr / RSS / Tumblr extent
*(this Q&A appeared twice verbatim in the paste; reproduced once)*

**Owner:**

> What is the extent of the mastadon, nostr, rss and tumblr integrations for
> https://github.com/alnkesq/AppViewLite

**Gemini** *(two rounds, condensed-faithful; `[UNVERIFIED]`)*:

> Per the repo's feature comparison table: **read-only viewing (✓)** for Mastodon/Fediverse,
> Nostr, RSS, Tumblr (and imageboards) — browse and consume those feeds inside the AppViewLite
> client alongside ATProto content; **no write support (⛔)** — no posting, replying, liking,
> boosting back to those networks (contrast: official bsky.app supports none of them).
> Mechanisms (asked to "dig in… grounded"): Mastodon via public REST/ActivityPub actor fetches;
> Nostr via relay WebSockets (Kind 1 notes by pubkey/relay); RSS/Atom via periodic XML fetch;
> Tumblr via public API/feed endpoints. Design intent: a low-resource self-hosted AppView/reader
> using external protocols as **supplemental read-only data sources**, not a bi-directional
> bridge (contrast Bridgy Fed's identity mapping).

## Thread 2 — the Bluesky origin story, retold on the owner's framing

**Owner** *(verbatim — the framing is the keepable part)*:

> ell me the story of twitter commissioning what would become bluesky only for the woman who was
> recruited to lead it requiring significant autonomy only for twitter to be bought
> unintentionally by Elon musk, gutted of its staff and functioning community support, and
> further pummeled into an empire known only as x, and the coop part is that autonomy turned
> into a real product and infrastructure for a more open and user friendly social protocol space
> and that's why we are able to build serious applications at real 40M user accessible scale
> without reinventing the wheel. It's a platform for online social activity, but open. Thanks.

**Gemini** *(condensed-faithful; narrative `[UNVERIFIED]` — E52 posture applies)*:

> "Strategic insulation": 2019, Dorsey launches Bluesky to build a protocol, not a product;
> 2021, Jay Graber recruited and conditions leadership on **significant autonomy** — spin-out as
> an independent Public Benefit Corporation, Twitter funding without control; 2022, Musk
> acquires Twitter — layoffs, degradation, rebrand to X — and the autonomy pact firewalls
> Bluesky from the chaos; the result is the AT Protocol: user portability, decentralized
> moderation, open infrastructure at scale. "The very platform which attempted to own the
> future of communication ended up losing the one piece of it that truly mattered."

## Thread 3 — Cloudflare R2 tiers, 1–50 TB, CDN op-cost reduction

**Owner:**

> Cf r2 storage tiers and cost for 1, 5, 10, 20, 30, 40, and 50 TB

**Gemini** *(point-in-time vendor pricing, `[UNVERIFIED]`, condensed-faithful)*:

> Two tiers, both **zero egress**: Standard $0.015/GB-mo (10 GB free; Class A $4.50/M, Class B
> $0.36/M; free retrieval) vs Infrequent Access $0.010/GB-mo (no free tier; **$0.010/GB
> retrieval fee**; Class A $9.00/M, Class B $0.90/M). Base monthly storage (Standard / IA):
> 1 TB ≈ $15.36 / $10.24 · 5 TB ≈ $76.65 / $51.20 · 10 TB ≈ $153.45 / $102.40 ·
> 20 TB ≈ $307.05 / $204.80 · 30 TB ≈ $460.65 / $307.20 · 40 TB ≈ $614.25 / $409.60 ·
> 50 TB ≈ $767.85 / $512.00. Standard includes 1M Class A + 10M Class B ops free monthly. IA
> caveat: frequent reads make Standard cheaper.

**Owner:**

> Can I reduce the op cost through cdn?

**Gemini:**

> Yes — a custom domain on the bucket puts Cloudflare's CDN in front: **cache hits never touch
> R2** (zero Class B ops, zero retrieval fees; a 95% hit ratio eliminates 95% of read-op
> cost). Maximize with explicit `Cache-Control` headers (`public, max-age=31536000` for static
> assets), Cache Rules, and Tiered Cache. **The IA trap:** low-hit-ratio files on IA pay both a
> Class B op and the $0.01/GB retrieval on every cache miss — IA + CDN can end up costlier
> than Standard.

**Owner:**

> How long is this — 31536000

**Gemini:**

> 31,536,000 seconds = 1 year (365 days) = 52 weeks = 8,760 hours.

## Thread 4 — the LinkedIn-shape UI/UX blueprint — owner annotation: **(stellin)**

**Owner:**

> I want you to analyze LinkedIn interface and workflows and compile a set of descriptions of
> each with particular notes comparing to other social media sites and describing in detail how
> the interface is structured in way that someone could use its form as a blueprint to build a
> site that is roughly shaped the same but without copying any of the trademarks or other
> protected content. This is strictly about UI, UX, workflows and capabilities

**Gemini** *(the blueprint, condensed-faithful — design input for stellin)*:

> A professional network is **a relational database masked as a social feed** — it optimizes
> for trust, verifiable data, and utility, not visual dopamine.
>
> **Global architecture:** persistent top nav (omni-search over people/companies/jobs/groups;
> Home / Network / Jobs / Messaging / Notifications; settings) over a three-column desktop grid
> that collapses to single-column tabbed mobile: left rail = identity/context (mini-profile
> card, metrics), center = composer + infinite algorithmic feed, right rail = discovery +
> monetization. The insight: variable user content is deliberately flanked by stable personal
> utility and discovery/monetization engines.
>
> **Modules:** (A) **Feed** — card structure: author metadata (avatar/name/headline/time/
> connection degree) → truncated text with "see more" (reading-intent measurement) → media
> incl. **inline document carousels** (a professional-network-specific necessity) → social
> proof → action bar; typography/whitespace over visual aggression. (B) **Profile** — a living
> resume of stacked modular cards (hero; about; experience; education; skills; written
> recommendations); **standardized data entry is the load-bearing rule** — company/school
> fields select existing database nodes, never free text, because that powers search and the
> graph. (C) **Graph engine** — reciprocal connections + asymmetric follows; pending-request
> hub; proximity-based suggestions; **degrees-of-separation badges (1st/2nd/3rd) on every user
> card** as the defining UX feature. (D) **Jobs** — two-pane enterprise layout (scrollable
> card list | sticky detail), faceted search, quick-apply piping structured profile data. (E)
> **Messaging** — persistent collapsible widget + full inbox; cold/paid outreach visually
> separated from real connections.
>
> **Comparative:** professional network = mixed graph, career-milestone currency, structured
> data + dense filters; vs Facebook (reciprocal, emotional engagement), X (asymmetric,
> velocity), Instagram (asymmetric, visual dominance).
>
> **Build principles:** trust indicators above all (avatar always paired with a headline —
> who is speaking and why they're qualified); tiered-access UI (gating/blur as the monetization
> surface); **action-density tolerance** — professional users accept enterprise UI complexity
> (split panes, dense tables, multi-step modals) because they're on a task.

## Thread 5 — the Reddit-shape UI/UX blueprint — owner annotation: **(forage)**

**Owner:**

> I want you to analyze the Reddit interface and workflows and compile a set of descriptions of
> each with particular notes comparing to other social media sites and describing in detail how
> the interface is structured in way that someone could use its form as a blueprint to build a
> site that is roughly shaped the same but without copying any of the trademarks or other
> protected content. This is strictly about UI, UX, workflows and capabilities

**Gemini** *(the blueprint, condensed-faithful — design input for Forage)*:

> The core paradigm: a **topic-driven aggregation platform** — content organized around
> subjects (nodes/communities), not people.
>
> **Desktop layout (three-column):** sticky global header (branding + scoped search; Home /
> Popular / All; create-post, notifications, profile w/ dark mode); left sidebar = the
> directory (global feeds, **user-created custom feeds grouping multiple topics**, joined
> nodes); center = the arena — sort controls (Hot/Best/Rising/New/Top) over the **post unit**:
> voting axis on the left edge (net score between +/−), tiny metadata line (node · author ·
> timestamp), title + payload (thumbnail/image/video/text preview), action bar (comments,
> share, save, report); right sidebar = context & governance — about card (description, member
> count, active-now, join), **the numbered local rules list**, moderator roster.
>
> **Core UX mechanics:** the **nested comment tree** with mandatory collapsibility (click the
> vertical line to fold a parent and its lineage — scanning arguments without scrolling
> tangents); **transparent democratic sorting** — Hot = net score with a logarithmic time
> penalty (today's 500 beats yesterday's 5,000), keeping the feed churning, vs the opaque
> "For You" black boxes elsewhere.
>
> **Submission workflow (strictly linear):** (1) context selection — the target node loads its
> rules and allowed formats; (2) format selection (tabs lock if the node disables a type); (3)
> drafting + mandatory tags/flairs/content warnings; (4) validation against node regex/rules →
> the New queue.
>
> **Comparative:** topic-driven (score + time decay; deep collapsible trees; volunteer
> governance) vs X (persona, opaque AI, linear branches) vs Facebook (real identity, shallow
> replies) vs Discord (private servers, flat chronological chat).
>
> **IP abstraction:** the mechanics (aggregation, voting, nested comments) are standard
> paradigms; the protected parts are nomenclature and trade dress — swap subreddit/karma/
> up-down for nodes-spaces-realms / reputation-trust / endorse-boost-bury or plain +/−, drop
> the mascot and the OrangeRed accent, use generic chevrons, and build on a modern component
> library. *(A model's layperson IP gloss — not legal advice.)*

## Thread 6 — Graze taglines — owner annotation: *"(interesting almos tall still work after graze to forage rename)"*

**Owner:**

> What's a good tagline for a reddit like site named Graze which harlens back to grazing in a
> field as a metaphor for grazing the Internet

**Gemini** *(pre-rename; the metaphor family survives Graze→Forage, per the owner note)*:

> Short & punchy: *Find your pasture. · Roam the open web. · Feed your curiosity. · The
> internet's open field.* Heavy metaphor: *Wander the fields of the web. · Where the internet
> roams free. · Green pastures for curious minds. · Find a better patch of the internet.*
> Content/consumption (rumination): *Chew on the best of the web. · Digest the internet, one
> bite at a time. · Bite-sized content. Endless fields. · Fresh content, as far as the eye can
> see.* Community: *Find your herd. · A pasture for every passion. · Explore every patch of
> the web.* Feature-naming tip: sub-communities as "Pastures"/"Patches", upvotes as "Sprouts",
> the front page as "The Field."

## Thread 7 — Goodreads licensing, and the open-data equivalents

**Owner:**

> What is the license of the content on Goodreads?

**Gemini** *(condensed-faithful, `[UNVERIFIED]`)*:

> No open license. User reviews: author keeps copyright but grants Goodreads/Amazon a
> non-exclusive, royalty-free, perpetual, irrevocable, sublicensable worldwide license. Catalog
> metadata (covers, descriptions, bios): owned by rightsholders, licensed to Goodreads for
> display only. Your license: limited, non-commercial; ToS expressly forbids scraping/mass
> download and derivative apps/databases without written consent.

**Owner:**

> Is there an open data Goodreads equivalent? Especially on atproto?

**Gemini** *(condensed-faithful; BookHive/BookWyrm already registered — §5k; Inventaire is the
new name)*:

> **BookHive** (atproto — reading data as lexicon records on your PDS, portable, "buzzes" into
> the Bluesky feed); **BookWyrm** (ActivityPub, AGPL, self-hostable, pulls open metadata from
> Open Library/Wikidata); **Inventaire** — linked-open-data book catalog on Wikidata and open
> bibliographic sources: track a personal physical library, list books for lending/trade,
> reading lists under open-data principles.

## Thread 8 — Bluesky feeds mechanics (Treksky, The 'Gram, hashtags, building feeds, lists)

*(The Treksky and 'Gram threads respond to owner screenshots not in the paste.)*

**Owner:**

> Ok m looking at bluesky and I have the treksky feed, how do I post into it or is it fixed?

**Gemini:**

> Custom feeds aren't containers you post into — they're user-created algorithms scanning the
> whole network. Post normally with the likely trigger keywords/hashtags (#StarTrek etc.) and
> the feed's generator scoops it up. Creators pull by keyword scraping, curated account lists,
> or hybrids; the feed's About page usually lists the criteria.

**Owner:**

> I don't see the option

**Gemini:**

> Right — the creator (@emlowz.com) didn't publish the trigger rules, so the feed operates as
> a **black box**: there is no "post to feed" button anywhere; use obvious franchise keywords
> and hashtags and see if the scrape catches it.

**Owner:**

> How about this one? *(The 'Gram)*

**Gemini:**

> A **filter feed**, not a community feed: "all the pictures posted by people you follow" —
> the only trigger is attaching an image, and it's relative to each viewer's follow list.
> Community feeds are for discovery (push to strangers by topic); filter feeds are curation
> (reshape your own timeline — an Instagram-style lens). No tags needed.

**Owner:**

> Compare feeds to hashtags

**Gemini:**

> Hashtags are labels on content (zero customization; a click shows every public post with the
> tag); feeds are subscribable user-made algorithms that *consume* those labels (multi-topic
> filters, image-only, account lists — the user-chosen replacement for a central algorithm).
> Feeds rely on hashtags as signal flares; the right tag gets your post surfaced to niche
> communities that don't follow you.

**Owner:**

> Explain making a new feed

**Gemini** *(condensed-faithful; tool specifics `[UNVERIFIED]`)*:

> Three routes: (1) **no-code builders** — blueskyfeedcreator.com (parameter-based: hashtags,
> blocklists, media-only, account/starter-pack restriction) and SkyFeed (visual blocks +
> regex, publish straight to your profile); (2) **developer** — a Feed Generator is a server
> returning post URIs: start from the official TS/Python template (SQLite), subscribe to the
> firehose, index by your algorithm, host over HTTPS, run the publish script; (3) **Lists** —
> if you just want specific accounts, the built-in Lists feature pinned to home acts as a
> curated timeline with no algorithm service at all.

**Owner:**

> And lists are handled locally rather than curated server side ? — Is graze a feed builder?

**Gemini:**

> Lists aren't strictly local: the list is a record on your PDS; viewing one just has your
> client query standard relays/AppView for those accounts' posts — no third-party generator
> service in the path (that's the actual feeds-vs-lists distinction). And yes — **Graze
> (Graze Social)** is a custom feed builder for Bluesky, built around "own your algorithm."

## Thread 9 — Graze Social — owner annotation: *"(led to graze rename)"*

**Owner:**

> How does graze work and does it make money

**Gemini** *(condensed-faithful, `[UNVERIFIED]`)*:

> **How it works:** build/customize/publish custom Bluesky feeds from foundation blocks or by
> remixing public feeds; interactive personalization rides Bluesky's native "See More"/"See
> Less" buttons — See-Less hides the post, repeated See-Less on an author filters them from
> that feed for that user (preferences expire over time), and **25 See-Less clicks on one
> author across all Graze feeds applies a global per-user filter**; feed operators get only
> anonymous aggregate metrics. **Money:** a US public benefit corporation with outside
> investment; feed operators can open their feeds to advertisers; brands run **sponsored
> content on custom feeds**, splitting revenue with the operators who curate the niche
> audiences.

*(End of paste. The existence of Graze Social — an established third party operating under the
"Graze" name in exactly this space — is what the owner marks as having "led to graze rename":
the forum project renamed to **Forage** on 2026-08-21; clearance record at
`alpha/research/forage-name-clearance-2026-08.md`.)*
