# RUN-11 — Settle (riders + Map fixes), refresh the living docs, and build

`Branch: claude/run-11-settle-build-mbsal3, off main (RUN-09 + RUN-10 merged). 2026-07-16. Parts 1/1b
markdown; Parts 2–4 code, each shipping independently. First run under the new site gate:
site/build.py (broken-ref gate + emitted-HTML anchor audit) ran clean — 0 hard-gated unresolved — at
every commit boundary; it is now part of this project's definition of green. The I9 firewall held
throughout; the parked list is unchanged. This summary is readable stand-alone.`

## Per-part status

| Part | What | Status | Commit |
|---|---|---|---|
| **1** | Riders — confirmed rulings + the Map fixes | ✅ done | `8af5dac` |
| **1b** | Living-docs currency refresh | ✅ done | `d2c9124` |
| **2** | Build L2a — the MLS-sealed happy-path frame | ✅ done (`Verified`/loopback) | `a88a77c` |
| **3** | The group-principal seam spike (backlog §2e) | ✅ done (`Design`; one FINDING-stop) | `a75da60` |
| **4** | Message continuity over real transport (first-to-drop) | ⛴️ dropped at the ship-without rule; RED shaped (§2f) | `79adc5e` |

Site gate: clean at every commit (0 hard-gated unresolved; the 7 companion/exploratory unresolved are
the pre-existing benign baseline). Emitted-HTML anchor audit: 0 broken cross-file / same-file anchors.

---

## Part 1 — riders: the confirmed rulings (owner, 2026-07-15) + the Map fixes

Per-rider before → after. All recorded in the `CONSISTENCY-FINDINGS-2026-07.md` `## Settlement (RUN-11,
2026-07-16)` section; `part-2-changelog.md` entries added for riders 3, 4, 8. **No experiment status
tag moved.**

1. **FND-T2 ratified.** *No edit* — recorded that the RUN-08 §10.5 wording stands as reconciled (the
   footnote already names the over-the-wire authority distribution as the residual); the two senses of
   "emitted" may be split explicitly whenever that footnote is next touched, not before.
2. **FND-T3 confirmed.** *No edit (doc-comments unchanged)* — the four inferred test §-mappings stand
   as owner-confirmed: `convergence.rs` → §7.3; `iroh_convergence.rs` → §6.10/§7.3 (loopback);
   `regress_free.rs` → §7.3; `dedup.rs` → §6.6.4.
3. **FND-T4 applied, narrowly.** The standard `(evidence: …, RUN-NN[, grade])` parenthetical reshaped
   the existing inline prose in three of the four candidate claims, no information added or dropped:
   - **§7.2 R7** — before: `` `Verified` … (RED→GREEN: `rulechange_threshold_enforced.rs`; … via the
     session API: `rulechange_quorum_via_api.rs`; … cargo-mutants sweep — RUN-07, X3 …) ``; after:
     `(evidence: `rulechange_threshold_enforced.rs` and … `rulechange_quorum_via_api.rs`, plus the … X3
     sweep, RUN-07, `Verified`): …`.
   - **§7.3.2** — before: `(RED→GREEN, `two_competing_rulechange_quorums`, RUN-03)`; after: `(evidence:
     `two_competing_rulechange_quorums`, RUN-03, `Modeled`; RED→GREEN)`.
   - **§8.2(e)** — before: `(RED→GREEN plus the … cargo-mutants sweep, §7.2 R7, RUN-07 X3)`; after:
     `(evidence: the §7.2 R7 count tests — RED→GREEN plus the … X3 sweep, RUN-07, `Verified`)`.
   - **§7.6.2 membership half — FINDING-stopped.** Its RUN-NN component is missing: the membership half
     was imported as already-`Verified` from the standalone experiments corpus (`replant-continuity`'s
     `e12_7_*` tests, commit `d52ed6f`) and carries no discovery-RUN stamp; EVIDENCE-MAP row 52 lists
     the test but no RUN. Per FND-T4's own rule ("a FINDING if any component turns out missing") it was
     **not** reshaped. This finding stays open.
4. **FND-R10-1 applied.** §5.10, before: *"What is unworked is the **key rotation scheme**, how the
   Group and its members jointly own the namespace and how the key rotates under churn, and whether the
   communal namespace is primary or secondary."* → after: reframed per the seam brief — a communal
   namespace has **no shared whole-namespace secret to rotate**, so the question decomposes into
   per-subspace write authority (§4.5) and the fold-gated asset key (§5.11), leaving a near-free
   identifier assignment plus the primary-vs-secondary choice; `Design` tag, seam brief cited
   (`beta/impl/drystone-design/group-principal-seam.md`).
