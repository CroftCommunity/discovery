# social-tree-core: the portable substrate, the chat tenant on it, and the client on both

- **Status:** Draft (2026-08-20) → **Pass 2 against reality (same day)** — claims verified
  against the actual crates, the repo-home question worked (owner's call: **the core lands in
  the croft repo — "this is no longer an experiment"**), phases revised — → **Pass 3 complete
  (2026-08-20): all five §6 questions closed with the owner, one at a time** (answers recorded
  in §6 and the Review Log) — → **Vetted twice and integrated (2026-08-21):** the independent
  vet (no BLOCKER; seven REVISEs, all folded) and the owner-directed alignment companion
  (O11–O15, sequencing) are absorbed; all fifteen opportunities dispositioned; the R4
  resolution-authorization direction and the AGPL-3.0 license are owner-decided and recorded
  (§6, Review Log). **The execution hold is CLEARED. Phase 1 starts on the owner's explicit
  go** — P1 runs concurrent with the M4 track (discovery-side, zero collision); P2 lands in
  croft at a coordinated moment.
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
  Phase 6), **E120** (DID ↔ persona-key binding seam — Phase 7's predictable design question),
  **E121** (charter presets & the three product postures — product layer, strengthens O6/O13).
- **Reviews:** `REVIEW-social-tree-core-plan-2026-08-20.md` (independent vet; findings cited
  as R1–R7/C1–C6/O1–O10) and `REVIEW-social-tree-core-alignment-2026-08-20.md` (program-fit
  companion; O11–O15) — both integrated 2026-08-21; the file:line evidence lives there.

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
   logic in substance is, and the governance plane's **evidence artifacts** run at Modeled rung
   (the substrate suite and C-series record use signature stand-ins; a real-Ed25519 fold path
   already exists in `social-graph-core/src/crypto.rs` — the vet's C4 correction).
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

**Phase 1 — E108: `CONTESTED` in the substrate (the queued front, absorbed with the review
amendments).** `fold_derived` gains the §7.3.2 member-view state: the contradiction artifact
becomes **set-valued and pair-carrying** (each entry holds both conflicting facts as data plus
the contested subject; the current `ForkStatus::Contradiction(TypesHash)` single slot cannot
represent two open contradictions), a **resolution fact type** closes a specific pair by both
byte-heads (hard-stop replay is not resolution), the membership projection returns `CONTESTED`
for the subject, `GroupState` wire bumps to v2 (no compat shim, pre-1.0).

- **Resolution authorization decided (owner, 2026-08-21 — closes the vet's R4):** resolving a
  contested pair is a **governed act whose threshold lives in the charter** (`GroupRules`) —
  the same species and machinery as the ban threshold. Product default **2**; never silently
  single-author (a one-signature resolution would be a verdict on exactly the contradiction
  the fold refused). P1's RED tests are written against charter-quorum resolution; the
  hard-floor question (may a power-mode charter dial it to 1?) and the exact §7.3.2 amendment
  (who signs, threshold source, pair reference) ride **the spec filing P1's close carries** —
  decided by the spec process, never invented mid-GREEN.
- **The projection stays total (the vet's Matrix lesson, L4):** no boolean convenience
  accessor on the membership projection, ever — an untyped mid-resolution state is how the
  lying-UI class returns through a helper method.
- **O9, adopted here (endorsed by both reviews):** `AssertionEnvelope.timestamp` sits inside
  the signed canonical bytes today — a Part 1 §2.0.1 assertion elevated to what a later
  reader will mistake for provenance. P1 is already bumping the envelope schema, so the field
  is **dropped from the core envelope or explicitly fenced** (display-only assertion, never an
  ordering or policy input) now, with a **standing test that no comparator consults it**.

All new types and projection code written **storage-free** (no redb types) — the down payment
on Phase 2. *RED first:* the arrival-order pin (both orders byte-identical) and the
two-open-contradictions test the current schema structurally fails. *Done when:* both pins
green; the resolution-fact tests green against the charter-threshold rule; the timestamp
decision landed with its standing test; corpus green; mutation pass on the changed module; the
§7.3.2 spec filing (resolution authorization + floor) filed; rung stated Modeled. *Evidence
home:* croft-chat TEST-LOG + C-SERIES-RESULTS addendum.

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

*Sizing (restated at integration — the vet's R1):* the Pass-2 redb counts (21/11/2/18) are
exact but they are **localization evidence, not the work**. The real job is a
**state-residency inversion**: today `DerivedFold` holds the database, five fold functions
take `&redb::WriteTransaction`, and the detection passes re-read the governance log from
tables mid-fold — the pure core must own an in-memory state model and take the log as input,
with the adapter becoming the orchestrator that feeds it. The extraction stays bounded for
the true reason: the heavy logic is already pure free functions (`check_authorization`,
`apply_governance`, the `detect_*` family, `genesis_initial_state`), and
`types.rs`/`traits.rs`/`head_ack.rs`/`head_currency.rs`/`horizon*.rs`/`completeness_ahead.rs`
carry zero redb references.

**Review amendments folded into this phase (2026-08-21; evidence in the two REVIEW files):**

- **The purge list (R2), beyond redb:** tokio (broadcast types in `surface.rs`;
  `rt-multi-thread` breaks the wasm gate outright), two `SystemTime::now()` sites, `proptest`
  misfiled under `[dependencies]`, and the **error split** — the core gets a protocol-error
  enum with no storage variants; the adapter gets its own. **`surface.rs`/`LocalStore`
  splits:** command construction lands core-side; notification and persistence orchestration
  land adapter-side.
- **License (owner decision — closes R3):** `social-tree-core` declares **AGPL-3.0**,
  consistent with the standing 2026-07-09 A14 decision (reference code → AGPL-3.0-or-later +
  DCO); croft-chat's `MIT OR Apache-2.0` label was an accident, corrected at the pin-bump;
  A1's MPL-2.0 `hpke-rs` gate is untouched.
- **Citation hygiene (R5):** any test file cited by canonical spec text or a ledger (e.g.
  §7.3.2 cites `croft-chat/tests/fold_ordering_keys.rs` as Measured evidence) either stays in
  place as an adapter-side regression or the citation updates **in the same commit** that
  moves it; the audit is a grep for migrated filenames across `beta/` and the ledgers.
- **CI ratchets into croft's own gates, never a parallel workflow (R7):** the gate lands as
  croft's **G6/G7** firing their recorded triggers (croft/CLAUDE.md "Commit gates" — G6
  triggers on "the first core lands"; G7: watch CI fail before trusting it), one gate command
  identical locally and in CI per CI-PATTERN rule 6.
- **Sequencing against M4 (companion §7):** P1 is discovery-side and runs concurrent with M4
  now; **P2 is the collision point** (root workspace + first required check in the repo where
  the M4 session is mid-milestone) — land it after M4's current milestone closes, or with an
  explicit heads-up so that session expects the new check rather than meeting it mid-push.
- **The layering ADR's full contents (R7 + O11 + O7):** foundation-vs-feature-core; **where
  `call-core` sits** (croft doctrine says "Calling is a capability, not a pond" while the
  skeleton carries `core/call-core` — the ADR resolves that the day the first real crate
  lands); **the two-admissions paragraph (O11):** *the relay admits traffic, never members;
  no fabric-admission signal is an input to the A-series* — forecloses the S16 failure class
  one layer down; and **the effect-composition rule (O7):** how a pond embeds the substrate's
  effects (wrapper enum, mapping, who owns the substrate model) — the one joint in this
  architecture with no prior art, fixed once here, never re-derived per pond. The core's docs
  also carry a short **spec-key → mechanism mapping** (the fold realizes §7.3.1's layered
  order as sequential lamport→hash replay + projections + hard-stops; G1's equivalence
  argument, kept from becoming folklore).
- **Adopted opportunities landing here (all ten adopted, owner 2026-08-21):** **O1**
  conformance vectors wired as croft CI fixtures with the honest scope note (they cover the
  §4/§5/§6 layer; the §7.3–§7.5 fold vectors land when the `[gates-release]` encodings pin —
  the win is the harness existing that day, and P3 gets the signed-preimage vectors free);
  **O2** version-byte discipline as a register (every serialized artifact opens with a
  version byte, every `from_bytes` refuses unknown versions loudly, one in-crate register
  file); **O3** the order-independence proptest migrates as a **named standing CI arm**, with
  P1's CONTESTED permutations extended into it; **O4** cargo-fuzz targets on the `from_bytes`
  surfaces; **O5** API stability markers from day one (sealed traits, `#[doc(hidden)]`,
  `#![warn(missing_docs)]`); **O6 + O13 (P2 acceptance criterion, strengthened by the owner's
  charter-presets direction, E121):** the profile dials as a typed charter/`Profile` struct
  with Croft's reference column as a named constructor, and **no [charter] dial value as a
  compile-time constant in the core — `GroupRules` is the socket** (a core that baked
  door-B/at-join in as assumptions would make the E111 sheet a fiction); **O8** purity
  enforced mechanically (clippy `disallowed-methods`/`disallowed-types` for
  `SystemTime::now`/`Instant::now`/tokio types in the core, plus a `--no-default-features`
  check arm beside the wasm arm); **O10** §11.11 measurement hooks behind a no-op `Metrics`
  port (fold depth, facts-folded, snapshot size, contradiction counts).
- **Boundaries that travel verbatim (O12-discipline + O14):** the `(DeviceId, PrincipalId)`
  credential-pair boundary is spec §4.5's multi-client guarantee, structurally present today —
  it crosses the re-cut intact and the test migration includes whatever pins it; the core's
  principal type stays opaque-but-attributable — **no atproto types anywhere near the core**
  (the DID ↔ persona-key binding seam is E120's, Phase-7 material).
- **Adapter deliverables, named rather than implicit:** persistence-format versioning (the
  comparator stamp + `needs_rebuild` rebuild path is the existing embryo); the redb adapter
  **re-exports the core** so existing path-dep consumers (meer-queue's HeadAck dev-dep) keep
  one hop; the first git pin resolves through the **`github-personal` SSH host** (workspace
  git-identity rule) — one line that saves an afternoon.

*Gate:* the **entire existing corpus green over the re-cut** — the migrated tests green in
croft, the adapter + C-series arms green in discovery against the pinned crate — plus the
**`cargo check --target wasm32-unknown-unknown` CI gate** on the core crate from this phase
forward. *Done when:* corpus green on both sides, wasm-check green in croft CI, no `redb` in
the core's dependency tree, the purge list executed, the O6/O13 acceptance criterion holds
(no charter dial as a core constant), and the ADR (with its R7/O11/O7 contents) landed.

**Phase 3 — real signatures on the governance plane (the E112 rung residual, taken here;
rescoped by the vet's C4).**
The problem is the **evidence artifacts, not a missing mechanism**: real Ed25519 already runs
on a fold path in the corpus — `social-graph-core/src/crypto.rs` ships
`Ed25519Signer`/`Ed25519Verifier` directly over `ed25519-dalek`, and the croft-chat behavior
tests fold through it. What stays mock-signed is the substrate's own suite (`MockSigner`) and
the C-series record (deterministic mock over a digest). P3 is therefore largely
**relocation**: move `crypto.rs` into/beside the core, swap the mocks out of the
substrate-side and meer-side suites, and reuse the conformance crates' signed-preimage
vectors (never redefining schemas in tests). Scope: fact authorship and quorum counting; the
HeadAck-over-real-transport upgrade **stays in E112** (transport rung, orthogonal to core
purity). **The mutation re-baseline (R6) closes this phase:** "mutation-vetted" does not
transfer across the re-cut — the 54-survivor justification ledger is bound to the old module
map, and the X3 harness needs path deps a git pin breaks — so re-baseline the sweep on
`social-tree-core` here (P3 touches the fold's authorship path anyway) and record the
`[patch]`-override recipe for future corpus-side sweeps. *Done when:* the fold's authorship
checks run on real Ed25519 in the core suite; the C-series arms re-run green with signatures
live; the mutation re-baseline is clean (survivors triaged, not scored); rung claim for the
governance plane restated per-plane, no composite grade.

**Phase 4 — the join: the key layer behind a port.**
The admission machinery (§11.7 token cross-check, merge rule, §7.3.8 stall, admission fact —
the S23–S26-measured shapes) exposed on the core's surface as intents/effects, with MLS behind a
`KeyLayer` port so the core stays pure; the meer-queue crates' openmls code becomes the port's
native adapter, adapted not rebuilt. This is the phase with genuine design risk — the port's
shape (sans-io module vs effect-port; where MLS state lives) gets a short design beat + ADR
before code. openmls-on-wasm is reported upstream but unverified here — probe, `[confirm]`
before the browser shell relies on it. **The P4 ADR additionally records two invariants
(integration, 2026-08-21):** the **A3 invariant stays core-side** — the admission *decision*
is computed in the core and the `KeyLayer` port carries artifacts only; a port shape that let
the adapter answer "admit?" would recreate S16's failure one layer up (the vet's C3 nudge).
And the **key-custody/recovery seam is named, not designed** — the port shape must not
foreclose §7.3.9's pluggable backup targets (recovery is Design/pending; naming the seam
costs a sentence). *Done when:* one end-to-end admission (invite path and
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
feature list of §6 Q4. Two integration additions: a **lightweight threat pass precedes the
first non-loopback demo** — P6 wires real iroh and identity, and "loopback-grade, stated"
must not quietly become "demoed to someone" (the deep serve-signature analysis stays E112's);
and one rendering is **explicitly deferred with a home** rather than dropped: the
**lost-race UX** (two concurrent admissions; the protocol side is S-measured, the losing
side's rendering is not) — deferred to Phase 7's product-shell plan, recorded beside the E116
debt. *Done when:* the Q4 MVP list demonstrably works over real iroh between
two nodes (honest rung: LAN/loopback per run, stated), the threat pass done before any
non-loopback demo.

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

### Integration decisions (owner, 2026-08-21 — the review round)

6. **R4, resolution-fact authorization:** charter-quorum-gated — the threshold lives in
   `GroupRules`, the same species and machinery as the ban threshold; product default **2**
   ("so no one gets accidentally banned" applies equally to verdicts); never silently
   single-author. The hard-floor question rides P1's §7.3.2 spec filing. Decided together
   with the wider product direction the owner sketched: **three charter postures at group
   creation** — a default close-circle mode (anyone invites, two to ban, thresholds only, no
   designated roles), a **moderated** mode (threshold slider + a couple of roles with
   pre-packaged selection rules — first-joined, elected), and a **power** mode (every
   justifiable dial exposed) — plus **named, savable, shareable charter configurations**
   (the same person runs a rowdy chat and a guardians group as two named presets; presets can
   be shared and posted; deeper preset packs later; naming open — "peer mode" flagged as
   loaded). Filed as **E121**; it upgrades O6/O13 from nice-to-have to load-bearing (the
   typed charter struct is what makes presets loadable and shareable).
7. **License:** **AGPL-3.0** for everything Drystone/Croft — the core crate declares it;
   croft-chat's `MIT OR Apache-2.0` was an accident, corrected at the pin-bump. Consistent
   with the standing A14 decision; A1's MPL gate untouched. (Closes the vet's R3.)
8. **Opportunities:** the vet's **O1–O10 all adopted as placed** (O9 → Phase 1; the rest →
   Phases 2–3 text and Done-whens); the companion's **O11–O15 folded** (O11/O13/O14 → Phase 2
   and the ADR; **O12 → new row E120**; **O15 → the COHESION seam-line, opened now** — its
   trigger, relay Phases 7–8 beginning, has already fired). No opportunity deferred without a
   named home.
9. **Landing:** the review branch merged to `main`; this integration commits on top (owner
   go-ahead recorded with the branch decision).

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

### Integration — both reviews folded, hold cleared (2026-08-21)

The independent vet (`REVIEW-social-tree-core-plan-2026-08-20.md`: right direction, safe to
execute, **no BLOCKER**; R1–R7, C1–C6, O1–O10) and the owner-directed alignment companion
(`REVIEW-social-tree-core-alignment-2026-08-20.md`: program-level fit, O11–O15, sequencing)
are integrated. **Every REVISE is folded; none waived:** R1 (sizing restated as the
state-residency inversion), R2 (purge list + `surface.rs` split assigned), R3 (closed by the
owner's AGPL-3.0 decision, §6.7), R4 (closed by the owner's charter-quorum direction, §6.6),
R5 (citation rule in P2), R6 (mutation re-baseline at P3's close + `[patch]` recipe), R7
(G6/G7 ratchet, M4 coordination + sequencing, `call-core` placed by the ADR). The **C4
correction** rescoped P3 to relocation (real Ed25519 already on a fold path in
`social-graph-core/src/crypto.rs`); the problem statement was scoped to the evidence
artifacts accordingly. **All fifteen opportunities dispositioned** (§6.8): O1–O10 adopted as
placed, O11/O13/O14 into P2 and the ADR, O12 filed as **E120**, O15 opened as the COHESION
croft-stack ↔ meer seam-line (trigger met — relay Phases 7–8 in flight). The vet's L4/L5
slivers landed where cheap: the projection-totality rule and O9 in P1; the A3-stays-core-side
and custody-seam invariants in the P4 ADR; the threat-pass-before-non-loopback-demo and the
lost-race-UX deferral in P6. New rows: **E120** (DID ↔ persona-key binding seam), **E121**
(charter presets & the three product postures, per the owner's 2026-08-21 sketch).

**Status: the execution hold is CLEARED.** Phase 1 starts on the owner's explicit go —
discovery-side, concurrent with M4; Phase 2 lands in croft at a coordinated moment per the
sequencing note in P2.

### Phase 1 — EXECUTED AND GREEN (owner's go 2026-08-21; closed 2026-08-22)

TDD RED-first throughout: the four pins recorded structurally RED (the old schema cannot
express them), then GREEN in three commits (CONTESTED + resolution; O9 envelope v2; pin 5),
each committed green before any mutation. Delivered exactly the amended P1 scope: the
set-valued pair-carrying `ContestedEntry` schema; the total `membership()` view (no boolean
accessor); `Resolution` (0x000C) charter-quorum-gated at the owner's default 2 riding the V5′
Approval machinery; `GroupState` wire v2 refusing unknown versions; all new logic
storage-free — and the P2 down payment grew: **one shared transition now serves live ingest
and the rebuild replay** (the replay previously ran no detection; a rebuilt contested store
silently lost its hard-stop — pre-existing, closed). O9 landed as the drop (not the fence):
envelope wire v2, standing layout pin, three decoders refusing v1, timeline windows and the
compaction age gate now position-denominated. Two more pre-existing defects fixed en route:
the order-dependent slot-fork label (now max-over-contenders, a pure function of the
contender set) and a phantom 8-byte read in governance's decoder copy.

Evidence: 5/5 pins; substrate **102/0**; croft-chat workspace **120/0**; bounded X3-pattern
mutation sweep **30/35 killed**, 5 survivors triaged (2 equivalent with stated arguments, 4
pre-existing NodeCard survivors from the X3 ledger) — the full re-baseline stays at P3's
close per R6. Durable record: `experiments/local_storage_projection/C-SERIES-RESULTS.md`
§P1/E108. Spec filings out of the build: **E133** (§7.3.2 amendment set — the R4 hard-floor
question plus four edges the build surfaced). E108 retires through this phase.

**Next: Phase 2 (the re-cut, landing in croft) — at a coordinated moment with the live M4
session per the sequencing note; the crate, workspace, and CI scaffolding land together.**

### Phase 2 — EXECUTED AND GREEN (owner's go 2026-08-23; landed the same day)

The coordinated moment arrived on its own: M4d closed its arc and croft's tree was clean.
Landed in croft (fast-forwarded to `1d00c05`): the root Cargo workspace;
**`core/social-tree-core`** — model, wire (ONE public canonical decoder + the
WIRE-REGISTER), update (the fold plus `evaluate()` over `FoldContext` — the
state-residency inversion as API), project (horizon, head-ack, head-currency,
completeness), ports (with the mocks deliberately ungated for the adapter's suites),
charter (`croft_default()` = E121's close-circle posture; every dial data), metrics
(no-op §11.11 hooks); AGPL-3.0; purity enforced mechanically (clippy
disallowed-methods, wasm32 + no-default-features CI arms); the five E108 pins restated
PURE plus the O3 standing order-independence proptest (27 green). `make gate` green end
to end — **G6 armed and fired; G7's workflow is wired** (pull_request trigger, one gate
command) with its watch-it-fail moment reserved for the first push. ADR-0002 records the
foundation-vs-ponds layering, call-core's resolution (the capability doctrine is about
authority, not layout), the two-admissions rule, and the effect-composition rule.

Discovery side (`919ddd8`, −3,745 lines): `local_storage_projection` is now the redb
adapter — seven modules are re-export shims, ingest and the rebuild replay assemble a
`FoldContext` and call core `evaluate()`, the adapter error bridges variant-to-variant
(zero downstream churn), and the pin is a git dep by commit (interim file:// URL; swaps
to the github-personal remote at first push). **The consolidation caught a live defect:**
the corpus's three decoder copies had two different byte contracts, and one still read
the retired timestamp slot — every storage boundary now slices the store byte once and
decodes through core's single decoder. Two toolchain traps surfaced and held: Homebrew's
same-version rust shadowing rustup (croft's verify.sh already refuses it — the gate
worked), and the worktree's missing `android/local.properties`.

Evidence, fresh: core 27/0 · adapter 82/0 · croft-chat workspace 120/0 (the corpus-green
gate, both sides of the pin) · wasm32 and no-default-features arms green · no `redb` in
the core's tree.

Honest deviations, named with homes: **O1** (conformance vectors as croft CI fixtures)
rides P3, which needs the signed-preimage vectors anyway; **O4** (fuzz targets) waits for
nightly in the toolchain manifest — an unrunnable fuzz dir is gate theatre by croft's own
G2; the **surface command-construction split** (construction core-side) rides P5's tenant
API; the adapter's **genesis-seed SystemTime** wart stays on the purge list; core doc
coverage rides the missing_docs ratchet (warn now, deny when clean).

**Next: Phase 3 — real signatures (largely relocating `social-graph-core/src/crypto.rs`)
+ the mutation re-baseline on the new crate (R6), which the re-cut has now made
X3-simple again (path-dep patch against the pin).**

### Phase 3 — EXECUTED AND GREEN (owner's go 2026-08-23; closed the same day)

Relocation, as C4 promised: `ports::ed25519` (deterministic, wasm-clean, zeroizing,
feature-gated so the lean arm proves the fold needs no crypto crate). Authorship evidence on
real Ed25519 end to end — core pins sign-and-verify (35/0 incl. the O1 fixture harness:
the conformance crate's emitted signing vectors verified through the core port), C2/C3
stand-ins swapped out (adapter 82/0). Per-plane rung restated in C-SERIES-RESULTS §P3.
**R6 closed:** full-crate re-baseline (629 mutants: 168 caught in-crate, 63 unviable, 398
in-crate survivors registered per-module as the standing corpus-side burn-down — the strong
killers live with the consumers, per MUTATION.md's [patch] recipe; the P1-scope functions
already carry cross-package verdicts). HeadAck-over-real-transport stays E112, as scoped.

### Phase 4 — EXECUTED AND GREEN (owner's go 2026-08-24; ADR beat + build in one arc)

**The ADR beat first, as Pass-3 Q2 ordered** (croft `docs/ADR-0003-keylayer-port.md`,
proposed 2026-08-23 → accepted 2026-08-24): the KeyLayer port carries artifacts and parsed
claims, never answers "admit?" — the admission decision is a pure core function whose
`MergeApproval` (private fields, fact riding inside) is the only key that turns the port's
merge, so A3 and the §11.7 merge-rule clause are type errors, not conventions. MLS state
adapter-side entirely; custody seam named (`KeyCustody`, future port), not designed —
KeyLayer owes no export surface. The same session produced the **tree-frame model pin**
(person-rooted; groups one aspect — ADR-0002 amendment, COHESION §72, row E134).

**Then the build, five RED→GREEN increments in croft (branch `p4-keylayer`,
`7c2a22f..8c35820`), workspace 62/0 with clippy/fmt/wasm arms green throughout:**

1. **`admission::evaluate_admission`** — S24's refusal set (no-issuance-fact, revoked,
   lineage-mismatch) + standing at position (Excluded refuses; **Contested stalls without a
   verdict** — E108's rule reaching admission) + the §7.3.8 stall consuming C3's
   `admits_membership_origination`, fail closed below k, lifting exactly at k.
2. **`ports::keylayer`** — stage returns claims as data; merge demands the slip; seam pins
   prove deposit-what-was-minted and that a refusal has no code path to the key layer.
3. **The §7.6.4 removal-kind distinction** (`MembershipRemove` = subject ‖ kind; kindless
   refused): the fold had conflated ban with departure, and the admission machinery cannot
   exist on the conflation (every returner has a removal in their history; only a BAN blocks
   re-entry). Two finds en route: **the old fold violated the exit floor** (Part 1 §2.5 —
   every removal demanded Admin role AND the remove quorum, so a member of a two-to-ban
   group could not leave alone; a self-departure now passes both gates, a self-authored ban
   refuses), and **contesting every non-commutative race WAS the old convergence strategy**
   — so the kind-narrowing owed a replacement: the benign departure-vs-readd race now
   reconciles by canonical full-log replay (§7.4.1), pinned convergent in both arrival
   orders (`RaceDisposition::{None, Contested, BenignReconcile}` through the shared
   transition).
4. **The admission machinery as chain data** — `TokenIssuance` 0x000D, `TokenRevocation`
   0x000E (names-an-issuance or refuses), `Admission` 0x000F; `issuance_view` derives the
   decision's context straight from the log; **GroupState v3** carries the standing-ceiling
   set (fed by ban-kind removals, cleared ONLY by a readmission DECISION) so replay applies
   an admission at position without reaching outside the fold. C4's two-sided boundary
   pinned in the core: fact-vs-ban folds silently-but-visibly to excluded in both orders,
   never CONTESTED; quorum-vs-ban still hard-stops. The adapter corpus meets v3 at the next
   pin bump as a rebuild (WIRE-REGISTER posture).
5. **The invite path under the same discipline** — `authorize_invite_enactment` mints the
   `InviteApproval` only when the fold has already seated the invitee (MLS seating follows
   the fold, never precedes it); the build generalized the ADR: **every membership-mutating
   port operation demands a core-minted slip.**

**The done-when: `ports/keylayer-openmls`** — meer-queue's measured code adapted, not
rebuilt (identity bridge: the leaf credential's bytes ARE the core `PrincipalId`; exact
version pins). The loopback e2e runs the invite path (fold decision → slip → real
Add-commit + Welcome → seated → AEAD round-trip), dormancy, and the token-return path (REAL
external commit + PSK proposal → staged claims → chain-derived cross-check → merge →
admission fact folded → the returner reads and is read again); the S16 arm holds on real
crypto (a stranger's flawless commit with leaked token bytes stages and dies at the
decision, never seated). **openmls-on-wasm `[confirm]` moves: COMPILES** for
wasm32-unknown-unknown with the js features (getrandom + openmls); browser runtime stays
unverified — compile-proof only. Per-plane rungs: governance real-Ed25519; MLS Rung A;
transport loopback = Modeled, never Verified.

**Audit:** bounded mutation pass on the phase's decision logic (admission.rs): 17 mutants —
12 caught, 5 unviable, **0 missed** after the two issuance_view payload-guard survivors
were killed with the truncated-entry test they were guarding for. A latent find fixed en
route: **the CI core-purity clippy arm could never pass** (crate-attribute `warn(missing_docs)`
overrides the CLI `-A`; `-D warnings` would elevate the 95-item docs burn-down) — CI has
never run, the exact G7 class; fixed with `--force-warn missing_docs`.

**Deviations, named:** `enact_departure` stays inherent and un-slip-gated (the removal
enactment joins the slip discipline with the eviction machinery — ADR-0003 consequences);
issuance authorization is Admin+/threshold-1 with no dial (spec filing → **E136** with the
other P4-surfaced questions); HeadAck transport and serve-time signatures stay E112.

**Next: Phase 5 — chat tenant v2 (`group-chat-core` onto the core, landing as
`croft/core/chat-core` per Pass-3 Q5): dependency surgery, each surface gap a RED test on
the core first.**

