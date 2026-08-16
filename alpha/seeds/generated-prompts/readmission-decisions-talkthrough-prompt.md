# Handoff prompt: talk through the five readmission/governance decisions, then merge one clean corpus

`Written 2026-08-16 at the end of the exclusion/readmission experiment arc (S15–S22, G1).`
`Copy everything below the line into a fresh session.`

---

We are going to talk through five open decisions from the MLS exclusion/readmission work and, once
they're all decided, do **one clean merge** of the candidate spec revisions onto mainline Part 2.
This is a decision conversation, not an experiment session — but every claim you make must be
grounded in what was measured, and where something is untested you say so explicitly.

## Orient first (read in this order, skim-level except where noted)

1. `discovery/beta/drystone-spec/DOSSIER-exclusion-and-readmission-2026-08-16.md` — the findings,
   organized as the **readmission dial** (read closely: Parts 3, 3.5, 4)
2. `discovery/beta/drystone-spec/SCENARIO-WALK-2026-08-16.md` — Appendix E's L1–L6 walked against
   the measurements, the 44-row scenario matrix, and G1 (the §7.3.1 fold checked against its own
   keys; read closely: Part 3)
3. `discovery/beta/drystone-spec/part-2-certifiable-design-WORKING-2026-08-16.md` — the candidate
   spec. **13 `[REV 2026-08-16]` blocks; the banner indexes them; grep the tag for the full diff.**
   The canonical `part-2-certifiable-design.md` is UNTOUCHED and stays that way until the merge at
   the end of this conversation.
4. `discovery/alpha/experiments/meer-queue/STATE-AND-NEXT.md` — the resume point; headlines of
   S15–S22 + G1
5. `discovery/alpha/ROADMAP_TODO.md` rows **E96, E105, E106, E107, E108** — the five decisions,
   each with evidence and a retire-by

Evidence ground truth: `discovery/alpha/experiments/meer-queue/tests/s15_*.rs … s22_*.rs` (32
tests, Rung A) and `discovery/alpha/experiments/croft-chat/croft-chat/tests/fold_ordering_keys.rs`
(G1, Modeled). Where any doc and the code disagree, the code is right.

## Non-negotiable context (learned this arc — do not relitigate, do not regress)

- **There is no server.** Part 1 §2.4: a Group MUST NOT structurally depend on any single persona;
  a meer is optional; everything else is distributed. Every member holds live group state and can
  export a `GroupInfo`. Any "policy" is a group-context rule every peer applies.
- **Every `GroupInfo` admits its holder** (S19): it carries `external_pub`, no export flag can
  strip it. The **ratchet tree** is the only withholdable artifact (S18) — a bare `GroupInfo`
  proves current state without admitting.
- **Re-entry is self-admission** (S20): there is no request to deny; the gates are who-is-served
  and a merge-time group-context policy. A per-member prompt is a partition generator, and the
  fork it causes is **invisible in the epoch counter** (S18).
- **A negative standing check fails OPEN at the least-synced peer; a positive credential fails
  CLOSED** (S22). G1 showed why: a peer folding an incomplete governance set resolves standing
  differently and is not wrong over the set it holds — "keep everyone synced" restates the
  gap-completeness beam (§11.11 item 3), it doesn't mitigate it.
- **§11.7's resumption-PSK credential is unbuildable on openmls 0.8.1** (S16); a
  **governance-issued external PSK** is measured working end to end and carries both credential
  halves (standing + keys) in one artifact.
- **The §7.3.1 fold hard-stops the realistic ban-vs-rejoin race order-independently** (G1) — it
  escalates, it does not fail open. The comparator was aligned to key 3 (v2, `lamport → hash`,
  versioned + rebuild-tested). What survives is E108: the membership *projection* while
  hard-stopped diverges by arrival order.
- Several findings in this arc were **made and then withdrawn** (S14's "neither mechanism
  applies"; S22's "small enumerable serving tier"; G1's "key 1 fails open"). The docs record the
  withdrawals at the point of the original claim. Do not resurrect a withdrawn claim; if you cite
  one of these areas, cite the corrected form.

## The conversation, in order

Work through these one at a time. After each decision, record it: update the ROADMAP_TODO row,
and finalize the corresponding `[REV]` block in the WORKING copy. Plain-English first — lead with
the decision and options, mechanics second (owner preference, on file).

### 1. E106 — ratify the governance-issued external PSK as the re-entry credential?

Standing recommendation: **ratify.** The token proves "governance vouches for you now," not "you
were there" — argue why that is *more* aligned with §11.8 (standing resolves at head, never over
returner-asserted history), not a compromise. The alternatives (wait for upstream openmls; fork
the library) are not live options.

