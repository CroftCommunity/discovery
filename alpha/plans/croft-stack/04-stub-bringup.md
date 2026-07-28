# Phase 4 — Ansible converge on the clean box (idempotent bring-up)

← [03-governance-telemetry.md](03-governance-telemetry.md) · [roadmap](README.md) · next →
[05-dns-tls.md](05-dns-tls.md)

**Status:** **playbook AUTHORED + locally validated** (`croft-stack/ansible/`, `d1a349f`; 7 roles,
code/data separated; `--syntax-check` clean, `ansible ping` OK, `--check` runs through modulo the
expected check-mode nftables-service artifact). **The converge (box mutation) is GATED on owner go** —
not yet run. Session: `croft-stack/sessions/2026-07-28-phase-4-ansible.md`. · **Depends-on:** Phase 2 (box
adopted + reimaged clean) + Phase 3 (governance stanzas in the generator; telemetry client built) ·
**Gate-out:** a second `ansible-playbook` run reports `changed=0`; the `canary` tenant is `active`,
governed, and serving `/healthz`; SSH is key-only; no lockout.

**In-box mechanism = Ansible** (Open decision 10; `bootstrap.sh` dropped, kept only as the spec below).

---

## Problem

Bring the freshly-reimaged bare Debian 13 box (`ssh croft-vps`, `debian`, passwordless sudo) to a
serving baseline **idempotently**, proving the infrastructure with the payload held constant (the
`canary` tenant) before any real service. Do it without locking ourselves out.

## Box baseline (verified 2026-07-28)

`debian@vps-e9655dff`, Debian 13 (trixie), 6 vCPU / 11 GiB / 94 GB free. Listening: only `:22` +
systemd-resolved. `/opt` empty; `caddy`/`node` absent; `python3` + systemd present; passwordless sudo.

## Approach — an Ansible playbook in `croft-stack/ansible/`

```
croft-stack/ansible/
  ansible.cfg          # inventory=inventory.ini; ssh via ~/.ssh/config (Host croft-vps)
  inventory.ini        # [croft]  croft-vps
  site.yml             # play: hosts=croft, become=true, roles in the order below
  roles/
    base/              # apt cache, base packages, unattended-upgrades
    firewall/          # nftables default-drop; ALLOW 22 first, then 80/443
    ssh_hardening/     # PermitRootLogin no; PasswordAuthentication no (key confirmed); validate before reload
    users/             # per-tenant + <name>-api system users (from services/*.toml)
    deploy_user/       # the forced-command deploy target
    caddy/             # apt caddy; base Caddyfile + generated vhosts
    units/             # copy generated/ systemd units (governed) → daemon-reload → enable --now
    telemetry/         # deploy the Python cgroup reader + its timer (Phase 3)
```

The **payload is the `canary` tenant** (`services/canary.toml` → `make generate` → its unit + vhost),
not the kit's old example tenants — write `canary.toml` and regenerate before the `units` role.

## Steps (sketch — fill on authoring)
1. Precondition: install Ansible locally (`brew install ansible` / `pipx install ansible-core`) — a
   dev-toolchain item (fold→1 doc).
2. Write `services/canary.toml`; `make generate`; confirm the canary unit + vhost render with the
   Phase-3 governance stanzas.
3. Author the roles (order above). TDD-ish: each role idempotent; `--check` clean on a second pass.
4. `ansible-playbook site.yml` (first converge) — **watch the SSH-affecting roles closely** (see
   safety). Then re-run; **gate on `changed=0`.**
5. Verify: `systemctl` shows `canary` active + governed (limits/accounting present); `curl localhost:8100/healthz`
   → `ok`; the telemetry client reports `canary`'s live cgroup usage.

## Safety — do not lock out (the load-bearing caution)
- **Firewall before SSH-harden, 22 first:** the nftables ruleset must allow `22` in the same apply that
  sets default-drop; never drop before allow. Keep the live SSH session's conntrack intact.
- **sshd validate before reload:** change `sshd_config` via a handler that runs `sshd -t` (validate)
  and only reloads on success; keep the current session open and confirm a NEW `ssh croft-vps` works
  before trusting it. Password auth off is safe — key auth is already confirmed.
- **Converge is a box mutation** — gated on explicit owner go; logged in `sessions/`.

## Reasoning
- **Ansible over bash** (Open decision 10): idempotence is structural (modules), not hand-rolled guards
  — directly addressing the bash-idempotency risk. `changed=0` on re-run is a real, checkable gate.
- **Canary-first**: proves bootstrap/TLS/supervision/governance/telemetry with an off-the-shelf-simple
  payload before any real or net-new service (relay next, Phase 6).
- **Generator still produces the units**; Ansible converges them. Two Python-friendly layers, no bash
  bring-up.

## Risks & cautions
- Lockout (above) — the top risk; mitigations baked into role ordering + validate handlers.
- Debian 13 vs the kit's 12 assumption (hold→2): confirm packages/modules (caddy, nftables) on trixie.
- The destroy→restore fire-drill stays **deferred** (backups paused).

## Validation
Second `ansible-playbook` run `changed=0`; `canary` active+governed; `/healthz` 200; telemetry live;
a fresh `ssh croft-vps` still connects (no lockout).

## References
`croft-stack/bootstrap/bootstrap.sh` (SPEC only), `scripts/render.py`, `stub/` (canary), `services/`;
Open decision 10; `03-governance-telemetry.md` (governance stanzas + telemetry client).
