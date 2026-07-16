# Drystone: The Group-Principal Seam — can a Meadowcap communal namespace carry the Group principal?

`Status: read-and-report design brief (RUN-10 Part 2). Verdict and construction proposed; the Meadowcap/Willow primitive facts are checked against the live specifications this pass; the Drystone bindings are Design, not frozen. No spec, register, or crate is edited by this document.`

`Serves: Part 2 §5.2 / §5.10 group-principal open seam — reports whether Meadowcap can carry the Group principal and recommends a construction. Narrows the §5.2 "open seam: the principal that anchors a multi-client lineage" and the first of §5.10's "two seams left open" (identity construction and key rotation of the Group-principal), leaving §5.10's second seam (cross-Group grants, [gates-release]) untouched.`

`Scope: whether Meadowcap's communal-namespace construction can carry the Group principal as §5.2/§5.10 sketch it; how capability (re-)issuance composes with MLS epoch rotation under churn; the exact points of impedance between Meadowcap identifiers/delegation/revocation and the Drystone principal/persona/lineage model and its §7.2 R1–R7 interface; and a recommended construction mapped onto §5.10's primary-vs-secondary communal-namespace question. Sits beneath Part 2 §5.10 (the Group-principal) and §7.2 (the grant interface), and above asset-keying.md, which already resolves the read-scoped half of the §5.10 seam.`

`Companion to: asset-keying.md, which this brief builds on and does not contradict — asset-keying.md §D resolves the read-scoped communal-namespace key construction (the fold-gated asset key wrapped to a Role's folded set) and this brief resolves the write-authority/identity-construction half above it and the Meadowcap composition check its Open items flagged as "the decisive check." Also companion to authority-and-complement.md (the authority-versus-capability line this brief keeps) and social-mapping.md (the per-author-union content-plane mapping). Grounds against ../../drystone-spec/part-2-certifiable-design.md (§5.2, §5.5, §5.10, §7.1, §7.2, §10.2, §10.4) and Part 1 §2.0.1/§2.3. Vocabulary inherits from the conventions-and-decisions reference.`

