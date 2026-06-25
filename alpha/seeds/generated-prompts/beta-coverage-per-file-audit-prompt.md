# Prompt: per-file alpha→beta coverage audit (close the coverage list to zero)

Copy this into a **fresh session after `/clear`**, ideally on the **highest-capability model available**.
This is a **per-file** completeness audit: walk *every* file in `discovery/alpha/` and give it an explicit,
evidence-backed disposition against the `discovery/beta/` synthesis — so nothing settled was dropped,
nothing unsettled was lost, and every alpha file is accounted for. It is deliberately more exhaustive than
the surface-grouped content sweep that preceded it (a grouped skim is exactly how coverage gaps slipped
through before).

---

You're in the CroftC workspace (`/Users/cpettet/git/chasemp/CroftC`), `discovery` repo. Git identity:
chasemp (`chase@owasp.org`, `github-personal`). **Do not commit or push unless I ask.** Do not resolve any
surfaced decision gate (MPL license, recovery anchor, cooperative legal review, the Noria name, CroftC
Phase-0 IP, genome-vs-strategy, capability-mechanism Track A/B, key-custody default). Surface, don't decide.

## Current state (so you don't re-do or re-surface)

- The maturity model is `alpha → beta → rc → publish`; the working corpus is `alpha/`, the resolved
  synthesis is `beta/` (eight theme docs `01`–`08` + `beta/README.md`).
- **Tier-cleanliness rule (just enforced):** a beta doc carries **no references back to alpha** — no
  `Sources (alpha)` footer, no provenance trace, no `thinking/…`/`research/…` quote attributions, no
  `COHESION §`/`BETA-ROLLUP` pointers, no `../alpha/…` paths. The full source→landing map lives **only** in
  `alpha/BETA-ROLLUP.md`. The discipline tightens upward (rc/publish shed even the residual verification
  markers). Full rule: `MATURITY-ROLLUP.md` §1/§3/§3a; `beta/README.md` "Tier discipline."
