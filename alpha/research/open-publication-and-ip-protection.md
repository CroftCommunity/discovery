# Protecting the Croft Spec for Open Sharing — IP Strategy

**Subject:** How to protect a complete Croft protocol specification when the goal is to share it openly — the patent vs. copyright/Creative-Commons vs. defensive-publication question.

**Purpose:** Record the decision and reasoning for how Croft's spec should be licensed and published so that it (1) spreads and gets used freely, and (2) cannot be patented out from under us by a third party. Pairs with [`socialization-and-publication-venues.md`](socialization-and-publication-venues.md), which covers *where* to publish; this doc covers *under what terms* and *why*.

**Date:** 2026-06-16

**Status:** Decision record + general IP practice. **Not legal advice**, and much of it is jurisdiction-specific — anything where a patent option is to be preserved warrants a patent attorney before any public disclosure. The recommended path assumes the defensive intent stated below.

---

## The question

Once Croft is a real, complete spec, what is the best way to protect it for open sharing — patent it, or publish openly under a Creative Commons release?

## The decision fork

Patent and "open publish now" pull in opposite directions. The first real decision is **which posture**:

- **Defensive** — keep it permanently free and unlockable; ensure no one (including us) can use patents to exclude others. The protection we want is *against someone else patenting the design*.
- **Offensive** — retain an exclusionary/commercialization option for ourselves via patents.

**Decision taken: defensive.** The stated goal is for Croft to spread and be used, while preventing a third party from coming along and patenting the design themselves. That is a textbook **defensive publication** scenario. No patent of our own is needed or wanted.

## What each mechanism actually protects

```
                    protects EXPRESSION        protects IDEAS / METHODS
                    (the spec document text)   (the protocol itself)
  ┌───────────────┬──────────────────────────┬──────────────────────────┐
  │ Copyright/CC  │ ✓ (automatic on creation)│ ✗ anyone can implement    │
  │ Patent        │ ✗                        │ ✓ (exclusionary, costly)  │
  │ Defensive pub │ ✗                        │ ✓ blocks OTHERS patenting │
  └───────────────┴──────────────────────────┴──────────────────────────┘
```

The trap: a Creative Commons license protects the *prose* of the spec, not the *ideas*. Anyone can read a CC-licensed spec and build the protocol regardless of the license. CC alone does nothing about the patent threat. Only a patent — or **prior art that blocks patents** — touches the ideas. For a defensive posture, the load-bearing instrument is prior art, not the license.

## The disclosure bar (why the posture must be decided first)

- **US:** a 1-year grace period after public disclosure during which a patent can still be filed.
- **EU, China, most elsewhere:** *absolute* novelty — any public disclosure before filing kills patentability immediately.

So "publish openly today, decide on patents later" is not available without forfeiting essentially all non-US patent rights and starting the US clock. Publish-vs-patent is a one-time, up-front decision. For Croft (defensive) this is fine and intended — but it is why the posture is decided before anything is published.

## What makes prior art actually block a later patent

A disclosure defeats a competitor's patent claim only if it (a) predates their filing and (b) teaches the claimed invention to a person skilled in the field. Two consequences shape how Croft must publish:

1. **Completeness matters.** Vague hand-waving blocks nothing. Each non-obvious mechanism — relay placement, the wire format, the handshake, the lineage/standing membership model, the freshness/no-false-current rule, the visibility/regime model — must be described in enough detail that someone could build it. A mechanism not specifically disclosed can still be patented around us. The thoroughness of the spec *is* the defense. (The crystallized wire spec in `discovery/crystallized/CROFT-PROTOCOL.md` is the right shape for this.)
2. **The date must be provable by a third party**, not just our own git log. An external, tamper-evident, examiner-discoverable timestamp is what actually functions as prior art.

## Recommended defensive-publication stack

1. **Spec document → CC-BY 4.0.** Attribution-only; matches how W3C/IETF-adjacent specs travel. Anyone may redistribute and implement while authorship stays attached.
2. **Reference implementation → Apache-2.0** (not MIT/BSD). Apache-2.0 carries an **express patent grant** plus a retaliation clause; MIT/BSD are silent on patents. For a protocol, that grant is the part that matters.
3. **Establish prior art with a durable, examiner-discoverable timestamp.** In rough order of strength/effort:
   - **IETF Internet-Draft** (`draft-<name>-croft-architecture`) — free, self-serve, no membership/sponsor; produces a dated, examiner-known, publicly archived disclosure that persists after the 6-month expiry. Doubles as a standards-engagement signal. **The single highest-leverage first move.**
   - **arXiv preprint** (cs.NI primary; cross-list cs.CR, cs.DC) — strong combined prior-art + reach + credibility. Endorsement is the friction for an independent author; an academic co-author solves it and also unlocks peer-reviewed venues.
   - **(Optional) formal defensive-publication registry** — Technical Disclosure Commons (free) or IP.com (paid, strongest examiner discoverability) if blocking competitor patents is an explicit goal beyond what the I-D + arXiv timestamps already accomplish.
4. **Make it discoverable.** Examiners search by keyword + classification. Use the terminology a searcher would type, give the spec a stable title/DOI/URL, and cross-link spec ↔ implementation. Prior art nobody can find does not function as prior art.
5. **Re-publish revisions the same way.** Each substantive version needs its own dated public record, or a mechanism added in v2 has no early date.

```
  complete spec ──► CC-BY 4.0  (the document)
       │
       ├──► IETF Internet-Draft ──► dated, examiner-known, persistent  ◄── do FIRST
       ├──► arXiv (cs.NI + cs.CR/cs.DC) ──► prior-art + reach + credibility
       │
  ref impl ──────► Apache-2.0  (express patent grant + retaliation)
```

## The one-directional disclosure caveat

Every step above is a **public disclosure**. The first Internet-Draft or community post starts the US grace clock and forecloses most foreign patent rights immediately. For the defensive posture this is the intended outcome — but the sequencing is one-way: once Croft is socialized, the option to patent it ourselves is effectively gone. This is acceptable here precisely because the goal is spread + use + block-others-patenting, not retained control.

## Cross-references

- **Where to publish / socialize:** [`socialization-and-publication-venues.md`](socialization-and-publication-venues.md) — the per-layer venue map (iroh, MLS WG, W3C CCG, atproto WG, Local-First Conf, etc.), the prioritized sequence, and the same disclosure caveat applied to venue timing.
- **The spec being protected:** `discovery/crystallized/CROFT-PROTOCOL.md` — the wire spec whose completeness is itself the prior-art defense.

## Summary

- Posture: **defensive publication** (spread + use + block-others-patenting). No own-patent.
- License the **document** CC-BY 4.0; license the **code** Apache-2.0 (for the express patent grant).
- The actual patent-blocking instrument is **complete, timestamped, examiner-discoverable prior art** — an **IETF Internet-Draft first**, then an **arXiv preprint**, optionally a defensive-publication registry.
- Completeness of the spec and a third-party-provable timestamp are the two properties that make the prior art bite.
- Disclosure is one-directional; this is intended given the defensive goal. Not legal advice — confirm with counsel if any patent option is to be preserved.
