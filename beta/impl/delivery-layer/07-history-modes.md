# Drystone dataplane history: modes, pruning, and self-destruct

`Status: beta (design + one open thread)`

`Realizes / touches: P-Knowable-Truth, P-Local-Truth (Part 1); Part 2 §5 (Willow/Meadowcap, RBSR), §7.3.1 (ordering), §7.4 (catch-up)`

`Companion to: 01-delivery-architecture.md`

`Substrate: MLS (RFC 9420 / 9750), iroh, Willow family (data model, prefix pruning, Meadowcap, RBSR)`

---

## Verification legend

*Verified*: checked against a primary this round (Willow spec, RFC).

*Synthesis*: the design's own reasoning, labeled as such.

**[confirm]**: load-bearing, to be pinned to a primary before normative text.

---

## 1. Why there are two dataplane history modes

A chat group's dataplane history can be managed in one of two ways, and they are **mutually exclusive** for a given group: a group runs one or the other, not both at once. They fit different group profiles, and naming them as distinct modes (rather than a spectrum or a set of toggles) is deliberate, because their convergence semantics and validity rules differ.

*Synthesis.* The split exists because two different things a group might want, unbounded scale versus coordinated mutability, pull the data model in incompatible directions. An append-only hash-linked fold scales and is simple to verify but cannot mutate mid-history without forking. A path-addressed overwrite store can mutate convergently but carries per-entry overwrite-tracking overhead that only makes sense below some practical size. You cannot have both properties in one structure, so the group chooses the mode that fits its case.

### 1.1 Forward-only mode (large-scale groups)

Append-only, hash-linked causal fold (the §7.3.1 ordering spine). Provenance accrues by accumulation; entries are never overwritten or deleted in place. History "cleanup," where wanted, is a **coordinated hash roll-up** (see §3), not per-entry mutation.

Fits: large groups, broadcast-like or community-scale scopes, anywhere the membership is too big or too loosely-coupled for coordinated mutation to converge cheaply. Scales because no node needs the entire history and the structure is a simple verifiable chain.

### 1.2 Willow-mutable mode (bounded-size groups)

Willow-shaped: entries are path-addressed, and newer entries overwrite older ones at the same path, with deletion expressed as **prefix pruning** (§2). Mutation and deletion are first-class, convergent, capability-gated operations.

Fits: bounded-size groups (families, teams, small communities) that want genuine coordinated edit/delete of dataplane content. The bound exists because Willow's convergence and overwrite-tracking are heavier than an append-only fold; below some practical size this is affordable and above it is not. **[confirm: the practical size bound is an engineering estimate, not yet measured; it depends on the chosen Willow parameters and storage backend.]**

### 1.3 Why "Willow-shaped in anticipation" already helps both modes

*Synthesis, an option-value argument.* The delivery design already chose range-based set reconciliation (the Willow / Negentropy family, design §7) for sync and content-hash addressing for dedup. That makes the substrate **already close to Willow's data model** even in forward-only mode. Consequence: adopting full Willow-mutable mode for the bounded-group case later is an *evolution of an already-Willow-shaped store, not a rewrite*. Being Willow-shaped now, before committing to mutability, keeps both doors open at low cost, and it costs nothing in the forward-only case because RBSR and content addressing are wanted there regardless. This is a deliberate hedge: shape for the harder mode, pay only for the easier one until a group needs the harder one.

---

## 2. Willow prefix pruning: what it buys, and the wall it does not move

### 2.1 The mechanism (grounded)

Willow gives hierarchical path names to payloads, and deletion works by overwriting a prefix: writing a new entry at a path deletes the entries prefixed by it, like overwriting a directory with an empty file, a mechanism Willow calls prefix pruning. *Verified (Willow data-model spec).* Critically, a delete is itself a convergent, synced operation: deletes remove the payload and associated metadata except for a retained tombstone, and both writes and deletes are restricted by the Meadowcap capability system, so who may delete what is an authorization question. *Verified (Willow spec; Meadowcap).*

This is qualitatively different from the forward-only fold. The fold is a linear hash chain, so removing a mid-history entry breaks every downstream hash (a fork from that point). Willow is not a linear chain; it is a set of path-addressed entries with overwrite rules, so deletion is a *convergent overwrite*, not chain surgery. *Synthesis from the two data models.* This is the real capability difference: **Willow can express coordinated, convergent, authorization-gated deletion of an entry as a normal synced operation, where the forward-only fold cannot without forking.**

### 2.2 The wall Willow does not move (grounded limits)

Two limits, both load-bearing, both leaving the trust wall exactly where it was.

- **Deletion is metadata-convergent, not existence-erasing.** The prune leaves a tombstone: the existence of the (now-removed) entry remains recorded even after the payload is gone. *Verified.* So deletion is *visible* (honest nodes converge on knowing it happened), not *secret*. Good for honesty, but it means "a value was here and was removed" is itself a propagated fact.

- **Convergence governs honest Willow nodes; it cannot reach a copy already taken.** Prefix pruning deletes the payload from the *store*. It does not and cannot reach a copy a node exported, screenshotted, or kept by running a modified client. This is the same irreversibility wall as cooperative chosen-ephemeral (design §8.1): Willow does **not** break it.

*Synthesis, the honest one-liner.* Willow turns deletion from a fork into a convergent, capability-gated tombstone, a real capability gain for coordinated removal, but it moves **none** of the trust wall: deletion remains cooperative non-retention against honest nodes, never enforced erasure against an adversary, and never corroborable. Willow upgrades the gentleman's agreement (from uncoordinated, unprovable, chain-breaking to coordinated, convergent, tombstoned, capability-gated); it does not eliminate the agreement.

---

## 3. Coordinated history roll-up (a distinct governance item, not deletion)

