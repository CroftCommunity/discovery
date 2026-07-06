# Drystone — Reviewer Handoff

This document gives a reviewer everything needed to check the three Drystone documents for **clarity, consistency, and correctness** without having to reconstruct the design history.

The three documents under review:

- `drystone-part1.md` — Reasoning Underpinnings (the *why*: principles, grounding, no wire formats).

- `drystone-part2.md` — The Certifiable Design (the *how*: data model, identity, transport, sync/resolution, security, substrate requirements).

- `drystone-changelog.md` — theme-organized record of changes, newest entry first.

Author: Chase Pettet. The author's working preferences (apply when editing): accuracy before fluency; no em-dashes as a narrative device; no run-on sentences; blank lines between markdown bullets; separate what a source says (cited) from interpretation (labeled "ours"); flag judgment-vs-fact; lead with the answer; concise.

---

## 1. What Drystone is, in one paragraph

Drystone is a **center-free peer protocol** for groups to hold shared state (membership, authority, content) with no server and no global apex. Its founding move is a single razor: a node establishes only **provenance** (what is cryptographically corroborated), never **truth** (what is right or fair). Monotonic conflicts auto-resolve through a timestamp-free causal-cryptographic fold; the non-monotonic residue (genuine standing contradictions) hard-stops and escalates to humans, terminating not in a verdict but in a **fork**. The thesis is **synthesis and terminus**, not invention: established results across several fields, taken seriously, force a humane shape for governance.

---

## 2. The founding razor and the four principles

**The razor (Part 1 §2.0):** compute provenance, never utility. Provenance is what the system corroborates (authorship, continuity, causal order, chain validity). Utility is what humans judge (what is right, fair, true, trustworthy). The protocol does the first and refuses the second. Everything else descends from this.

**The sharpest edge (§2.0.1): time is an assertion, never a fact.** A wall-clock timestamp fails corroboration at the root — not merely because a peer can lie about its clock (the weak reason), but because no node can prove when an event occurred on another node; there is no shared clock, only each node's local proxy. So a timestamp is on the utility side and cannot order anything that must converge or resolve authority.

**The four principles (Part 1 §2.1–§2.4):**

- **P-Local-Truth** — the only canonical state is local; replicas converge without a coordinator.

- **P-Knowable-Truth** — verify for yourself; the record is auditable, never silently mutable.

- **P-Peer-Equality** — see the equality model in §3 below. This is the most-edited principle and the one to scrutinize hardest.

- **P-Durable-Enablement** — participation, and exit, must be real on a bare node (no mandatory helper, no privileged tier required to participate or to leave).

**The forced terminus (§2.5): fork, not verdict.** When a genuine standing contradiction cannot be auto-resolved, the system does not adjudicate a winner. It hard-stops and the remedy is a dignified fork — both sides continue with their full state. This is a designed primitive, not a failure mode.

---

## 3. The equality model — the load-bearing vocabulary (scrutinize first)

This is where the most conceptual churn happened and where a reviewer should look hardest. The current, correct model:

**P-Peer-Equality answers one question: in what ways may one peer differ from another? There are exactly four properties — two necessarily equal, two legitimately unequal.**

The two **equalities** (equal for every peer, always):

- **Right** — what a principal *inherently holds*: the floor of voice, tenure, and exit/fork. Equal and unremovable. The proof a thing is a right and not a role: exit/fork survives even when every role is stripped and even when a quorum captures the group. A right cannot be delegated or revoked.

- **Weight** — how much a peer *counts* in governance. Flat: one per personhood-verified peer. Equal **by necessity, not by separate decree** — it follows from the equal right (equal standing-to-participate is the same fact as equal standing-to-be-counted). Weight is the governance image of the rights floor.

The two **inequalities** (legitimately different between peers):

- **Resource** — what a *device* has (storage, uptime, reachability, a radio). Intrinsic, descriptive, expected to be unequal, not delegable. Says what is *possible*, never what is *permitted* or how much a peer *counts*.

