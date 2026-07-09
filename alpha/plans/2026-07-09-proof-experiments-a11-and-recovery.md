# Proof-experiment plan — A11 revocation-immediacy spike + A2/A12 recovery-anchor prototype

date: 2026-07-09 · author-read source: `../../beta/DECISIONS.md` (2026-07-09 block);
`2026-07-09-engineering-validation-plan.md` (WS2 recovery-anchor prototype, WS4 T40 attestation);
`../../beta/OPEN-THREADS.md` (T22, T36, T40, T30); `../../beta/drystone-spec/part-2-certifiable-design.md`
§5.3/§5.4/§5.5, §7.2, §7.3.9, §7.4.2, §11.9.3, Appendix A (Track A/B entry), Appendix B (the beam);
`../../beta/cairn/willow-meadowcap.md` (Track A / Meadowcap); `../../beta/cairn/object-capability-and-decentralized-mls-prior-art.md`
(Track B / Keyhive maturity + DMLS/FREEK cost curve); `../../beta/impl/delivery-layer/08-experiment-methodology.md`
(the Rung A/B/C fidelity ladder); `../../beta/impl/experiments/drystone-experiments-consolidated.md`
(Stage 8 recovery ladder, Stage 9 tenure); `../../beta/impl/delivery-layer/12-replant-experiments.md` (E12 set);
`../../beta/drystone-spec/part-1-reasoning-underpinnings.md` §2.8 (faithful representation).

> **What this is.** A PLAN — experiment designs, success criteria, and sequencing for two decided-but-unproven
> items. It plans the proofs; it does **not** run them, and it edits no spec or thread doc. Thread numbers,
> spec sections, and E-numbers are cited for traceability; beta docs are referenced by path. Beta-tier
> discipline does not bind an `alpha/plans` doc, but the epistemic vocabulary here is the spec's own (§1.1
> status ladder; `08-experiment-methodology.md` fidelity ladder A/B/C) so plan and corpus speak one language.
> Every experiment below is tagged with its **Rung**, the **real library/tool** where applicable, the
> **claim it tests**, and the **pass/fail criterion** — the four things `08-experiment-methodology.md` §5
> requires of a result before it is admissible as evidence.

---

## Problem statement

Two items were **decided in direction** in the 2026-07-09 open-gate walkthrough (`../../beta/DECISIONS.md`)
but remain **unproven in mechanism**. Each is currently tagged `Design` / `[confirm]` / `Load-bearing,
unearned` in the spec, not `Verified`. The proofs below are what turn them.

1. **A11 — the capability mechanism.** Decision deferred to a **revocation-immediacy spike**: pick **Track A
   (Meadowcap-shaped: delegated attenuable tokens + per-Group epoch keys; no native revocation — revoke =
   decline-to-renew, revocation latency bounded by the epoch/expiry interval)** vs **Track B (Keyhive-shaped:
   convergent membership/capability graph; removal + re-encryption first-class; generation-bounded near-
   immediate revocation)**. Spec homes: §7.2 (R1–R6, the mechanism-neutral grant/revocation interface, tagged
   `Design`), §5.5 (the Meadowcap grounding), Appendix A (the Track A/B "Alternatives Considered" entry, which
   states plainly that **both satisfy R1/R2/R3/R6 identically — only revocation immediacy differs** — and
   names the exact next step: *define Drystone's revocation needs concretely, then test each track against
   them*). **On the v0.1 DOI critical path:** the **capability wire format is the last `ENABLING` encoding**
   that makes §7.2 buildable from text alone, hence the last domino before the Zenodo DOI is mintable
   (T30 sequencing; `2026-07-09-engineering-validation-plan.md` critical-path sketch).

2. **A2 + A12 — the recovery anchor.** The **principles are decided and fixed** (do not re-decide): the meer
   is **always blind** (invariant, never holds usable keys, §5.4); recovery is a **separate custodial role**
   with **conditional / break-glass** access (not standing read); it **composes** a social-recovery **quorum
   and/or a designated custodial delegate** over a **self-custody floor**; a **group-level default** plus an
   optional **per-user** designation; the `did:plc` rotation key (A10) folds into this custody. §7.3.9 records
   this as *decided direction; mechanism pending* and defers, verbatim, five parameters: *the exact access
   conditions, how the quorum and the delegate compose, the group-default arrangement's shape, the break-glass
   delay and contest window, and the recovery secret's encoding.* The prototype must fill exactly those.

The proofs move A11 from `Design`/deferred → a **decision memo + chosen capability wire-format direction**
(unblocking the DOI), and the recovery item from §7.3.9-pending → a **recovery mechanism spec (k, n, delay,
conditions, encoding) + a working re-provision-after-total-loss prototype** → `Verified`.

```
        A11 decided ──► capability wire format pinned ──► §7.2 buildable from text ──► v0.1 DOI mintable
                                                                                        (the publish road)

        recovery mechanism proven ──► §7.3.9 five parameters filled ──► §7.3.9 Design → Verified
                                                                        (the prove road, raises toward rc)
```

---

## Approach

### Fidelity discipline (binding on every experiment below)

Per `08-experiment-methodology.md`: every result states its **Rung in the verdict line**, pins and prints
exact library versions, and never substitutes a stand-in for the exact component a claim is about
(XOR-as-MLS is the canonical forbidden move). Rung A = real library. Rung B = model-form, naming what was
stood in; a Rung-B `CONFIRMED` does **not** retire a `[confirm]` about the real mechanism — it opens a
tracked Rung-A follow-up. Rung C = static / spec-check. A **FALSIFIED result is a first-class success**.

E-number namespaces below (`E-A11.*`, `E-REC.*`) are **new**, deliberately not colliding with the existing
`E1`–`E12` delivery-layer sets or the Stage 1–9 consolidated-experiments series. Where an experiment restates
a consolidated Stage as an executable measurement, the coupling is named (e.g. E-REC.1 ⇄ Stage 8 Group V).

### Host: the reference-implementation workspace (not greenfield)

Every Rung-A experiment below is built **in / against the existing reference implementation** —
`experiments/alpha/croft-chat/` (the `social-graph-core` + `group-chat-core` crates behind the Transport
port; the `croft-chat` CLI shell; substrate = the mutation-vetted `local_storage_projection` redb crate).
This is **WS0** of the engineering-validation plan (`2026-07-09-engineering-validation-plan.md`): the
workspace closed at P20 (2026-06-27) already proving multi-node iroh convergence + the §7.6 hard-stop, and
these proofs extend it rather than spinning up throwaways. Concretely: the A11 capability layer is added to
`social-graph-core` (or a `capability-core` sibling) and exercised through the CLI's real iroh transport;
the recovery experiments extend the same session/identity surface and reuse the P18–P20 convergence +
fingerprint/diff harness. Rung-A "real library" therefore means *the reference impl's own dependency-pinned
stack* (mls-rs/openmls, willow-rs/Meadowcap, iroh, redb), not a separate rig. A throwaway rig is allowed
only for a Rung-B model where the real component is unshippable (Keyhive, E-A11.B).

