# Object-capability and decentralized-MLS prior art: two strands Croft builds among

`Status: cairn layer (Layer 3, the open field). Register: survey / homage. Resolution: library — two smaller
prior-art strands carried to beta so their reasoning lives in one place. External facts carry verification
flags inline: the decentralized-MLS rows (DMLS/FREEK, draft-xue) carry a "confirm before publish" flag; the
Spritely facts are [UNVERIFIED current]; the Keyhive/Meadowcap capability-decision references are
[dialogue-sourced 2026-06-24, pending verification]. Refresh the flagged rows against primary sources before
external use.`

## Overview

Two prior-art strands sit close enough to Croft's capability and encryption layers to deserve crediting, but
neither is large enough to carry a document of its own — so they are registered together here. The first is
the **object-capability** tradition (Spritely Institute's Goblins / OCapN / CapTP): the clean formal frame
for authority as designation rather than as ambient permission. The second is the **decentralized-MLS**
family (DMLS / FREEK, and `draft-xue-distributed-mls` "TwoMLS"): the closest serverless-MLS relatives to
Drystone's own serverless ordering, and the empirical anchor for the claim that no *production* MLS
deployment orders without a server. The load-bearing reason to keep them together is that both answer the
same question from opposite ends — *what is the honest prior art for doing this without a trusted center* —
one for capability, one for group encryption.

## Charter: what this document covers

- **In scope:** the object-capability discipline as the formal frame for Croft's capability-not-authority
  layer, and the decentralized-MLS siblings that quantify the cost Drystone's serverless ordering incurs,
  each with the reason it is credited.
- **Out of scope (and where it lives):** the MLS core and the MIMI interop story (`mls-and-mimi.md`); the
  Willow/Meadowcap capability layer and the Track A / Track B capability-mechanism decision (A11) itself
  (`willow-meadowcap.md`); the substrate/transport prior art (`substrate-prior-art.md`).
- **Boundary call:** Keyhive (Track B) and Meadowcap (Track A) are the *capability-mechanism decision* and
  are documented where that decision lives, not re-filed here. This document credits the *object-capability
  discipline* those tracks operate within, and the *decentralized-MLS siblings* that neighbor Drystone's
  ordering — it does not re-open the mechanism choice.

## Object-capability prior art: Spritely's Goblins / OCapN / CapTP

The **Spritely Institute** builds object-capability distributed programming: **Goblins** (a distributed
object framework), **OCapN** (the Object Capability Network protocol), and **CapTP** (the Capability
Transport Protocol) that carries object references across a network so that holding a reference *is* holding
the authority to invoke it. The tradition's central maxim states the model in four words:

> *Designation is authorization.*
>
> — the object-capability maxim carried by the Spritely Institute's Goblins work `[UNVERIFIED current]`

Two named concepts from the tradition matter directly to Croft. The *principle of least authority* (POLA) —
each component holds only the authority it needs and no more — is the discipline that keeps a capability
system from silently accumulating ambient power. And *petnames* — locally-assigned, human-readable names that
each party binds for itself — are prior art for readable naming that needs no global registry and no central
allocator to resolve a name to a party.

Why this is load-bearing rather than a mere neighbor: the object-capability discipline is the clean formal
frame for Croft's capability-not-authority layer. It is the difference between "a peer may act because the
group granted it a role" and "a peer may act because it holds the specific reference that names the action" —
the latter is what *designation is authorization* buys, and it is the shape Croft's capability layer wants to
hold. The petname model is the matching prior art for Croft's local, human-readable naming without a global
namespace: naming that stays legible without conceding a registry that could capture it. Relationship:
homage, learn↔.

There is also a governance signal worth crediting alongside the technical one. Spritely is a **501(c)(3)**
funded through **NLnet / NGI** (EU public-interest infrastructure funding), with **no VC and no token**. That
funding-and-governance model — non-extractive, grant-funded, no equity or token overhang steering the
protocol — is prior art for the *sustainability without capture* question Croft carries in its own
cooperative lineage, and is credited here for that reason as much as for the code. `[UNVERIFIED current;
refresh Spritely funding and project facts before external use.]`

## Decentralized-MLS siblings: the closest serverless relatives

MLS (RFC 9420) assumes a delivery/ordering service — some node picks the winning concurrent commit and
imposes a linear epoch chain (the mechanics and the reason are in `mls-and-mimi.md`). A small research family
has tried to remove that assumption. These are the closest relatives to Drystone's own serverless ordering,
and they matter for two distinct reasons: they *quantify the cost* of ordering without a server, and they are
the *empirical anchor* for how rare it is.

**DMLS / FREEK (Phoenix R&D).** DMLS extends MLS to process **out-of-order Commits** — advancing group state
without a single privileged node imposing a global order — with **reduced forward-secrecy loss** via
**FREEK** (Fork-Resilient CGKA; Alwen, Mularczyk, Tselekounis). FREEK uses a **puncturable PRF (PPRF)** to
recover most of the forward secrecy that naive out-of-order processing would forfeit. The mechanism is
selective deletion, not wholesale retention: per commit a client **punctures** its retained key material —
deleting the direct-path secrets so the same output can no longer be re-derived (that deletion *is* the
forward-secrecy property) while keeping the co-path secrets so other, concurrent commits still process. This
is **cost-shifting, not magic**: the price is **storage** — on the order of **~8 kB per PPRF evaluation**,
scaling with the **retention window, group size, key size, and fork frequency** (not fork frequency alone).
The authors frame it modestly: a building block that *meaningfully improves* forward secrecy where forks are
inevitable, **not** a full restoration of the on-schedule-deletion forward secrecy that server-ordered MLS
gets for free. Status: an IETF draft plus a proof-of-concept OpenMLS fork (a Matrix-side DMLS fork also
exists), with **no production deployment as of mid-2026**. `[web 2026-06-26, confirm before publish.]`
Relationship: learn↔.

**`draft-xue-distributed-mls` ("TwoMLS", Naval Postgraduate School).** A second, independent approach to
serverless MLS, presented at **IETF 124**. It gives each member its own **"Send Group,"** achieving PCS and
forward secrecy **without global ordering consensus**, aimed explicitly at P2P and partitioned topologies
where no node can be trusted to order for everyone. `[confirm before publish.]` Relationship: learn↔.

Why these are load-bearing for Drystone. They are the nearest kin to Drystone's serverless ordering, and the
FREEK result in particular **quantifies the fork→forward-secrecy cost** that a fork-and-reconcile model
incurs: recovering forward secrecy after out-of-order or forked commits is not free, and the price is storage
that grows with the retention window, group size, and how often the group forks. That is the honest cost
curve Drystone's own fork/reconcile ordering sits on, stated by a sibling that measured it. Keeping the
reason with the reference is the point — the sibling is credited not for being adjacent but for pricing the
exact tradeoff Drystone makes. The engineering argument that *licenses* Croft's chosen ordering role sits in
`the-four-property-tension.md`; this doc is the home for the measured cost curve that argument relies on.

**The empirical anchor.** Set against these drafts, every MLS deployment that actually *ships* is
**server-ordered** — a centralized delivery service picks the winning commit: **Webex**, **Wire**,
**Discord**, and the **Google/Apple RCS MLS-E2EE** rollout (May 2026). The serverless approaches above are
drafts and proofs-of-concept, not shipping systems. That is what makes the narrow claim airtight: **no
*production* MLS deployment is serverless-ordered.** `[web 2026-06-26, confirm before publish.]` The value of
that anchor is precise — it lets Croft say its serverless ordering is *different, not weaker*: different
because no shipping system does it this way, not weaker because the difference is a deliberate choice with a
measured cost (the FREEK curve above), not an unproven gamble.

## What this establishes (and does not)

Establishes that Croft's capability-not-authority layer has a clean formal frame with real prior art — the
object-capability discipline (*designation is authorization*, POLA, petnames) carried by Spritely's Goblins /
OCapN / CapTP — and that Spritely's non-extractive funding model is itself prior art for the sustainability
question; and that Drystone's serverless ordering has named siblings (DMLS/FREEK, `draft-xue-distributed-mls`)
that both quantify the fork→forward-secrecy cost it incurs and anchor the airtight claim that no production
MLS deployment orders without a server.

Does **not** re-file the capability-mechanism decision itself — Keyhive (Track B) and Meadowcap (Track A)
are that decision (A11) and live in `willow-meadowcap.md`; does **not** re-document the MLS core or MIMI
(`mls-and-mimi.md`); and does **not** certify the flagged rows — the decentralized-MLS and MLS-in-production
facts carry a "confirm before publish" flag, the Spritely facts are `[UNVERIFIED current]`, and the
Keyhive/Meadowcap references are `[dialogue-sourced 2026-06-24, pending verification]`. All three need a
refresh pass against primary sources (IETF drafts, the FREEK paper, Spritely's own materials) before external
use.
