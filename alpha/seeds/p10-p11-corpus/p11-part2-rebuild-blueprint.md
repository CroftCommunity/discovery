# Drystone Part 2 rebuild: blueprint

`Status: working blueprint for a clean rebuild of Part 2 (mechanics) into a new document, p11-full-part2-mechanics.md, built section by section to replace p10-full-part2-mechanics.md once each piece is reviewed. This is the coherence authority the rebuild derives from; it is method, not mechanism.`

## Why rebuild rather than keep patching

The current Part 2 grew by accretion, and many recent additions were surgical folds placed where they fit rather than where they belong. Three costs followed, and the rebuild exists to remove them.

- **Duplication.** Load-bearing ideas are restated in several sections instead of stated once and referenced. The scan found the completeness-ahead beam touched across §7.3.3, §7.3.7, §7.3.8, §7.4, §7.9, §5.11, and Appendix B; the two-phase across §7.3.6, §7.9.1, §5.11, and §7.6.7; the asset key across §5.11, §6.8.5, and §7.7; R5 and the retained-copy floor across §7.2, §5.8, §5.11, §7.3.6, and §6.8.4.

- **Actor drift.** Running examples were threaded in piece by piece, so the cast and its beats risk inconsistency across sections, which is exactly what the closed-cast discipline exists to prevent.

- **Ordering by insertion, not dependency.** Sections and subsections sit where edits landed. The numbering even skips from §0 to §3, and §4.6, §5.11, §6.8.5, and §7.9's late additions were placed by convenience.

A rebuild lets the whole document be authored as one coherent object, which is the doc-method's "author the library first, derive downward" discipline applied to the library itself.

## The method

- **New document.** Build `p11-full-part2-mechanics.md` from scratch, section by section. The old `p10` stays intact and authoritative until a section's replacement is reviewed and accepted, so there is never a half-valid spec.

- **Piece by piece, reviewed between.** One section (or one coherent cluster) per pass. State the plan, write it, verify discipline, check it against the acceptance checklist below, present, pause. Chase reviews before the next piece.

- **Correctness is carried, not re-derived.** Every claim already grounded (the RFC 9420/9750 checks, the CALM and BLAKE3 and dag-cbor facts, the concept-by-concept companion verifications) is preserved with its status tag. The rebuild changes ordering, cohesion, and clarity, not settled results. Where a claim is only in a companion doc, it is folded once, in its canonical home.

- **Replace on acceptance.** When all sections are accepted, `p11` becomes Part 2 and `p10` is retired. The companion docs remain the derivation record; the two published specs plus the conventions doc remain the only documents a reader needs.

## The six acceptance properties, as a per-section checklist

Every section must pass all six before it is accepted. Stated as checks so a section either passes or does not.

- **Ordering.** The section depends only on sections already written above it (or on Part 1 and the conventions doc). Its `Depends on` and `Orthogonal to` lines in the §0 map are accurate. No forward reference is load-bearing.

- **Cohesion.** Each concept is introduced exactly once, in the canonical home named in the table below. Everywhere else the section references that home rather than restating it. A restatement longer than a clause is a defect. When a section defers content to a section not yet built, the content is recorded in the carry-forward ledger, so the deferral cannot silently drop it.

- **Actor narrative.** Any running example threads exactly one beat from the E1 to E10 spine, uses only the closed cast, and is consistent with how that beat is told everywhere else. No section introduces an actor not in §3.0.

- **Consistency.** Terms match the conventions-and-decisions lattice exactly (persona, principal, client, Group Role, capability, resource, weight). Section cross-references resolve to the right home. Status tags use the canonical set.

- **Correctness.** Every normative clause carries MUST, MUST NOT, SHOULD, or MAY with its reasoning ("MUST X because Y"), and every claim carries a status tag from the legend. Grounded facts keep their `Verified`, `Verified-RFC`, `Established`, or `Measured` tag; unearned properties keep `Load-bearing, unearned`; deferred encodings keep `[gates-release]`.

- **Clarity.** End-state prose, minimal formatting, no em-dashes, a blank line between every bullet, no consecutive blank lines. One idea per paragraph, the reasoning visible, no unexplained jargon.

