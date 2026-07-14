# Experiment backlog — defined but not-yet-run

A snapshot review (2026-07-13) of every experiment/spike that is **defined** (has a plan, spec,
or procedure written) but **not yet run or not yet complete** across `alpha/`. The point is
legibility: each item names what it is *for* and what it is *blocked on*, so the remaining work
can be picked up and pushed into the corpus.

Maturity legend:
- **Specified** — has a written method + a pass/falsification condition; runnable as soon as its
  blocker clears.
- **Sketched** — named with intent, no procedure yet; needs a plan pass before it runs.
- **Parked/gated** — deliberately deferred pending a hardware resource, a tool, or a design/legal
  decision; do not start blind.

Sources are the plan/roadmap/spec/next-session/TEST-LOG docs inside each experiment.

---

## 0. Doc hygiene

- ✅ **Done (2026-07-13).** Both manifests reconciled to all 12 experiments: the root
  `README.md` listed only 8 and `alpha/README.md` only 7 (it also omitted `iroh` and still used
  the pre-staging `experiments/` tree root). The four re-plant-line experiments
  (`local_storage_projection`, `mls-replant`, `replant-continuity`, `croft-chat`) plus `iroh` were
  added with summaries; both now cross-link this backlog and `MASTER-INDEX.md`.
- **Still open:** `iroh/` carries the **Alt.Drive → Croft.Drive rename** chore (TEST-LOG B5) and
  unconverted `[HYPOTHESIS]` → `MEASURED` tags in `RELAY-PLACEMENT-LAB-SPEC.md` for runs already
  done. Both are larger sweeps within `iroh/` — sequenced in `MASTER-INDEX.md`, not done here.

---

## 1. Cross-cutting blockers (the keys that unlock the most work)

Most unrun items are gated on one of five things. Clearing a key unblocks a whole column below.

| Key | What it unblocks |
|---|---|
| ✅ **Local iroh gossip testbed (loopback, no relay)** — *done 2026-07-13* | X2 fault injection, M1 fan-out, croft-chat multi-process convergence, croft-group L5. Runs on one host via `croft-chat/localhost.toml` (`relay_mode = "disabled"`); see `croft-chat/RUN.md` + `MASTER-INDEX.md §5` |
| **Reachable secroute boxes + public UDP ingress** (real NAT) | Only X1 (real-NAT traversal) and iroh E0-NAT/E4 — the items that *require* a relay-holepunch path, which can't exist where Internet UDP is blocked |
| **`cargo-mutants` installed** ✅ (installed 2026-07-13) | Battery 8 X3 (the auth/governance trust gate). Remaining X3 work is the automated cross-package sweep, not the tool |
| **macOS + iOS hardware** | iroh Spike 3 (macFUSE / B4), Spike 7 (iOS-iroh-blob feasibility — the iroh→Veilid breakpoint) |
| **`ipvsadm` on the boxes** | iroh E4 (LVS frontend HA) |
| **A design/legal decision** | identity & key-recovery model (E3.3), geer (compellability / legal review) |

---

## 2. Drystone governance fold + MLS re-plant
`local_storage_projection` · `mls-replant` · `replant-continuity` · `croft-chat`
Master ledger: `croft-chat/plans/2026-07-11-1-plan-next-experiments.md` (Batteries 5–8).

Batteries 5–6 (completeness + fold conformance) and the Rung-A MLS mechanics (E12.1, E12.3–E12.7)
are **done and green**. What remains:

| Item | For (claim under test) | Maturity | Blocked on |
|---|---|---|---|
| **E12.2** — atomic-swap *message* continuity | An in-flight conversation survives the §7.6.2 re-plant repoint without message loss/dup | Specified | Drystone dataplane hash structures (Rung B) |
| **E12.7 message facet** | The message-continuity half of the bridge (membership half is done) | Specified | pairs with E12.2 |
| **M1 fan-out half** | ✅ **DONE (RUN-01 EXP-1, 2026-07-14).** Fan-out curve captured over real iroh-gossip at N=2/4/8/16 (`croft-chat/FANOUT-M1.md`): per-node gossip cost **linear** (`live_sent=2N+1`), aggregate O(N²), head-convergence holds at all N (fingerprints match). **Flag:** connect-time resync is super-linear on the bootstrap hub and full-settle (`pending==0`) doesn't complete past N≈8 in-window — corroborates the open RBSR/steady-state gap. Register: `fanout-single-run` (proxy-measurement, magnitude indicative). | ✅ Complete (curve + shape) | — (loopback testbed) |
| **M2** — return-backfill vs dormancy | Cost of a returning member catching up vs staying dormant, at 1/7/30/90-day gaps | Specified (modeled lower-bound runnable now against redb history) | **Mechanism now built** — sync-on-connect resync (`iroh_bus`, `Event::NeighborUp` → re-broadcast retained log). M2 is the *sizing* study that remains (push-resync vs pull-on-connect, cost at 1/7/30/90-day gaps) + steady-state anti-entropy |
| **X1** — live cross-host over real NAT | Convert in-process fingerprint-equality into a real-network one | Specified (`RUN.md` cross-host recipe) | secroute boxes + NAT workstation (genuinely needs real NAT) |
| **X2** — fault injection during convergence | Kill/crash/heal mid-converge → same head, no reversion, catch-up | ✅ **DONE — all green (loopback testbed, 2026-07-13)** — `scripts/x2-fault-injection.sh` | crash-consistency + monotonic no-reversion + **catch-up** all PASS (`A head == B head`). Catch-up was first *refuted* (gossip dedups re-broadcasts) then *fixed* with a prototype nonce backfill in `iroh_bus`. See ledger Phase 7 |
| **X3** — `cargo-mutants` re-sweep on `fold_auth`/`governance` | A surviving mutant in the authority/threshold path = a real hole in the trust claim | Substrate sweep DONE; cross-package harness open | ✅ **Substrate sweep run (RUN-01 EXP-3, 2026-07-14):** 120 mutants → 54 caught, **0 survivors in threshold-counting** (`governance.rs` 13/13), 61 survivors **all** in the cross-package-covered authorization-*decision* path (`check_authorization`/`role_ge_*`/`act_subject`/`rule_change_approval_subject`). Demonstrated a survivor (`rule_change_approval_subject→const`) is killed by `croft-chat`'s `approval_for_a_different_change_does_not_count`. No real hole found. See `local_storage_projection/X3-CROSS-PACKAGE-SWEEP.md`. **Remaining:** the *automated* cross-package harness (mutate substrate while running `croft-chat`'s suite so all 61 survivors resolve mechanically) — separate crates/`Cargo.lock`, budgets the slow consumer suite. |
| **Fold open items** | ~~RuleChange thresholds~~ (✅ done); ~~contradicted-group byte-head naming~~ (✅ **done, RUN-01 EXP-4** — `competing_quorums.rs::contradicted_group_byte_head_is_min_hash_order_independent`: the byte-head is exactly `min(H(F),H(G))`, order-independent); **two-competing-quorums → §7.6.1** (⚠️ **FALSIFIED, RUN-01 EXP-4** — see finding row below); per-act approver-role granularity (**design-gated** — see finding row below); live "catching up…" TUI indicator (App holds no Replicator; UX, skipped unattended) | Partially done; 2 design-gated | — |

### 2a. Fold findings from RUN-01 EXP-4 (design-gated — do not decide autonomously)

**FINDING — competing RuleChange quorums auto-resolve (§7.6.1 gap).** RUN-01 EXP-4 refuted the
competing-quorum case: two concurrent conflicting RuleChanges on the same rule, each carrying a valid
k-of-n quorum, **silently auto-resolve order-dependently** (last-folded wins; `fork="clean"`, no
hard-stop) — an I5 violation on exactly the shape §7.6 says must escalate. Pinned by
`competing_quorums.rs::two_competing_rulechange_quorums` (refutation) and register row
`competing-quorum-autoresolve`. **Why it's here and not fixed:** the fold's contradiction predicate set
(mutual-expulsion, removed-then-included, role-thrash — see `2026-07-12-1-design-concurrent-contradiction.md`)
does not cover RuleChange, and adding a predicate is a **design decision**. *Options for the human:*
(A) a **same-rule-different-value** predicate: two concurrent RuleChanges whose `(rule_key)` matches and
`new_value` differs → `Contradiction`, byte-head `min(H(F),H(G))`, retain the pre-change rule value (no
verdict); narrowest, mirrors mutual-expulsion. (B) a broader **same-subject** predicate keyed on the
`rule_change_approval_subject` content hash (any two concurrent changes to the same rule slot). (C)
**causal-order-only**: require RuleChanges to a given rule to be totally ordered (reject a concurrent
second as incomplete) — shifts the cost to the client. Recommend (A) first (exact, symmetric, smallest
escalation surface), matching the design note's "implement the narrowest shape first" discipline.

