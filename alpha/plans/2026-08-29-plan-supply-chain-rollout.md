# Plan — rolling out the supply-chain dimension

**Status:** proposed, not started. Phase 0 partially complete (croft only).
**Standard:** `CroftC/.claude/SUPPLY-CHAIN.md` (landed 2026-08-29, `c6ff383`).
**Scope:** 18 checked-out repos, 24 in the org.
**Owner decisions already taken (2026-08-29):** free tools over paid; the gate blocks on
production-path findings and NOTEs the rest; freshness budget NOTE at ≥1 major behind,
FLAG at ≥2; one outbound licence (AGPL-3.0) everywhere.

---

## Problem statement

The workspace has a supply-chain standard and **zero enforcement of it**. That gap is not
theoretical, and the sweep that produced the standard measured its cost:

- **No repo scans for secrets.** GitHub's free scanning covers public repos only and
  *alerts* rather than blocks — so the two repos that actually hold credentials
  (`croft-stack`'s mint key, `CroftC`'s `.env`) are the two it does not reach.
- **No repo scans dependencies.** Dependabot exists in two repos, covering GitHub Actions
  in one and a single Cargo directory in the other. No JavaScript ecosystem is scanned
  anywhere.
- **Nothing validates licences,** inbound or outbound. Four public repos were publishing
  under default copyright — all rights reserved — until 2026-08-29.
- **Nothing tracks drift,** and drift is what makes the fixes expensive: every fixable npm
  advisory in this workspace requires a *major* bump (`vite` 5→6, `vitest` 2→3 at CVSS
  9.8). Nobody deferred a security fix; they deferred a version bump and it became one.
- **The audit FLAGs 54 findings** against the new checks 31–35 and nothing acts on them.

A standard with no gate decays into prose — `PATTERN.md` says so explicitly, and this
dimension has more surface to decay across than any other.

**What this plan is not.** It is not a vulnerability-remediation plan. The one repo whose
advisories were investigated in full — `croft` — resolved to **zero reachable** after the
rule-5 ladder was applied, and produced an exceptions file rather than a code change. The
problem here is the absence of *machinery*, not a backlog of exploitable bugs, and
conflating the two is how a rollout acquires false urgency.

---

## Approach

Six phases. Each is cheap **because** the one before it ran, and that dependency is the
reason for the ordering rather than a preference.

### Phase 0 — Baseline and exceptions, before any gate blocks

Record today's findings as the starting line, each with a reason and an expiry.

- Per-repo `osv-scanner.toml` and `.gitleaks.toml` where findings exist.
- Every entry dated and expiring (`SUPPLY-CHAIN.md` rule 9).
- **Done for `croft`** (`osv-scanner.toml`, nine entries, scan clean with it and nine
  vulnerabilities without). Remaining: `CISS`, `croft-stack`, and the JS repos.

**Acceptance:** `osv-scanner scan source -r .` exits clean in every repo, with every
suppression carrying a reason a stranger could audit.

### Phase 1 — Secrets, blocking immediately, everywhere

`gitleaks` in CI on `pull_request` + `push:main`, plus a weekly full-history run and a
pre-commit hook.

- Scan the **PR commit range**, not `HEAD` — a credential added and reverted inside a
  branch has still leaked.
- Allowlist the two known-benign findings by path with a reason: `CISS`'s test fixture
  SSH key, and the false positive in `discovery` prose.
- Pin the `gitleaks` version; never `@latest` (`CI-PATTERN` rule 7).

**Acceptance:** every repo's gate fails on a planted test secret at the expected step,
then goes green on revert. A gate nobody has watched fail is indistinguishable from one
that is not wired.

### Phase 2 — Dependencies, advisory everywhere, blocking on the enforcing surfaces

`osv-scanner scan source -r .` in the same job.

- **Blocking first in `croft`, `CISS`, `croft-stack`** — the surfaces that admit, refuse
  or hold keys.
- Advisory (NOTE) elsewhere for one cycle, then escalate per `PATTERN`.
- A **weekly `schedule:`** matters more here than anywhere else: a new advisory lands
  against code nobody touched, so a PR-only trigger never fires on quiet repos — which
  are exactly the drifted ones.

**Acceptance:** a deliberate downgrade to a known-vulnerable version turns the gate red in
`croft`; the same downgrade in a static-site repo produces a NOTE and a green build.

### Phase 3 — Licences, one allowlist

Now single, since the AGPL unification removed the MIT outlier.

