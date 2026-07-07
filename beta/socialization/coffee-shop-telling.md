# Drystone: the coffee-shop overview

`Resolution: coffee shop, a one-to-one-and-a-half-page telling for grounded discussion and debate. For the full mechanics and the definitive statement, read the library (Parts 1 and 2); for the shortest framing, the elevator pitch. Where this and the library diverge, the library governs.`

## The problem

Most systems put a server in the middle of people's relationships, and that server becomes the arbiter
of what is true: it holds the canonical copy, decides what is current, and can certify, revise, or revoke
any of it. That is convenient right up until the operator's interests and the participants' diverge, at
which point the participants discover that the thing they were standing on was never theirs. Everything
that leaned on the center, who you are, what was said, who may act, who can be reached, fails at once, and
there is no local ground to fall back to.

## The move, and why it is forced rather than preferred

Drystone is a **center-free peer protocol** for group messaging and governance: peers connect directly,
and no node holds privileged or canonical authority over the shared state. The reason is not taste. A
distributed system is a set of nodes communicating remotely about shared state, and no node can prove it
holds complete and current knowledge of the others. That single structural fact, not a value judgment, is
what forces the design. If no node can know it is current, then no node's word for the state of the world
can be the one everyone must take, so the protocol is built so that nobody ever has to take a center's
word, because there is no center whose word could be taken.

## What you get

- **Your copy is real and usable alone.** The primary copy of what you authored lives with you, complete
  and usable offline; the system reconciles between participants rather than constituting them.

- **You can check the record yourself.** State is auditable and never silently mutable: history is
  append-only and verifiable, so a change is something you can see rather than something done to you.

- **Everyone is equal where it counts.** Personae are equal in rights and therefore equal in weight, one
  per person however many devices they run. They are free to be unequal in resources and in authority, but
  authority is always revocable and escapable, so concentration never hardens into a center.

- **Participation and exit are real on a bare phone.** The floor of rights, including the ability to
  leave with your state intact, does not depend on running a powerful node. Exit is a first-class right,
  not a courtesy.

- **When a group genuinely splits, it splits cleanly.** This is the most distinctive part. The substrate
  computes provenance, who said what and who held standing, but it never renders the social verdict about
  who was right. So the harshest thing any authority can do is force a **fork**, and the excluded party
  walks away whole in their own lineage rather than being erased.

- **The group is represented, not resolved for.** The system's job is faithful representation of what the
  members did and decided, not a resolution an operator imposes on their behalf.

## The shape of the how

Underneath, Drystone builds on **MLS** (RFC 9420/9750) for the cryptographic group, an **iroh** transport
for peer connectivity, and a local-first, append-only **governance log** that every node folds to compute
the current authority: membership, thresholds, roles. Concurrency is expected, so when two facts genuinely
race, one of two things happens. If the clash is a benign sync artifact, a party-neutral deterministic
tiebreak picks the same single survivor on every node. If it is a real social dispute, the protocol
refuses to auto-merge and **hard-stops to the people**, because manufacturing one answer would be imposing
a fiction. A ban is not special machinery; it is the same fork primitive as a voluntary departure, which
is why a fork sits as the floor beneath every power in the system.

## Where it sits

The field split into two camps and Drystone occupies the seam. One camp solved the **data layer** by
abandoning global coordination, CRDTs and local-first, and stopped there; it has no notion of expulsion,
contested authority, or what a conflict means. The other camp solved the **governance layer** but kept a
coordinator, and under pressure concludes you need an apex. Drystone carries the no-coordination premise
of the first camp up into the governance layer of the second, removes the wall-clock and authority from
the ordering entirely, and builds the resolution mechanics as a peer protocol rather than a layer atop a
platform. It is a **complement** to ATProto-class networks such as Bluesky, not a competitor to them.

## Honest tradeoffs

Local-first is necessary for a humane system, not sufficient: the edge can be wrong too, and honest
friction between real nodes is the real cost, distinct from the manufactured friction a center imposes.
Removing the apex means there is no single authority that can force convergence, so a genuine disagreement
ends in a fork, which is the point, but it also means reunion afterward is a governed choice rather than
something the system does automatically. Parts of the design are still open and named as such, including
the property that lets a node know when it is behind on the very newest governance, and the exact rights
set under key rotation. And the novelty claim is scoped honestly: the mechanisms are largely prior art,
and what is Drystone's own is the vertical synthesis, epistemic limit to data model to equality to
human-adjudicated fork, and the fork-not-verdict terminus, unoccupied against the closest published
neighbors rather than first ever.

## Read on

For the complete, definitive treatment, read the **library**: Part 1 (the reasoning, why each principle
is forced) and Part 2 (the mechanics an implementation is built and validated against). For the shortest
framing, read the **elevator pitch**.
