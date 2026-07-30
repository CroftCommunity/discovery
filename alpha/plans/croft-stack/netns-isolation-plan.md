# Plan: network-namespace service isolation (reusable pattern; relay first)

date: 2026-07-30 · a reusable isolation pattern for the estate, first applied to the iroh relay.
Related: [06-iroh-relay.md](06-iroh-relay.md), [relay-mode-b-plan.md](relay-mode-b-plan.md).
Tracked: `ROADMAP_TODO` E76.

**Status: Phase 0 DONE — PASS (2026-07-30); full 3-pass plan below; Phases 1–3 build pending.** Owner
wants the *full* network-namespace isolation (not the cheap systemd-directive tightening), built as a
**reusable pattern** we can carry to another box or keep on this one. Phase 0 cleared the make-or-break
gate: a relay behind DNAT in a namespace still delivers **5/5 direct** connections *and* is isolated
from the host (can't reach the broker). Findings: `croft-stack/sessions/2026-07-30-netns-phase0.md`.

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
  loopback or other services. Defense-in-depth beyond systemd's `IPAddressDeny` (the cheap version we
  deliberately skipped in favour of this).
- **Reusable pattern:** parameterise it (ns name, subnet, exposed ports) so any external-facing unit
  can adopt it. One Ansible role, many consumers.
- **Portability:** the service's entire network surface becomes (a) its listen ports inside the ns and
  (b) the host's DNAT rules. Moving it = recreate the ns+DNAT elsewhere, or drop the ns and bind
  directly on a dedicated box (the "own box" endpoint, relay-mode-b **B2**).
- **Alternatives rejected:** (a) systemd `IPAddressDeny`/sandbox directives — cheaper but weaker (the
  service still shares the host stack; owner explicitly wanted the full version). (b) relay on its own
  box (B2) — the clean endpoint, but heavier (a second VPS) and Phase 0 showed it isn't necessary yet.

## The design (full version)

```
   external client ──▶ box public IP :8443/tcp, :7824/udp
                          │  (host netns)
                          ▼  nftables DNAT (dest-only; source preserved)
                    veth-host 10.77.0.1/30 ─┅veth pair┅─ veth-ns 10.77.0.2/30
                          │                                   │ (relayns netns)
                          │  masquerade (egress) + deny→host  ▼
                          ▼                             iroh-relay binds
                    box's real uplink ◀── relay egress    10.77.0.2:8443/7824
                                                          (cannot see host lo/other svcs)
```

- **Named netns** `relayns` + a veth pair (`veth-host` in the host ns ↔ `veth-ns` in `relayns`), a
  private /30 (`10.77.0.0/30`), default route in the ns via `veth-host`.
- **Ingress:** nftables `DNAT` box-public `:8443/tcp` + `:7824/udp` → `10.77.0.2`. **No inbound SNAT**
  so the relay observes the *client's real source address* (critical for address-discovery — Phase 0).
- **Egress:** `masquerade` the relay's outbound so it can reach external peers; a forward-chain rule
  **denies `relayns → 127.0.0.0/8` + the host's own addresses + RFC1918** (public internet yes, host/
  estate no).
- **The unit** runs in the ns via systemd `NetworkNamespacePath=/run/netns/relayns` (ordered after the
  ns-setup unit). Filesystem unchanged (separate mount ns) — cert reads from `/etc/iroh-relay/certs`
  still work; **cgroup unchanged** (the unit stays in `system.slice`, so the telemetry poller still
  sees it).
- **Reusable role** `netns_service`: inputs `{ ns, subnet, tcp_ports[], udp_ports[], deny_egress_cidrs[] }`
  → creates the ns + veth + DNAT/masquerade/egress-deny + a drop-in setting `NetworkNamespacePath` on
  the consumer unit. The relay is consumer #1; future external services reuse it.

## Additional isolation surfaces (beyond the network namespace)

