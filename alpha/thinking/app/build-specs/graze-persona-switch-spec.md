# Graze: Persona-Switch Prototype Spec

> **Provenance & status.** The build spec that produced the **Graze behavior-scale mock** (deployed at
> `graze.ing`; repo `CroftC/graze`). Authored in a claude.ai design dialogue (2026-07), filed
> **content-faithful** (cleaned-paste; PLAYBOOK §4). This is the persona-switch **mock** spec for the
> community-discussion forum pad (`NAMING.md` → "Forum-layer naming"; `graze.ing` supersedes the
> earlier `forum.croft.ing` domain — user, 2026-07-30). It re-substrates the **Topic-Driven
> Aggregation build spec** (`thinking/app/build-specs/graze-topic-aggregation-build-spec.md`) onto the
> behavior-scale methodology (`thinking/behavior-scale/`). **Design tension to track (COHESION):**
> §13 below assumes a Next.js + Postgres scaled sibling, while the corpus's active forum plan
> (`plans/2026-07-27-read-first-forum-mvp.md`) chose a read-first lens over the public Bluesky
> AppView — reconcile, do not silently merge. UX research it draws on:
> `research/reddit-aggregation-ux-slashdot-baseline-2026-07.md`. Raw session:
> `seeds/transcripts/raw/stellin-graze-behavior-scale-sessions-2026-07.md`.

---

**graze.ing** · Wordmark: **Graze** · Taglines: "Roam the open web." / "Feed your curiosity."

This document re-substrates the *Topic-Driven Aggregation Platform: Prototype Build Spec* (the "build spec") onto the *Persona-Switch Prototype* and *Behavior-Scale* methodologies. Nothing from the build spec is discarded: its data model becomes the event vocabulary, its API surface becomes the selector and action contract, its ranking math becomes pure engines, and its Postgres/Next.js stack becomes the future `api` substrate. What ships first is a static SPA on GitHub Pages where identity is a dev-bar dropdown and every UX path in the build spec is walkable from every seat.

## 0. Claim status convention

Carried from the build spec:

- **[source]** follows claims grounded in a primary source (the citation lives in the build spec, section noted).

- **[secondary]** marks blog/wiki-corroborated claims.

- Everything else in this document is our design.

## 1. What we are building, in one paragraph

Graze is a topic-driven aggregation site in the Reddit structural family: three-column desktop layout, community-scoped posting, universal +/- rating, hot ranking with time decay, deeply nested collapsible comments, volunteer stewards with a public audit log. Version one is a **behavior-scale mock with no production sibling yet**: the full front end running on in-memory reducers, browser persistence, deterministic seeds, and a persona switcher, built so that the same contract can later be pointed at a real backend capability by capability.

## 2. Brand

### 2.1 Wordmark and voice

- Wordmark: **Graze**, set in the brand green (tokens below), lowercase "g" acceptable in the app icon, full "Graze" in the header. The domain renders as `graze.ing` in footer and share URLs.

- Taglines: "Roam the open web." primary, "Feed your curiosity." secondary. Use one at a time (auth screen, logged-out banner, README), never stacked as a couplet in UI chrome.

- Voice: pastoral but not twee. The grazing metaphor is allowed exactly two structural appearances (Fields, and the empty-state copy "Nothing growing here yet"). Everything else is plain product language.

### 2.2 Nomenclature

The blueprint requires non-Reddit nomenclature. Graze's theme supplies it naturally:

| Concept | Graze term | Notes |
|---|---|---|
| Community node | **Field** | Routes use `/f/:slug`. "Join a Field." |
| Top-level submission | Post | No theming needed |
| Rating | **Boost / Bury** | Chevron up / chevron down, from the blueprint's suggested list |
| Net score | Score | |
| Accumulated standing | Reputation | Split post/comment as in the build spec |
| Node label | Tag | |
| Volunteer moderator | **Steward** | Already pastoral; carried straight from the build spec |
| Public mod log | Audit log | |
| Home feed | Home | "Your pasture" rejected: cute over clear |

## 3. Design tokens (Proposal)

All tokens as CSS custom properties in `/css/tokens.css`. No webfonts in the prototype layer; system stacks only.

### 3.1 Color

Brand anchor: dark leaf green. Palette built to complement it: warm paper neutrals so the green reads organic rather than corporate, a clay complement for the negative axis (red-orange is the approximate complement of leaf green), and a harvest gold accent used sparingly.

