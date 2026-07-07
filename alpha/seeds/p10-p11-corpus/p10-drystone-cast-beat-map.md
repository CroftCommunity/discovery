# Drystone: The Running Cast and the Per-Section Beat Map

`Status: working (Pass 3 planning). Purpose: the canonical cast, established once, and the map of which cast member demonstrates which mechanic in which section. The cast is an ADDITIVE demonstration layer over the spec, it never replaces a definition or a normative clause, it makes them concrete. Codified as doc-method Rule 16.`

---

## 1. The running cast (the canonical setup)

The cast and its resources are fixed. A per-section beat draws only on what is established here; it does not invent a new actor or resource, so the scattered beats chain into one coherent journey (Appendix E).

### Personae and their clients

- **Alice**, a persona. Two clients: **phone** and **laptop**, both under one key lineage. She holds a **moderator Group Role Set** in the Group. She is the worked case for multi-client-is-one-persona, for a Group Role Set over the flat floor, and for concentrated-but-revocable authority.

- **Bob**, a persona. One client: a **phone**. He is the worked case for the bare-node floor, and he is the member who is eventually banned and continues in his own lineage (ban is a forced fork).

- **Dave**, a persona. Two clients: **phone** and **tablet**. His **phone is offline** for a stretch. He is the worked case for stale-but-honest convergence, for mobile wake, and for a fork bystander/voter who lands where his own choice puts him.

### The helper

- **Carol's node**, an always-on node admitted **by delegation** as a durability-and-search helper. It holds clear text (it can serve queries and replay for offline members) but holds **no standing**: it cannot foreclose, remove, or govern, and it is revocable exactly as it was admitted. It is the worked case for capability-is-not-authority.

### Resources

- **The Group**, with its **append-only governance log** (the fact set the fold runs over).

