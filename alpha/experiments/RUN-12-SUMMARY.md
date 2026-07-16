# RUN-12 — provenance ruling, transport continuity, RBSR, Session-emit, L2b, and the horizon checkpoint

Branch `claude/run-12-multi-part-cznuc8`, off `main` after RUN-11 merged. One documentation part (1)
then five build/read parts (2–6), each shipping independently. The site gate (`site/build.py` broken-ref
+ emitted-HTML anchor audit, and the new companion-ref allowlist) ran clean (0 hard-gated) at every
commit boundary. The I9 firewall held throughout. TDD throughout: RED evidenced before GREEN per build
part. Full narrative in `beta/impl/experiments/drystone-reviews-and-experiments-log.md` (RUN-12 entry).

| Part | What | Outcome | Commit |
|---|---|---|---|
| **1** | The import-provenance ruling (A.9 rider, §7.6.2 retrofit, allowlist) | ✅ ruling applied; RUN-11 §7.6.2 FINDING closed | `48c9c6f` |
| **2** | §2f — message continuity over real transport | ✅ built; §7.6.2 message half stays `Modeled` | `076ac4c` |
| **3a** | The RBSR construction brief (read-and-report) | ✅ verdict: Negentropy-style 1-d | `c5613bd` |
| **3b** | The minimal RBSR build (gated on 3a) | ✅ landed at loopback (`Modeled`) | `bbfe3cc` |
| **4** | Session-emit completion | ✅ built; residual Closed | `09a0725` |
| **5** | croft-group L2b | ⚠️ FINDING — no discrete slice; dropped cleanly | `bbcb640` |
| **6** | EXP-H2 — the horizon checkpoint as a fact | ✅ built; §7.6.9 stays `Design` | `42d40a5` |

---

## Part 1 — the import-provenance ruling

**The A.9 sentence (the rider, added beside FND-T4).** *"Import-provenance slot (settled RUN-12,
2026-07-16): for evidence imported from outside the numbered-run system, the `RUN-NN` slot instead
carries import provenance — the source corpus and commit, `imported: <corpus> @ <commit>` — a verifiable
pointer to the exact tree where the evidence lives and passes, and a retroactive RUN number is never
invented."*

**The fourth retrofit (§7.6.2 membership half), before → after.**

- Before: `(evidence: e12_7_1/2/3_*.rs — E12.7 stamp-tracks-derivation, removal-propagates,
  unauthorized-no-drift — RUN-11 re-proof, Verified)`
- After: `(evidence: the e12_7_* tests, imported: replant-continuity @ d52ed6f, Verified)`

The RUN-11 follow-on had resolved the FND-T4 gap by re-proving in-environment and stamping `RUN-11
re-proof`; the RUN-12 ruling supersedes that with the stronger record (import provenance names the exact
tree where the evidence lives and passes). EVIDENCE-MAP row 52 carries the same. No status tag moved (was
and stays `Verified`). The RUN-11 §7.6.2 FINDING is thereby **closed**.

**The allowlist (the site gate), before → after.** The 7 companion/exploratory unresolved references
that passed as a *silent soft baseline* (reported, non-fatal, count untracked) became an explicit
`COMPANION_ALLOWLIST` in `site/build.py` — each entry `(doc_id, ref) → one-line reason`. The gate now
fails the build if the actual companion-unresolved set differs from the allowlist in **either**
direction: a NEW unlisted unresolved ref (a broken link a change introduces) or a listed entry that no
longer fires (a stale allowlist whose ref was fixed/removed). Both directions were verified to fail.
The 7 allowlisted refs are unchanged cross-references into unpublished companion docs (COHESION, ROADMAP,
doc-writing-method, social-layer) and one external MLS section.

Settlement recorded in `CONSISTENCY-FINDINGS-2026-07.md` (RUN-12 section); `part-2-changelog` entry.

---

## Part 2 — §2f: message continuity over real transport

