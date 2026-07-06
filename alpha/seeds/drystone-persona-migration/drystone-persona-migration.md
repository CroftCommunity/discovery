# Drystone: the peer → persona migration

status: applied this session. The migration described here has been carried
out; see `drystone-part1.md` / `drystone-part2.md` (edited specs) and the
`.diff` files. This document is the plan and rationale; for the authoritative
final model read `drystone-persona-session-summary.md`, and for the change
record read `drystone-persona-delta.md`. Reconciled to the final model
(persona is the human layer; no separate human-escalation rung).

scope: how to separate the entity sense of "peer" (→ persona) from the relation sense (peer, kept) across Part 1 and Part 2

decision recorded: Option A (persona replaces the system-local entity noun; peer retreats to the relation)

§3.1 resolution recorded: Resolution 1 (locus = principal; "system of peers" reread as the relation; heading punch kept)

---

## 1. The model this migration installs

Three positions plus one referent that is deliberately outside the protocol.

`human` (referent, not a rung)

  The person behind a persona. The protocol cannot see it, cannot verify
  it, and does not give it a rung. It is the thing a persona is *taken to
  manifest*, never a thing the system holds. There is no separate
  "human layer" rung above persona: persona *is* the human layer's
  manifestation, and the human itself stays outside the protocol as the
  unreachable referent.

