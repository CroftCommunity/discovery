# Intake Playbook: processing incoming narrative, experiments, and proofs

date: 2026-06-15

purpose: a repeatable process for folding new material into the CroftC repo set the same way
every time, so the corpus stays coherent, provenance stays intact, and nothing is silently
lost. Follow this whenever a new transcript, dossier, PR, or piece of research arrives.

This codifies what has been done by hand so far; keep it current as the process evolves.

---

## 0. The repo set (where things live)

```
CroftC/                       (not a repo; holds the repos + .claude orientation)
├── discovery/    thinking & synthesis: seeds, research, thinking, crystallized,
│                   narrative, ROADMAP, COHESION, ECOSYSTEM, this playbook
├── Proofs/       durable proofs — verify an invariant → becomes a design principle
└── experiments/  code-forward spikes — "does this work / what's actually true?"
```

Git identity on all three: chasemp account — `git@github-personal:CroftCommunity/<repo>.git`,
committer `Chase Pettet <chase@owasp.org>`. Set it on any fresh clone.

## 1. Classify the incoming material

Decide what it is — this determines where it lands:

- **Narrative / thinking** (dossier, design dialogue, vision, civic/philosophy) → `discovery`.
  Raw goes to `discovery/seeds/`; distilled outputs to `thinking/`, `crystallized/`,
  `narrative/`.

- **Industry research / comparison** (competitive analysis, field survey) →
  `discovery/research/`. Related to but distinct from `ECOSYSTEM.md` (relational register) —
  see `research/README.md`.

- **Proof** (durable, hypothesis-driven, validates an invariant that becomes a principle) →
  `Proofs/`. Watch for **mixed-in experiments** inside a proof (e.g. a pending live spike) —
  flag them, don't relabel the whole thing.

- **Experiment / spike** (code-forward, exploratory, "is this reachable / what's true") →
  `experiments/`.

When unsure between proof and experiment: a **spike** answers "does it work?"; a **proof**
answers "does this invariant hold such that we can build a principle on it?" If it is
hypothesis-driven with falsifiable invariants and a real/modeled validation, it is a proof.

## 2. If it is a PR (the common case)

The repeatable PR-import sequence (croftc PRs use the `cpettet_croftc` gh account; the
destination CroftC repos use the chasemp SSH host):

1. **Read it** — `gh pr view <N> --repo <org>/<repo> --json number,title,state,headRefName,
   additions,deletions,changedFiles,files`. Note the branch and the directory shape.

2. **Clone the head branch** — `gh repo clone <org>/<repo> /tmp/<name> --
   --branch <headRef> --depth 1`.

3. **Place the code** — copy the relevant subtree into the right repo/dir. **Exclude
   upstream plumbing** that would activate or mislead here: `.github/workflows/*`,
   `renovate.json`, and anything CI-specific to the source repo. Note exclusions.

4. **Verify verbatim** — `diff -rq <clone-subtree> <placed-dir>` excluding the files you
   added; an empty diff means the code is byte-identical. Record it.

5. **Scan for secrets / binaries** — grep for passwords / private keys / tokens (allow API
   *names* like `SecretKey`); confirm no committed `.so`/large binaries; confirm only
   `.env.example` templates, never real creds. Redact any in-session credentials.

6. **Capture the conversation** — write `PR-CONVERSATION.md` next to the code from `gh`
   (description + issue comments + reviews + inline `gh api .../comments --paginate`). This is
   verbatim provenance.

7. **Capture the coding transcript** — if a transcript was provided: save the **verbatim raw**
   to `discovery/seeds/transcripts/raw/pr<N>-<name>.md` (redact creds; reference, don't
   triplicate, briefs already saved verbatim elsewhere), and a readable **condensed**
   `CODING-TRANSCRIPT.md` next to the code.

8. **Carry forward findings** — note any unresolved CI/license gate, review finding, or
   PII/security flag (e.g. a logging-PII warning) so it is not lost.

**Binaries & fixtures:** reject committed build artifacts (`.so`, `target/`). Keep *modest*
test fixtures that aid reproducibility (e.g. a ~760KB `sample-photo.png` for a media demo) and
note them in the import header. Flag anything large or unexplained instead of importing
silently.

## 2b. If it is a non-PR transcript, dossier, or research deliverable

Not everything is a PR. Design dialogues, research write-ups, and dossiers arrive as pasted
text. For these:

1. **File the raw** to `discovery/seeds/transcripts/raw/<name>.md` (or `seeds/<name>.md` for a
   dossier/zip). Redact in-session credentials; reference — don't triplicate — any large brief
   already saved verbatim elsewhere. Two fidelity cases:
   - **Verbatim** when you have the exact source (a file, a clean export). Status
     `preserved-verbatim`.
   - **Cleaned-paste** when the source is a chat the user *pasted into the session* and no
     canonical export exists (the common case for claude.ai design dialogues). Reproduce the
     dialogue **content-faithfully** — strip only UI render chrome ("Searched the web", "Fetched:",
     "Ran a command", "Document · MD", "Show more", date/time dividers, "Presented N files") and
     bracket any mid-paste blocks that were themselves truncated, as pointers to the real artifact.
     The header **MUST** carry the §4 caveat: "content-faithful, not a byte-pristine export." Status
     `preserved-condensed (cleaned-paste)`. This is the accepted best-available raw when §4's "request
     the canonical copy" isn't possible because the user has already said the paste is all there is.

2. **Distill** into the right home: a research deliverable → `research/`; new design thinking →
   a `thinking/<topic>.md` and/or new principles in `crystallized/principles.md`; civic/vision →
   the dossier or a `narrative/verticals/` piece. For a coherent multi-doc body (e.g. the app/client
   layer), use a topic subdir (`thinking/app/`, with `build-specs/` and `ponds/` clusters under it).

3. **If a source archive (zip) is provided:** unpack to `seeds/<name>-unpacked/` as the **frozen
   verbatim seed**, and place **working copies** in the right `thinking/` home. Before retiring the
   archive, extract a fresh copy and `diff -rq` it against the frozen seed to confirm **byte-identical**.
   Only then retire the archive (contents preserved in the seed). **The user must authorize removal.**

4. Then do the same §3 corpus-coherence updates.

## 2c. When the user makes a decision

Capture it as a durable artifact, don't leave it only in chat. A naming choice → `NAMING.md`;
an architecture choice → an ADR-style note or a `thinking/` doc; a settled trade-off → a
`crystallized/principles.md` entry. Cross-link it, and close the corresponding open item in
`ROADMAP.md` / `COHESION.md` (e.g. the "pin the name map" item closed by `NAMING.md`).

## 3. Always-do: keep the corpus coherent

After placing anything, update the cross-repo connective tissue:

- **`discovery/crystallized/proof-ledger.md`** — add/adjust the I/E/V/S rows and their status
  (`green-real` / `green-model` / `spec` / `blocked`), linking to the proof.

- **`discovery/crystallized/test-narrative.md`** — for every test, add a narrative entry: **Why**
  we ran it · **Tells us** (the result) · **Means** (implication for the thesis/design) · **Open
  edges** (edge cases + next testing it surfaces). The ledger is status; this is the reasoning so
  we never mistake "green" for "done." A proof without a narrative entry is not fully filed.

- **`discovery/COHESION.md`** — the most important step. Ask: *does this material close a
  loose end another doc declared, or does it duplicate/contradict another?* Add a numbered
  entry mapping the loose-end ↔ the work that addresses it, marked CLOSED / OPEN / DRIFT /
  DUPLICATION. Backport surfaced findings into the thinking docs they affect.

- **`discovery/ROADMAP.md`** — fold any new "next to do," milestone, or feature-state change
  into the rough roadmap. **`discovery/ROADMAP_TODO.md`** is the single provenance-indexed
  **backlog** — add new open items *there* (with origin `file:line` + a durable section header),
  never start a parallel list; ROADMAP carries the reasoning, ROADMAP_TODO aggregates.

- **The relevant `README.md`** (repo-level) — add the new artifact to its contents list.

- **`discovery/seeds/transcripts/RAW-ARTIFACTS-MANIFEST.md`** — record what raw artifact came
  in and its preservation status (verbatim / condensed / distilled-only / missing).

- **`ECOSYSTEM.md`** — if the material names a new org/project/tool, add or update its row
  (org · project · purpose · capabilities · current state · relationship tag). Rows distilled from a
  dialogue but **not independently verified this session** must be flagged dialogue-sourced /
  pending-verification (see §5c/§5d for the pattern) — don't launder a model's claim into a
  verified-looking row.

- **`.claude/CLAUDE.md`** (top-level, and per-repo READMEs) — if the structure or the headline
  state changed (a new repo, a resolved gate, a name decision), keep the orientation current so
  a fresh agent lands correctly.

## 3b. Committing (when the user asks)

This repo set is reviewed before commit, so commit only on request. When asked:

- Use the chasemp identity already set on each repo (`Chase Pettet <chase@owasp.org>`).

- Commit each of the three repos separately (they are independent repos). Stage everything new
  + modified. Confirm no secrets/large build artifacts are staged before committing.

- End commit messages with the required co-author trailer:
  `Co-Authored-By: Claude Opus 4.8 (1M context) <noreply@anthropic.com>`.

- **Don't push or open PRs** unless separately asked — committing locally is enough to persist
  across a context clear.

## 4. Provenance discipline (non-negotiable)

