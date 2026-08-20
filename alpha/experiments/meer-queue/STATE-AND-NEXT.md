# Meer delivery — where we are, and what to pick up next

`Written 2026-08-12; amended through 2026-08-16 (S15-S22, G1, the scenario walk). The handoff artifact.`
`Read this first; it points at everything else.`

---

## The one-paragraph version

The meer Phase-0 spike ran to completion and **the central claim held**: a blind store-and-forward
node with no ordering, no group state and no key carries a real MLS conversation across an absence.
Then re-reading Part 2 §5.4 showed the spike had tested an **addressed** meer while the spec
describes a **fabric** one, which reshaped the design into **two delivery targets** — a group queue
keyed by a shared secret, and a personal inbox keyed by identity. Both are now measured at Rung A.
**Two capabilities are missing, both in CISS**, and both are planned.

**Then S15-S22 + G1 (2026-08-13 → 16) interrogated exclusion and readmission end to end**, produced
the **readmission dial**, and corrected several of our own conclusions along the way. The current
synthesis is NOT in this file — read, in order:

1. `beta/drystone-spec/DOSSIER-exclusion-and-readmission-2026-08-16.md` — the findings, as a dial
2. `beta/drystone-spec/SCENARIO-WALK-2026-08-16.md` — Appendix E's L1–L6 walked, the 44-row matrix,
   and G1 (the §7.3.1 fold checked against its own keys)
3. `beta/drystone-spec/part-2-certifiable-design-WORKING-2026-08-16.md` — the candidate spec-2
   revisions, 13 `[REV 2026-08-16]` blocks. **Canonical part-2 is untouched by design.**

Headlines: S15 walked limbo (escapable; corrected S14). S16: **§11.7's credential is not
implementable as written** — a governance-issued **external PSK** replaces both halves (E106). S17
built nested sealing at **28 flat bytes** (E96). S18: a removal is only as durable as `GroupInfo`
distribution, refusal holds at two layers, **a fork is invisible in the epoch counter** (E107). S19:
the epoch roll locks **derivation**; external join **never derives** — two doors, and no "safe"
`GroupInfo` exists. S20: the owner's N=10 ban scenario **confirmed** at AEAD grade; re-entry is
**self-admission** and the window is exactly the not-yet-synced. S21: one shared secret per epoch —
the invite path is gateable in MLS's own **proposal phase**; the external-join path has no such
phase. S22: **every member is a serving peer** (Part 1 §2.4 — no chokepoint), so a **negative**
standing check fails open at the least-synced peer while a **positive** credential fails closed —
**dial position 2 is the ban posture that holds**. G1: the §7.3.1 fold **hard-stops the realistic
ban-vs-rejoin race order-independently** (confirmed), the comparator was **aligned to key 3**
(v2, versioned, rebuild-tested), and the one open item is **what a contradicted group projects**
(E108).

## Read in this order

| doc | what it is |
|---|---|
| `../../thinking/meer-two-target-delivery.md` | **the current design.** Supersedes the delivery shape in `meer-as-custodian-queue.md` |
| `TEST-LOG.md` | every result, with fidelity rungs. M1, M2, S1–S22 |
| `S8-RESULTS.md` | the object-size sweep vs the 2 MiB cap |
| `PHASE-0-FINDINGS.md` | the seven discovery probes that preceded the build |
| `../../plans/2026-08-12-1-plan-two-target-delivery-blockers.md` | **what to build next** |
| `TEST-LOG.md` → S15–S22 | the 2026-08-13→16 exclusion/readmission arc, and the corrections it forced |
| `../../../beta/drystone-spec/SCENARIO-WALK-2026-08-16.md` | L1–L6 walked · the 44-row matrix · G1 (the §7.3.1 fold) |
| `../../../beta/drystone-spec/DOSSIER-exclusion-and-readmission-2026-08-16.md` | the readmission dial |
| `CISS/docs/plans/2026-08-11-object-lifecycle.md` | the other blocker's plan |
| `CISS/docs/notes/2026-08-11-reachability-audit.md` | why five CISS modules are unreachable |

## What is measured working (Rung A unless noted)

- **M1** — an offline member drains and decrypts; meer holds zero group keys.
- **M2** — byte-identical forwarding. *Its negative-arm hypothesis was falsified:* a re-frame is
  byte-identical, so the `MUST` stands on stronger grounds (the hazard is re-**sealing**).
- **Group queue** — named by `export_secret("croft/meer-queue/v1")`; members agree, non-members
  cannot derive, rotates per epoch, drained by name over real iroh (S9, S10).