## Target section order (clean, dependency-honoring)

The order is close to the current one, which is already mostly dependency-clean, with the numbering repaired and the late insertions given proper homes. Proposed top level:

- **§0. Map.** The annotated section list with `Depends on` and `Orthogonal to` per section, rebuilt last so it describes the final structure.

- **§1. Introduction and scope.** What Part 2 specifies, its relationship to Part 1 and the conventions doc, and the epistemic legend (the status tags). This repairs the §0-to-§3 jump by giving §1 and §2 real content.

- **§2. The peer diagnostic.** What makes this a system of peers, the diagnostic that orders the rest (currently §3.1). Placed early because it is the lens the whole spec is read through.

- **§3. Protocol overview and the running example.** The cast and the beat spine (currently §3.0), stated once here as the narrative authority the rest demonstrates on, followed by §3.2 the helper ecosystem, the fixed node-role cast the mechanisms reference by name.

- **§4. Data model.** Cryptographic foundation, identifiers and derivations, the signed message, integrity-versus-authorship, the multi-client fold, and canonical-on-the-wire.

- **§5. Identity, rights, roles, capabilities.** The two equalities and inequalities, the identity model, rights, resources, Group Roles and capabilities, weight, membership and revocation, exitability, the Group as a principal, and read-scoped content keys.

- **§6. Transport and delivery.** The two identity planes, the encryption stack, the fabric-versus-Group split, the metadata floor, the three transport planes (carriage, durability, presence), gap-aware history convergence and the history store, discovery, the gossip overlay, deployment modes, and media.

- **§7. Synchronization and governance-conflict resolution.** The grant-and-revocation interface, governance-facts-as-entries and the fold (the total order, conflicts, the snapshot cache and the read/enforce line, sign-the-state, the ceiling, decision-versus-enactment, the now, the finality gate, recovery), freshness, attributable acceptance, the reconcile hard-stop and re-formation fork, dataplane modes, side histories, and scaling.

- **§8. Security considerations.** Forward secrecy and durable history, and the honesty boundaries.

- **§9. Interoperability and conformance.**

- **§10. Substrate requirements and reference realizations.**

- **Appendices A to E.** Alternatives, open questions, prior art, the term lattice, and the chained running example (the E1 to E10 walkthrough).

Section numbering and any reordering inside §5, §6, and §7 are open for Chase's call before the build reaches them.

## The de-duplication map: one canonical home per load-bearing idea

The rule is one statement, many references. Each idea below is stated in full in exactly one home; every other section that needs it references the home in a clause and moves on. This table is the contract the rebuild holds each section to.

| Load-bearing idea | Canonical home | Referenced (never restated) by |
|---|---|---|
| The completeness-ahead beam (corroborated, not proven) | §7 finality gate, full statement in Appendix B | §7 snapshot, now, freshness, scaling; §5 read keys; §6 convergence |
| Read/enforce line: best-known versus final | §7 snapshot-cache section | §7 finality gate, freshness; §5 read keys; §6 |
| Fail-closed on finality, delay over breach | §7 finality gate | §7 read/enforce, freshness; §5 read keys |
| Sign the state, not the authorship | §7 sign-the-state | §7 ceiling, now, freshness |
| The membership ceiling | §7 ceiling | §7 finality gate, decision-versus-enactment; §7 fork |
| Decision versus enactment, the two-phase interval | §7 decision-versus-enactment | §7 scaling, hold-and-audience-split; §5 read keys |
| Non-exclusive recognition | §7 decision-versus-enactment | §7 enactment dial |
| The read-scoped asset key and fold-gated provisioning | §5 read keys | §6 history store; §7 dataplane modes |
| R5 and the retained-copy floor (revocation protects the future) | §7 grant-and-revocation interface, and §5 revocation | §5 read keys; §7 decision-versus-enactment; §6 chosen-ephemeral |
| Content-address tiebreak among concurrents | §7 total order | §7 ceiling, now; §5 read keys |
| Epoch and governance decoupling (now advances at zero epoch cost) | §7 the now | §7 scaling; §5 read keys; §6 metadata |
| The fork as lineage divergence (one primitive, distinct artifacts) | §7 the reconcile hard-stop and fork | §5 exitability; §7 ceiling |
| The three transport planes (carriage, durability, presence) | §6 transport | §6 history store; §7 scaling |
| The governance-generation stamp (self-locating dataplane entries) | §7 freshness | §6 history store; §5 read keys |
| The content-blind history store (mirror group, nested sealing, envelope) | §6 history convergence | §7 scaling; §5 read keys |
| The helper node roles (relay, meer, durable history store, swarm node, push notifier, read/search helper) | §3.2 the helper ecosystem | §6 planes; §5 read keys; every section naming a helper |

