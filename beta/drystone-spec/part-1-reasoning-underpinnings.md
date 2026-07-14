# Drystone, Part 1: Reasoning Underpinnings

`Status: beta`

`Resolution: library, the complete reference and the coherence authority for the Drystone set. For shorter tellings see the coffee-shop overview, then the elevator pitch.`

`Defines: P-Local-Truth, P-Knowable-Truth, P-Peer-Equality, P-Durable-Enablement (the principles Part 2 realizes)`

`Map: the per-section index (§0, scope, dependencies, and orthogonality per section) lives at the end of this part, after the upstream reference links; maintained as sections change.`

This part is the "why." It is prose, deliberately, and it earns its place by ending every principle on a
consequence the mechanics in Part 2 must satisfy. Each principle states a commitment, gives its grounded
reasoning, and names the obligation it places on the wire. A principle that does not cash out into a
mechanic does not belong here.

---

## 1. Introduction

A system that puts a server in the middle of people's relationships makes that server the arbiter of
what is true: it holds the canonical copy, decides what is current, and can certify, revise, or revoke
any of it. That arrangement is convenient until the operator's interests and the participants' diverge,
at which point the participants discover the thing they were standing on was never theirs. Every layer
that depended on the center (who you are, what was said, who may act, who can be reached) fails at once,
and there is no local ground to fall back to.

Local-first restores that ground: the primary copy lives with the unit that authored it, complete and
usable alone, and the system reconciles between units rather than constituting them. **But local-first is
not merely an engineering preference about where bytes live; it is forced by what a distributed system
can and cannot know.** This part argues that claim and follows it down into the obligations the protocol
must meet. Therefore the protocol must be designed so that no participant ever has to take the center's
word for the state of the world, because there is no center whose word could be taken.

A **distributed system**, throughout this specification, is a set of **nodes** communicating remotely
about some shared state. No node can prove it holds complete and current knowledge of the others.
That single fact (not a value judgment, a structural limit) is where the principles below come from.

> **A note on what Drystone is, in one sentence, and on the word we do not use.** Drystone is a
> **center-free peer protocol**: peers connect directly (the transport is genuinely peer-to-peer), and
> the load-bearing property holds: **no node holds privileged or canonical authority over the shared state.**
> We deliberately avoid the word *serverless*: in current usage it names managed ephemeral compute
> (the cloud-functions sense), which is nearly the inverse of what we mean. We also use *peer-to-peer*
> only for the transport, never as the identity of the design, because topology does not capture the
> property that matters. The distinction, connectivity versus where adjudication lives, is the whole
> subject of Part 2 §3.1. Where we need the precise authority claim we say *center-free* or *no node
> holds privileged or canonical state*; where we mean the wiring we say *peer-to-peer*.

> **A note on the two words this document leans on most: persona and peer.** A **persona** is the human
> layer's manifestation in the system: the entity that rights and weight attach to because a person stands
> behind it. It is introduced here because the argument uses it well before it is made precise; the
> mechanical definition (a persona is a principal by virtue of one key pair, with its devices and clients
> descending by signed lineage, counted one-per-rooting-key-pair) is fixed in Part 2 §5.2. **Peer** is not
> the entity; **peer names the relation** personae stand in: symmetric standing where each is a locus the
> others must respect and none a center the others must obey. So "personae are peers" is the entity
> standing in the relation, and we never use "peer" as a noun for the entity itself. (The genus above
> persona is the **principal**, of which persona is the human kind; Part 2 §5.2.)

> **A note on three more words, and a distinction of case: group, Group, and scope.** This document uses
> **group** (lowercase) for the *social* sense: a body of people, manifested as personae, who communicate,
> coordinate, and govern together, the real-life grouping the whole design exists to serve. It uses
> **Group** (capital G) for that same collective *once it is a principal in the system*: a bounded
> entitlement-and-governance unit that can hold assets, be granted to, and act as a single unit, with the
> fixed boundaries that make its tradeoffs real. The capital-G Group is what Part 2 realizes over an MLS
> group (Part 2 §5.10); the lowercase group is the human body it manifests. The two are the same collective seen
> from two layers, and the case marks which layer a sentence stands on. A social group of people can raise
> a hand and decide "Bob hosts the relay"; the capital-G Group can make the *same* decision, but its
> mechanics are fixed by technical truth (a Group-governance act, or a change to the scope, below). A
> **scope**, finally, is *wider* than a Group: it is the extent of exposure and processing for a Group's
> messaging, the reach of routing and metadata beyond the entitled membership. A node that carries or
> triggers on a Group's traffic without being a Group member (a relay, a push-notify node) sits in the
> Group's scope but not in the Group. Governance and entitlement facts attach to the **Group** (a Group
> forks, holds history, sets thresholds); exposure and routing-reach facts attach to the **scope**. The
> full mechanical treatment is Part 2 §5 and Part 2 §6; the note here is only to fix the case-and-sense so the
> argument below reads cleanly.

### 1.1. Scope

This specification defines Drystone, the protocol, and nothing above it: no governance policy for any
particular community, no application semantics, no product. Part 1 defines the principles; Part 2
defines the mechanics that realize them. Where a principle has political or economic consequences beyond
the wire, those are out of scope for this specification. Keeping the spec tethered to mechanics is what
stops the principles from drifting into general theory.

Drystone is **vendor-neutral by construction.** Ecosystems, applications, and hosting operators may be
built on it, Croft is one such ecosystem, and is intended to be one of several rather than the only one,
but no named ecosystem, brand, or operator is part of the protocol's definition. Where this document
refers to a consuming ecosystem it does so as an example, never as a dependency.

### 1.2. Where Drystone sits (and the one move that is genuinely its own)

This is stated here, up front, so the rest of Part 1 is read against an honest backdrop rather than as a
claim of inventing-from-nothing. The full prior-art accounting is in Part 2 Appendix C; the short version:

The field split into two camps, and Drystone occupies the seam between them.

One camp solved the **data layer** by abandoning global coordination, CRDTs, local-first
(Kleppmann / Ink & Switch), and the Willow data model. The CALM theorem (Hellerstein & Alvaro) even
proved the exact boundary: a problem has a consistent, coordination-free distributed implementation if
and only if it is monotonic *(Verified against the primary paper; Theorem 1, verbatim)*.
This camp is mature and correct, and it deliberately stops at data: it has no notion of
expulsion-as-social-event, contested authority, or what a conflict *means*.

The other camp solved the **governance layer** but kept a coordinator, Matrix resolves conflicts by
folding authority and a wall-clock into the ordering; blockchains reach global consensus on a canonical
chain and treat forks as failures to avert. Both, under pressure, conclude you need an apex.

The nearest neighbor in *intent* is **Modular Politics** (Schneider, De Filippi, Frey, Tan, Zhang),
which explicitly calls for governance as an open, portable protocol standard, but roots all permissions
in a platform operator and brackets the cryptographic resolution and wire encodings as future work. It
drew the map and left the territory.

Drystone carries the **causal-and-cryptographic, no-coordination** premise of the first camp up into the
**governance** layer of the second, removes the wall-clock and authority from the ordering spine
entirely, and builds the resolution mechanics Modular Politics left unspecified, as a center-free peer
protocol, not a layer atop a platform.

**What is genuinely Drystone's own is not the mechanisms** (those are largely prior art, cited in Part 2
Appendix C) **but two things:** the **vertical synthesis**, a single derivation in which the epistemic
limit forces the data model, which forces the equality constraint, which forces the human-adjudicated
fork, and the **terminus**: *fork-not-verdict as a designed primitive, forced by an intrinsic-utility
argument* (§2.0, realized in Part 2 §7.6). The honest claim is not "first ever" but "unoccupied against
the closest published neighbors in every adjacent field, for a structural reason: the synthesis demands
fluency in several fields that rarely meet, plus a willingness to treat an impossibility result as
*generative* rather than as a problem to engineer around."

---

## 2. Design Principles

### 2.0. The razor: compute provenance, never utility

This is the master claim the four named principles descend from.

**A node in a distributed system can only ever establish *provenance*, what is consistent and
corroborated, never *truth*.** The distributed system exists to facilitate nodes acquiring the view
they need; it is not itself a knower. A node sees two things: **assertions** (what personae say, signed and
positioned) and **agreement patterns** (who corroborates whom). It never sees the world the assertions
are *about*. So "is this true?" is a question about something structurally outside any node's reach. The
only questions answerable inside the system are whether a node's view is internally consistent (**local
consistency**) and who else asserts the same (**alignment**), neither of which is a truth claim.

From this follows one design imperative, the razor:

> **Nodes establish, through the system, what is consistent and corroborated, *provenance*. Humans, at
> the edges, supply what is true and right, *utility*.**

Provenance is a closed, determinate question: *given a record, does its chain of signatures and
references verify?*, a question a node can answer alone, with no appeal to any authority. Utility is
open in principle: it lives in plural, revisable human values, and no node can compute it. A system that
tried to certify utility as if it were provenance would have to encode some fixed conclusion, often a
single one, as objective fact, and in doing so would marginalize every node that corroborated a
different local state, including the dissenter later judged, by humans, to have had the better view. (We
say "later judged" deliberately: a node cannot be shown *objectively* right against the others, only
subsequently corroborated or preferred. The razor does not promise that truth is discoverable; it
promises that disagreement is survivable.) **The honest mission is therefore not truth, but trustworthy
disagreement**, and that single line is the whole protocol's posture, lifted to a principle about
knowledge itself. Part 2's refusal to algorithmically adjudicate a social dispute (the *no-adjudication
rule*, Part 2 §7) and its content-blind safety posture (Part 2 §8) are this same razor at the mechanism layer.

**The razor extends to identity, and this is where most systems quietly cheat.** Cryptography answers
*provenance* with certainty, "is this the same key that made that earlier assertion", but it cannot
answer "is this key the person I mean." **That second question is a utility call, not a provenance one,
and it is always a human judgment.** All trust ultimately roots in social trust: even the root
certificate authorities that secure the rest of the web are trusted because a community of people vouches
for them and polices them, not because of any cryptographic fact at the bottom. A deliberate human act,
scanning a code to add a device, exchanging keys in person, does not *establish* trust; it **binds
existing trust to a key** so that provenance can carry it forward. Pretending cryptography *establishes*
trust rather than *recording and amplifying* a human decision is itself a failure mode, because it leads
people to trust the math exactly where the human judgment was weak. So the protocol's job is to give
**certain provenance** as a substrate and to leave **graded, contextual trust** to humans, never to
compute a trust verdict. Where the system carries trust signals between personae (one persona vouching for
another), those signals **MUST** be graded, **MUST** be indexed by purpose ("trusted for *what*"), and
**MUST NOT** be treated as automatically transitive or as an automatic grant; they are input to a human
or policy decision, never a verdict. (This is the lesson of PGP's web of trust, which failed by making
trust scalar, context-free, and automatically transitive; the fix is to keep a vouch an honestly-weighted
signal, never a computed conclusion. See Part 2 §5 `vouch` and `get_trust_signals`.)

#### 2.0.1. Time is an assertion, never a fact: the razor's sharpest edge

A wall-clock timestamp is the case where the assertion/provenance distinction bites hardest, and naming
it precisely matters because it drives a concrete mechanism (Part 2 §7.3.1, the timestamp-free ordering).

The weak reason to distrust timestamps is that **a persona can lie about its clock**, deception. True, but
not the deep reason. The deep reason is that **a timestamp is not a fact in the first place, even from a
perfectly honest node.** Time discernment is a shared construct, *locally represented*: each node holds
its own proxy for a clock, and a node can be **objectively wrong about its own time and have no way,
locally, to know it.** No node can ever prove *when* an event occurred on another node, because there is
no shared clock to appeal to, only each node's local instance of one. A timestamp therefore fails the
corroboration test at the root: it is structurally an **assertion**, on the utility side of the razor,
never **provenance**.

