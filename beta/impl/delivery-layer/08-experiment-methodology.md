# How to run experiments for Drystone

`Status: methodology (binding for validation work)`

`Purpose: define what counts as a valid Drystone experiment, so a result can be trusted as evidence about the real system`

`Companion to: 05-Drystone-Delivery-Experiments.md (the plan), 01-delivery-architecture.md (the claims)`

---

## 1. Why this doc exists

The first delivery-layer validation round produced 13 useful results, but two of them (E3.3 entitlement, E3.4 lineage admission) used **stand-in cryptography** (an XOR cipher in place of the MLS seal, a hash-chain in place of credential validation). Those experiments validated the *logic* of the design but did **not** exercise the real library, so their verdicts had to be down-marked to "confirmed in model form" and re-listed as open residue.

That is an acceptable outcome only if it is *labeled*. The danger is a stand-in result being read later as if it validated the real mechanism. This doc sets the rules that prevent a stand-in from masquerading as a real test, and that make every result self-describing about what it actually exercised.

The governing principle, taken from the project's grounding discipline: **accuracy before fluency.** A test that looks like it passed but tested the wrong thing is worse than no test, because it retires a question that is still open.

---

## 2. The fidelity ladder

Every experiment sits on one of three rungs. The rung MUST be stated in the result, in the verdict line, not buried in prose.

### Rung A: real-library

The experiment exercises the actual production library for the property under test. An MLS claim runs against `mls-rs` and its real `encrypt_application_message` / `process_incoming_message` / credential-validation hooks. An iroh claim runs against real `iroh` / `iroh-gossip` endpoints. A claim about a primitive runs against the primitive's real implementation.

Only a Rung-A result may retire a `[confirm]` item about the real mechanism. Verdict tag: `CONFIRMED (real-lib)` or `FALSIFIED (real-lib)`.

### Rung B: model-form

The experiment exercises the *logic* of the design with a stand-in for some component (a placeholder cipher, a simulated transport, a simplified construction). It validates that the design's reasoning is internally sound, but says nothing about whether the real library behaves the same way.

A Rung-B result MUST be tagged and MUST name what was stood in for. It may support a design claim but may NOT retire a `[confirm]` about the real mechanism; it generates a follow-up Rung-A item instead. Verdict tag: `CONFIRMED (model-form: <what was stood in>)`.

Example, the right way to label E3.3: `CONFIRMED (model-form: XOR cipher stands in for the MLS seal; entitlement logic validated, mls-rs seal not exercised)`.

### Rung C: spec-check / static

The experiment is a guard or an inspection: asserting a payload stays under a size limit, reading an enum's variants, confirming an API shape. No runtime behavior of the system is exercised beyond the assertion itself.

