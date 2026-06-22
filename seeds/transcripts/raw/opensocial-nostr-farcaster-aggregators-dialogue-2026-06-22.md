# Raw transcript — Open-social protocols & aggregators: Nostr, Farcaster, Lens, thirdweb, Yup (Gemini)

source: design-exploration dialogue, pasted 2026-06-22 · **cleaned-paste, content-faithful —
NOT a byte-pristine export** (PLAYBOOK §4). Body 2 of a longer continuous Gemini session; companion
bodies: `iroh-quic-localfirst-ecosystem-dialogue-2026-06-22.md`,
`cooperative-social-union-governance-dialogue-2026-06-22.md`,
`atproto-pds-architecture-technical-dialogue-2026-06-22.md`.

> **Provenance & verification status (READ FIRST).** AI-assistant (**Gemini**) dialogue; user flags
> Gemini as sometimes unreliable. Fact-checked 2026-06-22 — see
> **`opensocial-nostr-farcaster-aggregators-dialogue-2026-06-22-FACTCHECK.md`** for verdict tables +
> source URLs. **Do not treat assertions as fact without the fact-check.** Gemini's failure mode
> here: fabricated/overstated **dollar figures, dates, acquisition details, founder attributions,
> and program names**, around a largely-accurate skeleton. Treat every funding number, DAU figure,
> acquisition date, and "Program X" name as **suspect until confirmed.**
>
> For atproto / iroh / iOS facts cite the project source of truth
> (`atproto-atmospheric-web-iroh-mobile-FACTCHECK.md`). Distilled (CONFIRMED/PARTLY only) into
> `ECOSYSTEM.md` / `research/` per the FACTCHECK. UI render chrome (citation chips, "AI generated"
> image captions) stripped per §4.

---

**[user]** Explain nostr to me

**[assistant]** Nostr = **Notes and Other Stuff Transmitted by Relays**. Not a platform or app — an
open, ultra-minimalist networking protocol for censorship-resistant social media/messaging. If
Twitter/X is a walled garden and Mastodon is federation, Nostr is a room full of independent bulletin
boards (relays) where you slap a signed note on a board and walk away.

Three pillars: (1) **Keys (identity)** — no usernames/passwords/email; a keypair: public key (npub,
your ID) + private key (nsec, your master password used to sign every note). You own your identity;
nobody can take it. (2) **Relays (plumbing)** — simple servers that accept/store/forward messages;
dumb and disposable; relays do NOT talk to each other; interchangeable (your client connects to a
handful; if one censors/spams, switch). (3) **Clients (apps)** — Damus/Amethyst (Twitter-like),
Primal (media curation), Coracle (relay-based network view). Paste your private key into a different
app and your followers/posts/DMs appear because they live on public relays, not in the app.

Trade-offs: censorship resistance vs public relays (anyone running a relay sees your posts); account
deletion is a "request" rogue relays can ignore; spam mitigated via proof-of-work or Lightning fees;
DMs encrypted to recipient pubkey but metadata (who/when/IP) largely public.

**[user]** Is there any service where I can pay for a nostr relay and have it store my content long
term?

**[assistant]** Yes. (1) Dedicated cloud hosting (you own the relay): Clovyr — managed
nostr-rs-relay (lightweight Rust relay) ~$10/mo, dedicated WebSocket endpoint, no terminal/SSL setup.
AWS Marketplace (KloudCover) — ARM-optimized relay servers, integrates with AWS Backup / encrypted
EBS. (2) Premium paid public relays (pay for admission via Lightning) — powered by software like
Nostream; the fee filters spam; predictable income → bigger storage / long-term retention; browse via
nostr.watch. ⚠️ The media catch: **Nostr relays only store text** (tiny JSON). Photos/videos/audio go
to a separate media server; only the text link is on the relay. For long-term media, pair a paid
relay with a paid **Blossom** server (modern Nostr media blob storage) or a media host (NostrCheck /
Void.cat). For certainty, run a personal instance (Clovyr) or self-host (Umbrel / StartOS).

**[user]** Explain how blossom works