This is a read-and-report brief for one open seam: whether the Group-principal (§5.2, §5.10) can be realized as a Meadowcap communal namespace, and what construction to recommend. Normative keywords are **MUST / SHOULD / MAY** (BCP 14) where a claim states a requirement. Each load-bearing claim carries a status flag: `Established` (an inherited primitive or a fact checked against a primary this pass), `[confirm]` (load-bearing and rests on an external fact or version not independently re-verified beyond the single live fetch below), `Design` (this brief's own binding proposal, to be judged as reasoning), `Synthesis` (assembled across several sources), `[gates-release]` (a wire/byte choice deferred). A claim tagged against the live Meadowcap or Willow spec is `Established` for the primitive and `[confirm]` where it turns on the exact version.

> **I9 firewall.** This is a read. It decides no trust tier and freezes nothing. Two decisions it surfaces are the owner's, not this brief's: (1) whether a Group **MAY** carve an *owned* sub-namespace for single-author content inside its communal Group-principal namespace (§5.10 permits it in principle; whether a given Group allows it is governance policy), and (2) whether the recommended primary-namespace stance is adopted for the whole-Group key or left to Appendix B. Both are flagged `owner-decides` where they arise.

---

## A. What is inherited, stated once

`Realizes: P-Peer-Equality`

The pieces this brief composes are already at Part 2's maturity and are not re-argued here:

- **Group-principal above MLS** (§5.10): the MLS group is the communication-and-safety substrate (epoch keys, FS, PCS); the Group-principal is an application-layer identity, "a Meadowcap **communal namespace** (§7.1), that *corresponds to* an MLS group for communication but is **not defined by it**." `Established` (Part 2 §5.10).
- **Communal, not owned** (§5.10): Drystone's Group-principal is communal by default because communal authority *is* `P-Peer-Equality` at the data layer; owned is the apex it rejects for governance, legitimate only for a single author owning a sub-namespace of their own content. `Established` (Part 2 §5.10).
- **The read-scoped half is resolved** (asset-keying.md §D, Part 2 §5.11): read confidentiality on durable content is the fold-gated **asset key**, a symmetric key wrapped to each current Role-holder, rotated on removal with fresh entropy, provisioned downstream of the governance fold, never triggered by an MLS epoch tick. This brief does not re-open it; it sits above it. `Established` (asset-keying.md, Part 2 §5.11).
- **The grant interface** (§7.2 R1–R7): Group Roles and capabilities share one mechanism-neutral interface — unforgeable (R1), attenuating (R2 = Meadowcap confinement), convergently revocable as a governance fact (R3), bounded stale-authority in epoch/generation not wall-clock (R4), forward-read-excluding (R5), attributable (R6), content-bound quorum for policy (R7). `Established` (Part 2 §7.2).

---

## B. The Meadowcap / Willow primitive facts, against the live specification

`Realizes: P-Knowable-Truth`

Ground truth for the composition check, fetched live this pass. **Meadowcap specification, labelled "Final (as of 21.11.2025)", willowprotocol.org/specs/meadowcap; Willow Data Model, labelled "Final", willowprotocol.org/specs/data-model; both fetched as of 2026-07-15.** `[confirm]` on the exact version labels (single live fetch, not independently corroborated); `Established` for the primitive semantics, which also match the verbatim quotes Part 2 §5.5/§5.10 already carry.

- **Capability.** "an unforgeable token that bestows read or write access for some data to a particular person, issued by the owner of that data." A capability answers recipient, access type (read/write), which entries, and validity. `Established`.
- **Communal namespace.** Authority "is derived from ownership of a given subspace key pair," horizontally: each subspace is "owned by a particular author," `SubspaceId`s are public keys, and authority derives from "valid signatures (which requires the corresponding secret key)." No one holds authority over the whole namespace. `Established`.
- **Owned namespace.** Authority derives from ownership of the *namespace* keypair; the creator "is the owner of all its data," and "peers reject all requests unless they involve a signature from the namespace keypair," including data written by delegatees. `Established`.
- **Delegation / attenuation (confinement).** "A capability bestows not only access rights but also the ability to mint new capabilities." Restrictions narrow a capability "by subspace_id, by path, and/or by timestamp." A delegation carries a `(Area, UserPublicKey, UserSignature)` tuple; validity is "defined based on the number of delegations," verified recursively: each step requires the prior capability valid, the new area **included in** the previous granted area, and a valid signature from the previous receiver. This is R2 attenuation exactly. `Established`.
- **Revocation.** **No built-in revocation mechanism exists in Meadowcap itself.** The only described workaround is owned-namespace-specific: the namespace owner writes future-timestamped entries that overwrite a delegatee's entries — "a semantic overwrite strategy, not a revocation primitive." Capability validity, once issued, is a signature fact that does not expire. `Established` (this confirms Part 2 §10.4's "differ only on revocation immediacy" framing and asset-keying.md's Open item).
- **Data model.** An `Entry` is `(namespace_id, subspace_id, path, timestamp, payload_length, payload_digest)`. `NamespaceId` and `SubspaceId` are **parameterized types**, not a fixed format: "Should namespaces be identified via human-readable strings, or via the public keys of some digital signature scheme? That depends entirely on the use-case." Write permission is proved by an `AuthorisationToken` checked by `is_authorised_write`; Meadowcap is one such token scheme. `Established`.

The load-bearing consequence, drawn out because §5.10's framing turns on it: **in a communal namespace there is no namespace secret key in use.** The `NamespaceId` is a public identifier (the spec leaves even its *type* free); write authority lives entirely in the per-subspace keypairs, and the namespace-keypair secret that an *owned* namespace uses is simply never exercised communally. `Established` (from the two spec quotes above). This is the pivot for Finding F1.

---

## C. Q1 — Can a communal namespace carry the Group principal?

`Realizes: P-Peer-Equality, P-Durable-Enablement`

**Verdict: yes, and it is the natural fit — with one reframing.** A Meadowcap communal namespace carries the Group principal as §5.2/§5.10 sketch it: the Group is the namespace, each member persona is an author owning its own subspace (write into your own subspace, no apex), and the artifact "lives in the Group's communal namespace" owned collectively while each contributor owns its subspace individually (§5.10 worked-fork mechanism). Every property §5.10 leans on holds against the live spec: horizontal authority (communal), no center to sever at a fork (authority distributed across subspaces all along), and both descendants carrying the whole artifact (the namespace is the shared object). `Design` for the binding; `Established` for the Meadowcap semantics it rests on.

The reframing is Finding F1: the seam asks how "the communal-namespace **key** ... rotates under churn," but the communal model has **no shared namespace secret to rotate.** What actually exists is (a) a static namespace *identifier*, (b) per-subspace author keypairs (write authority, rotated only when a persona rotates its own lineage key, §4.5), and (c) the read-scoped **asset key** (confidentiality, rotated by asset-keying.md §D under membership change). The "Group-principal key rotation under churn" problem therefore **decomposes into two already-owned problems plus a near-trivial identifier-assignment question**, and no third "group principal secret" needs a rotation scheme. This is the decisive-check answer asset-keying.md's Open items asked for: capability issuance sits cleanly *beneath* key wrapping and does **not** duplicate the authority the governance fold holds, because Meadowcap write authority is per-subspace (self-authorizing) while the fold decides *membership and Role*, and the asset key decides *read* — three non-overlapping planes over one roster. `Synthesis`.

---

## D. Q2 — Capability (re-)issuance vs MLS epoch rotation under churn

`Realizes: P-Knowable-Truth`

The two planes §5.10 insists on staying separate: **MLS = communication/safety substrate** (epoch keys advance on a Commit, i.e. on key-change need — add, remove, or PCS rotation, §10.2 K6); **Meadowcap capability = data-access grant**, a governance fact that folds (§7.2, §7.3). The Group-principal lives *above* MLS (§5.10), and that is exactly what lets capability rotation compose cleanly with epoch rotation: they ride different clocks and rendezvous only at a membership change.

- **Capability grant/revoke is NOT carried by the MLS epoch.** It is a governance fact on the governance chain, ordered by the timestamp-free causal-cryptographic fold (§7.3.1) and stamped with the governance generation counter `n` (asset-keying.md §F), never by the epoch `E`. A post-compromise MLS rotation advances `E` and touches no capability; an authority-only grant folds on the governance chain and advances no epoch. `Design` (consistent with asset-keying.md §H and Part 2 §5.11's "re-key triggered by the fold, never by the epoch").
- **Under churn, "capability rotation" is re-issuance to the folded set plus fold-carried revocation.** Because Meadowcap has no native revocation (§B), a removal cannot void a member's held capability cryptographically. Instead: the removal folds as a governance fact (R3 convergent revocation), honest nodes reject the removed persona's future writes as unauthorized (the same §5.5/§7.3 mechanism that rejects an unauthorized MLS Commit), and the MLS Commit that enforces the removal rotates the epoch key so the removed client stops receiving live traffic (K6). The read-scoped asset key rotates downstream of the fold (asset-keying.md §D). So one membership event produces three coordinated but independently-clocked effects — a governance fact (`n`++), an MLS Commit (`E`++), and an asset-key re-wrap — and they compose because the governance fold is the single upstream cause and the other two are downstream enforcement, never the reverse. `Synthesis`.
- **The stale-authority window is a governance fact, in generations not seconds.** A removed persona's held Meadowcap capability remains signature-valid forever (§B), so R4's bound is supplied entirely by the governance layer: a third party accepting a write under that capability records the governance frontier it had synced (R6), and a later-synced removal makes the acceptance detectable and attributable, bounding stale exposure to "until the revocation fact folds," expressed in governance generation `n`, never wall-clock (R4, Part 1 §2.0.1). `Design`.

The clean statement: **MLS rotates keys, the governance fold rotates authority, and Meadowcap capabilities are re-issued (not revoked-in-place) as a downstream consequence of the fold.** The Group-principal living above MLS is precisely what makes "capability rotation" a fold event rather than an epoch event. `Synthesis`.

---

## E. Q3 — The exact points of impedance

`Realizes: P-Knowable-Truth`

Three impedance surfaces, each a mapping the construction must pin. Named precisely so an implementer can tell a genuine mismatch from a naming difference.

**E.1 Identifier formats (three id namespaces, two mappings).** Three distinct identifier systems meet here:

- **Meadowcap/Willow:** `NamespaceId` (Group-principal identifier) and `SubspaceId` (author identifier) — both parameterized, free to be public keys (§B).
- **Drystone governance:** principal / persona / lineage ids — a persona is *one key-lineage* (§5.2), the unit of standing and weight.
- **MLS:** leaf / signature key / credential — *per client*, a device-level member (§5.2, §10.2 K5).

The two mappings that must be pinned: **(i) subspace ↔ author.** A communal `SubspaceId` should be the **persona lineage identity**, not the per-device client key, so that write attribution is per-persona (matching asset-keying.md §D "each contributor authors into its own subspace signed with its own key" and the §7.2 R6 attributability the read key preserves). **(ii) namespace ↔ Group.** The `NamespaceId` should be the Group's genesis identity, `H(tag ‖ group_id)` (§4.2, §7.3), which already exists and is unconflictable. The impedance is the **granularity split**: MLS counts clients (leaves), Drystone weight counts personae, and Meadowcap subspaces should count personae — so a persona's several MLS leaves must fold to one subspace identity by lineage (§4.5) before a write is attributed. This is the same client→persona fold the governance spine already performs (§5.2); it is inherited, not new, but it MUST be applied at the subspace boundary too. `Design`; the byte encoding of the `SubspaceId`←lineage mapping is `[gates-release]` (Appendix B).

**E.2 Delegation depth (Meadowcap chains vs the attenuating delegate).** Meadowcap delegation is an unbounded, offline-verifiable chain of `(Area, UserPublicKey, UserSignature)` tuples, valid by recursive attenuation (§B) — this *is* §7.2 R2 confinement and §5.5's "delegate only a subset, never a superset," and the alignment is exact and welcome. The impedance is that a Meadowcap chain is **self-certifying and un-revocable**: once signed, every link stays valid, and depth grows without a governance touch. Drystone's delegate (§5.5, §5.5.1) is a *governance fact that folds and is revocable under threshold*. So a Meadowcap delegation carries authority correctly forward but cannot express R3 revocation on its own; the governance fold must sit above the chain, and a delegation that outlives its authorizing Role grant must be caught by the fold, not by the chain. The §5.5.1 courier-vs-agent cut lands here cleanly: a **capability-only** delegation (a read-cap to a search/index helper) can be a bare Meadowcap chain that survives its principal, while an **authority-bearing** delegation (act-for-the-Group, §5.10) must be a folded Group Role, never a raw capability chain. `Design`.

**E.3 Revocation semantics (the central impedance).** Meadowcap has **no native revocation** (§B, live-spec confirmed). Drystone requires R3 convergent revocation and R4 bounded stale-authority, and §5.10's act-for-the-Group Role is explicitly revocable. The impedance is real and is the one the whole construction turns on — but it is **already owned**, not newly discovered: §10.4 states the capability mechanism satisfies R1/R2/R3/R6 via the governance layer and that Track A (Meadowcap) and Track B (Keyhive) "differ only on revocation immediacy," and asset-keying.md flags it as the decisive Meadowcap-composition check. The resolution is that revocation is **never a Meadowcap operation**; it is a governance fact that folds (R3), voids the grant on every honest node, and re-issues capabilities to the folded set, with the R4 window measured in governance generations. The owned-namespace future-timestamp overwrite trick the spec describes is **not** used — it presupposes a namespace owner, which the communal Group-principal deliberately does not have (§B, §5.10). `Established` for the Meadowcap fact; `Design` for the fold-supplies-revocation binding.

---

## F. Q4 — Recommended construction and its costs

`Realizes: P-Peer-Equality, P-Durable-Enablement`

**Recommendation: the Group-principal is a communal namespace at all times (PRIMARY), not established-only-at-fork (secondary).** This answers §5.10's explicit primary-vs-secondary question and aligns with the stance asset-keying.md §D already took ("primary stance: the Group-principal is a communal namespace at all times"). `Design`; `owner-decides` on whether to freeze this now or carry the choice to Appendix B.

The construction, stated as the mapping:

1. **Namespace = the Group, from genesis.** The communal `NamespaceId` is the Group's genesis identity `H(tag ‖ group_id)` (§4.2), allocated at creation and stable across all churn. There is no namespace secret to establish or rotate (§B, F1). `Design`.
2. **Subspace = the persona.** Each member writes into its own subspace keyed by its persona lineage identity (E.1); write authority is the persona's own signature, self-authorizing, needing no grant (communal model). `Design`.
3. **Read confidentiality = the fold-gated asset key** (asset-keying.md §D), rotated on removal, unchanged by this brief. `Established`.
4. **Authority to act-for-the-Group and to issue narrower capabilities = a folded Group Role** (§5.5, §5.10), revocable under threshold (§5.7); revocation and re-issuance are governance facts (E.3). `Design`.
5. **Fork** re-plants the namespace: both descendants carry the whole namespace (all subspaces, all entries) per §5.10 and §7.6, and each descendant takes a new genesis `NamespaceId` via the re-plant family (§10.2.1 Direct: MLS Reinitialization/Subgroup-Branching + resumption PSK), so the two forks are distinct Group-principals sharing history. `Design`.

**Why primary beats secondary, and the costs.** The secondary option — establish the communal namespace only when a fork or merge forces joint ownership to be made explicit — was attractive only under the mistaken premise that a communal namespace carries an expensive rotating key (F1). Once that dissolves, primary is strictly cheaper and safer:

- **Cost of primary (accepted):** every Group carries a communal `NamespaceId` from genesis even if it never forks — a single static identifier, effectively free since `H(tag ‖ group_id)` already exists (§7.3). The per-persona subspace mapping (E.1) must be maintained, but that fold is already run for governance weight. `Synthesis`.
- **Cost of secondary (rejected):** establishing joint ownership *at* the fork introduces a namespace-establishment step that **races the fork** — precisely the concurrency hazard asset-keying.md §D/§G shows eager, act-time keying creates. A fork is the worst moment to run a fresh establishment handshake, because the parties are by definition in a standing contradiction (§7.6). Primary avoids this: the shared object exists before it is contested, so honoring the fork is carriage, not creation. `Synthesis`.
- **Residual owner-decides cost:** primary means the communal namespace is the default *even for content a single author might prefer to own outright*. §5.10 permits an owned sub-namespace for that narrow case; whether a Group allows owned sub-namespaces inside its communal Group-principal is a governance/trust-tier decision this brief does not make (I9). `owner-decides`.

The one-line construction: **a Group is a communal Meadowcap namespace identified by its genesis hash, personae are subspaces, writes are self-authorizing per-subspace signatures, reads are gated by the fold-gated asset key, and authority is a folded revocable Group Role — no group secret, no rotation scheme, no fork-time establishment.** `Design`.

---

## FINDINGS

Labelled for the orchestrator to file to CONSISTENCY-FINDINGS-2026-07.md. Both-ways quotes given.

**FINDING F1 — "the communal-namespace key ... how the key rotates under churn" has no referent in Meadowcap's communal model.** Not a contradiction of a *claim* Part 2 makes (Part 2 correctly leaves the scheme "unworked"), but a framing that presupposes an object the live spec says does not exist communally; recording it so the seam is narrowed rather than restated.

- Part 2 §5.10: *"What is unworked is the **key rotation scheme**, how the Group and its members jointly own the namespace and how the key rotates under churn ..."* and §5.2: *"what is designed-not-frozen is its **key establishment and rotation** under membership change."*
- Live Meadowcap (Final, 21.11.2025): communal-namespace authority *"is derived from ownership of a given subspace key pair"*; each subspace is *"owned by a particular author,"* authority deriving from *"valid signatures (which requires the corresponding secret key)."* The namespace-keypair secret is used only in an *owned* namespace, where *"peers reject all requests unless they involve a signature from the namespace keypair."*
- Reconciliation (this brief, §B/§F): communally there is no shared namespace secret to rotate; "key rotation under churn" decomposes into (a) per-persona subspace-key rotation (§4.5, a persona rotating its own lineage), and (b) the read-scoped asset key (asset-keying.md §D). The seam's remaining content is *identifier assignment* (near-free) plus the already-owned asset-key rotation. Recommend the seam be re-stated in those terms. `Synthesis`.

**FINDING F2 — Meadowcap's only described revocation workaround is structurally unavailable to the communal Group-principal.** Consistent with Part 2's own posture, recorded so no future draft reaches for the overwrite trick.

- Live Meadowcap: *"No built-in revocation mechanism"*; the described workaround is owned-namespace-specific — the namespace owner writes future-timestamped entries that overwrite a delegatee's.
- Part 2 §5.10: the Group-principal is *"communal by default,"* and *"the **owned** model is the apex Drystone rejects for Group governance."*
- Reconciliation: the overwrite workaround presupposes a namespace owner, which the communal Group-principal does not have. Revocation MUST be supplied by the governance fold (R3/R4), never by Meadowcap. This confirms §10.4's "differ only on revocation immediacy" and closes asset-keying.md's decisive-check Open item in the affirmative. `Established`/`Design`.

No finding contradicts asset-keying.md; this brief sits above its read-scoped resolution and adopts it.

---

## Open items and residuals owned

- **The client→subspace lineage fold, at byte level** (E.1). Which key material is the `SubspaceId` and how a persona's several MLS leaves resolve to one subspace is `[gates-release]` (Appendix B), though the fold itself is inherited (§4.5).
- **Owned sub-namespaces inside the communal Group-principal** (§5.10, §F residual). Permitted in principle; per-Group policy is a trust-tier judgment left to the owner (I9). `owner-decides`.
- **Cross-Group grants** (§5.10 second seam) are out of scope here and remain `[gates-release]`.
- **`NamespaceId` type choice.** Willow leaves the type free ("strings ... 256 bit integers, or urls"); this brief recommends the genesis hash `H(tag ‖ group_id)`, an encoding to be pinned with the §4.2/§7.3.1 canonical-bytes work. `[gates-release]`.
- **Version currency.** The Meadowcap "Final (21.11.2025)" and Willow Data Model "Final" labels rest on a single live fetch (2026-07-15) and are `[confirm]`; Willow flags its payload hash (BLAKE3) as "to be replaced by WILLAM3" (§10.4), a moving target worth re-checking at freeze.

---

## Changelog

`Working design brief; per the suite method, transitions are recorded here so the body stays an end state.`

- **First consolidation (RUN-10 Part 2).** Establishes the verdict that a communal Meadowcap namespace carries the Group principal (§C), the capability-vs-epoch composition (§D), the three impedance surfaces (§E: identifiers, delegation depth, revocation), and the recommended primary-namespace construction with costs (§F). Records F1 (no communal namespace secret to rotate) and F2 (the owned-namespace revocation workaround is unavailable communally). Builds on and does not contradict asset-keying.md, whose read-scoped resolution is adopted wholesale and whose "decisive Meadowcap-composition check" Open item is answered: capability issuance sits beneath asset-key wrapping and does not duplicate the fold's authority.
- **Evidence (RUN-11 Part 3).** The §C/§D/§E construction is exercised as a `Design`-grade model in `alpha/experiments/group-principal-seam` (backlog §2e): the genesis-hash namespace (F1, stable across churn), self-authorizing per-subspace writes, capability issuance downstream of the folded Group Role, and **re-issue-not-revoke-in-place** across a fold-driven authority change (E.3 / §D) — a revoked persona's stale capability fails and a re-issued one succeeds, deterministically on both members. Bindings remain `Design`; the `SubspaceId`/genesis byte encodings stay `[gates-release]` (E.1), and the against-real-crypto `green-real` upgrade is FINDING-stopped by the run's Design-grade scope wall.
- **Tag posture.** The Meadowcap/Willow primitive facts are `Established` against the live specs (`[confirm]` on exact version labels, single fetch 2026-07-15). All Drystone bindings — namespace=genesis-hash, subspace=persona, fold-supplied revocation, primary stance — are `Design`. Nothing here is `green-real`. No trust tier is decided (I9).