*Synthesis, scoped here to keep it separate from self-destruct.* Distinct from per-entry deletion is **coordinated history roll-up**: a group collapsing or summarizing old history to bound storage. This is a **group governance decision with a group default for local execution and corroboration**, explored elsewhere and not specified in this doc. It is named here only to keep it firmly separated from the two adjacent ideas it is often confused with:

- It is **not** chosen-ephemeral (that is a per-message retention disposition).

- It is **not** self-destruct (§4).

- In forward-only mode it is the *only* form of "cleanup" available, and it takes the shape of a coordinated hash roll-up rather than mutation.

The mechanism (how the group decides, what the default is, how local execution is corroborated) belongs to its own governance treatment and is referenced, not redefined, here.

---

## 4. Open thread: self-destruct (time-bound sensitive value)

`Status: open thread, deliberately not specified; framing and open questions only`

### 4.1 The case and the threat model

The motivating case: share a sensitive value that all members would prefer not persist, time-bound. "Here is the guest wifi password; show it until next Monday." The threat model is **modest**: the goal is a good-enough, honest auto-clean that beats the status quo (passwords living forever in SMS, or written on paper and lost), not anti-forensic guaranteed erasure against a motivated adversary. Naming the modest threat model is what keeps this honest rather than snake oil.

### 4.2 The reframe: strength is bounded by node fidelity, not cryptography

*Synthesis, the central idea.* Self-destruct cannot be made cryptographically enforceable (you cannot prove a recipient did not copy the value, nor that every device expunged). So its strength is not a cryptographic property; it is a **trust-and-fidelity property**. The question is not "how do I make erasure provable" (impossible) but "which nodes are in play, whom am I trusting to honor the disposition, and how reliable is their handling", which is a social-utility judgment of exactly the kind §2.3 already frames for trust composition.

This makes self-destruct the **inverse of provenance**. Everything else in Drystone maximizes durability, replication, and provable retention; self-destruct deliberately *minimizes* all three, to keep the value within a boundary of nodes trusted to honor its removal.

### 4.3 What that implies (design sketch, not spec)

- **Deliberately opt out of blind-store durability.** A self-destruct value must not sit sealed in a meer past its window, because a meer is *outside the fidelity boundary*: it is blind, runs no disposition logic, and cannot be trusted to honor removal because it does not honor anything, it just holds bytes. So self-destruct is delivered live-durable to member devices and **skips D-meer and D-peer corroboration** (design §8.1). The opt-out is principled, not a mere knob: you skip the meer because it is not a node you can extend removal-fidelity trust to.

- **The fidelity boundary is the membership, and it must be legible.** The sender is trusting the member devices in scope to honor "mask or remove after T." The system should *show* the sender that boundary (this reaches these N member devices, none of which is a blind store, and honest clients will expunge), so the social-utility judgment is informed. The sender then decides whether those nodes are trustworthy enough for the sensitivity at hand, which for a guest wifi password is an easy and reasonable yes.

- **The disposition is "mask or remove," honored by honest nodes, never enforced.** Display until T, then mask (hide from view) and/or expunge (drop from store). The honest envelope is mandatory and identical to chosen-ephemeral: not enforceable against a modified client, not corroborable. At this threat model, "honest clients honor it and no blind store ever held it" is genuinely good enough.

### 4.4 Open questions for the dedicated investigation

- Selector signaling: how a payload marks "do not durably store in a meer, do not enter D-peer sets," and the delivery semantics when a member device is offline past T. (Likely **never deliver** an already-expired secret to a late device, since delivering it only widens exposure for no benefit.)

- Whether "mask" (hide but retain) and "remove" (expunge) are distinct dispositions, and whether the masked or removed form keeps a Willow-style tombstone (honest "a value was here and is gone") or vanishes entirely.

- How the fidelity boundary is computed and surfaced, and whether a member device's *client profile* (does it run a build known to honor dispositions) can be part of the legibility surface.

- **Interaction with the two history modes (the cross-cutting consequence):** self-destruct is *more capable* in Willow-mutable mode, where prefix pruning is a real convergent removal of the stored value, and *display-mask-only* in forward-only mode, where the sealed record is in the append-only fold and cannot be excised without a fork, so the value can be hidden from display but the sealed entry remains in history. So the same self-destruct request yields **different achievable semantics by mode**, and this must be made legible to the sender (in forward-only mode, "removal" honestly means "masked from display, not excised from the fold").

---

## 5. Summary of what is settled vs open

Settled:

- Two mutually-exclusive dataplane history modes: forward-only (large groups, coordinated hash roll-up for cleanup) and Willow-mutable (bounded groups, convergent prefix-prune deletion).

- Being Willow-shaped already (RBSR, content addressing) makes the mutable mode an evolution, not a rewrite, at no cost to the forward-only case.

- Willow prefix pruning is a real coordinated-deletion capability gain, with a hard, grounded limit: convergent tombstone among honest nodes, never enforced or corroborable erasure.

- Coordinated history roll-up is a distinct governance item, separated from both chosen-ephemeral and self-destruct.

Open:

- Self-destruct, as a node-fidelity-bounded, inverse-of-provenance payload class, framed here with its honest envelope and its mode-dependent achievable semantics, but deliberately not specified pending dedicated investigation.

**[confirm] residue for this doc:**

- The practical size bound separating forward-only from Willow-mutable (engineering estimate, parameter- and backend-dependent).

- The specific Willow instantiation parameters and Meadowcap capability shapes Drystone would adopt for the mutable mode (Part 2 §5 decision).

- Whether Drystone's forward-only fold and a Willow-mutable store can share enough storage substrate that a group could *migrate* modes, or whether mode is fixed at scope creation. (Suspected fixed-at-creation given the differing convergence semantics; verify.)
