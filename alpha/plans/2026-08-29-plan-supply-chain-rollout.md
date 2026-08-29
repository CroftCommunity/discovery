# Plan — rolling out the supply-chain dimension

**Status:** Pass 1 + 2 + 3 complete. NOT ready for execution — four open questions await owner confirmation of severity (see Open Questions + Review Log § Pass 3). Not started.
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
2. **Whether `openmls` 0.9 adoption is scheduled here or in `croft`'s roadmap.** It
   retires most of croft's exceptions file but is an MLS stack upgrade with a device
   re-validation obligation. **Severity: low** — not on this plan's critical path.
3. **The five repos with no CI** — reusable-workflow caller, or one scheduled
   workspace-wide scan reporting centrally. **Severity: medium** — decides Phase 1's shape
   for 5 of 18 repos.
4. **Org-level vs per-repo `sha_pinning_required`.** Needs `admin:org`. **Severity:
   medium** — decides whether Phase 5 is one API call or twelve.

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

**Confirmed ready:** **no** — four open questions remain and the user has not confirmed
their severities. Q3 and Q4 are medium and shape Phase 1 and Phase 5 respectively; the plan
should not start execution before they are walked through.
