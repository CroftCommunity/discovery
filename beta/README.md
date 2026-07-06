# discovery / beta — the resolved synthesis

date: 2026-06-24

**What this stage is.** `beta/` is the *resolved, cohesive, themed synthesis* of the `alpha/`
corpus. Where `alpha/` is first-pass, concurrently-discovered thinking spread across ~120 files,
`beta/` pulls the split threads together, collapses the contradictions `alpha/COHESION.md` tracked,
and harvests the conclusions that landed in a transcript but never got pulled up into the synthesis
layer. It is built for synthesis + real validation, not for re-deriving the thinking. (Stage model:
`../AGENTS.md` → "Maturity stages".)

**What this stage is NOT.** Not new intake (that still lands in `alpha/`). Not a rewrite of `alpha/`
— `alpha/` is the frozen provenance floor and is never moved, edited, or deleted (PLAYBOOK §4).

## Tier discipline (the core invariant)

- **Beta is cohesive within itself.** A beta doc references only *other beta docs* for its forward
  narrative. A reader should be able to walk beta end-to-end without dropping into alpha.
- **No prior-tier references inside a beta doc.** A beta doc reads clean at its own maturity level — it
  carries **no links back to alpha**: no `Sources (alpha)` footer, no `Provenance trace` line, no inline
  `thinking/…`/`research/…` quote attributions, no `COHESION §` / `BETA-ROLLUP` pointers, no `../alpha/…`
  paths. The entire "what was lifted from each alpha source, how it was treated, where it landed" map
  lives **only** in `../alpha/BETA-ROLLUP.md` (the handoff doc at the prior level), so the two tiers can
  be laid side by side without the matured tier carrying the trail. A beta doc ends on "What this theme
  establishes (and does not)." The one exception is `OPEN-THREADS.md`: an unsettled thread keeps its alpha
  connective tissue there until it lands — then the content goes into the (clean) beta doc and its mapping
  moves into the rollup.
- **The discipline tightens with maturity.** Each stage carries fewer prior-tier traces than the last; by
  `rc` and `publish` the documents are clean of prior-tier references *entirely* — even the markers a beta
  doc still carries (the `cite FACTCHECK SoT` pointer and per-claim verification flags) resolve as facts
  harden into settled statements. Beta applies the floor; rc/publish complete it.
- **"Do not carry forward" means absent, not annotated.** Excluded material (the crofting "ancient free
  clan" myth; the REFUTED crypto-wars quotes; the MO §351 statute specifics; `did:key`
  atproto-resolvability) simply **does not appear in beta** — that is what not carrying it forward
  means. *What* was excluded and *why* is recorded once, in the alpha rollup ledger (treatment code
  `excluded`), never as a list inside a beta doc.
- **Alpha stays frozen.** Raw stays frozen forever; keeping vs. eventually retiring the alpha tier
  is a later call (default: keep for provenance). The rollup ledger is the only *additive* file in
  alpha for this transition; existing alpha content is untouched.
- **Verification travels with carried claims.** Each claim that *is* carried forward keeps its status
  inline (`green-real` / `green-model` / `green-real-multimachine` / `spec` / `[UNVERIFIED]` /
  `NOT-LEGAL-ADVICE`), and each verbatim quote keeps its citation + per-quote verification flag (e.g.
  "confirm against primary edition before publish"). For atproto / iroh / iOS facts, cite the
  source-of-truth FACTCHECK
  (`../alpha/seeds/transcripts/raw/atproto-atmospheric-web-iroh-mobile-FACTCHECK.md`) — do not re-verify.

## The per-theme doc template

Every theme doc follows the same shape (established on 01 and 04):

1. **Title + status + verification line.**
2. **Theme narrative (overview)** — a prose summary of what the doc covers and the arc it walks, so a
   reader gets the whole shape before the sections.
3. **Charter — what this theme covers** — explicit *in scope* / *out of scope (and where it lives)* /
   *boundary calls* against adjacent themes.
4. **Body** — the resolved synthesis. **Direct quotes are preserved whole** as referenced block quotes
   with a citation and a per-quote verification flag, each tied to the conclusion(s) it grounds — never
   paraphrased away.
5. **What this theme establishes (and does not).** ← the doc ends here.

No `Sources (alpha)` footer, no `Provenance trace`, no "do not carry forward" list, and no detailed
rollup table inside a beta doc — all of it lives at the prior level (`../alpha/BETA-ROLLUP.md`), per the
tier discipline above.

## The themes (reading order)

