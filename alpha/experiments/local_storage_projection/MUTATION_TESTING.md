# Mutation Testing

Mutation testing measures whether the test suite actually *pins* behavior, not
just whether it passes. A tool ([cargo-mutants]) makes one small, semantically
meaningful edit to the source — a "mutant" — and reruns the tests. If a test
fails, the mutant is **caught**. If every test still passes, the mutant
**survived**, which means that exact line's behavior is unverified: you could
break it in production and no test would notice.

This is the discipline behind the project bar of "vetted with lots of tests and
mutations." A green suite proves the code does what the tests check; a high
mutation score proves the tests check what the code does.

Scope: the authorization/governance core — `src/fold_auth.rs` and
`src/governance.rs`. These are where an undetected behavior change is most
costly (a silently-weakened threshold or role check), so they are mutated first.

[cargo-mutants]: https://mutants.rs/

---

## What gets mutated

cargo-mutants generates one mutant per applicable site. For our two files the
269 mutants fall into five operator families (counts from `cargo mutants
--list`):

```
 Relational operator replacement (ROR) ────────────────  ~67
   < → >,  < → ==,  < → <=,  == → !=,  <= → >
   Targets every guard and boundary condition.

 Arithmetic operator replacement (AOR) ────────────────  ~110
   + → -,  + → *,  * → /,  * → +,  += → -=,  += → *=
   Targets offset math, thresholds, counters, index loops.

 Return-value replacement ─────────────────────────────  ~25
   role_to_u8 → 0,  role_ge_owner → true/false,
   read_all_assertion_bytes → Ok(vec![]),  ...
   Proves callers depend on the real return value.

 Match-guard replacement ──────────────────────────────  ~8
   match guard role_ge_admin(r) → true / → false
   Directly targets the authorization predicates.

 Statement / arm deletion ─────────────────────────────  ~12
   delete match arm,  delete !  (negation)
   Exhaustiveness and inverted-condition coverage.
```

The highest-value families for this layer are **match-guard** and
**return-value** mutations on the role/threshold predicates (`role_ge_admin`,
`role_ge_owner`, `required_threshold_for_rule_change`): a survivor there means an
authorization decision is not pinned by any test. The ROR/AOR families on the
byte decoders (`*::from_bytes`) are where boundary survivors cluster.

## Equivalent mutants

Some mutants produce a program that behaves *identically* to the original — for
example `+` → `*` where an operand is always `0`, or `<` → `<=` at a bound that
is never reached. These are **un-killable by definition**: no test can
distinguish them because there is no observable difference. They must be
recorded as `equivalent` in the ledger with the reason, not chased. The
meaningful mutation score is computed *after* equivalents are excluded:

```
                 caught
   score = ───────────────────────
            caught + survived(real)      (survived(real) excludes equivalents)
```

`unviable` mutants (the edit does not compile) are not counted either way.

---

## Configuration and the speed tradeoff

Each mutant run rebuilds and reruns the test suite. The multi-device property
tests in `tests_stage7` (`prop_diverse_*`, `prop_i2/i3/i4/i9`) dominate wall
time at ~130s per run and primarily exercise `fold_derived`, not the
`fold_auth`/`governance` code under mutation. Two settings cut each run from
~130s to ~24s without weakening coverage of the mutated code:

- **`.cargo/mutants.toml`** skips those heavy property tests via
  `additional_cargo_test_args` (the leading `--` hands the args to the libtest
  harness). It keeps the deterministic unit tests and `prop_valid_sequences`,
  which are what guard the auth/governance logic.
- **`PROPTEST_CASES=8`** in the environment lowers the proptest sample count for
  the property tests that remain.

Tradeoff recorded honestly: a lower `PROPTEST_CASES` and a smaller test set
means a *real* survivor under this config is a genuine gap, but a mutant marked
`caught` was caught by the reduced suite — the full suite (CASES=256, all props)
is at least as strong. We do not under-report survivors; we may slightly
under-report the strength of catches. Survivors are always the actionable
output, so this bias is in the safe direction.

## Reproducing the run

Local (laptop), single file, for a quick check:

```sh
cd alpha/local_storage_projection
PROPTEST_CASES=8 cargo mutants --file src/fold_auth.rs --timeout 120
```

Full sweep on a remote box (used so it can run ~1h without tying up the laptop;
see the SSH driving notes in session memory for the detached-subshell pattern):

```sh
# one-time: toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal
source "$HOME/.cargo/env"
cargo install cargo-mutants --locked

# sync the crate (exclude target/ and mutants.out/)
rsync -az --delete --exclude target/ --exclude mutants.out/ \
  local_storage_projection <host>:~/croft-mutants/

# run detached, log to a file, poll the log
cd ~/croft-mutants/local_storage_projection
PROPTEST_CASES=8 cargo mutants -j 2 --timeout 120 \
  --file src/fold_auth.rs --file src/governance.rs
```

