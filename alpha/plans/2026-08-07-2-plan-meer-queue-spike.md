# Meer queue Phase-0 spike — execution plan

date: 2026-08-07 (open questions walked, and Pass 3 run, 2026-08-08)
status: **Phase 0 (Discovery) EXECUTED 2026-08-08. Phases 1–5 GREEN 2026-08-09/10.**
All three planning passes complete, all 5 open questions resolved. Phases 6–14 not started.
**M1 CONFIRMED (real-lib) — the first must-pass claim holds.** Phase 0 falsified one hypothesis inside M2 and forced
seven plan changes — see § Phase 0 outcomes.

**Executes:** `alpha/experiments/meer-queue/SPIKE-SPEC.md` (M1, M2, S1–S8)
**Hypothesis under test:** `alpha/thinking/meer-as-custodian-queue.md`
**Lane position:** `alpha/plans/2026-08-07-meer-lane.md` → Phase 0
**Bound by:** `beta/impl/delivery-layer/08-experiment-methodology.md` (the fidelity ladder)
**Code lands in:** `alpha/experiments/meer-queue/`

---

## Problem Statement

The meer is Drystone's blind store-and-forward node — the single gating unbuilt component for "the
conversation stays alive while you sleep." The design in `meer-as-custodian-queue.md` is coherent on
paper and rests on one unexercised claim: **a node that does no ordering, holds no group state, and
holds no key is sufficient to carry a real MLS conversation across an absence.**

Everything downstream depends on that being true — the CISS typed-chain substrate (Phase 1), chain
kinds, ceilings, the gateway-service pattern. The substrate is also the piece that carries the
security-review cost, because it is the first time a non-owner can cause a write in an owner's
namespace. Building it before testing the shape would mean guessing at what it must support.

So Phase 0 tests the shape against plain CISS as it exists today, with the missing substrate pieces
registered as visible `SPEC-DELTA` stand-ins.

**Constraints:**

- Two must-pass claims (M1, M2) and eight shape-learning scenarios (S1–S8), all specified in advance
  along with what would falsify the design. The falsification criteria are already written down
  precisely so they cannot be rationalized after the fact.
- Every verdict states its fidelity rung. A bare `CONFIRMED` is inadmissible.
- Nothing about the seal may be stood in. XOR-as-MLS is the canonical forbidden move.
- The meer must never re-frame. M2 is the claim that byte-identical forwarding holds; any
  decode/re-encode in the forwarding path defeats the thing under test.
- CISS's `MAX_OBJECT_BYTES = 2 MiB` is load-bearing (it came from a real memory-exhaustion finding in
  the 2026-08-03 security review). It does not get raised to make a test pass.

**Blast radius:** none outside the workspace. This is a spike; no production code changes, no deploy,
no commits without explicit request. The one cross-repo touch is a read-only path dependency from the
spike crate to `CISS/` (Open Question 1).

## Reasoning

### Why extend rather than rebuild

Two ancestors already exist in this workspace at Rung A, and both are ours:

- `alpha/experiments/mls-replant` — standalone crate, own lockfile, pure openmls `=0.8.1`. Real group
  construction (`Persona`, `stamp`, `join`, `commit`, `apply_commit`, `membership`, `tree_bytes`,
  `leaf_keys`). No dependency on the absent `Proofs/lineage-mls` wrapper.
- `alpha/experiments/iroh/crates/mls-welcome-over-iroh` — real iroh `=1.0.0` carrying a real openmls
  Welcome across a real connection homed on a real loopback relay, joiner deriving the same exporter
  secret.

The spike depends on `mls-replant` by path for group construction and **copies** `relay.rs`/`node.rs`
from `mls-welcome-over-iroh` for the transport. Copying rather than depending is deliberate:
`mls-welcome-over-iroh` pulls in `Proofs/lineage-groups/lineage-mls`, which the spike does not need
and which would drag a second MLS wrapper into a crate whose whole point is that the meer touches no
MLS types at all.

**Correcting the hypothesis doc:** `meer-as-custodian-queue.md` § "What we test next" specifies "two
OpenMLS clients (the MIT `cli` PoC)". That is stale — it predates these ancestors. Driving the OpenMLS
reference delivery-service PoC as a subprocess would add a third-party dependency, plumbing we do not
control, and a delivery service we explicitly do not want, in order to test a design whose central
claim is that everyone else's DS is their product. The in-process ancestors are smaller, faster, and
already Rung A. This is folded back in Phase 13.

### Why the meer's blindness is structural, not asserted

The `meer` module **does not depend on openmls at all**. It takes `Vec<u8>` in and hands `Vec<u8>`
out. This makes M2's positive arm a property of the module graph rather than of a test assertion: the
meer *cannot* re-frame a message because it cannot name the type. A future edit that introduced a
re-frame would have to add an openmls dependency to `meer.rs` first, which is a visible, reviewable
act rather than a silent regression.

The negative arm needs a re-frame, so it lives in the test, not in the meer.

Alternative rejected: put a `re_frame: bool` flag on the meer and flip it for the negative arm. That
would place the forbidden operation inside the component under test, where a later mis-default could
turn it on in the positive path. Keeping the re-frame in test code makes the meer's inability
unconditional.

### Why real CISS over loopback HTTP

CISS ships **no binary** — it is a library plus an axum router, and its own test suite spawns
`App::router()` on an ephemeral loopback port and drives it over real HTTP with `reqwest`
(`CISS/tests/common/mod.rs`). The spike does the same. That gives us the real handler chain, the real
content-address round trip, the real re-verify-on-read, the real `MAX_OBJECT_BYTES` refusal, and
axum's real `DefaultBodyLimit` — the whole storage boundary M2 and S2 make claims about.

What it is *not* is a deployed instance with systemd, TLS, a real disk, or an operator. Nothing in
M1/M2/S2–S8 makes a claim about those, but the gap gets a register row anyway
(`meer-spike-ciss-inproc`) so a later reader cannot mistake "the spike ran against CISS" for "the
spike ran against a deployed CISS."

Alternative rejected: mock the storage layer behind a trait. That would stand in for the exact
component S2's dedup claim and M2's round-trip claim are about — forbidden by the methodology's hard
rule.

### Why S1 gets no code

Custodian mode does not exist. S1 asks what enrollment requires; the honest answer is produced by
inspection and enumeration, not by running something. Writing code that *simulates* enrollment would
produce a green test that proves nothing, which the methodology names as worse than no test because
it retires a question that is still open. S1 is recorded as **Rung C (static)** and says so.

### Why S4's two arms have different rungs

The without-device-group arm is Rung A: real group, real deliver-once, real prune-on-ack, and we
observe whether the second device starves. The with-device-group arm **cannot** be Rung A, because
§6.6.5 device-group fan-out is not built — and standing something in for it would be standing in for
the exact mechanism the claim is about. So the with-arm is reasoned in prose and left explicitly
open, generating a Rung-A follow-up item. The spike does not get to claim it validated the
compensating mechanism.

### The S8 prediction, recorded in advance

`mls-replant` already measured what the spike spec's S8 table lists as `~log N` for commits, and found
the sparse case is **O(N)** at ~80–130 B/member, flat across N (`mls-replant/README.md`, M1 floor);
the populated-tree case falls to O(log N) (90→52→30 B/member at N=8/16/32). So a commit is a *band*
whose floor is linear.

Extrapolating the worst measured end (~130 B/member) puts a commit at 2 MiB somewhere near N ≈ 16,000.

**Pass 3 correction — the Welcome half of this prediction was wrong, and the error mattered.** The
Pass-2 draft also cited E12.1's per-member Welcome figure (~152–155 B/member) as extrapolating
similarly. It does not, because **it measures a different configuration.**
`MlsGroupJoinConfig` derives `Default` and its `use_ratchet_tree_extension` is a plain `bool`, so it
defaults to **`false`**; `MlsGroupCreateConfig::default()` delegates to it (`config.rs:86–113`). And
`mls-replant::stamp_kps` builds every group with `MlsGroupCreateConfig::default()`. **So every
measurement the corpus currently has is the *without*-ratchet-tree case — the safe case, not the risk
case.** The O(N) object S8 exists to find — `GroupInfo` *with* the embedded tree — has **never been
measured in this workspace.** Citing the flat 152–155 B/member figure as a bound on the risk was
exactly the "prior result reused out of its configuration" trap.

Consequence: the commit prediction stands (commits do not carry the tree either way), but S8's
`Welcome`/`GroupInfo`-with-extension rows have **no prior** and no prediction. That is the honest
position, and it raises S8's value rather than lowering it.

**A second finding that reframes the S8 decision.** `mls-replant` already ships the ratchet tree
**out of band**: `stamp_kps` returns `ratchet_tree: group.export_ratchet_tree().into()` as a separate
field, and `join(persona, welcome, ratchet_tree)` takes it as a separate argument. So S8's **option 3
("ship the tree out of band") is already the corpus's de-facto default**, arrived at incidentally
rather than as a decision. That inverts the framing in the spike spec: option 3 is the status quo,
and options 1 and 2 are the departures that would need justifying. Phase 13 should say so.

**This prediction is recorded so S8 can embarrass it.** It is an extrapolation from a different
crate's measurement of a different object, not a result. It does not license skipping S8, and it does
not get cited as evidence. Its only function is that if S8 lands far from it, we know something about
group-size scaling that neither crate currently knows.

If the prediction holds, the spike spec's catastrophic branch — "application messages or ordinary
commits crossing 2 MiB → CISS needs streaming before it can be the meer's substrate at all" — is off
the table, and S8 reduces to picking among three tractable options for `Welcome`/`GroupInfo`, one of
which (ship the tree out of band) Part 2 §6.9.1 already mandates for the broadcast tier.

### Why CISS's SimClock rather than an invented seam

S5 ages a queue past its retention window. Wall-clock waiting is not runnable and `sleep`-based tests
are the standard way to produce a slow, flaky suite.

The Pass-1/Pass-2 draft planned a spike-local `Clock` trait in `queue.rs`. That was wrong, and the
Open-Question-2 severity override caught it: **CISS already has `SimClock`** (`CISS/src/clock.rs`) —
public, unit-tested, day-granularity, documented as existing so "timestamps and the byte-day rent
integral are reproducible run to run (no wall-clock reads)." That is the same problem for the same
reason at the same granularity a 14-day window needs. Inventing a parallel seam next to it would have
been the spike asserting a pattern the substrate had already chosen.

It currently has no callers in CISS — a dormant type ported from
`item-storage-protocol-standalone/src/clock.ts`. The spike is its first caller, which is a small
argument in its favor: a type with a user is a type whose behavior is checked.

The queue therefore takes `ciss::clock::SimClock` directly from Phase 3, introduced with the queue
rather than retrofitted in Phase 9. It still gets a register row (`meer-spike-clock`) — simulated
time is simulated time — but the row records the weakest form of the divergence.

Alternative considered and rejected: a real wall clock with retention configured to ~200ms, which
would eliminate the stand-in entirely and make S5 Rung A with no register row. Rejected because it
trades a fully honest, registered simulation for a test that can flake under load, and a flaky S5
would get muted or deleted long before the register row would.

### Observability: how a mid-run failure gets diagnosed

Added in Pass 3, which found the plan had no logging story at all. A spike driving real openmls, a
real axum server, and a real iroh relay in one process has three independent failure surfaces, and
"the test failed" does not say which one.

