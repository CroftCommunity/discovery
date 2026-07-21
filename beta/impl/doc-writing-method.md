# Writing method for Drystone design docs

`Status: working method, descriptive`

`Scope: how the Drystone design documents are written and reviewed. Distinct from the experiment-methodology doc, which governs how claims are validated; this doc governs how a design is written down once decided.`

`Companion to the two-part spec (Part 1 principles, Part 2 mechanics); Part 2 §6, the transport and delivery layer, is the reference exemplar of the practices below.`

---

## Why this doc exists

The Drystone design docs are written to a consistent method that took several passes to settle. Capturing it serves two purposes: new docs start in the right shape rather than rediscovering it, and an existing doc can be reviewed *against* the method, since a stated practice is a practice you can check for. The practices below are not style preferences; each earns its place by fixing a specific failure mode we hit and corrected.

The single organizing principle, from which most of the rest follows: **state the conclusion, grounded, so the merge or the reader can check it; never gesture at reasoning, history, or definitions that live elsewhere.** A doc that defers cannot be checked for consistency. A doc that self-states can.

---

## 1. The doc is an end state, not a record of how the thinking went

State what was decided, as the conclusion. Do not narrate the path: no "an earlier design said X," no "corrected from," no "this was previously framed as," no refutation of a position the reader never held.

**Why.** The reader was not in the discussion. A refutation with no antecedent assertion is pure confusion: it raises a question (what was the old view? why are we arguing against it?) that the doc then has to spend words answering, all of it noise. Interim reasoning belongs in a session log or a decision record, not in the artifact that states the design.

**The seam to watch for.** If a sentence only makes sense to someone who saw the prior version, cut it or rewrite it to stand alone. "It is no more plaintext than any other group" was a real example we cut: it refutes a worry the reader never had, and the clean version simply states what the device group *is* (a Group moving sealed bytes) with no argument against a ghost.

**The subtle form: implicit prior-state contrast.** The violation is not only the explicit "an earlier draft said X." Any construction that only parses if the reader supplies a *superseded prior state* is the same fault, even with no draft mentioned: "no longer receiving special treatment," "is now given a concrete shape," "X was rejected in favour of Y," "the enum was later flattened." Each smuggles in a before-state the reader is being asked to contrast against. The tell is the temporal word doing contrastive work, "no longer," "now," "still," "used to," "once," "later," "these days", where the sentence's meaning depends on an unstated prior. Rewrite to the timeless present: not "the crate no longer gets special treatment" but "the crate is separately versioned, outside the 1.0 guarantee"; not "the seam is now given a concrete shape" but "the seam's shape is Z." Two legitimate exceptions, both narrow: (a) a temporal word describing an *external* fact the reader may meet elsewhere (a dependency's own rename, "iroh's pre-1.0 material calls this `NodeId`") is orientation, not internal path-narration; (b) a **status-tracking appendix or decision record** whose declared job is to log transitions ("this item was open, now resolved") is the sanctioned home for exactly that contrast. Everywhere in the design prose, state the current fact and let it stand.

**Where the history *does* go.** A session summary or decision-record doc (e.g. a session-summary doc) is the right home for "why we changed our minds." Keep it there, so the design doc stays an end state and the history stays auditable, two documents with two jobs, neither muddying the other.

---

## 2. Self-define every load-bearing term, up front, in one place

Open with a Terms section that defines both the vocabulary the doc coins and the vocabulary it inherits. For an inherited term, write *this doc's working definition* and note the source of inheritance, rather than pointing at the source and assuming.

**Why.** A doc that defers its definitions ("see Part 1 for persona") cannot be diffed for consistency, and if its working meaning has quietly drifted from the source, the drift is invisible. A doc that self-defines can be checked term-by-term against the source at merge time, and where the two differ, that difference is *information the merge needs*, not noise to suppress. Self-definition is the stronger move precisely because it surfaces discrepancy instead of hiding it.

**Form.** One authoritative Terms section near the top, used consistently thereafter, beats inline definitions scattered at first use. Scattered definitions drift from later usage and are hard to audit; one anchor per term is checkable. This is the convention RFCs use (RFC 9420 and 9750 both open with Terminology), and it is the form that makes the merge-reconciliation tractable.

**The bug it prevents.** Without it, a notation gets used long before it is introduced. We had "(G, D) cursor" in use 150 lines before the section that defined the (G, D) structure, and MLS terms (leaf, epoch) used before MLS itself was introduced. A front-loaded Terms section dissolves the whole class of "used-before-defined" problems at once.

---

## 3. State the requirement first, then the current realization

For anything the design rests on (a substrate, a library, a primitive), state the *capability the layer requires* in technique-neutral but technically precise terms, then name the *current conforming implementation* with its verified specifics.

**Why.** A spec should say what must be true, not what tool happens to be in use. Requirement-first tells a future implementer or a reviewer evaluating a substitute exactly what they must provide, and it makes the version-pinned specifics visibly contingent rather than load-bearing on the architecture. Implementation-first inverts this: it reads as "we use X," leaving the reader to reverse-engineer which properties of X are essential and which are incidental.

**Form.** A two-part subsection: a **Requirement** paragraph (the durable commitment), then **Current conforming implementation** (one satisfying instance, named and version-pinned). Worth also stating the *non*-requirements where a naive design would over-build, e.g. "the overlay need not provide durable storage, and need not provide a separate presence signal."

**Example.** The overlay section states the requirement as "an epidemic broadcast overlay: a swarm in which nodes forward along a self-healing spanning tree, no node holding global membership," then names iroh-gossip with HyParView and PlumTree as the current realization at pinned versions. The requirement is the stable thing; iroh-gossip is the current way to meet it.

