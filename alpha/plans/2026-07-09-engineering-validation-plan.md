# Engineering & validation forward-road plan (the "build-and-prove" open threads)

date: 2026-07-09 · author-read source: `../../beta/OPEN-THREADS.md` (T5/T20, T2, T38, T36, T22,
T29, T25, T40, T10, T6); `../../beta/impl/README.md`; `../../beta/impl/delivery-layer/08-experiment-methodology.md`;
`../../beta/drystone-spec/part-2-certifiable-design.md` §11 + Appendix B.

> **What this is.** A PLAN — options, dependencies, and sequencing for the Croft/Drystone engineering
> and validation work, on the path to publication-final (T30 → the Zenodo DOI). It plans the work; it
> does not do it, and it edits no thread or spec doc. Thread numbers and spec sections are cited for
> traceability; beta docs are referenced by path. Beta-tier discipline does not bind an `alpha/plans`
> doc, but the epistemic vocabulary below is the spec's own (§1.1 status ladder;
> `08-experiment-methodology.md` fidelity ladder) so plan and corpus speak the same language.

---

## Problem statement

The beta corpus is mature on **design** and thin on **proof**. The Drystone spec (Part 1 + Part 2)
reads as a settled synthesis, but the load-bearing behavior underneath it is still tagged `Design`,
`[confirm]`, or `Load-bearing, unearned` (§1.1 status ladder) — not `Verified`. Concretely:

- Part 2 carries a **single load-bearing beam** (completeness-ahead / gap-completeness, Appendix B),
  which most of the §7 governance-fold `Design` claims lean on and which no experiment has discharged.
- Two §11 large-group measurements are explicitly `Load-bearing, unearned` (§11.11): per-commit /
  fan-out cost at hot-N, and return-backfill time vs dormancy gap. The §11.10.1 experiment matrix
  (Experiments A–G) that would size them is specified but **not run**.
- The re-plant / atomic-swap mechanism was folded into §7.6.4 as `design; needs verification`
  (`[confirm before publish]` throughout) — the composed operation has never been exercised on a real
  stack (T36).
- Several §5 rights claims (tenure as absolute, §5.3) carry verify-before-hardening checks that have
  not been run (T22).
- The MLS-epoch ↔ governance-log binding under partition/re-key is `not specified` (T29) — the spec's
  own "the exact problem Matrix is still solving."
- The experimental public-by-default regime (§11.9.3) rests on an unsolved crypto sub-item (T40, the
  non-member-verifiable attestation, `Load-bearing, unearned`).

The forward road is to **build the reference implementation + a shared test harness and turn these
claims into proofs** at the right fidelity rung, so the spec can mature from beta to publication-final
and mint the defensive-publication DOI (T30). The methodology already exists
(`08-experiment-methodology.md`: Rung A real-lib / Rung B model-form / Rung C static) — this plan
sequences the work against it.

**A distinction that shapes everything below.** Two roads run from here, and they are not the same road:

```
        ┌─────────────────────────────────────────────────────────────┐
        │  PUBLISH road (T30/DOI)   — "enabling disclosure"             │
        │  Goal: §7.2 buildable from text alone + wire encodings pinned │
        │        + external [confirm] facts verified + T32 legal        │
        │  Does NOT require the behavior to be Verified.                │
        └─────────────────────────────────────────────────────────────┘
        ┌─────────────────────────────────────────────────────────────┐
        │  PROVE road (WS1–WS4)     — "earn the beam"                   │
        │  Goal: Design / [confirm] / Load-bearing-unearned → Verified  │
        │  Rides on the shared harness. Raises maturity toward rc.      │
        └─────────────────────────────────────────────────────────────┘
```

The publish road is what the DOI literally gates on (spec README: *do not mint the v0.1 DOI until §7.2
is buildable from the text alone*). The prove road is what makes the disclosed design trustworthy and
takes it past rc. They can run **in parallel**; conflating them would either delay the protective DOI
behind experiments it does not need, or ship a "Verified" claim the text cannot yet back.

---

## Approach — workstreams

Five workstreams, grouped so dependencies are explicit. WS1 (the shared harness) is the highest-leverage
first build because WS2 and parts of WS4 are proven *on* it.

### WS1 — Scale & conflict test harness  (T5 + T2 + T20 reconcile corpus + T38 matrix)

The shared harness most other proofs ride on. It is the single build that most raises the corpus's
evidentiary floor, so it is sequenced first.

