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
SPEC-DELTA[<id> | <kind>]: <what it stands in for> — <spec requirement> — Register: alpha/experiments/SPEC-DIVERGENCE-REGISTER.md
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
| **run14-A2** | declared-stand-in | `appview-validation/src/viewserve.rs` (`recruiters` table; tag at module head) | EXP-A step 2's viewer-gate reads a `recruiters(did, affiliation)` SQLite table seeded in-test to decide who may be offered `openToWork`. The *admission* of a DID to that roster is asserted, not governed. | Recruiter-admission is a governance decision (R7 territory — who admits, under what quorum). The gate mechanism (verified-identity → field visibility) is what EXP-A earns; the admission policy is out of scope. | When recruiter-admission is designed, the roster becomes a governed projection (the §7.2 R7 membership path), replacing the seeded fixture. The *verification + gating* half needs no change. | **Bounded — intentional.** Green = the gate works against a verified identity; it does **not** claim admission governance. |
| **run14-A4** | declared-stand-in | `appview-validation/src/bin/authserve.rs` (service-DID `aud`; tag at module head) | The live credentialed leg self-issues the service-auth token with `aud` = the test account's **own DID**, because no real Stellin service DID can be provisioned in-environment. The signature-verification path is identical to a real service DID. | A production AppView has its own service DID (`did:web:…`) as the token audience; the client sets `aud` to it. | Provision a real service DID (domain purchase pending — same gate as the `app.stellin.*` lexicon namespace) and set it as `aud`. No verifier change. | **Bounded — intentional. Exercised** (owner-supplied creds, RUN-14 follow-up): a real getServiceAuth token with `aud`=own-DID verified end-to-end against the real DID-doc key. The stand-in is only the `aud` *value*, not the mechanism. |
| **run15-stub-verifier** | declared-stand-in | `appview-infra/kit/stub/stub.py` (`StubVerifier`; tag at module head) | The hosting-kit stub verifies a `test:<did>` bearer token behind the same `Verifier` interface a real atproto service-auth verifier implements. | Caller identity is a real atproto service-auth JWT verified against the issuer's DID-doc key (RUN-14 EXP-A). | Swap `StubVerifier` for the RUN-14 EXP-A verifier when the real binary lands; the interface is unchanged. | **Bounded — intentional.** The kit proves hosting/serving/gating; the verifier is the RUN-14 seam. |
| **run15-local-root** | declared-stand-in | `appview-infra/kit/tests/helpers.bash` + `stub.py` (`STUB_ALLOW_ROOT`) | The rehearsal container is root-only, so the stub's contract-§4 root refusal is overridden for tests. | The deployed service runs as a dedicated non-root user via systemd `User=`. | On the box, `User=` provides the non-root guarantee; the override is test-only. | **Bounded — intentional.** No production path runs as root. |
| **run15-tf-validate** | declared-stand-in | `appview-infra/kit/terraform/main.tf` + `scripts/terraform-check.sh` | `terraform fmt` runs; `terraform validate` is BLOCKED because the ovh provider download redirects to github.com (egress-proxy 403). `terraform-check.sh` reports BLOCKED, not skipped. | `terraform validate` confirms the provider schema of the HCL. | Run in Phase 2 where `registry.terraform.io` + github release assets are reachable, then `validate`. | **Bounded — BLOCKED, reported.** fmt-clean; schema re-confirmed in P2-2. |
| **run15-bootstrap-dryrun** | declared-stand-in | `appview-infra/kit/bootstrap/bootstrap.sh` | Only `--plan` (dry-run) idempotence + content is exercised; this is not a fresh Debian box and must not be mutated. | Real `--apply`, and a second `--apply` as a no-op, on a fresh Debian 12 VPS. | Phase 2 P2-3 runs `--apply` twice on the box. | **Bounded — intentional.** Plan-grade in discovery; apply is Phase 2. |
| **run15-sandbox-unshare** | declared-stand-in | `appview-infra/kit/tests/own_data.bats` (mount-ns ro bind) | No PID-1 systemd here, so the api's write-incapability is enforced with an equivalent `unshare -m` read-only bind (a write into the data dir is blocked). | systemd `ReadOnlyPaths=` on the api unit's data dir. | The generated api unit carries `ReadOnlyPaths` (asserted); the on-box guarantee is systemd's. | **Bounded — intentional.** Same guarantee, different enforcer. |
| **run15-usermode** | declared-stand-in | `appview-infra/kit/drill/lib.sh` (local rehearsal + fire drill) | The drill runs stubs as plain supervised processes (not systemd units) and replicates to a file:// litestream replica + a local rclone dir (not R2). The generated config is otherwise identical. | systemd-supervised units + Litestream/rclone to R2. | Phase 1.5 swaps the replica endpoints to real R2; Phase 2 swaps the substrate to systemd on the box. | **Bounded — intentional.** Real litestream/rclone/caddy/sqlite; only substrate + endpoints adapted. |
| **run15-s3-local** | declared-stand-in | `appview-infra/kit/drill/lib.sh` (`fire-drill --variant s3-local`) | The s3 drill variant exercises the real `s3://` code path (litestream `type: s3` replica + rclone s3 remote) against a **local MinIO** standing in for Cloudflare R2 — same S3 API. Ephemeral per-run creds (no static secret); the sandbox's proxy-injected AWS creds are overridden and the CA bundle cleared for the plain-HTTP localhost endpoint. | Litestream/rclone replicate to **Cloudflare R2** over the S3 API. | Phase 1.5 (P15-1/P15-2) points the same `s3://` configs at real R2 by swapping endpoint + credentials only; then records op counts vs the free tier (P15-3). | **Bounded — intentional.** Real S3 protocol end-to-end (MinIO); only the S3 backend + creds differ. Drill PASS: destroy→restore→assert green for both tenants against MinIO. |

