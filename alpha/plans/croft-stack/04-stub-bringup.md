# Phase 4 — `bootstrap.sh --apply` on the stub (idempotent bring-up)

← [03-governance-telemetry.md](03-governance-telemetry.md) · [roadmap](README.md) · next →
[05-dns-tls.md](05-dns-tls.md)

**Status:** SCAFFOLD (fill on arrival) · **Depends-on:** Phase 3 (governance defaults in the
generator) · **Gate-out:** a second `--apply` is a genuine all-`SKIP` no-op; all units `active` and
governed; the contract stub serving.

---

## Problem

The whole kit rests on `bootstrap.sh` being idempotent and step-guarded, but **that has never been
exercised against a real box.** This phase establishes it — with caution — on the validated stub, so
first go-live proves the *infrastructure* with the binary held constant.

## Approach

Run `bootstrap.sh --apply` on the box (as root) to bring up base packages, hardening, firewall, Caddy,
per-tenant users, the deploy user, and the generated units for the **stub tenant** only. Then re-run to
prove idempotency. The stub honors `CONTRACT.md`, so it stands in for real binaries (RUN-15).

## Steps (sketch — fill on arrival)
1. `--apply`: base packages; unattended-upgrades; SSH hardening (no root login, no password auth);
   nftables default-drop (22/80/443); Caddy from apt; per-tenant + `<name>-api` users; `deploy` user
   (forced-command target); install generated units → `/etc/systemd/`, vhosts → `/etc/caddy/conf.d/`;
   `daemon-reload`; `enable --now`.
2. **Read what each step actually did** — do not assume the guards held.
3. Re-run `--apply`; confirm every step reports `SKIP` (the idempotency claim, established for the
   first time on real hardware).
4. Confirm the stub unit is `active`, governed (limits/accounting present), and serving `/healthz`.

## TODO (decide on arrival)
- [ ] Q4 (stub tenant name/fqdn) from Phase 0.
- [ ] Capture the first-`--apply` output as the record of what the box now is.

## Risks & cautions
- **Idempotency is unproven** — this is the highest-risk infra step. If a second `--apply` mutates
  anything, stop and fix the guard before proceeding; a non-idempotent bootstrap poisons every later
  phase.
- Destroy→restore fire-drill is **deferred** (needs R2 / backups, which are paused) — note the gap.
- SSH hardening can lock you out; confirm key-based access works before disabling password auth.

## Validation
Second `--apply` = all-`SKIP`; `systemctl` shows the stub active+governed; `/healthz` (localhost) 200.

## References
`kit/bootstrap/bootstrap.sh`, `kit/stub/`, `RUN-15-SUMMARY.md` (stub + fire-drill).
