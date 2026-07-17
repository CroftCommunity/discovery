# Drystone: Social Data-Model Mapping, Public Lexicon Interop, the Group Layer, and Private Overlays

`Status: draft. Mapping settled and grounded in a primary-source research pass; several application-layer items and one confidentiality question (the AppView-provisioned scope key) open. See Open items.`

`Scope: how Drystone's data model maps onto the public ATProto/Bluesky ecosystem for interoperable public presentation, how the group layer Bluesky lacks is supplied by Drystone's governance and store flavors, and how private content relates to public objects through a single private-overlay-by-reference construct. Covers the per-author-union equivalence and its limit, the content-plane flavor mapping, lexicon reuse versus custom, the interaction gates, the group-annotation and private-overlay constructs, and the overlay distribution topologies.`

`Companion to: asset-keying.md (the per-scope content keys and store flavors this doc's records are sealed and classified by) and history-durability.md (the MLS transport, per-author chains, and content-blind store that carry private records here). Also companion to ../../cairn/atproto-ecosystem.md, which widens this document's prior-art section to the full ATProto ecosystem. Also companion to ../../drystone-spec/part-1-reasoning-underpinnings.md and ../../drystone-spec/part-2-certifiable-design.md; vocabulary inherits from the conventions-and-decisions reference.`

This is the "what" for the social-application surface: the mapping between Drystone's private, membership-scoped model and the public ATProto ecosystem, and the constructs that let the two present as one product without sharing a store. Normative keywords are **MUST / SHOULD / MAY** (BCP 14). Status flags per load-bearing claim: `Verified` (checked against a primary source; where the check was performed in the external research pass against the ATProto specifications rather than directly this cycle, it is marked `Verified`), `green-real` (exercised against a running system), `Design` / `Synthesis` (the design's own reasoning), `[confirm]` (load-bearing and resting on an unresolved external or internal item).

> **Scope is the store, never a field.** The governing rule of this document: the public-versus-private distinction is never a boolean on a record. Scope is a property of which store and which key a record lives under, decided at author time and immutable after. This is the mitigation for the scope-as-a-flag failure mode (the Twitter Circles incident, in which private content and public content shared one store separated only by a visibility attribute that a bug bypassed). Every construct here obeys it. `Synthesis.`

> **One primitive, one boundary.** Inherited from the companion docs: private data uses MLS over iroh and confidentiality is enforced by encryption under a withheld key, never by a second protocol or by a trusted read-time check. `Synthesis.`

---

## A. Terms

**content plane**: the microblogging records (posts, replies, likes, reposts, follows, blocks). Single-author, unioned. Reused from the public ATProto lexicon unchanged.

**grow-set discipline**: the application-layer rule that a content record is only ever created (with a timestamp-ordered key) and never rewritten in place, so it behaves as an immutable grow-set element. Necessary because the ATProto substrate does not enforce immutability (§B).

**group chrome**: the object that renders a group's frame (name, icon, banner, rules, sidebar or wiki, pinned pointers, flair). The one bounded multi-writer object in the model.

**group-annotation**: a separate single-author record declaring that a content record belongs to a group, attached by reference rather than by mutating the content record. Defined in §F.

**private overlay**: a scope-sealed record that references a public object by its public locator and is composed with it locally on a member's device. The single construct behind private replies, private annotations, and private fields over public profiles. Defined in §G.

**overlay reference**: the one-directional link a private overlay carries to its public target, a strong reference (uri plus content hash). Points private to public only; no public record ever references a private overlay.

**distribution topology**: how an overlay's audience is scoped and who serves it, one of MLS-group-powered, private-AppView-powered, or hybrid. Defined in §H.

**governance-view (of membership)**: the group's member list as a read of Drystone's k-of-n governance chain (Part 2 §5.7), distinct from an Arbiter-style service that computes membership as an application record. Defined in §I.

---

## B. The per-author-union equivalence, and its limit

`Realizes: P-Local-Truth`

**The core equivalence holds.** `Verified` against atproto.com/specs: an ATProto repository is a single-author signed Merkle structure mapping a collection-and-record-key path to record content hashes, and a relay firehose is a union of such per-author repositories at the commit-event level. This confirms the suite's model: each persona writes its own subspace, and a feed is the union of subspaces (companion history-durability §I; asset-keying §C). "Right to your own namespace, unions are the deal" is validated.

