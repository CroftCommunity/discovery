# Experiments ↔ Drystone spec — alignment and action plan (for discussion)

`Status: working synthesis, 2026-07-13. Produced when the standalone` experiments `tree was folded
into` discovery/alpha/experiments/ `so discovery and experimentation stay tight. Not authoritative:
the authoritative specs are` beta/drystone-spec/part-1-reasoning-underpinnings.md `(principles),`
part-2-certifiable-design.md `(mechanics), and` conventions-and-decisions.md `(vocabulary). This
document reconciles what the code spikes actually proved against what Part 2 currently claims, and
proposes the specific fold-backs and the remaining test surface.`

---

## 0. What just happened, and where things now live

The code-forward experiments (12 Rust spikes + three navigation registers) that previously lived in
the sibling `experiments/` repo are now under **`discovery/alpha/experiments/`**, mapped
stage-for-stage (`experiments/alpha/*` → `discovery/alpha/experiments/*`). The experiments' own
repo-root README is preserved as `REPO-README.md` for provenance (source SHA `c17b8c8`, identity
`chasemp`). Nothing was dropped: 456 files, 11 MB, no credentials, no compiled binaries; lockfiles
kept per convention.

This co-locates the **two corpora that were drifting**:

- **The spec corpus** — `beta/drystone-spec/` (Part 1, Part 2, conventions) plus the design folds in
  `beta/impl/drystone-design/` and the spec-side experiment tracking in `beta/impl/experiments/`.
- **The experiment corpus** — `alpha/experiments/`, the running code that answers "is this actually
  true?" against real substrates (real openmls, real iroh, real Automerge, live Jetstream).

They are bridged by three registers that shipped inside the experiments tree and are the backbone of
this alignment:

| Register | Question it answers | File |
|---|---|---|
| **MASTER-INDEX** | *Ordering* — what depends on what, the critical path | `alpha/experiments/MASTER-INDEX.md` |
| **EXPERIMENT-BACKLOG** | *What is unrun*, its maturity, its blocker | `alpha/experiments/EXPERIMENT-BACKLOG.md` |
| **SPEC-DIVERGENCE-REGISTER** | *Where a green rests on a stand-in* rather than the spec mechanism (grep `SPEC-DELTA`) | `alpha/experiments/SPEC-DIVERGENCE-REGISTER.md` |

The divergence register is the single most important artifact for keeping the corpus honest: it is
the list of places where "PASS" does **not** yet mean "the spec mechanism works."

---

## 1. The alignment matrix — spec claim ↔ experiment ↔ verdict

Reading of Part 2's section map against the spikes. **Verdict** is one of: `ALIGNED` (experiment
evidence matches the spec's current status tag), `FOLD` (experiment proved something the spec text
does not yet carry — an incorporation target), `PARTIAL` (proven under a stand-in / narrower than the
spec claim — evidence exists but a gap remains), `OPEN` (spec claims it, no experiment yet).

