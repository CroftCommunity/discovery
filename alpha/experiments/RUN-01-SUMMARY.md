# RUN-01 — summary (unattended experiment run)

`Branch: claude/experiments-run-01 (off origin/main). Date: 2026-07-14. Executed the queue in
alpha/experiments/NEXT-RUN-INSTRUCTIONS.md, scoped to this environment (loopback iroh, no secroute
boxes, no macOS/iOS hardware). One commit per experiment. No PR opened — the branch is ready for review.`

## Environment preflight (recorded)

- `rustc` / `cargo` **1.94.1** ✓ (0.7-automerge + iroh build fine).
- `cargo-mutants` was **not** installed; installed via the proxy → **27.1.0** ✓ (X3 unblocked).
- Loopback iroh-gossip works (no relay); Internet UDP blocked (no real-NAT path) — as `MASTER-INDEX §5`.

## Per-experiment result

| EXP | Verdict | Commit | One-line |
|---|---|---|---|
| **EXP-1** — A4/M1 fan-out | **PASS (with flag)** | `de82d2b` | Fan-out curve captured at N=2/4/8/16 over real iroh-gossip: per-node cost **linear** (`live_sent=2N+1`), aggregate O(N²), heads converge at every N. **Flag:** connect-time resync super-linear on the hub; full-settle doesn't complete past N≈8 (the open RBSR/steady-state gap). |
| **EXP-2** — Automerge 0.7 | **PASS** | `4938b7d` | All 4 partial-reconstruction invariants hold on the **0.7.4 ship target** (Rust 1.94.1), identical to 0.6.1. Retires the `automerge-0.6.1` proxy caveat. |
| **EXP-3** — X3 mutation sweep | **PASS (trust) / PARTIAL (mechanization)** | `cf6a8d1` | 120 mutants → 54 caught, 61 missed, 5 unviable. **Threshold-counting = 0 survivors**; all 61 survivors are cross-package-covered authorization-decision mutants (one hand-killed against the croft-chat test). **No real hole.** Automated cross-package harness remains open. |
| **EXP-4** — Fold open items | **1 PASS + 1 FALSIFIED + 2 deferred** | `ca4b53c` | Contradicted-group **byte-head naming = PASS** (`min(H(F),H(G))`, order-independent). **Two competing RuleChange quorums = FALSIFIED** — they auto-resolve order-dependently, no hard-stop (a real §7.6.1 gap). Approver-role granularity + the competing-quorum *fix* are design-gated (options → backlog). "Catching-up TUI" skipped per brief. |
| **EXP-5** — MLS/threshold-revoke over wire | **BLOCKED (design gate)** | `ee2f670` | **Stopped at the gate, not improvised.** Half 1 (key-distribution over wire) already realized in the `mls-welcome-over-iroh` spike; half 2 (real k-of-n threshold revoke) needs the **revocation-authority model** (I9). Options → backlog §6d-i. No conformance cats moved. |

## New SPEC-DELTA / register rows

**Active (introduced this run):**
- **`competing-quorum-autoresolve`** (weakened-assertion) — EXP-4. `competing_quorums.rs::two_competing_rulechange_quorums` pins the current order-dependent auto-resolution of two competing RuleChange quorums (no hard-stop) — a §7.6.1 gap; fix design-gated.
- **`fanout-single-run`** (proxy-measurement) — EXP-1. Fan-out curve is 1–2 runs/N, ±250 ms tick, star-bootstrap: shape robust, magnitude indicative.

**Reconciled (retired this run):**
- **`automerge-0.6.1`** (was proxy-measurement) → **Reconciled** — EXP-2. The 4 invariants now proven on the 0.7 ship target.

## proposed-changes F-items moved (`beta/drystone-spec/proposed-changes-2026-07-experiment-reconciliation.md`)

- **F4** (§11.11 #1): "per-commit measured, fan-out unearned" → **both halves measured** (fan-out earned in *shape*: linear per-node, O(N²) aggregate, heads converge; magnitude-open at hot-N=500+, and the super-linear resync flag consistent with F3). — EXP-1
- **F1** (§7.2 R7): mutation evidence added (threshold-counting mutation-clean; approval-subject cross-package-killed) supporting `Modeled`; boundary **sharpened** — R7 is enforced per-act but **not concurrent-conflict-aware** (competing quorums are an open §7.6.1 shape). — EXP-3, EXP-4
- **F7** (§10/§9): conformance cats 7/8/9 + revoke-authority stay `not_yet_emitted`, now with a **named blocker** (the revocation-authority decision) rather than open TBD. — EXP-5

No edits were made to Part 2 or any reviewed `part-*`/`conventions` spec file (guardrail #2). All spec
movement is staged in the proposed-changes doc; every experiment result is logged in
`beta/impl/experiments/drystone-reviews-and-experiments-log.md` (2026-07-14 RUN-01 section).

## Headline findings for the next session

1. **A real §7.6.1 gap (EXP-4, load-bearing):** two competing RuleChange quorums silently auto-resolve
   order-dependently — an I5 violation the fold's contradiction predicates don't cover. The fix (a
   competing-RuleChange predicate) is design-gated; options in backlog §2a. **Highest-value follow-up.**
2. **The X3 instrument is confirmed under-counting (EXP-3):** a substrate-only sweep shows 61 survivors
   that are cross-package-covered (demonstrated). The threshold-counting core is clean. The remaining
   work is the *automated* cross-package harness, not more manual triage.
3. **Fan-out scales on the live set for per-node cost (EXP-1)** but the connect-time resync is
   super-linear on the hub — concrete evidence for why RBSR (diff-only) matters; full-settle past N≈8
   is the open steady-state-anti-entropy item.
4. **EXP-5's real blocker is I9 (identity/key-recovery):** the threshold-revoke-over-wire half can't be
   built without the revocation-authority decision. Recommended option A (quorum-of-Ed25519 reusing the
   governance k-of-n) is the one path *not* blocked on I9 — a candidate for the next run if the human
   accepts that framing.

## State for the next session

- `EXPERIMENT-BACKLOG.md` and `MASTER-INDEX.md` updated to reflect the above (M1 fan-out done; automerge
  0.7 done; X3 substrate sweep done + harness open; fold byte-head done; two design-gated fold findings;
  EXP-5 design gate).
- Branch `claude/experiments-run-01` pushed, **ready for review — no PR opened** (per the brief).
- Reproduce recipes are in each experiment's report (`croft-chat/FANOUT-M1.md`,
  `automerge-partial-reconstruction/REPORT.md`, `local_storage_projection/X3-CROSS-PACKAGE-SWEEP.md`,
  `croft-chat/croft-chat/tests/competing_quorums.rs`).
