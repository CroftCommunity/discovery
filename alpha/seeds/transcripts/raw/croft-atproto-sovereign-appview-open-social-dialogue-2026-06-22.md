# Croft — AT Proto sovereign PDS/AppView + open-social naming/interop dialogue (Gemini, 2026-06-22)

> **Provenance / fidelity (PLAYBOOK §4).** Source: an AI-generated research dialogue with **Gemini**,
> pasted by the user (no canonical export). **Cleaned-paste, content-faithful — NOT a byte-pristine
> export.** Render chrome stripped (bare citation breadcrumbs reduced to bracketed `[src: …]`); code
> blocks preserved verbatim; the most repetitive assistant prose lightly compressed. **Gemini is
> flagged as sometimes unreliable; every substantive assertion was fact-checked** — see the companion
> `croft-atproto-sovereign-appview-open-social-dialogue-2026-06-22-FACTCHECK.md`.
>
> **Dupe/fork note (per the user, 2026-06-22):** the PDS/AppView "sovereign club" middle of this body
> was pasted **twice** — a primary run and a near-duplicate fork/re-run that **diverged at the tail**
> with net-new content (open-source client recommendations; the Twitter Circles → Communities → Group
> DMs history). This file consolidates the **superset** (primary + the fork's net-new tail), filed
> once to avoid triplication.
>
> **Overlap with already-filed intakes (cite, don't re-verify):** the PDS/Relay/AppView mechanics
> overlap the §27 architecture explainer (`atproto-architecture-appview-relay-explainer-2026-06-22.md`);
> Germ overlaps §26 (`croft-atproto-pds-germ-privatedata-dialogue…`) and §25 (crypto-wars). The
> standing source of truth for atproto/iroh facts is
> `atproto-atmospheric-web-iroh-mobile-FACTCHECK.md`. MST is correctly atproto's structure throughout.

---

## Part 1 — Naming: the opposite of a walled garden

**USER:** What would be the opposite of a walled garden? [pastes a Verge headline: "Twitter follows
Facebook down the walled garden path"]. For naming a project.

**GEMINI:** The opposite is an **open ecosystem / digital commons** — interoperability, decentralized
control, free movement of data and users. Naming angles:
- **Nature:** The Commons, Open Range, Greenfield, Wildwood, The Glade/Clearing.
- **Tech/philosophy classics:** **Bazaar** (from Eric S. Raymond's "The Cathedral and the Bazaar" —
  the walled garden is the Cathedral, the open web the Bazaar); **Agora** (the ancient Greek public
  assembly/market space); Fediverse/Federation; Protocol.
- **Structural/geometric:** Expanse, Concourse, Portal/Gateway, **Nexus**, Horizon.
- Quick pick: The Commons or Agora for a community/data tool; Bazaar or Open Range for
  barrier-breaking software.

**USER:** I like "wild" but not "wood," wild garden? Hmm.

**GEMINI:** "Wild Garden" has an ironic twist (letting the thing they want to fence in grow
unchecked). Variations leaning into "wild": **Rewild** / Project Rewild (ecology: tearing down fences,
letting nature heal — restoring the open web); **Wildscape**; **Wildland(s)**; **Outwild**;
**Wildroot** (foundational/protocol + open-source "root").

**USER:** How does rewild translate across cultures? Any red flags?

**GEMINI:** It carries baggage outside Western English tech circles. (1) **Linguistic:** in Romance
languages "wild" maps to *salvaje*/*sauvage*, which lean toward "savage/brutal" — could read as
unstable/unsafe. East Asian translations lean toward wilderness/open-country, with connotations of
wasteland/desolation/abandonment. (2) **Socio-political red flag:** "rewilding" has pushback in
conservation circles for association with **"fortress conservation"** — historically removing
Indigenous/local populations to create human-free "pristine" wilderness ("green colonialism") — the
exact opposite of giving power back to users. (3) **Agrarian friction:** in rural Europe, rewilding
means reintroducing apex predators (wolves/bears) and reads as economic threat/top-down overreach.
Verdict: fine for tech/open-source/Web3 audiences; for global non-technical communities prefer
structure-words (The Commons, Agora, Nexus).

**USER:** I want something in the rewild theme but more palatable.

**GEMINI:** Shift to regeneration/resilience/open-landscape vocab: **Meadow/Meadowscape**, Prairie,
Glade; **Perennial**, **Regen/Regeneration**, **Overgrow**; **Canopy**, **Grove**, **Mycelium** (the
underground fungal network — the ultimate decentralized-network metaphor). Standout: **Meadow /
Project Meadow** — keeps the botanical theme, removes the walls.

**USER:** Something about replanting or root-planting maybe. Meadow is good but used by a close
project (Willow).

**GEMINI:** Replanting/rooting is a great pivot (breaking the artificial pots so roots anchor in the
real earth). Ideas: **Rootstock** (the established root system you graft onto), **Taproot**, **Radix**
(Latin for "root", sleek); **Rhizome** (horizontal underground network, no central hub — the
decentralization metaphor), **Understory**; **Reseed**, **Outroot**, **Till/Tillage** (preparing the
soil for open growth). Standout: **Rootstock**, or **Radix/Reseed** for something punchier next to
"Willow".

**USER:** Tillage is interesting, how does it translate?

**GEMINI:** Much safer than "rewilding" — a foundational agricultural concept, implies deliberate
preparation/hard work/fertile foundation. Romance: *Labranza* (Sp), *Labourage* (Fr) — honest
industrious labor. East Asian: 耕作 (Gēngzuò / Kōsaku) — plowing/cultivating, connotes diligence.
**One modern red flag:** in regenerative-ag/permaculture circles, heavy tillage is seen as negative
("No-Till" is championed — excessive tilling disrupts the soil microbiome, accelerates erosion,
releases carbon). So in ultra-green-tech circles "Tillage" might read as old-school/disruptive. But it
signals human agency ("we work the ground to build a better environment") vs rewild's "get humans out
of the way."

**USER:** Also sounds like village so is a bit evocative — but how much of an issue is the no-till
stuff?

**GEMINI:** The **"village" connection is a branding win** (community/warmth). The "no-till" issue is
a non-issue outside climate-tech/carbon/permaculture circles — 99% of a general tech/dev audience
reads "Tillage" through its historical foundational meaning. Unless you're launching a regenerative-
farming platform, it's minor background trivia.

**USER:** What if I shorten the app name to Till and the platform to Tillage?

**GEMINI:** That app/platform split is sharp: **Till** = the active tool (verb — `till launch`,
`till connect`, `till status`), **Tillage** = the broader ecosystem/platform (the cultivated land).
Snappy verb vs heavier noun. One quirk: "till" also = a cash register/money drawer in English (and
glacial sediment in geology), but pairing with "Tillage" overrides that. Verdict: a clean, scalable
naming architecture that respects a neighboring project like Willow.

*(FILED AS RESERVOIR, NOT A DECISION — see `NAMING.md`. Till/Tillage and the open-ecosystem candidates
are vocabulary to consider, not adopted.)*

---

## Part 2 — Threads / Mastodon / ActivityPub interop

**USER:** So there is a big Mastodon instance called Threads and then Meta launched a product called
Threads but none of it interoperates?

**GEMINI:** A twist: they **do** interoperate, and it's not two things named "Threads." Meta built
Threads to plug into the Fediverse via **ActivityPub** (the W3C decentralized protocol that powers
Mastodon). Threads acts like a massive Meta-owned ActivityPub server. Cross-following works
(@zuck@threads.net from Mastodon), as do likes/replies/reposts. **Catches:** Fediverse sharing is
**opt-in** (toggle in settings); many Mastodon servers **defederated** (blocked) Threads over Meta
distrust; and DMs/edit-history/content-warnings don't translate cleanly yet.

**USER:** So Meta Threads is built on ActivityPub or a custom version?

**GEMINI:** Both, in a sense: **standard protocol, proprietary engine.** Meta uses the official W3C
ActivityPub + ActivityStreams 2.0 (not a Meta-only fork — otherwise Mastodon couldn't talk to it), but
the software running it is **closed-source corporate code** (vs Mastodon's open source). Where it gets
custom: base ActivityPub is bare-bones (Person/Note/Like/Follow) and doesn't define quote posts,
algorithmic trends, or advanced moderation — so platforms build **extensions via Fediverse Enhancement
Proposals (FEPs)**, and Meta implements those (e.g. quote-post FEPs) in custom code. [src: engineering.fb.com]

**USER:** Does Threads and Instagram integrate?

**GEMINI:** Deeply (Threads launched as an Instagram offshoot). Identity: you couldn't originally have
a Threads account without Instagram (shared login, bio auto-fill, verification carryover) — *though
standalone signup now exists in the EU/UK under the DMA*. Network: Threads seeds your graph from
Instagram follows. Content: share Threads posts to IG Stories/Feed/DMs. Ads: unified Meta Ads Manager.
Shared algorithmic memory across the two feeds.

**USER:** Could I stand up an AP server and integrate with Threads?

**GEMINI:** Yes — because Threads uses standard ActivityPub. Technical prerequisites: **WebFinger
discovery** (`/.well-known/webfinger`), **strict HTTP Signatures + Digests** (RSA-SHA256 — most
bare-bones servers fail here), valid SSL/TLS. Practical hurdles: the **opt-in filter** (you only see
Threads users who enabled Fediverse sharing); **IP/cloud-block reputation** (Meta blocks scrapers on
low-rep VPS ranges); **rate limiting**. Pro tip: use GoToSocial or AP middleware to handle the HTTP-
signature handshakes rather than hand-rolling crypto.

---

## Part 3 — The AP↔AT bridge (Bridgy Fed / A New Social)

**USER:** How functional is the AP to AT social-networking bridge?

**GEMINI:** Remarkably robust and **bi-directional**, driven mostly by **Bridgy Fed** (run by the
nonprofit **A New Social**). It's a real-time translator: follow/reply/like/repost across networks
from your native app. From Mastodon you look up a Bluesky user as `@username.bsky.social@bsky.brid.gy`;
from Bluesky you reach a Mastodon user via `@ap.brid.gy` routing. Advances: long-form bridging
(WordPress/Ghost → structured doc on the ATProto side), account **"bouncing"** (true portability —
migrate fully into a PDS without losing bridged followers), granular moderation dashboard. Friction:
strictly public-only (filters DMs/unlisted/followers-only); post-editing is one-way; character
truncation; feature degradation (polls/custom emoji break).

**USER:** Tell me about the nonprofit — mission, goals, stability.

**GEMINI:** **A New Social** — an independent nonprofit (described as 501(c)(3) [UNVERIFIED — sources
say "nonprofit"]) housing the open-source bridging ecosystem. Mission: *"centering social media around
people, not platforms"* — liberating users' social graphs. Goals: true interoperability, a **"credible
exit"** (the **Bounce** migration tool), expanding beyond microblogging. Stability: lean (co-founder
**Ryan Barrett / "snarfed"** + ~2 full-time volunteers + community contributions); grants + Patreon,
no ads/tracking. Scaling: grew from ~2,000 → ~150,000 users; re-architected to cut serving cost ~5×
(**$0.15 → $0.03** per active user/month); migrated a **~2.3 TB / ~700M-row** datastore; notes that if
~1/4 of bridged users gave $1/month it'd be self-sufficient. [src: blog.anew.social "Bridging on a budget"]

**USER:** Is it all open source? Does anyone else run a bridge instance and could they?

**GEMINI:** Entirely open source — GitHub **snarfed/bridgy-fed**, public-domain / **CC0**, mostly
**Python**, using **Granary** (social-web data translator) and **Arroba** (Python AT Protocol / PDS
impl). You *can* run your own (Google Cloud SDK + Firestore; the ATProto side must operate as a PDS
piping to the Relay), but almost no one runs a *duplicate public* instance — because 5 public bridges
would create 5 proxy profiles per user, fragmenting the graph and inviting impersonation. So the
community treats **fed.brid.gy** (A New Social) as the single trusted registry. Separate independent
bridges exist: **RSS Parrot** (RSS/Atom → Fediverse actors), **Pinhole** (Bluesky → AP, one-way),
**Fedisky** (an ActivityPub extension for Bluesky PDS).

**USER:** Can a user opt out of Bridgy Fed to run their own without these issues?

**GEMINI:** Yes — deploy a private single-user instance. You turn off bridging on the public service
(purging your proxy profiles), then run your own. Benefits: cryptographic sovereignty (your PDS owns
the keys, no spoofing), zero graph fragmentation. New costs: the Google-Cloud/Firestore infra tax (you
inherit the cloud bill + optimization), Relay rate limits for a new PDS, and "bot handle" confusion
(followers must route via your custom `@bsky.yourdomain.com` instead of the universal `@bsky.brid.gy`).
For most people the operational overhead is exactly why they delegate to the central nonprofit.

---

## Part 4 — The sovereign PDS/AppView "club" (the design-relevant core)

**USER (flagged "esp good here, bluesky private blocking with our own appview as a feature"):** If I
had my own PDS and AppView could I block users in a nonpublic way and have it be effective? [quotes
Bluesky's own statement: *"Blocks on Bluesky are public. This is not what users want, not what we want,
and a thorn in our side… This is a tension with the structure and values of the protocol, and might
persist even with private data features added to the protocol."*]

**GEMINI:** The short answer is **halfway.** AT Proto splits blocking into **inbound** (blocking them
from your sight) and **outbound** (blocking yourself from theirs). **Inbound = 100% effective and
private:** you own the AppView (which builds your feed), so you drop their posts/replies/notifications;
the blocklist lives only in your AppView, un-scrapable. **Outbound = ineffective while federated:**
your PDS broadcasts public posts to the Relays/firehose → the main Bluesky AppView serves them to the
"blocked" user regardless. The only way to enforce outbound is to publish a **public** `app.bsky.graph.block`
record. True private outbound blocking requires unfederating entirely.

**USER:** But on a large public network, preventing someone from seeing your otherwise-public content
is functionally impossible; what you *can* do is shape the experience so it doesn't matter.

**GEMINI:** Spot on — outbound blocking on a public network has always been an illusion (incognito,
sockpuppet, API/RSS). **X/Twitter leaned into this:** blocked users can still *view* public posts but
can't *interact*. With your own PDS+AppView the leverage is **experience-shaping**: starve the loop
(drop their data inbound), break amplification (refuse to render their quote-posts/feeds of you), deny
direct engagement (no DM, no tag-alert). Data visibility is binary (encrypted/private vs public); the
power is stripping their ability to *touch your corner* or get a reaction.

**USER:** And the key is that since no one can see your feed they can't really know without AppView
server access — you could build an equivalent experience where the blocked user is basically given a
shadow ban / black hole as far as the blocker is concerned.

**GEMINI:** Exactly — a **perfect local shadow ban**, exploiting the PDS/AppView separation of powers.
(1) *Mirage of engagement:* no public block record, so the target's reply "succeeds" and their
followers may see it — they're screaming into a void. (2) *Private black hole:* your AppView ingests
the firehose, spots their ID, silently discards — no notification, no thread branch. (3) *Absolute
secrecy:* the blocklist is a private DB row, un-scrapable (vs Bluesky's public ledger which third-party
tools scrape).

**USER:** Could you discard all interactions for a blocked user and keep the block list private at the
AppView level?

**GEMINI:** Yes. The AppView is the gatekeeper between firehose and client: **replies/mentions** —
intercept `listNotifications`/`getPostThread`, delete those nodes from the JSON; **likes/reposts** —
subtract from counts or sanitize `getLikes`; **quote posts** — drop the reference so it never indexes
as a quote of you. All downstream of the firehose, nothing written back. (Layer table: PDS=public
posts/no block records; Relay=public broadcast; **your AppView=private filter list, 100% private**.)

**USER:** How much processing power would an AppView to implement this take — doable as a small entity?

**GEMINI:** 100% doable; $5 vs $500/mo depends on approach. **Path 1 — AppView Proxy (ultra-light):** a
lightweight XRPC proxy between your client and the public Bluesky API; forward the request, scrub the
returned JSON against your blocklist, hand back. ~1 core / 512 MB RAM, ~$5/mo VPS or a Raspberry Pi.
**Path 2 — Dedicated AppView via Jetstream:** connect to **Jetstream** (the compressed-JSON firehose
translator), run a lightweight indexer (à la AppViewLite); 2–4 vCPU, 4–8 GB RAM, 100–200 GB NVMe,
~$15–35/mo (Hetzner/DO/Railway). For a private shadow ban, Path 1 is the home run.

**USER:** Could I also create a mechanism to let users DM me via a direct method outside of Bluesky?

**GEMINI:** Yes — you own your DID, so publish custom routing. **Method 1 — DID Document service
endpoint:** append an arbitrary service to your DID doc:
```json
"service": [
  { "id": "#atproto_pds", "type": "AtprotoPersonalDataServer", "serviceEndpoint": "https://pds.yourdomain.com" },
  { "id": "#external_dm", "type": "com.yourdomain.messaging.gateway", "serviceEndpoint": "https://api.yourdomain.com/v1/secure-dm" }
]
```
**Method 2 — custom-lexicon "sidecar" record** (`com.yourdomain.actor.contact`) pointing at Signal/
Matrix/Session, rendered as an "Alternative DM" button. **Method 3 — AppView OAuth bridge** (lowest
friction): a vanity link in bio → user signs in via ATProto OAuth (proving identity, anti-spam) →
message lands in your private backend (Discord webhook / encrypted SQLite / log), never touching a
Bluesky-owned server.

**USER:** Is anyone doing these things now as a service?

**GEMINI:** Split: the **out-of-band DM is live** — **Germ DM** (E2EE on MLS, ATProto-OAuth identity,
profile badge → launches an encrypted chat off-Bluesky). The **private-AppView shadow-ban proxy is
NOT a commercial service** — it's a build-it-yourself developer concept (Jetstream + `@skyware/jetstream`
/ `@atproto/api`; clients like Openvibe/Graysky/Flux do client-side filtering). Why no SaaS: a
commercial proxy would have to handle your OAuth/live authenticated stream — a trust barrier most users
won't cross. So the "local black hole" is a self-host trick.

**USER:** But if I built my own PDS and AppView for a small group with our own custom handle domain,
we wouldn't need a third party or OAuth, would we?

**GEMINI:** Right — that's data sovereignty. **You become the OAuth provider** (your PDS is its own
decentralized OAuth server); members log in with `@alice.yourdomain.com`, or for a bespoke closed app
you skip OAuth entirely (API keys). Zero third-party overlook: the blocklist/black-hole is invisible
(no public ledger to scrape); internal interactions route through your infra. The only time you touch
the global identity system is when an *outsider* wants to reach your tools (they auth against their own
PDS to prove identity, peer-to-peer).

**USER:** Are there other sought-after features/abilities we can unlock?

**GEMINI:** (1) **Asymmetrical federation ("gated castle"):** pull the whole global firehose in, but
configure your PDS *not* to broadcast internal replies/records out — monitor the world, stay invisible.
(2) **True ephemerality / hard deletions:** your PDS holds the master copy, so a retention policy
("self-destruct after 48h") actually purges rows+signatures and your AppView incinerates them locally.
(3) **Custom lexicons (mini-apps on one identity):** invent `com.yourdomain.*` schemas (calendar,
inventory, wiki) indexed by your AppView. (4) **Semantic AI firehose pipelines:** pipe incoming data
through a local LLM (summaries, auto-categorize, sentiment-drop hostile content).

**USER:** What are popular/great custom lexicons we could implement?

**GEMINI:** Real community building blocks: **`com.whtwnd.blog.entry`** (WhiteWind — long-form
markdown up to 100k chars → internal wiki/knowledge base); **`fyi.unravel.frontpage`** (Frontpage —
Reddit/HN-style link aggregation + upvotes); **`xyz.statusphere.status`** (Statusphere — Slack-style
presence/status, in the official ATProto tutorial); **`com.yourdomain.profile.internal`** (contextual/
role-based profiles parallel to `app.bsky.actor.profile`). Implementation: `@atproto/lex` + `lex build`
compiles type-safe validators from a JSON schema. [src: NPM]

**USER:** I could do internal custom-only feeds?

**GEMINI:** Yes — the most powerful part for a closed group. **Mechanics of non-federated
collections:** you control the PDS routing — "if a record is in our custom namespace, keep it local,
don't broadcast." So `app.bsky.feed.post` → global Relay (public), while `com.yourdomain.feed.internalpost`
→ stays on your hardware (only your AppView sees it). Build a 100%-private "Watercooler" feed, or a
blended "Intel" feed (local AI pulls public firehose keywords interspersed with internal notes).
Scenario A (org workspace): public handle for networking; internal portal resolves
`com.yourdomain.profile.internal` (Employee ID, SSH keys, clearances) + proprietary alerts. Scenario B
(RPG hub): `com.yourdomain.rpg.character` stats/inventory, invisible to outside Bluesky clients.

**USER:** Could I have one master identity with delegate keys — an internal more-private PDS and one
with public assets?

**GEMINI:** Natively, **no** — a DID maps to **one** active PDS endpoint + one repo signing key, and
the repo is a single MST (sync it, sync the whole tree). But you can engineer the outcome. **Method 1 —
"Sidecar" service endpoint:** your DID doc lists multiple services (`#atproto_pds` →
public-pds.yourdomain.com; `#internal_secure_storage` → private-vault.yourdomain.com); the internal
client connects to the private one; data never enters the public repo/firehose. **Method 2 — Off-Repo
PDS storage (single server):** public assets → public `repo` (signed, broadcast); private assets →
isolated SQL/encrypted volume that **bypasses the repo layer**; your AppView reads the off-repo DB.
Either way you avoid "leak by design" — keeping private data out of the public content-addressed tree.

**USER:** That way the streams can't mix and the internal PDS can be restricted more.

**GEMINI:** Precisely — an **air-gap** between public presence and internal ops. Once internal storage
is off the public grid: (1) **network isolation** (VPN/IP-whitelist/ZTNA — unreachable even if a PDS
zero-day existed); (2) **scraping immunity** (never broadcast → cryptographically non-existent to the
network); (3) **granular RBAC** (per-collection access by DID, vs the public network's flat "if it's in
your repo anyone can sync it"); (4) **simplified compliance/audit** (one contained server; deletion is
real because data never left).

**USER:** What are some PDS hacks/tricks? Could I store my blobs encrypted and handle it with my
internal PDS?

**GEMINI:** Yes — the **Encrypted Blob Vault**: your client encrypts a file (AES-GCM) locally → calls
`uploadBlob` → PDS hashes the *ciphertext*, generates a valid CID; public Relays mirror the encrypted
bytes; an outside `getBlob` downloads **unreadable garbage**; your group decrypts on the fly via the
private AppView. *The public network becomes your free, distributed, encrypted hard drive.* Other
hacks: (2) **Lexicon Firewall** — intercept `createRecord`/`putRecord`, regex-scan for internal
code-names, block/mutate the record to an internal-only namespace before it federates (leak
prevention). (3) **"Ghost Delete" cache trick** — write a public post, wait ~30 min for the Bluesky
AppView to cache it for followers, then purge locally without sending a deletion to the firehose
(visible briefly via CDN caching, gone from your repo). (4) **Federated Identity Virtualization** —
point `bot001…bot999.yourdomain.com` at one multi-tenant PDS bucket for cheap bot fleets.

**USER:** Other ideas — popular/desired features this would enable across the open social web?

**GEMINI:** (1) **Collective Defense Muting ("blast radius" shield)** — on a dogpile, your AppView maps
everyone who quote-posted/replied/liked the origin and adds the whole branch to the private blocklist,
shielding the group silently. (2) **Algorithmic filtering ("clickbait deflator")** — a "hype dial /
toxicity slider" running local semantic AI to strip rage-bait / summarize threads. (3) **Unified
multi-protocol stream** — AppView aggregates Bluesky + AP (via bridge) + RSS + Reddit into one
chronological timeline. (4) **Contextual identities ("work-life air-gap")** — toggle profile records
(`profile.gaming`/`profile.professional`) for a healthy boundary on one handle. *The inversion: a
utility that serves your life, not a casino competing for your adrenaline.*

**USER (flagged: leery of attention-economy traps; "Nielsen rating vs Meta manipulation"; community
not extraction):** Do some research — more? Is the AppView arch general enough for a multi-source event
stream not all from Bluesky?

**GEMINI:** Yes — it's practically built for it. ATProto uses **CQRS** (PDS = write side, AppView =
read side); the AppView just ingests an event stream, normalizes, serves an API — source-agnostic. It
can run concurrent ingestion workers:
```
[ATProto Jetstream] ──┐
[ActivityPub Inbox]  ──┼─➔ [Your Custom AppView] ➔ [Unified API] ➔ [Your Client App]
[Nostr Relays]       ──┘
```
Five more high-demand features: (1) **Offline-first "mesh" socializing** (signed records sync P2P over
BLE/Wi-Fi-Direct, re-federate on reconnect); (2) **true protocol aggregation** (AppView as on-the-fly
AP↔AT translator); (3) **private cooperative moderation** (pod-wide private **Labelers**); (4)
**zero-knowledge content discovery** (local AI ranks the firehose; no behavioral tracking leaves your
hardware); (5) **algorithmic forking/playbacks** (re-index a 4-hour window of the firehose with an
alternative ranking — your feed as a programmable archive).

**USER:** Explain how you'd see the offline-capable mesh integrating.

**GEMINI:** Elegant because **ATProto data is structured almost exactly like Git** — records are
self-certifying (signed on-device), so they don't need the internet to be valid, just a path to another
device. **4-phase lifecycle:** (1) *Local discovery* — shift to BLE/Wi-Fi-Direct/802.11s; cache the
group's DID docs + public keys locally; peer handshake verifies signatures offline. (2) *MST "Git
merge"* — exchange repo revision numbers (TIDs), compute the diff, swap just the missing record ops as
a compressed **CAR** file (each block signed → self-authenticating). (3) *Micro-AppView* — a local
SQLite AppView ingests the CAR files and regenerates the timeline offline. (4) *Re-federation* — when a
node hits coverage, it pushes the cumulative signed CAR to the public PDS → Relay; public updates appear
globally in order, internal lexicons stay isolated.

**USER:** Are there ready-to-hack performant/flexible AppView servers to jumpstart this?

**GEMINI:** Three: (1) **AppViewLite** (`alnkesq/AppViewLite`, C#/.NET) — lean/low-resource, ~**2.2 GB
disk/day** to index the firehose, memory-mapped columnar storage, connects to Jetstream/Relays — best
for the solo hobbyist / injecting shadow-ban logic. (2) **Blacksky AppView** (`blacksky-algorithms/…`) —
TS bsky appview + a Rust indexer **rsky-wintermute**, splits live indexing from backfill, **~10,000+
records/sec**, has scaffolding for **"Private Community Posts"** on the AppView layer — best for an
org. (3) **zeppelin-social/bluesky-appview** — Docker-Compose full stack (bsky, bsky-indexer,
labelmuncher, palomar search, postgres/redis/opensearch) — best for a full reference clone + custom
proxy middleware.

**USER:** Could I build an internal 2010-Facebook-like interface using a lexicon and my own AppView?

**GEMINI:** Absolutely — a case study for what ATProto was designed to do. Define `com.yourdomain.fb2010.*`:
status (`…status`), mutual friends (`…graph.friend`), photo albums (`…media.album`), the Poke
(`…action.poke`). **The Wall hurdle:** in ATProto you can only write to *your own* repo, so Bob writes
`{ $type:"…wall.post", target:"did:plc:alice", text:"Happy Birthday!" }` to *his* PDS; your AppView
indexes it, notes the target, and serves all records where `target == did:plc:alice` on Alice's Wall.
**Mutual friends:** AppView only indexes a friendship if *both* `graph.friend` records exist
(two-way handshake); delete one and it tears down. **Albums:** an `album` record + `photo` records
referencing the album's AT-URI with an `imageBlob`. The PDS is unchanged; you just build the iconic
2010 blue-and-white frontend.
```json
// com.yourdomain.fb2010.media.photo
{ "lexicon": 1, "id": "com.yourdomain.fb2010.media.photo", "type": "record", "key": "tid",
  "record": { "type": "object", "required": ["albumUri","imageBlob","createdAt"],
    "properties": {
      "albumUri": { "type": "string", "format": "at-uri" },
      "caption": { "type": "string", "maxLength": 300 },
      "imageBlob": { "type": "blob", "accept": ["image/jpeg","image/png"] },
      "createdAt": { "type": "string", "format": "date-time" } } } }
```

### Fork/continuation — net-new tail (this is where the re-run diverged)

**USER:** Is there an open-source Bluesky client that would be a good fit to work from as an upstream
to extend?

**GEMINI:** The official `bluesky-social/social-app` is a poor base (dense, app.bsky-coupled mobile
microblogging code). Better starting points: (1) **Ouranos** (`pdelfan/ouranos`, **Next.js** web
client) — clean React/Tailwind, easy to reshape into a 2010-FB multi-column layout, inject custom
AppView endpoints/lexicons. (2) **Heron** (`tunjid/heron`, **Kotlin Multiplatform + Jetpack Compose**)
— explicitly **offline-first** (Room/SQLite local cache, a local WriteQueue) — the perfect sandbox to
hook BLE/Wi-Fi-Direct mesh transport into its local repository layer. (3) **atcute** (`mary-ext/atcute`)
— ultra-lightweight modular TS protocol utilities (signing/serialization/XRPC, zero UI assumptions) —
a type-safe foundation to build from scratch.

**USER:** Tell me why Twitter Circles was rolled out and back so quickly.

**GEMINI:** A masterclass in how hard private spaces are on a public broadcast network. **Launched
globally ~Aug 2022**, killed **Oct 31, 2023.** *Why out:* chase Instagram "Close Friends" — an inner
circle of up to **150**, lowering the stakes of posting. *Why back so fast:* (1) the **April 2023
containment breach** — private Circle tweets leaked onto strangers' public "For You" timelines;
acknowledged as a "security incident" — fatal for a privacy feature. (2) **skeleton-crew engineering**
post-Musk-acquisition layoffs (~70-80%) — maintaining a segregated privacy layer over a fast-moving
public firehose needs continuous maintenance. (3) **strategic shift** to Communities + Group DMs (a
binary: fully public, or fully off-grid encrypted) — Circles sat in a structurally fragile middle.

**USER:** Compare Circles and Communities and Group DMs. … Current status of each?

**GEMINI:** **Circles** — *outbound identity shield* (broadcast from your profile with a gating clause;
single DB serving two masters → leaked when ranking logic changed); **dead, sunset Oct 31, 2023.**
**Communities** — *forum sandbox* (Reddit/FB-Groups-like topical hub, separate public indexer);
**discontinued ~May 2026** (head of product: used by **<0.4% of users yet ~80% of spam reports/scams/
malware**; discovery shifted to Grok "Custom Timelines"). **Group DMs / XChat** — *sovereign black box*
(off-timeline, off-indexer; **E2EE**, disappearing messages, screenshot-blocking, public **join
links**; limits raised toward **500 → 1,000** members); **the central focus / replacement for
Communities.** The lesson: a massive public square forces a binary — fully public, or fully isolated/
encrypted; the semi-private middle (Circles/Communities) is fragile and gets abandoned.

---

## Part 5 — Bluesky's VC funding + Aggregation Theory (strategic background)

**USER:** Why did Bluesky take VC money? Won't that probably backfire into enshittification?

**GEMINI:** Bluesky raised a **~$15M Series A (Blockchain Capital, Oct 2024)** and a **~$100M Series B
(Bain Capital Crypto** — closed Apr 2025, disclosed Mar 2026**)**, scaling past **~43M users** /
~**20B+** public records. *Why:* hyperscale social infra (Trust&Safety/Ozone, the global Relay) is
capital-intensive; donations/subscriptions can't fund it fast enough. *The structural shield (Cory
Doctorow's enshittification thesis):* enshittification relies on **high switching costs** — and on
ATProto Bluesky **doesn't own the hostage**. As a PBC built on an open protocol, if the flagship app
degrades, a user moves identity+history+followers to another PDS/AppView in minutes. Because switching
cost ≈ zero, aggressive enshittification triggers a fast exodus — the VCs are "trapped by the math."

**USER:** Why would a VC get involved that way? What's the expectation on both sides?

**GEMINI:** They're betting on **dominating the infrastructure layer of a new standard**, not capturing
users. Three theses: (1) **"Red Hat of social infra"** — the protocol is free, but operating Relays/
moderation/AppViews is complex; Bluesky becomes the enterprise-grade provider. (2) **Tollbooth/on-ramp
monopoly** — the flagship app is the on-ramp; monetize convenience (domains, tips/subscriptions cut,
premium video/storage/search). (3) **Aggregation Theory** — own the largest unified directory of
demand; developers plug into Bluesky's audience, making it the default marketplace. Expectations:
Bluesky gets a war chest to fight monopolies without desperate ad-surveillance; the VCs buy early
equity in the pole-position company on an inevitable open web (no token/NFT hyper-financialization
promised).

**USER:** Tell me more about Aggregation Theory — references, quotes, links, examples.

**GEMINI:** Coined by **Ben Thompson** on **Stratechery (2015)**. Pre-internet, winners controlled
**supply** (oil, presses, broadcast towers); online, with distribution/marginal/transaction costs → 0,
winners control **demand** via the best user experience. *"Zero distribution costs. Zero marginal
costs. Zero transactions. This is what the Internet enables, and it is completely transforming not just
technology companies but companies in every single industry."* — Thompson, Aggregation Theory. The
flywheel: superior UX → mass of users → suppliers forced to commoditize/modularize on the aggregator's
terms. Follow-up **"Defining Aggregators"** (2017) — three levels: **L1 supply acquisition** (Netflix —
pays for supply), **L2 transaction costs** (Uber/Airbnb — doesn't own supply, has onboarding friction),
**L3 zero supply costs** (Google/Meta — suppliers self-optimize via SEO, near-infinite margins).
Examples: NYT→Google/Meta; Hilton→Airbnb; taxis→Uber; Comcast→YouTube/TikTok. The endgame is monopoly;
the only counters are a new interface paradigm (conversational AI) or **decentralized protocols dropping
switching costs to zero** so users move demand freely — *which is the Croft/ATProto bet.* [src: stratechery.com]
