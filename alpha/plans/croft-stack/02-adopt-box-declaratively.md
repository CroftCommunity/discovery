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

## Reconciliation (spike vs kit) — noted, resolved in Phase 4/7

The spike is **throwaway** and will be **replaced by the production Rust auth broker (Phase 7)**. The
lifetime measurement is **not precious** (we already have spec-cited TTLs — Open decision 9 resolved);
revisit it later when things are stable. So the plan of record: let the spike run for now; when Phase 4
brings the box up via the kit and Phase 7 lands the real broker, **tear down the spike** (its
`BOX-CHANGELOG` teardown) and let the kit own Caddy + the `account.croft.ing` unit. Key collisions to
handle then: Caddy is already apt-installed with a hand-edited `Caddyfile` (kit uses generated vhosts);
`/opt/auth-helper` + `authhelper` user exist (kit would create its own).

## Reasoning

- **Read, not import** — you cannot cleanly `import` a hand-placed OVH order into the order resource;
  `data.ovh_vps` is the honest way to bring an adopted box under visibility without pretending terraform
  made it. The order path stays as the reproduce-next-box recipe, money-gated off.
- **DNS manual** — owner preference; DNS automation is high-blast-radius for low benefit at this scale.
  Phase 2 needs **no DNS change** (no services deploy here); per-service A/AAAA records are added by
  hand at each service's deploy phase (Phase 5+). `account.croft.ing` + `stellin.app` already point at
  the box from the spike.
- **R2/Cloudflare out** — there is no bucket to manage until backups are implemented (paused).
- **`--plan` not `--apply`** — Phase 2 mutates nothing; and bash idempotency being a known risk
  (fold→4), reading the plan is exactly how we catch a guard that would collide with the box's existing
  hand-config before anything runs.

## Risks & cautions

- **`bootstrap.sh` will collide with the box's hand-config** (Caddy + Caddyfile already present) — this
  is why Phase 4 is gated on the fold→4 in-box-mechanism decision and the spike teardown.
- **Debian 13 vs 12** — confirm the kit runs clean on trixie, or pin the reproduce recipe to 12.
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