| Spec § (Part 2) | Claim under test | Experiment(s) | Spec status *now* | Verdict |
|---|---|---|---|---|
| §7.3.1–7.3.2 timestamp-free fold order, referenced-gap detection | Permutation-invariant fold; a held-fact-with-absent-predecessor returns a gap, not a false-complete | `local_storage_projection` (fold lib 97/0); prior convergence-exp v2 (27 tests) | `Modeled` (upgraded from `Design` by v2) | **ALIGNED** — completeness-*ahead* (unreferenced tail) is still the open beam |
| §7.2 R3 (revocation folds deterministically); §7.3 threshold enforcement; **§8.2(e)** "membership-op freshness threshold and admin-floor rule decided but **not yet test-run**" | k-of-n RuleChange quorum is **enforced**, not merely stored | `local_storage_projection` `test_i6` (strengthened) + `rulechange_threshold_enforced.rs` (4 cases, RED→GREEN) + `rulechange_quorum_via_api.rs` (end-to-end through `Session`) | Mechanism (**content-hash approval subject**) **absent** from Part 2; §8.2(e) still says "not test-run" | **FOLD** — see §3 (delta `rulechange-quorum`) |
| §7.6 / §7.6.1 reconcile hard-stop, concurrent membership contradiction | A genuine contradiction hard-stops and escalates; reformed genesis is identical across peers | `croft-chat` (contradiction hard-stop banner over real gossip); `local_storage_projection` (every §7.6.1 residue shape hard-stopped, no false trips) | §7.6 already `Verified` (contradiction hard-stop; identical reformed genesis) | **ALIGNED** — already folded; experiments corroborate |
| §7.6.2 / §7.6.11 re-plant: read membership from chain → fresh MLS group → atomic repoint (three arities) | Dedup-not-fork keystone, drift reset, leaf rotation, last-resort availability; membership set = fold-derived set | `mls-replant` (E12.1, E12.3–E12.6; suite 7/0, real openmls 0.8.1) + `replant-continuity` (E12.7 membership bridge, 3/0) | Mix of `Verified-RFC` (arity↔ReInit/Branch mapping) + `Design` | **PARTIAL→FOLD** — *membership* continuity is now proven end-to-end (upgrade candidate); *message* continuity (E12.2/E12.7 message facet) is unbuilt |
| §7.6.3 ReInit-not-atomic stranding window; "intent-recorded-before-freeze" resolution | Can a stranded re-plant be completed from the chain by any member? | `mls-replant` last-resort availability (E12.6) touches availability, **not** the ordering guarantee | `[confirm]`, carried to Appendix B | **OPEN** — E12.6 bounds availability; the *intent-before-freeze ordering* remains `[confirm]` |
| §6.8 / §6.8.1 gap-aware history convergence; the adaptive selector; NeighborUp semantics | A late joiner catches up without a per-tick re-flood | `croft-chat` `iroh_bus.rs` sync-on-connect (X2 all-green on loopback: crash-consistency + no-reversion + catch-up) | §6.8.1 concept present; NeighborUp event set `Verified`/`[confirm]` | **ALIGNED** (delta `x2-backfill` reconciled to spec mechanism); *steady-state* anti-entropy still open |
| §8.2(a) freshness "proven in the model, **not yet over live transport**" | Convergence holds over real transport | `croft-chat` + `iroh`: 2- and 4-node `serve` over **real iroh-gossip**, identical fingerprints (loopback, no relay) | §8.2(a) honesty boundary | **PARTIAL** — live transport proven on **loopback**; the **relay + holepunch** path (X1) is unreproducible here (delta `hermetic-gossip`, still active) |
| §6.x large-media / durability plane | encrypt → content-address → store on real iroh-blobs → reference → fetch → decrypt, with MLS epoch rotation | `encrypted-blob-share` (PoC `Verified`) | §6 design/verified | **ALIGNED** — validates the large-binary path; the encrypt-then-address dedup-loss tradeoff is a noted design fact |
| §11.4 / §11.5 cost scales on live-N; **§11.11 measurement #1** (per-commit + fan-out, `Load-bearing, unearned`) | Per-*commit* re-key cost band on real openmls | `mls-replant` M1 (O(N) floor ↔ O(log N) ceiling measured) | `Load-bearing, unearned` | **PARTIAL** — the per-commit half is measured; the **fan-out** half (A4, N local `serve`) is now runnable with no new infra, not yet run |
| §11.6 / §11.7 dormancy + return backfill; **§11.11 measurement #2** | Return-backfill cost vs dormancy at 1/7/30/90-day gaps | Backlog M2 (modeled lower-bound only) | `Load-bearing, unearned` | **OPEN** — mechanism (sync-on-connect) now exists; the *sizing* study remains |
| §7.7 dataplane history modes; §7.9 scaling (late-joiner) | A node given only later-epoch changes with deps withheld holds them **inert** (no partial doc) | `automerge-partial-reconstruction` (4 scenarios PASS) | design | **PARTIAL** — proven on Automerge **0.6.1**; the **0.7** ship target is a `proxy-measurement` gap, now runnable (Rust 1.94 present) |
| §10.2 messaging backplane (MLS realization); §10.3 transport/overlay | MLS-Welcome-over-iroh; faithful-sync; meer tiers; conformance vectors | `iroh` (F4: E1/E3/E5/E6/E7/E10, meer P0/P1, conformance-core) | §10 realization ledger | **PARTIAL** — Tier-0 meer `Verified` (zero payload keys); conformance cats **7/8/9** (AR/visibility/freshness) recorded `not_yet_emitted`; MLS key-distribution-over-wire still modeled |
| §5.2 / §5.10 identity, group-as-principal; app/client layer | Functional-core/imperative-shell app over real Bluesky fixtures; shared-core group happy-path | `croft-app-phase0` (M1–M6, F5); `croft-group` (happy-path, F6) | design | **ADJACENT** — app-layer, looser spec-mechanism coupling; L2–L5 re-derive F1/F3 mechanics (decision in §4) |
| §4.x / §9 conformance (atproto substrate reality) | Custom NSIDs propagate with no pre-registration; cryptographic trust is free, semantic trust is not | `appview-validation` (7 binaries), `public-roundtrip` (CHAIN-OF-CUSTODY V1–V6 + moderation) | Substrate reference realizations (§10) | **ALIGNED** — grounds the "own your schema/threading/policy" posture the public-regime bridge (§11.9.3) rests on |

