# Drystone: The ATProto Ecosystem, Compared

`Status: survey, current as of mid-2026. The ATmosphere moves fast; treat maturity and roadmap claims as a snapshot, not a fixed state. Per-app facts are cited to primary or press sources inline and in Sources.`

`Scope: a comparison of consumer and infrastructure applications built on the AT Protocol, along the axes that matter for Drystone (public versus private data, access control versus confidentiality, data-model and storage locus, community and governance model), and an explicit positioning of Drystone against them, separating genuine novelty from alignment.`

`Companion to: ../impl/drystone-design/social-mapping.md (which maps the public content lexicon and group layer onto Drystone; this doc widens that to the whole ecosystem), ../impl/drystone-design/asset-keying.md, and ../impl/drystone-design/history-durability.md. Also companion to ../drystone-spec/part-1-reasoning-underpinnings.md and ../drystone-spec/part-2-certifiable-design.md.`

This is a landscape and positioning document, not a specification. It answers two questions: where does Drystone sit among what already exists on ATProto, and what in Drystone is genuinely new versus already being done. Normative keywords are not used here. Provenance flags: `Verified` (primary source, checked this cycle), `press` (reputable secondary reporting), `proposal` (a published but non-final design), `[confirm]` (worth re-checking as it evolves).

> **The framing to lead with: complement, not competitor.** Drystone is not a replacement for ATProto and is not competing with it. ATProto is built for public, broadcast, reach-oriented speech, and should remain that; Drystone is the private, member-scoped, governance-forward half ATProto has, by deliberate choice, left open. Its private-data direction is explicitly access control rather than confidentiality, leaving group-private end-to-end encryption to others. Everything below reads as Drystone filling a space left open on purpose, not contesting one already occupied. `Synthesis.`

> **The one-line finding.** The ecosystem has converged on two things Drystone also bets on, MLS for encryption and the reuse of the public content lexicon, and it is actively building the layer Drystone needs, permissioned and private data, but *no existing effort combines a durable social data model, cryptographic confidentiality, and peer-symmetric governance.* Germ has MLS confidentiality but only for ephemeral DMs; Bluesky's permissioned data has a durable model but is explicitly access-control, not confidentiality; the Arbiter has governance but an owner-authority model. Drystone's cell is empty. `Synthesis.`

> **The governing trust boundary: cryptographic trust for free, zero semantic trust.** The reason Drystone can ride atproto for the public path yet must own the private and governance layers reduces to one line: *atproto gives cryptographic trust (identity and integrity) for free, but zero semantic trust — so you must own your schema, threading, and moderation policy.* The cryptographic half is a live chain of custody, verifiable end to end with zero trust in the PDS or relay: a **DID → its signing key (from the DID document) → a signed commit → the MST root → the record CID → the record bytes**, each link checkable by a client. What atproto does *not* give is meaning: a PDS stores and serves records without validating unknown lexicons, so nothing at the substrate enforces that a record's *content* is well-formed, correctly threaded, or policy-compliant — that is the application's job. This is the same boundary as Drystone's own "compose MLS, own governance" stance, one plane up, and it is the governing principle for every public-spoke integration below: reuse the cryptographic guarantees, never assume semantic ones. The chain-of-custody was walked out live by the public-roundtrip proof (a verified DID→key→commit→MST→CID roundtrip, including the labeler model where labels are independently-signed, pull-only assertions). The self-certifying identity and transparency-log facts this rests on are carried in `cross-platform-identity-provenance.md`. `Synthesis`, grounded in the public-roundtrip proof (see reference-index).

---

## A. The axes that matter

Comparing ATProto apps on features misses the point; they mostly reuse the same content lexicon. The distinctions that matter for Drystone are structural:

- **Public versus private.** Does the app deal only in public broadcast data, or does it have a notion of non-public, audience-scoped data?

- **Access control versus confidentiality.** For the private ones, is privacy enforced by *who is allowed to read* (a trusted service checks, and can itself read) or by *encryption* (only key-holders can read, services cannot)? This is the sharpest axis, and the ecosystem is split on it.

- **Data model and storage locus.** Does everything live in the per-author PDS repo, or is there adjunct infrastructure for data that does not fit the repo shape (bulk data, encrypted data, CRDT state)?

- **Community and governance.** Is there a group or community primitive, and if so, is authority centralized in an owner, delegated by a service, or distributed among peers?

- **Identity.** Nearly all reuse the ATProto DID and handle, which is the ecosystem's genuine common foundation.

---

## B. The landscape, at a glance

`Verified`/`press` per the Sources section. "AC" = access control; "conf" = cryptographic confidentiality.

