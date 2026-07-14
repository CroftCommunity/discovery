# Spec-divergence register

Where a test or experiment goes green because of a **stand-in** — a prototype, a scaffold, a
weakened assertion, or an environment adaptation — rather than spec-conformant behavior, it must be
**visible and separable** from a genuine proof. Otherwise the corpus quietly overstates itself: a
reader sees "X2 PASS" and assumes the spec mechanism exists, when in fact a bandwidth-naive
prototype made it pass.

This register is the single place that enumerates those divergences so they can be reasoned about
and **reconciled back to the spec** — refined into the real mechanism, or corrected. It is seeded
from the re-plant / iroh testbed line of work (2026-07). It is **not** a claim to have audited the
entire imported corpus; new divergences are added as they are introduced, via the convention below.

## Convention (keep the register honest)

Every divergence carries a greppable tag at its site:

```
SPEC-DELTA[<id> | <kind>]: <what it stands in for> — <spec requirement> — Register: alpha/SPEC-DIVERGENCE-REGISTER.md
```

- `grep -rn "SPEC-DELTA" alpha/` lists every tagged site. Every tag should have a row here, and
  every active row should have a tag (or a file:line pointer if the site is in a crate we don't
  edit — e.g. the mutation-vetted substrate).
- **Kinds:**
  - `prototype-mitigation` — product code changed to make a test pass, but it is *not* the spec
    mechanism. **The category that most needs reconciling.**
  - `test-hermeticization` — a test was moved off the hard path (network/relay) so it runs in a
    restricted env; it now proves *less* than its name suggests.
  - `test-scaffold` — test-only machinery (hand-built inputs, fakes) that bypasses a real code
    path. Often acceptable, but flags an API/prod gap.
  - `weakened-assertion` — the test asserts a strictly weaker property than the spec, because the
    spec behavior isn't built yet.
  - `proxy-measurement` — a metric is measured via a stand-in quantity; direction holds, magnitude
    may not.
  - `declared-stand-in` — a substitute that the plan explicitly sequences (not hidden); listed for
    completeness.

## Active divergences (introduced in this line of work)

