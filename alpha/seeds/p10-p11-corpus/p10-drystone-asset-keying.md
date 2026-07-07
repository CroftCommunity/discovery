# Drystone: Data-Plane Asset Keying and the Governance/Key Seam

`Status: draft. Model shape settled; wire encodings and two external dependencies open (see Open items).`

`Scope: how a Group's data-plane content assets are keyed, read-scoped, and reconciled, and how that keying couples to the live message layer through the governance fold while staying decoupled from it. Sits beneath Part 2 §5 (governance) and §7 (history), and proposes a candidate resolution to the read-scoped half of the Part 2 §5.10 seam (communal-namespace key construction under membership change).`

`Companion to: p10-drystone-history-durability.md, which uses the per-scope asset keys defined here to seal the payloads its durability tier stores and reconciles, and which shares this document's single-primitive (MLS over iroh), single-threat-boundary posture. Also companion to p10-drystone-social-mapping.md, whose public-interop, group-layer, and private-overlay records are sealed and classified by the keys and store flavors defined here. Also companion to p10-drystone-part1-principles.md (the principles this realizes) and p10-drystone-part2-mechanics.md (the mechanics it extends). Vocabulary inherits from the conventions-and-decisions reference.`

This document is the "what" for one layer: the key material that gates read access to durable data-plane assets, and the rule that couples key rotation to governance without collapsing the two. Normative keywords are **MUST / SHOULD / MAY** (BCP 14). Each load-bearing claim carries a status flag: `green-real` (exercised against real crypto/transport), `green-model` (reference model), `Design` / `Synthesis` (the design's own reasoning, to be judged as reasoning), `[confirm]` (load-bearing and rests on an external fact or version not yet independently verified this round).

> **Maturity note.** The layers this document leans on are matured to different degrees in the suite. The three-plane authority model (§5.5), the two convergence modes (§7.7), the monotonic governance fold and its ordering (§7.3), the `(G, D)` cursor (§7.4), and gap-aware history convergence (§6.8.1) are Part 2 material at Part 2's maturity. The constructions this document adds, the plane split, fold-gated key provisioning, the quiescence predicate, the two-anchor content header, and the re-key trigger rule, are `Design`: their reasoning is complete but their wire encodings are unspecified and several external facts remain `[confirm]` (enumerated in the changelog tag posture). Nothing here is `green-real` yet.

> **Center-free framing.** This layer assumes no node holds privileged or canonical authority (Part 2 §3.1). Every "is this current" question therefore resolves to corroboration against held facts, never to a query against an authoritative vantage, and every timing question resolves to a causal or structural predicate, never to a wall-clock (`P-Knowable-Truth`, Part 1 §2.2; Part 2 §2.0.1).

---

## A. Terms

Definitions this document relies on. Inherited terms carry this document's working definition and the source of inheritance, so the usage can be checked against the source rather than assumed.

**persona** (inherited, conventions reference): a human's manifestation in the system, one root key lineage, one or more devices. The unit that holds standing and that a threshold counts.

**Group** (inherited, Part 2 §5.0): the capital-G in-system entitlement-and-governance principal, realized over an MLS group (Part 2 §6, §10.2). Distinct from the lowercase social **group**.

**Group Role** (inherited, Part 2 §5.5): a concrete, revocable, scoped grant of authority inside a Group. Distinct from the social **role**. A Group Role may carry the authority to issue capabilities.

**capability** (inherited, Meadowcap, Part 2 §5.5): an unforgeable read or write data-access grant, issued under a Group Role, living in the data plane one level beneath the peer-equality question.

**meer** (inherited, conventions reference): a store-and-forward node that holds sealed bytes it cannot read, sits in a Group's scope, and is not a member of the Group. A resource, not a grant.

**(G, D) cursor** (inherited, Part 2 §7.4): a node's two-dimensional position, its folded **G**overnance generation and its **D**ataplane/key generation. A returning member reports it; this document has a re-keying member and an authoring message consult it.

**gap-aware history convergence** (inherited, Part 2 §6.8.1): out-of-band reconciliation of the content-addressed history by per-author high-water marks, detecting nameable gaps and filling them from any self-verifying source.

**read-scoped asset**: a durable data-plane asset whose read is confined to the current holders of a named Group Role (the basic case: a moderation log readable by the moderator Role). An asset readable by all members is the degenerate case where the Role is the whole Group.

**asset key**: the symmetric key under which a read-scoped asset's payload is sealed. Wrapped to each current Role-holder's key. The object this document constructs and rotates.

**fold-gated provisioning**: the discipline that asset-key wrapping and rotation are a downstream convergent consequence of the governance fold, delivered out of band, never an inline act at the moment of a grant or revocation. The core mechanism of this document.

**quiescence** (of a membership frontier): the structural predicate that a node holds a causally-complete, freshness-corroborated view of that frontier, so the folded member set is final for it. Not a duration. Defined in §D.

**governance generation counter**: a monotonic integer advanced with each governance fact on the governance chain, stamped onto data-plane entries as a currency locator. Defined in §F.

**store, and convergence model**: a Group holds a family of independently-kept, independently-corroborated stores, not one. A store's convergence model is how its contents resolve once reconciled, fixed by three questions (mutable, order-load-bearing, multi-writer per unit) and landing on one of the recognizable flavors. Anchored in §E; the flavors are defined there rather than repeated here.

---

## B. Cast

The suite's running cast, specialized to this layer. Each role carries a fixed trust meaning.

**Della** and **Emil**: two personae holding an admin Group Role in the same Group, equal rank, under a k-of-n-by-persona threshold (Part 2 §5.7). They originate membership changes.

**Mara, Nils, Ola, Quinn**: four personae holding a moderator Group Role, which confers read on the moderation-log asset **M**.

**Pia**: a persona who is a candidate for the moderator Role, not yet a holder.

**M**: the moderation-log asset, read-scoped to the moderator Role, sealed under asset key K. The concrete read-scoped asset the cast acts on.

Throughout, "Ola cannot read post-removal M" means the same specific thing: Ola holds no key that decrypts M content authored causally after Ola's removal folds in.

---

## C. The layer split: three planes, one identity

`Realizes: P-Local-Truth, P-Knowable-Truth`

A Group's state moves on three planes with different requirements, and the failure this split prevents is treating them as one problem with one key system.

- **Live latest delivery.** A new message reaches members, the epoch advances, each member holds the current head with integrity. This plane is linear, forward-moving, and forward-secret. Its requirement is a single agreed current epoch, which MLS's ratchet provides. The linearity is a feature here.

- **History convergence.** The durable record is reconciled out of band over the content-addressed hash tree by gap-aware convergence (§6.8.1). This plane does no key agreement; it reconciles already-sealed bytes. Its integrity anchor is the live plane's latest head: a node knows what "now" is, so it can fill backward with confidence.

- **Read-scoped decryption.** A persona granted a read Group Role after content was authored must be able to read that content; a persona removed from the Role must be excluded from content authored after the removal folds. This is the plane the asset key serves, and the only plane where the choice of key system bites.

**Why the split, not one key system.** The live plane wants forward secrecy and a linear head, so a member who joins later must *not* silently gain old keys. The read-scoped plane wants the opposite for the assets it governs: a later-granted Role-holder must read history, because the assets are durable and often un-renderable without their causal predecessors. A single key system cannot deliver forward-secret-linear and readable-by-future-holders at once, so the planes are keyed separately. `Synthesis.`

**One identity across all three.** The split is not two rosters. Every plane derives membership from one truth: the personae's root key lineages and the governance chain. The asset key is wrapped to the Role-holders the governance fold names, the same fold that issues capabilities and evaluates thresholds (Part 2 §5.5, §5.7). Key derivation differs per plane; the membership it derives from is single. `Synthesis.`

**The security meaning of durability.** The read-scoped plane is deliberately not forward-secret for the assets it holds: a granted Role-holder holds the asset key and can read all content under it, and a later-compromised key exposes that history. This is intrinsic to entitled-to-history and is confirmed by independent local-first access-control work, where causal-history access is accepted as necessary to render a view and forward secrecy is correspondingly out of scope for it. `[confirm: Ink & Switch Keyhive causal-key model; p2panda access-control model.]` The consequence to state at the point a reader would assume otherwise: forward secrecy is meaningful only for content that stays on the live plane, so a Group's choice of what is durable-and-read-scoped versus ephemeral is a security decision, not only a storage one. It maps onto the Part 2 §7.7 mode axis and the read-scope axis, which are independent (§E). `Synthesis.`

---

## D. Fold-gated key provisioning

`Realizes: P-Knowable-Truth`

**Requirement.** A read-scoped asset MUST be readable by exactly the current holders of its read-granting Group Role, including personae granted the Role after the content was authored (Part 2 §7.2 R1, R2), and MUST exclude personae after their removal from that Role folds in (Part 2 §7.2 R5). The construction MUST produce one converged asset key per stable membership frontier with no coordinator, and MUST NOT rest on a wall-clock (Part 2 §7.3.1, §7.4 R4).

**Construction.** The asset lives in the Group's communal namespace (Part 2 §5.10, primary stance: the Group-principal is a communal namespace at all times). Its payload is sealed under an asset key wrapped to each current Role-holder's key. The two membership operations are asymmetric, and that asymmetry is the whole mechanism.

- **Add** to the read Role is additive. The current asset key is wrapped to the new holder's key. No rotation. Commutative with concurrent operations.

- **Remove** from the read Role requires a fresh asset key carrying new secret entropy the removed party does not hold, wrapped only to the folded remaining set. Deriving the new key from the old is forbidden, because the removed party held the old key; a derivation of the form `K' = KDF(K_old, frontier)` is computable by the removed party and breaks R5. This mirrors why continuous group key agreement injects fresh path secrets on removal rather than deriving forward.

- **Provisioning is fold-gated.** Key wrapping and rotation are a downstream convergent consequence of the governance fold (Part 2 §7.3), delivered out of band through the same channel as history convergence, never an inline act at the moment of a grant or revocation. The fold decides the current Role-holder set first; the current asset key is then provisioned to exactly that set.

**The asset key gates read, not write; write stays per-author.** The asset key seals payload, so it governs who may decrypt and nothing more. Write authority is separate: it is a capability issued under the read-granting Group Role (Part 2 §5.5), held per persona, and each contributor authors into its own subspace signed with its own key. A read-scoped asset is therefore the union of per-author, individually-attributable entries sealed under one shared read key, never a shared authoring secret. The distinction is load-bearing. A shared key used to author would collapse attribution, since every entry would read as "some Role-holder" and defeat Part 2 §7.2 R6; it could not be revoked from one holder without re-keying all of them; and it would let a removed holder who retained it forge entries indistinguishable from a current holder's. Sealing the union under a shared read key while keeping authoring per-author preserves attribution and per-persona revocation and still delivers the limited shared visibility the scope names. `Synthesis.`

**Per-scope keys are siblings, not a hierarchy.** Each read-scope has its own asset key, independently wrapped to that scope's folded set; no scope's key is derived from another's. A Group principal's all-members communal key and an admin Role's log key coexist as siblings, each wrapped to its own set, so a Group can hold a communal key for all-members assets and a narrower shared key for an admin or moderation log at the same time. Deriving the narrower key from the wider one would make the limitation fiction, because every member holds the communal key and could then derive the admin key. A narrower scope is enforced only by wrapping to fewer holders, never by a derivation any wider-scope holder could reproduce. `Synthesis.`

**Why fold-gated, not eager.** An eager scheme wraps or rotates the key at the moment of the governance act, racing the fold. Under concurrency this fails in two ways, both traced in §G: a member added on a branch that later loses a same-subject conflict still holds a key it should never have had, and two concurrent removals resolved by naively picking one winner silently drop the losing rotation, leaving a just-removed member able to derive the current key. Deferring provisioning to a converged fold removes both, because at fold time the member set is deterministic and every key operation is computed against it, not against a partial view. `Synthesis.`

**Concurrent re-key at quiescence is benign.** Two remaining members may both mint a fresh asset key at quiescence. Because provisioning follows a converged fold, every candidate key is wrapped to the same deterministic folded set, so every candidate already excludes the correct members and is R5-safe. The Group converges on one candidate by the Part 2 §7.3.1 content-address tiebreak. Content sealed under a superseded candidate remains readable by the folded set, which holds it, and is never re-sealed (Part 2 §6.5.3). Concurrency among candidates is thus a convergence question resolved by machinery the suite already has, not a dropped-rotation hazard. `Synthesis.`

**Quiescence is a predicate, not a timer.** A membership frontier is quiesced for a node when both hold:

- **Completeness.** For the batch of membership operations being resolved, the node holds every operation causally concurrent with them, with no nameable gap, checked via gap-aware convergence (§6.8.1).

- **Freshness.** The node has heard the same frontier head corroborated by at least the threshold of distinct persona lineages (Part 2 §7.4, §5.7).

Neither term is a duration. A node evaluates the predicate against the fact graph it holds and the corroboration signals it has received; local elapsed time enters only as a private input to "have I heard enough corroboration," never as a shared clock and never to order anything. This is the freshness-plus-completeness bar the suite already requires to *originate* a membership act, applied to the re-key that follows it. `Synthesis.`

**How it satisfies Part 2 §7.2.**

- **R3 (convergent revocation):** provisioning follows the convergent fold, so honest peers converge on the same current asset key for the same frontier.

- **R4 (bounded stale exposure):** the bound is "until the frontier quiesces and the fresh key converges," expressed in fold generation, not wall-clock.

- **R5 (forward read exclusion):** post-quiescence content is sealed under a key carrying entropy the removed party lacks. Content causally concurrent with a removal remains readable by the then-current set, which is correct, not a breach (§G).

- **R6 (attributable acceptance):** each grant, revocation, and provisioning act is a signed fact carrying the causal frontier it acted on, so a stale provisioning is detectable and attributable.

---

## E. One substrate, many convergence models

`Realizes: P-Local-Truth, P-Durable-Enablement`

A Group does not hold one store. It holds a family of independently-kept, independently-corroborated logs and record stores, each chosen for its use. Two properties vary across them and are orthogonal, so a store is a (convergence model, read-scope) pair rather than a point on one list. Stating the axes as independent prevents forcing one shape onto uses that need different ones.

**What is shared across every store, and what is per-store.** Every store sits on one substrate: per-author-attributable entries, each authored into the author's own subspace under a capability (§D), sealed under a per-scope asset key (§D), converged by gap-aware history convergence (§6.8.1) anchored on the latest delivered head (§H). What differs per store is only how its contents resolve once reconciled, whether prune is permitted and at what granularity, and whether it can carry a CRDT payload. So there is one identity model, one write model, one key model, and one convergence transport, with many content-resolution engines above them. This is the one-mechanism-named-once discipline (doc-method §7): the substrate is named once, the resolution is per-flavor. `Synthesis.`

**The convergence model is decided by three questions, not chosen from a fixed list.** A store's model is fixed by answering, in order:

- **Mutable?** Are entries immutable once written (append-only), or may a later write overwrite or delete an earlier one? This maps to the Part 2 §7.7 mode: append-only is forward-only mode, overwrite-capable is Willow-mutable mode, and the mode is a per-Group choice at creation.

- **Order-load-bearing?** If append-only, does resolution depend on ordering the writes (a fold whose output changes with order), or only on their presence (a union that does not)? Governance authority depends on order; a message stream does not.

- **Multi-writer per unit?** Is any single addressable unit written by more than one persona concurrently (needs a merge), or only ever by one (last-writer suffices)? Per-author subspaces make most units single-writer by construction; only a genuinely shared object is multi-writer.

An implementer classifies a new store by these three questions. The named flavors below are the common answer-combinations, not an exhaustive set. `Synthesis.`

**The recognizable flavors.** Each is stated with the use that forces it, its resolution, its prune posture, and the need it does not have, so a naive design does not over-build:

- **Ordered append-only log** (append-only, order-load-bearing). The governance chain, an audit trail, a moderation-action log, a ban list. Resolution is the monotonic fold by Part 2 §7.3.1; nothing is overwritten; prune is none, a coordinated roll-up (Part 2 §7.7.2) being the only reduction. The property that matters is that the fold is deterministic and cannot be reset. `Synthesis.`

- **Grow-set** (append-only, order-not-load-bearing, single-author entries). A message stream, a comment collection, an event feed. Resolution is set union, with causal order used only for display; entries do not conflict, they coexist. It does not need the log's conflict-ordering machinery, which is the over-build to avoid here. Prune is none, roll-up only. `Synthesis.`

- **Single-writer mutable record** (mutable, single-writer). A persona's profile or status, a message its author may edit. The only concurrency is intra-lineage across the author's devices, settled by last-writer-by-logical-clock. Resolution is per-path last-writer; prune is per-path overwrite or tombstone. It needs no merge CRDT, which is the over-build to avoid here. `Synthesis.`

- **Multi-writer mutable record** (mutable, multi-writer per unit). A collaborative document, a jointly-edited structured object. This is the one flavor with genuine multi-persona concurrent mutation of a single unit, and the only one that needs a merge CRDT: the asset payload is a convergent monotonic change structure (an operation-based CRDT) that merges inside the payload while the asset key and namespace operate at entry granularity. Because a CRDT assumes its causal history only accretes, prune MUST operate at asset granularity (drop the whole asset), never at individual-change granularity, so a tombstone cannot remove a change a later merge needs. `Synthesis.` `[confirm: efficiency of storing CRDT change payloads as Willow entries; independent local-first reports treat raw CRDT-on-Willow as workable but not efficient, and favor chunked compress-then-encrypt of change ranges.]`

A path-addressed key-value or structured-state store (settings, tag indices, a configuration map) is not a separate flavor. It is the mutable case resolved per key, each key being single-writer (last-writer) or multi-writer (CRDT) by the third question. `Synthesis.`

**Read-scope composes orthogonally (§D).** Any flavor sits at any read-scope, all-members, a Group Role, or Group-internal, because scope is enforced by which folded set the asset key is wrapped to (§D) and is independent of how the store's contents resolve. A ban list can be admin-scoped or all-members; a collaborative document can be Role-scoped or communal. The asset key (scope) and the convergence model (flavor) are chosen separately.

**Fork-carriage is a third axis, independent of both.** Whether a store travels to both descendants when a Group forks (Part 2 §7.6). A communal content store carries to both. A Group-internal meta store (for example, expulsion rationale readable only by an admin Role) may or may not be re-adopted by a descendant, and non-adoption is not erasure: the parent chain retains it, auditable. Fork-carriage of sensitive meta is a §7.6 utility judgment, not a mechanical default, because the meta may be exactly what a schism contests. `Synthesis.`

**Flavor table.** Read-scope and fork-carriage apply to every row, orthogonally.

| Flavor (§E) | Mutable | Order-load-bearing | Multi-writer per unit | Resolution | Prune | CRDT payload |
|---|---|---|---|---|---|---|
| Ordered append-only log | no | yes | no | monotonic fold (§7.3.1) | none (roll-up only) | no |
| Grow-set | no | no | no | set union, causal-for-display | none (roll-up only) | no |
| Single-writer mutable record | yes | not applicable | no | per-path last-writer (logical clock) | per-path overwrite or tombstone | no |
| Multi-writer mutable record | yes | not applicable | yes | merge CRDT in payload | asset granularity only | yes |

---

## F. The two-anchor content header

`Realizes: P-Knowable-Truth`

**Requirement.** A verifier MUST be able to determine, for an incoming data-plane entry, whether it holds the governance state the entry was authored against, so it can detect a nameable governance gap and locate the boundary at which a read-scoped asset key rotated. This MUST NOT rest on a wall-clock, and the signal MUST NOT be an authorization input on its own.

**Construction.** The MLS epoch and the governance chain advance on different events and are formally decoupled (§H). A node therefore holds two anchors, one per chain, and both are consulted:

- The **MLS epoch** `E`, the key-currency anchor, carried by delivery and grounded by the latest delivered head (§H).

- The **governance generation counter** `n`, the authority-currency anchor, stamped onto data-plane entries.

**Why a counter, not a clock and not a Lamport timestamp.** The governance chain is a Merkle chain of monotonic decisions that build on each other, so it is self-ordered by its own hash-linking and needs no Lamport clock to order concurrent authors; a Lamport clock earns its place only where concurrent authors share no sequencer, which the linear governance chain is not. A monotonic generation integer advanced with the chain is sufficient and cheaper, and it maps onto the MLS epoch integer already present. `Synthesis.`

**Currency check.** A verifier at folded governance generation `n_local` reading an entry stamped `n_msg` compares two integers. If `n_msg` is at or below `n_local`, the verifier holds at least the author's governance state and can interpret the entry with no gap. If `n_msg` is above `n_local`, the verifier has a nameable gap and knows its size, and MUST converge the missing governance before treating the entry as authoritative or re-keying against it. The entry thereby self-announces the governance facts a reader is missing, which closes part of the gap-detectability problem: a governance fact that no dataplane entry has yet depended on may be silently absent, but one that a dataplane entry did depend on is announced by that entry. `Synthesis.`

**Encoding, requirement and non-requirement.** The header MUST carry the governance generation as a currency locator. It SHOULD be delta-encoded: an entry carries the counter explicitly only when it differs from the author's previous stamped value, and inherits otherwise, because governance frontiers are stable across long runs of content, so the explicit stamp costs bytes only at the transitions where the re-key boundary and the R5 concurrency case actually consult it. Whether the counter or the full governance head hash is the wire value is an encoding choice deferred to Appendix B; the counter suffices for the currency check, the hash would additionally let a reader verify governance content from the entry alone. `Design.`

**Verified locator, never an authorization input.** A stamped `n` is the author's assertion about their governance frontier and is forgeable, so it MUST be used only to locate, to detect gaps, and to read the re-key boundary, each then confirmed against the governance chain the verifier independently folds. It MUST NOT authorize on its own; a stamped counter that authorized would be a forgeable fast path, which the suite's razor rejects (Part 2 §2.0.1). `Synthesis.`

---

## G. The concurrency trace

`Realizes: P-Knowable-Truth`

The cast (§B) authored into M under concurrent membership change, showing where eager keying fails and fold-gating (§D) holds. All cases assume the admin threshold is met, so each membership act is authorized; the question is only the key layer.

**Add plus remove, different subjects (Della adds Pia, Emil removes Ola, concurrent).** Governance folds cleanly; the operations are different subjects and do not conflict (Part 2 §7.3.2). Under fold-gated provisioning the folded set is {Mara, Nils, Pia}, and the current asset key (post-Ola-removal) is provisioned to exactly that set. Pia is provisioned the current key, not a stale one; Ola is not provisioned. Outcome: clean, no escalation. Under eager keying, Della would wrap the pre-removal key to Pia before folding Emil's removal, leaving Pia holding a stale key and momentarily unable to read post-removal content, an under-provisioned, self-repairing state, but avoidable.

**Add plus remove, same subject (Della adds Pia, Emil removes Pia, concurrent).** Governance conflicts (same subject, order-dependent) and resolves determinately by Part 2 §7.3.1: equal issuer rank, so authority-reducing outranks authority-expanding, so the removal wins and Pia is not a moderator. Under fold-gated provisioning Pia's key is never provisioned, because provisioning is downstream of the resolution that excluded her. Under eager keying Della would already have wrapped the key to Pia, leaking past M content to a persona whose grant lost the conflict. Fold-gating removes the leak.

**Two removals, different subjects (Della removes Quinn, Emil removes Ola, concurrent).** Governance folds cleanly to {Mara, Nils}. This is the case eager keying fails hardest: two concurrent rotations, and naively picking one winner drops the loser's rotation, so the winning key predates and ignores one removal and remains wrapped to a just-removed member, a true R5 breach. Under fold-gated provisioning a single fresh key is provisioned to exactly {Mara, Nils} after the fold, superseding both concurrent candidates; no rotation is dropped, and if two remaining members mint concurrently the candidates are all R5-safe and resolve by the content-address tiebreak. Outcome: breach removed.

**The window (Mara authors into M during the above).** The window is per-node, defined by what the author has folded. If Mara has folded neither removal, she holds and seals under the pre-rotation key, readable by the then-current set including the about-to-be-removed. This is correct, not a leak: the content is causally concurrent with the removals, not causally after them, and R5 excludes only post-removal content. If Mara has folded both and the frontier has quiesced, she seals under the fresh key and the removed cannot read it. The window content is never re-sealed (Part 2 §6.5.3), and the removed already hold the pre-rotation key, so re-sealing would be both forbidden and pointless. `Synthesis.`

**Under partition.** The quiescence predicate (§D) does not evaluate true, because the node cannot assemble a corroborated complete frontier, so no re-key occurs and authors continue under the last stable key. Safety holds under arbitrary partition, because R5 is causal, not temporal: however long the partition lasts, the only content a removed member can read is content causally concurrent with their own removal, which they could already read. The cost is liveness, not safety: a removal's forward-exclusion is eventually-consistent, gated on the same freshness as any membership act, and a partition delays the exclusion taking effect without ever letting post-removal content leak. `Synthesis.`

---

## H. The governance/epoch seam and the re-key trigger

`Realizes: P-Knowable-Truth`

**Epoch advance is triggered by key-change need, not by governance category.** The MLS epoch advances only when a Commit is applied, and a Commit is sent only when a key change is needed: a member added, a member removed, or a key rotated for post-compromise security. The epoch therefore counts key rotations. Its trigger is the key-change need, which is correlated with some governance events and not others: a membership change needs a key change, so it advances the epoch; an authority-only change (grant a Group Role, change a threshold, amend policy) needs no key change, so it does not. Authority-only changes are governance facts that fold on the governance chain (Part 2 §7.3) with no Commit and no epoch change. The two clocks are decoupled: the epoch tracks key-change need, the governance counter (§F) tracks authority, and their triggers coincide only at a membership change. This decoupling is the design's own, and it is the reading the Part 2 §7.3 fold already implies: were authority carried by the linear Commit sequence, that sequence would be the order and no separate governance fold or §7.3.1 conflict resolution would be needed, so the existence of the fold is itself evidence that governance does not ride Commits. `Design.`

**The epoch number is not an authority signal.** A node MUST NOT infer authority state from the epoch. The epoch advances for key hygiene (a post-compromise rotation) as well as for membership, so epoch equality does not imply authority equality, and an epoch advance does not imply a governance change. Authority currency is read only from the governance counter (§F), never from the epoch. This is the mirror of the trigger rule below: governance is not inferred from the epoch, and the epoch is not driven by governance category, so the two clocks stay separate in both directions and the counter is never overloaded to mean two things at once. `Synthesis.`

**Membership folds as a governance fact first, enforced by a subsequent Commit.** A membership change is both an authority change and a key change, so it produces both a governance fact and a Commit. These MUST occur as a two-phase sequence: the governance fact folds first (the governance counter advances, honest nodes know the member is out and reject their future authority), then the enforcing Commit rotates keys against the already-resolved folded set (the asset key rotates to exclude the removed member from future content, §D). They MUST NOT be bound into a single operation in which the Commit is the governance fact. Binding them would route concurrent membership through MLS's commit-collision path, two Commits against one epoch with one rejected and retried, whereas folding first lets concurrent membership resolve by the §7.3.1 order (§G), with the Commit then rotating keys against a set the fold has already settled. The interval between the two phases is a pending state, safe because the phases fail safe independently: during pending the removed member is known-out by authority so nothing they author is accepted as authoritative, and the only content they can still read is content causally concurrent with their removal, which R5 never excluded. Authority-knowledge running ahead of key-enforcement is the safe direction; keys rotating ahead of a folded authority to rotate them would be the unsafe direction, and fold-gating forbids it. `Design.`

**The trigger rule (stated once, referenced by §D and §F).** Read-scoped asset re-key MUST be initiated by a governance-fold membership change, never by MLS epoch advance. A routine epoch tick (a post-compromise rotation, a key refresh) MUST NOT initiate a read-scoped-asset re-key, and a governance removal MUST initiate one even if no independent epoch tick is due. This is the one rule that, reversed, reintroduces the unsafe direction: triggering on epoch advance would re-key on routine rotations wastefully and, worse, would fail to re-key on a governance removal not yet coincident with an epoch tick, so phase two would never fire. The two clocks rendezvous only at the membership change, where the governance fold causes the epoch-affecting re-key rather than the epoch happening to carry it. `Synthesis.`

**The anchor's durability.** The latest delivered head is the completeness witness for everything causally behind it: a delivered message is positioned in the chain and self-verifying, so it proves the node is complete up to that head without a poll. It cannot prove completeness ahead of the head; that is corroborated by beacon agreement across the threshold of lineages (§D freshness), never proven, and a genuinely unseen concurrent fact degrades to the benign candidate-key reconciliation (§D, §G), not a breach. Because the head grounds both the completeness check and the re-key boundary, its durability is the highest-value durability target on the data plane, which is a design pressure toward the meer, the D-peer, and the device-lineage replay buffer holding the head first (Part 2 §6.6). `Synthesis.`

---

## I. Why no continuous group key agreement is required here

`Realizes: P-Knowable-Truth, P-Peer-Equality`

A decentralized continuous group key agreement (a BeeKEM-shaped protocol, the key layer of Ink & Switch's Keyhive) exists to make concurrent re-key safe with no coordinating structure, by merging concurrent key operations through tree-healing rather than picking a winner. `[confirm: BeeKEM concurrency model and its rationale that TreeKEM requires a central total order.]` Two of its properties are worth separating for Drystone.

- **Causal-not-total ordering** is redundant here. Drystone already resolves membership deterministically by a monotonic governance fold (Part 2 §7.3.1), which is the coordinating structure a decentralized CGKA assumes it lacks. Gating key provisioning on that fold (§D) moves all re-key concurrency to after a converged membership decision, at which point every candidate key already excludes the correct members, so concurrency resolves by the content-address tiebreak the suite already has. `Synthesis.`

- **In-place decryption of held-but-locked ciphertext** (a Cryptree, where holding one chunk's key derives its causal predecessors') is redundant in this topology. It earns its cost when untrusted nodes hold read-scoped ciphertext that entitlement must unlock in place, which is the server-holds-encrypted-blob model. Drystone's durability nodes are meers that re-converge entitled history out of band to a member who holds the current key, so a member is never handed ciphertext they cannot read and there is nothing to walk. The forward-inclusion a Cryptree provides is delivered instead by convergence gated on the capability the member now holds. `Synthesis.`

**Residual value, and when to reach for it.** A CGKA's un-redundant value is cost: it makes re-key incremental (logarithmic, not one wrapping per remaining member) and merges concurrent re-keys by healing rather than by re-wrapping the whole set. Re-key cost under the §D construction is one wrapping per remaining Role-holder, negligible until a read-scoped asset is both large in its Role membership and high in concurrent churn. Neither the large-but-stable asset (rare re-keys) nor the small-but-churny asset (cheap re-keys) reaches that wall; only large-and-high-churn does, which is uncommon for a read-scoped asset such as a moderation log. A CGKA is therefore held in reserve as a scoped cost optimization for that case, adopted behind the same interface the §D construction presents (fresh entropy on removal, wrapped to the folded set, provisioned out of band downstream of the fold), never as the mechanism that makes concurrency safe, because fold-gating already makes it safe. `Synthesis.`

---

## J. Posture summary

Each row states a decided-and-bounded posture. Genuinely undecided threads are in Open items, not here.

| Case (§) | What the key layer would assume, naively | Drystone posture (outcome) | Forcing principle |
|---|---|---|---|
| Read-scoped asset, later-granted holder (§D) | Later holder cannot read prior content | Asset key wrapped to current holders; later holder reads history; not forward-secret by construction | `P-Durable-Enablement`; CRDT/history needs causal predecessors |
| Store with a use-specific convergence need (§E) | One convergence model forced on all stores | Per-store flavor by the three-question classification; substrate, write model, key model, and transport shared | `P-Durable-Enablement`; fit-to-use, no over-build |
| Add plus remove, different subject (§G) | Eager wrap leaves later-added holder on stale key | Fold-gated: current key provisioned to folded set; under-provision self-repairs | `P-Knowable-Truth` (under-authorize, never mis-authorize) |
| Add plus remove, same subject (§G) | Eager wrap leaks to a lost-conflict grantee | Fold-gated: key never provisioned to the excluded persona | Part 2 §7.3.1 (authority-reducing outranks expanding) |
| Two concurrent removals (§G) | Pick-a-winner drops a rotation, R5 breach | Fold-gated: single fresh key to folded set after the fold; candidates all R5-safe | Part 2 §7.2 R5 |
| Content authored in the window (§G) | Ambiguous whether removed may read it | Sealed under key at author's folded frontier; concurrent content readable by then-current set, correct | R5 is causal, not temporal |
| Partition (§G, §H) | Removal must bite immediately | Forward-exclusion is eventually-consistent; window lengthens, never leaks post-removal content | `P-Knowable-Truth`; no privileged vantage |
| Concurrent re-key at quiescence (§D) | Concurrent rotations conflict dangerously | All candidates wrapped to same folded set; resolved by content-address tiebreak | Part 2 §7.3.1 |
| Epoch advance vs governance change (§H) | Epoch equality implies full agreement | Two anchors; re-key triggered by governance fold, never by epoch advance | Governance/epoch decoupling (this doc) |
| Large, high-churn read-scoped asset (§I) | Per-member re-wrap is prohibitive | CGKA held in reserve as scoped cost optimization behind the §D interface | Cost only; correctness already held by fold-gating |

---

## K. Residuals owned

- **The retained-copy floor is irreducible.** A once-valid Role-holder keeps whatever it decrypted while valid, including window content. No key scheme, a CGKA included, changes this. Revocation protects the future, not the past (Part 2 §5.8).

- **The read-scoped plane is not forward-secret for its assets, by construction.** Stated in §C; repeated here because it is the residual a reader is most likely to over-trust away. Forward secrecy is meaningful only for content that stays on the live plane.

- **Quiescence inherits Part 2 §7.4 freshness and its residual.** Under partition the safe window lengthens unboundedly. The lengthening is safe (only concurrent, never post-fold, content is exposed) but its duration is partition-dependent and cannot be bounded from inside a partitioned segment.

- **Completeness-ahead is corroborated, never proven.** The re-key predicate proves completeness behind the delivered head and corroborates completeness ahead of it. A concurrent fact unseen by every reachable lineage degrades to benign candidate-key reconciliation, not a leak, but "no unseen concurrent fact exists" is not provable in a center-free system.

- **A read-scoped key grant confers capability, not standing.** Granting an asset key to a Role-holder, or to a delegated helper such as a search or index node admitted to the scope, lets that holder read content, and its compromise leaks what it holds (the not-forward-secret residual above). It confers no authority: the holder cannot foreclose on a persona or govern, and can be removed like any member. So the read-scoped plane is the mechanism by which capability is delegated without standing, and admitting a clear-text helper to a scope is a confidentiality-surface choice, never a transfer of authority and never the creation of a center. The principle is stated in p10-drystone-authority-and-complement.md.

---

## Open items

Genuinely undecided threads. Distinct from the decided postures in §J.

- **The all-members communal asset key and the whole-Group primary-namespace key.** This document resolves the read-scoped case of the Part 2 §5.10 seam (the asset key is wrapped to a Role's folded set). The all-members case and the whole-Group primary namespace are not resolved here and may or may not use the same fold-gated provisioning; the whole-Group key is not gated by a *read*-Role fold and likely has a different shape. Carried to Appendix B.

- **Meadowcap composition (the decisive check).** Whether Meadowcap's own communal read-capability issuance composes with fold-gated asset-key wrapping without introducing a second authority path is unverified. This is the narrowed form of the Part 2 §5.10 "dig into Meadowcap and check its alignment with MLS" step, reduced to one checkable question: does capability issuance sit cleanly beneath key wrapping, or does it duplicate the authority the governance fold already holds. `[confirm against willowprotocol.org/specs/meadowcap.]`

- **The gap-detectability completeness half.** The re-key predicate's completeness term (§D) rests on a governance-region gap being *nameable*, so a node can distinguish "missing a concurrent membership fact" from "no such fact exists." The header (§F) makes a gap nameable once a dataplane entry depends on the missing fact; the case of a governance fact no dataplane entry has yet referenced is not yet closed. `Synthesis; open.`

- **Header wire encoding.** Counter versus full governance head hash on the wire, and the delta-encoding rules, are deferred to Appendix B. `ENABLING.`

- **Hash-function reconciliation.** This document does not pin a hash function; it inherits the Part 2 §4-versus-§7 split (SHA-256 proven on the message layer, BLAKE3 designed on the governance layer) and the asset-key layer must land consistently with whichever the read-scoped assets are stored under. `[confirm.]`

---

## Changelog

`This document is a working design doc; per the suite method (doc-method §1), transitions are recorded here so the body stays an end state.`

- **Draft, first consolidation.** Establishes the three-plane split (§C), fold-gated key provisioning as the read-scoped-asset key mechanism (§D), the mode/read-scope/fork-carriage axes and CRDT-as-asset-type with the prune boundary (§E), the two-anchor content header (§F), the concurrency trace (§G), the governance/epoch seam with the re-key trigger rule (§H), and the posture that no CGKA is required for the common case (§I). Proposes §D as a candidate resolution to the read-scoped half of the Part 2 §5.10 seam.

- **Cross-pass obligation.** §D and §F must be checked against a primary Meadowcap read-capability treatment before either hardens (Open items), and the header wire encoding must be reconciled with Appendix B. Repoint or reconcile against Part 2 §5.10 and Appendix B when this material is folded; do not redefine the §5.10 seam, which this document only narrows.

- **Resolved: the epoch/governance coupling.** The prior `[confirm]` in §H (whether governance facts are necessarily epoch-advancing) is decided decoupled. Epoch advance is triggered by key-change need, not governance category; authority-only changes fold as governance facts with no Commit; the epoch number MUST NOT be read as an authority signal; and membership folds as a governance fact first, enforced by a subsequent Commit, never as a single bound commit-is-the-fact operation. This confirms rather than newly chooses the Part 2 §7.3 fold, which would be redundant under coupling. The two-anchor header (§F) and the two-phase removal (§H) are consequences of this decision, not open pending it.

- **Clarified (§D): read key versus authoring key, and sibling keys.** Stated that a read-scoped asset is the union of per-author, individually-attributable writes sealed under one shared read key, never a shared authoring secret, so attribution (R6) and per-persona revocation hold; and that per-scope keys are siblings wrapped independently to their folded sets, not a derivation hierarchy, so a communal all-members key and a narrower admin or moderation key coexist without either deriving the other. This is anti-footgun guidance against building the shared-authoring-secret version; it adds no new mechanism.

- **Expanded (§E): store flavors as a first-class family.** Reframed §E from a short axis list into the single home for the (convergence model, read-scope, fork-carriage) grid. Added the three-question classification (mutable, order-load-bearing, multi-writer per unit), the four recognizable flavors (ordered append-only log, grow-set, single-writer mutable record, multi-writer mutable record), a flavor table, and the statement that all flavors share one substrate, write model, key model, and convergence transport, differing only in content resolution. The prior CRDT-as-asset-type and prune-boundary material folded into the multi-writer flavor. A path-addressed key-value store is stated as the mutable case per key, not a separate flavor. Added a matching term to §A and a posture row to §J. No cross-references repointed, because §E kept its label.

- **Companion doc linked.** p10-drystone-history-durability.md now carries the durability tier, history convergence, and the content-blind history store. Payloads it stores are sealed under the per-scope asset keys defined here; the two docs share the single-primitive, single-boundary posture and should be read together.

- **Tag posture.** The inherited layers (§5.5, §7.2, §7.3, §7.4, §7.7, §6.8.1) carry Part 2's maturity. The constructions added here are `Design` with `[confirm]` on the external dependencies (Meadowcap composition, BeeKEM/Keyhive rationale, Cryptree model, CRDT-on-Willow efficiency). Nothing here is `green-real`.

- **Doc-model normalization.** Normalized epistemic tags to the suite's canonical set for spec-readiness: the compound `design/Synthesis` (and `design/ENABLING`) became `Design`, `Verified, research pass` became `Verified`, and bare lowercase `design` became `Design`. The `Realizes: P-<Principle>` linkage tags are retained as the principle-crosswalk layer, which now resolves against the catalog in p10-drystone-authority-and-complement.md. No substantive claims changed.