| App | Category | Public/Private | Privacy basis | Storage locus | Community/governance | Maturity |
|---|---|---|---|---|---|---|
| Bluesky (app.bsky) | Microblogging | Public | none (public) | PDS repo | none (follows/feeds) | mature |
| Frontpage | Link aggregator | Public | none | PDS repo, custom lexicon | none yet (single global feed) | early, active |
| WhiteWind | Longform blog | Public | none | PDS repo, custom lexicon | none | early |
| Leaflet | Docs/publishing | Public | none | PDS repo, block-composition | none | early, active |
| Standard.site / ATmosphere (Automattic) | Longform/CMS | Public | none | dual-write: bsky post + custom lexicon | none | new (2026) |
| Smoke Signal | Events/RSVP | Public | none | PDS repo, custom lexicon | none | early |
| recipe.exchange | Recipe sharing | Public | none | PDS repo, custom lexicon [confirm] | none | early, active |
| Tangled | Git collaboration | Public | none | knots (off-PDS git) + PDS pointers | per-repo collaborators; self-governance | alpha, active |
| Blacksky | Social infrastructure | Public (+ permissioned work) | AC (in progress) | own AppView/relay | community infra | active |
| Roomy (Muni Town) | Discord-like communities | Private-ish | encryption (experimental, leaks metadata) | CRDT (Loro/Automerge) + PDS identity | communities, channels; membership service | alpha |
| The Arbiter (Muni Town) | Membership service | Private | AC (space authority) | XRPC service + PDS | spaces, roles, space-delegation | alpha/directional |
| Germ | E2EE messaging | Private | confidentiality (MLS) | separate Germ layer; PDS identity only | none (DMs) | beta (iOS) |
| Bluesky Permissioned Data | Private-data protocol | Private | AC, explicitly not confidentiality | distinct protocol, own repo/sync | spaces, space authority | proposal |
| Peergos | E2EE PDS | Private | confidentiality (E2EE) | private blockstore (CHAMP) | writing spaces | deployed, standardization sought |
| **Drystone** | **Confidential social substrate** | **Both, hard-separated** | **confidentiality (MLS) + governance** | **MLS + Willow chains; content-blind durability tier** | **groups; peer-symmetric k-of-n fold** | **design** |

---

## C. Profiles, by category

**Content-plane apps (public, reuse-or-custom lexicon).** Frontpage is a Hacker-News-style link aggregator on a custom lexicon (post, comment, vote) with a single global feed and no groups yet (`../impl/drystone-design/social-mapping.md` §K). WhiteWind is longform blogging on `com.whtwnd.blog.*`. Leaflet is a documents-and-publishing app whose novelty is block-composition, where each block in a document may itself be a different lexicon (for example an embedded Smoke Signal RSVP). Automattic's ATmosphere WordPress plugin (May 2026) uses a telling dual-write pattern: it publishes a standard `app.bsky.feed.post` for Bluesky visibility and stores the full article as a Standard.site lexicon record so other apps can render the long form independently. Smoke Signal handles events and RSVPs (the deep dive, founding motivation, architecture, its two-generation storage-path migration, and its Rust/Postgres stack, lives in `atproto-nsid-and-lexicon-mechanics.md`). recipe.exchange is a community recipe-sharing app (265 community recipes as of 2026-07-08) that reuses the ATProto identity and Bluesky's image CDN (recipes carry `bsky.app` creator profiles and `cdn.bsky.app` images) and offers rich filtering by ingredient, cuisine, cooking method, and dietary constraint; its recipes are custom-lexicon records in the per-author PDS repo (`[confirm]` on the exact lexicon and storage locus, inferred from the identity-plus-CDN reuse pattern), making it another instance of the content-plane-first, custom-lexicon-on-public-identity practice. `Verified` (live site, retrieved 2026-07-08) for the product and identity-reuse facts. All are public, and their lesson for Drystone is that the content plane is tractable and the reuse-plus-custom-lexicon split is standard practice.

**Infrastructure-novel.** Tangled is a git-collaboration platform whose key idea is the "knot," a lean headless server that lets users host git repositories, suitable for a personal Raspberry Pi setup or a larger community server, with the AppView acting as a unifying view over repositories hosted across different knots. It adds "spindles" (self-hosted, Nix-powered CI runners) and assigns each repository a DID for stability across renames, while issues, pull requests, and collaborators live as ATProto records (`sh.tangled.*`). The architectural lesson for Drystone is direct: git repos do not fit the per-author-record shape, so Tangled puts the bulk data on adjunct servers (knots) and keeps only pointers and collaborative metadata in the repo. Blacksky is a Black-community social infrastructure project running its own AppView and relay, and one of the teams building permissioned data.

