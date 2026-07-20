# Run instructions: land the human-adjudication language codification

`Status: operator brief for Claude Code. Docs-only pass. Source of decisions:
human-adjudication-language-survey-and-proposal.md (2026-07-13, in the chat
workspace; the durable content is inlined below so this file is self-contained).`

## Ground rules for this run

1. Work on a fresh branch off `main` named `docs/human-adjudication-language`.
2. **Docs only.** No `.rs`, no code, no test files. Do not rename test functions
   or code identifiers (e.g. `hard_stops` in test names stays).
3. **Verify every anchor before editing.** The anchor strings below were taken
   from a 2026-07-13 snapshot; small refinements may have landed since. If an
   anchor is missing or ambiguous, do NOT approximate: log it in the run summary
   under "skipped: anchor drift" and move on.
4. One commit per edit block (E1, E2, ...), message prefixed `docs(adjudication):`.
5. Every touched Part 1 / Part 2 section requires the matching **back-map**
   entry update (the "0. Map" section at the back of each part, per Rule 15) and
   a **changelog** entry (part-1-changelog.md / part-2-changelog.md, per Rule 13).
   Do these inside the same commit as the edit they describe.
6. Quotes in corroboration entries must be **verbatim from the primary**, fetched
   during this run (URLs given). If a primary cannot be fetched, insert the entry
   with the quote slot marked `**[confirm-verbatim: fetch primary]**` rather than
   paraphrasing into quote marks.
7. Finish with `RUN-SUMMARY-adjudication-language.md` at repo root of the branch:
   what landed, what was skipped and why, anchors that had drifted.

## The vocabulary being codified (context, not an edit)

The specs already carry a consistent vocabulary for the mechanism by which
questions the protocol cannot answer (utility questions) are surfaced to the
humans who can. This run promotes it to a named convention set and repairs
drift. Canonical terms:

- **the seam**: provenance fully settled, utility still open (the razor's seam,
  Part 1 §2.0). When the mechanism applies.
- **the recognizer**: the governed classifier of Part 2 §7.4.1
  (benign-versus-dispute tolerance; per-Group; never a constant; ambiguity MUST
  escalate).
- **the event**: the **reconcile hard-stop** (Part 2 §7.6). Prose spelling:
  hard-stop, hyphenated, noun and verb.
- **the shapes**: **Contradiction** (too many valid claims) and
  **Under-determination** (too few valid claims). Code:
  `ForkStatus::Contradiction`, `ForkStatus::UnderDetermined`.
- **the channel**: the **algedonic channel** (lineage name, Part 1 §3, Beer);
  operationally, **escalation**.
- **the deliverable**: the **legible picture**: a grounded statement of the
  conflicting facts in governance language, full provenance, no editorializing,
  identical on every node.
- **the receiving side**: the **local authority**: the principal for its own
  judgments, the affected Group for shared ones.
- **planes of authority**: infrastructure authority (held by *operators*:
  relays, helpers, recovery services; legitimate and necessary) and
  social-utility authority (held by *local authorities*). Separated by design
  with no path from one into the other, like mutually exclusive Group Roles.
  This extends the existing plane idiom of conventions A.4/A.5 and the
  non-adjudicating node-role cast of A.7.
- **the outcome space**: the three registers (mute, governance, fork); the
  re-formation fork; re-plant; fork-not-verdict as the doctrine.
- **the boundary sentence** (reusable pattern, from §7.6.1): the protocol
  supplies the conditions for cheap human resolution, not the resolution; the
  judgment stays with the humans; the instantiation of whatever they choose is
  Drystone's.

---

## E1. conventions-and-decisions.md: add section A.11

File: `beta/drystone-spec/conventions-and-decisions.md`

Anchor: insert a new `### A.11` immediately after the end of section
`### A.10 Normative clauses carry their grounding (both MUST and MUST NOT)`
(i.e. before `## Part B. Synthesis decisions (the record)`).

Insert (adjust nothing except heading numbering if an A.11 already exists):

