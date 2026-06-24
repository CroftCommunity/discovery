# Fact-check — Open-social protocols & aggregators dialogue (Gemini)

date: 2026-06-22 · companion to `opensocial-nostr-farcaster-aggregators-dialogue-2026-06-22.md`

purpose: AI-generated (Gemini) dialogue, fact-checked at the user's request against primary sources
(official docs, GitHub, project sites, reputable news). Verdicts: **CONFIRMED** · **PARTLY** ·
**REFUTED** · **UNVERIFIABLE**.

## Headline

**Skeleton is solid — including the claims that looked most suspect.** The two big "this can't be
right" items both check out: the **Neynar→Farcaster acquisition (Jan 2026) with Merkle Manufactory
returning the full $180M** to investors, and **Mask Network assuming stewardship of Lens (Jan 2026)**.
Protocol mechanics, NIP/kind numbers, founders, parent companies, clients, repos, and licenses are
accurate. Gemini's residual drift is in **dollar figures and product micro-characterizations**:
Farcaster storage rent is ~$7/storage-unit/yr (historically ~$2), **not $5**; cumulative storage
revenue peaked ~$1.91M (Jul 2024), **not $2.8M** (unconfirmed); **Clanker is an AI token-launchpad,
not a "trading tool."** A few exact price points (Clovyr ~$10/mo, thirdweb $9/$99/$499 tiers) are
**[UNVERIFIED]** — products/tiers exist, dollar figures unconfirmed from primary sources.

## Verdict table

