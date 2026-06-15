# discovery

Consolidation and refinement home for the thinking journey behind an open, sovereign,
peer-to-peer social/data platform run as a cooperative — and the deep technical design of
its encrypted group-messaging subsystem.

This repo takes mixed "seed" material (transcripts, dossiers, research) and pulls it apart
into clean, separable document sets so the logical flow can be refined toward a build.

## Three sibling repos

```
discovery     ← you are here. Thinking/synthesis: thesis, research, principles,
                roadmap, narrative, the cohesion map.
Proofs        Durable proofs that verify invariants → become design principles.
                (lineage-groups [real openmls], lineage-group-model [TS model])
experiments   Code-forward exploration: "does this work / what's actually true?"
                (appview-validation [live atproto/Jetstream])
```

`discovery/crystallized/proof-ledger.md` tracks every invariant/experiment and links to its
proof in the `Proofs`/`experiments` repos. `discovery/COHESION.md` tracks where one
document's "loose end" is walked out by another's proof.

## The two bodies of thinking

```
UMBRELLA VISION  ──────────────────────────────────────────────────────────
  SOVEREIGN-COMMONS-DOSSIER.md
    the why (commons/civics/philosophy) · the what (goals) ·
    the how (vault substrate, iroh stack, identity, economics, cooperative)
    — naming/branding in flux (Sovereign Commons / Alt.Drive / Loci / …)

        contains, as one subsystem ▼

DEEP-DIVE: GROUP MESSAGING  ───────────────────────────────────────────────
  the lineage-groups protocol (encrypted local-first group chat + social graph)
    research/  messaging-solutions-landscape.md   — the competitive field
    thinking/  thesis-lineage-groups.md           — the protocol thesis + invariants
               multi-device.md                    — per-device-key model
               social-layer.md                    — the graph-you-hold layer
```

The dossier's "content-blind HA / anchor peer" is the lineage-groups "blind superpeer
broker." The dossier's DID/SSI identity is the lineage-groups DID lineage. The dossier's
"messaging = vault artifacts, Automerge when interactive" is the substrate the lineage-groups
protocol rides on. The group-messaging work is the most-developed slice of the larger vision.

## Layout

```
discovery/
├── SOVEREIGN-COMMONS-DOSSIER.md   umbrella vision (consolidated thinking journey)
├── ANALYSIS.md                    corpus map of the GroupDynamics seeds
├── ROADMAP.md                     rough milestones + features (organize-now, refine-later)
├── ECOSYSTEM.md                   relational register of related open work (homage /
│                                    integrate / partner / rebroadcast / learn↔)
│
├── seeds/                         raw, immutable source material
│   ├── groupdynamics-unpacked/    the 4 produced group-messaging docs as delivered
│   └── generated-prompts/         2 prompts spawned but not yet run/filed
│
├── research/                      industry research & comparison (analytical lens)
│   ├── README.md                  how research/ relates to ECOSYSTEM.md (same projects,
│   │                                different purpose/audience — keep both, cross-ref)
│   └── messaging-solutions-landscape.md
│
├── thinking/                      our design (ours), evolving
│   ├── thesis-lineage-groups.md
│   ├── multi-device.md
│   └── social-layer.md
│
├── crystallized/                  the distilled spine
│   ├── principles.md              design + civic principles, extracted
│   ├── proof-ledger.md            every I/E/V/S assertion + status + link to its proof
│   └── conclusions.md             what's settled vs. the open risks (drafting)
│
├── COHESION.md                    loose-ends ↔ the proof/experiment that addresses them
│
└── narrative/                     the story, for humans (drafting)
    ├── verticals/                 standalone topic narratives
    ├── long-form.md               full why→what→how narrative
    └── short.md                   max-3-page version
```

Proof/experiment CODE lives in the sibling `Proofs` and `experiments` repos, not here.

## Conventions

Blank line between bullet points. Raw `seeds/` stay frozen; `thinking/` evolves;
`crystallized/` and `narrative/` are the distilled outputs we refine.
Naming/branding is unsettled — do not propagate product names into structure until pinned.
