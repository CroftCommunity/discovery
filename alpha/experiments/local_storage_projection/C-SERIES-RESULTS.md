# C-series (fold side) — results: C2 behind-detection, C3 HeadAck, C5 ack cost

`Written 2026-08-17 (E112). The fold-side half of the C-series; the meer-side half (S23–S26, C4)`
`is in ../meer-queue/TEST-LOG.md. Plan: ../../plans/2026-08-17-1-plan-head-currency-and-admission-fact.md.`

**Home:** `local_storage_projection` — the governance fold (`fold_derived.rs`), the freshness
primitives (`completeness_ahead.rs`), and the two new modules `head_currency.rs` (C2) and
`head_ack.rs` (C3). Tests are inline (`src/tests_c2.rs`, `src/tests_c3.rs`), matching the crate's
convention (`tests_stage7`), because the fold harness (`Db::create_in_memory`, `traits::mocks`) is
`#[cfg(test)]`-gated.

**Fidelity: Modeled / loopback grade** throughout. Real redb fold, real signature *binding*
(via the `Signer`/`Verifier` traits — a deterministic mock scheme over a `compute_hash` digest, not
ed25519), a real wire crossing for HeadAck (serialize → re-parse). The transport for C3 is modeled
in-process; running the same acks over the FANOUT-M1 `IrohGossipBus` (which carries opaque frames
via `Transport::publish`/`drain`) is a **named, un-run mechanical upgrade**, not claimed here.

---

## C2 — behind-detection from traffic

`src/head_currency.rs` + `src/tests_c2.rs`. RED-first on arm 1 (per plan).

The §7.4 precondition — a member originating or co-signing a membership act must be
corroborated-fresh — wired to the real fold. The pure threshold arithmetic already lived in
`completeness_ahead.rs` (`quorum_k`, `admits_irreversible`, `detect_stamp_gap`) but was **not wired
into any origination path**. C2 wires it and measures the wiring.

### Arm 1 — a behind node refuses to originate or co-sign (RED-first)

RED step (watched fail): with the gate un-wired (a stub that admits unconditionally), a node that
had *already detected it was behind* was still admitted to originate. Then the fail-closed gate was
wired in.

**Verdict: `C2 arm 1 MEASURED (Modeled/loopback)`.** A **governance** fact whose antecedents named
a head the node had not folded made the real fold return `FoldError::MissingAntecedents`; the node
marked itself behind, `may_render_current()` went false, and the origination/co-sign gate
fail-closed refused with `Stalled::BehindViaTraffic` — *even with freshness seeded at k*, i.e. the
behind flag alone is sufficient to stall. Reads stayed available throughout; `note_caught_up()`
cleared the flag and re-admitted.

### Arm 2 — the quiet group cannot detect behind from ordinary traffic (expected-fail, kept)

**Verdict: `C2 arm 2 MEASURED (Modeled/loopback)`.** Under ordinary traffic (an applied governance
fact and an applied data-plane message), the behind-via-traffic signal **never fired** — because the
fold's completeness gate is **governance-only by the §2.0.1 razor** (data-plane facts are
optimistically accepted even with a dangling antecedent). So a genuinely-behind node believed itself
current. The *only* thing that kept it from originating was the fail-closed freshness leg (freshness
`0 < k`), and the only way to raise freshness is positive head-attestation. This is the Appendix-B
unreferenced-tail case, and it **proves the ack primitive (C3) is required, not assumed** — rather
than asserting it.

**Finding worth carrying to the merge prose:** behind-*detection*-from-traffic is a governance-only,
best-effort **negative** signal (silent when there is no governance traffic); safety rests on the
**positive** corroboration leg, which only HeadAcks can supply. The two legs are not
interchangeable.

---

## C3 — the HeadAck primitive

`src/head_ack.rs` + `src/tests_c3.rs`. Built module-alongside-tests (not plan-designated RED-first);
the forged-ack property is enforced at *compile time* by a typestate (`VerifiedHeadAck` is the only
thing `FreshnessTracker::record` accepts, and it is constructible only via `HeadAck::verify`).

`HeadAck { group_id, head, generation, signer_lineage, signer_device, sig }` as a §7.3.4
**sign-the-state** object: identity is the state attested; the signature binds a `compute_hash`
digest of `(tag, group_id, head, generation)` — **no wall-clock anywhere**, the horizon is a
`generation` (epoch counter).

### Arm 1 — request/response reaches freshness over a (modeled) wire

**Verdict: `C3 arm 1 MEASURED (Modeled/loopback)`.** k=3 distinct-lineage HeadAcks of the node's
head each crossed a wire (`to_bytes` → `from_bytes`), were verified, and unioned; freshness rose
0→3 and the §7.4 gate (`admits_membership_origination`, from C2) flipped stall→admit exactly at k.
This is the real freshness source that **replaces C2's seeded integer**.

