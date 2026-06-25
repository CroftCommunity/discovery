# Drystone Protocol — §2. Peers, Rights, Capabilities, and PeerSets

`Status: DRAFT`

`Realizes: P-Local-Truth, P-Peer-Equality, P-Knowable-Truth, P-Durable-Enablement`

`References: §X (Governance Conflicts); §1 (Design Principles)`

> **Provenance.** Working spec draft, filed 2026-06-25 from the design dialogue
> `../../seeds/transcripts/raw/drystone-peers-rights-governance-matrix-dialogue-2026-06-24.md`.
> DRAFT status. The `P-*` principles it `Realizes` and the §1 they live in are **not yet written**
> (tracked in `ROADMAP_TODO.md` E30); §X is filed alongside as `section-x-governance-conflicts.md`.
> Folding the takeaways into the beta corpus is a separate pass.

---

## 2.0 Purpose

This section defines what a peer is, what every peer is entitled to, what a peer can be granted, and how those three things stay separate. It is the conceptual anchor that §X (governance) and the capability-delegation model both build on. It is deliberately small. The work it does is to fix vocabulary so that later sections cannot accidentally smuggle a rights distinction into what should be a capability distinction.

The governing intuition is a commons-and-croft one. There is shared pasture (scopes, the data a group holds in common) and there is private croft (a peer's own local store). A peer may arrange for help working its croft, by delegating to one of its own devices or to a peer in the shared group, without surrendering the croft and without consolidating everyone onto a single keeper. Flexibility in who helps is the point. A simple, private default is the boundary that keeps the common case safe.

**A note on the term "PeerSet."** This section defines a concept named **PeerSet** (§2.5): a named, pinned bundle of roles and capabilities that a group recognizes and depends on. The compound name keeps it unambiguous against the ordinary word "set" (a set of capabilities, a set of facts, the conflicted set in §X), which keeps its plain meaning throughout. PeerSet always means the defined concept.

---

## 2.1 The only canonical state is local

A device can honestly know exactly one thing: its own local state. Everything else it believes about the world is either a comparison against another peer's asserted state, or an assertion it has accepted. This is not a limitation to be engineered away. It is the ground truth the whole protocol is built on, and naming it prevents a class of design errors.

- A peer's local store is **canonical for that peer**. It is the only state the device can prove rather than infer.

- Any belief about another peer's state is **comparative or asserted**, never canonical. Range reconciliation compares; signed facts assert. Both are honest, and both are distinct from the local canonical truth.

- There is no global canonical state, by design, because no participant in a distributed system can prove it holds complete and current knowledge of all other participants. A claim of global state would be a claim a device cannot honestly make.

`Realizes: P-Local-Truth, P-Knowable-Truth`.

This is the same property §X relies on when it makes authority a fold over an append-only local view: a lagging peer computes a stale-but-honest state, never a false one, because it is only ever reading its own canonical store. The comparative and asserted layers are how peers converge; the local layer is the only thing any of them can stand on.

---

## 2.2 There is one kind of peer

Drystone defines a single kind of participant: a **peer**. There are no peer species, no tiers, no lesser participants. Differences between peers are differences in two separate things, which §2.3 and §2.4 keep apart:

- **what a peer is entitled to** (rights, universal), and

- **what a peer has been granted the means to do** (capabilities, additive and delegated).

A peer is never defined by subtraction. We never say "this peer is a full peer minus a capability." We say "this peer holds the universal floor, plus whatever capabilities have been delegated to it." Every named configuration in this specification, whether an ad hoc role or a pinned PeerSet (§2.5), decomposes into the floor plus an additive capability set. If a proposed configuration cannot be written that way, it is not a role or a PeerSet; it is a smuggled rights distinction, and it is rejected (§2.5).

---

## 2.3 Rights: universal, and never delegated

A right is a claim a peer is entitled to. It is not a power the peer holds and could pass on. Rights cannot be delegated, because delegation operates on capabilities, and a right is not a capability.

The base rights floor, held identically by every peer:

- **Read your own local history.** Every peer may read the plaintext of its own local store. This right is unqualified and identical for all peers.

- **Read the history of a scope you are a member of, for the period of your membership.** A peer's read right over shared history begins when it joins a scope and ends when it leaves. It includes the history the peer was present for. It does not extend to content authored after the peer leaves, and it does not retroactively vanish for content the peer legitimately held while a member (see §2.6 on what leaving does and does not unmake).

- **A scope holds full history for itself.** The group's collective right to its own history is complete and continuous, independent of any individual member's tenure. Members come and go; the scope's history is the scope's.

Two consequences worth stating, because they are where the rights floor is most often misread:

**My history includes the groups I am still in, and ends at leaving.** There is no contradiction between "I always have full rights to my own history" and "my access to a group ends when I leave." My own history is mine permanently. My window into a shared scope is bounded by membership. These are two different histories, and the rights floor treats them as such.

**A peer with no local history of its own still holds the full read-your-own-local-history right.** The right ranges over whatever local history the peer has. A peer playing a pure availability role (§2.4) authored nothing and holds plaintext of nothing, so it satisfies the right vacuously, over an empty set. This is not a withheld or degraded right. It is the same universal right, exercised over nothing, because the peer's role gave it nothing of its own to read. Blindness is the absence of a granted capability (§2.4), not a restriction on a right. The meer PeerSet (§2.5) is exactly this case, named and pinned.

---

## 2.4 Capabilities: additive, delegatable, revocable

A capability is the means to do something. Capabilities sit on top of the rights floor. They are additive (a peer holds the floor plus zero or more capabilities), delegatable (a peer can grant a capability it holds to another peer), and revocable (a grant can be withdrawn, §2.6).

Delegation has two properties that matter:

**It is always additive and bounded.** A peer can delegate a subset of what it holds, never a superset (`Realizes: P-Peer-Equality`). This is the attenuation requirement that §X.5 R2 makes normative. Granting never widens authority.

**It can target either of two groups, using the same primitive.** A peer may delegate a capability to:

- a peer in its **own multi-device group** (one of the peer's other devices), so that trust stays entirely within the peer's personal control and leaks nothing to any third party; or

- a peer in the **shared scope** (a cooperative anchor, or another member's always-on node), extending trust to that peer for the delegated capability only.

The same grant mechanism serves both targets. The difference is the trust boundary the user chooses, not the mechanism. A group may also delegate on behalf of its entirety (the whole scope arranging a shared helper), which is the same primitive applied at scope scope rather than personal scope.

This two-target flexibility is what lets Drystone refuse consolidation. Durability and content-read can be delegated to different targets, per capability, per scope, per the user's posture. Nothing forces all needs onto a single keeper.

### Common capabilities

These are the capabilities the rest of the specification refers to. The list is not closed; it is the current set.

- **Availability.** Hold and serve a scope's encrypted objects for other members, including buffering for members who are offline. Holding availability does not imply read. A peer granted availability and not read holds others' ciphertext it cannot decrypt.

- **Read / search-offload.** Decrypt, index, retain, and serve a scope's history, so that a resource-limited peer can exercise its read right (§2.3) without holding and indexing the full history itself. This is the capability that makes deep search over long history practical for a phone.

- **Gating.** Act on the distribution or visibility of content within a scope, per the scope's own governed rules. See §2.7, which flags gating as the one capability that requires care against the rights floor.

---

## 2.5 Capability, role, and PeerSet

The earlier drafts used "role" for two different things, which caused real confusion. This section separates them into three layers with different change semantics. Getting the layers distinct is the point of the section.

### 2.5.1 The three layers

**Capability.** An atomic delegated power: availability, read/search-offload, gating (§2.4). A capability is fluid. Granting and revoking one is the normal operation, expected, no alarm. Each grant is a governance fact (§2.6).

**Role.** The ad hoc set of capabilities a peer actually holds at a given moment. A role is *descriptive*: it answers "what does this peer have right now." Roles compose by union, mutate freely as capabilities are granted and revoked, and trigger no alarm when they change, because change is their nature. "This peer currently holds availability and read-offload" describes a role.

**PeerSet.** A named, pinned, group-recognized bundle of roles and capabilities, with an *enforced composition* that includes both a required set and a forbidden set. A PeerSet is *prescriptive*: it answers "what is a peer of this name supposed to have, and supposed not to have." A meer is a PeerSet. Unlike a role, a PeerSet's composition is fixed and is part of its meaning, so drift from the pinned composition is an integrity event the group flags (§2.5.3).

The relationship nests cleanly: a capability is atomic, a role is the set of capabilities a peer holds, and a PeerSet is a *named and pinned* set of roles and capabilities that the group depends on meaning exactly what it says.

### 2.5.2 None of the three touches rights

All three layers live entirely in the capability plane. A meer has full standing and the complete rights floor (§2.3); its pinning is purely a capability-composition constraint. The discipline from the earlier draft holds for every layer, stated as a mechanical check:

- Every role and every PeerSet MUST be definable as `floor + [explicit capability composition]`. If it can be written that way, the name is legitimate.

- If a name would mean "and this peer is entitled to less," it is forbidden. Rights have no presets. Only capabilities do.

### 2.5.3 Roles compose freely; PeerSets are pinned and drift-flagged

This is the distinction the earlier draft got backwards. The two layers have *opposite* change semantics, and both are correct:

**A role composes by union and changes without alarm.** A peer may hold several capabilities at once; the result is just their union, still floor-plus-capabilities, still full standing. Acquiring or losing a capability changes the peer's role, and that is normal.

**A PeerSet's composition is pinned, and deviation is an alarm.** A PeerSet declares a required set of capabilities and a forbidden set. A peer that *declares* a PeerSet is asserting (as a governance fact) that it conforms. The group then runs a continuous consistency check: gather the capability grants in force for the peer from the governance log, and compare against the declared PeerSet's required-and-forbidden composition.

- Match is normal.

- Mismatch in *either direction* is the "errant and dangerous" signal. A peer declaring the meer PeerSet that has somehow acquired a forbidden capability (read) is dangerous in the obvious way. A peer declaring the meer PeerSet that has *lost* a required capability (availability) is also flagged, because it is failing the job the group relies on it for. Both are "this peer no longer matches what it claims to be."

The check is mechanical, not a judgment call, because both sides are facts already in the governance log: the declared PeerSet is a statement of record, and the capability grants are governance facts. Drift detection is therefore a consistency check between two things the log already contains, not a new mechanism. This is why a PeerSet is *safe* to rely on: "it is a meer" is a trustworthy statement precisely because the group enforces the pinned composition and flags any peer whose facts diverge from it.

The fixity does double duty. It is a convenience (the group reasons in terms of known PeerSets instead of raw capability lists) and a tripwire (a peer whose actual capabilities drift from its declared PeerSet is detectable *because* the PeerSet said what to expect).

### 2.5.4 Worked examples

The meer PeerSet, the availability-only configuration (informally, a "blind relay"). The forbidden clause is as load-bearing as the required clause:

```
meer (a PeerSet) ::= floor
                   + requires { availability }
                   + forbids  { read, * conferring read }
```

A meer has full standing and the full rights floor. It satisfies read-your-own-local-history vacuously (§2.3), because it requires availability and forbids read, so it holds no plaintext of its own. Its blindness is the enforced absence of read, not a special restricted nature. A peer declaring the meer PeerSet that acquired a read grant would be flagged as drifting from its declared composition (§2.5.3). Delete the PeerSet name and nothing about the peer's rights changes; the name is shorthand for the pinned composition the group enforces.

An ad hoc role, by contrast, has no forbidden clause and no drift alarm:

```
role(some-peer) = floor + { availability, read/search-offload }
```

This describes a peer that has been delegated both buffering and history-indexing, perhaps for different scopes by different parties. It is not a named PeerSet; it is just the union of what the peer currently holds. It can change freely.

A PeerSet may also be defined by stacking required capabilities, as long as the composition stays pinned and the forbidden clause is explicit where it matters:

```
search-delegate (a PeerSet) ::= floor
                             + requires { availability, read/search-offload }
                             + forbids  { gating }
```

The difference between this and the ad hoc role above is not the capability list. It is that the PeerSet is *named, pinned, and drift-checked*, while the role is *descriptive and free*.

---

## 2.6 Revocation reuses governance machinery

A capability grant is a fact in the governance log (§X.3.1). Revoking it is a new fact that supersedes the grant, resolved by the same total order and fold as any other governance conflict (§X.3.2, §X.8).

For capabilities that confer read (read/search-offload, and any future content-reading capability), revocation MUST rotate the scope epoch, so the revoked peer cannot read content authored after the revocation folds in. This is identical to the membership-expulsion mechanism (§X.5 R5). Revoking a delegate is, formally, an expulsion-shaped fact.

This inherits §X's honest limit, which §2.3's rights wording already anticipates: revocation protects the future, not the past. A peer that held a read capability while it was valid may have retained what it read. Revocation stops future access; it cannot unmake what was legitimately held. The plain-language form, which a non-expert can hold: you can revoke a delegate's access to new content at any time, and the revocation actually takes effect, but the delegate may keep copies of what it already saw. This is true of every party anyone has ever shared anything with, and stating it plainly is more honest than implying otherwise.

---

## 2.7 Open item: gating against the rights floor

Availability and read/search-offload are clean additive capabilities. They aid a peer in exercising rights, or hold ciphertext, and neither touches another peer's rights.

Gating is different and is flagged here rather than waved through. A gating peer acts on the distribution or visibility of content, which functionally bears on other peers' ability to exercise their read right (§2.3). So gating is the one capability where a delegated capability appears to bump against others' rights, which the additive framing does not automatically dissolve.

The likely resolution, to be specified rather than assumed: gating acts on distribution and visibility within a scope's own governed rules, not on the underlying right to read what one legitimately holds, and every gating action is itself a governed, attributable fact subject to §X. A gated peer's right to its own local history (including anything it already holds) is untouched; what gating affects is what propagates within the scope going forward, under rules the scope itself set.

> `ENABLING:` The precise relationship between a gating action and the read right must be specified: what a gating action can and cannot affect, how it relates to content a peer already holds locally, and how it is bounded by the scope's governed rules so that it cannot become a backdoor for suppressing a right under the guise of a capability. This is the one capability that could, if specified carelessly, re-introduce a rights distinction through the capability layer, and therefore the one that most needs an explicit forbidden clause wherever it appears in a PeerSet.

---

## 2.8 Exitability: the backstop that makes flexibility real

A delegated capability is only meaningfully different from an implicit, structural dependency if the delegation can be withdrawn and restructured without loss of rights. This subsection states that requirement, because it is what separates Drystone's delegate model from a server you cannot leave.

### 2.8.1 The requirement

Any default delegation a peer or group adopts MUST be revocable and restructurable down to the rights floor (§2.3) at any time, with:

- **no loss of rights**, and

- **only graceful degradation of capacity**, never loss of function or standing.

Concretely: a group that has delegated availability and search-offload to some peer (including a cooperative anchor, or any single operator) can move that delegation to a different peer, split it across several, pull it back into its own device groups, or drop it entirely, and in every case the group continues to function. What degrades is capacity (deep search may get slower or require a member's own device to be online; offline buffering may shrink), never the group's rights or its ability to communicate and govern itself.

### 2.8.2 Why this is the backstop, not the fine print

The value of this guarantee is not measured by how often it is exercised. Most groups, most of the time, will sit on a sensible default delegation, and that is correct, because a UX that forced every user to explicitly weigh tradeoffs before getting started would be a different failure (§2.4). The default delegated experience is what gives immediate utility.

The guarantee is measured by being unconditionally available to the minority who exercise it. Respecting the peer or group that wants to restructure or exit is constitutive of the system's health, in the same way a right you cannot exercise is not a right. The backstop is what makes "peers are equal in rights, not capabilities" true under stress rather than only at rest. An 80% default pattern is healthy precisely because the other 20% can deviate without penalty to their standing.

This is also the structural answer to the concern that a good-UX default delegate is indistinguishable from a server. At the surface, in the common case, it may well look identical. The difference is not at the surface; it is at the break. When trust fails, a Drystone group restructures or exits its delegation and loses only capacity. A group on a server it cannot leave loses everything and starts over. The delegate relationship is *explicit, granted, and exitable*; the server relationship is *implicit, structural, and captive*. The two can present the same experience right up until the moment the difference matters, which is exactly the moment it must hold.

### 2.8.3 Asymmetry of expressible range

This yields a precise, checkable claim, and it is worth stating as such rather than as a quality judgment: a flexible model can present as the rigid one, but the rigid one cannot present as the flexible one. Drystone can be configured to behave like a single-keeper, server-shaped deployment (one delegate holds everything for everyone). A server-shaped system cannot be configured to behave like the exitable, restructurable, per-capability-delegated model, because its central dependency is structural rather than granted.

This is an asymmetry of *expressible range*, not a claim that one is better. It is checkable in principle: enumerate the configurations each model can express, and the flexible model's set properly contains the rigid model's. The open design question this section hands forward is whether the flexibility holds *in the default case* and not only at the unused margins. If the exit and restructure path is genuinely lossless-in-rights and graceful-in-capacity for anyone at any time, the human structure is represented faithfully. If the backstop works only in theory, or only for experts, the model has quietly collapsed toward an extreme. That is the thing to press the specification on.

---

## 2.9 Summary

- The only canonical state is local. Everything else is comparison or assertion, which is fine and distinct, and it is the honest atomic unit of decision-making, since every decision in any system is made on local state (§2.1).

- There is one kind of peer. Differences are in rights (universal) or capabilities (additive, delegated), never in standing (§2.2).

- Rights are universal and never delegated. Every peer may read its own local history; a scope member may read scope history for the period of membership; a scope holds full history for itself (§2.3).

- Capabilities are additive, delegatable to either one's own device group or a shared-scope peer using one primitive, and revocable (§2.4).

- Three capability-plane layers, none touching rights: a **capability** is atomic and fluid; a **role** is the ad hoc set a peer holds, composing freely; a **PeerSet** is a named, pinned bundle with required and forbidden composition, where drift in either direction is flagged as a mechanical governance-log consistency check (§2.5).

- Revocation is an epoch-rotating governance fact; it protects the future, not the past, which is correct rather than a shortcoming (§2.6).

- Gating is the one capability that must be specified carefully against the read right, and is flagged open (§2.7).

- Any default delegation MUST be revocable and restructurable down to the floor with no loss of rights and only graceful capacity degradation. This backstop is what makes the flexibility real, and it yields a checkable asymmetry: the flexible model can present as the rigid one, but not the reverse (§2.8).
