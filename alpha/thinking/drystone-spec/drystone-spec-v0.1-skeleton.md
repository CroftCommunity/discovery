<!--
Working copy of the Drystone spec v0.1 SKELETON generated in the 2026-06-24
publication/defensive-disclosure dialogue
(../../seeds/transcripts/raw/drystone-publication-defensive-disclosure-dialogue-2026-06-24.md).

STATUS: DRAFT / scaffold only. This is the overall document scaffold (front matter +
defensive-publication notice + §1–§9 + appendices). The deep section drafts that fill
its §5 (identity/capabilities) and §X-equivalent governance live in
section-2-peers-rights-capabilities.md and section-x-governance-conflicts.md (T1).
The `[Your name]` / DOI / URL placeholders are intentional template fields.

Sequencing note (from the dialogue): do NOT mint the v0.1 Zenodo DOI off this skeleton.
Mint it off the first version where Section 7 (Synchronization) can be implemented from
the text alone — that is when the defensive disclosure becomes enabling. Until 7.2's
message formats are field-by-field, that part is not yet protected as prior art.
-->

# Drystone

**Title:** Drystone — An Open Peer-to-Peer Protocol for Local-First State Synchronization

**Version:** 0.1.0

**Status:** Draft / Defensive Publication

**Author:** [Your name]

**Date:** [ISO date]

**DOI:** [reserved Zenodo DOI]

**Canonical source:** [GitHub release URL]

**License (text):** CC0 1.0 Universal (Public Domain Dedication)

**License (implementations):** [e.g. Apache-2.0, or "any"]

---

## Notice of Defensive Publication and Open Implementation

This document is a public, timestamped disclosure of the Drystone protocol and
its mechanisms. It is published deliberately as prior art.

The author intends the protocol designs, methods, message formats, and
synchronization mechanisms described herein to be freely available for anyone to
implement, use, and build upon, without restriction and without royalty.

To the fullest extent permitted by law, the text of this specification is
released under the Creative Commons CC0 1.0 Universal Public Domain Dedication.
The author waives all copyright and related rights in the text.

The author makes no claim to any patent over the mechanisms disclosed in this
document, and asserts no patent rights against any party for implementing them.
This disclosure is intended to establish the described mechanisms as prior art
as of the publication date, such that they cannot be subsequently patented in a
manner that would restrict their open implementation.

Nothing in this document grants any right under any patent held by any third
party. Implementers are responsible for their own patent due diligence.

This document is preserved at the DOI above and may be independently verified by
its cryptographic timestamp.

---

## 1. Introduction

[Land the motivating problem hard and fast. One or two paragraphs. What breaks in
the centralized / server-mediated model, and what local-first restores. End
pointed at: "therefore the protocol must..." This is where a skeptical reader
decides whether to keep going, so lead with the concrete problem, not the
philosophy.]

### 1.1. Scope

[State plainly that this document specifies Drystone the protocol, and nothing
above it. No governance, no application layer. This scoping is what keeps the
principles tethered to mechanics rather than drifting into political theory.]

### 1.2. Terminology

[The key words MUST, MUST NOT, SHOULD, MAY etc. are to be interpreted as in
BCP 14 (RFC 2119 / RFC 8174) when in all capitals. Then a short table of
Drystone-specific terms: peer, entry, namespace, capability, payload, etc. Define
these once, here, and use them consistently. Precise terminology is itself part
of enabling disclosure.]

---

## 2. Design Principles

[This section is allowed to be substantial and prose-heavy. It is the "why." Each
principle states a commitment, gives the grounded reasoning, and ends pointed at a
consequence the mechanics must satisfy. Name each principle, because later
sections will reference them by name.]

### 2.1. Local State as the Unit of Truth

