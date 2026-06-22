# Raw transcript — atproto / Bluesky PDS architecture, Germ, object-storage shims, private data (Gemini)

source: design-exploration dialogue, pasted 2026-06-22 · **cleaned-paste, content-faithful —
NOT a byte-pristine export** (PLAYBOOK §4). Body 4 of a longer continuous Gemini session; companion
bodies: `iroh-quic-localfirst-ecosystem-dialogue-2026-06-22.md`,
`opensocial-nostr-farcaster-aggregators-dialogue-2026-06-22.md`,
`cooperative-social-union-governance-dialogue-2026-06-22.md`.

> **Provenance & verification status (READ FIRST).** AI-assistant (**Gemini**) dialogue; user flags
> Gemini as sometimes unreliable. Fact-checked 2026-06-22 against atproto.com, docs.bsky.app, the
> bluesky-social GitHub, and provider pages — see
> **`atproto-pds-architecture-technical-dialogue-2026-06-22-FACTCHECK.md`**. The architecture
> (PDS/Relay/AppView, MST repo, CID blobs, Lexicons, the endpoint names, the Private Data Working
> Group, Germ) is **accurate**. **Confirmed-WRONG, do NOT rely on:** the *current* federation limit
> numbers ("100 accounts / 2,600 events-hr / 50/s burst / 21,000-day") are **not in the spec or PDS
> config and appear fabricated** — only the **Feb-2024 early-access** numbers (10 accounts / 1,500
> events-hr / 10,000-day) are real, and open federation (no pre-registration) is live. Minor: iDrive
> e2 ~$5/TB (not $4), Hetzner ~$6.59 (not $5.99); Germ "Android waitlist" + "ger.mx" routing
> unverified; "Namespaces" terminology not verbatim-confirmed.
>
> For atproto / iroh / iOS facts cite the project source of truth
> (`atproto-atmospheric-web-iroh-mobile-FACTCHECK.md`) — note iroh-docs = range-based set
> reconciliation + LWW, **not** Merkle Search Trees (atproto repos DO use MSTs; do not cross-wire the
> two). Distilled (CONFIRMED-only) into `thinking/app/`. Some PDS-shim material (object storage,
> hosting economics) overlaps the in-flight `crypto-wars-to-p2p-pds-economics-dialogue-2026-06-22.md`
> — cross-reference, don't duplicate. **Do not copy code/Caddyfile/SQL snippets as working code.** UI
> render chrome stripped per §4.

---

**[user]** What happened to peers.org

**[assistant]** There's no major widely-known organization associated with "peers.org" that recently
shut down — you may be misremembering the URL. Possible matches: PSRS/PEERS (psrs-peers.org — the
Public School and Education Employee Retirement Systems of Missouri, very active); PEERS
(peersnet.org — "Peers Envisioning and Engaging in Recovery Services," a community mental-health peer-
support org in Alameda County, CA). [Note: an earlier "Peers" was a sharing-economy worker-advocacy
nonprofit ~2013–2016; possibly the intended reference — verify.]

**[user]** Still true? [early Bluesky federation limits: domain registration required, 10 accounts/
server, 1,500 events/hour, 10,000 events/day]

**[assistant]** Those constraints are no longer true. The limits were the early-access phase (early
2024) to keep infrastructure stable during testing. Since then Bluesky transitioned to open
federation and scaled the defaults. [FACTCHECK — IMPORTANT: the "current" numbers below appear
FABRICATED; they are not in the atproto spec or PDS config. Only the Feb-2024 early-access numbers are
real. Open federation without pre-registration IS confirmed.]

| Rule / Limit | Early Access (Feb 2024) | Current (per Gemini — UNVERIFIED) |
|---|---|---|
| Domain registration | Required via manual Discord allowlist | Removed; open federation live |
| Accounts per server | Max 10 | Max 100 per PDS by default |
| Events per hour | Max 1,500 | Max 2,600 repo stream events (50/s burst) |
| Events per day | Max 10,000 | Max 21,000 repo stream events |

These caps are applied by the central Bluesky Relay (bsky.network) to catch spam; legitimate self-
hosted PDSes outgrowing them can contact the Bluesky infra team to scale up.

**[user]** Does bluesky have dms?

