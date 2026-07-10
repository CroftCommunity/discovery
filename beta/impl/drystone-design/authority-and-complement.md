# Drystone: Authority, Capability, Fork, and the ATProto Complement

`Status: merge-ready foundation draft, written in Part 1's register to be folded into ../../drystone-spec/part-1-reasoning-underpinnings.md. This has grown from a short addendum into the foundational statement of the peer-symmetry premise: what a center is, why consolidation of capability and of authority are both permitted and both center-free, why the right to fork is inherent and cannot be configured away, and why local-first is the necessary condition that makes all of it safe. Principle material, not mechanism; the mechanisms it refers to live in Part 2 and the companion docs.`

`Scope: establishes the identity spine of principal, persona, and peer, and sharpens peer-equality into the authority/capability distinction (§1); derives the delegated-helper model (§2), the one-directional capability-consolidation range (§3), and the permitted authority-consolidation spectrum with the party-neutral-defaults corollary (§4); establishes the right to fork as inherent and unconfigurable (§5), local-first as the necessary foundation the whole premise rests on (§6), and faithful representation with the protocol/product division (§7); and records Drystone's positioning as the complement to ATProto (§8).`

`Terms: principal (any permission-holding entity, the widest category, §1); persona (the expression of a self, grounded in a root key pair and its key-pair lineage; the unit of standing and voice, and the kind of principal that has standing, §1); peer (a property of the relationship at the edge between two entities, the symmetry that neither stands above the other, never an actor, §1); node (a device or process running Drystone, which may be more or less capable); authority (standing, the power to decide membership, rules, and foreclosure); capability (what resources let a principal do, namely hold, index, serve, and stay available); center (a principal whose authority is irrevocable and inescapable, §4); helper (a principal granted capability by delegation and never authority, §2); fork (a persona continuing from their own local state, §5); lineage (a chain of state sharing history up to a divergence point); local-first (each participant holding their own authoritative copy, §6).`

`Companion to: ../../drystone-spec/part-1-reasoning-underpinnings.md (merge target), governance-finality.md (which specifies ban-as-forced-fork as the mechanism side of §5's inherent right; the two docs point at each other on that identity), ../../drystone-spec/part-2-certifiable-design.md (§5.7, §7.3 governance; §7.6 fork-as-escalation), asset-keying.md, history-durability.md, social-mapping.md, ../../cairn/atproto-ecosystem.md.`

Merge guidance: §1 sharpens and should absorb the existing peer-equality statement. §2, §3, and §4 are its consequences, to sit alongside it. §5, §6, and §7 are the foundation and should anchor Part 1's premise, since the rest rests on them (§5 fork, §6 local-first, §7 the faithful-representation and protocol/product division). §8 (positioning) belongs in Part 1's scope or relationship section, not as a numbered principle. Tag on the principle claims is `Synthesis`, grounded in the Part 2 governance mechanics and the ecosystem survey.

Principle catalog. The suite's `Realizes:` tags reference these four principles, and each is developed where noted. They are the spine Part 1 assembles around.

- **P-Peer-Equality**: no persona has standing above another persona, and no principal acquires standing merely by holding capability; authority is flat even though capability is not. Developed here, §1.

- **P-Local-Truth**: each participant holds their own authoritative copy of state and there is no central canonical copy, so truth is local. Developed here at §6, and applied in history-durability.md and social-mapping.md.

- **P-Knowable-Truth**: a participant establishes the truth and currency of state by verifying the facts it holds and corroborating with others, never by querying a privileged authority, so completeness behind a head is provable and completeness ahead of it is corroborated, not proven. Developed in fact-and-chain-representation.md (§6) and governance-finality.md (A7, A8, B3).

- **P-Durable-Enablement**: the system durably enables entitled access to content and history through delegated, revocable helpers and recoverable history, so enablement persists without a gatekeeper. Developed in history-durability.md and asset-keying.md.

---

## 1. Identity and peer-equality: principal, persona, and peer

Three terms name the actors and their relation, at three different levels, and keeping them at their levels is what makes the rest of the design statable. They are not three names for one thing.

- **Principal** is the authorization-layer term: any permission-holding entity. It is the widest category. A helper granted read access is a principal; a device may be a principal; a persona is a principal. What unites them is only that a capability can be bound to them.

