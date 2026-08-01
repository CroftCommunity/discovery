# Prototype-Ready Specification: A Clean-Room Professional Networking Platform

author: Research agent (claude.ai, commissioned; 2026-07)

date: 2026-07

status: filed content-faithful (cleaned-paste; PLAYBOOK §4) — competitive/industry research

`Commissioned clean-room UX/architecture research on LinkedIn (feed ranking, graph store, PYMK,
search, storage) that grounds the **Stellin** professional-networking behavior-scale mock. It separates
three layers explicitly: (1) what LinkedIn's published architecture does, (2) what any product could
choose, (3) what this prototype should do. It is the industry-research input to the mock build prompt
(`thinking/app/build-specs/stellin-meridian-build-prompt.md`) and the methodology
(`thinking/behavior-scale/`). Related: `research/stellin-name-clearance-2026-07.md`;
`seeds/stellin-unpacked/` (the atproto AppView scaled-sibling spikes). Raw session:
`seeds/transcripts/raw/stellin-graze-behavior-scale-sessions-2026-07.md`. Provenance caveat: pixel
metrics and third-party limit numbers are labeled as reconstructed/secondary in the Caveats.`

---

## TL;DR

- This document extends the existing blueprint into a build-ready specification: screen-by-screen UI specs (including onboarding, notifications, settings, company pages, search, and invitation manager), a full relational data model, prototype-fidelity algorithms, step-by-step workflows, trust/safety rules, a concrete tech stack with an MVP cut, and design tokens. Everything is a clean-room structural blueprint with no proprietary text or assets.

- Where LinkedIn has published primary sources, this spec separates three layers explicitly: (1) what LinkedIn's published architecture does (for example a two-pass feed ranker, the LIquid graph database, PYMK multi-stage ranking), (2) what any product could choose, and (3) what this prototype should do (Postgres with adjacency tables, a two-stage ranker, WebSocket messaging with polling fallback).

- The recommended MVP is Postgres plus a Next.js/React front end with Tailwind and a headless component kit, full-text search via Postgres or Typesense, WebSocket messaging with a polling fallback, and object storage for media. Graph traversal (degrees of separation, People You May Know) runs as adjacency-table queries with cached second-degree sets rather than a distributed graph store.

## Key Findings

### Layering of sources

LinkedIn publishes enough primary material to ground an architecture, but almost none of its exact UI pixel metrics are officially documented. The two reliably measured layout numbers are the roughly 1128px content container and the roughly 552px feed image width. Rail widths, card radius, and padding are conventions, not observed LinkedIn facts, and are labeled as recommendations below.

### Feed

LinkedIn's feed uses a two-pass architecture. Per the LinkedIn engineering blog "Community-focused Feed optimization," first pass rankers (FPR) create a preliminary candidate selection from their inventories based on predicted relevance, and a second pass ranker (SPR), also called the feed mixer, combines and scores the output into a single personalized ranked list. Ranking is multi-objective: the "Homepage feed multi-task learning using TensorFlow" post describes a utility function combining passive consumption (clicks, dwell time), active contribution (comments, reshares), and other objectives such as creator-side feedback. The "Spreading the Love in the LinkedIn Feed with Creator-Side Optimization" post (Oct 2018) reports the verbatim result that "the overall effect of the model was to take about 8% of all feedback away from the top 0.1% of creators and redistribute it to the bottom 98%," and adds that this produced a 5% increase in creators returning to post again.

LinkedIn's serving system FollowFeed uses fan-out-on-read. Per the FollowFeed engineering post, timelines are logically organized as key-value pairs keyed by a tuple of entity id and content type, with values as reverse-chronological content lists, and fan-out-on-read makes A/B testing of relevance models easier because scores are computed on the fly.

### Graph

