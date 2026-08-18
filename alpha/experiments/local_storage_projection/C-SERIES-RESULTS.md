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
