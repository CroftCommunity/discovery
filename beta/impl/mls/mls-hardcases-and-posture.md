# MLS Hard Cases and Drystone's Posture

`Status: draft, for the Drystone spec suite`

`Scope: the places MLS (RFC 9420 / RFC 9750) assumes a coordinator or an infrastructure guarantee, and the posture Drystone takes at each; plus a concept-alignment map (§10) folding Drystone concepts onto their native MLS representations where they exist. Governs design, not validation.`

`Companion to: 01-delivery-architecture.md (delivery planes and gap-aware convergence), 09-provenance.md (retention classes), Part 1 (principles). MLS vocabulary is defined in mls-overview-and-terms.md and restated here where load-bearing.`

---

## Why this doc exists

MLS is specified against a Delivery Service that, in most deployments, supplies a coordinator: a single party that orders commits so the group's history stays linear. Drystone removes that coordinator. This doc states, for each place the coordinator assumption is load-bearing, what MLS requires and what posture Drystone takes instead.

The organizing claim, stated once and then applied: MLS supplies a local cryptographic check almost everywhere it needs agreement, and assumes a coordinator supplies the global agreement. Drystone does not re-add the coordinator. For each case it decides whether the global agreement is unnecessary, reconstructible without coordination, or a social-utility judgment that must escalate to humans. The third kind is not a failure mode. It is the designed terminus named in Part 1 §2.5.

---

## Terms

This doc coins little; it inherits from MLS and from Part 1. Working definitions, with source of inheritance noted. Where a definition here diverges from the source, that divergence is information for the merge, not an error to smooth over.

`epoch`: a version of an MLS group between two changes. Epochs form a strictly linear chain, one epoch per predecessor, enforced by the transcript hash. (RFC 9420 §2 definition; §8.2 transcript-hash mechanism.)

`commit`: the message that enacts a set of proposals and advances the group to a new epoch. Any member may send one. (RFC 9420 §12.4.)

`transcript hash`: the hash chain binding each epoch to the commit that produced it and to the prior epoch, maintained as the confirmed and interim transcript hashes. It represents exactly one commit sequence and has no representation for a branch or a merge. (RFC 9420 §8.2.)

`ratchet tree`: the binary key-distribution tree with members at the leaves. A member knows a node's secret key iff its leaf sits in that node's subtree. Re-keying costs log(N) in the well-shaped case. (RFC 9420 §7.)

`blank node` / `unmerged leaf`: a keyless hole left by removal / a member not yet holding an ancestor node's secret. Both raise re-keying cost toward O(N) until healed by a commit through the affected nodes. (RFC 9420 §7.)

`Authentication Service (AS)` / `Delivery Service (DS)`: the two abstract services MLS relies on. The AS binds identities to keys and is highly trusted. The DS routes messages and stores KeyPackages, is untrusted for confidentiality, and can attack metadata and availability. In common deployments the DS also orders commits. (RFC 9750 §4 AS, §5 DS.)

`re-plant`: Drystone's operation of reading current membership from the governance chain, instantiating a fresh MLS group over it, and atomically repointing the conversation pointer. The MLS group is disposable; the conversation persists across a sequence of them. (Drystone; realized against 01-delivery-architecture.md.)

`ReInit` / `branching`: the two MLS operations that link a new group to an old one via a resumption PSK. ReInit closes a group and re-forms it over the same members with new parameters; branching starts a new group over a subset, leaving the original intact. They are the MLS-native shapes of Drystone's re-key and fork arities. (RFC 9420 §11.2, §11.3; overview §2.9.)

`resumption PSK`: a per-epoch pre-shared key that, injected into a linked group, proves co-membership at the source epoch irrespective of intervening key changes. It proves entitlement, not content. (RFC 9420 §8.6.)

`persona` / `node`: a persona is the human layer's manifestation, the entity rights and weight attach to. A node is a device. A persona has devices that are nodes; the two are never the same layer. (Part 1 §2.3.)

`right` / `role`: a right is the floor of equal standing (voice, tenure, exit/fork), held identically by every persona and unremovable. A role is a delegated, scoped, revocable admin-type authority a persona is charged with. Removing a role leaves standing intact; that is the test distinguishing the two. (Part 1 §2.3.)

`GroupInfo`: an MLS object describing a group's current public state, published so an external client can join by external commit. In stock MLS a rejoining client treats it as authoritative. (RFC 9420 §12.1.6.)

`external commit`: a join path where the joiner drives its own admission using a published GroupInfo, used when existing members hold no KeyPackage for it. (RFC 9420 §12.1.6.)

`governance threshold`: Drystone's per-scope, tunable rule for how many cosigners a governance change requires. It is a dial set to the group's trust posture, low for high-social-trust groups, high for adversarial ones. (Drystone; Part 1 §2.3 dial discipline.)

