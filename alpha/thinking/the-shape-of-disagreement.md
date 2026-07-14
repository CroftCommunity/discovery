# The shape of disagreement — forks, projection, and the narrator problem

`A captured design dialogue (2026-07-14). It began with a concrete bug found in RUN-01 EXP-4 — two
competing rule-change quorums silently disagreeing — and walked outward into what that bug is really a
symptom of: that this protocol is trying not to answer the question every consensus system answers.
This note keeps the original narrative voice; the blockquotes are the operator's own words. It is a
design exploration, not a decided spec — the concrete options land in EXPERIMENT-BACKLOG §2a and the
concurrent-contradiction design note. Nothing here is committed to Part 2.`

---

## 0. Where this came from

RUN-01 EXP-4 found a real hole. Two separate groups of admins, at the same moment, each *properly*
vote through a conflicting rule change — one says "set the limit to 5," the other "set it to 9." Both
votes are legitimate. The system is supposed to freeze and surface the conflict for a human. Instead it
**silently picks a winner by pure luck** — whichever message was processed last — and two devices can
end up with different rules and no warning.

The first instinct is: surely there's always a first and a second? Cryptographic ordering, first is
first. But across devices there is *no* built-in first and second. Two facts are only ordered if one
*points at* the other (an informed, sequential edit). Two facts written blind to each other are
**concurrent** — there is no first. The bug is that the system linearizes them by local arrival order,
which erases the fork and lets the last-arriving one win. Different arrival orders → different winners →
silent disagreement.

That's the bug. But the operator's response opened the real question:

> Right now, deterministic systems always wanna answer who won. That's the question. Yes. This is part
> of the thing that I've been trying to convey.

The fix isn't a better tiebreak. It's to **stop asking who won.**

---

## 1. Agree on the shape of the disagreement, not its resolution

The canonical hard case (the spec calls it the §2.5 irreducible residue) — A bans B while B bans A at
equal standing — has *no* coordination-free answer. There is no fact about who should remain. Every
deterministic system tries to manufacture one anyway. This protocol's move is to refuse.

Split the problem into two layers:

- **Layer 1 — objective, shared, cryptographic: *the fork exists, and here is its exact shape.*** Every
  device agrees *that* they disagree, *what shape* the disagreement has, and *who is on each branch* —
  and nothing more. This is a far weaker, far more achievable form of consensus than "who won": you get
  everyone to agree on the *shape of the disagreement*.

- **Layer 2 — subjective, local, per-viewer: *which branch(es) I follow.*** This is not in the log. It
  is a **lens** each member applies. The fork is truth; the resolution is a **projection**.

Once you make that split, the impossibility result stops being a problem to solve. You don't resolve
the fork at the consensus layer. You keep it as truth and resolve it locally, per viewer.

---

## 2. Why *all* conflicts fork

Asked "which conflicts should freeze?", the answer turned out to be: all of them.

> It's a conflict just when you ban someone, that's a conflict. Right? The conflict is they can't move
> past that point in the chain. That's a conflict with the person being banned. That's why bans are
> forced forks. So I guess all of them.

A removal *is* a conflict with the removed party by nature. Every concurrent contradiction is a genuine
fork. So Layer 1's job is not to decide *which* forks are real — it's to be **total and deterministic**:
detect and freeze *every* concurrent conflict, for *every* act type, and name it identically on every
device. That is the un-gated engineering work, and it is exactly what the EXP-4 bug violates: the fork
detector is not yet total (rule-changes slip through). Fixing that is not a design decision — generalize
the concurrency-plus-conflict predicate the mutual-expulsion path already uses. The *design* question
moves up a layer, to the resolution language.

---

## 3. Layer 1 — the shape, as a comparable object

A fork is not an event that needs a winner; it is a **descriptor** every device computes byte-identically:

```
ForkDescriptor {
  fork_id:  H_min(headA, headB)          // deterministic name — the EXP-4 byte-head, already built
  frontier: { shared antecedent hashes } // the exact point they diverge from
  kind:     CompetingRuleChange | MutualExpulsion | RemovedThenIncluded | RoleThrash | …
  about:    { member_limit }             // the subject(s) — what the fork is OVER (matchable)
  branches: [
    { head: hA, delta: rule(member_limit): 1→5, retains:{…}, removes:{} },
    { head: hB, delta: rule(member_limit): 1→9, retains:{…}, removes:{} },
  ]
}
```