| # | Claim | Verdict | Note (src) |
|---|---|---|---|
| 1 | Nostr = "Notes and Other Stuff Transmitted by Relays"; npub/nsec; relays don't talk | CONFIRMED | Acronym, NIP-19 npub/nsec, relay isolation correct (github nostr-protocol/nostr) |
| 2 | Clients: Damus, Amethyst, Primal, Coracle, Snort, Iris | CONFIRMED | All six real, active (nostrapps.com) |
| 3 | Blossom = SHA-256 content-addressed media; kind:24242 auth, kind:10063 server lists; "BUDs" | CONFIRMED | Kinds correct; BUD = Blossom Upgrade Documents (github hzrd149/blossom) |
| 4 | Relays damus.io (Casarin), nos.lol, snort, nostr.wine, cache.primal.net, purplerelay; nostr.watch | CONFIRMED | Casarin (@jb55) runs Damus relay; nostr.watch is the directory |
| 5 | NIP-29 + NIP-42 + NIP-59 "Gift Wrapping"; Marmot = Nostr+MLS glue; White Noise; Amethyst MLS | CONFIRMED | NIP numbers exact; Marmot + White Noise (parres-hq) real (github marmot-protocol/marmot) |
| 6 | nostr-rs-relay (Rust); Clovyr managed relay ~$10/mo | PARTLY | nostr-rs-relay (scsibug, Rust) ✓; Clovyr offers it managed but exact ~$10/mo **[UNVERIFIED]** |
| 7 | Farcaster 2020 Romero+Srinivasan; Merkle Manufactory; Optimism; Frames; rent ~$5/yr; Warpcast closed; Hubble; Snapchain (Rust >10k tps); $180M ($150M Series A) | PARTLY | All correct EXCEPT rent ~**$7**/storage-unit/yr (historically ~$2), not $5; Hubble is TS+Rust; $180M = $150M Series A + ~$30M seed (github farcasterxyz/snapchain) |
| 8 | Jan 2026 Farcaster acquired by Neynar; founders returning full $180M | CONFIRMED | Real — announced Jan 2026; Merkle returning full $180M; sold protocol+Warpcast+Clanker to Neynar (theblock.co/post/386816) |
| 9 | DAU ~40-60k; Power Badge ~4,500-5,000; cumulative rent ~$2.8M; Clanker autonomous trading tool | PARTLY | DAU plausible (bot-inflated); Power Badge ~4,360 (close); cumulative revenue peaked **~$1.91M** Jul 2024 — $2.8M **[UNVERIFIED]**; **Clanker = AI token-launchpad, not a "trading tool"** |
| 10 | OSS clients: Herocast, Opencast, Litecast, Yup | CONFIRMED | All four real (a16z/awesome-farcaster) |
| 11 | Lens 2022 Kulechov (Aave); Avara; Profile NFTs; Polygon→Lens Chain (zkSync ZK Stack); Momoka L3; Open Actions; ~$46M; Mask stewardship | CONFIRMED | All correct; Momoka = optimistic L3 over Polygon; Mask took stewardship Jan 2026 (operational, not IP/governance transfer) (theblock.co/post/386293) |
| 12 | thirdweb 2021 Rydhan/Loo/Bartlett; $24M Series A (Haun); tiers $9/$99/$499 | PARTLY | Founders, 2021, $24M Series A (Haun) ✓; exact pricing tiers **[UNVERIFIED]** (prnewswire thirdweb Series A) |
| 13 | Yup by Kabessa+Johnson; thirdweb acq. Feb 2025 then sunset; archive andrei0x309/yup-live (Vue3/Turborepo/Ionic/Tauri) | CONFIRMED | All confirmed incl. repo + full stack (github andrei0x309/yup-live) |
| 14 | Firefly (Mask); Bridgy Fed (CC0, AP↔ATProto, Ryan Barrett); Flare (DimensionDev, Kotlin MP, AGPL-3.0); SkyFeed (Dart/Flutter, EUPL-1.2); Mixpost (inovector); CrossPoster (run-llama) | CONFIRMED | All projects/authors/stacks confirmed; Flare AGPL-3.0 + SkyFeed EUPL-1.2 not individually re-verified (github snarfed/bridgy-fed; DimensionDev/Flare; run-llama/crossposter) |
| 15 | Mask founder Suji Yan; Bonfire Union $100M; owns mstdn.jp, mastodon.cloud, Pawoo.net; Web3.bio; Orb | CONFIRMED | All confirmed (Pawoo acquired 2022; Bonfire Union two funds = $100M) |
| 16 | Arbitrum grants; ArbiFuel gas sponsorship; $10M audit program; $1M "Trailblazer" AI-agent | CONFIRMED | All real and correctly named (ArbiFuel May 2025–Jan 2026; $10M audit/12mo; AI Trailblazer $1M) (blog.arbitrum.foundation) |

## Corrections that matter for Croft / ECOSYSTEM rows

- **Farcaster economics:** storage rent ~$7/unit (not $5); cumulative storage revenue ~$1.91M peak
  (not $2.8M, mark `[UNVERIFIED]`). The Neynar acquisition + $180M return is **real** and a notable
  data point on the SocialFi→infrastructure pivot (relevant to ECOSYSTEM "learn↔" framing).
- **Clanker:** an AI **token-launchpad**, not a generic "social trading tool" — correct any ECOSYSTEM
  row.
- **Mask→Lens stewardship** (Jan 2026) is real but **operational**, not an IP/governance transfer —
  phrase carefully.
- **Aggregator landscape is genuinely rich and forkable** (Flare AGPL-3.0, Bridgy Fed CC0, yup-live
  archive, Mixpost, SkyFeed, CrossPoster) — directly relevant to the Body-3 "fork a client" strategy;
  flag licenses (AGPL copyleft vs CC0 freedom) in any distilled note.
- **Unverified price points** (Clovyr, thirdweb tiers) — mark `[UNVERIFIED]` in any cost table.

## Provenance

Web verification 2026-06-22 via a dedicated research pass (25 tool calls); source URLs in the table.
ECOSYSTEM/research rows distilled from this dialogue but not independently re-verified beyond the
above must be flagged **dialogue-sourced / pending-verification** per PLAYBOOK §3.