```markdown
### A.11 The human-adjudication vocabulary and its description rules

Part 2 §7.6 defines the mechanism by which a question the protocol cannot
answer (a utility question) is surfaced to the humans who can. The mechanism
already has a settled vocabulary across both parts; this section makes it a
convention so experiment docs, plans, and test suites stop drifting from it.
The canonical terms, each anchored where it is defined:

- **the seam** — provenance fully settled, utility still open (Part 1 §2.0's
  razor, restated at Part 2 §7.6.1 as "the razor's seam"). The condition that
  triggers the mechanism.
- **the recognizer** — the governed classifier of §7.4.1: a per-Group
  benign-versus-dispute tolerance over verifiable provenance signals, never a
  constant, with the rule that genuinely ambiguous cases escalate. Recognizing
  the seam is itself partly a utility judgment, which is why the recognizer is
  governed rather than fixed.
- **the reconcile hard-stop** — the event (§7.6). Prose spelling is
  *hard-stop*, hyphenated, as noun and verb; snake_case only inside code
  identifiers.
- **the escalation shapes** — Contradiction (too many valid claims) and
  Under-determination (too few valid claims) (§7.6.1). Distinct from the
  **escalation parties**, the personae surfaced by a given case, who are
  presented symmetrically (both parties, no presumed wrongdoer).
- **the algedonic channel** — the lineage name for the designed escalation
  path (Part 1 §3, after Beer); in Part 2 and code, the operational family is
  *escalate / escalation*. One mechanism, one definition, two registers of
  name.
- **the legible picture** — the machine's deliverable: a grounded statement of
  the conflicting facts in governance language, full provenance, no
  editorializing, deterministically identical on every node (§7.6.1,
  P-Knowable-Truth).
- **the local authority** — the holder of the utility judgment on the social
  plane: the principal for its own judgments (A.4's adjudication locus), the
  affected Group for shared ones. Coined here so escalation text has a name
  for the receiving side that is not a node role.
- **planes of authority** — infrastructure authority (held by *operators*)
  and social-utility authority (held by *local authorities*). Operators are
  necessary, and a good operator is aligned to the separation; the design
  constraint is the separation itself: there is no path from the
  infrastructure plane into intra-Group social-utility authority. The two
  roles are mutually exclusive by design, like mutually exclusive Group Roles.
  This names, at the authority level, the same structure A.4/A.5 establish
  for permissions and revocation and A.7's node-role cast establishes for
  standing ("a capability or a resource but no standing").

Description rules (checkable; DR numbering is referenced by other docs):

- **DR-1 (one anchor).** The mechanism is defined once, at Part 2 §7.6; every
  other mention points there (doc-method Rule 7). No section re-derives it.
- **DR-2 (terms front-loaded).** The terms above live here and in the Part 2
  term surface; new docs inherit them by reference with a working definition
  (doc-method Rule 2).
- **DR-3 (spelling).** *hard-stop* in prose. No "hard stop", "hardstop", or
  "hard_stop" outside code identifiers.
- **DR-4 (planes of authority).** *Operator* names only the
  infrastructure-plane role. The holder of a social-utility judgment is the
  *local authority* (or *principal* / *affected Group* where the kind
  matters). State the relation as plane separation, never as distrust: write
  "the operator holds no path into social-utility authority" rather than "no
  operator to trust," and scope any trust statement to its plane ("trust for
  what").
- **DR-5 (continuity language, never moral language).** Escalated cases are
  described in symmetric, continuity-framed vocabulary: parties at equal
  standing, divergence, lineage, continuity, departure point. Not used in
  normative text about the mechanism: wrong, offender, violation, punish,
  guilty, or dispute-as-default. Motivation is unprovable from provenance and
  is never asserted. Test: the sentence must read identically well when the
  underlying story is a grievance and when it is routine mechanics (a member
  archiving a Group that removed them under a 30-days-inactive rule).
- **DR-6 (no machine verdicts).** Machine outputs are dispositions, statuses,
  or pictures; never verdicts, rulings, or decisions. Applies to test suites
  and vectors.
- **DR-7 (the recognizer is governed).** Text describing detection of the
  seam states that the classifier is governed per §7.4.1 and that ambiguity
  escalates. Never describe the recognizer as a constant or the protocol as
  "knowing" a dispute occurred.
- **DR-8 (name the shape).** Every escalation mention states which shape it
  concerns: Contradiction, Under-determination, or both.
- **DR-9 (the boundary sentence).** Where the machine/human boundary is
  described, use the three-clause pattern: the protocol surfaces the legible
  picture; the humans supply utility; the instantiation of their choice is
  Drystone's.
- **DR-10 (signal and authority stay local).** Escalation surfaces *to* the
  affected Group; it never relocates the decision *up* to anything.
```

Changelog: add a Part B entry (B.1 or B.2.1 style) recording that A.11 was
added and why (drift found between spec vocabulary and experiment corpus).

## E2. Part 1 §3: add the Internet-governance corroboration entry

