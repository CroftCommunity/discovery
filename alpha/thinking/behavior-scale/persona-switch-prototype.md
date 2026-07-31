# The Persona-Switch Prototype

> **Provenance.** Reusable methodology authored in a claude.ai design dialogue (2026-07), filed
> **content-faithful** (cleaned-paste — not a byte-pristine export; see PLAYBOOK §4). Raw session
> transcript: `seeds/transcripts/raw/stellin-graze-behavior-scale-sessions-2026-07.md`. This is the
> **prototype-layer** method (the mock tier of behavior-scale); the full-lifecycle discipline lives in
> the companion `behavior-scale-methodology.md`. Index + registry: `README.md`.

---

A method for building a local-only, fully interactive UI/UX mockup of a multi-user web2 site (social network, marketplace, forum, collaboration tool) as a static SPA/PWA on GitHub Pages, with no backend, no auth, and no persistence beyond the browser.

The point is hands-on testing of design and workflows across user roles before any server exists. You switch between mock identities from a dropdown, the site behaves fully from each seat, and two buttons move you between an empty world, a lived-in world, and whatever you add by hand.

## 1. What this is and when to use it

Definition: a persona-switch prototype is a complete front end where identity is a client-side selection instead of authentication, all data lives in browser storage, and every interaction runs through the same code paths a production app would, minus the network.

Use it when:

- The product is inherently multi-user and the interesting UX lives *between* accounts (invites, messages, listings, moderation, notifications), which clickable design-tool prototypes cannot express.

- You want to iterate on real interaction mechanics (optimistic updates, empty states, gating, ranking) before committing to a backend contract.

- You need something stakeholders can open from a URL on any device with zero setup.

Do not use it to validate: real latency variance, concurrency and conflict resolution, security, server-side validation, scale, or multi-device sync. Section 18 covers the limits honestly.

## 2. The five load-bearing ideas

Everything else in this document hangs off these.

1. **Persona switcher, not auth.** A dev-only dropdown lists mock users. Selecting one re-derives everything identity-dependent: permissions, badges, feeds, gated states. The default entry is Logged out, so public views are first-class.

2. **Three data states.** *Delete All* resets to accounts-only (users exist, nothing else): this is where you test onboarding and empty states. *Seed* replays a scripted history so the site looks used and the personas have interacted. Anything you add live layers on top of either. All three must be reachable in one click.

3. **Event log as source of truth.** Instead of storing pre-built state, store an append-only list of events (post created, invite accepted, message sent) and derive state by reducing over them. A reducer is a pure function `(state, event) -> state`. Seeding is just replaying a scripted event log through the same reducers the live UI uses, which guarantees seeded data and hand-added data behave identically, and makes counts, feeds, and badges derive automatically instead of being hand-maintained.

4. **Frontier markers.** Any real UI affordance whose path is intentionally unbuilt gets a visible, clickable marker (a dashed chip naming the gap) registered in one file, with a screen that lists them all. This gives the team shared vocabulary for what is deferred and bans dead buttons.

5. **Simulated network conditions.** Everything local is synchronous, so skeletons and optimistic UI never render unless you fake them. A dev-bar latency toggle (0/250/600 ms) wraps every action, and a one-shot "fail next action" checkbox exercises error states and optimistic rollback.

## 3. Hosting constraints: GitHub Pages

- Pages serves static files over https. A service worker (the script that lets a site cache itself and install as an app) only registers in a secure context, meaning https or localhost, so "self-contained" means serving the folder, never double-clicking index.html.

- A project site serves under a subpath (`https://user.github.io/repo/`). Consequences: every URL in the app must be relative (`./css/...`, `./js/...`), the web app manifest uses `start_url: "./"` and `scope: "./"`, and routing must not assume the site root.

- Use hash routing: routes live after `#` (`#/feed`, `#/item/42`), so the server never sees them and needs zero configuration. History-API routing on Pages requires a 404-redirect hack; hash routing avoids it entirely. Still ship a `404.html` that meta-refreshes to `./#/` as a safety net.

- Local development mirror: any static server (`python -m http.server`, `npx serve`) from the repo root.

## 4. Stack decision

Default: **no build step**. Plain HTML5, modern CSS, vanilla JS as native ES modules, zero runtime dependencies (no CDN scripts, no npm packages, no webfonts). Rendering via template literals or small hand-rolled components.

Why this default: deploys by pushing the folder, nothing to install, nothing to break in a bundler, and the browser debugger shows exactly the code you wrote. The rule is "as simple as possible, but no simpler."

