# Build Prompt: "Meridian" / Stellin — a self-contained professional networking PWA

> **Provenance & status.** The build prompt that produced the **Stellin behavior-scale mock** (deployed
> at `stellin.app`; repo `CroftC/stellin`). "Meridian" was the working name during the build; the
> product was renamed **Stellin** afterward (`js/config.js` holds the product name). Authored in a
> claude.ai design dialogue (2026-07), filed **content-faithful** (cleaned-paste; PLAYBOOK §4). This
> is the persona-switch **mock** spec; its intended scaled sibling is the atproto "Stellin by Croft"
> AppView (see `seeds/stellin-unpacked/` RUN-14/15 and `research/stellin-name-clearance-2026-07.md`).
> Methodology: `thinking/behavior-scale/`. UX research it draws on:
> `research/linkedin-ux-architecture-2026-07.md`. Raw session:
> `seeds/transcripts/raw/stellin-graze-behavior-scale-sessions-2026-07.md`.

---

You are building a complete, self-contained single-page application that looks, feels, and functions like a first-class professional networking site (LinkedIn-caliber), with zero backend. It exists for hands-on design and UX testing across mock personas before a real backend is built. Everything below is the specification. Where a decision is not specified, choose the simplest option that preserves the user experience, and record the decision in `DECISIONS.md`.

"Meridian" is a working title. Put the product name in one place (`js/config.js`) so it can be renamed later.

---

## 0. Mission and hard constraints

**What this is:** a fully functional professional-network UI (feed, profiles, connections, jobs, messaging, notifications, search, company pages, settings) where a tester switches between mock personas via a dev bar and experiences the site as each of them. All data lives in the browser.

**Hard constraints:**

- No build step. No frameworks. No runtime dependencies. Native ES modules, vanilla JS, modern CSS with custom properties, semantic HTML5.

- Deployed on GitHub Pages over https as a project site. All URLs (assets, imports, service worker registration, manifest paths) must be **relative** (`./`), never root-absolute (`/`), because project pages are served under `https://<user>.github.io/<repo>/`.

- Routing is hash-based (`#/feed`, `#/in/maya-chen`, `#/jobs/j1`, `#/company/brightpath`, `#/search?q=...`, `#/messaging`, `#/notifications`, `#/settings`, `#/network`, `#/signup`). Hash routing needs no server config and survives deep links on static hosting.

- Persistence is `localStorage` under a single versioned key (`meridian.v1`). No cookies, no network calls at runtime. Assume a practical budget of about 5 MB; keep the seeded state under 1.5 MB.

- There is no real authentication. "Logged in as" is a client-side session pointer set by the dev bar. Standard auth UI elements (sign in, sign out, join) exist visually and drive the persona system.

- Mobile is a v1 requirement, not an afterthought. Light mode only for v1.

- Works fully offline after first load (service worker precache; all data is local anyway).

---

## 1. Repository layout

Shape it like a real production site, because it will be iterated on as one:

```
/index.html
/manifest.webmanifest
/sw.js
/css/
  tokens.css        (design tokens: custom properties only)
  base.css          (reset, typography, layout primitives)
  components.css    (cards, buttons, chips, modals, toasts, badges)
  screens.css       (per-screen layout rules)
/js/
  config.js         (product name, feature flags, latency setting default)
  main.js           (boot, router mount, dev bar mount)
  router.js         (hash router + scroll restoration + focus management)
  store.js          (localStorage load/save, schema version, export/import)
  events.js         (event types, dispatch(), reducers, derived caches)
  actions.js        (user-facing actions; the ONLY way UI and seed mutate state)
  frontiers.js      (frontier chip registry)
  engines/
    graph.js        (BFS degrees, first-degree cache)
    feed.js         (candidate generation + scoring)
    pymk.js         (people you may know)
    search.js       (typeahead index + faceted search)
    notify.js       (aggregation reducer)
    rate.js         (invite rate limiting)
  components/       (navbar, devbar, feedcard, composer, reactionpicker,
                     hovercard, doccarousel, modal, toast, badge, tabs, ...)
  screens/          (one module per route)
/data/
  personas.js       (accounts + entities: companies, school, skills, jobs)
  seed.js           (the seed event log)
/assets/icons/      (PWA icons)
/tools/make-icons.mjs  (one-off node script to generate PNG icons; dev tooling,
                        not a runtime build step)
DECISIONS.md
```

