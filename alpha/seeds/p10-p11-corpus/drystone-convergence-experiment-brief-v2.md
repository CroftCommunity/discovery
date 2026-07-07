# Drystone convergence experiment v2: testing the §7.3.1 resolutions

## What changed since v1, and why this exists

The v1 brief (`drystone-convergence-experiment-brief.md`) asked you to build a permutation-invariance harness and, if no production fold existed, a reference stub. You did that. The stub passed, and in building it you surfaced four open semantic questions (OQ-1 through OQ-4) because the specification did not answer them, so the stub used a placeholder rule (highest-FactId-wins) whose order-independence was trivial and uninformative.

Those four questions are now resolved in the specification (see `p10-drystone-fold-semantics.md`, rules R1 through R4). This brief extends the experiment to test the *resolved* semantics. The change is significant: the reference fold you build now is **faithful to a real specified model**, not a trivial placeholder, and the new properties below are **discriminating**, meaning a wrong implementation fails them. In particular, the v1 stub's highest-FactId-wins rule now **fails** the causal-precedence property (Property B), which is exactly the point.

Read this whole brief and read `p10-drystone-fold-semantics.md` before writing code. The rules R1 through R4 there are authoritative for the semantics; this brief is authoritative for what to test.

## The honesty rule still holds, and is now sharper

Every result file MUST state in its first line which fold was tested:

- If a production fold exists in the repository, wire to it and say so.

- If not, build a reference fold **faithful to R1 through R4** and label it a reference fold, not production. A green run now means "an R1-through-R4-faithful fold is order-independent and causally correct," which is real evidence about the specified semantics, but it is still not evidence about a production implementation, because none exists in this repository (`croftc/securitypolicy`).

State this distinction in RESULTS.md exactly. Do not let "faithful reference fold" drift into "the protocol is verified." It means the specified conflict-resolution model is internally sound and testable, which is a genuine step up from the v1 stub, and no more.

## The fold contract, now specified (R1 through R4)

Implement the fold as a per-slot causal last-writer-wins register with cross-slot projection. The full rationale is in `p10-drystone-fold-semantics.md`; here is what to implement.

**Fact model (note the required addition).**

- `id: FactId`: content hash in production. **For the harness, allow ids to be assigned explicitly** (a synthetic ordered id), because Property B requires constructing a causally-later fact with a smaller id, which you cannot do if the id is a hash of the content. Document that production uses content hashes and that R1's correctness must hold for *any* id assignment, which is what makes explicit test ids legitimate.

- `author: AuthorId`: persona or device subspace id.

- `predecessors: Set<FactId>`: **new and required.** The frontier of facts the author had observed when authoring this fact. This is the causal dependency reference. R1's causal order and R3's gap-detection both depend on it; a fact model without it cannot implement these semantics.

- `payload`: one of `AddMember(m)`, `RemoveMember(m)`, `GrantRole(m, r)`, `RevokeRole(m, r)`, `SetThreshold(role, k, n)`.

**Causal order.** Build the DAG from `predecessors`. Fact B is *causally after* A iff A is in the transitive closure of B's predecessors within the folded set. Two facts are *concurrent* iff neither is causally after the other. If a referenced predecessor is not in the set, that is a **detected gap** (relevant in Stage 2, not Stage 1, where sets are complete).

**Slots.** `member:m`, `role:m:r`, `threshold:role`.

**Fold algorithm.**

1. Build the causal DAG from predecessors.

2. For each slot, find the causally-maximal facts touching it (touching the slot and not causally dominated by another fact touching the same slot).

3. Resolve each slot (R1): if one causally-maximal fact, its operation wins; if several (concurrent), the greatest by the tiebreak (FactId order) wins. The tiebreak is consulted **only** among concurrents and MUST NOT override causal order.

4. Membership: `member:m` is a member iff its resolved winning operation is `AddMember`. `RemoveMember` resolves it to not-a-member.

5. Roles (R2): the operations on `role:m:r` are `GrantRole` and `RevokeRole`, and additionally every `RemoveMember(m)` acts as a revoke on all `role:m:*` at that removal's causal position. Resolve `role:m:r` by causal-LWW over that operation set.

