# Experiments ↔ Part 2 overlay — proven / not-proven / pending / missing

`Status: analysis, 2026-07-13. The companion to SPEC-ALIGNMENT-AND-ACTION-PLAN.md, sharpened to
answer four questions directly: what is NOT proven out, what is EXPLICITLY pending, what the
experiments taught us that forces adapting Part 2, and what is MISSING from the experiment corpus.
"Part 2" = beta/drystone-spec/part-2-certifiable-design.md. Proposed spec edits are staged separately
in beta/drystone-spec/proposed-changes-2026-07-experiment-reconciliation.md (items F1–F7).`

The overlay reads Part 2 through the code spikes. Four buckets, deliberately distinct:

- **§1 Not proven out** — a green exists but it rests on a **stand-in, a proxy, a model, or a
  restricted environment**, so the spec claim it appears to support is *not actually* established.
  These are over-read risks.
- **§2 Explicitly pending** — work that is **named, scoped, and scheduled** as open (backlog items,
  `[confirm]`s, `Load-bearing, unearned` beams). Honestly flagged, just not done.
- **§3 What forces adapting Part 2** — things the experiments **taught** us that the spec text does
  not yet reflect (a missing mechanism, an over-claim to tighten, a shape that surprised us).
- **§4 Missing from the experiment corpus** — spec mechanisms with **no spike at all**; the coverage
  holes.

The dividing line between §1 and §2: *not-proven* is about the gap between a green and what it really
shows (a hygiene problem); *pending* is about work everyone already agrees is undone (a scheduling
problem). Both matter; conflating them is how a corpus quietly overstates itself.

---

## 1. What is NOT proven out (greens that rest on a stand-in / model / restricted env)

Each row: the spec claim, the green that appears to support it, and the reason it falls short. Tags
map to `SPEC-DIVERGENCE-REGISTER.md`.

| Part 2 claim | The green | Why it does **not** yet prove the claim | Tag |
|---|---|---|---|
| §8.2(a) freshness / convergence over **live transport** | 2-/4-node convergence to identical fingerprints over real iroh-gossip | Runs on **loopback, no relay**. The real deployment path is **n0 relay + holepunch**; that path is X1 and cannot exist where Internet UDP is blocked. Green ≠ relay path proven. | `hermetic-gossip` (active) |
| §6.8.1 **RBSR** (range-based set reconciliation — diff-only convergence) | Late-joiner catch-up over real gossip (X2 catch-up all-green) | The code re-broadcasts the **whole retained log** on `NeighborUp`, a coarse **push** cousin of RBSR — not the diff-only **range** reconciliation §6.8.1 specifies. Direction corroborated; the efficient mechanism is not implemented. Also: **steady-state anti-entropy** (loss to an existing neighbor, no new join) is untested. | (reconciled `x2-backfill`, residual noted) |
| §7.7 / §7.9 late-joiner **partial-reconstruction inertness** on the ship target | 4 scenarios PASS (a node given later-epoch changes with deps withheld holds them inert) | Proven on **Automerge 0.6.1**, not the **0.7** ship target (MSRV wall at import time). 0.7 confirmation is open (now runnable — Rust 1.94 present). | `automerge-0.6.1` (proxy) |
| §11.5 / re-plant **drift-reset** cost | E12.4 drift-reset measured | Measured as a **byte-size proxy**; direction holds, **magnitude understates** (openmls serializes blanks compactly). | `e12.4-byteproxy` (proxy) |
| §11.11 #2 return-backfill cost vs dormancy | M2 modeled | A **modeled lower bound** against redb history, not a measurement against a real history-convergence node. Mechanism now exists; the number does not. | `m2-modeled` (proxy) |
| §7.2 R7 / §8.2(e) policy-change quorum **enforcement** trust (against a subtle auth bug) | `rulechange_threshold_enforced` RED→GREEN + manual mutation gate | The **formal cross-package `cargo-mutants` sweep (X3)** has not run; positive-path coverage lives cross-package in `croft-chat`, which a substrate-only sweep can't see. Enforcement is `Modeled`, not mutation-`Verified`. | (X3 pending) |
| §7.3 / §8.2(g) **capped-root soundness** vs the Matrix uncapped-root steelman | — | **Argued, not proven.** No adversarial experiment (equivocation scheduler / bounded model checking) exists — the convergence-experiment Stage 3 is *specified only*. | §8.2(g) |
| §7.3.1 system-level **order-independence** (completeness-ahead) | Reference fold 27 tests; referenced-gap detection green | Completeness *behind* a checkpoint is shown; completeness *ahead* (the unreferenced tail — can a node know it holds the complete causal set?) is the **`Load-bearing, unearned` beam** and is not closed. | Appendix B beam |
| §6.4 metadata floor magnitude | Tier-0 meer proves zero payload keys (`Verified`) | The *key-blindness* is proven; the **quantitative leak** (relay-side timing + volume) is only *characterized by a bound*, never measured (AR-4 packet capture is sketched). | — |

