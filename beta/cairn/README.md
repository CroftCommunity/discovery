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
- **vs `fenced/`.** Cairn is the *open* field, the composable/decentralized tech Drystone builds among.
  `fenced/` is the *fenced* field, the centered commercial platforms Drystone is an alternative to (their
  roster/call/broadcast ceilings, E2EE stance, per-group rates, economics). Test: can you build Drystone
  out of it? → cairn. Is it a centered provider whose limits and posture you measure against? → fenced.
  atproto appears in both registers: the *protocol* is a cairn building block; the hosted *Bluesky
  platform's* posture is a fenced data point.

- **vs `activism/`.** Activism holds the harm case against incumbent platforms and the response to it.
  Cairn holds the enabling building blocks. The valence is opposite. (Activism reads its harm case off the
  `fenced/` map; cairn is the open-field twin of that map. See the field-and-response triad in
  `../LAYERS.md`.)

## Contents

**Orientation (added 2026-07).** Within cairn, stones carry one of three orientations:
**Drystone-oriented** (protocol building blocks the spec uses), **Croft-oriented** (product/app
building blocks), **related-ecosystem** (peer institutions/projects tracked as near neighbors but
neither built-on as a dependency nor shipped as product). Only the newest stone is tagged so far;
tagging the existing stones is a deferred review pass (ROADMAP_TODO C-item).

