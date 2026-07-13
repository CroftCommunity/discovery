# Adapt croft-chat-cli to the shared-core / per-platform-shell model (group-core, messaging happy-path)

> **Status: CLOSED (2026-06-22).** All 8 phases shipped; nothing skipped or deferred within
> scope. The happy-path slice is live: `group-core` (pure, transport-free, 10 tests) +
> `croft-chat-cli` shell (Transport port + InProcBus, perform_effect/apply/pump, render, the
> `croft-chat demo` binary) — 16 workspace tests green, the two-peer round-trip + binary smoke
> proving core-through-Transport wiring (option C). L1–L6 remain sequenced (see "Later phases"),
> not started. Tooling change this run: TDD enforcement moved from real-time hooks to a
> commit-time gate (see the close-out entry).

> Phase-plan **Pass 1** (base + reasoning), updated after the open-question review.
> Scope: extract `croft-chat-cli` into a new dedicated `croft-group` workspace, grow a pure
> `group-core`, and wire it through the existing `Transport` port + in-proc fake, with a thin CLI
> shell — the messaging **happy-path only** (join → send → receive → render over the in-proc bus).
> MLS/lineage identity, fork/merge + reconvergence-policy-per-plane, governance/delegate planes, and
> the real-iroh adapter are **later phases (L1–L6), sequenced but not built here**. Plan only — no
> code until Pass 3.
>
> **Confirmed decisions (2026-06-22 review):** wire = `serde_json`; pump = explicit synchronous tick;
> **crate location = NEW dedicated `croft-group` workspace** (user override of the in-place
> recommendation — adds Phase 1, the move); identity/topic = hardcoded placeholders.

## Outcome Summary

| Phase | Outcome | Commit | Note |
|---|---|---|---|
| 1 — workspace + move | ✅ SHIPPED | `6f2cbca` | `git mv` (pure renames) into `croft-group`; both workspaces build; 2 transport tests green; stale path pointers refreshed (discovery edits left for the user's own commit) |
| 2 — group-core scaffold + model | ✅ SHIPPED | `bb70e10` | pure `group-core` crate; `ChatMessage`/`Model` + inline test (RED→GREEN); clippy/fmt clean; workspace 3 tests green |
| 3 — intent/effect + update join | ✅ SHIPPED | `b14ced2` | `Intent`/`Effect` enums; `update` JoinGroup arm (fresh→[Subscribe], rejoin→no-op); exhaustive; 3 group-core tests |
| 4 — wire round-trip | ✅ SHIPPED | `3b5f1d9` | version-tagged JSON frame; `serialize`/`deserialize` + `WireError` (thiserror, no payload in errors); `ChatMessage` stays serde-free via private `FrameV1`; 5 group-core tests |
| 5 — update send/receive | ✅ SHIPPED | `46eb59a` | SendMessage→append+Publish; FrameReceived Ok→append / Err→observable `FrameDropped`; malformed boundary cases (empty/truncated/non-UTF8); 8 group-core tests. First commit through the new commit-gate (ran cargo test) |
| 6 — project/view | ✅ SHIPPED | `b0ac1fd` | pure `project(&Model)->ViewModel`; platform-agnostic `MessageLine`/`ViewModel`; ordered + joined-flag; 10 group-core tests. Core now happy-path-complete + transport-free |
| 7 — shell effects + pump (wiring) | ✅ SHIPPED | `f215384` | **make-or-break GREEN**: alice→bob over the real Transport port; `perform_effect` (port held only here) + `apply`/`pump`; wiring test mutation-checked (sabotaged Publish → test fails). 13 workspace tests |
| 8 — binary entry point | ✅ SHIPPED | `e25b2cb` | `croft-chat demo` two-peer scenario + `render`; binary smoke test (message in both views); README croft-group entry; 16 workspace tests |

## Problem Statement

The Croft client architecture is settled (ADR: `discovery/thinking/app/client-architecture-adr.md`,
option C): **per-pond domain cores (bounded contexts) unified by a shared shell, with two callout axes
— platform (`effects.rs`) and implementation (adapters behind a port).** The Bluesky-feed pond proved
the pattern in `experiments/croft-app-phase0/` (pure `core` + `cli`/`web`/`desktop` shells + a
`bluesky` port). The group/messaging pond — currently `experiments/iroh/crates/croft-chat-cli` —
predates the pattern: it has the **implementation seam done** (`Transport` port + deterministic
`InProcBus` fake + 2 passing tests) but **no core/shell** (`src/lib.rs` is just `pub mod transport;`;
`src/main.rs` is a WIP stub).

The task: bring the group pond onto the same pattern as a **per-pond domain core** (`group-core`),
plugged through the *existing* `Transport` port, with `croft-chat-cli` as its thin shell — **and house
both in a new dedicated `croft-group` workspace**, extracting `croft-chat-cli` from the alt-drive
(encrypted-vault) workspace where it has been a guest. Because the core does not exist yet, the core
build is **greenfield growth on an existing port, not a refactor**. Constraint: strict TDD (RED →
VERIFY RED → GREEN → VERIFY GREEN → MUTATE → REFACTOR), the rust-enforcer discipline,
wait-for-commit-approval per commit, and "deps only when a passing test forces them."

This plan delivers a vertical happy-path slice (two peers exchange a rendered message over the in-proc
bus) and sequences the richer group concerns as explicitly-named later phases.

## Reasoning

**Why a dedicated `croft-group` workspace (the override).** `croft-chat-cli` currently lives in
`experiments/iroh` — the **alt-drive** Cargo workspace, whose CLAUDE.md, README, and DESIGN are about
an encrypted personal vault, not chat. The chat CLI was always a guest there. The user chose to
extract it now into `experiments/croft-group/` rather than grow `group-core` inside the vault
workspace. This is **a same-repo move** (verified: `experiments/iroh` is *not* a separate git repo —
the `repository = chasemp/alt-drive` line in its Cargo.toml is stale manifest metadata; the real git
toplevel is `CroftCommunity/experiments`). `croft-chat-cli` has **no dependency on any alt-drive
crate** (its Cargo.toml deps are empty), so it extracts cleanly with `git mv`. Trade-off accepted:
a little churn now (the move) in exchange for a clean product home and not re-deciding move-vs-graduate
later. This also gives `group-core` + the feed pond's `app-core` a parallel home story under the
product, distinct from the substrate spikes.

**Why mirror Phase 0 rather than invent.** Phase 0 is the *accepted* pattern (the ADR). Mirroring it
keeps both ponds on one discipline — the maintenance-minimization driver. `group-core` mirrors
`app-core`'s file shape: `model.rs` / `intent.rs` / `effect.rs` / `update.rs` (pure
`update(model, intent) -> (Model, Vec<Effect>)`) / `project.rs` / `view.rs`, plus a `wire.rs` (the
chat analog of `bluesky/wire.rs` — the core authors messages, so it owns their serialization).
`croft-chat-cli` mirrors `pond-cli`: `effects.rs` (the only place the port is held/called — DECISION
1), a runtime, `render.rs`, `main.rs`.

