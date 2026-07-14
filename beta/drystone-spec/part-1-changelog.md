# Drystone Part 1: changelog

`Status: attached to drystone-part1.md`

`Purpose: the what-changed-here record for Part 1. The why-and-the-rule for each entry lives in drystone-conventions-and-synthesis-decisions.md (cited as "primer §X"). Together they make Part 1's history auditable while Part 1 itself stays an end state (doc-method Rule 1, Rule 13).`

---

## Pass 1: terminology alignment for the self-contained synthesis

This pass brought Part 1's vocabulary into line with the bounded-context conventions settled for the consolidated Part 1 / Part 2, and made two implicit things explicit (the sociotechnical alignment of right/role, and the consequence-not-constraint logic of weight-equality). No design claim changed; the derivation and the four principles are unaltered in substance.

### Terminology

- **Added a group / Group / scope convention note** after the existing persona/peer note, before first substantive use. States the three-way distinction (social group vs in-system Group vs the wider scope) with the "Bob hosts the relay" seam as the worked example. (Primer A.1, A.3.)

- **group → Group (capital G)** at every site meaning the in-system entitlement-and-governance unit: the recursion header ("a principal is recursively a Group"), the artifact-ownership/fork passage (§2.3), the recognition/personhood judgments ("the Group makes at its own confidence," "the Group's recognition"), the anti-capture claim ("what the Group judges"), and the §2.4 consequence. Left **lowercase group** where the sentence names a social body of people (the recursion illustration "a community is a group of people"; the Ostrom grounding). (Primer A.1.)

- **Tightened "a user is a group of devices"** to "a persona's own devices are a Group (in MLS terms a Group of clients, the key-and-author bindings that live across a persona's devices)." Replaces the rough early framing with the device-Group framing the rest of the suite uses. (Primer A.1.)

- **role → Group Role (capital G, capital R)** at concrete in-Group-grant sites (the Role bullet, "the authority to act for a Group," "lose admin on a Group," "even when every Group Role is stripped," the decomposition formula, the vocabulary-against-prior-art note). Left **lowercase role** for the genus (the category of delegated authority contrasted with rights in the category definitions). Added the genus/instance parenthetical at the Role bullet. (Primer A.2.)

- **PrincipalSet → Group Role Set (capital S)**, promoted to a first-class three-word term, with an inline gloss on first appearance (the two functions: unit grant/revoke, and mutual-exclusion for separation of powers). Carried as named-but-settling; mechanism deferred to Part 2 §5. (Primer B.2.)

- **per-scope governed tolerance → per-Group governed tolerance** (§2.5), the escalation tolerance being a Group-governance policy, not a scope-level (exposure) one. (Primer A.3, worked disambiguation.)

### Clarifications and rewrites (improvements surfaced during the pass, logged so they are not mistaken for silent drift)

- **"By necessity" weight-equality phrasing rewritten** (§2.3 header, Weight bullet, summary aphorism). The carried phrase read as a constraint tolerated ("stuck with"); rewritten to frame weight-equality as a *consequence* of rights-equality, the same commitment expressed in a different context, not an independent conclusion the design arrives at on its own. (Primer B.2.1.)

- **Right bullet gained an explicit scope-of-standing sentence**: a right is standing in the *system*, not in any one Group; it is what the fork carries into both descendants, which is why a Group's governance can never reach it. This makes explicit the claim the lowercase-role version had left ambiguous (rights are system-scoped, Group Roles are Group-scoped). (Primer A.2.)

- **Reasoning paragraph rewritten to name both planes** (§2.3). The right-vs-role distinction is now stated first as the layer-independent social/epistemological principle, then Drystone's Group Role is named as that social role made technical with fidelity, with the punchline that the mechanism is trustworthy exactly because the wire distinction reproduces the social one rather than inventing a different one. (Primer B.2.1; doc-method Rule 12.)

- **Ostrom grounding: social semantics preserved, technical realization linked** (§3). Reverted an over-eager capitalization; her principles stay lowercase group (human communities), with the capital-G Group named as the realization of the value, so the homage is not overwritten. (Primer B.2.1.)

