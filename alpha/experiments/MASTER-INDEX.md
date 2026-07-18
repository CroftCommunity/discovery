# Master index — the experiment series

The experiments in `alpha/` are not independent one-offs; they form a **dependency series** across a
few tracks (substrate → integration → re-plant → transport). This index ties the plan/spec docs
together into that series, names each stage's dependencies and blockers, and gives the critical
path. It is the "how the pieces connect" companion to `EXPERIMENT-BACKLOG.md` (the flat catalog of
every unrun item).

- **Backlog** = *what* is unrun, its maturity, its blocker. (`EXPERIMENT-BACKLOG.md`)
- **Index** (this file) = *ordering* — what depends on what, and what to run first.
- **Register** = where a green result rests on a **stand-in** (prototype / scaffold / weakened
  assertion) rather than spec-conformant behavior, so those can be reconciled back to spec.
  (`SPEC-DIVERGENCE-REGISTER.md` — grep the code for `SPEC-DELTA`.)
- **Published spec site** = the Drystone corpus rendered with every cross-reference as a followable
  link, at https://croftcommunity.github.io/discovery/ . Built by `site/build.py` (RUN-10 Part 1); the
  same build runs a **broken-reference gate** on every push and PR (`.github/workflows/pages.yml`), so a
  §-reference in Part 1 or Part 2 that resolves to no heading fails CI — a permanent, free consistency
  guard on the corpus. Contract in `site/README.md`.

---

## 1. Source-doc inventory (the plans)