---

## 2. What was *previously* tested (so we don't re-litigate it)

The spec-side already carries a prior layer of validation, logged in
`beta/impl/experiments/drystone-reviews-and-experiments-log.md`. Reading it forward:

- **Two feasibility reviews (2026-07-04)** verified the external primaries (RFC 9420/9750, CALM,
  CRDT SEC, the Matrix CVEs, BLAKE3, iroh 1.0) and produced exactly one spec addition (the §4
  length-extension check + wire-freeze caution in Appendix B). Most internal-mechanics findings were
  already handled in the text.
- **Convergence experiment v2 (2026-07-04)** — a *reference-model* fold (R1–R4 + A12), 27 + 16 tests,
  produced **two Part 2 status upgrades `Design → Modeled`** (the §7.3.2 tier-boundary projection
  consistency and R3 no-fold-time-rejection with referenced-gap detection). System-level
  order-independence stayed `Load-bearing, unearned` pending the completeness-ahead beam.
- **Convergence experiment v3** extended the above to the finality mechanics (A-series, Stages 4–6).

**The key continuity:** those were *reference-model* experiments (a faithful fold, not production
code). The spikes now imported are the **next layer down** — the same mechanisms exercised against
*real substrates*. Where v2 moved a claim to `Modeled`, `local_storage_projection` +
`mls-replant` + `croft-chat` are what can move the load-bearing ones further. The
fold-coverage audit (`beta/impl/experiments/drystone-fold-coverage-audit.md`) already confirms the
companion design docs are folded into Part 2 §§4.6, 5.11, 6.8.5, 7.3, 7.6 — so the *design* is
in-spec; what the code changes below touch is **status and one genuinely-missing mechanism**.

---

## 3. What changed *during experimentation to pass criteria* — and what each implies for the spec

This is the heart of the request. Four times, a green result initially rested on a stand-in rather
than the spec mechanism. Each is tagged `SPEC-DELTA` in the code and rowed in the divergence
register. Three were **reconciled** (the real mechanism was then built); one stays **active**.

### 3.1 Reconciled — the spec mechanism now exists

