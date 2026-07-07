# Raw transcript: Facebook group-size data (avg / median / max), 2026-07-07

`Provenance caveat (PLAYBOOK §4): content-faithful cleaned paste, not a byte-pristine export. Data on a
fenced-field platform (Facebook groups), augmenting the fenced operational-rates map. Source quality is
mixed (Meta publishes no official average/median; figures are triangulated from platform metrics, academic
scrapes, and admin trackers). Heavy-tailed distribution warning applies: report max, mean, and median
separately; the mean is distorted upward by outliers, the median is the honest "typical".`

## Headline

- **Maximum:** ~8.3M-8.6M members (community-tracking ceiling). No software cap on join count. Named giants:
  an English-learning group ~8.6M (private); a marriage/relationship-counseling group ~8.3M (public); a
  "Female Problems" group ~8.0M (public). Groups past ~1M typically need 5-10+ moderators and heavy
  "Admin Assist" automation. (Tracker-tier; indicative.)

- **Average (mean), distorted:** global platform math ~1,000-2,500 members/group (10M+ groups, ~1.8-2.5B
  users interacting monthly, average user in 5+ groups). Academic cross-section scrapes: public groups
  ~8,727, private groups ~13,277 (inflated by top-tier viral groups in the sample).

- **Median (the "real" typical group):** niche/special-interest median ~1,400 members (IQR ~765-2,800,
  from a JMIR-style mapping across hundreds of active groups). High-engagement median (groups people
  actually interact with daily: local, buy/sell, close friends) ~25-100 members. Admin rule of thumb: >1,400
  members puts you larger than half of active thematic groups; >100,000 is top-fraction-of-a-percent.

## Source-tier notes

- Meta treats backend group-size statistics as effectively secret; no official average/median is published.

- The picture is assembled from (1) global platform metrics (Meta-reported group/user counts), (2)
  academic web-scraping studies (the public/private-group averages), and (3) admin-community tracking and
  trackers (the max giants, the daily-engagement median). Trackers and roundup pages are indicative-only
  (T3); the academic medians (JMIR-style) are the stronger anchor.

Design relevance for the fenced map and the spec: Facebook groups are a clean example of the heavy-tailed
size distribution the design's premises rest on (Part 2 §11.13): a handful of multi-million-member outliers,
a mean of ~1-2.5k dragged up by them, and a *typical* (median) group of ~1,400 for thematic communities and
~25-100 for the groups people actually engage with daily. The mean-vs-median gap and the
membership-vs-daily-engagement gap are exactly the "most of the roster is dormant; size on the live set"
warrant. Facebook groups (the forum object) are non-E2EE and admin-flat-symmetric (see the fenced
capability map and the §11.13 governance-shape bracket).
