# Independent review: the social-tree-core plan, vetted against the corpus and the code

`Written 2026-08-20 by the independent-review session commissioned in`
`alpha/seeds/generated-prompts/vet-social-tree-core-plan-prompt.md. Method: refute-first — every`
`load-bearing plan claim was checked against the spec (Part 1 map + §2.0.1, Part 2 §§1.3, 4.6,`
`7.3–7.5, 9, 10.2.2, 10.5, 11.6–11.8, 11.9.2, 11.11, App B/C read directly), the implementation`
`profile, the client ADR + COHESION §23, the actual crates (local_storage_projection, croft-chat,`
`croft-group/group-core, meer-queue, Proofs/lineage-groups), the ledgers (ROADMAP_TODO E19/E108/`
`E110–E117, STATE-AND-NEXT, C-SERIES-RESULTS, X3 mutation reports), the croft repo skeleton, and`
`CI-PATTERN. Test suites were NOT re-run (the 214-green baseline is accepted as a record per the`
`verification budget). Every claim below is tagged verified-by-me or judged. This file is the only`
`artifact this review created.`

---

## 1. Plain-English owner summary

**The direction is right and the plan is safe to execute — no BLOCKER.** The merge thesis holds
under attack: the June lineage really is architecture-right/protocol-stale, the current substrate
really is protocol-right/architecture-wrong, and extraction genuinely preserves evidence a rewrite
would burn. Phase order (schema fix → re-cut → signatures → join → tenant → client) survives
refutation at each joint. The one thing I would change before Phase 2 starts: **the plan sizes the
re-cut by counting `redb` references, and that is the wrong metric.** The counts are exactly right
(I reproduced them), but the real work is a structural inversion the counts do not measure — today
the fold's state *lives in the storage tables* and the fold runs *inside storage transactions*;
a pure core holds state in memory and takes the log as input. Alongside that, the impurity
inventory is longer than "redb": the substrate also carries tokio, two wall-clock call sites, a
wall-clock field inside the signed envelope bytes, and test-only deps in the main dependency
tree — all of which must be named as Phase-2 scope or Phase 2 discovers its own scope in week one.
Seven REVISE findings (plan-text fixes, all cheap, none re-opening an owner decision), a set of
CONFIRMs, and ten concrete cheap-now/big-later moves follow. Phase 1 (E108) can start regardless:
none of the REVISEs touch it except the resolution-fact question (R4), which Phase 1 should treat
as its likely first stop-and-file rather than a surprise.

---

## 2. Findings, ranked

No BLOCKER findings. REVISE findings first (plan text or phase content should change before the
phase they touch), then CONFIRMs, then OPPORTUNITIES (§5 carries the L6 detail).

### R1 — REVISE (Phase 2): the sizing metric is honest arithmetic but mismeasures the work

**verified-by-me.** The plan's counts are exact — I reproduced 21/11/2/18 `redb`-referencing lines
in `fold_derived.rs`/`governance.rs`/`surface.rs`/`tables.rs`
(`alpha/experiments/local_storage_projection/src/`). But the plan characterizes them as "mostly
four `From<redb::*Error>` impls plus table reads" (plan:141–142) and concludes "a bounded
extraction, not a spread" (plan:143). What the counts do not show:

- `DerivedFold` **holds the database**: `db: Arc<Db>` (fold_derived.rs:766–774), and
  `ingest()` opens its own read/write transactions inline (fold_derived.rs:840–861).
- Five fold functions take `txn: &redb::WriteTransaction` as a parameter
  (fold_derived.rs:1253, 2235, 2268, 2309, 2357) — the fold executes *inside* storage
  transactions, not behind them.
- The detection passes re-read the governance log from tables mid-fold
  (`group_governance_log(txn, …)`, fold_derived.rs:1622, consumed by `detect_mutual_expulsion`,
  `detect_removed_then_included`, `detect_role_thrash`, `detect_competing_rulechange`).
- The public API is db-shaped: `read_group_state`, `rebuild(db, …)`, `needs_rebuild(db)`,
  `comparator_version(db)` (fold_derived.rs:821, 2004, 1959, 1933).

