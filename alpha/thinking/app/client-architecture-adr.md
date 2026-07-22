# ADR: Shared functional core + per-platform shells (the Croft client architecture)

**Status:** Accepted (2026-06-22) — the user's decision, captured per PLAYBOOK §2c.

**Deciders:** user (Chase). **Context source:** the Phase-0 import (CroftC PR #10,
`experiments/croft-app-phase0/`) demonstrated this shape running, and the user named it the most
mature model we have and the one prior client work should adapt to.

> **Companion (2026-07-22):** this ADR is the *shared-core / per-platform-shell* axis. How the *estate of
> apps* shares storage and session across origins in the browser — decided by the account-kernel spike
> (K1/KC0), incl. why a cross-subdomain shared kernel fails on WebKit/iOS and the single-origin +
> isolated-subdomains + optional-BFF topology — is its sibling doc:
> `estate-architecture-and-browser-constraints.md`.

---

## Context

We are accumulating more than one Croft client surface: the **Bluesky-pond feed CLI**
(`experiments/croft-app-phase0/crates/cli`), a **web** shell (Leptos/WASM), a **desktop** shell
(Tauri), and the separate **iroh/P2P group-chat CLI** (`experiments/croft-group/crates/croft-chat-cli`).
Without a shared discipline, each surface re-implements feed/group logic, multiplying maintenance and
letting behavior drift between platforms. The design-imperative dialogues
(`local-first-as-design-imperative.md`) and the app design-philosophy already pointed at a Crux-style
**functional-core / imperative-shell** split; Phase 0 proved it in code across three platforms.

## Decision

**Adopt one shared, pure functional core consumed by thin per-platform shells, with two orthogonal
callout axes. This is THE Croft client architecture; new and prior client work adapts to it.**

The shape, as Phase 0 realizes it:

```
SHARED (platform-agnostic, no I/O / async / clock; WASM-clean)
  core     pure (state, intent) -> (state, effects)  + projection (model -> view model)
  shell    cross-platform composition logic (layout / slots / pinning)
  design   design system (tokens / primitives)
  <port>   the I/O contract as a trait, held by the shell, never called by the core (DECISION 1)

PER-PLATFORM SHELLS (thin; the only place platform code lives)
  cli      effects.rs + runtime loop + text render
  web      effects.rs + runtime + Leptos components
  desktop  effects.rs + Tauri host
  ...      future surfaces add only their own effects.rs + render target
```

**Two callout axes (this is the "maybe more than platform" the user named):**
1. **Platform axis** — each platform shell provides its own `effects.rs`: the handler that *performs*
   the core's effect-requests (which the core emits as **data**, never as function calls) and feeds
   results back in as new intents. Same core, different effect-performers.
2. **Implementation axis** — swappable **adapters behind a port**, orthogonal to platform: the Bluesky
   port has a fixture-backed fake and a real HTTP adapter; the chat Transport port has an in-proc fake
   and (later) a real-iroh adapter. The core and shells are blind to which adapter is wired.

**Consequence for prior work:** `croft-chat-cli` adapts to this model. It already has the
implementation-axis seam done (the `Transport` port + in-proc fake); what it lacks is the shared core
+ shell + per-platform `effects.rs`. Because its core isn't built yet, this is **greenfield growth on
an existing port, not a refactor** (see `croft-chat-cli` memory + ROADMAP_TODO E19).

## Rationale

- **Maintenance minimization (the stated driver):** business logic lives once, in the core; a new
  platform is a new thin shell (one `effects.rs` + a render target), not a re-implementation. Behavior
  can't drift between cli/web/desktop because they share the core verbatim.
- **It is local-first / provenance discipline made structural:** the pure core *is* the unit that
  holds state and emits effects-as-data; the shell reconciles with the world. Effects-as-data (not
  effect-as-function-call) is what keeps the core pure, synchronous, testable, and WASM-clean — and it
  is the same DECISION 1 the port-ownership argument forced (`update` is
  `(model, intent) -> (model, Vec<effect>)`, which an awaited port call cannot satisfy).
