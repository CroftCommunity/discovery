# 07 — Sustainability and stewardship

date: 2026-06-24

status: synthesis — **DECISION-GATED** (see the banner below). Two pillars: the economic/governance
mechanism and the IP-stewardship structure.

> ⚠️ **DECISION-GATED.** This theme carries unresolved calls that are the user's, not engineering's: the
> cooperative legal-review gate, the **Noria** foundation-name clearance, fiscal-sponsor selection, the
> foundation-vs-coop entity form, and the "load-bearing-few principles (genome vs strategy)" question. They
> are surfaced under "What this theme establishes (and does not)," not resolved.

> **NOT-LEGAL-ADVICE.** The legal/financial *framework* here is real, but every statute section, IRS form,
> fee, and dollar figure is dialogue-sourced and **requires confirmation by a cooperative/nonprofit/IP
> attorney before any filing or reliance.** This theme carries the *reasoning*, not the citations; the
> specifics are excluded.

---

## Theme narrative (overview)

There is a pattern, and it is old: communities build real value through network effects, and that value is
repeatedly extracted, enclosed, or simply erased — GeoCities and Yahoo Groups deleted decades of collective
memory because users never held custody. Enshittification names the *behavior*; the root cause is a
*capital-structure* one: venture funding requires an exit, an exit requires an extraction story, and
extraction is the rug-pull. Lock-in supplies the leverage. The survivors (Wikimedia, Signal, AO3, Stocksy)
are notable not for better software but because each solved *funding and governance* in a way that removed
the extraction imperative — and each committed to it at inception.

That diagnosis sets up the theme's first pillar. Non-extraction is anti-fragile because a member-owned
utility paces its growth instead of feeding a burn rate; a revenue dip is leaner spend, not a death spiral.
The mechanism that makes non-extraction *survivable* is a cooperative — a "Social Union" — built on four
pillars: a legal vehicle supporting multi-class membership and indivisible reserves; progressive
decentralization with a hardcoded, externally-ascertainable founder-sunset; a closed, non-speculative
internal economy (dues, a capped royalty, donations through a parallel charitable entity); and labor treated
as a first-class moral good, because burnout is a failure of governance. Against this stands the verified
failure lineage — Ello, Ampled, Steemit, Diaspora, Coomappa — and Discord, where uncompensated volunteer-
moderator labor is literally embedded in the number bankers are pricing.

The second pillar answers a different question: who holds the brand and the code, and how, so the ecosystem
stays plural and capture is *structurally impossible* rather than merely promised. The answer is three
decoupled layers using opposite tools for opposite goals — code (AGPL-3.0-or-later + DCO) made maximally open
and forkable; brand (a foundation-held mark, plus a compatibility badge) made maximally controlled so a fork
is free to exist but not free to impersonate; and operation (a cooperative, one of potentially many) running
the canonical instance under a free-but-conditioned mark license repeatable for the next coop. The
DCO-not-CLA choice removes anyone's power to relicense or close the source — a credible commitment in the
game-theory sense: you remove your own ability to defect.

Both pillars defer overhead through entity phasing: Phase 0 (no entities — operate informally, do the
near-free brand separation now), Phase 1 (a fiscal sponsor rents the foundation function), Phase 2 (an own
foundation holds the marks neutrally + the first coop becomes the reference operator). The marks move by
assignment-with-goodwill, and the foundation must be named independently of the flagship (the Noria
candidate) precisely so its neutrality survives stewarding non-Croft things.

A third pillar extends the IP story outward. Where the second pillar keeps the *operating* ecosystem
uncapturable from the inside, the third governs how the *specification itself* enters the world so a third
party cannot patent the design out from under everyone — a settled defensive-publication posture: claim no
patent, license the specification document CC0 1.0 and the reference code Apache-2.0, and establish
patent-blocking prior art with a third-party-witnessed DOI'd archive (Zenodo) plus a cryptographic timestamp
first — an IETF Internet-Draft only as a later, more-encumbered destination — socializing each layer where
its experts already are.
The through-line across all three: the ethical choice and the durable choice keep turning out to be the same
choice — and each guard is a *structural property*, not a behavioral promise.

