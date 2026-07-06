# Drystone peer → persona: delta document

status: applied (this session)

inputs: drystone-part1.md, drystone-part2.md (as uploaded)

outputs: edited drystone-part1.md, drystone-part2.md, plus unified diffs
(drystone-part1.diff, drystone-part2.diff)

scope of change: Part 1 ~48 lines across 18 hunks; Part 2 ~182 lines across
44 hunks. Net additions reflect the argued rewrites (§3.1, §5.0, §5.2, §5.6)
that grew, not just substituted.

---

## 1. What changed, by sense

The word "peer" carried four senses. The migration kept one, retired one to
a new word, and reassigned two. The verification rule for each:

`peer` (relation) — KEPT

  The symmetric relation and predicate: "personae are peers," "every honest
  peer agrees," "two peers reconcile," "of peers," "peer-governed." Kept
  because the relation is the word's correct and only retained home. Per the
  session decision, relation-flavoured uses survive even when grammatically
  nominal, because they trade on how peers relate (including across layers).

`peer` (transport wiring) — KEPT

  "peer-to-peer," "NAT'd peer," "two peers reach each other," "identify
  peers by public key." Reserved for transport by the Part 1 §1 note;
  untouched.

`peer` (the human entity) — RETIRED → `persona`

  Every use where "peer" meant the human's manifestation in the system: "a
  peer is the representation of a human," "weight attaches to the peer,"
  "one per distinct peer," "a peer's rights floor," "a gated peer's right."
  These became `persona`.

