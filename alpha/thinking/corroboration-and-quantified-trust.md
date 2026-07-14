# Corroboration and quantified trust — the stamp beacon, the read side, and the beam contract

`A companion to reconciliation-horizon.md and the-shape-of-disagreement.md (captured 2026-07-14). Those
notes captured how a protocol built to not answer "who won" still lets a member see a bounded picture;
this note captures the outcome of a separate 2026-07-14 pass on what final state actually costs a node
that can never see past its own edge. The concrete, committed outcomes are three: the Part 2 §7.3.3
corroboration-dials paragraph and the §7.4.1 formula-valued freshness threshold (the two Design
paragraphs, RUN-04 T2/T3), and backlog item EXP-C1. Everything else here is exploratory, kept in the
original design voice; the self-echo idea in §4 in particular is not promoted to spec.`

---

## 0. The epistemic floor

§5.1 is the floor and nothing below it moves: everything a local authority holds about the world outside
itself is comparative or asserted, never canonical. The only canonical state is local. A node cannot know
that it has seen the latest governance fact, because "latest" presumes a global vantage no node has and
would have to rest on the wall-clock Part 1 §2.0.1 rules out. This is not a gap awaiting a better sensor.
It is the shape of the problem.

So all corroboration is quantified trust. External information arrives at a trust level the node itself
sets the terms for, never as proof. The node's job is therefore not to *know* the edge but to be
configured with the terms under which corroboration counts as sufficient: the thresholds (how many
distinct lineages, §7.4), and the act classes those thresholds gate. Sufficiency is a setting, not a
discovery.

## 1. The write side is already built

The read/write asymmetry is the thing the pass turned on. On write, the local authority *does* know the
last event it asserted on top of, because every outgoing entry already references it. The §7.4.3
governance-generation stamp makes this legible: each data-plane entry self-locates against the authority
chain by carrying the generation the author held when authoring it. So every ordinary message doubles as
a passive currency beacon, the "I'm good, you?" of the content plane, emitted for free by traffic that was
going to flow anyway. R6's attributable acceptance (§7.5.1) is the enforcement-side twin: a participant
accepting a write records the causal frontier of governance facts it had synced at acceptance, so a
later-synced revocation makes the stale acceptance detectable and attributable rather than silent. One
records what you asserted on top of; the other records what you had synced when you accepted.

What this closes is exactly the behind-via-traffic case. A node several generations behind that receives a
stamped entry reads a nameable gap of known size, not a current view, and must converge the missing
governance before enforcing against the entry. The case is converted from undetectable to a sized, named
gap. What it does not close is the tail no entry has yet depended on, which is §2.

## 2. The read side: solicitation and the unreferenced tail

A governance fact that nothing has yet stamped cannot be beaconed, by definition: the passive currency
signal rides on references, and an unreferenced fact emits none. The stamp closes detection *through
traffic* and leaves exactly the unreferenced tail (§7.4.3, Appendix B). Only an active frontier ask
reaches it, solicitation: a node asking peers directly for their frontiers, the read side of the same
coin whose write side is the stamp.

The answer arrives as assertion at quantified trust, never proof, because it is subject to the §5.1 floor
like everything else external. This gives the honest reading of an empty solicitation: absence of evidence
is only ever *corroborated absence*, never established absence. A node that asks and hears nothing has
learned that its reachable peers stamped nothing, not that nothing exists. That is the strongest true
statement available, and the protocol makes no stronger one.

## 3. Formula-valued thresholds

The freshness k need not be a constant. A Group MAY set it as a formula over folded state, f(folded member
count, folded roles) evaluated at the act's position, proportional to how many members the Group had then,
or weighted by folded Group Roles. This is sound iff every input to the formula is itself folded fact,
never an asserted or locally observed quantity. Under that condition k is deterministic and computes
identically on every honest node exactly as a constant would, because the fold is order-independent and
the inputs are the same folded facts everywhere.

A formula-valued threshold changes under R7 like any other rule and introduces no new trust surface: the
inputs were already folded, the evaluation is already deterministic. It moves the dial, not the machinery.
The reason to want it is that a flat constant is the wrong tool for a Group whose size swings by an order
of magnitude over its life; ceil(n/2) over the live member set tracks the Group instead of freezing a
number chosen when it was small.

## 4. Circular assertion awareness (exploratory)

Here is an idea the pass surfaced and deliberately did not promote. Your own stamp, echoed back inside
someone else's later entry, is delivery corroboration without receipts: passive evidence that your assertion
propagated, arriving free on traffic you were receiving anyway. It is tempting to lean on. Two seams keep
it honest, stated hard because both are load-bearing.

- **Positive-evidence-only.** An echo means your fact reached at least one other node. *No* echo means
  nothing at all. Silence and partition are indistinguishable at the node, so absence of echo can never be
  read as absence of propagation. Only the positive case carries information.
- **Liveness-signal-only.** The moment an echo influences ordering or authority it becomes a covert clock,
  a forgeable input smuggled onto the ordering spine, which Part 1 §2.0.1 forbids outright. An echo is
  admissible strictly as a liveness read: *this got somewhere*, never *this got somewhere first* and never
  *this therefore outranks that*.

Held to those two seams it is a genuine liveness signal and no more. For anyone else, your echo may mean
nothing, and that is fine; corroboration is quantified trust, and one node's propagation evidence is not
another's obligation. This is why it stays in the note and not the spec.

## 5. The beam, reframed

Part 2 already states the posture plainly, so this is a restatement rather than a new claim: a sufficiently
isolated node cannot establish final state on its own, because it cannot distinguish "no later fact exists"
from "I cannot currently reach anyone who would attest one." Final state is established only by
corroboration reaching the node, never by a lone node's self-certification. Enforcement therefore fails
closed under isolation, stalling the irreversible act while reads and content-plane liveness continue on
best-known state. Delay over breach.

Read that way, the completeness-ahead beam is not a mechanism awaiting invention. What remains of it, once
the stamp closes the behind-via-traffic case, is a family of Group-governed dials plus one intrinsic,
honest limit:

- **which act classes need final state** versus best-known (irreversible enforcement always final, reads
  never, the boundary per-Group);
- **the k, or the formula** that computes it (§3);
- **the solicitation posture**, how actively the node reaches for the unreferenced tail (§2).

The fail-closed rule itself is *not* a dial: it holds at every setting, which is what makes the whole dial
family safe to expose. A tight Group dials k high and enforcement slow; a loose Group dials it low and
accepts more exposure to delay. Both are safe. Earning the beam, then, means demonstrating the *contract*,
that the dials behave and the fail-closed gate holds, not eliminating the intrinsic limit. The limit is
stated, not closed, because it is the shape of §0 and cannot be closed.

## 6. The contract experiment

EXP-C1 (backlog §2c) is the demonstration side, loopback-runnable now with no new infrastructure, four
RED-able assertions: stall-at-threshold (delay over breach), stamp detection (the behind-via-traffic case
end to end), solicitation reach (the unreferenced tail surfaced by a frontier ask and folded identically
to normal arrival), and formula-valued k (ceil(n/2) over the folded member set computed identically across
arrival orders). It shares boundary machinery with EXP-H1 and discharges, at loopback grade, part of
§8.2(e)'s residual that the freshness precondition on originating such an op is not yet exercised over live
transport.
