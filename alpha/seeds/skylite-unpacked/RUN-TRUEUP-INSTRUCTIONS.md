# RUN-TRUEUP — Claude Code instructions (self-contained)

You are executing a correctness, care, and documentation run on
`CroftCommunity/skylite` (the Skylite PWA, skylite.croft.ing). It implements
the owner's rulings from the 2026-07-15 repo audit. No new product surface;
every change trues up something that already shipped.

## Conventions (non-negotiable)

1. TDD, always: each phase's acceptance criteria become FAILING tests before
   implementation; red-to-green order evidenced in RUN-TRUEUP-SUMMARY.md.
2. Fresh branch `run-trueup`; no pushes to main; PR + summary.
3. Hermetic gate green at every phase (lint · typecheck · unit · build · e2e);
   CI never touches the network; no credentials in-repo.
4. No new dependencies without listing + reason. Expect zero.
5. READ-ONLY, byte-frozen: CONCEPT.md, IDEAS.md, PROVENANCE.md, seeds/, and
   ALL existing RUN-*-SUMMARY.md files (they are historical record; a
   consistency pass never rewrites history). EXCEPTION, this run only:
   README.md may be updated per Phase 6.
6. Copy blocks marked [confirm before publish] are carried verbatim; lay out,
   never rewrite.

## Phase 0 — Tagline ruling (owner, 2026-07-15)

The tagline "A window to the stars." is replaced by **"A butterfly garden
window."** everywhere it appears on LIVING surfaces: the landing hero line,
the index.html meta description, the web-manifest description, and any
sponsor/help copy that echoes it. CONCEPT.md keeps the old tagline untouched
(historical, rule 5). The rest of the landing copy remains [confirm before
publish] pending the owner's line-by-line; this one line is now ruled.

Test-first: extend the landing spec to assert the new tagline; lint-grep
test failing on "window to the stars" in src/ and living docs.

## Phase 1 — Archive metadata true-up

The sealed search records currently expose precise `createdAt` timestamps
publicly; existence, count, and timing are readable by anyone even though
content is sealed.

- Move the precise timestamp INSIDE the ciphertext: add `at` (epoch ms) to
  the sealed payload (`{q, blocked, tier, at}`).
- Round the record-level `createdAt` to DAY granularity (UTC date at
  00:00:00Z). The sponsor's decrypted timeline uses the inner `at`.
- Older records without inner `at` fall back to the record `createdAt` when
  decrypting (tolerant read).
- Soften the explorer copy: "Your sponsor can read what you search here. It
  is stored scrambled, so no one else can read what you searched." (Note the
  scope: what, not whether/when.)
- docs/telescope-search.md gains a plain metadata paragraph: record
  existence, count, and day are public; content, blocked-status, tier, and
  precise time are sealed.

Test-first: unit test asserting built records carry day-granular `createdAt`
and a sealed payload containing `at`; decrypt path test covering both old and
new payload shapes.

## Phase 2 — Care-aware refusal (self-harm category)

The blocklist rightly refuses self-harm queries; the refusal moment must
open a door, not just close one.

- Give blocklist entries categories (the seed list already groups them in
  comments; make the self-harm group programmatic). Policy verdicts for
  blocked queries carry the category.
- When a blocked query is in the self-harm category, the Telescope refusal
  renders a gentle supportive panel with the EXISTING get-help handoff
  control (the RUN-05 care feature), instead of the generic "that search
  isn't available" line. No shame framing, no clinical language, no lecture.
- Copy v1 [confirm before publish — every line]:

  > Some things feel too heavy to carry alone. Your sponsor cares about you
  > and wants to hear from you — this button reaches them right away.
  > [ Get help ]

- Logging is unchanged: the attempt is logged/archived exactly like any
  other blocked query.
- All other blocked categories keep the generic refusal.

Test-first: hermetic e2e — a self-harm-category query shows the supportive
panel with the help control and no results; a generic blocked query shows
the plain refusal and no panel; both appear in the history list as blocked.

## Phase 3 — Vocabulary: "grown-up" → "sponsor"

