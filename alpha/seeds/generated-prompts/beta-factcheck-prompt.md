# Prompt: fact-check the `discovery/beta/` first-pass synthesis

Copy this into a fresh session, ideally on the **highest-capability model available**, to adversarially
fact-check the beta synthesis. The work was produced quickly and **six of the eight theme docs (02, 03,
05, 06, 07, 08) were written from same-model subagent synthesis briefs**, not from sources the main agent
read line-by-line — so treat every quote, citation, and verification flag as *unverified until you
re-derive it from the source itself.* **Quote and reference fidelity is the top priority.**

---

You're in the CroftC workspace (`/Users/cpettet/git/chasemp/CroftC`). I want a rigorous, adversarial
fact-check of the `discovery/beta/` first-pass synthesis and its companion ledger. **Do not edit the beta
docs or alpha in this pass** — produce a findings report I can review, then we'll correct together. Git
identity: chasemp (`chase@owasp.org`, `github-personal`). Don't commit or push.

## Orient first (read before auditing — these define the rules the work claims to follow)

- `discovery/AGENTS.md` — staging model + path convention.
- `discovery/MATURITY-ROLLUP.md` (root) — the method the beta pass followed: the per-doc template, the
  tier discipline, the verbatim-quote rule, "do-not-carry = absent," and the rollup-ledger pattern. This
  is the rubric you are auditing *against*.
- `discovery/beta/README.md` — the beta map (eight themes, statuses, the template, the surfaced gates).
- `discovery/alpha/BETA-ROLLUP.md` — the alpha→beta rollup ledger: per-source treatment + the section each
  landed in, the exclusions, and the coverage view. This is your map from each beta claim back to its
  alpha source.

## What to audit

1. `discovery/beta/01`–`08` (the eight theme docs) — the primary target.
2. `discovery/beta/README.md` — statuses, template claims, surfaced gates.
3. `discovery/alpha/BETA-ROLLUP.md` — is the trace *accurate* (did content land where it says; are
   treatments right; is the coverage view honest)?
4. `discovery/MATURITY-ROLLUP.md` — internal consistency only (low priority).

**Not in scope:** rewriting alpha (frozen), re-running proofs, or changing the eight-theme structure. You
are checking truthfulness and fidelity, not redesigning.

## Sources of truth (the hierarchy to check against)

- **Each beta doc's `Sources (alpha)` footer + its BETA-ROLLUP rows** are the map to its alpha origins.
  Open those alpha files and verify against them.
- **atproto / iroh / iOS facts:** `alpha/seeds/transcripts/raw/atproto-atmospheric-web-iroh-mobile-FACTCHECK.md`
  (and its addenda) is the project source of truth — the beta docs are supposed to *cite* it, not
  re-verify. Confirm two things: (a) every atproto/iroh/iOS fact in beta matches the FACTCHECK, and (b)
  beta defers to it rather than asserting independent verification. (iroh `1.0.0`; iroh-docs = range-set
  reconciliation + LWW, not MST; no native AT-Proto E2EE; `did:key` not atproto-resolvable; `did:plc` =
  "Public Ledger of Credentials".)
- **Proof statuses** (`green-real` / `green-model` / `green-real-multimachine` / `spec` / `characterized`):
  `alpha/crystallized/proof-ledger.md` + `test-narrative.md`. Re-derive every status flag in `04` and `06`
  from the ledger; do not trust the beta doc's own label.
- **Seam claims** ("collapses COHESION §N", "harvests …"): `alpha/COHESION.md` — confirm the cited section
  says what the beta doc claims it resolves.
- **Quotes flagged for external verification** (the beta docs mark these): verify against primary editions
  / reputable secondary sources via web. At minimum: Peirce ("block the road of inquiry"), Popper ("Our
  knowledge can only be finite…"), Ostrom (the subsidiarity passage — flagged as a 2013 generalization,
  *not* Governing the Commons), the **John Clare** poem lines (The Mores / Remembrances / To a Fallen Elm —
  editor-dependent; confirm wording), the Hush-A-Phone "privately beneficial without being publicly
  detrimental" standard, the Doctorow enshittification quote, the MED `croft` sense-split, and the OED 1772
  verb date. Socrates/Mill/Hayek/Ashby/Beer/Scott are flagged Verified in the alpha appendix — spot-check.

## The audit dimensions (run each across all eight docs)

1. **Quote fidelity (highest priority).** For every block quote in beta: does it match its alpha source
   *verbatim* — wording, punctuation, attribution? Where the alpha source is itself the only witness
   (cleaned-paste dialogue, internal design statements), confirm the beta quote matches the alpha text and
   that the alpha provenance/fidelity flag is carried. Where the quote is external and flagged for
   verification, verify against the primary/secondary source and report any drift. **A misquote or a wrong
   attribution is a BLOCKER.**
