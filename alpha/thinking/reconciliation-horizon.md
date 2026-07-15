# The reconciliation horizon — cadenced projection over a continuous fold

`A companion to the-shape-of-disagreement.md (captured 2026-07-14). That note asked how a protocol
built to *not answer* "who won" can still let a member see a bounded, up-to-date picture; this note
captures the answer that landed. The concrete, committed outcomes are two: the Part 2 §7.6 continuity
passage and the §7.6.9 horizon-cadence worked example (the two Design paragraphs, RUN-03 A1/A2), and
backlog item EXP-H1. Everything else here is exploratory — kept in the original design voice, not
promoted to spec.`

---

## 0. Motivation

Contradiction detection (Layer 1) is cheap and continuous: the fold freezes every concurrent
conflict into a descriptor the instant it sees it. Projection (Layer 2) — a member re-running its own
resolution policy over the fork DAG to render *its* view of history — is not cheap, and its cost is
unbounded in the wrong way: re-evaluating a projection over ever-longer history grows without bound,
so a member who never truncates re-reads the whole world each time it wants a current picture.

The naive fix is a clock: "re-project every hour." But there is no wall-clock in this system, and a
clock is a trusted assertion — one member's "now" is not another's. The release valve has to be the
*closest-to-consensus fact available with no trusted assertions*: a boundary every member derives
identically from the shared log. That is the whole design problem, and it has a clean answer, because
the system already carries exactly one totally ordered stream (the epoch chain) and one monotone
per-member counter (facts folded). A boundary built from those two is objective by construction.

## 1. The split: detection is continuous, evaluation is cadenced

The two layers run on different clocks on purpose.

- **Layer 1 (detection) is continuous.** Every concurrent conflict freezes into a descriptor the
  moment the fold sees it — the mutual-expulsion / role-thrash / competing-RuleChange predicate
  family. No cadence gates this: an open contradiction is open the instant it exists.
- **Layer 2 (evaluation) is cadenced.** At a horizon, a persona re-runs its resolution policy, but
  only against the forks that arrived *since the last horizon*. Three outcomes, and only the last is
  presented:
  - **no match against your world** — the arrived fork touches nothing your policy has an opinion on;
    nothing changes (the as-if-under-your-feet case — the world moved but not your part of it);
  - **a match your policy resolves** — it projects silently, folding into your rendered view without
    a surface;
  - **`escalate`, or no match to any rule** — this is what gets *presented*, and it is presented at
    bounded age, because the window it was evaluated in is bounded by the horizon.

The win is that presentation staleness is bounded (you never re-derive the whole DAG to get a current
picture) while resolution is never forced (nothing is decided by the cadence).

## 2. The cadence rule

A horizon fires on the disjunction of two log-derivable terms:

> **epoch-roll OR N-facts.** Fire on every epoch commit; additionally fire whenever N facts have
> accumulated since the last horizon. The counter resets at each boundary. N is per-group,
> R7-governed like any rule.

Why both terms:

- **Epoch commit** is the one serialized, totally ordered event stream in the system. Every member
  locates an epoch boundary identically, with nothing asserted — it is the strongest objective
  boundary available.
- **N-facts** covers the socially quiet Group where no epoch rolls for a long time but governance
  facts still accumulate; without it, backlog in such a Group would grow unbounded between epochs.
  The since-boundary locator is the §7.4 governance-generation stamp: "how many facts since the last
  horizon" is read straight off it, no clock.

Both terms are log-derivable, so no member has to trust another's boundary. The counter reset at each
boundary is what keeps the two terms from drifting relative to each other.

## 3. The horizon checkpoint

A horizon checkpoint is the §7.3.3 self-checkpoint **extended with a manifest**:

> `manifest = (frontier head, sorted set of open contradiction byte-heads)`

It is entirely objective — the frontier head is a hash, and the byte-heads are the same
order-independent min-hashes the fold already emits (`contradiction:{byte-head}`). There is no policy
in it and no rendered view in it. That is what makes it co-signable under unchanged §7.3.3 semantics:
a co-signature is corroboration that an independent member folded the identical set and saw the
identical open-contradiction landscape — never a trusted summary of one member's opinion. Two members
co-signing a horizon checkpoint have agreed on *the shape of their disagreement* (which contradictions
are open at this frontier) without either revealing how it intends to resolve any of them.

