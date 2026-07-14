# Experiment Suite: Verifying the Lineage-Based Group Model

author: experiment specification for a Claude Code session

date: 2026-06-14

## Purpose

Demonstrate that the technical substrate is cohesive and trustable: that ancestry holds, complementary divergence converges, contradictory divergence is detected and never auto-resolved, the trap door is always available, and social decisions sit on top as inputs rather than leaking into the mechanism. Each experiment scripts a social condition (the input), runs the mechanism, and asserts the provable outcome (the check). The provenance facts that a real UI would surface are made explicit and testable here.

## Scope decision (read first)

This is a simulation harness, not a production stack. The decision and its consequences:

- Ancestry is real. Use actual SHA-256 hash-linking and a real Merkle DAG. This is cheap and it is the core thing under test, so do not fake it.

- MLS is modeled, not real. Represent epochs, commits, and "enacted" state as plain objects with the same ordering and fork invariants RFC 9420 imposes (single linear epoch chain per branch, no merge across a fork). Do not pull in openmls for v1. Model the one property that matters: a commit binds to exactly one parent epoch, and two commits binding to the same parent epoch is a fork.

- Transport is modeled. Partition, delay, and reconnect are functions that control which peers can see which messages. No real networking.

- Social decisions are scripted inputs. Votes, removals, dial settings, and "which branch a human follows" are explicit test fixtures, never computed by the mechanism. This is the point: the experiments must show the mechanism never makes a social choice.

What this proves: the architecture's logic is internally cohesive and its invariants hold across the edge cases and at modest scale. What it does not prove: that real MLS key schedules, real forward-secrecy deletion timing, and real P2P transport behave as modeled. Section "Deferred to real-stack validation" lists which experiments must be re-run against openmls and a real transport before being fully trusted.

## Tech and structure

- Language: TypeScript or Python, implementer's choice. TypeScript is suggested because the DAG and assertion ergonomics are clean and it matches a likely eventual client.

- Layout suggestion:

  - `core/` the mechanism: DAG, ancestry, convergence, fork detection, epoch model, trap door.

  - `social/` scripted decision inputs: vote scripts, dial configs, follow-choices.

  - `harness/` transport model (partition/reconnect), peer model, scenario runner.

  - `experiments/` one file per experiment below, each a scenario plus assertions.

  - `report/` a runner that executes all experiments and emits a single pass/fail table plus a provenance trace per experiment.

- Every experiment emits two things: a machine-checked assertion result, and a human-readable provenance trace (the thing a UI would later render). The trace is itself a deliverable, because "demonstrable" means a person can read the flow.

## Core invariants every experiment must be able to assert

These are the trust properties. Build them as reusable assertion helpers.

- INV-ANCESTRY: any two nodes have a well-defined set of common ancestors, and the lowest common ancestor is computable and stable regardless of which peer computes it.

- INV-HASH-INTEGRITY: a node's hash covers its parent links and payload; tampering with any ancestor invalidates every descendant hash.

- INV-CONVERGE-COMPLEMENTARY: two branches whose operations do not contradict converge to a single state that is independent of merge order (order-independence is the commutativity check).

- INV-DETECT-CONTRADICTION: two branches that make opposing claims about the same membership fact are flagged as a contradiction, with both claims and their provenance preserved.

- INV-NO-AUTO-RESOLVE: the mechanism never picks a winner for a contradiction. The resolution must come from a scripted social input.

- INV-TRAPDOOR: from any state, including a stuck contradiction, a fork-to-new-history operation succeeds and yields a usable, searchable branch.

- INV-IMMUTABLE-ADMIN: admin-tier operations are append-only; no experiment can mutate or reorder a committed admin op without INV-HASH-INTEGRITY failing.

- INV-LINEAGE-NOT-LEAF: quorum and threshold checks count distinct lineages, never device leaves. A scenario adding N devices to one lineage must not change any threshold outcome.