**RED → GREEN.** New test `croft-chat/croft-chat/tests/iroh_message_continuity.rs` (2 tests): two
`IrohGossipBus` nodes at `RelayChoice::LocalDirect`; node A publishes the B1 `Record`s (test-only
serialization riding inside the gossip `Frame` payloads — no B1 wire pinning) and node B drains-and-folds
them into a `History`. The four continuity claims are re-asserted over real gossip delivery: (a) every
pre-repoint entry present exactly once; (b) in-flight entries land once in causal order; (c) both members
converge byte-identically across arrival orders; (d) an injected duplicate/dropped frame is *detected*,
not absorbed (the harness injects only the fault). **RED** was evidenced by a no-bootstrap probe: with
node B given an empty bootstrap list, no swarm forms, the records never traverse the transport, and the
convergence assertion (`b_hist.len() == 5`) fails — proving the test gates real delivery, not a
tautology. `replant-continuity`'s pure dataplane is reused behind the `iroh-it` feature, keeping the
default build openmls-free (verified: `cargo tree` shows 0 openmls in the default build). Only the
existing loopback gossip shapes (`iroh_convergence.rs`) are used — no new transport machinery, so the
stop rule was not triggered.

**Grade decision (A.9), with reasoning.** §7.6.2 message half **stays `Modeled`**. The records now ride
real transport, so the rationale drops "delivered by the harness"; but the record serialization is
test-only, so the `[gates-release]` B1 record encoding that two implementations would interoperate on is
still unpinned, and delivery is loopback with real-NAT the X1 residual. The A.9 when-in-doubt rule keeps
it `Modeled`; the rationale clause was updated to name what still gates (as RUN-11's shaping predicted).

Conditional edits: §7.6.2 body + §0 back-Map + EVIDENCE-MAP row; backlog §2f → done; `part-2-changelog`.

---

## Part 3a — the RBSR construction verdict

**Verdict (brief: `beta/impl/drystone-design/rbsr-construction.md`).** For §6.8.1's steady-state
history/message anti-entropy — which reconciles a **linear, totally-ordered `(device, lamport)` key
space** — adopt the **Negentropy-style one-dimensional recursive reconciler**. Negentropy is that
construction natively (stateless responder matching the content-blind history store; an
addition-mod-2²⁵⁶ fingerprint whose omission hardness is exactly the property the semi-trusted-store seam
wants; an `IdList`/`Fingerprint` mode switch that generalizes the RUN-09 whole-set `missing_frames` as
its degenerate single-bucket case), whereas Willow 3d-RBSR pays for a commutative fingerprint and
three-dimensional product ranges to serve a key space with a dimension count of one. Both candidates were
checked against their live published specs (Willow `willowprotocol.org/specs/rbsr/`, Negentropy
`github.com/hoytech/negentropy` + `logperiodic.com/rbsr.html`, fetched 2026-07-16). **Clarification, not
a FINDING:** the governance-fact surface (§6.8.5/§7.2) is a distinct **Willow-shaped 3d** key space and
remains a separate choice; §6.8.1 line 810 already frames the production construction as an open choice
and §7.2 commits only the governance-fact *data model* to Willow's shape, so no spec text is contradicted.

---

## Part 3b — the minimal RBSR build (landed at the gate)

**Landed** (not dropped): 3a's recommendation was unambiguous and fit the wall (experiment-grade,
loopback, no wire pinning, no new deps). `croft-chat::anti_entropy` gained `reconcile_partitioned` (a
Negentropy-style 1-d recursive reconciler — count-balanced sub-ranges, an additive-monoid
`RangeFingerprint`, an `IdList`/`Fingerprint` mode switch; dep-free `splitmix64` key mixing + inline
256-bit add), the whole-set `missing_frames` retained as its degenerate case.

**RED → GREEN.** `partitioned_anti_entropy.rs` (3 tests): (1) diff-only equivalence — a single-frame gap
ships exactly the whole-set diff and re-converges; (2) fingerprint composition — a range fingerprint
equals the monoid-combination of its sub-ranges', is commutative, and equal sets agree; (3+4+5) the scale
case — **an 8-record divergence in a 71-record set repaired in 4 rounds** (≈ log₄71, well under the
whole-set O(71) exchange), shipping only the 8 divergent records, then re-converged. **RED** evidenced by
a neutralized-fingerprint probe (all ranges falsely "agree" → the divergence is missed). Grade `Modeled`
at loopback (test-only fingerprint; `[gates-release]` wire fingerprint unpinned; real-transport = X1).

Conditional edits on green: §6.8.1 body (both residual clauses) + open-threads + two EVIDENCE-MAP rows +
backlog + `part-2-changelog`.

---

## Part 4 — Session-emit completion

