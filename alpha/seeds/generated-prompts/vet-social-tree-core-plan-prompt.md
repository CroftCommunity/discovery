# Handoff prompt: independent vet of the social-tree-core plan

`Written 2026-08-20, after the plan passed Draft → Pass 2 → Pass 3 and reads READY FOR`
`EXECUTION. The owner is deliberately holding execution for this review: "sometimes when you're`
`too close to the problem … you miss obvious flaws" (owner, 2026-08-20, paraphrase). Copy`
`everything below the line into a fresh session.`

---

You are an **independent reviewer**. You did not write the plan under review, and your job is
to find what its authors are too close to see. Do not extend it, do not execute it, do not
polish it — vet it.

**The plan:** `alpha/plans/2026-08-20-1-plan-social-tree-core.md` (all paths relative to
`discovery/` unless noted). Seven phases: E108's `CONTESTED` build in the existing substrate;
extraction of a pure `social-tree-core` crate landing at `croft/core/social-tree-core`; real
Ed25519 on the governance fold; the MLS join behind a `KeyLayer` port; the chat tenant onto the
core at `croft/core/chat-core`; the TUI client build-out; product-shell adoption named as a
successor plan.

**The owner's charge, in four parts:** (1) vet the plan against the corpus — spec Part 1, spec
Part 2, and whatever else in discovery matters — **philosophically and technically**; (2) think
through this architecture **in relation to the architectures of comparable platforms**; (3)
judge **whether the direction is right**; (4) name **small things done now that buy big wins
later**.

## Posture (non-negotiable)

- **Refute-first.** For every load-bearing claim the plan makes, attempt to refute it against
  the corpus or the code before accepting it. A claim you could not check is reported as
  unchecked, never as confirmed.
- **The plan's text is not evidence.** It cites tests, line counts, and repo states — verify
  the ones your findings rest on. Greps and file reads are cheap; use them freely.
- **Decisions that stand.** The readmission arc is closed (do not re-open it), and the five
  Pass-3 answers in plan §6 are owner decisions. You may flag their *consequences* — including
  hard — but relitigating a decision requires a BLOCKER-grade finding, not a preference.
- **Quotes are verbatim with `file:line`, or explicitly marked paraphrase.** History: a
  synthesized quote in an earlier generated prompt became a false review finding. Spot-check
  any quote this prompt itself contains before leaning on it.
- **Honest grades.** Loopback is Modeled, never Verified; the Appendix-B gap-completeness beam
  is undischarged; a projection is not a measurement. If the plan's grading slips anywhere,
  that is a finding.

## Read in this order

1. The plan itself, including its Review Log (Pass 2 findings, Pass 3 closures).
2. **Spec Part 2** (`beta/drystone-spec/part-2-certifiable-design.md`), at minimum: §7.3–§7.5
   (the fold and its ordering keys), §7.3.2 (`CONTESTED`, set-valued pair-carrying schema),
   §9 (conformance), §10.2.2 (the A-series admission interface), §10.5, §6.4 (sealing/E96),
   §11.6–§11.8 (dormancy, token, standing), Appendix B (the gap-completeness beam).
3. **Spec Part 1** (`beta/drystone-spec/part-1-reasoning-underpinnings.md`): the `P-*`
   principles and the razor. The plan must not quietly violate what Part 2 exists to realize.
4. `beta/drystone-spec/implementation-profile.md` — the Croft reference profile the core must
   be able to satisfy.
5. `thinking/app/client-architecture-adr.md` and `COHESION.md` §23 — the client architecture
   the plan claims to be completing.
6. **The code the plan re-cuts:** `alpha/experiments/local_storage_projection/src/`
   (`fold_derived.rs`, `traits.rs`, `governance.rs`, `tables.rs`, `surface.rs`);
   `alpha/experiments/croft-chat/` (three-crate layering; README states the
   substrate-vs-tenant thesis); `alpha/experiments/croft-group/crates/group-core` (the module
   template the plan harvests); `alpha/experiments/meer-queue/` (the MLS half, S23–S26 tests);
   and the croft repo skeleton (`CroftC/croft/`: README, `rust-toolchain.toml`, the `.gitkeep`
   state of `core/`/`ports/`/`shell/`).
7. **The ledgers:** `alpha/ROADMAP_TODO.md` rows E19, E108, E110, E111, E112, E116, E117;
   `alpha/experiments/meer-queue/STATE-AND-NEXT.md` (the 2026-08-19/20 closing sections);
   `alpha/experiments/meer-queue/TEST-LOG.md` (S16, S18, S21–S26);
   `alpha/experiments/local_storage_projection/C-SERIES-RESULTS.md`.