| Plan / spec doc | Covers | State |
|---|---|---|
| `local_storage_projection/` (MUTATION_TESTING, TRAIT_CONTRACTS, CONVERGENCE_FINDING, FROZEN_FLUID, …) | Governance fold + projection substrate | **Characterized** (lib 97/0) |
| `croft-chat/plans/2026-06-26-1-plan-integrated-drystone-cli.md` | Integrated Drystone CLI, P1–P20 (Milestones A–D) | **Closed** |
| `croft-chat/plans/2026-07-11-1-plan-next-experiments.md` | **Master ledger** — Batteries 5–8 + re-plant | **Active** (Phases 1–6 done; 7–8 remainder open) |
| `croft-chat/plans/2026-07-11-2-findings-plain-english.md` | Plain-English findings digest | Reference |
| `croft-chat/plans/2026-07-12-1-design-concurrent-contradiction.md` | §7.6.1 concurrent-contradiction design | Implemented |
| `croft-chat/plans/2026-07-12-2-design-threshold-enforcement.md` | k-of-n threshold design (Option A) | Implemented |
| `croft-group/plans/2026-06-22-1-plan-croft-chat-cli-group-core.md` | Shared-core happy-path + L1–L6 | **Closed** (L1–L6 sequenced, unbuilt) |
| `iroh/RELAY-PLACEMENT-LAB-SPEC.md` | Relay/placement lab E0–E12 | Spec (partial runs) |
| `iroh/TEST-LOG.md` | Running lab/spike log (✅/~/⏳) | Living log |
| `iroh/docs/phase-0-spikes.md` | De-risking Spikes 1–7 | Spec (only 02-irohblobs memo written) |
| `iroh/docs/roadmap.md`, `DESIGN.md` (§14), `RESUME-NEXT-SESSION.md`, `NEXT-SESSION-2026-06-16.md` | Roadmap, open questions, handoffs | Reference |
| `croft-app-phase0/BUILD-SPEC.md` | App layer M1–M6 | **Closed** (M1–M6 done) |
| `automerge-partial-reconstruction/REPORT.md` | Partial-reconstruction invariant | **Closed** (0.6.1; 0.7 open) |
| `appview-validation/` phases 8–9 (`authserve`, `sealed`) + `helper-seam/` | **Stellin AppView caller-identity** — service-auth JWT verify + viewer-gated serving (EXP-A); §H hybrid sealed offer-gating with a compilation-boundary content-blind store (EXP-B); the content-helper seam over real MLS, forward-blind on revocation (EXP-C) | **RUN-14** (EXP-A/B/C green, red-first; EXP-A live leg P-A1/A2 blocked on creds) |
| `appview-infra/` (kit + GROUPS.md) | **AppView hosting kit** — manifests+generator as source of truth (systemd/Caddy/litestream/rclone), Litestream-to-R2 + Porkbun backup plane, own-data API sidecar (self-scoping/export/timeout/containment), access-gated large-group tier (roster-gated, `GroupStore` fork-agnostic), backup-audit invariant, terraform+bootstrap+deploy, and a local destroy→restore fire drill. Design corpus (`GROUPS.md` — **v2 (RUN-16): the group tier model, one lineage/envelope/delivery-plane/catalogue on two policy axes; RUN-15 Variant A/B write-path fork preserved as Section B**, spec note staged) in discovery; `kit/` extracts to `CroftCommunity/appview-infra` | **RUN-15** (D1–D16 green, red-first, local-rehearsal grade; `terraform validate` BLOCKED; Phase 1.5/Phase 2 staged; 6 declared stand-ins). **RUN-16** — `GROUPS.md` v2 canonical model (docs-and-registers; corpus-tests extended RED→GREEN, 7 v1 + 12 v2; spec note staged `needs-call`, no tag moved; backlog §6d). **RUN-18** — reception-completeness paragraph (A.2, chaining/detection-not-delivery) + `PUBLICATIONS.md` (the publications positioning: degeneration principle, single-agent limit, delta table, subscriber reframe; corpus-tests +15 → 34; addendum staged `needs-call`; backlog §6f) |
| `tier-proof/` (+ `steward-seal/` sub-crate) | **The tier proof** — the RUN-16 group model built and run end to end: one signed envelope (`H(envelope)` identity, §A.5) carrying every record; a fold to catalogue + interval roster; open-tier one-signature self-registration + backfill reconstructability; write-policy axis + validate-before-relay (§A.8); gated two-sided facts, co-sign threshold, causal revocation cut, silence≠verdict, archive rebuild (§A.3); device-key delegation with event-driven revocation; blinded roster + a **real openmls** steward group (sealed reasoning → public verdict); tier transition as re-plant; three co-hosted delivery-role processes + interval backfill (§A.7); measured scale rehearsal; reception completeness executable (RUN-18 B1–B6: chaining at fold+relay, reader-side gap detection/repair, the honest tail claim, chaining×interval composition, the three-way retraction distinction, auditable reach) | **RUN-17** (P1–P9 green, red-first, 52 tests; component/loopback grade; P2/P5 live legs BLOCKED on creds; 7 declared stand-ins; no boundary/Variant decision). **RUN-18** (B1–B6 green, red-first, 69 tests total; component grade; B5 live deletion BLOCKED on creds, `run18-retraction-local` stand-in) |
| `attest-family/` (+ `PRIMITIVES-ATTEST.md`, `FINDINGS.md`, `FINDINGS-ANONYMITY-SETS.md`) | **Attestation family** (own-lane RUN-ATTEST-01) — one family, two axes (persona\|thing × mutual\|unilateral_notice\|unilateral_private): co-signed edges (halves + shared core), scoped vouches, unilateral-notice reviews, issuer predicates (substrate unrepresentable, compile-boundary), viewer-relative corroboration structure (no scalar — tested impossibility), resolvability governed by the named party, covenant as R7 rule on the reused substrate quorum machinery. 35 tests + 6 compile_fail doc-tests, red-first. **Anchor personas** (RUN-ATTEST-02, layered on 01) — no-default invariant (no rank representable, sibling indistinguishability), sibling unlinkability over the full public sweep, single-predicate credentials with vetting antecedents (fee-as-sybil-friction structure), blinded per-epoch commitment lineage (unordered folds), OCSP-shaped status checks, seam-typed issuer state (`SeamBoundary`), anchor-count dial on the reused R7 machinery, anonymity-set measurement (COOP-S 12 vs COOP-L 400 + bundle shrink). **Settlement riders + ATProto matchup** (RUN-ATTEST-03): closed `AntecedentKind` class with R7-governed qualifying register, kind-specific `edge_superseded` marker, withdrawn-is-absent-not-tombstoned, `atproto_map` lossless mapping spike, draft `ing.croft.attest.*` lexicons, `ATTEST-ATPROTO-MATCHUP.md` ten-row abilities brief. **Settlement riders V4–V10** (RUN-ATTEST-04, closing the walk): graded resolvability default (counterparts resolve; strangers get cardinality; OPEN is a policy supersede), V5 issuer transparency rework (signed tree heads over keyed commitments, holder-stapled inclusion proofs, per-epoch superseded sets — status check + receipt pile deleted), V6 era anchoring (governance lineage = era spine; R7-governed head cadence), era-reissue (holder-signed, chains original vetting, structurally free), `sole_anchor(context)` recorded REJECTION, era-graded membership with no standing computation, `treeHead` lexicon draft superseding `commitmentEpoch` | **RUN-ATTEST-01** (all parts green, `Modeled`; findings F-AT-1..5). **RUN-ATTEST-02** (EXP-PA1..6 green red-first, 23 new tests, `Modeled`; measurements in FINDINGS-ANONYMITY-SETS.md; findings F-PA-1..3). **RUN-ATTEST-03** (Parts A/B/C green red-first, 9 new tests + 4 compile_fail doc-tests, `Modeled`; AT-series OC-2/3/4 DECIDED (V1–V3, 2026-07-18); finding F-AT-6). **RUN-ATTEST-04** (Parts A–D green red-first, 16 new tests, `Modeled`; the 2026-07-18 owner-call walk COMPLETE — ten of ten, V1–V10 settled, ZERO pending OWNER-CALL tags in the attest lane; PRIMITIVES home decided (V10: alpha, named graduation trigger)) |
| `../../beta/drystone-spec/EVIDENCE-MAP.md` | Spec ↔ experiment traceability index (one row per Part 2 tag ≥ `Modeled`) | **Living index** (built RUN-08; an index, never a status source) |

