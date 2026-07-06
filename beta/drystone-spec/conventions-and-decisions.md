# Drystone: terminology conventions and synthesis decisions

`Status: living reference, produced alongside the Part 1 / Part 2 synthesis`

`Two jobs: (1) a decision record for the consolidation of the twelve-document suite into a self-contained Part 1 and Part 2, and (2) a terminology-and-conventions primer that folds into the document-structure guidance (companion to 11-doc-method.md) and that a later consistency pass reads the docs *against*.`

`Companion to: drystone-part1.md, drystone-part2.md, 11-doc-method.md`

---

## Why this document exists

The synthesis that produced the self-contained Part 1 and Part 2 required a large body of decisions: which of two conflicting definitions supersedes, where a folded-in mechanism lands, which open questions dissolve once context is pooled, and, most pervasively, how a handful of load-bearing words are bounded so they stop colliding. That reasoning is a deliverable, not scratch work. Keeping it only in the session that produced it would violate the same discipline `11-doc-method.md` argues for: the design docs are end states, and the reasoning about how they reached that state belongs in a durable, separate record.

This document is that record, and it is deliberately two things at once:

- A **synthesis decision record**: what was folded, what superseded what, which conflicts were resolved and how, and which hard cases dissolved once the pooled context made them resolvable. This is the auditable "how it was done."

- A **terminology-and-conventions primer**: the bounded-context rules for the load-bearing words, stated as rules with a test for each, so a later pass (or a different instance of the same kind of work) can check the docs against them and catch the specific failure modes: a term that slipped its boundary, or the same thing defined twice under two names.

The primer half is the part that carries forward. It is meant to fold into the document-structure guidance so the vocabulary discipline becomes a checkable practice, exactly as `11-doc-method.md` argues a stated practice should.

---

## Part A. Terminology conventions (the primer)

The organizing idea, and the reason this section is framed the way it is: several of the hardest reconciliations were not disagreements about the design but **general-language collisions**, one word carrying two or three referents that live at different layers. The fix in each case is the same move domain-driven design makes at a bounded-context boundary: name the layer each sense belongs to, and let the typography (case, or a qualifier) mark which layer a sentence stands on. The reader learns one genus/instance habit and applies it across every collided word.

### A.1 group (lowercase) vs Group (capital G)

**Two referents, same collective, two layers.**

- **group** (lowercase) is the *social* sense: a body of people, manifested as personae, who communicate, coordinate, and govern together. The real-life grouping the whole design exists to serve. When a lowercase group "decides something," that is a social event among humans.

- **Group** (capital G) is that same collective *once it is a principal in the system*: a bounded entitlement-and-governance unit that can hold assets, be granted to, and act as a single unit, with fixed boundaries that make its tradeoffs real. The capital-G Group is what Part 2 realizes over an MLS group (§5.10). Governance and entitlement facts attach here.

**The test.** Is the sentence about the collective *as a body of people* (they communicate, they raise a hand and decide), or about the collective *as a bounded principal in the system* (it holds assets, its governance strips a grant, it forks with its history intact)? People → lowercase group. In-system actor → capital-G Group.

**The seam that makes the distinction load-bearing.** A social group of people can raise a hand and decide "Bob hosts the relay." The capital-G Group can make the *same* decision, but its mechanics are fixed by technical truth: it is either a Group-governance act (a Group Role grant) or a change to the scope (below). Same human decision, different mechanical realization; the case marks which layer the sentence describes.

**A note on the third, generic sense.** In the recursion argument ("a principal is recursively a Group"), the collective-as-principal sense is meant, so it is capital-G. Where an illustrative clause names a plain social body ("a community is a group of people"), it is lowercase. The two can sit in one sentence; set each clause by its sense.

### A.2 role (lowercase) vs Group Role (capital G, capital R)

**Same genus/instance move as group/Group, boundary in a different place.**

