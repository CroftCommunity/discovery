# Fact-check — AT Proto / PDS / Germ / private-data dialogue

date: 2026-06-22 · companion to `croft-atproto-pds-germ-privatedata-dialogue-2026-06-22.md`

purpose: the raw dialogue is AI-generated (Gemini, flagged by the user as sometimes unreliable). Per
the standing intake discipline, every substantive assertion was fact-checked against web primary
sources (atproto.com specs, github.com/bluesky-social/atproto discussions, bsky.social blog +
docs.bsky.app, germnetwork.com, IETF datatracker, Paul Frazee's leaflet.pub, Dan Abramov / overreacted.io,
provider docs, tech press). Verdicts: **CONFIRMED** · **PARTLY** (real but mis-described) · **REFUTED**
(false / no such thing) · **UNVERIFIABLE**. Verified 2026-06-22 via four parallel research passes.

This file also **updates the standing source of truth** `atproto-atmospheric-web-iroh-mobile-FACTCHECK.md`
(a dated addendum was added there): a **real community-led ATProto Private Data Working Group now
exists**, which refines — but does not contradict — that file's "no native AT-Proto E2EE / working group
(REFUTED)" note. The refuted thing was a *fictional* "AT Messaging / MLS-standardizing" body; the real WG
is community-coordinated, about access-controlled (PDS-gated) private data, with E2EE a later separate
consideration. Native-in-the-*protocol* E2EE still does not exist; Germ remains third-party E2EE riding
atproto identity.

## Headline