- **Catch-up** — 124 ms for 10 missed epochs, ~12 ms/hop. N counts **governance events, not
  messages** (S10).
- **Personal inbox** — necessary (no queue name without group state), read-gated (`read_class:
  owner`: owner 200 / stranger 404 / anon 404), and the full stranger handshake works end to end
  (S12).
- **Handover** — a joiner's first queue is the epoch her `Welcome` seated her in; earlier history is
  **unnameable**, so the MLS privacy boundary and the queue-addressing boundary coincide (S13).
- **§11.6 / §11.7 alignment** — the queue name *is* a liveness indicator; migration to cold severs
  queue access with no mechanism; self-service re-entry by external commit works (S14).

## What S15-S17 added (2026-08-13)

- **S15 — limbo is real, reachable, and escapable.** A member 15 days absent is simultaneously
  seated in the hot Group, holding a watermark of lost mail, and able to name **exactly one** queue
  (the stale one). **Correction to S14:** she *can* re-enter by external commit — openmls does not
  distinguish "cold" from "stranded". **But the escape needs a current `GroupInfo` and NOTHING SERVES
  ONE** — not the group queue (unnameable to her by construction), not the inbox (`Welcome`s only),
  and a `GroupInfo` is not a queued object at all. **E105.** Constructively: retention set to the
  liveness window makes the same absence cost nothing, now enforced by
  `Meer::sweep_with_retention`.
- **S16 — §11.7's credential does not exist.** MLS checks **no standing whatsoever**: a party who was
  never a member joined on a `GroupInfo` alone and the incumbent merged it. And a **resumption PSK
  cannot be attached to an external commit at all** on openmls 0.8.1 (resolved from the group's own
  store, which an external-commit group initialises empty; `add` is `pub(crate)`). **A
  governance-issued EXTERNAL PSK carries both halves** and works today. The policy hook is complete
  and pre-merge: AAD, sender kind, and the joiner's credential. **E106.**
- **S18 — a removal is as durable as `GroupInfo` distribution, and no more.** A deliberately
  removed member **re-seated herself** on a current `GroupInfo` alone. **But refusing holds at two
  independent layers** — she cannot decrypt what a refuser sends (real AEAD failure), and she cannot
  even *name* the queue it sits in. **The admission surface is the ratchet tree, not the
  `GroupInfo`** (withhold it and re-entry is refused; the export flag is independent of the group
  config, so this must be enforced wherever `GroupInfo` is served). **And a fork is invisible in the
  epoch counter** — two branches agree on the number and share no secrets. **E107.**
- **S17 — nested sealing works.** `group_id` absent under the outer seal, object no longer parses as
  MLS, **28 flat bytes** (measured flat at 64 KiB), routing/dedup/byte-identity/catch-up all
  unaffected, non-member refused with a real `AeadDecryptionError`. **One new rule:** wrap at the
  epoch of the **queue**, so the commit that *closes* an epoch is wrapped at the epoch it closes —
  verified from the failing side, where getting it backwards deadlocks the walk silently.

## What is missing — three things now, two in CISS

1. **Third-party deposit.** A stranger cannot write into an owner's namespace: measured **HTTP 403**.
   Without it there is no inbox. **Not** "custodian mode" as originally designed — the group queue is
   pooled in the meer's own namespace, so the only third-party write is from **unnamed** strangers,
   and it therefore cannot be an allowlist.
2. **Object lifecycle.** CISS has no object `DELETE`, so "14 days then expunge" cannot be honoured.
   Plan exists (E95); owner's decision is **both** halves, A then B.
3. **A `GroupInfo` channel** (E105, new 2026-08-13, and *not* in CISS). Without it §11.7's
   self-service re-entry cannot execute at all, so a stranded member has no path. Note it is an
   **admission surface**, not a convenience: S16 measured that a `GroupInfo` alone admits a stranger.

## The open questions that matter

- **`[BLOCKING]` Who pays for a deposit?** Receipts bind to the **namespace DID**, so a deposit into
  A's namespace bills **A** — spam costs the victim. Three options in the plan, none free. **Nothing
  else is worth building until this is answered.**
- **Retention must be ≥ the Group's liveness window.** At `RETENTION_DAYS = 14` the meer is shorter
  than **seven of §11.6's eight** windows, creating a limbo state: live in the hot Group, unable to
  catch up, not yet cold, so neither recovery path applies. Working figure **30 days**; properly it
  is a **per-Group governance value**, not a service constant.
- **Which plane hosts the inbox?** Assertions already have DELETE/LIST/declared kinds; objects have
  the byte path. **Decides whether E95 is on the critical path or parallel to it.**

