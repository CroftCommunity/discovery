# Drystone: terminology conventions and synthesis decisions

`Status: living reference, produced alongside the Part 1 / Part 2 synthesis`

`Two jobs: (1) a decision record for the consolidation of the twelve-document suite into a self-contained Part 1 and Part 2, and (2) a terminology-and-conventions primer that folds into the document-structure guidance (companion to doc-writing-method.md) and that a later consistency pass reads the docs *against*.`

`Companion to: p10-full-part1-principles.md, p10-full-part2-mechanics.md, doc-writing-method.md`

---

## Why this document exists

The synthesis that produced the self-contained Part 1 and Part 2 required a large body of decisions: which of two conflicting definitions supersedes, where a folded-in mechanism lands, which open questions dissolve once context is pooled, and, most pervasively, how a handful of load-bearing words are bounded so they stop colliding. That reasoning is a deliverable, not scratch work. Keeping it only in the session that produced it would violate the same discipline `doc-writing-method.md` argues for: the design docs are end states, and the reasoning about how they reached that state belongs in a durable, separate record.

This document is that record, and it is deliberately two things at once:

- A **synthesis decision record**: what was folded, what superseded what, which conflicts were resolved and how, and which hard cases dissolved once the pooled context made them resolvable. This is the auditable "how it was done."

- A **terminology-and-conventions primer**: the bounded-context rules for the load-bearing words, stated as rules with a test for each, so a later pass (or a different instance of the same kind of work) can check the docs against them and catch the specific failure modes: a term that slipped its boundary, or the same thing defined twice under two names.

The primer half is the part that carries forward. It is meant to fold into the document-structure guidance so the vocabulary discipline becomes a checkable practice, exactly as `doc-writing-method.md` argues a stated practice should.

---

## Part A. Terminology conventions (the primer)

The organizing idea, and the reason this section is framed the way it is: several of the hardest reconciliations were not disagreements about the design but **general-language collisions**, one word carrying two or three referents that live at different layers. The fix in each case is the same move domain-driven design makes at a bounded-context boundary: name the layer each sense belongs to, and let the typography (case, or a qualifier) mark which layer a sentence stands on. The reader learns one genus/instance habit and applies it across every collided word.

### A.1 group (lowercase) vs Group (capital G)

**Two referents, same collective, two layers.**

- **group** (lowercase) is the *social* sense: a body of people, manifested as personae, who communicate, coordinate, and govern together. The real-life grouping the whole design exists to serve. When a lowercase group "decides something," that is a social event among humans.

- **Group** (capital G) is that same collective *once it is a principal in the system*: a bounded entitlement-and-governance unit that can hold assets, be granted to, and act as a single unit, with fixed boundaries that make its tradeoffs real. The capital-G Group is what Part 2 realizes over an MLS group (§5.10). Governance and entitlement facts attach here.

**The test.** Is the sentence about the collective *as a body of people* (they communicate, they raise a hand and decide), or about the collective *as a bounded principal in the system* (it holds assets, its governance strips a grant, it forks with its history intact)? People → lowercase group. In-system principal → capital-G Group.

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

**principal is the broad frame** (deliberately AWS-shaped): the entity we reason about in the system, reasoned about *through the permissions it carries*. The genus definition is therefore **permission-holding entity, identified by one key-lineage**, not "role-holding entity", because "role" (Group Role) is a governance-plane term and a non-Group principal like a meer holds permissions but no Group Role. Its **Group-governance instance** is the tightened definition, a role-holding entity identified by one key-lineage. These are not in tension: the tightened version is the broad frame inside the governance-bounded context.

**There are principals not within any Group, and principals within a Group whose authority does not reach outside it.** The spec must say which is which rather than letting one word flatten them.

- A **meer** is a principal in the *broad* sense (an entity with permissions) and *not* a principal in the *Group-governance* sense (it holds no Group Role, right, or weight in any Group). Both are true because they are claims on two different planes. The earlier apparent contradiction ("is the meer a principal?") was a plane confusion, not a real conflict.

