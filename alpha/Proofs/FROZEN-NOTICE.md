# Proofs — folded into discovery (frozen, canonical)

> **Correction banner (RUN-11, 2026-07-16; body below is the frozen original, untouched).** The
> "conformance-core that emits categories **1–6**" line in *Contents* below **understates the folded
> core**: the emitter's `categories_present` lists cats 1, 2, 3, 4, 5, 5b, 6, 7, 8, 9, and the suite
> **emits and re-proves categories 1–9 (66/0, RUN-08)** — confirmed by Part 2 §10.5 (FND-R10-4). This
> is a documentation lag inside the frozen record; the record itself is not edited (only this banner is
> added), and the body may be corrected at the next deliberate freeze-break (e.g. the `[gates-release]`
> pass). Everything beneath this banner is the frozen original.

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