- INV-VIEW-LOCAL-FIRST: two peers may hold different current views during a partition without either being "wrong"; both views must be renderable and searchable.

- INV-MEMBERSHIP-FRESH: an originate/co-sign of a membership op is admitted only from a strict CURRENT state (caught-up AND corroborated-fresh — agreement on the same head from ≥k distinct lineages, not a single beacon); ordinary content has no such gate; applying a received op is never blocked by an emit-time freshness bar (only by epoch-chain validity + the contradiction hard-stop).

- INV-THRESHOLD-SATISFIABLE: a threshold k_op is never in force above the eligible-lineage count at its effective epoch (≤ founding roster at genesis; a later raise only when n ≥ new_k); k never auto-tracks n downward.

- INV-FLOOR-NO-BRICK: no op may produce a post-state with fewer than k_op eligible lineages for an in-force capability; removal at the floor is replace-only; a legitimate within-policy quorum (including self-capture) is accepted, never structurally vetoed — capture is exited by re-formation fork, not blocked.

- INV-ROLE-REVOCABLE: every delegated role, the creator's included, is revocable — routinely under k_policy, always by unanimity of the non-holders, ultimately by the fork; no grant configuration can make a role irrevocable.

---

## Experiment group A: ancestry and integrity (foundation)

**A1. Common ancestor is stable across peers.**

Social condition: none, this is pure substrate. Build a DAG with branching. Have three independent "peers" each compute the lowest common ancestor of the same two nodes from their own partial views.

Assert: INV-ANCESTRY. All three compute the same LCA. The LCA is correct against a brute-force reference.

Provenance trace: the ancestry path each peer walked.

**A2. Tamper detection.**

Social condition: a malicious actor edits a historical admin op (e.g. rewrites a past removal to look like it never happened).

Assert: INV-HASH-INTEGRITY. Every descendant hash mismatches; the tamper is localized to the exact node; honest peers reject the rewritten history.

Provenance trace: the first node where the hash chain breaks.

**A3. Append-only enforcement.**

Social condition: an admin attempts to delete or reorder a committed op.

Assert: INV-IMMUTABLE-ADMIN. The operation is rejected at the mechanism level; only append succeeds.

---

## Experiment group B: complementary convergence (the easy, must-work cases)

**B1. Multi-device self-sync.**

Social condition: one user, one lineage, three devices. Each device sends non-conflicting messages and one device adds a fourth device. Devices are offline from each other, then reconnect pairwise in varying orders.

Assert: INV-CONVERGE-COMPLEMENTARY and INV-LINEAGE-NOT-LEAF. All devices converge to the same state. The convergence is identical regardless of reconnect order. Adding the fourth device did not alter any group-level threshold count (still one lineage).

Provenance trace: the shared ancestor each pair found, and the converged head.

**B2. Clean partition heal.**

Social condition: a five-lineage group, partitioned by a network split. Both sides keep chatting and make only non-contradictory admin changes (e.g. side 1 adds a member, side 2 changes a non-conflicting dial). Reconnect.

Assert: INV-CONVERGE-COMPLEMENTARY. The branches merge to one state; the added member and the dial change both survive; result is merge-order-independent.

**B3. Order-independence stress (commutativity).**

Social condition: generate many non-conflicting concurrent ops across branches, then merge them in randomized orders across many trials.

Assert: every merge order yields the identical final state hash. This is the empirical commutativity proof for the complementary class.

Modest scale: 10 lineages, 30 devices, 500 concurrent non-conflicting ops, 100 randomized merge orders.

---

## Experiment group C: contradiction (the hard-stop, the heart of the trust claim)

**C1. The canonical hard-stop: ejected-and-re-added.**

Social condition: five-lineage group, partitioned. On branch X, a scripted quorum ejects member M. On branch Y (which cannot see X), M is kept and posts normally; a scripted op on Y re-affirms M. Reconnect.

