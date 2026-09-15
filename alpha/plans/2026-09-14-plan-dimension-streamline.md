# Dimension streamline — the convention layer, concise where it can be, one home per why

**Status: PROPOSED 2026-09-14 — awaiting the owner's answers to the Open Questions; no dimension doc is edited until then.**

## Problem Statement

The owner (2026-09-14, after the `/insights` review of 71 sessions): *"refactor our
standard dimension documentation to make it more streamlined, concise where appropriate
and clear."* The layer's own rule already demands this. `PATTERN.md` § Anti-collapse:
*the auto-loaded `CLAUDE.md` summaries stay compressed, and when they outgrow roughly a
screen in total, compress them further rather than letting the canonical docs' weight
migrate up.* Measured today (survey, 2026-09-14, every file read in full):

| Surface | Lines | What it is |
|---|---|---|
| `CLAUDE.md` § Current focus (L53-213) | 161 | dated history 2026-08-17 → 2026-09-14, restating `croft-stack/sessions/*` and croft runbook §15/§16; only L208-213 is orientation |
| `CLAUDE.md` dimension table (L18-43) | 26 | rows 24, 34, 36-42 are 3-8 sentence paragraphs restating their canonical doc |
| `CLAUDE.md` concurrent-sessions summary (L215-253) | 39 | elaborates Rule 1b and Rule 2 rather than compressing them |
| 20 canonical docs | 4,517 | five over 300 lines; `COORDINATION` 727, `SUPPLY-CHAIN` 481 (L323-474 is a rollout log), `LEXICONS` 360, `MOCKS` 348, `TRACKING` 313 |
| `README.md` L69-123 | 55 | one bullet per dimension where `PATTERN.md` step 4 says "a line" |

Every session pays the `CLAUDE.md` cost every turn. The canonical docs are paid on
demand, but four of them carry 30-55 % narrative by line, and the same lesson is told in
up to five places (the dimension roster; the check-number collision; the Rule 1b
incident; the mention-vs-declaration lesson). One of the copies already contradicts its
canonical: `CLAUDE.md:244-245` gives an attribution command that
`COORDINATION.md:475-482` says is wrong.

## Reasoning

**Compress the top, relocate the middle, strike the rest — never delete a why.** Three
moves, in order of payoff:

1. The auto-loaded surfaces (`CLAUDE.md`, `README.md`, the `DECISIONS.md` rows) go back
   to being *indexes*: one line and a pointer per dimension, with at most one
   "most-missed" clause. Everything they say beyond that already exists in a canonical
   doc (verified line-by-line in the survey), except four whys that live **only** in the
   Current-focus narrative — those move to their homes first (ledger below), then the
   narrative is archived, not deleted (`PATTERN.md`: retire by striking).
2. The four oversized canonicals shed their rollout logs and runbooks into the reasoning
   homes that already exist for them (a plan's Review Log; a repo's docs), promoting the
   handful of rules that were only ever stated inside the log.
3. Every remaining restatement collapses to one home plus pointers.

**Alternatives rejected.** *Rewrite every doc for concision* — the docs' length is mostly
narrative-with-a-why, and rewriting sentences risks dropping the why, which is the one
thing the pattern forbids; the survey's per-doc verdicts keep 9 of 21 files untouched.
*Split `COORDINATION.md` into several docs* — confetti docs defeat findability from the
other direction (`PATTERN.md` § split and merge by load); the cheaper cut is removing its
roll-ups and moving one cross-cutting section to the dimension that owns the concern.
*A single big PR* — three phases each leave the layer coherent and each is reviewable in
one sitting; the owner can stop after any of them.

**Why the whys ledger is the plan's spine.** The one way this refactor fails is a rule
losing its reason and being simplified away by the next session. So every relocation is
a row: *from → to*, and Phase done-ness includes grepping each row at its new home.

## Verified Assumptions

- Sizes and narrative shares per doc: survey 2026-09-14, all 22 files read in full
  (table in Problem Statement; per-doc verdicts in Phase 2/3 lists).