- **Role** — what governance authority a principal has been *granted* (admin, moderation, gating, act-for-the-group, the authority to issue capabilities). Granted by consent, scoped, attenuating, always revocable. The one *operational* inequality the design permits, and it rides entirely above the two equalities — granting or revoking a role never changes a peer's rights floor or its unit of weight.

**The canonical sentence (must match everywhere):** *peers are equal in rights and (by necessity) weight, and unequal in resources and revocable roles.*

**Capability is NOT a fifth property.** Capability (the Meadowcap data-access grant — read/write an area of a namespace) is the **mechanism a role operates through**, a data-plane token (Part 2 §7.1, §10.4) one level *below* the peer-equality question. It sits under roles, not beside resources. A reviewer should flag any place that lists capability as a peer-equality axis or treats it as interchangeable with role.

**Known terminology traps to check for (these were errors that were fixed; verify no relapse):**

- "Rights are allowed to be unequal" or "equal in weight; unequal in (delegated) rights" — WRONG. Rights are always equal; the inequality is a role.

- Weight described as "the axis the design clamps" or "deliberately flat" — incomplete. Weight is equal *by necessity*, derived from equal rights, not a standalone clamp.

- "Capability" used for a device facility — WRONG. That is a *resource*. Capability is Meadowcap's data-access sense only.

- The phrase "three orthogonal axes" — obsolete. The model is "two equalities, two inequalities." (An always-equal "right" was never really an "axis.")

- "Object-capability capability = Drystone role" mapping — obsolete and wrong. Capability keeps Meadowcap's meaning; role is the separate governance-authority layer.

---

## 4. The identity model (Part 2 §5.2)

Two layers, because the rights-holder is not the same granularity as the device that acts:

- **Identity layer — principals and clients.** A **principal** is a role-holding entity identified by exactly one authenticatable identity (one key-lineage). A **client** is a single device (one MLS leaf, one signature key, one credential). A principal is *realized by one or more clients*.

- **Governance layer — peers and weight.** A **peer** is the kind of principal that carries the rights floor and local canonical state and is the source of one unit of weight.

**Kinds of principal:** peer; **group** (a communal-namespace collective identity, §5.10); **meer** (a blind broker — holds an availability-serve role but no rights floor and no weight, therefore a principal but NOT a peer; the legacy label "mere-peer" is deliberately avoided in normative text); **delegate** (a state, not a species).

**The peer-vs-personhood keystone (§5.2) — the conceptual center:**

- A **peer is a provenance object** — technically representable, the root of a cryptographic lineage, verifiable and countable.

- **Personhood is a social judgment** — whether that lineage is a distinct human. It has no technical representation, because it was never the protocol's to certify.

- The binding between the two is therefore an **adjudication, not a lookup** — the identity-layer instance of the provenance/utility split. This is *why* "peer" and "personhood" are distinct words and must not be used as synonyms.

**The governance-integrity spine:** quorums and thresholds count **peers by personhood-verified identity, never clients.** Adding devices adds resources, never rights or weight. Grounded in RFC 9750/9420 (multiple devices = distinct clients; the application owns access-control policy).

---

## 5. Personhood is contextual (Part 2 §5.6) — check this framing carefully

This was reframed and the old framing is wrong; verify no relapse.

**The protocol guarantees provenance; the group judges personhood.** The load-bearing claim is NOT "personhood is unforgeable" (that is false, and was a provenance/utility conflation). The correct split:

- *Protocol guarantee (provenance, by mechanism):* weight is flat per recognized peer and conserved under delegation, never minted by clients or resources.

- *Group judgment (personhood, contextual):* whether a recognized peer is a distinct person is a utility judgment the group makes at its own confidence, on the trust-to-do gradient — high (QR-scan family scope), medium-and-anonymous (a credential service enforcing one-per-ID without revealing the ID), low (open broadcast).

**Sybil resistance is contextual, not global.** In a low-binding scope one human may hold many lineages; that is an accepted property of that context, not a bug. Declining to solve global personhood is faithfulness, not a cop-out: enforcing one-key-one-human would prune legitimate **multiple self-presentation** (same person as parent, pseudonymous activist, anonymous participant), which is the variety argument applied to identity.