When to relax: if the team already lives in a framework and re-training costs more than tooling, a minimal Vite setup with a small library is acceptable. Relaxing costs you the push-to-deploy simplicity and adds a build artifact to keep honest. Decide once, up front.

If a helper is genuinely needed (a tiny store, a router), write it in-repo. They are each under a hundred lines.

## 5. Repository shape

Shape it like a real site from day one, because you will iterate on it as one.

```
/index.html
/manifest.webmanifest
/sw.js
/404.html
/css/
  tokens.css        design tokens as custom properties
  base.css          reset, typography, layout primitives
  components.css    cards, buttons, chips, modals, toasts, skeletons
  screens.css       per-screen rules
/js/
  app.js            bootstrap: store, router, dev bar
  router.js         hash router, scroll restoration, focus management
  store.js          event log, reducers, selectors, persistence
  actions.js        every user-visible mutation dispatches through here
  latency.js        fake-async wrapper honoring dev-bar settings
  engines/          pure functions: ranking, recommendations, search, limits
  ui/               shared components
  screens/          one module per route
  devbar.js
  frontier.js       registry of frontier markers
/data/
  seed.js           personas, entities, and the scripted event log
/assets/icons/
```

The seam that matters most: **actions.js**. Every mutation in the app calls an action, actions dispatch events to the store, and nothing else writes state. This is also your backend migration path (section 18).

## 6. Data architecture in detail

### Event shape

```js
{ id, t,        // timestamp; seed events store offsets, resolved at replay
  actor,        // userId or "system"
  type,         // "post.created", "invite.accepted", "message.sent", ...
  payload }     // type-specific
```

Enumerate the event vocabulary early: account lifecycle, profile edits, relationship changes (connect/follow/join/block), content (create/comment/react/share), transactions or applications if your domain has them, messaging, settings, notification reads.

### Reducers and selectors

State = `reduce(events)`. Selectors are pure read functions over that state (feed for viewer X, unread count for X, visible profile of Y as seen by X).

Put **policy in selectors**, not in components: blocking, visibility tiers, and gating are enforced where data is read, so a blocked pair is invisible to each other in *every* surface (feed, search, suggestions, comments) without per-screen code. This is a rendering-layer guarantee; test it from both seats.

### Timestamps

Seed events store offsets from seed time ("-3d 14:20"), resolved to absolute timestamps at replay. This keeps the seeded world reading fresh ("3d ago") no matter when you press Seed, and makes replay deterministic.

### Persistence

- One storage adapter module wrapping localStorage, keyed with a schema version (`app.v1`). On version mismatch: wipe and `console.warn`. The adapter makes swapping to sessionStorage a one-line change if you want strict per-session scope.

- Serialize `{schemaVersion, events, devPrefs}` on every dispatch, debounced ~250 ms.

- Export downloads that JSON; Import validates the version, replaces, and re-reduces. Export/Import is how you share exact repro states with teammates.

### Storage budget and media

localStorage quotas are browser-dependent, on the order of 5 MB per origin, so treat media as the budget risk:

- Avatars: generated inline SVGs (initials on deterministic per-user colors), never image files.

- Live image uploads: downscale via canvas (max ~1024 px), compress to JPEG, store as data URLs, and reject anything still large with a toast.

- Rich media in seed data (document pages, gallery slides): author as styled SVG or HTML, not embedded binaries.

## 7. Designing the persona roster

Design the roster as **coverage, not casting**. Method:

1. List every identity-dependent state your UX must exhibit. Typical axes: role (buyer/seller, member/moderator, member/admin, creator/consumer), tier (free/premium, with the paid features tested from both the have and have-not side), lifecycle (brand-new, established, dormant), relationship states (connected, pending both directions, withdrawn, blocked both ways, out of network), and limit states (at a rate cap, out of credits).

2. Assign each state to a seat so every one is reachable from at least one dropdown selection. Six to ten personas usually covers a web2 product; more means seats without a purpose.

3. **Wire the relationship graph deliberately.** Draw the edges so that from your default seat, every relationship distance and state is visible at once (someone close, someone one hop away, someone far, someone invisible). The graph, not the individual bios, is what makes switching seats informative.

4. Script **in-flight states** into the seed: a pending request the default seat can accept, an unread message in a secondary inbox, an active block, an unread aggregated notification, a seat sitting at a limit. These are the moments most UX bugs live in.

