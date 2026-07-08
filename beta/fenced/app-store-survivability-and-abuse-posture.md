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
  posture that follows (control/media plane decoupling, discovery off by default, hardcoded abuse controls,
  edge moderation).
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
pretext**: a fully cross-platform "watch together" threatened the platform owner's closed-ecosystem
equivalent, and vague content-moderation justifications supplied cover. `[confirm]` The two readings are
not mutually exclusive, and the survivability lesson holds under either: the architecture that made Rave
good — media moving directly, encrypted, without a central choke point — is the same architecture that left
it with no place to *demonstrate* moderation to a reviewer.

That is the trap in one line: an app can be removed **because its transport is too good at routing around
the control point the gatekeeper expects to exist.** Decentralization did not protect it; the absence of a
visible governance surface exposed it.

## The design consequence: govern the plane the transport cannot

The escape is to keep the media plane decentralized while giving the app a governance surface a reviewer
can see and a survivor can operate. Four moves carry it:

- **Decouple the control plane from the media plane.** Room discovery, authentication, metadata, link
  filtering, and reports run through a control path the operator can hold; the raw audio and video ride the
  direct, end-to-end-encrypted media path. The gatekeeper's compliance questions are answered on the plane
  that can answer them, without decrypting the plane that must stay blind.
- **Eliminate public discovery by default.** Treat the app as a private utility, not a public directory:
  connections require an explicit, out-of-band invite — a capability token, ticket, or scanned code —
  rather than a browsable "public rooms" surface. No default-public discovery means far less unmoderated-
  space liability and fewer reviewer flags.
- **Hardcode block, report, and kill-switch.** The three user-generated-content controls are not optional
  features: a **block** that terminates the stream and locally blackholes the peer; a **report** path that
  can surface evidence to an operator even under end-to-end encryption (a client-side capture on report,
  not a server-side tap); and a **kill-switch** by which sessions or registrations can be revoked. These
  are the minimum a reviewer expects a UGC surface to carry.
- **Edge-AI moderation.** Push detection to the client, where the plaintext already is, so explicit or
  prohibited content can be caught and the local broadcast cut without a decrypting helper in the middle —
  moderation that is compatible with an end-to-end-encrypted media plane rather than in tension with it.

The single line that anchors the whole lesson, from the design dialogue that surfaced it:

> App stores don't care if your transport is decentralized; they care if your governance is.

— AI-surfaced design dialogue on decentralized-media app-store survivability. `[AI-surfaced; confirm the
Rave specifics against primary sources before external use.]`

**Why this is load-bearing (the anti-rollup rule).** The four moves above are only defensible if the reason
they exist stays attached to them. The reason is the Rave case: an app removed not for weak transport but
for an absent governance surface. Strip the *why*, and "decouple the planes" or "no public discovery by
default" reads as arbitrary friction that a later contributor would be tempted to relax for convenience —
re-enabling default discovery, or letting the control path wither — walking straight back into the trap.
The removal case is what makes the posture non-negotiable, so it travels with the posture.

## What this establishes (and does not)

Establishes that the app-store gatekeepers are part of the fenced field Croft must survive; that the *Rave
trap* is a concrete case in which a decentralized-media app was removed for a **governance** deficit rather
than a transport one; and that the survivability variable at the gate is governance posture — decoupled
planes, discovery off by default, hardcoded block/report/kill-switch, and edge moderation — not the degree
of decentralization.

Does **not** restate the real-time media *transport* mechanics, which live in the spec at `drystone-spec/`
§6.12 (datagram carriage, per-sender keys from the Group epoch, the blind forwarding helper, and the
congestion rules); this document carries the governance and abuse posture that §6.12 does not. It is a new
data point that **complements** the other fenced surveys — the capability map (`group-scale-versus-e2ee.md`),
the operational rates and economics (`operational-rates-and-platform-economics.md`), the dominance and
adoption dynamics (`platform-dominance-and-adoption.md`), and the group-chat failure modes
(`group-chat-failure-modes.md`) — by adding the gatekeeper dimension none of them covers. It does **not**
certify the Rave / Apple-removal specifics: those are AI-surfaced and marked `[confirm]`, and need a
primary-source pass before external use.
