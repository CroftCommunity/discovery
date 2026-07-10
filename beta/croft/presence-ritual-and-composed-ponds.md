# Presence, ritual, and composed ponds: the specific ponds and the valuation edge

`Status: croft layer (product). Register: recovered product substance — the specific ponds and edge
model the sibling product docs leave as stubs. Resolution: library — carries the Presence & Ritual
pond, game-outcome-as-lexicon, the iroh tiered-exposure model, and the valuation edge; does not
reproduce the ponds/pads taxonomy (that is product-the-garden-of-ponds.md) or the durable-group
substrate (that is social-graph-as-substrate.md). Dialogue-sourced product calls carry verification
flags; one design problem (outcome attestation) is surfaced open, not resolved.`

## Overview

The two sibling product docs establish the frame. `product-the-garden-of-ponds.md` gives the taxonomy —
a warm shell hosting *ponds* (connections to external ecosystems) and *pads* (self-contained apps), with
honest seams and a proven functional-core spine. `social-graph-as-substrate.md` gives the substrate — the
durable group at the bottom of the pyramid, chat as one tenant, identity that is not the member set. Both
leave specific product substance as stubs: *which* ponds embody the ethic, *how* a game's result becomes a
durable record without polluting anything, *how* the web version actually works, and *how* trust between
separate groups is modeled without leaking into key access.

This doc recovers those four load-bearing conclusions:

- the **Presence & Ritual pond** — the project's heart, and the no-streak decision that carries the whole
  ethic in miniature;
- **game-outcome-as-custom-lexicon** — ephemeral play over the transport, only the settled outcome durable,
  rendering only where a client knows the lexicon;
- the **iroh tiered-exposure product model** — the product-facing answer to "how does the web version work";
- the **composition-vs-valuation** two-edge-types model — carrying the *valuation* edge specifically, since
  composition is already covered by the substrate doc.

## Charter

- **In scope:** the specific ritual/presence activities and the voice decision behind them; the durable-record
  design for peer games and the open attestation problem; the three-tier web-exposure product model and the
  relay's role in it; the valuation trust-edge that sits beside the (already-carried) composition edge.
- **Out of scope (and where it lives):** the ponds/pads taxonomy, the garden thesis, the functional-core
  spine, the inclusion pathways, and the quality bar — all in `product-the-garden-of-ponds.md`. The durable
  group, the implicit/sticky/pruned lifecycle, membership-vs-access, and the group's-face UX — all in
  `social-graph-as-substrate.md`. The composition edge itself (shared MLS lineage) — the substrate doc and
  the Drystone spec; this doc carries only the valuation edge that sits beside it.
- **Boundary call:** this is the "what specifically, and why it must be shaped this way" register. Where the
  sibling docs say "activities ride the garden" or "trust is an edge," this doc names the activities and the
  edge and keeps the reasoning attached.

## The Presence & Ritual pond: the project's heart

The utility ponds are tools a user reaches for. The Presence & Ritual pond is different in kind: its
activities do not accomplish a task, they make a small group *feel* like a group over time. That is why it is
treated as the heart of the product rather than one pond among many — it is where the thesis (presence
offered, never extracted) is most exposed, and most easily betrayed by a wrong default.

**The thinking-of-you ping** is the purest expression of the thesis. The reasoning is a direct cost
comparison: consumer bracelets (Bond Touch and kin) built an account, a business, and a cloud relay around
what is roughly fifty bytes over the wire. Croft delivers the identical felt experience as a near-empty
gossip packet — free, and undiscontinuable, because no server sits between two people to be shut down. The
design constraints follow from keeping it a gesture rather than a message: non-verbal and pressure-free (no
read receipts), a small fixed vocabulary rather than free text (free text turns it back into messaging), and
real-time-first — it lands now or not at all, like a knock, which is both more intimate and simpler than a
queued notification. `[dialogue-sourced 2026-06-20 to 22; verify before external use.]`

