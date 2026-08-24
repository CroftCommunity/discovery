# Raw: WebAuthn-PRF key derivation · atproto clients · Bluesky native chat · Germ alternatives · Waypoints (Gemini, pasted 2026-08-23)

**Preservation status: preserved-condensed (cleaned-paste, content-faithful — NOT byte-pristine) —
PLAYBOOK §4.** Source: a **Gemini** dialogue (owner-confirmed 2026-08-23: "yes to gemini"),
dialogue date unknown, pasted 2026-08-23. Citation chips ("Wikipedia +1", "graysky.app",
"Metricool", "Global Dating Insights", "Dating Industry Insights", "Bluesky", "GitHub") reproduced
as bracketed markers as-authored; Gemini's chip targets are unfollowable from a paste. The
Waypoints thread responds to a **screenshot the owner shared with Gemini that is not in the
paste**. No secrets.

**WHOLE-DOCUMENT FACT POSTURE — read before citing anything below.**

- **DRIFT CAUGHT (the §4 drift grep working as intended):** the Germ-alternatives answer
  reintroduces *"an active open-source effort within the ATProto community (the atmessaging-proto
  working group) defining a decentralized, open-standard MLS framework."* This is a
  **known-corrected error**: the FACTCHECK SoT
  (`atproto-atmospheric-web-iroh-mobile-FACTCHECK.md`, "AT Proto encryption working group
  standardizing 'AT Messaging' with MLS") records it as **REFUTED — no such working group
  exists**; atproto's own docs list MLS/Matrix/Signal only as *possible future* work, and E2EE on
  atproto is third-party (Germ). The adjacent claim "Bluesky is integrating protocols like MLS
  for future E2EE native chats" is the same overclaim. **Do not cite these.**
- **WebAuthn PRF baseline is already verified** in `../../../research/2026-07-27-social-tree-factcheck.md`
  (from the Social-Tree dialogue's claim set) — cite that, don't re-verify. The mechanism summary
  here (hmac-secret lineage, salt → 32-byte deterministic secret → HKDF → derived keys; the
  legacy signature-as-entropy workaround and why nondeterministic signatures break it) is
  consistent with it. Specific vendor claims (Bitwarden's use, Yubico docs) `[UNVERIFIED]`.
- **Client lists are not registry-grade.** Graysky, Blacksky, deck.blue, Openvibe are already in
  `ECOSYSTEM.md` (verified rows). The rest of the names (Skylight, Skyline, Indigo, Anisota,
  Red Dwarf, Bluepy, Leaflet, Standard Reader, Catsky, Witchsky; SDKs `bisky`/`zat`,
  `idunno.Bluesky`, `atproto.dart`) are **`[UNVERIFIED]` and plausibly partly invented** — a
  model-generated listicle. Deliberately NOT added to ECOSYSTEM. Leaflet (leaflet.pub) is the one
  most likely real-and-relevant; verify before use.
- **Waypoints (`@aturi.to/waypoints`)** — registered in ECOSYSTEM §5m as dialogue-sourced
  `[UNVERIFIED]`, **including its code example and supported-apps list** (the `resolveAtUri` API
  shape and the client URLs may be invented). The concept — an "Open in…" client picker resolving
  `at://` URIs to the viewer's preferred app — is the load-bearing part.
- Bluesky native-chat claims (`chat.bsky.convo.*` centralized off-repo service, PDS
  service-header proxying, TLS-only, admin-readable) are consistent with the corpus's prior
  atproto-mechanics material; specifics `[UNVERIFIED]` this session.

**What this is.** A five-thread Gemini lookup session: (1) re-finding the WebAuthn-PRF
derive-keypairs-from-passkeys technique the owner half-remembered; (2) an atproto client/SDK
landscape; (3) a mobile Bluesky-app comparison; (4) how Bluesky native chat actually works (not
PFS, not in the PDS repo); (5) whether non-Germ E2EE can integrate natively (yes — open lexicons +
DID auth; the WG claim is the drift, see above); (6) what Waypoints is, from an owner screenshot.
Seams distilled to COHESION §76 (PRF ↔ key-custody/device-loss; Waypoints ↔ the tier-zero
deep-link resolver).

---

## Thread 1 — finding the WebAuthn key-derivation technique

**Owner:**

> I once found an article about using webauthn to generate more keypairs to use in keypair
> operations can yo uhelp me find that?

**Gemini:**

> The technique you are thinking of centers on the WebAuthn PRF (Pseudo-Random Function)
> Extension (derived from the CTAP2 hmac-secret extension).
>
> Because WebAuthn natively restricts keypairs to hardware enclaves and only allows signing
> WebAuthn-formatted challenges, developers use the PRF extension to deterministically derive
> arbitrary symmetric or asymmetric keypairs (e.g., Ed25519, Secp256k1, or RSA) for external
> cryptographic operations like end-to-end encryption, Web3 wallets, or PGP.
>
> Key Articles & Resources Describing This Technique
>
> - **W3C WebAuthn Working Group – Explainer: PRF Extension.** The official W3C explainer
>   detailing how the prf extension uses the authenticator's hardware to evaluate a HMAC secret
>   based on a salt supplied by the website. Mechanism: Passkey + Salt Input ➔ Deterministic
>   32-Byte Secret ➔ HKDF ➔ Any Custom Keypair
> - **Bitwarden Tech Blog – "PRF WebAuthn and its role in passkeys".** Explains how Bitwarden
>   uses the WebAuthn PRF extension to derive client-side vault encryption and decryption keys
>   directly from passkeys without storing private key material on servers.
> - **Yubico Developer Documentation – HMAC-Secret & Using WebAuthn for Signing.** Details how
>   hardware keys (like YubiKeys) expose pseudo-random functions (hmac-secret) to output
>   deterministic entropy for local key derivation and non-repudiation signing workflows.
> - **Open-Source Crypto Projects & Medium/Reddit Walkthroughs.** Articles covering
>   Passkey-derived Web3 Wallets (deriving Ethereum/Solana private keys from Passkeys) and
>   Passkey PGP Key Management (deriving WASM-based PGP keypairs on-the-fly).
>
> How WebAuthn Key Derivation Works Under the Hood
>
> 1. **Salt Provision:** The application sends a fixed, public salt (e.g.,
>    "my-app-encryption-key-v1") during a WebAuthn assertion request.
> 2. **Hardware HMAC Computation:** The user's authenticator (Secure Enclave, YubiKey, Touch ID)
>    processes the salt internally with its master credential key via HMAC-SHA-256.
> 3. **Deterministic Output:** The authenticator returns a unique 32-byte secret that is
>    identical every time the same salt and passkey are used.
> 4. **Key Derivation (HKDF):** The client-side browser code feeds this 32-byte secret into HKDF
>    (HMAC-based Key Derivation Function) or WebCrypto API to generate sub-keys (e.g., AES-GCM
>    encryption keys or Ed25519/Secp256k1 signing keypairs).
>
> Alternative Legacy Approach (Signature as Entropy): Before the PRF extension gained wide
> browser support, developers used a workaround: requesting a WebAuthn signature on a static
> string and hashing the resulting signature to extract entropy for key derivation. However,
> because standard WebAuthn signatures often include random nonces or sign-counters, this
> approach was inconsistent compared to the native PRF extension.

## Thread 2 — atproto clients (apps + SDKs)

**Owner:** *(implicit query: "Atproto clients")*

**Gemini:**

> Because the AT Protocol (the decentralized architecture powering Bluesky) is entirely
> open-source, the term "client" usually refers to one of two things: End-User Apps (third-party
> applications designed to browse Bluesky and other AT Protocol networks) and Developer
> Libraries (SDKs used by developers to build bots, apps, and feeds on the protocol).
>
> End-User Apps (Bluesky Alternatives): Since the AT Protocol uses a modular architecture, you
> aren't forced to use the official Bluesky app. Anyone can build a custom interface (AppView)
> that connects to the same underlying data.
>
> - **Graysky** (iOS / Android): The most popular mobile alternative to the official Bluesky
>   app. Heavily praised for its polish, offering built-in post translation, GIF support, a
>   feed-first layout, and the ability to view other users' likes. *[cite: graysky.app]*
> - **Deck.blue** (Web): A TweetDeck-style web client for power users, multiple columns
>   monitoring different feeds, lists, and search queries side-by-side in real-time.
> - **Openvibe & Indigo** (Multi-Network): Built to bridge decentralized networks — unified
>   timeline and cross-posting to both the AT Protocol (Bluesky) and ActivityPub (Mastodon).
> - **Skyline**: A desktop-focused client geared toward performance and extensive UI
>   customization.
>
> Developer Client Libraries (SDKs): TypeScript — `@atproto/api` / `atcute` (official and most
> robust; web clients, React Native, Node backends). Python — `atproto` (bots, scrapers,
> Jetstream firehose). Dart/Flutter — `atproto.dart` (cross-platform mobile). Rust — `bisky` /
> `zat` (high-performance backend tools and firehose processing). C#/.NET — `idunno.Bluesky`
> (Windows desktop / enterprise .NET). (Note: Official SDKs have recently transitioned to using
> OAuth as the primary authentication method, replacing legacy App Passwords.)

## Thread 3 — best mobile Bluesky apps vs the official app

**Owner:**

> What are the best mobile apps for browsing Bluesky, and how do they compare to the official
> app?

**Gemini** *(condensed table reproduced as prose-faithful summary of its own rows)*:

> While the official Bluesky mobile app is the standard entry point, the AT Protocol's open
> architecture allows third-party developers to build custom mobile clients targeting power-user
> features, layout preferences, or multi-network capabilities the official app lacks.
>
> - **Official Bluesky App** (iOS, Android, Web): universal baseline, full account management,
>   deep moderation tooling; reliable, receives new protocol features first, conventional
>   single-feed UI.
> - **Graysky** (iOS, Android): polished performance, built-in translation, GIF picker, viewing
>   other users' likes; feed-first, surfaces a user's like history the official client hides.
> - **Skylight** (iOS, Android): built explicitly around anti-algorithm philosophies and
>   user-directed content control; clean, non-manipulative timelines.
> - **Openvibe / Indigo** (iOS, Android): multi-protocol bridging (Bluesky + Mastodon/
>   ActivityPub); cross-network timelines and cross-posting the official app cannot do.
>
> Key trade-offs: third-party clients often pioneer highly requested features first (native post
> translations, advanced GIF insertion, other accounts' likes) *[cite: graysky.app]*; background
> push notifications can be less reliable on indie apps; the official app wins on a clean
> straightforward feed, third-party on elevating pinned custom feeds or bridging networks.

## Thread 4 — how Bluesky native chat works

**Owner:**

> How does bluesky native chat work, I know it's not encrypted but is it just literally PFS
> entries in chat form?

**Gemini:**

> Bluesky's native direct messaging system is not a Perfect Forward Secrecy (PFS) setup wrapped
> in a chat interface, nor is it stored in your public AT Protocol repository. *[cite:
> Wikipedia]* Instead, native DMs are implemented via a centralized off-repo, server-side API
> service running parallel to the core federated network.
>
> How the Architecture Actually Works:
>
> - **Off-Repo Storage:** Public posts, likes, and follows are signed Merkle tree records stored
>   inside your Personal Data Server (PDS) repository and exposed on the global firehose. DMs do
>   not live in your PDS repo. Storing them in a public, cryptographically verifiable
>   repository — even unencrypted — would expose private metadata and message structures to
>   every relay indexing the firehose. *[cite: Wikipedia +1]*
> - **Centralized Chat Service (`chat.bsky.convo`):** Native DMs use a dedicated service
>   endpoint (`chat.bsky.convo.*`) operated directly by Bluesky Social. *[cite: Wikipedia]*
> - **Proxying via PDS:** When you send a DM, your client authenticates with your PDS. Your PDS
>   proxies the request via service headers to Bluesky's central chat service, which maintains
>   the relational state (conversations, read receipts, message logs) for both participants.
>   *[cite: Bluesky]*
> - **Standard Transport Security:** Security relies entirely on standard TLS/HTTPS in transit
>   and database-level access controls at rest. Bluesky moderators and system administrators
>   technically hold the keys to view conversation data if required for abuse or safety
>   investigations.
>
> Why It Isn't PFS or E2EE (Yet): PFS requires an End-to-End Encryption protocol (like Signal's
> Double Ratchet), where ephemeral keys are derived per message and discarded. Bluesky
> deliberately launched native DMs as a lightweight, unencrypted placeholder while designing a
> federated E2EE architecture. *[cite: Bluesky]* Bluesky is integrating protocols like Messaging
> Layer Security (MLS) for future E2EE native chats, alongside third-party extensions like Germ.
> **[CORPUS FLAG: this "integrating MLS" claim is the FACTCHECK-refuted overclaim — see header;
> MLS is possible-future-only per atproto's own docs.]** For now, native DMs operate like
> traditional, unencrypted database-backed REST APIs (similar to early Twitter/X DMs or Slack)
> routed through AT Protocol authorization tokens. *[cite: Metricool]*

## Thread 5 — integrating something other than Germ

**Owner:**

> Is it possible to integrate something other than germ natively in bluesky?

**Gemini:**

> Yes, absolutely. *[cite: Dating Industry Insights]* Because Bluesky is built on the open,
> extensible AT Protocol, the system architecture allows any third-party developer or service to
> build and natively integrate an alternative encrypted messaging protocol into the ecosystem.
> Germ is simply the first high-profile client implementation to utilize profile integration
> mechanisms, but it holds no exclusive monopoly. *[cite: Global Dating Insights +1]*
>
> 1. **The Open Lexicon Architecture.** Anyone can define a custom Lexicon (e.g.,
>    `com.example.chat.message`) for handling direct messaging; run your own independent
>    Messaging Delivery Service (MDS) server; clients (Bluesky web, third-party clients like
>    Blacksky, or custom apps) can consume your Lexicon to send and render messages natively.
> 2. **Standard ATProto Identity Authentication.** Any encrypted messenger can use ATProto's
>    decentralized identity layer (DIDs and PDS records) for authentication; verify that a user
>    controls @user.bsky.social using standard cryptographic signatures signed by their PDS; map
>    DIDs to public encryption keys anchored to the handle.
> 3. **Alternative Protocols You Could Implement:** Matrix (map DIDs to MXIDs; Olm/Megolm
>    E2EE); Signal Protocol / Double Ratchet (custom key server mapping pairwise pre-key bundles
>    to handles); "AT Messaging Proto (Open Standard) — an active open-source effort within the
>    ATProto community (the atmessaging-proto working group) defining a decentralized,
>    open-standard MLS framework for native E2EE" **[CORPUS FLAG: REFUTED — no such working
>    group exists; see header drift flag. Do not cite.]**; Nostr-style relays (NIP-04/NIP-17)
>    authenticating key ownership via the ATProto DID.
> 4. **Client-Level UI Integration Options:** custom app lexicons + profile link badges / action
>    buttons / App Clips-PWA popups triggered from profile pages (the Germ path) *[cite: Global
>    Dating Insights]*; or alternative ATProto clients (DeckBlue, Graysky, Blacksky) embedding
>    your messaging service into their primary inbox tab, bypassing `chat.bsky.convo` entirely.

## Thread 6 — Waypoints

**Owner:**

> What is a waypoint? What does this do?

*[Context: the owner shared a screenshot with Gemini — "Atmosphere Waypoints" from @aturi.to —
which is not in the paste.]*

**Gemini:**

> In general, a waypoint is an intermediate reference point or location marker used for
> navigation. In the context of the screenshot you shared, "Atmosphere Waypoints" (from
> @aturi.to) is an open-source software package (`@aturi.to/waypoints`) built for the AT
> Protocol ecosystem.
>
> What it does:
>
> - **App / Client Choice:** Because the AT Protocol is decentralized, the same social posts or
>   profiles exist across multiple different client applications. Waypoints provides an "Open
>   in..." prompt or client-picker interface so users can view content using whichever client
>   app they prefer.
> - **Universal Link Resolution:** It takes AT URIs (`at://...`) — the raw protocol addresses
>   for posts, profiles, or feeds — and resolves them into web links that direct users straight
>   to their chosen app interface.
> - **Developer Integration:** A drop-in library for developers building on atproto — client
>   routing, smart app recommendations, and link resolution with minimal code.

**Owner:**

> Show me an example and show me the apps

**Gemini** *(code + app list `[UNVERIFIED]` — the API shape and client URLs may be invented)*:

> Developers install the package via `npm i @aturi.to/waypoints` and use it to handle AT-URIs or
> display an "Open in..." client picker UI component:
>
> ```javascript
> import { resolveAtUri } from '@aturi.to/waypoints';
>
> // Raw AT Protocol URI pointing to a specific post
> const uri = "at://did:plc:z72i7hdynmk372xvre3231/app.bsky.feed.post/3kabc123";
>
> // Resolves the protocol address into a web URL for a specific client
> const destinationUrl = resolveAtUri(uri, { client: 'blacksky' });
> // Output: "https://blackskyweb.xyz/profile/did:plc:z72i7hdynmk372xvre3231/post/3kabc123"
> ```
>
> ```javascript
> import { WaypointPicker } from '@aturi.to/waypoints/react';
>
> function PostLink({ atUri }) {
>   return (
>     <WaypointPicker uri={atUri}>
>       <button>Open post in preferred app...</button>
>     </WaypointPicker>
>   );
> }
> ```
>
> Supported Apps & Clients — Social & Microblogging: Bluesky (bsky.app), Blacksky
> (blackskyweb.xyz), Anisota (anisota.net), Red Dwarf (reddwarf.app), Bluepy (bluepy.social).
> Blogging & Publishing: Leaflet (leaflet.pub), Standard Reader (standard-reader.app).
> Specialized Frontends: Catsky & Witchsky — niche, community-themed web clients.

*(End of paste.)*