## Things that would be easy to get wrong later

Each of these was learned the hard way in this session and is cheap to re-break:

- **Dispatch on the cleartext `content_type` before processing.** `process_message` consumes the
  message key; try-decrypt-then-fall-back destroys group state (S3b, S10).
- **Consult the watermark before concluding you are caught up.** A swept queue and an empty queue
  return identical empty drains (S13).
- **`read_class` defaults to world-readable.** An inbox that forgets to set it is public. This
  belongs in provisioning, not documentation (S12).
- **`EndpointId` is for rate limiting, never authorization.** Authorizing on it lets the meer build
  a device→groups map across every queue it serves.
- **Validate a fetched KeyPackage.** The convenient `From<KeyPackageIn>` conversion is
  `test-utils`-gated precisely because it skips validation.
- **Do not raise `MAX_OBJECT_BYTES`** without streaming first — the cap came from a real
  memory-exhaustion finding.
- **If you adopt nested sealing, wrap at the epoch of the QUEUE** (S17). The commit that *closes* an
  epoch is wrapped with that epoch's key, derived **before** committing. Backwards, the walk
  deadlocks silently and looks like data corruption.
- **A `GroupInfo` is an admission surface, not a lookup** (S16). Handing one out lets the receiver
  join. Do not build the E105 channel as if it were public metadata.

## Where the code is

```
alpha/experiments/meer-queue/
  src/    ciss_harness · mls · meer · queue · transport · relay · node · outer_seal
  tests/  w0–w3 (wiring) · m1, m2 (must-pass) · s2–s22 (scenarios)
  src/bin/ d1–d7 (Phase-0 discovery probes, still runnable)
```

`cargo test` → **85 tests**, seconds. `cargo clippy --all-targets` → clean.
S8's sweep is `#[ignore]`d (release-only, ~50 s):
`cargo test --release --test s8_object_sizes -- --ignored --nocapture --test-threads=1`
M2's negative arm needs `--features reframe`.

**Seven stand-ins**, all tagged in code and rowed in `../SPEC-DIVERGENCE-REGISTER.md`; correspondence
is checked by `tests/m2_byte_identity.rs`. The two that matter most:
`meer-spike-addressed-deposit` (the spike's meer is addressed; the spec's observes) and
`meer-spike-owner-write-standin` (the inbox deposit is owner-performed because 403).

## Backlog

**E91** (meer lane) · **E92** (device-group arm — likely dissolved by the fabric model) · **E93**
(Part 2 §6.6.2 rationale corrections) · **E94** (graph leak — an artifact of the addressed model)
· **E95** (object lifecycle) · **E96** (nested sealing — **built and measured, S17**) · **E97**
(announcement — resolved; groups are self-locating) · **E105** (nothing serves `GroupInfo` — new,
S15) · **E106** (§11.7's credential not implementable as written — new, S16) · **E107** (removal
durability + the invisible fork — new, S18).

## Suggested next steps

1. **Answer the payment question** (Phase 0 D1 of the blockers plan). It gates everything.
2. **Decide the inbox's plane** (D4). May take E95 off the critical path.
3. **Then build** third-party deposit → bound it → retire the stand-in → object lifecycle → the
   holistic workflow test in Phase 6 of the blockers plan.

**The readmission/exclusion arc (S15-S22, G1) is measured and documented. The decision
talk-through ran 2026-08-16 (decisions 1–2 of five); current status:**

- **E106 — RATIFIED (owner, 2026-08-16).** The governance-issued external PSK is §11.7's
  re-entry credential, with four scoping conditions recorded in the WORKING copy's §11.7 REV
  (scoped to the external-commit path; "I was there" homed on the governance chain; lineage
  binding by three-way cross-check, design/untested; conditional on the end-to-end proof).
- **E105 + E107 — DECIDED IN SHAPE, PRELIMINARY pending build.** Two issuance mechanisms
  (at-join canonical; at-need-with-deposit where meer + CISS exist), the token-ledger
  obligation, three serving doors as a charter attribute, strict-merge finality floor,
  serve-protects-roster / merge-protects-membership, admission evaluated at-position. All in
  the WORKING copy's §11.6/§11.7 DECISION-2 blocks. **Nothing merges until S23–S26 run green:**
  `../../plans/2026-08-16-1-plan-token-reentry-proveout.md` (S23 token ledger · S24 end-to-end
  admission decision + serve protocol · S25 stale-peer matrix ± finality gate · S26 catch-up
  replay determinism). **This build is the next go.**
