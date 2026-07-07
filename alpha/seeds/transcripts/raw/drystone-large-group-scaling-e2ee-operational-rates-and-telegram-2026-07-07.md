# Raw transcript: large-group scaling research — E2EE-vs-scale, per-group operational rates, and the Telegram field notes (2026-07-07)

`Provenance caveat (PLAYBOOK §4): this is a content-faithful cleaned paste of the originating
claude.ai web session, NOT a byte-pristine export. The substantive artifacts (the research report,
the platform survey, and the Telegram Q&A) are reproduced faithfully; the interstitial tool-activity
narration ("Searched the web", "Ran a command, edited a file") is condensed to its substantive
findings in the "Session narrative" section rather than reproduced line-by-line. Raw transcripts are
exempt from the em-dash discipline; source punctuation is preserved as received.`

`What this session produced, and where it landed: the batch-11 deliverables filed from eleven.zip are
(1) the Drystone Part 2 §7 large-group-scaling spec section → beta/drystone-spec/large-group-scaling.md,
and (2) its companion research prompt → beta/drystone-spec/research-prompt-operational-rates.md. THIS
transcript is the research REPORT that the prompt produced (it is not itself in the zip; it lives only
here), plus a Telegram field-notes tail from the same session. Its load-bearing conclusions are distilled
into the new "fenced" layer (beta/fenced/) and cited by spec §7.9.1 (encryption posture) and §7.13
(empirical basis).`

---

## Part A — The research report (final version)

# Three Per-Group Operational Rates for Large-Group Community Messaging Platforms: A Source-Tiered Triangulation of Bans, Joins, and Live Fraction

## TL;DR

- **Member-add (join) rate is the best-supported quantity; live-fraction is well-supported in shape but definition-sensitive; member-ban rate per group per day is the thinnest and stays largely inferential.** For a mature, mid-sized community, member bans run on the order of well under one per day to a few per day in steady state, joins run from a handful to low hundreds per day, and the live (recently-active) fraction sits in the low single-digit percent of the roster and shrinks as the roster grows.

- All three quantities are governed by heavy-tailed distributions: most communities are tiny and nearly dormant, while a small number of very large communities dominate every platform aggregate. Report medians and ranges by size band, never a single headline number.

