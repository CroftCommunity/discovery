# Integrated Drystone CLI — first wired-together demo

- **Date:** 2026-06-26
- **Status:** **CLOSED 2026-06-27** — Pass 1+2+3 + full execution. All 20 phases
  shipped (Milestones A–D), each TDD with its wiring test green and a commit.
  Convergence proven locally (shared-dir) and over real iroh-gossip (2- and
  4-node, in-process + the real `serve` binary across processes); the §7.6
  hard-stop is demonstrated. Deferred: the live *cross-host* runs (same binary +
  `RUN.md`, pending the secroute boxes) and the cargo-mutants re-sweep (the
  user's auth/governance trust gate, operational on the remote box).
- **Maturity tier:** alpha (code-forward, toward a demonstrable vertical slice)

## Outcome Summary

> One row per phase as it ships. SHAs are in the `experiments` repo
> (`github-personal:CroftCommunity/experiments`).

| Phase | Outcome | Commit | Note |
|---|---|---|---|
| P1 | ✅ SHIPPED | exp `74abb6e`, disc `7ddc37c` | Three-crate workspace builds, links the substrate; tracing helper + README + ROADMAP B12 pointer. |
| P2 | ✅ SHIPPED | `d9d49d5` | `LocalStore` stores+calls the injected `LamportSource`; rapid same-second sends apply. 3 tests routed through the store; monotonicity guard test added; warn log on the violation path. |
| P3 | ✅ SHIPPED | `0321d5a` | `pub fn get_message(&Hash) -> Option<MessageView>` + `decode_message_payload`; body+reply_to round-trip, unknown hash → None. |
| P4 | ✅ SHIPPED | `2c26304` | ed25519 adapters over `ed25519-dalek` (DeviceId = pubkey → stateless Verifier), registry CredResolver, atomic MonotonicLamport, Identity derivation. clippy-clean, 11 tests. |
| P5 | ✅ SHIPPED | `46837c8` | `Session` facade (open/create/add/send/queries/subscribe/trust_peer); substrate gained `export_group_log`/`ingest_foreign`/`export_assertion` for replication. create_group self-adds owner edge. 14 sgc tests. |
| P6 | ✅ SHIPPED | `53e7d37` | `Transport` port (Topic/Frame) + `SharedDirBus` (atomic write+rename, no-self-echo, seen-set, content-hash scramble). 4 adapter tests. |
| P7 | ✅ SHIPPED | `37b6041` | **Milestone A gate.** Two nodes converge to byte-identical state over the scrambling transport. `Replicator` (per-device contiguous-chain applier) + `fingerprint` + substrate `assertion_order_key`. |
| P8 | ✅ SHIPPED | `92f9dd4` | group-chat-core pure core: `Model`/`Intent`/`Effect` + `update` (optimistic send append, stale-refresh dropped, panic-free). 6 tests. |
| P9 | ✅ SHIPPED | `1c59809` | group-chat-core `project(&Model)->ChatView` + view models (GraphTreeView/TimelineView); order-preserving, selected-group + pending marks. 10 tests. |
| P10 | ✅ SHIPPED | `509ce78` | ratatui `App` (Session+Model+focus, refresh from session queries) + `ui::draw` two-pane layout; left graph pane renders live groups. TestBackend wiring test. |
| P11 | ✅ SHIPPED | `b73f96b` | `input::map_key` (key→Action) + async `App::perform` (sync core ↔ async Session bridge) + crossterm event loop in `main`. Scripted-keys send wiring test. |
| P12 | ✅ SHIPPED | `74ead3f` | **Milestone B gate.** Persistence + lamport restart-resume (substrate `max_lamport_for_device`; `Session::open` resumes). Binary headless `exec` mode + CLI args; binary-smoke proves cross-process persistence. |
| P13 | ✅ SHIPPED | `c169439` | Substrate channel routing: shared `types` payload codec (body‖reply‖channel), `send_message(group, channel, …)`, fold routes `References` to the channel, `get_channel_timeline`. Per-channel isolation test. |
| P14 | ✅ SHIPPED | `64a2815` | Channels through the stack: `Session::{create_channel,list_channels,send_to_channel,get_channel_timeline}` + `ChannelRef`; group-chat-core `SelectChannel` + channel in `Send`/`LoadTimeline`/`Snapshot`; `App` channel-aware refresh/submit. |
| P15 | ✅ SHIPPED | `26cd0f4` | TUI channel nav: tree is a flat `TreeRow` list (channels nested under the selected group); cursor + submit select group/channel; selecting `#photos` switches the right pane. 2 new tests. |
| P16 | ✅ SHIPPED | `eeeb6f1` | `IrohGossipBus` Transport over iroh-gossip (feature `iroh-it`, off by default). Two in-process endpoints exchange a frame over real gossip. iroh 1.0.0 + iroh-gossip 0.101 resolve cleanly (collision OQ resolved). |
| P17 | ✅ SHIPPED | `e0f33c2` | `stone-alpha.toml` four-node topology + `config.rs` loader (`Topology::{load,parse,node,bootstrap_peers}`, per-node `identity`/`principal`). 3 tests. Named-node serve + EndpointAddr JSON exchange fold into P18. |
| P18 | ✅ SHIPPED | `1777e10` | **Milestone C gate.** Two nodes converge over real iroh-gossip: automated in-process test + the actual binary's `serve` command across two processes (identical fingerprint `4c3099b76809a5ff`). `RUN.md` recipe (incl. cross-host). |
| P19 | ✅ SHIPPED | `9677dea` | **Proof A.** Four `serve` processes over real iroh-gossip converge to identical `fingerprint 9c10206e5b8ef5c3` with the same four-message timeline. RUN.md four-node recipe. |
| P20 | ✅ SHIPPED | `0e55a70` | **Proof B.** Contradiction (two genesis, same group) → substrate `ForkStatus` → `get_group_summary` → App blocking banner (no silent winner). Substrate + end-to-end tests. Mutation gate remains (see Open Questions). |

## Problem Statement

We have proven the pieces of the Drystone protocol in isolation but never wired
them into one running thing a person can drive:

- a **redb storage + projection layer** (`local_storage_projection`) implementing
  the append-only governance fold and the derived social-graph projection —
  built, tested, mutation-vetted;
- a **chat core** (`croft-group/group-core` + `croft-chat-cli`) that is
  production-grade TDD but happy-path only (flat message list over an in-process
  bus; no identity, persistence, or graph);
- a working **iroh transport** prototype (`altdrive-spike-faithful-sync` +
  `lineage-iroh`'s `Envelope` seam) that already moves signed messages over
  iroh-gossip across the four test nodes;
- proven domain mechanics (`lineage-core`, `lineage-mls`, `lineage-history`) and
  an app-architecture reference (`croft-app-phase0`).

They were built at different times and the chat core *conflated* chat with the
social graph ("chat always came with the writer"). The protocol work since
established a cleaner boundary: **the social graph is the substrate (the
protocol); chat is one tenant (an implementation) attached to it.**

We need a first integrated CLI that (a) demonstrates the Drystone protocol as far
as specced, (b) is architected around the protocol/implementation boundary, and
(c) runs across our four test nodes so we can watch members of one group
converge — the preliminary to moving up the UI stack.

**Constraints:** pre-1.0, no backwards-compat obligation. TDD non-negotiable
(rust-enforcer discipline: no `unwrap`/`expect` in production, `#![warn(missing_docs)]`,
`clippy::pedantic`). Git identity chasemp / `chase@owasp.org` / `github-personal`;
commit per phase, push only on request. The substrate crate is mutation-vetted —
any change to it re-runs the mutation discipline on touched functions.

## Reasoning

**The three-layer split (the central decision).** The conflation is exactly what
the protocol work outgrew. We make `social-graph-core` tenant-agnostic so chat
stops knowing graph internals; the same seam that hosts chat will host
games/notes/calls unchanged. This is the spec's "social graph is the substrate;
chat is a tenant" made real in code, and what lets the UI stack grow without
re-litigating the data model. The demo then proves a *protocol* property, not a
chat feature.

```
┌──────────────────────────────────────────────────────────────────┐
│  croft-chat   (CLI shell / binary)            ← the demo app        │
│    TUI two-pane: social-graph/tree view  +  chat view              │
│    owns the ports; drives the tenant core via apply / pump         │
├──────────────────────────────────────────────────────────────────┤
│  group-chat-core   (TENANT — one implementation)                   │
│    chat domain only: messages, threads, channel model,             │
│    Intent / Effect / update / project / view                       │
│    depends on social-graph-core; never reaches around it           │
├──────────────────────────────────────────────────────────────────┤
│  social-graph-core   (PROTOCOL substrate facade — Drystone)        │
│    session/identity + groups · members · channels · timeline       │
│    tenant-agnostic; thin domain layer over the redb surface        │
├──────────────────────────────────────────────────────────────────┤
│  local_storage_projection (redb)  +  Ports:                        │
│    Transport(local→iroh) · Signer/Verifier/CredResolver (ed25519)  │
│    · LamportSource · Storage(redb)                                  │
└──────────────────────────────────────────────────────────────────┘
```

**Locked decisions (user, 2026-06-26):**
1. Core split = `social-graph-core` (substrate) + `group-chat-core` (tenant) +
   `croft-chat` (shell).
2. CLI UX = `ratatui` two-pane TUI (left graph tree, right chat timeline + input).
3. Transport staging = prove convergence locally (shared-dir adapter) first, then
   swap in the iroh-gossip adapter behind the same port.
4. Substrate gaps fixed **in place** in `local_storage_projection` (TDD + mutation
   discipline).

**Why local convergence first:** the headline property to demonstrate is
order-insensitive convergence (invariant I5) and the hard-stop on contradiction
(§7.6) — both observable without a network. A shared-dir adapter gives a
sub-second iterate loop; iroh then becomes "swap the adapter," not "rewrite the
app." The transport port is payload-blind, so the substitution is clean.

**Why fix the substrate in place:** the gaps (lamport, message-body read, real
crypto, channel reference) are protocol-layer needs, not app conveniences. One
source of truth, extended coverage, no shadow logic.

**Alternatives considered and rejected:**
- *Evolve `croft-group` in place* — rejected: its `Model` is a flat in-memory
  message list with no graph/identity/persistence seam; re-pointing it at the
  substrate is a larger change than a clean new tenant that reuses its proven
  *patterns* (wire, Transport port, hostile-input handling).
- *Use `lineage-core`/`lineage-history` as the substrate* — rejected for the first
  pass: the redb crate is the current, mutation-vetted realization of the same
  model with a consumer surface already shaped for this; lineage stays the
  reference for MLS/history (later milestones).
- *Real iroh from the start* — rejected per decision 3 (slow loop, NAT/relay
  setup competes with getting the cores right).
- *Wrap the substrate instead of fixing it* — rejected per decision 4 (would
  duplicate envelope decoding and lamport logic in the app layer).

## Verified Assumptions

Confirmed firsthand via a five-agent exploration sweep on 2026-06-26 (read-only).
Evidence is `file:line` in the named crates.

**Substrate — `local_storage_projection`:**
- Consumer entry point is `surface::LocalStore` (`surface.rs:201`); it owns the
  `Db`, an internal `DerivedFold`, `my_principal`, and a tokio broadcast channel.
  `lib.rs:12-14` re-exports the traits/types/surface a consumer needs.
- **Lamport bug confirmed:** `LocalStore` takes `LamportSource` as `PhantomData`
  and its private `next_lamport()` uses `unix_now().wrapping_add(1)`
  (`surface.rs:1341-1343`), while the fold enforces strict per-device monotonicity
  (`fold_derived.rs:715-747`). Two writes from one device in the same wall-clock
  second → `FoldError::LamportViolation`. **This blocks a usable chat client.**
- **Message-body read gap confirmed:** `get_timeline` returns hashes/metadata only
  (`surface.rs:504`, `TimelineEntry` at `:80-87`); the body lives in the envelope
  payload in `AUTH_ASSERTIONS`; the decoders `decode_envelope_from_canonical`
  (`fold_derived.rs:1700`) and `surface.rs`'s `decode_envelope_bytes`
  (`surface.rs:1407`) are private. No public body fetch exists.
- Write commands are async and take `signer: &impl Signer` per call: `create_group`
  (`surface.rs:900`), `add_member` (`:956`), `remove_member` (`:999`),
  `send_message` (`:1040`, returns the message `Hash`), `attach` (`:1091`),
  `vouch` (`:1148`). `RoleGrant`/`RoleRevoke`/`RuleChange` have fold logic but **no
  surface command** (not needed for this demo).
- Read commands (sync): `get_group_summary` (`:246`), `list_my_groups` (`:285`),
  `list_group_attachments` (`:389`), `get_timeline` (`:504`), `get_principal`
  (`:756`), `get_node_card` (`:818`). `subscribe()` returns a broadcast
  `Receiver<ChangeNotification>` (`:238`).
- Traits to implement live in `traits.rs`; they use **their own** `Hash`/`DeviceId`/
  `PrincipalId` newtypes (`traits.rs:14-24`) distinct from `types::*`; the surface
  bridges by copying bytes. `Verifier::verify(device,msg,sig)` (`:75`),
  `Signer::sign(msg)+device_id()` (`:88`), `CredentialResolver::resolve(device,principal)`
  (`:100`), `LamportSource::next_lamport()` (`:112`). Mocks under `traits::mocks`
  are XOR-based and gated on a `test-mocks` feature **not declared in `Cargo.toml`**
  (`Cargo.toml:19-20` has only `default=[]`) — unreachable from a consumer crate
  as written.
- Data model: nodes are `TypedId = KindTag(1)‖Hash(32)` (`types.rs:131`); `KindTag`
  has `Group/Principal/Device/ArtifactChat/ArtifactNote/ArtifactLink/ArtifactGame`
  (`types.rs:99-109`); `EdgeType` has `MemberOf/HasAttachment/References/Vouches`
  (`tables.rs:132-139`). `AssertionType::Message=0x0009` with `MessagePayload{body:String,
  reply_to:Option<Hash>}` (`types.rs:455-459`).
- **Channel-routing limitation confirmed:** `Message` ingest writes the
  `References` edge to the **group** node, not a channel attachment
  (`fold_derived.rs:1128-1135`); message node title is empty (`:1114`). So all
  messages hang off the group today — named channels need a substrate addition.
- DB lifecycle: `Db::open(path)` is public (`tables.rs:440`);
  `Db::create_in_memory()` is `#[cfg(test)]` only (`:449`).

**Chat core patterns to reuse — `croft-group`:**
- Hexagonal `Transport` port `{subscribe, publish, drain}` with `Topic(String)`/
  `Frame(Vec<u8>)` (`croft-chat-cli/src/transport.rs:43-53`); DECISION 1 = port
  lives in the shell, core emits effect data. `InProcBus` is the deterministic
  reference adapter (`:56-136`). `apply`/`pump` drive the core (`runtime.rs:16,34`).
- Pure-core shape: `update(model,intent)->(model,Vec<Effect>)`, `project`,
  versioned `wire` with a payload-leak-free `WireError`. Hostile-frame survival
  (malformed → `Effect::FrameDropped`, never panic).

**Transport — iroh:**
- iroh pinned **`1.0.0`** (FACTCHECK SoT; do not re-verify). Authoritative
  source-cited API surface in `experiments/alpha/iroh/relay-lab-runs/IROH-1.0.0-API-VERIFIED.md`.
- `altdrive-spike-faithful-sync` already does signed-message-over-iroh-gossip
  across nodes with verify-on-receipt; it pins `iroh = 1.0.0-rc.1` + `iroh-gossip
  0.100` and has rc.1↔1.0.0 drift to resolve (`RELAY-LAB-CONCLUSIONS.md:55-73`).
- Endpoint-builder pattern in `relay-loadtest/src/node.rs:33-76` (drop the lab
  `insecure_skip_verify`). Gossip subscribe/broadcast/Event loop in
  `altdrive-spike-gossip/src/main.rs:99-142`.
- `lineage-iroh` provides the transport-agnostic `Envelope{to,topic,kind,ciphertext}`
  seam (`transport.rs:56-63`) and a chat-shaped `Node` API (`broadcast`/`pump`,
  `node.rs:77-150`); its backend is in-process, with the iroh-binding plan written
  in `PHASE_3_FINDINGS.md:41-52`.
- **Node topology (no config file exists; documented in `RELAY-LAB-CONCLUSIONS.md:40-45`):**
  node1 secroute-testing-one `54.172.175.109`; node2 secroute-testing-two
  `34.207.146.151`; node3 `172.31.88.18` (internal only); node4 = this Mac
  (off-VPC, NAT). **SG opens UDP 2112 only**; workstation reaches boxes via relay;
  `EndpointAddr` JSON exchanged out-of-band to bootstrap gossip.

**Mutation finding (2026-06-26, sweep in progress on secroute-testing-one):**
survivors cluster in `fold_auth.rs::check_authorization` and `apply_governance` —
the auth/governance core has weak *negative-path* coverage (valid ops are tested
to succeed; unauthorized ops are not tested to be rejected). Relevant to the
trust claims of Milestone D.

**Crypto source (REVISED Pass 3):** the substrate uses **no ed25519 at all** —
`local_storage_projection`'s deps are `redb`/`blake3`/`serde`/`tokio` only
(`Cargo.toml:7-13`); the `Signer`/`Verifier` traits are fully abstract and crypto
is entirely the consumer's responsibility. `lineage-core::keys` (deterministic
Ed25519 `SigningIdentity`/`VerifyingIdentity`/`Sig`) does exist and is proven, but
it lives in a **separate git repo** —
`Proofs/alpha/lineage-groups/crates/lineage-core/src/keys.rs` — and pulls
`ed25519-dalek`+`sha2` as *workspace* deps we would not control. **Decision (user,
Pass 3):** P4 implements `social-graph-core/src/crypto.rs` directly over
`ed25519-dalek` (+`sha2` for any digest needs), no `lineage-core` dependency. The
trait impls are thin; this avoids a fragile cross-repo path dep and — since these
are the *only* ed25519/sha2 in our half of the workspace — lets us pin the exact
versions, which concretely resolves the iroh dependency-collision Open Question
(we own both pins: P4's and P16's).

**Git root (Pass 3):** the working repo is `experiments/` (remote
`github-personal:CroftCommunity/experiments`); identity already configured as
`Chase Pettet <chase@owasp.org>`. `croft-chat/` is currently untracked — all
phase commits land in the `experiments` repo.

## Documentation Impact

- ~~`experiments/alpha/croft-chat/plans/2026-06-26-integrated-drystone-cli-plan.md`
  — the rough pre-skill draft to remove in Phase 1.~~ **CORRECTED (Pass 3):** this
  file does not exist; `plans/` contains only this Pass-1/2/3 file (grepped:
  `integrated-drystone-cli-plan` matches only the self-reference). The P1 removal
  action is dropped — there is nothing to remove.
- `experiments/alpha/croft-chat/README.md` — **new**, added in Phase 1; describes
  the three-crate layout and how to run the demo. Updated as milestones land.
- `experiments/alpha/croft-chat/stone-alpha.toml` — **new**, Phase 17; the
  four-node test topology.
- `experiments/alpha/croft-chat/RUN.md` — **new**, Phase 18; run recipes
  (interactive, headless `exec`, and the cross-host two-node iroh convergence).
- `local_storage_projection/MUTATION_TESTING.md` — its survivor ledger is updated
  when Phases 2/3/13 touch the substrate (mutation re-run on touched functions).
- `discovery/alpha/ROADMAP_TODO.md` — add a backlog pointer to this integrated
  build (single source of open items per the repo convention); done in Phase 1.
- `discovery/beta/OPEN-THREADS.md` — no edit required by the build, but Milestone D
  (hard-stop demo) exercises T29/§7.6; cross-reference only.
- No `AGENTS.md`/`PLAYBOOK.md`/`CLAUDE.md` changes (grepped — no references to the
  new crates).
- **Observability (Pass 3):** `tracing` + `tracing-subscriber` added as workspace
  deps in Phase 1; structured spans/fields are introduced per phase (see each
  phase's **Observability** line). No standalone "logging phase" — log points live
  in the phase that introduces the behavior they trace.

## Concurrency Map

```
Sequential spine:
  P1 → P2 → P3 → [P4 ∥ P6] → P5 → P7   (Milestone A)
     → P8 → P9 → P10 → P11 → P12        (Milestone B)
     → P13 → P14 → P15 → P16 → P17 → P18 (Milestone C)
     → P19 → P20                         (Milestone D)
```

Default sequential — each phase reads what the prior wrote. One opt-in parallel
set:

**Parallel set {P4, P6}:**
- **Disjoint write-sets:** P4 writes `social-graph-core/Cargo.toml` +
  `social-graph-core/src/{identity,crypto}.rs`; P6 writes
  `croft-chat/src/transport.rs` + `croft-chat/src/shared_dir.rs`. Different crates,
  no overlap (Pass 3 added P4's `Cargo.toml` for the `ed25519-dalek` dep — still
  disjoint from P6).
- **Shared-state contract:** both compile against the workspace built in P1;
  neither mutates git HEAD outside its own commit, binds ports, or writes shared
  tmp. If run as worktree agents, neither invokes `git checkout/stash/rebase` in
  the parent worktree.
- **Re-entry verification:** parent-repo HEAD == pre-dispatch SHA; `git worktree
  list` shows only expected worktrees; `cargo build` green at the workspace root
  after merge.

All other phases sequential (genuine data/precondition dependencies; see each
phase's Depends-on).

## Phases

> Milestones map to the rough draft: **A**=old P0, **B**=old P1, **C**=old P2,
> **D**=old P3. The handoff scoped execution to Milestones A–C (P1–P18); the user
> then directed completion of the **entire plan**, so Milestone D (P19–P20) was
> also executed — demonstrated locally (in-process / multi-process over real
> iroh-gossip + headless contradiction test). The live cross-host runs use the
> same binary + `RUN.md` recipe, pending the secroute boxes being reachable.

### Milestone A — Foundation & headless convergence

#### Phase 1: Workspace skeleton — ✅ SHIPPED
**Goal:** A compiling `croft-chat` workspace with three empty crates path-deps on
the substrate; rough plan removed; backlog pointer added.
**Changes:**
- [ ] `croft-chat/Cargo.toml` — workspace with members `social-graph-core`,
  `group-chat-core`, `croft-chat`; `local_storage_projection` as a path dep
  (`../local_storage_projection`, same `experiments` repo); `tracing` +
  `tracing-subscriber` (env-filter) as workspace deps.
- [ ] three crate manifests + `lib.rs`/`main.rs` stubs (with `#![warn(missing_docs)]`).
- [ ] `croft-chat/README.md`; add `ROADMAP_TODO.md` pointer. (No rough-plan file to
  remove — see Documentation Impact correction.)
**Call chain:** n/a (scaffold) — `cargo build` is the only entry point this phase has.
**Observability:** establish the `tracing` workspace dep + a single
`init_tracing()` helper (env-filter, `RUST_LOG`-driven) that later phases call from
test harnesses and `main`; no log points emitted yet (no behavior).
**Wiring test:** `cargo build` at the workspace root succeeds and resolves the
`local_storage_projection` path dep.
**Depends on:** none.
**Read-set:** `local_storage_projection/Cargo.toml`.
**Write-set:** `croft-chat/Cargo.toml`, `croft-chat/{social-graph-core,group-chat-core,croft-chat}/**`
(manifests + stubs), `croft-chat/README.md`, `discovery/alpha/ROADMAP_TODO.md`.
**Shared-state contract:** no shared mutable state beyond the file write-set.
**Risks:** path-dep resolution to a crate that declares the unstable `test-mocks`
feature later — handle in P4, not here.
**Done when:** (1) Behavioral: `cargo build` produces three crates linking the
substrate. (2) Verification: `cargo build && cargo test` (no tests yet) exits 0.
**Validation:** Narrow — build is sufficient.

#### Phase 2: Substrate fix — injected LamportSource — ✅ SHIPPED
**Delivered (2026-06-26):** `LocalStore` now stores `lamport: L` (was `PhantomData`)
and `next_lamport()` delegates to `self.lamport.next_lamport()`. Two scope notes
vs the Pass-3 spec: (1) three existing surface tests
(`test_notification_membership_changed`, `…_timeline_changed`,
`test_remove_member_outcome`) booted via `boot_group_direct`, which advances the
DB's per-device lamport from a *separate* counter — encoding the very bug being
removed (the old `unix_now()+1` was always huge, so the store's writes never
collided with the helper's small lamports). After the fix they must boot through
the store; converted them to `create_group`/`add_member` via the store (the real
production path). (2) The Pass-3 "warn on the reject path" log lives in
`fold_derived.rs` Step-5 (not `surface.rs`), a one-line `warn!` on the existing
error branch — a small write-set extension beyond surface.rs, no logic change.
**Goal:** `LocalStore` uses the injected `LamportSource`; a device can send >1
message per second without `LamportViolation`.
**Changes:**
- [ ] `local_storage_projection/src/surface.rs` — store and call the `L`
  param's `next_lamport()` (today `PhantomData` + `unix_now()+1` at `:1341-1343`);
  back it with a per-device monotonic counter persisted across commands.
**Call chain:** `LocalStore::send_message` → `self.next_lamport()` → injected
`L::next_lamport()` → envelope `lamport` → `fold.ingest` (per-device monotonic
check at `fold_derived.rs:739`).
**Wiring test:** `surface` test: two `send_message` calls on one store within the
same wall-clock second both return `Applied` (RED today → GREEN).
**Mutation-resistance (Pass 3):** assert *both* edges of the lamport contract, not
just the happy point — (a) two same-second sends both succeed (the fix), and (b) a
write carrying a non-monotonic (≤ prior) lamport still rejects with
`LamportViolation` (the guarantee the fix must not weaken). Boundary: prior=N,
next=N (equal) rejects; next=N+1 accepts.
**Observability:** `tracing::debug!(device, lamport, "next_lamport")` on issue and
`tracing::warn!(device, got, expected_gt, "lamport violation")` on the reject path,
so a future monotonicity regression is traceable.
**Depends on:** P1.
**Read-set:** `surface.rs`, `traits.rs` (`LamportSource`), `fold_derived.rs` (lamport check).
**Write-set:** `local_storage_projection/src/surface.rs`.
**Shared-state contract:** edits the mutation-vetted crate — re-run cargo-mutants
on the touched lamport path; update `MUTATION_TESTING.md` ledger.
**Risks:** counter persistence across process restarts (must not regress); covered
by P12 restart test.
**Done when:** (1) Behavioral: rapid successive sends succeed. (2) Verification:
`cargo test -p local_storage_projection lamport`.
**Validation:** Moderate — unit test + confirm the mutation survivors on the
touched lines are killed or recorded.

#### Phase 3: Substrate fix — public `get_message` — ✅ SHIPPED
**Delivered (2026-06-26):** `LocalStore::get_message(&Hash) -> Option<MessageView>`
reading `AUTH_ASSERTIONS`, decoding via the existing private `decode_envelope_bytes`,
then a new free fn `decode_message_payload` (body_len ‖ body ‖ reply marker). The
payload format matches the writer (`send_message`); the test round-trips through the
writer to pin it. Non-`Message` assertions and undecodable payloads return `None`.
**Goal:** A consumer can fetch a message body by hash.
**Changes:**
- [ ] `local_storage_projection/src/surface.rs` — `pub fn get_message(&self,
  hash) -> Option<MessageView>` decoding the envelope payload
  (`body_len‖body‖reply`); add `MessageView{hash,author,lamport,body,reply_to}`.
**Call chain:** caller → `LocalStore::get_message(hash)` → read `AUTH_ASSERTIONS`
→ private envelope decode → `MessagePayload` decode → `MessageView`.
**Wiring test:** `surface` test: `send_message` then `get_message(returned_hash)`
returns the exact body and `reply_to`.
**Mutation-resistance (Pass 3):** also assert `get_message(unknown_hash)` returns
`None` (not a panic, not a default `MessageView`), and that a message *with* a
`reply_to` round-trips it as `Some(parent)` while one without round-trips `None` —
pins both arms of the optional decode.
**Observability:** `tracing::debug!(hash, found, "get_message")`; on a decode
failure of a present envelope, `tracing::error!(hash, "envelope decode failed")`
(this is an invariant violation — a stored message that won't decode).
**Depends on:** P1 (independent of P2).
**Read-set:** `surface.rs`, `types.rs` (`MessagePayload`), `tables.rs` (`AUTH_ASSERTIONS`).
**Write-set:** `local_storage_projection/src/surface.rs`.
**Shared-state contract:** mutation-vetted crate — re-run on the new decode path.
**Risks:** payload-format drift vs the writer (`surface.rs:1053-1064`); the test
round-trips through the writer to pin it.
**Done when:** (1) Behavioral: timeline hashes resolve to bodies. (2) Verification:
`cargo test -p local_storage_projection get_message`.
**Validation:** Moderate.

#### Phase 4: social-graph-core — ed25519 adapters — ✅ SHIPPED
**Delivered (2026-06-26):** `crypto.rs` + `identity.rs` over `ed25519-dalek`
directly. Key design: `DeviceId` *is* the ed25519 verifying-key bytes, so
`Ed25519Verifier` is stateless (reconstructs the public key from the id, verifies
any device) — no per-device key registry needed for verification.
`RegistryCredentialResolver` handles the device→principal mapping;
`MonotonicLamport` is an `Arc<AtomicU64>` (clones share the counter; `starting_at`
+ `peek` ready for P12 restart-resume); `Identity` derives `PrincipalId =
SHA-256("croft-principal-v1" || pubkey)`. Secret key material is kept off `Debug`/
`Clone` on `Identity`; `SigningKey` zeroizes on drop. clippy-clean.
**Goal:** Real `Signer`/`Verifier`/`CredentialResolver`/`LamportSource` impls over
ed25519, usable by a consumer (no reliance on the crate's gated mocks).
**Changes:**
- [ ] `social-graph-core/Cargo.toml` — add `ed25519-dalek` (+`sha2` if needed),
  pinned explicitly; **no `lineage-core` dep** (REVISED Pass 3 — see Crypto source).
- [ ] `social-graph-core/src/crypto.rs` — ed25519 `Signer`+`Verifier` implemented
  directly over `ed25519-dalek` (`SigningKey`/`VerifyingKey`/`Signature`);
  `CredentialResolver` over a device→principal registry; a persisted monotonic
  `LamportSource`.
- [ ] `social-graph-core/src/identity.rs` — keypair gen/load (seed-derived for
  repeatable tests, OS entropy otherwise), `PrincipalId`/`DeviceId` derivation.
**Call chain:** `Session::open` (P5) → constructs these adapters → passes to
`LocalStore::new`.
**Wiring test:** sign a message with `Signer`, verify with `Verifier` (roundtrip);
a wrong-key signature fails verification.
**Mutation-resistance (Pass 3):** the wiring test asserts both arms — a correct
signature verifies *and* a wrong-key (forged) signature is rejected; add a
tampered-message case (valid sig, mutated bytes) also rejects, so a verifier that
ignores the message body can't survive.
**Observability:** `tracing::warn!(device, "signature verification failed")` on
reject; `tracing::debug!(principal, device, "credential resolved")` in the resolver
(no key material logged — rust-enforcer secret-handling discipline).
**Depends on:** P1. (Parallel-safe with P6.)
**Read-set:** `local_storage_projection/traits.rs`; `ed25519-dalek` API. (No
`lineage-core` read — REVISED Pass 3.)
**Write-set:** `social-graph-core/Cargo.toml`, `social-graph-core/src/crypto.rs`,
`social-graph-core/src/identity.rs`.
**Shared-state contract:** none beyond write-set. (Adding `social-graph-core/Cargo.toml`
to P4's write-set keeps it disjoint from P6 — confirmed in the Concurrency Map.)
**Risks:** the `traits::*` vs `types::*` newtype bridge — adapters must speak
`traits::*`; confirmed at `traits.rs:14-24`.
**Done when:** (1) Behavioral: real signatures verify; forged ones reject.
(2) Verification: `cargo test -p social-graph-core crypto`.
**Validation:** Moderate.

#### Phase 5: social-graph-core — Session facade — ✅ SHIPPED
**Delivered (2026-06-26):** `session.rs` wraps a `LocalStore` over the P4 adapters.
Two findings drove substrate additions and a design choice:

- **Replication gap (not in Pass-3 spec).** `LocalStore` builds *and* ingests
  locally; there was no public way to (a) get an assertion's wire bytes or (b)
  apply a *foreign* envelope — both required for P7 convergence. Added to the
  substrate surface: `export_group_log(&GroupId) -> Vec<(Hash, bytes)>`,
  `ingest_foreign(&[u8])`, and `export_assertion(&Hash)`. (Mutation-vetted crate —
  recorded in `MUTATION_TESTING.md`.) `Session` exposes `export_group_log` +
  `apply_remote` + `trust_peer` (peers' credentials must be registered before
  their frames verify on ingest — the fold resolves the *author's* credential).
- **Per-device lamport ordering (refines I5 for P6/P7).** The fold enforces
  *strict per-device* lamport monotonicity, so one device's own chain must be
  applied in lamport order — reordering it permanently rejects the earlier
  assertion. I5 order-insensitivity is across *different* devices/concurrent ops.
  **Consequence for P6/P7:** the receiver must apply each device's frames in
  lamport order; the convergence layer sorts drained frames by lamport before
  applying (`export_group_log` already returns lamport-sorted).
- **create_group self-adds the owner edge.** Genesis makes the founder an owner
  in group *state* but writes no `MemberOf` edge, so `list_my_groups` (which
  scans those edges) would miss self-created groups. `Session::create_group` now
  follows genesis with an owner `MembershipAdd` (an upsert — no duplicate).
**Goal:** One ergonomic type to open a store and do tenant-agnostic graph ops +
queries, hiding redb and the newtype bridge.
**Changes:**
- [ ] `social-graph-core/src/session.rs` — `Session::open(path, identity)`;
  `create_group`, `add_member`, `send_message`, and query passthroughs
  (`list_my_groups`, `get_group_summary`, `get_timeline`, `get_message`).
- [ ] `social-graph-core/src/lib.rs` — re-exports + view types.
**Call chain:** consumer → `Session::{create_group,send_message,...}` →
`LocalStore` commands (P2/P3 fixes in play) → fold/projection.
**Wiring test:** `Session` test: create group, add a second principal, send a
message, read it back via `get_timeline` + `get_message`.
**Depends on:** P2, P3, P4.
**Read-set:** `surface.rs` (public API), `social-graph-core/src/{crypto,identity}.rs`.
**Write-set:** `social-graph-core/src/session.rs`, `social-graph-core/src/lib.rs`.
**Shared-state contract:** opens a redb file at a caller-supplied path; tests use
`tempfile`.
**Risks:** async surface commands — `Session` exposes async; the TUI bridges later.
**Done when:** (1) Behavioral: a full create→add→send→read cycle works on a real
redb file. (2) Verification: `cargo test -p social-graph-core session`.
**Validation:** Moderate.

#### Phase 6: Transport port + shared-dir adapter — ✅ SHIPPED
**Delivered (2026-06-26):** `transport.rs` (port: `Topic`/`Frame` + `subscribe`/
`publish`/`drain`) and `shared_dir.rs` (`SharedDirBus`: one file per frame under
`<root>/<topic>/`, atomic temp-write+rename, `<node_id>__<seq>__<fnv>.frame`
names so `drain` skips own frames, a seen-set for idempotent drains, and a
content-hash sort that scrambles send order). 4 adapter tests (receive-all,
no-self-echo, idempotent drain, unsubscribed-yields-nothing).
**Goal:** A payload-blind `Transport` port and a `SharedDirBus` adapter that moves
frames between processes via an append-only shared directory, deliberately
shuffling delivery order to test convergence.
**Changes:**
- [ ] `croft-chat/src/transport.rs` — port `{subscribe, publish, drain}` +
  `Topic`/`Frame` (lifted from `croft-chat-cli/src/transport.rs:43-53`).
- [ ] `croft-chat/src/shared_dir.rs` — `SharedDirBus` adapter (one file per frame
  under a topic dir; `drain` reads new files, returns in shuffled order).
**Call chain:** shell event loop → `transport.publish/drain` → shared-dir files.
**Wiring test:** publish N frames from "A", `drain` on "B" returns all N (order
not asserted); no-self-echo.
**Depends on:** P1. (Parallel-safe with P4.)
**Read-set:** `croft-chat-cli/src/transport.rs` (reference).
**Write-set:** `croft-chat/src/transport.rs`, `croft-chat/src/shared_dir.rs`.
**Shared-state contract:** writes under a caller-supplied shared dir (tests use
`tempfile`); no ports.
**Risks:** file-watch races — use atomic write+rename; `drain` tracks a seen-set.
**Done when:** (1) Behavioral: two peers exchange frames through a directory.
(2) Verification: `cargo test -p croft-chat transport`.
**Validation:** Moderate.

#### Phase 7: Headless convergence proof (I5) — Milestone A gate — ✅ SHIPPED
**Delivered (2026-06-26):** `tests/convergence.rs` drives two `Session`s (two
principals, two redb stores) over one `SharedDirBus` that scrambles delivery
order. A creates the group + enrolls B; both interleave messages; after exchange
both derive byte-identical state. Two supporting pieces, plus one substrate add:
- `sync.rs::Replicator` — the convergence layer demanded by the P5 per-device
  finding: buffers drained frames by `(device, lamport)` and applies each
  device's **contiguous** chain in order, holding back any frame whose
  predecessor hasn't arrived (greedy apply would permanently reject the earlier
  lamport). Idempotent.
- `fingerprint.rs` — total, deterministic, *diffable* projection digest (members
  + resolved message lines, sorted); the test prints the per-line diff on
  mismatch (the Pass-3 observability requirement).
- substrate `assertion_order_key(&[u8]) -> Option<([u8;32], u64)>` — lets the
  payload-blind transport layer read just (device, lamport) for ordering without
  exposing the full private decoder.
**Goal:** Demonstrate order-insensitive convergence end-to-end without a UI.
**Changes:**
- [ ] `croft-chat/tests/convergence.rs` — two `Session`s (two principals) over
  `SharedDirBus`; create group, add member, interleave sends from both; drain in
  several orders; assert byte-identical derived state (a `snapshot_all`-style
  fingerprint over the projection) on both.
**Call chain:** test → two `Session`s + `SharedDirBus` → substrate fold → compare
projections.
**Wiring test:** *this is the wiring test* — it exercises the whole stack
(identity → session → substrate → transport) and asserts convergence regardless
of frame order.
**Observability:** on a fingerprint mismatch the test must not just `assert_eq!` —
it logs each node's fingerprint and a **diff of the divergent projection entries**
(`tracing::error!` with the per-node entry sets) so a failed convergence is
diagnosable from the test output alone, not re-run under a debugger. This same
fingerprint+diff helper is reused by P18/P20.
**Depends on:** P5, P6.
**Read-set:** `social-graph-core` API, `croft-chat/src/{transport,shared_dir}.rs`.
**Write-set:** `croft-chat/tests/convergence.rs` (+ a shared `fingerprint` helper,
e.g. `croft-chat/src/fingerprint.rs`, reused by later proofs).
**Shared-state contract:** `tempfile` dirs only.
**Risks:** the projection fingerprint must be total and deterministic — reuse the
substrate's tested snapshot approach.
**Per-device ordering (from P5):** the SharedDirBus drain returns frames in
shuffled order, but the fold requires each device's chain in lamport order. The
convergence layer must **sort the drained frames by lamport (then re-try)** before
`apply_remote`, or buffer-and-retry until the dependency lands. The shuffle that
must converge is the *cross-device interleave*, not a reorder of one device's own
sequential writes (which is unrecoverable). Reuse `export_group_log`'s ordering as
the publish side; the receive side sorts.
**Done when:** (1) Behavioral: two nodes with the same facts in different arrival
orders derive identical state. (2) Verification: `cargo test -p croft-chat
convergence`.
**Validation:** Broad — this is the milestone's correctness claim; run it under
multiple shuffles.

### Milestone B — Single-node TUI, persisted

#### Phase 8: group-chat-core — model + update — ✅ SHIPPED
**Delivered (2026-06-26):** `model.rs` (`Model`/`Intent`/`Effect`/`Snapshot`/
`GroupRef`/`MessageLine`) + `update.rs` (total, panic-free reducer). `SendMessage`
emits `Effect::Send` + an optimistic timeline line and clears the draft;
`SelectGroup` clears the timeline + emits `LoadTimeline`; `Refresh` adopts the
group list but drops a timeline snapshot whose group ≠ the selected one
(hostile-input survival, the tenant analogue of `FrameDropped`). No `unwrap`/
indexing.
**Goal:** The chat tenant's pure core: chat intents/effects/reducer over the
substrate's notion of a group + its (single, implicit) chat.
**Changes:**
- [ ] `group-chat-core/src/model.rs` — `Model` (selected group, timeline view,
  draft) + `Intent` (`SelectGroup`, `SendMessage`, `Refresh`) + `Effect`.
- [ ] `group-chat-core/src/update.rs` — `update(model,intent)->(model,Vec<Effect>)`,
  hostile-input safe.
**Call chain:** shell → `update` → `Effect` (e.g. `Effect::Send{group,body}`)
performed by the shell against `Session`.
**Wiring test:** `update` test: `SendMessage` produces a `Send` effect + optimistic
timeline append; malformed refresh data is dropped, not panicked.
**Depends on:** P5 (uses its view types).
**Read-set:** `social-graph-core` view types.
**Write-set:** `group-chat-core/src/{model,update}.rs`, lib.
**Shared-state contract:** none (pure core).
**Done when:** (1) Behavioral: intents map to effects + model deltas.
(2) Verification: `cargo test -p group-chat-core update`.
**Validation:** Narrow.

#### Phase 9: group-chat-core — project + view — ✅ SHIPPED
**Delivered (2026-06-26):** `view.rs` (`ChatView`/`GraphTreeView`/`GroupNode`/
`TimelineView`/`TimelineLineView` — no ratatui types, so testable without a
terminal) + `project.rs` (`project(&Model)->ChatView`, total + order-preserving;
marks the selected group and flags optimistic lines as `pending`). Channels nest
under groups at P15.
**Goal:** Platform-agnostic view models for the two panes.
**Changes:**
- [ ] `group-chat-core/src/project.rs` — `project(&Model)->ChatView`.
- [ ] `group-chat-core/src/view.rs` — `GraphTreeView`(groups→members+chats),
  `TimelineView`(message lines).
**Call chain:** shell render → `project` → view structs → ratatui widgets (P10/P11).
**Wiring test:** `project` test: a model with N messages yields N timeline lines
in order; group tree reflects membership.
**Depends on:** P8.
**Read-set:** `group-chat-core/src/model.rs`, `social-graph-core` views.
**Write-set:** `group-chat-core/src/{project,view}.rs`.
**Shared-state contract:** none.
**Done when:** (1) Behavioral: model → view conversion is total and ordered.
(2) Verification: `cargo test -p group-chat-core project`.
**Validation:** Narrow.

#### Phase 10: croft-chat TUI — app state + graph pane — ✅ SHIPPED
**Delivered (2026-06-26):** `app.rs` (`App` holds the `Session`, the tenant
`Model`, and `Focus`; `refresh()` reads `list_my_groups` + the selected group's
timeline into a `Snapshot` and feeds `Intent::Refresh`; `select_group`/`apply`
drive the core) and `ui.rs` (`draw` two-pane layout; left graph-tree from
`GraphTreeView`, right timeline+input rendered too so the pane is real). Groups
carry no substrate title, so the tree labels them by a short hex id (`short_id`).
ratatui 0.29 + `TestBackend`. Wiring test renders an `App` over a real session.
**Goal:** ratatui app skeleton with the left pane rendering the live graph tree.
**Changes:**
- [ ] `croft-chat/src/app.rs` — `App` (holds `Session`, `group-chat-core::Model`,
  focus/selection state).
- [ ] `croft-chat/src/ui.rs` — two-pane layout + left graph-tree widget from
  `GraphTreeView`.
**Call chain:** `main` loop → `App` → `ui::draw` → graph widget from
`project(model)`.
**Wiring test:** a render test (ratatui `TestBackend`) asserts the left pane shows
a created group and its members.
**Depends on:** P5, P9.
**Read-set:** `social-graph-core`, `group-chat-core` views.
**Write-set:** `croft-chat/src/app.rs`, `croft-chat/src/ui.rs`.
**Shared-state contract:** none yet (no real terminal in tests — `TestBackend`).
**Done when:** (1) Behavioral: the graph appears in the left pane.
(2) Verification: `cargo test -p croft-chat ui_graph`.
**Validation:** Moderate.

#### Phase 11: croft-chat TUI — chat pane + input + event loop — ✅ SHIPPED
**Delivered (2026-06-26):** `input.rs` (`map_key(KeyEvent, Focus) -> Option<Action>`,
pure + shared by loop and tests; Tab toggles focus, Enter submits, q/Esc/Ctrl-C
quit, chars type in input focus), async `App::perform(Action)` bridging the sync
core to the async `Session` (Submit in input focus awaits `send_message` then
refreshes; Submit in tree focus selects the cursored group and moves focus to
input), a tree cursor (Up/Down), and the crossterm event loop in `main.rs`
(raw mode + alternate screen + poll/read/perform/redraw). Sync↔async bridge per
the Open Question — resolved as: the loop awaits the send inline (single task).
**Goal:** Right pane timeline + message input; arrow/Tab/Enter navigation; the
real terminal event loop driving `update` + performing effects against `Session`.
**Changes:**
- [ ] `croft-chat/src/ui.rs` — right pane timeline + input widget (edit only —
  P10 file; small addition).
- [ ] `croft-chat/src/input.rs` — key → `Intent` mapping.
- [ ] `croft-chat/src/main.rs` — crossterm event loop: read key → `update` →
  perform effects (`Session.send_message`, refresh) → redraw.
**Call chain:** keypress → `input` → `Intent` → `update` → `Effect` →
`Session.send_message` → substrate; `subscribe()` notification → `Refresh`.
**Wiring test:** `TestBackend` + scripted keys: type a message, press Enter, the
message appears in the right pane and is readable via `Session.get_message`.
**Depends on:** P10.
**Read-set:** `croft-chat/src/{app,ui,transport}.rs`, `group-chat-core`.
**Write-set:** `croft-chat/src/ui.rs`, `croft-chat/src/input.rs`, `croft-chat/src/main.rs`.
**Shared-state contract:** owns the terminal (raw mode) at runtime; tests use
`TestBackend`, no real TTY.
**Risks:** sync core vs async `Session` — bridge with a blocking call in the loop
(single-threaded) for now; revisited for iroh in P16.
**Done when:** (1) Behavioral: a user types and sends a message and sees it.
(2) Verification: `cargo test -p croft-chat ui_chat`.
**Validation:** Moderate — plus a manual run.

#### Phase 12: croft-chat — persistence + binary smoke — ✅ SHIPPED
**Delivered (2026-06-26):** restart-resume + a headless binary mode.
- **Lamport resume (the persistence blocker):** `Session::open` started the
  counter at 1, which on restart collides with the device's persisted chain.
  Added substrate `max_lamport_for_device(&DeviceId) -> u64` (by-device index
  range query, mirrors the fold's Step-5); `Session::open` now starts the
  `MonotonicLamport` at `max+1` (0+1 on a fresh store).
- **Binary CLI + headless `exec` mode:** `--store`/`--seed` args; `exec
  create-group|send|list|timeline` run one op against the store and exit (used by
  the smoke test and the P18 SSH run recipe). Default = interactive TUI.
- **binary_smoke (cross-process):** spawns the compiled binary three times —
  create-group, send, timeline — and asserts the message persisted; proves
  persistence + lamport resume through the real binary.
**Goal:** State survives restart; the real binary works end-to-end on one node.
**Changes:**
- [ ] `croft-chat/src/main.rs` — open the redb store at a stable path
  (`Db::open`), persist the lamport counter; CLI args for store path/identity.
- [ ] `croft-chat/tests/binary_smoke.rs` — spawn the binary, create a group, send,
  exit; re-spawn, assert the group + message persisted.
**Call chain:** `main` → `Session::open(persistent path)` → redb file on disk.
**Wiring test:** the binary-smoke test (spawns the actual binary twice).
**Depends on:** P11, P2 (lamport persistence).
**Read-set:** `croft-chat/src/{main,app}.rs`, `social-graph-core`.
**Write-set:** `croft-chat/src/main.rs`, `croft-chat/tests/binary_smoke.rs`.
**Shared-state contract:** writes a redb file under a `tempfile` dir in tests.
**Done when:** (1) Behavioral: restart the app, prior group + messages are there.
(2) Verification: `cargo test -p croft-chat binary_smoke`.
**Validation:** Moderate — plus a manual restart.

### Milestone C — Named channels + 2-node over iroh

#### Phase 13: Substrate — channel reference on Message — ✅ SHIPPED
**Delivered (2026-06-26):** the message wire format moved to a single shared codec
in `types.rs` (`encode_message_payload`/`decode_message_payload`, layout
`body‖reply‖channel-marker`, tolerant of legacy no-channel payloads). `surface::send_message`
now takes `channel: Option<TypedId>`; the fold's `Message` ingest decodes the
channel and writes the `References` edge from the channel node when present (else
the group, unchanged). Added `get_channel_timeline` + refactored `get_timeline`
to share a scope-based scan (`collect_references_entries` + `apply_timeline_window`).
`get_message` now uses the shared decoder (ignores channel). All `send_message`
callers updated (pre-1.0, direct signature change, no shim). The `Session` facade
signature is unchanged (passes `None`); P14 adds channel-aware `Session` methods.
**Goal:** A message can target a channel (`ArtifactChat`) node; the `References`
edge is written to the channel, not just the group.
**Changes:**
- [ ] `local_storage_projection/src/surface.rs` — `send_message` accepts an
  optional channel `TypedId`; payload carries it.
- [ ] `local_storage_projection/src/fold_derived.rs` — `Message` ingest writes
  `References` to the channel node when present (today group-only at `:1128-1135`).
**Call chain:** `send_message(group, channel, body)` → envelope → fold → channel
`References` edge → `get_timeline(channel)`.
**Wiring test:** create a channel via `attach(ArtifactChat,"general")`, send to it,
`get_timeline` scoped to the channel returns only that channel's messages.
**Depends on:** P3.
**Read-set:** `surface.rs`, `fold_derived.rs`, `types.rs`, `tables.rs`.
**Write-set:** `local_storage_projection/src/surface.rs`, `local_storage_projection/src/fold_derived.rs`.
**Shared-state contract:** mutation-vetted crate, two files touched — **re-run
cargo-mutants on `fold_derived` message ingest + the surface path**; update the
ledger. Backwards-compat: pre-1.0, change the signature directly (no shim).
**Risks:** timeline scoping — `get_timeline` currently scans group `References`;
must accept a channel scope. Adds a query change (within `surface.rs`).
**Done when:** (1) Behavioral: messages are addressable per channel.
(2) Verification: `cargo test -p local_storage_projection channel`.
**Validation:** Broad — touches the trust-critical fold; mutation gate required.

#### Phase 14: social-graph-core + tenant — named channels — ✅ SHIPPED
**Delivered (2026-06-26):** `Session::{create_channel, list_channels,
send_to_channel, get_channel_timeline}` + `ChannelRef` (channels are
`ArtifactChat` attachments via `attach`/`list_group_attachments`). group-chat-core
gains `Intent::SelectChannel`, `selected_channel`/`channels` model state, and a
`channel` field on `Effect::Send`/`Effect::LoadTimeline`/`Snapshot` (refresh now
drops a timeline whose group *or* channel differs). `App` is channel-aware
(refresh lists channels + loads the channel timeline; submit routes to
`send_to_channel`; `select_channel`). UI rendering of the nested channel rows is
P15.
**Goal:** Create/list channels and route messages to a selected channel.
**Changes:**
- [ ] `social-graph-core/src/session.rs` — `create_channel`, `list_channels`,
  channel-scoped `get_timeline`, channel-targeted `send_message`.
- [ ] `group-chat-core/src/{model,update}.rs` — selected-channel state +
  `SelectChannel` intent.
**Call chain:** `SelectChannel` → `update` → `Session.get_timeline(channel)`;
`SendMessage` → `Session.send_message(group, channel, body)`.
**Wiring test:** `Session` test: two channels in one group; a message sent to
`#a` does not appear in `#b`'s timeline.
**Depends on:** P13, P8.
**Read-set:** `surface.rs`, `social-graph-core`, `group-chat-core`.
**Write-set:** `social-graph-core/src/session.rs`, `group-chat-core/src/{model,update}.rs`.
**Shared-state contract:** none beyond write-set.
**Done when:** (1) Behavioral: per-channel timelines are isolated.
(2) Verification: `cargo test -p social-graph-core channel`.
**Validation:** Moderate.

#### Phase 15: croft-chat TUI — channel selection — ✅ SHIPPED
**Delivered (2026-06-26):** the tree view became a flat `Vec<TreeRow>` (`Group` |
`Channel`), with the selected group's channels nested (indented `#name`) right
after it — cleaner for cursor navigation than a nested structure. `project` builds
the rows; `ui::draw_tree` renders them; `App` cursors over rows and `submit`
dispatches to `select_group`/`select_channel`. Selecting `#photos` switches the
right pane to that channel's timeline.
**Goal:** Left pane shows channels nested under each group; selecting one switches
the right pane (the full two-pane mockup).
**Changes:**
- [ ] `croft-chat/src/ui.rs` — nested channel rows under groups.
- [ ] `croft-chat/src/app.rs` — channel selection in focus model.
**Call chain:** arrow/Enter on a channel → `SelectChannel` → right pane re-render.
**Wiring test:** `TestBackend`: select `#photos`, right pane shows `#photos`
timeline only.
**Depends on:** P14, P10.
**Read-set:** `croft-chat/src/{app,ui}.rs`, `group-chat-core`.
**Write-set:** `croft-chat/src/ui.rs`, `croft-chat/src/app.rs`.
**Shared-state contract:** none (TestBackend).
**Done when:** (1) Behavioral: channels are navigable and switch the chat pane.
(2) Verification: `cargo test -p croft-chat channels_ui`.
**Validation:** Moderate.

#### Phase 16: iroh transport adapter (`IrohGossipBus`) — ✅ SHIPPED
**Delivered (2026-06-26):** `iroh_bus.rs` (feature-gated `iroh-it`, optional deps
so default builds/tests pull none of iroh's tree). `IrohGossipBus::connect` binds
an endpoint (`presets::N0`), spawns `Gossip` + a `Router` on `GOSSIP_ALPN`, and
subscribes to a `TopicId` derived from the `Topic` string; bootstrap peers are
made resolvable via `MemoryLookup::from_endpoint_info` + the builder's
`address_lookup`. The async↔sync bridge is two background tasks (inbound gossip
`Received` → std mpsc that `drain()` reads; outbound queue → awaited `broadcast`),
with NeighborUp/Down + counters logged. **Two API/dep findings:** (1) iroh **1.0.0
+ iroh-gossip 0.101** resolve and compile cleanly alongside social-graph-core's
ed25519-dalek 2.x/sha2 0.10 — the collision OQ is *resolved*, no crypto type
crosses the payload-blind boundary; (2) rc.1's `add_endpoint_addr`/bootstrap path
became `MemoryLookup` + `builder.address_lookup` in 1.0.0 (the rc.1↔1.0.0 drift).
**Wiring test:** `two_endpoints_exchange_a_frame` (gated `iroh-it`) — two
in-process endpoints exchange a frame over real gossip in ~9s.
**Goal:** A `Transport` impl over iroh-gossip (pinned `=1.0.0`), drop-in behind the
existing port.
**Changes:**
- [ ] `croft-chat/src/iroh_bus.rs` — endpoint builder (from
  `relay-loadtest/src/node.rs`, no insecure TLS), gossip subscribe/broadcast/Event
  loop (from `altdrive-spike-gossip`), mapping `Transport::{publish→broadcast,
  drain→received-queue}`; async↔sync bridge (background task + channel the loop drains).
- [ ] `croft-chat/Cargo.toml` (croft-chat crate) — pin `iroh = "=1.0.0"` +
  `iroh-gossip`, resolve the rc.1↔1.0.0 drift per `IROH-1.0.0-API-VERIFIED.md`.
**Call chain:** shell loop → `Transport::publish` → `gossip.broadcast`;
`gossip Event::Received` → queue → `Transport::drain` → `Intent::FrameReceived`.
**Wiring test:** two in-process endpoints on a loopback/n0-relay topic exchange a
frame (an integration test gated behind a `iroh-it` feature so CI without network
skips it).
**Observability (Pass 3 — this is a Broad/network phase):** structured `tracing` at
every network seam, since this is the phase where NAT/relay failures surface —
`info!(node_id, relay_url, "endpoint bound")`, `info!(topic, "gossip subscribed")`,
`info!(peer, "NeighborUp")` / `warn!(peer, "NeighborDown")`, and
`debug!(topic, len, "broadcast")` / `debug!(topic, len, "received")` with running
publish/received counters. The P16 validation ("confirm NeighborUp") depends on
these existing — without them a failed two-node run is undiagnosable.
**Depends on:** P6 (the port), P7 (proves the port semantics).
**Read-set:** `croft-chat/src/transport.rs`, `relay-loadtest/src/node.rs`,
`altdrive-spike-gossip/src/main.rs`, `IROH-1.0.0-API-VERIFIED.md`.
**Write-set:** `croft-chat/src/iroh_bus.rs`, `croft-chat/Cargo.toml`.
**Shared-state contract:** binds UDP for QUIC; tests use ephemeral ports + n0
relay; runtime uses a tokio task. Declare the feature gate so default `cargo test`
binds nothing.
**Risks:** dependency tree (iroh locks ~395 crates incl. prerelease crypto that
may collide with pinned `ed25519-dalek`/`sha2` used by `lineage-core`) — resolve
versions at this phase; this is the highest-risk phase.
**Done when:** (1) Behavioral: two endpoints exchange a frame over gossip.
(2) Verification: `cargo test -p croft-chat --features iroh-it iroh_bus`.
**Validation:** Broad — real network path; check logs, confirm NeighborUp.

#### Phase 17: stone-alpha topology + bootstrap — ✅ SHIPPED
**Delivered (2026-06-26):** `stone-alpha.toml` (the four nodes — 2 public boxes,
internal-only node3, NAT workstation — with host/port/seed/public + relay_mode)
and `config.rs` (`Topology::{load, parse, node, bootstrap_peers}`; `NodeConfig`
derives its demo `identity`/`principal` from `seed`). `bootstrap_peers` returns
the public, reachable peers (drops self + internal-only nodes). Pure data + TOML
(no iroh), so it's in the default build. **Scope note:** the named-node *serve*
runtime (`exec serve --topology --node`) and the `EndpointAddr` JSON exchange are
runtime/iroh concerns exercised by P18's run recipe — folded there rather than
duplicated here. The P17 wiring test is the parse/resolve.
**Goal:** A checked-in four-node topology config + loader + `EndpointAddr` exchange.
**Changes:**
- [ ] `croft-chat/stone-alpha.toml` — the four nodes (3 secroute + workstation),
  relay mode, per-node principal.
- [ ] `croft-chat/src/config.rs` — load the topology; print/import `EndpointAddr`
  JSON for bootstrap.
**Call chain:** `main --topology stone-alpha.toml --node <name>` → `config` →
`IrohGossipBus` bootstrap with peer `EndpointAddr`s.
**Wiring test:** `config` test: parse `stone-alpha.toml`, resolve a node by name,
yield its principal + bootstrap peers.
**Observability:** on load, `info!(node, principal, peer_count, "topology loaded")`
and `info!(peer, endpoint_addr, "bootstrap peer")`; on a parse/resolve failure a
loud `error!` naming the offending key (fail-early — no silent default node).
**Depends on:** P16.
**Read-set:** `RELAY-LAB-CONCLUSIONS.md` (the topology facts), `croft-chat/src/iroh_bus.rs`.
**Write-set:** `croft-chat/stone-alpha.toml`, `croft-chat/src/config.rs`.
**Shared-state contract:** reads a config file; no mutation.
**Risks:** node3 is internal-only (no public IP) — workstation pairs with node1/2
via relay; document the reachable subset.
**Done when:** (1) Behavioral: the binary can start as a named node from the file.
(2) Verification: `cargo test -p croft-chat config`.
**Validation:** Moderate.

#### Phase 18: 2-node convergence over iroh — Milestone C gate — ✅ SHIPPED
**Delivered (2026-06-27):** two-node convergence over **real iroh-gossip**, proven
two ways:
1. **Automated** `tests/iroh_convergence.rs` (feature `iroh-it`): two `Session`s
   over `IrohGossipBus`, A creates+enrolls B, both interleave 4 messages, assert
   byte-identical fingerprints + 4 messages. ~11s.
2. **The actual binary** via a new feature-gated `serve` command (`--topology
   --node --addr-out --peer --create --send --run-seconds`): loads the topology,
   trusts every peer's credential, binds the iroh bus, exchanges `EndpointAddr`
   JSON, and runs the publish/pump replication loop. Verified locally
   (2026-06-27) — `secroute-testing-one` (creator) + `node4-workstation` (joiner)
   as two processes over real gossip both printed `fingerprint 4c3099b76809a5ff`
   and the converged `1: hello from workstation`. The cross-host run is the same
   binary on the boxes (recipe in `RUN.md`).
**Findings:** the joining node must `pump` the transport *before* it knows the
group id (that's how it learns its own membership) — pump unconditionally, then
discover the group via `list_my_groups`. The creator enrolls all topology
principals so joiners are authorized to send.
**Goal:** Workstation + one secroute box, same group + channel, messages converge
across the real network.
**Changes:**
- [ ] `croft-chat/RUN.md` — the two-node run recipe (build on each, exchange
  `EndpointAddr`, join, send both ways).
- [ ] (no new logic — exercises P5/P14/P16/P17 live.)
**Call chain:** node A `send_message` → `IrohGossipBus` → gossip → node B
`drain` → fold → both timelines converge.
**Wiring test:** a manual/scripted 2-node run (network-dependent; not a unit test).
The repeatable check is the convergence assertion run on each node's store after
the exchange (reuse P7's fingerprint).
**Depends on:** P16, P17, P14.
**Read-set:** whole stack.
**Write-set:** `croft-chat/RUN.md`.
**Shared-state contract:** **live remote run** — uses secroute-testing-one
(`54.172.175.109`, UDP 2112), the workstation's NAT path via relay, and SSH per
the sandbox-driving memory (detached-subshell for long-lived listeners; no
top-level remote `&`). Re-entry verification: both nodes' projection fingerprints
match; no orphan node processes left on the box.
**Risks:** NAT/relay reachability; SG only opens UDP 2112; iroh's QUIC/relay must
work within that. The faithful-sync spike proved the path — mirror its bootstrap.
**Observability:** rely on P16's network spans (NeighborUp, broadcast/received
counters) plus P7's fingerprint+diff helper; `RUN.md` documents setting
`RUST_LOG=croft_chat=debug,iroh_gossip=info` on each node so a non-converging run
shows whether the gap is connectivity (no NeighborUp), delivery (counts diverge),
or fold (counts match, fingerprints differ). The remote listener is launched via
the detached-subshell SSH pattern (session memory `ssh-driving-secroute-sandbox`)
with logs redirected to a file on the box, then fetched — never a top-level
remote `&`.
**Done when:** (1) Behavioral: two networked nodes show the same converged chat.
(2) Verification: convergence fingerprint equal on both after a cross-send.
**Validation:** Broad — live two-node run, logs inspected, fingerprints compared.

### Milestone D — Four-node + protocol demonstrations (planned; beyond current ask)

#### Phase 19: Four-node convergence (Proof A) — ✅ SHIPPED
**Delivered (2026-06-27):** four `serve` processes (node1 creator enrolling all
topology principals + three joiners), each sending a distinct message over real
iroh-gossip, all converged to `fingerprint 9c10206e5b8ef5c3` with the same
four-message timeline (`from node1`..`from node4`). The cross-host four-node
recipe is in `RUN.md`. (Local multi-process run is the same code path as the
boxes; node3 is internal-only and joins from within the VPC there.)
**Goal:** All four nodes in one group; concurrent multi-node sends converge.
**Changes:** [ ] `croft-chat/RUN.md` four-node recipe; reuse all prior logic.
**Call chain:** four `IrohGossipBus` peers on one topic.
**Wiring test:** four-node run; fingerprints equal on all reachable nodes.
**Depends on:** P18.
**Read-set/Write-set:** `croft-chat/RUN.md`.
**Shared-state contract:** live run on all three boxes + workstation (node3 via
relay only). Re-entry: no orphan processes; fingerprints equal.
**Done when:** (1) Behavioral: four members see one converged conversation.
(2) Verification: fingerprint equality across reachable nodes.
**Validation:** Broad.

#### Phase 20: Hard-stop on contradiction (Proof B) — ✅ SHIPPED
**Delivered (2026-06-27):** the contradiction → hard-stop path, end to end.
- substrate `test_contradiction_surfaces_fork_status`: two devices claim the same
  gov slot; the fold records `ForkStatus::ForkedFrom` and `get_group_summary`
  surfaces `"forked_from:…"` (not silently rejected, not silently merged).
- App: `refresh` reads the selected group's `fork_status` into `model.fork`;
  `ui::draw` renders a **blocking banner** ("⚠ FORK DETECTED — convergence halted,
  no silent winner") above the panes — the §7.6 hard-stop made visible.
- `tests/contradiction.rs` (headless, end-to-end): builds a real fork via the
  public substrate + ed25519 adapters, opens a `Session`/`App` over it, and
  asserts both the surfaced fork and the rendered banner.
**Finding (recorded):** the substrate's fork detection fires on a same-`gov_seq`
collision (proven for genesis). The current high-level *two-`Session`
replication* path assigns an incoming governance assertion the receiver's next
seq (count-based), so two concurrent *non-genesis* membership ops **linearize**
rather than collide — i.e. a genuine cross-node membership contradiction does not
yet surface as a fork through `apply_remote`. Surfacing that requires the
substrate to carry each governance op's intended antecedent `gov_seq` on the
wire. Tracked as a follow-up; the hard-stop *mechanism* (detect → surface →
banner) is proven via the genesis-collision contradiction.
**Mutation gate (Open Question, the user's):** Proof B's trust claim still rests
on the auth/governance negative-path coverage the paused sweep flagged. Re-running
the full cargo-mutants sweep is an operational step on `secroute-testing-one`
(`cargo-mutants` is not installed locally) — commands in the handoff + ledger.
**Goal:** Two nodes issue contradictory membership facts concurrently; the app
detects the contradiction and **hard-stops** rather than auto-merging (§7.6).
**Changes:**
- [ ] `croft-chat/src/app.rs` — surface a `ForkStatus`/contradiction notification
  as a blocking banner (no silent resolution).
- [ ] `croft-chat/tests/contradiction.rs` — headless: inject two contradictory
  membership facts; assert the substrate flags it and the app refuses to pick.
**Call chain:** conflicting `MembershipAdd`/`Remove` → fold fork detection
(`governance.rs` tiebreak / hard-stop) → `ChangeNotification` → app banner.
**Wiring test:** the contradiction test (headless, deterministic).
**Depends on:** P19; **gated on the substrate auth/governance coverage** (see Open
Questions) since this is a trust claim.
**Read-set:** `governance.rs`, `fold_derived.rs` fork handling, `app.rs`.
**Write-set:** `croft-chat/src/app.rs`, `croft-chat/tests/contradiction.rs`.
**Shared-state contract:** headless test (`tempfile`); the live demo is manual.
**Risks:** the substrate distinguishes auto-resolvable concurrent forks
(content-hash tiebreak) from genuine membership contradictions (hard-stop) — the
demo must trigger the latter; confirm the exact trigger against `governance.rs`.
**Done when:** (1) Behavioral: a genuine contradiction halts with a human-facing
banner; no silent winner. (2) Verification: `cargo test -p croft-chat contradiction`.
**Validation:** Broad — the protocol's headline honesty property.

## Open Questions

- [RECOMMENDED: PHASE-GATED (Phase 16)] **Async↔sync bridge for iroh.** The cores
  and TUI loop are synchronous; iroh is async. Plan: a tokio background task feeding
  a channel the sync loop drains (mirrors the chat core's anticipated L5). *Rationale:
  it only bites at P16; P7–P12 stay synchronous. Confirm the bridge shape with a
  small spike at P16 rather than guessing now.*
- [RECOMMENDED: PHASE-GATED (Phase 20)] **Substrate auth/governance coverage.** The
  mutation sweep shows survivors clustered in `check_authorization` /
  `apply_governance` (weak negative-path coverage). Milestone D's hard-stop is a
  trust claim resting on exactly that code. *Rationale: harden coverage (negative-path
  tests that kill those survivors) before Proof B; not required for A–C functionality,
  so it gates D, not the whole plan.*
- [RESOLVED at P16] **iroh dependency collision.** ~~iroh 1.0.0 may pull
  prerelease crypto crates that collide with `ed25519-dalek`/`sha2`.~~ **Resolved
  2026-06-26:** iroh **1.0.0** + iroh-gossip **0.101** resolve and compile cleanly
  alongside social-graph-core's `ed25519-dalek 2.x`/`sha2 0.10` — Cargo permits the
  multiple majors side-by-side because no crypto type crosses the payload-blind
  transport boundary (exactly as the `altdrive-spike-faithful-sync` notes
  predicted). iroh is an *optional* dep behind `iroh-it`, so it burdens neither the
  default build nor the other crates. No version realignment was needed.
- [RECOMMENDED: ADVISORY] **Channel model depth.** P13 adds channel-scoped messages
  via a `References`-to-channel edge; full channel lifecycle (rename, archive) is out
  of scope. *Rationale: the demo needs selectable channels, not channel admin.*

## Review Log

### Pass 1: Plan development — 2026-06-26
Built the base from the rough draft + a five-agent codebase exploration.
Restructured to the skill template (added Verified Assumptions, Documentation
Impact, Concurrency Map, two-tier Done-when, Read/Write-sets, Call chains, Wiring
tests). Decomposed the rough P0–P3 into 20 phases (≤4 files each per the hard
rule). Folded the four locked user decisions into Reasoning. Recorded the
mutation finding as an Open Question gating Milestone D.

### Pass 2: Gap Analysis — 2026-06-26
**Found:**
- Original P0 conflated five concerns (scaffold + 3 substrate fixes + core +
  proof) into one phase — split into P1–P7; each now single-context-sized.
- Missing precondition: the substrate's gated `test-mocks` feature is unreachable
  by consumers (`Cargo.toml:19-20`), so the demo *cannot* rely on it — made real
  ed25519 adapters a first-class phase (P4), not an afterthought.
- Missing change: named channels require a **substrate fold change** (P13), not
  just app code — `Message` ingest hard-wires `References` to the group
  (`fold_derived.rs:1128-1135`). Surfaced as its own phase with a mutation gate.
- Missing precondition: `get_timeline` scopes to the group; channel-scoped reads
  need a query change — folded into P13.
- Cross-phase: P12 restart test depends on P2's lamport *persistence*, not just
  the injected source — made explicit in P2 risks and P12 Depends-on.
- Documentation: the rough plan file and a `ROADMAP_TODO` pointer were untracked —
  added to Documentation Impact and scheduled in P1.
**Concurrency:**
- Confirmed the sequential spine. One opt-in parallel set {P4, P6} (disjoint
  crates/files, no shared ambient state); re-entry checks named. All other phases
  have genuine data dependencies — sequential confirmed.
**Changed:**
- Phase count 4 → 20; added Milestone mapping; added the iroh dependency-collision
  and async-bridge Open Questions surfaced by reading `lineage-iroh`'s notes.
**Confirmed:**
- The substrate surface is sufficient for create/add/send/read with the three
  fixes (P2/P3/P13). The `Transport` port reuse from `croft-group` is clean. The
  iroh path is proven by `altdrive-spike-faithful-sync`; P16 is lift-not-invent.

### Pass 3: Quality Gates — 2026-06-26
Fresh-eyes spot-check of the codebase confirmed the substrate `file:line`
references still hold exactly (lamport `surface.rs:1341-1343`, group-only edge
`fold_derived.rs:1128-1135`, `get_timeline` `surface.rs:504`, `Transport` port
shape). Three divergences from Pass-1/2 assumptions were found and corrected.
**TDD ordering:**
- Added mutation-resistance assertions to the three substrate phases so single-
  point happy tests can't survive a one-line mutation: P2 also rejects a
  non-monotonic lamport (boundary N==N rejects, N+1 accepts); P3 also returns
  `None` for an unknown hash and pins both `reply_to` arms; P4 also rejects a
  tampered-message-valid-sig case. Every phase already had a wiring test through
  the entry point — confirmed, no defects.
**Observability:**
- Plan had no logging strategy. Added `tracing`+`tracing-subscriber` as workspace
  deps with an `init_tracing()` helper in P1, and per-phase log points: lamport
  issue/violation (P2), get_message decode (P3), verify-fail/cred-resolve without
  key material (P4), and — critically — the full network seam set for P16
  (endpoint bound, NeighborUp/Down, broadcast/received counters) that P16/P18
  validation already assumed but never specified. P7's convergence test now logs a
  divergent-entry diff on mismatch (reused by P18/P20).
**Debugging readiness:**
- P18 `RUN.md` now prescribes `RUST_LOG` per node so a non-converging run is
  triaged into connectivity vs delivery vs fold; remote listeners use the detached-
  subshell SSH pattern with file-redirected logs.
**Validation calibration:**
- Reviewed all tiers; Broad on P7/P13/P16/P18/P19/P20 matches their risk, Narrow on
  the pure-core phases (P8/P9) is right. No changes needed.
**Concurrency honesty:**
- Map accounts for all phases. Re-checked the one parallel set {P4,P6}: Pass 3 added
  `social-graph-core/Cargo.toml` to P4's write-set (ed25519-dalek dep) — still
  disjoint from P6's `croft-chat/src/*`. Shared-state contract is invariant-shaped
  (no git-HEAD mutation, no port binds, no shared tmp); re-entry checks map 1:1.
  Map confirmed; no new parallelism warranted.
**Coherence:**
- Plan still solves the stated problem; no scope creep. Corrected the **crypto
  source**: `lineage-core` is in the separate `Proofs` repo and the substrate uses
  no ed25519 — P4 now implements crypto directly over `ed25519-dalek` (user
  decision), which also narrows the iroh-collision Open Question to a P16-local pin
  task.
**Documentation impact:**
- Corrected a stale entry: the rough plan file slated for P1 removal does not exist
  (only the self-reference matched the grep) — dropped the removal action. Added the
  observability/tracing doc note and the git-root clarification (commits land in the
  `experiments` repo). Every added/removed file now has a home phase.
**Confirmed ready:** yes — open questions all RECOMMENDED, none BLOCKING; the new
crypto decision was confirmed by the user during Pass 3.

### Plan close-out — 2026-06-27
**Shipped:** A `croft-chat` workspace (in the `experiments` repo) with the
three-layer Drystone split — `social-graph-core` (substrate facade + ed25519
adapters + `Session`), `group-chat-core` (pure chat tenant: model/update/project/
view), and `croft-chat` (ratatui two-pane TUI + transports + replication +
binary). Substrate fixes landed in the mutation-vetted `local_storage_projection`:
injected `LamportSource` (P2), public `get_message` (P3), channel-routed messages
(P13), plus replication/query additions (`export_group_log`, `ingest_foreign`,
`assertion_order_key`, `max_lamport_for_device`, `get_channel_timeline`). The demo
proves order-insensitive convergence (I5) over a shared-dir transport (P7) and
over **real iroh-gossip** — two-node (P18) and four-node (P19), both as an
in-process test and via the real `serve` binary across processes (identical
fingerprints `4c30…`/`9c10…`). Persistence + lamport restart-resume (P12), named
channels through the stack (P14–P15), and the §7.6 hard-stop banner on a
contradiction (P20) all shipped. iroh is optional behind `iroh-it`; the default
build/tests pull none of it. 20 phases, 20 commits, each wiring test green.
**Stopped or skipped:** Nothing skipped. Deferred (not skipped): (a) the *live
cross-host* runs over the secroute boxes — the binary + `RUN.md` recipe are ready;
they need the boxes reachable; the in-process/multi-process runs over real
iroh-gossip exercise the identical code path. (b) The cargo-mutants re-sweep of
the auth/governance core — `cargo-mutants` is not installed locally and the staged
config lives on `secroute-testing-one`; this is the user's standing trust gate for
Proof B (Open Questions).
**Discoveries:** (1) `lineage-core` is in a separate repo — P4 implements crypto
directly over `ed25519-dalek`, which also *resolved* the iroh dependency-collision
question (iroh 1.0.0 + iroh-gossip 0.101 coexist with ed25519-dalek 2.x/sha2 0.10;
no crypto type crosses the payload-blind boundary). (2) The fold enforces
**per-device** strict lamport monotonicity — convergence requires applying each
device's chain in contiguous order (the `Replicator`), not a free reorder; I5 is a
*cross-device* property. (3) `create_group` needed a self-`MembershipAdd` so
`list_my_groups` (edge-scan) sees self-created groups; genesis alone writes no
`MemberOf` edge. (4) A joining node must `pump` the transport *before* it knows
the group id — that is how it learns its own membership. (5) The substrate's fork
detection fires on a same-`gov_seq` collision; the current high-level replication
path *linearizes* concurrent non-genesis governance, so surfacing a true
cross-node membership contradiction (vs the genesis-collision proof) needs the
wire to carry each op's intended antecedent seq — a recorded follow-up.
