# Next experiment run — operator brief for Claude Code (unattended)

`Status: runnable brief, 2026-07-13. Written to be executed by a fresh Claude Code session in this
repo while the humans merge/review the spec-alignment branch. Scoped to what runs in THIS environment
(no secroute boxes, no macOS/iOS hardware, no real-NAT) so it can run unattended. Everything here is
sequenced by leverage and carries an explicit PASS/FALSIFY condition and a stop rule.`

You are continuing the Croft/Drystone experiment program. Orientation lives in `discovery/AGENTS.md`
and the three registers in this directory: `MASTER-INDEX.md` (ordering/critical path),
`EXPERIMENT-BACKLOG.md` (what's unrun), `SPEC-DIVERGENCE-REGISTER.md` (where a green rests on a
stand-in). The experiments↔spec picture is in `SPEC-ALIGNMENT-AND-ACTION-PLAN.md` +
`SPEC2-OVERLAY.md`. Read those first (they're short); do not re-derive facts they already establish.

---

## 0. Guardrails (read before touching anything)

1. **Branch discipline.** Do **not** commit on `main` or on `claude/experiments-spec-alignment-d3tnvu`
   (that branch is under human review/merge). Start fresh:
   ```
   git fetch origin
   git checkout -B claude/experiments-run-01 origin/main     # if the alignment branch is merged
   # if alpha/experiments/ is absent on main (not merged yet), base off the alignment branch instead:
   #   git checkout -B claude/experiments-run-01 origin/claude/experiments-spec-alignment-d3tnvu
   ```
   Push with `git push -u origin claude/experiments-run-01`. One commit per experiment. Do **not** open
   a PR unless asked.

2. **Never edit the reviewed spec.** `beta/drystone-spec/part-2-certifiable-design.md` and the other
   `part-*`/`conventions` files are under review. When an experiment earns a spec change, **append to
   the staging doc** `beta/drystone-spec/proposed-changes-2026-07-experiment-reconciliation.md` (add a
   new F-item or update an existing one's status), and add a row to
   `beta/impl/experiments/drystone-reviews-and-experiments-log.md`. Do not touch Part 2 itself.

3. **The honesty contract is the whole point.** If a test only goes green because of a stand-in
   (prototype, scaffold, weakened assertion, environment adaptation), you **MUST** (a) tag the site
   `SPEC-DELTA[<id> | <kind>]: … — Register: alpha/experiments/SPEC-DIVERGENCE-REGISTER.md` and (b)
   add a row to the register in the **same commit**. A green with a hidden stand-in is worse than a
   red. Distinguish modeled-vs-real and never over-claim. When in doubt, weaken the claim.

4. **Environment preflight.** Before starting, confirm what's available and record it:
   ```
   rustc --version           # expect 1.94+ (0.7-automerge and iroh builds need modern Rust)
   cargo mutants --version   # X3 sweep; install via proxy if missing: cargo install cargo-mutants
   ```
   The known-true env facts (from `MASTER-INDEX.md §5`): loopback iroh-gossip works
   (`croft-chat/localhost.toml`, `relay_mode = "disabled"`); the n0 relay is reachable over HTTPS but
   **Internet UDP is blocked** so there is **no real-NAT/holepunch path** — anything needing X1 or the
   boxes is out of scope for this run. If a tool won't install or a build won't complete via the
   proxy, mark that experiment **BLOCKED** in your report and move to the next one; do not burn the run
   fighting infrastructure.

5. **Stop rules.** (a) Time-box each experiment; if a build/run exceeds ~30 min of wall time with no
   progress, checkpoint what you have, mark it PARTIAL, and move on. (b) If an experiment requires a
   **design decision** (not just code) — e.g. the identity/key-recovery model, or which RBSR
   construction to build — **do not decide it**; write the options into the backlog and skip. (c) If
   you finish the queue, stop and report; do **not** invent new scope.

6. **Per-experiment deliverable.** Each experiment ends with: a short `REPORT.md` (or an appended
   section in the spike's existing report/ledger) stating method + result + PASS/FALSIFY verdict +
   any SPEC-DELTA; the backlog row updated (`✅`/`PARTIAL`/`BLOCKED`); the register updated if a
   stand-in was used; the proposed-changes doc updated if a spec status moved. Then commit.

---

## The queue (do in order; each is self-contained)

### EXP-1 — A4 / M1 fan-out (earns §11.11 measurement #1, fan-out half)  · runnable now

- **Spec claim under test:** §11.4/§11.5 "cost scales on the live set," and §11.11 measurement #1
  (per-commit **and fan-out** cost). The per-commit half is already measured (`mls-replant` M1); this
  earns the fan-out half.
- **Method:** bring up **N local `serve` processes** on the loopback gossip testbed (follow
  `croft-chat/RUN.md` "Same-host recipe" and `croft-chat/localhost.toml`). Measure, at
  **N = 2, 4, 8 (and higher if cheap)**: fan-out latency to convergence, total gossip message count vs
  live-N, and per-boundary re-key cost. Capture the curve, not a single point.
- **PASS:** a fan-out cost/latency curve is captured across the N values and its shape is stated
  honestly (is it flat, linear, log?), with the message-count-vs-live-N relationship recorded.
  **FALSIFY / flag:** cost grows super-linearly in live-N (would contradict the "scales on live set"
  posture) — if so, that's a finding, report it prominently.
- **Deliverable:** a fan-out section in `mls-replant`'s or `croft-chat`'s report/ledger; update
  `EXPERIMENT-BACKLOG.md` M1 fan-out row; in the proposed-changes doc, move **F4** from "half-earned"
  toward "earned" (or record what's still missing). Note any measurement stand-in as a
  `proxy-measurement` register row.

### EXP-2 — Automerge 0.7 confirmation (retires the `automerge-0.6.1` proxy, §7.7)  · runnable now

- **Spec claim under test:** §7.7/§7.9 late-joiner partial-reconstruction inertness on the **0.7 ship
  target** (proven only on 0.6.1 today).
- **Method:** in `automerge-partial-reconstruction/`, bump `Cargo.toml` to `automerge = "0.7"`, apply
  the **two API deltas the README already documents**, re-run `src/main.rs`'s four scenarios.
- **PASS:** all four invariants hold on 0.7 (a node given only later-epoch changes with deps withheld
  holds them **inert**). **FALSIFY:** any scenario behaves differently on 0.7 — that is a load-bearing
  finding for the late-joiner design; report it and do **not** paper over it.
- **Deliverable:** update `automerge-partial-reconstruction/REPORT.md`; move the register row
  `automerge-0.6.1` from "Already-declared caveats" to **Reconciled** (with the 0.7 evidence) **iff**
  it passes; update the backlog item.

### EXP-3 — X3 cross-package mutation sweep on the auth/governance path  · runnable now

- **Spec claim under test:** §7.3 / §8.2(g) — the authority/threshold trust claim (a surviving mutant
  in `fold_auth`/`governance` = a real hole). Also raises F1's RuleChange-enforcement status from
  `Modeled` toward mutation-`Verified`.
- **Method:** run `cargo mutants` on `local_storage_projection` targeting `fold_auth` + `governance`.
  The known hard part (from the register): the **positive-path coverage lives cross-package in
  `croft-chat`**, so a substrate-only sweep under-counts — configure the sweep to include the
  cross-package tests, or run it in both packages and reconcile. Budget the slow substrate suite
  (scope to the auth/governance files, don't sweep the whole workspace).
- **PASS:** no surviving mutants in the authority/threshold path — **or** a clean list of survivors,
  each triaged as (real hole → file an issue-row in the backlog) vs (equivalent mutant → note why).
  **FALSIFY:** surviving mutants in threshold-counting or approval-subject logic — high-value finding.
- **Deliverable:** a mutation-sweep report under `local_storage_projection/`; update backlog X3; if
  the RuleChange approval-subject path is now mutation-clean, note it against **F1** in the
  proposed-changes doc (status `Modeled` → `Verified` candidate).

### EXP-4 — Fold open items (small, spec-adjacent)  · runnable now

- **Spec claim under test:** §7.6.1 (concurrent contradiction) and adjacent fold behaviors.
- **Method:** pick up the *unblocked* fold open items from `EXPERIMENT-BACKLOG.md §2` /
  `MASTER-INDEX.md A1`: **per-act approver-role granularity**; **two-competing-quorums → §7.6.1
  contradiction** (build the case, assert the hard-stop fires without false-tripping);
  **contradicted-group byte-head naming**. Skip the "catching up… TUI indicator" (UX, lower value
  unattended). Each gets a focused test.
- **PASS:** each item has a RED→GREEN test; the §7.6.1 contradiction case hard-stops and no benign
  case false-trips. **FALSIFY:** a competing-quorum case auto-resolves instead of hard-stopping —
  that would contradict §7.6, report it.
- **Deliverable:** tests in `local_storage_projection`; update backlog A1 rows; if the two-competing-
  quorums behavior confirms §7.6.1 as specified, add a corroboration note to the proposed-changes doc.

### EXP-5 (larger; only if the queue above is clear and time remains) — MLS key-distribution + threshold-revoke over the wire

- **Spec claim under test:** §10.2/§10.3 realization + §9 conformance cats **7/8/9** (currently
  `not_yet_emitted`), and the two standing honesty boundaries (the modeled verifying-key registry; the
  MD-G5 sha-256 MAC revoke stand-in).
- **Method:** in `iroh/`, replace the **modeled** verifying-key registry with a real **over-iroh**
  key distribution, and the sha-256 MAC revoke stand-in with a genuine **k-of-n threshold signature**
  over the wire; then emit conformance vectors for cats 7/8/9 and the revoke-authority vector (today a
  `PLACEHOLDER`).
- **PASS:** vectors 7/8/9 emit and verify; the revoke path is real k-of-n, not a MAC. **FALSIFY / flag:**
  if the over-wire distribution needs a design decision (key discovery, trust root) you cannot make
  autonomously, **stop and write the decision into the backlog** — do not improvise a trust model.
- **Deliverable:** update `iroh` conformance-core + TEST-LOG; update backlog §6d; against **F7** in
  the proposed-changes doc, record which conformance cats are now emitted. This is the one queue item
  most likely to hit a design gate — treat the gate as a stop, not a puzzle.

> **Not in this run (out of environment scope or needs a human decision):** X1 real-NAT convergence
> (needs the boxes); iroh Spikes 3/7 (macOS/iOS hardware); B1 dataplane hash structures → A5 message
> continuity (a design→build large enough to want its own run and plan); the identity/key-recovery
> model (the program's biggest open **design** problem — decision-gated). If you believe B1 is worth
> starting, write a plan into `alpha/experiments/` and stop — do not build it unattended.

---

## Wrap-up (end of run)

Write `alpha/experiments/RUN-01-SUMMARY.md`: one line per EXP with its verdict (PASS / PARTIAL /
FALSIFIED / BLOCKED), the commit that carries it, any new `SPEC-DELTA` rows, and which
proposed-changes F-items moved. Update `MASTER-INDEX.md` / `EXPERIMENT-BACKLOG.md` so the next session
sees current state. Push the branch. Do not open a PR; leave a note in the summary that the branch is
ready for review. Then stop.

`The single rule above all: a result is only worth what its honesty makes it. Green on a stand-in,
tagged and registered, is a real contribution; green on a hidden stand-in is a liability. Tag it,
register it, or weaken the claim.`
