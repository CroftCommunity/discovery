# Raw: Drystone publication & defensive-disclosure dialogue (2026-06-24)

**Source:** claude.ai design dialogue, pasted into a CroftC filing session.
**Date:** 2026-06-24.
**Preservation status:** `preserved-condensed (cleaned-paste)` — **content-faithful, NOT a byte-pristine
export** (PLAYBOOK §4). UI render chrome was stripped (web-search/file-create indicators, source-label
chips like "IETF"/"RFC Editor"/"Hacker News"/"Zenodo"/"Ugent", "Document · MD", the `Jun 24` dividers,
the "How specified is Drystone" Q/A widget render). Dialogue content reproduced faithfully. The generated
**Drystone spec v0.1 skeleton** at the end is reproduced as-is; its working copy lives at
`../../../thinking/drystone-spec/drystone-spec-v0.1-skeleton.md`.

**External-fact caveat:** all claims about IETF/RFC process, RFC numbers, Zenodo, OpenTimestamps, arXiv, and
IETF-Trust draft-reuse terms are **dialogue/web-sourced 2026-06-24 and were NOT independently re-verified in
the filing session** — treat as `[UNVERIFIED]` pending a primary-source pass. For iroh/Willow facts the
source of truth is `atproto-atmospheric-web-iroh-mobile-FACTCHECK.md` (iroh `1.0.0`; iroh-docs = range-set
reconciliation, not MSTs); cite it, do not re-verify.

---

## USER

It's my understanding that an etf protocol draft can have a decent amount of narrative or reasoning along
with technical specification, can you research this? What's the appetite for principles of design outside of
mechanics? How long can they be? Can I include broader philosophical thinking to reinforce design imperatives
or is that awkward and ill fitted?

## ASSISTANT

I'll research this. I think you mean IETF (Internet Engineering Task Force) drafts, not "ETF" — let me
verify how those documents actually handle design rationale and philosophical reasoning.

Short answer: yes, there's real appetite for design rationale, but it has a structural home and there's a
culture around how it's expressed.

**The appetite exists, and it's formalized.** The IETF tradition explicitly supports separating spec from
reasoning. RFC 2360 (the guide for standards writers) recommends one approach: divide the standard into
sections, one describing the protocol concisely, while another section consists of explanatory text. So
narrative reasoning isn't tolerated grudgingly — it's a recommended technique. `[UNVERIFIED]`

There's also a whole genre of rationale documents. The IKEv2 rationale draft is a clean example. It explains
the reasoning for the design choices made, as well as possible alternatives, the advantages and
disadvantages of these alternatives, including both the changes they chose to make and the changes they
considered but chose not to make. That last part is worth internalizing as a model: documenting the roads
not taken is highly valued in this culture. `[UNVERIFIED]`

**Where the reasoning goes: appendices.** The dominant convention is to push extended rationale into
appendices so the normative spec stays tight. RFC 6762 (Multicast DNS) is the canonical demonstration. Its
appendices include Design Rationale for Not Using Hashed Multicast Addresses, Design Rationale for Maximum
Multicast DNS Name Length, Benefits of Multicast Responses, Design Rationale for Encoding Negative
Responses, and Use of UTF-8. The body says "do X," and an appendix says "here's why X and not Y." This is
the pattern you want. `[UNVERIFIED]`

**On philosophy specifically.** Philosophical framing is genuinely present in the corpus, not just allowed.
The standards-process RFC itself states that users and providers of the equipment, software, and services
should anticipate and embrace this adaptability as a major tenet of Internet philosophy. RFC 6709 is
essentially a document of design philosophy, describing the architectural principles of sound extensibility
design. `[UNVERIFIED]`

The thing to calibrate: philosophy lands when it's load-bearing for a design imperative, not decorative. The
IETF reflex is "rough consensus and running code." Broad philosophical thinking works if each principle
clearly cashes out into a mechanic. It reads as awkward and ill-fitted if it's a manifesto that floats free
of the wire format.

