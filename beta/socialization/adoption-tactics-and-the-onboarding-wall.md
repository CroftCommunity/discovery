# Adoption tactics and the onboarding wall: the concrete crossing

`Status: socialization layer (Layer 8, brand / voice / adoption). Register: concrete adoption tactics
— the crossing-the-chasm playbook, not the strategy frame. Resolution: complement to
`adoption-strategy.md`, which holds the generic go-to-market argument (the chasm thesis and the three
abstract bridges) and self-flags at open thread T11 that it lacks concrete tactics; this doc supplies
those tactics. It is a direction under active development, not a committed plan. Much of the material
is dialogue-sourced from the project's own design thinking and carries inline verification flags;
external statistics and named studies need a citation pass against primary sources before any outward
use.`

## Overview

`adoption-strategy.md` names *what* a new entrant does about the adoption chasm: hold the chasm thesis
honestly, choose among three bridges (network-effect seeding, institutional mandate, a wedge
use-case), and run enablement tactics that make the chosen bridge carry weight. It is deliberately
generic and flags, at T11, that the concrete tactics are the missing piece. This document is that
missing piece. It carries four concrete plays, each mapped back to the abstract move it makes real:

- the *UX and cultural-inertia wall* and the specific antidotes that get a mainstream user across it —
  the concrete content of the strategy's "be ready for the spike" and "answer cold-start with seeded
  groups" tactics;
- the *event-app Trojan horse* — a concrete candidate for the strategy's abstract "wedge use-case"
  bridge;
- the *digital electric cooperative* analogy — the plain-language device that makes the co-op legible
  to a non-technical person, which the strategy's "make the sustaining-org story legible" tactic
  needs and does not supply;
- the *moat from not having things* — the sharpest articulation of *why* non-extraction is a durable
  competitive position, which is the affirmative case the strategy's "non-extractive sustaining org"
  premise rests on but does not itself argue.

The load-bearing claim is that the chasm is not crossed by a better argument; it is crossed by a set
of concrete, unglamorous moves that lower the cost of the first ten minutes and that only a
non-extractive structure can afford to make.

## Charter: what this document covers

- **In scope:** the concrete adoption tactics — the onboarding-wall antidotes, the event-app wedge,
  the electric-cooperative explainer, and the moat articulation, each with the reasoning for why it
  works.
- **Out of scope (and where it lives):** the strategy frame and the three abstract bridges live in
  `adoption-strategy.md` and are not restated here; the affirmative *feature* set that non-extraction
  generates (protocol-level ad-blocking, transparency-as-a-feature, human-choice feeds, and the rest)
  is the "non-mimicry moat" in `../governance/foundation-cooperative-and-sustainability.md` and is
  cross-referenced, not duplicated; the descriptive map of why incumbents win lives in the fenced
  layer, referenced by `adoption-strategy.md`.
- **Boundary call:** this is the "here are the moves, and why each one works" register. The strategy
  doc decides *which* bridge to lead with; this doc supplies the tactics that make a bridge real. The
  moat section here argues *why* the non-extractive position is defensible; the governance doc
  enumerates the features it makes possible.

## The UX and cultural-inertia wall

The *UX and cultural-inertia wall* is where decentralized and collectively-owned software has
repeatedly gone to die: the gap a mainstream consumer, conditioned by friction-free incumbent
interfaces, hits when trying a values-driven alternative, and retreats. It has two layers, and both
have to be answered.

The first is a **UX barrier**: cognitive overload. Forcing architectural decisions on a new user
(pick a server or relay, verify or back up a key) before showing any content spikes cognitive load
and violates the expectation that a new app works like the ones the user already knows. Onboarding
research puts cognitive overload as the top conversion drop-off; a figure of roughly 68% abandonment
circulates, attributed to a fintech-onboarding analysis synthesizing the "Battle to Onboard" study.
`[external stat, dialogue-sourced; confirm the ~68% figure and its attribution against primary sources
before any outward use.]`

The second is a **cultural-inertia barrier**: users arrive from frustration with an incumbent but
carry incumbent expectations — an instant infinite algorithmic feed, one-click login, their whole
social circle already present. A quieter, un-algorithmic, sparsely-populated stream reads to them as a
*broken product*, not a calmer one. The Great Mastodon Migration is the standing example: a large
influx during a migration spike, more than half of whom abandoned, defeated first by the
instance-selection roadblock and then by a lonely chronological timeline.
`[external example, dialogue-sourced; confirm migration figures against primary sources before outward
use.]`

The antidotes are progressive-layering moves — reveal architecture only when it buys the user
something, never at the door:

- **A soft *guest pass* (anonymous entry).** Let a newcomer explore real feeds before any email, node,
  or tier decision. Introduce account creation only at the first genuinely high-value action, so the
  decision arrives when the user already has a reason to make it rather than as a toll at the gate.
  This is the concrete content of the strategy's "onboarding that survives a sudden influx": the
  cheapest first step has no step.

