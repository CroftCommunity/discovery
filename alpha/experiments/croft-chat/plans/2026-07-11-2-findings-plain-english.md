# What the experiments found, in plain English

`Companion to 2026-07-11-1-plan-next-experiments.md. This is the prose version: what
each experiment was trying to break, what happened, and what it means for Drystone
and for coordination-free protocols in general. It assumes no Rust.`

## The bet these experiments test

Drystone's central wager is **governance without a coordinator**: independent peers
each fold the same facts and arrive at the same answer about who may act and what the
rules are, with nobody in charge and no server to appeal to. The only remedy for a bad
outcome is to leave (the fork), so the fold underneath has to be trustworthy on its
own — there is no admin to correct it in place.

That bet rests on three quieter assumptions the design leaned on but had never stress-
tested against the running code:

1. the fold gives the **same answer regardless of the order** facts arrive in;
2. it does so **only over a complete set** of facts — so the system must behave safely
   when a peer is missing something;
3. when the machine genuinely **cannot** decide, it stops and hands the problem to
   humans — in *every* shape that can happen, not just the obvious one.

We wrote each experiment as an attempt to **break** one of these, with a concrete
condition that would count as a failure. That framing is deliberate: a claim earns
confidence by surviving honest attempts to falsify it, never by a demo that confirms
what we already hoped.

## The headline finding: convergence silently assumes complete delivery

**Experiment G1.** Two honest peers, same group. One of them misses a single
governance fact — specifically a fact that a *later* fact causally depends on, where the
two facts were authored on different devices. We then let both peers settle and compared
their view of the group.

**What happened:** the peer that missed the fact did **not** notice. It folded straight
past the gap, admitted the dependent fact anyway, and settled on a **different** set of
group rules than the complete peer — and **both reported themselves clean**. No fork
flag, no "you're missing something" signal. A silent disagreement, dressed up as
agreement.

The sharp part is *why*. Every message in the system already lists the facts it causally
follows (its "antecedents"). That is exactly the breadcrumb a peer would need to notice
"I'm being asked to accept something that depends on a fact I've never seen." **The
information is on the wire; the fold just never looks at it.** Two separate layers order
incoming messages, and neither closes the gap: the replication layer only keeps each
*device's own* messages in order, and the governance fold checks who authored a fact and
nothing about what it depends on. A dependency that crosses between two devices falls
straight through the crack.

This is the single most important thing the design documents flagged as unresolved — the
"gap-completeness" problem — and G1 turns it from an abstract worry into something you
can reproduce in the real pipeline in half a second.

**What it means:** order-independence is genuine, but "convergence" as first built was
only trustworthy when **delivery is complete**. Under real, lossy gossip, two honest
peers could quietly diverge. The remedy is narrow and already named: check a fact's
declared antecedents before admitting it, so a peer missing a prerequisite *knows* it is
behind instead of believing it is current.

**Update — the guard has since been implemented, and G1 now verifies it.** The fold gained
one step: before folding a **governance** fact, confirm every antecedent it declares is
already present; if not, hold the fact back (it is not admitted and nothing is written)
until the predecessor arrives.

The guard is deliberately scoped to *governance* facts, not ordinary chat messages, and
that scoping is the razor (Part 1 §2.0.1) doing its job. A message that arrives before the
message it replies to is a *display* concern — you can show it, or hold it in the UI, and
nothing about who-may-act depends on it — so the data plane keeps its optimistic, never-
blocks behaviour (a pre-existing test pins exactly that). A governance fact that arrives
before the fact it depends on is a *decision* concern — fold it early and the authority head
diverges — so the control plane waits for completeness. Tolerable for what is shown,
disqualifying for what is decided: the same line the design draws for wall-clocks, applied
to causal gaps. With the guard in place, node B in the scenario above no longer admits
the dependent fact — it *holds* it, and its head becomes a **strict prefix** of the
complete peer's (a state the complete peer actually passed through), not a divergent head
the complete peer never held. That is the crucial change: the two peers are now
"one-is-behind," reconcilable by delivering the missing fact, rather than
"both-confidently-different." The dependent fact heals automatically the moment its
predecessor is delivered (see G2). The one-line test assertion "B admitted the dependent
fact" was written to flip when the guard landed — it flipped, and G1 now reads as a
guard-verification rather than a gap-demonstration. The original gap remains on the record
in git history and in this document.

One honest limit of the guard as built: a held-back peer is now *correct* (it never folds
an incomplete fact) but does not yet *advertise* "I am holding N facts" in its summary — it
simply looks like an earlier, clean state. Surfacing that pending-count so a peer can show
"catching up…" is a small, worthwhile follow-up; the correctness (no silent divergence) is
already in place.

## But the blast radius is small — and that is the other half of the result

A silent divergence sounds alarming until you ask *what* can diverge and *whether it
lasts*. Two experiments bound the damage, and the bound is reassuring.

**Experiment V3′ — the gap cannot touch authority.** We repeated G1, but this time the
missing fact was the one that *granted someone power*. Result: the dependent action was
correctly **rejected**. A peer missing an authority-granting fact simply does *less* — it
never lets someone act on power they can't see. So the gap is specifically about
*ordinary state* (a rule value, a threshold), never about *who is allowed to do what*.
In protocol terms: the fold **under-authorizes but never mis-authorizes**. A lagging peer
is stale, never wrong about permissions.