**[assistant]** **Blossom** is an open-source protocol for Nostr media storage; fixes the early flaw
where a dead media server = lost photo. It brings content-addressed storage to ordinary HTTP servers.
Core concept: SHA-256 addressing — a file is identified by its SHA-256 hash (64-char string from the
raw bytes), so a file's ID is identical on every compliant server. Flow (documented as "BUDs" =
Blossom Upgrade Documents): (1) Uploading — client signs a temporary Nostr authorization event
(kind:24242) proving identity + permission; file sent via PUT /upload. (2) Resolution & redundancy —
publish a server preference list (kind:10063) telling the network which Blossom servers host your
media; if the first server is dead, the client tries the alternatives for the same SHA-256:
```
GET https://primary-server.com/<sha256>   --> [Server Down]
GET https://backup-server.net/<sha256>    --> [Success]
```
(3) Local verification — client re-hashes the download; if it matches, the file wasn't corrupted/
modified. Server-agnostic, portable, tamper-proof. Brings IPFS-like resiliency to Nostr but on fast,
cheap web servers.

**[user]** Biggest most notable nostr relays?

**[assistant]** Foundational free relays: wss://relay.damus.io (William Casarin, creator of Damus —
the most trafficked free relay), wss://nos.lol (fast/reliable default fallback), wss://relay.snort.
social (Snort team). Premium/paid: wss://nostr.wine (paid write, server-side indexing/search),
wss://cache.primal.net + wss://relay.primal.net (Primal caching relays, pre-compiled timelines),
wss://purplerelay.com (strict spam mitigation, long-term storage). Check live status at nostr.watch.
Ideal setup: two free massive relays + one paid premium.

**[user]** Does nostr support private groups?

**[assistant]** Yes, in two different ways. (1) **Relay-enforced (NIP-29)** — a group has an ID on a
specific relay; relay configured with a private tag; read/write requires auth (NIP-42); if your npub
isn't on the whitelist the relay refuses to serve data. Best for moderated Discord-like communities
where you trust the operator. (2) **True cryptographic E2EE (Marmot & MLS)** — Marmot is the glue
between Nostr and MLS (Messaging Layer Security, an IETF standard). MLS creates a shared group "key
tree" that updates as members join/leave; messages encrypted before touching a relay. Often pairs
with NIP-59 ("Gift Wrapping") to hide metadata (sender/recipient/topic). Where to use now: **White
Noise** (privacy-focused messaging on Nostr + Marmot/MLS, like an anonymous Signal/WhatsApp with
metadata-hidden group chats); **Amethyst** (popular Android client, integrated Marmot-compatible
encrypted group channels); NIP-29 clients (e.g. groups.nip29.com).

**[user]** Nostr alternatives and comparisons?

**[assistant]** Three architectural philosophies: federated/server-based (ActivityPub), decoupled
data (AT Protocol), on-chain graphs (Farcaster/Lens). (1) **ActivityPub** — W3C standard powering the
Fediverse (Mastodon, Pixelfed, Lemmy, Threads); email-like; you sign up on an instance; servers talk
to each other. (2) **AT Protocol** — by Bluesky; separates identity from data storage and algorithm;
data lives on a PDS; Relays scrape PDSes into a firehose; identity = DID mapped to a domain handle,
easy to switch providers without losing username/followers. (3) **Farcaster** — hybrid by ex-Coinbase
engineers; identity anchored to a smart contract on Ethereum L2 (Optimism); posts stored off-chain on
P2P "Hubs"; pioneered "Frames" (interactive mini-apps in a post).

Deep-dive comparison table:

| Feature | Nostr | ActivityPub | AT Protocol | Farcaster |
|---|---|---|---|---|
| Core architecture | Dumb relays + smart clients | Server-to-server federation | PDS nodes + central indexing relays | On-chain identity + off-chain P2P hubs |
| Identity | Keys (npub/nsec) | Web domain (@user@server.com) | DID / domain | Ethereum smart contract |
| Portability | Absolute (paste keys anywhere) | Flawed (ask old server to redirect) | High (swap PDS) | Absolute (locked to ETH wallet) |
| Moderation | Client-side | Server-side | Marketplace (labelers) | Hybrid |
| Native economy | Bitcoin Lightning (Zaps) | None | None | Heavy ETH/SocialFi |
| Major pain | Raw keys hard; relay spam | Clunky migration; scaling cost | Heavy self-host; relies on big relays | Hub infra cost; pay rent to post |

**[user]** Tell me more about farcaster, origin, status, key features and capabilities, size