- **Persona** is the identity-layer term: the expression of a self, grounded in a root key pair and its key-pair lineage. A persona is the unit of standing and voice, the thing that decides, votes, forks, and consents, and it is a specific kind of principal, the one that has standing because it is an expressed self. Two acts are "the same actor" when they trace to the same key lineage.

- **Peer** is the relational-layer term, and it is not an actor at all. Peer is a property of the relationship at the edge between two entities: the symmetry that neither stands above the other. This is worth stating because "peer to peer" is often heard with "peer" as a noun for a participant, when peer is really a function of the edge between participants. In Drystone the peer relation is the flatness of standing between personas; it lives on the edge, not in a node. So "peer-symmetric" and "the peer relation" are correct, and "a peer decides" is a category error, because the decider is a persona.

The containment is exact: **every persona is a principal, but most principals are not personas.** A persona is the principal with standing, one that can govern, vote, consent, and hold authority; a helper-principal has capability but no standing. That is the authority-versus-capability line drawn at the level of who-holds-what.

Persona is a key-lineage construct, not a personhood claim, and this boundary is load-bearing. **Proof of personhood is out of scope for Drystone.** The protocol does not claim to know that one human stands behind a persona; it claims only cryptographic continuity, a root key and the chain of keys descending from it, which is what lets a persona rotate keys, add devices, and recover while remaining the same persona. This is exactly as much identity as a center-free system can honestly assert. A Sybil can run many personas and the protocol does not pretend otherwise; the social layer's thresholds and dials are how a group copes with that, not a personhood guarantee the protocol falsely offers. Persona is therefore the honest referent for adjudication: Drystone adjudicates among expressed selves it can cryptographically distinguish, and deliberately does not assert they are distinct people.

With the actors named, peer-equality can be stated exactly. It is often heard as "all nodes are equal," which is false and which Drystone does not claim: nodes differ enormously in capability, one holding a terabyte and never sleeping, another a phone usually asleep. What Drystone holds flat is not capability but authority, and the exact statement is two claims at their two levels:

- **No persona has standing above another persona.** This is peer-symmetry, the flatness of the relation among selves. Authority is held by personas collectively and moves only by the k-of-n governance fold (Part 2 §5.7, §7.3).

- **No principal acquires standing merely by holding capability.** A helper, a device, or a high-memory archive gains no authority from what it can do, however much that is. This covers every non-persona principal, and it is why no node becomes a center by growing.

Two quantities usually run together must stay apart, and the two claims above turn on the distinction.

- **Capability** is what a principal can do with resources: how much history it can hold, how much it can index, how much it can serve, how available it is. It is unequal by nature and Drystone places no ceiling on it.

- **Authority** is standing: the power to decide who is a member, to change the rules, to foreclose on a persona, to override a decision. It is flat, held by personas, moved only by the fold, and no accumulation of capability converts into any of it.

This gives the first meaning of a *center*: a principal that can exert authority beyond a persona, not one that has capability beyond a persona. A principal with vast resources and no standing is not a center; one with modest resources and the power to foreclose would be. Drystone forbids the second and places no limit on the first, and §4 sharpens this once authority concentration is on the table. It is also why the design is rooted in principle rather than in a privacy property: the claim is not "your data is hidden," which people have learned to discount, but "no persona has standing over you and no principal earns standing by capability," which is about control and holds no matter how capable any node becomes.

## 2. Helpers: capability by delegation, never standing

A consequence of §1 is that Drystone may use arbitrarily powerful helper infrastructure without creating a center, so long as a helper gains its capability by delegation and never gains authority.

A helper is a node that a persona or a group admits to do work: a store-and-forward node so members receive while offline, a high-memory archive so a phone-only member can converge deep history it cannot hold locally, a search or index helper that holds clear text so private content can be queried. A helper may hold clear text. What matters is how it comes to hold it and what standing that confers.

- A helper holds clear text because a persona, or a k-of-n group decision, granted it, by admitting it to the relevant scope exactly as any member is admitted, and it can be removed the same way. Its access is a grant, revocable by the peers who extended it.

- A helper holds no authority by virtue of that access. It cannot foreclose on a persona, cannot remove a member, cannot override governance, and can do nothing a persona could not authorize and revoke. It is additional, not elevated.

