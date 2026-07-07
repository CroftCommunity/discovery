# fenced / group-scale-versus-e2ee: the capability map (roster, calls, broadcast, and E2EE stance)

date: 2026-07-07 · register: descriptive / quantitative (the map, no argument)

**What this doc is.** The capability map of the fenced field across 14 centered commercial platforms:
how large each platform's text roster, mutual voice/video call, and broadcast object can grow; where each
one does and does not end-to-end encrypt, broken out by *surface* (1-to-1 vs group vs broadcast) and by
*layer* (text vs real-time voice/video); and the two forces that make roster size and group-text E2EE
trade off against each other. The headline the map draws out: on consumer platforms, the ability to
end-to-end encrypt a group and the ability to make that group large are in direct tension, and every
platform resolves it by giving up one. That tension is causal, not coincidental, and it is driven by two
distinct forces documented below. Every figure carries a source tier (T1 primary / T2 peer-reviewed or
direct reporting of a primary announcement / T3 secondary corroboration only); caps change, so re-verify
any single number before it enters an SLA.

## Scope

In scope: the roster / mutual-call / broadcast ceilings; the E2EE stance per surface and per layer; and
Force 1 (the key-agreement cost curve) and Force 2 (server-mediated core function) that couple the two.
Inclusive of the messaging apps (Signal, WhatsApp, iMessage, Facebook Messenger, LINE, WeChat), the
community/social platforms (Discord, Telegram, Matrix/Element, X, Reddit, Bluesky), and the enterprise
tools (Slack, Microsoft Teams).

Out of scope, and where it lives:

- Per-group operational rates (member-ban, member-join, live-fraction) and platform monetization live in
  the sibling doc `operational-rates-and-platform-economics.md`.

- The harm reading of this map (what platform power and centered authority mean for community labor, and
  what to do about it) is `activism/`, a different register. This doc makes no harm argument.

- The composable open field Drystone is built *among* (MLS, atproto, and kin) is `cairn/`. Here those
  same technologies appear only as the measuring stick against the centered platforms.

## The three layers, not one number

"Group size" is not a single quantity. Product and spec caps split into three separately-enforced limits,
set by three different constraints, and they can differ by three-plus orders of magnitude on the same
platform:

- **Roster.** Accounts that have joined a persistent text space. Bounded by storage and fan-out. This is
  the big number, and it is the one usually quoted.

- **Concurrent-online.** How many of those accounts can be connected at once. Bounded by connection
  handling. Far below roster.

- **Call-concurrency.** How many can share a live mutual voice/video call. Bounded by real-time media
  mixing. Two to three orders of magnitude below roster.

A platform's roster cap says almost nothing about how many are online at once, and nothing at all about
how many can share a call. Read every cap below with this split in mind.

## The comprehensive comparison (all platforms, all dimensions)

Figures are current as of mid-2026 and tagged by source tier where it matters. "E2EE group" means a
group whose text content the provider cannot read.