---

## Track A11 — revocation-immediacy spike

The Appendix A entry already isolates the decision to **one axis: revocation immediacy at acceptable cost**.
So the spike is (0) fix the bar, (A) measure Track A against it, (B) model Track B against it, then apply an
explicit decision rule. Everything R1/R2/R3/R6-shaped is held constant across tracks and is **not** re-tested
here — the spec establishes it holds identically either way.

### E-A11.0 — Threat model: define the acceptable revocation window (the pass/fail bar)

- **Type / Rung:** `needs-content` + adversarial analysis. **Rung C** (spec-check / static — this defines the
  yardstick; it exercises no runtime).
- **Claim it tests:** that Drystone *has* a concretely-stated revocation-immediacy requirement — the "needs
  definition" Appendix A names as the precondition for settling A11. Without this, neither track can be
  scored.
- **What it produces.** The **acceptable revocation window** = the gap between decision-to-revoke (the
  governance fact folds) and effective-revocation (no honest third party will accept the revoked grant),
  stated as an **epoch / membership-graph-generation bound, never a wall-clock interval** (Part 1 §2.0.1;
  §7.2 R4). Stated as a hard pass/fail latency bar against three threat scenarios:
  - **C10 ban-evasion re-add** (§11, T5 sub-corpus) — a banned actor re-adds via a new device leaf; the
    window in which a stale-but-unrevoked capability still reads.
  - **Kick-a-bad-actor** (routine expulsion cadence) — the moderation baseline: how fast a normal expulsion
    must take effect to be credible.
  - **Equivocation** (§7.3.1 concurrent-tiebreak surface; consolidated Stage 5 C9) — a member acting on both
    sides of a partition; the window a revoked-but-unsynced holder can exploit.
- **Pass/fail criterion (the bar itself).** Output a single **target window `W_target`** expressed in epochs/
  generations, plus a **complexity budget** and a **cost ceiling** (rekey frequency, bandwidth, MLS commit
  rate the deployment can absorb). `W_target` is the number every downstream A11 experiment is scored against.
- **Dependency:** **must complete before E-A11.A and E-A11.B** — it is the ruler they are measured with.

**Tradeoffs.** The single axis is **revocation-window tightness vs epoch/rekey cost**, and it does not resolve to one honest number. A tight window means frequent epoch turnover — high MLS commit rate, more bandwidth and battery per member, more forks to reconcile under partition, and a heavier FREEK-style storage curve. A loose window is cheap but leaves a wide read/act window for a banned actor re-adding (C10), a routine expulsion that has not yet taken effect, or an equivocating member on the wrong side of a partition. A **single global bar** is simplest to specify, prove, and reason about, but it forces the entire system to pay the cost of its most safety-sensitive use (interactive moderation) even in tiers that do not need it (large broadcast) — because epoch cost scales with group size, that overcharge is exactly where it hurts most. A **tiered bar** matches cost to need, but the honest downside is real: more configuration surface, a matrix of behaviors to test, a tier boundary that itself becomes a governance decision, and the failure mode that an abuser simply operates in the loosest tier — so the loosest tier must still be safe on its own terms, not merely cheap.

**Proposal (for review).** Adopt a **tiered `W_target`, expressed in epochs/generations**: interactive and small groups target **`W_target` = 1 epoch** (revocation effective by the next commit boundary); large/broadcast tiers tolerate a small bounded **`W_target` ≤ ~3 generations** to amortize rekey cost — because a tight interactive window is what makes a ban or expulsion credible while a broadcast tier cannot afford per-message rekey, so the bar should track the tier's actual safety need rather than a one-size compromise. The specific tier counts and the exact per-tier numbers are a product/safety judgment (how fast must a ban take effect) and are **the maintainer's call to accept or adjust** — this proposal fixes the *shape* (tiered, generation-denominated, loosest tier safe on its own) and offers defensible starting values, not a settled constant.

**Decided refinement (2026-07-09, with the maintainer): revocation is two-phase, and the bar is a layered structure, not one latency.** Because *all state and representation is local*, split revocation into two layers:

- **Phase 1 — experiential (immediate, local, free).** The moment the ban fact crosses the governance threshold, every peer's client **auto-ignores / filters** the revoked party as standard behavior. This needs no rekey and no round-trip — it is a local-representation change, so it is instant. This is the **ATProto-block-shaped layer**: exactly how an `app.bsky.graph.block` works today (AppView-layer experience-shaping, immediate in-app, *no* cryptographic exclusion) — see `../../beta/cairn/atproto-selfhosting-appviews-and-bridges.md` (the inbound/outbound blocking asymmetry, "experience-shaping, not visibility-control"). It couples the T3 moderation "block-as-revocation" lever and the `fenced/app-store-survivability-and-abuse-posture.md` blind-lever set.
- **Phase 2 — cryptographic (epoch-bounded).** The revoked party loses the key and can no longer decrypt *new* state, effective at an epoch roll.

**The force-immediate-roll lever (always available).** A group can force an epoch roll on demand, paying the rekey cost, to collapse phases 1 and 2 into one — closing the read-residual at once for the severe case.

**Default phase-2 timing tiers on the force-roll cost, which = f(group size, connectivity):**
- **Small (< ~50) or well-connected:** a forced/immediate roll is cheap, so *immediate* cryptographic revocation is viable as the norm — phase-2 effectively rides phase-1.
- **Hundreds+ or poorly-connected:** an immediate roll is expensive (high MLS commit rate, bandwidth, more forks to reconcile), so the **slow natural epoch roll is the default** and force-roll is reserved for the severe case, paid knowingly. Here the epoch-bounding is a *feature*, not a limitation.

**The honesty caveat (must be explicit in UX + governance).** Between phase 1 and phase 2 the revoked party can **still decrypt new messages** (including discussion of their own removal) — not merely "still appear." For an ordinary ban this is an acceptable awareness-residual with clear UX; for a **severe** case (a harasser, must-not-see-content) it is a genuine exposure, which is exactly what the force-roll lever is for. Never present phase 1 as cryptographic exclusion.

**Consequence for A11.** With immediacy carried by phase 1 regardless of capability track, the phase-2 epoch latency is a read-residual, not an abuse/participation window — so **Track A (Meadowcap, epoch-bounded) is near-certainly adequate**, and Track B's generation-bounded near-immediate revocation is not required (it becomes the documented future upgrade). E-A11.A now measures the *force-roll cost curve vs group size/connectivity* (where the ~50 boundary and the ≤~3-generation loose-tier bound land), not merely a fixed epoch window. The tier numbers stay tunable and are confirmed in E-A11.A; the *shape* (two-phase, force-roll lever, cost-tiered default) is decided.