- No audit check parses the `CLAUDE.md` dimension table or `README.md`'s bullets —
  `grep -nE 'CLAUDE\.md|Most-missed' .claude/bin/workspace-audit.sh` finds only per-repo
  orientation-doc checks (check 5/6) and prose. Compressing the table cannot move a
  finding.
- `workspace-audit.sh` takes ~4 minutes and reports `RESULT: <n> finding(s)`; the
  live baseline on 2026-09-14 is recorded in the Review Log when Phase 1 starts.
- Whys that live ONLY in text this plan cuts (survey § 5), with their destination:

| From | Why | To |
|---|---|---|
| `CLAUDE.md:176-177` | OAuth sessions die at ~6-11 days idle; re-sign-in is step 0 of every device run | `TESTBED.md` § Devices (both phones) |
| `CLAUDE.md:177-181` | `adb install -r` over the released APK clears app data, the iroh key with it; the Pixel carries a debug build whose record names that identity | `TESTBED.md` Pixel row |
| `CLAUDE.md:119-121` | `admitted sponsorship=` is debug-level and filtered in production; the `usage` line is the bake's instrument | `croft-stack/docs/RUNBOOK.md` (or its TODO decision row, which already tracks the filter) |
| `CLAUDE.md:199-204` | runbook §13 step 3 is no longer needed as written; a staging run is scoped to the wrong-key refusal only | croft `ops/RUNBOOK-two-device-call-test.md` §13 |
| `SUPPLY-CHAIN.md:368-371, 383-385, 386-393, 394-398, 417-421, 441-443` | Gradle classpath regex anchoring; `--retry-all-errors` load-bearing; cross-repo checkout `ref:` defaults to the caller; `requirements.txt` was unenumerated; osv-scanner exit 128/127; `test-*.sh` had no runner | rules 4, § The gate, 10, 6, § The gate, and `VERIFICATION.md` shape 2 respectively |
| `LEXICONS.md:248-252, 326-330` | CNAME/TXT near-miss; DAG-CBOR key reorder breaks byte compare | travel with the runbook to `croft-stack/docs/` beside `bin/publish-lexicons.mjs` |
| `COORDINATION.md:355-361` | "suspect the instrument first" — three sessions, three layers, one day | travels with § Verify-at-the-layer (Q3) |
| `VERIFICATION.md:143-158` | the re-export brace forms are the trap | one sentence kept with the script |
| `PATTERN.md:70-77` | check 36's first draft keyed on the fixing comment | kept (it is the why for "key on the layer that acts, including when checking a check") |
| `DECISIONS.md:170-176` | forage's deliberate divergences from Bluesky client behaviour | forage's ledger, before the row is cut to a pointer |

## Documentation Impact

Phase 1: `CroftC/.claude/CLAUDE.md`, `CroftC/README.md`, `CroftC/.claude/DECISIONS.md`
(rows L45-66), `CroftC/.claude/TESTBED.md` (two whys in), `croft-stack/docs/RUNBOOK.md`
or `TODO.md` (one why in), croft `ops/RUNBOOK-two-device-call-test.md` (one why in), a
new archive file for the Current-focus history (Q2 names where), each canonical doc that
gains a "Most-missed" block (Q4). Phase 2: `COORDINATION.md`, `SUPPLY-CHAIN.md`,
`LEXICONS.md`, `TRACKING.md`, `CHANGELOGS.md`, `VERIFICATION.md`,
`discovery/alpha/plans/2026-08-29-plan-supply-chain-rollout.md` (gains the rollout log),
`croft-stack/docs/` (gains the lexicon publish runbook). Phase 3: `DECISIONS.md`,
`SKINS.md`, `VERIFICATION.md`, `PATTERN.md`, `MOCKS.md`, `CI-PATTERN.md`, `TESTBED.md`,
forage's decision ledger. `DECISIONS.md` row `workspace/pattern` gains the line budget
if Q5 is yes. Grepped: no other file cites the `CLAUDE.md` line numbers being moved.

## Concurrency Map

