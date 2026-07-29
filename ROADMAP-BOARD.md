# Roadmap board — product lanes and kanban usage

date: 2026-07-29

**What this is.** How we track work across the Croft project on GitHub Projects, why the board is
shaped the way it is, and the conventions for keeping it healthy. This is the operational companion to
the layer model in `beta/LAYERS.md` (which defines the lanes) and the backlog in
`alpha/ROADMAP_TODO.md` (which is the item-level source of truth). This file governs *board usage*;
`LAYERS.md` governs *what the lanes mean*; `ROADMAP_TODO.md` governs *the full open-item list*.

## The product, briefly

Croft is a peer-to-peer, cooperatively-governed communication substrate. The stack is a nine-layer
cake (`beta/LAYERS.md`): material and intellectual history at the base, a survey of the surrounding
field (open and fenced), then the build — **Drystone** (the certifiable protocol) → a reference
implementation → the **Croft** product → its manifestation as a foundation-sponsored cooperative —
and the outward edges of socialization and activism. Croft is one "flavor" on a neutral core: the
foundation stewards the neutral protocol and reference implementation; Croft is a product built on
top; the cooperative operates it.

The board turns that layer cake into **lanes** so we can start at the build-order start (the spec) and
parallelize the layers that have no hard dependency.

## The board

- **Project:** `Croft Roadmap` — https://github.com/orgs/CroftCommunity/projects/2 (org-level, so it
  can span every nested repo, not just one).
- **Two custom fields drive everything:** `Lane` (which layer the work belongs to) and `Status` (the
  kanban column). One project, filtered/sliced by `Lane`, gives each lane its own board without
  fragmenting state across many projects.

### Lanes (the `Lane` field)

The lanes are the layer cake plus one cross-cutting lane for the decisions that gate the build.

```
◆ Decisions & Gates   cross-cutting — the choices that block a layer (mostly the owner's calls)
  L1 History          why it resonates (material history)
  L2 Philosophy       why it is right (principles + thinkers)
  L3 Field - cairn    survey of the OPEN field we build among
  L3 Field - fenced   map of the FENCED commercial platforms we are an alternative to
  L4 Spec (Drystone)  the protocol — the spine / critical path
  L5 Impl             reference core + validation spikes
  L6 Croft (product)  product + brand
  L7 Governance       foundation + cooperative (legal / financial actualization)
  L8 Socialization    getting the message out
  L9 Activism         why not the status quo
```

`Decisions & Gates` is not a layer; it exists because the owner-level decisions (the `A` items in
`ROADMAP_TODO.md`) block the spec spine, and burying them inside a layer hides the dependency. When a
decision is settled, its downstream work moves into the layer it unblocks.

### Status columns (the `Status` field)

```
Backlog → Ready → Parked → WIP → Done

Backlog   defined but needs more work to figure out the exact work, or an idea saved
Ready     vetted and ready for development when capacity is there
Parked    blocked on an external thing or other work (e.g. a manual DNS entry)
WIP       in flight
Done      complete
```