1. **`x2-backfill` → sync-on-connect.** *Symptom:* late joiners only caught up because a per-tick
   nonce **re-flood** of the whole log defeated gossip dedup — green on a bandwidth-naive stand-in,
   O(N) per tick. *Fix:* `iroh_bus.rs` now broadcasts each frame **once** (`TAG_LIVE`) and, on
   `Event::NeighborUp`, re-broadcasts the retained log **once** as `TAG_RESYNC` — O(log) per join.
   *Spec implication:* this **is** §6.8.1 gap-aware convergence; the spike confirms the spec was
   right. **Fold:** none required for the mechanism; consider a `Verified` corroboration note on
   §6.8.1 scoped to *connect-time* catch-up, and keep **steady-state anti-entropy** (recovering a
   live frame dropped to an *existing* neighbor) as explicitly open.

2. **`rulechange-quorum` → content-hash approval subject.** *Symptom:* RuleChange quorum was **not
   enforced** — an Owner-role proxy stood in, and the substrate test checked only threshold
   *storage*. *Fix:* RuleChange now carries a **content-hash approval subject**
   (`rule_change_approval_subject`), so Step 5.6 enforces it via the same distinct-personae-by-lineage
   path as membership; RED→GREEN proven (disabling the arm fails 2 cases), manual mutation gate
   passed. *Spec implication:* **this mechanism is not in Part 2** (grep confirms: no
   `rule_change_approval`/`rulechange` in the spec text). It is the concrete realization of §7.2 R3
   (revocation-as-deterministic-fold) generalized to *rule changes*, and it discharges part of
   **§8.2(e)** ("admin-floor rule … decided but not yet test-run"). **Fold: yes — the highest-value
   incorporation on this list** (see §5, item F1).

3. **`handcrafted-assertions` → `Session` emit API.** *Symptom:* tests hand-built RuleChange /
   Approval / cross-device facts because `Session` couldn't emit them — framed honestly as an API
   gap. *Fix:* `Session` now emits them (`propose_rule_change` + `approve_rule_change`); a full quorum
   runs end-to-end through the real API across two replicating sessions. *Spec implication:* this is
   an **implementation/API** closure, not a spec-text change — the spec is mechanism-neutral (§7.2).
   Worth noting the *residual*: `MembershipRemove`, `RoleGrant`/`RoleRevoke` are not yet emittable via
   `Session` (fold logic exists; no surface command) — a client-driven, not spec-driven, gap.

### 3.2 Active — green ≠ spec path proven

4. **`hermetic-gossip` (test-hermeticization, still open).** The two convergence tests were moved
   from `presets::N0` to `LocalDirect` so they run without Internet — they now exercise **loopback
   gossip only**. The real deployment reaches peers via the **n0 relay + holepunch**
   (`relay_mode = "n0"`); that path is **X1**, and it genuinely needs real NAT + public UDP ingress
   ("the secroute boxes"), unreproducible where Internet UDP is blocked. *Spec implication:* **do not
   let §8.2(a) be read as closed.** Loopback live-transport convergence is real and worth recording,
   but the honesty boundary (freshness/convergence over the *relay* path) stays until X1 runs.

**Also-declared caveats** (honest in their own docs, not hidden mitigations, but they bound spec
claims): `e12.4-byteproxy` (drift reset is a byte-size proxy), `m2-modeled` (return-backfill is a
modeled lower bound), `automerge-0.6.1` (invariants on 0.6.1 not the 0.7 ship target),
`lamport-depth-proxy` (compaction uses lamport as a proxy for log position). Each should be echoed as
a one-line caveat wherever the corresponding spec claim hardens.

---

## 4. Incorporate-back worklist — concrete spec edits

Ordered by value. Each names the target section and the exact change. These are **proposals for
discussion**, not applied edits — the spec is reviewed material.