**Deliverable.** One conformance + performance + adversarial harness with three surfaces:
1. **MLS/transport testbed** — OpenMLS/`mls-rs` on aarch64 + a real `iroh` / `iroh-gossip` gossip
   testbed (T38 §11.10.1 names exactly this rig).
2. **Scale/performance matrix** — Experiments A–G from §11.10.1 (fixed policy, sweep points, pass/fail
   thresholds), driving the two `Load-bearing, unearned` §11.11 measurements: per-commit + fan-out cost
   at hot-N = 500/1000/2000 (and the attesting-N 5k/10k/20k extension for the T40 public regime), and
   return-backfill time vs dormancy-gap size.
3. **Reconcile-semantics conformance corpus** — the C4/C7/C8/C9/C10 cases folded from T20 into T5
   (`../../beta/OPEN-THREADS.md` T5; `../alpha/thinking/merge-split-corpus.md` §4): add-vs-add on
   different device keys (C4), dissolve-vs-continue (C7, resolution *undefined* — must be defined
   first), diamond-recombine conflict (C8, extend `detect` to multi-parent ancestry), equivocation
   hardening (C9), ban-evasion re-add via a new device leaf (C10). Plus the T5 **synthetic high-churn /
   multi-partition test** (the churn-fold Achilles heel, still unactioned).
4. **Governance-model scenarios (T2)** — hypothesize the 2–3 large-scale governance models
   (liquid-delegate-vote / elected-admin-Reddit-style / broadcast-only) and play them out on the same
   harness across variety × quantity × peer count, up to the ~200k breakpoint.

**Proof/validation bar.** Rung A for MLS mechanics (`mls-rs`) and transport (`iroh`/`iroh-gossip`) —
versions pinned and printed per the hard rules. **Rung B** for Drystone's own governance-chain and
dataplane hash structures (not yet built), each stand-in named per the ladder. Success = the two
§11.11 measurements move from reasoned envelope to `Measured`; the reconcile cases move from
undefined/partial to conformance-covered; the churn/partition test either holds or `FALSIFIED` (a
first-class result) reshapes the fold.

**Dependencies (what must exist first).** The reconcile corpus (C4–C10) and the governance-fold scale
runs exercise the fold engine, so they couple **WS3 (T25 storage substrate)** — the Rung-B governance
structures live there. The MLS/perf matrix (A–G) is largely standalone and can start immediately.

**#1 gates.** **A11** (Track A Meadowcap vs Track B Keyhive) shapes the capability-wire portions of the
conformance suite — the revocation model differs by track — so capability-conformance cases wait on
A11. C10 ban-evasion couples the moderation surface (T31/T3), design-level not gate-level. The
survivor-determinism and pure-P2P-vs-superpeer-ordering honesty calls (T5) are internal design
decisions, not #1 gates, but must be made before their cases can assert a pass/fail.

### WS2 — Governance-fold proofs  (T36 re-plant/E12, T22 survivor-rekey↔tenure, T29 MLS↔log binding)

Three specific proofs, each with must-reject / must-accept vectors, all riding on WS1's harness and
WS3's fold engine.

**T36 — re-plant / atomic-swap (§7.6.4).**
- *Deliverable:* run the **E12 set (E12.1–E12.7)** to move §7.6.4 from `design; needs verification` to
  green-real. Resolve the two `mls-rs` library questions (does it expose ReInit first-class emitting
  the resumption PSK, vs fresh-create + manual PSK; does it surface resolution/blank counts directly vs
  a byte-size proxy) and the Appendix-B re-plant items (intent ordering before the ReInit freeze;
  seating default Welcome vs external-commit; PSK cross-group linking; `epoch_authenticator` overlap).
- *Bar:* Rung A on `mls-rs 0.55.2` for MLS mechanics; Rung B for Drystone's governance-chain +
  dataplane hash structures (so E12.7 is modeled until WS3 lands). Must-accept: unilateral O(N)
  instantiation with KeyPackage-per-member + last-resort floor; group-wide PCS on fresh stamp.
  Must-reject: planter byte-nondeterminism read as a fork (it is a dedup).

**T22 — does survivor re-key strand `tenure`? (§5.3).**
- *Deliverable:* a protocol-level check of the `04` survivor / re-key path against the absolute-tenure
  claim. Must-accept: a lawful peer can rejoin a re-keyed group. Must-reject/expose: any path that
  strands a peer — if one exists, specify its **bound** and write the precise caveat back into §5.3
  (currently over-claims tenure as absolute).