```css
:root {
  /* Brand green ramp */
  --green-950: #0F2314;
  --green-900: #1A3A22;
  --green-700: #2D5A34;   /* BRAND. Wordmark, primary buttons, boost-active */
  --green-600: #3A6E43;   /* hover, links */
  --green-500: #4C8656;
  --green-300: #A3C9AB;   /* subtle borders on green surfaces */
  --green-100: #E9F2EB;   /* tint backgrounds, joined-chip fill */

  /* Warm neutrals */
  --paper:     #FBFAF7;   /* app background */
  --surface:   #FFFFFF;   /* cards */
  --ink:       #20261F;   /* primary text, green-tinted near-black */
  --ink-soft:  #5C665C;   /* metadata, timestamps */
  --line:      #E3E1DA;   /* borders, the collapse gutter at rest */

  /* Complement + accents */
  --clay-600:  #A24D2C;   /* bury-active, controversial badge */
  --gold-500:  #C99A3C;   /* pinned posts, steward badge, tag default */
  --danger:    #9E2F26;   /* removals, ban states, destructive confirm */

  /* Dark theme */
  --dk-bg:      #121A14;
  --dk-surface: #1A241C;
  --dk-ink:     #E7EDE7;
  --dk-line:    #2A362C;
}
```

Rules:

- Boost-active is `--green-700`, bury-active is `--clay-600`. Idle chevrons are `--ink-soft`. Never color a score by sign at rest; only the user's own vote colors the control.

- `--gold-500` appears only on pins, steward badges, and tags. If gold shows up anywhere else, it is being overused.

- Contrast: `--green-700` on `--paper` and `--ink` on `--paper` must both pass AA; both do at these values by design, verify in the accessibility pass.

### 3.2 Type and space

- Body: `system-ui, -apple-system, "Segoe UI", Roboto, sans-serif`. Display (wordmark, Field titles, h1): system serif stack `ui-serif, Georgia, "Times New Roman", serif` used with restraint, per the prototype methodology's "characterful display face" guidance.

- 8-point spacing scale, radius 6px cards / 999px chips, type scale 13/14/16/20/26.

### 3.3 Signature element

The build spec names the collapse gutter as the interaction to make excellent. The token layer commits to it: the gutter is a 2px `--line` rule with a 24px hit target that fills `--green-700` on hover and animates the collapse. This is Graze's one signature element; spend distinctiveness here and keep everything else quiet.

## 4. Architecture: build spec concept → mock implementation

| Build spec section | Mock realization |
|---|---|
| §8 Data model (Postgres tables) | Event vocabulary (§5 below) + reducers deriving equivalent state shapes in memory |
| §9 API surface | Split into the action contract (writes) and selector contract (reads); same names survive into the future `api` substrate |
| §7 Ranking math | Pure functions in `/js/engines/rank.js`, unchanged formulas |
| §10 Governance | Policy in selectors; permission matrix enforced at read time so it holds on every surface |
| §11 Anti-abuse | The Limits engine: rolling-window counters over the event log |
| §12 Stack (Next.js + Postgres) | Deferred to the scaling layer. Prototype stack: no build step, vanilla ES modules, hash routing, GitHub Pages |
| §4 Route map | Same map, prefixed with `#` (hash routing) and `/n/` renamed `/f/` |

Repo shape, milestone discipline, dev bar, storage adapter, and PWA-last rule are exactly as the persona-switch methodology prescribes; this spec does not restate them.

## 5. The contract

### 5.1 Event vocabulary (Proposal)

Derived table-by-table from the build spec's data model. Actor and timestamp on every event; seed timestamps are offsets resolved at replay.

```
account.registered        {handle, email}
account.suspended         {userId, reason}            # site admin only
prefs.updated             {patch}                     # theme, comment threshold, feed defaults

field.created             {slug, title, description, settings}
field.settingsUpdated     {fieldId, patch}
field.joined              {fieldId}
field.left                {fieldId}

post.created              {fieldId, format: text|link|media, title, bodyMd?, url?, tagId?, nsfw, spoiler}
post.edited               {postId, patch}
post.deletedByAuthor      {postId}

comment.created           {postId, parentId?, bodyMd}
comment.edited            {commentId, patch}
comment.deletedByAuthor   {commentId}

vote.set                  {subjectType: post|comment, subjectId, value: -1|0|1}   # 0 retracts
save.set                  {subjectType, subjectId, saved: bool}

report.filed              {subjectType, subjectId, fieldId, reason, ruleId?, detail?}

mod.removed | mod.approved | mod.locked | mod.unlocked | mod.pinned | mod.unpinned
                          {subjectType, subjectId, reason}
mod.banned                {fieldId, userId, duration?, reason}
mod.unbanned              {fieldId, userId}
mod.stewardAdded          {fieldId, userId}
mod.stewardRemoved        {fieldId, userId}

notification.read         {notificationIds[]}
```

Reducer notes:

- Scores, comment counts, reputation, and unread badges are **derived**, never stored by hand: the reducers fold `vote.set` into per-item tallies and per-author reputation, exactly replacing the build spec's denormalized counters and nightly recount (drift is impossible when state is a fold over the log).

