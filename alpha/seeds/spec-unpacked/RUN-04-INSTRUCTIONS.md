# RUN-04 — The corroboration dials: quantified trust, the stamp beacon, and the beam contract

`Markdown surgery only — no code, no tests. Branch: fresh off main, e.g. run-04-corroboration-dials.`

## Sequencing gate

This run assumes **RUN-03 is merged to main** (it cross-references
`alpha/thinking/reconciliation-horizon.md` and places a backlog item beside EXP-H1). If
`reconciliation-horizon.md` or the EXP-H1 backlog entry is absent, STOP and report — do not
improvise the references.

## Context (the owner's framing this run encodes — do not re-open)

1. **Epistemic floor.** A local authority can never know more than assertion from outside itself
   (§5.1: comparative or asserted, never canonical). External information is taken at *quantified
   trust*, so the node must be configured with the thresholds and terms for sufficiency.
2. **Read/write split.** Solicitation is the read side: actively asking peers for their frontiers,
   the only reach into the unreferenced tail, and its answers are always assertions. On write, the
   local authority DOES know the last event it asserted on top of, and every outgoing entry already
   references it: the §7.4.3 generation stamp is the passive "I'm good, you?" beacon on the content
   plane.
3. **Formula-valued thresholds.** The freshness k may be a formula over folded state (proportional
   to member count, role-weighted), deterministic because every input is folded fact.
4. **Circular assertion awareness.** Seeing one's own stamp echoed back in others' entries is
   passive propagation evidence — positive-evidence-only (silence is indistinguishable from
   partition), and strictly a liveness signal, never an ordering or authority input.
5. **The beam reframed.** Completeness-ahead is an intrinsic limit plus a family of Group-governed
   dials (which acts require final state; the k or formula; the solicitation posture), not a
   mechanism awaiting invention. The fail-closed rule (delay over breach) is NOT a dial.

## Language guardrail

Part 2 inserts follow the DR block in `conventions-and-decisions.md`: continuity-framed, non-moral;
house em-dash style, status-tag placement, MUST/MAY casing.

---

## T1 — New design note: `alpha/thinking/corroboration-and-quantified-trust.md`

Create the file, voice-consistent with its siblings (`the-shape-of-disagreement.md`,
`reconciliation-horizon.md`): a captured design outcome, not committed spec. Banner: captured
2026-07-14; concrete landings are the two Part 2 `Design` paragraphs (T2, T3) and backlog item
EXP-C1; the self-echo idea is exploratory. Sections:

- **0. The epistemic floor.** §5.1 restated: everything external is comparative or asserted, never
  canonical; therefore all corroboration is quantified trust, and the node's job is not to *know*
  the edge but to be configured with the terms under which corroboration counts as sufficient.
- **1. The write side is already built.** The §7.4.3 generation stamp: every data-plane entry
  self-locates against the authority chain, so every ordinary message doubles as a passive currency
  beacon — the "I'm good, you?" — and R6's attributable acceptance is its enforcement-side twin
  (record the frontier you had synced when you accepted). What this closes: the behind-via-traffic
  case, converted from undetectable to a sized, named gap.
- **2. The read side: solicitation and the unreferenced tail.** A governance fact nothing has yet
  stamped cannot be beaconed by definition; only an active frontier ask reaches it, and the answer
  arrives as assertion at quantified trust. Absence of evidence is only ever corroborated absence.
- **3. Formula-valued thresholds.** k as f(folded member count, folded roles) at the act's position;
  sound iff every input is folded fact (never asserted or locally observed), so k is deterministic
  and identical everywhere; changes under R7; moves the dial, not the machinery.
- **4. Circular assertion awareness (exploratory).** Your own stamp echoed back in others' entries
  is delivery corroboration without receipts. Two seams, stated hard: positive-evidence-only
  (no echo means nothing — silence and partition are indistinguishable), and liveness-signal-only —
  the moment an echo influences ordering or authority it becomes a covert clock, which Part 1
  §2.0.1 forbids. For anyone else it may mean nothing; that is fine.
- **5. The beam, reframed.** Quote-free restatement of Part 2's own posture (the "load-bearing
  caveat" paragraph): final state is established only by corroboration reaching the node;
  enforcement fails closed; delay over breach. Therefore what remains of the beam is a dial family
  (which acts need final vs best-known; the k or formula; solicitation posture) plus one intrinsic,
  honest limit. Earning the beam means demonstrating the *contract*, not eliminating the limit.
- **6. The contract experiment.** Point to EXP-C1 (T4) with its four assertions.