`fanout-single-run` was the second Active row; it is **retired (Reconciled, RUN-09)** — see below. Active divergences are now `hermetic-gossip`, RUN-14's two **declared stand-ins** (`run14-A2` recruiter roster, `run14-A4` service-DID `aud`), and RUN-15's seven hosting-kit stand-ins (`run15-stub-verifier`, `run15-local-root`, `run15-tf-validate` [BLOCKED, reported], `run15-bootstrap-dryrun`, `run15-sandbox-unshare`, `run15-usermode`, and `run15-s3-local` [MinIO stands in for R2 — real S3 protocol, drill PASS]). All are sequenced by their briefs, not hidden, and none weakens a proven mechanism — each names the same guarantee's real enforcer/endpoint and the Phase-1.5/Phase-2 step that swaps it in.

## Reconciled (the spec mechanism now exists — tag retired)

| ID | Was | Reconciled to | Evidence |
|---|---|---|---|
| **x2-backfill** | prototype-mitigation: a per-tick nonce **re-flood** of the whole log defeated gossip dedup so late joiners caught up — green on a stand-in, not the design. | The spec mechanism: **sync-on-connect**. `iroh_bus.rs` now broadcasts each distinct frame **once** (`TAG_LIVE`) in steady state and, on `Event::NeighborUp`, re-broadcasts the retained log **once** as `TAG_RESYNC` (fresh ids). Cost is O(log) per join, not per tick — no re-flood. | X2 all-green across 2 runs on the resync mechanism (`A head == B head`; `84b4b1b0…`, `352b7feb…`); full iroh-it suite + clippy green. Code tag removed; ledger Phase 7 updated. |
| **rulechange-quorum** | weakened-assertion: RuleChange quorum was **not enforced** (Owner-role proxy); the substrate test verified only threshold *storage*, not enforcement. | RuleChange now has a **content-hash approval subject** (`rule_change_approval_subject`) so Step 5.6 enforces it via the same distinct-personae-by-lineage path as membership. | RED→GREEN proven (disabling the arm → 2 cases fail). Substrate `test_i6` strengthened to assert rejection; new `rulechange_threshold_enforced.rs` (4 cases). **Manual mutation gate:** `act_subject→None` killed by the reject cases; `rule_change_approval_subject→const` killed by the mismatched-approval case. Both `blake3` uses reverted clean. See note below on the formal sweep. Spec landing: §7.2 R7 (RUN-02). |
| **competing-quorum-autoresolve** | weakened-assertion: two **concurrent conflicting RuleChange quorums** on the same rule (each a valid k-of-n) **auto-resolved order-dependently** — order1 → `add_member_threshold=9`, order2 → `5`, both `fork="clean"`, no hard-stop. The refutation pin `two_competing_rulechange_quorums` asserted that weaker-than-spec behavior. | The fold now carries a **competing-RuleChange contradiction predicate** (`fold_derived::detect_competing_rulechange`, narrowest F8 form): two concurrent **admitted** RuleChanges on the **same `rule_key`** with **differing `new_value`** hard-stop, surfaced identically to mutual expulsion as `contradiction:{byte-head}` with the order-independent min-hash; the rule keeps its pre-conflict value (no verdict). Same rule_key + same value is concordant; different rule_keys never conflict. **Spec: §7.3.2 / §7.6.1 (F8); landing run: RUN-03.** | RED→GREEN: the pin `two_competing_rulechange_quorums` flipped and now asserts the hard-stop — **both fold orders identical** (`add_member_threshold=1` pre-conflict unchanged, `fork="contradiction:5680676b…"` byte-identical across orders). Two negative cases added and green: `concurrent_same_value_rulechanges_are_concordant` (fork clean, value applies) and `concurrent_disjoint_rulekey_rulechanges_do_not_conflict` (fork clean, both rules apply). Full `competing_quorums` suite (4 tests) + `local_storage_projection` and `croft-chat` suites + clippy green. |
| **automerge-0.6.1** | proxy-measurement: the 4 partial-reconstruction invariants were proven on **automerge 0.6.1**, not the 0.7 ship target (a Rust-1.75 MSRV wall blocked 0.7 in the original session). | The **0.7 ship target itself**: `automerge = "0.7"` → 0.7.4 builds and runs on Rust 1.94.1; the same four scenarios pass with the two documented API deltas applied. | RUN-01 EXP-2 (branch `claude/experiments-run-01`). All four PASS on 0.7.4 (`automerge-partial-reconstruction/run_output.txt`, `REPORT.md` top section). Only change-hash *values* differ (`cea08274…`→`e8524485…`), a serialization artifact; the behavioral invariants are identical. Moved here from "Already-declared caveats". |
| **handcrafted-assertions** | test-scaffold framed as an **API gap**: tests hand-built RuleChange / Approval / cross-device facts because `Session` could not emit them. | `Session` now emits them: `propose_rule_change` + `approve_rule_change` (over new substrate `rule_change`/`approve` commands); cross-device chains are just multiple `Session`s replicating. | `rulechange_quorum_via_api.rs` drives a full RuleChange **quorum end-to-end through the real `Session` API** across two replicating sessions (propose → approve → reference → apply → converge). **Retired in the 2026-07-13 real-substrate reconciliation landing** (reviews-log). See the residual-scaffolding note below. |
| **fold-auth-duplicate** | dead-parallel-copy: `local_storage_projection/src/fold_auth.rs` (`AuthFold`, 1685 lines) reimplemented the authorization/ingest path but was instantiated **only in its own `#[cfg(test)]`**; the live path is `surface::LocalStore` → `fold_derived::DerivedFold`. RUN-07's automated cross-package sweep found all 31 `fold_auth.rs` survivors survive the consumer suite — the copy was never linked, so no consumer test could pin it (a latent silent-drift surface for security-critical decision logic). | **Retired.** `fold_auth.rs` deleted and `pub mod fold_auth;` removed (RUN-07 follow-up, owner-authorized); the three `fold_derived.rs` comments that referenced it reworded. `fold_derived` is now the single authorization path in the crate, and it is the one the consumer suite pins (7 count-enforcement mutants killed cross-package, §7.2 R7). | `local_storage_projection` builds and tests green after removal (88 lib tests; the 9 dropped were `fold_auth`'s own unit tests of dead code); `croft-chat` suite (25 binaries) + clippy green; grep confirms no remaining reference to `fold_auth`. See `X3-AUTOMATED-SWEEP.md`. |
| **iroh-lww-language** | doc-language divergence: the iroh (Alt.Drive) corpus described **last-writer-wins by `(modified_at, node_id)` timestamp** as a DECIDED conflict-resolution design, contradicting two standing invariants (timestamps never order; concurrent contradiction hard-stops rather than auto-resolves). Not a test stand-in — a forbidden model presented as precedent to anyone grepping the corpus. | **Superseded in place:** DESIGN §7 retitled to a superseded block (body replaced, original recoverable from git history) and one-line superseded pointers added at every other mention, each pointing at §7 rather than repeating the reasoning (conventions A.11 DR-1). | E6 (human-adjudication language pass); grounds in Part 2 §7.3.1 (causal-and-cryptographic order excludes the clock) and §7.6 (reconcile hard-stop), conventions A.11. Spike 1's flat-LWW-too-weak finding (TEST-LOG B2) is the empirical corroboration. |
| **fanout-single-run** | proxy-measurement: the A4/M1 fan-out curve (EXP-1) was from **1–2 runs per N** (no averaging), so the magnitude was "indicative"; the retirement condition was replicates confirming the shape. | The **repeated-run arm** (RUN-09 Part 5): K = 5 sweeps at N = 2/4/8/16 on the same loopback harness. The **spread is narrow** and supports the recorded magnitudes — `live_sent = 2N+1` reproduced **exactly** (150/150 node-samples, zero variance), head-convergence held in **every** run at every N, and the super-linear hub-`resync_sent` shape reproduced with a **tight** band (N = 16: 349–422, median 401; the single-run 479 refined *downward*, not widened). The magnitude is now a measured band, not a single indicative point. | K = 5 repeated-run (`fanout-data/repeated-run09.csv`, `scripts/fanout-repeated.sh`), dated addendum in `FANOUT-M1.md`. §11.11 #1 caveat clause updated to record the replicated band (RUN-09). **Residual (narrow, not this row):** the resync *magnitude* is star-bootstrap-topology-sensitive; a mesh bootstrap would spread it — carried as a note, not a divergence. Hardware hot-N (500+) stays X1-adjacent. |

