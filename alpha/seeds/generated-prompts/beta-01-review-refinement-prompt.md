# Prompt: process the verbal review of beta theme 01 → refine `beta/01-epistemic-foundation.md`

Copy this into a **fresh session after `/clear`** (ideally the highest-capability model available). It is
self-contained on purpose — context was cleared to keep it clean, so everything you need to start is here.
The job is to turn a **voice-transcribed read-through review** of beta theme 01 into a structured,
classified refinement plan and then apply the approved changes to the beta doc. **Do not skip straight to
editing** — the transcript is stream-of-consciousness speech; extract and structure first.

---

You're in the CroftC workspace (`/Users/cpettet/git/chasemp/CroftC`), `discovery` repo. Git identity:
chasemp (`chase@owasp.org`, `github-personal`). **Do not commit or push unless I ask.** **Do not resolve any
surfaced decision gate** (MPL/AGPL & CC0 licenses, recovery anchor, cooperative legal review, Noria name,
CroftC Phase-0 IP, genome-vs-strategy, capability-mechanism Track A/B, key-custody default). Surface, don't
decide.

## The artifact

`discovery/beta/thinking/01_beta_review.txt` — a **voice-transcribed (Otter/whisper-style) read-through
review** of `beta/01-epistemic-foundation.md`, spoken by the user (labeled `[Speaker 1]`), reading the theme
top-to-bottom and giving verbal refinement notes "for Claude refinement." ~298 lines / ~30 KB. It is the
user's own editorial pass on a settled beta doc, so its notes are **authorized refinement direction** — not
an external opinion to weigh.

**It is stream-of-consciousness dictation. Before doing anything, normalize and de-dupe:**

- **Transcription errors to read through** (non-exhaustive — infer from context): "dry Stone" / "dry stone" =
  **Drystone**; "equine rights" / "equal inner rights" = **equal in rights**; "octave shapes" / "a octave
  shapes" = **all shapes** (the "equal possibility of all shapes" lift); "one print" / "in prints" / "one
  principal" = **in parentheses / a principled boundary** (disambiguate by context); "nun" = **none**;
  "products" (in "compute products and over utility") = **provenance** ("compute provenance, not utility");
  "principal" = **principled** where it modifies "boundary". `[Speaker 1]` tags and repeated false starts are
  dictation chrome — strip them.
- **The same note recurs** across paragraphs (the user circles back). Collapse duplicates into one item.

## Orient first (read before planning — these are the rubric)

- `discovery/AGENTS.md` → "Maturity stages" + "Tier cleanliness."
- `discovery/MATURITY-ROLLUP.md` → §1 (no prior-tier refs; graduated strictness), §2 (per-doc template).
- `discovery/beta/README.md` → tier discipline + the per-theme doc template.
- `discovery/beta/01-epistemic-foundation.md` → **the doc being refined** (read it in full).
- `discovery/alpha/BETA-ROLLUP.md` → how 01 was built; the K-table (K1–K17). **Note especially K17
  (2026-06-26):** the rights-vs-capability grounding was just folded into **01 §5** (the discriminating test,
  the four rights tenure/exit/voice/share by-removal, the voice-vs-amplification cut). The review predates or
  overlaps this — **reconcile, do not duplicate** (the review's "we have to define what the rights are"
  comment is partly already addressed by K17; check before adding).
- `discovery/beta/OPEN-THREADS.md` → **T21 (`share`-as-right) / T22 (`tenure`-vs-04-survivor-rekey)** are the
  two open checks on 01 §5's rights set; don't re-open them, reference them.

## The job

1. **Extract every discrete refinement note** from the transcript into a structured list, in document order,
   each tied to the **specific 01 location** it addresses (§ or quoted phrase). De-dupe.
