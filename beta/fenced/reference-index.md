# Reference index: the fenced field (extent, shape, capability, economics)

Every source cited across the `fenced/` layer, grouped by type, with the doc it supports, its
epistemic tag, a primary/secondary marker, and the source tier or confidence the doc itself carries.
The fenced docs run on their own three-level source-tier system rather than the activism layer's
epistemic-status labels, so that tier is preserved verbatim alongside the marker below.

Marker key:

PRIMARY = the artifact itself (the platform's own spec / support page / FAQ, the RFC, the
peer-reviewed paper or preprint, the SEC filing, the company's own signed note or announcement).

PRIMARY-VENUE = a first-person account reported at or by the venue where it was made (closest
available primary; used here for a maintainer's own public account of a migration).

SECONDARY = reporting, trackers, or vendor estimates standing in for a primary that is not public
(private-company financials; tracker-tier population figures).

[confirm] = the doc flags the figure as unverified / AI-surfaced pending a primary-source pass; the
flag is carried verbatim and must not be cleared without pulling the primary.

Source tiers, as the docs use them:

- T1 = standards body, spec, or platform primary docs.
- T2 = peer-reviewed / arXiv work, or direct reporting of a primary announcement.
- T3 = secondary commentary or trackers; corroborates, never founds.

Volatile figures (platform caps, revenue, MAU, valuations, subscriber counts) are marked
**point-in-time**: they were current as of the docs' mid-2026 authoring and drift, so re-verify any
single number before it enters an SLA or external use.

---

## Platform data & capability sources

**The 14-platform capability map** (roster / mutual-call / broadcast ceilings and E2EE stance per
surface and layer). PRIMARY (each platform's own spec / support / FAQ). Tier-tagged per platform;
**point-in-time**. Relied on by `group-scale-versus-e2ee.md` (the map) and `platform-dominance-and-adoption.md`
(the field map). The per-platform primaries and tiers:

- Signal — 1,000 group / 40 call, always-E2EE, private group system (T1).
- WhatsApp — 1,024 group, Signal-protocol sender keys (T2); Channels' E2EE status could not be
  confirmed at a primary and is **low-confidence**.
- iMessage — 32 group / 32 FaceTime, Apple-to-Apple E2EE, 20-cap non-E2EE SMS/RCS fallback (T1, Apple Support).
- Facebook Messenger — ~250 group, default E2EE since Dec 2023, Signal Protocol + Meta's Labyrinth (T1).
- LINE — 500 group, Letter Sealing default/non-disable-able since 2021, text E2EE caps at 50 (T1).
- WeChat — 500 group, no E2EE, AES-256 transit/at-rest, retains text ~72h / media ~120h (T1).
- Discord — 250,000 (up to 1M) server, 99 voice, DAVE (MLS) on the ≤99 call layer only (T1).
- Telegram — 200,000 supergroup, 200 group call, ~2,000-viewer livestream (T3), channels unlimited (T1).
- Matrix / Element — no hard cap, Olm+Megolm, E2EE-default for new private rooms since May 2020 (T1/2).
- X (XChat) — Nov 2025 Chat claims group E2EE; X's own help page concedes no MITM protection and
  possible legal-process access (T1); the claim is **disputed, not established**.
- Reddit — no cap (r/funny ~67M, r/announcements ~305M auto-subscribed), nothing E2EE (T1/T3);
  whether Reddit Chat rides a federated protocol is omitted for lack of confirmation.
- Bluesky — no cap, AT Protocol public-by-design, DMs not E2EE (T1).
- Slack — org-scale channels, Huddles up to 50 (paid), EKM is customer-held keys not E2EE (T1).
- Microsoft Teams — org-scale, meetings interactive to ~1,000, E2EE only on 1-to-1 calls / configured meetings, never chat (T1).

**MLS: RFC 9420 (protocol) and RFC 9750 (architecture).** PRIMARY (IETF standards). T1. The
measuring stick against the centered platforms and the standards-body answer to the Force-1
key-agreement cost curve (tree-based, logarithmic). Relied on by `group-scale-versus-e2ee.md` and
`group-chat-failure-modes.md`.

**Facebook group-size distribution.** Meta publishes no official average/median, so the doc
triangulates and reports max/mean/median separately; mixed-tier, **point-in-time**. Relied on by
`operational-rates-and-platform-economics.md`.

- Maximum ~8.3M–8.6M members (English-learning group ~8.6M private; relationship-counseling ~8.3M public). SECONDARY, tracker-tier, T3, indicative only.
- Mean ~1,000–2,500 (global platform math: 10M+ groups against ~1.8–2.5B monthly-interacting users). SECONDARY, T3.
- Mean ~8,727 public / 13,277 private (academic cross-section scrapes; inflated by viral outliers). PRIMARY (academic scrape), T2.
- Median ~1,400 for niche/thematic groups (IQR ~765–2,800, JMIR-style mapping across hundreds of active groups); ~25–100 for high-engagement daily groups. PRIMARY (academic mapping), T2 (the stronger anchor here).

**Reddit Transparency Report H1 2024.** PRIMARY (platform report). T1, **point-in-time**. Sitewide
content-removal totals (162,135,309 pieces removed from a base over 5.3 billion); bounds content
actions, not member bans (the community-level ban count is locked in a chart image). Relied on by
`operational-rates-and-platform-economics.md`.

**Platform-reported activity metrics.** Relied on by `operational-rates-and-platform-economics.md`.
- Reddit "Visitors" (seven-day-unique measure, replacing subscriber counts). PRIMARY, T1.
- Discord ~259M monthly-active against ~656M registered (~39–40%). PRIMARY / tracker, T1/T3, **point-in-time**.

**Per-group operational-rate empirical studies** (member-ban, join, live-fraction). All PRIMARY
(the paper itself), tiers as noted. Relied on by `operational-rates-and-platform-economics.md`.

- arXiv 2503.02661 — distribution shape; comments-per-user power law, exponent ~-1.44; "20% of users contribute 84% of the comments; ~50% make only one comment/month on Reddit." T2.
- arXiv 2205.14529 — Li, Hecht & Chancellor, ICWSM 2022. Private moderator logs, 126 subreddits, 900+ moderators, 3M+ actions; ~1.9 ban actions per subreddit per day. T2. (Ban rate is the thinnest of the three rates — LOW confidence overall.)
- CAT Lab — Matias, Platt & Gilbert 2024. r/politics ~4,000 bans/month (~130/day) against ~51,000 monthly participants (~7.6%); ~8.5% of comments removed in one active political subreddit. T2/3.
- "Understanding Community Resilience" — 6,306 posts, 1,320 subreddits, ICWSM 2024 / AAAI. r/popular front-page exposure = "70 times the number of newcomers." T2.
- arXiv 2304.10777 — Reddit new users +52% YoY at COVID onset (platform-shock burst magnitude). T2.
- arXiv 2106.05184 — WhatsApp moderation study, 5,051 groups, 2.6M messages, 302 days, 437,000 membership-action events; ~0.29 membership actions per group per day; 9k removals of 437k total (member-removal-is-rare). T2. Cross-platform anchor.
- arXiv 2410.21996 — size-independent active core (~2,000 heavy contributors) across differently-sized subreddits; the mechanism behind the shrinking live fraction. T2.
- NN/g 90-9-1 heuristic (~1% create, ~9% engage, ~90% lurk). SECONDARY, T2/3; corroborates shape only.

**Discord dominance and behavior data.** Relied on by `platform-dominance-and-adoption.md`.
- Launch May 23, 2015 (voice-for-gamers against TeamSpeak/Ventrilo/Mumble/Skype). PRIMARY-ish product history, T2/T3.
- Reported latency TeamSpeak ~20–40 ms vs Discord ~50–100 ms; RAM ~60–70 MB vs 200 MB+. SECONDARY, T3, moderate confidence.
- Aug 2017: 45M users, 9M daily, then-secret $50M raise. SECONDARY, T3, **point-in-time**.
- Matrix-only channel → after >1 year running both, >95% moved to Discord within a month, community grew orders of magnitude, against the maintainer's stated preference. PRIMARY-VENUE (maintainer's own account), T2.

**Adoption / crossing data.** Relied on by `platform-dominance-and-adoption.md`.
- Signal 100M+ installs (the one project across the chasm); inciting event the WhatsApp privacy-policy change of Jan 2021. T2, **point-in-time**.
- Adoption bridges: Briar in Myanmar post-coup; a Bluetooth-mesh app during Madagascar unrest; Signal spikes on surveillance news; Matrix mandated across 25+ countries for government-sovereign infrastructure. T2.

**WhatsApp abuse-posture data point.** WhatsApp bans hundreds of thousands of accounts per month on
metadata and reports, never content. PRIMARY (platform posture), **point-in-time**. Relied on by
`app-store-survivability-and-abuse-posture.md`.

**Third-party E2EE bolt-ons for Bluesky.** Germ (MLS on AT Protocol, built by an ex-Apple
FaceTime/iMessage engineer) — SECONDARY, T2/3; XMTP (MLS-family, binds Bluesky handles to encrypted
inboxes) — SECONDARY, T3. Relied on by `group-scale-versus-e2ee.md`.

---

## Economic & S-1 figures

**Telegram economics (the worked monetization example).** Relied on by
`operational-rates-and-platform-economics.md`; the model summarized in `platform-dominance-and-adoption.md`.
All **point-in-time**.

- H1 2025 revenue ~$870M, +65% YoY, split $300M Toncoin "exclusivity deals" / $223M Premium / $125M advertising. PRIMARY-announcement as reported, T2.
- H1 2025 net loss ~$222M (vs ~$334M net profit H1 2024) driven by a Toncoin write-down (TON lost ~69% of its value in 2025); operating profit ~$400M stripping the write-down. Reported, T2.
- "$2B revenue / $720M profit in 2025" — a **target/projection**, not a result. SECONDARY, T3. Treat as a stated goal.
- Full-year 2024 "$1.4B revenue, first-ever profit over $500M" — **reported-but-not-audited**. SECONDARY, T3.
- Premium: freemium, launched 2022, ~$4.99/mo (US), 15M subscribers up from 4M in late 2023; feature set per Telegram's own FAQ; ten-level boost ladder; @PremiumBot bypasses the 30% app-store fee. PRIMARY (Telegram FAQ), T1.
- Sponsored Messages appear only in channels above 1,000 members; advertisers buy with Toncoin; Telegram shares half the income with channel owners. PRIMARY / reported, T1/T2.
- Leverage note: recent debt offerings carry bond-to-equity conversion at a discount on IPO; Durov under formal investigation since 2024. SECONDARY, T2/T3.

**Discord platform economics (the S-1 story).** Private company, no audited financials, so every
figure is a third-party estimate — indicative only. **Point-in-time**. Relied on by
`platform-dominance-and-adoption.md`.

- Confidential S-1 filed with the SEC Jan 6, 2026; targeted ~$15B valuation (its 2021 mark); March 2026 debut slipped later. PRIMARY (the filing), T2; valuation indicative.
- Shipped ads in 2025 after nine years of refusing them (opt-in rewarded "Video Quests"). T2.
- Nitro estimated ~$207M in 2023 (~36% of revenue); ~$725M ARR at end-2024 with positive adjusted EBITDA for several quarters; separately ~$561M revenue in 2025 against ~260M MAU (~$2.16 per user annually). SECONDARY, third-party estimates, low-to-moderate confidence.

**Reddit S-1 (SEC EDGAR).** PRIMARY (the filing). T1, **point-in-time**. Bounds aggregate user growth
(used as an upper bound in `operational-rates-and-platform-economics.md`); the 2024 AI-data-licensing
move alongside the IPO filing is cited as a lifecycle case in `platform-dominance-and-adoption.md`. T2 for the case.

---

## Frameworks & analyses

**Aggregation Theory — Ben Thompson (Stratechery, 2015), with "Defining Aggregators" (2017).**
Framework, cited by name. Relied on by `aggregation-theory-and-the-enshittification-shield.md`.

- The demand-side account of how the centered platforms dominate; the three aggregator levels and the two named counters (a new interface paradigm, or decentralized protocols that drop switching costs to zero). PRIMARY (Thompson's essays) — but the levels/counters framing is **[confirm]** (dialogue-sourced, verify against Thompson's primary essays before external use).
- Verbatim quote "Zero distribution costs. Zero marginal costs. Zero transactions. …" — attributed to Aggregation Theory as a body of work. **[confirm]** exact wording against Thompson's primary Stratechery text before external use.

**Enshittification — Cory Doctorow (November 2022 formulation; American Dialect Society 2023 Word of
the Year).** Framework, cited by name; the platform lifecycle (good to users → shift value to business
customers → claw value back → decline). PRIMARY for the definition and provenance, T1. Relied on by
`platform-dominance-and-adoption.md` (read strictly as a lifecycle shape) and
`aggregation-theory-and-the-enshittification-shield.md` (the switching-cost hinge and the
"enshittification shield" inversion).

**Enshittification lifecycle recurrence case set.** GeoCities (2009), Yahoo Groups (2019–2020), Digg
v4 (2010), Reddit's 2023 API enclosure and 2024 AI-data licensing, Vine (2017), LiveJournal→Russia
(2016–2017), Twitter/X (2022). SECONDARY / mixed, T2 across cases, high confidence on the events.
Relied on by `platform-dominance-and-adoption.md`.

**The universal trade (corpus-synthesis finding).** No deployed system delivers usability,
decentralization, and metadata protection simultaneously; the empirical version is narrower (no
production deployment delivers all at once). Analysis, T2, corpus synthesis, moderate-to-high
confidence. Relied on by `platform-dominance-and-adoption.md`.

**Formal cryptographic analyses.** All PRIMARY (the paper itself), tiers as noted.

- Albrecht, Dowling & Jones (King's College London), "Formal Analysis of Multi-device Group Messaging in WhatsApp," IEEE S&P / Springer — message payloads remain E2EE but clients trust the server for the member list, so the server can inject a member. T1/2. Relied on by `group-scale-versus-e2ee.md`.
- Balbás, Collins & Gajland, "WhatsUpp with Sender Keys?" IACR ePrint 2023/1385 — an adversary with server-level control can add users without any member's authorization. T1. Relied on by `group-scale-versus-e2ee.md`.
- Rösler, Mainka & Schwenk, "More is Less" (2018) — the earlier result the above trace to. T2. Relied on by `group-scale-versus-e2ee.md`.
- Ginesin & Nita-Rotaru (Northeastern), PROVERIF analysis of Olm+Megolm, arXiv 2408.12743 — comparable to Signal+Sender Keys with signed pre-keys (which Matrix mandates); notes the Matrix Foundation intends to phase Olm/Megolm out for MLS. T1/2. Relied on by `group-scale-versus-e2ee.md`.
- Survey, arXiv 2401.09102 — puts a number on the pre-MLS cost: naive pairwise E2EE needs ~N² key-exchange messages (a 500-member group ~250,000). T2. Relied on by `group-scale-versus-e2ee.md`.
- Matthew Green (cryptographyengineering.com) on XChat — private keys stored on X servers (sharded via Juicebox), unlocked by PIN, no forward secrecy; if X controls the key store it can decrypt. PRIMARY (named cryptographer's published analysis), T2. Grounds the "disputed, not established" verdict in `group-scale-versus-e2ee.md`.

**AI-surfaced design-dialogue framings.** Analysis, all **[confirm]** (AI-surfaced, verify against
primary sources before external use). Relied on by the two lesson/analysis docs.

- "App stores don't care if your transport is decentralized; they care if your governance is." — `app-store-survivability-and-abuse-posture.md`.
- "trapped by the math" (the zero-switching-cost operator) and "the Red Hat of social infra" (where the value bet moves) — `aggregation-theory-and-the-enshittification-shield.md`.

---

## Security advisories & incidents

No CVE-numbered advisory appears in the fenced layer (the CVE-2025-49090 Matrix state-reset item lives
in the `drystone-spec/` layer, not here). The fenced incidents are the following.

**Discord third-party vendor breach (~September 2025).** Exposed data for users who had contacted
support, including ~70,000 government-ID photos collected for age verification. SECONDARY / reported,
T2, high confidence, **point-in-time**. Relied on by `platform-dominance-and-adoption.md`.

**The Rave removal (Apple App Store, August 2025).** A cross-platform decentralized co-viewing app
removed — read either as a content-moderation failure or an anti-competitive pretext. SECONDARY,
**[confirm]** (AI-surfaced, needs a primary-source pass). Relied on by
`app-store-survivability-and-abuse-posture.md`.

- Developer disputed the removal and filed antitrust suits in five countries (May 2026). **[confirm]**.
- Disabling the platform's single-sign-on reportedly locked ~11.4 million users out of years-old accounts. **[confirm]**, **point-in-time**.

**Telegram founder arrest and disclosure reversal.** Durov's arrest in France (August 2024); Telegram
reversed a long-standing "never hand over data" stance and began sharing user data on valid legal
orders; formal investigation ongoing since 2024. SECONDARY / reported, T2/T3. Relied on by
`app-store-survivability-and-abuse-posture.md` (the be-readable-so-you-can-be-compelled point) and the
Telegram economics leverage note in `operational-rates-and-platform-economics.md`.

---

## Coverage / confidence note

**Strongest (PRIMARY, standards / peer-reviewed / official filings):** RFC 9420 and RFC 9750; the
per-platform capability primaries (Apple Support, Telegram FAQ, platform specs); the WhatsApp and
Matrix formal cryptographic analyses (Albrecht/Dowling/Jones; Balbás/Collins/Gajland; Rösler et al.;
Ginesin & Nita-Rotaru); the operational-rate studies (arXiv 2503.02661, 2205.14529, 2106.05184,
2410.21996, 2304.10777; ICWSM 2024; CAT Lab); the enshittification definition (Doctorow); the S-1
filings (Discord Jan 6 2026, Reddit).

**Use-with-caveat (SECONDARY / triangulated / point-in-time):** every private-company financial
(Discord ARR/revenue/MAU/valuation are third-party estimates, not audited primaries; Telegram's
FY2024 profit is reported-but-not-audited); the Facebook group-size maxima and platform-math means
(tracker- and math-tier); Discord's TeamSpeak-comparison and 2017 metrics; the Thompson quote and the
levels/counters framing (dialogue-sourced pending a Stratechery primary pass).

**Lowest confidence in the layer (member-ban rate):** the per-group member-ban rate stays **LOW** and
largely inferential — there is no clean T1 per-group-per-day anchor; the ~1.9-actions-per-subreddit-per-day
figure (arXiv 2205.14529) skews to large, active, logging-enabled communities and does not push down to
the unmeasured small-community case. The defensible range spans zero-per-week to 100+/day by size band;
carry the widest safety margin here.

**Projections and unaudited targets, never laundered into fact:** Telegram's "$2B revenue / $720M
profit in 2025" is a target/projection; the FY2024 "$1.4B / >$500M profit" is reported-but-not-audited.

**Most notable low-confidence / `[confirm]` cluster:** the entire Rave / Apple-removal case
(`app-store-survivability-and-abuse-posture.md`) is AI-surfaced and flagged `[confirm]` — the August
2025 removal, the five-country antitrust suits (May 2026), and the ~11.4-million-user SSO lockout all
need a primary-source pass before external use. The one verbatim Ben Thompson quote and the
dialogue-coined framings ("trapped by the math," "the Red Hat of social infra") carry the same flag.
Preserve every `[confirm]` marker until the primary is pulled.
