# Lexicon Community — a schema-steward governance body

`Cairn stone, 2026-07. Orientation: **related-ecosystem** (a peer institution Croft neither
builds on as a dependency nor ships as product, but tracks as a near neighbor — the third
orientation within cairn, distinct from Drystone-oriented building blocks like MLS/Willow and
Croft-oriented product blocks like the app-pond references). This stone holds the governance-body
depth; the namespace *mechanics* (NSID, Smoke Signal as worked example) stay in
`atproto-nsid-and-lexicon-mechanics.md`, and the register row is in `reference-index.md`.`

> **Grounding & status.** Synthesized from their site, governance repo, and working-group
> threads (2026-07). Marked **grounded-from-site, pending governance-repo verification** — the
> actual `GOVERNANCE.md` bylaws text, TSC membership and seat-turnover rules, the full state of
> the attestation proposal thread, and whether any Lenses reference implementation has shipped
> are the open verification items (ROADMAP_TODO). The engagement brief and executed experiments
> live in `../../alpha/experiments/attest-family/ENGAGE-LEX.md` and
> `../../alpha/experiments/lexicon-community/`.

## What it is

Lexicon Community is a volunteer-run organization holding the `community.lexicon.*` namespace:
public, communal ownership of AT Protocol lexicon schemas, MIT-licensed, free to use, fork, and
remix. It exists because atproto schemas normally live under a vendor's namespace (`app.bsky.*`
belongs to Bluesky the company), so whoever owns the domain owns the schema. Gerakines
established it around Smoke Signal's first anniversary and transferred the event/RSVP lexicon
into it as a **credible-exit commitment**: the records Smoke Signal writes could be produced by a
different tool, and moving the schema out of his own hands made that promise structural rather
than personal.

## How it is governed

A **Technical Steering Committee** of seven stewards the namespace, with all decisions made in
public. The governance repo carries `GOVERNANCE.md` (TSC bylaws), `CONTRIBUTING.md`, and
`COVENANT.md` (contributor covenant / code of conduct). New lexicons are not designed by the TSC;
they come from small, focused **working groups**. The lifecycle:

- Ideas begin on their Discourse forum; the bar is **interoperability-first** — a proposal is
  expected to point at existing lexicons, apps, or SDKs it would connect and name the builders
  already working in the space.
- Interested parties collaborate on the design; the TSC designates a **sponsor** for the working
  group; collaborators plus at least one TSC member review the PR in the open.
- Merged lexicons are published through official discovery paths (PDS records **and** DNS), then
  announced.

Working groups get whimsical bird names. The notable current one, **Polite Goshawk**, is
designing **Lexicon Lenses**: transformations of records from one lexicon to another, aimed at
letting AppViews consume unfamiliar schemas through a known one, and giving versioning semantics
via lenses from past versions to new ones.

## What it stewards today

- **calendar** — events and RSVPs (the Smoke Signal lineage; now also consumed by OpenMeet,
  Dandelion, atmo.rsvp, and various importers)
- **location** — places in multiple encodings (street address, geo coordinates, H3 cells, POIs)
- **bookmarks**

Browsable via **lexicon.garden**, an independent lexicon browser and validator. Also in active
discussion there: the **Attestation and Signatures** proposal that the Croft attest lane's signed
RSVPs follow (the thread adjacent to `../../alpha/experiments/attest-family/`).

## Maturity, honestly

Young and thin, with good instincts. The good: when a merged location lexicon shipped with a bug
rendering a type invalid (February 2025), they published a **numbered incident report in public**
— the transparency habit you want in a schema steward. The rough: the repo is small
(a dozen-odd commits, ~103 stars), the process depends on GitHub-and-goodwill rather than
anything enforceable, and the authority question is openly unresolved — Gerakines himself argues
the Lexicon spec is missing authority and discovery primitives (his lexicons-as-records
proposal). So pin versions, watch the governance repo, and expect evolution.

## Why it matters to Croft — the three stones

**The door.** The working-group chartering process is exactly how the attestation work enters the
shared ecosystem, and Croft would arrive matching their stated bar unusually well: named builders
already there (Acudo, Smoke Signal), an existing proposal thread to join (the Attestation and
Signatures discussion), and a tested reference model behind it (RUN-LEX-01, executed —
`../../alpha/experiments/lexicon-community/RUN-LEX-01-SUMMARY.md`).

**The seam.** Lexicon Lenses are the formal shape of the envelope-to-calendar-record projection:
a lens is the projection relationship between Croft's envelope-canonical records and their
calendar records, and their versioning-by-lens is the answer Croft would otherwise have to
invent at the seam. Worked out in `../../alpha/experiments/lexicon-community/LENS-SEAM-WORKED-EXAMPLE.md`.

**The resonance.** lexicon.community is a seven-member committee governing a shared namespace by
convention and goodwill — a group doing *by social trust* exactly what Croft's governance
machinery (co-signed provable rules) does *by structure*. It is both a partner today and, someday,
the most natural demonstration audience imaginable for governance-by-provenance.

## Pointers

- Namespace/NSID mechanics + Smoke Signal worked example: `atproto-nsid-and-lexicon-mechanics.md`
- Register row: `reference-index.md` → "atproto apps, clients & aggregators"
- Engagement brief + the five open calls (EL OC-1…OC-5): `../../alpha/experiments/attest-family/ENGAGE-LEX.md`
- Executed experiment package (RUN-LEX-01): `../../alpha/experiments/lexicon-community/`
- Glossary edge (org-level): `../socialization/kindred-work.md` → `org-lexicon-community`