**Length.** There's no hard page limit on an internet-draft. Drafts run from a few pages to well over a
hundred. The cultural constraints are different: the RFC Editor follows generally accepted rules for grammar,
punctuation, sentence length and complexity as defined by the Chicago Manual of Style, with exceptions to
avoid ambiguity. So prose quality is actively edited. The real pressure is "get it right early" — bugs are
more expensive to fix the later they are found, the same applies to documentation, and there's emotional
baggage with mature documents. So length is fine if it earns its place. Bloat that delays consensus is the
actual cost, not a word count. `[UNVERIFIED]`

**Practical structure for your Drystone draft.** Introduction carries the motivating problem and the one or
two philosophical commitments that drive everything. Normative body is pure mechanics (transport, sync
semantics, message formats; RFC 2119 keywords; no philosophy here). Appendices are where deeper reasoning
lives ("Design Rationale for X," "Alternatives Considered"). One caution given your croft/enclosure
framework: a reviewer will respect a governance philosophy that explains why a protocol mechanic exists.
They'll bounce off political theory presented for its own sake. Anchor every principle to a mechanic and you
can include quite a lot. Want me to sketch a skeleton outline for the Drystone draft with the rationale
slotted into appendices the way RFC 6762 does it?

## USER

I'm not going to get into governance or anything outside of drystone but that can include a decent amount of
philosophy in grounded terms on local state and knowable truth and tie in to peer equality in rights so its a
lot of why on thinking yes, and then you have the expressed mechanics of the outcome of that thinking. Makes
me wonder if an rfc and an ietf draft together make more sense

## ASSISTANT

Your instinct is right, but let me clarify the terminology because it changes the answer.

An RFC and an IETF draft aren't two parallel things you publish together. A draft is the input format and an
RFC is the output format of the same document. You write `draft-yourname-drystone-00`, iterate it, and if it
goes the distance it gets published as an RFC with a number. Same artifact, two lifecycle stages. So "an RFC
and a draft together" isn't quite the split you're reaching for.

But the split you're describing is real and well-established. It's two documents: a protocol spec (Drystone
itself — transport, sync semantics, message formats, the mechanics; tight, normative, RFC 2119 keywords) and
a design rationale / architecture document (the "why" — local state, knowable truth, peer equality in rights,
and how each commitment forces a specific mechanic). The IKEv2 precedent is exactly this: the protocol lived
in one draft and the reasoning lived in a separate companion rationale draft. Both are drafts; both can
become RFCs (rationale docs publish as Informational). `[UNVERIFIED]`

You have three options, not two:

1. One document, rationale in appendices (the RFC 6762 model). Good when the why is moderate.
2. One document, a substantial "Design Principles" section up front, then mechanics. Good when the philosophy
   is the connective tissue — when you can't understand any mechanic without first accepting the premises.
   This sounds closest to what you're describing.
3. Two documents (the IKEv2 model). Good when the why is so extensive it's a reference in its own right, or
   when you expect the philosophy to outlive or generalize beyond this one protocol.

Here's the thing pushing me toward option 2 for you specifically. You said you're staying inside Drystone and
not touching governance. That keeps the philosophy tethered — local state, knowable truth, and peer equality
aren't free-floating political theory, they're claims about what the protocol can and cannot guarantee. When
the philosophy is that tightly coupled to the mechanics, splitting it into a separate document forces a
reader to hold two files open to understand one idea. The IKEv2 split made sense because its rationale was
largely retrospective ("why we changed this from v1"); yours is generative — the reasoning produces the
design in front of the reader.

Structure for a single document: Introduction (the problem, briefly) → Design Principles (the substantial
"why"; local state as the unit of truth; what "knowable" means and what it costs; peer equality as a rights
claim; each principle ends pointed at a consequence) → Mechanics (transport, sync, formats; each traceable
back to a principle by name) → Appendices (alternatives considered, the roads not taken).

One real cost to flag: the IETF reflex distrusts long preludes before any wire format appears. A heavy
front-loaded philosophy section invites "where's the protocol." That's not a reason to cut it — it's a reason
to make the introduction land the motivating problem hard and fast. Is this document headed for actual IETF
submission, or is it a spec you're authoring in RFC style for Drystone's own use?

## USER

Spec but is there a group I could share it with for feedback over there and how

## ASSISTANT