**RED → GREEN.** `session_emit_governance_via_api.rs` (3 tests, two-session end-to-end per act kind).
`LocalStore` gained `grant_role`/`revoke_role` and an approvals-carrying `remove_member` (returning the
act hash); `Session` gained `propose_/approve_` pairs for `MembershipRemove`, `RoleGrant`, `RoleRevoke`,
mirroring the proven `propose_rule_change`/`approve_rule_change` shape (same R7 content-bound counting,
same co-signed-op antecedents, the approval subject = the target principal). Each test raises the relevant
threshold to 2, collects a distinct-persona approval across sessions, enacts, and asserts the fold's
consequence: removal at a 2-of-n quorum with the removed persona's later fact rejected; a granted role
authorizing an Admin-only act it could not perform as a Member; a revoked role withdrawing it — converging
identically on a second session. **RED** was the compile-gated absence of the seven new methods (16
errors before implementation). Tests: croft-chat 3/3 + suite green; `local_storage_projection` 88/0;
`clippy --all-targets` clean on both crates.

**Scope wall (held).** Fold/API level only — no MLS re-key linkage (that stays `mls-replant` / L2b+
territory), no authority-model change (the existing role gate and quorum as-is), no trust tier.

**Conditional edits.** Applied: Session-emit residual → **Closed** (`SPEC-DIVERGENCE-REGISTER`,
`SPEC-ALIGNMENT-AND-ACTION-PLAN`), EVIDENCE-MAP §7.2 R7 row, MASTER-INDEX. **Withheld** (correctly): the
§7.2 R7 residuals paragraph — it names the role model and the competing-quorum case, **not** this
client-surface gap, so per Part 4.4's "if and only if it names this gap" it was left unedited.

---

## Part 5 — croft-group L2b (FINDING, dropped cleanly)

**FINDING — no discrete L2b slice inside the wall.** The readiness brief defines no L2b: L2a is stated to
be "the whole of L2's *mechanism* half" (§4), and the entire remainder — R8-tier (recovery trust
predicate), R9 (resolution-ACL / read-scope under fork, which the brief names croft-group **L3** per
F-L2-NAME), R10 (authorized revocation over the wire) — is the authority/projection half, firewalled
behind **I9** with an explicit **OWNER-DECISION** gate on each. Part 5's scope wall (mechanism-half-only;
FINDING-stop anything touching the authority half or the resolution-ACL) leaves nothing autonomously
buildable inside the wall. Per Part 5's own drop rule, the part was **dropped cleanly (no code)**, with
the **smallest shapeable slice proposed**: R10 (authorized revocation over the wire — the authority half
of the exact re-key L2a already built the mechanism for), which nonetheless sits *outside* the wall and
needs the I9/OWNER-DECISION revocation-authority call cleared first. Recorded as a settlement FINDING +
a shaped backlog row (croft-group L-series). No tag moved; the I9 firewall held.

---

## Part 6 — EXP-H2: the horizon checkpoint as a foldable fact

**RED → GREEN.** New pure module `local_storage_projection::horizon_checkpoint` (`CheckpointFact { signer,
manifest }`, `corroboration_count`). Test `horizon_checkpoint.rs` (3 tests, cross-package via the real
`DerivedFold`, mirroring `horizon_manifest.rs`): (1) the corroboration count for a `(frontier, manifest)`
pair folds deterministically and identically across members and arrival orders (and a signer's clients
collapse to one lineage); (2) a member whose fold does NOT match records nothing toward another's
manifest — no false corroboration; (3) an open contradiction persists in the manifest across successive
checkpoints (the H1 decay-is-presentation assertion, at the fact layer). **RED** evidenced by a probe
that drops the manifest-match guard, making a non-matching member falsely corroborate (count 3 ≠ 2).

**Scope wall (held).** Experiment-grade fact shapes, test-only serialization, no wire pinning (the
manifest encoding stays `[gates-release]`); §7.3.3 semantics unchanged — a co-signature is corroboration
of an independent identical fold, never a substitute.

**Grade judgment (A.9), stated either way.** The §7.6.9 worked example **stays `Design`**. EXP-H2 earns
the fact-form corroboration-determinism claim — a `Modeled`-grade sub-result carried in its own
EVIDENCE-MAP row — but §7.6.9's composition depends on the `[gates-release]` manifest encoding (unpinned)
and the full cadence composition is not proven end-to-end, so no tag moved. EVIDENCE-MAP row + backlog
§2b′ added.