The professional graph is stored in LIquid, a relational graph database queried with a Datalog-based language. Per InfoQ's write-up of the LinkedIn engineering material (2023), the Economic Graph "has 270 billion edges and growing, currently handling a workload of 2 million queries per second." Per "LIquid: The soul of a new graph database, Part 1," second-degree connection sets are too large to pre-materialize: the set of second-degree connections is typically at least 50,000 entities, and "the write amplification involved in keeping a pre-materialized second degree connection set up to date, roughly 250 times the base write rate, or once for each first degree connection, makes this approach impractical." This is the core reason the prototype computes second-degree relationships on demand with caching rather than storing them.

People You May Know (PYMK) is a multi-stage ranking system. Per the LinkedIn engineering page "People You May Know," PYMK was invented at LinkedIn and is responsible for building more than 50% of LinkedIn's professional graph, and per "Building a Large-Scale Recommendation System: People You May Know" it processes hundreds of terabytes of data and hundreds of billions of potential connections daily. That post describes three categories of candidate generation (graph-based, similarity-based, heuristic-based), an XGBoost light ranker, then neural network rankers estimating probability of invitation sent, invitation accepted, and downstream value, with Bayesian optimization in re-ranking. The foundational signal is triangle closing: if Alice knows Bob and Bob knows Carol, Alice may know Carol.

### Search

Search runs on Galene, built on Apache Lucene. Per the LinkedIn engineering post "Did you mean Galene," Lucene is retained as the indexing layer while other functionality sits outside it, and the architecture uses offline index building, live updates, static rank, early termination, and faceting.

### Storage

Member profiles and messages are served from Espresso, a document-oriented store built on MySQL/InnoDB with Avro schemas and Helix-managed partitions. Derived data (recommendations) is served from Venice.

## Details

### A. Screen-by-screen specifications

Global responsive strategy (recommended, based on Tailwind/Bootstrap conventions): base/mobile under 640px is a single column with a bottom tab bar; md at 768px introduces a two-column layout (center plus one rail); lg at 1024px and xl at 1280px enable the full three-column grid. Content container max-width is roughly 1128px (measured from LinkedIn banner specs and scraped markup). Recommended rail split within that container: left rail about 225px, center about 552px, right rail about 300px, with 24px gutters. The left/right rail widths are reconstructed estimates, not measured LinkedIn facts.

Every screen below specifies: component inventory, layout regions, states (empty, loading skeleton, error, permission-gated), responsive behavior, and interactions.

#### A1. Signup and onboarding (cold-start)

Component inventory: email/password form, OAuth buttons, role/intent survey, contact-import consent screen, first-connection grid, profile-completion meter, photo uploader.

LinkedIn personalizes onboarding by demographic. Per the LinkedIn engineering post "Building Dynamic Personalized Onboarding Flows for Mobile," the set and order of screens is customized based on demographic information at signup, all flows begin with an optional address-book import step, and students see different screens (company and influencer recommendations) than employed members.

Prototype flow:

1. Account creation (email + password or OAuth). State: inline validation errors; permission-gated resend-verification.

2. Basic identity: first/last name, country, and a role split (student vs employed vs job seeker), which branches the flow.

3. Position or education entry using entity typeahead (see workflows). This seeds the graph.

4. Contact import consent (skippable). Empty state if skipped routes to a manual "find people" screen.

5. First-connection grid: a suggested-people grid seeded by shared company/school. Optimistic UI on each "Connect."

6. Profile completion meter: a progress indicator showing completeness (photo, headline, one position, three skills). Recommended weights below in workflows.

States: skeleton cards during recommendation load; empty state ("We could not find matches yet, try importing contacts"); error state with retry.

#### A2. Home (feed)

Regions: top nav; left rail identity card (avatar, headline, profile-views and connection counts); center composer plus infinite feed; right rail (news, PYMK, ads).

Feed card structure top to bottom: author metadata (avatar, name, headline, timestamp, degree badge), text payload truncated with "See more," media (image, native video, or document/PDF carousel with inline viewer), social-proof counts, action bar (react, comment, repost, send).

