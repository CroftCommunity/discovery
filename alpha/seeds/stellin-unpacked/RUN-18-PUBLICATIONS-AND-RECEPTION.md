# RUN-18: Reception completeness + publications positioning — consolidated follow-up to the landed RUN-16/17 (v2)

`Status: runnable brief, 2026-07-17 (v2; supersedes the earlier RUN-18 reception-only brief IF
that brief has not been dispatched — if it HAS already run, execute only the parts marked
[NEW-v2] below as RUN-19 and renumber accordingly). SEQUENCING GATE: run only on a main that
already contains the MERGED RUN-16 (group tier model) and RUN-17 (tier proof). This is a
DELTA run: it amends living corpus documents and extends the landed experiment; it re-runs
nothing and NEVER edits frozen records (landed RUN summaries untouched; this run gets its own
summary). Confirm numbering against alpha/experiments/MASTER-INDEX.md. House rules: never
edit the reviewed spec; spec-facing notes go to the proposed-changes staging doc +
reviews-log row, same commit. TDD red-first per the standing directive: corpus-tests and
experiment tests are extended RED before docs or code land GREEN.`

## 0. What this run adds

Two design outcomes reached after RUN-16/17 were dispatched:

1. **Reception completeness for write-restricted scopes** (the newsletter's subscriber-side
   guarantee): per-author envelope chaining; completeness as DETECTION up to newest held,
   never delivery; the withheld-tail limit named per the completeness-ahead doctrine, with
   multimodal delivery as the mitigation; open enrollment never weakens verification.

2. **The publications positioning** [NEW-v2]: a standalone design doc (payload shipped with
   this brief as PUBLICATIONS-DESIGN.md; land it at
   `alpha/experiments/appview-infra/PUBLICATIONS.md`) recording the vanilla-atproto
   comparison — what the substrate proves natively (authorship, integrity, current-state
   completeness; the DEGENERATION PRINCIPLE binding our open/single scopes to bare records
   plus chaining only), the single-agent limit (no collective noun; lists as
   consent-free curator records; the provable multi-party fact as the one added atom), the
   delta table (authorization-at-position vs authorship; tamper-EVIDENT history vs
   tamper-FREE current state with the never-existed / retracted / withheld three-way
   distinction; institution vs account), and the subscriber reframe (two rosters, one
   lineage; guarantee beneficiary; auditable reach and churn; structural consent; the paid
   tier as a policy value; the honest scope of "managing").

## 1. Guardrails

1. Branch `claude/publications-run-18` from origin/main (post-merge per the gate). One red
   commit then one green commit per part (`test(pub):` / `feat(pub):`); summary shows order
   and grades.

2. Frozen records: `RUN-16-SUMMARY.md` and `RUN-17-SUMMARY.md` are not edited. Living corpus
   docs and experiment code/tests are the amendment surface. If landed headings or section
   numbers differ from the RUN-16 brief's labels, locate the landed equivalents semantically,
   amend THERE, and record the mapping (brief label → landed location) in this run's summary.

3. No credentials required for the mandatory path; B5's live variant is optional (guardrail
   4). No lexicon publication. Stop rules and SPEC-DELTA discipline as in RUN-17.

4. Live-optional: if `ATP_TEST_HANDLE/_PASSWORD` are present, B5 runs its record-deletion
   step against the real PDS (upgrading it to live grade); otherwise the deletion is
   simulated in the landed harness and tagged `SPEC-DELTA[run18-retraction-local |
   stand-in]`. Never silently downgrade.

## 2. Part A — corpus (the RUN-16-landed surface)

**A1 (red).** Extend `alpha/experiments/appview-infra/corpus-tests/groups_claims.bats` with
failing checks for: per-author chaining required in write-restricted scopes; reception
completeness defined as detection-up-to-newest-held, never delivery; the withheld-tail limit
tied to the completeness-ahead doctrine; open enrollment not weakening verification;
[NEW-v2] the degeneration principle stated as binding; the single-agent-limit claim and the
consent contrast with curator lists; the three-way retraction distinction; the
auditable-count claim; the honest-scope-of-managing language.

**A2 (green).** Amend the landed GROUPS.md at its two-axes section with the canonical
reception paragraph:

> **Reception completeness for write-restricted scopes.** In `single` and `named-set`
> scopes, authors MUST chain their envelopes: each envelope carries the author's previous
> envelope in its antecedents. Any reader holding envelope N can then verify the complete
> stream N-1..1 exists, detect any gap as a known omission, and retrieve it via interval
> backfill from any role or peer — a subscriber's completeness is verifiable from public
> data alone, and open enrollment never weakens it (verification requires no standing, only
> the envelopes). The honest limit, per the completeness-ahead doctrine: a withheld TAIL is
> undetectable until anything newer arrives by any path; multimodal delivery (DS plus
> optional swarm) is the mitigation, and freshness/solicitation posture remains a governed
> dial, never a mechanism that closes the limit. Delivery is best-effort; DETECTION of
> incompleteness up to the newest held envelope is the guarantee.