The most grounded Gemini intake yet on the technical side — the PDS/Relay/AppView model, the MST repo
structure, CID format, the public-by-default reality, the private-data design discussions (#3363 / #121),
and the Germ architecture are **accurate and primary-source-confirmed**. The federation-limit numbers
(100 accounts, 2,600/hr with 50/s burst, 21,000/day) and the May-2024 DM date are **exactly right**.
Gemini's residual failure mode is the familiar one: a few **invented/garbled proper nouns and
identifiers** (`ger.mx`, `distributed-mls-id`, the `/android-waitlist` URL), one **over-claimed
institutional framing** (the WG as "officially formed" — it's community-led), one **false marketplace
claim** (Vultr 1-Click PDS app), one **missed real entity** (peers.org), and **volatile commercial
pricing** that should not be enshrined.

## Errors not to carry forward

1. **"ger.mx" as a Germ routing endpoint — UNVERIFIABLE / likely invented.** No source names this
   domain. The *substance* (no standalone Germ federation server today; Germ Inc. operates the routing
   backend) is confirmed; the specific domain is not.
2. **IETF draft "distributed-mls-id" — REFUTED name.** The effort is real ("Distributed MLS," authored
   by Germ's Mark Xue, presented at IETF 124), but the draft is **`draft-xue-distributed-mls`**, not
   "distributed-mls-id." (Don't conflate with the separate, inactive `draft-kohbrok-mls-dmls`.)
3. **"germnetwork.com/android-waitlist" — UNVERIFIABLE URL.** iOS-only + Android-on-roadmap is real; the
   exact path is not confirmed — don't assert it.
4. **"ATProto Private Data Working Group" as *officially formed* by Bluesky — PARTLY.** A real WG exists
   but it is **community-led** (atproto.wiki + discourse.atprotocol.community, organized by Boris Mann);
   Paul Frazee participates **informally** and publicly hedged. Do not imply Bluesky chartered it.
5. **Vultr "1-Click Marketplace PDS app" — REFUTED.** No PDS app on the Vultr Marketplace. Vultr is a
   *supported VPS target* for the official installer (and `pdsadmin` works there), but the genuine
   1-Click Marketplace PDS app is **DigitalOcean's**. (An AWS Marketplace PDS listing also exists.)
6. **peers.org "no major org" — REFUTED (Gemini miss).** A notable **Peers** (peers.org, launched 2013,
   led by **Natalie Foster**) was a sharing-economy / gig-worker advocacy org (~250k members; 2014 "Keep
   Driving"), since wound down / merged into the **Indy Worker Guild**; Foster co-founded the **Economic
   Security Project**. (Mildly relevant to Croft's cooperative/sharing-economy lineage.)
7. **Commercial pricing (PDS hosting + object storage) — services real, $ figures volatile/point-in-time.**
   Don't enshrine. Weakest: **ElfHosted ~$9/mo** (store shows a $1 / 7-day intro trial).

## Cluster 1 — Bluesky federation limits + DMs

| Claim | Verdict | Note (src) |
|---|---|---|
| Early-access (2024) required domain registration via manual Discord allowlist | CONFIRMED | docs.bsky.app/blog/self-host-federation |
| Early limits: 10 accts/server, 1,500/hr, 10,000/day | CONFIRMED | exact match (same blog) |
| Limits no longer current; open federation live (auto plug-in, no pre-registration) | CONFIRMED | Discord requirement removed ~Apr/May 2024 (TechCrunch 2024-02-22) |
| Current bsky.network defaults: 100 accts/PDS; ~2,600/**hr** (50/s burst); ~21,000/**day**; raised on request | CONFIRMED | exact match (docs.bsky.app/docs/advanced-guides/rate-limits) — **attach the per-hour unit** |
| DMs exist; 1:1 text first rolled out May 2024 | CONFIRMED | shipped 2024-05-22 (bsky.social blog) |
| Settings Everyone / People I Follow / No One; default "People I Follow" | CONFIRMED | (same blog) |
| Send text, share posts into chats, use emojis | PARTLY | true mid-2026 but **not at May-2024 launch** (text-only then; reactions v1.100 Apr 2025, post-sharing v1.103) |
| Group DMs rolling out | PARTLY | confirmed-but-unreleased mid-2026 — "in development" accurate, "rolling out" overstates. (NB: a concurrent corpus row, ECOSYSTEM/§25, records Bluesky native group chats "launched 2026-06-11, up to 50" — reconcile the exact launch state at bundle time.) |
| Native DMs NOT E2EE; moderation can access to investigate reports | CONFIRMED | (same blog) |

## Cluster 2 — Germ Network

| Claim | Verdict | Note (src) |
|---|---|---|
| Bluesky partnered with privacy startup "Germ Network" for encrypted DMs | CONFIRMED | first private messenger launched from Bluesky's app (TechCrunch 2026-02-18) |
| Standalone app; team incl. ex-Apple privacy engineer (iMessage/FaceTime) | CONFIRMED | cofounder/CTO **Mark Xue** (Apple FaceTime/iMessage privacy); cofounder Tessa Brown (TechCrunch 2025-07-30) |
| No phone/email; identities "cards" incl. "burner cards" | CONFIRMED | germnetwork.com |
| Uses MLS, RFC 9420 | PARTLY | MLS-as-IETF-standard correct; RFC 9420 is genuinely the MLS RFC but not cited in Germ's own materials (they reference RFC 9750, MLS Architecture) — accurate, not Germ-sourced |
| Won "Cypherpunk Fellowship from Protocol Labs" extending MLS | CONFIRMED | Germ blog announces the win (Protocol Labs / Web3 Privacy Now Cypherpunk Camp) |
| Natively integrates atproto; links chat keys to handle/DID without seeing password | CONFIRMED | germnetwork.com/blog/integrating-germ-atproto |
| Germ button/badge on profile; one tap launches encrypted chat; Blacksky integrated same | CONFIRMED | badge opens an iOS App Clip; Blacksky added it (TechCrunch 2026-02-18) |
| Open-source "Autonomous Communicator Protocol" on MLS managing relationships/cards | CONFIRMED | "Autonomous Communicator (AC) Protocol," MIT-licensed Swift impl (germ-network/autonomous-comm-protocol) |
| iOS-only; Android on roadmap; waitlist at germnetwork.com/android-waitlist | PARTLY | iOS-only + roadmap confirmed; **exact URL unverified** (error #3) |
| No standalone Germ server; Germ Inc. runs routing; endpoints like "ger.mx" | PARTLY | no-self-host + Germ-runs-backend confirmed; **"ger.mx" unverifiable** (error #1) |
| Binds E2EE "Anchor Key" to DID; tracks changes via PDS public profile | CONFIRMED | "Anchor Key" real (UI: "pairing key") — publishes current Anchor Key in atproto profile text |
| Supports external "message mailbox services" | CONFIRMED | ephemeral/rendezvous mailboxes; exploring multiple for resiliency |
| Drafting IETF "Distributed MLS" (draft "distributed-mls-id") | PARTLY | effort real (`draft-xue-distributed-mls`, IETF 124, coauthored w/ Naval Postgraduate School; impl "TwoMLS"); **draft name wrong** (error #2) |
| Works across Blacksky and "Skylight" | CONFIRMED | targets the atproto "Atmosphere"; Blacksky already integrated |

## Cluster 3 — atproto private/non-public data + architecture (the consequential cluster)

| Claim | Verdict | Note (src) |
|---|---|---|
| atproto.com "Non-Public Data" section; planned; "recommend against bolting on encryption" | CONFIRMED | exact quotes verified (atproto.com/specs/atp) |
| GitHub #3363 "Private, non-shared data in repo?" — private "Namespaces" | CONFIRMED | title exact; @bnewbold describes private namespaces |
| Private namespaces: no MST, no firehose, plain DB requiring auth/ACLs | CONFIRMED | direct quote in #3363 ("don't have MSTs… just stored in a boring database… request authentication+authorization") |
| GitHub #121 "Encryption for private content" — group-private/E2EE, DEK rotation, DID key exchange, MLS/Signal | CONFIRMED | title exact; per-post DEK, DEK-to-recipient-keys, MLS/Matrix/Signal candidates |
| An *officially-formed* "ATProto Private Data Working Group" exists | **PARTLY** | real but **community-led** (atproto.wiki, discourse.atprotocol.community, Boris Mann), **not** an official Bluesky body (error #4) |
| Core devs incl. Paul Frazee; split personal-private (bookmarks/drafts/prefs) vs shared-private (accounts/circles/groups) | CONFIRMED | Frazee participates informally; his leaflet makes exactly this split (pfrazee.leaflet.pub) |
| Leaning "Private Namespace": bypass MST, scoped OAuth tokens → authenticated replication stream from separate DB partition | CONFIRMED | matches #3363 + Frazee leaflet. *Latest vocab has shifted to "buckets"/"realms"; "namespaces" is the earlier framing* |
| Key-revocation debate: per-post DEK vs rotated master key; retroactive re-encryption ("cat out of the bag") | CONFIRMED | exactly in #121 ("circle" is our framing, not source wording) |
| atproto repos use a Merkle Search Tree (MST) | CONFIRMED | repository spec: content-addressed MST, SHA-256, fanout 16 (Auvolat/Taïani 2019). *Aligns with the standing source-of-truth: MSTs are atproto's structure (the prior iroh conflation was the error)* |
| PDS / Relay / AppView three-component model (your data / firehose aggregator / indexer-app-logic) | CONFIRMED | atproto.com/guides/glossary (the "timelines/search/counts" phrasing is fair paraphrase) |
| Dan Abramov "Open Social — The protocol is the API" + "A Social Filesystem" | CONFIRMED | overreacted.io ("the protocol is the API" is a thesis line within Open Social, not a subtitle) |
| Official doc "AT Protocol for distributed systems engineers" | CONFIRMED | atproto.com/articles/atproto-for-distsys-engineers |
| Blobs content-addressed by CID = SHA-256 multihash, CIDv1, base32, "bafkrei…" | CONFIRMED | atproto.com/specs/blob (raw codec 0x55, sha-256, `b` prefix) |
| **PDS-as-selective-file-proxy shims** (streaming pipeline; CopyObject + SQLite-row injection; reverse-proxy getBlob interceptor) | PARTLY / UNVERIFIED | the *constraints* are real (CID content-addressing; signed repo record required; getBlob is the real sync endpoint; official PDS uses SQLite). The specific **SQLite schema** (`blob`/`repo_blob`/`record` tables, columns) and the injection/proxy recipes are **plausible but not spec-guaranteed and explicitly fragile** (Gemini says so). Treat as an *idea to prototype*, not a verified recipe — see ROADMAP_TODO |

## Cluster 4 — PDS hosting, alternative implementations, object storage, peers.org

| Claim | Verdict | Note (src) |
|---|---|---|
| ElfHosted fully-managed Bluesky PDS (~$9/mo) | PARTLY | service real; price weak (store shows $1 / 7-day trial) — volatile |
| Hostinger "Bluesky PDS VPS" template (~$6.49/mo) | CONFIRMED | official one-click Docker template; price volatile |
| DigitalOcean official Bluesky PDS 1-Click Marketplace app (~$4-6/mo) | CONFIRMED | slug `blueskysocialpds`, bundles Caddy; price volatile |
| Vultr 1-Click Marketplace PDS app (~$5/mo, pdsadmin) | **REFUTED** | no marketplace app; Vultr is a *supported installer VPS target* only (error #5) |
| Self-host needs own domain (A + wildcard CNAME) + SMTP provider | CONFIRMED | official pds README (wildcard for open account creation) + email service |
| atproto decouples identity from host; migrate repo/account between PDSes without losing followers | CONFIRMED | atproto.com/guides/account-migration (CAR export/import preserves follow graph) |
| Official `@atproto/pds` is TypeScript, single-tenant SQLite (primary + per-user .sqlite), local FS, no shared external Postgres/MySQL | CONFIRMED | ActorStore = per-user SQLite (WAL) + PDS-wide SQLite; SQLite-only |
| "Cocoon" (Go) and "rsky-pds" (Rust) exist, support PostgreSQL | CONFIRMED | Cocoon (haileyok/cocoon, optional Postgres, "highly experimental"); rsky-pds (blacksky-algorithms/rsky, Rudy Fraser, Postgres + S3 + Mailgun) — *Blacksky is already in this corpus* |
| Production PDS can offload blobs to S3-compatible storage (S3, R2, MinIO) | CONFIRMED | atproto.com/guides/going-to-production (⚠️ MinIO community edition repo archived Feb 2026 — Garage/SeaweedFS are maintained alternatives) |
| Object storage providers/tiers (iDrive e2, Hetzner, Backblaze B2 + Cloudflare Bandwidth Alliance free-egress, Wasabi flat + 90-day min, R2 zero-egress; cold tiers Glacier Deep/Instant, Azure Archive, GCS Archive, Intelligent-Tiering) | CONFIRMED (prices volatile) | providers/tiers/mechanics all real & in-range; exact $/TB point-in-time — do not assert as current |
| peers.org "no major org" | **REFUTED** | Gemini missed the real sharing-economy Peers.org (error #6) |

## Why this matters for Croft design

- **The real private-data WG sharpens — not erases — Croft's differentiation.** atproto's
  community-led Private Data Working Group is converging on **access-controlled, PDS-gated** private
  records (namespaces → buckets/realms): the PDS is treated as a *trusted agent* (like a browser), and
  **E2EE / true zero-knowledge is explicitly the harder, deferred problem**, behind maturing
  public-broadcast and Auth-Scopes work. That is precisely the seam Croft's **lineage-groups MLS proof**
  already occupies: *real E2EE group state that does not trust the host*, on our own terms. So the corpus
  headline "no native AT-Proto E2EE; real E2EE is third-party (Germ/MLS)" stays true and is now
  **better-evidenced**, and Croft's "MLS-on-our-terms, host-untrusted" answer is *more* differentiated,
  not less. The trusted-PDS-vs-zero-knowledge debate maps directly onto Croft's "blind broker /
  content-blind mule" stance — Croft sits on the zero-knowledge side the atproto core team is reluctant
  to take. (Link in COHESION §26.)
- **Germ is the closest atproto+MLS cousin and has matured.** It graduated from beta to *the first
  native-launched private messenger from a Bluesky profile* (2026-02-18), with confirmed open-source MLS
  work (AC Protocol, `draft-xue-distributed-mls`/TwoMLS, Cypherpunk Fellowship) and the **Anchor-Key-in-
  profile** identity-binding pattern — a concrete, shipped instance of "use atproto identity, do E2EE
  off-repo." Update its ECOSYSTEM row. The **Anchor-Key-published-in-profile** trick is itself worth
  noting alongside Croft's cross-platform-identity-provenance work (publishing a key/anchor in a public
  profile field).
- **The PDS-as-selective-file-proxy idea is genuinely interesting for the blind-broker line.** "Serve a
  blob the network believes is PDS-native while the bytes live in your own object store, zero
  duplication" rhymes with Croft's *content-blind mule* and the `encrypted-blob-share` experiment. But
  it's an unverified, self-described-fragile recipe (depends on the PDS's internal SQLite schema). Treat
  as an [explore] prototype, not a recipe — flagged in ROADMAP_TODO.
- **Provenance/term hygiene for any published copy:** `draft-xue-distributed-mls` (not "distributed-mls-id");
  don't assert `ger.mx` or `/android-waitlist`; the private-data WG is **community-led**; the genuine
  1-Click PDS host is **DigitalOcean** (not Vultr); don't enshrine the pricing tables; and peers.org *was*
  a real (now-wound-down) sharing-economy org.

## Provenance

Web verification 2026-06-22 via four parallel research passes (federation/DMs · Germ · atproto
private-data + architecture · PDS hosting/storage + peers.org); source URLs inline in the cluster
tables. Standing source of truth for atproto/iroh/iOS:
`atproto-atmospheric-web-iroh-mobile-FACTCHECK.md` (a dated addendum noting the real private-data WG +
Germ-now-native was added there).