### E-A11.A — Meadowcap track: measure the effective revocation window vs epoch length

- **Type / Rung:** `needs-experimentation`. **Rung A on `willow-rs` / Meadowcap if the real revocation-by-
  epoch path is exercisable**; else **Rung B model-form**, naming the stand-in (a faithful epoch-key +
  attenuable-token model — Meadowcap's Data Model + capability layer are `Final` per `willow-meadowcap.md`,
  but the Rust impl is pre-1.0, so real-lib exercise of the *epoch-key revocation* path may not be reachable;
  if not, Rung B and say so). Per-Group epoch keys ride on **`mls-rs` (0.55.2, pinned)** for the actual rekey
  — an epoch is an MLS commit — so the cost half is Rung A on real MLS regardless.
- **Claim it tests:** that Track A's **revocation window is a function of epoch length**, and whether a
  short-epoch configuration clears `W_target` (E-A11.0) at acceptable cost.
- **What it measures.**
  - **Effective revocation window as f(epoch length):** revoke = decline-to-renew at the next epoch boundary,
    so the window ≈ one epoch. Sweep epoch length; record the window in generations/epochs.
  - **Cost of short epochs:** rekey frequency, bandwidth per rekey, and **MLS commit rate** at
    representative hot-N (couple the WS1 §11.10.1 A–G matrix; reuse its `mls-rs`-on-aarch64 rig rather than
    duplicating). This is where the FREEK cost curve bites — see risks.
- **Pass/fail criterion.** **PASS** iff there exists an epoch length such that the effective window ≤
  `W_target` (E-A11.0) **and** the resulting rekey/commit cost ≤ the E-A11.0 cost ceiling. **FALSIFIED** (a
  first-class result) if no epoch length satisfies both — i.e. Track A cannot clear the bar without cost that
  exceeds budget. Verdict line names the Rung explicitly.

### E-A11.B — Keyhive track: model generation-bounded revocation latency + convergence cost + maturity risk

- **Type / Rung:** `needs-research` + `needs-experimentation`. **Rung B (model-form / paper-analysis)** — this
  is the expected rung, because **Keyhive is IN-FLIGHT Ink & Switch research, not shipped** (per
  `object-capability-and-decentralized-mls-prior-art.md`: Keyhive/Meadowcap references are `[dialogue-sourced
  2026-06-24, pending verification]`; the DMLS/FREEK siblings are drafts/PoC with *no production deployment as
  of mid-2026*). No Rung-A verdict is expected; any Rung-B `CONFIRMED` opens a tracked Rung-A follow-up that
  cannot be closed until Keyhive ships.
- **Claim it tests:** Track B's **generation-bounded revocation latency** (near-immediate: removal +
  re-encryption are first-class convergent ops) and its **convergence + re-encryption cost**, plus an explicit
  **research-maturity risk** estimate.
- **What it estimates/models.**
  - Generation-bounded revocation latency: the window ≈ one membership-graph generation to converge — model
    it and compare to `W_target`.
  - Convergence + re-encryption cost: the DMLS/FREEK cost curve is the honest anchor — recovering forward
    secrecy after forked/out-of-order commits costs **storage that scales with retention window × group size ×
    fork frequency** (~8 kB per PPRF evaluation; `object-capability-...` doc). Model this against the E-A11.0
    complexity budget.
  - **Research-maturity risk:** unshippability as a first-class output — Track B on the DOI critical path
    means waiting on external in-flight research.
  - **If feasible:** a *minimal convergent-capability-graph model* (Rung B) — enough to sanity-check the
    generation-bound latency claim, not a production construction. Explicitly optional; do not block the memo
    on it.
- **Pass/fail criterion.** This experiment does not "pass/fail" a build; it **produces a scored estimate**:
  (latency vs `W_target`, cost vs budget, maturity-risk rating). The scoring feeds the decision rule below.

### The decision logic (explicit)

```
   ┌────────────────────────────────────────────────────────────────────────┐
   │  E-A11.0 fixes W_target + cost ceiling                                   │
   └───────────────┬────────────────────────────────────────────────────────┘
                   ▼
   ┌───────────────────────────────┐        short-epoch Track A clears
   │  E-A11.A: does short-epoch     │  YES   the bar at acceptable cost
   │  Track A window ≤ W_target     │ ─────► ┌──────────────────────────────┐
   │  at cost ≤ ceiling?            │        │ PICK TRACK A NOW.            │
   └───────────────┬───────────────┘        │ Unblocks the capability wire │
                   │ NO                      │ format → the DOI, WITHOUT    │
                   ▼                         │ waiting on Keyhive.          │
   ┌───────────────────────────────┐        └──────────────────────────────┘
   │  E-A11.B: invest in Track B    │
   │  (Keyhive), accept the         │
   │  in-flight-research timeline.  │
   └───────────────────────────────┘
```

**One line:** *if short-epoch Track A clears the E-A11.0 threat-model bar at acceptable cost, pick A now (it
unblocks the DOI without waiting on Keyhive); only if A cannot clear the bar do we invest in B and accept the
timeline.*

- **Deliverable:** a **decision memo** (which track, scored against E-A11.0, with the FALSIFIED/PASS evidence)
  **+ the chosen capability wire-format direction** — the input that lets T30 pin the last `ENABLING` encoding
  and mint the DOI. Feeds back into §7.2 (remove "the mechanism is deferred"), Appendix A (resolve the Track
  A/B entry), and the T40 attestation-credential shape (WS4).

**Tradeoffs.** The choice trades **revocation immediacy against shippability and timeline risk**. Committing to Track A accepts revocation that is **epoch-bounded, not instantaneous**, plus the short-epoch cost the E-A11.0 tiered bar implies (higher commit rate and bandwidth on the tight tiers) — but Track A is buildable today (Meadowcap Data Model + capability layer are `Final` [verify current willow-rs impl status before committing]) and its epoch length is a tunable knob, so it can be dialed to whatever `W_target` the maintainer sets. Waiting for Track B buys near-immediate, generation-bounded revocation, but bets the DOI critical path on **in-flight Ink & Switch research (Keyhive) with no production deployment as of mid-2026** [verify before committing] and inherits the DMLS/FREEK storage-cost curve. The honest downside of the A-first recommendation: if E-A11.0 turns out to demand a window no feasible epoch length can hit at acceptable cost, the A work FALSIFIES — but that is a first-class result, reached faster and cheaper than discovering the same thing after betting the timeline on external research.

**Proposal (for review).** **Proceed as if Track A wins.** Run E-A11.A first, decide A the moment it clears the E-A11.0 bar, and treat Track B as a documented future upgrade rather than a gating dependency — because Keyhive (B) is unshippable today, E-A11.B is Rung-B-only, and Track A's epochs are tunable to the bar, so A is the only track that can actually unblock the DOI now. This is explicitly **the maintainer's call to accept or adjust**: it is a recommendation to sequence and bias toward A, not a decision that A has won — E-A11.A vs E-A11.0 still has to earn it, and a FALSIFIED there re-opens the choice.