The netns isolates the *network*. To keep the relay as cut off from the host as possible, layer the
**other namespaces + syscall filtering** on the same unit — all systemd directives, no extra plumbing
(they compose with `NetworkNamespacePath`). The relay needs almost nothing from the host, so it can be
near-totally sandboxed:

- **PID namespace** — `PrivatePIDs=yes` (or `ProtectProc=invisible` + `ProcSubset=pid`): can't see or
  signal other host processes.
- **Mount namespace (tighten)** — already `ProtectSystem=strict`/`PrivateTmp`/`ProtectHome`; go further
  with `TemporaryFileSystem=/` + `BindReadOnlyPaths=/opt/iroh-relay/current /etc/iroh-relay` +
  `BindPaths=/var/lib/iroh-relay` + `PrivateMounts=yes` — the relay sees only its binary, cert, state.
- **User namespace** — `PrivateUsers=yes`: uid remapped to an unprivileged namespaced id. **Caveat
  (verify):** certs are `0640 root:relay`; the mapping must still allow the read (see Open Questions).
- **IPC namespace** — `PrivateIPC=yes` + `RemoveIPC=yes`: no shared SysV/POSIX IPC.
- **UTS namespace** — `ProtectHostname=yes`.
- **cgroup namespace** — `ProtectControlGroups=yes` (already) hides the host cgroup tree from the relay.
  **Telemetry is unaffected:** the poller reads `/sys/fs/cgroup/system.slice/iroh-relay.service/…` from
  the **host** side, not from inside the relay — a cgroup ns on the relay doesn't hide the unit from it.
- **Syscall + misc surface** — `SystemCallFilter=@system-service`, `SystemCallArchitectures=native`,
  `RestrictNamespaces=yes` (no nest-escape), `LockPersonality`, `RestrictSUIDSGID`, `RestrictRealtime`,
  and (verify — Rust/rustls has no JIT, so likely fine) `MemoryDenyWriteExecute=yes`.
  `RestrictAddressFamilies=AF_INET AF_INET6` (drop `AF_UNIX` if the relay needs no unix socket).

Net: a compromised relay is confined to its own net/pid/mount/user/ipc/uts view + a minimal FS +
a restricted syscall set + (via the netns) no route to the host — while telemetry still governs and
observes it from the host. Built in **Phase 1b**.

## Verified Assumptions

- **Relay behind DNAT keeps direct handoff.** Phase-0 D2: two-node `relay-loadtest` through a DNAT'd
  test relay → **5/5 `direct`, relay=0** (session log `2026-07-30-netns-phase0.md`). dest-NAT preserves
  the client source, so address-discovery observed the real reflexive addr.
- **Forced relay tunneling works and must be preserved.** On the live mode-B relay, `relay-loadtest
  generate --mode passthrough` (clears IP transports) → **`ESTABLISHED=5 relay=5 direct=0`** (verified
  2026-07-30). The netns must not break the *relayed* fallback either — a Phase-2 acceptance check.
- **Telemetry is accurate and unaffected in principle by a net namespace.** The poller's sampled
  `memory_current` for `iroh-relay.service` (`17108992`) **exact-matched** the live cgroup
  `MemoryCurrent` (`17108992`), `pids_current=3` == `TasksCurrent=3` (verified 2026-07-30). It reads
  `/sys/fs/cgroup/system.slice/<unit>.service/…`; `NetworkNamespacePath` changes only the *network*
  namespace, not the cgroup, so the unit stays in `system.slice` and remains sampled. **Confirm live in
  Phase 2** (the unit is still sampled after the drop-in). Schema: `ts, unit, memory_current,
  memory_peak, cpu_usage_usec, pids_current, io_rbytes, io_wbytes`.
- **iroh-relay v1.0.0 config is unchanged by the netns** — it binds `https_bind_addr`/`quic_bind_addr`
  (mode-B config) inside whatever net namespace it runs in; Phase 0 ran it via `ip netns exec` and it
  bound + served (HTTP 200) + reloaded the cert. (`relay-mode-b-plan.md` has the schema, source-verified.)
