# Dating, Friendship, and Meetup Apps in 2025-2026: Current Landscape and Fit with Drystone

author: Research agent (commissioned)

date: 2026-07

status: draft for review

---

`Commissioned research deliverable, filed 2026-07 (content-faithful to the delivered report). It
answers the use-case question "could dating / friendship / meetup apps fit the Drystone group
substrate, and how would the components map?" — the analytical companion to the research plan in
`../../beta/cairn/social-lexicon-group-research-brief.md`. Substrate it maps onto:
`../experiments/appview-infra/PUBLICATIONS.md` and `GROUPS.md` (the group tier model, two-axes
membership/write policy, reception completeness, auditable counts). Platform-critique overlap lives in
`../../beta/fenced/` and `../../beta/activism/`. Point-in-time snapshot (July 2026); every
vendor-asserted number is attributed, not endorsed — see Caveats.`

## TL;DR
- **Meetup/community/IRL-event apps are the natural fit for Drystone; dating's core loop is mostly utility-layer and a poor fit.** The strongest match is the group/roster/governance domain (organizer succession, non-inflatable attendance counts, portable member lists, consent-based rosters, membership-scoped history), which is exactly where the incumbent platforms are failing users after ownership changes and fee hikes. Dating is dyadic, privacy-heavy, and dominated by matching/ranking/safety-vetting, all of which are utility-shaped and deliberately outside Drystone's provenance razor.
- **The industry is mid-pivot away from swiping toward IRL and group-based meeting, and toward AI**, which is both a feature (AI matchmakers, concierges) and a threat (AI-generated profiles, industrialized romance scams). Trust and identity are the central unsolved problems, and every prior decentralized attempt at dating has failed on privacy plus Sybil-resistance, not on protocol plumbing.
- **The pain points Drystone structurally addresses are real and current**: platform lock-in and roster loss (Meetup under Bending Spoons), inflated/opaque active-user counts, conscription onto lists, fee capture, and ghost-town groups. But Drystone offers no advantage on the things that actually make these apps work day to day: matching, discovery, safety screening, anonymity in early dating, and liability. It is best understood as trust/roster/governance infrastructure beneath an app, not an app.

## Key Findings

