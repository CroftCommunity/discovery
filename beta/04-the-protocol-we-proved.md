# 04 — The protocol we proved: lineage groups, green-real

date: 2026-06-24

status: synthesis (least-gated, spine-complete). Verification: every claim below carries its proof
status — `green-real` (real openmls 0.8.1 / real iroh), `green-real-multimachine` (≥3 real hosts),
`green-model` (TS simulation, real SHA-256 ancestry / modeled MLS+transport), `spec` (defined, not
built), or `characterized` (measured, bound documented). The make-or-break gate is GO; the residual
risks are named, not hidden.

> **One-line thesis.** Model a group as a *navigable lineage of conversations* rather than a flat
> eternal room; bind an MLS key ratchet to a separate signed governance log and a per-branch history
> CRDT; under partition, fork cleanly and escalate genuine conflict to a human rather than
> auto-adjudicate — and this is **green-real on openmls 0.8.1**, not a sketch. The same machinery
> required for security (provable genesis, signed membership ops, a verifiable epoch chain) is exactly
> what makes the social model legible, because *"knowing where a conversation branched from" is at once
> the security invariant and the social-legibility invariant.*

This theme is the engineering instantiation of `01`'s razor (compute provenance, never utility). Its
adversarial/social complement — staying safe while content-blind — graduated to its own theme, `06`.
The identity floor it counts standing against is `05`. The names (Croft the umbrella, **Drystone** the
protocol) are introduced in `02`.

---

## Theme narrative (overview)

This theme is the proven core. It starts from a mismatch: messaging tools model a group as a flat,
eternal room, but human groups fork, splinter, go dormant, revive, and recombine. Croft models the group
as a **navigable lineage of conversations** grounded in cryptographic provenance — and the payoff is
double in a way that turns out to be a single property: *knowing where a conversation branched from* is
at once the security invariant (standing, legitimacy of a recombine) and the social-legibility invariant
(faithful, foldable history).

It then lays out the **two-tree model** — a forward-only signed *governance log* bound to a single,
linear *MLS epoch chain* that cannot merge — plus a per-branch *history CRDT* that never merges either.
The binding rule (a governance event is enacted only once realized as an MLS commit) and the **survivor
reconnect model** (deterministically pick a surviving epoch, re-key the other side, heal silently or
hard-stop to a human, leave a rejected merge as a resting state) are the heart of the design.

The middle of the theme is **what was actually proved**, with status on every claim: the Phase-1
crypto-feasibility gate is **GO on real openmls 0.8.1** (survivor re-key with post-compromise security);
invariants I1–I10, multi-device (thresholds count lineages, not leaves), cross-machine byte-identical
reconcile, a faithful signature-AND-standing wire test, and a 66/0 conformance suite all hold. The
adversarial passes *falsified* as well as confirmed — they found and closed two real gaps. Media then
rides the **same substrate, blind** (E10 transport bound, E12 MLS-keyed E2EE, E11 metadata-only
broadcast admission, a running blind `meer`).

The theme closes honestly: what a validation does *and does not* establish, and the carried-open risks —
**total-device-loss recovery** (the largest residual), V3's automatic-only guarantee, S3 quiet
membership still unsolved, and the broadcast-tier ratchet-tree cost. (The spec-vs-code reconciliation
item, carried-open at first-pass, was surfaced and resolved 2026-06-17 — see §5.) The reader leaves
knowing the protocol is real, not a sketch — and exactly where the edges are.

## Charter — what this theme covers

**In scope.**

- The lineage-groups protocol thesis (group as navigable lineage; provenance as the dual primitive).
- The two-tree model (governance log + MLS epoch chain) and per-branch history CRDT; the binding rule.
- Survivor reconnect semantics (deterministic survivor, re-key, silent-heal vs. hard-stop, resting-state).
- Invariants I1–I10 and the proof apparatus + results (phases 1–3, adversarial passes, multi-device,
  cross-machine, the faithful wire test, conformance).
- Media riding the same substrate blind (E10/E12/E11/meer) — as protocol/transport facts.
- The honest boundaries and carried-open protocol risks.

**Out of scope (and where it lives).**

