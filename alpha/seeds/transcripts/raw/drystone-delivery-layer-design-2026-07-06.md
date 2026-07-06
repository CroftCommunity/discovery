# Raw transcript — Drystone delivery-layer design: atomic-swap re-plant, CALM grounding, center-free framing (2026-07-06)

**Preservation status: preserved-condensed (cleaned-paste, content-faithful — §4).** User-pasted tail of a
multi-thread claude.ai session (batch seven). UI render chrome stripped ("Searched the web", "Created a
file", "Presented N files", "Document · MD", "Show more"); repeated fetch/edit narration condensed to
substance. Content-faithful, not a byte-pristine export. The durable outputs are the 13-doc delivery-layer
corpus (00–12), filed verbatim at `beta/impl/delivery-layer/`; this file preserves the reasoning threads and
the user's prompts behind them, several of which (the atomic-swap logic, the tree-is-disposable insight, the
CALM grounding, the center-free framing) are the "why" the corpus does not spell out in full.

Corpus produced (filed 2026-07-06, batch seven): `beta/impl/delivery-layer/00-12`. The messaging-layer
research prompt that seeded this thread was filed earlier at
`seeds/generated-prompts/drystone-messaging-layer-research-prompt.md` (batch six).

---

## Thread A — the atomic-swap re-plant (the core design move)

**User:** Our governance chain has every current member at N; we don't have to replay, we do an atomic swap.

**Assistant:** That inverts the dependency correctly. Governance is the membership authority; MLS is
downstream of it. At boundary N the governance chain says "these are the members"; you plant a fresh MLS
tree over exactly that set and close the old group. No replay, no reconstruction from MLS state. The
resumption PSK is not carrying membership truth; it cryptographically confirms a continuity the governance
chain already asserts, two independent attestations of the same fact. This kills the replay-cost and
replay-divergence worries, and the fork risk nearly evaporates: the boundary is deterministic (every N) and
the member set is read from the governance chain (not the planter's judgment), so competing plantings
either produce the same member set (content-hash tie-break dedups them) or surface as a governance-chain
disagreement the existing machinery already resolves. Residual care: membership + timing are deterministic,
but tree *bytes* are not necessarily bit-identical across planters (different KeyPackages for the same
member), so it is "deterministic in membership and timing, tie-broken in tree bytes."

**User:** We don't use tree ordering for anything other than logistics of scale. Dataplane is a hash tree;
current governance chain and history governance chain are hash trees. All independent, not interlinked
except through application-layer logic. So tree reconstruction just adds each member to a new group and
stamps out at N with the governance current. The old tree is cut down, membership stays associated in the
app seamlessly, history too. The whole thing is ease of fork and merge and managing messaging history as
durable trees.

**Assistant:** Right, and that removes the worry entirely. The MLS tree does exactly one job: logistics of
scale for key distribution. Nothing downstream reads its shape, leaf order, or which KeyPackage seeded which
leaf. Continuity lives in three independent hash structures (dataplane, current-governance, history-
governance), none keyed to the tree. So two planters emitting different tree bytes over the same membership
is invisible to everything that matters; the tie-break is not resolving a fork, just keeping one of two
equivalent key-distribution trees. The tree carries neither identity nor continuity; its bytes are as
disposable as a TCP sequence number. The operation is: read current membership from the governance chain,
add each member to a new group, stamp it out seeded with current governance, cut down the old tree. MLS is
demoted from "the group" to a subordinate key mechanism under the app's durable hash structures.

## Thread B — grounding the instantiation mechanics (primary sources)

Confirmed against the MLS architecture/protocol and mls-rs/openmls:
- Group creation is unilateral and needs a KeyPackage per member; to add members you download their
  KeyPackages, batch Adds into a Commit, and emit Welcomes. So "stamp out at N" is an O(N) instantiation
  paid once at the boundary.
- KeyPackages are single-use and SHOULD NOT be reused, except a **last-resort KeyPackage**, which exists
  precisely so a member with no fresh package published can still be seated. That is the answer to "what if
  a member isn't reachable at N": seat them on last-resort; the swap never blocks.
- Because each fresh group draws a fresh KeyPackage per member, the re-plant **rotates every member's leaf
  key**, i.e. a group-wide key refresh (the favorable post-compromise-security answer, with last-resort
  seating the one honest exception).
- Both mls-rs and openmls ship reinit/branching natively, so it is buildable on the validated stack.
- The one genuinely new center-free constraint is KeyPackage availability at the boundary, the input to
  tuning N (bigger N amortizes the O(N) instantiation over more messages; smaller N keeps trees tight but
  pays instantiation more often).

This produced `12-replant-experiments.md` (seven experiments E12.1–E12.7, tagged on the doc-08 fidelity
ladder: Rung A on real mls-rs for the MLS mechanics, Rung B for Drystone's own hash structures that do not
exist yet). Held as `[confirm]`: whether mls-rs exposes ReInit as first-class emitting the resumption PSK vs
fresh-create-plus-manual-PSK, and whether resolution size is directly observable for the blank-node
experiment.

## Thread C — center-free is plurality, not absence of relation (the founding framing)

**User:** For Drystone concepts, center-free is not without relation, but it is subjective, because it is
fixed on plurality, like raindrops falling on a lake: each is the center of its own ripple, none is the
center of them all. There is some CRDT use for sure.

**Assistant (after grounding CRDT/SEC):** CRDT convergence is scoped and conditional, not global: strong
eventual consistency guarantees that replicas which have applied the *same set* of updates reach the same
state, per shared object among its participants, with no peer as the reference. That is compatible with
equal standing (convergence-among-equals-on-a-shared-object, not convergence-to-a-center). SEC's convergence
is conditioned on having the same set of updates and says nothing about detecting completeness, which is
exactly the governance tail-gap.

**User:** I don't want to hang our hat on CRDT; yes it's in the stack but too technical and too narrow.

**Assistant:** Agreed, and for a reason beyond narrowness: CRDT is a mechanism ("how"), and the founding
paragraph is at principle altitude ("what and why"). Naming it drags an implementation choice into the
frame and invites the reader to reason from the mechanism instead of the principle. State the property
directly, shared state with no privileged reference, and let "how" live in the mechanics half.

**User (successive refinements):** All peers do have a copy, and to them their copy is authoritative. Then:
they converge on governance facts and act on their own local state; "copy" is too much.

**Assistant (final framing):** The clean fence is two-layer, and "copy" is dropped (it implies an original,
smuggling back the center): **peers converge on governance facts** (who holds what authority, what has been
committed, membership, rights), **and each peer acts on its own local state**. No peer waits on the network
to act; no peer's state is subordinate to a master. Convergence is at the governance layer; local authority
is at the operating layer. The tail-gap lives at the governance layer: a peer can act in good faith on its
local state under a governance picture missing events committed elsewhere, and cannot, on its own, tell,
which is the epistemic gap the checkpoint closes. (Open calibration left to the doc set: the exact canonical
noun, "governance facts" vs "governance events" vs "committed set"; and whether the checkpoint is landed
(present tense) or still an open requirement.)

## Thread D — CALM theorem grounding (for the delivery-layer references)

**User:** Can you find the primary references for CALM? [draft sentence: "The CALM theorem (Hellerstein &
Alvaro) even proved the exact boundary: a problem has a consistent, coordination-free distributed
implementation if and only if it is monotonic."]

**Assistant (verified against primaries):**
- Verbatim (CACM, *Keeping CALM: When Distributed Consistency is Easy*): "THEOREM 1. Consistency As Logical
  Monotonicity (CALM). A problem has a consistent, coordination-free distributed implementation if and only
  if it is monotonic." The biconditional checks out.
- Primary refs: Hellerstein & Alvaro, arXiv:1901.01930 (2019) and CACM (2020, the numbered Theorem 1).
- **Attribution nuance:** conjectured by Hellerstein (2010 PODS keynote); stated/popularized by Hellerstein
  & Alvaro (Keeping CALM); **formal proof** of the original boundary by **Ameloot, Neven & Van den Bussche
  (2013)** via relational transducer networks. If "proved" means "produced the formal proof," credit Ameloot
  et al.

**User:** Monotonic means ordered by time increment? I thought CALM said timestamps in a distributed system
always cause coordination problems because time is local to each node.

**Assistant:** No, monotonic in CALM has nothing to do with time. A program is monotonic if more input can
only add to its output, never retract (S ⊆ T ⟹ P(S) ⊆ P(T)): conclusions only accumulate. It is about
information growth. The paper's example: deadlock = existence of a cyclic path (monotonic, cannot be
un-found); garbage = non-existence of a path (non-monotonic, can be retracted when more edges arrive). CALM
shifts consistency to deterministic outcomes rather than ordered histories; a monotonic program is confluent
(same result for any order/batching), so it needs no consistent time ordering. Timestamp logic often needs
coordination because last-write-wins ("is this the newest?") is non-monotonic, not because of timestamps as
such. [The LWW framing flagged as synthesis from the definition, not a quoted claim.]

