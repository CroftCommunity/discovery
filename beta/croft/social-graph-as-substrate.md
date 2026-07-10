# The social graph is the substrate; chat is a tenant

date: 2026-07-07

status: product synthesis (Layer 6, Croft). Register: the substrate CLAIM is core Drystone (specified in
`../drystone-spec/`); this doc is its PRODUCT surfacing. Decision-gated items are surfaced as open, not
resolved (tracked in `../OPEN-THREADS.md`).

---

## Overview

Most messaging products put the chat thread at the bottom of the pyramid and bolt games, calls, and shared
media onto it. The thread is then forced to be two things at once: the communication medium, and the index
of everything the group ever did. It is bad at the second. A game launched into a thread pollutes it; it
cannot be pinned; history cannot be managed; and ending the thread destroys the group, so zombie threads
live forever and membership and governance stay welded to a single conversation.

Croft inverts the pyramid. The durable thing is the **group**: a stable social object with its own identity
and history. Chat, games, calls, and shared media are **co-equal siblings attached to that group**, none
nested inside another. A chat can end while the group persists; you can spin up a fresh chat on the same
group later, with the games and photos still attached, because the group is distinct from any one chat but
determinate. The group is the durable index; the thread goes back to being only a thread.

This is the product expression of Drystone's relationship graph, equal in rights not capabilities, surfaced.
The substrate claim itself lives in the spec; what follows is how the product presents it, and where that
presentation is still an open design question.

## Why a graph you hold does not already exist

Work life has mailing lists, org charts, and distribution groups; real life has no equivalent. There is no
durable, nameable, reachable way to say "all of my grandparents' descendants except this one person" to reach
the aunts and uncles, or to name "the parents of the kid mine is staying with tonight." The structure of real
social life is rich and no tool reflects it — and the reason is not technical difficulty. Every attempt to
build one was built by an extractor who needed the graph for its own purposes, so it felt invasive and died.
Google+ Circles is the *canonical corpse*. Worse, the graph gets built regardless of consent, assembled about
you rather than held by you: the *shadow profile* is the graph made without you in the room.

So the inversion is about ownership, not a feature gap: you hold the graph of your life, you see it, you shape
it, no one monetizes it, and it is invisible to anyone you have not chosen to show it to. The reason a held
graph does not already exist is that there is no business model in a graph you cannot extract from — which
makes the absence of an extraction model not a missing feature but the entire point. The graph can be honest
precisely because no one is selling it. That is the same non-extractive stance the rest of Croft takes,
turned on the relationship structure itself, and it is what makes a graph-you-hold genuinely novel rather than
another social product.

## Chat is one tenant, activities are siblings

The foundational move is that the social graph is the bottom of the product and chat is one tenant on it,
not the floor. Putting the group at the bottom and hanging chat, games, calls, and media off it as siblings
dissolves a cluster of pains at once:

- A game gets its own surface hanging off the shared group, so it is a sibling and not a guest polluting a
  message river; it is pinnable, durable, and manageable.

- The thread is only a thread again, no longer conscripted as the group's permanent index.

- A group's durability never depended on any one chat, so "groups durable, chats need not be" finally
  coheres: a chat can end, and a fresh chat on the same group inherits the attachments intact.

- Membership and governance stop being welded to a conversation, so ending a chat does not end the group.

This is the same category error other products make structurally, corrected structurally. The activities
that ride on the group substrate (the garden of ponds and pads) are the sibling doc's subject; see
`product-the-garden-of-ponds.md`. This doc is about the substrate the garden grows from, not the garden.

## The durable group: identity is not the member set

The group is a first-class, durable object whose identity is **not** its member set. The same group survives
members joining and leaving, and the same people can form two different groups (two nodes, not one). "Same
members" is not the identity key. Concretely:

- Each group has a **stable internal ID** that exists from the moment of formation, independent of
  membership. This is what lets membership change without the group becoming a different group.

- The group carries a **presentation name** that is shareable but **locally overridable**: one identity,
  many faces. The ID is fixed; the name is a per-participant attribute a user can set for their own view.

- **Implicit and explicit formation produce the identical object.** Start a chat and a group quietly forms
  behind it; or make a group first and then attach a chat and a game. These are two affordances for one
  primitive, and an implicit and an explicit group must be indistinguishable once they exist. The only
  difference is the moment of naming.

