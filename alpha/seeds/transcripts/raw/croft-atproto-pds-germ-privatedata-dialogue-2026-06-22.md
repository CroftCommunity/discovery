# Croft — AT Proto / PDS self-hosting / Germ / private-data dialogue (Gemini, 2026-06-22)

> **Provenance / fidelity (PLAYBOOK §4).** Source: an AI-generated research dialogue with
> **Gemini**, pasted into the session by the user (no canonical export exists). This is a
> **cleaned-paste, content-faithful rendering — NOT a byte-pristine export.** Dialogue wording is
> preserved as faithfully as the paste allowed; only render chrome was stripped (Gemini's bare
> inline citation breadcrumbs — "Metricool", "Sprinklr +1", "ElfHosted", etc. — reduced to bracketed
> `[src: …]` pointers; code blocks preserved verbatim). **Gemini is flagged by the user as sometimes
> unreliable; every substantive assertion here was fact-checked** — see the companion
> `croft-atproto-pds-germ-privatedata-dialogue-2026-06-22-FACTCHECK.md` for per-claim verdicts. Do
> not carry any claim forward as fact without checking it against the FACTCHECK.
>
> **For atproto/iroh/iOS facts this corpus has a standing source of truth:**
> `atproto-atmospheric-web-iroh-mobile-FACTCHECK.md` (cite, don't re-verify). The most consequential
> finding here — that a **real, community-led ATProto Private Data Working Group exists** — refines
> that file's "no native AT-Proto E2EE / working group (REFUTED)" note; a dated addendum was added
> there. The still-true part: the refuted thing was a *fictional* "AT Messaging / MLS-standardizing"
> body; the real WG is community-coordinated and is about access-controlled (PDS-gated) private data,
> with E2EE a later separate consideration. Germ remains *third-party* E2EE riding atproto identity.

---

**USER:** What happened to peers.org?

**GEMINI:** There is no major or widely known organization associated with the domain "peers.org"
that has recently shut down. It is possible you may be misremembering the URL or confusing it with a
different entity. Depending on what you are looking for, you might be thinking of: **PSRS/PEERS**
(psrs-peers.org), the Public School and Education Employee Retirement Systems of Missouri; or
**PEERS** (peersnet.org), "Peers Envisioning and Engaging in Recovery Services," a community-based
mental-health organization in Alameda County, California. If you are referring to a different
organization or a site you believe has gone offline, more context can help identify if it has
rebranded or moved.

*(FACTCHECK: REFUTED — Gemini missed the real one. **Peers / peers.org** (launched 2013, led by
Natalie Foster) was a notable sharing-economy / gig-worker advocacy org (~250k members; the 2014
"Keep Driving" program), since wound down / merged into the Indy Worker Guild; Foster co-founded the
Economic Security Project.)*

---

**USER:** Still true? [pasted early-access Bluesky limits:] You have to register your domain with
Bluesky before your content can be relayed (federated); limited to 10 accounts per server; 1,500
events per hour; 10,000 events per day.

**GEMINI:** Those constraints are no longer true. The limits you quoted were established during the
early-access phase of federation in early 2024 to keep the infrastructure stable while testing.
Since then, Bluesky has transitioned to open federation and scaled up the default baseline limits
for self-hosted Personal Data Servers (PDS).

| Rule / Limit | Early Access (Feb 2024) | Current Status |
|---|---|---|
| Domain Registration | Required via a manual Discord allowlist | Removed. Open federation is live; new servers plug in automatically without pre-registering. |
| Accounts per Server | Max 10 accounts | Max 100 accounts per PDS by default. |
| Events per Hour | Max 1,500 events | Max 2,600 repository stream events (with a 50 events/second burst cap). |
| Events per Day | Max 10,000 events | Max 21,000 repository stream events. |

These caps are applied by default by the central Bluesky Relay (bsky.network) to catch spam bots and
prevent abuse before traffic hits the main firehose. A legitimate self-hosted PDS outgrowing the
100-account or daily-event threshold can contact the Bluesky infrastructure team to have limits
scaled up manually.

---

**USER:** Does Bluesky have DMs?

**GEMINI:** Yes. The feature first rolled out for one-to-one text chats in May 2024 and has continued
to evolve. [src: Sprinklr +1]

**Privacy and Settings.** By default chat permissions are set to "People I Follow." You can change
this to three options: **Everyone** (any user can message you); **People I Follow** (only accounts
you follow can initiate); **No One** (disables your inbox). [src: Bluesky / Metricool]

**Current Features.** Basic messaging: send text, share posts directly into chats, use emojis.
**Group Chats:** support for group DMs is rolling out. **Encryption / extended privacy:** while
native E2EE has been a long-term goal on their protocol roadmap, Bluesky also supports direct
integration with privacy-focused apps (like the Germ network) to launch secure encrypted chats
straight from a profile. Access the inbox via the envelope/chat-bubble icon. [src: Metricool / YouTube]

---

**USER:** Explain this and germ network [re: the E2EE / Germ integration].

**GEMINI:** Bluesky's native DMs are practical for quick chats but currently lack E2EE. Because
standard DMs are unencrypted, the moderation team can technically access messages to investigate
reports of spam, harassment, or abuse. [src: Metricool] To bridge this privacy gap without building
a massive E2EE infrastructure from scratch, Bluesky officially partnered with an independent,
privacy-focused startup called **Germ Network**. [src: Bluesky]

**What is Germ?** A standalone messaging app built by privacy veterans, including a former Apple
privacy engineer who worked on iMessage and FaceTime. Designed to act like Signal but built for the
decentralized social era. What makes it unique: **No phone/email required** — customizable
cryptographic identities called "cards" (incl. temporary "burner cards"). **Next-gen encryption** —
runs on Messaging Layer Security (MLS), an IETF-approved standard, so messages can't be read by
anyone (not Germ, not Bluesky). [src: App Store +1]

**How the integration works.** Because Bluesky is built on the open AT Protocol, third-party apps can
plug into your profile data with permission. You download Germ (iOS), authenticate with your Bluesky
handle; Bluesky places a Germ button/badge on your public profile; one tap launches a secure Germ
chat. Because it relies on an open protocol, other decentralized apps (like Blacksky) integrated the
same button — your secure inbox isn't locked to one app. Bluesky handles the public town square;
Germ handles the private vault. [src: Metricool / Germ Network]

---

**USER:** Is germ based on open standards?

**GEMINI:** Yes — heavily built on open standards and open-source foundations, engineered to
interface with existing open, decentralized protocols.

1. **Messaging Layer Security (MLS)** — encryption via MLS (RFC 9420), a modern IETF cryptographic
   standard. Unlike legacy E2EE protocols (Signal's Double Ratchet, designed for 1:1), MLS was built
   for secure group messaging across decentralized platforms. Germ won a **Cypherpunk Fellowship from
   Protocol Labs** for open-source contributions extending MLS for decentralized networks. [src: Germ
   Network / Global Dating Insights / Boris Mann]
2. **The AT Protocol** — Germ natively integrates with atproto, cryptographically linking your chat
   keys to your public handle/DID without ever seeing your password. Works across Bluesky, Blacksky,
   Skylight. [src: Germ Network]
3. **Open-Source "Autonomous Communicator Protocol"** — instead of phone numbers/emails, Germ uses
   its own open-source Autonomous Communicator Protocol, sitting on top of MLS to manage relationships
   and cryptographic identity "cards" in a decentralized environment, letting developers build
   compatible clients. [src: Germ Network +1]

---

**USER:** Is it available on Android?

**GEMINI:** Not yet. Germ is currently exclusive to iOS, but an Android version is on their roadmap.
If you're on Android, they have a landing page (germnetwork.com/android-waitlist) to get notified.
[src: Bluesky]

---

**USER:** Could I set up a germ federation server?

**GEMINI:** No, you cannot set up a standalone Germ federation server right now. Unlike Bluesky
(self-hosted PDS), Germ's core routing backend is still run as managed infrastructure by Germ Inc.
(routing through endpoints like ger.mx). However, because Germ is engineered around open protocols:
**It leverages your existing AT Protocol PDS** — if you host your own Bluesky server, Germ plugs in
out of the box, binding an E2EE cryptographic "Anchor Key" to your DID and tracking changes via your
server's public profile data. Your identity is federated; Germ acts as the secure pipeline. [src:
Germ Network] **Decoupled mailbox services** — the app supports external "message mailbox services"
for queues and delivery notifications, laying groundwork for separating the client from a single
centralized server, but no plug-and-play self-host server package is released. **Standardization in
progress** — the team is drafting an IETF standard called Distributed MLS (distributed-mls-id), aiming
to make decentralized secure group messaging an internet standard.

---

**USER:** Could I make my own messaging DM app and use it from bluesky?

**GEMINI:** Yes, absolutely. The AT Protocol is engineered to let you build whatever applications you
want while sharing the same underlying identity layer. [src: GitHub]

**The 3-step architecture for a custom DM app:**

1. **Bootstrap identity with DIDs and handles** — no registration form/password DB/phone numbers.
   The user logs in with their existing Bluesky handle or `did:plc:…`; authenticate via standard
   ATProto App Passwords or OAuth. [src: Germ Network +1]
2. **Tell the protocol about your app** — every account has a public repo managed by its PDS. Write a
   custom record into the public repo via `com.atproto.repo.putRecord` containing your app's routing/
   inbox ID, a locally-generated public key, and a signature proving authorization. [src: AT Protocol]
3. **Handle decentralized discovery** — because every repo is public and broadcast on the firehose,
   discovery is elegant: client looks up User B's public repo, parses it to find your app's record,
   extracts their chat routing IDs + encryption keys, and initializes the secure tunnel directly,
   without a centralized account directory.

**How it hooks into the Bluesky UI:** Deep-linking/bios (`myapp://chat/did:plc…` in a description/
website field — open to any developer today) or Native Profile Buttons (like Germ — requires your
Lexicon to be recognized/adopted by client developers, or building your own client). You can also
fork an open-source Bluesky client and hardcode your messaging protocol into the layout.

---

**USER:** Are there PDS for rent services?

**GEMINI:** Yes, multiple — from fully-managed to cheap one-click VPS.

1. **Fully managed (zero maintenance).** **ElfHosted** — a fully managed Bluesky PDS instance for
   ~$9.00/month; they provision, secure with HTTPS, manage the repo, and handle updates; you just
   point your domain. [src: ElfHosted]
2. **One-click VPS (turnkey).** **Hostinger** — a "Bluesky PDS VPS" template starting ~$6.49/month
   (Linux VM pre-configured with Docker + the official PDS image). **DigitalOcean** — an official
   BlueSky Social PDS 1-Click Marketplace app; basic droplet ~$4–6/month, auto-runs the setup script.
   [src: Hostinger / DigitalOcean Marketplace]

*(Follow-up "More?" added a comparison incl. **Vultr** "1-Click Marketplace app ~$5/mo, integrates
with pdsadmin." FACTCHECK: Vultr has **no** 1-Click PDS Marketplace app — it is a supported VPS
target for the official installer, not a marketplace app. DigitalOcean's IS a real 1-Click app.)*

**Hidden prerequisites:** your own domain name (A record to your IP + a wildcard CNAME) so the
network can locate the server; and an SMTP/transactional email provider (Resend, Mailgun, Gmail SMTP)
for verification emails / invite codes. **The ultimate benefit of renting:** atproto decouples
identity from host, so you can export your entire repo and migrate (profile, posts, social graph) to
a new server without losing a single follower.

---

**USER:** Would it be possible to run a PDS on top of a data store used for other things?

**GEMINI:** Yes, but it depends on whether you mean the relational database (account metadata, text
posts) or the blob storage (images/media), and whether you use official or third-party software.

1. **Relational DB layer.** The official reference implementation (`@atproto/pds`, TypeScript) uses a
   **single-tenant SQLite** architecture — a primary SQLite file for config plus separate isolated
   `.sqlite` files per user repo on disk. **Official PDS:** tightly coupled to local SQLite files; you
   **cannot** natively point it to a shared centralized DB cluster (corporate MySQL/PostgreSQL). It
   expects direct filesystem access. **Alternatives:** community PDS software like **Cocoon** (Go) or
   **rsky-pds** (Rust) explicitly support PostgreSQL backends — you can point them to an existing
   Postgres cluster, isolating Bluesky tables with a dedicated schema/prefix.
2. **Blob storage layer.** A production PDS offloads heavy binaries (profile pics, images, video) to
   an **S3-compatible object store** (AWS S3, Cloudflare R2, MinIO). This layer is multiplex-friendly:
   one bucket can hold media for multiple apps — just use a unique bucket folder prefix (e.g.
   `s3://my-shared-bucket/bluesky-pds/`). [src: AT Protocol]

---

**USER:** I meant use files that are already in an online data store and just serve some of them up
via pds.

**GEMINI:** The short answer is no — you cannot simply "point" a PDS at an existing folder of files
and have it serve them on demand, because of how atproto handles data security (a strict cryptographic
barrier prevents a PDS from acting as a passive file proxy). But you can build a lightweight
automation bridge.

**Why "point and serve" doesn't work natively.** Bluesky uses content-addressing and signatures.
**CIDs:** every blob is identified by a cryptographic hash of its exact binary contents; change one
pixel and the "name" changes. **The repository record:** for a file to exist on the network, a text
record pointing to that exact CID must be written into your repo and signed with your private key.
Because a PDS refuses to serve any file that hasn't been hashed, assigned a CID, and bound to a signed
record, it cannot read arbitrary files from an external drive on the fly. [src: AT Protocol]

**Two ways to make it work.** **Path A — Automation pipeline (best for regular posts):** a script
bridges your store and your PDS. `[Your Data Store] ──(script tags a file)──> [PDS API] ──>
[Bluesky Network]`. When you flag a file, the script downloads it, hits `com.atproto.repo.uploadBlob`,
and uploads the raw bytes; the PDS computes the CID, stores it, hands back a token, and you publish a
post. **Path B — Custom Feed Generator (the true proxy route):** to serve directly from your store
without copying, build an AppView or custom Feed Generator — a web service connecting to your store;
Bluesky asks your server what to show, it queries your store, formats a feed, streams metadata to the
client; files stay put. [src: docs.bsky.app] **Same backend S3 bucket:** you can configure your PDS
to use the same bucket where other app files live, but separated by folders (the PDS saves under
raw-CID names like `bafkreiebtv…`).

---

**USER:** I just want a PDS setup that can serve a file from my regular object storage even if it
takes some shim.

**GEMINI:** Yes, you can. You cannot tell the PDS to dynamically read a random path on demand, but you
can build a shim layer.

**Pattern 1 — Streaming pipeline (recommended).** A lightweight script (serverless function / background
microservice) opens a direct memory pipe between your storage and the PDS API; data streams from your
store through the script into the PDS without touching a local disk.

```javascript
import { S3Client, GetObjectCommand } from "@aws-sdk/client-s3";
import { BskyAgent } from "@atproto/api";

async function serveFileToBluesky(fileKey) {
  // 1. Open a live readable stream from your regular data store
  const s3 = new S3Client({ region: "us-east-1" });
  const s3Object = await s3.send(new GetObjectCommand({
    Bucket: "your-regular-application-bucket",
    Key: fileKey // e.g., "portfolio/artwork_102.jpg"
  }));

  const fileStream = s3Object.Body; // This is a live data stream

  // 2. Initialize your Bluesky agent pointing to your PDS
  const agent = new BskyAgent({ service: 'https://your-pds-domain.com' });
  await agent.login({ identifier: 'you.your-domain.com', password: 'your-app-password' });

  // 3. Pipe the stream directly into the PDS API endpoint
  // The PDS will hash the bytes on the fly, save it to its internal storage, and register it.
  const uploadRes = await agent.uploadBlob(fileStream, {
    encoding: 'image/jpeg'
  });

  // 4. Publish the record using the generated blob reference
  await agent.post({
    text: "This image was served straight out of my app's object storage via a stream shim!",
    embed: {
      $type: 'app.bsky.embed.images',
      images: [{ alt: 'Automated upload', image: uploadRes.data.blob }]
    }
  });
}
```

Pros: 100% AT-Protocol-compliant; auto-updates the PDS registries. Cons: it still transfers bytes from
your bucket to the PDS bucket (near-instant/free if same data center).

**Pattern 2 — Cloud-copy + DB-injection hack (zero byte movement).** For massive files, a deeper shim:

```
[Regular S3 Path] ──(1. S3 CopyObject)──> [PDS S3 Path /blocks/CID]
                                                │
[Your Automation Script] ──(2. SQL Insert)──────▼──> [PDS SQLite DB]
```

Workflow: (1) **calculate the CID** (an atproto-compliant Content Identifier — a SHA-256 raw
multihash, CIDv1 base32, `bafkreiebtv…`); (2) **S3 internal copy** (AWS `CopyObject`) mapping
`s3://my-bucket/video.mp4` → `s3://my-pds-bucket/blobs/bafkreiebtv…` (cloud-internal, no
download/upload); (3) **database injection** — connect to the PDS's SQLite (`pds.sqlite`) and `INSERT`
a row into the blob-tracking table marking the CID a valid authorized blob for the account; (4)
**publish** via `putRecord`. Pros: zero wasted bandwidth. Cons: **highly fragile** — if Bluesky changes
the SQLite schema, your injection breaks.

---

**USER:** I don't want to store the file twice.

**GEMINI:** To completely avoid storing the file twice, bypass the PDS storage layer entirely with a
**Reverse Proxy Interceptor Shim.** When the network/client wants a blob it sends
`GET /xrpc/com.atproto.sync.getBlob?cid=bafkrei…`. Instead of letting the PDS look in its own bucket,
your reverse proxy (Caddy/Nginx/Cloudflare Worker) intercepts the request, maps the CID to your
regular object-storage path, and streams the file from its original home.

```
[User/Relay Request] ──> [Reverse Proxy (Caddy/Nginx)]
                                │
         ┌──────────────────────┴──────────────────────┐
         ▼ (If regular PDS request)                    ▼ (If getBlob + matching CID)
   [Forward to PDS]                            [Fetch directly from your regular S3]
```

**Step 1 — Tell the PDS the file exists (SQL injection):**

```sql
INSERT INTO blob (cid, mimeType, size, createdAt)
VALUES ('bafkreih...your_file_cid...', 'image/jpeg', 1048576, '2026-06-07T00:00:00.000Z');

INSERT INTO repo_blob (did, cid)
VALUES ('did:plc:your_user_did', 'bafkreih...your_file_cid...');
```

**Step 2 — Maintain a local mapping** (key-value/text file/table): Key `bafkreih…` → Value
`https://s3.amazonaws.com/my-main-bucket/portfolio/artwork_102.jpg`.

**Step 3 — Configure the proxy shim (Caddy):**

```
# Intercept Bluesky blob requests for your zero-duplication files
@custom_blob {
    path /xrpc/com.atproto.sync.getBlob
    query cid=bafkreih...your_file_cid...
}

# Stream the file directly from your regular storage instead of the PDS
reverse_proxy @custom_blob s3.amazonaws.com {
    header_up Host s3.amazonaws.com
    rewrite /my-main-bucket/portfolio/artwork_102.jpg
}

# Fallback: Hand all other traffic over to the normal PDS container
reverse_proxy * localhost:3000
```

Why it works: zero double-storage (bytes live only in your primary bucket); no PDS forking required
(just routing HTTP at the edge); protocol-compliant (to the network the file looks natively served —
Relays/AppViews get the exact bytes + cryptographic validation they expect).

---

**USER:** Does a pds store only images and blobs or comments and text posts too?

**GEMINI:** A PDS stores everything that makes up your account's presence — not just images. Text
updates, replies, likes, follows, display-name changes — all stored authoritatively on your PDS. It
splits this into two layers:

1. **Repository layer (text, comments, likes, follows).** Packed into a single compressed cryptographic
   structure called a **Repository** ("repo") — exactly like a local Git repo. Each post/comment becomes
   a small formatted record dropped into your database tree and signed with your private key. Because the
   PDS must compute and sign these states to federate them, you **cannot** proxy/intercept text posts
   from an external system — they live natively inside the PDS's primary DB engine.
2. **Blob store layer (images/videos).** Packing giant media into a Git-like tree would bloat it, so
   atproto separates them: the text record holds a tiny string (the CID); the raw bytes go to the blob
   store (S3 bucket or local folder).

For your setup: text posts/comments/profile always live in the PDS's local DB; the Reverse Proxy
Interceptor Shim remains perfect — your PDS hosts text natively while the proxy steps in only for media
blobs, pulling raw bytes from your external storage without duplicating a megabyte.

---

**USER:** Can I access the text of posts and such directly myself in the PDS?

**GEMINI:** Yes. It's your own server — you have root access. While atproto packs your data into a
compressed Git-like tree (a **Merkle Search Tree**) for federation, the official PDS software indexes
and mirrors this into an easily readable format for local use.

**Method 1 — Clean API route (recommended).** Query the local `listRecords` XRPC endpoint:

```bash
curl "https://your-pds-domain.com/xrpc/com.atproto.repo.listRecords?repo=yourname.com&collection=app.bsky.feed.post&limit=5"
```

The raw text lives in `value.text`:

```json
{
  "records": [
    {
      "uri": "at://did:plc:123/app.bsky.feed.post/3k43tv4rft22g",
      "cid": "bafkreiebtv...",
      "value": {
        "$type": "app.bsky.feed.post",
        "createdAt": "2026-06-07T21:30:00.000Z",
        "langs": ["en"],
        "text": "This is the exact text of my post stored on my PDS!"
      }
    }
  ]
}
```

**Method 2 — Raw database route (the hacker way).** SSH in; the official PDS uses SQLite. Data usually
sits in `/pds/`; look for `pds.sqlite` (or the per-actor DB in a multi-tenant setup). The PDS maintains
a `record` table mirroring every post/comment/like/follow, with the `value` column storing record
details as stringified JSON:

```bash
sqlite3 /pds/pds.sqlite "SELECT value FROM record WHERE collection = 'app.bsky.feed.post';"
```

---

**USER:** What is the cheapest durable object store available?

**GEMINI:** The lowest price depends on how much you transfer out (egress). Because hyperscalers (AWS
S3 ~$23/TB) charge hefty egress penalties, alternative providers dominate the low-cost market.

| Provider | Storage / TB / Month | Egress Cost | Best Used For |
|---|---|---|---|
| iDrive e2 | $4.00 | Free up to 3x stored data | Ultra-budget active hosting |
| Hetzner Object Storage | $5.99 | Free up to 10 TB/month | European-centric infrastructure |
| Backblaze B2 | $6.00 | $10.00/TB (or Free via Cloudflare) | Backups or paired with a CDN |
| Wasabi | $6.99 | Free (fair-use) | Predictable flat-rate storage |
| Cloudflare R2 | $15.00 | $0.00 (Completely Free) | High-traffic public media/assets |

**1. Cheapest overall:** Backblaze B2 + Cloudflare CDN — B2 charges $6/TB; via the Bandwidth Alliance
with Cloudflare, routing requests through Cloudflare's proxy drops egress to $0. **2. Cheapest for
massive traffic:** Cloudflare R2 — despite $15/TB base, it has blanket zero egress + zero API-op fees.
**3. Cheapest standalone (no CDN):** iDrive e2 — $4/TB, no ingress/egress charge while download traffic
stays under 3x stored volume/month. **A warning on Wasabi:** the "free egress" flat rate comes with a
strict **90-day minimum retention** — delete a file after 5 days and you're still billed for the
remaining 85. [src: LeanOps / Mixpeek]

*(FACTCHECK: services/tiers real, figures in-range but volatile/point-in-time. ElfHosted's listed
$9/mo is weak — store shows a $1/7-day intro trial.)*

---

**USER:** Plans to store older less-accessed data cheaper but still available?

**GEMINI:** Use **Infrequent Access (IA), Coldline, or Archive** tiers — storage becomes very cheap,
but you pay a small per-GB fee on read/download. They split by how fast you need the data back.

**1. Instant access (millisecond delivery):** Google Cloud Archive $1.20/TB (ms access, $0.05/GB
retrieval + 365-day min); AWS S3 Glacier Instant $4.10/TB (ms, $0.03/GB + 90-day min); Google Cloud
Coldline $4.00/TB (ms, $0.02/GB + 90-day min); Azure Blob Cold $4.50/TB (ms, $0.03/GB + 90-day min).
The GCP Archive wildcard: millisecond access to the deepest archive tier, traded for a steep retrieval
fee + 1-year commitment. **2. Deep archive (delayed):** AWS S3 Glacier Deep Archive $1.01/TB (12–48h
restore); Azure Blob Archive $0.99/TB (1–15h rehydrate). **3. "Set it and forget it":** AWS S3
Intelligent-Tiering auto-moves objects (untouched 30d → IA; 90d → Archive Instant Access; clicked
later → back to active, no retrieval fee) for a tiny monitoring fee.

**Three cold-storage traps:** minimum-retention rules (billed for the full window even if you delete
early); the retrieval penalty (downloading 10 TB out of Deep Archive can top $1,000 in fees);
small-file bloat (per-object metadata fees — zip/tar millions of tiny files before archiving).

---

**USER:** What about wasabi?

**GEMINI:** Wasabi takes the opposite approach: a single "Hot" tier with flat pricing, positioned as a
replacement for both hot and cold. As of mid-2026, ~$6.99/TB/month (rising to $7.99/TB/month on July 1,
2026) across all regions. [src: Wasabi] **The good:** instant retrieval (millisecond, vs 12–48h for
Glacier Deep Archive); zero egress/retrieval/API fees (restoring a 10 TB archive costs nothing extra).
**The bad:** more expensive than true deep archive (Glacier Deep ~$1.01, Azure Archive ~$0.99 win if
egress is never a factor); the **90-day minimum retention trap** ("Timed Deleted Storage"). **Verdict:**
use Wasabi if your "cold" data is an *active* archive (stored cheaply, fetched instantly without surprise
bills); skip it for compliance/DR data you'll never touch unless the building burns down.

---

**USER:** Where can I read about the architecture of a PDS and at proto? At proto can be used for
things other than micro blogging right? And I still don't quite understand app view.

**GEMINI:**

**Where to read.** The official docs (atproto.com) are well-written: start with the Protocol Overview
and Understanding Atproto guides. **For backend thinkers:** "AT Protocol for distributed systems
engineers" (database scaling, federation bottlenecks, state management). **Community:** Dan Abramov's
essays — "Open Social — The protocol is the API" and "A Social Filesystem" — explain why atproto acts
more like a shared network hard drive than a social network. [src: atproto.com / AT Protocol]

**ATProto beyond microblogging.** Atproto is agnostic about what you store — essentially a
decentralized, public JSON database framework. It feels like microblogging because the first major app
on it is Bluesky, which defines its data with a schema system called **Lexicons** under `app.bsky.*`
(e.g. `app.bsky.feed.post`, `app.bsky.graph.follow`). You can ignore Bluesky's schemas and write your
own Lexicon namespace: blogging (`com.longform.article`), e-commerce (`xyz.marketplace.item`), media
sharing. Anyone can publish a new Lexicon, so developers are already building alternative photo apps,
discussion boards, and curation tools on the same accounts. [src: AT Protocol]

**Demystifying the AppView.** In atproto the responsibilities are torn apart:

| Component | What it is | What it knows |
|---|---|---|
| PDS (Personal Data Server) | Your personal vault | Only your account, posts, and local media |
| Relay | The network pipeline | Listens to every PDS, combines updates into a single live stream (the firehose) |
| AppView | The application engine | Consumes the firehose, organizes data, runs app logic (search, trends, timelines) |

Mental model: your PDS is your personal local hard drive — your posts get written there. But your phone
wants a timeline of 50 people on 40 servers, with like-counts and search; your PDS is too small. The
AppView acts like a specialized search-engine indexer: it watches the global firehose, grabs every raw
JSON post worldwide, dumps them into a heavily-indexed DB cluster, calculates global metrics, tracks the
social graph, runs search/algorithms. When your phone asks for the home timeline, it queries the
**AppView**, not your PDS. [src: GitHub / AT Protocol]

---

**USER:** A PDS assumes all files are public bc that is its purpose right?

**GEMINI:** You hit the nail on the head. Right now a PDS operates on the absolute assumption that
everything you write to your repository is 100% public — an open data layer for the social web. The
moment you write a post/comment/like or change your display name, the PDS signs it and broadcasts it to
the global Relay firehose. There are no ACLs or "private account" toggles in the core repository layer.
[src: AT Protocol]

Nuance: **1. Repositories vs. blobs** — records (text/links/follows) are blasted to the world; media
blobs are only accessible if someone knows the exact CID, but because CIDs are published in your public
posts, anyone can fetch your media without logging in. **2. Private data is on the roadmap** — the
creators have explicitly stated non-public data mechanisms (private groups, encrypted personal storage)
are planned, but they're taking their time to build a native architecture, and explicitly advise against
"bolting on" encryption to the current public setup, because a public content-addressed tree is a
terrible place to hide secrets. **3. How apps handle privacy today** — they step outside the standard
repo: Bluesky DMs use a separate traditional chat infrastructure (not your public repo tree); Germ
bypasses public PDS storage for messages, using your PDS identity only to look up public keys, building
an isolated E2EE pipeline elsewhere. So: if data goes into a standard PDS repository, assume it belongs
to the world. [src: AT Protocol]

---

**USER:** Where can I find this planning work? [re: non-public data mechanisms planned for the future]

**GEMINI:** The planning, architectural brainstorming, and official statements are spread across the
official spec documents and deep-dive GitHub threads:

1. **The official specification (atproto.com)** — the root spec ("AT Protocol Structure / Non-Public
   Data") maintains the team's formal stance, explicitly warning against hacking private data into the
   public layout: *"Mechanisms for personal and group data sharing are planned, and should support a
   wide variety of application functionality. These will be large additions to the protocol framework.
   We recommend against simply 'bolting on' encryption or private content using the existing protocol
   primitives."* [src: bluesky-jp +2]
2. **GitHub Discussion #3363 ("Private, non-shared data in repo?")** — the most active space; the team
   outlines a major proposed extension known as **"Namespaces."** Instead of one public repository, a
   PDS would allow separate data namespaces; unlike your public repo, these private namespaces wouldn't
   use Merkle Search Trees (MSTs) and wouldn't broadcast to the firehose — they'd sit in a standard
   database on your PDS and strictly require authentication + ACLs to read/write.
3. **GitHub Discussion #121 ("Encryption for private content")** — focuses on cryptography: group-private
   content and true E2EE without breaking decentralization. Maps out how Data Encryption Keys (DEKs)
   could be rotated, asymmetric key exchanges between DIDs, and how to eventually implement MLS or the
   Signal protocol. Captures the engineering debate on how to revoke access to old data without forcing
   re-encryption/re-upload of terabytes of historical media.

---

**USER:** Read the thread and give me current state and contention.

**GEMINI:** The discussions show the community is actively working on these features, shifting from
abstract planning to concrete implementation, marked by the official formation of the **ATProto Private
Data Working Group (WG)**. The architecture team, including core developers like **Paul Frazee**, has
split the problem into two categories: **Personal-Private Data** (unshared — bookmarks, drafts, app
preferences) and **Shared-Private Data** (restricted-audience — private accounts, "circles," group
interactions). [src: Paul's Leaflets — Leaflet.pub]

The core technical debates dividing the engineers fall into three areas:

**1. The "Trusted PDS" dilemma (zero-knowledge vs. trusted agent).** How much should your PDS operator
see? **Contention:** privacy advocates argue for a true zero-knowledge architecture where the client
encrypts before data leaves the device (even your PDS host can't read it). **Counterpoint:** the core
team treats the PDS like a web browser — an inherently trusted piece of infrastructure acting on your
behalf (it already holds your repo-signing keys and identity), so forcing it out of the loop creates
massive client-side friction.

**2. PDS complexity vs. cheap self-hosting.** Atproto scales because public data is broadcast-oriented —
a PDS just signs and dumps onto a firehose without thinking about who can read it. **Proposed solution:**
a parallel "Private Namespace" system; private data bypasses the public MST entirely, and apps use scoped
OAuth tokens to open a direct authenticated replication stream from a separate DB partition on your PDS.
**Contention:** adding ACLs, crypto verification middleware, and per-request token checking makes the PDS
significantly heavier; a core requirement is that a PDS stays cheap/simple enough to self-host on a $4
VPS, and engineers are fiercely protective of that low overhead. [src: Paul's Leaflets]

**3. Cryptographic key revocation (the "cat out of the bag" problem).** If you encrypt a post for a group
of friends, how do you kick someone out later? The team is against "plopping encrypted bytes" into the
public firehose (a public ledger lets malicious actors archive your encrypted data forever and wait for a
future key leak). Two choices: **Option A** — a unique DEK per post (highly secure, massive overhead);
**Option B** — a single master key per circle, rotated on removal for future posts. **Contention:**
whether a user must retroactively re-encrypt and re-upload all past content on rotation. One faction
pushes absolute historical privacy; the pragmatists lean "cat is already out of the bag" — if someone had
access before, they could have screenshotted/scraped it anyway, so rotation applies only moving forward.

*(FACTCHECK — the consequential cluster: GitHub #3363/#121, the atproto.com "Non-Public Data" quote, the
personal-vs-shared split, the Frazee leaflets, the trusted-PDS/cheap-self-host/key-revocation debates are
all CONFIRMED. The one correction: the WG is **community-led** (atproto.wiki / discourse.atprotocol.
community, organized by Boris Mann), **not an officially-chartered Bluesky body**; Frazee participates
**informally** and hedged it. Latest design vocab has moved toward "buckets"/"realms"; "namespaces" is
the earlier #3363 framing. This is the **real** private-data WG — distinct from, and not contradicting,
the earlier-refuted *fictional* "AT Messaging / MLS-standardizing" working group.)*
