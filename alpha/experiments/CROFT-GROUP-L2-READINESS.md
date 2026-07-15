# croft-group L2 (MLS / encryption) — readiness brief

`Status: read-and-report brief (RUN-10 Part 4). No spec/register/crate/frozen-record edit; no code; the
identified slice is shaped, not built. This is an index over existing evidence, never a source of truth —
where this brief and an EVIDENCE-MAP row or a Part 2 tag disagree, the row/sentence wins and this brief is
the bug.`

`Scope: what croft-group Layer-2 (MLS / encryption — the plan's L2) concretely requires; which of those
requirements the proven Drystone crates satisfy today (each cited to its EVIDENCE-MAP row); which genuinely
wait on the parked Layer-2 resolution-ACL design or on the open I9 trust-tier call; and the largest L2 slice
buildable now without touching either. The reuse decision is on record and assumed
(SPEC-ALIGNMENT-AND-ACTION-PLAN.md §"Decided (RUN-02)"; EXPERIMENT-BACKLOG §3): L2–L5 build on the proven
crates as a condition of considered compatibility.`

Serves: croft-group L2 readiness — what Layer-2 concretely requires, what the proven crates satisfy today,
what waits on the parked Layer-2 design / the I9 call, and the largest L2 slice buildable now.

Status flags (A.9-adjacent, as used by EVIDENCE-MAP / the experiment corpus): `Verified` / `green-real`
(exercised against real crypto/transport, has a real test), `Design` / `Sketched` (reasoning or plan only,
no test), `[gated]` (blocked on the parked design or on I9). Owner's-decision points are flagged
**OWNER-DECISION**; the I9 firewall holds — this brief decides no trust tier.

---

## 0. Where L2 sits, and one naming caution (FINDING F-L2-NAME)

croft-group's own layer ladder (plan `croft-group/plans/2026-06-22-1-…`, "Later phases"; EXPERIMENT-BACKLOG
§3) numbers **L2 = MLS / encryption**: "`Frame` payload becomes MLS-ciphertext; key/epoch state enters the
core; `Zeroize` applies." That is the L2 this brief is about.

The **parked "Layer-2 resolution-ACL" design** is a *different* Layer-2. It is the disagreement-projection
layer of `alpha/thinking/the-shape-of-disagreement.md` §10 step 3: "Design the resolution ACL (Layer 2,
design-gated) — the closed verb set + matcher language, evaluated deterministically and kept **private per
node** (the profiling floor)." There, Layer 1 = objective cryptographic fork detection; Layer 2 = the private,
per-node *projection* of which branch a viewer follows. That maps onto croft-group **L3** (fork/merge +
reconvergence-per-plane) and the read-scope half of **L4**, **not** onto croft-group L2=MLS-encryption.

The two "L2"s share a number and nothing else. The load-bearing consequence: **croft-group L2 (encrypt the
frame) does not depend on the parked resolution-ACL design** — you can seal the happy-path frame with MLS
without deciding whose projection wins a fork. Keeping the two namespaces distinct is what lets §3 below name
a genuinely non-trivial buildable slice. (Recorded as a naming/definitional finding, not a spec
contradiction — no EVIDENCE-MAP row or Part 2 sentence is contradicted.)

---

## 1. What L2 concretely requires

From the plan's L2 (folding L1's identity item, which "may fold into L2") and the group-core seam it extends
(`group-core/src/{wire,effect,model,update}.rs`), plus what "MLS / encryption for a Group" means against the
Drystone Group model (asset-keying.md §A "Group"; Part 2 §6/§10.2):

- **R1 — Sealed frame.** The `Frame` payload (today plaintext version-tagged JSON, `wire.rs::FrameV1`)
  becomes MLS application-message ciphertext: `SendMessage` seals, `FrameReceived` opens. A non-member / no-key
  peer cannot read it.
- **R2 — Key & epoch state.** An MLS group with epoch/ratchet state, held outside the pure WASM-clean
  `group-core` (the core emits topic-free effect *data* and must not hold secrets — DECISION 1 / DECISION 4).
  A sibling crate or the shell owns the keystore.