---

## 2. Core architecture: event log over reducers

State has two layers:

- **Canonical:** `{ schemaVersion, accounts[], entities: {companies, schools, skills, jobs}, events[], session: {activePersonaId|null}, settingsByUser }`.

- **Derived:** feeds, connection sets, degree caches, notification groups, unread counts, profile completeness. Recomputed from the event log on load and incrementally after each dispatch. Derived data is never persisted.

Every mutation is an event: `{ id, ts, actorId, type, payload }`. The UI never writes state directly; it calls functions in `actions.js` (for example `actions.createPost(...)`, `actions.react(...)`, `actions.acceptInvite(...)`), which validate, append the event, update derived caches, and return optimistically.

**The seed is an event log replayed through those same actions.** This is the load-bearing design decision: seeded data is indistinguishable from data a tester creates by hand, and notifications, badges, and feed ordering all derive automatically.

Canonical event types (keep the vocabulary this tight):

`account.created`, `profile.updated`, `position.added`, `education.added`, `skill.added`, `endorsement.added`, `recommendation.requested`, `recommendation.published`, `connection.requested`, `connection.accepted`, `connection.removed`, `invite.withdrawn`, `follow.added`, `companyFollow.added`, `block.added`, `post.created`, `comment.added`, `reaction.added`, `reaction.removed`, `repost.created`, `message.sent`, `message.read`, `conversation.created`, `job.created`, `job.applied`, `application.stageChanged`, `notification.read`, `settings.changed`, `profile.viewed`.

**Timestamps:** seed events specify offsets relative to "now" (for example `-3d4h`, `-45m`), resolved to absolute ms at seed time. The feed always looks recently alive; re-seeding refreshes it.

**Latency simulation:** all `actions.*` calls are async and await `simulateLatency()`, which resolves immediately when latency is off, or after 250 ms ± 100 ms jitter when on (dev bar toggle). Skeletons and optimistic UI only visibly exercise themselves when latency is on; build them regardless.

---

## 3. The dev bar (persona harness)

A visually distinct scaffolding strip pinned above the real top nav. It must read as tooling, not product: dashed bottom border, faint diagonal-stripe background, monospace `DEV` label. Contents:

- **Persona dropdown.** Default entry is **Logged out**. Then the nine personas, then any accounts created via the live signup flow. Switching personas resets the session, recomputes derived caches (degrees, unread counts), and re-renders the current route from the new perspective.

- **Seed** button: confirm dialog, then wipe and replay the full seed event log. Deterministic: seeding twice yields the same event ids and the same structure.

- **Delete All** button: confirm dialog, then reset to bare state: the nine accounts exist with only name, headline, and avatar; entities (companies, school, skills) exist; no connections, posts, messages, jobs applications, or profile sections. This is the "site with users but no data" state for testing onboarding and empty states.

- **Export JSON / Import JSON**: download the current canonical state; import replaces it. This is how exact repro states get shared.

- **Latency toggle** (off / on).

- **Frontiers panel**: expands to list every registered frontier chip in the build (see section 4).

Live-created data layers on top of either baseline until the next Seed or Delete All.

---

## 4. Frontier chips (naming the unbuilt)

A **frontier chip** is a small dashed-outline chip rendered inline where a real product would offer a path we have deliberately not built. Label format: `frontier: <name>`. Clicking one shows a toast: "Not built yet. Tracked as <name>." Every chip is registered in `js/frontiers.js` with a name and one-line description, and the dev bar lists them.

Initial registry:

- `premium-checkout` (upgrade/purchase flow behind premium gates)

- `pdf-import` (uploading a real PDF as a document post; v1 documents are generated pages)

- `video-post` (video upload and playback)

- `bulk-pipeline-actions` (multi-select actions in the employer console)

- `email-notifications` (digest emails)

- `password-reset` (auth recovery)

- `group-pages` (groups vertical)

- `analytics-dashboard` (creator/company analytics beyond simple counts)

Rule: if you hit a flow mid-build that would balloon scope, do not silently omit the entry point. Render the entry point with a frontier chip and register it.

---

## 5. Personas, entities, and the seed

### 5.1 Accounts (exist even after Delete All)

| id | Name | Headline | Tier | Purpose |
|---|---|---|---|---|
| maya | Maya Chen | Senior Product Manager at Northwind Labs | Free | Default well-rounded seat |
| marcus | Marcus Webb | Technical Recruiter at TalentBridge | Premium (5 outreach credits) | Paid features, sender-side outreach, employer console |
| jordan | Jordan Ellis | Software Engineer, open to work | Free | Job seeker, upsell surfaces, Requests recipient |
| sam | Sam Okafor | (headline empty) | Free | Pristine forever; onboarding and empty states |
| priya | Priya Sharma | CS Senior at Lakeview University | Free | Student branch, alumni suggestions, internships |
| david | David Park | Independent Data Analyst and Writer | Free | Creator: few connections, many followers, heavy content |
| elena | Elena Rodriguez | Founder at Brightpath Health | Free | Company page admin, small employer |
| alex | Alex Kim | Security Engineer | Free | Privacy-locked: private viewing mode, hidden connections, has a block |
| tony | Tony Russo | Sales Development Representative at Atlas Logistics | Free | At the weekly invite cap; low-quality outreach patterns |

Avatars are deterministic inline SVGs: initials on a color derived from a hash of the id. Company logos are SVG monograms. No image files for identity.

### 5.2 Entities

- Companies: Northwind Labs, TalentBridge, Brightpath Health, Atlas Logistics.

- School: Lakeview University.

- Skills: seed about 30 (Product Management, Roadmapping, SQL, Python, JavaScript, React, Data Analysis, A/B Testing, Technical Recruiting, Sourcing, UX Research, Public Speaking, Team Leadership, Agile, Security Engineering, Threat Modeling, Cloud Infrastructure, Sales Prospecting, CRM, Healthcare Operations, Fundraising, Content Strategy, Statistics, Machine Learning, Communication, Negotiation, Project Management, Figma, Accessibility, Go).

- Jobs (created in seed, owned by employers): Brightpath Health "Founding Frontend Engineer" (remote, mid-senior), TalentBridge on behalf of a client "Data Platform Engineer" (hybrid, senior), Atlas Logistics "Operations Analyst" (on-site, entry), Brightpath Health "Product Design Intern" (remote, internship). Jobs span every facet at least once (work mode, seniority, salary band present/absent).

### 5.3 Connection graph (seed)

Edges (all accepted unless noted): maya-jordan, maya-david, maya-elena, jordan-priya, jordan-alex, elena-marcus, elena-david, marcus-tony.

Follows (asymmetric): maya, jordan, priya, marcus, elena, alex, and tony all follow david; maya and jordan follow Brightpath Health (company).

Verification this graph must produce from Maya's seat: Jordan/David/Elena 1st degree; Priya/Alex/Marcus 2nd; Tony 3rd; Sam out of network.

### 5.4 In-flight states (seeded near the end of the log so they are pending)

- Priya → Maya: pending connection invitation with a note ("We overlapped at the Lakeview product club panel, would love to connect").

- Marcus → Jordan: cold outreach message sitting unread in Jordan's **Requests** tab (one credit spent; refund on reply).

- Alex has blocked Tony (their content and profiles are mutually invisible everywhere: feed, search, PYMK, typeahead).

- Tony is at the weekly invite cap with several pending sent invitations and one withdrawn.