Interactions: infinite scroll via Intersection Observer with cursor-based pagination; skeleton cards matching final layout during load; optimistic UI on reactions (icon fills instantly, reverts on failure); hover profile cards on names/avatars (a popover with mini-profile and Connect CTA after a short delay). "Back to top" affordance and preserved scroll position on back-navigation are recommended to avoid the pogo-sticking problem noted by Nielsen Norman Group.

States: empty ("Your feed is quiet, follow people or topics"); loading skeleton; error banner with retry; permission-gated ads slot hidden for logged-out preview.

#### A3. Profile

Modular stacked cards: hero (cover, avatar, name, headline, location, Connect/Message/More CTAs), About, Experience (reverse chronological), Education, Skills, Recommendations. Company/school fields are entity references, not free text.

States: own-profile edit affordances (pencil icons) vs visitor view; permission-gated sections (blurred or hidden per visibility tier); empty sections prompt the owner to add content; skeleton on load.

Interactions: inline edit modals with typeahead; "add new company" fallback (see workflows); endorsement chips on skills; degree badge and mutual-connections line near the hero for visitors.

#### A4. Network hub and invitation manager

Regions: pending inbound invitations (accept/ignore with optional message preview), sent-invitations tab with withdraw action, suggested-people grid, connections roster with search/sort/remove.

Interactions: optimistic accept/ignore; withdraw invitation (with a note that repeated withdrawal cleans the pending queue but does not itself reset rate limits); degree badges on all cards.

States: empty ("No pending invitations"); skeleton grid; error retry; rate-limit gate when the weekly invite cap is reached (see algorithms).

#### A5. Jobs

Two-pane enterprise layout: scrollable job-card list left, sticky detail pane right. Faceted search rail (remote/on-site/hybrid, salary band, seniority, date posted, company). Quick Apply pipes structured profile data. Employer console: post editor, candidate pipeline table with stages.

States: empty search result with facet-relaxation suggestion; skeleton list; error; premium-gated advanced filters (blurred with upsell).

Interactions: selecting a card updates the sticky pane without full navigation; saved-search creation with alert toggle; optimistic save/dismiss on job cards.

#### A6. Messaging

Persistent collapsible chat widget bottom-right on desktop plus a full-screen inbox. Inbox tabs: Focused (connection messages) and a separate Requests/Sponsored tab for tiered cold outreach.

Interactions: WebSocket-driven live messages with typing indicators and read receipts; optimistic send (message appears immediately, shows "sending" then confirmed or failed with retry); unread badge counts aggregated in top nav.

States: empty conversation; skeleton bubbles; offline banner when the socket drops with automatic reconnect; permission-gated compose when the user has no remaining cold-outreach credits.

#### A7. Notifications center

A grouped, reverse-chronological list with aggregation (for example "Person A and 3 others reacted to your post"). Filters by type. Unread badge in top nav.

LinkedIn routes notifications through a system called Air Traffic Controller (ATC). Per the "Air Traffic Controller" engineering post, aggregation rules can be defined via business logic, relevance scores, and member settings, and channel selection (email, SMS, in-app, push) is chosen per relevance models and member settings.

States: empty; skeleton rows; error; permission-gated push-permission prompt.

#### A8. Settings and privacy

Sections: account, visibility, communications, data/privacy, blocking. Visibility subsection controls profile-viewing mode, who-can-see-connections, and activity broadcasts (see trust/safety).

#### A9. Company pages

Regions: hero (logo, name, industry, follower count, Follow CTA), About, Posts, Jobs, People. Admin view adds post composer and analytics. Company is a first-class entity powering typeahead and the graph.

#### A10. Search results

Vertical tabs (People, Jobs, Companies, Posts, Groups) with a faceted rail per vertical. Per the Galene slides, LinkedIn supports discoverable facets (for example current company), static facets (for example network degree), and supplied facets (for example my groups), with approximate facet counts via heuristics. Blurred 3rd-degree names are a permission-gated state; advanced filters are premium-gated.

### B. Data model

The prototype uses a relational schema in Postgres. Below, "adjacency list" means storing each graph edge as a row (a from-node, a to-node) so neighbors are found by indexed lookups. Standardized entities (Company, School, Skill) are their own tables with unique ids, and profile rows reference them by foreign key; this is what powers typeahead (search the entity table by name prefix) and the graph (join through shared entity ids).