**The vocabulary corollary: state the requirement in the design's own terms, and keep the realization's terms in the realization.** Requirement-first is not only about paragraph structure; it governs *which noun* a sentence uses. Where a sentence states what Drystone requires, the noun is the Drystone concept (a **Group** holds continuously-rotated key material shared among its members); where a sentence describes how the substrate provides it, the substrate's own term belongs (MLS realizes this via the **group**'s ratchet tree), and the mapping between them is stated. Letting the realization's term stand in for the requirement, "an MLS group holds..." where the requirement is meant, quietly pushes the requirement down into the realization layer, which is backwards: the reader can no longer tell what Drystone needs from what MLS happens to provide. The test per sentence: is this stating what Drystone requires (use the Drystone term), or describing the substrate's own object as such, e.g. quoting an RFC definition or a wire structure (use the substrate's term, transparently)? A related failure is treating a realization's *keying or configuration choice* as the model: the reference profile deriving one gossip topic per Group from the Group ID is a realization detail, and stating it as "scope is Group-specific" imports an implementation's one-to-one mapping into the model. Mark such choices as realization, and state the actual requirement (here: a high-entropy scope-topic seed, with the Group mapping configurable).

---

## 4. Carry a fixed, closed cast as an additive narrative spine, chained in an appendix

Introduce a small fixed cast once, up front, and grow the scenario on the same cast as each mechanism is introduced: "now suppose this same person is offline / on a local network with no internet / has two devices." Each named role carries a consistent trust meaning throughout.

**Why.** A running example lets the reader track *who knows what* as the design escalates, which is exactly the thing that gets lost in abstract prose about "a node" and "a recipient." It is the established technique in the security-protocol literature (the Alice/Bob/Carol convention from Needham-Schroeder onward) and in specs that walk one cast through escalating cases (RFC 9750 scenarios, TLS 1.3 walking one client/server through every handshake variant). The fixed cast also keeps the trust semantics honest: "Carol cannot read" means the same specific thing every time Carol appears.

**The cast carries trust, so keep the roles precise.** A persona is not a node: an actor is a person with standing, and that person has devices that are the nodes. Conflating the two muddies the very distinctions the design depends on. We had to correct "Carol is a node" to "Carol is a persona with devices that are nodes," which also kept her properly parallel to the in-group persona (both are people with devices; the only difference is membership). When the cast is also the vehicle for a structural point, getting the role layer right *is* getting the point right. So `actor` is reserved for a named member of the demonstrative cast, `persona` (plural `personae`) is the identity term the mechanics use, and `node` names a device or helper and never the person: a clause stating an identity mechanic says persona, and actor appears only where a beat lands that mechanic on a cast member.

**The cast is additive, never a replacement.** The normative text stands on its own: every definition and every MUST and MUST NOT is complete and checkable without the cast. The cast is a second layer laid over that text, a concrete instance that shows the mechanic in place. A beat never carries an obligation in the clause's stead; it shows the obligation biting. This matters most at the normative layer, because a MUST or MUST NOT is easiest to internalize as a consequence happening to someone ("Bob's laptop MUST verify Alice's signature before acting, because otherwise it acts on unauthenticated bytes"), so the cast is the default vehicle for making each normative clause concrete, set alongside the clause rather than in place of it. This is the demonstration companion to grounding (Rule 14): Rule 14 gives every clause its consequence in the abstract, and the cast shows that consequence landing on a named actor.

**The setup fixes the whole roster, players and resources both, as a closed list.** Define not only the personae and their devices but the resources the scenario turns on (the shared log, the delivery fabric, the threshold, any role set), so that every later beat draws only on what the setup established. The roster is a **closed list**: a beat **MUST NOT** introduce an actor or resource the setup did not name, because a cast that grows ad hoc per section stops being one journey and the chained view below no longer closes. Adding an actor or resource is therefore an amendment to the setup section itself, made deliberately and carrying its own worked-case role there, after which a beat **MAY** use it; a beat never enlarges the cast on its own. The list stays closed by construction and checkable by saying so: the setup section states that it is the whole cast, so a reviewer can check any beat's names against it, and a name new to a beat signals a setup change to make on purpose rather than a silent drift to miss.

**The same discipline binds the infrastructure cast, the non-adjudicating node roles.** A spec references not only the demonstrative personae but a vocabulary of node roles that carry, hold, and signal traffic without governing: a relay, a store-and-forward node, an overlay node, a notifier, a durability store, a read/search helper. Treat that vocabulary as a second closed cast. Name the whole set once, in its own section after the persona cast, giving each role a consistent meaning, the plane or layer it serves, and what it can and cannot see, and reference a role by name thereafter rather than re-describing it. State the invariant that groups them once (each is a scope participant holding a capability or offering a resource but no standing, and each is revocable), so a later section states a role's exceptions against a known baseline instead of re-deriving the baseline. This is the closed-list rule applied to infrastructure: a section **MUST NOT** introduce a node role the cast did not name, because an ad-hoc role is the same drift as an ad-hoc actor, and the fixed set is what lets every mechanism point at "the meer" or "the relay" and have the name mean one checkable thing. The persona cast and the infrastructure cast differ in kind, named characters versus role-types, so they live in separate sections: the personae first as the narrative spine, the node roles second as the vocabulary the mechanisms draw on.

**Chain the scattered beats into one journey in an appendix.** The per-section beats are, by design, scattered: each section touches the cast only for its own point, so read in isolation they are a disconnected narrative. Carry a single appendix that chains them into one continuous arc, from the setup through each mechanic in the order the scenario would actually unfold, and have each in-body beat point to it. Write the chained arc first, then thread its beats back into the sections as excerpts, so the beats stay mutually consistent rather than independently invented; the reader then gets both the local demonstration (in place) and the whole journey (in one read).

**Form.** Define the cast and its resources in a short section before the body. Then each mechanism opens with a concrete "now suppose" on that cast, before or beside the general statement, leading with the intuition the scenario gives and then tightening to the precise rule. A closing appendix chains the beats into the whole journey, and the maintained beat map (the cast and the section-to-beat table) is the working index that keeps the two in step, updated whenever a section gains or changes a mechanic.

---

## 5. Why, not just what

For each mechanism, state the problem it solves and the tradeoff it settles, not only how it works. Lead with the reason; let the mechanism follow.

**Why.** A reader can evaluate a design only if they can see what it is trading against what. "The push host is byte-free" is a fact; "the push host is byte-free because the rights model wants the smallest metadata footprint *and* the platform's silent channel is too throttled to depend on, so wake-then-fetch is both safer and more robust" is a decision the reader can judge. The second form also defends the choice against the obvious objection before it is raised.

**The test.** If a paragraph describes a mechanism without anywhere stating what would go wrong under the alternative, it is probably what-without-why. Add the one sentence that names the alternative and its cost.

---

## 6. Mark epistemic status; ground before stating

Every load-bearing claim carries its status: verified against a primary, to-be-confirmed, or the design's own synthesis. Pull the primary source *before* writing the claim, not after.

**Why.** This is the project's accuracy-before-fluency rule applied to prose. A claim written from memory and verified later is a claim that ships wrong if the verification slips. Marking status also lets a reviewer triage: a *Synthesis* line is the design's reasoning (judge it as reasoning), a *Verified* line is a sourced fact (check the source), a **[confirm]** is a known gap (do not treat as settled). Mixing these unmarked invites a reader to trust a guess as a fact.

**Form.** A short legend (Verified / [confirm] / Synthesis), then the tags used inline. Version numbers, dates, API names, and specific figures are never stated unmarked; if it cannot be sourced, it does not go in.

**The standing tag vocabulary for this suite, and the fold-normalization directive.** The canonical status-flag ladder is fixed once, in the conventions reference (A.9), and every part and companion normalizes to it so a reader learns one legend, not one per document. The ladder: ***Verified*** (demonstrated against real crypto or real transport in a running reference implementation), ***Verified-RFC*** (verified against a normative primary such as RFC 9420 or RFC 9750), ***Modeled*** (a reasoning-complete reference model, not yet backed by real crypto), ***Measured*** (an empirical or benchmark result), ***Established*** (an established result in the literature, or an inherited primitive used as-is), ***Design*** (specified but unproven, a decision the spec commits to), ***Synthesis*** (a claim assembled across several sources rather than one citation), ***Load-bearing, unearned*** (a property the design leans on that is not yet earned), **[gates-release]** (a byte-level encoding that must be pinned before a public release), and **[confirm]** (rests on an external fact not yet independently verified; do not treat as settled). Two linkage markers sit alongside and are not status flags: **Realizes: P-X** (a Part 2 clause discharging a Part 1 principle) and the section cross-references.

Directive for folds: a companion doc's verification tags are **mapped onto the A.9 ladder during the fold**, not carried in their original wording, and the mapping is stated once in the changelog where it is not one-to-one obvious. A single normalized legend across the suite is itself part of the accuracy discipline: two synonyms for one status let a reader wonder whether the difference is meaningful, so the fold collapses them. The earlier suite's *green-real* and the delivery doc's *Validated* both became *Verified*, and *green-model* became *Modeled*, during the P10 fold.

**The discipline in practice.** When a claim about a dependency's internals comes up, pull the dependency's own primary (its FAQ, its spec, its source) rather than asserting from training. We corrected a relay-visibility claim twice this way: each correction came from reading iroh's own docs, not from reasoning about what a relay "should" do.

**Quote the source, mark the synthesis, and never paraphrase as if the words were ours.** When a claim leans on an upstream source, represent that source in one of exactly two forms: a direct quote, or a direct quote followed by synthesis that is explicitly labeled as our interpretation. Silent paraphrase, restating the source's content in our own words with only a citation appended, is not used, because it fuses what the source said with how we read it, a reviewer can no longer separate the two, and the ground under the claim goes soft. The exact wording is what a reader checks against the primary; our reading is what a reader should be free to dispute, so the two are kept typographically distinct. Quote the **shortest span that carries the point**, which may be more than one span from a single source when each carries a distinct point; a quote that runs past a paragraph is a signal that we have not understood the point well enough to state it crisply, not a length the quote earned, so the response is to sharpen our own grasp rather than to quote more. Every quote is attributed to a precise locator, an RFC section, a page, or a named author. Applied instance: the MLS access-control position in Part 2 §5.5 quotes RFC 9750 §6.4's operative clause and marks the Group Role reading as Drystone's synthesis, rather than paraphrasing the clause into our own sentence.

**When the primary is reachable, verify; do not reach for `[confirm]` as a shortcut.** `[confirm]` marks a fact that genuinely cannot be checked from here, not one that is merely unchecked. If a primary can be pulled with the tools on hand, pull it and either confirm the wording, upgrading the tag, or correct it; flagging `[confirm]` when the source was reachable is the same accuracy-before-fluency failure as paraphrase, one step earlier. Applied instance: a verification pass pulled RFC 9750 (§6.3, §6.4), the Willow Meadowcap specification, the Spritely values page, and the GnuPG manual, replacing our Meadowcap characterization with the spec's verbatim definition and clearing four `[confirm]` flags that had been standing only for reachability.

**Collect every external primary in one references appendix, and make every citation resolve to it.** The document keeps a single references appendix (Part 2's Appendix F) that gathers every external primary it leans on, grouped by kind, so a reader finds a source in one place rather than reconstructing it from scattered mentions. Every inline citation resolves to an entry there, and every entry carries a precise locator: an RFC number and section, a paper's venue with page range and DOI, a specification's name and site, a crate's name with its version-status, or a platform's own documentation. The list carries locators, not status, because the same source can back a *Verified* claim in one section and a **[confirm]** in another, so the status tags stay on the claims at their point of use per the A.9 ladder above. The appendix is maintained as sections are written and reconciled in the final pass, when citation-to-entry resolution is checked in both directions. This is the natural terminus of the two rules above: the source-quotation rule and the verify-when-reachable rule govern how a source is represented and when it is checked, and the references appendix is where a reader goes to check it. Applied instance: Part 2's Appendix F collects the RFCs, the HyParView and PlumTree papers with their venues, pages, and DOIs, the Willow, Pkarr, and Matrix specifications, n0's iroh crates, the APNs and FCM platform docs, and the related-systems prior art, each under its locator.

---

## 7. One mechanism, named once, pointed at from its uses

When the same mechanism underlies several features, name and define it once, and have the several features point at it, rather than re-describing it each place.

**Why.** Re-describing one mechanism three times invites three subtly different descriptions that drift apart. Naming it once makes the shared structure visible (which is itself a design insight) and gives a single place to maintain. We unified C-swarm hole-detection, D-peer, and device-group sync under one named mechanism (gap-aware history convergence); each of the three now points at the one definition and states only what is specific to it.

**The reframing this enables.** Often the act of naming the shared mechanism *is* the clarification: realizing three features are one mechanism at different scopes is both simpler to write and a truer description than three parallel write-ups.

---

## 8. Separate the layers explicitly

Keep distinct what a protocol guarantees vs what an implementation chooses, and keep distinct adversaries or observers that sit at different layers. Do not let one word cover two layers.

**Why.** Conflating layers produces claims that are false at one layer and true at another, read as a single confused claim. We had "gateway or proxy" covering two different observers: a relay (iroh layer, sees EndpointId pairs transiently) and a gateway (IP layer, sees ephemeral IPs only). Splitting them by layer made each claim precise and actually *strengthened* the security story, because the two observers see almost disjoint things and resist combination. The conflation had hidden that.

**The test.** If an adversary or a guarantee is described with an "or" joining two things that live at different layers, split it. Name the layer each observes at; state what each sees and is blind to at that layer.

**Two more layer-conflations to check for, both surfaced by mis-stating a single word.** These are the same failure as the gateway/relay case, an infrastructure or authority concept described one layer up or down from where it lives:

- *The thing vs the use of the thing.* A shared infrastructure element (a store-and-forward node in a delivery scope) exists at a layer no single participant controls, while a participant's *reliance on* it is an individual, per-participant decision. Do not let one verb cover both: "dropping the node" (which a participant cannot do to shared infrastructure) is not "withdrawing use of the node" (which it can). We had "the persona removes the meer from scope," which over-stated a fabric-level fact as a per-persona act; the fix was to separate the meer's fabric-level scope presence (not individually revocable) from the persona's use-and-trust decision (individually revocable, as the exit exercised at the client). The test: when a participant "revokes" or "adopts" a piece of shared infrastructure, ask whether it is changing a shared-layer fact (usually it cannot) or changing its own behavior toward that fact (usually what is meant).

- *A resource vs a granted authority.* A device *facility* (storage, uptime, blind availability) is descriptive and needs no grant; an *authority* (permission to decrypt, to act, to admit) is granted and revocable. A word that treats a facility as if it were a grant ("revoke the availability role") smuggles the facility into the governance plane, exactly the slide the property vocabulary exists to prevent. We had "the Group delegated the availability and search-offload roles"; the fix separated blind availability (a resource, shifted by withdrawing use) from the read/search-offload authority (a grant, revoked through governance). The test: before calling something a role or grant, ask whether there is anything to *permit*; if the node can do it blindly with no key and no authority conferred, it is a resource, not a role.

---

## 9. Own the residual honestly

State what the design does *not* do, in its own words, at the point where a reader would otherwise assume it does. Do not let a strong claim imply more coverage than it has.

**Why.** A design that hides its limits invites a reader to over-trust it, and a reviewer who finds the unstated gap trusts the whole doc less. Naming the residual is both more honest and more credible. Where a property genuinely cannot be achieved (enforced deletion, defeating transport-level traffic analysis), say so plainly and say what *is* achievable instead.

**Form.** A non-goal stated where it is relevant ("this detects gaps below a known mark, but cannot reveal an author never heard from"), and a scope statement for whole classes of attack the layer does not undertake ("transport-level metadata is addressed, if at all, by lower-layer countermeasures this design notes rather than provides"). An "Open items" section at the end collects what is genuinely undecided, distinct from what is decided-and-bounded.

---

## 10. Close a requirement-mapping doc with a posture summary table

When a doc maps a design onto an external dependency it must satisfy or work around (a spec, a substrate, a protocol), close it with a single table that collapses each case to one row: what the dependency assumes or requires, the design's posture, and the forcing reason. One row per case, in the same order the body treats them.

**Why.** The body argues each case at length, which is right for the reasoning but wrong for recall: a reviewer returning to the doc, or a reader deciding whether a case affects their work, needs the whole mapping visible at once, not reconstructed by re-reading eight sections. The table is the doc's index and its consistency check in one. If a row cannot be written in a sentence per cell, the corresponding section is probably still carrying an unresolved tangle, so the table doubles as a test that each case actually reached a statable posture. It also makes a missing case obvious: a gap in the table is a gap in coverage, visible at a glance.

**Form.** A table near the end, before Open items, with a column for the case (cross-referenced to its section), a column for what the dependency assumes or requires, a column for the design's posture stated as an outcome not a discussion, and a column for the forcing principle or reason. Keep each cell to a phrase. The table states only decided-and-bounded postures; genuinely undecided threads live in Open items, not as a hedged table row. This keeps the "decided" and "undecided" registers cleanly separated, which is the same separation Rule 1 and Rule 9 draw at the paragraph level.

**Example.** The MLS hard-cases doc closes with a nine-row posture summary: each row names an MLS assumption (the DS orders commits; GroupInfo is authoritative to a rejoining client; no insider-replay protection), the Drystone posture (MLS is subordinate; GroupInfo is a claim corroborated against the governance chain; isolated by out-of-band convergence), and the forcing principle from Part 1. The table is the fastest way to see the whole MLS-to-Drystone mapping, and writing it surfaced that two cases (external-join and insider-replay) shared a resolution shape that the prose had treated separately.

---

## 11. Mechanical hygiene

Small rules that keep the prose readable and the references sound:

- **No em-dashes.** They are a weak narrative tool and easy to overuse; a comma, a colon, or a sentence break is almost always clearer. Check with a grep before considering a doc done.

- **Blank line between bullets**, and between distinct labeled lines (e.g. a `key: value` header line), for visual separation.

- **No run-on sentences.** Reading flow is a large part of clarity; break a sentence that carries more than one load.

- **Cross-references resolve, and disambiguate when numbering could collide.** This doc has its own §N; a referenced companion's §N is written "Part 2 §N" so the two never read as the same pointer. When content moves between sections, repoint the references that followed it.

- **Check render-space defects against the rendered output, not source lines.** A duplication or whitespace artifact that straddles a wrapped line is invisible to a line-oriented grep, because neither physical line carries the pair; markdown joins the soft newline into a space, so only the reader sees it. Any check whose defect can appear only after rendering (a doubled word, a doubled section reference such as "Part 2 Part 2", a duplicated token across a tag boundary) MUST run against the rendered HTML or a render-normalized copy of the source, and any claim about how a passage renders needs an actual render as its evidence, never an inference from a source search. This is grounded in a concrete miss: a real doubled "Part 2" in Part 1 §2.5 was flagged correctly, then cleared twice as a false positive on the strength of a source grep, and stayed live on the site until a render confirmed it (RUN-SPEC-CCC). The site build now carries this as a doubled-word gate over Part 1 and Part 2 (`site/build.py`, `find_doubled_words`); a genuinely repeated term is allowlisted with a reason.

- **Overturn a prior finding only with evidence at least as strong as raised it.** Downgrading a recorded finding to a false positive is itself a claim, so it carries the burden of proof, not the convenience of a quicker check. A weaker method than the one that raised a finding can silently erase a true catch and leave a false "never real" note in the record, which is worse than the original defect because it discredits the correct observation. If a finding was raised by reading the rendered page, do not clear it on a source grep; clear it by the same check or a stronger one, and record what was run.

- **Review against this method.** A consistency-clarity-correctness pass reads the whole doc against itself: do the Terms definitions match usage, do cross-references resolve, is any layer conflated, is there leftover interim baggage, does each mechanism state its why. The practices above are the checklist.

---

## 12. Name the sociotechnical alignment where a concept spans both planes

Drystone's whole claim is a convergence: it argues that established technical results, taken seriously,
force a humane shape for social governance. That means many load-bearing words carry an aligned meaning in
**two planes at once**, the social/epistemological reality (how human groups actually work) and its
technical mirror (how the protocol represents that reality on the wire). When a concept lives in both, the
doc must make the alignment explicit: state the social meaning as the layer-independent principle, then name
the technical construct as that principle *made mechanical with fidelity*, and say where, if anywhere, the
mirror is imperfect.

**Why.** This is distinct from Rule 8 (which splits two *technical* layers that a word wrongly conflates,
relay vs gateway). Rule 12 is the opposite situation: a word *correctly* names the same idea in two planes,
and the failure mode is leaving the correspondence implicit, so the reader cannot tell whether the technical
construct is a faithful realization of the social principle or a different thing wearing the same name. The
design's trustworthiness often *rests* on the fidelity of that mirror: a Group Role is trustworthy as a
model of delegated authority precisely because its removal-leaves-standing test reproduces the social test,
rather than inventing a divergent one. Stating the alignment is therefore not ornament; it is where the
"technical reality has fidelity to the social reality it represents" claim is discharged or exposed.

**The two planes to hold in view.** The **social/epistemological** plane: the timeless, technology-independent
reality (a right is standing whose removal forecloses contestation; a group is a body of people who
coordinate). The **technical** plane: the protocol's representation of it, bounded by what the mechanics can
actually guarantee (a capital-G Group; a Group Role; a lineage). The relationship is *manifestation*: the
technical construct manifests the social reality, and the doc names both plus the manifestation link.

**Form.** Lead with the social principle stated plainly and marked as layer-independent ("in any human group,
whether or not it runs on a protocol..."). Then introduce the technical construct as "that social X made
technical, with fidelity," and show the shared test passing on the wire. Where the mirror is imperfect, say
so at that point (the honest-residual discipline of Rule 9 applied to the alignment itself): e.g. the social
"one person" has *no* faithful technical mirror, which is exactly why persona and personhood are separate
words and the binding is left to group judgment.

**Case and qualifier as the marker.** Where the same word names both planes, typography carries which plane a
sentence stands on, the same genus/instance habit applied uniformly: lowercase **group** (social body) vs
capital-G **Group** (in-system principal); lowercase **role** (the social category of delegated authority)
vs **Group Role** (the concrete in-Group grant). The reader learns the habit once and applies it across every
spanned word. The authoritative list of these bounded terms, with the test for each, lives in the
conventions reference (Rule 13); this rule is why the list exists.

**The test.** If a sentence uses a term that has both a social and a technical sense, ask: is it clear which
plane the sentence is on, and if the sentence asserts the technical construct *does* something, is it clear
that this is the social principle realized rather than a separate mechanism? If either is unclear, name the
alignment.

The worked convention set for the largest such concept, the human-adjudication mechanism and its
planes-of-authority vocabulary, lives in the spec conventions doc (conventions-and-decisions.md A.11) and is
normative for all Drystone docs, experiments included.

---

## 13. Work from a fixed conventions reference and a per-part changelog

Two companion documents make the practices above checkable across a multi-document suite and across time,
and a pass should be run *against* them rather than from memory.

- **A conventions-and-decisions reference** (the terminology primer plus the synthesis decision record).
  It states every bounded-context term with the test for each (Rule 12's list), and records which
  definitions superseded which, so a later pass does not re-litigate a settled call or accidentally reverse
  a supersession. This is the reference the term-consistency pass reads against: a usage that fails a stated
  test is a slip to fix, and a term defined twice under two names is the specific thing it catches.

- **A changelog attached to each spec part.** Each Drystone part carries its own changelog recording what
  changed in it and why, pointing back to the conventions reference for the reasoning. Together, the part's
  changelog and the shared conventions reference explain everything that happened to that part: the changelog
  is the *what-changed-here*, the conventions reference is the *why-and-the-rule*. This keeps the design part
  itself an end state (Rule 1) while making its history fully auditable, the same two-documents-two-jobs
  split Rule 1 draws between a design doc and a session log, now made routine per part.

**Why.** A stated practice is checkable; a remembered one drifts. A suite this size, refined over many
passes, cannot stay consistent on vocabulary, supersession, and layer-discipline unless the rules and the
decisions live in a fixed place a pass can be run against. The conventions reference is to the vocabulary
what this method doc is to the prose: the thing you check for.

**Work in layered passes, each with one job, and record cross-pass obligations rather than reaching ahead
silently.** A large document is refined in layers (for the Drystone Part 2 synthesis: terminology alignment,
then technical-reality fold, then narrative-actor consistency, then requirement/realization consistency),
because trying to do them at once produces a document half-aligned on every axis and clean on none. Each
pass has one job and is recorded in the changelog as it completes. Occasionally a correction inside one pass
genuinely needs a concept that a later pass will formally introduce (a terminology fix that leans on a
mechanism not yet folded in). When that happens, make the correction (it should not wait), but **record the
cross-pass obligation explicitly** in the changelog: name the forward dependency and the duty it creates for
the later pass (usually "repoint or reconcile, do not redefine"). This keeps the layered discipline intact
even when a pass has to reach slightly outside its lane, because the reach is logged as a debt the right
pass will settle, not left as a silent inconsistency for a reader to trip on.

**Folding a companion doc in: reconcile, do not rewrite, and let a well-written source pre-pay the cost.**
When a self-contained companion document is folded into a larger spec (a delivery-architecture doc into the
transport section), the fold *slots* the material in rather than re-authoring it, and its job is four kinds
of reconciliation, not rewriting: (i) residual vocabulary drift against the conventions reference; (ii)
cross-references (the source's own `§X` becoming the host's `§X`, and the source's references to the host
becoming explicit); (iii) any cross-pass obligations the fold discharges; and (iv) the section map (Rule 15), creating or updating the map entry for any section the fold adds, moves, or repurposes. This is only cheap if the source
was written to the same discipline: a companion that is already requirement-first (Rule 3), already
self-defines its vocabulary (Rule 2) against the shared conventions, and already uses the shared running
cast (Rule 4) arrives *pre-reconciled*, so the fold is mostly placement and reference-fixing. That is the
practical payoff of holding every doc in the suite to the same method: the companion docs written that way
fold in almost mechanically, while a companion written in its own private vocabulary would force a
rewrite-in-place that is really a hidden re-derivation. Write companions as if they will be folded, and the
fold stays a fold.

One thing a fold must actively strip, not merely carry: a companion or working doc almost always contains
its own **change-scaffolding**, "an earlier version did X," "renamed from Y," "this used to be filed under
Z," refutations of a framing its own drafting once held. That scaffolding is legitimate *in the working
doc* (it records how that doc's thinking moved), but the destination is a published end-state spec, so
folding it in imports a Rule 1 violation. The fold's job includes converting every such note into the plain
positive statement of the current conclusion, or cutting it. Treat "does this sentence only make sense to
someone who saw the prior version?" as a fold checklist item, applied to the incoming material, not only to
the host. This is Rule 1 enforced at the fold boundary, and it is where end-state violations most reliably
enter, because the incoming prose was written to a different (auditable-history) standard than the artifact
it is joining.

---

## 14. Normative clauses carry their grounding, both MUST and MUST NOT

Every normative clause states *why*, and the requirement is two-sided. A **MUST NOT** names the concrete failure its breach causes; a **MUST** names what it secures, or what would be lost without it. A bare directive is not enough.

**Why.** A prohibition or requirement with no grounding is unauditable in the way Rule 1 and Rule 9 guard against elsewhere: a reader cannot distinguish an inadvertent breach from correct behavior by its symptom, and cannot reason about the impact of a misunderstanding, only register that a rule was broken. This is Rule 5 (why-not-what) applied with force to the normative layer, where the stakes are highest: a MUST or a MUST NOT is a load-bearing commitment, so leaving its reasoning implicit hides the very contrast the spec is built on, what the system requires and refuses, and what each buys.

**Both directions ground in the principles, which largely flow from Part 1.** A Part 2 MUST discharges a Part 1 imperative (often the one its section `Realizes`), so its why traces to that imperative rather than being invented locally; a MUST NOT excludes a failure those same imperatives forbid. A normative clause whose justification does not trace to a principle is a signal that either the clause or the principle is missing, which is itself information the review needs. This is the burden-of-proof division of the two-part spec made checkable at the level of the single clause: Part 1 argues the imperative, and a Part 2 normative clause names the imperative it discharges rather than re-arguing it.

**The test, two-sided.** For a MUST NOT, ask: what breaks, concretely, if a node does this anyway? For a MUST, ask: why is this required, what does it secure, and which principle does it serve? "MUST NOT X" fails; "MUST NOT X, because doing X causes Y" passes. "MUST Z" fails; "MUST Z, because Z secures W (Part 1 §...)" passes. A `[confirm]` may mark a clause whose grounding is not yet pinned. The authoritative statement of this rule for a given suite, with its worked instances, lives in the conventions reference (Rule 13, convention A.10); this rule is why that entry exists, the same relationship Rule 12 has to the bounded-term list.

---

## 15. Carry an annotated section map at the back of each part

Every library-resolution document carries a per-section **map** (`## 0. Map`): one line per top-level section and per major subsection, each snippet stating what the section covers, what it depends on, and what it is orthogonal to. The map is maintained as sections change, so it always reflects the current structure.

The map lives at the **back** of the document, with the index and reference matter, not at the head. The head carries a one-line pointer in the front-matter meta block (`` `Map: ...` ``) naming where the full map sits. The pointer exists chiefly for agent notice: a reader or agent entering at the top learns on the first screen that a section index exists and where to find it, without the map's half page displacing the document's opening prose.

The map keeps the name `0. Map` wherever it is placed, so references to "the §0 map" remain stable across placements. A section-internal map (`### 0. Map` inside a large section) MAY stay at that section's head when it is a few lines; the relocation rule is for document-scale maps long enough to displace the opening.

**Why.** A part large enough to be useful is too large to hold in the head, and a reader, a later fold, or a discussion returning to the doc needs to find the one section that bears on a question without reading the whole part. A bare table of contents gives the titles; the annotated map gives the *shape*: which sections are load-bearing for which, and, crucially, which concerns are orthogonal. Naming orthogonality is the point a plain contents list misses: a section can belong in a part yet be independent of most of it (scaling analysis and conflict-handling both live under governance, yet a change to one rarely touches the other), and stating that lets a reader or a fold pull the relevant section without dragging in the unrelated ones. The map is therefore both an index and a dependency sketch, and it makes the part's internal seams legible.

**Form.** A `## 0. Map` block at the back of the document, with the index and reference matter, after the last appendix or reference section, and a one-line `` `Map: ...` `` pointer in the front-matter meta block naming where it sits. One entry per top-level section, with major subsections nested, each entry a phrase for scope plus, where it matters, a `depends on:` note and an `orthogonal to:` note pointing by section number. Keep each entry to a line or two; the body carries the detail, the map carries the shape. The map is a maintained artifact, not a one-time index: any fold or edit that adds, moves, removes, or repurposes a section, or that changes a section's scope, dependencies, or orthogonality, **MUST** update the affected map entry in the same pass, and a fold or edit that leaves the map stale has not completed. This duty is binding on folds and editors specifically, because a stale map is worse than no map: it silently misdirects the next reader or fold to the wrong section, or hides a dependency it claims to index, and a wrong index is trusted where an absent one at least prompts a search. This is the maintained-in-place discipline Rule 13 draws for the conventions reference and the changelog, applied to structure.

**Why it earns its place.** This is the navigational analog of the conventions reference (Rule 13): where that makes vocabulary and decisions checkable across the suite, the section map makes *structure* checkable within a part, and a gap or a stale entry in the map is a gap or a drift in the part's organization, visible at a glance. It is also what keeps interlinking honest: every mechanism naming the section it depends on produces a web of cross-references, and the map is the index over that web, so the two together let a reader traverse the part by dependency rather than by page order.

---

## 16. Write at three resolutions, and make each lead reliably to the next

The suite is not one document at one depth; it is the same design rendered at three resolutions, so a reader can enter at the depth that matches the need and move between depths without re-learning the design. The three, from lowest resolution to highest: the **elevator pitch**, the **coffee shop**, and the **library**. They differ in length and grain, never in truth: a claim made at one resolution must hold at every other, so a pitch is the spec compressed, not the spec softened into something the spec would not endorse. Crossed with resolution is a second axis, **register**: a plain-spoken telling and a technical telling of the same content, most sharply separated at the elevator pitch, which exists in both a plain-spoken and a technical form. Resolution times register is why the suite has about six concrete pieces, but resolution is the organizing idea and register is a retelling within it.

**The elevator pitch** speaks mostly to outcomes and ties them back to the principles that produce them, in a few sentences, and it exists in both a plain-spoken and a technical form. Its burden is to stand alone for the reader who needs only it, so it **MUST** be accurate and concrete rather than a vague gesture, and it **MUST** lead reliably into the coffee shop or the library, so a reader who wants more knows where to go and finds the pitch's claims honored when they arrive. Needing only the pitch is a success, not a shortfall: a correct, concrete pitch that hands off cleanly has done its whole job.

**The coffee shop** is a one to one-and-a-half page telling that speaks to outcomes and principles directly and adds a broader view of the how alongside the what and the why. Its burden is to carry enough for grounded discussion and debate: a reader who has finished it can argue the design's merits and tradeoffs without the full spec open, because the shape of the how is present even where the mechanism detail is not. It is the level at which the design is contested, so it owes honesty about tradeoffs, not only outcomes.

**The library** is the full specification, the resolution this method doc mostly governs. It carries whatever verbosity clear communication requires and no more, and it is the complete reference. Its burden is to clarify and to ensure coherence: it is where any ambiguity the lower resolutions leave is resolved, and it is the level against which the pitch and the coffee shop are checked for truth. The library does not compress, because it is what the others compress from.

**Why.** Readers arrive with different needs and different budgets, and a single depth serves at most one of them: a full spec buries the newcomer, and a pitch alone can neither ground a design argument nor settle a coherence question. Three resolutions let each reader start where they are and descend only as far as the need requires, and a reliable hand-off makes descending cost nothing but more reading, never a re-orientation. What makes this safe is that the resolutions are one design at three grains rather than three separate documents: because the library is the coherence authority, the pitch and the coffee shop can be terse without going vague, since anything they leave out is recoverable in full one level down. The failure mode this fixes is the pitch that oversells or the summary that quietly drifts from the spec; here every resolution answers to the library, so compression stays honest and drift becomes a checkable defect.

**Form.** Author the library first, since it is the coherence authority the others compress from, and derive the coffee shop and the pitch downward from it rather than growing the spec upward from a pitch, so their claims are the spec's claims said shorter. Give the pitch both registers, plain-spoken and technical. Make each lower resolution end by pointing at the next, so the path from one sentence to the full mechanism is always a single visible step. Label each artifact with the resolution it occupies, so a reader knows at a glance the grain being read and where to go for more or for less. Re-check periodically that a claim at a lower resolution still holds at the library, and treat any divergence as a defect in the lower resolution, never a license to let the higher one drift to match.

---

## 17. Diagrams: author once, render to text and vector

A spec diagram is subject to the same text-first discipline as the prose: it must survive as plain text (searchable, terminal-legible, archival) and also render cleanly where richer output is available. Do not hand-maintain two independent drawings that drift; author one structured source and generate both forms from it.

**Why.** This is the RFC discipline. The classic RFC series was strictly plain-text ASCII, with packet headers, state machines, and topologies drawn character-by-character, identical on any terminal. RFC 7990 (late 2016) moved the series to an XML source published as HTML/PDF/EPUB, allowing embedded SVG, but kept the archival rule: any RFC carrying an SVG must also carry an ASCII-art equivalent or text description for the plain-text rendering. The text form is not a fallback that can lag; it is a first-class rendering the vector form is checked against.

**Why downscaling does not work.** There is no reliable structural conversion from SVG to ASCII. Most tools rasterize the SVG to pixels and map brightness to character density, which suits a stylized picture but destroys a technical diagram: crisp lines become muddy character soup. The root cause is the grid. An SVG lives in a continuous coordinate system with effectively infinite precision, while text art is bound to a chunky fixed grid (typically a 1:2 aspect), so snapping arbitrary vectors and diagonals onto that grid distorts and aliases (the algorithm guesses `/` versus `\` versus `.`, and often guesses wrong). The direction that works is the reverse: draft grid-aligned and upscale to vector. Text art is already grid-aligned, so upscaling to SVG is clean and predictable (this is the path the IETF Internet-Draft author resources take, via tools such as `aasvg` or `asciitosvg`, and grid-clean editors like Asciiflow or Monodraw).

**Mermaid is the practical source.** Mermaid is a structured text diagram language (`A --> B`), so tooling parses the logical structure rather than guessing at pixels, and the same source renders to both target forms:

- Mermaid to SVG: the Mermaid engine renders SVG in the browser, and the CLI `@mermaid-js/mermaid-cli` (`mmdc`) compiles a `.mmd` to SVG/PNG/PDF as pure, scalable, themable vector paths.

- Mermaid to ASCII/Unicode: tools intercept the Mermaid code before rendering, build a layout tree, and map nodes and arrows onto a monospace grid. Options include `mermaid-ascii` (a CLI and library, with a hosted endpoint that accepts the source over `curl`), Beautiful Mermaid (parses the Mermaid AST and emits styled SVG and structured ASCII from the one source), and `merman-ascii` (a Rust crate for deterministic ASCII/Unicode layouts aimed at logs, docs, and terminals).

**Form.** Author each diagram as Mermaid, or as grid-clean ASCII, and treat that source as authoritative. From it, generate two artifacts: a plain-text/ASCII form (satisfying the text-first, terminal-legible, archival requirement, matching the RFC discipline) and an SVG (for HTML rendering). Do not maintain the two forms by hand, and do not try to downscale an Illustrator or Inkscape SVG into text; the generation runs one way, from the structured source outward. This mirrors the three-resolutions discipline of Rule 16 at the level of a single figure: one authoritative source, several renderings, each answerable to the source rather than drifting on its own.

---

## How the practices relate

Most of these are one principle in different places. Self-defining terms (2), requirement-first (3), grounding before stating (6), owning the residual (9), and the posture summary table (10) are all "state the checkable conclusion, do not defer or imply." The cast (4) and why-not-what (5) are "give the reader the intuition and the tradeoff, not just the mechanism." End-state (1), one-mechanism-named-once (7), and layer-separation (8) are "remove the confusions that make a doc un-checkable: stale history, drifted duplicates, conflated layers." Sociotechnical alignment (12) is the positive companion to layer-separation (8): where 8 splits two things a word wrongly merges, 12 links two planes a word correctly spans, and both serve "one word, one clear referent per sentence." The fixed conventions reference and per-part changelog (13) are what make 2, 8, and 12 checkable across a multi-document suite and across time, the vocabulary analog of this whole method doc. Normative-clause grounding (14) is why-not-what (5) enforced at the normative layer: it makes every MUST and MUST NOT carry its consequence or its secured value, and grounds each in the Part 1 principles, so it is the burden-of-proof discipline of the two-part split made checkable clause by clause. The section map (15) is the navigational analog of the conventions reference (13): 13 indexes the vocabulary and decisions across the suite, and 15 indexes the structure and the orthogonality within a part, so both make interlinking traversable rather than merely present. The mechanical rules (11) keep the surface clean enough that the structural work shows through. Three resolutions (16) sits one level up from the rest: practices 1 through 15 govern how a single document is written and made checkable, and 16 governs how the documents relate, one design compressed to three grains with each answerable to the fullest, which is the coherence discipline of the conventions reference (13) and the section map (15) carried from within a part out to the whole suite.
