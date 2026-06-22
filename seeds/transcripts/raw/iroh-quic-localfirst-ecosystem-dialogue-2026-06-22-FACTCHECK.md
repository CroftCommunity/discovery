# Fact-check — Iroh / QUIC / local-first stack & ecosystem dialogue (Gemini)

date: 2026-06-22 · companion to `iroh-quic-localfirst-ecosystem-dialogue-2026-06-22.md`

purpose: the raw dialogue is AI-generated (Gemini, flagged by the user as sometimes unreliable). Per
the user's request, every substantive assertion was fact-checked against primary sources (n0/iroh
blog + docs, GitHub, crates.io, awesome-iroh) plus the project's own settled facts. Verdicts:
**CONFIRMED** · **PARTLY** · **REFUTED** · **UNVERIFIABLE**. For settled iroh facts the source of
truth is `atproto-atmospheric-web-iroh-mobile-FACTCHECK.md` (iroh `1.0.0`; iroh-docs = range-based
set reconciliation + LWW, **not** MSTs; `NodeId`→`EndpointId`; relays formerly "DERP").

## Headline

**Unusually accurate skeleton — no fabricated projects.** Every named crate/project resolves to a
real repo. Gemini's residual drift here is on **exact API strings and relationship framing**, not
existence. Three corrections matter: (1) the Automerge ALPN string is **`iroh/automerge/2`**, not the
dialogue's `/iroh-automerge/1`; (2) Christian Huitema is a **parallel draft co-author** of the QUIC
NAT-traversal / address-discovery work, not an endorser who "validated/praised iroh" — the dialogue
overstates a peer relationship as endorsement; (3) Peat's "shifted from `ring` to `aws-lc-rs` for
FIPS" detail is **[UNVERIFIED]** (FIPS-via-approved-primitives is documented; the specific switch is
not sourced). Do not copy code snippets as working code.

## Verdict table

| # | Claim | Verdict | Note (src) |
|---|---|---|---|
| 1 | noq = "Number 0 QUIC", pure-Rust general QUIC by n0 powering iroh | CONFIRMED | Forked from Quinn; powers iroh since ~v0.96; multipath + NAT traversal (iroh.computer/blog/noq-announcement) |
| 2 | Iroh contributed to QUIC Multipath; Huitema validated/praised it | PARTLY | Multipath work real; Huitema **co-authored** the QUIC NAT-traversal/address-discovery drafts as a parallel effort — "praised/validated iroh" overstates it as endorsement (iroh.computer/blog/iroh-on-QUIC-multipath; datatracker draft-ietf-quic-address-discovery) |
| 3 | QAD via OBSERVED_ADDRESS frames; QNT real in iroh | CONFIRMED | QAD exclusive since iroh 0.90 (QAD+STUN since 0.32); QUIC NAT-traversal draft implemented in noq (iroh.computer/blog/qad) |
| 4 | ~90% hole-punch; relays stateless + blind via TLS 1.3 | CONFIRMED | ~9/10 hole-punch, ~95% data over direct; relays forward encrypted packets, stateless (docs.iroh.computer/about/faq) |
| 5 | Automerge + iroh; ALPN routing e.g. `/iroh-automerge/1` | PARTLY | Integration real (iroh-examples); actual ALPN is **`iroh/automerge/2`** (version 2), not `/iroh-automerge/1` (github n0-computer/iroh-examples) |
| 6 | CRDT alts: Loro, Y-CRDT (Yrs, port of Yjs), Diamond Types | CONFIRMED | All three real Rust crates, accurately described (loro-dev/loro; y-crdt/y-crdt; josephg/diamond-types) |
| 7 | Peat/peat-mesh by Defense Unicorns; Iroh+Automerge; BLE fallback peat-btle; ring→aws-lc-rs for FIPS | PARTLY | Peat, peat-mesh, peat-btle (BLE) all CONFIRMED (Defense Unicorns); the specific `ring`→`aws-lc-rs` switch **[UNVERIFIED]** (FIPS-approved-primitives documented, switch not sourced) (github defenseunicorns/peat-mesh) |
| 8 | Prime Intellect (prime-vllm/prime-iroh); Tandemn (tensor-iroh) | CONFIRMED | prime-iroh = "async P2P backend for decentralized pipeline parallelism"; Tandemn = "iroh for tensors" (github PrimeIntellect-ai/prime-iroh; awesome-iroh) |
| 9 | Bones engine + godot-iroh for P2P game netcode | CONFIRMED | godot-iroh (tipragot) real; Bones networked with iroh (Fish Folk: Jumpy) |
| 10 | cross.stream; iroh-ssh; Obsiroh | CONFIRMED | cross.stream (local-first event streaming, P2P via iroh); Obsiroh (Obsidian sync); iroh-ssh in awesome-iroh |
| 11 | Hubris; Teamtype; Zeco; Dash Chat (mDNS + wide-area) | CONFIRMED | All four real, accurately described; Dash Chat by darksoil studio (mDNS + DHT, works during shutdowns) |
| 12 | Holochain rewriting Kitsune2 networking onto iroh | CONFIRMED | Default transport switched tx5→iroh in Holochain v0.6.1; "Kitsune2 Iroh Transport" task (blog.holochain.org) |

## Corrections that matter for Croft design

- **ALPN string:** if Croft references the iroh+automerge example, the protocol string is
  `iroh/automerge/2`, not `/iroh-automerge/1`. Minor, but copy-paste-relevant.
- **Huitema framing:** treat the QUIC-multipath / NAT-traversal work as **collaborative IETF draft
  activity n0 implements and contributes to**, not "Huitema endorsed iroh." Don't repeat the
  endorsement framing in our docs.
- **Peat FIPS detail:** mark the `ring`→`aws-lc-rs` claim `[UNVERIFIED]` until confirmed against
  Peat's Cargo manifest. Peat/peat-mesh/peat-btle themselves are real and a genuinely relevant
  Iroh+Automerge+BLE-fallback precedent (note: overlaps the in-flight
  `crypto-wars-to-p2p-pds-economics` filing — dedup in COHESION).
- **No iroh-docs MST conflation here** (unlike the atproto-atmospheric dialogue) — iroh-docs is
  correctly described as range-based set reconciliation + BLAKE3/iroh-blobs split.

## Provenance

Web verification 2026-06-22 via a dedicated research pass (18 tool calls); source URLs in the table.
Internal iroh ground truth: `atproto-atmospheric-web-iroh-mobile-FACTCHECK.md` +
`experiments/iroh/relay-lab-runs/IROH-1.0.0-API-VERIFIED.md`.
