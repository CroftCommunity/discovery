# App-store survivability and the abuse posture: the Rave trap

`Status: fenced layer. Register: lesson / gatekeeper posture — a data point on the fenced field, not a
capability measurement. Resolution: the survivability lesson (what a decentralized-media app must carry in
its governance to remain shippable through the app stores). External facts carry verification flags; the
Rave / Apple-removal specifics are AI-surfaced and marked [confirm] pending a primary-source pass.`

## Overview

The fenced field is not only the centered messaging and community platforms whose limits the rest of this
layer measures. It also includes the **gatekeepers** — the Apple App Store and Google Play — through which
any consumer-facing Croft client must pass to reach a phone. A decentralized transport does not exempt an
app from that gate; it can make the gate *harder* to clear. The load-bearing lesson here is that the
survivability variable at the store is **governance posture, not decentralization**: the reviewer is not
asking whether your bytes move peer-to-peer, but whether you can demonstrate control over abuse, discovery,
and takedown. This document records that lesson — the *Rave trap* — as a fenced data point, so the design
consequence travels with the case that produced it.

## Charter: what this document covers

- **In scope:** the app-store gatekeepers as part of the fenced field; the *Rave trap* as a worked case of
  a decentralized-media app removed for a governance deficit rather than a transport one; and the design
  posture that follows — Signal's *deny-the-surface* path over Telegram's *readable-moderation* path:
  discovery off by default, scale caps on broadcast, a blind admission-and-scale choke, membership
  accountability, block and report handled blind, an opt-in content-visible moderation role for communities
  that need reach, and the CSAM-corner and fork-blast-radius limits stated rather than waved away.
- **Out of scope (and where it lives):** the *transport* mechanics of real-time media — the datagram
  carriage, per-sender keys, blind forwarding helper, and congestion rules — which live in the spec at
  `drystone-spec/` §6.12 and are **not** restated here. This document carries the governance and abuse
  posture that §6.12 deliberately does not.
- **Boundary call:** this is the "will it survive the gate, and why" register. It is descriptive of the
  fenced field (how the gatekeepers behave) and prescriptive only about the *shape* of the answer; the
  normative wire and transport requirements stay in the spec.

## The Rave trap

*Rave* was a cross-platform co-viewing app — iPhone, Android, Mac, and Windows users watching synced video
together in the same room — built on a high-performance decentralized peer-to-peer media path. It worked,
and it worked across the ecosystem boundary that the platform owner's own native feature could not cross.

Apple removed Rave from the App Store in **August 2025**. `[confirm]` Two readings of the removal sit side
by side. In the first, it was a **content-moderation failure**: an app whose direct encrypted media
pipelines routed around any central point where content could be inspected became, in effect, an
unmoderated public space — public rooms, unfiltered chat, pirated and adult content — which is exactly what
the store's user-generated-content rules exist to refuse. In the second, it was an **anti-competitive
pretext**: a fully cross-platform "watch together" threatened the platform owner's own closed-ecosystem
co-viewing feature — which works only inside the walled garden — and vague content-moderation
justifications supplied cover; the developer disputed the removal and filed antitrust suits in five
countries (May 2026), and disabling the platform's single-sign-on reportedly locked roughly 11.4 million
users out of years-old accounts. `[confirm]` The two readings are not mutually exclusive, and the
survivability lesson holds under either: the architecture that made Rave good — media moving directly,
encrypted, without a central choke point — is the same architecture that left it with no place to
*demonstrate* moderation to a reviewer.

That is the trap in one line: an app can be removed **because its transport is too good at routing around
the control point the gatekeeper expects to exist.** Decentralization did not protect it; the absence of a
visible governance surface exposed it.

## The two ways to avoid becoming an abuse hub — and the only one open to Croft

There are two opposite ways to avoid being the abuse hub, and the choice between them decides everything
downstream. They are cleanest to see in the contrast between two real messengers.

*Telegram's way: be readable, so you can moderate centrally.* Telegram's cloud chats, groups, and channels
are not end-to-end encrypted — the server can read them — so it *can* scan content and comply with takedown
demands. Its public channels, mass-broadcast reach, and large-file uploads made it a piracy and CSAM
distribution surface, and being readable is what let it respond: after its founder's arrest in France
(August 2024), it reversed a long-standing "never hand over data" stance and began sharing user data on
valid legal orders. The price of being *able* to moderate is being able to be *compelled*.

