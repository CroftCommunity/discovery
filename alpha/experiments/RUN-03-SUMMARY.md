# RUN-03 — Continuity decoupling, reconciliation horizon, and the competing-RuleChange predicate

`Branch: claude/run-03-horizon-continuity-nrs3to · captured 2026-07-14`

Two phases, separate commits. Phase A: markdown surgery only (the two post-merge design decisions).
Phase B: one scoped code change (the F8 implementation gap), test-first.

---

## Per-task status

### Phase A — documents

| Task | Status | Notes |
|---|---|---|
| **A1** — §7.6 continuity/decoupling passage | ✅ done | Inserted as a normal Part 2 paragraph (bold lead, house em-dash), immediately after the §7.6.2 "same operation at different arity" anchor paragraph. |
| **A2** — §7.6.9 horizon-cadence worked example + Appendix B line | ✅ done | Worked-example paragraph inserted immediately after the "There is no single correct configuration" anchor. Manifest encoding added to Appendix B's `[gates-release]` wire-encoding list, alongside the returning-member checkpoint-encoding item. |
| **A3** — `alpha/thinking/reconciliation-horizon.md` | ✅ done | New design note created; banner + all eight outline sections (motivation, the split, cadence rule, horizon checkpoint, projection checkpoint, decay-not-expiry, temperature-without-profiling, EXP-H1). |
| **A4** — shape-doc §10 resolution note | ✅ done | Sibling blockquote added immediately after the existing "**Note (RUN-02 F8).**" blockquote. |
| **A5** — backlog EXP-H1 | ✅ done | Added as §2b (reads best in the §2 governance-fold cluster). |
| **A6** — Map + changelogs | ✅ done | §0 Map §7.6 line updated (§7.6.2 continuity/decoupling; §7.6.9 horizon cadence) and Appendix B Map line updated; `part-2-changelog.md` entry appended (A1, A2, Appendix B). |
| **A7** — reviews-and-experiments log | ✅ done | `## 2026-07-14, Continuity decoupling and reconciliation-horizon design pass` appended to `beta/impl/experiments/drystone-reviews-and-experiments-log.md`. |

### Phase B — the competing-RuleChange contradiction predicate

| Step | Status | Notes |
|---|---|---|
| **B1** — extend the predicate family | ✅ done | `fold_derived::detect_competing_rulechange` + `rulechange_target` helper; hooked into the authorized branch of `DerivedFold::ingest`. Narrowest F8 form. |
| **B2** — flip the refutation pin; add negatives | ✅ done | `two_competing_rulechange_quorums` now asserts the hard-stop (both orders identical); SPEC-DELTA comment removed; two negative cases added. |
| **B3** — full suite + clippy green | ✅ done | `local_storage_projection` and `croft-chat` suites green; clippy introduces **zero** new warnings vs. baseline (28 lib warnings both with and without the change — all pre-existing infrastructure). |
| **B4** — register row Active → Reconciled | ✅ done | `competing-quorum-autoresolve` moved with evidence and "Spec: §7.3.2 / §7.6.1 (F8); landing run: RUN-03." |
| **B5** — backlog §2a impl gap closed | ✅ done | RESOLVED banner added; per-act approver-role granularity left open; EXP-H1 competing-RuleChange manifest note added. |

---

## Placement judgments

- **A1** sits between the §7.6.2 intro paragraph and its three-arity bullet list, exactly where the
  anchor instruction places it ("immediately after that paragraph"). The bullets that elaborate the
  three arities still read cleanly after the inserted decoupling paragraph.
- **A2** is a `*Worked example:*`-led paragraph in the temperament-dial section (§7.6.9), directly
  after the "no single correct configuration" paragraph it illustrates.
- **Appendix B manifest line** was added as a new clause inside the single `[gates-release]` wire
  encodings bullet, immediately after the returning-member checkpoint-encoding clause — the closest
  literal reading of "one line … alongside the existing checkpoint-encoding item," since that list is
  a semicolon-separated enumeration in one bullet rather than a per-line list.
- **A5 (EXP-H1)** placed as backlog **§2b** rather than folded into §2a: §2a is specifically "Fold
  findings from RUN-01 EXP-4," whereas EXP-H1 is a new experiment, so a sibling subsection in the
  same §2 governance-fold cluster reads better.
- **Changelog** entry uses hyphens, not em-dashes, matching the established `part-2-changelog.md`
  house convention (the Part 2 *body* inserts use em-dashes, matching the body).

## Anchor misses

**None.** Every verbatim anchor was found exactly:

- A1: "A fork is not a distinct mechanism from a heal or a routine re-key; all three are the **same
  operation at different arity**" / "atomically repoint the conversation to it" — found (§7.6.2).
- A2: "There is no single correct configuration, because respecting variety and canonical local
  state means different groups genuinely want different thresholds." — found (§7.6.9).
