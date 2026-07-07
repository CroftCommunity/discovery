# discovery / beta / cairn: Layer 3 (the field of existing bolstering tech)

date: 2026-07-07

**What this layer is.** The survey and catalogue of the *existing* technology Drystone builds on: the
solutions, products, libraries, specs, and offerings in the broader space. It sits between `philosophy/`
(Layer 2) and `drystone-spec/` (Layer 4) because **the spec had to survey the field first**, to learn
whether the ecosystem already held the parts needed to assemble a safe amount of novelty practically.
Designing iroh, MLS, CBOR-DAG, or Willow from scratch would have been too heavy a blocker; the work was to
find building blocks that could fulfill the requirements, and to keep the material honoring and tracking
those parts together in one place.

**Cairn is the inverse of activism (Layer 9).** Both layers survey the field; they differ in valence.
Activism is the case *against* the incumbents, the extractive tech we refuse and indict. Cairn is the
catalogue of what we build *on*, the enabling tech we credit and reuse. Same activity, opposite sign.

**The name.** A cairn is dry-stacked waymarker stones, raised by many hands, no mortar and no keystone,
left by those who went before to mark a path for those who come after. That is exactly what an index of
this space is, and it shares the dry-stone construction family with `drystone` itself: **cairn catalogues
the stones; drystone builds the wall.** ("Cairn" was developed as the name for the index of the polycentric
space in the local-authority working notes; making it a layer lands that idea.)

## Scope

In scope: existing protocols, specs, libraries, and products that are candidate or actual building blocks,
or that occupy the same "credible-exit / privacy-forward / capture-resistant" space. Inclusive of
ActivityPub (AP), atproto / AT / the atmosphere, general p2p, CRDTs, QUIC, iroh, MLS, CBOR-DAG,
Willow/Meadowcap, and products such as Roomy, Blacksky, p2panda, SimpleX, and Matrix.

Boundary calls:
- **Some bubbles up into the spec; some does not.** `drystone-spec/` (Layer 4) *cites* the parts it uses
  (iroh 1.0, MLS RFC 9420/9750, Willow/Meadowcap). Cairn catalogues the *whole surveyed field*, including
  parts the spec passed over (Roomy, p2panda, SimpleX, ActivityPub) that still matter for tracking the space
  and its network effects. Something like Roomy has nowhere else to be represented; cairn is where it is
  tracked and linked.
- **vs `philosophy/prior-art/`.** Philosophy holds academic *frames* and principles (e.g. Modular Politics,
  Ostrom-rooted governance theory). Cairn holds shippable *tech*. Test: can you install it, call it, or
  build on it? → cairn. Is it an idea or frame you reason with? → philosophy.
- **vs `activism/`.** Activism holds the harm case against incumbent platforms. Cairn holds the enabling
  building blocks. The valence is opposite.

## Contents

| doc | what it is |
|---|---|
| `atproto-ecosystem.md` | The atproto/AT ecosystem survey: the repository model, lexicons, sync semantics, and the field of atproto-native projects (Frontpage, Roomy, the Arbiter, and others). |
| `social-lexicon-group-research-brief.md` | The research brief mapping social-application lexicons and group feature models (Bluesky lexicon catalog, cross-platform group features, membership/moderation lifecycle) onto a per-author, membership-scoped substrate. |

## Migration backlog (ecosystem material living elsewhere that belongs here)

Filed elsewhere in earlier batches, to migrate into cairn when the consistency sweeps run:
- The **MLS scaling survey** (guarantees vs demonstrated reality; commit-serialization and Delivery-Service
  bottlenecks; CoCoA/SAIK; OpenMLS and Soler et al. 2025 benchmarks; Wire/Webex/Cloudflare/Matrix
  deployments) — in a raw transcript.
- The **Willow / Meadowcap analysis** (the state-based-CRDT data model, the timestamp-tiebreak wrinkle,
  maturity status) — in a raw transcript and the local-authority notes.
- The **Blacksky research** (People's-Assembly/Polis governance, the thin-AppView-fork + Rust path, Community
  Posts) — in a raw transcript.
- The **ecosystem landscape** section of the local-authority collaboration notes (SimpleX, Briar, Cwtch,
  Matrix, Session, Nostr rated on the capture-resistance and privacy axes; MLS/MIMI as the seam).
- **Roomy / p2panda** tracking (the customer-changing-suppliers vs supplier-releasing-hard-parts contrast).

## Provenance & status

Seeded 2026-07-07 (batch ten). `atproto-ecosystem.md` and `social-lexicon-group-research-brief.md` filed
byte-verbatim from the p10 corpus (`ten-willlow.zip`); the full p10/p11 corpus is frozen at
`../../alpha/seeds/p10-p11-corpus/`. See `../../alpha/seeds/transcripts/RAW-ARTIFACTS-MANIFEST.md`.

## What this layer establishes (and does not)

Establishes a home for the field survey, so the ecosystem material stops being homeless (previously
"candidate for ECOSYSTEM.md" with nowhere to land) and the parts Drystone builds on are credited and
tracked in one place. Does **not** duplicate the spec's own citations of the parts it uses, and does **not**
hold academic frames (those are `philosophy/prior-art/`) or the harm case (that is `activism/`).