## T2 — Part 2: the corroboration-dials paragraph

Anchor: the paragraph beginning "The load-bearing caveat, stated rather than claimed away: a
sufficiently isolated node **cannot** establish final state on its own". Insert the following as a
new paragraph immediately after it:

> **The residual is governed, not open-ended: the corroboration dials.** What remains of
> completeness-ahead once the §7.4.3 stamp closes the behind-via-traffic case is not an undesigned
> mechanism but a family of Group-governed settings, set where every other rule is set and changed
> under §7.2 R7: (i) **which act classes require final state** versus proceed on best-known —
> irreversible enforcement always requires final, reads never do, and the boundary between them is
> per-Group; (ii) **the freshness threshold itself**, the k distinct lineages whose corroboration of
> a frontier head constitutes currency (§7.4), including its formula-valued form (§7.4.1); and
> (iii) **the solicitation posture**, how actively a node asks peers for their frontiers to reach
> the unreferenced tail no stamp can announce — a read-side ask whose answer is always an assertion
> taken at quantified trust, never proof. A tight Group dials k high and enforcement slow; a loose
> Group dials it low and accepts more exposure to delay — the same temperament spectrum as the
> configuration posture of §7.6, and safe at every setting because the fail-closed rule above is
> not a dial. `Design`, decided as the framing; the demonstration contract is carried in the
> backlog (EXP-C1), and the exploration in
> `alpha/thinking/corroboration-and-quantified-trust.md`.

Render as a normal Part 2 paragraph (not a blockquote).

## T3 — Part 2: the formula-valued threshold paragraph (§7.4.1)

Anchor: section `#### 7.4.1. The false-positive tolerance is a governed utility judgment, not a
constant`. Insert the following as a new final paragraph of §7.4.1 (immediately before the §7.4.2
heading):

> The threshold's *value* admits the same governance treatment as its tolerance: a Group **MAY**
> set k as a formula over folded state rather than a constant — proportional to the folded member
> count at the act's position, or weighted by folded Group Roles — provided every input to the
> formula is itself folded fact, never an asserted or locally observed quantity, so the resulting k
> is deterministic and identical on every honest node exactly as a constant would be. A
> formula-valued threshold changes under R7 like any rule and introduces no new trust surface: it
> moves the dial, not the machinery. `Design.`

## T4 — Backlog: EXP-C1, the beam contract

In `alpha/experiments/EXPERIMENT-BACKLOG.md`, beside EXP-H1, add **EXP-C1 — the completeness-ahead
contract (loopback, runnable now, no new infra)**. Four assertions, each RED-able:

1. **Stall-at-threshold (delay over breach).** Withhold one governance fact from node X; X's
   attempted enforcement of a dependent irreversible act stalls below freshness threshold k while X
   continues reads on best-known state. No breach, no stall of reads.
2. **Stamp detection.** X receives a data-plane entry whose generation stamp is ahead of X's
   frontier; the gap is detected, sized, and named, and X fills it before acting (the
   behind-via-traffic case, demonstrated end to end).
3. **Solicitation reach.** The withheld fact is stamped by nothing (the unreferenced tail); X's
   frontier ask to any peer holding it surfaces it; the fold then admits it identically to normal
   arrival.
4. **Formula-valued k.** With k = ceil(n/2) over the folded member set, every node computes the
   identical k at the same act position across arrival orders.

Note in the entry: shares boundary machinery with EXP-H1; discharges, at loopback grade, part of
§8.2(e)'s residual that "the freshness precondition on originating such an op (§7.4.2) is not yet
exercised over live transport."

## T5 — Rule 15 and changelogs

Update Part 2's back `## 0. Map` for §7.3 (or wherever the beam paragraph's section indexes) and
§7.4. Append a `part-2-changelog.md` entry in house style covering T2 and T3.

## T6 — Reviews-and-experiments log

Append `## 2026-07-14, Corroboration dials and the beam contract (design pass)` covering: the
quantified-trust framing, the stamp-as-beacon reading, the two Part 2 `Design` paragraphs, the
self-echo exploration (with its two seams), and EXP-C1.

---

## Guardrails

- Verbatim-anchor rule: if any anchor text above is not found exactly, stop that task, record the
  miss, continue with the rest. Do not guess a nearby location.
- Markdown only. No Part 1 edits, no `conventions-and-decisions.md` edits, no code.
- Minimal diffs; no reflowing untouched paragraphs.

## Output

`alpha/experiments/RUN-04-SUMMARY.md`: per-task status, placement judgments, anchor misses, and the
full file list.
