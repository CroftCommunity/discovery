# RUN-09 — the runnable remainder (five parts, one branch)

`Branch: claude/run-09-remainder-2rvwf3, off main (RUN-08 merged). 2026-07-15. Rust 1.94.1,
cargo-mutants 27.1.0. Five parts in dependency-and-risk order, each shipping independently (one
commit + this summary section per part), so the branch merges cleanly after any part. This summary is
readable stand-alone: per-part status, red→green evidence for the code parts, every conditional edit
applied or withheld with its reason, and the parked list restated.`

## Per-part status

| Part | What | Status | Commit |
|---|---|---|---|
| **1** | Riders — traceability settlement (FND-T1/T4/T5/T6/T7) + Proofs confirmation | ✅ done | `a9c3035` |
| **2** | Vouch payload-validation coverage (backlog §2d) | ✅ done | `a944317` |
| **3** | B1 → A5: re-plant message-continuity half | ✅ done (loopback / `Modeled`) | `3e3f37a` |
| **4** | RBSR steady-state anti-entropy at loopback (§6.8.1 open half) | ✅ done (loopback / `Modeled`) | `a0add6c` |
| **5** | Fan-out repeated-run measurement | ✅ done (K=5; `fanout-single-run` retired) | this commit |

Both Rust suites (`local_storage_projection`, `croft-chat`) + the `replant-continuity` bridge + clippy
were green at every commit boundary; zero new warnings versus the pre-existing baseline (the
`tables.rs` unused-const warnings are RUN-07-era, untouched). The I9 firewall held throughout.

---

## Part 1 — riders: traceability settlement + the Proofs confirmation (commit `a9c3035`)

**Status: done.** Markdown-only; no code touched, no status tag moved. The five owner-confirmed
(2026-07-15) RUN-08 traceability rulings are applied and recorded in the
`CONSISTENCY-FINDINGS-2026-07.md` settlement section (`## Settlement (RUN-09, 2026-07-15)`):

- **FND-T1 (per-band target form).** One A.9 evidence-linkage note now defines the forward-link target
  by band: an *experiment-earned* tag (`Verified` on real crypto/transport, `Modeled`, `Measured`)
  resolves to a named test/report + RUN; a `Verified-RFC` / literature-anchored tag resolves to its
  primary-source anchor. The ~40-tag substrate band, re-audited under the right form, **closes as
  satisfied**: the `Verified-RFC` / literature rows (§6 RFCs, §7.4.2, §7.6.3–§7.6.4 MSC/RFC + self-cite,
  §10.2–§10.4 MLS/primitive) resolve to their primary source; the experiment-earned substrate rows
  (§4 wire derivations, §6 loopback gossip, §6.6.4 dedup) already carry test+RUN pointers in
  `EVIDENCE-MAP.md` §B. No experiment-earned tag was found still lacking a pointer, so none stays a
  FINDING.
- **FND-T4 (no retrofit).** One A.9 note adopts the `(evidence: …, RUN-NN[, grade])` parenthetical as
  the recommended forward form, not back-fitted. (Used from RUN-09 forward in the Part 3/4 spec edits.)
- **FND-T5.** §10.4's off-ladder `` `Reviewer-judgment` `` de-backticked to plain prose (no eleventh
  rung); `part-2-changelog.md` entry added.
- **FND-T6.** One A.9 note records the legacy `green-real`/`green-model`/`not_yet_emitted` vocabulary
  as alpha-tier only.
- **FND-T7.** The compound qualifier `real-NAT` canonicalized (hyphenated) in the living docs; the
  single drift instance was `MASTER-INDEX.md`. The spec set was already compliant; frozen transcripts
  and the free-noun "real NAT" left as-is.

