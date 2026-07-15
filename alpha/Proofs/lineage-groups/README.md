# Lineage Groups — validation workspace

Rust workspace validating the **Lineage Groups** thesis: model a group as a
navigable lineage of conversations grounded in cryptographic provenance, where
"knowing reliably where a conversation branched from" is simultaneously the
security invariant and the social-legibility invariant.

This repo implements the phased experiment plan from the thesis. Work is
**sequenced and gated** — each phase has a go/no-go gate, and a failed gate is a
thesis finding, not just a bug.

## Status

| Phase | Scope | Status |
|-------|-------|--------|
| **0 — Scaffold** | workspace, deterministic clock + RNG, scenario/invariant harness | ✅ done |
| **1 — Crypto feasibility** (the gate) | openmls create/add/remove, **external-commit survivor re-key**, fresh-genesis, queued revocation | ✅ **GO** — see [`PHASE_1_FINDINGS.md`](./PHASE_1_FINDINGS.md) |
| **2 — Data model + merge semantics** | governance fork/heal, deterministic survivor selection, conflict hard-stop, history-as-tree, verifiable backfill | ✅ **GO** — see [`PHASE_2_FINDINGS.md`](./PHASE_2_FINDINGS.md) |
| **3 — End-to-end over a transport** | genesis→fork→recombine, partition/reconnect, recovery, blind broker | ✅ **GO** (with documented iroh caveat) — see [`PHASE_3_FINDINGS.md`](./PHASE_3_FINDINGS.md) |
| **2.5 — Adversarial pass** | fuzzed convergence, admin equivocation, backfill forgery | ✅ — closed 2 real gaps; see [`PHASE_2_5_FINDINGS.md`](./PHASE_2_5_FINDINGS.md) |
| **2.6 — Authority & override** | departed-admin authority lifetime, quorum override of conflict | ✅ — closed 1 gap + built governed override; see [`PHASE_2_6_FINDINGS.md`](./PHASE_2_6_FINDINGS.md) |

**All three gates are GO.** Every invariant the thesis set out to falsify
(I1–I10) held in deterministic simulation across the phases. Phase 3 runs over an
**in-process blind broker behind a transport trait** — the real `iroh`
dependency is deliberately *not* vendored (it locks ~395 packages incl.
pre-release crypto colliding with our pinned stable crates, and real P2P is
outside this environment's network policy), so **no real P2P transport was
exercised**. Through that seam it does wire the real Phase 1 MLS + Phase 2
governance across nodes. See the Phase 3 findings.

## Crates

- **`lineage-core`** — deterministic logical clock, seeded RNG, identity/
  provenance types, **and the Phase 2 data model**: signed governance op log
  with immutable genesis rules (`gov`), lineage DAG + standing (`dag`),
  deterministic survivor selection (`survivor`), conflict detector with human
  escalation (`conflict`), Ed25519 governance signing (`keys`).
- **`lineage-mls`** — thin, auditable wrapper over `openmls` (RFC 9420).
  Isolates every openmls assumption in one place. **Phase 1 lives here.**
- **`lineage-history`** — per-branch signed message history with fold/unfold and
  verifiable consensual backfill (no cross-branch interleave). **Phase 2.**
  Automerge (for in-branch concurrent edits) is deferred; see findings.
- **`lineage-sim`** — in-process transport + partition simulator and the
  scenario/invariant harness.
- **`lineage-iroh`** — Phase 3 transport-seam slice: a structurally-**blind
  broker** (relay + queue + recovery snapshots over opaque bytes) behind a
  transport trait, with node wiring that exercises the real MLS + governance
  across nodes *without* vendoring real `iroh` (no real P2P; see findings).

## Running

```sh
cd experiments/lineage-groups
cargo test            # whole suite (Phase 0 + Phase 1)
cargo test -p lineage-mls   # Phase 1 experiments E1.1–E1.4
cargo clippy --workspace --all-targets
```

## Conventions

- **Determinism is mandatory** in logic crates: no `SystemTime::now()` / unseeded
  RNG. Time is a Lamport clock; randomness is seeded ChaCha20. (MLS key material
  is necessarily non-deterministic — see the honesty boundary in the findings.)
- **Pinned dependencies:** exact `=x.y.z` versions in `Cargo.toml`, committed
  `Cargo.lock`. No floating ranges.
- **Verify against the real library, not docs.** A wrong assumption caught early
  is the plan working as intended.
- Experiments are integration tests named to match the plan (`e1_2_external_commit_survivor`, …).
