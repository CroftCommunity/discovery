# Fact-check — atproto / Bluesky PDS architecture, Germ, object-storage shims, private data (Gemini)

date: 2026-06-22 · companion to `atproto-pds-architecture-technical-dialogue-2026-06-22.md`

purpose: AI-generated (Gemini) dialogue, fact-checked at the user's request against atproto.com,
docs.bsky.app, the bluesky-social GitHub, atproto.wiki, and provider pages. Verdicts: **CONFIRMED** ·
**PARTLY** · **REFUTED** · **UNVERIFIABLE**. For settled atproto/iroh facts the source of truth is
`atproto-atmospheric-web-iroh-mobile-FACTCHECK.md` (note: atproto repos DO use Merkle Search Trees;
iroh-docs does NOT — don't cross-wire the two).

## Headline

**Architecture is accurate; one numeric block is fabricated.** PDS/Relay/AppView roles, the MST repo
+ CID blob split, Lexicons, the exact XRPC endpoint names, the **Private Data Working Group** (real,
with both GitHub discussions confirmed), and **Germ** (Mark Xue ex-Apple, MLS/RFC 9420, IETF
`draft-xue-distributed-mls`, Protocol Labs Cypherpunk Fellowship, Blacksky integration) all
**CONFIRM**. **Do NOT rely on the dialogue's "current" federation limits** ("100 accounts / 2,600
events-hr / 50/s burst / 21,000-day") — they are **not in the spec or PDS config and appear
fabricated**; only the **Feb-2024 early-access numbers** (10 accounts / 1,500-hr / 10,000-day) are
real, and **open federation without pre-registration is live (since May 2024)**. Minor: iDrive e2
~$5/TB (not $4), Hetzner ~$6.59 (not $5.99); Germ "Android waitlist" URL + "ger.mx" routing
**[UNVERIFIED]**; "Namespaces" terminology + explicit Frazee attribution not verbatim-confirmed. Do
not copy code/Caddyfile/SQL snippets as working code.

## Verdict table

| # | Claim | Verdict | Note (src) |
|---|---|---|---|
| 1 | PDS/Relay/AppView roles + `com.atproto.sync.subscribeRepos` firehose | CONFIRMED | PDS holds repo, Relay aggregates firehose, AppView indexes/builds UIs (atproto.com/specs/sync, /guides/glossary) |
| 2 | Repo = MST of signed records; blobs separate, CID (CIDv1, base32, SHA-256, `bafkrei…`) | CONFIRMED | MST key/value; commit signed; blob CID = CIDv1 raw codec + SHA-256 + base32 `b`-prefix (atproto.com/specs/repository, /specs/blob) |
| 3 | Lexicons; app.bsky.* namespace; putRecord/listRecords/uploadBlob/sync.getBlob | CONFIRMED | All four endpoint names exact; `uploadBlob` is `com.atproto.repo.uploadBlob`, `getBlob` is `com.atproto.sync.getBlob` (docs.bsky.app/docs/api) |
| 4 | Official PDS (TS) per-actor SQLite, S3 blobs, Caddy; Cocoon (Go) + rsky-pds (Rust) w/ Postgres | CONFIRMED | Per-actor SQLite default, Caddy, S3 blobs; Cocoon (haileyok); rsky-pds (blacksky-algorithms/rsky) (atproto.com/guides/self-hosting; github haileyok/cocoon; blacksky-algorithms/rsky) |
| 5 | Early-access (Feb 2024) limits: 10 accts, 1,500-hr, 10,000-day, Discord allowlist | CONFIRMED | Blog states exactly these; Discord registration was the initial allowlist (docs.bsky.app/blog/self-host-federation) |
| 6 | Current: open federation live; 100 accts / ~2,600-hr (50/s burst) / ~21,000-day defaults | **PARTLY / suspect** | Open federation w/o pre-registration CONFIRMED (since May 2024); the **100 / 2,600 / 21,000 numbers are UNVERIFIABLE** — not in spec or PDS config (env.ts has no such defaults); likely fabricated |
| 7 | DMs launched May 2024, 1:1 text, not E2EE, separate; Everyone/Follow/No One; groups rolling out | CONFIRMED | Launched May 22 2024; no encryption/images at launch; settings as stated; group DMs planned (bsky.social/about/blog/05-22-2024) |
| 8 | Germ: MLS (RFC 9420) E2EE, ATProto, iOS-only, ex-Apple iMessage eng, Cypherpunk Fellowship, ACP, IETF distributed-mls, ger.mx, Blacksky | PARTLY (mostly confirmed) | MLS, ATProto, iOS-only, **Mark Xue (ex-Apple FaceTime/iMessage)**, Protocol Labs Cypherpunk Fellowship, Autonomous Communicator Protocol, IETF `draft-xue-distributed-mls`, Blacksky — all CONFIRMED; **Android-waitlist URL + ger.mx routing [UNVERIFIED]** (germnetwork.com/blog/integrating-germ-atproto; datatracker draft-xue-distributed-mls) |
| 9 | "Private Data Working Group" exists; Frazee involved; namespaces/private records via scoped OAuth+ACLs; Discussions #3363 & #121 | CONFIRMED | WG real (atproto.wiki/working-groups/private-data); private lexicon records, ACL-based, in-transit (not E2EE/at-rest), via OAuth; #3363 + #121 both exist; "Namespaces" term + explicit Frazee attribution not verbatim-confirmed |
| 10 | ElfHosted ~$9/mo; Hostinger ~$6.49; DigitalOcean 1-click ~$4-6; Vultr ~$5 | PARTLY | ElfHosted **$9/mo** ✓; DigitalOcean 1-click PDS ✓ (~$6 droplet); **Hostinger & Vultr PDS templates [UNVERIFIED]** (store.elfhosted.com; marketplace.digitalocean.com) |
| 11 | Storage/TB: iDrive e2 ~$4, Hetzner ~$5.99, B2 ~$6 (free egress via Cloudflare), Wasabi ~$6.99 (90-day min, free egress), R2 ~$15 (zero egress); cold tiers ~$1/TB | PARTLY (mostly confirmed) | iDrive e2 **~$5** (not $4); Hetzner **~$6.59** (not $5.99); B2 $6 + Cloudflare Bandwidth-Alliance free egress ✓; Wasabi $6.99 + 90-day retention + free egress ✓; R2 $15/zero-egress ✓; Glacier Deep ~$1/TB ✓; Azure/Google exact figures [UNVERIFIED] (backblaze pricing; aws glacier) |

## Corrections that matter for Croft design

- **Federation limits:** for any Croft PDS planning, treat the "current default" numbers as
  **unknown** — open federation is live, but specific account/event caps must be read from the actual
  PDS `env.ts` config / current Bluesky docs, not this dialogue. The Feb-2024 numbers are historical.
- **The Private Data Working Group is real and directly relevant** to Croft's private-groups thesis:
  the in-transit-ACL-via-OAuth model (private records bypassing the MST + firehose) is the atproto
  team's current direction — distinct from true E2EE/at-rest, which is exactly the gap Croft's
  lineage-groups MLS proof answers. Link in COHESION (mirrors the existing "no native AT-Proto E2EE"
  finding).
- **Germ** is a genuine, MLS-based, atproto-integrated precedent (the IETF `draft-xue-distributed-mls`
  is a real standardization effort) — strengthens the ECOSYSTEM Germ row; Mark Xue is the named
  ex-Apple engineer.
- **The object-storage shim patterns** (streaming pipeline, reverse-proxy CID interceptor, SQLite
  injection) are plausible architecture but **experimental and fragile** (PDS schema changes break the
  SQL-injection variant) — flag as such; do not present the snippets as working code. Pricing tiers
  are roughly right with the iDrive/Hetzner corrections.
- **Alternative PDS impls** Cocoon (Go) + rsky-pds (Rust, blacksky) are real and support Postgres —
  relevant if Croft ever needs a shared-datastore PDS.

## Provenance

Web verification 2026-06-22 via a dedicated research pass (27 tool calls); source URLs in the table.
Some PDS-hosting/object-storage economics overlap the in-flight
`crypto-wars-to-p2p-pds-economics-dialogue-2026-06-22.md` — dedup in COHESION rather than duplicating.