This is the categorical difference from the access-control model of the surrounding ecosystem. There, a personal data server or an authorized view holds readable data structurally, as a property of the architecture that cannot be removed without removing the system, and it occupies a privileged tier by construction. In Drystone a helper holds readable data by delegation, as a grant the peers extended and can revoke, and it occupies no tier. Same clear text, opposite standing: one can read because it is the infrastructure, the other can read because it was invited and can be uninvited.

The honest tradeoff, stated plainly: delegating clear text to a helper expands the confidentiality surface to that helper, since its compromise leaks what it was shown, but it never transfers authority, since the helper can neither foreclose nor govern, and it is revocable. So a content-blind durability store (history-durability.md) and a clear-text search helper are two points on one spectrum, capability delegated without authority, differing only in how much the peers chose to reveal. The peers choose per helper where to sit on that spectrum. Neither point is a center, because neither holds standing above a persona.

## 3. Consolidation of capability is one-directional

A further consequence: Drystone may be arranged anywhere from fully peer-distributed to fully consolidated, as a deployment and economics decision, and it is center-free at every point on that range. This range is available to Drystone and not to the authority-centralized designs, and that asymmetry is the point.

Drystone can run maximally distributed, every persona on its own device holding its own history, or maximally consolidated, a single operator hosting the delivery helpers, the archives, the search helpers, the whole apparatus, so that a user cannot tell it apart from an ordinary hosted product. The consolidated arrangement is still center-free, because the operator, however much infrastructure it runs, holds no authority over any persona: it is a collection of delegated-capability helpers, and any persona or group can re-home elsewhere without anyone's permission. This is a deployment choice about capability consolidation, not an authority structure.

The inverse move is impossible. An authority-centralized design cannot be arranged into peer-symmetry, because its authority is structural: the owner or space authority, and the readable, gatekeeping infrastructure tier, are load-bearing, and removing them removes the system. Their center is not a dial that can be turned down; it is in the architecture.

So the durable statement is this. Drystone spans the full range from peer-distributed to consolidated while never acquiring a center, and the authority-centralized designs cannot make the reverse journey while keeping their function. The convenience of consolidation is available to Drystone without the cost of a center, which is a choice not otherwise available.

## 4. Consolidation of authority is also permitted, and also center-free, because it is revocable and exitable

§3 concerns capability. The stronger and more surprising claim concerns authority itself: a group may concentrate authority to a high degree and remain center-free, because the concentrated authority is always revocable and exitable.

Governance concentration is a permitted spectrum, and every point on it is legitimate: flat consensus, delegated roles, concentrated authority, even a group created so that only its creator's signature counts as the quorum. Drystone does not forbid a group from adopting a near-authoritarian internal structure. A group may even be created with a governance model that makes useful disagreement difficult. That is a permitted configuration, not a violation of the design.

What makes every point on this spectrum center-free is not that the points are egalitarian, because they are not. It is that the concentrated authority is held up only by the continued participation of the members, and the escape hatch is invariant. A "creator is king" group is not a center in the Drystone sense, because the creator's authority is not structural: it is granted by everyone staying, and it evaporates the moment they leave or revoke. The creator can make the room hard to argue in; they cannot lock the doors.

This yields the final and sharpest definition of a center. **A center is not a node with a great deal of authority. A center is a node whose authority is irrevocable and inescapable.** Concentrated-but-revocable authority is not a center; modest-but-locked-in authority would be. The whole design is arranged so that the second is impossible and the first is a permitted social choice. What distinguishes a Drystone group with a near-dictator from an actual authoritarian platform is not the concentration of power, which may look identical day to day, but that the Drystone version sits on a substrate where the power is always revocable by threshold and always escapable by fork.

The escape hatch is a strict hierarchy of last resorts, and it MUST be readable as such:

- First, work within the rules: persuade, re-vote, use the governance the group has.

- Else, if the rules have been captured, revoke the capturing authority, which is possible whenever the revocation threshold can be met. High threshold to revoke is still revocable.

- Else, if even revocation is blocked because the threshold is set too high to reach, the floor is fork and exit, which needs no threshold and no one's permission (§5).

So the two asymmetries sit together, and rest on one foundation. Capability may consolidate freely because capability is not authority (§3). Authority may consolidate freely because authority is revocable and exitable (§4). Both are safe for the same underlying reason: the peers hold their own edges (§6), so neither capability nor authority can ever become a trap. The permissiveness at the top of the spectrum, that a group may concentrate authority almost arbitrarily, is underwritten by the floor at the bottom, that any concentration can always be escaped by a primitive the concentration cannot reach. `Synthesis.`

