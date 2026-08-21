# social-tree-core: the portable substrate, the chat tenant on it, and the client on both

- **Status:** Draft (2026-08-20) → **Pass 2 against reality (same day)** — claims verified
  against the actual crates, the repo-home question worked (owner's call: **the core lands in
  the croft repo — "this is no longer an experiment"**), phases revised — → **Pass 3 complete
  (2026-08-20): all five §6 questions closed with the owner, one at a time** (answers recorded
  in §6 and the Review Log). **READY FOR EXECUTION; not started — execution begins on the
  owner's go.** Phase 1 is the already-queued E108 build, absorbed here unchanged.
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

**Phase 2 — the re-cut: `social-tree-core` extracted pure, landing in the croft repo.**
New crate at **`croft/core/social-tree-core`** (owner's call, Pass 2 — the product repo, not the
experiment corpus; "this is no longer an experiment"): the fold, projections, ordering keys,
contradiction/resolution machinery, horizon, head-currency — no redb, no I/O, no clock; storage
becomes a `Store` port; `local_storage_projection` becomes the redb adapter, consuming the core.
One layering note so the ADR stays true: this crate is **not a pond** beside `call-core` and
`feed-core` — it is the **substrate ponds consume**; `core/` gains that second layer
deliberately. Module surface follows croft-group's `group-core` template
(`model/intent/effect/update/wire/project/view`); croft-group's workspace is marked
superseded-for-behavior once the harvest is done.

Landing consequences, priced (Pass 2):

- **croft's `core/`, `ports/`, `shell/` are `.gitkeep` placeholders today** — verified. This
  phase therefore also scaffolds the croft root Cargo workspace and stands up **CI per
  CI-PATTERN** (a gate with a `pull_request` trigger; the CI toolchain pinned to the same
  1.97.1 the repo pins locally — the two rules most often missed). The toolchain file already
  carries `wasm32-unknown-unknown` and the android targets, so the wasm gate is a target away,
  not a toolchain change.
- **The behavior-pinning tests migrate with the code** (fold ordering, contradiction/CONTESTED,
  projection, horizon, head-currency — they are the core's tests, not the experiment's);
  `local_storage_projection` keeps the redb adapter, the adapter-grade tests, and the C-series
  arms that exercise storage specifics; TEST-LOG and the evidence ledgers stay in discovery
  (records, not code).
- **Dependency direction reverses:** discovery experiments consume `social-tree-core` as a
  **git dep pinned to a commit** (the dependency-sourcing rule — never a cross-repo path dep),
  bumped at phase gates when the corpus re-runs; the tight edit loop during extraction stays
  intra-croft because the migrated tests travel with the crate.
- **Vendor-neutrality unchanged:** the §9 conformance vectors and the Proofs crates stay the
  neutral bar in discovery; croft's crate is the product realization measured against them.

*Gate:* the **entire existing corpus green over the re-cut** — the migrated tests green in
croft, the adapter + C-series arms green in discovery against the pinned crate — plus the
**`cargo check --target wasm32-unknown-unknown` CI gate** on the core crate from this phase
forward. *Done when:* corpus green on both sides, wasm-check green in croft CI, no `redb` in
the core's dependency tree. *Sizing (Pass 2, measured):* the redb entanglement is localized —
`fold_derived.rs` 21 references (mostly four `From<redb::*Error>` impls plus table reads),
`governance.rs` 11, `surface.rs` 2, and `tables.rs` (18) staying adapter-side wholesale — a
bounded extraction, not a spread.

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
API module or retires. With the core in croft, the tenant's natural landing is
**`croft/core/chat-core`** — a pond beside `call-core` and `feed-core`, exactly the ADR's
symmetry (§6 Q5; recommended). The pond contract is already spoken — this phase is dependency
surgery plus whatever surface gaps the move exposes (each gap = a RED test on the core first).
*Done when:* the chat stack is green on the new core and social-graph-core no longer reaches
around it.