- A4: "**Note (RUN-02 F8).**" blockquote — found (shape-doc §10).
- Appendix B: the existing checkpoint-encoding item ("returning-member `(G, D)` cursor and checkpoint
  encoding") — found.

One naming note (not a miss): the inserted passages cross-reference the contradiction byte-heads to
**§7.6.1** (the escalation-shapes subsection, whose Contradiction shape is their home), per the
verbatim insert text; the §0 Map anchors the horizon-cadence example under §7.6.9 where it lives.

---

## Phase B — test output (the flipped pin, both orders)

Command: `cargo test -p croft-chat --test competing_quorums -- --nocapture`

```
COMPETING-QUORUMS: order1 -> add_member_threshold=1 fork="contradiction:5680676b5d1dfe002f11a013b47f086d7118f7bf7871cb3e30f8e006d9290ab8"; order2 -> add_member_threshold=1 fork="contradiction:5680676b5d1dfe002f11a013b47f086d7118f7bf7871cb3e30f8e006d9290ab8"
COMPETING-QUORUMS: order_dependent=false hard_stopped=true
CONCORDANT: order1 -> add_member_threshold=5 fork="clean"; order2 -> add_member_threshold=5 fork="clean"
DISJOINT: order1 -> add=5 remove=7 fork="clean"; order2 -> add=5 remove=7 fork="clean"
BYTE-HEAD: expected=contradiction:40bfbe59…; order1=…; order2=…   (unchanged, still order-independent)

test contradicted_group_byte_head_is_min_hash_order_independent ... ok
test concurrent_disjoint_rulekey_rulechanges_do_not_conflict ... ok
test concurrent_same_value_rulechanges_are_concordant ... ok
test two_competing_rulechange_quorums ... ok
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

**Both orders identical:** `add_member_threshold=1` (the pre-conflict genesis value, unchanged — neither
contested value 5 or 9 wins) and byte-identical `contradiction:5680676b…` status. Order-independence
restored; the silent I5 violation is gone.

Negative cases prove the predicate does not over-trip: same-rule-same-value stays concordant (`fork=clean`,
value applies), and disjoint rule_keys commute (`fork=clean`, both rules apply).

Full-suite result: **green.** `local_storage_projection` — **97 passed, 0 failed** (`cargo test`, exit 0),
which covers `governance.rs::test_i7_contested_change_divergent_lineages` (unaffected: it folds a single
RuleChange per store and both its facts carry empty antecedents, which the predicate's positively-establish-
concurrency guard skips). `croft-chat` — every test binary `ok`, **0 failed** across all binaries
(`competing_quorums` = 4 passed), `cargo test` exit 0. Clippy green on both crates (`cargo clippy
--all-targets`), with **zero new warnings** vs. baseline on `local_storage_projection` (28 pre-existing lib
warnings unchanged with/without the change).

---

## Stop-rule check (Phase B)

None triggered. The change touched only `local_storage_projection/src/fold_derived.rs` (the
`governance.rs` predicate family it consumes) and `croft-chat/.../tests/competing_quorums.rs`. No
wire/envelope encoding touched; the surfaced-status format gained the new case only by reusing the
existing `contradiction:{byte-head}` shape (no format change); no design question beyond the
owner-decided narrowest F8 form was resolved.

---

## Full file list

**Phase A commit (`71685c0`):**

- `beta/drystone-spec/part-2-certifiable-design.md` — A1 (§7.6.2 passage), A2 (§7.6.9 worked example),
  Appendix B `[gates-release]` line, §0 Map (§7.6 + Appendix B).
- `beta/drystone-spec/part-2-changelog.md` — new pass entry (A1, A2, Appendix B).
- `alpha/thinking/reconciliation-horizon.md` — **new** design note (A3).
- `alpha/thinking/the-shape-of-disagreement.md` — §10 continuity-resolution note (A4).
- `alpha/experiments/EXPERIMENT-BACKLOG.md` — EXP-H1 §2b (A5).
- `beta/impl/experiments/drystone-reviews-and-experiments-log.md` — 2026-07-14 design-pass section (A7).

**Phase B commit:**

- `alpha/experiments/local_storage_projection/src/fold_derived.rs` — `detect_competing_rulechange`
  + `rulechange_target`; predicate hooked into the authorized branch of `ingest`.
- `alpha/experiments/croft-chat/croft-chat/tests/competing_quorums.rs` — flipped pin + two negatives.
- `alpha/experiments/SPEC-DIVERGENCE-REGISTER.md` — `competing-quorum-autoresolve` Active → Reconciled.
- `alpha/experiments/EXPERIMENT-BACKLOG.md` — §2a RESOLVED banner + §2 table maturity update.
- `beta/impl/experiments/drystone-reviews-and-experiments-log.md` — Phase B paragraph finalized.

---

## Guardrail compliance

- No edits to Part 1, `conventions-and-decisions.md`, or any crate not named in Phase B.
- DR language (conventions A.11): the A1/A2 inserts are continuity-framed and non-moral.
- House style: em-dashes in the Part 2 body inserts; MUST/MAY casing preserved; minimal diffs, no
  reflowing of untouched paragraphs.
