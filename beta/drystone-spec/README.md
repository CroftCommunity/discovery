# Drystone: an open peer-to-peer protocol for local-first state synchronization

**Version:** 0.1.0 (beta maturity, draft / defensive-publication track)

**Status:** Draft, consolidated (Part 1 = the p10 principles set; Part 2 = the p11 rebuild; document-pass-8):
self-contained and consistent, with the transport/delivery and deep-MLS designs folded in, and the §7.6
reconcile/fork treatment expanded (ten subsections incl. ban-as-fork and the re-plant instantiation
mechanism at §7.6.11). **Pending design review**: the remaining open work is the design decisions and
`ENABLING` wire encodings tracked in Part 2, Appendix B, not editorial cleanup. Build-against shape complete.
(The prior batch-9 p9 consolidation was superseded here after a content-loss audit; the full p10/p11 corpus
is frozen at `../../alpha/seeds/p10-p11-corpus/`.)

**License (text):** CC0 1.0 Universal (Public Domain Dedication).

**License (implementations):** Apache-2.0 (or any).

---

## What this is

Drystone is the protocol. This specification defines it in **two parts**, and nothing above it (no
application layer, no product surface):

```
  ┌─ Part 1 - REASONING UNDERPINNINGS ───────────────────────────┐
  │   The "why." Named design principles, each stating a          │
  │   commitment, its grounded reasoning, and the consequence     │
  │   the mechanics MUST satisfy. A reviewer reads this to         │
  │   understand why the wire looks the way it does.              │
  │                                                               │
  │   P-Local-Truth · P-Knowable-Truth · P-Peer-Equality ·        │
  │   P-Durable-Enablement - and the razor they share.            │
  ├─ Part 2 - THE CERTIFIABLE DESIGN ────────────────────────────┤
  │   The "what." Normative mechanics: data model, identity,      │
  │   rights and capabilities, transport, synchronization and     │
  │   governance-conflict resolution, security, interoperability. │
  │   This is the half an implementation is built and validated   │
  │   against; two independent implementations following it MUST  │
  │   interoperate.                                               │
  └───────────────────────────────────────────────────────────────┘
```

- **Part 1**: `part-1-reasoning-underpinnings.md` (the consolidated "p9" set; document-pass-6)
- **Part 2**: `part-2-certifiable-design.md` (the consolidated "p9" set; transport + MLS folded in;
  identity model at §5.2, open items / `ENABLING` at Appendix B, external-fact confirmations at Appendix C,
  term definitions at Appendix D)
- **Conventions + decisions**: `conventions-and-decisions.md` (the synthesis conventions and the
  cross-session design decisions; includes the vocabulary discipline)
- **Large-group scaling (Part 2 §11)**: the section-length treatment of large-group scaling, dormancy,
  and re-entry, folded into `part-2-certifiable-design.md` as **§11** (cost-scales-on-the-live-set, scaled
  post-compromise security, the hot/cold split, the two-part re-entry credential, the governance chain and
  lineage-scoped bans, delivery under scale, the encryption posture and the two forces at §11.9.1, the
  optional public-projection cache at §11.9.2 and the experimental public-by-default regime above ~7k at
  §11.9.3, tiered performance with a buildable experiment matrix at §11.10.1, the empirical basis at
  §11.13, and the research obligation at §11.14). It builds on Part 2 §7 (Synchronization and
  governance-conflict resolution), §5, and §6. It was filed at document-pass-9 as a standalone companion
  titled "§7", then folded to §11 at document-pass-10 to resolve the collision with the existing §7; the
  byte-identical original is frozen at `../../alpha/seeds/large-group-scaling-batch11/eleven.zip`. (Former
  open thread T37, now resolved.)
- **Research prompt**: `research-prompt-operational-rates.md` (the companion Part 2 §11.14 points to: the
  standing research task to quantify the three per-group operational rates from ancillary evidence, with
  its source-tiering and anti-fabrication guardrails). Its research *output* (the centered-platform survey)
  is distilled into `../fenced/`; the raw report is preserved as a transcript.