| Platform | Category | Largest text group (roster) | Largest mutual voice/video call | Largest E2EE group (text) | Group text E2EE by default? | Notable dimensions |
|---|---|---|---|---|---|---|
| **Signal** | Messaging | 1,000 (T1) | 40 (call) (T1) | 1,000 (all groups) | Yes, always | No non-E2EE mode; server holds no group metadata (private group system); pairwise-channel model, hence small cap |
| **WhatsApp** | Messaging | 1,024 (T2) | 32 (call) | 1,024 (all groups) | Yes, always | Signal-protocol sender keys; Communities + Channels (broadcast, likely not E2EE, low-confidence); group membership not cryptographically bound, server can inject a member (Albrecht/Dowling/Jones, KCL, IEEE S&P) (T1/2) |
| **iMessage** | Messaging | 32 (group) (T1) | 32 (FaceTime) (T1) | 32 (all iMessage-to-iMessage) | Yes, between Apple devices | E2EE only Apple-to-Apple; mixed SMS/RCS group falls back to 20-cap, not E2EE; caps verified at Apple Support |
| **Facebook Messenger** | Messaging | ~250 (group) | ~32 (call) | ~250 (group, E2EE) | Yes (default since Dec 2023) (T1) | Signal protocol + Meta's Labyrinth protocol; group calls add SFrame over SFU; server-side key recovery via PIN |
| **LINE** | Messaging | 500 (group) | ~500 (call, varies) | 50 (E2EE only for groups up to 50) | Yes for text 1-to-1 and groups up to 50 (default, non-disable-able since 2021) (T1) | Letter Sealing default and non-disable-able since 2021, but text E2EE caps at 50-member groups even though groups go to 500; group calls/video/Meeting transport-only, not E2EE |
| **WeChat** | Messaging | 500 (group) (T1) | ~9 (video) | None | No | Transport/at-rest AES-256 only, not E2EE; retains text ~72h, media ~120h on servers |
| **Discord** | Messaging/community | 250,000 (server; up to 1M) (T1) | 99 (voice channel) (T1) | None (text); calls E2EE via DAVE/MLS | No (text never E2EE, by design) | E2EE only on the ≤99 call layer via MLS (DAVE); text non-E2EE for moderation; Stage channels excluded |
| **Telegram** | Messaging/community | 200,000 (supergroup); channels unlimited (T1) | 200 (group call) (T1); ~2,000 livestream (indicative) (T3) | 0 (only opt-in 1-to-1 Secret Chat is E2EE) | No | Default is server-readable Cloud Chat; only 1-to-1 opt-in Secret Chat is E2EE; gigagroups remove the 200k cap for broadcast |
| **Matrix / Element** | Federated messaging | No hard cap (thousands; large rooms strain) | Element Call, tens (SFU-based) | Same as room size (Megolm), large but costly | Yes (default for new private rooms since 2020) (T1/2) | Federated/self-hostable; Megolm per-room-per-device ratchet; large-room send slow due to per-device key distribution |
| **X (Twitter)** | Social + messaging | Group Chat (claimed E2EE, disputed) | Built into Chat | Claimed for groups, disputed | Claimed, widely disputed | XChat: keys stored server-side via Juicebox, no forward secrecy, no MITM protection per X's own help page; cryptographers call it not true E2EE; Communities are public |
| **Reddit** | Social/forum | No cap (subscribe model; tens of millions) | n/a | None | No (public forum, cannot be) | Subreddits are public ranked/searchable forums; Chat/DMs and Chat Channels not E2EE; the extreme Force-2 endpoint |
| **Bluesky** | Social/forum | No cap (public follow model) | n/a | None natively (E2EE via external MLS apps: Germ, XMTP) | No (public-by-design AT Protocol) | AT Protocol built for public discourse; DMs not E2EE, moderation can access; E2EE bolted on via MLS-based third parties |
| **Slack** | Enterprise | Workspace/channel: org-scale | Huddles, up to 50 (paid) | None | No | Enterprise Key Management is customer-held keys in AWS KMS, still not E2EE; Slack states search cannot compile encrypted data (Force 2) |
| **Microsoft Teams** | Enterprise | Team/channel: org-scale (10k+) | Meetings to ~1,000 interactive | 1-to-1 calls + configured meetings only; never chat | No (group text/chat never E2EE) | Optional E2EE covers only audio/video/screen-share, not chat; E2EE disables eDiscovery, DLP, translation, recording (Force 2, per Microsoft) |

Category note: the pattern holds across all three categories. Messaging apps that keep E2EE cap near
1,000 (Signal, WhatsApp, iMessage, Messenger); community/social platforms that go large drop group-text
E2EE entirely (Discord, Telegram, Reddit, Bluesky, X); enterprise tools drop it for compliance/search
reasons even at moderate scale (Slack, Teams). Matrix is the interesting middle: it keeps E2EE and allows
large rooms, but pays the Force-1 cost in large-room latency, which is precisely the cost MLS is designed
to remove.

## The layered fact

"Is it E2EE?" has no single answer per platform, because encryption status varies by *surface* (1-to-1 vs
group vs broadcast) and by *layer* (text vs real-time voice/video). A claim true of one surface is usually
false of another.

| Platform | Group text roster cap | 1-to-1 text E2EE | Group text E2EE | Group voice/video E2EE | Broadcast object E2EE |
|---|---|---|---|---|---|
| **Signal** | 1,000 | Yes (always) | Yes (always) | Yes (calls to 40) | n/a |
| **WhatsApp** | 1,024 | Yes (Signal protocol) | Yes (sender keys) | Yes (calls to 32) | Channels: not E2EE (low-confidence) |
| **Discord** | 250,000 | No (text never E2EE) | No (by design, for moderation) | Yes (DAVE, calls to 99; excludes Stage) | Stage channels: not E2EE |
| **Telegram** | 200,000 (supergroup) | Only in opt-in Secret Chat | No (any size) | No (group voice chats) | Channels/gigagroups: not E2EE |
| **Reddit** | No cap (tens of millions) | No (Reddit Chat/DMs) | No (subreddits public; chat channels not E2EE) | n/a | The subreddit itself is the broadcast surface: public, not E2EE |

## Why the inverse relationship holds (the two forces)

