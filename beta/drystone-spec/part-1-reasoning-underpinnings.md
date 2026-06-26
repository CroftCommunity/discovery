# Drystone — Part 1: Reasoning Underpinnings

`Status: beta`

`Defines: P-Local-Truth, P-Knowable-Truth, P-Peer-Equality, P-Durable-Enablement (the principles Part 2 realizes)`

This part is the "why." It is prose, deliberately, and it earns its place by ending every principle on a
consequence the mechanics in Part 2 must satisfy. Each principle states a commitment, gives its grounded
reasoning, and names the obligation it places on the wire. A principle that does not cash out into a
mechanic does not belong here.

---

## 1. Introduction

A system that puts a server in the middle of people's relationships makes that server the arbiter of
what is true: it holds the canonical copy, decides what is current, and can certify, revise, or revoke
any of it. That arrangement is convenient until the operator's interests and the participants' diverge —
at which point the participants discover the thing they were standing on was never theirs. Every layer
that depended on the center (who you are, what was said, who may act, who can be reached) fails at once,
and there is no local ground to fall back to.

Local-first restores that ground: the primary copy lives with the unit that authored it, complete and
usable alone, and the system reconciles between units rather than constituting them. **But local-first is
not merely an engineering preference about where bytes live — it is forced by what a distributed system
can and cannot know.** This part argues that claim and follows it down into the obligations the protocol
must meet. Therefore the protocol must be designed so that no participant ever has to take the center's
word for the state of the world, because there is no center whose word could be taken.

A **distributed system**, throughout this specification, is a set of **nodes** communicating remotely
about some shared state. No node can prove it holds complete and current knowledge of the others.
That single fact — not a value judgment, a structural limit — is where the principles below come from.

### 1.1. Scope

This specification defines Drystone, the protocol, and nothing above it: no governance policy for any
particular community, no application semantics, no product. Part 1 defines the principles; Part 2
defines the mechanics that realize them. Where a principle has political or economic consequences beyond
the wire, those are out of scope for this specification. Keeping the spec tethered to mechanics is what
stops the principles from drifting into general theory.

Drystone is **vendor-neutral by construction.** Ecosystems, applications, and hosting operators may be
built on it — Croft is one such ecosystem, and is intended to be one of several rather than the only one —
but no named ecosystem, brand, or operator is part of the protocol's definition. Where this document
refers to a consuming ecosystem it does so as an example, never as a dependency.

---

## 2. Design Principles

### 2.0. The razor: compute provenance, never utility

This is the master claim the four named principles descend from.

**A node in a distributed system can only ever establish *provenance* — what is consistent and
corroborated — never *truth*.** The distributed system exists to facilitate nodes acquiring the view
they need; it is not itself a knower. A node sees two things: **assertions** (what peers say, signed and
positioned) and **agreement patterns** (who corroborates whom). It never sees the world the assertions
are *about*. So "is this true?" is a question about something structurally outside any node's reach. The
only questions answerable inside the system are whether a node's view is internally consistent (**local
consistency**) and who else asserts the same (**alignment**) — neither of which is a truth claim.

From this follows one design imperative, the razor:

> **Nodes establish, through the system, what is consistent and corroborated — *provenance*. Humans, at
> the edges, supply what is true and right — *utility*.**

Provenance is a closed, determinate question: *given a record, does its chain of signatures and
references verify?* — a question a node can answer alone, with no appeal to any authority. Utility is
open in principle: it lives in plural, revisable human values, and no node can compute it. A system that
tried to certify utility as if it were provenance would have to encode some fixed conclusion — often a
single one — as objective fact, and in doing so would marginalize every node that corroborated a
different local state, including the dissenter later judged, by humans, to have had the better view. (We
say "later judged" deliberately: a node cannot be shown *objectively* right against the others, only
subsequently corroborated or preferred. The razor does not promise that truth is discoverable; it
promises that disagreement is survivable.) **The honest mission is therefore not truth, but trustworthy
disagreement** — and that single line is the whole protocol's posture, lifted to a principle about
knowledge itself. Part 2's refusal to algorithmically adjudicate a social dispute (the *no-adjudication
rule*, §7) and its content-blind safety posture (§8) are this same razor at the mechanism layer.

