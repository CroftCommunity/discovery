# Industry research & comparison

This directory holds our **industry research and comparison** of the field — analytical work
that positions our design against what exists, surfaces lessons, and informs decisions.

## Relationship to the ecosystem register (they overlap on purpose)

`research/` and `../ECOSYSTEM.md` cover **much of the same set of projects**, but they are
written for different purposes, audiences, and needs — keep both, and let them overlap:

```
ECOSYSTEM.md (relational)              research/ (analytical / comparative)
  purpose: homage, integration,          purpose: position our design, extract
    partnership, rebroadcast, learn↔       lessons, inform build decisions
  audience: us + future collaborators;   audience: us (design), reviewers, and
    a "movement we're part of" record      re-cuttable for funders / public
  shape: register (org/project/state/    shape: deep comparative analysis along
    relationship tags)                     axes (usability/security/capability, …)
```

Same projects, different lens. A fact about, say, Delta Chat's iroh usage may appear in both:
in ECOSYSTEM.md as "closest Rust+iroh cousin, partner/learn↔"; here as a detailed
usability/security/capability comparison with our stack. That duplication is intended — do
not try to merge them into one document; cross-reference instead.

## Contents

- `messaging-solutions-landscape.md` — three-axis competitive analysis (usability, security,
  capability) of Signal, Delta Chat, SSB, Matrix, Briar, Session, WhatsApp/Telegram, mapped
  to our planned stack. The design-lens comparison.

- `social-platform-cycle.md` — commissioned research narrative on the cycle of VC-funded social
  platforms since the 1990s and the recurring community "rug-pull." Thesis: the cycle is a
  capital-structure phenomenon (VC exit → extraction imperative) leveraged by lock-in; the
  antidote couples unsellable governance + portable/user-owned data with the UX the community
  side historically skipped. Statements labeled Documented history / Analysis / Commissioner's
  thesis, with `[UNVERIFIED]` inline. Imported from a web-research session; `<cite>` tags
  stripped on import. Status: draft for review.

- `discord-dominance.md` — commissioned analysis of how Discord came to dominate community chat,
  including FLOSS communities (e.g. iroh) whose values it doesn't reflect. Finds zero-friction
  joining the largest single driver (above brand/stability), separates Discord's genuinely-good
  live-presence core from its genuinely-bad knowledge/search/ownership, and frames the
  "cohesive product on open foundations" middle path as a real under-occupied position.
  Documented fact / `[inference]` / `[UNVERIFIED]` flagged throughout. Status: draft for review.

- `public-social-protocols.md` — comparative analysis of public social products and the protocols
  beneath them (Twitter/X, Bluesky/atproto, Threads, Mastodon/ActivityPub, Pixelfed), assessing
  what integrating our public side with Bluesky / AT Protocol gives us, costs us, and aligns with
  vs. the ActivityPub and proprietary alternatives. Keeps protocol-level and product-level facts
  distinct throughout; `[UNVERIFIED]` flagged. Status: research deliverable. (Its identity-layer
  follow-on, the DID-method decision, lives at `../thinking/plc-identity-resilience.md`.)

- `discord-matrix-groupchat.md` — feature & UX comparison of Discord (richness benchmark) and
  Matrix (our decentralized-encrypted cousin) for group chat, with a feature matrix rating each
  capability's fit against our stack as Natural / Effortful / Hard tied to specific layers (MLS
  epochs, Automerge, iroh, blind broker, DID, blob path). Headline: privacy-preserving behaviors
  are our free wins; the features that feel effortless in a centralized app (server-side search,
  presence, link previews) are exactly what the blind broker makes costly. Includes the
  private/public lanes analysis and a Bluesky/Germ/Wire-core-crypto feasibility dig. `[UNVERIFIED]`
  flagged. The design conclusion is written up at `../thinking/group-privacy-lanes-design-note.md`.

- `germ-xchat-features.md` — feature & UX comparison of Germ DM (MLS, atproto-native) vs. X Chat
  (Juicebox server-held keys), mapped against our stack as Natural / Effortful / Hard. Carries the
  interaction-tiers model (interactive / quiet-large / broadcast, type-at-creation not switchable
  mode), multi-device flagged as the primary open problem, and a durable-product design-principles
  appendix (three-audience settings, LTS-for-interfaces). Raw dialogue:
  `../seeds/transcripts/raw/germ-xchat-design-dialogue.md`.