**Definitions used in this report.** *Provenance* = a verifiable record of what was said, by whom, in what causal order (Drystone's domain). *Utility* = judgments of who is right or who is a good match (left to humans/apps at the edges). *Protocol layer* = the shared data/verification rules; *deployment layer* = who hosts/relays; *product layer* = the app users actually see. I separate these throughout.

1. **Dating apps are in a measurable decline-and-turnaround fight, financialized around subscriptions.** Match Group's flagship Tinder had monthly-active-user declines that only narrowed to -7% year-over-year in March 2026 (its slowest decline in 31 months) and -6.6% in April, with registrations returning to growth for the first time since June 2020. Total Match Group paying users fell 5% year over year to 13.5 million in Q1 2026 even as revenue per payer rose 10% to $20.90. Hinge is the growth engine (Q1 2026 direct revenue $194 million, up 28%, payers up 15% to 2 million, targeting $1 billion by 2027). Bumble cut exactly 240 jobs (about 30% of its global workforce) in a move its board approved June 23, 2025, expecting roughly $40M in annual savings against $13-18M in severance charges. Match Group's market value fell from an all-time high near $46.58 billion (2021) to roughly $7.5 billion (2025).

2. **User sentiment is corroborated across independent surveys: fatigue, disappointment, and safety concerns dominate.** A Forbes Health/OnePoll survey (2024, 1,000 US daters) found 78% report burnout; follow-up reporting put Gen Z burnout at 79%. Pew Research Center (February 2, 2023) found that 49% of US adults say dating sites/apps are "not at all or not too safe" versus 48% who say very or somewhat safe, with a majority of women (57%) saying they are not a safe way to meet people. The UK saw about 1.4 million people leave dating apps between 2023 and 2024. Around 84% of Gen Z and Millennial daters report having been ghosted.

3. **"Enshittification" is now a documented, even academic, critique specifically applied to dating.** The dating-app business model creates a perverse incentive (a matched, departed user stops paying), and a Groundwork Collaborative report ("Swipe Right to Pay," 2025) argues apps hide better matches behind paywalls, restrict free features, and push upgrades. Per Match Group's own Form 10-Q, the parties reached an agreement in principle on June 9, 2025 to settle the long-running FTC case for $14 million, and the court approved the settlement on August 13, 2025 (the FTC announced it August 12), resolving allegations over deceptive guarantees, fake-interest messages, and hard cancellation. A separate class action (Oksayan v. Match Group, filed February 14, 2024) alleges addictive design; it was ordered to arbitration in December 2024.

4. **Verification is the current battlefront, and it is centralizing biometric trust.** A Match Group press release (October 22, 2025) made Tinder "Face Check" facial-liveness verification required for all new users in seven countries and California, reporting an over-60% decrease in exposure to potential bad actors and over-40% fewer bad-actor reports; Yoel Roth, Head of Trust & Safety, called it "perhaps the most measurably impactful Trust and Safety feature I've seen in my 15-year career." Bumble uses opt-in ID verification via Veriff; Bumble BFF requires selfie verification for every member. This matters for the mapping: the market is solving identity/trust with centralized biometrics because no shared trust primitive exists across apps.

5. **Romance scams have industrialized via AI, raising the stakes on identity.** FTC Consumer Sentinel data show 55,604 romance-scam reports totaling $1.16 billion in January-September 2025 (up 22% year over year), with a median loss of $2,218 in Q3 2025 and more than 11,200 reports and $398M lost in that quarter alone. "Pig butchering" (romance-baited crypto investment fraud) losses are recorded largely under FBI investment-fraud categories that rose to $5.8 billion in 2024. AI-generated faces defeat reverse-image search; Norton's 2025 report found one in four daters targeted by scams and under half able to distinguish AI-generated profile photos.

6. **The clear trend is away from swiping toward IRL, group-based, and curated meeting.** Timeleft (dinners with five strangers, personality-matched, restaurant revealed the night before) hit €18M ARR and 150,000 monthly participants in August 2025 across 200+ cities in 52 countries (per CEO Maxime Barbier), generating roughly $1M/week for partner restaurants. Bumble launched a standalone BFF friendship app (September 2025) built on the Geneva community tech it acquired, with a Groups feature and all features free. 222 uses personality quizzes and machine learning to place strangers into curated in-person group experiences ("engineering chance"). Eventbrite reported "friending" events up 35% year over year in 2025.

7. **Meetup's ownership change is the clearest real-world case of the lock-in pain Drystone targets.** Bending Spoons acquired Meetup in January 2024. Organizers report tripled fees, paywalled core features (including a "pay to skip the waitlist" Meetup+ prompt), and broken member email; one organizer with 10,000+ members reported email to members "has become undeliverable." Meetup reports 60 million users. Bending Spoons has a documented pattern (Evernote, WeTransfer, Komoot) of price hikes and mass layoffs post-acquisition.

8. **Event tooling has fragmented into lightweight, low-lock-in products.** Partiful (~500,000+ monthly active users, TIME100 Most Influential Companies 2025, $27.3M raised) is text-blast, casual, and app-nudgy; Luma is the clean, community/ticketing-oriented "Silicon Valley" tool with a freemium model; Apple Invites launched early 2025 but is iOS-only. Facebook Events persists mainly for public discovery. Discord is a major host of ongoing (including local) community organizing, with formal Friend vs Community server types and organizer-side moderation tooling.

9. **Every prior decentralized/federated attempt at dating has failed or stalled, and the reason is instructive.** Alovoa (open-source dating, AGPL) is alive (v2.0.0, January 2025, ~710 GitHub stars) but is self-hostable-centralized, not federated; its ActivityPub request has sat open since February 2020. Nostr and ATProto dating are effectively vaporware. The recurring blockers named in fediverse discussions are privacy (public-broadcast protocols are hostile to private dating data) and Sybil-resistance/proof-of-personhood, not data plumbing. ATProto still lacks private/permissioned data, which Bluesky's own docs acknowledge.

10. **The event/RSVP and group primitives Drystone would build on already exist in early form on ATProto.** Smoke Signal (Nick Gerakines, built on ATProto, rewritten in Rust and open-sourced by its one-year mark in July 2025) uses strong references (AT-URI + CID) so an RSVP proves you responded to a specific version of an event, and has wrestled explicitly with tombstones vs deletion. Bryan Newbold's "Community Spaces on AT Protocol" proposal describes bidirectional invite/confirm membership records and role mechanisms, but remains unimplemented. Mobilizon (ActivityPub events) is alive but constrained by weak network effects; Gathio is a deliberately minimal, accountless, ephemeral event tool.

## Details

### PART 1 — The current landscape

#### Dating apps

**Market structure and numbers.** Match Group operates the largest portfolio (Tinder, Hinge, Match, Meetic, OkCupid, Plenty of Fish, Azar, BLK and others). In 2025 Tinder held roughly 25% market share, Bumble about 24%, Hinge about 18% (Groundwork Collaborative). Match Group beat Q1 2026 estimates ($0.95 non-GAAP EPS vs ~$0.62 consensus; revenue $863.93M, a slight miss) on the strength of a Tinder "product-led turnaround." Management guidance for Tinder in FY2026 was for direct revenue to decline at roughly the same rate as 2025, including a three-point headwind from user-experience tests and a one-point headwind from the Face Check rollout, with a $230M Tinder marketing budget. Jefferies analysts remained cautious, citing structural Gen Z concerns and "multiple false starts." Grindr (not Match-owned) grew 5.2% to 15 million MAU in 2025 (1.26 million paying), reported revenue of $440M, and is pushing an AI-driven "Edge" premium tier; users and researchers describe its degradation as "enshittification."

**Core mechanics, now diversifying.** The classic loop is swiping (Tinder, Bumble) versus prompt/comment-based liking (Hinge, "designed to be deleted"). Match-group matchmaker/AI models and third parties are proliferating: Tinder's "Chemistry" (AI-curated daily shortlist, rolling out late 2025), Hinge's AI Core Discovery Algorithm (reported +15% matches since March 2025), Iris Dating (AI trained on rated faces to predict mutual attraction, ~4 million users), and concierge/agentic entrants (Sitch, Keeper, Amata's "no swipe, no DM" pay-per-date model at $16, Known's voice-AI profiling). AI assistants (Rizz at $9.99, YourMove.AI) ghostwrite openers; Bumble's CEO floated an "AI dating concierge" that was widely mocked. Roughly a quarter of singles used AI to enhance dating in 2025 (up sharply year over year).

**Monetization mechanics.** Subscription ladders (Tinder+, Gold, Platinum; Hinge tiers), consumable boosts and super-likes, and pay-to-see-who-likes-you gating are standard. The Groundwork report characterizes this as artificial match scarcity: matches are "programmed to be locked away." Reported daters spend $200-300+/month (Match's "Singles in America"/Kinsey data). Revenue per payer is rising even as payer counts fall, the defining financial signature of the sector.

**Verification and safety.** Tinder Face Check (mandatory facial liveness for new US users, October 2025) stores a "non-reversible, encrypted face map and face vector" and enforces one-face-one-account; Bumble uses Veriff; Match plans to extend Face Check across its portfolio in 2026. The House passed a Romance Scam Prevention Act (June 2025) that would require apps to warn users who receive messages from previously banned fraud accounts. Women disproportionately report unwanted/harassing contact; Pew found a majority of women consider apps unsafe.

**Niche/newer entrants.** Feeld (ethical non-monogamy/kink) reported membership up 368% 2021-2025 and about 30% year-over-year growth since 2022. Raya (invite-only, ~$15/month) remains deliberately gated (reported figures vary widely, from ~1 million to ~15 million registered depending on source, a good example of unverifiable platform-asserted counts). Thursday (only works on Thursdays) and Happn (proximity-based, ~100 million users) round out the long tail.

#### Friendship apps

Friend-finding differs mechanically from dating: it is less dyadic, more group- and interest-oriented, less privacy-sensitive, and less monetized. Bumble BFF relaunched as a standalone app in September 2025 ("The Great Frienaissance"), built on Geneva (acquired 2024), with interest-based matching, photo prompts, mandatory selfie verification, a Groups area (chat/posts/video/IRL planning), and everything free. Bumble discontinued "Bumble for Friends" in the US (kept internationally). Competitors named in coverage include Timeleft, 222, Clockout, Clyx, and Les Amís. Wink and Yubo skew younger (Yubo is a Gen Z live-social app). Reviews of BFF's relaunch complain the discovery algorithm and filters are weak, a reminder that friend-matching is still a utility/ranking problem.

#### Meetup / IRL / community-event apps

**Meetup.com** (60M users) is the cautionary tale: after the January 2024 Bending Spoons acquisition, organizers report tripled fees, new paywalls, "pay to skip the waitlist," and undeliverable member email, with the roster (member list) effectively hostage to the platform. Bending Spoons pledged nearly $50M of investment and a Community Fund, but its track record (Evernote, WeTransfer, Komoot: price hikes, roughly three-quarters staff cuts) frames organizer distrust.

**Partiful** (casual, text-blast, Gen Z; ~500K+ MAU; TIME100 2025) and **Luma** (clean, ticketing/community, freemium) are the leading modern event tools; both are praised for low friction, though Partiful nudges app downloads. **Timeleft** and **222** are curated-group-meeting products (personality-matched dinners/experiences; own-check payment; subscription plus curation fees). **Facebook Groups/Events** persist for public discovery. **Discord** is a dominant host of ongoing community organizing (Friend vs Community server types, moderation tooling, invite links), though it is chat-first, not event-first, and offers no member-list portability.

**Organizer-side pain points (cross-platform).** Platform fee capture; roster ownership and the inability to export member lists; lock-in and losing members if terms change; no-shows and spam; and the "ghost town" problem when a platform's changes kill a once-vibrant group. Timeleft field reports also show a concrete attendance-trust gap: one attendee reported only two of the expected group showing up, "no guarantee who will actually arrive."

#### Cross-cutting themes

- **Swiping to IRL/group.** Corroborated by Eventbrite "friending" events up 35% YoY (2025) and board-game dating events up 55%, Tinder testing group/Double Date features, and the entire Timeleft/222 category.
- **AI's double role.** Feature (matchmakers, concierges, opener-writers) and threat (AI-generated profiles, voice cloning, deepfakes; 78% of users say AI makes scams harder to spot).
- **Identity/trust.** The unsolved core problem; the market's answer is centralized biometrics (Face Check, Veriff, selfie gating).
- **Data portability/lock-in.** Users can export their own data via GDPR/CCPA (Tinder DMD tool, Hinge "Download My Data"), but this is a personal-copy right, not a portable, verifiable, reusable identity/roster/reputation. Organizers generally cannot take their rosters with them.
- **Trust in counts.** Platform-asserted MAU/attendee numbers are taken on faith; Raya's wildly divergent user figures and the historical Facebook video-metrics inflation controversy are illustrative.

### PART 2 — Mapping needs to Drystone capabilities

The central analytic move: sort each need into **provenance-shaped** (fits Drystone's razor), **utility-shaped** (deliberately outside the protocol), or **centralized-model-consequence** (a pain point that exists because a single company owns the graph, which Drystone's model structurally addresses). Then assess the three sub-domains separately.

#### Table A — Provenance-shaped needs (natural Drystone fit)

| Need / pain point | Drystone capability that maps | Sub-domain where it matters most |
|---|---|---|
| Organizer succession without losing the group | Group-as-institution: genesis-hash principal whose lineage persists as stewards change; roles electable/removable via co-signed operations | Meetup/community |
| Roster ownership and portability | Structural consent on rosters: nobody appears without a self-authored record; membership survives the platform because it is not tied to one host | Meetup/community; friendship groups |
| No conscription onto lists | Structural consent (contrast Bluesky lists, which can conscript); nobody held after deleting their record | All three; especially friendship/community |
| Verifiable, non-inflatable attendance / member counts | Auditable counts: a roster folded from public self-registrations is re-derivable by anyone; churn equally provable | Meetup/community; event RSVPs |
| Proof "I RSVP'd to this specific event version" | Provable multi-party fact + per-author chaining (mirrors Smoke Signal's AT-URI+CID strong refs) | Events |
| "This person is vouched by N mutuals" (as a fact, not a score) | Provable multi-party co-signed fact; the vouch is provenance, the weighting is left to the app | Dating trust layer; community gating |
| Detecting silent deletion / withheld history | Tamper-evident per-author chaining: never-existed vs retracted vs withheld-from-me distinguishable | All three |
| Membership tiers (free/gated, reader/writer) | Tier ladder with zero new machinery; membership-interval-scoped history access | Community; paid supper clubs; premium groups |
| Paid delivery without platform capturing authority | Operator/plane separation: SLA sells delivery, never social authority; revoked access fails closed | Directly answers the Meetup/Bending-Spoons capture problem |
| Genuine disputes surfaced, not auto-resolved | Governance: content-bound quorum approvals, sealed deliberation with public verdicts, contradictions surfaced for human adjudication; terminus is fork, not verdict | Community governance; moderation appeals |

#### Table B — Utility-shaped needs (deliberately outside Drystone)

| Need | Why it is utility, not provenance | Where it lives |
|---|---|---|
| Matching / compatibility scoring | A judgment of "who is a good match," explicitly the utility side of the razor | App layer / AI |
| Discovery and recommendation | Ranking and relevance are subjective utility | App layer |
| Swipe/prompt UX, conversation prompts | Product design, not fact-ordering | Product layer |
| Safety vetting / scam detection | Judgment calls requiring ML, human review, and liability | App + centralized trust vendors |
| Anonymity/pseudonymity in early dating | Requires withholding identity until mutual consent; provenance's transparency is the wrong tool here | App layer (arguably centralized is better) |
| Moderation content decisions | "Who is right" is utility; Drystone can prove what was said and surface contradiction, not adjudicate | Human moderators / labelers at the edges |

#### Table C — Pain points that are consequences of the centralized model (Drystone structurally helps)

| Pain point | Root cause | How Drystone's model addresses it |
|---|---|---|
| Meetup fee tripling, roster held hostage | Single owner controls the graph and can re-price at will | Roster and group identity live in provable records the group controls; a delivery operator can be swapped without losing authority or membership |
| Losing member lists if terms change | Members exist only in the platform DB | Structural consent rosters are re-derivable by the group, not owned by the host |
| Inflated/opaque active-user counts | Counts are platform-asserted | Auditable, re-derivable counts; churn provable |
| Conscription onto lists (Bluesky-style) | Lists can include non-consenting people | No record on a roster without the subject's own authored record |
| Ghost-town groups after ownership change | Community value captured by, and trapped in, one company | Group-as-institution persists across stewards and operators |
| Pay-to-win / artificial match scarcity | Monetization incentive of a single matchmaker | Not directly solved (matching is utility), but a portable reputation/roster layer weakens any single app's lock-in, reducing the leverage that enables enshittification |

#### Sub-domain verdicts

- **Dating (dyadic, privacy-heavy, utility-dominant): weak fit for the core loop, narrow fit for a trust sidecar.** The matching engine, discovery, anonymity, and safety vetting, that is, the parts users actually pay for, are all utility or require private/permissioned data and liability that a provenance protocol deliberately does not provide. The one genuinely provenance-shaped opportunity is a *portable, consent-based trust/vouching layer* ("verified real person," "vouched by N mutuals," "previously reported by co-signed records") that apps could consume. But note the hard constraint: dating needs privacy and Sybil-resistance, and every decentralized dating attempt has died on exactly those two rocks. Drystone's public-provenance model helps with the "is this a real, corroborated person" question only if paired with a private layer; its own Layer-2 private-per-viewer model and MLS-based sealed messaging are relevant here, but this is a hard, unproven space.

- **Friendship (group- and interest-oriented, semi-private): moderate fit.** Friend-matching itself is utility (ranking), but friendship increasingly happens in *groups* (Bumble BFF Groups, Geneva, Discord), and groups are exactly Drystone's atom. Consent-based rosters, portable community membership, and non-inflatable group counts map well. The matching/discovery remains an app concern.

- **Meetup/community (group-shaped, roster-heavy, governance-shaped): strong fit, and the recommended beachhead.** Nearly every acute, current pain point (Meetup lock-in, roster loss, fee capture, opaque counts, organizer succession, no-show/attendance proof, membership tiers, operator-vs-authority separation) is provenance-shaped and lands directly on Drystone's machinery. The utility residue (event discovery, recommendation) is small and can be left to apps.

#### Prior art: what to learn from

- **Decentralized dating is a graveyard.** Alovoa (alive but centralized-self-hosted, AP request open since 2020), Nostr/ATProto dating (vaporware). Lesson: do not lead with dating; the unsolved problems (privacy, Sybil-resistance) are not the problems Drystone's provenance razor solves.
- **Event/RSVP primitives already work on ATProto.** Smoke Signal proves the strong-reference RSVP pattern (RSVP references a specific event version by AT-URI+CID) and has already reasoned about tombstones vs deletion, directly relevant to Drystone's "never-existed vs retracted vs withheld" distinction. Bryan Newbold's Community Spaces proposal independently arrives at bidirectional invite/confirm membership and role records, converging on Drystone's structural-consent roster. lexicon.community shows a governance model for shared schemas (Smoke Signal + OpenMeet interop is the first example).
- **Federated events have modest but real traction; network effects are the constraint.** Mobilizon (ActivityPub) is alive and used by activist groups for censorship-resistant, self-controlled RSVPs, but its own team names "fatigue due to a lack of network effects" as the risk. Gathio's minimal, accountless, ephemeral design (no accounts, password-per-event, 7-day auto-deletion on the flagship instance) is a strong privacy-first model but deliberately non-networked.
- **Web-of-trust vouching is intellectually influential but hard to scale and keep attack-resistant.** Advogato (dead 2016; its trust metric later shown attackable), Duniter/Ğ1 (alive but ~6,000 members after 8+ years, requiring 5 certifications within 5 steps), BrightID (~73,000 verified, crypto-niche, plateaued), Keybase (zombie after the $42.9M Zoom acquisition in May 2020, confirmed via Zoom's SEC 10-Q). Lesson for the "vouched by N mutuals" feature: Drystone can make the vouch a *provable fact* cheaply, but converting vouches into a trustworthy *utility* (a Sybil-resistant reputation score) is the hard, historically unsolved part, and by Drystone's own razor that scoring belongs at the app edge, not in the protocol.

#### Where Drystone offers no advantage, or centralization is genuinely better

- **Matching quality.** The thing users most want is pure utility; Drystone is neutral-to-irrelevant here.
- **Early-dating anonymity and safety screening.** These need private data, active ML moderation, and someone to hold liability. A centralized operator that can be sued and can run trust-and-safety teams (Match's Face Check, Yoel Roth's org) is genuinely better positioned than a center-free protocol.
- **Cold-start network effects and discovery.** Centralized apps buy growth (Tinder's $230M 2026 marketing budget); Mobilizon's struggles show the decentralized cold-start problem is real.
- **Liability and legal compliance** (age verification, biometric law, romance-scam warnings mandated by pending legislation) sit more naturally with an accountable legal entity than with a protocol.

## Recommendations

1. **Lead with meetup/community, not dating.** Target the Meetup-refugee organizer as the first user: someone who has just watched fees triple and their member email break, and who wants to own their roster. Build (or partner on an app that builds) the "group-as-institution + consent roster + re-derivable counts + operator-swappable delivery" bundle. This is where Drystone's machinery maps almost one-to-one onto acute, current, documented pain. **Benchmark that would change this:** if a centralized competitor shipped verifiable data portability and roster export as a standard (unlikely given incentives), the differentiation narrows.

2. **Ship the event RSVP + roster primitive next, reusing proven patterns.** Adopt the Smoke Signal strong-reference RSVP pattern and align with lexicon.community governance so you interoperate rather than fork the ATProto events ecosystem. Prove three things publicly: (a) an attendance count anyone can re-derive, (b) organizer succession without member loss, (c) a member's ability to leave and provably disappear from the roster. **Benchmark:** first 50 real recurring groups that survive an operator change with zero roster loss.

3. **Treat dating as a *trust-layer* opportunity only, and only after the private-data model is real.** Offer a portable "verified/vouched" primitive that dating apps can consume, rather than building a dating app. Do not attempt the matching loop. **Threshold to enter at all:** a working, audited private/permissioned layer (Drystone Layer 2 + sealed messaging), because public-broadcast provenance is fatal to dating, exactly as it killed prior decentralized attempts. Until then, dating is out of scope.

4. **For friendship, ride the group wave, not the match.** Position Drystone under interest-group and community-of-friends formation (the Bumble BFF Groups / Discord use case), where rosters and governance matter and privacy needs are milder than dating.

5. **Be explicit in all positioning that Drystone is infrastructure, not an app.** It supplies provenance, rosters, counts, governance, and operator/authority separation; it does not supply matching, discovery, safety judgment, or anonymity. Pair with an app layer and a legally accountable operator for anything touching safety, minors, or biometrics.

6. **Instrument the "trust in counts" wedge early.** The auditable-count capability is a uniquely legible, demonstrable differentiator against platform-asserted MAU/attendance numbers. Use it as the lighthouse feature in marketing to organizers and sponsors who are tired of taking numbers on faith.

## Caveats

- **Vendor-asserted numbers are taken on faith and sometimes conflict.** Raya's user count ranges from ~1 million to ~15 million across sources; Match Group's "turnaround" framing is management's and analysts (Jefferies) openly doubt it; Timeleft, 222, Bumble-commissioned friendship statistics, and Iris's accuracy claims are self-reported. I have attributed these to their sources rather than endorsing them. The Keybase-Zoom $42.9M figure is the one prior-art number confirmed via a primary SEC filing.
- **Several sources are press/blogs/marketing, flagged as such.** Enshittification specifics (slowdatingbln, Medium), some app comparisons (Lemonvite, party.pro), and dating-safety guides (GRASS) are secondary or interested parties; primary anchors used where possible are SEC filings, the FTC, Pew, Groundwork Collaborative, and project repos/docs.
- **Drystone's capabilities are treated as given, not tested.** The mapping assumes the design corpus performs as stated; the private-side (Layer 2, MLS sealed messaging) is the most consequential unknown for the dating sub-domain and should be considered unproven for that use.
- **ATProto itself still lacks private/permissioned data** (acknowledged in Bluesky's own docs), which constrains any near-term dating or sensitive-roster deployment regardless of Drystone's design.
- **This is a point-in-time snapshot (July 2026).** The AI-matchmaking and verification landscape is moving monthly; figures like Tinder's MAU trend and the Romance Scam Prevention Act's status will shift.
- **Forward-looking items are marked as such**: Match's $1B Hinge-by-2027 target, "agentic discovery by late 2026" forecasts, and pending legislation are projections/plans, not accomplished facts.