The seven lineages of thinking (G1–G7) resolved into narrative themes. Reading order runs
why → history → field → protocol → identity → safety → sustainability → product.

**The "why" is now the Drystone protocol spec.** Theme `01` (the epistemic foundation) was
**superseded 2026-06-26**: its reasoning became **Part 1 (Reasoning Underpinnings)** of the
vendor-neutral **Drystone protocol specification** (`drystone-spec/`), which pairs it with **Part 2 (The
Certifiable Design)** — the build-against, certify-against mechanics. The `02`–`08` numbering is retained
for stability (cross-references are stable); there is intentionally no `01` theme doc. Drystone is the
protocol; Croft is one ecosystem built on it (see `drystone-spec/README.md`).

| # | doc | thesis (one line) | lineage | status |
|---|---|---|---|---|
| — | `drystone-spec/` (Part 1) | The "why": no center can hold the truth; compute provenance, never utility; local-first is the generative premise; a peer is a locus of adjudication. | N1 / G1 | **spec — Part 1** (supersedes the former `01`) |
| 02 | `02-enclosure-and-its-inversion.md` | Every era's commons gets enclosed; the croft is the rare halt — a private plot coupled to a surviving common. | N2 / G3a+G1 | **drafted** |
| 03 | `03-the-living-ecosystem.md` | Against the live field (atproto/Solid/DSNP/Nostr/Matrix/Germ), Croft's bet is "different, not weaker." | N3 / G3b | **drafted** |
| 04 | `04-the-protocol-we-proved.md` | A group is a navigable lineage, not an eternal room — and it is green-real on openmls 0.8.1, not a sketch. (The *narrative* of proving the protocol; the spec itself is `drystone-spec/`.) | N5 / G4 | **drafted** |
| 05 | `05-identity-you-carry.md` | Keys are not identity; a person is a DID lineage; cross-platform continuity is attestation, never a master key. | N6 / G5 | **drafted** |
| 06 | `06-safety-without-surveillance.md` | A content-blind system stays safe by structure, not inspection; membership ≠ access. | N8 / G6 | **drafted** (graduated to its own theme) |
| 07 | `07-sustainability-and-stewardship.md` | Non-extraction is anti-fragile — if a cooperative mechanism *and* an IP-stewardship structure carry it. | N4 / G2 | **drafted** — **decision-gated** |
| 08 | `08-croft-the-product.md` | The social graph is the substrate; chat is a tenant; a durable group carries co-equal sibling activities — surfaced as a composable garden of ponds + pads on one core, thin shells. | N7 / G7 | **drafted** — **decision-gated** |

```
  drystone-spec ─► 02 ─► 03 ─► 04 ─► 05 ─► 06 ─► 07 ─► 08
  why + design     history field protocol identity safety sustainability product
  └─ the protocol ─┘  └──── what we built & proved ────┘  └── how it survives & ships ─┘
```

## The layer-cake (emerging structure, populated across sessions)

Alongside the theme docs, beta is being organized into a **maturity layer-cake**: the themes migrate into
layers as they settle. The ordering runs from the "why" that grounds everything, up through the build, out
to the edges. The canonical layer model, the two traversals, and the register distinctions live in
**`LAYERS.md`**; this table is the index. Layer directories are seeded as material arrives, not all at once:

| Layer | Dir | What it holds | State |
|---|---|---|---|
| 1 | `history/` | MATERIAL history: crofting, dry-stone, cairns, the space itself (theme 02 to migrate) | not yet created |
| 2 | `philosophy/` | INTELLECTUAL history: the principles + thinkers, the pure peer-standing argument | **seeded 2026-07-06** (peer-standing → cooperative-form set) |
| 3 | `drystone-spec/` | Protocol spec (Parts 1+2, persona-definition, CHANGELOG, open-items, review-handoff) | populated |
| 4 | `impl/` | Reference implementation, experiment-informed (themes 04/05/06 to migrate) | not yet created |
| 5 | `croft/` | Product thinking, Croft as a "flavor" (theme 08 to migrate) | not yet created |
| 6 | `governance/` | The manifestation: foundation + cooperative, legal/financial actualization (theme 07 to migrate) | **seeded 2026-07-06** (reserved; peer-standing argument moved to `philosophy/`) |
| 7 | `socialization/` | Brand / voice / adoption, *presentation* register (T4 / T11 terrain) | **seeded 2026-07-06** (human-facing peer-standing pieces) |
| 8 | `activism/` | The case against the status quo: why incumbent platforms are harmful, *empirical* register | **seeded 2026-07-06** (the "platforms author the relational field" research set) |

