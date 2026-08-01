# Behavior-Scale

> **Provenance & naming.** Reusable methodology authored in a claude.ai design dialogue (2026-07),
> filed **content-faithful** (cleaned-paste — not a byte-pristine export; see PLAYBOOK §4). Raw
> session: `seeds/transcripts/raw/stellin-graze-behavior-scale-sessions-2026-07.md`. The dialogue
> named this "the behavioral twin"; **renamed to *behavior-scale* (user, 2026-07-31)** because "twin"
> wrongly implies exactly two deployments — there is one behavioral model, scaled across a **continuum
> of backends (mock → small → medium → large)**. This is one of the two reusable method docs for
> building Croft's persona-switch pads; see `README.md` for the index, the build pipeline, and the
> registry of built builds (Stellin, Graze). Companion: `persona-switch-prototype.md`.

---

One behavioral contract, scaled across a continuum of backends. A methodology for maintaining a fully functional in-browser model of a web2 product, using it as the permanent instrument for UX work, function-level testing, and agent-driven development — and scaling that same model, capability by capability, from a no-backend **mock** up to a **large** production backend without ever forking the front end.

This document is the reference to return to. It defines the vocabulary, the machinery, the discipline, the agent-instruction design, and a layered implementation roadmap, with file-format skeletons in the appendices.

## 1. Thesis

A **behavior-scale build** is not a fork and not a faked demo. It is the same front end and the same behavioral contract at every point on the continuum, running on a different **substrate** underneath: in-memory reducers at one end, a real backend at the other. Nothing in it is simulated; the reducers, engines, and policy are real software running real logic. Therefore:

- It stays truthful as a UX instrument, because seeded data and live data flow through identical code paths.

- It can test one function against another, because functions are routed to substrates independently.

- It can be published as a real product at a small scale for high-trust communities, because it is the product minus a large backend, not a demo dressed as one.

- It never answers scale questions from below its intended tier. Any question with a number attached (latency, load, concurrency, capacity) belongs to the tier that actually runs at that scale.

### The continuum (by backend)

One model, deployed at a point on a scale axis. The axis is set by how much real backend sits under it:

- **Mock** — no real backend and no real data: the starting UI. Identity is a persona dropdown; state is an in-memory event log over browser storage; every capability is on the `memory` substrate. "Mock" names the *absent backend*, **not** faked behavior — the logic is real; that is the whole point. This is what ships first and what the two current builds (Stellin, Graze) are today.

- **Small** — a small real backend for a high-trust group: local-first / honor-system identity, event-log sync across a handful of devices (`sync` substrate). Correct, honest software for a small community; not a hardened multi-tenant service.

- **Medium** — a mid-scale real backend: some capabilities on `api`, real accounts, real multi-user concurrency, but not yet at platform scale.

- **Large** — the full scaled platform: the production backend the mock was always a model of (e.g. an atproto AppView, or a Next.js + Postgres service).

The crux: **you scale one behavioral model for the site.** The front end and the contract do not change as you move along the continuum — only the substrate under each capability does. The methodology's whole job is to make that guarantee **mechanical instead of cultural**. Three artifacts carry the weight: a shared contract, a conformance harness, and agent-legible law. Everything else is detail.

## 2. Vocabulary

Terms this document leans on, defined before use:

- **Contract**: the single schema of events (mutations) and selectors (reads) that every deployment imports. If a mutation is not in the schema, it cannot exist anywhere.

- **Continuum / tier**: where a whole deployment sits on the backend scale axis — **mock / small / medium / large** (§1). A tier is the product of the substrate choices below plus the size of the real backend.