- **Already done (treat as known state; verify but don't re-surface as new):**
  - Twelve dropped settled conclusions **K1–K12 were folded** into beta on 2026-06-25 (landings recorded
    in `alpha/BETA-ROLLUP.md` → "Settled conclusions not yet folded" table). `crystallized/principles.md`
    Tier 1 is now drained.
  - Seventeen unsettled threads **T1–T17 are staged** in `beta/OPEN-THREADS.md` (not folded — correctly,
    they're unsettled).
  - Two lower-priority items remain intentionally unfolded (user-need-first / Google+; the
    "type at creation" MLS-coherence rationale) — see the ledger.

## Orient first (read before auditing — these are the rubric)

- `discovery/AGENTS.md` → "Maturity stages" + "Tier cleanliness."
- `discovery/PLAYBOOK.md` → filing model.
- `discovery/MATURITY-ROLLUP.md` → §1 principles, §3/§3a (no prior-tier refs; graduated strictness), §3b
  (the open-threads ledger pattern), §2 (per-doc template).
- `discovery/beta/README.md` → the eight themes, tier discipline, the per-doc template.
- `discovery/alpha/BETA-ROLLUP.md` → the canonical alpha→beta map: per-theme source tables, the K1–K12
  fold table, and the **"Coverage view — what is dispositioned, and what is not."**
- `discovery/beta/OPEN-THREADS.md` → T1–T17 (so you don't re-stage them).

## The job — disposition EVERY alpha corpus file

Enumerate every file under `discovery/alpha/` (use `find discovery/alpha -name '*.md'`; also note `app/`,
`drystone-spec/`, `verticals/`, `seeds/`). For **each file**, assign exactly one primary disposition (a
file may carry a secondary note), with **evidence**:

1. **FOLDED** — its substantive conclusions live in beta. **Cite the beta theme + section** and, **adversarially**, confirm the content is actually there (do NOT trust `BETA-ROLLUP`'s tables — the ledger was caught overstating Tier-1 coverage; open the named beta section and check).
2. **CITED-NOT-FOLDED** — beta relies on it as a source/SoT but correctly does not lift it verbatim (e.g. `crystallized/proof-ledger.md` / `test-narrative.md` / `CROFT-PROTOCOL.md` / `conformance-suite.md` / `TEST-CORPUS.md` back 04's flags; the atproto/iroh FACTCHECK is the cited SoT). Alpha-only **by design** — not a gap.
3. **PROVENANCE / RAW** — a `seeds/transcripts/raw/*` source or `generated-prompts/*`; stays alpha as frozen provenance — not a gap.
4. **PROCESS / INDEX** — connective-tissue/working surface (`COHESION.md`, `ROADMAP.md`, `ROADMAP_TODO.md`, `ECOSYSTEM.md`, `NAMING.md`, `ANALYSIS.md`, `TEST-PLAN.md`, `RAW-ARTIFACTS-MANIFEST.md`, `BETA-ROLLUP.md` itself, `plans/*`); stays alpha **by design** — not a gap.
5. **STAGED** — its beta-bound content is an open thread (T1–T17) not yet landed; cite the `Tn`.
6. **EXCLUDED (do-not-carry)** — deliberately not carried; confirm it is genuinely ABSENT from beta (appearing only in an excluded/REFUTED trace is fine). The do-not-carry set incl.: the "ancient free clan" myth; the REFUTED crypto-wars fabrications; the MST/iroh-docs conflation; the fictional "AT Messaging working group"; `did:key` atproto-resolvability; `did:plc`="Public Liaison Corporation"; the MO §351 statute/fee/$ specifics; over-claimed "serverless"; the "Equivalency Assertion" label; Steem HF23=$5M / Farcaster rent ~$5.
7. **GAP — CONCLUSION-MISSING** — a **settled** conclusion that belongs *in* a theme but isn't there (a new coverage gap, like K1–K12 were). This is the priority find.
8. **GAP — THREAD-MISSING** — an **unsettled** beta-bound item not yet in `OPEN-THREADS` (a new `Tn` candidate).
9. **DEFERRED** — real alpha material acknowledged as not-yet-pulled (e.g. 01's "objection-by-objection toolkit"); confirm it's tracked.

Be adversarial on FOLDED specifically. For any file you mark FOLDED, name the exact beta `file:§` and quote
or paraphrase the landed content; if you can't find it, it's a GAP, not FOLDED. Watch for **partial folds**
(the headline landed but a load-bearing nuance/sub-conclusion was flattened — that's a CONCLUSION-MISSING at
sub-file granularity, like the meer "confidentiality dial" was before K8).

## Method

Fan out by subtree, one agent each, then synthesize a master matrix: (a) `crystallized/`; (b) `thinking/`
(top level); (c) `thinking/app/` + `thinking/drystone-spec/`; (d) `research/`; (e) `narrative/` + `verticals/`
+ `SOVEREIGN-COMMONS-DOSSIER.md`; (f) top-level index/process files + `seeds/`. Each agent reads its files in
full (or skims index/raw appropriately), checks representation against the eight beta themes +
`OPEN-THREADS` + `BETA-ROLLUP`, and returns a per-file disposition table with evidence. **Model rule
(standing, all Croft work): every subagent runs on the *same model as this session's primary model* — set
`model` explicitly on each Agent call; never let a subagent downgrade.**

## Output and actions

1. Write a **per-file coverage report** to `discovery/alpha/plans/<YYYY-MM-DD>-beta-coverage-per-file-audit.md`:
   a master matrix (one row per alpha file: path · disposition · evidence/beta-location · action), then the
   two gap lists (CONCLUSION-MISSING, THREAD-MISSING), each ranked.
2. **Safe, additive actions you may take directly:** stage new THREAD-MISSING finds in
   `beta/OPEN-THREADS.md` (as `T18+`, same schema), record new CONCLUSION-MISSING finds in
   `alpha/BETA-ROLLUP.md`'s coverage section, and **update the "Coverage view"** so it reflects true state —
   close each listed alpha source to either "folded → beta §" or an explicit "alpha-only by design (reason)."
   The goal is the coverage list closed to zero-or-explicitly-alpha-only.
3. **Do NOT silently edit the beta theme docs.** Folding settled conclusions into the resolved themes
   changes the synthesis — **present the CONCLUSION-MISSING fold candidates and wait for my approval**,
   then fold per-theme as clean beta narrative (no prior-tier links; verification flags only where the
   source has them), exactly as K1–K12 were done.
4. Re-verify **tier-cleanliness** after any beta edit: `grep -rE "\.\./alpha/|COHESION §|## Sources \(alpha\)|## Provenance trace|> — \`(thinking|research|crystallized|narrative)/" discovery/beta/0*.md` must return nothing.
5. Don't commit/push unless I ask; surface gates, don't resolve them.

## Definition of done

Every file under `discovery/alpha/` appears exactly once in the master matrix with a disposition + evidence;
all FOLDED claims independently verified against the named beta section (not trusted from the ledger); new
gaps recorded (threads staged, conclusions listed for approval); the `BETA-ROLLUP` coverage view closed to
zero-or-explicitly-alpha-only; tier-cleanliness re-confirmed; no gate resolved; nothing committed unless I
ask.