The honest answer is that it depends on how Drystone is positioned. Let me check what's active.

The honest framing first: the IETF is probably the wrong venue, at least initially. The active P2P working
groups are either dead (PPSP, DECADE — both wound down years ago) or adjacent but not your layer (QUIC WG
cares about the transport, not what you build on top of it). The IETF wants two independently-developed
interoperable implementations and rough consensus before a spec means much there, and a solo spec drops into
that culture as "where's the running code." `[UNVERIFIED]`

Where the actual conversation lives, given your stack:

- **The iroh / number 0 community is your most direct line.** Drystone is built on iroh/QUIC and Willow sync
  semantics, and these are the people actively building exactly that intersection. iroh's roadmap explicitly
  says they want to put community-contributed protocols on the same footing as the ones the core team
  maintains. Their Discord and GitHub discussions are where that happens.
- **The Willow team takes spec feedback directly.** The iroh team has said they've been working with the
  Willow team and giving feedback on the spec; there are joint iroh-x-Willow design calls. If your "why"
  touches Willow's data model or sync semantics, those maintainers are the sharpest possible readers.
- **The local-first community is the right home for the philosophy.** Local state as the unit of truth,
  knowable truth, peer equality in rights — the native vocabulary of local-first.
- **Malleable Systems and similar forums are a softer option.** The Willow spec was discussed there on
  release; lower rigor, friendlier, good for early shaping.

One sharp question those readers will press, worth pre-empting in the spec: a commenter on the Willow launch
asked the obvious interop question — what value a protocol has if two "compatible" implementations aren't
actually compatible with each other. That's the question your normative section has to answer cleanly, and
it's the thing that most tests whether your philosophy actually cashes out into mechanics. If "peer equality
in rights" is a principle, a reader will want to see exactly where in the wire format two independent
implementations are forced to honor it.

Order I'd suggest: share it first with the iroh community and, if it touches their semantics, the Willow
maintainers; use the local-first community for the design-principles layer; treat the IETF as a possible
later destination once you have an implementation and maybe a second one.

## USER

Sure. And I'm partially interested in even a draft apex bc it puts a timestamp on it so it's public domain
early as well

## ASSISTANT

Let me get you the current channels, and also correct an important misconception about the timestamp idea,
because an IETF draft does not do what you're hoping there.

**Where to share, with actual links:**

- iroh GitHub Discussions — `https://github.com/n0-computer/iroh/discussions`. Active and current. A
  protocol-on-iroh writeup fits the "Ideas" or "General" category. Best written, async, citable venue.
- iroh Discord — `https://iroh.computer/discord` (the canonical non-expiring invite; ignore discord.me/iroh,
  an unrelated server). The maintainers actively pull people into Discord for deeper design conversation.
- awesome-iroh — `https://github.com/n0-computer/awesome-iroh`. Worth scanning: there's an **iroh-rings**
  entry described as a relationship-based access control protocol for resources over iroh, and **p2panda**,
  building blocks for peer-to-peer applications. Both are conceptual neighbors; reading their framing on
  peer-equality questions will sharpen yours.
- Willow team — contact paths linked from `https://willowprotocol.org` and their GitHub. Go here only if
  Drystone's "why" actually touches Willow's data model or sync semantics.
- Malleable Systems forum — `https://forum.malleable.systems`. Lower-rigor, friendly, philosophy-tolerant.

One read on the iroh community worth absorbing before posting: the culture there is anti-hype and
pro-engineering. The 1.0 writeup made a point of saying there's no token economics or consensus mechanisms,
just networking primitives that solve real problems. Lead with the concrete problem and the mechanics; let
the philosophy follow. `[UNVERIFIED — iroh 1.0 per FACTCHECK is 1.0.0]`

**The timestamp idea — the important correction.** A draft does not put anything into the public domain.
Two separate things are getting conflated:

1. **Timestamp / proof of priority.** Yes, posting publicly creates a dated public record. A GitHub
   Discussion, a commit, a forum post — all timestamped and visible. An IETF draft is no better at this than a
   Git commit, and a draft carries overhead a commit doesn't.