**Phase 6 — the client build-out.**
The croft-chat client grows from demo harness to usable client on core + tenant: ports wired
(redb store, iroh transport, identity), the `CONTESTED` / "membership pending resolution"
rendering (E108's product half), the E116 renderings that apply to chat (factual fork statement;
three registers reachable — mute is a client feature; "admission voided" legibility), and the
feature list of §6 Q4. *Done when:* the Q4 MVP list demonstrably works over real iroh between
two nodes (honest rung: LAN/loopback per run, stated).

**Phase 7 — product-shell adoption (named, not committed).**
With the core and pond born in croft (P2/P5), the remaining seam is the **product shells**
consuming them — the android/apple/web shells rendering the chat pond, the uniffi surface, the
rebuild of the inherited croftcall client onto the shared core. That is its own plan with its
own constraints per platform. Explicitly out of scope here; this plan's client (P6) is the
dev-harness chat client, not the product shells.

## 5. What this plan does NOT do

- No new protocol design: every mechanism is canonical Part 2 or measured experiment shape; a
  phase that discovers a spec gap stops and files it rather than improvising.
- Does not drain E112: serve-signature adversarial analysis, door-A end-to-end, lapse/invite
  tests, ledger pricing, HeadAck transport rung all stay on that row.
- Does not touch the relay/M4 track (concurrent session).
- Does not touch the product shells (android/apple/web) — the core and pond land in
  `croft/core`, but shell adoption is Phase 7's successor plan, not this one.
- No SLA-grade sizing claims: the §11.11 measurements remain unearned; anything measured here is
  loopback-grade unless stated.

## 6. Open questions — CLOSED at Pass 3 (owner, 2026-08-20, one at a time in plain English)

Answers first; the original questions kept below each for provenance.

1. **CLOSED — yes to all three**: path `croft/core/social-tree-core`; the short croft-side ADR
   (foundation-vs-feature-core layering); P2 owns the workspace + CI scaffolding.
2. **CLOSED — ADR beat at phase 4**: the `KeyLayer` port is designed against the real core
   surface when P4 arrives, recorded as a croft ADR before any P4 code.
3. **CLOSED — yes**: the real-signatures residual moves from E112 into this plan as Phase 3;
   E112 keeps its other residuals.
4. **CLOSED — all four in the MVP**: persistent multi-group chat; invite/join incl. token
   return; the truthful membership panel (`CONTESTED` + returner-side "admission voided");
   mute.
5. **CLOSED — both as recommended**: chat tenant lands as `croft/core/chat-core` at P5; the TUI
   client stays discovery-side as the dev harness on pinned crates.

1. **Landing details (home is decided — croft repo, owner 2026-08-20).** Confirm the specifics:
   path `croft/core/social-tree-core`, the substrate-beside-ponds layering note recorded in the
   croft repo (a short ADR there — croft carries its own architecture record), and that P2's
   scope now includes the croft root workspace + CI-PATTERN gate scaffolding (Pass-2 finding:
   `core/`/`ports/`/`shell/` are `.gitkeep`, no workflows exist).
2. **Phase 4 port shape** gets its own design beat + ADR before code — flagging now that a beat
   sits mid-plan; the alternative (decide now) trades a cheap ADR for design-under-pressure.
3. **Phase 3 scope confirmation:** pulling the real-signatures residual out of E112 into this
   plan (recommended, reasons in §3; Pass 2 confirmed the machinery — real `ed25519-dalek` in
   `lineage-core`, vectors in the conformance crate); E112 keeps the rest.
4. **Phase 6 MVP feature list** — what "built out" means for the chat client (strawman:
   persistent multi-group chat, invite/join flows incl. token return, membership/standing panel
   with CONTESTED, mute; anything more?). Needs your list before Phase 6 starts; does not gate
   Phases 1–5.