**Why `group-core` is a separate crate from `croft-chat-cli`.** Option C = per-pond domain cores
unified later by the shared shell. The core must be a standalone, pure, WASM-clean crate (so the same
core can later back a web/desktop shell), exactly as `app-core` is separate from `pond-cli`. The
**`Transport` port stays in `croft-chat-cli`** (the shell side): DECISION 1 — the shell holds the
port, the core never touches it. `group-core` therefore depends on **nothing transport-related** — it
emits `Subscribe` / `Publish { bytes }` as *effect data* and the shell performs them.

**The one genuine architectural delta from Phase 0 — inbound needs a pump.** Phase 0's runtime
(`drive`) runs to *quiescence*: a request/response feed. Chat is **bidirectional and long-lived**:
receiving means the shell must **`drain()` the transport** and feed each inbound `Frame` back as a
`FrameReceived` intent (the core cannot — drain is I/O). So the shell adds a **pump** step (drain →
`FrameReceived` intents → process to quiescence), the chat analog of `drive`. Confirmed shape: an
**explicit synchronous tick** the test/binary calls (no async runtime), mirroring Phase 0's offline
`block_on` philosophy. The real-iroh adapter (L5) supplies the real event loop. This pump is the only
structurally-new piece.

**Why scope to the happy-path.** The group core's eventual richness (MLS epoch state, fork/merge with
reconvergence-policy-per-plane, governance/delegate planes) is real but is exactly the content that
should not be rushed — it is where the planes discipline earns its keep and deserves its own plans. A
working send/receive/render slice first (a) proves the core-through-Transport wiring end-to-end (the
make-or-break for option C here), (b) establishes the workspace/crate/shell skeleton the richer phases
extend, and (c) gives the deterministic regression bed croft-chat-cli was always meant to be.

**Alternatives considered and rejected.**
- *Grow group-core inside the alt-drive workspace (in-place).* The Pass-1 recommendation; **overridden**
  by the user in favor of the dedicated workspace. Recorded for provenance: in-place was lower-churn
  but kept chat as a guest in the vault workspace.
- *Grow the core inside `croft-chat-cli` (no separate crate).* Rejected: violates option C and blocks a
  future shared web/desktop shell over the same core.
- *One shared core hosting feed + group as planes now.* Rejected: option A (god-core), explicitly not
  chosen.
- *Push-based transport / real async runtime now.* Rejected: the existing `Transport` is pull-based and
  its in-proc fake is deterministic; the pump adapts to the port. Async arrives with real-iroh (L5).

## Verified Assumptions

All confirmed by reading source on 2026-06-22 (not inferred):

- **Repo boundary:** `experiments/iroh` is **not** a separate git repo — `git rev-parse --show-toplevel`
  from inside it returns `…/CroftCommunity/experiments`. The `repository = chasemp/alt-drive` in
  `experiments/iroh/Cargo.toml` is stale workspace metadata, not a git remote. So moving
  `croft-chat-cli` to `experiments/croft-group/` is a same-repo `git mv` (no cross-repo extraction, no
  IP wrinkle beyond the already-exercised A8 posture).
- **`croft-chat-cli` extractability:** its `Cargo.toml` `[dependencies]` is empty ("None yet"); it uses
  only `std`. No alt-drive crate depends on it (workspace members are the `altdrive-*` crates +
  `croft-chat-cli`; the altdrive crates do not reference it). Clean to move.
- **`Transport` port API** (`croft-chat-cli/src/transport.rs:43-53`): `subscribe(&mut, &Topic)`
  (idempotent), `publish(&mut, &Topic, Frame)` (to *other* subscribers, **no self-echo**),
  `drain(&mut) -> Vec<Frame>` (frames since last drain, **consuming**). `Topic`/`Frame` opaque; payload
  never inspected. `tests/transport.rs` confirms multi-peer round-trip + drain-consumes.
- **alt-drive workspace** (`experiments/iroh/Cargo.toml`): resolver 2; members include
  `crates/croft-chat-cli` + 5 `altdrive-*` crates; `[workspace.package]` edition 2021, rust-version
  1.75, license `MIT OR Apache-2.0`, author `Chase Pettet <chase@owasp.org>`. `croft-chat-cli`'s
  manifest inherits these via `*.workspace = true`, so the **new `croft-group` workspace must define a
  matching `[workspace.package]`** or the inherited fields fail to resolve.
- **Phase-0 pattern** (`experiments/croft-app-phase0/crates/...`, read firsthand): `core/effect.rs`
  (data enum), `core/intent.rs`, `core/update.rs` (pure `(model,intent)->(Model,Vec<Effect>)`,
  exhaustive match, debug invariant), `cli/runtime.rs` (`drive` to quiescence), `cli/effects.rs`
  (`perform_effect`, the only port caller), `cli/executor.rs` (offline `block_on`, panics on pending),
  `cli/main.rs` (arg parse, fake/real, drive, render). Package names `app-core`/`app_core`,
  `pond-cli`/`pond_cli`, bin `pond`.
- **TDD discipline mandated** (`experiments/iroh/CLAUDE.md`, which travels with `croft-chat-cli` and
  applies equally in the new workspace): test-first for all `crates/*/src/` code; rust-enforcer
  (`Result<T,E>`, no `unwrap()`/`expect()` outside tests, `thiserror`, doc comments, `clippy::pedantic`,
  `Zeroize` for secrets); commit approval before each commit; Cargo manifests are build config (not
  TDD-gated). The new `croft-group` workspace should carry a CLAUDE.md asserting the same discipline.

## Documentation Impact

- `experiments/iroh/Cargo.toml` — **remove** `crates/croft-chat-cli` from `[workspace] members`
  (Phase 1).
- `experiments/croft-group/Cargo.toml` — **new** workspace manifest with a matching
  `[workspace.package]` (edition 2021, rust 1.75, license, authors) and members `crates/croft-chat-cli`
  + `crates/group-core` (Phase 1 / grown in Phase 2).
- `experiments/croft-group/CLAUDE.md` — **new**: assert the TDD + rust-enforcer discipline for the
  workspace (port the relevant parts of the alt-drive CLAUDE.md; drop the vault-specific content)
  (Phase 1).
- `experiments/croft-group/crates/croft-chat-cli/Cargo.toml` — moved; add `group_core` dep (Phase 7)
  + `serde`/`serde_json` carried by `group-core` (not the shell).
- `experiments/croft-group/crates/group-core/Cargo.toml` — new (Phase 2); `serde`/`serde_json` added by
  the wire round-trip test (Phase 4).
- `experiments/iroh/docs/{roadmap.md, transport-layers.md}`, `experiments/iroh/README`/`DESIGN`, and
  `experiments/iroh/CLAUDE.md` — *grep at execution* for `croft-chat-cli`/`croft-chat`/`Transport`
  references and update/remove (the chat CLI is leaving the vault workspace) (Phase 1). (Pass-2:
  widened the grep scope — transport-layers.md and CLAUDE.md were missing.)