- **E108 — RATIFIED (owner, 2026-08-17): rule (c) — `CONTESTED` as a first-class membership
  state, visible by default** (not a dial). Closes G1's projection divergence by construction;
  outstanding: the member-view schema change in `fold_derived` + the arrival-order pinning test
  (croft-chat, separate from the meer-queue build).
- **E96 — RATIFIED (owner, 2026-08-17): nested sealing as an attribute-conditioned MUST.**
  Mechanism + wrapping rule MUST when in effect; charter attribute defaults ON for
  confidential-on-fabric, OFF for LAN/public-regime; the deviation is the declared
  `carrier-visible` attribute (E94 transparency guard), never a silent exception. **Adopted
  generally as Drystone normative style: attribute-conditioned MUST over SHOULD.**
- **New rows from the talk-through:** E109 (confidentiality regime as a charter attribute;
  transitions are re-plant-shaped), E110 (admission interface as an A-series requirement set),
  E111 (the implementation-profile dial sheet; Croft as reference profile).
- **2026-08-17 — the independent review ran, and amended the build.** Verdict: the decisions are
  sound; findings + missed-issues register in
  `../../../beta/drystone-spec/REVIEW-decision-talkthrough-2026-08-17.md`. Out of it (owner
  talk-through, same day): the **admission fact** (every external-commit admission deposits an
  R6-shaped, span-opening acceptance record — typed to never compete on the membership slot), the
  **HeadAck head-currency primitive**, S24/S25 amendments (forged-ledger arm; banned-holder
  population arm — "fails CLOSED at a stale peer" is population-dependent), and the gate aligned
  to **S23–S26 + C2–C4**. Plan: `../../plans/2026-08-17-1-plan-head-currency-and-admission-fact.md`;
  row **E112**; handoff prompt for the build session:
  `../../seeds/generated-prompts/c-series-proveout-prompt.md`. **This build is the next go**
  (supersedes the bare S23–S26 framing above).
- **Spec-2 merge (step 5): all four decisions are now in; merge scope is the open question.**
  Ratified and evidence-complete: the corrections (§11.8 + App E L6 strike, §11.6 three states,
  §11.7 recovery bounds, §11.11 sharpenings), E106, E108, E96, cold-is-a-state. Decision-2
  mechanism text is agreed-in-shape but build-gated (S23–S26). Options: merge ratified-only now
  with decision-2 in a second pass post-build, or one clean merge now with decision-2 text
  carried at ***Design, preliminary — gated on S23–S26*** (house style precedent: §11.9.3's
  "Design, experimental — prototype before relying"). **Canonical part-2 remains untouched
  until the merge pass runs.**

---

## 2026-08-17 (evening) — E112 built and run: the C-series + amended S-series are GREEN

The build session for `../../plans/2026-08-17-1-plan-head-currency-and-admission-fact.md` ran. All
of the gate (**S23–S26 + C2–C4**, C5 informative) is green, RED-first where the plans designate it
(S23 arm 1, C2 arm 1). Work is on the `c-series-proveout` worktree branch, not merged.

**Fold side** — `local_storage_projection` (`../local_storage_projection/C-SERIES-RESULTS.md`),
Modeled/loopback grade:
- **C2** (behind-detection) — 2 arms. New `src/head_currency.rs`. Arm 1 (RED-first) wires the §7.4
  fail-closed gate to the fold's real `MissingAntecedents` signal. Arm 2 proves the ack primitive
  is *needed* (governance-only detection is silent under ordinary traffic).
- **C3** (HeadAck) — 4 arms. New `src/head_ack.rs`: a §7.3.4 sign-the-state object; union counts
  distinct **lineages** (§5.7); forged fails (typestate); unknown head is a gap not authority.
- **C5** (ack cost, informative) — the scoped/lazy-piggyback dial is a constant ops/finality volume
  reduction, N-independent; a volume lever, not a safety one.

**Meer side** — `meer-queue` (`TEST-LOG.md`), Rung A for the MLS half / Modeled governance plane:
- **S23** (token ledger) — 3 arms. Missing ledger = clean loud `process_message` refusal (not a
  silent drop); ledger must reach late-joiners; revocation is a chain fact (no key-deletion race).
- **S24** (position-2 end-to-end + admission fact) — 8 arms. New `src/admission.rs`: the admission
  fact mints or the merge does not happen; arm (d) severs fact-from-bytes (REVIEW gap 1); serve
  s-i/s-ii; perishability (real MLS).
- **C4** (Bob/Dana end-to-end) — 4 arms. Real MLS seat + counted exposure window + re-key exclusion;
  arm 1a pins the comparator placement (admission fact opens a span, never competes on the standing
  slot — byte-identical in both arrival orders); arm 3 holds the routine/genuine boundary.