It is objective, content-addressed, identical everywhere, and says **nothing about who wins.** It is
*comparable* precisely because `kind` / `about` / `branches` are structured — a policy can pattern-match
on it. This is the shared substrate the projections ride on.

---

## 4. Layer 2 — resolution as a Unix-style ACL, not a value judgment

The operator's instinct about the resolution language: it is closer to file-rights than to morality.

> This is probably much closer to how rights are encoded on a file in Unix than it is to any kind of
> value judgment — like, always follow, sometimes follow. You're setting your own threshold for
> adjudication, and you could be as conservative or not as you want.

So a policy is an **ordered, first-match-wins list of `match → verb`**, evaluated locally over a
ForkDescriptor. Compact, composable, and *shareable so it can be compared*:

```
resolve(fork):
  [ kind=MutualExpulsion  AND  branch.removes(wife) ]  → follow(wife)      # I go with her
  [ about ∈ my.tier1_set ]                             → multi_home        # keep everyone close, in both worlds
  [ kind=CompetingRuleChange ]                         → follow_default    # I don't care — use the group norm
  [ * ]                                                → follow_canonical  # else: smaller-hash, so I land with everyone who also just wants to converge
```

The **verb set is closed** — that is what keeps it a legible, shareable *norm-vocabulary* and not
arbitrary code:

- `follow(P)` — take the branch that keeps P
- `multi_home` — subscribe to *both* branches; the fork **multiplies** your world (you end up in both groups)
- `follow_default` — defer to the group's published norm policy
- `follow_canonical` — deterministic tiebreak; the *only* verb that guarantees you agree with everyone else who chose it
- `follow_larger` / `drop` / `escalate` — majority-ish / leave / kick it to the human

### The flattening this fixes

> A friend on Facebook has no actual meaning. You're friends with your sister and you're friends with
> someone you knew fifteen years ago you get a Christmas card once a year from, and they're the same
> layer.

The extractive social graph flattens every tie into one undifferentiated edge — *legible to the data
system, meaningless to you.* Here it inverts:

> You have different relationships with people. What it looks like is all up to you. And even what they
> know about it is all up to you.

