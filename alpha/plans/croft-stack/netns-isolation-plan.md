# Plan: network-namespace service isolation (reusable pattern; relay first)

date: 2026-07-30 · a reusable isolation pattern for the estate, first applied to the iroh relay.
Related: [06-iroh-relay.md](06-iroh-relay.md), [relay-mode-b-plan.md](relay-mode-b-plan.md).

**Status: Phase 0 DONE — PASS (2026-07-30); implementation spec below, build pending.** Owner wants the
*full* network-namespace isolation (not the cheap systemd-directive tightening), built as a **reusable
pattern** we can carry to another box or keep on this one. **Phase 0 cleared the make-or-break gate:**
a relay behind DNAT in a namespace still delivers **5/5 direct** connections *and* is isolated from the
host (can't reach the broker) — so B2 (own box) is **not** forced. Findings:
`croft-stack/sessions/2026-07-30-netns-phase0.md`.

## Problem Statement

A service like the relay serves **external** clients and should not be able to reach anything else on
the host — not the broker on `127.0.0.1:8001`, not other units, not the host's other interfaces. Today
the relay is sandboxed by systemd (mount/dev/caps/cgroup) but still shares the **host network stack**,
so a compromised relay could reach localhost services. We want each such service in its **own network
namespace**: it sees only its own interface, its public ports are the only ingress, and its egress
can't touch the host. As a bonus, this makes the service's network coupling explicit and **easy to
move off** to its own box.

## Reasoning

- **Isolation:** a net namespace is the strong form — the service literally cannot address the host's
  loopback or other services. Defense-in-depth beyond systemd's `IPAddressDeny` (which is the cheap
  version we deliberately skipped in favour of this).
- **Reusable pattern:** parameterise it (ns name, subnet, exposed ports) so any external-facing unit
  can adopt it. One Ansible role, many consumers.
- **Portability:** the service's entire network surface becomes (a) its listen ports inside the ns and
  (b) the host's DNAT rules. Moving it = recreate the ns+DNAT elsewhere, or drop the ns and bind
  directly on a dedicated box (the "own box" endpoint).

## The design (full version)

```
   external client ──▶ box public IP :8443/tcp, :7824/udp
                          │  (host netns)
                          ▼  nftables DNAT
                    veth-host 10.77.0.1/30 ─┅veth pair┅─ veth-ns 10.77.0.2/30
                          │                                   │ (relayns netns)
                          │  masquerade (egress)              ▼
                          ▼                             iroh-relay binds
                    box's real uplink ◀── relay egress    10.77.0.2:8443/7824
                                                          (cannot see host lo/other svcs)
```

- **Named netns** `relayns` with a veth pair (`veth-host` in the host ns ↔ `veth-ns` in `relayns`),
  a private /30 (e.g. `10.77.0.0/30`), default route in the ns via `veth-host`.
- **Ingress:** nftables `DNAT` box-public `:8443/tcp` + `:7824/udp` → `10.77.0.2`. **No inbound SNAT**
  so the relay still sees the *client's real source address* (critical for address-discovery).
- **Egress:** `masquerade` the relay's outbound so it can reach external peers; a forward-chain rule
  **denies `relayns → 127.0.0.0/8` + the host's own addresses + RFC1918** (the relay can talk to the
  public internet but not back into the host/estate).
- **The unit** runs in the ns via systemd `NetworkNamespacePath=/run/netns/relayns` (ordered after the
  ns-setup unit). Filesystem is unchanged (separate mount ns) — cert reads from `/etc/iroh-relay/certs`
  still work.
- **Reusable role** `netns_service`: inputs `{ ns, subnet, tcp_ports[], udp_ports[] }` → creates the
  ns + veth + DNAT/masquerade + a drop-in setting `NetworkNamespacePath` on the consumer unit. The
  relay is consumer #1; future external services reuse it.

## The load-bearing risk (Phase-0 gate): NAT vs the relay's own job — RESOLVED (2026-07-30)

**Phase 0 settled this: a relay behind DNAT still yields 5/5 direct connections** (dest-NAT preserves
the client source, so address-discovery observes the true reflexive address). The concern below stood
until tested; it no longer blocks the shared-box netns for the relay. Kept for the reasoning record:


A relay is a **NAT-traversal helper**; putting it *behind* NAT (the host's DNAT/masquerade) is in
tension with that role. QUIC address-discovery works by the relay **observing the client's public
address** and by clients reaching the relay at a **stable public address**. Behind DNAT:

- Inbound must preserve the client source (no SNAT on ingress) so the relay observes the real address.
- The relay must advertise the **box's public** `relay.croft.ing:8443/7824`, not its `10.77.0.2` — it
  derives this from `hostname`/config, so likely fine, but **must be verified**.
- QUIC (UDP) through DNAT + conntrack must not mangle the flows address-discovery depends on.

If any of these break address-discovery, the honest conclusion is that **full network isolation for a
relay wants its own box/IP (relay-mode-b "B2")**, not a NAT'd netns on the shared box — while the netns
*pattern* remains valuable for non-relay external services (a cache, an index) that aren't themselves
NAT helpers. Phase 0 decides this before we commit.

## Concurrency Map
All sequential. Phase 0 (empirical, on the box) gates the build.

## Phases

### Phase 0 — Discovery — ✅ DONE, PASS (2026-07-30)
All gates cleared on the box (throwaway netns + a separate test relay on alt ports 18443/18824; live
relay untouched; `croft-stack/sessions/2026-07-30-netns-phase0.md`):
- **D1 (reachable behind DNAT):** ns relay served HTTP **200** internally (veth) and externally
  (`relay.croft.ing:18443` via prerouting DNAT). ✓
- **D2 (the gate — direct handoff survives NAT):** two-node `relay-loadtest` through the DNAT'd relay →
  **5/5 `direct`, relay=0**, RTT ~63–89 ms. dest-NAT preserves the client source, so address-discovery
  observed the desktop's real reflexive addr and hole-punch succeeded. ✓ — **B2 not forced.**
- **D3 (isolation holds):** from inside the ns, the broker (`127.0.0.1:8001`) is refused and
  `10.88.0.1:8001` times out — a compromised relay can't reach the host. ✓
- **D4 (persistence):** mechanism works (via `systemd-run`); deterministic boot recreation is Phase-1.
- **Insight:** prerouting DNAT covers external clients; an **output-chain DNAT** is needed *only* if an
  **on-box** process must reach the relay by public name (a test artifact here — real clients are
  external, so production needs prerouting only).

### Phase 1 — Reusable `netns_service` Ansible role  ⏳ NEXT
A parameterised role, inputs `{ ns, subnet (/30), tcp_ports[], udp_ports[], deny_egress_cidrs[] }`:
- **ns + veth setup as a systemd oneshot** `netns-setup@<ns>.service` (not a converge-time-only script,
  so it survives reboot): `ip netns add`, veth pair, addrs, ns default route, `ip_forward`. Idempotent
  (guard on `ip netns list`). `RemainAfterExit=yes`; ordered `Before=` the consumer unit.
- **nftables:** extend the firewall role's template with a generated `ip <ns>nat` table — prerouting
  DNAT of `tcp_ports`/`udp_ports` → the ns IP (no inbound SNAT: preserve client source), postrouting
  masquerade for the subnet, and forward accepts for the veth. Egress-deny (`deny_egress_cidrs`, e.g.
  `127.0.0.0/8 ::1 10/8 172.16/12 192.168/16 169.254/16`) in the forward chain so the ns can't reach
  the host/estate. Keep default-drop otherwise. (Output DNAT only if an on-box client needs the relay.)
- **Consumer wiring:** a systemd drop-in setting `NetworkNamespacePath=/run/netns/<ns>` on the service
  unit + `After=/Requires=` the setup unit.
- **TDD:** bats over the rendered nftables (DNAT/masquerade/egress-deny present; `inet filter` default-
  drop intact; no `flush` of the managed table) + the setup unit (idempotent create). Converge idempotent.

### Phase 2 — Adopt for the relay
Apply `netns_service{ ns: relayns, ports: 8443/tcp + 7824/udp, deny_egress: host+RFC1918 }` to the
relay: DNAT the box's public 8443/7824 → the ns relay (replacing the plain input accepts), add the
`NetworkNamespacePath` drop-in to `iroh-relay.service`. `certsync` still writes `/etc/iroh-relay/certs`
(FS is shared — the mount ns is unaffected). Converge; re-run the two-node acceptance test live
(expect 5/5 direct, as Phase 0); confirm the relay can't reach `127.0.0.1:8001`.

### Phase 3 — Document + generalise
Record the pattern; note it suits **external, non-relay** services outright (a cache/index behind DNAT
has no NAT-helper tension), and that the relay is a *validated* consumer (Phase 0). Update
`06-iroh-relay.md`, the stack review §C/§G, and the netns role README.

## Documentation Impact
- New role `ansible/roles/netns_service/`; relay unit drop-in; nftables changes (DNAT/masquerade).
- `06-iroh-relay.md`, `relay-mode-b-plan.md` (topology note), the stack review §C/§G.
- `ROADMAP_TODO.md` item (the build).

## Risks & cautions
- **QUIC/NAT vs address-discovery** (Phase-0 D2) — was the make-or-break; **RESOLVED (2026-07-30): 5/5
  direct behind DNAT.** No longer a blocker.
- **Ingress DNAT must not SNAT** (preserve client source) or address-discovery sees the wrong address.
- **Reboot persistence** — the ns + veth + DNAT must come up deterministically before the unit.
- **Reversible** — drop the drop-in + DNAT to fall back to bare mode B.