> Note: the `meer` superpeer and `geer` gating-peer **design** docs referenced by the iroh handoffs
> live in the sibling `discovery`/`Proofs` repos, not in this workspace. In-repo, meer is realized
> in `iroh/crates/relay-loadtest/src/meer.rs` + the `relay-lab-runs/E9-meer-tier0-*` run.

---

## 2. Foundations (COMPLETE — the base the series stands on)

| ID | Foundation | Where |
|---|---|---|
| **F1** | Governance fold characterized (lossy delivery, convergence, §7.6.1 hard-stops, k-of-n by lineage) | `local_storage_projection` |
| **F2** | Integrated Drystone CLI P1–P20; local + iroh-gossip convergence; contradiction hard-stop | `croft-chat` |
| **F3** | MLS re-plant Rung A (E12.1, E12.3–E12.6) + M1 cost band + E12.7 membership bridge | `mls-replant`, `replant-continuity` |
| **F4** | iroh transport spikes: E1/E3/E5/E6/E7/E10, E11-logic, E12-local, meer P0/P1, conformance-core, B/T-series | `iroh` |
| **F5** | App layer Phase 0 (M1–M6) | `croft-app-phase0` |
| **F6** | Shared-core messaging happy-path | `croft-group` |
| **F7** | Encrypted large-media path over iroh-blobs + MLS rotation | `encrypted-blob-share` |

---

## 3. The series (open stages, with dependencies + blockers)

Notation: **needs** = an in-repo prerequisite stage; **blocked** = an external resource/decision
(see the cross-cutting keys in `EXPERIMENT-BACKLOG.md §1`).