- *Bar:* Rung A on the real re-key path. Outcome gates hardening the §5.3 four-rights closed set.

**T29 — MLS group state ↔ governance-log/Automerge consistency.**
- *Deliverable:* **specify** the membership-fact → MLS-commit binding and its behavior at
  fork/partition/survivor-re-key (today unspecified; the §7.5 frontier-closure sibling). Design against
  or adopt the **DMLS/FREEK** mechanism (the puncturable-PRF FS cost that scales with retention window
  × group size × fork frequency — Drystone's "forks self-heal" is *not free*). Confirm the
  Matrix-in-federation comparison `[confirm before publish]`.
- *Bar:* spec-first (a new Part 2 § or an Appendix-B `ENABLING` item), then Rung A/B validation of the
  binding on the harness once specified. The redb `CONVERGENCE_FINDING` (canonical `(created_at,
  created_by)` MIN, not first-writer-wins) is the storage-layer twin already found — fold its rule in.

**Dependencies.** All three ride on **WS1** (harness) and **WS3** (fold engine for the Rung-B
governance structures). T29 must be *specified* before T22/T36 can be fully closed against it (the
binding is what re-key correctness is judged by).

**#1 gates.** **A11** shapes T29 (the capability/revocation wire the binding carries) and, transitively,
T22 (revocation = the survivor-strand risk surface). **A12** (key-custody: blind-relay Option A vs
revocable-delegate Option B) shapes the re-provision leg of T36/T22 (who can re-seat a device). **A2**
(recovery anchor) touches the strand case in T22 (total-device-loss re-entry).

### WS3 — Storage substrate  (T25 redb build/proof)

The local authoritative-vs-derived engine under the fold — foundational for WS1's reconcile corpus and
WS2's governance-fold proofs.

**Deliverable.** The local **derived-state engine**: authoritative signed-assertion store + governance
log + a rebuildable redb projection (graph adjacency index + declarative snapshot) behind a typed
query/command/notification surface, with crypto/MLS/credentials/blob I/O as **injected traits**
(testable in isolation). Concrete contract at
`../../beta/impl/delivery-layer/drystone-design/redb-storage-contract.md`.

**Status.** Already **`in-progress`** — being built externally from
`../alpha/seeds/generated-prompts/redb-social-graph-layer-build-prompt.md`.

**Proof/validation bar.** The build *is* the proof (Rung B for the fold logic): property tests for
order-insensitive convergence / rebuildability / authoritative-vs-derived consistency; mutation testing
on fold + validation; adversarial / fork / partial-knowledge / compaction / scale. Where "vetted" is
won or lost: the property-test **generators** (diverse/forked/partial) and the **mutation-survivor
list**. Standing risk to state plainly: redb has **no published Jepsen/linearizability crash-safety
evidence** (per the storage contract) — carry it as a caveat, do not claim it away.

**Dependencies.** None blocking (underway). Couples T1 (the protocol the fold validates against). The
edge-table representation (composite-key vs multimap) is an explicit build-time measurement.

**#1 gates.** None block *starting* (it is building). **A11** may shape the query/command surface (what
capability facts the projection must index), so the capability-facing tables should be left adaptable
until A11 lands.

### WS4 — Experimental & edge  (T40 public regime + non-member attestation, T10 media hardening)

The most speculative workstream and the one most gate-blocked.

**T40 — experimental public-by-default regime (§11.9.3) + its blocking crypto sub-item.**
- *Deliverable:* prototype and stress the regime (messages public by default above ~7k; MLS retained
  for attestation + membership, not payload encryption). **Blocking sub-item (was T39):** solve the
  **non-member-verifiable membership attestation** — let a non-member reader verify "attested member at
  standing X authored this," from a forwarded artifact, without trusting the bridge and without being
  an MLS member. Likely reduces to signing authored items with a credential chain verifiable against
  the group's published membership/governance state (a sketch, not a solution). Prototype the MLS-aware
  relay bridge → AppView-shaped read view (§11.9.3.1); confirm the read path stays in the "helper cannot
  lie, only stall" box (content-addressed, governance-positioned, gap-detectable omission).