Your policy encodes the *texture* of your relationships (follow-my-wife, multi-home my inner tier,
don't-care about the rest) — and that texture is **yours**, not the platform's. There is a hard privacy
floor implied by "what they know is up to you":

> One node can never indicate to another [what its policy is] because it would be a security risk. You
> could profile people based on their [policies] and try to determine how they might react. You'd always
> make a guess, but the surety is very dangerous there — though no more or less dangerous than any human.

So the resolution policy is **local and private by default** — a node's follow-rules are not
advertised, because advertised adjudication rules are a profiling surface. Guessing is human; *certainty*
about how someone will resolve is the danger, and the protocol should not hand it out.

### Normative, not literal

The default/norm layer encodes relationship *roles* in a normative rather than literal way:

> The follow-default merges and encodes a spouse status and a friend status, but in a normative way — so
> it'd be, rather than "parents," it's "guardian," kinda thing.

The vocabulary abstracts "the person I follow into any fork that would separate us" into a role
(guardian, tier-1, spouse-equivalent) rather than pinning it to a literal kinship term. The norm is a
*shape of trust*, not a census category.

---

## 5. Floors — the same dial, turned to its hard extreme

The correction that unlocks the clean version: the projection model does **not** mean everything is
fluid. Rigidity is not a different system — it is this dial pinned to maximum.

Some conflicts do not fork at all. They resolve to **one world for everyone**, and *no local policy
verb can touch them.* A floor is a Layer-1 rule marked **non-projectable**:

```
FLOOR  all_but_self_revocation:  (n−1 of n vote to revoke role R of P)  → R revoked. Period.
FLOOR  key_admission:            group.rules require 100% consensus     → single agreed member set
```

The operator's baked-in example:

> If everyone in a group except you votes to revoke your powers — your delegated group role — that's it.
> That is a built-in, can't-be-broken quorum. A floor, non-dial quorum. The majority of that is just
> explicit handling of an obvious case, but that gives us room to build on.

And the master move: **which acts are floor-resolved versus fork-and-project is itself a group governance
dial**, set where the group's rules are set —

> The rules of a group have to be encoded with the group. And part of that would be whatever language
> you need to handle those points of contention — and that's part of what governance thresholds are.

So:

- A high-stakes group turns the dial toward **floors** — most acts hard-consensus, keys never projected,
  roster global and crisp. You can even demand 100% cryptographic consensus.
- A loose social group turns it toward **projection** — most acts fork, membership emergent, history a lens.

Same machinery. This is why "keys force a real decision that projection can't dodge" is *not* a problem:
**key-admission is simply configured as a floor.** The projection model doesn't threaten hard boundaries
— it *contains* them as its extreme setting.

---

## 6. Coherence as a temperature

> It's not one dial. It's a set of dials that all add up to a repeatable, deterministic statistical
> pattern.

Convergence becomes **emergent and statistical instead of enforced and binary.** For any given fork, the
fraction of the group that lands together equals the fraction whose policies map that ForkDescriptor to
the same verb. A widely-adopted **norm policy** is the *cooling force* — everyone who says
`follow_default` moves as one; personal override rules are the *heat*. Turn the population's dials toward
the norm and you get tight convergence; toward personal rules, deliberate and predictable fragmentation.

The group has a **convergence temperature**, and the norm sets it. Deterministic per person; statistical
in aggregate. That is the "repeatable deterministic pattern."

---

## 7. Identity — and history — live in the lineage

The band metaphor is load-bearing:

> What's that band, the Doobie Brothers — there are no original members, but they still play as the
> Doobie Brothers. That's the shape of human relationships. You can have a core friendship group that
> breaks apart, but your history with those people still remains — your history in that context.

Identity lives in the lineage/genesis, not the roster. Every fork **re-plants** a new group-principal off
the shared, immutable prefix (a fork is the same operation as a heal or a re-key, at a different arity —
the corpus already calls it re-plant). The trunk is permanent cryptographic fact no later fork can
rewrite; the forks are new branches off it. So "the band" persists through total membership turnover, and
your history with those people stays *true* because the shared prefix cannot be edited.

But the deeper turn is that **history itself is also a projection** — not just membership:

> Identity lives in the lineage genesis — but so does history, from your point of view. Each one of those
> dudes in the Doobie Brothers has a different memory of their lives, and they are equally *all* the
> memories of the Doobie Brothers. Some of them maybe never met. That's a representation of the band as
> an object.

So "the conversation" is not one object. It is a **view** over the shared cryptographic DAG, shaped by
your follow-rules. Two people "in the same group" can hold different histories because they resolved a
past fork differently — and both are legitimate.

> It's a subjective overlay over a global, universal, collective truth. But really, all that matters is
> what you can render of it.

All the *facts* are shared truth; the *narrative* is a lens. The rendered projection is the lived
experience.

---

## 8. Why it hasn't been built — and can't be captured

> I'm not trying to monetize this. How could you monetize giving away all authority, basically? You
> can't. But you could operate the machinery to run those systems.

There is no privileged global narrator here — no server, no owner, no canonical history — only a shared
field of facts and each persona rooted at the center of its own graph, resolving that field through its
own private policy. That is the thing an extractive platform cannot do and therefore has never built: it
requires *giving away the authority* that platforms exist to hold.

> The technological part and the socio-outcome part are now decoupled firmly.

You can operate the substrate (the machinery) without owning a single person's judgment (the outcome).
And the reason people can bear this much sovereignty and fluidity at all:

> Encryption means it's not scary as fuck to do this for people.

The privacy floor is what makes radical projection *safe* rather than exposing.

---

## 9. The bet

> This is one of those proofs-in-the-pudding things. The protocol and a reference spec and a
> visualization of it kinda has to travel together. In my mind it makes perfect sense. Getting the
> representation right isn't guaranteed — but I think the squeeze is worth the juice. It's a problem
> worth solving, and critically a solvable problem.

The wager: human relationships work fundamentally differently from consensus protocols — consensus is
*limiting* — and the only way around it is to **codify the relational experience** so that each person's
world can be legible, mostly-shared, and their own. The fork is not the failure mode. Collapsing it to a
single lucky winner is.

> Your projection and mine, which mostly — but not always — agree. And maybe not even mostly. Maybe just
> sometimes agree. But the point is: we are all the narrator, equally, of our own story.

---

## 10. What this means for the protocol (concrete, and how it lands on the EXP-4 finding)

The competing-quorum bug is not a corner case to defer — it is *the* case, because the fork-handling
contract lives in the rules, and concurrent edits to the rules are concurrent edits to *how you handle
everything else*. Making rule-change forks sound is making the contract-negotiation layer sound.

> **Note (RUN-02 F8).** The parallel RUN-02 already *decided* the specific competing-quorum case at the
> spec level — two quorum-met RuleChanges to one rule are a §7.6-class genuine contradiction that
> **must hard-stop, human-adjudicated, never content-address-tiebroken** (§7.3.2 / §7.6.1). That is
> precisely this note's Layer-1 principle ("all conflicts freeze") applied to one act type. So step 1
> below is no longer a design question for RuleChange — it is decided, and un-gated *implementation*.
> What stays a design frontier is the *broader* picture: which acts fork vs. floor (the meta-dial), and
> the whole of Layer 2 (the resolution ACL). F8 decided the *freeze*; it did not decide the *projection*.

The work, in order:

1. **Make Layer 1 total (un-gated).** Generalize the concurrency-plus-conflict detector so *every* act
   type that can concurrently contradict — including competing RuleChange — freezes and produces a
   deterministic ForkDescriptor. This closes the EXP-4 refutation (register `competing-quorum-autoresolve`).
   Mechanically it mirrors the existing mutual-expulsion predicate.
2. **Add the ForkDescriptor** as the shared, content-addressed object (§3) — extend the byte-head naming
   already built to carry `kind` / `about` / `branches`.
3. **Design the resolution ACL (Layer 2, design-gated).** The closed verb set + matcher language (§4),
   evaluated deterministically and kept **private per node** (the profiling floor). Start with the
   verbs listed; the DSL must stay constrained enough to publish norms over.
4. **Encode floors as non-projectable Layer-1 rules (§5)**, and expose the *meta-dial* — which acts are
   floor-resolved vs. fork-and-project — as group governance, set where the rules are set.
5. **Make history a projection in the UX** — the render layer resolves each viewer's DAG through their
   policy. This is where "spec + visualization travel together"; the projection has to be *seen* to be
   believed.

This supersedes and enriches the earlier design options in `EXPERIMENT-BACKLOG.md §2a` (the
competing-quorum fix) and extends `croft-chat/plans/2026-07-12-1-design-concurrent-contradiction.md`.
Steps 1–2 are buildable now; steps 3–5 are the design frontier and remain a human call.

---

## Talking points (the tight version)

- **Every consensus system asks "who won?" This one refuses to.** For a genuine concurrent conflict there
  is no fact about who won — so we don't manufacture one.
- **Agree on the *shape* of the disagreement, not its resolution.** Layer 1: every device agrees a fork
  exists and what shape it has (objective, cryptographic). Layer 2: which branch you follow is a private,
  local *projection* — a lens, not a log entry.
- **All conflicts fork; the detector just has to be total.** A ban is a conflict by nature. The machine's
  only job is to refuse to paper it over, so a human can judge it.
- **Resolution is a Unix-style ACL, not a morality engine** — an ordered `match → verb` list with a
  closed verb set (follow-a-person, multi-home into both worlds, defer-to-norm, converge-canonically).
  Private by default, because advertised adjudication rules are a profiling weapon.
- **Rigidity is the same dial at its extreme.** Hard boundaries (key-admission, the "everyone-but-you
  revokes → done" floor) are non-projectable Layer-1 rules. A group dials where the hard/fluid boundary
  sits, in its own encoded governance.
- **Coherence is a temperature, not a guarantee.** Deterministic per person, statistical in aggregate;
  the shared norm is the cooling force.
- **Identity and history live in the lineage, not the roster.** The Doobie Brothers persist through total
  turnover. Every member holds a different, equally-valid memory of the same band. History is a subjective
  overlay over one collective cryptographic truth — and all that matters is what you can render.
- **It can't be monetized because it gives authority away** — you can only operate the machinery, never
  own the judgment. Tech plane and social plane, firmly decoupled. Encryption is what makes that safe.
- **We are all, equally, the narrator of our own story.** The fork isn't the failure. Collapsing it to a
  lucky winner is.
