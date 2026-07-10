# The recurring inversion

`Status: philosophy layer (Layer 2, why it is right). Register: argument / framing. Resolution: the
philosophy-layer statement of the *recurring inversion* — the single civic pattern that ties five
separate design moves into one recognizable move, and the reasoning for why they are one pattern rather
than five coincidences. The mechanisms each inversion produces are specified elsewhere (the spec, impl,
croft) and are cross-referenced here, not restated. The naming source for the protocol-layer posture the
pattern operationalizes (Masnick, "Protocols, Not Platforms," 2019) is attributed framing, not a
verified verbatim quotation.`

## Overview

Across the whole design the same move is made at five different scales, and naming the move is what lets
a reader see that they are one argument. The move is an inversion of the extractive intermediary. Take a
component that in the incumbent model is an *extractive stateful intermediary* — a party that sits between
peers, holds state about them, and monetizes the position — and do two things to it in sequence: first
*reduce* it to something stateless, content-blind, or optional, stripping away the standing capacity that
made extraction possible; then *wrap* what remains in an institution that reinforces the peer relationship
rather than extracting from it.

The reduction and the wrap are two halves of one idea. The reduction is the technical half: an
intermediary that holds no durable state, cannot read what it carries, or can be routed around entirely
has no *capacity* to dominate the peers it serves, because domination is a standing capacity, not a
conduct (the argument this layer makes in `peer-standing-and-the-cooperative-form.md` §3). The wrap is the
institutional half: even a blinded, optional intermediary still has to be *run by someone*, and who runs
it, on what terms, decides whether the position drifts back toward extraction — so the residual role is
placed inside a form (a cooperative, a members' utility) whose interest is aligned with the peers rather
than against them. Strip the capacity, then institutionalize the non-extraction. This is the same
two-step the cooperative argument makes about the corporate form as a whole (dissolve the asymmetric edge,
then bind the alignment in a charter, `peer-standing-and-the-cooperative-form.md` §6); the recurring
inversion is that argument recognized as a *general* move and applied component by component, not only to
the top-level firm.

## Charter

- **In scope:** the statement of the inversion as a repeatable template (extractive stateful intermediary
  → stateless / content-blind / optional → wrapped in a reinforcing institution); the five named scales at
  which the project applies it; and the reasoning for why they are one pattern (each removes a standing
  capacity to dominate and then re-homes the residual role in an aligned form).
- **Out of scope (and where it lives):** the *mechanisms* each inversion produces. The blind relay and the
  content-blind Delivery Fabric are specified in `../drystone-spec/part-2-certifiable-design.md` (§6, §8);
  the optional superpeer and its centralization-risk register live in `../impl/the-four-property-tension.md`;
  the consumer-side / demand-side ad broker is named but not yet designed and is tracked as an open thread
  (croft layer); the cooperative form is argued in full in `peer-standing-and-the-cooperative-form.md`.
  This document names each rung and points at its home; it does not re-specify any of them.
- **Boundary call:** this is the "why these are one argument" register — the through-line a newcomer needs
  in order to hear five design decisions as a single civic pattern. The individual decisions carry their
  own reasoning at their own homes.

## The template

The inversion is a three-position template, and the middle position does the load-bearing work.

1. **Start:** an *extractive stateful intermediary* — a party positioned between peers that (a) holds
   durable state about them, (b) reads or shapes what passes through, and (c) monetizes the position. This
   is the incumbent shape of the relay, the router, the ad platform, and the operator.
2. **Reduce:** make the intermediary *stateless*, *content-blind*, or *optional* — ideally more than one.
   Each reduction removes a different axis of the standing capacity to dominate: statelessness removes the
   held leverage, blindness removes the read/shape leverage, optionality removes the chokepoint leverage
   (a peer who can route around you cannot be held by you). A component reduced on all three axes retains a
   *resource* it offers but no *standing* it can exert — the resource/standing distinction the spec draws
   at `../drystone-spec/part-2-certifiable-design.md` §5.
3. **Wrap:** place the residual role inside an institution that *reinforces* rather than extracts. The
   reduction makes extraction *technically* hard; the wrap makes non-extraction *durable*, because the
   party still operating the reduced role sits inside a form whose surplus returns to the peers it serves
   rather than to an outside constituency. Without the wrap, a blinded optional relay is still run by
   someone who can re-accrue advantage through sheer circumstance (it holds the state, it is always
   present, few others can run it); the institution is what keeps that circumstance from hardening into
   position.

## The five scales

The project applies the template at five named scales. Each cell below is the same move; the "home" column
is where the mechanism (not the framing) is specified.

| Extractive intermediary | Reduced to | Reinforcing wrap | Mechanism home |
|---|---|---|---|
| Relay (holds sessions, sees traffic) | stateless rendezvous (holds nothing; a meeting point, not a middleman) | run as shared reachability infrastructure, not a product | blind relay, `../drystone-spec/part-2-certifiable-design.md` §6 |
| Relay (permanent required hop) | optional superpeer ("lovely to have, not a have-to") | the inequality is *chosen and tracked*, registered as a standing centralization risk, never smuggled | `../impl/the-four-property-tension.md` |
| Routing server (reads and orders content) | content-blind carrier (the blind Delivery Fabric; carries ciphertext and routing metadata at most) | a shared carrying commons no one Group owns | blind fabric, `../drystone-spec/part-2-certifiable-design.md` §6, §8 |
| Ad platform (broker serving advertisers) | consumer-side / demand-side broker (the member, not the advertiser, is who it serves) | revenue flows to members; the co-op takes only a small transparent fee | *named, not yet designed* — tracked as an open thread (croft) |
| Compellable operator (a party that can be leaned on) | the operator's leverage dissolved into a member-owned form | a cooperative from inception, so there is no extractive edge to compel | `peer-standing-and-the-cooperative-form.md` §6 |

The five are at different layers of the stack — transport, availability, dissemination, economics,
governance — which is the point: the pattern is not a property of any one layer, it is the design's
recurring answer to "there is an intermediary here; how do we keep it from becoming a master?"

## Why it is one pattern, not five coincidences

The unifying claim is that each rung removes the *same* thing — a standing capacity to interfere with
impunity — and then re-homes the residual role in an *aligned* form. That is precisely the shape of the
non-domination argument this layer makes about the corporation
(`peer-standing-and-the-cooperative-form.md` §§3, 6): the wrong is the standing capacity, not its exercise,
so the remedy is to strip the capacity rather than to promise good behavior, and then to bind the
alignment institutionally rather than leaving it to discretion. Applied to the firm, that argument
produces the cooperative. Applied to a relay, a router, or an ad platform, the *same* argument produces a
stateless rendezvous, a blind fabric, a consumer-side broker. Recognizing them as one move is what turns a
list of engineering choices into a civic posture that can be stated, defended, and checked for
consistency: if a proposed component adds an intermediary that is not reduced-and-wrapped, it is a
departure from the pattern and has to be justified as one.

The pattern also inherits the same discipline the cooperative argument carries: the reduction is *necessary
but not sufficient*. A blinded, optional intermediary that is run extractively will drift back toward the
master position; a wrapped-but-unreduced intermediary keeps the technical capacity to dominate even inside
an aligned form. Both halves are load-bearing, which is why the template is a three-position move and not a
two-position one.

## The naming source: "Protocols, Not Platforms"

The posture the recurring inversion operationalizes has a name in the literature. Mike Masnick's 2019
essay *Protocols, Not Platforms: A Technological Approach to Free Speech* argued for moving the locus of
value from a controlled platform to an open protocol layer — a layer where multiple entities compete,
where identity and data exist independently of any one application, and where *credible exit* is
guaranteed rather than promised (Masnick's position, stated as attributed synthesis; see the reference
index). Each rung of the recurring inversion is a protocols-not-platforms move made concrete: it takes a
place where the incumbent shape is a platform-that-holds-you and re-expresses it as a protocol-you-can-
leave. The essay names the *direction*; the recurring inversion is the project's account of how to walk it
component by component, and the cooperative wrap is the project's answer to the question the essay leaves
open — who keeps the protocol layer honest once it exists.

## Where the argument connects

- **`peer-standing-and-the-cooperative-form.md`** is the top-level instance of the same move (the firm
  itself as the fifth rung) and the source of the non-domination reasoning the pattern generalizes.
- **`commensurability-and-the-two-ledgers.md`** grounds the *blindness* half of the reduction: an
  intermediary that does not read content is one that does not compute meaning it has no standing to
  compute (the moderation-by-standing-not-by-reading posture, §VIII there).
- **The spec (Layer 4)** and **impl (Layer 5)** carry the mechanisms; **croft (Layer 6)** carries the one
  rung — the consumer-side broker — that is named here but not yet designed.
