# Drystone: open items to clarify and settle

This is a read-and-decide document, regenerated after the latest round of rulings. It has two parts.

Part one is a short ledger of what your rulings settled this round, so you can confirm I captured each correctly. If any line misstates your intent, that is the thing to flag.

Part two is what genuinely remains open, framed so you can mark a decision against it. Items you have explicitly deferred (with no action needed now) are listed last so they do not clutter the live set.

Where an item carries a recommendation it is labeled "my read" and is a judgment, not a fact.

---

## Part one: settled this round (confirm I have these right)

These are applied in the documents. Read them as "here is what I recorded you deciding."

**Logical clock is per-client.** A device can host multiple clients in multiple scopes (the Linux namespace / cgroup case: one device, several clients each with their own time), so the lamport counter advances per client, not per device. Applied and confirmed; no longer open.

**The peer/member/lineage relationship is corrected.** A group recognizes **members**, and a member is a **client** (MLS sense). Lineage then resolves a group's member-clients to **one peer**. The phrase "group-recognized peer" is gone; weight and thresholds now read "one per distinct peer (by lineage)." The personhood judgment (whether that peer is one human) is kept separate as the group's contextual call.

**"Meer" is a retained colloquialism** for a blind store-and-forward node, kept for clarity. The earlier model where it was a full group member with no local history is abandoned, and the language is cleaned to match.

**The meer is optional, and distinct from an iroh relay.** Verified against RFC 9750: a central Delivery Service is not required, and clients can communicate directly peer-to-peer. Because MLS is asynchronous (no two clients need be online at once), a message for an offline recipient must be held somewhere; the meer is the optional store-and-forward that does this when a high-availability node is available. It is one realization of the DS store-and-forward function. The iroh relay is a separate, transport-layer blind packet forwarder for reachability. Both are blind, both optional, different layers. The section 10.5 summary now says Drystone removes the **ordering** center while keeping the blind store-and-forward function available as the optional meer.

**The rights floor is three: tenure, voice, exit.** `share` is dropped as a right. A claim on shared assets is not part of the inalienable floor; where it has substance it is ownership of a Meadowcap communal namespace, a data-layer matter. The only rights item still open is the tenure-under-rekey check (see B1 below).

**Grounds of authority recorded.** A peer's authority is grounded in the rights floor being variety-enabling and therefore system-sustaining: negating a right lowers the variety available to resist the next negation, so rights-negation is the self-amplifying move toward collapse, and the cost is systemic, not local. Separately, what binds a human to a peer (mint-and-bind) is contextual: a family group is simpler and higher-trust to bind, a large disconnected group harder, the same trust gradient as personhood.

**Escalation tolerance is the default value, left to implementation.** The spec states the axes and declines to pick the default; what is left open is knob granularity and shipped defaults, both tuning decisions against a real deployment, not protocol constants.

**Citation fixes applied.** RFC 9750 reference corrected from section 3.5 to section 6.4 (Access Control); the Matrix "trusting servers not to lie about the time" line softened from a verbatim quote to a paraphrase with the "common case" marked as your gloss; the duplicated Appendix line removed.

**Em-dashes removed.** Both documents are now free of em-dashes (was 219 in Part 1, 343 in Part 2). Replacements were chosen by grammatical role: colons for bullet labels and headings, commas for appositives, semicolons for independent-clause joins, parentheses where a comma would pile up. One thing to confirm: I converted heading title-separators to colons too (e.g. "P-Local-Truth: the only canonical state is local"). If you want headings to keep a different separator, say so; it is a quick change.

**Part 1 §2.6 added (voice requires field-integrity).** A bridging subsection was merged into Part 1 after §2.5, linking the **voice** right to a precondition the existing mechanics already satisfy: that the field a peer asserts into is not authored by an external party against an objective the peer cannot see. It names three properties of legitimate ordering (peer-governed, legible, exitable), maps each to an existing mechanism (§2.2 no-silent-mutation, §2.4 fork, Part 2 §7.4 silence-is-not-currency, §7.6 fork), and marks the endemic-ordering point as a tension (the center-free field is not unshaped; it removes the structurally-adverse curator and returns ordering to peer governance). It adds **no new principle and no new wire obligation**; it is explicitly not a fifth peer-property. Part 2 §7.4 gained a one-sentence back-reference tying its silence rule to this legibility property. The empirical and ownership-form grounding lives in the separately-maintained companion narrative, not in the spec. No ruling needed; recorded so the addition is not lost.

---

## Part two: what genuinely remains open

### B1. Tenure under re-key (the one remaining rights check)

The rights floor is now three (tenure, voice, exit). The open question is whether the section 7 survivor / re-key path can leave a peer formally a member but unable to re-establish its standing after a re-key. If it can, tenure is not yet a clean right and the set cannot harden.

The concrete test, now written into Appendix B: take a peer with valid standing, drive a survivor re-key of the scope, and check whether that peer can still re-establish its membership standing from its retained lineage and local state, or whether the re-key strands it. If it strands, tenure needs work before freezing.

The decision owed: you said you need to clarify what to test so you can run it. The test above is my proposed shape. Confirm it captures the failure you are worried about, or correct it, and the check can run.

### B2. Group-principal as a communal namespace: key rotation, and primary-vs-secondary