**[assistant]** Yes — direct messaging rolled out for 1:1 text in May 2024. Privacy settings (default
"People I Follow"): Everyone, People I Follow, No One. Features: text, share posts into chats, emojis;
group DMs rolling out; native E2EE is a long-term protocol-roadmap goal, and Bluesky supports
integration with privacy apps (Germ) for secure encrypted chats from a profile. Access via the
envelope/chat icon.

**[user]** Explain this and germ network

**[assistant]** Bluesky's native DMs lack E2EE (the moderation team can technically access messages
to investigate reports). To bridge the gap, Bluesky partnered with **Germ Network** — a standalone
privacy messaging app by privacy veterans (including a former Apple privacy engineer who worked on
iMessage/FaceTime). Like Signal but for the decentralized-social era: no phone numbers/emails
(cryptographic identity "cards," incl. temporary "burner cards"); next-gen encryption via MLS
(Messaging Layer Security, IETF). Integration (because Bluesky is on the open AT Protocol): download
Germ (iOS), authenticate with your Bluesky handle; Bluesky places a Germ badge on your public profile;
one tap launches a secure encrypted Germ chat. Other atproto apps (Blacksky) integrated the same
button, so your secure inbox isn't locked to one app.

**[user]** Is germ based on open standards?

**[assistant]** Yes. (1) Messaging Layer Security (MLS, RFC 9420) — an IETF standard; unlike Signal's
Double Ratchet (1:1-focused), MLS handles secure group messaging efficiently across decentralized
platforms; Germ won a Cypherpunk Fellowship from Protocol Labs for open-source work extending MLS for
decentralized networks. (2) the AT Protocol (atproto) — cryptographically links chat keys to your
public handle without seeing your password; works on Bluesky/Blacksky/Skylight. (3) an open-source
"Autonomous Communicator Protocol" — on top of MLS, manages relationships + cryptographic identity
cards in a decentralized environment, letting devs build compatible clients. [FACTCHECK: Germ, MLS,
the Protocol Labs Cypherpunk Fellowship, the Autonomous Communicator Protocol, the IETF
`draft-xue-distributed-mls` draft, and Blacksky integration all CONFIRMED; the engineer is Mark Xue
(ex-Apple FaceTime/iMessage).]

**[user]** Is it available on Android?

**[assistant]** Not yet — iOS-exclusive, with an Android version on the roadmap; a waitlist at
germnetwork.com/android-waitlist. [FACTCHECK: iOS-only CONFIRMED; the specific Android-waitlist URL
UNVERIFIED.]

**[user]** Could I set up a germ federation server?

