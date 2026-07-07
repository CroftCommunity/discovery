# Open publication and IP stewardship (Layer 7)

date: 2026-07-07

register: governance / manifestation. This doc records the IP-stewardship posture and the route to durable open publication for the neutral stack: the license choices, the prior-art-first defensive-publication strategy, and the per-layer venue map. It is a decision record plus general IP practice, **NOT LEGAL ADVICE**; much of it is jurisdiction-specific, and any point where a patent option would be preserved warrants a patent attorney before any public disclosure.

## Overview

The foundation carries the stewardship of the protocol's intellectual property and its passage into the open. That stewardship has one governing intent, stated up front so the mechanics that follow read as consequences of it rather than as free-standing preferences.

The intent is **defensive**: keep the protocol permanently free and unenclosable, so that it spreads and gets used, while ensuring no third party (and not the foundation either) can use patents to exclude others. This is a textbook defensive-publication posture. No patent of the foundation's own is wanted or needed.

That intent forces three things, which are the three sections of this doc:

- a **license posture** that frees the spec text and the reference code on the strongest open terms available;

- a **defensive-publication strategy** whose load-bearing instrument is prior art (a complete, timestamped, discoverable disclosure), not the license; and

- a **venue map** that routes each layer of the stack to the community where its experts already are, sequenced so the foundations are validated before any broad public moment.

The protocol content itself, the wire encodings whose completeness is the prior-art shield, is specified in the sibling `../drystone-spec/`. This doc does not re-specify any wire encoding; it cross-references the spec at the one seam where publication timing depends on the encoding being complete.

## License posture (current project choices)

The two threats to openness are distinct and take distinct instruments. Copyright and Creative Commons reach only the **expression** (the prose of the spec); anyone may read a copyright-licensed spec and build the protocol regardless of the license. Only a patent, or prior art that blocks patents, reaches the **ideas and methods** (the protocol itself). So the license does one job (frees the text) and prior art does the other (keeps the methods unpatentable); neither substitutes for the other.

Current choices:

- **Spec text: CC0 1.0 Universal.** This is the current decided choice (user-approved 2026-06-25). CC0 is as close to public domain as the law allows: the author waives copyright in the text so no one can claim or restrict the words. CC-BY 4.0 (attribution-only) was the earlier research recommendation and remains a coherent alternative; CC0 was chosen over it for maximal "no one can claim or restrict the idea." Posting text publicly grants nothing by itself (public is not the same as public domain), which is why an explicit dedication is required.

- **Reference code: Apache-2.0.** Chosen over MIT/BSD deliberately. Apache-2.0 carries an **express patent grant** plus a retaliation clause; MIT and BSD are silent on patents. For a protocol, that patent grant is the part that matters.

- **A defensive-publication notice travels in the spec front matter.** It does three jobs in three paragraphs: a CC0 dedication (the expression threat), a **patent non-assertion plus prior-art assertion** (the single most load-bearing sentence: the author asserts no patent and intends the disclosure to bar others' patents), and a third-party disclaimer (the honest limit: it cannot promise the mechanisms are free of someone else's existing patent). The settled language lives in `../drystone-spec/` (its "Notice of defensive publication and open implementation").

The license choices are project decisions, stated here as current choices rather than as settled law. **NOT LEGAL ADVICE**: a patent attorney should still review the patent-non-assertion paragraph specifically, and enabling-disclosure standards and grace periods are jurisdiction-specific.

## Defensive-publication strategy (prior-art first)

A Creative Commons license does nothing about the patent threat. The instrument that keeps the methods open is **prior art**: a patent requires novelty, so a complete, publicly-timestamped disclosure that predates a filing can block or invalidate a later patent on the same mechanism. A dated public spec makes the disclosed mechanisms unpatentable-by-others; it got there first, on the record.

A disclosure functions as prior art only if two author-controlled conditions hold:

- **The disclosure must be enabling.** It must describe each non-obvious mechanism (relay placement, the wire format, the handshake, the lineage/standing membership model, the freshness rule, the visibility/regime model, the synchronization layer) in enough detail that a skilled implementer could build it from the text alone. Vague gestures defeat nothing. A mechanism not specifically disclosed can still be patented around the project. The thoroughness of the spec is the defense.

- **The disclosure protects only what was disclosed.** Any core mechanism meant to stay open must be in the published text, not held back.

Two further properties make the prior art bite in practice:

- **The date must be provable by an independent third party**, not asserted by the project's own git log. A local repo is fakeable (self-asserted dates); the record needs an external, tamper-evident, examiner-discoverable timestamp with third-party custody.

- **Every substantive version needs its own dated public record.** A mechanism added in a later revision has no early date unless that revision is itself timestamped, so the dated chain must be kept continuous.

**The one-directional caveat.** Every publication step is a public disclosure. Under US law a one-year grace period follows public disclosure during which a patent could still be filed; under EU, China, and most other jurisdictions, novelty is absolute and any public disclosure before filing kills patentability immediately. So "publish now, decide on patents later" is not available without forfeiting essentially all non-US patent rights and starting the US clock. Publish-versus-patent is a one-time, up-front decision. For the defensive posture this is the intended outcome: once the stack is socialized, the option to patent it is effectively gone, and that is acceptable because the goal is spread plus use plus block-others-patenting, not retained control.

### The DOI gate (cross-layer seam with the spec)

The sequencing that gates a publication-final archival DOI is tracked in `../drystone-spec/` and is not re-derived here. In short: the defensive disclosure becomes enabling, and therefore protective as prior art, only once a skilled implementer can build the synchronization layer from the text alone. The spec's message formats and its `ENABLING` byte-level wire encodings (the ones that must be pinned before two independent implementations can interoperate) gate this. **Do not mint the archival v0.1 DOI off a draft where those encodings are still open**; mint it off the first version where they are closed. The encodings themselves are the spec's to specify; this doc only records that publication-final timestamping waits on them.

## The venue map (per-layer, then sequence)

The stack is layered (encrypted QUIC transport with relay placement on iroh, MLS group key agreement, Ed25519 device identities under a lineage/standing membership model, content addressing and range-set reconciliation for sync, and a public-social side keyed to atproto DIDs), and no single venue covers all of it. The strategy is therefore per-layer: socialize each layer where its experts already are.

| Layer | Primary venue | Fit |
|---|---|---|
| Transport (iroh QUIC, relay placement, NAT fallback) | iroh / number 0 community (GitHub Discussions, Discord, `awesome-iroh`) | Home community; the stack rides iroh, and its roadmap wants community-contributed protocols on the same footing as core ones. No clean standards home exists for iroh-style relay placement. |
| Group crypto (MLS) | OpenMLS maintainers plus IETF MLS WG | Correctness review of MLS usage before broadcasting; lineage/standing maps onto the WG's pluggable-credential work. |
| Device identity / DIDs | W3C Credentials Community Group | Lowest-barrier expert review of the DID layer; home for DID-method discussion (the DID WG does not standardize methods). |
| Public-social side (atproto DIDs) | IETF Authenticated Transfer WG (atp) plus W3C Social Web WG plus atproto dev community | The public, atproto-DID-keyed side is in scope for the atp WG; the encrypted core is explicitly out of scope (that is MLS/MIMI territory). |
| Data model / membership / sync | Willow / Earthstar / p2panda peers plus Malleable Systems forum | Sharpest peer critique of content addressing, sync, and membership models. |
| Decentralization thesis | IRTF DINRG plus ANRW workshop | Research forum to present the architecture to decentralization researchers; produces a citable association, will not standardize the stack. |
| Flagship public moment | Local-First Conf | Highest concentration of the exact target audience (its theme names peer-to-peer, self-sovereign identity, and atproto). |
| Prior-art timestamp | Zenodo DOI plus OpenTimestamps (see sequence) | Third-party-witnessed, dated, persistent record. |