**Force 1, the key-agreement cost curve.** Traditional group E2EE (Signal's pairwise channels, WhatsApp's
sender keys) has setup cost that grows with membership, so the moment a platform wants a very large roster
it must either abandon group E2EE or confine E2EE to a naturally-bounded layer. This is the force that
caps the E2EE-first platforms near 1,000 (Signal, WhatsApp) and that pushed Discord to E2EE only the small
real-time call layer (DAVE on the ≤99 voice channel, not the 250,000 text roster). MLS (RFC 9420) is the
standards-body answer to exactly this force: tree-based key agreement with logarithmic rather than linear
cost. One survey paper (arXiv 2401.09102, T2) puts a number on the pre-MLS cost: naive pairwise E2EE in an
N-member group needs on the order of N-squared key-exchange messages, so a 500-member group needs at least
250,000, and E2EE in large groups works best when a minority actively transmits and the majority passively
receives. That last point rhymes with the live-fraction skew of large communities: the same skew that
makes large groups mostly-lurkers is what makes sender-oriented E2EE tractable at all.

**Force 2, server-mediated core function.** Independent of per-member cost, the more a platform's core
function depends on the server reading content (ranking/sorting, full-text search, discovery and
recommendation, sitewide moderation, ad targeting, one-to-many broadcast), the less E2EE is even possible,
because E2EE by definition denies the server the plaintext those functions consume. This is the force that
makes Reddit subreddits non-E2EE by construction and Telegram channels/gigagroups non-E2EE. MLS alone
would not make them encryptable: you cannot tree-ratchet your way to an encrypted feed the server still
has to rank and search.

**The two forces stack.** Discord text is non-E2EE for Force 2 (moderation needs plaintext) even though
its call layer solved Force 1 with MLS. Reddit is non-E2EE for Force 2 at every layer and never even
reaches Force 1, which is why it can carry tens-of-millions-member "communities": there are no per-member
keys to cost anything because the server reads everything. The design implication for a system that wants
large and encrypted at once: use MLS (or an equivalent log-cost group key agreement) to defeat Force 1,
and avoid any feature that requires the server to read content to defeat Force 2. Giving up server-side
ranking, search, and discovery is the price of E2EE at scale, and it is a product decision, not a
cryptographic one.

## Per-platform detail (condensed)

- **Signal:** every surface E2EE, no non-E2EE mode; "private group system" so the service has no record of
  memberships, titles, avatars, or attributes; limit 1,000; group calls 40; the pairwise per-pair channel
  model explains the small cap (T1).

- **WhatsApp:** group contents encrypted using the Signal Protocol; roster scaled 256 → 512 → 1,024. Two
  caveats. (a) The King's College London formal analysis (Albrecht, Dowling & Jones, "Formal Analysis of
  Multi-device Group Messaging in WhatsApp," IEEE S&P / Springer, T1/2) proved message payloads remain
  E2EE but clients trust the server to supply the group member list, so membership changes carry no
  cryptographic binding and the server can inject a member; the computational companion "WhatsUpp with
  Sender Keys?" (Balbás/Collins/Gajland, IACR ePrint 2023/1385, T1) states an adversary with server-level
  control can add users without any member's authorization, tracing to the 2018 Rösler/Mainka/Schwenk
  "More is Less" result (T2). (b) Whether WhatsApp Channels are non-E2EE could not be confirmed at a
  primary source; low-confidence.

- **iMessage:** E2EE only Apple-to-Apple; group and FaceTime caps 32 (Apple Support, T1); a mixed
  SMS/RCS group falls back to a 20-cap and is not E2EE.

- **Facebook Messenger:** default E2EE since Dec 2023, Signal Protocol + Meta's Labyrinth; group text via
  Signal "Sender Keys" (GroupCipher); group calls add an SFrame layer because SRTP through an SFU is not
  end-to-end. Shares WhatsApp's server-side key recovery via PIN (T1).

- **LINE:** Letter Sealing is default and non-disable-able since 2021, but text E2EE caps at 50-member
  groups even though groups themselves go to 500; group calls, video, and Meeting are transport-only, not
  E2EE (T1). This was a correction on cap-pinning, not just a citation.

- **WeChat:** 500-member groups, no E2EE; AES-256 in transit and at rest only, retaining text ~72h and
  media ~120h on servers (T1).

- **Discord:** deliberately does not E2EE text so it can moderate; audio/video E2EE via DAVE, built on MLS
  (media session members undergo an MLS group key exchange, the voice gateway is the MLS
  delivery/authentication service, members export a ratcheted per-sender symmetric key); covers only the
  ≤99 call layer, excludes Stage and text (T1).