### Track A — Drystone verification frontier
| ID | Stage | Needs | Blocked on |
|---|---|---|---|
| **A1** | Fold open items (~~RuleChange thresholds~~ ✅; ~~contradicted-head byte naming~~ ✅ **RUN-01 EXP-4**; ~~competing-quorum~~ ✅ **design DECIDED (RUN-02 F8: hard-stop) + impl BUILT & test-run (RUN-03 Phase B: `detect_competing_rulechange`; register Reconciled)**; per-act approver granularity **still design-open**; catching-up TUI = UX, skipped) | F1 | approver-role = **design decision** (backlog §2a) |
| **A2** | `cargo-mutants` re-sweep on `fold_auth`/`governance` (Battery 8 X3) | F1 | ✅ **DONE (RUN-07)** — automated cross-package harness (`X3-AUTOMATED-SWEEP.md`): 61 substrate survivors → **7 killed, 54 justified, 0 unjustified**; R7 count-enforcement now cross-package `Verified` (§7.2); surfaced `fold_auth` as an off-consumer-path duplicate, then **retired it** (RUN-07 follow-up: `fold_auth.rs` deleted; register `fold-auth-duplicate` Reconciled) |
| **A3** | Local multi-process convergence + fault injection (X2) | F2 | ✅ **DONE (§5)** — X2 all-green on the loopback testbed: crash-consistency, monotonic no-reversion, **and** catch-up-after-absence — closed by **sync-on-connect resync** (the spec mechanism; the prototype spec-delta was reconciled and retired). **Steady-state anti-entropy** now demonstrated at loopback (RUN-09 Part 4: `anti_entropy` range-summary/diff + `steady_state_anti_entropy.rs`; §6.8.1 → `Modeled`). M2's *sizing* study remains; the range-partitioned production construction + real-transport loss stay open |
| **A4** | ~~M1 fan-out~~ ✅ **DONE RUN-01 EXP-1** (`croft-chat/FANOUT-M1.md`: per-node linear `2N+1`, O(N²) aggregate, heads converge; resync super-linear on hub past N≈8); M2 (return-backfill vs dormancy) still open | F3, A3 | M2 modelable now |
| **A5** | ~~E12.2 + E12.7 message-continuity (atomic repoint of an in-flight conversation)~~ ✅ **DONE (RUN-09 Part 3)** — B1 dataplane hash structure built (`replant-continuity/src/dataplane.rs`); `e12_2_message_continuity.rs` proves pre-repoint exactly-once, in-flight causal-order, cross-order digest equality, dup/drop detection. §7.6.2 message half → `Modeled` (loopback). **Real transport landed RUN-12 Part 2** (`iroh_message_continuity.rs` — the four claims over real iroh-gossip at loopback; stays `Modeled`, the `[gates-release]` B1 encoding + real-NAT = X1 still gate) | F3, **B1** | ✅ B1 built at loopback; real transport RUN-12 |
| **A6** | X1 — live cross-host over **real NAT** | F2 | **the secroute boxes** (needs real NAT + relay holepunch — *not* localhost-satisfiable) |

### Track B — Re-plant dataplane (new build)
| ID | Stage | Needs | Blocked on |
|---|---|---|---|
| **B1** | Drystone dataplane hash structures (the §7.6.2 message-plane substrate) | F1, F3 | — (design → build) |

### Track I — iroh substrate
| ID | Stage | Needs | Blocked on |
|---|---|---|---|
| **I1** | MLS key-distribution over the wire (**✅ green-real, reproduced RUN-08** — `mls-welcome-over-iroh` runs here after the Proofs fold-in; `relay-lab-runs/C-mls-welcome-2026-07-15-run08`; not yet in the conformance *emitter*) + threshold-revoke as real k-of-n (**⛔ DESIGN-GATED, firewall**) | F4 | threshold-revoke = the **revocation-authority decision** (I9); backlog §6d-i |
| **I2** | Conformance vectors cats 7/8/9 (AR / visibility / freshness) — **✅ emitted, 66/0 re-proven RUN-08** (folded conformance-core; cat 7 real Rust, cats 8/9 TS-authoritative, cat-5b revoke-authority *mechanism*). Residual: the over-the-wire *authority distribution* (b) | I1 | the I1 revoke design decision (firewall, I9) |
| **I3** | meer **P2** bridge mode → runs lab **E8** (superpeer crossover) | F4 | — |
| **I4** | meer **P3** Tier-1 + no-mirror curve → runs lab **E9** rest | I3 | — |
| **I5** | meer **P4/P5** (RoQ SFU / MoQ relay) → E12-transport / E11-full form | F4 | — |
| **I6** | meer **P6** Tier-2 (policy-gated) | I4 | — |
| **I7** | Relay lab: **E2** (pkarr/DNS-origin), **E0** reconnect-storm (runnable); **E4** (LVS), **E0-NAT** (blocked) | F4 | `ipvsadm` (E4); NAT + public UDP ingress (E0-NAT) |
| **I8** | Phase-0 spikes: **1** (10K manifest), **4** (ticket pairing) runnable; **5** (ADR, needs 1), **6** (retro); **3** (macFUSE), **7** (iOS) hw-gated | F4 | macOS/iOS hardware (3, 7) |
| **I9** | Identity & key-recovery model (quorum social recovery vs VC issuer) → **BIP39 round-trip spike ✅ landed RUN-08** (`bip39-recovery-roundtrip` — the Tier-1 *lock* mechanism; the trust tier stays the open call) | F4 | **design decision** (largest open problem) — the Tier-2 trust predicate |
| **I10** | HashSeq single-file striping; Automerge-over-iroh interactive-artifact; DESIGN §14 spikes | F4 | — |

