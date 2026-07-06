# Session summary: the relational-field argument and its merge into Drystone

date: 2026-06-29

purpose: a record of what this working session produced and decided, to file alongside the
deliverables. It is a map of the work and its epistemic state, not a new argument. Where something
is unverified or owed, it says so.

---

## What this session set out to do

Build a case, grounded enough to defend and plain enough for a non-expert to act on, that
advertising-funded social platforms do not merely expose users (the privacy harm) but author the
content and texture of users' relationships against the users' own interests. Then connect that
case to Drystone as the protocol layer and to the cooperative as the ownership layer, as one
mutually-reinforcing stack rather than three separate arguments.

The framing decision that shaped everything: lead with **authorship and agency**, not privacy.
Privacy has lost to convenience for twenty years because it is a loss-prevention good. The
authorship frame (they are writing the room you live in, and you are not the author and not the
beneficiary) is a different and more motivating category of claim.

---

## Deliverables produced (7 files)

**peer-standing.md** : the grounded structural argument. Securitized ad-funded corporations cannot
constitute peer relationships; the cooperative is the form in which alignment can be made binding
(not the form that is automatically aligned). Carries the full empirical limb in section 7.

**tilling-the-soil.md** : the human-facing essay. The farmer/soil/crop allegory, landing on "they
can only farm us, and we are not a field." The persuasive entry point; points to peer-standing for
grounding.

**elevator-pitch.md** : three registers (one-liner, ~20-second spoken, short written), all leading
with the reframe and routing around privacy on purpose.

**drystone-part1.md** : the spec's "why," now with section 2.6 merged in (see below).

**drystone-part2.md** : the spec's "what," with one back-reference added at section 7.4 (see below).

**drystone-open-items.md** : the read-and-decide ledger, with this session's additions recorded.

**drystone-part1-voice-bridge.md** : the standalone pre-merge draft of section 2.6, kept as a
reference artifact. Redundant with section 2.6 inside Part 1; the only file still containing
em-dashes (5), left as-is because it is superseded.

---

## What was added or changed this session

**The companion narrative set** (peer-standing, tilling-the-soil, elevator-pitch) was built and
then hardened. The essay's allegory was judged to earn its place as the lead rather than the
payoff. The pitch leads the reframe because at pitch length only one outcome lands.

**section 7 of peer-standing was rebuilt into a four-plank layered structure**, strongest claim
first, so the thesis does not depend on the contested layers:

- Plank A (misaligned objective): the company's own admission that engagement and endorsement
  diverge (Zuckerberg 2018), corroborated by an external causal audit (Milli et al., PNAS Nexus
  2025). Requires no claim that anyone's attitudes were changed.

- Plank B (mechanism): the revealed-preference assumption that makes the divergence structural
  (Kleinberg, Mullainathan & Raghavan, Management Science 2024).

- Plank C (the reflexive harm, the sharpest claim): the corrective faculty and the damaged faculty
  are the same. Now sourced (Milano et al. 2021) for the targeting case, with the
  engagement-ranking extension marked as our step and supported by a ranking-specific causal
  experiment (Burton, Herzog & Lorenz-Spreen 2024/2026).

- Plank D (the honest other side, at full strength): the Meta/2020 null (Guess et al. 2023), the
  63 break-glass changes, the 2026 X-study directional asymmetry, and Nyhan's own hedge. The
  contested literature refutes the overclaim (the algorithm radicalizes the average user; feeds
  flip stable attitudes), which was never our claim; it does not touch the exposure/emotion/
  salience layer the thesis targets.

**The alignment-by-design correction** was folded into section 6 of peer-standing: the cooperative
is necessary but not sufficient. It is the form in which alignment becomes a binding, enforceable
charter commitment rather than a discretion an external capital constituency may revoke. The
funding frontier (edge-preserving capital formation) is named as the live open problem, with the
ICA autonomy principle stated as its principled form.

**Drystone Part 1 section 2.6 was written and merged** (placed after section 2.5, before section
3). It names one dependency the four principles imply but none states: the voice right requires
field-integrity. A peer cannot hold voice on a substrate where a center authors the field, because
the right and field-integrity are the same precondition seen from two sides. Field-integrity is
not an unshaped field (the field is always ordered); it is three properties of the shaping:
peer-governed, legible, exitable. Each maps to an existing mechanism. It adds no new principle and
no new wire obligation, and is explicitly not a fifth peer-property. The endemic-ordering point is
carried as a marked tension. The "What Part 1 establishes" summary gained one sentence noting the
dependency.

**Drystone Part 2 section 7.4 gained a one-sentence back-reference**, tying the
silence-is-not-currency rule to section 2.6's legibility property. Nothing else in Part 2 was
touched, per the keep-it-minor instruction.

