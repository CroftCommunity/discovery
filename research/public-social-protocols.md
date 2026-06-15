# Comparative Analysis of Public Social Protocols and Products

**Subject:** Twitter/X, Bluesky/atproto, Threads, Mastodon/ActivityPub, Pixelfed

**Purpose:** Assess what integrating our public-social side with Bluesky / AT Protocol concretely gives us, costs us, and aligns with, relative to the ActivityPub and proprietary alternatives.

**Date:** 2026-06-13

**Status:** Research deliverable. No application code. Protocol-level facts and product-level facts are kept distinct throughout. Items not verifiable against current sources are marked `[UNVERIFIED]`.

---

## Executive summary

We are building an encrypted local-first messaging and social architecture in Rust. The public side integrates with Bluesky / atproto using custom Lexicon-defined record types. The private side runs group messaging over a separate encrypted P2P path (iroh + MLS + Automerge), with identity for both sides keyed to atproto DIDs.

The central finding is that the strongest single justification for the Bluesky / atproto choice holds up under scrutiny, and it is the **dual-use of identity**. An atproto DID is a portable, self-authenticating, host-independent cryptographic identifier. It can serve as the same identity primitive on the public side (atproto-native) and on the private side (as a stable handle to which MLS credentials and group membership can be bound). ActivityPub's identity model could not serve that dual role nearly as cleanly, because ActivityPub identity is instance-bound (`@user@instance`), the cryptographic keys are server-held rather than user-held, and account migration does not move content or preserve identifiers. This is examined in detail below, and it is the anchor reason.

The honest counterweights are real and should not be minimized. ActivityPub has the longer and broader track record of diverse interoperating application types (Mastodon, Pixelfed, PeerTube, and more), which is precisely the pattern we are attempting with custom Lexicons. The fediverse is more mature as a federation. And atproto's "decentralization in practice" still traces back heavily to Bluesky-the-company, which runs the default PDS, a major relay, the default AppView, and the reference labeler. atproto is also young, with its IETF standardization only beginning in early 2026.

The "everything public" property of atproto is the structural fact that makes our public/private split necessary rather than optional. Today, all atproto repo data is public by design. That is why nothing private can live on-protocol and why we need the separate encrypted P2P path. Notably, atproto's own roadmap now includes "Permissioned Data" work, which would change this calculus over time and is the most important development to track.

---

## Per-platform / protocol investigation

### Twitter / X

**Layer:** Product only. No open protocol.

X is centralized and proprietary. There is no open protocol beneath it, no self-hosting, and no ability for third parties to run alternative servers or build alternative full clients on the underlying network. It remains the incumbent baseline for large-public-social UX and network effects, on the order of 500+ million users, far larger than any of the open-protocol networks discussed here. `[UNVERIFIED]` exact current X user count; figures are disputed and not independently auditable since the platform went private.

For our purposes X is the "proprietary path" comparison column. It offers reach and nothing else relevant to our architecture: no portable identity, no user-owned data, no custom record types, no ability to run our own infrastructure, no protocol to build on. The API has also become costly and restrictive since 2023.

### Bluesky (product) on AT Protocol / atproto (protocol)

**Layer distinction:** Bluesky is one product (one AppView plus a default PDS and relay). atproto is the protocol others can build on. Holding this distinction is essential because much of the value to us is at the protocol layer, not the Bluesky-app layer.

**Architecture.** atproto separates concerns into distinct components rather than federating monolithic servers. The model, confirmed across multiple current sources:

- A **PDS (Personal Data Server)** holds a user's repository of signed records (posts, likes, follows, profile). The PDS is the sole write source; clients never write directly to a relay or AppView. The PDS can be hosted by a third party or self-hosted, and every record is cryptographically signed by the user's DID.

- A **Relay** polls PDSes across the network and aggregates their changes into a single event stream, the **firehose**. It is a fan-in becoming a fan-out: many per-user PDS streams transformed into one network-wide stream.

- An **AppView** consumes the firehose, indexes records into its own database, and serves the user-facing product experience (timelines, search, follower lists). Read requests are proxied through the user's PDS to the AppView.

