# Handoff prompt — the "left-behind" audit (a finer-grained, whole-corpus double-check)

*Created 2026-07-10. Copy the block below into a fresh Claude Code session (or hand it to an agent) from the
`discovery/` working directory. It is self-contained. It does NOT run the remediation — it produces a
second-pass ledger of what was left behind, for review.*

---

## Mission

Do a comprehensive **left-behind audit** of the Croft discovery corpus: find the **quotes, references,
concepts, ideas, conclusions, corrections, caveats, prior art, analogies, and framings** that exist in the
**alpha** source content (or that are orphaned/under-developed **within beta**) but were **never carried
forward, were dropped, or were flattened** during the alpha→beta maturation. This is a finer-grained,
whole-corpus double-check — read-only, no edits; the deliverable is a categorized ledger for the maintainer.

## Why this pass exists (what the earlier audit did and did NOT do)

A Phase-0 coverage audit already ran (`alpha/plans/2026-07-08-beta-coverage-gap-ledger.md`). It was
**conclusion-focused and transcript-focused**: a 10-cluster fan-out re-read the raw dialogue transcripts and
tagged *load-bearing conclusions* present-in-beta / alpha-only / transcript-only. It found real gaps and much
was then recovered (20 new beta docs). But by construction it **missed**:
- **Granularity** — it hunted *conclusions*, not the finer material: individual **verbatim quotes**,
  **secondary references/bibliographies**, **coined concepts/terms**, **plain-language analogies**,
  **scholarly roots**, **corrections/caveats**, **negative results**, **named movements/precedents**.
- **Whole-corpus reach** — it read the raw *transcripts* but explicitly **excluded** the `pr3`–`pr9` proof
  records, the `-FACTCHECK` companions, and under-mined the non-transcript alpha (`thinking/`, `research/`,
  `narrative/`, `crystallized/`, `SOVEREIGN-COMMONS-DOSSIER.md`, `ECOSYSTEM.md`), plus the `Proofs/` repo.
- **Consolidation completeness** — the recovery docs were written from each conclusion's *one* ledger-named
  home, not the full prior-tier scatter (this is the still-open backlog item **C12**).
- **Beta-internal orphans** — it did not check whether beta docs reference concepts they never define, cite
  sources absent from the reference-index files, or make claims with no reasoning home.

The pattern the rollup exhibited (confirmed last pass): it **kept the decision and dropped the
rationale / prior-art / illustration / present-day arc**. Hunt in that shape.

## Scope — read exhaustively, do not sample

**Alpha source content:**
- `alpha/thinking/` (all, incl. `app/`, subdirs), `alpha/research/`, `alpha/narrative/` (incl. `long-form.md`,
  `short.md`, `verticals/`), `alpha/crystallized/` (principles, conclusions, CROFT-PROTOCOL, proof-ledger,
  test-narrative), `alpha/SOVEREIGN-COMMONS-DOSSIER.md`, `alpha/ECOSYSTEM.md`.
- `alpha/seeds/transcripts/raw/` — **including** the `-FACTCHECK.md` companions (a source of confirmed facts,
  corrections, and REFUTED items) and the `pr3`–`pr9` proof records (excluded last pass).
- `Proofs/` repo (the durable proofs) and `experiments/alpha/` (the code-forward spikes + their plans/READMEs).

**Beta (check for orphans + under-development, and as the "was it carried?" target):**
- every layer dir under `beta/` (history, philosophy incl. prior-art, cairn, fenced, drystone-spec, impl,
  croft, governance, socialization, activism) and each layer's `reference-index.md`; plus `beta/DECISIONS.md`,
  `beta/OPEN-THREADS.md`, `beta/CLOSED-THREADS.md`, `beta/LAYERS.md`.

## What to hunt (the granular kinds — this is the point of the pass)

For each source, surface items in these categories that were left behind or flattened:
1. **Quotes** — verbatim source quotations (named thinkers, RFCs, cases, papers, primary texts) present in
   alpha/transcripts but absent from beta, or **collapsed** into beta narrative prose (folded fragment /
   silent paraphrase) rather than standing with attribution.
2. **References** — any thinker, book, paper, RFC/spec, project, org, URL, DOI cited in alpha but **not in the
   corresponding beta layer's `reference-index.md`** (and, conversely, index entries with no actual use).
