# Discord and Matrix Group Chat: Feature & UX Comparison Against Our Stack

author: research agent

date: 2026-06-13

status: draft for review

---

## Executive summary

Discord and Matrix bracket the design space our stack sits inside. Discord is the richness benchmark: a centralized, non-E2EE platform that defines what users now consider a normal group experience (servers, channels, roles, presence, voice, stage). Matrix is the architectural cousin: a federated, open-protocol system with optional E2EE that has shipped decentralized encrypted group chat in production and paid for the hard lessons we are about to face.

The central finding is the one the brief predicted, and it holds up under investigation. The features that feel effortless in a centralized app are effortless *because* a trusted server can read message content: server-side search, link-preview generation, instant presence fan-out, server-side moderation. Every one of those is costly or impossible under a cryptographically blind broker. Conversely, the privacy-preserving behaviors are the ones our architecture gives us close to free: forward-only history on join maps almost exactly onto MLS epoch semantics, and content-addressed encrypted media maps onto our blob path.

The single most valuable body of transferable knowledge is Matrix's encrypted-group-chat operational experience. Their "unable to decrypt this message" (UTD) failure mode, device-verification and cross-signing friction, and the awkwardness of giving a newly added device access to old history are not edge cases. They are the dominant UX complaints of E2EE Matrix, and our MLS-plus-multi-device design will hit the same wall unless we plan for it. Those lessons are extracted in detail in the synthesis.

A note on asymmetry, as instructed: the Matrix material below is richer and more directly transferable because the protocol is public and the design rationale is published. Discord material is necessarily more behavioral (observed limits, support docs) because the system is closed. The matrix reflects that.

---

## Discord: feature investigation

### Structure & scale

Discord uses a two-level hierarchy: a *server* (internally a "guild") contains *channels*, and channels are grouped under *categories*. Channels are typed: text, voice, announcement, forum, stage.

New servers default to a 500-channel cap (text and voice combined) and a 250,000-member cap. The documented limits are 250 roles, 500 channels, and 250,000 members, with the member limit being a default that servers must contact Discord to raise. Once a server is within 10,000 members of the 250,000 limit, the owner can request a cap increase, and servers with around 5,000 concurrently online may hit "Server Unavailable" errors and need an upgrade.

Sub-structure: categories group channels; threads hang off a parent text channel. Discord allows up to 1,000 active threads per server including public and private, with no limit on archived threads, and auto-archives threads after 1, 3, or 7 days of inactivity. Threads come in public and private variants. Public threads are visible to all members with access to the parent channel, while private threads are accessible only to invited users or admins.

### Membership & roles

This is Discord's richest surface and the benchmark for our roles analysis. Permissions are a bitmask of discrete nodes evaluated at server level, then overridden per category and per channel. As of 2025 there are 47 permission nodes, and roles are meant for broad management rather than fine-detail per-permission assignment.

Resolution order matters and is worth internalizing because it is a real policy engine, not a flat list. Discord evaluates explicit per-channel allow/deny overrides for a role first, and if all of a user's roles are neutral at the channel and category level, it falls back to the @everyone permissions for that channel. The Administrator permission grants every permission and bypasses all channel overrides, so it is recommended only for owners. Private channels are built by denying View Channel to @everyone and allowing it for specific roles. Making a channel private is done by denying View Channel for @everyone and allowing it for the roles that should have access, which can also be applied at the category level.

Roles are capped and the cap is firm. Discord enforces a 250-role limit per server and caps role names at 100 characters, though managed roles such as bot or booster roles can push the count past 250. Invite flows support expiring links, max-use limits, and (for Community servers) membership-screening approval gates. [UNVERIFIED: exact current default expiry windows for invite links.]

History visibility on join: this is the headline question, and Discord's answer is the simple one. A new member who can see a channel can scroll its **entire** history. There is no forward-only mode at the channel level; visibility is binary and governed by the View Channel permission, not by join time. This is a direct consequence of being non-E2EE with server-stored history, and it is the cleanest illustration of what centralization buys.

### Identity & profile

Discord supports a global identity plus per-server profiles: per-server nickname, per-server avatar, and per-server profile are available (the latter two gated behind Nitro for users). Servers have custom icons and, at boost tiers, custom invite vanity URLs. Identity is account-centric and non-portable: there is no federation, so identity lives entirely on Discord's servers.

### Presence & activity

Discord is presence-heavy by design: online/idle/do-not-disturb/offline, custom status text, "Playing"/rich-presence activity, typing indicators, and (in the client) read state per channel. This is cheap for Discord precisely because a central server sees every connection and every read, and fans the state out. These are exactly the metadata-heavy signals that are expensive under a blind broker.

### Message content & media

Formatting (Markdown subset), reactions, replies, per-message threads, @-mentions and role pings, and pinned messages are all present. Images, video, files, and voice messages are supported with inline previews and thumbnails. Default upload is 8 MB. The default file size limit is 8 MB, raisable to 100 MB via compressed archives, with higher limits at boost tiers.

Link unfurling: Discord generates preview cards server-side. Because content is not E2EE, the server can fetch the URL and build the card with no privacy tension. This is the contrast case for the Matrix encryption tension below.

