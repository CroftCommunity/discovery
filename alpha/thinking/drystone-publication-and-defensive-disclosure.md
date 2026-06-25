# Drystone publication & defensive disclosure — how the spec goes out and stays open

date: 2026-06-24

source: distilled from the 2026-06-24 publication/defensive-disclosure dialogue
(`../seeds/transcripts/raw/drystone-publication-defensive-disclosure-dialogue-2026-06-24.md`,
cleaned-paste). The companion spec scaffold is
`drystone-spec/drystone-spec-v0.1-skeleton.md`.

purpose: capture the settled *how* of putting Drystone into the world — document shape, feedback venue, and
the defensive-publication mechanism that keeps the protocol unenclosable — distinct from `drystone-spec/`
(the protocol content / T1) and from the *internal* IP story in beta `07` Pillar B.

> **Scope discipline (carried from the dialogue):** Drystone the protocol only — no governance, no
> application layer. The philosophy in the spec stays tethered because local state, knowable truth, and peer
> equality are claims about what the protocol *can and cannot guarantee*, not free-floating political theory.

> **External-fact caveat:** the IETF/RFC-process facts, RFC numbers (2360, 6762, 6709, 2119/8174, 9000), the
> IKEv2 rationale-draft precedent, Zenodo specifics, OpenTimestamps, arXiv, and IETF-Trust draft-reuse terms
> below are **dialogue/web-sourced 2026-06-24 and not independently re-verified** — `[UNVERIFIED]` pending a
> primary-source pass. iroh facts cite the FACTCHECK SoT (iroh `1.0.0`; iroh-docs = range-set reconciliation).

## 1. Document shape — one document, principles up front, mechanics as the outcome

The dialogue resolved the structure question. There were three options:

