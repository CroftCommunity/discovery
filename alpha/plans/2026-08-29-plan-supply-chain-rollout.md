# Plan — rolling out the supply-chain dimension

**Status:** Pass 1 + Pass 2 complete. Not started. Pass 3 not run.
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

**Shared-state contract (all phases):** no phase runs `git checkout`, `git restore`,
`git reset` or `git clean` in a shared checkout; each works in
`worktrees/<feature>/<repo>` on a `claude/<feature>*` branch; no phase binds a port; no
phase writes outside its own repo's tree except Phase 4, which writes only
`CroftC/.claude/DEP-DRIFT.md`. **Ambient state actually touched:** GitHub org settings
(Phase 5 only — repo Actions permissions), and the network (all scanning phases).

---

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
- **Validation:** **the gate must be watched to fail.** Plant a test secret in a PR,
  confirm red at the gitleaks step, revert, confirm green. Tests are not the floor here —
  a CI gate nobody has seen fail is indistinguishable from one that is not wired
  (`CI-PATTERN` "verify it bites").
- Scan the **PR commit range**, not `HEAD`. Pin `gitleaks v8.30.1`. Allowlist the two
  known-benign findings by path.

### Phase 2 — Dependencies, advisory everywhere, blocking on enforcing surfaces

- **Depends on:** Phase 0, Phase 1 (shares the workflow file).
- **Read-set:** all lockfiles; `SUPPLY-CHAIN.md` rule 5.
- **Write-set:** the same per-repo workflow + gate target; `<repo>/TODO.md` as items close.
- **Re-entry verification:** `make security` (or `npm run security`) exits 0 locally and
  in CI.
- **Validation:** a deliberate downgrade to a known-vulnerable version turns the gate red
  in `croft`; the *same* downgrade in a static-site repo produces a NOTE and a green
  build. Both directions, or the production-path rule is unproven.
- Blocking first in `croft`, `CISS`, `croft-stack`. Weekly `schedule:` — a new advisory
  lands against untouched code, so a PR-only trigger never fires on quiet repos, which
  are exactly the drifted ones.

### Phase 3 — Licences, one allowlist

- **Depends on:** Phase 2 (same job).
- **Write-set:** per-repo gate target; `k1-appa`/`k1-appb`/`kernel-k1` `LICENSE`.
- **Re-entry verification:** audit check 35 silent.
- **Validation:** a deliberately added GPL-2.0-only dependency is refused; `UNKNOWN`
  entries are resolved by name in the config, not blanket-ignored.

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
- **Re-entry verification:** audit check 33 silent; `zizmor` exits 0.
- **Validation:** `sha_pinning_required` reads `true` via the API on each repo, and a PR
  introducing a floating tag is refused by GitHub itself. Prefer the native setting over
  manual pinning where it works — see Review Log R3.

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

**Pass 3 — not run.** Quality gates (TDD ordering, diagnostic logging, debugging
readiness, validation calibration) have not been applied to this plan.