2. **Verification-flag correctness.** Re-derive each flag (CONFIRMED / REFUTED / `[UNVERIFIED]` /
   green-real / green-model / spec / NOT-LEGAL-ADVICE) from the source of truth. Flag any status that is
   stronger than the evidence (e.g. green-real where the ledger says green-model; CONFIRMED where the
   FACTCHECK says PARTLY).
3. **Claim support.** Every substantive claim must trace to an alpha source. Flag invented claims,
   over-claims, and "smoothed" syntheses that assert more than the source does.
4. **Do-not-carry absence.** Confirm the excluded items are genuinely **absent** from beta (not merely
   relabeled): the "ancient free clan" myth; the REFUTED crypto-wars fabrications (Zimmermann "Stalin",
   the Meyer letter, "Voskop"); the MST conflation; the fictional "AT Messaging working group"; `did:key`
   resolvability; "Public Liaison Corporation"; the MO §351 statute/fee specifics and dollar figures;
   unsettled brand/product names propagated into structure; over-claimed "serverless". Any that leaked in
   is a BLOCKER.
5. **Cite-the-SoT discipline.** Confirm atproto/iroh/iOS facts cite the FACTCHECK and match it (dimension
   above), and that volatile figures (Discord/Bluesky valuations, DAU) carry `[UNVERIFIED]`.
6. **Ledger accuracy (`alpha/BETA-ROLLUP.md`).** Spot-check that "landed in §X" pointers are real, that
   treatments (synthesized/collapsed/harvested/carried-flag/excluded/deferred) are accurate, and that the
   coverage view honestly lists what is not yet pulled up. Flag any source claimed as covered that isn't.
7. **Tier discipline.** Beta forward-references point only to other beta docs; provenance points down to
   alpha; alpha corpus content is untouched (only `BETA-ROLLUP.md` was added). Flag any beta→alpha forward
   dependency in the narrative, or any alpha edit.
8. **Internal consistency.** Cross-theme references resolve; shared mechanisms are stated consistently
   (freshness-signal + revocation-authority as facts in `04`, reasoned socially in `06`); recurring numbers
   agree across docs (openmls `0.8.1`; iroh `1.0.0`; conformance `66/0`; Hard Fork 23 ≈ `$6.3M / 23.6M
   STEEM`, not $5M; the four-property impossibility; Phase-1 gate GO).

## Elevated-risk areas (weight effort here)

- **02 (enclosure):** the most quote-dense doc — Clare poems, Chambers, the "Magna Carta" attribution
  variants, MED specifics, the 1772 date. Verify each; the tertiary colour quotes must stay `[UNVERIFIED]`.
- **03 (ecosystem) and 05 (identity):** the most fact-dense; every atproto/iroh fact must match the
  FACTCHECK SoT. Confirm none were re-asserted as independently verified.
- **04 (protocol) and 06 (safety):** every proof status must match `proof-ledger.md`; the shared
  freshness/revocation mechanisms must be consistent between the two.
- **07 (sustainability):** NOT-LEGAL-ADVICE — confirm no statute sections / fees / dollar figures survived,
  and that the Noria foundation name is presented as a *candidate pending clearance*, not decided.

## Method

Fan out (one verification agent per beta theme is a reasonable shape — each opens that theme's alpha
sources and the relevant SoT, and checks the eight dimensions for its doc), plus a dedicated **quote-
fidelity sweep** across all eight and a **verification-flag sweep** against `proof-ledger.md` + the
FACTCHECK. Be adversarial: assume errors were introduced; re-derive, don't trust the beta doc's own label.
For external quotes, actually fetch and compare.

**Model rule (standing, all Croft work):** every subagent you spawn must run on the *same model as this
session's primary model* — set `model` explicitly on each Agent/Explore call; never let a subagent
downgrade. Run this fact-check on the highest-capability model available, and its fan-out inherits that.

## Output

Write a findings report to `discovery/alpha/plans/<today>-beta-factcheck-report.md` (a process artifact;
do not put it in `beta/`). Per finding: the beta location (`file:line`), the claim/quote as written, what
the source of truth actually says (with the source path or URL), a **severity** (BLOCKER = fabricated/wrong
quote or attribution, refuted item carried forward, over-claimed status that changes meaning; MAJOR =
wrong flag/citation, unsupported claim, ledger mistrace; MINOR = punctuation/wording nit, missing
cross-ref; OK), and a recommended correction. End with a summary table (count by severity per theme) and a
go/no-go on whether beta is fit for the next stage. **Do not silently edit the beta docs** — propose the
corrections and wait for my review.

## Definition of done

A single findings report I can read in one sitting, keyed to `file:line`, with every block quote in beta
verified against its source (and external quotes web-verified), every verification flag re-derived, the
do-not-carry exclusions confirmed absent, the ledger spot-checked for accuracy, and a severity-ranked list
of corrections — produced without editing beta or alpha, and without resolving any of the surfaced
decision gates.