- **R3 — Join = key distribution (Welcome).** A joiner obtains the group's read key over the wire (an MLS
  Welcome), deriving the same epoch secret as the committer — the L2 realization of the happy-path `JoinGroup`
  → `Subscribe`.
- **R4 — Membership-change re-key.** Adding/removing a member advances the epoch and re-keys; a removed member
  loses forward read access (survivor-epoch re-key / PCS); a fresh stamp seats exactly the governed member set.
- **R5 — Sender identity from the credential (folds L1).** `ChatMessage.sender` (today a hardcoded handle)
  becomes the MLS credential / lineage identity; the wire version bumps to carry it.
- **R6 — `Zeroize` on secret material.** All key/epoch/exporter material lives in `Zeroize`-on-drop newtypes
  with no `Debug`; no secret bytes leak into an `Effect`, a `WireError`, or a rendered view (the CLAUDE.md
  "Zeroize for any secret material … arrives with MLS/encryption (L2)" obligation).
- **R7 — Membership authority = the governance-derived set.** MLS membership is a *function of* the fold's
  derived member set, never an independent authority (the re-plant keystone).
- **R8 — Recovery anchor.** A key-backup/recovery path (BIP39 Tier-1 lock) so a persona can recover read
  access. *The lock mechanism is L2-adjacent; the trust tier of who-may-recover is I9 (see §3).*
- **R9 — Read-scope / decryption ACL.** Which viewer may open which asset under membership change and under a
  fork — the read-scoped resolution. *This is the parked resolution-ACL surface; see §3.*
- **R10 — Authorized revocation over the wire.** Removing a member *with authority* (who may revoke, the k-of-n
  dial, co-sign-vs-vote ordering) as distinct from the mechanical re-key. *This is I9; see §3.*

---

## 2. What the proven crates satisfy TODAY

Each row cites the EVIDENCE-MAP row (`beta/drystone-spec/EVIDENCE-MAP.md`) that carries the `Verified` /
`green-real` status. All are real-test-backed; grade bounds are quoted, not softened.