Assert: INV-DETECT-CONTRADICTION and INV-NO-AUTO-RESOLVE. The merge halts at a flagged contradiction. Both claims (M out, M in) are preserved with full provenance: who voted, on which branch, descending from which shared ancestor. The mechanism produces no winner. The state remains two valid branches until a scripted social input chooses.

Provenance trace: the exact shared ancestor, the diverging ops, the attribution of each. This is the trace a UI must surface for a human to adjudicate.

**C2. Social resolution as input.**

Social condition: take the halted state from C1. Provide a scripted social decision (e.g. "the group follows branch X; M is out") as an explicit input.

Assert: the mechanism now converges to the chosen branch, and the *losing* branch remains preserved and attributable (not deleted), available as folded history. The decision is recorded as a new admin op with its own provenance.

This experiment demonstrates the boundary directly: nothing converged until a human input arrived, and when it did, the mechanism executed it faithfully without having made it.

**C3. Re-formation backstop.**

Social condition: M (ejected on X) forms a new group consisting of the shared ancestor minus the ejecting parties. Some members follow M; others stay.

Assert: the re-formation is a legible branch off the shared ancestor; its provenance proves common lineage; both the original and the re-formed group are valid resting states; no member is forced into either.

Provenance trace: the ancestry proof linking the re-formed group to the original root.

**C4. Contradiction must not be silently commutative.**

Social condition: attempt to feed a contradiction through the complementary-merge path (simulate a bug or an attacker trying to get an auto-merge of an in/out conflict).

Assert: INV-NO-AUTO-RESOLVE holds even under adversarial input; the contradiction path cannot be bypassed; any attempt to auto-resolve is rejected. This is the negative test that proves the boundary is enforced, not just observed.

---

## Experiment group D: the trap door (the guaranteed floor)

**D1. Fork from a stuck contradiction.**

Social condition: from the unresolved C1 state, a party invokes the trap door (fork and start new history).

Assert: INV-TRAPDOOR. The fork succeeds from a genuinely stuck state; the new branch is usable; the old history is preserved and searchable; ancestry to the shared root is retained (related fork).

**D2. Unrelated fork.**

Social condition: a party starts a fresh group with no ancestor relationship.

Assert: INV-TRAPDOOR. Succeeds; the new group is independent; no false ancestry is claimed.

**D3. Usable and searchable divergent history.**

Social condition: a peer holds a divergent, multi-branch history (post-C1, pre-resolution).

Assert: INV-VIEW-LOCAL-FIRST. The local-first view renders a palatable current view; full-text search returns correct results across folded branches; the worst-case state is still a working, searchable chat.

This is the experiment that distinguishes the design from SSB: divergent state is livable, not broken.

---

## Experiment group E: governance dials and capture (social posture on top)

**E1. Dial tuning changes outcomes, mechanism unchanged.**

Social condition: run the same C1 ejection scenario under three dial settings: inclusion-priority (low removal threshold), balanced, and fidelity-priority (high removal threshold).

Assert: the threshold outcomes differ as the dials dictate, but the underlying mechanism (detect, preserve, escalate) is byte-for-byte identical across all three. Posture is an input; mechanism is invariant.

**E2. Genesis-fixed vs runtime dials.**

Social condition: attempt to change a genesis-fixed dial at runtime; attempt to change a runtime dial.

Assert: the genesis-fixed change is rejected with provenance pointing to genesis; the runtime change succeeds and is recorded as an admin op.

**E3. Captured-quorum entrenchment (adversarial governance).**

Social condition: a malicious majority turns the removal dial to entrench itself and eject a minority lineage.

Assert: the mechanism executes the (valid under the dials) ejection without objection, AND the minority's re-formation backstop (C3) remains available, AND the entire capture is fully attributable in provenance. This demonstrates the honest claim: crypto does not prevent the social bad outcome, but it guarantees legibility and a clean exit.

