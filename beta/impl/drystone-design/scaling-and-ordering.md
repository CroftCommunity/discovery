# Drystone: Commits, Folds, and the Relocation of Ordering Cost

`Status: draft, written to survive a hostile read. Every load-bearing sentence carries an epistemic tag (legend below). The central governance-scaling claim is conditional on a convergence property that is currently unearned; this is stated as such, not softened.`

`Scope: what a Drystone deployment forces through MLS's two scaling bottlenecks (commit serialization and the Delivery Service) compared to a typical messenger, why most of Drystone's ordering happens off the epoch chain, and where the cost that was removed from the chain actually went. Consolidates what currently lives implicitly across the epoch/governance decoupling (asset-keying.md §H), the deterministic governance fold (Part 2 §7.3.1), and the tiered delivery model (history-durability.md).`

`Companion to: asset-keying.md, history-durability.md, ../../drystone-spec/part-2-certifiable-design.md, authority-and-complement.md.`

`Terms: commit (an MLS Commit, the message that advances an epoch and rekeys); epoch (an MLS key-schedule generation); Delivery Service or DS (the MLS role that serializes and fans out commits); fold (the governance conflict-resolution function, fold-semantics.md); order-independence (the property that any delivery order of a complete fact set folds to the same result); gap-completeness (a node's ability to tell whether it holds the complete causal set). Tags are defined in the Epistemic legend below.`

## Epistemic legend

This note distinguishes five statuses, because flattening them would hide which claims are risks and which are scope boundaries.

- `Verified-RFC`: a protocol fact checked against RFC 9420 or RFC 9750. Solid and external.

- `Measured`: a figure from a single measurement study (Soler et al. 2025), obtained under specific conditions (an OpenMLS implementation, groups up to about 5,000 members). An observation under those conditions, not a protocol guarantee, and not to be read as a settled constant.

- `Design`: a Drystone architectural decision. Taken as given for this note. Not externally verifiable here, but not in question either. A scope boundary, not a risk.

- `Load-bearing, unearned`: a property the scaling claim *requires* that is not yet established. Falsifiable. If it fails, the claim it supports collapses. A verification path exists (see Verification), but the property is unearned until that path is walked. These are the risks, and they are flagged as risks.

- `[confirm]`: a smaller item resting on an external fact or a planned check.

## What this note does not claim

Leading with the disclaimers, because the strength of the argument is entirely in scoping and relocating cost, not in any conjured efficiency. This note does not claim any of the following:

- **It does not beat MLS cryptographically.** TreeKEM key agreement is already cheap and near-logarithmic in the good case; the cryptography was never the limiter, so there is no crypto win to claim. `Verified-RFC.`

- **It does not offer a free lunch on ordering.** The ordering work is not eliminated. It is relocated from central slot-contention to distributed, per-node deterministic computation plus convergence bandwidth. That relocation scales out and has no center, but it is not free, and the note treats the relocation as the claim, not as a cost avoided. `Design.`

- **It does not solve the giant single-group case.** A single MLS group still pays a ratchet-tree cost linear in its membership (below). The topology mitigates this by keeping groups scoped, but that mitigation is conditional (see Topology), not a solution to arbitrarily large single groups. `Verified-RFC.`

- **It does not offload MLS cryptographic-state catch-up, only content-history catch-up.** A joining member must still ingest the group's ratchet tree, which is MLS group state carried in the Welcome and GroupInfo objects and scales with member count. Drystone's tiered delivery offloads the *content* half of catch-up to a content-blind store; the *cryptographic-state* half remains MLS's problem and is not offloaded. `Verified-RFC.`

And the central caveat, stated once here and again where it bites: the governance half of the bottleneck-2 claim is **conditional on the governance fold being order-independent, a property that is currently unearned.** It is not "conditional but we are confident." Confidence is what the verification produces; designing the verification does not confer it.

## Bottleneck 1: commit serialization

MLS advances the group key through a strict chain of epochs, and only one commit may close each epoch, so concurrent commits collide and all but one must rebuild and retry. The load on this single slot is not activity, it is specifically the rate of *key-changing* operations, because only those become commits; application messages never touch the epoch chain. `Verified-RFC.`

The question for any deployment is therefore what fraction of its events are key changes. Drystone's answer differs sharply from a messenger's, by design.

- **The content plane is not commits.** Posts, comments, likes, replies, profile edits, moderation-log entries, and votes are per-author writes converged out of band, not MLS commits, and not even MLS application messages. So the highest-volume traffic contributes nothing to the slot. This follows from RFC once the content plane is kept off MLS, and keeping it off MLS is the `Design` decision (history-durability.md).