5. Keep **one pristine persona** who receives no seed activity ever, so onboarding and empty states stay testable without a full reset.

6. Make **Logged out** the dropdown default, and make signup a real flow that creates a genuinely new persona and adds it to the dropdown, so onboarding is repeatable without consuming the pristine seat.

## 8. Authoring the seed

Write the seed as a chronological scripted event log, roughly 40 to 60 events for a first version, authored to exercise the UI on purpose:

- Content that stresses rendering: one long post that triggers truncation, one of each media type, one item with heavy engagement and one with zero, at least one nested comment thread, one repost-with-comment.

- Cross-persona interaction: comments and reactions flowing between seats so feeds, notifications, and profiles all show relationships, not monologues.

- Entity variety for search and filters: enough companies/categories/tags/listings that no results page or facet rail lands empty.

- Order matters: put pending things (an unaccepted request, an unread message) near the end of the log so they are still pending after replay.

- Seeding must be deterministic: same log, same world, every time. Seed wipes before replaying.

## 9. The dev bar

A visually distinct scaffold strip above the real navigation (dashed or striped background so it cannot be mistaken for product UI):

- Persona dropdown, defaulting to Logged out.

- Seed and Delete All (with confirm).

- Export and Import state.

- Latency toggle and one-shot Fail Next Action.

- Frontier markers show/hide.

- Unregister Service Worker + hard reload (see section 13).

Persona switch must re-derive every identity-dependent cache in one place, not scattered per screen.

## 10. Client-side engines

The "smart" behaviors of a web2 site are implementable as small pure functions over derived state. Keep each in its own module under `/js/engines/` so a real backend can replace them one at a time later:

- **Ranking** (feed, listings, results): candidate generation by simple rules (recency window, relationship), then a hand-tuned weighted score (affinity, engagement with log damping, exponential recency decay). Interpretable beats clever at this stage.

- **Recommendations** (people/items you may like): neighbor-of-neighbor traversal plus attribute boosts, with exclusions (already related, pending, blocked, self).

- **Relationship or permission computation**: breadth-first search (level-by-level graph traversal) capped at a small depth, with the viewer's near sets cached on persona switch and invalidated on relationship events.

- **Search**: an in-memory prefix index over the entities, rebuilt on relevant events; facets are grouped counts over the filtered set.

- **Notification aggregation**: give each notification an aggregation key per (type, target) and fold actors into "X and N others" in the reducer; derive unread counts by viewer.

- **Limits**: rolling-window counters (invites, posts, credits) with a soft warning threshold and a hard block, surfaced in the UI.

## 11. Screen and state discipline

Every screen implements four states, and the checklist (section 17) verifies them:

- **Loading skeleton**, shaped like the final layout, visible under the latency toggle.

- **Empty**, written as an invitation to act, not a mood.

- **Error**, triggered via Fail Next, explaining what happened and offering retry.

- **Gated**, wherever tier, privacy, or relationship restricts access (blur, lock, upsell), tested from both the granted and denied seat.

## 12. Fidelity mechanics worth building

These are what make a mockup feel like a product:

- Optimistic UI: mutations render instantly, then confirm or roll back with a toast (a rollback is only observable with Fail Next on, which is why the dev bar has it).

- Infinite scroll via IntersectionObserver (a browser API that fires when a sentinel element scrolls into view) with cursor pagination over the ranked list.

- Unread badge plumbing derived from selectors, never hand-incremented.

- Hover cards on names (desktop, short delay); plain navigation on touch.

- A single toast system in an `aria-live` region; a single modal system with focus trapping.

- Route changes move focus to the new screen's `h1` and set `document.title`.

## 13. The PWA layer, shipped last

- Manifest: name, `display: "standalone"`, relative `start_url` and `scope`, icons (192/512 plus a maskable variant) generated from one SVG glyph, theme colors from tokens.

- Service worker: precache an explicit app-shell list, cache-first with a versioned cache name, delete old caches on activate, bump the version every deploy.

- Ship it in the **final milestone**. A stale service-worker cache fighting you mid-development is the classic time sink of this pattern. Escape hatches regardless: a `?nosw` query param that skips registration, and the dev-bar unregister button.

- Standalone quirks: `viewport-fit=cover` and `env(safe-area-inset-*)` padding on any bottom bar.

## 14. Mobile and accessibility, v1 not passes

- Breakpoints: single column with a bottom tab bar under ~640 px; add a rail through tablet; full multi-column desktop at ~1024 px inside a fixed-width container.

