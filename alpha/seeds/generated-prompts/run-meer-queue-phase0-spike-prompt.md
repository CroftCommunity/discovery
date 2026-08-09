# Prompt: build and run the meer queue Phase-0 spike

Copy this into a fresh session to execute Phase 0 of the meer lane. It is a **thin pointer** by
design — the spike's claims, method, and pass/fail lines live in `SPIKE-SPEC.md`, and the rules
that bind it live in the methodology doc and `CLAUDE.md`. Follow those; do not re-derive them here.

Written 2026-08-07, at commit `fa12eb8` in `discovery` (E91).

---

You're in the CroftC workspace (`/Users/cpettet/git/chasemp/CroftC`). I want to build and run the
**meer queue Phase-0 spike** — the discovery phase for the MLS store-and-forward meer.

ORIENT FIRST (read in this order, they are the source of truth):

1. `discovery/alpha/experiments/meer-queue/SPIKE-SPEC.md` — **the spec you are executing.** Two
   must-pass claims (M1, M2) and eight shape-learning scenarios (S1–S8), with the registered
   stand-ins and the falsification criteria.
2. `discovery/alpha/thinking/meer-as-custodian-queue.md` — the hypothesis the spike tests, and why
   the meer needs no fold, no ordering, and no store of its own.
3. `discovery/alpha/plans/2026-08-07-meer-lane.md` — where Phase 0 sits, and what it is meant to
   inform (the CISS typed-chain substrate is Phase 1 and is deliberately **not** built yet).
4. `discovery/beta/impl/delivery-layer/08-experiment-methodology.md` — the fidelity rules this run
   is bound by. Non-negotiable: state the fidelity rung in every verdict line (a bare `CONFIRMED`
   is inadmissible), never substitute a stand-in for the component a claim is about, pin and print
   resolved versions, **do not assert an API shape from memory** — read the crate docs first, and
   treat a FALSIFIED result as a success.

TASK — build the spike and run it. Test-first per `CLAUDE.md` (RED before GREEN, watch it fail).

BUILD NOTES that will save you time (verified 2026-08-07, re-check before relying on them):

- **Extend, do not rebuild.** `discovery/alpha/experiments/iroh/crates/mls-welcome-over-iroh`
  already creates a real OpenMLS group and carries a real Welcome across a real iroh connection on
  a real relay, with the joiner deriving the same exporter secret. That is the ancestor of this
  spike. `mls-replant` is the second reuse source for group construction. **You do not need to
  write an MLS client.**
- **The MLS library is OpenMLS**, already pinned in the workspace: `openmls = "=0.8.1"`,
  `openmls_rust_crypto = "=0.5.1"`, `openmls_basic_credential = "=0.5.0"`,
  `openmls_traits = "=0.5.0"`. The round-2 delivery experiments used `mls-rs 0.55.2`, but **that
  code does not exist in this workspace** — do not go looking for crates `e6`–`e11`, they are not
  here. Note the library difference in the results; do not cross-compare without saying so.
- **Run against plain CISS as it exists today.** No custodian chain mode, no chain kinds. The three
  stand-ins are named in the spec; tag them in code as
  `SPEC-DELTA[<id> | <kind>]: …` per the convention in
  `discovery/alpha/experiments/SPEC-DIVERGENCE-REGISTER.md`, and **add a register row for each tag
  when the code lands** (rows and tags must correspond; that is why they are not registered yet).
- **The meer must never re-frame.** M2 is the claim that byte-identical forwarding holds. Any
  decode/re-encode in the forwarding path defeats the thing under test — the only place that is
  allowed is M2's deliberate negative arm.
- **CISS's `MAX_OBJECT_BYTES` is 2 MiB, refused on put *and* get** (`CISS/src/blobstore.rs`), with
  axum's `DefaultBodyLimit` capping bodies at the same figure. It is load-bearing (it came from the
  2026-08-03 security review's memory-exhaustion finding), so do not raise it. **S8 measures where
  real MLS objects cross it** — that measurement may reshape the design, so run it properly rather
  than treating it as a footnote.

REPO / PROCESS RULES:

- **`discovery/` is the single active repo.** The standalone `experiments/` and `Proofs/` repos at
  the `CroftC` root are **frozen and archived** (PLAYBOOK §3b); their corpora live folded under
  `discovery/alpha/`. Everything you write goes under `discovery/alpha/experiments/meer-queue/`.
- Git identity is the chasemp account (`Chase Pettet <chase@owasp.org>`, `github-personal`), already
  set on the repo. **Do not commit or push unless I ask** — this repo set is reviewed before commit.
- The Claude-in-Chrome extension is disabled here; use Playwright if you somehow need a browser
  (you should not for this).

ON COMPLETION:

- Verdict lines **with fidelity rungs** into `discovery/alpha/experiments/meer-queue/TEST-LOG.md`.
- SPEC-DELTA rows into `discovery/alpha/experiments/SPEC-DIVERGENCE-REGISTER.md`.
- Fold falsifications back into `discovery/alpha/thinking/meer-as-custodian-queue.md`, and flag
  anything that touches normative text for `discovery/beta/drystone-spec/`.
- Tell me plainly what the spike **taught** us about the shape — especially S4 (does deliver-once
  actually depend on the device group being present) and S8 (where does `Welcome` cross 2 MiB) —
  before proposing Phase 1.

Start by reading the four orientation docs and telling me your plan. Do not start writing code
until I have seen it.
