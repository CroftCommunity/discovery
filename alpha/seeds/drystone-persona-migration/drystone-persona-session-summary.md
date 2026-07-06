# Session summary: peer → persona, and the model it forced

status: conclusions (this session)

purpose: a standalone record of the problem, the reasoning, the model
reached, and the open items, so the decision is reconstructable without
replaying the chat.

---

## 1. The problem

"Peer" was doing too many jobs at once. It named:

- a **relationship** (symmetric standing at the edge between two or more
  things), which is its strongest and most defensible sense;

- the **entity** itself (the human's manifestation in a system), which is
  the sense that was overloading the word;

- in §3.1, an **adjudication-locus genus** (anything that holds authority,
  including a group), cross-referenced to §5.2 as if it were the same rung
  as the human-entity, which it is not;

- in §7, a **solo sync-protocol actor** (a node performing a mechanical
  step).

Using one noun for the relation and the thing-in-the-relation loses the
fidelity needed to talk about the parts separately. The fix was to find a
word for the entity and let "peer" retreat to the relation.

---

## 2. Candidates considered

- **persona** — chosen. Etymology (per + sonare, the mask the voice sounds
  through) gives a genuine-channel reading rather than a disguise, and the
  theatrical root supplies "a cast of personae" so symmetric standing is
  latent. Connotation risk ("performed/fake") is defused by a definitional
  reach back to the root.

- **citizen** — rejected: too civic, imports a polity, forces a primary
  belonging that fights the one-to-many.

- **avatar** — rejected: gaming/VR baggage; too technical-feeling.

- **author** — rejected: imports authorship/origination, conflates standing
  with creation.

- **identity** — rejected: overloaded across the stack.

---

## 3. The model reached

Three positions, plus one referent deliberately outside the protocol.

`human` — the referent. No rung. The protocol cannot see or verify it.

`persona` — the human layer's manifestation in the social graph. It is a
principal *by virtue of having a key pair*; that key pair, through lineage
and verification, is its entire manifestation. A persona adjudicates the
social-utility calls the system must not compute (it is a principal, a locus
of adjudication). Persona is the human-representing kind of principal.

`principal` — the genus (existing term). Kinds: persona, group, delegate.
The §3.1 adjudication-locus is the principal.

`peer` — the relation. "Personae are peers." No longer a noun for an entity.

And one thing that is explicitly *not* a rung: the persona-to-human binding.
It has no technical representation; it is the group's contextual judgment.

---

## 4. The load-bearing conclusions

**The two guarantees sit on separate rungs.**

  Mechanical (provenance, certain): one recognized persona = equal rights +
  one flat unit of weight, non-inflatable. The protocol holds this.

  Social (utility, contextual): whether a persona is one distinct human, and
  how strongly to bind before recognizing it, is the group's call.

  The mechanical guarantee makes the social judgment meaningful: the group
  calibrates recognition over a unit the protocol has already made clean.

**The protocol cannot promise or impose the binding (not merely declines).**

  It could not promise one-persona-one-human: there is no fact to deliver
  (the binding has no technical representation). It could not impose it as a
  duty: there is no authority tier above the group to enforce from, and
  trying would relocate adjudication and reintroduce a center. Both are the
  same §2.0 limit, seen from the delivery side and the enforcement side. So
  the judgment does not get handed to the group; it necessarily falls there.

**Gating is not enforcement.**

  A group gating its own entry to its own standard is legitimate and often
  desirable; it is the group's recognition dial operating at the door, not
  the protocol enforcing a binding from above. The impossibility claim is
  narrow: it is about the protocol guaranteeing/policing the binding-as-fact,
  not about a group admitting whom it chooses.

**Multiplicity is across systems; within a group it is one, unenforceably.**

  Many personae per human is the across-discrete-systems case, by design
  (one per group: volleyball team, school district, work graph, each fully
  valid, none primary). Within a single group the intent is one persona per
  human. No technical control enforces that within-group; the consequence of
  violating it is degraded governance (weight that should be one unit counts
  as two, so per-persona equality stops corresponding to per-human equality).

**Recognition is proportional; weight is not.**

  The group defines its own tolerance and standard, aligned with its function
  and goals: a financial scope requires a stronger binding than a casual
  messaging scope, which requires more than a public event invite. This
  proportionality is on *recognition strength* (how strong the evidence must
  be before a persona is counted as a distinct human), never on weight. Once
  recognized, weight is flat, one per persona, regardless of binding
  strength. (Weight-proportionality would reintroduce weighted voting, which
  P-Peer-Equality forbids.)

**"Recognize" means "decide to treat as," never "verify."**

  Even after recognition the binding is not a fact anywhere; it stays a
  standing judgment the group can revise, and the revision path is
  withdraw-recognition-and-fork.

---

## 5. §3.1 resolution

§3.1 overloaded "peer" as both the adjudication-locus and (via its §5.2
cross-ref) the human-representation. Resolution 1 was chosen: the locus
becomes **principal**; "system of peers" is kept and reread as the relation,
with a new sentence defining the peer-relation where the heading invokes it.
Resolution 2 (rename the heading to "of principals") remains available if the
kept-relation heading reads as residual overload in practice.

---

## 6. What was produced

- `drystone-part1.md`, `drystone-part2.md` — edited specs, full migration
  applied.

- `drystone-part1.diff`, `drystone-part2.diff` — unified diffs of every
  changed line.

- `drystone-persona-delta.md` — the change record (what moved to persona vs
  principal vs participant vs kept, and why).

- `drystone-persona-migration.md` — the plan, model, classification rules,
  and the seven argued rewrites with before/after.

- `peer-inventory-worksheet.txt` — the line-by-line worksheet.

- `persona-definition.md` — reconciled normative definition + the term
  lattice/invariants oracle (Note 4). Mirrored into Part 2 Appendix D.

---

## 7. Open items carried forward

- **persona-definition.md is partly superseded.** It predates two
  corrections: that persona *is* the human layer (no separate human rung
  above it), and that multiplicity is across-systems-not-within-group. It
  should be reconciled to the final model, with "recognize" used as
  "decide to treat as," never "verify."

- **PrincipalSet naming.** Confirm a PrincipalSet can in fact hold a
  group/delegate, not only personae. If it only ever holds personae in
  practice, revisit the name.

- **"sensor"** (§3.1 contrast term): flagged for review, not changed.

- **§3.1 heading**: Resolution 1 applied; Resolution 2 available.

- **Two-leg argument in §5.6**: the "declining to solve personhood is
  faithfulness" passage currently argues mostly from variety. The could-not
  framing adds a second leg (impossibility). Consider stating both, since
  they answer different objections (should-you vs could-you).

---

## 8. Caveats on this record

- The "degraded governance" consequence is the model's internal logic
  (stated by the author, formalized here), not an external citation.

- Not every one of the ~235 inventory lines was read in deep context; the
  argued sections and a representative sample were. The diffs are the
  ground truth for what actually changed.

- Nothing in the underlying source-discipline of the specs (the
  [confirm before publish] flags, the verbatim-quote status of Spritely,
  Ostrom, Lamport, etc.) was touched or re-verified by this migration; those
  flags stand exactly as they were.
