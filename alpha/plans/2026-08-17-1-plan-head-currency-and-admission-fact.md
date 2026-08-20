# Plan: head-currency, the ack primitive, and the admission fact (C2–C5 + S24/S25 amendments)

`Written 2026-08-17, out of the independent review (beta/drystone-spec/`
`REVIEW-decision-talkthrough-2026-08-17.md) and the owner talk-through of the same date.`
`Status: RUN GREEN (2026-08-17; branch c-series-proveout, merged to main). The gate — S23–S26 +`
`C2–C4, C5 informative — is discharged, and the admission-fact amendment GRADUATED and MERGED into`
`canonical part-2 on 2026-08-19 (step 5), together with decision-2 and the reviewer's five-item`
`checklist. Kept as the experiment record; the E112 residuals live on the ROADMAP row.`

## Problem statement

Three connected gaps, all surfaced or sharpened by the 2026-08-17 review:

1. **The corroboration primitive is named but unbuilt, and the fresh-precondition tests have
   never run.** §7.4 requires a member originating or co-signing a membership act to be
   corroborated-fresh (same head from k distinct lineages) — "decided; tests specified, not yet
   run." §7.3.8's strict merge stalls on corroboration a node has no specified way to *obtain*;
   Appendix B names the missing piece: "a corroboration-on-latest request answered by a governed
   standard-of-care over head-attestations from distinct personae." Without it, S24/S25 would
   test "standing at head" against a harness oracle — verifying a mechanism no deployment has.

2. **The external-commit path has no governance-plane shadow.** An admission collapses decision
   and enactment into one MLS act performed by the joiner (S21: the external-join path has no
   proposal phase), so a stale admission (the Bob/Dana scenario) deposits nothing on the chain:
   repair triggers only from folding the ban, fork detection leans on the unbuilt
   epoch-counter-independent signal (E107 retire-by), and the invite path already has the
   symmetric fact half-decided (the DECISION-2 block names the Add-commit as the invitation's
   governance-fact mint point) while the self-service path has nothing.

3. **Three review findings that gate the merge:** door B's "fails CLOSED at a stale peer" is
   population-dependent (a banned ex-member holds a genuine token + lineage key; only the
   *standing* leg is stale — REVIEW verdict item 1); the "never admission" corollary is
   unqualified by §7.3.8's corroborated-not-proven residual (REVIEW verdict item 2); and the
   positive chain-fact leg of the three-way binding has no test arm (REVIEW item 24 gap 1).

Unverified compositions are indistinguishable from hallucinations; the spec merge is gated on
these runs.

## Approach

### The admission fact — design amendment (owner-agreed 2026-08-17, PRELIMINARY)

**Every admission deposits a governance fact, or it does not happen.** Amends DECISION-2's
"recognition is the merge" to: *recognition is the merge, and the merge deposits the admission
fact.*

- **Shape and author.** The joiner's signed attestation rides the AAD as today (S16: signed by
  the joiner's new leaf key — authenticates the carrier, never the claim). The **merging member**
  mints the admission fact: an R6-shaped acceptance record (§7.5.1 — an acceptance record *is* a
  governance fact) stating "merged lineage L's `NewMemberCommit` [content address], redeeming
  token T, at my frontier F," chained into the acceptor's acceptance chain with its frontier
  commitment. The admission *event* is identified by the commit's content address (§7.3.4
  sign-the-state: per-acceptor facts corroborate one event, never rival it). The returner mints
  nothing — a non-member (and at head possibly banned) authors no chain fact.
