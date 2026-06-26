# Drystone — an open peer-to-peer protocol for local-first state synchronization

**Version:** 0.1.0 (beta maturity — draft / defensive-publication track)

**Status:** Draft. Build-against shape complete; the byte-level `ENABLING` encodings that gate a
publication-final release are listed in Part 2, Appendix B, not yet pinned.

**License (text):** CC0 1.0 Universal (Public Domain Dedication).

**License (implementations):** Apache-2.0 (or any).

---

## What this is

Drystone is the protocol. This specification defines it in **two parts**, and nothing above it (no
application layer, no product surface):

```
  ┌─ Part 1 — REASONING UNDERPINNINGS ───────────────────────────┐
  │   The "why." Named design principles, each stating a          │
  │   commitment, its grounded reasoning, and the consequence     │
  │   the mechanics MUST satisfy. A reviewer reads this to         │
  │   understand why the wire looks the way it does.              │
  │                                                               │
  │   P-Local-Truth · P-Knowable-Truth · P-Peer-Equality ·        │
  │   P-Durable-Enablement — and the razor they share.            │
  ├─ Part 2 — THE CERTIFIABLE DESIGN ────────────────────────────┤
  │   The "what." Normative mechanics: data model, identity,      │
  │   rights and capabilities, transport, synchronization and     │
  │   governance-conflict resolution, security, interoperability. │
  │   This is the half an implementation is built and validated   │
  │   against; two independent implementations following it MUST  │
  │   interoperate.                                               │
  └───────────────────────────────────────────────────────────────┘
```

- **Part 1** — `part-1-reasoning-underpinnings.md`
- **Part 2** — `part-2-certifiable-design.md`

Each Part 2 section names the Part 1 principle(s) it `Realizes`, so the two halves are traceable
against each other: every mechanic should cash out a principle, and every principle should bind a
mechanic. A principle that binds no mechanic does not belong in Part 1; a mechanic that realizes no
principle is unexplained.

This document is **not** the narrative of how Drystone was designed and proved — that account is theme
`04` of the Croft project's discovery synthesis, which reads as a story of the design process. The spec
is the vendor-neutral artifact you build against; `04` is one project's account of why it believes the
protocol works.

**Drystone is the protocol; Croft is an ecosystem built on it.** They must not be conflated. Drystone is
intended to carry more than one ecosystem; Croft is the first, comprising (at least) a Croft-branded
application, and a Drystone-compliant cooperative hosting operator that participates as an ordinary
`Peer` / `PeerSet`. The protocol text is CC0 and names no ecosystem in its normative content. (Stewardship
of the protocol's IP and marks is intended to sit with an independent foundation — candidate name *Noria*,
pending clearance, a project decision not settled by this spec.)

## Notice of defensive publication and open implementation

This document is intended as a public, timestamped disclosure of the Drystone protocol and its
mechanisms, published deliberately as prior art. The author intends the protocol designs, methods,
message formats, and synchronization mechanisms described herein to be freely available for anyone to
implement, use, and build upon, without restriction and without royalty. The text is released under
CC0 1.0 Universal; the author waives copyright in it, makes no patent claim over the mechanisms
disclosed, and asserts no patent rights against any implementer. Nothing here grants rights under any
third party's patent; implementers are responsible for their own due diligence.

> **Sequencing (do not mint a DOI off this draft).** The defensive disclosure becomes *enabling* — and
> therefore protective as prior art — only once a skilled implementer can build the synchronization layer
> from the text alone. Part 2 §7.2 (message formats) is not yet field-by-field, and the `ENABLING`
> encodings in Part 2 Appendix B are open. Mint the v0.1 archival DOI off the first version where those
> are closed, not off this one. (Spec-text license `CC0 1.0`; reference-code license `Apache-2.0`.)

## How to read the normative text

- Normative keywords **MUST / MUST NOT / SHOULD / SHOULD NOT / MAY** are to be read as in BCP 14
  (RFC 2119 / RFC 8174) when in all capitals.
- **Proof / status flags** travel with each carried claim: `green-real` (demonstrated with real
  crypto/transport), `green-model` (proven in the reference model), `design` (specified, not yet
  proven), `ENABLING` (a byte-level encoding that must be pinned before two implementations can
  interoperate — these gate a publication-final release). Where a claim rests on an external fact not
  yet independently confirmed, it carries **[confirm before publish]**.
- **External facts.** For atproto / iroh / iOS facts, the source of truth is the FACTCHECK SoT (iroh
  `1.0.0`); cite it, do not re-verify. Several comparative claims about other protocols (Matrix State
  Resolution, Willow, Meadowcap, Keyhive) are load-bearing for the design rationale and carry
  **[confirm before publish]** until verified against their primary sources.

## Terminology (defined once, used consistently)

| term | meaning |
|---|---|
| **peer** | A participant. There is exactly one kind (Part 2 §5.2). Peers differ in *capabilities*, never in *rights* or *standing*. |
| **device** | A keypair-holding endpoint. Several devices may act under one peer's lineage. |
| **scope** | The unit of shared state a set of peers holds in common (the "common pasture"); a group. A peer's own local store is its private croft. |
| **right** | A claim every peer is entitled to, universal and never delegated (Part 2 §5.3). |
| **capability** | A delegated, additive, revocable means to *do* something, sitting on top of the rights floor (Part 2 §5.4). |
| **role** | The descriptive, free-composing set of capabilities a peer holds right now (Part 2 §5.5). |
| **PeerSet** | A named, pinned, group-recognized bundle of capabilities with a required *and* forbidden composition, drift-checked against the governance log (Part 2 §5.5). The compound word is always the defined concept, never the ordinary "set." |
| **governance fact** | A signed, append-only entry recording a governance decision (admit / expel / grant / revoke / amend). Authority is a fold over these (Part 2 §7). |
| **meer** | An always-on, governed, blind helper peer — formally a PeerSet (`floor + requires{availability} + forbids{read}`). |
| **fork / re-formation** | A peer or minority's lossless-in-rights exit into a differently-shaped scope, preserving history and provenance. The backstop that makes delegation exitable. |

## Provenance of this synthesis

Part 1 is the reasoning layer the design rests on. Part 2 is matured from the proven protocol mechanics
and the governance/peer-model drafts. Verification flags travel inline; the full source-to-section
trace lives in the prior-stage rollup ledger, per the maturity discipline — not in this document.
