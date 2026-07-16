# Drystone Part 2 — the evidence map (spec ↔ experiment traceability index)

`Status: living index, first built RUN-08 (2026-07-15). One row per Part 2 status-tagged claim at
or above` Modeled `on the A.9 ladder, plus the load-bearing` Load-bearing, unearned `beam. Built by
the RUN-08 traceability pass (Part 2) from links verified in that pass.`

## What this is, and what it is NOT

- **This map is an index, never a source of truth.** The authoritative statement of any claim's
  status is the **tagged sentence in Part 2** (`part-2-certifiable-design.md`). If this map and a
  Part 2 sentence disagree, the sentence wins and this row is the bug. The map never *sets* a
  status; it only points at where the evidence for a Part 2 tag lives.
- Tags are the A.9 ladder (`conventions-and-decisions.md` §A.9): `Verified`, `Verified-RFC`,
  `Modeled`, `Measured`, `Established`, `Design`, `Synthesis`, `Load-bearing, unearned`,
  `[gates-release]`, `[confirm]`. Rows below `Modeled` (pure `Design`/`Synthesis`) are out of scope
  for this map except the one load-bearing beam, which is included because it is the program's
  spine.
- **Unresolved links carry a FINDING id** (`FND-T*` in `alpha/experiments/CONSISTENCY-FINDINGS-2026-07.md`,
  RUN-08 section), never an invented pointer. A blank evidence cell with a FINDING id means "the
  Part 2 tag stands, but this pass could not resolve a named test/report/RUN for it."

## Regeneration recipe (the 2.0 scan)

1. `grep -nE '\`(Verified|Verified-RFC|Modeled|Measured|Established|Load-bearing)' part-2-certifiable-design.md`
   for the tagged sentences; A.9 for the ladder; A.11 for the adjudication vocabulary.
2. Cross-reference each tag's evidence against: the `RUN-0N-SUMMARY.md` files, the per-experiment
   reports (each now opens with a `Serves:` header, RUN-08 §2.2a), the spec-earning test
   doc-comments (each names its §, RUN-08 §2.2b), `SPEC-DIVERGENCE-REGISTER.md`, and
   `EXPERIMENT-BACKLOG.md`.
3. A row is *fully linked* when its tag ≥ `Modeled` resolves to a named test/report + RUN with its
   environment bound stated. Otherwise it carries a `FND-T*` id.

## Columns

`section | claim (short clause) | tag | bounds | evidence (tests / reports / RUN) | register | gates`

---

## A. Governance-fold claims earned by the experiment program (the RUN-driven core)

