# Writing method for Drystone design docs

`Status: working method, descriptive`

`Scope: how the Drystone design documents are written and reviewed. Distinct from 08-experiment-methodology.md, which governs how claims are validated; this doc governs how a design is written down once decided.`

`Companion to: 01-delivery-architecture.md, which is the reference exemplar of the practices below.`

---

## Why this doc exists

The Drystone design docs are written to a consistent method that took several passes to settle. Capturing it serves two purposes: new docs start in the right shape rather than rediscovering it, and an existing doc can be reviewed *against* the method, since a stated practice is a practice you can check for. The practices below are not style preferences; each earns its place by fixing a specific failure mode we hit and corrected.

The single organizing principle, from which most of the rest follows: **state the conclusion, grounded, so the merge or the reader can check it; never gesture at reasoning, history, or definitions that live elsewhere.** A doc that defers cannot be checked for consistency. A doc that self-states can.

---

## 1. The doc is an end state, not a record of how the thinking went

State what was decided, as the conclusion. Do not narrate the path: no "an earlier design said X," no "corrected from," no "this was previously framed as," no refutation of a position the reader never held.

**Why.** The reader was not in the discussion. A refutation with no antecedent assertion is pure confusion: it raises a question (what was the old view? why are we arguing against it?) that the doc then has to spend words answering, all of it noise. Interim reasoning belongs in a session log or a decision record, not in the artifact that states the design.

**The seam to watch for.** If a sentence only makes sense to someone who saw the prior version, cut it or rewrite it to stand alone. "It is no more plaintext than any other group" was a real example we cut: it refutes a worry the reader never had, and the clean version simply states what the device group *is* (a Group moving sealed bytes) with no argument against a ghost.

**Where the history *does* go.** A session summary or decision-record doc (e.g. `00-session-summary.md`) is the right home for "why we changed our minds." Keep it there, so the design doc stays an end state and the history stays auditable, two documents with two jobs, neither muddying the other.

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

---

## 4. Carry a fixed cast as a narrative spine

Introduce a small fixed cast once, up front, and grow the scenario on the same cast as each mechanism is introduced: "now suppose this same person is offline / on a local network with no internet / has two devices." Each named role carries a consistent trust meaning throughout.

**Why.** A running example lets the reader track *who knows what* as the design escalates, which is exactly the thing that gets lost in abstract prose about "a node" and "a recipient." It is the established technique in the security-protocol literature (the Alice/Bob/Carol convention from Needham-Schroeder onward) and in specs that walk one cast through escalating cases (RFC 9750 scenarios, TLS 1.3 walking one client/server through every handshake variant). The fixed cast also keeps the trust semantics honest: "Carol cannot read" means the same specific thing every time Carol appears.

**The cast carries trust, so keep the roles precise.** A persona is not a node: an actor is a person with standing, and that person has devices that are the nodes. Conflating the two muddies the very distinctions the design depends on. We had to correct "Carol is a node" to "Carol is a persona with devices that are nodes," which also kept her properly parallel to the in-group persona (both are people with devices; the only difference is membership). When the cast is also the vehicle for a structural point, getting the role layer right *is* getting the point right.

**Form.** Define the cast in its own short section before the body. Then each mechanism opens with a concrete "now suppose" on that cast, before the general statement. Lead with the intuition the scenario gives, then tighten to the precise rule.

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

**The discipline in practice.** When a claim about a dependency's internals comes up, pull the dependency's own primary (its FAQ, its spec, its source) rather than asserting from training. We corrected a relay-visibility claim twice this way: each correction came from reading iroh's own docs, not from reasoning about what a relay "should" do.

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

- **Review against this method.** A consistency-clarity-correctness pass reads the whole doc against itself: do the Terms definitions match usage, do cross-references resolve, is any layer conflated, is there leftover interim baggage, does each mechanism state its why. The practices above are the checklist.

---

## How the practices relate

Most of these are one principle in different places. Self-defining terms (2), requirement-first (3), grounding before stating (6), owning the residual (9), and the posture summary table (10) are all "state the checkable conclusion, do not defer or imply." The cast (4) and why-not-what (5) are "give the reader the intuition and the tradeoff, not just the mechanism." End-state (1), one-mechanism-named-once (7), and layer-separation (8) are "remove the confusions that make a doc un-checkable: stale history, drifted duplicates, conflated layers." The mechanical rules (11) keep the surface clean enough that the structural work shows through.
