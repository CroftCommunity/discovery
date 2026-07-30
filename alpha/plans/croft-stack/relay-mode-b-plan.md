# Plan: iroh-relay mode B — QUIC address-discovery (direct handoff)

date: 2026-07-29 · component of [06-iroh-relay.md](06-iroh-relay.md). Supersedes the mode-A default.

**Status: LIVE (2026-07-29) — B1 implemented + converged; idempotent (`changed=0`).** Owner chose
topology **B1** (shared box). The relay serves its own TLS on TCP 8443 (HTTPS/WSS) + UDP 7824 (QUIC
address-discovery), reusing Caddy's Let's Encrypt cert via `cert_mode=Reloading` + an `iroh-relay-certsync`
copy; firewall opens 8443/tcp + 7824/udp; `relay.croft.ing:8443` serves `200` with a valid cert.
Phase 0 (config surface) verified empirically on the box. **Acceptance gate MET (2026-07-30):** a
two-node `relay-loadtest` run (box generator ↔ NAT'd desktop responder, both on `relay.croft.ing:8443`
/QUIC 7824) classified **5/5 connections `direct` (relay=0)**, RTT ~64–94 ms — direct handoff via the
relay's address-discovery confirmed. Commits `f01d794` + `242ae02` (croft-stack); test session log
`croft-stack/sessions/2026-07-30-relay-two-node-direct-handoff.md`.

## Problem Statement

The relay was deployed in **mode A** (plain HTTP on `127.0.0.1:8440`, Caddy terminates TLS on
`relay.croft.ing:443` and reverse-proxies; `enable_quic_addr_discovery` OFF). That was a
**misunderstanding**: with address-discovery off, nodes get less help establishing *direct* P2P
connections, so more traffic is **tunnelled through the relay** instead of going direct.

We want the opposite: **direct handoff.** iroh does ~90% hole-punch / ~95% of data over direct
connections, with the relay as the ~5% fallback (FACTCHECK, CONFIRMED). QUIC address-discovery is the
mechanism that enables that direct path. So **mode B (addr-discovery ON) is the target.**

## Reasoning

- Direct connections are the point of iroh; the relay should be a fallback, not the default tunnel.
- Address-discovery (a QUIC/STUN-like probe) is what lets a node learn its reflexive address and
  hole-punch. It requires the relay to run a **QUIC endpoint (UDP) with its own TLS**.
- This is a dev/test relay for our own dogfooding — but dogfooding direct handoff is exactly what we
  need to exercise, so mode B is required for the test to be meaningful.

## Verified Assumptions (iroh-relay v1.0.0 config schema — from source)

From `n0-computer/iroh` `iroh-relay/src/main.rs` @ `v1.0.0` (WebFetch, 2026-07-29):

- Top-level: `enable_relay`, `http_bind_addr`, `enable_quic_addr_discovery`, `enable_metrics`,
  `metrics_bind_addr`, `key_cache_capacity`, `access`.
- `[tls]`: `https_bind_addr`, `quic_bind_addr`, `hostname` (list), `cert_mode`
  (`Manual` | `LetsEncrypt` | `Reloading` [feature-gated]), `cert_dir`, `manual_cert_path`,
  `manual_key_path`, `prod_tls`, `contact`, `dangerous_http_only`.
- **Architecture (the crux):** with `[tls]` configured, the relay serves its **HTTP relay services on
  `https_bind_addr`** (default 443); the plain `http_bind_addr` remains "primarily for captive
  portal." QUIC (`quic_bind_addr`) and relay functions operate independently via their bindings.
- QUIC server default port is **UDP 7824**; `--dev` ignores all TLS fields (plain HTTP only).
- DNS prerequisite **met (2026-07-29):** `relay.croft.ing` A/AAAA now point at the box
  (`15.204.81.133` / `2604:2dc0:222::431`); the `*.croft.ing` wildcard parking no longer shadows it.

## The design crux: relay TLS vs Caddy on :443

With `[tls]` on, the relay wants to terminate TLS itself (on `https_bind_addr`, default 443) — but
**Caddy already owns TCP :443** on the shared box (canary + account by SNI). Two processes can't bind
:443. Candidate approaches:

- **(B1) Shared box, relay on a non-standard HTTPS port + UDP QUIC (recommended for a dev relay).**
  Relay `https_bind_addr = [::]:8443`, `quic_bind_addr = [::]:7824`, `cert_mode = Manual` pointing at
  the Let's Encrypt cert **Caddy already obtains for `relay.croft.ing`** (reuse Caddy's stored cert
  files; `Reloading` if the binary has it, to pick up renewals). Caddy stops *proxying* relay traffic
  but is kept as the **cert issuer/renewer** for the name. Clients use a `RelayUrl` with the explicit
  port (`https://relay.croft.ing:8443`, QUIC 7824). Firewall opens **TCP 8443 + UDP 7824**.
  - Pro: no new box; reuses Caddy's ACME; canary/account untouched on 443.
  - Con: non-standard relay HTTPS port (fine — we control the dogfooding client config); relay reads
    Caddy's cert files (perms: `relay` user needs read on Caddy's cert dir, or a copy hook).