[The reasoning. What it means for each peer's local state to be authoritative.
What "truth" is in a system with no central authority. End with the consequence:
e.g. "Therefore the protocol MUST NOT depend on any peer holding privileged or
canonical state."]

### 2.2. Knowable Truth

[What "knowable" means and what it costs. What a peer can verify for itself versus
what it must take on trust. The honesty about cost is what makes this credible.
End with the consequence for verifiability and cryptographic self-description.]

### 2.3. Peer Equality in Rights

[State this as a rights claim, with the reasoning. No peer holds structural
authority another lacks. Be precise about what equality of rights does and does
not mean (it is not equality of capability or resources). End with the
consequence: what the wire protocol must guarantee so that equality is enforced
by mechanism, not convention. The concrete rights-vs-capability distinction this
rests on is drafted in ../rights-vs-capabilities-definitions.md.]

### 2.4. [Further principle if needed]

[Add only if a mechanic later requires a premise not covered above. Resist adding
principles that do not cash out into a mechanic.]

---

## 3. Protocol Overview

[A short, concrete map of how the pieces fit before the detailed mechanics. A peer,
its local store, how two peers establish a session, how state moves. One diagram
if it helps. This orients the reader so Section 4+ is not read cold. Keep it
descriptive, not normative.]

---

## 4. Data Model

> Realizes: 2.1 (Local State as the Unit of Truth), 2.2 (Knowable Truth)
>
> [ENABLING: This section must let a skilled implementer represent and verify
> state identically to any other implementation. Define structure precisely.]

### 4.1. Entries and Identifiers

[What the atomic unit of state is. How it is named and addressed. How identity of
an entry is derived (e.g. content addressing). Exact enough to reproduce.]

### 4.2. Namespaces and Organization

[How entries are grouped and scoped. The addressing scheme.]

### 4.3. Authentication and Integrity of State

[How a peer verifies an entry without a central authority. Signatures, hashes,
what is signed over. This is where 2.2 becomes concrete.]

---

## 5. Identity and Capabilities

> Realizes: 2.3 (Peer Equality in Rights)
>
> [ENABLING: This section must specify exactly how rights are represented and
> checked on the wire, so that equality is enforced by the protocol rather than
> assumed. This is the section a reviewer will press hardest on. Deep draft:
> section-2-peers-rights-capabilities.md.]

### 5.1. Peer Identity

[What an identity is and is not. Note the deliberate minimalism if you follow the
Willow lineage of making no assumption about what an identity is.]

### 5.2. Capabilities and Access

[How permission to read or write is represented, granted, and verified. How this
is symmetric across peers such that no peer is structurally privileged.]

---

## 6. Transport

> Realizes: [link to relevant principle, likely 2.1 / 2.3]
>
> [ENABLING: Specify or normatively reference the transport precisely. If built on
> iroh/QUIC, cite the exact specs and state what Drystone adds on top.]

### 6.1. Connection Establishment

[How two peers reach and authenticate each other. Reference iroh/QUIC where you
inherit behavior; specify only Drystone's additions.]

### 6.2. Session Lifecycle

[Open, exchange, close. Error and interruption handling.]

---

## 7. Synchronization

> Realizes: 2.1 (Local State as the Unit of Truth), 2.2 (Knowable Truth)
>
> [ENABLING: This is the core defensive-disclosure section. The sync method is the
> most patent-sensitive mechanism and therefore the one that most needs complete,
> reproducible detail. Two independent implementations following this section MUST
> interoperate. Specify the algorithm, the messages, and the ordering. This is the
> section that gates minting the v0.1 DOI.]

### 7.1. Reconciliation Model

[How two peers determine what differs between their states. The Willow/range-based
set reconciliation lineage goes here if that is your approach.]

### 7.2. Message Formats

[Exact wire formats. Field by field. Encoding. This is the heart of enabling
disclosure — vague description here protects nothing.]

### 7.3. Conflict and Convergence

[What happens when peers hold divergent state. The convergence guarantee and how
it is achieved. Eventual consistency semantics.]

### 7.4. Deletion

[If following the Willow lineage of true destructive deletion, specify it here. How
deletion propagates and how it interacts with convergence.]

---

## 8. Security Considerations

[Required in any serious protocol spec. Threat model, what Drystone defends against
and what it explicitly does not. Be honest about residual risks. A reviewer in the
iroh/local-first community will read this section closely.]

---

## 9. Interoperability

[Answer the question the Willow launch crowd asked directly: what does it mean for
two implementations to both be "Drystone-compatible," and where exactly does the
spec force them to agree? Point to the normative sections that guarantee it. This
is where "peer equality in rights" gets shown to be enforced by mechanism.]

---

## Appendix A. Alternatives Considered

[The roads not taken, in the IKEv2-rationale tradition. For each major design
choice, what else was on the table and why you chose as you did. Highly valued in
this culture, and it strengthens the defensive disclosure by documenting the design
space.]

## Appendix B. [Open Questions]

[Honest list of what is unresolved at v0.1. Since the mechanics are still forming,
this appendix is where the forming bits live without weakening the normative
sections. Move items up into the body as they firm up across versions.]

## References

[Normative references: BCP 14, iroh, QUIC (RFC 9000), Willow, etc. Separate
normative from informative.]