Voice/video and stage: this is Discord's signature strength. Persistent voice channels, video, screen share, and Stage channels for one-to-many broadcast with a speaker/audience split. Real-time media runs over Discord's own infrastructure (SFU/voice servers).

### Message lifecycle

Edit and delete (self), moderator delete (everyone), server-side full-text search across channels, and message forwarding (a more recent addition). There is no native disappearing-messages/retention feature comparable to Signal's; retention is effectively unbounded unless a bot prunes. Server-side search is the key item: it is trivial for Discord and impossible for a blind broker.

### Encryption & decentralization

Discord is **not** E2EE for text/group messaging. Message content is readable server-side, which is what enables server-side search, link previews, and centralized moderation. As of 2024 Discord introduced E2EE for *voice and video* (DAVE protocol), but text/group chat remains server-readable. [UNVERIFIED: whether any text E2EE is on Discord's roadmap as of mid-2026.] The takeaway for us: Discord's feature simplicity in search/preview/moderation is a direct dividend of *not* being end-to-end encrypted.

### Platform & maturity

Native clients on web, desktop (Windows/macOS/Linux), iOS, and Android with high parity. No self-hosting; no open protocol; bot ecosystem via the public API. Widely praised for polish and voice quality; criticized for being a walled garden with no data portability.

---

## Matrix: feature investigation

For Matrix, protocol-level capability and leading-client (Element) behavior are distinguished throughout, and Olm/Megolm-today is separated from MLS-in-progress.

### Structure & scale

The protocol primitive is the *room*: a replicated event DAG (directed acyclic graph) synchronized across every homeserver with a participating user. Clients communicate by emitting events in a virtual room, room data is replicated across all homeservers whose users participate, and no single homeserver owns a given room.

Hierarchy is layered on top via *Spaces*, which are themselves rooms of a special type that reference child rooms. So Discord's server→channel nesting is approximated by a Space containing rooms, and Spaces can nest. Spaces are subject to the same access mechanisms as rooms. This is a protocol-level capability; Element implements the Space UI. There is no hard member cap in the protocol; practical scale limits come from the state-resolution cost of large rooms across federation rather than a fixed number. [UNVERIFIED: current practical large-room ceiling on matrix.org; historically very large rooms have had join/state-res performance issues.]

### Membership & roles

Matrix uses *power levels*: an integer per user, with integer thresholds required to perform actions (send a given event type, kick, ban, redact, change state). This is coarser than Discord's per-channel bitmask. There is no native per-channel permission matrix within a single room, because the room *is* the permissioning unit; the Spaces-of-rooms pattern is how Matrix gets channel-like access segmentation. Multiple admins are supported (multiple users at high power levels).

Join rules support public, invite, knock (request to join), and restricted (join allowed if you are in a designated other room/Space). Invites, kicks, bans, and re-joins are all membership state events.

History visibility on join is the headline question, and Matrix's answer is unusually precise and directly relevant to us. The setting `m.room.history_visibility` has four values. The options are: invited (you see messages from when you were invited), shared (you see history up to when this setting was set, applied non-retroactively), and world_readable (anyone can read without joining). The fourth is `joined`. Under joined visibility people need to join the room to see history and will only see new messages since they joined, mimicking IRC behavior, so channel history is only ever visible to users currently in the channel. A critical subtlety for us: under `shared`, history sharing is **not** retroactive to the setting change but **is** retroactive across a member's join. The spec states that for shared visibility, previous events are always accessible to newly joined members, including events sent when the member was not part of the room. Clients can also constrain which options they offer. As of Matrix v1.14, clients can choose which history-visibility options they offer to users when creating rooms.

The lesson: Matrix lets a room choose forward-only (`joined`) vs. backfill-on-join (`shared`), and the long-running community debate over the `shared` default leaking prior private conversation to new invitees is precisely the design question our MLS epochs answer by default.

### Identity & profile

Matrix identity is a user ID bound to a homeserver (`@user:server`). Per-room display names and avatars are supported. Matrix allows users to set their display names to be different things in different rooms. Identity portability across homeservers is historically weak (your ID is tied to your server), which is a known limitation and an area of active work; this is exactly the gap our DID-based portable identity is meant to close.

### Presence & activity

Presence (online/offline/unavailable), typing notifications, and read receipts/markers exist at the protocol level. Presence is widely considered expensive and is frequently disabled on large homeservers for performance reasons. "Public" rooms with respect to presence are defined through their join rule. The relevant point for us: even a federated-but-not-blind system finds presence costly enough to disable, which foreshadows how hard it is under a blind broker.

### Message content & media

Formatting (Markdown/HTML subset), reactions (via `m.annotation` relations), replies and threads (via relations), mentions, and pinned events are supported. Media (images, video, files, voice messages) is uploaded to the homeserver's content repository; in encrypted rooms the file is client-side encrypted and the keys travel in the event.

Link previews are the crucial encryption-tension case, and Matrix's handling directly informs our blind-broker decision. URL previews are generated server-side, and are therefore generally disabled in encrypted rooms to avoid leaking message content to the homeserver. The mechanism matters: because the homeserver builds the card, in an E2EE room it would have to see the URL (i.e. the plaintext) to do so, so Matrix disables it. There has been a long-standing request to let senders suppress previews rather than only recipients. URL preview behavior is currently defined by message recipients rather than senders, and if a sender wants to post a URL without generating a backend HTTP request there is no way to control that. For us this implies the choice is binary: either the client fetches the preview (leaking the user's IP to the target site, but not to the broker) or no preview.