`OOB history convergence`: Drystone's reconciliation of the durable dataplane hash tree, run out of band from the live MLS message stream via anti-entropy, so retained or replayed old state never re-enters live protocol operation. (Drystone; realized against 01-delivery-architecture.md gap-aware convergence.)

`resolvable` / `residue`: a conflict is resolvable if monotonic, so causal and cryptographic ordering yields a determinate answer with no human and no clock. The residue is the provably non-empty set of concurrent non-monotonic conflicts that have no coordination-free determinate resolution. (Part 1 §2.5.)

Epistemic legend, used inline below: *Verified* (checked against a primary this round), **[confirm]** (load-bearing, to be checked against the cited primary before freeze), *Synthesis* (the design's own reasoning, to be judged as reasoning).

---

## The cast

One fixed cast, carried through every case, so the trust semantics stay concrete. Following the security-protocol convention and RFC 9750's own scenario style.

- **Alice**: a persona in a group. Runs two nodes, a phone and a laptop, so she holds two MLS clients with one identity. Her second device is the recurring vehicle for multi-client and staleness cases.

- **Bob**: a persona in the same group, one node. The recurring vehicle for compromise-and-heal and for the "briefly offline" case.

- **Carol**: a persona in the same group, one node. The recurring counterpart in the mutual-expulsion case (Carol and Bob expel each other at equal standing).

- **Dave**: a persona granted a needed role (say, the sole admin). The vehicle for role under-determination when his only node goes stale.

Each name carries its trust meaning unchanged wherever it appears.

---

## 1. The linear epoch chain

**Requirement MLS places on the deployment.** Exactly one commit must win each epoch, because the transcript hash can represent only one commit sequence. In common deployments the DS meets this by ordering commits: an ordering server broadcasts a single sequence and clients apply the first valid commit per epoch. *Verified* against RFC 9750's strongly-consistent DS description.

**Why this is a problem here, not there.** Now suppose Bob and Carol each send a commit against the same epoch, and no ordering server exists to pick one. Both are valid. MLS has no representation for this other than two incompatible histories, and no MLS-level operation stitches them back together. The alternative to solving it (letting the two histories persist inside one MLS group) is not available: the transcript hash forbids it structurally.

**Posture.** Do not ask MLS to represent a fork. The MLS group is key-distribution infrastructure with a lifespan, subordinate to the application-layer structures that carry continuity: the dataplane hash tree (conversation history) and the governance chain (membership and roles). A conversation persists across a sequence of MLS groups; the MLS group identity is not the conversation identity.

**Why this posture and not coordination.** *Synthesis.* The linear chain is a coordination mechanism, and coordination over the non-monotonic residue is exactly what Part 1 §2.0's razor forbids. Using the epoch chain to resolve a governance fork would take the coordinate horn of the CALM dilemma that §2.5 rules out. So the chain's linearity is not a limitation to defeat; it is a mechanism kept for the layer where linearity is harmless (the monotonic dataplane) and declined for the layer where it is not (the governance residue).

---

## 2. Fork as escalation, not failure

**The distinction that governs the rest of this doc.** Part 1 §2.5 splits conflict into the resolvable class and the residue (see Terms). The resolvable class settles by the fold. The residue (Carol expels Bob while Bob expels Carol at equal standing; a removed-then-included merge) has no coordination-free determinate resolution, because a total social order over concurrent non-monotonic operations is itself non-monotonic. *Verified* against Part 1 §2.5.

The residue is not uncomputable by this machine; it is not a computation at all. When Bob and Carol expel each other at equal standing, no fact about the world makes one correct. The question is a value, held by the people whose relationships are at stake, not a fact awaiting discovery. *Verified* against Part 1 §2.5.

**Requirement this places on the key layer.** A fork must be expressible as an outcome, cleanly, without either concurrent commit silently winning. *Synthesis.*

**Realization: one primitive, three arities.** Fork, heal, and routine re-key are the same operation (re-plant, see Terms) at different arity.

- Legitimate governance fork: the split is real, so two conversations are correct. Plant two fresh groups, one per branch, each seeded from its branch's membership, each carrying a default message stating the conflict and the resolution options. One conversation lineage becomes two.

- Accidental fork (a benign concurrency artifact, no governance divergence): the split is spurious, so heal to one. Plant one fresh group over the reconciled membership; repoint both former branches to it.

- Routine re-key: repoint one to one.

The MLS group never represents any of this. It is always just the current key layer for whichever conversation points at it. These three arities map onto MLS's native ReInit and branching operations, whose freeze-then-strand hazard is treated in §9. *Synthesis; realized against 01-delivery-architecture.md.*

**Why the classifier stays governed.** Whether a given concurrent contradiction is a real dispute or a benign artifact is itself partly a utility judgment, vulnerable to alarm fatigue, so it is a per-scope governed tolerance over verifiable provenance signals, not a hardcoded constant. *Verified* against Part 1 §2.5. The accidental-heal path is valid only when governance did not diverge: both branches must carry identical authoritative membership, with the fork purely below the governance layer. Any governance divergence, even one contested act, disqualifies auto-heal, because healing it would manufacture the verdict §2.5 forbids. The mechanism is identical either way; only the decision of which arity to apply is governed, and it must stay governed.

---

## 3. The escalation set has a second member

**Requirement.** The escalation machinery must catch the whole residue, not one visible kind of it. *Synthesis.*

**The two members.** Part 1 §2.5 derives fork-not-verdict for contradiction: too many valid claims (mutual expulsion). Role under-determination is a distinct second member: too few. Now suppose Dave holds the sole admin role and his only node goes stale and is pruned (see §4). The survivors need the role filled and cannot agree who fills it. There is no conflicting act to order; there is an absence no cryptographic operation should fill.

Both members share the defining property: provenance is fully settled (everyone agrees Dave is gone, the role is vacant, these are the survivors) and utility is open (what to do about it). That is the razor's seam. They differ only in detection: contradiction is seen as concurrent conflicting operations; under-determination is seen as a required role vacant with no valid grant available. A mechanism watching only for contradiction misses under-determination. *Synthesis; proposed for explicit naming in Part 2 §7.6,* **[confirm]** *against that text.*

**Posture.** Under-determination is expected and is explicitly not given a technical resolution, because it is a social-utility judgment. Drystone supplies the conditions for cheap human resolution, not the resolution:

- Legibility: every node deterministically computes the situation (role vacant, these survivors, this governance posture). This is Part 1 §2.2 (P-Knowable-Truth) ensuring humans adjudicate over one corroborated picture.

- Cheap exits in every direction the humans might pick: re-delegate the role (one grant), run with the role unfilled (do nothing; the group persists role-less, honoring the no-helper path of Part 1 §2.4), or split (two plants).

- Refusal to manufacture the missing authority: the moment the tree operations invent a role-holder the survivors did not agree on, a center is certifying utility, which §2.0 forbids.

The aftermath of the human decision is nearly free to instantiate in every direction. The judgment stays with the humans; the instantiation is Drystone's and it is cheap. *Synthesis.*

---

## 4. Rights versus roles under key hygiene

**Requirement MLS states.** A persistently-offline member holds stale key material that is a standing forward-secrecy and post-compromise-security liability if later compromised, and MLS says such members should eventually be removed and that updates should be sent at regular intervals. *Verified* against RFC 9750 §8.3 (not much can be done on the inactive side) and RFC 9420 §16.6 (updates SHOULD be sent at regular intervals; non-updating members SHOULD eventually be removed). Neither "eventually" nor "regular" has an enforcer without a coordinator.

**What the removal touches, and what it does not.** Now suppose Alice stops using her laptop; its keys go stale. Pruning that node touches resources (a node fact) and, if it held one, a role. It does not touch Alice's rights floor: she keeps voice, tenure, and exit/fork, so she keeps full standing to contest, rejoin, and fork. The hard sub-case (Dave's only node, holding a needed role) is therefore not a rights violation; it is capacity degradation that resolves through the role/right split, adapting by re-delegation, running role-less, or the §3 fork. *Synthesis, grounded in Part 1 §2.3 rights/roles.*