2. **Public domain.** Posting something publicly does not place it in the public domain. Your spec is
   copyrighted automatically the moment you write it. "Public" and "public domain" are unrelated. If you want
   others to freely implement and build on Drystone, you grant that with an explicit license — a permissive
   spec license or CC0 (the deliberate "as close to public domain as you can get" tool) for the text. A patent
   grant is separate again: copyright on the spec text doesn't stop someone patenting a mechanism, and
   describing it publicly mainly creates prior art against others' patents.

And the IETF case specifically cuts against your instinct: IETF drafts are not freely reusable by default.
The IETF Trust holds rights, drafts carry boilerplate restricting reuse, and the standard note is that you
may not use an Internet-Draft as reference material or cite it other than as work in progress. So a draft is
arguably more encumbered than a repo you license yourself, not less. `[UNVERIFIED]`

What I'd do if early-and-open priority plus reusability is the goal: put the spec in a public Git repo, add an
explicit license (CC0 or a spec license for the text, plus a clear patent-grant statement if relevant), tag a
v0.1, and link it into iroh Discussions. I'm describing how copyright and these licenses generally work, not
giving legal advice.

## USER

I was hoping for something more declarative to put it out there so it's not just a local git repo which,
frankly, can be faked. But that's all good info, other means to share a draft thinking that lands?

## ASSISTANT

Right — you want a public, independently-witnessed record, not something on your own machine where the dates
are self-asserted. Means that actually land, roughly in order of fit:

**The cleanest fit is publishing to a repository that issues a DOI. Zenodo is the usual choice.** Run by
CERN, free, you upload the spec, pick a license at upload time, and it mints a permanent DOI and an archived,
timestamped copy you don't control and can't backdate. That's the declarative "this existed, here, on this
date, under this license" artifact, and the third-party custody is the whole point. arXiv is the same idea
but gatekept and oriented to academic papers, so Zenodo is the better match for a protocol spec. `[UNVERIFIED]`

**A close second, complementary:** push your Git tag to a public host and have that snapshot archived
externally. A GitHub release tag is harder to fake than a local repo (GitHub's timestamps + the public commit
graph are third-party records). Then submit that release URL to the Internet Archive's Wayback Machine. Two
unaffiliated witnesses.

**Strongest "this exact text existed at this moment":** cryptographic timestamping. OpenTimestamps takes a
hash of your document and anchors it into the Bitcoin blockchain, producing a proof that the content existed
before a given block. It proves nothing about authorship or content, only existence-by-this-time, but for the
"can't be backdated" worry it's about as strong as it gets and it's free. Pairs with Zenodo rather than
replacing it. `[UNVERIFIED]`

The trap to keep straight: none of these change your rights, they only create evidence. The DOI, the archive
capture, the blockchain anchor all establish "this existed on this date" (prior-art and priority evidence).
What others may do with the spec is still governed entirely by the license you attach. Zenodo makes you choose
one on the way in, which is a nice forcing function. If "freely buildable by anyone" is the intent, that's
CC0 or a permissive spec license on the text, plus a separate explicit patent-grant statement if the
mechanics matter.

For the feedback goal: do both halves. Publish the durable, witnessed artifact (Zenodo DOI, ideally with an
OpenTimestamps proof), then link that into iroh's GitHub Discussions and Discord. Citing a DOI rather than
"my repo" reads as more serious to that crowd. One caution: a DOI is permanent on purpose. You can version it
(Zenodo supports new versions with their own DOIs under a parent DOI), but you can't un-publish a version. So
put out a v0.1 you're comfortable having on the permanent record.

## USER

Sure and tell me about Zenodo

## ASSISTANT

(Verified the specifics rather than working from memory.) `[UNVERIFIED — web-sourced 2026-06-24]`

**What Zenodo is.** A free, open-access repository run by CERN, built on the same Invenio software and stored
in the same data centers as the LHC's research data. The custody and timestamping are CERN's, not yours —
exactly the third-party witness a local git repo can't be. Files stored in CERN data centres in Geneva with
replicas in Budapest, multiple replicas in a distributed file system, backed up to tape nightly. Items
retained for the lifetime of the repository (the lifetime of CERN; an experimental programme defined for at
least the next 20 years). It accepts any research output and every upload gets a DOI. It doesn't constrain
format or license.