**The razor extends to identity, and this is where most systems quietly cheat.** Cryptography answers
*provenance* with certainty — "is this the same key that made that earlier assertion" — but it cannot
answer "is this key the person I mean." **That second question is a utility call, not a provenance one,
and it is always a human judgment.** All trust ultimately roots in social trust: even the root
certificate authorities that secure the rest of the web are trusted because a community of people vouches
for them and polices them, not because of any cryptographic fact at the bottom. A deliberate human act —
scanning a code to add a device, exchanging keys in person — does not *establish* trust; it **binds
existing trust to a key** so that provenance can carry it forward. Pretending cryptography *establishes*
trust rather than *recording and amplifying* a human decision is itself a failure mode, because it leads
people to trust the math exactly where the human judgment was weak. So the protocol's job is to give
**certain provenance** as a substrate and to leave **graded, contextual trust** to humans — never to
compute a trust verdict. Where the system carries trust signals between peers (one peer vouching for
another), those signals **MUST** be graded, **MUST** be indexed by purpose ("trusted for *what*"), and
**MUST NOT** be treated as automatically transitive or as an automatic grant — they are input to a human
or policy decision, never a verdict. (This is the lesson of PGP's web of trust, which failed by making
trust scalar, context-free, and automatically transitive; the fix is to keep a vouch an honestly-weighted
signal, never a computed conclusion. See Part 2 §5 `vouch` and `get_trust_signals`.)

The four principles below are what the razor requires of the wire.

### 2.1. P-Local-Truth — the only canonical state is local

**Commitment.** A device can honestly know exactly one thing: its own local state. Its local store is
**canonical for that device** — the only state it can *prove* rather than infer. Any belief about
another node's state is **comparative** (a reconciliation against another node's asserted state) or
**asserted** (a signed claim it has accepted), never canonical. There is no global canonical state, by
design, because a claim of global state is a claim no node in a distributed system can honestly make.

**Reasoning.** This is the same sentence as the epistemology, said in engineering terms: *the primary
copy lives with the unit; the network reconciles* is the same concept restated as *truth is local and
corroborated, never certified from a center.* Because the architecture and its justification run on the
same engine, the protocol earns belief the way it describes belief being earned. Just as a person can
only decide from the experience they actually hold, every node decides from the local state it actually
has at the time; the system's job is to keep that state honest and to deliver more of it, not to
substitute a center's state for the node's. Local state is the unit of decision-making because it is the
only state any participant can stand on.

**Consequence.** Therefore the protocol **MUST NOT** depend on any node holding privileged or canonical
state. A node that has seen fewer facts **MUST** compute a *stale-but-honest* view, never a false one.
The comparative and asserted layers are how nodes converge; the local layer is the only thing any of
them stands on. *(Part 2 §4, §5.1, §7.)*

### 2.2. P-Knowable-Truth — verify for yourself; the record is auditable, never silently mutable

**Commitment.** What a node takes on trust must be minimized and what it can verify for itself must be
maximized. A node verifies an entry's integrity and authorship without consulting any central authority,
and the history a node relies on is **append-only and auditable** — facts accrete and can be audited;
they are never silently revised or reset to an earlier value.

**Reasoning.** Certainty *for a node in a distributed system* is unreachable: a node cannot know what
else is happening beyond its own view and can only work from what it has. The honest response is not to
manufacture certainty but to make the record self-describing, so that what a node cannot personally
witness it can at least cryptographically check, and so that no decision can be quietly unmade. The
provenance the razor leaves us — a survivable record of what was asserted and corroborated — is exactly
what a system that cannot certify truth *can* honestly compute.

**Consequence.** Therefore state **MUST** be cryptographically self-describing (integrity and authorship
checkable from the bytes), and authority **MUST** be a monotonic fold over an append-only fact set rather
than a recomputed mutable value — so a lagging node under-authorizes (acts on less) but never
mis-authorizes (acts on a reverted or forged state). Every superseded or stale decision **MUST** remain
in the record, attributable. *(Part 2 §4.3, §7.3, §8.)*

### 2.3. P-Peer-Equality — equal in rights, not in capabilities

