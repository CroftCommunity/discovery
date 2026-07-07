# Handoff prompt: continue filing the Drystone/Croft web-session batches

`Copy this into a fresh Claude Code session started in /Users/cpettet/git/chasemp/CroftC/discovery. It is a
thin pointer: the canonical process is in the repo (AGENTS.md, PLAYBOOK.md, beta/LAYERS.md, and the
manifest). This file records where we are and what is left, as of 2026-07-07, after batch 10 of 11.`

---

## What this work is

I am filing a series of web-session deliverables (the "batches") into the CroftC `discovery` repo. Each
batch arrives as one or more zips dropped in the `discovery/` root, named `<n>-<label>.zip` (e.g.
`eleven-*.zip`), usually with the originating claude.ai conversation pasted into the chat. Your job per
batch: unpack, audit against the tree, route the deliverables into the right beta layer, preserve raw
provenance, keep the corpus coherent, and commit + push.

Read first, in this order: `discovery/AGENTS.md`, `discovery/PLAYBOOK.md`, `discovery/beta/LAYERS.md`, and
the tail of `discovery/alpha/seeds/transcripts/RAW-ARTIFACTS-MANIFEST.md` (the running intake log; every
batch has an entry, newest at the bottom). Those are the source of truth. This prompt does not repeat them.

## The per-batch workflow (what has worked for 10 batches)

1. **Inspect** the zip(s): `unzip -l`. A zip may be a clean increment or a full-corpus snapshot spanning
   several maturity iterations (p9/p10/p11); do not assume.

2. **Audit before trusting.** The web agent has a known habit of losing/dropping content across iterations.
   Before overwriting any canonical doc (especially the spec), diff the candidate against the committed
   version: heading-level (`comm -23` of sorted `grep '^#'`) plus content-presence greps for load-bearing
   terms. If a candidate is a parallel-lineage rebuild, it may lack tree-side additions (this bit us in
   batch 10: p11 did not carry the tree-side §7.6.4 re-plant fold; it was extracted and re-folded as
   §7.6.11). Preserve, do not drop.

3. **Route into layers** (see `beta/LAYERS.md` for the model). Byte-verify every filed copy with `diff -q`.

4. **Preserve raw.** Freeze full-corpus snapshots and superseded/duplicate versions under
   `alpha/seeds/<name>/` (never delete; provenance is non-negotiable, PLAYBOOK §4). Preserve the pasted
   conversation as a cleaned-paste raw transcript in `alpha/seeds/transcripts/raw/` with the §4 caveat
   ("content-faithful, not a byte-pristine export").

5. **Keep coherent.** Update the relevant layer README(s), `beta/LAYERS.md`, `beta/README.md`,
   `beta/OPEN-THREADS.md` (the beta ledger, threads T1..T36 so far), the spec `CHANGELOG.md`
   (document-pass-N, newest at top), and add a manifest entry.

6. **Commit + push.** The user wants a commit per batch and asks to push (recent batches all pushed). Use
   the chasemp identity and the CroftC co-author trailer (see below).

## Hard conventions (do not drift)

- **Em-dash discipline.** No em-dashes (U+2014) in the spec, the beta companion docs, or anything you
  author; use commas, colons, semicolons, or parentheses. Blank line between markdown bullets. The
  pre-existing docs that predate the discipline (spec `README`, `CHANGELOG`, `beta/README`,
  `feasibility-review-v2`) are a **deferred** tidy, do not sweep them yet (see pending item 3).

- **Drift grep** (PLAYBOOK §4) on new spec/companion content: `grep -riE
  'connect_to_peer|merkle search tree|AT Messaging working group|ger\.mx'` should be clean. For
  atproto/iroh/iOS facts cite the FACTCHECK SoT (iroh is `1.0.0`), do not re-verify.