The consequence that Part 2 must honor: **timestamps cannot enter any computation that must converge or
resolve authority.** And, this is the part that survives even a hardened deployment, **this is a
social-engineering vector even when membership is gated.** Membership gating answers *who* is acting; it
says nothing about *whether their clock is real*. An authorized, honest member with a skewed clock can
manufacture a favorable causal position with no malice at all, and a malicious member can do it
deliberately while passing every membership check. So excluding the wall-clock from the ordering spine is
**not merely a defense against attackers**; it is a consequence of the razor: time is not corroborable,
therefore it is not provenance, therefore it cannot order what must converge. What *can* be corroborated,
causal precedence (did this fact reference that one), per-device logical clocks, and liveness over a
window (Part 2 §7.4), is what the protocol orders by instead.

There is a "right tool for the job" lesson folded into this, and it is worth stating because it is easy to
miss. A timestamp is a perfectly serviceable *convenience* for the **data plane**, ordering chat messages
for display, where being slightly wrong is cosmetic and nothing downstream depends on it being
adversarially sound. The error is carrying that same convenience into the **control plane**, the path
that decides *who won a governance conflict*, where a forgeable ordering input becomes a capture vector. A
value's acceptability depends on the job: tolerable when it only sequences what is *shown*, disqualifying
when it sequences what is *decided*. This is why a system can reasonably timestamp messages and yet must
never timestamp the resolution of authority (the concrete contrast with Matrix's `origin_server_ts`
tiebreak is in Part 2 §7.3.1).

One honesty note about the scope of that "must," because it is **architecture-relative, not a universal law
of distributed systems.** The disqualification follows from two properties of Drystone that are two sides
of one coin. *First:* every node is responsible for its own canonical view, with **no authority tier above
the principal** to appeal to, so a corrupted governance resolution **cannot be overridden in place**, because
there is no higher authority to do the overriding. *Second, the same fact from the other side:* Drystone's
**only remedy for a bad governance outcome is the fork** (§2.5, Part 2 §7.6), exit, not override. A system
that *does* have an authority tier (Matrix's homeservers and room admins) can correct a bad resolution
cheaply, in place, by issuing a new authority event ("start a new epoch," de-op, ban), and can therefore
*rationally tolerate* a routinely-corruptible ordering input, absorbing the occasional gamed result with a
cheap administrative correction. Drystone cannot: with the fork as its only correction, letting routine
ordering be forgeable would mean a heavyweight schism every time the input is gamed. So the constraint is
not "every system must exclude forgeable ordering inputs"; it is "**a system whose nodes each hold their
own canonical view and whose only remedy is exit rather than in-place override must**," which is our case
and the case of any design sharing those two properties, not a claim about systems we are not building.
*(Realized in Part 2 §4.5.1, Part 2 §7.3.1, Part 2 §7.4.)*

The four principles below are what the razor requires of the wire.

### 2.1. P-Local-Truth: the only canonical state is local

**Commitment.** A device can honestly know exactly one thing: its own local state. Its local store is
**canonical for that device**, the only state it can *prove* rather than infer. Any belief about
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
state, because a node the rest must defer to for canonical state is a center, exactly what P-Local-Truth
denies. A node that has seen fewer facts **MUST** compute a *stale-but-honest* view, never a false one.
The comparative and asserted layers are how nodes converge; the local layer is the only thing any of
them stands on. *(Part 2 §4, Part 2 §5.1, Part 2 §7.)*

### 2.2. P-Knowable-Truth: verify for yourself; the record is auditable, never silently mutable

**Commitment.** What a node takes on trust must be minimized and what it can verify for itself must be
maximized. A node verifies an entry's integrity and authorship without consulting any central authority,
and the history a node relies on is **append-only and auditable**, facts accrete and can be audited;
they are never silently revised or reset to an earlier value.

**Reasoning.** Certainty *for a node in a distributed system* is unreachable: a node cannot know what
else is happening beyond its own view and can only work from what it has. The honest response is not to
manufacture certainty but to make the record self-describing, so that what a node cannot personally
witness it can at least cryptographically check, and so that no decision can be quietly unmade. The
provenance the razor leaves us, a survivable record of what was asserted and corroborated, is exactly
what a system that cannot certify truth *can* honestly compute.

**Consequence.** Therefore state **MUST** be cryptographically self-describing (integrity and authorship
checkable from the bytes), and authority **MUST** be a monotonic fold over an append-only fact set rather
than a recomputed mutable value, so a lagging node under-authorizes (acts on less) but never
mis-authorizes (acts on a reverted or forged state). Every superseded or stale decision **MUST** remain
in the record, attributable. *(Part 2 §4.3, Part 2 §7.3, Part 2 §8.)*

> **This consequence has a deployed cautionary tale.** The append-only-monotonic-fold requirement is not
> abstract caution. A peer protocol in the same design space (Matrix) that instead recomputed a mutable
> state map shipped, in 2025, a state-reset vulnerability class (CVE-2025-49090) in which room state
> could revert to an earlier value with no event validly producing that reversal. The monotonic-fold
> rule is the structural defense against precisely that class: a lagging node under-authorizes, it never
> reverts. See Part 2 §7.3 and Appendix C for the grounded comparison. **[verified: the CVE-2025-49090
> root cause (a state reset to an earlier or incorrect value absent a validly-producing event) and the
> MSC4297 fix are confirmed against the Matrix State Resolution v2.1 implementer's guide, the Project Hydra
> disclosure, and the CVE record; the primary-source check lives in Part 2 §7.5.2 and Appendix C.]**

### 2.3. P-Peer-Equality: equal in rights, therefore equal in weight; unequal in resources and revocable Group Roles

**Commitment.** A **persona is the human layer's manifestation** in the system, not a node or a device; rights
and weight attach to it *because* a person stands behind it. Peer-equality is therefore **equality of personae as
manifested**, and the protocol guarantees, by mechanism, that one recognized persona carries equal rights and
one flat unit of weight. Whether a persona corresponds to one distinct human is a social-utility judgment the
Group makes at its own standard, never one the system can attest, and never one it **could** attest (the
binding has no technical representation; the razor of §2.0 applied to identity). There is **one kind of persona**,
and "equal" is made precise by asking *in what ways may one persona differ from another?*, and answering with
**four properties, two necessarily equal and two legitimately unequal**. (Personae stand in **peer
relation**; "peer" names that relation, not the entity.)

The two **equalities**, equal for every persona always:

- **Right**: what a **principal** *inherently holds*: the floor of voice, tenure, and exit/fork. Held
  **identically by every persona, and unremovable.** A right is precisely the thing that *cannot* be delegated
  or revoked, the tell is that exit/fork survives even when every Group Role is stripped and even when a quorum
  captures the Group. A right is standing in the *system*, not in any one Group: it is what the fork carries
  into both descendants, which is exactly why a Group's governance can never reach it.

- **Weight**: how much a **persona** *counts* in governance. **Flat: one per distinct persona (by lineage)**, and
  equal **not by a separate decree but as a consequence of the equal right**: if standing-to-participate
  is equal, standing-to-be-counted is equal. Weight is the governance image of the rights floor, the same
  commitment expressed in a different context, not an independent conclusion the design arrives at on its own.

The two **inequalities**, legitimately different between personae:

- **Resource**: what a **node** *has* (storage, uptime, reachability, a radio). Intrinsic to the
  device, descriptive, **expected to be unequal**, and not delegable. A node with more resources can *do*
  more. This is a fact about every node in the system, not only a persona's devices: meers and relays have
  resources too. It appears here because, *across personae*, it is one of the two legitimate inequalities.

- **Group Role**: what governance authority a **principal** has been *granted within a Group* (admin, moderation,
  gating, the authority to act for a Group). **Granted by consent, scoped, attenuating, and always revocable.** This is
  the one *operational* inequality the design permits, and it rides **entirely above** the two equalities:
  granting or revoking a Group Role never touches a persona's rights floor or its unit of weight. (Where this
  document contrasts the *category* of granted authority with the *category* of inherent standing, it says
  "role" in the lower case, the genus; a concrete grant inside a Group is a "Group Role.")

The one-sentence statement of the model: **personae are equal in rights, and therefore equal in weight,
and unequal in resources and revocable Group Roles.** The moment more resources, more devices, or any granted role
buys more *rights* or more *weight*, the design has leaked. (The Meadowcap **capability**, a read/write
data-access grant, is *not* a fifth property here; it is the mechanism a role operates through, one layer
down in the data plane. See Part 2 §5.0, Part 2 §5.5.)

**Reasoning.** What separates an inherent **right** from a granted **role** is an intrinsic property, not a
list, and the distinction lives at two layers that must stay aligned. At the **social** layer, in any human
group whether or not it runs on a protocol, a *right* is standing that must survive for any dispute about
it to remain contestable, remove it and the holder loses the very means to object, whereas a *role* is
delegated authority whose removal leaves the holder's standing intact. That is the epistemological
distinction, and it holds independently of any technology. Drystone's **Group Role** is that social *role*
made technical, with fidelity: it is a delegated governance authority (a moderation power, the authority to
act for a Group) whose removal, by the same test, leaves standing untouched. Lose admin on a Group and you
can still contest the loss through your tenure, voice, and exit; that is the tell it was a Group Role, not a
right, and it is the technical mirror of the social test passing. The two layers agree by construction,
which is the point: the mechanism is trustworthy exactly because the wire-level distinction between a right
and a Group Role reproduces the social distinction between standing and delegated authority, rather than
inventing a different one. And **weight** is not a third thing to be clamped by fiat: it is the
governance image of the equal right, the flat count of one per distinct persona (by lineage) that no Group Role grant
and no device count may inflate, equal *because* rights are equal. Equality of *weight*, over a floor of
equal *rights*, is what lets equal nodes hold and reconcile divergent views, the whole job of nodes that
stand in for a human's experience. So the rights floor and the weight that follows from it are a
**consistency-and-equality requirement, not a moral overlay**: together they make "no center can certify
truth" hold at the level of who-may-act and who-counts, not just what-is-true.

The load-bearing anti-capture claim, stated honestly as what the protocol *guarantees* versus what the
Group *judges* (the provenance/utility split of §2.0, applied to identity):

- *Protocol guarantee (provenance):* **weight is flat per recognized persona and conserved under
  delegation**, allocated one-per-persona at the source, only ever moved, never minted. Adding devices adds
  resources, never weight. This holds by mechanism.

- *Group judgment (personhood, contextual):* whether a recognized persona is a distinct person is a
  **utility judgment the Group makes at its own confidence**, on the same trust-to-do gradient as every
  other delegation, high in a QR-scan family Group, deliberately decoupled-but-still-one-per-person via a
  verifiable-credential service in an anonymous Group, loose in an open broadcast Group. Drystone does
  **not** attempt to guarantee one-lineage-one-human, **by design**: that binding is the kind of truth the
  system structurally cannot certify (§2.0), and enforcing it would prune the legitimate **multiple
 presentation** (the same person as parent, pseudonymous activist, anonymous participant) that is part
  of the social substrate (the variety argument, applied to identity). So the claim is not "you cannot
  forge personhood"; it is *given the Group's recognition of its personae, weight is flat and uninflatable by
  resources.* Sybil resistance is contextual, supplied by the Group's chosen confidence mechanism, not
  global and not the protocol's. *(Part 2 §5.6 carries the full treatment and the Spritely / ActivityPub
  grounding.)*

This yields **one negative boundary** the whole protocol refuses to cross, *no persona may remove another's
standing to hold variety*, distinct from the several positive **rights** that define what a persona is.
Rights-removal is the only self-amplifying move toward collapse: it lowers the variety available to resist
the next removal, the way a monoculture lowers a system's capacity to absorb the next shock. The
discriminating test for any proposed action is whether, generalized, it would remove the conditions of its
own contestation; if so it is illegitimate by nature, and the tell is self-cancellation.

