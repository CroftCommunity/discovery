# Raw transcript — AT Proto architecture explainer: AppView / PDS / Relay / DIDs / firehose (Gemini)

source: explainer Q&A dialogue, pasted 2026-06-22 · **cleaned-paste, content-faithful — NOT a
byte-pristine export** (PLAYBOOK §4). A walkthrough of AT-Protocol mechanics: what an **AppView**
is (relational index over the firehose), the three-layer split (PDS → Relay → AppView → client),
**Lexicons as schemas**, **did:web vs did:plc**, how **long-form** rides shared identity, the
**PDS↔Relay** WebSocket (`subscribeRepos`), CAR/DAG-CBOR framing, **rev (TID) / seq** dedup,
**backfill** (`getRepo`/`listRepos`), `requestCrawl` discovery, stale-endpoint handling on
migration, and the **feed-generator (skeleton) vs AppView (hydration)** split.

> **Provenance & verification status (READ FIRST).** AI-assistant (**Gemini**) explainer; the user
> flags Gemini as sometimes unreliable. Fact-checked 2026-06-22 — see companion
> **`atproto-architecture-appview-relay-explainer-2026-06-22-FACTCHECK.md`**. **This transcript is
> unusually accurate** — the atproto mechanics are overwhelmingly correct and mostly *restate
> facts already settled in the corpus*. Carry-forward corrections:
> - **REFUTED:** "**did:plc** = *Public Liaison Corporation*" — it is **"Public Ledger of
>   Credentials"** (did-method-plc spec). Do not propagate the fabricated expansion.
> - **OUTDATED/PARTLY:** "**Relays maintain a full backup copy of every user's repository**" and
>   "call `getRepo`/`listRepos` **on the Relay** to backfill the whole network" describe the
>   *legacy archival relay (BGS)*. Post-**Sync v1.1 (2025)** relays are **non-archival** — they
>   "eliminate the need to crawl or store user data," keeping only a configurable **backfill
>   window** (e.g. ~24h ≈ a few hundred GB). Full-history backfill now goes to the **PDS** via
>   `getRepo`, or to a tool like **Tap**. (The transcript is internally inconsistent: its own
>   "2 vCPU / 12 GB RAM" figure *exists because* relays stopped storing full repos.)
> - **CONFIRMED-despite-suspicion:** the **"2 vCPU / 12 GB RAM / ~$34-a-month / runs on a
>   Raspberry Pi" full-network relay** is real and *current* (Sync v1.1; Bryan Newbold); **Tap**
>   is a real official Go repo-sync tool; `atblueprints/awesome-atproto` exists (but the active
>   canonical list is `awesome-atproto/awesome-atproto`; `beeman/awesome-atproto` was archived
>   Jul 2025).
>
> **For atproto facts, the project source of truth is
> `atproto-atmospheric-web-iroh-mobile-FACTCHECK.md`** (+ its dated addenda) — cite, don't
> re-derive. Note: the *atproto repo IS a Merkle Search Tree* (correct here; the MST error only
> ever applied to iroh-docs). Seam map: COHESION. Inline citation source-name lines + figure
> references ("1000007923.png", "this pic") preserved as pasted; user/Gemini typos verbatim.

---

## The dialogue (verbatim content)

> Explain an app view on blusky

On Bluesky and its underlying technology, the AT Protocol, an AppView (Application View) isn't just a screen layout on your phone. It is a critical backend architectural component that acts like a specialized search engine for the decentralized web.
AT Protocol

Because Bluesky is federated, user data isn't locked inside one giant company database. Instead, everyone's posts, likes, and follows are scattered across thousands of independent servers. The AppView is the engine that pulls all that scattered data together into a cohesive, fast social media experience.

### How It Works: The Three-Layer System
To understand an AppView, it helps to see how it works alongside the other two core pieces of Bluesky's infrastructure:

The PDS (Personal Data Server): This is your personal digital vault. Whenever you post, upload a photo, or follow someone, that data is saved directly to your PDS. It holds the raw material, but it doesn't know what the rest of the network is doing.
AT Protocol

The Relay: This is a high-speed pipeline. It constantly listens to every PDS across the entire network, aggregates all new actions into a single massive live stream of data (often called the "firehose"), and blasts it out.
AT Protocol

The AppView: This is the indexer. The AppView drinks from the Relay's firehose, organizes the messy stream of raw data, and builds a massive relational database. When your phone app asks to "load the timeline," it talks directly to the AppView.
GitHub

The Restaurant Analogy: Think of your PDS as an independent farm producing ingredients. The Relay is the delivery logistics company rushing all the ingredients to a central hub. The AppView is the chef who takes those raw ingredients, cooks them into specific dishes, and plates them neatly for the customer (your app client).

### Why Is This Important?
Traditional social media networks handle data storage and the user interface all under one roof. Bluesky splits them up. This structural split gives you a few major advantages:

Lightweight Client Apps: Your phone doesn't have to melt its processor trying to contact millions of individual mini-servers to fetch your timeline. It just makes a clean API call to an AppView, which serves the compiled data instantly.

True Account Portability: Because your data lives safely in your PDS and not inside the AppView, you can completely swap out your client app or move to a different server without losing your posts, followers, or social graph.

Custom Feeds and Algorithmic Choice: Anyone can build their own AppView or custom feed generator. If you don't like how the default Bluesky AppView ranks posts or handles moderation, you can easily plug into a different one that organizes the exact same public data by completely different rules.
AT Protocol

---

> Tell me more about how bluesky architecture works

Bluesky's entire system runs on an open-source framework called the AT Protocol (Authenticated Transfer Protocol). Unlike older decentralized networks (like Mastodon/ActivityPub) where individual servers handle hosting, processing, and moderation all at once, Bluesky separates these jobs into independent microservices.
Steve Klabnik

This design choice ensures that the user experience is just as fast and seamless as a centralized network, while still giving you complete control over your identity, data, and algorithms.
The AT Protocol Architecture, AI generated
The AT Protocol Architecture. Source: Level Up Coding - Gitconnected

### The Core Components
As shown in the architectural layout above, the network relies on a pipeline where data is created at the user level, broadcast to the infrastructure layer, and processed by specialized services.

#### 1. Identity & Data Repositories (The Foundation)
User Identity (DID): Your account isn't tied to a server's domain. Instead, you are assigned a DID (Decentralized Identifier)—a permanent, cryptographic string. Your visible handle (like @chase.com) simply points to this DID. If you switch servers, you change the pointer, but your cryptographic identity remains identical.
Bluesky API

Data Repositories: Your posts, likes, and follows are stored in a signed data repo that behaves like a Git repository. Every update you make is cryptographically signed by your private key, proving you created it, no matter where it is hosted.
Bluesky API

Lexicons: To make sure different apps can talk to each other, the AT Protocol uses schemas called Lexicons. They define the rules for data structures (e.g., what fields a post or a like must contain).
Bluesky API

#### 2. Personal Data Servers (PDS)
Think of the PDS as your personal home in the cloud. It manages your login, securely holds your cryptographic signing keys, and hosts your raw data repository. When you press "Post," that data goes directly into your PDS. Because your data is self-certified with your DID, if your PDS host goes offline, you can seamlessly import your repository backup to a brand-new PDS without losing a single follower.
Bluesky API

#### 3. Network Relays (The Firehose)
In traditional federation, servers talk to each other in a massive web of individual connections. To avoid the massive lag this creates, Bluesky uses Relays. Relays do the "big world" heavy lifting: they crawl every PDS across the entire internet, pull all the data repos into one place, and broadcast a massive, real-time public data stream known as the Firehose.
Wikipedia

#### 4. App Views (The Custom Data Indexes)
The raw Firehose is just text and bytes. An AppView drinks from the Relay's Firehose and turns that data into a high-performance database. It tallies metrics (like counting how many total likes a post received) and organizes threads so your client app can load your feed instantly.
The Fediverse Report

#### 5. Opinionated Services (Feeds & Moderation)
Because the data layer is decoupled from the user experience, anyone can build complementary services that hook directly into the AppView:

Feed Generators: These are independent algorithms. Developers use custom rules to filter the network's data (e.g., creating a feed dedicated entirely to Linux, photography, or hot news) without needing permission from Bluesky.
The Fediverse Report

Labeling Services (Moderation): Moderation is completely open. Independent entities run Labelers that look at the network data and apply descriptive labels (e.g., flagging spam, adult content, or misinformation). You choose which labeling services to subscribe to in your app settings, giving you control over what content gets blurred, hidden, or highlighted.
The Fediverse Report

### Why This Architecture Solves the "Decentralization Dilemma"
Historically, decentralized platforms faced a trade-off: they were either fast but highly centralized, or open but slow and difficult to scale. Bluesky bypasses this by implementing a "big world with small world fallbacks" design.
Bluesky

| | Centralized Networks | Traditional Federated (ActivityPub) | Bluesky (AT Protocol) |
|---|---|---|---|
| Data Ownership | Corporate silo | Tied to your specific server | — |
| Performance | Instant | Slow sync times across servers | — |
| Algorithmic Choice | Single corporate algorithm | Reverse-chronological only | — |
| Moderation | Top-down dictate | Decided entirely by your server admin | — |

By ensuring that the base data structure is verifiable by default, the architecture permits massive aggregators (like Relays and AppViews) to optimize for speed and scale without creating the vendor lock-in that defines traditional social media.

---

> Lexicon is a schema?
>
> Explain using did::web vs alternatives

Yes, exactly. A Lexicon is just the AT Protocol's word for a schema.