- **Changelogs**: `CHANGELOG.md` (filing-side revision log, document-pass-0..10, newest first) plus
  `part-1-changelog.md` and `part-2-changelog.md` (the consolidation's detailed per-part content-pass logs)

**Superseded companions** are moved to **`superseded/`** and kept as provenance, marked superseded and
treated as raw transcripts would be (do not read as current). Their content is folded into the consolidated
spec: `persona-definition.md` → §5.2 + Appendix D; `open-items.md` → Appendix B + `open-threads.md`;
`bounded-contexts-and-vocabulary.md` → `conventions-and-decisions.md`; `review-handoff.md` (predates the
persona migration); the two SVGs (Part 2 no longer references them by figure number). See
`superseded/README.md`.

Each Part 2 section names the Part 1 principle(s) it `Realizes`, so the two halves are traceable
against each other: every mechanic should cash out a principle, and every principle should bind a
mechanic. A principle that binds no mechanic does not belong in Part 1; a mechanic that realizes no
principle is unexplained.

This document is **not** the narrative of how Drystone was designed and proved; that account is theme
`04` of the Croft project's discovery synthesis, which reads as a story of the design process. The spec
is the vendor-neutral artifact you build against; `04` is one project's account of why it believes the
protocol works.

**Drystone is the protocol; Croft is an ecosystem built on it.** They must not be conflated. Drystone is
intended to carry more than one ecosystem; Croft is the first, comprising (at least) a Croft-branded
application, and a Drystone-compliant cooperative hosting operator that participates as an ordinary
`principal` / `Group Role Set`. The protocol text is CC0 and names no ecosystem in its normative content. (Stewardship
of the protocol's IP and marks is intended to sit with an independent foundation, candidate name *Noria*,
pending clearance, a project decision not settled by this spec.)

## Notice of defensive publication and open implementation

This document is intended as a public, timestamped disclosure of the Drystone protocol and its
mechanisms, published deliberately as prior art. The author intends the protocol designs, methods,
message formats, and synchronization mechanisms described herein to be freely available for anyone to
implement, use, and build upon, without restriction and without royalty. The text is released under
CC0 1.0 Universal; the author waives copyright in it, makes no patent claim over the mechanisms
disclosed, and asserts no patent rights against any implementer. Nothing here grants rights under any
third party's patent; implementers are responsible for their own due diligence.

> **Sequencing (do not mint a DOI off this draft).** The defensive disclosure becomes *enabling*, and
> therefore protective as prior art, only once a skilled implementer can build the synchronization layer
> from the text alone. Part 2 §7.2 (message formats) is not yet field-by-field, and the `ENABLING`
> encodings in Part 2 Appendix B are open. Mint the v0.1 archival DOI off the first version where those
> are closed, not off this one. (Spec-text license `CC0 1.0`; reference-code license `Apache-2.0`.)

## How to read the normative text

- Normative keywords **MUST / MUST NOT / SHOULD / SHOULD NOT / MAY** are to be read as in BCP 14
  (RFC 2119 / RFC 8174) when in all capitals.
- **Proof / status flags** travel with each carried claim: `green-real` (demonstrated with real
  crypto/transport), `green-model` (proven in the reference model), `design` (specified, not yet
  proven), `ENABLING` (a byte-level encoding that must be pinned before two implementations can
  interoperate; these gate a publication-final release). Where a claim rests on an external fact not
  yet independently confirmed, it carries **[confirm before publish]**.
- **External facts.** For atproto / iroh / iOS facts, the source of truth is the FACTCHECK SoT (iroh
  `1.0.0`); cite it, do not re-verify. Several comparative claims about other protocols (Matrix State
  Resolution, Willow, Meadowcap, Keyhive) are load-bearing for the design rationale and carry
  **[confirm before publish]** until verified against their primary sources.

## Terminology (defined once, used consistently)

| term | meaning |
|---|---|
| **principal** | The genus of the identity model: a role-holding entity identified by one key-lineage (Part 2 §5.2). A persona or a group is a principal. Rights, roles, and weight attach to principals. |
| **persona** | The entity a human is manifested as: a principal that folds one human's clients and devices to a single identity by lineage (Part 2 §5.2; `persona-definition.md` is the vocabulary of record). One human may hold several personae across systems. Personae differ in *capabilities*, never in *rights* or *standing*. |
| **peer** | The **relation** of equal standing across an edge, not the entity (Part 2 §3.1, §5.2). *Peer* names the relation; the entity in relation is a persona. The word also carries a distinct transport-plane sense (peer = iroh EndpointId, §6.1); see `bounded-contexts-and-vocabulary.md`. |
| **client / device** | A **client** is software that is a group member: one MLS leaf, one signature key, one credential (Part 2 §5.2). A **device** is hardware (a node, §5.4). The hosting chain is human → devices → clients, folded to one persona; client-count and device-count are not persona-count (§4.5). |
| **scope** | The unit of shared state a set of principals holds in common (the "common pasture"); a group. A principal's own local store is its private croft. |
| **right** | A claim every principal is entitled to, universal and never delegated (Part 2 §5.3). |
| **capability** | A delegated, additive, revocable means to *do* something, sitting on top of the rights floor (Part 2 §5.4). |
| **role** | The descriptive, free-composing set of capabilities a principal holds right now (Part 2 §5.5). |
| **Group Role Set** | A named, pinned, Group-recognized bundle of Group Roles and the capabilities they imply, with a required *and* forbidden composition, drift-checked against the governance log (Part 2 §5.5). The compound is always the defined concept, never the ordinary "set." (Formerly "PrincipalSet," earlier "PeerSet.") |
| **governance fact** | A signed, append-only entry recording a governance decision (admit / expel / grant / revoke / amend). Authority is a fold over these (Part 2 §7). |
| **meer** | Infrastructure, **not** a principal: a blind store-and-forward node offering availability capacity, configured by a scope to serve ciphertext (Part 2 §5.4, §6). Holds no rights, no role, no identity; the legacy labels "blind peer / blind member" are retired. |
| **fork / re-formation** | A persona or minority's lossless-in-rights exit into a differently-shaped scope, preserving history and provenance. The backstop that makes delegation exitable. |

## Provenance of this synthesis

Part 1 is the reasoning layer the design rests on. Part 2 is matured from the proven protocol mechanics
and the governance/peer-model drafts. Verification flags travel inline; the full source-to-section
trace lives in the prior-stage rollup ledger, per the maturity discipline, not in this document.