---

## Track A2 + A12 — recovery-anchor prototype

The principles are fixed (§7.3.9; `../../beta/DECISIONS.md` A2+A12). These experiments work out the
**mechanism** — the five deferred parameters — and prove the two must-hold invariants and the must-reject
adversarial cases. They map onto and make executable the consolidated **Stage 8 (recovery ladder)** and
**Stage 9 (tenure / survivor-re-key)** properties.

**The three-case model (confirmed 2026-07-09 with the maintainer) — the organizing spine.** "Recovery" is
not one problem; it is three cases distinguished by *what was lost* and *what survives to bridge it*. Every
experiment below is placed against this spine.

- **Case 1 — lose a device, other clients remain.** This is **not recovery** at all: you **two-phase BAN the
  lost client** — the immediate local ignore + epoch roll of the **E-A11.0 revocation** (see Track A11 above;
  *reuse that mechanism, do not build a second one*). The persona survives unbroken through your other
  clients; nothing is reconstructed.
- **Case 2 — lose ALL devices, but the parent key was BACKED UP.** Recover the backed-up parent-key material
  → **rotate the lineage forward** → rejoin as the **same principal** (it is a lineage, not a new identity).
  This is the backup+unlock problem. **The backup target is pluggable** — one axis (*where the encrypted
  backup lives*), orthogonal to the unlock axis (*passphrase and/or quorum*, E-REC.1) and to the Case-3
  social path. Three targets, which compose freely (belt-and-suspenders, e.g. paper **and** PDS):
  - **QR / printed sheet, vaulted** — air-gapped self-custody. Immune to online/remote compromise; the risk
    is **physical** (loss, fire, being photographed), not offline-guessing. Classic seed-phrase pattern.
  - **File export** — an encrypted keystore file the user places (USB, drive, password manager); as safe as
    where it lands plus its own encryption.
  - **PDS encrypted-blob-vault** — the **provider-recovery analog** and the closest to the familiar "recover
    through your account" model: durable, always-available, survives total device loss with nothing physical
    kept. It is the already-recovered *encrypted-blob-vault* pattern
    (`../../beta/cairn/atproto-selfhosting-appviews-and-bridges.md` — client-encrypt before `uploadBlob`, the
    PDS/relays mirror unreadable bytes, the public atproto network as a **free durable distributed encrypted
    store**). The Croft twist that keeps it honest: the PDS stores **ciphertext it cannot read** — provider
    *UX* without provider-*holds-your-keys*.
  Three invariants govern whichever target holds the ciphertext:
  1. **Ciphertext-only, never a decryptor.** A store holds ciphertext only; the blob's key is user-held
     (recovery passphrase) and/or quorum-held, **never store-derivable** — the **meer-blind invariant applied
     to recovery storage**. Most load-bearing for the PDS target (a PDS that could decrypt is the readable
     homeserver Croft refuses, §2.8 / A12).
  2. **Offline-attack-resistant (for exposure-attackable targets).** The PDS blob is effectively **public**
     (atproto public-by-default) and a synced file may be too, so an attacker gets **unlimited offline
     guesses** — the encryption MUST be offline-attack-resistant (strong memory-hard KDF / quorum-gated). For
     the air-gapped QR/paper target the control is **physical security** instead, so the invariant is
     target-specific.
  3. **Portable.** The PDS vault travels via CAR export/import anchored to the **A9 stable logical URI** (a
     credible exit); file and paper are inherently portable.
- **Case 3 — parent key GONE, no recoverable backup, want to rejoin as the same persona.** No cryptographic
  link remains, so a new keypair **cannot** be a crypto extension of the old lineage. Only the **group can
  bridge it**: a **quorum vouches** that "persona X's lineage now roots at this new key." No secret is
  recovered — the group's say-so *is* the continuity. **Feasibility is tiered by social closeness:** a small,
  close circle can quorum-vouch; a large or anonymous group has no basis to, so there the person is simply a
  **new persona** (or continuity is *not possible*). This case is irreducibly social.

**The crypto split — this resolves E-REC.1's previously-open VSS-vs-FROST question. It is NOT either/or; the
two recovering cases want different primitives:**
- **Case 2 (unlock the backup) → threshold-DECRYPTION / VSS-of-the-blob-key.** Guardians hold shares of the
  blob-unlock key; k of them **reconstruct** it on the recovering device, verifiably so a cheating guardian's
  share is caught. This is secret *reconstruction*.
- **Case 3 (social re-establishment) → FROST-shaped threshold-AUTHORIZATION.** Guardians jointly **sign** the
  re-attestation fact; **no secret is ever assembled**. This is threshold *signing*.

### E-REC.0 — The Case-2 backup store (pluggable target: QR/paper · file · PDS-vault)

- **Type / Rung:** `needs-proving`. **Rung A** for each target on the reference implementation's own stack:
  the **PDS-vault** on the atproto/PDS integration (real `uploadBlob`/blob-fetch, client-side KDF + AEAD);
  the **file** target as an encrypted keystore export/import; the **QR/paper** target as encode-to-QR /
  printable-sheet + scan/re-key. Drystone's binding of the PDS-vault to the **A9 stable logical URI** is
  **Rung B** until that anchor lands.
- **Claim it tests:** that the Case-2 backup store supports the full **store → lose-all-devices → recover →
  unlock → rotate-lineage-forward → rejoin-as-same-principal** loop across a **pluggable target set** — QR/
  printed-sheet (air-gapped), file export (portable keystore), and the PDS encrypted-blob-vault (provider-
  style, blind) — with the targets composable, and holds the target-aware Case-2 invariants.
- **What it works out (fills §7.3.9 parameters).** That the recovery material is a **portable encrypted blob
  plus an unlock threshold**, **not a single monolithic "recovery secret"**; the **target set** {QR/paper,
  file, PDS} and which are default/offered; the KDF/AEAD choice **`[verify before committing]`** (a strong
  memory-hard KDF for the passphrase path; a **threshold-KEM / HPKE** for the quorum-gated unlock — couples
  E-REC.1); and, for the PDS target, the **CAR export/import + logical-URI anchoring** that makes it portable.
- **Pass/fail criterion.** **PASS** iff, for each target: the loop completes end-to-end (PDS target against a
  real PDS); the recovered material **rotates the lineage forward and rejoins as the same principal**; and the
  target-aware invariants hold. **Must-reject (any one = FAIL):** a store (esp. the PDS) that **can decrypt**
  the blob (ciphertext-only broken = the readable homeserver); an exposure-attackable target (PDS blob /
  synced file) whose **weak KDF is offline-crackable** (offline-resistance broken); a PDS-vault that **cannot
  be exported / re-imported across a PDS migration** (portability broken). The QR/paper target's control is
  physical, so it is scored on air-gap + physical-recovery, not offline-resistance.

