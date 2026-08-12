# Meer delivery — where we are, and what to pick up next

`Written 2026-08-12 at the end of a long session, as the handoff artifact.`
`Read this first; it points at everything else.`

---

## The one-paragraph version

The meer Phase-0 spike ran to completion and **the central claim held**: a blind store-and-forward
node with no ordering, no group state and no key carries a real MLS conversation across an absence.
Then re-reading Part 2 §5.4 showed the spike had tested an **addressed** meer while the spec
describes a **fabric** one, which reshaped the design into **two delivery targets** — a group queue
keyed by a shared secret, and a personal inbox keyed by identity. Both are now measured at Rung A.
**Two capabilities are missing, both in CISS**, and both are planned.

## Read in this order

| doc | what it is |
|---|---|
| `../../thinking/meer-two-target-delivery.md` | **the current design.** Supersedes the delivery shape in `meer-as-custodian-queue.md` |
| `TEST-LOG.md` | every result, with fidelity rungs. M1, M2, S1–S14 |
| `S8-RESULTS.md` | the object-size sweep vs the 2 MiB cap |
| `PHASE-0-FINDINGS.md` | the seven discovery probes that preceded the build |
| `../../plans/2026-08-12-1-plan-two-target-delivery-blockers.md` | **what to build next** |
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

## What is missing — two things, both in CISS

1. **Third-party deposit.** A stranger cannot write into an owner's namespace: measured **HTTP 403**.
   Without it there is no inbox. **Not** "custodian mode" as originally designed — the group queue is
   pooled in the meer's own namespace, so the only third-party write is from **unnamed** strangers,
   and it therefore cannot be an allowlist.
2. **Object lifecycle.** CISS has no object `DELETE`, so "14 days then expunge" cannot be honoured.
   Plan exists (E95); owner's decision is **both** halves, A then B.

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

## Where the code is

```
alpha/experiments/meer-queue/
  src/    ciss_harness · mls · meer · queue · transport · relay · node
  tests/  w0–w3 (wiring) · m1, m2 (must-pass) · s2–s14 (scenarios)
  src/bin/ d1–d7 (Phase-0 discovery probes, still runnable)
```

`cargo test` → **53 tests**, seconds. `cargo clippy --all-targets` → clean.
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
· **E95** (object lifecycle) · **E96** (nested sealing — *unblocked* by the queue name) · **E97**
(announcement — resolved; groups are self-locating).

## Suggested next steps

1. **Answer the payment question** (Phase 0 D1 of the blockers plan). It gates everything.
2. **Decide the inbox's plane** (D4). May take E95 off the critical path.
3. **Then build** third-party deposit → bound it → retire the stand-in → object lifecycle → the
   holistic workflow test in Phase 6 of the blockers plan.

Experiments that would still teach something, if experiments are wanted before building:

- **The limbo state end to end** — currently asserted as a policy comparison, not walked.
- **The two-part credential** — S14 confirmed §11.7's *key* half (external commit); the **governance
  attestation** half is untested.
- **Nested sealing (E96)** now that the queue name unblocks it — does an outer seal still route?
