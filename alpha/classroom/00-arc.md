# The Classroom Arc — Drystone from two people to the planet

`Draft chapter-beat sketch for the classroom tier. Character: the guide — patient, cumulative,
need-before-mechanism. Invariant across every chapter: true at every altitude. Every chapter runs
the same beat structure: NEED (the story breaks without it) → STORY → DIAGRAM (Mermaid) → the
PRECISE STATEMENT → PROVE-IT (test names / EVIDENCE-MAP rows / one command) → the REFRAIN.`

**The refrain** (closes every chapter, the pedagogical spine): *"And underneath, nothing changed:
it is still two people keeping their own memory of what was said, signing it, and pointing at each
other's words."*

**The escalation logic.** Three acts, each adding one axis while the provenance plane stays fixed:
Act I adds **people** (social complexity), Act II adds **space** (network complexity), Act III adds
**institutions** (economic complexity). The reader should be able to say at the end of every act
what grew and what provably did not.

---

## Act I — The Room (people)

### Ch. 1 — Two people
- NEED: remember what was said, without either owning the record.
- STORY: Alice and Bob talk. Each keeps their own notebook. Nobody's notebook is "the" notebook.
- MECHANISM: canonical local state; a statement is a fact signed by its author; "as you said
  before" is a hash reference — ordering by who-references-whom, no clock anywhere.
- REAL-WORLD: any conversation; "you said yesterday" works without a timestamp server.
- PROVE-IT: `dedup.rs`, `convergence.rs` (byte-identical folds across arrival orders);
  EVIDENCE-MAP §7.3.
- DIAGRAM: two devices, two logs, facts crossing with reference arrows (drafted).

### Ch. 2 — Three people: the witness
- NEED: hearsay, handled honestly.
- STORY: Carol joins. Carol hears from Bob what Alice said. Carol also sees Alice and Bob speak at
  the same moment — which came first? Neither.
- MECHANISM: assertion vs corroboration; quantified trust (a claim is as strong as its independent
  attestations); first genuine concurrency, and why the notebooks still converge.
- REAL-WORLD: gossip; "Bob told me you said…"; two people talking over each other and the group
  still agreeing on what was said, if not on the interleaving.
- PROVE-IT: `convergence.rs` order-independence; the corroboration counting path.

### Ch. 3 — Seven people: the club
- NEED: decisions that stick, with no boss.
- STORY: a book club. Dues change needs 3 of 7. The treasurer role. Someone is asked to leave —
  and can no longer read what's said after the door closes. Two motions pass simultaneously in
  opposite corners; the room stops and looks at both.
- MECHANISM: rules and roles as facts; k-of-n with the content-bound approval (you approve *these
  exact words*); the co-signed op; prior-rule-governs (you can't lower the gate with the act being
  gated); removal → ceiling → re-key (PCS); the contradiction banner: two honest quorums are a
  fact to show, never a race to win.
- REAL-WORLD: committee votes; minutes recording exactly what was moved; bait-and-switch on a
  motion's text failing because approvals name the text.
- PROVE-IT: `rulechange_quorum_via_api.rs` (end to end), `competing_quorums.rs` (both orders, same
  banner, nobody's version wins), `l2a_sealed_frame.rs` assertion 4 (the removed reader really
  can't read); EVIDENCE-MAP §7.2 R7, §7.3.2.

### Ch. 4 — Twenty-one: the hall
- NEED: you can't hear everyone anymore.
- STORY: a town-hall-sized group. Side conversations carry the news; a latecomer catches up from
  whoever's nearest; the group keeps periodic minutes so catching up never means re-reading a year.
- MECHANISM: swarm fan-out (epidemic delivery), catch-up on connect, steady-state repair (a
  dropped remark gets refilled without anyone reconnecting), horizons and checkpoints (minutes =
  corroborated "we all folded to the same page here").
- REAL-WORLD: how news actually moves through a large room; meeting minutes; "what did I miss?"
- PROVE-IT: `FANOUT-M1` (live_sent = 2N+1, exact, 150/150), the M2 steady-state tests, EXP-H1
  manifest determinism; EVIDENCE-MAP §6.8.1, §7.6.9.

---

## Act II — The World (space)

### Ch. 5 — The split room
- NEED: same club, two buildings.
- STORY: seven of the twenty-one travel. Hotel wifi. The two rooms keep talking as one group; a
  new member in Room 2 is welcomed by someone in Room 1 and can immediately read.
- MECHANISM: NATs and why devices can't just dial each other; the relay (coordination only, reads
  nothing); holepunching with relay fallback; the Welcome crossing a real wire; the group object
  never noticing the room boundary (continuity lives in the lineage).
- REAL-WORLD: any distributed family or team; the phone network carrying a call it cannot hear.
- PROVE-IT: `mls-welcome-over-iroh` (a real Welcome, both sides derive the same epoch secret,
  RUN-08); §7.6.2 continuity tests. AND the honesty lesson taught explicitly: the one arrow not
  yet proven on real routers is register row `hermetic-gossip` — the classroom shows the ladder,
  not just the rungs we hold.
- DIAGRAM: two rooms, relay, holepunched path (drafted).

### Ch. 6 — The helpers
- NEED: somebody runs the relays; somebody stores your encrypted history while your phone is dead;
  somebody answers a page at 3 a.m.
