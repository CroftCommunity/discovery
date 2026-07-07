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
attachments need a shared anchor), and shared membership is explicitly not shared access. The graph is
load-bearing and invisible, experienced as "me and these people," never as graph administration.

**Does not establish (decision-gated).** The group's-face UX, the load-bearing but invisible home surface,
is the hardest product problem and its iteration is open, not resolved. Reconciling the sticky-group
lifecycle with the membership-vs-access boundary (what a rejoin or a match may and may not expose of prior
history) is a live product decision. These stay surfaced in `../OPEN-THREADS.md`, not closed here. The
substrate claim itself is core Drystone, specified in `../drystone-spec/`; this doc surfaces its product
expression only, and the garden of activities that grows on the substrate is the subject of
`product-the-garden-of-ponds.md`, not reproduced here.