- **Telegram:** the only E2EE surface is a 1-to-1 Secret Chat (manual, device-specific). Cloud chats
  (default DMs, all groups, supergroups to 200k, channels, gigagroups, Saved Messages) are client-server
  encrypted, stored encrypted with Telegram holding jurisdiction-split keys. So a 200,000-member group is
  readable by Telegram in principle; the folk claim "regular DMs are encrypted, groups aren't" is wrong,
  because the default DM is also not E2EE (T1). Cap detail: supergroup mutual text 200,000; a basic group
  auto-converts above 200; channel broadcast subscribers unlimited; a gigagroup converts a supergroup so
  only admins write and the participant limit is removed; group calls (mutual voice/video) 200; livestream
  officially unlimited but unstable around ~2,000 simultaneous viewers (T3).

- **Matrix / Element:** E2EE-by-default for new private rooms since May 2020 via Olm (1-to-1) and Megolm
  (group). Megolm is a scaling compromise: one ratchet per room per device rather than per-message
  pairwise sessions (in a 200-person room, encrypting each message with 199 Olm sessions would be
  prohibitive), but the initial Megolm session key still distributes per-device, so sending in a large
  room can be slow. That residual per-member key-distribution cost is exactly what MLS's tree removes.
  Ginesin & Nita-Rotaru (Northeastern, PROVERIF, arXiv 2408.12743, T1/2): Olm+Megolm is comparable to
  Signal+Sender Keys if Olm pre-keys are signed, provably worse post-compromise if not; Matrix's spec
  mandates signing, so Matrix is fine. Most relevant, the paper states the Matrix Foundation intends to
  phase out Olm/Megolm in favor of MLS, citing better post-compromise security and better measured
  performance.

- **X / Twitter:** Nov 2025 replaced DMs with Chat, claiming E2EE including groups; the same help page
  concedes no MITM protection and possible access under legal process (T1). Matthew Green
  (cryptographyengineering.com, T2): XChat stores private keys on X servers (sharded via Juicebox),
  unlocked by PIN, and lacks forward secrecy; if X controls the key-storage servers it can decrypt, which
  is game-over for an E2EE claim. Verdict: disputed, not established.

- **Reddit:** a subreddit is a public content forum, not a mutual-messaging group; nothing is E2EE and
  there is no subscriber cap (r/funny ~67M; r/announcements ~305M auto-subscribed). It cannot be E2EE by
  construction, because ranking, search, sitewide moderation, and ad targeting all need server-readable
  content. Reddit Chat/DMs are not E2EE (transport encryption, Reddit holds keys), and Chat Channels ride
  the same non-E2EE infrastructure. Whether Reddit Chat uses an underlying federated chat protocol was not
  confirmed and is omitted.

- **Bluesky:** DMs are not E2EE, and moderators may open them to investigate abuse, because the AT
  Protocol was built for public discourse, not private messaging (T1). E2EE exists only bolted on from
  outside: Germ (built by an ex-Apple FaceTime/iMessage engineer) uses MLS on top of AT Protocol,
  integrated natively (T2/3); XMTP does the same via MLS-family encryption binding Bluesky handles to
  encrypted inboxes (T3). Two independent teams reaching for MLS to add E2EE to a public-by-design
  protocol corroborates the MLS-plus-public-substrate architecture.

- **Slack:** non-E2EE for compliance, a pure Force-2 case at moderate scale. Enterprise Key Management
  gives customers key control (customer-held keys in AWS KMS) but is not end-to-end; Slack states search
  cannot compile encrypted data. Huddles up to 50 on paid tiers (T1).

- **Microsoft Teams:** E2EE covers only 1-to-1 calls and configured meetings (audio/video/screen-share),
  never chat, and enabling it disables eDiscovery, DLP, translation, and recording (Force 2, per
  Microsoft, T1). Meetings interactive to ~1,000. Shows the tradeoff is not purely about scale.

## What this doc establishes (and does not)

Establishes the capability map of the fenced field: the roster / mutual-call / broadcast ceilings across
14 platforms, the E2EE stance resolved per surface and per layer, and the two forces (Force 1 the
key-agreement cost curve, Force 2 server-mediated core function) that make roster size and group-text
E2EE trade off. It records that the tradeoff is one-directional on every consumer platform and causal,
and that defeating both forces at once (MLS for Force 1, no server-read features for Force 2) is what
lets a system be large and encrypted at the same time, at the cost of server-side ranking, search, and
discovery.

Does not carry the per-group operational rates or platform monetization (those are the sibling doc), does
not make the harm argument (that is `activism/`), and does not catalogue the composable open field (that
is `cairn/`). Where the evidence is thin or contested, this map keeps it that way: WhatsApp Channels'
E2EE status is low-confidence, X's E2EE claim is disputed and not established, Reddit Chat's underlying
protocol is omitted for lack of confirmation, and the caps that were general-knowledge before being
pinned to primary sources (iMessage, LINE, WeChat, Slack, Teams) are flagged where they moved.