**How to use this section:** before citing any of these greens as closing a spec claim, quote the
"why it falls short" column. Each is a legitimate result *at its actual scope* — the danger is only in
reading it one notch wider than it earns.

---

## 2. What is EXPLICITLY pending (named, scoped, scheduled — just undone)

Straight from `EXPERIMENT-BACKLOG.md` / `MASTER-INDEX.md` and Part 2's own open-item sections, grouped
by the gate that unblocks them.

**Runnable now, no new infrastructure:**
- **A4 / M1 fan-out** — earns the second half of §11.11 measurement #1.
- **Automerge 0.7 confirmation** — retires the `automerge-0.6.1` proxy (§7.7).
- **MLS key-distribution-over-wire + threshold-revoke-over-wire** — unblocks conformance cats 7/8/9.
- **Fold open items** — per-act approver-role granularity; two-competing-quorums → §7.6.1
  contradiction; contradicted-group byte-head naming; live "catching up…" TUI.
- **iroh Spike 1** (10K manifest sync) and **Spike 4** (ticket pairing + BIP39 confirm + MitM test).

**Needs a build first:**
- **B1 — dataplane hash structures** (§7.6.2 message plane) → unblocks **A5 / E12.2 + E12.7 message
  continuity** (the message half of re-plant).
- **X3 automated cross-package mutation sweep** (§7.3 / §8.2(g) trust gate).
- **meer P2→P6** — turn iroh lab experiments E8/E9/E11/E12 into running form (§10.3).
- **M2 sizing study** — §11.11 measurement #2.

**Spec `[confirm]` / `Load-bearing, unearned` items Part 2 itself carries** (the design-side pending):
- §7.6.3 **ReInit intent-recorded-before-freeze** ordering — `[confirm]`; E12.6 addressed availability,
  not this.
- §6.8.1 **RBSR construction choice** (Willow 3d-range vs Negentropy) — Appendix B.
- §11.11 measurements #1 (fan-out half) and #2, plus items 3–7 (gap-completeness beam, re-entry
  credential adversarial analysis, resume-vs-fresh fork, single-member resumption-PSK, non-member
  attestation) — all `Load-bearing, unearned` or `Design`.
- §8.2 honesty boundaries (b) failed-op leak-and-immune dial (design-only), (d) media engine (design),
  (f) false-positive escalation tolerance (design, value left to Group policy).

**Gated on hardware / a decision (do not start blind):**
- **X1 real-NAT** (closes `hermetic-gossip` / §8.2(a) relay path) — secroute boxes + public UDP.
- **iroh Spike 3** (macFUSE) / **Spike 7** (iOS-iroh-blob — the iroh→Veilid decision) — hardware.
- **Identity & key-recovery model** — the largest open design problem; BIP39 round-trip is the cheap
  first step.

---

## 3. What the experiments taught us that forces adapting Part 2

The things the code surfaced that the spec text does not yet reflect. These are the fold-backs; each
maps to an item in the staged diff (`proposed-changes-2026-07-experiment-reconciliation.md`).

1. **A real mechanism is missing from the spec: content-hash approval subject (→ F1).** The most
   important finding. Enforcing a RuleChange/policy-change quorum required a mechanism Part 2 does not
   name — approvals that reference the **content hash** of the exact change, counted by distinct
   personae per lineage. Without it, a threshold is *stored but not enforced* (the original bug). This
   is R3 generalized to policy changes and should be in §7.2.

2. **A spec claim was coarser in practice than written: sync-on-connect ≠ RBSR (→ F3).** We learned
   that a simple whole-log re-broadcast on join is enough to pass connect-time catch-up — which means
   the green does **not** exercise §6.8.1's efficient range reconciliation. The spec should say the
   live-gossip evidence is a coarser push cousin, and that RBSR + steady-state anti-entropy remain
   open, so nobody reads the green as "RBSR works."

3. **A design hazard's discharge is subtler than the availability fix: ReInit non-atomicity (→ F6).**
   E12.6 proves a stranded re-plant can be *completed*, but the actual discharge needs
   **intent-recorded-before-freeze ordering**, which the experiment does not establish. The spec's
   `[confirm]` is *correct to keep*; the lesson is not to mistake availability for the ordering
   guarantee.

4. **Two claims are now partly earned and should say which part (→ F2, F4, F5).** Re-plant
   **membership** continuity is proven end-to-end (message continuity is not); the **per-commit** cost
   band is measured (fan-out is not); **freshness** holds over loopback transport (the relay path is
   not). In each case the honest move is to record the earned half and name the unearned half, rather
   than leave a flat "Design"/"unearned"/"not over live transport."

5. **Substrate reality confirms two posture choices (no edit needed, but load-bearing).**
   `appview-validation` + `public-roundtrip` established that on atproto **cryptographic trust is free,
   semantic trust is not** (own your schema/threading/policy), and that **custom NSIDs propagate with
   no pre-registration**. This is the ground the §11.9.3 public-regime bridge stands on — worth citing
   there as the empirical basis, though it changes no mechanism.

