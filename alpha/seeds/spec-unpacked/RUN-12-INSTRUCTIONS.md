# RUN-12 — Provenance ruling, transport continuity, RBSR, Session-emit completion, L2b, and the horizon checkpoint

`Branch: fresh off main (RUN-11 merged), e.g. run-12-transport-and-surfaces. Part 1 markdown;
Parts 2–6 code, each shipping independently. Drop order if the run runs long: 3b first, then 6,
then 5 — Parts 1, 2, and 4 are core. Site gate (site/build.py: broken-ref + anchor audit) green at
every commit boundary. The I9 firewall and the parked list are unchanged and hold run-wide. TDD
throughout: RED first, red → green evidenced per part.`

## Part 1 — the import-provenance ruling (owner, 2026-07-16)

1. **A.9 rider.** One sentence in `conventions-and-decisions.md` A.9, beside the evidence-form
   note: for evidence imported from outside the numbered-run system, the RUN-NN slot of the
   standard parenthetical carries **import provenance** instead — the source corpus and commit,
   `imported: <corpus> @ <commit>` — a verifiable pointer to the exact tree where the evidence
   lives and passes; a retroactive RUN number is never invented.
2. **The fourth retrofit.** §7.6.2's membership-half evidence prose reshaped to the standard form
   using the new slot: `(evidence: the e12_7_* tests, imported: replant-continuity @ d52ed6f,
   `Verified`)` — wording adapted to the sentence, no information added or dropped.
3. **EVIDENCE-MAP row 52** updated to carry the same import provenance in its evidence cell.
4. **Site allowlist (small).** The 7 companion/exploratory unresolved references currently pass as
   a soft baseline; convert them to an explicit allowlist in the site build config (each entry:
   file, ref, one-line reason), so the count going stale in either direction fails the build —
   companion drift becomes catchable too.
5. Settlement section records the ruling and closes the RUN-11 FINDING; `part-2-changelog.md`
   entry; reviews-log RUN-12 entry at end of run covering all parts.

## Part 2 — §2f: message continuity over real transport (execute as shaped)

Execute backlog §2f exactly as RUN-11 shaped it. In brief (defer to the row): two `IrohGossipBus`
nodes at loopback (`RelayChoice::LocalDirect`); node A publishes the B1 `Record`s with test-only
serialization riding inside the existing gossip `Frame` payloads (commented as such — no B1 wire
pinning); node B drains-and-folds into a `History`; the four continuity assertions re-asserted
over transport delivery (pre-repoint exactly-once; in-flight causal order; cross-order
byte-identical convergence; injected dup/drop detected, not absorbed — the harness injects only
the fault). RED first; red → green evidenced.

On all-green: re-evaluate the §7.6.2 message-half grade per A.9 — upgrade only if the ladder's
rung genuinely covers real-transport-at-loopback; when in doubt it stays `Modeled` with the
rationale clause updated to drop "delivered by the harness" and name what still gates (the
`[gates-release]` record encoding; real-NAT = X1). The summary states which way and why. Backlog
§2f → done; EVIDENCE-MAP row; changelog/MASTER-INDEX.

Stop rules: no MLS-internals changes; no wire pinning; if the iroh async/runtime integration
exceeds the loopback shapes the existing convergence tests already use, FINDING and stop rather
than inventing new transport machinery.

## Part 3 — the range-partitioned RBSR construction (read, then maybe build)

### 3a — the brief (always ships)

Read §6.8.1 as landed (the RUN-09 steady-state slice and its "range-partitioned production form
open" residual), the M2 test shapes, and the two candidate constructions from their primary
sources: Willow's 3d-range-based set reconciliation and Negentropy's range-based protocol (fetch
current published specs; cite versions/dates). Report to
`beta/impl/drystone-design/rbsr-construction.md` (with a `Serves:` header): what §6.8.1's
steady-state repair actually requires at Drystone's scale and privacy posture; how each candidate
meets or misses it (fingerprint shape, partition strategy, round complexity, payload privacy
against the §5.11 read-scoping); the recommended construction with costs; and the RED-able
assertion set a build would land. Anything contradicting current spec text is a FINDING, quoted
both ways.

### 3b — the minimal build (first-to-drop; gated on 3a)

Only if 3a's recommendation is unambiguous and fits the wall (experiment-grade, loopback, no wire
pinning, no new deps beyond what the recommendation strictly needs): land 3a's RED-able assertions
against the recommended construction, replacing the RUN-09 diff-only `missing_frames` exchange
with the partitioned form under the same M2 test shapes, plus one scale-shaped case (a large
divergent range repaired in O(log)-ish rounds rather than a full list exchange — the assertion is
round-count, not wall-clock). Conditional edit on green: the §6.8.1 residual clause updates to
name the partitioned form landed at loopback grade (tag per A.9, when-in-doubt-`Modeled`). If 3a
is ambiguous or the wall is hit: ship the brief, shape the build as a backlog row, drop 3b
cleanly.