Where a home is listed as a section rather than a subsection number, the exact subsection is fixed when that section is built (subsection numbering is being repaired).

## The cast and beat spine (the actor-narrative authority)

The cast is closed: five personae (Alice, Bob, Carol, Dave, Erin) and two helper nodes (Carol's node and the history store), and no others. §3 states this as a hard invariant.

- **Alice.** Founder. One persona, two clients (phone and laptop) under one key lineage, so one member with one unit of weight.

- **Bob.** Joins with a single phone at the equal rights floor. The subject of the concurrent removal and the ban.

- **Carol.** A persona whose highly-available node is admitted by delegation as a read-and-search helper, holding a read capability into a read scope so it holds clear text within that scope; capability without standing, its availability a resource.

- **Dave.** Joins with a phone and a tablet. Lands on one side of the fork.

- **Erin.** Joins with a phone. Lands on the opposite side of the fork from Dave, is banned by her lineage during the years apart, and is excluded at the re-composition. Her thread carries the equal-branch-in-individual-outcome cases.

- **The history store.** A helper node admitted for durability alone, holding the Group's history only as sealed blobs and reconciling over a content-free envelope, so it holds no clear text; capability without standing, one confidentiality level below Carol's node, its availability a resource.

The node roles are a second closed cast, fixed in §3.2 after the personae: relay and swarm node on carriage, meer and durable history store on durability, push notifier on presence, and the read/search helper on a read scope. Every section names a role by its §3.2 definition rather than re-describing it, Carol's node and the history store above are the running example's instances of two of these roles, and no section introduces a node role §3.2 did not name.

The beats, each demonstrated by specific sections, and each section's running example threading exactly one beat:

- **E1. Formation.** Alice creates the Group; multi-device-to-one-persona; Bob joins at the floor.

- **E2. The first message.** Signed, hash-chained, two encryption layers; Bob verifies signature and standing; a relay sees only ciphertext.

- **E3. Admitting the helpers.** Two helpers admitted at different confidentiality levels: Carol's node by an attenuating read-capability delegation (clear text within its scope), and a content-blind history store for durability (no clear text).

- **E4. Adding Dave and Erin, setting the threshold.** k-of-n for membership and revocation.

- **E5. The concurrent removal.** Alice and Bob move to remove each other at equal standing, no causal answer.

- **E6. Resolution: tiebreak, or hard-stop.** The two paths, by what the contradiction is.

- **E7. The ban as a forced fork.** The Group bans Bob; a ban is a forced fork, not an erasure; Bob continues whole.

- **E8. The fork landing.** Dave and Erin land on opposite sides; the epoch roll is the audience split; equal branches in individual outcome.

- **E9. The return and catch-up.** Dave's device comes back and converges missing history, content-blind store included.

- **E10. Re-composition.** The lineages re-compose as a governed view; Bob is re-admitted by explicit act, Erin excluded; equal branches again.

The beat-to-section binding is fixed as each section is built, and the full chained walkthrough lives in Appendix E, with each in-body beat pointing back to it.

## Doc-method application: how the rebuild honors each practice

The rebuild is written to the full method (p10-drystone-doc-method.md). Two practices are already operationalized in the sections above, the closed cast (Practice 4) as the cast-and-beat spine and one-mechanism-named-once (Practice 7) as the de-duplication map; the rest are stated here as build obligations, so each section can be checked against all sixteen.

- **1. End state, not a record of thinking.** Every section is authored as the current conclusion, with no path-narration and no temporal word ("no longer," "now," "still," "used to") doing contrastive work against an unstated prior. Authoring fresh is the advantage here: it removes the accreted contrast baggage that patching leaves behind, and the only sanctioned home for transitions is the p11 changelog. The fold-boundary form of this check (Practice 13) is applied to every sentence carried over from p10 or a companion.

- **2. Self-define load-bearing terms, up front, in one place.** Each of §1, §4, and §5 opens by defining the terms it coins and gives this document's working definition for every inherited term, with the conventions-and-decisions doc named as the source, so a term can be diffed against its source rather than deferred to it. Building front-to-back with terms ordered ahead of their use dissolves the used-before-defined class of bug. Appendix D is the vocabulary of record.

- **3. Requirement first, then the current realization.** Every section resting on a substrate states the Drystone requirement in the design's own noun (a Group holds continuously-rotated key material shared among its members), then names the current conforming implementation version-pinned (MLS realizes this via the group's ratchet tree), and states the non-requirements where a naive reader would over-build. §10 is the whole-document instance, and the per-sentence vocabulary test, the Drystone term for a requirement and the substrate's term for a realization, is a clarity check on every section.

