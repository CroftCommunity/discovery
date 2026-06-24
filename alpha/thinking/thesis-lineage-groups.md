# Lineage Groups: A Technical Thesis and Validation Plan

author: ISaT / Product Security

date: 2026-06-13

status: draft thesis + experiment plan for Claude Code execution

repo path: experiments/lineage-groups/

---

## 1. Thesis

Messaging infrastructure models a group as a flat, eternal container: a fixed room, a single linear scroll, one mutable member list. Human groups are not like that. They are fluid and interconnected. They fork, splinter, go dormant, revive, and sometimes recombine. Today's tools fight that reality, so users absorb the friction as normal: a new group every time composition shifts, no relationship preserved between "vacation 2024" and "vacation 2025," no way to express that this splinter came from that group.

The thesis is a single claim with two payoffs that turn out to be the same property:

**Model a group as a navigable lineage of conversations rather than a flat eternal room, ground that lineage in cryptographic provenance, and you get a system that is simultaneously more secure and more faithful to how people actually socialize — because "knowing reliably where a conversation branched from" is at once the security invariant and the social-legibility invariant.**

The supporting sub-claims, each of which the experiments below are designed to confirm or falsify:

1. **Common ancestry is the safety primitive.** Reliable knowledge of where a group branched from is what lets the system reason about standing, legitimacy of a recombine, and whether two conversations are even related. Lose fork provenance and you lose the ability to say anything safe about the relationship between two groups.

2. **Forward key convergence and history reconciliation are different problems and must not be conflated.** The MLS epoch ratchet is single and linear and refuses to merge, by cryptographic necessity. Message history is just data and never needs to merge into one canonical transcript. Interleaving two forked branches by timestamp produces noise ("six tapes playing in a room"), so we do not do it.

3. **Reconciliation is consensual backfill within a lineage, not an automatic system merge.** A member who holds a branch may gift it to another member of the same lineage; the recipient chooses to absorb it. The privacy boundary is "were you ever party to a group on this lineage," which is exactly what shared genesis proves.

4. **Membership governance is a separate signed chain from the key ratchet, with per-operation thresholds fixed immutably at genesis.** This grounds the "who decides who decides" regress at the root and stops the turtles.

5. **Under partition, contradictory-but-valid membership commits are unavoidable; the correct resolution is a clean, attributable, non-insulting fork — promoted from failure mode to feature.** Heal silently when there is no conflict; hard-stop and escalate to a human on genuine membership conflict; never try to algorithmically adjudicate a social dispute.

### 1.1 The two-tree model

There are two distinct data structures, and the entire design lives in the binding between them.

**The governance tree.** A forward-only, signed log of membership operations (add, remove, leave, dissolve, fork, recombine), evaluated against immutable genesis rules. CRDT-friendly. Fork-detecting. Attributable. This is where "did this operation meet its threshold" is decided. It can fork and heal gracefully because it is just signed data with a known ancestor.

**The MLS epoch chain.** The actual key ratchet (openmls, RFC 9420). Single, linear, per-epoch rekey on membership change. It cannot merge. Ever.

The binding rule: **a governance event is only "enacted" once it has been realized as an MLS commit.** Under partition you can have a validly-signed governance op that is not yet enacted on one side. The open engineering seam — and the primary thing the experiments exist to de-risk — is what happens at reconnect.

### 1.2 Reconnect semantics (the survivor model)

Merge in the literal sense is impossible: two keys cannot occupy the same space. So every "merge" is really *one side adopts a surviving epoch, or both adopt a fresh third genesis*. Concretely:

- **Pick a survivor epoch deterministically.** Both sides must compute the same winner with no negotiation (e.g. ordered by member count, then by genesis hash as tiebreak — the exact rule is an experiment parameter, the requirement is determinism).

- **Re-key the losing side's members into the survivor** via MLS external commits (each joining member) or a batched re-add. Members keep their identity; only their live key changes.

- **History does not merge.** Each side already holds its own decrypted messages locally. The forward key converging says nothing about reconciling the past. Folded-away branches remain navigable per-person (the "code-fold" model: still there, out of the way, unfoldable on demand).