**Two caveats the research surfaced, both stated.**

- **The wire is diff-based and evolving.** `Verified`: the firehose commit event ships a Merkle-tree diff plus an operations array, and the ecosystem is moving toward record-level and collection-filtered consumption. The union-of-logs framing is accurate at the commit level but MUST NOT be read as a literal per-author append-only log on the wire. This affects only public interop consumption, not Drystone's private transport.

- **The substrate is last-writer-wins, not append-only.** `Verified`: create, update, and delete are all first-class, deletion leaves no tombstone, and the backward commit pointer is virtually always null, so there is no durable backward chain by default. Therefore immutability of public content records is an application-layer discipline, not a substrate guarantee. If a design ever needs to mutate a content record in place, that record has left the grow-set flavor and must be reclassified as single-writer-mutable.

**The limit resolves in Drystone's favor.** The addressing shapes match (per-author, keyed, unioned), but the convergence disciplines differ. A Willow store is a state-based CRDT with defined overwrite and prefix-prune semantics (asset-keying §E; Willow data-model, `Verified`), whereas an ATProto repo is a plainer LWW key-value store with no tombstone and no enforced chain. Drystone's private side carries a per-author hash-linked chain (history-durability §I, the `predecessor_digest` linkage), which is exactly the durable backward chain the ATProto substrate lacks and is what makes gaps nameable and deletion auditable. So the private transport is the more disciplined of the two, and the public ride is where immutability must be enforced by grow-set discipline rather than assumed. `Synthesis.`

---

## C. Content-plane flavor mapping

`Realizes: P-Durable-Enablement`

The research classified the public lexicon, and it maps onto the asset-keying §E flavors as follows. `Verified` for the per-record classification.

- **Grow-set** (single-writer, immutable-in-practice): post, reply, quote, like, repost, follow, block, and list items. These are the scaled content surface, reused unchanged, converged by union.

- **Single-writer mutable** (last-writer-wins per key): the actor profile (fixed key, overwritten), list metadata, feed generator, starter pack, and the interaction gates threadgate and postgate.

- **Multi-writer mutable** (bounded, merge CRDT): group chrome only, and even there weakly, since most chrome fields are set once by an owner and only a few are genuinely co-edited (§I). The research found essentially no other genuinely concurrent multi-writer object at social scale.

This confirms the suite's inversion: multi-writer is the small bounded exception, and everything unbounded is single-writer unions. None of the public content records are substrate-enforced append-only logs; that discipline is imposed by the application (§B). `Synthesis` for the mapping onto Drystone flavors.

---

## D. Lexicon strategy: reuse the content plane, supply the group layer

`Realizes: P-Durable-Enablement`

**Reuse the public content lexicon unchanged for the public case.** `Verified`: custom lexicons ride the same infrastructure and coexist on the same account as standard content records, and adopting the standard post, reply, like, repost, follow, block, and profile records guarantees identical presentation on public relays and existing clients. This is the interop dividend: the public presence is a genuine public presence, not a private format imitating one.

**Supply the group layer as a custom lexicon, because Bluesky has none.** `Verified`: Bluesky has no first-class membership-gated community or subreddit record; its group-chat lexicons are DM-oriented and server-mediated, not a general community primitive. So the group layer (chrome, membership view, moderation, rules acceptance, group-annotation) is Drystone's to define (§I), reusing the content lexicon beneath it. LinkedIn, Facebook, and Reddit are all this shape: standard content plus a custom group layer, differing only in policy and defaults (§J). `Synthesis` for the split.

---

## E. Interaction gates are public write-gates, not privacy

`Realizes: P-Knowable-Truth`

**What threadgate and postgate are.** `Verified`: threadgate lives at the root post's key and restricts who may reply, via an allow list of rules (mentioned actors, your followers, accounts you follow, members of a named list), where an empty list permits nobody and an absent record permits anyone; it also carries a hidden-replies list. postgate governs who may quote or embed a post. Both are entirely public: they gate who may write public content, and a reply that passes the gate is still a world-readable public post. They are not confidentiality mechanisms.