---

## The parked list (restated, verbatim)

**I9** (the identity/key-recovery trust tier, the largest open problem; the Tier-1 lock landed RUN-08,
the Tier-2 trust predicate is the open call); **X1** (real-NAT, needs the boxes); **hot-N 500+** (fan-out
magnitude at scale); **`[gates-release]` + BLAKE3** (the Appendix B wire/byte pinning); **emitter
integration** (formally deferred by decision — Option C, defer to the `[gates-release]` pass, owner
2026-07-15; Option B fallback); and the **resolution-ACL (croft-group L3)** design frontier. Unchanged
this run. RUN-12 additionally leaves open, at loopback grade, the `[gates-release]` B1 record encoding
(Part 2) and the `[gates-release]` RBSR wire fingerprint (Part 3b) — both Appendix B items on the same
pinning pass — and the R10 revocation-authority call surfaced by Part 5's FINDING (behind I9 /
OWNER-DECISION).

---

## Files changed

**Part 1 (ruling):** `beta/drystone-spec/conventions-and-decisions.md` (A.9 rider),
`beta/drystone-spec/part-2-certifiable-design.md` (§7.6.2 membership-half retrofit),
`beta/drystone-spec/EVIDENCE-MAP.md` (row 52), `site/build.py` (`COMPANION_ALLOWLIST` + gate),
`alpha/experiments/CONSISTENCY-FINDINGS-2026-07.md` (settlement), `beta/drystone-spec/part-2-changelog.md`.

**Part 2 (transport):** `alpha/experiments/croft-chat/croft-chat/tests/iroh_message_continuity.rs` (new),
`alpha/experiments/croft-chat/croft-chat/Cargo.toml` (+ `Cargo.lock`; `replant-continuity` optional under
`iroh-it`), `beta/drystone-spec/part-2-certifiable-design.md` (§7.6.2 body + §0 Map),
`beta/drystone-spec/EVIDENCE-MAP.md`, `alpha/experiments/EXPERIMENT-BACKLOG.md`,
`alpha/experiments/MASTER-INDEX.md`, `beta/drystone-spec/part-2-changelog.md`.

**Part 3a (brief):** `beta/impl/drystone-design/rbsr-construction.md` (new).

**Part 3b (build):** `alpha/experiments/croft-chat/croft-chat/src/anti_entropy.rs` (partitioned
reconciler), `alpha/experiments/croft-chat/croft-chat/tests/partitioned_anti_entropy.rs` (new),
`beta/drystone-spec/part-2-certifiable-design.md` (§6.8.1), `beta/drystone-spec/open-threads.md`,
`beta/drystone-spec/EVIDENCE-MAP.md`, `alpha/experiments/EXPERIMENT-BACKLOG.md`,
`beta/drystone-spec/part-2-changelog.md`.

**Part 4 (Session-emit):** `alpha/experiments/local_storage_projection/src/surface.rs`
(`grant_role`/`revoke_role` + `remove_member`), `alpha/experiments/croft-chat/social-graph-core/src/session.rs`
(6 propose/approve methods), `alpha/experiments/croft-chat/croft-chat/tests/session_emit_governance_via_api.rs`
(new), `alpha/experiments/SPEC-DIVERGENCE-REGISTER.md`, `alpha/experiments/SPEC-ALIGNMENT-AND-ACTION-PLAN.md`,
`alpha/experiments/MASTER-INDEX.md`, `beta/drystone-spec/EVIDENCE-MAP.md`.

**Part 5 (FINDING):** `alpha/experiments/EXPERIMENT-BACKLOG.md` (L2b FINDING row),
`alpha/experiments/CONSISTENCY-FINDINGS-2026-07.md` (settlement FINDING).

**Part 6 (EXP-H2):** `alpha/experiments/local_storage_projection/src/horizon_checkpoint.rs` (new) +
`src/lib.rs`, `alpha/experiments/croft-chat/croft-chat/tests/horizon_checkpoint.rs` (new),
`beta/drystone-spec/EVIDENCE-MAP.md`, `alpha/experiments/EXPERIMENT-BACKLOG.md`.

**Output:** `alpha/experiments/RUN-12-SUMMARY.md` (this file),
`beta/impl/experiments/drystone-reviews-and-experiments-log.md` (RUN-12 entry).