> **Mutation-gate note (rulechange-quorum).** The touched substrate functions were mutation-tested
> *manually and targeted* (each mutant above is killed by a named test). **RUN-01 EXP-3 (2026-07-14)**
> then ran the formal scoped `cargo-mutants` sweep across `fold_auth` + `governance` + `fold_derived`
> (120 mutants → 54 caught, 61 missed, 5 unviable): **threshold-counting has 0 survivors** in-substrate
> (`governance.rs` 13/13), and the approval-subject survivor `rule_change_approval_subject→const` was
> **hand-killed against the cross-package test** `approval_for_a_different_change_does_not_count` —
> confirming the manual gate above. See `local_storage_projection/X3-CROSS-PACKAGE-SWEEP.md`.
> **RUN-07 (2026-07-15) closed the automated harness** (`x3_cross_package_harness.py` +
> `X3-AUTOMATED-SWEEP.md`): re-running the current-code substrate sweep (61 survivors) and then
> driving each survivor through the croft-chat consumer suite resolves all 61 mechanically — **7
> killed, 54 individually justified, 0 unjustified**. The 7 killed are exactly R7's content-bound
> quorum *count* path (approval subject `rule_change_approval_subject`, approval-subject resolution
> `act_subject`, rule-key decode, membership-admin gate), so **R7's count claim is now cross-package
> mutation-`Verified`** (§7.2, RUN-07). The harness also refuted the original "all survivors are
> cross-package-covered" reading: **31 survivors are in `fold_auth.rs`, an off-consumer-path
> duplicate** the suite never links (`surface` → `fold_derived`); the rest are the role-authorship
> gate (7; R7 excludes the role model), uncovered Vouch payload validation (10), node-card provenance
> (3), and boundary/adjacent-rule mutants (3). See the new `fold-auth-duplicate` row.