- **Check governance ops for contradiction.** If no conflict (e.g. transient partition, nobody disagreed about membership): heal silently. If conflict (one side booted someone the other still includes): **hard-stop, escalate to a human**, optionally gated by a quorum to accept a merge that re-admits a removed member.

- **Rejected merge is a resting state, not an error.** The two groups simply remain two groups, each with intact lineage. The "fresh genesis / third key" path exists only when both sides *want* to merge but neither existing epoch is acceptable: mint a new genesis, everyone joins clean, both prior logs are inherited as read-only ancestry. This is the "sixteenth-great-grandparent" case and it costs nothing.

### 1.3 Why provenance is both the security and the UX primitive

The same key-lineage machinery required for security — provable genesis, signed membership ops, a verifiable epoch chain — is exactly the structure needed to represent fluid, branching social reality faithfully. Provenance was the security requirement. It turns out provenance is also what makes the social model legible: foldable histories that are *real* (cryptographically guaranteed unaltered) and unfoldable *legitimately* (lineage proves you have standing). Not ironic by accident: "knowing where things came from" is one fact with two payoffs.

### 1.4 Scope and honesty boundaries

- This is **outside pure MLS** for the history and governance layers. MLS handles only the forward key. History is Automerge. Governance is a separate signed log. We are explicit that we are composing MLS with two non-MLS structures, not extending MLS.

- We do **not** attempt to algorithmically resolve social disputes. Conflict escalates to humans. Any cleverness here ("exclude the members who voted to boot") is a losing battle against social complexity and is explicitly out of scope.

- **Known-unverified at thesis time:** the precise openmls API surface for (a) bringing an external group's members into a surviving epoch and (b) reinit/fresh-genesis with inherited ancestry. External-commit support is confirmed present in openmls (external commit builder, stabilized through 2025); the *graceful re-key-the-other-side* flow built on top of it is the thing Phase 1 must prove, not assume.

---

## 2. Architecture under test

```
+-----------------------------------------------------------+
|  Client (per device)                                      |
|                                                           |
|  Identity:   DID + per-device signing key                 |
|  Governance: signed forward-only op log (verified vs.     |
|              immutable genesis rules)                      |
|  History:    Automerge document(s), one per branch        |
|  Keys:       openmls MlsGroup (the live epoch ratchet)    |
|  Lineage:    DAG of genesis-anchored group nodes          |
+-----------------------------------------------------------+
            |                         |
   sim transport (Phase 1-2)   real iroh (Phase 3 spike)
            |                         |
+-----------------------------------------------------------+
|  Superpeer broker (optional, always-on when present)      |
|  - cryptographically blind: ciphertext + routing only     |
|  - rendezvous, queue, snapshot store                      |
|  - can carry revocation / rekey commits when human peers  |
|    are not co-present                                      |
+-----------------------------------------------------------+
```

Crate layout (workspace inside `experiments/lineage-groups/`):

- `lineage-core` — the lineage DAG, genesis rules, governance op types and threshold evaluation. No crypto, no transport. Pure logic, exhaustively testable.

- `lineage-mls` — thin wrapper over `openmls`: create group, add/remove, external commit join, reinit. Isolates every openmls assumption in one place so a wrong assumption fails one crate, not the thesis.

- `lineage-history` — Automerge wrapper: per-branch documents, append message, fold/unfold, backfill import with provenance verification.

- `lineage-sim` — in-process transport + partition simulator. Models N devices, controllable network partitions, message reordering, drop. This is where logic tests live.

- `lineage-iroh` — Phase 3 only. Real iroh endpoints, one integration spike. Not on the critical path for the thesis logic.

- `xtask` / test harness — scenario runner that scripts genesis → fork → recombine sequences and asserts invariants.

---

## 3. Invariants (the assertions every experiment checks)

These are the falsifiable core. If any cannot be made to hold, that part of the thesis is wrong and we want to know cheaply.

**I1 — Genesis immutability.** Threshold rules set at genesis cannot be altered by any later governance op. Any op attempting to change them is rejected by every honest client deterministically.

**I2 — Threshold soundness.** An add/remove/dissolve op is "valid" iff it carries signatures meeting the genesis threshold for that op type from members with standing in the current epoch. No under-threshold op is ever enacted.