**User:** So if the chain only moves forward, probably in a consensus, then the consensus is the
coordination.

**Assistant:** Careful, that is circular. Two different "forwards": monotonic (the set of conclusions grows,
a property of what is computed, no ordering required) vs chain/consensus forward (nodes agree on a single
sequence, a property of how the system runs, requires waiting for agreement). Monotonicity is what lets you
*skip* consensus. Consensus (Paxos, Raft, a blockchain's total order) is the coordination you are *forced*
to build when monotonicity fails, the cost of non-monotonicity, not a picture of it. A blockchain is a
coordination engine; the problems it solves (double-spend = a non-existence claim, "one true history") are
non-monotonic. [Blockchain/double-spend mapping flagged as synthesis consistent with the definition.]

*(The CALM verification produced a standalone `calm-session-summary.md` in the source session; it is not in
`seven-grounding.zip`. The CALM grounding is reflected in the corpus's `02-references.md`; if the standalone
summary is wanted in the tree it can be filed when provided.)*

---

*End of preserved tail. The next session (Claude Code, filing) created `beta/impl/` (Layer 4) and filed the
13-doc corpus verbatim to `beta/impl/delivery-layer/`, added the layer README, and recorded the intake. The
center-free "raindrop / plurality" framing and the two-layer (governance-converges / local-state-authority)
fence are preserved here as durable concepts; more Drystone changes are expected (batch 7 of 11), so no spec
or philosophy doc was modified to absorb them this session.*