**Mechanics that matter for a spec:**

- The DOI is the citeable, permanent handle. By default Zenodo registers DOIs for all uploads when published.
  Useful trick: you can reserve a DOI in advance and include it inside the document itself before uploading —
  so your spec text can cite its own DOI.
- Versioning is built for an evolving spec. Two kinds of DOI: **version DOIs** (specific versions) and a
  **Concept DOI** (represents all versions, resolves to the latest). Cite the specific-version DOI for
  reproducibility; cite the Concept DOI to point at "Drystone the protocol" without pinning a version.
- Licensing is mandatory and set per upload — the forcing function. Pick deliberately: CC0 or a permissive
  spec license on the text if you want it freely buildable, with a separate patent-grant note if warranted.
- Immutability — the constraint. Files can be modified only during the first 30 days after publishing, and
  that's only to catch small errors; large changes generate a new version with a new DOI. Metadata can be
  edited at any time without generating a new version.
- Limits are irrelevant for a spec: up to 100 files and 50GB per record.
- GitHub integration: a specific release of your repo is archived in Zenodo, assigned a DOI, and made
  citeable; a `.zenodo.json` keeps metadata stable across releases.

**Publication sequence I'd suggest:** write Drystone v0.1 to a state you'd stand behind permanently (spec
body + design-principles section, single self-contained document) → decide the license now (text license:
CC0 or permissive; explicit patent posture) and put the notice in the document → reserve the DOI and drop it
into the header so the spec cites itself → put it on GitHub, connect the Zenodo–GitHub integration, cut a v0.1
release tag (Zenodo archives the snapshot and issues the version DOI under a Concept DOI) → optionally add an
OpenTimestamps proof of the document hash → take it to the audience by linking the Concept DOI (not the raw
repo) into iroh's GitHub Discussions and Discord. Each subsequent draft becomes a new release → new version
DOI chained under the same Concept DOI.

Honest caveat: this gives a strong, CERN-witnessed, DOI-anchored record that the text existed on a date under
a stated license. That's excellent priority and prior-art evidence. It is not itself a grant or reservation
of any patent rights, and it's evidence rather than adjudication. If the rights position could carry real
consequences, the patent side is worth a short conversation with an IP attorney. I'm describing how the
tooling works, not giving legal advice.

## USER

Mainly I just want to make sure no one can later claim or restrict the idea

## ASSISTANT

Then let me be precise, because the goal you just named is a defensive one, and the tools sort cleanly once
you frame it that way. "No one can later claim or restrict the idea" breaks into two distinct threats.

**Threat 1: someone restricting the expression (your actual spec text).** Nearly handled by how copyright
works. The moment you write Drystone, you hold copyright in that text automatically; nobody can claim
authorship of your words or stop you from using them. If you additionally want to guarantee everyone stays
free to copy and build on the text, you grant that with an open license, and **CC0 is the strongest version**
because it pushes the text as close to public domain as the law allows. Publish under CC0 and the text itself
can never be enclosed. Fully within your control.

**Threat 2: someone later patenting the idea (a mechanism/method) and using the patent to restrict others.**
Copyright protects expression, never ideas or methods — so CC0 on your text does nothing to stop a patent on
the technique. The defense is **prior art**. A patent requires novelty; if your idea was publicly disclosed,
in enough technical detail, on a date provably before someone files, that disclosure is prior art that can
invalidate their claim or block the patent. So a complete, detailed, publicly-timestamped Drystone spec is a
genuine defensive instrument: it makes the core ideas unpatentable-by-others because you got there first,
publicly, on the record. This is why a Zenodo DOI or OpenTimestamps anchor matters — not because it grants you
anything, but because it's hard-to-dispute evidence of the disclosure date.

Two things determine whether the prior-art defense holds, both in your hands:

1. **Disclosure has to be enabling.** Vague gestures don't defeat a later patent. The spec has to describe the
   mechanism in enough detail that a skilled person could build it. Your mechanics section — wire format, sync
   semantics, the actual method — is what does the defensive work. The more completely you specify, the
   stronger the shield.