| doc | what it is | orientation |
|---|---|---|
| `mls-and-mimi.md` | MLS (RFC 9420) and MIMI as building blocks: the group-E2EE core, the TreeSync/TreeKEM/TreeDEM decomposition and its proofs (plus the external-ops weakening), MIMI's per-room hub (the seam Drystone occupies), the scaling reality (commit-serialization + Delivery-Service, not crypto; Soler 2025; CoCoA; the designated-committer fix), and how Drystone uses MLS as a subordinate key-distribution backplane. | — |
| `willow-meadowcap.md` | Willow and Meadowcap, mental model corrected: Willow is a state-based CRDT (join-semilattice) held locally and reconciled by range, not an object shipped whole; the Entry/subspace/merge model, the writer-claimed-timestamp wrinkle (no causality, silent loss), what that means for a governance fold, and Meadowcap's unforgeable attenuable capabilities. Drystone is built Willow-shaped. | — |
| `blacksky-and-atproto-community.md` | Blacksky (Rudy Fraser): atproto community infrastructure. People's-Assembly/Polis governance, paid community moderators, subscription funding; the thin-AppView-fork + Rust rsky-wintermute path; Community Posts inverting PDS-as-source-of-truth; the transferable ideas (traffic-class queue separation, invariant-inversion honesty) and the governance-vs-corporate-form tension. | — |
| `adjacent-systems.md` | The privacy-forward / capture-resistant landscape rated on two axes: Roomy and p2panda (and their opposite postures), the MLS/MIMI standards seam, and SimpleX / Briar / Cwtch / Matrix / Session / Nostr. Conclusion: the "both" corner is niche-and-young or mature-and-metadata-leaky, the structural reason it is empty and the space Drystone occupies. | — |
| `atproto-ecosystem.md` | The atproto/AT ecosystem survey: the repository model, lexicons, sync semantics, and the field of atproto-native projects (Frontpage, Roomy, the Arbiter, and others). | — |
| `social-lexicon-group-research-brief.md` | The research brief mapping social-application lexicons and group feature models (Bluesky lexicon catalog, cross-platform group features, membership/moderation lifecycle) onto a per-author, membership-scoped substrate. | — |
| `atproto-nsid-and-lexicon-mechanics.md` | NSID (Namespaced Identifier) mechanics: the reverse-DNS authority binding, the naming-rule tension (h3 vs hthree), and the fetchability gap; Smoke Signal as the worked example (founding motivation, PDS-plus-AppView architecture, the events.smokesignal.* to community.lexicon.* migration, and its Rust/Postgres/Redis/AIP stack). Answers research item A4 in `social-lexicon-group-research-brief.md`. | — |
| `atproto-content-portability-and-backdating.md` | The atproto write path for content backfill (uploadBlob, embed.images, backdated createdAt, listMissingBlobs); the Pixelfed export gotcha as a migration case study; the tooling-gap survey (mastodon-to-bluesky, Bounce, Bridgy Fed); a self-correction discipline example (retracting an unsourced motive claim); and the backdated-post labeler as a "detectable, not blocked" moderation primitive. | — |
| `substrate-prior-art.md` | What Croft's transport+data substrate builds among: **Peat** (the strongest existing prior art for the exact Rust+iroh+CRDT+MLS bet, proven in denied/degraded); the recursive/locality routing lineage (RINA, NDN, Yggdrasil, cjdns); the substrates set aside with their reasons (libp2p, Veilid, Holochain); and the neighbors to read before the capability layer hardens (p2panda, iroh-rings). *(Phase-1 recovery from ECOSYSTEM.md §1/§6.)* | — |
| `identity-and-data-ownership-poles.md` | The two identity/data-ownership poles Croft sits between: **Solid** (private-by-default direct-access Pods; WebID/Solid-OIDC/DPoP) and **DSNP + Frequency** (social-graph-as-public-utility, on-chain, no core token, delegation without master keys). Croft as a third E2EE location sharing DSNP's unbundle+delegate goals while rejecting the chain. *(Recovery from ECOSYSTEM.md §5.)* | — |
| `cross-platform-identity-provenance.md` | The hub-and-spoke provenance thesis: OOB attestation as the only real cross-platform linkage; did:webvh SCID root + did:plc spoke; the evidentiary alsoKnownAs ladder; the load-bearing negative result (atproto resolves only did:plc/did:web → the bidirectional-alsoKnownAs workaround); plc.directory as transparency-log-not-CA; DIDComm delegate + CT/CONIKS equivocation-detection. Surfaces decisions A9/A10; couples T7. *(Recovery from ECOSYSTEM.md §4.)* | — |
| `atproto-selfhosting-appviews-and-bridges.md` | The atproto self-hosting / AppView / bridge landscape: AppView-as-private-gatekeeper (inverts public-by-default); **Groundmist** (closest local-first-private relative); PDS impls (Cocoon, rsky-pds) + managed hosts + blob backends; bridges + credible-exit (Bridgy Fed, Bounce); infra movements (Free Our Feeds). *(Recovery from ECOSYSTEM.md §5e/§5f; complements `blacksky-and-atproto-community.md`.)* | — |
| `atmospheric-web-and-aggregators.md` | The "atmospheric web" demand-side argument (every serious version hits the public-by-default wall = Croft's gap), the aggregator/fork license-map + the no-per-activity-fee finding (feeds the garden-of-ponds aggregator strategy), and the fused-timeline anti-pattern (Openvibe) that confirms honest-seams. §5c client tooling is dialogue-sourced [UNVERIFIED]. *(Recovery from ECOSYSTEM.md §5b/§5c.)* | — |
| `iroh-app-pond-building-blocks.md` | The games / file-drop / realtime-media / on-device-AI building blocks for the pads: libmarathon, ascii-royale, iroh-lan, godot-iroh, GGRS+matchbox, the webxdc catalog (license traps), the Cure53 audit (disable webview WebRTC), on-device AI (Foundation Models / Gemini Nano), and the iroh media floor (callme/iroh-roq; media ceiling rows Gemini-sourced). Consumer is `../croft/product-the-garden-of-ponds.md`. *(Recovery from ECOSYSTEM.md §5d.)* | — |
| `object-capability-and-decentralized-mls-prior-art.md` | Two prior-art strands: object-capability (Spritely Goblins/OCapN/CapTP — "designation is authorization", POLA, petnames, and the no-VC governance model) and decentralized-MLS siblings (DMLS/FREEK, draft-xue "TwoMLS" — the fork→FS cost and the "no production MLS is serverless-ordered" anchor). Cross-refs the A11 Track-A/B capability-mechanism decision, which is recorded in `../drystone-spec/part-2-certifiable-design.md` (Appendix A + §5.5) and gated in `../DECISIONS.md` — `willow-meadowcap.md` carries the Willow/Meadowcap prior art, not the A11 choice. *(Recovery from ECOSYSTEM.md §2/§4.)* | — |
| `lexicon-community-governance.md` | Lexicon Community as a schema-steward governance *body*: the volunteer 7-member TSC, GOVERNANCE/CONTRIBUTING/COVENANT, Discourse-first interoperability-first working-group chartering (Polite Goshawk → Lexicon Lenses), the calendar/location/bookmarks namespaces, lexicon.garden, the public Feb-2025 incident report, Gerakines' credible-exit schema transfer, and the door/seam/resonance mapping to Croft. Governance-body depth; NSID mechanics stay in `atproto-nsid-and-lexicon-mechanics.md`. | related-ecosystem |
| `activitypub-atproto-and-the-defacto-standard.md` | ActivityPub vs ATProto (product/protocol/protocol; email-model vs web-model; the identity/exit, Lexicon-vs-JSON-LD, and global-vs-local consequences); Mastodon as the **de-facto AP standard** (WebFinger / HTTP-sigs / REST-not-C2S / FEP); the **HTML living-standard cautionary lesson** for Drystone (what befalls an underspecified protocol when one implementation reaches critical mass first); and Berjon's ap-at thesis (AP atop an ATProto PDS). Complements `atmospheric-web-and-aggregators.md` (fee/aggregator angle). | related-ecosystem |

## Provenance & status

Seeded 2026-07-07 (batch ten); the migration backlog was distilled the same day. `atproto-ecosystem.md` and
`social-lexicon-group-research-brief.md` filed byte-verbatim from the p10 corpus (`ten-willlow.zip`). The
four survey docs (`mls-and-mimi`, `willow-meadowcap`, `blacksky-and-atproto-community`, `adjacent-systems`)
were distilled from the raw transcripts (`../../alpha/seeds/transcripts/raw/mls-scaling-willow-ecosystem-and-cairn-2026-07-07.md`
and `../../alpha/seeds/transcripts/raw/mls-blacksky-modular-prior-art-2026-07-06.md`), which remain the
provenance. The full p10/p11 corpus is frozen at `../../alpha/seeds/p10-p11-corpus/`. The former migration
backlog (MLS scaling, Willow/Meadowcap, Blacksky, the ecosystem landscape, Roomy/p2panda) is now filed as
those four docs. See `../../alpha/seeds/transcripts/RAW-ARTIFACTS-MANIFEST.md`.

## What this layer establishes (and does not)

Establishes a home for the field survey, so the ecosystem material stops being homeless (previously
"candidate for ECOSYSTEM.md" with nowhere to land) and the parts Drystone builds on are credited and
tracked in one place. Does **not** duplicate the spec's own citations of the parts it uses, and does **not**
hold academic frames (those are `philosophy/prior-art/`) or the harm case (that is `activism/`).
