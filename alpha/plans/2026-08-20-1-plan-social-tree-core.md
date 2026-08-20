# social-tree-core: the portable substrate, the chat tenant on it, and the client on both

- **Status:** Draft (2026-08-20), phases scoped, four open questions for the owner (§6). Not yet
  started; Phase 1 is the already-queued E108 build, absorbed here unchanged.
- **Origin:** owner direction 2026-08-20 — *"build up a phased plan on the social-tree-core of this
  and then have chat built on top of it right after and then build out the current chat client as
  well"* — closing the loop the client-architecture ADR left open (COHESION §23: adoption is
  "greenfield growth, not a refactor"; "plan not yet drafted — the user's next-step call", E19).
- **Existing code this plan builds from (all current, none discarded):**
  `alpha/experiments/local_storage_projection/` (the substrate: governance fold + derived
  projection + head-currency + horizon; mutation-vetted; carried the C-series, green 2026-08-17);
  `alpha/experiments/croft-chat/` (social-graph-core facade, group-chat-core tenant, croft-chat
  TUI shell); `alpha/experiments/croft-group/` (the June E19 lineage: `group-core` is the
  ADR-correct pond template — pure, WASM-clean, `model/intent/effect/update/wire/project/view` —
  but protocol-shallow, predating the current fold; harvested for shape, superseded for
  behavior); `alpha/experiments/meer-queue/` (the MLS half at Rung A: token, ledger, doors,
  admission fact — S23–S26); `Proofs/lineage-groups/crates/conformance` (the §9 vectors, 66/0).
- **Spec anchors:** Part 2 §7.3 (fold), §7.3.2 (`CONTESTED`), §10.2.2 (A-series admission
  interface), §11.7/§11.8 (re-entry + standing), §4.6 ("free in local storage"), §7.3.3 (snapshot
  is a cache); `implementation-profile.md` (the Croft reference profile the core must satisfy).
- **Related rows:** E108 (Phase 1), E19 (discharged by Phases 5–6), E112 (this plan takes the
  real-signatures residual as Phase 3; leaves HeadAck-over-real-transport, serve-signature
  analysis, door-A, lapse tests in E112), E116 (client-side renderings, partially landed by
  Phase 6).

---

## 1. Problem statement

Three true things do not yet compose:

1. The **client architecture is decided** (ADR 2026-06-22): one shared functional core + thin
   per-platform shells; per-pond domain cores; the core pure — no I/O, no async, no clock —
   WASM-clean. The croft repo implements the skeleton (`core/call-core`, `core/feed-core`,
   `ports/`, shells) but the **social/group pond — the backbone — has no core crate**.
2. The **Drystone substrate exists and is current** — `local_storage_projection` implements the
   canonical fold through the step-5 merge and is the code the C-series gate ran on — but it is
   **cut wrong for the core contract**: redb lives inside the crate (`FoldError:
   From<redb::CommitError>`, `tables.rs`), so the crate is not WASM-clean even though the fold
   logic in substance is, and the governance plane runs at Modeled rung (signature stand-ins).
3. The **chat stack above it is shaped right but points at the wrong layer**: `group-chat-core`
   already speaks the pond contract (Intent/Effect/update/project/view), but depends on the
   experiment-cut substrate, and the TUI client is a demo harness, not a client.

Stated as lineages: **croft-group** (June, E19) got the *architecture* right and the protocol
has since moved past it; **croft-chat + local_storage_projection** (current) has the *protocol*
right and the architecture cut wrong. The owner's suspicion — "Drystone has changed enough that
we would need a new version of the social tree core" — is true of the June crate specifically;
the new version's behavior already exists in the current substrate. This plan is the merge of
the two lineages, not a third rewrite.

The social tree — groups, membership, standing — is the backbone; chat is one tenant; call and
feed are others. Until the backbone exists as a portable core, every tenant either re-implements
protocol behavior or couples to an experiment crate that cannot ship in a shell.

## 2. Approach