`src/lib.rs` initializes `tracing_subscriber` with an `EnvFilter` writing to **stderr**, matching
`mls-welcome-over-iroh/src/main.rs:45–48` (the ancestor's own pattern) so nothing new is invented.
That gives, for free, iroh's own connection/relay tracing and axum's request tracing — CISS already
emits `tracing::info!` at the object boundary (`server.rs:1443`, `:1469`) with method, DID, key, and
byte count, which is exactly the record S2's dedup claim and M2's digest chain need when they
disagree.

Levels: the spike's own spans at `debug` (publish, queue append, drain diff, sweep); verdict lines
and measurements at `info` so `--nocapture` runs read cleanly; nothing at `warn`/`error` that is an
expected negative-arm outcome, since M2's re-frame rejection and S7's decrypt failure are *results*,
not faults, and logging them as errors would train the reader to skim past real ones.

Every result phase prints its verdict line to stdout regardless of the filter, so a verdict is never
lost to a log-level setting.

### Registered stand-ins

Three are named in the spike spec; the plan adds two it did not anticipate. All five get code tags and
register rows in Phase 12, and tags-to-rows correspondence is asserted by grep.

| ID | Kind | Stands in for |
|---|---|---|
| `meer-spike-namespace` | stand-in | No custodian chain mode; the meer owns one CISS namespace and queues are slots within it. Spec target is per-DID queues under a custodial grant. Changes *who signs*, not the delivery shape. |
| `meer-spike-kind-gate` | absent | Chain kinds and the queue-only custodial-write gate do not exist; nothing enforces them. |
| `meer-spike-drain-auth` | stand-in | Drain scoped by iroh `EndpointId` (free from the authenticated QUIC connection) rather than CISS account identity. No multi-device-per-account auth exercised. |
| `meer-spike-clock` | test-scaffold | **New.** S5 ages the queue via CISS's own `SimClock` (`CISS/src/clock.rs`) rather than wall time. Weakest form of stand-in: the substrate's own deterministic day clock, built for exactly this, not a spike invention. |
| `meer-spike-ciss-inproc` | test-hermeticization | **New.** CISS ships no binary; the spike spawns its real axum router in-process on loopback. Real server, real HTTP, real re-verify-on-read — not a deployed instance. |

Nothing about the seal is stood in.

## Verified Assumptions

Everything below was confirmed firsthand on 2026-08-07 by reading source at the cited line, not from
memory. Anything not listed here is unverified and appears as a Phase 0 discovery task.

**Toolchain**
- Local `rustc 1.97.1 (8bab26f4f 2026-07-14) (Homebrew)`, `cargo 1.97.1` at `/opt/homebrew/bin/rustc`.
  CISS pins `1.97.1` in `CISS/rust-toolchain.toml`. **They match**, so the CI-PATTERN rule-7 skew trap
  does not apply here. Re-check before relying on it.

**CISS**
- `MAX_OBJECT_BYTES: u64 = 2 * 1024 * 1024` — `CISS/src/blobstore.rs:25`.
- Refused on put — `blobstore.rs:133`; refused on get — `blobstore.rs:230`; bounds manifest leaves —
  `manifest.rs:72`.
- No binary: `Cargo.toml` declares no `[[bin]]`, `src/bin/` does not exist. CISS is a library plus a
  router.
- Routes: `PUT|GET /{did}/objects/{addr}`, `PUT|GET /{did}/manifest`, `PUT|GET /{did}/policy`,
  `GET /{did}/meter`, `GET /{did}/du`, `GET /healthz` — `src/server.rs:362–396`.
- `App::new(seed, blobs, db)` at `src/server.rs:241`; `App::router()` at `:353`. Router holds its own
  `Arc`s, so `App` may be dropped after building it — `src/server.rs:350–352`.
- Test harness pattern: bind `127.0.0.1:0`, `axum::serve` with graceful shutdown, drive over real
  HTTP with `reqwest` — `CISS/tests/common/mod.rs:48–75`, `put_object`/`get_object` at `:335`/`:341`.
- Manifest carries a signed `heads: BTreeMap<String, String>` frontier (`device_id → cid`) bound into
  the signing preimage — `src/manifest.rs:128–140`, `:196`. This is the natural per-recipient queue
  slot under the namespace stand-in.

**CISS construction details** (added Pass 3 — these change Phase 1's shape)
- `Blobs` and `Db` are `pub enum` — `server.rs:134` / `:142`. Variants: `Blobs::Memory`,
  `Blobs::Fs(PathBuf)`; `Db::Memory`, `Db::File(PathBuf)`. So D1's reachability question is
  already partly answered: the types are public.
- **`App::new` reads ambient environment.** It delegates to `Limits::from_env()` (`server.rs:242`,
  `:176–188`), which reads `CISS_MAX_STORE_BYTES` and `CISS_MAX_DID_BYTES`. **`App::with_limits`
  (`server.rs:250`) is the documented injection point "for tests that need a small ceiling."** The
  spike uses `with_limits` with explicit `Limits`, so an ambient store ceiling on the developer's
  machine cannot perturb S2's fan-out or S8's sizes. This also corrects Phase 1's shared-state
  contract, which claimed no env involvement.
- CISS's own harness builds with `App::new("test-provider", Blobs::Memory, Db::Memory)` —
  `tests/common/mod.rs:131`; the `Blobs::Fs` variant is exercised at `:152`.

**OpenMLS 0.8.1** — source re-extracted from the cached `.crate` tarball during Pass 3 (the
registry's extracted copy had been evicted between passes; the tarball remains cached and the pin is
exact `=0.8.1`, so the citations below stand, but re-verifying them requires a `cargo build` or
`tar xzf` first)
- `MlsGroup::create_message(&mut self, provider, signer, message: &[u8]) -> Result<MlsMessageOut,
  CreateMessageError>` — `src/group/mls_group/application.rs:16`.
- `MlsGroup::process_message` — `src/group/mls_group/processing.rs:119`.
- `MlsGroup::merge_pending_commit` — `src/group/mls_group/processing.rs:307`.
- `MlsMessageOut::to_bytes() -> Result<Vec<u8>, MlsMessageError>` — `src/framing/message_out.rs:150`.
- `MlsMessageIn::extract() -> MlsMessageBodyIn`, `wire_format()`, `try_into_protocol_message()` —
  `src/framing/message_in.rs:96–116`.
- **`use_ratchet_tree_extension(bool)` is a real toggle** on both `MlsGroupCreateConfig` (`config.rs:292`)
  and `MlsGroupJoinConfig` (`config.rs:154`), readable at `:201`. This is the S8 lever for measuring
  `GroupInfo` with and without the embedded tree — confirmed, not assumed.
- **Its default is `false`** (Pass 3): `MlsGroupJoinConfig` derives `Default` (`config.rs:43`) and
  `use_ratchet_tree_extension` is a plain `bool` (`config.rs:56`); `MlsGroupCreateConfig`'s hand-written
  `Default` delegates to `MlsGroupJoinConfig::default()` (`config.rs:102–113`). **Load-bearing for S8
  — see the S8 prediction section.**

**Existing experiment crates**
- `mls-replant` — standalone, own lockfile, pins `openmls =0.8.1`, `openmls_rust_crypto =0.5.1`,
  `openmls_basic_credential =0.5.0`, `openmls_traits =0.5.0`, `tls_codec 0.4`. Public API confirmed at
  `src/lib.rs:23,34,49,61,73,96,107,141,151,158,173,192,207,225,239`. **No application-message
  helpers** — the spike adds them.
- **`Persona`'s fields are all `pub`** — `provider: OpenMlsRustCrypto`, `signer: SignatureKeyPair`,
  `cwk: CredentialWithKey`, `id: Vec<u8>` (`src/lib.rs:22–28`). **This resolves the signer half of D4
  during planning** (Pass 3): `create_message(provider, signer, msg)` can be called directly with
  `persona.provider` and `persona.signer`. D4 narrows to confirming the call compiles and round-trips.
- **`stamp_kps` hardcodes `MlsGroupCreateConfig::default()`** (`src/lib.rs:107–112`) and **discards the
  `GroupInfo`** (`let (commit, welcome, _gi) = group.add_members(...)`). So `mls-replant` can produce
  neither a tree-embedding group nor a `GroupInfo` object. **S8 therefore cannot be built on
  `stamp()`** and needs its own config-parameterized construction — a Pass 3 finding that adds
  `src/mls.rs` to Phase 11's write-set.
- **`Stamp` returns `ratchet_tree` as a separate field** and `join()` takes it as a separate argument
  (`src/lib.rs:84,207`) — i.e. the tree already travels out of band. See the S8 prediction section for
  why this reframes the S8 decision.
- `mls-welcome-over-iroh` — `iroh =1.0.0` (`test-utils`), `iroh-relay =1.0.0` (`server`,
  `test-utils`); loopback relay spawn in `src/relay.rs`, endpoint build + `ALPN` in `src/node.rs:14–57`.
  Depends on `../../../../Proofs/lineage-groups/crates/lineage-mls`, which **does** resolve
  (`alpha/Proofs/lineage-groups/crates/` exists).

**Prior measurements informing S8** (evidence, not assumption — cited as prior results, re-measured
by S8 in this crate)
- Sparse self-update commit is O(N) at ~80–130 B/member — `mls-replant/README.md` M1 floor.
- Populated-tree commit falls to O(log N), 90→52→30 B/member at N=8/16/32 — M1 ceiling.
- Per-member Welcome bytes ~flat at ≈152–155 B/member up to N=500 — E12.1.

## Phase 0 outcomes (run 2026-08-08)

Full record: `alpha/experiments/meer-queue/PHASE-0-FINDINGS.md`. Probes are runnable at
`meer-queue/src/bin/d*.rs`. Summary of what discovery established, and what it changed:

| Probe | Verdict | Plan impact |
|---|---|---|
| D1 CISS path dep | CONFIRMED | `Limits` has no `Default`; build by literal (fields are `pub`) |
| D2 real PUT/GET | CONFIRMED | **Over-cap refusal is axum's 413, not CISS's `ObjectTooLarge`** — Phase 1's wiring test corrected; `du` + on-disk `blocks/{did}/{cid}` give S2 two independent sources |
| D3 re-frame | **SPEC HYPOTHESIS FALSIFIED** | Re-encode is **byte-identical**; both re-frame conversions are `test-utils`-gated with a "MUST NOT" comment. **Phase 6 restructured** |
| D4 MLS layer | CONFIRMED | `Persona` fields `pub`; `wire_format() == PrivateMessage` assertion satisfiable |
| D5 iroh relay | CONFIRMED | Fallback not needed. Dial by `Endpoint::addr()`; `RelayPorts::ephemeral()` added; ALPN renamed |
| D6 scale | NO PATHOLOGY | ~linear; 500 members in 126 ms. Full S8 sweep is cheap |
| D7 tree extension *(added in Phase 0)* | CONFIRMED + first data | S8's construction path exists; **corpus's first tree-ON measurements** |

**The one that matters:** D3 falsified the spec's negative-arm hypothesis before the arm was written.
The `MUST` on byte-identical forwarding is not weakened — its stated *rationale* was wrong. Re-framing
is byte-preserving; the dangerous operation is **re-sealing**, which needs a key the meer does not
have. The rule turns out to be nearly self-enforcing, for a stronger and simpler reason than the spec
gave. Phase 13 folds this into the hypothesis doc and flags Part 2 §6.6.2's rationale as normative
text needing correction.

**Nothing invalidated the spike's premise.** The design remains testable as scoped.

## Documentation Impact

- `alpha/experiments/meer-queue/TEST-LOG.md` — **new.** Verdict lines with fidelity rungs and printed
  versions. Created Phase 5, appended by every result phase, S1 added Phase 12.
  **Format (settled in Pass 3):** follows `08-experiment-methodology.md` §5 — per result, in order:
  claim + backing doc section; exact resolved library versions; fidelity rung (stand-in named if B);
  code reference; raw output; one-line verdict; design consequence + any Rung-A follow-up generated.
  Deliberately **not** the format of `iroh/TEST-LOG.md`, which is a chronological session narrative
  for a multi-node campaign — a different artifact for a different job. Naming which convention
  applies here so the executor does not improvise a third.
- `alpha/experiments/meer-queue/PHASE-0-FINDINGS.md` — **new (written 2026-08-08).** The discovery
  record: seven probes, their verdicts, resolved versions, and the seven plan changes they forced.
- `alpha/experiments/meer-queue/README.md` — **new.** Crate orientation, how to run, what each test
  claims. Phase 14.
- `alpha/experiments/meer-queue/S8-RESULTS.md` — **new.** S8's measurement table, kept separate so the
  long-running measurement does not contend with the queue phases on `TEST-LOG.md` (see Concurrency
  Map). Phase 11; folded into `TEST-LOG.md` in Phase 12.
- `alpha/experiments/SPEC-DIVERGENCE-REGISTER.md` — five new rows. **Seeded 2026-08-08** (Phase 0
  introduced the first live tag, and the register's convention requires a row per tag); Phase 12
  verifies correspondence and flips the "declared" statuses.
- `alpha/experiments/meer-queue/SPIKE-SPEC.md` — status line updated 2026-08-08 with the Phase-0 run
  and a correction banner on M2's falsified negative-arm hypothesis.
- `alpha/plans/2026-08-07-meer-lane.md` — status line updated 2026-08-08 (Discovery run).
- `alpha/thinking/meer-as-custodian-queue.md` — falsifications folded back; the stale "MIT `cli` PoC"
  line in § "What we test next" corrected; § "Open" updated with what S8 measured. Phase 13.
- `alpha/experiments/EXPERIMENT-BACKLOG.md` and `alpha/experiments/MASTER-INDEX.md` — register the
  experiment on the transport track (required by SPIKE-SPEC § "On completion"). Phase 14.
- `alpha/ROADMAP_TODO.md` — the Rung-A follow-up for S4's with-device-group arm (Open Question 4).
  Phase 8, in the phase that creates the debt rather than a later cleanup.
- `alpha/experiments/meer-queue/SPIKE-SPEC.md` — status line `specified, not yet run` → run, with the
  date. Phase 14.
- `alpha/plans/2026-08-07-meer-lane.md` — Phase 0 status; open items resolved by the spike marked as
  such. Phase 14.
- `alpha/plans/2026-08-07-2-plan-meer-queue-spike.md` (this file) — status line `planned` → executed,
  with the run date. Phase 14.
- `beta/drystone-spec/` — **not edited by this plan.** Anything touching normative text is *flagged*
  in `TEST-LOG.md` under an explicit "Normative-text flags" heading for a separate decision. Phase 13.
  Spec edits are not a spike's call.
- Grepped for existing references to `meer-queue`: only `SPIKE-SPEC.md` itself and
  `alpha/plans/2026-08-07-meer-lane.md`. No other file references the crate path.

## Concurrency Map

**Decision (Open Question 5, 2026-08-08): the executor runs everything sequentially.** The parallel
analysis below is retained on record — the disjointness holds and a future re-run may want it — but
it is not exercised. Sequential is simpler to follow and easier to attribute a failure to.

**Pass 3 update: the parallel option is now also *unsafe*, not merely unused.** Pass 3's write-set
additions put `src/mls.rs`, `src/meer.rs`, and `src/ciss_harness.rs` into both branches. The plan is
sequential by choice **and** by disjointness. Details below.

```
Executed spine (sequential — by user decision AND, after Pass 3, by necessity):
  Phase 0 → 1 → 2 → 3 → 4 → 5 → 6 → 7 → 8 → 9 → 10 → 11 → 12 → 13 → 14

Parallel opportunity analyzed in Pass 2 — INVALIDATED by Pass 3, retained for the reasoning trail:
  Phase 0 → 1 → 2 → 3 → 4 → 5 → 6 → [ {7 → 8 → 9 → 10}  ||  {11} ] → 12 → 13 → 14
```

Phases 0–6 are strictly sequential: each reads what the prior wrote (harness → MLS layer → meer core →
transport → M1 → M2), and M2's negative arm depends on M1's forwarding path existing.

**Parallel set {Phase 7–10 chain, Phase 11} — ANALYZED IN PASS 2, INVALIDATED IN PASS 3.**

> **Pass 3 disjointness re-check: the parallel set no longer holds.** Pass 3 added
> `meer-queue/src/mls.rs` to Phase 11's write-set (S8 cannot use `mls-replant::stamp` — it hardcodes
> `MlsGroupCreateConfig::default()` and discards `GroupInfo`), and `src/mls.rs` is in the **read-set
> of Phases 7–10** via `mls::seal`/`mls::open`. A phase mutating a module its would-be-parallel
> sibling reads is not parallel-safe. Pass 3 also added `src/meer.rs` to Phase 5, `src/ciss_harness.rs`
> to Phase 7, and `src/meer.rs` + `src/queue.rs` to Phase 9 — so the two branches now overlap on
> source files, not just on `TEST-LOG.md`.
>
> **Operationally this changes nothing** — the user chose sequential execution (Open Question 5)
> before Pass 3 ran. But the recorded analysis would have been *wrong* if a future re-run trusted it,
> which is precisely the failure mode the disjointness re-check exists to catch. Corrected rather than
> deleted, so the reasoning trail survives.

The Pass-2 analysis, now superseded, ran as follows:

- **Disjoint write-sets (as believed in Pass 2).** Phases 7–10 write `tests/s2_fanout_dedup.rs`,
  `tests/s3_dual_delivery.rs`, `tests/s4_multi_device.rs`, `tests/s5_expiry_watermark.rs`,
  `tests/s6_repoint.rs`, `tests/s7_carol_carries.rs`, and `TEST-LOG.md`. Phase 11 writes
  `tests/s8_object_sizes.rs` and `S8-RESULTS.md`. `S8-RESULTS.md` was introduced specifically to keep
  these disjoint. **This is the claim Pass 3 falsified** — the src/ files were not yet in either
  write-set when Pass 2 checked.
- **Shared-state contract.** Both branches run `cargo test` in the same crate, so they share
  `target/`. Cargo's own file locking makes concurrent builds safe but serializes them, which removes
  most of the benefit; the real benefit is that S8's long large-N runs overlap the queue phases'
  authoring time. Both branches bind ephemeral loopback ports (CISS harness `127.0.0.1:0`, iroh
  loopback relay). **Phase 11 binds no ports at all** — S8 is pure in-process openmls measurement with
  no CISS and no iroh, which is what makes the port question moot rather than merely unlikely.
  Neither branch invokes any `git` operation. Neither writes outside the crate directory except
  Phase 12+.
- **Re-entry verification.** On return from each branch: (a) `git status --porcelain` in `discovery/`
  lists only files in the union of the two declared write-sets; (b) `git rev-parse HEAD` in
  `discovery/` equals the pre-dispatch SHA (no phase commits — commits happen only on request);
  (c) `lsof -iTCP -sTCP:LISTEN -P -n | grep -c 127.0.0.1` returns to its pre-dispatch count, proving
  no CISS harness or relay outlived its test; (d) no `S8-RESULTS.md` content appears in `TEST-LOG.md`
  before Phase 12.

**Default is still sequential, and sequential is what was chosen.** Parallelism here was opt-in and
worth considering for exactly one reason: S8 at large N is the only phase whose wall-clock is
dominated by computation rather than authoring, and it has no dependency on the queue, the transport,
or CISS. The user's call (Open Question 5) is to run the single stream. The re-entry verification
items below therefore do not fire during this run; they stay documented against the analyzed
grouping.

## Phases

### Phase 0: Discovery

**Goal:** Resolve every unknown that could invalidate a later phase, before any spike code is
committed to a shape. Six tasks; all are cheap.

- [ ] **D1: Does a cross-repo path dependency from `discovery/` to `CISS/` build?**
  - **Probe:** Minimal crate at `alpha/experiments/meer-queue/` with `ciss = { path =
    "../../../../CISS" }`, `cargo build`. Record whether CISS's lib target is reachable and whether
    `App`, `Blobs`, `Db` are `pub` from outside the crate.
  - **Success criteria:** A binary that constructs an `App` and calls `.router()` compiles. Or: a
    named compile error that tells us CISS must be reached another way.
  - **Disposition:** `promote` — becomes `src/ciss_harness.rs` in Phase 1.
  - **Gates:** Open Question 1.

- [ ] **D2: What does an `App` need to construct, and what auth does a `PUT /{did}/objects/{key}` require?**
  - **Probe:** Read `CISS/src/server.rs:230–260` (`App::new`, `Blobs`, `Db`) and `authenticate()`; read
    `CISS/tests/common/mod.rs` `Actor::auth`. Stand up the harness and PUT one object, print status.
  - **Success criteria:** A recorded 2xx with the returned content address, and the exact header set
    that produced it.
  - **Disposition:** `promote`.

- [ ] **D3: Can an `MlsMessageIn` be re-serialized, and if not, what is the honest re-frame for M2's negative arm?**
  - **Probe:** Check whether `MlsMessageIn` implements `tls_codec::Serialize` in openmls 0.8.1. If
    not, evaluate `MlsMessageBodyIn::PrivateMessage(PrivateMessageIn) → PrivateMessage →
    MlsMessageOut → to_bytes()`. Record which path exists and its exact signature.
  - **Success criteria:** A named, compiling round-trip that produces bytes semantically equivalent to
    the input with a different digest — or a finding that openmls 0.8.1 offers no such path, which is
    itself a result about how hard the forbidden move is to make by accident.
  - **Disposition:** `promote` — becomes the negative arm in Phase 6.
  - **Note:** This is the methodology's "do not assert an API shape from memory" rule biting. The
    negative arm's whole value is that it reports what OpenMLS *actually* does; guessing the
    re-encode path would undermine it.

- [ ] **D4: Does `mls-replant` compose as a path dependency, and does `create_message` accept its
      persona state?**
  - **Partly resolved during Pass 3 — no probe needed for the signer question.** `Persona`'s fields
    are all `pub`, including `signer: SignatureKeyPair` and `provider: OpenMlsRustCrypto`
    (`mls-replant/src/lib.rs:22–28`). The "does the spike need its own persona type" branch is closed.
  - **Probe (what remains):** `mls-replant = { path = "../mls-replant" }`; call `stamp()` then
    `create_message(&p.provider, &p.signer, msg)` on the resulting group and round-trip it.
  - **Success criteria:** A two-member group producing one real application message that the other
    member opens.
  - **Disposition:** `promote` — becomes `src/mls.rs` in Phase 2.

- [ ] **D5: Do the copied `relay.rs` / `node.rs` build without `lineage-mls`?**
  - **Probe:** Copy both from `mls-welcome-over-iroh`, add `iroh =1.0.0` + `iroh-relay =1.0.0`, build.
    Record any lineage-* symbol that leaked in.
  - **Success criteria:** A loopback relay spawns and two endpoints connect over the spike's own ALPN.
  - **Disposition:** `promote` — becomes Phase 4.

- [ ] **D6: How expensive is group construction at scale, and does the cost curve look pathological?**
  - **Probe:** Time `stamp()` at N = 2, 10, 50, 200 in release mode. Extrapolate to 500, 1000, and
    beyond.
  - **Success criteria:** A wall-clock figure per N, and a stated expectation for the full sweep.
  - **Disposition:** `throwaway` — the real sweep is Phase 11.
  - **No longer a decision gate.** Open Question 3 was resolved in advance: the full sweep is
    pre-authorized however long it takes. D6 survives because it sizes expectations and catches a
    pathological curve (e.g. quadratic wall-clock) *before* Phase 11 spends hours discovering it —
    but it cannot shorten the sweep.

**Outputs fed back into the plan:** Verified Assumptions updated with D1–D6 findings; Open Questions 1
and 3 resolved; phases adjusted if a probe invalidates an assumption, with the change recorded in the
Review Log.

**Done when:** All six probes have recorded answers and no later phase still rests on inference.
(Open Questions 1–5 are all resolved as of 2026-08-08, before execution — Phase 0 no longer gates a
decision, only the assumptions.)

**Discovery Exemption applies** — Phase 0 produces knowledge, not production code. No TDD, no wiring
tests, no commit-per-item. Each task honors its declared Disposition.

---

### Phase 1: CISS harness — GREEN 2026-08-09

> **Executed.** RED first (`w0_ciss_roundtrip` failed to compile against a non-existent
> `ciss_harness`), then GREEN: 3 tests passing, clippy clean. Both load-bearing assertions were
> **mutation-checked** rather than trusted: swapping `Blobs::Fs`→`Blobs::Memory` killed the
> disk-count assertion (`found: []`), and flipping the expected status 413→200 killed the cap
> assertion. A compile-error RED proves a test needs the code; it does not prove the assertions bite,
> so each was made to fail deliberately.
>
> One deviation from the plan: `src/lib.rs` was created here rather than in Phase 2, because the
> integration test needs a lib target to import. Phase 2 still adds `resolved_versions()` and the
> tracing init to it.

**Goal:** Spin the real CISS axum router on loopback and round-trip an object over real HTTP.
**Changes:**
- [ ] `Cargo.toml` — crate manifest, own lockfile (mls-replant's pattern). Pins printed at run time.
- [ ] `src/ciss_harness.rs` — spawn/shutdown the real router on `127.0.0.1:0`; `put_object` /
      `get_object` / `du` over `reqwest`. Built with **`App::with_limits(seed, Blobs::Fs(tempdir),
      Db::Memory, explicit_limits)`** — not `App::new`, which reads `CISS_MAX_STORE_BYTES` /
      `CISS_MAX_DID_BYTES` from the ambient environment (Pass 3). `Limits` has no `Default`; its
      fields are `pub`, so build it by literal with a stated ceiling (Phase 0 D1). `Blobs::Fs` rather than
      `Blobs::Memory` so stored objects are countable on disk, which is what makes S2's dedup claim
      observable rather than inferred. Carries the
      `SPEC-DELTA[meer-spike-ciss-inproc | test-hermeticization]` tag at module head.
- [ ] `tests/w0_ciss_roundtrip.rs` — RED first.
**Call chain:** test → `CissHarness::spawn()` → `ciss::server::App::router()` → real axum handler →
real blobstore.
**Wiring test:** `w0_ciss_roundtrip` — PUT bytes, GET them back by content address, assert byte
equality, and assert the cap at **both** edges: an object of exactly `MAX_OBJECT_BYTES` is **accepted**
and one of `MAX_OBJECT_BYTES + 1` is **refused**. Pass 3 added the at-boundary accept case: testing
only the over-cap side is a single-point assertion that an off-by-one in the comparison would survive.
**Phase 0 correction (D2):** the refusal is **HTTP 413 from axum's `DefaultBodyLimit`**
(`Failed to buffer the request body: length limit exceeded`), *not* CISS's `ObjectTooLarge` — the
request never reaches the blobstore, which is a second line of defence the HTTP path never exercises.
Assert the 413; do not assert a CISS error that cannot occur on this path.
**Depends on:** Phase 0 (D1, D2).
**Read-set:** `CISS/src/server.rs`, `CISS/src/blobstore.rs`, `CISS/tests/common/mod.rs`,
`mls-replant/Cargo.toml`.
**Write-set:** `meer-queue/Cargo.toml`, `meer-queue/src/ciss_harness.rs`,
`meer-queue/tests/w0_ciss_roundtrip.rs`.
**Shared-state contract:** Binds one ephemeral loopback TCP port per test, released on shutdown.
Writes CISS blob state under a per-test `$TMPDIR` temp dir (`Blobs::Fs`), metering in-memory
(`Db::Memory`). **Sets no env vars and — via `with_limits` — reads none** (Pass 3: `App::new` would
have read two, making the harness sensitive to ambient state). No git operations.
**Risks:** CISS's `App` construction may require more setup than the test harness suggests (resolver,
service DID, keys). D2 de-risks this. If auth turns out to need a signed credential, the harness grows
an identity — which is real, not a stand-in.
**Done when:**
1. *Behavioral:* `cargo test --test w0_ciss_roundtrip` stores and retrieves bytes through the real
   CISS HTTP boundary; an object of exactly 2 MiB is accepted and one of 2 MiB + 1 is refused with
   CISS's own error.
2. *Verification:* `cargo test --test w0_ciss_roundtrip -- --nocapture`.
**Validation:** Moderate. Tests plus a manual `curl` against a harness held open, to confirm the
boundary is real HTTP and not an in-process shortcut.

---

### Phase 2: MLS layer — GREEN 2026-08-09

> **Executed.** RED first (unresolved import `meer_queue::mls`), then GREEN: 3 tests, clippy clean.
> Three mutations run, all killed:
> - **`seal` replaced with an XOR placeholder** — the methodology's canonical forbidden move. Killed
>   at the `wire_format()` assertion. Worth recording *why*: the weaker "plaintext is not in the
>   bytes" check **passes under XOR**, so it was the Pass-3 framing assertion, not the obvious one,
>   that caught it. The mutation-resistance addition earned its place empirically.
> - **Banner drifted from the manifest** (`0.5.0`→`0.4.9`) — killed by the `Cargo.toml` cross-check.
> - **`open` panics instead of returning `Err`** — killed by the S7 pre-condition test.
>
> **Deliberate deviation from the plan's signature:** `open` returns `Result`, not `Vec<u8>`. S7 must
> record *the named error a non-member receives*, and an unwinding panic cannot be recorded. Pinned
> by a test so it cannot regress.

**Goal:** Real OpenMLS application messages on top of `mls-replant`'s group construction.
**Changes:**
- [ ] `src/lib.rs` — module wiring; the version banner (`resolved_versions()`) every result prints;
      **`tracing_subscriber` + `EnvFilter` to stderr** (Pass 3 — see Reasoning § Observability),
      matching `mls-welcome-over-iroh/src/main.rs:45–48`.
- [ ] `src/mls.rs` — `seal(group, persona, plaintext) -> Vec<u8>` via `create_message` +
      `to_bytes()`; `open(group, bytes) -> Vec<u8>` via `MlsMessageIn` + `process_message`. Real
      library only. Calls take `&persona.provider` / `&persona.signer` directly (fields are `pub`).
- [ ] `tests/w1_mls_roundtrip.rs` — RED first.
**Call chain:** test → `mls::seal` → `mls_replant::stamp`-built `MlsGroup::create_message` → bytes →
`mls::open` → `MlsGroup::process_message` → plaintext.
**Wiring test:** `w1_mls_roundtrip` — Alice seals, Bob opens, plaintext matches; the sealed bytes do
**not** contain the plaintext; and (Pass 3) the sealed bytes parse as `MlsMessageIn` with
`wire_format() == WireFormat::PrivateMessage`. "Not plaintext" alone is survivable by any
transformation at all — asserting the real MLS wire format is what makes the assertion specific to a
genuine seal rather than to mere obfuscation.
**Depends on:** Phase 0 (D4).
**Read-set:** `mls-replant/src/lib.rs`, openmls `application.rs`, `processing.rs`, `message_out.rs`,
`message_in.rs`.
**Write-set:** `meer-queue/src/lib.rs`, `meer-queue/src/mls.rs`,
`meer-queue/tests/w1_mls_roundtrip.rs`.
**Shared-state contract:** No shared mutable state beyond the file write-set. No ports, no network, no
temp files.
**Risks:** `Persona` may not expose its signer. D4 resolves; fallback is a spike-local persona type
built the same way, still real openmls.
**Done when:**
1. *Behavioral:* A real two-member group round-trips an application message, and `resolved_versions()`
   prints every pinned crate version.
2. *Verification:* `cargo test --test w1_mls_roundtrip -- --nocapture`.
**Validation:** Narrow. Tests are sufficient; the plaintext-absence assertion is the guard against a
degenerate seal.

---

### Phase 3: Meer core and queue — GREEN 2026-08-09

> **Executed.** RED first, then GREEN: 7 tests, clippy clean. Five mutations run; **one survived
> and exposed a vacuous assertion.**
>
> **The surviving mutant.** Making `publish` deposit **once per recipient** did not fail the suite.
> Cause: CISS is content-addressed, so five `PUT`s of identical bytes still leave **one file**.
> `blob_files().len() == 1` was therefore testing *CISS's dedup*, not *the meer's store-once* — two
> different claims that the on-disk count cannot tell apart.
>
> **Why that distinction is load-bearing, not pedantic.** Storage dedups; **transit does not**. The
> design meters transit (the hypothesis doc's "meter both transit and at-rest", and the transit meter
> *is* the offline-data fraction). A meer that deposited per-recipient would bill N × the bytes for
> one delivered message while looking identical on disk. Fix: `CissHarness` now counts `PUT`s, and
> the test asserts **`put_count() == 1`** alongside the file count, as two separate claims. Mutant
> re-run and killed.
>
> **This reshapes S2.** Phase 7's fan-out measurement must count deposits, not just files — the plan's
> "measure actual storage cost" is the weaker half of what S2 is really about.
>
> Four other mutations killed: `ack` not pruning; `want` ignoring the have-set; `append` not
> idempotent on digest; a constant deposit day.
>
> **One in-phase correction:** `append`'s dedup-on-digest was written without a test driving it
> (spotted before commit). A test was added rather than the code deferred — it is genuine Phase-3
> queue behaviour, and it is what makes S3's dual delivery free rather than a special case.

**Goal:** Four of the five operations, blind, with the have/want digest diff. **Sweep-and-watermark
is deliberately NOT built here** — see below.
**Changes:**
- [ ] `src/meer.rs` — accept publish; PUT once to CISS; append per-recipient entry; serve drain.
      **No openmls dependency in this module** — that is the structural form of M2's positive arm.
      Carries `SPEC-DELTA[meer-spike-namespace | stand-in]` and
      `SPEC-DELTA[meer-spike-kind-gate | absent]` at module head.
- [ ] `src/queue.rs` — entries, have/want diff, ack + prune. Takes `ciss::clock::SimClock` (**not** a
      spike-local seam — see Reasoning) so entries carry a timestamp from the outset; the *expiry*
      logic that reads it arrives in Phase 9 with the test that drives it. Carries
      `SPEC-DELTA[meer-spike-clock | test-scaffold]`.
- [ ] `tests/w2_queue_diff.rs` — RED first.

**Pass 3 correction — the fifth operation moved to Phase 9.** The Pass-2 draft built "sweep expired,
leave a watermark" here, but nothing tests it until S5 in Phase 9. That is production code with no
failing test behind it, and six phases of dead code by the plan's own "built means wired" rule. Sweep
and watermark now land in Phase 9, test-first. Phase 3 builds exactly what `w2_queue_diff` exercises.
**Call chain:** test → `Meer::publish(bytes, recipients)` → `CissHarness::put_object` → per-recipient
`Queue::append` → `Meer::drain(have_set)` → `Queue::want(have_set)` → `CissHarness::get_object` →
bytes.
**Wiring test:** `w2_queue_diff` — publish two messages to Bob, drain with an empty have-set (both
returned), ack one, drain again with the have-set containing it (only the other returned). Exercises
publish → CISS → queue → diff → fetch end to end. **Edge cases required (Pass 3):** draining an empty
queue returns empty rather than erroring; a have-set containing a digest the queue never held is
ignored rather than crashing or echoing it back; and draining twice with no intervening publish is
idempotent. The happy path alone is a single-point assertion on branching set logic — these are the
boundaries a one-line mutation to the diff would otherwise survive.
**Depends on:** Phases 1, 2.
**Read-set:** `src/ciss_harness.rs`, `src/lib.rs`.
**Write-set:** `meer-queue/src/meer.rs`, `meer-queue/src/queue.rs`,
`meer-queue/tests/w2_queue_diff.rs`.
**Shared-state contract:** Inherits Phase 1's port and temp-dir contract via the harness. `SimClock`
means no dependence on wall time. No git, no env.
**Risks:** The temptation to let the meer peek at message type for "just" dedup or ordering. The
module's lack of an openmls dependency is the guard; a `cargo tree` assertion in Phase 6 makes it
checkable.
**Done when:**
1. *Behavioral:* A publish to two recipients stores one blob in CISS and produces two queue entries; a
   drain returns exactly the un-acked difference.
2. *Verification:* `cargo test --test w2_queue_diff -- --nocapture`.
**Validation:** Moderate. Tests plus a manual `ls` of the harness's `Blobs::Fs` temp directory
confirming exactly one stored file for a two-recipient publish. (This is why Phase 1 specifies
`Blobs::Fs` rather than `Blobs::Memory` — Pass 3 caught that the Pass-2 draft called for inspecting a
blob directory that the in-memory backend would not have had.)

---

### Phase 4: iroh transport — GREEN 2026-08-09

> **Executed.** RED first, then GREEN on the first run: 2 tests. Four mutations, all killed —
> including the one that matters, **a drain that ignores the caller and serves every queue**
> (Mallory received Bob's message; the negative half of the wiring test caught it).
> Also killed: a byte flipped in transit, the have-set dropped on the wire, and an ignored ack.
>
> **A design property worth recording, because it is stronger than a test.** The wire format has
> **no recipient field on a drain**. The server derives the queue from `connection.remote_id()`, so
> there is nothing for a caller to claim and therefore nothing to validate — a scope that *cannot be
> misstated*, rather than one that is checked. Sealed payloads travel as raw length-prefixed bytes
> and are never re-encoded, which keeps M2's claim out of the transport's reach.
>
> **Refactor this phase forced:** `Meer` now owns an `Arc<CissHarness>` instead of borrowing it, and
> `CissHarness::shutdown` takes `&self` via interior mutability. A spawned accept loop must be
> `'static`, and a borrow cannot be. Phases 1–3 tests updated; all still green.
>
> **Process note (recorded because it nearly cost a silent regression):** a mutation-restore used a
> stale `/tmp` backup that predated the `Arc` refactor and clobbered `meer.rs` with the older
> lifetime-bearing version. Caught by a compile failure, recovered with `git checkout` + re-applied
> patch. Later mutations in this phase snapshot to a per-phase directory from a known-green state
> instead. The lesson is not "be careful with `cp`" — it is that **mutation testing edits working
> code, so the restore path needs to be as reliable as the mutation**, and git is that path.

**Goal:** Deposit and drain over a real iroh connection homed on a real loopback relay.
**Changes:**
- [ ] `src/relay.rs` — copied from `mls-welcome-over-iroh/src/relay.rs`, attribution comment at head.
- [ ] `src/transport.rs` — endpoint build, spike ALPN, deposit/drain framing; drain scoped by
      `EndpointId`. Carries `SPEC-DELTA[meer-spike-drain-auth | stand-in]`.
- [ ] `tests/w3_transport_drain.rs` — RED first.
**Call chain:** test → client `Endpoint::connect` (real relay) → meer's ALPN handler →
`Meer::drain` → response frames → client.
**Wiring test:** `w3_transport_drain` — a publish deposited over the wire is drained over the wire by a
second endpoint, and a third endpoint with a different `EndpointId` draining the same queue gets
nothing. The negative half is what proves the drain scope is live rather than decorative.
**Depends on:** Phases 0 (D5), 3.
**Read-set:** `mls-welcome-over-iroh/src/relay.rs`, `.../src/node.rs`, `src/meer.rs`.
**Write-set:** `meer-queue/src/relay.rs`, `meer-queue/src/transport.rs`,
`meer-queue/tests/w3_transport_drain.rs`.
**Shared-state contract:** Binds loopback relay ports. `mls-welcome-over-iroh` hardcodes 3340/3343/3478/9099;
the spike **must** use ephemeral or per-test-offset ports so a concurrent run does not collide — the
copied file gets changed for this, and the change is noted in the attribution comment.
**Risks:** ~~iroh's relay spawn is the most environment-sensitive piece here.~~ **Retired by Phase 0
(D5): a real loopback relay comes up and carries the connection; the fallback was not needed.** Two
concrete findings replace it: dial by **`Endpoint::addr()`**, never a bare `EndpointId` — `presets::Minimal`
configures no DNS discovery, so a bare id fails with `No addressing information available`
(the ancestor does the same, `mls-welcome-over-iroh/src/main.rs:65,103`); and the copied `relay.rs`
now carries `RelayPorts::ephemeral()` because the original's fixed 3340/3343/3478/9090 collide across
concurrent runs. Residual risk: none identified.
**Done when:**
1. *Behavioral:* A message deposited over a real iroh connection is drained over a real iroh
   connection by the intended endpoint and only by it.
2. *Verification:* `cargo test --test w3_transport_drain -- --nocapture`.
**Validation:** Broad. Tests plus confirmation from iroh's own logs that the connection was
established through the relay, not by a loopback shortcut.

---

### Phase 5: M1 — an offline member drains and decrypts — GREEN 2026-08-10

> **M1 CONFIRMED (real-lib).** Verdict line:
> `M1 CONFIRMED (real-lib): offline member drained 1 blob(s) and decrypted; meer group keys held = 0,`
> `storage credentials = 1. [openmls =0.8.1, openmls_rust_crypto =0.5.1, openmls_basic_credential`
> `=0.5.0, openmls_traits =0.5.0, tls_codec 0.4]`
>
> **A mutation survived and exposed a vacuous assertion — the second time this pass.** The test
> asserted only that Bob was *unreachable after* teardown. Removing the teardown entirely still
> passed, because nothing listened on Bob's endpoint in either world, so the dial failed both ways.
> The assertion was equally true of the correct state and the broken one — Anti-Pattern 7(a),
> asserting an absence. Fixed by requiring **both halves**: reachable before teardown, unreachable
> after. `MeerClient` gained a real accept loop to make that discriminating — which is not test
> scaffolding but the **live-carriage path S3 needs anyway**, so the fix bought a phase's work.
>
> **Deliberate deviation from the plan.** The plan specified `keys_held() -> usize` returning 0.
> Replaced with `KeyInventory { group_keys, storage_credentials }`, because a bare zero is both a
> tautology (this module cannot name an MLS type) **and an overstatement** — the meer holds exactly
> one credential, its own CISS namespace key, without which it could not write the mail anywhere.
> *Blind to content is not the same as credential-less.* The verdict reports both numbers.

**Goal:** The first must-pass claim.
**Changes:**
- [ ] `tests/m1_offline_drain.rs` — RED first.
- [ ] `src/meer.rs` — add `keys_held() -> usize` (Pass 3). M1's assertion
      `meer_payload_keys_held == 0` needs an accessor that does not exist after Phase 3; the Pass-2
      draft had a tests-only write-set and no phase creating it. Written here, driven by the failing
      M1 test, rather than speculatively in Phase 3 where nothing would have exercised it.
- [ ] `TEST-LOG.md` — created; M1 verdict line with rung and printed versions.
**Call chain:** real group → Bob's endpoint dropped → `mls::seal` → transport deposit →
`Meer::publish` → CISS → Bob reconnects → transport drain → `mls::open` → plaintext.
**Wiring test:** `m1_offline_drain` is itself the wiring test — it is the full entry-point path.
**Depends on:** Phases 2, 3, 4.
**Read-set:** all `src/`.
**Write-set:** `meer-queue/tests/m1_offline_drain.rs`, `meer-queue/src/meer.rs`,
`meer-queue/TEST-LOG.md`.
**Shared-state contract:** Inherits Phases 1 and 4. No git, no env, no wall-clock dependence.
**Risks:** "Offline" must be a real endpoint teardown, not a flag the meer reads. If Bob's endpoint is
merely marked absent, the test proves less than its name.
**Done when:**
1. *Behavioral:* A member offline for a message's entire live window recovers it from the meer and
   decrypts it with real `process_message`, and the meer reports zero keys held.
2. *Verification:* `cargo test --test m1_offline_drain -- --nocapture` prints
   `M1 CONFIRMED (real-lib): offline member drained N blobs and decrypted; meer keys held = 0.` or the
   FALSIFIED form with what happened.
**Validation:** Broad. Beyond the assertion: confirm from `cargo tree` that `meer.rs`'s module graph
reaches no openmls crate, so "holds no key" is structural.

---

### Phase 6: M2 — byte-identical forwarding, and the negative case

**Goal:** The second must-pass claim. **Restructured after Phase 0 — the spec's negative-arm
hypothesis was falsified by D3 before the arm was written.**

> **D3 finding (see `PHASE-0-FINDINGS.md`).** The spec hypothesized that a re-framed copy is
> "detectably different at the byte level." It is not: decode→re-encode is **byte-identical** for both
> application messages (189 B) and commits (490 B), because TLS-codec serialization is canonical.
> Separately, both conversions that make a re-frame possible are `#[cfg(any(feature = "test-utils",
> test))]` in openmls 0.8.1, each carrying the comment *"break abstraction layers and MUST NOT be made
> available outside of tests."*
>
> The `MUST` is not weakened — the **risk model behind it was mis-stated**. The hazard the spec names
> (ratchet-key / nonce reuse) comes from **re-sealing**, not re-framing; and re-sealing needs a key the
> meer does not have. So a blind forwarder cannot produce a semantically-equivalent-but-byte-different
> copy *at all*: the only transformation available to it is byte-preserving, and the dangerous one is
> cryptographically out of reach.

**Changes:**
- [ ] `tests/m2_byte_identity.rs` — RED first.
      **Positive arm (unchanged):** digest stable at production, after PUT, after CISS
      re-verify-on-read, and at Bob's receive pre-decode.
      **Negative arm (rewritten):** three assertions replacing the falsified one —
      (i) the re-frame is **unreachable** in a default build (a `#[cfg(not(feature = "reframe"))]`
      compile-time assertion that the conversion does not exist);
      (ii) under `--features reframe`, the forced re-encode is **byte-identical**, recorded as the
      measurement that falsified the spec's hypothesis — and Bob still processes it, because it is the
      same bytes;
      (iii) the operation that *would* break the seal is **re-sealing**, which the meer cannot perform:
      asserted structurally via the `cargo tree` check that `meer.rs` reaches no openmls crate, so it
      holds no `create_message`.
- [ ] `TEST-LOG.md` — M2 verdict, recording the falsification **loudly** (a FALSIFIED result is a
      first-class success) with the exact digests from both arms, and flagging Part 2 §6.6.2's stated
      rationale for the `MUST` as normative text needing correction.
**Call chain:** `mls::seal` → sha256 → deposit → CISS PUT → CISS GET → drain → sha256 → `mls::open`.
**Wiring test:** `m2_byte_identity` — four digests equal across the full path, plus a `cargo tree`
assertion that `meer.rs` has no openmls edge.
**Depends on:** Phases 0 (D3), 5.
**Read-set:** all `src/`, openmls framing sources.
**Write-set:** `meer-queue/tests/m2_byte_identity.rs`, `meer-queue/TEST-LOG.md`.
**Shared-state contract:** As Phase 5.
**Risks:** The pull toward reporting this as "M2 CONFIRMED" and moving on. It is a **confirmed
positive arm plus a falsified negative hypothesis**, and the verdict line must say both. The design
consequence is favourable — the `MUST` turns out to be nearly self-enforcing — which makes it *more*
tempting to smooth over, not less.
**Done when:**
1. *Behavioral:* The digest is stable across store-and-serve; the re-frame is shown unreachable in a
   default build and byte-preserving when forced; and the meer is shown structurally incapable of the
   one operation (re-seal) that could break the seal.
2. *Verification:* `cargo test --test m2_byte_identity -- --nocapture` and
   `cargo test --test m2_byte_identity --features reframe -- --nocapture` both print their arms of the
   M2 verdict.
**Validation:** Broad. Both arms, plus the module-graph assertion.

---

### Phase 7: S2 fan-out and dedup, S3 dual delivery

**Goal:** Confirm dedup is real at the CISS boundary, and that dual delivery is free.
**Changes:**
- [ ] `tests/s2_fanout_dedup.rs` — RED first. One message, five recipients: one blob stored, five
      references; measure actual storage against the naive per-recipient copy. **Edges (Pass 3):**
      one recipient → one blob, and two *distinct* messages to the same five → two blobs. A
      five-recipients-one-blob assertion alone is survived by an implementation that always stores
      exactly one object.
- [ ] `src/ciss_harness.rs` — add the `du` read if D2 did not already land it (Pass 3: the Pass-2
      draft's validation called for `GET /{did}/du` but no phase created the method).
- [ ] `tests/s3_dual_delivery.rs` — same message carried live and drained: dedups to one entry on
      content hash; MLS applies it idempotently.
- [ ] `TEST-LOG.md` — S2, S3 verdicts.
**Call chain:** as Phase 3, with five recipients (S2) and two delivery paths (S3).
**Wiring test:** each test drives the full publish → CISS → drain path.
**Depends on:** Phase 6.
**Read-set:** all `src/`.
**Write-set:** `meer-queue/tests/s2_fanout_dedup.rs`, `meer-queue/tests/s3_dual_delivery.rs`,
`meer-queue/src/ciss_harness.rs` (the `du` read), `meer-queue/TEST-LOG.md`.
**Shared-state contract:** As Phase 5. **Not in a live parallel set** — sequential run (Open Question 5),
and the Pass-2 grouping is INVALIDATED (see Concurrency Map).
**Re-entry verification:** *Does not fire — sequential run.* Retained from the invalidated Pass-2
grouping: `git status --porcelain discovery/` lists only this phase's write-set; `discovery/` HEAD
unchanged; loopback listener count back to baseline.
**Risks:** S2's storage measurement must come from CISS's own accounting (`/{did}/du` or the meter),
not from counting bytes we handed it — otherwise it measures our bookkeeping, not CISS's.
**Done when:**
1. *Behavioral:* A five-recipient publish stores exactly one object; a doubly-delivered message
   produces exactly one queue entry and one MLS application.
2. *Verification:* `cargo test --test s2_fanout_dedup --test s3_dual_delivery -- --nocapture`.
**Validation:** Moderate. Tests plus **two independent** confirmations of the storage figure: a read
of `GET /{did}/du` (CISS's own accounting) and a file count in the `Blobs::Fs` temp directory. Two
sources because the whole point of S2 is that the dedup is CISS's, not our bookkeeping's.

---

### Phase 8: S4 — multi-device and deliver-once

**Goal:** Make the deliver-once dependency concrete rather than asserted. **The scenario the ask
singles out.**
**Changes:**
- [ ] `tests/s4_multi_device.rs` — Bob's phone and laptop enrolled, no device group. Deliver-once with
      prune-on-ack. Observe whether the laptop starves. Then record, in prose, exactly what a
      device-group presence check would have to detect.
- [ ] `TEST-LOG.md` — S4 verdict, **explicitly two-rung**: Rung A for the without-device-group arm,
      and an explicit "not tested at Rung A, follow-up item generated" for the with-arm.
- [ ] A tracked Rung-A follow-up item for the with-device-group arm (Open Question 4), filed where the
      corpus tracks residue — `alpha/ROADMAP_TODO.md`, the single open-item backlog. The question
      stays open until §6.6.5 fan-out exists and the arm is re-run against it.
**Call chain:** publish → queue → two draining endpoints with distinct `EndpointId`s → prune on first
ack.
**Wiring test:** `s4_multi_device` drives both endpoints through the real transport.
**Depends on:** Phase 7 (sequential within the branch).
**Read-set:** all `src/`; `beta/drystone-spec/part-2-certifiable-design.md` §6.6.5 for the fan-out
claim being leaned on.
**Write-set:** `meer-queue/tests/s4_multi_device.rs`, `meer-queue/TEST-LOG.md`,
`alpha/ROADMAP_TODO.md` (the Rung-A follow-up row).
**Shared-state contract:** As Phase 5. Not in a live parallel set (sequential run).
**Re-entry verification:** *Does not fire — sequential run.*
**Risks:** The temptation to "fix" the starvation by adding per-device cursors mid-phase. **Do not.**
If the laptop starves, that is the expected falsification of naive deliver-once and the result is the
deliverable. The design consequence — whether §6.6.5 fan-out is a sufficient compensating mechanism —
is Phase 13's, and the honest answer may be that per-device cursors come back.
**Done when:**
1. *Behavioral:* We can state, from a run, whether a second enrolled device starves under deliver-once
   with no device group, and what a presence check would need to detect.
2. *Verification:* `cargo test --test s4_multi_device -- --nocapture` prints the S4 verdict with both
   rungs stated, and `ROADMAP_TODO.md` carries the Rung-A follow-up.
**Validation:** Moderate. Tests plus a written argument, in `TEST-LOG.md`, for what the with-device-group
arm would require — clearly marked as reasoning, not measurement, per Open Question 4. The
methodology's rule applies: a result that does not exercise the real mechanism does not retire the
question, so the follow-up item is part of "done," not an optional extra.

---

### Phase 9: S5 expiry and watermark, S6 revocation and re-point

**Goal:** Whether "loud, visible gap" is constructible, and whether "it never left home" holds.
**Changes:**
- [ ] `tests/s5_expiry_watermark.rs` — **RED first, and it drives the implementation below.** Age a
      queue past its window via `SimClock` with no drain; assert bytes gone, watermark remains, and
      that an honest "here is what you missed and it is gone" is renderable **from what the meer
      retains** — not from what the test happens to know. **Edges (Pass 3):** an entry at exactly the
      retention boundary (day == window) versus one past it (day > window) — the 14-day ceiling is a
      comparison, and testing only "much later" lets an off-by-one survive. Also: a queue drained
      *before* expiry leaves no watermark, since "14 days **or until drained**" is the actual rule and
      a watermark for successfully-delivered mail would be a false gap report.
- [ ] `src/meer.rs` + `src/queue.rs` — **the fifth meer operation: sweep expired, leave a watermark**
      (moved here from Phase 3 by Pass 3, so it is written against a failing test rather than sitting
      untested for six phases).
- [ ] `tests/s6_repoint.rs` — RED first. Bob points at a second meer and stops using the first; assert
      no mail lost and nothing migrated.
- [ ] `TEST-LOG.md` — S5, S6 verdicts.
**Call chain:** S5: publish → clock advance → sweep → drain → watermark. S6: publish to meer A →
enroll meer B → publish to B → drain both → assert completeness.
**Wiring test:** both drive the full path; S6 stands up two independent meer instances.
**Depends on:** Phase 8 (sequential within the branch).
**Read-set:** all `src/`.
**Write-set:** `meer-queue/tests/s5_expiry_watermark.rs`, `meer-queue/tests/s6_repoint.rs`,
`meer-queue/src/meer.rs`, `meer-queue/src/queue.rs`, `meer-queue/TEST-LOG.md`.
**Note on the 4-file rule:** this phase touches 5 files. It stays whole because the sweep/watermark
implementation and the S5 test that drives it are one RED-GREEN cycle and splitting them would
recreate the untested-code defect Pass 3 just removed. S6 is independent and may be split out to its
own phase if the executor finds the phase does not fit one context window.
**Shared-state contract:** As Phase 5, plus a second CISS harness and a second set of loopback ports
for S6's second meer. Not in a live parallel set (sequential run).
**Re-entry verification:** *Does not fire — sequential run.* Both harnesses' ports are still released
on teardown, which the test asserts regardless.
**Risks:** S6's claim is the design's strongest story, which makes it the one most likely to be
tested leniently. Under the namespace stand-in the mail lives in the *meer's* namespace, not Bob's —
so a naive S6 would pass for the wrong reason. The test must state explicitly what the stand-in makes
untestable, and the verdict must be qualified. **If S6 shows re-pointing loses mail, "it never left
home" is false under the stand-in and depends on custodian mode in a way the hypothesis doc does not
admit** — a first-class falsification.
**Done when:**
1. *Behavioral:* An expired queue yields a visible watermark and no bytes; a re-pointed recipient
   loses no mail, with the stand-in's limits on that claim stated.
2. *Verification:* `cargo test --test s5_expiry_watermark --test s6_repoint -- --nocapture`.
**Validation:** Moderate for S5. Broad for S6 — its verdict must name what the namespace stand-in
prevents it from proving.

---

### Phase 10: S7 — Carol carries and learns nothing

**Goal:** Ground §6.4's leak profile in a measurement instead of an assumption.
**Changes:**
- [ ] `tests/s7_carol_carries.rs` — Carol handles the sealed bytes; given them directly she cannot
      decrypt (a real OpenMLS failure, named, not garbage-out). Enumerate what she *can* observe:
      digest, length, timing, recipient-count.
- [ ] `TEST-LOG.md` — S7 verdict with the **observed** metadata set.
**Call chain:** Alice seals → bytes handed to Carol's non-member group state → real `process_message`
→ recorded error.
**Wiring test:** `s7_carol_carries`.
**Depends on:** Phase 9 (sequential within the branch).
**Read-set:** all `src/`.
**Write-set:** `meer-queue/tests/s7_carol_carries.rs`, `meer-queue/TEST-LOG.md`.
**Shared-state contract:** As Phase 5. Not in a live parallel set (sequential run).
**Re-entry verification:** *Does not fire — sequential run.*
**Risks:** Asserting "it failed" is not enough — the error variant must be recorded, because
"cannot decrypt" and "rejected before decryption was attempted" are different security stories.
**Done when:**
1. *Behavioral:* We can state the named OpenMLS error a non-member gets, and list the metadata a
   carrier actually observes.
2. *Verification:* `cargo test --test s7_carol_carries -- --nocapture`.
**Validation:** Moderate. The metadata list must be derived from what the code exposes, not written
from the spec.

---

### Phase 11: S8 — object sizes against the 2 MiB cap

**Goal:** The measurement most likely to change the design. Runs **sequentially** after Phase 10 (Open
Question 5; and after Pass 3 added `src/mls.rs` here, the parallel option is unsafe as well as unused
— see Concurrency Map).
**Changes:**
- [ ] `tests/s8_object_sizes.rs` — real OpenMLS groups at N = 2, 10, 50, 200, 500, 1000, **and beyond
      until a crossover is found or the harness fails** (Open Question 3: the full sweep is
      pre-authorized, no time ceiling). For each, serialized byte length of: application message;
      commit (add / remove / update);
      `GroupInfo` **with** the ratchet-tree extension; `GroupInfo` **without** it; `Welcome` (1
      joiner); `Welcome` (k joiners). Release mode.
- [ ] `S8-RESULTS.md` — the table, the crossover, and the recorded prediction versus what was
      measured.
- [ ] `src/mls.rs` — **a config-parameterized group construction** (Pass 3). S8 **cannot** be built on
      `mls-replant::stamp`: it hardcodes `MlsGroupCreateConfig::default()` (so
      `use_ratchet_tree_extension` is always `false`) and **discards the `GroupInfo`** returned by
      `add_members`. Both of the objects S8 exists to measure are therefore unreachable through it.
      The spike adds its own builder taking the extension flag, and returns the `GroupInfo` rather than
      dropping it. Real openmls throughout — this is a construction path, not a stand-in.
**Call chain:** `mls::` config-parameterized group construction at N → serialize each object → record
length. No CISS, no iroh, no queue — pure openmls measurement.
**Wiring test:** `s8_object_sizes` asserts, at minimum, that an application message stays under
`MAX_OBJECT_BYTES` at every N tested, and that the with/without-extension `GroupInfo` figures differ —
proving the `use_ratchet_tree_extension` toggle is actually being exercised rather than silently
ignored.
**Depends on:** Phases 0 (D6), 2. **Not** on 3, 4, or 6 — which is what made it the parallel
candidate, though the run is sequential per Open Question 5.
**Read-set:** `src/mls.rs`, `mls-replant/src/lib.rs`, openmls config sources.
**Write-set:** `meer-queue/tests/s8_object_sizes.rs`, `meer-queue/src/mls.rs`,
`meer-queue/S8-RESULTS.md`.
**Shared-state contract:** **Binds no ports. Opens no network. Writes no temp files.** Shares only
`target/` with the other branch, where cargo's own locking applies. No git, no env.
**Re-entry verification:** *Does not fire — sequential run.* (And the Pass-2 write-set it named is
stale: Pass 3 added `src/mls.rs` to this phase.)
**Risks:** Large-N group construction may be slow enough to dominate the whole spike. That cost is
pre-authorized (Open Question 3), so the risk is not "we run out of budget" but "we stop early and
call it covered." **If the sweep terminates for any reason short of finding the crossover — harness
failure, memory exhaustion, an N that will not construct — `S8-RESULTS.md` must say exactly where it
stopped and why.** A silent truncation reads as "we covered it" when we did not. A harness failure at
some N is itself a finding about group scale and gets recorded as one, not treated as a run that
failed to happen.
**Done when:**
1. *Behavioral:* We can state, from measurement, where each object type crosses 2 MiB, or the N at
   which the harness stopped and why — and therefore which of the three options (out of scope for v0,
   transparent chunking, ship the tree out of band) the measurement selects.
2. *Verification:* `cargo test --release --test s8_object_sizes -- --nocapture` prints
   `S8 MEASURED: <object> crosses 2 MiB at N = <n>; <object> stays under at N = <max tested>.`
**Validation:** Broad. The measurement drives a design decision, so: re-run at two N values to confirm
stability, confirm the with/without-extension delta matches the O(N) expectation directionally, and
state the prediction-versus-measurement delta explicitly.

**Phase 0 (D7) already took a first reading, and Phase 11 must extend it rather than repeat it.** The
construction path is confirmed available (`MlsGroupCreateConfig::builder().use_ratchet_tree_extension(b)`,
and `add_members` returns `(MlsMessageOut, MlsMessageOut, Option<GroupInfo>)`). First tree-ON data,
N = 2/10/50/200: the extension roughly **doubles** per-member Welcome cost (~153 → ~333 B/member),
`GroupInfo`-with-tree runs ~180 B/member at N=200, and **`commit` bytes are identical tree-ON vs
tree-off at every N**. Straight-line extrapolation puts every O(N) crossover in the
**several-thousand-member** range. That is four points and an extrapolation — S8 establishes the
crossover. Note in `S8-RESULTS.md` that the with-extension rows had no prior before D7.

---

### Phase 12: S1 inspection and the divergence register

**Goal:** Record what enrollment actually requires, and make every stand-in visible.
**Changes:**
- [ ] `TEST-LOG.md` — S1 as **Rung C (static)**: the enrollment sequence walked, every piece of state
      it implies, and whether "one line in your inventory" survives contact. Plus `S8-RESULTS.md`
      folded in.
- [ ] `alpha/experiments/SPEC-DIVERGENCE-REGISTER.md` — **seeded 2026-08-08, verified here.** All five
      rows were written when Phase 0 introduced the first live tag, because the register's own
      convention is that tags and rows correspond — a tag without a row is the exact drift the
      register exists to prevent. Four rows are marked *declared — code lands Phase N*. Phase 12's job
      is therefore to **verify** correspondence (every tag has a row, every row has a tag, and the
      "declared" statuses have flipped to live) rather than to author the rows.
**Call chain:** n/a — documentation phase.
**Wiring test:** `grep -rn "SPEC-DELTA" alpha/experiments/meer-queue/` and the register must
correspond exactly: every tag has a row, every row has a tag. Run it and paste the output.
**Depends on:** Phases 10, 11 (both branches returned).
**Read-set:** all of `meer-queue/`, `SPEC-DIVERGENCE-REGISTER.md` conventions section.
**Write-set:** `meer-queue/TEST-LOG.md`, `alpha/experiments/SPEC-DIVERGENCE-REGISTER.md`.
**Shared-state contract:** No ports, no processes. Edits one file outside the crate.
**Risks:** S1's honest answer may be that enrollment needs state the hypothesis doc does not mention.
That is a finding, not a gap to paper over.
**Done when:**
1. *Behavioral:* Every stand-in in the spike is greppable from its code site and enumerated in the
   register, and S1's enrollment sequence is written down with its implied state.
2. *Verification:* the grep-to-register correspondence check passes with output pasted into
   `TEST-LOG.md`.
**Validation:** Narrow. The correspondence check is mechanical and sufficient.

---

### Phase 13: Fold falsifications back

**Goal:** Update the hypothesis where the spike contradicted it. **A FALSIFIED result is a success and
is recorded loudly.**
**Changes:**
- [ ] `alpha/thinking/meer-as-custodian-queue.md` — fold in every falsification, naming the branch it
      reshapes. Correct the stale "MIT `cli` PoC" line in § "What we test next". Update § "Open" with
      what S8 measured (the `Welcome`/`GroupInfo` versus 2 MiB item is the one the spike was built to
      resolve).
- [ ] `meer-queue/TEST-LOG.md` — a "Normative-text flags" section listing anything that touches
      `beta/drystone-spec/` (candidates: M2's `MUST` on byte-identical storage if the negative arm
      shows acceptance; §6.6.5's fan-out sufficiency if S4 starves even in principle; §6.9.1's
      broadcast-tier ratchet-tree mandate if S8's crossover lands low).
**Call chain:** n/a.
**Wiring test:** every FALSIFIED verdict in `TEST-LOG.md` has a corresponding edit in the thinking doc
or an explicit entry under "Normative-text flags". Checked by walking the log.
**Depends on:** Phase 12.
**Read-set:** `TEST-LOG.md`, `S8-RESULTS.md`, `meer-as-custodian-queue.md`.
**Write-set:** `alpha/thinking/meer-as-custodian-queue.md`, `meer-queue/TEST-LOG.md`.
**Shared-state contract:** No ports, no processes.
**Risks:** The strong pull toward softening a falsification into "mostly confirmed with a caveat."
The spike spec's pre-registered falsification list exists precisely to make that visible; check each
verdict against it by name.
**`beta/drystone-spec/` is NOT edited here.** Normative text is flagged for a separate decision.
**Done when:**
1. *Behavioral:* The hypothesis doc reflects what was measured, and every normative-text implication
   is flagged without being unilaterally applied.
2. *Verification:* Walk the pre-registered falsification list in `SPIKE-SPEC.md` § "What would falsify
   the design" and record, per item, whether it fired.
**Validation:** Moderate. A reader of the thinking doc alone should be able to tell what changed and
why.

---

### Phase 14: Register and close out

**Goal:** The spike is findable and its status is honest.
**Changes:**
- [ ] `meer-queue/README.md` — orientation, how to run, what each test claims and at what rung.
- [ ] `alpha/experiments/EXPERIMENT-BACKLOG.md` and `alpha/experiments/MASTER-INDEX.md` — register on
      the transport track (required by `SPIKE-SPEC.md` § "On completion").
- [ ] `meer-queue/SPIKE-SPEC.md` status line and `alpha/plans/2026-08-07-meer-lane.md` Phase 0 status.
**Call chain:** n/a.
**Wiring test:** `MASTER-INDEX.md` links resolve and `SPIKE-SPEC.md` no longer says "not yet run".
**Depends on:** Phase 13.
**Read-set:** `EXPERIMENT-BACKLOG.md`, `MASTER-INDEX.md` conventions.
**Write-set:** `meer-queue/README.md`, `alpha/experiments/EXPERIMENT-BACKLOG.md`,
`alpha/experiments/MASTER-INDEX.md` (plus two status-line edits).
**Shared-state contract:** No ports, no processes. **No commits** — commits happen only on request.
**Risks:** None material.
**Done when:**
1. *Behavioral:* A reader arriving at `MASTER-INDEX.md` can find the spike, see its rungs, and read
   its verdicts without prior context.
2. *Verification:* Link check across the touched index files.
**Validation:** Narrow.

## Open Questions

- `[CONFIRMED: BLOCKING — RESOLVED]` **Is a cross-repo path dependency from `discovery/` to `CISS/`
  acceptable, or should the spike reach CISS another way?** *This determines the crate's build shape
  and gates Phase 1. Precedent exists — `mls-welcome-over-iroh` path-depends on `alpha/Proofs/`. But
  CISS is a separate top-level repo with its own history and CI, and a `discovery/` crate that will
  not build without a sibling repo checked out is a real coupling. Alternatives considered: vendor a
  pinned CISS commit, or add a thin `[[bin]]` to CISS and drive it as a subprocess.*
  **Resolution (2026-08-08, user): path dependency to `../../../../CISS`.** D1 still probes the
  mechanics (whether `App`/`Blobs`/`Db` are reachable from outside the crate); the policy question is
  settled.

- `[CONFIRMED: BLOCKING]` **Are the two additional stand-ins (`meer-spike-clock`,
  `meer-spike-ciss-inproc`) acceptable, or should either be done differently?** *Both are honest
  weakenings that the spike spec did not anticipate. The clock is standard practice and the
  alternative is a slow, flaky suite. The in-process CISS is forced by CISS shipping no binary.
  Registered either way.*
  **User overrode ADVISORY → BLOCKING (2026-08-08.)** The override was correct: the probe it forced
  turned up two facts the Pass-1/Pass-2 draft had missed, and one of them changed the design.
  **Resolution (2026-08-08, user), both halves:**
  - **Clock — use CISS's own `SimClock`** (`CISS/src/clock.rs`), not a spike-local seam. It is a
    real, public, unit-tested type in the dependency the spike already takes, documented as "time
    only advances when explicitly told to, so timestamps and the byte-day rent integral are
    reproducible run to run (no wall-clock reads)" — day-granularity, which is the granularity a
    14-day retention window needs. It currently has **no callers** in CISS (a dormant type ported
    from `item-storage-protocol-standalone/src/clock.ts`); the spike is its first. This downgrades
    the stand-in from "the spike invented a fake clock" to "the spike uses the substrate's own
    deterministic clock," and it removes the invented `Clock` trait from `queue.rs` entirely.
  - **CISS process — in-process axum router, registered.** Verified firsthand that no CISS crate
    ships a server binary: `ciss-cli` declares the only `[[bin]]` (`ciss-ctl`) and it is a *client*
    (`put`/`get`/`login`/`whoami`/`sync`, no `serve`); `ciss-iroh`, `ciss-sync`, `ciss-auth`,
    `ciss-resolve` declare none. So the in-process router is not a shortcut past an existing binary
    — it is the only way to run CISS without adding one, and it is how CISS's own suite drives
    itself. Registered as `meer-spike-ciss-inproc`.

- `[CONFIRMED: RESOLVED — decided now, not phase-gated]` **How high should S8's group-size sweep go,
  and what is the time budget?** *The spec suggests up to 1000 "and as high as the harness sustains."*
  **Resolution (2026-08-08, user): go as high as it takes — the full sweep is pre-authorized,
  including N ≥ 1000, however long it runs.** S8 is the measurement that picks between three design
  options, and a sweep that stops just below the crossover is the worst outcome. D6 still runs (it
  sizes expectations and catches a pathological cost curve early), but it is no longer a decision
  gate. **Consequence: Phase 11 has no ceiling to document as "dropped" unless the harness fails
  outright** — and if it does, the failure mode itself is the finding and gets recorded as such.

- `[CONFIRMED: PHASE-GATED (Phase 8) — treatment settled]` **Is prose-only treatment of S4's
  with-device-group arm acceptable?** *§6.6.5 fan-out is not built, so that arm cannot be Rung A
  without standing in for the exact mechanism the claim is about — forbidden.*
  **Resolution (2026-08-08, user): prose plus a tracked Rung-A follow-up.** The without-device-group
  arm runs at Rung A; the with-arm is reasoned in writing, marked unambiguously as reasoning rather
  than measurement, and generates an explicit Rung-A follow-up item that keeps the question open in
  the residue. This is the methodology's own stand-in pattern (§3 Rung B, §6): a result that does not
  exercise the real mechanism does not retire the question.

- `[CONFIRMED: ADVISORY — sequential]` **Should the parallel set (Phases 7–10 alongside Phase 11) be
  used?** **Resolution (2026-08-08, user): run everything sequentially.** Simpler to follow, easier to
  attribute a failure, and the benefit was bounded anyway — cargo serializes the builds, so only S8's
  compute would have overlapped the other phases' authoring time. **The Concurrency Map's parallel set
  is retained as analysis** (the disjointness holds and is worth having on record if a re-run ever
  wants it), but the executor runs the sequential spine. `S8-RESULTS.md` is kept: it was introduced to
  make the write-sets disjoint, and it remains useful as a place for a long measurement table that
  would otherwise crowd `TEST-LOG.md`.

- `[RECOMMENDED: PHASE-GATED (Phase 9)]` **Add S9 — drain the queue with no meer in the loop?**
  *Raised by the owner 2026-08-09, reviewing the walkthrough: the "your mail never leaves home"
  claim is **not compelling as user value**, and that critique is correct. A user does not experience
  namespace ownership; the mail is sealed either way, so ownership adds no confidentiality; and the
  queue is a 14-day transient buffer, not an archive. The claim does real work for anti-entrenchment
  (an operator/governance property) but was being presented as user-facing value it cannot carry.*
  *The owner's reframing is stronger and, unlike the original, **falsifiable**: if the queue really
  lives in the recipient's namespace, the meer is a **writer**, not a required **reader** — any
  authorized client could drain straight from CISS (`ciss-sync`, a second meer, a user script, or
  nothing at all). The claim becomes "**no single service sits on the critical path for reading your
  own mail**."*
  *Proposed S9: after a normal publish, drain the queue **with the meer process absent**, straight
  from CISS, and assert the recipient recovers the same sealed bytes. Cheap — the harness and the
  queue entries already exist by Phase 9. Caveat: under `meer-spike-namespace` the queue lives in the
  meer's namespace, so S9 would prove the **mechanism** (a meer-free read path exists) and not yet the
  **entitlement** (that it is the recipient's own namespace being read). That limit must be stated in
  the verdict, same as S6's.*
  *Owner's position as of 2026-08-09: interested, not decided ("I'm not there yet"). Nothing before
  Phase 9 depends on the answer, so this does not gate execution.*

## Review Log

### Pass 1: Plan development — 2026-08-07

**Grounding performed:** Read all four orientation docs. Read `CISS/src/server.rs`,
`CISS/src/blobstore.rs`, `CISS/src/manifest.rs`, `CISS/tests/common/mod.rs`,
`CISS/rust-toolchain.toml`; `mls-replant/{Cargo.toml,src/lib.rs,README.md}`;
`mls-welcome-over-iroh/{Cargo.toml,src/main.rs,src/node.rs}`; openmls 0.8.1 source for
`application.rs`, `processing.rs`, `config.rs`, `message_in.rs`, `message_out.rs`;
`SPEC-DIVERGENCE-REGISTER.md` conventions. Confirmed local toolchain version.

**Two spec expectations identified as likely wrong:**
1. S8's table lists commit growth as `~log N`; `mls-replant` already measured the sparse case as O(N).
   Recorded as a band with a pre-registered prediction, to be measured rather than assumed.
2. The hypothesis doc's "two OpenMLS clients (the MIT `cli` PoC)" is stale; two better in-process
   ancestors exist in-workspace. Correction scheduled in Phase 13.

**Two stand-ins added beyond the spec's three:** `meer-spike-clock`, `meer-spike-ciss-inproc`.

### Pass 2: Gap analysis — 2026-08-07

**Found:**
- **Phase 0 was absent from the Pass-1 draft.** Six assumptions were carrying multiple later phases
  (cross-repo path dep, CISS auth shape, the re-frame API for M2's negative arm, `mls-replant`'s
  signer visibility, iroh relay portability, S8's feasible ceiling). Rework on any of them would be
  multiplicative. Phase 0 added with D1–D6.
- **D3 is the sharpest gap.** M2's negative arm needs a decode/re-encode path in openmls 0.8.1, and
  `MlsMessageIn` does not obviously expose one — `message_in.rs:96–116` shows `extract()`,
  `wire_format()`, `try_into_protocol_message()`, but no serialize. Planning the negative arm around an
  assumed API would violate the methodology's own rule in the test whose entire purpose is to report
  real library behavior.
- **Port collision risk in the copied relay.** `mls-welcome-over-iroh/src/main.rs:53` hardcodes ports
  3340/3343/3478/9099. Copying it unchanged into a crate that may run tests concurrently is a flake
  source. Phase 4's shared-state contract now requires ephemeral or per-test-offset ports.
- **S2's storage measurement could have measured our own bookkeeping.** Added the requirement that the
  figure come from CISS's `/{did}/du` or meter, independently of what we handed it.
- **S6 could pass for the wrong reason.** Under `meer-spike-namespace` the mail lives in the meer's
  namespace, not Bob's — so "it never left home" is not fully testable here. Phase 9 now requires the
  verdict to name what the stand-in prevents it from proving.
- **Phase 5's "offline" needed sharpening** — a flag the meer reads is not an absence. Now a real
  endpoint teardown.
- **M2's positive arm was assertion-only in the draft.** Strengthened to a structural property: the
  `meer` module takes no openmls dependency, asserted via `cargo tree`.
- **Documentation Impact was under-specified** on the `beta/drystone-spec/` question. Resolved: the
  spike flags normative text, it does not edit it.

**Concurrency:**
- Pass 1 declared all phases sequential. The missed-parallelism check found one genuine candidate:
  Phase 11 (S8) depends only on Phase 2, binds no ports, opens no network, and is the only
  compute-dominated phase.
- The blocker was that S8 wrote into `TEST-LOG.md`, colliding with Phases 7–10. **Fixed by giving S8
  its own `S8-RESULTS.md`**, merged in Phase 12. This is a plan change made *to enable* the
  parallelism, recorded here so it is not mistaken for an arbitrary file split.
- Re-entry verification added to every phase in the parallel set: write-set-scoped `git status`,
  unchanged HEAD, loopback listener count returned to baseline, and no premature `TEST-LOG.md` merge.
- Shared-state contracts upgraded from mechanisms to invariants throughout — "binds no ports" rather
  than "runs in isolation."

**Changed:**
- Added Phase 0 (D1–D6). Renumbered implementation phases 1–14.
- Added `S8-RESULTS.md` to Documentation Impact and to Phase 11's write-set.
- Added the `cargo tree` module-graph assertion to Phase 6's wiring test.
- Added the `/{did}/du` cross-check to Phase 7's validation.
- Added the stand-in-limits requirement to Phase 9's S6 verdict.
- Added the grep-to-register correspondence check as Phase 12's wiring test.
- Added Open Questions 1 and 3 gating on D1 and D6 respectively.

**Confirmed:**
- Every phase touches at most 3 files; no phase needs splitting under the 4-file rule.
- Every phase has a wiring test that exercises the entry-point path, not just an isolated module.
- The fidelity rung is declared per phase in advance, so no result can be written up at a rung it did
  not earn.
- Every cited file:line in Verified Assumptions was read firsthand during this session.
- The pre-registered falsification list in `SPIKE-SPEC.md` maps onto Phases 6, 8, 9, and 11, and
  Phase 13 walks it item by item.

### Open-question walk-through — 2026-08-08

All five questions confirmed with the user. Two were overridden or decided differently from the
recommendation, and one of those changed the plan's substance.

**Found (via the Q2 override):**
- The user overrode Q2 from ADVISORY to BLOCKING, which forced a probe before acceptance. That probe
  found **`CISS/src/clock.rs` — a public, unit-tested `SimClock`** that exists for precisely the
  reason S5 needs one ("time only advances when explicitly told to, so timestamps and the byte-day
  rent integral are reproducible run to run"), at day granularity, which is the granularity a 14-day
  retention window works in. The plan had specified an **invented** spike-local `Clock` trait in
  `queue.rs`. That was a real defect: the spike would have asserted a parallel pattern beside one the
  substrate had already chosen. **Recorded as a lesson about severity: a stand-in the executing agent
  finds convenient is exactly the kind that should require a decision rather than an acknowledgement.**
- The same probe confirmed the other half firsthand rather than by inference: **no CISS crate ships a
  server binary.** `ciss-cli` declares the only `[[bin]]` (`ciss-ctl`), and it is a client
  (`put`/`get`/`login`/`whoami`/`sync`; no `serve`). `ciss-iroh`, `ciss-sync`, `ciss-auth`, and
  `ciss-resolve` declare none. The in-process router is therefore not a shortcut past something that
  exists — it is the only option short of changing CISS.

**Changed:**
- **Q1 (BLOCKING, resolved):** path dependency to `../../../../CISS`. D1 keeps probing the mechanics;
  the policy is settled.
- **Q2 (overridden to BLOCKING, resolved):** `queue.rs` now uses `ciss::clock::SimClock` directly; the
  invented `Clock` trait is deleted from the plan. Reasoning section rewritten ("Why CISS's SimClock
  rather than an invented seam"), including the rejected wall-clock alternative and why. The
  `meer-spike-clock` register row is retained but now records the weakest form of the divergence.
  `meer-spike-ciss-inproc` accepted as specified, with the no-binary finding recorded as evidence.
- **Q3 (decided now rather than phase-gated):** the S8 sweep is pre-authorized with no time ceiling.
  D6 survives but is demoted from a decision gate to an early-warning probe for a pathological cost
  curve. Phase 11's risk framing shifted from "we run out of budget" to "we stop early and call it
  covered," and a harness failure at some N is now explicitly a finding rather than a non-result.
- **Q4 (confirmed PHASE-GATED, treatment settled):** prose plus a **tracked** Rung-A follow-up. Added
  `alpha/ROADMAP_TODO.md` to Phase 8's write-set and to Documentation Impact — the follow-up is part
  of the phase's "done," filed in the phase that creates the debt, not a later cleanup.
- **Q5 (confirmed ADVISORY, sequential):** the executor runs the sequential spine. The Concurrency
  Map's parallel analysis is **retained on record** rather than deleted — the disjointness holds and a
  re-run may want it — but is marked NOT executed. `S8-RESULTS.md` is kept: it was introduced to make
  write-sets disjoint, and it still earns its place as somewhere to put a long measurement table.

**Confirmed:**
- No phase exceeded the 4-file rule after these edits; Phase 8 now touches 3 files.
- Every open question is resolved before execution. Phase 0 no longer gates a decision — only
  assumptions.

### Pass 3: Quality Gates — 2026-08-08

Run in the same context as Passes 1–2 at the user's request, rather than the fresh context the skill
prescribes. Compensated by checking every claim against source rather than against the plan's own
prose — which is what surfaced the S8 finding below.

**TDD ordering:**
- **Phase 3 was building untested production code.** "Sweep expired, leave a watermark" was written in
  Phase 3 but not exercised until S5 in Phase 9 — six phases of code with no failing test behind it,
  and dead code by the plan's own "built means wired" rule. Moved to Phase 9, written against the S5
  test that drives it. Phase 3 now builds exactly what `w2_queue_diff` exercises, and its goal line
  says "four of the five operations" so the omission is deliberate rather than an oversight.
- **Phase 5 asserted on an accessor no phase created.** M1 checks `meer_payload_keys_held == 0`, but
  Phase 5's write-set was tests-only and Phase 3 never planned a `keys_held()`. Added to Phase 5,
  test-first — not backfilled into Phase 3, where nothing would have exercised it.
- **Phase 7 called for a `du` read that no phase built.** Added to Phase 7's write-set.
- Marked "RED first" explicitly on the S-phase tests, which the Pass-2 draft left implicit.

**Mutation resistance:**
- Phase 1 tested only `MAX_OBJECT_BYTES + 1`. A cap check is a comparison, and testing one side of it
  lets an off-by-one survive. Added the at-boundary accept case (exactly 2 MiB).
- Phase 2's "sealed bytes do not contain the plaintext" is survived by *any* transformation. Added:
  the bytes must parse as `MlsMessageIn` with `wire_format() == PrivateMessage` — specific to a real
  seal rather than to obfuscation.
- Phase 3's `w2_queue_diff` was a happy-path assertion on branching set logic. Added empty-queue
  drain, unknown-digest-in-have-set, and double-drain idempotence.
- Phase 7's S2 "five recipients → one blob" is survived by an implementation that always stores one
  object. Added one-recipient and two-distinct-messages edges.
- Phase 9's S5 tested only "much later." The 14-day rule is a comparison **and** carries an "or until
  drained" clause, so added the at-boundary case and the drained-before-expiry case (which must leave
  *no* watermark — a false gap report would violate the no-invisible-loss rule it exists to serve).

**Observability:**
- The plan had **no logging story at all** for a spike with three independent failure surfaces
  (openmls, axum, iroh). Added a Reasoning subsection and put `tracing_subscriber` + `EnvFilter` into
  Phase 2's `src/lib.rs`, matching `mls-welcome-over-iroh/src/main.rs:45–48`. Levels chosen so that
  expected negative outcomes (M2's re-frame rejection, S7's decrypt failure) are **results, not
  errors** — logging them as errors would train the reader to skim past real ones. Verdict lines go to
  stdout so they survive any filter setting.

**Debugging readiness:**
- CISS already emits `tracing::info!` at the object boundary with method, DID, key, and byte count
  (`server.rs:1443`, `:1469`) — named in the plan, because that is the record M2's digest chain and
  S2's dedup count need when they disagree.

**Validation calibration:**
- Phase 3's validation called for inspecting "the CISS blob directory," but no phase had chosen a blob
  backend and `Blobs::Memory` (CISS's test default) has no directory. Phase 1 now specifies
  `Blobs::Fs(tempdir)`, which also upgrades S2 from one accounting source to two (CISS's `du` **and**
  an on-disk file count).
- Phase 1 now builds with **`App::with_limits`, not `App::new`** — `App::new` calls
  `Limits::from_env()` and reads `CISS_MAX_STORE_BYTES` / `CISS_MAX_DID_BYTES` (`server.rs:242`,
  `:176–188`). An ambient store ceiling could have failed S2 or S8 for an unrelated reason. Phase 1's
  shared-state contract corrected: it had claimed no env involvement.

**Discovery:**
- **D4 partly resolved during planning** rather than deferred: `Persona`'s fields are all `pub`,
  including `signer` and `provider` (`mls-replant/src/lib.rs:22–28`), so the "does the spike need its
  own persona type" branch is closed. D4 narrows to confirming the call compiles and round-trips.
- All six tasks keep an explicit Disposition; the four marked `promote` each name their follow-up
  phase.

**Concurrency honesty:**
- **The Pass-2 parallel set is invalidated by Pass 3's own edits.** Adding `src/mls.rs` to Phase 11 —
  and `src/meer.rs`, `src/queue.rs`, `src/ciss_harness.rs` to Phases 5/7/9 — means the two branches now
  overlap on source files that the other branch *reads*, not just on `TEST-LOG.md`. Operationally
  irrelevant (sequential was already chosen), but the recorded analysis would have misled a future
  re-run that trusted it. Corrected in place and marked INVALIDATED rather than deleted.
- Phase 9 now touches 5 files, over the 4-file rule. Kept whole with an explicit justification: the
  sweep implementation and the S5 test that drives it are one RED-GREEN cycle, and splitting them
  would recreate the untested-code defect this pass just removed. S6 is flagged as the split point if
  the phase does not fit one context window.

**Coherence:**
- **The S8 prediction was extrapolating from the wrong configuration — the most consequential finding
  of this pass.** `MlsGroupJoinConfig` derives `Default` with `use_ratchet_tree_extension: bool`, so
  it defaults to `false`, and `MlsGroupCreateConfig::default()` delegates to it (`config.rs:43,56,102–113`).
  `mls-replant::stamp_kps` uses that default. **So every Welcome measurement the corpus has is the
  *without*-tree case — the safe case — and the O(N) object S8 exists to find has never been measured
  here.** The plan had cited the flat ~152–155 B/member figure as if it bounded the risk. Corrected:
  the commit prediction stands, the Welcome/GroupInfo-with-extension rows now have **no prior and no
  prediction**, which is the honest position and raises S8's value.
- Consequently **S8 cannot be built on `mls-replant::stamp` at all** — it also discards the `GroupInfo`
  (`let (commit, welcome, _gi) = ...`). Phase 11 gains its own config-parameterized construction in
  `src/mls.rs`. Had this not been caught, Phase 11 would have measured the wrong thing and reported it
  as the answer.
- **A reframing worth carrying into Phase 13:** `mls-replant` already ships the ratchet tree out of
  band (`Stamp.ratchet_tree`, passed separately to `join`). So S8's "option 3" is the corpus's de-facto
  status quo, arrived at incidentally rather than decided — which inverts the spike spec's framing,
  where option 3 reads as the change. Options 1 and 2 are the departures.

**Documentation impact:**
- `TEST-LOG.md`'s format is now named: methodology §5 per-result, explicitly **not**
  `iroh/TEST-LOG.md`'s chronological campaign narrative — so the executor does not improvise a third
  convention.
- This plan file's own status line added to Phase 14.

**Environment note:**
- The extracted openmls source was evicted from `~/.cargo/registry/src/` between Pass 2 and Pass 3
  (the `.crate` tarballs remain cached). Citations stand — the pin is exact `=0.8.1` and the source was
  re-extracted to verify this pass's findings — but re-checking them needs a `cargo build` or
  `tar xzf` first. Recorded so a future reader does not conclude the citations were fabricated when
  the path does not resolve.

**Post-pass consistency sweep (same session, after the user asked whether the file was actually up to
date):** re-read the file rather than trusting the edit log. Found and fixed six stale artifacts of
the invalidated parallel grouping — Phase 11's goal line still read "Runs in parallel with Phases
7–10," and Phases 7–11 still carried live-looking "In the parallel set" / "Re-entry verification"
fields. All now marked *does not fire — sequential run*, with the Pass-2 text retained rather than
deleted so the reasoning trail survives. Phase 11's re-entry entry additionally notes that the
write-set it named went stale when Pass 3 added `src/mls.rs`.

**Confirmed ready:** yes. All five open questions were confirmed by the user before this pass; none
were reopened by it, and no new ones were raised. Every phase has a wiring test that exercises its
entry point, and every verification command runs through the call chain rather than an isolated
module.

### Phase 0: Discovery — executed 2026-08-08

Seven probes run (six planned, one added). Full record in
`alpha/experiments/meer-queue/PHASE-0-FINDINGS.md`; summary table in § Phase 0 outcomes above.

**Found:**
- **D3 falsified the spec's M2 negative-arm hypothesis.** A decode→re-encode round trip is
  **byte-identical** (189 B application message and 490 B commit, same sha256 both times) because
  TLS-codec serialization is canonical. The spec expected "detectably different at the byte level."
  Separately, both conversions that make a re-frame possible are gated behind openmls's `test-utils`
  feature, each carrying the source comment *"break abstraction layers and MUST NOT be made available
  outside of tests."* Consequence: the `MUST` stands but its stated rationale does not — the hazard is
  **re-sealing** (needs a key the meer lacks), not re-framing (byte-preserving). Phase 6 restructured
  around three new assertions; Phase 13 gains a normative-text flag against Part 2 §6.6.2.
- **D2: the over-cap refusal is axum's, not CISS's.** `2 MiB + 1` is rejected at HTTP **413** by
  `DefaultBodyLimit` before the request reaches the blobstore, so `ObjectTooLarge` is a second line of
  defence the HTTP path never exercises. Phase 1's wiring test asserted the wrong enforcer.
- **D1: `Limits` has no `Default`.** Fields are `pub`, so the spike builds it by literal — which is
  what we wanted anyway (a stated ceiling, not an inherited one).
- **D5: dial by `Endpoint::addr()`, not bare `EndpointId`.** `presets::Minimal` configures no DNS
  discovery; a bare id fails with `No addressing information available`. Also: the copied relay's
  fixed ports needed replacing with `RelayPorts::ephemeral()`, as Pass 2 predicted.
- **D7 (added during Phase 0, closing a gap Pass 3 left).** Pass 3 established that S8 cannot use
  `mls_replant::stamp` but created no probe to confirm the replacement exists. It does — and it
  produced the **corpus's first tree-ON measurements**: the extension roughly doubles per-member
  Welcome cost (~153 → ~333 B/member), `GroupInfo`-with-tree runs ~180 B/member at N=200, and commit
  bytes are identical with the extension on or off.

**Changed:**
- Phase 6 rewritten: negative arm replaced with (i) a compile-time unreachability assertion in the
  default build, (ii) the byte-identity measurement recorded as the falsification, (iii) a structural
  assertion that the meer cannot re-seal. Verdict must report a confirmed positive arm **and** a
  falsified negative hypothesis.
- Phase 1: assert HTTP 413, not a CISS error; `Limits` by literal.
- Phase 4: dial by `addr()`; relay-spawn risk retired as tested.
- Phase 11: extend D7's reading rather than repeat it; record that with-extension rows had no prior.
- Added a `reframe = ["openmls/test-utils"]` cargo feature, on **only** for M2's negative arm.
- `PHASE-0-FINDINGS.md` added to Documentation Impact.

**Confirmed:**
- No probe invalidated the spike's premise; the design is testable as scoped.
- D6 shows the full S8 sweep is cheap (500 members in 126 ms, roughly linear), so Open Question 3's
  pre-authorization costs little.
- The `meer-spike-drain-auth` stand-in works as specified — the responder reads the caller's
  `EndpointId` off the authenticated connection.
- The relay fallback the plan hedged against was not needed.
- Every `promote` probe names the phase that re-implements it test-first; the one `throwaway` (D6)
  produced no code to carry forward.