**"Permission" spans planes; role/right/weight are Group-governance-plane terms.** A meer holds enumerable **ecosystem permissions** (be-in-swarm, talk-to-a-push-node, and that push node in turn talk-to-an-external-third-party), which are connectivity/delivery permissions, not in-Group governance. Pinning "Group Role," "right," and "weight" to the governance plane is what keeps the meer infrastructure while still letting the spec reason about it as a principal with real, enumerable, revocable ecosystem permissions.

### A.5 The two revocation planes

Revocation is not one mechanism; it lives at two layers with different principals, different mechanics, and different authority sources. Both are grounded in the same place: whether and how to revoke is a **social-utility judgment** about trust and tradeoffs, not a computation (the §2.0 razor, one layer down). The machinery makes each response cheap and available; which response is right (move to a new relay, split across several, pull it in-house, one of *n* combinations) is a judgment, which is exactly why it cannot be deterministic.

- **Group-governance revocation.** Acts *inside* a capital-G Group, on Group facts: a Group Role, or membership. Runs through the Group's replicated policy and the k-of-n threshold counted per persona; evaluated against the Group's governance. Global to the Group by construction. (Part 2 §5.7, §5.8.)

- **Node-local withdrawal of use.** Acts at the layer of the software an individual persona runs as a **standalone authoritative node**. It is how a persona withdraws its own reliance on a *non-Group* helper: it stops calling the helper, stops depending on it for durability, declines to interact. Its authority comes from the standalone-authoritative-node premise, so it needs no threshold and no governance round. It is **not** removal of the helper from scope: the meer remains in the delivery fabric and still sees whatever routing metadata the fabric exposes. It is the **exit right exercised at the client level**, a recognition of the persona's autonomy over its own node, local to the withdrawing persona by construction. (Architecturally this holds by construction, since a persona controls whether its own client calls, answers, or connects to a helper, even where a dedicated affordance is not yet implemented.)

**The asymmetry is the tell they are two planes.** A Group Role revocation is global to the Group and changes a Group fact; a node-local withdrawal is local to the one persona and changes only that persona's behavior, leaving the helper in scope. This is the concrete cash-out of "you are a standalone authoritative node": the ability to unilaterally stop relying on a helper, without waiting on the Group, is what distinguishes a standalone node from a client dependent on a shared configuration it cannot override.

**Use is per-persona; scope-presence is not; and availability is a resource, not a Group Role.** Two corrections carried from the pass. (1) Whether to *use* a meer is per-persona (the trust-and-use decision at the client), but the meer's *presence in scope* is a fabric-level fact no single persona controls, so do not say "adoption is per-persona" or "withdrawal removes it from scope" (A.3). (2) In the exitability treatment (Part 2 §5.9), separate the **read / search-offload Group Role** (a governance grant, revocable under threshold, rotates the epoch) from **availability** (a blind *resource*, §5.4, needing no grant); an earlier sentence wrongly called availability a delegated "role." What a member can always do with a blind-availability helper is withdraw use and shift durability elsewhere, which is the node-local act above, and this per-member freedom aggregates to the Group-level property that the Group is never hostage to any one helper.

### A.6 persona (unchanged; recorded so it is not re-litigated)

Part 2's definition is correct and load-bearing; the synthesis does **not** loosen it. A **persona** is the human layer's manifestation in the system, a principal by virtue of *one root key pair* (its lineage root), from which its devices' and clients' membership keys descend by signed credential, counted **one per rooting key pair** regardless of device or client count. The root key pair is the persona's representation in the world, which is what lets an escalation terminate in a decidable locus (a persona can make representable decisions only because it has a representation) and what lets weight be counted per-lineage so governance stays consistent. "Persona" and "personhood" stay separate words: the lineage is technically representable and counted with certainty; whether a lineage corresponds to a distinct human is a group judgment, never a protocol fact.