Cross-references: the transports section's swarm paragraph gains one sentence noting the
second path converts a withheld tail from silent to detected; the tier table's
history-access row gains a pointer if the landed table warrants it.

**A3 (green) [NEW-v2].** Land PUBLICATIONS-DESIGN.md as
`alpha/experiments/appview-infra/PUBLICATIONS.md`, largely verbatim, adjusting only headings
and links to house style and pointing its §6 at the landed GROUPS.md sections by their real
anchors. Add a reciprocal pointer in GROUPS.md's introduction (one line). Site gate: the new
doc joins the broken-ref + anchor audit.

**A4.** Staged spec note: append an addendum to the RUN-16 section of the proposed-changes
doc — the chaining requirement and detection-not-delivery framing; [NEW-v2] the degeneration
principle as a design constraint (open/single scopes ride bare records + chaining) and the
tamper-evident-history delta as the publication-facing consequence of chaining. Reviews-log
row same commit. Backlog rows: reception completeness (proven by Part B); [NEW-v2] the
publications doc landed; the auditable-count claim tied to its B6 evidence.

## 3. Part B — experiment (the RUN-17-landed surface)

Extend the landed `alpha/experiments/tier-proof/` crate; new tests red first, then minimal
implementation to green. Reuse the landed P3 scopes and P8 processes; add no new processes.

**B1. Chaining validation.** An envelope from a `single` writer that does not reference the
author's prior envelope fails validation (first envelope anchors to scope genesis); an
unchained envelope injected at the relay function is dropped unpropagated
(validate-before-relay extended with the chaining check for write-restricted scopes).

**B2. Gap detection and repair.** A reader handed the newsletter stream minus one MIDDLE
envelope detects the gap from the chain alone (no oracle), names the missing identity,
repairs via backfill from the landed DS/convergence store, and ends provably complete up to
the newest envelope held.

**B3. The tail, honestly.** With the newest envelope withheld, the detector's own claim is
under test: it MUST report "complete as of <newest held>" and MUST NOT claim full currency;
the withheld tail then arrives via the landed P8 swarm-path stand-in, is detected and
folded, and the claim advances — the multimodal closure, executable.

**B4. Interval interaction.** Chaining and interval backfill compose: a subscriber enrolled
after position J repairs gaps within [J, now) only; the chain crossing J is visible as
structure (the antecedent reference exists) without the pre-J envelope being offered —
detection may see history's shape; offering respects the interval rule.

**B5. Retraction, the three-way distinction [NEW-v2].** The tamper-evident-history delta
made executable. Publish a short chained stream; then RETRACT a middle issue (delete the
content record — live against the real PDS if credentials permit per guardrail 4, else
simulated and tagged). Assertions: the issue's EXISTENCE remains provable from the chain
(its identity is referenced by its successor) even though its content is gone; the reader's
detector classifies the three cases distinctly and correctly — an identity never referenced
by any chain (never-existed), an identity referenced but whose content is verifiably deleted
at source (retracted), and an identity referenced whose content no source will currently
offer while deletion cannot be shown (withheld-from-me); and a vanilla current-state check
over the same repo shows the retracted issue as simply absent — the contrast that motivates
the delta, asserted, not narrated.

**B6. Auditable reach [NEW-v2].** The subscriber-count claim made executable on the landed
P2 open-tier machinery: an INDEPENDENT second fold over the same records re-derives exactly
the roster count the DS serves; an unsubscribe (authenticated deletion) moves both counts
identically; and a count the DS merely asserts without folded records behind it is
detectable as unsupported by the auditor. Component grade against the landed harness (P2's
live records may be reused where still present; note grade either way).

## 4. Reporting

`alpha/experiments/RUN-18-SUMMARY.md`: red→green table with grades; the brief-label →
landed-location mapping; corpus-tests before/after counts; predicted-vs-actual for any live
B5 step; one paragraph mapping B1–B6 to the landed GROUPS.md paragraph and the new
PUBLICATIONS.md sections so evidence tags can cite `(evidence: …, RUN-18)` per claim;
registers and MASTER-INDEX rows; site gate green. Non-goals, one line each: no re-run of
RUN-16/17; no frozen-record edits; no freshness-dial decision (governed, owner's); no swarm
transport upgrade (the P8 stand-in suffices); no paid-tier build (the doc notes it as a
policy value, nothing more); no Variant A/B or boundary decisions.