**The only thing borrowed is the attachment mechanic.** These gates attach to a post by sharing its key and referencing it, without mutating the post. That attach-by-separate-record, do-not-mutate pattern is the template for the group-annotation (§F) and the private overlay (§G). Nothing about their public write-gate semantics is borrowed; conflating the gate's purpose with a privacy boundary would be the scope-as-a-flag error. `Synthesis.`

---

## F. The group-annotation construct

`Realizes: P-Durable-Enablement`

**Requirement.** A content record that belongs to a group MUST remain byte-identical to its public form. Group membership of a content record MUST be declared by a separate single-author annotation record that references the content record by its locator, and MUST NOT be a group-identifier field added to the content record. This keeps the content record reusable and identical to public Bluesky while a custom AppView indexes which group a record belongs to, mirroring how the interaction gates attach by reference rather than by mutation (§E). `Design.`

The annotation record (`postTag` in the research sketch) carries a strong reference to the content record and the group identifier, is authored by the post's author, and takes the read-scope of the post. Whether a post is admitted to a moderated group's view is a separate governance or capability relation (the group's roles accept or hide the referenced post), not a property of the annotation itself. `Design.`

---

## G. The private overlay by reference

`Realizes: P-Local-Truth, P-Knowable-Truth`

This is the single construct behind private replies to public posts, private annotations on public content, private commentary within a group about a public item, and the private-fields-over-public-profile case of a meet or dating application. It is, at the data level, a link that is rendered well; all the design beyond the link is presentation.

**Requirement.** A private contribution that relates to a public object MUST be a scope-sealed record that references the public object by its public locator (uri plus content hash) and lives in a private store, and MUST NOT be expressed as a field on the public object. The reference is one-directional: a private overlay references a public object, and no public record ever references a private overlay. `Design.`

**Why one-directional: the reference is itself the leak.** The one-way rule is a privacy invariant, not an interop or reuse convenience. A public record that referenced a private record would leak two things about that private record even though the public record discloses none of its sealed content: its *existence* (that some private overlay exists at all) and its *AT-URI* (the exact address that names it). Both are disclosures the private scope is supposed to prevent, so a public-to-private reference breaks confidentiality by structure regardless of what it contains. The rule therefore has a positive enforcement half as well as a prohibition: the prohibition is that a public record MUST NOT name a private one, and the enforcement is that any public record whose reference points at a target that is not itself public MUST be redacted (dropped) before it crosses the boundary, because publishing it would disclose the existence and AT-URI of the referenced private record. A dangling public reference to a private target is a cross-boundary integrity failure, not a cosmetic loose end. This was demonstrated on a working public-private split: visibility is default-deny (any record not explicitly public stays private), and a public reaction whose subject strong-reference pointed at a non-public record was dropped rather than published. See the reference-index (Proof: public-private split, no-dangling-reference redaction). `Synthesis`; the redaction behavior is `Verified` on the split prototype.

**Local join.** The public object and the private overlay are never joined server-side. A member's authenticated device holds the public object (fetched from a relay) and decrypts the overlay (from the private store), and composes them into one view. The only place the two meet is the client. This is the topological separation of the companion docs, applied to the overlay relationship. `Synthesis.`

**Version binding.** Because the overlay reference includes the public object's content hash, the overlay is bound to a specific version of that object. If the public author edits or deletes the object, the overlay still names the exact content it addressed, so the client can render "in reply to (this original), since changed or deleted" rather than silently reattaching. This correctness property matters more here than for public replies, because the two objects live in different systems with different lifetimes. `Synthesis.`

**Rendering.** Composition is seamless in layout, one thread, one interaction model, but MUST be explicit in scope affordance: the user MUST see which contributions are private and to which scope, because authoring into the wrong scope is the scope-as-a-flag failure at the level of a single message. The scope marker is not a field on any record; it renders which store the composer will write to, decided before authoring. UX consistency in reading and interaction, with an unambiguous marker of where a contribution will land. `Design.`

**The invariant that keeps it safe.** A public object's exposure is exactly zero regardless of how many private overlays reference it, because the references live only in the private records, which only holders of the scope key can read. Private-to-public references only, scope is the sealing store, and no scope field anywhere. `Synthesis.`

---

## H. Overlay distribution topologies

`Realizes: P-Knowable-Truth`

An overlay's record is identical across topologies; what differs is the scope the sealing key belongs to and who serves the records. There are three.

- **MLS-group-powered.** The overlay records are sealed under an existing MLS group's key and reconciled through the machinery of the companion history-durability doc (live delivery, history convergence, content-blind store). The audience is the group's membership, defined by key possession. Strongest confidentiality, group-scale, with rekey-on-membership-change as the cost. This is the default for bounded, governed audiences.

- **Private-AppView-powered.** The overlay records are served and indexed by a private AppView, which composes them over public content and gates a broader or more fluid audience than a single group's roster. The audience is defined by what the AppView authorizes at serve time.

- **Hybrid.** MLS-group sealing with AppView-mediated serving: the AppView is a highly-available distributor of sealed overlay records to group members and never holds the scope key. This is the content-blind history store of the companion doc, presenting an AppView-shaped read API, and is the sweet spot for most private-comment cases.

**The invariant across all three, stated once.** Encryption is the confidentiality boundary in every topology. An AppView gates *offering* (who is handed ciphertext), never *reading* (who can decrypt), so a compromised AppView leaks ciphertext to someone who still cannot read it. Collapsing serving and confidentiality, letting an AppView serve cleartext to authorized readers, rebuilds the visibility flag and makes the AppView a single point of confidentiality failure. The private-AppView topology is therefore safe only when the overlay records remain scope-sealed and the AppView's role is serving and indexing, not being the boundary. `Synthesis.`

**The serve half is now demonstrated (offer-gating against a verified identity).** A working spike stood up the hybrid serve half end to end: a content-blind store offers sealed overlay records only to a viewer whose identity is cryptographically verified — an atproto inter-service-auth JWT checked against the caller's `#atproto` DID-document key (real secp256k1/p256, confirmed live end to end) — and refuses non-members, anonymous callers, and even the existence question with one indistinguishable refusal. The store's content-blindness is enforced at a *compilation boundary*, not by convention: the seal/open code is absent from the serving binary's dependency graph, so the distributor cannot read what it hands out. Removing a member stops future offering while already-fetched ciphertext plus a retained key still decrypts — offering gated, reading left to encryption, exactly as the invariant requires. This covers server-to-server service auth, not the interactive OAuth/DPoP client-login leg, which remains unproven. `green-real` (offer-gating + compilation-boundary content-blindness, on the sealed-serve spike at loopback / in-process; the AppView-provisioned scope key remains the open item below).

**The AppView is source-agnostic, and the index is a disposable projection.** What lets one AppView serve both a private encrypted source and the public Bluesky firehose without special-casing either is that ingest, index, and serve are written against a source abstraction, not against a particular origin. This is not only a design intention: a working prototype ran the identical ingest, indexer, and server code (the `indexer`, `server`, and `views` modules, asserted byte-identical at runtime) over two sources in turn, a local encrypted stack and a real atproto Jetstream firehose (with genuine CIDv1 dag-cbor records, create, update, delete, and cursor-resume), and produced the same served views from each. Because the index is rebuilt from source, it is a disposable projection, not a system of record: it can be dropped and reconstructed from the sealed history or the firehose, which is why an AppView serving both a private and a public source is a plausible topology and not a special integration. This is the empirical ground under the byte-identical-content requirement (§F) and the AppView-serves-not-decrypts invariant above. See the reference-index (Proof: source-agnostic AppView over local and Jetstream sources). `Verified` (source-agnostic ingest/serve, on the AppView prototype).

**The visibility regime is default-deny, keyed by store not field.** The three topologies above are three ways to scope an audience, but the classification that decides whether a given record is exposed at all is a single regime: default-deny. A record is private unless it is explicitly published to a public store, and the public-or-private decision is a property of which store the record lives under, never a `visibility` attribute on the record itself. This is the same "scope is the store, never a field" rule that governs the whole document (§the opening rule), and the reason is the same closed-schema reason that motivates the annotation-by-reference construction (§F): a `visibility` field would itself be published as part of the record, so a record that tried to carry its own privacy flag would announce its own privacy state to the world. Default-deny plus store-keyed scope is what makes the leak-of-existence redaction rule above enforceable: a boundary crossing consults the target's store, not a field it might have forgotten to set. `Synthesis.`

| Topology | Audience defined by | Confidentiality basis | Scaling and cost |
|---|---|---|---|
| MLS-group-powered | key possession (group membership) | encryption; members hold keys, store is blind | group scale; rekey on membership change |
| Private-AppView-powered | AppView authorization at serve time | encryption, only if records stay sealed; unsafe if AppView serves cleartext | broader or more fluid audience; AppView is a trusted distributor, not a trusted reader |
| Hybrid (MLS seal, AppView serve) | key possession, with AppView as distributor | encryption; AppView never holds the scope key | group scale with high availability; the recommended default |

---

## I. The group layer, mapped to governance and flavors

`Realizes: P-Peer-Equality, P-Durable-Enablement`

The research's five custom group records map onto Drystone's flavors and governance as follows, with two corrections to the research framing.

- **Group chrome (`group.profile`): multi-writer mutable, bounded, scope A or G.** The one object where concurrent co-editing is plausible (co-moderators editing rules, sidebar, wiki). Correction to the research: do not make the whole object a merge CRDT. Most fields (name, category, icon, banner) are set-once-by-owner and are single-writer mutable; reserve the merge CRDT for the genuinely co-edited subset (rules, sidebar, wiki), which scopes the CRDT's prune-boundary cost (asset-keying §E) to the fields that need it. `Design.`

- **Membership: an ordered governance log, resolved by fold, scope R or G.** Correction to the research: this is not an application record written by an Arbiter-style service account. It is a *view* of Drystone's k-of-n governance chain (Part 2 §5.7), resolved by the monotonic fold (asset-keying ordered-log flavor). The Arbiter pattern is the public or interoperability shape of the same member list; the private shape is the governance chain itself. Both present the same membership; keep them distinct, and treat the Arbiter-style XRPC service as the interop projection, not the source of truth. `Design.`

- **Moderation: an ordered append-only log, resolved by fold, scope R.** Never overwritten, an auditable action trail. Maps directly to the ordered-log flavor.

- **Rules acceptance: a grow-set of signed records, scope G.** One signed record per member per rules version, binding the acceptance to the content hash of the rules at acceptance time. This is the behavior-contract signing raised earlier: a signed acceptance fact that a posting capability can be conditioned on. Immutable grow-set.

- **Group-annotation (`postTag`): single-author, scope matches the post.** Per §F, the reuse-preserving link from a content record to its group.

Join-gating (open, request-and-approve, invite-only) and application-level member statuses (approved contributor, restricted, muted, banned) sit above the governance primitives (join, remove, ban, role grant) that Drystone already provides; the custom lexicon encodes the application-level statuses and gates, not the underlying primitives. `Synthesis.`

---

## J. Professional versus casual is policy, not mechanism

`Realizes: P-Durable-Enablement`

`Verified`: across Reddit, Facebook, LinkedIn, and Discord, the group mechanisms are near-identical (owner and moderator roles, request-and-approve join, optional pre-moderation with an approval queue, remove and block, pin and announce, rules). The LinkedIn professional character is policy and defaults (real-identity norms, professional community policies, a post-approval window, promotional-content routing, moderation tone), not different record types. A single group lexicon serves both, and the professional-or-casual axis is expressed through defaults and policy text. So Drystone needs one group lexicon, parameterized by policy, not a family of platform-specific ones. `Synthesis` for the conclusion for Drystone.

---

## K. Prior art

`Verified` for the existence and shape of each; all are early-stage and directional, not stable standards.

- **Frontpage** is a link-aggregator on a custom lexicon (post, comment, vote), the closest existing analog to a community on a custom lexicon, but it has no community, membership, or moderation record type and runs a single global feed, with sub-communities a stated future goal. It confirms the content-plane shape and the group-layer gap at once.

- **Roomy (Muni Town)** models communities and spaces with channels and categories on a CRDT stack over ATProto, but its wiki pages are last-writer-wins, not real-time collaborative, which corroborates that even the multi-writer chrome is weakly concurrent. It is moving toward ATProto permissioned data for membership.

- **The Arbiter (Muni Town)** is a proposed standardized XRPC group and membership service with access levels and space-delegation, designed to feed Bluesky's permissioned-data direction, and is the closest existing design to Drystone's governance layer. It is the model for the *interop projection* of membership (§I), not for the private source of truth.

---

## L. Posture summary

| Concern (§) | Naive assumption | Drystone posture | Forcing principle |
|---|---|---|---|
| Repo equivalence (§B) | ATProto repo is an append-only log | Per-author keyed store, unioned; substrate is LWW, immutability is app discipline | atproto.com/specs (Verified) |
| Convergence discipline (§B) | Public and private stores are the same | Willow CRDT with prune versus ATProto LWW; the private chain is the added discipline | P-Local-Truth |
| Content flavor (§C) | Rich records need rich convergence | Grow-set at scale, single-writer-mutable metadata, chrome the only bounded multi-writer | asset-keying §E |
| Lexicon (§D) | Find an existing group lexicon to copy | Reuse the content lexicon, supply the group layer custom | no Bluesky group primitive (Verified) |
| Interaction gates (§E) | threadgate could carry privacy | Public write-gates only; borrow the attach-don't-mutate mechanic, not the semantics | scope is the store, not a field |
| Group tag (§F) | Add a groupId field to the post | Separate single-author annotation by reference; post stays byte-identical | reuse and interop preservation |
| Private overlay (§G) | Private reply is a flagged public reply | Scope-sealed record referencing the public locator, joined on-device, private-to-public only | scope-as-a-flag failure (Twitter Circles) |
| Overlay serving (§H) | AppView enforces who can read | Encryption enforces reading; AppView gates offering only | one boundary; encryption is the boundary |
| Membership (§I) | An Arbiter service computes membership | Membership is a view of the k-of-n governance chain; Arbiter is the interop projection | Part 2 §5.7 |
| Chrome CRDT (§I) | Make group.profile a CRDT | CRDT only for co-edited fields; rest single-writer-mutable | scope the prune-boundary cost |
| Professional axis (§J) | Different platforms need different lexicons | One lexicon, parameterized by policy and defaults | Verified (D-5) |

---

## M. Residuals owned

- **Public immutability is not enforced by the substrate.** On the public ride, grow-set behavior is Drystone's discipline; the substrate permits update and delete and leaves no tombstone, and cannot be relied on to refuse a delete of an author's own record. Any correctness assumption of immutability lives in Drystone's layer.

- **Public sync semantics are in flux.** The union-of-logs framing holds at the commit level today, but the wire mechanism is diff-based and evolving; public interop code must track the ATProto sync direction rather than assume a fixed per-author log on the wire.

- **A helper's clear-text access is a delegation, not a structural tier, and never authority.** Two distinct helper roles must be kept apart. A *distribution* helper that was not admitted to the content scope must serve only ciphertext, per the §H invariant; letting such a helper serve cleartext would rebuild the visibility flag. A *content* helper that a persona or a k-of-n group deliberately admits to the scope (a search or index helper, for example) may hold clear text, because it holds it by the same grant any member holds keys by, and it is revocable. That second case does not contradict the §H invariant; it is a different helper with a different grant. In neither case does the helper gain authority: it cannot foreclose on a persona or govern, and it is not a center however capable (authority-and-complement.md). So Drystone does not structurally forgo server-side capability over private content; the peers may delegate clear text to a helper to get search, indexing, or aggregation. The tradeoff is honest: delegating clear text expands the confidentiality surface to that helper, since its compromise leaks what it was shown, while the content-blind store forgoes that capability to keep the surface minimal. Both are center-free; the peers choose per helper. The AppView-provisioned-audience scope-key question (Open items) is the remaining unresolved piece of the distribution case. The content-helper case is now demonstrated end to end: a helper admitted by a real MLS Welcome decrypts group records as any member does, feeds the *same* disposable index the public source feeds (one query returning both a public and a helper-fed hit), and is made **forward-blind** by revocation — after a membership removal rolls the epoch, MLS forward secrecy denies the helper the later records so no rows land, while what it was already shown stays indexed (revocation is forward-only; what the helper was shown, it was shown). The helper holds no authority surface — join and ingest only. `green-real` (index-by-grant + revocation forward-blindness, on real openmls at loopback / in-process).

- **The multi-writer chrome is weak.** Even the one genuinely multi-writer object is mostly single-writer in practice, so the CRDT is scoped to a few fields and should be monitored for whether it earns its place at all.

---

## Open items

- **The AppView-provisioned scope key (the open thread from this cycle).** For the private-AppView and hybrid topologies (§H), what the scope key protecting an AppView-served audience actually *is*, and how it is granted to and rotated among authorized readers, is unresolved. This is the item that determines whether the AppView path can match the MLS-group path's cryptographic confidentiality or only approximate it: if the audience shares a provisioned scope key with proper rotation on membership change, the AppView path is as strong as the MLS path with more flexible audience definition; if not, it degrades toward trusted-gatekeeper confidentiality. Until this is pinned, the MLS-group topology is the safe default, and the AppView topology SHOULD be used only in the hybrid form (MLS-group sealing, AppView serving). `Design; open.`

- **Group.profile field-level CRDT scoping (§I).** Exactly which chrome fields are co-edited (merge CRDT) versus single-writer-mutable, and the merge semantics for the co-edited subset. `[confirm.]`

- **Membership view versus Arbiter projection reconciliation (§I).** How the private governance-chain member list and a public Arbiter-style XRPC projection are kept consistent for interoperability without the projection becoming a second source of truth. `Design; open.`

- **Moderated-group post admission (§F).** The relation by which a moderated group accepts or hides a referenced post (a posting capability issued under a role, or a gating-fact folded by honest clients), which is the write-gate half of group membership for content. `[confirm.]`

- **MST fanout (minor).** The research flagged a documentation conflict: the ATProto specification states a Merkle-tree fanout of 4, while a widely-circulated implementation package doc states 16; the specification governs. Noted only so public-interop implementers trust the spec. `Verified.`

---

## Changelog

`Working draft; per doc-method §1, transitions are recorded here so the body stays an end state.`

- **Draft, first consolidation.** Establishes the per-author-union equivalence and its LWW limit (§B), the content-plane flavor mapping (§C), the reuse-plus-custom lexicon strategy (§D), the interaction-gate clarification (§E), the group-annotation construct (§F), the private-overlay-by-reference construct (§G), the overlay distribution topologies with the encryption-is-the-boundary invariant (§H), and the group layer mapped to governance and flavors (§I). Consolidates the professional-versus-policy finding (§J) and the prior art (§K).

- **Provenance.** The ATProto and platform facts here are `Verified`: verified in an external research pass against atproto.com/specs and the platforms' own documentation, not re-fetched this cycle. The MLS and Willow facts these constructs rely on are `Verified` directly, in the companion docs (asset-keying §M-equivalent and history-durability §M). The mapping onto Drystone flavors and governance, and the overlay and annotation constructs, are `Design`.

- **Companion linkage.** Records here are sealed under the per-scope asset keys of asset-keying.md and carried by the transport and stores of history-durability.md. The private-overlay distribution topologies (§H) are a specialization of the history-durability tier model. Repoint against Part 2 §5.7 (k-of-n membership) and §7.6 (fork) when this material is folded.

- **Tag posture.** Nothing here is `green-real`. External facts are `Verified` or `Verified`; constructs and mappings are `Design`; the AppView scope key, chrome CRDT scoping, membership-projection reconciliation, and moderated-post admission are open or `[confirm]`.

- **Doc-model normalization.** Normalized epistemic tags to the suite's canonical set for spec-readiness: the compound `design/Synthesis` (and `design/ENABLING`) became `Design`, `Verified, research pass` became `Verified`, and bare lowercase `design` became `Design`. The `Realizes: P-<Principle>` linkage tags are retained as the principle-crosswalk layer, which now resolves against the catalog in authority-and-complement.md. No substantive claims changed.
