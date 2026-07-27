# Phase 2 — Adopt the OVH box, declaratively

← [01-extract-croft-stack.md](01-extract-croft-stack.md) · [roadmap](README.md) · next →
[03-governance-telemetry.md](03-governance-telemetry.md)

**Status:** ready (reviewed against the real `terraform/` + `BOX-CHANGELOG.md`, 2026-07-27) ·
**Depends-on:** Phase 1 (`croft-stack` repo) · **Gate-out:** `tofu plan` cleanly READS the adopted VPS
(no order placed), the reproduce-next-box recipe (plan_code/region/image) is captured, `bootstrap.sh
--plan` is reviewed with caution, and the box's current (spike) state is inventoried. **No box
mutation.**

---

## Problem

The OVH box is up (manually provisioned, adopted). Left as-is it is not reproducible. We want it under
declarative management — but *simply*, and only where a declarative tool actually helps.

## Scope — corrected after reading the kit

The kit's `terraform/` is **VPS-only** (provider `ovh` only; **no DNS, no R2, no Cloudflare** — an
earlier draft over-scoped those). So the resource layer OpenTofu owns is just the OVH VPS.

```
OpenTofu  →  the OVH VPS resource ONLY  (provider ovh; local state, gitignored)   [resource layer]
Ansible   →  in-box converge: packages · users · nftables · Caddy · units · limits [in-box layer]
generator →  render.py: manifests → the unit/vhost artifacts Ansible converges     [artifacts]
DNS       →  MANUAL, Porkbun by hand    (owner preference — automating DNS is too much risk)
R2/backup →  OUT until backups are implemented (paused/future; no Cloudflare now)
(bootstrap.sh — DROPPED; bash idempotency not a fit. Kept only as the SPEC for the Ansible playbook.)
```

**Adopted box ⇒ read, not import.** The terraform models *order-or-read*, not import: a money-gated
`ovh_order_cart_item` (`place_order`, default **false** — nothing buys by accident) plus a
`data.ovh_vps` read (`vps_service_name`). Since the box was created by hand (outside terraform), there
is no terraform-created order to `import`. So we **READ** the adopted box via `data.ovh_vps`
(`vps_service_name = "vps-e9655dff"`) with `place_order=false`, and separately confirm the
plan_code/region/image that would **reproduce** a next box (`scripts/catalog-vps.sh` + the order path,
left un-applied). OpenTofu thus gives visibility + a reproduce recipe; it does not retroactively "own"
the hand-made box.

## What the box currently has (from `BOX-CHANGELOG.md`)

- `15.204.81.133`, service `vps-e9655dff`, user `debian` (passwordless sudo).
- **OS is Debian 13 (trixie), not Debian 12** — the kit assumes 12 (see fold→2/hold in the queue).
- The auth-helper spike configured it **imperatively by hand**: apt `nodejs`+`caddy`, a `authhelper`
  user, `/opt/auth-helper/` (helper on `127.0.0.1:8001` + secret keys, mode 0600), a hand-edited
  `/etc/caddy/Caddyfile` with `account.croft.ing` + `stellin.app` vhosts, and a daily-refresh timer.
- Full teardown is documented in `BOX-CHANGELOG.md`.

## Steps

1. **Provide OVH API credentials** (env, never in files): `OVH_APPLICATION_KEY` / `OVH_APPLICATION_SECRET`
   / `OVH_CONSUMER_KEY`; set `ovh_endpoint` (subsidiary — `ovh-eu`/`ovh-ca`/`ovh-us`). *(Owner provisions
   the API keys.)*
2. **Read the adopted VPS.** Set `vps_service_name = "vps-e9655dff"`, `place_order = false`; `tofu plan`
   → the `data.ovh_vps` read succeeds and shows the box; `tofu plan` is otherwise clean (no create).
3. **Capture the reproduce recipe.** Resolve a live `plan_code`/region via `scripts/catalog-vps.sh`;
   record the values that would order an equivalent box — but **do not apply** (`place_order` stays
   false). Decide the target OS (Debian 13 to match, or pin 12).
