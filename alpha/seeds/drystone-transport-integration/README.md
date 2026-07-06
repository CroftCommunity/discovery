# Drystone transport/identity integration — process artifacts (frozen)

date: 2026-07-06

**What this is.** Process artifacts from the transport/identity/encryption integration (Drystone spec
document-pass-5), frozen here rather than placed in the clean spec tree. The durable output is in the
spec: Part 2 §6 (expanded to §6.1–6.8) plus the two figures (`drystone-exposure.svg`,
`drystone-catchup-flow.svg`) now in `beta/drystone-spec/`.

- `drystone-integration.diff` — unified diff of the integrated docs against the pre-integration uploads,
  with a scope header. Net: Part 2 +569/−53, exposure SVG +12/−12, Part 1 and catch-up SVG unchanged.
  **Point-in-time:** matches the batch-six upload, not necessarily any later in-tree edit; the CHANGELOG
  (document-pass-5) is the durable change narrative.
- `drystone-integration-summary.md` — the work summary: inputs/outputs, the three passes (integration,
  iroh 1.0 differentiation, consistency lint), the verification ledger (what was pulled from primaries vs
  what stays `[confirm]`), and the recommended pre-publication steps.

**Not kept:** `drystone-transport-section.md` (the standalone transport-section draft, from
`six-transport.zip`) is superseded by the merged Part 2 §6 and was not filed.

**Related handoff:** the messaging-layer research prompt this session produced (MLS-over-iroh delivery
models, the DS/meer/push-host question) is filed as a forward-looking prompt at
`../generated-prompts/drystone-messaging-layer-research-prompt.md`.

**Preservation status: preserved-verbatim** (byte-identical to the `six-integrating.zip` payload). Process
artifacts, not spec: the spec bodies govern. See `../transcripts/RAW-ARTIFACTS-MANIFEST.md` for the intake.