**Commitment.** There is **one kind of peer**. Peers differ in two separate things that the protocol
keeps strictly apart: **what a peer is entitled to** (rights — universal, identical for all, never
delegated) and **what a peer has been granted the means to do** (capabilities — additive, delegated,
revocable). A bigger peer can *do* more; it cannot *decide* more. The moment a capability difference
becomes a rights difference, the design has leaked.

**Reasoning.** What separates a right from a capability is an intrinsic property, not a list: **a right
is standing that must survive for any dispute about it to remain contestable — remove it and the holder
loses the very means to object.** A capability — a role, a delegated authority, a moderation power,
write-access, a vote weight — is a power whose removal leaves the holder's standing to object intact
(lose admin on a scope and you can still contest the loss through your tenure, voice, and exit; that is
the tell it was a capability). Equality of rights is what lets equal nodes hold and reconcile divergent
views — which is the whole job of nodes that stand in for a human's experience. So the rights floor is a
**consistency-and-equality requirement, not a moral overlay**: it is what makes "no center can certify
truth" hold at the level of who-may-act, not just what-is-true.

This yields **one negative boundary** the whole protocol refuses to cross — *no peer may remove another's
standing to hold variety* — distinct from the several positive **rights** that define what a peer is.
Rights-removal is the only self-amplifying move toward collapse: it lowers the variety available to
resist the next removal, the way a monoculture lowers a system's capacity to absorb the next shock. The
discriminating test for any proposed action is whether, generalized, it would remove the conditions of
its own contestation; if so it is illegitimate by nature, and the tell is self-cancellation.

**Consequence.** Therefore equality **MUST** be enforced by mechanism, not convention. **Rights have no
presets; only capabilities do.** Every named configuration of a peer **MUST** be expressible as
`floor + [explicit capability set]`; any configuration meaning "this peer is entitled to *less*" is
rejected as a smuggled rights distinction. Delegation **MUST** attenuate (a subset of held authority,
never a superset). *(Part 2 §5, §7.)*

**A peer is recursively a group — and two edge types keep the recursion honest.** A peer is a *locus of
adjudication* (it holds authority others must respect), not merely a node that senses and relays; and the
same primitive nests: a **user is a group of devices**, a **community is a group of users**, a federation a
group of communities. The recursion is held together by **two distinct kinds of relationship that must
not be conflated**: **composition** — members co-deriving shared authoritative state (a device pool, a
scope's membership; this is the MLS-lineage / shared-key relationship) — and **valuation** — one group
directionally *weighting* another group's assertions without any shared key (trust between
cryptographically-separate principals). Composition merges state; valuation weights signals. Blur them and
trust leaks into key access. A consequence the rest of the design leans on: **adversarial posture is a
per-edge property, not a global stance** — your own device pool is a high-trust composition edge that needs
no Byzantine defense, while a stranger valuation edge needs more; forcing strong-adversarial rigor where it
doesn't fit is as wrong as omitting it where it does. *(Part 2 §3.1, §5.)*

