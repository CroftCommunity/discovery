# Handoff prompt: integrate the two social-tree-core reviews into the plan

`Written 2026-08-20, after the independent vet and the owner-directed alignment pass both`
`landed. Audience: the session that authored the plan and holds its context. Copy everything`
`below the line into that session.`

---

The independent vet of `alpha/plans/2026-08-20-1-plan-social-tree-core.md` you commissioned is
complete, a second owner-directed wider-angle pass is also complete, and the owner has made the
decisions recorded below. Your job now: **talk the findings through with the owner where a call
is still needed, fold the accepted amendments into the plan, and clear the E117 execution
hold.** Integration, not re-review — do not re-litigate what either review confirmed, and do
not re-open the readmission arc or the five Pass-3 answers (both reviews honored them; so must
this pass).

## The artifacts (read both in full before editing anything)

Both reviews are committed on a worktree branch — NOT yet merged to the shared tree:

- Worktree: `CroftC/worktrees/discovery/review-social-tree-core`
  (branch `review/social-tree-core-2026-08-20`, created per the concurrent-sessions rule)
- **The independent vet:** `alpha/plans/REVIEW-social-tree-core-plan-2026-08-20.md` —
  refute-first, every claim tagged verified-by-me/judged, findings ranked
  BLOCKER/REVISE/CONFIRM/OPPORTUNITY. Verdict: **right direction, safe to execute, no
  BLOCKER; seven REVISEs; ten opportunities (O1–O10).**
- **The alignment companion:** `alpha/plans/REVIEW-social-tree-core-alignment-2026-08-20.md`
  — program-level fit (calling track, relay, identity, estate, spec trajectory), five further
  opportunities (O11–O15), sequencing advice.

This prompt is the map, not the evidence — the reviews carry the `file:line` support; quote
from them verbatim or mark paraphrase (the standing rule). Part of this integration is
deciding, with the owner, how the review branch lands (merge, or copy the files into the
shared tree and drop the worktree) — do not commit or push without the owner's go-ahead.

## Decisions the owner has already made (settled inputs — record, do not re-ask)

1. **License (closes the vet's R3): AGPL-3.0 for everything Drystone/Croft.** The core crate
   declares AGPL-3.0; croft-chat's `MIT OR Apache-2.0` was an accident to correct at the
   pin-bump, not a position. Note in the plan that this is consistent with the standing
   2026-07-09 decision (ROADMAP_TODO A14: reference code → AGPL-3.0-or-later + DCO); A1
   (MPL-2.0 `hpke-rs`) is unaffected and stays A1's gate.
2. **Both reviews stand as input.** The owner directed the second pass explicitly and wants
   its material integrated alongside the vet's, not treated as advisory-only.

## The REVISE findings to fold (vet §2; all plan-text/phase-content, none touch a Pass-3 answer)

Walk these with the owner in plain English, one at a time, in the Pass-3 style — most need
only acknowledgment, one needs a genuine call:

- **R4 — the only Phase 1 item, and the one open owner call.** Neither the plan nor merged
  §7.3.2 specifies who may *author* a resolution fact; a single-member resolution fact would
  be a one-signature verdict on exactly the contradiction the fold refused. Options: (a)
  pre-declare this as P1's expected stop-and-file (the plan's existing spec-gap rule), or (b)
  settle the authorization shape with the owner now, before P1's RED tests. Get the call,
  record it in P1's text.
- **R1 — restate P2's sizing paragraph.** The redb counts are exact but mismeasure the work:
  the real job is a state-residency inversion (state lives in tables today, fold runs inside
  write transactions; a pure core holds state in memory and takes the log as input). Keep the
  counts as localization evidence; the vet's §R1 has the specifics and the pure-function
  inventory that keeps the extraction bounded.
- **R2 — add the impurity purge list to P2 and assign `surface.rs` a side.** tokio (broadcast
  in surface.rs; `rt-multi-thread` breaks the wasm gate), two `SystemTime::now()` sites, the
  wall-clock timestamp inside the signed envelope bytes, proptest under `[dependencies]`, the
  FoldError/storage error split. The plan must state where `surface.rs`/`LocalStore` lands
  (the vet judges: it splits — command construction core-side, orchestration adapter-side —
  but the plan should say so itself).
- **R5 — the citation rule.** Any test file cited by canonical spec text or a ledger (e.g.
  §7.3.2 cites `croft-chat/tests/fold_ordering_keys.rs` as Measured evidence) either stays in
  place as an adapter-side regression or the citation updates in the same commit that moves
  it; a grep for migrated filenames across `beta/` and the ledgers is the audit.
