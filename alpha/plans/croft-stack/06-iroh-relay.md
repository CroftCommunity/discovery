# Phase 6 — iroh relay (first real service, infra shakedown)

← [05-dns-tls.md](05-dns-tls.md) · [roadmap](README.md) · next → [07-auth-helper.md](07-auth-helper.md)

**Status:** **DONE (2026-07-29, `89b8b4b`)** as a **dev/test relay** (owner intent — dogfooding, not
primetime). iroh-relay v1.0.0 (prebuilt musl, pinned+sha256) deployed **mode A** (plain HTTP on
`127.0.0.1:8440`, Caddy TLS-terminates `relay.croft.ing` + reverse-proxies), no Rust build. Governed,
telemetry-sampled, idempotent. Session: `croft-stack/sessions/2026-07-29-phase-6-relay.md`.

**Now LIVE in mode B (QUIC address-discovery / direct handoff), 2026-07-29.** Mode A suppressed direct
handoff (address-discovery off → traffic tunnels the relay instead of hole-punching; iroh is ~90%
hole-punch / ~95% direct), so we moved to mode B (topology **B1**, shared box): the relay terminates
its own TLS on TCP 8443 + UDP 7824 (QUIC), reusing Caddy's LE cert via `cert_mode=Reloading` +
`iroh-relay-certsync`; firewall opens 8443/tcp + 7824/udp; `relay.croft.ing:8443` → `200` valid cert;
converge idempotent. DNS fixed (`relay.croft.ing` → box). Plan + acceptance gate (two-node direct
test, pending): **[relay-mode-b-plan.md](relay-mode-b-plan.md)**. ·
**Depends-on:** Phases 3–5 (governance defaults; box up; DNS/
TLS) · **Gate-out:** the relay runs supervised + governed + observable; the box is proven under a real
long-running service *before* any net-new invention.

---

## Problem

The first real service after the stub should isolate *infra* faults from *application* faults. An
off-the-shelf iroh relay is the ideal shakedown: known behavior, long-running, resource-hungry, and not
on the pads' critical path. It also tests whether the kit contract generalizes to a third-party binary.

## Approach

Deploy the off-the-shelf iroh relay as a governed mini-stack at `relay.croft.ing`. **Scope is
connectivity only** — NAT traversal / "making the ends meet"; peer discovery is via atproto (a PDS
record resolved by Bluesky id, cached by the cache/index server), **not** pkarr/DHT, so the relay
carries no discovery role.

## Steps (sketch — fill on arrival)
1. Open the relay's UDP/QUIC port in nftables — the **documented exception** to 22/80/443-only.
2. DNS + TLS for `relay.croft.ing` (Phase 5).
3. systemd unit with the Phase-3 cgroup limits/accounting.
4. **Contract fit check:** does the iroh relay binary honor `CONTRACT.md` (`--data-dir`, `/healthz`,
   non-root, ports ≥ 1024)? If not, add a thin wrapper or register a contract exception (this is a key
   generality finding — resolves Open decision 12's sub-item).
5. Watch it under real load via the telemetry client (the E5/E6 governance bet); tighten limits from
   observed usage.

## TODO (decide on arrival)
- [ ] Which iroh relay binary/version (cite FACTCHECK for the iroh version; iroh is `1.0.0`).
- [ ] Contract fit vs wrapper vs exception (Open decision 12 sub-item).
- [ ] Whether the relay's `:443`/DERP expectation conflicts with Caddy owning `:443` (relay lab detail).
- [ ] Noisy-neighbor headroom: does a bandwidth-heavy relay co-located on this VPS need its own box
      soon (WHEN-TO-SPLIT trigger)?

## Risks & cautions
- Relay is bandwidth-heavy (relay lab E0: ~186 MiB/s passthrough on 2 vCPU) — the noisy-neighbor case
  governance exists to bound; watch it starve nothing.
- QUIC/UDP firewall exception widens the attack surface beyond 22/80/443 — document it.

## Validation
Relay `active`, governed (limits/accounting), observable in telemetry; a NAT-traversal connection
succeeds through it; box stable under its load.

## References
`alpha/experiments/iroh/` (relay lab: RELAY-LAB-CONCLUSIONS.md, RELAY-PLACEMENT-LAB-SPEC.md, E0/E5/E6);
FACTCHECK for iroh version; roadmap → discovery-via-atproto note.