- The single most important guardrail held throughout: member bans (removing a person) are one to two orders of magnitude rarer than content removals (removing posts/comments). Content removal touched a meaningful share of all Reddit content (the platform's mods and admins removed 162,135,309 pieces of content from a base of over 5.3 billion in H1 2024) and about 8.5% of comments in an active political subreddit; member bans touch a low-single-digit percent of monthly active participants and a far smaller fraction of the total roster.

- **Roster size and end-to-end encryption are inversely related, and the relationship is causal, not coincidental.** The platforms with the largest group rosters (Telegram 200,000, Discord 250,000, and Reddit with no cap at all) do not end-to-end encrypt group text, while the platforms that do E2EE group text (WhatsApp 1,024, Signal 1,000) cap rosters two-plus orders of magnitude lower. Two distinct forces drive this: (1) the key-agreement cost curve (pairwise/sender-key E2EE has per-member setup cost that grows with the group), and (2) server-mediated core function (ranking, search, discovery, moderation, and broadcast all require the server to read plaintext, which E2EE denies). Reddit is the extreme endpoint, largest "groups," zero E2EE, driven mainly by Force 2. See the Encryption stance section.

## Reading the numbers: scope before magnitude

Every quantity in this report is a *rate or ratio with a scope*, and the scope matters more than the digits. Three scope distinctions recur and are the most common way these figures get misused:

- **Roster is not concurrency, and neither is call capacity.** A platform's "group size" cap counts accounts that have *joined a persistent text space*. It says almost nothing about how many are online at once, and nothing at all about how many can share a live voice/video call. These are three different limits set by three different constraints (storage/fan-out for the roster, connection handling for concurrency, real-time mixing for calls), and they differ by three-plus orders of magnitude on the same platform. See the Caveats section for the per-platform breakdown.

- **Member action is not content action (Guardrail 3).** Removing a person (a ban) is rare; removing a post or comment is frequent. They differ by one to two orders of magnitude. Any figure below is tagged as one or the other.

- **Active is not member-count, and the activity window defines "active" (Guardrail 4).** "Online now," "posted this month," and "visited in 7 days" are different denominators that can differ by an order of magnitude. Every live-fraction figure carries its window.

## Encryption stance: group size versus E2EE capability

This is a distinct axis from the operational rates, added because it turns out to be tightly coupled to roster size. The headline: **on consumer platforms, the ability to end-to-end encrypt a group and the ability to make that group large are in direct tension, and every platform resolves it by giving up one.** All figures below are from primary or near-primary sources (platform docs, protocol whitepapers, or a peer-reviewed analysis), verified this pass.

### The comprehensive comparison (all platforms, all dimensions)

Figures are current as of this pass (mid-2026) and tagged by source tier where they matter; caps change, so re-verify before using any single number in an SLA. "E2EE group" means a group whose text content the provider cannot read.

| Platform | Category | Largest text group (roster) | Largest mutual voice/video call | Largest E2EE group (text) | Group text E2EE by default? | Notable dimensions |
|---|---|---|---|---|---|---|
| **Signal** | Messaging | 1,000 | 40 (call) | 1,000 (all groups) | Yes, always | No non-E2EE mode; server holds no group metadata (private group system); pairwise-channel model, hence small cap |
| **WhatsApp** | Messaging | 1,024 | 32 (call) | 1,024 (all groups) | Yes, always | Signal-protocol sender keys; Communities + Channels (broadcast, likely not E2EE, low-confidence); group membership not cryptographically bound, server can inject a member (Albrecht/Dowling/Jones, KCL, IEEE S&P) |
| **iMessage** | Messaging | 32 (group) | 32 (FaceTime) | 32 (all iMessage-to-iMessage) | Yes, between Apple devices | E2EE only Apple-to-Apple; mixed SMS/RCS group falls back to 20-cap, not E2EE; caps verified at Apple Support |
| **Facebook Messenger** | Messaging | ~250 (group) | ~32 (call) | ~250 (group, E2EE) | Yes (default since Dec 2023) | Signal protocol + Meta's Labyrinth protocol; group calls add SFrame over SFU; server-side key recovery via PIN |
| **LINE** | Messaging | 500 (group) | ~500 (call, varies) | 50 (E2EE only for groups up to 50) | Yes for text 1-to-1 and groups up to 50 (default, non-disable-able since 2021) | Letter Sealing default and non-disable-able since 2021, but text E2EE caps at 50-member groups even though groups go to 500; group calls/video/Meeting transport-only, not E2EE |
| **WeChat** | Messaging | 500 (group) | ~9 (video) | None | No | Transport/at-rest AES-256 only, not E2EE; retains text ~72h, media ~120h on servers |
| **Discord** | Messaging/community | 250,000 (server; up to 1M) | 99 (voice channel) | None (text); calls E2EE via DAVE/MLS | No (text never E2EE, by design) | E2EE only on the ≤99 call layer via MLS (DAVE); text non-E2EE for moderation; Stage channels excluded |
| **Telegram** | Messaging/community | 200,000 (supergroup); channels unlimited | 200 (group call); ~2,000 livestream (indicative) | 0 (only opt-in 1-to-1 Secret Chat is E2EE) | No | Default is server-readable Cloud Chat; only 1-to-1 opt-in Secret Chat is E2EE; gigagroups remove the 200k cap for broadcast |
| **Matrix / Element** | Federated messaging | No hard cap (thousands; large rooms strain) | Element Call, tens (SFU-based) | Same as room size (Megolm), large but costly | Yes (default for new private rooms since 2020) | Federated/self-hostable; Megolm per-room-per-device ratchet; large-room send slow due to per-device key distribution |
| **X (Twitter)** | Social + messaging | Group Chat (claimed E2EE, disputed) | Built into Chat | Claimed for groups, disputed | Claimed, widely disputed | XChat: keys stored server-side via Juicebox, no forward secrecy, no MITM protection per X's own help page; cryptographers call it not true E2EE; Communities are public |
| **Reddit** | Social/forum | No cap (subscribe model; tens of millions) | n/a | None | No (public forum, cannot be) | Subreddits are public ranked/searchable forums; Chat/DMs and Chat Channels not E2EE; the extreme Force-2 endpoint |
| **Bluesky** | Social/forum | No cap (public follow model) | n/a | None natively (E2EE via external MLS apps: Germ, XMTP) | No (public-by-design AT Protocol) | AT Protocol built for public discourse; DMs not E2EE, moderation can access; E2EE bolted on via MLS-based third parties |
| **Slack** | Enterprise | Workspace/channel: org-scale | Huddles, up to 50 (paid) | None | No | Enterprise Key Management is customer-held keys in AWS KMS, still not E2EE; Slack states search cannot compile encrypted data (Force 2) |
| **Microsoft Teams** | Enterprise | Team/channel: org-scale (10k+) | Meetings to ~1,000 interactive | 1-to-1 calls + configured meetings only; never chat | No (group text/chat never E2EE) | Optional E2EE covers only audio/video/screen-share, not chat; E2EE disables eDiscovery, DLP, translation, recording (Force 2, per Microsoft) |

Category note: the pattern holds across all three categories. Messaging apps that keep E2EE cap near 1,000 (Signal, WhatsApp, iMessage, Messenger); community/social platforms that go large drop group-text E2EE entirely (Discord, Telegram, Reddit, Bluesky, X); enterprise tools drop it for compliance/search reasons even at moderate scale (Slack, Teams). Matrix is the interesting middle: it keeps E2EE and allows large rooms, but pays the Force-1 cost in large-room latency, which is precisely the cost MLS is designed to remove.

### The layered fact

"Is it E2EE?" has no single answer per platform, because encryption status varies by *surface* (1-to-1 vs group vs broadcast) and by *layer* (text vs real-time voice/video). A claim true of one surface is usually false of another.

| Platform | Group text roster cap | 1-to-1 text E2EE | Group text E2EE | Group voice/video E2EE | Broadcast object E2EE |
|---|---|---|---|---|---|
| **Signal** | 1,000 | Yes (always) | Yes (always) | Yes (calls to 40) | n/a |
| **WhatsApp** | 1,024 | Yes (Signal protocol) | Yes (sender keys) | Yes (calls to 32) | Channels: not E2EE (low-confidence) |
| **Discord** | 250,000 | No (text never E2EE) | No (by design, for moderation) | Yes (DAVE, calls to 99; excludes Stage) | Stage channels: not E2EE |
| **Telegram** | 200,000 (supergroup) | Only in opt-in Secret Chat | No (any size) | No (group voice chats) | Channels/gigagroups: not E2EE |
| **Reddit** | No cap (tens of millions) | No (Reddit Chat/DMs) | No (subreddits public; chat channels not E2EE) | n/a | The subreddit itself is the broadcast surface: public, not E2EE |

### Why the inverse relationship holds (the design lesson)

Force 1, the key-agreement cost curve. Traditional group E2EE (Signal's pairwise channels, WhatsApp's sender-keys) has setup cost that grows with membership, so the moment a platform wants a very large roster it must either abandon group E2EE or confine E2EE to a naturally-bounded layer. This is the force that caps the E2EE-first platforms near 1,000 (Signal, WhatsApp) and that pushed Discord to E2EE only the small real-time call layer (DAVE on the ≤99 voice channel, not the 250,000 text roster). MLS (RFC 9420) is the standards-body answer to exactly this force: tree-based key agreement with logarithmic rather than linear cost. One survey paper (arXiv 2401.09102) puts a number on the pre-MLS cost: naive pairwise E2EE in an N-member group needs on the order of N-squared key-exchange messages, so a 500-member group needs at least 250,000, and E2EE in large groups works best when a minority actively transmits and the majority passively receives. That last point rhymes with the live-fraction finding: the same skew that makes large groups mostly-lurkers is what makes sender-oriented E2EE tractable at all.

Force 2, server-mediated core function. Independent of per-member cost, the more a platform's core function depends on the server reading content (ranking/sorting, full-text search, discovery/recommendation, sitewide moderation, ad targeting, one-to-many broadcast), the less E2EE is even possible, because E2EE by definition denies the server the plaintext those functions consume. This is the force that makes Reddit subreddits non-E2EE by construction and Telegram channels/gigagroups non-E2EE. MLS alone would not make them encryptable: you cannot tree-ratchet your way to an encrypted feed the server still has to rank and search.

The two forces stack. Discord text is non-E2EE for Force 2 (moderation needs plaintext) even though its call layer solved Force 1 with MLS. Reddit is non-E2EE for Force 2 at every layer and never even reaches Force 1, which is why it can carry tens-of-millions-member "communities": there are no per-member keys to cost anything because the server reads everything. The design implication for a system that wants large and encrypted at once: use MLS (or equivalent log-cost group key agreement) to defeat Force 1, and avoid any feature that requires the server to read content to defeat Force 2. Giving up server-side ranking, search, and discovery is the price of E2EE at scale, and it is a product decision, not a cryptographic one.

### Per-platform detail, with sources (condensed)

- **Signal:** every surface E2EE, no non-E2EE mode; "private group system" so the service has no record of memberships, titles, avatars, attributes; limit 1000; group calls 40; pairwise per-pair channel model explains the small cap.

- **WhatsApp:** group contents encrypted using the Signal Protocol; roster scaled 256 → 512 → 1,024. Two caveats: (a) the King's College London formal analysis (Albrecht, Dowling & Jones, "Formal Analysis of Multi-device Group Messaging in WhatsApp," IEEE S&P / Springer LNCS) proved message payloads remain E2EE but clients trust the server to supply the group member list, so membership changes carry no cryptographic binding and the server can inject a member; the computational companion "WhatsUpp with Sender Keys?" (Balbás/Collins/Gajland, IACR ePrint 2023/1385) states an adversary with server-level control can add users without any member's authorization; traces to the 2018 Rösler/Mainka/Schwenk "More is Less" result, Marlinspike's rebuttal noting every member still sees the join. (b) whether WhatsApp Channels are non-E2EE could not be confirmed at a primary source; low-confidence.

- **Discord:** deliberately does not E2EE text so it can moderate; audio/video E2EE via DAVE, built on MLS (media session members undergo an MLS group key exchange, voice gateway is the MLS delivery/authentication service, members export a ratcheted per-sender symmetric key); covers only the ≤99 call layer, excludes Stage and text.

- **Telegram:** only E2EE surface is a 1-to-1 Secret Chat (manual, device-specific). Cloud chats (default DMs, all groups, supergroups to 200k, channels, gigagroups, Saved Messages) are client-server encrypted, stored encrypted with Telegram holding jurisdiction-split keys. So a 200,000-member group is readable by Telegram in principle; "regular DMs are encrypted, groups aren't" is wrong, the default DM is also not E2EE.

- **Reddit:** subreddit is a public content forum, not a mutual-messaging group; nothing E2EE, no subscriber cap (r/funny ~67M; r/announcements ~305M auto-subscribed). Cannot be E2EE by construction (ranking/search/sitewide-moderation/ad-targeting need server-readable content). Reddit Chat/DMs not E2EE (transport encryption, Reddit holds keys). Chat Channels ride the same non-E2EE infra. Whether Reddit Chat uses Matrix was not confirmed and is omitted.

- **Facebook Messenger:** default E2EE since Dec 2023, Signal Protocol + Meta's Labyrinth; group text via Signal "Sender Keys" (GroupCipher); group calls add an SFrame layer because SRTP through an SFU is not end-to-end. Shares WhatsApp's server-side key recovery via PIN.

- **X / Twitter:** Nov 2025 replaced DMs with Chat, claiming E2EE including groups; the same help page concedes no MITM protection and possible access under legal process. Matthew Green: XChat stores private keys on X servers (sharded via Juicebox), unlocked by PIN, lacks forward secrecy; if X controls the key-storage servers it can decrypt, which is game-over for an E2EE claim. Verdict: disputed, not established.

- **Matrix / Element:** E2EE-by-default for new private rooms since May 2020 via Olm (1-to-1) and Megolm (group). Megolm is a scaling compromise: one ratchet per room per device rather than per-message pairwise sessions (in a 200-person room, encrypting each message with 199 Olm sessions would be prohibitive), but the initial Megolm session key still distributes per-device, so sending in a large room can be slow. That residual per-member key-distribution cost is exactly what MLS's tree removes. Ginesin & Nita-Rotaru (Northeastern, PROVERIF, arXiv 2408.12743): Olm+Megolm is comparable to Signal+Sender Keys if Olm pre-keys are signed, provably worse post-compromise if not; Matrix's spec mandates signing, so Matrix is fine. Most relevant: the paper states the Matrix Foundation intends to phase out Olm/Megolm in favor of MLS, citing better post-compromise security and better measured performance. Matrix moving off a working custom E2EE stack toward MLS is independent corroboration of the MLS bet.

- **Bluesky:** DMs not E2EE, moderators may open them to investigate abuse, because the AT Protocol was built for public discourse, not private messaging. E2EE exists only bolted on from outside: Germ (ex-Apple FaceTime/iMessage engineer) uses MLS on top of AT Protocol, integrated natively; XMTP does the same via MLS-family encryption binding Bluesky handles to encrypted inboxes. Two independent teams reaching for MLS to add E2EE to a public-by-design protocol is strong corroboration of the MLS-plus-public-substrate architecture.

- **Enterprise (Slack, Teams):** non-E2EE for compliance, a pure Force-2 case at moderate scale. Slack EKM gives customers key control but is not end-to-end; Slack states search cannot compile encrypted data. Teams E2EE covers only 1-to-1 calls and configured meetings (audio/video/screen-share), never chat, and enabling it disables eDiscovery, DLP, translation, recording. Shows the tradeoff is not purely about scale.

## Key Findings (operational rates)

1. **Distribution shape first.** Users-per-community and comments-per-community are heavy-tailed; comments-per-user follows a power law with exponent about -1.44. Per arXiv 2503.02661: "20% of users contribute 84% of the comments; and approximately 50% of users make only one comment in a whole month on Reddit."

2. **Member-ban rate per group per day (LOW confidence).** Best anchor: arXiv 2205.14529 (Li, Hecht & Chancellor, ICWSM 2022), private mod logs for 126 subreddits, 900+ moderators, 3M+ actions, ~220 human + 20 bot ban/unban actions per day across the sample, ≈1.9 ban actions per subreddit per day over active logging-enabled communities. In r/politics, of ~51,000 monthly participants, CAT Lab (Matias, Platt & Gilbert 2024) reports ~4,000 accounts banned per month, ~130/day, ~7.6% of the 51,000 monthly participants (the study's separately reported "1.9% of all posters and commenters per month" uses a larger accumulated base). Steady-state central estimate mid-sized: substantially less than one ban/day; large: a few to tens/day; very large: ~100+/day.

3. **Member add/join rate per group per day (MEDIUM confidence).** Mature communities grow at low-single-digit percent per month, a handful to low hundreds of joins/day by size. Bursts dramatic: r/popular front-page posts had "70 times the number of newcomers" of matched non-front-page posts (Understanding Community Resilience, 6,306 posts, 1,320 subreddits). COVID onset: Reddit new users +52% YoY (arXiv 2304.10777).

4. **Live fraction as a function of roster (MEDIUM-HIGH on shape).** Small and shrinks with size: ~1-5% "active," well under 1% "online now," ~1% "creating" (90-9-1). Reddit moved from subscriber counts to "Visitors" (7-day uniques); Discord ~259M MAU vs ~656M registered (roughly 39-40%). arXiv 2410.21996: a size-independent active core (~2,000 heavy contributors) across very-different-sized subreddits, so its fraction collapses as roster grows.

### Cross-platform anchor (WhatsApp groups)

A WhatsApp-groups moderation study (arXiv 2106.05184, 5,051 groups, 2.6M messages over 302 days) recorded 437,000 membership action events: 61k added by a member, 73k added by an admin, 132k joined via invite link, 154k left, 9k removed. Normalized: ~0.29 membership actions per group per day averaged across ordinary (non-viral) groups, with adds and voluntary leaves dominating and admin removals rare. Independent-platform sanity check that steady-state per-group join/leave churn is low in absolute terms.

## Recommendations

1. Design to the distribution, not the mean: small (~1K) assume <1 ban/day, a few joins/day, 5-15% active; mid (~50K) <1 to a few bans/day, tens of joins/day, 1-5% active; large (millions) up to tens of bans/day (100+ extreme), low hundreds of joins/day, well under 1% active.

2. Keep the widest safety margin on member-ban rate (thinnest quantity); provision burst multipliers of at least 2-5x.

3. Provision joins for virality (70x front-page influx), one-to-two-order-of-magnitude transient spikes.

4. Never size live infrastructure to the roster: presence/fan-out to the active fraction (~1-10%, often less); real-time subsystems to the call-concurrency ceiling (two-to-three orders of magnitude below roster).

## Caveats: platform hard limits, three layers not one number

Product/spec caps split into three separately-enforced limits: **roster** (accounts in a persistent text space; bounded by storage and fan-out; the big number), **concurrent-online** (bounded by connection handling; far below roster), and **call-concurrency** (mutual real-time voice/video; bounded by media mixing; two-to-three orders of magnitude below roster).

- **Discord.** Roster 250,000 standard (Tier-3 boosting OR a reviewed increase raises base to 1,000,000; Partner servers 100,000). Concurrent-online ~5,000 default before errors (Tier 3). Call-concurrency: voice channel 99; Stage asymmetric (≤99 speakers + large listen-only audience). Discord tracks actions against users/servers, not per-piece content.

- **Telegram.** Supergroup (mutual text) 200,000; basic group auto-converts above 200. Channel (broadcast) unlimited subscribers. Gigagroup: convert a supergroup, only admins write, participant limit removed. Group calls (mutual voice/video) 200. Livestream officially unlimited; unstable ~2,000 simultaneous viewers (Tier 3).

- **WhatsApp.** Roster 1,024 (256 → 512 → 1,024 in 2022). Call-concurrency 32. Communities and Channels are separate objects. Server-side fan-out copies one ciphertext to all members (a single group encryption, not one-per-member), heavy cost being initial per-member session setup.

- **Signal.** Groups 1,000; group calls 40; all surfaces E2EE via the private group system with no server-held group metadata.

## Sources (consolidated reference table)

Tier key: T1 = standards body/spec/platform primary docs; T2 = peer-reviewed or arXiv, or direct reporting of a primary announcement; T3 = secondary commentary/trackers, corroboration only.

Encryption / E2EE / capacity: Signal Support "Group chats" (T1); 9to5Google WhatsApp group limit (T2); Albrecht/Dowling/Jones "Formal Analysis of Multi-device Group Messaging in WhatsApp," IEEE S&P / Springer (T1/2); Balbás/Collins/Gajland "WhatsUpp with Sender Keys?" IACR ePrint 2023/1385 (T1); Rösler/Mainka/Schwenk "More is Less" 2018 (T2); Apple Support HT209022 (T1); Meta "Default E2EE on Messenger" + engineering overview PDF (T1); LINE Encryption Report 2025 + Help Center (T1); WeChat Help Center (T1); Discord "Meet DAVE" + DAVE whitepaper (T1); Discord Support account/server caps (T1); Telegram E2EE FAQ + API end-to-end + FAQ + Channels API (T1); Telegram limits tracker (T3); Matrix.org E2EE guide + client tutorial (T1/2); Ginesin & Nita-Rotaru arXiv 2408.12743 (T1/2); X Help "About Chat" (T1); Matthew Green cryptographyengineering.com (T2); The Register XChat (T2); Bluesky DMs blog (T1); Germ/Metricool + TechBuzz (T2/3); XMTP bluesky-chat repo (T3); Slack EKM (T1); Computerworld Slack-no-E2EE (T2); Microsoft Teams encryption blog June 2025 (T1); Wire Teams-E2EE-limits (T2); Factually Reddit-chat (T3); TechCrunch Reddit chat channels (T2); arXiv 2401.09102 decentralized-messaging survey (T2).

Operational rates: Reddit Transparency Report H1 2024 (T1); Reddit S-1 SEC EDGAR (T1); Discord Transparency Report H1 2024 (T1); Discord platform statistics anchored to a May 2026 Texas AG filing (T3); Li/Hecht/Chancellor arXiv 2205.14529 (T2); CAT Lab r/politics (T2/3); arXiv 2503.02661 super-linear growth (T2); arXiv 2410.21996 multi-layer network analysis (T2); arXiv 2304.10777 Reddit-in-COVID (T2); arXiv 2106.05184 WhatsApp groups dataset (T2); r/popular study ICWSM 2024 / AAAI (T2); NN/g 90-9-1 (T2/3); Chandrasekharan 2018 CSCW + Jhaver 2019 TOCHI (T2); subreddit-analytics vendors (T3, corroboration only); r/funny + r/announcements (T1 page / T3 counts).

## Honest Summary

Join rate and live-fraction shape are supported reasonably well; member-ban rate per group per day is supported only weakly and stays inferential (best anchor ~1.9 ban actions/subreddit/day skews to large active communities; Reddit's Tier-1 community-level count is locked in a chart image; small-community rates essentially unmeasured; defensible range zero-per-week to ~100+/day). On encryption the evidence is strong and one-directional: roster size and group-text E2EE trade off on every consumer platform, and the trade is causal, driven by Force 1 (key-agreement cost) and Force 2 (server-mediated function). For a design that wants large and encrypted, defeat Force 1 with a scalable group key-agreement protocol (MLS, RFC 9420, the only such protocol in production consumer use via Discord DAVE), whose authenticated-membership property is exactly what WhatsApp's sender-key model was found to lack; and defeat Force 2 by giving up server-side ranking, search, and discovery, a product decision, not a cryptographic one.

---

## Part B — Session narrative (build log, condensed to substantive findings)

The report was built and then hardened across several follow-up turns. The substantive verification work:

- **Platform verification pass.** Verified at primary/near-primary sources: X's Chat (server-side key storage via Juicebox, no forward secrecy, no MITM protection, disputed E2EE); Messenger default E2EE (Signal + Labyrinth, sender keys, SFrame over SFU); Matrix (Megolm per-room-per-device, large-room latency, Foundation intends MLS migration); Bluesky (DMs not E2EE, Germ + XMTP add MLS from outside the core protocol).

- **Cap-pinning pass (the five general-knowledge rows moved to primary):** iMessage/FaceTime 32 (Apple Support; mixed SMS/MMS 20); LINE (Letter Sealing default and non-disable-able since 2021, text E2EE capped at 50-member groups even though groups go to 500, group calls transport-only — a real correction, not just a citation); WeChat 500 + no E2EE (AES-256 in transit/at rest, retains text ~72h media ~120h); Slack Huddles 50, EKM not E2EE (search cannot compile encrypted data); Microsoft Teams E2EE only 1-to-1 calls and configured meetings, never chat, disables eDiscovery/DLP/translation/recording.

- **Paper-attribution pass.** Matrix paper is Northeastern (Ginesin & Nita-Rotaru, PROVERIF symbolic), distinct from the King's College computational analysis it builds on. WhatsApp membership-injection primary is Albrecht/Dowling/Jones (KCL, IEEE S&P / Springer) plus the Balbás/Collins/Gajland computational companion (IACR ePrint 2023/1385), tracing to 2018 Rösler/Mainka/Schwenk.

- **Bonus items folded in, arithmetic-checked.** O(N-squared) pre-MLS cost concretized (arXiv 2401.09102: 500-member group ≥250,000 key-exchange messages; E2EE at scale works best when a minority transmits — ties to the live-fraction skew). WhatsApp-groups dataset (arXiv 2106.05184: ~0.29 membership actions/group/day, adds and voluntary leaves dominating, admin removals rare) as a cross-platform anchor.

- **Reference-grounding pass.** Every publisher-level locator pinned to a retrieved source (Reddit Transparency PDF, Reddit S-1 on SEC EDGAR, Discord Transparency Report, r/popular ICWSM/AAAI proceedings, NN/g 90-9-1, Chandrasekharan CSCW + Jhaver TOCHI DOIs). The one genuine holdout is Discord platform statistics (no single canonical live page), anchored to a dated May 2026 Texas AG filing and labeled T3.

---

## Part C — Telegram field notes (Q&A tail from the same session)

### Is Telegram broadcast-only or a discussion app?

Both. Telegram is a general-purpose messaging app; the broadcast-vs-discussion question maps onto two architecturally distinct surfaces.

- **Groups are the discussion side.** An interactive chat where all members can send, reply, and share media, up to 200,000 members. Many-to-many, closer to a forum or group chat.

- **Channels are the broadcast side.** One-to-many; only the owner and appointed admins publish, subscribers read but cannot directly reply. Subscriber count uncapped.

- **The seam:** a channel admin can link a discussion group behind the scenes so each post gets a comment thread. In every case where comments exist on a channel, those comments are technically happening in a linked group, not in the channel itself. The two primitives stay separate under the hood.

- Structural note (secondary sources, not Telegram's own docs): Telegram reportedly does not support converting a group into a channel or vice versa, because they are fundamentally different structures.

Practical shorthand: delivery → channel; interaction → group; many setups use both (a channel for announcements linked to a group for discussion). So "broadcast only or discussion app" is a false binary: channels are the broadcast primitive, groups the discussion primitive, wired together.

### Telegram for profit? How do they make money?

Now for-profit and, on the underlying business, profitable, a recent shift (roughly its first decade ran on the founders' money and made essentially nothing).

- **H1 2025 revenue: $870 million, up 65% YoY.** Split: ~$300M from "exclusivity deals" tied to Toncoin (the ecosystem cryptocurrency); $223M from Premium subscriptions; $125M from advertising.

- **Premium:** freemium, launched 2022, ~$5/month; paid subscriptions rose to 15M from 4M in late 2023.

- **Advertising:** deliberately narrow — Sponsored Messages appear only in large channels (above 1,000 members); advertisers buy with Toncoin; Telegram shares half the income with channel owners.

- **Toncoin (TON):** cryptocurrency woven into Telegram's payment plumbing; settlement currency for much of the ecosystem (Premium and Fragment-marketplace purchases rely on TON). Telegram holds Toncoin on its books, so the token's price feeds its reported financials.

- **The accounting split:** H1 2025 net loss ~$222M (vs $334M net profit H1 2024), driven largely by a write-down after Toncoin lost ~69% of value in 2025; stripping that out, operating profit ~$400M, so the underlying business remained profitable before crypto losses.

- Caution: the "$2 billion revenue / $720 million profit in 2025" headline is a target/projection, not a result. Full-year-2024 "$1.4B revenue, first-ever profit >$500M" comes from secondary commentary, treat as reported-but-not-audited.

- Smaller streams: in-app purchases (paid stickers, Telegram Stars), TON-ecosystem partnerships. Leverage: recent debt offerings include bond-to-equity conversion options at a discount if an IPO proceeds; Durov under formal investigation since 2024 over allegations the platform failed to address criminal content.

### Telegram Premium features (canonical, per Telegram's own FAQ)

Complete current set per Telegram: Doubled Limits, 4 GB File Uploads, Faster Download Speed, Voice-to-Text Conversion, Premium Stickers, Unique Reactions, Advanced Chat Management, Animated Profile Pictures, Profile Badges, Premium App Icons, No Ads, Custom Emoji, Voice Message Privacy Settings, Voice-to-Text for Video Messages, Emoji Statuses, and Real-time Chat and Channel Translation.

- **Raised limits:** file uploads 2GB → 4GB; join 1,000 channels/groups instead of 500; pin 10 chats instead of 5; 20 folders with 200 chats each instead of 10; prioritized download speeds.

- **Utility:** voice-to-text transcription (also video messages; can disable incoming voice messages); real-time translation bar; auto-archiving of messages from unknown users.

- **No ads:** official Sponsored Messages hidden for subscribers; but promotional content channel admins post themselves is outside Telegram's control and still appears (so "ad-free" means Telegram's own ads, not everything).

- **Cosmetic:** full-screen animated stickers (visible to all, sendable only by Premium), 10+ exclusive emoji reactions, animated profile videos, custom app icons, profile badge (badge is not optional, no toggle to hide it).

- **Early access:** Premium users get new features first (Stories, advanced admin tools).

- **Boosts:** the mechanism by which Premium spills into communities. A boost is a Premium subscriber lending their account's weight to a channel/group; a ten-level ladder gated by subscriber count. Channels: Level 1 unlocks 1 story/day, 1 custom reaction, 7 name colors, 7 link/quote styles; Level 10 adds custom backgrounds, 1000+ emoji statuses, custom logos. Groups: different, utility-flavored ladder, e.g. Level 6 unlocks unlimited voice-to-text, higher levels add group emoji packs, cover colors, custom backgrounds.

- **Perks leak to free users:** any user can download the 4 GB files a Premium user uploads, watch Premium sticker animations, and tap to increase an exclusive reaction a Premium user already added. A single subscriber raises the ceiling for those around them.

- **Price:** ~$4.99/month US (£4.99 UK, ~€4.99 EU); subscribing through the official @PremiumBot bypasses Apple's/Google's 30% fees, passed on as a cheaper direct price.

- **Cancellation degrades gracefully:** features kept until end of billing period, then locked at the free ceiling (extra channels/folders stay but can't add more, GIFs past 200 hidden, bio truncates to 70 chars, pinned chats back to 5, badge disappears); nothing created is deleted.
