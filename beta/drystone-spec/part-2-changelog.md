# Drystone Part 2: changelog

`Status: attached to drystone-part2.md; populated as the synthesis passes land`

`Purpose: the what-changed-here record for Part 2. The why-and-the-rule for each entry lives in drystone-conventions-and-synthesis-decisions.md (cited as "primer §X"). Together they make Part 2's history auditable while Part 2 itself stays an end state (doc-method Rule 1, Rule 13).`

`The Part 2 synthesis is done in layered passes, each with one job, rather than all at once (this is the layers-of-consistency discipline): (1) terminology alignment; (2) technical-reality fold of the delivery, MLS-hard-case, and history-mode material; (3) a narrative-actor pass making the whole part reflect the Alice/Bob/Carol/Dave walkthrough top to bottom; (4) a requirement/definition/realization consistency pass. Each pass is recorded below as it completes.`

---

## Pass 1: terminology alignment

`Status: complete (one deferred sub-item: adding Delivery Fabric and the C/D/P plane prefixes as first-class Appendix D lattice entries, which lands with the Pass 2 delivery fold since those coinages are formally introduced there)`

Scope: bring Part 2's vocabulary into line with the bounded-context conventions (primer Part A), the same pass Part 1 received. Part 2 predates the delivery and MLS docs and carries the most stale conflations, especially "scope" used as a primary noun where "Group" is meant. Method: apply the attach-rule by best judgment (governance/entitlement facts → Group; exposure/routing-reach facts → scope), escalating only genuinely ambiguous sites for a decision, the process that worked for Part 1.

**§3 (Protocol Overview), done.** Rewrote the anchor definition, which had scope and group backwards: "Peers participate in **Groups**: entitlement-and-governance units holding shared state, each realized over an MLS group" (was "scopes (groups holding shared state)"). Added a Group-scope paragraph establishing scope as the wider exposure envelope where relays/meers/push-nodes sit without being members. This is the first mechanical introduction of both terms (Part 1 introduces them at the principles level). (Primer A.1, A.3.)

**§5 (Identity, Principals, Rights, Roles, Capabilities), done.**

- **scope → Group** across §5.0 to §5.10 for all governance/entitlement facts: membership, history-of-a-Group, replicated policy, versioned Group state, k-of-n thresholds, weight totals, capture, the fork, the recognition/personhood judgments, the trust-posture kinds (family/anonymous/broadcast Group), and the Group-as-principal treatment (§5.10). Left lowercase **scope** only at genuine exposure sites (the meer/relay exposure reach) and lowercase **scoped** (the adjective). (Primer A.1, A.3.)

- **role → Group Role** at concrete in-Group-grant sites throughout (§5.0 Role bullet, §5.5, §5.7, §5.8, §5.9, §5.10 act-for-the-Group). Left genus **role** lowercase where categories are defined ("a principal is a role-holding entity," "the mechanism a role operates through"), with the genus/instance parenthetical added at the §5.0 and §5.2 definitions. (Primer A.2.)

- **PrincipalSet → Group Role Set** (capital S) throughout §5.4, §5.5, the §5.5 heading, and the Appendix D lattice entry, with the definition sharpened to name the two functions (unit grant/revoke; mutual-exclusion for separation of powers) and flagged first-class-but-settling. (Primer B.2.)

- **"By necessity" weight phrasing rewritten** (§5.0, §5.6) to the consequence-of-the-equal-right framing, matching Part 1. (Primer B.2.1.)

- **group-as-principal capitalized** (§5.2, §5.10): "a Group is a principal," "Group-principal," "cross-Group grants," per the settled decision that a grouping-of-principals acting as an in-system actor is capital-G. The §5.10 recursion illustration was aligned to Part 1's treatment (device Group; social "group of people" whose personae or sub-Groups are members). (Primer A.1.)

- **Two revocation planes and ecosystem permissions added at §5.4** (the meer definition), the substantive new text: the meer as a broad-plane principal holding enumerable, blind, revocable ecosystem permissions (be-in-swarm, talk-to-push, talk-to-external); the two revocation planes (Group-governance vs node-local trust) with the asymmetry as the tell; and the grounding that the response shape is a social-utility judgment, not a mechanical toggle. (Primer A.4, A.5.)

- **Correction landed mid-pass:** meer election/adoption is a **per-persona, per-device node-local decision**, not a Group-governance configuration act (an initial draft of the §5.4 edit had it as a Group act, which contradicted the node-local revocation plane and the cross-Group scope-overlap fact). Corrected in §5.2, §5.4, §5.9, and reflected in primer A.3 (cross-Group helper overlap) and A.5 (adoption symmetric with revocation). The Group's *objects* the meer carries remain Group facts; the *choice of helper* is per-persona. (Primer A.3, A.5.)

- **members-hold-keys, not "the scope/Group holds keys"** (§5.9 material reversibility), the same actor-ambiguity fix as Part 1 §2.4, sharpened because a Group can itself be a principal. (Primer B.2.1.)

- **Refinements landed after review of the §5.4/§5.9 helper material** (correcting over-statements in the first draft of this pass):

  - **principal genus is "permission-holding entity," not "role-holding entity"** (§5.0, §5.2, Appendix D.1). "Role" (Group Role) is a governance-plane term; a meer is a principal by holding ecosystem permissions and no Group Role, so the genus must be permission-holding. (Primer A.4.)

  - **A helper's presence in scope is a fabric-level fact; only a persona's *use* of it is per-persona.** The earlier "meer adoption is per-persona" and "node-local revocation removes the meer from scope" both over-stated it. A persona cannot remove a meer from scope (its exposure is a delivery-fabric fact, like a swarm node seeing the sealed envelope); a persona can only withdraw its own use and reliance. The node-local plane is renamed **node-local withdrawal of use** and framed as the exit right exercised at the client, a recognition of autonomy, with the meer remaining in scope. (§5.4; primer A.3, A.5.)

  - **availability is a resource, not a Group Role** (§5.9). The exitability sentence had conflated raw blind availability (a §5.4 *resource*, needing no grant) with the read/search-offload Group Role (a governance grant, revocable under threshold, epoch-rotating). Separated: the read Group Role is revoked via governance; blind-availability reliance is shifted by the per-persona withdrawal-of-use, which aggregates to the Group-level non-hostage property.

  - **Helper governance and alignment named as a first-class concern** (§5.4 forward-pointer; primer B.5). Because scope spans Groups and includes helper roles, helper operation/governance shapes system-wide exposure at a layer no single persona's non-interaction dissolves. Captured and deferred (not specified in this pass).