**Consequence.** Therefore equality **MUST** be enforced by mechanism, not convention, because equality that
rests on convention is not a right: it can be withdrawn by whoever holds power the moment it is inconvenient.
**Rights have no
presets; Group Roles, capabilities, and Group Role Sets do, and weight is flat.** Every named configuration of a persona
**MUST** be expressible as `floor + [explicit Group Role Set] + [implied capabilities] + [expected resources]`;
any configuration meaning "this persona is entitled to *fewer rights*" is rejected as a smuggled rights
distinction, and any meaning "this persona *counts for more*" is rejected as smuggled weight. Delegation
**MUST** attenuate (a subset of held authority, never a superset), because a delegation that could exceed what
the delegator holds would mint authority from nothing, an increase no persona consented to. Governance
thresholds **MUST** count distinct personae by lineage, never clients, because counting clients would let one
persona inflate its weight simply by adding devices, collapsing the flat-weight equality the principle secures. (A **Group Role Set** is a named bundle of Group Roles that
travel together, so a Group can grant or revoke them as one unit rather than role-by-role, and can constrain
them at the meta-governance level, "a holder of this Set may not also hold that Group Role", which is how
separation of powers is expressed inside a Group. The concept is carried but still settling; its mechanism
is developed in Part 2 §5.) *(Part 2 §5, Part 2 §7.)*

> **A note on vocabulary against the prior art.** Drystone uses three distinct nouns at distinct planes,
> none colliding with the systems it builds on: **resource** (a device facility, one of the two
> inequalities), **Group Role** (an in-Group **governance authority**, the other inequality, the layer MLS
> deliberately leaves to the application), and **capability** (Meadowcap's term, kept verbatim: an
> unforgeable read/write **data-access** grant). The capability sits **beneath** Group Roles, not beside
> resources: a Group Role may carry the authority to issue capabilities, and the capabilities themselves are
> data-plane tokens, one level below the peer-equality question. So Meadowcap's "capability" is Drystone's
> capability, unchanged, the device facility is a "resource," and the in-Group governance authority is a "Group Role"
> (lowercase "role" is the genus, the category of granted authority; a "Group Role" is a concrete grant inside a Group).
> Detailed in Part 2 §5.0 / Part 2 §5.4 / Part 2 §5.5.

> **An equal-and-opposite design exists, and it is worth naming as the steelman.** Under adversarial
> security review, Matrix concluded the opposite of P-Peer-Equality: that sound decentralized
> conflict-resolution requires an *uncapped* root (room creators with "infinite" power), reasoning that
> an attacker who can backdate events can already exercise de facto apex control, so the apex must be
> made explicit to be bounded **[verified against MSC4289 and the Matrix Project Hydra disclosure (Aug
> 2025): the room creator holds an "infinitely high" power level, with Matrix's stated reason that
> backdating already gives the creator's server de facto control; primary-source check in Part 2 §7.3 and
> Appendix C]**. Drystone's wager is that this
> conclusion is *forced by their inputs, not by the problem*: their ordering consumes a wall-clock, so
> backdating manufactures authority, so they pin authority to an apex. Drystone removes the wall-clock
> from the ordering spine (§2.0.1), so the specific attack that forced their apex has no purchase here.
> Whether that fully discharges their attack class is an explicit open item, not a settled claim, see
> Part 2 §7.3 and Appendix B. The deeper difference is one of *design philosophy*: Matrix **prevents**
> capture with an apex; Drystone **permits** capture by a legitimate quorum and makes **exit (the fork)
> the remedy** (§2.4, Part 2 §5.7 "Capture ≠ brick"). The two are different answers to the same threat,
> and Drystone's is the one consistent with peer-equality.

**A principal is recursively a Group, and two edge types keep the recursion honest.** A principal is a *locus of
adjudication* (it holds authority others must respect), not merely a node that senses and relays; and the
same primitive nests: a **principal can be a Group**, a **persona's own devices are a Group** (in MLS terms a
Group of clients, the key-and-author bindings that live across a persona's devices), a **community is a group
of people** that can itself act as a Group-principal, a federation a grouping of communities. (Note the case:
where the clause names a *social body* it is lowercase group; where it names that body *as a bounded
principal in the system* it is capital-G Group.) The recursion is held together by
**two distinct kinds of relationship that must not be conflated**: **composition**, members co-deriving
shared authoritative state (a device Group, a Group's membership; this is the MLS-lineage / shared-key
relationship), and **valuation**, one Group directionally *weighting* another's assertions without
any shared key (trust between cryptographically-separate principals). Composition merges state; valuation
weights signals. Blur them and trust leaks into key access. A consequence the rest of the design leans on:
**adversarial posture is a per-edge property, not a global stance**, your own device Group is, *by
default*, a high-trust composition edge that needs little Byzantine defense, while a stranger valuation
edge needs more; forcing strong-adversarial rigor where it doesn't fit is as wrong as omitting it where it
does. But *by default* is load-bearing: **even the device-Group edge is a dial, not a fixed truth.** A
person whose threat model includes a single device being seized or coerced, an activist, a journalist,
someone in an unsafe household, may rationally want Byzantine-style suspicion *within their own device
Group*, treating a captured device as a hostile signer. The family-tablet case and the seized-device case
are two settings of one dial; a design that hardcodes "device Group = trusted" prunes the second case out
of existence, which is the variety failure in miniature, and it bites hardest exactly when the stakes are
highest. *(Part 2 §3.1, Part 2 §5.)*

> **The dial-discipline principle, 80/20 defaults, and the 20% must stay representable.** Once every
> trust relationship is recognized as a dial, a real risk appears: *if everything is a dial, nothing is
> usable.* Preserving variety is not exposing every knob to everyone; that collapses under its own
> complexity. The discipline is: **default the common case hard** (most groups never touch the dial), and
> **keep the uncommon case representable without ceremony** (the person who needs the unusual setting can
> reach it), and **never let a default calcify into a structural assumption that forecloses the
> alternative.** Variety is preserved by the 20% being *expressible*, not by it being *foregrounded*. This
> is not a footnote: the whole point of refusing a hardcoded center is undone if the defaults quietly
> become a center by foreclosing the settings the minority depends on.

**Why this matters concretely, who owns a shared artifact when a Group forks.** A Group, being a
principal, can *own* things, and that ownership is what makes the fork humane rather than destructive.
Picture three personae collaborating on a document by automatic merge; they hit a genuine disagreement and
the Group forks (§2.5). Who keeps the document? The honest answer is **both layers own it, and so both
forks keep the whole thing.** The artifact lives in the Group's *communal* space, the Group-principal
owns it as a collective, and each persona owns its own contributions as an individual, so when the Group
splits, neither half is orphaned (there was no center holding the artifact to sever) and neither half is
left with fragments (the shared object was communal all along). Each fork walks away with a complete copy
and full standing to continue, exactly as an open-source fork carries the entire repository rather than a
slice. This is *fork-not-verdict* (§2.5) seen at the data layer: the system does not rule on who deserves
the document; it lets both continue. The mechanism, a Meadowcap *communal* namespace, the act-for-the-Group
authority as a revocable Group Role, and the recursion bottoming out at flat-weight personae so composition cannot
launder governance weight, is specified in Part 2 §5.10. The reason it is peer-equal and not apex is the
same reason this whole principle is: a communal namespace gives every member equal authority and no one the
whole, which is `P-Peer-Equality` expressed in who-owns-the-data.

> **Open seams left before the model fully hardens** (carried, not resolved): whether the survivor /
> re-key path can strand a persona's `tenure`; and the precise **communal-namespace key construction** for a
> Group-as-principal (Part 2 §5.10 gives it a concrete shape, a communal namespace rather than a derived
> central credential, but the key establishment and rotation under membership change are
> designed-not-frozen, and cross-Group grants are `[gates-release]`). These gate freezing the model into
> normative text; see Part 2 §5.2, Part 2 §5.6, Part 2 §5.10, and Appendix B.

### 2.4. P-Durable-Enablement: participation, and exit, must be real on a bare node

**Commitment.** Participation **MUST** be possible on a bare node, ordinary device, no purchased
infrastructure required, and any default delegation a persona or Group adopts **MUST** be revocable and
restructurable down to the rights floor at any time, with **no loss of rights** and only **graceful
degradation of capacity**. The enabling set is mostly restraints, not features: secure standing; a real,
cheap, dignified exit; an honest, non-equivocating record; resolution that defers judgment to humans; and
a refusal to optimize toward a single legible perspective. **The negative space, what the protocol
declines to do, is an intentional and critical part of the design.**

**Reasoning.** A delegated Group Role is only meaningfully different from a captive structural dependency
if the delegation can be withdrawn and restructured without loss of rights. A good default helper and a
server you cannot leave can look identical right up until trust fails, and that is exactly the moment
the difference must hold: the Drystone Group restructures or exits and loses only capacity; the captive
arrangement loses everything and starts over. The guarantee is measured not by how often it is exercised but by
being unconditionally available to the minority who do; a right you cannot afford to exercise elsewhere
is not a right you hold. Giving up the authority to decide outcomes is also the protocol's falsifiability
discipline: you can be observably corrected when the conditions you set fail to keep variety alive.

