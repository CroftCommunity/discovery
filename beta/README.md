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

## The synthesis is the layer-cake (themes retired 2026-07-07)

The synthesis was first drafted as eight narrative **themes** (`02`–`08`, plus a `01` that became the
spec). That structure was superseded: a theme cut *across* maturity layers, so a single conclusion ended up
split between layers or duplicated, and the one-home-per-claim invariant could not hold. The corpus has been
**re-filed directly onto the layer-cake** (below) and the theme docs **discarded**. The former `01`
(epistemic foundation) is **Part 1 of the Drystone spec** (`drystone-spec/`). Drystone is the protocol;
Croft is one ecosystem built on it (see `drystone-spec/README.md`).

The complete source-to-layer trace (every alpha and theme source, its treatment, and the conclusion-
coverage gate) is `../alpha/LAYER-ROLLUP.md`. The theme-keyed `../alpha/BETA-ROLLUP.md` is retained frozen
as the historical record of the theme era.

## The layer-cake (emerging structure, populated across sessions)

Alongside the theme docs, beta is being organized into a **maturity layer-cake**: the themes migrate into
layers as they settle. The ordering runs from the "why" that grounds everything, up through the build, out
to the edges. The canonical layer model, the two traversals, and the register distinctions live in
**`LAYERS.md`**; this table is the index. Layer directories are seeded as material arrives, not all at once:

| Layer | Dir | What it holds | State |
|---|---|---|---|
| 1 | `history/` | MATERIAL history: crofting, dry-stone, cairns, the space itself, the enclosure story | **seeded 2026-07-07** (crofting/dry-stone/enclosure-inversion, folded from theme 02 + alpha) |
| 2 | `philosophy/` | INTELLECTUAL history: the principles + thinkers, the pure peer-standing argument (+ `prior-art/`) | **seeded** (peer-standing set; Modular Politics prior-art) |
| 3 | `cairn/` | THE FIELD (open half): the existing bolstering tech we build among (iroh, MLS, Willow/Meadowcap, CBOR-DAG, atproto/AT, ActivityPub, CRDT, QUIC; products Roomy, Blacksky, p2panda, SimpleX, Matrix). What we credit and reuse. | **seeded 2026-07-07** (atproto-ecosystem + social-lexicon research brief; more to migrate) |
| 3′ | `fenced/` | THE FIELD (fenced half): the centered commercial platforms (Telegram, Discord, WhatsApp, Signal, Slack, Teams, Reddit, X, iMessage, Messenger, LINE, WeChat). Roster/call/broadcast ceilings, E2EE stance, per-group rates, economics. A descriptive map (no argument); feeds the spec (§11.9.1, §11.13) and activism. | **seeded 2026-07-07** (batch eleven: capability map + operational-rates/economics) |
| 4 | `drystone-spec/` | Protocol spec (Parts 1+2, persona-definition, CHANGELOG, open-items, feasibility review, §11 (large-group scaling)) | populated |
| 5 | `impl/` | Reference implementation, experiment-informed (themes 04/05/06 to migrate) | **seeded** (`delivery-layer/` + `mls/` bundles + shared `doc-writing-method.md`) |
| 6 | `croft/` | Product thinking, Croft as a "flavor" on the neutral core: the garden of ponds/pads, the social-graph-as-substrate reframe | **seeded 2026-07-07** (product-shape + social-graph-as-substrate, folded from theme 08 + alpha) |
| 7 | `governance/` | The manifestation: foundation + cooperative, legal/financial actualization | **seeded 2026-07-07** (foundation/co-op + IP-stewardship + preventative-work, folded from theme 07 + alpha) |
| 8 | `socialization/` | Brand / voice / adoption, *presentation* register (T4 / T11 terrain) | **seeded** (human-facing peer-standing pieces) |
| 9 | `activism/` | The case against the status quo: why incumbent platforms are harmful, *empirical* register | **seeded** (the "platforms author the relational field" research set) |

**The ordering.** The "why" sits at the base (Layers 1–2: history = why it resonates, philosophy = why it
is right), then the field survey in two halves (3 cairn = the open tech we build among; 3′ fenced = the
centered platforms we are an alternative to), because the spec is built *on* the principles and the
surveyed field. Then the build (4 spec → 5 impl → 6 croft → 7 governance), then the outward edges (8
socialization = get the message out; 9 activism = why not the status quo, the *present* "why"). **The
field-and-response triad:** cairn is the inverse of activism in valence (credit vs indictment); fenced is
the neutral descriptive map both are drawn against, and activism reads its harm case off the fenced half.
Read `LAYERS.md` for the triad and the build-order vs justification-order traversals.

The layer dirs are now the sole structure (the theme docs are retired); each layer carries its own
`README.md` index. The peer-standing companion work (2026-07-06) was **split by register**: the pure
argument → `philosophy/` (Layer 2), the empirical harm case → `activism/` (Layer 9), the human-facing
presentation → `socialization/` (Layer 8); `governance/` (Layer 7) holds the *manifestation* of that
argument (foundation + co-op), not the argument itself. The Drystone spec's Part 1 §2.6 (voice
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

The corpus is now filed on the layer-cake; the narrative themes (`02`–`08`) were re-filed into layers and
discarded (2026-07-07), and the former `01` is the **Drystone protocol spec** (`drystone-spec/`, Parts 1 &
2). Built without touching `alpha/` corpus content; verbatim quotes preserved whole with citation +
verification flag; the user's decisions surfaced, not resolved (the gates are listed below). The auditable
source→layer trace and the conclusion-coverage gate live in `../alpha/LAYER-ROLLUP.md` (the theme-keyed
`../alpha/BETA-ROLLUP.md` is retained frozen as the historical record).

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