## The group lifecycle: implicit, sticky, pruned

Every group has a stable ID and accrues history, but not every group should be surfaced and rediscoverable.
The lifecycle resolves the privacy tension with three states:

- **Sticky.** Persistent, surfaced, and **matchable** for reconciliation ("oh, it is us again"). A group
  becomes sticky by an affirmative gesture, not automatically.

- **Live but non-sticky.** Real, accruing history, but not matchable; it lives and fades without the user
  ever curating it, and prunes when quiet.

- **Pruned.** Deliberately ended and **never resurrected**. Reforming the same people afterward is
  definitionally a new group.

The load-bearing rule is that reconcile-vs-fresh-vs-prune is a **per-formation human choice, never forced**.
Reusing the same people *suggests* an existing group; it never silently rejoins one. This matters acutely in
a family-safety product, where "the system grouped me back with someone" is exactly the surprise to avoid.
The member set is a hint to a human decision, not a key that forces a match.

## The seam: local projection vs shared anchor

Presentation and association are local; access and cross-participant group identity need a shared anchor.
Keeping this seam honest is what keeps the product truthful about what it can and cannot promise.

- **Local (no agreement needed):** naming, stickiness, local reconciliation, and a participant's own
  presentation of the group. These are local projections; each user can hold their own without consensus.

- **Shared (needs an anchor):** membership changes, pruning ("never resurrect"), and **new attachments
  landing as "the same group" for everyone**. The thin shared layer is the set of stable group IDs
  participants recognize, plus the shared log of events and artifacts.

Membership must be shared, but shared membership is not shared access. Membership is agreement about who is
*currently* in, going forward. A participant removed from a group stops receiving new content, but cannot be
made to un-see what they already hold: local removal is not global revocation. The UX must communicate that
boundary truthfully. "Removed, will not see anything new" is sayable; "can no longer see what was already
shared" generally is not. Reconciling the sticky-group lifecycle with this membership-vs-access boundary is
a live product decision, surfaced below.

## Membership and access are two axes

The seam above draws one membership boundary — shared membership is not shared access. A second, orthogonal
cut runs beside it and must not be confused with it: **membership** and **access** gate different layers.
Membership gates the infrastructure-and-governance layer — who owns and helps govern the group (a stake and a
vote), including any sponsees a member brings along. Access gates the room layer — who may enter or post in a
given activity or pad. A member can hold a public, even anonymous, door open onto a specific pad while the
group itself stays member-governed: the visitor is a guest in a room, not a member of the co-op.

The decoupling is load-bearing because it keeps the frictionless door without diluting ownership, and it puts
Sybil resistance exactly at the seam that matters. Because an anonymous guest carries no governance weight, a
cheap public door grants nothing that could capture the group — spam in a guest room cannot become spam votes
in the co-op. Frictionless onboarding and Sybil resistance are in tension only when the door also confers a
stake; separating the two axes lets the public door stay cheap precisely because it grants nothing to
capture. The governance side of this — whether a sponsee is a full governance-constituent, an access-only
dependent, or a middle tier, and the general rule that a member is not automatically a governance-constituent
— is a decision carried in `../OPEN-THREADS.md`; what belongs here is the substrate distinction that a group's
membership and a room's access are two independent axes.

## Freeze by default

The default posture for anyone at risk is that nothing enters your view without your pull. Your social world
does not grow because someone else's did; trust is transitive only as far as you personally walk it. This is
the positive statement of the discipline the spec records negatively as the graded-vouch rule — the refusal
of web-of-trust's automatic transitive trust, where trust propagates on its own and the mess is cleaned up
afterward. Here that refusal becomes a default: you decide what enters, and nothing arrives uninvited. It is
promoted from a setting to a core invariant precisely because a safety setting an at-risk user has to find is
a setting that fails the person who most needs it. The vouch mechanics themselves are specified in
`../drystone-spec/part-1-reasoning-underpinnings.md`; this is their product-surface default.

## Structure leaks identity: the only safe share is consented

