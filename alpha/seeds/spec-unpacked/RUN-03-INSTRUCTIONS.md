# RUN-03 — Continuity decoupling, reconciliation horizon, and the competing-RuleChange predicate

`Two phases. Phase A: markdown surgery only. Phase B: one scoped code change (the F8 implementation
gap), test-first. Separate commits per phase. Branch: fresh off main, e.g. run-03-horizon-and-continuity.`

## Context

- The 2026-07-14 merge already landed RUN-01, RUN-02 (F1–F8), the adjudication-language pass (the DR
  block), and the Map relocation. This run lands the design decisions made after that merge.
- Key inputs: `alpha/thinking/the-shape-of-disagreement.md` (the Layer-1/Layer-2 design dialogue),
  the RUN-01 EXP-4 refutation (register row `competing-quorum-autoresolve`: the fold currently
  auto-resolves competing quorum-met RuleChanges order-dependently), and Part 2 as merged.
- Decisions this run encodes (the owner's calls — do not re-open):
  1. **Projection is render-over-re-plant, not a new protocol mode.** At the protocol layer every
     fork IS a re-plant (MLS absorbs members or instantiates fresh groups; never merges; always an
     epoch change). None of that binds a member's attachment to the Group object, which lives in the
     lineage. Part 2 gets an explicit continuity/decoupling passage.
  2. **Reconciliation horizon**: Layer-1 contradiction detection is continuous; Layer-2 (projection)
     evaluation is cadenced. The cadence is a genesis seed rule combining the two log-derivable
     streams — fires on every epoch roll, and additionally whenever N facts accumulate since the
     last horizon (counter resets at each boundary; N is a per-group temperament dial; no wall-clock
     anywhere). A horizon checkpoint is the §7.3.3 self-checkpoint extended with a manifest of open
     contradiction byte-heads; co-signing keeps §7.3.3 semantics (corroboration of independent
     identical folds, never a trusted summary). A contradiction never expires: horizons bound
     presentation staleness, never resolution.
  3. **The competing-RuleChange contradiction predicate is un-gated implementation** (F8 decided the
     spec; EXP-4 confirmed the gap). Narrowest form: two concurrent admitted RuleChanges on the same
     rule with differing values contradict, surfaced identically to mutual expulsion.

## Language guardrail (applies to every Part 2 insert)

Follow the DR block in `conventions-and-decisions.md`: continuity-framed, non-moral language. No
right/wrong wording about members' choices or narratives; motivation is unknowable from provenance.

---

# Phase A — documents

## A1 — Part 2: the continuity/decoupling passage (§7.6)

Anchor: the §7.6 paragraph beginning "A fork is not a distinct mechanism from a heal or a routine
re-key; all three are the **same operation at different arity**" (contains "atomically repoint the
conversation to it"). Insert the following as a new paragraph immediately after that paragraph:

> **Continuity lives in the lineage, never in an MLS instance.** The object a member is attached to
> is the Group: the lineage rooted at genesis (§5.10), whose trunk no later fork can rewrite. The MLS
> groups underneath it are disposable enforcement vessels — re-plant instantiates a fresh one and
> atomically repoints, and the Group persists across any number of such instances exactly as it
> persists across membership turnover. Two decouplings follow and are normative for how the layers
> may be read. First, **an epoch roll carries no inherent social meaning**: a re-key driven by
> routine churn and a re-key driven by a removal are the same protocol event, and nothing about a
> member's standing may be inferred from the bare fact of an epoch change — standing is read from
> the decision plane (§7.3.5), never from the enforcement plane. Second, **membership history
> composes over gaps**: a member who departs and later rejoins the same lineage holds one continuous
> relationship with the Group object, rendered as two spans with a gap between them, and §5.11's
> read-scoped content keys make the gap substantive rather than presentational, since content sealed
> for intervals of non-membership stays unreadable. A member's rendered narrative of the Group is
> therefore a view over the shared fact field, and two members holding different narratives over
> identical facts is expected operation, not divergence: divergence is a property of folded state
> (§7.3), never of renders. `Design`, decided; the render layer itself is out of protocol scope, and
> the design exploration is captured in `alpha/thinking/the-shape-of-disagreement.md` and
> `alpha/thinking/reconciliation-horizon.md`.

Render as a normal Part 2 paragraph (bold lead, house em-dash style), not a blockquote.

## A2 — Part 2: the horizon-cadence worked example

Anchor: the paragraph beginning "There is no single correct configuration, because respecting
variety and canonical local state means different groups genuinely want different thresholds."
Insert the following as a new paragraph immediately after it:

> *Worked example: the reconciliation-horizon cadence.* A Group **MAY** set, at genesis and
> thereafter under R7 like any rule, a **horizon cadence**: a boundary at which each member
> re-evaluates its open contradictions and **MAY** record a **horizon checkpoint** — the §7.3.3
> self-checkpoint extended with a manifest of the contradiction byte-heads (§7.6.1) open at that
> frontier, co-signable with unchanged §7.3.3 semantics (a co-signature is corroboration of an
> independent identical fold, never a trusted summary). The cadence composes the two log-derivable
> event streams: it fires on every epoch commit — the one serialized, totally ordered event stream
> in the system, so every member locates the boundary identically with nothing asserted — and
> additionally whenever N facts accumulate since the last horizon, which bounds backlog in a
> socially quiet Group where no epoch rolls; the counter resets at each boundary so the two terms
> cannot drift. N is a temperament dial in exactly this section's sense: small for a tight
> operational Group, large for a loose social one, and safe at every setting because the horizon
> gates nothing — an open contradiction remains open truth regardless of how it is presented, and no
> cadence can expire it. No wall-clock appears in either term. `Design` (a composition of §7.3.3
> checkpoints, the §7.4 generation stamp as the since-boundary locator, and epoch serialization; the
> manifest's byte-level encoding joins the other `[gates-release]` items, Appendix B; full
> treatment: `alpha/thinking/reconciliation-horizon.md`).

Render as a normal paragraph in house style. Add the manifest encoding as one line in Appendix B's
`[gates-release]` list, alongside the existing checkpoint-encoding item.

## A3 — New design note: `alpha/thinking/reconciliation-horizon.md`

Create the file with this content (keep the voice consistent with `the-shape-of-disagreement.md`;
a captured design outcome, not committed spec):

- Banner: companion to `the-shape-of-disagreement.md`; captured 2026-07-14; concrete landings are
  the two Part 2 `Design` paragraphs (A1, A2) and backlog item EXP-H1; everything else exploratory.
- **Motivation.** Re-evaluating a projection over ever-longer history grows without bound; the
  release valve must be the closest-to-consensus fact available with no trusted assertions: a
  boundary every member derives identically from the shared log.
- **The split.** Layer-1 detection is continuous (every concurrent conflict freezes into a
  descriptor the moment the fold sees it); Layer-2 evaluation is cadenced. At a horizon, a persona
  re-runs its policy only against forks arrived since the last horizon: no match against your world,
  nothing changes (as-if-under-your-feet); a match projects silently; `escalate` or no-match is what
  gets presented, at bounded age.
- **The cadence rule** (as in A2): epoch-roll OR N-facts, counter resets at each boundary, both
  terms log-derivable, R7-governed, per-group N.
- **The horizon checkpoint**: §7.3.3 self-checkpoint + manifest `(frontier head, sorted set of open
  contradiction byte-heads)`. Entirely objective; co-signing corroborates a shared disagreement
  landscape without revealing any policy.
- **The projection checkpoint**: a member's rendered state cached as of the horizon frontier, a
  §4.6 derived view (rebuildable, non-authoritative); evaluation thereafter incremental in the
  window. The one full-replay case is editing one's own resolution policy: rare, user-initiated,
  always possible via the genesis floor. Checkpoints accelerate, never replace.
- **Decay, not expiry.** Open descriptors are Layer-1 truth and never expire (expiry would be a
  verdict by timeout). Presentation can age an untouched fork out of the working set into an
  archive shelf after some horizons; the descriptor and the pressure gauge stay accurate.
- **Temperature without profiling.** Policies stay private (the shape doc's profiling floor), but
  which branch a persona builds on is already public in the DAG by construction, so each horizon
  yields the observed convergence fraction per fork — the coherence temperature — with the rule that
  produced each choice staying unguessable-with-certainty.
- **First spike (EXP-H1).** Two members, one contradiction (mutual expulsion works today), drive a
  horizon boundary in both trigger modes (epoch roll; N-facts), assert byte-identical manifests
  across arrival orders. Competing-RuleChange joins the manifest once the Phase B predicate lands.

## A4 — Shape-doc resolution note

In `alpha/thinking/the-shape-of-disagreement.md` §10, immediately after the existing
"**Note (RUN-02 F8).**" blockquote, add a sibling blockquote:

> **Note (2026-07-14, continuity resolution).** The §7.6-versus-projection tension is resolved as
> *render over re-plant*, not a new protocol mode: at the protocol layer every fork IS a re-plant
> (MLS absorbs members or instantiates fresh groups; there is no merge; there is always an epoch
> change), and none of that binds a member's attachment to the Group object, which lives in the
> lineage. Even `multi_home` reduces cleanly: membership in both re-planted groups — two ordinary
> groups sharing an immutable prefix. Part 2 now carries the decoupling explicitly (the §7.6
> continuity passage), and the Layer-2 evaluation cadence is designed in
> `reconciliation-horizon.md`. Steps 3–5 below remain the design frontier.

## A5 — Backlog

In `alpha/experiments/EXPERIMENT-BACKLOG.md` §2 (or §2a if it reads better there), add **EXP-H1 —
horizon-manifest determinism** per A3's first-spike description, noting: runnable today against the
mutual-expulsion contradiction; extends to competing-RuleChange after Phase B; the manifest is
`(frontier head, sorted open byte-heads)` and the assertion is byte-identity across members and
arrival orders. Cross-reference `reconciliation-horizon.md` and the A2 spec paragraph.

## A6 — Rule 15 and changelogs

Update Part 2's back `## 0. Map` for §7.6 (and Appendix B). Append a `part-2-changelog.md` entry in
house style covering A1, A2, and the Appendix B line.

## A7 — Reviews-and-experiments log

Append `## 2026-07-14, Continuity decoupling and reconciliation-horizon design pass` covering: the
render-over-re-plant resolution, the two Part 2 `Design` paragraphs, the new thinking note, EXP-H1,
and (if Phase B runs) the predicate landing and register-row retirement.

Commit Phase A before starting Phase B.

---

# Phase B — the competing-RuleChange contradiction predicate (F8 implementation gap)

Scope: close register row `competing-quorum-autoresolve`. The refutation pin in
`croft-chat/croft-chat/tests/competing_quorums.rs::two_competing_rulechange_quorums` is the RED.

1. Locate the existing concurrent-contradiction path the fold uses for mutual expulsion and role
   thrash (the `governance.rs` predicate family: `are_concurrent` / `detect_fork` /
   `is_under_determined`, consumed in `fold_derived.rs`). Extend it with the narrowest F8 form: two
   **concurrent, admitted** RuleChange facts on the **same rule_key** with **differing new_value**
   contradict. Same rule_key with the same value concurrent facts are concordant (no contradiction);
   different rule_keys never conflict. Surface exactly as membership contradictions do:
   `contradiction:{byte-head}` with the byte-head as the existing order-independent min-hash.
2. Update the refutation pin to assert the fixed behavior: both fold orders yield the identical
   contradiction status and identical (unchanged, pre-conflict) effective rules; remove the
   SPEC-DELTA comment at the site. Add the two negative cases from step 1 (concordant same-value;
   disjoint rule_keys).
3. Full suite + clippy green across `local_storage_projection` and `croft-chat`.
4. `SPEC-DIVERGENCE-REGISTER.md`: move `competing-quorum-autoresolve` from Active to Reconciled,
   with evidence (the flipped test names, both-orders-identical output) and "Spec: §7.3.2 / §7.6.1
   (F8); landing run: RUN-03."
5. `EXPERIMENT-BACKLOG.md` §2a: mark the impl gap closed; leave per-act approver-role granularity
   open; note EXP-H1 can now include a competing-RuleChange manifest entry.

**Phase B stop rules.** Stop and report instead of improvising if the predicate would require:
touching wire/envelope encodings; changing the surfaced-status format beyond adding the new case;
edits outside `governance.rs` / `fold_derived.rs` / the named tests; or resolving any design
question not decided above (e.g. broader same-subject conflict shapes — narrowest form only).

---

# Guardrails (both phases)

- Verbatim-anchor rule: if any anchor text above is not found exactly, stop that task, record the
  miss, continue with the rest. Do not guess a nearby location.
- House style for Part 2 inserts (em-dash usage, status-tag placement, MUST/MAY casing); DR language
  rules for A1/A2; minimal diffs, no reflowing untouched paragraphs.
- No edits to Part 1, `conventions-and-decisions.md`, or any crate not named in Phase B.

# Output

`alpha/experiments/RUN-03-SUMMARY.md`: per-task status, placement judgments, anchor misses, Phase B
test output for the flipped pin (both orders), and the full file list.