### Track C — cross-cutting
| ID | Stage | Needs | Blocked on |
|---|---|---|---|
| **C1** | `croft-group` L1–L6 (identity, MLS/encryption, fork/merge, governance, real-iroh L5, shared-shell L6). **L2a (MLS-sealed happy-path frame) ✅ DONE (RUN-11 Part 2)** — `croft-group/crates/group-seal`, `Verified` at loopback; the L2 mechanism half (R1–R7) reusing `lineage-mls`; R8-tier/R9/R10 firewalled | F6 | L5 needs iroh testbed; **decision:** build L2–L5 on the Drystone crates (F1/F3) rather than re-implement |
| **C2** | ~~automerge **0.7** confirmation~~ ✅ **DONE RUN-01 EXP-2** (0.7.4 on Rust 1.94.1; all 4 invariants hold; `automerge-0.6.1` proxy reconciled) | — | — |
| **C3** | Doc chores: Alt.Drive→Croft.Drive rename (TEST-LOG B5); `[HYPOTHESIS]`→`MEASURED` tag pass | — | — |

### Track G — gated (do not start without the gate)
`geer` (legal review — compellability) · S3/S4 (design gate G5) · T8 (UX decision) · T10/T13
(bsky / iOS host). Mostly point into the sibling `discovery`/`Proofs` repos.

---

## 4. Dependency map + critical path

```
F1 ──┬─> A1 (fold open items; RuleChange thresholds ✅)   F4 ──┬─> I1 ─> I2 (conformance 7/8/9)
     ├─> A2 (mutants ✅ tool; cross-pkg sweep left)             ├─> I3(E8) ─> I4(E9) ─> I6
F2 ──┼─> A3 ✅ (local convergence + X2 fault: done)        ─┐    ├─> I5 (RoQ/MoQ)
     │                                                     │    ├─> I7 (E2 now; E4/E0-NAT blocked)
F3 ──┼─> A5 (msg continuity) <─ B1 ────────────────────────┘    ├─> I8 (spikes 1/4 now; 3/7 hw)
     └─> A4 (M1 fan-out/M2) <─ A3 ✅ [runnable now]              ├─> I9 (recovery: needs decision)
A6 (X1 real-NAT) <─ boxes  [NOT localhost]                      └─> I10 (striping / automerge-iroh)
F6 ─> C1 (L1–L6; L5<─testbed ✅)   C2 (automerge 0.7: runnable now)
```

Reconciled spec-deltas this line of work (see `SPEC-DIVERGENCE-REGISTER.md`): `x2-backfill` (→
sync-on-connect), `rulechange-quorum` (→ enforced), `handcrafted-assertions` (→ `Session` emits
governance facts; the `MembershipRemove`/`RoleGrant`/`RoleRevoke` emit-surface residual **CLOSED
RUN-12 Part 4** — `session_emit_governance_via_api.rs`, propose/approve/enact mirroring rule-change),
— **RUN-01 (2026-07-14)** — `automerge-0.6.1` (→ 0.7 ship target confirmed), and
— **RUN-03 (2026-07-14)** — `competing-quorum-autoresolve` (→ the F8-decided hard-stop is now built
and test-run: the competing-RuleChange contradiction predicate `fold_derived::detect_competing_rulechange`;
register row moved Active → Reconciled).
Active: `hermetic-gossip` (needs X1 / the boxes) — now the **only** Active row. `fanout-single-run`
retired (Reconciled, RUN-09 Part 5: K=5 repeated-run, `live_sent=2N+1` exact, head-convergence every
run, super-linear hub-resync shape reproduced with a tight band; addendum in `FANOUT-M1.md`).