### Message lifecycle

Edits (replacement relations), redactions (delete), and search exist, but search is the tell. In encrypted rooms the homeserver cannot index plaintext, so search must be client-side over locally decrypted events. Element implements this with a local index (Seshat); it is inherently per-device and limited to what that device has decrypted and stored. This is the same constraint our blind broker forces, so Matrix's client-side-search approach is a working precedent.

### Encryption & decentralization specifics

Today's production E2EE is Olm (1:1 key agreement) plus Megolm (group ratchet). The Megolm design is the key thing to understand because our MLS epochs solve the same problem differently. Message events are encrypted with Megolm, a combination of AES with a ratchet that ticks forward one step per message; a recipient given the key at a ratchet position can decrypt all future messages but not older ones. Keys rotate on membership change and on age/volume thresholds. When someone leaves the room a new key is created, and clients also rotate after about 7 days or 100 messages, so compromising one key does not expose all messages. Megolm keys are distributed via Olm to each device. Megolm keys are sent Olm-encrypted directly between clients, delivered to a single device and decryptable only by that device, and a device missing a key can request it from other devices.

Device trust uses cross-signing on top of per-device verification. Cross-signing establishes a key hierarchy so that once one device is verified, trust extends automatically to all cross-signed devices, while interactive device verification provides the initial trust anchor. The cross-signing hierarchy is three keys (master, self-signing, user-signing) with private parts held in Secure Secret Storage (SSSS).

Encrypted-history key sharing to new devices and key backup/recovery is where the UX cost concentrates. Secure Message Recovery incrementally encrypts and backs up message keys to the user's homeserver, kept secure because the homeserver never sees the passphrase or key, and a recovery passphrase or key restores history on a new device. The failure mode is severe and well documented. If a user has encrypted messages, is logged into only one session, and logs out without having set up a Security Phrase or Recovery Key, they lose access to those messages. The "unable to decrypt" problem is the recurring operational scar: it occurs when a device lacks the Megolm session for an event, whether because the key was never shared to that device, the sender's device was unknown/unverified, or session state broke. A bot or new device that is never verified does not get session keys shared with it because the sender's client refuses to share to an unverified device, producing "unable to decrypt" errors.

MLS status: not in production for Matrix as of this writing. Matrix has been working on integrating MLS since 2020, the main task being to make MLS operate in a decentralized environment, and much of the acceleration comes from a BWI/Element commercial arrangement to build "Matrix over MLS" for the German Armed Forces. For the IETF MIMI interoperability effort it is considered desirable to adopt MLS instead of Megolm, with bookkeeping changes needed to support MLS in a decentralized environment ("Decentralised MLS"). So Matrix today = Megolm; MLS = in development. We are starting where they are trying to get to, which is an advantage and a warning: the "MLS in a decentralized setting" bookkeeping problem they are still solving is one we own from day one.

### Multi-device & continuity

Multi-device is native: a user has multiple devices, each with its own keys, tied together by cross-signing. History sync across devices depends on key backup (above). Client parity is good via Element on web/desktop/mobile, plus many third-party clients. Self-hosting is a first-class concept: anyone can run a homeserver (Synapse, Dendrite, Conduit). Account/key recovery on device loss is the painful path, gated entirely on whether the user set up recovery in advance.

### Moderation & safety

Moderation is per-room (power levels, redaction, bans) plus server-level tooling and policy lists (e.g. moderation bots, server ACLs). E2EE complicates moderation because servers cannot scan content; reporting relies on the reporting client surfacing decrypted content. This is the same tension our blind broker creates: moderation must happen at the client/membership layer, not the transport.

### Platform & maturity

Mature open protocol with a published spec, multiple homeserver and client implementations, and a federation model. Praised for openness, self-hostability, and genuine E2EE. Criticized for the E2EE UX (UTD errors, verification friction, recovery complexity) and for large-room performance over federation.

---

## Feature matrix: Discord vs Matrix vs Our Stack

Fit ratings are my analysis, tied to named layers (MLS epochs, Automerge CRDT/snapshots, iroh transport, blind broker, DID identity, blob path). A question mark means lower confidence with reasoning given.

