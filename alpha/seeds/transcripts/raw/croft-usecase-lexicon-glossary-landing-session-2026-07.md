# Session raw: use-case fit (dating/meetup) · lexicon.community governance · kindred-work glossary · croft.ing landing page

**Preservation status: preserved-condensed (cleaned-paste, content-faithful — NOT a
byte-pristine export) — PLAYBOOK §4.** Pasted from a claude.ai session (spanning ~2026-07-17…20).
Render chrome removed (thinking-summary headers like "Contemplated…", "Prioritized…",
"Added to memory", doubled title lines, "Document · MD"). Where the substance was already filed as
a durable artifact, this raw **points to that artifact rather than re-embedding it** (dedup, per
the user's "watch for dupe content" instruction). No credentials present. Volatile vendor facts in
the dating thread are `[UNVERIFIED]` and carried in the filed report's Caveats.

---

## Thread 1 — Dating / friendship / meetup use-case fit

**User (attaching design docs — a framework for social applications):**
> I am wondering about dating/friendship/meetup apps and if that is a use case that could fit
> here and how the components might map to the needs and so what those needs are, I have not
> really used any of them in many years so not sure what the current experience and capabilities
> are, asking you to research to catalogue and compare to Drystone capabilities.

Attached alongside: the **PUBLICATIONS** design doc ("Publications on the group substrate") —
**already filed**, authoritative copy at `alpha/experiments/appview-infra/PUBLICATIONS.md`
(the RUN-18 landed version; the pasted brief-payload variant is preserved in git history, commit
265f875). Not re-embedded here.

**Assistant (research deliverable):** the "Dating, Friendship, and Meetup Apps in 2025-2026:
Current Landscape and Fit with Drystone" report. **Filed verbatim** at
`alpha/research/dating-friendship-meetup-fit-2026-07.md`. Headline: meetup/community is the
recommended beachhead (roster/governance/auditable-counts map ~1:1 onto documented Meetup-under-
Bending-Spoons pain); dating's core loop is utility-shaped and a poor fit (matching/anonymity/
safety), a narrow trust-sidecar opportunity only after a real private layer. Every vendor number
attributed, not endorsed (see the report's Caveats).

---

## Thread 2 — Lexicon Community governance synthesis

**Assistant (grounded from their site, governance repo, and working-group threads):** the
lexicon.community governance-body synthesis — MIT-licensed `community.lexicon.*` namespace held by
a volunteer 7-member TSC; `GOVERNANCE.md`/`CONTRIBUTING.md`/`COVENANT.md`; Discourse-first,
interoperability-first working-group chartering; whimsically bird-named WGs (Polite Goshawk →
Lexicon Lenses); stewards calendar/location/bookmarks; lexicon.garden browser; a public numbered
incident report (Feb 2025); Gerakines' credible-exit transfer of the Smoke Signal schema out of
his own hands; the open authority/discovery question. **Filed** as the cairn stone
`beta/cairn/lexicon-community-governance.md` (orientation: related-ecosystem), grounded-from-site /
pending governance-repo verification.

**User:**
> That's awesome, note to research and cairn this.

**Assistant:** the cairn wants three stones — **the door** (their WG chartering process + the live
Attestation and Signatures thread, with Acudo as a named builder already there), **the seam**
(Lexicon Lenses as the formal shape of the envelope-to-calendar-record projection and the
versioning answer we'd otherwise invent), and **the resonance** (a seven-member TSC governing a
shared namespace by convention — governance-by-provenance's demonstration audience waiting to
exist). Deep-research follow-ups: the actual `GOVERNANCE.md` bylaws, TSC seat turnover, the full
attestation-thread state, and whether a Lenses reference implementation has shipped.

**User:**
> Can you give me a brief for engaging with that lexicon working group, shared concerns, proposal,
> evidence etc and see if there are Claude code experiments we could run that get us further along
> to a spec ready to show and tell.

**Assistant (engagement brief + experiment package):** the conversation already exists with a
published spec at its center (lexicon-community Discussion #8 "Attestation Lexicon"; Gerakines'
formal ATProtocol attestation spec + attested.network). Stance: not a rival proposal but
independent implementation + adversarial fixtures + one missing piece (issuer status without
verifier callback). Three planks (clean-room verifier + fixture corpus; stapled status as an
optional extension; WG-charter/thread revival). Experiment package **RUN-LEX-01** (five red-first
experiments). **All filed and EXECUTED** — brief at
`alpha/experiments/attest-family/ENGAGE-LEX.md`; run at
`alpha/experiments/lexicon-community/` (RUN-LEX-01-SUMMARY, AMBIGUITIES, BENCH-STAPLE,
LENS-SEAM-WORKED-EXAMPLE). Not re-embedded here.

**User:**
> What are the five calls so we can run the experiments?

**Assistant (the five open calls, EL OC-1…OC-5):** OC-1 revive Discussion #8 vs fresh Discourse
thread (→ fresh thread citing #8; gates the post). OC-2 name/placement of the freshness extension
(→ **staple** for the holder-presented record, **era head** for the issuer's published record,
avoid "status"; era head public in the issuer's repo; gates EXP-LEX-04). OC-3 what the demo
attests (→ **our own test event** on a real PDS, not a stranger's record; gates EXP-LEX-03's demo).
OC-4 TypeScript companion verifier (→ defer to a traction-triggered RUN-LEX-02). OC-5 who speaks
(→ **personal with affiliation**; user's call). Minimal ruling to start: confirm OC-2 + OC-3.
Verdicts folded into the run; **captured in ENGAGE-LEX.md** (§ open calls) — not re-embedded.

---

## Thread 3 — Kindred-work glossary (collaborators / reviewers / supporters)

**User:**
> We really need to start a glossary of potential collaborators, reviewers and supporters that are
> real people like Trevor with the context noted. Even a "aligned sibling thinking in x field" and
> a "wrote y discuss expression and implementation". We are going to need lots of broad feedback
> and I worry we are missing an opportunity to gather leads along the way, we should do this and
> include it in our normal discovery procedures.

**Assistant:** seed glossary — eleven people, seven organizations, each carrying a category tag +
an accurate "Kindred:" shared-concern line + possible role. Structural decision surfaced: the
discovery repo is public, so this is framed as an **influences-and-kindred-work ledger**
(respectful descriptions of public work only); outreach status/contact strategy live **outside**
the public repo, keyed by the same slugs. Procedure adopted as standing discovery practice
(append at capture time).

**User:**
> I think a public respectful external framing is fine, and this is a set of edges across tech,
> philosophy, social governance etc, and we will need the context to otherwise already public info
> and respectful yet accurate shared concerns.

**Assistant:** reframed the file as an **edge list** — each entry an edge labeled with the shared
concern; the "Kindred:" line an accurate, respectful statement of the overlap that never implies
the person knows of or endorses the project. Accuracy is the respect.

**Filed** verbatim at `beta/socialization/kindred-work.md` (with the procedure in its header).
Marked debts carried in that file: Berjon's ap-at piece URL; verify Schneider's online-governance
work before citing; fill the lexicon-community engagement-thread link.

---

## Thread 4 — croft.ing landing page (in flight, not filed by this session)

**User:**
> With more thinking beyond it but I want to work on the croft.ing landing page which is going in
> this repo: https://github.com/CroftCommunity/crofting_site . I am including a zip of the planning
> and discovery repo which should have the latest thinking on design and some visual artifacts.
> Let's make the initial crofting landing page, we'll talk it out then you give me a set of
> instructions for Claude code to execute.

Delivered as **`crofting.zip`** (10 planning docs: RUN-01/02/03 build instructions, page-copy
drafts, terms/treatise/arecipe-share, `coop-messaging-research.md`, and `kindred-work.md`). Frozen
as a seed (see RAW-ARTIFACTS-MANIFEST). The crofting/drystone "small plot of land tended with
care / no mortar / no VC glue" philosophy the user pasted is **already filed** (NAMING.md,
`beta/socialization/logo-drystone-stacking.md`, the `beta/history/` enclosure-inversion pair, the
naming + website-plot-tender raw dialogues) — a duplicate, not re-filed. The landing-page build
itself is a separate work stream against `CroftCommunity/crofting_site` (not this discovery repo);
its design sources-of-record already exist in `beta/socialization/` + `beta/croft/`.