`Backlog` and `Ready` split "not started" by readiness: `Backlog` still needs scoping; `Ready` is
vetted and can be picked up the moment there is capacity. `Parked` is specifically for *external*
blockers (a legal review, a DNS change, someone else's work) — not "we haven't gotten to it," which is
`Backlog`.

## Intent — how the board expresses the plan

Two orderings run over the layer cake (`beta/LAYERS.md` → "Two traversals"). The board is arranged for
the **build order**:

```
START HERE (parallel, no cross-dependency):
  Decisions & Gates  ── resolve the gates that block the spec   ◀ highest leverage
  L4 Spec            ── the spine (partly blocked by the gates)
  L1 / L2            ── history + philosophy, independent writing
  L3 cairn / fenced  ── field survey, informs the spec
  L8 / L9            ── socialization + activism, outward edges

NEXT (unlocks as the spec firms up):
  L5 Impl            ── validation spikes run ahead to de-risk; the core follows the spec

LATER / GATED:
  L6 Croft (product) ── follows impl
  L7 Governance      ── parked on legal review
```

The single highest-leverage move is clearing the `Ready`-shaped decision gates in
`Decisions & Gates`, because they unblock Layer 4.

## Conventions (keep the board healthy)

- **`ROADMAP_TODO.md` is the backlog of record; the board is a curated working surface.** Do not
  re-list every backlog item as a card. A card is something we intend to actually move soon. The full
  open-item list, with provenance back-references, stays in `alpha/ROADMAP_TODO.md`. Adding a parallel
  list is the anti-pattern that file exists to prevent.
- **Every card gets a `Lane` and a `Status`.** A card with no lane is invisible to the per-lane views.
- **Card titles carry the origin code** where one exists (`A11`, `B2`, ...), so a card traces back to
  its `ROADMAP_TODO.md` row and reasoning home. The card body names the origin file.
- **Graduating work onto the board:** when a lane's next items are ready to order, pull them from
  `ROADMAP_TODO.md` into the board as cards, lane-tagged, starting in `Backlog`. We decide together
  what graduates; the board is not a dumping ground.
- **State management is ongoing work.** Moving cards across columns, opening new ones, and retiring
  done ones is part of the normal loop, not a one-time setup.

## Board views (the one manual step)

GitHub's API can create the project, the fields, and the cards, but it has **no mutation to create
board *views***. So the per-lane boards are set up once in the web UI:

- Fastest: open the project, switch the default view to **Board** layout, set the column field to
  **Status**, then turn on **Slice by → Lane** (left panel). One board; click any lane in the slicer
  to see that lane's kanban.
- Or make a dedicated view per active lane: `+ New view` → Board → column field `Status` → filter
  `Lane is "<lane>"`.

## Managing board state programmatically

An agent updating the board discovers the current field and option IDs (they are internal and can
change) rather than hardcoding them:

```bash
# project node id + fields
gh api graphql -f query='{ node(id:"<projectId>"){ ... on ProjectV2 {
  fields(first:30){ nodes {
    ... on ProjectV2SingleSelectField { id name options { id name } } } } } } }'
```

The project id for `Croft Roadmap` is discoverable via
`gh project list --owner CroftCommunity`. Setting a card's field is
`updateProjectV2ItemFieldValue`; adding a card is `addProjectV2DraftIssue`. Creating the project
requires the `project` token scope (`gh auth refresh -h github.com -s project`).

## Current seed (2026-07-29)

The first pass seeded only the critical-path start; everything else stays in `ROADMAP_TODO.md` until
we order it onto the board.

```
Decisions & Gates
  Ready    A11  Capability mechanism: Track A (Meadowcap) vs Track B (Keyhive)   ◀ unblocks spec
  Ready    A12  Key-custody default: blind-relay vs revocable delegate           ◀ unblocks spec
  Ready    A13  Rename the geer gating-peer
  WIP      A2   Total-device-loss recovery anchor (architecture decided → prototype)
  Parked   A1   MPL-2.0 license gate (compliance / legal review)
  Done     A14  Drystone spec text license → CC0 1.0

L4 Spec (Drystone)
  Ready         Spec currency / CCC pass (core-only Part 1+2)
  Backlog       Part 1 / Part 2 open-threads reconciliation

L5 Impl
  Ready    B2   Verify openmls leaf-credential dependency
```

Active lanes stood up first: `Decisions & Gates`, `L4 Spec`, `L5 Impl`. The other eight lanes exist as
`Lane` values (cards can be filed against them) and get dedicated views as work graduates into them.

## Deferred — next board steps (todo for later)

Both are tracked as `Backlog` cards on the board (lane `Decisions & Gates` as a placeholder — re-lane
during triage) so they surface when we get to them.

1. **Set up the per-lane board views.** The data model and cards are in place; only the *views* remain,
   and GitHub has no API to create them. In the project UI: switch the default view to **Board**
   layout, set the column field to **Status**, then **Slice by → Lane** (one board, click a lane in the
   slicer to see its kanban); or add a dedicated Board view per active lane filtered on `Lane`. This is
   a manual step because it requires driving the authenticated GitHub UI (the `chasemp` account +
   2FA) — not something to automate headless from the sandbox.

2. **Order the fuller backlog onto the board.** `ROADMAP_TODO.md` sections B (validation/spikes),
   C (backports), D (strategic), and E (explorations) hold ~118 more open items. Map them to lanes and
   decide, lane by lane, which graduate onto the board next — keeping the board a curated surface, not
   a mirror of the whole backlog.
