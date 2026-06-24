# Membership vs. access — the public door a stakeholder can hold open

date: 2026-06-22

source: distilled from `seeds/transcripts/raw/croft-discord-money-ipo-onboarding-dialogue-2026-06-22.md`
(the user's own design thinking, developed against Discord's onboarding model; treat the *reasoning* as
primary). A small but load-bearing distinction that resolves an apparent tension between "member-owned /
sponsored entitlements" and "frictionless, even anonymous, joining."

## Problem

Discord conflates two things into one "server membership," and the conflation is the source of the
friction the user hit (approval gates) and of the extraction analysis (volunteer moderators create value
they hold no stake in). Croft's member-owned model risks inheriting the *opposite* failure: if "you must
be a member" guards every room, the **ten-second door** — the single property that puts even iroh's own
developers on Discord (`research/discord-dominance.md`) — is lost, and with it the only real acquisition
path.

## Approach — two orthogonal axes, deliberately decoupled

Discord (and most platforms) treat these as one gate. Croft separates them:

```
                     WHO CAN WALK IN  (access — the room layer)
                     ┌──────────────────────────┬──────────────────────────┐
                     │  open / public door      │  gated door               │
  WHO HOLDS A STAKE  ├──────────────────────────┼──────────────────────────┤
  (membership —      │  ANONYMOUS GUEST          │  invited/screened guest   │
   the infra +       │  in a pad: posts, plays,  │  (admin-chosen friction   │
   governance layer) │  no governance weight     │   for spam resistance)    │
                     ├──────────────────────────┼──────────────────────────┤
  member / sponsee   │  member opens a public    │  member-only pond/pad     │
  (stake + a vote)   │  pad onto a member pond   │  (the cozy union)         │
                     └──────────────────────────┴──────────────────────────┘
```

- **Membership** gates the **infrastructure layer**: who *owns and governs* the pond (the
  cap-table-equivalent + a vote). Members and their **sponsees** (member-included entitlements — e.g. a
  family-of-six) are the stakeholders. This is where the cooperative ownership and governance live.
- **Access** gates the **room layer**: who can enter/post in a given **pad**. A stakeholder can open a
  **public, even anonymous, door** into a specific pad (a support channel, a public event) while the
  **pond stays member-governed.** The anonymous visitor is a **guest in a room, not a member of the
  co-op** — access without a stake.

This is the inverse of the Discord problem (where contributors get a stake's worth of labor *extracted*
but no ownership): here a guest gets *access* without owing labor or holding a stake, and a contributor
who wants in becomes a *member-owner*, not unpaid enterprise value (see the moderator-extraction analysis
in `research/discord-dominance.md` and the cooperative counter-model in
`thinking/cooperative-social-union-model.md`).

## Reasoning — what the decoupling buys

- **It keeps the ten-second door without diluting ownership.** The public/anonymous door is a property
  of a *pad*, not of *membership*, so a co-op can be member-owned *and* hold open a frictionless public
  room. This is the design space behind the tier-zero **deep-link resolver / "ten-second door"** work —
  the property to protect is an anonymous guest's **first entry into an iroh/p2p pad being as fast as a
  Discord invite link**, which is hard precisely because there's no central server minting an instant
  session (key-exchange + peer-discovery happen underneath). Tracked: ROADMAP_TODO **E11** (the
  deferred-deep-link reality: cold-install deep-linking isn't privately achievable → a claim-code
  one-more-tap, framed as a feature). This note is the *membership-model* half of why E11 is the whole
  game.
- **Sybil resistance softens at the seam that matters.** Anonymous guests in a public pad carry **no
  governance weight**, so spam in a guest room **cannot capture the co-op** the way spam votes could in a
  flat-membership system. Frictionless onboarding and Sybil resistance are in direct tension *only when
  the door also confers a stake*; decoupling them lets the public door be cheap because it grants nothing
  to capture. This is the access-layer expression of D9's "**member ≠ governance-constituent must be
  modeled**" and relates the cheap-fork Sybil defense (D9) and quiet membership (S3).
- **It composes with the existing visibility/moderation dials.** A guest pad is a natural place for the
  **geer** (the opt-in, disclosed, content-visible *gating peer* — `thinking/geer-gating-peer.md`) to do
  the moderation a public anonymous room needs, without touching the member pond's blind-by-default
  guarantee. And it sits beside **S3 quiet membership** (`thinking/social-layer.md`): a guest is present
  in a room without being mapped into the member graph at all — the strongest form of "reachable without
  being mapped."

## Open edges

- **The protocol expression** (the unsolved part): how does an anonymous guest's first entry into a pad
  actually work on iroh — ticket/claim-code → key-exchange → peer-discovery — and can it hit the
  ten-second bar? This is the E11 engineering question; this note only fixes the *membership semantics*
  around it (guest ≠ member; no governance weight; revocable room access).
- **Sponsee mechanics:** how member-included entitlements (the family-of-six) are represented — are
  sponsees full governance-constituents, access-only dependents, or a middle tier? Ties to D9 and the
  cooperative model's multi-class membership (`thinking/cooperative-social-union-model.md`).
- **Guest → member conversion:** the path by which a frequent guest becomes a member-owner (the
  anti-extraction inverse of Discord's mod-with-no-stake), and whether that path is the natural
  acquisition funnel.

## Cross-references

- Competitive grounding + the moderator-extraction analysis: `research/discord-dominance.md` (Update 2026-06-22)
- The ten-second door / deep-link reality: ROADMAP_TODO E11; `thinking/app/ponds/build-order.md`
- Member ≠ governance-constituent; Sybil/cheap-fork: ROADMAP_TODO D9
- Quiet membership (reachable without being mapped): `thinking/social-layer.md` (S3)
- Content-visible moderation for a public room: `thinking/geer-gating-peer.md`
- The ownership counter-model: `thinking/cooperative-social-union-model.md`, `thinking/foundation-and-ip-stewardship.md`
- Ponds (infra/membership) vs pads (rooms/access): `thinking/app/` (design-philosophy, ponds catalog)
- Seam log: `COHESION.md` §36