- `atproto-private-data-architecture.md` — positions Croft's host-untrusted MLS group state + blind
  broker against where AT Proto's own private/non-public-data work is heading (the real community-led
  Private Data WG; GitHub #3363 "namespaces/buckets/realms" + #121 encryption). Headline: the atproto
  core team is choosing the **trusted-PDS** side of the line Croft sits on the other side of (zero-
  knowledge), so the direction *sharpens* Croft's differentiation. Covers the trusted-PDS-vs-ZK,
  cheap-self-host, and key-revocation contentions; the Germ "Anchor-Key-in-profile" idiom; and the
  unverified PDS-as-file-proxy idea (content-blind-mule rhyme). Related projects/tools registered in
  `../ECOSYSTEM.md` §5e/§6. Web-verified 2026-06-22; raw + FACTCHECK in `../seeds/transcripts/raw/`.

- `group-chat-failure-modes.md` — sourced analysis of the field's recurring group-chat failure
  modes (SSB fusion-identity, Megolm forward-secrecy nuance, MLS ordering/fork unsolved-merge,
  Keybase's trusted-server answer to per-device ordering), our model graded against each, the
  ranked case-against, and open questions. Headline: the decentralized-MLS "referee problem" is
  the consensus-acknowledged wall, not paranoia.

- `group-chat-failure-modes-plain.md` — the same analysis in plain English with examples/analogies
  (diary-you-can-only-add-pages-to, the train-track-tunnel referee problem, lost-key recovery).

- `p2p-founder-motivations-adoption.md` — research on why founders built 16 P2P/local-first
  projects (SSB, Briar, Reticulum, Nostr, Veilid, Keet, Matrix, Signal, BitChat, etc.) and whether
  anyone crossed the chasm. Finds only Signal did; institutional mandate (Matrix) is a possible
  fourth bridge; the Staltz paradox (building intimacy tools among strangers) is the human warning.
  Sourced inline; self-reported numbers flagged.

- `socialization-and-publication-venues.md` — where to socialize and share the Croft spec: a
  per-layer venue map (iroh/n0, MLS WG, W3C Credentials CG, atproto WG, Willow/Earthstar, Local-First
  Conf, etc.), standards bodies (IETF/IRTF/W3C), academic venues (NSDI/USENIX Sec/PoPETs), and
  defensive-publication channels (IETF I-D, arXiv, IP.com). Two-track framing (prior-art vs. spread)
  and a prioritized sequence. Current (2026) facts verified; `[UNVERIFIED]` flagged. Pairs with the
  IP-protection doc below.

- `open-publication-and-ip-protection.md` — IP strategy for protecting the spec when the goal is open
  sharing: the patent vs. Creative-Commons vs. defensive-publication fork. Decision recorded:
  **defensive publication** (spread + use + block-others-patenting, no own-patent) — CC-BY 4.0 for the
  document, Apache-2.0 for the reference implementation (express patent grant), and complete,
  timestamped, examiner-discoverable prior art (IETF Internet-Draft first, then arXiv) as the actual
  patent-blocking instrument. Not legal advice; the disclosure caveat is one-directional. Pairs with
  the venues doc above.

- `dating-friendship-meetup-fit-2026-07.md` — commissioned use-case-fit analysis of dating,
  friendship, and meetup apps (2025-26 landscape) against the Drystone group substrate. Sorts each
  need into provenance-shaped / utility-shaped / centralized-model-consequence, then verdicts the
  three sub-domains: **meetup/community is the recommended beachhead**, friendship a moderate fit,
  dating a narrow trust-sidecar only after a real private layer. Point-in-time snapshot; vendor
  numbers attributed not endorsed. Feeds ROADMAP_TODO D11.

## Anticipated (different audiences, same underlying research)

The same comparative material will likely be re-cut for different needs — e.g. a
funder/partner-facing brief, a public-facing "why this is different" piece, or a
threat-model-focused security comparison. When that happens, each lives here as its own
audience-targeted document rather than overloading the design-lens analysis.

Refresh discipline matches ECOSYSTEM.md: verify volatile current-state before external use.