- **4. Closed cast, additive spine, chained in an appendix.** Operationalized above as the cast and beat spine: the five personae, the E1 to E10 beats, §3 introducing the cast once, each mechanism growing the same scenario, and Appendix E chaining the beats with each in-body beat pointing back. The same discipline binds a second closed cast, the node roles fixed in §3.2 (relay, meer, durable history store, swarm node, push notifier, read/search helper), each named once and referenced by name thereafter. Building §3 and Appendix E first makes the spine the fixed authority every later example is checked against.

- **5. Why, not just what.** Every mechanism states the failure it prevents or the value it secures, not only the mechanism, in explanatory prose as well as in normative clauses. The correctness property enforces this at the normative layer (Practice 14); this practice extends the same burden to the prose around it.

- **6. Mark epistemic status, ground before stating.** Every claim carries a tag from the legend §1 states, and no claim is asserted before it is grounded. Grounded facts keep their `Verified`, `Verified-RFC`, `Established`, or `Measured` tag, the one unearned property keeps `Load-bearing, unearned`, and deferred encodings keep `[gates-release]`. These tags are carried intact from the grounded verification work, not re-litigated during the rebuild.

- **7. One mechanism, named once, pointed at from its uses.** Operationalized above as the de-duplication map: one canonical home per load-bearing idea, referenced everywhere else in a clause. This is the practice the rebuild most directly discharges, since restatement across several sections was the central accretion cost.

- **8. Separate the layers explicitly.** Where one word conflates two technical layers, the rebuild splits them and names the layer each sits at: relay versus gateway among the §6 observers, the thing versus the use of the thing (a store node's fabric-level presence versus a persona's individually-revocable use of it), and a resource versus a granted authority (blind availability versus a read grant). Every "or" that joins two things living at different layers is a flag to split.

- **9. Own the residual honestly.** Non-goals are stated where a reader would otherwise assume coverage (no enforced deletion, no defeat of transport-level traffic analysis), §8.2 carries the honesty boundaries, and Appendix B collects the genuinely undecided, kept distinct from the decided-and-bounded. The beam's `Load-bearing, unearned` status is this practice at the normative layer, and the open-threads tracker is its derivation-side companion.

- **10. Close a requirement-mapping section with a posture summary table.** Part 2 maps onto external dependencies it must satisfy or work around (MLS, Willow, iroh), so it carries posture tables: the §10.5 dependency-versus-realization ledger and the MLS hard-cases summary, one row per case in body order, decided-and-bounded rows only, with undecided cases held in Appendix B. Each table is both the index over its mapping and its consistency check.

- **11. Mechanical hygiene.** No em-dashes, a blank line between bullets and labeled lines, no run-on sentences, cross-references that resolve and that disambiguate a companion's "Part 1 §N" from a local §N, and a review-against-the-method pass per section. This is the clarity property, verified on every piece before acceptance.