Grow the core out of the vetted experiment code by **re-cutting crate boundaries, not
rewriting**, in six phases, each leaving the whole corpus green. Order: close the known canonical
delta first (E108 — cutting a crate around a stale schema copies debt), then extract the pure
core, then raise the governance plane to real signatures, then join the MLS half behind a port,
then move the chat tenant onto the core, then build the client out. TDD RED-first throughout;
commit green before any hand-mutation; the C-series arms are the standing regression gate for
every phase that touches the fold.

## 3. Reasoning

- **Why extraction, not rewrite:** the substrate is not stale — it evolved with the spec and
  carries the C-series evidence. A rewrite discards mutation-vetted code to reproduce behavior
  we would then have to re-prove. The known gaps are enumerated (E108, rungs, the join), which
  is what makes extraction plannable at all.
- **Why the core never sees storage:** the ADR's contract ("ports held by the shell, never
  called by a core") is stronger than storage-behind-a-trait, and the spec licenses it: §4.6
  makes local storage free (never the interop surface) and §7.3.3 makes the projection a cache
  (truncation verifiable) — so a platform store's job is availability, not truth. redb serves
  every native shell; the browser gets its own adapter behind the same port (a redb-over-OPFS
  probe is noted, deferred, not load-bearing).
- **Why signatures before the join:** a product core cannot rest on MAC stand-ins, and the
  Verifier trait boundary already exists (`traits.rs`); doing this before the MLS join keeps the
  join phase's surface honest (one rung claim per plane, no mixed-grade composite claims —
  experiment-verdict-hygiene).
- **Why chat immediately after the core:** the tenant is the proof the core's surface is
  sufficient — a core with no consuming pond is an untested API. Chat is the cheapest real
  tenant (it exists) and the owner's named next use case.

## 4. The phases

**Phase 1 — E108: `CONTESTED` in the substrate (the queued front, absorbed unchanged).**
`fold_derived` gains the §7.3.2 member-view state: the contradiction artifact becomes
**set-valued and pair-carrying** (each entry holds both conflicting facts as data plus the
contested subject; the current `ForkStatus::Contradiction(TypesHash)` single slot cannot
represent two open contradictions), a **resolution fact type** closes a specific pair by both
byte-heads (hard-stop replay is not resolution), the membership projection returns `CONTESTED`
for the subject, `GroupState` wire bumps to v2 (no compat shim, pre-1.0). All new types and
projection code written **storage-free** (no redb types) — the down payment on Phase 2.
*RED first:* the arrival-order pin (both orders byte-identical) and the two-open-contradictions
test the current schema structurally fails. *Done when:* both pins green, corpus green, mutation
pass on the changed module, rung stated Modeled. *Evidence home:* croft-chat TEST-LOG +
C-SERIES-RESULTS addendum.

**Phase 2 — the re-cut: `social-tree-core` extracted pure.**
New crate (incubated at `alpha/experiments/social-tree-core/`): the fold, projections, ordering
keys, contradiction/resolution machinery, horizon, head-currency — no redb, no I/O, no clock;
storage becomes a `Store` port; `local_storage_projection` becomes the redb adapter + the
existing test harness, consuming the core. Module surface follows croft-group's `group-core`
template (`model/intent/effect/update/wire/project/view`) so the crate is croft-core-shaped from
day one; croft-group's workspace is marked superseded-for-behavior once the harvest is done. *Gate:* the **entire existing corpus green over the
re-cut** (C2/C3/C5, the croft-chat fold tests, stage tests) — behavior-identical extraction,
verified by the tests that already pin behavior, and a **`cargo check --target
wasm32-unknown-unknown` CI gate** on the core crate from this phase forward (pinned toolchain,
per CI-PATTERN). *Done when:* corpus green, wasm-check green, no `redb` in the core's
dependency tree.