**[assistant]** Farcaster separates identity from apps. (1) Origin: founded 2020 by Dan Romero and
Varun Srinivasan (former Coinbase execs); parent company Merkle Manufactory. Pioneered "Sufficient
Decentralization" — identity secured by blockchain, feed activity on high-speed off-chain servers.
(2) Features: hybrid architecture (account registry / Farcaster ID (FID) / signing keys on Optimism
L2; posts "casts", likes, follows off-chain on P2P Hubs that gossip the social graph); Frames
(interactive posts, launched early 2024 — vote/play/mint/checkout without leaving the feed); storage
rent (~$5/year buys a fixed allocation, e.g. 5,000 casts / 2,500 reactions); total client portability
(Warpcast is flagship but doesn't own data). (3) Current status — the 2026 Neynar acquisition: in
January 2026 the Farcaster protocol was acquired by Neynar (dominant dev-infra/data provider); the
Merkle Manufactory founders (Romero, Srinivasan) departed; plans to restructure incl. repaying the
original $180M VC backing; shift from venture startup toward an infrastructure play (enterprise tools,
cross-chain incl. BNB, Frames v2 for AI-agent integrations). (4) Size: raised $180M total, capped by a
$150M Series A led by Paradigm and a16z (~$1B valuation); >$2.8M cumulative storage-rent revenue;
~40,000–60,000 DAU (public Dune dashboards) but spam-filtered "Power Badge" humans closer to
~4,500–5,000 DAU. A high-value niche network (tech devs, crypto founders, creators), not a mainstream
X/Threads competitor.

**[user]** Compare frames and mini apps used by delta chat. Is farcaster open source?

**[assistant]** Is Farcaster open source? Yes, the core protocol — specs, smart contracts, the
TypeScript Hub nodes (hubble), and the Rust storage network (snapchain) — under MIT and GPL-3.0,
public on GitHub. Anyone can host a hub or build a competing app. The catch: Warpcast (flagship
client) is closed-source/proprietary (like email's open IMAP/SMTP vs Gmail's proprietary app).

Frames vs Delta Chat Mini Apps (WebXDC): Frames are centralized, server-driven web widgets for public
monetization; WebXDC are decentralized, local-first packages for private P2P collaboration. (1)
Execution: Frames = HTML Open Graph metadata on a remote server; clicking a button POSTs to the dev's
server which returns a new image+buttons; dependent on that server. WebXDC = self-contained .xdc
package (zipped HTML/CSS/JS) running in a local WebView sandbox; no dev server. (2) Networking/state:
Frames rely on Web2 cloud DB + active internet. WebXDC works offline; a move is packaged into a tiny
textual payload synced over Delta Chat's email/Iroh network. (3) Identity/crypto: Frames integrate
ETH/Optimism (signed by FID, can trigger contracts / buy / mint). WebXDC integrate private encryption
keys (Autocrypt/OpenPGP), no blockchain.

Summary table:

| Feature | Farcaster Frames | Delta Chat Mini Apps (WebXDC) |
|---|---|---|
| Use case | Marketing, minting, checkouts, feed games | Private collaborative pads, local multiplayer, shared lists |
| Architecture | Remote HTTP server + meta tags | Local HTML5/JS package (.xdc) |
| Offline | No | Yes |
| Storage | Centralized dev DB | Decentralized, mirrored across participant devices |
| Blockchain | Yes (Optimism/ETH) | No (P2P key mapping) |

**[user]** Could you use the same root crypto identity for both?

**[assistant]** Natively, no. Farcaster (and Nostr) use the **secp256k1** curve (Bitcoin/Ethereum).
Delta Chat uses OpenPGP/Autocrypt (Curve25519 / Ed25519). Clients can't natively compute each other's
keys. Theoretical loophole (BIP-39 seeds): one master mnemonic + different derivation paths can derive
an ETH key, a Schnorr key, and an Ed25519 key — but no consumer app supports this crossover; you'd
write custom code. Why a single root identity is a bad idea: the "blast radius" problem — one leaked
key collapses your entire footprint. Keep public social identity (Farcaster/Nostr) segregated from
private messaging (Delta Chat).

**[user]** Sure but could we sign a key for each from the same root for provenance

**[assistant]** Yes — cryptographic cross-signing / attestations. Generate two separate isolated keys,
then use a master identity (or the keys themselves) to sign a statement proving common ownership —
provenance without blast radius. Method A (bi-directional cross-signing): sign the PGP key with your
Farcaster wallet key, and sign the Farcaster ID with your PGP key; publish both signatures. Method B
(master root / DID document): a master identity (DID, or hardware key like a YubiKey) publishes a DID
Document listing operational keys as verified verification methods:
```json
{
  "id": "did:example:chaserootidentity",
  "verificationMethod": [
    { "id": "did:example:chaserootidentity#farcaster-key", "type": "JsonWebKey2020",
      "publicKeyJwk": { "crv": "secp256k1", "x": "..." } },
    { "id": "did:example:chaserootidentity#deltachat-key", "type": "Ed25519VerificationKey2020",
      "publicKeyMultibase": "z6Mkm..." }
  ]
}
```
Benefits: isolation, easy revocation (sign a revocation statement), protocol agility.

**[user]** What did all that farcaster seed money actually produce?

**[assistant]** The twist: the money wasn't spent. On the Neynar acquisition, Merkle Manufactory
co-founder Dan Romero confirmed returning the full $180M to investors (Paradigm, a16z); the founders
were already wealthy from Coinbase and ran lean. What 5 years produced: (1) infrastructure — Hubble
(TypeScript node gossip) + Snapchain (Rust storage layer, >10,000 tps, sync the whole network for
<$1,000/mo); (2) interaction — Frames v1 & v2 (Frames v2 = near-native HTML/CSS/JS apps in-feed); (3)
native embedded wallets (Sign In With Farcaster AuthKit); (4) a "scenius" — dense builder community
(~40–60k DAU, ~5,000 Power Badge humans); storage-rent economy ($2.8M cumulative); Clanker (autonomous
social trading tool, generated millions in fees, bundled into the Neynar handoff).

**[user]** Open source alternative clients?

**[assistant]** Warpcast is closed-source but the graph is open: (1) **Herocast** — leading
open-source power-user client (TweetDeck-like: multi-account, analytics, scheduling, multi-column).
(2) **Opencast** — minimalist self-hostable web client (Twitter-flavored, Docker, connect your own
Hub). (3) **Litecast** — lightweight mobile-first web client; strips heavy animations; good reference
codebase. (4) **Yup (Yup.live)** — multi-protocol aggregator (Farcaster + Lens + Bluesky + X);
cross-post + unified notifications.

**[user]** Tell me more about yup.live

**[assistant]** Yup (yup.live / app.yup.io) — open-source all-in-one decentralized social aggregator
merging Bluesky/Farcaster/Lens/X into one interface. Origin: co-founded by Nir Kabessa and Vernon
Johnson, started ~2019–2020 as a Web3 "social consensus protocol" (browser extension to rate/like
content anywhere; "Influence Mining" with the YUP token). Pivoted late 2023 to a multi-protocol
aggregator (attention fragmentation was the real problem). Features: one-click cross-posting, unified
chronological feed, native inline actions (like/reply/repost across networks), aggregated
notifications, cross-following. Tech: centralized OAuth for X; direct protocol integration for
Farcaster/Lens; WalletConnect auth. Status & funding: backed by IDEO CoLab Ventures ("Open Social
Meta" thesis); YUP token still on-chain for curation metrics; app is a clean consumer web utility.

**[user]** How do they make money?

**[assistant]** In February 2025, Yup was acquired by **thirdweb** (a Web3 dev-infra platform). Modern
monetization split: (1) multi-platform ad marketplace (cross-posted sponsored campaigns); (2)
transaction micro-fees (mints/tipping, e.g. Zora rails or $DEGEN — routing/commission cuts); (3) the
$YUP token economy; (4) enterprise developer tooling (thirdweb packages Yup's multi-protocol
indexers / firehoses / account-linking into SDKs sold via enterprise API tiers).

**[user]** Can you use it all without paying on your own? [Delta Chat / Yup / Nostr / Farcaster
costs] ... I meant the third web assets

**[assistant]** Housekeeping: the consumer app yup.live no longer exists — thirdweb sunset it after
acquisition and absorbed the codebase/team; the multi-protocol social tech became developer SDKs.
thirdweb pricing: free ($0) — client SDKs (React/React Native/TS/Gaming), smart contracts (you pay
only blockchain gas), self-hosted thirdweb Engine. Paid (managed cloud): Starter $9/mo ($25 usage
credits), Growth $99/mo (production, custom auth, 48-hr support), Scale/Pro $499+/mo (dedicated infra,
white-label). Indie self-hosters can build for free; pay $9/mo to skip dev-ops.

**[user]** What do you actually get for your money? Could you build another yup.live and charge for
it?

**[assistant]** You're paying for managed backend infrastructure, not the (open-source) code. Usage
credits feed five services: account abstraction & wallets (invisible in-app wallets via Google/Apple/
Sign-in-with-Farcaster; 1,000 MAU free then ~$0.015/MAU); gas sponsorship (paymaster, gasless UX);
high-speed RPC nodes (2,500+ networks); thirdweb Engine cloud (auto-scaling tx backend); cross-chain
bridge/payments (thirdweb Pay). Could you build another Yup and charge? Yes — Farcaster/Lens/Bluesky
are open at the data layer. Use thirdweb's Social Profiles API (matches a wallet to ENS/Lens/
Farcaster), Connect SDK for the front-end, Engine for the action layer. Monetize via: TweetDeck SaaS
($5–$15/mo power-user features); premium gasless tier; micro-transaction convenience cut (0.5–1% on
mints/tips via thirdweb Pay).

**[user]** Ok but only the crypto backend charge so the at proto and activity pub posting would not
incur per activity fees?

**[assistant]** Correct. AT Protocol (Bluesky) and ActivityPub (Mastodon) have no per-activity fees,
network rent, or gas. ActivityPub = standard HTTP POST to a server's API (only the admin pays
hosting). AT Protocol = XRPC pushes a tiny text record to a PDS repo; no blockchain validation layer →
no execution fee. Dev blueprint: Web3 pipeline (Farcaster/Lens) routes through thirdweb SDKs/Engine
(draws cloud credits for gas/queuing); federated pipeline (Bluesky/Mastodon) bypasses thirdweb (use
@atproto/api for Bluesky + REST for Mastodon; only your own server bandwidth). Margins: if 80% of a
power-user's posts target Bluesky/Mastodon/X, your infra cost is tiny; you can pocket margin on a flat
$5–$10/mo SaaS or subsidize free users with a small tip-volume cut.

