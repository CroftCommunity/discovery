# Fact-check — AT Proto architecture explainer (Gemini)

date: 2026-06-22 · companion to `atproto-architecture-appview-relay-explainer-2026-06-22.md`

purpose: verify the architecture explainer. Method: cite the project source of truth for settled
atproto mechanics (`atproto-atmospheric-web-iroh-mobile-FACTCHECK.md` + its dated addenda), and do
targeted live web checks (atproto.com / docs.bsky.app / did-method-plc spec / Bryan Newbold's relay
write-ups / GitHub) only for the *new specific* claims. Verdicts: **CONFIRMED** · **PARTLY** (real
but mis-described / outdated) · **REFUTED** · **UNVERIFIABLE**.

## Headline

**Unusually accurate.** The AppView/PDS/Relay/Lexicon/DID/firehose mechanics are overwhelmingly
correct and largely **restate facts already settled in this corpus** (atproto FACTCHECK,
`thinking/atproto-atmospheric-web.md`, `plc-identity-resilience.md`, `cross-platform-identity-provenance.md`).
The whole explainer is a clean teaching pass with **one fabrication**, **one outdated claim**, and
three details that smelled fabricated but checked out.

## REFUTED — do not propagate

1. **"did:plc = *Public Liaison Corporation*."** PLC stands for **"Public Ledger of Credentials"**
   (the did-method-plc spec / `did-method-plc` repo). The "Public Liaison Corporation" expansion is
   invented. (The corpus already discusses did:plc heavily — `plc-identity-resilience.md`,
   `cross-platform-identity-provenance.md` — so this correction matters only to stop the bad
   backronym spreading.) (src: atproto.com/specs/did-plc; web.plc.directory/spec/v0.1/did-plc)

## PARTLY / OUTDATED — the one substantive mechanics error

2. **"Relays maintain a full backup copy of every user's data repository across the entire
   network"** and **"call `com.atproto.sync.listRepos` / `getRepo` *on the Relay*"** to backfill the
   whole network. This describes the **legacy archival relay (the old "BGS")**. As of **Sync v1.1
   (2025)** the canonical relay is **non-archival**: per Bluesky, the change "eliminat[es] the need
   to crawl or store user data," keeping only a **configurable backfill window** (a 24h window ≈ a
   couple hundred GB of disk), not full history. Full-history backfill now pulls from the **PDS**
   (`com.atproto.sync.getRepo`) or via a sync tool (see Tap, below), not from the relay's cache.
   **Internal contradiction:** the transcript's own "2 vCPU / 12 GB RAM" relay figure (item 4) is
   *cheap precisely because* relays stopped storing full repos — it can't coexist with "the relay
   keeps every repo." Treat the "relay = full network backup" framing as superseded. (src:
   docs.bsky.app/blog/relay-sync-updates; bnewbold WhiteWind "Full-Network Relay for $34 a Month")

## CONFIRMED-despite-suspicion — do NOT discard these