**Community and CRDT.** Roomy (Muni Town) is a Discord-like communities app that uses ATProto for social discovery and a CRDT for peer-to-peer sync, is in alpha, and encrypts messages, though it is experimental and unaudited and leaks some non-encrypted metadata showing who you talked to but not the contents. Its communities have channels and categories, but its wiki pages are last-writer-wins rather than real-time collaborative (`../impl/drystone-design/social-mapping.md` §K). The Arbiter (Muni Town) is a proposed standardized XRPC group-and-membership service with access levels and space-delegation (a space can be a member of another space), designed to feed Bluesky's permissioned-data direction, and is the closest existing thing to Drystone's governance layer, though it uses an owner-authority model rather than peer-symmetric governance.

**Private and E2EE, the crux category.** Three efforts, three different answers.

Germ is an MLS-based end-to-end-encrypted messenger that integrates with ATProto for identity. It uses MLS and ATProto, and instead of a phone number it uses the ATProto identity, so Germ's messages cannot be decrypted by any other service, including Germ itself or Bluesky. Bluesky added a Germ badge to profiles in February 2026, making it the first private messenger to launch natively within the Bluesky app. Crucially for the comparison, Germ runs as a separate encrypted layer that sits on top of ATProto rather than modifying the protocol, and MLS provides forward secrecy and post-compromise security. Its investors include a co-author of MLS. This is the closest existing architecture to Drystone's, ATProto for identity plus a separate MLS confidentiality layer, but it is scoped to DMs, not a durable data model.

Bluesky's Permissioned Data is a published proposal for non-public data. The roadmap frames it as complementing public conversation with mechanisms for less-visible interactions, involving new protocol concepts and sync mechanisms, with several teams (Blacksky, Northsky, Habitat) working in parallel and a sketch design from the Bluesky team. The proposal is explicit about what it is and is not: it shares the shape of public ATProto (identity-based authority, per-user repositories, lexicon-typed records) but is a distinct protocol with its own repository format, sync mechanism, and addressing, built for party-to-party transmission within an access boundary, and it provides access control, not confidentiality; it is not end-to-end encrypted, and services can read the data they handle, with E2EE left as a separate concern an application may layer on top. It uses "spaces" with a space authority, delegation tokens, and client attestations.

Peergos is an existing E2EE system whose maintainers have offered to standardize its PDS protocol for ATProto. In the private-data discussion they describe a design where the PDS has a private blockstore of access-controlled blocks forming distinct writing spaces, each a CHAMP (a compressed hash array mapped prefix trie, the unordered sister structure of MSTs) with dag-cbor-encoded values, and everything is end-to-end encrypted. It is the one deployed E2EE-everything design in the conversation, using a different data structure (CHAMP) from both ATProto's MST and Drystone's Willow chains.

---

## D. The private-data axis in depth

This is where the ecosystem splits and where Drystone is positioned, so it deserves its own comparison. The question is not "public or private" but "when private, is privacy enforced by a trusted reader or by encryption."

| Effort | Durable data model | Privacy enforced by | Can a server read it | Group semantics | Governance model |
|---|---|---|---|---|---|
| Bluesky Permissioned Data | yes (distinct protocol) | access control (space authority) | yes, by design | spaces | owner/authority |
| The Arbiter | membership only | access control (space authority) | yes | spaces, delegation | owner/authority |
| Roomy | yes (CRDT) | encryption (experimental) | contents no, metadata yes | communities | service-mediated |
| Germ | no (ephemeral DMs) | encryption (MLS) | no | none | n/a (pairwise) |
| Peergos | yes (CHAMP) | encryption (E2EE) | no | writing spaces | owner-centric |
| **Drystone** | **yes (Willow chains)** | **encryption (MLS) + governance fold** | **no (content-blind stores)** | **groups, read-scoped assets** | **peer-symmetric k-of-n** |

Two observations from this table. First, the efforts cluster: access-control ones (Bluesky, Arbiter) have durable models and server-readable data with authority-based governance; confidentiality ones (Germ, Peergos) enforce by encryption but are either not a social data model (Germ) or not governance-oriented (Peergos). Second, Drystone is the only row that is durable *and* confidential *and* peer-governed simultaneously. Roomy is the nearest neighbor (durable, encrypted, communities) but is experimental, leaks conversation metadata, and is service-mediated rather than peer-governed, and Drystone's content-blind store and gap-aware convergence are specifically designed to close the metadata-leak gap Roomy openly has. `Synthesis.`

