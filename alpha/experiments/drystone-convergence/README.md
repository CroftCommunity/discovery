# drystone-convergence

> **Provenance (imported 2026-08-06).** This harness was originally written on a branch of an
> unrelated work repo and never landed here. Recovered from that branch (`drystone-convergence-harness`,
> 5 commits, last `2f37dc1`) and imported verbatim apart from scrubbing employer references to
> `croftc/upstream-repo`, matching the convention already used in
> `../../../beta/impl/experiments/drystone-experiments-consolidated.md`. Re-verified green on import:
> 46 tests pass (27 Stage 1, 3 Stage 2, 16 Stage 4). The prose describing these results already lived
> in `beta/impl/experiments/drystone-reviews-and-experiments-log.md`; this is the code behind it.

**TESTED AGAINST: Faithful reference fold implementing R1–R4 — NOT a production fold.**

No production implementation of the Drystone governance fold was found in this
repository (`croftc/upstream-repo`). The fold in `src/fold.rs` is a reference
implementation built faithfully from the §7.3.1 conflict-resolution semantics
(rules R1 through R4). A green run proves the *specified* conflict-resolution
model is order-independent and causally correct. It does not prove a production
implementation; none exists in this repository.

---

## What is being tested

### Stage 1 v2: R1-R4 properties (A through F)

The fold is a pure function from a set of governance facts to an authority state:

```
fold(facts: Set<Fact>) -> Result<AuthorityState, GapError>
```

**Property A (permutation invariance):** For any complete valid fact-set S and
any permutation π, `fold(S) == fold(π(S))`. The condition of order-independence
the entire governance-scaling claim rests on.

**Property B (causal precedence, R1):** A causally-later fact wins even when it
carries a smaller FactId. The discriminating property: the v1 highest-id-wins
stub fails this. Tested with explicit synthetic ids (legitimate because R1 must
hold for any id assignment).

**Property C (concurrent tiebreak, R1):** Genuinely concurrent conflicting facts
resolve to the greatest FactId. Deterministic and permutation-invariant.

**Property D (role cascade, R2):** Cross-slot effects (member removal revoking
roles; the effective-roles projection) are computed on the final resolved slots,
never incrementally. Covers re-add-does-not-restore (D3), re-grant-works (D4),
and permutation-invariance at scale (D5).

**Property E (idempotent no-ops, R3):** Operations on absent targets are no-ops;
never rejected at fold time.

**Property F (threshold LWW, R4):** Threshold changes resolve by R1 (causal-LWW,
tiebreak for concurrents).

### Stage 2: gap-detection simulation

Seeded deterministic multi-node simulation with controlled delivery. Tests:

- **Referenced-gap detection (MUST PASS):** A node holding a fact whose
  predecessor is absent must detect the gap rather than fold as if complete (R3).
- **Unreferenced-tail gap (documented limit):** A missing new-head fact that
  nothing points to is NOT detectable by references alone. See RESULTS.md.
- **Convergence after fill:** After the missing fact is delivered, a gapped node
  converges to the identical fingerprint as a node that always held the full set.

---

## Repository layout

```
drystone-convergence/
├── Cargo.toml
├── README.md
├── RESULTS.md
├── proptest-regressions/        # proptest writes failing seeds here; commit them
│   └── .gitkeep
├── src/
│   ├── lib.rs                   # crate root
│   ├── types.rs                 # Fact, FactId, FactPayload, AuthorityState
│   ├── fold.rs                  # R1-R4 + A12 reference fold + GapError
│   ├── finality.rs              # Stage 4: Vote, quorum_fold, Ceiling, InFlightTally, Now
│   └── simulation.rs            # Stage 2 Node model
└── tests/
    ├── convergence.rs           # Properties A–F + D6/D7 A12 (proptest + hand cases)
    ├── stage2.rs                # Gap-detection and convergence-after-fill tests
    └── stage4.rs                # Groups G/H/I/K: quorum, ceiling, Now
```

---

## How to run

```bash
cd drystone-convergence

# Run all tests:
cargo test

# Run Stage 1 properties only:
cargo test --test convergence

# Run Stage 2 simulation only:
cargo test --test stage2

# Run Stage 4 finality tests only:
cargo test --test stage4

# Heavy run — raise proptest case count:
PROPTEST_CASES=10000 cargo test --test convergence

# Verbose output for a specific property:
cargo test prop_a_permutation_invariance -- --nocapture
```

---

## Case counts

### Stage 1 v2 (convergence.rs) — 27 tests total

| Test                          | Cases | Method         |
|-------------------------------|------:|----------------|
| `prop_a_permutation_invariance` | 1 000 × 5 shuffles | proptest |
| `prop_b_causal_beats_id_proptest` | 500  | proptest / explicit ids |
| `prop_c_concurrent_tiebreak_proptest` | 500 | proptest |
| `prop_d5_role_cascade_permutation_invariance` | 500 × 5 shuffles | proptest |
| `prop_f2_threshold_tiebreak_proptest` | 500 | proptest |
| Hand-written tests (B–F, D6/D7 A12) | 20 | deterministic |

### Stage 2 (stage2.rs) — 3 tests total

| Test | Method |
|------|--------|
| Referenced-gap detection | deterministic simulation |
| Unreferenced-tail gap (documented limit) | deterministic simulation |
| Convergence after fill (multi-node) | deterministic simulation |

### Stage 4 (stage4.rs) — 16 tests total

| Group | Tests | What it verifies |
|-------|------:|-----------------|
| G (quorum folding) | 5 | A1: `quorum_fold` correctness, permutation-invariance, sub-k detection |
| H (concurrent completion) | 2 | A2: non-exclusive recognition; unanimous canonical result |
| I (ceiling) | 3 | A3: `at_head` correctness, canonical head via R1 tiebreak, `voids_action_at` |
| K (the Now) | 6 | A7: fingerprint stability, replacement semantics, attestation exclusion, K3 commit predicate |

---

## Honest scope of a passing result

- Passing A–F establishes that the **specified** R1-R4 model is order-independent
  and causally correct in the reference fold. It does not establish that a
  production fold is correct; none exists.

- The order-independence result is **conditional on gap-completeness**: an
  undetected gap could hide a causally-later fact, changing the resolved value
  of a slot. The referenced-gap test exercises this partially (and passes). The
  unreferenced-tail case documents where reference-based detection stops.

- Sampling is not exhaustion; Stage 3 (bounded exhaustive model checking, not
  yet implemented) is the path to a stronger claim.

---

## Staged plan

| Stage | Status       | Description |
|-------|-------------|-------------|
| 1 v2  | ✅ complete  | R1-R4 + A12 faithful fold; Properties A–F + D6/D7; confirmed broken stub fails B |
| 2     | ✅ complete  | Gap-detection simulation; referenced-gap PASS; unreferenced-tail documented |
| 3     | specified only | Adversarial scheduler; equivocation detection; bounded exhaustive model checking |
| 4     | ✅ complete  | Quorum folding, Ceiling, InFlightTally, Now (Groups G/H/I/K; 16 tests) |
