# Operational rates and platform economics (the internal dynamics of the fenced field)

register: descriptive / quantitative. The map of how communities behave inside the fenced platforms, and how the platforms are paid for. No harm argument; that reading lives elsewhere.

This doc records two things about the fenced field's interior: the per-group operational rates (how often members get banned, how often they join, and what fraction of a roster is ever live) and the platform economics (how a centered platform turns those rosters into revenue, worked through Telegram). Both are quantitative maps, tier-tagged and confidence-marked. Where the evidence cannot pin a number, that is stated as plainly as the number itself.

Source-tier key, used on every figure below:

- **T1**: standards body, spec, or platform primary docs.

- **T2**: peer-reviewed or arXiv work, or direct reporting of a primary announcement.

- **T3**: secondary commentary or trackers; corroborates, never founds.

## Scope

In scope: the three per-group operational rates (member-ban, member-join, live-fraction) triangulated across the tiers with confidence stated, and the monetization model of a centered platform (Telegram as the worked example: revenue mix, the Toncoin dependency and its accounting split, the narrow-advertising model, and the Premium/boost model).

Out of scope: the capability map (roster / call / broadcast ceilings and the E2EE-vs-scale tradeoff) is a separate doc in this layer. The harm reading of what these rates and this economics *mean* is a different layer's register and is not attempted here. This doc draws the interior of the map; it does not argue about it.

Boundary note on what a "rate" is here: every quantity below is a rate or ratio *with a scope*, and the scope governs the digits. Three guardrails hold throughout, and they are the most common way these numbers get misused:

- **Member-removal is not content-removal.** Removing a person (a ban) and removing a post or comment are different events that differ by one to two orders of magnitude. Content removal can touch a meaningful share of all content (Reddit's mods and admins removed 162,135,309 pieces of content from a base over 5.3 billion in H1 2024, T1; about 8.5% of comments in one active political subreddit, T2), while member bans touch a low-single-digit percent of monthly-active participants and a far smaller fraction of the full roster. Every figure below is tagged as one or the other.

- **Active is not member-count, and the activity window defines "active."** "Online now," "posted this month," and "visited in seven days" are different denominators that can differ by an order of magnitude. Every live-fraction figure carries its window.

- **Tier 3 corroborates but never founds.** A secondary tracker or vendor estimate can sanity-check a Tier 1 or Tier 2 anchor; it never establishes a rate on its own.

## The distribution shape governs which statistic is meaningful

Lead with the shape, because it decides whether a mean is even a legitimate thing to report. Users-per-community and comments-per-community are heavy-tailed; comments-per-user follows a power law with exponent about -1.44:

> 20% of users contribute 84% of the comments; and approximately 50% of users make only one comment in a whole month on Reddit.

Attributed to arXiv 2503.02661 (T2).

The consequence: most communities are tiny and nearly dormant, while a small number of very large communities dominate every platform aggregate. A single headline rate is meaningless. Report medians and ranges by size band (small, median-active, large), and treat bursts as a separate regime from steady state. All three rates below are broken down this way.

## Part 1: the three per-group operational rates

### Rate 1: member-ban rate per group per day (LOW confidence)

**Best estimate.** For a mature, mid-sized community, member bans run substantially less than one per day in steady state; large communities run a few to tens per day; very large communities run on the order of 100+ per day. Confidence is LOW and the quantity stays largely inferential. Dominant tier: T2 (a single private-log study), with no clean T1 community-level anchor.

**Distribution-aware breakdown.**

- Small (~1K roster): assume well under one ban per day; small-community ban rates are essentially unmeasured in the literature.

- Median-active (~50K): substantially less than one per day up to a few per day.

- Large (millions): up to tens per day, with 100+ per day at the extreme.

- Burst: moderation actions spike with brigading and controversy events; provision a 2-5x multiplier over steady state.

**Triangulation.**

- Tier 1 upper bound: Reddit's Transparency Report H1 2024 (T1) gives sitewide content-removal totals, but its community-level *member-ban* count is locked in a chart image and cannot be read as a clean per-group-per-day figure. So the Tier 1 anchor bounds content actions, not member bans, and the division from sitewide totals down to per-group-per-day carries the unstated assumption that bans distribute across communities the way content does, which the distribution shape says they do not.

- Tier 2 anchors: arXiv 2205.14529 (Li, Hecht & Chancellor, ICWSM 2022, T2) analyzed private moderator logs for 126 subreddits, 900+ moderators, and 3M+ actions, recording roughly 220 human plus 20 bot ban/unban actions per day across the sample, which normalizes to about 1.9 ban actions per subreddit per day over the active, logging-enabled communities. Separately, CAT Lab (Matias, Platt & Gilbert 2024, T2/3) reports r/politics banning about 4,000 accounts per month (about 130 per day) against roughly 51,000 monthly participants, about 7.6% of that monthly base (the study's separately reported "1.9% of all posters and commenters per month" uses a larger accumulated denominator, which is the active-is-not-member-count guardrail in action).

- Tier 3 corroboration: subreddit-analytics vendors provide directional ban/removal figures; used only to sanity-check direction, independent of the T2 logs and not treated as founding evidence.

**Residual.** The 1.9-actions-per-subreddit-per-day anchor skews to large, active, logging-enabled communities and cannot be pushed down to the small-community case, which is unmeasured. There is no clean Tier 1 per-group figure. The defensible range spans from zero-per-week to 100+ per day depending on band. This is the thinnest of the three rates and should carry the widest safety margin in any design that leans on it.

### Rate 2: member add/join rate per group per day (MEDIUM confidence)

**Best estimate.** Mature communities grow at low-single-digit percent per month in steady state, which translates to a handful of joins per day for small groups up to low hundreds per day for large ones. Confidence is MEDIUM. Dominant tier: T2, with a cross-platform T2 anchor available.

**Distribution-aware breakdown.**

- Small (~1K roster): a few joins per day.

- Median-active (~50K): tens of joins per day.

- Large (millions): low hundreds of joins per day.

- Burst: dramatic and short-lived. r/popular front-page exposure produced "70 times the number of newcomers" versus matched non-front-page posts (Understanding Community Resilience, 6,306 posts, 1,320 subreddits, ICWSM 2024 / AAAI, T2). A platform-wide shock does the same: Reddit new users rose 52% year-over-year at COVID onset (arXiv 2304.10777, T2). Bursts are one-to-two-order-of-magnitude transient spikes over steady state.

**Triangulation.**

- Tier 1 upper bound: platform growth totals (Reddit S-1 on SEC EDGAR, T1) bound aggregate user growth, but dividing sitewide growth to a per-group-per-day join rate assumes joins distribute evenly across communities, which the heavy tail contradicts; the large communities absorb most of the joins.

- Tier 2 anchors: the r/popular front-page study (6,306 posts, 1,320 subreddits, ICWSM 2024, T2) gives the burst multiplier (70x); arXiv 2304.10777 (T2) gives the platform-shock magnitude (+52% YoY). Steady-state growth of low-single-digit percent per month is the consensus shape across these.

- Tier 3 corroboration: subreddit-analytics vendors (T3) provide directional join/subscriber trend lines; corroboration only.

- Cross-platform anchor (WhatsApp groups): arXiv 2106.05184 (T2), a moderation study of 5,051 groups and 2.6M messages over 302 days, recorded 437,000 membership-action events (61k added by a member, 73k added by an admin, 132k joined via invite link, 154k left, 9k removed). Normalized, that is about **0.29 membership actions per group per day** averaged across ordinary (non-viral) groups, with adds and voluntary leaves dominating and admin removals rare. Because it is a different platform with a different join mechanism, it is an independent sanity check that steady-state per-group join/leave churn is low in absolute terms, and it independently corroborates the member-removal-is-rare guardrail (9k removals against 437k total actions).

**Residual.** The steady-state rate is well-supported in shape but definition-sensitive: "join" bundles invite-link joins, admin adds, and member adds, which the WhatsApp anchor shows have very different magnitudes. Burst behavior is well-characterized in direction (70x, +52%) but not in duration or decay, so provisioning must assume the spike is possible without knowing how long it lasts.

### Rate 3: live fraction as a function of total roster (MEDIUM-HIGH on shape)

**Best estimate.** The live (recently-active) fraction sits in the low single-digit percent of the roster and shrinks as the roster grows. Confidence is MEDIUM-HIGH on shape (the direction and rough magnitude are robust), lower on any single percentage because the activity window defines the number. Dominant tier: T2, with T1 platform-metric corroboration.

**Distribution-aware breakdown (windows stated).**

- Small (~1K roster): 5-15% active.

- Median-active (~50K): 1-5% active.

- Large (millions): well under 1% active.

- The classic 90-9-1 shape (NN/g, T2/3): about 1% creating, about 9% engaging, about 90% lurking. "Online now" is well under 1%; "posted this month" is a larger window and a larger fraction.

**Triangulation.**

- Tier 1 upper bound: platform-reported activity metrics (Reddit's shift from subscriber counts to "Visitors," a seven-day-unique measure, T1; Discord roughly 259M monthly-active against roughly 656M registered, about 39-40%, T1/T3) bound the active fraction from above. The MAU-over-registered ratio is a coarse ceiling: it is platform-wide, not per-group, and MAU is a wide window, so it overstates the per-group "live now" fraction.

- Tier 2 anchor: arXiv 2410.21996 (T2) found a size-independent active core (on the order of 2,000 heavy contributors) across very-differently-sized subreddits. This is the mechanism behind the shrinking fraction: if the active core is roughly constant in absolute terms, its share of the roster collapses as the roster grows. It ties directly to the distribution-shape finding (20% of users make 84% of comments).

- Tier 3 corroboration: analytics vendors and the 90-9-1 heuristic (T2/3) corroborate the rough proportions; independent of the arXiv core-size finding and used only to confirm shape.

**Residual.** The number is only as meaningful as its window, and "active" is reported against incompatible windows (online-now, seven-day, monthly) that differ by an order of magnitude. The size-independent-core finding is a strong shape result but comes from Reddit; whether the absolute core size transfers to other platforms is not established. What is solid: never size live infrastructure to the roster. Presence and fan-out load tracks the active fraction (roughly 1-10%, often less), and real-time subsystems track a call-concurrency ceiling that sits two-to-three orders of magnitude below the roster.

### Platform data point: Facebook group-size distribution (a fenced platform)

A clean external instance of the heavy-tailed shape the whole doc rests on, on a platform where the numbers are unusually hard to pin. Meta publishes no official average or median (it treats backend group-size statistics as effectively secret), so every figure below is triangulated across platform metrics, academic scrapes, and admin trackers, and is mixed-tier. Because the distribution is heavy-tailed, the three summary statistics must be reported separately; a single headline number is meaningless here for the same reason it is in the size-band breakdowns above.

- **Maximum: about 8.3M-8.6M members (T3, tracker-tier, indicative only).** There is no software cap on join count; named giants include an English-learning group at about 8.6M (private) and a relationship-counseling group at about 8.3M (public). Groups past roughly 1M typically run 5-10+ moderators plus heavy admin-automation. This bounds the tail; it does not describe a typical group.

- **Mean, distorted upward: about 1,000-2,500 members from global platform math (T3), and about 8,727 (public) / 13,277 (private) from academic cross-section scrapes (T2).** The platform-math figure divides 10M+ groups against roughly 1.8-2.5B monthly-interacting users (average user in 5+ groups); the scrape figures are inflated by top-tier viral groups in the sample. The mean is the wrong statistic here for the same reason the doc reports medians and ranges everywhere else.

- **Median, the honest typical: about 1,400 members for niche/thematic groups (IQR about 765-2,800, JMIR-style mapping across hundreds of active groups; T2, the stronger anchor here), dropping to about 25-100 members for high-engagement groups people interact with daily (local, buy/sell, close friends).** Admin rule of thumb: past about 1,400 members a group is larger than half of active thematic groups; past 100,000 it is in the top fraction of a percent.

The design point: this is a clean instance of the two gaps the "most of the roster is dormant, size on the live set" warrant rests on. The mean-vs-median gap (a mean of ~1-2.5k dragged up by multi-million-member outliers against a thematic median of ~1,400) and the membership-vs-daily-engagement gap (thematic ~1,400 versus a daily-interaction median of ~25-100) are the same pattern the live-fraction rate above describes from the platform side, and it feeds spec Part 2 §11.13. Facebook groups (the forum object) are non-E2EE and admin-flat-symmetric.

## Part 2: platform economics (Telegram as the worked example)

Telegram is the worked example because it is a centered community platform that recently turned its rosters into revenue, and its filings expose both the model and its dependency. Telegram is now for-profit and, on the underlying business, profitable, which is a recent shift: roughly its first decade ran on the founders' money and earned essentially nothing.

### The monetization model

H1 2025 revenue was about **$870M, up 65% year-over-year** (T2, reporting of the primary announcement). The split:

- About **$300M** from "exclusivity deals" tied to Toncoin, the ecosystem cryptocurrency.

- **$223M** from Premium subscriptions.

- **$125M** from advertising.

Three distinct engines, then: a crypto-ecosystem deal stream, a freemium subscription, and a deliberately narrow ad product. The subscription and ad engines are examined below; the crypto stream is where the accounting risk concentrates.

### The Toncoin/TON dependency and the accounting split

Toncoin (TON) is woven into Telegram's payment plumbing and is the settlement currency for much of the ecosystem (Premium and Fragment-marketplace purchases rely on TON). Telegram holds Toncoin on its books, so the token's price feeds the reported financials directly.

The accounting split for H1 2025 (T2): a net loss of about **$222M** (against about $334M net profit in H1 2024), driven largely by a write-down after Toncoin lost about **69% of its value** in 2025. Stripping that write-down out, operating profit was about **$400M**, so the underlying business remained profitable before crypto losses. The headline loss is a mark-to-market artifact of the token holding, not an operating result; the operating engine and the balance-sheet crypto exposure move independently.

Two figures here must not be laundered into settled fact:

- The **"$2B revenue / $720M profit in 2025"** figure is a target/projection, not a result (T3). Treat it as a stated goal.

- The full-year-2024 **"$1.4B revenue, first-ever profit over $500M"** comes from secondary commentary and is reported-but-not-audited (T3).

Leverage note (T2/T3): recent debt offerings include bond-to-equity conversion options at a discount if an IPO proceeds, and Durov has been under formal investigation since 2024 over allegations the platform failed to address criminal content.

### The narrow-advertising model

Advertising is deliberately narrow (T1/T2). Sponsored Messages appear only in channels above **1,000 members** (the broadcast surface, not the discussion groups), advertisers buy placement with Toncoin, and Telegram shares **half** the income with channel owners. The design keeps ads off the discussion side entirely and ties the ad economy back into the same TON dependency as the deal stream.

### The Premium / boost model

Premium is freemium (T1), launched 2022, at about **$4.99/month** in the US (about £4.99 UK, about €4.99 EU). Paid subscriptions rose to **15M** from 4M in late 2023.

The feature set (canonical, per Telegram's own FAQ, T1): Doubled Limits, 4 GB File Uploads, Faster Download Speed, Voice-to-Text Conversion, Premium Stickers, Unique Reactions, Advanced Chat Management, Animated Profile Pictures, Profile Badges, Premium App Icons, No Ads, Custom Emoji, Voice Message Privacy Settings, Voice-to-Text for Video Messages, Emoji Statuses, and Real-time Chat and Channel Translation. Concretely, the raised limits double file uploads (2GB to 4GB), allow joining 1,000 channels/groups instead of 500, pinning 10 chats instead of 5, and 20 folders of 200 chats each instead of 10.

The "No Ads" perk is narrow in the same way the ad model is: it hides Telegram's own Sponsored Messages, but promotional content that channel admins post themselves is outside Telegram's control and still appears. "Ad-free" means Telegram's ads, not everything.

The **boost ladder** is the mechanism by which Premium spills into communities. A boost is a Premium subscriber lending their account's weight to a channel or group, gated on a **ten-level** ladder by subscriber count:

- Channels: Level 1 unlocks one story per day, one custom reaction, seven name colors, and seven link/quote styles; Level 10 adds custom backgrounds, 1000+ emoji statuses, and custom logos.

- Groups: a different, utility-flavored ladder, for example Level 6 unlocking unlimited voice-to-text, with higher levels adding group emoji packs, cover colors, and custom backgrounds.

**Perks leak to free users.** Any user can download the 4 GB files a Premium user uploads, watch Premium sticker animations, and tap to increase an exclusive reaction a Premium user already added. A single subscriber raises the ceiling for everyone around them, which is the community-network mechanic that makes the subscription spread.

Distribution economics: subscribing through the official **@PremiumBot bypasses Apple's and Google's 30% app-store fee**, and the saving is passed on as a cheaper direct price. Cancellation degrades gracefully: features stay until the end of the billing period, then lock at the free ceiling (extra channels and folders remain but cannot grow, pinned chats drop back to five, the badge disappears), and nothing created is deleted.

## What this doc establishes (and does not)

Establishes the interior of the fenced-field map: the three per-group operational rates with their confidence and dominant tiers made explicit (member-ban rate stays LOW and inferential; join rate is MEDIUM; live-fraction is MEDIUM-HIGH on shape), the distribution shape that makes size-banded medians the only honest statistic, and a worked platform-economics example (Telegram) showing how a centered platform monetizes its rosters and where the accounting risk sits.

Does not argue what any of these numbers mean in harm terms (a different layer's register), does not draw the capability map of rosters and E2EE stance (a separate doc in this layer), and does not launder the flagged figures: the member-ban rate remains inferential, the "$2B / $720M" figure remains a projection, and the full-year-2024 profit remains reported-but-not-audited.