| § | claim | tag | bounds | evidence | register | gates |
|---|---|---|---|---|---|---|
| 7.2 R7 | content-bound-quorum **count** enforced (approval binds to the change hash) | `Verified` | cross-package mutation-clean; count only (role model open) | `rulechange_threshold_enforced.rs`, `rulechange_quorum_via_api.rs`; `X3-AUTOMATED-SWEEP.md` (RUN-07) | `rulechange-quorum` (Reconciled), `fold-auth-duplicate` (Reconciled), mutation-gate note | — |
| 7.2 | R7 residual: two competing quorum-met RuleChanges → `contradiction:{byte-head}` | `Modeled` | fold-side; loopback | `two_competing_rulechange_quorums` (RUN-03) | `competing-quorum-autoresolve` (Reconciled) | — |
| 7.2 R7 | `MembershipRemove` / `RoleGrant` / `RoleRevoke` driven end-to-end through the real `Session` emit API (propose→approve across sessions→enact at k-of-n; same content-bound counting and co-signed-op antecedents as RuleChange; subject = target principal) | `Verified` | two live sessions, loopback replication; fold/API only — no MLS re-key linkage, no authority-model change, no trust tier | `session_emit_governance_via_api.rs` (RUN-12 Part 4, 3 tests: remove-at-quorum + removed persona's later fact rejected; role-grant authorizes an act; role-revoke withdraws it) | Session-emit residual (SPEC-DIVERGENCE-REGISTER, Closed) | — |
| 7.3.2 | effective-roles projection order-independence | `Modeled` | fold-side | convergence-v2 reference model (reviews-log) | — | — |
| 7.3.2 | absent-target no-op / absent-predecessor gap (R3 no-fold-time-rejection) | `Modeled` | fold-side | `completeness.rs`, `heal.rs` (referenced-gap detection) | — | — |
| 7.3.2 | competing rule changes hard-stop | `Modeled` | fold-side; loopback | `two_competing_rulechange_quorums` (RUN-03) | `competing-quorum-autoresolve` (Reconciled) | — |
| 7.5.2 | subgraph-closure (attributable acceptance) | `Verified` | vs Matrix State-Res v2.1 guide | §7.5.2 cite | — | — |
| 7.6 | contradiction hard-stop; identical reformed genesis across independent nodes | `Verified` | loopback (in-process nodes) | `contradiction.rs`, `mutual_expulsion.rs`, `removed_then_included.rs`, `role_thrash.rs` | — | — |
| 7.6.1 | contradicted-group byte-head naming is `min(H(F),H(G))`, order-independent | `Modeled`→run | fold-side | `competing_quorums.rs::contradicted_group_byte_head_is_min_hash_order_independent` (RUN-01 EXP-4) | — | — |
| 7.6.1 | Under-determination escalation shape hard-stops | (§7.6.1) | fold-side | `under_determination.rs` | — | — |
| 7.6.2 | re-plant **membership** continuity: MLS group stamps exactly the fold-derived set | `Verified` | membership half; real openmls 0.8.1 | `e12_7_1/2/3_*.rs` (E12.7; **imported: replant-continuity @ `d52ed6f`** — import provenance carries the evidence in place of a discovery-RUN stamp per the RUN-12 A.9 ruling, closing the FND-T4 §7.6.2 gap); `mls-replant` e12_* | `e12.4-byteproxy` | — |
| 7.6.2 | re-plant **message** continuity: in-flight conversation survives the repoint, no loss/dup | `Modeled` | loopback / B1 hash-structure grade; harness delivery, no real transport, no wire pinning | `e12_2_message_continuity.rs` (RUN-09, 5 tests: pre-repoint exactly-once, in-flight causal-order, cross-order digest equality, dup/drop detection) | — | `[gates-release]` B1 record encoding (App-B) |
| 7.6.9 | horizon manifest determinism (frontier head + sorted open byte-heads) | `Design` (spec); manifest determinism earned | fold-side; test-only serialization, NOT the release encoding | `horizon_manifest.rs` (4 tests, RUN-07 EXP-H1) | — | `[gates-release]` §7.6.9 manifest encoding (App-B) |
| 7.3.3 / 7.4.1 | completeness-ahead contract (stall-at-threshold, stamp gap, solicitation reach, formula-k) | `Design` (spec); contract demonstrated | loopback / fold grade | `completeness_ahead.rs` (4 tests, RUN-07 EXP-C1) | — | `[gates-release]` stamp + (G,D) cursor encodings |
| 8.2(e) | policy-change enforcement + origination freshness precondition | `Verified` (count) / exercised (freshness) | count cross-package mutation-clean; freshness at **loopback** grade, real-NAT = X1 | §7.2 R7 evidence; `completeness_ahead.rs` (EXP-C1, RUN-07) | `hermetic-gossip` | — |

## B. Delivery / transport / convergence claims (substrate-verified)

| § | claim | tag | bounds | evidence | register | gates |
|---|---|---|---|---|---|---|
| 4.1–4.6 | tagged wire derivations; forged-message reject; two-devices-fold-to-one | `Verified` | against real Ed25519 / live iroh; conformance cats 1–3 | conformance-core `run-vectors` cats 1–4 (66/0); `convergence.rs`, `iroh_convergence.rs` | — | — |
| 6.x | iroh transport: EndpointId=pubkey, blind relay, direct-first, dedup, gap-fill | `Verified` / `Verified-RFC` | against iroh 1.0 / RFC 9420, RFC 9750; **loopback** for the live-gossip rows | `iroh_convergence.rs` (loopback); relay-lab-runs; iroh TEST-LOG | `hermetic-gossip`, `sharddir-standin` | — |
| 6.6.4 | content-addressed dedup is idempotent under duplication/reorder | `Verified` | loopback | `dedup.rs` (G3) | — | — |
| 6.8.1 | steady-state anti-entropy: a frame lost between connected peers is detected + repaired without reconnect, folds re-converge | `Modeled` | loopback; whole-set `(device,lamport)` range-compare stand-in; range-partitioned production construction open | `steady_state_anti_entropy.rs` (RUN-09); `anti_entropy.rs` (range-summary/diff) | — | — |
| 7.7 / 7.9 | late-joiner partial-reconstruction inertness | `Verified` | automerge 0.7.4 ship target | `automerge-partial-reconstruction/REPORT.md` (RUN-01 EXP-2), `run_output.txt` | `automerge-0.6.1` (Reconciled) | — |
| 11.11 #1 / 11.4–11.5 | per-commit **and** fan-out cost scales on the live set | `Measured` | per-node linear `2N+1`, O(N²) aggregate; **loopback**; magnitude replicated K=5 (RUN-09), open at hardware hot-N | `m1_per_commit_cost.rs`, `m1_populated_tree.rs`; `FANOUT-M1.md` (RUN-01 EXP-1 + RUN-09 addendum) | `fanout-single-run` (Reconciled, RUN-09) | — |
| 9 / 10.5 | conformance suite built + re-proven | `Verified` (suite) | 66/0; cats 8/9 TS-authoritative (`Modeled`-grade); over-the-wire authority distribution open | conformance `run-vectors` 66/0 (RUN-08 reprove, `relay-lab-runs/C-mls-welcome-2026-07-15-run08/conformance-suite-reprove.txt`) | — | — |
| 10.5 (a) | MLS key-distribution over the wire (verifying-key/standing registry sourced from real MLS) | `Verified` | **loopback** grade; not yet wired into the conformance emitter; real-NAT = X1 | `mls-welcome-over-iroh` (RUN-08, `relay-lab-runs/C-mls-welcome-2026-07-15-run08`) | — | — |
| 10.2 / 10.5(a) / 7.6.2 (croft-group **L2a**) | MLS-sealed happy-path frame: seal≠plaintext round-trip, Welcome key distribution (`epoch_secret` match), governed removal re-keys the departed reader out (PCS), Zeroize/no-`Debug` secret newtypes, pure core stays crypto-free, firewall guard | `Verified` | **loopback** grade — real openmls 0.8.1 crypto; in-process harness delivery (no real transport, no wire pinning); real-NAT = X1; **mechanism half only** (R1–R7) | `croft-group/crates/group-seal/tests/l2a_sealed_frame.rs` (RUN-11, 6 assertions); reuses `lineage-mls`/`mls-welcome-over-iroh` (§10.5(a)) + `replant-continuity` (§7.6.2) | — | R8-tier/R9/R10 firewalled (I9, parked resolution-ACL / croft-group L3) |
| 10.5 (b) | threshold-revoke as real k-of-n over the wire + co-sign-vs-vote ordering | `Design` (gated) | firewall — the revocation-authority trust model (I9) | — (MD-G5 sha-256 MAC stand-in) | — | I9 revocation-authority decision |
| 5.2 / 5.10 (group-principal seam, §2e) | Group principal = communal namespace `H(tag ‖ group_id)`; personae = self-authorizing subspaces; capability issuance downstream of the folded Group Role, **re-issued not revoked-in-place** across a fold-driven authority change (stale cap fails, re-issued succeeds, deterministic on both members) | `Design` | model — no trust tier (I9), test-only serialization; the `SubspaceId` byte encoding stays `[gates-release]` (E.1) | `group-principal-seam/tests/seam.rs` (RUN-11 Part 3, 5 assertions) | — | `[gates-release]` `SubspaceId`/genesis-id encoding (App-B, E.1) |
| 5.2 / 5.10 (subspace-derivation half, E.1) | a persona's several real MLS device leaves fold to one subspace identity (the client→subspace fold) | `Verified` | **loopback** grade — real openmls 0.8.1 leaf credentials, via the `Verified` `fold_by_lineage`; the `SubspaceId` byte encoding stays `[gates-release]` | `group-principal-seam/tests/subspace_fold_green_real.rs` (RUN-11 follow-on); reuses `lineage-mls` `fold_by_lineage` (RUN-08 `fold_matches`) | — | `[gates-release]` `SubspaceId` byte encoding (E.1); revocation-authority trust tier = I9 |

## C. The load-bearing beam and the Tier-1 lock

| § | claim | tag | bounds | evidence | register | gates |
|---|---|---|---|---|---|---|
| App-B / 2649 | completeness-ahead beam (system-level order-independence ahead of the frontier) | `Load-bearing, unearned` | the intrinsic isolated-node limit is not eliminable; the *contract* is demonstrated (EXP-C1) | `completeness_ahead.rs` (RUN-07, loopback); convergence-v2 (behind-checkpoint half) | — | the beam contract |
| 7.3.9 | recovery-anchor **Tier-1 lock** mechanism (BIP39 round-trip + secretbox-wrap) | `Verified` (experiment-grade) | in-process; crate choice not `[gates-release]`; the **trust tier is I9** | `bip39-recovery-roundtrip` (RUN-08, 11 tests) | — | Tier-2 trust predicate (I9, firewall) |

## D. Substrate `Verified` / `Verified-RFC` rows without a named test+RUN pointer

The §4–§6, §7.3, §7.4.2, §7.6.3–§7.6.4, §8.1, §10.2–§10.4 `Verified` / `Verified-RFC` tags (≈40)
resolve to a **primary-source reference** (an RFC/draft/spec section, or "against iroh 1.0",
"measured") rather than a named experiment test + RUN. Per A.9, `Verified-RFC` *is* a
literature-anchored rung and needs no experiment pointer; the substrate `Verified` rows are
anchored in the feasibility review and the conformance-core (cats 1–6). This whole band is tracked
as **FND-T1** (RUN-08): none uses the standardized `(evidence: …, RUN-NN[, grade])` parenthetical,
and adding a test/RUN pointer where the claim rests on an RFC would be an invented link. The
parenthetical is adopted here in the map's columns (section B) rather than retro-fitted into every
spec sentence.

**Settled RUN-09 (2026-07-15).** FND-T1's forward-link target is now defined per band in A.9:
literature-anchored / `Verified-RFC` rows resolve to their primary source (their correct evidence);
the experiment-earned substrate rows already carry a test+RUN pointer in section B. The band re-audit
found no experiment-earned tag still lacking a pointer, so it closes as satisfied. FND-T4's
parenthetical is adopted as the recommended forward form (A.9), not retrofitted; FND-T6's legacy
vocabulary is fixed as alpha-tier only (A.9). See `CONSISTENCY-FINDINGS-2026-07.md`,
`## Settlement (RUN-09, 2026-07-15)`.

---

## Open FINDINGs referenced by this map

- **FND-T1** — the substrate `Verified`/`Verified-RFC` band carries no standardized evidence
  parenthetical and no test/RUN pointer (resolves to RFC/substrate reference). **Settled RUN-09**:
  forward-link target defined per band (A.9); band re-audit closes as satisfied.
- **FND-T2** — the §10.5 footnote's "cats 7/8/9 not yet emitted" was superseded by the folded
  conformance-core (66/0); reconciled by RUN-08 Part 1B, residual meaning-ambiguity recorded.
- **FND-T3** — a few spec-earning test §-refs (convergence P7, iroh_convergence P18, regress_free
  V3′) were mapped at the section level from the corpus rather than a prior explicit mapping.
- **FND-T4** — the standardized `(evidence: <test/report>, RUN-NN[, grade])` parenthetical (2.1d)
  exists nowhere in Part 2; proposed for the reconciled governance claims where all components exist.
  **Settled RUN-09**: adopted as the recommended forward form (A.9), not retrofitted.

See `alpha/experiments/CONSISTENCY-FINDINGS-2026-07.md`, `## Traceability findings (RUN-08)`.
