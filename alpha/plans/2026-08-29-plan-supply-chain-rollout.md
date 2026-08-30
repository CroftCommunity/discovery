# Plan — rolling out the supply-chain dimension

**Status:** Pass 1 + 2 + 3 complete, all open questions resolved by the owner 2026-08-29.
**ALL SIX PHASES EXECUTED and landed 2026-08-29.**
**Standard:** `CroftC/.claude/SUPPLY-CHAIN.md` (landed 2026-08-29, `c6ff383`; rule 5
extended `fa53ddc`).
**Scope:** 18 checked-out repos, 24 in the org.
**Owner decisions taken 2026-08-29:** free tools over paid; the gate blocks on
production-path findings and NOTEs the rest; freshness budget NOTE at ≥1 major behind,
FLAG at ≥2; one outbound licence (AGPL-3.0) everywhere.

---

## Problem Statement

The workspace has a supply-chain standard and **zero enforcement of it**. Measured by the
sweep that produced the standard:

- **No repo scans for secrets.** GitHub's free scanning covers public repos only and
  *alerts* rather than blocks — so the two repos that hold credentials (`croft-stack`'s
  mint key, `CroftC`'s `.env`) are the two it does not reach.
- **No repo scans dependencies.** Dependabot covers GitHub Actions in one repo and one
  Cargo directory in another. No JavaScript ecosystem is scanned anywhere.
- **Nothing validates licences.** Four public repos published under default copyright —
  all rights reserved — until 2026-08-29.
- **Nothing tracks drift**, and drift is what makes fixes expensive: every fixable npm
  advisory here requires a *major* bump (`vite` 5→6, `vitest` 2→3 at CVSS 9.8). Nobody
  deferred a security fix; they deferred a version bump and it became one.
- **Audit checks 31–35 FLAG 54 findings** and nothing acts on them.

`PATTERN.md`: a standard with no check decays into prose. This dimension has more surface
to decay across than any other.

**What this plan is not.** It is not vulnerability remediation. The one repo investigated
in full — `croft` — resolved to **zero reachable** under rule 5's ladder and produced an
exceptions file, not a code change. The problem is absent machinery, not a backlog of
exploitable bugs; conflating them gives the rollout false urgency.

---

## Reasoning

**Why staged.** Blocking gates switched on against an unrecorded backlog get disabled
within a week, and a disabled gate is worse than none because it reads as coverage.
Phase 0 exists so every later phase starts green.

**Why secrets first.** Cheapest phase, widest hole. The backlog was *measured* at two
benign entries across the full history of eight repos, so the allowlist is two lines. That
window closes as the workspace grows: a scanner adopted at zero backlog is a gate;
adopted at two hundred findings it is a permanent mute.

**Why enforcing surfaces before static sites.** Same shape as the enforce flip — prove the
mechanism where it matters, then widen.

**Why the rule-5 ladder belongs *in* the gate, not applied by hand.** The Android scan
produced 43 advisories, 19 High, of which zero reach the APK. A severity-only gate would
have blocked a client release on netty CVEs in the emulator-control plugin.

**Why a reusable workflow rather than 18 copies.** Five repos have no CI at all, so
universal secret scanning otherwise means authoring five workflows for repos with nothing
else to run. One reusable workflow keeps the pinned scanner versions in one place — which
is where `CI-PATTERN` rule 7 wants them.

**Alternatives rejected.**
- *`cargo-deny` + `npm audit` + `pip-audit` per ecosystem.* Better per-ecosystem depth;
  rejected because four tools mean four config dialects and four places an exception rots
  unnoticed. `osv-scanner` reads all four lockfile formats from one config.
- *GitHub Secret Protection.* Rejected on owner instruction (cost) and on merit: it does
  not gate, and its free tier misses exactly the private repos that hold secrets.
- *Blocking everything from day one.* Rejected — see "why staged".
- *One big security workflow per repo, copied.* Rejected — 18 copies of a pinned version
  string is 18 places to forget.

**What could make this plan wrong.** The "adopt at zero backlog" premise rests on a secret
scan of **8 of 18 repos**. If Phase 1 finds real secrets in the other 10, Phase 1 becomes
a remediation project that should be planned separately rather than absorbed here.

---

## Verified Assumptions

Everything below was confirmed on 2026-08-29. Anything not listed is unverified.

| Assumption | How confirmed |
|---|---|
| Exactly 5 repos have no workflows: `stellin`, `crofting_site`, `arecipe_treatise`, `homebrew-tap`, `experiments` (frozen) | enumerated `*/.github/workflows` across all 18; no others |
| 4 org repos are private: `CroftC`, `croft-stack`, `experiments`, `Proofs`; 20 public | `gh repo list CroftCommunity --json visibility` |
| **A private repo CAN call a public repo's reusable workflow** | `gh api repos/CroftCommunity/croft-pwa/actions/permissions/access` → HTTP 422 *"Access policy only applies to internal and private repositories"*; `croft-stack` has `allowed_actions: all` |
| `croft-pwa/docs/CI.md` exists and is the CI reference | file present; named canonical in `.claude/CI-PATTERN.md` |
| Pinnable scanner releases exist | `gitleaks v8.30.1`, `osv-scanner v2.5.1`, `zizmor v1.29.0` (latest release tags via `gh api`) |
| `osv-scanner` auto-discovers `osv-scanner.toml` in the scanned dir | croft: identical filtered output with and without `--config` |
| `osv-scanner` reads `gradle.lockfile` | croft/android: 245 packages parsed, 43 advisories reported |
| Every fixable npm advisory needs a major bump | `vite` 5.4.21→6.4.3, `vitest` 2.1.9→3.2.6 in the scan output |
| croft's 7 advisories are all unreachable | rule-5 ladder; recorded in `croft/osv-scanner.toml` |
| GitHub has a native `sha_pinning_required` repo setting, currently **false** on all 10 repos checked | `gh api repos/CroftCommunity/<r>/actions/permissions` |

**Not verified — must be checked before the phase that depends on it:**
- Org-level Actions policy (`gh api orgs/CroftCommunity/actions/permissions` → HTTP 403,
  needs `admin:org`). Phase 5 depends on whether `sha_pinning_required` can be set
  org-wide or must be per-repo.
- Secret-scan backlog in the 10 repos not yet scanned (Phase 1's premise).
- Private-repo Actions minutes headroom for `croft-stack` + `CroftC`.

---

## Documentation Impact

| Doc | What changes | Phase |
|---|---|---|
| `CroftC/.claude/SUPPLY-CHAIN.md` § Current state | rollout status advances; the staged list is the live state | every phase |
| `CroftC/.claude/CI-PATTERN.md` § Current state table | gains a security-gate column alongside the nine rules | 1 |
| `croft-pwa/docs/CI.md` | documents the reusable security workflow and how to call it | 1 |
| `CroftC/.claude/DECISIONS.md` | a `workspace/ci-security-workflow` row if the reusable-workflow home becomes a decision rather than an implementation detail | 1 |
| `CroftC/.claude/DEP-DRIFT.md` | regenerated, and Rust/Gradle move out of "Not measured here" | 4 |
| `CroftC/.claude/SUPPLY-CHAIN.md` rule 10 | restate against GitHub-native enforcement if `sha_pinning_required` replaces manual pinning | 5 |
| `<repo>/TODO.md` (croft, CISS, connect) | items close as their phase lands | 2, 5 |
| `croft/osv-scanner.toml` | the two invalidation conditions re-checked at expiry (2026-11-29) | 0 |

No file is renamed or removed by this plan. Grepped for references to the checks by
number: `SUPPLY-CHAIN.md` and `workspace-audit.sh` only.

---

## Concurrency Map

**Sequential by default. Only one parallel set is safe.**

```
Phase 0 ─► Phase 1 ─► Phase 2 ─► Phase 3 ─► Phase 5
                 └─► Phase 4 ─┘        (parallel with 2 and 3)
Phase 6 — independent of all of it, any time
```

- **Parallel set: {Phase 4, Phase 2}** — disjoint write-sets. Phase 4 writes
  `CroftC/.claude/DEP-DRIFT.md` and `bin/dep-drift.sh`; Phase 2 writes per-repo workflow
  and config files. No overlap.
- **Phases 1, 2, 3, 5 are strictly sequential.** They all write **the same two files per
  repo** — `.github/workflows/security.yml` (or the reusable caller) and the `security`
  target in `Makefile`/`package.json`. Any overlap disqualifies parallelism; this is a
  four-way overlap.
- **Phase 6 is independent** — it writes no CI file and gates nothing.

**Shared-state contract (all phases) — invariants, not mechanisms.** "Works in a
worktree" is a mechanism and can be violated; each line below is a checkable statement
about what a phase will and will not do:

- **G1** Invokes no `git checkout`, `git restore`, `git reset --hard`, `git stash`,
  `git clean` or `git rebase` in any shared checkout at `CroftC/<repo>/`.
- **G2** Commits only to a branch matching `claude/supply-chain*`; never to any `main`.
- **G3** Stages by explicit path (`git add <paths>`); never `git add -A` or `-u`.
- **G4** Writes no file outside the repo it owns, except Phase 4 which writes exactly
  `CroftC/.claude/DEP-DRIFT.md` and `CroftC/.claude/bin/dep-drift.sh`.
- **G5** Binds no network port and starts no long-running daemon.
- **G6** Mutates no GitHub org or repo *setting* — Phase 5 is the sole exemption and
  declares it below.
- **G7** Consumes no credential beyond the ambient `gh` token; writes no secret to any
  tracked file.

**Re-entry verification — one check per invariant, run after any parallel dispatch:**

| | Check |
|---|---|
| G1 | `git -C <repo> rev-parse HEAD` equals the pre-dispatch SHA for every shared checkout |
| G2 | `git -C <repo> branch --show-current` is `main` and `git status -sb` shows no divergence beyond the landing |
| G3 | `git -C <repo> status --porcelain` lists nothing the phase did not name |
| G4 | `git -C CroftC status --porcelain` empty except the Phase 4 paths |
| G5 | `lsof -i -P -n \| grep LISTEN` shows no new listener |
| G6 | `gh api repos/CroftCommunity/<r>/actions/permissions` unchanged outside Phase 5 |
| G7 | `gitleaks detect --no-git` clean on the worktree before landing |

**Ambient state actually touched:** GitHub repo Actions settings (Phase 5 only), the
network (all scanning phases), and the `gh` token's rate limit (Phase 5's per-repo API
writes).

---

## Progress tracking and debugging readiness

**The audit is the progress tracker.** No separate state file: each phase is defined by
which audit checks it silences, so `bash .claude/bin/workspace-audit.sh` answers "how far
did we get?" and "which phase broke?" from one command. Baseline at Pass 3 time — 54 FLAGs
total, of which checks 31–35 contribute:

| after | check 31 (SCA gate) | 32 (secret gate) | 33 (pinning) | 34 (drift) | 35 (licence) |
|---|---|---|---|---|---|
| today | 14 FLAG | 18 FLAG | 1 FLAG + 11 NOTE | 1 NOTE | 3 NOTE |
| Phase 1 | 14 | **0** | — | — | — |
| Phase 2 | **0** | 0 | — | — | — |
| Phase 3 | 0 | 0 | — | — | **0** |
| Phase 4 | 0 | 0 | — | **4 FLAG** (expected: the four over budget) | 0 |
| Phase 5 | 0 | 0 | **0** | 4 FLAG | 0 |

A count that moves the wrong way, or a check that goes silent a phase *early*, is the
signal to stop — silence arriving early means the check stopped reading rather than the
drift being fixed, which is the failure mode checks 12 and 36 were both built around.
Check 34 is expected to FLAG **more** after Phase 4, not less: the register does not exist
yet, so today's single NOTE is "never generated", not "clean".

**Checkpoint between every phase:** the shared checkouts are clean and equal to upstream
(`git status --porcelain` empty, `status -sb` shows no divergence, in all 18). The rollout
touches every repo in the workspace; a phase that lands leaving one dirty will be
misattributed to the next phase.

## Phases

### Phase 0 — Baseline and exceptions

Record today's findings as the starting line, each reasoned and expiring.

- **Depends on:** nothing.
- **Read-set:** every lockfile; `osv-scanner`/`gitleaks` output.
- **Write-set:** `<repo>/osv-scanner.toml`, `<repo>/.gitleaks.toml`.
- **Re-entry verification:** `osv-scanner scan source -r .` exits 0 in the repo.
- **Validation:** the scan is clean *with* the config and non-clean *without* it — proving
  the config is what changed the result, not an empty tree. Every entry carries a reason
  a stranger could audit and an `ignoreUntil`.
- **Done for `croft`** (9 entries; clean with, 9 vulnerabilities without). Remaining:
  `CISS`, `croft-stack`, the JS repos.

### Phase 1 — Secrets, blocking, everywhere

- **Depends on:** Phase 0 (allowlist exists before the gate blocks).
- **Read-set:** `croft-pwa/docs/CI.md`; existing workflows in 13 repos.
- **Write-set:** `croft-pwa/.github/workflows/security-reusable.yml` (the host);
  `<repo>/.github/workflows/security.yml` in all 18; `croft-pwa/docs/CI.md`;
  `.claude/CI-PATTERN.md`.
- **Shared-state contract:** the reusable workflow declares `permissions: contents: read`
  and receives no secrets from callers.
- **Re-entry verification:** `gh run list --workflow=security.yml` shows a green run on
  the repo's default branch.
- **Wiring test (entry point):** the RED case must be driven **from a caller repo**, not
  from `croft-pwa`. A reusable workflow that passes its own tests while no repo actually
  calls it is the isolation trap in its purest form: `gh run list` in the host would look
  green and 17 repos would be unprotected. **Entry point = a PR in a caller repo.**
- **Validation — the gate is watched failing, at four named edges, not one.** A
  single planted secret proves the regex fired; it does not prove the gate is wired
  correctly, and a one-line change to the workflow would survive it:

  | case | expected | proves |
  |---|---|---|
  | AWS-shaped key in the PR head commit | **red** at the gitleaks step | the gate runs at all |
  | secret added in commit 1, reverted in commit 2 of the same PR | **red** | the scan reads the *commit range*, not `HEAD` — the failure this rule exists for |
  | `CISS/crates/ciss-cli/tests/fixtures/id_ed25519` untouched | **green** | the allowlist works and will not be muted wholesale on day two |
  | a caller repo with no other CI (`stellin`) | red then green | the reusable call works where there is no existing workflow to piggyback on |

  Revert after each; a red that is never returned to green proves only that CI is broken.
- **Observability:** the job must print, on failure, the rule id, the file and line, and
  the string `.claude/SUPPLY-CHAIN.md rule 1` — a gate whose output does not name its rule
  gets cargo-culted around. `--redact` stays on so the secret itself never enters the log,
  which is public on 20 of these repos.
- Scan the **PR commit range**, not `HEAD`. Pin `gitleaks v8.30.1`. Allowlist the two
  known-benign findings by path.

### Phase 2 — Dependencies, advisory everywhere, blocking on enforcing surfaces

- **Depends on:** Phase 0, Phase 1 (shares the workflow file).
- **Read-set:** all lockfiles; `SUPPLY-CHAIN.md` rule 5.
- **Write-set:** the same per-repo workflow + gate target; `<repo>/TODO.md` as items close.
- **Re-entry verification:** `make security` (or `npm run security`) exits 0 locally and
  in CI.
- **Validation — both directions with named packages, because the production-path rule is
  the thing most likely to be silently wrong:**

  | case | expected | proves |
  |---|---|---|
  | pin `h2 = "0.4.15"` in `CISS` (RUSTSEC-2026-0258, normal path via axum) | **red** | production-path findings block |
  | pin `vite@5.4.21` in `view` (dev-only, CVSS 8.2) | **NOTE, build green** | dev-only does not block — the half a severity-only gate gets wrong |
  | re-add `RUSTSEC-2026-0212` to `croft` with its `osv-scanner.toml` entry removed | **red** | the exceptions file is load-bearing, not decorative |
  | restore that entry | **green** | and the exception is what silences it, not an empty tree |

  The third and fourth cases exist because Phase 0's own validation can pass against a
  repo that simply has no findings; only removing a live exception proves the config is
  doing work.
- **Observability:** the gate prints the advisory id, the package, **and the rung of rule
  5 that decided it** (not-compiled / not-production / wrong-target / dead-function). A
  finding suppressed without naming which rung suppressed it is an exception nobody can
  re-audit at expiry.
- Blocking first in `croft`, `CISS`, `croft-stack`. Weekly `schedule:` — a new advisory
  lands against untouched code, so a PR-only trigger never fires on quiet repos, which
  are exactly the drifted ones.

### Phase 3 — Licences, one allowlist

- **Depends on:** Phase 2 (same job).
- **Write-set:** per-repo gate target; `k1-appa`/`k1-appb`/`kernel-k1` `LICENSE`.
- **Re-entry verification:** audit check 35 silent.
- **Validation:** add `readline-sync` (GPL-3.0) or any `GPL-2.0-only` crate as a direct
  dependency in a scratch branch — **refused**; remove it — **green**. Boundary case that
  matters more than the happy path: `MPL-2.0` (12 packages already in croft's tree) must
  stay **allowed**, because file-level copyleft is compatible and an allowlist that trips
  on it will be widened wholesale on its first false positive. `UNKNOWN` resolved by name
  in the config — croft's four are git/path workspace crates deps.dev has no record of.
- **Observability:** the failure names the offending package, its licence string, and the
  allowlist entry it violated — not just "licence violation".

### Phase 4 — The freshness register in CI *(parallel with Phase 2)*

- **Depends on:** nothing beyond the landed `bin/dep-drift.sh`.
- **Read-set:** every `package.json`/`Cargo.toml`; `npm outdated`, `cargo outdated`.
- **Write-set:** `CroftC/.claude/DEP-DRIFT.md`, `.claude/bin/dep-drift.sh`. **Disjoint
  from every other phase** — this is what makes the parallel set safe.
- **Re-entry verification:** `bash .claude/bin/dep-drift.sh` regenerates without error and
  audit check 34 parses it.
- **Validation:** the register regenerates unattended on a schedule and check 34 FLAGs the
  four repos already over budget (`croft-pwa` 7, `fun` 7, `view` 6, `bluebird` 6).
  Extend to Rust and Gradle, which it currently declares **unmeasured** rather than
  silently skipping.

### Phase 5 — The CI supply chain itself

- **Depends on:** Phase 1 (workflow file exists); the org-policy check under
  *Not verified*.
- **Read-set:** all `.github/workflows`; `gh api .../actions/permissions`.
- **Write-set:** `uses:` lines in 12 repos; repo Actions settings; `SUPPLY-CHAIN.md`
  rule 10.
- **Shared-state contract:** **this phase mutates GitHub org/repo settings** — the only
  phase that touches state outside a git tree, and the only one that is not revertible by
  `git revert`.
- **Re-entry verification:** audit check 33 silent; `zizmor` exits 0; and **G6's check
  inverts for this phase only** — `actions/permissions` is *expected* to differ, and the
  diff must be exactly `sha_pinning_required: false → true` on the named repos and nothing
  else.
- **Validation:** `sha_pinning_required` reads `true` via the API on each repo, and a PR
  introducing a floating tag is refused **by GitHub**, not by our check. Order matters:
  **pin the tags first, flip the setting second** — flipping it while 12 repos still carry
  floating tags blocks every open PR in the org at once.
- **Rollback — this is the only phase `git revert` cannot undo.** Record each repo's prior
  value before writing (`gh api ... --jq '.sha_pinning_required'`, all currently `false`);
  the undo is a per-repo `PUT` back to the recorded value. A phase that mutates state
  outside the tree needs its rollback written before it runs, not discovered after.
- **Observability:** `zizmor` output is archived as a run artifact; its findings are
  triaged in the Review Log rather than silenced inline.

### Phase 6 — The authored-code pass, advisory forever

- **Depends on:** nothing.
- **Read-set:** plan docs under `<repo>/plans/`.
- **Write-set:** none (review output goes in the plan's own Review Log).
- **Validation:** **none, by construction.** This phase is a habit, not a gate, and it has
  no audit check on purpose (`SUPPLY-CHAIN.md` rule 0): an LLM reviewer cannot be proven
  RED on a fixture, so a check over it would report green without meaning it.

---

## Open Questions

1. ~~Reusable-workflow home and cross-visibility calling~~ **RESOLVED in Pass 2.** A
   public host works: GitHub's access policy applies only to private/internal hosts, and
   `croft-stack` allows all actions. Recommended home `croft-pwa`, which owns the CI
   standard.
2. ~~Whether `openmls` 0.9 adoption is scheduled here or in `croft`'s roadmap~~
   **RESOLVED — owner, 2026-08-29** (severity low, confirmed). It stays in **croft's own
   roadmap**, already filed in `croft/TODO.md`. *Why:* it is an MLS stack upgrade carrying
   a device re-validation obligation, and coupling the rollout to a client release would
   make every phase here wait on §12/§13 device runs. The exceptions file lives until
   then, which is what a dated exception is for.
3. ~~The five repos with no CI~~ **RESOLVED — owner, 2026-08-29** (severity medium,
   confirmed). Each of `stellin`, `crofting_site`, `arecipe_treatise`, `homebrew-tap` gets
   a **reusable-workflow caller**; `experiments` is frozen and exempt. *Why:* the
   alternative — one scheduled workspace-wide scan — does not gate their PRs, so a secret
   could land on `main` and only surface on the next scheduled run. Consistency with the
   other 13 also means one shape to maintain, not two.
4. ~~Org-level vs per-repo `sha_pinning_required`~~ **RESOLVED — owner, 2026-08-29**
   (severity medium, confirmed). **Check org-level first, fall back to per-repo.** Phase 5
   opens with `gh auth refresh -h github.com -s admin:org`, then
   `gh api orgs/CroftCommunity/actions/permissions`. *Why the org attempt is worth an auth
   scope:* it also covers org repos not checked out here — `levelforge`, `k1-appa`,
   `k1-appb`, `kernel-k1` — which a twelve-repo loop silently misses.

**All open questions are resolved. None remain for the executor.**

---

## Review Log

**Pass 1 (2026-08-29)** — plan drafted. First version met only the `plan-doc-reasoning`
floor (Problem / Approach / Reasoning) and was landed as `a8279f0` before the
`phase-plan` template was applied. Missing: Verified Assumptions, Documentation Impact,
Concurrency Map, per-phase Read/Write-set and Shared-state contracts, Validation
calibration, this log. Rewritten here.

**Pass 2 (2026-08-29)** — gap analysis, claims verified against the repos rather than
against the plan's own logic.

- **R1 — Concurrency was implied but never checked, and was wrong.** The first draft read
  as six independent phases. Comparing write-sets shows **Phases 1, 2, 3 and 5 all write
  the same two files per repo** (the security workflow and the gate target) — a four-way
  overlap that disqualifies any parallel grouping. Only {2, 4} is disjoint. Recorded as
  the Concurrency Map; the sequential default now has a reason rather than being an
  accident.
- **R2 — Open question 1 was not open.** Verified rather than assumed: `gh api
  .../croft-pwa/actions/permissions/access` returns HTTP 422 *"Access policy only applies
  to internal and private repositories"*, so a **public** host is callable from the
  private `croft-stack`, which additionally has `allowed_actions: all`. Phase 1 no longer
  has an unresolved dependency.
- **R3 — Phase 5 had a weaker mechanism than GitHub now offers.** The plan proposed manual
  SHA pinning plus Dependabot. GitHub exposes a repo-level **`sha_pinning_required`**
  setting, currently `false` on all 10 repos checked. Enforcing at the platform means a
  floating tag cannot be merged at all, rather than being caught by an audit check after
  the fact — the difference between a gate and a report. Phase 5 rewritten around it, with
  manual pinning as the fallback where it does not apply.
- **R4 — A stated blocker resolved during the pass.** The plan noted `discovery`'s local
  main diverged with an `E150` collision. A peer session landed the pile and renumbered
  `E150`→`E153` (`6b30c3f`), so this plan lands by ordinary merge; the note is kept
  because the *reason* the earlier landing used a cherry-pick fast-forward is still the
  record of why that shape exists.
- **R5 — Validation was uncalibrated.** Every phase said "acceptance"; three of them
  needed something stronger than a passing command. Phase 1 and 2 now require the gate to
  be **watched failing** in both directions, and Phase 6 records that it has **no**
  validation on purpose rather than by omission.
- **R6 — Three assumptions were unverifiable and are now labelled so** rather than left
  implicit: the org Actions policy (needs `admin:org`), the secret backlog in 10 unscanned
  repos, and private-repo Actions minutes. Each names the phase that depends on it.

### Pass 3: Quality Gates — 2026-08-29

**TDD ordering:**
- This plan builds gates, not application code, so the TDD analogue is **RED-first on the
  gate itself**: the violation is committed, the gate is watched failing at the expected
  step, then reverted. That was present for Phases 1–2 in prose and absent from 3 and 5;
  it is now the explicit Validation for all four.
- **Specificity:** every RED case now names a concrete package and version
  (`h2 = "0.4.15"`, `vite@5.4.21`, `RUSTSEC-2026-0212`, `MPL-2.0`) instead of "a
  known-vulnerable version". Vague test descriptions produce vague gates.

**Observability:**
- Added to Phases 1, 2, 3 and 5, all missing before. The rule: **a gate's failure output
  must name its own rule** (`.claude/SUPPLY-CHAIN.md rule N`), the artefact, and — for
  Phase 2 — which rung of rule 5 suppressed a finding. An exception that does not record
  the rung cannot be re-audited at expiry, which is the entire point of the expiry.
- `--redact` retained and justified rather than assumed: 20 of these repos are public, so
  an unredacted secret in a log is a second disclosure.

**Debugging readiness:**
- Added a **Progress tracking** section. No separate state file — the audit *is* the
  tracker, with a per-phase table of which checks go silent. Includes the counter-intuitive
  case: check 34 is expected to FLAG **more** after Phase 4, because today's single NOTE
  means "register never generated", not "clean".
- Named the failure signal explicitly: a check going silent a phase *early* means the
  check stopped reading, not that drift was fixed — the failure mode behind both check 12
  and check 36.
- Added an inter-phase checkpoint: all 18 shared checkouts clean and equal to upstream.

**Validation calibration:**
- Phase 5 was under-calibrated for its blast radius. It is the only phase that mutates
  state **outside a git tree**, so it now carries a written rollback (prior values
  recorded before the write) and an ordering constraint discovered here: **pin tags
  first, flip `sha_pinning_required` second** — flipping it while 12 repos still carry
  floating tags would block every open PR in the org simultaneously.
- Phase 0's validation could pass vacuously against a repo with no findings. Phase 2 now
  includes removing a *live* exception and confirming red, which is the only case that
  proves the config is load-bearing.
- Phase 6 confirmed as deliberately un-validated rather than under-validated.

**Concurrency honesty:**
- Write-set disjointness for {2, 4} re-checked after this pass's edits: Phase 2 gained
  observability requirements but no new write paths; Phase 4 still writes only
  `DEP-DRIFT.md` and `dep-drift.sh`. **Still disjoint.**
- **The shared-state contract was mechanisms, not invariants** — "each works in
  `worktrees/<feature>/<repo>` on a `claude/<feature>*` branch" describes a wrapper, and a
  wrapper can be violated. Replaced with seven checkable invariants **G1–G7**, and the
  re-entry verification rewritten as a one-to-one table against them. This was the single
  largest defect Pass 3 found.
- No new parallel candidates. Phases 1, 2, 3, 5 share two files per repo; that is
  structural, not an artefact of ordering.

**Coherence:**
- The plan still answers its Problem Statement, and scope has not crept — Pass 3 added no
  phase and moved no work between phases.
- **Wiring gap found and closed:** Phase 1 builds a reusable workflow in `croft-pwa`, and
  its RED case would naturally have been run *there*. That is the isolation trap: the host
  would go green while 17 repos stayed unprotected. The entry point is now named as a PR
  **in a caller repo**, with `stellin` (no other CI) as the case that proves the call works
  where there is nothing to piggyback on.

**Documentation impact:**
- Section present, eight rows, every row assigned to the phase that makes its reference
  stale. No end-of-plan docs phase. Re-checked after this pass: the invariants and rollback
  added here do not create new doc obligations.

**Confirmed ready:** **yes**, as of the owner walk-through on 2026-08-29. All four open
questions are resolved — Q1 in Pass 2 by measurement, Q2/Q3/Q4 by owner decision with
their severities confirmed rather than agent-assigned. Nothing is left for the executor
to decide.

**Execution starts at Phase 0**, whose remaining scope is `CISS`, `croft-stack` and the JS
repos — `croft` is already done. The three unverifiable assumptions stand as the first
real work of the phases that depend on them: the org Actions policy (Phase 5, now with an
owner-approved `admin:org` refresh), the secret backlog in 10 unscanned repos (Phase 1,
and the one thing that could invalidate this plan's premise), and private-repo Actions
minutes.

---

## Review Log — entry 7: executing Phase 2 (2026-08-29)

**Two deviations from the plan as written, both because a measurement replaced a guess.**

- **Enforcement is ON everywhere from the first commit**, not "advisory everywhere,
  blocking on enforcing surfaces". The staging existed to protect against a backlog, and
  the plan said so honestly. Before writing any CI, the gate was run across all 18 repos:
  **zero blocking findings in every one**. There is no backlog to stage around, and an
  advisory gate nobody has to fix is how a gate becomes decoration. The staged version
  would have been insurance against a risk that had already been measured away.
- **A new caller input, `advisory-paths`.** Phase 2's design assumed rung 2 could be read
  off each ecosystem's lockfile. For npm it can (`dependency_groups`), for Gradle it can
  with work (the lockfile's own configuration list — osv-scanner reports null there), and
  for Cargo and PyPI it cannot at all. But `discovery` is 52 of the workspace's 53
  lockfiles and almost all of them are frozen research that ships nothing: **215 blocking
  findings without the input, 0 with it.** "This subtree publishes nothing" is a claim
  about the repo, so it belongs with the caller rather than in the shared config.

**What Phase 2's validation table got right.** All four named cases behaved exactly as
specified, run in scratch repos so no shared checkout was touched. The two that mattered
were the third and fourth — removing `RUSTSEC-2026-0212` from `croft`'s `osv-scanner.toml`
and putting it back — because they are the only ones that distinguish "the exceptions file
is load-bearing" from "this repo happens to be clean". Writing them into the plan at Pass 3
is the reason they were run.

**What the plan did not anticipate, in rising order of how much it should have.**

1. *osv-scanner's directory scan finds nothing here.* `scan source -r .` on 2.3.5 walks
   `/`, visits one inode and reports "No package sources found". The gate enumerates
   lockfiles from `git ls-files` and passes them with `-L`, which also makes the
   empty-set case impossible to mistake for a clean one.
2. *Config discovery does not walk up.* `croft/osv-scanner.toml` never applied to
   `croft/android/app/gradle.lockfile`; its 43 advisories had been arriving unfiltered.
3. *Five CI-only defects.* A checksum that could not find its own file; a transient
   `curl (35)` with no retry; and the one worth carrying forward — **omitting `ref:` on a
   cross-repo checkout does not fall back to the reusable workflow's ref.** It defaults to
   `github.ref` of the *current* repo, so it works in the host and silently checks out an
   unrelated branch in all seventeen callers. Measured from a caller on runner 2.336.0:
   `github.job_workflow_sha` is **empty**, despite GitHub documenting it as the reusable
   workflow's SHA.
4. *The gate had a hole of its own.* `requirements.txt` was not in the enumerated
   filenames, so `site/requirements.txt` — one pinned line — went unscanned and carried
   GHSA-5wmx-573v-2qwq at CVSS 7.5. **This is the finding worth the most.** It was not
   caught by any test, review or validation case; it surfaced only from reconciling a
   lockfile count between a terminal (53) and a CI log (52). Every check in this plan
   asks whether a finding is real. None asked whether the *scope* was complete, and a
   gate's scope is exactly as checkable as its verdicts. The fix is a tested tuple; the
   lesson is that "what does this not look at?" belongs in a validation table beside
   "what does it decide?".

**Documentation impact:** the Phase 1 and 2 rows are closed —
`SUPPLY-CHAIN.md` § Current state, `croft-pwa/docs/CI.md` (a new rule 9, which Phase 1 had
listed and not written), both CHANGELOGs, and the audit's checks 31/32. Two rows Phase 1
left open were closed here rather than left to rot.

## Review Log — entry 8: executing Phase 3 (2026-08-29)

**Outcome: licences land as part of the dependency gate, not beside it — and the phase's
single biggest finding is about the *check*, not the dependencies.**

**Deviation 1 — the licence gate is not a separate mechanism.** The plan implied a licence
check alongside the SCA one. Measurement said they are one question: a CVE matters if the
vulnerable code ships, and a licence term attaches if the licensed code is **distributed**.
So rung 2 decides both, out of a single `osv-scanner --licenses --all-packages` run
(verified not to change which vulnerabilities are reported).

This is load-bearing rather than tidy. **All 38 Maven licence violations in `croft/android`
are unshipped**, the LGPL-2.1 `jna` included — and the `jna` that actually ships is 5.14.0,
reporting a compatible licence. A licence gate without rung 2 blocks a client release on
the emulator-control plugin's licence: rule 4's failure wearing a hat. Across the
workspace, **40 licence violations, all unshipped, zero per-package exceptions needed**.

**Deviation 2 — the allowlist is a script constant, not a workflow input.** One outbound
licence means one inbound list. A per-caller override would let the repo with the loosest
list become the one others copy from — the failure that made croft-pwa's MIT relicensing
necessary in the first place.

**Deviation 3 — the plan's suggested RED fixture was wrong.** It proposed `readline-sync`
as "GPL-3.0"; it is MIT. Two further guesses also missed. The fixture used is
`ffmpeg-static`, which osv-scanner *reports* as GPL-3.0-or-later — chosen by asking the
scanner rather than by recalling a licence.

### Findings the plan did not anticipate

1. **Check 35 could not see the violation it exists to catch.** It graded the repos
   checked out under `CroftC/` — 18 of the org's 24 — while `k1-appa`, `k1-appb` and
   `kernel-k1` had been public with no LICENSE since July. They were found by an org-wide
   `gh api` sweep during the rollout, not by the check that owns the rule. **This is the
   finding worth the most in this phase, and it rhymes with entry 7's:** that one was a
   gate whose *scope* was incomplete; this is a check whose *population* was a convenient
   subset. Both report green by construction, and neither is caught by asking "is this
   finding real?". The fix is a generated roster (`bin/org-register.sh` → `ORG-REGISTER.md`)
   covering all 24, proven RED on this morning's state and GREEN on today's.

2. **The GREEN half of a validation found a defect the RED half could not.** Removing the
   offending dependency left a valid lockfile with zero packages, and the gate reported
   "could not run". osv-scanner exits **128** for "no package sources found" and **127**
   for a rejected argument — both with **empty stdout**, so only the code separates them.
   Collapsing them into "not 0 or 1, so broken" fails any repo that legitimately has no
   dependencies. Had the validation stopped at "it refuses the bad thing", this would have
   shipped. *Watch it fail* and *watch it pass* are two requirements, not one.

3. **One tool disagrees with itself about what SPDX is.** deps.dev **reports** `MPL-2.0+`;
   osv-scanner's `--licenses` **validator refuses** it as non-SPDX — and on that refusal
   writes to stderr, emits no JSON and **exits 0**. A gate that trusted the exit code
   would have reported green having scanned nothing. Handled in the gate, and the
   empty-set guard now names this as the cause to look for.

4. **A finding stated something false.** Check 35 called `experiments` "a public repo"
   while it is private *and* archived. Worse than a missing finding: an untrue one that a
   reader must disprove. Private and archived repos are now a recorded exemption.

5. **Mutation testing earned its place again.** 12 of 13 mutants killed; the survivor that
   mattered relaxed the first-party check from the path segment `/CroftCommunity/` to a
   bare substring, which would launder a fork *named* `CroftCommunity` into first-party and
   wave its `UNKNOWN` licence through. Review did not catch it; the mutant did. The one
   remaining survivor (dropping `^…$` anchors from the Cargo.lock source regex) is triaged
   **equivalent** — TOML keys sit at column 0, so both forms select the same text on any
   lockfile Cargo can emit, and killing it would mean inventing input Cargo never produces.

### Deferred to the owner

**Whether the AGPL-compatible copyleft family belongs in the allowlist.** `GPL-3.0-or-later`
and `LGPL-3.0-*` are compatible with an AGPL-3.0 outbound licence (GPLv3 §13), so blocking
them is arguably a false positive — and rule 7 warns that a list which trips on a false
positive gets widened wholesale. They are omitted today because **no package in the
workspace carries them**, and the strictly-measured list is the one that can be defended.
The consequence is that a future copyleft dependency stops for a human decision, which is
a defensible thing for a gate to do. Widening is a one-line named PR either way.

**Documentation impact:** Phase 3's row is closed — `SUPPLY-CHAIN.md` rule 7 and § Current
state, the checks table (35 is now FLAG and grades the org), `.claude/CLAUDE.md`'s
dimension row, `croft-pwa/docs/CI.md` rule 9 and CHANGELOG. `ORG-REGISTER.md` is a new
generated register and is listed as such.

## Review Log — entry 9: executing Phase 5 (2026-08-29)

**Outcome: 109 Action references pinned across 12 repos, `sha_pinning_required` on for all
22 non-archived repos, 18 of 18 green under enforcement — and the phase's central unknown
was settled by measurement before anything was changed.**

**The probe that decided the phase's shape.** The plan did not say whether GitHub's
`sha_pinning_required` would also refuse the reusable-workflow call every repo makes at
`@main`. If it did, the shared gate built in Phases 1–3 would have had to be pinned and its
eighteen callers bumped on every change — the exact cost one shared workflow exists to
avoid. Rather than reason about it, the setting was switched on for `stellin` (one workflow,
one `uses:`) and a throwaway PR added a floating tag. GitHub answered in its own words:

> `The action actions/checkout@v4 is not allowed in CroftCommunity/stellin because all
> actions must be pinned to a full-length commit SHA.`

— while the `@main` reusable call in the same repo ran **green**. The policy governs
**actions**, not reusable workflows. The exemption is now encoded in check 33 with that
measurement as its reason, rather than as a preference.

**The order the plan insisted on was the right one.** Tags first, setting second. Prior
values were recorded for every repo before any write (`false` everywhere), and the read-back
confirmed the diff was exactly `sha_pinning_required: false → true` with `enabled` and
`allowed_actions` untouched — the plan's inverted G6 check. Validation was not "the setting
reads true": the security workflow was dispatched across the workspace afterwards and
**18 of 18 completed green under enforcement**.

### Findings the plan did not anticipate

1. **The population was larger than the set being graded — for the third time.**
   `levelforge`, a public repo nobody has cloned here, carried six floating tags and had to
   be pinned before the flip could include it. Entry 8 found the same shape in check 35 and
   entry 7 in the gate's own scope. Three instances of one failure mode is a pattern, not a
   coincidence: **every check in this dimension was written against the 18 repos on this
   laptop, and the org has 24.** The org register now closes it for checks 33, 34 and 35.

2. **And it was still open in Phases 1 and 2.** `levelforge`, `k1-appa`, `k1-appb` and
   `kernel-k1` had **no secret or dependency gate at all** — a rollout that reported itself
   complete had covered 18 of 24. Fixed the same session (four callers, all green), but the
   lesson is that "complete" was measured against the wrong denominator twice before anyone
   noticed.

3. **Check 33 was reporting where the forge can refuse.** Reading `uses:` lines proves the
   tree is pinned *today*; it cannot stop the next PR, and the audit runs when someone
   remembers to run it. That is the same distinction rule 1 draws between GitHub's secret
   *alerts* and a blocking scan — and this dimension had been on the wrong side of it for
   its own rule 10. Check 33 now verifies the forge control as well as the file contents.

4. **`zizmor` found a template injection in the workflow eighteen repos run.** Both scanner
   versions were expanded with `${{ inputs.* }}` *inside* a `run:` block — the runner
   pasting caller-supplied text into the program before bash sees it. Moved to `env:`; 2
   findings before, 0 after, and the fix was verified in CI on a pinned throwaway branch
   because croft-pwa calls its own workflow at `@main` and a PR against the host exercises
   the old copy.

5. **`sha_pinning_required` is enforced TRANSITIVELY, and the flip broke five Pages
   deploys.** `actions/upload-pages-artifact@v3` is a **composite** action whose own
   `action.yml` uses `actions/upload-artifact@v4` — a floating tag. Pinning *our*
   reference to it is not enough; GitHub walks inside and refuses the job at setup. Five
   repos (`connect`, `discovery`, `fun`, `pdsview`, `view`) lost their Pages deploy the
   moment the setting went on. Fixed by bumping to `v5.0.0`, which pins its own
   dependency, and verified end to end — pdsview's and discovery's real Pages deploys on
   `main` both succeeded afterwards.

   **Two pieces of tooling failed to see it, and the enforcement is what caught it.** The
   pinning script reads a repo's own workflow files and cannot see inside a third-party
   composite action. The post-flip smoke test dispatched `security.yml` across all 18
   repos and reported *18 of 18 green* — a true statement about the wrong workflows,
   since none of them use this action. **"18 of 18 green" was evidence of the wrong
   thing**, which is the same failure as entry 7's incomplete scope and entry 9's
   convenient population, arriving this time as too narrow a *validation set* rather than
   too narrow a *grading set*. A sweep of all 22 pinned actions for unpinned internal
   references now exists and finds exactly this one.

### zizmor triage — 81 findings, 1.29.0, `--persona regular`

Recorded rather than silenced (the plan's Observability line for this phase):

| rule | count | verdict |
|---|---|---|
| `artipacked` | 43 | **Open, accepted for now.** `actions/checkout` leaves the token in `.git/config` unless `persist-credentials: false`. Real hardening, mechanical, and touches every workflow in the org — a change of that width deserves its own pass rather than riding along with the pinning one. |
| `unpinned-uses` | 17 | **Won't fix, measured.** These are the org reusable-workflow calls at `@main`. GitHub itself permits them (see the probe above), and pinning them would undo the shared gate. |
| `template-injection` | 8 error + info | **One fixed** (the shared gate, above). The rest interpolate `steps.*.outputs.pr` and `.sha` — a PR number and a commit SHA, both machine-generated — or run in maintainer-triggered release workflows. Open, low. |
| `excessive-permissions` | 8 | **Open, low.** Job-level `permissions:` narrower than the workflow default; worth a pass, not urgent. |
| `cache-poisoning` | 5 | **Open, worth a look.** Release workflows that restore a cache before building a published artifact. The only class here that touches what ships. |

## Review Log — entry 10: Phase 6, the authored-code pass (2026-08-29)

**This phase has no check and no green light, by construction** (`SUPPLY-CHAIN.md` rule 0):
an LLM reviewer cannot be proven RED on a fixture, so a check over it would report green
without meaning it. What follows is the reading, and what it changed.

**Rule 11 says the pass reads plans, not only diffs**, and the plan under review is this
one — a rollout that added three enforcing surfaces (a blocking dependency gate, a blocking
secret gate, and a forge-level Actions policy). The questions asked were about *trust
boundaries*, which is what a plan can be reviewed for before any code exists.

**1. What can a caller of the shared gate do to it?** Nothing, as it turns out, and that is
worth stating: no caller passes `secrets:`, the reusable workflow declares
`permissions: contents: read`, and it runs with the caller's own `GITHUB_TOKEN`. A
compromised caller cannot reach another repo through the gate.

**2. What stops a caller declaring everything unshipped?** `advisory-paths` is
caller-declared and unbounded — the widest trust boundary the gate has. The pass found that
a limit exists and **was accidental**: the match is a *directory* prefix, so a lockfile at
the repo root cannot be covered by any declaration. The root manifest, which describes what
the repo actually ships, is unsilenceable. Now pinned by test (croft-pwa
`AdvisoryPathsCannotSilenceTheWholeGate`), because an invariant nobody has stated is one a
refactor removes without noticing. **Residual risk accepted and recorded:** a caller can
silence a *nested* lockfile it should not; that is the input's purpose, it lives in the
caller's tracked file, and it is reviewed like any change.

**3. Where does the drift register's PAT live, and who can reach it?** `DRIFT_TOKEN` is a
`repo`-scoped PAT — the most powerful secret this rollout introduces. Containment checked:
`dep-drift.yml` triggers on `schedule` and `workflow_dispatch` only, **never
`pull_request`**, so a fork or a branch cannot run it; the token is bound to the two steps
that need it, so the step running repo test code cannot read it; and reaching the token
requires landing on `main` through a PR. Accepted. The one thing worth saying plainly: a
malicious change to `.claude/bin/dep_drift.py` that lands on `main` would run with an
org-wide read token, so that file's reviews are not routine.

**4. Does anything here fail open?** The three refusals were each exercised rather than
assumed: the licence/vulnerability gate exits 2 when the scanner returns no packages, the
drift generators refuse to publish a roster smaller than the committed one (proven — a
21-of-24 token was caught in a real CI run), and the audit refuses to run against a root
with no sibling repos. The pattern this rollout kept rediscovering is that **failing open is
never loud**, so each of those was made to fail closed and then watched failing.

**No finding rose to blocking, which is what rule 0 predicts and not evidence of anything.**
The pass is a habit, not a gate.
