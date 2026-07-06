# discovery / beta / philosophy / prior-art

date: 2026-07-06

Source-grounded analyses of the intellectual and academic prior art Drystone builds on and beyond: the
nearest-neighbor frames, what each established, and precisely where it stopped (the seam Drystone works in).
These sit in `philosophy/` (Layer 2) because they are the intellectual-lineage register: the thinkers and
frameworks the principles descend from or diverge from, not the empirical harm case (that is `activism/`,
Layer 8) or the manifestation (that is `governance/`, Layer 6).

## Contents

| doc | what it is |
|---|---|
| `modular-politics-analysis.md` | Analysis of *Modular Politics* (Schneider, De Filippi, Frey, Tan, Zhang; CSCW 2021), the nearest-neighbor academic frame: governance as an open, portable, composable protocol standard, rooted in Ostrom's IAD. Grounded against the arXiv v3 full text. The key finding for Drystone: it is asymmetric by construction (all permissions derive from the platform operator at the Instance level) and substrate-agnostic by choice (security/crypto explicitly deferred, wire encodings never reached). "It drew the map and left the territory": the cryptographic-resolution and wire-encoding layers Drystone works in are exactly what it left as future work. |
| `modular-politics-session-summary.md` | The session record for that analysis. |

## Provenance & status

Assembled from conversation, delivered 2026-07-06 via `eight-modular.zip`. Filed byte-verbatim (the
session-summary had two em-dashes normalized to match the layer convention). The analysis is grounded in the
primary source (arXiv:2005.13701 v3, matching the published CSCW version by DOI); the one un-checked item
noted in it is whether the authors' later work (Schneider's 2024 *Governable Spaces*) revises the
operator-rooted-permissions or deferred-crypto stance. Raw transcript at
`../../../alpha/seeds/transcripts/raw/mls-blacksky-modular-prior-art-2026-07-06.md`.

## Related prior-art in the transcript (not yet distilled into docs)

The same session's raw transcript also holds substantial research on **Blacksky** (an atproto community-infra
project: People's-Assembly / Polis governance, the thin AppView fork + Rust rsky performance path, Community
Posts as an AppView-resident private-data lexicon, and the participatory-governance-vs-corporate-form gap)
and the **MLS journey/ecosystem** (MIMI as the interop half, the formal-proof record and its found limits,
adoption). These are candidates for an `ECOSYSTEM.md` register update and/or their own prior-art notes if you
want them distilled; for now they live in the raw transcript.