4. **Read `bootstrap.sh --plan` as the SPEC** for the Phase-4 Ansible playbook — it enumerates what the
   bring-up must do (base packages, SSH hardening, nftables, Caddy, per-tenant users, the deploy user,
   unit install). `bootstrap.sh` is **dropped as a runnable** (Open decision 10); never `--apply`.
5. **Confirm the spike learning is persisted** in `discovery/spike/auth-helper/` (FINDINGS, FLOW-SPEC,
   BOX-CHANGELOG, `deploy/`, `helper/`+tests, `pad/`) — it is; the only box-only artifacts are the
   spike's throwaway secret keys (the production Rust broker regenerates its own). This clears the way
   for a clean reinstall (below).

## Reconciliation → clean reinstall (decided)

Rather than delicately merge the spike's hand-config with the kit, **do a clean OS reinstall** and bring
the box up fresh via OpenTofu + Ansible. Rationale: the spike configured the box **imperatively by
hand** (apt nodejs+caddy, `useradd`, a hand-edited `Caddyfile`; `BOX-CHANGELOG.md`), so a merge is
fragile — a reinstall is simpler and gives a known-clean baseline for the idempotent layers.

Precondition (met): **all spike learning is persisted in the repo** (`discovery/spike/auth-helper/`).
The lifetime measurement is **not precious** (spec-cited TTLs already captured — Open decision 9
resolved), so losing the running daily-refresh on reinstall is acceptable; the throwaway on-box keys are
regenerated by the production Rust broker (Phase 7). Sequencing: capture-learning (done) → reinstall →
OpenTofu read/reproduce → Ansible converge (Phase 4) → production broker (Phase 7). The OVH panel
reinstall (or a fresh order via the money-gated terraform) is the reinstall mechanism.

## Reasoning

- **Read, not import** — you cannot cleanly `import` a hand-placed OVH order into the order resource;
  `data.ovh_vps` is the honest way to bring an adopted box under visibility without pretending terraform
  made it. The order path stays as the reproduce-next-box recipe, money-gated off.
- **DNS manual** — owner preference; DNS automation is high-blast-radius for low benefit at this scale.
  Phase 2 needs **no DNS change** (no services deploy here); per-service A/AAAA records are added by
  hand at each service's deploy phase (Phase 5+). `account.croft.ing` + `stellin.app` already point at
  the box from the spike.
- **R2/Cloudflare out** — there is no bucket to manage until backups are implemented (paused).
- **Read, don't run** — Phase 2 mutates nothing. `bootstrap.sh --plan` is read as the *spec* for the
  Ansible playbook (bootstrap dropped, Open decision 10); the in-box converge itself is authored and run
  in Phase 4 against a freshly reinstalled box.

## Risks & cautions

- **Do not bring up over the spike's hand-config** — clean reinstall first (decided above), so the
  idempotent layers start from a known baseline rather than fighting the hand-edited Caddyfile /
  `/opt/auth-helper`.
- **Reinstall loses the running measurement + on-box keys** — acceptable (measurement not precious;
  keys throwaway), but confirm the learning is persisted in the repo before wiping.
- **Debian 13 vs 12** — confirm Ansible + the kit run clean on trixie, or pin the reinstall to 12.
- **OVH API keys are secrets** — env only, never committed; scope them minimally.
- **`place_order` must stay false** — the money gate; never apply the order path in adoption.

## Validation

`tofu plan` reads `vps-e9655dff` cleanly with no create and `place_order=false`; reproduce recipe
(plan_code/region/OS) recorded; `bootstrap.sh --plan` read and the spike-collision points written down.

## References

- `croft-stack/terraform/` (`main.tf` order-or-read + money gate; `variables.tf` owner-decision vars;
  `versions.tf` `ovh` provider, local state), `scripts/catalog-vps.sh`.
- `discovery/spike/auth-helper/BOX-CHANGELOG.md` (the box's current hand-config + teardown).
- Roadmap → Open decision 10 (OpenTofu VPS-only; DNS manual; R2 out) and the fold→4 in-box-mechanism
  followup.
