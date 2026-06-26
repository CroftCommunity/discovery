# Beta 01 review → Drystone protocol spec (2026-06-26)

**Status:** APPROVED — building. Scope expanded beyond a prose refinement (see "Pivot" below).

## Pivot (2026-06-26, user-directed)

The review started as "refine `beta/01`," but in dialogue the user reframed the target: **01's content
becomes Part 1 of a new beta Drystone protocol spec** — a concrete, build-against, certify-against
document, in the format of the alpha `thinking/drystone-spec/` scaffold. Two parts:

- **Part 1 — Reasoning Underpinnings:** the named design principles (the "why"), each ending in the
  consequence the mechanics MUST satisfy. **This is what refined 01 provides** — and it fills a real gap:
  the alpha spec drafts already *reference* `P-Local-Truth`, `P-Knowable-Truth`, `P-Peer-Equality`,
  `P-Durable-Enablement` by name, but **the §1 that defines them was never written** (ROADMAP E30).
- **Part 2 — The Certifiable Design:** the normative mechanics, matured from the proven
  `crystallized/CROFT-PROTOCOL.md` + the two alpha drystone-spec section drafts
  (`section-2-peers-rights-capabilities.md`, `section-x-governance-conflicts.md`), carrying proof /
  `ENABLING` flags. `04 — the protocol we proved` stays untouched (it is the narrative of the design
  *process*, not a spec).

The review refinements below are **not discarded** — they are the distillation guide for turning 01's
philosophical prose into Part 1's named, mechanic-bearing principles (cut Socrates/Peirce, node-not-
system, lead-with-grounds, define terms, deflate orphaned aphorisms — exactly the moves the skeleton
asks for: "Resist adding principles that do not cash out into a mechanic").

**Confirmed decisions (2026-06-26):** (2) cut Socrates + Peirce; (3) stage Ashby/Beer as an OPEN-THREADS
entry — do **not** fabricate replacement quotes (new Beer transcript pending from the user); (4) relocate
Hush-A-Phone/Bazelon to a new alpha **historical peer-rights** doc (not Croft-specific); (5) move the
review transcript to `beta/thinking/raw/` + register `preserved-verbatim` in the manifest.

**Integrity guardrail:** Part 2 carries the proof flags verbatim and does **not** invent the byte-level
`ENABLING` encodings (canonical fact encoding, frontier-closure, frontier-commitment, capability wire
format) nor assert the unverified Matrix/Willow/Meadowcap/Keyhive facts — those stay flagged for
confirmation and the forming bits live in Appendix B. iroh facts cite the FACTCHECK SoT (iroh 1.0).

## Original problem statement (the review)

The user voice-reviewed `beta/01-epistemic-foundation.md` top-to-bottom and dictated refinement notes
"for Claude refinement" (`beta/thinking/01_beta_review.txt`, ~298 lines). The transcript is
stream-of-consciousness speech with transcription errors, false starts, and the same note circling back
several times. The classified extraction below turns it into a structured, de-duped change set tied to
specific 01 locations — now repurposed as the Part 1 distillation guide.

## Reasoning

The review walks narrative → §1 → §2 → §3 → §4 → §5, and **stops mid-sentence at §5's four rights
("…and share.")**. §6.1–6.3 (collectives, capabilities-not-rights planes, recurring inversion,
centralization-capture) are **essentially un-reviewed** except (a) the user *likes* the
"equal possibility of all shapes" lift, and (b) the "define collective/federation" concern reaches into
§6. So this pass refines the narrative-through-§5 spine; §6 changes are limited to consistency items the
review explicitly raised.

The dominant signal is not many small edits but a few **recurring structural reframes** (node-not-system;
lead-with-grounds; real-quotes-not-paraphrases; define-your-terms). Most individual line notes are
instances of those.

---

## Normalization applied (transcription read-throughs)