All phases sequential: each phase rewrites pointers the previous one created, and all
three touch `CLAUDE.md`. Shared state: the CroftC meta-repo worktree
`worktrees/dimension-streamline/CroftC` (to be created at Phase 1 start), one PR per
phase, landing per COORDINATION Rule 2. **Write-set overlap to respect:** `VERIFICATION.md`
is also being edited on `claude/hooks-config-dir` (the 2026-09-14 hook incident); that
PR lands before Phase 2 touches the file.

## Phases

### Phase 1: the auto-loaded surfaces become indexes again
**Goal:** `CLAUDE.md` at roughly one screen of dimension summary plus a short focus block;
`README.md` and `DECISIONS.md` rows at one line each; zero whys lost.
**Changes:**
- [ ] Relocate the four orphan whys (ledger rows 1-4) to their homes — first, in the same PR.
- [ ] `CLAUDE.md` § Current focus → ≤15 lines: state now, next in order, pointers to
      `croft-stack/sessions/*` and the croft runbook; the 161 lines archived per Q2.
- [ ] Dimension table rows → one line + pointer (+ one most-missed clause per Q4); each
      row's elaboration moves into its canonical doc's own "Most-missed" block
      (`DESIGN.md:41-45` is the existing shape).
- [ ] Concurrent-sessions summary → compression only (Rule 1b and Rule 2 whys already in
      `COORDINATION.md:288-298` and `214-243`); fix the attribution command to match
      `COORDINATION.md:472`.
- [ ] `README.md` L69-123 → one line per dimension. `DECISIONS.md` rows L45-66 → one line
      each (the header promises one line).
- [ ] If Q5 is yes: audit NOTE when `CLAUDE.md` exceeds the budget; RED fixture = today's
      file at `c0d1f2c`, landed above the summary line (check 36).
**Call chain:** every session's system prompt → `CroftC/.claude/CLAUDE.md` → the table's
pointer → the canonical doc. The chain is the whole point: the top must say less, and the
pointer must resolve.
**Wiring test:** `bash .claude/bin/workspace-audit.sh` — `RESULT` count unchanged from the
recorded baseline (or each delta named); every ledger row greppable at its destination;
every `→` pointer in the table names a file that exists.
**Depends on:** Q1, Q2, Q4, Q5 answered.
**Read-set / Write-set:** as in Documentation Impact, Phase 1 line.
**Shared-state contract:** one worktree, one branch, no checkout in the shared tree.
**Risks:** a session running mid-landing loaded the old layer — nudge via `SendMessage`
(PATTERN step 6). Cutting a "most-missed" clause that is the only thing a peer cited.
**Done when:** (1) `CLAUDE.md` ≤ ~100 lines with every dimension still one row and every
pointer resolving; a reader of the table can find any rule in two hops. (2) Wiring test
above, output in the Review Log.
**Validation:** moderate — audit green with named deltas, plus one peer session asked to
find three specific rules from the new table and report the hop count.

### Phase 2: the four oversized canonicals shed what is not theirs
**Goal:** `SUPPLY-CHAIN`, `COORDINATION`, `LEXICONS`, `TRACKING` at rules-and-whys, with
logs and runbooks in their reasoning homes.
**Changes:**
- [ ] `SUPPLY-CHAIN.md` L323-474 → the rollout plan's Review Log, keeping the not-met
      table (L466-474) and promoting the six ledger whys into the rules they belong to.
- [ ] `COORDINATION.md`: delete the L708-716 check roll-up (each doc's Maintenance
      section already owns it); fix the double table header L626-629; cut "Whose commit"
      L467-550 to the command plus its three limits; per Q3, move § "Verify at the layer
      that acts" (L332-403, with L355-361) to `VERIFICATION.md` as shape 4.
- [ ] `LEXICONS.md` L216-252 + L278-330 → `croft-stack/docs/` beside
      `publish-lexicons.mjs`, the two near-miss whys travelling with it.
- [ ] Check-number collision lesson: one home (`TRACKING.md:210-218`); `SUPPLY-CHAIN.md:
      316-321` (stale — says "allocate from a fresh git log" where the tool is
      `next-id.sh check`) and `CHANGELOGS.md:240-246` become pointers.