> **Residual-scaffolding note (handcrafted-assertions).** The `tests/common` scaffolding is **not**
> removed and its remaining use is **legitimate, not a divergence**: (a) the Battery-5 refutation
> tests deliberately build *adversarial* delivery — a dropped / duplicated / reordered frame, a fact
> whose antecedent is withheld — which a well-behaved emit API must **refuse** to produce, so it can
> only be hand-built; (b) the fold-level threshold tests (`rulechange_threshold_enforced` etc.) ingest
> directly for focus, not from necessity. What is closed is the **capability gap**: well-formed
> governance facts are now producible through `Session`. ~~Not yet emittable via `Session` (a
> lesser, separate gap): `MembershipRemove`, `RoleGrant`/`RoleRevoke`~~ **CLOSED (RUN-12 Part 4):**
> `Session` now emits all three — `propose_remove_member`/`approve_remove_member`,
> `propose_role_grant`/`approve_role_grant`, `propose_role_revoke`/`approve_role_revoke` — over new
> `LocalStore` `grant_role`/`revoke_role` commands and an approvals-carrying `remove_member`,
> mirroring the proven `propose_rule_change`/`approve_rule_change` shape (same R7 content-bound
> counting, same co-signed-op antecedents, subject = the target principal). Driven end-to-end across
> two sessions in `croft-chat/croft-chat/tests/session_emit_governance_via_api.rs` (3 tests, green):
> removal at a 2-of-n quorum with the removed persona's later fact rejected; a granted role
> authorizing an act the persona could not perform as a Member; a revoked role withdrawing it. No
> authority-model change (the existing role gate and quorum as-is), no MLS re-key linkage (that stays
> `mls-replant`/L2b+ territory), no trust tier.

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
