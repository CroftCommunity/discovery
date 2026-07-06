# Drystone spec session: summary of work

This summarizes a multi-round review and editing session on the Drystone protocol specification (Part 1: Reasoning Underpinnings; Part 2: The Certifiable Design). The work moved from a consistency-and-citation review, through five rounds of model corrections, to a final round applying author rulings plus a full em-dash removal pass.

All deliverables are in this folder. This summary is a record, not a spec document.

---

## What changed, in order

### Initial review

A 10-point consistency and citation review of both parts, with primary-source verification. Findings: one genuine internal contradiction (the meer's rights floor stated three inconsistent ways), one citation error (MLS access control attributed to RFC 9750 section 3.5 rather than section 6.4), an imprecise Matrix quote, an overclaim that MLS forbids shared signature keys, a duplicated line, and heavy em-dash use. Primary sources verified across the session: RFC 9420 / 9750 (MLS), Meadowcap / Willow, Matrix State Resolution v2 and the 2025 MSCs and CVE, the CALM theorem, and iroh.

### Model corrections (five rounds)

**Meer reclassification.** The meer is not a principal, member, peer, or role-holder. It is blind store-and-forward infrastructure: a node, governed as scope configuration, structurally blind (never given a key). This dissolved the rights-floor contradiction at its root. "Meer" is retained as a colloquialism for a blind store-and-forward node; the earlier model (a full group member with no local history) is abandoned.

**Resource scope.** "Resource" is a property of any node (peer client devices, meers, relays), not only peer devices. The peer/meer distinction lives in the identity model, never in resources.

**Peer as human-representation.** A peer is the representation of a human, not a node. Rights and weight attach because a person is behind it. Peer-equality is equality of humans as represented, holding as long as the human-to-peer binding is one-to-one, which the system cannot attest. The overclaiming phrase "personhood-verified" was retired.

**Client / device cardinality.** A client is software on a device that is a member of a group (MLS sense, one leaf, one signature key, one credential), not the device itself. A device is hardware and may host more than one client; a human may have more than one device. This fixed the prior conflation and, as a side effect, resolved the MLS shared-key overclaim (verified against RFC 9750 section 6.7: applications may share keying material, so not-sharing is Drystone's requirement, not an MLS rule).

**Lineage as literal cryptographic rooting.** A peer is rooted in a cryptographic key pair; lineage is the literal chain of signed credentials by which each device's and client's membership key descends from that key pair. The fold follows each client's lineage back to its rooting peer and counts one peer per rooting key pair, which is what enforces equal governance weight. Lineage is not a tier between device and peer.

### Author rulings applied (final model round)

- **Group recognizes members, not peers directly.** A member is a client; lineage resolves a group's member-clients to one peer. "Group-recognized peer" was replaced by "one per distinct peer (by lineage)," with the personhood judgment kept separate.

- **Logical clock is per-client** (a device can host multiple clients in multiple scopes).

- **Meer is optional and distinct from an iroh relay.** Verified against RFC 9750: clients can communicate directly peer-to-peer with no central Delivery Service; the meer is the optional store-and-forward for offline persistence; the iroh relay is a transport-layer blind packet forwarder. The summary now says Drystone removes the ordering center while keeping the blind store-and-forward function as the optional meer.

- **Rights floor is three: tenure, voice, exit.** `share` dropped as a right (it belongs in the data layer as Meadowcap communal-namespace ownership). Only the tenure-under-rekey check remains open.

- **Grounds of authority.** The rights floor is variety-enabling and therefore system-sustaining; rights-negation is the self-amplifying move toward collapse, so the cost of violation is systemic. The human-to-peer mint-and-bind is contextual (family simpler and higher-trust; large disconnected groups harder).

- **Group-principal as communal namespace.** Motivated by fork/merge survival of shared assets; key rotation and primary-vs-secondary left open, pending a Meadowcap/MLS alignment investigation.

- **Escalation tolerance** clarified as the default value, an implementation-tuning item.

- **Track A vs Track B** deferral confirmed; Keyhive noted as the stronger revocation option, not ruled out; next step is to define revocation needs.

### Citation and cosmetic fixes

RFC 9750 section 3.5 corrected to section 6.4; the Matrix "trusting servers not to lie about the time" line softened from a verbatim quote to a paraphrase; the duplicated Appendix line removed.

### Em-dash removal

Both documents taken from 219 (Part 1) and 343 (Part 2) em-dashes to zero, with replacements chosen by grammatical role: colons for bullet labels and headings, commas for appositives, semicolons for independent-clause joins, parentheses where commas would pile up. Bibliography and attribution artifacts introduced by the pass were caught and fixed.

---

## Deliverables in this folder

**drystone-part1.md** : Part 1, Reasoning Underpinnings. Current. All model corrections applied; zero em-dashes.

**drystone-part2.md** : Part 2, The Certifiable Design. Current. All model corrections applied; zero em-dashes.

**drystone-open-items.md** : The forward-looking read-and-decide document. Part one is a ledger of what the latest rulings settled (confirm these are captured right). Part two is what genuinely remains open (B1 tenure-under-rekey and B3 succession need rulings; B2 and B6 are design investigations; B4 is the priority security item to bring back with test coverage).

**drystone-review-report.md** : The original change-log style review report. Superseded in purpose by the open-items document (you asked to move away from the change-log format), but retained as a historical record of the edits and the primary-source verifications. Still contains em-dashes, since it is a meta-document about the work rather than a spec deliverable.

**drystone-meer-reclassification.md** and **drystone-meer-patches-A.md** : Early-session planning documents for the meer reclassification (Answer A). Historical; their conclusions are now folded into Part 2. Retained for traceability.

---

## What is on the author's plate next

Two rulings to move work: B1 (confirm the tenure-under-rekey test shape) and B3 (is succession in scope for the next revision). Two design investigations to schedule: B2 (Meadowcap/MLS fork-merge alignment) and B6 (define revocation needs). B4 (capped-root test coverage) remains the priority security item. The ENABLING wire encodings gate a publication-final release and are filled in once nomenclature is stable. Several primary sources still carry confirm-before-publish flags.