- **S25** (stale-peer matrix, amended) — 5 arms. Consumes C3's HeadAck as the strict-gate freshness
  (new dev-dep on `local_storage_projection`); banned-holder population arm retires the unqualified
  "fails CLOSED at a stale peer" (REVIEW verdict item 1).
- **S26** (catch-up at-position) — 3 arms. At-position evaluation; the head-anchored mutation is
  caught; stale-majority invalid joins are corrected governance-forward.

**Review coverage gaps closed (section H / item 24):** gap 1 (positive chain-fact leg / fact-from-
bytes) — S24 arm (d); verdict item 1 (population-dependent fails-closed) — S25 banned-holder arm;
verdict item 2 residual (the routine admission-vs-ban never CONTESTS) — C4 arm 1a + arm 3; the
orphaned §11.8 positioning MUST (item 23.3) — the admission fact's span-opening + C4/S26 forward
correction; add-commit-as-mint-point (gap 6) — S24 makes it an asserted property.

**What is explicitly NOT discharged (honest rungs):** the Appendix-B completeness beam's *proof*
statements (these runs earn evidence, mapped to obligations 1–4, not the theorem); real transport
(C3's iroh-bus arm is a named, un-run upgrade); real crypto on the fold path (mock digest-binding);
the governance plane on the meer side (issuance ledger, acceptance chain, serve challenge-response
are Modeled). Loopback is **Modeled**, never **Verified**.

**STOP LINE (this session does not cross it):** the WORKING §11.7/§11.6 REV blocks stay
PRELIMINARY, canonical part-2 is untouched, and the six-artifact gate-wording alignment
(S23–S25 vs S23–S26; review item 25) is left for the owner. Graduation of decision-2's REV and the
step-5 merge are the owner's next conversation.

---

## 2026-08-19 — GRADUATED AND MERGED: step 5 is closed

The graduation-and-merge session ran (owner-directed: one clean merge, plus the L7 beat).
**Canonical part-2 now carries the whole readmission arc**: the corrections (§11.8
zero-marginal-exposure strike + the two orphaned-paragraph rewrites, §11.6 three-states), E106
(the governance-issued token + four scoping conditions), E96 (attribute-conditioned sealing MUST +
wrap-once + the style principle), E108 (`CONTESTED`, with the set-valued pair-carrying schema
requirement), cold-is-a-state (family dial qualified per the review), decision-2 (ledger, doors,
walk-in mechanics with evidence tags, pull/push split, layered gates with the §7.3.8 qualifier,
at-position ↔ at-head reconciliation), the **admission fact**, the E109 regime-transition bridge
(§8 ↔ §11.9.3/§11.10), §11.11 items 4/6 rewritten to post-gate status, and Appendix E updated
(L2/L3/L6 fixed; **L7** added — the stale admission and the one-directional roll). Gate wording
aligned to **S23–S26 + C2–C4 (green 2026-08-17)** across the WORKING copy, ROADMAP
E96/E105/E106/E107/E108/E110/E111/E112, both plans, and this file. The WORKING copy is marked
MERGED/historical; canonical governs. **What remains** (E112 residuals, on the row): the
serve-signature surface, ledger hygiene + pricing, door-A end-to-end, lapse/invite-unification
tests, rung upgrades, the croft-chat E108 implementation, and the Croft presentation obligations
(riding E111). Next fronts: E110's A-series writing, E111's profile template, and — separate
track — Phase 11 on the client side (`CroftCommunity/connect` `docs/PHASE11-HANDOFF.md`).

## 2026-08-19 — E110 RETIRED: the A-series is written into canonical §10.2.2

The admission-interface consolidation landed as canonical **§10.2.2** ("The admission interface:
requirement versus realization"): the A1–A8 compliance table (A8 = every admission deposits its
admission fact, per the step-5 merge), disqualifiers (each anchored to the measured failure it
excludes), and a role-by-role realization mapping — MLS supplies the artifacts, §11.7/§11.8 the
decision layer, so a K-bar substitute replaces the artifact column only. The pull/push split and
invite-lifecycle unification are folded in under A4 (the unification stays `Design`/untested —
E112 residual). §10.5 gained an admission-interface ledger row; §11.7 and the §11 §0 map point at
the consolidation. Evidence basis unchanged: S12/S16/S18/S21 for the shapes, the S23–S26 + C2–C4
gate (green 2026-08-17) for the composition. Next fronts: E111's profile template, then per the
post-merge queue.