- **Substrate**: where an individual capability executes. Values: `memory` (in-browser reducers), `api` (real backend), `hybrid` (this capability on `api` while neighbors stay on `memory`), `sync` (memory plus event-log merge across devices — the small tier's transport).

- **Capability**: a named function of the product (posting, inviting, messaging, search) that the adapter can route to a substrate independently of its neighbors.

- **Adapter**: the routing layer inside the actions module. Every user-visible mutation dispatches through it; the substrate per capability is chosen in a config file, never at call sites. Which tier a build is at = which substrates its capabilities are on.

- **Scenario**: a named, deterministic event log with expected-outcome assertions attached. Scenarios are simultaneously UX repro cases, seeds, and backend test fixtures.

- **Conformance**: replaying the same scenarios against two substrates and asserting equivalent observable outcomes within declared tolerances.

- **Frontier marker**: a visible, clickable chip on any production capability the current tier intentionally lacks.

- **Proposal marker**: the mirror image, flagging experiments not yet shipped in production, so a mockup is never mistaken for shipped behavior.

- **Divergence ledger**: the one file recording every intentional difference between tiers, with reason and tolerance. Frontier and proposal markers render from it.

- **ADR**: architecture decision record, a short dated note capturing a choice, its context, and its reason.

- **Reducer / selector**: a reducer is a pure function `(state, event) -> state`; a selector is a pure read function over derived state. Policy (visibility, blocking, gating) lives in selectors so it holds on every surface.

## 3. The contract

One schema module, imported by the mock's reducers and by the real API alike. It defines:

- The event vocabulary: every mutation type with its payload schema (`post.created`, `invite.accepted`, `message.sent`, ...), including actor and timestamp semantics.

- The selector contract: the named reads and their result shapes (feed for viewer X, unread count, visible profile of Y as seen by X). Selectors become the read-API contract when a capability scales.

- Validation: payloads are checked against the schema at dispatch on every substrate, so malformed events fail identically everywhere.

Rules:

- The schema is the only place mutations are defined. Adding a mutation anywhere else is a build error, not a review comment.

- Schema changes are versioned; every tier migrates together or the change does not merge.

- Time and randomness are event inputs, never generated inside reducers, so replay is deterministic on every substrate.

## 4. The adapter and substrates

The actions module owns a routing table:

```
capability      substrate
-------------   ---------
posting         memory
inviting        memory
messaging       api
search          hybrid
```

- Same dispatch signature regardless of substrate. Calling code cannot tell where a capability runs.

- Substrate changes happen only in this config. Editing a call site to reach a backend directly is a violation.

- A build's **tier is read off this table**: all-`memory` is the mock; `sync` in play is the small tier; capabilities on `api` move it toward medium/large.

- `hybrid` exists so a newly scaled service is exercised inside full UX context from day one: one capability hits the real API while everything around it stays in-memory.

- The migration path per capability: implement the API to the schema → flip to `hybrid` → pass conformance → flip to `api` → **keep the memory implementation**, because it remains the substrate for the mock and small tiers.

## 5. The scenario library

Promote the single seed of the mock into a library of named event logs, one per situation:

- Each scenario is deterministic: fixed inputs, timestamps as offsets resolved at replay, ordered events, no hidden randomness.

- Each scenario carries assertions: seat-level, observable expectations ("after replay, seat A's unread count is 2; seats C and D see nothing of each other in feed, search, or suggestions").

- Coverage rule with teeth: **no new mutation without a scenario touching it**. A schema addition and its first scenario land in the same change.

- Scenarios double as UX repro cases (Export/Import a scenario to hand a designer an exact state), as seeds for manual testing, and as fixtures the backend runs in CI.

- Variant testing rides on this for free: because engines are pure and scenarios deterministic, run ranking v1 and v2 over the same scenario and diff outputs; a dev-bar variant picker makes it interactive.

## 6. The conformance harness

The harness is the proof that two tiers are the same product.

- Mechanism: replay each scenario against substrate A and substrate B; evaluate the same selector assertions on both; compare.

- Compare **observable outcomes only**: selector outputs, counts, visibility, ordering where ordering is contractual. Never compare internals; the substrates are supposed to differ inside.

- Tolerances are declared per assertion, explicitly. Example: "ranking order may differ, membership may not." An undeclared difference is a failure; a declared one references its divergence-ledger entry.

- The harness is a **merge gate**: a change to a scaled capability is not done until the mock passes conformance on the affected scenarios, and vice versa.

- Output is a report an agent can read and act on: per scenario, per assertion, pass/fail/tolerated, with the ledger reference for tolerated items.

## 7. The divergence ledger, frontiers, and proposals

Drift is what kills a behavior-scale build. The countermeasure is one file where every intentional difference lives:

```
id, kind (frontier | proposal | tolerance), capability,
description, reason, tolerance (if any), owner, date, status
```

- **Frontier** entries render as dashed chips wherever a higher-tier-only path would sit; clicking explains the gap. Registering the frontier and deferring the path happen in the same commit. Dead buttons are banned.

- **Proposal** entries mark experiments mocked ahead of production with a visibly different chip, so no one mistakes a mockup for shipped behavior.

- **Tolerance** entries back the conformance harness's allowed differences.

- The ledger has a screen (`#/frontiers` grown up): the UI itself tells the truth about what is real where.

## 8. Invariants (the law)

Stated as MUST/NEVER because agents and reviewers apply these mechanically:

1. All mutations MUST flow through the actions adapter. No component writes state or calls a backend directly.

2. Policy (visibility, blocking, gating) MUST live in selectors, NEVER in components.

3. Reducers and scenarios MUST NOT call `Date.now()` or any randomness. Time and randomness are inputs resolved at dispatch or replay.

4. Substrate selection (and therefore a build's tier) MUST happen only in the routing config.

5. Every mutation type MUST exist in the shared schema before any implementation uses it.

6. Every new mutation MUST land with at least one scenario exercising it.

7. Deferring a path and registering its frontier MUST be the same commit.

8. Behavior present only at one tier MUST carry a proposal (or frontier) entry before it renders.

9. Conformance MUST pass (or differences be ledgered as tolerances) before a capability change merges on either side.

10. Results from a tier below the intended production scale MUST NEVER be cited as evidence for performance, concurrency, capacity, or any scale claim. The mock answers "does it behave and feel right," never "does it hold up."

11. When documentation and behavior disagree, behavior (schema plus scenario suite) wins and the documentation is corrected. Prose is never the source of truth.

## 9. Decision procedures

One procedure per situation, written as ordered steps an agent executes verbatim.

### Adding a user-visible function

1. Define the event(s) and payloads in the schema.
2. Write or extend the reducer; state derives, nothing is hand-maintained.
3. Expose reads as selectors, with policy enforced there.
4. Add the action through the adapter; wire the UI with all four states (skeleton, empty, error, gated).
5. Author a scenario touching the new mutation, with seat-level assertions.
6. Run conformance if any affected capability is on `api` or `hybrid`.
7. Run the acceptance checklist items for the affected screens.

### Changing an engine (ranking, recommendations, limits)

1. Implement the change as a variant alongside the current version.
2. Run both variants over the scenario library; record the output diff.
3. If the diff violates a contractual assertion, either revise or ledger a tolerance with reason.
4. Promote the variant; keep the diff record with the change.

### Scaling a capability (moving it up the continuum)

1. Implement the API against the schema for that capability's events and selectors.
2. Flip the routing config to `hybrid`.
3. Run conformance on every scenario touching the capability; ledger tolerances explicitly.
4. Flip to `api`. Keep the memory implementation; it is still the substrate for the mock and small tiers.
5. Add the capability's scenarios to the backend CI as fixtures.

### Escalation

On schema conflicts, tolerance ambiguity, or any case where the procedure underdetermines the choice: STOP, surface the question, and log it as an ADR rather than inventing an answer. An invented convention that ships is drift with a head start.

## 10. Designing instructions for future agents

Agents follow invariants, decision procedures, and runnable checklists far better than prose philosophy. The root agent document is therefore short law, roughly one page:

- The invariants of section 8, verbatim.

- The decision procedures of section 9, verbatim.

- Pointers, not copies: the schema module, the scenario library, the routing config, and the divergence ledger are named as the sources of truth. The doc states rule 11 explicitly so an agent resolves doc-vs-behavior conflicts by fixing the doc.

- Self-verification with teeth: every task ends by running the conformance command and the affected acceptance checklist, and reporting results in the task output. Checklist items stay seat-level and observable ("switch to seat B; the acceptance notification is present and the badge reads 1st").

- The escalation rule and the ADR format, so stopping is a defined action rather than a failure.

Anti-patterns to keep out of the agent doc: narrative history, duplicated schema definitions (they drift), aspirational sections ("eventually we should..."), and any instruction that requires judgment about scale versus function; the boundary is rule 10 and it is absolute.

## 11. The small tier (a shippable, honest small-scale product)

The **small tier** publishes the build as a real product at a small scale for a high-trust community. It is truthful because nothing in it is simulated. Requirements to do it honestly:

- **Profile picker, not scaffolding.** The persona dropdown becomes the product's profile switcher for shared devices (a family tablet, a clubhouse kiosk, a classroom machine). This mode works with zero changes.

- **The `sync` substrate** for multi-device communities. The event log is the natural unit of sync: union two logs, order events deterministically (timestamp, then actor, then id), keep reducers idempotent, and every device converges on the same state. This is essentially a grow-only set CRDT (conflict-free replicated data type: a structure designed so replicas can merge in any order and still agree). Transport options in ascending effort: export/import files passed around (already built, genuinely workable for a tiny group), a shared folder, or a relay on the order of 50 lines that accepts and rebroadcasts event batches. Moving to the small tier is a routing-config value, not a fork.

- **Identity is honor-system, and the README says so.** No auth means anyone can select any persona, and selector-enforced blocking is client-side courtesy, not security. For a small high-trust community this is the historically correct model, the same one BBSs and family servers ran on, but it must be named out loud, never discovered.

- **Storage graduates.** For daily use, move persistence from localStorage to IndexedDB behind the existing storage adapter, since a quota on the order of 5 MB stops being a budget and becomes a wall. The adapter was built for exactly this swap.

- **Data ownership as a feature.** Export/Import is the community's backup and portability story; document it as such.

The elegant end state: the mock, the small tier, and the large platform are **points on one continuum of one contract**, the same codebase with different substrate values, and the conformance harness built for engineering reasons doubles as the public proof they behave identically.

## 12. Boundaries

Write this into the build's own README and the agent doc, verbatim in spirit:

- A behavior-scale build answers "does it behave correctly and does it feel right."

- The mock never answers "does it hold up." In-memory correctness implies nothing about concurrency, conflicts, latency under load, abuse handling, or capacity. Any question with a number attached routes to the tier that runs at that scale.

- The small tier inherits the same boundary: it is correct and honest software for a small high-trust group, not a hardened multi-tenant service.

## 13. Implementation roadmap, in layers

Each layer is independently valuable; stop at any of them and the artifacts still pay rent.

1. **Prototype layer** (the persona-switch mock): event log, reducers, selectors, personas, seed, dev bar, frontier chips, acceptance checklist. Value: a truthful UX instrument.

2. **Contract layer**: extract the schema into a standalone module with payload validation; convert the seed into the first scenarios with assertions. Value: mutations become law; repro states become shareable.

3. **Adapter layer**: formalize capabilities and the routing table with `memory` as the only substrate. Value: the seam (and the tier dial) exists before anything needs it.

4. **Conformance layer**: build the harness running scenarios against two substrates (initially memory vs memory with a variant engine, to prove the harness itself). Add the divergence ledger and proposal markers. Value: drift becomes detectable.

5. **Scaling layer**: first capability to `hybrid` then `api` via the section 9 procedure; scenarios enter backend CI; conformance becomes a merge gate. Value: the mock and the large platform are provably the same product.

6. **Agent layer**: write the root agent doc (section 10) and adopt ADRs. Value: future agents extend the system without eroding it.

7. **Small-tier layer**: IndexedDB migration, `sync` substrate, profile-picker polish, honest README. Value: a publishable small-scale edition that is true.

## 14. Failure modes and countermeasures

- **Parallel-copy drift**: maintaining a tier as a separate codebase. Countermeasure: it is the same front end with a swapped adapter; there is no second codebase to drift.

- **Doc drift**: prose describing behavior that changed. Countermeasure: rule 11; docs point at schema and scenarios as truth.

- **Simulated creep**: stubs that fake outcomes instead of computing them. Countermeasure: reducers must derive; anything faked is either a frontier (visible) or does not render.

- **Undeclared differences**: a build quietly behaving unlike its scaled sibling. Countermeasure: conformance as a merge gate; tolerances only via the ledger.

- **Scenario rot**: scenarios that no longer assert anything meaningful. Countermeasure: the no-mutation-without-a-scenario rule plus periodic pruning recorded as ADRs.

- **Scale claims from below-tier**: "it worked in the mock so the service is fine." Countermeasure: rule 10, stated everywhere the build reports results.

- **Small-tier misrepresentation**: shipping without naming the honor-system identity model. Countermeasure: the section 12 boundary text in the README.

## Appendix A: root agent doc skeleton

```
# Agent Instructions: <project>

Sources of truth, in order: /contract/schema.*, /scenarios/, /config/routing.*, /ledger/divergence.*.
When these disagree with any document including this one, they win; fix the document.

## Invariants
[Section 8, verbatim, numbered.]

## Procedures
[Section 9, verbatim: add-function, change-engine, scale-capability, escalate.]

## Verification
Every task ends with:
1. `npm run conformance -- --scenarios <affected>`  (or project equivalent)
2. The acceptance checklist items for affected screens, executed seat by seat.
3. A report of results, including tolerated differences with ledger ids.

## Escalation
STOP and write an ADR to /adr/ when: schema conflict, tolerance ambiguity,
procedure underdetermines the choice. Never invent a convention.
```

## Appendix B: file-format skeletons

Scenario:

```js
{
  id: "invite-accept-basic",
  description: "Pending invite accepted; badges and notifications update",
  events: [ { t: "-2d 09:00", actor: "u.priya", type: "connection.invited", payload: {...} }, ... ],
  assertions: [
    { seat: "u.maya", selector: "pendingInvites", expect: { count: 1 } },
    { seat: "u.priya", after: "accept", selector: "notifications.unread", expect: { count: 1 } },
    { seat: "u.maya", selector: "degree", args: ["u.priya"], expect: 1 }
  ],
  tolerances: []            // ledger ids if any
}
```

Routing config (the tier dial):

```js
export const routing = {
  posting:   "memory",
  inviting:  "memory",
  messaging: "api",
  search:    "hybrid",
  // small tier flips relevant capabilities to "sync"
};
```

Divergence ledger entry:

```js
{ id: "DL-014", kind: "tolerance", capability: "feed-ranking",
  description: "Order may differ between substrates; membership may not",
  reason: "Server ranker uses model features unavailable in-memory",
  tolerance: "set-equality on visible post ids per page",
  owner: "…", date: "…", status: "active" }
```

ADR:

```
# ADR-007: <decision>
Date / Status / Context (2-3 sentences) / Decision / Consequences
```
