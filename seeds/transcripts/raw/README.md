# Raw transcripts archive (reference & provenance)

date: 2026-06-15

These are **verbatim** raw transcripts as provided, kept for reference and provenance — the
unedited source behind the condensed `CODING-TRANSCRIPT.md` summaries that live next to each
proof/experiment. Where a transcript embedded a brief that is already saved verbatim
elsewhere in the repo (the lineage thesis, the experiment-suite spec), this archive points to
that canonical copy instead of triplicating it, and preserves the session log verbatim.

## Provenance status

- **Code:** verbatim. Each proof/experiment tree was `git clone`d from its croftc
  SecurityPolicy PR branch and copied unchanged; `diff -rq` against the branches is empty
  (only the added PR-CONVERSATION/CODING-TRANSCRIPT files and excluded SecurityPolicy
  plumbing differ).

- **PR conversations:** verbatim, pulled from `gh` into each `PR-CONVERSATION.md`.

- **Coding transcripts:** verbatim here in `raw/`; condensed/readable renderings in each
  artifact's `CODING-TRANSCRIPT.md`.

## Files

| File | PR | Artifact | Embedded brief (saved elsewhere) |
|---|---|---|---|
| `pr6-appview-validation.md` | #6 | experiments/appview-validation | — |
| `pr9-lineage-group-model.md` | #9 | Proofs/lineage-group-model | experiment-suite spec → in lineage-group-model |
| `pr8-lineage-groups.md` | #8 | Proofs/lineage-groups | lineage thesis → thinking/thesis-lineage-groups.md |
| `pr4-public-roundtrip.md` | #4 | experiments/public-roundtrip | — |
| `pr3-encrypted-local-first.md` | #3 | Proofs/encrypted-local-first-atproto | — |

## Still outstanding (see ../RAW-ARTIFACTS-MANIFEST.md)

The original **design-dialogue transcript** (the first large paste — the messaging research
and the multi-device/social-layer/lineage-fork conversation) is still only distilled into
ANALYSIS.md, not preserved verbatim. Re-drop it to save as
`../design-dialogue-2026-06-13-to-14.md`.
