# Side Histories and Threading

`Status: candidate, idea held for design. Not settled mechanism.`

`Scope: how a conversation carries more than one history, from lightweight in-conversation threading up to a cryptographically separate subgroup. Primarily a dataplane-structure question; touches MLS only at the top tier.`

`Companion to: 01-delivery-architecture.md (dataplane hash trees, gap-aware convergence), mls-overview-and-terms.md (branching, resumption PSK, §2.9), mls-hardcases-and-posture.md (§9 ReInit/branch cost and the freeze-then-strand hazard).`

---

## Why this note exists

Threading and side conversations look like one feature but are three different mechanisms at three different costs, and conflating them would either overbuild the common case or underbuild the rare one. This note separates the tiers and gives the single rule that selects between them, so that design work downstream reaches for the cheapest tier that meets the actual need.

The whole note is a held idea, not a decided design. The tier-1 case is effectively settled (it needs no new mechanism). Tiers 2 and 3 are candidates, and the open question of whether tier 2 deserves a named construct at all is stated in Open items.

---

## Terms

`dataplane hash tree`: the content-addressed history structure of a conversation, sealed under a group's keys and reconciled by gap-aware convergence. A conversation's durable history. (01-delivery-architecture.md.)

`entitlement`: the right to read a given history, determined by the keys one holds. Inherited from a group's key layer, not a separate grant. (Drystone; governance layer.)

`thread` (tier 1): a subid field on a message pointing into the existing dataplane hash tree, used to group related messages for display. No new history, no new keys. (This note.)

`side history` (tier 2): a second, separate dataplane hash tree hosted under the same group's keys, addressable and convergeable on its own, exportable as its own tree, with entitlement inherited from the parent group. No new MLS group. (This note, candidate.)

`subgroup` (tier 3): a real MLS branch, a new group over a subset of members with its own key layer and its own entitlement, linked to the parent by resumption PSK. (RFC 9420 §11.3; mls-overview §2.9.)

Epistemic legend: *Settled* (needs no new mechanism), *Candidate* (a held idea, not decided), **[confirm]** (open question to resolve before adoption).

---

## The three tiers

### Tier 1: threading as a subid

A thread is a field in the message, a subid pointing into the one dataplane hash tree of the existing group. Nothing new is created: the messages are already sealed under the existing group's keys, already in the one history, already converging. Threading here is a grouping over content that already exists, and it is a UI and UX function rather than a protocol one.

This is the common case and it is nearly free. It is the expected default for ordinary in-conversation threads (replies, topic grouping). *Settled: no new mechanism.*

### Tier 2: a separate-but-inherited side history

Some side conversations want their own history, not just a display grouping: a "2026 vacation" collection kept aligned with the group but separate, searchable, and exportable on its own; or a "guestbook" off a group, a place to leave a specific message that is not inline and can be exported or processed as its own tree.

The key property: everyone in the parent group may read it. The separation is one of structure, not access. So this is a second dataplane hash tree sealed under the same group's keys, addressable and convergeable on its own, but not a new cryptographic group. Entitlement is inherited from the parent, because there is no new key layer to grant a different one.

Why this is not branching: branching mints a new MLS group with its own membership and key layer (tier 3). Tier 2 mints no group. Its cost is another hash tree to converge, and nothing more, no O(N) instantiation, no new ratchet tree to maintain, no Welcomes, no freeze-then-strand hazard. That is the whole reason to keep tier 2 distinct from tier 3: the vacation and guestbook cases want separate history, not separate access, and paying for a subgroup to get separate history would be overbuilding. *Candidate.*

### Tier 3: a subgroup with its own entitlement

When the side history must have different entitlement than the parent, a subset who may read it and others who may not, cryptographically enforced, only a real MLS branch will do. Branching starts a new group over the subset, with its own key layer and its own membership, linked to the parent by resumption PSK.

This carries the full cost: O(N) instantiation over the subset, a new tree to maintain, and the ReInit/branch freeze-then-strand hazard treated in mls-hardcases-and-posture.md §9. It is the fork arity of the re-plant family, and it is the right tool only when the separation is a membership and entitlement separation. *Candidate; cost and hazard verified against mls-hardcases §9.*

---

## The selector rule

One question chooses the tier: does the side history need different keys, or just different structure?

- Different structure only, everyone in the parent may read it: tier 1 if it is display grouping, tier 2 if it wants its own convergeable, exportable tree.

- Different keys, only some members may read it: tier 3, a real branch.

The load-bearing consequence: entitlement inheritance is what makes tier 2 cheap, and it is also what disqualifies tier 2 the moment access must narrow. In tier 2 a member's right to the side history is definitionally the parent-group right, because there is no separate key layer. The instant the design wants "only some of us see this," inheritance breaks and the case is forced up to tier 3. So the tier-2/tier-3 boundary is not a UX preference; it is a hard line drawn by whether entitlement diverges from the parent. The vacation and guestbook examples sit in tier 2 precisely because they are separate-but-visible-to-the-whole-group.

---

## Why keep this defined now

Holding the three tiers explicit during design does two things. It stops the common case (a reply thread) from being built as anything more than a subid, and it stops the rare case (a private subset side channel) from being mistaken for cheap structure when it actually needs a full subgroup. The cost cliff is entirely at the tier-2/tier-3 boundary, so naming that boundary early keeps later design from stumbling over it.

---

## Open items

- Whether tier 2 deserves a first-class named construct with its own lifecycle (creation, export, garbage collection, convergence scope), or is simply an emergent use of the existing data model, a group hosting more than one dataplane hash tree, needing no new protocol mechanism. This is the central undecided question and it determines whether "side history" is a feature or just a data-model note. **[confirm]**

- If tier 2 is a named construct: how a side history is addressed and discovered, how its convergence scope relates to the parent's (does it converge on the same schedule, the same peers, or independently), and how it is exported as a standalone tree while remaining sealed under the parent's keys.

- Whether a tier-2 side history can be promoted to a tier-3 subgroup later (access narrows after the fact) without losing its history, and what that migration costs. This is the mirror of the re-plant family and likely reuses it.

- Whether tier 1 subids and tier 2 side histories should share an addressing scheme so that a thread can be losslessly promoted to a side history if it outgrows display grouping. *Candidate, not required.*