| ID | Kind | Site | What makes it pass / what it stands in for | Spec requirement | Path back to spec | Status |
|---|---|---|---|---|---|---|
| **hermetic-gossip** | test-hermeticization | `croft-chat/croft-chat/tests/iroh_convergence.rs`; `.../src/iroh_bus.rs` unit test | The two convergence tests were moved from `presets::N0` to `RelayChoice::LocalDirect` so they run without Internet. They now exercise **loopback gossip only**. | The real deployment reaches peers via the **n0 relay + holepunch** (`relay_mode = "n0"`); cross-host convergence must hold over that path. | The relay/holepunch path is **X1** — it genuinely needs the boxes (unreproducible where Internet UDP is blocked). Until X1 runs, these tests do not cover it. | **Bounded — intentional.** Green ≠ relay path proven. |
| **competing-quorum-autoresolve** | weakened-assertion | `croft-chat/croft-chat/tests/competing_quorums.rs` (`two_competing_rulechange_quorums`) | RUN-01 EXP-4 refutation: two **concurrent conflicting RuleChange quorums** on the same rule (each a valid k-of-n) currently **auto-resolve order-dependently** — order1 → `add_member_threshold=9`, order2 → `5`, both `fork="clean"`, no hard-stop. The test asserts this current (weaker-than-spec) behavior as a refutation pin. | §7.6/§7.6.1: an irreducible concurrent governance conflict with no causal order to decide it MUST hard-stop for the humans, not be silently auto-resolved (I5). **The spec was decided by RUN-02 (F8, §7.3.2 / §7.6.1): two quorum-met RuleChanges to one rule are a §7.6-class genuine contradiction, hard-stopped, never content-address-tiebroken.** The fold escalates membership (mutual-expulsion) and role (thrash) races but has **no competing-RuleChange predicate** yet. | **Implementation gap, not a design decision** (RUN-02 F8 decided it; EXP-4 is the experiment F8 said would earn the fold's evidence tag). Build the predicate — options are now *implementation* choices (backlog §2a): same-rule-different-value (narrowest, mirrors mutual-expulsion), or the broader same-subject form. When built, the refutation pin flips RED and asserts the hard-stop. | **ACTIVE — confirmed impl gap vs. decided spec.** Green ≠ competing quorums resolved; it pins the gap EXP-4 confirmed against F8. |
| **fanout-single-run** | proxy-measurement | `croft-chat/FANOUT-M1.md`; `croft-chat/scripts/fanout-measure.sh`; `croft-chat/fanout-data/` | The A4/M1 fan-out curve (EXP-1) is from 1–2 runs per N (no averaging), latency sampled at ±250 ms tick resolution, over a **star-bootstrap** loopback topology. `live_sent = 2N+1` (linear per-node) and head-convergence are topology-robust; the super-linear creator `resync_sent` magnitude is topology-sensitive. | §11.11 #1 wants the fan-out re-key cost on representative hardware across the live set. | Re-run with replicates + a mesh bootstrap + finer timing for tight magnitudes; the *shape* (linear per-node, O(N²) aggregate, super-linear connect-time resync) is what is claimed, and it held across both runs. | **Bounded — direction/shape hold, magnitude indicative.** |
## Reconciled (the spec mechanism now exists — tag retired)

| ID | Was | Reconciled to | Evidence |
|---|---|---|---|
| **x2-backfill** | prototype-mitigation: a per-tick nonce **re-flood** of the whole log defeated gossip dedup so late joiners caught up — green on a stand-in, not the design. | The spec mechanism: **sync-on-connect**. `iroh_bus.rs` now broadcasts each distinct frame **once** (`TAG_LIVE`) in steady state and, on `Event::NeighborUp`, re-broadcasts the retained log **once** as `TAG_RESYNC` (fresh ids). Cost is O(log) per join, not per tick — no re-flood. | X2 all-green across 2 runs on the resync mechanism (`A head == B head`; `84b4b1b0…`, `352b7feb…`); full iroh-it suite + clippy green. Code tag removed; ledger Phase 7 updated. |
| **rulechange-quorum** | weakened-assertion: RuleChange quorum was **not enforced** (Owner-role proxy); the substrate test verified only threshold *storage*, not enforcement. | RuleChange now has a **content-hash approval subject** (`rule_change_approval_subject`) so Step 5.6 enforces it via the same distinct-personae-by-lineage path as membership. | RED→GREEN proven (disabling the arm → 2 cases fail). Substrate `test_i6` strengthened to assert rejection; new `rulechange_threshold_enforced.rs` (4 cases). **Manual mutation gate:** `act_subject→None` killed by the reject cases; `rule_change_approval_subject→const` killed by the mismatched-approval case. Both `blake3` uses reverted clean. See note below on the formal sweep. Spec landing: §7.2 R7 (RUN-02). |
| **automerge-0.6.1** | proxy-measurement: the 4 partial-reconstruction invariants were proven on **automerge 0.6.1**, not the 0.7 ship target (a Rust-1.75 MSRV wall blocked 0.7 in the original session). | The **0.7 ship target itself**: `automerge = "0.7"` → 0.7.4 builds and runs on Rust 1.94.1; the same four scenarios pass with the two documented API deltas applied. | RUN-01 EXP-2 (branch `claude/experiments-run-01`). All four PASS on 0.7.4 (`automerge-partial-reconstruction/run_output.txt`, `REPORT.md` top section). Only change-hash *values* differ (`cea08274…`→`e8524485…`), a serialization artifact; the behavioral invariants are identical. Moved here from "Already-declared caveats". |
| **handcrafted-assertions** | test-scaffold framed as an **API gap**: tests hand-built RuleChange / Approval / cross-device facts because `Session` could not emit them. | `Session` now emits them: `propose_rule_change` + `approve_rule_change` (over new substrate `rule_change`/`approve` commands); cross-device chains are just multiple `Session`s replicating. | `rulechange_quorum_via_api.rs` drives a full RuleChange **quorum end-to-end through the real `Session` API** across two replicating sessions (propose → approve → reference → apply → converge). See the residual-scaffolding note below. |
| **iroh-lww-language** | doc-language divergence: the iroh (Alt.Drive) corpus described **last-writer-wins by `(modified_at, node_id)` timestamp** as a DECIDED conflict-resolution design, contradicting two standing invariants (timestamps never order; concurrent contradiction hard-stops rather than auto-resolves). Not a test stand-in — a forbidden model presented as precedent to anyone grepping the corpus. | **Superseded in place:** DESIGN §7 retitled to a superseded block (body replaced, original recoverable from git history) and one-line superseded pointers added at every other mention, each pointing at §7 rather than repeating the reasoning (conventions A.11 DR-1). | E6 (human-adjudication language pass); grounds in Part 2 §7.3.1 (causal-and-cryptographic order excludes the clock) and §7.6 (reconcile hard-stop), conventions A.11. Spike 1's flat-LWW-too-weak finding (TEST-LOG B2) is the empirical corroboration. |

> **Mutation-gate note (rulechange-quorum).** The touched substrate functions were mutation-tested
> *manually and targeted* (each mutant above is killed by a named test). **RUN-01 EXP-3 (2026-07-14)**
> then ran the formal scoped `cargo-mutants` sweep across `fold_auth` + `governance` + `fold_derived`
> (120 mutants → 54 caught, 61 missed, 5 unviable): **threshold-counting has 0 survivors** in-substrate
> (`governance.rs` 13/13), and the approval-subject survivor `rule_change_approval_subject→const` was
> **hand-killed against the cross-package test** `approval_for_a_different_change_does_not_count` —
> confirming the manual gate above. The 61 survivors are all authorization-*decision* mutants whose
> pinning tests live cross-package; the **automated** cross-package harness (so they resolve
> mechanically) remains **X3**. See `local_storage_projection/X3-CROSS-PACKAGE-SWEEP.md`.

> **Residual-scaffolding note (handcrafted-assertions).** The `tests/common` scaffolding is **not**
> removed and its remaining use is **legitimate, not a divergence**: (a) the Battery-5 refutation
> tests deliberately build *adversarial* delivery — a dropped / duplicated / reordered frame, a fact
> whose antecedent is withheld — which a well-behaved emit API must **refuse** to produce, so it can
> only be hand-built; (b) the fold-level threshold tests (`rulechange_threshold_enforced` etc.) ingest
> directly for focus, not from necessity. What is closed is the **capability gap**: well-formed
> governance facts are now producible through `Session`. Not yet emittable via `Session` (a
> lesser, separate gap): `MembershipRemove`, `RoleGrant`/`RoleRevoke` (surface has `remove_member`;
> the role acts have fold logic but no surface/Session command) — add if/when a client needs them.

> M2 is **not** closed by x2-backfill — the *mechanism* is now spec-shaped, but M2's **sizing** study
> (return-backfill vs dormancy cost at 1/7/30/90-day gaps; push-resync vs pull-on-connect) remains.
> Steady-state anti-entropy (recovering a *live* frame dropped to an existing neighbor, with no new
> join to trigger a resync) is also still future work — the resync covers connect-time catch-up.

## Already-declared caveats (honest in their own docs — listed so nothing hides)

These are not hidden mitigations; each is stated where it lives. Collected here for one-look
reasoning.

| ID | Kind | Where | The caveat |
|---|---|---|---|
| sharddir-standin | declared-stand-in | `croft-chat/croft-chat/src/shared_dir.rs` | `SharedDirBus` is the plan's explicit local stand-in for the iroh transport (the "prove convergence locally first, then swap the adapter" decision). By design, not hidden. |
| e12.4-byteproxy | proxy-measurement | `mls-replant` README / ledger (E12.4) | Drift reset is a **byte-size proxy**; direction corroborates, magnitude understates (openmls serializes blanks compactly). |
| m2-modeled | proxy-measurement | `EXPERIMENT-BACKLOG.md` (M2) | M2's return-backfill cost is a **modeled lower bound** against redb history until the history-convergence node exists. |
| lamport-depth-proxy | proxy-measurement | `local_storage_projection/src/governance.rs` (~L412) | Compaction uses the envelope **lamport as a proxy for log position** in the depth gate. |

## How to use this register

1. **Before trusting a green result**, grep the touched area for `SPEC-DELTA` and check here. A
   `prototype-mitigation` or `weakened-assertion` means the spec mechanism is *not* what passed.
2. **When reconciling**, close the highest-risk kinds first (`prototype-mitigation`,
   `weakened-assertion`), then remove the tag and move its row to a "Reconciled" list with the
   commit that did it.
3. **When adding a new stand-in**, add the tag *and* a row here in the same change — that is the
   contract that keeps "passes" honest.