Useful for guards and for settling API-shape questions (E1.1's static half was Rung C and was decisive because the question *was* an API-shape question). Verdict tag: `CONFIRMED (static)` or `FALSIFIED (static)`.

---

## 3. Hard rules

These are not style preferences; a result that violates one is not admissible as evidence about the real system.

- **Never substitute a stand-in for the exact component a claim is about.** An MLS claim may not use XOR, a fixed key, ROT13, or any placeholder in place of the MLS seal. A signature claim may not use a hash where a real signature is required. If the claim is about component X, component X must be real, or the rung is B and the verdict says so. XOR-as-MLS is the canonical forbidden move.

- **State the rung in the verdict line.** Not in a footnote, not implied. `CONFIRMED (real-lib)`, `CONFIRMED (model-form: ...)`, or `CONFIRMED (static)`. A bare `CONFIRMED` is not an acceptable verdict.

- **Pin and print exact versions.** Every result prints the resolved versions of every library it touched (e.g. `iroh 1.0.1, iroh-gossip 0.101.0, mls-rs 0.55.2`). Reproducibility against a known state is mandatory because crate versions move (iroh-gossip moved twice during design).

- **Do not assert an API shape from memory.** Begin each experiment by reading the then-current crate docs or running `cargo doc` / inspecting the actual types. The plan's own results were nearly wrong about the gossip event surface until the real enum was read.

- **A FALSIFIED result is a first-class success.** It is not a failure of the experiment; it is the experiment working. Record it loudly, state the design consequence, and rewrite the affected branch. (E1.1 is the model: falsified, branch reshaped, documented.)

- **Stand-in results generate follow-up real-lib items.** A Rung-B `CONFIRMED` does not close the question; it opens a tracked Rung-A item to re-run against the real library. The question stays open in the residue until a Rung-A result lands.

- **Separate "the logic holds" from "the library does this."** These are different claims. Model-form tests the first. Only real-lib tests the second. The verdict must make clear which was tested, because the design depends on both and they fail independently.

---

## 4. Per-domain fidelity requirements

What "real-lib" concretely means for each component the delivery layer touches, so there is no ambiguity about when a stand-in has crept in.

- **MLS / content sealing.** Real `mls-rs`. Group creation, member add via real key packages and Welcome, `encrypt_application_message`, `process_incoming_message`. A claim about confidentiality, dedup of sealed bytes, or epoch behavior is Rung A only if the bytes are real MLS PrivateMessage bytes. (E2.1 was correctly Rung A here; E3.3 was Rung B and must be re-run.)

- **Credential / lineage validation.** Real `mls-rs` credential-validation hook (the `CustomProposalRules` / identity-provider surface named in E3.4). A hash-chain stand-in is Rung B. The real test admits a lineage-bearing credential and rejects a non-lineage one *through the library's own validation path*, not a hand-rolled predicate.

- **iroh transport / gossip.** Real `iroh` and `iroh-gossip` endpoints, real `Router`, real `subscribe_and_join`. API-shape questions (what events exist) may be settled Rung C by reading the types, but behavioral claims (does an offline node recover messages) need real endpoints (E1.2 did this correctly).

- **RBSR / set reconciliation.** For a *scaling-shape* claim, a faithful model of the algorithm is acceptable as Rung B and should be labeled (E3.1 used an XOR-fingerprint bucketing model, which validates the shape, not the production construction). A claim that *the chosen production construction* (Willow 3d-range or Negentropy) behaves a certain way is Rung A only against that construction's real implementation.

- **Push / APNs / FCM.** The architectural property (wake-then-fetch removability) is testable Rung A in-process by suppressing the wake and confirming recovery (E4.1 did this). The *platform throttle numbers* cannot be tested without real device credentials and are explicitly a manual, observational, non-CI procedure, never a pass/fail in the suite.

---

## 5. Result format

Each experiment's result records, in order:

- The claim under test and the doc section it backs.

- The exact resolved library versions.

- The fidelity rung, with the stand-in named if Rung B.

- The code (or a reference to it in the results document).

- The raw output.

- A one-line verdict: `CONFIRMED (rung)` / `FALSIFIED (rung)` / `INCONCLUSIVE`, with the measured evidence.

- The design consequence, and for any Rung-B result, the follow-up Rung-A item it generates.

Every verdict feeds back into the design doc's `[confirm]` residue: a Rung-A CONFIRMED promotes an item out; a FALSIFIED rewrites the branch; a Rung-B CONFIRMED leaves the real-mechanism item open and adds the follow-up.

---

## 6. Standing follow-up items from round 1

**Status update: the two model-form debts below were RETIRED in round 2 at real-lib fidelity (see `10-experiments-round2.md` R0.1/R0.2 and `06-Drystone-Delivery-Experiment-Results-R2.md`).** Recorded here for the audit trail of how the fidelity ladder was applied: round-1 model-form results generated explicit Rung-A follow-ups, and those follow-ups were run and passed.

- **E3.3 (entitlement), retired by R0.1:** re-run at Rung A against real `mls-rs`. Result: a non-member given another member's Welcome cannot join (HPKE binds the Welcome to a specific key package; `join_group` errors). The XOR stand-in is gone; the boundary is shown to be cryptographic, not a policy check.

- **E3.4 (lineage admission), retired by R0.2:** re-run at Rung A against the real `mls_rs_core::identity::IdentityProvider` hook. Result: a non-lineage credential is rejected at commit-build time. The hash-chain predicate is gone; the real library validation path enforces the policy.

- **E3.1 (RBSR scaling)** remains the Rung-B shape validation it was; a Rung-A confirmation still waits on the Part 2 §5 choice of production construction (Willow vs Negentropy), at which point it is re-run against that implementation. This is the one outstanding ladder item.

This is the fidelity ladder working as designed: model-form results were tagged, generated Rung-A follow-ups, and did not retire the real-mechanism questions until the Rung-A runs landed.