## Charter — what this theme covers

**In scope.**

- The VC rug-pull / enshittification cycle *as the problem this theme answers* (the capital-structure
  diagnosis).
- Non-extraction as anti-fragile, and the cooperative as the *mechanism* that makes it survivable.
- The cooperative "Social Union" model: multi-class membership, capped royalty, indivisible reserve,
  501(c)(3) tandem, labor-as-first-class, progressive-decentralization / founder-sunset.
- The anti-rug-pull guarantee: bankruptcy-remote steward + pre-funded static archive + graceful-exit runway.
- The three-layer IP-stewardship structure (code/brand/coop), the AGPL+DCO lock, the foundation-held mark,
  the compatibility badge, entity phasing, assignment-with-goodwill.
- The external IP twin: the defensive spec-publication posture (CC0 1.0 specification document, Apache-2.0
  reference code), prior art via a third-party-witnessed DOI'd archive (Zenodo) plus a cryptographic timestamp
  first — an IETF Internet-Draft only later — and the per-layer socialization/venue map.
- **The Noria foundation-name *candidate*** (surfaced as pending, not decided).

**Out of scope (and where it lives).**

- The provenance-not-utility razor, requisite-variety, "no right to remove the rights of others" → `drystone-spec`
  (this theme inherits non-extraction as a value; it does not re-derive its epistemic basis).
- The crofting/commons history and the settled **Croft** + **Drystone** names → `02`.
- The related-projects register (Platform Cooperativism Consortium, Stocksy, SPI/SFC/Aspiration rows) → `03`.
- Zero-friction joining, the durable-knowledge wedge, the brand pitch → `08`. Discord appears here *only* as
  the labor-extraction illustration, not the competitive-product analysis.

**Boundary calls.**

- Discord is cited in two themes: `08` owns "how Discord won / how to compete"; this theme owns *only* the
  moderator-labor-as-captured-value finding. Survivability is split: the legal/economic anti-rug-pull
  structure lands here; the technical archive mechanics are cross-referenced, not resolved here. The
  "is the badge where principles get teeth" question sits at the seam with `drystone-spec` — surfaced, not resolved.

## Pillar A — the economic / governance mechanism

### A0. Two operating systems — the values contrast under the mechanism

The mechanism in this pillar is the operational expression of a choice between two operating systems. The
**linear / extractive** one reads time as a line toward an end-state, nature as a resource to subdue, people
as expendable and tethered to net worth, and waste as the natural by-product of growth — a fixation on
infinite growth that runs as a kind of social virus. The **cyclical / relational** one reads time as
continuous renewal, value as inherent to a person rather than indexed to labor, and waste as a design
failure ("no such thing as waste"). Non-extraction is not a tax levied on the first system; it is the second
system made structural. The pillars that follow — anti-fragile economics, paid keepers, a steward that
cannot be captured — are what the cyclical operating system looks like when it is *built* rather than merely
professed.

### A1. The rug-pull is a capital-structure phenomenon

> enshittification: "first, they are good to their users; then they abuse their users to make things better
> for their business customers; finally, they abuse those business customers to claw back all the value for
> themselves. Then, they die."
> — Cory Doctorow

*Verification:* **documented** (ADS 2023 Word of the Year). *Grounds:* enshittification is the *behavior*;
the *cause* is that growth capital requires an exit, an exit requires an extraction story, and lock-in is the
leverage. Reddit's IPO and Discord's confidential filing matter as signals — going public installs the
fiduciary obligation to extract. *Buried conclusion harvested:* you cannot retrofit governance after taking
growth capital; the commitment must be made at inception. A "better owner" is not a structural fix:

> "A good king is still a king."

### A2. Non-extraction is anti-fragile, not just ethical

> "Non-extraction is anti-fragile, not just ethical." The VC model is brittle — grow exponentially or die;
> "A member-owned utility only needs *growth-pacing* … a revenue dip is leaner spend, not a death spiral."

*Verification:* **settled value.** *Grounds:* anti-fragility is a *competitive* property, not a concession —
the ethical choice and the durable choice are the same choice. The structural reason a survivor stays a
survivor is that it cannot be sold: "even if I decided to become evil" (Whittaker, on why Signal cannot be
sold) — governance, not decentralization, is the defense, because it removes the motive.

### A2b. Why fee-for-everything is structurally corrosive, not merely distasteful

The non-extraction thesis has a third support beyond the capital-structure (rug-pull) diagnosis and the
anti-fragility argument: a peer-reviewed account of *why* pricing every interaction damages the community it
is meant to monetize. Introducing a price does not simply add a transaction on top of a social relationship —
it *crowds out* the intrinsic motivation that made the relationship real in the first place.

The canonical evidence is Gneezy & Rustichini, "A Fine is a Price" (*Journal of Legal Studies*, 2000): Israeli
day-care centers that introduced a fine for late pickups saw lateness *increase*, because parents reframed a
social obligation ("I am imposing on the staff") as a purchasable price ("I am paying for the extra time").
The fine did not raise the cost of lateness; it converted a moral relationship into a market one — and the
market one was weaker. The broader name for the mechanism is the overjustification effect: attach an extrinsic
price to behavior that was intrinsically motivated, and the intrinsic motivation does not stack on top, it
erodes.

*Verification:* **settled value, externally grounded.** *Grounds:* this is the empirical backbone for
non-extraction as *structure*, not taste. A fee-for-everything model is corrosive because the very act of
metering participation crowds out the contribution that constitutes the community — the contributor who would
hold the door open for free stops once a turnstile appears. The cooperative's internal-credit, dues-and-
donation economy is the deliberate inverse: it funds the commons without putting a price on the social acts
that make it worth funding.

### A3–A5. Credit union, paid keepers, democracy by sunset

> "Credit union, not a club." Member-owned, dues-funded, surplus reinvested — "the co-op is the maintenance
> plan."

> "Pay the keepers — burnout is a failure of governance." — "Just as a city hires engineers to keep the
> water running, we pay engineers to keep the social commons running."

> "Founder-speed early, democracy by sunset — not Day-One direct democracy."