2. **Classify each note** so the change set is reviewable:
   - **(W) wording / copy-edit** — phrasing, a deleted line, a clarified sentence. Low-risk, apply directly.
   - **(S) structural reframe** — changes how a claim is framed (see the node-vs-system item below). Medium —
     list the before/after.
   - **(C) content / claim change** — alters what 01 *asserts*. Highest — present for my confirmation before
     applying; this is a settled beta doc.
   - **(F) formatting** — parentheses, emphasis, callout structure.
   - **(D) defer to a separate pass / doc** — the review proposes new scope (e.g. a separate "historical
     alignment" document); record it, don't force it into 01.
3. **Produce the plan first** (the classified list + proposed edits, with W/F applied-inline-able and S/C/D
   surfaced), and **stop for my approval** before editing 01. Then apply per the maturity discipline below.

## Notes already visible in the transcript (seed the extraction — confirm against the full text, don't treat as complete)

- **(S) Node, not system, is the actor.** Recurring: "a distributed system can only establish provenance" →
  "**a node in a distributed system** can only establish provenance"; the system *facilitates nodes having
  the necessary view." This reframes the razor's subject and recurs through §1/§2/§6.
- **(C/S) "distributed system" vs "network" are intermixed — make consistent or define both.** Flagged
  twice. The phrase "the network reconciles" is called out as conflating; prefer node-centric phrasing and a
  one-line definition (a distributed system = nodes communicating remotely about some common truth; truth is
  local and corroborated, never certified from a center).
- **(W) Drop the "none arguing with the others" line** in the §2.6 convergence passage ("that's weird").
- **(C) "name resolution"** in the premise-propagation list ("identity, history, governance, federation, and
  even name resolution") — the user is unsure it belongs; maybe reframe to hierarchical naming /
  point-of-presence, or cut. Confirm.
- **(W/F) "design the conditions, then get out of the way" → "the negative space is the design"** reword to
  "the negative space is an **intentional and critical part of** the design."
- **(F) Parenthetical formatting is wrong** for "no right to remove the rights of others" and the "design the
  conditions" line — they read as legitimate call-outs, not asides; lift them out of parens.
- **(D/C) The Hush-A-Phone / confirmed-legal-ancestor (Bazelon, §5).** The user questions whether it belongs
  in the *reasoning-foundations* doc at all — proposes it may be "a second pass / another document of
  historical alignment." Treat as a **defer-or-relocate** candidate (a possible new doc), not a silent
  delete; surface the option. (Note: this is the K-folded confirmed legal ancestor; moving it has ledger
  implications — record in BETA-ROLLUP if it moves.)
- **(S/C) "more than one principled boundary."** The review observes there are *several* boundaries/rights,
  that **the rights-floor defines a peer**, and ties this to "equal in rights, not capabilities" and the need
  to **define the rights** — **this is exactly K17 / 01 §5 / T21 / T22**. Reconcile with what's already
  there; likely a framing tweak in §5/§6, not new content.
- **(W) Likes the lift** of "equal in rights, not capabilities" → "equal possibility of all shapes" + variety
  (keep / strengthen).

There will be **more** in the body past line 60 — extract the whole file.

## Apply discipline (when I approve)

- **01 is settled beta.** These are the author's refinements, so they are authorized — but apply with the
  same care as the K-folds: **tier-cleanliness** (a beta doc carries **no** prior-tier references — no
  `../alpha/…`, no `COHESION §`, no `thinking/…` attributions), **preserve verbatim quotes whole** with their
  citation + verification flag, and keep the per-doc template shape (narrative → charter → body → "what this
  establishes / does not").
- **Verify after any edit:** `grep -rEn "\.\./alpha/|COHESION §|## Sources \(alpha\)|## Provenance trace" beta/0*.md`
  must return nothing.
- **Record substantive (S/C) changes** in `alpha/BETA-ROLLUP.md` (a short "01 review refinements 2026-06-26"
  note — what changed and why), and add a `COHESION.md` seam entry if a change opens/closes a loose end. If
  the Hush-A-Phone material relocates to a new historical-alignment doc, that doc lands in `alpha/` (it's new
  thinking), not beta, until matured.
- **Register the artifact's provenance:** `beta/thinking/01_beta_review.txt` is currently an unregistered raw
  transcript. Per `PLAYBOOK.md`, add a row to `alpha/seeds/transcripts/RAW-ARTIFACTS-MANIFEST.md`. **Filing
  decision to surface (don't auto-resolve):** whether its canonical raw home stays `beta/thinking/` (it
  reviews a beta doc) or a cleaned-paste copy also lands under `alpha/seeds/transcripts/raw/` per the usual
  convention. Recommend, then ask.
- **No external facts to fact-check** (the user's own design reasoning) — except if a refinement touches an
  atproto/iroh/iOS claim, cite the FACTCHECK SoT
  (`alpha/seeds/transcripts/raw/atproto-atmospheric-web-iroh-mobile-FACTCHECK.md`; iroh `1.0.0`), don't re-verify.
- **Commit/push only when I ask.** Co-author trailer: `Co-Authored-By: Claude Opus 4.8 (1M context)
  <noreply@anthropic.com>`.

## Definition of done

Every refinement note in the transcript is extracted, de-duped, classified (W/S/C/F/D), and tied to a 01
location; the plan was presented and approved; approved W/S/C/F changes applied to `beta/01` with
tier-cleanliness re-verified; D items recorded (and any new historical-alignment doc stubbed in `alpha/` if I
approve it); substantive changes noted in `BETA-ROLLUP`; the transcript registered in the manifest; the K17 /
T21 / T22 overlap reconciled (no duplication); no gate resolved; nothing committed unless I ask.

> **Scope:** this is the **01** theme only. If it goes well, the same shape repeats for a review of any other
> theme dropped into `beta/thinking/`.