A corollary governs which mechanisms may be defaults and which must be chosen. **A mechanism may be a default only if it is party-neutral; any mechanism that can privilege a party must be opt-in and itself governed.** A party-neutral mechanism (one that privileges no participant, such as an ordering over kinds of operation, or a canonical tiebreak that is a deterministic coin flip) may ship as a default, because it confers no advantage on anyone. A party-privileging mechanism (one that advantages specific participants or their actions, such as assignable weights that let some facts outrank others) must be opt-in and placed under the same k-of-n governance as any other authority, because a settable advantage is itself a lever of authority, and a self-assertable one would be a backdoor to power that bypasses the vote. This is the concentration principle applied at the smallest scale: even a tiebreak that privileges a party is a concentration, so it is permitted but must be revocable and governed, never a silent default. The mechanism side of this appears in governance-finality.md A13 (the configurable tiebreak keys). `Synthesis.`

## 5. The right to fork is inherent, not granted, and cannot be configured away

A fork is not an operation the group performs or a permission the group extends. **A fork is a persona continuing from their own local state.** A persona holds their keys, their copy of the history, their identity, and their edges; to fork is simply to keep going from what they hold, accepting writes into their own lineage, without requiring the rest of the group's ongoing corroboration.

This is why the right to fork cannot be configured away, and the reason is mechanical, not a matter of enforced policy. Governance operates on the corroborated, shared state: who may write, what reaches quorum, whose authority counts. But a persona's local state is theirs by possession, not by grant, and forking is the persona declining to subordinate that local state to the group's corroboration any further. There is no governance rule that can remove a persona's ability to keep their own copy and continue from it, because that ability was never something the group conferred. So "you cannot configure away the right to fork" is not a promise the system makes against would-be tyrants; it is a structural fact. There is no lever to pull, because a fork lives entirely on the forker's side of the line, in state the group cannot reach.

- **The minimum fork is one.** Forking has zero dependency on anyone else's cooperation, which is exactly what makes it the un-blockable floor. A single persona, alone, disagreeing with everyone, can fork, and their lineage is as legitimate as any other; it is simply small. No configuration and no coalition can prevent it, because preventing it would require reaching into a single peer's own possession, which local-first makes impossible.

- **The mechanical right is distinct from the social outcome.** The system guarantees you can fork, down to a group of one. It guarantees nothing about whether anyone joins you. Whether others follow is the aggregate of many individual social decisions, each persona choosing which lineage to corroborate going forward. The protocol furnishes the ability to leave with everything that is yours and stays deliberately silent on whether you leave alone or with company, because who-follows-whom is the peers' call, not the substrate's. This is a social-utility value statement, not a system guarantee, and keeping the two apart is the point.

A ban is the same primitive seen from the other side: a forced fork that is equal in outcome to a voluntary one, differing only in its artifacts. The mechanism side of this is specified in governance-finality.md (ban and voluntary fork are one primitive, lineage divergence at a head, equal in outcome, distinct in artifacts). The consequence worth stating at the principle level is that the group's harshest power over a member is bounded by this floor: a ban cannot erase a member, it can only end the group's corroboration of them going forward, and the banned member continues whole in their own lineage holding everything they had. The most a concentrated authority can do to a member is force a fork, and a forced fork leaves the member intact. `Synthesis.`

## 6. Local-first is the necessary foundation, not merely a preference

Everything above rests on one condition, and it is worth stating as the deepest premise of the project rather than leaving it implicit. Local-first is not an efficiency choice and not a privacy choice. It is the enabling condition for peers to hold real, portable edges, and real edges are what make the rest possible.

The argument is a chain, and each link is necessary:

- Local-first gives each persona possession of their own side of every relationship: their identity, their keys, their copy of the history, their graph of who-knows-whom, independent of any shared server.

- Real, portable edges make exit lossless. A persona who leaves takes their keys, their copies, their identity, and their relationships, and re-forms elsewhere losing nothing but the disagreement.

- Lossless exit makes fork a genuine resolution rather than a punishment. When personae genuinely and irreconcilably disagree about the rules of their shared space, the fork costs the departing side nothing it held, so it is a real option and not a threat.

- Fork-as-resolution is the only center-free way irreconcilable disagreement can settle. There is no cosmic referee to decide which faction was right; there is the fact that people can walk and take their relationships with them and re-form. The disagreement is not adjudicated, it is resolved by reconfiguration.