**Experiment G2 — the gap heals and never rolls back.** We let the missing fact arrive
late. The lagging peer caught up to the correct state and, crucially, **kept everything
it had already accepted** — nothing it previously admitted was reverted. This is the
precise structural defense against the failure that hit Matrix in 2025 (CVE-2025-49090),
where room state could silently roll back to an earlier value. Drystone's fold only ever
moves forward.

**What this pair means:** the failure mode G1 exposes is "temporary, silent divergence on
non-authority state that repairs itself once the data arrives" — not "permanent
corruption," not "privilege escalation," not "rollback." That is a far more benign
envelope, and it tells you exactly where the one missing guard needs to go: into
*detection and signalling*, so a behind peer knows it's behind. The correctness of
authority and the no-rollback property are already holding.

## Convergence is now a property, not an anecdote

**Experiment V1′.** The earlier proof of order-independence was a single hand-built
scenario. We replaced it with 64 randomly shuffled delivery orders of a six-fact,
two-writer history that includes *concurrent* edits from both writers, and confirmed all
64 land on one identical head. That upgrades "we watched it converge once" to "it
converges across many orderings, including the concurrent cases where a content-hash
tiebreak, not causality, has to decide the order." **Experiment G3** adds the other thing
gossip does to you — duplicate and reordered delivery — and shows it is a no-op: a peer
fed every message three times, scrambled, reaches the identical result. Together these
say the ordering spine is solid; the completeness gap is the one real seam, and it is
about *missing* facts, not *misordered* or *repeated* ones.

## The escalation story is now complete

The design says the machine must never fake a decision it cannot legitimately make: when
it hits an irreducible conflict, it stops and surfaces the situation for people to
resolve. There are **two** shapes of that, and the code only had one.

- **Too many valid claims** (a contradiction — e.g. two conflicting foundational facts):
  already built and demonstrated.
- **Too few** (the group loses its last owner, with no valid successor): **was missing.**
  Before this work, a group that lost all its authority silently became "headless" while
  still reporting itself healthy — precisely the case a contradiction-only watcher walks
  right past.

**Experiment V4′** builds the second one. An admin removes the group's sole owner; the
fold now recognizes there is no one left who can legitimately govern and **hard-stops**
with a distinct, legible signal ("under-determined"), surfaced all the way up to a
blocking banner in the app — not mislabeled as a fork. The general lesson for any protocol
that adjudicates: you must enumerate *all* the shapes of "the machine can't decide,"
because a system watching only for contradiction will happily keep running a group that
has quietly become ungovernable.

## What is proven, and what is merely demonstrated

Nothing here is "proven" in the absolute sense, and the design's own stance says it never
will be — claims are corroborated by surviving refutation, not proven. Precisely:

- **G1 is a refutation.** It falsified the unstated reading that "convergence is safe
  under loss," and localized the fix. A refutation that points at the exact missing guard
  is the most useful result an experiment can produce.
- **V3′, G2, G3, V1′ are corroborations.** We tried to break authority-safety, no-rollback
  healing, dedup, and order-independence, and all four held.
- **V4′ is a build plus corroboration.** It constructed a safety mechanism the code was
  missing and showed it fires on the case it is meant to catch.

And an honesty boundary on *scope*: all of this runs against Drystone's **own governance
fold** — real code, real signatures, the real replication path — but in-process, not
across a network and not through the cryptographic key layer (MLS). So these are measured
against the actual implementation, not a model, but they are the fold's story, not yet the
network's or the key layer's. Live cross-host runs and the fork/heal/re-key key mechanics
are the next batteries, still ahead.

## Why this matters beyond Drystone

The completeness gap is not really a Drystone bug; it is a known hard edge of the field
showing up exactly where the theory says it should. Coordination-free convergence is only
free when the operations are *monotonic* — when facts only ever add, never retract.
Governance is not monotonic: it revokes, removes, demotes. Where monotonicity fails, you
fall back on *completeness* — and completeness is the one thing a distributed node can
never confirm about itself locally. So a governance layer built on a coordination-free
fold will always have this seam; the honest engineering response is to **detect and
signal incompleteness**, which the antecedent breadcrumbs already make possible, rather
than to pretend a peer always holds the whole picture.

The comparison with Matrix is the clarifying one. Faced with the same
governance-conflict problem, Matrix concluded you need an all-powerful root to stay safe —
partly because their ordering consumed a forgeable input (a wall-clock), so an attacker
could manufacture authority, so they pinned authority to an apex. Drystone's wager is that
you can *drop* the apex if you drop the forgeable inputs and accept the fork as the
remedy. These experiments test whether the no-apex fold actually holds up on the two
properties whose absence forced Matrix's hand — **can't-escalate-privilege** and
**can't-roll-back** — and both hold. The price Drystone pays instead is the completeness
gap, and the finding is that this price is **detectable and self-healing**, not the silent
privilege or rollback failures the apex was there to prevent. That is a favorable trade,
and now a measured one rather than an asserted one.