5. **FND-R10-4 bannered.** One correction banner atop `alpha/Proofs/FROZEN-NOTICE.md` (frozen body
   untouched): the "emits categories **1–6**" line understates the folded core, which emits and
   re-proves categories **1–9 (66/0, RUN-08)**.
6. **FND-R10-5 applied.** The parked design is referred to as the "resolution-ACL (croft-group L3)";
   croft-group L2 (MLS) does not depend on it. The L2-readiness brief and backlog §3 already carry the
   correct label (the model); a one-line disambiguation note was added at the collision's origin
   (`alpha/thinking/the-shape-of-disagreement.md` §4, whose own internal "Layer 2" names the resolution
   ACL). No other living doc conflates the two.
7. **Emitter decision recorded.** `EMITTER-INTEGRATION-BRIEF.md` annotated: *"Decided: Option C — defer
   to the `[gates-release]` pass (owner, 2026-07-15); Option B remains the fallback."* The §10.5
   residual line stays as-is.
8. **The Map fixes.** (a) The flagged duplicate §7.6 back-Map line is **not present** in the current
   tree — a single §7.6 entry stands at `## 0. Map`, so nothing was removed (recorded so the audit is
   not left with a phantom, same disposition as RUN-05 FND-8). (b) The surviving copy, before:
   *"…message-continuity half open"* → after: *"…message-continuity half `Modeled` at loopback grade
   (RUN-09)"*, matching the §7.6.2 body and the EVIDENCE-MAP row.

**Site-gate result:** clean (gate + anchor audit) after the Part 1 batch — 0 hard-gated unresolved,
0 broken anchors.

---

## Part 1b — living-docs currency refresh (the currency diff list)

Living docs only; every change provable against the RUN-09/10 summaries and the current registers.
Site gate re-run after the part: clean.

- **`EXPERIMENT-BACKLOG.md` execution order rewritten.** Done-since-snapshot preamble extended through
  RUN-11 Part 1 (Vouch §2d; the E12.2/E12.7 message facet; the M2 steady-state slice; the fan-out
  repeated-run; the traceability settlement; RUN-07/08/10 landings all struck with their runs). The
  forward queue is now: **L2a (Part 2)** → **§2e seam spike (Part 3)** → **message continuity over real
  transport (Part 4)** → **range-partitioned RBSR** (Willow 3d-range vs Negentropy, a read-then-build)
  → **croft-group L2b+** → **the parked list verbatim**.
- **`MASTER-INDEX.md` pressing-edges narrative refreshed** to the post-RUN-10 board: one Active
  divergence (`hermetic-gossip`, X1-gated); one open design call (**I9**); the parked release pass
  (`[gates-release]` + BLAKE3); the **resolution-ACL (croft-group L3)** frontier; everything else
  `Modeled`-or-better at its stated grade or shaped as a RED-able backlog row.
- **`open-threads.md` brought current:** the §5.10 communal-namespace-rotation thread notes the seam
  brief's dissolution (per the Part 1 reframe) and that the Meadowcap-composition `[confirm]` is
  answered affirmatively; a **§6.8.1** steady-state anti-entropy thread added (`Modeled` at loopback,
  RUN-09; the range-partitioned production form open). The recovery thread was already current (Tier-1
  lock landed RUN-08; the trust tier is the open call) — verified, no edit.

---

## Part 2 — build L2a: the MLS-sealed happy-path frame

Executed backlog §3 L2a exactly as shaped in `CROFT-GROUP-L2-READINESS.md` §4. **Reuse-as-condition
honored:** built *on* `lineage-mls` (the openmls `Device` wrapper that `mls-welcome-over-iroh` reuses
for the Welcome and `mls-replant`/`replant-continuity` reuse for the re-plant re-key), never
re-implementing their mechanics.

**Architecture.** New standalone crate `croft-group/crates/group-seal` — own `Cargo.lock`, empty
`[workspace]` (detached from the pure `group-core` + `croft-chat-cli` workspace, added to its
`exclude`), so **openmls never enters the pure workspace's lock** (verified: 0 `openmls` in
`croft-group/Cargo.lock`). It depends **on** `group-core` (the core stays crypto-free and WASM-clean —
DECISION 1/4).

**RED → GREEN (evidenced).** The six shaped assertions landed as `tests/l2a_sealed_frame.rs` first.
Assertion 1 (seal≠plaintext) was driven RED by a temporary identity-seal stub (`seal` returned the
plaintext frame) — the test failed on "the sealed frame must not contain the plaintext body 'hi'" —
then GREEN on the real `Device::send` MLS seal. All six green; clippy-pedantic + fmt clean; the pure
croft-group workspace still builds and tests openmls-free.