### Publication sequence

The order pairs the two goals (validated foundations first, durable prior art locked in, then reach):

1. **Validate the foundations privately, while pre-launch.** iroh Discord and GitHub Discussions for transport feedback; OpenMLS / MLS WG for a "is the crypto right" check; Willow / Earthstar / p2panda peers plus the Malleable Systems forum for the data model, membership, and sync. Culture in these communities is anti-hype and pro-engineering: lead with the concrete problem and the wire format, let the design principles follow.

2. **Lock in the prior-art timestamp.** The primary vehicle is a **Zenodo DOI** (free, CERN-run, third-party custody is the whole point; a Concept DOI for the protocol across versions plus a version DOI per frozen draft, with a GitHub-release integration), paired with an **OpenTimestamps** anchor of the document hash and a GitHub release tag plus a Wayback capture as complementary witnesses. This refines the earlier research posture, which put an individual IETF Internet-Draft first as the timestamp vehicle; the Internet-Draft was reconsidered because IETF Trust rights and draft-reuse boilerplate make a self-licensed archived copy cleaner, and because the IETF wants running code and two interoperable implementations before a solo spec carries weight there. `[UNVERIFIED]` (dialogue/web-sourced 2026-06-24, not independently re-verified): the IETF Trust draft-reuse terms, Zenodo specifics, and OpenTimestamps mechanics.

3. **Reach and credibility, after a runnable demo exists.** An **arXiv preprint** (cs.NI primary; cross-list cs.CR, cs.DC) for combined prior-art strength, reach, and academic credibility (an academic co-author solves the endorsement friction and unlocks peer-reviewed venues); then the **flagship venue** (Local-First Conf) as the public moment; then aggregators (an `awesome-iroh` PR, local-first newsletters, a Show HN with a working demo, higher-signal lobste.rs).

4. **Standards and peer review, ongoing and later.** Engage per layer (W3C Credentials CG for the DID layer; IETF atp WG and W3C Social Web WG for the public-social layer; IRTF DINRG / ANRW for the decentralization thesis). A peer-reviewed full paper (PoPETs is the most accessible for an independent author; NSDI if deployment or measurement data exists) adds scrutiny plus citable prior art. **Whether the protocol ever goes to the IETF is a possible later destination, not a first move.**

The **foundation and cooperative structure** that carries this stewardship is the subject of this layer's sibling governance work (the operating body, its edge-preserving capital-formation problem, and the foundation legal structure; candidate foundation name *Noria*, pending clearance, a project decision not settled here). The stewardship recorded in this doc is one of the concrete things that structure exists to hold.

## What this establishes (and does not)

Establishes the IP-stewardship posture the foundation carries: a defensive intent (spread plus use plus block-others-patenting, no own-patent); the current license choices (CC0 1.0 for the spec text, Apache-2.0 for the reference code, with a defensive-publication notice in the front matter); the prior-art-first strategy whose load-bearing instrument is a complete, enabling, third-party-timestamped disclosure rather than the license; and the per-layer venue map with its sequence (foundations first, a Zenodo DOI plus OpenTimestamps timestamp, then arXiv and the flagship venue, standards later).

Does not resolve the legal-review gates: the license choices are stated as current project decisions and not as settled law, the enabling-disclosure standard and patent grace periods are jurisdiction-specific, and the patent-non-assertion language wants an attorney's eye. It does not specify any wire encoding (that is `../drystone-spec/`), and it does not decide the publication-final DOI date, which waits on the spec's `ENABLING` encodings being closed. It does not build the foundation or cooperative form (this layer's sibling governance work), and it does not argue why that form is required (that is the philosophy layer). This is general IP practice, **NOT LEGAL ADVICE**.