The group-principal is shaped as a Meadowcap communal namespace, motivated by fork/merge: to honor a cheap fork, a shared asset has to be jointly owned by the clients (and so the peers) and the group, so both forks carry the whole thing like an open-source repository fork. A communal namespace fits that.

Two things are unworked, and the spec says so:

- The **key rotation scheme**: how the group and its members jointly own the namespace and how the key rotates under membership churn.

- **Primary vs secondary**: is the group-principal a communal namespace at all times (primary), or is the communal namespace established only at a fork or merge, when joint ownership has to be made explicit (secondary)?

The decisive next step you named: dig into Meadowcap and check its alignment with MLS, specifically whether group-associated assets can fork and merge sanely across the two layers, before committing the construction. No ruling needed now; this is a design investigation to schedule.

### B4. Capped-root soundness: state your test coverage (the priority security item)

Matrix concluded under adversarial review that sound decentralized resolution needs an uncapped root (MSC4289). Drystone claims a capped, succession-revocable root is sound because its ordering is timestamp-free. The spec marks this argued, not proven.

The decision is not "is it sound" in the abstract but what coverage you can claim. The MSC4289 attack class has at least three components: backdating to manufacture position (Drystone's timestamp-free order addresses this), create-event / root forgery (the genesis hash addresses this, MSC4291 / CVE-2025-54315), and self-demotion / promote-others entrenchment (the anti-entrenchment ladder addresses this). The open question is which of these, and which of their compositions, your tests actually exercised.

You said you will work out the tests and bring them back. Nothing to apply now; this stays the top open item, and the spec will state "components argued, composition coverage pending" until you supply it.

### B3. Root-authority succession

How founding authority transfers when founders leave is deferred to a Lifecycle section and called the most dangerous operation in the system. It interacts with B4 (a capped root only works if succession is sound). Open question: is succession in scope for the next revision, or explicitly deferred to the Lifecycle work?

### B6. Capability mechanism: Track A (Meadowcap) vs Track B (Keyhive)

You confirmed the deferral is correct, and recorded the lean: Track A's Meadowcap vocabulary is preserved deliberately, because Willow and Meadowcap are the foreshadowed data-layer path and not colliding with that vocabulary is the stronger default. Keyhive (Track B) is the better mechanism on revocation immediacy and is not ruled out.

What would settle it, and the actual open work: define Drystone's revocation needs concretely (expulsion cadence, acceptable stale-authority window, complexity budget), then test each track against them. Until that needs definition exists, the deferral stands. This is a design task to schedule, not a ruling.

---

## Deferred with no action needed now

These you have explicitly parked. Listed so nothing is lost, but none needs your attention this round.

**B7. Hash function (SHA-256 vs BLAKE3).** Not a worry. Testing ran on SHA-256; any reliable hash works. This is a follow-up note for the proofs that ship with the spec, not a design decision. The only mechanical consequence to remember: if the final suite is renamed, the section 4 signature proofs re-open, since the hash is signed over.

**B5. Escalation tolerance default.** Settled as open-by-design (see Part one). The implementation will set knob granularity and defaults against a real deployment.

**C. ENABLING wire encodings.** The 16 byte-level markers are known and need filling in, but only after the conceptual and nomenclature work settles, which is what this round was. They gate a publication-final release, not the build-against shape. When the nomenclature is stable, the load-bearing ones to specify first are: the canonical governance-fact byte encoding (section 7.3.1, the base the others extend), frontier-closure-and-subgraph-closure before sort (section 7.5.2, the most likely place two implementations diverge), the frontier-commitment and acceptance-record formats (section 7.5.1), and the content-id pre-image now that timestamp is removed (section 4.2).

---

## Still-flagged primary sources (before external publication)

Not design items, but tracked: these carry `[confirm before publish]` and were not re-pulled against primary texts this round. Beer "Brain of the Firm" and the Cybersyn/OGAS figures; Ostrom's 1990 principles 6 and 7 and the 2013 subsidiarity wording; the decentralized-MLS drafts (`draft-kohbrok-mls-dmls` / FREEK, `draft-xue`); TLS/X.509 against RFC 5280 and SSH against the OpenSSH man pages; Keyhive. The MLS facts (RFC 9420/9750), the Meadowcap/Willow semantics, the Matrix MSCs and CVE-2025-49090, and the CALM theorem were verified against primary sources across these sessions.

**Companion narrative tracked separately.** The §2.6 motivation now points to a companion document set (the grounded argument "Peer Standing, the Securitized Corporation, and the Cooperative Form," the essay "Tilling the Soil of Our Relationships," and an elevator pitch). These carry their *own* source discipline and verification state and are **not** part of the Drystone spec's `[confirm before publish]` list; the spec depends on them only for motivation, never for a mechanism. One live item inside the companion to surface here because it is time-sensitive: the "Project Mercury" allegations (a Nov 2025 litigation filing) had a hearing set for Jan 26, 2026; a re-check of public reporting as of late June 2026 still surfaced no post-hearing ruling, so the docket itself (PACER) must be pulled directly before that document is published externally. This does not gate the spec.

---

## What is actually on your plate

Two things need a ruling to move: **B1** (confirm the tenure-under-rekey test shape so it can run) and **B3** (is succession in scope for the next revision).

Two are design investigations to schedule, not rulings: **B2** (Meadowcap/MLS fork-merge alignment) and **B6** (define revocation needs).

**B4** stays the priority security item and is yours to bring back with test coverage. Everything else is parked or done.