- **R6 — mutation re-baseline as a phase-gate item.** "Mutation-vetted" does not transfer
  across the re-cut (the X3 harness needs path deps; a git dep cannot be mutated in place).
  Schedule the re-baseline on `social-tree-core` (after P2 or folded into P3's close) and
  record the `[patch]`-override recipe for future corpus-side sweeps.
- **R7 — three small P2 amendments:** CI lands as croft's own G6/G7 ratchet firing their
  recorded triggers (croft/CLAUDE.md "Commit gates"), never a parallel workflow; name the
  coordination with the live M4 session in croft (see sequencing below); the layering ADR
  must place `call-core` (the skeleton carries `core/call-core` while croft doctrine says
  "Calling is a capability, not a pond" — the ADR should resolve that tension the day the
  first real crate lands).

Also absorb the vet's **C4 correction** (not a REVISE, but the plan text should change): real
Ed25519 already runs on the fold path in `social-graph-core/src/crypto.rs` — P3 is largely
relocation, cheaper than the plan's Proofs-crates-only pointer suggests; rescope P3's problem
statement to the *evidence artifacts*.

## The alignment companion's findings to fold (its §§4–7, O11–O15)

- **O11 — the two-admissions paragraph** in the croft layering ADR: fabric admission (the
  relay's — traffic, D3/M4) vs group admission (the A-series — membership). "The relay admits
  traffic, never members; no fabric-admission signal is an input to the A-series." One
  paragraph; forecloses the S16 failure one layer down.
- **O12 — a ROADMAP_TODO row for the DID ↔ persona-key binding seam** — where the calling
  track's identity (OAuth-proven DID) meets the core's (persona keys); Phase 7's biggest
  predictable design question. Plus the P2 discipline: no atproto types anywhere near the
  core.
- **O13 — charter-as-data as a P2 acceptance criterion:** no [charter] dial value
  (implementation-profile §2.1/§2.5) as a compile-time constant in the core; `GroupRules` is
  the existing socket.
- **O14 — the `(DeviceId, PrincipalId)` credential-pair boundary travels verbatim** through
  the re-cut (it is spec §4.5's multi-client guarantee, structurally present today); the P2
  test migration includes whatever pins it.
- **O15 — a COHESION seam-line for croft-stack ↔ meer convergence**, opened when relay
  Phases 7–8 begin (two growth paths toward "the helper that holds things for absent
  members" must not drift into two implementations).

## The opportunity register (O1–O10, vet §5) — triage with the owner

Adopt/defer each explicitly; record the disposition in the plan (adopted ones become phase
text or Done-when items, deferred ones get a named home, e.g. a ROADMAP_TODO row — do not
let them evaporate). Priority signals from the two passes:

- **O9 (the signed wall-clock envelope field) belongs at Phase 1**, not later — P1 is already
  bumping the envelope schema, so dropping or fencing the field is free now and never again;
  the companion's §3 sharpens the Part 1 §2.0.1 case.
- **O1** (conformance vectors as croft CI fixtures from P2, with the honest §4/§5/§6 scope
  note) and **O8** (purity enforced mechanically: clippy disallowed-methods on
  `SystemTime::now`/tokio types, the wasm arm, a no-default-features arm) are the vet's two
  highest-leverage.
- O2 (version-byte register), O3 (order-independence proptest as a standing CI arm), O4
  (fuzz `from_bytes`), O5 (API stability markers), O6 (profile as typed struct — note O13
  strengthens this), O7 (fix the two-stacked-cores effect-composition rule in the P2 ADR —
  the one joint with no prior art), O10 (§11.11 metrics behind a no-op port): walk the list;
  none is expensive.

## Sequencing (companion §7)

Phase 1 is discovery-side only — **it can start on the owner's go, concurrent with M4, zero
collision.** Phase 2 is the collision point (root workspace + first CI gate in croft while
the M4 session is mid-milestone): land it at a coordinated moment — after M4's current
milestone closes, or with an explicit heads-up so that session expects the new required
check. Put this in P2's text as part of the R7 amendment.

## Done when

1. Every REVISE is folded or explicitly waived-with-reason in the plan's Review Log; R4's
   owner call is recorded in P1.
2. The C4 correction and the license decision are in the plan text.
3. O1–O15 each have a recorded disposition (adopted → phase text; deferred → named home).
4. The plan's Review Log gains a dated integration entry (the Pass-2/Pass-3 style) citing
   both review files, and the status line moves from "execution holds for review" to ready —
   with Phase 1's start still gated on the owner's explicit go.
5. E117's row reflects the reviews and the new status; new rows (at least O12; O15 when its
   trigger nears) are filed rather than parallel-listed.
6. The disposition of the review worktree/branch is decided with the owner (merge vs
   copy-in), executed only on their go-ahead — nothing committed or pushed without it.

Honest-grading discipline throughout: Loopback stays Modeled, the gap-completeness beam stays
undischarged, projections are not measurements — both reviews verified the plan holds this
line; integration must not blur it.