- **Testability:** the core is tested as a pure function (Phase 0's 20 acceptance tests A1–D2 + P1–P7);
  each adapter is tested behind its port with a deterministic fake; platform shells stay thin enough to
  cover with render/snapshot tests.
- **It composes toward one-shell-many-ponds:** a single core/shell can host a feed pond and a group
  pond behind their respective ports — the "Croft Group pond = lineage-groups surfaced, planes anchored
  to principals" direction. (See Consequences for the open structural question.)

## Consequences

- **Positive:** minimal maintenance; no cross-platform drift; cheap new surfaces; the implementation
  axis lets the same scenario script run against fake or real adapters (the deterministic regression
  bed `croft-chat-cli` was always meant to be).
- **The delta that is NOT a copy:** the **chat core is richer than the feed read-model.** Phase 0's
  core is one-directional (fetch → project → render; effects are fetches). The group core is
  bidirectional/real-time and carries MLS epoch state, fork/merge with **reconvergence-policy-per-plane
  (declared at intent-to-collaborate)**, and the delegate/governance planes. The *pattern* transfers
  cleanly; the *core content* is substantially more complex, and today's transcripts supply exactly the
  discipline it needs (planes-by-blast-radius, the rights-floor) that the feed core never did.
- **Structural decomposition — RESOLVED 2026-06-22 (option C):** **per-pond domain cores
  (bounded contexts) unified by the shared `shell` composition layer** — *not* one god-core (which
  couples a Bluesky read-model with an MLS group engine), *not* two disconnected cores (which would
  re-fatten the per-platform shells). The group pond is symmetric to the feed pond: add a `group-core`
  + a Transport port; the existing `shell` composes both ponds' view models. This is the honest-seams
  thesis made structural (ponds kept native, not fused) and is what Phase 0 already started
  (`crates/core` = the feed pond's domain; `crates/shell` = composition; `crates/bluesky` = the pond's
  port). Per-pond concerns (the group core's MLS epoch state, fork/merge with
  reconvergence-policy-per-plane, governance/delegate planes) live *inside that pond's core*, never
  smeared across a shared core.

  **Cross-pond awareness vs. interactivity (the line that keeps C clean):**
  - **Awareness (expected now, cheap):** read-only surfacing of one pond's content inside another's
    view — "show a Bluesky reply in the chat." This is **composition in the shell**: the chat message
    carries a *reference* (the `at://` URI — provenance), the shell resolves it via the feed-core's
    port to a renderable card, nothing flows back. No core coupling, no idiom translation.
  - **Interactivity (deferred until wanted):** *acting* in pond A from pond B (reply to Bluesky from
    chat) needs a **broker** to translate idioms (a chat action → a Bluesky API call). It sits
    *between* the cores (or as its own adapter), **never inside a pond core**. Out of scope by default
    per honest-seams; introduce the broker only when a concrete cross-pond action is committed to.
- **Carry:** the as-built Phase-0 spec and the more-developed `thinking/app/` spec must reconcile on
  graduation (ROADMAP_TODO C7); platform exclusion realities apply (Phase 0 keeps `web` wasm-only and
  `desktop`/Tauri out of the host workspace — a precedent for how per-platform shells stay isolated).

## Links

- Demonstrated: `experiments/croft-app-phase0/` (core/shell/design/cli/web/desktop/bluesky).
- Pattern + derivation: `thinking/local-first-as-design-imperative.md`,
  `thinking/app/design-philosophy.md`, `seeds/transcripts/raw/croft-app-portdecision-review-2026-06-21.md`.
- Prior work to adapt: `experiments/croft-group/crates/croft-chat-cli` (ROADMAP_TODO E19).
- Principle: `crystallized/principles.md` (Tier 3).