2. **Disclosure protects what you disclosed.** Prior art defeats patents on the ideas actually described; it
   doesn't pre-empt genuinely novel extensions someone builds later. Core mechanisms you want to keep open
   need to be in the published text, not held back.

One asymmetry, plainly: publishing as prior art is a **defensive** move. It keeps the idea open for everyone,
including you, by making it unpatentable. It does not give you exclusive rights or the ability to stop others
from using the idea — it stops them from monopolizing it. If "no one can restrict it" means "it stays free for
all," prior art plus CC0 achieves that. If you ever wanted the power to control who uses it, that's the
opposite strategy (file a patent yourself — expensive, time-limited, against the open posture). From
everything about Drystone, the defensive open route fits.

Concrete recipe: write the spec with the mechanics fully enabling (not just principles); publish under CC0
(text) with a clear statement that you intend the disclosed mechanisms as an open, freely-implementable
standard; anchor the date with a Zenodo DOI, ideally plus an OpenTimestamps proof; keep every version you
publish so the dated chain is continuous. That makes the text uncopyrightable-against-you, the ideas
unpatentable-by-others, and the date undisputable. Honest limit: I'm describing how copyright, patents, and
prior art generally function, not rendering a legal opinion. Patent law has jurisdiction-specific wrinkles
(what counts as enabling disclosure, how grace periods work, defensive-publication formalities). If
foreclosing patent enclosure of Drystone genuinely matters, an hour with a patent attorney to confirm the
disclosure is structured to hold up is money well spent.

## USER

Sure [draft the defensive-publication front matter and notice]

## ASSISTANT

(Choices explained after; the language itself does the work. "Intended as prior art" stated plainly; CC0
covers the text; patent non-assertion + open-implementation covers the mechanisms. Header conventions kept:
labeled lines, blank lines between, no em-dashes.)

```markdown
# Drystone

**Title:** Drystone — An Open Peer-to-Peer Protocol for Local-First State Synchronization

**Version:** 0.1.0

**Status:** Draft / Defensive Publication

**Author:** [Your name]

**Date:** [ISO date, e.g. 2026-06-24]

**DOI:** [reserved Zenodo DOI]

**Canonical source:** [GitHub release URL]

**License (text):** CC0 1.0 Universal (Public Domain Dedication)

**License (implementations):** [e.g. Apache-2.0, or "any"]
```

```markdown
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
```