Provenance trace: the dial change, the quorum that made it, the ejection, the available re-formation.

---

## Experiment group F: roll-ups and two-mode operation (scaling and the superpeer)

**F1. Roll-up correctness (settled history compaction).**

Social condition: a long-running group with much settled, un-forked history. Mint a checkpoint.

Assert: a client that accepts the checkpoint computes the same current membership as a client that replays the full chain. The checkpoint covers only settled, un-forked history (attempting to checkpoint across an open fork is rejected).

**F2. Checkpoint trust: threshold-signed, not authority-signed.**

Social condition: mint a checkpoint two ways: (a) signed by a single superpeer, (b) threshold-signed by a quorum of lineages.

Assert: the harness flags (a) as an authority-trust dependency (the leak) and (b) as decentralized-valid. Two clients can verify (b) without a shared trusted signer; (a) requires trusting the signer. This makes the referee-leak visible as a test result.

**F3. Two-mode convergence equivalence.**

Social condition: run an identical scenario twice: once superpeer-assisted (sweeping done opportunistically), once pure-P2P (sweeping via threshold ceremony).

Assert: both modes reach the same final governance state. The superpeer mode is faster (fewer rounds) but reaches no outcome unreachable in pure-P2P mode. This is the conformance test in code: assert no outcome exists in mode A absent from mode B.

**F4. Mode-toggle path warmth.**

Social condition: run superpeer-assisted for a long horizon (superpeer does all sweeping), then remove the superpeer and require a pure-P2P checkpoint.

Assert: the pure-P2P checkpoint ceremony still succeeds (the path was kept warm). Include a negative variant where the path was never exercised and show it failing or stalling, to demonstrate why scheduled warming matters.

**F5. Availability-as-rights-escalation probe.**

Social condition: make the superpeer the only reliably-online peer, then require a governance action.

Assert: the action is still possible without the superpeer (peers can complete it when brought online), proving the right was not escrowed to the superpeer's presence. The negative variant: show that if the action *required* the superpeer to be present, the test flags a rights-escalation leak.

---

## Experiment group G: modest-scale soak (the sleeper risk)

**G1. Long-horizon device churn.**

Social condition: 30 lineages, each cycling devices (add/remove) repeatedly over a long simulated time, several partitions and heals, a handful of contradictions resolved socially, periodic roll-ups.

Assert: with roll-ups enabled, member-list rendering cost stays bounded (verification from last checkpoint, not full replay). Without roll-ups, show the cost growing unbounded (reproduce the sleeper risk on purpose, to prove the roll-up is what fixes it).

Metrics to emit: governance-log length over time, per-render verification cost with and without roll-ups, time-to-converge after each heal.

**G2. The month-eighteen scenario.**

Social condition: the specific failure narrative from the analysis: 30-person group, every member swapped devices twice, four partitions, a newcomer joins and must render the member list.

Assert: the newcomer computes the correct member list from a checkpoint plus live tail; the result matches the full-replay reference; rendering stays within a modest cost bound.

This is the experiment that proves the sleeper risk is actually mitigated, not just theoretically addressed.

---

## Experiment group H: membership freshness (no admin act on a stale view)

These exercise INV-MEMBERSHIP-FRESH. They specify the MEMBERSHIP-FRESH decision (2026-06-17, `freshness-signal.md` / CROFT §9). Model-first; H5/H6/H7 require a real transport (see "Deferred to real-stack").

**H1. Cold rejoin, clean — corroboration before acting.**

Social condition: a node off past the tier horizon while the group advanced content only (no membership change); it rejoins and syncs.

Assert: INV-MEMBERSHIP-FRESH. It reaches CURRENT, and may originate a membership op only after ≥k-distinct-lineage stable agreement on the head — not after the first single beacon.

Provenance trace: the beacons heard, the lineages corroborating, and the head they agree on.