### Arm 2 — the threshold is exactly k

**Verdict: `C3 arm 2 MEASURED (Modeled)`.** k-1 distinct-lineage vouchers stall
(`NotCorroboratedFresh { have: k-1, need: k }`); the k-th admits.

### Arm 3 — union counts distinct lineages, never clients (§5.7)

**Verdict: `C3 arm 3 MEASURED (Modeled)`.** Two distinct lineages attesting one head → freshness 2
(one head entry, two vouchers). **Two devices of one lineage → freshness 1**, not 2. A re-heard ack
unions to 1 (idempotent — the same state attested twice is one object, never a rival). §5.7 upheld:
corroboration counts personae, never clients.

### Arm 4 — adversarial: forged fails; unknown head is a gap, not authority

**Verdict: `C3 arm 4 MEASURED (Modeled)`.** A forged signature and a tampered head/generation both
fail verification with `BadSignature` and cannot be recorded (typestate). A **validly-signed** ack
attesting an *unknown* head at a newer generation is classified `DetectedGap { ahead: true }` and
contributes **zero** to freshness — §7.4.3 upheld: an ack is a **locator** ("converge before
trusting"), never an authorization input.

**Note (caught by the failing arm 4):** the mock signer only binds the first 32 bytes of a message,
so an early layout that concatenated the fields left `head`/`generation` unsigned. Fixed by signing
a `compute_hash` **digest** of the state — which is what a real scheme does anyway, and now binds
every field. A real ed25519 HeadAck would need no such care, but the digest form is the honest one.

---

## What the fold-side C-series does NOT discharge

- **Not the Appendix-B completeness beam.** C2/C3 exercise the *mechanism* of obligation (1) (the
  completeness predicate + its coordination) and give the partition/degraded behavior for (2)
  (fail-closed stall). The beam's *proof* statements remain open (Appendix B).
- **Not real transport.** Loopback/Modeled only. The FANOUT-M1 iroh bus upgrade for C3 arm 1 is
  named but un-run.
- **Not real crypto.** The `Signer`/`Verifier` mocks bind a digest deterministically; ed25519 is not
  exercised on this path (it is, elsewhere, in social-graph-core).

---

## C5 — ack cost honesty (informative)

`src/tests_c5.rs`. **Modeled / loopback grade** (message counts; the FANOUT-M1 harness measured real
gossip volume shape separately). Informative, not gating.

**Verdict: `C5 MEASURED (Modeled)`.** Per-op normative acks are `N·ops` (every member answers every
op). The scoped alternative — explicit acks only for finality-needing ops (membership/governance,
§7.4's scope), lazy piggyback (your next authored fact carries your head) otherwise — is
`N·finality_ops`. The saving is exactly the non-finality tail `N·(ops − finality)`. Over a curve
N ∈ {2,4,8,16,32} with ops=100, finality=5, the reduction is a constant **20× (= ops/finality),
independent of N**; both remain O(N) per solicited op.

**Conclusion (informative):** scoping acks to finality-needing ops is a volume lever, not a safety
one — safety is C2/C3's fail-closed freshness gate. It makes the per-op ack cost of the strict
posture affordable, which is the premise the strict-merge floor rests on (the plan's Reasoning:
"the strict-merge stall is only livable if corroboration is cheap, and whether it is cheap is C5's
number").

---

## P1 / E108 — CONTESTED as a first-class membership state (2026-08-22, E117 Phase 1)

**Scope:** canonical §7.3.2 as merged, plus the two review amendments (R4 direction, O9) and
the owner's 2026-08-21 resolution-authorization decision. **Rung: Modeled** (real fold over a
real store; governance facts are the experiment envelope, not a wire-final Drystone encoding;
signatures real Ed25519 on the croft-chat path, MockSigner in-crate).

**Built:** `ForkStatus::Contested(Vec<ContestedEntry>)` — the pair as data (ordered), the
membership-contested subjects, the replay-withheld facts; two simultaneously open
contradictions representable (the retired `Contradiction(min-hash)` single slot structurally
could not). `MembershipView { Member / NotMember / Contested(pairs) }` via total
`GroupState::membership()` — no boolean accessor. `AssertionType::Resolution (0x000C)`:
closes exactly one named open pair, charter-quorum-gated (`GroupRules.resolution_threshold`,
minted at default **2**, dialable by governed `RuleChange`; rides the V5′ Approval machinery
with the content-hash subject). Closing is not un-deciding: resolved pairs stay
replay-excluded, derived from the log itself. `GroupState` wire **v2** (5 thresholds +
contested entries; unknown versions refused loudly). **One shared transition**
(`compute_next_governance_state`) now serves live ingest AND the rebuild replay — the replay
previously ran no detection, so a rebuild of a contested store silently lost its hard-stop
(pre-existing divergence, closed). **O9:** envelope wire **v2** drops the signed wall-clock
field (Part 1 §2.0.1); standing layout pin + all three decoders refuse v1; timeline windows
and the compaction age gate become lamport/position-denominated.

**Pins (croft-chat/tests/contested_projection.rs, 5/5 green, RED-first — the four originals
recorded structurally RED at 2b7afed):** (1) mutual expulsion projects both subjects
CONTESTED in both arrival orders, byte-identically (head stamp normalized as a locator);
(2) two open contradictions carried simultaneously, each with its pair; (3) single-author
resolution refused at the default threshold; (4) quorum resolution closes exactly the named
pair, the other stays open, post-resolution state byte-identical across orders; (5) resolved
exclusions persist through later replays (added by survivor triage — the original fixture's
lamport-1 removes replayed before the adds and no-opped; reworked to post-add lamports).

**Suites:** substrate 102/0; croft-chat workspace 120/0 (both fresh on the rebased tree —
see the branch log for the exact runs).

**Mutation (bounded, X3-pattern cross-package — the croft-chat suite as killer; full
re-baseline rides P3 per the vet's R6):** 35 mutants scoped to the new functions
(`compute_next_governance_state`, `replay_excluding`, `resolved_excluded`,
`mutual_expulsion_entry`, `detect_authorized_contested`, `membership`, wire codec).
**30 killed** (29 by the sweep + recheck, 1 hand-run after a patch-apply failure —
committed-green first, restored via `git checkout HEAD --`). **5 survivors, all triaged:**
2 equivalent (`seq += → *=` in `replay_excluding` — the per-step seq feeds only intermediate
bookkeeping overwritten by the head stamp; `&& → ||` in `resolved_excluded` — within the
governance log only Resolution facts carry 64-byte payloads, so the OR-arm is unreachable);
4 pre-existing NodeCard `created_at`/`created_by` field-deletion survivors in
`upsert_node_*`, outside P1's functions, already documented in the X3 ledger. Results:
scratchpad `p1_mutation_results.json` / `p1_recheck2_results.json`; summary here is the
durable record.

**Also fixed (exposed by the new preimage, both pre-existing):** the slot-fork `ForkedFrom`
label was order-dependent with 3+ contenders (last-pairwise only; old green was hash-luck) —
now the max over all observed contenders, a pure function of the contender set
(`test_fork_convergence_at_scale` re-pins); `governance.rs`'s decoder copy silently
mis-parsed v2 (phantom 8-byte read) — third copy of the same decoder, consolidation noted
for P2's error-split work.

**Spec filings out of this phase → ROADMAP E133** (the §7.3.2 amendment set).

---

## P3 / E117 — real signatures on the authorship plane + the mutation re-baseline (2026-08-23)

**The C4 truth held:** P3 was relocation, not construction. `crypto.rs` moved from
social-graph-core into the core as `ports::ed25519` (deterministic construction only —
wasm-clean; SigningKey zeroizes on drop; behind a default feature so the lean arm proves the
fold needs no crypto crate). **Authorship evidence now runs on real Ed25519 end to end:** the
five core pins sign every cast fact and verify against the author device's key before
`evaluate` sees it (core suite 35/0 incl. the O1 fixture); the C-series arms swap their
stand-ins out — C2's delegating MultiVerifier dies for the stateless real verifier
(registrations bind DERIVED device ids, never seeds), C3's HeadAck signs/verifies through the
real port (adapter suite 82/0). **O1's portable slice landed:** the conformance crate's
EMITTED signing vectors are croft CI fixtures, verified through the core port (good accepts,
tampered rejects) — the harness now exists for the fold-vector categories at the
`[gates-release]` pin.

**Per-plane rung, restated:** authorship (signatures over canonical bytes; HeadAck
sign-the-state) — **real Ed25519**, no stand-ins in the evidence artifacts; governance-fold
projection — Modeled (real fold, experiment-grade encodings); transport — loopback where
exercised, per the standing honesty line. Remaining MockSigner usage sits in
storage-plumbing tests (stage7/surface/governance) where the mock is fixture convenience,
not the claim.

**Mutation re-baseline (R6) — the new baseline ledger, croft tip `ea2ce71`:** full-crate
cargo-mutants on social-tree-core: 629 mutants, 21m — **168 caught in-crate, 63 unviable,
398 in-crate survivors** (update 149 · model 109 · wire 84 · project 49 · ports 7). The
in-crate-only scope is stated deliberately: the crate's strong killers live corpus-side
(adapter + croft-chat), reachable via MUTATION.md's `[patch]` recipe; the update.rs
P1-scope functions already carry cross-package verdicts (30/35 killed, survivors triaged,
this file §P1). The 398 register is the standing burn-down for corpus-side sweeps at phase
closes — a periodic audit, not a gate, per the house rule.
