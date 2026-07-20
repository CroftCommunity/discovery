# Dating, Friendship, and Meetup Apps in 2025-2026: Landscape and Fit with Drystone (v2)

`Revision 2, 2026-07-19. Supersedes the 2026-07-18 report. Part 1 (landscape) is updated
in place where this session grounded new facts; Part 2 (the mapping) is rewritten against
the actual corpus state: GROUPS.md v2 + RUN-17/18 executable evidence, the attestation
lane through RUN-ATTEST-04 (verdicts V1-V10 closed), and the staged CONTACT.md design.
The original report treated Drystone capabilities as stated design; this revision cites
evidence grades and marks what remains Design-only.`

## TL;DR (revised)

- **Meetup/community remains the beachhead, but the claim upgraded from "natural fit" to
  "demonstrated mechanisms."** The organizer bundle (consent rosters, non-inflatable
  counts, succession, operator-swappable delivery, interval-scoped history) is no longer
  a design assertion; it is exercised end to end in the tier proof and the
  reception/publications amendment, including a measured 100k-roster scale rehearsal.

- **Dating's verdict changes from "out of scope until a private layer exists" to "the
  contact layer is designed and the private substrate is real; only the matching loop
  stays out."** The sealed tier exists at Verified/loopback grade and dating's dyadic
  shape sits inside its comfortable ceiling. A complete first-contact model is now staged
  (CONTACT.md): contact policy enforced at the gate, structured knocks with standing,
  mutual reveal via fair-reveal, provider-visible messaging as an honestly named helper
  posture, blocks, and posture transitions under one lineage. Matching, ranking, and
  discovery remain deliberately outside the protocol, but they now have a specified
  provenance floor (user-ranked attribute assertions) with advanced matchers as opt-in
  helpers at the app layer, which is a different statement than "poor fit."

- **The identity/trust battlefront now has a designed alternative, not just a critique.**
  The market is centralizing biometric trust (Tinder Face Check mandatory for new US
  users) because no shared trust primitive exists. The co-op-as-issuer model answers it
  without biometric custody: anchor-persona credentials with sibling unlinkability,
  era-grain transparency (signed tree heads over keyed commitments, holder-stapled
  proofs, no verifier callback), recorded rejection of portable proof-of-personhood, and
  a no-monetization covenant as a governed rule. The honest cost is stated rather than
  hidden: a paying repeat abuser gets N chances; that is the price of not building Face
  Check.

- **The industry picture sharpened in the platform-consolidation direction.** Bending
  Spoons IPO'd July 1, 2026 at an $18.4B valuation and its portfolio now spans Meetup,
  Evernote, WeTransfer, Komoot, Vimeo, AOL, Brightcove, and Eventbrite, with 1,000+
  further targets identified. The extraction arc the original report described is now a
  publicly traded, capitalized machine, which strengthens the organizer-refugee thesis
  and the activism-layer framing.

## Key definitions (unchanged, one addition)

*Provenance* = a verifiable record of what was said, by whom, in what causal order.
*Utility* = judgments of who is right or who is a good match, left to humans and apps at
the edges. *Protocol / deployment / product layers* separated throughout. New in v2:
*posture* = which protection model a scope runs; **provider-visible** (operator-readable
for screening and evidence, hidden from the world, never described as encryption) versus
**sealed** (real MLS end-to-end encryption, operator content-blind). The two are
different protection models, not strong and weak versions of one.

## PART 1: The landscape (updated where this session grounded new facts)

The Part 1 findings of the original report stand; the deltas grounded since:

1. **Dating financials.** Unchanged: Tinder MAU decline narrowing (-7% March 2026, -6.6%
   April, registrations growing for the first time since June 2020), Match payers down
   5% to 13.5M while revenue per payer rose 10% to $20.90
   ([Perplexity/MTCH](https://www.perplexity.ai/finance/MTCH/earnings),
   [SEC 8-K](https://www.sec.gov/Archives/edgar/data/891103/000089110326000020/mtch8-k20260203ex992.htm)).
   The extraction signature (rising per-payer revenue on a shrinking payer base) remains
   the sector's defining financial fact. The widely quoted $200-300/month dater spend
   traces to Match's own Singles in America survey and remains unverified self-report;
   treat as illustrative only.

2. **Safety numbers, grounded to Pew primary sources.** 57% of US women vs 41% of men
   say dating apps are not a safe way to meet people; 56% of women under 50 who have
   used the apps received unsolicited explicit content; roughly four in ten were
   contacted after saying they were not interested; 52% of users believe they have
   encountered a scammer (Pew Research Center, Feb 2, 2023 report on the 2022 survey).
   The 2020 Pew study (n=4,860): among women 18-34 who online date, 60% report continued
   contact after expressed disinterest, 44% report offensive names, 19% report threats
   of physical harm. These figures are load-bearing for the design: they motivate the
   structured knock (no media), gate-enforced contact policy, screening in the
   provider-visible posture, and freeze-by-default.

3. **Verification centralization.** Tinder Face Check mandatory for new US users since
   October 2025, storing a face map and enforcing one-face-one-account
   ([TechCrunch](https://techcrunch.com/2025/10/22/tinder-will-require-new-users-in-the-us-to-verify-their-identity-with-a-selfie/),
   [Tinder Newsroom](https://www.tinderpressroom.com/2025-10-22-Tinder-to-Expand-Facial-Verification-Feature-Across-the-U-S-,-Setting-a-New-Standard-for-Dating-Safety));
   Match plans portfolio-wide extension in 2026. Romance-scam industrialization
   unchanged ($1.16B reported Jan-Sep 2025, FTC Consumer Sentinel).

4. **Bending Spoons, updated and expanded (grounded this session, July 2026).**
   Acquisition-and-extraction record per target: Evernote (acquired January 2023;
   essentially the entire legacy staff laid off July 2023; personal plan raised 63%,
   $80 to $130/year), Meetup (January 2024, layoffs within a month, tripled organizer
   fees and new paywalls reported
   ([Medium/Hoffbits](https://medium.com/@hoffbits/pay-to-skip-why-meetup-risks-undermining-its-own-community-635cb1c25f9c))),
   Mosaic Group (all ~330 staff), WeTransfer (July 2024, roughly three-quarters of staff
   cut, free tier capped
   ([Follow the Money](https://www.ftm.eu/articles/wetransfer-owner-buys-up-apps-then-makes-them-more-expensive))),
   StreamYard ($25 to $45/month), Brightcove (about two-thirds of US staff cut March
   2025), Komoot (2025, ~75% cut). New since the original report: the portfolio also
   includes Vimeo, AOL, Harvest, and Eventbrite, and Bending Spoons went public July 1,
   2026 at $29/share, raising $1.68B at an $18.4B valuation, with management citing
   1,000+ identified targets and over $2B deployed in Q1 2026 alone. Two consequences:
   the "Eventbrite friending events up 35%" trend datum now sits inside the same owner
   as Meetup, and the enshittification arc is now an institutionalized, exchange-listed
   business model, which is the strongest possible version of the organizer-refugee
   thesis. Filed to the activism layer as the next documented arc.

5. **Event tooling.** Partiful (~500K+ MAU, playful mobile-first pages, Text Blasts,
   app-nudgy for guests) and Luma (clean app-free invites, subscribable host calendars,
   waitlists/ticketing, 5% platform fee free tier, $59/month Plus at 0%) remain the
   modern pair ([Lemonvite comparison](https://www.lemonvite.com/blog/partiful-vs-luma-vs-lemonvite),
   [party.pro](https://party.pro/partiful/)). Both hold the guest list privately;
   neither offers a portable roster. Luma's subscribable calendar is the closest
   incumbent shape to the lineage-plus-reader-roster model, minus every guarantee.

6. **Prior art.** Unchanged: decentralized dating is a graveyard (Alovoa self-hosted
   silos, no federation; the blockers everywhere are privacy and Sybil resistance, not
   plumbing); Smoke Signal proves strong-reference RSVPs on ATProto and lexicon.community
   now stewards neutral event schemas with Smoke Signal + OpenMeet as first interop.
   New caveat from this session's design work: public-lexicon interop is right for
   community events and wrong by default for dating-adjacent surfaces, so interop is a
   per-scope publish choice, never a blanket alignment (see §11.1 of CONTACT.md).

## PART 2: The mapping, realigned

The analytic sort is unchanged: **provenance-shaped** (fits the razor), **utility-shaped**
(deliberately outside), **centralization-consequence** (structurally addressed). What
changed is the evidence column and a fourth category the original lacked:
**designed-seam** items, where the protocol supplies a provenance floor and names exactly
where a utility helper attaches, at whose discretion. The original report's binary
(fits / doesn't fit) hid that seam; most of the dating domain lives on it.

### Table A: Provenance-shaped needs (fit, with evidence grades)

| Need / pain point | Capability that maps | Evidence grade | Sub-domain |
|---|---|---|---|
| Organizer succession without losing the group | Group-as-institution: genesis-hash lineage persists across stewards; roles granted/revoked by governed co-signed ops | Demonstrated (tier proof, RUN-17) | Meetup/community |
| Roster ownership and portability | Structural consent: membership facts are two-sided records the group re-derives; scopes are lineages, not operator rows; another operator can serve the same grants | Demonstrated (RUN-17; roles as separate replaceable processes) | Meetup/community; friendship groups |
| No conscription onto lists or rosters | Nobody appears without a self-authored record; nobody held after deleting it; conscription cannot construct a valid roster | Demonstrated (RUN-17 membership facts) | All three |
| Verifiable, non-inflatable member/attendance counts | Counts re-derivable by anyone from self-registrations; churn equally provable; audit refold | Demonstrated (RUN-18 auditable-reach refold; 100k-roster scale rehearsal) | Meetup/community; events |
| RSVP bound to a specific event version | Envelope antecedents carry the event envelope hash; "responded to this version" is structural | Design (falls out of A.5 envelope shape; Smoke Signal proves the pattern on ATProto) | Events |
| Detecting silent deletion / withheld history | Per-author chaining: never-existed vs retracted vs withheld-from-me is a three-way distinction a reader computes | Demonstrated (RUN-18) | All three |
| Membership tiers, incl. paid, for readers and writers | Two-axes tier ladder; membership-interval-scoped history | Demonstrated (RUN-17 interval backfill) | Community; supper clubs; publications |
| Paid delivery without authority capture | Plane separation: SLA sells delivery never authority; revoked access fails closed; all delivery roles optional and replaceable | Demonstrated (RUN-17 three-role separation) + doctrine | Directly answers Meetup/Bending Spoons |
| Disputes surfaced, never auto-resolved | Content-bound quorum, sealed deliberation with public verdicts, contradictions to human adjudication; terminus is fork | R7 Verified for the count; contradiction handling Modeled | Community governance |
| **Consent-enforced contactability** (new) | Contact policy as a self-authored record, enforced at validate-before-relay: non-conforming knocks dropped unpropagated; `closed` is the unpublished default | Design (CONTACT.md §4; gate machinery proven, clause new) | Dating; friendship |
| **First contact as a bounded act** (new) | Structured knock: bounded text, zero media, standing attached; one envelope, not a channel; silence is a first-class outcome authored as no record anywhere | Design (CONTACT.md §5) | Dating; friendship |
| **Interest revealed only when mutual** (new) | Fair-reveal commit-reveal: both commit, reveal fires on match, DS is an untrusted rendezvous; the incumbent match mechanic without a matchmaker in the middle | Design over an existing module (per its run grade) | Dating |
| **Vouched-by / verified-person as portable facts** (new evidence) | Attestation family: co-signed edges with ceremony grades, scoped vouches with qualifying antecedents (closed class, V1), mutual-count-without-identity, graded resolvability (counterparts resolve, strangers get cardinality, V4) | Modeled (attest-family, RUN-ATTEST-01..04, 91 tests red-first) | Dating trust layer; community gating |
| **Verified-person issuance without biometric custody** (new) | Co-op-as-issuer: anchor credentials, sibling unlinkability absolute, era-grain transparency (signed tree heads over keyed commitments, holder-stapled inclusion proofs, no verifier callback, V5-V6), free era-reissue as voluntary re-engagement, covenant as governed rule | Modeled (RUN-ATTEST-02/04) | All three; the Face Check alternative |
| **Revocable contact with nothing left behind** (new) | Capability supersession + causal cut at the gate; per-device delegation gives lost-phone vs done-with-person granularities; auto-block on pull-back; blocks are provider-visible policy facts, never public records | Design (CONTACT.md §7-8) over proven delegation (RUN-17) | Dating; friendship |
| **Attendance disclosure as a governed dial** (new) | Three positions per scope: publicly re-derivable RSVPs; steward-co-signed aggregate only; nothing. Participation trace erasable by the person it describes (open-tier self-registration, blinded) | Design (CONTACT.md §10.2-10.3) over RUN-18 audit machinery | Events, esp. dating-adjacent |
| **Late-binding venue** (new) | Venue as its own roster-gated record released near event time; the proven curated-dinner pattern at the cost of one envelope | Design (CONTACT.md §10.4) | Events |
| **Age gating as an auditable rule** (new) | Verified-18 standing as genesis admission policy; operator refuses to serve without it; liability deliberately placed with the accountable entity | Design (CONTACT.md §11.2) riding the anchor credential | Dating discovery |
| **Evidence-capable messaging** (new) | Provider-visible posture: signed cleartext is non-repudiable; reporting breaks no seal; retention covenant (screening data expires, reports pin) | Design (CONTACT.md §3) | Dating; friendship first contact |

### Table B: Utility-shaped needs (outside the protocol), now with the seam named

| Need | Status in v2 | Where it lives, and what the protocol supplies underneath |
|---|---|---|
| Matching / compatibility scoring | **Split.** The baseline is now provenance: users rank a shared attribute vocabulary themselves; assertions are user-authored facts; fit is a transparent evaluation either client recomputes ("you matched because X" is inspectable). Scoring beyond that stays utility | Baseline at protocol+product; advanced matchers are opt-in helpers at user discretion, app layer, never default |
| Discovery and recommendation | Unchanged: ranking and relevance are utility | App layer; catalogue itself is roster-gated (profiles never public records) |
| Swipe/prompt UX, conversation design | Unchanged: product design | Product layer |
| Safety **judgment** (is this a scammer, is this abuse) | **Split.** Judgment stays utility, but screening now has a designed home: the provider-visible operator reads to filter, at a seat with zero standing, under a published retention covenant, with the jurisdiction sentence in the banner | Operator helper role at deployment layer; protocol supplies the posture, the gate, and the evidence property |
| Anonymity / staged identity in early dating | **Largely withdrawn from "centralized is better."** Mutual reveal hides interest until mutual; blinded scopes hide participation; anchor personas hard-split graphs with no discernible default; graded resolvability gives strangers cardinality only. What remains utility is presentation choreography (photo blurring, staged profile reveal) | Product layer over protocol primitives that now exist |
| Moderation content decisions | Unchanged: who-is-right is utility; the protocol proves what was said and surfaces contradiction | Humans at the edges; governed gates for exclusion (see Table C, recidivism) |
| Sybil-resistant reputation **scores** | Unchanged and now doubly deliberate: scalar trust rejected on principle ("trust... for what"), portable proof-of-personhood rejected on the record (V7: only one-credential-per-ID gives real uniqueness; anything short is gameable; uniqueness is group-local vetting, never a portable credential) | Apps may weight facts however they like; the protocol will not mint the score |
| Cold-start liquidity | Reclassified from "centralization is better" to **sequencing**: discovery launches as a feature of events (co-presence knocks, event-cohort reveals), so early users get value with zero network effect; the standing catalogue arrives seeded by ceremony-graded edges | Product strategy (Croft layer), stated in CONTACT.md §15 |

### Table C: Centralization consequences (structurally addressed)

| Pain point | Root cause | v2 answer |
|---|---|---|
| Meetup fee capture, roster hostage | Single owner controls the graph and re-prices at will | Scopes are lineages; grants are portable; delivery roles demonstrated as separate, replaceable processes (RUN-17). The Bending Spoons IPO makes this the strongest row: the incumbent model is now explicitly a roll-up machine |
| Losing members on terms changes | Members exist only in the platform DB | Consent rosters re-derivable by the group; leaving is deleting your own record |
| Inflated / opaque counts | Counts are platform-asserted | Anyone re-derives the count; churn provable; audit refold demonstrated (RUN-18). Raya's 1M-vs-15M spread and asserted MAU everywhere remain the foil |
| Conscription onto lists | Lists describe non-consenting people | No roster entry without the subject's own record; conscription cannot construct a valid roster (RUN-17 negative tests) |
| Ghost-town groups after ownership change | Community value trapped in one company | Group-as-institution persists across stewards and operators |
| Pay-to-win match scarcity | A single matchmaker monetizes the bottleneck | Directly weakened now, not just indirectly: mutual reveal and the knock are protocol acts no operator can ration, price, or paywall; there is no "see who liked you" toll booth because interest disclosure is the users' own commit-reveal. The covenant forbids monetizing identity data |
| Verification as biometric centralization | No shared trust primitive across apps | Co-op anchor credentials: portable, unlinkable across personas, transparently issued at era grain, no biometric custody, no verifier callback. Honest cost stated: paid recidivism gets N chances; exclusion concentrates at the governed issuer, not at a face database |
| Screening as surveillance | Safety features used to justify total content access | Posture is chosen per scope with banner language; screening reads expire under a published covenant; sealed is always one consented transition away; silence preserves the more protective state |

### Sub-domain verdicts (revised)

- **Meetup/community: strong fit, now demonstrated, still the beachhead.** Every row of
  the organizer bundle has executable evidence. What remains is product surface (the
  group's face, the invite pad) and the A.10 owner decisions. The Bending Spoons IPO
  turned the target user from a hypothesis into a growing refugee population with a
  named, publicly documented antagonist.

- **Friendship: strong fit on the group side, seam-fit on the match side.** Groups,
  rosters, and consent are the demonstrated machinery; friend-matching remains utility
  but now sits on the transparent attribute baseline, and the sad fact the original
  report noted in passing (friendship itself being monetized) becomes the positioning:
  the co-op model can offer the group substrate without a match-scarcity business model
  to protect.

- **Dating: narrow fit became real fit at the contact layer; the matching loop stays
  out, on purpose.** What changed: the sealed tier is real at loopback grade and dyads
  sit inside its ceiling; CONTACT.md specifies the whole first-contact model (policy,
  knock, mutual reveal, postures, blocks, transitions, banners); the attestation lane
  closed all ten owner calls with 91 tests behind it; and the safety story is no longer
  a gap but a designed trade between two named protection models. What did not change:
  no matching engine, no discovery ranking, no reputation scores, no
  proof-of-personhood, and person-targeted unilateral public reviews are excluded in
  v1 (aligned with the V3 deferral). The original report's hard constraint (privacy
  plus Sybil resistance killed every prior attempt) is now answered in halves: privacy
  by blinded scopes, mutual reveal, anchor personas, and graded resolvability; Sybil
  by fee friction plus issuer-side exclusion, with the residual honestly priced rather
  than claimed solved.

### Prior art lessons (revised)

Unchanged lessons: do not lead with dating as a product; ride community.lexicon for
public event interop; network effects are the constraint for federated events. Revised
lesson: interop is a per-scope publish choice, because the coherence rule (records that
describe people never touch a public path by default) collides with public lexicons on
dating surfaces. The web-of-trust graveyard lesson lands differently now: the corpus
did not solve what killed Advogato and BrightID; it declined to attempt it (V7's
recorded rejection), which is the defensible position the graveyard teaches.

### Where centralization remains genuinely better, or the model pays a stated cost

- **Matching quality at scale.** Unchanged; deliberately conceded.

- **Determined, funded abuse.** Face Check's one-face-one-account stops a paying
  recidivist at attempt one; the co-op model stops them at credential refusal after
  adjudication. N chances versus one is a real gap, stated in the design rather than
  hidden.

- **Marketing-bought liquidity.** Tinder's $230M budget has no analogue; the answer is
  sequencing (events first), which is a strategy, not a neutralizer.

- **Sealed-tier reach.** The sealed browser story is deferred; a fully private tier-two
  event asks casual invitees to install a native app. Real friction today.

- **Liability seats.** Unchanged, but now embraced: the design deliberately places age
  verification and screening liability with accountable entities (operator, co-op)
  rather than pretending a protocol can hold them.

## Recommendations (revised)

1. **Lead with meetup/community; the pitch is now demonstrable, not conceptual.** The
   demo is the RUN-17/18 bundle run live: succession, roster portability across an
   operator swap, and a count the audience re-derives themselves. Target the
   Bending Spoons refugee explicitly; the IPO made the antagonist public and permanent.

2. **Ship events with the CONTACT.md dials.** Attendance-disclosure dial (aggregate-only
   default for dating-adjacent), late-binding venue, co-presence standing, permanence
   split by role. Interop with community.lexicon where the scope elects publication.

3. **Run RUN-CONTACT-01 before any dating-adjacent surface.** The nine proof sets are
   sketched red-first; the gate clause, structured-knock bounds, silence semantics,
   and posture transitions are the load-bearing safety claims and should be tests
   before they are marketing.

4. **Launch discovery as a feature of events** (CONTACT.md §15): event series, then
   co-presence knocks and event-cohort mutual reveal, then the standing catalogue
   seeded by ceremony-graded edges. This is the cold-start answer and the Timeleft
   lesson in one move.

5. **Position the co-op credential as the anti-Face-Check**, with the honest-cost
   sentence in the open. The market is proving demand for verification; the
   differentiator is verification without biometric custody, without linkable
   personas, and with era-grain transparency anyone can audit.

6. **Keep the counts wedge as the lighthouse**, now with the audit refold as a live
   demo rather than a claim.

7. **Per-deployment posture presets before launch** (CT OC-7): the provider-visible
   default is right for mainstream open discovery and wrong for audiences where the
   operator's readability is itself the threat; the jurisdiction sentence is mandatory
   banner content either way.

## Caveats (revised)

- **Grades are mixed and stated.** Demonstrated: tier proof and reception machinery
  (RUN-17/18). Modeled: attestation family and issuer transparency (RUN-ATTEST-01..04,
  91 tests, loopback-grade environments). Verified/loopback: group-seal. Design only:
  everything introduced in CONTACT.md, including the gate clause, knock, mutual-reveal
  integration, postures-as-dial, and all banner semantics. Nothing here claims field
  deployment, adversarial network conditions, or usability evidence.

- **Vendor-asserted numbers remain vendor-asserted** (Raya's spread, Timeleft's ARR,
  Match's turnaround framing, the $200-300/month figure). Attributed, not endorsed.

- **The Bending Spoons IPO facts were grounded in this session's searches of July 2026
  coverage**; the acquisition/layoff/pricing record per target is corroborated across
  independent outlets but individual figures (exact staff percentages) vary by report.

- **ATProto still lacks native private/permissioned data**, which constrains
  ATProto-side integration surfaces; the private side here rides Drystone's own
  substrate (sealed tier, blinded backplane scopes), not ATProto, and the
  ATTEST-ATPROTO-MATCHUP work bounds exactly which abilities ATProto currently
  supplies. Correlator risks at the ATProto seam are on file (F-AT-6: PLC op-log
  timing, hosting, enumerability).

- **Point-in-time snapshot, July 2026.** The verification and AI-matchmaking landscape
  moves monthly; the Romance Scam Prevention Act status and Tinder's MAU trend will
  shift.