`Drystone` (← "dry Stone"); `equal in rights` (← "equine/equal inner rights"); `all shapes`
(← "octave shapes"); `principled boundary` / `in parentheses` (← "one print / in prints / one
principal"); `none` (← "nun"); `provenance` (← "products"); `algedonic` (← "alginic"); `dissenter`
(← "descender/Decenter"); `Bazelon` (Hush-A-Phone). `[Speaker 1]` tags and false starts stripped.

---

## Cross-cutting / recurring items (the spine of the change set)

These recur across many paragraphs; each line-note below that is an instance points back to one of these.

| ID | Class | Item | Where it lands |
|---|---|---|---|
| **X1** | **S** | **Node, not the system, is the actor.** "a distributed system can only establish provenance" → "**a node in** a distributed system…"; the system *facilitates nodes having the necessary view*. Recurs in narrative, §1 (×3), §2.2, §2.3, §2.4, §2.5, §5. | narrative, §1, §2.2–2.5, §5 |
| **X2** | **C/S** | **"distributed system" vs "network" intermixed.** Make consistent (prefer node/distributed-system phrasing; drop "the network reconciles"); add a one-line definition: *a distributed system = nodes communicating remotely about some common truth; truth is local and corroborated, never certified from a center.* | narrative, §2.3, §3 code block |
| **X3** | **S** | **Lead each §2.x with the Croft grounding, then the outside quote as the tie-in, then walk it out.** Sections currently lead with the quote. The *grounds* are the top-line item. | §2.2, §2.3, §2.4, §2.5 |
| **X4** | **C** | **Don't present paraphrases as quotes.** Pull a *real verbatim* quote (the user believes better ones exist in the alpha transcripts) or cut — applies to the Ashby gloss, the Beer paraphrase, and the un-quoted Scott case. Then explain the real quote. | §2.5 |
| **X5** | **S/C** | **Frame 01 as the epistemology of *Drystone*** — the philosophical underpinnings that lead to the protocol. (`Drystone` is already a beta-level term in 02/03/04/07/08, so this is tier-clean; 01 just hasn't named it yet.) | narrative, §3, closing |
| **X6** | **D** | **Cross-reference style** — bare "`04` refuses…" reads awkwardly as an actor. Want a short *named principle* (e.g. "the no-adjudication rule (`04`)"). Doc-wide / cross-theme → defer the global convention; for 01, propose local named-rule phrasings. | doc-wide (defer) |
| **X7** | **C/S** | **Define "collective" and "federation" before use.** Currently used as undefined "concrete pronoun-waves" in §3, §5, §6 — either introduce them (groups of peers acting as a collective; federation = local-first at collective scale) or defer. | §3, §5, §6 |
| **X8** | **S/W** | **Deflate orphaned transcript aphorisms.** Lines that "made sense in a transcript" but read opaque stitched together ("there was only ever one thing being said"; "earns belief the way it describes belief being earned"). Clarify or cut. | §3 |
| **X9** | **C/S** | **De-emphasize "2,400 years"** and **de-duplicate §2.6.** Duration is not the point (one node right against all others doesn't depend on how long); §2.6's three-part conclusion repeats earlier statements. | §2 title, narrative, §2.6 |

---

## Refinement notes in document order

Legend — **(W)** wording (apply inline) · **(S)** structural reframe · **(C)** content/claim change
(confirm before applying) · **(F)** formatting · **(D)** defer to separate pass/doc. "→X#" = instance of
a cross-cutting item.

### Narrative (overview)

- **R1 (S → X5).** Frame the theme as the philosophical underpinnings of **Drystone** that lead to the
  protocol. *Proposed:* one clause in the opening narrative naming Drystone as the protocol the
  epistemology grounds.
- **R2 (S → X1).** "a distributed system can only ever establish *provenance*… never *truth*" → "**a node
  in a distributed system** can only…"; add that the system *facilitates nodes having the necessary view*.
- **R3 (C/S → X2).** "Truth is structurally outside a **network's** reach" + "never certified from a
  center" — reconcile network/distributed-system usage; add the one-line definition of a distributed
  system. **Confirm wording.**
- **R4 (W, flag).** "That is the razor." — user: "I don't know what that means exactly; you might want to
  remove it." *Proposed:* keep but only if it earns its place; candidate cut. **Confirm.**
- **R5 (W).** Drop "**none arguing with the others**" in the narrative convergence line ("that's weird").
  (Also check the sibling phrasing in §2.6, R34.)
- **R6 (C, flag → X2).** "name-resolution" in the premise-propagation list — user unsure it belongs;
  reframe to *hierarchical naming / point-of-presence* or cut. **Confirm.** (Pairs with R28, R30.)
- **R7 (W/F).** "the negative space is the design" → "the negative space is an **intentional and critical
  part of** the design." (Also §4, R26.)
- **R8 (F).** Lift "**no right to remove the rights of others**" and the "design the conditions…" line
  **out of parentheses** — they are legitimate call-outs, not asides.
- **R9 (S/C → "one boundary" reconcile).** "the **one** principled boundary" — user: there is *more than
  one* (the rights set defines a peer; plus the no-removal rule; plus inverse-correlation). Reframe:
  several **rights** (positive), one **negative** boundary (no removal). **This is the K17/§5/T21/T22
  zone — reconcile, see below. Confirm framing.**

### Charter

- **R10 (S → X1).** In-scope bullet "truth as structurally out of reach for a distributed system" →
  node-framing. (Mechanical, rides X1.)
- *(No-op note: user observed the charter "is just enumerating" — not actionable.)*

### §1 — The razor

- **R11 (S → X1).** "A distributed system sees two things" → "**A node** sees two things"; tie to **the
  rights-floor that defines a peer** (to be a peer, these rights must hold). **Confirm the rights-floor
  link here vs. keeping it in §5.**
- **R12 (C/S → X1).** The boxed design imperative — "**The system** establishes what is consistent and
  corroborated; humans supply what is true and right" → "**nodes in the system** establish what is
  consistent and corroborated **through interaction with the system**…". Edits a highlighted central
  claim. **Confirm.**
- **R13 (W).** "Provenance is a closed, determinate question — **does the chain verify**." User: "I don't
  understand that sentence." Clarify / expand.
- **R14 (W/S).** "A system that tried to certify truth would encode one conclusion…" → recast affirmative
  & general: "**systems that attempt to certify truth must encode** fixed conclusions (often one) as
  objective fact and marginalize **all other nodes** who corroborated different local state."
- **R15 (C).** "…including the dissenters who **turn out to be right**." User flags the tension: if truth
  is undiscernible, "right" can't mean objectively-right — it's *the node later judged highest-utility by
  humans*. Firm up to remove the implied single-truth. **Confirm.**
- **R16 (S/F).** "The honest mission is not truth but **trustworthy disagreement**." User: "that line is a
  banger — should be its own thing." Elevate / feature it.

### §2 — The convergence

- **R17 (S/C → X9).** Title "five disciplines, one claim, **2,400 years**" — de-emphasize the time-span
  (duration isn't the point). **Confirm how far to pull it back** (keep "five disciplines," soften
  "2,400 years").
- **R18 (W).** "Thinkers who never collaborated… arrive at pieces of the same conclusion." — like the
  point, "a little weirdly said." Reword.

#### §2.1 Ethics

- **R19 (C).** **Cut the Socrates quote(s).** "Too broad / hand-wavy… you could call in Buddha and
  beginner's mind… the case is strong enough without reaching that far." (Was *Verified* — removal has
  verification + ledger implications; ethics survives via Mill.) **Confirm cut.**
- **R20 (S).** **Lead §2.1 with Mill** — "the moral spine of *let the dissenter fork*… we buried the
  lead." Pairs with R19.
- **R21 (W).** Grounds "**Silencing is theft from the present and from posterity**" — "comes across as
  hand-wavy"; make the point a different way.
- **R22 (W/C → X6).** Rework the `04`/`06` tie as a clearer lead-in: *the protocol refuses to
  algorithmically adjudicate social disputes and refuses moderation-as-surveillance; instead it lets
  nodes set their own relationship to moderation and adjudication outcomes.* Optionally qualify
  "surveillance" (un-agreed-upon). Name the rule rather than "`04` refuses" (X6).

#### §2.2 Epistemology

- **R23 (S → X1).** "certainty is unreachable" → "**certainty for a node in a distributed system** is
  unreachable" (a node can't know what else is happening; works only from its own view).
- **R24 (C).** **Peirce "Do not block the way of inquiry"** — user dislikes it, "too hard to understand at
  this point in the reading." Candidate cut/replace (lower-confidence than Socrates). **Confirm.**
- **R25 (C).** Grounds "truth is what a community of inquiry converges toward over unlimited time" — "still
  makes it seem like there's one truth." Reword to differentiate **corroborated reality** (what we can
  establish) from **truth** (what we cannot possess). **Confirm.**
- **R26 (W).** "gossip-convergence with humility built into the math; the system must never foreclose
  revision" — confusing; reword.
- **R27 (S/W).** Popper quote *liked* — keep. Optionally strengthen its grounds with the node-ignorance
  framing ("a node has infinite ignorance about what other nodes are doing; accepting that lets us design
  for real conditions"). Also (S→X3): the "theories are never verified, only corroborated by surviving
  refutation" + "provenance is exactly what a never-verified epistemology can honestly compute" is the
  **heart-and-soul**, currently buried — elevate; and **simplify** the "mouthful" sentence (W).

#### §2.3 Economics

- **R28 (W).** Title/gloss "the knowledge a center would need cannot be centralized" — reword the awkward
  gloss.
- **R29 (S → X3).** Lead with the grounding, then the Hayek quote (which the user *loves*), then exposition.
- **R30 (S → X2).** Grounds "the **network's** actual condition" → "a **distributed system's** actual
  condition."
- **R31 (W, low).** "decisions belong at the edges" → "decisions **and value judgments** belong at the
  edges." Low-value.

#### §2.4 Political science

- **R32 (S/W → X1).** Gloss "lowest jurisdiction" = **the edge nodes** (in a P2P system, all nodes are
  edge nodes). Add node-framing. Lead with "keep governance local where consent is cheap; federate only
  the irreducible minimum" (the top line), then Ostrom, then "why necessary, not optional" (S→X3).
- **R33 (verification, no resolve).** User reaffirms the standing gate: the subsidiarity passage is the
  2013 generalization, **pull the primary text** before citation. Keep the verification flag; **do not
  resolve** (it is the existing Ostrom decision-gate).

#### §2.5 Systems science

- **R34 (S/W → X1).** "plurality is a survival condition, not a preference" → "**plurality is a necessary
  condition for node health in a distributed system where nodes are equal peers** (with naturally
  diverging views of what's corroborated vs asserted)."
- **R35 (C → X4).** **Ashby** — "Only variety can destroy variety" too terse standing alone; the second
  block is a **paraphrase/gloss, not verbatim**. Pull a better real Ashby quote (user believes one exists
  in the transcripts) or cut the gloss. **Confirm / needs an alpha-transcript pull.**
- **R36 (W/C).** Grounds "a regulator that collapses plural perspectives…" — gloss it concretely (a
  central store of "pretend-canonical" truth) and add the sharper consequence: it doesn't just get
  "overwhelmed by reality," it **prunes/removes the variety it can't represent.**
- **R37 (S → X3).** "Preserving plural perspectives… is requisite variety made architectural. *Plurality
  is the robustness.*" — *loved*, currently buried; elevate.
- **R38 (C/S → X4).** **Beer** — "jumping right into the weeds" (attenuate/amplify/algedonic) without
  exposition; the Beer block is a **paraphrase** — replace with a real verbatim quote, then explain.
  **Confirm / needs an alpha-transcript pull.**
- **R39 (W/S).** **Scott monoculture forest** — simplify; the trees=nodes metaphor over-extends. State
  plainly: a complex distributed system reduced to a one-commodity machine becomes brittle; legible
  optimization from the center prunes the variety reality needs. Tie the analogy to "set the conditions
  for variety, don't create it." Move the grounds higher (S→X3).
- **R40 (C/F → X4).** **Scott "Seeing Like a State" is referenced but not actually quoted**, unlike the
  other grounds — the asymmetry is noticed. Decide: add a real quote or keep the deliberate no-quote
  treatment (the doc already justifies it). **Confirm.**

#### §2.6

- **R41 (C/S → X9).** User lukewarm on "why the convergence is itself the proof" ("a bit like a
  preponderance argument"), and its three-part conclusion **repeats** earlier statements — de-duplicate /
  trim. **Confirm scope of trim.**

### §3 — Local-first is the same sentence as the epistemology

- **R42 (W).** "the engineering premise and the epistemological premise are the *same sentence* said
  twice" → "the **same concept restated**."
- **R43 (W/C → X8).** "the design *earns belief the way it describes belief being earned*" — liked but
  "I don't have a clue what it means"; clarify. "**there was only ever one thing being said**" — too
  confusing; cut or clarify (orphaned aphorism).
- **R44 (S → X5).** The premise-propagation list is "where we get into **Drystone**" — name it.
- **R45 (C/S → X7).** Federation bullet introduces "**federation**" and "**collective**" without
  defining them — define (groups of peers acting as a collective; federation = local-first at collective
  scale) or defer. (Same item resurfaces in §5, R52.)
- **R46 (W → X2/X6).** "Even **resolution** (DNS) is local-first applied to dependencies" — user dislikes
  the line; cut or rework. (Pairs with R6.)
- **R47 (S/W).** Two-part theorem — reorder/reframe: lead with "**the person, the arbiter of utility, is
  the unit of composability**," then contrast central primary state making "the person **and their
  variety** secondary."
- **R48 (W, low).** Friction passage — minor clarifications (the center is bounded, can't represent all
  states; honest vs manufactured friction). Mostly affirms existing text.

### §4 — Design for the conditions

- **R49 (W).** "the enabling set is mostly restraints, **almost no features**" — "almost no features" is
  confusing; reword.
- **R50 (W/C).** "**accessible resolution that defers judgment**" — unclear, clarify. "a single legible
  **value**" → consider "a single legible **perspective/state**."
- **R51 (W).** "you can be corrected observably **when the variety dies**" — confusing sentence; reword.
- *(R7/X-instance: "the negative space is the design" reword also lands here.)*

### §5 — The one principled boundary

- **R52 (S/C → X7/R9).** Title "**The one principled boundary**" — reframe per R9 (one *negative* boundary
  vs several *rights*). And **define "collective"** (used in "not the collective's to remove") — same
  define-your-terms item as R45. **Confirm framing.**
- **R53 (W).** Wolf test "any action that, generalized, would remove the conditions of its own
  contestation… the tell is self-cancellation" — liked but confusing; clarify.
- **R54 (W).** "the **monoculture mechanism in a polity**" — unclear jargon; reword/explain.
- **R55 (W/S → X1).** "the rights-floor is a consistency and equality requirement, not a moral overlay" —
  strengthen grounds with node-framing ("equal nodes must represent divergent views; a node proxies a
  digital human experience").
- **R56 (W).** "constitutional rigidity **bites hardest**" — disliked; reword (e.g. "is required for
  fidelity"). The inverse-correlation line itself is *liked* — keep.
- **R57 (W).** "a right is standing that must survive for any dispute **about it** to stay contestable" —
  confusing (what does "about it" refer to); clarify while preserving the K17 meaning.
- **R58 (W/C, low).** Capability definition — optionally align vocabulary with the role/delegation terms
  used elsewhere (capabilities = things you can do; roles = delegated authority to exercise a capability
  on a principal's behalf; principal can be group/person/device). Mostly affirming.
- **R59 (W/S → X1).** "rights are local-first state the person holds and cannot be reached into" —
  confusing (rights defined as *standing* earlier); reword to "protect the **standing** of local-first
  state the person/node holds, which cannot be reached into / inspected directly by other nodes."
- **R60 (positive + open-threads).** The four rights "each fixed by what their removal would foreclose" —
  *liked*. Two open questions the user raised map to **T22** ("what exactly adds up to tenure" — survivor
  re-key) and **T21** (`share`, where the transcript ends). **Do not resolve — reference T21/T22.**

### §6 — un-reviewed (transcript ends at §5)

- **R61 (positive).** "equal in rights, not capabilities → the **equal possibility of all shapes**" + the
  variety point — the user *likes the lift*; keep / consider strengthening. (Only §6 note in the review
  besides the X7 collective/federation definition.)

---

## K17 / T21 / T22 reconciliation (do not duplicate, do not resolve)

- K17 (folded 2026-06-26) already put the **rights-vs-capability discriminating test**, the **four rights
  (tenure/exit/voice/share) fixed by what removal forecloses**, and the **voice-vs-amplification cut**
  into §5. The review's "we have to define what the rights are" (R9, the "more than one boundary" note)
  is **largely already addressed by K17** — so R9/R52 are a **framing tweak** (several rights vs one
  negative boundary), *not* new content. Reconcile against the existing §5 paragraph; do not re-add the
  definitions.
- **T21** (`share` fully a right vs membership-class capability) and **T22** (does survivor re-key strand
  `tenure`) are the two open checks. The user re-touched both (R60). **Reference them; do not re-open or
  resolve** — they gate hardening the four-rights closed set into the Drystone spec.

---

## Decision-gates surfaced (the user decides; this plan does not)

1. **Drystone framing (X5).** Recommend: yes — introduce "Drystone" into 01 as the protocol the
   epistemology grounds (already a beta-level term elsewhere; tier-clean). *Confirm.*
2. **Cross-reference naming convention (X6).** A doc-wide "named principle instead of `0N` refuses"
   change touches all beta docs and needs a name per rule. Recommend: **defer the global convention** to
   a separate cross-theme pass; for 01, apply local named-rule phrasings only. *Confirm.*
3. **Hush-A-Phone / Bazelon relocation (R-narrative + §5).** User proposes it may belong in a separate
   *historical-alignment* document, not the reasoning-foundations doc. Recommend: **defer-or-relocate**,
   not silent delete; if it moves, the new doc lands in `alpha/` (new thinking) until matured, and the
   move is recorded in `BETA-ROLLUP` (K-folded ancestor — ledger implication). *Confirm: keep in 01,
   stub a new alpha doc, or hold?*
4. **Content cuts (C):** Socrates (R19, recommend cut — ethics survives via Mill), Peirce (R24, weaker
   case), the Ashby gloss / Beer paraphrase (R35/R38 — need a real quote pulled from the alpha
   transcripts, or cut), Scott quote asymmetry (R40). *Confirm each.*
5. **Provenance filing (PLAYBOOK + manifest).** `beta/thinking/01_beta_review.txt` is an unregistered raw
   transcript. Recommend: canonical raw home **stays `beta/thinking/`** (it reviews a beta doc and is a
   beta-gate working artifact), registered in `RAW-ARTIFACTS-MANIFEST.md` as `preserved-verbatim`
   (it is an exact file, not a cleaned chat paste). No second copy under `alpha/seeds/transcripts/raw/`
   unless you want the usual raw-dialogue convention mirrored. *Confirm home + status.*

---

## Apply order & verification (post-approval only)

1. Apply **W/F** inline (low-risk): R5, R7, R8, R13, R18, R21, R26, R28, R31, R42, R46, R49, R50, R51,
   R53, R54, R56, R57, R59, R31.
2. Apply approved **S** reframes: X1 (node-not-system sweep), X2 (network/distributed-system + the
   one-line definition), X3 (lead-with-grounds in §2.2–2.5), X5 (Drystone framing), X7 (define
   collective/federation), X8 (deflate aphorisms), R9/R52 (boundary framing), R16/R27/R37 (elevate the
   buried lines), R47 (two-part theorem reorder).
3. Apply approved **C** changes (highest care): R3, R6, R12, R15, R19/R20, R24, R25, R35, R38, R40, R41.
4. Record **D** items: X6 (cross-ref convention pass), Hush-A-Phone relocation if approved.
5. **Verify tier-cleanliness:**
   `grep -rEn "\.\./alpha/|COHESION §|## Sources \(alpha\)|## Provenance trace" beta/0*.md` → must be empty.
6. **Ledger:** add a "01 review refinements 2026-06-26" note to `alpha/BETA-ROLLUP.md` for substantive
   (S/C) changes (esp. any Socrates/Peirce cut, Ashby/Beer quote-pulls, Hush-A-Phone move); add a
   `COHESION.md` seam entry if a change opens/closes a loose end.
7. **Manifest:** register `beta/thinking/01_beta_review.txt` per the filing decision (gate 5).
8. **No gate resolved; nothing committed unless asked.**

## Definition of done

Every transcript note extracted, de-duped, classified, tied to a 01 location (above); plan approved;
approved W/S/C/F applied with tier-cleanliness re-verified; D items recorded; K17/T21/T22 overlap
reconciled without duplication; transcript registered; no gate resolved; nothing committed unless asked.
