# The human-adjudication mechanism: survey and canonical language proposal

date: 2026-07-13

status: PROPOSAL (survey of current usage plus a codification plan). No files changed. Feeds three later edits: the Part 1 §3 prior-art addition, a Part 2 description-rules block, and the Claude Code instruction set to be drafted after the next repo drop lands.

## Terms (per Rule 2 of doc-writing-method.md)

- **The mechanism** (this doc's subject): the designed path by which a question the protocol cannot answer (a utility question) is surfaced to the humans who can. It spans a recognizer, an event, a channel, a deliverable, and an outcome space, named below.

- **The seam** (inherited, Part 2 §7.6.1: "the razor's seam, Part 1 §2.0"): the condition that triggers the mechanism, provenance fully settled and utility still open.

- **Utility / provenance** (inherited, Part 1 §2.0): what is right or wanted vs what is consistent and corroborated. The razor: compute provenance, never utility.

- **Planes of authority** (this doc, from the operator clarification): infrastructure authority (running relays, helpers, recovery services; held by *operators*) and social-utility authority (judgments about what a group or person wants; held by *local authorities*). The two are separated by design, with no path from one into the other, like mutually exclusive Group Roles. Operators are necessary and good when aligned to this separation; the separation, not the operator, is the load-bearing constraint.

## 1. Survey: where the mechanism lives today

**Part 1.** §2.0 states the razor. §2.5 is the forced terminus: fork, not verdict, "the provably-non-empty residue." §3 grounds the escalate-the-residue posture in Beer's algedonic channel (Brain of the Firm, 1972), with two open `[confirm]` items against the primary edition. The phrase "human-adjudicated fork" appears in the §0 map. No IANA/IETF material anywhere in Part 1 yet (grep: zero hits for IANA, IETF, designated expert, 8126, 7282).

**Part 2 §7.6 is the anchor definition** and it is already strong. The normative event is the **reconcile hard-stop** ("MUST detect membership contradictions... and hard-stop; MUST NOT silently auto-resolve"). The hard-stop "is Drystone's **algedonic channel** (Part 1 §3)... a *designed* channel, not a failure path," which "keeps both the signal and the authority local," surfacing to "the affected Group rather than relocating the decision to a center." §7.6.1 names the two shapes of the residue: **Contradiction (too many valid claims)** and **Under-determination (too few valid claims)**. The machine's deliverable is **legibility** ("every node deterministically computes the same picture") plus **cheap exits in every direction** and **refusal to manufacture the missing authority**; "the judgment stays with the humans; the instantiation of whatever they choose is Drystone's." §7.4.1 makes the recognizer itself governed (the benign-versus-dispute tolerance, per-Group, never a constant) and requires that genuinely ambiguous cases **escalate** rather than auto-recover. §7.6.5 gives the outcome space three registers: mute, governance, fork.

**Conformance suite** (alpha/crystallized/conformance-suite.md). Reconcile corpus C1–C10: "each merge scenario → the expected **verdict** (converge | hard-stop contradiction | re-formation fork)," and the must-reject list includes "a contradiction (hard-stop, no auto-resolve)."

**Experiments.** The freshest and best-aligned artifact is croft-chat plans/2026-07-12-1-design-concurrent-contradiction.md: `ForkStatus::Contradiction` / `ForkedFrom` / `UnderDetermined`, "the escalation channel, the one thing §7.5.2/§7.6 says must not erode," "surfaces both claims (the legible picture) for the humans → fork. No auto-resolve, by §2.5." The iroh corpus (the Alt.Drive-flavored spike) speaks a different language entirely: "conflict resolution," last-writer-wins, timestamp tiebreaks (iroh/DESIGN.md §7, threat-model.md).

**Usage counts, spec vs experiments** (grep across .md):

| term | drystone-spec | experiments |
|---|---|---|
| fork-not-verdict | 16 | 0 |
| algedonic | 11 | 0 |
| escalation set | 6 | 1 |
| hard-stop family | 30 | 56 (4 spellings: hard-stop, hard stop, hardstop, hard_stops) |
| conflict resolution | 0 (in normative text) | 9 (all iroh) |

Reading: **the specs are internally consistent; the drift is one-directional, in the experiment corpus.** The task is not to invent language but to promote the spec's existing language to canonical, resolve four collisions, and write the rules that keep experiment docs on it.

## 2. Findings: collisions and drift to resolve

**F-A. "Operator" names a different plane of authority, so it cannot also name this one.** Corpus-wide, *operator* means the infrastructure/service operator (threat-model S10/S11, §7.6's "helper operated by a cooperative or external operator," the no-operator-mediated-recovery posture). That role is legitimate and necessary, and the design point is not distrust of it but **plane separation**: trust is a gradient, and "no operator to trust" scopes to *social utility*. There is deliberately no path for an operator to cross planes and exercise authority inside a group's social graph, and a good operator is one aligned to that separation. The operator role and the local-authority role are therefore **mutually exclusive by design**, the same way mutually exclusive Group Roles are, which is exactly why one word cannot cover both. The corpus already has the right words for the social plane: the **principal** is the adjudication locus (persona-definition.md I5: "the locus-of-adjudication is the principal; persona is its human kind"), the collective recipient is **the affected Group**, and §7.6 already says the escalation "keeps... the authority **local**." Proposal: coin **local authority** as the mechanism-facing name for whoever holds a utility judgment (a principal for its own judgments, the affected Group for shared ones), keep *operator* for the infrastructure plane, and state the relation between them as plane separation rather than exclusion of a threat.

**F-B. "Verdict" is used mechanically where fork-not-verdict is the doctrine.** The conformance suite's "expected verdict (converge | hard-stop contradiction | re-formation fork)" uses the one word the spec spends sixteen mentions forbidding. Rename to **expected disposition**. Feasibility-review-v2.md also uses "Verdict:" as a review-note header; harmless in a review artifact but worth avoiding in anything normative.

**F-C. Two names for one channel** (Rule 7 violation in the making). Part 2 calls it the algedonic channel; the croft-chat design doc calls it the escalation channel. Same thing. Proposal: **algedonic channel** is the Part 1 lineage name (keeps the Beer credit and the prior-art thread); **escalation** is the operational verb family in Part 2 and code ("escalates," "the escalation," "escalation shapes"). One mechanism, one definition at §7.6, the lineage name and the operational name both anchored there.

**F-D. "Escalation set" is doing double duty.** §7.6.1's heading uses it for the two *shapes* of the residue; the beat E5 running example uses it for the two *people* ("the escalation set is both Alice and Bob, not one of them"). Split the term: **escalation shapes** (Contradiction, Under-determination) for the taxonomy, and **escalation parties** for the humans surfaced, with the symmetry point (both parties, no presumed wrongdoer) attached to the latter.

**F-E. Spelling drift on the core noun.** Four variants of hard-stop in experiments (TEST-LOG.md test names use `hard_stops`, fine for identifiers; prose uses "hard stop" and "hardstop"). Canonical prose spelling: **hard-stop**, hyphenated, noun and verb.

**F-F. The iroh spike's LWW-by-timestamp language is a rogue; clean it out.** "Conflict resolution," last-writer-wins, and timestamp tiebreaks (iroh/DESIGN.md §7 and 319/402/591, threat-model.md 417/441, README 327/440) contradict both fork-not-verdict and the timestamps-are-unprovable-assertions invariant, and they read as precedent to anyone grepping the corpus. Decision: excise rather than scope-note. The Claude Code pass should rewrite those passages (or mark the affected sections superseded with a pointer to §7.6 and the ordering invariant) so the corpus stops teaching a resolution model the protocol forbids.

## 3. The canonical vocabulary (proposal)

The mechanism, end to end, in the corpus's own words:

- **The seam**: provenance fully settled, utility open (the razor's seam, Part 1 §2.0). This is *when* the mechanism applies.

- **The recognizer**: the governed classifier of §7.4.1 (benign-versus-dispute tolerance, per-Group, never a constant). Recognizing the seam is itself partly a utility judgment, which is why the classifier is governed and why ambiguity MUST escalate. The spec already says this (Part 1: "distinguishing a benign sync artifact is *itself* partly a utility judgment"); the rule below makes it a required framing wherever the recognizer is discussed.

- **The event**: the **reconcile hard-stop** (Part 2 §7.6). Spelled hard-stop.

- **The shapes**: **Contradiction** (too many valid claims) and **Under-determination** (too few). Code: `ForkStatus::Contradiction`, `ForkStatus::UnderDetermined` (plus `ForkedFrom` for genesis collision).

- **The channel**: the **algedonic channel** (lineage name, Part 1 §3, Beer); operationally, **escalation**. Designed, not a failure path; signal and authority stay local.

- **The deliverable**: the **legible picture**, a grounded statement of the conflicting facts in governance language, full provenance, no editorializing, identical on every node (P-Knowable-Truth).

- **The receiving side**: the **local authority**, the principal for its own judgments, the **affected Group** for shared ones. *Operator* is reserved for the infrastructure plane; the two roles are mutually exclusive by design (plane separation), so escalation text never routes a utility judgment to an operator and never uses *operator* to mean the human deciding.

- **The outcome space**: the **three registers** (mute, governance, fork); the terminus register is the **re-formation fork**, mechanically a **re-plant** at fork arity; **fork-not-verdict** names the doctrine.

- **The boundary sentence** (the reusable pattern, from §7.6.1): *the protocol supplies the conditions for cheap human resolution, not the resolution; the judgment stays with the humans; the instantiation of whatever they choose is Drystone's.*

## 4. Description rules for Part 2 (the codifiable block)

Drafted to sit beside the doc-writing-method conventions; each is checkable by grep or by read.

**DR-1 (one anchor).** The mechanism is defined once, at §7.6, and every other mention points there (doc-method Rule 7). No section re-derives it.

**DR-2 (terms front-loaded).** The Terms/conventions anchor gains entries for: seam, reconcile hard-stop, escalation shapes (Contradiction, Under-determination), algedonic channel, legible picture, local authority, three registers, re-formation fork, re-plant (doc-method Rule 2).

**DR-3 (spelling).** *hard-stop* in prose, hyphenated, noun and verb. Snake case only inside code identifiers.

**DR-4 (planes of authority; operator is the infrastructure plane).** *Operator* refers only to the infrastructure-plane role, which is legitimate and necessary. The holder of a social-utility judgment is the *local authority* (or *principal* / *affected Group* where the specific kind matters). The relation between the roles is stated as **plane separation**, never as distrust: write "the operator holds no path into social-utility authority" rather than "no operator to trust," and scope any trust statement to its plane ("trust for what"). The two roles are mutually exclusive by design, like mutually exclusive Group Roles.

**DR-5 (continuity language, never moral language).** Descriptions of escalated cases use symmetric, continuity-framed vocabulary: parties at equal standing, divergence, lineage, continuity, departure point. Banned in normative text about the mechanism: wrong, offender, violation, punish, guilty, dispute-as-default. Motivation is unprovable from provenance and is never asserted; a case description must read identically whether the underlying story is grievance or routine mechanics (the archived-after-30-days-inactivity test: if the sentence would sound accusatory applied to that case, rewrite it).

**DR-6 (no machine verdicts, anywhere).** Machine outputs are dispositions, statuses, or pictures, never verdicts, rulings, or decisions. Applies to test suites: conformance C1–C10 "expected verdict" becomes "expected disposition."

**DR-7 (the recognizer is governed).** Any text describing detection of the seam must state that the classifier is governed per §7.4.1 and that ambiguity escalates. Never describe the recognizer as a constant or as the protocol "knowing" a dispute occurred.

**DR-8 (name the shape).** Every escalation mention states which shape it concerns, Contradiction, Under-determination, or both. A mechanism watching only for one misses the other (§7.6.1).

**DR-9 (the boundary sentence).** Where the machine/human boundary is described, use the three-clause pattern: surfaces the legible picture / the humans supply utility / the instantiation of their choice is Drystone's. Keeps the division of labor identical at every mention.

**DR-10 (both signal and authority stay local).** Escalation text must not imply relocation of the decision: it surfaces *to* the affected Group, never *up* to anything.

## 5. The Part 1 addition: the designated-expert lineage

Where: §3 (Why these principles are corroborated, not invented), alongside the Beer entry, plus one sentence in the §2.5 terminus where the fork is called human-adjudicated.

The claim to make, and its shape:

1. **Internet governance already names a human judgment step inside its own machinery.** RFC 8126 (BCP 26) defines the Designated Expert registration policy: IANA forwards a registration request to a named human for evaluation, and the expert informs IANA whether to make the assignment; the experts are listed in the registry itself, and escalation runs to the IESG. A formal spec of the Internet age that says, at this point in the procedure, a person decides.

2. **Web standards make the human input normative, not advisory.** The W3C Permissions framework defines powerful features that user agents must not exercise without the user's express permission, and the Geolocation spec enforces this with normative check-permission steps inside the API algorithms. The spec pins the mechanics on both sides and leaves a typed hole only a human can fill.

3. **The IETF documents its own human layer in the same series as its protocols.** RFC 7282 (On Consensus and Humming) specifies rough consensus as addressing objections, not counting votes, precedent for publishing social-adjudication semantics alongside wire formats.

4. **Drystone's inversion, stated explicitly.** RFC 8126 delegates the judgment to one named expert per registry, a center. Drystone distributes the designated-expert role to every principal: each local authority is the designated expert for its own utility judgments, because a persona only represents a person if that person holds the adjudication (the peer-standing argument). Beer gives the channel (escalate the residue, keep authority where the context lives); RFC 8126 gives the named human role inside formal machinery; Drystone composes them and removes the center.

Grounding duties (doc-method Rules 6 and 14): cite RFC 8126 §5 (Designated Experts) and §4 (registration policies) as primary; W3C Permissions TR and Geolocation TR as primary, noting the express-permission requirement is normatively enforced by the check-permission steps; RFC 7282 as informational. Mark for verbatim confirmation against the primaries before landing: the exact RFC 8126 clause on IANA-forwards-expert-decides, and the current Geolocation CR wording of the express-permission requirement (the 2016 REC wording differs slightly from the 2026 CR).

## 6. Sequencing (unchanged from what you set)

1. Next repo drop lands (your in-flight refinements).
2. Then: Claude Code instruction set covering (a) Part 1 §3 + §2.5 additions per §5 above, (b) the Part 2 Terms entries and DR block per §4, (c) the conformance-suite verdict→disposition rename, (d) cleaning the rogue LWW/timestamp conflict-resolution language out of the iroh corpus, (e) the two hard-stop spelling fixes in experiment prose, (f) back-map updates for every touched section per the set's map convention.