Core entities and key fields:

- **User/Member**: id, email, password_hash, first_name, last_name, headline, location, industry_id, profile_photo_url, cover_photo_url, visibility_mode, created_at.

- **ProfileSection**: polymorphic pattern; concrete tables below.

- **Company**: id, name, logo_url, industry_id, size_band, description. Standardized entity.

- **School**: id, name, logo_url. Standardized entity.

- **Position**: id, user_id, company_id (FK), title, start_date, end_date (nullable), description.

- **EducationRecord**: id, user_id, school_id (FK), degree, field, start_year, end_year.

- **Skill**: id, name (unique). Standardized entity.

- **UserSkill**: user_id, skill_id, endorsement_count (denormalized cache).

- **Endorsement**: id, endorser_id, endorsee_id, skill_id, created_at.

- **Recommendation**: id, author_id, recipient_id, relationship, text, status (pending/visible), created_at.

- **Connection edge**: user_a_id, user_b_id, status (pending/accepted), created_at. Stored as a canonical ordered pair with a status. Reciprocal.

- **Follow edge**: follower_id, followee_id, created_at. Asymmetric.

- **Invitation**: id, sender_id, recipient_id, note (nullable), status (pending/accepted/ignored/withdrawn), created_at.

- **Post**: id, author_id, text, visibility, created_at, edited_at.

- **Comment**: id, post_id, author_id, parent_comment_id (nullable), text, created_at.

- **Reaction**: id, target_type (post/comment), target_id, user_id, type, created_at.

- **Share/Repost**: id, original_post_id, actor_id, quote_text (nullable), created_at.

- **Hashtag**: id, tag (unique). **PostHashtag**: post_id, hashtag_id.

- **Media/Document**: id, post_id, kind (image/video/document), storage_url, mime_type, page_count (for documents).

- **Job**: id, company_id, poster_id, title, description, location, work_mode, salary_min, salary_max, seniority, status, created_at.

- **JobApplication**: id, job_id, applicant_id, profile_snapshot_json, status (submitted/reviewed/etc.), created_at.

- **SavedSearch**: id, user_id, vertical, query_json, alert_frequency.

- **Conversation**: id, type (direct/group), created_at. **ConversationParticipant**: conversation_id, user_id.

- **Message**: id, conversation_id, sender_id, body, is_cold_outreach, created_at, delivered_at, read_at.

- **Notification**: id, recipient_id, type, actor_id, target_type, target_id, aggregation_key, is_read, created_at.

- **PremiumEntitlement**: id, user_id, tier, cold_outreach_credits_remaining, credits_reset_at, features_json.

How this maps to LinkedIn per primary sources vs the prototype simplification:

- LinkedIn stores the graph in LIquid (a distributed relational graph database with a Datalog query language, per the LIquid engineering posts). The prototype uses Postgres adjacency tables plus cached second-degree sets. This is a deliberate simplification; the LIquid posts explain second-degree sets are not pre-materialized at LinkedIn scale because write amplification is roughly 250x, but at prototype scale materialize-and-cache is fine.

- LinkedIn serves profiles and messages from Espresso (document store on MySQL/InnoDB with Avro and Helix, per "Introducing Espresso"). The prototype uses plain Postgres rows.

- LinkedIn serves recommendations from Venice (derived data platform, per "Open Sourcing Venice"). The prototype computes recommendations in a scheduled job and caches them in a table.

### C. Key algorithms at prototype fidelity

#### Feed ranking

What LinkedIn does (primary sources): a two-pass architecture with first pass rankers producing candidates and a second pass ranker/feed mixer combining them into one list, using multi-objective optimization over passive consumption, active contribution, and creator-side objectives.