3. **"A single ~2 vCPU / 12 GB RAM server can process the entire global firehose"** — **CONFIRMED,
   and current.** This is the **post-Sync-v1.1** relay: Bryan Newbold documents 2 vCPU / 12 GB RAM /
   ~30 Mbps, a full-network relay for ~$20–34/month, runnable on a Raspberry Pi. (Note: this is the
   "validating, non-archival" relay — see item 2 for why it's that cheap.) Cross-links ROADMAP_TODO
   B5 (relay capacity). (src: bnewbold WhiteWind; docs.bsky.app/blog/relay-sync-updates)
4. **"Tap"** as an open-source backfill/repo-sync tool — **CONFIRMED, official.** "Introducing Tap"
   (atproto.com / docs.bsky.app): a single-tenant **Go** service that subscribes to a Relay, does
   **automatic backfill** (fetches a repo's full history via `getRepo`, marks events `live:false`,
   then switches to live), SQLite/Postgres backed. The transcript's description is accurate.
   (src: atproto.com/blog/introducing-tap; docs.bsky.app/docs/advanced-guides/backfill)
5. **"Awesome ATProto" at `github.com/atblueprints/awesome-atproto`** — **PARTLY.** That repo *does*
   exist, but the active canonical list is **`github.com/awesome-atproto/awesome-atproto`**;
   `beeman/awesome-atproto` was **archived July 2025**. So the link is real but not the primary one.

## CONFIRMED — settled atproto mechanics (cite source of truth, not re-derived)

| Claim | Verdict |
|---|---|
| AppView = relational index over the firehose; PDS → Relay → AppView → client three-layer split; client makes one API call to the AppView | CONFIRMED |
| **Lexicon = schema** (like OpenAPI/JSON-Schema); `app.bsky.feed.post` shape; custom NSIDs propagate without registration | CONFIRMED (matches prior FACTCHECK: open-union embeds, official AppView ignores unknown `app.bsky.*`-only) |
| Atproto repo = **Merkle Search Tree**, Git-like, self-authenticating, CID content hashes, DAG-CBOR | **CONFIRMED** (MST is correct *for atproto* — the MST error only ever applied to iroh-docs) |
| **did:web** = `/.well-known/did.json`, DNS-based, vulnerable to domain expiry/seizure; **did:plc** = signed append-only op log at `plc.directory`, portable, currently Bluesky-run → transitioning toward independent governance | CONFIRMED (mechanics; name expansion REFUTED — item 1) |
| Long-form rides the same identity/repo (Standard.site / WhiteWind / Leaflet); cross-app interop via shared DID; platform-independence via the PDS-held record | CONFIRMED (Standard.site real per prior FACTCHECK — though "not Markdown-primary"; `site.standard.document` NSID plausible, not separately re-verified) |
| Relays are **lexicon-agnostic** — verify signature + MST fold + bundle DAG-CBOR + rebroadcast; **Open Unions** let custom data ride standard posts; official AppView shows a fallback | CONFIRMED (matches prior FACTCHECK) |
| PDS↔Relay = persistent WebSocket; relay subscribes to **`com.atproto.sync.subscribeRepos`**; split frame = JSON header (`#commit`, did, seq) + binary **CAR** payload (DAG-CBOR blocks) | CONFIRMED |
| Dedup/order via **`rev`** (a monotonic **TID** Timestamp-ID, lexicographically sortable) + relay **`seq`**; content-addressing makes data self-deduplicating | CONFIRMED |
| **`requestCrawl`** (unauthenticated POST) for new-PDS discovery; passive identity-registry tracing (`#identity` events) to find migrated/new PDSs | CONFIRMED |
| Firehose subscribers are **anonymous** — no relay-side registry of connected AppViews; discover *servers* (not listeners) via in-repo `app.bsky.feed.generator` / `com.atproto.labeler.service` records, client "Custom Service" toggles, and community lists | CONFIRMED |
| Feed/labeler registrations are published **once** (a record, mutated in place on edit) | CONFIRMED |
| Stale-endpoint handling: did:plc migration emits an **`#identity`** firehose event → instant cache invalidation; did:web relies on **TTL expiry + signature-error-driven refresh** | CONFIRMED |
| **Feed Generator returns a skeleton** (URIs) → **AppView hydrates** (text, avatars, counts, blocks). Generator stays cheap because the AppView carries the DB burden | CONFIRMED (this is the `getFeedSkeleton` model) |
| Scale safeguards: per-PDS/per-DID rate limiting; expensive work pushed to AppViews; multiple independent relays possible | CONFIRMED |

## Distillation note

Low new-design yield — this is a teaching restatement of settled mechanics. The two genuinely
useful updates for the corpus: **(a)** the **Sync-v1.1 non-archival relay** + its **~$34/mo,
2 vCPU/12 GB, Raspberry-Pi-capable** economics (relevant to the operator-relay tier in
`open-considerations.md` §8 / ROADMAP_TODO E20 and the relay-capacity item B5), and **(b)** **Tap**
as the official repo-sync/backfill tool (ECOSYSTEM atproto tooling). A dated addendum recording
both — plus the did:plc-name correction — is appended to the source-of-truth
`atproto-atmospheric-web-iroh-mobile-FACTCHECK.md`.