---

## E. Where Drystone aligns with the ecosystem (validations)

The survey validates several Drystone bets, which is reassuring rather than threatening; convergence by independent teams is evidence the choices are sound.

- **MLS is the ecosystem's chosen confidentiality primitive.** Germ ships MLS, Bluesky's own engineers name MLS the top candidate for eventual E2EE, and Germ's backers include an MLS co-author. Drystone's MLS foundation is strongly validated as the direction the whole ecosystem is moving. `Verified`/`press`.

- **Encrypted content does not belong in public repos.** Bluesky's team explicitly discourages storing encrypted content in public ATProto repositories, citing the raised stakes of key loss and archived leakage. This is exactly why Drystone uses the MLS envelope and off-relay private stores rather than putting encrypted Willow entries on public relays (`../impl/drystone-design/history-durability.md`, and `../impl/drystone-design/social-mapping.md` §B). Alignment. `Verified`.

- **Private data needs a distinct substrate, not a flag on the public one.** Bluesky's permissioned data is a *separate protocol* with its own repo format, sync, and addressing, not an extension of public ATProto. This validates Drystone's choice to carry private data on MLS-plus-Willow-chains rather than on public ATProto repos, and it echoes Drystone's own scope-is-the-store rule. `proposal`.

- **Adjunct off-PDS storage is a recognized pattern.** Tangled's knots hold bulk git data off the PDS with pointers in the repo, the same architectural move as Drystone's durability tier holding sealed blobs off the main channel. Drystone's addition is content-blindness, a knot that reconciles structure it cannot read. Alignment. `Verified`.

- **Content-plane-first, groups-later is the natural build order.** Frontpage shipped the content plane and lists communities as a future goal, which mirrors Drystone's staging (`../impl/drystone-design/social-mapping.md` §D). `Verified`.

- **Membership-as-a-service is emerging.** The Arbiter is a standardized membership calculator, which is exactly the interop projection Drystone anticipated for its governance-chain member list (`../impl/drystone-design/social-mapping.md` §I). Alignment, with the difference that Drystone's source of truth is the peer-symmetric fold, not an authority. `proposal`.

- **Germ's architecture is Drystone's, scoped down.** ATProto for identity, a separate MLS layer for confidentiality, not touching the public protocol. Drystone generalizes precisely this pattern from ephemeral DMs to a durable social data model with groups and history. That an independent, funded team validated the identity-plus-separate-MLS-layer pattern in production is the strongest single alignment in the survey. `press`.

---

## F. Where Drystone is genuinely novel

Netting out the survey, Drystone's novelty is not any single feature but a combination no one else holds, plus two specific constructs.

- **Cryptographic confidentiality applied to a full durable social data model.** Germ has MLS confidentiality but only for messaging; Bluesky's permissioned data has a durable model but only access control; Peergos has E2EE durability but is not social or governance-oriented. Drystone is the only effort applying MLS-grade confidentiality to durable feeds, groups, read-scoped assets, and converged history. This is the empty cell. `Synthesis.`

- **Peer-symmetric governance rather than an owner or authority.** Every governance-bearing effort in the survey (Bluesky spaces, the Arbiter, Peergos, Meadowcap owned namespaces) rests on an owner or a space authority. Drystone's k-of-n monotonic fold with no privileged node (Part 2 §5.7, §7.3) is a genuinely different governance stance, and it is the thing that makes Drystone's private groups center-free rather than owner-run. `Synthesis.`

- **The content-blind durability tier.** A node that reconciles history structure without reading content (`../impl/drystone-design/history-durability.md`), which is stronger than Tangled's knots (which read the git data) and closes the metadata leak Roomy has. Structure-aware reconciliation without content visibility is not present elsewhere in the survey. `Synthesis.`

- **The private-overlay-on-public construct.** A scope-sealed record referencing a public locator, joined only on-device (`../impl/drystone-design/social-mapping.md` §G), lets Drystone present a seamless public-and-private product while keeping a topological separation none of the access-control efforts achieve, since theirs are server-readable and could leak across the boundary. `Synthesis.`

- **Membership as a cryptographic gate, not a checked permission.** In Drystone a non-member cannot produce a readable record because they lack keys; in the access-control efforts a non-member is refused by a service that could itself read. This is the categorical difference that makes Drystone's private groups private in a way the ecosystem's permissioned data explicitly is not. `Synthesis.`

---

## G. Open questions and what to watch