- The epistemic *why* the protocol embodies → `01`.
- Naming / what "Drystone" means and the commons rationale → `02`.
- The DID-lineage identity story and the recovery-anchor *decision* → `05`.
- The adversarial/*social* safety problem (abuse hubs, the geer, freshness-as-authority-gate,
  membership-vs-access) → `06`.
- The product surface that consumes the substrate (the Croft Group pond) → `08`.

**Boundary calls.**

- 04 owns the **cryptographic/structural** invariants (what verifies); `06` owns the
  **adversarial/social** problem (what a hostile member or abuse hub does). They *share* the
  `freshness-signal` and `revocation-authority` mechanisms: stated here as protocol facts, reasoned
  about socially in `06`.
- "Keys are not identity; thresholds count lineages, not leaves" appears here as a *protocol invariant*;
  the DID-lineage / cross-platform / recovery story is `05`.

---

## 1. Why a lineage, not a room

Messaging infrastructure models a group as a flat, eternal container: a fixed room, one linear scroll,
one mutable member list. Human groups are not like that — they fork, splinter, go dormant, revive, and
recombine. Today's tools fight that reality, so users absorb the friction as normal: a new group every
time composition shifts, no relationship preserved between "vacation 2024" and "vacation 2025," no way
to express that *this* splinter came from *that* group.

Croft models the group as a **navigable lineage** grounded in cryptographic provenance. The payoff is
double and turns out to be one property: reliable knowledge of where a group branched from is what lets
the system reason about standing and the legitimacy of a recombine (security), and it is also exactly
the structure needed to represent fluid, branching social reality faithfully (legibility). Foldable
histories that are *real* (cryptographically guaranteed unaltered) and unfoldable *legitimately*
(lineage proves standing) are not an accident — they are one fact with two payoffs.

## 2. The two-tree model (the design lives in the binding)

There are two distinct data structures, and the whole design lives in how they bind:

```
 ┌───────────────────────────────────┐     ┌──────────────────────────────────┐
 │ GOVERNANCE TREE                    │     │ MLS EPOCH CHAIN                   │
 │ forward-only signed log of         │ bind│ the real key ratchet             │
 │ membership ops, evaluated against  │ ◄──►│ (openmls, RFC 9420)              │
 │ IMMUTABLE genesis rules.           │     │ single, linear, per-epoch rekey  │
 │ CRDT-friendly · fork-detecting ·   │     │ on membership change.            │
 │ attributable · CAN fork and heal   │     │ It CANNOT merge. Ever.           │
 └───────────────────────────────────┘     └──────────────────────────────────┘
   binding rule: a governance event is "enacted" only once realized as an MLS commit.
```

Plus a third, per-branch: **history is Automerge** — just data, one document per branch, and it *never*
merges into one canonical transcript. Interleaving two forked branches by timestamp produces noise
("six tapes playing in a room"), so it is structurally not done. We are explicit that we **compose** MLS
with two non-MLS structures (governance log + history CRDT), not extending MLS.

**Reconnect is the survivor model.** Literal merge is impossible — two keys cannot occupy the same
space — so every "merge" is really *one side adopts a surviving epoch, or both adopt a fresh third
genesis*:

- **Pick a survivor epoch deterministically** (both sides compute the same winner with no negotiation).
- **Re-key the losing side's members into the survivor** via MLS external commits; members keep
  identity, only their live key changes.
- **History does not merge** — folded-away branches stay navigable per person (the "code-fold" model).
- **Check governance ops for contradiction:** no conflict → heal silently; real conflict (one side
  booted someone the other still includes) → **hard-stop, escalate to a human**.
- **A rejected merge is a resting state, not an error** — the two groups simply remain two groups, each
  with intact lineage. The fresh-genesis "sixteenth-great-grandparent" path exists only when both sides
  *want* to merge but neither epoch is acceptable.

### The substrate model — capability vs. authority, and planes of blast radius

Two design invariants shape what the substrate is allowed to delegate. They are not proofs; they are the
rules the proven mechanisms above are built to respect.

**Capability is not authority.** *Capability* means "able to do the work"; *authority* means "permitted
to act as you." A single grant must never conflate the two. A delegation is modeled as a
**(predicate, sealed-payload) pair** — a peer or threshold holds it and emits the payload when the
predicate fires — with exactly three knobs: a **trigger** (time, event, peer-online, or
quorum-attested condition), a **threshold** (1 for capability; k-of-n for authority), and
**attribution** (the emission names its trigger and its delegate, for audit). The safety rule is
**courier, not agent**: pre-seal the payload so the delegate carries no abusable authority and delegate
only the trigger (a dead-man's-switch, a timelock, an escrowed pre-signed action). Only an action whose
content depends on *future* state cannot be pre-sealed, and only that action needs live authority and
full threshold treatment — so the courier-vs-agent boundary is decided before building, because it sets
the blast radius. The load-bearing invariant: **capability delegation can outlive the principal;
authority delegation cannot exist without a living principal.** *Liveness is the boundary* — which is
why search, discovery, and group-liveness are separately considerable, and distinct from revocation.

**Planes are equivalence classes of blast radius.** A *plane* is a functional grouping (chat,
scheduling, action/C2, audit, governance) whose members share a substrate but differ in *the consequence
of being wrong*. Planes are anchored to **principals, not groups** — a principal is any identity that can
hold and delegate authority (person, device, role, service), forming a hierarchy where delegation flows
down. A delegation is scoped to a single **(principal, plane)** cell, so a compromised token leaks one
cell of that grid and no more. The invariant that enforces it: **namespace delegations never cross.**
Each token is bound to its plane's namespace as a non-removable caveat, checked at emit time, so
cross-plane reuse **fails closed** by construction — the substrate is shared, the namespaces are not.
**Reconvergence policy is per-plane** (asset-overridable), declared at intent-to-collaborate and bound
immutably into the asset's hash: automatic merge where divergence is incidental concurrency, human-gated
reconvergence where divergence is substantive disagreement. The substrate cannot tell concurrent typing
from fundamental disagreement at merge time — only the declared meaning can — so the policy is made as
non-equivocable as the content it governs.

## 3. What was actually proved

The thesis was sequenced so the riskiest thing ran first. The protocol invariants I1–I10 and the phased
experiments were validated against **real openmls 0.8.1** (the `Proofs/lineage-groups` Rust workspace)
and a TS model with real SHA-256 ancestry (`Proofs/lineage-group-model`).

**The Phase 1 crypto-feasibility gate — the make-or-break of the whole thesis — is GO** (`green-real`).
openmls 0.8.1 expresses survivor-epoch re-key with post-compromise security intact: removed members
cannot decrypt post-removal traffic (E1.1); an external commit brings a B-member into A's epoch with
identical group secrets — the survivor primitive (E1.2); fresh-genesis produces a clean third epoch with
both parents dead (E1.3); a queued remove commit applied later still rekeys and the removed member stays
out — the broker-carries-revocation claim (E1.4). Every downstream phase was conditional on this; it
held.

| Layer | Result | Status |
|---|---|---|
| Phase 1 — survivor re-key + PCS on openmls 0.8.1 (E1.1–E1.4) | the gate; GO | **green-real** |
| Phase 2 — two-tree model, deterministic survivor, conflict hard-stop, verifiable backfill (E2.1–E2.8) | all hold under fuzzed orderings | **green-real** |
| Invariants I1–I10 | genesis immutability, threshold soundness, provenance-from-signed-data, forward-key linearity, deterministic survivor, no-silent-contradiction, history-never-corrupts, backfill-verifiability, fold-inert, convergence | hold | **green-real** (I9 green-model) |
| Multi-device (per-device keys, one lineage) | thresholds count *lineages, not leaves* — own-device quorum manufacture blocked; T1 lineage claim rides a real openmls leaf | E2.9–E2.15, T1, INV-LINEAGE-NOT-LEAF | **green-real** |
| Phase 3 — thin slice over real iroh + blind broker | partition→broker-carries-commit→one live epoch; contradiction hard-stops over the wire; broker observes only ciphertext + routing | E3.1/E3.2/E3.4 | **green-real** (IP/timing still observable, as expected) |
| Cross-machine (3 AWS boxes + a NAT'd laptop) | disconnected peers compute a **byte-identical** reconcile verdict; the superpeer is a capability, not a right (no broker-only outcome); the trap-door re-formation yields an identical reformed genesis on all hosts | A1/A3/B-series | **green-real-multimachine** |
| Faithful wire test | the real Ed25519-signed message verified for **signature AND standing** on receipt: HONEST→accept, FORGED→reject (BadSignature), NONMEMBER (valid sig, no standing)→reject (UnauthorizedAuthor) — the attack a hash chain cannot catch | — | **green-real** |
| Governance-log roll-up / threshold-signed checkpoint | settled, un-forked history compacts into a checkpoint a **quorum of admin lineages** co-signs (real Ed25519), so a client renders the member list without replaying the whole log; a **single-authority/broker checkpoint is rejected**, the head must match the log, and a checkpoint **cannot span a fork** — the broker is no finality authority | — | **green-real** (CLOSED 2026-06-16) |
| Conformance suite v0.1.0 | a black-box vector suite a second implementation must pass, derived from the real code | 66 pass / 0 fail | **green** |

**The adversarial passes earned their keep** — they falsified, not just confirmed. Two real gaps were
found and closed: governance equivocation needed hardening (A2.2 → `detect_equivocation`, now C9
green-real), and a departed genesis-admin who still governed (A2.4 → **authority is now per-epoch**). A
hard-stop can be cleared only by an explicit threshold-signed `quorum_override`; below threshold the
stop stands (A2.5). Sybil lineages never reach a threshold without authorized admin standing (AR-1);
DID double-counting and replay are rejected (AR-6).

## 4. Media rides the same substrate, blind

A later round proved the substrate carries real-time media without giving the relay sight of it
(2026-06-17):

- **Transport (E10, `characterized`):** iroh's QUIC datagram path passes loss/delay/jitter through
  transparently to a media estimator (audio holds to 30% loss with visible degradation), but an
  over-cap source bufferbloats — so the media estimator **must be authoritative and back off on the
  path-RTT trend**, and media + bulk transfer **must be on separate flows** (a bulk transfer on the same
  connection starves the media).
- **Keying (E12, `green-real`):** media E2EE keyed off a real openmls 0.8.1 group — per-sender SFrame
  key = HKDF(MLS exporter secret, epoch‖leaf); distinct per-sender keys, out-of-order frames decrypt,
  replays rejected, removal rotates the epoch so a revoked sender's later frames fail, and a **blind SFU
  recovers 0 bytes of plaintext** from clear headers. (Synthetic frames / modeled SFrame header — the
  honest scope.)
- **Broadcast (E11, `characterized`):** a lazy MoQ-style relay produces zero publisher egress with no
  subscriber, fans out linearly in N, stays blind, and admits by **metadata only** (max-audience cap,
  members-only) — reading zero frame bytes. That is the abuse answer in one line, carried into `06`:
  *limit scale and membership from metadata alone, never by content inspection.*
- **Meer (P0+P1, `green-real`):** a blind always-on superpeer (`meer`) runs as a binary — it serves
  offline state holding **zero payload keys**, an offline member syncs through it and decrypts locally,
  and the member can be **re-homed into a replacement meer** (state portability — the anti-entrenchment
  guard that keeps delegation materially reversible, per `01`/`07`).

## 5. The honest boundaries

A successful validation establishes that the two-tree model is implementable on real libraries; that
survivor-epoch reconnect with PCS is expressible on openmls; that governance fork/heal with
deterministic survivor selection and conflict hard-stop is sound under adversarial reordering; that
history-as-navigable-tree with verifiable consensual backfill works; and that an optional blind broker
can carry revocation/rekey and recovery snapshots. It does **not** establish production readiness,
large-scale behavior, real-world UX of fold/unfold and conflict-escalation flows, network-layer metadata
resistance, or an audited security posture — and none of those should be claimed from it.

Carried open items:

- **Total-device-loss recovery (the largest residual risk).** A new device for the same DID joining via
  external commit + a broker-held snapshot is designed (E3.3) but **not proven**; the recovery-anchor
  choice (trust-minimized key backup vs. device delegation) is a standing decision the user owns
  (see `05`).
- **V3 — no-silent-regime-crossing is structural for *automatic* crossing only** (`green-model`). The
  protocol cannot stop a human from typing intimate text into a public republish; that is a UX-layer
  control and must be addressed before shipping republish. (Carried into `06`.)
- **S3 quiet membership (reachable without being mapped) is still `spec` — the hard one, unsolved.**
- **Broadcast tier must disable the embedded MLS ratchet-tree** — with it on, commits grow ~linearly in
  member count (1.4 KB @ 8 → 11 KB @ 128 leaves; AR-5, measured on openmls 0.8.1). Affordable at human
  scale, not at broadcast scale.
- **The unbounded-log death is closed, not carried.** Splitting the governance log into a settled,
  un-forked past and a still-churning live tail, then compacting the settled part into a threshold-signed
  checkpoint (above), is the direct answer to the SSB failure `03` names as a cautionary tale: every
  client no longer replays an ever-growing log just to render the member list. The cost moves from a
  continuous, mandatory, grows-forever verification to a periodic, optional, bounded one — and because
  the checkpoint is signed by a quorum of admin lineages rather than an authority or broker, the
  decentralization survives the compaction. This is `green-real`, not a managed-someday item.
- **Spec-vs-code reconciliation — surfaced AND resolved (2026-06-17).** The earlier divergence between
  the spec's §2 domain-tagged genesis/topic pre-images and the code's plain `sha256` +
  `"lineage-topic-v1"` topic tag was closed: the tagged pre-images are now canonical in both the spec
  (§2, incl. the 2026-06-17 addendum) and the code, and the
  derivations are byte-identical (proof ledger: "RESOLVED"; test narrative: "surfaced and
  resolved"). No live spec/code divergence remains; this is no longer an open item.

**Verification note:** for iroh facts cite the FACTCHECK SoT — iroh `1.0.0`; iroh-docs uses range-based
set reconciliation + LWW, not Merkle Search Trees. Do not re-verify.