### 2. The proactive-issuance move (the load-bearing one — E105 + E107 fall out of it)

The proposal: **issue the token proactively at migration-to-cold** (part of the migration
commit's bookkeeping), serve the **bare** `GroupInfo` to anyone, release the **tree only against
a token**. Croft defaults to dial position 2, and it feels like position 1 to every honest user —
every legitimate dormant returner already holds a token, so requiring one costs the graceful path
nothing; the only tokenless parties are strangers and banned lineages.

**The owner's first question, answer it before advocating: what does this look like in CONCRETE
terms.** Walk it as real sequences, not abstractions:

- Boreas goes dormant → what exactly is minted, by whom, stored where, bound to what (lineage?
  device? epoch?), and how does Boreas's client hold it across the dormancy?
- Boreas returns → who does he ask (any member — S22), what bytes does he present, what does the
  serving peer check (token validity + standing at head — the S22 stub), what is released (tree),
  and what does the external-commit + merge-time check sequence look like end to end?
- Cyrus (banned) tries the same → where exactly does each attempt die, at a synced peer and at a
  stale peer? Be honest about the residual: banned-after-token-issue at a stale peer serves —
  same eventual-consistency window §11.8 owns, bounded by re-keying, but confined to
  banned-former-members rather than open to anyone.
- A total stranger tries → dies where?
- What of this exists today (S16 measured the PSK attach + pre-merge visibility; S22 measured the
  serve policy shape) vs what is unbuilt (issuance at migration, token storage, revocation
  semantics, the merge-time group-context rule, position 2 as an end-to-end admission decision —
  E107's untested thirds)?

**The owner's second question, treat it as a formal step: a conformance sweep.** Go through the
existing spec expectations and identify anything the current thinking CANNOT meet. At minimum
check: §11.7's self-service claim (cost on the returner — does token issuance reintroduce a
dependency on live members at return time? At migration time?); §11.7's progressive-return MUST;
§11.6's liveness-window machinery and hot-tree scaling (does token bookkeeping ride the migration
commit without breaking batching?); §11.8's ban-lineage interlock and head-resolution rule; Part
1 §2.4 (no structural dependency — who mints the token if the group is quiescent when the
liveness window lapses? Is proactive issuance itself a liveness assumption?); §7.3.6
decide-vs-enact; the §11.11 open items. **Produce an explicit list: "expectations met /
expectations changed / expectations we cannot meet and must renegotiate in the spec."** That list
gates the merge.

### 3. E108 — what does a contradicted group project?

Standing recommendation: **(c) "contested" as a first-class membership state** (not member / not
not-member / contested-with-the-pair), because (a) restrictive projection manufactures a
mini-verdict and (b) no-answer lies by omission, while §7.6's posture is present-the-contradiction.
Cost: touches the member-view type — a schema change, not prose. Decide, and note Croft renders it
however it likes ("membership pending resolution").

### 4. E96 — adopt nested sealing?

Cost fully measured (S17): 28 flat bytes, one local AEAD op, and the **wrapping rule** (wrap at
the epoch of the queue; the closing commit at the epoch it closes — get it backwards and the walk
deadlocks silently). Leak it closes: `group_id` cleartext linkability to ANY swarm participant
(E94's widened scope). Standing recommendation: **SHOULD for fabric-carried envelopes**, wrapping
rule normative alongside; not MUST.

### 5. The merge

Only after 1–4 are decided. One pass onto canonical `part-2-certifiable-design.md`:

- Apply the merge-ready corrections (the §11.8 + Appendix E L6 "zero marginal exposure" strike is
  the load-bearing one; §11.6 three-states; §11.7 recovery bounds; §11.11 sharpenings).
- Fold each decided EXX's REV into final prose — no `[REV]` scaffolding, no strikethrough
  artifacts, claim labels upgraded where measurements earn it (***Design*** → ***Measured*** with
  evidence pointers).
- Resolve the L2/L3/L6 beats of Appendix E per the scenario walk's verdicts.
- Retire the WORKING copy (its job is done once merged — note it in the changelog per the repo's
  convention, `part-2-changelog.md`).
- Update ROADMAP_TODO rows to retired/narrowed as decided; refresh STATE-AND-NEXT.
- Commit in clean logical pieces. Identity: chasemp (`chase@owasp.org`). Do not push unless asked.

## Ground rules

- Canonical part-2 is edited ONLY in step 5, after all decisions.
- Every position you argue gets its evidence pointer (S-test or G1); anything untested is labeled
  untested, and nothing untested is presented as a reason to skip the E107 grounding work.
- If talking-through surfaces a scenario we have not measured and the decision turns on it, say
  so and propose the experiment rather than assuming the answer — this arc's corrections all came
  from exactly that discipline.