**[user]** Tell me about lens and more about third web and how it got started, current status, and
philosophy

**[assistant]** **Lens Protocol** — pure infrastructure play. Origin: launched 2022 by Stani Kulechov
(founder of Aave) under parent company Avara. Philosophy: on-chain composability — Profiles as NFTs;
content/links/follows as on-chain records bound to that NFT; take your Lens NFT to many apps. Current
status: launched on Polygon, then migrated to a dedicated Lens ZK Chain (built on zkSync's ZK Stack);
Momoka = an "Optimistic L3" data-availability layer batching high-frequency actions; Open Actions
(embed smart-contract logic in a post). Funding: completed Token Generation Event; >$46M total funding
(Faction VC, Alchemy, Circle).

**thirdweb** — Web3 developer plumbing. How it started: founded 2021 by Furqan Rydhan, Jake Loo, and
Steven Bartlett (host of The Diary of a CEO); $24M Series A led by Haun Ventures (Coinbase Ventures,
Polygon, Shopify); ~$29M total. Philosophy: radical account abstraction — the end-user should never
know they're on a blockchain (log in with Apple/Google, invisible wallet, sponsored gas). Current
status: full-stack dev platform; pricing $9/$99/$499; pivot into the AI Agent Economy (Nebula App,
Agentic Wallet architectures — AI models owning crypto keys, holding capital, interacting via natural
language). Its acquisition of consumer aggregators (Yup) turns data-scraping software into
enterprise-grade developer APIs.

**[user]** It doesn't look like thirdweb kept the no crypto social network integrations

**[assistant]** Correct. When thirdweb acquired Yup and dismantled the consumer app, they gutted the
federated/non-crypto social-graph engine — they did NOT port AT Protocol (Bluesky) or ActivityPub
(Mastodon) cross-posting/aggregation. What they kept: (1) the "Social Profiles" API is strictly crypto
— ENS, Lens, Farcaster only (ignores Bluesky/Mastodon). (2) Web2 integrations reduced to logins —
X/Discord/Google/Apple as OAuth login buttons only (to spin up an embedded wallet), not feed indexing/
posting. Why: maintaining X/Bluesky/Mastodon API integrations is high-maintenance with little Web3
upside. To build a "TweetDeck for the entire open web," use thirdweb for Farcaster/Lens and write your
own adapters for Bluesky/Mastodon.

**[user]** Are the yup.live prior code available to inspect? Can I see where they left off?

**[assistant]** Yes, public. The yup-io org on GitHub has docs/historical contracts, but the full
front-end aggregator ecosystem (web/mobile/desktop) is preserved at **github.com/andrei0x309/yup-live**
— a community monorepo by Andrei Olteanu (andrei0x309), frozen open after the thirdweb acquisition for
historical purposes. Structure (Turborepo, Vue 3, TypeScript): apps/yup-live (Vite/Vue3 web), apps/
yup-live-mobile (Ionic + Vue3), apps/yup-live-desktop (Tauri), packages/shared (multi-protocol data
models + API abstractions for Farcaster hubs/Arweave/Lens/Bluesky/Web2). README notes: "Yup was
acquired by thirdweb around 11 FEB 2025. There are no plans to continue development... Repo will stay
open for historical purposes."

**[user]** Any other aggregation projects that exist? ... Opensource and repackageable to make a
downstream distro? ... Link to flare? And sky feed? Any other options? Economic models for these?

**[assistant]** Active aggregation/cross-posting projects: (1) **Firefly** (by Mask Network) — the most
polished consumer multi-protocol aggregator (X + Farcaster + Lens + Bluesky; plus Mirror, Snapshot).
(2) **Bridgy Fed** — protocol-level bridge mapping ActivityPub (Mastodon) ↔ AT Protocol (Bluesky);
creates a ghost mirror of your profile across networks; CC0 / public domain (fork freely, commercial
Docker, no source-share required). (3) **Fedica & Buffer** — mainstream SaaS suites with Bluesky/
Mastodon adapters (no Web3/Farcaster). (4) **SkyFeed** — open-source AT-Protocol feed builder/
customizer.

Repackageable to make a downstream distro: yup-live (Vue3/Turborepo/Ionic/Tauri; unmaintained);
Bridgy Fed (Python/Granary; CC0 — total freedom, no copyleft); **Flare** (DimensionDev / Mask Network;
Kotlin Multiplatform Android/iOS/Web; pulls Mastodon/Bluesky/X/Misskey/Nostr/RSS; **AGPL-3.0** — host
it as a service and you must publish modified source); **SkyFeed** (Dart/Flutter; EUPL-1.2 copyleft).
Links: Flare = github.com/DimensionDev/Flare; SkyFeed = github.com/skyfeed-dev/app. Other repackageable
options: **Mixpost** (github.com/inovector/mixpost — self-hosted Buffer/Hootsuite alternative,
open-core/dual-license); **CrossPoster** (github.com/run-llama/crossposter — Next.js cross-poster with
AI drafting).

Economic models for downstream distros: (1) open-core / dual-licensing (Mixpost-style: free Community
Edition + paid Pro/Enterprise white-label); (2) managed hosting / DevOps SaaS ($9–$49/mo); (3)
protocol transaction cut (0.5–1% on mints/swaps/tips for Web3 components); (4) ecosystem grant &
foundation funding (Mask Network, Arbitrum Foundation grants for downstream clients).

**[user]** Tell me about these — "networks like Mask Network (the backer behind Flare) and The
Arbitrum Foundation actively..."

**[assistant]** (1) **Mask Network** ("the Tencent of Web3") — founded by Suji Yan; started as a
browser extension for encrypted messages/crypto tips on top of X/Facebook; now a conglomerate governed
by MaskDAO. Strategy: acquire/fund every layer. Assumed full stewardship over **Lens Protocol** (Avara/
Aave stay as technical advisors). Owns consumer apps: **Firefly** (aggregator), Web3.bio (identity
mapping), **Orb** (mobile social on Lens, >50,000 active users). Operates large Mastodon instances:
mstdn.jp, mastodon.cloud, Pawoo.net. Through VC arm **Bonfire Union**, manages two funds totaling
$100M, invested in 120+ decentralized social projects. (2) **Arbitrum Foundation** — Ethereum L2;
wants social apps on its rails (high-frequency micro-transactions). Programs: milestone grants
($20k–$150k in ARB, no equity); "ArbiFuel" (gas-fee sponsorship pools so downstream apps are free for
users); a $10M smart-contract audit program; a $1M "Trailblazer" program for AI agents in open social.
Big picture: Mask = product builder/steward/app acquirer ($100M Bonfire Union + MaskDAO); Arbitrum =
infrastructure provider / grant funder (ARB treasury; gas/grants/audits).