**[assistant]** No standalone Germ federation server right now — unlike Bluesky (self-hostable PDS),
Germ's core routing backend is run as managed infrastructure by Germ Inc. (endpoints like ger.mx)
[FACTCHECK: ger.mx UNVERIFIED]. But: it leverages your existing AT Protocol PDS (binds an E2EE "Anchor
Key" to your DID, tracks changes via your server's public profile); supports external "message mailbox
services" (groundwork for decoupling client from a single server); and the team is drafting an IETF
standard "Distributed MLS" (distributed-mls-id) aiming to make decentralized secure group messaging an
internet standard, eventually enabling independent nodes. Best move for ownership: self-host your
Bluesky PDS — Germ respects it as the authoritative source for your identity keys.

**[user]** Could I make my own messaging DM app and use it from bluesky?

**[assistant]** Yes — the AT Protocol lets you build apps sharing the same identity layer (XMTP and
Germ blueprint this). The 3-step architecture: (1) Bootstrap identity with DIDs/handles — no
registration form; log in with a Bluesky handle / did:plc via App Passwords or OAuth. (2) Tell the
protocol about your app — write a custom record into the user's public repo via
`com.atproto.repo.putRecord` (your app's routing/inbox ID, a locally-generated public key, a signature
authorizing the pairing). (3) Decentralized discovery — to message User B, look up B's public repo,
find your app's record, extract routing IDs + encryption keys, initialize a secure tunnel without a
central directory. Hooking into the Bluesky UI: deep-linking/bios (a deep-link scheme like
`myapp://chat/did:plc...` in a bio — open to any developer today); native profile buttons (like Germ's
— requires Bluesky-team coordination or building your own client). You can also fork an open-source
Bluesky client and hardcode your messaging protocol.

**[user]** Are there PDS for rent services?

**[assistant]** Yes. (1) Fully managed (zero maintenance): ElfHosted — managed Bluesky PDS ~$9.00/mo
(provisions server, HTTPS, repo, auto-updates; point your domain). (2) One-click VPS deployments:
Hostinger — "Bluesky PDS VPS" template from ~$6.50/mo (Linux + Docker + official PDS image); 
DigitalOcean — official "BlueSky Social PDS" 1-Click App, droplet ~$4–$6/mo; (added in follow-up)
Vultr — 1-Click Marketplace app ~$5/mo. [FACTCHECK: ElfHosted $9/mo and the DigitalOcean 1-click PDS
CONFIRMED; Hostinger and Vultr specific PDS templates UNVERIFIED.]

**[user]** More?

**[assistant]** [Comparison] ElfHosted (fully managed, $9/mo, no terminal); Hostinger (one-click
template, $6.49/mo, AMD EPYC, 50 GB NVMe, Docker dashboard, free domain year 1); DigitalOcean (1-click
marketplace, $4–$6/mo, DNS + SSH); Vultr (1-click, $5/mo, NVMe, pdsadmin tools, manage firewall ports
80/443). Hidden prerequisites: your own domain (DNS A record + wildcard CNAME so the network can locate
your server); an SMTP/transactional email provider (verification emails / invite codes — Resend,
Mailgun, Gmail SMTP). The ultimate benefit: atproto decouples identity from host — export your repo and
migrate (profile, posts, social graph) to any PDS without losing followers.

**[user]** Would it be possible to run a PDS on top of a data store used for other things?

**[assistant]** Depends on the layer. (1) Relational database layer (account metadata + text posts):
the official @atproto/pds (TypeScript) uses single-tenant SQLite — a primary config DB + separate
per-user .sqlite repo files on disk; it expects direct filesystem access, so you can't natively point
it to a shared MySQL/PostgreSQL cluster. Alternative impls support PostgreSQL backends — Cocoon (Go),
rsky-pds (Rust) — letting you share a Postgres cluster with a dedicated schema/prefix. (2) Blob storage
layer (media): a production PDS offloads to S3-compatible storage (AWS S3 / Cloudflare R2 / MinIO).
This is multiplex-friendly — share one bucket with other apps using a unique prefix (e.g.
`s3://bucket/bluesky-pds/`). Summary: account metadata = local SQLite (no external engine) on official
PDS; blobs = S3-compatible (shareable); alternative Go/Rust impls can share both. [FACTCHECK: Cocoon
(haileyok) + rsky-pds (blacksky-algorithms) CONFIRMED.]

**[user]** I meant use files that are already in an online data store and just serve some of them up
via pds

**[assistant]** Short answer: no, you can't simply "point" a PDS at an existing folder and have it
serve files — atproto uses content-addressing + cryptographic signatures, so a PDS won't act as a
passive file proxy. Why: content identifiers (CIDs) name a file by a cryptographic hash of its exact
bytes; for a file to exist on the network a text record pointing to that CID must be in your repo,
signed by your key. Two ways to achieve the result: Path A — the automation pipeline (best for regular
posts/images): a script bridges your store and PDS — download/stream the file, hit
`com.atproto.repo.uploadBlob`; the PDS computes the CID, stores it, returns a token; publish a post
referencing it. Path B — a custom Feed Generator / AppView (true proxy route): a lightweight service
queries your existing store, formats files into a feed, streams metadata to clients; files stay where
they are. Shared S3 bucket: a self-hosted PDS can use the same bucket as other apps, separated by
folders; the PDS manages its own directory (files named by raw CID strings like `bafkrei...`).

**[user]** I just want a PDS setup that can serve a file from my regular object storage even if it
takes some shim

**[assistant]** [Two patterns.] Pattern 1 — the streaming pipeline (recommended): a lightweight script
(serverless function / microservice) opens a memory pipe from your store to the PDS API, streaming
without touching local disk:
```javascript
import { S3Client, GetObjectCommand } from "@aws-sdk/client-s3";
import { BskyAgent } from "@atproto/api";
async function serveFileToBluesky(fileKey) {
  const s3 = new S3Client({ region: "us-east-1" });
  const s3Object = await s3.send(new GetObjectCommand({
    Bucket: "your-regular-application-bucket", Key: fileKey }));
  const fileStream = s3Object.Body;
  const agent = new BskyAgent({ service: 'https://your-pds-domain.com' });
  await agent.login({ identifier: 'you.your-domain.com', password: 'your-app-password' });
  const uploadRes = await agent.uploadBlob(fileStream, { encoding: 'image/jpeg' });
  await agent.post({ text: "served from app object storage via stream shim",
    embed: { $type: 'app.bsky.embed.images',
      images: [{ alt: 'Automated upload', image: uploadRes.data.blob }] } });
}
```
Pros: spec-compliant; auto-updates DB registries. Cons: still transfers bytes store→PDS (free/instant
if same data center). Pattern 2 — cloud-copy + DB injection hack (zero byte movement, for huge files):
the official PDS keeps a local SQLite DB tracking valid files; a post referencing an unregistered file
is rejected. So: calculate the CID (SHA-256 raw multihash, CIDv1 base32, `bafkrei...`); do an internal
S3 CopyObject mapping your path to the PDS path (no internet download); inject a row into the PDS
SQLite blob table; publish via putRecord. Fragile — a PDS schema change breaks the injection script.

**[user]** I don't want to store the file twice

**[assistant]** Bypass the PDS storage layer with a Reverse Proxy Interceptor Shim. When the network/
client requests a blob (`GET /xrpc/com.atproto.sync.getBlob?cid=bafkrei...`), your reverse proxy
(Caddy/Nginx/Cloudflare Worker) intercepts, maps the CID to your regular object-storage path, and
streams the file from its original home.
```
[User/Relay Request] --> [Reverse Proxy (Caddy/Nginx)]
                                |
         +----------------------+----------------------+
         v (regular PDS request)                       v (getBlob + matching CID)
   [Forward to PDS]                            [Fetch directly from your regular S3]
```
Step 1 — tell the PDS the file exists (SQL injection into pds.sqlite):
```sql
INSERT INTO blob (cid, mimeType, size, createdAt)
VALUES ('bafkreih...', 'image/jpeg', 1048576, '2026-06-07T00:00:00.000Z');
INSERT INTO repo_blob (did, cid) VALUES ('did:plc:your_user_did', 'bafkreih...');
```
Step 2 — maintain a local CID→path mapping (KV store / text file / DB table). Step 3 — configure the
Caddy shim to catch getBlob for your CIDs and route to your storage:
```
@custom_blob { path /xrpc/com.atproto.sync.getBlob
               query cid=bafkreih... }
reverse_proxy @custom_blob s3.amazonaws.com {
    header_up Host s3.amazonaws.com
    rewrite /my-main-bucket/portfolio/artwork_102.jpg }
reverse_proxy * localhost:3000
```
Zero double-storage; no PDS fork; protocol-compliant (relays/AppViews get the expected bytes + CID
validation). [Caution: SQL injection into PDS internals is fragile across PDS upgrades; treat as
experimental.]

**[user]** Does a pds store only images and blobs or comments and text posts too?

**[assistant]** A PDS stores everything that makes up your account — not just images. Text updates,
replies, likes, follows, display-name changes are all authoritatively on your PDS, split into two
layers. (1) Repository layer (text/comments/likes/follows) — packed into a compressed cryptographic
structure (the "repo," like a local Git repo); each write is a signed record in the tree; because the
PDS must compute/sign these to federate, you can't proxy text records from an external system — they
live natively in the PDS's primary DB. (2) Blob store layer (images/video) — packing big files into
the Git-like tree would be slow, so atproto stores only a tiny CID hash in the repo record and the raw
bytes in the blob store (S3/local). So: text/comments always live in the PDS DB; the reverse-proxy
shim remains the right tool for serving media from external storage without duplication.

**[user]** Can I access the text of posts and such directly myself in the PDS?

**[assistant]** Yes — it's your server (root access). Although the protocol packs data into a
compressed Git-like tree (Merkle Search Tree) for federation, the official PDS indexes/mirrors it into
a readable form. Method 1 — the clean API route (recommended): query the local XRPC `listRecords`:
```bash
curl "https://your-pds-domain.com/xrpc/com.atproto.repo.listRecords?repo=yourname.com&collection=app.bsky.feed.post&limit=5"
```
Returns JSON; post text is in `value.text`. Method 2 — the raw database route: SSH in; the official PDS
uses SQLite (e.g. under `/pds/`); the `record` table mirrors every post/comment/like/follow with the
`value` column as stringified JSON:
```bash
sqlite3 /pds/pds.sqlite "SELECT value FROM record WHERE collection = 'app.bsky.feed.post';"
```
Pipe/parse/back up as you like.

**[user]** What is the cheapest durable object store available?

**[assistant]** Depends on egress. Baseline (per 1 TB/mo): iDrive e2 ~$4 (free egress up to 3x stored)
[FACTCHECK: ~$5]; Hetzner Object Storage ~$5.99 (free egress up to 10 TB/mo) [FACTCHECK: ~$6.59];
Backblaze B2 ~$6 ($10/TB egress, or free via Cloudflare Bandwidth Alliance); Wasabi ~$6.99 (free egress
subject to fair use; 90-day min retention); Cloudflare R2 ~$15 (zero egress). Which wins: Backblaze B2
+ Cloudflare CDN (lowest storage with free public delivery via the Bandwidth Alliance); Cloudflare R2
(cheapest for massive traffic — zero egress despite $15/TB); iDrive e2 (rock-bottom standalone, no CDN
needed). Wasabi warning: a strict 90-day minimum retention (delete early, still billed) — poor for
high-churn data.

**[user]** Plans to store older less accessed data cheaper but still available?

**[assistant]** Use Infrequent Access / Coldline / Archive tiers (storage up to ~95% cheaper, but you
pay per-GB to read). (1) Instant-access tiers (ms delivery): Google Cloud Archive ~$1.20/TB ($0.05/GB
retrieval, 365-day min); AWS S3 Glacier Instant ~$4.10 ($0.03/GB, 90-day min); Google Coldline ~$4.00
($0.02/GB, 90-day); Azure Blob Cold ~$4.50 ($0.03/GB, 90-day). GCP Archive gives ms access to its
deepest tier (steep retrieval + 1-yr commitment). (2) Deep-archive tiers (delayed): AWS S3 Glacier Deep
Archive ~$1.01/TB (12–48 hr restore); Azure Blob Archive ~$0.99/TB (1–15 hr). (3) AWS S3 Intelligent-
Tiering (auto-moves objects between tiers by access pattern; small monitoring fee). Three cold-storage
traps: minimum-retention rules (early delete still billed); the retrieval penalty (downloading 10 TB
from Deep Archive can top $1,000 in retrieval+egress); small-file bloat (per-object metadata fees — tar/
zip millions of tiny files first). [FACTCHECK: Glacier Deep ~$1/TB CONFIRMED; Azure/Google exact figures
roughly plausible but not individually confirmed.]

**[user]** What about wasabi?

**[assistant]** Wasabi takes the opposite approach — a single "Hot" tier, flat pricing, positioned as
a hot+cold replacement. ~$6.99/TB/mo as of mid-2026 (rising to $7.99 on July 1, 2026), all regions. No
dedicated cold tier, but popular for archives: instant retrieval (ms, vs Glacier's 12–48 hr); zero
egress/retrieval/API fees (restore a 10 TB archive for $0 extra). Limitations: more expensive than true
deep archive (Glacier Deep ~$1.01, Azure Archive ~$0.99 win if egress never matters); the 90-day
minimum-retention trap (early delete = "Timed Deleted Storage" billed for the remaining days). Verdict:
use Wasabi if "cold" data is actually an active archive (cheap storage + instant fetch, no surprise
retrieval bill); skip it for true never-touched compliance/DR data (AWS Deep Archive at ~$1/TB wins).

**[user]** Where can I read about the architecture of a PDS and at proto? At proto can be used for
things other than micro[b]logging right? And I still don't quite understand app view

**[assistant]** Docs: atproto.com (Protocol Overview, "Understanding Atproto"); "Atproto for
distributed systems engineers" (DB scaling, federation bottlenecks); Dan Abramov's community essays
("Open Social — The protocol is the API," "A Social Filesystem") — atproto is more like a shared
network hard drive than a social network. Beyond microblogging: atproto is a decentralized public JSON
database framework, agnostic about data type. Bluesky just happens to be the first big app, defining
its schemas (Lexicons) under `app.bsky.*` (e.g. `app.bsky.feed.post`, `app.bsky.graph.follow`). You can
write your own Lexicon namespace: blogging (`com.longform.article`), e-commerce
(`xyz.marketplace.item`), media. Demystifying the AppView: in monolithic platforms DB + logic + UI live
together; in atproto they're torn apart:

| Component | What it is | What it knows |
|---|---|---|
| PDS | Your personal vault | Only your account, posts, local media |
| Relay | The network pipeline | Listens to every PDS, combines into one live firehose |
| AppView | The application engine | Consumes the firehose, organizes data, runs app logic (search, trends, timelines) |

Mental model: the PDS is your local hard drive (your post writes there). But your timeline needs posts
from 50 people on 40 servers, plus like-counts and search — your small PDS can't do that. The AppView
is a specialized search-engine indexer: it watches the global Relay firehose, dumps every JSON post into
a heavily-indexed cluster, calculates global metrics (likes/reposts), tracks the social graph, runs
search/algorithms. Your phone queries the AppView for the home timeline; the AppView does the heavy
calculations and streams the finished view.

**[user]** A PDS assumes all files are public bc that is its purpose right?

**[assistant]** Right — a PDS currently assumes everything you write to your repository is 100% public;
it's an open data layer. Write a post/comment/like/display-name change and the PDS signs it and
broadcasts it to the global Relay firehose; there are no ACLs or "private account" toggles in the core
repository layer. Nuance: (1) repositories vs blobs — repo records (text/links/follows) are broadcast;
media blobs are accessible if you know the CID, but CIDs are published in public posts, so anyone can
fetch your media without logging in. (2) private data is on the roadmap — the creators say non-public
mechanisms (private groups, encrypted personal storage) are planned, but they're taking time to build a
native architecture and explicitly advise against "bolting on" encryption to the public repo. (3) how
apps handle privacy today — Bluesky DMs use a separate traditional chat infra (not the public repo);
Germ bypasses the public PDS for messages, using your PDS identity only to look up public keys, then an
isolated E2EE pipeline elsewhere. As of now, data in a standard PDS repository = public.

**[user]** Where can I find this planning work? [non-public data mechanisms]

**[assistant]** Three places: (1) the official specification (atproto.com — "AT Protocol Structure /
Non-Public Data"): "Mechanisms for personal and group data sharing are planned... These will be large
additions to the protocol framework. We recommend against simply 'bolting on' encryption or private
content using the existing protocol primitives." (2) GitHub Discussion #3363 ("Private, non-shared data
in repo?") — the team outlines a proposed "Namespaces" extension: separate data namespaces per account
that don't use Merkle Search Trees and don't broadcast to the firehose, sitting in a standard DB on
your PDS requiring authentication + ACLs. (3) GitHub Discussion #121 ("Encryption for private content")
— the cryptography side: group-private content + E2EE without breaking decentralization; Data
Encryption Keys (DEKs) rotation, asymmetric key exchange between DIDs, eventually MLS or Signal; the
debate on revoking access to old data without re-encrypting/re-uploading historical media. [FACTCHECK:
the Private Data Working Group is REAL (atproto.wiki); discussions #3363 and #121 both exist; "Namespaces"
terminology not verbatim-confirmed; Paul Frazee involvement plausible.]

**[user]** Read the thread and give me current state and contention

**[assistant]** The community has shifted from abstract planning to concrete implementation, marked by
the official **ATProto Private Data Working Group (WG)**. The team (incl. core developers like Paul
Frazee) split the problem into: personal-private data (bookmarks, drafts, app preferences) and shared-
private data (private accounts, "circles," group interactions). Three core debates: (1) the "Trusted
PDS" dilemma (zero-knowledge vs trusted agent) — privacy advocates want client-side encryption so even
the PDS operator can't read private data; the core team treats the PDS like a browser (an inherently
trusted agent already holding your repo-signing keys), arguing a zero-knowledge model adds massive
client-side friction. (2) PDS complexity vs cheap self-hosting — atproto scales because public data is
broadcast-oriented (the PDS just signs + dumps to the firehose); the proposed fix is a parallel
"Private Namespace" bypassing the public MST, using scoped OAuth tokens to open an authenticated
replication stream from a separate DB partition; but adding ACLs / verification middleware / per-request
token checks threatens the requirement that a PDS stay cheap to self-host on a $4 VPS. (3) cryptographic
key revocation (the "cat out of the bag" problem) — the team is against plopping encrypted bytes onto
the public firehose (archived forever, decryptable on a future leak); two choices — a unique DEK per
post (secure, heavy overhead) vs a single master key per circle (rotate to remove a member); the debate
is whether to force retroactive re-encryption/re-upload of historical content, or accept a "cat is
already out of the bag" mindset where rotation applies only going forward.