*Verification:* **settled values.** *Grounds:* the co-op is a *utility*, not a hobby (the Rural Electric
Cooperative model — build the lines the community owns when incumbents won't); market-rate cash pay for core
builders is a first-class operational expense prioritized above dividends and investor returns (the explicit
antidote to the verified Ampled burnout death); and directive founder authority auto-expires at hardcoded,
externally-ascertainable milestones into one-member-one-vote, with the sunset locked so it cannot be clawed
back (resolving the Day-One-democracy paralysis without founder entrenchment).

### A6–A7. The cooperative "Social Union" mechanism

A limited cooperative association supporting, in one entity: **multi-class membership** (a founder/dev class
with an early code-veto; a patron/user class with one-member-one-vote; a non-patron investor class with a
capped return and limited/no vote — governance attaches to the person, financial rights stack separately); a
**capped revenue-share/royalty** for seed capital (terminating at a multiple cap — the RBF/Demand-Dividend
shape); an **indivisible reserve** (the co-op endowment); and a parallel **501(c)(3) tandem** (the nonprofit
owns the open-source code + indexers, grant-funded; the co-op owns the app, nodes, and member ledger,
dues-funded). Value is locked in **internal utility credits** (offset dues, no token to dump → no bot
incentive — the anti-Steemit). *(Collapses the existential "sustainability ↔ cooperative mechanism" open item
into a concrete, legally-grounded shape; the statute specifics are excluded.)*

### A8. Anti-rug-pull: bankruptcy-remote steward + pre-funded graceful exit

The guarantee is *not* "we keep your data forever" (an unfundable forever-liability) but "if we die, a
pre-funded runway lets you retrieve and relocate your data before it is gone." Structural core: a
bankruptcy-remote sibling entity / purpose trust whose sole charter is survivability, funded by a restricted
reserve the operating entity cannot touch; the operating nonprofit's plan of dissolution names this trust as
recipient; nonprofit-dissolution law forbids a fire-sale to members. The steward is a *fiduciary to the data
producers* (users), not to a business licensee — the novelty over standard software/data escrow. *Buried
conclusion:* the fund need only keep a static bucket readable — the difference between an impossible promise
(permanence) and a trivial one (a finite, pre-fundable graceful exit). *(The technical archive mechanics are
cross-referenced, not resolved here.)*

### A9. The non-mimicry moat — a feature class only non-extraction can ship

Anti-fragility (A2) is the *defensive* case for non-extraction; the **non-mimicry moat** is the affirmative
one. A cooperative capital structure makes a class of features *possible* that an extractive competitor
cannot copy without unwinding its own business model: protocol-level ad-blocking; transparency-as-a-feature
(an open-book financial dashboard); a member-voted micro-grant treasury; non-algorithmic, human-choice feeds;
maintenance-first budgeting (a majority of spend to tech-debt and security rather than growth); user-
controlled data TTL — the "digital living room," which also *lowers* hosting cost; a community-jury
moderation model; digital-legacy and estate trusts; and a "right-to-repair" forkability promise. None of
these are features an ad-funded, growth-capitalized platform can mimic, because each one removes a lever that
model depends on. The moat is not a better version of what competitors do — it is the set of things they are
structurally unable to do. *(Individual items are product-roadmap-grade; the load-bearing conclusion is the
unifying claim that non-extraction is a feature generator, not only a cost.)*

## Pillar B — the IP-stewardship structure

### B1. Three decoupled layers, opposite tools for opposite goals

```
   LAYER       TOOL                       RULE                      HELD BY
   Code        AGPL-3.0-or-later + DCO    maximally OPEN, forkable  nobody (license governs;
   (commons)                              — gives away code control  foundation holds copyright)
   Brand       trademark → compat badge   maximally CONTROLLED —    the neutral FOUNDATION
   (trust)                                only conformant carries    (licensed out)
                                          the name/badge
   Operation   a cooperative (one of      runs the canonical        the COOP (free, conditioned
   (model)     potentially many)          instance under license     mark license)
```

> "Code and brand are opposite tools and must never touch." — "The fork is free; the *confusion* is what the
> mark prevents." — the mark is "anti-counterfeiting for a community asset — defensive of users' trust, not
> mercantile."

*Verification:* **design intent** (not a legal filing). *Grounds:* AGPL says *take it, fork it*; the mark says
*but you cannot call your fork by our name unless it conforms*. This matters most exactly when success makes
the project a scammer target.

### B2. The structural-openness lock — DCO, not CLA

> "a credible commitment in the game-theory sense (you remove your own ability to defect, which is what makes
> the promise trustworthy downstream)."

*Verification:* **design intent.** *Grounds:* a CLA would collect relicensing rights in one party (a capture
vector); a DCO grants none, so no party — not a future founder, not a captured board, not an acquirer — can
relicense or close the source. The promise is trustworthy *because* it is structurally impossible to break.

### B3. Two tiers of mark + the compatibility badge