- Therefore any governance concentration is safe to permit (§4), because it is always revocable and exitable, and the exit is real precisely because local-first made the edges portable.

This is why a centralized architecture cannot represent genuine social conflict resolution, and the impossibility is structural, not a matter of will. On a central server, exit is not real: leaving means losing your history, your identity in that space, and your connections, because the server owned them. So a centralized system can only offer adjudication-by-the-host, which is exactly what personae in genuine peer standing do not have and do not want. The host becomes a referee by default, because it holds what you would need to take with you. Real social conflict, historically and socially, resolves by reconfiguration, congregations split, co-ops fracture, communities fork, and only local-first can reflect that, because only local-first gives personae the real edges that make reconfiguration lossless.

So the chain closes on itself: local-first gives personae real edges, real edges make exit lossless, lossless exit makes fork a genuine resolution, fork-as-resolution is the only center-free settlement of disagreement, and that is what makes it safe to permit any governance a group chooses. The permissiveness at the top is underwritten by the necessity at the bottom. `Synthesis.`

## 7. Faithful representation, and the protocol/product division

Everything above resolves into a single statement of what Drystone is for, and it draws the line between what the protocol does and what a product built on it does.

**The protocol faithfully represents individual, local, personal choice, and never centrally resolves social conflict, because social conflict has no central resolution, only individual responses that aggregate.** When people genuinely disagree about their shared space, there is no authority that can decide correctly on everyone's behalf, because the right outcome differs per person. The faithful thing, and the only center-free thing, is to let each persona act as themselves (stay, leave, mute, join both) and to let the group-level outcome be the aggregate of those choices, a realization of the social adjudication rather than an input to it. A literal divorce with a shared friend group is the anchoring example: when two groups want to merge but one has removed a member of the other, there is no technical fix and no correct group-level answer, only each mutual friend deciding, for themselves, where they want to be. A system that forced a single group-level answer would be imposing a fiction, exactly as a central host resolving a divorce's fallout would.

This divides the labor cleanly, and the division is the frame for the whole social layer:

> Making it possible faithfully is Drystone. Making it representable and as easy as possible is the product layer.

- **The protocol (Drystone) is composable, unopinionated, and safe in every configuration.** Its job is *faithful possibility*: the guarantee that every local, personal choice can be represented truthfully and that every configuration is safe. It provides mechanisms and imposes no social shape. That it gets complicated under composition is correct at the protocol layer, because it must support all social shapes safely rather than choose one.

- **The product is opinionated, legible, and defaulted by temperament.** Its job is *easy representability*: choosing which of the protocol's possibilities to surface, and defaulting them to a coherent, temperament-matched posture, so most people never touch a dial and the default is a sensible posture. The product makes the choices usable; the protocol makes them possible and honest.

The two are complementary, not in tension: the protocol's refusal to decide is what makes the product's opinions safe to hold, because no product default can trap anyone when the underlying protocol always permits mute, revoke, fork, and exit. Drystone's contribution is faithful possibility; opinion, ease, and legibility are the product's. The mechanism side of this layer is specified in governance-finality.md A11 through A17. `Synthesis.`

## 8. Positioning: the complement to ATProto, not a competitor