- **Additive nftables + managed-reload teardown is SSH-safe.** Phase 0 added a separate `ip rtnat`
  table + two `inet filter forward` accepts, and tore down via `nft -f /etc/nftables.conf` (atomic
  reload of the managed ruleset) — SSH survived, live services untouched.
- **Cert reuse holds inside the ns.** `certsync` writes `/etc/iroh-relay/certs` on the host FS; the
  netns'd unit shares the mount namespace, so the files are visible (Phase 0 reused the live cert).
- **Not yet verified (see Open Questions):** systemd `NetworkNamespacePath` behaves identically to
  Phase-0's `ip netns exec` launch; IPv6 ingress through the ns (Phase 0 was IPv4-only).

## Documentation Impact
- `ansible/roles/netns_service/` (new) — role README describing inputs + the pattern (Phase 1).
- `ansible/roles/relay/tasks/main.yml`, `relay/deploy/iroh-relay.service.d/netns.conf` (drop-in),
  `ansible/group_vars/all.yml`, `ansible/roles/firewall/templates/nftables.conf.j2` (nat/egress) — Phase 1.
- `relay/deploy/iroh-relay.service.d/hardening.conf` (drop-in: other namespaces + syscall filter) — Phase 1b.
- `06-iroh-relay.md`, `relay-mode-b-plan.md` (topology note), `croft-stack/reviews/…` §C/§G — Phase 3.
- `ROADMAP_TODO.md` E76 — status transitions.
- New session log `croft-stack/sessions/<date>-netns-relay.md` (the build + live acceptance).

## Concurrency Map
**All phases sequential:** Phase 0 (done) → Phase 1 (build+wire the netns role to the relay) → Phase 1b
(deepen the unit sandbox: other namespaces + syscall filter) → Phase 2 (live acceptance) → Phase 3
(document). Each phase reads what the prior wrote and mutates the same box (netns + nftables + the
running relay unit) — overlapping write-sets, so **no parallelism**.

## Phases

### Phase 0 — Discovery — ✅ DONE, PASS (2026-07-30)
All gates cleared (throwaway netns + a separate test relay on alt ports 18443/18824; live relay
untouched; `croft-stack/sessions/2026-07-30-netns-phase0.md`):
- **D1 (reachable behind DNAT):** ns relay served HTTP **200** internally (veth) and externally
  (`relay.croft.ing:18443` via prerouting DNAT). ✓
- **D2 (the gate — direct handoff survives NAT):** two-node `relay-loadtest` → **5/5 `direct`, relay=0**,
  RTT ~63–89 ms. ✓ — **B2 not forced.**
- **D3 (isolation holds):** from inside the ns, broker `127.0.0.1:8001` refused, `10.88.0.1:8001` timed
  out. ✓
- **D4 (persistence):** mechanism works (via `systemd-run`); deterministic boot recreation is Phase 1.
- **Insight:** prerouting DNAT covers external clients; **output-chain DNAT** is needed *only* if an
  on-box process must reach the relay by public name (a test artifact; production clients are external).

### Phase 1 — Build the `netns_service` role and adopt it for the relay
**Goal:** the relay runs inside its own network namespace — public ports DNAT-exposed, egress denied to
the host/estate, reboot-persistent — via a reusable, parameterised Ansible role.
**Changes:**
- [ ] `ansible/roles/netns_service/` — defaults (`ns`, `subnet`, `tcp_ports[]`, `udp_ports[]`,
  `deny_egress_cidrs[]`), tasks, and templates:
  - [ ] `netns-setup@.service` (systemd oneshot, `RemainAfterExit=yes`): `ip netns add`, veth pair,
        addrs, ns default route, `ip_forward` — idempotent (guard on `ip netns list`), enabled so it
        recreates the ns on boot **before** the consumer.
  - [ ] a generated nftables fragment: a separate `ip <ns>nat` table (prerouting DNAT of the ports →
        ns IP, **no inbound SNAT**; postrouting masquerade for the subnet), plus `inet filter forward`
        accepts for the veth and an **egress-deny** (`deny_egress_cidrs`) in the forward chain. Must
        coexist with the managed `inet filter` (default-drop intact; never `flush` it).
  - [ ] a consumer drop-in template setting `NetworkNamespacePath=/run/netns/<ns>` +
        `After=/Requires=netns-setup@<ns>.service`.