**Recommended critical path (highest leverage first).** The board after RUN-10 is nearly
consolidated. Exactly **one Active divergence** remains — `hermetic-gossip` (X1-gated, needs the
boxes). Exactly **one open design call** — **I9**, the identity/key-recovery trust tier and the
largest open problem (the Tier-1 *lock* landed RUN-08 via `bip39-recovery-roundtrip`; the Tier-2 trust
predicate is the call). Beyond those sit the **parked release pass** (`[gates-release]` + BLAKE3
wire/byte pinning, which the now-deferred emitter integration rides) and the **resolution-ACL
(croft-group L3)** design frontier. **Everything else is `Modeled`-or-better at its stated grade, or
shaped as a RED-able backlog row.** The immediate forward work is this run's build queue — L2a (RUN-11
Part 2), the §2e group-principal seam spike (Part 3), and message continuity over real transport (Part
4, first-to-drop) — then the range-partitioned RBSR construction and croft-group L2b+. The banked
history below records how the board reached this state.

0. ✅ **RUN-01 banked:** A4/M1 fan-out (EXP-1), automerge 0.7 (EXP-2, C2), X3 substrate sweep (EXP-3),
   fold byte-head naming (EXP-4). See `RUN-01-SUMMARY.md`. **RUN-03/04 banked:** the competing-RuleChange
   contradiction predicate (RUN-03 Phase B), plus the continuity-decoupling, reconciliation-horizon, and
   corroboration-dials design passes (RUN-03/04). See `RUN-03-SUMMARY.md`, `RUN-04-SUMMARY.md`.
   **RUN-05/06 banked:** the full consistency pass (RUN-05) and the settlement of its 11 register findings
   (RUN-06 — F4 fan-out grade landed, register/doc-method path repoints, A.11 vocabulary extension). See
   `RUN-05-SUMMARY.md`, `RUN-06-SUMMARY.md`, `CONSISTENCY-FINDINGS-2026-07.md`.
1. ✅ **Closed (RUN-03 Phase B):** the real §7.6.1 gap (EXP-4) — two competing RuleChange quorums
   auto-resolving order-dependently (no hard-stop) — is fixed by the competing-RuleChange contradiction
   predicate (`fold_derived::detect_competing_rulechange`, the narrowest F8 form); the refutation pin
   flipped RED→GREEN and the register row is Reconciled. Per-act approver-role granularity stays a
   design call (backlog §2a).
2. **X3 automated cross-package harness — ✅ DONE (RUN-07).** The harness (`x3_cross_package_harness.py`,
   `X3-AUTOMATED-SWEEP.md`) mutates the substrate in place and runs the `croft-chat` suite per survivor
   (the single-invocation config is blocked by the separate workspaces; the thin driver is the shape the
   X3 report anticipated). All 61 substrate survivors resolve mechanically: **7 killed, 54 individually
   justified, 0 unjustified.** The 7 killed are R7's content-bound-quorum count path, so **R7's count
   claim is now cross-package mutation-`Verified`** (§7.2). Two findings: `fold_auth.rs` is an
   off-consumer-path duplicate (31 survivors never linked by the suite; register `fold-auth-duplicate`),
   and the role-authorship gate (7) plus Vouch payload validation (10) are uncovered residuals outside
   the R7 count claim.
2b. **EXP-H1 horizon-manifest determinism (§2b) — ✅ DONE (RUN-07).** `local_storage_projection::horizon`
   (pure fold-side, experiment-grade) + `croft-chat/tests/horizon_manifest.rs`: the manifest
   `(frontier_head, sorted open-contradiction byte-heads)` is byte-identical across members and arrival
   orders for mutual expulsion and competing RuleChange; both trigger modes fire at the same fact
   position. Earns manifest-determinism only; §7.6.9 stays `Design`.
2c. **EXP-C1 completeness-ahead contract (§2c) — ✅ DONE (RUN-07).** `local_storage_projection::completeness_ahead`
   + `croft-chat/tests/completeness_ahead.rs`: all four assertions green at loopback / fold grade
   (stall-at-threshold, stamp detection, solicitation reach, formula-valued k = ceil(n/2)). §8.2(e)
   records the loopback exercise; real-NAT path stays X1.
