# Drystone Convergence Experiment — Results (v2)

**TESTED AGAINST: Faithful reference fold implementing R1 through R4 (NOT a production fold).**

No production implementation of the Drystone governance fold exists in this
repository (`croftc/upstream-repo`). The fold in `src/fold.rs` is a reference
implementation built faithfully from the §7.3.1 conflict-resolution semantics
(rules R1 through R4, from `p10-drystone-fold-semantics.md`). A green run
establishes that the *specified* conflict-resolution model is order-independent
and causally correct. It does not establish that a production implementation
exists or is correct; no production fold is present in this repository.

---

## Stage 1 v2 — R1-R4 + A12 properties (A through F, D6/D7)

| Metric              | Value |
|---------------------|-------|
| Fold tested         | Faithful reference fold (`src/fold.rs`), R1–R4 + A12 layered-fold |
| Test file           | `tests/convergence.rs` |
| Proptest cases      | 1 000 (Property A), 500 (Properties B, C, D5, F2) |
| Shuffles per case   | 5 (Property A, D5) |
| Hand-written tests  | 20 (Properties B–F hand cases, D6/D7 A12 discriminating, + retained unit tests) |
| Total test count    | 27 |
| Result              | All pass |

### Property A — permutation invariance (retained from v1)

For any complete valid fact-set S and any permutation, `fold(S) == fold(π(S))`
by fingerprint. Now verified against the R1-R4 faithful fold.

### Property B — causal precedence beats FactId order (R1, discriminating)

A causally-later fact must win even when it carries a smaller FactId. This is
the property the v1 highest-id-wins stub fails.

**Minimal hand case** (from the brief):
- `Add(m)` [id=5, no preds]
- `GrantRole(m,r)` [id=4, preds={5}] — causally after Add
- `RevokeRole(m,r)` [id=1, preds={4}] — causally last, smallest id

The revoke is causally last. An id-only fold would keep the grant (id 4 > id 1).
The R1 fold correctly selects the revoke; `(m, r)` is not effective.

**Broken fold verification**: an inline simulation of the id-only rule confirms
it selects the grant (id 4 wins over id 1), which is wrong. The broken variant
was never added as a real fold function; only the R1-R4 fold is shipped.

**Explicit-id legitimacy**: R1 requires causal precedence to hold for any id
assignment. Explicit synthetic ids directly exercise this guarantee. The
harness documents this so a reader does not mistake synthetic ids for a shortcut.

### Property C — concurrent tiebreak determinism (R1)

Genuinely concurrent conflicting facts (neither in the other's predecessor
closure) resolve to the greatest FactId. Verified for both orderings.

### Property D — role cascade via projection (R2) + A12 type precedence

| Sub-property | Scenario | Result |
|---|---|---|
| D1 | Causal chain: Add → Grant → Remove | m not a member; (m,r) not effective ✓ |
| D2 | Concurrent Grant and Remove (both after Add) | (m,r) not effective regardless of which wins role tiebreak ✓ |
| D3 | Add → Grant → Remove → Add2 | m is member; (m,r) NOT effective (re-add does not restore) ✓ |
| D4 | Extends D3 with causal Grant2 | m is member; (m,r) IS effective ✓ |
| D5 | Permutation-invariance for D scenarios at scale | All pass ✓ |
| D6 | A12 discriminating: concurrent RemoveMember(tier 2) vs AddMember(tier 5), AddMember has larger id | RemoveMember wins; flat id-only fold would pick AddMember ✓ |
| D7 | A12 discriminating: RemoveMember cascade(tier 2) vs concurrent GrantRole(tier 4) | Role slot: Remove cascade wins over Grant; flat fold would pick Grant ✓ |

All D scenarios checked for every permutation (Heap's algorithm, exhaustive for
the hand cases) and by proptest sweep. A naive incremental cascade would fail
D5 because it is order-dependent by construction.

### A12 — layered-fold type precedence (added in v2)

