# Drystone convergence experiment: implementation brief for a coding agent

## What you are building and why

You are building a test harness that checks one property of the Drystone governance fold: **convergence**, meaning any set of governance facts, delivered in any order and with any gaps-then-fills, yields one identical folded authority state on every node. This property is what lets Drystone order governance without a central referee. It is currently asserted by the design (spec section 7.3.1) but not demonstrated. Your job is to demonstrate it, or to find the minimal case where it fails.

This is a property-based and simulation testing task in Rust. Build it in stages. Stage 1 is the priority and must be complete and passing before Stage 2 is started. Do not skip ahead.

Read this whole brief before writing code. If anything here conflicts with what you find in the repository, stop and surface the conflict rather than guessing.

## The critical seam: real fold versus stub

The experiment tests a `fold` function. There are two possibilities and you must determine which applies, and state which you used in every result you report.

1. **A real fold implementation exists in this repository.** If so, find it, wire the harness to it, and test the real thing. Search for the governance fold, the term is likely "fold", "governance", "authority state", or a reference to "7.3.1". Report the path you wired to.

2. **No fold implementation exists yet.** If so, build a faithful reference stub from the specification (see "The fold contract" below) in the test crate, and test that. State loudly in your output and in the code comments that this is a stub, not the production fold.

This distinction is load-bearing and must never be blurred. A green run against a stub proves the harness and the reference logic are sound; it does not prove the production protocol is sound. A green run against the real fold is the result that matters. Every report you produce must say, in its first line, which of the two you tested.

If you build a stub, keep it deliberately simple and obviously correct, because its purpose is to exercise the harness and to encode the spec's intended semantics, not to be efficient. Simplicity is the point.

## The fold contract

The fold is a pure function from a set of governance facts to an authority state:

```
fold(facts: Set<Fact>) -> AuthorityState
```

The properties the fold must satisfy, which are exactly what the harness checks:

- **Determinism.** Same set in, same state out, every time, on every machine. No dependence on hash-map iteration order, floating point, wall-clock, thread scheduling, or insertion order.

- **Order-independence (commutativity over the set).** The result depends only on which facts are in the set, not on the order they were folded in. This is the core property.

- **Totality of the tiebreak.** When two facts conflict (for example two changes to the same threshold), the specification defines a total order that decides the outcome deterministically. There must be no pair of distinct facts for which the tiebreak is undefined or symmetric (a symmetric tiebreak would make the outcome order-dependent).

The minimal fact and state model for Stage 1 (extend only if the real fold needs more):

- A `Fact` has: a unique id (content hash or ULID), an author (persona/subspace id), a logical counter or dependency reference establishing its causal position, and a payload that is one of a small set of governance operations. Start with these operation kinds because they exercise the interesting cases:
  - `AddMember(member_id)`
  - `RemoveMember(member_id)`
  - `GrantRole(member_id, role)`
  - `RevokeRole(member_id, role)`
  - `SetThreshold(role, k, n)`

- An `AuthorityState` is: the current member set, the role assignments, and the current thresholds. It must implement equality that is exact and canonical (see "Comparing states").

Do not invent semantics the spec does not have. If a semantic question arises that the spec section 7.3.1 does not answer (for example, what happens when a role is granted to a member who was concurrently removed), do not silently pick an answer. Surface it as an open question in your output, because an ambiguity there is precisely the kind of latent order-dependence this experiment exists to find.

## Comparing states

The equality check is as important as the fold. Two `AuthorityState` values must compare equal if and only if they represent the same authority, regardless of internal representation. Pitfalls to handle:

- Sets and maps must compare by content, not by iteration order. Use ordered collections (BTreeMap, BTreeSet) or a canonical serialization before comparison, so that two states built by different fold orders compare byte-identically.

- Serialize the state to a canonical form (sorted keys, deterministic encoding) and hash it, then compare hashes, so a mismatch gives you a stable fingerprint you can log and diff. Do not compare debug strings.

## Stage 1: permutation invariance of the fold

This is the priority deliverable. Use `proptest`.

The property, in words: for any valid set of facts, folding it under any permutation of that set yields the identical state.

Structure:

1. **A generator for valid fact-sets.** Write a `proptest` strategy that produces a `Vec<Fact>` representing a valid, causally-consistent set. "Valid" means the facts could actually have been produced by some run: a `RemoveMember(x)` should generally be preceded by an `AddMember(x)`, a `GrantRole` references a member that was added, and so on. Generate realistic causal dependencies, do not generate pure noise, because the interesting bugs live in valid-but-concurrent cases, not in malformed input. Include, deliberately, cases with concurrency: two facts from different authors with no causal ordering between them, and pairs of conflicting facts (two `SetThreshold` on the same role, an `AddMember` and `RemoveMember` of the same member with no causal order between them). Those conflict cases are where a non-total tiebreak will surface.