5. **Tenant and client homes (new, follows from the croft landing).** Recommended: the chat
   tenant lands as `croft/core/chat-core` at P5 (a pond beside call/feed, the ADR's symmetry);
   the TUI chat client stays discovery-side as the dev harness consuming pinned crates (the
   product shells are Phase 7's successor plan). Confirm or redirect.

## 7. Review log

- 2026-08-20 — drafted. Phases 1–7 scoped; four open questions posed.

### Pass 2 — against reality (2026-08-20)

Owner input driving the pass: the core belongs in the **croft repo** ("this is no longer an
experiment"), and the plan gets its Pass 2/Pass 3 before anything starts. Findings, each
verified this pass, plan revised in place:

1. **Repo home worked and adopted.** P2 now lands `croft/core/social-tree-core`; consequences
   priced in P2 (workspace + CI scaffolding, test migration, dependency-direction reversal,
   vendor-neutrality note). P7 rewritten from "graduation seam" to "product-shell adoption"
   and the §5 croft-exclusion corrected — the draft's "does not adopt into croft/core"
   contradicted the new landing and is struck.
2. **croft skeleton verified**: `core/`/`ports/`/`shell/` are `.gitkeep` placeholders; no
   `.github/workflows/`; no root Cargo workspace. New P2 work discovered and added
   (scaffolding + CI-PATTERN gate). Toolchain verified ready: pinned 1.97.1 with
   `wasm32-unknown-unknown` and android targets, clippy/rustfmt components.
3. **Green baseline measured, not assumed** (fresh runs this pass): croft-chat workspace
   **115 passed, 0 failed**; local_storage_projection suite **green (exit 0)** — the corpus
   the P2 gate re-runs is real and passing today.
4. **redb entanglement sized**: fold_derived.rs 21 refs (four `From<redb::*Error>` impls +
   table reads), governance.rs 11, surface.rs 2, tables.rs 18 (stays adapter-side wholesale).
   Bounded extraction.
5. **P3 machinery confirmed**: real `ed25519-dalek` in `alpha/Proofs/lineage-groups/crates/
   lineage-core`; conformance crate present beside it. The reuse claim is grounded.
6. **openmls pin noted**: meer-queue pins `openmls =0.8.1` exactly (with rust-crypto and
   basic-credential companions); the P4 adapter adapts that code at that pin; openmls-on-wasm
   stays `[confirm]`.
7. **New question posed** (§6 Q5): tenant home (`croft/core/chat-core` recommended) and client
   home (TUI stays discovery dev harness) — a consequence of the croft landing the draft could
   not have asked.

**Status after Pass 2: revised, not started. Pass 3 (quality gates; §6 closed with the owner)
is the remaining gate before execution.**

### Pass 3 — the five questions closed with the owner (2026-08-20)

Walked one at a time in plain English at the owner's request; answers recorded in §6 in full.
Summary: **Q1** landing specifics confirmed (path, croft ADR, P2 owns plumbing); **Q2** the
MLS/core joint gets its ADR beat at P4, not a premature pin; **Q3** real signatures come in as
P3; **Q4** the P6 MVP is all four strawman features; **Q5** chat-core to croft at P5, TUI stays
the discovery dev harness. One Pass-2 loose end closed: the local_storage_projection baseline
count completed after the Pass-2 commit — **99 passed, 0 failed** — so the measured baseline is
**214 green across the two suites** (115 croft-chat + 99 substrate), superseding the interim
"green (exit 0)" wording in Pass-2 finding 3.

**Status after Pass 3: READY FOR EXECUTION. Nothing started; Phase 1 (E108) begins on the
owner's go.**

### Independent vet ordered (2026-08-20) — execution holds for it

The owner ordered a fresh-eyes review before execution ("too close to the problem … you miss
obvious flaws", paraphrase). Reviewer prompt:
`alpha/seeds/generated-prompts/vet-social-tree-core-plan-prompt.md` — six lenses (Part-1
philosophy, Part-2 fidelity, technical soundness, architecture against comparable platforms,
direction, cheap-now/big-later), refute-first posture, findings as
BLOCKER/REVISE/CONFIRM/OPPORTUNITY. Expected artifact:
`alpha/plans/REVIEW-social-tree-core-plan-2026-08-20.md`. **Execution begins only after the
review lands and the owner clears it** (BLOCKERs resolved, REVISEs folded or waived).