**Assertions landed (all six):** (1) seal ≠ plaintext and round-trips through real ciphertext; (2) a
no-key peer's `open` fails observably (no panic) and the core `Model` is byte-identical via
group-core's existing hostile-input `FrameDropped` discipline; (3) a real Welcome distributes the read
key (`epoch_secret` match) then decrypts; (4) a governed removal re-keys the departed reader out (PCS)
— retained member reads, removed member cannot — with no authority knob; (5) `EpochSecret` is
Zeroize-on-drop / no-`Debug` and group-core carries no crypto dep; (6) firewall guard — the public API
exposes no revocation-authority / co-sign / recovery-tier / projection surface.

**Grade (A.9):** `Verified` at **loopback grade**. Rationale: real openmls 0.8.1 crypto (real seal/
unseal AEAD, real Welcome key distribution, real PCS re-key), consistent with the sibling `Verified`
rows §10.5(a) and §7.6.2-membership; the honest bound is **in-process harness delivery** (no real
transport, no B1/wire pinning) and **real-NAT = X1**. **Mechanism half only (R1–R7);** R8-tier/R9/R10
(the authority/projection halves) stay firewalled (I9 / the parked resolution-ACL, L3). No scope-wall
stop triggered — no assertion required the authority half.

**Conditional edits (applied — all-green):** backlog §3 L2a row → done (RUN-11); EVIDENCE-MAP row
(§10.2/10.5(a)/7.6.2 croft-group L2a, `Verified`/loopback); `CROFT-GROUP-L2-READINESS.md` §4 landing
note; `MASTER-INDEX.md` C1 row.

---

## Part 3 — the seam spike: execute backlog §2e as shaped

Executed §2e per `beta/impl/drystone-design/group-principal-seam.md`, RED-first. New standalone
`Design`-grade crate `alpha/experiments/group-principal-seam` (blake3 for the content-addresses,
test-only serialization).

**RED → GREEN (evidenced).** The five assertions landed as `tests/seam.rs`; RED was driven by
temporarily dropping the authority-generation floor in `capability_valid` (a superseded pre-change
capability wrongly stayed valid) — the re-issue-not-revoke assertion failed — then GREEN on the real
floor check. clippy-pedantic + fmt clean.

**Assertions landed (all five):** (1) the Group principal **is** the genesis hash `H(tag ‖ group_id)`,
stable across churn (no whole-namespace secret to rotate — F1); (2) a persona is a **self-authorizing
subspace** (per-subspace ownership, no grant); (3) capability issuance is **downstream of the folded
Group Role**; (4) **re-issue, never revoke-in-place** across a fold-driven authority change — the
revoked persona's stale capability fails, the surviving member's pre-change capability is *superseded*
(not overwritten — the old object is unchanged), and the re-issued one succeeds; (5) **deterministic on
both members**, order-independent (two members applying two removals in opposite orders reach the
identical folded set and identical verdicts).

