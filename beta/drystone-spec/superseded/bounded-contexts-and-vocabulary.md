# Bounded contexts and the Drystone vocabulary

`Status: spec-layer design/language note. Not normative protocol; it governs how the spec talks about
itself. Lives at the spec layer because it is about the ubiquitous language of the design, not about the
principles (philosophy) or the world-manifestation (governance).`

## The problem it answers

The same word keeps looking correct in more than one place while meaning different things. *Peer* is the
clearest case: at the transport plane a peer is a network endpoint (an iroh EndpointId, the identity of a
channel); at the governance/relational level a peer is a member-as-relation, the standing one persona
holds toward another. Both usages are right. The question is whether that is a defect to eliminate or a
structure to manage.

## The answer, from Domain-Driven Design

It is a structure to manage, and DDD names it directly. The relevant concepts (Eric Evans):

- **Bounded context.** A defined part of the design within which particular terms, definitions, and rules
  apply in a consistent way. A term has one precise meaning *inside* a context; the same term may mean
  something different in another context.
- **Ubiquitous language.** Within one bounded context, a term has one shared meaning used identically in
  prose, diagrams, and code. The language is ubiquitous *within a context*, not across the whole system.
  Evans' official three-point framing ends: "Speak a ubiquitous language within an explicitly bounded
  context." The word *explicitly* is load-bearing.
- The standard illustrations are exactly this shape: *account* meaning a login for IT and a bank account
  for finance; *claim* meaning one thing in insurance and another in legal. *Peer* as transport endpoint
  vs *peer* as governance relation is the same pattern, and the literature is explicit that these
  differences are acceptable and even necessary.

So term overload across genuinely separate contexts is DDD working as intended, not a modeling failure.
Forcing one word to mean one thing everywhere produces a worse model: either a bloated concept that
serves no context well, or false agreement where two parts say "peer" and quietly mean different things.

## The test (overload vs. rename)

Overload is safe only when the two meanings live in genuinely separate contexts that rarely share a
sentence. The failure mode is not the two definitions existing; it is both senses leaking into a single
readable unit (one module, one doc, one paragraph) where the reader has to guess which is meant. That is
not bounded-context overload; it is plain ambiguity, and it is a real cost.

**The test:** is each sense quarantined to its own context, with an explicit translation step at the
border, or do both senses have to coexist in the same readable unit?

- Quarantined, translated at the seam: **keep the overload.** DDD as intended.
- Both senses in one readable unit: **rename one.**

This is exactly why the *peer* -> *persona* migration (spec document-pass-4) was the right move and not a
retreat from the word *peer*. When the relational sense had to coexist with the transport sense in shared
prose, the entity sense was renamed to **persona** so *peer* could keep naming only the relation. The
rename was the test firing, not a rejection of the bounded-context principle.

## What this asks of the spec

- Name the contexts. The two live ones so far: the **transport plane** (peer = EndpointId; see Part 2 §6.1)
  and the **governance/relational** context (peer = the relation between personae; persona = the entity;
  see Part 1 §2.3, Part 2 §5.2, `persona-definition.md`).
- Translate at the seam, never silently adopt. Where one context's term crosses into another (an
  anti-corruption layer, in DDD terms), the mapping is explicit. Part 2 §6.1's whole point is the seam: a
  verified channel from a peer-endpoint grants nothing at the governance plane; membership is a separate,
  later check.
- Apply the test before adding a term or reusing one. If a new usage would force two senses into one
  readable unit, rename rather than lean on context.

## Provenance

DDD bounded-context / ubiquitous-language framing per Eric Evans (the 2014 reference definition and the
2019 DDD Europe keynote framing of a bounded context as the place where terms apply consistently).
Grounded against primary/near-primary sources in the originating session
(`../../alpha/seeds/transcripts/raw/drystone-transport-integration-and-ddd-2026-07-06.md`). One open
judgment the sources give the axis for but not the answer: whether two work-surfaces are two *bounded
contexts* or two *subdomains under one context* is decided by where the language actually breaks (if the
same reader disambiguates *peer* in one breath, they are effectively one context and you rename; if they
are genuinely separate surfaces with their own vocabularies, the overload is legitimate).