**Why staleness, not event-count, is the trigger.** The variable the hazard is about is per-member key age, readable by every node from the shared tree with no coordinator, so "a leaf unrefreshed longer than the scope's tolerance is stale" is a deterministic, cross-corroborable predicate on the same epistemic footing as governance-chain reads. An event-count trigger (every Nth join) measures group activity instead, and the proxy is loosest exactly where risk is highest: a quiet group never triggers yet is likeliest to harbor a long-offline node. So event-count serves only as a cheap secondary poke, never the primary trigger. *Synthesis.*

**Why the action scales to what tripped it.** A single stale node in a healthy tree needs only a targeted Remove (any member may commit it), which blanks the path and re-keys at log-ish cost; a re-plant would be overkill and, worse, blunt, since it prunes anyone lacking a fetchable KeyPackage at plant time and so conflates "long stale" (Alice's abandoned laptop) with "briefly offline" (Bob on a plane). A degraded tree, or a governance-boundary crossing, is the case where the full re-plant is right, because it resets staleness for free while reconstructing anyway. *Synthesis;* KeyPackage-at-plant-time requirement *Verified* against RFC 9750 group-creation flow.

**What is codifiable at group creation, and what is not.** This is the provenance/utility line, and overshooting it re-installs a center.

- Codifiable (provenance, deterministic, cross-corroborable), belongs in the governance chain: the staleness predicate and threshold T; the event-count secondary poke if used; the re-plant trigger conditions; and the space of permitted responses with the group's current selection within it.

- Not codifiable (utility, stays human): whether to act on a given firing, and how; whether a specific stale node is "gone, prune" or "seized device, hold the seat."

Codifying the action with the predicate ("stale beyond T implies automatic hard removal") collapses the utility call into the provenance rule, making the threshold an unappealable authority that evicts people for being offline. The dial-discipline fix (Part 1 §2.3): predicate codified and mandatory; response a per-scope governed dial with the aggressive-auto-prune end available but not default and never the only representable option, and removal treated as rejoinable-without-ceremony so a returning node recovers full rights and history to the point of removal. *Synthesis, grounded in Part 1 §2.3 dial discipline and §2.4 graceful degradation.*

---

## 5. External-join recovery: a stale GroupInfo cannot defeat PCS here

**Requirement, and the MLS hazard.** When a client cannot process a commit, a common recovery is to rejoin by external commit using a published GroupInfo. Stock MLS treats that GroupInfo as authoritative, and the recovering client trusts it to represent the current group state rather than an earlier state containing a compromised leaf. An adversary who can feed a victim an unprocessable commit and then serve a stale GroupInfo can drive the victim to rejoin into a compromised epoch, defeating post-compromise security, or serve a corrupted GroupInfo to block rejoin entirely, a denial of service. *Verified* against RFC 9750 §8.1.4 and §5.3 (invalid-commit and external-join recovery hazards). In a center-free mesh this is sharper than in a server deployment, because GroupInfo is served by peers, so any peer, not one identifiable DS, can mount it.

**Why this is a problem there and not here.** The hazard exists because in stock MLS the GroupInfo is the only thing a rejoining client can check against, so a single untrusted artifact is authoritative. In Drystone the GroupInfo is not the authority; the governance chain is. Now suppose a peer feeds Bob an unprocessable commit and serves a stale GroupInfo pointing at a superseded epoch. Bob does not trust it; he corroborates it against the governance chain he already holds, and a GroupInfo pointing at a membership or epoch the chain has already moved past is detectable locally, with no trusted third party. The MLS step "trust the GroupInfo" is demoted to "treat the GroupInfo as a claim, corroborate against the authoritative chain." *Synthesis.*

**Two stacked defenses, kept distinct.** They discharge different halves of the hazard and are not one mechanism.

- The monotonic fold (Part 1 §2.2) means a member who asserts a governance state that does not reconcile against the rest of the group has not rewritten group state; they have diverged their own view and forked themselves out, a fork of one. The fold never reverts, so a bad assertion cannot roll the group back, only fail to advance in step with it. This is what makes a forged assertion self-forking rather than corrupting.

- The governance threshold (see Terms) sets how expensive a forged assertion is to attempt at all. MLS leaves the hazard unquantified because it has no membership-change authorization model; any member may commit. Drystone quantifies it per group: a change requires N cosigners, and mounting the attack costs compromising a quorum, not one member.

The threshold is a posture dial expected to track context. A nuclear family joined in person by QR has strong identity-axis social trust settled out of band at join, so it can run survivable low thresholds. A journalist or activist group has weaker social trust and an adversarial posture, so it runs strict thresholds, raising the attack cost accordingly. This is not only a mitigation; it is a per-group quantification of an attack the spec leaves unquantified. *Synthesis, grounded in Part 1 §2.3 dial discipline.*

**The residual, owned.** The resolution rests on a rejoining node being able to tell a stale GroupInfo from a current one by its own governance view. The case still to stress-test is a GroupInfo pointing at a recent-but-superseded epoch when the rejoining node is itself far enough behind on the governance chain that its view has not advanced past that epoch. The §2.2 property (a node behind on the fold under-authorizes but never mis-authorizes) suggests this holds, but it is reasoned, not proven here. Carried to the discussion thread and to Open items. **[confirm]**

---

## 6. Insider replay and nonce reuse on restore: isolated by out-of-band convergence

**Requirement, and the two MLS hazards.** MLS does not protect against replay by insiders: a member can re-inject an old application message, and the per-sender generation counter detects a gap but does not prevent replay within the sequence. *Verified* against RFC 9750 §8.6. Separately, MLS carries a reuse_guard because a client that reverts to earlier state (for example from a backup) can reuse a nonce and break AEAD security. *Verified* against RFC 9420 §6.3.1 and §16.7.

**Why one mechanism answers both.** Both hazards are the same shape: old bytes re-entering the live protocol stream, an old message re-injected as new, or old key state reused after a revert. Both are dangerous only if the durability and history layer shares a stream with live protocol operation, so retained old state can pollute live operation. Drystone runs OOB history convergence (see Terms): anti-entropy reconciles the durable hash tree out of band and never re-injects into the live MLS stream. So a replayed old message arrives as a content-addressed history-tree entry to be reconciled, idempotent and non-advancing, not as a live MLS application message to be processed; the replay is inert because it hits the convergence path, not the live path. The same separation covers nonce reuse: a node restoring from a snapshot restores history-tree state on the OOB layer, not live MLS key material, so the revert-and-reuse scenario the reuse_guard exists for does not arise. *Synthesis; realized against 01-delivery-architecture.md gap-aware convergence.*

**The residual, owned.** This holds only if a node never restores a live MLS group's secret state from a snapshot and resumes sending in the same epoch. If the recovery model is always "re-plant or re-join fresh, converge history out of band," live epoch secrets are never restored and the hazard is structurally absent. If any path resurrects a live group's ratchet or secret-tree state and resumes in place, the hazard returns. This property of the restore path is not yet confirmed against the recovery design. **[confirm]**

---

## 7. Forward secrecy and durable history are not in tension

**What forward secrecy actually assumes.** Forward secrecy is defined against ciphertext retention: it assumes an adversary holds all the ciphertext, and delivers its guarantee by requiring that the keys to that ciphertext be deleted on schedule. So a durability node holding sealed bytes indefinitely is within the exact scenario forward secrecy is built to survive, provided keys were deleted on schedule. The deletion schedule operates on keys, never on ciphertext. *Verified* against RFC 9750 (FS holds against access to all encrypted traffic history combined with current keying material, provided keys are deleted after use).

**Where friction does live.** Two frictions remain, both about keys, not ciphertext.

- Key retention to process reordering: in decentralized settings a functional DS requires members to retain key material to process commits out of order, which violates the deletion schedule and reduces forward secrecy. This is the seam a reorder-prone delivery layer creates. *Verified* against draft-kohbrok-mls-decentralized-mls-00.

- The persistently-offline node (§4): forward and post-compromise security rely on active deletion and replacement, so an offline node holding old keys is a residual hazard MLS admits it cannot fully close. *Verified* against RFC 9750.

**Self-deleting messages are a different object at a different layer.** MLS has no disappearing-message feature; its deletion schedule is a key mechanism that produces forward secrecy and says nothing about content. Drystone's chosen-ephemeral retention (a signed "do not retain past T" disposition honest clients apply) is a content-layer policy, defined in 09-provenance.md, not a transport mode. MLS's deletion schedule neither implements nor conflicts with it; the two touch different objects (keys versus content), and the layer separation must be kept when specifying either.

**The residual, owned.** Chosen-ephemeral retention is cooperative non-retention, never enforced deletion: a node cannot prove it expunged, and a modified client keeps what it read. This is stated where the reader would otherwise assume enforcement. *Verified* against 09-provenance.md.

---

## 8. Decentralized MLS (DMLS / FREEK): a research pointer, not a dependency

**Requirement it addresses.** The §7 key-retention seam: keeping forward secrecy acceptable when a reorder-prone, center-free delivery layer would otherwise force retaining key material.

**What it provides.** DMLS (draft-kohbrok-mls-decentralized-mls-00, March 2025), built on FREEK (Alwen, Mularczyk, Tselekounis; eprint 2023/394), modifies the MLS key schedule to derive multiple init secrets via a puncturable pseudorandom function keyed by a base init secret, so retained key material loses less forward secrecy, and it adds content-derived epoch identifiers (a byte string from the epoch secret) so forks off one integer epoch are uniquely identifiable. *Verified* against the draft.

**Status, owned honestly.** The draft is preliminary and is not a dependency. Its introduction and security-considerations sections are empty TODOs. Its state-consolidation procedure assumes two servers that coordinate to prevent forks, which is the inverse of Drystone's premise. It self-flags a residual cost: the PPRF gives forward secrecy for init secrets, but old ratchet-tree secret state must still be retained to process old messages, damaging their forward secrecy. Its consolidation tie-break (alphanumerically larger transcript hash) is unspecified as to whether determinism is required and cannot handle the no-member-overlap case. *Verified* against the draft.

**Posture.** Track DMLS/FREEK as the most relevant active work on the cost side of the §4 calculus, and adopt two ideas independently of the whole protocol: content-derived epoch identifiers, and the PPRF approach to forward-secure retained init secrets. Do not adopt its consolidation procedure; Drystone's consolidation is the governed fork/heal of §2, above the key layer rather than inside it. If DMLS matures on the cost side, it widens the "briefly offline" window of §4 without weakening forward secrecy as much; it does not change the §2 escalation posture, which the principles force regardless of key-layer efficiency. *Synthesis.*

---

## 9. ReInit non-atomicity: the freeze-then-strand window

**What MLS provides, and validates.** MLS's ReInit and branching operations (see Terms) are the native shapes of Drystone's re-plant family, linked by resumption PSK. This is strong external validation: the standards body converged on the same shape Drystone uses, close the old group and re-form over the membership, linked by a PSK that proves co-membership. ReInit is the re-key arity; branching is the fork arity. *Verified* against RFC 9420 §11.2, §11.3, §8.6.

**The hazard MLS names.** ReInit is not atomic, and the spec is explicit about the consequence. Committing a ReInit immediately freezes the existing group and triggers creation of a new group with a new group_id, but creating that new group and sending its Welcomes is a separate step. So a member can go offline after committing the ReInit but before creating the new group, and then another member must continue the reinitialization by creating the new group and sending the Welcomes. *Verified* against RFC 9750 §6.1 and §7. The freeze is instant; the re-form is not; the gap between them is a window in which the old group is dead and the new one does not yet exist.

**Why this is sharp here.** In a server-mediated deployment the committer is often reliable infrastructure. In a center-free mesh the committer is an ordinary peer, so the freeze-then-strand window is a routine risk, not an edge case: any peer that commits a re-plant and then loses connectivity strands the group. This is the concrete, spec-grounded form of the escalation-window question. The danger is not primarily two commits racing (that is §2); it is one commit that freezes the old group and then fails to complete the new one. *Synthesis.*

**The tradeoff the freeze buys.** Freezing first is not a defect; it purchases replay-immunity. Because a committed ReInit cannot be duplicated by a conflicting commit, the re-form is immune to the replay attacks that afflict resumable 0-RTT key material. *Verified* against draft-tian-quic-quicmls-00 (informative, a QUIC-over-MLS draft, not normative). So the design choice is replay-immunity at the cost of a stranding window, not a free lunch either way.

**Posture, and the candidate resolution.** The spec's own recovery (another member completes the stranded reinit) requires that a second member have enough authoritative information to finish the job. Drystone's candidate resolution: the governance chain records the re-plant intent (we are re-planting to membership M) before the freeze, so any member can complete a stranded re-plant from the authoritative instruction rather than needing the original committer. Whether the ordering is actually intent-recorded-before-freeze is the open question that decides whether this hazard is resolved or merely bounded; it is carried in Open items. If intent is recorded first, the hazard is discharged by the governance chain; if not, the stranding window is real and needs a different completion mechanism. *Synthesis; the ordering property is* **[confirm]** *against the delivery and governance-chain design.*

---

## 10. Drystone-to-MLS concept alignment

This section captures where Drystone concepts already have a native MLS representation, so overlapping ideas are folded together rather than built twice or left to fight. Each row carries a correspondence strength, because "these align" comes in three different strengths with three different consequences.

- **Direct**: the Drystone concept is the MLS construct, possibly generalized. Build on the MLS primitive; do not reinvent it.

- **Partial**: they align on one layer but not another, usually because MLS handles entitlement and Drystone handles content, or MLS states an intent that Drystone must supply the mechanism for. Use the MLS piece for its layer, supply the rest, and keep the boundary explicit.

- **Drystone-only**: MLS has no representation, by design or by omission. Drystone must build it; there is nothing to fold into.

- **Underused**: MLS offers a construct Drystone is currently reinventing or not leaning on. Candidate to adopt rather than rebuild. This is the row type worth hunting for, because it is where effort is being spent needlessly.

### Direct correspondences

Re-plant family maps onto ReInit and branching. Drystone's re-plant (read membership, instantiate fresh group, repoint) is ReInit generalized; the legitimate-fork arity is branching; the heal arity is ReInit over reconciled membership. All three are linked by resumption PSK. This is the strongest alignment in the suite and it lowers implementation risk: the primitives exist rather than being bespoke. *Verified* against RFC 9420 §11.2, §11.3, §8.6; treated in §2 and §9.

Entitlement continuity across a re-plant maps onto the resumption PSK. Drystone needs "these members carried over from the prior group"; the PSK proves exactly co-membership at the source epoch, irrespective of intervening key changes. Use it directly as the continuity link. *Verified* against RFC 9420 §8.6.

State-loss recovery (entitlement half) maps onto MLS §6.6 rejoin-with-PSK. A member who lost state rejoins as a new member and proves prior membership by resumption PSK. Drystone uses this directly for the entitlement re-proof. *Verified* against RFC 9750 §6.6.

### Partial correspondences

Conversation continuity is only partly the resumption PSK. The PSK carries entitlement continuity across a re-plant; it carries no content. Drystone's conversation persists across a sequence of groups by the dataplane hash tree, which the PSK does not touch. Use the PSK for the entitlement thread and the dataplane tree for the content thread, and keep them separate (overview §5.8). *Synthesis; grounded in RFC 9420 §8.6 and 01-delivery-architecture.md.*

State-loss recovery (content half) has no MLS representation. MLS rejoin proves prior membership but explicitly does not restore messages missed during the loss window. Drystone supplies that content backfill through out-of-band history convergence. So the recovery is MLS-for-entitlement, Drystone-for-content, two separate exchanges. *Verified* against RFC 9750 §6.6; content mechanism is Drystone's.

Staleness removal partly maps onto MLS's "eventually remove non-updating members." MLS states the intent (RFC 9420 SHOULD remove members who do not update) but supplies no mechanism and no enforcer. Drystone supplies the mechanism: a deterministic staleness predicate over the shared tree plus a governed response (§4). Use the MLS intent as the requirement; the mechanism is Drystone's. *Verified* against RFC 9420 and RFC 9750; mechanism is Drystone's (§4).

### Drystone-only, by design

Out-of-band history convergence has no MLS representation, and this is deliberate on both sides. MLS is a key-agreement protocol and explicitly does not carry durable history or restore missed content. Drystone's dataplane hash tree and gap-aware convergence are wholly application-layer. There is nothing to fold in; the alignment is that MLS stays out of history entirely. *Verified* against RFC 9750 (MLS does not restore missed content); mechanism is Drystone's.

The monotonic governance fold has no MLS representation. MLS enforces no access control on group operations; any member may commit any membership change. Drystone's append-only monotonic authority fold and its cosign thresholds are entirely Drystone's, layered above MLS. MLS's silence here is precisely why the §5 external-join hazard is unquantified in the spec and quantifiable in Drystone. *Verified* against RFC 9750 (MLS enforces no access control); mechanism is Drystone's.

Fork-not-verdict and the escalation set have no MLS representation. MLS's linear transcript chain cannot express a fork at all; the governed fork/heal lives entirely above the key layer. Drystone-only. *Verified* against RFC 9420 (single linear epoch sequence); treated in §2, §3.

### Underused: where Drystone may be reinventing

Whole-group consistency detection may already be provided by the epoch_authenticator. Drystone needs to detect that all members share the same group state (that no silent partition has occurred), and the design leans on the governance chain and out-of-band comparison for this. MLS derives a per-epoch epoch_authenticator specifically as the value members compare out of band to confirm they share the same state; two members on different branches compute different authenticators. This is a candidate for adoption rather than rebuilding: the question is whether Drystone's whole-group consistency check can use the epoch_authenticator directly instead of a separately-built comparison, and where the governance chain's own consistency signal and the epoch_authenticator overlap versus complement. Worth resolving so the two are folded rather than parallel. *Candidate;* **[confirm]** *the overlap against RFC 9420 §8.7 and the delivery-layer consistency design.*

The resumption PSK's cross-group linking may be underused for the parallel-groups PCS problem. MLS notes that a PSK exported from one group and injected into another carries some post-compromise-security properties across, which bears on the per-group-healing limitation (a compromised member must heal each group separately). Whether Drystone's re-plant family already exercises this or could use it to link healing across a persona's groups is an open thread, not yet examined. *Candidate; not yet examined.*

---

## 11. Posture summary

| Hard case | What MLS assumes | Drystone posture | Forcing principle |
|---|---|---|---|
| Linear epoch chain (§1) | DS orders commits, one wins per epoch | MLS subordinate; continuity in app-layer hash structures | Part 1 §2.0 |
| Concurrent commits / fork (§2) | one commit wins per epoch | fork/heal/re-key are one primitive, three arities; never represented in MLS | Part 1 §2.5 |
| Role under-determination (§3) | not addressed | second escalation class; expected; escalates to humans; cheap instantiation of their choice | Part 1 §2.4, §2.5 |
| Stale / offline node (§4) | should eventually be removed | mechanical staleness detector, governed response; rights untouched, capacity degrades | Part 1 §2.3, §2.4 |
| External-join recovery (§5) | GroupInfo is authoritative to a rejoining client | GroupInfo is a claim corroborated against the governance chain; threshold dial quantifies attack cost; bad assertions self-fork | Part 1 §2.2, §2.3 |
| Insider replay / nonce reuse on restore (§6) | no replay protection; reuse_guard needed on revert | isolated by out-of-band history convergence; live epoch secrets never restored in place | Part 1 §2.2 |
| FS versus durable bytes (§7) | keys deleted on schedule | no tension; FS assumes ciphertext retention; real risk is key retention under reordering | Part 1 §2.4 |
| Self-deleting messages (§7) | no such feature | application-layer content policy; cooperative non-retention | Part 1 §2.4 |
| Decentralized operation (§8) | strongly consistent DS | track DMLS/FREEK for the cost side; adopt epoch-id and PPRF ideas, not the consolidation | engineering |
| ReInit non-atomicity (§9) | committer completes the re-form, or another member does | re-plant is the native ReInit/branching shape; freeze-then-strand window closed if governance chain records intent before freeze | Part 1 §2.4 |

---

## Open items

Distinct from the decided-and-bounded postures above; these are genuinely undecided.

- The §3 under-determination generalization, against the full Part 2 §7.6 text. **[confirm]**

- The KeyPackage-exhaustion seating trilemma. For an offline member who has exhausted pre-published KeyPackages at a boundary, at most two of three hold: fresh-KeyPackage forward secrecy, offline seatability, and avoiding the external-join path (§5). Reusable last-resort KeyPackages buy offline seatability at an FS cost (RFC 9420 §16.8); refusing them forces either late joining or the external-join path. The resolution is likely another posture dial, but whether the external-join corner is safe enough to be a default depends on the §5 GroupInfo-validation defense holding under adversarial analysis. *Open thread, to be talked out.*

- The §5 residual: whether a rejoining node far behind on the governance chain can always distinguish a recent-but-superseded GroupInfo from a current one. The §2.2 under-authorize-never-mis-authorize property suggests yes; reasoned, not proven. **[confirm]**

- The §6 residual: whether the recovery design ever restores a live group's epoch secrets in place (which would reopen the replay/nonce hazards) versus always re-planting or re-joining fresh. **[confirm]**

- The escalation-window behavior, now grounded as the ReInit non-atomicity hazard (§9): whether the governance chain records re-plant intent before the freeze, so any member can complete a stranded re-plant. If yes, §9 is discharged; if not, the freeze-then-strand window needs a different completion mechanism. **[confirm]** against the delivery and governance-chain design.

- The re-plant seating default at a boundary: Welcome-seating versus external-commit-seating, which moves the KeyPackage-availability burden between planter and joiner and changes what an unreachable node does to boundary closure. Note §5 reweights this: the external-commit path carries a PCS-integrity hazard the Welcome path does not, even with the governance-chain defense. *Undecided.*

- The communal-namespace key construction for a group-as-principal under membership change (Part 1 §2.3 flags designed-not-frozen; Part 2 §5.10). **[confirm]**

- Tenure under re-key: whether the survivor/re-key path can strand a persona's tenure (Part 1 §2.3, §2.4). **[confirm]**

- The exact §8.2 transcript-hash construction formula (the precise inputs to the confirmed and interim transcript hashes) was not read verbatim this session. The section structure and the entropy and deletion mechanics are grounded in primary text (RFC 9420 §2, §8, §9.2, §12); the step-by-step §8.2 hashing formula should be read verbatim before any Drystone text restates it as a construction rather than as the "one linear commit sequence" property it relies on. **[confirm]**

- Epoch-number metadata leak versus re-plant frequency. MLS header metadata is an opaque group_id plus a numerical epoch equal to the count of changes made to the group, and while not individually sensitive, it can reconstruct sensitive information under correlation by a network observer (*Verified* against RFC 9750 §8.1.2). The monotonic epoch counter therefore leaks a group's change rate, which correlates with membership churn and administrative activity even when content is opaque. This bears on the re-plant model specifically: re-plant is cheap and so freely used, but each re-plant reads as group activity to an observer, so the cheapness that makes re-plant attractive also makes it a metadata emitter. Two mitigations to weigh: the DMLS content-derived epoch identifier (§8) changes this leakage profile because a hash-like id leaks differently than a monotonic count, and the RFC's own recommendation is to carry MLS over a transport providing metadata confidentiality (TLS or QUIC), which in a center-free mesh is the delivery layer's concern, not MLS's. *Open thread; interacts with the delivery-layer metadata posture in 01-delivery-architecture.md.*

- The epoch_authenticator overlap (§10 underused row): whether Drystone's whole-group consistency detection can use the MLS epoch_authenticator directly rather than a separately-built comparison, and how it relates to the governance chain's own consistency signal. **[confirm]** against RFC 9420 §8.7 and the delivery-layer consistency design.

- The resumption-PSK cross-group linking (§10 underused row): whether the re-plant family already exercises, or could use, the PSK's cross-group PCS-carrying property to link healing across a persona's parallel groups. *Not yet examined.*