Worth understanding per paragraph: the "published deliberately as prior art" line is the single most
important sentence for the stated goal (a future examiner or court reads it). The CC0 paragraph handles the
expression threat (belt-and-suspenders with the header license line; defensive publications benefit from
prose, not just metadata). The patent paragraph handles the idea threat and does two things — a non-assertion
(you won't wield a patent against implementers) and a prior-art assertion (the disclosure should block others
from patenting). The third-party disclaimer is the honest limit (you can't promise the mechanisms are free of
someone else's existing patent). Two cautions, same direction: the notice is only as strong as the mechanics
it sits on top of (prior art defeats patents on what's actually, enablingly disclosed — the detail in the
body is the shield, the notice just declares intent); and the standard limit — this is well-established
defensive-publication language in spirit, but it's not a legal opinion, and the patent non-assertion
paragraph in particular is where precise wording can matter by jurisdiction, so have a patent attorney read
those two paragraphs.

## USER

Sure [draft the spec body skeleton]

## ASSISTANT — calibration

Q: How specified is Drystone right now in your head? — A: **Principles are firm, mechanics still forming.**

That's the most useful state to draft against. The principles anchor the document; the mechanics sections
become structured prompts that pull the design out of you while flagging exactly where the defensive
disclosure has to be enabling later. Each mechanics section carries a short bracketed marker telling you what
"enabling" requires there; every mechanics section traces back to a named principle, so the document reads as
"premise, therefore mechanism."

Notes on the structure: Section 2 (principles) is ready to write now. Sections 4–7 are the mechanics and
carry the ENABLING markers. Anything still forming goes in Appendix B rather than being half-specified in the
body (a vague normative section reads as underspecified, not deliberately deferred). **Section 7
(Synchronization) is load-bearing** — both the heart of the protocol and the most patent-sensitive mechanism,
so it's where "enabling disclosure" does the most defensive work; until 7.2's message formats are specified
field-by-field, that part isn't yet protected as prior art. The `Realizes:` lines make the document read as
premise-then-mechanism; a mechanics section that traces to no principle is either unmotivated or signals a
missing principle. Principle 2.4 is marked optional because adding principles that don't cash out into a
mechanic is the failure mode that makes philosophy float.

On sequencing: the skeleton is fine to iterate publicly on GitHub as a living draft. **Don't mint the v0.1
DOI off the skeleton — mint it off the first version where Section 7 can be implemented from the text alone.**
The skeleton is for thinking against; the DOI is for the first version worth freezing.

[The generated skeleton document follows — working copy at
`../../../thinking/drystone-spec/drystone-spec-v0.1-skeleton.md`.]

---

### Generated artifact: Drystone spec v0.1 skeleton (as produced in-dialogue)

```markdown
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

## Notice of Defensive Publication and Open Implementation
[CC0 text dedication + patent non-assertion + prior-art assertion + third-party disclaimer — see the
front-matter block above; reproduced verbatim in the working copy.]

## 1. Introduction
[Motivating problem, hard and fast. What breaks in the centralized / server-mediated model, what local-first
restores. End pointed at "therefore the protocol must..."]
### 1.1 Scope
[Drystone the protocol and nothing above it. No governance, no application layer — the scoping that keeps
principles tethered to mechanics.]
### 1.2 Terminology
[BCP 14 (RFC 2119 / RFC 8174); then a short table of Drystone terms: peer, entry, namespace, capability,
payload. Precise terminology is itself part of enabling disclosure.]

## 2. Design Principles
[Substantial, prose-heavy — the "why." Each principle: commitment, grounded reasoning, ends pointed at a
consequence the mechanics must satisfy. Named, because later sections reference them.]
### 2.1 Local State as the Unit of Truth
### 2.2 Knowable Truth
### 2.3 Peer Equality in Rights
### 2.4 [Further principle only if a mechanic requires a premise not covered above]

## 3. Protocol Overview
[Concrete map before the detailed mechanics — peer, local store, session establishment, how state moves.
Descriptive, not normative.]

## 4. Data Model            > Realizes: 2.1, 2.2  [ENABLING]
### 4.1 Entries and Identifiers
### 4.2 Namespaces and Organization
### 4.3 Authentication and Integrity of State

## 5. Identity and Capabilities   > Realizes: 2.3  [ENABLING — the section a reviewer presses hardest]
### 5.1 Peer Identity
### 5.2 Capabilities and Access

## 6. Transport            > Realizes: 2.1 / 2.3  [ENABLING — reference iroh/QUIC precisely; specify only
Drystone's additions]
### 6.1 Connection Establishment
### 6.2 Session Lifecycle

## 7. Synchronization      > Realizes: 2.1, 2.2  [ENABLING — the core defensive-disclosure section; two
independent impls following this MUST interoperate]
### 7.1 Reconciliation Model      [Willow/range-based set reconciliation lineage if that's the approach]
### 7.2 Message Formats           [exact wire formats, field by field — the heart of enabling disclosure]
### 7.3 Conflict and Convergence  [convergence guarantee; eventual-consistency semantics]
### 7.4 Deletion                  [true destructive deletion if following the Willow lineage]

## 8. Security Considerations
[Threat model; what Drystone defends and explicitly does not; residual risks honestly.]

## 9. Interoperability
[Answer the Willow-launch question directly: what does "Drystone-compatible" mean, and where exactly does the
spec force two impls to agree? This is where peer-equality-in-rights is shown to be enforced by mechanism.]

## Appendix A. Alternatives Considered
[Roads not taken, IKEv2-rationale tradition; strengthens the defensive disclosure by documenting the design
space.]
## Appendix B. Open Questions
[What's unresolved at v0.1 — where the forming mechanics live without weakening the normative sections.]
## References
[Normative: BCP 14, iroh, QUIC (RFC 9000), Willow. Separate normative from informative.]
```