The reviews-log RUN-09 entry records the **Proofs fold-in authorization confirmed by the owner
2026-07-15**. `EVIDENCE-MAP.md` §D + its open-FINDINGs list mark T1/T4/T6 settled (index, not source).
FND-T2/T3 stay open as recorded (not covered by this run's rulings).

---

## Part 2 — Vouch payload-validation coverage (commit `a944317`)

**Status: done.** Closes backlog §2d; a coverage residual, not a claim — **no spec tag moved**.

**RED → GREEN.** RED-first consumer-path tests `croft-chat/croft-chat/tests/vouch_payload.rs` (9
tests) exercise the fold's I5 Vouch payload gate (`fold_derived.rs:447–472`) end-to-end through the
real `DerivedFold` (the `surface::LocalStore` path): a well-formed Vouch is accepted and shows up as a
derived trust signal via `get_trust_signals` (folded state); empty-context / too-short / truncated /
over-declared / bad-strength are rejected and fold nothing; oversized trailing bytes are accepted. The
tests pass on the clean substrate.

**Prove (the sweep).** The cross-package mutation sweep re-ran over the whole Vouch payload region
(`fold_derived.rs` lines 449/461/462/469/470) — the **10 RUN-07 justified survivors** plus the **9
additional** operator mutants the current tool lists — via the `fold_derived.rs`-scoped harness
(`x3_vouch_harness.py`, over `x3-sweep-data/vouch-region-all-run09.txt`). Result: **19 killed, 0
survived** (wall ~1244 s). Dated addendum appended to `X3-AUTOMATED-SWEEP.md` (the original verdict
text unchanged); raw artifacts under `x3-sweep-data/vouch-*`. Backlog §2d retired.

*Note on the observable:* the read path (`get_trust_signals`) re-validates, so it alone cannot
distinguish a gate mutation that merely mis-rejects a malformed payload; the tests therefore observe
both the derived trust signal (accept case) and the fold's accept/reject outcome, and pin the
minimum-length boundary via the rejection kind (MalformedEnvelope vs AuthorizationFailed) at exactly
37 bytes — that is what kills the two `449` mutants that are otherwise accept/reject-equivalent.

---

## Part 3 — B1 → A5: the message-continuity half of re-plant (commit `3e3f37a`)

**Status: done at loopback grade (`Modeled`).** Built on the existing `replant-continuity` crate; the
membership half of §7.6.2 was already `Verified`, this lands the message half.

**RED → GREEN.** RED: `tests/e12_2_message_continuity.rs` (5 tests) asserts the four continuity claims
over a real re-plant (genesis + authorized add + membership restamp). The RED order was evidenced by
stubbing the B1 detection (the dup and drop tests failed) before the fold logic made them pass. GREEN:
a minimal B1 dataplane hash structure `src/dataplane.rs` — content-addressed, causally-linked records
folding into an arrival-order-independent BLAKE3 digest (test-only serialization, no wire pinning).
Proven at loopback:
- (a) every pre-repoint entry present after, exactly once;
- (b) entries authored in the repoint window land exactly once, in causal order, on the post-repoint
  group;
- (c) both members converge byte-identically across arrival orders (a fully-reversed stream folds to
  the identical digest via buffer-and-retry);
- (d) an injected duplicate (`Fold::Duplicate`) or dropped frame (an unresolved causal gap) is
  detected, not absorbed.

**Conditional edits (all applied — all-green):**
- §7.6.2 continuity sentence upgraded to name both halves, the message half `Modeled` at loopback
  grade, carrying the A.9 evidence parenthetical.
- `EVIDENCE-MAP.md` row (§7.6.2 message half, `Modeled`).
- Backlog E12.2 + E12.7 message facet → done; the item-7 "Build B1 → A5" line struck through.
- `part-2-changelog.md` entry; `MASTER-INDEX.md` A5/B1 → done; crate README section.

**Grade rationale (`Modeled`, not `Verified`).** The records are driven over the *real* re-plant
membership but *delivered by the harness*, not real transport; the record encoding is deliberately not
`[gates-release]` wire-pinned. **Stop rules:** none triggered — the in-flight (b) case needed no
transport machinery at loopback, so nothing was split out. Real over-the-wire delivery and the B1 wire
encoding remain open (Appendix B / B1).

---

## Part 4 — RBSR slice: steady-state anti-entropy at loopback (commit `a0add6c`)

**Status: done at loopback grade (`Modeled`).** §6.8.1's open half: connect-time catch-up was proven;
recovering a frame dropped *between already-connected peers* was not.

**RED → GREEN.** RED: `croft-chat/croft-chat/tests/steady_state_anti_entropy.rs` establishes two
connected members in steady state, then the harness loses one live frame to B with no new join.
Because gossip carries no per-recipient ack, B's `Replicator` buffers no stranded successor — the gap
is invisible to live delivery (B is settled), yet the folds diverge. RED evidenced by stubbing
`missing_frames` to empty (the flow fails to converge). GREEN: a minimal `croft-chat::anti_entropy`
module — a range summary over the `(device, lamport)` key space + a diff-only `missing_frames`
reconcile — detects the 1-frame gap and ships *only* the lost frame; B folds it through the ordinary
`Replicator` and the folds re-converge byte-identically, with no reconnect and no whole-log
re-broadcast.

**Conditional edits (all applied — all-green):**
- §6.8.1 sentence: steady-state anti-entropy → `Modeled` at loopback grade, A.9 evidence parenthetical.
- `EVIDENCE-MAP.md` row (§6.8.1, `Modeled`).
- Backlog M2 row updated; `part-2-changelog.md` entry; `MASTER-INDEX.md` A3 note.

**Grade rationale (`Modeled`, not `Verified`).** The whole-set `(device, lamport)` range compare is
the simplest RBSR form, at loopback. The range-*partitioned* production construction (Willow 3d-range
versus Negentropy) and real-transport loss stay open (§5 / Appendix B). **Ship-without rule** (Part 4
was the first to drop if the run ran long): not invoked — the part landed.

---

## Part 5 — fan-out repeated-run (commit this commit)

**Status: done — retirement fired.** The loopback-cheap arm of the `fanout-single-run` register row.

**Measurement.** Re-ran the FANOUT-M1 sweep **K = 5** at N = 2/4/8/16 over real iroh-gossip
(`--features iroh-it`, `relay_mode = "disabled"`, `RUN_SECONDS = 30`), same harness. Per-N spread
(min / median / max), from `fanout-data/repeated-run09.csv` (150 node-samples), reported as a dated
addendum to `FANOUT-M1.md`:

| N | `live_sent`/node | creator `resync_sent` | `head_ms` | fully-settled | fingerprints |
|---|---|---|---|---|---|
| 2  | 5 / 5 / 5    | 3 / 3 / 3       | 305 / 682 / 1034   | 2/2 | ✅ all 5 runs |
| 4  | 9 / 9 / 9    | 15 / 15 / 15    | 852 / 860 / 1096   | 3–4/4 | ✅ all 5 runs |
| 8  | 17 / 17 / 17 | 64 / 65 / 67    | 980 / 1050 / 1390  | 1–2/8 | ✅ all 5 runs |
| 16 | 33 / 33 / 33 | 349 / 401 / 422 | 2899 / 3569 / 4049 | 0–1/16 | ✅ all 5 runs |

**Disposition — the spread is narrow, so the row retires.** `live_sent = 2N+1` reproduced **exactly**
(zero variance, 150/150 samples); head convergence held in **every** run at every N; the super-linear
hub-resync **shape** reproduced with a **tight** band (N = 16: 349–422, median 401 — the single-run
479 refined *downward*, not widened). The magnitude is now a measured band, not a single indicative
point. Full-settle degraded past N ≈ 8 reproducibly (the flag Part 4 answers).

**Conditional edits (all applied — spread supports the magnitudes):**
- `fanout-single-run` **retired (Reconciled, RUN-09)** in `SPEC-DIVERGENCE-REGISTER.md`; `hermetic-gossip`
  is now the only Active divergence.
- §11.11 measurement-#1 caveat clause updated to record the replicated band.
- `FANOUT-M1.md` dated addendum + `Serves` header; `EVIDENCE-MAP.md` bounds; `part-2-changelog.md` entry;
  `MASTER-INDEX.md` + `EXPERIMENT-BACKLOG.md` Active-rows updated. Driver `scripts/fanout-repeated.sh`
  committed alongside the CSV.

**Residual, carried forward honestly (not a divergence row):** the resync *magnitude* is
star-bootstrap-topology-sensitive (a mesh bootstrap would spread the hub-absorbed `NeighborUp` load);
hardware hot-N = 500+ stays X1-adjacent and parked.

---

## Findings walk

**No new owner-call FINDINGS opened this run.** The judgment calls RUN-09 made were resolved in place
under the run's stated rules and recorded at their site, not deferred:
- The Part 3 and Part 4 grades were set to `Modeled` under the "when in doubt, `Modeled` and say why"
  rule (both say why: harness delivery, not real transport; whole-set compare at loopback).
- The Part 5 `fanout-single-run` retirement was taken under the ruling's own decision gate (narrow
  spread supporting the recorded magnitudes) and is documented with the replicated data in the
  `FANOUT-M1.md` addendum, the register Reconciled row, and the §11.11 clause.