- **Subagents match the parent model.** Spawn distillation/audit subagents with `model: opus` (per the
  user's global rule); one file each, they read on-disk sources, report em-dash 0.

- **Git identity (path is `/Users/cpettet/git/chasemp/*`):** committer `Chase Pettet <chase@owasp.org>`,
  remote `origin = git@github-personal:CroftCommunity/discovery.git`, branch `main`. Commit trailer:
  `Co-Authored-By: Claude Opus 4.8 (1M context) <noreply@anthropic.com>`. (gh, if needed, `gh auth switch
  --user chasemp` first, but push uses the SSH key, not gh.)

## Current state (after batch 10, commit e27632e, pushed)

The beta layer-cake (canonical model in `beta/LAYERS.md`), why-first ordering:

```
1 history (not yet created)   2 philosophy (+prior-art)   3 cairn (the field of existing tech)
4 drystone-spec   5 impl   6 croft (not yet created)   7 governance (reserved)
8 socialization   9 activism
```

- **drystone-spec (Layer 4):** the current spec is the p10/p11 lineage, `part-1-reasoning-underpinnings.md`
  (p10) + `part-2-certifiable-design.md` (p11 rebuild), at **document-pass-8**. Companions:
  `conventions-and-decisions.md`, `open-threads.md`, `feasibility-review-v2.md`, the `CHANGELOG.md`, and the
  per-part changelogs. `superseded/` holds provenance (persona-definition, open-items, bounded-contexts,
  review-handoff, the two SVGs), marked superseded, treated as raw transcripts.
- **impl (Layer 5):** `delivery-layer/` (00-12 corpus), `mls/` (substrate bundle), `drystone-design/` (ten
  p10 design companions), `experiments/`, and `doc-writing-method.md` (the single shared canonical method).
- **cairn (Layer 3, seeded batch 10):** `mls-and-mimi`, `willow-meadowcap`, `blacksky-and-atproto-community`,
  `adjacent-systems`, `atproto-ecosystem`, `social-lexicon-group-research-brief`. Migration backlog cleared.
- **philosophy (2):** the peer-standing set + `prior-art/` (Modular Politics). **socialization (8):**
  essay, pitch, coffee-shop + elevator-pitch tellings. **activism (9):** the "platforms author the
  relational field" research set. **governance (7):** reserved (README only). **history (1), croft (6):**
  not yet created (theme 02 seeds history; theme 08 seeds croft).
- The full p9/p10/p11 corpus is frozen at `alpha/seeds/p10-p11-corpus/`.

## Pending work

- **Batch 11 of 11** is expected. Process it per the workflow above. Watch for whether it supersedes the
  spec again (audit before swap).
- **Deferred sweeps, end-of-run, current-versions-only** (the user parked these):
  1. **peer -> persona reconciliation** across the *current* companion bodies that still use "peer" as the
     entity noun (the spec itself is migrated; the philosophy/impl companions may lag). Not the superseded
     docs.
  2. **em-dash tidy** of the pre-existing docs never in the normalization scope (spec `README`, `CHANGELOG`,
     `beta/README`, `feasibility-review-v2`).
  3. Optionally retire, or keep-as-provenance, whatever the batch-11 work supersedes.
- **Open threads** (`beta/OPEN-THREADS.md`): T33 edge-preserving capital formation; T34 Project Mercury
  PACER docket (time-sensitive, pre-publication); T35 uncompensated-community-labor activism thread; T36
  verify the §7.6.11 re-plant mechanism (run the E12 experiment set on mls-rs 0.55.2).
- **Spec open items** (Part 2 Appendix B / `open-threads.md`): the priority capped-root soundness question;
  Track A (Meadowcap) vs Track B (Keyhive) capability choice; the `ENABLING` wire encodings that gate a
  publication-final release + DOI; the BLAKE3-based §4 end-to-end re-proof (BLAKE3 is pinned, the proof-out
  is open); the completeness-beam / tail-gap checkpoint (fail-closed on finality).

## Start here

Say what batch/zip is present (or that none is yet), then follow the workflow. If nothing is dropped yet,
confirm orientation from AGENTS.md + LAYERS.md + the manifest tail and wait for the batch.