Prototype version: Stage 1 candidate generation pulls recent posts from first-degree connections and followed entities via an indexed query on the Post table joined to Connection/Follow edges (fan-out-on-read). Stage 2 scoring computes a simple weighted score per candidate: score = w1 * predicted_reaction + w2 * predicted_comment + w3 * recency_decay + w4 * affinity, where affinity is a count of prior interactions with the author and recency_decay is an exponential function of post age. Start with hand-tuned weights, sort descending, paginate by cursor.

#### People You May Know

What LinkedIn does: multi-stage ranking (graph, similarity, heuristic candidate generation; XGBoost light ranker; neural rankers; Bayesian re-ranking), founded on triangle closing, and responsible for building more than 50% of the professional graph.

Prototype version: for user U, query second-degree nodes (friends of friends) via a self-join on the Connection table, count shared connections per candidate, add boosts for same company/school/industry, exclude existing connections and pending invitations, rank by the combined score, cache the top N in a recommendations table refreshed nightly.

#### Degree-of-separation

Definition: BFS (breadth-first search) explores a graph level by level from a start node, so the first level is 1st-degree, the second is 2nd-degree, and so on.

Prototype version: compute degree on demand by BFS over the Connection adjacency table, capped at depth 3 (beyond that, label "3rd+"). Cache each viewer's first-degree set in memory/Redis; compute second-degree as the union of neighbors' neighbors; a target is 2nd-degree if it appears there, else run a shallow bidirectional BFS to detect 3rd-degree. Cache results per (viewer, target) with a short TTL. This mirrors the LIquid rationale: materialize the cheap first degree, compute deeper degrees on demand (LinkedIn notes second-degree sets are typically at least 50,000 entities, which is why pre-materialization is impractical at scale).

#### Typeahead and faceted search

Definition: typeahead returns ranked suggestions as the user types a prefix; faceted search filters results by structured attributes (facets) with counts.

Prototype version: index People, Companies, Schools, Jobs, Posts. Use Postgres full-text plus trigram indexes for the MVP, or Typesense for prefix/typo-tolerant typeahead. Facets are computed as grouped counts on the filtered result set. Personalize people-typeahead by boosting closer degrees, mirroring LinkedIn's personalization.

#### Job-candidate matching

What LinkedIn does: JYMBII personalizes job recommendations from profile and activity; per "Improving job matching," it builds activity embeddings, and candidate selection uses query clauses matching profile title/skills/location to job fields.

Prototype version: represent each job and each user as bags of tokens (title, skills, location, seniority). Score matches by weighted token overlap plus location and seniority match. Rank and cache. This is the interpretable analog of LinkedIn's embedding approach.

#### Notification aggregation and batching

What LinkedIn does: ATC aggregates by business logic, relevance, and member settings, and selects channels. A batching window (for example email digests) consolidates alerts.

Prototype version: give each notification an aggregation_key (for example "reactions:post:123"). On write, upsert into the key; the UI renders "Actor and N others." A background job flushes email digests on a fixed window.

#### Connection-request rate limiting

What LinkedIn does per third-party analysis (labeled as corroboration, not primary): a weekly invitation cap. Per Evaboot (2026), the limit was previously set at 700 invites per week but was reduced to combat spam; basic accounts now sit at roughly 100 per week, and older accounts with a high Social Selling Index may reach up to about 200 per week. The cap is dynamic based on acceptance rate and pending backlog. Also per Evaboot (2026), withdrawing pending invites "does NOT reset or free up your weekly limit," though it does clean the acceptance signal LinkedIn reads, and LinkedIn starts flagging accounts that let large numbers of invitations (roughly 700 or more) go unanswered.

Prototype version: a fixed rolling 7-day counter per user with a configurable cap (default 100). Show a soft warning near the cap and a hard block at it. Track acceptance ratio and reduce the cap if it drops below a threshold.

### D. Core workflows

#### Create a post with @mentions, hashtags, and document upload

