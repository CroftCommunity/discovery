# Proofs — folded into discovery (frozen, canonical)

`Folded in 2026-07-15 (RUN-08). Source: the CroftCommunity/Proofs repo (main export). This is the
same move already applied to the standalone experiments/ repo (see
alpha/experiments/FROZEN-NOTICE-for-standalone-repo.md): the sibling repo's alpha-tier corpus now
lives inside discovery so discovery is the single authoritative home.`

## What this is

The **durable-proofs** corpus — the crates that verify an invariant which becomes a design
principle. Its alpha-tier content is placed here at **`discovery/alpha/Proofs/`**, which is exactly
where the already-folded experiment spikes' Cargo path-dependencies point
(`alpha/experiments/iroh/crates/*/Cargo.toml` → `../../../../Proofs/lineage-groups/crates/…` resolves
to `alpha/Proofs/lineage-groups/crates/…`). So the fold-in resolves those path-deps with **zero
Cargo edits** — `mls-welcome-over-iroh`, `media-sframe-spike`, and `altdrive-spike-faithful-sync`
build against the real crates again.

Contents (Proofs alpha stage):

- **`lineage-groups/`** — the real-openmls validation workspace: `lineage-core`, `lineage-mls` (the
  thin auditable openmls wrapper), `lineage-history`, `lineage-iroh`, `lineage-sim`,
  `reconcile-harness`, `history-harness`, and **`conformance`** (the `emit-vectors` / `run-vectors`
  conformance-core that emits categories 1–6). Own Cargo workspace + committed `Cargo.lock`.
- **`lineage-group-model/`** — the TypeScript reference model (the authoritative runner for the
  green-model conformance categories).
- **`encrypted-local-first-atproto/`** — the encrypted-local-first ATProto proof line.

## Status

- **Frozen / read-only.** New proof work is not expected to land here as a matter of course; this is
  a canonical snapshot folded in to make the experiment spikes buildable in-repo and to give the
  conformance-core a home discovery can build against. If Proofs resumes active development, the
  standalone repo's history is the origin of record.
- **Tier note.** Per the maturity conventions this is alpha-tier content (matching the experiments
  fold-in); it carries its own inner staging only as the imported crates had it.
- **Firewall (RUN-08).** `lineage-groups` + `conformance` are MLS/transport/proof code; they do not
  touch the I9 identity/key-recovery trust tier. Folding them in is clean of the run's firewall.