*Signal's way: don't create the surface.* Signal is end-to-end encrypted and blind by design; it has no
public discovery (contact-based only), is not a mass-broadcast medium, and refuses content scanning on
principle. It is not a hub because it never built the surface that makes one. Its moderation is report,
block, account accountability, and the minimal metadata it holds — WhatsApp, on the same encrypted model,
bans more than 300,000 accounts a month suspected of sharing child sexual abuse material, acting on
**unencrypted surfaces** (proactively scanning profile and group photos, group names and descriptions),
**behavioral signals**, and **user reports** — never on encrypted message content. `[verified 2026-07-10
against WhatsApp's own Help Center; note the mechanism is unencrypted-surface scanning + behavioral signals
+ reports, not metadata alone.]` That figure is the load-bearing existence-proof under this whole
posture: abuse-resistance-while-blind-to-message-content is a shipping practice at scale, not an aspiration —
a message-content-blind platform already moderates at hundreds of thousands of actions a
month without reading a single encrypted message.

A blind, non-extractive design can only take Signal's path — the readable-moderation path is not available
to it, because there is no plaintext at the center to inspect. That is not a limitation to apologize for; it
is the same property that makes the system private. And it names the trap precisely: **the worst possible
combination is Telegram's surface with Signal's blindness** — public, mass-broadcast reach that the operator
cannot see into and therefore cannot moderate. That combination is exactly what Rave was, and it is what
gets a platform pulled. The discipline follows in one line: do not build Telegram's surface on a blind
transport.

## The design consequence: deny the surface, govern blind

The survivability question — can you demonstrate control over abuse, discovery, and takedown? — has a
tempting wrong answer for a decentralized-media app. The reflex, and the one the design dialogue that
surfaced this case reached for, is to bolt on a *central* control plane: a server that holds auth, room
metadata, link filtering, and reports; runs content-moderation APIs over messages and URLs; owns a
kill-switch over identity; and pushes an edge-AI classifier that samples the camera and reports matches to a
backend. That answer recreates the exact extractive choke point a blind protocol exists to remove — a party
that inspects content, identity, and links, and that can therefore be subpoenaed, breached, or repurposed.
It is closed to Croft on principle, and the last clause of the dialogue's own golden rule — *keep a
centralized hand on identity and compliance* — is precisely the part Croft refuses.

The thesis-consistent answer attacks the *distribution topology*, which is visible in metadata, not the
*content*, which is not. Six levers carry it:

- **No public discovery, by default — and this is already the model, not a new feature.** Connections
  require an explicit, out-of-band invite: a capability token, ticket, or scanned code, admitted by a member
  with standing. There is no browsable public-rooms directory to become a piracy or CSAM index. This is the
  single biggest lever, and it *aligns* with the thesis (scoped visibility, quiet membership) rather than
  fighting it — a system with no anonymous-stranger discovery surface is structurally not a hub.
- **Scale caps, because broadcast-at-scale is the dangerous surface — not media as such.** A one-to-one call
  is low-impact, like a direct message; the hub-forming surface is one-to-many broadcast and large-file
  distribution at scale — the Telegram-channel shape. So the axis to cap is conversational versus
  broadcast-at-scale, expressed as tier policy: bounded fan-out, rate, and audience size.
- **A blind admission-and-scale choke, never a content choke.** A relay or broker sees only the metadata it
  already sees — membership count, fan-out, namespace, rate — not content. So a co-operatively run relay
  *can* enforce scale, rate, and admission policy blindly (refuse to be a mass-distribution CDN, cap
  audience, throttle) and *cannot* enforce content policy. That boundary is honest and it is the enforcement
  surface: abuse-at-scale needs a relay, and co-op relays will not serve abusive scale.
- **Membership accountability through standing, not anonymity.** A participant is present because a member
  with standing admitted them; within a group, authorship is attributable (signed messages) and removable
  (revocation). This is accountability without central identity surveillance — the group governs itself.
- **Block is revocation; report is client-attested and blind.** A block terminates the media stream and
  locally blackholes the peer. A report carries only what the reporter's own client can attest from its own
  decrypted view — a client-side capture on report, never a server-side content tap, because the relay has
  no plaintext to snapshot. Moderation lives at the client-and-membership layer, as it must under end-to-end
  encryption.
- **No edge-AI content-scanning-then-reporting.** A client-side classifier that samples the plaintext and
  reports matches to a backend is surveillance with a center, and "who defines explicit?" makes the
  classifier a censor. It is rejected for the same reason the central control plane is. Proactive detection,
  where a community genuinely needs it, is available only through the opt-in role below, and even there only
  at its narrowest rung.

The single line that anchors the whole lesson, from the design dialogue that surfaced it:

> App stores don't care if your transport is decentralized; they care if your governance is.

— AI-surfaced design dialogue on decentralized-media app-store survivability. `[AI-surfaced; confirm the
Rave specifics against primary sources before external use.]`

The governance a reviewer wants is real; Croft simply supplies it *blind* — deny the surface, cap the
scale, attribute the membership — rather than by installing a hand on the content.

## The opt-in escape for communities that need reach