If you've ever worked with OpenAPI (Swagger) or JSON Schema, it's the exact same concept. A Lexicon is a JSON document that defines the strict rules for data structures and API endpoints. For example, the app.bsky.feed.post Lexicon dictates that a post must have a text string, a createdAt timestamp, and can optionally have an embed object for images. Because everyone agrees to follow these Lexicons, any developer can build an app that reads and writes Bluesky data flawlessly.

Now, let's dive into how the network handles identity using DIDs (Decentralized Identifiers), specifically contrasting did:web with Bluesky's custom alternative, did:plc.

### The Identity Problem: Handles vs. DIDs
On Bluesky, your handle (like @alice.com) is just a user-friendly wrapper. Under the hood, the entire network tracks you using a DID—a permanent, unique string of characters that never changes.

If you change your handle, your DID stays the same, meaning you keep all your followers, posts, and history. How that DID is managed depends on the "method" used. The two main options on the AT Protocol are did:web and did:plc.

#### 1. did:web (The Domain-Based Approach)
The did:web method ties your cryptographic identity directly to a traditional web domain that you own.

How it works: Your DID might look like did:web:example.com. When the Bluesky network wants to verify that you actually own this identity, it sends a standard HTTPS request to [https://example.com/.well-known/did.json](https://example.com/.well-known/did.json). That file contains your public cryptographic keys.

The Big Benefit: It relies entirely on the existing, battle-tested World Wide Web infrastructure (DNS). You don't need a blockchain or a centralized registry. If you already own a website, you completely own your identity.

The Flaw (Why Bluesky doesn't use it by default): It breaks host portability. If you host your identity on a domain name, and you stop paying your annual domain registration fee, someone else can buy that domain, change the did.json file, and completely hijack your identity and all your followers. Furthermore, if a government or registrar seizes your domain, your digital identity is erased.

#### 2. did:plc (The Portable Alternative)
Because did:web makes it risky to move your identity if you don't permanently own a domain, Bluesky created did:plc (Public Liaison Corporation). This is the default method used when you sign up for a standard Bluesky account.

How it works: Instead of looking up a file on a private website, did:plc looks up your identity in a highly optimized, public, append-only directory ledger run by a consortium. Your DID looks like a random string: did:plc:z72yd78kl....

The Big Benefit (True Portability): This directory maps your permanent DID to two things: your current public key and your current PDS (where your data lives). If you decide to move your account from Bluesky's official servers to a self-hosted server in your basement, you just sign a transaction with your private key to update the PLC directory. Your DID never changes, your followers never notice a thing, and your identity cannot be killed by a domain expiration.

The Trade-off: It introduces a small degree of centralization. Right now, the PLC directory is maintained by the Bluesky team (though they are transitioning it to an independent, multi-organization consortium).

### Summary Comparison
| Feature | did:web | did:plc |
|---|---|---|
| What hooks it together? | A web domain (example.com) | A public, append-only registry ledger |
| Who controls it? | You (via your domain registrar) | You (via cryptographic keys) |
| What happens if you move hosts? | Harder. You must maintain control of the original domain forever. | Seamless. You just update the registry pointer. |
| Censorship Resistance | Vulnerable to domain seizure/DNS takedowns. | High. It relies entirely on cryptography, not DNS. |
| Best For | Brands, organizations, and developers who already own and tightly control a prominent domain. | Everyday users who want effortless account portability without managing web infrastructure. |

---

> Explain at proto more in depth and walk me through how long form sharing is built on at proto to share identity

To understand the AT Protocol (atproto) deeply, you have to think of it less like a traditional web API and more like Git.

On the legacy web, data validity depends entirely on where it came from (e.g., "I trust this post because I fetched it directly from a specific corporate database"). If you move that data somewhere else, the trust is broken. Atproto fixes this by using self-authenticating data. Every piece of content you create is cryptographically signed by your private key and assigned a CID (Content Identifier)—a unique hash based on the content itself. Because the signature and the identity are baked directly into the data, the content is verifiable no matter who is hosting it, caching it, or moving it around.
Wikipedia

Your entire social history is stored as a collection of these signed records in a personal Repository. Structurally, this repository is organized as a Merkle Search Tree (MST), allowing the network to sync changes instantly by only analyzing what changed—identical to how Git handles code commits.
James Dumar

### How Long-Form Sharing is Built on Atproto
Because atproto doesn't dictate what an application must look like, it isn't limited to short microblogging posts. The protocol treats short thoughts, long-form articles, photo galleries, and code repositories exactly the same: they are just different Lexicons (schemas) stored inside your repository.

The open ecosystem utilizes publishing formats like Standard.site (site.standard.document), which allows creators to publish full-length articles directly onto the protocol. Here is the step-by-step architectural walkthrough of how a long-form article is written, published, and instantly tied to your global identity across the entire network:

**1 — Anchor the Cryptographic Identity (Step 1)**
Before you type a single word, your identity is verified by your permanent DID (like did:plc:123...). This identifier acts as your global public key. No matter what blogging app, CMS, or text editor you open to write your article, you log in using this single identity via atproto OAuth.

**2 — Write and Sign the Record (Step 2)**
When you hit publish, the writing platform packages your text, title, and formatting into a JSON document that fits the long-form Lexicon schema. Your local client uses your account's private cryptographic key to sign the record. This creates an unforgeable proof that your DID—and by extension, your public username—authored this exact text.

**3 — Commit to your PDS (Step 3)**
The signed long-form article record is pushed directly to your Personal Data Server (PDS), which appends this record to your repository tree. If the article contains massive assets (like high-res photos), the text record references them as Blobs—secure, content-addressed binary blocks hosted right alongside your text.

**4 — Broadcast to the Firehose (Step 4)**
The moment your PDS receives the new article, it signals the network Relays. The Relays ingest the update and stream it out across the global Firehose (a live WebSocket stream of every action happening on the network). Every indexer listening to the network now knows a new piece of long-form content exists.

**5 — Cross-App Indexing and Rendering (Step 5)**
Specialized indexers and AppViews parse the firehose and see the long-form record signed by your DID. Because the identity is shared network-wide, a microblogging app like Bluesky instantly recognizes you as the author. It automatically generates a rich article card preview right in your followers' timelines, matching it perfectly with your profile avatar, bio, and handle.

### The Identity Payoff: Cross-App Interoperability
Decoupling identity from the specific app enables a unified experience for long-form content that traditional blogging or RSS feeds cannot match:

Native Cross-Platform Interactions: When someone opens your long-form article on a standalone blog site and leaves a comment, that comment is written back to the network as an atproto reply record. It instantly surfaces as a reply thread back on your social media timeline.

Zero Profile Fragmentation: You don't have to maintain separate, disconnected profiles across a portfolio of blogging and social platforms. You have one identity (your DID). Your long-form articles, short-form thoughts, and personal moderation preferences all orbit around that single cryptographic node.
AT Protocol

Platform Independence: If the blogging application you used to write your long-form articles shuts down, your articles don't vanish. The raw, signed text records are sitting completely intact inside your own PDS repository. You can simply log into a entirely different atproto-compatible blogging client, point it at your repository, and your entire blog layout is restored instantly.

---

> So the default relays will process any lexicon?

Yes, the default relays will absolutely process any lexicon. They don't care what your data actually means.

This design detail highlights the genius of the AT Protocol's layer separation. Because the Relay and the AppView are independent, they look at your data through completely different lenses.

### The Relay's Perspective: "Just Move the Blocks"
The default Bluesky Relays (like the main bsky.network relay) are lexicon-agnostic. Their only job is infrastructure and cryptographic security, not content moderation or formatting.

When you publish a record with a brand-new, completely custom lexicon—for example, com.chase.gardening.plant—the Relay handles it exactly like a standard Bluesky post:

It checks the cryptographic signature to ensure it matches your DID.

It verifies that your PDS correctly updated your repository's structural tree (the Merkle Search Tree).

It bundles the raw, binary-encoded data (DAG-CBOR) into a commit package.

It blasts it out over the global Firehose WebSocket stream for the whole world to see.

The Relay doesn't validate whether the content matches a known schema, nor does it block it for being unfamiliar. As long as the cryptography is valid and the data isn't corrupt, the Relay will pass it along.

### The Catch: Relays Broadcast, AppViews Choose
While your custom lexicon will successfully stream across the global firehose via the default Relay, nothing will happen to it visually until an indexer notices it.

The Default Bluesky AppView Ignores It: The official Bluesky AppView is hardcoded to look only for schemas in the app.bsky.* namespace (posts, likes, follows, reposts). When your custom com.chase.gardening.plant data flies past on the firehose, the Bluesky AppView simply discards it because it doesn't know how to render a plant database.

A Custom AppView Saves It: To make use of your custom lexicon, you (or another developer) would build a specialized AppView. This custom server listens to the exact same default Relay firehose, filters specifically for com.chase.gardening.plant records, and populates its own custom database to power a brand-new app interface.

### Real-World Use: Open Unions
Developers are already using this exact mechanic to experiment on the live network without needing Bluesky's permission.

The protocol includes things called Open Unions—slots in existing schemas designed to accept unknown data types. For example, the standard Bluesky post schema has an open union slot for embeds. Normally, this holds images or link cards. However, a developer could define a custom lexicon like com.whiteboard.canvas and embed a collaborative drawing file directly inside a standard Bluesky post.
Bluesky API

The default Relay will process it effortlessly, custom clients will render an interactive drawing board, and the official Bluesky app will simply display a clean fallback message saying it encountered an unrecognized media format.

---

> So am app view is a relational view of firehouse data ready for a presentation layer which could look like basically anything?

Exactly. You have nailed the architectural concept perfectly.

An AppView is essentially a highly optimized, read-heavy query engine built on top of a chaotic, append-only event stream.

The Firehose itself is just a firehose—a linear, never-ending broadcast of raw events: "User A liked Record X," "User B updated Profile Y," "User C created Custom Record Z." You can't build a responsive user interface by reading a linear stream of events from scratch every time a user opens their phone; it would take forever just to calculate how many likes a single post has.

The AppView steps in to do the heavy lifting:

Ingest & Aggregate: It drinks the Firehose stream continuously.

Normalize & Index: It processes that data into a traditional database state (whether that's relational SQL, a graph database, or a document store like Elasticsearch). It tracks state, counts metrics, resolves blocks, and stitches disjointed data pieces into coherent threads.

Expose an API: It wraps that optimized database in a clean API layer (using atproto's XRPC, which is just HTTP/JSON with strict Lexicon enforcement).

### The Presentation Layer Can Be Anything
Because the AppView simply serves structured JSON endpoints, the frontend presentation layer has total design freedom.

Depending on which Lexicons the AppView chooses to index, the "presentation layer" could manifest in wildly different ways using the exact same underlying protocol infrastructure:

A Microblogging App: The default Bluesky client (looks like Twitter/X).

A Long-Form Magazine: A platform like Whitewind or Standard.site that renders the exact same identity and text records as a beautiful, distraction-free reading layout.

An E-commerce Marketplace: An AppView that indexes custom product listing and review lexicons, displaying a grid of items for sale, shopping carts, and seller ratings.

A Forum or Link Aggregator: A Reddit-style layout that groups records by "sub-communities" and orders them strictly by upvote/downvote records rather than a chronological timeline.

A Terminal CLI: A text-only hacker interface running inside a Linux terminal that pulls the exact same data but strips out all the CSS, images, and UI fluff.

By separating the data generation (PDS), the data transport (Relay), the data indexing (AppView), and the data presentation (Client UI), the AT Protocol turns the entire social web into a public utility Lego set.

---

> How does the PDS and Relay actually communicate? Does the PDS just send up JSON event data on each new item? What prevents a PDS from sending things more than once? How does fetching historical content work?

The communication layer of the AT Protocol handles high-volume data exchange by relying on cryptographic state trees and binary formatting rather than standard text-based web patterns.
AT Protocol

### 1. How the PDS and Relay Communicate
The connection between a PDS and a Relay is a persistent WebSocket connection.

Interestingly, the data flow is driven by the Relay reaching out to the PDS. The Relay subscribes to a public, unauthenticated endpoint on the PDS called com.atproto.sync.subscribeRepos. Once this handshake is complete, the PDS maintains the open pipe and pushes data up to the Relay in real time whenever a user creates, updates, or deletes content.

### 2. The Data Format (It's Not Just JSON)
The PDS does not simply emit raw JSON event strings. Throwing plain text across the web for millions of users would choke network performance and fail identity validation. Instead, atproto uses a split frame format over the WebSocket:

The Header (JSON): A lightweight metadata wrapper that tells downstream consumers basic information: the event type (e.g., #commit), the user's did, and a global sequence number.

The Payload (Binary CAR Blocks): The actual meat of the event is sent as a CAR (Content Addressable Archive) slice. This binary package contains raw blocks encoded in DAG-CBOR (a binary, deterministic variation of JSON).
atproto.blue

Instead of sending text like "Chase posted a photo," the PDS transmits a cryptographic delta of your repo's Merkle Search Tree (MST) along with the content hash (CID) of the new record, all cryptographically signed by your private key.

### 3. Preventing Duplicates and Out-of-Order Delivery
Because distributed systems are notorious for network lag, retries, and out-of-order packets, atproto uses two distinct tracking numbers to maintain absolute state sanity:

#### The Repo Revision (rev)
Every single mutation to a user's repository creates a new commit block. This commit is stamped with a rev identifier, which uses a format called a Timestamp ID (TID). TIDs are strictly monotonic (always increasing) and lexicographically sortable. If you make three posts, they are explicitly ordered rev A, rev B, and rev C.

#### The Relay Sequence (seq)
As the Relay drinks from thousands of individual PDS streams, it stamps every single inbound event with a globally incrementing sequence number (seq = 101, 102, 103...).

How Downstream Apps Filter Duplicates: Because content is addressed by its hash, data is self-deduplicating. When an AppView indexes your data, it remembers the highest rev it has seen for your specific DID. If a PDS experiences a network hiccup and sends the same commit frame twice, the AppView inspects the frame, realizes the rev is less than or equal to the state it already saved, and instantly drops it without processing it twice.

### 4. How Fetching Historical Content Works (Backfilling)
If an AppView crashes, loses its database, or a brand-new service launches and needs to learn what happened on the network over the last two years, it cannot rely on the live firehose. It must perform a Historical Backfill.

To do this, the service bypasses the Relay firehose entirely and communicates with the PDS over standard HTTP using the endpoint com.atproto.sync.getRepo.
AT Protocol

Full Export: If the service has no record of a user, it calls getRepo with the user's did. The PDS packages the entire historical Merkle Search Tree and every single record that user has ever created into a single .car file and streams it down as a cold file download.

Catch-Up (Delta Sync): If the service went offline for twelve hours, it doesn't need to re-download the user's entire history. It calls getRepo but passes an optional query parameter: since=rev_XYZ (the last known revision it successfully saved before crashing). The PDS reads its database, calculates the exact structural difference between rev_XYZ and the current state, and replies with a tiny CAR slice containing only the blocks missing from that window.

---

> Won't the relays be overrun at some point if they continue to allow bespoke lexicons?
>
> How does a relay know to register a web socket with a PDS?

Both of these questions touch exactly on the major scaling bottlenecks the creators of the AT Protocol had to solve when designing a network intended to handle millions of users without centralizing.

### 1. Won't Relays Be Overrun by Bespoke Lexicons?
It seems like a massive vulnerability: if anyone can invent a com.chase.heavy-data-dump lexicon and flood the network, wouldn't the Relays collapse under the weight?

The reason they don't is that Relays completely blind themselves to the content.

To a Relay, your data isn't text, images, or schemas—it is just raw cryptographic math. The Relay performs three fast, low-overhead operations:

It validates the cryptographic signature against your DID.

It verifies that your repository's structural tree (the Merkle Search Tree) folds together correctly.

It bundles those raw bytes and pushes them out the door.

Because it does zero database indexing, text searching, string parsing, or Lexicon validation, a Relay requires surprisingly little computational power. In fact, a single mid-range server (about 2 vCPU cores and 12 GB of RAM) can process the entire global network firehose, validating thousands of events per second.
AT Protocol

#### The Real Scale Safeguards
If a rogue PDS begins blasting petabytes of junk data, the protocol handles it at the infrastructure level, not the data level:

Rate Limiting: Relays aggressively track inbound data per PDS and per DID. If a server acts up, the Relay throttles or blocks that specific host's IP/domain.

The AppView Buffer: The expensive work (parsing text, calculating likes, rendering threads) is pushed entirely to the AppViews. If a custom lexicon floods the network, the default Bluesky AppView simply drops the packets instantly.

Multiple Relays: There doesn't have to be just one Relay. The ecosystem is designed so there can be dozens of independent Relays. If a specific Relay gets overwhelmed or refuses to carry certain traffic, custom apps can point to a different Relay that filters or handles data differently.
WhiteWind

### 2. How Does a Relay Know to Connect to a PDS?
A Relay cannot pull data from a PDS if it doesn't know the PDS exists. The protocol handles discovery using two main pathways: active notification and passive identity tracking.

#### Pathway A: Active Notification (requestCrawl)
When a developer spins up a brand-new self-hosted PDS, that PDS has to introduce itself to the network's Relays. It does this by making a standard HTTP POST request to the Relay's public endpoint:

```
POST /xrpc/com.atproto.sync.requestCrawl
Host: relay.bsky.network
Content-Type: application/json

{
  "hostname": "pds.chase.com"
}
```

The Handshake: This endpoint is entirely unauthenticated. When the Relay receives this ping, it notes the hostname, runs a quick DNS check, and verifies that the endpoint is a valid, functional PDS.

The Connection: Once verified, the Relay turns around and opens the persistent WebSocket back to that PDS (com.atproto.sync.subscribeRepos), dialing into its live data stream.

#### Pathway B: Passive Identity Tracing
What happens if a PDS goes offline, restarts, or a user migrates to a server the Relay hasn't actively mapped?

Relays constantly watch the global identity registries (like the did:plc directory or standard did:web text files). Every time a user changes their handle, signs up, or migrates their account, an identity event occurs.
AT Protocol

The Relay parses that identity change, extracts the associated PDS web address (declared in the user's DID document), and checks its internal list. If that PDS address isn't currently in its database, the Relay automatically triggers a crawl request to that new host to catch up on any missing data blocks.

---

> Is there a way to see app views feeding off of the main relays?

Technically speaking, no—there is no master dashboard, admin panel, or public API endpoint on the main Relays that lists every AppView currently drinking from the firehose.

This omission is an intentional security and architectural feature of the protocol, but the open nature of the network provides alternative methods for discovering these applications.

### The Architectural Reason: The Firehose is Anonymous
The main Relay endpoint (com.atproto.sync.subscribeRepos) is entirely public and unauthenticated.

The Relay does not require developers to register an API key, sign an agreement, or declare who they are to stream the network's data. Anyone with a server and a WebSocket client can run a command and instantly start downloading the global firehose.

Because the Relay treats incoming connections purely as anonymous TCP/WebSocket pipes, it has no semantic understanding of whether a connected client is:

A massive, production-grade AppView compiling a database.

A small Feed Generator calculating a custom algorithm.

A university researcher archiving data for an academic paper.

A hobbyist developer running a script in a Linux terminal just to watch the text fly by.

Sharing a live list of every connected IP address would create a massive security and privacy vulnerability.

### How to Actually Discover Alternative AppViews
While the Relay won't show you who is listening, the ecosystem makes it easy to find who is serving by looking at the protocol's data registry and client configurations.

#### 1. In-Repo Registrations (The Declarative Way)
While full alternative AppViews operate independently, specialized mini-AppViews like Feed Generators and Independent Labelers (moderation services) must declare themselves directly to the network so users can find them.

They do this by publishing a record inside their own PDS repository using specific core lexicons (like app.bsky.feed.generator or com.atproto.labeler.service). Because these records are public on the firehose, any client or indexer can crawl the network and build a searchable directory of them. This is how the native Bluesky "Feeds" and "Moderation" tabs function.

#### 2. Client-Side Switching
Because an AppView's job is to serve a presentation layer, the easiest way to see alternative AppViews is to look at alternative client apps.

Many advanced AT Protocol clients feature a settings toggle labeled "Custom Service" or "AppView URL." By default, these point to Bluesky's official indexing server (api.bsky.app), but you can manually swap that URL out for an alternative AppView domain.

#### 3. Community Curated Lists
Because the developer ecosystem is deeply rooted in open-source culture, the most reliable way to find full-scale alternative AppViews (like WhiteWind for long-form blogging or SmokeSignal for event planning) is through developer-maintained directories. The central hub for tracking these creations is the community-driven "Awesome ATProto" repository on GitHub, which categorizes active AppViews, alternative clients, and custom network tools as they launch.

---

> They publish this once or continually?
>
> They do this by publishing a record inside their own PDS repository using specific core lexicons (like app.bsky.feed.generator or com.atproto.labeler.service). Because these records are public on the firehose, any client or indexer can crawl the network and build a searchable directory of them. This is how the native Bluesky "Feeds" and "Moderation" tabs function.
>
> Link to?
>
> Awesome ATProto" repository on GitHub, which categorizes active AppViews, alternative clients, and custom network tools as they launch.

### How Often Are These Records Published?
They publish it once.

In the AT Protocol, a feed generator or labeling service registration behaves exactly like your profile bio or a standard social media post: it is a single, discrete record saved inside that account's PDS repository.

The Initial Push: When the developer first deploys their service, they run a script that creates the record (e.g., at app.bsky.feed.generator). The PDS appends it to their Merkle Search Tree, and the global Firehose blasts out a single commit event notifying the network that the record now exists.

Updates and Edits: If the developer needs to change the feed's name, upload a new avatar image, or point it to a different backend server endpoint, they don't create a new record. Instead, they perform a mutation (an update) on that exact same record. The PDS updates the file, and the Firehose broadcasts the edit event once.

Once published, the record sits quietly in their repo. Any new AppView or directory that spins up months later doesn't need to wait for a continuous broadcast; it simply crawls that specific repository directly to read the setup configuration.

### The Community Directory Link
As requested, here is the curated directory tracking the decentralized ecosystem:

The Directory: GitHub - atblueprints/awesome-atproto

This repository is an excellent starting point if you want to explore what developers are building. It keeps a living, categorized list of alternative client apps, specialized AppViews (like blogging or event platforms), code libraries across different programming languages, and experimental custom tools running on the protocol.

---

> So you would need to crawl every PDS?
>
> Once published, the record sits quietly in their repo. Any new AppView or directory that spins up months later doesn't need to wait for a continuous broadcast; it simply crawls that specific repository directly to read the setup configuration.
>
> How would you discover the PDS's available?

You actually do not have to crawl every individual PDS to fetch historical data or find out what servers exist.

Because the AT Protocol treats full-network Relays as massive, global replication caches, a brand-new AppView or directory can grab the entire history of the network from just a single Relay.

### 1. Do You Need to Crawl Every PDS for History?
No. Relays aren't just pass-through pipes for the live firehose; they maintain a full backup copy of every user's data repository across the entire network.

Crucially, Relays implement the exact same programmatic endpoints as a PDS. If you launch a brand-new AppView tomorrow and need to perform a historical backfill (downloading the past several years of network activity), your data pipeline looks like this:

Connect to a Full-Network Relay (like relay.bsky.network).

Call com.atproto.sync.listRepos directly on the Relay. The Relay will hand you a massive master list of every single active account identifier (DID) on the network.

Call com.atproto.sync.getRepo on the Relay for each DID. The Relay will stream down a .car (Content Addressable Archive) file containing that specific user's entire repository history.

Developers use open-source utility tools (like Tap) to automate this transition. Tap handles downloading the historical database blocks for the Lexicons you care about, buffers incoming live firehose events in the background, and seamlessly switches your app over to the real-time stream once the history is caught up.

### 2. How Do You Discover Which PDSs Exist?
If you ever did want to bypass the Relay's cache and talk directly to the authoritative origin servers hosting the data, you discover the PDS locations by working backward from user identities.

The protocol maps accounts to physical servers using a two-step discovery chain:

```
[DID List from Relay] ──> [Resolve DID Document] ──> [Extract PDS Service URL]
```

Step 1: Get the DID List
As mentioned above, you pull the master list of active identities (did:plc:... or did:web:...) from a Relay or public ledger.

Step 2: Resolve the Cryptographic Identity
For every DID on your list, you look up its public DID Document. This is a small JSON file containing the user's public keys and metadata. The lookup method depends on the identifier type:

If the account is a did:plc:1234..., you query the public, open registry at [https://plc.directory/did:plc:1234](https://plc.directory/did:plc:1234)....

If the account is a did:web:chase.com, you query the domain directly at [https://chase.com/.well-known/did.json](https://chase.com/.well-known/did.json).

Step 3: Extract the PDS Location
Inside that resolved DID Document, there is a standardized block called service. It explicitly declares the web address of the machine currently hosting that user's data. A typical entry looks like this:

```json
"service": [
  {
    "id": "#atproto_pds",
    "type": "AtprotoPersonalDataServer",
    "serviceEndpoint": "https://pds.chase.com"
  }
]
```

By parsing these documents, your system automatically builds a dynamic, self-updating map of every single functional PDS on the global internet without needing a central server registry.

---

> Won't service endpoints be stale if a user moves their PDS? Or domain?
>
> In this pic, what is a feed generator? Qn app view?

### Handling Stale Endpoints During Migration
When a user moves their PDS or updates their domain, the network prevents endpoints from staying stale using a combination of live event broadcasts and lazy error-recovery.

How the network updates depends entirely on which identity method the user chosen:

#### 1. If using did:plc (The Default)
When a user migrates to a new PDS, the final step of the migration script submits a signed update to the public PLC directory.
AT Protocol

The Firehose Alert: The moment the directory updates, the ledger emits an #identity event directly onto the global network Firehose.
AT Protocol

Instant Cache Invalidation: Active Relays and AppViews listening to the Firehose see this #identity packet, drop their old record for that user, and immediately map the user's DID to the new PDS address.
AT Protocol

#### 2. If using did:web (Domain-Based)
If a user hosts their identity on their own website domain (like did:web:example.com), they have to manually update the .well-known/did.json file on their server. Because there is no central registry to blast out a Firehose event for a domain change, downstream services rely on two backup defenses:
GitHub

Time-To-Live (TTL) Expiration: AppViews don't cache identity lookups forever. They routinely expire entries after a fixed window and force a clean lookup.

Error-Driven Refresh: If an AppView attempts to deliver data or verify an authorization token from a user's old PDS, the signature validation will fail. The protocol dictates that when a service encounters a definitive signature validation error, it must immediately clear its local identity cache and run a fresh lookup against the origin domain to check for updates.
AT Protocol

### What Is a Feed Generator vs. An AppView?
Looking at the text in the file 1000007923.png, a Feed Generator is not a full AppView. Instead, think of it as a lightweight, specialized satellite service that works in tandem with an AppView to deliver a custom experience.
GitHub

The text in 1000007923.png notes that anyone can create a feed generator using arbitrary criteria—like a curated list of accounts, hashtags, emojis, or machine learning algorithms.

```
[User App] ──(1. Request Feed)──> [AppView] ──(2. Request Skeleton)──> [Feed Generator]
                                      │                                       │
[User App] <──(4. Hydrated Feed)──────┴ <──(3. Return Post URIs)──────────────┘
```

The split of responsibilities highlights why a Feed Generator is much simpler than an AppView:

#### The Feed Generator Only Handles "The Skeleton"
Building a full AppView requires immense database infrastructure to track every user profile, image blob, comment thread, block list, and like button. A Feed Generator doesn't do any of that.
GitHub

As outlined in 1000007923.png, the generator simply runs its specific sorting trick (e.g., looking for posts containing a specific emoji). When called, it does not return full posts; it returns a Skeleton—a simple JSON list of raw content hashes and post identifiers (URIs).
GitHub

#### The AppView Handles "The Hydration"
Once the Feed Generator hands that bare skeleton of URIs back to the network, the AppView takes over the heavy lifting. The AppView reads the list, cross-references it with its massive relational database, and hydrates the feed. It pulls the actual text, attaches user avatars, calculates the current like/repost counts, filters out posts from accounts you have blocked, and hands a fully formed, beautiful timeline to your device client.
GitHub

This division of labor means a developer can host a custom algorithmic feed generator on a tiny, low-cost server because the massive burden of database rendering is entirely offloaded to the AppView infrastructure.
GitHub

---

*(End of pasted dialogue.)*