- Maya has an unread aggregated notification ("Elena and 2 others reacted to your post").

- Jordan has two applications in different pipeline stages (one Screening at Brightpath, one Applied at TalentBridge's client role).

### 5.5 Seed content (about 45 to 55 events, chronological)

Write realistic professional copy. No real companies, real people, or brand names. No lorem ipsum. Anchor posts, verbatim:

- David, document post, "Six charts on remote work five years in" (5 generated pages of simple SVG charts): "I pulled five years of survey data on remote and hybrid work into six charts. Page 4 surprised me most: commute time saved does not convert to leisure, it converts to more meetings. Full methodology on the last page."

- David, long text post (must exceed truncation so "see more" is exercised): begins "Unpopular opinion after ten years in analytics: most dashboards are write-only..."

- Jordan, open-to-work post: "After four great years, my role was eliminated in last month's reorg. I build reliable, boring-in-the-best-way backend systems, mostly Go and Postgres. If your team needs someone who writes the runbook before the outage, my DMs are open." Draws Support reactions and encouraging comments from maya, priya, elena.

- Elena, posted **as Brightpath Health**: "We are hiring a founding frontend engineer. Small team, real patients, no growth hacks." Links the job.

Fill the rest in kind: Maya's product-launch lessons post (comments from jordan and elena) and her quote-repost of David's carousel; Marcus's "we're hiring" post and his comment on Jordan's post; Priya's internship announcement with a high reaction count and a question comment on David's carousel that David replies to (nested thread); Elena's personal post; Alex's one old low-engagement post; Tony's one promotional post with zero engagement. Endorsements sprinkled across skills; one published recommendation (Elena → Maya) and one pending (Jordan requested from Maya).

**Sam receives no seed events.** Sam stays pristine even after seeding.

---

## 6. Screen inventory

Every screen defines: layout, components, all states (empty, loading skeleton, error, permission-gated), and mobile behavior. Global chrome: top nav with omni-search, tabs (Home, Network, Jobs, Messaging, Notifications), avatar menu; unread badges on Messaging and Notifications.

1. **Logged out (default).** Public marketing-lite home, public read-only profiles and company pages, Join and Sign in CTAs on every gated action. Sign in opens a modal explaining this build uses the persona switcher. **Join now runs the real signup flow.**

2. **Signup and onboarding** (`#/signup`). Steps: identity (name), branch question (student / employed / looking), position or education entry via entity typeahead, skippable "import contacts" screen (explains it is simulated; offers suggested people instead), first-connection grid, photo step (upload, downscaled, or keep generated avatar). Emits real events; the new account appears in the persona dropdown and the session switches to it. Profile completion meter weights: photo 20, headline 15, about 10, first position or education 25, second section 15, three skills 15.

3. **Feed** (`#/feed`). Three columns desktop: identity rail (avatar, headline, profile views count, connection count), composer + infinite feed, right rail (news module with 3 static items, PYMK module, one clearly-labeled house ad slot). Feed card anatomy: author block (avatar, name, headline, timestamp, degree badge), text with "see more" truncation, media (image or document carousel), social proof line ("You and 12 others", comment count), action bar (React, Comment, Repost, Send). Infinite scroll via IntersectionObserver, cursor pagination over the scored list, skeleton cards, scroll restoration on back-navigation.

4. **Profile** (`#/in/:handle`). Stacked cards: hero (cover, avatar, name, headline, location, degree badge and mutual-connections line for visitors, Connect/Message/More CTAs), About, Experience, Education, Skills (endorsement chips, endorse action for 1st-degree), Recommendations (pending items visible only to the recipient with Approve/Dismiss). Own view shows edit pencils opening modals with entity typeahead and an "add new company/school" fallback that creates a lightweight unverified entity. Visiting a profile emits `profile.viewed`, respecting the viewer's viewing mode (section 8).

5. **Network hub** (`#/network`). Received invitations (accept/ignore, note preview), Sent (withdraw; withdrawal does not free the weekly window), PYMK grid, connections roster (search, sort, remove).

6. **Jobs, seeker side** (`#/jobs`). Two-pane: scrollable job cards left, sticky detail right (single column + push navigation on mobile). Facet rail: work mode, seniority, salary present, date posted, company. Quick Apply confirms then snapshots profile data into the application (later profile edits do not alter submitted applications). Saved searches with an alert toggle (alerts render as notifications). "My applications" list shows each application's current stage.

7. **Employer console** (`#/talent`, visible to marcus and elena). Job post editor (draft/publish/close) and the **candidate pipeline**: applicants listed per job with a stage dropdown per row: Applied → Screening → Interview → Offer → Closed (Hired or Declined). Changing a stage emits `application.stageChanged` and notifies the applicant. Multi-select bulk actions render as `frontier: bulk-pipeline-actions`.

8. **Messaging** (`#/messaging`). Desktop: persistent collapsible widget bottom-right plus full inbox route. Mobile: full-screen only, entered from a top-bar bubble icon. Tabs: Focused and Requests. Cold outreach to a non-connection requires a credit (premium only); the message lands in the recipient's Requests; the credit refunds when they reply. Zero credits gates the composer with an upsell containing `frontier: premium-checkout`. Read receipts are real within this build: opening a thread as the recipient persona emits `message.read`, and the sender persona sees the receipt after switching back. Optimistic send with sending/failed-retry states.

9. **Notifications** (`#/notifications`). Grouped reverse-chronological list using aggregation keys ("Elena and 2 others reacted to your post"), type filters, mark-read on view, unread badge derives from the log.

10. **Search** (`#/search?q=`). Omni-typeahead in the nav (people, companies, school, skills, jobs; degree-boosted). Results page with vertical tabs: People, Jobs, Companies, Posts; a facet rail per vertical. Free-tier gating: 3rd-degree names blur in People results with an upsell row (`frontier: premium-checkout`); premium personas (marcus) see them clear. Hashtag links route here filtered to posts.

11. **Company pages** (`#/company/:handle`). Visitor: hero (logo, name, industry, follower count, Follow), About, Posts, Jobs, People (members with positions there). Admin (elena on Brightpath): composer posting **as the company**, job management shortcuts, simple follower count; deeper analytics is `frontier: analytics-dashboard`.

12. **Settings** (`#/settings`). Account (name, headline), Visibility: profile viewing mode (full / semi-private "A recruiter at TalentBridge" / anonymous; choosing anonymous on a free tier hides your own viewer list), who-can-see-my-connections (everyone-1st-degree / only me; mutual connections always visible), Blocking (list, unblock), Data (links to dev-bar export).

---

## 7. Engines (pure functions in `js/engines/`)

- **graph.js.** First-degree sets cached per user. Degree badge: BFS over connection edges capped at depth 3; label 1st, 2nd, 3rd, or "Out of network". Recompute cache on persona switch and connection events. Blocked pairs are removed from the graph before any traversal.

- **feed.js.** Candidates: posts and reposts authored by 1st-degree connections, followed people, followed companies, and self. Score: `1.0*exp(-ageHours/36) + 0.4*ln(1+reactions) + 0.6*ln(1+comments) + 0.8*affinity + 0.5*firstDegree + 0.3*followedOnly`, where affinity is the viewer's prior interactions with the author normalized to 0..1. Weights live in one exported object; deterministic tie-break by timestamp then id.

- **pymk.js.** Second-degree candidates via friends-of-friends. Score: `1.0*mutualCount + 0.5*sameCompany + 0.5*sameSchool + 0.25*sameIndustry`. Exclude connected, pending either direction, blocked, and self. Top 8.

- **search.js.** Lowercased prefix index over people, companies, school, skills, job titles; people results boosted by closeness (1st > 2nd > 3rd). Faceted search filters then computes facet counts on the filtered set.

- **notify.js.** Reducer keyed by `aggregation_key` (for example `reaction:post:p7`). Renders "Actor and N others". Unread count is the number of unread groups.

- **rate.js.** Rolling 7-day invite counter per user, cap 100 (configurable). Soft warning banner at 90, hard block at 100. Withdrawn invites still count toward the window. Tony seeds at the cap.

---

## 8. Interaction mechanics (the feel)

- **Reaction picker:** hover 350 ms on desktop, long-press 450 ms on mobile, on the React button. Five reactions: Like, Celebrate, Support, Insightful, Funny, each with an accent color from the token file. Tap without holding applies Like. Optimistic: fills instantly, reverts with a toast on simulated failure.

- **Comments:** flat list plus exactly one level of replies; reactions on comments; composer with the same mention typeahead.

- **Reposts:** instant repost or "repost with your thoughts" (quote card embedding the original).

- **Mentions and hashtags:** `@` opens typeahead over connections and companies; selection stores an entity token rendered as a link. `#` tokens create or link hashtags.

- **Document carousel:** paged viewer with dots and arrow keys on desktop, swipe on mobile; v1 documents are arrays of generated SVG pages (`frontier: pdf-import` on the upload path).

- **Images:** user uploads pass through a canvas downscale (longest edge 800 px, JPEG quality ~0.72) before storage, to respect the quota.

- **Hover profile card:** desktop only, on names and avatars, 300 ms delay; mini profile with headline, degree, mutuals, Connect/Message.

- **Optimistic everywhere:** reactions, connection requests, message send, endorsements. Failure paths only trigger with latency on plus a hidden dev "fail next action" hook; still implement revert + toast.

- **Toasts:** aria-live polite, action-consistent copy ("Invitation sent", "Post shared").

- **Skeletons** mirror final layout for feed cards, profile cards, message threads, and search results.

---

## 9. PWA shell

- `manifest.webmanifest`: name, short name, `display: standalone`, theme and background colors from tokens, icons 192 and 512 (generated once by `tools/make-icons.mjs`, committed).

- `sw.js`: precache the full static shell with a `VERSION` constant in the file; bump the constant to invalidate. `skipWaiting` + `clientsClaim`. Cache-first for same-origin GETs. Registration is relative (`./sw.js`) and skipped when `?nosw` is in the URL or `localStorage['meridian.devNoSW']` is set (dev escape hatch for the classic stale-cache trap).

- Service workers require https or localhost; GitHub Pages provides https, and local dev runs through a static server (`npx serve .`), never `file://`.

---

## 10. Design system

Follow this brief exactly; where it leaves an axis free, make a deliberate choice and write it in `DECISIONS.md`. Do not default to generic AI-design cliches (cream background with terracotta accent, dark page with acid green, or fake-broadsheet hairlines). This is a calm, trustworthy professional tool.

- **Tokens (`css/tokens.css`):** container max 1128 px; desktop rails 225 / 552 / 300 with 24 px gutters; 8-point spacing scale (4/8/12/16/24/32); card radius 8 px; card padding 16 px; 1 px neutral borders on white cards over a cool light-gray canvas.

- **Color:** pick a professional trust-blue as primary that is clearly not LinkedIn's brand blue (#0A66C2 is off-limits; choose and name your own, for example a slightly deeper slate-blue), a neutral gray ramp, one success green, one warning amber, and five reaction accents. Name all of them as custom properties.

- **Type:** system font stack for body (16 px base, line-height 1.5); a distinctive but professional display treatment for names and screen titles (weight and tracking, not a webfont, to stay dependency-free); modular scale ratio 1.25; captions 13 px.

- **Signature element:** the **degree badge**, a small circular mark next to names whose ring count encodes degree (one ring 1st, two rings 2nd, three 3rd, dotted for out-of-network). It appears everywhere identity appears and doubles as the tier-gating visual (blurred name keeps its dotted badge). Make it crisp; it is the one memorable thing.

- **Breakpoints:** under 640 px single column with a bottom tab bar (Home, Network, Post, Notifications, Jobs) and messaging via a top-bar bubble; 768 px center + right rail; 1024 px and up full three columns.

- **Quality floor:** visible keyboard focus everywhere, focus moves to the main heading on route change, modals trap focus and close on Escape, `prefers-reduced-motion` respected, all interactive elements reachable by keyboard including the reaction picker (arrow keys select, Enter applies).

- **UX writing:** active voice, sentence case, buttons say what they do and keep their name through the flow ("Send invitation" → toast "Invitation sent"). Empty states are invitations to act ("Your feed is quiet. Follow people and topics to fill it."), errors say what happened and what to do next, never apologize vaguely.

---

## 11. Build order (execute as milestones; each ends runnable)

1. **M1 Shell.** index.html, tokens, base CSS, router, store, event/dispatch core, dev bar with persona dropdown (Logged out default), Delete All, Export/Import, latency toggle. Stub screens render their route names.

2. **M2 Seed + read-only feed.** personas.js, seed.js replaying through actions, feed engine, feed cards with degree badges, skeletons, infinite scroll. Logged-out public home.

3. **M3 Creation.** Composer (text, mentions, hashtags, image, generated document), reaction picker, comments and replies, reposts, toasts, optimistic UI.

4. **M4 Identity and graph.** Profile view and edit modals, entity typeahead with add-new fallback, endorsements, recommendations, network hub, invitations, rate limiting, PYMK, hover cards.

5. **M5 Comms.** Messaging widget and inbox, Focused/Requests, outreach credits with refund, read receipts across persona switches, notifications center with aggregation, unread badges.

6. **M6 Work.** Jobs seeker side with facets and Quick Apply snapshots, saved searches, employer console with the stage pipeline, company pages with post-as-company.

7. **M7 Edges.** Search verticals and facets with 3rd-degree blur, settings/privacy (viewing modes, connections visibility, blocking), signup/onboarding creating live personas.

8. **M8 Ship.** Manifest, service worker, icons, mobile polish pass, frontier registry audit, run the acceptance checklist and fix failures.

---

## 12. Acceptance checklist (self-verify before calling it done)

- Fresh load shows the logged-out home; the persona dropdown reads "Logged out".

- Delete All → nine personas in the dropdown, every profile bare (name, headline, avatar only), feed and network screens show designed empty states.

- Seed → Export, Seed again → Export: identical event count and ids.

- As Maya: degree badges show Jordan/David/Elena 1st, Priya/Alex/Marcus 2nd, Tony 3rd, Sam out of network; Priya's invitation is pending with its note; the aggregated unread notification renders.

- As Jordan: Marcus's message is in Requests; replying moves the thread to Focused; switching to Marcus shows the credit refunded (5/5) and a read receipt.

- Alex and Tony never see each other anywhere: feed, search, typeahead, PYMK, profiles.

- As Tony: any Connect attempt shows the cap block; the sent list shows pending invites and one withdrawn.

- As Elena: can post as Brightpath Health; the post renders with company authorship in followers' feeds; visitors see no admin tools on the company page.

- Change Jordan's Brightpath application from Screening to Interview as Elena; as Jordan, the application list and a notification reflect it.

- Join now from logged out creates a working new persona that appears in the dropdown, with the completion meter arithmetic correct at each step.

- Sam remains pristine after Seed.

- At 375 px wide: bottom tabs present, three-column layout gone, reaction long-press works, document carousel swipes, messaging is full-screen.

- Airplane-mode reload after first visit: the app loads and works.

- Keyboard-only pass: post, react, comment, connect, and message without a pointer.

- No console errors on any route in either seeded or bare state; seeded localStorage under 1.5 MB; the app installs as a PWA from the deployed GitHub Pages URL.

---

## 13. Content and legal rules

Clean-room throughout: no LinkedIn branding, copy, or assets; no real people, companies, or products in any seeded content; the house ad slot advertises a fictional product. All names and copy in this document are fictional and free to use.
