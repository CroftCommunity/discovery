# Willow and Meadowcap: the correct mental model

`Status: survey, current as of mid-2026. Willow took breaking changes into 2026 (see Maturity); treat version and readiness claims as a snapshot. Distilled from the batch-ten Willow thread and the feasibility pass; grounded to that source, not re-verified against Willow's own specs.`

`Scope: what Willow actually is as a building block (a state-based CRDT reconciled locally, not a document shipped whole), the Entry structure and its merge, the timestamp wrinkle that matters for any governance fold, Meadowcap as the capability layer, and current maturity. Ends with what Drystone takes from Willow and what it leaves.`

This is a survey document, not a specification. It exists to correct one specific mental model, because getting it wrong changes what you think Drystone inherits for free.

> **The framing to lead with.** Willow is not a JSON object that peers ship whole to each other. It is a state-based CRDT that each node holds locally and reconciles range by range. That distinction is the whole point: it tells you which guarantees come for free (convergence) and which do not (causality, and therefore conflict preservation).

---

## The question this answers

The starting question was direct: is Willow a data-struct object exchanged between peers, or is it closer to a CRDT structure that nodes hold locally and reconcile.

The grounded answer, also confirmed in the feasibility pass, is the second one. Nothing below makes sense until that is settled, so it comes first.

## What Willow actually is

Willow's data model is a **state-based CRDT**, specifically a **join-semilattice**. The practical consequence of that shape is that merging replicas is order-independent and idempotent: peers can merge in any order, more than once, and still converge on the same state.

Nodes hold their state locally. They do not ship the whole structure to each other. Instead they **reconcile ranges**: Willow carries its own range-based set reconciliation plus a confidential sync layer, so two peers can compare and settle a slice of the space without transferring everything.

That is the correction. "A CRDT held locally and reconciled," not "an object exchanged whole."

## The Entry structure

The unit of state is the **Entry**, which is metadata:

- `namespace_id`

- `subspace_id`

- `path`

- `timestamp`

- `payload_length`

- `payload_digest`

The **subspace is the writer dimension**. That is the axis along which different writers are separated within a namespace, and it is the hook any higher layer would use to keep writers apart.

## The merge

Merging is set union followed by a deterministic reduction. In order:

- **Set union** of the entries being merged.

- **Prefix pruning**: a newer entry at a prefix prunes the older entries beneath it (this is how hierarchical delete works, a newer entry higher in the path can remove a whole subtree).

- **Tiebreak** when entries collide: greater `timestamp` wins, then greater `payload_digest`, then greater `payload_length`.

Put together, single-writer union behaves as **last-writer-wins per path by timestamp**, with hierarchical delete, over a pruned grow-set.

## The wrinkle worth remembering

This is the part that matters most for anything built on top.

The `timestamp` is a **writer-assigned, "claimed" U64**. The recommended value is microseconds of TAI since the year 2000, but that is only a recommendation and it is unenforceable. It is **not a logical or causal clock**.

The consequences follow directly:

- Single-writer semantics remove inter-author concurrency, but **not** same-author-across-devices concurrency (one writer, two devices, is still concurrent).

- Different timestamps: the later one wins, the earlier one is silently pruned.

- Identical timestamps: the digest tiebreak fires, and the survivor is **content-arbitrary** (whichever digest happens to be greater, not whichever edit was meant to win).

Because the timestamp carries no causality, **Willow tracks no causality at all**. A concurrent conflicting edit and an intentional sequential overwrite are indistinguishable to it. The guarantee Willow gives is convergence; the guarantee it does not give is semantic fidelity, and the loss is silent (nothing surfaces to say an edit was overwritten rather than merged).

## Why that matters for Drystone

Convergence is not the same as conflict preservation. Any layer that needs to **detect concurrency** or **preserve conflicting events**, for example a governance or authority fold where two members act at once and both actions must be kept and reconciled, cannot lean on Willow to do it.

Such a layer must carry that itself: **per-writer subspaces, its own logical clock, and its own merge rule on top**. This is the same completeness concern as Drystone's cheap-merge open thread, seen from the Willow side.

## Meadowcap

**Meadowcap** is Willow's capability system. It provides **unforgeable grants** with **attenuation by subsetting**: a holder can mint a narrower capability from a broader one (the Alfie-and-Betty minting example in the source), but never a broader one from a narrower, so authority only ever shrinks as it is delegated.

## Maturity

Readiness is uneven across the pieces:

- **Data Model** and **Meadowcap**: Final.

- **Confidential Sync**, the **Drop Format**, and the **Willow'25 parameter set**: took breaking changes into 2026.

- **Implementations**: pre-1.0 (TypeScript at 0.x, Rust newer, Earthstar at 11 beta).

That is the same "not-ready-as-a-dependency" profile that Roomy and p2panda both ran into. It is a reason to treat Willow as a source of design shape rather than as a drop-in library, at least at this snapshot.

## What Drystone takes vs leaves

Drystone is built **Willow-shaped** without taking Willow whole:

- **Takes:** the two structural ideas that Willow gets right, **range-based set reconciliation** and **content-hash addressing**. These are the parts that give convergence and efficient sync for free.

- **Takes, in bounded form:** a **Willow-mutable mode for bounded groups**, where last-writer-wins over a reconciled grow-set is an acceptable model because the writer set is small and known.

- **Leaves:** the assumption that convergence is enough. Where Drystone needs to detect concurrency or preserve conflicting events, it does not inherit that from Willow (Willow tracks no causality), so it supplies its own per-writer subspaces, logical clock, and merge rule on top, as noted above.
