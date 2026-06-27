# Handoff: Pass 3 + execute the integrated Drystone CLI

Copy everything below the line into a fresh Claude Code session at the CroftC
workspace root (`/Users/cpettet/git/chasemp/CroftC`). It is self-contained.

---

You are picking up an integrated build for the Croft/Drystone project. The plan is
written and has been through Pass 1 + Pass 2 of the phase-plan skill. Your job:
run **Pass 3** (quality gates) on it, then **execute Milestones A–C**.

## The plan

**Plan file:** `experiments/alpha/croft-chat/plans/2026-06-26-1-plan-integrated-drystone-cli.md`

Read it in full first — it carries all reasoning, Verified Assumptions (with
`file:line` evidence), a 20-phase decomposition grouped into Milestones A–D, a
Concurrency Map, Documentation Impact, Open Questions, and a Review Log.

What it builds: a `ratatui` two-pane CLI chat that demonstrates the Drystone
protocol, architected around the protocol/implementation boundary —
`social-graph-core` (tenant-agnostic substrate facade over the redb
`local_storage_projection` crate) + `group-chat-core` (chat as one tenant) +
`croft-chat` (TUI shell). Convergence is proven locally first (a shared-dir
transport adapter), then over real iroh-gossip across the four test nodes.

## The phase-plan skill

Use the skill at `/Users/cpettet/git/chasemp/coding-agents/skills/phase-plan/`.
Read `phase-plan.md` (the entry point: plan-doc template, guardrails) first, then
the per-pass files as you reach them.

## Step 1 — Pass 3 (quality gates)

Read `skills/phase-plan/pass3.md` and apply it to the plan. Pass 3 layers quality
gates on top of the existing plan **without rewriting it** (extend, don't
restructure; every change gets a Review Log entry). Focus on:

- **TDD ordering** per phase — confirm each phase names a failing test first and
  the wiring test is RED→GREEN. The project's TDD discipline is non-negotiable
  (rust-enforcer: no `unwrap`/`expect` in production, `#![warn(missing_docs)]`,
  `clippy::pedantic`).
- **Observability / diagnostic logging** — especially the iroh phases (P16–P18)
  and the convergence/contradiction proofs (P7, P20): what gets logged to debug a
  failed convergence or a NAT/relay problem.
- **Validation calibration** — confirm each phase's Validation tier matches its
  risk (the iroh and fold-touching phases are Broad).
- **Documentation Impact coverage** — confirm every added/removed file has a home
  phase.

Then walk the user through any new/revised Open Questions (brief listing first,
one-at-a-time confirm unless they say "accept all as recommended"), add a
**Pass 3 Review Log** entry, and report. Pass 3 is analysis only — **do not write
code during Pass 3.**

Carry these already-locked decisions (do not re-litigate): three-layer core split;
`ratatui` two-pane TUI; local-convergence-first then iroh; substrate gaps fixed
in place. Open Questions currently recommended (none BLOCKING): async↔sync iroh
bridge (PHASE-GATED P16); substrate auth/governance coverage (PHASE-GATED P20);
iroh dependency-crate collision (ADVISORY); channel model depth (ADVISORY).

## Step 2 — Execute Milestones A–C

After Pass 3 is confirmed, read `skills/phase-plan/execute.md` and execute the
plan **phase by phase, P1 through P18** (Milestones A, B, C). Milestone D
(P19–P20, four-node + hard-stop) is planned but **out of this execution scope** —
stop after P18 (2-node convergence over iroh) and report.

Execution rules (from the skill + project conventions):
- **TDD, no stubs.** Failing test first; minimum code to green; the wiring test
  proves the call chain is live, not just that a unit works.
- **Commit at every stable point** — one commit per phase, never batch. Git
  identity for this path is **chasemp**: `"Chase Pettet" <chase@owasp.org>`, SSH
  host `github-personal`, `gh auth switch --user chasemp` if using `gh`. End
  commit messages with: `Co-Authored-By: Claude Opus 4.8 (1M context) <noreply@anthropic.com>`.
  **Push only when the user asks.**