**FINDING — approver-role granularity is role-agnostic (undecided).** Step 5.6 counts distinct approver
personae **by lineage regardless of role** — a Member's `Approval` currently counts toward a RuleChange
or RoleGrant quorum the same as an Admin's. Whether an act's quorum should require approvers holding a
minimum role *for that act* is an **undecided design question** (the spec's R-series is
mechanism-neutral; nothing decides approver role-gating). Not tested/implemented here — deciding it is a
trust-model call. *Options for the human:* (A) **role-agnostic** (status quo — any member's approval
counts; simplest, but a low-privilege member can help meet a high-privilege quorum). (B) **per-act role
floor** — each act type carries a minimum approver role (e.g. RuleChange/RoleGrant need Admin+
approvers), enforced in `gather_approvers`/Step 5.6 by filtering approvers below the floor before
counting. (C) **weight-by-role** — richer, likely over-engineered for now. No recommendation without
the trust-model owner; flagged so the next session decides deliberately rather than by omission.

---

## 3. croft-group (shared-core / per-shell)
`croft-group/plans/2026-06-22-1-plan-croft-chat-cli-group-core.md` — happy-path slice CLOSED;
L1–L6 sequenced, not built. Each gets its own plan.

| Item | For | Maturity |
|---|---|---|
| **L1 — Real identity** | hardcoded handle → real DID/lineage identity in `ChatMessage` + versioned wire (may fold into L2) | Sketched |
| **L2 — MLS / encryption** | `Frame` payload becomes MLS-ciphertext; key/epoch state enters the core; `Zeroize` applies | Sketched |
| **L3 — Fork/merge + reconvergence-per-plane** | multi-head DAG + per-plane reconvergence policy bound into the asset hash | Sketched |
| **L4 — Governance / delegate planes** | threshold group-principal, capability-vs-authority delegates, the rights-floor | Sketched |
| **L5 — Real-iroh Transport adapter** | second `Transport` impl over iroh-gossip; real async runtime; same scenario tests. **Goes live P2P here.** | Sketched (blocked on iroh testbed) |
| **L6 — Shared-shell composition** | shared shell crate hosting feed + group ponds (Tauri/web); cross-pond read-only awareness (broker deferred) | Sketched |

> Note the overlap with §2: croft-group L2–L5 re-derive, in the shared-shell architecture, mechanics
> the Drystone line has already proven in `local_storage_projection`/`mls-replant`. Worth deciding
> whether L-series builds on those crates rather than re-implementing.

---

## 4. automerge-partial-reconstruction

| Item | For | Maturity | Blocked on |
|---|---|---|---|
| **0.7 confirmation** | ✅ **DONE (RUN-01 EXP-2, 2026-07-14).** Re-ran `src/main.rs` against `automerge = "0.7"` (→ 0.7.4) on Rust 1.94.1: all 4 partial-reconstruction invariants hold on the ship target (only change-hash values differ). Register row `automerge-0.6.1` → Reconciled. See `automerge-partial-reconstruction/REPORT.md`. | ✅ Complete | — (Rust 1.94 present) |

---

## 5. croft-app-phase0
Phase 0 (M1–M6) is **complete**. One design deferral remains, not a runnable experiment:
the moderation/labeling question is deferred with M6's live-adapter reversibility (see `BUILD-SPEC.md`).

---

## 6. iroh substrate
Largest surface. Done items (E1/E3/E5/E6/E7/E10/E11-logic/E12-local, meer P0/P1, conformance-core,
Part A, B-series, T-series) excluded. Sources: `RELAY-PLACEMENT-LAB-SPEC.md`, `TEST-LOG.md`,
`RESUME-NEXT-SESSION.md`, `NEXT-SESSION-2026-06-16.md`, `docs/roadmap.md`, `docs/phase-0-spikes.md`.