- [ ] `ansible/roles/relay/tasks/main.yml` — use `netns_service` (`relayns`, `8443/tcp`+`7824/udp`,
  deny host+RFC1918) and install the drop-in on `iroh-relay.service`.
- [ ] `ansible/group_vars/all.yml` — relay netns params; **remove the plain `8443`/`7824` input accepts**
  (traffic is now DNAT'd → forwarded, not host-input) once the DNAT path is confirmed.
- [ ] `ansible/roles/firewall/templates/nftables.conf.j2` — host the generated nat/egress rules (or a
  role-owned include) additively.
**Call chain:** `site.yml` → `relay` role → includes `netns_service` role → renders
`netns-setup@relayns.service` + the `ip relaynsnat` table + the `iroh-relay.service` drop-in →
`iroh-relay.service` starts **inside `relayns`** and binds `8443`/`7824` there; DNAT forwards the box's
public ports to it.
**Wiring test:** after converge — `ssh croft-vps 'systemctl is-active netns-setup@relayns iroh-relay;
ip netns exec relayns ss -ltnp | grep 8443'` shows both active + the relay bound *in the ns*; and
`curl -sS -o/dev/null -w "%{http_code}" https://relay.croft.ing:8443/` → `200` (DNAT ingress live).
This is RED before the phase (no netns), GREEN after.
**Depends on:** Phase 0 (design validated).
**Read-set:** `ansible/roles/relay/tasks/main.yml`, `ansible/roles/firewall/templates/nftables.conf.j2`,
`ansible/group_vars/all.yml`, `relay/deploy/iroh-relay.service`, `ansible/site.yml`.
**Write-set:** `ansible/roles/netns_service/**` (new), `ansible/roles/relay/tasks/main.yml`,
`relay/deploy/iroh-relay.service.d/netns.conf` (new drop-in), `ansible/group_vars/all.yml`,
`ansible/roles/firewall/templates/nftables.conf.j2`, `relay/tests/test_relay_deploy.bats` (+netns asserts).
**Shared-state contract:** the converge mutates box network namespaces (creates `relayns`), the
nftables ruleset (adds `ip relaynsnat` + forward accepts/egress-deny + drops the old 8443/7824 input
accepts), and restarts `iroh-relay` into the ns. **Never `flush` the managed `inet filter`** (SSH/
established stay up); adds are to a separate nat table + additive forward rules. No git/port/env
mutation beyond this. Reboot ordering: `netns-setup@relayns` `Before` `iroh-relay`.
**Re-entry verification:** N/A (sequential).
**Risks:** a bad forward/nat rule could black-hole the relay (revert = drop the drop-in + DNAT, restart
relay on the host); reboot ordering wrong → relay starts before the ns exists (mitigate with
`Requires=`+`After=`); IPv6 (see Open Questions).
**Done when:**
1. **Behavioral:** the relay runs inside `relayns`, is reachable at `relay.croft.ing:8443` (`200`), and
   from inside `relayns` **cannot** reach `127.0.0.1:8001`; the full converge is idempotent.
2. **Verification:** `ansible-playbook site.yml` then a second run → `changed=0`; the wiring-test
   command above returns active+bound+`200`; `ssh croft-vps 'sudo ip netns exec relayns curl -m4 -sS
   127.0.0.1:8001/healthz || echo ISOLATED'` → `ISOLATED`; `bats relay/tests/test_relay_deploy.bats`.
**Validation:** **Broad** (network surgery + a live service). bats over rendered artifacts + live
converge + the isolation/ingress checks; keep a one-command revert ready.

### Phase 1b — Deepen the unit sandbox (other namespaces + syscall filter)
**Goal:** confine the relay to the minimum host surface — layer PID/mount/user/IPC/UTS namespaces +
syscall filtering onto the unit (composes with the netns from Phase 1).
**Changes:**
- [ ] `relay/deploy/iroh-relay.service.d/hardening.conf` (drop-in): `ProtectProc=invisible` +
  `ProcSubset=pid` (PID), `PrivateIPC`+`RemoveIPC`, `ProtectHostname`, tightened mount
  (`TemporaryFileSystem=/` + `BindReadOnlyPaths=/opt/iroh-relay/current /etc/iroh-relay` +
  `BindPaths=/var/lib/iroh-relay` + `PrivateMounts=yes`), `SystemCallFilter=@system-service`,
  `SystemCallArchitectures=native`, `RestrictNamespaces`, `LockPersonality`, `RestrictSUIDSGID`,
  `RestrictRealtime`; and — gated on the caveats (Open Questions) — `PrivateUsers` and
  `MemoryDenyWriteExecute`. Apply **incrementally**, confirming the relay stays active+serving after each.
- [ ] `relay/tests/test_relay_deploy.bats` — assert the hardening directives are present.
**Call chain:** `iroh-relay.service` (hardened, in `relayns`) → still binds `8443`/`7824`, reads its
cert, writes its state — under the tighter sandbox.
**Wiring test:** after converge, the relay is **still `active` and serves `200`** with all directives
applied (a too-tight directive — e.g. a mount set hiding the cert, or `PrivateUsers` breaking the cert
read — shows as a failed/crashlooping unit), and `systemd-analyze security iroh-relay.service` exposure
drops materially vs the pre-hardening baseline.
**Depends on:** Phase 1 (netns in place).
**Read-set:** `relay/deploy/iroh-relay.service`, the netns drop-in.
**Write-set:** `relay/deploy/iroh-relay.service.d/hardening.conf` (new), `ansible/roles/relay/tasks/main.yml`
(install the drop-in), `relay/tests/test_relay_deploy.bats`.
**Shared-state contract:** restarts the relay under a tighter sandbox; no other box mutation. Revert =
remove the drop-in.
**Risks:** an over-tight directive breaks the relay (cert read under `PrivateUsers`; mount set hides a
needed path; `MemoryDenyWriteExecute` if a dep needs W^X). Add directives one group at a time; watch
the unit stay active + serving; `systemd-analyze security` guides.
**Done when:**
1. **Behavioral:** the relay runs with the deepened sandbox, still `active`, still serves
   `relay.croft.ing:8443` (`200`), still passes the two-node direct + forced-relay tests; the
   `systemd-analyze security` exposure score is materially lower than baseline.
2. **Verification:** `ssh croft-vps 'systemctl is-active iroh-relay; systemd-analyze security
   iroh-relay.service | tail -3'` + the Phase-2 acceptance run + `bats relay/tests/test_relay_deploy.bats`.
**Validation:** **Moderate→Broad** — each directive verified to keep the relay serving; the security
score confirms the isolation gain.

### Phase 2 — Live acceptance (function + telemetry survive the namespace)
**Goal:** prove isolation didn't break the relay's job (both **direct** and **relayed**) or telemetry.
**Changes:** none (verification phase); record results in the session log.
**Call chain:** the two-node `relay-loadtest` (desktop ↔ box) against the netns'd relay, exercising
both path modes; the telemetry poller against the relay's cgroup.
**Wiring test:** (a) `--mode matchmaking` → direct path; (b) `--mode passthrough` → relayed path — both
through `relay.croft.ing:8443`/QUIC 7824 (now DNAT → the ns relay).
**Depends on:** Phase 1.
**Read-set:** none (runtime verification); `/var/lib/telemetry/samples.db` (read).
**Write-set:** `croft-stack/sessions/<date>-netns-relay.md` (results).
**Shared-state contract:** spawns throwaway relay-loadtest processes (desktop + box); no persistent
box mutation. Box tooling cleaned after (as in the Phase-0 / two-node runs).
**Re-entry verification:** N/A.
**Risks:** if `direct` regresses to `relay` (or forced-relay fails) behind the *systemd*
`NetworkNamespacePath` launch (vs Phase-0's `ip netns exec`), that's a real finding → adjust Phase 1 or
fall back to B2.
**Done when:**
1. **Behavioral:** behind the netns, matchmaking yields **direct>0** (expect 5/5, as Phase 0) **and**
   passthrough yields **relay>0** (expect 5/5, forced-relay preserved); the telemetry poller's next
   `iroh-relay.service` sample's `memory_current` still **matches** the live cgroup; the ns still can't
   reach `127.0.0.1:8001`.
2. **Verification:** `relay-loadtest generate --mode matchmaking …` → `direct=5`; `… --mode passthrough
   …` → `relay=5`; `sqlite3 …/samples.db` latest `iroh-relay` `memory_current` == `systemctl show
   iroh-relay -p MemoryCurrent`; the isolation curl → `ISOLATED`.
**Validation:** **Broad** — two-node cross-network runs (both path modes) + telemetry cross-check +
isolation probe.

### Phase 3 — Document + generalise
**Goal:** capture the pattern and its applicability; make the docs current.
**Changes:**
- [ ] `ansible/roles/netns_service/README.md` — inputs, the DNAT/egress model, when to use it.
- [ ] `06-iroh-relay.md`, `relay-mode-b-plan.md` (topology note), the current stack review §C/§G — the
  relay now runs in `relayns`; the pattern suits **external, non-relay** services outright (a cache/
  index behind DNAT has no NAT-helper tension), and the relay is a *validated* consumer.
- [ ] `ROADMAP_TODO` E76 → done.
**Call chain / Wiring test:** n/a (docs).
**Read-set:** the changed code from Phases 1–2. **Write-set:** the doc files above.
**Shared-state contract:** none (docs only).
**Done when:** (1) the docs describe the live netns'd relay + the reusable role accurately; (2) `grep`
finds no stale "relay shares the host network stack" claims.
**Validation:** **Narrow** — doc review + a consistency grep.

## Open Questions
- [RECOMMENDED: PHASE-GATED (Phase 1)] **IPv6 through the namespace.** Phase 0 tested **IPv4 only**
  (`10.88.0.0/30` veth + `ip` nat). The live relay binds `[::]` (dual-stack) and `relay.croft.ing` has
  **AAAA → the box**. An IPv4-only netns would drop IPv6 clients. Decide: add a dual-stack veth +
  `ip6` DNAT/masquerade (full parity), or accept **IPv4-only** for the dev relay (and drop/ignore the
  AAAA path). *Rationale: real functional gap the IPv4 Phase-0 didn't cover; cheap to decide now,
  annoying to discover after adoption.*
- [RECOMMENDED: PHASE-GATED (Phase 1)] **`NetworkNamespacePath` vs `ip netns exec`.** Phase 0 launched
  the relay via `ip netns exec` + `systemd-run`; production uses the systemd `NetworkNamespacePath`
  directive. Confirm equivalence (binds in the ns; cert FS visible; cgroup unchanged) at Phase-1
  converge before declaring done. *Rationale: the directive is the production launch path; validate it,
  don't assume it matches the manual exec.*
- [RECOMMENDED: ADVISORY] **Telemetry `io_rbytes`/`io_wbytes` are `NULL`** for the relay unit (memory/
  cpu/pids are accurate). Confirm whether io accounting is simply absent for the unit (expected) or the
  `io.stat` parse returns `None` (a poller gap). *Rationale: surfaced during the telemetry check;
  tangential to netns but worth a look while we're in the telemetry code; not blocking.*
- [RECOMMENDED: ADVISORY] **Output-chain DNAT** — include it in the role only if an on-box process ever
  needs the relay by public name (none today). *Rationale: keep the rule surface minimal.*
- [RECOMMENDED: PHASE-GATED (Phase 1b)] **`PrivateUsers=yes` vs the cert read.** The relay's cert is
  `0640 root:relay`; user-namespacing remaps uids, so the read may break. Decide: relax cert perms
  (e.g. `0644` — the cert is public anyway; the **key** is the sensitive one), or map the user, or
  skip `PrivateUsers`. *Rationale: `PrivateUsers` is a big isolation win but the most likely directive
  to break the relay; settle it before enabling.*
- [RECOMMENDED: PHASE-GATED (Phase 1b)] **`MemoryDenyWriteExecute=yes` with iroh/rustls.** Rust has no
  JIT, so W^X should be fine, but a dependency (e.g. a crypto backend) could need it. Verify the relay
  stays active with it on; drop it if not. *Rationale: cheap hardening if it holds; don't assume.*
- [RECOMMENDED: ADVISORY] **Drop `AF_UNIX`** from `RestrictAddressFamilies` if the relay opens no unix
  socket. *Rationale: narrow the address-family surface; confirm the relay doesn't need it first.*

## Risks & cautions
- **QUIC/NAT vs address-discovery** — was the make-or-break; **RESOLVED (Phase 0): 5/5 direct behind
  DNAT.** No longer a blocker.
- **Ingress DNAT must not SNAT** (preserve client source) or address-discovery sees the wrong address.
- **Never `flush` the managed `inet filter`** during the nat additions — SSH/established would drop.
  Adds go in a separate `ip <ns>nat` table + additive forward rules; teardown/revert reloads the
  managed ruleset.
- **Reboot persistence** — `netns-setup@<ns>` must come up (and the veth/DNAT exist) before the unit.
- **Reversible** — drop the drop-in + the nat table, restart the relay on the host → back to bare mode B.

## Review Log
- **2026-07-30 — Pass 1 (base) + Phase 0 executed.** Base plan (problem/reasoning/design) written; Phase 0
  run on the box — PASS on D1–D4 (5/5 direct behind DNAT; isolation holds; B2 not forced). Findings
  folded into Verified Assumptions.
- **2026-07-30 — Pass 1+2+3 buildout.** Added Verified Assumptions (incl. forced-relay `passthrough`
  5/5 relayed, and telemetry `memory_current` exact-match to live cgroup); full per-phase rigor
  (Read/Write-sets, shared-state, wiring tests, two-tier Done-when, Validation); Concurrency Map (all
  sequential, one reason); Documentation Impact; Open Questions. **Pass-2 gap caught:** Phase 0 was
  **IPv4-only** but the live relay is dual-stack with an AAAA record → new PHASE-GATED question on IPv6
  through the ns. Also flagged `NetworkNamespacePath`-vs-`ip netns exec` equivalence and the telemetry
  `io_*` NULL. Restructured Phase 1 to **build the role and wire it to the relay in the same phase**
  (avoids dead code); Phase 2 is the live acceptance re-checking **both** direct and forced-relay plus
  telemetry accuracy.
- **2026-07-30 — Added Phase 1b (deepen the unit sandbox).** Owner asked what *other* namespace
  isolations apply. Added an "Additional isolation surfaces" design section (PID/mount/user/IPC/UTS/
  cgroup namespaces + syscall filtering, all systemd directives composing with `NetworkNamespacePath`)
  and a **Phase 1b** to apply them incrementally to the relay unit, with `systemd-analyze security` as
  the exposure gauge. Noted telemetry is unaffected (the poller reads the unit's cgroup from the host,
  not from inside the relay). New PHASE-GATED questions: `PrivateUsers` vs the cert read, and
  `MemoryDenyWriteExecute` with iroh/rustls.