**H2. Cold rejoin, missed membership change.**

Social condition: while the node slept, member E was removed (epoch advanced).

Assert: it converges to the new epoch, rejects sending to E, and retains its own pre-sleep history (no claw-back).

**H3. Stale co-sign rejected post-rejoin.**

Social condition: the node emits a co-sign composed against its pre-sleep epoch.

Assert: receivers reject it (pre-image names a stale epoch) — the epoch-validation gate, exercised on the rejoin path.

**H4. Single-beacon insufficiency (the load-bearing case).**

Social condition: a rejoined node syncs the chain, hears exactly one beacon, and immediately attempts to originate a membership op.

Assert: INV-MEMBERSHIP-FRESH. It MUST NOT originate on one beacon; corroboration (≥k lineages, or the co-sign gather for k>1) is required. This is the test that justifies condition (b).

**H5. Fresh-but-wrong partition → hard-stop, not a freshness fix.**

Social condition: a node rejoins a partition {A,B} that holds k admin lineages but is behind the true tip; A+B act in-partition; later the partitions heal.

Assert: INV-DETECT-CONTRADICTION / INV-NO-AUTO-RESOLVE. The §7 hard-stop fires on heal; freshness did not (and is not expected to) prevent the in-partition act. This is the honest residual: freshness narrows, the hard-stop closes.

**H6. Withholding attack is fail-safe.**

Social condition: an adversary starves the node of beacons.

Assert: the node stays UNVERIFIED → cannot originate a membership op (fail-safe); content stays readable, labeled behind/unverified.

**H7. Stale-beacon flood is fail-safe.**

Social condition: an adversary floods valid-signed but old-head beacons.

Assert: stale beacons never advance best-seen nor manufacture a false CURRENT; the node either holds CURRENT against the real tip or detects BEHIND against a real future beacon — in no case does the flood enable a bad admin op.

**H8. Partial chain sync.**

Social condition: the node fetches N→N+1 but a real N+2 (advertised by a beacon) is unreachable.

Assert: it is BEHIND (best-seen ahead) → cannot originate a membership op; the catch-up path is unaffected; content readable-labeled.

---

## Experiment group I: the admin floor (threshold-satisfiability + never-irrevocable roles)

These exercise INV-THRESHOLD-SATISFIABLE, INV-FLOOR-NO-BRICK, and INV-ROLE-REVOCABLE. They specify the ADMIN FLOOR decision (2026-06-17, `revocation-authority.md` / CROFT §6). Model-first.

**I1. Satisfiability at establishment (solo ⇒ k=1).**

Social condition: create a group solo and attempt to set k_add = 5; separately, create solo with k_add = 1.

Assert: INV-THRESHOLD-SATISFIABLE. The k=5 solo genesis is rejected as unsatisfiable; the k=1 solo group is valid and the founder can add.

**I2. Genesis roster, born matured.**

Social condition: genesis declares a 10-member roster with k_add = 5.

Assert: the group is matured at epoch 0, k=5 binds immediately, the floor n≥5 is in force, and no provisional/ramp state exists.

**I3. Raise-k valid only when n ≥ new_k.**

Social condition: a k_add=1 group grows to 6, then a policy op raises k_add to 5; a second op attempts to raise k_add to 9.

Assert: the raise to 5 (n=6 ≥ 5) succeeds and is recorded under the current (lower) policy; the raise to 9 (n=6 < 9) is rejected as self-bricking.

**I4. Floor reject + replace-not-remove.**

Social condition: a matured k=5 group at exactly 5 members; (a) a bare remove of one member; (b) an atomic remove+add preserving 5.

Assert: INV-FLOOR-NO-BRICK. (a) is structurally rejected regardless of valid signatures; (b) succeeds.

**I5. No threshold-downgrade by attrition/shrink.**

Social condition: a matured k=5 group; an attempt to remove members to lower the effective add-bar, then admit a slate under the lowered bar.