| # | Spec target | Change | Source |
|---|---|---|---|
| **F1** | **§7.2 (R3) / §7.3 governance facts** + **§8.2(e)** | Add the **content-hash approval subject** as the realization that makes a RuleChange quorum *enforced* (not merely stored), counted by distinct personae per lineage. Move §8.2(e)'s "admin-floor rule … decided but not yet test-run" toward **tested** (cite `rulechange_threshold_enforced.rs`, RED→GREEN). | delta `rulechange-quorum` |
| **F2** | **§7.6.2 / §7.6.11 re-plant** | Upgrade the **membership-continuity** half of re-plant from `Design` toward `Verified`: the MLS-stamped member set is provably exactly the fold-derived set across genesis / authorized adds / real removals / rejected unauthorized changes. | `mls-replant` + `replant-continuity` (E12.7 membership) |
| **F3** | **§6.8.1** | Add a scoped `Verified` corroboration for **connect-time** gap-aware catch-up (sync-on-connect, O(log)/join), and state **steady-state anti-entropy** as the remaining open sub-item so the claim isn't over-read. | delta `x2-backfill` |
| **F4** | **§11.11 measurement #1** | Record the **per-commit** re-key cost band as *partially earned* (O(N) floor ↔ O(log N) ceiling on real openmls); keep the **fan-out** half unearned pending A4. | `mls-replant` M1 |
| **F5** | **§8.2(a)** | Note loopback live-transport convergence as evidence (2-/4-node identical fingerprints) while keeping the boundary open for the **relay/holepunch** path (X1). Do **not** mark §8.2(a) closed. | delta `hermetic-gossip` |
| **F6** | **§7.6.3** | Leave the ReInit stranding-window `[confirm]` **as-is**; note that E12.6 addressed *availability* but not the *intent-before-freeze ordering* the discharge needs. | `mls-replant` E12.6 |
| **F7** | **§10.2 / §10.3, §9 conformance** | Note conformance cats **7/8/9** (AR/visibility/freshness) as `not_yet_emitted` and the revoke-authority vector as a `PLACEHOLDER`, gated on MLS-key-distribution-over-wire + threshold-revoke-over-wire. | `iroh` conformance-core |

**Suggested landing spot for the spec-side record:** extend
`beta/impl/experiments/drystone-reviews-and-experiments-log.md` with a "2026-07 real-substrate spikes"
entry capturing F1–F7 as the spec effect, mirroring how v2's entry recorded its two `Design→Modeled`
upgrades. That keeps the reconciliation auditable without editing Part 2 until reviewed.

---

## 5. What remains to be tested (the frontier)

Straight from `EXPERIMENT-BACKLOG.md` / `MASTER-INDEX.md`, framed by leverage. Full detail and
blockers live in those registers; this is the spec-facing summary.

**Runnable now, no new infrastructure (highest leverage):**
- **A4 / M1 fan-out** — N local `serve` processes on the loopback testbed → earns the second half of
  §11.11 measurement #1.
- **Automerge 0.7 confirmation** — re-run the 4 partial-reconstruction invariants on the ship target
  (retires the `automerge-0.6.1` proxy; Rust 1.94 is present).
- **MLS key-distribution-over-wire + threshold-revoke-over-wire** → unblocks conformance cats 7/8/9
  (F7).
- **Remaining fold open items** — per-act approver-role granularity; two-competing-quorums →
  §7.6.1 contradiction; contradicted-group byte-head naming; the live "catching up…" TUI indicator.
- **iroh Spike 1** (10K-entry manifest sync) and **Spike 4** (ticket pairing + BIP39 confirm + MitM
  negative test).

**Needs a build first:**
- **B1 — Drystone dataplane hash structures (§7.6.2 message-plane substrate)** → unblocks **A5 /
  E12.2 + E12.7 message-continuity** (an in-flight conversation surviving the re-plant repoint). This
  is the *message* half of re-plant that F2 leaves open.
- **X3 automated cross-package mutation sweep** — `cargo-mutants` is installed; the harness must span
  packages (V5′ positive coverage lives in `croft-chat`, not the substrate suite). Touches the §7.3 /
  §8.2(g) capped-root soundness trust claim.