- **Protocol abstraction (jargon-hiding).** Magic-link or biometric login; provision keys
  automatically behind the scenes; drop the vocabulary that reads as a barrier ("relays," "pods,"
  "shards") in favor of plain names ("feeds," "channels," "security codes"). The architecture is a
  feature to an engineer and a barrier to a teacher or a small-business owner; the wall is built out
  of asking the second person to think like the first.

- **Feed scaffolding via curated *starter packs* (= *graph injection*).** The empty-feed problem is
  answered by human-curated, topic-based bundles of accounts and feeds that a newcomer follows in one
  tap, so the timeline is dense and high-quality from the first session without a data-harvesting
  ranking algorithm. This is the concrete mechanism behind the strategy's "network-effect seeding"
  bridge and its "answer cold-start with seeded groups" tactic: the graph arrives with the user
  instead of having to form around them. A prominent implementation reports starter packs driving a
  large share of daily follows during migration spikes, with users placed in a pack gaining
  materially more followers and engagement. `[external stat, dialogue-sourced; the specific
  follow-share and engagement figures need confirmation against the cited analysis before outward
  use.]`

- **Live-action posts (Farcaster *Frames*).** A pattern from Farcaster turns a static post into a
  live mini-app — poll, vote, mint, buy — actioned without leaving the feed, converting passive
  scrolling into a place where things happen. It is worth studying as prior art for making a feed feel
  active rather than inert. `[external prior art, dialogue-sourced; confirm the Frames mechanism and
  its adoption claims against primary sources before relying on them.]`