The role vocabulary is role-based, not age-based; "your grown-up" re-imports
the age framing and reads wrong for an elder explorer.

- Sweep all explorer-facing copy: "your grown-up" → "your sponsor"
  ("Recent searches (your sponsor can see these)", the archive note per
  Phase 1, any care/help copy variants like "trusted adult" where it means
  the sponsor role).
- Historical docs and run summaries are NOT swept (rule 5). docs/ living
  files are.

Test-first: extend the existing copy assertions in the telescope/care specs
to the new strings; add a lint-style grep test failing on "grown-up" in
src/ UI-copy string literals.

## Phase 4 — Retroactive RUN-SEARCH-SUMMARY.md

The search-system phases (the PRs referenced as #19–#23 in
docs/telescope-search.md) shipped without a run summary. Write
RUN-SEARCH-SUMMARY.md reconstructing it from git history: what shipped per
phase (gradient, gates, logging, sealed box, vault, config exchange, sealed
write, audit decrypt, PRF wiring), the red-to-green evidence per phase read
from the actual commit sequence (cite commit hashes; where a test landed
with its implementation rather than before it, say so plainly rather than
retrofitting a claim), dependencies (none), and the verify-in-run items it
left open. Mark it clearly as written retroactively and dated.

## Phase 5 — Encryption language true-up

- Fix the sealed-box comment: fresh ephemeral keys mean a leaked ephemeral
  key exposes only its own message; compromise of the sponsor's PRIVATE key
  decrypts the entire archive — that is what the vault (WebAuthn PRF /
  passphrase wrap) protects. State it exactly so.
- Owner ruling on user-facing language: sponsor/explorer copy MAY say
  "bank-grade encryption" (AES-256 + P-256, the same primitives as banking
  TLS). The precise scheme lives in docs/telescope-search.md. HARD RULE: no
  copy anywhere may claim "unbreakable", "impossible", or "no one can ever";
  the honest shape is "so no one else can read what you searched."
- Apply the phrase where the archive is explained in the sponsor UI
  (enable-archive copy) and audit page.

Test-first: extend the copy assertions; lint-grep test failing on
"unbreakable"/"impossible to" in src/ string literals.

## Phase 6 — Documentation correctness & consistency pass

Scope: LIVING documents only — README.md (exception granted this run),
docs/custody.md, docs/telescope-search.md, docs/git-verified-commits.md,
lexicons/ descriptions, in-app help/how-it-works copy, sponsor onboarding
copy. Historical artifacts (CONCEPT/IDEAS/PROVENANCE/seeds, run summaries,
plans/) are untouched.

- README.md: add a "Skylite today" section at top — what Skylite is (the
  landing's own framing), the live URL, the run-based build process, where
  the docs live — and clearly mark the original idea-capture content below
  it as the historical seed (preserved verbatim under a heading, not
  deleted).
- Sweep living docs for: dead vocabulary (guardian/custodian/viewer/child
  where it means the roles; scrapbook), stale claims (features described as
  staged that have shipped, and vice versa — reconcile docs/telescope-search.md's
  Status section with reality), the Phase 1 metadata paragraph, the Phase 5
  language rules, and internal cross-references (docs pointing at files that
  exist, correct NSIDs, correct page paths).
- Consistency check the three lexicon docs against the code (config fields,
  like/follow/search record shapes) and fix the DOCS to match the CODE;
  never change a shipped record shape in this run.
- Summary lists every file touched with a one-line reason.

Test-first where testable: the lint-grep vocabulary test (Phase 3) covers
src/; for docs, include a scripted check (extend the existing test tooling)
asserting the banned vocabulary is absent from docs/ living files and that
every path referenced in README/docs exists in the tree.

## Summary requirements (RUN-TRUEUP-SUMMARY.md)

Red-to-green evidence per phase; the old→new record shape and its tolerant
read; screenshots-in-words of the two refusal states; the vocabulary sweep
counts; every doc touched with reason; any [confirm] copy still pending;
confirmation that CONCEPT/IDEAS/PROVENANCE/seeds and all prior summaries are
byte-identical to before the run.
