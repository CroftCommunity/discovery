# Substrate prior art: what Croft's stack builds among

`Status: cairn layer (Layer 3, the open field). Register: survey / homage. Resolution: library — the full
prior-art grounding for the substrate bet; for the transport mechanics themselves see the impl transport
notes. External facts carry verification flags; iroh version facts cite the FACTCHECK source of truth and
are not re-verified here. Rows marked [UNVERIFIED] or dialogue-sourced need a refresh pass before external
use.`

## Overview

Croft's substrate bet — Rust, iroh for QUIC-first peer-to-peer transport, a CRDT / local-first data layer,
and MLS for group encryption — was not invented in a vacuum. This document registers the prior art that
bet builds among: the one production system that already assembles the *exact* combination (Peat), the
recursive and locality-aware routing lineage that grounds the federation-routing direction (RINA, Named
Data Networking, Yggdrasil, cjdns), the substrates that were considered and set aside (libp2p, Veilid,
Holochain) together with the reasons they were, and the near neighbors worth reading before the capability
layer hardens (p2panda, iroh-rings). The load-bearing conclusion is that Croft's stack is a *recombination
of proven parts*, and the strongest evidence it can carry is that a defense-grade system already ships that
recombination in the field.

## Charter: what this document covers

- **In scope:** substrate, transport, and data-layer prior art, and the reason Croft credits, reuses, or
  set each aside.
- **Out of scope (and where it lives):** iroh's own mechanics (the impl transport notes); MLS and the
  decentralized-MLS siblings (`mls-and-mimi.md`); the Willow/Meadowcap capability layer
  (`willow-meadowcap.md`); the atproto ecosystem (`atproto-ecosystem.md`).
- **Boundary call:** this is the "why these substrate choices, and among whom" register. The *how* of the
  chosen transport lives in impl; here we carry only what grounds the choice.

## Peat — the strongest prior art for the exact bet

**Peat** (Defense Unicorns), with its companion `peat-gateway`, is off-grid / denied-environment
peer-to-peer data-sync middleware written in Rust. It assembles **iroh** transport (QUIC, with a BLE
fallback), **Automerge** CRDTs, and **MLS** group security into a self-healing mesh that stitches together
servers, Android devices, Raspberry Pis, drones, and ESP32-class hardware; it integrates with ATAK, and
`peat-gateway` re-synchronizes to identity providers (Okta / Keycloak) when a link returns. It is active
open source, in production defense, disaster-response, and industrial use.

Why this is load-bearing rather than a mere neighbor: Peat is the single strongest existing prior art for
Croft's *exact* substrate combination — Rust + iroh + CRDT + MLS — and it is proven in denied and degraded
conditions, which is a harder environment than Croft's own. It answers the question every substrate bet has
to answer ("is this recombination real, or aspirational?") with a shipping affirmative. It is also the
clearest rebuttal to the consumer peer-to-peer graveyard (the pattern where pure-P2P consumer messengers
stall — see the cautionary cases in `adjacent-systems.md`): the lesson Peat embodies is *ship the
substrate, not a "P2P WhatsApp."* Croft and Peat differ in target — Peat is field-ops data sync, Croft is a
social substrate — but the load-bearing parts are the same, and that is exactly why Peat is the prior art
to watch and learn from. Relationship: build-on, learn↔.

`[confirm: Peat facts against github.com/defenseunicorns/peat before external use.]`

## The recursive / locality-aware routing lineage

Croft's federation-routing direction — scoped, locality-aware addressing that does not depend on a flat
global routing table — has a formal lineage worth crediting, because it is the difference between a
hand-waved "it will federate" and a direction with known prior art.

- **RINA** (Recursive InterNetwork Architecture, John Day): the thesis that "networking is one recursive
  layer repeated at scale," which bounds routing state by recursion rather than by a single global table.
  The closest formalization of Croft's recursive-federation routing direction. Research / academic.
- **Named Data Networking / Yggdrasil / cjdns:** NDN routes on aggregatable hierarchical names; Yggdrasil
  and cjdns route over cryptographic-identity trees with no global table and locality awareness. Yggdrasil
  is a working small-scale network and is roughly the shape of the federation-routing proof-of-concept
  target.

Why load-bearing: these ground the claim that no-global-table, locality-aware, scoped routing is a known
and buildable direction, relevant when Croft's federation routing hardens. Relationship: learn↔, build-on
(proof-of-concept prior art). `[dialogue-sourced 2026-06-20; verify before reliance.]`

## Considered and set aside (with the reasons kept)

The substrate choice was iroh; the reasons the alternatives were set aside travel with that decision,
because a choice whose reasons are dropped cannot be re-derived or defended.

- **libp2p** — a modular P2P stack (transports, pubsub, DHT). **Set aside as primary: mobile-weak relative
  to iroh.** Relationship: homage.
- **Veilid** — privacy-first P2P with source-address-free routing (Ed25519 / x25519 / XChaCha20 / BLAKE3 /
  Argon2; a DHT for small mutable records). **Demoted to a future metadata-resistant messaging-layer
  candidate: no large-blob primitive.** Relationship: learn↔ (future).
- **Holochain** — agent-centric P2P with no global consensus (source chains, rrDHT, membrane proofs).
  **Dropped as substrate: it runs on iroh transport anyway, and it is mobile-weak.** Relationship: homage
  (borrow patterns). Note the reinforcing signal: Holochain adopting iroh (via Kitsune2) is itself evidence
  for the iroh bet.

The reason to keep these reasons visible is the anti-rollup rule: the decision (iroh over the
alternatives) is only trustworthy if the reason (mobile weakness, a missing blob primitive, a
consensus-model mismatch) matures alongside it. `[dependency-selection rationale, verified: dossier.]`

## Neighbors to read before the capability layer hardens

- **p2panda** — building blocks for peer-to-peer applications; the closest "p2p app building blocks"
  neighbor. Read its peer-equality framing before Croft's own peerhood model is treated as settled.
  `[dialogue-sourced 2026-06-24, pending verification.]`
- **iroh-rings** — relationship-based access control for resources over iroh; a direct neighbor to
  Drystone's peer-equality / capability layer. Compare it before the capability-mechanism decision (the
  Track A / Track B choice) hardens. `[dialogue-sourced 2026-06-24, pending verification.]`

## iroh itself

iroh (n0) is the transport substrate Croft depends on, not a prior-art option to choose among, so its
mechanics are documented in the impl transport notes rather than here. It is credited in this register as
the load-bearing dependency: QUIC-first peer-to-peer with EndpointId (an Ed25519 public key as the network
identity), hole-punching, relays, multipath connection migration, and first-party language bindings; it is
in production in Delta Chat, Nous Research (distributed LLM training), and Paycode (point-of-sale). Version
facts (iroh at 1.0; companion crates still pre-1.0) cite the FACTCHECK source of truth and are not
re-verified here.

## What this establishes (and does not)

Establishes that Croft's substrate is a recombination of proven parts rather than a novel gamble; that a
production, defense-grade system (Peat) already ships the exact Rust + iroh + CRDT + MLS combination in
denied and degraded conditions, which is the strongest available evidence the recombination holds; that the
federation-routing direction has a formal lineage (RINA, NDN, Yggdrasil, cjdns); and that the substrates
set aside were set aside for stated, checkable reasons that travel with the decision.

Does **not** re-document iroh, MLS, or Willow mechanics (they live elsewhere in cairn and impl), does
**not** resolve the capability-mechanism decision (the Track A / Track B choice), and does **not** certify
the dialogue-sourced rows — those carry verification flags and need a refresh pass against primary sources
before external use.