So the extraction is not "delete the From impls and route reads through a trait" — it is an
inversion of state residency: the core must own an in-memory state model (or take the log as an
explicit input) and the adapter must become the orchestrator that feeds it. **judged:** the work
is still bounded — a large share of the logic is already pure free functions
(`check_authorization` fold_derived.rs:384, `apply_governance`:627, the `detect_*` family,
`genesis_initial_state`:597), and `types.rs`/`traits.rs`/`head_ack.rs`/`head_currency.rs`/
`horizon*.rs`/`completeness_ahead.rs` carry zero redb references (verified-by-me) — so the
plan's conclusion survives; its stated basis does not. Fix: restate P2's sizing paragraph around
the inversion (state-in-tables → state-in-memory + Store port), keeping the counts as the
localization evidence they actually are.

### R2 — REVISE (Phase 2): the impurity inventory beyond redb is missing, and `surface.rs`'s landing side is unstated

**verified-by-me**, each item:

- **tokio** is a substrate dependency (Cargo.toml: `tokio = { version = "1.18", features =
  ["sync", "rt", "macros", "rt-multi-thread"] }`) and is load-bearing in `surface.rs`
  (broadcast channel types at surface.rs:190–191, 243); the croft-chat workspace comment states
  it plainly: "Async runtime (the substrate's write commands are async)"
  (`croft-chat/Cargo.toml`). The croft repo's own doctrine: "One `async fn` in a core destroys
  the property that makes the whole architecture testable" (croft/CLAUDE.md, "The rules that are
  load-bearing"). And `rt-multi-thread` does not build on `wasm32-unknown-unknown`, so the P2
  wasm gate fails if this travels.
- **Wall-clock**: `std::time::SystemTime::now()` at surface.rs:1082 (genesis GroupId seed) and
  surface.rs:1709 (`unix_now()`, called from ~9 command sites, surface.rs:1139–1517). Croft
  doctrine again: "a clock read is the classic slow rot. If a core needs the time, it is an
  effect" (croft/CLAUDE.md).