### 6a. Relay & placement lab (SPEC §4)
| Item | For | Maturity | Blocked on |
|---|---|---|---|
| **E4** — LVS frontend HA + within-shard persistence | IPVS director gives HA + intra-shard balancing without breaking DNS co-location; source-hash pins a peer across reconnects | Specified | `ipvsadm` + public ingress |
| **E8** — multi-namespace peers vs superpeer bridge | crossover density where a straddling superpeer beats N per-peer relay connections | Specified | meer P2 (bridge mode) |
| **E9** — meer confidentiality tiers (T1/T2/no-mirror) | each tier's convergence cost/capability curve (Tier 0 done) | Specified | meer P3/P6 |
| **E0** — hole-punch FAILS (CGNAT/mobile) | worst-case relay cost when the pair stays relay-bound | Specified | NAT'd Mac + UDP 3478 / TCP 3343 ingress |
| **E0** — reconnect-storm CPU driver + sustained-transfer | find NIC/handshake walls under a reconnect storm + long-lived transfer | Sketched | — |
| **E2** — DNS-origin + pkarr integration | replace `MemoryLookup` with the real DNS/pkarr-published placement record | Sketched | — |

### 6b. Media-round follow-ons (SPEC §4a)
| Item | For | Maturity |
|---|---|---|
| **E11 full form** | real moq-rs Tracks + GStreamer/Opus + WebTransport browser reach (only relay logic is green) | Parked (= meer P5) |
| **E11 — TC-META1/TC-META2** | media metadata-leak bound + CBR-padding measurements (AR-4 for media) | Sketched |
| **E12 transport-carried** | SFrame-over-MLS keying over the live E10 datagram rig, real Opus/RTP + RFC 9605 wire (green only with synthetic frames) | Sketched |
| **E10 — mesh-vs-meer comparison** | remaining E10 follow-on (raw-UDP baseline done) | Sketched |

### 6c. Meer build (production, TDD-gated — `meer-superpeer-design.md` P2–P6)
The *running form* of E8/E9/E11/E12. P0/P1 (Tier-0 blind mirror) done.
| Item | For | Maturity |
|---|---|---|
| **Meer P2 — bridge mode** | straddle 2 namespaces/relays (runs E8) | Specified |
| **Meer P3 — Tier-1 + no-mirror + reliability/overlap curve** | runs the rest of E9 | Specified |
| **Meer P4 — RoQ SFU role** | transport form of E12 | Specified |
| **Meer P5 — MoQ relay role** | transport form of E11 | Specified |
| **Meer P6 — Tier-2 (policy-gated) meer** | key-holding, policy-gated mirror | Specified |

### 6d. Faithful follow-ons + conformance gaps (production, TDD)
| Item | For | Maturity | Blocked on |
|---|---|---|---|
| **MLS key-distribution over the wire** | make the modeled verifying-key registry a real over-iroh distribution (standing FAITHFUL honesty boundary) | Specified | — |
| **Threshold revoke-authority as real k-of-n over the wire** | replace the MD-G5 sha-256 MAC stand-in with a genuine k-of-n authority signature | Specified | — |
| **Conformance vectors cats 7/8/9** (AR / visibility / freshness) | recorded `not_yet_emitted`; revoke-authority-threshold vector is a `PLACEHOLDER` | Sketched | the k-of-n work above |
| **Domain-tagged pre-image reconciliation** | decide whether `lineage-core` (plain sha256) + the iroh spike adopt CROFT-PROTOCOL §2 domain-tagged genesis/topic pre-images | Sketched | — |

### 6e. Smaller open-edges (spike-class, RESUME §8 E)
`metadata-leak-under-failed-ops` · E6 steady-state goodput + bandwidth-cap · E7 churn storm ·
`fold_by_lineage` cost under churn · AR-4 relay-side timing+volume packet capture (quantify the
leak the bound only characterized). All **Sketched**.

