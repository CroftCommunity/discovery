# Dimension streamline — the convention layer, concise where it can be, one home per why

**Status: ACCEPTED 2026-09-14 — owner: all three phases; the Current-focus history is archived (Q2a). Q3-Q5 taken as recommended. Pass 2 done 2026-09-14 (Review Log; one new question, Q6). Pass 3 next, then Phase 1.**

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
  finding. *(Pass 2, re-verified: `workspace-audit.sh:261,350,710` read `<repo>/CLAUDE.md`,
  never the workspace one.)*
- **Only one script parses a canonical doc by structure** (Pass 2): `bin/lexicon-register.sh:77`
  reads `LEXICONS.md` between `<!-- audit:owned-namespaces -->` markers (the "Namespaces we
  may mint into" list, L254-277 — a section Phase 2 keeps). Every other script names a doc
  only inside its finding text (`changelog-shape.sh:33`, `commit-shape.sh:27`,
  `worktree-layout.sh:69,110`, `skin-vocab.sh:47-95`, `plan-ordinal.sh:42,65`,
  `workspace-audit.sh` throughout) — section *names* in those messages must survive
  (§ Feature workspaces, § Claims, § Knowledge placement, § Two piles, § Plan files,
  § Commit messages, § Landing messages, SUPPLY-CHAIN rules 1/3/7/8/10, VERIFICATION
  shape 2, LEXICONS § 1/§ 2/§ Scope) but no line numbers do.
- `workspace-audit.sh` takes ~4 minutes and reports `RESULT: <n> finding(s)`; the
  live baseline on 2026-09-14 is recorded in the Review Log when Phase 1 starts.
  **It reads `$ROOT/.claude/*` — the main checkout's docs, not the branch's**
  (`workspace-audit.sh:41`, `ROOT` via `workspace-root.sh`), so a pre-landing run proves
  only that nothing *else* moved; the post-landing run is the one that grades the edit.
- **The "survey" cited by Phase 2/3 Done-criteria is not a file** (Pass 2: the only file
  matching `survey` under `plans/` is this plan). Its numbers that the plan leans on are
  re-derivable from the docs themselves and are pinned here instead: `SUPPLY-CHAIN.md`
  has 11 rules (`grep -cE '^\*\*[0-9]+\. ' = 11`, L33-264); `LEXICONS.md` four acts
  (`^## [1-4]\.`); `COORDINATION.md` Rules 1, 1a, 1b, 2, 3 plus the sections at L20, 67,
  99, 332, 405, 415, 580, 624, 706; `TRACKING.md` sections L15, 34, 95, 119, 189, 266.
  Phase 2's rule-by-rule diff is against these counts; Phase 3's pair list is its own
  Changes list.
- Open `CroftC` branches that touch the write-set (Pass 2, `gh pr list` + worktree diffs):
  **PR #62** `claude/hooks-config-dir` (e0bdb2d) — `VERIFICATION.md` (+§ 2b, +checklist
  item 6, Maintenance "checks 23 and 37") **and** `bin/workspace-audit.sh` (check 23,
  L534-590); **PR #34** `claude/feature-manifest` (2c85047, stale) — `COORDINATION.md`
  § Feature workspaces + the Maintenance roll-up, and appends **check 47**, a number main
  has since given to SHARED-CODE (`workspace-audit.sh:1300`; main's next free is 49).
  `claude/mocks-handoff` touches nothing in `.claude/`.
- The meta-repo worktree **already exists**: `worktrees/dimension-streamline/CroftC` at
  `c0d1f2c` on `claude/dimension-streamline`, beside the `discovery` member that holds
  this plan (`08353a8`). Nothing to create at Phase 1 start.
- Whys that live ONLY in text this plan cuts (survey § 5), with their destination:

| From | Why | To |
|---|---|---|
| `CLAUDE.md:175-177` | OAuth sessions die at ~6-11 days idle; re-sign-in is step 0 of every device run | `TESTBED.md` § Devices — the "Device-rig facts that keep getting relearned" paragraph (L44-51), both phones |
| `CLAUDE.md:177-181` | `adb install -r` over the released APK clears app data, the iroh key with it; the Pixel carries a debug build whose record names that identity | `TESTBED.md` Pixel row (L42) |
| `CLAUDE.md:118-122` | ~~`admitted sponsorship=` is debug-level and filtered in production~~ **stale since E148** (croft-stack `TODO.md:140-142`, DONE 2026-08-28: `relay_log_filter` carries `croft_relay=debug` on both listeners). What survives — *a successful mint is silent at every layer; the attributed `usage` line was the instrument before E148* — is **already home** at croft `ops/RUNBOOK-two-device-call-test.md:503-508` | Nothing moves. Strike from `CLAUDE.md` with the two pointers. Not `croft-stack/docs/RUNBOOK.md` — it has no admission-log text at all, and it is a claimed surface (croft-stack `CLAUDE.md` § Concurrent sessions) |
| `CLAUDE.md:202-204` | runbook §13 step 3 is no longer needed as written; a staging run is scoped to the wrong-key refusal only | **Already home**: croft-stack `TODO.md:193-196` (scope to `SignatureOrMalformed` "if it runs") and croft runbook §15.4 (L752-760, "not needed and was not run … Prefer this shape"). Strike with pointers; at most a one-line struck note at §13 step 3 (L514 shows it ran once, 2026-08-28) |
| `SUPPLY-CHAIN.md:368-371, 383-385, 386-393, 394-398, 417-421, 441-443` | Gradle classpath regex anchoring; `--retry-all-errors` load-bearing; cross-repo checkout `ref:` defaults to the caller; `requirements.txt` was unenumerated; osv-scanner exit 128/127; `test-*.sh` had no runner | rules 4, § The gate, 10, 6, § The gate, and `VERIFICATION.md` shape 2 respectively (rule numbers verified against L52, L105, L222; the `ref:` why fits § The gate better than rule 10 if § The gate is where the reusable-workflow caller is described — decide when the text is in hand) |
| `LEXICONS.md:248-252, 326-330` | CNAME/TXT near-miss; DAG-CBOR key reorder breaks byte compare | **Split, not one move** (Pass 2): the script is `CroftC/.claude/bin/publish-lexicons.mjs`, not in croft-stack. The DNS half (L216-252, the measured zone state + the CNAME/TXT why) → `croft-stack/docs/DNS.md` § "Pending, not created: `_lexicon` TXTs" (L44-50 — a stub that is itself stale, since the two domains' `_lexicon` records were decided 2026-08-29); the publish runbook (L278-330, the DAG-CBOR why) — Q6 |
| `COORDINATION.md:355-361` | "suspect the instrument first" — three sessions, three layers, one day | travels with § Verify-at-the-layer (Q3); the four citations of that section repoint in the same PR (`PATTERN.md:128`, `LEXICONS.md:197`, `SHARED-CODE.md:93`, `workspace-audit.sh:569`) |
| `VERIFICATION.md:143-158` | the re-export brace forms are the trap | one sentence kept with the script |
| `PATTERN.md:70-77` | check 36's first draft keyed on the fixing comment | kept (it is the why for "key on the layer that acts, including when checking a check") |
| `DECISIONS.md:170-176` | forage's deliberate divergences from Bluesky client behaviour | forage's ledger — `forage/ledger/divergence.js` (the file audit check 20 reads, `workspace-audit.sh:330`), DL-026's entry — before the row is cut to a pointer |

## Documentation Impact

Phase 1: `CroftC/.claude/CLAUDE.md`, `CroftC/README.md`, `CroftC/.claude/DECISIONS.md`
(rows L45-66), `CroftC/.claude/TESTBED.md` (two whys in), ~~`croft-stack/docs/RUNBOOK.md`
or `TODO.md` (one why in), croft `ops/RUNBOOK-two-device-call-test.md` (one why in)~~
*(Pass 2: both whys are already recorded there — strike-with-pointers only; the croft
one-liner is optional)*, a new archive file for the Current-focus history
(`discovery/alpha/ROUND-2026-09-14-enforcement-flip-and-first-call.md` + one Layout line
in `discovery/alpha/README.md`), each canonical doc that gains a "Most-missed" block (Q4).
Phase 2: `COORDINATION.md`, `SUPPLY-CHAIN.md`, `LEXICONS.md`, `TRACKING.md`,
`CHANGELOGS.md`, `VERIFICATION.md`, `PATTERN.md:128`, `SHARED-CODE.md:93`,
`bin/workspace-audit.sh:569` (repoints), `discovery/alpha/plans/2026-08-29-plan-supply-chain-rollout.md`
(gains the rollout log as entry 11), `croft-stack/docs/DNS.md` (gains the `_lexicon` zone
state; the publish runbook per Q6). Phase 3: `DECISIONS.md`,
`SKINS.md`, `VERIFICATION.md`, `PATTERN.md`, `MOCKS.md`, `CI-PATTERN.md`, `TESTBED.md`,
forage's decision ledger. `DECISIONS.md` row `workspace/pattern` gains the line budget
if Q5 is yes. Grepped: no other file cites the `CLAUDE.md` line numbers being moved.

**Section citations that must keep resolving (Pass 2, grepped workspace-wide incl.
`~/.claude/coding-agents`, per-repo `CLAUDE.md`/`AGENTS.md`, `discovery/AGENTS.md`):**
- `CLAUDE.md` § *Current focus* is cited by `README.md:11,135`, `TRACKING.md:30-31,113,254`.
  Phase 1 keeps the heading text **"Current focus"** verbatim, and rewrites the two
  *descriptions* of it — `README.md:135-136` ("a dated narrative of the active thread")
  and `TRACKING.md:30-31` ("a dated narrative of citations") — to say what the ≤15-line
  block now is, with the archive named. Adds `README.md` + `TRACKING.md` (two lines) to
  Phase 1's write-set.
- `COORDINATION.md` § "Verify at the layer that acts" is cited by `PATTERN.md:128`,
  `LEXICONS.md:197`, `SHARED-CODE.md:93`, `workspace-audit.sh:569` — all four repoint to
  `VERIFICATION.md` shape 4 in Phase 2 (Q3). `VERIFICATION.md` itself cites it nowhere,
  on main or on PR #62 — Q3's "§ 2b already cites it" was wrong; the case for the move is
  the section's subject, not an existing pointer.
- `CHANGELOGS.md:244` cites "`SUPPLY-CHAIN.md` § Checks predicts of a counter" — that
  sentence is inside the L240-246 block Phase 2 turns into a pointer to `TRACKING.md`,
  so the citation goes with it.
- Every per-repo `CLAUDE.md` ends with a "Concurrent sessions (workspace norm)" block
  pointing at `COORDINATION.md` (not at `CLAUDE.md`'s summary); `discovery/AGENTS.md:11`
  cites `CroftC/.claude/CLAUDE.md` only as the importer. Nothing in a repo cites a
  section this plan cuts.
- Phase 2 adds `croft-stack/docs/DNS.md` (§ `_lexicon`, replacing the L44-50 stub) and
  the rollout plan's log is a sequence of `## Review Log — entry N` headings (last:
  entry 10, `2026-08-29-plan-supply-chain-rollout.md:716`) — the moved
  `SUPPLY-CHAIN.md` L323-474 lands as **entry 11**, dated, headed "moved verbatim from
  SUPPLY-CHAIN.md § Current state (2026-08-29) on 2026-09-XX", not spliced into entries
  it post-dates.

## Concurrency Map

All phases sequential: each phase rewrites pointers the previous one created. *(Pass 2
corrected the reason: only Phase 1 touches `CLAUDE.md`. What actually serialises them is
that Phase 1's "Most-missed" blocks (Q4) land in the same canonicals Phase 2 restructures
— `COORDINATION`, `SUPPLY-CHAIN`, `LEXICONS`, `TRACKING` — and `VERIFICATION.md` is in
both Phase 2's write-set (shape 4 in) and Phase 3's (3b cut), `DECISIONS.md` in Phase 1's
(rows) and Phase 3's (prior-art section). One shared file disqualifies a parallel set;
no missed parallelism.)* Shared state: the CroftC meta-repo worktree
`worktrees/dimension-streamline/CroftC` (**exists**, `c0d1f2c`, with the `discovery`
member holding this plan), one PR per phase, landing per COORDINATION Rule 2.

**Shared-state contract (invariants, all phases):** writes only under
`worktrees/dimension-streamline/<repo>`; no `git checkout`, `stash`, `reset` or `add -A`
in any main checkout under `CroftC/`; no claim file needed for Phase 1 (nothing in its
write-set is a contested surface — `croft-stack/docs/RUNBOOK.md` left it in Pass 2);
the audit is run read-only and its `ROOT` line is recorded with each result (it grades
main's docs, see Verified Assumptions). **Re-entry verification:** `git -C
worktrees/dimension-streamline/CroftC status --porcelain` empty between steps; `git -C
CroftC status --porcelain` empty always; `ls .coordination/claims/` unchanged by this work.

**Write-set overlaps with open branches (Pass 2, both must be sequenced, not merged blind
— audit check 19 will NOTE them):**
- **PR #62** `claude/hooks-config-dir` edits `VERIFICATION.md` *and* `bin/workspace-audit.sh`
  (check 23). That overlaps **Phase 1** (Q5 appends a check to the same script) as well
  as Phase 2 — land #62 before Phase 1 opens its PR, or rebase Phase 1 over it (textually
  far apart: check 23 at L534-590, the new check goes above the summary at the end).
- **PR #34** `claude/feature-manifest` (stale at 2c85047) edits `COORDINATION.md`
  § Feature workspaces *and* the Maintenance roll-up that Phase 2 deletes, and appends
  **check 47** — a number main has since allocated to SHARED-CODE. Its owner must renumber
  via `bin/next-id.sh check` and rebase regardless of this plan; Phase 2's COORDINATION
  cut should land *after* #34 or #34 rebases onto a Maintenance section with no roll-up
  (which is fine: the sentence it adds there has no home once each doc owns its checks).
  Message its owner at Phase 2 start (COORDINATION § Claims).
- Q5's new check takes its number from `bash .claude/bin/next-id.sh check` at the moment
  it is written (reads main + every unlanded `claude/*` tip; 49 today), never `max+1`.

## Phases

### Phase 1: the auto-loaded surfaces become indexes again
**Goal:** `CLAUDE.md` at roughly one screen of dimension summary plus a short focus block;
`README.md` and `DECISIONS.md` rows at one line each; zero whys lost.
**Changes:**
- [ ] Relocate the two orphan whys (ledger rows 1-2) into `TESTBED.md` — first, in the same
      PR. Rows 3-4 need no move (Pass 2: both already recorded in croft-stack `TODO.md` and
      the croft runbook; row 3's claim is stale since E148) — strike with pointers; the
      optional one-line note at croft runbook §13 step 3 is a croft-repo change and can be
      skipped without losing a why.
- [ ] `CLAUDE.md` § Current focus → ≤15 lines: state now, next in order, pointers to
      `croft-stack/sessions/*` and the croft runbook; the 161 lines archived per Q2.
      **Keep the heading text "Current focus"** (five citations resolve on it) and reword
      its two descriptions, `README.md:135-136` and `TRACKING.md:30-31`, to name the
      archive and say the block is now a pointer set, not a narrative.
- [ ] The archive (Q2): `discovery/alpha/ROUND-2026-09-14-enforcement-flip-and-first-call.md`,
      verbatim L53-213 of `CLAUDE.md@c0d1f2c` under a header saying so (struck
      2026-09-14, span 2026-08-17→2026-09-14, "evidence lives in `croft-stack/sessions/*`
      and croft runbook §§11-16"); one Layout line added in `discovery/alpha/README.md`
      (findability rule). Same `discovery` worktree as this plan, one PR.
      *(Pass 2 found the draft already on disk, untracked, as
      `alpha/STATE-ARCHIVE-2026-09.md` (21:53, 173 lines): its 12-line header is the right
      header, and its body is byte-identical to L53-213 — `cmp` exit 0. Adopting the
      ROUND name is one `mv` plus the README line; the owner decides which name, since
      a session had already reached for the Pass 1 one.)*
- [ ] Dimension table rows → one line + pointer (+ one most-missed clause per Q4); each
      row's elaboration moves into its canonical doc's own "Most-missed" block
      (`DESIGN.md:41-45` is the existing shape).
- [ ] Concurrent-sessions summary → compression only (Rule 1b and Rule 2 whys already in
      `COORDINATION.md:288-298` and `214-243`); fix the attribution command to match
      `COORDINATION.md:472`.
- [ ] `README.md` L69-123 → one line per dimension. `DECISIONS.md` rows L45-66 → one line
      each (the header promises one line).
- [ ] If Q5 is yes: audit NOTE when `CLAUDE.md` exceeds the budget; RED fixture = today's
      file at `c0d1f2c`, landed above the summary line (check 36); number from
      `next-id.sh check`; run the whole harness and confirm the RESULT total moved
      (PATTERN step 5) — and its rule sentence goes in `PATTERN.md` § Anti-collapse beside
      "roughly a screen", since a check with no written rule is drift (COORDINATION L715).
**Call chain:** every session's system prompt → `CroftC/.claude/CLAUDE.md` → the table's
pointer → the canonical doc. The chain is the whole point: the top must say less, and the
pointer must resolve.
**Wiring test:** `bash .claude/bin/workspace-audit.sh` — `RESULT` count unchanged from the
recorded baseline (or each delta named), **run twice: pre-landing (proves nothing else
moved) and post-landing (the run that grades the docs — see Verified Assumptions on
`ROOT`)**; every ledger row greppable at its destination; every `→` pointer in the table
names a file that exists; the five § Current focus citations still resolve
(`grep -rn 'Current focus' .claude README.md`).
**Depends on:** Q1, Q2, Q4, Q5 answered; PR #62 landed or rebased over (shared
`workspace-audit.sh`).
**Read-set / Write-set:** as in Documentation Impact, Phase 1 line, plus `README.md:135-136`,
`TRACKING.md:30-31`, `PATTERN.md` § Anti-collapse (Q5 rule sentence), `discovery/alpha/README.md`
(one Layout line). Minus `croft-stack/docs/RUNBOOK.md` (Pass 2).
**Shared-state contract:** the Concurrency Map's invariants; nothing beyond them.
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
- [ ] `LEXICONS.md` L216-252 (the measured DNS state + CNAME/TXT why) → `croft-stack/docs/DNS.md`
      § `_lexicon`, replacing its stale L44-50 stub; L278-330 (the publish runbook + DAG-CBOR
      why) per **Q6**. The `<!-- audit:owned-namespaces -->` markers (L254-277 region) stay
      exactly where `lexicon-register.sh:77` reads them — run `bin/test-lexicon-register.sh`
      after the cut. The "What is not met yet" table (L204-214) stays: PATTERN step 1
      requires the state-of-play table *in the dimension doc*.
- [ ] Repoint the four citations of § "Verify at the layer that acts" — `PATTERN.md:128`,
      `LEXICONS.md:197`, `SHARED-CODE.md:93`, `workspace-audit.sh:569` — to
      `VERIFICATION.md` shape 4, same PR as the move (the audit line is a finding
      message; `grep -rn 'Verify at the layer' .claude` must return only VERIFICATION
      afterwards).
- [ ] Check-number collision lesson: one home (`TRACKING.md:210-218`); `SUPPLY-CHAIN.md:
      316-321` (stale — says "allocate from a fresh git log" where the tool is
      `next-id.sh check`) and `CHANGELOGS.md:240-246` become pointers (the
      `CHANGELOGS.md:244` citation of SUPPLY-CHAIN § Checks goes with the block).
- [ ] The `SUPPLY-CHAIN.md` L323-474 move lands as `## Review Log — entry 11` in the
      rollout plan, dated and headed as a verbatim move (the plan's log is a sequence of
      numbered entries, last = 10 at L716); the not-met table (L466-474) stays in
      `SUPPLY-CHAIN.md` as the dated state of play.
**Wiring test:** audit `RESULT` unchanged (pre- and post-landing, as Phase 1); every moved
section greppable at its destination by its heading; `bin/test-*.sh` all green
(`test-lexicon-register.sh` is the one that reads a doc this phase cuts); rule count
unchanged against the counts pinned in Verified Assumptions.
**Depends on:** Phase 1 landed; PR #62 landed (VERIFICATION.md overlap); PR #34
sequenced with its owner (COORDINATION.md § Feature workspaces + Maintenance overlap,
and its check-47 collision) — see the Concurrency Map.
**Risks:** `SUPPLY-CHAIN` rules 4/6/10 grow — keep each promoted why to two lines.
**Done when:** the four docs each under ~250 lines with no rule removed (diff reviewed
rule by rule against the counts pinned in Verified Assumptions — 11 SUPPLY-CHAIN rules,
four LEXICONS acts, COORDINATION Rules 1/1a/1b/2/3 and its nine sections, TRACKING's
six); wiring test in the Review Log.
**Validation:** moderate — audit plus a rule-by-rule diff read.

### Phase 3: the tightens
**Goal:** the single-home rule holds everywhere the survey found a second copy.
**Changes (one line each, survey § 1 and § 3):**
- [ ] `DECISIONS.md` L146-182 (DL-026 Bluesky client behaviour) → forage's ledger
      (`forage/ledger/divergence.js`, DL-026), after L170-176 is confirmed present there;
      L197-206 → `fun/plans/2026-09-08-plan-looseends-mechanic.md` (exists, Pass 2). Both
      are other-repo edits: `worktrees/dimension-streamline/{forage,fun}` members, their own
      PRs, landed before the `DECISIONS.md` pointer lands.
- [ ] `SKINS.md` L33-70 restates ADR-003 while L3 says it does not → pointer.
- [ ] `VERIFICATION.md` 3b (90 lines for one technique) → the script, its two limits, one
      sentence on the brace forms.
- [ ] `PATTERN.md` L141-171 repeats the check-12/DL-011 story told at L88-90 → one telling.
- [ ] `MOCKS.md` L318-321 revision logs → met/not-met + sha (each mock's Revisions block
      holds the detail, per its own rule 1).
- [ ] `CI-PATTERN.md` L154-173 "Closed since" → struck with date. `TESTBED.md` cell notes
      L31/33/41 → the row's Notes column, one clause each (the tables have no "why"
      column — Resource / What / Credentials / Notes, L27-28 and L39-40). `CHANGELOGS.md`
      L248-254 is state, not rule — verify the seeded `claude/changelog` branches landed
      before striking.
**Wiring test:** as Phase 2.
**Depends on:** Phase 2 landed.
**Done when:** no paragraph in the pair list above (this phase's Changes — the survey's
§ 3 list, pinned here since the survey is not a file) exists in more than one file;
audit green; Review Log carries the before/after line totals.
**Validation:** narrow — audit plus grep of each § 3 pair.

## Open Questions

Answered 2026-09-14 by the owner: **Q1 all three phases; Q2 (a) archive.** Q3 (move), Q4 (one clause), Q5 (budget check) taken as recommended — the owner accepted the plan without overriding them.

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
  verification dimension's concern, ~~and `VERIFICATION.md` § 2b (2026-09-14) already
  cites it~~ (Pass 2: it does not — the four files that do are listed in Documentation
  Impact and repoint in Phase 2); COORDINATION keeps a one-line pointer.*
- [RECOMMENDED: PHASE-GATED, Phase 1] **Q4 — keep one "most-missed" clause per table
  row, or drop the clause and rely on each canonical doc's own "Most-missed" block?**
  *Recommend keep one clause of at most one line: the table's job is to make the
  rule most often missed visible before the doc is opened.*
- [RECOMMENDED: ADVISORY] **Q5 — make the `CLAUDE.md` budget a check.** `PATTERN.md` says
  "roughly a screen"; an audit NOTE at, say, 120 lines makes it a rule with a fixture.
  *Recommend yes, in Phase 1, RED on today's file.*
- [RECOMMENDED: PHASE-GATED, Phase 2 — **new in Pass 2**] **Q6 — where the lexicon
  *publish* runbook (`LEXICONS.md` L278-330) goes.** Pass 1 sent it to `croft-stack/docs/`
  "beside `publish-lexicons.mjs`", but the script lives in `CroftC/.claude/bin/`, and
  croft-stack owns only the DNS half (`docs/DNS.md`). Options: (a) it stays in
  `LEXICONS.md` under § Publishing, compressed to the commands plus the DAG-CBOR why
  (~20 lines); (b) it moves into the script's own header comment, which already carries
  usage (`publish-lexicons.mjs:24-25`), and `LEXICONS.md` § 2 points there; (c) it goes to
  croft-stack anyway, which then documents a tool it does not own. *Recommend (a): the
  runbook is the "publish" act's own procedure, the doc is the act's home, and the line
  saving is the whole gain of (b). Reject (c).* With (a), the DNS half still leaves for
  `DNS.md` (its zone measurements are croft-stack's) — Phase 2's LEXICONS target of
  ~250 lines is still met (360 − 37 DNS lines − ~30 of runbook compression).

**Q2's answer, made concrete (Pass 2):** the archive is
`discovery/alpha/ROUND-2026-09-14-enforcement-flip-and-first-call.md`, not
`STATE-ARCHIVE-2026-09.md`. Why there: it is cross-repo synthesis (discovery's scope,
`discovery/AGENTS.md:5`), not a croft-stack session (`croft-stack/sessions/README.md`:
one file per session, command-level actuals, private repo) and not orientation-layer
content (`REPO-GRAMMAR.md:14-17` — the root admits only `.claude/`, `README.md`,
`.gitignore`). Why that name: `ROUND-<date>-<slug>.md` is the leaf shape already at
`discovery/alpha/` root for exactly this — "a plain-language summary of what we ran …
the hard evidence lives in …" (`ROUND-2026-06-17-media-meer-conformance.md:1-6`);
a name is extracted, not invented. `alpha/README.md` § Layout maps neither the ROUND nor
the ROLLUP files today; Phase 1 adds the one line for this file and leaves the older gap
as a NOTE in the Review Log.

## Review Log

- 2026-09-14 — Pass 1 written from the full-read survey (subagent, all 22 files, every
  claim cited by file:line). Not yet reviewed (Pass 2/3 pending the owner's answers to
  Q1-Q5, since Q1 and Q2 change the phase list).
- 2026-09-14 — Owner accepted: all three phases, archive the history (Q2a). Q3-Q5 as recommended.

### Pass 2: Gap Analysis — 2026-09-14

Every ledger row and every line range the plan cites was opened and compared against
the file on `main` at `c0d1f2c`; the audit and the `bin/*.sh` helpers were grepped for
each doc name and section heading; the workspace (per-repo `CLAUDE.md`/`AGENTS.md`,
`discovery/AGENTS.md`, `~/.claude/coding-agents`) was grepped for citations of every
section cut or moved; open `CroftC` branches were diffed against main.

**Found:**
- Ledger row 3 (`CLAUDE.md:118-122`) relocates a **stale** why: E148 (croft-stack
  `TODO.md:140-142`, DONE 2026-08-28) widened the relay log filter, so `admitted …` lines
  are visible on production — the plan's own Current-focus text says so at L95 and L205.
  The surviving lesson (silence is success; `usage` was the instrument) is already at
  croft runbook L503-508. Its proposed home, `croft-stack/docs/RUNBOOK.md`, contains no
  admission-log text and is a claimed surface. Row 4 (`CLAUDE.md:202-204`) is likewise
  already recorded at croft-stack `TODO.md:193-196` and croft runbook §15.4 (L752-760).
  Both rows become strike-with-pointers; Phase 1 loses two cross-repo edits.
- The LEXICONS destination was wrong on a fact: `publish-lexicons.mjs` is at
  `CroftC/.claude/bin/`, not croft-stack. croft-stack owns the DNS half (`docs/DNS.md`
  L44-50, a stub that is itself stale). Split into DNS → `DNS.md`, runbook → **Q6**.
- The "survey" that Phase 2/3 Done-criteria measure against is not a file (only this plan
  matches). Pinned the rule counts and section lists in Verified Assumptions instead.
- Q3's justification claimed `VERIFICATION.md` § 2b cites COORDINATION § Verify; it does
  not (zero hits on main and on PR #62's branch). Four *other* files do —
  `PATTERN.md:128`, `LEXICONS.md:197`, `SHARED-CODE.md:93`, `workspace-audit.sh:569` — and
  none were in the plan's write-set. Added as a Phase 2 step.
- `bin/lexicon-register.sh:77` parses `LEXICONS.md` by `<!-- audit:owned-namespaces -->`
  markers — the one script that reads a canonical doc by structure. The markers sit in
  the L254-277 region Phase 2 keeps; added the test to Phase 2's wiring test.
- Five citations of `CLAUDE.md` § Current focus (`README.md:11,135`,
  `TRACKING.md:30-31,113,254`) resolve on the heading text; two of them *describe* it as
  "a dated narrative", which stops being true in Phase 1. Heading kept verbatim, the two
  descriptions added to Phase 1's write-set.
- PR #62 touches `workspace-audit.sh` as well as `VERIFICATION.md` — an overlap with
  **Phase 1** (Q5's new check), not only Phase 2. PR #34 (`claude/feature-manifest`,
  stale) edits the COORDINATION Maintenance roll-up Phase 2 deletes and appends check 47,
  a number main has since given to SHARED-CODE (`workspace-audit.sh:1300`) — a collision
  outside this plan's scope that its owner must fix, and an ordering constraint inside it.
- The audit reads `$ROOT/.claude/*` — the main checkout's docs — so a pre-landing run
  cannot grade a branch's doc edits (VERIFICATION shape 3 if read as if it could). Wiring
  tests now say pre- and post-landing, and record `ROOT`.
- The meta-repo worktree the map said "to be created" already exists at `c0d1f2c`.
- Q5's check needs its number from `next-id.sh check` (TRACKING L214-218) and a rule
  sentence in `PATTERN.md` § Anti-collapse, or it is a check with no written rule
  (COORDINATION L715) — both added to the Phase 1 step.
- `TESTBED.md` tables have a Notes column, not a "why column" (Phase 3 wording fixed).
  forage's ledger is `forage/ledger/divergence.js` (path added). The fun plan exists.
- The rollout plan's Review Log is a sequence of `## Review Log — entry N` headings (last
  10); the SUPPLY-CHAIN move lands as entry 11, dated as a move, not spliced.
- **Phase 1 is already executing, uncommitted, in `worktrees/dimension-streamline/CroftC`**
  (found at the end of this pass: `CLAUDE.md` −295/+, `DECISIONS.md`, `TESTBED.md` +10,
  `workspace-audit.sh` +16, `README.md` −77/+; plus the untracked archive draft in the
  `discovery` member). `pass2.md` says planning does not execute; this pass touched none
  of those files. What the in-flight diff already matches: check number **49** from the
  allocator; the `## Current focus (2026-09-14)` heading kept; the two TESTBED whys in.
  What it does not yet reflect, for whoever holds it: the archive name (Q2, above —
  owner's call), rows 3-4 as strike-only (no croft-stack/croft edits), the
  `README.md:135-136` / `TRACKING.md:30-31` descriptions, the Q5 rule sentence in
  `PATTERN.md` (the check's budget is 150, not the 120 Q5 floated — fine, but the doc
  must say 150), and PR #62's `workspace-audit.sh` overlap (check 23 vs the new 49 —
  rebase before opening the PR, and confirm check 49 still sits above the summary
  line after it).

**Concurrency:**
- Map confirmed sequential, with the reason corrected (only Phase 1 touches `CLAUDE.md`;
  the serialisers are the shared canonicals between 1→2 and `VERIFICATION.md`/`DECISIONS.md`
  between 2→3 and 1→3). No missed parallelism: every candidate pair shares a file.
- Shared-state contract rewritten from a mechanism ("one worktree") to invariants, with
  re-entry checks named. Two open-branch overlaps recorded with their sequencing.

**Changed:**
- Status line; Verified Assumptions (+6 bullets, ledger rows 1-4, SUPPLY-CHAIN,
  LEXICONS, COORDINATION, DECISIONS revised); Documentation Impact (+ citation list);
  Concurrency Map (rewritten in place, extended); Phase 1 (rows 3-4 → strike, archive
  step with name, heading/description step, Q5 allocator + rule sentence, wiring test,
  Depends on, write-set); Phase 2 (LEXICONS split, repoint step, entry-11 shape, marker
  test, Depends on, Done counts); Phase 3 (ledger path, Notes column, other-repo PR
  sequencing, Done pair list); Open Questions (+Q6; Q2 answer made concrete with the
  archive path and its placement reasoning).

**Confirmed:**
- Ledger rows 1-2 (`CLAUDE.md:175-181`) exist as cited; `TESTBED.md` § Devices L37-51 is
  the right home under its own scope statement (L3-5), the "keep getting relearned"
  paragraph at L44-51 being the exact shape.
- Every SUPPLY-CHAIN, COORDINATION, VERIFICATION, PATTERN, DECISIONS, TRACKING, CHANGELOGS,
  SKINS, MOCKS, CI-PATTERN line range in the plan matches the file at `c0d1f2c`;
  SUPPLY-CHAIN rule numbers 4/6/10 are the rules the plan means (L52, L105, L222).
- No audit check parses the `CLAUDE.md` table, `README.md` bullets or `DECISIONS.md` rows;
  `next-id.sh` reads `workspace-audit.sh` section headers, not docs. Compressing the
  auto-loaded surfaces cannot move a finding.
- `COORDINATION.md` L708-716 is a check roll-up, not PATTERN step 4's "layer listing" —
  that listing is the diagram at L28-45, which the plan leaves alone. Deleting the
  roll-up breaks no funnel surface.
- The alternatives-rejected reasoning holds; the three-phase split still leaves the
  layer coherent after each phase, and more so now that Phase 1 has no croft-stack edit.
