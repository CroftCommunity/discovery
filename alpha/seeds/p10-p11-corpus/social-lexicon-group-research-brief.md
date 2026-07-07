# Research brief: social application lexicons and group feature models over a per-author, membership-scoped substrate

## Purpose

This is a research plan to be executed in a separate session and brought back. The goal is to gather two things and map them onto a specific data-model architecture (described in Context below):

1. The **ATProto / Bluesky content lexicon** (the public microblogging schema set), precisely enough to reuse it as the content-plane semantics.

2. The **group and community feature models** of Reddit, Facebook Groups, and LinkedIn Groups (and Discord as a comparison), enumerated as membership operations, member statuses, moderator capabilities, content interactions, moderation workflows, and group-chrome fields.

The output should let the requester define a content data model that presents identically to public Bluesky while being backed, in the private case, by a different storage and key model, plus a custom group lexicon that Bluesky does not provide.

Do not just describe the platforms in prose. Produce the structured deliverables listed at the end, because they are meant to drop directly into a data-model design.

---

## Context: the architecture these findings map onto

A brief model of the target system, so findings can be mapped rather than merely collected. The researcher does not need to evaluate this architecture, only to map findings onto it.

**Stores are per-author and unioned.** Every participant writes only into their own subspace, signed with their own key. A "feed" or "group view" is the union of many single-author records, gathered and filtered at read time, not a single shared object. This is the same shape as an ATProto repository (a per-account signed log) and its relay firehose (union of repos). Confirming that equivalence is part of the research.

**Content resolves by one of four convergence flavors.** A store is classified by three questions: is it mutable, is order load-bearing, is any single unit written by more than one person concurrently. The flavors:

- Ordered append-only log (governance, audit, moderation actions): resolved by a deterministic fold, never overwritten.

- Grow-set (posts, comments, likes, votes): single-author, immutable, resolved by set union, ordered only for display.

- Single-writer mutable record (a profile, an editable post): one writer, resolved by last-writer-wins per path.

