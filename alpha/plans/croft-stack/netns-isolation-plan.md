# Plan: network-namespace service isolation (reusable pattern; relay first)

date: 2026-07-30 · a reusable isolation pattern for the estate, first applied to the iroh relay.
Related: [06-iroh-relay.md](06-iroh-relay.md), [relay-mode-b-plan.md](relay-mode-b-plan.md).

**Status: PLANNED — comprehensive design; not yet executed.** Owner wants the *full* network-namespace
isolation (not the cheap systemd-directive tightening), built as a **reusable pattern** we can carry to
another box or keep on this one.

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

## The load-bearing risk (Phase-0 gate): NAT vs the relay's own job

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

### Phase 0 — Discovery (empirical; the go/no-go for the relay specifically)
- **D1:** Create `relayns` + veth + DNAT/masquerade by hand; run the relay in it; confirm it binds and
  is reachable at `relay.croft.ing:8443` (TCP) from outside. Success: HTTPS 200 as today.
- **D2 (the gate):** With a two-node test (see the mode-B acceptance gate), confirm **address-discovery
  still yields DIRECT connections** with the relay behind DNAT — i.e. NAT didn't break the relay's job.
  Success: `direct>0` (same as bare mode B). Failure → relay isolation = B2 (own box); keep the netns
  pattern for non-relay services only.
- **D3:** Confirm the relay in the ns **cannot** reach `127.0.0.1:8001` (broker) or other host services
  (the isolation actually holds). Success: connection refused/unreachable from inside the ns.
- **D4:** systemd `NetworkNamespacePath` ordering + persistence across reboot (ns recreated on boot
  before the unit). Disposition: throwaway hand-setup; promote the working recipe into the role.

### Phase 1 — Reusable `netns_service` Ansible role
Parameterised ns+veth+DNAT/masquerade+egress-deny + a systemd drop-in. TDD the rendered nftables
(DNAT/masquerade/egress-deny present; default-drop preserved) + the ns-setup unit. Idempotent.

### Phase 2 — Adopt for the relay
Wire the relay unit to `NetworkNamespacePath=/run/netns/relayns`; move its 8443/7824 exposure from the
plain firewall accepts to the DNAT path; converge; re-run the mode-B acceptance test (D2) live.

### Phase 3 — Document + generalise
Record the pattern; note which service types suit it (external, non-NAT-helper) vs which want their own
box; update `06-iroh-relay.md` + the stack review's firewall/relay sections.

## Documentation Impact
- New role `ansible/roles/netns_service/`; relay unit drop-in; nftables changes (DNAT/masquerade).
- `06-iroh-relay.md`, `relay-mode-b-plan.md` (topology note), the stack review §C/§G.
- `ROADMAP_TODO.md` item (the build).

## Risks & cautions
- **QUIC/NAT vs address-discovery** (Phase-0 D2) — the make-or-break; a relay behind NAT may defeat its
  own purpose. Decide empirically before building the role for the relay.
- **Ingress DNAT must not SNAT** (preserve client source) or address-discovery sees the wrong address.
- **Reboot persistence** — the ns + veth + DNAT must come up deterministically before the unit.
- **Reversible** — drop the drop-in + DNAT to fall back to bare mode B.
