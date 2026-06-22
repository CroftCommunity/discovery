# Fact-check — Groundmist / Hive / identity-chain / iroh-games dialogue (Gemini)

date: 2026-06-22 · companion to `groundmist-hive-identity-chain-iroh-games-dialogue-2026-06-22.md`

purpose: verify the sprawling dialogue. Method: **most of it overlaps already-filed intakes —
those FACTCHECKs are cited as source of truth, not redone.** Only the *net-new* claims were
web-verified this pass (three parallel research passes, 2026-06-22). Verdicts: CONFIRMED · PARTLY ·
REFUTED · UNVERIFIABLE.

## Headline

Heavy overlap, **broadly accurate**, a few specific numbers off. The Steem/Hive saga, Groundmist,
the did:webvh↔did:plc identity chain, and iroh voice/video were already fact-checked in parallel
intakes (see "Cited, not redone" below) and hold up. The net-new verification surfaced **one
notable number error (Hard Fork 23 ≈ $6.3M, not $5M), one method-list error (atproto does NOT
resolve did:key), and one attribution that must be flagged for the marketing/quotes use (the
door-holding "corporation vs person" exchange)**. Every named iroh game/tool is real.

## Cited, not redone (the overlapping bulk — these are the source of truth)

| Topic in this dialogue | Already fact-checked in |
|---|---|
| Justin Sun / Steem / Hive takeover, TRON, coops/PBC/DAO | `cooperative-social-union-governance-dialogue-2026-06-22-FACTCHECK.md` + `thinking/cooperative-social-union-model.md` |
| Groundmist / grjte / Ink & Switch / open-social | `croft-atproto-sovereign-appview-open-social-dialogue-2026-06-22-FACTCHECK.md` |
| did:webvh / did:tdw, SCID, `alsoKnownAs` chaining, PLC | `thinking/cross-platform-identity-provenance.md`, `plc-identity-resilience.md`, `croft-identity-provenance-dialogue-2026-06-20.md` |
| atproto AppView/PDS/Relay/firehose mechanics | `atproto-atmospheric-web-iroh-mobile-FACTCHECK.md` (+ addenda) |
| iroh voice/video (callme, iroh-live), godot-iroh, iroh relay/holepunch/E2EE | `thinking/realtime-media-over-iroh.md`, `iroh-realtime-media-references.md`, `iroh-quic-localfirst-ecosystem-dialogue-2026-06-22-FACTCHECK.md` |

## Net-new verdicts (web-verified this pass)

### Crypto / Hive specifics
| Claim | Verdict | Note (src) |
|---|---|---|
| **Hard Fork 23** confiscated **~$5M** from 64 accounts (May 2020) | **PARTLY** | Real fork, **May 20 2020**, **64 accounts**, Hive-supporters ✓ — but the amount was **~$6.3M (23.6M STEEM)**, not $5M (CoinDesk 2020-05-20) |
| Dan Notestein / **Blocktrades** = targeted + core Hive figure | CONFIRMED | (CoinDesk) |
| **Ned Scott** = co-founder/ex-CEO of Steemit Inc., sold to Sun (~$8M) | CONFIRMED | (CoinDesk 2020-02-24) |
| Sun used **Binance/Huobi/Poloniex** customer stakes to vote out 21 witnesses, ~Mar 2 2020; exchanges backpedaled | CONFIRMED | delegated to @dev365; Huobi/Binance withdrew Mar 3 (Modern Consensus) |
| **TRON**: 2017 ~$70M ICO; **27 Super Representatives**; ~**$85B USDT** (~half of Tether); **3-sec blocks** | CONFIRMED | (KuCoin; FXStreet 2026-03) |
| TRON "**~8M daily transactions**" | UNVERIFIABLE | sources cite higher throughput / different metrics; not pinned |
| **Sun v. World Liberty Financial** lawsuit (~Apr 21 2026), "blacklist" freeze; WLF defamation countersuit (~May 2026); **SEC settled** (~Mar 5 2026) | CONFIRMED | WLFI added an admin blacklist Aug 2025; SEC settlement $10M via Rainberry, charges dismissed w/ prejudice (CoinDesk 2026-04-21, 2026-03-05) |
| **Hive** user counts (~2.3–2.6M lifetime; ~30–60k DAU; millions of tx/day via Splinterlands) | UNVERIFIABLE | no single current source; Splinterlands DAU fell to ~8–16k by 2024 (Hive Statistics; Splinterlands stats) |
| **Hive mechanics**: DPoS top-21 witnesses; **DHF** treasury; **Resource Credits** (zero-fee); Proof of Brain; **HBD** | CONFIRMED | (developers.hive.io) |