### E-REC.1 — Quorum mechanism (threshold recovery)

- **Type / Rung:** `needs-proving`. **Rung A** on **two real threshold-crypto libraries — one per case** (a
  verifiable-secret-sharing crate for the Case-2 unlock *and* a threshold-signature crate for the Case-3
  vouch; the split is resolved below, no longer an open choice) **+ Rung A on `mls-rs 0.55.2`** for the MLS
  re-provision material. Each library must be real per the fidelity ladder, not a hand-rolled polynomial.
  Drystone's own governance-chain structures are **Rung B** until WS3 (redb) lands (couples
  `2026-07-09-engineering-validation-plan.md` WS3).
- **Claim it tests:** that a guardian quorum supports recovery — noting the quorum plays **two distinct roles
  depending on the case**:
  - **Case-2 unlock (threshold-DECRYPTION / VSS-of-the-blob-key).** k-of-n guardians hold shares of the
    E-REC.0 blob-unlock key (and/or the **`did:plc` rotation key (A10) + MLS re-provision material**) and
    **reconstruct** it on the recovering device — secret *reconstruction*, verifiable so a cheating
    guardian's share is caught. Reconstructed only by k concordant guardians, never k-1.
  - **Case-3 vouch (FROST-shaped threshold-AUTHORIZATION).** k-of-n guardians jointly **sign** the
    re-attestation "persona X's lineage now roots at this new key" — threshold *signing*, **no secret ever
    assembled**.
  In both roles the action fires only under a conditional-access trigger with a break-glass delay + contest
  window, and restores **exactly the lost principal's authority, never more**. (Consolidated Stage 8 Rung 2 +
  Group V1/V2.)
- **What it works out (fills §7.3.9 parameters).** The reconstruction protocol; **k and n** defaults; the
  **conditional-access trigger** (what fact fires recovery); the **break-glass delay** and **contest window**
  length (in epochs/generations, not wall-clock).
- **Pass/fail criterion.** **PASS** iff: k concordant guardian shares reconstruct exactly the lost principal's
  key material and **k-1 does not** (Stage 8 V1); the recovery decision **folds order-independently and
  converges** (Stage 8 V3 / Stage 4 group-H shape); reconstruction restores **exactly the lost principal's
  authority, never more** (Stage 8 U1 — the center test). **FALSIFIED** if any k-1 path reconstructs, or the
  fold is order-dependent.

**Tradeoffs.** What was previously framed as a *fork* — secret reconstruction (Shamir/VSS) **vs** threshold-signing (FROST/BLS) — is now **resolved by the three-case model into a split, not a choice**: Case 2 (unlock the backed-up blob, plus the `did:plc` rotation key + MLS re-provision material) is a secret-*reconstruction* problem that wants the reconstruction family, and Case 3 (socially re-establish a lost lineage) is a threshold-*authorization* problem that wants the signing family. A second axis, **verifiable vs not**, applies within the reconstruction family. Candidates, each **`[verify before committing]`** for maintenance/features/audit status:
  - **`vsss-rs`** — *verifiable* secret sharing. The **Case-2 unlock primitive**: it reconstructs the blob-unlock key / re-provision material and lets a guardian's bad share be *detected*, which is exactly the E-REC.5 contested/hijack surface. Downside: smaller ecosystem/maturity than the signing crates `[verify]`, and like any reconstruction scheme it re-materializes the plaintext secret at one place at recovery time — a momentary single point of compromise (acceptable for Case 2 because there *is* an existing secret to recover).
  - **`sharks`** — plain Shamir, no verifiability. Simpler and smaller, but a malicious dealer or guardian can submit a garbage/forged share undetected — it cannot prevent guardians from cheating, so it fails the adversarial-guardian requirement E-REC.5 raises. Rejected for either role.
  - **`frost-ed25519` (FROST)** — threshold *signing*. This is the **Case-3 primitive**: guardians jointly sign the re-attestation fact **without ever assembling a plaintext secret**, which is exactly the social-re-establishment shape (and the stronger no-single-point-of-compromise property, correct here precisely because Case 3 has *no* secret to reconstruct). It does not reconstruct the Case-2 blob-unlock key or MLS re-provision material — that is reconstruction, not signing — so it **complements** VSS rather than replacing it.
  - **BLS-threshold** — same threshold-signing shape as FROST plus a pairing-curve dependency; the pairing-curve alternative to FROST for the Case-3 signing role.
The fidelity ladder forbids a hand-rolled polynomial stand-in for either family — Rung A requires the real, vetted crates.

**Proposal (for review) — both, per case (the VSS-vs-FROST question is now RESOLVED as a split, not a choice).** Use **`vsss-rs` (verifiable secret sharing) `[verify before committing]`** for the **Case-2 unlock** — reconstruct the blob-unlock key / re-provision material, verifiability catching a cheating guardian (the E-REC.5 contested surface) — **and** **`frost-ed25519` (FROST) `[verify before committing]`** for the **Case-3 vouch** — threshold-sign the re-attestation, never assembling a secret. Plain `sharks` is rejected for either role (no verifiability); **BLS-threshold** is the pairing-curve alternative to FROST for the Case-3 signing role. The one-sentence why: the two recovering cases are structurally different problems — reconstructing an existing secret (Case 2) vs authorizing a social fact with no secret to reconstruct (Case 3) — so they take different primitives rather than one compromise crate. The crate choices remain **the maintainer's call to accept or adjust**, and current maintenance/audit status must be verified before either is pinned.

### E-REC.2 — Custodial-delegate mechanism, and how it composes with the quorum

- **Case placement (Case 2).** The custodial delegate is **one Case-2 custody vector** — an alternative or
  complement to the E-REC.0 PDS-vault and the E-REC.1 quorum for holding and conditionally releasing the
  backed-up unlock material. It is not a Case-3 mechanism (Case 3 is social vouch, not custody).
- **Type / Rung:** `needs-proving`. **Rung A** on `mls-rs` for the sealed re-provision material; **Rung B**
  for Drystone's governance-role structures until WS3.
- **Claim it tests:** the **conditional-access custodial role** — holds sealed recovery material, releases
  only under defined conditions — is a **capability, not authority** (Part 1 §2.7; §5.4), and that **delegate
  and quorum compose** cleanly (Stage 8 Rung 1 + U2/U3).
- **What it works out (fills §7.3.9 parameters).** How the release **conditions are defined and verified**
  (attributable, revocable read — Stage 8 U3: reading the secret must move **no** governance slot for the
  custodian); the **composition semantics** — delegate-as-guardian (delegate is one of the n)? both-required
  (delegate ∧ quorum)? either (delegate ∨ quorum)? — resolved as a **group-default composition rule** with the
  per-user override handled in E-REC.3.