3. **Concepts & ideas** — framings, coinages, distinctions, models, sub-arguments raised in alpha that never
   surfaced in beta.
4. **Conclusions** — decisions/findings/lessons the Phase-0 pass may have missed (re-check, don't assume it
   was exhaustive).
5. **Corrections, caveats, negative results, cautions** — the honest limits / "don't do X" / REFUTED items
   (esp. from the FACTCHECK docs) that ground a claim; a claim carried without its caveat is a gap.
6. **Prior art, precedents, existence-proofs, named movements** — the "it's real / it's been done" grounding.
7. **Analogies, metaphors, plain-language devices, scholarly roots** — the "bring folks along" material the
   rollup was prone to drop (e.g. the kind of thing that turned out to be Jo Freeman as the Princeps root, or
   the Rural Electric Cooperative analogy).

## Method (per source)

- Read the source in full. For each hunted item, tag **coverage**: `BETA` (carried — name the doc+line),
  `ALPHA-ONLY` (in alpha synthesis, not beta), `TRANSCRIPT-ONLY` (only in a transcript), or
  `BETA-ORPHAN` (in beta but undefined / unsupported / index-mismatch / no reasoning home).
- For each non-`BETA` item: judge **load-bearing vs incidental** (one line why) and name the **target beta
  layer** it should land in.
- **Cross-check, do not repeat:** before flagging, check whether the Phase-0 gap ledger
  (`alpha/plans/2026-07-08-beta-coverage-gap-ledger.md`) already caught it and whether the recovery docs
  (the T41–T48 cohort + the 12 outer-layer docs) already carry it. Flag only what is genuinely still missing
  or flattened. Note where an item is a **C12 consolidation** case (the concept is in beta but thin because
  only one alpha instance was folded).

## Disciplines the audit's own findings must respect

- **Never fabricate a quote.** If a source's words aren't verbatim-available, say so; mark AI-surfaced quotes
  `[UNVERIFIED, confirm against primary]`. Coined terms of art are *italic*, not quote-marked.
- **RFC-style vs narrative:** the `drystone-spec/` Part 1 & 2 legitimately cite clauses **inline with a
  locator + verification flag** (not block quotes) — that is compliant, not a gap (see `beta/LAYERS.md` →
  "Technical design docs follow RFC-style citation"). Do not flag the spec for not using block quotes.
- **FACTCHECK is the source of truth** for atproto / iroh / iOS facts and for what is REFUTED — cite it, do
  not re-verify; and never resurface a REFUTED item as a gap-to-recover.
- **Anti-rollup:** a decision or claim carried without its reasoning is itself a finding.
- Respect **one-home-per-claim** and **beta tier discipline** when naming targets (no `../alpha/` paths inside
  layer docs; OPEN-THREADS is the exception).

## Output

Write a new ledger: `alpha/plans/2026-07-10-left-behind-audit-ledger.md`, carrying:
- A per-source (or per-cluster) table: `| # | Item (one line) | Kind (quote/ref/concept/conclusion/correction/prior-art/analogy) | Coverage | Evidence (path:line or "absent") | Load-bearing? why | Target beta layer |`.
- A **`### Top left-behind`** section per cluster: the load-bearing ALPHA-ONLY / TRANSCRIPT-ONLY / BETA-ORPHAN
  items, most-critical first, one-sentence why each matters.
- A closing **meta-note**: what Phase 0 structurally missed and why, the C12 consolidation cases found, and a
  recommended remediation list grouped by target beta layer (so a Phase-1b recovery can act on it). Do NOT
  perform the remediation — surface it.

## Scale & fan-out

The corpus is large (~120 alpha files, ~70 transcripts incl. FACTCHECK, 10 beta layers + their
reference-indexes, plus Proofs/ and experiments/). **Fan out** like Phase 0 did — one reader per source
cluster or per beta layer, each returning a structured section, synthesized into the ledger. Use **same-model
subagents** (match the session model; pass `model` explicitly). **Exhaustive, not sampled** — the whole point
of this pass is to catch what a sampled/conclusion-level pass missed.

## Definition of done

Every source in scope read in full; every left-behind item categorized by kind + coverage + load-bearing +
target; beta-orphans and index-mismatches checked; the meta-note + grouped remediation list written; nothing
sampled; the ledger committed (identity: `chasemp` / `chase@owasp.org`; do not push unless asked).
