# Functional Specification: Topic-Driven Aggregation Platform, With Slashdot and Peer Baseline

author: Research agent (claude.ai, commissioned; ~271 sources, 2026-07)

date: 2026-07

status: filed content-faithful (cleaned-paste; PLAYBOOK §4) — competitive/industry research

`Commissioned clean-room UX/architecture research on Reddit (ranking math from the open-sourced code,
data model, moderation, karma) with a comparative baseline against Slashdot, Hacker News, Lobsters,
Lemmy, and Tildes. Grounds the **Graze** aggregation behavior-scale mock. The build-ready condensation of
this is the "build spec" at thinking/app/build-specs/graze-topic-aggregation-build-spec.md; the twin
spec is graze-persona-switch-spec.md; methodology is thinking/behavior-scale/. Note the corpus's
active forum vision (plans/2026-07-27-read-first-forum-mvp.md) chose a read-first lens over the public
Bluesky AppView rather than the Next.js+Postgres backend this research assumes — see COHESION for the
tracked tension. Raw session: seeds/transcripts/raw/stellin-graze-behavior-scale-sessions-2026-07.md.
Ranking constants and third-party limit numbers are labeled [source]/[secondary] in-text.`

---

## TL;DR

- Build the MVP as a single-database web app (Next.js/React + Tailwind + PostgreSQL) around one hard slice: one or a few topic Nodes, text and link posts, endorse/reject voting, nested comments stored as an adjacency list plus a materialized path column, the Reddit "hot" sort (log10 of score plus a time term divided by 45000, verbatim from Reddit's open-sourced `_sorts.pyx`), and the Wilson lower-bound "best" comment sort.

- The Reddit-derived model your blueprint already follows is the right base: user-created communities, fully user-published posts, transparent open-source ranking, and thing-typed IDs (t1_ comment, t3_ link, t5_ community) as a proven API pattern. Adopt selected Slashdot mechanisms as enhancements: labeled moderation reasons, comment score caps and floors, and metamoderation as a phase-3 trust experiment.

- Slashdot differs from Reddit at the root: paid editors hand-pick front-page stories from a submission queue, moderation is a temporary randomly-granted privilege (5 points, labeled reasons, capped +5/-1), karma is a hidden coarse label not a public number, and readers filter by score threshold. These are governance choices, not layout choices, and most conflict with a decentralized user-created-community model.

---

## Recommended MVP Slice (read this first)

Ship the smallest thing that still feels like the product:

- One or a small fixed set of Nodes (communities), no user-created Nodes yet.

- Two payload types: text (self) posts and link posts. No media pipeline yet.

- Endorse/reject voting on posts and comments, idempotent, with toggle and reversal.

- Nested comment trees with collapse, "load more comments" stubs, and a depth cap.

- Two sorts on the feed (Hot, New) and two on comments (Best, New).

- Accounts, sessions, basic profile, and a single moderator role per Node with remove/lock/pin.

Defer everything else (media, flairs, custom feeds, automod, messaging, metamoderation) to later phases described at the end of Part 1.

---

# PART 1: EXTENDED PROTOTYPE SPECIFICATION

## 1. Core concepts and nomenclature (defined before use)

Per your IP-abstraction guidance, this spec uses neutral names. Mapping to the Reddit model:

- **Node** = community/subreddit. A topic container with its own rules, moderators, and feed.

- **Reputation** = karma. A per-user trust signal, split into post reputation and comment reputation.

- **Endorse / Reject** = upvote/downvote. Net score = endorsements minus rejections.

- **Thing IDs**: every entity has a typed fullname. This mirrors Reddit's proven scheme where the prefix identifies the type. Reddit uses t1_ for a comment, t2_ for an account, t3_ for a link/post, t4_ for a message, t5_ for a subreddit, and t6_ for an award, with a base-36 id (for example t3_8dmv8z). The clone should adopt the same idea: c1_ comment, u2_ user, p3_ post, m4_ message, n5_ node. This makes listings, votes, and reports polymorphic over a single id space.

Definitions used later, stated once here:

- **Materialized path**: a string column on each comment storing the ordered chain of ancestor ids (for example `0001.0007.0002`). A prefix match (`LIKE '0001.0007.%'`) returns an entire subtree, and an alphabetical sort by that column yields depth-first thread order.

- **Adjacency list**: each comment stores only its immediate `parent_id`. Simple to write, needs recursion to read a subtree.

- **Closure table**: a separate table with one row per ancestor/descendant pair plus depth. Fast subtree and ancestor queries at the cost of O(n^2)-ish storage and expensive re-parenting.

- **Wilson score interval**: a statistical confidence interval for a proportion (here, the fraction of votes that are endorsements). Sorting by its lower bound balances the observed ratio against sample size, so a comment with 5/0 does not automatically beat 90/10.

- **Time decay**: a ranking term that reduces an item's effective score as it ages, so fresh content can outrank older content at equal votes.

- **Metamoderation**: a second review layer where users judge whether prior moderation actions were fair, feeding back into who is allowed to moderate.

- **ActivityPub federation**: a W3C protocol letting independent servers exchange posts, comments, and votes so that a user on one server can subscribe to a community on another. This is the architectural fork Lemmy took.

## 2. Full screen and page inventory

Each screen below lists layout, key components, states, and primary actions. The three-column desktop shell from your blueprint (sticky header, left Directory sidebar, center feed, contextual right sidebar) wraps most of these.

### 2.1 Home / landing feed

- **Layout**: three-column shell. Center shows an aggregated feed of the user's joined Nodes (or a default set when logged out).

- **Components**: sort tab bar (Hot, Best, Rising, New, Top with a timeframe dropdown for Top), Post Units, right sidebar with global trending topics.

- **States**: loading (skeleton Post Units), empty ("join some Nodes to fill your feed" with suggestions), error (retry banner). Logged-out shows Popular instead of a personalized feed.

- **Primary actions**: vote, save, open post, open Node, create post, switch sort.

### 2.2 Node page

- **Layout**: three-column shell; center feed scoped to one Node.

- **Components**: Node banner and icon, right-sidebar About card (description, member count, active-user count, Join/Leave button), numbered rules list, moderator roster, feed sort bar, per-Node create-post button, flair filter bar.

- **States**: loading, empty (no posts yet, prompt to be first), error, restricted/private (join-to-view gate), banned-user notice.

- **Primary actions**: join/leave, create post, filter by flair, sort, open moderation entry point if the viewer is a mod.

### 2.3 Post detail / comment page

- **Layout**: post payload at top, sort control for comments, comment composer, then the comment tree.

- **Components**: full Post Unit (expanded), comment sort tabs (Best, Top, New, Controversial, Old, Q&A), threaded comments with per-node collapse toggles, "load more comments" and "continue this thread" stubs, sticky/pinned comment slot, locked-thread banner when applicable.

- **States**: loading (post first, then comments), empty (no comments yet), error, removed post (tombstone), locked (composer disabled with explanation).

- **Primary actions**: vote, comment, reply inline, share, save, report, crosspost, collapse subtree, load more.

### 2.4 Comment permalink view

- **Layout**: single comment promoted to the top as the thread root, with a "view full discussion" link and the ancestor breadcrumb.

- **Components**: the focused comment, its descendants, a context control (show N parents).

- **States**: loading, error, removed/deleted tombstone, "context unavailable" when parents are gone.

- **Primary actions**: expand context, jump to full thread, vote, reply.

### 2.5 User profile

- **Layout**: header with username, cake/join date, post reputation and comment reputation, then tabbed content.

- **Components**: tabs for Posts, Comments, Saved (owner-only), Upvoted/Downvoted (owner-only, optional), About card, trophy/flair area.

- **States**: loading, empty per tab, error, suspended/deleted account ("this account has been deleted").

- **Primary actions**: follow (optional), message, view content, filter own content, manage saved.

### 2.6 Settings

- **Layout**: left settings nav, right form panels.

- **Components**: account (email, password), profile (display name, avatar, bio), feed (default sort, show NSFW, autoplay), notifications (per-type toggles and delivery), privacy/blocking, safe-browsing/threshold controls.

- **States**: loading, saved-confirmation, validation error, save failure.

- **Primary actions**: edit and save each panel, manage blocks, deactivate account.

### 2.7 Node creation flow

- **Layout**: linear wizard.

- **Components**: step 1 name and URL slug with live availability check, step 2 type (public/restricted/private) and topic tags, step 3 description and rules, step 4 appearance (icon, banner, accent color), step 5 confirm.

- **States**: validating slug, slug taken, gate-not-met (creator lacks required reputation/age), success redirect to new Node.

- **Primary actions**: check availability, advance/back, publish Node. Creator becomes first moderator.

### 2.8 Moderation dashboard

- **Layout**: mod-only shell with sub-tabs.

- **Components**: Mod Queue (unreviewed/reported items), Reports view (grouped by reason), Spam view, Modmail, Mod Log (append-only action history), Banned Users, Rules editor, Automod-style rule editor, Flair manager, Node settings.

- **States**: loading, empty queue ("nothing needs review"), error, permission-scoped (buttons hidden if the mod's tier lacks the permission).

- **Primary actions**: approve, remove (with reason), mark spam, lock, sticky/pin, ban (temp/perm) with note, assign flair, distinguish, edit rules, edit automod config.

### 2.9 Search results

- **Layout**: query bar with scope toggle (this Node / all), filters, result list.

- **Components**: tabs for Posts, Comments, Nodes, Users; filters for sort (relevance, top, new), time range, and Node; result cards.

- **States**: loading, no results, error, query-too-short hint.

- **Primary actions**: refine query, switch scope, open result, subscribe to a Node from results.

### 2.10 Notifications inbox

- **Layout**: list of notification rows, filter chips.

- **Components**: rows for replies, mentions, mod actions, Node announcements; read/unread state; mark-all-read.

- **States**: loading, empty, error.

- **Primary actions**: open source item, mark read/unread, adjust notification settings.

### 2.11 Direct messages

- **Layout**: two-pane (conversation list, thread) on desktop, stacked on mobile.

- **Components**: conversation list, message thread, composer, block/report controls, modmail distinction (messages to a Node go to its mod team).

- **States**: loading, empty, error, blocked, message-request pending.

- **Primary actions**: send, reply, block, report, mark read.

### 2.12 Custom feed / multi-node builder

- **Layout**: builder panel plus live preview.

- **Components**: feed name and description, Node picker with search, visibility (public/private), ordering, resulting combined feed.

- **States**: loading, empty (no Nodes added), error, name conflict.

- **Primary actions**: add/remove Nodes, rename, set visibility, save, share URL. This mirrors Reddit's multireddit creation, which takes a name, description, visibility, and a set of subreddits.

### 2.13 Wiki pages

- **Layout**: per-Node wiki with a page index and article view.

- **Components**: markdown article, edit history/revisions, page permissions (who can view/edit), table of contents.

- **States**: loading, page-not-created, error, permission-denied, edit-conflict.

- **Primary actions**: read, edit (if permitted), view revisions, revert, set page permissions.

## 3. Component-level interaction specs

### 3.1 Voting mechanics

- **Toggle and reversal**: endorsing an item you have not voted on sets your vote to +1. Endorsing again removes the vote (back to 0). Rejecting flips a +1 to -1 in one action. State is stored per (user, thing) so votes are idempotent: re-sending the same vote is a no-op server-side.

- **Score display**: show net score (endorsements minus rejections). Optionally show an approval ratio. Do not expose separate raw up/down counts by default.

- **Score fuzzing / vote obfuscation**: this is a documented Reddit anti-manipulation technique. Reddit historically fuzzed the displayed up and down counts while keeping the net score accurate, so bots could not measure whether a vote registered. A 2015 arXiv study notes Reddit "certified that a post's score (i.e., up-votes minus down-votes) was always accurate" during fuzzing, and that in July 2014 Reddit removed public up/down totals in favor of score plus an approval ratio; Reddit admin KeyserSosa confirmed in that overhaul that scores "aren't just 'upvotes minus downvotes'" and involve "some slight fuzzing" to stymie reverse-engineers, with the front page beginning to show "a 'k' instead of '000' at the end of posts with scores over 10,000." For the clone: keep the internal true tallies for ranking, and only publish net score plus ratio; optionally add small display jitter for fresh items. Mark this as an anti-abuse display layer, not a change to the stored data.

### 3.2 Comment composer

- Markdown input with a formatting toolbar and live preview.

- Inline reply expands a composer directly under the target comment without a page reload.

- Draft autosave to local storage.

- Client and server both sanitize and render markdown; server stores raw markdown (mirroring how Reddit's edit endpoint replaces content with raw markdown text).

### 3.3 Edit, delete, and tombstoning

- **Edit**: author-only; show an "edited" timestamp. Store raw markdown.

- **Delete vs remove (tombstone distinction)**: a user-deleted item shows the author as `[deleted]`; a moderator/admin-removed item shows `[removed]`. This mirrors Reddit and Hacker News conventions, where `[deleted]` means the author removed it and a killed/removed item is a separate state. Keep the row for tree integrity; blank the body and author on the appropriate tombstone.

### 3.4 Save, share, report, crosspost

- **Save**: private per-user bookmark; appears on the profile Saved tab.

- **Share**: copy permalink; optional external targets.

- **Report**: opens a reason picker (see 8.2), routes to the Node mod queue, optionally to site admins for policy breaches.

- **Crosspost**: create a new post in another Node that references the original, preserving attribution.

### 3.5 Feed loading: infinite scroll vs pagination

- Use cursor pagination keyed on the fullname of the last item (Reddit's listing model uses `after`/`before` fullname cursors with a default limit of 25 and a maximum of 100). Reddit's listings also cap at roughly 1,000 items total; adopt a similar hard cap to bound deep queries. Offer infinite scroll in the UI backed by cursor pages, with an accessible "load more" fallback.

### 3.6 Comment tree loading

- Render the most relevant comments first and replace the remainder with stubs. Reddit renders selected comments and stubs the rest as either "load more comments" or "continue this thread". The `morechildren` endpoint takes the submission's link_id plus a comma-delimited list of child comment ids and returns objects in pre-order DFS order; each comment carries a `depth` field. Adopt the same: a stub stores the parent and the list of hidden child ids, and expanding it fetches that batch.

## 4. Comment tree loading strategy and sorts

- **Depth limit and "continue this thread"**: cap inline nesting at a fixed depth (commonly ~10 levels; Reddit clients and API tools default comment-tree depth around 10, with a default fetch depth of 3 in several tools). Beyond the cap, show "continue this thread" linking to that comment's permalink where depth resets to 0.

- **Truncation / "load more comments"**: limit the number of siblings rendered per level; overflow becomes a stub listing hidden child ids, fetched on demand. This bounds initial payload for threads with thousands of comments.

- **Collapse thresholds**: auto-collapse comments below a score threshold (analogous to Slashdot's reader threshold and Hacker News desaturation of downvoted comments). Collapsing a parent collapses its entire lineage, per your blueprint.

- **Comment sort options** (match Reddit's set): Best (Wilson lower bound), Top (net score), New (recency), Controversial (high engagement with balanced up/down), Old, and Q&A (question/answer pairing). Formulas in section 5.

## 5. Ranking algorithms (from primary sources)

All formulas below are transcribed from Reddit's open-sourced `_sorts.pyx` in the `reddit-archive/reddit` repository. The file header shows copyright "reddit Inc." and it defines the functions used to rank posts and comments. Treat these as what the 2010-2015-era open-source code guarantees, not necessarily what live Reddit runs today (see the disagreement note at the end of this section).

### 5.1 Hot (posts)

Verbatim logic from `_hot(ups, downs, date)`:

```
s = ups - downs
order = log10(max(abs(s), 1))
sign = 1 if s > 0, -1 if s < 0, 0 if s == 0
seconds = date_epoch_seconds - 1134028003
score = round(sign * order + seconds / 45000, 7)
```

Interpretation:

- `1134028003` is the fixed reference epoch (a moment in December 2005). `seconds` is the submission time measured from that reference.

- The `45000`-second divisor (12.5 hours) is the time weight: roughly every 12.5 hours of newer submission time is worth one order-of-magnitude (10x) more votes.

- `log10` means the first 10 votes matter as much as the next 100, then the next 1,000. Early votes dominate.

- The `sign` term makes downvoted items sort below neutral ones. A known quirk, documented in a Reddit pull request, is that this makes older negative-scoring posts rank above newer negative ones; the clone should fix the sign handling if negative-score ordering matters.

### 5.2 Best / confidence (comments)

Verbatim from `_confidence(ups, downs)`, the Wilson score lower bound:

```
n = ups + downs
if n == 0: return 0
z = 1.281551565545     # this is the 80% confidence z-score
p = ups / n
left = p + z*z/(2*n)
right = z * sqrt(p*(1-p)/n + z*z/(4*n*n))
under = 1 + z*z/n
score = (left - right) / under
```

Provenance: the code comments cite Evan Miller's "How Not To Sort By Average Rating," which proposes ranking by the lower bound of the Wilson score confidence interval, and which Miller says "has since caught on at places like Reddit, Yelp, and Digg." Reddit's guest blog post by Randall Munroe of xkcd announced the "best" comment sort and referenced Miller's write-up. Note that Reddit's production code uses z = 1.28 (80% confidence), even though Miller's illustrative default is z = 1.96 (95%); this is a real, sourced difference worth noting. Reddit also precomputes a lookup table of confidence values for ups in [0,400) and downs in [0,100) for performance.

### 5.3 Controversial

Verbatim from `controversy(ups, downs)`:

```
if downs <= 0 or ups <= 0: return 0
magnitude = ups + downs
balance = downs/ups if ups > downs else ups/downs
score = magnitude ** balance
```

Interpretation: high total volume combined with a near-even up/down split scores highest. Items that are purely endorsed or purely rejected score 0.

### 5.4 Q&A

Verbatim intent from `qa(...)`: score the question by its own confidence, then add the confidence of the best OP-directed answer, plus a dampened length bonus (`log10(question_length + answer_length) / 5`), so strong question/answer pairs surface. The comment notes it gives "more weight to longer posts, but count[s] longer text less and less to avoid artificially high rankings for long-spam posts."

### 5.5 Rising, Top, New

- **Top** sorts by net score within a chosen timeframe (hour/day/week/month/year/all).

- **New** sorts by creation time descending.

- **Rising** is not a static formula in `_sorts.pyx`; it surfaces items gaining votes quickly relative to their age using recent vote velocity. Reddit's exact rising logic is not fully in the open-source ranking file, so the clone should implement rising as "score gained per unit time since submission, over a short recent window" and label it as an approximation rather than a Reddit-exact reproduction.

### 5.6 What the archived code guarantees vs live Reddit today

- The archived `_sorts.pyx` is authoritative for the 2010-2015-era algorithms and is the correct basis for a prototype.

- Live Reddit today additionally personalizes home feeds, applies vote fuzzing and a contributor-quality vote weighting, and does not publish its current ranking source. Secondary writeups (for example Amir Salihefendic's "How Reddit ranking algorithms work" and various 2026 marketing explainers) corroborate the open-source formulas but are not primary for current behavior. Where current Reddit and the archived code disagree, this spec follows the archived code and flags the gap rather than asserting current internals.

## 6. Data model

Entities with key fields and notes. Assume every content entity carries a typed fullname (section 1) plus `created_at`, `updated_at`.

- **User**: id, username (unique), email, password_hash, display_name, avatar_url, bio, post_reputation, comment_reputation, created_at, is_suspended, settings JSON.

- **Node** (community): id, slug (unique), title, description, type (public/restricted/private), rules JSON (ordered), icon/banner, accent_color, member_count (denormalized), nsfw flag, created_by, created_at.

- **Membership**: user_id, node_id, role (member/approved), joined_at. Unique on (user_id, node_id). Drives the joined-Nodes list and personalized feed.

- **Post**: id, node_id, author_id (nullable for deleted), type (text/link/media), title, body_markdown, url, thumbnail_url, flair_id, nsfw, spoiler, is_locked, is_stickied, is_removed, removed_reason, score, endorsements, rejections, hot_rank (denormalized), comment_count, created_at.

- **Comment**: id, post_id, author_id (nullable), parent_id (adjacency list), path (materialized path), depth, body_markdown, score, endorsements, rejections, confidence_rank, is_removed, is_deleted, created_at. See tree discussion below.

- **Vote**: user_id, thing_id (polymorphic fullname), value (+1/-1), created_at. Unique on (user_id, thing_id) for idempotency. Never expose per-user votes publicly.

- **Flair**: id, node_id, kind (post/user), text, css_class/color, is_mod_only, is_required.

- **Report**: id, thing_id, reporter_id, reason_code, free_text, status (open/actioned/dismissed), created_at, handled_by.

- **ModAction / ModLog**: id, node_id, mod_id, action (remove/approve/ban/lock/sticky/flair/spam), target_thing_id, reason, created_at. Append-only; optionally publicly viewable.

- **Notification**: id, user_id, type (reply/mention/mod/announcement), source_thing_id, is_read, created_at.

- **Message**: id, sender_id, recipient_id (user or node for modmail), body_markdown, is_read, created_at, thread_id.

- **SavedItem**: user_id, thing_id, created_at. Unique on (user_id, thing_id).

- **Ban / Mute**: id, node_id, user_id, kind (ban/mute), is_permanent, expires_at, reason, mod_id, created_at.

### 6.1 Comment tree structure choice

Three options, defined in section 1:

- **Adjacency list** (`parent_id` only): trivial writes, but reading a deep subtree needs a recursive query. In PostgreSQL a recursive CTE (`WITH RECURSIVE`) handles this well for moderate depth.

- **Materialized path** (`path` string): one column, prefix queries return whole subtrees, constant-width zero-padded segments sort into depth-first thread order directly. Cheap inserts; re-parenting is rare for comments so its main downside barely applies. Practical string-length limits bound maximum depth, which aligns with the depth cap anyway.

- **Closure table**: fastest arbitrary ancestor/descendant queries, but O(n^2)-ish storage growth and heavier writes.

**Recommendation**: store both an adjacency `parent_id` (source of truth, integrity, easy re-parent on rare moves) and a materialized `path` (fast subtree reads and natural thread ordering). This hybrid is exactly what several production tree libraries do (keep a `parent` column even when a closure/ancestor structure exists, to avoid a join on a huge table). Use PostgreSQL recursive CTEs for ad hoc traversals and the `path` for the common "load this subtree in order" case.

### 6.2 Indexing for feed queries

- Post feed: composite index on (node_id, hot_rank desc) and (node_id, created_at desc); a global (hot_rank desc) for aggregate feeds.

- Comments: index on (post_id, path) for ordered subtree reads and (post_id, parent_id) for adjacency.

- Votes: unique (user_id, thing_id); plus (thing_id) for tally recomputation.

- Recompute hot_rank on each vote (cheap) rather than at read time, mirroring how "hot" implementations update rank on vote.

## 7. API surface sketch

REST-ish, cursor-paginated, thing-typed. Modeled on Reddit's listing conventions (`after`/`before` fullname cursors, `limit` default 25 / max 100).

- `GET /r/{node}/{sort}` and `GET /feed/{sort}` with `?after=&limit=&t=` (t = timeframe for top). Returns a listing of post things.

- `GET /post/{id}` returns the post; `GET /post/{id}/comments?sort=best&depth=&limit=` returns the comment tree with stubs.

- `POST /api/morechildren` with `link_id` and comma-delimited `children` ids returns the next batch in pre-order DFS with `depth` fields (Reddit-proven shape).

- `POST /post`, `PATCH /post/{id}`, `DELETE /post/{id}` for CRUD.

- `POST /comment` (parent = post or comment fullname), `PATCH`, `DELETE`.

- `POST /api/vote` with `{ thing_id, dir: 1|0|-1 }`, idempotent on (user, thing).

- `POST /api/subscribe` `{ node_id, action: join|leave }`.

- `POST /api/report` `{ thing_id, reason_code, detail }`.

- Mod: `POST /api/mod/remove`, `/approve`, `/spam`, `/lock`, `/sticky`, `/ban`, `/flair`, each writing a ModLog row.

- `GET /search?q=&scope=&type=&sort=&t=`.

Auth via session/OAuth2 bearer tokens with a per-token rate limit. For a reference point, Reddit's free API tier is documented at roughly 60 requests per minute for a sustainable authenticated (PRAW) rate, tracked over a rolling window, with Reddit's newer approved-client documentation citing 100 queries per minute per OAuth client. A 60-100 QPM per-client budget is a sensible starting envelope for the clone.

## 8. Reputation, moderation, and governance

### 8.1 Reputation (karma) mechanics

Grounded in Reddit's official "What is karma?" Help Center page and Reddit's archived `account.py`:

- **Post reputation vs comment reputation**: two separate tallies. Reddit's archived `account.py` stores `link_karma` and `comment_karma`; post karma bundles both link submissions and self/text posts ("link karma includes both 'link' and 'self' values"), while comment karma is tracked separately. The clone mirrors this split.

- **Not 1:1 with votes**: Reddit's Help Center states plainly that karma is "just an approximate reflection" and that "upvotes and karma don't have a 1:1 relationship." Reddit deliberately does not publish the exact dampening formula. So the clone should apply diminishing returns (for example a sub-linear function of net votes per item) and explicitly document that reputation is an approximate trust signal, not a vote counter. Do not claim a specific Reddit formula, because none is officially disclosed.

- **Reputation gates**: Reddit's Help Center confirms "some communities require a certain amount of karma before allowing you to post there" as an anti-spam measure, and Reddit's archived code shows age-plus-karma thresholds for privileged actions (for example creating a community requires minimum account age and minimum link/comment karma). Critically, there is no official sitewide karma number; thresholds are set per community, commonly enforced via AutoModerator. The clone should implement per-Node configurable minimum account age and minimum reputation to post/comment, with no global magic number.

- **Purpose and non-transfer**: reputation is a trust/anti-spam signal, not a currency. Reddit's User Agreement bars transferring the account itself ("You will not license, sell, or transfer your Account without our prior written approval"); there is no official statement that karma is a separately transferable token, so treat reputation as account-bound and non-exchangeable by design.

### 8.2 Moderation workflows

- **Mod queue**: unreviewed and reported items land here. Actions: approve, remove (with a reason), mark spam (removal that also trains spam heuristics), lock (no new comments), sticky/pin, assign flair, distinguish.

- **Report reasons**: a fixed list (spam, harassment, misinformation, off-topic, rule-specific) plus free text; Node-specific reasons are configurable. Lobsters offers a good reason taxonomy to borrow: off-topic, already posted, broken link, spam, me-too, troll, unkind.

- **Removal vs spam**: removal takes the item down for a stated reason; spam additionally signals the anti-abuse system and is used for obvious crapfloods.

- **Bans**: temporary (with `expires_at`) or permanent, with a mod note and optional user-facing reason. Mute restricts modmail.

- **Mod permission tiers**: granular flags (manage posts, manage users, manage settings, manage flair, manage wiki, full). Hide actions the tier lacks.

- **Public mod log**: an append-only ModLog, optionally public per Node. Lobsters demonstrates the trust value: its public moderation log "lists the edits, deletions, and bans by moderators" and the site explicitly prohibits shadow banning. Offer this as a per-Node toggle.

### 8.3 Automod-style rule engine

A YAML-ish condition/action ruleset evaluated on new posts/comments, modeled on Reddit's AutoModerator and community bots that use the same syntax. Each rule has a trigger (conditions) and actions. Supported conditions for the prototype:

- keyword/phrase and regex match on title or body,

- account age below threshold,

- reputation below threshold,

- link domain allow/deny lists,

- repeat-post detection.

Actions: remove, filter to mod queue, comment/notify, set flair, report. Example rule shape (illustrative, following the documented AutoModerator/bot pattern of info/trigger/actions blocks):

```
- info:
    name: low-age-filter
  trigger:
    author_account_age_days: "< 3"
    type: [post, comment]
  actions:
    - filter        # send to mod queue
    - notify: "Your account is too new to post here yet."
```

Account age is deliberately included because, as security writeups note, account age "is harder to game than karma" since it advances in real time.

## 9. Anti-abuse basics

- **Rate limits**: per-account and per-IP posting/commenting throttles. Slashdot's documented rule is a useful baseline: per its FAQ, "the same person can't post more than once every 120 seconds. Also, if a single user is moderated down several times in a short span, a temporary ban will be imposed on that user ... a cooling off period." (The FAQ states the cooling-off ban is temporary but does not fix its exact length; do not assume a specific number of hours.)

- **New-account restrictions**: gate voting weight, posting frequency, and link submission for young accounts (Lobsters restricts new accounts for their first 70 days from sending invites, flagging, and editing titles/tags; Hacker News gates comment downvoting behind a karma threshold, documented as 501: per the community "hacker-news-undocumented" reference, "After users reach 501 Karma, they gain the ability to downvote another comment... the minimum score is -4 points.").

- **Shadow-ban concept**: a state where the offender's content is visible only to themselves. Powerful against spammers but corrosive to trust; if used at all, disclose the policy. Lobsters and Tildes explicitly reject shadow banning in favor of public logs. Recommend against silent shadow bans for the clone; prefer transparent removals.

- **Vote-manipulation detection (high level)**: keep true (unfuzzed) tallies internally; publish only net score plus ratio. Flag coordinated patterns (many young accounts converging on one item, shared exit IPs/ASNs, sudden ratio collapse with a volume spike). Drop suspicious votes from ranking rather than from the displayed score. Reddit's policy explicitly names multi-account self-voting, voting services/scripts, and off-platform vote coordination (brigading) as prohibited.

## 10. Onboarding and notifications

- **Signup**: email plus password or OAuth; username selection; optional email verification.

- **Interest selection**: pick topics to seed Node suggestions; auto-suggest Nodes from those topics. Avoid Reddit's early misstep of auto-subscribing everyone to default announcement Nodes (its 2009 auto-subscribe change drew user complaints).

- **Join suggestions**: ranked by topic match and Node activity.

- **Notification types**: post replies, comment replies, mentions, mod actions (removal/ban with reason), Node announcements. Per-type toggles.

- **Delivery**: in-app inbox always; optional email; push later. Batch low-priority notifications.

## 11. Mobile responsive behavior

- **Column collapse**: the three columns stack. The left Directory becomes a slide-in drawer from the hamburger; the right sidebar (About/rules/trending) moves below the feed on Node/post pages or into an expandable panel.

- **Bottom navigation**: a fixed bottom bar with Home, Search, Create, Inbox, Profile (a common mobile pattern, and the shape Lemmy's mobile-friendly clients use).

- **Post Unit** compacts: voting axis becomes a horizontal control under the payload; thumbnail shrinks; action bar collapses secondary actions into an overflow menu.

- **Comment trees**: reduce inline indent per level, add a tap-to-collapse hit area on the whole comment header, and rely more aggressively on "continue this thread" to control width.

## 12. Tech stack and phased build

### 12.1 Recommended stack

- **Frontend**: Next.js/React with Tailwind (your blueprint already allows Material UI or Tailwind; Tailwind pairs cleanly with a custom component set and avoids trade-dress lookalike risk). Generic chevrons for voting, neutral palette, no mascot.

- **Backend**: Next.js API routes or a separate Node service; PostgreSQL as the primary store. Use recursive CTEs plus a materialized path column for comment trees (section 6.1). Redis for rate limits, vote counters, and feed caching.

- **Auth**: session cookies or OAuth2 bearer tokens.

- This stack is explicitly a proven shape: Lemmy runs Rust/Actix + Diesel + PostgreSQL with a TypeScript/React frontend, showing PostgreSQL comfortably backs a federated Reddit-like at scale.

### 12.2 Phases

- **Phase 1 (MVP)**: the slice at the top. Single or fixed Nodes, text+link posts, endorse/reject, nested comments with adjacency+path, Hot and New feed sorts, Best and New comment sorts, one mod role with remove/lock/pin, basic profiles.

- **Phase 2**: media posts and upload pipeline, flairs (post and user), full mod dashboard (queue, reports, mod log, bans, permission tiers), report flow, search, notifications inbox, NSFW/spoiler handling, user-created Nodes with the creation wizard.

- **Phase 3**: custom/multi-node feeds, automod rule engine, direct messages and modmail, wiki, crossposting, Controversial/Rising/Top-timeframe and Q&A sorts, reputation gates, and optionally metamoderation and public mod logs as trust experiments (see Part 2 recommendation).

### 12.3 Reference implementations and licenses to study

- **Lemmy** (Rust, ActivityPub federation): AGPL-3.0. Study for federation and PostgreSQL schema; AGPL means any hosted fork must publish source. Created by the GitHub user Dessalines in February 2019, with an initial release on May 5, 2019.

- **Postmill** (PHP/Symfony; powers raddle.me): a self-hosted Reddit-like. Listed as Zlib-licensed (permissive) on AlternativeTo, though verify the repository license directly before reuse. Symfony itself is MIT.

- **reddit-archive/reddit** (Python/Pylons): the historical Reddit codebase, archived read-only since November 2017. Released under the Common Public Attribution License (CPAL) 1.1, a copyleft license with network-use and attribution clauses; this is the authoritative source for the ranking formulas but its license and age make it a reference, not a base to fork.

- **Tildes** (Python): AGPL-3.0, non-profit; study for trust-based governance ideas.

Pick a license posture early: a permissive base (Postmill/Symfony) allows a closed product; an AGPL base (Lemmy/Tildes) obligates you to open your hosted source.

---

# PART 2: SLASHDOT AND PEER BASELINE COMPARISON

Sources here are primarily the Slashdot FAQ (comments/moderation and metamoderation sections), the Hacker News FAQ and guidelines, the Lobsters about page, Lemmy's own site/repo, and the Tildes announcement. Where I lean on a blog or wiki, it is corroboration and labeled as such.

## 13. Slashdot vs the Reddit model

### 13.1 Editorial front page (the core difference)

Slashdot is editor-gated, not user-published. Users submit stories to a queue, and paid editors choose and write up which stories hit the front page. This is the founding contrast that Digg and then Reddit reacted against; as one history notes, "Kevin Rose created Digg because he didn't like how Slashdot editors controlled content instead of users." Slashdot's "firehose" is the raw stream of submissions and recently-published items that users can view and rate before/around editorial selection, a partial nod toward user-surfacing without giving up editorial control. In the Reddit model, by contrast, every accepted submission is immediately live in the target community's New queue and rises purely by votes.

### 13.2 Slashdot's moderation system

From the Slashdot FAQ (answers attributed to CmdrTaco, Slashdot founder Rob Malda):

- Moderation is a temporary privilege granted to eligible logged-in users, not a standing role. Eligibility requires being a logged-in, regular, longer-term reader who is "willing to serve" and has non-negative karma ("Positive, Good, or Excellent karma").

- The system periodically hands out tokens; accumulating enough makes you a moderator for a short while. Moderators receive a small number of points (the FAQ repeatedly references 5 points), and "each comment they moderate deducts a point."

- Moderation applies a labeled reason from a dropdown. Positive: Insightful, Interesting, Informative, Funny, Underrated. Negative: Offtopic, Flamebait, Troll, Redundant, Overrated. Each up or down moves the comment by a single point.

- Scores are bounded: "All comments are scored on an absolute scale from -1 to 5." Logged-in users start at 1 (can vary 0-2 by karma); anonymous users ("Anonymous Coward") start at 0.

- Points expire after 3 days if unused.

- Moderators "can not participate in the same discussion as both a moderator and a poster," and if you post in a discussion you moderated you do not get those points back, to prevent agenda-pushing.

- Reddit contrast: Reddit moderation is a persistent per-community role held by a mod team with broad remove/ban powers, and ordinary ranking is driven by unlimited user votes rather than scarce labeled points.

### 13.3 Metamoderation (M2) and hidden karma

- Metamoderation is "a second layer of moderation" that lets logged-in users "rate the rating" of randomly selected past moderations, judging each as fair, unfair, or neither. The FAQ describes M2 as showing a metamoderator ten randomly selected moderated comments. This feeds back into who keeps moderation eligibility, "encourag[ing] good moderators, and ideally remov[ing] moderator access from bad ones."

- Karma on Slashdot is a coarse label, not a public integer: the tiers are "Terrible, Bad, Neutral, Positive, Good, and Excellent." Slashdot deliberately hid the number: "People like to treat their Slashdot Karma like some sort of video game... The text label is one way we've decided to emphasize the point that karma doesn't matter." Karma is capped at Excellent to stop users becoming "immune from moderation."

- Reddit contrast: Reddit shows karma as a public number and has no metamoderation; quality control is votes plus mod removal.

### 13.4 Reading experience

- Readers set a score threshold; only comments at or above it display. Verbatim from the FAQ: "Comments are scored from -1 to 5... If you set your threshold to 2, only comments rated 2 or above would be displayed. Setting your threshold at -1 will display all comments. 0 is almost all comments. 1 filters out most Anonymous Cowards."

- Starting-score bonuses: logged-in +1 by default (a karma bonus can push new comments to start at 2; very bad users can be penalized toward -1), anonymous starts at 0. The karma bonus is stripped once a comment is moderated down twice.

- Anonymous posting ("Anonymous Coward") is a deliberate, retained feature: "We think the ability to post anonymously is important."

- Reddit contrast: Reddit hides low-score comments by collapsing rather than by a user-set numeric threshold, and generally requires an account to post.

### 13.5 Structural differences summary

- Single site with editor-chosen topic sections vs user-created communities.

- No downvote-driven story ranking (stories are editorially placed; only comments are moderated) vs vote-ranked everything.

- Chronological/editorial front page vs algorithmic vote-decay feed.

## 14. Peer comparisons (brief)

### 14.1 Hacker News

Single shared community, no user-created sub-communities. Ranking (from the open-sourced Arc `news.arc` and Paul Graham's confirmations) divides points by a power of age: score roughly `(points - 1) / (age_hours + 2)^gravity`, with gravity around 1.8 in published versions, plus penalties for controversy, "lightweight" content, and flags. Key mechanics from the HN FAQ and the community "undocumented" notes: stories cannot be downvoted, flags act as a "super downvote"; comment downvoting unlocks only above a karma threshold (documented at 501); comment scores are hidden to prevent bandwagoning; `[dead]` items are hidden unless the viewer enables `showdead`; high-karma users can `vouch` to restore killed items. Karma is "roughly the number of upvotes on their posts minus the number of downvotes," and higher karma does not make a user's stories rank higher.

### 14.2 Lobsters

Invite-only via a public invitation tree (each user is a branch off their inviter), created in 2012 by Joshua Stein after he was shadow-banned from HN, explicitly to prioritize moderation transparency. Tag-based rather than community-based: every submission carries predefined tags users can filter or subscribe to. Downvotes must pick a labeled reason (off-topic, spam, me-too, troll, unkind, etc.). A public moderation log records edits, deletions, and bans, and the site prohibits shadow banning. "Hats" let users post with a verified capacity (employee, project maintainer). New accounts are restricted for their first 70 days.

### 14.3 Digg v4 (cautionary tale)

Digg's August 25, 2010 v4 redesign let publishers auto-submit their RSS feeds to the front page, removed the "bury" (downvote) button, and stripped power-user tooling. Users revolted ("Quit Digg Day," upvoting Reddit links to fill Digg's front page). Digg traffic dropped sharply (Hitwise data via TechRadar put the fall at about 34% in the UK and 26% in the US), while Reddit surged: TechCrunch reported Reddit pageviews grew "from 250 million in January of 2010 to 829 million in December of 2010, a 232% growth." Lessons for the clone: do not privilege publishers over the community, do not remove the downvote/negative-signal that users rely on, and never ship a drastic redesign without beta-testing with your power users. Digg later reversed the RSS auto-submission feature.

### 14.4 Lemmy / kbin (the federation fork in the road)

Lemmy is an open-source (AGPL-3.0), self-hostable link aggregator where each server is an "instance" and instances federate over ActivityPub (the same protocol Mastodon uses), forming the "threadiverse." Users are Person actors and communities are Group actors; a community "announces" new posts to every subscribing instance. This is a genuine architectural fork: federation buys censorship-resistance and user data ownership but adds moderation, spam, and consistency complexity across servers. kbin/Mbin occupy the same federated niche. Your blueprint calls the platform "decentralized"; if that means true federation, Lemmy is the reference and the clone should design actors and an ActivityPub outbox early. If "decentralized" only means topic-decentralized (many communities on one server), skip federation for the MVP.

### 14.5 Tildes

Non-profit, donation-funded, open-source (AGPL-3.0), created by Chad Birch ("Deimos"), a former ~4-year Reddit developer. It is designed around trust-based governance: privileges are meant to be earned through demonstrated good participation, with mechanics intended to scale toward reddit-like activity, though in practice much moderation authority still sits with the founder during its alpha. It uses topic groups plus tags and emphasizes discussion quality over volume.

## 15. Feature / functionality matrix

Rows are mechanisms; columns are Reddit model, Slashdot, Hacker News, Lobsters, and the proposed prototype baseline.

| Mechanism | Reddit model | Slashdot | Hacker News | Lobsters | Proposed prototype |
|---|---|---|---|---|---|
| Who can publish stories | Any user, instantly to a community | Users submit; paid editors choose front page | Any user, instantly | Any invited user, instantly | Any member, instantly to a Node |
| Community creation | User-created subreddits | None; editor-chosen sections | None; single community | None; tag-based, single community | User-created Nodes (phase 2) |
| Voting model | Up/down on posts and comments | Labeled +1/-1 on comments only, by temp moderators | Up on stories; up/down on comments (down gated) | Up/down; down needs a labeled reason | Endorse/reject on posts and comments |
| Score visibility / caps | Public net score, no cap | Capped -1 to +5; per-comment | Comment scores hidden; no cap; min -4 | Scores shown; no hard cap | Net score shown; optional cap+floor (Slashdot-style) as option |
| Moderator selection | Persistent per-community mod teams | Temporary, randomly granted to eligible users (5 points) | Site admins + user flags | Small admin team + community flags | Per-Node mod roles; temp-mod as later option |
| Meta-moderation | None | Yes (M2) | No | No | Phase-3 experiment only |
| Karma visibility | Public number (post + comment) | Hidden coarse label (Terrible-Excellent) | Public number | Public-ish; profile karma | Split reputation; number shown, de-emphasized |
| Comment filtering | Collapse below threshold | User-set numeric score threshold | Desaturate downvoted; showdead | Collapse; tag filters | Collapse + optional score threshold |
| Anonymity | Pseudonymous accounts | Anonymous Coward posting allowed | Pseudonymous accounts | Invite-tied identities | Pseudonymous accounts |
| Ranking transparency | Open-source 2010-era; opaque today | FAQ-documented; editorial | Formula partly public; penalties opaque | Public code and mod log | Fully open, documented formulas |
| Federation | No | No | No | No | Optional; Lemmy/ActivityPub if true decentralization is required |

## 16. Recommendation: what to adopt from Slashdot, what to reject

**Adopt (fits the topic-driven model):**

- **Labeled moderation reasons.** Slashdot's dropdown reasons and Lobsters' labeled downvotes both reduce meta-argument and produce useful signal. Cheap to add and improves mod-log quality. Do this in phase 2.

- **Comment score caps and floors as an option.** Slashdot's -1 to +5 bound blunts pile-ons and dampens both bandwagoning and mob downvoting; offer it as a per-Node setting rather than a global rule.

- **User-set comment thresholds / aggressive collapse.** Slashdot's threshold reading model is a clean way to let readers self-select signal level; pair it with your collapse mechanic.

- **De-emphasized reputation.** Slashdot's hidden-number philosophy is worth partially borrowing: show reputation but do not gamify it, to discourage karma-farming.

- **Public moderation log (from Lobsters/Tildes).** Strong trust builder; ship as a per-Node toggle.

- **Metamoderation as a phase-3 trust experiment.** Valuable for large Nodes that want community-checked moderation, but only once there is enough volume for random sampling to work.

**Reject or defer (conflicts with the decentralized user-created model):**

- **Editorial front-page gating.** Paid editors choosing stories is the opposite of user-published communities and would undo the core premise. Reject.

- **Moderation-by-scarce-random-points as the only system.** Slashdot's temporary-random-moderator model does not map onto persistent per-Node mod teams that communities expect; keep standing mod roles and treat temp-moderation as an optional supplement, not the base.

- **Anonymous-by-default posting.** Useful on a single editorial site, but on user-created Nodes it complicates per-Node bans and reputation gates; keep pseudonymous accounts instead.

- **Hiding the karma number entirely.** Reddit-style users expect to see reputation; hide the gamification, not the signal.

## 17. Caveats and open questions

- The ranking formulas are transcribed from Reddit's archived open-source code, which is authoritative for 2010-2015 behavior but not for live Reddit today. Current Reddit personalizes feeds, weights votes by a contributor-quality score, and does not publish its ranking source. This spec follows the archived code and flags that gap rather than guessing at current internals.

- Reddit's exact karma dampening formula and its "rising" algorithm are not officially published. The spec uses the official "not 1:1" statement and implements approximations, labeled as such.

- Several peer-mechanic specifics (HN's 501-karma downvote threshold, the ~12.5-hour hot half-life, HN gravity ~1.8) come from community reverse-engineering and archived code snapshots, not always from current official docs; treat exact constants as historically accurate rather than guaranteed-current. Slashdot's cooling-off ban length is stated as temporary in the FAQ but not fixed to a specific number of hours, so no exact duration is asserted here.

- The word "decentralized" in the source blueprint is ambiguous. If it means federation, adopt ActivityPub (Lemmy pattern) and plan for cross-instance moderation from the start; if it means many communities on one server, no federation is needed for the prototype. Resolve this before Phase 1, because it changes the data model (actors, outboxes) fundamentally.

- Licensing of reference code matters: the archived Reddit code is CPAL, Lemmy and Tildes are AGPL-3.0, and Postmill is reported as Zlib. Verify each license in-repo before reusing code, and choose the clone's own license posture early.