- **Merge-rule clause (new).** A `NewMemberCommit` merge that would not emit its admission fact
  is refused. (Symmetric with the invite path's Add-commit mint point, DECISION-2.)
- **Comparator placement (discharges §11.11 item 3's mapping obligation — owner talk-through,
  2026-08-17).** The admission fact is typed as an **acceptance/event record that opens a
  membership span — never a slot-competing membership addition.** It records the mechanical
  execution of the charter's pre-agreed merge rule, not a governance decision about the subject's
  standing, so it does not enter the §7.3.1 tier contest at all. This typing is load-bearing
  twice over: (a) it is what keeps a stale admission from ever forming a contradiction *pair*
  with a ban — a readmission **quorum** racing a ban is two decisions on the standing slot and
  correctly hard-stops → `CONTESTED` (G1/E108, as measured), while an admission fact racing a
  ban is an enactment record vs a decision and folds silently-but-visibly (routing the routine
  case to humans would drown the algedonic channel, §7.4.1); (b) it is what keeps correction
  **governance-forward, never chain-refusal** (the DECISION-2 posture): the admission was
  valid-at-its-position (fold-to-the-acceptor's-frontier showed good standing — the S26 rule),
  the ban governs standing-at-head, the **effective-membership projection reads standing over
  spans** (§7.3.2 projection pattern) so the subject is excluded the moment the ban folds
  (phase 1), and the corrective removal (§11.8 re-fire) **closes the span forward** — the window
  was real, the record says so, exposure is everything in it (corrected §11.8), nothing is
  retroactively unmade. Same fact set → same result in every arrival order, with no new
  comparator tier; the acceptor's frontier commitment classifies them (concurrently stale, no
  fault, §7.5.1). Span-opening also closes a pre-existing hole: cold-is-a-state homes spans on
  the chain, and self-service return previously had no span-opening fact.
- **What it preserves.** No per-member prompt at the gate — A7 (E110) stands: the fold outcome is
  deterministic and identical everywhere. The human decision moves to the response registers
  (§7.6.5), post-repair: accept / advocate readmission through governance / fork. **Croft
  presentation obligations** (product layer, rides E111, not a protocol dial): (1) the factual
  statement per §7.6.6 discipline ("you admitted L while offline; the group banned L at
  [position]; the admission was voided per group rules"); (2) the exposure disclosure per
  §7.6.12's honesty caveat + corrected §11.8 ("during the window L could read N messages");
  (3) the three registers reachable, governance/advocacy highlighted; (4) returner-side
  legibility per E108's no-lying-by-omission pattern ("admission voided — banned at head"), never
  silent unreadability.
- **What it buys** (recorded so the merge prose can cite it): fork detection from chain data for
  admission-shaped forks (narrows E107's open signal); restores §7.3.6's decision/enactment split
  on the one path that lacked it; successor text for the orphaned §11.8
  "positioning-artifacts-against-the-single-chain" MUST (REVIEW item 23.3); the event class whose
  broadcast carries the head and solicits acks (below); closes REVIEW coverage gap 6
  (add-commit-as-mint-point becomes an asserted property).

### C2 — behind-detection from traffic (RED-first)

*Home: croft-chat / local_storage_projection (the fold).* The §7.4 precondition, finally run.

1. **Detection arm (RED first):** a behind node receives a fact whose frontier reference names an
   unseen head → marks itself "behind," refuses to render current, and **refuses to originate or
   co-sign a membership op** (§7.4: corroborated-fresh required). Watch it fail before the
   precondition is wired in.
2. **Quiet-group negative arm (expected-fail, kept):** no traffic → no detection possible. This
   arm *proves* the ack primitive is needed rather than assuming it (the Appendix B
   "unreferenced tail," CALM-bounded).

### C3 — the HeadAck primitive

*Home: croft-chat (fold + real iroh-gossip at loopback, the FANOUT-M1 harness).*

Pin `HeadAck {group_id, head, generation, sig}` as a §7.3.4 sign-the-state object: identity is
the state attested, signatures union as corroboration. No wall-clock anywhere; horizons in
epochs/generations; local elapsed time only as a private freshness input (§7.4).

1. **Request/response:** a corroboration-on-latest request answered by signed head-attestations,
   over loopback iroh.
2. **Corroborated-fresh threshold:** freshness reached at k distinct **lineages** (never clients,
   §5.7) attesting one head; a node below k stays "behind" for membership-op purposes.
3. **Union property:** two acks of the same head from different signers = one object, two
   vouchers (never rivals).
4. **Adversarial arms:** a forged ack fails signature; an ack naming an unknown head is a
   **detected gap** (converge before trusting), never an authorization input (§7.4.3 discipline:
   locator, not authorization).

### C4 — the Bob/Dana end-to-end: stale admission, collision, repair

*Home: meer-queue (openmls 0.8.1 + the S24/S25 serve/merge harness). This is the review's
Bob/Dana story run whole, with the admission fact as the detection trigger.*

1. **Same-branch arm (whole group stale):** liberal posture; stale members serve + merge the
   banned-at-head returner (her *own* valid token + lineage key — both legs genuine; only
   standing is stale); the merge mints the admission fact (span opens). Sync arrives → the
   projection reads standing over spans (excluded, phase 1), R6 classifies the acceptors
   concurrently-stale, the §11.8 re-fire enacts the corrective removal (span closes forward),
   re-key excludes. Assert: the collision is **chain-visible** before any read failure; the
   exposure window is **counted** (messages sealed between span-open and re-exclusion — the S25
   propagation number, from the repair side); the group converges with **no residual
   divergence**; and **no hard-stop fired** (the typing kept the routine case routine).
1a. **Arrival-order permutation arm (the G1 pattern applied to the new type):** both ingest
   orders — admission-fact-then-ban and ban-then-admission-fact — project **byte-identically**:
   span recorded, subject excluded, acceptor classified concurrently-stale. This is the arm that
   pins the comparator placement (acceptance record, not slot competitor); its mutation target
   is an implementation that lets the admission fact compete on the membership slot.
2. **Diverged-branch arm:** synced members refuse; stale Bob merges → branch. Governance did not
   diverge (no rival quorum facts), so the **accidental-fork heal** (§7.6.2) re-plants over the
   fold-derived membership (returner excluded); Bob repoints; his fork-window messages carry over
   via the B1 continuity machinery (loopback rung, RUN-12 shapes); the returner is stranded in a
   branch of one. Assert: the admission fact — not queue-name divergence — is what named the fork.
3. **Genuine-contradiction control (the boundary the typing must hold):** a readmission *quorum*
   racing the ban — two governance decisions on the standing slot — hard-stops with the
   order-independent contradiction and projects `CONTESTED`, **while an admission fact racing
   the same ban never does**. Both assertions in one arm, so the routine/genuine line is pinned
   from both sides. (The `CONTESTED` pinning test itself remains croft-chat's, per E108.)

### C5 — ack cost honesty

*Home: croft-chat loopback.* Per-op normative acks are O(N) responses per event. Measure ack
volume vs N; test the scoped alternative: explicit acks only for finality-needing ops
(membership/governance, §7.4's own scope), lazy piggyback (your next authored fact carries your
head) otherwise — §7.3.3's solicitation-posture dial, given numbers. Informative, not gating.

### Amendments to S24/S25 (the token-reentry plan)

- **S24 graceful arm:** additionally asserts the admission fact is minted, chain-positioned, and
  refused-if-absent.
- **S24 new refusal arm (d):** PSK bytes present in incumbent storage with **no issuance fact**
  (forged/out-of-band ledger entry) → refused at merge. Closes the positive chain-fact leg
  (REVIEW coverage gap 1: the arm that severs fact-from-bytes).
- **S25 new population arm:** the **banned-holder** case — a banned lineage presenting its own
  genuine token + lineage key at a stale peer. Asserts the serve gate's behavior by population
  (stranger: fails closed, S22; banned holder: serve succeeds at the stale peer, admission dies
  at strict merge / voids at fold per C4). Retires the unqualified "fails CLOSED at a stale peer"
  wording (REVIEW verdict item 1).
- **S25 corroboration source:** arms 2–3 consume **C3's HeadAck** as the freshness source instead
  of a harness oracle.

### Gate alignment

Decision-2's merge gate = **S23–S26 + C2–C4** (C5 informative). This supersedes both the
"S23–S25" wording (DECISION-2 header; ROADMAP E105/E107/E110/E111) and the bare "S23–S26"
wording (banner, STATE-AND-NEXT, plan exit criteria); all six artifacts align to it at the next
edit pass. The WORKING §11.7 REV recording the admission-fact amendment and this gate is the
owner's to ratify.

## Reasoning

- **Why the ack primitive must precede (or accompany) S24/S25:** testing "standing at head"
  against a harness oracle verifies a mechanism no deployment will have; the strict-merge stall
  is only livable if corroboration is cheap, and whether it is cheap is C5's number, not an
  assumption.
- **Why the admission fact:** the external-commit path is the only admission path with no
  governance-plane record, and every gap the review found in this area (fork detection,
  population-dependent fails-closed, the orphaned §11.8 MUST, add-commit-mint coverage) is
  downstream of that asymmetry. One fact closes the set.
- **Why the routine case must not escalate:** §7.4.1's alarm-fatigue rule — the algedonic channel
  stays trustworthy only if determinate cases fold silently. The fact makes the routine case
  *visibly* silent (auditable superseded entry), which is the upgrade over invisible.
- **Why C4 replaces queue-name divergence as the detector:** a chain fact is readable by every
  member from the fold; queue-name divergence is observable only by members who happen to
  exchange traffic across the fork, and S18 measured how silent that is.
- **What a green C-series does NOT discharge:** the Appendix B completeness beam. Mapping of arms
  to the beam's four discharge obligations: C2/C3 exercise the *mechanism* half of obligation (1)
  (the completeness predicate and its coordination) and give partition behavior for (2)
  (fail-closed stall as the degraded-but-safe mode); C4 arm 1 bears on (3) (a late-arriving
  pre-checkpoint fact cannot silently reverse enforcement — the void-and-refire path is exactly
  that argument, for admissions); C4 arm 2 bears on (4) (fork-composition: explicit heal, never
  silent disagreement). The obligations' *proof* statements remain Appendix B's open work; these
  runs earn evidence, not the theorem.
- **Method guards** (standing findings): surface every ingest/processing result — no `let _ =`
  (G1); compare branches at equal epoch numbers for AEAD-grade checks (S19); commit green state
  before any hand-mutation; scenario realism per the owner's S-series challenges (the
  banned-holder arm exists precisely because the realistic adversary is a former member, not a
  stranger).

## Exit criteria

C2–C4 arms green (or failing arms with named, understood failure modes) at their stated rungs
(Modeled/loopback acceptable for C2/C3/C5; C4 at Rung A for the MLS half, loopback for the
continuity half); amended S24/S25 arms green; TEST-LOG.md rows written with fidelity rungs;
STATE-AND-NEXT amended; the six gate artifacts aligned. Then decision-2's REV blocks (including
the admission-fact amendment) graduate from PRELIMINARY and the step-5 merge may include them.