- **12. Name the sociotechnical alignment where a concept spans both planes.** Drystone's central claim is a convergence, that technical results taken seriously force a humane social shape, so where a word spans both planes (group and Group, role and Group Role, right, weight, the fork) the section states the social principle as layer-independent, then names the technical construct as that principle made mechanical with fidelity, and says where the mirror is imperfect (the social "one person" has no faithful technical mirror, which is why persona and personhood stay separate words). Case and qualifier typography carries which plane each sentence stands on. This is load-bearing in §2, §5, and §7.

- **13. Work from the conventions reference and a per-part changelog.** Every section's pass runs against the conventions-and-decisions doc rather than from memory, and p11 carries its own changelog. The fold-boundary guard is explicit: because carried prose was written to a different, auditable-history standard, the "does this sentence only make sense to someone who saw the prior version?" check is applied to the incoming material, which is where end-state violations most reliably enter.

- **14. Normative clauses carry their grounding, both MUST and MUST NOT.** Every MUST names what it secures and the Part 1 imperative it discharges, usually the section's `Realizes`; every MUST NOT names the concrete failure its breach causes. A clause whose justification does not trace to a principle is treated as a signal that either the clause or the principle is missing. This is the correctness property stated as a rule.

- **15. Carry an annotated section map at the top.** §0 is the annotated map, one line per section and major subsection with scope, a `depends on` note, and an `orthogonal to` note, and it is built last so it describes the finished structure. Thereafter it is maintained in place: any edit that adds, moves, removes, or repurposes a section updates the affected map entry in the same pass, since a stale map misdirects the next reader worse than an absent one would.

- **16. Write at three resolutions, each leading to the next.** The rebuilt Part 2 is the library, the coherence authority the other resolutions compress from, which is why it is authored first and in full. Once it is accepted, the elevator pitch, in both plain-spoken and technical registers, and the coffee-shop telling are re-derived downward from it and re-checked against it, and any divergence is a defect in the lower resolution, never a license to let Part 2 drift to match. The existing suite pitches and coffee-shop pieces are re-derived from the rebuilt spec.

## Carry-forward ledger: content deferred to a later section, not dropped

When a rebuilt section moves content to a section not yet built, the content is recorded here in enough detail to re-place it, so a deferral never loses it. This is the safeguard against the failure where "leave it for the later section" quietly drops material. An entry is cleared only when its target section is built and the content has landed.

- **Gossip-topic seeding cases → §6.3 (scope mechanics). LANDED and cleared.** The three cases now live in §6.3 as "How the topic seed and the helper set fix a Group's scope, in three cases": seed from one Group's ID so scope tracks the Group one-to-one; seed from a value shared across several Groups so scope is wider than any one Group (showing scope and Group are different objects); and include or exclude a meer or relay so the same Group with the same topic has different scope (the proof the topic alone does not fix scope). §4.2 retains only the requirement-versus-realization point. No open carry-forward entries remain.

## Build sequence

Front to back, because that is the dependency order and it lets each section reference only what is already written.

1. This blueprint, reviewed and adjusted.

2. §3 (the cast and beat spine) and Appendix E (the chained walkthrough) together, since they are the narrative authority every later running example refers to. Building them first means every section's example is checked against a fixed spine.

3. §1 and §2 (introduction, scope, legend, the peer diagnostic), short and foundational.

4. §4 (data model), then §5 (identity and rights), then §6 (transport), then §7 (synchronization and governance), each honoring the canonical-home table.

5. §8, §9, §10.

6. §0 (the map) last, so it describes the finished structure, and a final consistency pass across all cross-references, tags, and the discipline checks. That final pass also completes Appendix F (references): every inline citation across the finished document resolves to an entry there, the appendix lettering is confirmed against the assembled appendix set, and the SHA-256 versus BLAKE3 reconciliation (Appendix B) is reflected in the cryptographic-primitives entries.

## Open calls for Chase before the build proceeds

- Section numbering and the §5, §6, §7 internal ordering: keep as proposed, or adjust.

- The canonical-home assignments in the table: confirm, or move any idea to a different home.

- The build sequence: front-to-back as above, or start somewhere specific.

- Whether Appendix E and §3 are built first as the narrative spine, which is the recommendation.
