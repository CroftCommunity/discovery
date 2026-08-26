# Tenant platform plan — model relay/broker as portable tenants

**Status:** DRAFT (Pass 1, 2026-07-30). Tracks `ROADMAP_TODO` E79. Follows the completed hardening work
(`service-hardening-plan.md`) — reuses its identity role, hardening baseline, and cert-copy model.

## Problem Statement

`canary` is defined declaratively (a manifest → `render.py` → hardened/governed/observed unit + Caddy
vhost). The **relay** and **broker** are hand-wired Ansible roles. That means each new or moved workload
is bespoke work, and there's no single shape that a workload can be relocated to another box or scaled
in. Graduate the tenant model from "simple stub apps" to "any workload," so relay/broker (and future
services) are manifests, inheriting hardening + governance + identity + cert-delivery uniformly, and
become relocatable — the E79 direction the owner raised during the hardening work.

## Reasoning

The tenant model already carries the expensive parts (hardening deep set via `${sandbox}`, cgroup
governance, StateDirectory, a `reverse_proxy` vhost, per-manifest overrides). What's missing is
**schema expressiveness** for what relay/broker need beyond a stub. The cert-**copy**-to-a-path model
(hardening C2) is what makes a tenant relocatable: a tenant declares "deliver my cert to this path"
rather than depending on co-location with Caddy's store. So this plan is mostly *extending the manifest
schema + generator*, not new infrastructure.

**Broker first, relay second.** The broker is closest to the current shape (localhost HTTP port +
`reverse_proxy` vhost + state-dir keys), so it validates the schema additions (custom env, bare/binary
ExecStart, InaccessiblePaths) at low risk. The relay is the hard one (netns, DNAT'd UDP+TCP, cert
delivery to tmpfs, certsync dep, `cert_mode=Reloading`) and validates the netns + non-HTTP + cert
extensions — and is exactly what makes "split the relay to its own box" (mode-B B2) fall out.

**Build stays role-side.** The broker is `cargo build`-ed; the relay is a pinned prebuilt binary. The
tenant model defines the *runtime* (unit + vhost + governance + hardening + identity + cert); build/
provisioning stays in a role (or a build step). A tenant manifest references the built binary path.

## Broker delta (scoped 2026-07-30) — the schema extensions Phase 1 adds

Comparing `broker/deploy/croft-broker.service.j2` to the tenant template, the broker needs:
1. **`[env]` table** → quoted `Environment=` lines (BROKER_LISTEN/DATA_DIR/ORIGIN/SCOPE/ALLOWED_*).
   (Quoting is mandatory — the unquoted-space bug truncated BROKER_SCOPE; see hardening Phase 3.)
2. **Exec override** — the stub exec is `--data-dir/--listen` flag-style; the broker is a bare binary
   configured by env. Add an `exec_start` (or `bin` + `config=env`) manifest field.
3. **`inaccessible_paths`** — broker hides other services' state (currently in the hardening role's
   per-unit spec; becomes a manifest field so a tenant declares it).
4. Everything else already fits: localhost port + `reverse_proxy` vhost, state-dir keys (keys are just
   0600 files in StateDirectory — no new concept), `[limits]`, the deep-set sandbox, identity.

**Non-goals for the broker migration:** moving the `cargo build` into the tenant model (stays role-side);
changing broker key management (keys stay self-generated in the state dir).

## Relay delta (Phase 3 — larger; scoped later)

netns membership (the `netns_service` role becomes manifest-driven), DNAT'd UDP+TCP ports (not a
localhost HTTP port — the vhost is cert-only, not reverse_proxy), cert delivery to a tmpfs path +
`certsync` dependency, `cert_mode=Reloading`. This is the schema's stretch case; design after the broker
proves the env/exec/inaccessible extensions.

## Phases

### Phase 0: Discovery — DONE (2026-07-30, inline above)
Broker + relay deltas scoped against the current schema; tenant vhost confirmed as `reverse_proxy`
(fits broker). Sequencing decided (broker → relay). Build stays role-side.

### Phase 1: Extend the schema + model the broker as a tenant (generate-and-diff proof first)
- [ ] Extend `render.py` + the tenant template: `[env]` (quoted), `exec_start`/`bin` override,
  `inaccessible_paths`. Keep all defaults so existing tenants (canary) render unchanged.
- [ ] Write `services/broker.toml` expressing the broker (name, fqdn account.croft.ing, port 8001,
  bin, env, inaccessible_paths, limits).
- [ ] **Gate (no box change):** render and `diff` the generated `croft-broker.service` against the
  current committed unit — must be directive-equivalent (order-insensitive). Prove the manifest
  reproduces the hand-wired unit before touching the box.
- [ ] Migrate the broker role: keep the build/install tasks; replace the hand-templated unit + vhost
  with the generated ones (from `generated/`). Converge; broker stays active + `/jwks` 200 + exposure
  1.5 + `transition:generic` scope intact; `changed=0`.
- [ ] bats: `render.bats`-style assertions that broker.toml renders the env (quoted) + inaccessible_paths.

### Phase 2: (optional) canary/existing-tenant regression
- [ ] Confirm the schema additions leave canary + the other manifests byte-identical (defaults inert).

### Phase 3: Model the relay as a tenant (the stretch case)
- [ ] Extend the schema for netns + DNAT ports + cert-delivery-to-path + Reloading; make
  `netns_service` manifest-driven. Generate-and-diff vs the current relay unit. Migrate + verify
  (5/5 direct still holds, TLS in-netns, reboot-safe).

## Documentation Impact
- `HARDENING-BASELINE.md` — note tenants (now incl. broker) carry the deep set via the template.
- Role READMEs (broker, later relay) — "unit + vhost now generated from `services/<name>.toml`".
- `ROADMAP_TODO` E79 — progress/'done' as phases land.

## Open Questions
- **Q1 (non-blocking):** `exec_start` free-form override vs a structured `{bin, config: env|flags}`.
  Recommendation: structured (`bin` + `config_style`) — keeps manifests declarative and validatable.
- **Q2 (non-blocking):** does `inaccessible_paths` live on the manifest or stay in the hardening role's
  per-unit map? Recommendation: manifest (a tenant declares its own isolation), hardening role keeps
  it only for the non-tenant units until they migrate.
- **Q3 (defer to Phase 3):** how far to push netns into the manifest vs keeping `netns_service` as a
  role the manifest opts into.

## Review Log
- 2026-07-30: Pass 1. Broker/relay deltas scoped inline (Phase 0 folded in). Broker-first sequencing.