- *Bar:* the attestation is **`needs-proving` crypto** — Rung A (real credential chain, real `mls-rs`
  identity path) plus an adversarial analysis of attestation-extraction. Success moves §11.9.3 from
  Design-experimental toward Design. The §11.9.3.3 attesting-core ceiling measurements are the
  attesting-N extension of the **WS1** matrix.
- *Depends on:* WS1 (attesting-N measurements) and the WS2/T29 credential + lineage interlock.

**T10 — real-time media-layer hardening.**
- *Deliverable:* close the residual `[OPEN]` — is str0m's strong/weak boundary precisely tested (it is
  production-grade server-side, weak on P2P ICE which Croft routes around). Feeds **TC-ENG0** (engine
  API audit) and **TC-INT3** (the A1-vs-A2 engine/transport decision — note: this "A1/A2" is the
  *media-engine* pair TC-INT3, distinct from the #1 decision gates A1/A2).
- *Bar:* Rung A on str0m with real frames (04 currently has media only `characterized`, E12 green-real
  on synthetic frames). Largely de-risked — a "close the last decisions" thread.
- *Depends on:* TC-ENG0 done, TC-INT3 decided.

**#1 gates.** **A11 hard-blocks T40**: the capability track gates the attestation credential shape *and*
the capability wire format that gates the DOI. **A2** (recovery anchor) shapes the resume-vs-fresh
identity fork inside the attestation. T10 is not #1-gated.

### WS5 — Cross-platform  (T6 per-platform trust-model doc)

Couples the S4 per-platform design files; largely `needs-content` + fact-confirmation, not a harness build.

**Deliverable.** The per-network trust-model write-up (Bluesky / ActivityPub / Mastodon / GoToSocial /
Threads / Hive): the field used, what Croft claims and does not claim, the backlink mechanism, exact
verifier steps + pseudocode. 05 names it the "highest-leverage next artifact" but cannot assert it
because it does not exist. Home: `../alpha/thinking/app/platforms/` (S4 scaffolds).

**Proof/validation bar.** Rung A fact-confirmation against real atproto for the load-bearing facts:
`alsoKnownAs` extra-entry persistence (`[UNVERIFIED]`, E14); the anchor-URI stability contract (A9).
Otherwise `needs-content`.

**Dependencies.** Partly on **T7** (atproto Permissioned/Private-Data external watch-item — nothing to
action until the WG settles its E2EE/ZK posture; `did:webvh` native support to be confirmed against the
FACTCHECK SoT).

**#1 gates.** **A10** (PDS-held vs self-controlled `did:plc` rotation key) determines what each spoke
can claim and whether `did:webvh` genuinely functions as a recovery anchor — so it shapes the doc's
per-spoke claims. **A2** relates (A10 is the Bluesky-spoke face of the recovery-anchor question).

---

## Reasoning — why this sequence

- **WS1 first, because everything rides on it.** WS2's three proofs and T40's attesting-N
  measurements are *run on* the harness. Building it once, at Rung A for the real libraries and Rung B
  for the not-yet-built Drystone structures, is the single highest-leverage move: it converts the two
  §11.11 `Load-bearing, unearned` measurements to `Measured` and gives every downstream proof a place
  to run. The methodology (`08-experiment-methodology.md`) is already binding, so WS1 is a build, not a
  design task.
- **WS3 is a near-peer of WS1 and already moving.** The reconcile corpus (C4–C10) and the
  governance-fold proofs need the fold engine, so WS3 is on the critical path *for the prove road* even
  though it is not gated. Its Rung-B property/mutation results are what let the single load-bearing beam
  (completeness-ahead) start to be earned rather than asserted.
- **WS2 is the payoff, but it is gated by WS1+WS3 and by T29-being-specified.** T29 must be written
  before T22/T36 can be fully closed against it — the binding is the yardstick for re-key correctness.
  So within WS2: spec T29 → run T36/E12 and T22 on the harness.
- **The DOI (T30) is on a *different* critical path than the proofs.** Publication-final = *enabling
  disclosure* (§7.2 buildable from the text alone), which needs the **wire encodings pinned in
  sequence** (canonical governance-fact byte encoding → frontier-closure-before-sort → frontier-
  commitment + acceptance-record → §7.2 → **capability wire format, gated on A11**), the
  `[confirm before publish]` external-fact sweep, and **T32** (attorney patent-non-assertion review).
  It does **not** require WS1–WS4 to have turned Design into Verified. So the publish road can proceed
  in parallel with the prove road — the harness raises maturity toward rc, it does not gate the
  protective DOI.
- **What can start now, unblocked:** WS1's MLS/perf matrix (A–G); WS3 (underway); T29 spec-writing;
  T10 media hardening; the non-capability wire encodings on the publish road; WS5 content (minus the
  A10-shaped per-spoke claims).
- **What waits on a #1 decision:** anything capability-shaped (A11) — the capability wire format (and
  thus the DOI's last encoding), T40's attestation, and the capability-conformance cases in WS1/WS2.
  See the decision-gated section.

---

## Sequencing / critical path

```
NOW ──────────────────────────────────────────────────────────────────────► PUBLICATION-FINAL (T30/DOI)

 PROVE road (raises maturity toward rc):

   WS3 redb substrate ─┐        (in-progress)
                       ├──► WS1 reconcile corpus (C4–C10) + churn/partition ──┐
   WS1 MLS/perf matrix ┘        (A–G, §11.10.1) ── §11.11 → Measured           │
                                                                              ├──► WS2 proofs:
   T29 binding SPEC ───────────────────────────────────────────────────────┘     T36/E12 (§7.6.4 green-real)
                                                                                   T22 tenure↔re-key (§5.3)
   WS1 attesting-N (5k–20k) ──► WS4 T40 attestation prototype  [needs A11]
   T10 media (TC-ENG0 → TC-INT3 → str0m ICE)  ── parallel, ungated

 PUBLISH road (the DOI's own critical path):

   pin wire encodings in sequence ──► §7.2 buildable from text alone ──┐
   (gov-fact byte enc → frontier-closure-before-sort → frontier-       │
    commitment+acceptance → §7.2 → CAPABILITY WIRE FORMAT [needs A11]) ├──► DOI mintable
   [confirm before publish] external-fact sweep ──────────────────────┤     (+ vehicle: Zenodo+OTS,
   T32 attorney patent-non-assertion review ──────────────────────────┘      spec text CC0, code AGPL-3.0)
```

**The critical-path item is the A11 capability-track decision → the capability wire format.** It is the
last domino on the publish road (the final `ENABLING` encoding that makes §7.2 buildable, hence the DOI
mintable) and it simultaneously unblocks T40's attestation and the capability-conformance cases on the
prove road. Everything else can advance without it; the DOI cannot.

---

## Decision-gated

Workstreams (or sub-items) that cannot start or cannot finish until a #1 gate (`../alpha/ROADMAP_TODO.md`
A-series; `../../beta/README.md` "Standing decisions surfaced") is decided:

| Gate | What it is | Blocks / shapes |
|---|---|---|
| **A11** — capability mechanism: Track A (Meadowcap) vs Track B (Keyhive) | revocation-immediacy call; §X.5 deliberately UNCOMMITTED | **Blocks** the capability wire format → **the DOI's last encoding (T30)**; **blocks** T40's non-member attestation (WS4); shapes the capability-conformance cases in WS1 and the T29 binding (WS2). *The critical-path gate.* |
| **A12** — key-custody default: blind-relay (Option A) vs revocable trusted delegate (Option B) | who can re-provision a device | Shapes the re-provision leg of T36/T22 (WS2) and the recovery UX; the §5.8/§2.8 exitability requirement is the partial structural answer. |
| **A2** — total-device-loss recovery anchor | mnemonic seed vs social-recovery quorum vs broker-held backup | Shapes the strand case in T22 (WS2) and the resume-vs-fresh identity fork in T40's attestation (WS4). Largest residual risk; none chosen. |
| **A10** — PDS-held vs self-controlled `did:plc` rotation key | who can issue future ops; whether `did:webvh` is a real recovery anchor | Shapes WS5's per-spoke claims (T6); relates A2. |
| **A1** — MPL-2.0 substrate license gate | `hpke-rs` mandatory for RFC 9420 HPKE; compliance call, not code | Does not block a build, but the substrate license posture must be settled before publication-final ships alongside the CC0 spec text + AGPL-3.0 reference code (A14/C13, already decided). |

Not #1-gated but internal design decisions that must be made before their cases can assert pass/fail:
survivor-selection determinism and pure-P2P-vs-superpeer ordering honesty (WS1/T5); the T2 delegation
model and concentration-resistance levers; the T29 binding shape (spec-writing); TC-INT3 (WS4/T10).