**Phase 3 — real signatures on the governance plane (the E112 rung residual, taken here).**
Ed25519 signing/verification on the fold path through the existing `Verifier` boundary, reusing
the conformance crates' real vectors (never redefining schemas in tests). Scope: fact authorship
and quorum counting; the HeadAck-over-real-transport upgrade **stays in E112** (transport rung,
orthogonal to core purity). *Done when:* the fold's authorship checks run on real Ed25519 in the
core test suite; the C-series arms re-run green with signatures live; rung claim for the
governance plane restated accordingly (per-plane, no composite grade).

**Phase 4 — the join: the key layer behind a port.**
The admission machinery (§11.7 token cross-check, merge rule, §7.3.8 stall, admission fact —
the S23–S26-measured shapes) exposed on the core's surface as intents/effects, with MLS behind a
`KeyLayer` port so the core stays pure; the meer-queue crates' openmls code becomes the port's
native adapter, adapted not rebuilt. This is the phase with genuine design risk — the port's
shape (sans-io module vs effect-port; where MLS state lives) gets a short design beat + ADR
before code. openmls-on-wasm is reported upstream but unverified here — probe, `[confirm]`
before the browser shell relies on it. *Done when:* one end-to-end admission (invite path and
token-return path) runs through the joined surface at loopback, with the per-plane rung split
stated (governance per Phase 3; MLS Rung A; transport loopback = Modeled, never Verified).

**Phase 5 — chat tenant v2: `group-chat-core` onto the core.**
Move the tenant's dependency from social-graph-core/local_storage_projection to
`social-tree-core` (+ adapters); social-graph-core's facade folds into the core's tenant-facing
API module or retires. The pond contract is already spoken — this phase is dependency surgery
plus whatever surface gaps the move exposes (each gap = a RED test on the core first). *Done
when:* the croft-chat workspace is green on the new stack and social-graph-core no longer
reaches around the core.

**Phase 6 — the client build-out.**
The croft-chat client grows from demo harness to usable client on core + tenant: ports wired
(redb store, iroh transport, identity), the `CONTESTED` / "membership pending resolution"
rendering (E108's product half), the E116 renderings that apply to chat (factual fork statement;
three registers reachable — mute is a client feature; "admission voided" legibility), and the
feature list of §6 Q4. *Done when:* the Q4 MVP list demonstrably works over real iroh between
two nodes (honest rung: LAN/loopback per run, stated).

**Phase 7 — graduation seam (named, not committed).**
When croft's group pond wants the core: pin `social-tree-core` as a git dep at a commit
(dependency-sourcing rule) or relocate the crate to its long-term home; croft-repo adoption is
its own plan against `croft/core`'s contract (E19's remaining half). Explicitly out of scope
here.

## 5. What this plan does NOT do

- No new protocol design: every mechanism is canonical Part 2 or measured experiment shape; a
  phase that discovers a spec gap stops and files it rather than improvising.
- Does not drain E112: serve-signature adversarial analysis, door-A end-to-end, lapse/invite
  tests, ledger pricing, HeadAck transport rung all stay on that row.
- Does not touch the relay/M4 track (concurrent session).
- Does not adopt into `croft/core` (Phase 7 names the seam only).
- No SLA-grade sizing claims: the §11.11 measurements remain unearned; anything measured here is
  loopback-grade unless stated.

## 6. Open questions for the owner

1. **Crate home + name.** Recommend incubating as `alpha/experiments/social-tree-core` (stays in
   the evidence machinery), name `social-tree-core` (your backbone framing). Confirm or rename.
2. **Phase 4 port shape** gets its own design beat + ADR before code — flagging now that a beat
   sits mid-plan; the alternative (decide now) trades a cheap ADR for design-under-pressure.
3. **Phase 3 scope confirmation:** pulling the real-signatures residual out of E112 into this
   plan (recommended, reasons in §3); E112 keeps the rest.
4. **Phase 6 MVP feature list** — what "built out" means for the chat client (e.g., persistent
   multi-group chat, invite/join flows incl. token return, membership/standing panel with
   CONTESTED, mute; anything more?). Needs your list before Phase 6 starts; does not gate
   Phases 1–5.

## 7. Review log

- 2026-08-20 — drafted (this entry). Phases 1–7 scoped; four open questions posed.