- **Keep raw.** Every incoming transcript/dossier/PR is preserved verbatim somewhere (raw
  archive, `PR-CONVERSATION.md`, the seed file). Condensed renderings are additive, never a
  replacement. If exact fidelity can't be guaranteed (e.g. reconstructing a huge paste from
  memory), say so and request the canonical copy rather than producing a lossy "raw" file.

- **Redact secrets, preserve everything else.** Credentials are stripped; the surrounding
  text stays verbatim. Note the redaction inline.

- **Mark uncertainty.** Volatile facts (versions, dates, "current state") get `[UNVERIFIED]`
  until confirmed against a primary source. Distinguish protocol-level from product-level
  facts, and modeled-vs-real for proofs.

- **Don't over-claim.** A `green-model` proof is not `green-real`. State what a validation
  does *not* establish.

- **Cite the fact-check, don't re-derive; check for drift.** For atproto / iroh / iOS facts, the
  source of truth is `seeds/transcripts/raw/atproto-atmospheric-web-iroh-mobile-FACTCHECK.md` — cite
  it. When distilling a model-generated dialogue, run a quick drift grep over the new content for
  known-corrected errors (e.g. `rc.0`, `connect_to_peer`, "merkle search tree", "AT Messaging working
  group") so a superseded claim isn't reintroduced. Align to the corrections (iroh `1.0.0`; iroh-docs
  = range-set reconciliation + LWW; no native AT-Proto E2EE — third-party only).

## 5. Don't, by default

- Don't commit to git unless asked (this repo set is reviewed before commit).

- Don't run workflows / push / open PRs on the CroftC repos without explicit ask.

- Don't relabel a whole proof as an experiment (or vice-versa) because one part differs —
  flag the mixed part instead.

- Don't merge `ECOSYSTEM.md` and `research/` — they overlap on purpose, different audiences.

- Don't auto-resolve a flagged compliance/security/social decision that is the user's to make
  (e.g. the MPL-2.0 license gate) — surface it.

## 6. Quick checklist (per incoming item)

```
[ ] classified (narrative / research / proof / experiment)
[ ] placed in the right repo/dir; upstream plumbing excluded
[ ] code verified verbatim (empty diff) — if PR
[ ] secrets/binaries scanned; creds redacted
[ ] PR-CONVERSATION.md captured (verbatim)
[ ] raw transcript archived + condensed CODING-TRANSCRIPT.md (if provided)
[ ] proof-ledger.md updated
[ ] COHESION.md updated (loose-ends / duplication / drift + backport)
[ ] ROADMAP.md updated; open items added to ROADMAP_TODO.md (origin file:line, no parallel list)
[ ] repo README.md updated
[ ] RAW-ARTIFACTS-MANIFEST.md updated
[ ] ECOSYSTEM.md updated (if new org/project)
[ ] .claude / READMEs updated (if structure or headline state changed)
[ ] decision captured as a durable artifact (if the user made one)
[ ] source archive retired only after byte-identical verification (if applicable)
[ ] carried-forward findings + open decisions noted
```

## 7. Current state pointers (keep fresh)

- **Name center of gravity:** Croft (`NAMING.md`). Org `CroftCommunity`.

- **Headline proof result:** lineage-groups Phase 1 crypto gate is GO on real openmls 0.8.1.

- **Top open problem (protocol):** multi-device + total-device-loss recovery
  (backup-vs-delegation fork).

- **Active body (2026-06-22): the app / client layer** ("Croft" the product) — a composable garden
  of **ponds** (Bluesky/Mastodon/Lemmy, native) + **pads** (small apps), with the **Croft Group**
  pond = lineage-groups surfaced on iroh. Lives in `thinking/app/` (core design + `build-specs/` +
  `ponds/` catalog). Two make-or-break open items (in `ROADMAP_TODO.md`): the **deep-link resolver**
  (tier-zero: core UX unlock + the only acquisition path) and the **sustainability ↔ cooperative
  *mechanism*** (existential). Phase 0 (functional core + CLI) was built in **CroftC PR #10** —
  import deferred, the IP/ownership call is the user's (ROADMAP_TODO A8); surface, don't resolve.

- **Imported so far:** Proofs = lineage-groups (#8), lineage-group-model (#9),
  encrypted-local-first-atproto (#3). experiments = appview-validation (#6), public-roundtrip
  (#4), android-p2p-app (#7), encrypted-blob-share (#5).

- **Provenance:** complete. The GroupDynamics design dialogue is filed verbatim; the two app/client
  dialogues (design + ponds/games) and the atproto-atmospheric/crofting material are filed as
  **cleaned-paste** (content-faithful, §4-labeled — no pristine export existed). Source of truth for
  atproto/iroh/iOS facts: `seeds/transcripts/raw/atproto-atmospheric-web-iroh-mobile-FACTCHECK.md`.
  No known gaps.

- **Agent orientation:** canonical at `discovery/AGENTS.md` (version-controlled); the top-level
  `CroftC/.claude/CLAUDE.md` imports it.