Tier 1 = the house mark (Croft), held by the foundation, licensed tightly to the canonical coop — a fork must
rename. Tier 2 = a compatibility / interoperability badge for the published protocol ("Speaks Drystone") —
the honest place for forks. The protocol spec itself is the unowned commons (open spec, controlled badge).
The badge **starts as an honor system, not a USPTO certification mark** (a formal cert mark carries
heavyweight obligations — control the standard, don't use it on your own goods, don't discriminate among
qualifiers — to defer); transparency is the early enforcement, with a revocation clause. *Surfaced, not
resolved:* does flying the badge assert only *technical* interop, or also adherence to the non-negotiable
principles — i.e. is the badge where the principles get their teeth? (Seam with `drystone-spec`.)

### B4. Entity phasing — don't build a governor before the thing it governs exists

> "Rent the foundation function for years; you cannot rent the coop — build it when it's real."

*Verification:* **design intent.** *Grounds:* **Phase 0** (now) — no entities; code AGPL-3.0-or-later;
near-free brand separation today (a `TRADEMARK.md` stating the name/logo are NOT AGPL-covered and forks must
rename; use ™; get the logo copyright assigned in writing by the artist day one — the Debian-logo trap).
**Phase 1** — a fiscal sponsor rents the *foundation* function (not the coop; a charitable sponsor cannot
house a member-owned business). **Phase 2** — spin up the own foundation to hold the marks neutrally + form
the first coop as reference operator; the gated, verified `.coop` TLD becomes available here.

### B5. Marks move by assignment-with-goodwill; capture made structurally impossible

Transfer = a charitable contribution of property; trademarks assigned **with the goodwill** (an assignment
"in gross" can invalidate the mark); logo copyright via separate written assignment; code mostly stays put
(DCO/AGPL means the foundation holds copyright + marks, not a code monopoly). The foundation licenses the
house mark to the coop **royalty-free but with quality-control conditions** (free ≠ uncontrolled — active
control is what preserves the mark), and the *same terms are repeatable for future coops* → even-footing made
mechanical: no coop owns the name; the first has no structural privilege over the second. *(Gives the
"centerless-meets-center" tension — the legal entity / name registrar / the money — a concrete shape arranged
to stay plural.)* This is *why* the foundation must be named independently of the flagship — neutrality collapses the
moment it stewards a non-Croft asset under a Croft-derived name (the Noria candidate, below).

### B6. The verified failure-case lineage

Ello (VC capture — anti-VC mission funded by VC, no revenue engine, silent shutdown, data wiped) → answered by
a capped exit, not a VC veto. Ampled (pure co-op, terminal volunteer burnout) → answered by paid builders.
Steemit (tokenized every click into a bot-farmed casino, hostile crypto buyout) → answered by internal
credits, not a speculative token. Diaspora (leaked metadata to cloud middleware) → answered by native
indexers. Coomappa — the sharpest line:

> "owned a cooperative but actually owned a technological dependency"

*Verification:* **dialogue-sourced, FACTCHECK-verified as a failure pattern.** And Discord as the clean
illustration of the *default* extractive arrangement:

> "volunteer moderator labor is literally embedded in the number bankers are pricing, while moderators are
> external to the cap table" — Discord contributor monetization is "a tenant who can sublet, not an owner."

*Verification:* all Discord *figures* are `[UNVERIFIED]` (private company); the *labor-extraction reasoning*
is the load-bearing part. *Grounds:* contribution and ownership are fully decoupled in the default
arrangement — the member-owned coop is designed to invert exactly that.

## Pillar C — spec publication and prior-art defense

Pillar B holds the *internal* IP story: who holds the brand and the code, and how, so the operating ecosystem
cannot be captured. Pillar C is its external twin — how the *specification itself* is put into the world so
that a third party cannot patent the design out from under everyone. Same goal (capture made structurally
impossible), opposite direction: B keeps the inside plural; C keeps the outside from fencing it off.

### C1. The posture is defensive, decided up front

The first call is *which kind* of protection, and the two pull in opposite directions. An **offensive** posture
retains an exclusionary/commercialization patent option for ourselves; a **defensive** posture keeps the design
permanently free and ensures no one — including us — can use patents to exclude others. **The settled posture
is defensive.** The goal is for the spec to spread and be used while a third party is blocked from patenting
the design; that is a textbook defensive-publication scenario, and no patent of our own is wanted.

