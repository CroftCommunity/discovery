# ActivityPub vs ATProto, and the de-facto-standard lesson

`Cairn stone, 2026-07. Orientation: **related-ecosystem** (the peer federation protocols Croft
bridges to and measures against — not a Drystone building block), carrying one **Drystone-oriented
governance lesson** (the underspecified-protocol capture risk). Distilled from a 2026-07 dialogue;
primary anchors named inline (The Fediverse Report; berjon.com/ap-at; the 2019 W3C–WHATWG MOU;
FEP-2c59). Deployment/adoption facts are point-in-time (2026) and marked as such. Complements
`atmospheric-web-and-aggregators.md` (the no-per-activity-fee finding, the fused-timeline
anti-pattern, the aggregator license map) — that angle is not re-derived here.`

## The layer separation (the category error to avoid)

Three different kinds of thing:

- **Mastodon** is a *product* — one server implementation (Rails) that speaks ActivityPub.
- **ActivityPub (AP)** is a *protocol* — a W3C Recommendation since 2018 defining how servers
  federate.
- **ATProto** is a *rival protocol stack*, with Bluesky as its flagship product.

So "Mastodon vs ATProto" is a category error in the same shape as "Gmail vs IMAP." The real
comparison is **ActivityPub vs ATProto**, with Mastodon as ActivityPub's dominant deployment.

## The architectural difference (email vs the web)

The Fediverse Report framing: **ActivityPub resembles email** — independent servers sending messages
to each other; **ATProto resembles the web** — independent sites publishing data that indexers
aggregate into views. In AP, identity + data + app experience are **fused to your instance**. In
ATProto they are **deliberately split**: identity (DID) is portable, data lives in your PDS as signed
repos, and AppViews/relays index the network. (pdsview and skylite exist precisely because the
AppView is a separable layer.)

Consequences:
- **Identity & exit.** AP identity is server-bound (`@you@instance`); moving instances is a
  bolted-on, followers-only migration. ATProto has key rotation + account recovery from the start —
  though `did:plc` currently routes through Bluesky's PLC directory, a *deployment* centralization the
  protocol doesn't require (the ecosystem has it anyway; cf. F-AT-6 correlator posture).
- **Schemas.** ATProto's **Lexicons** enforce cross-implementation consistency; AP extends via ad-hoc
  JSON-LD, which is *why* Mastodon's dialect effectively is the spec for much of the fediverse.
- **Global vs local view.** ATProto's relay/AppView design gives a global firehose + swappable feed
  algorithms; AP servers see only what federates to them, so search/discovery are inherently partial.

**The honest trade-off (point-in-time 2026).** Most ATProto users rely on Bluesky's own relays and
AppViews — real centralization risk despite the portable design — and self-hosting the full stack is
much harder than running a Mastodon instance. AP has the broader multi-platform adoption; ATProto is
still primarily a Bluesky ecosystem. It cuts both ways: **AP's decentralization is real but its data
is siloed per-instance; ATProto's data is open but its indexing layer is oligopolistic for now.** A
deployment fact, not a protocol guarantee.

## Mastodon as the de-facto ActivityPub standard

Most of Mastodon's divergence is *filling holes the W3C spec left open* rather than contradicting it —
but since everyone must fill the same holes the same way to interoperate, **"Mastodon-flavored
ActivityPub" is the de-facto wire protocol.** Three categories:

1. **Gaps the spec never covered, where Mastodon's answer became mandatory.** AP says nothing about
   HTTP request signing; the network runs Mastodon's draft-cavage HTTP Signatures. **WebFinger**
   (RFC 7033) is not part of AP, but Mastodon depends on `acct:` URIs, so any implementation without
   WebFinger fails to interoperate. Mastodon-compatibility = AP + WebFinger + unspecified signature
   crypto, documentation fragmented across all three.
2. **Spec features Mastodon ignored.** AP defines both C2S and S2S; Mastodon implemented only S2S and
   shipped its own REST API for clients. Result: nearly every client and many non-Mastodon servers
   (Pleroma, GoToSocial) implement the **Mastodon API**, so AP's C2S is effectively dead code and the
   Mastodon REST API is a *second* de-facto standard beside the federation protocol.
3. **Mastodon-originated extensions others must honor.** Secure mode / authorized fetch (signed GETs),
   destination-side followers-only addressing, custom vocabulary for pinned posts / profile fields /
   suspensions — all living in Mastodon's own docs, which function as the real interop reference.

The community response is the **FEP process** (Fediverse Enhancement Proposals), which retroactively
formalizes these de-facto behaviors; Mastodon's docs now cite FEPs directly (e.g. FEP-2c59 for
WebFinger handles, as of 4.6.0). Net: **the W3C Recommendation is a skeleton, Mastodon is the reference
deployment, and the deployment's choices are normative in practice.**

## The Drystone-oriented lesson: the HTML living-standard cautionary tale

It is the HTML story again — *living-standard-by-dominant-implementation, with the standards body
trailing*:

- Through the late 90s HTML was a versioned W3C Recommendation. In 1998 the W3C pivoted to XHTML.
- In 2004 Apple/Mozilla/Opera formed the **WHATWG** and kept developing HTML as browsers actually
  implemented it — the "HTML Living Standard," no version numbers, defined by what dominant
  implementations do (including specifying real browsers' error-handling).
- In **May 2019** a W3C–WHATWG MOU made HTML/DOM develop principally in the WHATWG; the W3C stopped
  independently publishing them. The Recommendation stamp became a snapshot of a document the
  implementers write.

Mapping: **AP Recommendation (2018, frozen) ≈ HTML4/XHTML** (official, increasingly not operative);
**Mastodon ≈ the dominant browsers** (its behavior defines "compatible"); **FEP process ≈ WHATWG's
living standard**. The seam: HTML's capture was by a *cartel of several* vendors that negotiate;
Mastodon is closer to a *single* dominant vendor, and FEP has far less institutional weight, with no
MOU moment where the SocialCG ceded AP. The fediverse sits at roughly the 2010-era stage — parallel
de-facto and de-jure specs, resolution still open.

**Why this is on the board for Drystone.** It is the concrete cautionary case of *what happens to an
underspecified protocol when one implementation reaches critical mass first*: the reference deployment
becomes normative, and the spec trails its own ecosystem. The steer for Drystone is spec completeness
and conformance rigor **before** a dominant implementation sets the de-facto wire behavior — the
opposite failure mode from AP's. (Croft's own AP bridge — the ap-ambassador, `../../alpha/experiments/
ap-ambassador/` — deliberately "respects the customs of the protocol federated with," which is the
delivery-plane way of honoring Mastodon-flavored AP without importing its governance.)

## Berjon's ap-at thesis (the layers may not compete)

Robin Berjon (`berjon.com/ap-at`) argues **ActivityPub could plausibly run atop an ATProto PDS**,
since ATProto is a generic data layer and the AP Actor document gives a clean indirection point —
i.e. these are not necessarily competitors *at the same layer*. Relevant framing for where Drystone
sits relative to both. A full walkthrough (with attention to indie AP sites using custom domains) is
deferred — ROADMAP_TODO E41. Berjon is tracked in `../socialization/kindred-work.md` (berjon-robin).

## Pointers

- Aggregator/fee/license angle + the fused-timeline anti-pattern: `atmospheric-web-and-aggregators.md`
- ATProto ecosystem survey: `atproto-ecosystem.md`; register rows: `reference-index.md` (ActivityPub)
- Croft's AP bridge (delivery-plane, governance-excluded): `../../alpha/experiments/ap-ambassador/`
- The kindred author: `../socialization/kindred-work.md` → berjon-robin