- Primary creation action becomes a full-screen sheet on mobile; touch targets at least 44 px; long-press (~500 ms) replaces hover interactions.

- Keyboard operability for every picker, menu, and modal; visible focus ring from tokens; AA text contrast; `prefers-reduced-motion` disables shimmer and pop animations.

## 15. Visual direction and tokens

- All tokens as CSS custom properties in one file: palette (4 to 6 named values plus a neutral ramp), spacing on an 8-point scale, radius, borders, and a type scale with roles (a characterful display face used with restraint, a body face, a utility size). With no webfonts allowed, system stacks still give you a voice: `system-ui` for body, a system serif stack for display is one honest option.

- Clean-room rule: if the mockup is "a site like X," it must read as the *genre*, not the brand. No cloned logos, copy, or exact brand colors.

- Pick **one signature element** that encodes the product's core idea (in a networking product, the relationship-distance badge; in a marketplace, the trust/verification mark) and use it identically everywhere. Spend your distinctiveness there and keep everything else quiet.

- Copy is design material: active voice, sentence case, buttons say what happens and keep the same name through a flow, errors say what went wrong and how to fix it, empty states direct the next action.

## 16. Build order methodology

Sequence so every milestone ends runnable, and commit per milestone:

1. Scaffold: tokens, router, store with persistence, dev-bar shell, Delete All state.

2. Seed log plus Seed/Export/Import, latency and fail-next.

3. The core content loop (create, view, react, comment).

4. Identity surfaces (profiles, editing, completion).

5. Relationships (requests, acceptance, distance computation, recommendations, limits).

6. Messaging or transactions, whichever your domain centers.

7. Notifications and unread plumbing.

8. Search and facets, with gating.

9. Settings, privacy, blocks.

10. Public/logged-out views and the real signup flow.

11. Frontier registry and screen.

12. PWA layer, then mobile polish, then the accessibility pass and the acceptance run.

## 17. Acceptance checklist methodology

Write the checklist **before building**, as concrete seat-level assertions the builder (human or agent) can self-verify. Good items name a seat, an action, and an observable result:

- "Fresh load with cleared storage lands Logged out on the public view with a Join CTA."

- "After Seed, seat A sees B's pending request; accepting it produces an acceptance notification in B's seat and updates the relationship badge everywhere."

- "Seats C and D (blocked pair) are invisible to each other in feed, search, suggestions, and comments, verified from both seats."

- "With latency 600 and Fail Next armed, a reaction fills optimistically then reverts with an error toast."

- "Export, Delete All, Import restores state exactly, spot-checking unread counts."

- "Under 640 px, bottom tabs and long-press interactions work; keyboard-only can complete the core loop; focus lands on the h1 per route."

- "Deployed on Pages: installable, offline reload serves the shell, `?nosw` bypasses."

Fifteen to twenty such items is enough to keep an agent-built or team-built prototype honest.

## 18. Limits, and the migration path to a real backend

What this pattern cannot validate: real network variance beyond the fake-latency toggle, concurrent multi-user editing and conflicts, security and abuse handling, server-side validation, data at scale, cross-device sync, and real third-party integrations.

What it sets up well: the **actions.js seam**. Because every mutation already flows through one module as a named event with a payload, migrating means reimplementing actions as API calls, keeping the same event vocabulary as your API's write operations, and moving reducers server-side (or keeping them client-side over server-sent events). Selectors become your read-API contract. The engines migrate one at a time from client heuristics to server services. The persona dropdown becomes real auth, and the seed log becomes your integration-test fixture.

## 19. Pitfalls, collected

- Service worker cache during development: ship it last, keep `?nosw` and the unregister button.

- localStorage bloat from base64 images: downscale, compress, cap, and prefer SVG.

- Pre-built state instead of an event log: seeded data drifts from live behavior and counts go stale; the log is the whole trick.

- Absolute paths anywhere: breaks under the Pages subpath.

- Policy in components instead of selectors: blocks and gating leak on surfaces you forgot.

- Non-deterministic seeds (random IDs, `Date.now()` inside seed content): breaks repro; resolve all randomness and time at replay from fixed inputs.

- Dead buttons for unbuilt paths: register a frontier marker the moment a path is deferred.

- Skipping the pristine persona: after one Seed, you can never see first-run states again without a reset.

- Treating mobile and accessibility as final passes: retrofitting focus management and touch layouts costs more than building them in.