*Verification:* **settled posture.** *Grounds:* the posture has to be decided before anything is published,
because disclosure is one-directional — the first public draft or post forecloses the patent-it-ourselves
option for good. For a defensive goal that foreclosure is the intended outcome, which is exactly why the
direction is chosen first rather than deferred.

### C2. Opposite tools for the document and the ideas

A license protects the *prose* of a spec; it does nothing about the *ideas*. Anyone may read a license-bound
spec and build the protocol regardless of the license, so the license alone never touches the patent threat —
only a patent, or prior art that blocks patents, reaches the ideas. The settled instruments split accordingly:

- **Specification document → CC0 1.0.** A public-domain dedication — the text pushed as close to the public
  domain as the law allows, so the goal ("no one can later claim or restrict the idea") holds for the prose
  with no attribution string anyone must satisfy to redistribute or implement. (Chosen over attribution-only
  CC-BY 4.0: for a spec written to be a freely-buildable standard, maximal openness beats keeping authorship
  bound to the text.)
- **Reference implementation → Apache-2.0** (not MIT/BSD). Apache-2.0 carries an **express patent grant** plus
  a retaliation clause; the permissive-but-patent-silent licenses do not. For a protocol, that grant is the
  load-bearing part.

*Verification:* **settled posture** (general IP practice, not a filing; NOT-LEGAL-ADVICE). *Grounds:* the
document license and the code license are doing two different jobs — CC0 makes the writing maximally free to
travel and reuse; Apache-2.0 makes the patent grant ride along with the implementation. Neither, by itself,
blocks a third-party patent; that is what
Pillar C's prior-art move is for.

### C3. The load-bearing instrument is timestamped, examiner-discoverable prior art