| Feature / use case | Discord | Matrix | Our Stack: Fit |
|---|---|---|---|
| Forward-only history on join | No (full scrollback if channel visible) | Yes (`joined` visibility) | **Natural** — MLS epoch keys gate decryption to membership tenure; new member bootstraps from an Automerge snapshot at the join epoch. |
| Full searchable backfill on join | Yes (server-stored, server-searched) | Yes (`shared`/`world_readable`) | **Hard** — sharing pre-join epoch keys fights MLS forward secrecy; and a blind broker cannot index, so any search is client-side over locally decrypted CRDT state. |
| Server/Space → channel hierarchy | Native (guild→category→channel) | Spaces-of-rooms (protocol) | **Effortful** — model as nested MLS groups or sub-documents; each channel is its own epoch domain, so cross-channel membership needs a policy layer above MLS. |
| Rich roles, granular per-channel perms | Native (47 nodes, per-channel overrides) | Coarser (power levels) | **Effortful** — capability/delegation layer over MLS membership; admin actions become ordered membership/permission commits in Automerge, needing conflict-free ordering and a policy schema. |
| Multiple admins | Yes | Yes (power levels) | **Natural** — multiple high-capability members expressed as CRDT-recorded grants; concurrent admin actions merge via Automerge. |
| Invite links / expiring invites / approval gates | Yes | Yes (invite/knock/restricted) | **Effortful** — invites are membership commits; expiry/approval is policy state in the CRDT; DID identity makes the invitee addressable without a central directory. |
| Per-group nickname & avatar | Yes (per-server) | Yes (per-room) | **Natural** — profile is CRDT state scoped to the group document; no server needed. |
| Portable identity across servers | No (account-locked) | Weak (homeserver-locked) | **Natural / differentiator** — AT Protocol DIDs decouple identity from any broker; this is a place we beat both. |
| Presence (online/idle/custom) | Native, cheap | Exists, often disabled for cost | **Hard** — low-latency ephemeral signaling between possibly-asleep peers; leaks metadata a blind broker shouldn't see; works over iroh realtime channels only when peers are connected. |
| Typing indicators | Yes | Yes | **Hard** — same as presence: ephemeral, metadata-leaking, requires live peer connectivity. |
| Read receipts / markers | Yes | Yes | **Hard** — per-message per-member metadata; either leaks to broker or requires direct peer exchange when co-present. |
| Reactions | Yes | Yes (annotations) | **Natural** — a reaction is a small CRDT edit on the message object; merges conflict-free. |
| Replies / threads | Yes | Yes (relations) | **Natural** — parent-reference fields in the Automerge document. |
| Pinned messages | Yes | Yes | **Natural** — pin is a CRDT flag/set on the document. |
| Edit / delete (self) | Yes | Yes (edit/redact) | **Natural** — edits are CRDT updates; "delete" is a tombstone edit (true erasure from peers' local copies is best-effort, same as everyone else). |
| Images / video / files | Yes (8 MB default) | Yes (client-encrypted in E2EE) | **Natural** — encrypted content-addressed blobs over iroh-blobs, referenced by hash from the document. |
| Inline thumbnails / previews of media | Yes (server-generated) | Client-side in E2EE | **Effortful** — thumbnail must be generated client-side by sender and shipped as its own blob; broker stays blind. |
| Link preview cards | Yes (server fetches) | **Disabled by default in E2EE** | **Hard / tension** — broker cannot fetch without seeing plaintext URL; only option is client-side fetch, which leaks the user's IP to the target. See synthesis. |
| Voice / video calls | Native strength (SFU) | Supported (WebRTC/MatrixRTC) | **Hard / out-of-scope initially** — real-time media is a separate subsystem; iroh transport could carry P2P media later but is not a day-one MLS concern. |
| Stage / broadcast | Native | Partial | **Hard** — one-to-many low-latency media; defer. |
| Server-side full-text search | Native | Impossible in E2EE (client-side only) | **Hard** — blind broker cannot index; search is client-side over locally held decrypted data, same constraint Matrix's Seshat hit. |
| Multi-device with synced history | Yes (server-stored) | Yes via key backup (painful) | **Effortful / Hard?** — MLS gives each device its own leaf, but giving a new device old history means re-encrypting or backing up past epoch secrets, reintroducing exactly Matrix's key-backup problem. |
| Account/key recovery on device loss | Trivial (server holds all) | Hard (recovery key or lost history) | **Hard** — no server holds plaintext; recovery requires a user-managed key escrow or social recovery, or history is lost. Plan this early. |
| Centralized moderation / content scan | Native | Hard in E2EE | **Hard / by design** — blind broker cannot scan; moderation lives at client + membership layer. |
| Offline / local-first operation | No (needs connection) | Limited | **Natural / differentiator** — Automerge is local-first by construction; edits merge on reconnect. |
| True serverless P2P when co-present | No | No (always needs homeserver) | **Natural / differentiator** — iroh direct peer transport with the superpeer only as optional fallback. |

---

## Synthesis: roadmap implications

### Free wins (Natural — ship early)

Ship the features whose grain runs *with* the architecture, because they are nearly free:

- Forward-only history on join. This is the default behavior of MLS epochs, not a feature we build; a new member simply cannot derive keys for prior epochs, and bootstraps from an Automerge snapshot at join. Matrix had to add `joined` visibility as an option and argue about defaults; we get it for free.

- Reactions, replies, threads, pins, edits. All are small CRDT edits on the shared document and merge conflict-free. This is Automerge's home turf.

- Media via the blob path. Encrypted content-addressed blobs over iroh-blobs, referenced by hash, cover images/video/files/voice messages. Sender-generated thumbnails ship as their own blobs.

- Per-group nicknames and avatars as group-scoped CRDT state.

- Basic membership and multiple admins as CRDT-recorded grants over MLS membership.

- Portable identity. DIDs make this a strength rather than the weakness it is in both Discord (locked) and Matrix (homeserver-bound).

### Architectural tensions (users expect these; we make them Hard)

For each, the tension and realistic options:

- **Server-side search.** Users expect instant full-history search; Discord delivers it because the server reads plaintext. Our blind broker cannot. Options: (a) client-side index over locally decrypted CRDT state, per-device, like Matrix's Seshat; (b) accept that search only covers what this device has and has decrypted. There is no blind-broker option that searches plaintext. Recommend (a), set expectations that search is local and device-scoped.

- **Link previews.** Discord fetches server-side; Matrix disables in E2EE precisely because the server would have to see the URL. Our broker is blind, so it has the same block. Options: (a) client fetches the preview, leaking the user's IP and the fact-of-visiting to the target site but not the URL to the broker; (b) a user-run preview proxy (their own infra) that they trust; (c) no previews. Recommend (a) as opt-in with a clear privacy note, since it matches user expectation while keeping the broker blind.

- **Presence / typing / read receipts.** Metadata-heavy, ephemeral, and exactly what a blind broker should not see. Even Matrix disables presence at scale for mere performance reasons. Options: (a) peer-to-peer-only when co-present over iroh realtime channels, so the signal never touches the broker; (b) opt-in and off by default; (c) accept the metadata leak for groups that want it. Recommend (a)+(b): presence works when peers are directly connected and is otherwise absent.

- **Full history backfill on join.** Fights forward secrecy. Sharing old epoch keys to a new member defeats the property MLS exists to provide. Options: (a) don't (forward-only, the default); (b) explicit, logged, opt-in re-encryption of selected prior history to the new member, treated as a deliberate disclosure like Matrix's `shared` debate surfaced. Recommend (a) as default, (b) only as a conscious, visible action.

- **Multi-device history.** The honest hard one. MLS gives each device a leaf, but old history predates that leaf's epochs. Giving a new device the back catalog reintroduces Matrix's key-backup problem wholesale. Options below in the Matrix lessons.

### Lessons from Matrix specifically (the highest-value section)

Matrix has already paid for these in production. Each maps to a concrete decision for us.

- **The "unable to decrypt" failure mode is the dominant E2EE complaint, and it is a key-availability problem, not a crypto-strength problem.** It happens when a device lacks the session for an event: key never shared, sender device unknown/unverified, or session state broke. MLS helps us here because group membership *is* the key-distribution mechanism (every member of the current epoch can derive the key), which removes a whole class of Megolm's "I never received the room key" failures. But we inherit the equivalent risk at epoch boundaries and for messages sent while a device was offline. Design implication: treat "can every current member decrypt every current-epoch message" as an invariant to test continuously, and build an explicit, friendly key-request/healing path rather than showing a dead tile. Matrix's dead "Unable to decrypt this message" tile is the anti-pattern.

- **Device verification and cross-signing friction drives people away.** Matrix's interactive emoji/QR verification and the three-key cross-signing hierarchy are powerful but confusing, and unverified devices silently fail to receive keys. Design implication: lean on DID-anchored device identity so that adding a device is an identity operation, not a separate manual verification ritual per contact. Avoid the state where an unverified-but-legitimate device exists and silently cannot decrypt.

- **Key backup / recovery is a cliff, not a slope.** The documented Matrix failure is brutal: a single-session user who logs out without setting up recovery loses their encrypted history outright. There is no server with plaintext to fall back on, which is the whole point and also the whole danger. Design implication: make recovery setup a near-mandatory part of onboarding (key escrow the user controls, or social recovery across their own devices/contacts), and never let a user reach a single-device state with no recovery configured without a loud, blocking warning.

- **History-sharing to a newly added device is awkward and is where backfill and forward secrecy collide.** Matrix solves it with server-side encrypted key backup (Secure Message Recovery), which works but adds the SSSS passphrase burden and a homeserver dependency we do not have. Design implication: decide deliberately whether new devices get history at all. The cleanest stance consistent with our principles is that a new device sees history from its join epoch forward, with optional user-driven backup for those who want continuity, sized as the real cost it is rather than assumed free.

- **Presence is expensive even without a blind broker.** Matrix disabling presence for performance is a free data point that our instinct to keep it peer-to-peer-and-co-present-only is the right one.

### Differentiators (what we can do that neither does well)

- **Local-first offline operation.** Automerge merges concurrent edits on reconnect with no server arbitration. Discord needs a connection; Matrix needs a reachable homeserver. We work on a plane.

- **True serverless P2P when peers are co-present.** iroh direct transport with the superpeer as optional fallback means a group of co-present peers needs no infrastructure at all. Neither Discord nor Matrix can operate with zero server.

- **User-run infrastructure with a cryptographically blind broker.** Matrix self-hosting still trusts the homeserver with metadata and (in unencrypted rooms) content. Our broker is blind by construction, so "run your own" carries a stronger guarantee.

- **CRDT-based conflict-free concurrent edits.** Collaborative mutable state (shared docs, pinned sets, ordered lists) merges without conflict. Matrix's event DAG and Discord's server ordering are both weaker models for shared mutable state.

- **Public/private content lifecycle.** Public social content can flow through atproto while private group messaging stays on the encrypted P2P path, a split neither product expresses.

### Expectation gaps (table-stakes features that are surprisingly hard for us)

Flag these now so they are planned, not discovered late:

- Instant full-history search. Users will assume it; we can only offer local, device-scoped search. Communicate early.

- Link previews. Users expect rich cards; we can only offer client-fetched, opt-in previews with an IP-leak note.

- Read receipts and typing indicators. Ubiquitous in modern chat; we make them co-present-only or absent. Users may read silence as the feature being broken.

- "Log in on a new phone and see all my history." The single most dangerous gap, because the Discord/WhatsApp-cloud mental model assumes it. Our recovery story must be designed and onboarded, or users will lose history and blame the app.

### Prioritized roadmap

**Ship first (high value × Natural):**

- Forward-only history via MLS epochs.

- Reactions, replies, threads, pins, edits as CRDT operations.

- Encrypted media via the blob path, with sender-generated thumbnails.

- Per-group profiles and basic multi-admin membership.

- DID portable identity (a differentiator that is also foundational).

**Needs design work before committing (high value × Hard):**

- Multi-device history continuity and key recovery. Treat as a first-class workstream; this is Matrix's deepest scar.

- Client-side search index.

- Opt-in, co-present-only presence/typing over iroh realtime channels.

- Roles and per-channel permissions as a policy layer over MLS membership (the Spaces-of-groups modeling decision).

- Opt-in client-fetched link previews with explicit privacy disclosure.

**Defer / drop:**

- Voice/video and stage/broadcast (separate real-time subsystem; revisit once iroh media transport is proven).

- Server-side anything that requires plaintext (full server-side search, server-generated previews, server-side content moderation) — these conflict with the blind broker by design and should be dropped rather than worked around.

---

## Private vs. public groups, and the relationship to Bluesky / Germ / atproto

This section addresses a design question raised after the initial analysis: should the stack support both private and public group chat depending on user choice, and should it piggyback on Bluesky's newly shipped group feature.

### Two lanes, not one group with a visibility flag

"Public" and "private" are not two settings on one object. They are two different trust models, and conflating them recreates the exact footgun visible in Matrix's `shared` history-visibility default, where a quiet setting governs who can read. The clean model is three options the user routes between at compose time:

- **Private, closed membership.** An MLS group, invite-gated. Natural for our stack: membership is the ratchet tree, epoch keys gate decryption, the broker stays blind, forward-only history falls out of epoch semantics.

- **Private, open-join.** Still a real MLS group, but the approval gate is dropped and joiners are auto-admitted; each becomes a full member with forward-only history. Costs one epoch rekey per join, so it is Effortful at high churn but remains fully end-to-end encrypted.

- **Truly public / world-readable.** Not an MLS group at all. A world-readable group cannot run inside MLS, because if a non-member can read the content then the content is by definition not encrypted-to-members, which is the one property MLS exists to provide. Public content belongs on the atproto public lane (published records), not on a weakened MLS group.

The key correction to "users can decide": user choice governs the lane and the openness, but it cannot make MLS simultaneously gate content to members and expose it to non-members. The decision is routing, not a `visibility` boolean on an encrypted group.

A note on terminology, since it caused confusion in discussion: MLS is not "encryption in transit." Transit encryption (iroh's QUIC/TLS link layer) protects a hop and terminates at each end. MLS is end-to-end group encryption where the broker never sees plaintext. In-transit-only is the weaker model MLS exists to surpass, not a synonym for it.

### What Bluesky actually shipped (verified June 2026)

Bluesky's June 2026 group chats are the wrong thing to adopt as a private-messaging transport, because they are not end-to-end encrypted. Bluesky's standard DMs are not end-to-end encrypted, which means in rare cases the moderation team can access messages to investigate abuse, and Bluesky has committed to adding E2EE later. The lack of full E2EE in native DMs is described as a deliberate protocol choice to avoid complexity, per a Bluesky engineer. The group chats are an extension of that native DM layer: group chats for up to 50 users, framed as private conversations, with the creator managing access and sharing invite links, and media sharing disabled pending a moderation system. The June announcement coverage explicitly frames encryption as the earlier, separate Germ step and the group chats as the new step, with creator-managed access and no mention of E2EE. Coverage notes Bluesky added messaging in 2024 and only more recently began offering encrypted chats by integrating Germ, and "now" is adding group chats of up to 50 people with creators deciding who may participate. The native DM layer these extend is operator-visible. Messages sent through the native DM feature are visible to Bluesky PBC and can be accessed for content moderation, in response to law enforcement warrants, or exposed in a breach. So the defensible read is that v1.124 group chats are the moderatable native DM service extended to 50 people, not Germ-backed E2EE. [UNVERIFIED: the exact group-chat transport/backend; no primary source states it outright, and the inference rests on the announcement treating Germ and group chats as separate and on native DMs being operator-visible.] "Private" here means access-controlled, not cryptographically private.

The genuinely public, "shared-subscription" model the team intuited maps to Bluesky's forthcoming **communities** feature, not group chats. Communities are being built with custom handles and public/private settings, and that is the lane where the atproto "anything on the relay is public" property actually applies. Private messaging on both our design and Bluesky's stays off the public relay by construction.

### Germ is the right reference, and a plausible interop target

Germ, the third-party messenger Bluesky integrated for encryption, is effectively a shipped instance of our own thesis. Germ uses MLS and the AT Protocol, and instead of requiring a phone number it integrates with ATProto so users can chat across Bluesky and the wider open social web. Because it ties to ATProto rather than a phone number, Germ messages cannot be decrypted by any other service, including Germ or Bluesky themselves. The founders are credible for this work. Germ was co-founded by Tessa Brown, a former Stanford communications scholar, and Mark Xue, a former Apple privacy engineer who worked on FaceTime and iMessage. They also built deliberately for third-party adoption. Germ released guidelines allowing other ATProto-based clients to integrate similarly, with Blacksky quickly adding support, and much of the technology is open-sourced.

That overlap (MLS for group crypto, ATProto DIDs for identity, off the public relay) is exactly our private lane minus our transport. So the realignment of "piggyback on Bluesky's groups" is: adopt the **taxonomy** (private chats vs. public communities), align **identity** to ATProto/DID, and treat **Germ as the interop reference for the MLS layer**, while keeping iroh P2P transport as our differentiator.

### Germ integration feasibility — deeper assessment

The honest read is that **identity-layer alignment is feasible and worth pursuing; full message-layer interop is not currently buildable from public artifacts and carries a real architectural mismatch.** Breakdown:

**What is public and usable.** Germ's ATProto lexicon is open and MIT-licensed. The germ-network/lexicon repository contains the lexicon for all com.germnetwork.* collections and records. Their integration design is documented in an engineering writeup. The core mechanism is an "Anchor Key" bound to a DID: Germ binds a DID to an Anchor Key by publishing the user's current Anchor Key in the AT Protocol profile text, and the app monitors its own and others' bios for changes to their Anchor Keys. At the application layer, a valid message goes from a delegate of Alice's current Anchor Key to a delegate of Bob's current Anchor Key, making the anchor key an application-layer identity for the ATProto identity. They are candid that the bio-publishing binding is a stopgap. They note the Anchor Key could move out of the ATProto bio if there were a standardized way for PDSs to publish an E2EE key with corresponding transparency for users to review and remove unexpected values.

**What is not public.** The lexicon is schema only (record shapes), not the MLS wire protocol, ciphersuite selection, key-package distribution, or a reusable client SDK. As of June 2026, no Germ crypto library or protocol SDK could be located in public sources beyond the lexicon repo and the integration writeup; the "open-sourced" framing in coverage refers to the approach, lexicon, and integration guidelines rather than a drop-in implementation. [UNVERIFIED: a direct enumeration of the germ-network GitHub org was attempted but blocked by an unauthenticated-API rate limit, so absence of a public SDK is inferred from search, not confirmed by listing the org.] So "interoperate with Germ" today means re-implementing against their MLS usage and matching their anchor-key binding, not importing their stack.

A useful adjacent lead for our own build, independent of Germ: Wire's `core-crypto` is a production wrapper over `openmls` that is worth evaluating as a reference, and possibly as a dependency with one significant caveat. Core Crypto wraps OpenMLS to provide an ergonomic API for creating, managing, and interacting with MLS groups, usable on the web via WebAssembly and on mobile via FFI. Its internal parts are directly relevant to our stack: CoreCrypto abstracts MLS and Proteus in a unified API, CoreCryptoFFI provides bindings for iOS, Android, and WASM, the Keystore is an encrypted store powered by SQLCipher on all platforms except WASM (which uses an IndexedDB-backed AES256-GCM store), and the MLS provider is built on RustCrypto plus that keystore.

The single most important design detail for us is that core-crypto already treats transport as pluggable, which is exactly the seam our iroh path needs. It exposes an MlsTransport trait of client callbacks for communicating with the delivery service, with two endpoints, one for messages and one for commit bundles. That means the MLS engine does not assume Wire's own server; a consumer supplies the delivery mechanism, which in our case could be iroh with the blind superpeer as fallback. It also models mutating operations transactionally. All mutating operations are performed through a TransactionContext, which provides transactional support around the session. That maps cleanly onto our need to order membership and permission commits, and to reconcile them with Automerge state.

The caveat is licensing, and it is a real gate, not a footnote. The core-crypto crate is published under GPL-3.0-only. The JVM and Android bindings are likewise published under GPL-3.0. GPL-3.0 is copyleft: linking it into our client likely obligates us to release our corresponding source under compatible terms. That may be acceptable if our client is itself open source, but it is a decision to make consciously rather than discover at integration time. The alternatives are to use `openmls` directly (the Apache/MIT-licensed library core-crypto itself wraps) and build our own thin transport/keystore layer, or to evaluate `mls-rs` (AWS Labs, Apache-2.0) as a permissively licensed engine. [UNVERIFIED: exact current `openmls` and `mls-rs` license terms and their fit with our `openmls` baseline; confirm before committing.]

The takeaway: core-crypto is the best available proof that an `openmls`-based, transport-agnostic, multi-platform MLS layer with an encrypted keystore is viable, and its `MlsTransport` design is a strong reference for how to keep our iroh transport decoupled from the crypto. Whether we depend on it or reimplement against `openmls` directly turns on the GPL question.

**The architectural mismatch that matters.** Germ delivers over the ATProto/PDS-mediated path; our stack specifies MLS over iroh P2P with an optional blind superpeer. MLS is explicitly transport-agnostic, so the crypto can be shared in principle, but the delivery and metadata models differ. In the current atproto architecture almost all client requests go to the account's PDS and are proxied onward. A PDS-mediated path means the PDS is a metadata-bearing intermediary, which is a weaker position than our cryptographically blind broker. Adopting Germ's transport would reintroduce a metadata-visible hop we designed out.

**The weakest seam to scrutinize: the anchor-key binding.** Publishing the E2EE identity key in mutable profile bio text means whoever controls the ATProto account (or its PDS) can swap the anchor key, which is a masquerade vector Germ itself calls out. They note the risk of someone with temporary access to a user's ATProto account masquerading in E2EE conversations, and that users must monitor the binding to detect and react to anchor-key changes. Our DID-based identity should bind device/MLS credentials more tightly than mutable bio text (for example, via DID-document verification methods or signed key records), and should treat any key rotation as a security-boundary event surfaced to members rather than a silent bio edit. This is the place to improve on Germ, not copy it.

### Recommendation

- Implement the three-lane model (closed MLS, open-join MLS, public atproto), routed at compose time, never a visibility flag on an encrypted group.

- Align identity to ATProto DIDs so a private MLS group and a public atproto presence share one identity without sharing one message path. This is the real, low-risk piggyback.

- Treat Germ as a design reference and a possible future identity-layer interop target (anchor-key-style binding, but hardened via the DID document rather than profile bio). Track whether Germ publishes a fuller protocol SDK; revisit message-layer interop if it does.

- Do not route private messaging through Bluesky group chats (non-E2EE) or adopt a PDS-mediated transport that defeats the blind-broker guarantee. Keep MLS-over-iroh as the private transport.

- Borrow Bluesky's invite UX where it is genuinely good: invite-link-as-embedded-card, and the "who can invite me" control (everyone / only people I follow / no one). Bluesky lets users set who is allowed to invite them: everyone, only people they follow, or no one.

---

## Sources

- Matrix history visibility options and semantics: Matrix.org blog "What happened with archive.matrix.org"; Synapse admin API docs; matrix-spec issue #587; Synapse issue #13968; Matrix v1.14 release notes.

- Matrix E2EE (Olm/Megolm, ratchet, key rotation, key sharing): blog.neko.dev "Unable to Decrypt a Message on Matrix"; Matrix.org End-to-End Encryption implementation guide; hermes-agent issues #3521/#6174 (UTD in practice).

- Matrix cross-signing, SSSS, recovery, and UX: matrix-ios-sdk DeepWiki cross-signing page; Matrix.org "Cross-signing and E2EE by Default is HERE"; Matrix.org "User Experience Preview: End-to-end encryption"; FU-Berlin Matrix encryption wiki (device-loss failure mode).

- Matrix MLS/MIMI status: Matrix.org "A giant leap forwards for encryption with MLS"; IETF draft-ralston-mimi-matrix-message-format; TechPolicy.Press MLS/MIMI playbook; IETF MLS publication blog.

- Matrix URL previews under E2EE: Synapse 1.45.1 release notes; matrix-spec issue #1588; Synapse issue #2497.

- Matrix Spaces and rooms model: spec.matrix.org latest; matrix-spec releases (Spaces access mechanisms).

- Discord limits, roles, permissions, structure: Discord support community posts on channel and role limits; metacrm.inc channel-limit guide; Best Friends Club Discord limits article; Oreate AI Discord limits blog; cybrancee "Managing Permissions In Discord"; devvyy.xyz 2025 Discord role permissions guide.

- Bluesky group chats (June 2026): TechCrunch "Bluesky launches group chats"; News9live; TheNextWeb (communities pivot); Thurrott; KuCoin.

- Bluesky native DMs not E2EE / Germ integration: metricool "Bluesky DMs: A Short Guide"; TechCrunch "Germ brings end-to-end encrypted messages to Bluesky"; CB Insights Germ profile; Sovereign Magazine; Global Dating Insights.

- Germ technical design: Germ Network engineering blog "Integrating Germ with AT Protocol" (Anchor Key / DID binding); germ-network/lexicon GitHub repo (MIT, schema only); Germ DM App Store listing (MLS).

- atproto architecture (PDS-mediated, relay-is-public): atproto.com/specs/atp; bluesky-social/atproto PDS discussion #2350.

- Wire core-crypto (openmls wrapper, MlsTransport trait, keystore, GPL-3.0): wireapp.github.io/core-crypto rustdoc; wireapp/core-crypto crypto/Cargo.toml (license = GPL-3.0-only); npm @wireapp/core-crypto; Maven Central com.wire/core-crypto-jvm and core-crypto-android.

- [UNVERIFIED] items noted inline: current Discord invite-link default expiry; whether Discord plans text E2EE; current practical large-room ceiling on matrix.org; exact Bluesky v1.124 group-chat backend; whether a full Germ protocol/crypto SDK exists publicly beyond the lexicon.