- **Pass/fail criterion.** **PASS** iff: the custodian can surface material to re-establish the lost lineage
  but **holds no Group Role, cannot act as the principal, cannot govern** (Stage 8 U3); the delegate is itself
  **revocable and forkable** (Stage 8 U2); the chosen composition rule (∧ / ∨ / delegate-as-guardian) is
  stated and each mode folds convergently. **FALSIFIED** if a model where the custodian gains standing by
  holding the secret passes.

**Tradeoffs.** The composition rule trades **recovery UX against the §2.8 readable-homeserver risk**. Delegate-as-sole-path (delegate ∨ nothing) is the best UX — one custodian, trivial recovery, no coordinating a quorum — but it is exactly the §2.8 concern: a single conditional-access custodian is one compromise (or one coerced release) away from reconstituting standing read, a readable homeserver by another name. Quorum-only (k-of-n, no delegate) is the strongest structurally — no single party is ever a path — but it is the hardest UX, forcing every user to recruit and coordinate n guardians and stranding those who cannot. The **guardian-equivalent middle** (delegate counts as one of the n under conditional access, never a sole path) balances these: the delegate improves UX without ever being sufficient alone, so no single compromise yields access. Its honest downside is that it adds composition semantics to specify and prove, and the delegate remains a privileged, standing share — it concentrates *some* risk in one party even though it can never act alone.

**Proposal (for review).** Group-default composition = a **k-of-n social-recovery quorum**, with a designated custodial delegate (when one exists) counting as **one guardian-equivalent under conditional access — never a sole path** — and a **per-user override to tighten** (raise k, or drop the delegate entirely). The one-sentence why: this keeps recovery usable while structurally forbidding the single-custodian standing-read that A12 / §2.8 rejects, because the delegate can improve the odds of assembling a quorum but can never be sufficient by itself. This is **the maintainer's call to accept or adjust** — in particular whether the delegate composes as ∧ (delegate *and* quorum) for higher-assurance groups is a knob left open for override.

### E-REC.3 — Group-default + per-user designation

- **Case placement (Case 3).** The **Case-3 social vouch is per-group**: a group's recovery arrangement
  includes whether — and by what quorum — it will re-attest a member who has lost all key material with no
  backup. Its **feasibility follows social closeness** (per the three-case model): a small, close group can
  carry a per-user override designating close vouchers, while a large or anonymous group has no basis to vouch
  and its default is "no Case-3 continuity — rejoin as a new persona."
- **Type / Rung:** `needs-proving`. **Rung B** (model-form on the governance-fact fold — this is a fold-
  precedence property, not an MLS-mechanics property; name the stand-in). Re-run Rung A on the fold once WS3
  redb lands.
- **Claim it tests:** that a **group sets a default recovery arrangement** (default quorum/delegate/k/n/delay)
  **and a user overrides with a more-specific per-user designation**, and the more-specific one wins
  deterministically under the fold.