## Part 4 — Session-emit completion (the RUN-02-era client residual)

The fold has understood `MembershipRemove` and `RoleGrant`/`RoleRevoke` since the original spikes,
but no client can issue them: `Session` still exposes only the rule-change surface. This closes
that residual by extending `Session` (social-graph-core) with propose/approve/enact for removals
and role changes, mirroring the proven rule-change shape — the same R7 content-bound counting, the
same co-signed-op antecedent pattern, no new authority machinery.

1. Read first: the backlog row for the Session-emit residual; the existing
   `propose_rule_change`/`approve_rule_change` surface and `rulechange_quorum_via_api.rs` (the
   model end to end).
2. RED: a two-session end-to-end test per act kind — proposal → approvals across sessions →
   enacting act → the fold enforces it (the removed persona's later facts are rejected per the
   existing fold semantics; the granted/revoked role changes what the fold authorizes), identical
   across arrival orders.
3. Scope wall: fold/API level only. The ceiling → MLS re-key linkage is `mls-replant`'s proven
   territory and is NOT wired here (that composition is L2b+/app territory); no authority-model
   changes (the existing role gate and quorum as-is); no trust tier.
4. Conditional edits on green: backlog residual row → done (RUN-12); EVIDENCE-MAP row; the R7
   residuals paragraph's client-surface clause updated if (and only if) it names this gap;
   changelog/MASTER-INDEX.

## Part 5 — croft-group L2b (execute as shaped in the readiness brief)

Execute the next croft-group slice exactly as `CROFT-GROUP-L2-READINESS.md` shapes it beyond L2a
(defer entirely to the brief's own L2b definition and assertion set; do not invent scope). Same
rules as L2a: reuse is a condition of considered compatibility (built on `group-seal`,
`lineage-mls`, and the proven crates); RED first; the L2a scope wall verbatim (mechanism half
only; FINDING-stop anything touching the authority half or the resolution-ACL, land the rest);
grade per A.9 with when-in-doubt-`Modeled` and the rationale in the sentence; conditional edits on
green mirror L2a's (backlog, EVIDENCE-MAP, readiness-brief landing note, MASTER-INDEX). If the
brief defines no discrete L2b slice inside the wall: FINDING with the smallest shapeable slice
proposed, and drop the part cleanly.

## Part 6 — EXP-H2: the horizon checkpoint as a foldable fact

EXP-H1 (RUN-07) proved the manifest computes identically; the checkpoint itself is still only a
pure function. This lands the fact form.

1. RED: a member records a horizon-checkpoint **fact** carrying (frontier digest, manifest) at a
   cadence boundary; a second member, computing the identical pair, records a co-signing fact
   naming the same digests. Assertions: the corroboration count for a given (frontier, manifest)
   pair folds deterministically and identically across members and arrival orders; a member whose
   fold does NOT match records nothing (no false corroboration); an open contradiction persists in
   the manifest across successive checkpoints (the H1 decay-is-presentation assertion, now at the
   fact layer).
2. Scope wall: experiment-grade fact shapes, test-only serialization, no wire pinning (the
   manifest encoding stays `[gates-release]`), §7.3.3's semantics unchanged (a co-signature is
   corroboration of an independent identical fold, never a substitute).
3. Conditional edit on green: no tag move unless A.9 clearly supports one — the §7.6.9 worked
   example stays `Design` unless the evidence genuinely earns `Modeled`, and the summary states
   the judgment either way; EVIDENCE-MAP row; backlog row → done.

## Output

`alpha/experiments/RUN-12-SUMMARY.md`: Part 1 before → after for the retrofit, the A.9 sentence,
and the allowlist; Part 2 red → green evidence and the grade decision with reasoning; Part 3a
verdict paragraph with the brief pointer; Part 3b landed / dropped-at-gate and why; Parts 4–6
red → green evidence, assertions landed vs FINDING-stopped or dropped per the stated order;
conditional edits applied or withheld; the parked list restated; files changed.