Artifacts land in `mutants.out/`: `outcomes.json` (machine-readable),
`missed.txt`, `caught.txt`, `unviable.txt`, and per-mutant build/test logs.

## Provenance

`mutants.out/missed.txt` and a generated summary (counts per file/function plus
the score) are committed alongside this doc as the raw record. The multi-MB
per-mutant build logs are not committed. Each result here is reproducible from
the config above against the committed source.

---

## Killing discipline

Every survivor we choose to kill follows the same loop, and the kill is recorded
only after the last step:

1. Write a test that asserts the behavior the mutant breaks.
2. Hand-apply the mutation to the source, run the test, and **watch it fail** —
   this proves the test actually targets the mutant, not something adjacent.
3. Revert the mutation; confirm the test passes against correct code.
4. Record the survivor → test mapping in the ledger below, and name the exact
   mutant in the test's doc comment.

A survivor that is `equivalent` skips steps 1-3 and is recorded with the reason
it cannot be observed. A survivor left as `accepted-risk` carries a dated
rationale.

---

## Survivor ledger

Status legend: `killed` (test added, kill verified per the loop above) ·
`equivalent` (un-killable, behaves identically) · `accepted-risk` (left open,
dated reason) · `open` (triaged, not yet resolved).

### fold_auth.rs

| Mutant (line: edit) | Function | Verdict | Killing test / rationale |
|---|---|---|---|
| `153: < → ==` | `GroupState::from_bytes` | killed | `group_state_below_min_len_is_rejected` (+`zero_member`) — verified: mutant panics slicing `b[17..21]` on a 20-byte buffer |
| `153: < → <=` | `GroupState::from_bytes` | killed | `group_state_zero_member_roundtrip_is_exactly_min_len` — verified: mutant rejects a valid 21-byte zero-member state |
| `165: * → /` | `GroupState::from_bytes` | killed | `group_state_truncated_members_is_rejected` — verified: mutant computes `required=21`, panics reading member 0 |
| `166: < → >` (and `<=`/`==` variants) | `GroupState::from_bytes` | killed | `group_state_truncated_members_is_rejected` — truncated buffer must error, not proceed |
| `174: * → /` | `GroupState::from_bytes` | killed | `group_state_multi_member_roundtrip_preserves_members` — wrong member offset reads the wrong principal |

### governance.rs

