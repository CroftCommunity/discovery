# Topic-Driven Aggregation Platform: Prototype Build Spec

> **Provenance & status.** The Reddit-family "build spec" that the **Graze persona-switch spec**
> (`graze-persona-switch-spec.md`) re-substrates onto the behavior-scale methodology. Authored in a
> claude.ai design dialogue (2026-07), filed **content-faithful** (cleaned-paste; PLAYBOOK §4). It
> extends a structural blueprint into build-ready detail: routes, screen specs, data model, API
> surface, governance workflows, ranking math from Reddit's open-sourced code, and an M0–M3 milestone
> plan, with a Slashdot / Lobsters / Hacker News baseline. The deeper competitive-research version is
> `research/reddit-aggregation-ux-slashdot-baseline-2026-07.md`. Raw session:
> `seeds/transcripts/raw/stellin-graze-behavior-scale-sessions-2026-07.md`.

---

Extends the "structural blueprint" document (three-column layout, post unit, nested comments, submission workflow) into something a small team can build against. Adds: a grounded baseline comparison against Slashdot, Lobsters, and Hacker News; the actual ranking math from Reddit's open-sourced code; routes; screen specs; data model; API surface; governance workflows; and a milestone plan.

## 0. How to read this document

Claims are marked by status:

- **[source]** links follow claims taken from a primary source retrieved for this spec (official site docs, source code).

- **[secondary]** marks claims corroborated only by blogs, wikis, or community docs. Treat as probably-true, verify before relying on them.

- Everything unmarked under "Proposal" headings is our design, not a fact about any existing site.

## 1. Product goal and core user stories

The user story we are building toward, in priority order:

1. **Reader:** "I open the site and see a churning feed of the best recent posts from topics I chose, and I can collapse my way through a 1,000-comment thread in under a minute."

2. **Contributor:** "I post into a specific community, its rules are shown to me before I hit publish, and honest feedback (votes, replies) arrives without an editor gatekeeping me."

3. **Community founder:** "I can create a space, write its rules, and enforce them with tools that leave a public record, so members trust the moderation."

4. **Operator (us):** "Abuse is rate-limited and auditable without requiring paid staff to read every post."

Everything below serves these four. Where a feature serves none of them, it is cut from the prototype.

## 2. Working vocabulary

Defined here once so the rest of the doc can lean on them. Mapping shown so the comparison sections stay readable.

| Our term | Meaning | Reddit equivalent | Slashdot equivalent |
|---|---|---|---|
| Node | A topic community with its own rules, mods, feed | Subreddit | (none: one shared comment space) |
| Post | Top-level submission into a Node (text, link, or media) | Post | Story (but editor-selected) |
| Thread | A Post plus its comment tree | Thread | Story discussion |
| Rating | A user's +1 or -1 on a post or comment | Upvote / downvote | Moderation (+1/-1 with adjective) |
| Score | Net rating on an item (ups minus downs) | Score | Comment score (clamped -1 to +5) |
| Reputation | Per-user accumulated standing, split post/comment | Karma | Karma (worded tiers) |
| Tag | Node-scoped label on a post | Flair | (site topic sections) |
| Steward | Volunteer moderator of a Node | Moderator | (rotating mod-point holders) |
| Audit log | Public per-Node record of moderation actions | Mod log (mostly private) | (none) |

## 3. Baseline comparison: Slashdot, Lobsters, Hacker News

The original blueprint compared against X, Facebook, and Discord. Those are different species. Slashdot, Lobsters, and HN are the same species (link/discussion aggregators), so the differences here are the actual design decisions we must take a position on.

### 3.1 Slashdot: scarcity-based moderation instead of universal voting

Grounded in Slashdot's own moderation document and FAQ ([moderation.shtml](https://slashdot.org/moderation.shtml), [karma FAQ](https://slashdot.org/faq/karma.shtml), [metamod FAQ](https://slashdot.org/faq/mod-metamod.shtml)):

- **Editorial front page.** Users submit stories; editors select what runs. The community layer is the comments, not the feed. This is the deepest structural difference from the Reddit model, where the community selects the feed itself via voting. **[source]**