**Cross-pass note (§5.4 → §6).** The corrected §5.4 helper treatment now leans on the *delivery fabric* concept (a meer's scope presence is a fabric-level fact) ahead of its formal introduction in §6. This is deliberate (the correction could not wait), but it creates an obligation for the Pass 2 delivery fold: the Delivery Fabric definition landed in §6 must be made consistent with how §5.4 already uses it, rather than introduced as if fresh. Flagged so the §6 fold repoints rather than redefines.

**§4 (Data Model), done (light).** Mostly key-layer identifier terms where lowercase **group** correctly survives per convention (`group_id`, `group-genesis`, `group gossip topic`, "the group's ratchet tree/secrets", "member of this group at this epoch"), left as-is. Fixed one **member of a group → Group** in the §4.5.1 client-definition citation to match §5.2.

- **Gossip-topic model-vs-realization conflation fixed (§4.2), as a worked demonstration of scope.** Rather than only asserting the mapping is configurable, the note now *teaches scope by consequence*: it holds the topic-seed choice as a variable and reads off three outcomes, (i) seed from one Group's ID (the reference default, scope tracks Group one-to-one on the gossip axis), (ii) seed from a value shared across Groups (one swarm serves several Groups, scope is wider than any Group, the case that shows scope and Group are different objects), (iii) include/exclude a helper from the path (same Group, same topic, wider or narrower scope, the proof the topic alone does not fix scope). The mapping of scope to Group is thereby shown to be a **delivery-fabric decision**, not a fixed coupling; the Group-ID derivation is a reference-realization choice, and the model requires only a high-entropy topic seed. **Clarified (per review): the gossip topic is *not synonymous* with scope; it is one large contributor.** Scope is the whole delivery-fabric exposure envelope (gossip, direct connections, relays, meers, push-notify nodes, and the metadata each sees); a relay/meer/push node in the path adds to scope independently of the topic. This does double duty: corrects the earlier tacit "scope is Group-specific" assumption *and* is the clearest available illustration of the scope-vs-Group distinction. (Primer A.3; doc-method Rule 3 vocabulary corollary.)

**§6 (Transport), terminology-only, done. Delivery-fabric *content* fold deferred to Pass 2.**

- **membership-sense scope → Group** in §6.1.2: "membership in a governed Group," "act through several clients in one Group," "member of many Groups," "actor in a Group," "member of Group S." Left MLS-key-layer **group** lowercase (ratchet tree, secrets, "this group at this epoch", group ID/epoch inference in the metadata discussion, group data/authentication), per convention. "Peer identity vs Group identity" and "one Group weighting another" capitalized (Group-identity plane, valuation edge). (Primer A.1, A.3.)

- **the peer and the Group planes** (§6.1 opening), and **a Group acting as a grantor** (§5.10 line that describes MLS's lack of the concept), capitalized as Group-identity / Group-as-principal.

- **Interaction tiers chosen at Group creation** (§6.3), a Group-level configuration decision.

- **meer "role" mislabel corrected** (§6.5.3): the meer's offline durability is an **additive, blind durability function / resource**, not a "revocable role." Rewritten consistent with the §5.9 resource-vs-Group-Role correction: it is a resource a node offers (blind availability needs no grant), not individually removable from scope, but any member may withdraw its own reliance (node-local withdrawal of use). Heading "the additive role" → "an additive function." Also §6.6.2 heading "the seed role" → "the seed function" (avoid role/Group-Role collision; it names a function, not a governance grant). Left "no DS in its ordering role" (§6.5, the MLS Delivery-Service function-sense) as-is.

- **Cross-pass note honored:** the §5.4 forward-lean on the delivery fabric is noted for the Pass 2 §6 fold to reconcile; §6's Delivery Fabric definition is *not* introduced or reworked in this pass (that is Pass 2).

**§7 (Synchronization and Governance-Conflict Resolution), done.** scope → Group for all governance facts: the unconflictable-root founding fact and Group genesis (§7.3, and corrected "the scope id is H(tag ‖ group_id)" to "the Group's genesis id"), the per-Group escalation tolerance throughout §7.4.1 and its §7.3/§7.4 cross-references, the Group's current governance head (§7.3.3), and the re-formation fork producing a differently-shaped Group / surfacing to the affected Group (§7.6). role → Group Role in the §7.2 grant-interface note ("in-Group governance authority"). Left genuine ordinary/range senses lowercase ("out of scope," "replay scope," "scope of events replayed"). (Primer A.1, A.2, A.3.)

**§8 (Security Considerations), done.** scope → Group: capture a Group, a Group's regime/visibility born at genesis, action to Group governance, Group immune memory, and (§8.1) the escalation tolerance left to Group policy plus the content-visible gating **Group Role**. (Primer A.1, A.2, A.3.)

**§9 / §10 (Interoperability/Conformance; Substrate Requirements), done for terminology.** scope → Group where governance is meant (per-Group epoch keys §10.2; survivor re-key of the Group / rejoin its own Group in the §10 tenure test). Left the transport/discovery **scope** senses lowercase and correct (T6 "overlay for scope-member discovery keyed on the scope topic," "any two scope members can reach each other"), which the §4.2 gossip-topic clarification now makes precisely right (these are the exposure/dissemination layer, not Group membership). role → Group Role in the §10.4 Group-Role-layer references ("in-Group governance authority"). **Appendix D.7 invariants sharpened:** I1 now says only a Group-governance principal holds a Group Role/right/weight, with an explicit note that a meer is still a broad-plane principal holding ecosystem permissions (so "holds none of these" is a governance-plane claim, not a claim the meer is authority-less); I2 and I4 updated (Group Role→granted-to-principal; ecosystem permission→held-by-non-Group-principal; capability issued-under-a-Group-Role). (Primer A.1, A.2, A.3, A.4; doc-method Rule 3 vocabulary corollary and Rule 8 resource-vs-grant.)

**Requirement-vs-realization (Pass 4 queue).** Per the (a)-scope decision, this pass fixed outright *model* conflations on sight (notably the §4.2 gossip-topic demonstration) but did **not** systematically recast every "MLS group stating a requirement" into "Group, realized by MLS." That systematic recasting is Pass 4's defined job and is consolidated in §10; MLS-key-layer "group" (ratchet tree, secrets, RFC-quoted definitions, wire structures) was therefore left lowercase in this pass where it describes MLS's own object as such. Queue for Pass 4: sweep §6/§10 for sentences where the requirement is meant and the noun should be capital-G Group with MLS named as realization.

**Verification (whole document).** No em-dashes, no double-hyphens in prose; no leftover "per-scope" or "in-group"; the sole remaining "PrincipalSet" is the Appendix D note explaining the rename; 14 "Group Role Set" instances; scope nouns doc-wide are either the §3 definition, the §4.2 demonstration, or genuine exposure/range senses.

---

## Pass 2: technical-reality fold

`Status: complete (all three folds done: delivery → §6; MLS hard-cases → §7/§8/§10.2; history-modes + side-histories → §7.7/§7.8)`

Scope: fold in the delivery-architecture material (`01`: three-plane C/D/P model, Delivery Fabric, gap-aware history convergence, D-peer, device-Group durability amplifier) into §6; the MLS hard cases (`mls-hardcases-and-posture`) into §7/§8/§10.2 with the inline alignment table; and the history-modes (`07`) and side-histories (`12`) material into §7. Resolve the three dissolving hard cases (external-join residual, insider-replay, epoch_authenticator) and promote under-determination into §7.6. Open items to Appendix B. (Primer B.1, B.3, B.4, B.5.)

Method for the fold (distinct from the Pass 1 terminology method): the source docs are written to the same requirement-first structure and the same Alice/Bob/Carol running example Part 2 targets, and they self-define their vocabulary (source `01` "Terms and definitions" already uses permission-holding principal, capital-G Group, the Delivery Fabric coinage) so the vocabulary is largely pre-reconciled to the Pass 1 result. The fold therefore *slots* material into Part 2's sections rather than rewriting it, reconciling three things as it goes: (i) any residual vocabulary drift against the Pass 1 conventions; (ii) cross-references (source `§X` → `§X` within Part 2, external → `Part 2 §X`); (iii) the cross-pass obligation from Pass 1 (below). The narrative-actor consistency of the *whole* Part 2 (making pre-existing sections match the running example top to bottom) stays Pass 3; the fold only carries the example where the folded material already uses it.

**Discharges the Pass 1 cross-pass obligation (§5.4 → §6).** Pass 1's corrected §5.4 leaned on the Delivery Fabric concept ahead of its introduction. This fold introduces Delivery Fabric formally in §6 and must **reconcile, not redefine**: the §6 definition is made consistent with §5.4's usage (a blind content-agnostic carrying overlay, larger than and overlapping the Groups over it), and the §5.4 forward-reference is repointed to the §6 definition. Also lands the deferred Appendix D lattice entries (Delivery Fabric, C/D/P plane prefixes) since these coinages are now formally introduced.

**Verification-tag normalization.** The delivery doc's ***Validated*** tag (functionally exercised against the real libraries iroh 1.0.1 / iroh-gossip 0.101.0 / mls-rs 0.55.2) is normalized to Part 2's ***green-real*** during the fold, per the doc-method Rule 6 directive; the two carry the same claim. Other delivery-doc tags map one-to-one (*Verified* → *Verified*, *[confirm]* → *[confirm]*, *Synthesis* → *design*/*Synthesis*).

### §6 delivery fold

`Status: complete`

§6 was **rebuilt on the three-plane model as its spine** (not inserted into the old structure), because the transport-and-delivery design outgrew a subsection: the merge is the old §6 folding *into* the delivery design, then seated back into the document. Title changed to "Transport and Delivery: the Three Planes, Identity, and the Encryption Stack." Old §6 (488 lines) → new §6 (~1044 lines).

**Structure (Option 2 order, confirmed with the author): substrate before outcomes.**

- **Frame:** delivery is three independent questions (Carriage / Durability / Presence); the pairing-and-independence rule; the three fusions named; the requirement-vs-realization stance in the header, carrying the **MLS DS ordering-declined demonstrative** (MLS's spec has a DS ordering function; Drystone references MLS heavily but declines that function because ordering rides inside the sealed messages, the standing example of "present in the realization, not required by the model").

- **§6.1 identity planes, §6.2 encryption stack** (the substrate), placed first because the fabric's blindness and the narrow observer exposure are *outcomes* of sealing and the identity split, not premises. Folded from the old §6.1/§6.2, terminology already Pass-1-clean; discovery ref repointed to §6.9, observer picture cross-linked forward to §6.4 to avoid duplication.

- **§6.3 Two populations: the Delivery Fabric** (first-class), and **§6.4 observers/metadata floor**, as the outcomes. §6.3 defines the Delivery Fabric and states scope against it: **a Group's scope is its exposure envelope measured over the Delivery Fabric**, gossip topic one contributor, each helper another. **Discharges the Pass 1 cross-pass obligation:** the §5.4 "delivery fabric" references (and the §4.2 one) are repointed/capitalized to the formal §6.3 definition, reconciled not redefined.

- **§6.5 Carriage** (C-direct / C-relay / C-swarm), **§6.6 Durability** (D-self / D-meer / D-peer, racing, and the device-Group durability amplifier), **§6.7 Presence** (P-none / P-gossip / P-meer / P-push, detector-plus-actuator, the minimal push-notify role, mobile-wake-as-accommodation), the three-plane spine, folded from delivery `01` §3a/§3b/§3c/§4/§5/§6. The old §6.5 meer material (byte-identical-sealed-bytes rule) merged into §6.6.2; the meer "role" mislabel already corrected in Pass 1 carried through.

- **§6.8 gap-aware history convergence** (single definition per name-once, durability-homed but cross-plane), the plane-fusions gathered, the adaptive selector, and the payload classes (live-durable / intrinsic-ephemeral / chosen-ephemeral / self-destruct).

- **§6.9 discovery** kept distinct and placed after the planes, flagged **under-developed** (owed future work, Appendix B), with an early allusion at the frame and connection-establishment lead; **§6.10 the gossip overlay** as the current realization of C-swarm, requirement-first (the epidemic-broadcast requirement stated before HyParView/PlumTree, swappability explicit); **§6.11 deployment modes** reframed as named bundles of plane choices; **§6.12 media**.

**Verification-tag normalization applied:** all delivery-doc *Validated* → *green-real* (10 green-real tags in the section). **Mechanical hygiene:** no em-dashes, no double-hyphens in prose, no un-normalized *Validated*.

**Cross-reference repointing (whole document):** all §6-subsection references outside the rebuilt section updated to the new numbering (old discovery §6.4.x → §6.9.2; old meer §6.5.3 → §6.6.2; old gossip §6.6.x → §6.10.x; old deployment §6.7 → §6.11; old media §6.8 → §6.12), across §7, §8, §10.3, Appendix B, and the References block. Internal §6 self-references verified (every §6.5.3 is C-swarm, every §6.6.x durability, §6.7.1/§6.7.2 the push/wake subsections).

**Appendix D.8 added:** the delivery-plane vocabulary as first-class lattice entries (Delivery Fabric, scope-over-fabric, the three planes and their C/D/P prefixes, the plane fusions, gap-aware history convergence, device-Group, the three resource-asymmetry roles). This lands the deferred coinage entries flagged at the end of Pass 1.

---

## Flow-and-alignment pass (Rule 1 end-state + Rule 7 name-once enforcement)

`Status: complete (this round; standing check on all future fold output)`

Triggered by review: the fold (and older material) had accreted **past-reasoning** (legacy-label callouts, "an earlier draft said/mislabelled X," retirement notices, refutations of positions no reader holds) and **duplicate definitions**, both forbidden by doc-method Rule 1 (the doc is an end state) and Rule 7 (one mechanism named once). A published spec states the current conclusion once; the path belongs in the session log, not the artifact. This was a **whole-document** pass, not §6-only, because the fault was systemic.

**Past-reasoning removed / rewritten to state the conclusion positively:**

- §5.2 meer definition: cut the "legacy labels mere-peer / blind member / blind peer are all wrong" refutation and the "peer as an entity noun is retired in favour of persona" aside; now states plainly what a meer is (a broad-plane principal holding ecosystem permissions, a blind store-and-forward node, no Group Role/right/weight, defined in §5.4, realized in §6). This was the example flagged in review.

- §6 frame: cut the "filing it as a durability option then apologizing it is not durable" ghost, and the "Two structural notes on what comes first and why" document-ordering meta-commentary (the section is now simply ordered substrate-then-consequence, stated as fact).

- §6.2.3: cut "an earlier Drystone draft asserted the opposite ... that claim was unsourced and is removed" (kept the sourced positive claim).

- §6.6.2 (D-meer): "does not order messages, and this is worth being precise about because it is easy to misread as a sacrifice. It is not." → "does not order messages, and ordering is not lost by this; it is sourced elsewhere."

- §6.8.2 and §6.9: cut "a flatter model would file the swarm ... then have to apologize," and "flagged as under-developed rather than dressed as complete" (now: the swarm's non-durability is stated outright; discovery is under-developed, stated plainly).

- §3.1: cut "(This replaces the looser 'serverless' usage in earlier drafts.)"

- §4.2 content-id note: "Earlier drafts included a timestamp field ... It is removed" → "The content-id pre-image carries no timestamp field" (stated as the rule).

- §5: "a single overloaded word ('peer') previously hid several independent ideas" → the section's job stated positively; both "an earlier draft mislabelled/called 'capability'" resource-rename notes cut (kept the positive reason the word is "resource"); the dropped-`share`-right narration ("an earlier draft floated a fourth ... dropped") rewritten as "a claim on a Group's commons is not a right"; "conflating them (as an earlier draft did, by asserting 'personhood is unforgeable')" rewritten to state the two-layer split and the collapse it avoids.

- §8: "earlier drafts listed 'timestamp' among observed join-metadata" → states what a relay observes (arrival ordering as locally observed, never a timestamp).

- §10 opening: "not a weakness of the cryptographic-systems lineage but its point" and "the human-adjudication layer is not a defect of this class either" rewritten as positive statements of the design's intended class and the explicit human layer.

- Appendix D: I5 "never the retired entity-'peer'" → "never a node, and never the relation-sense 'peer'"; D.6 "Renamed from the earlier 'PrincipalSet,' which wrongly implied ..." cut (now "It is a set of Group Roles, not of principals"); D.6 heading "the reassigned senses" → neutral. **"PrincipalSet" now appears nowhere in the document** (correct end state; the earlier "one intentional occurrence" note is void).

- The iroh-gossip version note tightened from mildly narrative ("split out of the iroh crate before 1.0 and explicitly no longer receiving 'special treatment'") to a current-state statement ("a separately-versioned crate on its own pre-1.0 release line, outside the 1.0 guarantee"). This is a *current fact about an external dependency*, deliberately kept, not Drystone past-reasoning.

**Duplicate definition consolidated (Rule 7):** the §5 opening had a principal/client/persona overview that fully re-defined all three, duplicating §5.2 (the identity-model section that owns them). The overview is now a two-layer *framing* (identity layer = principals/clients; governance layer = personae/weight) that defers definitions to §5.2. Verified nothing was stranded: §5.2 carries every detail the overview dropped (persona-is-neither-client-nor-device, the hosting chain, one-persona-per-rooting-key, the human-binding-is-a-Group-judgment).

**Verification:** no em-dashes, no double-hyphens in prose, no "PrincipalSet," no stray "Validated," 14 "Group Role Set." Residual "no longer" / "kept for" instances audited and confirmed to be current-state statements (external dependency status; current usage rules), not past-reasoning.

**Standing check going forward:** Rule 1 and Rule 7 enforcement is now applied to fold output as a matter of course, because a fold pulls in companion-doc prose that tends to carry its own "we changed this" scaffolding; the fold must strip it, since the destination is a published end-state spec.

**Second round (prior-state-contrast + narrative flow).** Review caught that Rule 1 covers more than the explicit "earlier draft" phrasing: any construction that only parses if the reader knows a *superseded prior state* is the same violation. Cleaned: "no longer receiving special treatment" and "no longer carried as open" (iroh/iroh-gossip status, restated as current fact); "is now given a concrete shape" (the Group-principal open seam, twice, restated as the current shape); "'operator' and 'user' were both rejected" (naming history cut); the event-enum version archaeology ("an earlier `Joined` event was folded into `NeighborUp`; the enum was later flattened" → "the exact enum variants vary across releases"); and a further §5 autopsy missed the first round ("the sentence that replaces every earlier formulation ... the old phrase was wrong twice over" → the model stated positively in one sentence). The `EndpointId`/`NodeId` note was kept but tightened to a clean *external-dependency* cross-version fact (iroh's own rename), not Drystone past-reasoning; likewise the Appendix B open-items tracker legitimately records status transitions and was tightened, not stripped.

Then a **narrative-flow read** across every edited passage, checking for seams opened by the surgical cuts (orphaned connectives, transitions that lost their antecedent, broken punctuation). Findings: the cuts held cleanly, because they removed asides and refutation-tails rather than load-bearing connective tissue. Two transitions were smoothed where a cut list-item had left an opener dangling (§5's "And two layers" → "The four properties sit across two layers"; the one-sentence model statement reconnected to the capability note). Punctuation-artifact and orphan-connective scans came back clean (the apparent hits were code, the Appendix D `1..N`/`is-a` lattice notation, and legitimate concluding "So ..." statements). Hygiene re-verified: no em-dashes, no double-hyphens in prose, no "PrincipalSet."

---

### §7 / §8 / §10.2 MLS hard-cases fold

`Status: complete`

Folded `mls-hardcases-and-posture.md` (ten hard cases + concept-alignment map + posture table + open items) into §7, §8, and §10.2. Source was written to the same requirement-first discipline and Alice/Bob/Carol/Dave cast, so it slotted in; the fold reconciled vocabulary (the source's pre-Pass-1 "per-scope" → "per-Group", "role" → "Group Role"), cross-references (source `§N` → the Part 2 section it lands in), verification tags (source `Synthesis` → `design`), and applied the **Rule 1 fold-boundary check from the first draft** (the source carried "not a defect" / "not a failure" framings; each was stated positively or trimmed on the way in, so no scrubbing round was needed after).

- **§7.6 rebuilt around the two-member escalation set and the re-plant mechanism.** §7.6 already had fork-not-verdict for *contradiction*; the fold added **§7.6.1** (the escalation set has **two** members: contradiction = too many valid claims, and **under-determination** = too few, a required Group Role vacant with no admissible successor, the Dave-sole-admin case, which discharges the primer's "promote under-determination into §7.6"), **§7.6.2** (fork / heal / routine re-key are **one primitive at three arities**, the delivery layer's re-plant, with the governed classifier of §7.4.1 choosing the arity), and **§7.6.3** (MLS is subordinate: the MLS group is disposable key infrastructure, the conversation persists across a sequence of them; the transcript hash can represent only one linear sequence, RFC 9420 §8.2; the three arities map onto MLS ReInit/branching linked by resumption PSK; and the ReInit non-atomicity freeze-then-strand hazard, RFC 9750 §6.1/§7, with the record-intent-before-freeze candidate resolution).

- **§7.4.2 added: two MLS recovery hazards the corroboration model dissolves.** External-join (a stale `GroupInfo` cannot defeat PCS because the governance chain, not the `GroupInfo`, is the authority; the monotonic fold makes a forged assertion self-forking and the per-Group threshold quantifies the attack MLS leaves unquantified) and insider-replay / nonce-reuse-on-restore (isolated because gap-aware convergence runs out of band and never re-injects into the live MLS stream). These are two of the "dissolving cases" the primer flagged; each carries its owned residual as **[confirm]** to Appendix B.

- **§8.1 added: FS and durable history are not in tension** (FS assumes ciphertext retention and deletes *keys* on schedule, so a durability node holding sealed bytes is inside the scenario FS survives; the real friction is key-retention-under-reordering and the offline node, both about keys not ciphertext; self-deleting content is a different object at a different layer, cooperative non-retention). **§8.1.1: DMLS / FREEK as a research pointer, not a dependency** (adopt content-derived epoch ids and the PPRF idea, decline the two-server consolidation). This realizes the primer's FS-vs-durable retraction (B.3).

- **§10.2.1 added: concept-alignment map + posture-summary table** (the inline alignment table requested). Direct (re-plant↔ReInit/branching, resumption-PSK, rejoin-with-PSK), Partial (conversation continuity, content-half recovery, staleness), Drystone-only (OOB convergence, monotonic fold, fork-not-verdict), Underused (epoch_authenticator, cross-group PSK). The ten-row posture table maps each case to what MLS assumes, the Drystone posture, and the forcing principle, with every mechanism cell pointing at its developed section. Reduces to one posture: MLS supplies the local check and assumes a coordinator for global agreement; Drystone does not re-add the coordinator and decides per case whether global agreement is unnecessary, reconstructible, or a §7.6 human-escalation.

- **Appendix B: MLS hard-case residuals added** as a grouped block (external-join far-behind node, in-place secret restore, re-plant intent ordering, the KeyPackage-exhaustion seating trilemma, re-plant seating default, epoch_authenticator overlap, cross-group PSK, transcript-hash formula, epoch-number metadata-leak-vs-re-plant-frequency), each a specific stress-test rather than a design gap. Tenure-under-re-key was already present.

**Verification:** no em-dashes, no double-hyphens in prose, no "PrincipalSet," no "per-scope," no stray "Validated." All new cross-references (§6.8.1 convergence, §7.4/§7.4.1/§7.4.2, §7.6.x, §8.1, §10.2.1, Appendix B) resolve.

---

### §7.7 / §7.8 history-modes and side-histories fold

`Status: complete`

Folded `07-history-modes.md` and `side-histories-and-threading.md` into two new §7 subsections, preserving each source's epistemic status (modes are settled; side-history tiers 2/3 are candidate; self-destruct is an open thread), with the Rule 1 fold-boundary check applied from the first draft (no scrubbing round needed).

- **§7.7 Dataplane history: two modes.** The two mutually-exclusive, chosen-at-Group-creation modes: **forward-only** (append-only hash-linked fold, the §7.3.1 spine applied to content, for large/loosely-coupled Groups, cleanup only via coordinated roll-up) and **Willow-mutable** (path-addressed, overwrite-and-prefix-prune, convergent capability-gated deletion, for bounded Groups). The Willow-shaped-already hedge (RBSR + content addressing from §6.8.1/§7.1 make the mutable mode an evolution not a rewrite, free in the forward-only case). **§7.7.1** prefix pruning's capability gain and its two hard limits (metadata-convergent-not-erasing tombstone; cannot reach a copy already taken), the same cooperative-non-retention wall as §6.8.4. **§7.7.2** coordinated history roll-up as a distinct governance item, separated from chosen-ephemeral and self-destruct. **§7.7.3** self-destruct as node-fidelity-bounded and inverse-of-provenance (opts out of D-meer/D-peer on principle; fidelity boundary is the membership and must be legible; **achievable semantics differ by mode**, real convergent removal in Willow-mutable, display-mask-only in forward-only). This realizes the §6.8.4 forward-pointer and keeps self-destruct an open thread.

- **§7.8 Side histories: three tiers.** Tier 1 threading (a subid into the existing tree, settled, no new mechanism), tier 2 separate-but-inherited side history (a second dataplane hash tree under the same keys, entitlement inherited, no new MLS group, candidate), tier 3 subgroup (a real MLS branch with its own entitlement, the fork arity of the re-plant family §7.6.2, candidate). The selector rule (different keys → tier 3; different structure only → tier 1/2) and the load-bearing point that **entitlement inheritance makes tier 2 cheap and disqualifies it the moment access must narrow**, so the tier-2/tier-3 boundary is the cost cliff.

- **Appendix B** gained the history/side-history open threads (mode migration and fixed-at-creation question, the size bound, self-destruct specification, the tier-2 feature-or-data-model-note central question, tier-2→tier-3 promotion).

**Verification:** no em-dashes, no double-hyphens in prose, no "PrincipalSet," no "per-scope," no stray "Validated." Also fixed two Pass-1-residue lowercase "role" instances in §7.2 (R1) → Group Role. New cross-references (§6.8.1, §6.8.4, §7.1, §7.3.1, §7.6.2, §7.6.3) resolve.

---

## Pass 3: narrative-actor consistency

`Status: complete`

A full read of the whole document for running-cast consistency, distinct from the folds (no new material, just reconciling the Alice/Bob/Carol/Dave example top to bottom). The payoff of fold discipline showed here: because every source doc used the same cast with the same fixed trait meanings, the fold introduced **no collisions**, and Pass 3 was light anchoring rather than repair.

- **Cast verified consistent.** Alice is the two-device persona (phone + laptop, the device-Group case §6.6.5); Bob is one device, the recurring "briefly offline" case; Carol is the equal-standing mutual-expulsion counterpart; Dave is the sole-admin whose only node goes stale (§7.6.1). Every use of each name checked against its fixed trait: Alice is never given one device, Bob never multiple, and the §6 device-Group walkthrough ("Alice's laptop caught a message her phone missed") is consistent with her anchor.

- **Defining traits anchored at first mention.** Rather than a formal dramatis-personae block (too heavy for a spec), the first appearance of each character now carries a short appositive fixing its trait, so later uses read as "the Alice we know" rather than re-introducing her. §6.5.1 anchors Alice (two devices) and Bob (one device, briefly offline); §7.6.1 introduces Carol and Dave with their governance roles inline.

- **Fixed a forward-reference.** §7.6.1 said "the mutual expulsion above" for a case first named there; rewritten to introduce Carol and Bob's equal-standing mutual removal cleanly at the point of first use.

- **Generic actors left generic on purpose.** Property and definitional claims in §3–§5 and §7 (e.g. "a returning member that cannot...", "mounting the attack costs compromising a quorum, not one member") correctly stay generic; casting them would be decorative. The cast is used only in concrete scenario walkthroughs (§6 delivery, §7.6 governance conflict) where continuity earns its keep.

**Verification:** no em-dashes, no double-hyphens in prose, no "PrincipalSet." Both cast anchors present.

---

## Pass 4: requirement / definition / realization consistency

`Status: complete`

The queued systematic recast (primer: "the capital-G Group is what Part 2 realizes over an MLS group"): where a sentence states what Drystone *requires*, the noun is capital-G Group in the design's own terms; where it describes how MLS *provides* that, the realization is named in MLS's terms. The standing demonstrative is the MLS DS ordering function, which Drystone declines because ordering rides inside sealed messages (§6.6.2, §7.6.3).

The finding: the separation was **already largely maintained**, because the §6 rebuild and the MLS-hard-cases fold applied requirement-vs-realization discipline as they went. A full scan confirmed:

- **Every "MLS group" usage is correctly on the realization side** (the disposable key layer): §3 "Groups... each realized over an MLS group," §5.10 "the MLS group is the key layer... not defined by it," §7.6.2/§7.6.3 "instantiate a fresh MLS group / the MLS group is key-distribution infrastructure / cannot be expressed inside an MLS group," §7.8 "not a new MLS group," §10.2 "the MLS group-key core." None states a requirement in realization-nouns.

- **Two genuine fusions found and recast**, both in §5.8, where requirement language (`MUST`) was fused to a key-layer verb: "revocation MUST rotate the Group epoch" → "revocation MUST exclude the revoked principal from reading content authored after it folds in (R5 forward-read exclusion, §7.2), which the MLS realization delivers by advancing to a new epoch." The requirement is now stated in the design's terms (forward-read exclusion) with the epoch advance named as the MLS realization; the §5.8 recap was made consistent.

- **The rest confirmed correctly layered, not over-corrected.** Realization-describing prose that legitimately uses MLS vocabulary (leaf, ratchet tree, `PrivateMessage`, "sealed to the Group's epoch" in the §6.2 encryption-stack description, "MLS actively deletes keying material as the Group advances") was left as-is: forcing "the MLS group's epoch" into every such sentence would be pedantic and would obscure that it is *this Group's* content being sealed. The recast applies to requirement-verbs fused to key mechanics, not to every epoch mention.

- **§10 is the consolidation point** and already states the requirement-vs-realization discipline as its organizing principle (K1–K8 requirements with MLS named as the current best realization, §10.2; the concept-alignment map, §10.2.1). One Rule 1 residue fixed here: "the earlier mechanism-neutral text stands" (a prior-state-contrast construction) → "the mechanism-neutral treatments elsewhere carry the same separation."

**Verification:** no em-dashes, no double-hyphens in prose, no "PrincipalSet," no "per-scope," no stray "Validated," 14 "Group Role Set."

---

## Primary-source verification pass (Appendix B / C `[confirm]` items)

`Status: complete for the checkable subset; internal design decisions correctly left open`

Worked through the `[confirm]` items, separating those a primary source can settle now from internal design decisions (which no primary can close). Verified against primaries this session and updated in place:

- **Transcript-hash §8.2 formula** (was flagged not-read-verbatim, load-bearing for §7.6.3). Read verbatim: `confirmed_transcript_hash[n] = Hash(interim_transcript_hash[n-1] || ConfirmedTranscriptHashInput[n])`, `interim_transcript_hash[n] = Hash(confirmed_transcript_hash[n] || InterimTranscriptHashInput[n])`, seeded from zero-length strings. This is exactly the strict single-predecessor chain (no branch/merge representable) §7.6.3 relies on. §7.6.3 parenthetical strengthened; Appendix B item marked resolved. *(RFC 9420 §8.2, via the verbatim quote in mlswg/mls-protocol issue #881.)*

- **Resumption PSK §8.6** co-membership property confirmed verbatim ("members entering the new epoch agree on a key iff they were members during the epoch from which the resumption key was extracted"). §7.6.3 / §10.2.1 already correct.

- **Epoch-number metadata leak** confirmed verbatim against RFC 9750 (opaque `group_id` + numerical epoch = change count; correlatable by a network observer; mitigation is a metadata-confidential transport). Appendix B item split: the leak is confirmed, the re-plant-frequency interaction stays the open design part.

- **Matrix Project Hydra facts** confirmed against Matrix's own primaries (Aug 2025 disclosure, v1.16 release notes, CVE records): **CVE-2025-54315** (rooms < v12 lack cryptographic create-event uniqueness; High; no known exploitation path), **MSC4289** (creator "infinitely high" power, *with the stated reason that backdating already gives the creator's server de facto control*, plus creator self-demotion and multiple creators), **MSC4291** (room ID = hash of create event). This **sharpened** the capped-root framing: Matrix's apex is forced *by backdating*, not as an abstract requirement, which strengthens Drystone's claim that a timestamp-free order removes the forcing reason. Corrected the Appendix B item and cleared the MSC4289 / MSC4291 / creator-power `[confirm]` markers in §5.7 and §7.3; the `H(tag ‖ group_id)` genesis is noted as the *same fix* Matrix reached (room/group id as genesis hash).

- **CALM theorem** attribution and statement confirmed (Hellerstein & Alvaro, "Keeping CALM," arXiv 1901.01930 / CACM 2020, conjectured PODS 2010, proof via Ameloot et al.): "consistent coordination-free implementation iff monotonic." Matches the Part 1 §2.5 / Part 2 §7.6.3 grounding exactly. Appendix C status updated.

**Left open, correctly:** the **MSC4297** conflicted-subgraph mechanism and the exact **CVE-2025-49090** root-cause wording (not read verbatim this session, so still `[confirm]`); the **epoch_authenticator §8.7** adoption question (already appropriately hedged as a candidate, not over-claimed); and all the **internal design decisions** (external-join far-behind node, in-place secret restore, re-plant intent ordering, KeyPackage seating trilemma/default, mode migration, size bound, self-destruct spec, tier-2 named-construct, hash-function pin, vendor naming, `ENABLING` encodings) that no primary can settle. The honest principle: clear a marker only when a primary actually grounds it; do not present an unread specific as verified.

**Verification:** no em-dashes, no double-hyphens in prose, no "PrincipalSet."

---

---

## Versioned upstream reference links + cross-document reference disambiguation

`Status: complete`

**Added a versioned "Upstream reference links" section** at the end of Part 2, pinning each external dependency and citation to a canonical, version-specific source so an implementer resolves the exact version this spec was written against, not a moving "latest." Organized by layer (MLS core; transport/overlay; data model/capabilities; governance-conflict resolution; formal spine; countersigning). Each entry carries a verification marker: those confirmed against primaries this revision (RFC 9420 §8.2/§8.6, RFC 9750 metadata/DS-ordering, iroh 1.0.0 on crates.io/docs.rs, Willow data model + Meadowcap at willowprotocol.org, CALM at CACM/arXiv, the Matrix Project Hydra / v1.16 / MSC4289 / MSC4291 / CVE-2025-54315 facts) are marked *(verified this revision)*; identifiers not confirmed verbatim (RFC 9714 for RoQ, the RBSR arXiv id, the CRDT report identifiers, the Sigstore mechanism, the DMLS draft revision) carry **[confirm]** rather than being presented as settled. The pre-1.0 iroh crate pins (`iroh-gossip`, the address-lookup crates) are stated as the production profile's to fix and cross-referenced to Appendix B.

**Cross-document reference disambiguation (a real correctness fix).** A full sweep found that bare `§2.x` references in Part 2 (41 of them) pointed at Part 1's section space without the "Part 1" prefix, so a Part 2 reader seeing "(§2.0)" would search Part 2 (which starts at §3) and fail. All 41 were prefixed to "Part 1 §2.x" (leaving the 4 genuine "RFC 9420 §2" references untouched). This brings the total consistent "Part 1 §2.x" citations to 94. The mirror defect in Part 1 (bare `§4`–`§8` references to Part 2's space) is fixed in the Part 1 changelog.

**Within-doc review confirmed clean:** every internal §-reference resolves (the non-matching ones are all RFC sections or the now-fixed cross-doc refs); the RFC section citations (§16.3 sender-data, §16.4.1 metadata, §16.6 FS/PCS, §16.7 key-pair uniqueness, §6.3.1/§6.3.2 encryption, verified against the RFC 9420 table of contents this revision) match what each cited section actually covers; the coined terms (Delivery Fabric, meer, re-plant) are each glossed or forward-pointed at first use; and the Pass 1 terminology discipline held through the folds (the only lowercase "role" tokens in the folded sections are the generic-English component-function sense, correctly not "Group Role").

**Verification:** no em-dashes, no double-hyphens in prose, no "PrincipalSet"; cross-document references disambiguated.

---

## Pre-publication verification pass (the checkable `[confirm]` items)

`Status: complete for everything a primary can settle now`

Worked the remaining checkable `[confirm]` items against primaries and cleared each only where a primary actually grounded it.

- **MSC4297 conflicted-subgraph mechanism** (adopted at §7.5.2): verified verbatim against the Matrix State Res v2.1 implementer's guide. The two changes are exactly (1) start the iterative auth checks from the empty set (was the unconflicted set) and (2) replay the conflicted state subgraph between any two conflicting facts, characterized as the strongly-connected component containing the contracted conflicted supernode and computed by intersecting forward-reachable and backward-reachable sets. §7.5.2 and the reference entry updated to verified.

- **CVE-2025-49090 root cause**: verified verbatim against the CVE record and Hydra disclosure, a state reset to an earlier/incorrect value absent a validly-producing event, exploitable by a malicious homeserver via a crafted event/API sequence, fixed by State Res v2.1. §7 supporting-case passage and the reference entry updated to verified.

- **Range-Based Set Reconciliation**: identifier confirmed (arXiv:2212.13567, Aljoscha Meyer) and the peer-reviewed venue added (Proc. 42nd IEEE SRDS 2023, pp. 59-69). Marker cleared.

- **CRDT identifiers**: confirmed and made precise, SSS 2011 (LNCS 6976, pp. 386-400, DOI 10.1007/978-3-642-24550-3_29) plus the companion INRIA RR-7506 "comprehensive study" (Jan 2011), noting RR-7506 is distinct from the later RR-7686. Corrected in both docs.

- **RFC 8446 record-padding + traffic-analysis residue**: both parts verified verbatim, RFC 8446 §5.4 (records may be padded to obscure lengths, but TLS does not hide transmitted length) and the corroborating arXiv:2406.15686 §6.2 (restates exactly this RFC 8446 limitation). Marker cleared.

- **The capped-root testing item** (§7.3 / Appendix B): its fact-dependency (MSC4289 / MSC4291 / CVE-2025-54315) is now discharged; reframed so only the Drystone-side test-coverage question stays open, which is a design/validation item, not a fact to confirm.

**One real correction.** **RTP-over-QUIC (RoQ) is not RFC 9714**, it is still an Internet-Draft (`draft-ietf-avtcore-rtp-over-quic`, -14 as of this revision), with no RFC number yet, the draft's own text states only implementations of the final published RFC may use the "roq" ALPN token. The earlier "RFC 9714 (Standards Track)" citation was wrong and is corrected to the draft, with a note that the media path rides a not-yet-final draft. **Sigstore** was also loosely characterized as "countersigning"; corrected to name its actual primitive (Rekor, an append-only Merkle transparency log with signed tree-head checkpoints), noting Drystone draws the checkpoint/Merkle-consistency analogy, not a literal Sigstore feature.

**What remains `[confirm]`, correctly:** the pre-1.0 iroh crate version pins (production profile's to fix), the DMLS working-group draft revision (moves, and only tracked not depended on), the Pkarr spec, the Keyhive canonical location, and the internal design decisions (MLS hard-case residuals, history-mode threads, capped-root test coverage). No checkable fact-verification marker is left open.

**Verification:** no em-dashes, no double-hyphens in prose, no "PrincipalSet."

---

## Pass: 2026-07 real-substrate experiment reconciliation

`Status: complete (RUN-02, 2026-07-13)`

Scope: fold the reconciled results of the 2026-07 real-substrate spike corpus (imported into `alpha/experiments/`) into Part 2, per the reviewed diff set `proposed-changes-2026-07-experiment-reconciliation.md` (items F1 through F8). The two `needs-call` items were answered by the owner, and F8 was added as a new decision this run. Each item cites the experiment that earns it; the divergence register and the reviews-and-experiments log carry the evidence.

**§7.2, done.** Added **R7, content-bound quorum for policy changes**, the one genuinely new mechanism this pass (a grep of the prior spec for rulechange returned nothing). A policy change (a threshold or a rule) is admitted only when k distinct personae by lineage each author an approval fact referencing the content hash of the exact change, so the quorum is enforced at fold time rather than merely stored. Three semantics are pinned as normative: the prior rule governs (the threshold consulted is the one in force at the act's causal position, so a proposer cannot lower the gate with the gated act), under-quorum is pending rather than partial (approvals are antecedent governance facts, and an enacting act without sufficient matching approvals is rejected deterministically), and the approval subject is the digest of the canonical payload bytes (§4.6), not of the enclosing fact envelope. R7 is R3 generalized from membership and role revocation to rule changes. Two residuals are recorded alongside it: a hard-coded role-authorship gate (a spike simplification; per-act approver-role granularity stays open) and the two-competing-quorums case (F8). `Modeled`, pending the X3 cross-package mutation sweep. (F1.)

**§7.3, done.** Added a cross-reference at the §7.3.1 authorization precondition pointing policy-change threshold enforcement, the approval subject, and the prior-rule-governs semantics to §7.2 R7 (F1). At §7.3.2, recorded the two-competing-quorums decision (F8): two concurrent policy changes to the same rule that each meet quorum are a §7.6-class genuine contradiction, hard-stopped and human-adjudicated, never content-address-tiebroken, because a silently losing rule rewrites the Group's constitution downstream with the disagreement never seen, manufacturing a utility verdict Part 1 §2.5 forbids. `Design`, decided; no evidence tag until the two-competing-quorums experiment runs.

**§7.6, done.** §7.6.1's contradiction enumeration gained the R7 quorum-collision as a third example of too-many-valid-claims (F8). §7.6.2's re-plant paragraph gained a corroboration: the membership half is `Verified` (an MLS group stamped over the fold-derived set has exactly that set as its cryptographic membership, across genesis, authorized adds, real removals, and rejected unauthorized changes, driven through the real fold-ingest path on real openmls 0.8.1), while the message-continuity half stays open pending the dataplane hash structures (Appendix B / B1) (F2).

**§8.2, done.** Tightened two honesty boundaries. Clause (e): policy-change quorum enforcement is now test-run (RED to GREEN, §7.2 R7), with the freshness precondition on originating such an op (§7.4.2) still not exercised over live transport (F1). Clause (a): freshness is demonstrated over loopback live transport (2-node and 4-node real iroh-gossip convergence to identical fingerprints) but not yet over the relay plus holepunch real-NAT path, which needs live NAT traversal (X1) (F5).

**§6.8.1, done.** Annotated the RBSR-construction line: connect-time catch-up was demonstrated over real iroh-gossip (a late joiner reaches an identical head on NeighborUp) but via a whole-retained-log re-broadcast, a coarser push cousin of RBSR rather than the diff-only range reconciliation itself, and steady-state anti-entropy stays unexercised. Both remain open (F3). The annotation removes a possible over-read rather than adding a claim.

**§11.11, done.** Annotated measurement #1: the per-commit band is now measured on real openmls 0.8.1 (an O(N) floor to an O(log N) ceiling per commit; `mls-replant` M1), while the fan-out half stays unearned though runnable with no new infrastructure. The measurement moves from unearned to half-earned (F4). The §11.12 posture table is untouched.

**§10.5, done.** Added a conformance-vector footnote beneath the dependency-versus-realization ledger: conformance categories 7/8/9 (attributable acceptance, visibility, freshness) and the revoke-authority-threshold vector are specified but not yet emitted, gated on MLS key-distribution over the wire and threshold-revoke as real k-of-n over the wire (F7). No ledger row unambiguously covers the conformance vectors, so no row was annotated.

**F6, no change.** §7.6.3's ReInit-stranding `[confirm]` stands: `mls-replant`'s last-resort-availability result (E12.6) shows a re-plant can be completed from the chain (availability) but does not establish the intent-recorded-before-freeze ordering the discharge requires. Recorded so E12.6 is not mistaken for the discharge.

**Map, done.** Updated the §0 map entries for §6, §7.1/§7.2, §7.3, §7.6, §8, §10, and §11 per Rule 15.

**Verification:** no em-dashes, no double-hyphens in prose, no "PrincipalSet"; every F-item reflected, the F6 non-change recorded.

---

## Human-adjudication language pass (§7.6 shapes/parties and local-authority terms)

`Status: complete`

Part of the cross-suite adjudication-language codification (conventions A.11, Part B §B.8).

**§7.6.1, done.** Added a one-clause terminology note after "they differ only in how they are detected": the two members are the **escalation shapes** (Contradiction, Under-determination), distinct from the **escalation parties** a given case surfaces, who are presented symmetrically (both parties, no presumed wrongdoer), per conventions A.11. The §7.6.1 running example's "the escalation set is both Alice and Bob" was corrected to "the escalation parties are both Alice and Bob," since the two personae are parties, not shapes; the rest of the sentence (the vacant-role point, beat E5) is unchanged.

**§7.6, done.** Where the hard-stop is first said to surface the conflict "to the affected Group," named the receiving side: ", the **local authority** for a shared judgment (conventions A.11)." This gives escalation text a name for the receiving side that is not a node role, per A.11 DR-4/DR-10.

**Map, done.** Updated the §0 map §7.6 line per Rule 15: it now names the local authority as the receiving side and the two escalation shapes (distinct from the escalation parties), all anchored to conventions A.11.

**Verification:** no em-dashes, no double-hyphens in prose; no code identifier renamed (`ForkStatus::Contradiction`/`UnderDetermined` untouched); cross-references resolve.

---

## Continuity decoupling and the reconciliation-horizon cadence (§7.6.2, §7.6.9, Appendix B)

`Status: complete (RUN-03, 2026-07-14)`

Scope: land the two design decisions made after the 2026-07-14 merge, both in §7.6, both `Design`-tagged. No mechanism changes; two paragraphs added and one Appendix B gates-release item, per the render-over-re-plant and reconciliation-horizon calls (owner-decided). DR language (conventions A.11) applied throughout: continuity-framed, no moral framing of a member's choices.

**§7.6.2, done.** Added the continuity/decoupling passage immediately after the one-primitive-three-arities paragraph. It states that a member's attachment is to the Group (the lineage rooted at genesis, §5.10), never to an MLS instance, which is a disposable enforcement vessel a re-plant repoints. Two decouplings are made normative: an epoch roll carries no inherent social meaning (standing is read from the decision plane §7.3.5, never from the enforcement plane), and membership history composes over gaps (a depart-and-rejoin is one continuous relationship rendered as two spans, with §5.11 read-scoped keys making the gap substantive). Divergence is a property of folded state (§7.3), never of renders, so two members holding different narratives over identical facts is expected operation. `Design`, decided; the render layer is out of protocol scope. Design exploration: `alpha/thinking/the-shape-of-disagreement.md` and the new `alpha/thinking/reconciliation-horizon.md`.

**§7.6.9, done.** Added the reconciliation-horizon cadence as a worked example of the section's temperament-dial frame, after the there-is-no-single-correct-configuration paragraph. A Group MAY set a horizon cadence (R7-governed like any rule): a boundary at which each member re-evaluates its open contradictions and MAY record a horizon checkpoint, the §7.3.3 self-checkpoint extended with a manifest of open contradiction byte-heads (§7.6.1), co-signable with unchanged §7.3.3 semantics. The cadence composes the two log-derivable streams (fire on every epoch commit; additionally on N facts since the last horizon, counter resetting at each boundary), with N a per-group temperament dial and no wall-clock in either term. The horizon gates presentation staleness, never resolution: a contradiction never expires. `Design`. Full treatment: `alpha/thinking/reconciliation-horizon.md`.

**Appendix B, done.** Added one `[gates-release]` wire-encoding item alongside the existing returning-member checkpoint-encoding item: the horizon-checkpoint manifest encoding (frontier head plus the sorted set of open contradiction byte-heads that extends the §7.3.3 self-checkpoint), §7.6.9.

**Map, done.** Updated the §0 map §7.6 line (§7.6.2 continuity/decoupling passage; §7.6.9 horizon-cadence worked example) and the Appendix B line (the horizon-checkpoint manifest joins the wire-encoding items) per Rule 15.

**Verification:** no em-dashes or double-hyphens in this changelog prose; the Part 2 inserts use house em-dash style; MUST/MAY casing preserved; no mechanism or code identifier changed; cross-references (§5.10, §5.11, §7.3, §7.3.3, §7.3.5, §7.4, §7.6.1) resolve.

---

## Corroboration dials and the formula-valued freshness threshold (§7.3.3, §7.4.1)

`Status: complete (RUN-04, 2026-07-14)`

Scope: land two owner-decided `Design` framings against the completeness-ahead beam, both in §7. No mechanism changes; two paragraphs added and two §0 map lines updated, per the corroboration-dials call. DR language (conventions A.11) applied: continuity-framed, non-moral. The exploration and the demonstration contract live outside Part 2 (`alpha/thinking/corroboration-and-quantified-trust.md` and backlog EXP-C1).

**§7.3.3, done.** Added the corroboration-dials paragraph immediately after the load-bearing-caveat paragraph. It states that what remains of completeness-ahead once the §7.4.3 stamp closes the behind-via-traffic case is not an undesigned mechanism but a family of Group-governed settings, changed under §7.2 R7: (i) which act classes require final state versus proceed on best-known (irreversible enforcement always final, reads never, the boundary per-Group); (ii) the freshness threshold k itself, including its formula-valued form (§7.4.1); and (iii) the solicitation posture, the read-side frontier ask whose answer is always an assertion taken at quantified trust. A tight Group dials k high and enforcement slow; a loose Group dials it low and accepts more delay exposure, the same temperament spectrum as the §7.6 configuration posture, safe at every setting because the fail-closed rule is not a dial. `Design`, decided as the framing.

**§7.4.1, done.** Added a final paragraph admitting a formula-valued freshness threshold: a Group MAY set k as a formula over folded state (proportional to folded member count at the act's position, or weighted by folded Group Roles) rather than a constant, provided every input is folded fact and never an asserted or locally observed quantity, so the resulting k stays deterministic and identical on every honest node. Changes under R7 like any rule; introduces no new trust surface. It moves the dial, not the machinery. `Design`.

**Map, done.** Updated the §0 map §7.3 line (§7.3.3 now names the corroboration dials) and the §7.4 line (§7.4.1 now also admits the formula-valued freshness threshold k) per Rule 15.

**Verification:** no em-dashes or double-hyphens in this changelog prose; the Part 2 inserts use house em-dash style; MUST/MAY casing preserved; no mechanism or code identifier changed; cross-references (§7.2 R7, §7.3.3, §7.4, §7.4.1, §7.4.3, §7.6) resolve.

---

## Two-competing-quorums fold behavior recorded as test-run (§7.3.2, §7.2 R7 residual)

`Status: complete (RUN-05, 2026-07-14)`

Scope: a consistency-pass staleness fix, not a new decision. RUN-03 Phase B built and test-ran the competing-RuleChange contradiction predicate (`fold_derived::detect_competing_rulechange`; register row `competing-quorum-autoresolve` moved Active to Reconciled), but two Part 2 passages still said the fold's behavior in this case carried no evidence tag until the two-competing-quorums experiment runs. Both are corrected to record the landed evidence. No new mechanism, no code change.

**§7.3.2, done.** The F8 boundary paragraph's closing clause is updated: the two concurrent quorum-met RuleChanges case is now test-run, the fold hard-stops with the order-independent `contradiction:{byte-head}` and the rule retains its pre-conflict value (RED to GREEN, `two_competing_rulechange_quorums`, RUN-03). The passage moves from decided-but-untested to `Modeled`.

**§7.2, done.** The R7 residuals paragraph's second residual (two concurrent same-rule quorum-met RuleChanges) receives the same upgrade, keeping the first residual (the role-authorship gate) untouched. `Modeled`.

**Verification:** no em-dashes or double-hyphens in this changelog prose; the Part 2 inserts use house em-dash style; no code identifier or mechanism changed; cross-references (§7.2 R7, §7.3.2, §7.6) resolve. Evidence: RUN-03 Phase B, `SPEC-DIVERGENCE-REGISTER.md` (Reconciled row), `RUN-03-SUMMARY.md`.

## Fan-out grade landed and re-plant preservation-banner tags normalized (§11.11, §7.6.11)

`Status: complete (RUN-06, 2026-07-14)`

Scope: two findings-settlement edits from RUN-05's register (FND-2, FND-5). No new mechanism, no code change.

**§11.11, done (FND-2).** Measurement #1's partial-measurement annotation is updated to record the landed fan-out evidence. RUN-01 EXP-1 measured fan-out over real iroh-gossip on a loopback testbed at N of 2/4/8/16 (`croft-chat/FANOUT-M1.md`): per-node gossip cost linear in the live set (2N+1), aggregate order N-squared, head convergence at every N. The grade moves from half-earned to earned in shape (both halves), magnitude-open at scale, carrying two named boundaries: the figures are loopback, not representative hardware at hot-N of 500-plus (register row `fanout-single-run`), and the connect-time resync cost is super-linear on the bootstrap hub. This lands the follow-on edit staged as F4 in `proposed-changes-2026-07-experiment-reconciliation.md`.

**§7.6.11, done (FND-5).** The re-plant subsection's preserved status banner is normalized to the canonical A.9 ladder: the lowercase `design` tag becomes `Design` and `[confirm before publish]` becomes `[confirm]` (the same B.6 mappings applied across Part 2 in the unification pass). The two cited source paths (`12-replant-experiments.md`, `mls-hardcases-and-posture.md`) get the leading `../` so they resolve from the spec directory. The block stays a preserved historical fold; only its tags and paths are regularized.

**Verification:** no em-dashes or double-hyphens in this changelog prose; the Part 2 inserts use house em-dash style; no code identifier or mechanism changed; cross-references (§6.8.1, §7.2 R7, §7.4.3) resolve. Evidence: RUN-01 EXP-1 (`croft-chat/FANOUT-M1.md`), F4 in the proposed-changes doc, conventions A.9/B.6.

## Single-status-tag polish and the FND-1 range cite (§7.3.2, §8.2(e))

`Status: complete (RUN-07, 2026-07-14)`

Scope: two riders, no new mechanism and no code change. Both are consistency-pass polish carried on the RUN-07 code run.

**§7.3.2, done (T11).** The F8 boundary paragraph's closing sentence carried two status tags, an inline `` `Design` `` and the terminal `` `Modeled.` ``, which double-tagged one sentence. The inline `` `Design`, decided and now test-run: `` opener becomes prose, `Decided by design and now test-run:`, so the sentence carries exactly one status tag, the terminal `` `Modeled.` `` that records the landed test evidence. No change to the decision or the evidence, only the tag count.

**§8.2(e), done (FND-1).** The originating-op freshness-precondition citation is widened from the point cite `(§7.4.2)` to the range cite `(§7.4–§7.4.2)`, so the living-doc reference names the whole §7.4 freshness plus §7.4.2 recovery cluster rather than the recovery subsection alone. The same one-site widening is applied to the EXPERIMENT-BACKLOG.md quote of that clause. The two §7.4.2 cites in the §7.4.2-hazards table rows are correct on their own terms and are left untouched. This refines the RUN-06 FND-1 owner call (accept §7.4.2 as shorthand) to the explicit range in the living docs; the refinement is recorded in the CONSISTENCY-FINDINGS-2026-07.md settlement addendum.

**Verification:** no em-dashes or double-hyphens in this changelog prose; the Part 2 inserts use house em-dash style; no code identifier or mechanism changed; cross-references (§7.2 R7, §7.3.2, §7.4, §7.4.2) resolve. Evidence: RUN-03 Phase B (`two_competing_rulechange_quorums`), CONSISTENCY-FINDINGS-2026-07.md addendum (RUN-07).

## R7 content-bound-quorum count upgraded to Verified by the automated cross-package sweep (§7.2, §8.2(e))

`Status: complete (RUN-07, 2026-07-15)`

Scope: an evidence-tag upgrade earned by the RUN-07 X3 automated cross-package mutation sweep. No mechanism change and no code change; the sweep is a test-driver harness that patches and reverts in place.

**§7.2 R7, done.** R7's evidence tag moves from `Modeled` to `Verified` for the content-bound-quorum count. The RUN-07 automated harness re-ran the current-code substrate sweep (61 survivors) and drove each survivor through the croft-chat consumer suite: all 61 resolve mechanically as 7 killed and 54 individually justified. The 7 killed are exactly the count-enforcement path on the live fold: the approval subject (`rule_change_approval_subject`), the approval-subject resolution (`act_subject`), the rule-key decode, and the membership-admin gate that carries the count, on top of the already mutation-clean `governance.rs` threshold counting. The evidence line now cites the automated sweep instead of the prior pending-X3 note. The tag is scoped to the count: the role-authorship gate stays the open residual named in the same subsection (the role model, not the count), which the sweep quantifies as unpinned by the consumer suite.

**§8.2(e), done.** The R7 clause is updated to record that policy-change quorum enforcement is now cross-package mutation-Verified for the count, alongside the existing RED to GREEN note.

**Verification:** no em-dashes or double-hyphens in this changelog prose; the Part 2 inserts use house em-dash style; no code identifier or mechanism changed; cross-references (§7.2 R7, §8.2) resolve. Evidence: `alpha/experiments/local_storage_projection/X3-AUTOMATED-SWEEP.md`, `x3-sweep-data/cross-package-run07.json`, `SPEC-DIVERGENCE-REGISTER.md` (mutation-gate note + `fold-auth-duplicate` row), `MASTER-INDEX.md` A2.

## Origination freshness precondition exercised at loopback grade (§8.2(e))

`Status: complete (RUN-07, 2026-07-15)`

Scope: one honesty-boundary refinement earned by EXP-C1 (backlog §2c). No mechanism change and no spec status change; the completeness-ahead contract is demonstrated at loopback / fold grade only.

**§8.2(e), done.** The clause on the origination freshness precondition is refined from "not yet exercised over live transport" to "exercised at loopback grade (EXP-C1, RUN-07) but not yet over live transport (real-NAT path remains X1)". EXP-C1 landed the completeness-ahead contract's four assertions at loopback / fold grade (`croft-chat/tests/completeness_ahead.rs` + `local_storage_projection::completeness_ahead`): stall-at-threshold below freshness k with reads unaffected, generation-stamp gap detection-and-fill, solicitation reach for the unreferenced tail folding to the identical fingerprint, and the formula-valued k = ceil(n/2) identical across arrival orders. This records the loopback exercise without upgrading any status: the relay/real-NAT path stays X1.

**Verification:** no em-dashes or double-hyphens in this changelog prose; the Part 2 insert uses house em-dash style; no code identifier or mechanism changed; cross-references (§7.4, §8.2, X1) resolve. Evidence: EXP-C1 (`croft-chat/tests/completeness_ahead.rs`, 4 tests), backlog §2c (RUN-07 done).

## Conformance-vector footnote reconciled to the folded conformance-core (§10.5)

`Status: complete (RUN-08, 2026-07-15)`

Scope: one conditional annotation earned by RUN-08 Part 1B. No mechanism change and no code change in Part 2; the edit reconciles the §10.5 conformance-vector footnote to the ground truth after the Proofs corpus was folded into discovery, and records the key-distribution over-the-wire piece as green-real at loopback grade. The threshold-revoke over-the-wire piece is untouched (the RUN-08 firewall).

**§10.5, done.** The conformance-vector footnote is updated from its 2026-07 wording. The reference conformance-core is now folded into `discovery/alpha/Proofs/lineage-groups/crates/conformance` and re-proves 66/0 across categories 1 through 9 in-environment (cat 7 adversarial in real Rust, cats 8 and 9 TS-authoritative, and the cat-5b revoke-authority mechanism), so the earlier "categories 7/8/9 not yet emitted" reading is superseded for the vectors themselves. The footnote now names the true residual as the over-the-wire sourcing, split into the two pieces the footnote already named: (a) MLS key-distribution over the wire is now Verified green-real at loopback grade (a real openmls Welcome over a real iroh connection distributes the group key and the lineage-folded membership and standing; `alpha/experiments/iroh/crates/mls-welcome-over-iroh`, reproduced at `relay-lab-runs/C-mls-welcome-2026-07-15-run08`), with wiring that source into the emitter itself the residual and the real-NAT path staying X1; and (b) threshold-revoke as real k-of-n over the wire plus the co-sign-versus-vote authority ordering stays gated on the revocation-authority trust model (the I9 identity and key-recovery tier).

**Verification:** no em-dashes or double-hyphens in this changelog prose; the Part 2 insert uses house em-dash style; no code identifier or mechanism changed; cross-references (§9, §10.5, §8.2, X1, I9) resolve. Evidence: `mls-welcome-over-iroh` reproduced RUN-08 (`relay-lab-runs/C-mls-welcome-2026-07-15-run08/verdict.json`), conformance `run-vectors` 66/0 (`.../conformance-suite-reprove.txt`), F7 (proposed-changes, RUN-08 update).

## Evidence map pointer added to the honesty-boundaries preamble (§8.2)

`Status: complete (RUN-08, 2026-07-15)`

Scope: one pointer sentence, no mechanism change and no status change. The RUN-08 traceability pass built a per-claim evidence index (`EVIDENCE-MAP.md`) and this records where it lives.

**§8.2, done.** The honesty-boundaries preamble gains a closing sentence pointing to `EVIDENCE-MAP.md`, the regenerable per-claim index of where each status tag at or above Modeled resolves to its evidence (named tests, report files, RUN numbers, environment bounds). The sentence states the index-not-source rule: the map is an index only, and the tagged sentence in Part 2 remains authoritative wherever the two differ. No tag moved.

**Verification:** no em-dashes or double-hyphens in this changelog prose; the Part 2 insert uses house em-dash style; no code identifier or mechanism changed; the referenced file `EVIDENCE-MAP.md` exists in this directory. Evidence: RUN-08 Part 2 traceability pass.

## Off-ladder token de-backticked in the length-extension note (§10.4)

`Status: complete (RUN-09, 2026-07-15)`

Scope: one meaning-preserving edit, no mechanism change and no status-tag move. Settles the RUN-08 traceability finding FND-T5 (owner-confirmed for settlement 2026-07-15).

**§10.4, done.** In the BLAKE3 hash note, the off-ladder token `` `Reviewer-judgment` `` is de-backticked into plain prose: "the construction-level check rests on reviewer judgment, to be formalized as a per-context note at freeze." The A.9 ladder gains no eleventh rung; the backticked form had read as a status tag, and the plain prose no longer does. The `` `Verified` `` tag on the BLAKE3 length-extension property is unchanged.

**Verification:** no em-dashes or double-hyphens in this changelog prose; the Part 2 edit removes backticks only and preserves the sentence meaning; no code identifier or mechanism changed; §10.4 and the A.9 ladder resolve. Evidence: FND-T5 settlement, `CONSISTENCY-FINDINGS-2026-07.md` `## Settlement (RUN-09, 2026-07-15)`.

## Re-plant message-continuity half recorded at loopback grade (§7.6.2)

`Status: complete (RUN-09, 2026-07-15)`

Scope: one conditional status record earned by RUN-09 Part 3 (the B1 → A5 build). The membership half of the re-plant is already `Verified`; this records the message-continuity half at `Modeled`/loopback grade. No mechanism change; the membership tag is untouched.

**§7.6.2, done.** The re-plant sentence previously recorded the message-continuity half as not yet built, needing the dataplane hash structures (Appendix B / B1). It now records that half as `Modeled` at loopback grade: a B1 dataplane hash structure (content-addressed, causally-linked records folding into an arrival-order-independent digest, `replant-continuity/src/dataplane.rs`) carries the conversation across the repoint so that every pre-repoint entry is present after it exactly once, in-flight entries land exactly once in causal order on the post-repoint group, both members converge byte-identically across arrival orders, and an injected duplicate or dropped frame is detected rather than absorbed (RED→GREEN, `e12_2_message_continuity.rs`, 5 tests, RUN-09). It is `Modeled` rather than `Verified` because delivery is harness-driven over the real re-plant membership, not real transport; real over-the-wire delivery and the `[gates-release]` B1 record encoding remain open.

**Verification:** no em-dashes or double-hyphens in this changelog prose; the Part 2 insert uses house em-dash style; the membership-half `Verified` tag is unchanged and no other tag moved; cross-references (§7.6.2, Appendix B / B1) resolve; the evidence parenthetical follows the A.9 recommended forward form (FND-T4, RUN-09). Evidence: `alpha/experiments/replant-continuity` (`dataplane.rs` + `tests/e12_2_message_continuity.rs`), RUN-09 Part 3.

## Steady-state anti-entropy recorded at loopback grade (§6.8.1)

`Status: complete (RUN-09, 2026-07-15)`

Scope: one conditional status record earned by RUN-09 Part 4. Connect-time catch-up was already recorded; this records the other §6.8.1 half, steady-state anti-entropy, at `Modeled`/loopback grade. No mechanism change to any existing claim.

**§6.8.1, done.** The sentence previously read that steady-state anti-entropy (a live frame lost to an existing neighbor, no new join) is not yet exercised. It now records that half as `Modeled` at loopback grade: a range-summary compare over the `(device, lamport)` key space detects the gap (invisible to live delivery, since gossip carries no per-recipient ack and the lagging peer buffers no stranded successor) and a diff-only repair re-converges the folds byte-identically with no reconnect and no whole-log re-broadcast (RED→GREEN, `steady_state_anti_entropy.rs` + `croft-chat` `anti_entropy` module, RUN-09). It is `Modeled` rather than `Verified` because it runs at loopback over the whole-set range compare in its simplest form; the range-partitioned production construction (Willow 3d-range versus Negentropy) and real-transport loss remain open (§5 / Appendix B).

**Verification:** no em-dashes or double-hyphens in this changelog prose; the Part 2 insert uses house em-dash style; the connect-time catch-up record and the RBSR scaling-shape `Verified` tag are unchanged and no other tag moved; cross-references (§6.8.1, §5, Appendix B) resolve; the evidence parenthetical follows the A.9 recommended forward form (FND-T4, RUN-09). Evidence: `croft-chat/croft-chat/src/anti_entropy.rs` + `tests/steady_state_anti_entropy.rs`, RUN-09 Part 4.

## Fan-out magnitude replicated; `fanout-single-run` retired (§11.11 #1)

`Status: complete (RUN-09, 2026-07-15)`

Scope: one caveat-clause update on §11.11 measurement #1, earned by RUN-09 Part 5 (the repeated-run arm). No mechanism change; the `Measured` tag is unchanged.

**§11.11, done.** Measurement #1's fan-out caveat is updated to record the replicated evidence. RUN-09 Part 5 re-ran the FANOUT-M1 sweep K of 5 at N of 2/4/8/16 on the same loopback harness (`croft-chat/FANOUT-M1.md` addendum, `fanout-data/repeated-run09.csv`): per-node cost 2N+1 reproduced exactly with zero variance, head convergence held in every run at every N, and the super-linear connect-time hub-resync shape reproduced with a tight magnitude band (N of 16 spanned 349 to 422, median 401, refining the single-run 479 downward). The spread is narrow and supports the recorded magnitudes, so the `fanout-single-run` register row is retired (Reconciled, RUN-09). The clause now records the replicated band; the remaining boundaries are hardware hot-N of 500-plus and the star-bootstrap topology-sensitivity of the resync magnitude (a narrow note, not a divergence).

**Verification:** no em-dashes or double-hyphens in this changelog prose; the Part 2 insert uses house em-dash style; the `Measured` tag and the per-commit band are unchanged and no other tag moved; cross-references (§11.11, §11.4, §11.5, §6.8.1) resolve. Evidence: `croft-chat/FANOUT-M1.md` repeated-run addendum + `fanout-data/repeated-run09.csv` + `scripts/fanout-repeated.sh`, RUN-09 Part 5.

## §5.10 group-principal key-rotation seam reframed per the seam brief (FND-R10-1)

`Status: complete (RUN-11, 2026-07-16)`

Scope: one framing correction to §5.10, applying FND-R10-1 (RUN-10). The seam previously framed an unworked "key rotation scheme" for a communal-namespace key; the group-principal seam brief established that a communal namespace has no shared whole-namespace secret to rotate. A Design-grade reframe; no experiment tag moved (the construction stays `Design`).

**§5.10, done.** The "what is unworked is the key rotation scheme, how the key rotates under churn" sentence is reframed: a communal Meadowcap namespace has no shared whole-namespace secret to rotate (authority derives from per-subspace key pairs, and the namespace-keypair secret is exercised only in an owned namespace), so the question decomposes into per-subspace write authority (each persona rotates only its own lineage key, §4.5) and the fold-gated asset key (§5.11, re-wrapped to the current Role-holders by the governance fold under churn), leaving only a near-free identifier assignment plus the primary-versus-secondary choice (the brief recommends primary). The seam brief is cited (`beta/impl/drystone-design/group-principal-seam.md`) and the sentence carries a `Design` tag. The open primary-versus-secondary question is preserved, not dropped.

**Verification:** no em-dashes or double-hyphens in this changelog prose; the Part 2 insert uses house em-dash style; cross-references (§4.5, §5.11, and the seam-brief path) resolve on the site build; no status tag at or above `Modeled` moved. Evidence: `beta/impl/drystone-design/group-principal-seam.md` (RUN-10 Part 2), FND-R10-1, RUN-11 Part 1 rider 4.

## Evidence parentheticals standardized in three governance claims (FND-T4)

`Status: complete (RUN-11, 2026-07-16)`

Scope: the FND-T4 reshape, applied narrowly. The standardized `(evidence: test-or-report, RUN-NN, grade)` parenthetical is fitted onto existing inline evidence prose in exactly the claims where every component already exists, with no information added or dropped. Three of the four candidate claims qualified; the fourth is recorded as a FINDING.

**§7.2 R7, §7.3.2, §8.2(e), done.** The content-bound-quorum count claim (§7.2 R7), the competing-quorums hard-stop (§7.3.2), and the policy-change enforcement clause (§8.2(e)) each already named their test, RUN, and grade in loose prose; that prose is reshaped into the standard parenthetical form (§7.2 R7: `rulechange_threshold_enforced.rs`, `rulechange_quorum_via_api.rs`, and the X3 sweep, RUN-07, `Verified`; §7.3.2: `two_competing_rulechange_quorums`, RUN-03, `Modeled`; §8.2(e): the R7 count tests plus the X3 sweep, RUN-07, `Verified`). No pointer, grade, or descriptive detail was added or removed. The fourth candidate, the §7.6.2 membership half, is NOT reshaped: its RUN-NN component is missing (the membership half was imported as already-`Verified` from the standalone experiments corpus and carries no discovery-RUN stamp; EVIDENCE-MAP row 52 lists the test but no RUN), so per FND-T4's own rule it is a FINDING rather than a reshape.

**Verification:** no em-dashes or double-hyphens in this changelog prose; the Part 2 inserts use house em-dash style; every named test resolves on grep; no status tag moved (the reshape is form-only). Evidence: FND-T4 (RUN-08), owner-confirmed for narrow application RUN-11; RUN-11 Part 1 rider 3.

## §7.6 back-Map entry updated to the landed message-continuity grade (§0 Map)

`Status: complete (RUN-11, 2026-07-16)`

Scope: one wording update in Part 2's §0 back Map, an audit catch (RUN-11 rider 8). RUN-09 landed the re-plant message-continuity half at `Modeled`/loopback, but the back-Map §7.6.2 clause still read "message-continuity half open". No mechanism change; no tag moved (the Map mirrors the already-recorded §7.6.2 body grade).

**§0 Map (§7.6 entry), done.** The §7.6.2 clause in the back Map read "re-plant's membership-continuity half corroborated, message-continuity half open"; it now reads "message-continuity half `Modeled` at loopback grade (RUN-09)", matching the §7.6.2 body sentence and the EVIDENCE-MAP row (both landed RUN-09). The audit also checked for a duplicated §7.6 Map line (flagged in the rider): none is present in the current tree, a single §7.6 entry stands, so no line was removed.

**Verification:** the site build (broken-ref gate plus the emitted-HTML anchor audit) stays clean; no em-dashes or double-hyphens in this changelog prose; backticked `Modeled` renders as content, never a broken reference; cross-references resolve. Evidence: RUN-11 Part 1 rider 8.

## §7.6.2 membership-half evidence parenthetical completed on a RUN-11 re-proof (FND-T4 follow-on)

`Status: complete (RUN-11 follow-on, 2026-07-16)`

Scope: closes the one FND-T4 claim that was FINDING-stopped in the RUN-11 Part 1 pass. The §7.6.2 membership half was `Verified` but carried no discovery-RUN stamp (imported as already-`Verified` from the standalone experiments corpus), so its RUN-NN component was missing and the reshape could not be applied without inventing a pointer. Rather than invent one, the E12.7 membership-continuity tests were re-proven in-environment to earn a genuine stamp.

**§7.6.2, done.** The E12.7 keystone tests re-proved 3/3 green on real openmls 0.8.1 in this environment (RUN-11 re-proof): `e12_7_1_stamp_tracks_derivation`, `e12_7_2_removal_propagates`, `e12_7_3_unauthorized_no_drift`. The membership-half sentence now carries the standard parenthetical (evidence: `e12_7_1/2/3_*.rs`, RUN-11 re-proof, `Verified`), matching the message-half form landed RUN-09. No status tag moved (the membership half was and stays `Verified`); the reshape adds only the now-earned evidence pointer. EVIDENCE-MAP row updated with the re-proof stamp.

**Verification:** no em-dashes or double-hyphens in this changelog prose; the Part 2 insert uses house em-dash style; the re-proof is a re-run of the existing tests (no new production code, no mechanism change); cross-references resolve on the site build. Evidence: `alpha/experiments/replant-continuity/tests/e12_7_*` (RUN-11 re-proof), FND-T4.

## §7.6.2 membership-half evidence reshaped to import provenance (RUN-12 A.9 ruling)

`Status: complete (RUN-12, 2026-07-16)`

Scope: the RUN-12 Part 1 import-provenance ruling, applied to the one §7.6.2 evidence parenthetical that a RUN-11 re-proof had stood in for. The ruling adds an A.9 rider: for evidence imported from outside the numbered-run system, the standard parenthetical's `RUN-NN` slot carries import provenance (`imported: <corpus> @ <commit>`) instead, and a retroactive RUN number is never invented. No mechanism change; no status tag moved (the membership half was and stays `Verified`).

**§7.6.2, done.** The membership-half sentence read `(evidence: e12_7_1/2/3_*.rs — E12.7 stamp-tracks-derivation, removal-propagates, unauthorized-no-drift — RUN-11 re-proof, `Verified`)`; it now reads `(evidence: the e12_7_* tests, imported: replant-continuity @ `d52ed6f`, `Verified`)`. The `e12_7_*` tests were imported already-`Verified` from the standalone experiments corpus and carry no discovery-RUN stamp; import provenance names the exact tree where they live and pass, which the RUN-11 re-proof stamp only approximated. The RUN-11 §7.6.2 FND-T4 FINDING is thereby closed on the cleaner record. EVIDENCE-MAP row 52 updated with the same import provenance.

**Verification:** no em-dashes or double-hyphens in this changelog prose; the Part 2 insert uses house em-dash style; the reshape is form-only (no pointer, grade, or descriptive claim added or removed beyond swapping the RUN-11-re-proof note for import provenance); cross-references resolve on the site build; the broken-ref gate plus the emitted-HTML anchor audit stay clean, and the new `site/build.py` companion allowlist (7 entries) passes. Evidence: `alpha/experiments/replant-continuity/tests/e12_7_*` (imported: replant-continuity @ `d52ed6f`), A.9 import-provenance rider (RUN-12), FND-T4.

## §7.6.2 message-continuity half re-asserted over real transport (RUN-12 Part 2, §2f)

`Status: complete (RUN-12, 2026-07-16)`

Scope: the four re-plant message-continuity claims, previously harness-delivered (RUN-09), are now re-asserted over real iroh-gossip at loopback. No mechanism change; the grade **stays `Modeled`** (A.9 when-in-doubt), so no tag moved — only the rationale is updated to reflect real-transport delivery.

**§7.6.2 message half, done.** The sentence read that the half is `Modeled` "because the records are driven over the real re-plant membership but delivered by the harness, not real transport"; it now records that the four claims (pre-repoint exactly-once, in-flight causal order, cross-order byte-identical convergence, injected dup/drop detected-not-absorbed) are re-asserted over **real iroh-gossip at loopback** (`RelayChoice::LocalDirect`): node A publishes the B1 records inside the gossip frame payloads and node B drains-and-folds them into a `History`, the harness injecting only the dup/drop fault. It stays `Modeled` rather than `Verified` because the record serialization is test-only (the `[gates-release]` B1 record encoding, Appendix B / B1, is unpinned) and delivery is loopback, with real-NAT the X1 residual. The §0 back-Map §7.6.2 clause and the EVIDENCE-MAP §7.6.2 message-half row carry the same update.

**Verification:** no em-dashes or double-hyphens in this changelog prose; the Part 2 inserts use house em-dash style; the grade is unchanged (the edit is rationale + evidence-pointer only); cross-references resolve on the site build; the broken-ref gate plus the emitted-HTML anchor audit stay clean. Evidence: `alpha/experiments/croft-chat/croft-chat/tests/iroh_message_continuity.rs` (RUN-12 Part 2, 2 tests green, RED→GREEN via a no-bootstrap probe), §2f, A.9.