- The Part 1 settlements are recorded in `CONSISTENCY-FINDINGS-2026-07.md` `## Settlement (RUN-09, …)`;
  the RUN-08 findings not covered by this run's rulings (FND-T2, FND-T3) stay open, unchanged.

One residual is carried forward as a note (not a divergence, not a FINDING): the fan-out resync
*magnitude* is star-bootstrap-topology-sensitive; a mesh bootstrap would spread it. It is recorded in
the register's Reconciled row and the `FANOUT-M1.md` addendum.

## The parked list (restated, so the board state is readable here)

None of the following are in this run, by design:

- **I9 — the trust tier.** No release predicates, threshold shares, revoke-authority ordering, or
  co-sign-vs-vote work. The I9 firewall held across all five parts.
- **X1 — real-NAT.** The relay + holepunch path stays an open honesty boundary (needs the secroute
  boxes; unreproducible where Internet UDP is blocked). All RUN-09 code is loopback.
- **hot-N = 500+.** Fan-out magnitude at representative scale is out of scope (Part 5 is the
  loopback-cheap repeated-run arm only).
- **`[gates-release]` + BLAKE3 wire-freeze.** No byte-level encoding was pinned; the B1 record
  encoding (Part 3) and the horizon manifest stay test-only serialization.
- **Frozen-emitter integration.** No iroh was wired into the conformance emitter (§10.5's over-the-wire
  authority-distribution residual).
- **Layer-2 design frontier.** The centerless-meets-center seam and the broader Layer-2 design work are
  untouched.