| Mutant (line: edit) | Function | Verdict | Killing test / rationale |
|---|---|---|---|
| all sweep-scoped mutants | `threshold_met`, `count_personae_by_lineage`, `required_threshold_for_rule_change`, `is_under_determined`, `is_ancestor`/`are_concurrent` | **caught (0 survivors)** | RUN-01 EXP-3 scoped sweep: `governance.rs` = **13 caught, 0 missed** (2 unviable: `tiebreak`/`detect_fork` `Default::default()` don't compile). The threshold-counting & quorum arithmetic is mutation-clean by the substrate's own suite. See `X3-CROSS-PACKAGE-SWEEP.md`. |

> The `GroupState::from_bytes` rows above were found by the first sweep and
> resolved in commit `b0105a0` (the four `group_state_*` tests). **RUN-01 EXP-3
> (2026-07-14)** ran the scoped sweep across `fold_auth.rs` + `governance.rs` +
> `fold_derived.rs`: **120 mutants → 54 caught, 61 missed, 5 unviable**. The 61
> survivors are **all** in the authorization *decision* path (`check_authorization`
> guards, `role_ge_*`, `act_subject`/`rule_change_approval_subject`), **none** in
> threshold counting — and they survive only because the positive-path coverage
> lives cross-package in `croft-chat` (demonstrated: `rule_change_approval_subject→const`
> is killed by `croft-chat`'s `approval_for_a_different_change_does_not_count`). See
> `X3-CROSS-PACKAGE-SWEEP.md` for the full triage; the automated cross-package harness
> remains the residual X3.

## Consumer-build touches (croft-chat integration)

Changes made while wiring the integrated Drystone CLI
(`experiments/alpha/croft-chat`, plan `…/plans/2026-06-26-1-plan-integrated-drystone-cli.md`).
These touch the consumer surface, mostly outside the current mutation scope
(`fold_auth.rs` + `governance.rs`); recorded here so the end-of-implementation
re-sweep (staged on secroute-testing-one) knows what moved.

| Date | Phase | Touched | Coverage added | Mutation status |
|---|---|---|---|---|
| 2026-06-26 | P2 | `surface.rs::next_lamport` (now delegates to the injected `LamportSource` instead of `unix_now()+1`); `fold_derived.rs` lamport check gains a `warn!` log (no logic change) | `surface::tests::test_lamport_rapid_sends_apply` (rapid same-second sends apply) + `test_lamport_monotonicity_still_enforced` (equal-lamport rejects, N+1 applies — pins both arms of the `fold_derived` Step-5 guard) | `cargo-mutants` not installed locally; the `next_lamport` delegation + the Step-5 guard are scoped into the deferred remote re-sweep. The guard's negative arm is now test-pinned, which should kill any `<= → <`/`== → !=` mutant on that comparison. |
| 2026-06-26 | P3 | `surface.rs`: new `get_message` + free fn `decode_message_payload` (read-only decode path; no fold/governance change) | `test_get_message_round_trips_body_and_reply` (body + Some/None reply_to arms) + `test_get_message_unknown_hash_is_none` (unknown → None) | New decode-path code; outside `fold_auth`/`governance` scope. The body-length and reply-marker boundaries (`< 4`, `len < body_end+4`, reply_len `0`/`32`/other) are pinned by the round-trip + unknown-hash tests; folds into the deferred re-sweep if scope is widened to `surface.rs`. |
| 2026-06-26 | P5 | `surface.rs`: new `export_group_log`, `ingest_foreign`, `export_assertion` (replication: scan-by-group, foreign-envelope decode+`fold.ingest`+notify, raw-bytes-by-hash) | `test_export_log_and_ingest_foreign_converges_shuffled` (whole-log replicate into a 2nd store, lamport-ordered apply, idempotent re-apply, body+reply+membership converge) | `ingest_foreign` routes through the same `fold.ingest` (sig+credential+monotonicity) as local writes, so the fold's existing vetted guards apply unchanged. The new surface scan/decode paths fold into the deferred `surface.rs`-scope re-sweep. |
| 2026-06-26 | P7 | `surface.rs`: new free fn `assertion_order_key` (decode device+lamport only) | exercised end-to-end by `croft-chat` `tests/convergence.rs` (two-node convergence) | Thin read-only decode wrapper over the private `decode_envelope_bytes`; no fold/governance change. Convergence over the scrambling transport is the behavioral proof. |
| 2026-06-26 | P12 | `surface.rs`: new `max_lamport_for_device` (by-device index range query) | `test_max_lamport_for_device_tracks_high_water_mark` (0 when empty, genesis=1→message=2 ⇒ 2) + sgc restart-resume test + binary-smoke cross-process | Read-only range over `auth_assertions_by_device_v1` (the same index the fold's Step-5 monotonicity check uses); no write path, no fold/governance change. |
| 2026-06-27 | P20 | none (read path only) | `test_contradiction_surfaces_fork_status` (two-genesis fork → `get_group_summary().fork_status` = `forked_from`) + croft-chat `tests/contradiction.rs` end-to-end | No substrate write change. **Trust gate UNCHANGED:** Proof B (hard-stop on contradiction) rests on `fold_auth::check_authorization` / `apply_governance` negative-path coverage — the survivors the paused sweep flagged. **Re-run the full sweep on `secroute-testing-one`** (cargo-mutants not installed locally) per the re-run commands below; prioritize killing the unauthorized-op survivors before any production trust claim. |
| 2026-06-26 | P13 | `types.rs`: shared message-payload codec; `surface.rs`: `send_message` channel param + `get_channel_timeline`; **`fold_derived.rs`: `Message` ingest routes the `References` edge to the channel** | `test_message_routes_to_channel_not_group` (two channels + group-level msg; #a isolated, #b empty, group holds only the channel-less msg) | **Touches the trust-critical fold** (the `Message` write path). The new branch is `decode_message_payload(payload) → channel ? channel : group` for the edge source; `channel=None` preserves prior behavior (covered by all existing convergence/timeline tests). The channel-routing branch + codec boundaries (reply markers 0/32, channel markers 0/1, legacy tail) need coverage in the deferred re-sweep — flag `fold_derived` `Message` ingest as a **priority target** alongside the auth/governance survivors. |

## Score

**RUN-01 EXP-3 scoped sweep (auth/threshold functions, `fold_auth` + `governance` + `fold_derived`):**
120 mutants → **54 caught, 61 missed, 5 unviable**. Substrate-only score = 54 / 115 = **47%**.

This 47% is the *substrate-only* figure and is **expected to be low by design**: the 61 survivors are
all authorization-*decision* mutants whose pinning tests live cross-package in `croft-chat`. Read by
population: **threshold-counting / quorum arithmetic (`governance.rs`) = 13/13 caught, 0 survivors**;
authorization decision (`check_authorization` + `role_ge_*`, split across `fold_auth.rs` and
`fold_derived.rs`) = cross-package-covered (one survivor hand-killed against the consumer test as
proof). The meaningful score requires the automated cross-package harness (residual X3). Full triage:
`X3-CROSS-PACKAGE-SWEEP.md`; raw record: `x3-sweep-data/`.