**Consequence.** Therefore the no-helper path **MUST** stay exercised and real (a Group **MUST NOT**
structurally depend on any single persona's presence to act), delegation **MUST** be materially reversible
(encrypted state whose keys the Group's members hold, not a box), and the fork / re-formation
exit **MUST** remain available as the final backstop, preserving history and provenance to the point of
departure. *(Part 2 §5.4, Part 2 §5.8, Part 2 §6, Part 2 §7.)*

### 2.5. The forced terminus: why fork, not verdict

This subsection is new emphasis, not a new principle: it makes explicit the single conclusion that the
razor and the four principles jointly force, because it is the move that most distinguishes Drystone and
it should not be left implicit.

Most conflicts in any shared state are resolvable by codified logic. Anything monotonic, anything where
causal-and-cryptographic ordering yields a determinate answer, resolves by the fold with no human and no
clock (Part 2 §7.3). **But there is an irreducible residue, and it is provably non-empty.** Genuine
concurrent contradictions over standing, A expels B while B expels A at equal standing; a
removed-then-included merge, cannot be totally ordered without folding back in one of exactly two
forgeable inputs: a **wall-clock** (not corroborable, §2.0.1) or an **authority ranking** (the very thing
being contested). This is the CALM boundary applied to governance: a total social order over concurrent
non-monotonic operations is itself a non-monotonic problem, so it has no coordination-free determinate
resolution. You either coordinate (an apex, a consensus round, a delivery service) or you accept the
order is partial and something must resolve the residue.

**And the residue is not merely uncomputable-by-this-machine; it is not a computation at all.** When A
and B each expel the other at equal standing, there is no fact about the world that makes one correct.
Both assertions verify; the causal structure is symmetric; "who should remain" is not a question with a
truth-value waiting to be discovered. It is a question about whose continued participation the *people*
want, a value, not a fact. No additional sensing closes the gap because there is nothing there to sense.
The residue is exactly the set where **provenance is fully determined and utility is still open**, the
razor's seam, made operational. This is why "intrinsically personal" is doing real work and not
emphasis: the resolution input must come from the people whose relationships are at stake because *they
are the only locus where the value being adjudicated exists.* They do not have privileged access to the
answer; they **constitute** it. (The residue has more than one shape: the mutual-expulsion case here is
*too many* valid claims, and a required role left vacant with no valid successor is *too few*; Part 2
Part 2 §7.6.1 enumerates both, since a mechanism that watches only for contradiction would miss the second. Both
are the same seam, provenance-settled and utility-open.)

**Therefore the terminus is a fork, not a verdict.** A verdict would presuppose the question had an answer
the loser should accept; the fork presupposes it did not, and lets divergence persist as two communities
rather than forcing one false resolution. When even the humans cannot agree, because they irreducibly
want different things, both legitimately, the protocol's last service is to make the split **clean**
(history and provenance preserved to the point of departure, nothing legitimized or erased
retroactively), not to manufacture a consensus that was never available. This is Mill's dissenter, made
mechanical: when people irreducibly disagree about a value, the humane move is not to compute a winner
but to stop pretending there is one. *(Realized in Part 2 §7.6; the machine's job ends at surfacing the
contradiction with full provenance, Part 2 §8's label-not-enforce, and the humans supply utility.)* The
move has formal precedent: Internet governance names exactly this step, a person deciding inside the
machinery, as the Designated Expert (§3); the terminus differs only in distributing that role to every
local authority rather than delegating it to one.

The fork right has a **floor and a limit**, and both matter.

**The minimum fork is one.** Forking has zero dependency on anyone else's cooperation, which is exactly
what makes it the un-blockable floor. A single persona, alone, disagreeing with everyone, can fork, and
their lineage is as legitimate as any other; it is simply small. No configuration and no coalition can
prevent it, because preventing it would require reaching into a single persona's own possession, which
local-first (§2.1) makes impossible.

**The mechanical right is distinct from the social outcome, and the protocol guarantees only the first.**
The system guarantees you *can* fork, down to a group of one; it guarantees nothing about whether anyone
joins you. Whether others follow is the aggregate of many individual decisions, each persona choosing which
lineage to corroborate going forward. The protocol furnishes the ability to leave with everything that is
yours and stays deliberately silent on whether you leave alone or with company, because who-follows-whom is
the personae's call, not the substrate's. This is a social-utility statement, not a system guarantee, and
keeping the two apart is the same discipline the razor imposes everywhere (§2.0).

**A ban is this same primitive seen from the other side.** A ban is a forced fork, equal in outcome to a
voluntary one and differing only in its artifacts (the mechanism is Part 2 §7.6, where a ban and a
voluntary fork are one lineage-divergence primitive, distinct only in what each records). The consequence
worth stating at the principle level is that the group's harshest power over a member is **bounded by this
floor**: a ban cannot erase a member, it can only end the group's corroboration of them going forward, and
the banned member continues whole in their own lineage holding everything they had. The most a concentrated
authority can do to a member is force a fork, and a forced fork leaves the member intact. This is what ties
§2.5 back to §2.7: the escape hatch's last resort is un-blockable precisely because the fork floor is one.
`Synthesis.`

A note on what this is *not*. It is **not** "send every conflict to humans"; that would drown the
escalation channel and destroy the one thing that has to stay trustworthy (Part 2 §7.4, Part 2 §7.6). Codified
logic resolves everything it determinately can; only the provably-non-empty non-monotonic residue is
handed to people. And whether a given concurrent contradiction is a genuine social dispute versus a
benign sync artifact is *itself* partly a utility judgment, vulnerable to alarm-fatigue, and therefore a
per-Group governed tolerance over verifiable provenance signals, not a hardcoded constant (Part 2 §7.4,
Part 2 §7.6).

### 2.6. The voice right requires field-integrity: why the substrate's ownership form is in scope

This subsection adds no new principle and no new wire obligation. It names one dependency that the four
principles jointly imply but none states outright, because it is the joint between the protocol and the
larger question of who may own the substrate the protocol runs on. It is kept short on purpose: the full
grounding, including the empirical case, lives in a companion argument (*Peer Standing, the Securitized
Corporation, and the Cooperative Form*, Part 2 §7) and is not re-argued here. What belongs in the spec is only
the structural claim and the existing mechanisms that already realize it.

**The claim.** `P-Peer-Equality` (§2.3) makes **voice** a right: the standing to assert into the record
and be corroborated or refuted *on the assertion's own terms*. (The congruence is not incidental: *persona*
is, at its root, *per + sonare*, the thing a human's voice **sounds through**; voice is the right that
manifestation exists to carry. The name and the right are the same claim in two registers.) By §2.3's own
test, a right is standing
whose removal leaves the holder unable to contest the removal. Voice meets that test only if one further
condition holds: that the field a persona asserts into, and perceives in order to respond, is not itself
authored by a party with an interest the persona does not share and cannot see. Call this **field-integrity**.
The two are not separate requirements. **A persona cannot hold the voice right on a substrate where a center
authors the field, because the right and field-integrity are the same precondition seen from two sides.**
If a center decides which of your assertions propagate, to whom, and with what salience, against an
objective that is not yours, then your voice has been quietly demoted from a right you hold to a capacity
the center grants and shapes, which is exactly the right-to-role leak §2.3 names as the tell of a broken
design, instantiated in the communication layer rather than the governance layer.

**What field-integrity does and does not require.** The field is *always* ordered: there is no unordered
feed, and removing a center does not remove the need to decide what a persona sees first. So field-integrity
is **not** "an unshaped field." It is three properties of the *shaping*:

- **Peer-governed.** The ordering objective is determined by personae judgement, not installed by an external party with
  a non-peer interest. A default ordering is legitimate and expected (the dial-discipline of §2.3: default
  the common case hard, keep the uncommon case representable). What is illegitimate is the objective being
  structurally someone else's and unremovable.

- **Legible.** A view is knowable *as a view*. The system must not present a curated slice as if it were
  the unmediated field. This is the relational-layer instance of `P-Knowable-Truth` (§2.2, no silent
  mutation) and of the freshness rule (Part 2 §7.4: silence must not be rendered as currency): the
  protocol already refuses to present a partial or stale state as the whole and current one.

- **Exitable.** A persona who rejects how the field is ordered can change it or leave with standing intact.
  This is `P-Durable-Enablement`'s fork (§2.4) seen from the perception layer. Capture you can see and
  leave is not domination in the §2.3 sense; capture you cannot see or cannot escape is.

**Why "especially silent" is load-bearing.** The failure mode is not that a field is shaped; it is that
the fact and objective of the shaping are concealed, so a persona cannot form the intent to contest it. This
is not incidental to an attention-optimizing center; it is required by it. A curator that announced "this
ordering is selected to maximize your time here against your stated preferences" would defeat its own
objective. The opacity is structural, which is why the legibility property is not a nicety but the
specific defense: the center can only farm the field if the persona cannot see the plough.

**The full-stack dependency, in one line.** The protocol guarantees a peer-governed, legible, exitable
field by removing the center that would otherwise author it; an aligned ownership form (the companion's
argument for the cooperative) guarantees that nothing external is installed to re-author it, because the
member is who the entity serves and the member's attention is therefore not the salable good. **You need
both:** the protocol can be deployed under an ownership form that reintroduces a curating center, and the
ownership form can be adopted over a substrate that still has one. Neither layer alone closes the gap, and
that mutual dependence is why this specification treats the substrate's ownership form as in scope for the
*motivation* even though it is out of scope for the *mechanics* (§1.1).

> **A note on the ownership claim's status.** The companion argues that the aligned form is the
> cooperative, not because the cooperative is automatically aligned (it is only as aligned as its charter
> and governance make it), but because it is the form in which alignment can be made a binding, enforceable
> commitment held by the people who use the thing, rather than a discretion an external capital
> constituency may revoke. That is a claim argued in the companion, not a protocol guarantee, and it is
> named here only to mark where the dependency points.

> **[tension] Field-integrity is not a pure good, and the center-free field does not escape all shaping.**
> Some shaping is endemic to any delivery system that must order what a persona sees: even an altruistic
> re-ranker is pulled toward engagement absent better signal on true utility (the revealed-preference
> trap; companion Part 2 §7). Removing the external-objective curator does **not** yield an unshaped field; it
> removes the *structurally adverse* curator and returns the ordering to peer governance. So the claim is
> the narrower, defensible one, peer-governed-legible-exitable shaping, not no shaping, and the protocol's
> job is to make the ordering contestable and the curator absent, not to pretend ordering away. The
> reflexive harm by which concealed shaping degrades the collective correction that would otherwise
> contest it is grounded in the companion (Part 2 §7) and is not re-argued here.

*(Realized by: §2.2 no-silent-mutation; §2.4 fork-as-exit; Part 2 §7.4 silence-is-not-currency, Part 2 §7.6 fork.
This subsection adds no wire obligation; it names why the existing ones are the realization of the voice
right's field-integrity precondition.)*

---

### 2.7. Consolidation and the center: capability freely, authority revocably

The equality of §2.3 fixes what may not differ between personae. This principle fixes what a **center** is,
and shows that neither kind of consolidation the design permits creates one. It is the reasoning Part 2
leans on wherever it admits an operator, a helper, or a concentrated governance posture (Part 2 §5.5, Part 2 §5.9,
Part 2 §7.6.9, Part 2 §7.9), and it turns on one distinction: **capability is not authority.**

**The distinction.** Two quantities usually run together must be kept apart.

- **Capability** is what a principal can do with resources: how much history it can hold, how much it can
  index or serve, how reachable it is. It is unequal by nature, and the design places **no ceiling** on it.

- **Authority** is standing: the power to decide who is a member, to change the rules, to foreclose on a
  persona, to override a decision. It is **flat**, held by personae collectively, and moved **only** by the
  k-of-n governance fold (Part 2 §5.7, Part 2 §7.3). No accumulation of capability converts into any of it.

This gives the first definition of a center: **a center is a principal that can exert authority beyond a
persona, not one that has capability beyond a persona.** A principal with vast resources and no standing is
not a center; one with modest resources and the power to foreclose would be. This is why the design is
rooted in a control claim rather than a privacy property: not "your data is hidden," which people have
learned to discount, but "no persona has standing over you, and no principal earns standing by capability,"
which holds no matter how capable any node becomes.

**Helpers are the delegation case: capability without standing.** A helper is a node a persona or group
admits to do work, a store-and-forward node so members receive while offline, a high-memory archive so a
phone-only member can converge deep history, a clear-text search or index helper. A helper **MAY** hold
clear text, and what matters is how it comes to hold it and what standing that confers, because the same
clear text read by infrastructure-by-construction and read by an invited-and-revocable helper are opposite
in standing though identical in content.

- A helper holds clear text only because a persona or a k-of-n decision granted it, by admitting it to the
  scope exactly as a member is admitted, and it is revocable the same way.

- A helper holds **no** authority by virtue of that access: it cannot foreclose, remove a member, or
  override governance, and can do nothing a persona could not authorize and revoke. It is additional, not
  elevated.

This is the categorical difference from the surrounding ecosystem's access-control model, where a personal
data server or an authorized view holds readable data **structurally**, as a property of the architecture
that cannot be removed without removing the system, occupying a privileged tier by construction. In Drystone
a helper holds readable data **by delegation**, a grant personae extend and can always revoke, occupying no
tier. The honest tradeoff, stated plainly: delegating clear text to a helper expands the confidentiality
surface to that helper, since its compromise leaks what it was shown, but it never transfers authority and
it is revocable, so a content-blind durability store and a clear-text search helper are two points on one
spectrum, role delegated without authority, differing only in how much the personae chose to reveal.

**Capability may consolidate freely, and the move is one-directional.** Because capability is not authority,
Drystone may be arranged anywhere from fully peer-distributed, every persona on its own device holding its
own history, to fully consolidated, a single operator hosting the delivery helpers, archives, and search
apparatus so that a user cannot tell it apart from an ordinary hosted product. It is center-free at **every**
point, because the operator, however much it runs, holds no authority over any persona: it is a collection
of delegated-capability helpers, and any persona or group can re-home elsewhere without permission. The
inverse move is **impossible**. An authority-centralized design cannot be arranged into peer-symmetry,
because its authority is structural: the owner or space authority and the gatekeeping infrastructure tier
are load-bearing, and removing them removes the system. Their center is not a dial that can be turned down;
it is in the architecture. So Drystone spans the full range without acquiring a center while the
authority-centralized designs cannot make the reverse journey and keep their function: the convenience of
consolidation is available to Drystone without the cost of a center. `Synthesis.`

**Authority may consolidate freely too, because it is revocable and exitable.** The stronger claim concerns
authority itself. Governance concentration is a permitted spectrum, and every point on it is legitimate:
flat consensus, delegated roles, concentrated authority, even a group where only the creator's signature
counts as the quorum. Drystone does not forbid a near-authoritarian internal structure, and a group may
even be created so that useful disagreement is difficult. What makes every point center-free is **not** that
the points are egalitarian, because they are not; it is that the concentrated authority is held up only by
the continued participation of the members, and the escape is invariant. A "creator is king" group is not a
center, because the creator's authority is not structural: it is granted by everyone staying, and it
evaporates the moment they leave or revoke. The creator can make the room hard to argue in; they cannot lock
the doors.

This yields the sharpest definition of a center. **A center is not a node with a great deal of authority. A
center is a node whose authority is irrevocable and inescapable.** Concentrated-but-revocable authority is
not a center; modest-but-locked-in authority would be. What distinguishes a Drystone group with a
near-dictator from an authoritarian platform is not the concentration of power, which may look identical day
to day, but that the Drystone version sits on a substrate where the power is always revocable by threshold
and always escapable by fork.

**The escape hatch is a strict hierarchy of last resorts, and it MUST be readable as such,** because a hatch
that a captured minority cannot recognize or reach in order is a hatch only in name, and the whole
permissiveness above rests on its being real:

- First, work within the rules: persuade, re-vote, use the governance the group has.

- Else, if the rules have been captured, revoke the capturing authority, which is possible whenever the
  revocation threshold can be met. A high threshold to revoke is still revocable.

- Else, if even revocation is blocked because the threshold is set unreachably high, the floor is **fork and
  exit**, which needs no threshold and no one's permission (§2.5).

**The two asymmetries rest on one foundation.** Capability may consolidate freely because capability is not
authority; authority may consolidate freely because authority is revocable and exitable. Both are safe for
the same underlying reason:  peers of equal standing hold their own edges (§2.1, local-first), so neither capability nor
authority can become a trap. The permissiveness at the top of the spectrum, that a group may concentrate
authority almost arbitrarily, is underwritten by the floor at the bottom, that any concentration can always
be escaped by a primitive the concentration cannot reach (§2.4, §2.5). `Synthesis.`

**A corollary governs which mechanisms may be defaults.** A mechanism **MAY** be a default only if it is
**party-neutral**; any mechanism that can privilege a party **MUST** be opt-in and itself governed, because
a settable advantage is a lever of authority and a self-assertable one would be a backdoor to power that
bypasses the vote. A party-neutral mechanism, an ordering over kinds of operation or a canonical tiebreak
that is a deterministic coin flip, confers no advantage on anyone and may ship as a default; a
party-privileging one, such as assignable weights that let some facts outrank others, is a concentration, so
it is permitted but must be revocable and governed like any other authority, never a silent default. This is
the concentration principle applied at the smallest scale, and its mechanism side is Part 2 §7.3.1 (the
configurable tiebreak keys). `Synthesis.`

---

### 2.8. Faithful representation: what the substrate is for

Everything above resolves into one statement of what Drystone is for, and that statement draws the line
between what the protocol does and what a product built on it does. It is a capstone, not a new obligation:
it names the single purpose the razor (§2.0), the fork terminus (§2.5), peer-equality (§2.3), and
field-integrity (§2.6) jointly serve.

**The protocol faithfully represents individual, local, personal choice, and never centrally resolves
social conflict, because social conflict has no central resolution, only individual responses that
aggregate.** When people genuinely disagree about their shared space, no authority can decide correctly on
everyone's behalf, because the right outcome differs per person. The faithful thing, and the only
center-free thing, is to let each persona act as themselves (stay, leave, mute, join both) and to let the
group-level outcome be the **aggregate** of those choices, a realization of the social adjudication rather
than an input to it. The anchoring example is a literal divorce inside a shared friend group: when two
groups want to merge but one has removed a member of the other, there is no technical fix and no correct
group-level answer, only each mutual friend deciding, for themselves, where they want to be. A system that
forced a single group-level answer would be imposing a fiction, exactly as a central host resolving a
divorce's fallout would. (The mechanism that carries this is Part 2 §7.6, the fork placement that lets a
voter, a bystander, and a subject each land where their own choice puts them.)

This divides the labor cleanly, and the division is the frame for the whole social layer:

> Making it possible faithfully is Drystone. Making it representable and as easy as possible is the product
> layer.

- **The protocol (Drystone) is composable, unopinionated, and safe in every configuration.** Its job is
  *faithful possibility*: the guarantee that every local, personal choice can be represented truthfully and
  that every configuration is safe. It provides mechanisms and imposes no social shape. That it gets
  complicated under composition is correct at the protocol layer, because it must support all social shapes
  safely rather than choose one.

- **The product is opinionated, legible, and defaulted by temperament.** Its job is *easy
  representability*: choosing which of the protocol's possibilities to surface, and defaulting them to a
  coherent, temperament-matched posture, so most people never touch a dial. The product makes the choices
  usable; the protocol makes them possible and honest.

The two are complementary, not in tension: the protocol's refusal to decide is what makes the product's
opinions safe to hold, because no product default can trap anyone when the underlying protocol always
permits mute, revoke, fork, and exit. Drystone's contribution is faithful possibility; opinion, ease, and
legibility are the product's. The mechanism side of this division is Part 2 §7.6.9 (the posture dials, and
the rule that the protocol stays composable while the product stays opinionated). `Synthesis.`

---

## 3. Why these principles are corroborated, not invented

The razor is underrepresented in shipped systems, but it is not a wilderness. Thinkers who never
collaborated, arguing from different starting points, independently arrived at pieces of the same
conclusion. That cross-field corroboration is the strongest support the design has, and notably, *how
long ago* any of them wrote is not the argument. A claim is not stronger for being old; one node right
against all the others is still right, and correctness does not accrue with time. What matters is that
independent witnesses, with no shared agenda, describe the same structure. The grounding below leads with
the conclusion each discipline supplies, then the verbatim source as the outside corroboration, in the
spirit of: state the ground, then show who else, unprompted, found it.

Each quote is preserved whole, with its citation and a per-quote verification flag.

> **Why this convergence persisted unassembled, the structural reason Drystone's synthesis was
> available to find.** The grounding below spans six fields that rarely cite one another: distributed
> systems, ethics, economics, systems science, epistemology, and political science, plus the
> governance-theory frontier (Part 2 Appendix C). Each field stops before the fusion for its own reason.
> The distributed-systems camp stops at the data layer because governance is not its problem. The
> governance-theory camp (e.g. Modular Politics) stops at the conceptual layer because wire protocols are
> not its craft, its own scope note brackets "security and database structures." The blockchain camp
> cannot take the fork as a first-class *good* because its economics treat chain splits as
> value-destroying. The multi-agent camp stays in a knowledge-locality framing and keeps trying to
> compute a verdict. Assembling the fusion requires fluency across all of them plus the disposition to
> treat an impossibility as generative. That is why the convergence below was real and yet unassembled
> into a wire obligation with conformance vectors, which is the gap Drystone fills, and the honest scope
> of its novelty. **This is not a claim of technical supremacy. It is a claim that established technical
> results, taken seriously, *force* a humane shape for social governance, the references below are the
> shoulders this stands on, named so the lineage is transparent.**

**Distributed systems, the formal spine: no global truth is an established result, not a design taste.**
This is the field Drystone most directly lives in, and it supplies the *formal* grounding for the razor:
the impossibility results below are why local-first is **forced**, not preferred (§1, §2.0). They are
listed first because the four named principles are, in large part, these theorems read as obligations.

*There is no global clock; causal order is partial.* Lamport's foundational result establishes that a
distributed system has no shared time reference and that the only honest ordering of events is the partial
"happened-before" relation, a total order requires *adding* an arbitrary tiebreak the causal structure
does not itself contain:

> "The concept of time is fundamental to our way of thinking... we will see that it is not always
> possible to say that one of two events occurred first. The relation 'happened before' is therefore only
> a partial ordering of the events in the system."
> L. Lamport, "Time, Clocks, and the Ordering of Events in a Distributed System," *CACM* 21(7) (1978) ·
> **[confirm, verbatim against the primary paper].**

This is the field's own statement of §2.0.1 (time is not a corroborable fact) and Part 2 §7.3.1 (order causally,
break ties cryptographically, the deliberately-arbitrary-but-deterministic total-order extension, never
a wall-clock). *The seam, kept honest:* Lamport also showed how clocks can be used to **build** a total
order; he was not arguing that wall-clocks are an attack surface. He supplies the structural fact (no
global clock; happened-before is partial); the design consequence (therefore exclude the wall-clock from
authority) is Drystone's inference built on his foundation, not his claim.