`peer` (adjudication-locus genus) — REASSIGNED → `principal`

  Where the locus could be a group as well as a human ("a peer is a locus
  that can adjudicate," "no authority tier above the peer," "recursive
  peer-is-a-group," "leaves adjudication with the peer"). These became
  `principal`, the genus the spec already defined, of which persona is the
  human kind.

`peer` (solo sync-protocol actor) — REASSIGNED → `participant`

  Where "peer" named a single node performing a mechanical step with no
  relating in play ("a peer accepting a write MUST record," "a peer that has
  seen fewer facts computes a stale state," "the peer's frontier," "a lagging
  peer under-authorizes"). These became `participant` (or `node` where it was
  literally the hardware/admission case).

---

## 2. Defined-term changes

`PeerSet` → `PrincipalSet`

  Decided this session. The set binds roles and capabilities to *any*
  principal (a persona, but also a group or delegate), so it was never the
  entity sense; it is the genus. PersonaSet would have been too narrow.
  15 occurrences in Part 2, 1 in Part 1.

`peerhood` → `personahood`

  Entity-sense ("what it is to be the entity"). Includes the §8
  "peerhood-preserving primitive" → "personahood-preserving primitive."

`P-Peer-Equality` — KEPT as the principle name

  The principle's identifier is unchanged. Under the relation reading,
  "peer-equality" reads correctly as equality-in-the-peer-relation among
  personae. Renaming a cross-referenced principle was judged high-cost,
  low-gain. Its *body* was rewritten (Part 1 §2.3, Part 2 §5.0) so the
  entity is persona throughout while the principle name stays.

`peer-governed`, `peer-equal`, "of peers" — KEPT

  Relation-compounds.

---

## 3. Argued rewrites (grew, not just substituted)

These passages were re-cut because they carry argument. Each now states the
two-rung split (mechanical guarantee vs social judgment) and, where
relevant, the could-not-not-merely-does-not framing.

Part 2 §3.1 opening and ordering sentence — Resolution 1: locus = principal;
"system of peers" reread as the relation, with a new sentence defining the
peer-relation at the spot the heading invokes it. "sensor" kept (flagged for
later review per session note; not changed).

Part 2 §4.5 — the fold now folds clients/devices to "one persona"; "every
participant computes identically" for the consensus step.

Part 2 §5.0 crux paragraph — persona defined as the human layer's
manifestation, a principal by virtue of its key pair; mechanical guarantee
and social judgment on separate sentences; added the "could not certify"
clause.

Part 2 §5.2 — heading "Principal, client, persona"; the kind-of-principal
bullet redefines persona with the adjudication property foregrounded; the
keystone box now distinguishes lineage / persona / personhood and adds that
to *recognize* is to decide to **treat** a lineage as a distinct person, never
to *verify* it; a note that "peer" as an entity noun is retired in favour of
persona.

Part 2 §5.6 — weight attaches to persona, flat one per distinct persona; the
two-layer box gains the could-not framing (no fact to deliver, no tier to
impose from, with the gating-vs-enforcement carve-out); the Sybil gradient
lead rewritten to separate across-systems multiplicity (intended) from
within-group multiplicity (unenforceable, consequence = degraded governance)
and to state recognition-strength proportionality (never weight
proportionality).

Part 1 §2.3 commitment — parallels Part 2 §5.0: persona as the human layer's
manifestation, the could-not clause, and an explicit "personae stand in peer
relation; peer names the relation, not the entity."

Part 1 §2.3 recursion — "A principal is recursively a group" (genus).

Part 1 field-integrity (§2.x) — the asserting/perceiving/contesting actor is
the persona throughout.

Appendix B (Part 2) — authority-grounding passage now grounds a persona's
authority; "peer→sensor rot" → "principal→sensor rot"; "what binds a human
to a persona."

---

## 4. Deliberately NOT changed (with reason)

- All `peer-to-peer` and transport reachability language. (Reserved.)

- Consensus/relation "peer": "every honest peer MUST agree," "deterministic
  on every peer," "which a peer can defend," "two peers that saw a different
  view." (Relation sense, per session decision.)

- The quoted line at §7.3.1 ("trivially gamed by a peer lying about its
  clock"). (It is a quotation of external framing.)

- Meta-discussion of the word itself in §5 ("a single overloaded word
  (\"peer\") previously hid several ideas"). (Talking about the word.)

- "sensor" as the §3.1 contrast term. (Flagged for review, not changed, per
  session instruction.)

- `P-Peer-Equality` name. (§2.)

---

## 5. Open items carried forward (not resolved by this migration)

These are unchanged from the specs' own open lists and are flagged so the
vocabulary migration is not mistaken for resolving them:

- `tenure` under re-key (Part 2 §5.3, Appendix B): unaffected by the rename;
  still open.

- `PrincipalSet` membership semantics: confirm a PrincipalSet can in fact
  hold a group/delegate, not only personae, since that is the premise of the
  PrincipalSet (vs PersonaSet) naming. If in practice it only ever holds
  personae, revisit the name.

- The "sensor" term review flagged in §3.1.

- Whether the §3.1 heading should ultimately read "of peers" (kept) or "of
  principals" (Resolution 2). Resolution 1 was applied; Resolution 2 remains
  available if the kept-relation heading reads as residual overload in
  practice.

---

## 6. How to review

The unified diffs (drystone-part1.diff, drystone-part2.diff) show every
changed line. Suggested review order:

1. Read the four argued rewrites first (§3 above points to them); these are
   where meaning, not just wording, moved.

2. Skim the diffs for the mechanical substitutions; these should read as
   obvious once the rewrites are accepted.

3. Spot-check the KEPT consensus-"peer" lines (§4) to confirm they still
   read as the relation and not as residual entity-nouns.

---

## 7. Validation pass (post-migration)

Both specs were validated against the term lattice and six invariants now
recorded in `persona-definition.md` Note 4 (the conformance oracle).

Result: clean. No violations of I1 (only principals hold role/right/weight),
I2 (weight→persona, right→principal-flows-to-clients, resource→node,
role→principal), I3 (counts personae by lineage, never clients), I4
(capability is issued-under-a-role, never a fifth equality-property), I5
(adjudication-locus = principal), or I6 (binding is a group judgment, never
verified/attested by the protocol).

One edge case checked and **kept** as a sanctioned exception: the §3.1
archetype "zero peers in the sense that matters," which reads as the relation
(absence of peer-standing in a sensor mesh), not the retired entity sense.
Recorded in oracle Note 4i.

Attachment statements confirmed at source: rights "Attaches to the
principal, flows to its clients" (§5.0); weight "Attaches to the persona"
(§5.0, §5.6). §3.1 Resolution-1 sentence present and coherent.

---

## 8. Definitions folded into the specs (travels-together pass)

To make the vocabulary travel with the documents rather than only in the
companion file:

- **Persona is now defined in both parts**, each at its own register. Part 1
  §1 carries a labeled definition note (prose/principle form) placed before
  the term first does load-bearing work (~line 49, ahead of first use at
  ~114); Part 1 §2.3 keeps the commitment. Part 2 §5.0/§5.2 carry the
  mechanical definition. The two are deliberately at different altitudes and
  do not duplicate text.

- **The peer/persona distinction line** ("peer names the relation, never a
  noun for the entity") now appears consistently in Part 1 (§1 note and
  §2.3), Part 2 §5.2, and Appendix D.

- **The term lattice + invariants (the oracle)** moved into **Part 2 Appendix
  D** as the vocabulary of record, with §5 named as the governing source if
  the two ever differ. §5.2 points to it. Part 1 §1 note points to Part 2
  §5.2 for the mechanics. The standalone `persona-definition.md` Note 4
  remains the working copy of the same oracle; Appendix D is now the
  in-spec home.

Single-source discipline: the prose definitions (§5) are primary; Appendix D
and the standalone doc are synthesis over them and say so. No claim is stated
as primary in more than one place.