- STORY: enumerate the helper cast — relay operators, blob/storage providers, app vendors,
  recovery share-holders — and give the group *several* of each, interchangeably.
- MECHANISM: the operator plane and plane separation. Trust is a gradient with a direction:
  "trust *for what*?" You trust the relay to carry, never to read; the store to hold ciphertext,
  never to open it; and NO helper holds a path into the group's social authority — operator and
  member-authority are mutually exclusive roles by construction. Interchangeability follows:
  because helpers live entirely on the provenance plane's outside, swapping them changes nothing
  the group can see in its own history.
- REAL-WORLD: the phone company carries the call without joining the marriage; your email host
  doesn't get a vote in your family.
- PROVE-IT: mostly proofs of absence, said plainly — §5.11 read-scoping (the store can't read),
  the L2a firewall-guard test (the API exposes no authority/projection knob), R6 attributable
  acceptance (a helper-fed lie is attributable). This chapter introduces the idea that *absence
  proofs are proofs*.

---

## Act III — The Institutions (money and mission)

### Ch. 7 — The co-op
- NEED: the group wants its infrastructure to answer to it.
- STORY: the club and its neighbors charter a co-op to run their relays and storage. Members own
  the operator. And here the arc folds onto itself: the co-op's own bylaws — dues, board quorum,
  changing the change-rule — are the *same machinery from Chapter 3*, because a co-op is just
  another group.
- MECHANISM: nothing new — that is the lesson. Governance of the infrastructure reuses R7
  verbatim; the cooperative-ownership companion argument grounds why this pairing is natural.
- REAL-WORLD: rural electric co-ops, food co-ops, community mesh networks.
- PROVE-IT: the reuse itself — the co-op's rule changes are RuleChange facts through the same
  tests as Chapter 3.

### Ch. 8 — The nonprofit
- NEED: groups that can't pay; neutrality; and someone to hold a recovery share.
- STORY: a foundation runs free relays for shelters, schools, a union local. It also serves as one
  of the *independent trust domains* holding a share of a member's sealed recovery payload — the
  library that keeps a copy of your spare key without the power to use it alone.
- MECHANISM: same operator plane, different funding — and the protocol *cannot see the
  difference*: funding model is a utility-layer fact. Recovery's two tiers: the lock (threshold
  shares across independent domains, built) vs the trust predicate (who may trigger release — the
  open human question, taught as open).
- REAL-WORLD: public libraries, Signal-style foundations, community bail funds.
- PROVE-IT: the BIP39 lock spike (the lock round-trips, wrong key fails); the trust tier cited
  honestly as I9, the field's largest open call — the classroom teaches the open problem by name.

### Ch. 9 — The for-profit and the dispatch desk
- NEED: paid guarantees. A county emergency-services department runs its dispatch channels on the
  protocol and pays a vendor a premium for 24/7 operations, redundant relays, audits, and response
  times.
- STORY: the vendor sells uptime and pager duty. What the vendor *cannot* sell — at any price — is
  a supervisor knob: who is on dispatch tonight is the department's own k-of-n, and when a
  responder's access is revoked, their device fails closed even if the vendor's server is behind.
  The department, being prudent, ALSO peers with the co-op's relays: two providers, one group,
  and the group's history can't tell which carried which frame.
- MECHANISM: an SLA is a contract about delivery and availability, never about content or
  membership; the corroboration dials as *operational* settings (dispatch dials freshness k tight;
  delay-over-breach reads as "a stale badge doesn't open the door"); multi-provider redundancy as
  a consequence of plane separation; price discrimination lives entirely on the utility plane.
- REAL-WORLD: county EMS/fire dispatch, its vendor contracts and audits; badge revocation that
  actually sticks.
- PROVE-IT: EXP-C1 stall-at-threshold (enforcement stalls below corroboration, reads continue),
  the PCS re-key test (the removed reader is out), the formula-valued k test (the department's
  "majority of on-shift supervisors" threshold folds identically everywhere).

### Ch. 10 — The planet (coda)
- NEED: see it whole.
- STORY: one persona, many groups — family, club, union local, the dispatch job — differently
  sized, spread over the planet, served by a co-op, a nonprofit, and a vendor at once. A fork in
  one group multi-homed; a temperature read on another; a horizon's minutes co-signed across an
  ocean.
- MECHANISM: recapitulation only. The closing exhibit is the EVIDENCE-MAP itself: every claim the
  course made, linked to its proof — and the honest edges named (real-NAT, the trust predicate) as
  what the field still owes the classroom.
- REFRAIN, final form: *"Twenty-one people, three institutions, and an ocean — and underneath it
  is still two people keeping their own memory of what was said."*

---

## Production notes

- One Mermaid diagram minimum per chapter; the Ch. 1 and Ch. 5 seeds exist. Institutional chapters
  diagram the *planes* (group plane vs operator plane, arrows that exist vs arrows that provably
  don't — draw the absent arrow as a struck-through edge; the absence is the figure).
- Prove-it boxes are generated against EVIDENCE-MAP rows so the site gate catches drift.
- Act III's proofs are mostly proofs of absence plus reuse — say so in the text; it's the point,
  not a weakness.
- The I9 and real-NAT open items are taught by name, never papered over: the classroom teaches the
  ladder, including the rungs not yet climbed.