- **The delivery fabric**, the nodes that carry frames (a superset of the Group; Carol's node lives here).

- **A k-of-n threshold** the Group sets for membership and revocation decisions.

- **Alice's moderator Group Role Set**, to make `floor + [roles] + [capabilities] + [resources]` concrete against Bob, who holds the floor alone.

Coverage rationale: three personae (one multi-client with a role, one single-client subject, one with an offline device) plus one helper node and one Group with a log and a threshold are the smallest roster that can speak to every section below without a cast too large to hold in mind.

---

## 2. Beat map, Part 1 (the principles)

*(Held in reserve. With the setup at Part 2 §3.0, the cast is a Part 2 device and Part 1 remains principle-prose. These beats are recorded for the option of extending the cast into Part 1 later; they are not built in the current pass.)*

The Part 1 beats demonstrate the principle and, where the principle cashes out into a wire obligation, the obligation. `[M]` marks a beat that makes a MUST or MUST NOT concrete.

- **§1.2 Where Drystone sits.** Alice's Group presents publicly as an ordinary feed while backing private content differently: the complement-to-ATProto framing, made concrete.

- **§1.3 The running cast (NEW, the setup block).** The roster above is established here, up top, as the vehicle the rest of the spec reuses.

- **§2.0 The razor.** When Alice and Bob later disagree with no fact of the matter, the protocol furnishes provenance (who asserted what, in what causal order) and never the verdict.

- **§2.0.1 Time is an assertion.** Alice's phone and laptop disagree on the wall clock; neither timestamp is corroborable to the other, so order comes from causal-and-cryptographic structure, not time.

- **§2.1 P-Local-Truth.** `[M]` Dave's offline phone holds its own local truth, and no node holds a canonical copy the others defer to (the MUST NOT: a canonical-state node would be a center).

- **§2.2 P-Knowable-Truth.** `[M]` Bob verifies Alice's assertion himself from the bytes, and every superseded decision stays auditable (the MUST: self-describing state, never a silent mutation).

- **§2.3 P-Peer-Equality.** `[M]` Alice with her moderator Set and two clients has exactly Bob's rights and exactly one unit of weight; her two clients count as one persona in any threshold. Makes concrete: equality-by-mechanism, `floor + roles + capabilities + resources`, delegation attenuates, thresholds count personae not clients.

- **§2.4 P-Durable-Enablement.** Bob on a bare phone can participate and can leave with everything that is his.

- **§2.5 The forced terminus.** Alice and Bob concurrently removing each other at equal standing is the residue; Bob's continuation in his own lineage is the fork floor; a ban is the same primitive and leaves Bob whole.

- **§2.6 The voice right requires field-integrity.** What Alice sees first in the Group is peer-governed, legible as a view, and exitable, never authored by an unremovable outside interest.

- **§2.7 Consolidation and the center.** Carol's node is capability without standing; Alice-as-moderator is concentrated-but-revocable authority, not a center; Bob's fork is the escape hatch's floor.

- **§2.8 Faithful representation.** The Alice/Bob split with Dave and Carol each choosing where to land is faithful representation: the group-level outcome is the aggregate of individual choices, not a verdict.

---

## 3. Beat map, Part 2 (the mechanics)

Grouped by section; subsections share a beat unless noted. `[M]` marks a MUST/MUST NOT beat.

- **§3.1 System-of-peers diagnostic.** Run the diagnostic on the cast's Group: no node, not even Carol's always-on helper, is a center.

- **§4.1 Cryptographic foundation.** `[M]` Bob's laptop MUST verify Alice's signature before any other check, because acting on it first would be trusting unauthenticated bytes.

- **§4.2 Identifiers.** The Group's genesis and Alice's lineage identifiers are tagged, domain-separated derivations.

- **§4.3 The signed message.** `[M]` Alice's post is the unit of history; a receiver MUST recompute the pre-image and verify.

- **§4.4 Integrity vs authority.** `[M]` A valid-chain frame from a non-member passes the structural check but fails the standing check: integrity alone MUST NOT be treated as authorization.

- **§4.5 Multi-client fold.** Alice's phone and laptop are one persona; client-count and device-count are not persona-count.

- **§5.0 Two equalities, two inequalities.** Alice (unequal in roles and resources) and Bob (equal in rights and weight) side by side.

- **§5.1 Canonical state is local.** Dave's offline phone again.

- **§5.2 Principal, client, persona.** The whole cast: Alice's two clients, Bob's one, Dave's two, and Carol's node as a principal that is not a persona.

- **§5.3 Rights floor.** `[M]` Bob's floor equals Alice's despite her role; the floor is never delegated and never unequal.

- **§5.4 Resources.** Carol's node has vast resources, Bob's phone modest; resources are descriptive, not delegable, and confer no standing.

- **§5.5 Group Role, capability, delegation.** `[M]` Alice delegates a read capability to Carol's node, attenuating (a subset of what she holds, never a superset); she MAY delegate within her own device Group or to the shared Group.

- **§5.6 Weight.** Alice's two clients carry one unit of weight, anchored to personhood.

- **§5.7 Membership and revocation authority.** Adding Dave, and the k-of-n needed to revoke.

- **§5.8 Revocation protects the future.** Revoking Carol's node stops future access; it does not un-see what the node already saw.

- **§5.9 Exitability.** Bob's exit is the backstop that makes the Group's flexibility real.

- **§5.10 The Group as a principal.** The Group owns a shared artifact; when it forks, neither half is orphaned.

- **§6.1 Two identity planes.** Alice's peer-level endpoint versus the Group's MLS identity.

- **§6.2 Encryption stack.** Alice's message to Bob rides Layer A (transport) inside Layer B (MLS).

- **§6.3 Delivery Fabric vs Group.** Carol's node is in the fabric; the Group is Alice, Bob, Dave.

- **§6.4 Metadata floor.** `[M]` A relay carrying Alice's frame sees endpoints, timing, and size, and MUST NOT be required to read content.

- **§6.5 Carriage.** Alice to Bob direct (C-direct), via relay when no direct path forms (C-relay), or via swarm for fan-out (C-swarm).

- **§6.6 Durability.** Bob offline, Carol's node (D-meer) holds the bytes; Alice's phone and laptop stay in sync as her device-Group.

- **§6.7 Presence.** A minimal push wakes Dave's phone to come look.

- **§6.8 Gap-aware convergence.** `[M]` Dave's phone returns, computes a stale-but-honest view, then closes the gap; it never renders a false current.

- **§6.9 Discovery and tiers.** The Group's interaction tier is chosen at creation.

- **§6.10 Gossip overlay.** Fan-out of Alice's post across the cast.

- **§6.11 Deployment modes.** The cast in relay/meer mode, and the same Group on Dave's tablet over a local network (direct P2P).

- **§6.12 Real-time media.** A call among the cast.

- **§7.2 Grant/revoke interface.** Granting Dave membership through the interface.

- **§7.3 Governance facts are entries.** Adding Dave and setting the threshold are append-only facts, not mutations.

- **§7.3.1 Total order.** The Alice/Bob mutual expulsion, being genuinely concurrent, is decided by the content-address tiebreak (key 3).

- **§7.3.2 What conflicts.** Alice-removes-Bob and Bob-removes-Alice target the same slot.

- **§7.3.3 Snapshot cache.** A truncated snapshot of the Group's current state, verifiable against the log.

- **§7.3.4 Sign the state.** Signing Bob's removal signs the resulting membership state, not the authorship.

- **§7.3.5 Membership ceiling.** `[M]` Bob cannot be re-admitted above the ceiling the log establishes.

- **§7.3.6 Decision vs enactment.** The Group recognizes Bob's removal; enactment is the MLS commit that excludes him.

- **§7.3.7 The now.** The materialized current membership (Alice, Dave) read over the chain.

- **§7.3.8 Finality gate.** `[M]` An irreversible action premised on Bob's removal fails closed until the removal is final.

- **§7.4 Freshness.** Dave's stale view is never presented as current.

- **§7.4.1 False-positive tolerance.** The Group's governed tolerance for treating a concurrent as a genuine dispute.

- **§7.4.2 MLS recovery hazards.** The two hazards, dissolved by corroboration, shown against the cast's epochs.

- **§7.5 Attributable acceptance.** Who accepted Bob's removal, and why the fold does not regress.

- **§7.5.2 Authority-ordering regress.** Broken, so the Alice/Bob contest does not loop on whose authority ranks whom.

- **§7.6 Reconcile hard-stop and fork.** The Alice/Bob dispute hits the hard-stop and becomes a re-formation fork.

- **§7.6.1 Escalation set has two members.** Alice and Bob both, not one, so a mechanism watching only for contradiction would miss the vacancy case.

- **§7.6.2 One primitive, three arities.** The fork-and-heal mechanism the cast's split runs on.

- **§7.6.3 MLS subordinate.** The Alice-and-Dave conversation outlives the key layer.

- **§7.6.4 A ban is a fork.** `[M]` Bob is banned; a ban is a forced fork, distinct artifacts, and Bob continues whole.

- **§7.6.5 Three registers.** Dave mutes Bob (register 1); the Group governance-removes him (register 2); Bob forks (register 3).

- **§7.6.6 Fork placement.** Dave the voter, Carol the bystander, Bob the subject, each lands where his own choice puts him.

- **§7.6.7 Hold and audience split.** The epoch roll is the audience split; a hold suspends enactment.

- **§7.6.8 Both permanent, merge cheap.** Bob's lineage and the Group both persist; a later merge is cheap, not required.

- **§7.6.9 Posture and the protocol/product division.** The Group's posture is a governed dial Alice's product surfaces; the party-neutral tiebreak defaults while a party-privileging weighting must be governed.

- **§7.7 Dataplane history modes.** The Group's forward-only versus Willow-mutable history.

- **§7.8 Side histories.** A side thread in the Group with its own history.

- **§7.9 Scaling.** The Group as it grows, commit serialization, and the completeness beam the fold leans on.

- **§8.1 Forward secrecy vs durable history.** `[M]` Carol's node holds durable history and forward secrecy still holds; the two are not in tension.

- **§8.2 Honesty boundaries.** The residuals the cast's journey still carries.

- **§9 Interoperability.** Two implementations of the cast's Group interoperate on the shared normative sections.

- **§10.2 to §10.5 Substrate.** The cast's Group mapped onto MLS and iroh, then the dependency-versus-realization ledger.

- **Appendices A to D.** Alternatives, open questions, prior art, term lattice; the cast appears only where an example clarifies.

- **Appendix E (NEW): the chained journey.** Every beat above, chained into one continuous arc: cast and resources, formation, admitting Carol's node, adding Dave and setting the threshold, the concurrent Alice/Bob removal and the residue, the tiebreak or the reconcile hard-stop, the ban as a forced fork, the placement of Dave and Carol and Bob, and Dave's phone converging. The scattered beats are excerpts of this arc, and each points here.

---

## 4. Coverage check

Every top-level section and every subsection carrying a demonstrable mechanic has a beat, and every beat draws only on the roster in §1. The load-bearing MUST/MUST NOT beats (`[M]` above) are: verify-first (§4.1), recompute-and-verify (§4.3), integrity-is-not-authorization (§4.4), the rights floor (§5.3), attenuating delegation (§5.5), the metadata floor (§6.4), stale-but-honest (§6.8, §2.1), the membership ceiling (§7.3.5), the finality gate (§7.3.8), ban-is-a-fork (§7.6.4), forward-secrecy-with-durability (§8.1), plus the Part 1 principle-level MUSTs in §2.1, §2.2, §2.3. No section requires an actor or resource beyond the setup.

Placement (decided): the setup block is **Part 2 §3.0** (the running example, at the top of the mechanics where the MUST/MUST NOT beats live), and the chained arc is **Part 2 Appendix E**. The cast is therefore a Part 2 device; Part 1 stays principle-prose, so the Part 1 beats in §2 above are held in reserve, available if the cast is ever extended into Part 1.
