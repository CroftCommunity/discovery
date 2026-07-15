# Proofs

Durable proofs for the Sovereign Commons / lineage-groups effort — equal and distinct from
the `experiments` repo. A **proof** verifies an invariant rigorously enough that a design
principle can be built on it; it is meant to graduate into
`discovery/crystallized/principles.md`. (Experiments, in the sibling `experiments` repo, are
code-forward exploration: "does this work / what is actually true?")

## Contents

```
Proofs/
├── lineage-groups/        (croftc PR #8) Rust validation of the lineage-groups thesis
│       against REAL openmls 0.8.1. Six-crate workspace: lineage-core (DAG, governance,
│       thresholds, survivor selection — pure logic), lineage-mls (openmls wrapper),
│       lineage-history (Automerge backfill), lineage-sim (partition simulator),
│       lineage-iroh (real-transport spike). Phase findings PHASE_0..3 + 2.5 + 2.6.
│       HEADLINE: the Phase 1 crypto-feasibility gate is GO — openmls can express
│       "pick a survivor epoch and re-key the other side, or mint a third" with PCS intact.
│
├── lineage-group-model/   (croftc PR #9) TypeScript simulation suite proving the model's
│       invariants (INV-ANCESTRY, INV-DETECT-CONTRADICTION, INV-NO-AUTO-RESOLVE,
│       INV-TRAPDOOR, INV-LINEAGE-NOT-LEAF, …) across groups A–H + V. Ancestry is REAL
│       (SHA-256 hash-DAG); MLS and transport are MODELED. Includes the visibility-regime
│       tests (V) and SOCIAL_LAYER_FINDINGS.md.
│
└── encrypted-local-first-atproto/  (croftc PR #3) The comprehensive validation of the
        central bet: a private MLS+CRDT group can interoperate with public atproto as a
        POLICY + TRANSPORT boundary, not a data-model fork. 14 sub-experiments (Phases 1–12);
        hypothesis-accounting H1–H9 in README.md. Key results: removal = forward secrecy
        only (redaction needs re-encryption); a membership SEQUENCER is load-bearing under
        concurrency; rkeys must be pinned; a public reference to a private record is itself
        a leak. The live-bsky-validate/ subdir is a PENDING live spike (RESUME.md handoff —
        needs egress allowlist + creds). Overlaps the public-path spikes in `experiments/`.
```

Each proof carries `PR-CONVERSATION.md` (the PR discussion + reviews) and
`CODING-TRANSCRIPT.md` (the build narrative) for provenance.

## The modeled-vs-real boundary (why two lineage proofs, not one)

`lineage-group-model` proves the *logic* is internally cohesive — but models MLS and
transport. `lineage-groups` proves the *crypto* is real on openmls. They are the two halves
of one validation: the TS suite establishes the design holds together; the Rust workspace
establishes the make-or-break primitive (survivor-epoch re-key with post-compromise
security) actually exists in the library. The TS suite's "Deferred to real-stack validation"
list is largely discharged by the Rust workspace. See `discovery/COHESION.md`.

## Provenance / exclusions

Both proofs were authored in `croftc/SecurityPolicy` (PRs #8, #9) and imported here.
SecurityPolicy plumbing (`.github/workflows/confluence-sync.yml`, `renovate.json`) was not
imported. One open compliance item travels with `lineage-groups`: the MLS dependency tree
pulls MPL-2.0 (`hpke-rs`, mandatory for RFC 9420 HPKE) — flagged, not resolved.

## Convention

Git identity: chasemp (`chase@owasp.org`, `github-personal` SSH host). Pin deps; commit
lockfiles; never commit credentials.