**`actor`, `persona`, and `node` kept distinct.** `actor` is reserved for a named member of the demonstrative running-example cast (Part 2 §3.1); the identity term proper is `persona` (plural `personae`), and a clause stating an identity or authority mechanic uses persona, or `principal` (A.4) in the broad frame, never actor. A `node` is a device or a helper (A.7, and the node-role cast fixed in Part 2 §3.2), never the person. So the running example says "Alice," the mechanics say "the persona," and the wire says "the node," and the three never substitute for one another.

### A.7 Coinages promoted to first-class Part 2 vocabulary

These originate in the delivery suite (`01`, `09`) and are promoted into Part 2 §6 and the Appendix D term lattice:

- **Delivery Fabric (DF)**: the blind, content-agnostic routing/dissemination overlay (gossip, direct links, relays) that moves sealed messages. Larger than and overlapping the Groups that run over it; carriers see ciphertext and routing metadata at most. Distinct from the meer (the fabric *carries*; the meer *holds*).

- **The C- / D- / P- plane prefixes**: Carriage (C-direct, C-swarm, C-relay), Durability (D-self, D-meer, D-peer), Presence (P-none, P-gossip, P-meer, P-push). A delivery arrangement is one choice from each plane; the planes are independent axes except where one mechanism serves two by construction, which the text flags.

- **gap-aware history convergence**: the one named mechanism behind C-swarm hole-detection, D-peer, and device-Group sync: detect a nameable gap (via the per-author high-water mark, which *is* the dataplane half of the (G, D) cursor) and fill it from a self-verifying source (via range-based set reconciliation).

- **The node-role cast**: the non-adjudicating node roles a Group's traffic uses are a fixed, closed set, named once in Part 2 §3.2 and referenced by name thereafter: relay and swarm node (carriage), meer and durable history store (durability), push notifier (presence), and the read/search helper (a read scope). Each is a scope participant holding a capability or offering a resource but no standing, and every role but the read/search helper is content-blind. Part 2 §3.2 is the canonical home, and a later section names a role rather than re-describing it.

Naming discipline (from `09`): established literature terms for inherited primitives (gossip / epidemic dissemination, anti-entropy / state reconciliation, eventual consistency); coinage only for Drystone's own compositions (Delivery Fabric, gap-aware history convergence). "DS" stays the upstream MLS role; the meer is Drystone's store-and-forward instantiation of the half it keeps; the DF is the routing overlay, a distinct concept.

### A.8 peer (the edge relation, never a principal)

**peer is a *relation*, not a kind of principal.** It is the symmetric standing between two participants on the **transport plane**: two nodes are peers when neither holds privileged or canonical authority over the other, which is the property §3.1 uses to diagnose the system as center-free and §6.1.1 names as *peer-level identity (the transport plane)*. "peer" therefore describes an **edge** (the relationship across a link), not a node and not a principal. Writing "a peer decides" or "the peer holds a Group Role" is a category error: the entity that decides, holds rights and weight, and is escalated to is a **persona** (A.6), a governance-plane principal; the peer is the relation that persona's node stands in with another node.

**The test.** Is the sentence about a *symmetric transport relationship between two endpoints* (they connect, reconcile, gossip, relay for each other), or about an *entity that holds authority and is counted* (it decides, it holds a Group Role, its weight is one)? Relationship on the wire, use **peer** (and *peer-to-peer* for the wiring itself, §6, and *peer-symmetric* or *center-free* for the property). Authority-holding entity, use **persona** (A.6) or, in the broad frame, **principal** (A.4).

**Why the boundary matters, and why the word was easy to slip.** Colloquial "peer-to-peer" uses *peer* as a noun for "one of the machines," which invites the slide from *peer* (a machine at the other end of a link) to *peer* (a principal with standing). Drystone keeps the two apart because its authority model lives on personae counted **one per root key pair** (A.6), while its transport model is symmetric across nodes: the same human can run several nodes (several peers-on-the-wire) that remain **one persona**, one unit of standing. Collapsing the two would let "more nodes" read as "more standing," which is exactly the property the identity model refuses (§5.2, §5.6). So *peer* stays a transport-plane relation, and standing is always spoken of in persona or principal terms.