**The guestbook is the connective tissue.** It is the one activity that *accumulates*, which is how a group
acquires shared memory at all. It is naturally multimodal (notes, photos as blobs, a drawing, a saved game
outcome), so the present-tense activities deposit durable traces into it — the ping evaporates, but a moment
worth keeping can be laid down here. Its edit model is append-only with redaction, not true delete
(void-don't-mutate): shared memory that could be silently rewritten is not trustworthy memory.
`[dialogue-sourced 2026-06-20 to 22; verify before external use.]`

That the guestbook accumulates is exactly why it carries a launch gate the other activities do not. The moment
a family trusts it with years of memory, it has created a **custodial obligation** — a promise to keep that
memory, which a product with no server between two people can only honor if it can outlive its own
enthusiasm. So the concrete link runs straight from the build sequence to the governance argument: the
sustainability model (the cooperative-dues-or-foundation answer) must be at least sketched before the
guestbook ships, because that answer is what turns "no one makes a buck" from a vulnerability into a credible
forever-promise, and it is the first thing a skeptic pokes. Do not invite that trust before you can keep the
promise. The sustainability model itself is worked out in `../governance/foundation-cooperative-and-sustainability.md`;
what belongs here is the gate — the guestbook is the point in the build where accreting memory makes that
model a precondition, not a later concern. `[dialogue-sourced 2026-06-20 to 22; verify before external use.]`

**The question-of-the-day is the engine that keeps the pond alive — and it carries no streak.** This is a
voice decision, stated here as load-bearing rather than incidental. A daily prompt is exactly the surface
where a conventional product would attach a streak counter, and the decision is to refuse it:

> an invitation, never a Skinner box; that one decision is the whole ethic in miniature
>
> — Croft design dialogue, 2026-06-20 to 22 `[dialogue-sourced; confirm against primary before external use.]`

The why is not squeamishness about metrics. A streak manufactures obligation: it converts presence into
compliance and punishes absence, which is the precise opposite of a product whose stance is that presence is
offered and never extracted. The no-engagement-trap principle is therefore a voice decision that binds every
activity in this pond, not a preference for the daily prompt alone — the moment any activity here rewards
frequency or punishes a lapse, the pond has stopped being the project's heart and become a retention
mechanism wearing its skin.

**Invites close the outward loop.** Where the ping and the guestbook turn a group in on itself, the invite
points it outward to a gathering and then feeds the result back: a find-a-time settles *when*, the invite
carries the details and reminds, and the gathering itself fills the guestbook. That arc is why the invite
earns a place beside the ping and guestbook rather than sitting in a utility pond — it is the mechanism that
turns present-tense activity into accreting memory, joining the pond's two ends. The honest caveats are
operational and follow directly from having no server: reminders fire as local notifications per device (there
is no push server to guarantee delivery), times are stored as absolute instants so a crossed time zone does
not misfire the reminder, and a rendered map would reintroduce the map-tile dependency the product otherwise
avoids. `[dialogue-sourced 2026-06-20 to 22; verify before external use.]`

**The organizing axis is persist-vs-evaporate.** Every activity in the pond is placed on one axis, decided
per activity: the ping evaporates, the guestbook accretes, the invite sits between (it matters until the
event, then it is spent). The axis is load-bearing because it is what keeps a casual gesture from leaving an
accidental permanent record and keeps a memory worth holding from being lost to ephemerality — getting the
placement right per activity is the design work, not a default applied uniformly. The axis is also concrete,
not only editorial: the evaporate end rides gossip (live and unstored) and the persist end rides the
document-and-blob store, so where an activity sits on the axis decides which substrate mechanism actually
carries it. `[dialogue-sourced 2026-06-20 to 22.]`

**The pond is a family of rituals, not a fixed three.** The ping, guestbook, question-of-the-day, and invite
are the load-bearing set, but the same ethic and the same axis admit a wider roster of low-stakes rituals — a
rose-thorn-bud check-in, a would-you-rather, a gratitude exchange, a countdown, an ambient status-glance, a
collaborative playlist. Two properties hold across all of them: each is placed on the persist-vs-evaporate
axis rather than defaulted onto one, and each obeys the no-engagement-trap rule. One carries a design point
worth stating on its own — the collaborative playlist shares *references*, not audio: the pond passes pointers
to tracks and never the media itself, which sidesteps both the licensing hazard and the blob-transport cost
while still leaving a shared artifact behind. `[dialogue-sourced 2026-06-20 to 22; verify before external
use.]`

A reusable filter explains why this pond stays small-group and friends-only: the appetite problems that sink
watch-together and similar features (discovery surfaces, stranger rooms, moderation burden, engagement metrics
to justify the infrastructure) are inherent to *scale plus strangers*, not to co-experience itself. Keep the
activities friends-only and small, and those problems never form. `[dialogue-sourced 2026-06-20 to 22.]`

## Game outcome as a custom lexicon

A peer game raises a specific product hazard: if the moves are written as ordinary public records, the open
network fills with machine chatter nobody wants in a feed, and the write frequency strains the underlying
store. The resolution is a clean split between what is ephemeral and what is durable.

The play itself lives on the direct peer connection over the transport and stays there — moves never touch a
durable record, and the game is gone when it ends. The only durable artifact is the *settled outcome*: one
record per completed game, not one per move, which removes the write-frequency tension entirely.

That outcome record is a **custom lexicon** — its own record type, public in the open-network sense (fully
traceable and resolvable, not hidden), but it is not a post and does not render in an ordinary timeline. It
therefore surfaces only where a client knows the lexicon. The product framing the user coined for this is
exact:

> a public leaderboard of direct peer interactions openly attributable and rendered where desired
>
> — Croft design dialogue, 2026-06-20 to 22 `[dialogue-sourced; confirm against primary before external use.]`

This is *follow-or-ignore at the data layer*: the private thing (play) leaves no record, the public thing
(the outcome) is an attributable record with no leakage between them, and the opt-in lives in the rendering,
not the privacy. Anyone with a reason can follow the outcome down to its source; anyone without one never
renders it. This is the same follow-or-ignore contract the garden doc makes mandatory at the surface, pushed
down to the record layer.

**Outcome attestation is the real design work, and it is open.** The hazard the split does *not* close is that
a self-reported win is attributable but gameable. A secure authenticated channel — which the transport gives
for free, since peers are mutually authenticated by construction — is not the same thing as a non-repudiable
record a third party can verify later; when the session ends, the channel's authentication evaporates. The
direction is a mutually-signed attestation (both peers sign the agreed outcome, producing an artifact anyone
can check against both identities), which defeats unilateral lying without a blockchain. But the honest limit
must be stated plainly:

> a co-signature proves both agreed, not that the outcome is true
>
> — Croft design dialogue, 2026-06-20 to 22 `[dialogue-sourced; confirm against primary before external use.]`

Two colluders can co-sign a fabricated result; mutual attestation defeats a lying loser, not a colluding pair,
and that residue is a social/sybil problem handled at the social layer, not a cryptographic one. This is
surfaced as open design work, deliberately set aside — the durable-outcome mechanism is settled; how an
outcome earns trust is not.

## The iroh tiered-exposure product model

"How does the web version work" is a product question, not only an engineering one, because the answer sets
what a web visitor is honestly promised. The model is three tiers mapped to effort-versus-value.

**Tier 1 — public read-only bridge.** A group opts to expose; an anonymous visitor reads a normal web URL; a
server-side native node projects public content over HTTPS. This is the 80/20 winner and the default. The hard
problems evaporate precisely because it is public and read-only: no key custody (nothing is private) and no
auth (the visitor is anonymous). Its product role is the growth funnel — encounter before commit — which is
what makes it worth building first for a friends-only tool that has no public discovery by design. The honest
caveat is that the bridge sees plaintext: it is a trusted-server client, not an end-to-end-private one.

**Tier 2 — logged-in browser peer.** A real peer running in the browser, relayed but genuinely E2EE. Because
browsers cannot hole-punch, a browser peer's traffic is relayed for the entire session:

> the relay carries all traffic for the entire session — a complete relay broker, not setup-and-handoff
>
> — Croft design dialogue, 2026-06-20 to 22 `[dialogue-sourced; confirm against primary before external use.]`

The load-bearing distinction is that "relay carries everything" is not "relay sees everything": encryption is
independent of routing, so the relay relays ciphertext it cannot decrypt. A browser peer therefore gives up
the direct connection (a performance and topology property) but keeps full end-to-end encryption (the security
property) — which is exactly what makes Tier 2 worth doing rather than folding into the plaintext bridge. Its
capability ceiling is upstream-gated (encrypted text over gossip is real today; richer document and blob
transport in the browser is not yet), so it is built only as far as upstream solidly supports and grows as
upstream does. `[dialogue-sourced 2026-06-20 to 22; iroh capability facts cite the FACTCHECK source of truth
and are not re-verified here.]`

**Tier 3 — native full peer.** A full native node with the entire feature set (direct connections,
hole-punching, the full data layer). The server-side node in Tier 1 is itself a full native node, so the same
model that serves the local app serves the public-but-authenticated version; only the node's location differs,
never its capability.

**Run-your-own-relay is the product-facing consequence.** For a normal native peer the relay is
setup-and-handoff and drops out once a direct connection forms; for a browser peer it is load-bearing for the
whole session. That makes relay infrastructure a real, standing operational cost the bridge-only path does not
carry, and the honest charter answer is that Croft would run its own relay rather than treat the default
public relays as free-forever plumbing. `[dialogue-sourced 2026-06-20 to 22.]`

## Composition and valuation: the two edge types

Every group is a group of groups. A user is a group of devices; a community is a group of users; there is one
recursive principal primitive underneath all of it. `social-graph-as-substrate.md` carries the durable group
and its identity; the piece to carry here is that trust and structure between these recursive groups are *two
distinct edge types over that one primitive*, and they must be kept crisp or trust leaks into key access.

- **Composition** — shared MLS lineage, the authoritative-state-merging edge: the device-pool-in-user-in-
  community nesting where the parties share key lineage. This edge is already covered by the substrate doc and
  the Drystone spec; it is named here only to locate the boundary, not re-derived.

- **Valuation** — the new piece this doc carries: a *directional, weighted trust* relationship between
  cryptographically-separate groups, with **no shared keys**. "We weight this group's word highly" is a
  valuation edge; it does not merge state and it does not grant decryption.

The why for the strict separation is the whole point. If valuation shared keys with the group it trusts, then
expressing trust in a group would silently grant that group access to protected content — trust would leak
into key access, which is the confused-deputy failure at the architecture level. Keeping valuation keyless
means directional trust can be expressed, weighted, and revised without ever becoming a key-management fact. A
corollary the model makes explicit: adversarial posture is a *per-edge property, not a global stance* — a
device pool is a high-trust, low-adversarial composition edge; a stranger valuation is a low-trust edge — and
forcing Byzantine rigor onto your own device pool is as wrong as omitting it from a stranger edge.
`[dialogue-sourced 2026-06-26; the recursive-principal and edge model is core Drystone — cite the spec as
source of truth.]`

## What this establishes (and does not)

Establishes the specific product substance the sibling docs leave as stubs. The Presence & Ritual pond is the
project's heart: a thinking-of-you ping delivered as a near-empty packet, a guestbook as accreting connective
tissue, an invite that closes the loop from gathering back into that memory, and a question-of-the-day whose
*absence of a streak* is a voice decision carrying the whole no-engagement-trap ethic — a family of rituals,
not a fixed three, all organized on a persist-vs-evaporate axis that maps to the gossip-vs-store substrate
split. Peer-game results use
game-outcome-as-custom-lexicon: ephemeral play over the transport, only the settled outcome durable, rendering
only where a client knows the lexicon (follow-or-ignore at the data layer). The web version is a three-tier
exposure model (public read-only bridge as growth funnel, logged-in browser peer that is relayed but E2EE,
native full peer), with the relay a complete broker for browser peers and run-your-own-relay the honest
operational consequence. Trust between recursive groups is two edge types — composition (shared MLS lineage)
and valuation (directional, weighted, keyless) — kept separate so trust cannot leak into key access.

Does not reproduce the ponds/pads taxonomy, the garden thesis, or the functional-core spine
(`product-the-garden-of-ponds.md`), and does not reproduce the durable-group substrate, lifecycle, or
group's-face UX (`social-graph-as-substrate.md`). Does not re-derive the composition edge (the substrate doc
and the Drystone spec own it; only the valuation edge is carried here). Does not resolve outcome attestation:
the durable-outcome mechanism is settled, but how a self-reported outcome earns trust against collusion is
open design work, surfaced and set aside. Dialogue-sourced product calls carry verification flags and need a
refresh pass against primary sources before external use.
