# Prompt: process Claude Code session logs — extract decisions & narrative, leave execution behind

Copy this into a fresh session to process pasted Claude Code transcripts. It is a **thin pointer** by
design — the real filing rules live in `discovery/AGENTS.md` and `discovery/PLAYBOOK.md`, which are the
source of truth. Follow them; do not re-derive them here.

---

You're in the CroftC workspace (`/Users/cpettet/git/chasemp/CroftC`). I'll paste **Claude Code session
transcripts** next — the logs of runs already executed against the repos. **Most of each session is
execution noise; a few carry inline thinking and decisions.** Your job is to **extract the decisions and
the narrative and leave the execution logs behind.** Don't start until I've pasted them.

ORIENT FIRST (read before processing — source of truth):
- `discovery/AGENTS.md` → "Reference indexes, filing & the backlog."
- `discovery/PLAYBOOK.md` — the filing process (classify → preserve raw → distill → update connective
  tissue), preservation statuses, header convention, commit rules (§3b).

## The load-bearing discipline: VERIFY AGAINST MAIN FIRST

Before claiming anything is done / merged / new / absent / pending, **check the actual current state of
`main` across the WHOLE repo** — `git ls-files | grep`, `git log --oneline | grep`, and read the file —
not one directory, not the pasted plan's own framing. **Almost every pasted Claude Code session is the
narrative of work that has ALREADY been executed and merged.** The plan-in-hand is usually a stale
snapshot; the spec/registers/thinking on main are the truth. So for every apparent "delta" or "proposal,"
the first question is: *is this already on main?* Branch names from the last `git pull` (`claude/*`) are
strong hints that a run landed. (This repo has been burned by the opposite assumption — a "pending" claim
for merged work drives the wrong next step.)

## The triage rule (what to keep, what to drop)

**DISCARD (already captured — do not preserve):** tool-call logs, red→green cycles, test output, file-edit
narration, commit mechanics, "running X / created Y / 191→199 tests." All of this already lives in the
run's `RUN-*-SUMMARY.md`, the git history, and the registers (`alpha/experiments/MASTER-INDEX.md`,
`EXPERIMENT-BACKLOG.md`, `SPEC-DIVERGENCE-REGISTER.md`, the spec status tags, `crystallized/proof-ledger.md`).
Re-preserving execution logs is noise.

**EXTRACT (the signal — file it):**
- **Decisions** the user made, with the **WHY** (not just the what). A naming choice → `NAMING.md`; an
  architecture/gate decision → `beta/DECISIONS.md` / a `thinking/` doc; a settled trade-off →
  `crystallized/principles.md`. Close the matching `ROADMAP_TODO` / `COHESION` item.
- **Narrative / design shifts** — a reframe, a corrected mental model, a design implication a run
  surfaced (e.g. "the browser is now a full sealed-tier client; the operator is a role not a villain;
  the PDS is cold storage with receipts"). These go in `thinking/` or `narrative/` — the "so what does
  this mean for the story/design" layer, which is the one that lags.
- **Naive-test-vs-spec deltas** — where a green rested on a stand-in, and whether the spec was already
  right (corroboration note only), genuinely missing a mechanism (a real fold), or an API/test artifact.
- **Owner-gated open calls** — surface them (into `ROADMAP_TODO` / the open-threads registers), never
  resolve them.
- **New prior-art / orgs / people / tools** named → `ECOSYSTEM.md` / a `cairn` stone / the
  `kindred-work.md` collaborator glossary (append at capture time, PLAYBOOK §3).
- **Quotable framings** and analogies worth keeping (attributed accurately; no synthesis-as-quote).

## Capture on BOTH axes

For each extracted item, check it is captured (a) **at the status level** (the right register/tag
reflects what landed — experiment RUNs → MASTER-INDEX/EXPERIMENT-BACKLOG; Proofs invariants →
proof-ledger; ecosystem → ECOSYSTEM/cairn; build state → `BUILD-INVENTORY.md`) AND (b) **integrated into
narrative/thinking** (the implication reached the design layer, not just the spec+registers). The second
is the usual gap. Fix pure status gaps inline; **surface narrative/thinking gaps for the owner rather
than writing narrative unilaterally** unless told to.

## Provenance (only for the genuinely-new signal)

- Preserve a **cleaned-paste raw** ONLY of the decision/reasoning/narrative that isn't already on main —
  never the execution logs. `seeds/transcripts/raw/<name>.md`, status
  `preserved-condensed (cleaned-paste, content-faithful — §4)`. Strip tool-call/UI chrome; if there's
  audio/TV bleed or transcription garble, remove it and mark `[removed]`. Header carries the §4 caveat.
- Mark volatile facts `[UNVERIFIED]`. For atproto/iroh/iOS facts, **cite the FACTCHECK source of truth,
  do not re-verify** (iroh is `1.0.0`).
- If a session comes with a **zip/attachment**: extract its contents to files and **verify byte-identical**
  (`diff -rq`) before it's deleted; freeze as a seed (`seeds/<name>-unpacked/`), retire the `.zip`, add a
  `RAW-ARTIFACTS-MANIFEST.md` row. The working zips are deleted after — nothing may depend on a `.zip`.

## Tooling gotcha (real, this environment)

Use **clean, direct greps**. A `for`-loop grep with a piped `head` under the command proxy misfires
(SIGPIPE → false `0` results). Before trusting any "not found / 0 files" result, **sanity-check the tool
against a file you know contains the term.** Never report a coverage gap from a coarse grep.

## Rules

- Git identity: chasemp (`chase@owasp.org`, `github-personal`). Repos: `discovery` (most filing),
  plus `arecipe` / `skylite` / `experiments` / `Proofs` — commit each separately, match the right identity.
- **Show me the filing plan first** — per session: a one-line triage verdict (execution-only → discard;
  or what decisions/narrative you extracted and where each lands), a dedup note (already-on-main vs
  genuinely-new, verified), and any index rows you'll touch — and **wait for my commit approval**
  (PLAYBOOK §3b). Push only when I ask.
- Don't resolve my open decisions; surface them. When most of a session is dupe, say so plainly and move
  on — the win is the small extracted signal, not a big new artifact.