### A.9 The canonical status-flag ladder (the tag set both parts defer to)

Part 1 and Part 2, and the working documents as they fold, carry **one** status-flag vocabulary, recorded here so a reader who lands in either part reads the same ladder. An earlier draft of Part 2 used a parallel set (`green-real`, `green-model`, `design`, `ENABLING`, `[confirm before publish]`); those are unified into the canonical rungs below (the mapping is in B.6). Each normative claim carries exactly one status flag:

- **`Verified`**: demonstrated against real crypto or real transport in a running reference implementation. (Absorbs the former `green-real`.)

- **`Verified-RFC`**: verified against a normative primary source, for example RFC 9420 or RFC 9750.

- **`Modeled`**: a reference model exists and is reasoning-complete, but is not yet backed by real crypto. (Absorbs the former `green-model`; kept as a distinct rung because "modeled" is more mature than "specified but unbuilt.")

- **`Measured`**: an empirical or benchmark measurement.

- **`Established`**: an established result in the literature, or an inherited primitive used as-is.

- **`Design`**: specified but unproven, a design decision the spec commits to. (Absorbs the former lowercase `design`.)

- **`Synthesis`**: a claim assembled across several sources rather than resting on a single citation.

- **`Load-bearing, unearned`**: a property the design leans on that is not yet earned or proven, flagged honestly rather than smoothed over.

- **`[gates-release]`**: a byte-level encoding or specification item that must be pinned before a publication-final release, and before two implementations can interoperate. (Absorbs the former `ENABLING`; renamed so the reason it blocks release is legible in the tag itself.)

- **`[confirm]`**: rests on an external fact not yet independently verified. (Absorbs the former `[confirm before publish]`.)

Two linkage markers sit alongside the ladder and are **not** status flags: **`Realizes: P-X`** (a Part 2 section names the Part 1 principle it discharges) and the **`P-X` principle codes** themselves (`P-Local-Truth`, `P-Knowable-Truth`, `P-Peer-Equality`, `P-Durable-Enablement`). Normative keywords are **MUST / SHOULD / MAY** (BCP 14).

**Evidence-linkage notes (settled RUN-09, 2026-07-15).** Three conventions govern how a tag points at its evidence; each settles a RUN-08 traceability finding.

- **Forward-link target, per evidence band (FND-T1).** A tag's forward link to its evidence takes one of two forms, chosen by band. An **experiment-earned** tag (a `Verified` resting on real crypto/transport, a `Modeled` reference model, a `Measured` benchmark) resolves to a **named test or report plus its RUN**. A **`Verified-RFC` or otherwise literature-anchored** tag (an `Established` primitive, an RFC/draft/spec-section citation) resolves to its **primary-source anchor** — the RFC and section, or the inherited primitive — which already *is* its correct evidence and needs no experiment pointer. The substrate `Verified` band that is not RFC-anchored is anchored in the feasibility review and the conformance-core (cats 1–6); `EVIDENCE-MAP.md` §D carries that band explicitly.

- **The evidence parenthetical is the recommended forward form, not retrofitted (FND-T4).** An evidence sentence written from RUN-09 onward SHOULD carry the standardized `(evidence: <test or report>, RUN-NN[, grade])` parenthetical. It is **not** back-fitted onto sentences that already carry inline test+RUN prose; adoption is forward-only, so existing tag-adjacent prose is left as-is.

- **Legacy status vocabulary is alpha-tier only (FND-T6).** The former tag words `green-real`, `green-model`, and `not_yet_emitted` (and `PLACEHOLDER`) belong to pre-A.9-unification, alpha-tier, and staging docs, where they are acceptable (B.6 records the absorption: `green-real → Verified`, `green-model → Modeled`). They never appear in a live Part 1 / Part 2 sentence; a token that migrates into one is mapped to its ladder rung.