- **Authority-only governance is not commits.** Granting a role, changing a threshold, amending policy: these fold as governance facts and advance the governance generation counter, not the epoch, so they are not key changes and not commits. This is the epoch/governance decoupling. `Design` (asset-keying.md §H), with the consequence that they do not hit the slot following by `Verified-RFC` reasoning once the decoupling is granted.

What remains that actually hits the slot is only genuine key changes: a member added, a member removed, a post-compromise key rotation. So a Drystone group's commit rate is its membership-change-and-rotation rate, not its activity rate and not even its governance rate. A large group in vigorous discussion with constant moderation and role changes is, from the epoch chain's view, nearly silent. `Verified-RFC` for the reduction, given the two `Design` decisions above.

One reduction is Drystone-internal and should be read as such: a membership *removal* is both a governance fact and an enforcing commit, and the two-phase removal separates them so that concurrent removals resolve by the governance order without slot collision, colliding only if their enforcing commits land on the same epoch. This narrows the collision surface to the key-enforcement step. `Design` (asset-keying.md §H); unverified externally here, and load-bearing only for the "concurrent removals rarely collide" sub-claim, not for the main reduction.

## Bottleneck 2: the Delivery Service, split into its two jobs

The DS does two things that are worth separating, because Drystone's relationship to each is different. It *orders* concurrent commits (picks the single winner per epoch), and it *fans out and delivers* messages. RFC 9750 leaves the DS unstandardized and describes fan-out as client-side, server-side, or mixed, so how it scales is the application's engineering problem, not a protocol guarantee. `Verified-RFC.`

### The ordering job: relocated, and conditional on a property not yet earned

For the high-volume planes, Drystone is built to need no single orderer, because ordering is a property of the data rather than a decision imposed by a referee.

- The content plane converges by set union over content-addressed chains, which is order-insensitive: a grow-set does not depend on arrival order. `Design.`

- The governance plane converges by a deterministic monotonic fold whose result is specified to be a function of the folded set, not of delivery order (Part 2 §7.3.1). `Design` for the specification.

If those hold, concurrent governance and content events do not contend for a slot; they simply exist and fold identically on every node, so no DS decision is required for them, and the DS's ordering job shrinks to the small residue of actual commits.

Here is the load-bearing beam, stated plainly and not softened. The governance half of this claim is **conditional on the fold being genuinely order-independent, and that property is not yet earned.** `Load-bearing, unearned.` If two governance events can fold to different results under different delivery orders, then honest nodes can diverge, and the system needs a referee to reconcile them, which puts a DS decision back in the governance loop and collapses the "no DS ordering needed for governance" claim. The status of this beam has improved but not closed. The §7.3.1 conflict-resolution semantics have since been resolved (fold-semantics.md, rules R1 through R4: a per-slot causal last-writer-wins register, a tiebreak restricted to genuine concurrents, cross-slot effects as projections on final sets, and no fold-time validity rejection). Under those rules the fold's order-independence is *constructive*: it follows from the model rather than being asserted, because every slot's value is a pure function of the fact set and its causal DAG and the projections are pure functions of the resolved slots. So the risk has narrowed from "is the fold order-independent at all" to a single remaining condition: **gap-completeness**, meaning a node can tell whether it holds the complete causal set to fold, since an undetected gap could hide a causally-later fact and change a slot's resolved value. That single condition is still `Load-bearing, unearned`. The correct reading is now: *given* gap-completeness, the governance ordering cost is relocated off the chain, and order-independence follows by construction; gap-completeness itself is corroborated, not proven (the dataplane checkpoint and completeness-ahead corroboration), and until the convergence experiment demonstrates gap-detection, this remains conditional and unearned, not a result. Designing and resolving the semantics narrowed the risk; it did not earn the claim.

The content half is on firmer ground, because union convergence is order-insensitive by a much weaker argument than the governance fold requires, but it inherits the same gap-detectability dependency: a node must know whether it has all the entries, or it can present an incomplete union as complete. `Design` for union convergence; `Load-bearing, unearned` for gap detectability.

The relocation, stated as the claim: Drystone does not remove the ordering work, it moves it from central slot-contention to distributed per-node deterministic computation plus the bandwidth of convergence. That scales out and has no center, but it is real work and shows up as convergence traffic and per-node fold cost rather than as slot contention. This is the honest and unassailable form of "deterministic yet scaled-out ordering": not a free lunch, a relocation.