`persona` (the human layer's manifestation in the graph)

  A persona is a principal by virtue of having a key pair. That key pair,
  carried through lineage and verification, is its entire manifestation
  within the social graph. A persona is a locus of adjudication (it is a
  principal, §3.1): it is where the social-utility calls the system must
  not compute deterministically (§2.0) are adjudicated, because a human
  stands behind it. Persona is the human-manifesting *kind* of principal.

`principal` (the genus; unchanged term)

  A role-holding entity identified by one key-lineage; a locus that can
  adjudicate (§3.1). Kinds: persona (the human kind), group, delegate.
  Persona is-a principal.

`peer` (the relation; the word's only retained sense)

  A symmetric relation between principals: each is a locus the others must
  respect, none a center the others must obey. "Personae are peers" means
  personae stand in this relation. Peer is no longer a noun for an entity.

The binding between a persona and the human it manifests is **not a rung**.
It has no technical representation; it is the group's contextual judgment
(see §3).

---

## 2. The two guarantees, on separate rungs

The single most important outcome of this migration is that one mechanical
guarantee and one social judgment, currently both hung on the word "peer,"
end up on visibly different rungs.

**Mechanical guarantee (provenance, certain, the protocol's job).**

  One recognized persona carries equal rights and exactly one flat unit of
  governance weight, by lineage, non-inflatable by clients, devices,
  resources, or roles. The protocol holds this airtight.

**Social judgment (utility, contextual, the group's call).**

  Whether a persona corresponds to one distinct human, and how strong a
  binding the group requires before recognizing it as such, is the group's
  determination. The protocol does not compute it (§2.0), and the stronger
  point is that it *could not*: the binding has no technical representation
  the protocol could read (§5.2 keystone), so there is no fact for the
  protocol to compute. The judgment does not merely fall outside the
  protocol's job; it falls outside the protocol's reach.

**The relationship between them, stated so it cannot be misread.**

  The mechanical guarantee is what makes the social judgment *meaningful*:
  because the protocol can hold "one recognized persona = equal rights +
  one weight" as clean and countable, the group's judgment about how many
  humans those personae represent is a judgment over a well-formed unit. If
  weight were not mechanically flat-per-persona, the personhood judgment
  would be deciding nothing crisp.

---

## 3. Multiplicity, recognition, and the consequence (the corrected claims)

These four statements are the corrected core. Earlier framings blurred
them; the migration must keep them distinct.

**Multiplicity is across systems, not within a group.**

  A human holding several personae is the across-discrete-systems case (a
  persona for the volleyball team, one for the school district, one for the
  work graph), different keys, each its own persona, all equally valid. The
  intent within any single group is one human, one persona.

**Within a group, the binding is unenforceable by mechanism, by design.**

  No technical control can stop a human presenting as two personae inside
  one group. Drystone does not pretend to provide one, because that binding
  is exactly the utility call §2.0 says the system cannot certify. The
  protocol defines the intent and the consequence; it does not supply a
  control.

**The consequence is degraded governance, stated specifically.**

  Two personae for one human in a group is weight that should be one unit
  counted as two. The harm is internal and precise: the per-persona
  equality the protocol guarantees stops corresponding to per-human
  equality, and every governance count downstream is silently skewed. The
  intent is all personae weighted equally, and therefore all humans
  weighted equally; multi-presentation within a group is the decay of that
  correspondence, not an external break-in.

**Recognition is proportional; weight is not.**

  The group defines its own tolerance and standards for binding a persona
  to a distinct human, aligned with the group's function and goals. A group
  with access to financials will set a tighter standard than a volleyball
  messaging group, which sets a tighter one than a public-but-registered
  event invite. This proportionality lives entirely on the *recognition*
  side: how strong a binding the group requires before it counts a persona
  as a distinct human. It never touches weight. Once recognized, weight is
  flat, one, regardless of binding strength. Stronger binding requirement,
  never a heavier vote. (Collapsing proportionality onto weight would
  reintroduce weighted voting, which P-Peer-Equality forbids.)

  Note this is not the protocol delegating a duty downward. The protocol
  **cannot promise** one-persona-one-human (there is no fact for it to
  deliver) and **cannot impose** it as a duty (there is no authority tier
  above the group from which to enforce or adjudicate compliance, and
  trying would relocate adjudication and reintroduce a center, §3.1, §8).
  Both impossibilities are the same §2.0 limit seen from two sides: the
  delivery side (nothing to promise) and the enforcement side (nowhere to
  impose from). So the judgment does not get *handed* to the group; it
  **necessarily falls** to the group, because the protocol can reach the
  binding from neither direction. The group defines its own standard
  because that is the only place the standard can live.

---

## 4. Classification rules for every "peer" occurrence

Apply in order; first match wins. The worksheet in §7 tags every line.

`KEEP-RELATION`

  "peer" used as the symmetric relation or predicate. Examples from the
  text: "other peers must respect," "across peers," "honest peers agree,"
  "two peers' views diverge," "peers are equal." These stay, and are the
  word's correct home. Where a sentence reads more clearly as "personae"
  (entities standing in the relation) vs "peer relation" (the relation
  itself), prefer naming the entities personae and the relation peer.

`KEEP-TRANSPORT`

  "peer-to-peer," "NAT'd peer," "two peers reach each other," "peers stuck
  at different heads," reachability/wire usage. Reserved for transport by
  the Part 1 §1 note. Do not touch. In pure transport context a "peer" is a
  network endpoint, not a governance entity, and that is fine; the planes
  are explicitly separated.

`KEEP-DEFINED-TERM` (decision required, see §6)

  `P-Peer-Equality`, `PeerSet`, `peerhood`, `Peer Standing`,
  `peer-governed`. Renaming these is a larger decision than the prose
  migration; §6 records the recommendation per term.

`→ PERSONA` (the entity sense)

  "peer" used as the noun for the human-representing entity: "a peer is the
  representation of a human," "a peer's rights floor," "one per distinct
  peer," "weight attaches to the peer," "a peer accepting a write." These
  become persona.

`→ PRINCIPAL` (the genus/locus sense)

  "peer" used as the adjudication locus where the point is genus-level, not
  specifically the human kind: §3.1 "a peer is a locus that can
  adjudicate," "recursive peer-is-a-group," "no authority tier above the
  peer." Where the locus could be a group as well as a human, principal is
  the right word, not persona.

The hard cases are the §3.1/§5.2 pair (locus vs human kind) and the
recursion passages (peer-is-a-group). The rule: if a group can be the thing
referred to, it is `→ PRINCIPAL`; if it is specifically the human-behind-it
that rights and weight attach to *because a person is there*, it is
`→ PERSONA`.

---

## 5. Load-bearing rewrites (surgery, not find-replace)

These passages cannot be migrated by substitution; the sentences carry
argument and have to be re-cut. Before/after for each.

### 5.1 §3.1 opening (Part 2, lines 64-67) — Resolution 1

Before:

> A **peer is a locus that can *adjudicate***; it holds genuine authority
> over some domain that other peers must respect, not merely a node that
> can sense, store, and relay. A node that only senses and relays, with its
> decisions made elsewhere, is a **sensor**, however well-connected it is.

After:

> A **principal is a locus that can *adjudicate***: it holds genuine
> authority over some domain that other principals must respect, not merely
> a node that can sense, store, and relay. A node that only senses and
> relays, with its decisions made elsewhere, is a **sensor**
> `[term flagged for review]`, however well-connected it is. To say a system
> is one *of peers* is to say its principals stand in **peer relation**:
> each is a locus the others must respect, none a center the others must
> obey.

The final sentence is load-bearing for Resolution 1: it is what defines the
peer-relation at the spot the heading invokes it. Do not drop it.

### 5.2 §3.1 ordering sentence (Part 2, lines 85-86)

Before:

> define the peer (a locus of adjudication, §5.2), then peer rights (§5.3)

After:

> define the **principal** (a locus of adjudication, §5.2) and the
> **persona** as its human-representing kind, then the rights floor (§5.3)

### 5.3 §5.0 "what a peer is" (Part 2, lines 262-267)

This is the crux paragraph. Before:

> What makes the equality *matter* is what a peer **is**: a peer is the
> **representation of a human** in the system... it does **not** certify
> that one peer is one person. That is the razor (§2.0) applied to the peer
> concept itself.

After (re-cut to put the two rungs on visibly separate sentences):

> What makes the equality *matter* is what a **persona** is: a persona is
> the **human layer's manifestation in the system**, a principal by virtue
> of its key pair, present through lineage and verification (§4.5), not a
> node or a device. Peer-equality is **equality of personae as represented**,
> equality in expression and in count. The protocol guarantees, by
> mechanism, that one recognized persona carries equal rights and one flat
> unit of weight. Whether a persona corresponds to one distinct human is a
> separate question the protocol does **not** answer: that binding is a
> social-utility judgment the group makes at its own standard (§2.0, §5.6).
> The mechanical guarantee is what makes that judgment meaningful; it does
> not substitute for it.

### 5.4 §5.2 kind-of-principal entry (Part 2, lines 355-358)

Before:

> - a **peer**, the principal that **represents a human** in the system,
>   carrying the rights floor...

After:

> - a **persona**, the principal that **manifests a human** in the system,
>   a principal by virtue of its key pair and the lineage descending from
>   it (§4.5), carrying the rights floor and one unit of weight, and the
>   locus at which the social-utility calls the system cannot compute (§2.0)
>   are adjudicated, because a person stands behind it. The common case is
>   one human, one persona, possibly many devices.

### 5.5 §5.2 keystone box (Part 2, lines 389-410)

The box currently reads "a lineage is a provenance object; a peer is the
human it represents." Re-cut:

> A **lineage** is technically representable: a cryptographic-provenance
> chain the protocol verifies and counts with certainty. A **persona** is
> the human that lineage is taken to manifest. Whether a given lineage
> corresponds to a distinct person has **no technical representation at
> all**; it was never a fact the system holds. The protocol counts
> lineages; the group decides which lineages it recognizes as distinct
> persons, to its own standard, and that recognition is what turns a counted
> lineage into a weighted persona.

Keep the rest of the box's argument (category error, why the separation is
load-bearing); replace "peer"→"persona" for the entity, and read the
"why peer and personhood are separate words" line as now "why persona and
personhood are separate words," which is even cleaner since the relation
sense is gone from the sentence entirely.

### 5.6 §5.6 weight + the proportionality addition (Part 2, lines 601-668)

Two changes. First, entity "peer"→"persona" throughout (weight attaches to
the persona, one per distinct persona, conserved under delegation). Second,
the §3 proportionality claim, which §5.6 currently states only descriptively
(the high/medium/low gradient), gains its normative form:

> The strength of binding a group requires before recognizing a persona as
> a distinct human is **proportional to the group's function and goals**: a
> scope with access to financials sets a tighter standard than a casual
> messaging scope, which sets a tighter one than a public event invite. This
> proportionality is on **recognition**, never on weight: a stronger binding
> requirement, not a heavier vote. Once recognized, weight is flat, one per
> persona, regardless of how strong the binding was. The group defines its
> own tolerance and standard; the protocol neither sets it nor enforces it.

Also re-cut the "one human may hold many peer-lineages, accepted property of
that context" line (line 655): under the corrected model this is the
across-systems case by default, and within a group it is the
unenforceable-by-design case whose consequence is degraded governance. State
both rather than the single flat "accepted property."

### 5.7 Appendix B grounding note (Part 2, lines 1959-1971)

"define a peer as a locus of adjudication" → "define a principal as a locus
of adjudication; a persona is its human kind." And "what binds a human to a
peer" → "what binds a human to a persona." The mint-and-bind discussion is
about persona binding specifically, so persona is correct there.

---

## 6. Defined-term decisions (KEEP-DEFINED-TERM)

These are not prose; each is a named token. Recommendation per term, for
your decision, not applied yet.

`P-Peer-Equality`

  Recommend KEEP the principle's name. It is a stable identifier referenced
  dozens of times and across both parts; renaming to `P-Persona-Equality`
  is a large mechanical change for modest gain, and "peer-equality" reads
  naturally as "equality-in-the-peer-relation," which is now *exactly*
  correct under the relation reading. Add one line at its definition: the
  equality is among personae standing in peer relation.

`PeerSet`

  Recommend rename to `PersonaSet` *if* the set is a set of the entities
  (it is: §5.5 pins roles to a set of the human-representing principals).
  This one is entity-sense, so it should migrate for consistency. 13 hits
  in Part 2; mechanical. Flagged as the one defined term that genuinely
  should move.

`peerhood`

  Entity-sense ("what it is to be a peer-entity") → `personahood`. 1 hit
  Part 1, 1 compound Part 2. Low cost.

`Peer Standing`, `peer-governed`

  `peer-governed` is relation-sense (governed by the peers-in-relation),
  KEEP. "Peer Standing" (Part 1) is entity-sense, → "persona standing."

The split to hold: principle *names* and relation-compounds keep "peer";
set/standing/hood tokens that denote the entity migrate to persona.

---

## 7. Full line inventory (worksheet)

Every bare-"peer" line, by file and line number, for the line-by-line pass.
Tag column to be filled as each is cut: R (keep-relation), T
(keep-transport), P (→persona), N (→principal), D (defined-term, see §6).
Lines already covered by a §5 rewrite are marked [§5.x].

The safe buckets, already classified mechanically and needing no
line-by-line judgment:

- KEEP-TRANSPORT: Part 1 lines 35, 82 (compound), Part 2 lines 109, 978,
  984, 1003, 1671, 1682, 1733, and all `peer-to-peer` occurrences.

- KEEP-DEFINED-TERM: all `P-Peer-Equality` (7 in Part 1, 16 in Part 2),
  all `PeerSet` (1 + 13), `peerhood`, `peer-governed`, `Peer Standing`,
  `peer-equality`. Decisions in §6.

The judgment bucket (bare entity/relation/locus "peer"), 51 lines in Part 1
and 184 in Part 2, is extracted in full in the companion file
`peer-inventory-worksheet.txt` (line number + text + blank tag column),
because at ~235 lines it is a checklist to work through rather than prose to
read. The §5 rewrites above cover the load-bearing subset; the worksheet is
the remainder, which are mostly straightforward `→ PERSONA` (entity) or
`R` (relation) once the rule in §4 is applied.

---

## 8. Order of operations (suggested)

1. Lock §3.1 (Resolution 1) and the §5.0/§5.2 crux paragraphs first (§5.1-5.5
   above). These fix the vocabulary the rest of the spec runs on; doing them
   first means the worksheet pass has a settled target.

2. Do the `PersonaSet` token rename (§6) as one mechanical pass.

3. Walk the worksheet, tagging and cutting each remaining line by the §4
   rule. Most are single-word substitutions once the crux is set.

4. Re-read every `KEEP-RELATION` line in context to confirm it now reads as
   "personae standing in peer relation," not as a leftover entity-noun.

5. Final consistency check: search the result for any bare "a peer" /
   "the peer" / "each peer" (entity articles) that survived; under the
   finished model those should be near-zero outside quoted legacy labels.