- **Which permissioned-data design wins, and does Drystone interoperate with it.** Blacksky, Northsky, Habitat, and Bluesky are building in parallel, and the outcome will set ecosystem norms. Drystone should track whether its confidentiality model can present an access-control-compatible projection for interop, the way its membership can project to an Arbiter shape. `[confirm.]`

- **Whether Germ's MLS layer is a substrate to build on or interop with.** Germ published guidance for other ATProto apps to use its E2EE. Whether Drystone could ride or interoperate with Germ's MLS layer, versus running its own, is worth evaluating, since it is the one production MLS-on-ATProto deployment. `[confirm.]`

- **CRDT community models versus Drystone's.** Roomy builds communities on a general CRDT (Loro or Automerge per differing reports) with server-mediated membership. Comparing its convergence and metadata posture against Drystone's Willow-chains-plus-governance would sharpen where Drystone's discipline pays off. `[confirm.]`

- **Peergos's CHAMP versus Willow.** Peergos offers a deployed E2EE substrate on a different data structure. A direct comparison of CHAMP writing-spaces against Willow chains for the private store is worth doing, especially as Peergos seeks standardization. `[confirm.]`

- **The Arbiter's space-delegation.** Its "a space can be a member of another space" is close to Drystone's nested-group thinking; whether the two models can share a projection is worth exploring. `[confirm.]`

---

## H. Summary

Drystone's position in one paragraph: the ATProto ecosystem has validated Drystone's foundations (MLS for encryption, ATProto identity, reuse of the public content lexicon, off-PDS adjunct storage, and content-plane-first staging), and is actively building the private-data layer Drystone needs, but every existing private-data effort is either confidentiality-without-a-data-model (Germ), a-data-model-without-confidentiality (Bluesky permissioned data), or E2EE-without-social-governance (Peergos). Drystone is the only design combining a durable social data model, cryptographic confidentiality, and peer-symmetric governance, and its distinctive constructs, the content-blind durability tier, the private-overlay-on-public, and membership as a cryptographic gate, are not present elsewhere. The ecosystem is converging toward Drystone's neighborhood; the specific corner Drystone occupies is still empty.

---

## Sources

Primary and proposal sources: the ATProto Spring 2026 roadmap (atproto.com/blog/2026-spring-roadmap); the Permissioned Data proposal, bluesky-social/proposals PR #94; the encryption-for-private-content and private-data discussions, bluesky-social/atproto discussions #121 and #3363 (the latter containing the Peergos description); the Tangled documentation (docs.tangled.org, blog.tangled.org). Press and secondary: TechCrunch on Germ (2026-02-18) and on the ATProto app landscape including Roomy (2025-06-13); Global Dating Insights and Sovereign Magazine on Germ (2026-02); LavX and developer blogs on Tangled; the AT Protocol Wikipedia entry (retrieved 2026-05) for Leaflet, Blacksky, and Automattic ATmosphere. Frontpage specifics are carried from `../impl/drystone-design/social-mapping.md` and its cited research pass. recipe.exchange facts are from the live site (recipe.exchange, retrieved 2026-07-08); the custom-lexicon/storage detail is inferred, not confirmed. All dates and maturity levels are a mid-2026 snapshot and will drift.

---

## Changelog

`Working draft; per the suite's doc-method, transitions are recorded here.`

- **Draft, first consolidation.** Surveys the ATProto ecosystem along the axes relevant to Drystone (§A), tabulates the landscape (§B), profiles apps by category (§C), analyzes the private-data axis in depth (§D), and separates Drystone's alignments (§E) from its genuine novelty (§F). The central finding is that Drystone occupies an empty cell: durable social data model plus cryptographic confidentiality plus peer-symmetric governance.

- **Provenance.** Facts are `Verified` against primary sources or flagged `press`/`proposal`; the analysis and Drystone positioning are `Synthesis`. This is a snapshot of a fast-moving ecosystem; the Open questions (§G) are the live threads to re-check.

- **Companion linkage.** Widens the prior-art section of `../impl/drystone-design/social-mapping.md` (§K) to the whole ecosystem. The private-data axis (§D) is the ecosystem context for the confidentiality posture of `../impl/drystone-design/asset-keying.md` and `../impl/drystone-design/history-durability.md`.

- **2026-07-08, recipe.exchange added.** Filed a small public content-plane app (community recipe sharing on ATProto, reusing Bluesky identity and CDN) into the §B landscape table and §C content-plane profiles. Verified live for product/identity-reuse; storage locus (custom lexicon in PDS repo) flagged `[confirm]`. Another data point for the content-plane-first, custom-lexicon-on-public-identity pattern; no change to the private-data findings.