### 6f. Phase-0 de-risking spikes (`docs/phase-0-spikes.md`)
| Item | For | Maturity | Blocked on |
|---|---|---|---|
| **Spike 1** — iroh-docs 10K-entry manifest sync + load | sync timing/memory + deterministic-nonce key behavior (B2 did 10 entries same-process) | Specified | — |
| **Spike 3** — macFUSE hello-world mount (= B4) | encrypt-on-write FUSE mount, throughput + app-compat | Specified | macOS hardware |
| **Spike 4** — full dumbpipe-shape ticket pairing | ticket encoder/QR + 6-digit code + 4-word BIP39 confirm + MitM negative test (B3 did the bootstrap seam) | Specified | — |
| **Spike 5** — iroh-docs vs version-vector ADR | the go/no-go decision doc | Specified | Spike 1 results |
| **Spike 6** — DESIGN.md update + retro | close-out chore | Specified | — |
| **Spike 7** — iOS-iroh-blob runtime feasibility | on-device build + fg/cellular fetch + wifi↔cellular handoff + APNs background sync + battery — the iroh→Veilid breakpoint | Parked | physical iOS device |

### 6g. Identity / key-recovery (NEXT-SESSION EXPLICIT TODOs — the largest open problem)
| Item | For | Maturity |
|---|---|---|
| **Recovery model — quorum social recovery vs minimal-central-authority VC issuer** | spec the two candidate recovery models together (proof-ledger E3.3) | Parked (kept out of autonomous work by design) |
| **BIP39 paper-recovery round-trip spike** | recoveryKey ↔ 24-word mnemonic (KAT-verified) then secretbox-wrap the masterKey — cheapest first step for the above | Sketched |

### 6h. Standalone / after-layer spikes (NEXT-SESSION · roadmap · DESIGN §14)
| Item | For | Maturity | Blocked on |
|---|---|---|---|
| **Track 2** — cross-host iroh-docs 3-replica | 3-machine convergence + reconnect-after-partition + durable-queue + LWW-clobber evidence (B2 same-process only) | Specified | boxes (largely subsumed by E3) |
| **HashSeq single-file striping** | stripe one large blob across providers without OOM (currently OOM-kills) | Sketched | — |
| **Automerge-over-iroh interactive-artifact spike** | CRDT sync for edits/reactions/read-receipts + "declare consistency model per artifact type" | Sketched | — |
| **DESIGN §14 open-question spikes** | idle-node memory footprint; NodeId-change-on-reinstall re-pair flow; SQLCipher-value threat-model walkthrough | Sketched | — |

### 6i. Gated — do not start without the resource/decision
- **geer (gating-peer)** — design only; blocked on legal review (compellability).
- **S3/S4** (design gate G5), **T8** (UX decision), **T10** bsky / **T13** iOS host — named as gated;
  the T/S items point into the sibling `discovery`/`Proofs` repos, not this one.

---

## Recommended execution order

**Done this line of work (2026-07-13):** the local iroh gossip testbed (loopback, no relay);
**X2** fault injection (all green — crash-consistency + no-reversion + catch-up via sync-on-connect);
`cargo-mutants` installed; **RuleChange thresholds** enforced; the `Session` governance emit API. Of
the spec-deltas surfaced, `x2-backfill`, `rulechange-quorum`, and `handcrafted-assertions` are
reconciled; only `hermetic-gossip` (needs the boxes / X1) stays active. See
`SPEC-DIVERGENCE-REGISTER.md`.

Remaining, in leverage order:

1. **Runnable today, no new infra** — **A4 / M1 fan-out** (N local `serve` processes on the
   testbed); automerge 0.7 confirmation (Rust 1.94 present); iroh Spike 1 + Spike 4; MLS
   key-distribution-over-wire + threshold-revoke-over-wire (unblocks conformance cats 7/8/9); the
   remaining Drystone fold open items.
2. **X3 automated mutation sweep** — the tool is installed; build the cross-package harness (V5′
   positive coverage lives in `croft-chat`) and budget the slow substrate suite.
3. **Build B1** (dataplane hash structures) → unblocks A5 (E12.2 + E12.7 message continuity).
4. **Meer build P2→P6** — each phase turns one lab experiment (E8/E9/E11/E12) into its running form.
5. **Decide the identity/key-recovery model** — the largest open design problem; start with the
   BIP39 round-trip spike.
6. **Hardware / boxes when available** — X1 real-NAT (the last active spec-delta); macFUSE (Spike 3);
   the iOS feasibility spike (Spike 7, the iroh→Veilid decision point); E4/E0-NAT.