For a defensive posture the actual patent-blocking instrument is not the license but **complete, dated,
examiner-discoverable prior art**. A disclosure defeats a later patent claim only if it predates the filing
*and* teaches the claimed mechanism to someone skilled in the field — so two properties are non-negotiable:
**completeness** (each non-obvious mechanism described well enough to build; a mechanism not specifically
disclosed can still be patented around the project, which makes the thoroughness of the spec *itself* the
defense) and **a third-party-provable date** (an external, tamper-evident timestamp, not just our own commit
log — a self-hosted repository's dates are self-asserted and can be disputed). The settled sequence:

- **A third-party-witnessed archive with a DOI, first — Zenodo.** Run by CERN, it mints a permanent DOI and
  an archived, timestamped copy held in custody the author does not control and cannot backdate — the
  external witness a self-hosted commit log cannot be. It carries a Concept DOI over the evolving spec with a
  version DOI per draft, integrates with a public Git release so the tagged snapshot is archived
  automatically, and forces an explicit license choice at upload. The single highest-leverage first move.
- **plus a cryptographic timestamp — OpenTimestamps.** Anchoring the document hash into a public blockchain
  yields a tamper-evident existence-by-date proof that pairs with the DOI for the narrow "cannot be
  backdated" property (it proves existence-by-a-date, nothing about authorship or content).
- **then the per-layer socialization venues (C4)** carry the disclosure to the experts who would otherwise
  never find it.
- **an IETF Internet-Draft is a possible later destination, not the first move.** It timestamps no better
  than a public Git commit and is *more* encumbered — the IETF Trust holds reuse rights and a draft may not
  be cited other than as work in progress — and the IETF expects two interoperable implementations plus rough
  consensus before a solo spec carries weight there. arXiv is the academic analogue but is gatekept (requires
  endorsement) and paper-oriented, so it is a secondary, post-implementation option rather than the
  foundation.

Revisions re-publish the same way — each substantive version needs its own dated record (a new version DOI
under the Concept DOI), or a mechanism added later carries no early date. The disclosure only functions once
it is *enabling* — for Croft that means the synchronization wire format is specified field-by-field, which is
the gate on minting the first version DOI rather than freezing a scaffold.

*Verification:* **settled posture.** *Grounds:* prior art that nobody can find does not function as prior art,
so discoverability (searchable terminology, a stable title/URL, cross-linked spec and implementation) is part
of the instrument, not a nicety.

### C4. A per-layer socialization map, sequenced foundations-first

Croft is layered — encrypted transport with relay placement (the iroh stack), MLS group key agreement,
device identities under the lineage/standing model, content addressing and gossip, and a public-social side
keyed to atproto DIDs — so no single venue covers it. The settled approach is **per-layer**: socialize each
layer where its experts already are, and sequence so the foundations are validated in high-signal communities
*before* any broad public launch. The map:

- **Transport / relay placement →** the iroh community (its home venue, which amplifies work built on the
  stack).
- **Group crypto →** OpenMLS maintainers and the IETF MLS working group, for correctness review before
  broadcasting (the lineage/standing model maps onto the working group's pluggable-credential work).
- **Device identity / DIDs →** the W3C Credentials Community Group, the lowest-barrier home for DID-method
  discussion.
- **Public-social side →** the IETF atproto working group (the public, DID-keyed layer is in scope there; the
  encrypted core is explicitly out) plus the W3C Social Web working group.
- **Data model / membership / sync →** the Willow / Earthstar / p2panda peers, the sharpest critique of content
  addressing and membership models.
- **Flagship public moment →** Local-First Conf, whose theme names peer-to-peer, self-sovereign identity, and
  atproto almost exactly.

*Verification:* **settled posture**, with specific venue dates and editions flagged `[UNVERIFIED]` where they
turn on a calendar. *Grounds:* the strongest prior-art vehicles (the DOI'd third-party-witnessed archive, the
cryptographic timestamp) double as a stable citation and reach, so the prior-art track and the spread track
are run together rather than as separate efforts.

> **NOT-LEGAL-ADVICE (carried).** The *posture* above is settled — defensive, a DOI'd third-party-witnessed
> archive (Zenodo) plus a cryptographic timestamp first, **CC0 1.0 for the specification text**, Apache-2.0
> for the reference code. The statutory specifics behind it (disclosure-bar grace
> periods, absolute-novelty rules, the legal weight of any given timestamp as patent prior art) are
> jurisdiction-dependent and **require a patent attorney before any reliance or any disclosure where a patent
> option is to be preserved.** This pillar carries the reasoning and the sequence, not the statute citations.

---

## What this theme establishes (and does not)

**Establishes:** non-extraction has concrete carriers, not a slogan — a cooperative mechanism (the
economic/governance answer), a three-layer IP-stewardship structure (the internal anti-capture lock), and a
defensive spec-publication posture (the external anti-capture lock) — the first two deferring entity overhead
via phasing, all three making capture structurally impossible rather than merely promised. The verified
failure lineage shows each guard answers a real, documented death, and the motivation-crowding-out evidence
(Gneezy & Rustichini, "A Fine is a Price," 2000) gives the non-extraction thesis an empirical backbone beyond
the capital-structure and anti-fragility arguments: pricing every interaction erodes the intrinsic motivation
that constitutes the community.

**Does not establish (decision-gated):** any legal/financial specific (excluded — NOT-LEGAL-ADVICE; the
framework is real, the statute sections / fees / dollar figures require counsel); the **Noria** foundation
name (a candidate pending trademark clearance); the fiscal-sponsor selection and the coop-vs-coop+501(c)(3)
form; and whether the compatibility badge is where the non-negotiable principles get their teeth (the
genome-vs-strategy question, seam with `drystone-spec`). The sources settle on AGPL-3.0-or-later + DCO as design intent;
the MPL-vs-AGPL relationship to the *substrate* is flagged for confirmation against the protocol docs, not
asserted here.