**I3 — Provenance / standing.** For any branch B and any actor A, the system can decide from signed data alone whether A had standing on B's lineage, without trusting any party's assertion.

**I4 — Forward-key linearity.** At no point do two live epochs claim to be "the group going forward." After any reconnect, exactly one survivor epoch is live; all other members are either re-keyed into it or are in a separately-acknowledged branch.

**I5 — Deterministic survivor selection.** Given the same two reconnecting branch states, every honest client independently computes the same survivor. No negotiation round required for the no-conflict case.

**I6 — No silent membership contradiction.** If a reconnect would place a removed member back into a group that removed them, the system never does so silently. It either hard-stops to human escalation or records a quorum-approved override.

**I7 — History never corrupts.** No reconcile/backfill operation mutates or reorders an existing branch's messages. Backfill only *adds* a separately-navigable branch to the recipient's local store.

**I8 — Backfill verifiability.** A backfilled branch is accepted only if its messages verify against their authors' signatures and chain to a genesis shared with the recipient's lineage. Unverifiable history is rejected, not taken on faith.

**I9 — Fold/unfold is lossless and inert.** Folding a branch hides it from the daily view without deleting it; unfolding restores full context. A folded branch generates no ambient notifications or pressure.

**I10 — Convergence.** After a partition heals with no conflict, all participating honest clients reach the same lineage DAG and the same live epoch.

---

## 4. Phased experiment plan

Three phases, sequenced. Each phase has an explicit go/no-go gate; a failed gate is a thesis finding, not just a bug.

### Phase 0 — Scaffold (half a day)

**Goal.** Workspace, CI, the invariant-assertion harness, and a deterministic RNG/clock so every scenario is reproducible.

**Build.**

- Cargo workspace with the six crates above (iroh crate stubbed).

- A `Scenario` harness: declares devices, scripts a sequence of ops and partition events, runs to quiescence, asserts a list of invariants.

- Deterministic logical clock (Lamport or hybrid logical clock) and seeded RNG so survivor selection and merge order are reproducible.

**Gate.** A trivial scenario (one device, genesis, one message) runs green under the harness and is bit-reproducible across runs.

### Phase 1 — Crypto/protocol feasibility (the riskiest, do it first despite "all three")

**Goal.** Prove the openmls operations the whole reconnect model depends on actually exist and compose: create group, add/remove with epoch rekey, **external-commit join**, and **reinit to a fresh genesis**. Prove the survivor-epoch re-key is expressible.

**Build (in `lineage-mls`).**

- Wrap openmls group creation tied to a DID-bearing credential.

- Implement add/remove producing commits + welcomes; verify epoch advances and removed members can no longer derive new secrets (I4 component).

- Implement external-commit join: a member of branch B joins branch A's surviving epoch without a prior welcome. This is the survivor re-key primitive.

- Implement reinit/fresh-genesis: mint a new group, add all members from two prior branches, verify both prior epochs are now dead.

**Experiments / assertions.**

- E1.1: removed member cannot decrypt post-removal traffic (PCS holds). Asserts I4.

- E1.2: external commit brings a B-member into A's epoch; both compute identical group secrets afterward. Asserts the survivor primitive.

- E1.3: reinit produces a clean third epoch; the two parents are unusable for new messages. Asserts the fresh-genesis path.

- E1.4: revocation under no-co-present-peers — a queued remove commit applied later still rekeys correctly. Asserts the broker-carries-revocation claim.

**Gate (go/no-go for the whole thesis).** If external commit + reinit cannot be made to express "pick survivor, re-key the other side, or mint a third" with PCS intact, the reconnect model does not work on openmls and we must either change crypto libraries or change the thesis. **This gate is the single most important result in the plan.** Verify against the real library, not docs.

### Phase 2 — Data model + merge semantics (the conceptual core)

**Goal.** Prove the two-tree model: governance fork/heal, deterministic survivor selection, conflict detection with hard-stop, history-as-navigable-tree, and consensual backfill with provenance.

**Build (in `lineage-core` + `lineage-history`, driven by `lineage-sim`).**

