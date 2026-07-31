# Raw sessions: the behavior-scale methodology + the Stellin & Graze builds (2026-07)

> **Preservation status: `preserved-condensed (cleaned-paste)`.** Content-faithful, **not a
> byte-pristine export** (PLAYBOOK §4). Source: claude.ai design dialogues the user pasted into a
> Claude Code session on 2026-07-30. **Naming:** the dialogue named the methodology "the behavioral
> twin" and produced a document titled "The Behavioral Twin"; it was **renamed *behavior-scale* (user,
> 2026-07-31)** because "twin" implied exactly two deployments. This synthesis uses the current name;
> the one preserved historical mention of the original title is marked as such.
> UI render chrome has been stripped (collapsed "thinking"
> summary headers, "Document · MD", "Show more", "All sources gathered", "Research complete · N
> sources", date/time dividers). The large deliverables produced in these sessions are filed
> **verbatim in their working homes** and are **referenced here, not re-embedded** (to avoid
> triplication per §4):
>
> - Methodology → `thinking/behavior-scale/behavior-scale-methodology.md`,
>   `thinking/behavior-scale/persona-switch-prototype.md`
> - Stellin mock build prompt → `thinking/app/build-specs/stellin-meridian-build-prompt.md`
> - Graze mock spec → `thinking/app/build-specs/graze-persona-switch-spec.md`
> - Graze "build spec" (topic-aggregation) → `thinking/app/build-specs/graze-topic-aggregation-build-spec.md`
> - LinkedIn research → `research/linkedin-ux-architecture-2026-07.md`
> - Reddit/Slashdot research → `research/reddit-aggregation-ux-slashdot-baseline-2026-07.md`
>
> The two original UX-seed descriptions the user pasted as inputs ("a relational database masked as a
> social feed" for LinkedIn; "a topic-driven, decentralized aggregation platform" for Reddit) were
> themselves **truncated in the paste** ("[truncated]" / "pasted" markers in the source) — they are
> preserved as **truncated pointers** below, not reconstructed. Verbatim user turns are preserved with
> `[sic]` on obvious typos. See `thinking/behavior-scale/README.md` for the synthesized index.

---

## Session A — Stellin (professional-networking mock; working name "Meridian")

### A0. Seed input (truncated in paste)

User pasted a prior research description of LinkedIn UX/UI/workflows, opening:

> "To build a professional networking platform, you must shift away from the design philosophies of
> casual social media. While platforms like Instagram or TikTok optimize for visual dopamine and
> endless discovery, a professional network is fundamentally a **relational database masked as a
> social feed** …" [truncated in source paste — the full description is the input the clean-room
> research doc extends]

### A1. User

> Can you take this description of linked in UX, UI and workflows
>
> And extend it even further in enough detail that we can look to build a prototype of a site that
> functions similarly

**Assistant** (deliverable): produced the clean-room professional-networking specification — screen
specs, relational data model, prototype-fidelity algorithms, workflows, trust/safety rules, tech
stack + MVP cut, design tokens, separating (1) LinkedIn's published architecture, (2) any product's
options, (3) the prototype's choices. → filed verbatim at `research/linkedin-ux-architecture-2026-07.md`.

### A2. The behavior-scale thread

**Assistant** (prose, on making the mock rigorous — précis of what became the methodology doc):

> Doing it right means making the mock's guarantees mechanical instead of cultural. Three artifacts
> carry all the weight: a shared contract, a conformance harness, and agent-legible law. Everything
> else is detail.
>
> The machinery: one contract, imported by both sides — the event vocabulary and payload schemas live
> in a single schema module that both the mock's reducers and the real API import; a mutation that
> isn't in the schema cannot exist on either side, which is what makes drift structurally hard. A
> formalized adapter — every function gets a named capability whose substrate (memory, api, hybrid) is
> selected in a config file, never by editing call sites. A conformance harness — replay the same
> scenario logs against both substrates and assert equivalent observable outcomes (selector outputs,
> counts, visibility), never equivalent internals; tolerances declared explicitly ("ranking order may
> differ, membership may not"). The scenario library as shared currency — named deterministic event
> logs with expected-outcome assertions, simultaneously UX repro cases, mock seeds, and backend test
> fixtures; rule: no new mutation without a scenario touching it. A divergence ledger — one file
> recording every intentional mock/prod difference with reason and tolerance; frontier chips
> (prod-only paths) and proposal markers (mock-only experiments) render from it so the UI itself tells
> the truth.
>
> Defining instructions for future agents: agents follow invariants, decision procedures, and runnable
> checklists far better than prose philosophy, so the root doc is short law, not explanation —
> invariants stated MUST/NEVER; decision procedures one per situation; self-verification that ends
> every task by running conformance + the acceptance checklist; an escalation rule that stops and
> writes an ADR rather than inventing a convention. The one failure mode to design against explicitly
> is doc drift: the agent doc must point at the schema and scenario suite as the source of truth, so
> when prose and behavior disagree, behavior wins and the prose gets fixed.

### A3. User

> And incredibly it could be published as it's own pre built local alternative for small communities
> and it would be true

**Assistant** (prose — the community-edition insight):

> It would be true, and for a precise reason: nothing in the mock is simulated. The reducers, engines,
> and policy are real software running real logic; the frontier discipline even keeps the gaps honest.
> So publishing it isn't shipping a demo dressed as a product, it's shipping the product minus a
> server. The persona dropdown stops being scaffolding and becomes a feature: on a shared device (a
> family tablet, a clubhouse kiosk, a classroom machine) it's simply a profile picker, and that mode
> works today with zero changes.
>
> The one load-bearing caveat is that "local-only" currently means one browser. A distributed
> community needs shared state, and here the architecture pays off again, because an event log is the
> natural unit of sync: union two logs, order events deterministically (timestamp, then actor, then
> id), keep reducers idempotent, and every device converges on the same state — essentially a
> grow-only set CRDT. So the adapter's routing table gains a fourth substrate value alongside memory,
> api, and hybrid: sync, whether that's export/import files passed around (already built), a shared
> folder, or a ~50-line relay. The community edition is a config value, not a fork.
>
> Two things to say out loud in its README rather than discover later. Identity is honor-system: no
> auth means anyone can select any persona, and selector-enforced blocking is client-side courtesy,
> not security — for a small high-trust community that's the historically correct model (the same one
> BBSs and family servers ran on), but it must be named. And for real daily use, storage moves from
> localStorage to IndexedDB, since 5MB stops being a budget and starts being a wall.
>
> The elegant end state is three deployments of one contract: the dev mock, the community edition, and
> the scaled platform are the same codebase with different substrate values, and the conformance
> harness you'd build anyway is the proof they behave identically.

### A4. User

> Describe that methodology, practice and discipline in enough detail for us to return to and
> implement layer[ed] and put it in a markdown document

**Assistant** (deliverable): produced **The Behavioral Twin** (the founding methodology document, now filed as *behavior-scale*) — vocabulary, the three load-bearing
artifacts, the four substrates (memory/api/hybrid/sync), eleven invariants, decision procedures, the
community-edition requirements, a seven-layer roadmap, and copy-paste appendix skeletons (root agent
doc, scenario, routing config, ledger entry, ADR). Two structural choices baked in: rule 11 makes
behavior (schema + scenarios) the source of truth over any document including the agent doc; the
roadmap is ordered so stopping after any layer still pays rent. → filed verbatim at
`thinking/behavior-scale/behavior-scale-methodology.md`. The companion prototype-layer method (**The
Persona-Switch Prototype**) → `thinking/behavior-scale/persona-switch-prototype.md`. The executed
build prompt (**Meridian** → Stellin) → `thinking/app/build-specs/stellin-meridian-build-prompt.md`.

---

## Session B — Graze (Reddit-family aggregation mock)

### B0. Seed input (truncated in paste)

User pasted a prior research description of Reddit UX/UI/workflows, opening:

> "To build a platform with the structural shape and functionality of Reddit without infringing on its
> intellectual property, you must abstract the design down to its core architectural paradigm: a
> **topic-driven, decentralized aggregation platform**. Unlike standard social media, which organizes
> conte…" [truncated in source paste — the input the aggregation research/build spec extends]

### B1. User

> Can you take this description of reddit in UX, UI and workflows
>
> And extend it even further in enough detail that we can look to build a prototype of a site that
> functions similarly
>
> Also looking at sites like slashdot and what feature and functionality differs is of interest to get
> a baseline

**Assistant** (deliverables): a full research pass grounding ranking math in Reddit's open-sourced
`_sorts.pyx` (Hot = `sign·log10(max(|s|,1)) + seconds/45000`; Best = Wilson lower bound at z=1.2816;
Controversial), and a Slashdot / Lobsters / Hacker News baseline. Produced both a build-ready
"Topic-Driven Aggregation Platform: Prototype Build Spec" and a comprehensive "Functional
Specification … With Slashdot and Peer Baseline" (adds Lemmy + Tildes). → filed at
`thinking/app/build-specs/graze-topic-aggregation-build-spec.md` and
`research/reddit-aggregation-ux-slashdot-baseline-2026-07.md`.

Assistant note (grounding, verbatim-in-substance):

> The ranking math no longer needs guessing. Reddit open-sourced its code (archived 2017), and the
> actual Hot formula is sign times log10 of net score, plus seconds since a fixed epoch divided by
> 45000. That 45000 means an older post needs 10x the score per 12.5 hours of age gap. Comment "Best"
> is the Wilson confidence sort at z = 1.2816. Slashdot's baseline difference is scarcity: moderators
> are occasionally chosen from eligible readers, get 5 points that expire in 3 days, rate via
> adjectives at ±1 each, and comments live on a -1 to +5 scale with reader-set thresholds. Editors,
> not votes, pick the front page. Lobsters contributes the transparency layer: a public invitation
> tree with a 70-day new-user restriction, a public moderation log, and typed flag reasons (lobste.rs
> blocks automated fetching, so mod-log details are secondary-sourced). One thing that changes the
> build calculus: Lobsters' entire codebase is public on GitHub, worth a read before M0.

### B2. User (the persona-switch pivot)

> Hold on, let's use this guide [the Persona-Switch Prototype methodology]
>
> And relate that back to your reddit ui/UX research, our goal is a reddit-esque site in this form,
> with personas and ability to travel UX paths as them. We want to build this as much like a real site
> in terms of maintainability and structure.
>
> We can call the mock graze.ing and the name is often seen as Graze and uses these trademark like
> statements
>
> Roam the open web.
> Feed your curiosity.
>
> The word Grave [sic — "Graze"] is a dark leaf green, and design the rest of the color pallet to
> complement

**Assistant** (deliverable): produced **Graze: Persona-Switch Prototype Spec** — re-substrates the
build spec onto the persona-switch + behavior-scale methodologies: event vocabulary, selector/action
contract, pure ranking engines, the dark-leaf-green palette + complement, `/f/:slug` "Field"
nomenclature, an 8-seat persona roster, a ~70-event deterministic seed + a 1,000-comment stress
thread, frontier markers, milestone build order, acceptance checklist. → filed verbatim at
`thinking/app/build-specs/graze-persona-switch-spec.md`.

### B3. User (the Lobsters correction)

Earlier in the session the user also described the target directly:

> I'd like to build a single page application PWA that runs on GitHub pages … a self contained local
> full on forum like experience here with several personas … a drop down at the top, which starts out
> logged out … and then I can just drop it down and select various profiles … two buttons next to
> that. One that is delete that removes all data from the site and one that is seed …

And earlier questioned the Lobsters framing:

> I thought Lobster's was basically a hacker[]news clown [sic — "Hacker News clone"], not sort of
> Reddit shaped, but I haven't really used it.

**Assistant** (correction, verbatim-in-substance):

> Right on Lobsters, and worth correcting my framing: it is HN-shaped, not Reddit-shaped. It's a
> social news site with one shared front page where category tags let people filter topics "without
> fragmenting the community between different boards", so no sub-communities at all. I pointed at it
> only for its moderation/transparency code, not its structure.

### B4. Continuation (the failed sibling session)

At one point the user tried to resume a separate build session that kept failing:

> Can you read this chat so we can continue, the other session just kept failing
> https://claude.ai/share/13a01c39-21ba-4d89-b2ed-c91d058eb86f

**Assistant** (verbatim-in-substance):

> I can't open that link, unfortunately. Claude.ai share pages block automated fetching, so I have no
> way to read the other session's contents from here. Two ways to get me caught up: paste the relevant
> parts of the conversation directly, or summarize where things stood. If the old chat produced any
> files or artifacts, re-upload those too.

The build then proceeded from the substantial pieces already present (reducers, selectors, store,
actions, engines, seed, tokens/components CSS, five screen modules), assembling the remaining
bootstrap (app/router/devbar), the remaining screens (queue, audit, settings, search, directory,
signup, landing, frontier), `index.html`/`sw.js`/`manifest`/`404`, the acceptance smoke test, and an
export patch. One normalization noted during assembly: the submit screen read `d.fmt` while the
router's draft used `format`; normalized to `format` to match `router.js`/`actions.js`.

---

## Provenance notes

- Both builds were built and deployed: **Stellin** at `stellin.app` (repo `CroftC/stellin`; product
  renamed from the working name "Meridian"), **Graze** at `graze.ing` (repo `CroftC/graze`).
- `graze.ing` was acquired and now supersedes the earlier `forum.croft.ing` domain thinking (user,
  2026-07-30) — see `NAMING.md` → "Forum-layer naming" and `COHESION.md`.
- Design tension carried forward (tracked in COHESION): the Graze mock spec §13 assumes a Next.js +
  Postgres scaled sibling, while the corpus's active forum plan
  (`plans/2026-07-27-read-first-forum-mvp.md`) chose a read-first lens over the public Bluesky
  AppView. Reconcile, do not silently merge.
