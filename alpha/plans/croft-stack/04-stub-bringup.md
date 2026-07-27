# Phase 4 — Ansible converge on a clean box (idempotent bring-up)

← [03-governance-telemetry.md](03-governance-telemetry.md) · [roadmap](README.md) · next →
[05-dns-tls.md](05-dns-tls.md)

**Status:** SCAFFOLD (fill on arrival) · **Depends-on:** Phase 2 (VPS read + reinstall plan; spike
learning persisted) + Phase 3 (governance/telemetry defaults) · **Gate-out:** a second Ansible run is a
clean no-op (`changed=0`); all units `active` and governed; the `canary` tenant serving.

**Reframed (Open decision 10):** the in-box bring-up is **Ansible**, not `bootstrap.sh` (dropped —
bash idempotency not a fit). `bootstrap.sh --plan` is the *spec/checklist* the playbook must satisfy.

---

## Problem

Bring a **freshly reinstalled** box (Phase 2 decided a clean reinstall over reconciling the spike) to a
serving baseline, **idempotently** — proven by a second converge changing nothing. First go-live proves
the infrastructure with the payload held constant (the `canary` tenant).

## Approach

Author an **Ansible playbook** (Python-ecosystem, idempotent modules — `apt`, `user`, `copy`/`template`,
`systemd`, `community.general.nftables`/templated ruleset) that converges the box to the baseline the
old `bootstrap.sh --plan` describes. The `render.py` generator produces the systemd units + Caddy
vhosts; Ansible drops them and enables. Test idempotence (a second run reports `changed=0`) and,
optionally, with `molecule` in a container.

## Steps (sketch — fill on arrival)
1. Clean OS reinstall of the box (Phase 2), OpenTofu read confirms the VPS.
2. Author the playbook from the `bootstrap.sh --plan` spec: base packages; unattended-upgrades; SSH
   hardening (no root, no password); nftables default-drop (22/80/443 + the relay's UDP/QUIC later);
   Caddy; per-tenant + `<name>-api` users; the `deploy` user (forced-command target); install generated
   units + vhosts; `daemon-reload`; `enable --now`. TDD-ish: assert idempotence.
3. Converge; **re-run and confirm `changed=0`** — the idempotency claim, now structurally guaranteed by
   Ansible rather than hand-rolled bash guards.
4. Confirm the `canary` tenant (`canary.croft.ing:8100`) is `active`, governed (limits/accounting), and
   serving `/healthz`.

## TODO (decide on arrival)
- [ ] Playbook structure (roles per concern) + where it lives in `croft-stack` (e.g. `ansible/`).
- [ ] Idempotence test: `changed=0` assertion in CI; `molecule` container run (optional).
- [ ] Debian 13 vs 12 (hold→2): the reinstall target OS; ensure modules/packages match.
- [ ] Does the `deploy-receive.sh` forced-command deploy channel stay bash, or fold into Ansible?
- [ ] Port the old `bootstrap.bats` intent into Ansible-native checks; retire `bootstrap.sh` + its bats.

## Risks & cautions
- Bring up only on a **clean reinstall** — do not converge over the spike's hand-config.
- SSH hardening can lock you out; confirm key access before disabling password auth (Ansible does this
  in one run, so order the tasks carefully).
- The destroy→restore fire-drill stays **deferred** (needs the backup toolchain; backups paused).

## Validation
Second `ansible-playbook` run: `changed=0`; `systemctl` shows `canary` active+governed; `/healthz` 200.

## References
`croft-stack/bootstrap/bootstrap.sh` (as the SPEC only), `scripts/render.py`, `stub/` (the canary);
Open decision 10 (Ansible in-box); roadmap → Resource governance.