3. **Decide the identity/key-recovery model (I9)** — now the explicit blocker for EXP-5's
   threshold-revoke-over-wire half. Recommended EXP-5 option A (quorum-of-Ed25519 reusing the governance
   k-of-n) is the one path *not* blocked on I9; the MLS-key-distribution half is already spike-realized.
4. ~~**Build B1** (dataplane hash structures) → unblocks A5 (message-continuity, the last re-plant facet).~~ ✅ **DONE (RUN-09 Part 3):** B1 built at loopback, A5 message-continuity landed (§7.6.2 message half → `Modeled`). Real transport + wire pinning are the open follow-ons.
5. **meer P2→P6** (I3–I6): each phase turns one lab experiment (E8/E9/E11/E12) into its running form.
6. **Make the identity/key-recovery decision** (I9) — then the BIP39 spike.
7. **Hardware/boxes when available:** A6 (real-NAT X1), I8 spikes 3 & 7, I7 E4/E0-NAT.

---

## 5. Running the top blocker locally in *this* env (iroh over loopback) — ✅ DONE

The single highest-leverage key is "a live iroh gossip testbed." As of 2026-07-13 it **runs
here**, on loopback, with no relay and no Internet dependency. Two separate `serve` processes
converge over real iroh-gossip to an identical fingerprint (`503af2f0895c9b2d`, both settled) —
see the recipe in `croft-chat/RUN.md` (Same-host recipe). Below is what it took; the probe facts
are retained because they explain the design.

**What's already true here (measured, not assumed):**
- The full iroh stack **builds** (`cargo build -p croft-chat --features iroh-it` → exit 0, 300 MB binary).
- **Loopback UDP works** (`127.0.0.1` ⇄ `127.0.0.1` datagram round-trips).
- The **n0 relay is reachable over HTTPS** through the agent proxy (net_report probes succeed;
  "home is now relay …use1-1.relay.n0.iroh.link").
- **Internet UDP is blocked** (`net_report … udp_v4: false, udp_v6: false`) and **IPv6 is absent**
  (`bind [::]:0 → os error 97`). So there is **no direct/holepunch path to the outside** — only the
  HTTPS relay — which is why the real-NAT experiment (A6/X1) genuinely needs the boxes and can't be
  faked here.

**Why the harness doesn't run as-is:** `croft-chat/src/iroh_bus.rs` (and every `iroh/` spike)
hardcodes `Endpoint::builder(presets::N0)` then `endpoint.online().await`, which spends ~3 s doing
Internet net_report and waits for an n0 relay home. Two processes *on the same host* don't need any
of that — they can connect over loopback directly — but the code never offers that mode. The
`stone-alpha.toml` topology even carries a `relay_mode` field that `iroh_bus` currently ignores.

**The change that was made (small, localized, verified 1.0.0 API):** `iroh_bus.rs` now honors the
topology's `relay_mode` via a `RelayChoice` enum. `"n0"` keeps the relay path (`presets::N0` +
`online()`); `"disabled"`/`"local"` selects direct-only (`presets::Minimal` +
`RelayMode::Disabled`). The one subtlety that made this non-trivial: under `Disabled` there is **no
home relay**, so `endpoint.online().await` — which watches home-relay status — blocks forever; the
direct path instead waits (bounded) for a local IP address to appear before publishing its
`EndpointAddr`. A `croft-chat/localhost.toml` (`relay_mode = "disabled"`) drives it. The two
in-process/​integration convergence tests were also switched to `LocalDirect`, so they are now
hermetic (they previously hung against the unreachable n0 relay).

Result in this env: `cargo test --features iroh-it` green (22 unit + integration), clippy-pedantic
clean, and the two-process loopback `serve` run converges to `fingerprint 503af2f0895c9b2d` on both
nodes. See `croft-chat/RUN.md` → "Same-host recipe."

**What this unlocked (no boxes):** A3 (multi-process convergence + X2 fault injection — `SIGKILL`
a node then heal), A4's M1 fan-out (N local `serve` processes), `croft-chat`'s
convergence-over-real-gossip proofs, and `croft-group` L5's adapter tests. **What still needs the
boxes:** A6/X1 (real-NAT traversal + relay holepunch) — by definition unreproducible where Internet
UDP is blocked.