**The open-items ledger was cross-checked.** None of B1 through B7 changed; this session did not
touch the protocol mechanics they concern. Two additive entries: section 2.6 recorded in the
settled-this-round ledger, and a note that the companion narrative is tracked separately with its
own verification state (so the two documents' source-discipline are not conflated).

**Em-dashes were stripped** from peer-standing (57 to 0) and elevator-pitch (9 to 0), using
role-based replacement (commas for appositives, semicolons for clause joins, colons for
definitions and headings), with a splice-check pass that hand-fixed 12 comma-splices the
conversion would otherwise have introduced. All six core deliverables are now em-dash-free.

---

## Verification state (what was checked against primary sources this session)

Confirmed verbatim or as-characterized, this session:

- Milli et al., PNAS Nexus 2025: the "beyond stated preferences" finding is verbatim accurate;
  scope is political/out-group-hostile content on Twitter vs. reverse-chronological. The
  stated-preference timeline's own pro-attitudinal-bubble failure mode is in the paper and is
  carried as a tension.

- Kleinberg, Mullainathan & Raghavan, Management Science 2024: revealed-preference mechanism and
  the "transcends the specific incentives of any particular platform" framing, including the
  altruistic-platform model.

- Milano et al., Nature Machine Intelligence 2021: the epistemic-fragmentation and reflexive-failure
  lines, verbatim. Scope is targeted advertising; the ranking extension is marked as ours.

- Guess et al., Science 2023: the null result verbatim; the 63 break-glass changes confirmed in the
  published Science correspondence; Nyhan's hedge confirmed in Science news coverage.

- The political effects of X's feed algorithm, Nature 2026: the switching-on-shifts-opinion,
  switching-off-does-not asymmetry.

- ICA Statement on the Cooperative Identity (1995): one-member-one-vote, patronage-based surplus,
  limited compensation on capital, and the autonomy principle's external-capital condition.

- Project Mercury: the three quoted lines and Meta's rebuttal, confirmed across independent reports
  of the November 2025 filing.

Corrected or removed for cause:

- The "Algorithms vs. Peers" WeChat RCT was removed from support. On verification its authors frame
  it as challenging filter-bubble concerns; it cuts against the use it was put to.

- The Facebook MSI claim was reframed from stated causal fact to "Facebook's own internal research
  concluded," with the correlational study caveated.

- The PNAS Nexus claim was scoped down from "shapes relations generally" to the political-content
  instance, with the generalization marked as ours.

---

## Open and owed (carried, not resolved)

**Project Mercury docket (time-sensitive).** The hearing was set for January 26, 2026. A re-check
of public reporting as of late June 2026 surfaced no post-hearing ruling or disposition of the
motion to strike. Before peer-standing is published externally, the court docket (PACER, N.D. Cal.
social-media MDL) must be pulled directly, because a ruling would change whether the quotes may be
cited as unsealed and how they must be characterized. This does not gate the Drystone spec.

**Two citations still owed in peer-standing section 7.** A "ranking for engagement" / ScienceDirect
source the prior draft referenced could not be located; the nearest located study is contrary
rather than supporting. And a replacement for the WeChat RCT, if the narrower-exposure point is to
be kept, needs a source that actually supports it. Both are flagged in the document, not used as
support.

**The four-faculty decomposition in Plank C is our synthesis**, not individually sourced. Milano
sources the reflexive structure; Burton sources that ranking shapes belief and consensus; the
specific four-way carve (shared reference, good-faith disagreement, calibrating trust, attention)
is the structuring move and is marked as such. Each faculty has its own literature that could
ground it individually (false polarization for shared reference, the Lees & Cikara meta-perception
work for trust); that is a future pass, not a current gap.

**Drystone open items unchanged.** B1 (tenure under re-key) and B3 (succession scope) need a
ruling; B2 (Meadowcap/MLS fork-merge) and B6 (define revocation needs) are design investigations to
schedule; B4 (capped-root soundness coverage) stays the priority security item. None was advanced
this session, which did not touch protocol mechanics.

**Carried spec primary-source confirmations** (unchanged from the open-items ledger): Beer and the
Cybersyn/OGAS figures; Ostrom 1990 principles 6 and 7 and the 2013 subsidiarity wording; the
decentralized-MLS drafts; TLS/X.509 against RFC 5280 and SSH against the OpenSSH man pages; Keyhive.

---

## The stack, in one line

The protocol guarantees a peer-governed, legible, exitable field by removing the center that would
otherwise author it; the cooperative form guarantees nothing external is installed to re-author it,
because the member is who the entity serves and the member's attention is therefore not the salable
good. You need both: the protocol can be deployed under an ownership form that reintroduces a
curating center, and the ownership form can be adopted over a substrate that still has one.