**Wiring test:** audit `RESULT` unchanged; every moved section greppable at its
destination by its heading; `bin/test-*.sh` all green (some read these docs).
**Depends on:** Phase 1 landed; `claude/hooks-config-dir` landed (VERIFICATION.md overlap).
**Risks:** `SUPPLY-CHAIN` rules 4/6/10 grow — keep each promoted why to two lines.
**Done when:** the four docs each under ~250 lines with no rule removed (diff reviewed
rule by rule against the survey's rule counts); wiring test in the Review Log.
**Validation:** moderate — audit plus a rule-by-rule diff read.

### Phase 3: the tightens
**Goal:** the single-home rule holds everywhere the survey found a second copy.
**Changes (one line each, survey § 1 and § 3):**
- [ ] `DECISIONS.md` L146-182 (DL-026 Bluesky client behaviour) → forage's ledger, after
      L170-176 is confirmed present there; L197-206 → `fun/plans/2026-09-08-plan-looseends-mechanic.md`.
- [ ] `SKINS.md` L33-70 restates ADR-003 while L3 says it does not → pointer.
- [ ] `VERIFICATION.md` 3b (90 lines for one technique) → the script, its two limits, one
      sentence on the brace forms.
- [ ] `PATTERN.md` L141-171 repeats the check-12/DL-011 story told at L88-90 → one telling.
- [ ] `MOCKS.md` L318-321 revision logs → met/not-met + sha (each mock's Revisions block
      holds the detail, per its own rule 1).
- [ ] `CI-PATTERN.md` L154-173 "Closed since" → struck with date. `TESTBED.md` cell notes
      L31/33/41 → the row's why column. `CHANGELOGS.md` L248-254 is state, not rule —
      verify the seeded `claude/changelog` branches landed before striking.
**Wiring test:** as Phase 2.
**Depends on:** Phase 2 landed.
**Done when:** no paragraph in the survey's § 3 list exists in more than one file;
audit green; Review Log carries the before/after line totals.
**Validation:** narrow — audit plus grep of each § 3 pair.

## Open Questions

- [RECOMMENDED: BLOCKING] **Q1 — scope.** Phase 1 only, Phases 1-2, or all three?
  *Recommend all three, one PR per phase, stopping after any; Phase 1 alone removes the
  per-turn tax, which is where the owner's ask bites hardest.*
- [RECOMMENDED: BLOCKING] **Q2 — where the 161-line Current-focus history goes.**
  (a) archived verbatim as a dated file — `discovery/alpha/STATE-ARCHIVE-2026-09.md` —
  and struck from `CLAUDE.md` with a pointer; (b) rely on the `croft-stack/sessions/*` and
  runbook files it restates, and cut it. *Recommend (a): `PATTERN.md` says retire by
  striking, and the narrative's cross-repo ordering exists nowhere else even though each
  fact does.*
- [RECOMMENDED: PHASE-GATED, Phase 2] **Q3 — move `COORDINATION.md` § "Verify at the
  layer that acts" into `VERIFICATION.md` as shape 4?** *Recommend yes: it is the
  verification dimension's concern, and `VERIFICATION.md` § 2b (2026-09-14) already
  cites it; COORDINATION keeps a one-line pointer.*
- [RECOMMENDED: PHASE-GATED, Phase 1] **Q4 — keep one "most-missed" clause per table
  row, or drop the clause and rely on each canonical doc's own "Most-missed" block?**
  *Recommend keep one clause of at most one line: the table's job is to make the
  rule most often missed visible before the doc is opened.*
- [RECOMMENDED: ADVISORY] **Q5 — make the `CLAUDE.md` budget a check.** `PATTERN.md` says
  "roughly a screen"; an audit NOTE at, say, 120 lines makes it a rule with a fixture.
  *Recommend yes, in Phase 1, RED on today's file.*

## Review Log

- 2026-09-14 — Pass 1 written from the full-read survey (subagent, all 22 files, every
  claim cited by file:line). Not yet reviewed (Pass 2/3 pending the owner's answers to
  Q1-Q5, since Q1 and Q2 change the phase list).