- **"The Group holds keys" → "the members hold the keys"** (§2.4 consequence). Sidesteps the ambiguity, newly sharp because a Group can itself be a principal, between the Group-as-principal holding a credential and the member clients holding the group key. The intended referent (material reversibility, no helper holds the data hostage) is the member clients. (Primer B.2.1, with the standing vigilance note.)

### Mechanical

- Verified: no em-dashes and no double-hyphens in prose; blank-line-between-bullets preserved; no leftover "PrincipalSet" or "by necessity"; cross-references intact.

### Not changed (recorded so the boundary is clear)

- The four principles, the razor (§2.0), the time-is-not-a-fact corollary (§2.0.1), the forced terminus (§2.5), and the field-integrity bridge (§2.6) are unchanged in substance.

- **persona** was already sharp in Part 1 and is unchanged; the synthesis does not loosen it. (Primer A.6.)

- All `[confirm before publish]` external-fact flags (Lamport, CAP, CALM, CRDT, Mill, Hayek, Ashby, Beer/Cybersyn/OGAS, Popper, Ostrom, Matrix MSCs, Spritely) are preserved as-is; this pass touched vocabulary and framing, not source verification.

---

## Cross-document consistency read (Part 1 against the reshaped Part 2) + carried-forward Rule 1 fixes

`Status: complete`

After Part 2 grew substantially through the folds (2751 to ~3874 lines), a read confirming Part 1 stays consistent with it and applying the Rule 1 prior-state-contrast sharpening that postdated Part 1's own pass.

- **Cross-references verified in both directions.** All four governance principle labels (`P-Local-Truth`, `P-Knowable-Truth`, `P-Peer-Equality`, `P-Durable-Enablement`) match Part 1 and Part 2 exactly; Part 2's `P-gossip`/`P-meer`/`P-none`/`P-push` are the §6 presence-plane namespace, no collision. Every `Part 1 §` reference in Part 2 resolves to a real Part 1 section, and every `Part 2 §` forward-reference in Part 1 (§5.0, §5.2, §5.4, §5.6, §5.7, §5.10, §4.3, §4.5.1, §7.3, §7.3.1, §7.4, §7.6, §8, §10, §3.1, Appendix C) resolves to a real Part 2 section that the folds added beneath but did not renumber or remove.

- **Concept consistency confirmed** on the two highest-risk seams. §2.5 (fork-not-verdict) and Part 2 §7.6 share framing, the A/B (Carol/Bob) running example, and the "not every conflict to humans" caveat. §2.3 (rights/weight/resources/Group-Roles) matches Part 2 §5's four-property split term-for-term. §2.6 (field-integrity) is realized in Part 2 (§7, the Track A/B capability decision, and the explicit "legibility property of field-integrity, Part 1 §2.6" at §7.4.2), so the principle is not orphaned.

- **§2.5 forward-note added.** Part 2 §7.6.1 now enumerates the residue's *two* shapes (contradiction = too many claims; under-determination = too few). Part 1 §2.5 stated the residue generally (which already covers both) but illustrated only contradiction, so a one-clause note was added pointing to §7.6.1, keeping Part 1 at the principle level while flagging the Part 2 elaboration so a reader is not surprised.

- **Rule 1 fixes carried into Part 1.** Part 1 §2.3 carried the same prior-state-contrast pattern scrubbed from Part 2 ("an older, looser 'peers are equal' had tangled into one phrase"; "the sentence that replaces every earlier formulation"). Both recast to the positive, using the same "one-sentence statement of the model" phrasing as the Part 2 §5 fix, so the two documents now match verbatim on that line. These survived Part 1's earlier pass only because the prior-state-contrast sharpening of Rule 1 postdated it.

**Verification:** Part 1 clean, no em-dashes, no double-hyphens in prose, no "PrincipalSet," no residual prior-state-contrast tells.

---

## Versioned upstream reference links + cross-document reference disambiguation

`Status: complete`

