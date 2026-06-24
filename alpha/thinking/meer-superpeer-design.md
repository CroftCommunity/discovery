# The meer — critical-path design for the always-on, blind superpeer (E8/E9)

date: 2026-06-16
status: thinking (design). Problem / Approach / Reasoning. The meer is the single gating unbuilt
component for the "stays alive while you sleep" story; the relay substrate (E0–E7) is proven, the
meer binary (E8/E9) is characterized but not built. This doc scopes that build.

## Problem

Croft's messaging spine is proven and its relay substrate (placement, co-location, churn, accounting,
shaping) is measured (E0–E7). But a relay is a **stateless, co-location-required forwarder** — it has
nothing to forward when no live peer is on the other end. The thing that lets a group **converge while
its members are offline** — the thing Discord's always-present servers do for free, non-extractively —
is the **meer**: an always-on protocol *participant*. It is characterized in
`experiments/iroh/RELAY-PLACEMENT-LAB-SPEC.md` (§3a relay-vs-meer, §3b confidentiality tiers, E8/E9)
but **no binary exists**. Until it does, "always-on" is a design claim, not a capability — and it is on
the critical path for a real Discord alternative (see the feasibility map in
`realtime-media-over-iroh.md` and the Discord-alt synthesis).

The meer is also where the **whole confidentiality thesis** lives: "an always-on node that shuffles
encrypted state without reading it." Getting it right (blind by default) is what makes user-run /
co-op-run infrastructure carry a stronger guarantee than Matrix self-hosting or Discord.

## Approach — one always-on participant, three blind roles, a confidentiality dial

**A meer is not a relay that stays awake.** A relay forwards opaque frames between *live* peers
(co-location required); a meer *participates* — it holds state and serves convergence for peers whose
data owner is offline. They are orthogonal and bind on different resources: a relay's ceilings are
connection-memory / passthrough-NIC / accept-CPU (E0/E1); a meer's capacity is **fan-out** and is
**payload-CPU-bound only at Tier 2** (it processes content) — at Tier 0 it is metadata-reconciliation-
bound. Measure the **offline-data fraction** (requests for state whose owner is offline) separately; no
relay tuning serves it, and it sizes the meer fleet independently.

### The three blind roles (one binary, role per workload)

1. **Message blind broker (Tier 0).** Carries encrypted ops/commits/rekeys and runs range
   reconciliation on the *metadata Willow cannot encrypt* (timestamps, digest+length, namespace/path
   hashes) — never payload. Green-real in the model as E3.4; the meer is its always-on home.
2. **Conversational SFU (RoQ).** Forwards **SFrame-encrypted RTP** it cannot decode among call peers,
   reading only RTP/SFrame headers for layer selection (the DAVE shape). Blind. The media analog of
   role 1 for *interactive* media. (See `realtime-media-over-iroh.md`.)
3. **Broadcast relay (MoQ).** Forwards **MoQ Tracks** (pub/sub) it needn't decode to subscribers; the
   **lazy** property means it costs nothing until a subscriber appears. Blind. The media analog for
   *broadcast* media.

All three are **blind by construction** — the meer forwards/reconciles ciphertext + the unavoidable
routing/order metadata, and holds no payload key. That single property is the thesis.

### The confidentiality dial (per-group policy, §3b) — what "blind" costs

- **Tier 0 — Blind mirror.** Encrypted payloads + cleartext join-metadata; range reconciliation; no
  payload key. Max reliability, min trust. **Croft default for always-on groups.**
- **Tier 1 — Double-enveloped.** An outer envelope keyed to the authorised set wraps the inner E2EE
  payload; the meer strips the outer to route/verify-sender, inner stays sealed. More sealing = less
  reconciliation ability.
- **Tier 2 — Semantic meer.** Holds the key, merges CRDTs, answers queries. Only tier that reads
  plaintext; run only where a cooperative designates a trusted always-on member.
- **No-mirror group.** Fixed-list peer set, no meer; connection allowlist + namespace capability
  (Meadowcap/MLS). Converges only when members co-online. Privacy-max, reliability-min — a governance
  dial a co-op votes on.

The dial is **chosen per group**, surfaced as policy, not a global pick. Freshness gates it: a meer
serving a stale view must surface "unverified," not "current" (`freshness-signal.md`).

### Authority and identity (do not skip)

- A meer is a **named, governed participant**, not anonymous infrastructure. Its standing to serve a
  group is a membership/authority question (the threshold dial, `revocation-authority.md`) — a group
  *admits* a meer the way it admits a member, and can revoke it (role 1 = a member that never sleeps).