| Req | Satisfied by (crate) | EVIDENCE-MAP row (quoted) | Grade / honest bound |
|---|---|---|---|
| **R3** (Welcome key distribution) | `mls-welcome-over-iroh` → path-deps `Proofs/lineage-groups/lineage-mls` | §10.5(a) "MLS key-distribution over the wire (verifying-key/standing registry sourced from real MLS)" — `Verified` | **loopback** grade; "a real 1466-byte openmls Welcome crosses a real iroh connection; the joiner derives the identical MLS exporter secret (`epoch_secret_match: true`)" (RUN-08). Real-NAT = X1; not yet in the conformance emitter. |
| **R4** (membership re-key; survivor-epoch / PCS; fresh stamp = derived set) | `mls-replant` (E12.3/5/6), `replant-continuity` (E12.7) | §7.6.2 "re-plant **membership** continuity: MLS group stamps exactly the fold-derived set" — `Verified` | "**membership half only; message half not built**; real openmls 0.8.1." E12.5 = group-wide leaf re-key; E12.7.2 = removal re-keys the departed member out; E12.6 = last-resort availability. |
| **R7** (membership authority = derived set) | `replant-continuity` (E12.7.1/.3), `local_storage_projection` fold | §7.6.2 (above) + §7.2 R7 "content-bound-quorum **count** enforced" / §7.6 "contradiction hard-stop" — `Verified` | E12.7.3: "an add authored by a non-member is **rejected at ingest** … the fold — not MLS — is the sole membership authority." Governance fold: substrate lib 97/0. |
| **R5** (credential/lineage identity, signing, forged-message reject, two-devices-fold-to-one) | conformance-core `run-vectors` (`Proofs/lineage-groups/conformance`) | §4.1–4.6 "tagged wire derivations; forged-message reject; two-devices-fold-to-one" — `Verified` | "against real Ed25519 / live iroh; conformance cats 1–3"; 66/0 re-proven (RUN-08). The lineage fold `fold_matches: true` (alice's two devices → one actor) is exercised in `mls-welcome-over-iroh`. |
| **R6 + R8** (Zeroize secret newtypes; recovery anchor round-trip + secretbox wrap) | `bip39-recovery-roundtrip` | §7.3.9 "recovery-anchor **Tier-1 lock** mechanism (BIP39 round-trip + secretbox-wrap)" — `Verified` (experiment-grade) | "in-process; crate choice not `[gates-release]`; the **trust tier is I9**." 11/11 green; secret newtypes are `Zeroize` + no `Debug`; secretbox wrap/unwrap bit-exact, wrong-key/tamper fail cleanly. Proves the *lock*; **who may open it is I9**. |

**Net (§2).** R3, R4-membership-half, R5, R6, R7, and the R8 lock *mechanism* are `Verified`/`green-real`
against real openmls 0.8.1 + real iroh + real Ed25519, at loopback grade. The crypto and the wire that L2
needs are proven — reuse (not re-implementation) is available and is the compatibility condition.

---

## 3. What genuinely WAITS

### 3a. Waits on the PARKED resolution-ACL design (the other "Layer 2")

- **R9 — read-scope / decryption ACL under fork.** The resolution-ACL DSL is "**design-gated**" — "the closed
  verb set + matcher language … evaluated deterministically and kept **private per node**"
  (the-shape-of-disagreement.md §10 step 3; "Steps 3–5 … remain the design frontier and remain a human call").
  Until the verb set + matcher language exist there is nothing to build against for *which viewer projects
  which branch* / read-scopes which asset in a multi-head world. **This is croft-group L3, not L2** (F-L2-NAME).
  It does **not** block sealing the single-head happy-path frame.
  **OWNER-DECISION:** the verb set/DSL is an explicit human call, not an autonomous build.

### 3b. Waits on the I9 trust-tier call (firewall holds — this brief decides nothing)

- **R10 — authorized revocation over the wire.** EVIDENCE-MAP §10.5(b): "threshold-revoke as real k-of-n over
  the wire + co-sign-vs-vote ordering" — `Design` (gated), bound "**firewall — the revocation-authority trust
  model (I9)**"; evidence is the "MD-G5 sha-256 MAC stand-in", gate "I9 revocation-authority decision."
  MASTER-INDEX I1: threshold-revoke = "⛔ DESIGN-GATED, firewall." What waits is *who may revoke, the k-of-n
  dial, co-sign-vs-vote ordering* — **not** the mechanical re-key, which is proven under R4/R7 keyed off the
  already-`Verified` governance set.
- **R8 — the trust tier of recovery.** The BIP39 lock is `Verified`, but §7.3.9 gates the **Tier-2 trust
  predicate (I9, firewall)**: quorum-of-Ed25519 social recovery vs VC-issuer (MASTER-INDEX I9, "the largest
  open problem"). The lock round-trips today; *who is authorized to recover a persona's key* is I9.
  **OWNER-DECISION:** the identity/key-recovery + revocation-authority model (I9) is the run's largest open
  call and is out of scope here.

**Why these and not the rest.** R10/R8-tier are the *authority* half of encryption's lifecycle — deciding
*who is empowered* to remove a reader or restore a reader. That is trust-tier policy (I9), firewalled from the
mechanism. R9 is the *projection* half — whose lens resolves a fork — which is the parked resolution-ACL. Both
are downstream of, and separable from, R1–R7 (seal a frame, distribute the key, re-key on a governed
membership change), which the crates already satisfy.

---

## 4. The largest L2 slice buildable NOW (shaped, NOT built)

**Slice "L2a — sealed happy-path frame over reused MLS."** Turn the plaintext `FrameV1` into an MLS
application-message ciphertext, obtain the read key via a reused Welcome, hold epoch/key state in a sibling
crate outside the pure core, and re-key on a *governance-derived* membership change — reusing
`lineage-mls` + `mls-welcome-over-iroh` + `mls-replant`/`replant-continuity` + the `bip39`/`dryoc` Zeroize
pattern. Covers R1, R2, R3, R4-membership-half, R5, R6, R7. Touches **no** resolution-ACL (no fork projection)
and **no** I9 (no revocation-authority model, no recovery-trust tier). This is non-trivial and is the whole of
L2's *mechanism* half; the *authority/projection* halves (R8-tier, R9, R10) are the firewalled remainder.

Shaped below as a backlog row with RED-able assertions — concrete failing tests a future run turns GREEN. **It
is not built here.**

> **Backlog row (proposed for EXPERIMENT-BACKLOG §3, croft-group L-series). Not built this run.**
>
> **L2a — MLS-sealed happy-path frame (mechanism half of L2).** *Maturity: Sketched → shaped (RED-able).*
> *Reuses (compatibility condition, RUN-02): `Proofs/lineage-groups/lineage-mls`, `mls-welcome-over-iroh`,
> `mls-replant` + `replant-continuity`, the `bip39-recovery-roundtrip` Zeroize/secretbox pattern.* *Firewall:
> touches neither the parked resolution-ACL (L3) nor I9 (revocation-authority / recovery trust tier).*
>
> RED-able assertions (each a failing test to turn green):
> 1. **Seal ≠ plaintext, and round-trips.** On a joined, keyed two-peer group, `SendMessage{"hi"}` emits a
>    `Publish{bytes}` whose bytes do **not** contain the substring `hi` (assert sealed), and peer B's
>    `FrameReceived{bytes}` decrypts to the identical `ChatMessage` — round-trip through real openmls
>    ciphertext, not JSON. (Mutation: swap seal→identity → assertion 1 fails.)
> 2. **No key ⇒ observable drop, no panic.** A peer that never received a Welcome, fed the sealed frame,
>    emits exactly one `FrameDropped{reason}` and its `Model` is byte-identical-unchanged (hostile/undecryptable
>    frame survived per the existing hostile-input discipline).
> 3. **Welcome distributes the read key.** A joiner processing a real openmls `Welcome` (reuse
>    `mls-welcome-over-iroh`) derives the committer's exporter secret (`epoch_secret_match: true`) and then
>    decrypts a message sealed in that epoch — join = key distribution, end to end.
> 4. **Governed removal re-keys the departed reader out (PCS).** An authorized removal driven by the
>    fold-derived member set (reuse `replant-continuity` `e12_7_2_removal_propagates`) advances the epoch; the
>    removed member cannot decrypt a post-removal message while a retained member can. No revocation-*authority*
>    knob is introduced — the removal rides the existing `Verified` k-of-n threshold path (firewall).
> 5. **Secrets never leak the core.** All key/epoch/exporter material is a `Zeroize`-on-drop, no-`Debug`
>    newtype living in the sibling crate/shell; a test asserts no secret bytes appear in any `Effect` variant or
>    `WireError`, and `group-core` gains no transport/crypto dep (DECISION 1/4 hold — the pure core stays
>    WASM-clean and secret-free).
> 6. **Firewall guard.** The slice's public API exposes no "who-may-revoke", no co-sign-vs-vote ordering, and
>    no recovery-trust-tier selector; a guard test/grep asserts their absence so L2a cannot silently pre-decide
>    I9 or the resolution-ACL.
>
> Retirement condition: all six green against real openmls 0.8.1 at loopback grade, `group-core` unchanged in
> purity, and a RUN summary recording the reuse (not re-implementation) of the four named crates. Real-NAT
> stays X1; the authority/projection halves (R8-tier, R9, R10) remain their own gated rows.

---

## FINDINGS

- **F-L2-NAME (definitional, LOW).** Two distinct layers share the label "Layer 2 / L2": croft-group's
  **L2 = MLS/encryption** (plan/backlog §3) vs the parked **Layer-2 resolution-ACL** (the-shape-of-disagreement
  §10, a fork-projection layer = croft-group L3). No EVIDENCE-MAP row or Part 2 sentence is contradicted; the
  finding is that the shared number invites conflating them, and the load-bearing correction is that L2=MLS
  does **not** depend on the parked resolution-ACL. Recorded, not auto-rewritten.

*(No EVIDENCE-MAP-vs-spec contradiction was found this pass. §7.6.2's explicit "message half not built" bound
is consistent with L2 being unbuilt; §10.5(a)/(b), §7.3.9, §4.1–4.6 are cited as-written.)*