- `mod.removed` keeps the row, blanks body/author for non-stewards at the selector layer, matching the build spec's removal semantics.

- Every `mod.*` event **is** the audit log; the public log screen is a selector over them, so transparency is free, as the build spec predicted.

### 5.2 Selector contract (Proposal)

The read API of the future, expressed as pure functions now. All take `viewer` first; policy lives here.

```
feed(viewer, scope: home|all|popular|field:slug, sort, timeframe?, cursor?)
thread(viewer, postId, sort: best|top|new|controversial, focusCommentId?, context?)
field(viewer, slug)                      # about card, rules, stewards, join state
auditLog(viewer, slug, cursor?)          # PUBLIC: no viewer gating except removed-body masking
modQueue(viewer, slug, cursor?)          # steward-gated
profile(viewer, handle, tab)
notifications(viewer, cursor?)  unreadCount(viewer)
search(viewer, q, scope, type, cursor?)
permissions(viewer, fieldId?)            # the §10.1 matrix as a function
limits(viewer)                           # remaining post/comment budget, probation status
```

Policy enforced in selectors, tested from both seats: banned-in-Field users see the Field read-only with a ban notice; probation seats get rate-limit surfacing from `limits`; removed content masks body and author for non-stewards everywhere it can appear (feed, thread, profile, search).

## 6. Engines

`/js/engines/rank.js`, pure, variant-swappable per the mock's change-an-engine procedure.

- **Hot:** `sign(s) * log10(max(|s|,1)) + (created - 1134028003)/45000`, submission-time based, so rank updates only on vote events. **[source: build spec §7.1, reddit-archive `_sorts.pyx`]**

- **Best (comments):** Wilson score lower bound at z = 1.281551565545. **[source: build spec §7.3, same file]**

- **Controversial:** `(ups+downs) ** (min/max)`, zero if one-sided. **[source: build spec §7.2]**

- **Rising:** hot over posts younger than 6 hours with a minimum vote velocity; ours to tune (the archived code lacks it, per build spec §7.2).

- **Limits:** rolling windows over the viewer's own events: 1 comment/60s, 1 post/5min, doubled in probation, halved past the reputation threshold, plus the cooling-off slowdown after rapid down-rating. Precedents in build spec §11 **[source: Slashdot karma FAQ]**.

The seed's power-law vote distribution makes Hot vs New visibly different from the first replay, which is the point of having real math in a mockup.

## 7. Persona roster (Proposal)

Coverage-driven, eight seats plus Logged out. Every row in the build spec's permission matrix is reachable from at least one seat.

| Seat | Handle | Covers |
|---|---|---|
| 0 | **Logged out** (dropdown default) | Public reads, auth-gate prompts on every write |
| 1 | `admin.wren` | Site admin: suspend account, close Field, sitewide queue |
| 2 | `owner.sage` | Owner of Field `gardening`: settings, steward mgmt, rules editor |
| 3 | `steward.briar` | Steward of `gardening` + plain member elsewhere: mod queue, remove/lock/pin/ban, the dual-hat experience |
| 4 | `member.fern` | Established member: full rights, joined 4 Fields, the default "reader" seat |
| 5 | `newbie.moss` | Probation: registered 3 days ago, rate-limited, cannot create Fields, low report weight |
| 6 | `banned.thorn` | Banned from `gardening` (ban visible in its audit log), active elsewhere: tests ban UX from the receiving end |
| 7 | `heavy.aspen` | High reputation, sitting at the post rate limit in seed, saved items populated: tests limit surfacing and saved/profile density |
| 8 | `pristine.dove` | **Never receives seed activity.** First-run and empty states, forever |

Relationship graph wired so seat 4 (default logged-in seat) can see, at once: a Field they steward nothing in, a pinned post, a locked thread, a removed comment stub, and `banned.thorn` posting normally in a shared Field. In-flight states scripted near the end of the seed: one open report in `gardening`'s queue (seat 3 can action it), one unread reply for seat 4, one automod-held post for seat 2's review.

## 8. Seed scenario (Proposal)

`/data/seed.js`, one scripted log, ~70 events plus one generated stress thread, deterministic:

- **5 Fields:** `gardening` (large, governed, the mod-flow stage), `urbanism`, `retrocomputing`, `slowcooking`, and `meta` (site announcements, admin-posted).