2. **The invariant.** For a generated fact-set `S`:
   - Compute `baseline = fold(S)`.
   - Generate several random permutations of `S` (use proptest's shuffling, or take the permutation order as an additional generated input).
   - Assert `fold(permutation) == baseline` for every permutation, comparing by the canonical fingerprint.

3. **Shrinking.** Ensure the generator supports proptest's shrinking so that any failure is reported as a minimal counterexample: the smallest fact-set and the specific permutation pair that diverge. This is the single most valuable output of the whole stage. When it fails, it will hand you the minimal case, preserve it as a regression test.

4. **Volume and reproducibility.** Run a large number of cases (start at the proptest default, then raise the case count for a longer run). Proptest seeds its RNG and records failing seeds in a regression file; commit that file so failures are reproducible.

Also add a small number of **hand-written unit cases** alongside the property test, covering the cases you most expect to be tricky: two concurrent conflicting threshold changes, a grant concurrent with a removal of the same member, an add and remove of the same member with no causal order. These document intent and catch regressions even if the generator's distribution shifts.

Stage 1 acceptance: the property test runs green over a large case count against the fold (real or stub, stated), the shrinking path is exercised (write one deliberately-broken fold variant in a test to confirm shrinking produces a minimal counterexample, then remove it), and the regression file is committed.

## Stage 2: interleaving and gap-fill (specify, then build after Stage 1)

This stage moves from "complete set, any order" to "partial sets that get completed", which is the realistic case and the one that tests gap detectability. Build it as a deterministic simulation, not a network test.

The model:

- N node-models, each holding a subset of the fact stream. Deliver facts to different nodes in different orders, with some facts delivered late (a gap that later fills) and some never delivered to a given node (a permanent partition for that node).

- Nodes reconcile pairwise (model the history-convergence exchange: exchange what each holds, request and transfer the difference), then fold.

The two assertions:

- **Convergence.** Any two nodes that have exchanged the same set of facts hold the identical folded state.

- **Gap detection (the assertion that matters most).** A node that is missing a fact must be able to *detect* that it has a gap, rather than folding its incomplete set and presenting the result as complete and final. Inject a specific gap, then assert the node reports "I am missing something" rather than producing a confident wrong state. This is the assertion that directly tests the design's open items (the dataplane checkpoint and completeness-ahead corroboration). A silent confident wrong fold under a gap is the failure that would reintroduce a central referee, so this is the highest-value check in Stage 2.

Use a single-threaded, seed-driven simulator so every run is reproducible from its seed: all delivery order, all gaps, and all timing are decided by a seeded RNG, nothing by real concurrency. Run many seeds. The reference points for this technique are deterministic simulation testing as used in the Rust ecosystem (the `turmoil` crate for networked models) and the FoundationDB and TigerBeetle style of seeded single-threaded simulation; you do not need those exact libraries, but build in that style.

Do not start Stage 2 until Stage 1 is green and committed.

## Stage 3: adversarial and bounded-exhaustive schedules (specify only)

Do not build this yet. It is recorded so the staging is legible.

- Make the simulator's scheduler adversarial: instead of random delivery orders, search for an order (plus gaps and equivocations, a node emitting two conflicting facts to different partitions) that maximizes divergence between honest nodes. Assert honest nodes still converge once they hold the same facts, and that equivocation is detected (both conflicting facts retained and surfaced as a fork, not one silently overwriting the other).

- For small models (3 to 5 nodes, small fact counts), add exhaustive model checking rather than sampling, using a Rust model checker such as `stateright`, or a TLA+ model checked with TLC. Exhaustive checking of the bounded case gives a much stronger statement ("no interleaving in this bounded space diverges") than sampling can.

## Honest scope of a passing result (put this in the README)

State this plainly in the harness README so a green run is not overclaimed:

- Property-based testing samples the input space; it does not exhaust it. Bounded model checking (Stage 3) exhausts only the bounded case. So all stages passing is strong evidence, not a theorem.
- A green run against a stub proves the harness and the reference semantics, not the production fold. Only a green run against the real fold speaks to the protocol.
- The property being tested is convergence (order-independence and gap detectability). Passing does not establish any other property (not liveness, not performance, not Byzantine fault tolerance beyond the specific equivocation-detection check in Stage 3).

## Deliverables and layout

- A Rust test crate (or a `tests/` module in the existing workspace) containing: the fold wiring or the clearly-labeled reference stub; the `proptest` generator and the permutation-invariance property; the hand-written unit cases; the committed proptest regression file; and a README stating what was tested (real fold or stub), how to run it, the case counts used, and the honest-scope paragraph above.
- If Stage 2 is built, the seed-driven simulator and its two assertions, in the same crate, with its own seed-corpus committed.
- A short RESULTS file: which fold was tested (real path or stub), how many cases ran, what passed, and any open semantic questions the generator surfaced (especially any spec ambiguity you had to flag rather than resolve).

## The one rule above all

If the fold you test is a stub, say so in the first line of every result. If you had to resolve a semantic ambiguity the spec did not answer, do not resolve it silently, list it as an open question in RESULTS, because such an ambiguity may itself be the order-dependence this experiment is meant to catch.
