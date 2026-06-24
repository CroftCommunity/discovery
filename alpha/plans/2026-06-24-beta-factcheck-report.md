# Beta first-pass adversarial fact-check — findings report

date: 2026-06-24

purpose: adversarial fact-check of the `discovery/beta/` first-pass synthesis (themes `01`–`08`,
`README.md`) and its companion ledger `alpha/BETA-ROLLUP.md`, per the handoff at
`alpha/seeds/generated-prompts/beta-factcheck-prompt.md`. Quote and reference fidelity were the top
priority; six of eight theme docs (`02`,`03`,`05`,`06`,`07`,`08`) were drafted from same-model
subagent briefs, so every quote/citation/flag was re-derived from source, not trusted from the beta
label.

method: one verification agent per theme (each opened the beta doc, its `BETA-ROLLUP` rows, its cited
alpha sources, and the relevant source-of-truth — FACTCHECK / `proof-ledger.md` / `COHESION.md`),
plus a dedicated external-quote **web-verification sweep** and a **proof-flag + recurring-numbers
sweep** against `proof-ledger.md` and the FACTCHECK. All agents ran read-only on the session's Opus
model. **No beta or alpha file was edited.** Corrections below are proposed, not applied — awaiting
review.

> Severity rubric (from the handoff): **BLOCKER** = fabricated/wrong quote or attribution, a REFUTED
> item carried forward, or an over-claimed status that changes meaning. **MAJOR** = wrong
> flag/citation, unsupported claim, or ledger mistrace. **MINOR** = punctuation/wording nit or missing
> cross-ref. **OK** = verified correct.
>
> Note on regrading: two sub-agents labeled a finding "BLOCKER" that is, by the rubric, a citation
> mistrace (MAJOR). Those are regraded here and noted. Conversely, the external web sweep upgraded one
> beta-internally-faithful quote to MAJOR (Ashby) because it diverges from the primary source the alpha
> appendix claimed had verified it.

---

## Headline

- **0 BLOCKERS.** No fabricated quote, no REFUTED item leaked into a beta body, no proof status
  inflated in a way that changes meaning. Every do-not-carry exclusion is genuinely absent. Recurring
  numbers (openmls `0.8.1`, iroh `1.0.0`, conformance `66/0`, Steem HF23 `$6.3M`/`23.6M STEEM` not
  `$5M`, Phase-1 gate GO, the four-property impossibility) are consistent across docs.
- **6 MAJORS**, all fixable: one genuine misquote + one over-confident verification flag in `01`
  (surfaced only by web verification), one body cross-reference mistrace in `02`, one stale
  open-vs-resolved claim in `04`, one source-attribution over-reach in `05`, one cross-doc proof-status
  inconsistency in `06`.
- **One systematic pattern worth a single fix pass:** **FACTCHECK over-attribution** — real facts that
  actually originate in another alpha doc are tagged "cite FACTCHECK SoT" in `05` and `08`. The facts
  are sound; the source label is wrong.
- **Verdict: conditional GO.** Beta is structurally sound and faithful enough to carry into rc-stage
  work. It is **not yet fit for external publication** until the `01` Ashby quote is corrected and the
  two over-confident alpha "Verified" flags (Ashby, Beer) are downgraded. See "Go / no-go" at the end.

---

## MAJOR findings (fix before rc-promotion; the quote/flag items block external publish)

### M1 — `beta/01:190` — Ashby quote is the paraphrase, not Ashby's words (and the alpha "Verified" flag is wrong)
- As written: `"Only variety can absorb variety."` — attributed to W. R. Ashby, *An Introduction to
  Cybernetics* (1956), and flagged **Verified** (carried from `alpha/narrative/lineage-of-a-design-imperative.md:193`).
- Source of truth (web): Ashby's text (Pt. 11, ~p.207) is **"only variety can destroy variety."**
  "Absorb variety" is the widely circulated **restatement** (popularized via Stafford Beer), *not*
  Ashby's 1956 wording. The beta presents "absorb" inside quotation marks as a direct Ashby quote.
- Why MAJOR: a quoted attribution with the wrong load-bearing verb. The theme-01 agent correctly found
  beta matches the alpha *verbatim* — so this is an **alpha-origin misquote that propagated**, and the
  alpha appendix's "Verified" flag is itself incorrect. The handoff explicitly asked to spot-check the
  "Verified" set; this is the spot-check failing.