### The fan-out and delivery job: the ~2-second window, entered rarely, and the catch-up seam

Delivery latency is where the DS cost is real and Drystone does not escape it, only enters it less often.

A commit is not done when a CPU finishes it; it is done when the DS has ordered it and delivered it to everyone, and during that interval the group sits in a transient inconsistent state. One measurement study observed that window reaching up to about two seconds, dominated by the DS and network rather than the millisecond-scale cryptography. `Measured` (Soler et al. 2025, OpenMLS, up to ~5,000 members): this is one study under specific conditions, not a protocol constant. Any system delivering a key change to N members over a network pays some such window; Drystone inherits it *per commit*, and because commits are rare in its traffic mix (membership churn only), it is in that window rarely rather than constantly. `Verified-RFC` for the inheritance; the rarity follows from the bottleneck-1 reduction.

Content delivery does not go through the commit-ordering path at all: a post propagates by the convergence layer (live MLS application message for freshness, out-of-band history convergence for completeness), so content latency is decoupled from commit latency. `Design.`

The catch-up seam, stated precisely because an MLS-literate reader will look for it. Catch-up has two halves. Content-history catch-up (the record of posts, comments, governance facts) offloads to the content-blind store by chain reconciliation, off the MLS DS. `Design.` Cryptographic-state catch-up does not: a joining member must ingest the group's ratchet tree, which travels in the Welcome and GroupInfo objects, is MLS group state rather than content, and scales linearly in member count because the tree has a node per member. `Verified-RFC.` So the tiering offloads the unbounded-over-time half (content history) and not the membership-linear half (the tree). A content-blind store can serve sealed history blobs; it cannot serve or reason about the ratchet tree, and it should not be claimed to.

## Topology: many small groups, and the cross-group cost it trades against

The ratchet-tree cost being per-group and linear in that group's membership is a reason to prefer many scoped groups (a group per family, club, or channel) over one enormous group, since the linear cost is then bounded by each group's smaller membership rather than by a single large one. `Verified-RFC` for the per-group linear cost.

But the topology has a cross-group cost that must be named, or the win is asserted rather than shown. Turning one large group into many small ones means something spans them: a persona in fifty groups is in fifty MLS groups. Fan-out to that persona is now fifty groups' worth of delivery, and catching that persona up is fifty trees rather than one. The cost has moved from one tree of size M to N trees of smaller size, and whether that is a win depends on whether the sum of the small trees is smaller than the one large tree. It usually is, because trees are per-group and most personae are not in most groups, so the sum is sparse. But this note states the small-groups advantage as *conditional on groups staying scoped and personae not being in pathologically many groups*, and does not assert it unconditionally, because Drystone's actual group-membership distribution is not something established here. `Design` for the topology; the win is `[confirm]` pending a realistic membership distribution.

## Honest costs, collected

- **Membership-change bursts still serialize as commits and still enter the inconsistency window.** Two hundred joins in a minute are two hundred key-affecting events. The intended handling is the same as the production systems: a designated committer that *batches* adds and removes into far fewer commits, so a join storm becomes a handful of batched commits rather than a storm of collisions. Webex and Cloudflare use a designated committer for this (from the research report; secondary, not re-verified here), and it composes with Drystone's peer-symmetry because the committer is a mechanical role that needs no authority and cannot foreclose or govern, so it is a delegated-capability helper, not a center. `Design` for the composition; the industry practice is secondary-sourced.

- **The ratchet-tree half of catch-up is linear in group membership and is not offloaded.** The mitigation is the scoped-groups topology, itself conditional (above).

- **The relocated ordering cost is real.** It appears as convergence bandwidth and per-node fold computation. It scales out and has no center, but it is not zero.

- **The governance scaling claim is conditional and currently unearned, but the risk has narrowed.** With the §7.3.1 resolutions (fold-semantics.md, R1 through R4), the fold's order-independence is now constructive rather than asserted, so the remaining unearned beam is the single condition of gap-completeness: a node being able to tell whether it holds the complete causal set. That condition is still not established, it is corroborated not proven, and it is the largest single caveat and the one most likely to be probed. The extended Stage 2 gap-detection experiment is its discharge path.

## Summary posture