> **Two checks left open before the rights set hardens** (carried, not resolved): whether `share` (a
> claim on a scope's commons) is fully a right or partly a membership-class capability, and whether the
> survivor / re-key path can strand a peer's `tenure`. These gate freezing the rights set into normative
> text; see Part 2 Appendix B.

### 2.4. P-Durable-Enablement — participation, and exit, must be real on a bare node

**Commitment.** Participation **MUST** be possible on a bare node — ordinary device, no purchased
infrastructure required — and any default delegation a peer or scope adopts **MUST** be revocable and
restructurable down to the rights floor at any time, with **no loss of rights** and only **graceful
degradation of capacity**. The enabling set is mostly restraints, not features: secure standing; a real,
cheap, dignified exit; an honest, non-equivocating record; resolution that defers judgment to humans; and
a refusal to optimize toward a single legible perspective. **The negative space — what the protocol
declines to do — is an intentional and critical part of the design.**

**Reasoning.** A delegated capability is only meaningfully different from a captive structural dependency
if the delegation can be withdrawn and restructured without loss of rights. A good default helper and a
server you cannot leave can look identical right up until trust fails — and that is exactly the moment
the difference must hold: the Drystone scope restructures or exits and loses only capacity; the captive
scope loses everything and starts over. The guarantee is measured not by how often it is exercised but by
being unconditionally available to the minority who do; a right you cannot afford to exercise elsewhere
is not a right you hold. Giving up the authority to decide outcomes is also the protocol's falsifiability
discipline: you can be observably corrected when the conditions you set fail to keep variety alive.

**Consequence.** Therefore the no-helper path **MUST** stay exercised and real (a scope **MUST NOT**
structurally depend on any single peer's presence to act), delegation **MUST** be materially reversible
(encrypted state the group holds keys to; a re-issuable grant, not a box), and the fork / re-formation
exit **MUST** remain available as the final backstop, preserving history and provenance to the point of
departure. *(Part 2 §5.4, §5.8, §6, §7.)*

---

## 3. Why these principles are corroborated, not invented

The razor is underrepresented in shipped systems, but it is not a wilderness. Thinkers who never
collaborated, arguing from different starting points, independently arrived at pieces of the same
conclusion. That cross-field corroboration is the strongest support the design has — and notably, *how
long ago* any of them wrote is not the argument. A claim is not stronger for being old; one node right
against all the others is still right, and correctness does not accrue with time. What matters is that
independent witnesses, with no shared agenda, describe the same structure. The grounding below leads with
the conclusion each discipline supplies, then the verbatim source as the outside corroboration, in the
spirit of: state the ground, then show who else, unprompted, found it.

Each quote is preserved whole, with its citation and a per-quote verification flag.

**Ethics — silencing the dissenter is a cost no center can justify.** This is the moral spine of *let the
dissenter fork.* Mill names the cost directly:

> "If all mankind minus one were of one opinion, and only one person were of the contrary opinion,
> mankind would be no more justified in silencing that one person, than he, if he had the power, would
> be justified in silencing mankind."
> — J. S. Mill, *On Liberty* (1859) · *Verified.*

So the fork must always stay available as the dignified exit — which is why the protocol refuses to
algorithmically adjudicate a social dispute (Part 2 §7) and refuses moderation-as-surveillance (§8),
accepting instead that each node determines its own relationship to moderation and to the outcomes of
adjudication.

**Economics — the knowledge a center would need cannot be centralized.** Decisions belong at the edges
because the relevant knowledge only exists there:

> "The peculiar character of the problem of a rational economic order is determined precisely by the
> fact that the knowledge of the circumstances of which we must make use never exists in concentrated or
> integrated form but solely as the dispersed bits of incomplete and frequently contradictory knowledge
> which all the separate individuals possess."
> — F. A. Hayek, "The Use of Knowledge in Society" (1945) · *Verified verbatim.*

"Incomplete and frequently contradictory" is a distributed system's actual condition, named in 1945.
Much of the judgment a human brings to a divergence is inarticulable local knowledge no center can
capture — which is *why utility cannot be computed* and decisions, and value judgments, belong at the
edges.

**Systems science — plurality is a survival condition, not a preference.** Plurality is a necessary
condition for the health of nodes that are equal peers and that, by nature, hold diverging views of what
is corroborated and what is merely asserted:

> "Only variety can destroy variety."
> — W. R. Ashby, *An Introduction to Cybernetics* (1956), p. 207 (the Law of Requisite Variety) · *Verified.*

A regulator that collapses plural perspectives into one certified truth is brittle by formal law: it
sits below the variety it governs, so it cannot represent every state, and it survives only by *pruning*
the variety it cannot hold — removing from the system the very parts reality will later require.
Preserving plural perspectives — allowing forks, refusing to collapse divergence — is requisite variety
made architectural. **Plurality is the robustness.**

Stafford Beer turned this law into a design discipline, and it is the formal grounding for Part 2's
*escalate-the-hard-case-to-a-human* posture. Beer's rule was to **not over-specify**:

> "instead of trying to specify in full detail, you specify it only somewhat. You then ride on the
> dynamics of the system in the direction you want to go."
> — S. Beer, *Brain of the Firm* (1972) · **[confirm before publish]** (defensible verbatim; confirm
> against the primary edition).

Because you have not specified every case in advance, the system will meet cases the rules never
anticipated — so you need a channel for those cases to surface. Beer named it the **algedonic** channel
(Greek *algos*, pain + *hedone*, pleasure): a low-bandwidth, high-priority alarm that **bypasses the
normal hierarchy and carries no analysis** — "this hurts, a human needs to look now." It is not a fallback
or an admission of failure; it is a designed channel, as load-bearing as the automated path, because no
filter is perfect and the cost of silently absorbing a missed hard case is too high. In Drystone this is
exactly the **hard-stop-and-escalate** rule (Part 2 §7.6) and the **label-not-enforce** posture (§8): the
machine *annotates rather than acts*, surfacing the signal and leaving the decision with a person — which
is also what keeps adjudication, and therefore peerhood, distributed (Part 2 §3, §5.2).

The historical natural experiment that grounds this is **Cybersyn vs OGAS** — two early-1970s attempts to
run an economy by computer, with opposite assumptions about *where judgment lives*. Beer's Cybersyn
(Chile) pushed autonomy to the edge and escalated only the residue to humans in a room; during the October
1972 truckers' strike it let the government move what mattered on **10–30% of normal transport capacity**.
The Soviet OGAS routed adjudication to a Moscow apex and was strangled before it ran — not by a technical
limit but by the institutional self-interest a single point of control invites. The lesson is not
"decentralized vs centralized infrastructure" (both had distributed sensing) but **where adjudication
lives** — which is the thread Part 2 §3 makes structural. (Beer's stance that the computer should serve
human autonomy rather than excuse automatic command is the spine here; the often-quoted "aids to human
viability, not excuses for automatic command" phrasing is a synthesis gloss, not a verbatim Beer line.)

**Epistemology — finitude is the design condition.** That every node is permanently, structurally
ignorant of most of what the others are doing is not a defect to engineer away but the condition to
design *for*:

> "Our knowledge can be only finite, while our ignorance must necessarily be infinite."
> — K. Popper, *Conjectures and Refutations* (1963), §XVII (p. 30) · *Verified.*

A node has effectively infinite ignorance about what other nodes are doing. Accepting that is what lets
the protocol design for real conditions instead of a pretended global view: theories are never verified,
only corroborated by surviving refutation, so a peer's history is a conjecture, agreement is
corroboration, and a claim is only ever *not-yet-overturned*. That is precisely the shape of provenance.

**Political science — self-governance without a sovereign arbiter is documented to work.** This is the
empirical answer to "is this utopian?" — no. Ostrom's Nobel-recognized study of long-lived commons
records communities governing shared resources for generations without privatization or a central
authority, with two principles Drystone mirrors directly: accessible conflict-resolution, and a
recognized right to self-organize. The capstone is subsidiarity:

> governance tasks are assigned "by default to the lowest jurisdiction, unless explicitly determined to
> be ineffective."
> — E. Ostrom (polycentric-governance generalization, 2013) · **[confirm before publish]** — this
> wording is from the 2013 generalization, not *Governing the Commons* (1990); pull the primary text
> before any external citation.

In a peer-to-peer system the "lowest jurisdiction" is the edge node — and every node is an edge node, so
governance stays local where consent is cheap and federates only the irreducible minimum.

**The convergence is the corroboration.** Witnesses who never met — describing the same structure from
ethics, economics, systems science, epistemology, and political science — give the design its strongest
support: no center can hold the truth; plurality is a survival condition for a system of equal nodes; and
the honest design surfaces disagreement and leaves judgment at the edges. That is the ground Part 2 is
built to stand on.

---

## What Part 1 establishes (and does not)

**Establishes:** the protocol's shape is *derived*, not preferred — from a structural fact about what a
node in a distributed system can know. The razor (provenance, not utility) and the four principles
(`P-Local-Truth`, `P-Knowable-Truth`, `P-Peer-Equality`, `P-Durable-Enablement`) are the obligations the
wire must meet, each ending in a consequence Part 2 realizes.

**Does not establish:** that local-first *guarantees* a humane system — it is necessary, not sufficient
(the edge can be wrong too; honest friction between real nodes is the cost, distinct from the
manufactured friction a center imposes). Nor does it settle the two open checks on the rights set
(`share`, `tenure` under re-key), or supply the still-pending verbatim grounding (Beer) and the
primary-source confirmation (Ostrom) flagged above.