- Correction: change to "Only variety can destroy variety" with the *Introduction to Cybernetics*
  cite, **or** drop the quotation marks and present "absorb variety" as a gloss (and attribute the
  "absorb" phrasing to Beer if retained). Downgrade the alpha "Verified" flag for this line to
  "confirm against primary edition." (Beta-level fix; alpha is frozen — re-flag at the beta layer.)

### M2 — `beta/01:205-207` — Beer "aids to human viability" quote unconfirmable as primary-text verbatim, yet flagged Verified
- As written: treat computers, dashboards, simulations `"as aids to human viability, not as excuses
  for automatic command."` — S. Beer, *Brain of the Firm* (1972), flagged **Verified**
  (`lineage:195-196`).
- Source of truth (web): the phrase appears in secondary sources *describing* Beer; the web sweep
  could **not** source it as a verbatim line in *Brain of the Firm*. Beer's humane-cybernetics position
  is correctly represented, but the quotation marks imply a direct quote that is unverified.
- Why MAJOR (escalation): the verification flag ("Verified") is stronger than the evidence. Same class
  as M1 — a "Verified" appendix flag that does not hold up to primary-source spot-check.
- Correction: confirm against *Brain of the Firm* before any external use, or reframe as a paraphrase
  and downgrade the flag to "confirm against primary edition." Flagged for your judgment — it may be
  genuinely verbatim in an edition the sweep could not reach.

### M3 — `beta/02:136` — wrong COHESION section cited for the trap-and-balance reconciliation
- As written: `"(Collapses COHESION §30/§34: the trap-reading and the balance-reading reconciled as
  one lesson.)"`
- Source of truth: `alpha/COHESION.md` §34 (`:879`) is the trap-vs-balance refinement; **§30**
  (`:782`) is the Groundmist/Hive/identity-chain dialogue (whose only relevance to `02` is the Hard
  Fork 23 `$6.3M` correction, correctly attached at the provenance trace `beta/02:275`).