- Deny: SSPL, BUSL, Elastic, CC-BY-NC, and **GPL-2.0-only** — genuinely incompatible with
  AGPL-3.0 and the one routinely missed.
- `UNKNOWN` resolved **by name**, never blanket-ignored.
- Close the outbound half: `k1-appa`, `k1-appb`, `kernel-k1` are still public with no
  LICENSE (audit check 35).

**Acceptance:** check 35 clean; a deliberately added GPL-2.0-only dependency is refused.

### Phase 4 — The freshness register in CI

`bin/dep-drift.sh` runs weekly and commits the regenerated `DEP-DRIFT.md`; audit check 34
reads it.

- Extend it to Rust (`cargo-outdated`) and Gradle, which it currently declares
  **unmeasured** rather than silently skipping.
- Current state: `croft-pwa` 7 majors behind, `fun` 7, `view` 6, `bluebird` 6 — all four
  over the FLAG threshold on day one.

**Acceptance:** the register regenerates unattended and check 34 FLAGs those four.

### Phase 5 — The CI supply chain itself

- SHA-pin Actions in the 12 repos that still resolve through movable tags (`arecipe` and
  `greetings_site` are done), so the existing Dependabot configs finally have SHAs to bump.
- Add `zizmor` to the gate command.

**Acceptance:** check 33 clean; `zizmor` green or its findings triaged.

### Phase 6 — The authored-code pass, advisory forever

- Plan review with `/security-review` before building anything touching an enforcing
  surface — the axis neither scanner covers, and the cheapest place to catch a
  trust-boundary error.
- Optionally the `anthropics/claude-code-security-review` Action on the enforcing repos.
- **Never a gate**, never a check (`SUPPLY-CHAIN.md` rule 0 and the checks table).

**Acceptance:** none, by construction. This phase is a habit, not a gate, and pretending
otherwise is the failure it exists to avoid.

---

## Reasoning

**Why staged rather than all at once.** Blocking gates switched on against an unrecorded
backlog get disabled within a week, and a disabled gate is worse than none because it
reads as coverage. Phase 0 exists so that every later phase starts from green.

**Why secrets first.** It is the cheapest phase and closes the widest hole. The backlog
was *measured* at two benign entries across the full history of eight repos, so the
allowlist is two lines. That window closes as the workspace grows: a scanner adopted at
zero backlog is a gate; adopted at two hundred findings it is a permanent mute.

**Why enforcing surfaces before static sites.** Same shape as the enforce flip — prove the
mechanism where it matters, then widen. It also puts the strictest gate where the
production-path rule has already been exercised.

**Why the production-path rule needs the rule-5 ladder wired in, not applied by hand.**
The Android scan produced 43 advisories, 19 High, of which **zero reach the APK**. A
severity-only gate would have blocked a client release on netty CVEs in the
emulator-control plugin. The reachability determination is what makes the gate survivable,
so it belongs in the gate.

**Why a reusable workflow rather than 18 copies.** Five repos have no CI at all
(`stellin`, `crofting_site`, `arecipe_treatise`, `homebrew-tap`, and the frozen
`experiments`), so universal secret scanning otherwise means authoring five workflows for
repos with nothing else to run. A single reusable workflow called with three lines per
repo keeps the pinned scanner versions in one place — which is also where rule 7 wants
them. **Recommended home: `croft-pwa`**, which already owns the CI standard
(`docs/CI.md`). The cost to name: `croft-stack` is private and calling across visibility
boundaries needs the org's Actions settings checked before Phase 1 depends on it.

**Why drift is its own phase and not folded into Phase 2.** It is the only signal here
that fires *before* a vulnerability exists, and it is advisory, so it can land without the
gate-biting ceremony the blocking phases need.

**What could make this plan wrong.** If Phase 1 finds more than a handful of real secrets,
the "adopt at zero backlog" premise collapses and Phase 1 becomes a remediation project
that should be planned separately rather than absorbed. The measurement says otherwise
today; it was taken on eight of eighteen repos, and the other ten are unmeasured.

---

## Open questions (to-be-planned, not proposable as work)

1. **Reusable-workflow home and cross-visibility calling** — recommended `croft-pwa`;
   needs the org Actions setting verified before `croft-stack` can call it.
2. **Whether `openmls` 0.9 adoption is scheduled here or in `croft`'s own roadmap.** It
   retires most of croft's exceptions file, but it is an MLS stack upgrade with a device
   re-validation obligation and does not belong to this plan's critical path.
3. **The five repos with no CI** — reusable workflow, or a single scheduled workspace-wide
   scan that reports centrally.