- **Substrate fixes touch the mutation-vetted crate** (`local_storage_projection`,
  phases P2/P3/P13). After each, re-run the mutation discipline on the touched
  functions and update `local_storage_projection/MUTATION_TESTING.md`'s survivor
  ledger. The cargo-mutants config + run script are already staged on the remote
  box (see "Mutation testing" below).
- **Parallel set {P4, P6}** is the only opt-in concurrency (disjoint crates); the
  rest is sequential. Default to sequential unless the Concurrency Map says
  otherwise.
- **No assumed behavior** — the plan's Verified Assumptions cite `file:line`; if
  something differs mid-phase, stop and update the plan (add a Review Log entry)
  before continuing.

Key references the plan relies on (already verified):
- Substrate consumer API: `local_storage_projection/src/surface.rs` (LocalStore);
  the lamport bug at `surface.rs:1341-1343`; message-body read gap (no public
  `get_message`); channel-routing limitation at `fold_derived.rs:1128-1135`.
- Transport port to reuse: `croft-group/.../croft-chat-cli/src/transport.rs:43-53`.
- iroh: pinned `=1.0.0`; verified API in
  `experiments/alpha/iroh/relay-lab-runs/IROH-1.0.0-API-VERIFIED.md`; working
  reference `experiments/alpha/iroh/crates/altdrive-spike-faithful-sync`; endpoint
  builder `relay-loadtest/src/node.rs:33-76` (drop the insecure TLS).
- Four-node topology facts (no config file yet — P17 authors `stone-alpha.toml`):
  `RELAY-LAB-CONCLUSIONS.md:40-45`; secroute-testing-one `54.172.175.109`,
  secroute-testing-two `34.207.146.151`, node3 `172.31.88.18` (internal only),
  node4 = this Mac (NAT); **SG opens UDP 2112 only**; workstation reaches boxes
  via relay; `EndpointAddr` JSON exchanged out-of-band.
- SSH driving the boxes: use the detached-subshell pattern for long-lived remote
  processes; no top-level remote `&`; avoid `pkill -f <pattern>` where the pattern
  appears in the SSH command line (use a variable-concat split). See session
  memory `ssh-driving-secroute-sandbox`.

## Mutation testing (paused — re-run at end of implementation)

The cargo-mutants sweep on the substrate's auth/governance core was **paused** on
secroute-testing-one (`54.172.175.109`) at 55/269 to free the box for the build.
Partial findings: survivors cluster in `check_authorization` and
`apply_governance` (weak negative-path coverage — valid ops are tested to succeed,
unauthorized ops are not tested to be rejected). Snapshot saved on the box:
`~/mutants-run.PAUSED-2026-06-26.log` and `~/mutants-survivors.PAUSED.txt`.

**Re-run plan:** at the end of the implementation (after P18, or before the
Milestone-D hard-stop trust demo), re-run the full sweep against the final
substrate. The config and runner are staged on the box:
`~/croft-mutants/local_storage_projection/.cargo/mutants.toml` (skips heavy props)
and `~/run-mutants.sh` (`PROPTEST_CASES=8`, scoped to `fold_auth.rs` +
`governance.rs`). Re-sync the latest crate then relaunch:

```sh
rsync -az --delete --exclude target/ --exclude 'mutants.out*' -e ssh \
  experiments/alpha/local_storage_projection secroute-testing-one:~/croft-mutants/
ssh secroute-testing-one '( setsid bash -c "~/run-mutants.sh >\$HOME/mutants-run.log 2>&1; \
  echo DONE_\$? >>\$HOME/mutants-run.log" </dev/null >/dev/null 2>&1 & )'
```

Then triage survivors per `local_storage_projection/MUTATION_TESTING.md` (kill /
equivalent / accepted-risk), prioritizing the `check_authorization` /
`apply_governance` negative-path gap — that gap gates the Milestone-D hard-stop
trust claim (Open Question, PHASE-GATED P20).

## Done when

- Pass 3 has extended the plan with quality gates + a Review Log entry, and open
  questions are confirmed.
- Phases P1–P18 are executed TDD, each committed, each wiring test green; the
  2-node-over-iroh convergence (P18) is demonstrated.
- The mutation sweep has been re-run against the final substrate and survivors
  triaged.