- **A wall-clock field inside the signed bytes**: `AssertionEnvelope.timestamp: u64` ("Wall-clock
  timestamp (Unix seconds)", types.rs:251–252) is serialized into `canonical_bytes()`
  (types.rs:298) and therefore signed. The comparator does not consult it (G1's v2 comparator is
  lamport → hash, `croft-chat/tests/fold_ordering_keys.rs` header), so this is not a §7.3.1
  violation today — but Part 1 §2.0.1 makes it an assertion, never a fact, and the re-cut is the
  moment to drop it or fence it (see O9).
- **Test-only deps in the main tree**: `proptest` and `proptest-derive` sit under
  `[dependencies]`, not `[dev-dependencies]` (Cargo.toml), used only in `#[cfg(test)]` modules
  (verified via lib.rs:13–23). Cheap Cargo.toml fix, but it must not be copied into the core.
- **Error taxonomy**: `FoldError` carries `StorageError(String)` plus the four redb `From` impls
  (fold_derived.rs:117–145). A pure core needs a protocol-error enum with no storage variants;
  the adapter needs its own. The plan never mentions the error split.
- **Unstated boundary**: the plan's P2 core scope is "the fold, projections, ordering keys,
  contradiction/resolution machinery, horizon, head-currency" (plan:108–109) — `surface.rs`
  (`LocalStore`, the command/query surface, 2733 lines) is counted in the sizing but never
  assigned a side. It is where the tokio and wall-clock impurities live, so the assignment is
  load-bearing.

Fix: add the inventory to P2 as an explicit purge list, and state where `surface.rs`/`LocalStore`
lands (judged: it splits — command construction is core, notification/persistence orchestration is
adapter — but the plan should say so, not the reviewer).

### R3 — REVISE (Phase 2): the license consequence of the croft landing is unpriced

**verified-by-me.** The croft repo is AGPL-3.0 (`croft/LICENSE`: "GNU AFFERO GENERAL PUBLIC
LICENSE Version 3"). The croft-chat workspace is `MIT OR Apache-2.0`
(`croft-chat/Cargo.toml:7`); `local_storage_projection` declares no license at all. Landing
`social-tree-core` in croft makes the portable protocol substrate AGPL by default, and the
reversed dependency direction then pulls AGPL code into MIT/Apache-labeled discovery workspaces.
**judged:** legally inert while nothing is distributed, but it is exactly the kind of silent
default the corpus refuses elsewhere — and for a crate positioned as "the substrate ponds
consume", possibly someday by a second implementer, strong copyleft vs. permissive is an owner
decision, not an accident of which repo the crate landed in. Fix: one line in P2 — the core
crate's license is declared deliberately (owner call), and the discovery workspaces' license
fields are made truthful at the pin-bump.

### R4 — REVISE (Phase 1): the resolution fact's authorization semantics are unspecified — pre-declare the stop

**verified-by-me** (the gap is the spec's, inherited by the plan). Canonical §7.3.2 requires the
fact type but never says who may author it: "resolution needs its own fact type, since the
hard-stop replay is not human resolution" and "on resolution the projection returns to member or
not-member per the resolving facts" (part-2:1066). The plan's P1 scope is "a **resolution fact
type** closes a specific pair by both byte-heads" (plan:96–97) — also silent on authorization.
**judged:** this is the sharpest philosophical edge in Phase 1: a resolution fact any single
member can author is a one-signature verdict on exactly the contradiction the fold refused to
resolve — an authority-bearing artifact Part 1 §2.5 and P-Peer-Equality forbid. Presumably it
must be quorum-gated (a §7.2 R7-shaped decision, or the §7.6.7 hold-lift), but *presumably* is
not a spec cite. The plan already has the right rule — "a phase that discovers a spec gap stops
and files it rather than improvising" (plan:190–192) — so the fix is cheap: name this as P1's
expected stop-and-file (or resolve it with the owner before P1's RED tests are written), so the
schema (who signs, what threshold, does the fact reference the pair by both byte-heads or by the
promoted label) is decided by the spec process, not invented mid-GREEN.

### R5 — REVISE (Phase 2): test migration will dangle canonical evidence pointers unless handled in the same move

**verified-by-me.** Canonical Part 2 cites experiment test files as its Measured evidence — §7.3.2
cites the divergence as "***Measured*** (G1, `croft-chat/tests/fold_ordering_keys.rs`)"
(part-2:1066). The plan migrates "the behavior-pinning tests … with the code (fold ordering,
contradiction/CONTESTED, projection, horizon, head-currency — they are the core's tests)"
(plan:124–126) — `fold_ordering_keys.rs` is squarely in that set. After migration the spec's
pointer names a file that no longer carries the pin (or no longer exists). **judged:** the plan's
provenance posture ("TEST-LOG and the evidence ledgers stay in discovery (records, not code)",
plan:127–128) is right but incomplete — a spec citation is a provenance edge too. Fix: add one
rule to P2: any test file cited by canonical spec text or a ledger either stays in place as an
adapter-side regression (with the migrated copy noted), or the citation is updated in the same
commit that moves it. A grep for the migrated filenames across `beta/` and the ledgers is the
whole audit.

### R6 — REVISE (Phase 2/3): "mutation-vetted" does not transfer across the re-cut — schedule the re-baseline

**verified-by-me.** The vetting is real: RUN-07's automated cross-package sweep resolved every
in-substrate survivor — "**7 killed, 54 individually justified, 0 unjustified survivors**"
(`local_storage_projection/X3-AUTOMATED-SWEEP.md`). But the evidence is bound to the current cut
twice over: the 54 justifications are per-line of the current module structure, and the harness
itself (`x3_cross_package_harness.py`) exists *because* cargo-mutants cannot mutate across
workspace boundaries — it applies diffs to the substrate source and runs the croft-chat suite
over a **path** dependency (X3-AUTOMATED-SWEEP.md, "Tool configuration"). After the re-cut plus
the git-dep reversal, both break: the module map changes and a git dep cannot be mutated in place
at all (a corpus-side sweep would need a `[patch]` local override). **judged:** the plan invokes
mutation only for P1's changed module (plan:101–102) and carries "mutation-vetted" as a standing
property of the substrate (plan:14–15). Fix: name a mutation re-baseline on `social-tree-core`
as a phase-gate item (after P2, or folded into P3's close since P3 touches the fold's
authorship path anyway), and record the `[patch]`-override recipe for future corpus-side sweeps.

### R7 — REVISE (Phase 2, small): ratchet into croft's existing gates, name the concurrent-session interaction, and let the ADR place call-core

**verified-by-me**, three parts:

- croft already prescribes its own gate ratchet: G5 (`make verify`), G6 (`make gate` when "the
  first core lands"), G7 ("CI runs the same `make gate` … **Watch it fail before trusting it**")
  (croft/CLAUDE.md, "Commit gates"). CI-PATTERN rule 6 demands "One gate command, identical
  locally and in CI" (.claude/CI-PATTERN.md). P2's "stands up CI per CI-PATTERN" (plan:119–120)
  should say it lands as G6/G7 firing their recorded triggers — not a parallel workflow with its
  own command.
- The croft repo is under a live concurrent track: the last commits are M4 work dated through
  2026-08-20 (`git log`: c32cca9 "M4b: proof acquisition…"). P2 adds a root Cargo workspace and
  a PR/push gate under that session's feet. **judged:** the gate is Rust-scoped and the M4 work
  is Kotlin/relay-side, so collision risk is low — but the plan should name the coordination
  (the M4 session should not meet a surprise required check mid-milestone).
- The Q1-decided croft ADR ("foundation-vs-feature-core layering", plan:205–206) has one more
  tenant to place: croft doctrine says "**Calling is a capability, not a pond**"
  (croft/CLAUDE.md) while the skeleton carries `core/call-core` (.gitkeep, verified). The ADR
  that introduces the substrate-vs-pond layer should say where call-core sits in it, or the
  repo's architecture record contradicts its own tree the day the first real crate lands.

### C1 — CONFIRM: the Pass-2 repo-state and machinery claims all check out

**verified-by-me**, item by item: croft `core/`(incl. `call-core`/`feed-core`), `ports/`,
`shell/` contain only `.gitkeep` files; no `.github/workflows/`; no root `Cargo.toml`;
`rust-toolchain.toml` pins 1.97.1 with `wasm32-unknown-unknown` + both android targets +
clippy/rustfmt (croft/rust-toolchain.toml). meer-queue pins `openmls = "=0.8.1"` with the three
companions exactly (meer-queue/Cargo.toml:34–37). Real `ed25519-dalek` in
`Proofs/lineage-groups/crates/lineage-core` (its Cargo.toml:12) with the conformance crate
beside it; the 66/0 suite is the spec's own record (part-2:1681). The `Verifier`/`Signer`
boundary exists (traits.rs:75, 88). The two CI rules the plan names as "most often missed"
(plan:119–122) are CI-PATTERN rules 1 and 7 verbatim in substance. GroupState already carries a
version byte (0x01) and the `comparator_version`/`needs_rebuild` rebuild path exists
(fold_derived.rs:180, 1933, 1959) — so "wire bumps to v2" has a socket to bump.

### C2 — CONFIRM: Phase 1's CONTESTED scope is exactly canonical §7.3.2 as merged — nothing weaker, nothing invented

**verified-by-me** against part-2:1066: set-valued ("the conflicting pair **as data, in a set**"
↔ plan "set-valued and pair-carrying"), pair-as-data, the resolution fact type, the projection's
third state, arrival-order invariance as the pin, and the structural-failure claim is true in
code — `ForkStatus::Contradiction(TypesHash)` is a single promoted label slot
(fold_derived.rs:170; one `fork_status` per `GroupState`, :197), so two simultaneously open
contradictions are unrepresentable today. The single deliberate narrowing — no compat shim,
pre-1.0 — matches the workspace's stated backwards-compat default. The one genuinely open edge is
R4 (the spec's own gap, not an invention by the plan).

### C3 — CONFIRM: Phase 4's surface matches the §10.2.2 A-series, and nothing leans on unearned properties or openmls-on-wasm early

**verified-by-me** at the requirement level: the P4 scope (token cross-check, merge rule, §7.3.8
stall, admission fact; invite path and token-return path end-to-end; plan:153–162) covers
A1/A2/A5 (artifacts + proposal gate, behind the port), A3/A6 (the decision + standing-at-head +
finality gate), A7 (refusal shapes per S22/S25), A8 (the fact-or-refuse rule) — and §10.2.2's own
split ("a substitute … replaces the artifact column, never the requirement column",
part-2:1837) licenses exactly the plan's KeyLayer cut: MLS artifacts behind the port, the
decision layer in the core. Honest-grading holds where the prompt told me to press: P4's done
states "transport loopback = Modeled, never Verified" (plan:161–162); §5 disclaims §11.11 sizing
(plan:198–199); gap-completeness is nowhere claimed (the head-currency modules move as code, not
as a discharged beam); openmls-on-wasm is `[confirm]` and nothing before P7 needs it (P6 is a
native TUI; the wasm gate is compile-only on the core). **judged**, one nudge for the P4 ADR:
write the A3 invariant into it explicitly — the admission *decision* is core-side and the
KeyLayer port carries artifacts only; a port shape that let the adapter answer "admit?" would
recreate S16's failure one layer up.

### C4 — CONFIRM (with a correction the plan should absorb): Phase 3's premise is right, but the harvest is closer than the plan points

**verified-by-me.** The rung claim is accurate for the evidence that carries it: the C-series ran
on "a deterministic mock scheme over a `compute_hash` digest, not ed25519"
(C-SERIES-RESULTS.md, fidelity note), and the substrate's own suite uses the XOR `MockSigner`
(traits.rs:148–213). But real Ed25519 already runs on the fold path in the corpus:
`social-graph-core` ships `Ed25519Signer`/`Ed25519Verifier` implemented "directly over
`ed25519-dalek` (Pass-3 decision — no `lineage-core` …)" (social-graph-core/src/crypto.rs:1–6),
and the croft-chat behavior tests fold through `Ed25519Verifier`
(fold_ordering_keys.rs imports it). The plan's P3 points only at the Proofs crates
(plan:145–148, 261–262). **judged:** P3 is even cheaper than planned — it is largely relocating
`crypto.rs` into/beside the core and swapping the mock out of the substrate-side and meer-side
suites — and the plan's problem statement ("the governance plane runs at Modeled rung (signature
stand-ins)", plan:43–44) should be scoped to *the evidence artifacts*, since the corpus already
contains a real-signature fold path. No behavior change to the phase; the text and the pointer
should change.

### C5 — CONFIRM: the dependency reversal is workable at the stated cadence, with one re-export nit

**judged** (mechanics verified where checkable): pin-bumps "at phase gates when the corpus
re-runs" (plan:131–132) align with each phase's Done-when; the tight loop stays intra-croft
because the migrated tests travel. Two notes: meer-queue consumes the substrate as a dev-dep for
HeadAck (STATE-AND-NEXT, S25) and `head_ack.rs` is core-bound (zero redb refs) — have the redb
adapter re-export the core so existing path-dep consumers keep one hop; and cargo git deps on the
private croft remote must resolve through the `github-personal` SSH host (workspace git-identity
rule) — worth one line in P2 so the first pin doesn't cost an afternoon.

### C6 — CONFIRM: the 214-green baseline and the C-series/S-series records are internally consistent

**verified-by-me as records** (not re-run): plan Pass 3 (plan:279–282) matches E117's row;
C-SERIES-RESULTS carries C2/C3/C5 fold-side and points at meer-queue for S23–S26+C4;
STATE-AND-NEXT's closing sections match the plan's evidence claims arm-for-arm, including the
honest-rung block ("Loopback is **Modeled**, never **Verified**"). The owner's fresh-eyes quote
in the Review Log is marked paraphrase in the source prompt (verified).

---

## 3. Per-lens verdicts

**L1 — Philosophical fidelity: holds, with two watch-items.** The vendor-neutral split survives
the croft landing as priced: the §9 vectors and Proofs stay the neutral bar in discovery, the
croft crate is "the product realization measured against them" (plan:133–135), which is exactly
§10's requirement-vs-realization discipline. No phase introduces a center, an authority-bearing
helper, or a wall-clock dependency — with two edges: the resolution fact could become a
one-signature verdict if its authorization is improvised (R4), and the substrate currently signs
a wall-clock assertion into every envelope (R2/O9) — both pre-existing, both cheapest to fix at
exactly the re-cut the plan schedules. The extraction-not-rewrite thesis respects provenance in
the ledgers but misses the spec-citation edge (R5) and overstates evidence transfer for mutation
(R6). One **judged** structural note the crate's own docs should carry: the implementation
realizes §7.3.1's layered order *differently than the spec narrates it* (sequential replay in
lamport→hash order plus projections and hard-stops, rather than an explicit tiered fold — G1's
withdrawn-alarm analysis is the equivalence argument); a short spec-key→mechanism mapping in the
core's docs keeps that equivalence from becoming folklore (O-adjacent, see L6).

**L2 — Spec fidelity: exact where it must be.** P1 is canonical §7.3.2-as-merged with nothing
invented (C2); P4 is the A-series with the right requirement/realization cut (C3); the plan
leans nowhere on gap-completeness or §11.11 sizing and says so (plan:198–199); rung statements
are per-plane and Modeled-honest throughout the Done-whens. The two spec-side gaps the plan will
meet are named in this review rather than papered over: the resolution fact's authorization
(R4) and — inherited, not created — the fact that the §7.3–§7.5 fold vectors are not yet in the
conformance suite (part-2:1681), so P2's "measured against the vectors" posture is §4/§5/§6-scoped
until the `[gates-release]` encodings pin (O1 keeps the socket warm).

**L3 — Technical soundness: sound conclusion, mis-stated basis.** The extraction is feasible and
bounded, but because the pure functions already exist — not because the redb references are few
(R1). The full impurity inventory (tokio, SystemTime, signed timestamp, proptest placement,
error taxonomy, surface.rs's side) must be Phase-2 text (R2). The test-migration line is
drawable in practice — the behavior pins are cleanly identifiable — but carries the citation
(R5) and mutation-transfer (R6) obligations. The dependency reversal is workable (C5). CI
scaffolding matches CI-PATTERN's two most-missed rules and should ratchet into croft's own
G5–G7 (R7). The wasm gate is achievable precisely because the core-bound modules are already
dependency-clean (verified: zero redb/tokio/clock refs in types/traits/head_ack/head_currency/
horizon/completeness_ahead) — the breakage candidates are all in the R2 inventory, none in the
fold logic itself.

**L4 — Architecture against the field.** *atproto:* the plan is on the proven side of the
pattern the corpus already borrows — canonical local data, derived read views as helpers
(§11.9.2's AppView-shaped cache states the helper-not-authority property explicitly). The
atproto mistake we are closest to repeating is reference-implementation gravity: when the only
maintained realization lives in the flagship product repo, the neutral spec starts trailing the
shipped code; the countermeasures are exactly the ones the plan half-has — vectors stay neutral
(C1), citations stay live (R5), and the profile sheet stays the declaration of record. *Matrix:*
same skeleton, opposite spine (App C.2); on the client side Matrix spent years with per-platform
SDKs drifting (including E2EE divergence) before consolidating into one shared Rust core with
thin bindings — the ADR starts where Matrix arrived, which is the strongest external validation
the plan has. The Matrix client trap we are closest to: resolution complexity leaking into views
as untyped states (rooms showing different memberships mid-resolution). `CONTESTED` as a typed
third state is the right countermeasure — keep it total (no boolean convenience accessor on the
membership projection, ever) or the Matrix lying-UI class returns through a helper method.
*Server-ordered MLS products (Wire/Webex-class):* the DS buys them total commit order — trivial
client retry, no fork machinery, a cleaner FS story. Drystone pays instead with retained-key FS
relaxation (owned, App A.1/§11.7), fork/contradiction machinery in the core, and commit-race
handling at every client. The plan locates those costs honestly (core + E116 renderings), with
one unpriced sliver: the P6 MVP list has no racing-commit UX item (two concurrent admissions;
the protocol side is S-measured, the lost-race rendering is not) — cheap to add to the Q4 list
or explicitly defer. *Local-first CRDT stacks (Automerge-class):* they would applaud
snapshot-is-cache (§7.3.3 is their compaction lesson specified up front) and the storage-adapter
split. Their scars we are nearest: a too-fine-grained storage API that dies on mobile write
amplification (design the Store port around append-batch + snapshot-load from day one, not
per-key ops), and wasm packaging discovered late (the P2 compile gate is the right cheap move —
add the no-default-features arm, O8). *Functional-core client patterns (Elm/redux/Crux/uniffi):*
effects-as-data and ports-held-by-shell are the pattern's core and the ADR states them
correctly. The known failure modes at product scale — effect explosion, port sprawl, state
duplicated between core model and platform view-stores, FFI chattiness — all concentrate at one
novel joint this plan creates: *two stacked Elm-shaped cores* (substrate + pond), which is the
piece with the least prior art anywhere. The composition rule (how a pond's effect enum embeds
the substrate's; who owns the substrate's model) should be fixed once, in the P2 ADR, not
re-derived per pond (O7).

**L5 — Direction: right, including the timing.** croft-home now is coherent with the repo's own
ratchet (G6 literally triggers on "the first core lands") and with the client architecture being
the named next front; the priced consequences were verified and three unpriced ones found
(license R3, gate-ratchet/M4 coordination R7, citation hygiene R5) — all cheap, none
direction-changing. E108-before-extraction is right (cutting a crate around a schema you know is
wrong copies debt — and P1's storage-free discipline is a genuine down payment on P2).
Signatures-before-join is right and cheaper than planned (C4). Missing-phase scan: persistence
versioning has an embryo (comparator stamp + rebuild path) that P2 should name as an adapter
deliverable rather than leave implicit; the key-custody/recovery seam should be *named* in the
P4 KeyLayer ADR so the port shape doesn't foreclose §7.3.9's pluggable backup targets (recovery
is Design/pending — naming the seam costs a sentence); a lightweight threat pass belongs before
P6's first non-loopback demo (E112 keeps the deep serve-signature analysis, fine — but P6 wires
real iroh and identity, and "loopback-grade, stated" should not quietly become "demoed to
someone"). P6-as-TUI is the right vehicle for *evidence* under Q5's decision; its known cost —
the four E116 renderings remain product debt afterward, TUI renderings do not transfer to
Kotlin/Swift — is already stated honestly ("partially landed by Phase 6", plan:27–28).

**L6 — Cheap now, big later: ten concrete moves, §5.** The two highest-leverage are O1
(conformance vectors wired as croft CI fixtures now, with the honest §4/§5/§6-scope note) and
O8 (turn the core's purity doctrine into lints and CI arms, so "no clock, no async" is enforced
by the machine that will outlive the doctrine's authors).

---

## 4. What I did not check

- **No suite was re-run.** The 214-green baseline (115 croft-chat + 99 substrate), the 66/0
  conformance count, and every S/C-series verdict are accepted as records; no finding above
  depends on doubting them.
- **No wasm build was attempted** (P2 will); my wasm claims are dependency-tree reasoning, not a
  compile.
- **Test code depth:** I read G1's harness and headers, the traits mocks, and the fold's
  public surface; I did not read the S12–S26 or C-series test bodies — their characterization
  rests on TEST-LOG/STATE-AND-NEXT/C-SERIES-RESULTS plus the 2026-08-17 independent review that
  already audited those claims item-by-item.
- **RFC 9420/9750 claims** are taken from the spec's own Verified-RFC tags, not re-checked
  against the RFCs.
- **Part 1 was read via its map, §2.0.1, and the principle sections the findings touch**, not
  line-by-line in full; Part 2 sections outside the commissioned list were skimmed by heading.
- **openmls-on-wasm upstream status**: not investigated (the plan's `[confirm]` stands as the
  correct posture; nothing before P7 needs it).
- **The L4 platform comparisons** (Matrix client history, Wire/Webex DS behavior, Automerge
  storage lessons, Crux/uniffi practice) rest on my general knowledge as of early 2026, not on
  fresh primary-source pulls this session; treat them as judged context, not verified findings.
- **The relay/M4 track** was not touched beyond reading croft's git log for the R7 coordination
  finding.

---

## 5. Opportunities (the L6 register — small, named, outsized later)

- **O1 — Conformance vectors as croft CI fixtures from P2.** Wire the §9 vector set
  (`alpha/Proofs/lineage-groups/crates/conformance`, 66/0) into the croft gate as fixtures now,
  with the honest scope note the spec itself carries: the suite covers the §4/§5/§6 layer; the
  §7.3–§7.5 fold vectors land when the `[gates-release]` encodings pin (part-2:1681). The win is
  the *harness existing* on the day those vectors appear — and P3 gets the signed-preimage
  vectors for free.
- **O2 — Version-byte discipline as a register.** GroupState's leading version byte and the
  comparator stamp already model the pattern (fold_derived.rs:180, types.rs "version: 1 byte").
  Make it a rule in the core: every serialized artifact opens with a version byte, every
  `from_bytes` refuses unknown versions loudly, and one in-crate register file lists them — the
  `[gates-release]` wire-freeze then has sockets everywhere instead of an archaeology project.
- **O3 — The order-independence proptest as the standing harness.** It exists
  (fold_derived.rs:2785–2913, permutation proptest; `convergence_property.rs`/
  `guard_property.rs` in croft-chat) — migrate it as a named, standing CI arm and extend P1's
  6-permutation CONTESTED pin into it, so every future fold change is permutation-tested by
  default rather than by authorial virtue.
- **O4 — Fuzz the `from_bytes` surfaces.** `GroupState::from_bytes`, envelope decode, and every
  future wire stub: cargo-fuzz targets are an hour each, and the crate is graduating from
  experiment inputs to product inputs.
- **O5 — API stability markers from day one.** Sealed traits, `#[doc(hidden)]` on
  not-yet-contract surfaces, `#![warn(missing_docs)]` (the rust-enforcer floor) — pre-1.0
  breakage stays cheap only while the public surface is deliberate; a git-pinned downstream
  makes accidental API surface permanent faster than a path dep ever did.
- **O6 — The profile sheet as a typed struct.** The implementation-profile dials (door,
  issuance, lifetime, serve posture, temperament) as a `Profile` config type in the core, with
  Croft's reference column as a named constructor. The conformance declaration becomes
  executable, and dial values stop being scattered constants the day a second charter differs.
- **O7 — Fix the effect-composition rule in the P2 ADR.** The substrate core and pond cores are
  both update/effect-shaped; define once how a pond embeds substrate effects (wrapper enum,
  mapping function, who owns the substrate model). This is the Elm `Cmd.map` lesson and the one
  joint in this architecture with no prior art to copy — deciding it per-pond later is how port
  sprawl starts.
- **O8 — Enforce purity mechanically.** Alongside the wasm32 `cargo check` gate: a
  `--no-default-features` check arm, and clippy `disallowed-methods`/`disallowed-types` for
  `SystemTime::now`, `Instant::now`, and tokio types in the core crate. Doctrine ("if a core
  needs the time, it is an effect") becomes CI, which is the only form of doctrine that
  survives contributor turnover.
- **O9 — Decide the envelope's wall-clock field at the re-cut.** `timestamp: u64` sits inside
  the signed canonical bytes today (types.rs:251–252, 298). Either drop it from the core's
  envelope or fence it explicitly (display-only assertion, never an ordering or policy input) —
  plus a standing test that no comparator consults it. Cheapest now, while P1 is already bumping
  the schema; it also removes a signed metadata leak nobody has argued for.
- **O10 — §11.11 measurement hooks behind a trait.** Fold depth, facts-folded, snapshot size,
  contradiction counts as a cheap `Metrics` port (no-op default). The M1/M2 measurements and the
  §11.10.1 experiment matrix then instrument instead of re-plumb — and the hot-N ceiling work
  inherits a core that was always countable.

---

## 6. Disposition

**No BLOCKER. Execution can begin on the owner's go once the REVISEs are folded or waived**, with
the natural split: R4 is the only finding touching Phase 1 (pre-declare the stop); R1, R2, R3,
R5, R6, R7 are Phase-2-and-later plan-text amendments that do not gate Phase 1's start. The five
Pass-3 owner decisions were treated as standing throughout; nothing above asks to re-open any of
them, and the readmission arc was not re-entered.