- Why MAJOR (regraded from the agent's "BLOCKER"): a wrong in-body cross-reference is a citation
  mistrace, not a fabricated quote — MAJOR per the rubric.
- Correction: cite **§34** alone for the trap/balance reconciliation; keep §30 with the §6/provenance
  Hard-Fork-23 material. (The flat rollup source list at `BETA-ROLLUP.md:100` listing "§14,§16,§24,§30,§34"
  is fine; the error is the body's attribution of §30 to the trap-reading.)

### M4 — `beta/04:215-217` — spec-vs-code reconciliation presented as an open item; the SoT says it is RESOLVED
- As written: "the tagged pre-images are now canonical in `lineage-core::ids`, but **the spec text
  needs to be reconciled to match.**"
- Source of truth: `alpha/crystallized/CROFT-PROTOCOL.md` §2 (`:50-66`, incl. the 2026-06-17 addendum)
  already states the tagged pre-images as canonical; `proof-ledger.md:180` = "Spec-vs-code derivation
  discrepancy **RESOLVED**"; `test-narrative.md:620` = "surfaced **AND resolved** (2026-06-17)." The
  divergent `"lineage-topic-v1"` tag is no longer the canonical spec tag.
- Why MAJOR: beta presents a resolved item as a live divergence — an under-claim of completion that
  misstates project status. (`BETA-ROLLUP.md:254` hedges "full reconciliation … is a 04 follow-on,"
  so beta/04 inherited the ambiguity, but the SoT is unambiguous.)
- Correction: reframe as resolved (the residual is at most prose tidy-up), and do not present
  `"lineage-topic-v1"` as the current spec tag.

### M5 — `beta/05:203-206` — §8 mis-attributes most atproto facts to the FACTCHECK SoT
- As written: "**Precise atproto facts (cite FACTCHECK SoT):** `did:plc`/`did:web` are atproto's two
  blessed methods (`did:plc` = "Public Ledger of Credentials"); `did:plc` has a 72h recovery window …;
  rotation keys must be k256 or p256; `plc.directory` is run by Bluesky PBC as a transparency log …
  governance handoff planned, not done."
- Source of truth: the FACTCHECK contains **only** two of these — `did:plc` = "Public Ledger of
  Credentials" (`FACTCHECK:58`) and the blessed-methods point (`FACTCHECK:93`). The 72h window +
  k256/p256 come from `alpha/thinking/plc-identity-resilience.md:41,145`; the PBC / transparency-log /
  governance-handoff facts come from `alpha/.../croft-identity-provenance-dialogue-2026-06-20.md:519-525`
  and `plc-identity-resilience.md:161`. (Grep of the FACTCHECK for `72h|k256|p256|PBC|transparency log|
  governance handoff` returns zero hits.)
- Why MAJOR: wrong citation — over-claims FACTCHECK coverage. The facts themselves are each
  well-supported by the cited alpha sources; only the source label is wrong.
- Correction: split the attribution — FACTCHECK for blessed-methods + "Public Ledger of Credentials";
  `plc-identity-resilience.md` / the provenance dialogue (and COHESION §21) for the 72h window,
  k256/p256, PBC/transparency-log, governance-handoff.

### M6 — `beta/06:90` — E3.4 blind broker labeled `green-model`; the ledger and `04` say `green-real`
- As written: "the Tier-0 blind broker is **green-model** (E3.4)".
- Source of truth: `alpha/crystallized/proof-ledger.md:109` = E3.4 "Broker observes only ciphertext +
  routing | blind-broker | **green-real**"; `beta/04:156` itself states E3.4 = **green-real**.
- Why MAJOR (regraded up from the 06 agent's MINOR; confirmed by the flag sweep): a proof status that
  contradicts both the ledger and the sibling beta doc for the *same* experiment. It is an under-claim
  (does not inflate), but it is a cross-doc inconsistency on a verification flag, which the tier
  discipline treats as load-bearing.
- Correction: `beta/06:90` → "the Tier-0 blind broker is **green-real** (E3.4)". (The paired "meer
  binary is green-real (E9)" on the same line is already correct.)

---

## MINOR findings (wording/citation hygiene; fix opportunistically)

### Theme 01
- `01:114-116` — Plato *Apology* 21d line ("he supposes he knows something … do not even suppose that
  I do") matches **neither** the cited Fowler translation nor Jowett; reads as a paraphrase/conflation.
  The locus (21d–22d) is correct, and 22d ("conscious that I knew practically nothing") is a verbatim
  MATCH. Correction: substitute the verified Fowler 21d text or relabel the 21d line as a paraphrase.
- `01:150-151` — Popper: beta "can only be finite"; primary edition reads "can **be only** finite"
  (word order). Beta already carries "confirm exact wording/edition" — apply it.
- `01:142-145` — Peirce: beta "block the **road** of inquiry"; Peirce's canonical (CP 1.135) is "Do
  not block the **way** of inquiry." The beta hedge ("phrasing varies by essay") softens this to MINOR;
  consider the canonical wording before external use.
- `01:209` — Cybersyn "survived" elides that it was destroyed from outside by the 1973 coup (the alpha
  makes this point load-bearing: "no architecture is immune to the larger reality it sits inside,"
  `lineage:118-119`). Optionally "survived until destroyed from outside."
- `01:186` — awkward in-document self-reference "the federation design in `01`§6" from inside doc 01;
  drop the `01` prefix. Cosmetic.

### Theme 02
- `02:167` — Sheriff title reordered ("Sheriff William Ivory of Inverness-shire" vs source "the
  Sheriff of Inverness-shire, William Ivory"); the quoted span and archive ref (NRS, GD1/36/1/3/52) are
  verbatim. Same person, no meaning change.
- `02:150-153` (and the Clare quotes generally) — Clare's punctuation is editor-dependent (en-dash vs
  em-dash); beta's word choice matches recognized editions and is flagged public-domain. Optionally
  name the edition. (Web sweep: all Clare lines MATCH.)

### Theme 03 (all citation/attribution hygiene — facts are sound)
- `03:154-158` — the atproto private-data block quote is a faithful **synthesis** rendered as a
  verbatim `>` block; no single source sentence matches. If `>` is reserved for verbatim, reset as
  prose. Every fact checks out.
- `03:165-167` — the Germ block quote lists `research/germ-xchat-features.md` first, but that
  (pre-maturation) file carries **none** of the 2026-02-18 / Mark Xue / AC-Protocol / Anchor-Key
  facts; they live in `ECOSYSTEM.md:254` + the FACTCHECK addendum. Drop `germ-xchat-features.md` from
  this quote's attribution (or cite it only for the privacy-inversion it actually supplies).
- `03:103-105` — the "designed forward as the bounded broadcast tier" clause is pinned to COHESION §3
  (the *cautionary tale*); that framing actually lives in **§11** (the lesson applied forward). Both
  are cited; only the clause attribution is crossed.
- `03:243-249` vs `BETA-ROLLUP.md:127` — the doc's §2 collapses COHESION §3/§11/§13, but the declared
  COHESION set for theme 03 in the ledger is "§17/§25/§26/§28/§29/§31/§32." §3/§11/§13 are legitimately
  in scope (messaging-landscape seams) but undeclared. Add them to the rollup set or drop the claim.

### Theme 04
- `04:142` — Phase-1 gate parenthetical `(green-real)`: the ledger tags the four constituent
  experiments E1.1–E1.4 green-real and the gate verdict as bare "GO"; the compression is faithful in
  substance. Acceptable as written.
- `04:159` — conformance row shown as bare "**green**"; matches the ledger's own bare "green" (mixed
  green-real/green-model rows). Optionally annotate "(mixed rows; design rows deferred)."
- (`04:244` design-dialogue path is **correct** — file is at `alpha/seeds/transcripts/...`, not under
  `raw/`. Non-issue; recorded so it is not re-raised.)

### Theme 05
- `05:204` — "two blessed methods" omits that atproto `did:web` is hostname-level only (no path-based
  DIDs) per `plc-identity-resilience.md:49`; acceptable for a synthesis, flagged because §3 leans on
  did:web's limits. (The FACTCHECK-attribution error is M5 above.)

### Theme 06
- `06:197` — re-formation cited as "C3"; the ledger's re-formation row is "**A1 re-formation (trap
  door)**" green-real-multimachine (`proof-ledger.md:164`). C3 (`:212`) is "concurrent identical remove
  heals" — unrelated. Beta inherited the "C3/re-formation" shorthand from `meer-superpeer-design.md:131`.
  Cite as "A1 re-formation." Claim's truth (re-formation is green-real-multimachine) is unaffected.

### Theme 07
- `07:37-38` — narrative overview paraphrases "embedded in the **valuation** bankers are pricing"
  (unquoted) where the source/verbatim block quote at `07:243` reads "embedded in the **number**
  bankers are pricing." Acceptable (not in quotation marks), flagged for tightness.

### Theme 08
- `08:192-193` — the "browser iroh peers are permanently relayed; hole-punch falls back to relays"
  claim is tagged "(cite FACTCHECK SoT)," but that specific fact is **not** in the FACTCHECK — it
  originates in `thinking/app/README.md:78` and COHESION §19 (`:372-375`). The FACTCHECK does support
  iroh `1.0.0`, DERP→relays rename, and Tauri-native-WebView. Re-attribute the relay-fallback claim to
  the README/§19. (Same FACTCHECK-over-attribution pattern as M5.)
- `08:30` (and `:24,:56,:62,:207`) — "**Croft Group**" / "'Croft' the product" used as working
  structural labels while `BETA-ROLLUP.md:229-231` / COHESION §18 flag them do-not-carry brand DRIFT.
  **Escalation for your judgment:** the doc loudly disclaims them (decision-gated banner +
  `08:213-214` "do not propagate unsettled names into structure"), and the alpha sources themselves use
  "Croft Group" as a working handle. If the beta-stage rule is *zero* use of unsettled brand names even
  as working handles, upgrade to BLOCKER; if working handles under an explicit disclaimer are
  acceptable, leave as-is. The over-claim risk is contained by the banner.

---

## Cross-cutting confirmations (the adversarial checks that came back clean)

**Do-not-carry exclusions — all confirmed ABSENT from beta bodies** (present only inside explicit
"excluded/REFUTED" callouts where noted):
- crypto-wars fabrications (Zimmermann "Stalin", the Meyer letter, "Voskop") — absent from `01`
  (grep-confirmed). Hush-A-Phone/Carterfone correctly carried as CONFIRMED.
- the "ancient free clan" / "noble chief betrayed" myths — absent from `02`; the genocide/ethnic-
  cleansing framing is **not** presented as settled. `$5M` Steem figure absent (correct `$6.3M`).
- iroh-docs "Merkle Search Trees", the fictional "AT Messaging working group", `did:key`
  atproto-resolvability, `did:plc`="Public Liaison Corporation", the false Vultr 1-Click PDS,
  "Voskop", `$5M`/`$5` figures — all absent from `03` bodies (named only inside REFUTED callouts).
- `did:key` resolvability, the "Equivalency Assertion" label, "Public Liaison Corporation",
  over-strong "cannot be faked" — absent from `05` body (named only in the excluded-items list);
  did:webvh native support and "PLC governance handoff completed" correctly carried as `[UNVERIFIED]`/
  "planned, not done".
- the "central hand on the wheel" / edge-AI-scan-then-snitch playbook appears in `06` only as the named
  **rejected** anti-pattern; the "Princeps Problem" formulation is absent from `06` (grep-confirmed).
- all MO §351 statute sections, fees, royalty %/cap/runway/fund dollar figures, Discord ARR/MAU/
  valuation specifics, acquisition prices, state-comparison citations — **all absent from `07`**
  (do-not-carry grep returned zero `$`, zero `%`, zero statute section numbers). Noria is consistently
  a **candidate pending clearance**, never "decided." Both banners (DECISION-GATED + NOT-LEGAL-ADVICE)
  present.
- unsettled brand names not propagated into `08` structure (except the Croft Group working-handle
  escalation above); "serverless" appears only in scare-quotes with the relay asterisk.

**Proof-status sweep (`04`,`06`,`08` vs `proof-ledger.md`):** every status re-derived; all match the
ledger **except M6** (E3.4 in `06`). Confirmed correct: Phase-1 gate GO; E1.1–E1.4 / E2.1–E2.8
green-real; I1–I10 green-real with **I9 green-model**; multi-device + thresholds green-real; cross-machine
green-real-multimachine; faithful-wire green-real; conformance 66/0 green; E12 green-real (synthetic
frames); E10/E11 characterized; meer P0+P1 green-real; V3 green-model (automatic crossing only); S3
spec/unsolved; total-device-loss recovery OPEN (largest residual); Phase-0 green-real (20/20).

**04 ↔ 06 shared-mechanism consistency:** CONSISTENT. `04` owns the freshness-signal + revocation-
authority crypto wire (tip beacon, `meets_threshold_by_lineage` validator, per-epoch authority,
`quorum_override`, ADMIN FLOOR, never-irrevocable ladder, capture≠brick); `06` reasons them socially
(no-false-current, MEMBERSHIP-FRESH admin bar, freshness-gates-authority) and routes every wire claim
and the fresh-but-wrong residual to `04`. No mechanism divergence. The three weighted over-claims the
alpha ledger said were avoided are all correctly held off in `06`: freshness does **not** "solve"
partition; the geer does **not** "solve" compellability; the ADMIN FLOOR prevents **brick**, not
capture.

**atproto/iroh FACTCHECK facts:** every fact matches the FACTCHECK and is framed as citing it; no
CONFIRMED-where-FACTCHECK-says-PARTLY; `did:plc`="Public Ledger of Credentials" everywhere;
no "Public Liaison Corporation". (The two source-label slips are M5 and the `08:192` MINOR.)

**Recurring numbers:** openmls `0.8.1` (9 occurrences in `04`/README, consistent); iroh `1.0.0` (single
pin at `04:250`; other mentions unversioned prose); conformance `66/0`; Steem HF23 `$6.3M`/`23.6M STEEM`
(no `$5M` anywhere); four-property impossibility stated identically across `03`; Phase-1 GO consistent;
Farcaster rent figure not present in beta (nothing to flag).

**External-quote web verification — the clean MATCHES:** Plato 22d; Mill "If all mankind minus one…";
Hayek "dispersed bits of incomplete and frequently contradictory knowledge…"; Hush-A-Phone "privately
beneficial without being publicly detrimental"; MED *croft* Towneley "[hell]" citation (MED10396); OED
verb "to croft" 1772 Manchester Directory (date confirmed; verbatim sentence correctly left
`[UNVERIFIED]` behind the paywall); Clare *The Mores* and *To a Fallen Elm* (all lines); "Magna Carta of
the Highlands and Islands" → John Lorne Campbell (with the "of Gaeldom" variant correctly flagged);
John Ball "When Adam delved and Eve span…"; Winstanley "common treasury for all, both rich and poor"
(26 April 1649, correct primary form preferred over the variant); "Goose and the Common" (wording match;
dating correctly flagged disputed); Twitter Circles dates (Aug 2022 → Oct 31 2023); Doctorow
enshittification (verbatim body; beta drops only the "Here is how platforms die:" lead-in). The
Discourse-founder "95% stopped using Matrix within a month" anecdote is correctly framed as the
founder's own account (self-sourced, not independently verifiable).

**Tier discipline:** all forward-references in all eight docs point only to other beta docs; provenance
points down to alpha via `../alpha/...` paths in the footers; the only additive alpha file is
`BETA-ROLLUP.md`. No beta→alpha forward dependency in any narrative; no alpha edit.

**Ledger accuracy (`BETA-ROLLUP.md`):** "landed in §X" pointers and treatments spot-checked per theme
and accurate, except the M3 body cross-reference (a beta-doc issue, not a ledger row) and the `03`
undeclared-COHESION-set MINOR. Coverage view honestly lists the not-yet-pulled-up alpha sources.

---

## Summary table — count by severity per theme

| Theme | BLOCKER | MAJOR | MINOR | Notes |
|---|---|---|---|---|
| 01 epistemic | 0 | 2 | 4 | M1 Ashby misquote, M2 Beer flag; Socrates 21d / Popper / Peirce / Cybersyn / self-ref |
| 02 enclosure | 0 | 1 | 1 | M3 COHESION §30 miscite; Ivory title order. All external quotes MATCH. |
| 03 ecosystem | 0 | 0 | 4 | block-quote framing ×2, crossed §3/§11 clause, undeclared COHESION set |
| 04 protocol | 0 | 1 | 2 | M4 stale spec-vs-code "open"; gate/conformance label nuances |
| 05 identity | 0 | 1 | 1 | M5 FACTCHECK over-attribution; did:web hostname-only nuance |
| 06 safety | 0 | 1 | 1 | M6 E3.4 green-model vs green-real; "C3" vs "A1" re-formation |
| 07 sustainability | 0 | 0 | 1 | unquoted paraphrase; gate checks (NOT-LEGAL-ADVICE, Noria) clean |
| 08 product | 0 | 0 | 2 | FACTCHECK over-attribution (08:192); "Croft Group" handle (escalation) |
| README | 0 | 0 | 0 | statuses/template/surfaced-gates accurate |
| **Total** | **0** | **6** | **16** | + 1 escalation (08:30 brand handle) |

---

## Go / no-go

**Conditional GO.**

- **For continued rc-stage work (internal):** beta is fit to proceed. Zero blockers, do-not-carry
  exclusions clean, proof statuses accurate (bar the single E3.4 label), recurring numbers consistent,
  tier discipline intact, and the 04↔06 shared mechanisms consistent. The 6 MAJORs are point fixes, not
  structural problems.
- **For external publication: NOT yet.** Two items must be fixed first, both in `01` and both
  surfaced only by primary-source verification (not visible from beta-vs-alpha checking):
  1. **M1 — the Ashby quote** ("absorb" → "destroy", or de-quote). A wrong-verb quoted attribution is
     the one finding closest to a publication blocker.
  2. **M2 — the Beer "Verified" flag** (confirm against *Brain of the Firm* or reframe as paraphrase).
  Both reveal that the alpha appendix's "Verified" set was not fully reliable — recommend a quick
  re-pass over the remaining "Verified" classical quotes before any external draft.

Recommended order of correction (after your review): M1, M2 (publication-gating) → M3, M4, M5, M6
(citation/status integrity) → the FACTCHECK-over-attribution MINORs in `05`/`08` as one pass → the
remaining MINORs opportunistically. Resolve the `08:30` "Croft Group" escalation with a one-line ruling
on whether disclaimed working-handles are allowed at beta stage.

No surfaced decision gate (MPL license, recovery anchor, cooperative legal review, the Noria name,
CroftC Phase-0 IP, genome-vs-strategy) was touched or resolved by this audit.