- **Moderation is a scarce, rotating privilege, not a universal right.** Eligible users (logged in, mid-pack regular readers, not among the newest accounts, willing to serve, non-negative karma) are occasionally granted moderator access. They receive 5 points; each moderation spends one; unused points expire after 3 days, explicitly to prevent stockpiling points for topics you care about. **[source: moderation.shtml]**

- **Typed ratings.** A moderation is an adjective from a drop-down (Insightful, Informative, Flamebait, Troll, etc.). Good adjectives add one point, bad adjectives subtract one. So every score change carries a stated reason. **[source: moderation.shtml]**

- **Bounded scores.** All comments live on an absolute scale from -1 to +5. Logged-in users' comments start at 1 (varying 0 to 2 with their standing); anonymous comments start at 0. **[source: moderation.shtml]** The open-source Slash engine's documented defaults: karma bounded -25 to 50, "good karma" threshold 25 unlocks a +1 posting bonus, "bad karma" threshold -10 imposes a penalty. **[secondary: O'Reilly, "Running Weblogs with Slash"](https://www.oreilly.com/library/view/running-weblogs-with/0596001002/ch06s05.html)**

- **Conflict-of-interest rule.** You cannot moderate and post in the same discussion, and you do not get points back if you post afterward. **[source: moderation.shtml]**

- **Metamoderation.** A second layer where logged-in users "rate the rating" of randomly selected past moderations; only accounts among the oldest 92.5% may participate. Metamod outcomes feed back into karma and moderator eligibility. **[source: metamod FAQ, karma FAQ]**

- **Reader-side filtering.** Each reader sets a score threshold (-1 to +5); comments below it are hidden for that reader. Filtering is a reader preference, not a removal. **[source: Slashdot FAQ]**

- **Anti-abuse precedents worth copying:** one comment per user per 120 seconds; a user moderated down several times in a short span gets a temporary posting ban ("cooling off"). **[source: karma FAQ]**

### 3.2 Lobsters: transparency as the product

- Invite-only membership with a **public invitation tree**; each profile shows who invited whom, creating an accountability chain. New users are restricted for their first 70 days (cannot send invites, flag, etc.). **[source: about-page text quoted in the official repo issue tracker](https://github.com/lobsters/lobsters/issues/2013)**

- **Public moderation log** covering edits, deletions, and bans; explicit rejection of shadow banning; flags require a typed reason (off-topic, already posted, broken link, spam, me-too, troll, unkind). **[secondary: [syften.com overview](https://syften.com/blog/lobsters-hacker-news-alternative/), [Tildes discussion](https://tildes.net/~tildes/1qq/tildes_versus_lobste_rs)]** I could not fetch lobste.rs/about directly (it blocks automated access), so the mod-log details rest on secondary sources plus the repo.

- **Tags instead of sub-communities.** One shared front page, filterable by tag. No per-community governance. This is the main structural fork from our model.

- The entire site is open source: [github.com/lobsters/lobsters](https://github.com/lobsters/lobsters) (Rails). Worth reading as a reference implementation of votes, flags, and a public mod log; check the repo's LICENSE before reusing code.

### 3.3 Hacker News: karma-gated abilities

- Submissions cannot be downvoted; comments can, but only by users above a karma threshold (staff have stated it as 501). **[secondary: [HN thread with moderator confirmation](https://news.ycombinator.com/item?id=7606145), [hacker-news-undocumented](https://github.com/minimaxir/hacker-news-undocumented)]**

- Flagging is the community's negative signal on stories, available at a low karma bar (stated as ~30 in a YC blog post), with moderators reviewing flags and revoking abuse. **[source: [YC blog, "An Update on Hacker News"](https://ycombinator.com/blog/an-update-on-hacker-news)]**

- Comment scores are hidden from everyone but the author, start at 1, and floor at -4. **[secondary: hacker-news-undocumented]**

### 3.4 Decisions this forces on our baseline (Proposal)

| Axis | Slashdot's answer | Our baseline | Why |
|---|---|---|---|
| Who selects the feed | Editors | The crowd, via ratings + time decay | The churning community-driven feed is the product |
| Who can rate | Rotating 5-point moderators | Every account, unlimited | Universal voting is what makes feed self-selection work |
| Rating vocabulary | Typed adjectives | Plain +/- on content; **typed reasons on reports and flags** (Lobsters-style) | Keeps voting frictionless, keeps enforcement legible |
| Score range | Clamped -1..+5 | Unbounded internally; optionally cap the **displayed** score | Ranking math needs magnitude; display caps are a later anti-pile-on experiment |
| Comment start score | 0-2 by karma | 1 for everyone | Reputation-weighted starting scores are a Phase 3+ experiment, not baseline |
| Moderator accountability | Metamoderation | **Public per-Node audit log** (Lobsters-style) at launch; metamod-style review deferred | A public log is cheap to build and buys most of the trust |
| New accounts | Excluded from moderating | Probation window: limited posting rate, no Node creation, no report weight | Direct lift from Slashdot eligibility + Lobsters' 70-day restriction, scaled down |
| Anonymity | Anonymous Coward posting | No anonymous posting in prototype | Cuts an entire abuse surface for v1 |
| Reader-side thresholds | Core feature | Adopt a light version: per-user "hide comments below score X" setting | Cheap, and it reframes downvoting as filtering rather than deletion |

## 4. Information architecture (Proposal)

Route map. `:slug` is a URL-safe Node name, `:id` a short base36 identifier.

```
/                          Home feed (subscribed Nodes; logged-out: sitewide popular)
/all                       Everything, sitewide
/popular                   Sitewide, quality-filtered
/n/:slug                   Node feed (default sort: hot)
/n/:slug/new|top|rising    Node feed, explicit sort (top takes ?t=day|week|month|year|all)
/n/:slug/p/:id/:titleSlug  Thread page
/n/:slug/p/:id/c/:cid      Comment permalink (renders focused subtree + ?context=N ancestors)
/n/:slug/about             Rules, description, steward roster
/n/:slug/mod/queue         Reports + held items (stewards only)
/n/:slug/mod/log           PUBLIC audit log
/n/:slug/mod/settings      Node settings, tags, automod rules, bans (stewards only)
/submit                    Submission wizard (Node picker first)
/n/:slug/submit            Wizard with Node preselected
/u/:handle                 Profile: overview | posts | comments | saved (saved: self only)
/create-node               Node creation (gated by account age + reputation)
/search?q=&scope=          Sitewide or Node-scoped search
/notifications             Replies, mentions, mod messages
/settings                  Account, feed prefs (incl. score threshold), content filters
```

## 5. Screen specifications (Proposal)

Each screen: purpose, regions, key states. Component names are the frontend inventory.

### 5.1 App shell (all screens)

- `HeaderBar` (sticky): logo, `ScopedSearchBox` (searches current Node when inside one, with a "search everywhere" escape), `CreatePostButton`, `NotificationBell` with unread count, `ProfileMenu` (theme, settings, logout).

- `LeftNav` (desktop only; becomes a drawer under 1024px): Home / Popular / All links, then `NodeList` of joined Nodes sorted by recent activity, then "Create a Node".

- Mobile (<768px): single column; `RightRail` content moves to a collapsible block at the top of Node feeds; `LeftNav` behind a hamburger.

### 5.2 Feed screens (Home, All, Popular, Node)

- `FeedSortBar`: Hot | New | Rising | Top(+timeframe). Sort is in the URL so links are shareable.

- `PostCard` anatomy (matches the blueprint's Post Unit, made concrete):

  - `VoteControl`: chevron up, score, chevron down, left-aligned. States: idle, mine-up, mine-down, disabled (logged out: clicking opens auth prompt).

  - `PostMeta`: Node chip, author handle, relative timestamp, tag chip, NSFW/Spoiler badges.

  - `PostPayload`: title (link posts: title links out, domain shown beside it), then text preview clamped to 4 lines, or media with click-to-expand. Spoiler media blurred until tapped.

  - `PostActionRow`: comment count (links to thread), share (copies canonical URL), save toggle, report, and for stewards an inline `ModActionMenu` (remove, lock, pin).

- Feed loading is **cursor pagination**: the server returns an opaque `nextCursor` token encoding the last item's rank; the client requests the next page with it. (Offset pagination breaks when the feed reorders under you; cursors do not.) Infinite scroll with a "back to top" affordance.

- States: skeleton loading, empty ("This Node has no posts yet" + submit CTA), error with retry, logged-out home (redirects to /popular with a banner).

### 5.3 Thread screen

- Post rendered full-width at top (no clamp), then `CommentSortBar`: Best | Top | New | Controversial | Q&A. "Best" is the confidence sort defined in section 7.3.

- `CommentComposer` directly under the post (disabled with reason when thread is locked).

- `CommentTree`: recursive `CommentNode` components. Each node: vote control (compact), author, score, timestamp, body, action row (reply, share, report, collapse).

- **Collapse gutter:** the vertical line spanning a comment's children is itself the collapse control (full-height hit target, minimum 24px wide on touch). Collapsing shows a one-line stub: author, score, "N children hidden". This interaction is the signature element of the UI; make it excellent.

- **Continuation stubs:** the server returns at most `MAX_INITIAL = 200` comments per page and at most depth 10. Beyond either limit, render a `MoreRepliesStub` ("load 34 more replies") that fetches the pruned subtree, and past depth 10 a "continue this thread" link to the comment-permalink route.

- Comment permalink view renders the focused comment highlighted, `?context=N` ancestors above it, and a "view full discussion" link.

### 5.4 Submission wizard

Four linear steps, exactly as the blueprint prescribes, with the enforcement points made explicit:

1. **Node picker.** Search-as-you-type across joined Nodes first, then all public Nodes. Selecting loads that Node's rules card and posting requirements into the wizard.

2. **Format tabs.** Text | Link | Media. Tabs the Node has disabled render locked with a tooltip naming the rule. Link tab fetches the URL server-side for a title suggestion and duplicate detection ("posted 2 days ago in this Node" with a link).

3. **Draft.** Title (limit 300 chars), body (Markdown with preview), required tag picker if the Node mandates tags, NSFW/Spoiler toggles.

4. **Validate and publish.** Client-side checks first, then the server runs the Node's automod rules (section 10.3). Outcomes: published; held for steward review (say so explicitly); rejected with the specific rule named. Never fail silently.

### 5.5 Steward screens

- `ModQueue`: table of open reports and automod-held items. Row: content preview, report reason counts, reporter count, age. Actions: approve, remove (reason required, from the Node's removal-reason list plus free text), lock, ban author (duration + reason). Keyboard: j/k to move, a/r to act.

- `AuditLog` (public): reverse-chronological `mod_actions` (section 8): timestamp, steward handle, action, target link (or "[removed]" placeholder for the content body), reason. Filterable by action type and steward.

- `NodeSettings`: description, rules editor (ordered list, each rule has a short name used in report dialogs), tag manager, allowed post formats, automod rule editor, banned-user list.

### 5.6 Visual identity note

Do not clone Reddit's trade dress (the blueprint already says this). Concretely: pick a palette and type pairing specific to this product, keep voting affordances as plain chevrons, and spend the design boldness on the collapse-gutter interaction rather than on decoration. Density is the aesthetic: this UI is judged by how many readable items fit on one screen.

## 6. Interaction mechanics (Proposal)

- **Voting is optimistic UI:** the client updates the score and arrow state immediately, sends the request, and rolls back with a toast on failure. (Optimistic UI: render the assumed success before the server confirms.) Clicking your active arrow retracts the vote (server deletes the row). Votes are switchable any time.

- **Logged-out users** see scores and can read everything public; any write action opens the auth sheet with the intended action preserved and replayed after login.

- **Keyboard (desktop):** j/k next/prev post, a/z upvote/downvote, Enter open thread, c compose. Cheap to build, disproportionately loved by power users.

- **Score display:** exact integers up to 10,000, then "10.4k". Comments below the reader's personal threshold (default -5) collapse automatically with a "below your threshold" stub, honoring the Slashdot lesson that filtering should be the reader's dial.

## 7. Ranking math

Taken from Reddit's open-sourced codebase ([reddit-archive/reddit, r2/lib/db/_sorts.pyx](https://github.com/reddit-archive/reddit/blob/master/r2/r2/lib/db/_sorts.pyx)). The repo was archived read-only in 2017, so this is the historical production algorithm, not necessarily what reddit.com runs today. It is exactly what a prototype needs. **[source]**

### 7.1 Post "hot" sort

With `ups`, `downs`, and `date` = submission time as a Unix timestamp:

```python
s = ups - downs
order = log10(max(abs(s), 1))
sign = 1 if s > 0 else -1 if s < 0 else 0
seconds = date - 1134028003          # epoch offset baked into the code
hot = round(sign * order + seconds / 45000, 7)
```

Implications you can derive directly from the formula:

- The time term uses **submission time**, so a post's hot value only changes when its votes change. You can store `hot_rank` as an indexed column and update it inside the vote transaction. No recomputation sweep needed.

- 45,000 seconds is 12.5 hours, and the score term is log base 10. So to tie with a post 12.5 hours younger, an older post needs 10x the net score. This confirms the blueprint's worked example (500 points today outranks 5,000 from yesterday: the 24-hour gap is worth ~1.92 units, the 10x score gap only 1).

- Negative-score posts sink fast because the sign flips the whole score term.

### 7.2 Other post sorts

- **New:** `created_at` descending. **Top:** score descending within the timeframe. **[source: builder.py sort operators in the same repo](https://github.com/reddit-archive/reddit/blob/master/r2/r2/models/builder.py)**

- **Controversial:** `magnitude ** balance` where `magnitude = ups + downs` and `balance = min(ups,downs) / max(ups,downs)`, zero if either side is zero. High traffic plus near-even split ranks highest. **[source: _sorts.pyx]**

- **Rising** is not in that file; propose: hot restricted to posts under 6 hours old with a minimum vote velocity. Ours to tune.

### 7.3 Comment "best" sort (confidence)

Hot is wrong for comments because early comments accumulate votes first. The reddit code instead sorts comments by the **Wilson score lower bound**: treat each vote as a Bernoulli trial, estimate the true upvote proportion, and rank by the pessimistic (lower) end of the confidence interval, so a 5-0 comment does not outrank a 200-20 one on ratio alone. The code uses z = 1.281551565545, commented as 80% confidence, and cites [Evan Miller's write-up](http://www.evanmiller.org/how-not-to-sort-by-average-rating.html) as the derivation. **[source: _sorts.pyx]**

```python
def confidence(ups, downs):
    n = ups + downs
    if n == 0: return 0
    z = 1.281551565545
    p = ups / n
    left  = p + z*z/(2*n)
    right = z * sqrt(p*(1-p)/n + z*z/(4*n*n))
    under = 1 + z*z/n
    return (left - right) / under
```

Comment sort menu in the archived code: new, old, controversial, confidence, qa, hot, top, random. **[source: builder.py]** Ship Best (confidence), Top, New, Controversial; add Q&A later for AMA-style threads.

## 8. Data model (Proposal)

Postgres. Key tables, primary keys bold, essential constraints noted. Timestamps (`created_at`, `updated_at`) on everything, omitted below.

```
users            **id**, handle UNIQUE, email UNIQUE, password_hash,
                 rep_posts int, rep_comments int, status enum(active|suspended),
                 prefs jsonb (theme, comment_threshold, feed defaults)

nodes            **id**, slug UNIQUE, title, description, creator_id -> users,
                 visibility enum(public|restricted|private), nsfw bool,
                 settings jsonb (allowed_formats[], require_tag bool, min_account_age_days)

memberships      **(user_id, node_id)**, role enum(member|steward|owner),
                 state enum(active|banned|muted), ban_expires_at, ban_reason

posts            **id**, node_id, author_id, format enum(text|link|media),
                 title, body_md, url, domain, tag_id NULL -> tags,
                 nsfw bool, spoiler bool, locked bool, pinned bool,
                 ups int, downs int, score int, hot_rank double (INDEX (node_id, hot_rank DESC)),
                 comment_count int, state enum(live|removed_mod|deleted_author)

post_media       **id**, post_id, kind enum(image|video), storage_key, width, height, blurhash

comments         **id**, post_id, parent_id NULL -> comments, author_id,
                 depth int, body_md, ups, downs, score,
                 state enum(live|removed_mod|deleted_author)
                 INDEX (post_id, parent_id)

votes            **(user_id, subject_type, subject_id)**, value smallint CHECK (value IN (-1, 1))

tags             **id**, node_id, label, color        -- one tag per post; content warnings are booleans

reports          **id**, reporter_id, subject_type, subject_id, node_id,
                 reason enum(spam|harassment|rule_break|illegal|other),
                 rule_id NULL, detail text, status enum(open|actioned|dismissed), handled_by NULL

mod_actions      **id**, node_id, actor_id, action enum(remove|approve|lock|unlock|pin|unpin|
                 ban|unban|edit_settings|add_steward|remove_steward), subject_type, subject_id,
                 reason text                          -- this table IS the public audit log

automod_rules    **id**, node_id, ordinal, enabled bool, rule jsonb   -- see 10.3

notifications    **id**, user_id, kind enum(reply|mention|mod_message|report_update),
                 subject_type, subject_id, read_at NULL

saved_items      **(user_id, subject_type, subject_id)**
```

Design notes:

- **Comment tree storage:** adjacency list (`parent_id`) plus `depth`, fetched per thread with a recursive CTE (a SQL query that walks parent-child links), tree assembled and sorted server-side. This is the simplest correct approach and fine to ~2,000 comments per thread. If threads outgrow it, add a materialized `path` column (the full ancestor chain stored on each row, e.g. `0003.0021.0004`) so subtrees become a prefix range scan. Do not build that first.

- **Denormalized counters** (`ups/downs/score/comment_count` stored on the row instead of counted live) update in the same transaction as the vote or comment insert; a nightly job recounts from `votes` and fixes drift. Deleting a vote row and decrementing must be atomic.

- **Removal semantics:** `removed_mod` hides body and author from non-stewards but keeps the row so the thread shape and audit log stay coherent; `deleted_author` blanks the body, keeps children.

- **Reputation:** `rep_posts` and `rep_comments` increment/decrement with votes on your content (author of the content, not the voter). Used only as gates (Node creation, probation exit), never displayed as a leaderboard. Slashdot caps karma to stop score-gaming **[source: karma FAQ]**; adopt a soft cap (diminishing returns past 10k) if gaming appears.

## 9. API surface (Proposal)

JSON over HTTPS, session cookie auth, cursor pagination everywhere a list is returned.

```
POST /api/auth/register | login | logout
GET  /api/feed?scope=home|all|popular|node:{slug}&sort=hot|new|rising|top&t=&cursor=
GET  /api/nodes/:slug                      POST /api/nodes
POST /api/nodes/:slug/join | leave
GET  /api/nodes/:slug/rules | tags | stewards | modlog?cursor=
POST /api/posts        {node, format, title, body|url|media, tag, nsfw, spoiler}
GET  /api/posts/:id    PATCH /api/posts/:id    DELETE /api/posts/:id   (soft)
GET  /api/posts/:id/comments?sort=best|top|new|controversial&cursor=&focus=&context=
POST /api/comments     {post_id, parent_id?, body}
PATCH|DELETE /api/comments/:id
PUT  /api/vote         {subject_type, subject_id, value: -1|0|1}    (0 = retract)
PUT  /api/save         {subject_type, subject_id, saved: bool}
POST /api/reports      {subject_type, subject_id, reason, rule_id?, detail?}
GET  /api/mod/:slug/queue?cursor=          (steward)
POST /api/mod/actions  {action, subject_type, subject_id, reason, duration?}   (steward)
GET  /api/users/:handle/overview|posts|comments?cursor=
GET  /api/notifications?cursor=            POST /api/notifications/read
GET  /api/search?q=&scope=&type=posts|comments|nodes&cursor=
```

Media upload: `POST /api/media/presign` returns a presigned URL for direct upload to object storage; the post references the returned `storage_key`. Notifications are polled (30s) in the prototype; no websockets.

## 10. Governance and moderation (Proposal)

### 10.1 Roles and permission matrix

| Action | Guest | Member (probation) | Member | Steward | Owner | Site admin |
|---|---|---|---|---|---|---|
| Read public Nodes | yes | yes | yes | yes | yes | yes |
| Vote | no | yes | yes | yes | yes | yes |
| Post / comment | no | rate-limited | yes | yes | yes | yes |
| Report | no | yes (low weight) | yes | yes | yes | yes |
| Create a Node | no | no | age+rep gate | yes | yes | yes |
| Remove/approve/lock/pin in Node | no | no | no | yes | yes | yes |
| Ban from Node | no | no | no | yes | yes | yes |
| Edit Node settings / manage stewards | no | no | no | no | yes | yes |
| Suspend accounts sitewide / close Nodes | no | no | no | no | no | yes |

Probation = first 14 days or until small reputation threshold, whichever comes second. Direct descendant of Slashdot excluding the newest accounts from moderation **[source: moderation.shtml]** and Lobsters restricting new users for 70 days **[source: repo issue quoting the about page]**, scaled to a faster-moving site.

### 10.2 Report lifecycle

1. Reporter picks a typed reason; choosing "breaks a Node rule" shows that Node's numbered rules to pick from (Lobsters-style typed flags, applied to reports rather than votes).

2. Report lands in the Node's queue; sitewide-severity reasons (illegal content) also copy to the site-admin queue.

3. Steward actions it; every action writes a `mod_actions` row.

4. Reporter gets a notification: "actioned" or "reviewed, no action". Closes the loop that most platforms leave open.

### 10.3 Automod rule engine (minimal)

Per-Node ordered JSON rules evaluated at submit time, first match wins:

```json
{ "if":   { "field": "title|body|domain|author_age_days|author_rep",
            "op": "contains|regex|lt|gt|in", "value": "..." },
  "then": { "action": "reject|hold|flag", "message": "shown to the author" } }
```

`reject` blocks with the message; `hold` publishes to the queue only; `flag` publishes but marks for review. Regex evaluation must be time-limited (regex can be crafted to run forever; cap execution at a few ms and fail open to `hold`).

### 10.4 Transparency defaults

- `mod_actions` is public per Node from day one (`/n/:slug/mod/log`). No shadow bans: a banned user is told, and Node bans appear in the log. This is Lobsters' most-praised property **[secondary: Tildes discussion, syften.com]** and it is nearly free to build since the audit table must exist anyway.

- Deferred, borrowed from Slashdot for later: metamoderation-style review of steward actions **[source: metamod FAQ]**, and reputation-weighted comment starting scores.

## 11. Anti-abuse baseline (Proposal)

- One vote per user per item, enforced by the `votes` composite primary key, not application logic.

- Rate limits: 1 comment / 60s and 1 post / 5 min for members (halved intervals after reputation threshold; doubled during probation). Precedent: Slashdot's 120-second comment gap. **[source: karma FAQ]**

- Cooling-off: an account whose content is heavily downrated in a short window gets a temporary posting slowdown, mirroring Slashdot's temporary ban on rapid down-moderation targets. **[source: karma FAQ]**

- New-account friction: probation (10.1), email verification before posting, Node creation gated on age + reputation.

- Out of scope for prototype (say so, do not half-build): vote-ring detection, IP reputation, ML spam filtering, vote fuzzing.

## 12. Build plan (Proposal)

**Stack:** Next.js (App Router) + Postgres + Drizzle or Prisma + Auth.js; S3-compatible object storage for media; Postgres full-text search (`tsvector`) for v1 search; Redis only if rate limiting outgrows a Postgres table. Alternative worth an afternoon before writing code: read the Lobsters Rails codebase ([github.com/lobsters/lobsters](https://github.com/lobsters/lobsters)) as a working reference for votes, flags, and the public mod log; check its LICENSE before lifting anything.

**M0, walking skeleton (1-2 weeks):** auth, create Node, text posts, flat comments, New sort, seed script generating ~50 users / 5 Nodes / 500 posts / 5k comments with a power-law vote distribution. Acceptance: two browsers, two accounts, one conversation.

**M1, the actual product (2-3 weeks):** votes with optimistic UI, hot/top/controversial feeds off the stored `hot_rank`, nested comments with collapse gutter + continuation stubs + Best sort, join/leave with subscribed Home feed, link posts with server-side title fetch. Acceptance: a 1,000-comment seeded thread loads under 1s and collapses smoothly on a phone.

**M2, governance (2 weeks):** reports with typed reasons, mod queue, remove/lock/pin/ban, public audit log, tags, automod rules, probation + rate limits. Acceptance: a steward can run a Node for a week without touching the database.

**M3, retention:** media uploads, notifications, search, profiles, saved items, reader score-threshold setting.

## 13. Open decisions

- Display-score caps and hidden comment scores (HN hides them from non-authors **[secondary]**): anti-pile-on wins vs. transparency costs. Decide after watching real threads.

- Whether Rising uses velocity (votes/hour) or a shortened hot window. Needs live traffic to tune.

- Node creation policy at launch: open-with-gates (specced above) vs. request-based while small. Request-based is easier to keep clean early.

- Custom multi-Node feeds (the blueprint's "Custom Feeds"): pure read-side feature, cleanly deferred to post-M3.