File: `beta/drystone-spec/part-1-reasoning-underpinnings.md`

Anchor: section `## 3. Why these principles are corroborated, not invented`.
Place the new entry adjacent to the existing Beer / algedonic-channel entry
(the block quoting *Brain of the Firm*), following the section's established
format: the conclusion the discipline supplies, then the verbatim source as
outside corroboration, with a per-quote verification flag.

Fetch these primaries during the run and pull the verbatim quotes from them:

- RFC 8126 (BCP 26), §5.2 "The Role of the Designated Expert":
  https://www.rfc-editor.org/rfc/rfc8126 — quote the sentences stating that
  IANA forwards assignment requests to the designated expert for evaluation
  and that the expert informs IANA whether to make the registration, and that
  the experts are listed in the registry.
- W3C Permissions (TR): https://www.w3.org/TR/permissions/ — the definition of
  a powerful feature requiring express permission before use; and W3C
  Geolocation (current CR): https://www.w3.org/TR/geolocation/ — the
  normative check-permission enforcement. Note in the flag that the 2016 REC
  wording differs from the current CR wording.
- RFC 7282: https://www.rfc-editor.org/rfc/rfc7282 — the definition of rough
  consensus as objections addressed, not votes counted.

Entry to write (fill the quote slots from the primaries; keep the surrounding
prose as given, in the section's voice):

```markdown
**Internet governance, from the discipline that runs the namespaces.** The
formal specification tradition of the Internet age has already reconciled
"the machine must stop and a person must decide" into its own machinery, in
both of the shapes Drystone uses. The first is a named human role inside the
procedure: the IANA registration policies (RFC 8126, BCP 26) define the
**Designated Expert**, a person to whom the evaluation of a registration
request is formally delegated: [verbatim quote from RFC 8126 §5.2 here]
> RFC 8126, §5.2 · *Verified against the primary at edit time.*
The second is a normative human-input step inside an algorithm: the W3C
platform defines **powerful features** that a user agent must not exercise
without the end-user's express permission, enforced by check-permission steps
the API methods normatively depend on (Permissions TR; Geolocation).
[verbatim quote here] And the IETF publishes the semantics of its own human
layer in the same document series as its wire formats: rough consensus is
objections addressed, not votes counted (RFC 7282). [verbatim quote here]

The corroboration cuts one level deeper than precedent. RFC 8126 shows a
formal spec can say, at this step a person decides, and survive as running
governance for decades. What Drystone changes is the *locus*, not the move:
where the registry delegates the judgment to one named expert per registry (a
center), Drystone distributes the designated-expert role to every principal.
The **local authority** is the designated expert for its own utility
judgments, because a persona only represents a person if that person holds
the adjudication (§2.3, §2.5). Beer supplies the channel (escalate the
residue, keep authority where the context lives, §3 above); the IANA
tradition supplies the named human step inside formal machinery; Drystone
composes the two and removes the center.
```

Also add matching entries to the reference list at the end of §3 / the refs
section, in the existing format (author/spec, year, what it grounds,
verification flag):

- RFC 8126 (Cotton, Leiba, Narten), 2017, BCP 26 — grounds the
  named-human-step precedent (§2.5, Part 2 §7.6). Flag per fetch outcome.
- W3C Permissions TR and Geolocation (current CR) — grounds the normative
  human-input-step precedent. Flag per fetch outcome; note REC-vs-CR wording.
- RFC 7282 (Resnick), 2014 — grounds publishing human-layer semantics in the
  same series as wire formats. Flag per fetch outcome.

Back map + part-1-changelog entries required.

## E3. Part 1 §2.5: one forward-pointer sentence

File: `beta/drystone-spec/part-1-reasoning-underpinnings.md`

Anchor: within `### 2.5. The forced terminus: why fork, not verdict`, at the
point where the fork is characterized as human-adjudicated. Append one
sentence in place (adapt conjunction to the surrounding sentence):

> The move has formal precedent: Internet governance names exactly this step,
> a person deciding inside the machinery, as the Designated Expert (§3); the
> terminus differs only in distributing that role to every local authority
> rather than delegating it to one.

Back map note only if §2.5's map line changes meaning; changelog entry yes.

## E4. Part 2 §7.6.1: shapes vs parties disambiguation

File: `beta/drystone-spec/part-2-certifiable-design.md`

Two anchored micro-edits:

1. In `#### 7.6.1. The escalation set has two members, not one`, first
   paragraph, after "they differ only in how they are detected." append:

   > (Terminology, per the conventions primer A.11: these two are the
   > **escalation shapes**; the personae a given case surfaces are the
   > **escalation parties**, presented symmetrically, both parties and no
   > presumed wrongdoer.)

2. The beat E5 running example currently reads "*the escalation set is both
   Alice and Bob, not one of them, so a mechanism watching only for a
   contradiction would miss the case where a required role is left vacant*".
   Replace "the escalation set is both Alice and Bob, not one of them" with
   "the escalation parties are both Alice and Bob, not one of them", leaving
   the rest of the sentence intact.

Also add a one-line coinage note where §7.6 first says the escalation
"surfaces the conflict to the affected Group": append ", the **local
authority** for a shared judgment (conventions A.11)".

Back map + part-2-changelog entries required.

## E5. Conformance suite: verdict → disposition

File: `alpha/crystallized/conformance-suite.md`

Anchor: vector category 6, "Reconcile corpus C1–C10. Each merge scenario →
the expected verdict (converge | hard-stop contradiction | re-formation
fork)". Replace "the expected verdict" with "the expected **disposition**",
and add at the end of that item: "(*Disposition*, not *verdict*: machine
outputs are dispositions; verdicts are what the protocol declines to compute,
per fork-not-verdict and conventions A.11 DR-6.)"