8. Orientation and law: `AGENTS.md` (root); `../.claude/CI-PATTERN.md` → `croft-pwa/docs/CI.md`
   (the CI rules the plan's Phase 2 must satisfy); the workspace rules in
   `../.claude/CLAUDE.md` (dependency-sourcing: ours = git dep pinned to a commit;
   repo-content-belongs-in-its-repo).

## The six lenses (structure your review by these)

**L1 — Philosophical fidelity (Part 1).** Does landing the protocol substrate inside a product
repo threaten the vendor-neutral protocol/product split the corpus is careful about (Part 2
§1.3, §9; the conformance vectors staying in discovery)? Does any phase quietly introduce a
center, an authority-bearing helper, or a wall-clock dependency? Does the plan's "extraction
and merge, not rewrite" thesis respect provenance (evidence stays attached to what it proved)?

**L2 — Spec fidelity (Part 2).** Is Phase 1's CONTESTED scope exactly canonical §7.3.2 as
merged (set-valued, pair-as-data, resolution fact type, arrival-order invariance) — nothing
weaker, nothing invented? Does Phase 4's admission surface match the §10.2.2 A-series
(A1–A8) — in particular A3 (validity MUST NOT imply admission), A6 (standing-at-head +
§7.3.8), A7 (no per-member prompts), A8 (admission fact or refuse)? Where does the plan lean
on properties the spec marks unearned (gap-completeness; §11.11 sizing) — and does it say so?

**L3 — Technical soundness.** The extraction mechanics: the plan sizes redb entanglement by
reference counts — is that honest about the *structural* work (the fold currently operates
against storage transactions; a pure core holds state differently)? The test-migration split
(core tests to croft, adapter/C-series arms stay) — is the line drawable in practice? The
dependency reversal (discovery consumes a pinned git dep) — is the pin-bump cadence workable
or a friction trap? Phase 2's CI scaffolding against CI-PATTERN's two most-missed rules. The
wasm claims: `wasm32-unknown-unknown` for the *core* is a compile gate — what in the intended
core tree could break it (rand, getrandom, time)? openmls-on-wasm is `[confirm]` — is anything
in the plan accidentally load-bearing on it earlier than Phase 7?

**L4 — Architecture against the field.** The shape is: pure per-domain cores + a shared
substrate core, ports held by shells, per-platform shells; a governance fold with CRDT-adjacent
order-independence; MLS without an ordering DS. Compare deliberately against: **atproto**'s
PDS/AppView split (the feed pond already rides it; §11.9.2/§11.9.3 borrow the pattern);
**Matrix** (Part 2 Appendix C.2 claims same skeleton, opposite spine — does our client
architecture avoid Matrix's known client-complexity traps?); **server-ordered MLS products**
(what do they get for the DS we refuse, and does the client architecture pay that cost
somewhere visible?); **local-first CRDT stacks** (Automerge-class: what did they learn about
storage adapters, compaction, and wasm builds that we are about to relearn?); **functional-core
client patterns** (Elm/redux-shaped cores, uniffi mobile practice: known failure modes of
"pure core + effects at the edge" at product scale — effect explosion, port sprawl, state-sync
between core and platform stores). For each: one paragraph — what they'd say our plan gets
right, and the one mistake of theirs we're closest to repeating.

**L5 — Direction.** Is croft-home right *now* (the owner's call — evaluate consequences and
timing, not the decision)? Is the phase order right — in particular E108-before-extraction,
and signatures-before-join? Is anything missing as a phase: persistence-format versioning and
migration story, key custody/recovery seams, a security review gate, the §11.11 measurement
hooks, error taxonomy, observability? Is Phase 6 (TUI client) the right vehicle for the MVP
list, or is effort better spent closer to the product shells?

**L6 — Cheap now, big later.** Concrete, small, named moves with outsized future payoff.
Candidates to evaluate (add your own): conformance vectors wired as CI fixtures in croft from
Phase 2; wire-encoding stubs behind a version byte everywhere state is serialized (the
`[gates-release]` register is coming — cheap to leave sockets for it); property tests on the
fold's order-independence as a standing harness; a `no_std`-adjacent discipline check;
fuzzing hooks on `from_bytes` surfaces; API stability markers (`#[doc(hidden)]`, sealed
traits) so pre-1.0 breakage stays cheap; the profile sheet's dials as a typed config struct
from day one.

## Verification budget

Reads and greps: unlimited. Test suites: the recorded baseline is **214 green** (croft-chat
workspace 115/0; local_storage_projection 99/0; both run fresh 2026-08-20) — re-run only if a
finding depends on doubting the record (the substrate suite takes several minutes). Do not run
mutation testing; do not build for wasm (Phase 2 will); do not touch the relay/M4 track (a
concurrent session owns it).

## Output

Write **`alpha/plans/REVIEW-social-tree-core-plan-2026-08-20.md`** (precedent:
`beta/drystone-spec/REVIEW-decision-talkthrough-2026-08-17.md`):

1. **Plain-English owner summary first** — the verdict in one short paragraph: right
   direction or not, safe to execute or not, the one thing you'd change.
2. **Findings**, ranked, each tagged: **BLOCKER** (execution should not start) / **REVISE**
   (plan text or phase content should change first) / **CONFIRM** (checked and holds) /
   **OPPORTUNITY** (the L6 material). Every finding carries its evidence as `file:line`, and
   every claim is tagged **verified-by-me** or **judged** — never blended.
3. **Per-lens verdicts** (L1–L6), one paragraph each.
4. An honest **"what I did not check"** section.

Rules of engagement: review artifact only — do not edit the plan, the spec, or any code; do
not execute any phase. Work in a worktree (`git -C discovery worktree add
../worktrees/discovery/<name> -b <branch>`) — concurrent sessions `git add -A`, never work in
the shared tree. Do not commit or push without the owner's go-ahead. Plain-English summaries
for the owner; the technical detail belongs in the review body.