- `experiments/README.md` (experiments-repo index) — add a `croft-group/` entry (new product workspace:
  `group-core` + `croft-chat-cli`); this **supersedes** the COHESION §23 "iroh experiment not indexed"
  gap for the chat CLI (it's no longer in iroh). Final phase.
- **discovery-repo path updates — Phase 1, separate commit (Pass-3 reschedule from Phase 8).** The move
  makes these stale at the P1 commit, so they refresh in P1: `COHESION.md` §23, `ROADMAP_TODO.md` E19,
  `thinking/app/client-architecture-adr.md`, `thinking/local-first-as-design-imperative.md`, and the
  `croft-cli-tdd-production` **memory** all cite `experiments/iroh/crates/croft-chat-cli` → update to
  `experiments/croft-group/crates/croft-chat-cli`. Separate repo → its own commit under the chasemp
  identity.
- *grep done at plan time:* the discovery refs above + the memory are pointers that stay valid in intent;
  only the **path** and the "core not built yet" framing go stale on the move/slice.

## Concurrency Map

**All phases sequential.** Phase 1 (workspace + move) must precede everything. The core crate is then
built file-by-file (`model` → `intent`/`effect`/`update` → `wire` → `update` again → `project`/`view`),
then the shell consumes the finished core, then the binary drives the shell. `update.rs` is written in
Phase 3 and extended in Phase 5 (same file, ordered). No parallelism available or attempted; no
worktrees. Single feature branch off the experiments repo; commit per phase after green.

**Pass-2 disjointness/missed-parallelism audit (Pass-3-refreshed):** confirmed sequential. The nearest
parallel candidates all collide: Phases 3 and 4 share `crates/group-core/src/lib.rs` (both edit
re-exports); Phases 3 and 5 share **both** `crates/group-core/src/update.rs` **and**
`crates/group-core/src/effect.rs` (Pass-3 added the `FrameDropped` variant to effect.rs in Phase 5, so
effect.rs is now a second 3↔5 collision — does not change the conclusion). Phase 1 must precede
everything (the workspace must exist). No disjoint-write-set pair exists, so no parallel set is proposed. Shared state is limited
to the in-test `InProcBus` (constructed per test) — no git/process/port/env ambient state; no re-entry
verification needed (no parallel phases).

## Phases

> No Phase 0 (Discovery): assumptions verified by reading. Each phase ≤3 source files (Cargo/CLAUDE
> manifests are config, not counted), compiles, leaves a working state, RED-first.
>
> **Wiring-gate structure (Pass-3 calibration of the TDD ordering gate).** This slice grows a pure
> `group-core` file-by-file (Phases 2–6) before any entry point exists, then wires it through the
> Transport port (Phase 7) and exposes the binary (Phase 8). Phases 2–6 are therefore legitimately
> *component-level* — their `cargo test -p group-core` verification proves a unit, not the call chain,
> because **there is no call chain to exercise until the shell exists.** The wiring tests are
> **Phase 7** (two-peer round-trip over the real `Transport` port — the make-or-break for option C)
> and **Phase 8** (binary smoke check through `main`). This is a deliberate exception to the
> "every phase verifies through the entry point" gate, not an oversight: the entry point is built in
> Phase 7. If Phase 7's round-trip cannot be made to pass over the in-proc bus, the architecture —
> not just a component — is wrong, and that is exactly what this structure surfaces.

### Phase 1: Create the `croft-group` workspace + move `croft-chat-cli` into it — ✅ SHIPPED (`6f2cbca`)
**Delivered (2026-06-22):** as specified, with three reconciliations: (1) the iroh-side doc cleanup was
a **verified no-op** — grep found zero `croft-chat-cli` refs in iroh docs/README/DESIGN/CLAUDE (the
"transport" hits are the vault's own iroh-vs-Veilid layer map). (2) The discovery stale-*path* set was
`ROADMAP_TODO.md`, `thinking/app/client-architecture-adr.md`, and `thinking/app/README.md` (the last not
in the plan's inferred list); `COHESION.md` §23 and `local-first-as-design-imperative.md` carry no path,
so per "only the path goes stale" they needed no edit. Discovery edits were left **unstaged** (separate
repo with the user's own in-flight changes) for the user to fold into their commit. (3) Added
`croft-group/.gitignore` (build config, not itemized but necessary).
**Goal:** `croft-chat-cli` lives at `experiments/croft-group/crates/croft-chat-cli`, builds and tests
green in the new workspace; it is removed from the alt-drive workspace.
**Changes:**
- [ ] `git mv experiments/iroh/crates/croft-chat-cli experiments/croft-group/crates/croft-chat-cli`
  (preserves `transport.rs` + `tests/transport.rs` byte-identical — verify with `git status` showing
  renames, no content diff).
- [ ] Remove `crates/croft-chat-cli` from `experiments/iroh/Cargo.toml` members.
- [ ] `experiments/croft-group/Cargo.toml` — new workspace; `[workspace.package]` **must define every
  field croft-chat-cli inherits via `*.workspace = true`**: `version`, `edition` (2021),
  `rust-version` (1.75), `license` (`MIT OR Apache-2.0`), `authors` (`Chase Pettet <chase@owasp.org>`)
  — omitting any one fails resolution. Members `["crates/croft-chat-cli"]` (group-core added Phase 2).
- [ ] `experiments/croft-group/CLAUDE.md` — TDD/rust-enforcer discipline for the workspace. **Pass-3
  correction:** the alt-drive CLAUDE.md's absolute `@/Users/cpettet/git/chasemp/coding-agents/...`
  imports **do resolve on this machine** — that path is the real directory and `~/.claude/coding-agents`
  is a symlink to it (verified Pass-3). So this is *not* a "confirm it resolves" task; the reason to
  prefer a portable form (or `~`-relative) is cross-machine/cross-user portability, not breakage here.
  Drop the vault-specific content.
- [ ] **`[workspace.package]` `repository` field (Pass-3):** the alt-drive workspace carries
  `repository = "https://github.com/chasemp/alt-drive"`, but `croft-chat-cli` does **not** inherit it
  (it inherits only `version`/`edition`/`rust-version`/`license`/`authors` via `*.workspace = true` —
  verified). So the new `croft-group` `[workspace.package]` must define exactly those five and must
  **not** copy the stale alt-drive `repository`; either omit `repository` or set it to the experiments
  repo.
- [ ] **Cargo.lock (Pass-2):** removing the member dirties `experiments/iroh/Cargo.lock`
  (croft-chat-cli drops out) and the new workspace generates `experiments/croft-group/Cargo.lock` on
  first build. Commit both (org convention: commit lockfiles).
- [ ] **Stale-pointer refresh, scheduled here because the move makes them stale (Pass-3 fix for the
  docs-at-the-end anti-pattern):** the `git mv` invalidates every reference to
  `experiments/iroh/crates/croft-chat-cli` **the moment Phase 1 commits** — so the refresh belongs in
  Phase 1, not Phase 8. (a) **iroh-side**, in this repo: grep `experiments/iroh/docs/{roadmap.md,
  transport-layers.md}`, `experiments/iroh/README`/`DESIGN`, `experiments/iroh/CLAUDE.md` for
  `croft-chat-cli`/`croft-chat`/`Transport` and update/remove (chat CLI is leaving the vault workspace).
  (b) **discovery-repo** (separate repo → its own commit, with the chasemp identity): `COHESION.md`
  §23, `ROADMAP_TODO.md` E19, `thinking/app/client-architecture-adr.md`,
  `thinking/local-first-as-design-imperative.md` — repath `experiments/iroh/crates/croft-chat-cli` →
  `experiments/croft-group/crates/croft-chat-cli`. (c) **memory**: the `croft-cli-tdd-production`
  memory file — same path repath. Leaving any of these to Phase 8 means a P1 commit ships a tree whose
  pointers name a path that no longer exists.
**Call chain:** (structural phase) — the existing transport tests are the live entry point that must
keep passing through the move.
**Wiring test:** the **existing** `tests/transport.rs` (2 tests) must pass unchanged in the new
location: `cargo test -p croft-chat-cli` from `experiments/croft-group/`. This proves the move didn't
break the build/workspace inheritance. (No new test; the move is verified by the existing suite + a
byte-identical `git mv`.)
**Depends on:** nothing.
**Read-set:** `experiments/iroh/Cargo.toml`, `experiments/iroh/CLAUDE.md`,
`experiments/iroh/crates/croft-chat-cli/**`.
**Write-set:** `experiments/croft-group/Cargo.toml`, `experiments/croft-group/CLAUDE.md`,
`experiments/croft-group/crates/croft-chat-cli/**` (moved), `experiments/iroh/Cargo.toml` (member
removal); both `Cargo.lock`s; **iroh-side docs** (`experiments/iroh/docs/{roadmap,transport-layers}.md`,
README/DESIGN, `experiments/iroh/CLAUDE.md`); **discovery repo** (`COHESION.md`, `ROADMAP_TODO.md`, the
two `thinking/` docs — separate commit, chasemp identity); **memory** (`croft-cli-tdd-production`).
**Shared-state contract:** a `git mv` (touches the index, not HEAD across worktrees); single
worktree, no parallelism. No process/port/env state.
**Risks:** workspace-inheritance fields (`version.workspace` etc.) fail if the new
`[workspace.package]` omits one → `cargo` errors immediately. The alt-drive workspace must still build
after removal (it has no dep on croft-chat-cli — verified).
**Done when:** 1. **Behavioral:** `croft-chat-cli` builds and its transport tests pass from the new
`croft-group` workspace; the alt-drive workspace still builds without it. 2. **Verification:**
`cargo test -p croft-chat-cli` (in croft-group) green **and** `cargo build` (in iroh/alt-drive) green.
3. **Pointers refreshed:** a tree-wide grep for `experiments/iroh/crates/croft-chat-cli` returns no
stale hits in this repo or the discovery repo (run the grep after editing; silence = done), and the
`croft-cli-tdd-production` memory path is updated.
**Validation:** Moderate — run both workspaces' builds; confirm `git mv` produced renames, not
delete+add (provenance intact); grep-verify no stale path pointers remain.

### Phase 2: `group-core` crate scaffold + domain model — ✅ SHIPPED (`bb70e10`)
**Goal:** A new pure crate exists with the chat domain model; a fresh `Model` is empty and unjoined.
**Changes:**
- [ ] `experiments/croft-group/crates/group-core/Cargo.toml` (config) + add to workspace members.
- [ ] `crates/group-core/src/model.rs` — `ChatMessage { sender: String, text: String }` (the pond's
  native domain type — DECISION-2 analog) and `Model { messages: Vec<ChatMessage>, joined: bool }`.
- [ ] `crates/group-core/src/lib.rs` — `#![warn(missing_docs)]`, module wiring, re-exports.
**Call chain:** (foundation) — `Model` constructor exercised by the test; later phases' `update`
constructs/returns `Model`.
**Wiring test:** `group-core` test: a fresh `Model` has no messages and `joined == false`. RED first.
**Depends on:** Phase 1.
**Read-set:** `experiments/croft-app-phase0/crates/core/src/model.rs` (reference).
**Write-set:** `crates/group-core/{Cargo.toml, src/model.rs, src/lib.rs}`,
`experiments/croft-group/Cargo.toml` (members).
**Shared-state contract:** none beyond write-set.
**Risks:** member path typo (caught by `cargo build`).
**Done when:** 1. **Behavioral:** the workspace builds with the new crate; a fresh `Model` is
empty/unjoined. 2. **Verification:** `cargo test -p group-core` green.
**Validation:** Narrow.

### Phase 3: `intent` / `effect` enums + `update` join transition — ✅ SHIPPED (`b14ced2`)
**Goal:** `JoinGroup` joins and emits a `Subscribe` effect; a second `JoinGroup` is a no-op.
**Changes:**
- [ ] `crates/group-core/src/intent.rs` — `enum Intent { JoinGroup, SendMessage { text: String },
  FrameReceived { bytes: Vec<u8> } }`.
- [ ] `crates/group-core/src/effect.rs` — `enum Effect { Subscribe, Publish { bytes: Vec<u8> } }`.
  (Pass-3: a third variant `FrameDropped { reason }` is added additively in Phase 5 when the receive
  arm needs it — like `update.rs`, `effect.rs` is touched in both Phase 3 and Phase 5.)
- [ ] `crates/group-core/src/update.rs` — `pub fn update(Model, Intent) -> (Model, Vec<Effect>)` with
  the `JoinGroup` arm (fresh→joined + `[Subscribe]`; already-joined→no-op) and an exhaustive match
  (other intents no-op for now).
**Call chain:** shell (later) → `update(model, JoinGroup)` → `(joined, [Subscribe])`.
**Wiring test:** `update(fresh, JoinGroup)` → joined + `[Subscribe]`; `update(joined, JoinGroup)` →
unchanged + `[]`. RED first.
**Depends on:** Phase 2.
**Read-set:** `crates/group-core/src/model.rs`; phase-0 `core/{intent,effect,update}.rs` (reference).
**Write-set:** `crates/group-core/src/{intent.rs, effect.rs, update.rs, lib.rs}`.
**Shared-state contract:** none.
**Risks:** non-exhaustive match (compiler catches); no-op fall-through keeps later arms additive.
**Done when:** 1. **Behavioral:** joining a fresh group requests a subscribe; re-joining does nothing.
2. **Verification:** `cargo test -p group-core` green.
**Validation:** Narrow.

### Phase 4: `wire` — ChatMessage ↔ bytes round-trip (serde_json) — ✅ SHIPPED (`3b5f1d9`)
**Delivered:** as specified, with one design refinement — `ChatMessage` stays
serde-free (the plan's risk note floated "serde derive on `ChatMessage`"). The version
tag and serde derives live on a private `FrameV1` struct, so the domain type carries no
serialization/versioning knowledge (honest seam; keeps the door open for an alternate
wire later). `WireError` carries only parse position / version, never payload bytes.
**Goal:** A `ChatMessage` round-trips through bytes; malformed bytes fail as a typed error, not a panic.
**Changes:**
- [ ] `crates/group-core/src/wire.rs` — `serialize(&ChatMessage) -> Vec<u8>` /
  `deserialize(&[u8]) -> Result<ChatMessage, WireError>` (`thiserror`). serde_json with a version tag
  (`{ v: 1, sender, text }`) so L1/L2 fields extend it without breaking the wire.
- [ ] `crates/group-core/Cargo.toml` — add `serde` (derive) + `serde_json` (forced by the round-trip
  test).
- [ ] `crates/group-core/src/lib.rs` — re-export wire + `WireError`.
**Call chain:** `update` (Phase 5) → `wire::serialize` for `Publish`; `FrameReceived` → `update` →
`wire::deserialize`.
**Wiring test:** `deserialize(serialize(msg)) == Ok(msg)`; `deserialize(b"not-json")` →
`Err(WireError::..)`. RED first.
**Depends on:** Phase 2 (`ChatMessage`).
**Read-set:** `crates/group-core/src/model.rs`; phase-0 `bluesky/src/wire.rs` (reference).
**Write-set:** `crates/group-core/src/{wire.rs, lib.rs}`, `crates/group-core/Cargo.toml`.
**Shared-state contract:** none.
**Risks:** keep the format versioned from day one; serde derive on `ChatMessage`.
**Done when:** 1. **Behavioral:** a message round-trips; garbage fails cleanly. 2. **Verification:**
`cargo test -p group-core` green incl. round-trip + malformed.
**Validation:** Narrow.

### Phase 5: `update` send + receive transitions — ✅ SHIPPED (`46eb59a`)
**Delivered:** as specified (incl. the Pass-3 `FrameDropped` observability seam). Note:
the sender is a single hardcoded `LOCAL_SENDER = "alice"` placeholder in `update.rs` —
the advisory's "alice/bob" two-identity flavor is not reachable through the shipped
signatures (`Intent::SendMessage` carries no sender, `Model` no identity); real identity
is L1. Flagged in code.
**Goal:** `SendMessage` appends the sender's own message + emits `Publish { bytes }`; `FrameReceived`
appends the decoded message; a malformed frame is dropped without corrupting state.
**Changes:**
- [ ] `crates/group-core/src/effect.rs` — **add a third variant** `FrameDropped { reason: String }`
  (Pass-3). This is the observability seam for the malformed-frame case: the core is pure and
  WASM-clean (DECISION 1) so it **cannot `eprintln`** — it must emit the drop as *effect data* and let
  the shell perform the I/O. Keeps the match exhaustive; the shell's `perform_effect` (Phase 7) maps it
  to stderr.
- [ ] `crates/group-core/src/update.rs` — add `SendMessage { text }` (append own `ChatMessage` —
  no self-echo from the port, so optimistic append — + `[Publish { bytes: wire::serialize(&msg) }]`)
  and `FrameReceived { bytes }` (`wire::deserialize`; `Ok`→append the decoded message, no effect;
  `Err(e)`→**state byte-identical-unchanged + `[FrameDropped { reason: e.to_string() }]`** — drop, but
  *observably*). The core owns wire (the deserialize stays here, not in the shell), so the drop signal
  must leave the core as an effect — that is why the variant exists.
**Call chain:** shell → `update(model, SendMessage)` → `[Publish]`; shell drains a frame →
`update(model, FrameReceived)` → appended.
**Wiring test (mutation-resistant — name the edges, Pass-3):** RED first.
- `update(joined, SendMessage{"hi"})` → exactly one own-`ChatMessage` appended **and** exactly one
  `Publish` whose bytes `wire::deserialize` back to that same message (round-trip the emitted bytes,
  don't just assert the variant) **and** no `FrameDropped`.
- `update(joined, FrameReceived{serialize(remote)})` → `remote` appended, **zero** effects (a valid
  inbound frame is silent — assert the empty effect vec, so a stray effect fails the test).
- **Malformed boundary cases** (not just one `b"junk"`): each of `b""` (empty), `b"{"` (truncated/
  JSON-ish), and `b"\xff\xff"` (non-UTF8 garbage) → model **byte-identical unchanged** (assert the
  whole `Model`, not just `messages.len()`) **and** effects == `[FrameDropped { .. }]` (exactly one,
  carrying a non-empty reason) **and** no panic. The negative assertion (drop ⇒ *not* appended, valid ⇒
  *not* dropped) is what survives a one-line mutation that swaps the Ok/Err arms.
**Depends on:** Phases 3, 4.
**Read-set:** `crates/group-core/src/{model,intent,effect,wire}.rs`.
**Write-set:** `crates/group-core/src/{update.rs, effect.rs}` (effect.rs gains `FrameDropped`).
**Shared-state contract:** none.
**Risks:** hardcoded sender handle (confirmed placeholder); optimistic append relies on no-self-echo
(verified). **Pass-2 clarification (malformed frame):** a bad inbound frame is the **hostile/corrupt
sender** case — a peer can put any bytes on the wire — so `FrameReceived(junk)` must **drop, not
panic and not fatally propagate**: the receiver has to survive hostile input. This is *not* a
"fail loud" violation — fail-loud governs *our* bugs; surviving adversarial input is the
structural-enforcement / hostile-sender principle. **Pass-3 resolution of where the log emits:** the
core is pure/WASM-clean and must not do I/O, so "log + drop" splits across the seam — the **core**
emits `FrameDropped { reason }` (effect data, asserted in the core unit test) and the **shell** turns
that into a stderr line (Phase 7). That preserves DECISION 1 (core touches no I/O) while keeping the
drop observable, which is what makes a dropped-frame round-trip failure diagnosable (gate #2/#5).
**Done when:** 1. **Behavioral:** sending appends locally + emits a publishable frame; receiving a
valid frame appends silently; a bad frame leaves state byte-identical and emits an observable
`FrameDropped`. 2. **Verification:** `cargo test -p group-core` green (incl. the three malformed
boundary cases). **Validation:** Moderate — all arms incl. the error path with the drop-effect assertion.

### Phase 6: `project` / `view` — render-ready projection — ✅ SHIPPED (`b0ac1fd`)
**Goal:** The model projects to a platform-agnostic view model (ordered messages + joined/empty flag).
**Changes:**
- [ ] `crates/group-core/src/view.rs` — `ViewModel { joined: bool, lines: Vec<MessageLine> }`,
  `MessageLine { sender, text }`.
- [ ] `crates/group-core/src/project.rs` — `pub fn project(&Model) -> ViewModel` (pure; the only
  model→view conversion — honest seams).
- [ ] `crates/group-core/src/lib.rs` — re-export.
**Call chain:** shell `render` → `project(&model)` → `ViewModel` → platform render.
**Wiring test:** two-message model → two ordered `MessageLine`s + `joined == true`; empty unjoined
model → `joined == false`, no lines. RED first.
**Depends on:** Phases 2–5.
**Read-set:** `crates/group-core/src/model.rs`; phase-0 `core/{project,view}.rs` (reference).
**Write-set:** `crates/group-core/src/{view.rs, project.rs, lib.rs}`.
**Shared-state contract:** none.
**Risks:** keep `ViewModel` platform-agnostic (no CLI strings) for web/desktop reuse.
**Done when:** 1. **Behavioral:** a model renders to an ordered, platform-agnostic view model.
2. **Verification:** `cargo test -p group-core` green. `group-core` is now happy-path-complete and
transport-free (DECISION 1 holds).
**Validation:** Narrow.

### Phase 7: `croft-chat-cli` shell — effects + pump (THE WIRING PHASE) — ✅ SHIPPED (`f215384`)
**Delivered:** as specified. One manifest correction: the dep key is `group-core`
(the package name), not `group_core` — Rust still imports it as `group_core`. The
wiring test was mutation-checked (sabotaging `perform_effect`'s `Publish` arm makes
it fail), confirming it gates the wiring.
**Goal:** The shell drives `group-core` through the existing `Transport` port end-to-end: peer A's
`SendMessage` reaches peer B's model over the in-proc bus. **Make-or-break wiring proof.**
**Changes:**
- [ ] `crates/croft-chat-cli/src/effects.rs` — `perform_effect<T: Transport>(&Effect, &mut T, &Topic)`:
  `Subscribe`→`transport.subscribe`; `Publish{bytes}`→`transport.publish(topic, Frame::new(bytes))`;
  **`FrameDropped { reason }`→`eprintln!("croft-chat: dropped malformed frame: {reason}")`** (Pass-3 —
  this is the I/O side of the core's drop signal; stderr, matching phase-0's `eprintln!` convention, no
  log crate). The **only** place the port is called (DECISION 1). The match is **exhaustive over all
  three** effect variants. **Pass-2: returns `()`, not `Intent`** — unlike Phase-0's
  `perform_effect -> Intent`, these effects are fire-and-forget (the port's subscribe/publish have no
  return; `FrameDropped` is a log) so they produce **no follow-up intent**. Inbound arrives only via
  `pump`/`drain`, never as an effect-follow-up.
- [ ] `crates/croft-chat-cli/src/runtime.rs` — `apply<T: Transport>(model, intent, &mut T, &Topic) ->
  Model` and `pump<T: Transport>(model, &mut T, &Topic) -> Model`. **Pass-2: `apply` is NOT a
  drive-to-quiescence loop** (Phase-0's `drive` looped because each effect yielded a follow-up intent;
  here no effect does) — it is a **single pass**: `update` once, then perform each returned effect
  (side-effect only). `pump` is the confirmed explicit tick: `drain()` → for each frame, `apply(model,
  FrameReceived{bytes}, ...)`. **Topic is a shell-side constant** in the happy-path: the core's
  `Subscribe`/`Publish` effects are **topic-free** (the core stays transport-agnostic); the shell
  supplies the fixed `Topic`. That keeps DECISION 1 clean (the core knows nothing of topics/frames).
  **Pass-3 debugging readiness:** keep `pump`/`apply` free of always-on logging (phase-0's runtime is
  log-free; library code that tests call in a loop should not spew to stderr). Localize a failing
  round-trip via **test-side checkpoints**, not log spew — see the wiring test's intermediate
  assertions. The only stderr from the runtime path is the malformed-frame `FrameDropped` line, which
  fires only on hostile input.
- [ ] `crates/croft-chat-cli/src/lib.rs` — `pub mod {effects, runtime};`; add `group_core` dep as a
  **workspace path dep** (`group_core = { path = "../group-core" }`) in `croft-chat-cli/Cargo.toml`.
**Call chain:** test/binary → `apply(model, SendMessage, &mut A, &topic)` → update → `Publish` →
`perform_effect` → `A.publish`; then `pump(modelB, &mut B, &topic)` → `B.drain` → `FrameReceived` →
apply → update → appended.
**Wiring test:** `croft-chat-cli` integration test: one `InProcBus`, two peers, each a `group-core`
`Model`; both `apply(JoinGroup)`; A `apply(SendMessage{"hi"})`; B `pump`; assert **B's model contains
`ChatMessage{sender,"hi"}`** and A's contains its own copy. RED until effects+runtime exist; GREEN
proves the core is wired through the Transport port. **Pass-3 — intermediate checkpoints so a failure
localizes itself** (these are the debugging-readiness substitute for runtime logging): after A's send,
assert A's model has the message (update fired) **before** pumping B; after `B.pump`, assert the
happy-path emitted **no `FrameDropped`** (so a silently-corrupted frame can't masquerade as "not
received"). A failing assertion then points at the exact seam — update vs publish vs drain vs
deserialize. **Mutation checks:** (a) comment the `Publish` arm → B never receives (the canonical
wiring mutation); (b) make `perform_effect` ignore `Publish` → same failure, proving the effect is
performed, not just produced.
**Depends on:** Phase 6 + the moved `transport.rs`.
**Read-set:** `crates/croft-chat-cli/src/transport.rs`, all `crates/group-core/src/`, phase-0
`cli/{effects,runtime}.rs` (reference).
**Write-set:** `crates/croft-chat-cli/src/{effects.rs, runtime.rs, lib.rs}`,
`crates/croft-chat-cli/Cargo.toml`.
**Shared-state contract:** the `InProcBus` is shared *within a test* by design; no process/git/fs/env
state; tests build their own bus.
**Risks:** the pump is the novel piece — keep it the explicit tick (confirmed), not a loop.
**Done when:** 1. **Behavioral:** a message sent by one peer appears in another's model after a pump,
over the real `Transport` port. 2. **Verification:** `cargo test -p croft-chat-cli` green incl. the
two-peer round-trip.
**Validation:** Moderate — the integration test is the gate; verify it fails when the core is unwired.

### Phase 8: binary entry point — `croft-chat` scenario + render — ✅ SHIPPED (`e25b2cb`)
**Delivered:** as specified. The binary smoke test spawns the real `croft-chat` binary
(via `CARGO_BIN_EXE_croft-chat`) and asserts the message appears in *both* peers' views
(the receiver got it through the bus). `render`'s test landed with its impl rather than
strictly RED-first; compensated with a mutation check (dropping the sender fails it).
**Goal:** The `croft-chat` binary runs a deterministic two-peer demo and prints the rendered
conversation — the user-facing entry point is live.
**Changes:**
- [ ] `crates/croft-chat-cli/src/render.rs` — `render(&ViewModel) -> String` (text), mirroring
  `pond-cli/render.rs`.
- [ ] `crates/croft-chat-cli/src/main.rs` — replace the WIP stub with a scenario driver: build an
  `InProcBus`, attach two peers, join, send a scripted message, pump the other, print both views.
  Obvious arg parsing (`demo`, optional `--message`).
**Call chain:** `croft-chat demo` → `main` → bus/peers → `apply`/`pump` → `project` → `render` → stdout.
**Wiring test:** `render` test (a `ViewModel` renders expected lines) + a binary smoke check that the
entry point runs the round-trip and output contains the sent message. RED first.
**Depends on:** Phase 7.
**Read-set:** all `crates/croft-chat-cli/src/` + `crates/group-core/src/`; phase-0 `cli/{render,main}.rs`.
**Write-set:** `crates/croft-chat-cli/src/{render.rs, main.rs}`; plus the **one** Documentation Impact
item that belongs at the end because it *describes the now-built workspace* (not a stale-path fix):
`experiments/README.md` — add/enrich the `croft-group/` index entry (group-core + croft-chat-cli, now
real). **Pass-3:** the iroh cleanup, discovery-repo repath, and memory path update **moved to Phase 1**
(the phase that makes them stale); they are no longer Phase 8 work.
**Shared-state contract:** none beyond the test bus.
**Risks:** keep the binary deterministic (scripted, no network); render is cosmetic.
**Done when:** 1. **Behavioral:** `croft-chat demo` prints a conversation where the receiver's view
shows the sender's message. 2. **Verification:** `cargo test -p croft-chat-cli` green + manual run.
3. `experiments/README.md` carries the `croft-group/` entry (the stale-pointer refreshes already
   landed in Phase 1).
**Validation:** Moderate — run the binary, read output; render/smoke tests.

### Later phases (sequenced, NOT built in this slice — each gets its own plan)
- **L1 — Real identity:** hardcoded handle → real identity (DID/lineage) in `ChatMessage` + versioned
  wire. (May fold into L2.)
- **L2 — MLS / encryption:** `Frame` payload becomes MLS-ciphertext; key/epoch state enters the core
  (or a sibling crate); `Zeroize` applies.
- **L3 — Fork/merge + reconvergence-policy-per-plane:** multi-head DAG + per-plane reconvergence
  policy (declared at intent-to-collaborate, bound into the asset hash).
- **L4 — Governance / delegate planes:** threshold group-principal, capability-vs-authority delegates,
  the rights-floor.
- **L5 — Real-iroh Transport adapter:** a second `Transport` impl over iroh-gossip (mirrors Phase-0
  M6); real async runtime; the same scenario tests run against it. **This is where it goes live P2P.**
- **L6 — Shared-shell composition (option C unification):** shared shell crate hosting feed + group
  ponds (the Tauri/web surface); cross-pond **awareness** (read-only `at://` reference resolution);
  cross-pond **interactivity** stays deferred (the broker).

## Open Questions

All four reviewed and confirmed 2026-06-22:
- [CONFIRMED: PHASE-GATED (Phase 4)] **Wire format = `serde_json`** (version-tagged). *As recommended.*
- [CONFIRMED: PHASE-GATED (Phase 7)] **Pump = explicit synchronous tick.** *As recommended.*
- [CONFIRMED: structural] **Crate location = NEW dedicated `croft-group` workspace** (user **override**
  of the in-place recommendation). Adds Phase 1 (workspace + same-repo `git mv`).
- [CONFIRMED: ADVISORY] **Identity/topic = hardcoded placeholders** (alice/bob + fixed `Topic`),
  flagged in code. *As recommended.* Real identity = L1/L2.

No BLOCKING questions remain.

## Review Log

- **Pass 1 (2026-06-22):** Base plan authored, grounded in firsthand reads of `transport.rs`,
  `tests/transport.rs`, the alt-drive workspace manifest, and the Phase-0 `core`/`cli` sources. Mirror
  Phase 0; `group-core` a separate pure crate; Transport port stays in the shell (DECISION 1); the one
  delta is the inbound **pump**. Scoped to happy-path; richer concerns → L1–L6. No Phase 0 Discovery.
- **Pass 1 review / decisions (2026-06-22):** 4 open questions confirmed. Three as recommended
  (serde_json, explicit tick, hardcoded identity). **One override:** dedicated `croft-group` workspace
  instead of in-place. Verified the repo boundary (experiments/iroh is *not* a separate repo — same-repo
  `git mv`) and that `croft-chat-cli` has no alt-drive deps (clean extraction). **Added Phase 1**
  (create workspace + move), renumbered the former Phases 1–7 to 2–8, repathed all phases to
  `experiments/croft-group/`, and relocated this plan doc to `experiments/croft-group/plans/`. Updated
  Documentation Impact for the move (remove from alt-drive members, new workspace + CLAUDE.md, iroh
  doc cleanup, experiments index, discovery path refreshes, memory path). Concurrency unchanged (all
  sequential; Phase 1 precedes all).

### Pass 2: Gap Analysis — 2026-06-22
**Found:**
- **`apply` is not a quiescence loop, and `perform_effect` returns `()` not `Intent`.** Verified
  against Phase-0's `cli/{runtime,effects}.rs`: Phase-0 loops because every effect yields a follow-up
  intent; here `Subscribe`/`Publish` are fire-and-forget and `FrameReceived` yields no effect, so the
  shell is a single update+perform pass, with inbound arriving only via the separate `pump`/`drain`.
  Sharpened Phase 7.
- **`Topic` is a shell-side constant; the core's effects are topic-free.** Keeps the core
  transport-agnostic (DECISION 1). Added to Phase 7.
- **Malformed inbound frame = log + drop, not panic/fatal** (hostile-sender robustness, distinct from
  the fail-loud principle which governs our own bugs). Added to Phase 5 with a test assertion.
- **Cargo.lock handling on the move** (iroh lock loses the crate; croft-group gets a new lock; commit
  both) and the **exact `[workspace.package]` field set** croft-chat-cli inherits. Added to Phase 1.
- **CLAUDE.md `@`-import paths** in the alt-drive file are absolute and may not resolve here; the new
  workspace's CLAUDE.md must use a resolving form. Added to Phase 1.
- **Widened the move-grep scope** to `transport-layers.md` + `experiments/iroh/CLAUDE.md`. Added to
  Documentation Impact.
- **`group_core` is a workspace path dep** (`{ path = "../group-core" }`). Added to Phase 7.
**Concurrency:**
- No changes — map confirmed all-sequential. Audited the two nearest parallel candidates (Phases 3&4
  share `lib.rs`; Phases 3&5 share `update.rs`) — both collide, so no parallel set proposed. Recorded
  the disjointness audit in the Concurrency Map. No ambient (git/process/port/env) shared state beyond
  the per-test `InProcBus`.
**Changed:**
- Extended Phases 1, 5, 7 and the Concurrency Map + Documentation Impact with the above. No phases
  reordered; no reasoning rewritten.
**Confirmed:**
- DECISION 1 holds end-to-end: `group-core` depends on nothing transport-related; the port stays in the
  shell; the core emits topic-free effect data.
- The Transport port API (subscribe/publish/drain, no-self-echo, drain-consumes) maps cleanly to the
  effect set and the optimistic-append send path (no double-append, since no self-echo).
- The move is clean: `croft-chat-cli` has empty `[dependencies]`, no alt-drive crate depends on it,
  same-repo `git mv`.
- **4-file rule interpretation:** Cargo manifests *and* `lib.rs` re-export edits are module-wiring, not
  logic; under that reading no phase exceeds 3 logic files (Phase 3 = intent/effect/update; the lib
  re-export is wiring). Documented so it's a deliberate call, not an oversight — split enums-then-update
  during execution if Phase 3 feels large.
- No new open questions; no BLOCKING items. serde_json round-trip for `String`/`u8` fields is standard
  (no new external unknown introduced by the gap-fills).

### Pass 3: Quality Gates — 2026-06-22
Spot-checked the codebase first (fresh eyes): `transport.rs` API (subscribe idempotent / publish
no-self-echo / drain `mem::take`-consuming) confirmed verbatim; `croft-chat-cli` `[dependencies]` empty;
`[workspace.package]` inherited set = exactly `version`/`edition`/`rust-version`/`license`/`authors`
(**not** `repository`); phase-0 `cli/effects.rs` returns `Intent` (confirming the Pass-2 `()` delta);
phase-0 uses **`println!`/`eprintln!` only, no `tracing`/`log` crate**; the alt-drive CLAUDE.md's
absolute `@`-imports **resolve here** (`~/.claude/coding-agents` is a symlink to the real dir).

**TDD ordering:**
- Confirmed every phase is RED-first and test changes sit in the phase that adds the code.
- Added a **wiring-gate calibration note** under the Phases header: Phases 2–6 are legitimately
  component-level (no entry point exists until the shell), so the gate's "verify through the entry
  point" rule is satisfied at **Phase 7** (two-peer round-trip) and **Phase 8** (binary smoke) — stated
  explicitly so 2–6 aren't misread as plan defects.
- Sharpened Phase 5's malformed-frame test from one `b"junk"` single-point assertion to **three
  boundary cases** (`b""`, `b"{"`, non-UTF8) with whole-`Model` byte-identical assertion + negative
  assertions (drop ⇒ not appended, valid ⇒ not dropped) — mutation-resistant against an Ok/Err arm swap.
- Phase 7 wiring test gained a **second mutation check** (perform_effect ignores `Publish`) and
  intermediate checkpoints.

**Observability:**
- **Resolved a purity/observability tension Pass 2 left open:** "malformed frame = log + drop" cannot
  emit from the pure, WASM-clean `group-core` (DECISION 1 forbids core I/O). Split it across the seam —
  the core emits a new `Effect::FrameDropped { reason }` (effect data, asserted in the core unit test),
  and the shell's `perform_effect` turns it into a single `eprintln!` (stderr, matching phase-0's
  convention — **no log crate**, so "deps only when a test forces them" holds). Added the variant to
  Phase 3's enum note (added additively in Phase 5) and wired the shell arm in Phase 7.
- Kept `pump`/`apply` **log-free** (phase-0's runtime is log-free; library code tests call in a loop
  should not spew). The only runtime-path stderr is the hostile-input `FrameDropped` line.

**Debugging readiness:**
- In lieu of always-on runtime logging, Phase 7's wiring test now carries **intermediate checkpoints**
  (A's model has the message before B pumps; happy path emits no `FrameDropped`) so a failing
  round-trip localizes to the exact seam (update vs publish vs drain vs deserialize).
- Phase 1's "Done when" gained a grep-verification (no stale `experiments/iroh/crates/croft-chat-cli`
  pointers remain in either repo) — a concrete post-move health check.

**Validation calibration:**
- Reviewed every phase's strategy against scope. Calibration holds: Narrow for the pure-unit phases
  (2,3,4,6); Moderate for the move (1), the error-path (5), the wiring phase (7), and the binary (8).
  No change to labels; the higher-risk phases (1, 7) carry the strongest verification (dual-workspace
  build + git-mv-rename check; two-peer round-trip + dual mutation check).

**Concurrency honesty:**
- Map confirmed sequential. Refreshed the disjointness audit: Pass-3's `FrameDropped` addition makes
  `effect.rs` a second Phase 3↔5 collision (alongside `update.rs`) — does not change the all-sequential
  conclusion. No parallel set; no shared-state-invariant/re-entry checks needed (none exist). No ambient
  state beyond the per-test `InProcBus`.

**Coherence:**
- Plan still solves the stated problem (vertical happy-path slice proving core-through-Transport
  wiring); no scope creep — the `FrameDropped` effect is the minimum needed to make the already-scoped
  hostile-frame drop *observable*, not new behavior. L1–L6 untouched.

**Documentation impact:**
- **Fixed a docs-phase-at-the-end anti-pattern (gate #7).** The `git mv` lands in Phase 1, but the
  discovery-repo path refreshes (`COHESION.md`, `ROADMAP_TODO.md`, two `thinking/` docs) and the
  `croft-cli-tdd-production` memory path update were scheduled in **Phase 8** — leaving those pointers
  naming a non-existent path through a P1 commit and six subsequent phases. **Moved them into Phase 1**
  (the phase that makes them stale), updated Phase 1's Changes/Write-set/Done-when and the Documentation
  Impact section, and trimmed Phase 8 to the one doc item that genuinely belongs at the end (the
  `experiments/README.md` `croft-group/` index entry, which *describes the built workspace*).
- Recalibrated the Phase 1 CLAUDE.md note (imports resolve here → portability, not breakage, is the
  reason for a portable form) and added the `repository`-field caveat (don't copy the stale alt-drive
  URL; croft-chat-cli doesn't inherit it).

**Confirmed ready:** yes. All four open questions were confirmed by the user in the Pass-1 review
(serde_json, explicit tick, dedicated workspace, hardcoded identity); none are BLOCKING. No new open
questions introduced.

### Plan close-out — 2026-06-22
**Shipped:** the messaging happy-path slice, end to end, on the `croft-group-core-happy-path`
branch of `CroftCommunity/experiments` (8 phase commits + per-phase plan markers; not yet pushed).
Final git state: a new `experiments/croft-group/` workspace with two crates. `crates/group-core`
(`6f2cbca`→`b0ac1fd`) is a pure, transport-free, WASM-clean functional core —
`model`/`intent`/`effect`/`update`/`wire`/`project`/`view`, 10 unit tests; the wire is a
version-tagged serde_json frame whose `WireError` leaks no payload; the core emits topic-free
effect data including `FrameDropped` for observable drops. `crates/croft-chat-cli` (`f215384`,
`e25b2cb`) is the thin shell holding the `Transport` port + the deterministic `InProcBus` fake
(moved byte-identical from `iroh/` in `6f2cbca`): `perform_effect` (the only port caller),
`apply` (single pass) + `pump` (explicit synchronous tick), `render`, and the `croft-chat demo`
binary. 16 workspace tests green; the two-peer wiring test and the binary smoke test prove
core-through-Transport wiring (the make-or-break for option C), both mutation-checked. The
alt-drive workspace still builds without the chat CLI. Discovery-repo path pointers + the
`croft-cli-tdd-production` memory were refreshed to the new path; `experiments/README.md` carries
the new workspace entry; ROADMAP_TODO E19 updated to "execution started".
**Stopped or skipped:** nothing in scope. L1–L6 (real identity, MLS/encryption, fork/merge +
reconvergence-per-plane, governance/delegate planes, the real-iroh adapter, shared-shell
composition) were deliberately out of scope and remain sequenced in "Later phases" — each gets its
own plan. The single hardcoded `LOCAL_SENDER` placeholder means the advisory's "alice/bob"
two-identity flavor is not realized (the shipped `Intent`/`Model` signatures carry no identity);
that folds into L1. Discovery-repo edits were left **unstaged** for the user to commit (separate
repo, user's in-flight changes). The `coding-agents` hook fixes (below) are uncommitted in that
separate repo.
**Discoveries:**
- **The TDD guards were blind to Rust inline tests** and the Stop guard emitted schema-invalid JSON
  (`hookSpecificOutput` without `hookEventName`). Found mid-Phase-2 when the Stop guard blocked on
  `model.rs` (whose test is an inline `#[cfg(test)]` mod). Fixed all three shared hooks
  (`tdd-{stop,edit}-guard.sh`, `pre-commit-tdd-guard.sh`) to detect inline tests by content, and
  corrected the Stop output to top-level `decision`/`reason`. Then, at the user's direction,
  **moved TDD enforcement from real-time hooks to a commit-time gate**: removed the edit-guard +
  stop-guard from global `settings.json`, and installed a per-repo `.git/hooks/pre-commit` wrapper
  that runs the shared guard *and* `cargo test` for each changed sub-workspace (this repo has no
  root Cargo.toml). Phases 5–8 committed through that gate (it ran the suite each time). This was
  unplanned tooling work but was the precondition for clean per-phase commits.
- **`group_core` vs `group-core`:** the Cargo dep *key* must be the package name (`group-core`);
  Rust still imports it as `group_core`. The plan's `group_core = {...}` key was wrong; caught at
  Phase 7 compile.
- **`ChatMessage` is cleaner kept serde-free:** the version tag + serde derives live on a private
  `FrameV1`, so the domain type carries no wire knowledge — a better seam than the plan's floated
  "derive on ChatMessage."
- **Wiring held exactly as designed:** the pull-based `Transport` (no self-echo, drain-consumes)
  mapped onto optimistic-append-on-send + pump-on-receive with no surprises; the explicit-tick pump
  was the only structurally-new piece and it worked first try once the dep key was fixed.