**The ordering.** The "why" sits at the base (Layers 1–2: history = why it resonates, philosophy = why it
is right), because the spec is built *on* the principles. Then the build (3 spec → 4 impl → 5 croft → 6
governance), then the outward edges (7 socialization = get the message out; 8 activism = why not the status
quo, the *present* "why"). Read `LAYERS.md` for the build-order vs justification-order traversals.

The `02`–`08` theme docs at beta root remain the current reading spine; layer dirs carry their own
`README.md` index. The peer-standing companion work (2026-07-06) was **split by register**: the pure
argument → `philosophy/` (Layer 2), the empirical harm case → `activism/` (Layer 8), the human-facing
presentation → `socialization/` (Layer 7); `governance/` (Layer 6) holds the *manifestation* of that
argument (foundation + co-op), not the argument itself. `history/` (Layer 1) is not yet seeded (the
material crofting/dry-stone lineage; theme `02` is its seed). The Drystone spec's Part 1 §2.6 (voice
requires field-integrity, document-pass-3) is the joint tying the protocol to this set: it points at
`philosophy/` for structural grounding and `activism/` for empirical grounding, depending on neither for a
mechanism.

## Structural decisions taken for this synthesis (2026-06-24)

- **N2 (history) and N3 (ecosystem) are two themes**, not one — a historical arc reads differently
  from a present-day field comparison.
- **N8 (safety-under-blindness) graduates to its own theme** (06), reinforced by the new
  membership-vs-access node; it is not folded into 04 as a mere adversarial chapter.
- **N1 (epistemic) is its own theme** (01), upstream of the history (02) — splitting the old
  "civic-why" the alpha plan noted was conflated.
- **Naming is distributed**: croft/Drystone meaning rides in 02; the Noria foundation-name candidate
  rides in 07. No standalone naming doc.

## Standing decisions surfaced, not resolved (the user's calls)

Themes 07 and 08 carry a "decision-gated" banner until these land. They are surfaced here so beta
never silently resolves them:

- **MPL-2.0 license** for the substrate (vs the AGPL-3.0-or-later choice for the app/foundation layer).
- **Total-device-loss recovery anchor** (backup-vs-delegation fork — the headline open protocol problem).
- **The cooperative legal-review gate** (MO Chapter 351; NOT-LEGAL-ADVICE — carry the reasoning, not
  the citations).
- **The Noria foundation name** (CANDIDATE pending trademark clearance — not decided).
- **The CroftC Phase-0 IP/ownership call** for the app body.
- **The load-bearing-few principles (genome vs strategy)** — whether the compatibility badge is where
  the non-negotiable principles get their teeth.

## State

Themes `02`–`08` are drafted, and the former `01` is now the **Drystone protocol spec** (`drystone-spec/`,
Parts 1 & 2). Built without touching `alpha/` corpus content; verbatim quotes preserved whole with citation
+ verification flag; the user's decisions surfaced, not resolved (the gates are listed below and bannered in
`07` and `08`). The auditable alpha→beta trace and the coverage view of alpha sources not yet pulled up live
in `../alpha/BETA-ROLLUP.md`.

Companion artifacts:

- **`OPEN-THREADS.md`** (this level) — the staging queue at the beta gate: threads being pulled toward
  beta that are not yet settled enough to enter a theme doc (DRAFT / decision-gated / fact-unconfirmed
  material), each with its gates named and a promotion target. Keeps the need from being lost without
  letting it pollute the settled themes. A process artifact, not a theme doc.
- **`../alpha/BETA-ROLLUP.md`** (prior level) — the auditable alpha→beta rollup ledger: per-source
  treatment + landing, the exclusions, and the coverage gap still to close. Reflect on it to confirm
  nothing was missed or pulled up unexpectedly.
- **`../MATURITY-ROLLUP.md`** (discovery root, cross-stage) — the repeatable *method* for maturing a
  stage into the next, with accumulating learnings to recall at the beta→rc transition.

## Next two efforts (flagged, not started here)

- **experiments-beta** — most spikes are still genuinely alpha-maturity; they stay in
  `experiments/alpha/` and keep being built up. Do not graduate yet.
- **Proofs-beta** — a dedicated pass: strip the point-in-time / specific-conversation references in
  the proofs that don't read out of context, and build a comprehensive view of how the proofs relate
  to the discovery themes and the experiment content. Separate effort.
