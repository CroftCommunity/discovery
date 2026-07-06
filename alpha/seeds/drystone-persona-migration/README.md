# Drystone persona/peer vocabulary migration — process artifacts (frozen)

date: 2026-07-06

**What this is.** The process/tracking artifacts from the persona/peer vocabulary migration (Drystone spec
document-pass-4), frozen here rather than placed in the clean spec tree. The migration sharpened *peer* to
name only the relation and introduced **persona** (plural **personae**) as the entity a human is manifested
as. The durable outputs are in the spec: `beta/drystone-spec/part-1…`, `part-2…` (Appendix D + identity
model), and the companion `beta/drystone-spec/persona-definition.md`. These files are the working-out
behind that.

- `drystone-persona-migration.md` — the migration plan (reconciled).
- `drystone-persona-delta.md` — the change record + validation result + fold-in note.
- `drystone-persona-session-summary.md` — the authoritative model + conclusions as of the session.
- `peer-inventory-worksheet.txt` — the line-by-line inventory of every "peer" occurrence and its
  disposition (kept as relation / reassigned to persona / other).
- `drystone-part1.diff`, `drystone-part2.diff` — the per-part diffs against the pre-migration specs, as
  regenerated at end of session. **Point-in-time:** they match the batch-five upload, not necessarily any
  later in-tree edit; the CHANGELOG (document-pass-4) is the durable change narrative.

**Preservation status: preserved-verbatim** (byte-identical to the `five-persona.zip` payload). Process
artifacts, not spec: do not treat as the source of truth for any claim, `persona-definition.md` and Part 2
§5 govern. See `../transcripts/RAW-ARTIFACTS-MANIFEST.md` for the batch-five intake entry.