- **Content variety per the methodology:** one long text post that truncates at the 4-line clamp, one link post with domain display and a scripted duplicate (submission wizard's dupe detection has something to find), one NSFW-flagged, one spoiler-flagged, one pinned, one locked, one `mod.removed` with its audit entry, tags on every `gardening` post since that Field requires them.

- **The stress thread:** a generator function (seeded PRNG, fixed seed constant, resolved at replay so determinism holds) producing a ~1,000-comment tree in `gardening`, depth up to 14, power-law votes. This is the acceptance target for collapse performance and continuation stubs, mirroring the build spec's M1 criterion.

- **Governance history:** 6 to 8 `mod.*` events so the public audit log renders a believable page one, including `banned.thorn`'s ban with reason.

## 9. Routes and screens: deltas from the build spec

The build spec's §4 route map and §5 screen specs carry over wholesale with these changes:

- Hash routing, `/n/` → `/f/`: `#/f/gardening`, `#/f/gardening/p/:id/:slug`, `#/f/gardening/mod/log`, etc. `#/frontiers` added.

- **Dev bar** above the header, dashed background: persona dropdown, Seed, Delete All, Export/Import, latency toggle (0/250/600ms), Fail Next Action, frontier toggle, SW unregister. Persona switch re-derives all viewer-dependent caches in one place.

- Voting stays optimistic UI; the rollback path is only observable with Fail Next armed, which is why the dev bar has it.

- The submission wizard runs the same four linear steps; automod evaluation at step 4 runs the Field's rules from `field.settings` in a time-boxed evaluator, and `hold` outcomes route to the queue exactly as specced.

- All four screen states (skeleton under latency, empty, error via Fail Next, gated) on every screen, per the methodology.

## 10. Frontier markers at launch (Proposal)

Registered in `/js/frontier.js`, rendered as dashed chips, listed at `#/frontiers`:

- Media upload (post format tab present, locked): storage budget decision deferred; text and link posts only in v1.

- Search facets beyond type filter.

- Custom multi-Field feeds (build spec §13 deferred this too).

- Metamoderation-style steward review (deferred in build spec §10.4).

- Vote-ring detection, IP reputation, ML spam (build spec §11 out-of-scope list; the chip text says "requires the scaled side").

## 11. Build order

The methodology's 12-step order, with the build spec's milestones mapped in:

1. **Scaffold + contract:** tokens, hash router, store with schema-versioned localStorage adapter, event schema with payload validation from day one (the contract layer pulled forward: it costs a file now and saves a refactor later), dev-bar shell, Delete All.

2. **Seed + safety:** the §8 seed, Export/Import, latency, Fail Next.

3. **Core loop (= build spec M0/M1):** Fields, text posts, votes with optimistic UI, hot/new/top/controversial feeds, nested comments with the collapse gutter, Best sort, continuation stubs, join/leave with Home feed, link posts.

4. **Identity surfaces:** profiles, prefs including the reader score threshold.

5. **Governance (= M2):** reports with typed reasons, mod queue with j/k+a/r keys, remove/lock/pin/ban, public audit log, tags, automod rules, probation + limits engine.

6. **Notifications, saved, search** (= M3 minus media).

7. **Logged-out views + real signup** (creates a genuinely new persona in the dropdown).

8. **Frontier registry screen.**

9. **PWA layer last**, then mobile polish, accessibility pass, acceptance run.

Acceptance gates worth keeping verbatim from the build spec: the 1,000-comment thread loads fast and collapses smoothly on a phone; a steward can run `gardening` for a session without touching the console.

## 12. Acceptance checklist (Proposal, excerpt to extend to ~20)

- Fresh load, cleared storage: Logged out on `#/popular` with a Join CTA and the tagline "Roam the open web."

- After Seed, seat 3 opens `gardening`'s queue, actions the open report with a typed reason; the audit log gains the entry; the reporter seat gets an "actioned" notification.

- Seat 6 opens `gardening`: read-only with ban notice and the ban visible in the public log; posts fine in `retrocomputing`. Verified nothing leaks in feed composition from other seats.

- Seat 5 posts twice quickly: second attempt blocked with the probation rate-limit message naming the wait.

- The stress thread: collapse a depth-2 comment with ~400 descendants in one interaction; "continue this thread" appears past depth 10; "load N more replies" works.

- Sort toggle Hot→New→Controversial visibly reorders the seeded `gardening` feed.

- With latency 600 + Fail Next: a boost fills green then reverts with an error toast.

- Export → Delete All → Import restores exactly; seat 4's unread count spot-checked.

- Seat 8 (pristine): empty Home with "Nothing growing here yet" and a Field-discovery CTA.

- Deployed on Pages under the subpath: installable, offline shell reload, `?nosw` bypasses.

## 13. Later layers

Unchanged from the mock methodology: adapter layer with `memory` as sole substrate once the contract stabilizes, conformance harness when the first capability grows an `api` implementation (the build spec's Next.js + Postgres stack is that implementation's spec, already written), community edition via the `sync` substrate with the honor-system identity model named in the README. The divergence ledger starts empty; the §10 frontier list becomes its first entries.