- **meer P2→P6** — each phase turns one iroh lab experiment (E8/E9/E11/E12) into its running form
  (§10.3 transport realizations).
- **M2 sizing study** — return-backfill vs dormancy at 1/7/30/90-day gaps (§11.11 measurement #2);
  the mechanism exists, the numbers don't.

**Gated on hardware / a decision (do not start blind):**
- **X1 real-NAT convergence** (the only way to close delta `hermetic-gossip` / §8.2(a) relay path) —
  needs the secroute boxes + public UDP ingress.
- **iroh Spike 3** (macFUSE) and **Spike 7** (iOS-iroh-blob feasibility — the iroh→Veilid decision
  point) — need macOS / iOS hardware.
- **Identity & key-recovery model** (quorum social recovery vs minimal-central-authority VC issuer) —
  the largest open **design** problem; start with the BIP39 paper-recovery round-trip spike.

---

## 6. Divergence hygiene, going forward

Now that experiments live in this repo, the honesty contract travels with them:

- **The `SPEC-DELTA` convention is the seam.** Every stand-in carries a greppable tag pointing at
  `alpha/experiments/SPEC-DIVERGENCE-REGISTER.md`; every active row has a tag; adding a stand-in and
  its row is one change. `grep -rn "SPEC-DELTA" alpha/experiments/` is the audit.
- **Before trusting any green, grep the touched area for `SPEC-DELTA`** and check the register. A
  `prototype-mitigation` or `weakened-assertion` means the spec mechanism is *not* what passed.
- **Reconcile highest-risk kinds first** (`prototype-mitigation`, `weakened-assertion`), then move the
  row to "Reconciled" with the commit that did it — exactly as the three deltas in §3.1 were.
- **This bridges to the beta-side seam-tracker.** `COHESION.md` (loose-end ↔ the proof/spike that
  closes it) and the `crystallized/proof-ledger.md` should reference the divergence register so the
  two honesty systems point at each other rather than drifting.

---

## 7. Open decisions for you (surfaced, not resolved)

1. **Placement + retirement.** Confirm `alpha/experiments/` is the right home. If yes, the standalone
   `experiments/` repo should be **frozen/retired** so there is one source of truth — otherwise the
   divergence this document exists to fix simply reopens between two repos. (The imported tree carries
   its own `alpha/beta/rc/publish` staging language in `REPO-README.md`; decide whether that collapses
   into discovery's staging or is dropped.)

2. **Fold now vs later.** Of the F1–F7 spec edits in §4, which land in Part 2 now vs stage in the
   reviews-and-experiments log pending a review pass? F1 (RuleChange enforcement mechanism) is the one
   genuinely-missing mechanism; the rest are status moves and caveats.

3. **croft-group L2–L5.** Build on the proven Drystone crates (F1 `local_storage_projection` / F3
   `mls-replant`) or re-implement in the shared-shell architecture? The backlog flags the overlap;
   deciding avoids proving the same mechanics twice.

4. **Identity / key-recovery model.** The largest open design problem (quorum social recovery vs VC
   issuer). Kept out of autonomous work by design — it needs your call before the BIP39 spike is more
   than a first step.

5. **X1 boxes.** Real-NAT convergence is the only path to closing the last active spec-delta
   (`hermetic-gossip`) and the §8.2(a) relay honesty boundary. Worth deciding whether to provision the
   secroute boxes now or hold the boundary open.

---

`Cross-refs: alpha/experiments/{MASTER-INDEX,EXPERIMENT-BACKLOG,SPEC-DIVERGENCE-REGISTER}.md ·
beta/drystone-spec/part-2-certifiable-design.md (§6.8, §7.2, §7.3, §7.6, §8.2, §11.11) ·
beta/impl/experiments/drystone-reviews-and-experiments-log.md · beta/impl/drystone-design/.`
