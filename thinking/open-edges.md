# Open edges — consolidated review surface

date: 2026-06-16
status: triage. Harvests every "Open edges" bullet from `crystallized/test-narrative.md`, the
cross-cutting surface, the design-doc open edges, and the explicitly-gated corpus items, into one
place to triage from. Each edge is tagged:

- **[doable]** — no blocker; could be picked up now.
- **[decision]** — blocked on a user/product/design decision (a gate).
- **[resource]** — blocked on an external resource (hardware, account, network policy).

This is a review surface, not a plan; promote items to the test plan / refinements as you choose.

---

## 1. Transport faithfulness

- **[doable]** MLS key-distribution over the wire. The faithful path models the verifying-key registry
  + lineage membership as agreed state; the real MLS key-distribution is not yet run end-to-end over
  iroh. This is the next faithful step now that signature+standing travel the wire.
- **[decision]** Threshold revoke-**authority** over the wire. MD-G5's revoke is a MAC; the real
  "who-may-revoke" is a k-of-n signature bundle (`revocation-authority.md`). Needs the policy-state +
  co-sign-vs-vote ordering decision before implementing.
- **[doable]** Real beacon over transport. Freshness (E2.16) is green-model; a signed tip beacon over
  live iroh-gossip + an AR-4-style leak measurement of the beacon is the faithful follow-on.
- **[doable]** MD-G5 single-node both-transitions. Gossip de-dupes identical payloads, so one survivor
  showed "retain pre-revoke" and another "refuse post-revoke"; per-round-varying branch bytes would
  show both on one node.

## 2. Scale / load

- **[resource]** Relay capacity ceilings. E0/E1/E5 understated absolute throughput because generators
  were co-located on 2-vCPU boxes; the true accept-rate / NIC / handshake-CPU walls need a larger
  generator fleet. **[resource]** reconnect-storm handshake-CPU driver (RELAY-LAB §4).
- **[doable]** E6 steady-state goodput under shaping + a bandwidth-cap condition (only delay/loss +
  establishment were measured). **[doable]** E7 churn *storm* (many peers reassigned at once) + a
  precise window-length-vs-TTL distribution (only a single binary fail-window was shown).
- **[doable]** Fold cost under churn (recomputed from scratch each receive; incremental cost
  unmeasured). MLS-tree O(N) commit cost (AR-5) → the **broadcast tier MUST disable the embedded
  ratchet-tree** (ship out-of-band) — implementation edge.

## 3. Governance & races

- **[decision]** Vote-accumulation (pattern B) under churn/partition: vote expiry, retraction, stale-
  vote rejection (`revocation-authority.md`).
- **[decision]** Removing an admin / last-admin / quorum-breaking removals — needs a floor rule so a
  group can't be bricked or captured. (Shadows T12 last-device-revocation.)
- **[doable]** Policy-change races reduce to the reconcile contradiction; confirm the hard-stop covers
  concurrent policy edits.
- **[doable]** Revoke ordering vs a racing legitimate branch (revoke + a fresh op from the target in
  the same round) — untested.

## 4. Recovery

- **[decision/resource]** Recovery (T12/E3.3) is the largest residual risk: last-device revocation,
  stolen-device-same-lineage-1-sig, new-device-for-same-DID via external commit + broker snapshot.
  Needs a recovery design pass.

## 5. Social-layer gates

- **[decision]** **S3 — quiet membership** (be in a group without it exposing your other edges; the
  inside-adversary problem). Needs **design gate G5** (`social-layer.md` §75–77) before any test.
- **[decision]** **S4 — multi-identity, no forced linkage** (distinct lineages with no provable
  linkage vs one lineage with scoped facets). Needs G5.
- **[decision]** **T8 — V3 republish UX control.** Structural V3 is done; the human-layer control that
  stops someone pasting private content into a public republish is a UX/product decision.
- **[doable]** S2 realism: the re-identification uses a synthetic fingerprint; a real deployment needs
  a calibrated anonymity-set estimator over actual graph features. The distance metric itself (who is
  at distance d, who decides) is unmodelled.

## 6. Metadata / privacy

- **[doable]** **Metadata leak under failed adversarial ops** (task #10) — quantify what an observer
  learns when an op is rejected (forged history, failed non-member join), and the leak-vs-immune-signal
  duality + the silent/blackhole response dial (`failed-op-response.md`). Extends AR-4.
- **[resource]** AR-4 quantification: an actual relay-side timing+volume capture to measure the leak
  (vs the current characterization).

## 7. Freshness / liveness

- **[doable]** Beacon-rate vs battery/metadata calibration; threshold constants (15s / 6h placeholders)
  need measurement on the real fabric.
- **[doable]** Fresh-but-wrong partition: a clique keeps each other "fresh" while collectively behind
  the true tip. Freshness proves liveness, not global currency; the reconcile hard-stop on reconnect is
  what catches it — confirm end-to-end.
- **[decision]** Freshness threshold for membership ops specifically (a removal may warrant a stricter
  bar than ordinary content).

## 8. Hard-gated corpus items (external resources — not attempted this batch)

- **[resource]** **T10** — Bluesky republish (app-password + egress allowlist).
- **[resource]** **T13** — an iOS build host.
- **[resource]** **E0-NAT hole-punch** — public ingress on 3343/3478 (currently closed).
- **[resource]** **E4 — LVS frontend** (`ipvsadm`).
- **[resource]** **E8 / E9** — the `meer` binary (superpeer bridge + confidentiality tiers).

---

## Triage summary

- **Doable now (no blocker):** MLS-keys-over-wire, real beacon transport, MD-G5 single-node, E6
  goodput/bandwidth, E7 storm, fold-churn cost, policy-change race, revoke-ordering race, S2 estimator,
  metadata-leak-on-failure spike, freshness calibration, fresh-but-wrong confirmation.
- **Blocked on decision (gates):** threshold revoke-authority, vote-accumulation, admin-floor rule,
  recovery design, S3, S4, T8, membership freshness bar.
- **Blocked on resource:** relay capacity fleet, reconnect-storm driver, AR-4 capture, T10, T13,
  E0-NAT, E4, E8/E9.