Grep the rest of the file for "verdict" and apply the same rename to any
machine-output usage (leave prose about the doctrine itself untouched).

## E6. iroh corpus: clean out the rogue LWW/timestamp resolution language

Files: `alpha/experiments/iroh/DESIGN.md`, `alpha/experiments/iroh/docs/threat-model.md`,
`alpha/experiments/iroh/README.md` (snapshot line refs: DESIGN 24, 319, 402,
416 (§7), 591; threat-model 417, 441; README 327, 440 — verify by grep for
`conflict resolution` and `last-writer-wins`/`LWW`).

These passages describe last-writer-wins by timestamp as a decided design.
That contradicts two standing invariants (fork-not-verdict; timestamps are
unprovable assertions and never used for ordering) and reads as precedent to
anyone grepping the corpus. Decision from the design owner: excise, don't
scope-note.

Method, to preserve auditability without teaching the forbidden model:

1. In DESIGN.md §7 ("Conflict resolution rules — DECIDED"): retitle to
   "§7. Superseded: timestamp conflict resolution (do not reuse)" and replace
   the body with a short superseded block: state that the original section
   decided LWW-per-key by `(modified_at, node_id)`; that this is superseded
   and MUST NOT be reused in Drystone work because ordering never derives
   from timestamps and concurrent contradiction hard-stops rather than
   auto-resolves (Part 2 §7.3.1, §7.6; conventions A.11); and that the
   original text is recoverable from git history.
2. Apply the same one-line superseded treatment to the other listed mentions
   (README bullets, threat-model assumption 5 "system clock is roughly
   correct" and line 441), each pointing at the DESIGN.md §7 superseded block
   rather than repeating it (DR-1).
3. Do not silently delete: every removal leaves the one-line pointer.

## E7. hard-stop spelling normalization (experiment prose only)

Grep `alpha/` and `beta/` markdown for `hardstop`, `hard stop`, `hard_stop`
in prose and normalize to `hard-stop`. Known snapshot hits:
`alpha/experiments/iroh/META-TRANSCRIPT-next-session.md:98` ("hard stop") and
`alpha/experiments/appview-validation/README.md:452` ("a hard stop in
practice" — judgment call: this one is the English idiom, not the mechanism;
if it reads as the idiom, leave it and note in the summary). Never touch code
identifiers or test names (`e2_4_conflict_hard_stops` etc. stay).

## E8. doc-writing-method.md: pointer only

File: `beta/impl/doc-writing-method.md`

At the end of `## 12. Name the sociotechnical alignment where a concept spans
both planes`, append one sentence:

> The worked convention set for the largest such concept, the
> human-adjudication mechanism and its planes-of-authority vocabulary, lives
> in the spec conventions doc (conventions-and-decisions.md A.11) and is
> normative for all Drystone docs, experiments included.

## E9. Registers and summary

- Add a line to `alpha/experiments/SPEC-DIVERGENCE-REGISTER.md` (matching its
  existing entry format) recording the iroh LWW language as a resolved
  divergence (superseded in place, E6).
- Write `RUN-SUMMARY-adjudication-language.md`: per-edit status
  (landed/skipped), anchor drift encountered, fetch outcomes for the E2
  primaries, and any judgment calls (E7's idiom case).