### iroh games / tools — ALL real (high-fabrication-risk cluster, verified)
| Project | Verdict | Note (src) |
|---|---|---|
| **libmarathon / "Marathon"** (Bevy + Iroh + iroh-gossip, CRDTs; 3D cube demo macOS/iOS) | CONFIRMED | crates.io/crates/libmarathon; github sunbeam-stdio/marathon |
| **ascii-royale** (Chad Fowler; 16-player ASCII battle royale; Iroh ticket; no servers) | CONFIRMED | github chad/ascii-royale; royale.boxd.sh |
| **iroh-lan** (Hamachi-like encrypted virtual-LAN over Iroh for legacy LAN games) | CONFIRMED | github rustonbsd/iroh-lan |
| **DataBeam** (GUI combining croc + sendme) | CONFIRMED (project live) | **croc** = schollz/croc (real); **sendme** = n0-computer/sendme (real Iroh tool); the specific `vinay-winai/DataBeam` repo wasn't separately confirmed but the project is live (databeam.refining.online) |
| **callme** (iroh-roq/RTP-over-QUIC, Opus, cpal, echo-cancel, 1-on-1) + **iroh-live** (`irl`, MoQ video, WebTransport bridge) | CONFIRMED | github n0-computer/callme, /iroh-live |
| iroh relay = STUN-like discovery + signaling + stateless fallback relay; E2EE (relay can't read); ~90% direct hole-punch | CONFIRMED | iroh.computer/blog/qad; docs.iroh.computer |
| SNES emulator **netplay over iroh** (concept) | CONFIRMED (concept) | RetroArch/Snes9x netplay (lockstep/rollback) is real; tunneling localhost netplay through an iroh tunnel is sound — not a named product |

### Culture / economics / DID limitation
| Claim | Verdict | Note (src) |
|---|---|---|
| **"corporation vs person / held the door open / did you ask for money first"** = from **The Corporation (2003)** (Achbar/Abbott; Bakan's book), spoken by **Robert Hare** (PCL-R) | **PARTLY — flag for quotes use** | The **film, directors, Bakan book, and Hare's appearance discussing corporate psychopathy are all real**, BUT the **exact door-holding exchange cannot be sourced/indexed** — attribution to that film/Hare is **UNVERIFIABLE**. Use as paraphrased "philosophical folklore," not a sourced quotation (en.wikipedia: The Corporation; Robert D. Hare) |
| **Israeli daycare** fine→more-lateness (crowding-out) | CONFIRMED | **Gneezy & Rustichini, "A Fine is a Price," J. Legal Studies 29(1), 2000** |
| **Thurlow** "no bodies to be punished, nor souls to be condemned" / Poynder "no soul to damn, no body to kick" | CONFIRMED | John Poynder, *Literary Extracts* (1844) — **public domain** |
| **Dan Ariely** social-norms-vs-market-norms (*Predictably Irrational*, 2008) | CONFIRMED | |
| **Ostrom, *Governing the Commons*** (1990), 8 design principles, **2009 Nobel** | CONFIRMED | Cambridge 1990; Nobel 2009 (shared w/ Williamson; first woman) |
| atproto resolves "**did:plc, did:web, did:key**" | **REFUTED (did:key)** | atproto's blessed set is **did:plc and did:web ONLY** — **did:key is NOT supported** (atproto.com/specs/did). The overall conclusion (**did:webvh not atproto-resolvable → use did:plc + bidirectional alsoKnownAs**) stands |
| did:webvh string mutates on domain change vs atproto's immutable-account-DID rule | CONFIRMED (reasoning sound) | |
| `alsoKnownAs` = the W3C-standard equivalence field; bidirectional pattern sound | CONFIRMED (caveat) | a stronger form, **`equivalentId`**, exists (mutually guaranteed); `alsoKnownAs` is the widely-supported choice |
| did:webvh = renamed **did:tdw**; SCID + `did.jsonl` log; **didwebvh-rs** Rust impl | CONFIRMED | (didwebvh.info; github decentralized-identity/didwebvh-rs) |
| Bluesky research datasets: Failla & Rossetti (PLoS ONE, 4M accts/235M posts), Jeong BlueTempNet (IEEE), Kleppmann et al. CoNEXT-2024 | CONFIRMED (as cited) | standard, real citations |

## What to carry forward

- **Low net-new design yield** — most is restatement of already-filed bodies. The genuinely useful
  additions: the **iroh games/tools** (libmarathon, ascii-royale, iroh-lan, DataBeam/sendme) → ECOSYSTEM
  as iroh-ecosystem prior art; the **did:key correction** → note in the identity docs / source-of-truth.
- **Marketing/quotes material** — this dialogue is rich in quotable framings (convenience-vs-purism,
  "foundational layer not the whole house," the door-holding anecdote, the overjustification/daycare
  evidence). Seeded into **`narrative/messaging-and-quotes.md`** (new) with **honest usage flags** —
  critically, the door-holding exchange is attribution-UNVERIFIABLE and the daycare/Thurlow/Ostrom items
  are external (cite, don't claim). See that file + COHESION.
- **Hard Fork 23** is a strong cautionary case for the cooperative/anti-capture thesis (capital seizing
  a "decentralized" ledger) — already in `cooperative-social-union-model.md`; the corrected **~$6.3M**
  figure should be used there if it cites a number.