**Credential-service caveat (a correction to watch for):** delegating the personhood check to a third-party attester is itself a *utility judgment to accept*. If the attester turns adversarial the remedy is withdraw-trust-and-fork — NOT a technical revocation primitive. "Revocable" need not mean a protocol off-switch; the fork is the revocation. A reviewer should flag any text implying the credential service is a structural dependency needing a technical primitive (that re-collapses utility into provenance).

**Grounding (confirmed):** cryptographic trust always bottoms out in a social judgment — three verified examples: TLS/X.509 CA (chain airtight, anchored to CA reputation; trust-store is social — but seam marked: a CA is *centralized*, Drystone's attesters are forkable per-edge), PGP web-of-trust (validity by transitivity of human attestations, user-set thresholds, explicitly NOT designed for personhood/Sybil), SSH TOFU (crypto guarantees continuity; the human decides "right host").

---

## 6. The control-plane / data-plane lesson (Part 1 §2.0.1, Part 2 §7.3.1)

The most recent refinement. The point: **governance ordering is a control-plane job and must not be handled like data-plane chat messages.**

Concretely, this is about **timestamps in the authority-deciding path**. A timestamp is a fine convenience for the *data plane* (ordering chat for display, where being wrong is cosmetic), and a *capture vector* in the *control plane* (deciding who won a governance conflict, where a forgeable ordering input lets an adversary manufacture a result). Same value, different blast radius, because the jobs differ. Drystone orders conflicting governance facts by causal-and-cryptographic means only (issuer authority rank, governance precedence class, causal length, content-address tiebreak) — never by wall-clock.

**Architecture-relativity (important — do not let this read as universal or as "Matrix erred"):** the requirement that the authority path exclude forgeable ordering inputs is **forced by Drystone's architecture, not a universal law.** Two sides of one coin: (1) no authority tier above the peer, so a bad resolution cannot be overridden in place; (2) the fork is the only remedy, so a routinely-gameable input would mean a heavyweight schism each time it is gamed. A system *with* an authority tier (Matrix) can override a bad resolution cheaply in place and can therefore rationally tolerate a timestamp tiebreak. The constraint applies to "any system whose nodes each hold their own canonical view and whose only remedy is exit rather than in-place override" — Drystone and anything sharing those two properties, not systems we are not building.

---

## 7. The group as a principal (Part 2 §5.10)

A group is a principal: a **Meadowcap communal namespace** (horizontal authority, no apex = P-Peer-Equality at the data layer), living **above MLS** (MLS is the communication-and-safety substrate; the group-principal is the application-layer ownership/governance construct in the artifacts).

**Forked-artifact ownership (the worked case, illustrated in both Part 1 §2.3 and Part 2 §5.10):** when three peers collaborate on an auto-merged document and the scope forks, *both layers own it and both forks carry the whole artifact* — like an open-source fork carrying the full repository. Communal authority was distributed all along, so the fork orphans nothing and fragments nothing. This is the data-layer face of fork-not-verdict.

**Composition** nests (user → community → federation), each a communal namespace. **Act-for-the-group** is a governed, revocable role (a group cannot sign; some principal must act for it). **Recursion bottoms out at flat-weight peers** so composition cannot launder governance weight: total weight in any scope reduces to the count of distinct personhood-verified peers at the leaves.

**Communal vs owned (Meadowcap, confirmed):** communal = peer-equal (Drystone's default for governance); owned = apex single-keyholder (rejected for governance, allowed only for single-author sub-content — the share/tenure seam).

---

## 8. The dial-discipline principle (Part 1 §2.3)

Adversarial posture is **per-edge, not global** — your own device pool is, *by default*, a high-trust composition edge needing little Byzantine defense; a stranger is a low-trust valuation edge needing more.

**But every edge is a dial, including your own device group.** A seized-or-coerced-device threat model (activist, journalist, unsafe household) may rationally want Byzantine suspicion within its own device group. Hardcoding "device group = trusted" prunes that case.

**The discipline (so "everything is a dial" doesn't become unusable):** default the common case hard (most groups never touch it), keep the uncommon case representable without ceremony, never let a default calcify into a structural assumption that forecloses the alternative. Variety is preserved by the 20% being *expressible*, not *foregrounded*. This is explicitly not a footnote.

**Two relationship types that must not be conflated:** *composition* (members co-deriving shared authoritative state — the MLS/shared-key relationship; merges state) vs *valuation* (one group weighting another's assertions without a shared key; weights signals). Blur them and trust leaks into key access.

---

## 9. Prior-art grounding and its status (Part 2 §10, Appendix C; Part 1 §3)

What Drystone builds on, and what the relationship is:

- **CALM theorem** — the formal spine. A problem has a coordination-free implementation iff it is monotonic. This is exactly the razor's resolvable/irreducible split: monotonic governance folds without coordination; the non-monotonic residue is what §7.6 escalates.

- **MLS (RFC 9420/9750)** — the group key-agreement reference. Provides FS, PCS, membership agreement, async operation. Drystone adds FS-under-concurrency (it has no Delivery Service). MLS deliberately leaves access-control to the application — that is the slot Drystone's *role* layer fills. The AS role is filled by the user-principal-as-its-own-CA.

- **iroh** — the transport reference (public-key endpoint identity, blind relay, QUIC). Requirement is reachability, not topology.

- **Willow / Meadowcap** — the data model and capability reference (namespace/subspace/path; communal/owned namespaces; attenuating read/write capabilities). Drystone is built Willow-*shaped*, not Willow-*dependent*.

- **Matrix State Resolution** — the closest neighbor at the resolution layer: same DAG-plus-topological-sort skeleton, *opposite* spine (Matrix folds sender power and timestamp into the ordering; Drystone removes both). Cited as both kin and cautionary evidence.

- **Modular Politics** — the closest governance-as-protocol prior work ("drew the map; Drystone built the territory").

- **Cross-disciplinary** (Part 1 §3): Ostrom (commons governance), Ashby (requisite variety — the backbone of the variety arguments), Spritely/Lemmer-Webber (contextual identity, don't-overclaim), plus ethics/economics/epistemology grounding.

**Source-confirmation status (this is a consistency item — check the markers match reality):**

- **CONFIRMED verbatim this revision (markers cleared):** RFC 9420/9750 (MLS access-control, device-correlation §8.2.4, FS/PCS); Meadowcap/Willow (capability definition, communal/owned semantics, BLAKE3-256, namespace/subspace/path); Spritely Institute (contextual-flows, don't-overclaim, networks-of-consent quotes); Matrix State-Resolution-v2 timestamp tiebreak (`power_level` → `origin_server_ts` → `event_id`, plus the authors' "trusting servers not to lie about the time" admission); PGP web-of-trust and SSH/GnuPG TOFU.

- **STILL FLAGGED `[confirm before publish]` (NOT yet verified against primary texts):** Matrix MSC4289 (uncapped creator power), CVE-2025-49090 / MSC4297 (state-reset root cause and conflicted-subgraph mechanism), MSC4291 / CVE-2025-54315 (create-event uniqueness); the CALM primary paper statement; the Beer "Brain of the Firm" quote and the Cybersyn/OGAS figures; TLS/X.509 against RFC 5280 and SSH against the OpenSSH man pages (the two halves of the three-example case not directly pulled); iroh relay-blindness against iroh 1.0 docs; the MLS-DMLS / FREEK drafts for the FS-under-concurrency claim. A reviewer should treat any claim still carrying a marker as unverified, and should NOT clear a marker without pulling the cited primary source.

`ENABLING` markers are a separate category: they flag wire-format/encoding items the author has intentionally left open (canonical byte encodings, cross-group grant formats, communal-namespace key construction). These are open design items, not source-confirmation gaps.

---

## 10. Document structure (for navigation)

**Part 1:** §1 Introduction (§1.1 Scope, §1.2 positioning). §2 Design Principles (§2.0 razor, §2.0.1 time-is-assertion, §2.1–§2.4 the four principles, §2.5 forced terminus/fork). §3 Why corroborated not invented (six fields). References.

**Part 2:** §3 Overview (§3.1 system-of-peers diagnostic). §4 Data Model (§4.1–§4.5, §4.5.1 per-device authorship/self-AS). §5 Identity (§5.0 two-equalities-two-inequalities + two layers, §5.1 local-canonical, §5.2 principal/client/peer + keystone, §5.3 rights floor, §5.4 resources, §5.5 role/capability/PeerSet/delegation, §5.6 weight + contextual personhood, §5.7 membership/revocation, §5.8 revocation-protects-future + §5.8.1 gating, §5.9 exitability, §5.10 group-as-principal). §6 Transport. §7 Sync/Governance-Conflict (§7.1 data model, §7.2 grant-and-revocation interface, §7.3 governance-facts-as-entries + §7.3.1 timestamp-free order + §7.3.2 what-conflicts + §7.3.3 snapshot-cache, §7.4 freshness + §7.4.1, §7.5 attributable acceptance + §7.5.1 + §7.5.2 regress-free closure, §7.6 hard-stop/fork). §8 Security (§8.1 honesty boundaries). §9 Interop/Conformance. §10 Substrate Requirements (§10.2 MLS, §10.3 transport, §10.4 primitives, §10.5 ledger). Appendix A (alternatives), B (open questions), C (consolidated prior art).

---

## 11. Specific consistency checks for the reviewer

A focused checklist, in priority order:

1. **The equality model.** Confirm the canonical sentence (§3 above) appears consistently in Part 1 §2.3, Part 2 §5.0, and §5.6, and that none of the "terminology traps" in §3 have reappeared anywhere. This is the highest-risk area.

2. **Capability placement.** Confirm capability is always Meadowcap's data-access sense, always beneath roles, never listed as a peer-equality property, never used for device facilities (that is "resource").

3. **Rights are always equal.** Confirm no text says or implies rights can be unequal; the inequality is always a role.

4. **Weight by necessity.** Confirm weight is presented as derived from equal rights, not as a standalone arbitrary clamp.

5. **Peer vs personhood.** Confirm the two words are never used as synonyms, and that personhood is always framed as contextual group judgment, never "unforgeable."

6. **Cross-references resolve.** Every §X.Y pointer should target a real section. (A prior pass fixed three dangling refs: §7.5.4→§7.5.2, §5.7.1→§5.8.1; and confirmed RFC 9750 §8.2.4 is an external ref, not internal.) Re-running a cross-reference check is cheap and worthwhile.

7. **Confirmation markers match reality.** Confirm no `[confirm before publish]` marker remains on a claim listed as CONFIRMED in §9, and that no claim listed as STILL FLAGGED has had its marker removed.

8. **Em-dashes and run-ons.** Per author preference, prose should avoid em-dashes as a narrative device and avoid run-on sentences. (Note: the technical spec uses dashes structurally in places; the author has not objected to those, but a reviewer optimizing for the stated preference may flag them.)

9. **Length of §2.3 and §5.6.** Both are very long and dense (each roughly a screen-and-a-half of stacked block-quotes). They are load-bearing so length is defensible, but a reviewer may consider whether either benefits from a subsection break. This is a judgment call, not an error.

10. **Provenance vs interpretation.** Per author preference, confirm that claims attributed to a source are actually from that source (cited), and that the author's or the assistant's own synthesis is labeled as such ("ours"), not silently attributed to the literature.

---

## 12. The one-line summary of what makes Drystone novel

Not the mechanisms (all borrowed and credited). The novelty is **vertical synthesis plus terminus**: the epistemic limit (provenance not truth) forces a data model (local-first, monotonic fold), which forces an equality constraint (flat weight from equal rights), which forces a human-adjudicated exit (fork, not verdict) when the monotonic fold hits genuine contradiction. And the fork-not-verdict terminus is a *designed primitive* forced by an intrinsic-utility argument, not a fallback. A reviewer evaluating the documents' central claim should check that this chain is stated consistently and that no section overclaims invention of the underlying mechanisms.