1. User opens composer. 2. As they type "@", a typeahead queries connections/companies; selecting inserts a mention token bound to an entity id. 3. "#" tokens create or link Hashtag rows. 4. Document upload streams to object storage, returns a URL, creates a Media row with page_count; the client renders an inline paged viewer. 5. On submit, create the Post, PostHashtag, and Media rows in a transaction; optimistic UI shows the post immediately. Edge cases: upload failure (retry, keep draft); mention of a non-connection (allowed but no special notification privilege); oversized file (reject with message).

#### Send and accept invitations with optional notes

1. Sender clicks Connect, optionally adds a note (note length limited; free-tier note availability can be gated). 2. Rate-limit check; block if over cap. 3. Create Invitation (pending); optimistic UI flips button to Pending. 4. Recipient sees it in the invitation manager, accepts (create reciprocal Connection, mark accepted) or ignores. Edge cases: duplicate invitation (dedupe); recipient already 1st-degree (no-op); sender withdraws (status withdrawn, does not free the weekly window).

#### Messaging including credit-limited cold outreach

1. Standard messages between connections are unlimited and land in Focused. 2. Cold outreach to a non-connection requires a cold-outreach credit; decrement PremiumEntitlement.cold_outreach_credits_remaining. 3. The message lands in the recipient's Requests tab. 4. If the recipient replies, refund the credit (mirroring LinkedIn's reply-refund behavior, per third-party sources; LinkedIn's own tiers reportedly grant on the order of 5 credits per month on entry-level premium, indicative only). Edge cases: zero credits (compose gated with upsell); recipient has open-profile flag (no credit charged).

#### Job posting and application

Employer: create Job (draft), publish, view candidate pipeline table. Seeker: Quick Apply snapshots profile fields into JobApplication.profile_snapshot_json so later profile edits do not alter the submitted application. Edge cases: closed job (apply disabled); duplicate application (blocked); required custom questions (multi-step modal).

#### Profile editing with entity typeahead and add-new fallback

1. User edits Experience, types a company name. 2. Typeahead queries the Company table. 3. If found, bind company_id. 4. If not found, an "add new company" fallback creates a lightweight Company row (name only, unverified flag) and binds it, preserving the standardized-entity model while allowing growth. Same pattern for School and Skill.

#### Endorsements and recommendations

Endorsement: one-click on a connection's skill, creates an Endorsement row and increments the cached count; optimistic UI. Recommendation: author writes text, recipient must approve before it becomes visible (status pending to visible). Edge cases: self-endorsement blocked; endorsement removal decrements count; recommendation edit re-enters pending.

### E. Trust, safety, and privacy

Profile visibility tiers: public, connections-only, and private per section. Profile-viewing modes mirror LinkedIn's documented three modes. Per the LinkedIn Help page "Who's viewed your profile visibility settings," the modes are your name and headline (full), private profile characteristics (semi-private, showing job title/company/industry only, with examples like "VP of Marketing in Internet Industry"), and private mode (shown as an anonymous member); a Basic account that chooses private mode loses its own viewer history. No premium tier can unmask a member who viewed in private mode (well-corroborated across sources).

Who-can-see-my-connections: per the LinkedIn Help page "Who can see your connections," the default lets 1st-degree connections browse your list, and a toggle restricts it to only you; mutual connections remain visible regardless.

Block/report: blocking removes the mutual connection and hides both profiles from each other; per LinkedIn Help, once a member is blocked they can no longer view your profile. Report opens a review flow.

Invitation withdrawal: supported from the sent tab.

Anonymous vs identified viewing: the viewer's chosen mode governs what the viewed member sees.

### F. Tech stack recommendation

Front end: Next.js/React with Tailwind and a headless component kit (shadcn-style). Tradeoff: Next.js gives SSR for SEO on public profiles and company pages; a headless kit keeps the dense enterprise patterns (split panes, data tables, multi-step modals) fully controllable.

Database: Postgres with adjacency tables for edges. Tradeoff vs a graph database: a native graph store makes deep traversals cheaper, but adds operational complexity the prototype does not need. Postgres handles 1st/2nd-degree with indexed self-joins and caching; this is the honest prototype tradeoff against LinkedIn's LIquid.

