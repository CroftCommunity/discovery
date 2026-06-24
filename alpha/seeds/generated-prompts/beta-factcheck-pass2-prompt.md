# Prompt: second clean fact-check pass of `discovery/beta/` (post-correction)

Copy this into a **fresh session after `/clear`**, ideally on the **highest-capability model available**.
This is the **second** adversarial fact-check of the beta synthesis — run *after* a round of corrections
was applied to the first pass. The corpus moved enough (10 files, ~200 lines) that a clean re-audit is
warranted. Treat everything as unverified again: re-derive from source, do not trust any label — including
the corrections log's own claims that an issue was fixed.

---

You're in the CroftC workspace (`/Users/cpettet/git/chasemp/CroftC`). I want a rigorous, adversarial
**second-pass** fact-check of `discovery/beta/` and its companion ledger. **Do not edit beta or alpha in
this pass** — produce a findings report I can review, then we'll correct together. Git identity: chasemp
(`chase@owasp.org`, `github-personal`). Don't commit or push unless I ask.

## What changed since pass 1 (read these first, then verify against source — do not trust them)

- **First-pass findings report:** `discovery/alpha/plans/2026-06-24-beta-factcheck-report.md` (0 blockers,
  6 majors, 16 minors, conditional-GO).
- **Corrections accounting log:** `discovery/alpha/plans/2026-06-24-beta-factcheck-corrections-log.md` —
  every edit applied, before→after, with the source claimed. Commits `62d5585` (corrections) and the
  follow-on "Croft Group" placeholder tag sit on top of `7dc899a` (the pass-1 report) and `4d45ac8` (the
  beta first-pass synthesis). `git log --oneline -6` and `git show 62d5585` will show the exact diffs.
- The corrections touched **all eight beta docs** + `alpha/BETA-ROLLUP.md` (the only additive alpha file).

## Orient (read before auditing — the rubric you audit against)

- `discovery/AGENTS.md` — staging model + path convention.
- `discovery/MATURITY-ROLLUP.md` — the method beta followed (per-doc template, tier discipline,
  verbatim-quote rule, "do-not-carry = absent," the rollup-ledger pattern).
- `discovery/beta/README.md` — the beta map (eight themes, statuses, surfaced gates).
- `discovery/alpha/BETA-ROLLUP.md` — the alpha→beta trace; your map from each beta claim to its source.

## Two jobs this pass

### Job A — verify the corrections actually landed AND are themselves correct (re-derive, don't trust the log)

For each entry in the corrections log, confirm (a) the edit is present in beta, and (b) it is *right* —
re-derive from the source of truth, do not accept the log's word:

- **01 quotes — re-verify against primary editions** (the pass-1 fixes changed wording): Plato *Apology*
  21d (now claimed verbatim Fowler/Loeb), Mill *On Liberty* ch.2 (colon after "truth"), Peirce ("Do not
  block the way of inquiry" / CP 1.135), Popper ("can be only finite" / C&R p.30), Ashby ("Only variety
  can destroy variety" / p.207), and the **Beer** line (now a paraphrase, no quotation marks — confirm it
  is genuinely de-quoted and not re-attributed as a quote). Web-verify each.
- **01 "Verified" flags** — pass 1 found two alpha "Verified" flags (Ashby, Beer) were unreliable.
  Spot-check that the remaining classical quotes still flagged "Verified" (Socrates 22d, Mill, Hayek,
  Scott-as-referenced-case) genuinely hold, and that Ashby/Beer are now correctly re-flagged.
- **02** — COHESION cross-ref is now §34 only (not §30); Ivory title reordered. Confirm §34 is the
  trap/balance refinement and §30 is not.
- **03** — the SSB §3/§11 clause (uncrossed), the Germ quote re-attribution (ECOSYSTEM §6 + FACTCHECK
  addendum, not germ-xchat-features.md), and the expanded declared-COHESION set (§3/§8/§9/§11/§13 added to
  both the beta footer and the BETA-ROLLUP row). Confirm each cited COHESION section says what beta claims.
- **04** — the spec-vs-code item was reframed from "open" to "surfaced AND resolved (2026-06-17)". Confirm
  against `proof-ledger.md` ("RESOLVED"), `test-narrative.md`, and `CROFT-PROTOCOL.md` §2 + addendum that
  this is accurate and not an over-correction.
- **05** — §8 atproto-fact attribution split (FACTCHECK only for blessed-methods + did:plc expansion; the
  72h window / k256-p256 → plc-identity-resilience.md; PBC/transparency-log/governance-handoff → the
  identity dialogue + COHESION §21). Confirm each fact is now sourced to a doc that actually contains it.
- **06** — E3.4 is now `green-real` (was green-model); re-formation is now `green-real-multimachine` cited
  as "A1 re-formation (trap door)". Re-derive both from `proof-ledger.md` and confirm 04 and 06 now agree.
- **07** — narrative paraphrase tightened to "embedded in the number bankers are pricing" (matches the
  verbatim block quote at 07:243).
- **08** — relay-fallback re-attributed to `thinking/app/README.md` + COHESION §19 (iroh 1.0.0 /
  DERP→relays / Tauri-native-WebView still cite FACTCHECK); and **"Croft Group" now carries a
  placeholder tag** "[working name, pending brand reconciliation …]". The Croft Group naming is a
  **settled call** (keep + tag) — do NOT re-flag it as a blocker; only confirm the tag reads cleanly and
  the names are not asserted as decided elsewhere in 08.

### Job B — a fresh, full adversarial sweep (catch what pass 1 missed, and anything the edits introduced)