*Under partition, you cannot have both consistency and availability.* The CAP theorem is the impossibility
that forces the local-first choice: a center-free system that stays available when the network splits
**must** give up a single always-consistent global state, which is exactly why local-canonical-plus-
reconciliation is the honest model and not a preference:

> a shared-data system cannot simultaneously provide Consistency, Availability, and Partition tolerance;
> since partitions are unavoidable, a system must trade consistency against availability.
> S. Gilbert & N. Lynch, "Brewer's Conjecture and the Feasibility of Consistent, Available,
> Partition-Tolerant Web Services," *ACM SIGACT News* 33(2) (2002), formalizing E. Brewer's 2000 PODC
> conjecture · **[confirm, statement against the primary paper].**

*The seam:* CAP is specifically about linearizable reads/writes on shared storage under network partition;
Drystone stretches it exactly as far as "no always-consistent global state while staying available," not
to "no shared truth of any kind."

*Coordination-free consistency is exactly the monotonic fragment, and this is the formal statement of the
razor's resolvable/irreducible split.* The CALM theorem is the spine of both §2.2 (the fold resolves the
monotonic majority with no coordination) and §2.5 (the non-monotonic residue provably cannot):

> "A problem has a consistent, coordination-free distributed implementation if and only if it is
> monotonic." Monotonic problems are safe under missing information and need no coordination; non-monotonic
> problems must wait for all information to arrive, and so require coordination.
> J. M. Hellerstein & P. Alvaro, "Keeping CALM: When Distributed Consistency is Easy," *CACM* 63(9)
> (2020); conjectured at PODS 2010, proven by Ameloot, Neven & Van den Bussche · *(Verified: the statement matches Theorem 1 verbatim against the CACM primary source.)*

Read forward, CALM justifies the coordination-free fold; read backward (the *only-if*), it says the
genuinely non-monotonic case has no coordination-free resolution, which is §2.5's forced terminus stated
as a theorem rather than a preference. *The seam:* CALM is about *consistency*; Drystone's application to
*social* non-monotonic operations (mutual expulsion) is "the CALM boundary applied to governance," a
defensible application, not a claim that CALM itself speaks to social adjudication.

*Convergence without a coordinator is not hypothetical; it is formalized and proven.* CRDTs are the
existence proof that the center-free data plane is buildable, and they illustrate the boundary from the
mechanism side:

> under a Strong Eventual Consistency model, replicas of a Conflict-free Replicated Data Type are
> guaranteed to converge, with no need for consensus or remote synchronisation, despite any number of
> failures.
> M. Shapiro, N. Preguiça, C. Baquero & M. Zawirski, "Conflict-free Replicated Data Types," *SSS 2011*
> (Stabilization, Safety, and Security of Distributed Systems) · **[confirm, statement
> against the primary paper].**

*The seam, and it is the instructive one:* CRDTs converge precisely *because* they are restricted to
operations whose merge is deterministic, the same monotonic class CALM describes, and exactly **not** the
social-contradiction residue. So CRDTs ground Drystone's data plane (Part 2 §4, Part 2 §7.1) and simultaneously show
*why* governance needs the Part 2 §7.6 escalation: the convergence guarantee holds for the resolvable class and
stops exactly where utility begins. Same boundary, seen from the mechanism side.

Taken together, these four results are why Part 1 says local-first is *derived, not chosen*: no global
clock (Lamport), no consistent-and-available global state under partition (CAP), coordination-free only in
the monotonic fragment (CALM), and convergence-without-a-coordinator demonstrably real for that fragment
(CRDTs). The humane consequences (equal standing, surfaced disagreement, the fork) are what these technical
facts *force* once you decline to hide the center.

**Ethics, silencing the dissenter is a cost no center can justify.** This is the moral spine of *let the
dissenter fork.* Mill names the cost directly:

> "If all mankind minus one were of one opinion, and only one person were of the contrary opinion,
> mankind would be no more justified in silencing that one person, than he, if he had the power, would
> be justified in silencing mankind."
> J. S. Mill, *On Liberty* (1859) · *Verified.*

So the fork must always stay available as the dignified exit, which is why the protocol refuses to
algorithmically adjudicate a social dispute (Part 2 §7) and refuses moderation-as-surveillance (Part 2 §8),
accepting instead that each node determines its own relationship to moderation and to the outcomes of
adjudication. This is the ethical statement of the forced terminus of §2.5: the fork is not a fallback,
it is the only non-coercive output when a value is genuinely contested.