A held graph invites an obvious-looking feature: show a viewer the useful *structure* of your connections
while hiding the *names* — an anonymized shape. It cannot be offered, because graph topology is itself a
near-fingerprint. Modeling the canonical attack makes the failure exact. In a town of 4,000 where 3,994
people are structurally generic, five touch the Henderson family group, and exactly one of those five also
touches the Oak-Street school-parents group, the anonymity set for that person's connection shape collapses to
**one**. Withholding every name does not help; the shape alone re-identifies. "Show the structure, hide the
names" is therefore unsafe by construction, not merely risky — and it is treated as *unrepresentable*, a
share a viewer can never be handed, rather than a setting to use with care.

What remains safe is the inverse: a share scoped to what a person has consented to expose to someone at a
given distance, at the resolution they chose. A viewer at distance *d* sees exactly the content the owner
consented to expose at distance *d* — never nearer distances' content, and reaching past the consented horizon
returns silence, not "structure minus names." This is the honest version of the six-degrees idea: distance
gates *consented content*, never the graph's shape. It is the highest-value privacy result in the substrate
work because it rules out an entire class of "anonymized graph" features at the root rather than hardening them
after the fact. The result was established empirically, not asserted (see the social-layer visibility proof in
`reference-index.md`).

## The group's-face UX: load-bearing but invisible

The discipline that keeps all of this from becoming a chore is that the graph is **load-bearing and
invisible at the same time**: foundational as architecture, almost entirely absent as experience. The user
lives "me and these people, and the things we do together," and never "administering a graph."

The one surface the group genuinely needs is its **home, or face**: a place you and these people share,
navigable into its attached chats, games, and media. The hardest product problem in the whole surface is
keeping that face from degrading into a settings page for a graph node. Entry points are **plural but
convergent**: a user can arrive through a person, an activity, a chat, or the face itself, and all of them
route to the same group. Many doors, one room.

This is the hardest UX problem, and it is not solved. The iteration on the group's-face UX is a live product
decision, carried as open below rather than presented as settled.

## What this dissolves

Read against the pains of a thread-bottomed product (the Delta-Chat shape), the substrate reframe removes
each one at its root rather than patching it:

- Games polluting a thread: each activity is a sibling surface on the group, not content inside a
  conversation.

- Chats coupling membership and governance: membership and governance attach to the durable group, not to
  any one chat.

- Un-pinnable activities: a sibling surface on the group is pinnable and manageable by construction.

- Chats living forever: chats are disposable tenants; the group is what persists, and the lifecycle lets a
  chat end (and a group prune) cleanly.

## What this establishes (and does not)

**Establishes.** The product is built on the social graph as its substrate: the durable group is the bottom
of the pyramid, and chat, games, calls, and media are co-equal siblings attached to it. A chat can end while
the group persists, and a fresh chat on the same group inherits the attachments. The group's identity is not
its member set (a stable internal ID plus a shareable, locally-overridable presentation name); implicit and
explicit formation yield the identical object; and the implicit / sticky / pruned lifecycle makes
reconcile-vs-fresh-vs-prune a per-formation human choice, never a forced match. The local-projection vs
shared-anchor seam is drawn honestly (naming and stickiness are local; membership, pruning, and new
attachments need a shared anchor), and shared membership is explicitly not shared access. Membership and
access are two independent axes — membership gates ownership and governance (including sponsees), access gates
a room — so a member can hold a cheap public door open without diluting ownership, and Sybil resistance holds
because a guest carries no governance weight. A held graph is genuinely novel because no extractor-built
version survived (Google+ Circles the canonical corpse; the shadow profile the graph built without consent),
and the absence of an extraction model is the point, not a gap. Two safety defaults are structural, not
optional: freeze-by-default (nothing enters your view without your pull), and the result that graph structure
leaks identity — "show the structure, hide the names" is unrepresentable because topology re-identifies (the
town-of-4,000 anonymity set collapses to one), so the only safe share is per-distance consented content. The
graph is load-bearing and invisible, experienced as "me and these people," never as graph administration.

**Does not establish (decision-gated).** The group's-face UX, the load-bearing but invisible home surface,
is the hardest product problem and its iteration is open, not resolved. Reconciling the sticky-group
lifecycle with the membership-vs-access boundary (what a rejoin or a match may and may not expose of prior
history) is a live product decision. These stay surfaced in `../OPEN-THREADS.md`, not closed here. The
substrate claim itself is core Drystone, specified in `../drystone-spec/`; this doc surfaces its product
expression only, and the garden of activities that grows on the substrate is the subject of
`product-the-garden-of-ponds.md`, not reproduced here.