### A.10 Normative clauses carry their grounding (both MUST and MUST NOT)

A normative clause **MUST** be stated with an affirmative account of *why*, and the requirement is
two-sided. A **MUST NOT** names the concrete failure its breach causes. A **MUST** names why it is
required: what it secures, or what would be lost without it. Neither is a bare directive. The spec is not a
list of things to do and not do; each clause carries the reasoning that makes it a requirement, so a reader
can (1) recognize an inadvertent breach by its symptom, and (2) reason about the impact if the clause was
misunderstood rather than only that it was broken.

Both directions ground out in the same place: the overall design and its principles, which in large part
flow from Part 1. A Part 2 **MUST** discharges a Part 1 imperative (often the one its section `Realizes`),
so its "why" traces to that imperative rather than being invented locally; a **MUST NOT** excludes a
failure those same imperatives forbid. Grounding a clause in Part 1 is therefore the norm, and a clause
whose justification does not trace to a principle is a signal that either the clause or the principle is
missing.

**The test, two-sided.** For a **MUST NOT**, ask: what breaks, concretely, if a node does this anyway? For
a **MUST**, ask: why is this required, what does it secure, and which principle does it serve? If the text
does not answer, the grounding is missing and **MUST** be supplied. "MUST NOT X" fails; "MUST NOT X,
because doing X causes Y" passes. "MUST Z" fails; "MUST Z, because Z secures W (Part 1 §...)" passes.

**Why the rule earns its place.** The whole spec is built on a contrast, what Drystone requires and refuses
and what each buys, so a clause whose grounding is left implicit hides exactly the reasoning the design
rests on. Naming it also makes the clause falsifiable in operation: an implementer who observes symptom Y
has a named clause to look to, and a reviewer can check whether a change quietly reopens Y or drops what a
MUST secured. This convention applies going forward and is the standard a later consistency pass reads the
normative clauses against; a `[confirm]` may mark a clause whose grounding is not yet pinned.

### A.11 The human-adjudication vocabulary and its description rules

Part 2 §7.6 defines the mechanism by which a question the protocol cannot
answer (a utility question) is surfaced to the humans who can. The mechanism
already has a settled vocabulary across both parts; this section makes it a
convention so experiment docs, plans, and test suites stop drifting from it.
The canonical terms, each anchored where it is defined:

- **the seam** — provenance fully settled, utility still open (Part 1 §2.0's
  razor, restated at Part 2 §7.6.1 as "the razor's seam"). The condition that
  triggers the mechanism.
- **the recognizer** — the governed classifier of §7.4.1: a per-Group
  benign-versus-dispute tolerance over verifiable provenance signals, never a
  constant, with the rule that genuinely ambiguous cases escalate. Recognizing
  the seam is itself partly a utility judgment, which is why the recognizer is
  governed rather than fixed.
- **the reconcile hard-stop** — the event (§7.6). Prose spelling is
  *hard-stop*, hyphenated, as noun and verb; snake_case only inside code
  identifiers.
- **the escalation shapes** — Contradiction (too many valid claims) and
  Under-determination (too few valid claims) (§7.6.1). Distinct from the
  **escalation parties**, the personae surfaced by a given case, who are
  presented symmetrically (both parties, no presumed wrongdoer).
- **the algedonic channel** — the lineage name for the designed escalation
  path (Part 1 §3, after Beer); in Part 2 and code, the operational family is
  *escalate / escalation*. One mechanism, one definition, two registers of
  name.
- **the legible picture** — the machine's deliverable: a grounded statement of
  the conflicting facts in governance language, full provenance, no
  editorializing, deterministically identical on every node (§7.6.1,
  P-Knowable-Truth).