- **What it works out (fills §7.3.9 parameters).** The **group-default arrangement's shape** (the exact
  deferred parameter); the precedence rule that lets a per-user designation supersede the group default
  without a wall-clock (a governance fact at the user's own causal position, §7.3.1 causal precedence).
- **Pass/fail criterion.** **PASS** iff: a per-user designation deterministically overrides the group default
  everywhere (order-independent, §7.3.2 projection-not-mutation), and absence of a per-user designation falls
  back to the group default. **FALSIFIED** if two nodes fold the same facts to different effective recovery
  arrangements.

**Tradeoffs.** A prescriptive default is predictable and means no group is silently left with no recovery, but it may not fit every group's threat model; a fully open no-default is maximally flexible but leaves groups with no recovery until someone configures one — a silent no-recovery hole. The middle keeps a safe default while letting groups and users specialize.

**Proposal (for review).** Small groups **default to a quorum drawn from existing members**; a group **may designate a delegate** (composing per the E-REC.2 rule); a **user may always override per-user** with a more-specific arrangement that wins deterministically under the fold. The why: a members-drawn quorum is the safe, always-present floor, and the per-user override preserves individual agency without a wall-clock. **The maintainer's call to accept or adjust** — in particular the small-group size threshold and whether the default quorum's k/n is fixed or scales with membership.

### E-REC.4 — Meer-blind invariant proof (must-hold)

- **Type / Rung:** `needs-proving`. **Rung A on `mls-rs`** for the key-material path (this is the exact
  component the claim is about — a stand-in is forbidden here). See risks re: whether the negative
  ("never gains usable keys") is fully testable at Rung A.
- **Claim it tests:** the **meer never gains usable key material via the recovery path** (§5.4 invariant); the
  **recovery role is provably distinct** from the meer PeerSet; **and the PDS / backup store holds ciphertext
  only** — the **meer-blind invariant applied to recovery storage** (E-REC.0 invariant 1): *no* storage node,
  meer or PDS, can derive a decryptor.
- **What it verifies.** Through the real `mls-rs` sealing path: the meer forwards only sealed material (as in
  §7.4.2's out-of-band convergence); the recovery custodian's material is separately held and separately
  role-typed; there is **no path** by which meer + recovery custody collude to a usable key without satisfying
  the E-REC.1 quorum / E-REC.2 conditions.
- **What it verifies (recovery storage).** The E-REC.0 vault stores **ciphertext only**: the store node (PDS
  or meer-mirrored blob) holds bytes it cannot decrypt, the blob's key being user-held and/or quorum-held and
  **never store-derivable**. A store that can decrypt is the readable homeserver the invariant forbids — the
  same meer-blind boundary, applied to storage rather than transit.
- **Pass/fail criterion.** **PASS** iff the meer role, exercised through real MLS, is shown to hold **no
  usable key material** at any point in the recovery flow, and the recovery role is a distinct, revocable
  capability. This is a **must-hold** — a FALSIFIED here falsifies the A2/A12 principle set and stops the
  prototype.

**Tradeoffs.** Proving a negative ("the meer *never* gains usable keys") trades **strength of guarantee against constraint on the API**. A purely test-based approach can only ever *sample* the flow space — it can never fully prove a negative, so a green test suite here would over-claim. Enforcing the invariant **structurally at the type/capability level** — the meer role's API literally cannot name or hold a usable decryption key, it only ever receives sealed material — converts "prove a negative" into "show the type system/capability model forbids it," a compiler-checked, exhaustive guarantee. Its honest cost: it constrains API design (every recovery flow must be expressible as sealed-only through the meer type, which is more rigid and can complicate legitimate flows), and the guarantee is only as strong as the boundary — an escape hatch, an `unsafe`, or a serialization path that leaks the key silently breaks it, so the structural claim still has to be shown airtight.

**Proposal (for review).** Enforce the invariant **structurally at the type/capability level** (the meer role type only ever receives sealed material and cannot express a usable key), then back it with a **Rung-C adversarial pass** that attempts every recovery flow and must fail to yield the meer a usable key. This is the Rung-A *constructive* half plus a Rung-C *"no other path"* half the risks section already anticipates — it is the only framing that honestly discharges a negative claim, since a type-level prohibition is exhaustive where a test suite is not. **The maintainer's call to accept or adjust**, in particular how much the API is allowed to bend to make the sealed-only meer type ergonomic.

### E-REC.5 — Adversarial (must-reject)

- **Type / Rung:** `needs-proving` + adversarial analysis. **Rung A** where a real component is under attack
  (`mls-rs` re-provision; the threshold lib); **Rung B** for the governance-fold-shaped attacks until WS3.
- **Claim it tests:** the recovery mechanism **rejects** the following, all must-reject:
  - **The §2.8 / A12 concern (the headline).** Does the delegate/recovery role **quietly rebuild a readable
    homeserver** — a standing-read center by another name (Part 1 §2.8 faithful-representation capstone; A12
    open gate: *what structurally resists Option B quietly rebuilding a readable homeserver*)? **Conditional
    access + revocability + the meer-blind invariant (E-REC.4) must prevent standing read.** Must-reject: any
    configuration where the recovery role yields continuous readable access rather than conditional break-glass.
  - **Contested / hijack recovery** via the quorum: a break-glass **fired while the holder is in fact fine**
    must be **detectable, contestable within the delay window, and revocable or out-forkable** before it takes
    effect (Stage 8 V2). Concurrent rival recovery claims resolve by the §7.3.1 tiebreak + quorum, **never as
    rival holders** (Stage 8 V3).
  - **Recovery under partition:** a recovery quorum assembled in one partition must not silently diverge; a
    divergent per-partition recovery is the `RemovedThenIncluded`-class contradiction and **hard-stops /
    escalates** (§7.6), never silently merges.
  - **Case-3 impostor-vouch (new).** An attacker tricks or colludes a quorum into re-attesting a **fake
    continuation of persona X** — binding X's lineage to a key the attacker controls. Must-reject: the
    FROST-signed re-attestation (E-REC.1 Case-3 role) must require k concordant, correctly-authenticated
    guardians, be contestable within the delay window, and out-forkable; a quorum below k, a coerced/deceived
    single guardian, or a vouch that takes effect before the contest window closes is a FALSIFIED. (This is
    the social-attestation analogue of the contested/hijack quorum above — since Case 3 recovers *no* secret,
    the group's say-so is the *only* thing an attacker needs to subvert.)
  - **PDS public-ciphertext offline attack (new).** The E-REC.0 blob is effectively public (atproto
    public-by-default), so an attacker gets **unlimited offline guesses** against it. Must-reject: any vault
    whose encryption is not offline-attack-resistant — a weak or fast KDF on the passphrase path, or a
    non-quorum-gated blob key, that a well-resourced attacker can crack offline. (This is E-REC.0 invariant 2
    stated as an adversarial must-reject; a green E-REC.0 store loop does not by itself prove offline
    resistance — this vector does.)
  - **The resume-vs-fresh identity fork:** recovery must be "**re-plant or re-join fresh, converge history out
    of band**" and **never resurrect a live group's epoch secrets in place** (§7.4.2 residual — if any path
    does the latter, the replay/nonce-reuse hazard returns). **Couples the E12 re-plant set** (E12.1–E12.7,
    `12-replant-experiments.md`, T36): the recovery re-provision leg *is* an E12-style atomic re-plant, so
    E-REC.5 shares that harness and must be run against the same `mls-rs 0.55.2` re-plant path.
  - **Backstop always terminates:** even with no delegate and no quorum, **survivors fork with full history**
    (Stage 8 V4 / the §5.3 exit-and-fork floor). Must-accept (the one accept inside this reject set): the
    backstop never bricks.
- **Pass/fail criterion.** **PASS** iff every must-reject above is rejected and the backstop terminates.
  Any silent standing-read, silent partition-merge, in-place epoch-secret resurrection, or brick is a
  **FALSIFIED** that reshapes the mechanism.

### Recovery deliverable

- **The recovery mechanism spec that fills §7.3.9's pending parameters — now as a THREE-CASE structure**
  (rather than one monolithic "recovery secret"):
  - **Case 1 — the revocation two-phase.** Not recovery: reuse the E-A11.0 two-phase BAN (immediate local
    ignore + epoch roll). No new mechanism.
  - **Case 2 — the encrypted-blob-vault + unlock threshold.** The PDS encrypted-blob-vault (E-REC.0) plus the
    Case-2 unlock threshold (E-REC.1 VSS-of-the-blob-key, and/or the E-REC.2 custodial delegate). **There may
    be no single monolithic "recovery secret" — it is a ciphertext blob plus a threshold**, with the three
    invariants (ciphertext-only, offline-resistant, portable).
  - **Case 3 — a social-attestation threshold + its closeness-tiered feasibility.** The FROST-shaped
    threshold-authorization (E-REC.1 Case-3 role), configured per-group (E-REC.3), with feasibility tiered by
    social closeness (small close circle can vouch; large/anonymous group cannot → new persona).
  - **The deferred parameters, per case:** **k, n, break-glass delay, contest window, the exact access
    conditions, the quorum/delegate composition rule, the group-default shape, and the recovery-secret
    encoding** (Case-2: blob + unlock threshold; Case-3: the re-attestation fact and its quorum).
- **A working re-provision-after-total-loss prototype** (Rung A on `mls-rs` re-provision + the threshold lib;
  Rung B on Drystone's own structures until WS3), demonstrating total-device-loss → recover → rejoin with
  tenure preserved (couples Stage 9 Group R — no stranded right — and T22).
- Moves §7.3.9 from `Design (decided direction; mechanism pending)` → `Verified`, and discharges the A2 and
  A12 open gates in `../../beta/DECISIONS.md` Section 2.

---

## Dependencies & sequencing

```
 A11 SPIKE (publish-road critical path):

   E-A11.0 threat model (Rung C) ──► E-A11.A Meadowcap (Rung A/B) ─┐
   (fixes W_target + cost ceiling)   E-A11.B Keyhive (Rung B)   ───┴─► DECISION MEMO
                                                                       │
                                                                       ▼
                                             capability wire format ──► §7.2 buildable ──► v0.1 DOI mintable
                                             (also shapes T40 attestation credential, WS4)

 RECOVERY PROTOTYPE (prove-road; raises maturity toward rc):

   three cases: (1) lose a device = reuse E-A11.0 two-phase BAN — not recovery
                (2) lose all, backup exists = blob-vault + unlock threshold
                (3) key gone, no backup = social vouch, closeness-tiered

   uses A10 rotation-key custody ─┐
   uses identity-provenance chain ┤
   WS3 redb fold engine (Rung A) ─┤ (Rung B until it lands)
                                  ▼
   E-REC.0 PDS encrypted-blob-vault (Case-2 store) ─┐
   E-REC.1 quorum: Case-2 unlock (VSS) + Case-3 vouch (FROST) ─┤
   E-REC.2 delegate + composition (Case-2 custody vector) ─────┤─► E-REC.3 group-default + per-user
                                                               │       (Case-3 vouch per-group)
   E-REC.4 meer-blind + ciphertext-only store (must-hold, gates the rest) ──► E-REC.5 adversarial (must-reject)
                                                       └─ couples E12 re-plant set (T36); adds Case-3
                                                          impostor-vouch + PDS offline-attack vectors
                                  ▼
   recovery spec = 3-case structure (blob+threshold, social threshold) + re-provision prototype ──► §7.3.9 filled → Verified
```

- **E-A11.0 before A/B** — the threat model is the ruler; A and B are scored against it.
- **Recovery prototype uses the A10 rotation-key custody** (the `did:plc` rotation key folds into recovery
  custody, §7.3.9) **and the identity-provenance chain** (`cairn/cross-platform-identity-provenance.md`).
- **A11's outcome shapes T40's attestation credential (WS4):** the capability track gates the credential
  shape *and* the capability wire format (`2026-07-09-engineering-validation-plan.md` WS4; T40).
- **E-REC.4 (meer-blind) gates E-REC.5** — no point running adversarial cases until the must-hold invariant
  is proven; a FALSIFIED there stops the prototype.
- **E-REC.5 couples the E12 re-plant set** (T36) — the recovery re-provision leg is an E12-style atomic
  re-plant, shares that harness and `mls-rs 0.55.2`.
- **WS3 (redb) coupling:** E-REC.1/.2/.3 run Rung B on Drystone's own governance-chain structures until WS3
  lands, then re-run Rung A on the fold (per the fidelity ladder's stand-in-generates-follow-up rule).

**Critical-path sketch:**

```
   E-A11.0 → E-A11.A/B → A11 DECIDED → capability wire format → §7.2 buildable → DOI mintable
   E-REC.1..5 → recovery mechanism spec → §7.3.9 filled → §7.3.9 Verified
```

---

## Reasoning — why this sequence, what each proof buys

- **A11 is on the DOI critical path; recovery is not.** Two roads (`2026-07-09-engineering-validation-plan.md`):
  the **publish road** gates on §7.2 being buildable from text alone, whose **last `ENABLING` encoding is the
  capability wire format**, which cannot be pinned until A11 is decided. So the A11 spike is the single highest-
  leverage unblock for the DOI. The recovery prototype rides the **prove road** — it raises maturity toward
  rc and discharges the largest residual protocol risk, but the DOI does not wait on it.
- **The A11 decision logic is asymmetric on purpose.** Appendix A already establishes that both tracks satisfy
  R1/R2/R3/R6 identically — *only revocation immediacy differs*. Track A ships today (Meadowcap Data Model +
  capability layer are `Final`); Track B waits on in-flight Ink & Switch research and carries the DMLS/FREEK
  storage-cost curve. So the cheapest path to the DOI is: **prove A is good enough (E-A11.A vs E-A11.0), and
  only fall to B if it is not.** Investing in B first would put the DOI behind external research it may not
  need. E-A11.0 exists precisely because Appendix A says the deferral stands *until a needs-definition exists*
  — this experiment set is that definition.
- **The recovery prototype turns a known pattern into a specified one.** §7.3.9 and Stage 8 both note recovery
  is a **known pattern (social + threshold recovery), not open research** — so the work is mechanism
  specification and adversarial hardening, not invention. E-REC.4 (meer-blind) and E-REC.5 (the §2.8 "readable
  homeserver" rejection) are what buy the trust: they prove the recovery role is a **conditional capability,
  not a smuggled center** — the exact objection the A12 gate raises.
- **What is on the DOI critical path vs parallel.** Critical path: E-A11.0 → E-A11.A/B → decision memo →
  capability wire format. Parallel (prove road): the entire E-REC set, which can proceed as soon as its inputs
  (A10 custody, identity-provenance chain, and WS3/WS1 harness) are available, independent of the DOI.

---

## Open questions / risks

- **Keyhive un-shippability (Track B maturity risk).** Track B rests on in-flight Ink & Switch research with
  no production deployment as of mid-2026 (`object-capability-and-decentralized-mls-prior-art.md`). E-A11.B is
  therefore expected to be **Rung B / paper-analysis only** — no Rung-A verdict is reachable, and picking B
  means accepting an external-research timeline on the DOI critical path. This is the strongest reason the
  decision rule is biased toward proving A adequate first.
- **Threshold-crypto library choice (E-REC.1).** The library is undecided. The fidelity ladder forbids a
  hand-rolled stand-in for the exact component under test, so a **real, vetted** threshold/Shamir/VSS crate
  must be selected before E-REC.1 can claim Rung A — this is a prerequisite decision, flagged here, not made
  in this plan. Candidate selection should verify maintenance status and audit history before adoption.
- **Is the meer-blind proof testable at Rung A (E-REC.4)?** The claim is a **negative** ("the meer *never*
  gains usable keys"). Rung A can exercise the real `mls-rs` sealing path and show the meer handles only
  sealed bytes, but a negative is proven by exhaustion/adversarial coverage, not by a single green run. The
  honest bar may be Rung A for the constructive half + an adversarial-analysis (Rung C reasoning) for the
  "no other path" half; if so, label it that way rather than over-claiming a Rung-A negative.
- **Meadowcap real-lib reachability (E-A11.A).** Meadowcap's Data Model + capability layer are `Final`, but
  the Rust impl is pre-1.0 and Confidential Sync / Willow'25 took breaking changes into 2026
  (`willow-meadowcap.md`). The **epoch-key revocation** path specifically may not be exercisable at Rung A; if
  not, E-A11.A's window measurement drops to Rung B (model-form) and the cost half stays Rung A on `mls-rs`.
- **`W_target` is a judgment call, not a measurement (E-A11.0).** The acceptable revocation window against the
  moderation/ban-evasion threat model is a threat-model decision (the spec declines to pick tolerance defaults,
  §7.4.1). E-A11.0 must state it as a defensible bar with reasoning, and the A11 decision inherits whatever
  conservatism that bar encodes — surface it as a decision, not smuggle it as a constant.
- **Cost coupling to unproven §11.11 measurements.** E-A11.A's short-epoch cost half depends on the WS1
  §11.10.1 A–G matrix, whose per-commit / fan-out numbers are themselves `Load-bearing, unearned`. E-A11.A
  should reuse that rig but its cost verdict is only as firm as the matrix underneath it — note the dependency
  rather than treating the cost numbers as settled.
- **WS3 timing.** E-REC.1/.2/.3 are Rung B on Drystone's own structures until the redb fold engine (WS3)
  lands; a slip in WS3 delays the Rung-A re-runs (not the Rung-B first passes).