**Added a versioned "Upstream reference links" section** at the end of Part 1, giving edition- or version-specific canonical sources for the principled lineage (Lamport, CAP/Gilbert-Lynch, CALM, CRDTs, Mill, Hayek, Ashby, Beer, Popper, Ostrom 1990, Wilson-Ostrom-Cox 2013, Modular Politics, Spritely/ActivityPub) so a reader resolves the exact work cited rather than a later edition or a secondary summary. Each entry carries DOIs or canonical URLs where they exist, and a verification marker: CALM and the Spritely quotations are *(verified this revision)*; the items whose verbatim wording or edition is still to be pulled (Lamport, CAP, CRDT statement, Beer's "specify only somewhat," the Ostrom principle wordings, the Modular Politics quotations) carry **[confirm]** consistent with the in-text citations, rather than being presented as settled. The section explicitly points to Part 2's own versioned links for the mechanism/software lineage, so the split (Part 1 = principled lineage, Part 2 = mechanism lineage) stays clean.

**Cross-document reference disambiguation (a real correctness fix).** A full sweep found that bare `§4`–`§8` references in Part 1 (46 of them) pointed at Part 2's section space without the "Part 2" prefix, so a Part 1 reader seeing "(§8)" or "the §7.6 fork" would search Part 1 (which has only §1–§3) and fail. All 46 were prefixed to "Part 2 §x". The sweep correctly left Part 1's own `§2.x` self-references bare (including inside the reference list, where "Grounds §2.0.1, Part 2 §7.3.1" now disambiguates a Part-1 self-reference from a Part-2 reference in the same line), and did not touch any DOI or URL. The mirror defect in Part 2 (bare `§2.x` references to Part 1's space, 41 of them) is fixed in the Part 2 changelog. Net result: a bare section number is no longer ambiguous about which document it points to, in either direction.

**Verification:** Part 1 clean, no em-dashes, no double-hyphens in prose, no "PrincipalSet," zero truly-dangling cross-document references (confirmed by the same lookbehind logic used to apply the fix).

---

## Pre-publication verification (CRDT reference precision)

`Status: complete`

As part of the cross-document pre-publication verification pass (detailed in the Part 2 changelog), the **CRDT reference** in Part 1's principled lineage was confirmed and made precise: SSS 2011 (LNCS 6976, pp. 386-400, DOI 10.1007/978-3-642-24550-3_29) with the companion INRIA RR-7506 "A Comprehensive Study of Convergent and Commutative Replicated Data Types" (January 2011), noting RR-7506 is distinct from the later RR-7686. The `[confirm]` marker on that entry is cleared. All other Part 1 reference markers (Lamport, CAP, Mill, Hayek, Ashby, Beer, Popper, Ostrom, Modular Politics) remain as-is; their verbatim-wording confirmations are pre-publication work against the specific print editions, not identifier questions.

---

## Human-adjudication language pass (§3 Internet-governance corroboration)

`Status: complete`

Part of the cross-suite adjudication-language codification (conventions A.11, Part B §B.8).

- **§3 gained an Internet-governance corroboration entry**, placed adjacent to the Beer / algedonic-channel block, in the section's established conclusion-then-verbatim-source format. It grounds the escalate-to-a-human posture in the formal-specification tradition in both of the shapes Drystone uses: a named human role inside a procedure (the IANA **Designated Expert**, RFC 8126 / BCP 26 §5.2) and a normative human-input step inside an algorithm (W3C **powerful features** requiring express permission, Permissions TR and Geolocation CR), plus the IETF publishing its own human-layer semantics in the same series as its wire formats (rough consensus is objections addressed, not votes counted, RFC 7282). The entry names the one-level-deeper point: Drystone keeps the move (at this step a person decides) and changes only the *locus*, distributing the designated-expert role to every **local authority** rather than delegating it to one center.

- **All three quotes fetched verbatim from the primaries this run** (RFC 8126 §5.2, W3C Permissions §3.3 and Geolocation §3.1, RFC 7282 §3) and marked *Verified against the primary at edit time*; the Geolocation entry flags that the current CR wording differs from the 2016 REC.

- **Three reference-list entries added** (RFC 8126 / Cotton-Leiba-Narten 2017; W3C Permissions TR + Geolocation CR; RFC 7282 / Resnick 2014) in the existing annotated-bibliography format, each carrying the verification marker.

- **Back-map updated:** the §3 map line now lists Internet governance among the grounding disciplines.

**Verification:** no em-dashes, no double-hyphens in prose; quotes verbatim; cross-references resolve.