1. one document, rationale in appendices (the RFC 6762 model — good when the "why" is moderate);
2. **one document, a substantial Design Principles section up front, then mechanics** (good when the
   philosophy is the connective tissue — you can't understand a mechanic without first accepting the premise);
3. two documents, a separate architecture/rationale doc + spec (the IKEv2 model — good when the "why" is a
   reference in its own right or generalizes beyond the protocol).

**Decision-direction: option 2.** Because Drystone's "why" is *generative*, not retrospective — the reasoning
(local state / knowable truth / peer equality) *produces* the design in front of the reader — splitting it
into a separate doc would force a reader to hold two files open to understand one idea (the IKEv2 split made
sense only because its rationale was retrospective, "why we changed this from v1"). So: a single document,
**Introduction → Design Principles → Mechanics (each mechanic traceable back to a named principle) →
Appendices (alternatives / roads-not-taken)**. Terminology, RFC-2119 keywords, and a Drystone term table go
in §1.2; extended rationale and the still-forming mechanics go in appendices so the normative body stays
tight.

**The cost to manage:** the IETF reflex distrusts a long philosophical prelude before any wire format appears
("where's the protocol"). The mitigation is not to cut the philosophy but to make the Introduction land the
*motivating problem* hard and fast, so a skeptical reader reaches the principles already wanting them. Design
rationale is a *recommended* technique in this tradition (RFC 2360), not a tolerated one; the IKEv2 rationale
draft and RFC 6762's named "Design Rationale for X" appendices are the models. Philosophy lands when it is
**load-bearing for a design imperative**, not decorative — every principle must cash out into a mechanic, or
it floats.

## 2. Terminology that matters — draft vs RFC

A draft and an RFC are **not** two parallel artifacts. A draft (`draft-<name>-drystone-00`) is the *input*
format; an RFC is the *output* format of the **same document** after iteration. "Publish an RFC and a draft
together" is a category error. The real two-document pattern (IKEv2) is spec + companion rationale, both
drafts, both eligible to become RFCs (rationale publishes Informational) — but per §1 we are not taking that
split.

## 3. Feedback venue — iroh first, not the IETF

The IETF is the **wrong first venue**. The active P2P working groups are dead (PPSP, DECADE) or adjacent but
not this layer (QUIC WG owns the transport, not what rides on it), and the IETF wants *two independently
developed interoperable implementations* plus rough consensus before a spec carries weight there — a solo
spec lands as "where's the running code." It is a possible *later* destination once there is an
implementation (and maybe a second one), not a first reader.

Where the conversation actually lives, in order:

- **iroh / number 0 community** — the most direct line. Drystone rides iroh/QUIC + Willow-shaped sync, and
  iroh's roadmap explicitly wants community-contributed protocols on the same footing as core ones. Best
  written/async/citable venue: **iroh GitHub Discussions** (Ideas/General). Real-time design: **iroh
  Discord**. Culture is **anti-hype, pro-engineering** — lead with the concrete problem and the wire format;
  let the philosophy follow.
- **Willow maintainers** — go here only if Drystone's "why" actually touches Willow's data model or sync
  semantics; then they are the sharpest readers alive (there are joint iroh×Willow design calls).
- **local-first community** — the native home for the *design-principles* layer (local state / knowable truth
  / peer equality is its vocabulary).
- **Malleable Systems forum** — lower-rigor, friendly, philosophy-tolerant; good for early shaping.

**The interop question to pre-empt in the spec:** "what value does a protocol have if two 'compatible'
implementations are not actually compatible?" Section 9 (Interoperability) must answer this cleanly — and it
is the exact test of whether *peer equality in rights* cashes out into a mechanic: a reader will want to see
**where in the wire format two independent implementations are forced to honor it.**

## 4. The goal restated — make sure no one can later claim or restrict the idea

This is a **defensive** goal, and it splits into two distinct threats with different defenses:

- **Restricting the expression (the spec text).** Copyright is automatic the moment it is written; nobody can
  claim authorship of the words. To *guarantee* everyone stays free to copy and build on the text, grant it
  with an open license — **CC0** is the strongest ("as close to public domain as the law allows"). Public ≠
  public domain; posting publicly grants nothing by itself.
- **Patenting the idea (a mechanism/method).** Copyright never reaches ideas or methods, so a text license
  does nothing here. The defense is **prior art**: a patent requires novelty, so a complete, detailed,
  publicly-timestamped disclosure that predates a filing is prior art that can block or invalidate the
  patent. A dated public Drystone spec makes the disclosed mechanisms **unpatentable-by-others** — it got
  there first, on the record.

Two conditions decide whether the prior-art defense holds, both in the author's hands:

1. **Disclosure must be *enabling*** — described in enough detail that a skilled person could build it. The
   mechanics section (wire format, sync semantics, the method) does the defensive work; vague gestures defeat
   nothing. **The more completely Section 7 specifies, the stronger the shield.**
2. **Disclosure protects only what was disclosed** — core mechanisms meant to stay open must be in the
   published text, not held back.

**The asymmetry:** publishing-as-prior-art is *defensive* — it keeps the idea open for everyone (including
the author) by making it unpatentable; it does **not** grant exclusive rights or the power to stop others
using the idea. Wanting that power would mean filing a patent yourself (expensive, time-limited, against the
open posture). The defensive open route fits Croft. **Recipe:** enabling mechanics + CC0 text + an explicit
open-implementation/patent-non-assertion notice + a date anchor (below) + keep every version, so the dated
chain is continuous.

## 5. Priority / timestamp vehicle — Zenodo DOI (+ OpenTimestamps), not a bare repo, not an IETF draft

The author's instinct was right that a **local git repo is fakeable** (self-asserted dates) — the goal is a
public, **independently-witnessed** record. Ranked by fit:

- **Zenodo DOI — the primary vehicle.** A free CERN-run repository; you upload the spec, pick a license at
  upload, and it mints a permanent **DOI** + an archived, timestamped copy you cannot backdate. **The
  third-party (CERN) custody is the whole point** — it is the witness a self-hosted repo cannot be. It
  supports **Concept DOI** (all versions / "Drystone the protocol") vs **version DOIs** (each frozen draft),
  has a **GitHub-release integration** (a `.zenodo.json` + a release tag auto-archives the snapshot and mints
  the version DOI), a reserve-DOI-before-publish trick (the spec can cite its own DOI), and mandatory
  per-upload licensing (a useful forcing function). Constraint: files are editable only ~30 days
  post-publish; larger changes mint a new version (permanent on purpose — only freeze a v0.1 worth keeping).
  arXiv is the same idea but gatekept/academic; **Zenodo is the better match for a protocol spec.**
- **GitHub release tag + Internet Archive (Wayback) capture** — complementary second/third witness (GitHub's
  timestamps + the public commit graph + an unaffiliated archive).
- **OpenTimestamps** — the strongest "this exact text existed by this moment": hashes the document and anchors
  it into the Bitcoin blockchain. Proves existence-by-date only (nothing about authorship/content), free,
  and **pairs with Zenodo** rather than replacing it.

**An IETF draft is the wrong tool for timestamping** and is arguably *more encumbered* than a repo you
license yourself: the IETF Trust holds rights, drafts carry reuse-restricting boilerplate, and the standard
note is you may not cite an Internet-Draft other than as work in progress. A Git commit timestamps as well as
a draft and without that overhead.

**Sequence:** write v0.1 to a permanently-standable state (body + principles, single doc) → decide the text
license + patent posture and put the notice *in* the document → reserve the DOI and drop it in the header →
push to GitHub, connect the Zenodo integration, cut the v0.1 release tag → optionally add an OpenTimestamps
proof of the document hash → share by linking the **Concept DOI** (not the raw repo) into iroh Discussions /
Discord. Each later draft = a new release → a new version DOI chained under the Concept DOI. **Gate:** do not
mint the v0.1 DOI off the skeleton — mint it off the first version where **Section 7 is implementable from the
text alone** (the point the disclosure becomes enabling).

## 6. The defensive-publication notice (the front-matter language)

The settled language is in `drystone-spec/drystone-spec-v0.1-skeleton.md` (front matter + "Notice of
Defensive Publication and Open Implementation"). It does three jobs in three paragraphs: a CC0 dedication
(expression threat), a **patent non-assertion + prior-art assertion** ("published deliberately as prior art"
— the single most load-bearing sentence; the author asserts no patent and intends the disclosure to bar
others' patents), and a third-party disclaimer (the honest limit — it cannot promise the mechanisms are free
of *someone else's* existing patent). The notice is only as strong as the enabling mechanics beneath it.

## 7. What is settled vs the user's open calls

**Settled (direction):** single-document shape with principles up front; iroh-first feedback venue; the
defensive (not offensive) posture; Zenodo-DOI-(+OpenTimestamps)-as-priority-vehicle over a bare repo or an
IETF draft; mint-the-DOI-only-when-Section-7-is-enabling.

**Open — the user's calls (surface, do not resolve; NOT-LEGAL-ADVICE where noted):**

- **Text license: CC0 — DECIDED 2026-06-25 (user-approved).** The spec text is **CC0 1.0** (over
  attribution-only CC-BY 4.0; maximal "no one can claim or restrict the idea"). Folded into beta `07` C2 +
  narrative/charter/banner; Apache-2.0 reference-code license unchanged. Intersects but does not resolve the
  bannered MPL-2.0/AGPL substrate gate. NOT-LEGAL-ADVICE — a patent attorney should still review the
  patent-non-assertion paragraph specifically; enabling-disclosure standards and grace periods are
  jurisdiction-specific.
- **Priority order, Zenodo+OTS-first vs IETF-I-D-first.** This dialogue **refines** beta `07` Pillar C's
  settled "IETF Internet-Draft first then arXiv" posture (the user confirmed Zenodo is the correct
  conclusion) — folded into `07` 2026-06-25; see `../BETA-ROLLUP.md`.
- **Whether Drystone ever goes to the IETF** — a possible later destination, not a first move.