Some communities *need* demonstrable proactive moderation — to clear a gatekeeper, to operate at semi-public
scale, or because a moderated space is the feature people join for. For them, blind-by-default plus reactive
report-and-revoke can be supplemented by a **content-visible moderation role a group consensually admits**:
opt-in by the group's own governance, disclosed as a named role to every member, scoped to the
least-invasive rung that serves the need, and revocable. It is the protocol expression of an elected club
officer, not a state wiretap, and it is the answer that is neither "all-blind, no-moderation" (which cannot
reach some app stores) nor "all-readable" (which is Telegram).

The cost must be stated, because it is the heart of the tradeoff: **any content-visible capability erodes
the "we cannot comply" shield.** A blind system is protected by impossibility — there is nothing to hand
over, so it cannot be ordered to. The moment a content-visible role *can* exist, capability invites
compulsion — *you can, therefore you must* — and a regulator could move to mandate it. The mitigation is to
keep the system default firmly blind and the role strictly per-group, opt-in, disclosed, and revocable, so
the honest claim remains "the *system* cannot; only groups that chose it can." This is a policy and legal
question, not one that engineering resolves, and it gates any real deployment. The normative rule for this
role — its label-not-enforce posture and its compellability constraint — lives in the spec at
`drystone-spec/`; this document records only that it is the survivability escape and what it costs.

## The CSAM corner — named, not waved away

CSAM is the category where "we are blind, so we cannot scan" is hardest to stand behind, for any platform.
Croft's honest position is that resistance comes from the absence of an anonymous public-discovery surface,
from membership-and-standing accountability, from scale caps that prevent mass broadcast, and from client
reporting plus revocation — not from content scanning, which is impossible while blind. This is the same
posture as Signal and Matrix, and it is defensible, but it is a *stated limitation*, not a solved problem,
and it belongs in the threat model and the public posture rather than hidden.

## The fork reality — bound the blast radius, do not pretend to prevent

Croft is open, so a fork *can* strip the limits and add public discovery and unbounded broadcast. That
cannot be prevented and should not be pretended away; what can be done is bound the blast radius. The
mainline app ships none of the abuse affordance (no public directory, scale-capped broadcast, metadata-only
admission). Co-operative, non-extractive infrastructure does not subsidize abuse scale: mass distribution
needs relays, and the co-op's relays enforce scale and admission policy, so a fork wanting piracy-CDN scale
must run its own relays — its own infrastructure, its own legal liability. And physics caps the worst
pure-peer-to-peer case: a mesh with no relay cannot reach mass-distribution scale, because the same
no-central-forwarder topology that makes Croft private makes a no-infrastructure fork unable to become a
CDN. The worst case is therefore bounded: a fork must take on both its own infrastructure *and* its own
liability to do at scale what the mainline structurally prevents.

**Why this is load-bearing (the anti-rollup rule).** These levers are only defensible if the reason they
exist stays attached to them. The reason is the Rave case: an app removed not for weak transport but for an
absent governance surface it could not supply blind. Strip the *why*, and "no public discovery by default"
or "cap the broadcast tier" reads as arbitrary friction a later contributor would relax for convenience —
re-enabling default discovery, lifting the caps, or quietly reaching for the central content plane — walking
straight back into the trap. The removal case is what makes the posture non-negotiable, so it travels with
the posture.

## What this establishes (and does not)

Establishes that the app-store gatekeepers are part of the fenced field Croft must survive; that the *Rave
trap* is a concrete case in which a decentralized-media app was removed for a **governance** deficit rather
than a transport one; that there are two opposite ways to avoid becoming an abuse hub — Telegram's
*be-readable-and-moderate* and Signal's *deny-the-surface* — and a blind, non-extractive design can take
only Signal's; and that the survivability variable at the gate is therefore governance posture supplied
*blind*: no public discovery, broadcast scale caps, a metadata-only admission-and-scale choke, membership
accountability, block-as-revocation and client-attested reporting — with an opt-in, disclosed, revocable
content-visible moderation role as the escape for communities that need reach, at the cost of eroding the
"we cannot comply" shield. It also establishes what is *not* solved: the CSAM corner is a stated limitation,
not a solved problem, and a fork can strip the limits — bounded only because co-op infrastructure will not
subsidize abuse scale and physics caps the pure-peer-to-peer case.

Does **not** restate the real-time media *transport* mechanics, which live in the spec at `drystone-spec/`
§6.12 (datagram carriage, per-sender keys from the Group epoch, the blind forwarding helper, and the
congestion rules), nor the normative moderation-role and compellability rule, which the spec owns; this
document carries the governance and abuse posture that the spec does not. It is a data point that
**complements** the other fenced surveys — the capability map (`group-scale-versus-e2ee.md`), the
operational rates and economics (`operational-rates-and-platform-economics.md`), the dominance and adoption
dynamics (`platform-dominance-and-adoption.md`), and the group-chat failure modes
(`group-chat-failure-modes.md`) — by adding the gatekeeper dimension none of them covers. It does **not**
certify the Rave / Apple-removal specifics: those are AI-surfaced and marked `[confirm]`, and need a
primary-source pass before external use.