**FINDING-stopped (scope wall).** The §2e row's aspiration to move the subspace-mapping half `Design →
green-real` *against real crypto* and to close the `SubspaceId` byte encoding (E.1) **exceeds Part 3's
`Design`-grade / no-wire-pinning / no-MLS-internals wall** — FINDING-stopped: the bindings stay
`Design`, and the `green-real` upgrade plus the `[gates-release]` E.1 encoding stay open. No trust tier
decided (who-may-remove is an *input* governance fact — I9).

**Conditional edits (applied — all-green):** backlog §2e row → done (RUN-11); EVIDENCE-MAP row
(§5.2/5.10 group-principal seam, `Design`); one-line evidence note on the seam brief's changelog.

---

## Part 4 — message continuity over real transport (first-to-drop)

**Dropped cleanly at the ship-without rule.** After Parts 1–3 landed, re-driving the E12.2 continuity
records over real iroh-gossip is the run's heaviest integration (real iroh async runtime + gossip
subscription + cross-crate B1 wiring, an env-sensitive `LocalDirect` loopback setup), and the §7.6.2
message-half grade would, per the A.9 when-in-doubt rule, most likely stay `Modeled` regardless
(harness-injected faults, no wire pinning). The run's own rule designates this part first-to-drop.

**Recorded as the shaped backlog row §2f** (RED-able, not built): two `IrohGossipBus` nodes at loopback
(`RelayChoice::LocalDirect`); node A publishes the B1 `Record`s (test-only serialization riding
**inside** the existing gossip `Frame` payloads, commented as such — no B1 wire pinning) and node B
drains-and-folds them into a `History`; re-assert the four continuity claims over transport delivery
(pre-repoint exactly-once; in-flight causal order; cross-order byte-identical convergence; injected
dup/drop *detected*, not absorbed — harness injects only the fault). On green, re-evaluate the §7.6.2
grade per A.9.

**Grade outcome:** §7.6.2 message half **stays `Modeled`** (unchanged) — no spec tag moved, no
EVIDENCE-MAP change this run.

---

## The parked list (restated, unchanged)

- **I9** — the identity/key-recovery + revocation-authority trust tier (the largest open problem; the
  Tier-1 *lock* landed RUN-08, the Tier-2 trust predicate is the open call).
- **X1** — real-NAT traversal (needs the boxes; not localhost-satisfiable).
- **hot-N 500+** — fan-out magnitude at hardware scale.
- **`[gates-release]` + BLAKE3** — the Appendix B wire/byte pinning (the emitter integration now rides
  this pass by decision).
- **emitter integration** — now **formally deferred by decision** (Option C, owner 2026-07-15); Option
  B remains the fallback.
- **the resolution-ACL (croft-group L3)** design frontier — the fork-projection read-scope; croft-group
  L2 (MLS) does not depend on it.

Two FINDINGs surfaced this run were then addressed in a **RUN-11 follow-on** (commit below), on request:

- **§7.6.2-membership-half FND-T4 RUN-NN gap (Part 1) — resolved.** Rather than invent a RUN pointer,
  the E12.7 keystone tests were **re-proven in-environment, 3/3 green on real openmls 0.8.1 (RUN-11
  re-proof)**; the membership-half sentence now carries `(evidence: e12_7_1/2/3_*.rs, RUN-11 re-proof,
  `Verified`)`. No status tag moved.
- **§2e `green-real`/`SubspaceId`-encoding scope-wall stop (Part 3) — partly resolved.** The
  subspace-derivation half was moved to `green-real` by reusing the `Verified` `fold_by_lineage`
  primitive on real openmls leaves (`subspace_fold_green_real.rs`). The `SubspaceId` **byte encoding**
  (`[gates-release]`, Appendix B / E.1) and the revocation-authority **trust tier** (**I9**) stay
  parked — deliberate gates, not defects.

FND-T2/T3 recorded, no edit.

---

## Files changed

**Part 1 (settle):** `alpha/experiments/CONSISTENCY-FINDINGS-2026-07.md` (settlement section),
`beta/drystone-spec/part-2-certifiable-design.md` (§5.10 reframe + three FND-T4 reshapes + the §7.6 Map
line), `beta/drystone-spec/part-2-changelog.md` (three entries), `alpha/Proofs/FROZEN-NOTICE.md`
(banner), `alpha/experiments/EMITTER-INTEGRATION-BRIEF.md` (decision annotation),
`alpha/thinking/the-shape-of-disagreement.md` (label-collision note).

**Part 1b (refresh):** `alpha/experiments/EXPERIMENT-BACKLOG.md` (execution order),
`alpha/experiments/MASTER-INDEX.md` (pressing-edges narrative), `beta/drystone-spec/open-threads.md`
(§5.10 + new §6.8.1 threads).

**Part 2 (L2a):** `alpha/experiments/croft-group/crates/group-seal/` (new: `Cargo.toml`, `Cargo.lock`,
`src/lib.rs`, `tests/l2a_sealed_frame.rs`), `alpha/experiments/croft-group/Cargo.toml` (exclude),
`beta/drystone-spec/EVIDENCE-MAP.md` (row), `alpha/experiments/CROFT-GROUP-L2-READINESS.md` (landing),
`alpha/experiments/MASTER-INDEX.md` (C1), `alpha/experiments/EXPERIMENT-BACKLOG.md` (§3 L2a row).

**Part 3 (§2e):** `alpha/experiments/group-principal-seam/` (new: `Cargo.toml`, `Cargo.lock`,
`src/lib.rs`, `tests/seam.rs`), `beta/drystone-spec/EVIDENCE-MAP.md` (row),
`beta/impl/drystone-design/group-principal-seam.md` (evidence note),
`alpha/experiments/EXPERIMENT-BACKLOG.md` (§2e row).

**Part 4 (shaped, dropped):** `alpha/experiments/EXPERIMENT-BACKLOG.md` (§2f shaped row + queue item).

**Output:** `alpha/experiments/RUN-11-SUMMARY.md` (this file);
`beta/impl/experiments/drystone-reviews-and-experiments-log.md` (RUN-11 entry).