| Claim | Status | Rests on |
|---|---|---|
| Commit rate is key-change rate, not activity rate | `Verified-RFC` | RFC 9420 |
| Content plane and authority-only governance are not commits | `Design` + `Verified-RFC` consequence | asset-keying.md §H, history-durability |
| DS splits into ordering and fan-out, unstandardized | `Verified-RFC` | RFC 9750 |
| Content ordering needs no referee (order-insensitive union) | `Design`, with gap-detectability `Load-bearing, unearned` | §7.3.1, open items |
| Governance ordering needs no referee | `Load-bearing, unearned`, narrowed | constructive given gap-completeness (R1-R4, fold-semantics.md); gap-completeness not yet demonstrated |
| Ordering cost is relocated, not eliminated | `Design` | the convergence architecture |
| ~2s inconsistency window, entered per commit | `Measured`, rarity by reduction | Soler et al. 2025 |
| Content-history catch-up offloads to a blind store | `Design` | history-durability |
| Ratchet-tree catch-up does not offload, is linear in membership | `Verified-RFC` | RFC 9420 Welcome/GroupInfo |
| Small-groups topology bounds the tree cost | `Design`, win `[confirm]` | membership distribution, unestablished |
| Membership bursts handled by batched designated committer | `Design` composition, secondary industry practice | Webex/Cloudflare (research report) |

## Open items

`Distinct, per the doc-method. These restate the unearned and unresolved points flagged inline, gathered in one place.`

- **O1. Gap-completeness.** The single `Load-bearing, unearned` property the governance-scaling claim rests on: a node being able to tell whether it holds the complete causal set to fold. Order-independence is constructive given it (R1 through R4, fold-semantics.md); it is itself corroborated, not proven (governance-finality.md B3). Its discharge path is the extended Stage 2 gap-detection experiment. `Load-bearing, unearned.`

- **O2. Content-union gap-detectability.** The content half inherits the same dependency: a node must know it holds all entries, or it can present an incomplete union as complete. `Load-bearing, unearned.`

- **O3. The measured window is one study.** The roughly two-second commit-serialization window is `Measured` from a single study (Soler et al. 2025) under specific conditions, not a constant; deployment figures may differ. `[confirm.]`

## Verification

The governance scaling claim is discharged, if it can be, by the convergence experiment: property-based permutation-invariance of the fold (does any permutation of a complete fact-set yield an identical result), deterministic-simulation of concurrent interleaving and gap-fill (do nodes converge, and does a node with a gap *detect* it rather than fold an incomplete set as complete), and adversarial and bounded model-checked schedules (can a malicious orderer or an equivocator force honest divergence). Until permutation-invariance and gap-detectability pass, the governance ordering claim remains conditional and unearned, and this note should be read with that claim marked open. The experiment is the path to earning it; it is not itself the earning.

One half of this is now empirically discharged. A separate proof ran the reconcile computation on three genuinely separate machines with no superpeer and no orderer and produced a byte-identical verdict, invariant across every merge order tested (a three-way fork that converged regardless of order). That earns permutation-invariance of the fold-reconcile on real hardware, so that half moves from `Load-bearing, unearned` to `Verified` for the reconcile computation (fold-semantics.md records the same result). Gap-detectability is untouched: the proof models the partition as op-log exchange rather than a live transport partition, so the completeness half stays `Load-bearing, unearned` and the governance ordering claim overall remains open until it too passes. See the reference-index (Proof: cross-machine deterministic reconcile).

## Changelog

`Working draft; transitions recorded here per the suite's doc-method.`

- **Draft, first consolidation.** States the two MLS bottlenecks and what Drystone forces through each, framed as scoping (bottleneck 1: almost nothing is a commit) and relocation (bottleneck 2: ordering moved off the chain to deterministic per-node computation). Leads with four disclaimers including the catch-up seam. Marks the governance ordering claim as conditional and unearned, resting on fold order-independence, with the convergence experiment as the discharge path.

- **Tag posture.** Five statuses used deliberately: `Verified-RFC` for RFC 9420/9750 facts, `Measured` for the single Soler et al. 2025 figure (not a protocol constant), `Design` for Drystone architecture taken as given (scope boundaries, not risks), `Load-bearing, unearned` for properties the claim requires but that are not established (the risks: fold order-independence, gap detectability), and `[confirm]` for smaller pending items (the small-groups win pending a membership distribution). The distinction between `Design` and `Load-bearing, unearned` is intentional: the first is architecture in hand, the second is a risk with a verification path.

- **Doc-model standard pass.** Added a Terms block and a consolidated Open items section (gap-completeness, content-union gap-detectability, the single-study measured window) restating the inline-flagged residuals in one place. No claim status changed.