6. **A design tradeoff is now measured, not just asserted: encrypt-then-content-address loses cross-user
   dedup** (`encrypted-blob-share`). The media path (§6) works over real iroh-blobs with MLS epoch
   rotation, but the dedup loss is a real, named cost. If §6 anywhere implies content-address dedup for
   sealed media, it should carry this caveat.

---

## 4. What is MISSING from the experiment corpus (spec mechanisms with no spike)

Spec sections that **no experiment touches** — the coverage holes, ranked roughly by how load-bearing
they are.

| Missing coverage | Part 2 § | Why it matters | Nearest existing work |
|---|---|---|---|
| **Non-member-verifiable membership attestation** | §11.9.3.1 | The **mechanical core of the entire public-regime bridge** — a non-member must verify "attested member at standing X authored this" without trusting the bridge or being an MLS member. `Load-bearing, unearned`; a real unsolved crypto-design problem. Nothing in the corpus attempts it. | atproto grounding only (`public-roundtrip` chain-of-custody) |
| **Adversarial / equivocation scheduler + bounded model checking** | §7.3, §8.2(g) | The capped-vs-uncapped-root soundness claim (the Matrix steelman) is **argued, not proven**, precisely because this experiment (convergence Stage 3) is *specified only*. This is the biggest unbuilt safety experiment. | reference fold (permutation tests only, benign schedules) |
| **Message-continuity across re-plant** | §7.6.2 (message half), §7.8 side histories | An in-flight conversation surviving the atomic repoint with no loss/dup. The membership half is proven; the message half needs B1 (dataplane hash structures) and is entirely unbuilt. | `replant-continuity` (membership half only) |
| **Read-scoped content-key enforcement** | §5.11 | Per-scope **asset keys wrapped to Role-holders**, **fold-gated provisioning** (a new member is keyed only after the fold admits it). `encrypted-blob-share` encrypts and content-addresses but does not exercise the *governance-gated* per-scope keying that enforces a read Role. | `encrypted-blob-share` (encryption path, not the fold-gated keying) |
| **Freshness precondition on governance ops over live transport** | §7.4.2 | "Caught up **and** corroborated-fresh (k distinct lineages on the same head) at signing." The fold enforces *thresholds*; the *freshness precondition* on originating an op is modeled, not exercised over transport. | fold threshold tests (not the freshness gate) |
| **Legitimate-governance-fork (two-group) arity + cheap merge** | §7.6.2, §7.6.7, §7.6.8 | Re-plant's **fork** arity (plant two groups, one per branch) and the **cheap-merge / permanent-both** path. `mls-replant` did the *heal* arity (dedup-not-fork, accidental fork); the genuine two-group split + merge is untested. | `mls-replant` (heal arity) |
| **Metadata-leak quantification** | §6.4, §8 (AR-4) | The relay-side timing + volume packet capture that would turn the metadata *bound* into a *measurement*. Sketched only. | Tier-0 meer key-blindness (`Verified`), not the leak magnitude |
| **Real-codec / RTP media path** | §6.12, E11 full | Real moq-rs Tracks + Opus/RTP + WebTransport; only relay *logic* is green (synthetic frames). Parked (= meer P5). | iroh E11 relay logic |
| **Total-device-loss recovery + the recovery ladder** | §11.7, Appendix B (E3.3) | The identity/key-recovery model (quorum social recovery vs VC issuer) — flagged elsewhere as the program's **biggest open problem**. Only the BIP39 round-trip is even sketched. | — |

**Reading of §4:** the corpus is strongest on the **substrate and re-plant membership** line
(local_storage_projection, mls-replant, replant-continuity, croft-chat) and on **atproto reality**
(appview-validation, public-roundtrip). It is weakest exactly where Part 2 is also weakest — the
**public-regime attestation**, the **adversarial-safety proof**, **message continuity**, and
**recovery** — which is reassuring (no blind spots hiding behind greens) but tells you where the next
builds have to go.

---

## One-screen summary

- **Not proven out** — live-transport (relay), RBSR efficiency + steady-state, Automerge 0.7, drift
  magnitude, M2 numbers, mutation-verified auth, capped-root soundness, completeness-ahead,
  metadata magnitude.
- **Explicitly pending** — A4, Automerge 0.7, key-dist/revoke-over-wire, B1→message continuity, X3,
  meer P2–P6, M2; the spec's own `[confirm]`/`unearned` items; X1 + hardware gated.
- **Forces adapting Part 2** — add the content-hash approval subject (F1); tighten §6.8.1, §8.2(a),
  §11.11 to name earned-vs-unearned halves (F2–F5); keep §7.6.3 `[confirm]` (F6).
- **Missing from the corpus** — non-member attestation, adversarial-safety proof, message continuity,
  read-scoped keying enforcement, live freshness gate, fork/merge arity, metadata magnitude, real
  media, recovery.