When concurrent facts in the same slot are causally maximal (neither is in the
other's predecessor closure), the lowest-tier type wins rather than the greatest
FactId alone:

| Tier | Type | Beats |
|------|------|-------|
| 1 | SetThreshold | all others |
| 2 | RemoveMember | tiers 3–5 |
| 3 | RevokeRole | tiers 4–5 |
| 4 | GrantRole | tier 5 |
| 5 | AddMember | — |

Causal-later always wins regardless of tier (R1 still dominates). Within the
same tier, the greatest FactId tiebreak (R1) applies. Tests D6 and D7 are
discriminating: they construct scenarios where a flat id-only fold would produce
the wrong answer (selecting the higher-id fact when type precedence should select
the lower-id one).

### Property E — idempotent no-ops (R3)

| Sub | Scenario | Result |
|---|---|---|
| E1 | `RemoveMember(m)` with no `AddMember(m)` | m not a member; no error ✓ |
| E2 | `RevokeRole(m,r)` with no grant | (m,r) not effective; no error ✓ |
| E3 | `{Remove(m)}` and `{Add(m), Remove(m)}` agree on m's membership | Both not-a-member ✓ |

### Property F — threshold LWW via R1 (R4)

| Sub | Scenario | Result |
|---|---|---|
| F1 | Causal chain: SetThreshold(2,3) → SetThreshold(3,5) | (3,5) wins ✓ |
| F1-explicit | Causally-later threshold has smaller id | Smaller-id (causally later) wins ✓ |
| F2 | Concurrent SetThreshold on same role | Tiebreak winner's value; permutation-invariant ✓ |

---

## Stage 2 — Gap-detection simulation

| Test | Result |
|---|---|
| Referenced-gap detected (MUST PASS) | **Pass** ✓ |
| Unreferenced-tail gap not detectable (DOCUMENTED LIMIT) | Documented correctly ✓ |
| Convergence after fill (single-gap) | **Pass** ✓ |
| Convergence after fill (multi-node reconciliation) | **Pass** ✓ |

### Referenced-gap detection

A node holding fact G whose predecessor F is absent (and where F, if present,
would change a resolved slot value) returns `GapError` from `fold()`, naming F
as the missing fact. The node does not fold onward as if complete (R3). After
delivering F, the node converges to the identical fingerprint as a node that
always held the complete set. **This is the direct OQ-1-to-gap-detection tie.**

### Unreferenced-tail gap — documented limit

A node whose set looks complete (no absent predecessors) but is missing a fact
F that is a new head (nothing the node holds points to F) folds successfully
without detecting the gap. The stale result differs from the full result, but
the node cannot detect this discrepancy through predecessor references alone.

**This is the expected behaviour** and is recorded here as the documented limit:
completeness *behind* a known checkpoint is detectable; completeness *ahead*
requires completeness-ahead corroboration and the dataplane checkpoint (per
`p10-drystone-scaling-and-ordering.md` and `p10-drystone-fold-semantics.md`
open items). This test confirms precisely where reference-based detection stops.

---

## Honest scope (v2)

Passing A through F establishes that the **specified** conflict-resolution model
(R1 through R4) is order-independent and causally correct as implemented in the
reference fold. The whole order-independence result remains **conditional on
gap-completeness**, per the fold-semantics doc: an undetected gap could hide a
causally-later fact and change a slot's resolved value.

- The referenced-gap test partially exercises gap-completeness and passes.
- The unreferenced-tail case confirms the limit: completeness *ahead* of a known
  checkpoint is not provable by references alone and requires additional
  mechanisms not yet implemented.
- This is not a production result; no production fold exists in this repository.

---

## Dependency license notes

The `Cargo.lock` includes `r-efi` (versions 5.3.0 and 6.0.0) as a transitive
dependency via `getrandom → (rustix / wasip2)`. `r-efi` is a UEFI-platform-only
library; on Linux and macOS `getrandom` uses `libc` instead and `r-efi` is never
compiled or linked. The package is licensed `MIT OR Apache-2.0 OR LGPL-2.1-or-later`;
it is used under the MIT or Apache-2.0 option. The LGPL flag from Cycode's scanner
was reviewed and acknowledged via `#cycode_ignore_non_permissive_license_use` on PR #13.

---

## Stage 4 — Quorum folding, ceilings, and the Now

| Metric              | Value |
|---------------------|-------|
| Source              | `src/finality.rs` |
| Test file           | `tests/stage4.rs` |
| Hand-written tests  | 16 |
| Result              | All pass |

### Group G — quorum folding (A1)

| Test | Scenario | Result |
|------|----------|--------|
| G1 | k-1 concordant votes → Insufficient | ✓ |
| G2 | Exactly k concordant votes → Crossed | ✓ |
| G3 | Single vote insufficient when k > 1 | ✓ |
| G4 | Vote set in any input ordering yields same Crossed result | ✓ |
| G5 | Sub-k enactment returns Insufficient; Ceiling::stamp returns None | ✓ |

`quorum_fold` deduplicates by author (keeps max FactId per author), requires
concordant and eligible votes, and selects `completing_vote` as the k-th entry
in descending FactId order (the minimum of the quorum set). A sub-k call returns
`Insufficient`; a valid ceiling cannot be stamped from it, making sub-k
enactments detectable as fork origins.

### Group H — non-exclusive recognition (A2)

| Test | Scenario | Result |
|------|----------|--------|
| H1 | Three pairs each independently see quorum; all three report Crossed | ✓ |
| H2 | Unanimous case (k=N=3): one canonical Crossed regardless of ordering | ✓ |

Quorum recognition is non-exclusive: multiple nodes crossing the threshold at
the same moment each observe `Crossed` with the **same** `completing_vote`.
There are N observations, not N rival decisions.

### Group I — ceiling (A3)

| Test | Scenario | Result |
|------|----------|--------|
| I1 | `at_head` equals `completing_vote` from quorum result | ✓ |
| I2 | Two concurrent ceilings; canonical head = max(at_head) by R1 tiebreak | ✓ |
| I3 | `voids_action_at(h)` is true iff h strictly after ceiling head | ✓ |

### Group K — the Now (A7)

| Test | Scenario | Result |
|------|----------|--------|
| K1 | Re-derivation from same facts reproduces fingerprint; tampered Now differs | ✓ |
| K2 | Advancing the Now replaces it (different head → different fingerprint) | ✓ |
| K3 | `requires_enforcing_commit`: true for Add/Remove; false for Grant/Revoke/SetThreshold | ✓ |
| K4 | Same in-flight content in two insertion orders produces identical fingerprint | ✓ |
| K5 | N attestations do not change fingerprint; one Now object, N signatures | ✓ |
| K6 | 2-of-3 tally is not crossed; 3-of-3 tally is crossed but not enacted | ✓ |

The Now fingerprint covers authority state, in-flight tallies (sorted by key),
and head; it intentionally excludes attestations (per-node metadata, must not
affect convergence identity).

---

## Stage 3 — Specified only

Stage 3 (adversarial scheduler maximising divergence, equivocation detection
surfacing a fork per §7.6, bounded exhaustive model checking) is specified in
the brief and not yet implemented.

---

## Open semantic questions (resolved)

OQ-1 through OQ-4 from the v1 experiment have been resolved in the specification
(`p10-drystone-fold-semantics.md`, rules R1 through R4). No new semantic
ambiguities arose during implementation of the v2 fold. If any arise in future
work, they will be listed here rather than resolved silently.