Assert: the effective k_add never drops below the matured 5; the shrink-below-5 op is rejected (k does not auto-track n downward).

**I6. Legitimate quorum accepted (capture ≠ brick).**

Social condition: 2-of-3 admins A,B,C; A+B remove C; separately A+B raise then strip to a 1-of-1 they control.

Assert: INV-FLOOR-NO-BRICK boundary. Each op is accepted (within policy, floor satisfied, group stays governable); the out-voted minority's §7 re-formation fork remains available and the whole sequence is attributable (mirrors E3 capture). The floor does not over-reach into legitimate decisions.

**I7. Roles are never irrevocable — the anti-entrenchment ladder (creator/admin).**

Social condition: a creator holds the bootstrap admin role; (a) the role is revoked routinely under k_policy; (b) the role's grant is (mis)configured to gate k_policy itself, and the non-holders revoke it by unanimity; (c) a 2-person group where the single non-creator votes the role away.

Assert: INV-ROLE-REVOCABLE. All three succeed; no grant configuration makes the role unremovable; (c) leaves two equals or ends the group. Disclosure-on-join is present in the provenance.

**I8. The ladder generalizes to meer/geer — strip + detach with history preserved.**

Social condition: a group running an elected geer (or meer) operated by a co-op / external authority; all other members vote (non-holder unanimity) to strip the role.

Assert: INV-ROLE-REVOCABLE. The role is stripped; the group **detaches** from the operator and becomes a differently-shaped group (an outcome that could not have been prevented anyway — the operator's leaving is always possible). History and provenance up to the detachment are preserved and remain navigable; no content the geer/meer held is retroactively legitimized or erased. Cross-ref §6 materially-reversible, §6.1 geer, §8.1 meer.

---

## The report

The runner produces:

- A pass/fail table across all experiments, grouped A through G.

- For each experiment, the human-readable provenance trace (the UI-bound facts), so a reader can follow the flow without reading code.

- A summary mapping each core invariant (INV-*) to the experiments that exercise it, so coverage is visible.

- An explicit "social inputs used" list per experiment, making it auditable that every social decision was an input and never a mechanism output.

The last item is the most important for the trust claim. The suite's headline result is: every convergence was mechanical and order-independent, every contradiction was escalated and never auto-resolved, and every social decision entered as a labeled input. If the report shows that cleanly, the technical side is demonstrably cohesive and the social boundary is demonstrably respected.

---

## Deferred to real-stack validation

These experiments model MLS and transport, so the following must be re-run against real components before being fully trusted, and should be called out as such in any writeup:

- Forward-secrecy timing: the model does not delete key material on a real schedule, so it cannot show the FS loss from out-of-order commit retention (the DMLS tradeoff). Re-run F-group against openmls to measure real FS cost.

- Real fork mechanics in MLS: the model treats "two commits on one epoch" as a fork by fiat. openmls will have its own behavior at that boundary; verify the model matches.

- Transport-level partition realism: real iroh/P2P partitions have timing and partial-delivery behavior the model abstracts away. Re-run B and C groups over a real transport to confirm convergence and contradiction-detection survive realistic message loss and reordering.

- Threshold-signature and accumulator crypto: F2 models the trust distinction; a real implementation must use actual threshold signatures or a real accumulator (Merkle Mountain Range or recursive proof) and re-verify F1 and F2.

- Freshness/liveness over a real transport: H5 (fresh-but-wrong partition → reconnect hard-stop), H6 (beacon withholding), and H7 (stale-beacon flood) depend on real partition/timing and beacon delivery. Specify model-first, then re-run over live iroh-gossip and bind to the green-real faithful path (C-faithful-revoke, C-mls-welcome) before trusting the freshness gate end-to-end.

Stating these openly is part of the trust claim: the simulation proves the logic is cohesive; it does not stand in for cryptographic validation of the real primitives.