- **role** (lowercase) is the *genus*: delegated authority as a category, the kind-of-thing that a right is contrasted *with* when the two categories are being defined.

- **Group Role** (capital G, capital R) is a *concrete grant inside a Group*: admit or remove members, moderate, gate, hold the act-for-the-Group authority, issue capabilities. Granted by member consent, scoped, attenuating, revocable.

**The test.** Is the sentence about the *category* (role as the thing rights are not), or about a *grant operating inside a Group* (something a Group's governance conferred and can strip)? Category → lowercase role. Grant-in-a-Group → Group Role.

**Why the boundary sits where it does.** A **right** is standing in the *system*, not in any one Group. It is what the fork carries into both descendants, which is exactly why a Group's governance can never reach it. A **Group Role** is stripped *inside* a Group, by that Group's governance. So "a right survives even when every Group Role is stripped" is sharper than the lowercase version: it contrasts a system-scoped thing (right) against a Group-scoped thing (Group Role), and the scope difference is *why* one is inalienable and the other revocable. The ambiguity the lowercase-everywhere version hid was precisely this scope difference.

**The distinctness payoff.** Capitalizing the concrete grant keeps it from colliding with the *ecosystem permissions* a non-Group principal holds (A.4), which are authority of a different kind and are explicitly not Group Roles. This makes the Part 2 Appendix D invariant "only a principal holds a role" precise: only a Group member holds a Group Role.

### A.3 scope

**scope is *wider* than a Group.** It is the whole envelope of exposure and processing for a Group's messaging: the reach of routing and metadata beyond the entitled membership, taken across the delivery fabric as a whole (gossip, direct connections, relays, meers, push-notify nodes, and the metadata each sees). A node that carries or triggers on a Group's traffic without being a Group member (a relay, a meer, a push-notify node) sits in the Group's **scope** but not in the **Group**. No single mechanism defines scope: the gossip topic is a large contributor (A.3, gossip note below), but each helper in the path adds to the envelope independently.

**A single helper can sit in the scope of several Groups at once.** One relay or meer can be in scope for many Groups simultaneously, because scope is exposure reach and is not bounded by any one Group's membership. This is the clean tell that scope and Group are different: if a helper's standing were a Group-governance fact, cross-Group sharing would be incoherent (which Group governs the shared helper?); it is coherent precisely because the helper sits in the delivery fabric carrying sealed bytes, and its presence in scope is a fabric-level fact, not a per-Group or per-persona grant.

**A helper's presence in scope is a fabric-level fact; a persona's *use* of it is a per-persona decision. These are different layers.** A persona cannot unilaterally remove a meer from a Group's scope, any more than it can stop a swarm node from seeing the sealed envelope pass through; the exposure exists at the delivery-fabric level and one persona declining to interact does not change it for anyone. What a persona *can* do is withdraw its own use and reliance (stop calling in for held messages, stop depending on the helper, decline to interact). So the correct framing is **not** "meer adoption is per-persona" (its scope presence is a fabric fact, not adopted per persona) but "**respecting and using** the meer is a per-persona, client-level decision." Only the trust-and-use layer is the individual's; the scope-presence layer is not. (This corrects an earlier over-statement in this reference and in Part 2 §5.4 that framed the node-local act as "removing the meer from scope.")

**The gossip topic is a major contributor to scope, not synonymous with it; its mapping to Groups is configurable, and the Group-ID derivation is a reference-realization detail, not the model.** Scope is the whole exposure-and-processing envelope, the delivery fabric as a whole (gossip, direct connections, relays, meers, push-notify nodes, and the metadata each sees). The gossip topic is **one large factor** in that envelope (it governs swarm membership, hence who sees the sealed traffic and its routing metadata), but a relay, meer, or push-notify node in the path adds to scope independently of the topic, so topic and scope are not the same object. The reference profile derives one gossip topic per Group from the Group's ID (`H(tag ‖ group_id)`), a reasonable default the testing implementation uses, but the model does not couple scope to Group one-to-one: a deployment may run one topic across several Groups, one per Group, and may include or exclude a given helper from the path. What the model requires is only a high-entropy topic seed. Treating "the reference profile keys the topic from the Group ID" as "scope is Group-specific" is the realization-for-requirement slide (Part 2 §4.2, §10). The clean tell that topic alone does not fix scope: the same Group on the same gossip topic has a wider or narrower scope depending on which helpers are in the path.

**The attach rule (the load-bearing division).**

- Governance and entitlement facts attach to the **Group**: a Group forks, holds full history, sets thresholds, recognizes its personae, adopts and revokes Group Roles.

- Exposure and routing-reach facts attach to the **scope**: the scope of a gossip topic, per-scope metadata exposure.

**Worked disambiguation, the escalation tolerance.** The false-positive / escalation tolerance (how readily a concurrent contradiction escalates to humans) is a **per-Group** governed policy, not per-scope. It lives in the governance chain, manifest in the thresholds and dials, and is a function of the Group's purpose and the human relationships inside it. Scope is the wrong home because scope is broader than the Group and includes relays and carriers that have no bearing on escalation likelihood: a relay's presence in the scope does not change whether two personae kicking each other is likely, but the k-of-n threshold does, and that is Group governance. (This corrects earlier drafts, including the delivery-suite docs, that used "scope" and "group" interchangeably.)

### A.4 principal, and permission across planes

**principal is the broad frame** (deliberately AWS-shaped): the actor we reason about in the system, reasoned about *through the permissions it carries*. The genus definition is therefore **permission-holding entity, identified by one key-lineage**, not "role-holding entity", because "role" (Group Role) is a governance-plane term and a non-Group principal like a meer holds permissions but no Group Role. Its **Group-governance instance** is the tightened definition, a role-holding entity identified by one key-lineage. These are not in tension: the tightened version is the broad frame inside the governance-bounded context.

**There are principals not within any Group, and principals within a Group whose authority does not reach outside it.** The spec must say which is which rather than letting one word flatten them.

- A **meer** is a principal in the *broad* sense (an actor with permissions) and *not* a principal in the *Group-governance* sense (it holds no Group Role, right, or weight in any Group). Both are true because they are claims on two different planes. The earlier apparent contradiction ("is the meer a principal?") was a plane confusion, not a real conflict.

**"Permission" spans planes; role/right/weight are Group-governance-plane terms.** A meer holds enumerable **ecosystem permissions** (be-in-swarm, talk-to-a-push-node, and that push node in turn talk-to-an-external-third-party), which are connectivity/delivery permissions, not in-Group governance. Pinning "Group Role," "right," and "weight" to the governance plane is what keeps the meer infrastructure while still letting the spec reason about it as an actor with real, enumerable, revocable ecosystem permissions.

### A.5 The two revocation planes

Revocation is not one mechanism; it lives at two layers with different actors, different mechanics, and different authority sources. Both are grounded in the same place: whether and how to revoke is a **social-utility judgment** about trust and tradeoffs, not a computation (the §2.0 razor, one layer down). The machinery makes each response cheap and available; which response is right (move to a new relay, split across several, pull it in-house, one of *n* combinations) is a judgment, which is exactly why it cannot be deterministic.

- **Group-governance revocation.** Acts *inside* a capital-G Group, on Group facts: a Group Role, or membership. Runs through the Group's replicated policy and the k-of-n threshold counted per persona; evaluated against the Group's governance. Global to the Group by construction. (Part 2 §5.7, §5.8.)

- **Node-local withdrawal of use.** Acts at the layer of the software an individual persona runs as a **standalone authoritative node**. It is how a persona withdraws its own reliance on a *non-Group* helper: it stops calling the helper, stops depending on it for durability, declines to interact. Its authority comes from the standalone-authoritative-node premise, so it needs no threshold and no governance round. It is **not** removal of the helper from scope: the meer remains in the delivery fabric and still sees whatever routing metadata the fabric exposes. It is the **exit right exercised at the client level**, a recognition of the persona's autonomy over its own node, local to the withdrawing persona by construction. (Architecturally this holds by construction, since a persona controls whether its own client calls, answers, or connects to a helper, even where a dedicated affordance is not yet implemented.)

**The asymmetry is the tell they are two planes.** A Group Role revocation is global to the Group and changes a Group fact; a node-local withdrawal is local to the one persona and changes only that persona's behavior, leaving the helper in scope. This is the concrete cash-out of "you are a standalone authoritative node": the ability to unilaterally stop relying on a helper, without waiting on the Group, is what distinguishes a standalone node from a client dependent on a shared configuration it cannot override.

**Use is per-persona; scope-presence is not; and availability is a resource, not a Group Role.** Two corrections carried from the pass. (1) Whether to *use* a meer is per-persona (the trust-and-use decision at the client), but the meer's *presence in scope* is a fabric-level fact no single persona controls, so do not say "adoption is per-persona" or "withdrawal removes it from scope" (A.3). (2) In the exitability treatment (Part 2 §5.9), separate the **read / search-offload Group Role** (a governance grant, revocable under threshold, rotates the epoch) from **availability** (a blind *resource*, §5.4, needing no grant); an earlier sentence wrongly called availability a delegated "role." What a member can always do with a blind-availability helper is withdraw use and shift durability elsewhere, which is the node-local act above, and this per-member freedom aggregates to the Group-level property that the Group is never hostage to any one helper.

### A.6 persona (unchanged; recorded so it is not re-litigated)

Part 2's definition is correct and load-bearing; the synthesis does **not** loosen it. A **persona** is the human layer's manifestation in the system, a principal by virtue of *one root key pair* (its lineage root), from which its devices' and clients' membership keys descend by signed credential, counted **one per rooting key pair** regardless of device or client count. The root key pair is the persona's representation in the world, which is what lets an escalation terminate in a decidable locus (a persona can make representable decisions only because it has a representation) and what lets weight be counted per-lineage so governance stays consistent. "Persona" and "personhood" stay separate words: the lineage is technically representable and counted with certainty; whether a lineage corresponds to a distinct human is a group judgment, never a protocol fact.

### A.7 Coinages promoted to first-class Part 2 vocabulary

These originate in the delivery suite (`01`, `09`) and are promoted into Part 2 §6 and the Appendix D term lattice:

- **Delivery Fabric (DF)**: the blind, content-agnostic routing/dissemination overlay (gossip, direct links, relays) that moves sealed messages. Larger than and overlapping the Groups that run over it; carriers see ciphertext and routing metadata at most. Distinct from the meer (the fabric *carries*; the meer *holds*).

- **The C- / D- / P- plane prefixes**: Carriage (C-direct, C-swarm, C-relay), Durability (D-self, D-meer, D-peer), Presence (P-none, P-gossip, P-meer, P-push). A delivery arrangement is one choice from each plane; the planes are independent axes except where one mechanism serves two by construction, which the text flags.

- **gap-aware history convergence**: the one named mechanism behind C-swarm hole-detection, D-peer, and device-Group sync: detect a nameable gap (via the per-author high-water mark, which *is* the dataplane half of the (G, D) cursor) and fill it from a self-verifying source (via range-based set reconciliation).

Naming discipline (from `09`): established literature terms for inherited primitives (gossip / epidemic dissemination, anti-entropy / state reconciliation, eventual consistency); coinage only for Drystone's own compositions (Delivery Fabric, gap-aware history convergence). "DS" stays the upstream MLS role; the meer is Drystone's store-and-forward instantiation of the half it keeps; the DF is the routing overlay, a distinct concept.

---

## Part B. Synthesis decisions (the record)

### B.1 What was folded into what

(To be completed as each fold lands. Structure: source doc → destination section, with the superseding note where a definition changed.)

- Delivery architecture (`01`): three-plane model, Delivery Fabric, gap-aware history convergence, D-peer, device-Group-as-durability-amplifier → Part 2 §6.

- MLS overview/terms (`mls-overview-and-terms`) and hard cases (`mls-hardcases-and-posture`): the client-as-keys model, the two-service architecture, the ten hard cases and the alignment table → Part 2 §7 / §8 / §10.2, with the alignment table inline in §10.2.

- History modes (`07`) and side histories (`12`): forward-only vs Willow-mutable, the threading tiers → Part 2 §7 body, open items to Appendix B.

### B.2 Definitions that superseded a looser prior

- **principal**: the delivery-suite "anything that holds a permission set (incl. helper nodes)" is reconciled, not discarded: it is the broad-plane frame, with the tightened "role-holding entity, one key-lineage" as the Group-governance instance. The meer is a broad-plane principal and not a Group-governance principal. (See A.4.)

- **Group vs scope**: earlier drafts (including the delivery suite) used "scope" and "group" interchangeably. They are now two bounded terms (A.1, A.3): Group is the entitlement-and-governance unit; scope is the wider exposure extent.

- **persona**: Part 2's sharp definition stands; no regression. (A.6.)

- **PrincipalSet renamed to Group Role Set** (capital S, first-class three-word term): a named bundle of Group Roles that travel together, grantable/revocable as one unit, with mutual-exclusion constraints for separation of powers inside a Group ("a holder of this Set may not also hold that Group Role"). Motivated by the human-fatigue goal: people reason about "admin," not fifteen individual dials, even though the dials are tracked individually at the provenance level. Carried as a **named-but-settling** concept (design-flagged); full mechanism shakes out in Part 2 §5. The old name wrongly suggested a set of principals; it is a set of Group Roles.

### B.2.1 Clarifications and rewrites (not renames, but improvements surfaced during the pass)

- **"By necessity" weight-equality phrasing rewritten.** The carried phrase "equal in rights and (by necessity) weight" read as a *constraint tolerated* ("stuck with") when the intended meaning is the opposite: weight-equality is a *consequence* of rights-equality (standing-to-participate and standing-to-be-counted are the same fact), and specifically a consequence the design's *principles entail*, not an independent rule imposed. Rewritten across Part 1 (§2.3 header, Weight bullet, summary aphorism) to carry the consequence-of-principle logic. Carry the same fix into any Part 2 echo.

- **Dual-domain treatment for aligned concepts (the right/role Reasoning paragraph).** Where a concept has an aligned meaning in *both* the social/epistemological domain and the technical domain, Part 1 now makes the alignment explicit rather than implicit: a *right* and a *role* are first stated as the layer-independent social distinction (standing vs delegated authority), then Drystone's **Group Role** is named as that social *role* made technical *with fidelity*, passing the same removal-leaves-standing test. The mechanism is trustworthy exactly because the wire-level right/Group-Role distinction reproduces the social standing/authority distinction rather than inventing a different one. This "social reality and its faithful technical mirror" framing is the organizing move of Part 1 and should be applied wherever a term spans both planes.

- **Ostrom grounding: lowercase group preserved, capital-G Group linked as realization.** Ostrom's principles are about human communities (the social plane); capitalizing "group" there would overwrite her semantics and break the homage. The fix links the two planes instead: her value stated in the lowercase social sense, with the capital-G Group named as the *realization* of the value (the §7.6 hard-stop surfacing to the affected group, adjudicated within the Group that manifests them; the unforbiddable fork). General rule: when referencing an external source that grounds a *value*, keep the source's own (social) semantics and link the technical realization rather than recasting the source in system terms.

- **"The Group holds keys" avoided in favor of "the members hold the keys."** Because a capital-G Group can itself be a principal (a possible actor), any sentence like "the Group holds keys" is ambiguous between the Group-as-principal holding a credential and the member clients (leaf nodes) holding the group key. Where the intended referent is the latter (e.g. material reversibility: no helper holds the data hostage because the members hold the decryption keys), name the actual key-holders. **Standing vigilance note:** the Group-as-possible-principal is a recurring source of actor-ambiguity; wherever the Group could be read as the actor, differentiate explicitly (member clients vs Group-principal).

### B.3 Retractions carried forward

- **The "forward secrecy vs durable history" tension is retracted.** Forward secrecy is defined against *ciphertext* retention, so retaining sealed bytes is FS-safe. The real friction is **key retention under reordering** (the documented decentralized-MLS problem) and the persistently-offline member. Where the hard-cases material is folded, the corrected framing is carried and the item is named "key retention under reordering," not "FS vs durable bytes." (Source: session summary §5; hard-cases §7 body already states it correctly; the hard-cases posture-table row is the only place still using the old framing and is corrected in the fold.)

### B.4 Hard cases that dissolve once context is pooled

Confirmed for the fold:

- **External-join / stale GroupInfo residual** (hard-cases §5): answered by Part 2 §7.4's monotonic-fold property (incompleteness under-authorizes, never mis-authorizes). Downgrades from open thread to a stated consequence with a named residual only at the exact corner (a node behind the relevant epoch).

- **Insider replay / nonce reuse on restore** (hard-cases §6): resolves into Part 2 §6.5.3's "meer stores byte-identical sealed bytes, never re-sealed" rule plus out-of-band convergence. The residual ("does any path restore live epoch secrets in place?") becomes a single normative MUST-NOT in the catch-up section.

- **epoch_authenticator "underused"** (hard-cases §10): folded in as a *decided adoption*, Part 2's whole-Group consistency need cites the MLS epoch_authenticator (RFC 9420 §8.7) directly rather than parallel-building.

- **Under-determination as a second escalation member** (hard-cases §3): promoted into Part 2 §7.6 as a named companion to the mutual-expulsion case.

### B.5 Open items preserved (not resolved by the fold)

Carried to Part 2 Appendix B rather than smoothed: tenure-under-re-key; the KeyPackage-exhaustion seating trilemma; the ReInit non-atomicity completion (intent-recorded-before-freeze); the re-plant seating default; the communal-namespace key construction under membership change; history-mode migration (forward-only vs Willow-mutable, suspected fixed-at-creation); whether tier-2 side histories deserve a first-class construct; self-destruct semantics; the resumption-PSK cross-group linking; the RBSR production construction; and the standing `[confirm]` external-fact set (Matrix MSCs/CVEs, Beer/Cybersyn/OGAS, decentralized-MLS drafts, and the iroh-gossip/address-lookup crate specifics).

**Helper governance and alignment (a first-class concern, newly named).** Because scope is broader than a Group and a single helper (meer, relay, push-notify node) can be in scope for many Groups at once, and because scope includes these helper roles, the **operation and governance of helper nodes meaningfully shapes the scope of exposure across the whole system**, at a layer no individual persona's node-local withdrawal of use can dissolve. A persona declining to interact is the individual backstop (the exit exercised at the client); it is not a substitute for the question of who runs these helpers and whether they are ideologically and operationally aligned. That question is therefore first-class in its own right, not something each Group settles internally. Named here and flagged in Part 2 §5.4; the mechanism is deliberately not specified in the terminology pass (capture-and-defer, not solve).

---

## Part C. How to use this document in a later pass

Read the docs *against* Part A. For each load-bearing word, the test is stated; a usage that fails the test is a slip to fix. The specific failure modes to hunt: a term that has slipped its bounded context (a scope-level fact attached to a Group, or vice versa); the same referent defined twice under two names; a coinage used before it is introduced; and a superseded definition left co-existing with its replacement. Part B is the record of what was already decided, so a later pass does not re-lit­igate a settled call or accidentally reverse a supersession. This is the same relationship `11-doc-method.md` has to the design docs: a stated practice you can check for.