- **Lexicons** are formal schemas defining record types and API methods, ensuring consistent data structures across independent implementations.

- **Labelers** (e.g. Bluesky's Ozone) apply moderation labels as a composable layer on top, rather than as a single gatekeeping authority.

All of these components are available as open-source software, and Bluesky itself runs almost entirely on these open-source components.

**Who can build.** Anyone can run a PDS, relay, AppView, or labeler, and anyone can define new Lexicons and build alternative clients and apps. Alternative full stacks already exist in practice, for example Blacksky (Rudy Fraser / Blacksky Algorithms), which runs an alternative implementation with its own application, PDS, and relay infrastructure.

**Identity.** Users are identified by a **DID** (decentralized identifier), with `did:plc` as the default method and `did:web` as a domain-linked option. The DID is permanent and host-independent: it remains static even if the user migrates their entire account to a new server. A human-readable handle (often a domain) maps to the DID and is changeable without changing the underlying identity. This is the property our architecture leans on.

**Data ownership / portability.** The user's content lives in their PDS repo as signed records. Because records are signed by the DID and the DID is portable, a user can in principle move their PDS to a different host without losing identity or social graph. The practical caveat, current as of 2026: migration off Bluesky's PDS works, but you cannot currently migrate back onto Bluesky's PDS. So portability is real but not yet fully symmetric in the reference deployment.

**Scale.** Bluesky reported roughly 40 million registered users crossing in October 2025, and sources through April–May 2026 put registered users in the ~42–43.5 million range. Daily active users are estimated in the ~3.5–4.5 million range (roughly 8–11% of registered users), so it is a mid-sized, engaged, but not mass-scale network. It raised a $100M Series B at a reported ~$700M valuation. `[UNVERIFIED]` the various third-party DAU figures differ; treat the range, not any single number, as the signal.

**"Everything public."** All atproto repo data is public by design today. This is a protocol-level fact, not a Bluesky-app choice, and it is central to our split (see synthesis). The roadmap "Permissioned Data" effort (Blacksky, Northsky, Habitat, plus a Bluesky sketch proposal) is actively working to add non-public data to the protocol, but it is explicitly early and unfinalized as of spring 2026.

**Standardization / governance risk.** atproto is young. The IETF chartered an Authenticated Transfer working group in early 2026, beginning a multi-year shift from Bluesky's implementation toward independent specs. Bluesky, Inc. is a public benefit corporation that today operates the reference implementations and most default infrastructure users rely on (default PDS, a major relay, the default AppView, the Ozone labeler). Anyone can run alternatives, but switching away from Bluesky's defaults takes real effort. The centralization risk therefore still traces back substantially to one company.

### Threads (Meta) on ActivityPub (partial federation)

**Layer:** Centralized proprietary product bolting on an open protocol. A hybrid, instructive case.

Threads is a centralized Meta product that has added ActivityPub federation as an opt-in, explicitly beta, and still-partial capability. Current state, verified:

- Federation is **opt-in per account** and must be explicitly enabled, which limits how many Threads accounts actually federate.

- Federation has been substantially **one-way / asymmetric**. A Mastodon user can follow a federating Threads user and see their posts, but the Threads-side experience of inbound fediverse interaction is limited. Reports indicate likes federate into the Threads UI with limited detail, while replies and boosts from the fediverse have not been surfaced as full native interactions, and Threads users have not been able to follow arbitrary fediverse accounts in the way fediverse users can follow them. `[UNVERIFIED]` the exact current matrix of which interactions federate both ways as of mid-2026; Meta has described two-way communication as a work in progress.

- Meta added a separate **Fediverse feed** (around summer 2025), a chronological feed of federated content kept deliberately separate from the AI-ranked main "For You" feed. Fediverse posts are not blended into the main algorithm by design.

- Moderation interop (handling of Flag, Block, Reject activities across the boundary) has been a standing open concern in the fediverse community.

Threads is enormous relative to the open networks (Meta has cited figures in the hundreds of millions of monthly users for Threads). But the federated surface of Threads is far smaller than its raw user count, because federation is opt-in and partial. For our analysis, Threads is the cautionary example of "walled garden bolts on open protocol": the protocol connection exists but the product retains central control and gates the interop.

### Mastodon on ActivityPub

**Layer distinction:** ActivityPub is the W3C-standardized protocol; Mastodon is the reference microblogging implementation. ActivityPub ≠ Mastodon.

**Architecture.** ActivityPub federates monolithic servers (instances). Each instance hosts its users and their content. Federation uses a server-to-server model built on an **inbox/outbox** pattern over ActivityStreams 2.0 / JSON-LD vocabulary, with Webfinger for discovering user profiles across instances. Servers relay activities (follows, boosts, replies, etc.) between instances so content is visible across the fediverse. It prioritizes simplicity, instance autonomy, and community governance, trusting instance admins to make decisions for their communities.

**Identity.** Users are identified as `@user@instance`. Identity is **instance-bound**: the identifier encodes the home server. There is no host-independent cryptographic identifier equivalent to a DID. Account keys are held by the instance, not the user.

**Migration / portability.** This is ActivityPub's weakest axis and a key contrast with atproto. Per current Mastodon documentation:

- Moving an account moves **followers** (via the ActivityStreams `Move` activity, where supported), and lets you import follows, blocks, mutes, and bookmarks from exported CSV files.

- It does **not** move your posts or media. Mastodon does not support importing posts, "due to technical limitations." A downloadable archive exists but is for viewing, not for re-homing content.

- The underlying reason is structural: each post has a unique ID encoding the origin URL, referenced by every reply, and posts are cryptographically signed by a key that is never exported. So posts are inherently locked to the original profile on the original domain.

- There are cooldowns (a 30-day migration cooldown; archive requests once per 7 days), and the `Move` operation is not part of core ActivityPub but an ActivityStreams convention with non-uniform implementation across servers.

A future "LOLA" protocol effort aims to improve content migration, and end-to-end encryption for ActivityPub has been discussed, but both are prospective. `[UNVERIFIED]` current delivery status of LOLA and AP E2EE.

**Visibility model.** Importantly, ActivityPub is **not** strictly all-public. It has addressing semantics (to/cc, followers-only, direct) that support non-public posts. Mastodon's "private mentions" are not end-to-end encrypted (instance admins can read them), so this is access-control privacy, not cryptographic privacy. But the existence of non-public addressing is a genuine difference from atproto's all-public model, and is addressed in the synthesis.

**Scale.** The non-Threads ActivityPub fediverse is commonly estimated around ~1 million+ monthly active users, with one widely cited 2026 prediction putting registered fediverse accounts (excluding Threads) crossing ~15 million with ~2–3 million monthly active. `[UNVERIFIED]` precise current fediverse MAU; the network is fragmented across thousands of instances, making exact counts hard.

### Pixelfed on ActivityPub

**Layer:** A distinct product (photo sharing, Instagram-like) on the same ActivityPub protocol as Mastodon, fully interoperable with it.

Pixelfed is the instructive "same protocol, different product vertical, still interoperable" case. It is a separate application serving a different use case (image-centric sharing) yet federates with Mastodon and the rest of the fediverse via ActivityPub. Alongside PeerTube (video) and others, it demonstrates that ActivityPub supports a genuinely diverse ecosystem of non-microblog app types that interoperate across a shared network. This is the most important real-world evidence about protocol extensibility, and it favors ActivityPub on track record (see the Pixelfed lesson in the synthesis).

Notably, the Bounce cross-protocol migration tool supports moving a Bluesky account to Pixelfed (as well as Mastodon), which shows Pixelfed sitting squarely inside the interoperable ActivityPub world.

---

## Table 1 — Protocol / platform comparison

Reading note: "protocol" rows describe the underlying protocol; "product" rows describe the named app.

| Dimension | Twitter / X | Bluesky / atproto | Threads | Mastodon / ActivityPub | Pixelfed |
|---|---|---|---|---|---|
| **Open protocol?** | No. Proprietary | Yes. atproto, IETF WG started early 2026 | Product proprietary; uses ActivityPub for federation | Yes. ActivityPub, W3C standard | Yes. ActivityPub |
| **Centralized / federated / decentralized** | Centralized | Decentralized-by-design (PDS/relay/AppView split), centralized-in-practice via Bluesky defaults | Centralized product, partial federation | Federated instances | Federated instances |
| **Self-hostable** | No | Yes (PDS, relay, AppView, labeler) | No | Yes (instance) | Yes (instance) |
| **Who can build clients/apps/views** | Effectively no one (restrictive API) | Anyone (new Lexicons, AppViews, clients) | No (Meta-controlled) | Anyone (AP-compliant apps) | Anyone (AP-compliant apps) |
| **Identity model** | Platform account, no portable ID | DID (`did:plc`/`did:web`), host-independent, user-signed | Meta account; federated actor when enabled | `@user@instance`, instance-bound, server-held keys | `@user@instance`, instance-bound |
| **Identity portable across providers** | No | Yes (DID stays constant across PDS moves) | No | Limited (profile/followers move; identifier changes) | Limited (same as Mastodon) |
| **Where content lives** | Meta/X servers | User's PDS repo (signed records) | Meta servers | Origin instance | Origin instance |
| **Content portability** | None | High (repo is portable; off-Bluesky works, back-to-Bluesky not yet) | None | Low (posts/media do not migrate; followers do) | Low (same as Mastodon) |
| **Public vs scoped visibility** | Platform-controlled | All-public by design today (Permissioned Data on roadmap) | Public posts federate (opt-in) | Mixed: public + non-public addressing (not E2EE) | Mixed (as AP) |
| **Content typing / extensibility** | N/A (closed) | Lexicons (formal schemas; new record types straightforward) | N/A (uses AP vocab via Meta) | ActivityStreams / JSON-LD vocabulary; extensible but loosely | ActivityStreams (image-focused) |
| **Interop / reach** | Island (no protocol interop) | atproto network; bridges to AP via Bridgy Fed (opt-in) | Federates to AP (opt-in, partial); huge native base | Full fediverse interop | Full fediverse interop |
| **Approx scale** | 500M+ (disputed) | ~42–43.5M registered, ~3.5–4.5M DAU | Hundreds of millions MAU; far smaller federated surface | ~1M+ MAU fediverse-wide (fragmented) | Subset of fediverse |
| **Moderation / governance** | Central (company) | Composable labelers (e.g. Ozone) atop infra | Central (Meta) | Per-instance admin | Per-instance admin |
| **Builder effort** | High barrier, little payoff | Moderate (run a PDS/AppView; good SDKs emerging) | N/A | Moderate (run an instance) | Moderate (run an instance) |

`[UNVERIFIED]` markers from the per-platform sections apply to the scale row and to the exact Threads two-way interaction matrix.

---

## Table 2 — What Bluesky / atproto integration gives *us*

Rows are capabilities that matter to our stack. This table is the heart of the deliverable.

| Capability for our stack | What atproto gives us | What ActivityPub would have given us instead | What a proprietary / X path would have given us | Net assessment for our use case |
|---|---|---|---|---|
| **Portable identity that can double as our private-side DID** | A real DID (`did:plc`/`did:web`): host-independent, user-signed, stable across moves. Usable directly as the identity primitive on both public and private sides | `@user@instance` identity, instance-bound, server-held keys, not a DID. Would need a parallel/external identity system for the private side | A platform account, not usable off-platform at all | **Decisive win for atproto.** This is the anchor reason. The same DID keys public records and private MLS membership |
| **User-owned data via PDS** | Content lives in a user-controlled, portable, signed repo | Content lives on the origin instance; posts do not migrate | No user ownership | **Strong win for atproto** |
| **Custom record types for a non-microblog app** | Lexicons: define our own record schemas as first-class, consistently validated across implementations | Extensible via ActivityStreams/JSON-LD, and proven across diverse apps (Pixelfed/PeerTube), but typing is looser and convention-driven | Not possible | **Win for atproto on schema rigor; ActivityPub wins on demonstrated diversity.** See Pixelfed lesson |
| **Public/private split compatibility** | Clean: atproto is all-public, so the boundary is unambiguous (public → atproto, private → our encrypted path). Permissioned Data may extend this later | AP has non-public addressing (not E2EE), which blurs rather than cleanly draws the boundary, and still would not give us cryptographic privacy | Closed; no split possible | **Win for atproto** for a clean architectural boundary; nuance below |
| **Ability to run our own AppView** | Yes: index the firehose, serve our own product view of our Lexicon records | Analogous via running an instance, but instances are monolithic rather than a separable read/index layer | No | **Slight edge atproto** (separation of concerns fits a custom app) |
| **Network reach** | ~42–43.5M registered, growing, engaged. Bridgeable to fediverse (opt-in) | ~1M+ MAU fediverse, more fragmented, but longer tail of app types; Threads' large base only partially reachable | 500M+, but unreachable as a protocol | **Mixed.** X has reach we cannot use; atproto and AP are comparable-order open networks |
| **Moderation composability** | Composable labelers; we can choose/run labelers | Per-instance admin control | Central company control | **Win for atproto** for a builder who wants choice |
| **Builder effort** | Moderate; reference stack open-source, SDKs maturing, test suites in progress | Moderate; very mature, many libraries, long track record | N/A | **Edge to ActivityPub on maturity; atproto adequate and improving** |

---

## Synthesis — alignment and contrast with our stack

### Alignment wins

**Identity (the anchor).** Our architecture depends on one identity serving two roles: the public-social identity (atproto-native) and the private-encrypted-messaging identity (MLS credentials and group membership keyed to that same identity). atproto's DID serves this directly. The DID is host-independent, user-signed, and stable across provider moves, so it is a sound root to which we can bind MLS credentials and to which group membership can be anchored, while the same DID names the user on the public side.

ActivityPub could **not** have served this dual role nearly as cleanly, for concrete reasons rather than aesthetic ones:

- ActivityPub identity is `@user@instance`, bound to a home server. There is no host-independent cryptographic identifier we could reuse as a private-side root.

- The keys backing an ActivityPub actor are held by the instance, not the user. We need a user-held key as the basis for a private encrypted side; a server-held key is the wrong custody model and is not exportable.

- ActivityPub account migration does not move content and changes the practical identity binding (followers move via `Move`, posts do not, identifiers are origin-locked). An identity primitive that breaks on migration is a poor root for long-lived private group membership.

So on ActivityPub we would almost certainly have had to introduce a *separate* identity system for the private side and then map between it and the AP actor, which is exactly the clumsiness we avoid by using the DID for both. This holds up under scrutiny and is the strongest single argument for the Bluesky choice.

**Data ownership.** atproto's "your PDS holds your signed repo" maps well onto a user-sovereignty stance. ActivityPub's origin-instance custody and non-portable posts do not.

**Clean public/private boundary.** Because atproto is all-public today, the boundary is unambiguous and easy to reason about and audit.

### Capability contrasts (where atproto is weaker for us)

- **Network maturity and app-type diversity.** The fediverse has a longer, broader track record of diverse interoperating app types. That is a real strength of ActivityPub and directly relevant to what we are attempting.

- **Decentralization in practice.** atproto's reference infrastructure still centers on Bluesky-the-company. Running independent infrastructure is possible but non-trivial, and the firehose/relay model favors a small number of large relays in practice.

- **Youth and flux.** atproto's IETF standardization only began in early 2026, the AppView reference implementation has been mid-refactor, and "Permissioned Data" is unfinalized. We are building on a moving target.

- **The all-public property cuts both ways.** It gives us a clean boundary, but it also means *nothing* private can ever live on-protocol today. That is the reason we need the separate encrypted P2P path at all. Framed correctly, this is context, not a defect: we were always going to keep private messaging off any public social protocol.

### The "everything public" property

atproto data is public by design. Every record in a PDS repo is signed and intended to be served openly through relays and AppViews. Therefore our split is necessary, not stylistic:

- **Public records → atproto.** Anything intended to be publicly visible becomes a Lexicon-typed record in the user's repo.

- **Private messaging → our encrypted P2P path.** iroh transport, MLS for group key agreement and membership, Automerge for state. None of this touches the public protocol.

- **Identity → shared DID across both.** The one element that legitimately spans the boundary.

On **ActivityPub** the calculus is slightly different but does not change the conclusion. ActivityPub has non-public addressing (followers-only, direct), so it is not strictly all-public the way atproto is. But that non-public addressing is access-control privacy enforced by instance admins, not end-to-end encryption (Mastodon explicitly warns that private mentions are readable by instance staff). So even on ActivityPub we would still need our own encrypted path for genuine private messaging. AP's non-public addressing would have given us a slightly fuzzier boundary to reason about, not a way to avoid the separate private path. atproto's bright-line all-public model is arguably cleaner for us precisely because it forces the separation to be explicit.

One forward-looking caveat: atproto's roadmap "Permissioned Data" work could eventually allow non-public data on-protocol. If it matures, it could narrow the gap our private path fills, or it could become a complementary mechanism. It is the single most important thing to track, because it touches the core assumption behind our split.

### The Pixelfed lesson

Pixelfed is a distinct, non-microblog product (photos) thriving on a shared open protocol and interoperating across it with Mastodon and the wider fediverse. That is a direct parallel to what we are attempting on atproto: a distinct application using custom Lexicons on a shared network.

The honest assessment: **ActivityPub has the stronger demonstrated track record for diverse app types.** Pixelfed, PeerTube, and others are existence proofs of varied verticals interoperating over ActivityPub for years. atproto's extensibility mechanism (Lexicons) is arguably *cleaner* for defining a brand-new record type with rigorous schema validation, and non-microblog atproto apps are emerging. But "cleaner mechanism" is a design argument, while Pixelfed is empirical evidence. We should hold both honestly: Lexicons give us better-specified custom records; ActivityPub has proven that diverse apps can coexist and interoperate at scale over years. For a custom-app builder, atproto's schema rigor is attractive, but we are betting partly on a maturing ecosystem rather than a fully proven one.

A relevant nuance: interoperation on atproto means other AppViews can choose to index and understand our Lexicon records, but they are not obligated to, and unknown record types are simply not surfaced by AppViews that do not implement them. The same is true on ActivityPub for unknown activity types. Neither protocol gives automatic cross-app semantic understanding; both require the *other* app to implement support for your types. So the Pixelfed-style interop is achievable on atproto but, like on ActivityPub, depends on other builders adopting the schema.

### Risks and unknowns

- **Dependence on Bluesky-the-company.** Default PDS, a major relay, the default AppView, and the Ozone labeler are Bluesky-operated. If we rely on those defaults, we inherit that dependency; if we run our own, we take on operational cost.

- **Protocol youth.** IETF work began in early 2026, specs and test suites are still being written, and reference components have been in active refactor.

- **Portability asymmetry today.** Migration off Bluesky's PDS works; migrating back does not yet. Cross-protocol migration (Bounce) is currently Bluesky→Mastodon/Pixelfed only, for the same reason.

- **Scale ceiling.** ~3.5–4.5M DAU is engaged but not mass-market. If reach is a hard requirement, neither atproto nor the fediverse rivals X or Threads in raw size, and X/Threads are not usable as open protocols.

- **Permissioned Data is unsettled.** Its eventual shape could materially affect where our public/private boundary sits.

- **Bridging is opt-in and partial.** Bridgy Fed connects atproto and ActivityPub, but opt-in on both sides, and prominent fediverse figures have opted out. Cross-protocol reach is real but should not be assumed as default-on.

---

## Decision-support summary

**Strongest reasons the Bluesky / atproto integration serves our stack (prioritized):**

1. **Dual-use identity.** The atproto DID uniquely serves as the same host-independent, user-held identity primitive on both the public side and the private MLS side. ActivityPub's instance-bound, server-held, migration-fragile identity could not do this without a separate parallel identity system. This is the anchor reason and it survives scrutiny.

2. **User-owned, portable data** via the signed PDS repo, versus ActivityPub's origin-locked, non-migratable posts.

3. **Clean public/private boundary** created by atproto's all-public model, which makes the architecture easy to reason about and audit.

4. **Rigorous custom record types** via Lexicons, well-suited to a non-microblog app.

5. **Composable moderation** and the ability to run our own AppView.

**Most significant things we give up versus alternatives:**

1. **Proven app-type diversity.** ActivityPub's track record (Pixelfed, PeerTube) for varied interoperating apps is stronger than atproto's still-emerging one.

2. **Federation maturity** and a longer-standing standard (W3C since well before atproto's 2026 IETF start).

3. **Reach**, in absolute terms, versus X and Threads, neither of which is usable as an open protocol anyway.

4. **Independence from a single steward.** atproto's practical centralization in Bluesky, Inc. is a real dependency that the fediverse's fragmentation, for all its downsides, avoids.

**Open questions that could change the assessment:**

1. How does atproto **Permissioned Data** land, and does it narrow or complement the role of our private path?

2. Does atproto's **decentralization in practice** improve as IETF standardization and independent infrastructure mature, reducing Bluesky-company dependence?

3. Does the **non-microblog atproto ecosystem** reach Pixelfed-like maturity, validating the custom-Lexicon bet empirically rather than only by design?

4. Does **back-migration to Bluesky's PDS** and **symmetric cross-protocol migration** ship, closing the current portability asymmetry?

The integration is defensible on genuine merits, with identity as the load-bearing justification. The honest costs are ecosystem maturity and single-steward dependence, neither of which undercuts the identity argument but both of which deserve monitoring.

---

## Sources

Protocol architecture and roadmap

- AT Protocol Roadmap (Spring 2026), atproto.com/blog/2026-spring-roadmap — Permissioned Data work (Blacksky, Northsky, Habitat), IETF ATP working group, PDS-hosted accounts shared across apps.

- "What Is the AT Protocol? A Developer's Mental Model," Jeff Bailey (May 2026) — PDS as sole write source, relay fan-in/fan-out, IETF WG early 2026, Bluesky, Inc. operating default infrastructure.

- "What is AT Protocol atproto," James Dumar (Mar 2026) — DID portability (`did:plc`/`did:web`), PDS migration without losing followers.

- bitcrowd blog (Mar 2026) and atproto.brussels architecture summary — firehose, AppView dataplane, open-source components.

- Blacksky (Wikipedia) — alternative atproto stack (app, PDS, relay).

ActivityPub vs atproto

- "ActivityPub vs. ATProtocol," Fediview (Apr 2026) — relay/AppView layering, Lexicon schemas, portability, differing protocol priorities.

- "ActivityPub Protocol: Understanding the Fediverse," dasroot.net (Apr 2026) — inbox/outbox, Webfinger, PeerTube/Pixelfed examples, Mastodon version note.

Scale

- Sprout Social, Backlinko, Proxidize, resourcera, limelightdigital (Jan–May 2026) — Bluesky ~40–43.5M registered, ~3.5–4.5M DAU, Series B ~$700M valuation.

- Tim Chambers 2026 open social web predictions (via news.hada.io) — fediverse (ex-Threads) ~15M registered / ~2–3M MAU estimate, Threads 500M+ MAU.

Threads federation

- anderegg.ca, "Poking at Threads in the Fediverse" — opt-in, one-way posting, limited like notifications, quote-post extension issue.

- recurpost.com (Apr 2026) — Fediverse feed (summer 2025) separate from AI feed.

- Social Media Today (Jun 2025) — expanded but still limited fediverse engagement.

- W3C public-swicg list (Jan 2024) — moderation activity (Flag/Block/Reject) concerns.

Bridging and migration

- Bridgy Fed: indieweb.org/Bridgy_Fed, github.com/snarfed/bridgy-fed, fed.brid.gy/docs, docs.bsky.app — atproto↔ActivityPub bridging, opt-in on both sides, Eugen Rochko opt-out (Jan 2026).

- TechCrunch (Jun 2024) — Bridgy Fed enabling Mastodon↔Bluesky interaction, opt-in decision.

- TechCrunch, "Bounce" — cross-protocol migration Bluesky→Mastodon/Pixelfed, one-directional due to no back-migration to Bluesky PDS.

- Mastodon documentation (docs.joinmastodon.org/user/moving), mastodon/documentation GitHub, Privacy Guides (Jul 2025), Steve Bate, fedimeister — posts/media do not migrate, followers do, structural reasons (origin-locked IDs, non-exported signing keys), LOLA and AP E2EE as prospective.