- Multi-writer mutable record (a shared wiki, a family to-do list, a group's chrome object): several people co-edit one bounded object, resolved by a merge CRDT. This is the bounded exception, not the scaled default.

**The working hypothesis to test:** at social-media scale, essentially all content is the grow-set flavor (single-writer unions), and multi-writer objects are small and bounded (a group's banner/description/rules/wiki that a few moderators co-edit, a family to-do list). The research should confirm or complicate this by finding where real platforms have genuinely concurrent multi-writer objects versus single-owner-overwrite objects.

**Membership, roles, and bans are a governance layer, not content.** Who is in a group, who holds which role, and who is banned are handled by a separate membership/governance mechanism, not by application records. So "membership operations" split into two categories the research should keep distinct: governance primitives (join, leave, remove, ban, role grant, threshold) which the target system already has, and application-level member statuses and gates (approved-contributor, screening questions, rules acceptance) which are what the research is enumerating.

**Public and private are hard-separated.** The same lexicon (semantics) may describe both a public post and a private post, but scope is never a field on the record; it is a property of which store and which key the record lives under, decided at author time and immutable afterward. Public data rides real public relays; private data stays in a membership-scoped, key-sealed store. The two are joined only locally on the user's authenticated device, never server-side. The research does not need to design this, but should note, per platform, how each currently handles public-versus-private content and where that separation has historically failed (for example, the Twitter Circles leak) as cautionary reference.

**The lexicon is an interface contract, not a storage coupling.** The plan is to reuse Bluesky's content lexicon for interoperability and identical presentation, and to define a custom group lexicon for the group layer Bluesky lacks. The research should therefore treat the ATProto lexicon as a schema to catalog and the group features as requirements for a schema to design.

---

## Track A: the ATProto / Bluesky content lexicon (the reuse layer)

Pull the current schemas. Treat any specific field name below as a starting point to verify against the live lexicon, not as fact, because these evolve.

**A1. The repository and identity model.** Confirm and document: the account identifier (DID), the personal data server (PDS), the repository as a signed Merkle structure of records, the record addressing (collection NSID plus record key), the commit signing, and the relay/firehose plus AppView indexing pipeline. The specific question to answer: is an ATProto repo structurally a single-author append-only signed log, and is a relay firehose structurally a union of such logs. Note how records are created, overwritten, and deleted (whether there is an update operation or only put-by-key and delete).

**A2. The content record types.** Catalog the current schema for at least: the post record, the like record, the repost record, the reply/threading mechanism (how a reply references its parent and thread root), the quote-post mechanism, the follow record, the block record, the list and list-item records, the actor profile record, the feed generator record, and the starter pack record. Bluesky NSIDs to look up include `app.bsky.feed.post`, `app.bsky.feed.like`, `app.bsky.feed.repost`, `app.bsky.feed.threadgate`, `app.bsky.feed.postgate`, `app.bsky.feed.generator`, `app.bsky.graph.follow`, `app.bsky.graph.block`, `app.bsky.graph.list`, `app.bsky.graph.listitem`, `app.bsky.graph.starterpack`, `app.bsky.actor.profile`. For each: its fields, how it references other records (strong references, AT URIs, content hashes), whether it is single-author-owned, and whether it is immutable or overwritten.

**A3. The interaction-gating records.** Pay special attention to `threadgate` and `postgate` (who may reply, who may quote), because they are Bluesky's answer to the "who decides a post's interactions" question and are directly relevant to the group-posting-gate design. Document exactly what they can express.

**A4. Lexicon mechanics.** Document how lexicons are defined (the NSID namespacing scheme, the lexicon schema language and its types), how a custom (non-`app.bsky`) lexicon rides the same PDS and relay infrastructure, and whether custom-lexicon records appear on the firehose and can be indexed by a custom AppView. The question to answer: can a custom group lexicon coexist on the same account and infrastructure as the standard content lexicon.

**A5. Map each content record to a flavor.** For every record type cataloged, state which of the four convergence flavors it is (expected: nearly all grow-set, profile and list single-writer-mutable). Flag anything that does not fit.

---

## Track B: group and community feature models (the layer to define)

Enumerate, for **Reddit (subreddit)**, **Facebook Groups**, and **LinkedIn Groups**, with **Discord (server)** as a comparison point. Produce a feature matrix (see Deliverables), not prose. Cover these categories for each platform:

**B1. Join and membership gating.** How does someone become a member: open join, request-and-approve, invite-only, or a mix. What barriers exist at join: rules acceptance, screening or membership questions, interest or eligibility gating, bot or spam checks, minimum account age or karma. How is leaving handled.

**B2. Member statuses.** Enumerate every distinct status a member can hold: for example plain member, approved or trusted contributor, muted, restricted, banned (temporary and permanent), moderator, admin or owner. Note the difference between a status that gates content (approved contributor) and a role that grants authority (moderator).

**B3. Moderator and admin capabilities.** Enumerate what elevated roles can do: edit group chrome (name, icon, banner, description, rules, sidebar or wiki), pin or sticky content, remove or hide content, approve or reject submitted posts (pre-moderation), lock or archive threads, assign roles to others, manage flair or tags, configure automated moderation, view a moderation log, ban or mute members, manage join requests. Note the granularity of the permission model (for example Reddit's per-permission moderator system versus a coarse admin-or-moderator split).

**B4. Content interactions available to ordinary members.** Enumerate: post or submit, comment, nested reply and the maximum depth, react or like, vote (up and down, or up-only), share or crosspost, report or flag, award or tip, edit own content, delete own content.

**B5. Content and moderation workflow states.** Determine whether the platform is pre-moderation (a post is queued and must be approved before it is visible), post-moderation (a post is visible immediately and may be removed later), or configurable per group. Enumerate the states a piece of content moves through (for example submitted, in queue, approved, published, removed, hidden). This decides whether a staged submitted-then-published pipeline is needed.

**B6. Group chrome (the object that renders the top bar and frame).** Enumerate the fields that define the group itself and are loaded to render its view: name, icon, banner, short and long description, rules, sidebar or wiki content, member count, pinned or featured content pointers, flair or tag definitions, topic or category, visibility setting, join setting. This is the candidate multi-writer-mutable object; note which fields are genuinely co-edited by multiple moderators versus set once by an owner.

**B7. Rules and behavior contract.** Document how each platform presents group rules and whether joining requires explicit acceptance, and how community guidelines are surfaced. This informs a signed-rules-acceptance design.

**B8. Blocking and muting.** Document how blocking and muting work per platform, whether a block is visible to the blocked party, and whether blocks are personal or group-scoped.

**B9. The professional-versus-casual axis.** Compare LinkedIn Groups against Facebook and Reddit specifically to determine whether the professional character is a matter of policy and defaults (real-name enforcement, default visibility, moderation norms, content tone) or of mechanism (different features). State the finding explicitly, because the hypothesis is that the mechanisms are near-identical and only the defaults and policies differ.

---

## Track C: the public/private boundary case (a dating or meet application)

Lighter than Tracks A and B, to inform the field-partition design.

**C1.** For a small number of dating or meet applications (for example Hinge, Bumble, or comparable), enumerate which profile and activity fields are public or discoverable versus private: display information, photos, interests, location and its precision or fuzzing, who-liked-you, matches, and messages.

**C2.** Document how matching consent works, specifically whether messaging is gated on mutual opt-in, because that is a two-sided-consent pattern relevant to gating private interaction.

**C3.** Note any documented failures where a public or private boundary leaked, as cautionary reference, alongside the Twitter Circles case.

---

## Track D: prior art

**D1. Custom lexicons on ATProto.** Investigate existing applications built on ATProto with their own lexicons rather than the Bluesky microblogging one. Candidates to look into (verify they exist and are current): Frontpage (a link-aggregator, Reddit-like, which is high-value because it is the closest existing analog to a community with posts and comments on a custom lexicon), WhiteWind (long-form blogging), Smoke Signal (events), and any others found. For each, document what custom lexicon they defined and, crucially, whether any of them implemented groups, communities, membership, or moderation, and how.

**D2. A community or group lexicon.** Search specifically for any existing or proposed community, group, or forum lexicon in the ATProto ecosystem, since Bluesky's own lexicon reportedly lacks first-class membership-gated groups. Report whether one exists, and if so its shape.

**D3. Local-first and CRDT social applications.** Briefly survey local-first or peer-to-peer social and community applications that use CRDTs or per-author logs (for example Roomy or Muni Town, and any others), focusing on how they model a group or community object and its membership, and how they handle group chrome versus content.

---

## Deliverables

Produce these as structured artifacts, in this order.

**D-1. Content lexicon catalog.** A table with one row per ATProto content record type: NSID, purpose, key fields, how it references other records, single-author or shared, immutable or overwritten, and which of the four convergence flavors it maps to.

**D-2. Group feature matrix.** A table with feature rows grouped by the B1 through B8 categories, and columns for Reddit, Facebook, LinkedIn, and Discord, with two synthesis columns: a common-core column (the feature as it appears across all or most platforms) and a notable-outliers column (where one platform differs meaningfully).

**D-3. Membership and moderation lifecycle model.** A consolidated, platform-independent enumeration of: join-gating options, member statuses, moderator and admin capabilities, and content workflow states. This is the synthesized model, distinct from the per-platform matrix.

**D-4. Grid mapping.** For each feature or object identified, a row stating: the feature, which convergence flavor it maps to (grow-set, single-writer mutable, multi-writer mutable, ordered log), which read-scope it would take (all-members, role-scoped, group-internal), and whether it reuses a standard ATProto lexicon record or requires a new custom-lexicon record. This is the artifact that connects the research to the data-model design.

**D-5. Professional-versus-casual finding.** A short explicit statement: is the LinkedIn-versus-Facebook difference policy and defaults, or mechanism, with the evidence.

**D-6. Prior-art notes.** For each application investigated in Track D: name, link, what custom lexicon it uses, and whether and how it handles groups, membership, or moderation.

**D-7 (stretch). A draft custom group lexicon sketch.** If time allows, propose the record types a custom group lexicon would need, informed by the findings: a group-profile or chrome record (the multi-writer object), a membership or role record or governance mapping, a moderation or gating record, and a post-annotation mechanism by which a content record declares the group it belongs to. Fields per record, and for each note whether it is single-writer or multi-writer and its read-scope.

---

## Framing to keep front of mind

- The content plane is expected to be grow-set (single-writer unions) almost everywhere; multi-writer is expected to be small and bounded (group chrome, shared wikis, shared lists). Actively look for counterexamples, but do not assume big collaborative objects; social scale is unions.

- Membership and roles are a governance layer, already provided by the target system. Enumerate application-level statuses and gates, but do not conflate them with the underlying join/remove/ban/role primitives.

- Scope (public versus private) is a property of the store, never a field on a record. When documenting how platforms mix public and private content, note where the boundary is a mere visibility flag on a shared store, because that is the pattern the target architecture deliberately avoids.

- The lexicon is an interface contract. The aim is identical presentation across a public relay ride and a private membership deployment, with different storage underneath. Catalog the standard lexicon for reuse; treat group features as requirements for a lexicon to design.

## Suggested search seeds

- ATProto lexicon: the official ATProto and Bluesky documentation and the lexicon schema repository; searches for the specific NSIDs listed in A2; ATProto repository and Merkle structure documentation; threadgate and postgate documentation.

- Group features: each platform's own help or moderator documentation for subreddit settings and moderator permissions, Facebook group admin tools, LinkedIn group management, and Discord roles and permissions.

- Prior art: Frontpage on ATProto, custom ATProto lexicon applications, ATProto community or group lexicon proposals, and local-first community applications.
