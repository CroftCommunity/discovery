# Two-target delivery — clearing the blockers, then proving the whole workflow

date: 2026-08-12
status: **planned, not started.**

**Design:** `alpha/thinking/meer-two-target-delivery.md` (walked out at Rung A, S9–S12)
**Evidence:** `alpha/experiments/meer-queue/TEST-LOG.md`, `S8-RESULTS.md`
**Sequences:** `CISS/docs/plans/2026-08-11-object-lifecycle.md` (E95) — referenced, not duplicated
**Backlog:** E91, E95, E96, E97

---

## Problem statement

The two-target delivery design is measured working except for **two capabilities**, both in CISS:

1. **A third party cannot write into an owner's namespace.** Measured: HTTP **403** (S12). Without
   it there is no personal inbox, and therefore no way to reach someone you have never met.
2. **Nothing can be deleted.** Measured: CISS has no object `DELETE` (S5). Without it "14 days, then
   expunge" is a claim we cannot honour.

Everything else — group queue naming, drain-by-capability, catch-up, read gating, KeyPackage
publication and fetch, the full stranger handshake — works today at Rung A.

### The first blocker is not what the design originally called for

The hypothesis doc specified **custodian mode**: a *named* helper holding a revocable grant to append
to the owner's queue chain. **The two-target design does not need that**, and noticing why matters:

- The **group queue is pooled in the meer's own namespace**, so the meer writes to *itself*. No
  third-party write is involved at all.
- The **inbox** receives from **unnamed strangers** — anyone who has your KeyPackage. There is no
  grant to issue, because there is nobody to issue it to in advance.

So the capability needed is narrower in one way (no grant lifecycle, no per-custodian revocation) and
harder in another (**it cannot be an allowlist**). Calling it "custodial write" would import the
wrong mental model. This plan calls it **third-party deposit**.

### The question this exposes, which has no answer yet

**Who pays?** `op_put_object` gates on the namespace owner's ceiling and `append_receipt` binds the
receipt to the **namespace DID**. So today a deposit into A's namespace **bills A** — the
spam-costs-the-victim problem, concrete rather than theoretical. Phase 0 settles this before any
code.

## Approach

**Four gates, in order. Each is a place to stop.**

```
  P0 discovery ─► P1 declare ─► P2 accept ─► P3 bound ─► P4 retire stand-in
                                                              │
  E95 object lifecycle (separate plan) ───────────────────────┤
                                                              ▼
                                                    P5 holistic workflow
```

Nothing after P4 is worth starting if P0 says the economics do not work.

## Reasoning

**Why not an allowlist.** The inbox must accept a first message from someone you have never met —
that is its entire purpose. Any pre-authorization scheme defeats it. So the gate can only be
*authentication* (who, verifiably) plus *bounds* (how much), never *authorization* (may they).

**Why the write gate cannot be a capability.** S11 refuted the appealing candidate: making a
consumed KeyPackage the write token **inverts**, because anyone who can read a published KeyPackage
can build a valid `Welcome` against it, so a passer-by can burn the owner's supply and deny
legitimate invitations. The bound lands on the owner's reachability rather than the attacker's
effort.

**Why unwanted invitations cannot be prevented at all.** Also S11: a stranger *can* seat you in a
group you never asked to join. That is MLS as specified. So the honest target is **bounded and
accountable**, not unsolicited-free — the posture email reached, for the same reason.

**Why this does not reduce what CISS must build.** The fabric model briefly made third-party write
look unnecessary. It is necessary again, for the inbox. The work moved; it did not shrink.

## Verified assumptions

Read firsthand 2026-08-12, not from memory:

- **Third-party write is refused today** — HTTP 403, measured (S12,
  `tests/s12_personal_inbox.rs`). Invariant **Z2**: *"writes and the meter are owner-only"*, enforced
  by `require_owner` (`SECURITY-POSTURE.md` §368).
- **Receipts bind to the namespace DID**, not the writer — `Store::append_receipt(did, receipt)`
  (`persist.rs:927`). Ceilings gate on the namespace owner (`op_put_object`).
- **`Authorship` has no third-party variant** — `Derived | OwnerSigned | ProviderSigned | CoSigned`
  (`kind_spec.rs:40`). A deposit is authored by neither the owner nor the provider.
- **`Erasure::Erasable` and a generic DELETE exist for assertions** (A2), **not for objects** —
  `/{did}/objects/{addr}` is still `put().get()`.
- **Read gating works and its default does not** — `read_class: owner` gives owner 200 / stranger 404
  / anon 404; **unset, a namespace is world-readable** (measured, S12).

## Phases

### Phase 0 — Discovery (blocking; the economics gate)

- [ ] **D1: Who pays for a deposit?** Today the receipt binds to the namespace owner, so A pays for
      what B sends. Decide: **depositor pays** (needs the meter to attribute to a non-owner, a real
      change), **owner pays under a ceiling** (simple, and the ceiling is the whole defence), or
      **neither — deposits are unmetered below a size/rate bound** (simplest, and creates a free-tier
      hole). **Nothing else in this plan is worth building until this is answered**, because it
      decides whether the inbox is safe to expose at all.
- [ ] **D2: Where does a deposit's authority live?** `Authorship` has no third-party variant. Is a
      deposit a new variant, a property of the *kind* (an "inbox" kind that accepts them), or a
      policy record (`write_class`, the `[PLANNED]` sibling of `read_class`)? Read ADR 0005 and the
      gated-reads spec before choosing; the third is most likely and least invasive.