Socrates is the older root of the same ethic, and the one that names the failure mode directly. In
Plato's *Apology* (399 BCE) the method is detection, not oracle: it surfaces contradiction rather than
imposing an answer, and the error it diagnoses is *supposing you know what you do not* — "he supposes he
knows something when he does not know, while I, just as I do not know, do not even suppose that I do"
(Plato, *Apology* 21d) [confirm, verbatim against a primary edition (Fowler trans.); phrasing varies by
translation] — which is exactly the error of a truth-certifying system that mistakes consensus for
knowledge. *Drystone's reading, not Plato's words:* the razor's refusal to compute a verdict is that
Socratic humility made structural, the system surfaces the disagreement and declines to pronounce on it. A
citation caution the design keeps for itself, in the spirit of the razor: the popular paraphrase "I know
that I know nothing" appears nowhere in Plato; the grounded text is *Apology* 21d–22d, where the accurate
line is "For I was conscious that I knew practically nothing" (Plato, *Apology* 22d) [confirm, verbatim
against a primary edition (Fowler trans.); phrasing varies by translation] — the distinction between
holding a view and certifying it as known, not a boast of knowing nothing at all.

**Economics, the knowledge a center would need cannot be centralized.** Decisions belong at the edges
because the relevant knowledge only exists there:

> "The peculiar character of the problem of a rational economic order is determined precisely by the
> fact that the knowledge of the circumstances of which we must make use never exists in concentrated or
> integrated form but solely as the dispersed bits of incomplete and frequently contradictory knowledge
> which all the separate individuals possess."
> F. A. Hayek, "The Use of Knowledge in Society" (1945) · *Verified verbatim.*

"Incomplete and frequently contradictory" is a distributed system's actual condition, named in 1945.
Much of the judgment a human brings to a divergence is inarticulable local knowledge no center can
capture, which is *why utility cannot be computed* and decisions, and value judgments, belong at the
edges. Note the careful boundary, which the razor demands of itself: this is an argument that the
*utility* layer is irreducibly local, **not** a claim that everything is local, the provenance layer is
exactly what *can* be made global and checkable, and §2.5's residue is precisely the part Hayek's
argument bites and the fold does not.

**Systems science, plurality is a survival condition, not a preference.** Plurality is a necessary
condition for the health of nodes that are equal peers and that, by nature, hold diverging views of what
is corroborated and what is merely asserted:

> "Only variety can destroy variety."
> W. R. Ashby, *An Introduction to Cybernetics* (1956), p. 207 (the Law of Requisite Variety) · *Verified.*

A regulator that collapses plural perspectives into one certified truth is brittle by formal law: it
sits below the variety it governs, so it cannot represent every state, and it survives only by *pruning*
the variety it cannot hold, removing from the system the very parts reality will later require.
Preserving plural perspectives (allowing forks, refusing to collapse divergence) is requisite variety
made architectural. **Plurality is the robustness.**

Stafford Beer turned this law into a design discipline, and it is the formal grounding for Part 2's
*escalate-the-hard-case-to-a-human* posture. Beer's rule was to **not over-specify**:

> "instead of trying to specify in full detail, you specify it only somewhat. You then ride on the
> dynamics of the system in the direction you want to go."
> S. Beer, *Brain of the Firm* (1972) · **[confirm]** (defensible verbatim; confirm
> against the primary edition).

Because you have not specified every case in advance, the system will meet cases the rules never
anticipated, so you need a channel for those cases to surface. Beer named it the **algedonic** channel
(Greek *algos*, pain + *hedone*, pleasure): a low-bandwidth, high-priority alarm that **bypasses the
normal hierarchy and carries no analysis**, "this hurts, a human needs to look now." It is not a fallback
or an admission of failure; it is a designed channel, as load-bearing as the automated path, because no
filter is perfect and the cost of silently absorbing a missed hard case is too high. In Drystone this is
exactly the **hard-stop-and-escalate** rule (Part 2 §7.6) and the **label-not-enforce** posture (Part 2 §8): the
machine *annotates rather than acts*, surfacing the signal and leaving the decision with a person, which
is also what keeps adjudication, and therefore personahood, distributed (Part 2 §3, Part 2 §5.2). The §2.5 residue
*is* Beer's "case the rules never anticipated," and the fork is what the dynamics ride toward when the
rules correctly decline to decide.

The historical natural experiment that grounds this is **Cybersyn vs OGAS**, two early-1970s attempts to
run an economy by computer, with opposite assumptions about *where judgment lives*. Beer's Cybersyn
(Chile) pushed autonomy to the edge and escalated only the residue to humans in a room; during the October
1972 truckers' strike it let the government move what mattered on **10–30% of normal transport capacity**.
The Soviet OGAS routed adjudication to a Moscow apex and was strangled before it ran, not by a technical
limit but by the institutional self-interest a single point of control invites. The lesson is not
"decentralized vs centralized infrastructure" (both had distributed sensing) but **where adjudication
lives**, which is the thread Part 2 §3 makes structural. (Beer's stance that the computer should serve
human autonomy rather than excuse automatic command is the spine here; the often-quoted "aids to human
viability, not excuses for automatic command" phrasing is a synthesis gloss, not a verbatim Beer line.)
**[confirm, Cybersyn capacity figures and the OGAS history against primary sources.]**

**Internet governance, from the discipline that runs the namespaces.** The
formal specification tradition of the Internet age has already reconciled
"the machine must stop and a person must decide" into its own machinery, in
both of the shapes Drystone uses. The first is a named human role inside the
procedure: the IANA registration policies (RFC 8126, BCP 26) define the
**Designated Expert**, a person to whom the evaluation of a registration
request is formally delegated:

> "IANA forwards requests for an assignment to the expert for evaluation, and
> the expert (after performing the evaluation) informs IANA as to whether or
> not to make the assignment or registration." ... "The list of designated
> experts for a registry is listed in the registry."
> RFC 8126 (BCP 26), §5.2 · *Verified against the primary at edit time.*

The second is a normative human-input step inside an algorithm: the W3C
platform defines **powerful features** that a user agent must not exercise
without the end-user's express permission, enforced by check-permission steps
the API methods normatively depend on:

> "A powerful feature is a web platform feature (usually an API) for which a
> user gives express permission before the feature can be used." (W3C
> Permissions.) "Geolocation is a powerful feature that requires express
> permission from an end-user before any location data is shared with a web
> application." (W3C Geolocation, current CR.)
> W3C Permissions (TR) and Geolocation (current CR) · *Verified against the
> primaries at edit time; the 2016 Geolocation REC predates the
> powerful-feature framing, so its wording differs from the current CR quoted
> here.*

And the IETF publishes the semantics of its own human layer in the same
document series as its wire formats: rough consensus is objections addressed,
not votes counted:

> "Rough consensus is achieved when all issues are addressed, but not
> necessarily accommodated."
> RFC 7282, §3 · *Verified against the primary at edit time.*

The corroboration cuts one level deeper than precedent. RFC 8126 shows a
formal spec can say, at this step a person decides, and survive as running
governance for decades. What Drystone changes is the *locus*, not the move:
where the registry delegates the judgment to one named expert per registry (a
center), Drystone distributes the designated-expert role to every principal.
The **local authority** is the designated expert for its own utility
judgments, because a persona only represents a person if that person holds
the adjudication (§2.3, §2.5). Beer supplies the channel (escalate the
residue, keep authority where the context lives, §3 above); the IANA
tradition supplies the named human step inside formal machinery; Drystone
composes the two and removes the center.

**Epistemology, finitude is the design condition.** That every node is permanently, structurally
ignorant of most of what the others are doing is not a defect to engineer away but the condition to
design *for*:

> "Our knowledge can be only finite, while our ignorance must necessarily be infinite."
> K. Popper, *Conjectures and Refutations* (1963), §XVII (p. 30) · *Verified.*

A node has effectively infinite ignorance about what other nodes are doing. Accepting that is what lets
the protocol design for real conditions instead of a pretended global view: theories are never verified,
only corroborated by surviving refutation, so a persona's history is a conjecture, agreement is
corroboration, and a claim is only ever *not-yet-overturned*. That is precisely the shape of provenance.

Charles Sanders Peirce supplies the epistemology's other half, the community rather than the individual
conjecture. Peirce's fallibilism holds that no belief is ever absolutely certain and that the cardinal
methodological sin is to block the road of inquiry — his first rule of reason, "Do not block the way of
inquiry" (Peirce, *Collected Papers* 1.135) [confirm, verbatim against the primary; phrasing varies by
edition]; truth on his account is not possessed but approached, what a *community of inquiry* converges
toward over unlimited time. (The fallibilism and convergence formulations are attributed synthesis of
Peirce's position, not verbatim clauses, and *community of inquiry* is his term of art.) That is Drystone's
gossip model with humility built into the mechanism: heads cross-checked across peers, agreement treated as
corroboration and never as proof, the road left open. *Drystone's reading:* Popper grounds the single
node's claim as *not-yet-overturned*, while Peirce grounds the many nodes' convergence as never-finished,
and the pairing is why provenance is computed while a final verdict is not, the community converges without
any of its members ever holding the certified answer.

**Political science, self-governance without a sovereign arbiter is documented to work.** This is the
empirical answer to "is this utopian?", no. Ostrom's Nobel-recognized study of long-lived commons
records communities governing shared resources for generations without privatization or a central
authority. Two of her eight design principles map directly onto Drystone, and these are confirmable to the
primary 1990 text:

> Principle 6, "Conflict-resolution mechanisms: appropriators and their officials have rapid access to
> low-cost local arenas to resolve conflicts among appropriators or between appropriators and officials."
> Principle 7, "Minimal recognition of rights to organize: the rights of appropriators to devise their
> own institutions are not challenged by external governmental authorities."
> E. Ostrom, *Governing the Commons: The Evolution of Institutions for Collective Action* (1990),
> design principles 6 and 7 · **[confirm, verbatim wording against the 1990 primary].**

Principle 6 is Drystone's accessible conflict-resolution-at-the-edge: in Ostrom's terms a community of
people has low-cost local arenas to resolve its own conflicts, and Drystone realizes that value as the Part 2 §7.6
hard-stop surfacing to the affected **group** (the people), adjudicated within the capital-G **Group** that
manifests them, never relocated to a center. Principle 7 is `P-Peer-Equality`'s right to self-organize and
`P-Durable-Enablement`'s fork: no external authority may forbid a **group** of people from devising or
re-forming its own institutions, which Drystone realizes as the capital-G Group's unforbiddable fork. (The
lowercase **group** here is deliberate: Ostrom's principles are about human communities, and the homage
holds only if her semantics are preserved; the capital-G Group is named as the *realization* of the value,
not as a rewrite of her claim.)

The capstone, **subsidiarity**, comes from the *later* generalization and must be cited as such, not
attributed to the 1990 book:

> governance tasks are assigned "by default to the lowest jurisdiction, unless explicitly determined to
> be ineffective."
> D. S. Wilson, E. Ostrom & M. E. Cox, "Generalizing the core design principles for the efficacy of
> groups," *Journal of Economic Behavior & Organization* 90S (2013) · **[confirm, verbatim
> against the 2013 paper; this wording is the 2013 generalization, distinct from the 1990 principles
> above].**

In a peer-to-peer system the "lowest jurisdiction" is the edge node, and every node is an edge node, so
governance stays local where consent is cheap and federates only the irreducible minimum. Note that
Ostrom's work is *governance theory*; it documents the principles durable commons exhibit (and the
digital-commons literature observes communities implement them "often without naming them"), but it
stops short of a wire protocol. The move from "these principles work" to "here is the byte-level
obligation a conforming implementation must meet" is exactly the value-to-mechanism gap Drystone is built
to close, and the reason citing Ostrom is corroboration of the *values*, not of the mechanics.

**The convergence is the corroboration.** Witnesses who never met, formal results from distributed
systems and arguments from ethics, economics, systems science, epistemology, and political science, give
the design its strongest support: no center can hold the truth; plurality is a survival condition for a
system of equal nodes; and the honest design surfaces disagreement and leaves judgment at the edges. The
distributed-systems theorems make the shape *forced* (you cannot build the center even if you wanted to);
the human-sciences arguments make it *humane* (you should not, even where you could). That pairing,
technical necessity meeting humane alignment, is the ground Part 2 is built to stand on, and it is the
whole of the claim: not technical supremacy, but principled delivery of an established technical reality
in a form that matches how human governance actually has to work.