- A meer **must authenticate** at the relay/accept layer (the `on_connect` HTTP gate) and carry an
  identity; "rogue meer" is a real threat (a meer that quietly upgrades Tier 0 → Tier 2, or logs
  metadata it shouldn't). Tier 0/1 meers **must be able to *prove* they hold no payload key** to the
  group (assert-and-log, per E9's failure mode), and key rotation/admission is a security-boundary
  event surfaced to members.

## Build phases (what to actually build, in order)

```
  P0  meer skeleton: always-on iroh endpoint + admission (on_connect identity) + meer_* metrics
      └ reuse: relay-lab harness, the faithful-path MLS/standing machinery
  P1  ROLE 1 — Tier-0 blind message mirror over the E3 sync workload (assert no payload key;
      log observed metadata) ........................................ this is E9 Tier 0, made real
  P2  bridge mode — straddle two namespaces/relays, measure cross-namespace fan-out ... this is E8
  P3  Tier 1 (double-envelope) + no-mirror group + the reliability-vs-overlap curve ... rest of E9
  P4  ROLE 2 — conversational SFU (RoQ): forward SFrame frames blind ......... media, with C3 keying
  P5  ROLE 3 — broadcast relay (MoQ): pub/sub Track fan-out, lazy ............. media, with iroh-live
  P6  Tier 2 (semantic) — only where a group designates a trusted meer ....... measured, gated by policy
```

P1 (Tier-0 blind message mirror) is the **first real milestone** — it converts E3.4's modeled
blind-broker result into a running always-on binary and is the foundation every other role builds on.

## Reasoning

- **Why blind-by-default is non-negotiable:** it is the entire differentiator over Matrix self-hosting
  (homeserver sees metadata + plaintext in unencrypted rooms) and Discord (central, can un-blind). A
  Tier-0 meer that *provably* holds no key is "run your own infrastructure" with a guarantee neither
  competitor offers.
- **Why one binary, three roles:** message reconciliation, conversational SFU, and broadcast relay are
  all "forward/serve ciphertext + metadata, hold no key." Collapsing them into one always-on
  participant (role per workload) keeps the trust story uniform and the deployment simple — though they
  may be *deployed* on separate boxes for the AR-4 compartmentalization reason (don't correlate media
  traffic-analysis with message metadata).
- **Why it is the critical path:** every "Discord-feel" feature that assumes an always-present server —
  history available when you rejoin, a channel that stays converged, a livestream that exists when you
  open it — needs the meer. The relay gives connectivity; the meer gives *presence-of-state*.
- **Why measure the offline-data fraction:** it is the only number that sizes the meer fleet, and it is
  invisible in relay metrics (a relay drops a packet for an offline peer; a meer answers for it). It
  decides how many meers a co-op must run.

## Anti-entrenchment — a delegated meer must not become a de-facto authority

A meer is a **revocable delegation from equal peers** (see the "delegated authority, never imposed"
principle), not an office it holds by right. The danger is **entrenchment through sheer circumstance**:
even with perfect de-jure revocability, a resourced, always-on, state-holding peer can accrue outsized
weight because it holds the state, it is always present, and few others can run it — so "reversible"
hollows out when exit or replacement is impractical. De-jure revocability is necessary but not
sufficient; the design must keep the delegation **materially** reversible:

1. **No state hostage — state portability is the primary guard.** The meer holds **encrypted** state it
   cannot read; the group holds the keys; the state is replicated/exportable so the group can **re-host
   on another meer or leave cheaply.** Losing a meer costs *availability*, never *data*. A meer that
   becomes the sole custodian of recoverable state is the entrenchment failure — design against it.
2. **Blindness caps the weight.** A blind meer (Tier 0) cannot accrue *content* power; its circumstantial
   advantage is bounded to availability + the AR-4 metadata. This is *why* the default-blind posture is
   also an anti-entrenchment property, not only a privacy one. (A geer — content-visible — is the
   higher-risk case; its label-not-enforce separation and revocability are the corresponding caps,
   `geer-gating-peer.md`.)
3. **The trap-door / re-formation backstop is the ultimate exit.** If a meer (or a captured election)
   entrenches, the group can **fork — re-form the lineage minus the incumbent**, carrying the state it
   holds and excluding the entrenched party (the proven C3/re-formation result, applied to roles). De-
   facto entrenchment is therefore never terminal: there is always a clean, attributable exit.
4. **Scoped, non-creeping rights.** The meer forwards/serves/reconciles; it does **not** govern. Role
   creep ("it's always there, let it decide") is the entrenchment vector — keep its capabilities
   downstream of an explicit, minimal, revocable grant.
5. **Stand up a replacement and elect it — the decisive guard.** The group does not have to *depend on
   an alternative already existing*; the software is open and the role is a re-issuable grant, so a
   group of peers can **stand up a fresh meer/geer (different hardware, different party) and elect it in
   place of the incumbent.** Combined with state portability (#1), the incumbent has **no lock-in**:
   the group re-hosts its (encrypted) state on the new peer it stood up and re-issues the grant. This is
   the everyday, low-drama replacement path — and because a *different party's* peer can be elected, it
   also diversifies away from a curious/entrenched incumbent. **Routine replacement (this) vs the fork
   (#3) is the key pair:** rotation is the normal check; the trap-door fork is the adversarial backstop.
   Rotation-friendly governance defaults (multiple holders / periodic re-election) keep it normal.
6. **Metadata transparency.** The meer sees metadata members don't (AR-4); surface what it sees and keep
   it minimal (e.g. the content-free freshness beacon), so informational asymmetry isn't a quiet weight.

The ethos: **equal peers delegate revocable authority, and the substrate must make that revocation
real — not just by allowing exit, but by letting the group *stand up and elect a different holder* —
so a delegate never hardens into an owner by circumstance.**

## Open edges

- **Rogue/curious meer** detection: how a group verifies a Tier-0/1 meer's blindness over time (not
  just at admission) — ties to the failed-op/immune-signal dial and to a "prove-no-key" protocol.
- **Meer fleet sizing** under real offline-data fraction — unmeasured; E8/E9 + P2 produce the first
  numbers.
- **Tier-2 governance:** when is a semantic meer ever acceptable, and how is that consent expressed and
  revoked without bricking the group.
- **Media-meer cost** (roles 2/3) is the heavy-throughput case (E0 active-passthrough wall, AR-4-loud);
  likely a separate box class from the message meer.
- **Recovery interaction:** a meer holding Tier-0 encrypted history is *also* a backup substrate for
  multi-device recovery — but only if the user holds the keys; this couples to the recovery design
  (the other big open gap).