- Governance op log: signed ops, threshold evaluation against immutable genesis (I1, I2).

- Lineage DAG: genesis-anchored nodes, fork edges, recombine edges. Standing queries (I3).

- Survivor selection function: deterministic, parameterized by rule, seeded (I5).

- Conflict detector: identifies removed-then-readmitted contradictions; routes to silent-heal vs. hard-stop (I6).

- Automerge per-branch history: append, fold/unfold, no cross-branch interleave (I7, I9).

- Backfill: export a branch, import on another lineage member with full signature + genesis-chain verification; reject unverifiable (I8).

**Experiments / assertions.**

- E2.1: under-threshold remove is rejected by all honest clients. I2.

- E2.2: genesis rules survive a malicious op claiming to change them. I1.

- E2.3: partition → both sides make non-conflicting ops → heal → identical DAG + epoch. I10, I5.

- E2.4: partition → contradictory remove/keep → reconnect → hard-stop fires, no silent re-admit. I6.

- E2.5: rejected conflict-merge leaves two valid groups, each with intact lineage and standing. (Resting-state, not error.)

- E2.6: forced fresh-genesis merge inherits both logs as read-only ancestry; no message reordered. I7.

- E2.7: backfill of a branch the recipient was entitled to (shared genesis) verifies and imports; a forged branch (broken signature or foreign genesis) is rejected. I8, I3.

- E2.8: timestamp-interleave is *not* used — assert that reconcile produces distinct navigable branches, not a merged scroll. (Guards against regressing into the "six tapes" mistake.)

**Gate.** All of I1–I3, I5–I10 hold in simulation across a fuzzed set of partition/op orderings. If deterministic survivor selection or conflict hard-stop cannot be made to hold under adversarial reordering, the reconcile model is unsound and needs rework before any transport work.

### Phase 3 — End-to-end thin slice over real iroh (one integration spike)

**Goal.** Show genesis → fork → recombine actually working across real processes over real iroh, with an optional blind broker. Not productionizing — proving the logic survives a real transport.

**Build (in `lineage-iroh`).**

- iroh endpoints per device; gossip topic per group (mirroring the Delta Chat pattern: random topic id, lazy P2P bootstrap).

- A minimal broker process: blind relay + queue + snapshot store. Carries commits when peers are not co-present (exercises E1.4 for real).

- Wire the Phase 1 MLS and Phase 2 governance/history through the real transport.

**Experiments / assertions.**

- E3.1: three devices, two of one user; genesis a group; partition by killing connectivity; each side sends; reconnect; assert I10 + I4 over the wire.

- E3.2: induce a conflict reconnect; assert hard-stop surfaces to the (simulated) human and no silent re-admit. I6.

- E3.3: device-loss + recovery — a device drops, a new device for the same DID joins via external commit carrying a broker-held snapshot; assert it reaches the live epoch and can decrypt forward only. (Probes the recovery open question.)

- E3.4: broker is blind — instrument the broker and assert it observes only ciphertext + routing metadata, never plaintext or group membership semantics. (Scopes the blind-broker claim honestly; note IP/timing is still observable, consistent with the field.)

**Gate.** A scripted genesis→fork→recombine demo runs across real processes with invariants intact. Honest negative result also acceptable: documenting exactly where real transport breaks an assumption that held in sim is itself a valuable Phase 3 output.

---

## 5. Open questions the experiments are explicitly probing

These are carried forward from the design discussion as first-class unknowns, each mapped to where it gets tested.

- **Survivor-epoch re-key feasibility on openmls** → Phase 1 gate (E1.2, E1.3). The make-or-break.

- **Total-device-loss recovery** → E3.3. We have a design (DID + external commit + broker snapshot) and no proof. This is flagged as the largest residual risk; the thesis does not claim recovery is solved.

- **Deterministic survivor selection under adversarial reordering** → E2.3, E2.4. If two honest clients can be made to disagree on the survivor, I5 fails and the no-negotiation heal is impossible.

- **Backfill provenance without having witnessed the epochs** → E2.7. Validating an epoch chain you were not present for; tractable in principle (signatures + genesis anchor) but unproven in this composition.

