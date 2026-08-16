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

**The readmission/exclusion arc (S15-S22, G1) is measured and documented; what remains there is
decision work, tracked in the dossier and backlog:**

- **E105 — who serves `GroupInfo`?** Answered structurally: **any member** (Part 1 §2.4 — no
  chokepoint; S22). What remains is the group-context serving policy and the tree-withholding
  default.
- **E106 — rewrite §11.7** around the governance-issued external PSK (drafted as a `[REV]` in the
  WORKING copy; ratification is the owner's).
- **E107 — the readmission dial's open thirds:** position 2 as an end-to-end admission decision, and
  the propagation window quantified.
- **E108 — what a contradicted group projects** (G1's surviving finding; two candidate rules in the
  WORKING copy §7.3.2 REV).
- **E96 — adopt nested sealing, or don't.** Cost measured (28 flat bytes); a decision, not an
  experiment.
- **Spec-2 candidate review:** the WORKING copy's 13 `[REV]` blocks await the owner's merge-back
  pass. **Canonical part-2 remains untouched until then.**