6. Effective-roles projection (R2): role `(m, r)` is effective iff `role:m:r` resolved to granted **and** `member:m` resolved to member. Compute this on the final resolved slots, once, never incrementally.

7. Thresholds (R4): `threshold:role` resolved by causal-LWW (R1).

8. Absent targets (R3): operations on absent targets are idempotent no-ops. A remove of a never-added member leaves it not-a-member; a revoke of a never-granted role leaves it ungranted. Never reject a fact at fold time for an absent target.

**AuthorityState.** `{ members: Set, effective_roles: Set<(m, r)>, thresholds: Map<role, (k, n)> }`. Canonical ordering (BTreeSet/BTreeMap) and a `fingerprint()` (canonical serialization then SHA-256), exactly as in v1.

## Properties to test

Property A is retained from v1. B through F are new and target the resolutions. Build them as `proptest` properties plus hand-written cases, in the style v1 established.

**Property A (retained): permutation invariance.** For any complete valid fact-set S and any permutation, `fold(S) == fold(permutation)` by fingerprint. Now against the faithful fold.

**Property B (new, R1, the discriminating one): causal precedence beats id order.** Construct fact-sets containing a causally-later fact and a causally-earlier fact that conflict on the same slot, where the causally-later fact is assigned the **smaller** id. Assert the causally-later fact wins. This is the property the v1 highest-id-wins stub fails, so implement it first after the fold, and confirm that a deliberately id-only fold fails it (then remove that broken variant). Because it uses explicit ids, state in the harness that the property must hold for every id assignment, which is R1's guarantee.

- Minimal hand case: `Add(m)` [id 5, no preds]; then `GrantRole(m, r)` [id 4, preds {5}]; then `RevokeRole(m, r)` [id 1, preds {4}]. The revoke is causally last and has the smallest id. Assert `(m, r)` is not effective. An id-only fold would keep the grant (id 4 beats id 1) and get this wrong.