Do not limit to the corrected lines. Re-run the full audit on all eight docs — pass 1 was thorough but
not infallible, and edits can introduce new errors (broken cross-refs, a paraphrase that now overstates,
a citation that no longer matches surrounding prose, a quote whose flag no longer fits the new wording).
Pay special attention to text *adjacent to* the edits.

## Source-of-truth hierarchy (unchanged from pass 1)

- Each beta doc's `Sources (alpha)` footer + its `BETA-ROLLUP` rows → open those alpha files.
- atproto/iroh/iOS: `alpha/seeds/transcripts/raw/atproto-atmospheric-web-iroh-mobile-FACTCHECK.md` is the
  SoT — beta must *cite* it, not re-verify (iroh `1.0.0`; iroh-docs = range-set reconciliation + LWW, not
  MST; no native AT-Proto E2EE; `did:key` not atproto-resolvable; `did:plc` = "Public Ledger of
  Credentials").
- Proof statuses: `alpha/crystallized/proof-ledger.md` + `test-narrative.md`. Re-derive every flag.
- Seam claims ("collapses COHESION §N"): `alpha/COHESION.md` — confirm the cited section resolves what
  beta says.
- External quotes: verify against primary editions / reputable secondary sources via web.

## The audit dimensions (run each across all eight docs)

1. **Quote fidelity (highest priority)** — every block quote verbatim vs source; external quotes
   web-verified. Misquote or wrong attribution = BLOCKER.
2. **Verification-flag correctness** — re-derive each flag (CONFIRMED / REFUTED / `[UNVERIFIED]` /
   green-real / green-model / spec / characterized / NOT-LEGAL-ADVICE) from the SoT. Flag any status
   stronger than the evidence.
3. **Claim support** — every substantive claim traces to a source; flag invented/over-claimed/smoothed.
4. **Do-not-carry absence** — confirm the excluded items are genuinely absent (not relabeled): the
   "ancient free clan" myth; the REFUTED crypto-wars fabrications (Zimmermann "Stalin", Meyer letter,
   "Voskop"); the MST conflation; the fictional "AT Messaging working group"; `did:key` resolvability;
   "Public Liaison Corporation"; the MO §351 statute/fee/dollar specifics; over-claimed "serverless".
5. **Cite-the-SoT discipline** — atproto/iroh/iOS facts cite the FACTCHECK and match it; volatile figures
   (Discord/Bluesky valuations, DAU) carry `[UNVERIFIED]`.
6. **Ledger accuracy** (`alpha/BETA-ROLLUP.md`) — "landed in §X" pointers real; treatments accurate;
   coverage view honest. (Note the theme-03 COHESION row was expanded this round — confirm it's right.)
7. **Tier discipline** — beta forward-refs only to other beta docs; provenance points down to alpha;
   alpha corpus untouched (only `BETA-ROLLUP.md` is additive). Flag any beta→alpha forward dependency or
   any alpha edit.
8. **Internal consistency** — cross-theme refs resolve; shared mechanisms stated consistently (freshness
   + revocation as facts in `04`, reasoned socially in `06`); recurring numbers agree (openmls `0.8.1`;
   iroh `1.0.0`; conformance `66/0`; Hard Fork 23 ≈ `$6.3M / 23.6M STEEM`; the four-property
   impossibility; Phase-1 gate GO).

## Elevated-risk areas (weight effort here)

- **01** — the most-edited doc this round; re-verify every classical quote against primary editions and
  confirm the paraphrase reframings (Ashby second line, Beer) don't read as verbatim.
- **02 / 03 / 05 / 08** — the citation/attribution edits; confirm the new citations point at docs that
  actually carry the facts, and that surrounding prose still matches the corrected text.
- **04 / 06** — re-derive every proof status; confirm the 04↔06 freshness/revocation consistency held and
  the E3.4 / re-formation status changes are right.

## Method

Fan out (one verification agent per beta theme is a reasonable shape), plus a dedicated **external-quote
web sweep** and a **proof-flag + recurring-numbers sweep** against `proof-ledger.md` and the FACTCHECK.
Be adversarial: assume errors remain or were introduced by the edits; re-derive, don't trust labels.
For external quotes, actually fetch and compare.

**Model rule (standing, all Croft work):** every subagent runs on the *same model as this session's
primary model* — set `model` explicitly on each Agent/Explore call; never let a subagent downgrade. Run
this on the highest-capability model available; the fan-out inherits it.

## Output

Write a findings report to `discovery/alpha/plans/<YYYY-MM-DD>-beta-factcheck-pass2-report.md` (a process
artifact; not in `beta/`). Per finding: beta location (`file:line`), the claim/quote as written, what the
SoT actually says (with path/URL), a **severity** (BLOCKER / MAJOR / MINOR / OK), and a recommended
correction. Separate Job-A findings (a correction that did not land or is itself wrong — call these
**REGRESSION** and rank them first) from Job-B findings (newly discovered). End with a summary table
(count by severity per theme), a short "corrections-verified" checklist (which pass-1 fixes are confirmed
good), and a go/no-go on whether beta is fit for the next stage. **Do not edit beta or alpha** — propose
corrections and wait for my review. Do not resolve any surfaced decision gate (MPL license, recovery
anchor, cooperative legal review, the Noria name, CroftC Phase-0 IP, genome-vs-strategy).

## Definition of done

A single findings report keyed to `file:line`: every pass-1 correction independently re-verified against
source (REGRESSION items flagged first), every block quote re-checked (external quotes web-verified),
every proof flag re-derived, the do-not-carry exclusions re-confirmed absent, the ledger spot-checked, and
a severity-ranked list of any remaining or newly-introduced issues — produced without editing beta or
alpha, and without resolving the surfaced gates.
