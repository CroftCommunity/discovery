# Drystone consolidation: session summary (p9 doc set)

`Status: working notes`

`Scope: summarizes the consolidation, verification, and pre-publication-review work that produced the p9 doc set; not normative`

`Companion to: p9-drystone-part1-principles.md, p9-drystone-part2-mechanics.md, p9-drystone-conventions-and-decisions.md, p9-drystone-doc-method.md, and the two p9 changelogs`

---

## What the p9 doc set is

The **p9** prefix marks the current, most-recent deliverable set: fully consolidated, internally and cross-document consistent, hygiene-clean, with every checkable external fact verified against a primary. The only work these documents still await is **design review** (the design decisions the specs themselves flag as open), not further editing or fact-checking.

The set:

- **p9-drystone-part1-principles.md** (975 lines): the "why", the reasoning underpinnings. The razor (compute provenance, never utility), the four principles (P-Local-Truth, P-Knowable-Truth, P-Peer-Equality, P-Durable-Enablement), the forced terminus (fork, not verdict), and the field-integrity bridge. Prose, principle-level.

- **p9-drystone-part2-mechanics.md** (3961 lines): the "what", the mechanics. The three-plane delivery model (Carriage / Durability / Presence), the MLS substrate treatment, the governance-resolution fold, the fork-as-re-plant primitive, the rights/roles/capability model, history modes, and the full appendices (open items, prior art, upstream reference links).

- **p9-drystone-conventions-and-decisions.md**: the terminology conventions (group/Group, role/Group Role, persona, principal, scope, meer, Group Role Set) and the synthesis decisions, the primer that keeps the two specs disciplined.

- **p9-drystone-doc-method.md**: the writing method (Rules 1-13), including the end-state-not-how-thinking-went discipline and the fold-boundary obligation.

- **p9-drystone-part1-changelog.md**, **p9-drystone-part2-changelog.md**: the full edit history.

---

## How the set got to p9

The path, most recent last:

1. **Consolidation.** A twelve-document working suite was folded into the two self-contained parts plus the primer and doc-method. Four editing passes (terminology, technical-reality fold, narrative-actor consistency, requirement/definition/realization).

2. **Flow and Rule 1 enforcement.** All past-reasoning and prior-state-contrast constructions removed, so the docs read as end-state, not as a record of how the thinking moved.

3. **Primary-source verification.** The transcript-hash formula (RFC 9420 §8.2), resumption PSK (§8.6), the epoch-number metadata leak (RFC 9750), the Matrix Project Hydra facts, and CALM all read verbatim against their primaries.

4. **Cross-document consistency read.** Principle labels, cross-references, and the shared fork/rights/field-integrity concepts confirmed coherent across both parts.

5. **Versioned upstream reference links.** A link-resolvable, version-pinned reference section added to the bottom of both docs so an implementer finds the exact version each claim was written against.

6. **Cross-reference disambiguation.** Bare section numbers were ambiguous about which document they pointed to. All 41 bare Part-1 references in Part 2 (`§2.x`) and all 46 bare Part-2 references in Part 1 (`§4`-`§8`) were prefixed. Zero dangling cross-references remain in either direction.

7. **Pre-publication verification pass.** Every remaining checkable `[confirm]` item was worked against a primary and cleared only where a primary grounded it (see below).

---

## Verified this final pass (the checkable `[confirm]` items)

- **MSC4297** conflicted-subgraph mechanism (adopted at Part 2 §7.5.2): verbatim against the Matrix State Res v2.1 implementer's guide. Two changes: start the iterative auth checks from the empty set, and replay the conflicted state subgraph (the SCC containing the contracted conflicted supernode, via forward-backward reachability intersection).

- **CVE-2025-49090** root cause: verbatim, a state reset to an earlier/incorrect value absent a validly-producing event, fixed by State Res v2.1.

- **RBSR**: arXiv:2212.13567 (Meyer), peer-reviewed at SRDS 2023, pp. 59-69.

- **CRDT**: SSS 2011 (LNCS 6976, pp. 386-400, DOI 10.1007/978-3-642-24550-3_29) plus INRIA RR-7506 (Jan 2011), distinct from the later RR-7686.

- **RFC 8446** record padding and the traffic-analysis residue: verbatim (§5.4), corroborated by arXiv:2406.15686 §6.2.

### Two corrections made this pass

- **RoQ is not RFC 9714.** RTP-over-QUIC is still an Internet-Draft (`draft-ietf-avtcore-rtp-over-quic`, -14), no RFC number yet. The earlier "RFC 9714" citation was wrong and is corrected to the draft, with a note that the media path rides a not-yet-final draft.