- [ ] **D3: What does Z2 actually protect, and what weakens if a third party can write?** Z2 is
      stated as an invariant. Enumerate what depends on it — billing integrity, the manifest's
      meaning, B1/B3 — and confirm which survive a bounded, authenticated exception.
- [ ] **D4: Is the object plane or the assertion plane the right home for the inbox?** Assertions
      already have DELETE, LIST, declared kinds and a seq. Objects have the byte path and the 2 MiB
      cap. A `Welcome` is bytes, but it is also a bounded record. **Decides whether E95's object
      DELETE is even on this plan's critical path.**

**Done when:** the payment model is decided and written down, and the inbox's home (plane +
authority) is chosen with its reasoning.

### Phase 1 — Declare an inbox (CISS)

**Goal:** an owner can declare that a slot accepts third-party deposits, with a ceiling. Nothing
accepts them yet.
**Wiring test:** the declaration round-trips over HTTP; a malformed one is refused; an unknown kind
is still refused (**the fail-closed property must not regress**); and **without a declaration,
behaviour is exactly as today** — that last assertion is the one that protects every existing
namespace.
**Doc:** the ADR amendment lands in this commit, not later.

### Phase 2 — Accept a third-party deposit (CISS)

**Goal:** a verified non-owner DID can deposit into a declared inbox. Still unbounded.
**Wiring test:** B deposits, A reads it back byte-identically; **an unauthenticated deposit is
refused**; a deposit into an **undeclared** namespace is still 403; a deposit by a DID that fails
verification is refused.
**Risk:** this is the invariant-weakening commit. Z2 changes meaning here, and `SECURITY-POSTURE.md`
must change with it **in the same commit** — a posture doc that lags the code is worse than none.

### Phase 3 — Bound it (CISS)

**Goal:** abuse is bounded and attributable.
**Changes:** the owner-declared ceiling enforced on deposits; per-depositor-DID rate limiting;
metering per D1's decision.
**Wiring test:** deposits past the ceiling are refused **and the refusal names the bound**; one
depositor cannot exhaust the ceiling faster than the rate limit allows; the meter attributes as D1
decided. **Both edges of the ceiling**, since it is a comparison.
**Done when:** a hostile depositor can waste a bounded, owner-chosen amount and no more.

### Phase 4 — Retire the stand-in (discovery)

**Goal:** S12's handshake runs with a **real** stranger deposit.
**Changes:** `tests/s12_personal_inbox.rs` — remove the owner-write stand-in; delete
`meer-spike-owner-write-standin` from the register.
**Wiring test:** the full handshake, deposit performed by Bob as himself.
**Done when:** the register has one fewer row and the correspondence check still passes.

### Phase 5 — Object lifecycle (CISS)

**Separate plan** — `CISS/docs/plans/2026-08-11-object-lifecycle.md`. Sequenced here, not duplicated:
A (manifest-driven reclamation, owed by atproto-compat regardless) then B (declared expiry as a
seventh `KindSpec` axis). **D4 may move this off the critical path** — if the inbox lives on the
assertion plane, it inherits A2's DELETE and only the *group queue* needs object expiry.

### Phase 6 — The holistic workflow test (discovery)

**Goal:** one test that walks the entire system as a user experiences it, with **nothing stood in**.

```
  Bob and Alice have never met
    │
    ├─ Alice publishes a KeyPackage to her namespace         (S12: works)
    ├─ Bob fetches it, validates it, creates a group         (S12: works)
    ├─ Bob DEPOSITS the Welcome into Alice's inbox           (Phase 2: new)
    │    └─ bounded by Alice's ceiling, billed per D1        (Phase 3: new)
    ├─ Alice is OFFLINE — endpoint genuinely torn down       (M1: works)
    ├─ Bob sends messages; the group commits several times   (S10: works)
    ├─ Alice returns, reads her inbox, joins                 (S12: works)
    ├─ Alice derives the group queue name and walks the chain (S9/S10: works)
    ├─ Alice reads every missed message                      (S10: works)
    ├─ Retention elapses; the meer expunges                  (Phase 5: new)
    └─ Alice sees an honest watermark for what is gone       (S5: works)
```

**Wiring test:** the above, single test, real OpenMLS + real CISS + real iroh over a real relay.
**Assertions that must hold, each already measured in isolation:** bytes byte-identical end to end;
the meer holds zero group keys; a stranger cannot read Alice's inbox; a non-member cannot derive the
queue name; expired bytes are **gone**, not merely unserved.
**Validation:** broad. This is the first time the pieces run together, so the failure most worth
watching for is an **interaction** nobody tested — particularly the handover from inbox to group
queue, and expiry racing a drain.
**Done when:** one test tells the whole story, and the SPEC-DELTA register is down to the stand-ins
that are genuinely about the substrate rather than about missing capabilities.

## Open questions

- `[BLOCKING]` **Who pays for a deposit?** D1. Decides whether the inbox is exposable at all.
- `[PHASE-GATED — Phase 1]` **Which plane hosts the inbox?** D4. Decides whether E95's object DELETE
  is on this plan's critical path or parallel to it.
- `[ADVISORY]` **Does the group-context nomination (E97) belong in this plan?** It is needed to *use*
  a meer in production but not to *prove the workflow*, which can name the meer directly. Currently
  excluded to keep the critical path short.