> **Frontier-head pinning note (EXP-H1, RUN-07 → the §7.6.9 pinning pass).** "The frontier head is a
> hash" understates one choice the release encoding must make. EXP-H1 (RUN-07) found that the *naive*
> last-ingested head is **arrival-order-dependent** — the raw `computed_at_gov_head` is whichever hash
> the member most recently folded, so two members converging on the same state can carry different
> last-ingested heads. So the experiment's manifest instead leads with an **order-independent digest of
> the converged state** (gov_seq + rules + sorted members), which is byte-identical across arrival
> orders by construction. When `[gates-release]` pins the §7.6.9 manifest encoding, it must decide
> between that converged-state digest and the **sorted set of DAG tip hashes** (closer to §7.3.3's
> governance-head commitment, at the cost of being a set rather than a single digest). Both are
> order-independent; they differ in what they commit to. Decision deferred to the pinning pass — this
> note only records that the naive single last-ingested head is not a valid choice.

## 4. The projection checkpoint

Distinct from the horizon checkpoint (objective, shared) is the **projection checkpoint**: a member's
own *rendered* state cached as of the horizon frontier. This is a §4.6 derived view — rebuildable and
non-authoritative — so it is an accelerator, never a source of truth. Evaluation thereafter is
incremental within the window (project only the since-boundary forks against the cached view).

The one case that forces a full replay is a member **editing its own resolution policy**: a new rule
can re-decide history all the way back, so the cached projection is invalid and the member re-projects
from the genesis floor. This is rare, always user-initiated, and always possible (the genesis floor
guarantees a member can always rebuild its own view from scratch). Checkpoints accelerate; they never
become a thing you cannot get behind.

## 5. Decay, not expiry

An open descriptor is Layer-1 truth and **never expires**. Expiry-by-timeout would be a verdict by
timeout — the machine deciding a contradiction is resolved because nobody looked, which is exactly the
utility verdict the protocol refuses to manufacture. Horizons bound *presentation staleness*, never
*resolution*.

What horizons *may* do is **decay presentation**: an untouched fork — one no persona has built on
across some number of horizons — can age out of the working set onto an archive shelf, so the live
view stays legible. The descriptor is unchanged and the pressure gauge (how many open contradictions,
how old) stays accurate; only the render foregrounds the live and shelves the dormant. Aging is a
presentation dial, not a state change.

## 6. Temperature without profiling

The shape doc's floor holds: resolution policies stay **private per node** (publishing them invites
profiling and gaming). But one thing is already public by construction — *which branch* a persona
builds its next fact on is right there in the DAG, because a fact points at its predecessor. So each
horizon yields, for free, the **observed convergence fraction per fork**: of the members who acted
since the last horizon, what fraction extended each branch. Call it the **coherence temperature** — a
Group-legible read on how aligned the room is on a given open contradiction, computed from public
structure alone.

The privacy line stays intact: temperature says *that* a persona chose a branch (public), never *the
rule that produced the choice* (private). The rule stays unguessable-with-certainty — the observed
choice is consistent with many policies.

## 7. First spike (EXP-H1)

The narrowest thing that earns the cadence and the manifest:

- Two members, one contradiction. **Mutual expulsion works today**, so the spike is runnable now
  against that predicate; it extends to competing-RuleChange the moment the Phase B predicate lands.
- Drive a horizon boundary in **both trigger modes**: once by an epoch roll, once by N-facts
  accumulating with no epoch roll.
- Each member emits its horizon manifest `(frontier head, sorted open byte-heads)`, and the assertion
  is **byte-identity of the manifest across members and across arrival orders**. Same open-contradiction
  landscape, same bytes, regardless of who folded what in which order.
- Competing-RuleChange joins the manifest as a second contradiction entry once the Phase B predicate
  exists, so EXP-H1 grows one assertion rather than changing shape.

This is the objective half of the whole design — no policy, no render — so it is the half that can be
pinned as a determinism test before any of the Layer-2 projection machinery is built.