- **Sigstore** was loosely called "countersigning." Corrected to name its actual primitive (Rekor, an append-only Merkle transparency log with signed tree-head checkpoints); Drystone draws the checkpoint/Merkle-consistency analogy, not a literal Sigstore feature.

---

## The known open threads (what "pending design review" means)

These are the items the specs themselves track as open. None is an editing loose end; all are design decisions or genuinely-moving external pins.

### Design decisions (need a person, not a pass)

- **Track A vs Track B capability mechanism**: Meadowcap-shaped delegated tokens vs Keyhive-shaped convergent membership graphs. They differ on revocation immediacy. (Part 2 §7.2, Appendix B.)

- **Root-authority succession**: how founding authority transfers when founders leave. The most dangerous operation in the system; deferred to a Lifecycle section. (Part 2 §7.3.)

- **Capped-vs-uncapped-root soundness**: the priority open security item. The Matrix facts it rests on are verified; what is open is which coverage the Drystone test suite actually exercises (a validation item). (Part 2 §7.3, Part 1 §2.3.)

- **History-mode size bound**: the practical size separating forward-only from Willow-mutable mode, an unmeasured engineering estimate. (Part 2 §7.7.)

- **Mode migration**: whether a Group can migrate between forward-only and Willow-mutable modes, or the mode is fixed at creation (suspected fixed). (Part 2 §7.7.)

- **Self-destruct specification**: selector signaling, offline-past-T delivery semantics, mask-vs-remove, and whether client build profile is part of the legibility surface. (Part 2 §7.7.3, §6.8.4.)

- **Tier-2 side history, feature or data-model note**: whether a separate-but-inherited side history is a first-class named construct or an emergent use of the data model. The central undecided history-structure question. (Part 2 §7.8.)

- **Tier-2 to tier-3 promotion**: whether a side history can be promoted to a subgroup later without losing history, and whether tier-1 subids and tier-2 side histories should share an addressing scheme. (Part 2 §7.8.)

- **The open rights check**: whether the §7 survivor/re-key path can strand `tenure`. Gates freezing the rights set. (Part 2 §5.3.)

- **False-positive escalation tolerance**: the byte-level definition of the exposed signals and the policy-fact format for the governed tolerance. (Part 2 §7.4.1.)

- **`ENABLING` wire encodings**: the byte-level encodings that gate a publication-final version, canonical governance-fact encoding, frontier-commitment construction, the returning-member cursor, the capability wire format. (Part 2 §7.3.1, §7.5, §4.2, §6.6.2, Appendix B.)

- **Hash-function and vendor-naming reconciliation**: the single committed suite (SHA-256 vs BLAKE3) and the vendor-neutral tag namespace, both real wire changes that re-open the §4 signature proofs. (Part 2 Appendix B.)

### MLS hard-case residuals (specific stress-tests, reasoned not proven)

- **External-join far-behind node** (§7.4.2), **in-place secret restore** (§7.4.2), **re-plant intent ordering** (§7.6.3), **KeyPackage-exhaustion seating trilemma** (§7.4, §7.6), **re-plant seating default at a boundary** (§7.6.2), **epoch_authenticator overlap** (§10.2.1), **resumption-PSK cross-group linking** (§10.2.1).

### External pins that genuinely move (correctly left `[confirm]`)

- **Pre-1.0 iroh crate versions**: `iroh-gossip`, `iroh-mainline-address-lookup`, `iroh-mdns-address-lookup`. The production profile must pin these; iroh core itself is 1.0 and verified.

- **DMLS working-group draft revision**: `draft-kohbrok-mls-decentralized-mls`, tracked not depended on.

- **Pkarr spec** (self-signed-record integrity model) and **Keyhive canonical location** (Ink & Switch).

### Print-edition wording confirmations (Part 1 lineage)

- The verbatim wordings of Lamport, CAP, Mill, Hayek, Ashby, Beer ("specify only somewhat"), Popper, Ostrom (principles 6 and 7), and the Modular Politics quotations, to be pulled from the specific print editions before external publication. Identifiers and statements are correct; only the exact quoted wording is pending.

---

## State at p9

Both specs are hygiene-clean (no em-dashes, no double-hyphens in prose, no legacy "PrincipalSet," blank lines between markdown bullets), cross-references are disambiguated in both directions, and no checkable external fact-verification marker remains open. Every `[confirm]` still in the documents is a design decision, a moving external pin, or a print-edition wording confirmation, exactly the set that belongs to design review and publication, not to further editing.