- **DNS-handle identity.** Domain-based handles (a handle of the form `name@institution`) give an
  institution zero-cost self-verification and pull authoritative accounts in without a central
  verification bureaucracy. This is the identity-side complement to the strategy's institutional-
  mandate bridge: the institution's own domain is the credential. `[external pattern, dialogue-
  sourced; confirm before outward use.]`

Why load-bearing: these are not polish. Each one removes a specific drop-off point on the path from
"curious" to "present with my people," and the readiness the strategy calls for is exactly the sum of
these moves being in place *before* an inciting-event spike arrives, so the attention a spike delivers
lands on a product a non-technical person can actually enter.

## The event-app Trojan horse (a concrete wedge)

The recurring blind spot of the field is that the incumbents' non-extractive challengers preach to the
choir — they advertise and socialize only inside their existing circles. A wedge has to reach people
who are not already looking. The *event-app Trojan horse* is a concrete candidate for the strategy's
abstract "wedge use-case" bridge: enter through a narrow real-world problem where the peer-to-peer
approach is the obviously better option, for a captive audience with an immediate need.

The problem it exploits is **big-venue-small-pipe**: conferences and festivals overwhelm cell towers
and venue Wi-Fi, and commercial event apps are both expensive and centralized, so when the external
pipe saturates the app becomes a useless brick exactly when it is needed most. `[market claim,
dialogue-sourced; confirm commercial-event-app pricing and the connectivity failure mode before
outward use.]`

The fix is a **local peer-to-peer mesh**: build local data replication into the client so attendees'
phones sync the schedule, map, and messages directly over the venue's local network (or a
device-to-device radio), bypassing the broken external pipe entirely. The event app is a specialized
view of the broader social hub; an attendee thinks they installed a schedule tool and has in fact
installed a node of the network. The onboarding inversion is the whole point: instead of "read a
manifesto, set up a node, arrive in a ghost town," it is "need the schedule, one-tap install,
hyper-local active feed." A tactic worth pairing with it is on-site conversion — turning an event
profile into a member-owner share at the venue, while the user is already present and active.

Why load-bearing: this is the strategy's wedge premise made concrete — a case where peer-to-peer is
not the ideological choice but the *only working* choice, giving real users a real reason to be there
before full-surface parity with an incumbent is reached. It remains a candidate, not a validated
entry point, and the open question the strategy names — whether a wedge both has genuine pull and
bridges outward to the general case — applies here in full. `[direction, dialogue-sourced; not
validated.]`

## The digital electric cooperative (the plain-language device)

The cooperative is hard to explain to a non-technical person in the abstract. The *digital electric
cooperative* is the analogy that makes it land in one breath. When private utilities refused to run
electric lines to rural farms in the 1930s — too costly, too little profit — farmers formed
cooperatives and built the grid themselves, owning the lines the incumbents would not build. A federal
lending program made it possible, and rural electric cooperatives still own much of that
infrastructure today. `[historical CITE; the rural-electrification history is real and attributable,
but confirm specifics and attribute to a primary source before commercial use.]`

The mapping is exact enough to carry the whole co-op thesis without a word of jargon: the incumbents
serve the profitable center and will not build for you on terms that are actually yours; so the people
who want the service build and own the infrastructure together, at cost, and it does not get taken
away because there is no outside owner to sell it. It is the plain-language front door to the same
argument the philosophy layer grounds and the governance layer formalizes — used here as the
explainer for a non-technical audience, not as the argument itself, which is not restated in this doc.

Why load-bearing: the strategy's "make the sustaining-org story legible" tactic needs a device a
prospective member can hold in their head, and an abstract "member-owned non-extractive cooperative"
is not that device. "It is a digital electric cooperative" is.

## The moat from not having things

The *moat from not having things* is the sharpest articulation of *why* the non-extractive position is
defensible, and it is distinct from the "non-mimicry moat" in
`../governance/foundation-cooperative-and-sustainability.md`. That governance framing enumerates the
*features* a cooperative structure makes possible that an extractive competitor cannot copy (ad-
blocking, transparency-as-a-feature, human-choice feeds, and the rest); this section argues the
underlying *why* those are uncopyable and is not a list of features. The two are complements: the why
here, the what there.

The premise is that a friend-to-friend, peer-to-peer client carrying no per-user server cost and no
extraction mandate can offer experiences that are *structurally impossible* for anyone carrying either.
The moat comes from the absence, not from a better version of the same thing:

- **No latency tax on generosity.** Every server-backed app meters generosity against its cost of
  goods — free-tier ceilings, retention windows, "upgrade to keep your history." With no per-user
  marginal cost, the things that are always first to be capped elsewhere (unlimited history, unlimited
  groups, no "three free this month") are free, permanently, with **no asterisk**. The asterisk-free-
  ness is itself the moat: a competitor can match any one of these as a loss-leader, but not all of
  them, forever, against their profit-and-loss statement.

- **The app can be genuinely done.** Software that costs nothing to run is allowed to stop changing.
  Finished-and-stable is nearly extinct because it generates no recurring revenue and therefore no
  reason for a funded org to hold still; a non-extractive project can promise a finished, stable thing
  and mean it.

- **Privacy true by construction.** Not "we respect your privacy" but "we could not violate it." When
  the data never reaches a central party, there is no warehouse, no logs, and no entity to compel —
  the property is enforced by *physics, not policy*. A competitor's own infrastructure (a data
  warehouse, an ads SDK someone added years ago) is exactly what betrays the same promise from them.

- **Trust compounds instead of eroding.** With no monetization phase waiting downstream, there is no
  enshittification arc — no scheduled turn where the product gets worse for the user to get better for
  the business. (*Enshittification* is Cory Doctorow's coinage [CITE]; the platform-lifecycle argument
  it names is grounded in `../fenced/aggregation-theory-and-the-enshittification-shield.md`, cited to
  that layer's source-of-record and not re-argued here.)

The single most quotable articulation of the whole position, from the project's own design thinking:

> The experiences you can't match aren't features — they're promises with no expiry date.

`[dialogue-sourced 2026-06-22; AI-articulated wording reflecting the project's own design reasoning,
treated as primary; not brand-cleared for external quotation — confirm-vs-primary before outward use.]`

And on privacy specifically:

> We couldn't violate it; the data never reaches us and there is no us to subpoena.

`[dialogue-sourced 2026-06-22; AI-articulated wording, reasoning treated as primary; paraphrase, do not
quote-and-attribute externally until brand-cleared.]`

The honest counter travels with the claim, per the anti-rollup rule, because a moat whose limits are
dropped is a claim that will break on contact: server-backed apps genuinely win wherever a neutral
third party is required — stranger matchmaking, server-witnessed anti-cheat, async play against an
offline peer, scale spectating, a trusted dealer for hidden-information games. The moat is therefore
specific to the friend-to-friend, present-together, nothing-at-stake-but-fun zone, and the discipline
is to stay in that zone rather than sprawl into the ones a server wins. `[boundary, dialogue-sourced;
the reasoning is the project's own, the scope limit is load-bearing and must travel with the moat
claim.]`

Why load-bearing: the strategy premises the entire crossing on "a non-extractive sustaining org," and
treats it as a standing investment. This section is the affirmative reason that investment is not just
an ethical stance but a competitive one — the argument that turns "no one makes a buck off this, ever"
from a funding vulnerability into a promise a competitor cannot answer.

## What this establishes (and does not)

Establishes the concrete adoption tactics that `adoption-strategy.md` flags as its missing piece at
T11: the specific antidotes that get a mainstream user across the UX and cultural-inertia wall (a soft
guest pass, protocol abstraction, curated starter packs as graph injection, live-action Frames, and
DNS-handle identity); the event-app Trojan horse as a concrete candidate for the abstract wedge
bridge; the digital-electric-cooperative analogy as the plain-language device for explaining the
co-op to a non-technical audience; and the moat-from-not-having-things as the affirmative *why* behind
the non-extractive premise, with its honest scope limit attached. Each tactic is mapped back to the
abstract strategy move it makes real.

Does **not** restate the strategy frame or the three bridges (they live in `adoption-strategy.md`),
does **not** duplicate the feature enumeration of the non-mimicry moat (that is the governance doc),
does **not** re-argue the grounded philosophy or re-describe the product, and does **not** validate any
of these tactics as chosen or sequenced — the wedge is a candidate, the antidotes are directions, and
the dialogue-sourced statistics and named studies carry verification flags that must clear against
primary sources before any outward use. This remains open thread T11: the direction the tactical
argument is trending, to be firmed up as the case accretes evidence.