**The meta-principle: the ethical choice and the technical strength keep turning out to be the same
choice, not a happy accident.** The convergence above is read one direction throughout this section, that
the technical facts *force* a humane shape (§3, and "humane consequences are what these technical facts
force," above). The design dialogue observed the identity runs the other way too, and stated it three
times as a recurring finding worth promoting from coincidence to principle: the ethical commitment
*forces* the technical strength. Refusing extraction is not a moral gloss layered onto a design chosen on
other grounds; it is what forecloses the center, and foreclosing the center is exactly what delivers the
resilience, the availability-under-partition, and the metadata-privacy the design claims. Run it either
way and it closes: decline to hide the center and you get the humane consequences; decline to extract and
you get the resilient architecture. That two-way identity, and not either half alone, is the reason the
design is defensible rather than merely idealistic, an idealism that happened to be free would be
suspicious, whereas an idealism that is *forced by* the engineering, and forces it in return, is load-
bearing. It is stated here as a first-class principle rather than left as the fortunate coincidence it
first appears to be. (Recurring meta-observation of the design dialogue; *Drystone's own framing*, offered
as synthesis, not attributed to any external source.)

---

## What Part 1 establishes (and does not)

**Establishes:** the protocol's shape is *derived*, not preferred, from a structural fact about what a
node in a distributed system can know. The razor (provenance, not utility), the time-is-not-a-fact
corollary (§2.0.1), and the four principles (`P-Local-Truth`, `P-Knowable-Truth`, `P-Peer-Equality`,
`P-Durable-Enablement`) are the obligations the wire must meet, each ending in a consequence Part 2
realizes. The forced terminus (§2.5), fork-not-verdict for the provably-non-empty, intrinsically-utility
residue, is the move that most distinguishes the design, and it is *derived*, not chosen: it is what the
razor plus the CALM boundary jointly require. A bridging dependency (§2.6) names that the **voice** right
requires field-integrity, peer-governed, legible, exitable ordering, which is why the substrate's
ownership form is in scope for the motivation; it adds no new principle and no wire obligation, only the
link from voice to the field it is asserted into.

**Does not establish:** that local-first *guarantees* a humane system; it is necessary, not sufficient
(the edge can be wrong too; honest friction between real nodes is the cost, distinct from the
manufactured friction a center imposes). Nor does it settle the open check on the rights set
(`tenure` under re-key; the candidate `share` right is dropped, see §2.3), or the capped-vs-uncapped-root question against the Matrix steelman
(§2.3, Part 2 §7.3, Appendix B), or supply the still-pending verbatim grounding (Beer) and the
primary-source confirmations (Ostrom, and the Cybersyn/OGAS history) flagged above. The **Matrix** and
**CALM** external-fact claims are now confirmed against primaries (Part 2 Appendix C), so they are no
longer pending. The novelty claim is scoped honestly: *synthesis and terminus, unoccupied against the closest
published neighbors*, not "first ever", see Part 2 Appendix C.

---

## References (Part 1)

These references exist for transparency and to acknowledge connected and prior art. **Nothing here is a claim of technical supremacy or priority.** The thesis of this document is the opposite: that established results across several fields, taken seriously, *force* a humane shape for social governance, and that Drystone's contribution is the synthesis and the delivery, principled delivery of a technical reality in a form aligned to how human governance actually works, not the invention of the underlying results. Where a source grounds a *value* rather than a *mechanism*, that is noted; where this document leans on a secondary or recent source rather than the primary, that is noted too.

Verification legend: *Verified*, quoted/checked against the primary this round or a prior round; **[confirm]**, load-bearing and to be confirmed against the cited primary before any external publication. Per the document's own discipline, no specific date, attribution, or wording flagged **[confirm]** should be treated as settled until pulled from the primary.

### Distributed systems (the formal spine, §2.0.1, §2.2, §2.5, and Part 2 §7)

- Lamport, L. "Time, Clocks, and the Ordering of Events in a Distributed System." *Communications of the ACM* 21(7), 1978. Grounds: no global clock; "happened-before" is a partial order; total order requires an arbitrary tiebreak. Supports §2.0.1 and Part 2 §7.3.1. **[confirm, verbatim]**. *Seam:* supplies the structural fact, not the design consequence that wall-clocks are an attack surface (that inference is Drystone's).

- Gilbert, S. & Lynch, N. "Brewer's Conjecture and the Feasibility of Consistent, Available, Partition-Tolerant Web Services." *ACM SIGACT News* 33(2), 2002, formalizing Brewer, E., PODC 2000 keynote ("Towards Robust Distributed Systems"); see also Brewer, E., "CAP Twelve Years Later," *IEEE Computer* 45(2), 2012. Grounds: under partition, consistency and availability cannot both hold, which forces local-first (§1, §2.1). **[confirm, statement]**. *Seam:* about linearizable shared storage under partition; not "no shared truth of any kind."

- Hellerstein, J. M. & Alvaro, P. "Keeping CALM: When Distributed Consistency is Easy." *Communications of the ACM* 63(9), 2020. Conjectured at PODS 2010 (Hellerstein, "The Declarative Imperative"); proven for queries by Ameloot, T. J., Neven, F. & Van den Bussche, J., "Relational Transducers for Declarative Networking," *J. ACM* 60(2), 2013. Grounds: coordination-free consistency iff monotonic, the formal statement of the razor's resolvable/irreducible split (§2.2 forward, §2.5 backward). **[confirm, statement]**. *Seam:* about consistency; the application to social non-monotonic operations is "CALM applied to governance," not a CALM claim.

- Shapiro, M., Preguiça, N., Baquero, C. & Zawirski, M. "Conflict-free Replicated Data Types." *SSS 2011* (13th Int'l Symposium on Stabilization, Safety, and Security of Distributed Systems). See also the companion INRIA RR-7506, "A Comprehensive Study of Convergent and Commutative Replicated Data Types," 2011. Grounds: convergence without consensus is real and proven for the monotonic class, the existence proof for the center-free data plane (Part 2 §4, Part 2 §7.1). **[confirm, statement]**. *Seam:* converges precisely because restricted to deterministic-merge operations, the same boundary that makes the governance residue need Part 2 §7.6.

### Ethics (§2.5, §3, and Part 2 §7.6, Part 2 §8)

- Socrates (Plato, *Apology*, 399 BCE). Grounds the value at its root: method as detection-not-oracle, and the diagnosis of the truth-certifying error (supposing you know what you do not). Carries a citation correction, "I know that I know nothing" is a paraphrase not in Plato; the grounded text is *Apology* 21d–22d. **[confirm, verbatim against a primary edition (Fowler trans.); verbatim clauses held for a later pass].**

- Mill, J. S. *On Liberty*, 1859. Grounds the value: silencing the dissenter is an unjustifiable cost, hence the fork as the dignified exit. *Verified.*

### Economics (§2.0, §3)

- Hayek, F. A. "The Use of Knowledge in Society." *American Economic Review* 35(4), 1945. Grounds the value: the knowledge a center would need is irreducibly dispersed, so utility cannot be computed centrally. *Verified verbatim.* *Seam:* an argument about the *utility* layer; the *provenance* layer is exactly what can be made global and checkable.

### Systems science (§2.3, §3, and Part 2 §7.6, Part 2 §8)

- Ashby, W. R. *An Introduction to Cybernetics*, 1956 (the Law of Requisite Variety, p. 207). Grounds: only variety can absorb variety; collapsing plurality is brittle by formal law. *Verified.*

- Beer, S. *Brain of the Firm*, 1972 (the algedonic channel; the "specify only somewhat" design discipline). (Grounds the escalate-the-hard-case posture (§2.5, Part 2 §7.6). **[confirm) the "specify only somewhat" verbatim against the primary edition]**. The Cybersyn/OGAS natural experiment and its capacity figures are **[confirm]** against primary sources; the "aids to human viability, not excuses for automatic command" phrasing is a synthesis gloss, not a verbatim Beer line, and is labeled as such.

### Epistemology (§2.0, §2.2, §3)

- Popper, K. *Conjectures and Refutations*, 1963 (§XVII). (Grounds: knowledge is finite, ignorance infinite; theories are corroborated by surviving refutation, never verified) which is the shape of provenance (a claim is only ever *not-yet-overturned*). *Verified.*

- Peirce, C. S. (fallibilism and the *community of inquiry*). Grounds the community-convergence half of the epistemology: no belief is ever absolutely certain, the road of inquiry must not be blocked, and truth is what a community converges toward over unlimited time (never possessed), the humility-in-the-mechanism shape of Drystone's gossip model. **[confirm]** (phrasings vary across Peirce's essays; verbatim wording held for a later pass, confirm against the primary before quoting).

### Political science / commons governance (§2.3, §2.4, §3, and Part 2 §7.6)

- Ostrom, E. *Governing the Commons: The Evolution of Institutions for Collective Action.* Cambridge University Press, 1990 (design principles 6, conflict-resolution mechanisms; and 7, minimal recognition of the right to organize). Grounds: self-governance without a sovereign arbiter is empirically durable; the two mirrored principles map to Part 2 §7.6 escalation-at-the-edge and to the right to self-organize/fork. **[confirm, verbatim wording of principles 6 and 7 against the 1990 primary]**.

- Wilson, D. S., Ostrom, E. & Cox, M. E. "Generalizing the core design principles for the efficacy of groups." *Journal of Economic Behavior & Organization* 90S, 2013 (subsidiarity: lowest jurisdiction unless ineffective). Grounds the subsidiarity capstone. **[confirm, verbatim; distinct from the 1990 book, do not conflate]**.

### Governance-as-protocol frontier (the nearest neighbor in intent, see Part 2 Appendix C.4)

- Schneider, N., De Filippi, P., Frey, S., Tan, J. Z. & Zhang, A. X. "Modular Politics: Toward a Governance Layer for Online Communities." *Proc. ACM Human-Computer Interaction* 5 (CSCW1), 2021. The closest published governance-as-protocol work; shares the ambition, roots authority in a platform operator, and brackets the resolution mechanics and wire encodings, the layer Drystone's Part 2 builds. **[confirm, quotations against the CSCW paper]**.

- Spritely Institute (Lemmer-Webber, C., Executive Director), "Technical Values and Design Goals" (spritely.institute/about) and the W3C ActivityPub lineage (Lemmer-Webber lead author). Grounds the **contextual-identity** posture of §2.3 / Part 2 §5.6: no global town square, contextual flows over context collapse, the principle that one should not claim guarantees one cannot provide, and trust as contextual/revocable rather than all-or-nothing. Supports Drystone's stance that personhood is a contextual group judgment, not a protocol guarantee, and that legitimate multiple self-presentation is part of the social substrate. The petname tradition (Spritely Brux; Stiegler) is the nearest prior art for human-meaningful naming over non-human-meaningful keys, relevant where Drystone later addresses naming. Quotations verified verbatim against the primary page.

> **A consolidated, component-by-component prior-art map (covering the data layer (CALM, CRDTs, Willow/Meadowcap/Keyhive), the resolution layer (Matrix State Resolution and the 2025 CVEs, MSC4289/4291/4297), the cryptographic group-state layer (MLS, decentralized-MLS / FREEK), and the governance-as-protocol frontier) lives in Part 2 Appendix C, and the substrate requirement-vs-realization treatment (MLS, iroh, and the primitives) is Part 2 §10.** Part 1's references are the *principled* lineage; Part 2's are the *mechanism* lineage. Both exist for the same reason: to name the shoulders this stands on, transparently, and to make clear that the claim is synthesis and humane delivery, not invention or supremacy.

---

## Upstream reference links (versioned)

Canonical, edition- or version-specific sources for the principled lineage above, so a reader resolves the exact work cited rather than a later edition or a secondary summary. Part 1's sources ground *values and formal results*; the mechanism lineage (MLS, iroh, Willow, Matrix, and the software versions) is in Part 2's own versioned links section. Where a source was checked against its primary, it is marked *(verified)*; where a specific date, wording, or edition is still to be pulled from the primary, it carries **[confirm]** consistent with the citations above.

### Distributed-systems spine

- **Lamport, L.**, "Time, Clocks, and the Ordering of Events in a Distributed System," *Communications of the ACM* 21(7), July 1978, pp. 558–565. https://doi.org/10.1145/359545.359563 . Grounds §2.0.1, Part 2 §7.3.1 (partial order; total order needs an arbitrary tiebreak). **[confirm verbatim.]**

- **Gilbert, S. & Lynch, N.**, "Brewer's Conjecture and the Feasibility of Consistent, Available, Partition-Tolerant Web Services," *ACM SIGACT News* 33(2), 2002, pp. 51–59. https://doi.org/10.1145/564585.564601 . With Brewer's PODC 2000 keynote and "CAP Twelve Years Later," *IEEE Computer* 45(2), 2012 ( https://doi.org/10.1109/MC.2012.37 ). Grounds §1, §2.1. **[confirm statement.]**

- **Hellerstein, J. M. & Alvaro, P.**, "Keeping CALM: When Distributed Consistency is Easy," *CACM* 63(9), 2020, pp. 72–81. https://cacm.acm.org/research/keeping-calm/ (arXiv https://arxiv.org/abs/1901.01930 ). Conjectured PODS 2010; query proof Ameloot, Neven & Van den Bussche, *J. ACM* 60(2), 2013. Grounds the §2.5 resolvable/residue split. *(Attribution and statement verified this revision.)*

- **Shapiro, M., Preguiça, N., Baquero, C. & Zawirski, M.**, "Conflict-free Replicated Data Types," *SSS 2011* (LNCS 6976), pp. 386–400, https://doi.org/10.1007/978-3-642-24550-3_29 ; companion INRIA RR-7506, "A Comprehensive Study of Convergent and Commutative Replicated Data Types," January 2011, https://hal.inria.fr/inria-00555588 . Grounds Part 2 §4, Part 2 §7.1 (convergence without consensus, for the monotonic class). *(Venue, DOI, and report number verified this revision.)*

### Ethics, economics, systems science, epistemology

- **Socrates / Plato**, *Apology*, 399 BCE (Fowler trans., Loeb; e.g. https://www.perseus.tufts.edu/hopper/text?doc=Plato+Apol. ). Grounds the original-fallibilism ethics root (§3); method-as-detection, and the correction that "I know that I know nothing" is a paraphrase not in Plato (grounded text *Apology* 21d–22d). **[confirm verbatim against a primary edition; clauses held for a later pass.]**

- **Mill, J. S.**, *On Liberty*, 1859. Canonical text: https://www.gutenberg.org/ebooks/34901 . Grounds the fork-as-dignified-exit value (§2.5). *(Verified.)*

- **Hayek, F. A.**, "The Use of Knowledge in Society," *American Economic Review* 35(4), 1945, pp. 519–530. https://www.jstor.org/stable/1809376 . Grounds §2.0, §3 (dispersed knowledge; utility is not centrally computable). *(Verified verbatim.)*

- **Ashby, W. R.**, *An Introduction to Cybernetics*, Chapman & Hall, 1956 (the Law of Requisite Variety, p. 207). Canonical scan: http://pcp.vub.ac.be/books/IntroCyb.pdf . Grounds §2.3, §3 (only variety absorbs variety). *(Verified.)*

- **Beer, S.**, *Brain of the Firm*, 1972 (2nd ed. Wiley, 1981), the algedonic channel and the "specify only somewhat" discipline. Grounds §2.5, Part 2 §7.6. **[confirm the "specify only somewhat" wording against the primary edition; the Cybersyn/OGAS figures and the "aids to human viability" gloss are [confirm] / labeled synthesis.]**

- **Cotton, M., Leiba, B. & Narten, T.**, "Guidelines for Writing an IANA Considerations Section in RFCs," RFC 8126 (BCP 26), 2017. https://www.rfc-editor.org/rfc/rfc8126 . Grounds the named-human-step precedent, a formal spec that delegates evaluation of a registration request to a Designated Expert (§2.5, §3, Part 2 §7.6). *(§5.2 quotations verified verbatim against the primary this revision.)*

- **W3C Permissions** (TR), https://www.w3.org/TR/permissions/ , and **W3C Geolocation** (current Candidate Recommendation), https://www.w3.org/TR/geolocation/ . Ground the normative human-input-step precedent, a powerful feature that requires express permission, enforced by a check-permission step the API normatively depends on (§3, Part 2 §7.6). *(Quotations verified verbatim against the current TR/CR this revision; the 2016 Geolocation REC predates the powerful-feature framing and its wording differs from the CR quoted.)*

- **Resnick, P.**, "On Consensus and Humming in the IETF," RFC 7282, 2014. https://www.rfc-editor.org/rfc/rfc7282 . Grounds publishing the human layer's semantics (rough consensus is objections addressed, not votes counted) in the same document series as the wire formats (§3). *(§3 quotation verified verbatim this revision.)*

- **Popper, K.**, *Conjectures and Refutations*, 1963 (Routledge). Grounds §2.0, §2.2, §3 (corroboration, never verification). *(Verified.)*

- **Peirce, C. S.**, fallibilism and the *community of inquiry* (Collected Papers; e.g. "The Fixation of Belief," 1877, and the "do not block the way of inquiry" maxim). Grounds §3 (community-convergence-over-unlimited-time, humility built into the mechanism). **[confirm]** (exact wording varies by essay; confirm against the primary before quoting).

### Commons governance and the governance-as-protocol frontier

- **Ostrom, E.**, *Governing the Commons*, Cambridge University Press, 1990 (design principles 6, conflict-resolution mechanisms; and 7, minimal recognition of the right to organize). https://doi.org/10.1017/CBO9780511807763 . Grounds §2.3, §2.4, Part 2 §7.6. **[confirm the verbatim wording of principles 6 and 7 against the 1990 primary.]**

- **Wilson, D. S., Ostrom, E. & Cox, M. E.**, "Generalizing the core design principles for the efficacy of groups," *Journal of Economic Behavior & Organization* 90S, 2013, pp. S21–S32. https://doi.org/10.1016/j.jebo.2012.12.010 . Grounds the subsidiarity capstone. **[confirm verbatim; distinct from the 1990 book.]**

- **Schneider, N., De Filippi, P., Frey, S., Tan, J. Z. & Zhang, A. X.**, "Modular Politics: Toward a Governance Layer for Online Communities," *Proc. ACM Human-Computer Interaction* 5 (CSCW1), 2021, article 16. https://doi.org/10.1145/3449090 . The closest governance-as-protocol neighbor (Part 2 Appendix C.4). **[confirm quotations against the CSCW paper.]**

- **Spritely Institute** (Lemmer-Webber, C.), "Technical Values and Design Goals," https://spritely.institute/about/ , and the **W3C ActivityPub** Recommendation (2018), https://www.w3.org/TR/activitypub/ . Ground the contextual-identity posture (§2.3, Part 2 §5.6). The petname tradition (Stiegler, "An Introduction to Petname Systems") is the nearest prior art for human-meaningful naming over keys. *(Spritely page quotations verified verbatim.)*

The component-by-component mechanism prior-art map is in Part 2 Appendix C, and the versioned software/spec pins are in Part 2's upstream-links section. Part 1's references are the principled lineage; nothing here is a claim of priority or supremacy, only of the shoulders this stands on.

---

## 0. Map

`A per-section index with scope, dependencies, and orthogonality, maintained as sections change; kept here at the back and pointed to from the front matter, so the opening page stays prose. Part 1 is the reasoning: §1 positions the system, §2 states the principles the mechanics must satisfy, and §3 shows they are corroborated rather than invented, closed by a coda on what Part 1 does and does not establish and the reference list. Cross-part references name the part (Part 1 §… / Part 2 §…), so this §3 and Part 2's §3 overview are distinct. Each entry states what the section covers and, where it matters, what it depends on and what it is orthogonal to, so a reader can follow the argument's structure without treating independent principles as if one rested on another.`

- **§1. Introduction.** What the document is and where it sits. §1.1 scope (the mechanics are Part 2; the substrate's ownership form is in scope for motivation, out of scope for mechanics); §1.2 where Drystone sits, the complement-to-ATProto positioning and the one move that is genuinely its own. Frames everything below.

- **§2. Design Principles.** The principles, each ending on an obligation the wire must satisfy.

  - **§2.0. The razor: compute provenance, never utility.** The founding razor: the protocol furnishes verifiable provenance and never renders the social verdict, so everything downstream is this razor applied. **§2.0.1** time is an assertion, never a fact, the razor's sharpest edge (no wall-clock is corroborable). *Depends on:* nothing; it is the root.

  - **§2.1. P-Local-Truth.** The only canonical state is local; there is no authoritative global copy. The foundation that makes exit and fork mechanically real. *Orthogonal to:* §2.2 and §2.3 (where state lives is independent of how it is audited or who has standing).

  - **§2.2. P-Knowable-Truth.** Verify for yourself; the record is auditable and never silently mutable. *Orthogonal to:* §2.3 (auditability is independent of standing).

  - **§2.3. P-Peer-Equality.** Equal in rights, therefore equal in weight; unequal in resources and revocable Group Roles. The equality spine, and the source of the voice right (§2.6) and the center analysis (§2.7). *Depends on:* nothing internal. Its Consequence paragraph carries the enforced-by-mechanism obligations Part 2 §5 and Part 2 §7 realize.

  - **§2.4. P-Durable-Enablement.** Participation, and exit, must be real on a bare node. *Depends on:* §2.1 (local-first is what makes exit possible without anyone's permission).

  - **§2.5. The forced terminus: why fork, not verdict.** The provably-non-empty residue, where provenance is settled and utility is open, has no coordination-free resolution, so the terminus is a fork, not a verdict; the fork has a floor (the minimum fork is one) and a limit (a ban is the same primitive, so the harshest power leaves the member whole). *Depends on:* §2.0 and §2.0.1 (the razor's seam, no clock) and §2.3 (equal standing makes the conflicts symmetric). *Realized in* Part 2 §7.6. *Orthogonal to:* §2.6 (conflict resolution versus perception).

  - **§2.6. The voice right requires field-integrity.** Voice is a right only if the field a persona asserts into is not authored by an unremovable adverse party; field-integrity is peer-governed, legible, exitable shaping, which is why the substrate's ownership form is in scope for the motivation. *Depends on:* §2.3 (voice as a right), §2.2 (legible), §2.4 (exitable). *Orthogonal to:* §2.5.

  - **§2.7. Consolidation and the center: capability freely, authority revocably.** Capability is not authority, so capability may consolidate one-directionally without a center, and authority may consolidate because it is revocable and exitable; a center is authority that is irrevocable and inescapable, guarded by the escape hatch and the party-neutral-default corollary. *Depends on:* §2.3 (the capability/authority line), §2.4 (exit), §2.1 (peers hold their edges). *Realized in* Part 2 §5.5, Part 2 §5.7, Part 2 §5.9, Part 2 §7.6.9, Part 2 §7.9.

  - **§2.8. Faithful representation: what the substrate is for.** The capstone: the protocol faithfully represents individual choice and never centrally resolves social conflict, only aggregates it, and the protocol/product division follows (faithful possibility is the protocol, easy representability is the product). *Depends on:* §2.0, §2.3, §2.5, §2.6. *Realized in* Part 2 §7.6, Part 2 §7.6.9.

- **§3. Why these principles are corroborated, not invented.** The cross-disciplinary grounding: each principle mapped to its nearest prior art across distributed systems, ethics, economics, systems science, epistemology, commons governance, and Internet governance (the IANA Designated Expert, W3C powerful features, IETF rough consensus, corroborating the escalate-to-a-human posture of §2.5 and Part 2 §7.6), showing the set is discovered rather than asserted. *Depends on:* all of §2.

- **What Part 1 establishes (and does not).** The coda bounding the claims: what the reasoning secures, and what it defers to Part 2 or to the companion ownership argument.

- **References (Part 1).** The source list, grouped by discipline, with versioned upstream links.

- **Upstream reference links (versioned).** The pinned, versioned URLs for the sources above, kept as a separate back-matter section so the discipline-grouped list stays readable.
