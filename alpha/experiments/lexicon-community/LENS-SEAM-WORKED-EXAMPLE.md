# EXP-LEX-05 — the Lens seam, as a worked example

`Stretch experiment, downgraded per its own instruction ("if the WG has no
concrete format yet, downgrade to a worked-example document"). Status check
2026-07-20: the Polite Goshawk WG (Lexicon Lenses — record→record transformation,
Cambria-inspired) is at working-group-FORMATION stage
(lexicon-community/governance issue #14); the public repo
lexicon-community/wg-polite-goshawk returns 404. There is no concrete lens
definition format to implement against today. So this is a worked example of the
seam, ready to become a real lens the day a format lands.`

## The seam

Croft's envelope-projection (the calendar view a Croft group emits) and the
lexicon.community `community.lexicon.calendar.event` record are two spellings of
the same fact: an event with a name, a start, and a place. A **lens** is exactly
the transformation between them — so that a Croft AppView can publish its native
projection AND a lens that renders it as the community calendar type, and any
calendar AppView (Smoke Signal, etc.) can display it without special-casing Croft.

That is the Polite Goshawk thesis restated in our terms: "other AppViews create
records as they wish, and also publish a lens to allow their types to conform to a
virtual type." The virtual type here is `community.lexicon.calendar.event`.

## The transform (worked, against EXP-LEX-02's real fixtures)

Source — a Croft envelope projection (illustrative shape):

```json
{
  "$type": "ing.croft.project.calendarProjection",
  "title": "Lexicon Community sync",
  "body": "Attestation lexicon working session.",
  "window": { "opens": "2026-07-28T17:00:00.000Z", "closes": "2026-07-28T18:00:00.000Z" },
  "presence": "online"
}
```

Lens → `community.lexicon.calendar.event` (the virtual type):

| target field | ← source | note |
|---|---|---|
| `name` | `title` | rename |
| `description` | `body` | rename |
| `createdAt` | (lens-time / envelope stamp) | required by target; supplied by the projector |
| `startsAt` | `window.opens` | path move |
| `endsAt` | `window.closes` | path move |
| `mode` | `presence` → `#virtual` \| `#inperson` \| `#hybrid` | value map into the target's `knownValues` |
| `status` | (default `#scheduled`) | target default |

The **round-trip** property a real lens must hold (and which we already have the
machinery to test): projecting Croft→calendar then reading the calendar record
back must preserve every field the target models; fields the target cannot hold
(Croft-specific `presence` nuance beyond the three modes) are the lens's declared
lossy edge, recorded, not hidden. EXP-LEX-02 already proves the calendar side
round-trips byte-identically through DAG-CBOR — so once a lens format exists, the
only new surface is the field map above, and its inverse.

## Why this is the right shape to wait with

- It rides single-author records (a Croft repo publishes its own projection + its
  own lens). No cross-repo writes. Same scope line as everything else in RUN-LEX-01.
- It needs nothing from the attestation work — it's a separate seam, offered to a
  separate WG. We surface it only after the attestation traction (brief §5, step 5).
- When Polite Goshawk publishes a format, this document becomes a lens file plus a
  `round_trip_stable`-style test over `fixtures/recorded/` — a small, ready follow-up.

## The ask, if it comes up in-thread

Not "adopt our projection." Rather: as the lens format firms up, the
Croft→calendar map above is a concrete, real-fixture-backed test case the WG can
use to pressure-test the format's expressiveness (renames, path moves, enum value
maps, and one honestly-declared lossy edge).