- **the local authority** — the holder of the utility judgment on the social
  plane: the principal for its own judgments (A.4's adjudication locus), the
  affected Group for shared ones. Coined here so escalation text has a name
  for the receiving side that is not a node role.
- **planes of authority** — infrastructure authority (held by *operators*)
  and social-utility authority (held by *local authorities*). Operators are
  necessary, and a good operator is aligned to the separation; the design
  constraint is the separation itself: there is no path from the
  infrastructure plane into intra-Group social-utility authority. The two
  roles are mutually exclusive by design, like mutually exclusive Group Roles.
  This names, at the authority level, the same structure A.4/A.5 establish
  for permissions and revocation and A.7's node-role cast establishes for
  standing ("a capability or a resource but no standing").

The reconcile and freshness mechanisms grew a set of load-bearing terms across
the RUN-02..04 spec work; they are added to the shared surface here (same rule
as the terms above: defined once in Part 2, inherited by reference) so
experiment docs, plans, and test suites name them one way:

- **approval subject** — the digest of a policy change's canonical payload
  bytes that an approval fact references, so a quorum is enforced at fold time
  and an approval cannot be replayed onto a different change (Part 2 §7.2 R7).
- **contradiction byte-head** — the order-independent min-hash a reconcile
  hard-stop surfaces as `contradiction:{byte-head}`, byte-identical across fold
  orders, naming the contradiction without ranking the conflicting facts (Part 2
  §7.3.2, §7.6.1).
- **horizon checkpoint** — a §7.3.3 self-checkpoint extended with a manifest of
  the contradiction byte-heads open at a frontier, recorded on the
  reconciliation-horizon cadence; its **horizon-checkpoint manifest** is the
  frontier head paired with the sorted set of those byte-heads (Part 2 §7.6.9,
  a `[gates-release]` wire item, Appendix B).
- **corroboration dials** — the family of Group-governed settings that bound the
  completeness-ahead residual once the §7.4.3 stamp closes the behind-via-traffic
  case: which act classes require final state, the freshness k (or its formula),
  and the solicitation posture; safe at every setting because the fail-closed
  rule is not a dial (Part 2 §7.3.3).
- **quantified trust** — the standing of an answer to a read-side frontier
  solicitation: always an assertion taken at a governed confidence level, never
  proof, which is why the freshness threshold is a dial rather than a constant
  (Part 2 §7.4.1, §7.3.3).

Description rules (checkable; DR numbering is referenced by other docs):

- **DR-1 (one anchor).** The mechanism is defined once, at Part 2 §7.6; every
  other mention points there (doc-method Rule 7). No section re-derives it.
- **DR-2 (terms front-loaded).** The terms above live here and in the Part 2
  term surface; new docs inherit them by reference with a working definition
  (doc-method Rule 2).
- **DR-3 (spelling).** *hard-stop* in prose. No "hard stop", "hardstop", or
  "hard_stop" outside code identifiers.
- **DR-4 (planes of authority).** *Operator* names only the
  infrastructure-plane role. The holder of a social-utility judgment is the
  *local authority* (or *principal* / *affected Group* where the kind
  matters). State the relation as plane separation, never as distrust: write
  "the operator holds no path into social-utility authority" rather than "no
  operator to trust," and scope any trust statement to its plane ("trust for
  what").
- **DR-5 (continuity language, never moral language).** Escalated cases are
  described in symmetric, continuity-framed vocabulary: parties at equal
  standing, divergence, lineage, continuity, departure point. Not used in
  normative text about the mechanism: wrong, offender, violation, punish,
  guilty, or dispute-as-default. Motivation is unprovable from provenance and
  is never asserted. Test: the sentence must read identically well when the
  underlying story is a grievance and when it is routine mechanics (a member
  archiving a Group that removed them under a 30-days-inactive rule).
- **DR-6 (no machine verdicts).** Machine outputs are dispositions, statuses,
  or pictures; never verdicts, rulings, or decisions. Applies to test suites
  and vectors.
- **DR-7 (the recognizer is governed).** Text describing detection of the
  seam states that the classifier is governed per §7.4.1 and that ambiguity
  escalates. Never describe the recognizer as a constant or the protocol as
  "knowing" a dispute occurred.
- **DR-8 (name the shape).** Every escalation mention states which shape it
  concerns: Contradiction, Under-determination, or both.
- **DR-9 (the boundary sentence).** Where the machine/human boundary is
  described, use the three-clause pattern: the protocol surfaces the legible
  picture; the humans supply utility; the instantiation of their choice is
  Drystone's.
- **DR-10 (signal and authority stay local).** Escalation surfaces *to* the
  affected Group; it never relocates the decision *up* to anything.

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

- **peer (newly bounded, not a supersession)**: "peer" was used throughout Part 2 (§3.1's *system of peers*, §6.1.1's *peer-level identity*) with no glossary entry. It is now bounded in A.8 as the **transport-plane edge relation, never a principal**. No prior definition existed to supersede, so this closes a gap rather than resolving a conflict, and it is consistent with the existing §5.2 identity model (standing is counted per persona, never per node), so no §5.2 text changes on its account.

### B.2.1 Clarifications and rewrites (not renames, but improvements surfaced during the pass)

- **"By necessity" weight-equality phrasing rewritten.** The carried phrase "equal in rights and (by necessity) weight" read as a *constraint tolerated* ("stuck with") when the intended meaning is the opposite: weight-equality is a *consequence* of rights-equality (standing-to-participate and standing-to-be-counted are the same fact), and specifically a consequence the design's *principles entail*, not an independent rule imposed. Rewritten across Part 1 (§2.3 header, Weight bullet, summary aphorism) to carry the consequence-of-principle logic. Carry the same fix into any Part 2 echo.

- **Dual-domain treatment for aligned concepts (the right/role Reasoning paragraph).** Where a concept has an aligned meaning in *both* the social/epistemological domain and the technical domain, Part 1 now makes the alignment explicit rather than implicit: a *right* and a *role* are first stated as the layer-independent social distinction (standing vs delegated authority), then Drystone's **Group Role** is named as that social *role* made technical *with fidelity*, passing the same removal-leaves-standing test. The mechanism is trustworthy exactly because the wire-level right/Group-Role distinction reproduces the social standing/authority distinction rather than inventing a different one. This "social reality and its faithful technical mirror" framing is the organizing move of Part 1 and should be applied wherever a term spans both planes.

- **Ostrom grounding: lowercase group preserved, capital-G Group linked as realization.** Ostrom's principles are about human communities (the social plane); capitalizing "group" there would overwrite her semantics and break the homage. The fix links the two planes instead: her value stated in the lowercase social sense, with the capital-G Group named as the *realization* of the value (the §7.6 hard-stop surfacing to the affected group, adjudicated within the Group that manifests them; the unforbiddable fork). General rule: when referencing an external source that grounds a *value*, keep the source's own (social) semantics and link the technical realization rather than recasting the source in system terms.

- **"The Group holds keys" avoided in favor of "the members hold the keys."** Because a capital-G Group can itself be a principal, any sentence like "the Group holds keys" is ambiguous between the Group-as-principal holding a credential and the member clients (leaf nodes) holding the group key. Where the intended referent is the latter (e.g. material reversibility: no helper holds the data hostage because the members hold the decryption keys), name the actual key-holders. **Standing vigilance note:** the Group-as-possible-principal is a recurring source of referent-ambiguity; wherever the Group could be read as the acting principal, differentiate explicitly (member clients vs Group-principal).

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

### B.6 Tag ladder unified to the canonical set (the p10 pass)

Part 2's parallel status vocabulary was unified to the canonical ladder (A.9). Mechanical mappings applied across Part 2, with counts from this pass: `green-real` to `Verified` (34), `green-model` to `Modeled` (3), the lowercase inline `design` tag to `Design` (17), `ENABLING` to `[gates-release]` (17), and `[confirm before publish]` to `[confirm]` (14, several carrying an inline note, for example `[confirm, the DS/AS trust model against RFC 9420/9750]`). Part 1's single `ENABLING` was mapped the same way. Prose uses of the word "design" (83 occurrences in Part 2) and the emphasis forms `*designed*` and `*design philosophy*` were left untouched, since only the tag forms were in scope. The status-flag legend at the head of Part 2 now points here as the single source.

### B.7 Governance-ordering reconciliations (decided in the p10 pass; applied in the §7.3 fold)

Two loose spots in Part 2 §7.3.1 were reconciled against this session's matured governance work, and both are now applied in the §7.3.1 order (governance fold, sub-pass 2a-i). The decisions, as applied:

- **C1: authorization is a gate, not a ranking (the "issuer authority rank" sort key is absorbed, not retained).** §7.3.1's first sort key was phrased as "issuer authority rank at the causal frontier," which read as though a single author's standing orders governance outcomes. That verbiage was loose. The matured model separates an **asserted fact** (authored by one persona, ordered among conflicting facts), a **k-of-n quorum decision** (the assembly that *is* the governance act), and **the fold's handling** of the authorized decisions. The resolution is that authority is established **structurally, as a precondition**: the §7.5.2 forward pass decides who may act at each causal position, a slot moves only on an authorized k-of-n quorum, and a sub-threshold or unauthorized fact never enters the order at all. Authority is therefore **not** a comparator key, and issuer rank does **not** survive as a sort key, because ranking conflicting decisions by their author's standing would let authority tip an outcome, a center living in the comparator, which the peer-symmetry premise and the configurable-tiebreak principle both forbid. Among authorized decisions the order is operation-type precedence, then causal precedence, then the concurrent tiebreak. Applied in §7.3.1, sub-pass 2a-i.

- **C2: causal-length and causal-precedence are one relation (unify the statement).** §7.3.1 step 3 ("causal length, more-informed wins") and the working fold-semantics rule ("causal precedence, a causally-later fact supersedes") describe the **same** causal-DAG relation from two ends. The merged text states it once, precisely, as causal-DAG precedence, with "more-informed / longer referenced history" and "causally-later" as two descriptions of that single relation rather than two separate keys. This session's rule set (fold-semantics R1 through R4) is the mature statement; §7.3.1 step 3 was a conceptual stub. Applied in §7.3.1, sub-pass 2a-i (causal precedence stated once, as key 2 of the order).

### B.8 The human-adjudication vocabulary codified as A.11 (the adjudication-language pass)

A drift audit across the spec set and the experiment corpus found the settled
human-adjudication vocabulary (the seam, the recognizer, the reconcile
hard-stop, the escalation shapes, the algedonic channel, the legible picture,
the local authority, planes of authority) used consistently in Part 1 §2–§3
and Part 2 §7.6 but re-derived, misspelled ("hard stop"), or contradicted
(iroh's last-writer-wins-by-timestamp language) elsewhere. **A.11** promotes
that vocabulary to a convention with ten checkable description rules (DR-1
through DR-10), so experiment docs, plans, and test suites inherit it by
reference rather than each restating (and drifting from) the mechanism. Two
terms are coined here rather than merely anchored: **the local authority**
(a name for escalation's receiving side that is not a node role) and **planes
of authority** (the authority-level statement of the operator /
local-authority separation A.4/A.5/A.7 already establish for permissions,
revocation, and standing). Applied across Part 1 §2.5/§3, Part 2 §7.6, the
conformance suite, the iroh corpus, and doc-writing-method §12.

---

## Part C. How to use this document in a later pass

Read the docs *against* Part A. For each load-bearing word, the test is stated; a usage that fails the test is a slip to fix. The specific failure modes to hunt: a term that has slipped its bounded context (a scope-level fact attached to a Group, or vice versa); the same referent defined twice under two names; a coinage used before it is introduced; and a superseded definition left co-existing with its replacement. Part B is the record of what was already decided, so a later pass does not re-lit­igate a settled call or accidentally reverse a supersession. This is the same relationship `doc-writing-method.md` has to the design docs: a stated practice you can check for.