Realtime messaging: WebSocket transport with a polling fallback. Tradeoff: WebSockets give low-latency bidirectional chat but need reconnection logic; long polling is simpler and universal but higher overhead. Recommendation: WebSocket first, polling fallback for restrictive networks.

Search: Postgres full-text for the MVP; migrate to Typesense or Meilisearch for typo-tolerant typeahead as the corpus grows; Elasticsearch only if analytics-scale search is needed. Tradeoff: Postgres FTS avoids a new service; Typesense gives better prefix/typo UX with modest ops.

Media storage: object storage (S3-compatible) with signed URLs and a CDN.

Auth: session or JWT via a managed auth library; OAuth for social login and contact import.

MVP cut (ship first): auth/onboarding with entity typeahead, profile, connections with invitations and degree badges, a basic ranked feed with posts/reactions/comments, and 1:1 messaging. Phase 2: jobs board and Quick Apply, company pages, PYMK, notifications center with aggregation, search verticals with facets, premium entitlements and cold-outreach credits, endorsements/recommendations, document carousel viewer.

### G. Design tokens and layout metrics

Measured (from credible teardowns/specs): content container about 1128px (corroborated by LinkedIn's own 1128 x 191px company banner spec and scraped markup showing a 1128px top-card container); feed image render width about 552px, which sets the center column at roughly 552 to 555px.

Recommended (conventions, not observed LinkedIn facts):

- Breakpoints (Tailwind defaults): sm 640, md 768, lg 1024, xl 1280, 2xl 1536. Bootstrap 5's xl container max-width is 1140px, very close to the measured 1128px.

- Rails within the 1128px container: left about 225px, center about 552px, right about 300px, gutters 24px. Reconstructed estimate, verify in DevTools if precision matters.

- Card padding 16px (Material Design convention: "padding should be set to 16px"), border-radius 8px, inter-card gap 16 to 24px, subtle 1px border or low-elevation shadow on white.

- Spacing on an 8-point grid (4/8/12/16/24px).

- Typography base 16px (1rem), modular scale ratio about 1.25 (Major Third): 16, 20, 25, 31, 39, 49px; small/caption about 13 to 14px; body line-height about 1.4 to 1.6.

## Recommendations

1. Build the MVP cut first (auth/onboarding, profile, connections/invitations with degree badges, basic feed, 1:1 messaging) on Postgres + Next.js. Benchmark to move on: feed queries under roughly 200ms at seed data volume and correct 1st/2nd-degree computation.

2. Keep the graph in Postgres adjacency tables with cached first-degree sets and nightly second-degree materialization. Threshold to reconsider a graph database: when second-degree queries exceed acceptable latency at your data size, or write amplification from caching becomes costly (LinkedIn's roughly 250x figure, driven by second-degree sets of at least 50,000 entities, is the warning sign at scale).

3. Start search on Postgres full-text; adopt Typesense when typeahead needs typo tolerance or the corpus makes FTS slow.

4. Use WebSocket messaging with a polling fallback; add read receipts and typing indicators only after basic send/receive is stable.

5. Implement rate limiting and privacy tiers early, since they shape data access patterns and are hard to retrofit. Default the invite cap to 100 per rolling 7 days and make it configurable.

6. Treat all pixel metrics except the roughly 1128px container and roughly 552px feed width as recommendations; verify rail widths against a live inspection before finalizing the grid.

## Caveats

- Exact LinkedIn UI metrics (rail widths, card radius, padding) are not officially published; values here are conventions or reconstructed estimates and are labeled as such.

- Connection-request and cold-outreach limits come from third-party analysis (for example Evaboot 2026), not LinkedIn primary sources; treat specific numbers as indicative and make them configurable. Could not find a LinkedIn primary source stating exact invite or InMail-style credit caps.

- Some architectural details (for example the Quasar ranking engine, or greedy set cover in PYMK serving) appear in secondary summaries; the prototype does not depend on them.

- LinkedIn's systems operate at a scale the prototype does not target; every "recommended simplification" is a deliberate tradeoff, not a claim that LinkedIn works this way.