**Property C (new, R1): concurrent tiebreak determinism.** Construct genuinely concurrent conflicting facts (same slot, neither in the other's predecessor closure). Assert the resolved value is the tiebreak winner (greatest id among the concurrent maximal facts) and is permutation-invariant. Assert two fold invocations over different input orders agree.

**Property D (new, R2): role cascade via projection.** Several cases, each also checked for permutation-invariance (fold the permuted set, effective roles identical):

- D1, causal grant then remove: `Add(m)`, `Grant(m, r)`, `Remove(m)` in a causal chain. Assert m is not a member and `(m, r)` is not effective.

- D2, concurrent grant and remove: `Add(m)` [earlier]; then concurrent `Grant(m, r)` and `Remove(m)` (siblings, both with preds pointing only to the Add). Assert m is not a member and `(m, r)` is not effective, **regardless of which of grant or remove wins the role-slot tiebreak**, because the membership projection filters it. If you can construct both id orders, assert both give no effective role.

- D3, re-add does not restore: `Add(m)`, `Grant(m, r)`, `Remove(m)`, `Add(m)` in a causal chain. Assert m is a member and `(m, r)` is **not** effective (the prior grant was revoked by the removal; re-adding does not restore it).

- D4, re-grant after re-add works: extend D3 with a causally-later `Grant(m, r)`. Assert m is a member and `(m, r)` is effective.

- D5, cascade order-independence: for each of D1 through D4, assert every permutation yields identical effective roles. This is the assertion that a naive incremental cascade would fail.

**Property E (new, R3): idempotent no-ops.** 

- E1: fold `{ Remove(m) }` with no `Add(m)`. Assert m is not a member, and the membership outcome equals folding the empty set for m.

- E2: fold `{ Revoke(m, r) }` with no grant. Assert `(m, r)` not effective, no error.

- E3: assert `fold({ Remove(m) })` and `fold({ Add(m), Remove(m) })` (causal) agree on m's membership (both not-a-member), demonstrating the no-op equivalence.

**Property F (new, R4): threshold LWW.**

- F1, causal: `SetThreshold(role, 2, 3)` then causally-later `SetThreshold(role, 3, 5)`. Assert the resolved threshold is `(3, 5)`.

- F2, concurrent: two concurrent `SetThreshold` on the same role. Assert the resolved threshold is the tiebreak winner's value and is permutation-invariant.

Stage 1 acceptance (v2): the faithful fold passes A through F over a large proptest case count, Property B is confirmed to fail an id-only fold (then that variant removed), the hand cases for B, D2, D3 are present as explicit unit tests, and the regression file is committed. State in RESULTS.md that the fold is a faithful reference implementation of R1 through R4, not production.

## Stage 2 (extended): gap-detection, including the causal-gap case

Build the seeded deterministic simulation as v1 Stage 2 specified (N node-models, controlled delivery orders, controlled omissions, pairwise reconciliation then fold, single-threaded and seed-reproducible). Add the following, which ties directly to R1 and R3 and is the highest-value part of this stage.

**Referenced-gap detection (MUST pass).** Give a node a fact G whose `predecessors` include a fact F that the node does not hold, where F is a causally-later operation on some slot than what the node has folded (so F, if present, would change a resolved slot value). Assert the node **detects** the gap, because G references an absent predecessor F (R3's predecessor-absence-is-a-gap rule), rather than folding its incomplete set and confidently emitting the stale slot value. This is the direct OQ-1-to-gap-detection tie: a gap that hides a causally-later fact must not silently produce the wrong causal winner.

**Unreferenced-tail gap (documents the limit, expected NOT detectable by references alone).** Give a node a complete-looking set that is missing a fact F that is a new head nothing the node holds points to (a pure tail). Assert that reference-based detection alone does **not** catch this, and record it in RESULTS.md as the case that the completeness-ahead corroboration and the dataplane checkpoint must handle. This is honest scope: completeness *behind* a known checkpoint is provable, completeness *ahead* is only corroborated (see `p10-drystone-scaling-and-ordering.md` and `p10-drystone-fold-semantics.md` open items). Do not fake a pass here; the point is to show precisely where reference-based detection stops and the open item begins.

**Convergence after fill.** After the missing fact is delivered and reconciled, assert the previously-gapped node converges to the identical fingerprint as a node that always had the complete set.

Do not start Stage 2 until Stage 1 v2 is green and committed.

## Stage 3 (specified only, unchanged from v1)

Adversarial scheduler maximizing divergence, equivocation detection (both conflicting facts retained and surfaced as a fork per §7.6, not silently overwritten), and bounded exhaustive model checking (stateright or TLA+) for small node counts. Do not build yet.

## Honest scope (put in README, extends v1)

- The v1 honest-scope paragraph still holds: sampling is not exhaustion; a reference fold is not the production protocol; passing establishes convergence, not other properties.

- New for v2: passing A through F establishes that the **specified** conflict-resolution model (R1 through R4) is order-independent and causally correct as implemented in the reference fold. It does not establish gap-completeness, which Stage 2's referenced-gap test partially exercises and whose unreferenced-tail case is explicitly left open. The whole order-independence result remains **conditional on gap-completeness**, per the fold-semantics doc.

- Property B's use of explicit ids is legitimate precisely because R1 requires causal precedence to hold for any id assignment; the harness should state this so a reader does not mistake synthetic ids for a shortcut.

## Deliverables

Extend the existing `drystone-convergence/` crate. Add the faithful fold (or wire to production if it now exists), the B-through-F properties and their hand cases, the extended Stage 2 simulation with the two gap cases and the convergence-after-fill check, updated README and RESULTS. RESULTS.md must state, first line, real-versus-reference and faithful-to-R1-through-R4, then report case counts, what passed, and specifically the outcome of the referenced-gap test (expected pass) and the unreferenced-tail case (expected documented-limit, not a pass).

## The one rule above all (unchanged)

State in the first line of every result whether the fold is production or a reference implementation, and now also that the reference implements R1 through R4. If any new semantic ambiguity arises that R1 through R4 do not cover, do not resolve it silently; list it in RESULTS.md as a new open question, because it may be a residual source of order-dependence the resolutions did not reach.