- **Blind-broker claim scope** → E3.4. Confirm blindness to content/membership; document honestly that network-layer traffic analysis (IP, timing, volume) remains observable, same residual as Signal's sealed sender.

- **Recursive admin-set forking** → grounded by I1 (genesis immutability). The admin/quorum set is anchored at genesis and is more stable than the membership it governs; E2.2 tests that the anchor holds.

---

## 6. Working instructions for Claude Code

Paste-ready brief for executing this plan agentically. Adjust paths if the workspace already differs.

### 6.1 Operating constraints

- Language: Rust, stable toolchain. Workspace under `experiments/lineage-groups/`.

- Dependencies: `openmls` (latest stable, v0.7-era external-commit-builder API), `automerge`, `ed25519-dalek` (or `openmls`-bundled signature keys), `iroh` + `iroh-gossip` (Phase 3 only). Pin exact versions in a committed lockfile; do not use floating ranges.

- Determinism is mandatory: seeded RNG and a logical clock injected everywhere; no `SystemTime::now()` or unseeded `rand` in logic crates.

- Every experiment (E*) is an integration test under the scenario harness, named to match (e.g. `e1_2_external_commit_survivor`).

- No network in Phases 0–2. `lineage-iroh` must not be a dependency of `lineage-core`, `lineage-mls`, `lineage-history`, or `lineage-sim`.

### 6.2 Build order (do not reorder — Phase 1 is the risk gate)

1. **Phase 0 scaffold.** Workspace, six crates (iroh stubbed), `Scenario` harness, deterministic clock + RNG. Prove the trivial scenario green and reproducible.

2. **Phase 1 first, in full, before any data-model work.** Implement `lineage-mls` and run E1.1–E1.4. **Stop and report at the Phase 1 gate.** If external commit + reinit cannot express survivor-re-key with PCS intact, do not proceed to Phase 2 — surface the finding, propose options (alternative library, thesis change), and wait.

3. **Phase 2.** Implement `lineage-core` + `lineage-history`, run E2.1–E2.8 under fuzzed partition/op orderings. Report at the Phase 2 gate.

4. **Phase 3.** Implement `lineage-iroh` + minimal broker, run E3.1–E3.4. A documented negative result is an acceptable deliverable.

### 6.3 Reporting format per phase

For each phase, produce a short `PHASE_N_FINDINGS.md` containing: which invariants held, which failed and why, any openmls/automerge API assumption that turned out false, and a go/no-go recommendation for the next phase. Keep prose tight; tables for invariant pass/fail.

### 6.4 Guardrails

- When an openmls or automerge call does not behave as the thesis assumes, **do not paper over it** — isolate it, write a failing test that documents the real behavior, and report it as a finding. A wrong assumption caught in Phase 1 is the plan working as intended.

- Do not implement social-conflict auto-resolution. Conflict paths terminate in an escalation hook (a callback / event), not an algorithm that picks winners.

- Do not interleave branch histories by timestamp anywhere. E2.8 exists to catch this regression.

- Keep the broker blind: it must compile against ciphertext + routing types only, with no access to plaintext or decrypted membership. If a feature seems to need broker visibility into content, that is a finding to report, not a license to widen its access.

---

## 7. What a successful validation would and would not establish

**Would establish.** That the two-tree model is implementable on real libraries; that survivor-epoch reconnect with PCS is expressible on openmls; that governance fork/heal with deterministic survivor selection and conflict hard-stop is sound under adversarial reordering; that history-as-navigable-tree with verifiable consensual backfill works; and that an optional blind broker can carry revocation/rekey and recovery snapshots.

**Would not establish.** Production readiness, scale behavior at large group/branch counts, real-world UX of the fold/unfold and conflict-escalation flows, network-layer metadata resistance, or an audited security posture. Those are explicitly beyond this validation and should not be claimed from it.

The single most important outcome is the Phase 1 gate: a verified yes/no on whether openmls can express "pick a survivor epoch and re-key the other side, or mint a third" with post-compromise security intact. Everything else in the thesis is conditional on that answer, and getting it cheaply and early is the whole point of sequencing Phase 1 first.