(Merge into Part 1's scope or relationship section, not as a numbered principle.)

Drystone is not a replacement for ATProto and is not in competition with it. It is the complement ATProto has, by deliberate choice, left open.

ATProto is built for public, broadcast, reach-oriented speech: content that wants to be seen, indexed, aggregated, and amplified, addressed by public identity and servable by anyone. It is very good at that, and it should remain that. Its own design takes public broadcast as its core, and its private-data direction is explicitly access control rather than confidentiality, leaving group-private end-to-end encryption to be layered on by others (../../cairn/atproto-ecosystem.md).

Drystone is built for the other half: private, member-scoped, governance-forward spaces, where what is discussed and how it is governed is the point, and where the governance is a "for now" arrangement the members hold and can change. A complete social world wants both halves. Drystone hooks into ATProto for the public half, reusing its identity, its public content lexicon, and its public presentation, and supplies the confidential, peer-governed half that ATProto has chosen not to build into its core. The alignments between the two are therefore complements, not overlaps. Drystone fills the space ATProto left open, and the fact that the ecosystem punted end-to-end encryption to third parties and chose access control for permissioned data is that space being left open on purpose.

---

## Open items

`Distinct, per the doc-method.`

- **O1. persona versus peer (resolved this cycle).** Settled by the identity spine in §1: persona is the singular actor (an expressed self, a root key and its lineage), principal is any permission-holder, and peer is the edge relation, never an actor. The singular-actor "peer" uses in §5 and §6 have been corrected to "persona"; relational uses (peer-symmetric, the peer relation) and networking uses (peer reconciliation across a sync edge) are kept, as they are correct under this definition. `Resolved.`

- **O2. Principle-catalog placement on merge.** The catalog above lives here because this is the Part 1 foundation piece, but two of its principles (P-Knowable-Truth, P-Durable-Enablement) are developed in Part 2 mechanism docs. On assembly, decide whether the catalog sits in Part 1 with forward references or is split across the two parts. `Design, open.`

- **O3. Party-privileging defaults.** §4's corollary permits governed, opt-in party-privileging mechanisms but sets no default; the concrete defaults are tracked in governance-finality.md B9. `[confirm.]`

## Changelog

`Working draft; per the suite's doc-method, transitions are recorded here.`

- **Draft, for merge into Part 1.** Records the authority-versus-capability distinction and the exact statement of peer-equality (no node has standing above a persona), §1; the delegated-helper model with its honest confidentiality-surface tradeoff, §2; the one-directional capability-consolidation asymmetry, §3; and the ATProto-complement positioning, §7.

- **Expanded into a foundation piece.** Added the permitted authority-consolidation spectrum with the final definition of center as irrevocable-and-inescapable authority and the three-tier escape hatch (§4); the right to fork as inherent, unconfigurable, minimum-one, and mechanically-distinct-from-social-outcome (§5); and local-first as the necessary enabling condition, with the chain from portable edges to lossless exit to fork-as-resolution and the argument that centralized architectures structurally cannot represent it (§6). The two asymmetries (capability and authority both consolidate freely, for the one reason that peers hold their own edges) are now linked, and the whole premise is grounded in the mechanical fact of §5 rather than asserted.

- **Cross-linkage.** §5 states the inherent right to fork; its mechanism side, ban-as-forced-fork (equal in outcome, distinct in artifacts), is specified in governance-finality.md, and the two docs reference each other on that identity. The companion docs' residuals about private-content server-side features read as the §2 tradeoff, and the ecosystem doc leads with the §8 complement framing.

- **Party-neutral corollary and the protocol/product division.** Added to §4 the corollary that a mechanism may default only if it is party-neutral, while any party-privileging mechanism (down to a weighted tiebreak) must be opt-in and governed, the concentration principle at the smallest scale; its mechanism side is governance-finality.md A13. Added §7, faithful representation and the protocol/product division: the protocol faithfully represents individual local choice and never centrally resolves social conflict (the divorce anchor), framed as "making it possible faithfully is Drystone, making it representable and easy is the product," with the protocol composable-and-safe-in-all-configurations and the product opinionated-and-defaulted-by-temperament. The mechanism side is governance-finality.md A11 through A17. Positioning renumbered to §8; merge guidance and scope updated.

- **Doc-model standard pass.** Added a Terms block, the suite Principle catalog (P-Peer-Equality, P-Local-Truth, P-Knowable-Truth, P-Durable-Enablement) so the `Realizes:` tags across the data-plane docs resolve, and an Open items section (persona/peer reconciliation, catalog placement on merge, party-privileging defaults). No principle claims changed.

- **Identity spine (principal, persona, peer), closing O1.** §1 expanded from a peer-equality statement into the identity foundation: principal (any permission-holder, the widest category), persona (an expressed self grounded in a root key pair and its key-pair lineage, the unit of standing and the kind of principal that has standing), and peer (the symmetry at the edge between entities, never an actor). Adds the load-bearing boundary that proof of personhood is out of scope, so persona is cryptographic continuity rather than a personhood claim and a Sybil may run many personas. The peer-equality invariant is restated as two claims at their levels, no persona has standing above another persona and no principal acquires standing merely by holding capability, replacing the level-mixing "no node has standing above a persona." Terms, Scope, and the P-Peer-Equality catalog entry updated; singular-actor "peer" corrected to "persona" in §5 and §6; relational and networking uses of "peer" kept. O1 closed.