- **(B2) Relay on its own box/IP** — owns 443 + 7824 + its own ACME cleanly, no Caddy contention.
  The "when-to-split-under-load" path; heavier. Defer unless B1's non-standard port is unacceptable.

**Owner decision to confirm:** B1 (shared box, port 8443 + UDP 7824, reuse Caddy cert) vs B2 (own
box). Recommendation: **B1** for the dev/test relay.

## Concurrency Map
All sequential (single host, single unit). Phase 0 discovery must precede the config change.

## Phases

### Phase 0 — Discovery (empirical, on the live dev relay; safe — canary/account are isolated)
Resolve the remaining unknowns by configuring + restarting the relay and observing (revert to mode A
if anything misbehaves):
- **D1:** With `[tls]` set, does the relay still answer the relay protocol usably, and on which ports
  (confirm `https_bind_addr` serves the relay; `http_bind_addr` role)? Success: relay reachable over
  HTTPS on the chosen port; `curl -k https://relay.croft.ing:8443/` responds.
- **D2:** Can `cert_mode = Manual` read Caddy's LE cert for `relay.croft.ing`? Locate Caddy's cert
  path (`/var/lib/caddy/.local/share/caddy/certificates/…/relay.croft.ing.crt|.key`); confirm the
  `relay` user can read them (perms/ACL or a copy-on-renew hook). Success: relay starts with a valid
  cert; no self-signed warning for the hostname.
- **D3:** Does the binary support `cert_mode = "Reloading"` (feature-gated) for renewal pickup, or do
  we need a renew hook that copies + restarts? Success: a renewal path that doesn't need manual steps.
- **D4:** Confirm `quic_bind_addr` UDP 7824 + `enable_quic_addr_discovery = true` starts and a client
  can use it for address discovery. Disposition: throwaway config experiments; promote the working
  config into the deploy file.

### Phase 1 — Firewall: allow the QUIC (UDP) + relay HTTPS (TCP) ports
The nftables role today allows **TCP** 22/80/443 only. Add:
- `allowed_udp_ports: [7824]` (new — the role needs a UDP accept rule; today it's TCP-only).
- `allowed_tcp_ports: + 8443` (if B1). TDD the render/nftables assertion (a `render.bats`/firewall
  test that the UDP rule + the new TCP port are present). Keep default-drop otherwise.

### Phase 2 — Relay config + cert wiring (deploy files + Ansible)
- `relay/deploy/relay.toml`: add `[tls]` (`hostname`, `https_bind_addr = [::]:8443`,
  `quic_bind_addr = [::]:7824`, `cert_mode`, cert paths, `prod_tls = true`) + `enable_quic_addr_discovery = true`.
- Cert wiring: point the relay at Caddy's `relay.croft.ing` cert (Phase-0 D2/D3 outcome — Reloading or
  a renew-copy hook). Ensure the `relay` user can read the cert.
- Caddy: keep a `relay.croft.ing` mechanism that obtains/renews the cert (cert-only; stop proxying the
  relay data path). Update `relay/deploy/relay.croft.ing.caddy` accordingly.
- Update the relay `bats` (mode-B assertions: `[tls]` present, `enable_quic_addr_discovery = true`,
  quic/https bind ports, cert paths; the vhost is cert-only). Governed unit unchanged (already 512M/200%).

### Phase 3 — Converge + verify direct handoff
- `ansible-playbook site.yml` (idempotent). Relay active on TCP 8443 (HTTPS) + UDP 7824 (QUIC).
- Verify: relay reachable by name over HTTPS on the port; QUIC endpoint reachable; a client using this
  relay establishes a **direct** connection (address-discovery working) rather than relaying — the
  mode-B acceptance gate. (Needs a two-node test — the relay-loadtest/lab crate or two iroh nodes.)

## Documentation Impact
- `06-iroh-relay.md` — mode B is the target; link this plan; record the B1 design + rationale.
- `relay/deploy/relay.toml`, `.caddy`, `ansible/group_vars/all.yml` — flip the mode-A comments to
  "mode A was interim; mode B (this plan) is target," then to LIVE once converged.
- `croft-stack/reviews/2026-07-29-stack-review.md` §C (firewall now also UDP 7824 + TCP 8443) and §G (relay in mode B:
  direct handoff; QUIC endpoint; verification of a direct connection).
- `ROADMAP_TODO.md` — mode-B build item until landed.

## Risks & cautions
- **Firewall posture:** opening a UDP port + a non-standard TCP port widens the surface. Both are
  needed for mode B; keep default-drop otherwise; the relay is governed.
- **Cert coupling:** the relay depends on Caddy's cert files — a renewal that doesn't propagate would
  break the relay's TLS. Solve in Phase 0 (Reloading or a hook), verify renewal.
- **Non-standard port (B1):** clients must be told the relay URL includes `:8443`. Fine for our
  controlled dogfooding; document it.
- **Reversible:** revert to mode A (drop `[tls]`, close the ports) if Phase 0/3 misbehaves.
