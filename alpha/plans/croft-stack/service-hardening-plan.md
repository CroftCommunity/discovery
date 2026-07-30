# Service hardening plan — one least-privilege baseline, test-gated per unit

**Status:** DRAFT (Pass 1+2 + Phase 0 DONE, 2026-07-30). Scope broadened during Phase 0 (user
direction): the baseline is a full **least-privilege profile per component** — identity (uid/gid/group
membership), filesystem (owned paths, modes, owners), TLS certificate handling, capabilities, and
namespaces — realized **deterministically** (systemd `sysusers.d`/`tmpfiles.d` + explicit modes) so the
box reconstructs on any host. Gates come in **two tiers**: positive (the protection is configured and
the service still works) and **negative** (the protection actually blocks — cross-service reads,
out-of-sandbox writes, host routes, etc.). Depends on
`croft-stack/reviews/2026-07-30-service-hardening-review.md`. Tracks `ROADMAP_TODO` E78.

← [netns-isolation-plan.md](netns-isolation-plan.md) · review → `croft-stack/reviews/2026-07-30-service-hardening-review.md`

---

## Problem Statement

`systemd-analyze security` across the deployed estate (2026-07-30):

| unit | exposure | note |
|------|----------|------|
| `iroh-relay` | **1.7 OK** | hardened reference (netns + deep sandbox) |
| `caddy` | **8.8 EXPOSED** | distro unit: `NoNewPrivileges=no`, full `cap_*` set; holds every LE key |
| `croft-broker` | 4.7 OK | base set only; holds ES256 assertion key + AES store key |
| `canary` (tenant template) | 4.7 OK | base set only; template inherited by all future tenants |
| `telemetry-poll` | 4.4 OK | base set only; must retain cross-unit cgroup read |

Four units carry only the base hardening set and lack the deep layer the relay already has. caddy is
nearly unconfined despite being the TLS front door. We want to bring each toward the relay baseline —
**and** capture the hardening as a *defined, reusable baseline* (a contract + a test-gate library + a
shared Ansible fragment) so every current and future unit is measured against one standard, not
per-service ad-hoc checks. The test-gate logic is a first-class deliverable, not scaffolding.

**Scope (broadened during Phase 0).** Hardening is least privilege on *every* axis, not just systemd
sandboxing. The baseline profile per component covers:
1. **Identity** — a dedicated system user + group, `nologin`, no supplementary groups beyond a
   justified minimum, and **deterministic uid/gid** (so a restore/rebuild on another box owns files
   identically). Phase 0 found ids are dynamically allocated (not portable) and caddy carries a
   vestigial `www-data` group.
2. **Filesystem** — the service owns only the paths it needs, at the tightest mode; secrets `0600`/
   `0640`; `ReadWritePaths` scoped exactly; cross-service state mutually unreadable (verified).
3. **Certificates** — a dedicated pass (see below): Caddy is the sole ACME authority; the relay key is
   duplicated to a `root:relay` copy; the admin API is exposed; keys are plaintext at rest.
4. **Capabilities & namespaces** — the relay's proven deep set, per-unit carve-outs from Phase 0.

**Portability is a first-class goal (user direction):** the smaller and more *explicit* the identity /
permission / capability graph, the fewer implicit host dependencies, and the more trivially the estate
rebuilds elsewhere. Declaring the full profile and realizing it deterministically (`sysusers.d` +
`tmpfiles.d` + explicit modes + the shared drop-in) *is* the portability story.

**Certificate exposure (Phase 0 findings, folded in):** (C1) caddy's admin API is on and locally
unauthenticated — any local user reached `:2019/config/`; (C2) the relay's private key exists in two
places (caddy store + `0640 root:relay` copy, identical sha256); (C3) no FDE — keys plaintext at rest on
a VM whose snapshots are a vector. These drive a dedicated certificate phase.

## Reasoning

**Why a shared baseline rather than per-unit drop-ins.** The relay already proved a directive set
(exposure 1.7). Copying it four times would drift. Instead we extract it once into (a) a human-readable
contract, (b) a bats assertion library, and (c) a templated Ansible drop-in with per-unit override vars.
"One policy, per-unit data" — the same shape as the existing cgroup-governance template and the
`netns_service` role. The user explicitly asked to "keep building up our test gate logic as a defined
baseline," so the baseline is the spine, and each service is a thin application of it.

**Why test-gate the hardening (TDD fit).** Hardening is unusually clean for TDD: the assertion *is* the
security property. RED = the directive is absent / the unit is at its current exposure; GREEN = add the
directive, the static assertion passes, and the live exposure drops with the service still active and
functional. "Watch it fail" is literal — run the new bats against the un-hardened unit first.

**Two-tier gate, matching the netns plan.**
- **Static tier** (offline, CI-able, matches the existing bats convention): assert the rendered
  drop-in / template carries the baseline directive set (with the unit's documented carve-outs), and
  Ansible reports `changed=0` on a second converge (idempotency).
- **Live tier** (on the box, recorded in a `sessions/` log, matches the existing converge convention):
  `systemd-analyze security <unit>` ≤ the recorded ceiling; `systemctl show` confirms the directives
  are active; a functional probe passes; the service stays `active`.

**Exposure as a ratchet, not an aspirational absolute.** caddy will never reach 1.7 — it keeps a
capability, broad egress (ACME), and a writable cert store. So the deterministic gate is
**directive-presence** (the contract), and the exposure number is a **regression ratchet**: record the
score actually achieved on the box, then assert the unit never regresses above it. No guessed target
that fails the gate on day one.

**Positive and negative gates (two polarities, orthogonal to static/live).** A positive gate asserts the
protection is present and the service still works. A negative gate asserts the protection *actually
blocks* — the boundary is enforced, not merely declared. `NetworkNamespacePath=` in a unit file is a
positive fact; "the relay process cannot open a socket to the broker" is the negative fact that proves
isolation. Negative gates are inherently live (they need the running sandbox) and adversarial (they try
the forbidden thing and require it to fail). The baseline library carries both; a directive is only
"done" when its negative gate confirms enforcement. This mirrors behavior-not-implementation testing:
we test the security *behavior*, not the config string.

**Least-privilege profile + deterministic realization = portability.** Each component's baseline is a
declared profile (identity, filesystem, certs, caps, namespaces). Identity and directories are realized
with systemd-native, idempotent primitives — `sysusers.d` (fixed uid/gid) and `tmpfiles.d` (dirs with
explicit owner+mode) — rather than ad-hoc `useradd`/`mkdir`. That makes the id/permission graph explicit
and reproducible: the same profile rebuilds the same ownership on any box, and a `/var/lib` restore
owns correctly by uid. Fewer implicit dependencies, simpler portability — exactly the user's ask.

**Why migrate the relay onto the shared baseline (Phase 1b).** The relay is the known-good reference.
Re-rendering its drop-in from the shared template and confirming it stays 1.7 + 5/5 direct + accurately
telemetered is the strongest possible validation that the baseline reproduces the reference exactly —
before we point it at four un-hardened units.

**Alternatives rejected.** (1) Per-unit standalone drop-ins (no shared source) — drifts, defeats the
"defined baseline" ask. (2) A single systemd drop-in symlinked into every `.d/` — can't carry per-unit
carve-outs (caddy's cap, telemetry's cgroup exception), and systemd drop-ins don't parameterize. (3)
Live-only gates (no static bats) — not CI-able, and the existing convention is static bats + converge.

## Verified Assumptions

- Relay `hardening.conf` directive set and that it yields exposure **1.7** — read the file; score from
  this session's live run. `PrivateUsers`+`MemoryDenyWriteExecute` verified live on the relay 2026-07-30
  (relay stays active + serving; userns mapping preserves the cert-read group).
- Exposure scores caddy 8.8 / broker 4.7 / canary 4.7 / telemetry 4.4 — this session's live
  `systemd-analyze security` runs.
- bats convention = **static** artifact assertions (grep the `deploy/` files), shared `tests/helpers.bash`
  at kit root, per-component `<component>/tests/*.bats`; live validation happens at converge and is
  recorded in `sessions/` — read `relay/tests/test_relay_deploy.bats`, `tests/helpers.bash`.
- Roles present: base, broker, caddy, deploy_user, firewall, netns_service, relay, ssh_hardening,
  telemetry, tenants — `ls ansible/roles/`.
- caddy role has no `service.d` yet and ships a distro unit; `group_vars/all.yml` has no hardening keys
  yet — `find ansible/roles/caddy`, grep all.yml.
- No `CONTRACT.md` at root/docs — so the baseline contract is a new doc, not an edit.

**Resolved in Phase 0 (live probes 2026-07-30; full detail in `sessions/2026-07-30-hardening-phase0.md`):**
- caddy: `PrivateUsers=yes` **breaks** the :443 bind (namespaced `CAP_NET_BIND_SERVICE`) → carve-out =
  skip PrivateUsers. Everything else holds; `MemoryDenyWriteExecute` fine (Go, no JIT); needs
  `AF_UNIX`+`AF_NETLINK`; `ReadWritePaths=/var/lib/caddy`. Achieved **8.8 → 1.9**.
- broker: full deep set incl. `PrivateUsers` → **1.5**, keys load (`/jwks.json` serves the ES256 key). No
  carve-out.
- telemetry: full deep set incl. `PrivateUsers` → **1.2**, cross-unit cgroup read intact (15 units
  sampled fresh in-run; live vs sampled match). Carve-out = never add a cgroup namespace.
- Identity/filesystem/cert audit results: see Verified in the session log — ids are non-deterministic
  (no `sysusers.d`), caddy in `www-data`, state modes/secret modes correct, cross-service reads denied,
  caddy admin API exposed (C1), relay key duplicated (C2), no FDE (C3).

## Documentation Impact

- `croft-stack/HARDENING-BASELINE.md` — **NEW** (Phase 1). The contract: required directive set, the
  exposure-ratchet table (one row per unit), and the documented per-unit exceptions. The single source
  of truth for "what hardened means here."
- `croft-stack/ansible/roles/hardening/README.md` — **NEW** (Phase 1). The shared role: what it
  templates, the per-unit override vars, how to onboard a new unit.
- `croft-stack/tests/hardening.bash` — **NEW** (Phase 1). Shared bats assertion helpers (the test-gate
  library). Referenced by every per-service hardening bats.
- `croft-stack/ansible/roles/caddy/README.md` (create if absent), `broker`, `telemetry`, `tenants`
  role docs / relay README — note the hardening drop-in and link the baseline. Each in its own phase.
- `croft-stack/reviews/2026-07-30-service-hardening-review.md` — Phase 6: flip status to APPLIED, record
  the achieved exposure scores.
- Latest stack review (`croft-stack/reviews/2026-07-29-stack-review.md`) — Phase 6: security-posture
  addendum with the new estate-wide scores (or cut a fresh dated snapshot — see Q3).
- `discovery/alpha/ROADMAP_TODO.md` — E78 → DONE on completion (Phase 6).
- `croft-stack/relay/...` README/hardening.conf header — Phase 1b: note the drop-in is now rendered
  from the shared `hardening` role.
- Cross-refs: `netns-isolation-plan.md` references the relay's `hardening.conf` — Phase 1b keeps the
  path/behavior identical, so no stale reference, but grep to confirm (search: `hardening.conf`).

## Concurrency Map

Sequential spine: **Phase 0 → Phase 1 → Phase 1b → Phase 2 → Phase 3 → Phase 4 → Phase 5 → Phase 6.**

All phases sequential. Reason: every service phase (2–5) applies its change to the **single shared live
box** and must pass its live gate (`systemd-analyze security` + functional probe + service active)
before the next begins — the box is shared mutable state, and the whole method is "apply incrementally,
verify each." The *code* edits for phases 2–5 touch mostly disjoint files (separate roles + separate
bats), but they all append to `ansible/group_vars/all.yml` (shared write-set) and all read the Phase 1
`hardening` role, so parallelizing would collide on `all.yml` and on the box. Sequential by priority
(caddy → broker → tenant → telemetry) is the deliberate choice, not an oversight.

## Phases

**Cross-cutting per-phase additions (apply in every service phase 1b–5).** Beyond the sandbox drop-in,
each phase also (a) pins the unit's identity via the shared `sysusers.d` (fixed uid/gid, decision Q7)
and drops any vestigial group (caddy's `www-data`, I2); (b) sets the unit's home to `/nonexistent`/state
dir (I3) and its state dir to the tightest mode via `tmpfiles.d` (0700 where no group reader, F1); (c)
adds the unit's **negative gates** (cross-service unreadable, no out-of-sandbox write, `PrivateUsers`
remap confirmed, `InaccessiblePaths` enforced) to its bats. The identity remap is realized once via
sysusers.d; if Q7 chooses a clean uid block, a one-time `chown -R` of state dirs + restart is part of
1b's converge. Each phase's Done-when includes its negative gates passing, not just the positive ones.

### Phase 0: Discovery — DONE (2026-07-30) ✓
Resolved all five directive probes (D1–D5) plus the identity/filesystem and certificate audits. Results
in `sessions/2026-07-30-hardening-phase0.md` and folded into Verified Assumptions + the findings/decision
list below. Net: caddy 8.8→**1.9** (carve-out: skip PrivateUsers), broker 4.7→**1.5**, telemetry
4.4→**1.2** (no carve-outs); identity/fs findings I1–I3, F1–F2; cert findings C1–C5. Every probe was
transient (`/run` drop-in) and reverted; box left clean. Disposition: throwaway (satisfied).

### Phase 1: Define the baseline — DONE (2026-07-30) ✓
Delivered: `HARDENING-BASELINE.md` (five-axis contract + ratchet table), `tests/hardening.bash`
(positive static/live + negative/adversarial helpers), `tests/test_hardening_baseline.bats` (7/7 green
— RED proof, relay reference, caddy carve-out, cap-only, canonical ids, **rendered-role wiring gate**,
live negative gate), the `hardening` role (defaults + drop-in template + tasks/handlers + README), and
`scripts/render-hardening-dropin.py`. Not yet converged on the box (that's 1b onward). TDD: library +
self-test first (RED against a deliberately-incomplete fixture), then the role template that satisfies it.

- [ ] Write `croft-stack/HARDENING-BASELINE.md` — the contract, now a **per-component profile** with
  five axes: (1) **identity** — canonical uid/gid + the justified group-membership list; (2)
  **filesystem** — owned paths with owner+mode, secret modes, `ReadWritePaths`/`InaccessiblePaths`; (3)
  **certificates** — the ACME-authority/consumer topology, key-copy policy, admin-API policy, at-rest
  policy, backup-exclusion invariant; (4) **capabilities**; (5) **namespaces**. Plus the exposure-ratchet
  table (one row per unit, "recorded / assert ≤") and the per-unit exception column (from Phase 0).
- [ ] Write `croft-stack/tests/hardening.bash` — the test-gate library, **two tiers**:
  - *Positive (static, grep):* `assert_baseline_directives <file> [exceptions...]`, `assert_directive`,
    `assert_cap_only`, `assert_not_present` (telemetry no-cgroup-ns), `assert_uid_gid <unit> <uid> <gid>`,
    `assert_path_mode <path> <owner> <group> <mode>`.
  - *Negative (live, adversarial):* `assert_cannot_read <user> <path>`, `assert_only_readers <path> <owner> <group>`,
    `assert_uid_remapped <unit>`, `assert_inaccessible <unit> <path>`, `assert_cannot_write <unit> <path>`,
    `assert_netns_no_host_route <ns> <host:port>`, `assert_egress_denied <ns> <target>`,
    `assert_admin_api_not_tcp`. These SSH to the box / use `sudo -u` / `nsenter` and require the forbidden
    action to fail.
- [ ] Write `croft-stack/tests/test_hardening_baseline.bats`: (a) positive helper flags a fixture missing
  a directive (RED proof); (b) positive helpers pass the relay's rendered `hardening.conf` (reference);
  (c) at least one negative helper self-tests against a known-denied case (e.g. `broker` cannot read
  `/var/lib/caddy` — already true today, so GREEN immediately and proves the negative harness works).
- [ ] Create `croft-stack/ansible/roles/hardening/`: `templates/hardening.conf.j2` (deep set + per-unit
  vars via the loop `item`: `base_set`, `private_users`, `mdwe`, `extra_caps`, `address_families`,
  `readwrite_paths`, `inaccessible_paths`), `defaults/main.yml` (`croft_identities`, `croft_state_dirs`,
  `hardening_units`), `tasks/main.yml`, `handlers/main.yml`, `README.md`.
  **Design decision (during build):** identity/filesystem are realized with the idempotent
  `group`/`user`/`file` modules keyed on the canonical ids — **not** `sysusers.d`/`tmpfiles.d`. Reason:
  `sysusers.d` only *creates* missing users; it cannot renumber the existing 991–999 accounts, so it
  can't perform the remap Q7 requires. The `user`/`group` modules pin uid/gid idempotently (no-op once
  correct) *and* remap in place; the canonical ids in `defaults/main.yml` remain the portable source of
  truth (fresh boxes converge to the same numbers). The recursive re-own is owner-only (never touches
  file modes — that would break `0600` key files); the top dir mode is set separately.

**Wiring test:** `test_hardening_baseline.bats` sources `tests/hardening.bash` and asserts the *rendered*
drop-in (relay vars) satisfies the positive baseline, **and** a negative helper runs end-to-end against
the live box — proving library + template + role + the live harness all compose.
**Validation:** positive bats green; the negative self-test denies as expected; `HARDENING-BASELINE.md`
covers all five axes. **Read-set:** relay `hardening.conf`, unit files, `tests/helpers.bash`, Phase 0
audit. **Write-set:** the new files + role dir. **Shared-state:** the box (negative self-test is
read-only/`sudo -u` probes). **Re-entry:** box unchanged; no probe residue.
**Done-when (static):** positive bats green. **Done-when (live):** the negative self-test passes (a
known-forbidden action is denied).

### Phase 1b: Migrate the relay onto the shared baseline (prove reproduction of the reference)
**Reproduction proof DONE (2026-07-30) ✓** — relay converged onto the shared `zz-hardening.conf`
(standalone `hardening.conf` removed), exposure reproduced at **1.7**, TLS serving in-netns (HTTP 200),
negative gates hold, telemetry unaffected. Session: `sessions/2026-07-30-hardening-phase1b-relay.md`.
**Identity remap deferred** (see finding below). Idempotency re-run pending confirmation.

**Structural finding (Q11) — identity pinning must run EARLY, not in the end-stage hardening role.**
The relay role installs the cert copy (`root:relay`) before the hardening role would run last; a late
gid remap leaves that cert's group stale and the relay can't read it on restart. Generally: any role
creating files owned by a canonical id must run *after* the id is pinned. **Decision:** move the
identity + state-dir tasks to an early step (dedicated `identity` role first, or fold into `base`),
leaving `hardening` to own only the sandbox drop-in. The `hardening_apply_identity` toggle already
lets us defer identity; the estate remap happens in the early step once added. Recommendation: add an
`identity` role at the front of `site.yml` before the estate-wide remap in the service phases.

- [ ] Add `hardening` role invocation for the relay in `site.yml` (or fold into the relay role) with the
  relay's carve-out vars; **remove** the relay's standalone `hardening.conf` in favor of the rendered
  one. Keep the drop-in path/content byte-compatible where possible.
- [ ] Converge; confirm `changed` only on the intended files, then `changed=0` on a second run.
- [ ] **Remap ordering (relay gid 984→644):** the certsync copy is `root:relay`, so after the gid
  change its group is a stale number until certsync re-runs. Sequence: remap uid/gid → `chown -R`
  state dir → **re-run `iroh-relay-certsync`** (so `install -g relay` re-owns the cert to the new gid)
  → restart relay. Verify the relay can read the cert (TLS serves) before declaring 1b done. (This
  interacts with the Q9 tmpfs move in Phase 2c — do the plain re-own here, tmpfs later.)

**Wiring test:** relay's existing `test_relay_deploy.bats` still green; the rendered relay drop-in
satisfies `assert_baseline_directives`. **Validation:** live — relay `systemd-analyze security` still
**1.7**; the two-node `relay-loadtest` still **5/5 direct** and **5/5 forced-relay**; telemetry still
exact-matches (per the 2026-07-30 method). Recorded in the Phase-0/1 session log.
**Read-set:** relay role + hardening role. **Write-set:** `site.yml`/relay role tasks, relay README,
remove standalone `hardening.conf`. **Shared-state:** the live box (relay only). **Re-entry:** relay
`active`; exposure == 1.7; 5/5 direct preserved.
**Done-when (static):** relay bats green; rendered drop-in passes the baseline. **Done-when (live):**
relay 1.7 + 5/5 direct + 5/5 forced-relay + telemetry exact-match, `changed=0` on re-converge.

### Phase 2: caddy — DONE (2026-07-30) ✓ (8.8 → 1.9)
Sandbox drop-in converged (identity deferred). caddy `NoNewPrivileges=yes`, caps reduced to exactly
`cap_net_bind_service`, `PrivateUsers=no` (D1), exposure **1.9**, still serving TLS on 443; 11/11 bats
green incl. the ProtectSystem write-deny negative gate. Added `ProtectKernelModules`+`ProtectClock` to
the canonical set (proven safe in Phase 0) — improved the relay to **1.5** as a bonus. Session:
`sessions/2026-07-30-hardening-phase2-caddy.md`. Admin-API socket (C1) + tmpfs key (C2) remain for 2c.
- [ ] Per-service bats `croft-stack/caddy/tests/test_caddy_hardening.bats` (create `caddy/tests/` or add
  to the role tests): sources `tests/hardening.bash`; asserts the rendered caddy drop-in carries the
  baseline set with caddy's carve-outs (`assert_cap_only ... CAP_NET_BIND_SERVICE`,
  `AmbientCapabilities=CAP_NET_BIND_SERVICE`, `NoNewPrivileges=yes`, address families incl. AF_NETLINK,
  `ReadWritePaths` for the cert store; PrivateUsers/MDWE present or excluded per Phase 0). **Run it first
  — watch it fail** (no drop-in yet).
- [ ] Add `caddy_hardening_*` vars to `group_vars/all.yml`; wire the `hardening` role for caddy.
- [ ] Converge caddy; verify.

**Wiring test:** the bats asserts the drop-in is installed into `caddy.service.d/` (not just present in
`deploy/`), i.e. the role task renders it to the unit. **Validation:** live — `systemd-analyze security
caddy` drops from 8.8 to its recorded ceiling; `curl https://<host>` serves a valid cert; ACME renew
path intact; caddy `active`. Record the achieved score in the ratchet table + session log.
**Read-set:** caddy role, hardening role, all.yml. **Write-set:** caddy role tasks + vars, new bats,
all.yml (caddy block), baseline ratchet row. **Shared-state:** live box (caddy). **Re-entry:** caddy
`active` + serving; other units' scores unchanged.
**Done-when (static):** caddy hardening bats green; `changed=0` on re-converge. **Done-when (live):**
exposure recorded + ratcheted; 443 serves valid cert; ACME path writable.

### Phase 2c: Certificate handling — DONE (2026-07-30) ✓ (C1 + C2)
2c-A: caddy admin API moved off unauthenticated `localhost:2019` onto a root-only unix socket
(`admin unix//run/caddy/admin.sock` + `RuntimeDirectory` + retargeted `ExecReload`); TCP 2019 closed,
relay user gets 000, reload still works via the socket. 2c-B: relay key copy moved to tmpfs
(`/run/iroh-relay/certs` via tmpfiles.d), old `/etc/iroh-relay/certs` removed — no private key at rest;
relay still serves TLS in-netns; rationale inline in `certsync.sh`. 26/26 hardening bats; changed=0.
C3/C4 (at-rest FDE / backup exclusion) documented; C5 (event-driven certsync) not needed. Session:
`sessions/2026-07-30-hardening-phase2c-certs.md`. Q10 (broker `LoadCredentialEncrypted`) still deferred.

### (original 2c task list follows)
Runs with/after caddy since caddy is the ACME authority. TDD: each fix gets a negative gate that proves
the exposure is closed.
- [ ] **C1 — close the admin API exposure.** Add `admin unix//run/caddy/admin.sock` to the Caddyfile
  (keeps `caddy reload`; removes the local-TCP `:2019` surface). Wire the socket dir via tmpfiles/
  RuntimeDirectory. *Negative gate:* `assert_admin_api_not_tcp` — another local user gets connection
  refused on `127.0.0.1:2019` (was 200). *Decision Q8 if `admin off` preferred instead.*
- [ ] **C2 — shrink the duplicate-key footprint.** Move the certsync destination to **tmpfs**
  (`/run/iroh-relay/certs`, mode 0750 root:relay) so the relay's key copy never persists to disk;
  certsync already runs on boot (`Persistent=true`) and the relay `Wants=/After=` it. Update the relay
  role's cert path + the certsync target. *Negative gate:* the key path is on a `tmpfs` mount and is
  `assert_only_readers root relay`; *positive:* relay still serves TLS (mode-B two-node still connects).
  *Decision Q9.*
- [ ] **C3/C4 — at-rest + backup invariants.** Record in `HARDENING-BASELINE.md`: caddy store stays
  plaintext (accepted); broker keys are candidates for `LoadCredentialEncrypted` (Q10, may defer);
  **any** future backup MUST exclude/encrypt the private-key paths. *Gate:* a bats doc/assertion that
  the baseline lists the key-path exclusion set (so a future backup phase can't silently omit it).
- [ ] **C5 (optional) — event-driven certsync.** Replace/augment the daily timer with a `systemd.path`
  watching caddy's cert dir for immediate propagation. Nice-to-have; skip if it complicates idempotency.

**Wiring test:** the negative gates run against the live box and confirm the exposures are actually
closed (admin API refuses TCP; relay key path is tmpfs + only root:relay). **Validation:** live — caddy
still serves all vhosts; the relay two-node mode-B connection still succeeds reading the tmpfs cert; the
daily/path certsync still lands the cert. **Read-set:** caddy role, relay role, certsync. **Write-set:**
Caddyfile.j2, certsync.sh + target, relay cert path, baseline cert section, new/updated bats.
**Shared-state:** live box (caddy + relay). **Re-entry:** caddy + relay `active`; TLS intact end-to-end.
**Done-when (static):** cert bats green; `changed=0` on re-converge. **Done-when (live):** admin API not
on TCP; relay key on tmpfs, only root:relay; relay serves TLS; two-node mode-B still connects.

### Phase 3: croft-broker — DONE (2026-07-30) ✓ (4.7 → 1.5)
Full deep set + `PrivateUsers` + `InaccessiblePaths` converged (identity deferred). Exposure **1.5**,
`/jwks.json` serves the ES256 key (loads under the userns), and the incidental `BROKER_SCOPE`
truncation is fixed (multi-token `Environment=` values now quoted → `transition:generic` restored).
Negative gates confirmed live: `/var/lib/caddy` + `/var/lib/iroh-relay` show 0 entries in the broker's
mount ns; uid_map is a non-identity remap. Fixed two test-harness bugs (assert_inaccessible semantics;
ssh pipe quoting) — not security gaps. 17/17 hardening bats green. Session:
`sessions/2026-07-30-hardening-phase3-broker.md`.
*Incidental fix (from Phase 0):* `croft-broker.service` line 22 `Environment=BROKER_SCOPE=atproto
transition:generic` is unquoted → systemd drops `transition:generic` and the broker's OAuth scope is
**silently truncated to `atproto`**. Quote it: `Environment="BROKER_SCOPE=atproto transition:generic"`.
Add a bats assertion that the rendered unit quotes any multi-token `Environment=` value.
- [ ] `broker/tests/test_broker_hardening.bats`: baseline set + `InaccessiblePaths` for the other
  services' state (`/var/lib/iroh-relay /var/lib/telemetry /var/lib/caddy`); PrivateUsers per D5. Watch
  it fail first.
- [ ] `broker_hardening_*` vars; wire the `hardening` role for the broker; converge.

**Wiring test:** drop-in installed into `croft-broker.service.d/`; broker bats + existing
`test_broker_deploy.bats` both green. **Validation:** live — `systemd-analyze security croft-broker`
toward ~1.7; broker health/metadata endpoint 200 (keys still loaded under PrivateUsers); the other
services' state is inaccessible from the broker (probe). Record score.
**Read-set:** broker role, hardening role, all.yml. **Write-set:** broker role tasks + vars, new bats,
all.yml (broker block), ratchet row. **Shared-state:** live box (broker). **Re-entry:** broker `active`
+ endpoint 200; caddy still serving (it fronts the broker).
**Done-when (static):** broker hardening bats green; `changed=0`. **Done-when (live):** exposure
ratcheted; broker functional; InaccessiblePaths confirmed.
*(Netns for the broker is out of scope here — noted as a follow-on in the review; deep unit hardening
first.)*

### Phase 4: tenant template / canary — DONE (2026-07-30) ✓ (4.7 → 1.5)
Deep set folded into the render.py tenant template via a `${sandbox}` block + a `[hardening]` manifest
table (default-on; `private_users`/`mdwe`/`syscall_filter` overridable per tenant). Regenerated
`generated/`; canary converged to **1.5**, active, `/healthz` 200, `PrivateUsers` effective, can't read
broker state. Fixed a tenants-role gap (a unit change wasn't restarting the tenant — added a restart
handler). 3/3 hermetic tenant bats + render.bats regression green. Session:
`sessions/2026-07-30-hardening-phase4-tenants.md`.
- [ ] `tenants/tests/test_tenant_hardening.bats` (or extend the render test): assert the **rendered**
  `config-templates/service.service.tmpl` carries the baseline deep set, and that the riskier directives
  (`SystemCallFilter`, `MemoryDenyWriteExecute`, `PrivateUsers`) are **overridable per manifest** with a
  baseline default-on. Assert a second (fixture) tenant manifest inherits the directives. Watch it fail.
- [ ] Fold the deep set into `service.service.tmpl` with per-manifest override vars; re-render canary.
- [ ] Converge; verify.

**Wiring test:** render the template for *two* manifests (canary + a fixture tenant) and assert both
emit the baseline directives — proving the leverage (all tenants inherit), not just canary.
**Validation:** live — `systemd-analyze security canary` drops to recorded ceiling; canary `/healthz`
200; canary `active`. Record score.
**Read-set:** tenants role, `config-templates/service.service.tmpl`, hardening baseline. **Write-set:**
the template, tenants role vars, new bats, ratchet row. **Shared-state:** live box (canary). **Re-entry:**
canary `active` + healthy; no other tenant disturbed.
**Done-when (static):** tenant hardening bats green (both manifests inherit); `changed=0`. **Done-when
(live):** canary exposure ratcheted; `/healthz` 200.

### Phase 5: telemetry-poll — DONE (2026-07-30) ✓ (4.4 → 1.2)
Full deep set + `PrivateUsers`, no cgroup namespace. Exposure **1.2**; the functional gate confirms a
poll under the sandbox still records fresh cross-unit cgroup samples (≥5 other units), and telemetry
cannot read other services' state. Session: `sessions/2026-07-30-hardening-phase5-telemetry.md`.
All four long-lived units now hardened via the shared baseline (relay 1.5, caddy 1.9, broker 1.5,
telemetry 1.2). Identity remap still deferred (Q11).
- [ ] `telemetry/tests/test_telemetry_hardening.bats`: baseline deep set **minus cgroup-hiding** —
  assert `ProtectControlGroups=yes` (RO) present, `assert_not_present` any cgroup namespace directive,
  and the safe deep directives present (PrivateUsers per D4, MDWE, ProtectProc, RestrictNamespaces,
  SystemCallFilter, LockPersonality). Watch it fail.
- [ ] `telemetry_hardening_*` vars; wire the `hardening` role for telemetry; converge.

**Wiring test:** drop-in installed into the telemetry unit's `.d/`; telemetry bats + existing
`test_deploy_units.bats` green. **Validation:** live — `systemd-analyze security telemetry-poll` drops
to recorded ceiling; a **fresh poll still records cross-unit cgroup samples** (the exact-match check)
under PrivateUsers; `/var/lib/telemetry` writable; retention prune still runs. Record score.
**Read-set:** telemetry role, hardening role, all.yml. **Write-set:** telemetry role tasks + vars, new
bats, all.yml (telemetry block), ratchet row. **Shared-state:** live box (telemetry). **Re-entry:**
telemetry `active`; a fresh sample matches a live cross-unit cgroup value.
**Done-when (static):** telemetry hardening bats green; `changed=0`. **Done-when (live):** exposure
ratcheted; cross-unit cgroup read + write + prune all confirmed.

### Phase 6: consolidate — finalize the baseline, docs, stack review
- [ ] Fill the ratchet table in `HARDENING-BASELINE.md` with all recorded scores; finalize the per-unit
  exception column from what Phases 2–5 actually needed.
- [ ] Flip `reviews/2026-07-30-service-hardening-review.md` status → APPLIED with the achieved scores.
- [ ] Security-posture addendum on the latest stack review (or a fresh dated snapshot — Q3).
- [ ] Role READMEs reference `HARDENING-BASELINE.md`; document how a **new** unit onboards (add a
  ratchet row, set its `*_hardening_*` vars, add a one-liner bats using the library) — the "keep
  building up the baseline" property, written down.
- [ ] `ROADMAP_TODO` E78 → DONE.
- [ ] Session log for the whole pass; commit/push (on user request).

**Validation:** full `bats` suite green; a clean converge reports `changed=0` across every hardened
unit (idempotency, estate-wide); `systemd-analyze security` for all five units recorded in one table.
**Read-set:** all touched docs. **Write-set:** the docs above. **Shared-state:** none (docs) + one final
verify converge. **Re-entry:** estate `changed=0`; all units `active`.
**Done-when (static):** whole bats suite green. **Done-when (live):** estate-wide `changed=0` + a
recorded five-unit score table.

### Phase 7: Reboot persistence gate (final, user-requested)
After every unit is hardened + the identity remap is done, **reboot the box** and confirm the whole
estate comes back in the expected hardened state — persistence is not assumed, it's proven.

- [ ] Reboot `croft-vps`. On return, assert:
  - all units active: caddy, croft-broker, iroh-relay (in its netns), telemetry-poll.timer; canary.
  - **netns recreated on boot** (the reboot-persistent oneshot) and the relay reachable via DNAT.
  - **tmpfs cert copy re-populated before the relay started** (Q9: `/run/iroh-relay/certs` is wiped on
    reboot; certsync `Persistent=true` + the relay `Wants=/After=` it must re-sync first, or the relay
    can't read its cert). This is the highest-risk reboot interaction — assert relay serves TLS.
  - exposure scores match the ratchet table for every unit (drop-ins are in `/etc`, persistent).
  - identity remap persisted (uid/gid in `/etc/passwd`); state dirs owned + moded correctly.
  - firewall/nftables + egress-deny restored; telemetry DB intact and the timer fires a fresh sample.
  - negative gates still hold post-boot (cross-service reads denied, admin API not on TCP).
- [ ] A `changed=0` converge after reboot (nothing drifted across the boot).

**Validation:** live — a single post-reboot script runs the full positive + negative gate set across all
units and prints a pass/fail table (reuse `tests/hardening.bash`). **Done-when (live):** every unit
active + hardened + serving, tmpfs cert re-synced, `changed=0`, all negative gates pass after a cold boot.

## Open Questions

- **Q1 (BLOCKING → Phase 0/D1):** Does caddy tolerate `PrivateUsers` with ambient
  `CAP_NET_BIND_SERVICE`? Recommended handling: probe in D1; if it breaks the bind, caddy's documented
  carve-out is "skip PrivateUsers" (still a large win from 8.8). *No user decision needed — a probe.*
- **Q2 (BLOCKING → Phase 0/D4):** Does telemetry keep cross-unit cgroup read under `PrivateUsers`?
  Handling: probe in D4; carve-out if not. *Probe, not a user decision.*
- **Q3 (non-blocking, user preference):** Phase 6 — security-posture **addendum** to the 2026-07-29
  stack review, or a **fresh dated stack-review snapshot**? Recommendation: addendum + status flip on
  the hardening review (a snapshot is heavier than this change warrants). *Confirm at Phase 6.*
- **Q4 (non-blocking):** Shared `hardening` **role** vs folding the template into `base`. Recommendation:
  a dedicated role (discoverable, single responsibility, mirrors `netns_service`). *Proceeding with the
  role.*
- **Q5 (non-blocking):** Exposure gate = **ratchet** (assert ≤ recorded) vs absolute targets.
  Recommendation: ratchet — deterministic, no day-one-fail on caddy. *Proceeding with the ratchet.*
- **Q6 (non-blocking):** Tenant override mechanism = per-manifest var with baseline default-on.
  Recommendation: yes (default-hardened, explicit opt-out per tenant). *Proceeding.*
- **Q7 — RESOLVED (2026-07-30): clean croft block + remap.** Canonical ids **uid=gid per service**:
  caddy 640, broker 641, canary 642, telemetry 643, relay 644 (documented, uncontended). Realized via
  `sysusers.d` (authoritative on fresh boxes). **This box needs a one-time guarded migration** —
  `sysusers.d` only *creates* missing users, it won't renumber the existing 991–999 accounts. So the
  role carries an idempotent migration: for each service, if the live uid/gid ≠ canonical, stop the
  unit → `groupmod -g`/`usermod -u` → `chown -R <uid>:<gid>` every owned path (state dir + any others)
  → start. Guarded on "current ≠ desired" so it's a no-op after the first converge (`changed=0`). Runs
  in Phase 1b (before the relay re-verify) and re-checked per service phase.
- **Q8 — RESOLVED: caddy admin API on a root-only unix socket** (`admin unix//run/caddy/admin.sock`).
  Keeps `caddy reload`; removes the local-TCP `:2019` surface.
- **Q9 — RESOLVED: relay key copy on tmpfs** (`/run/iroh-relay/certs`, 0750 root:relay). Duplicate key
  is RAM-only, re-synced on boot (certsync `Persistent=true`, relay `Wants=/After=` it).
- **Q10 (non-blocking — certs/C3, deferred):** `LoadCredentialEncrypted` for the broker keys (host-bound
  at-rest encryption). Fast-follow after the core baseline; additive, doesn't block the profile.

## Status: hardening complete (2026-07-30), reboot test pending

Every unit hardened via the baseline + on the clean identity block:

| unit | exposure | uid=gid |
|------|----------|---------|
| iroh-relay | 1.5 | 644 |
| caddy | 1.9 | 640 |
| croft-broker | 1.5 | 641 |
| telemetry-poll | 1.2 | 643 |
| canary (tenant) | 1.5 | 642 |

Phases 0–5, 1b, 2c, 4, and Q11 all DONE + committed + idempotent (`changed=0`). Certs: admin API on a
unix socket (C1), relay key on tmpfs (C2). Only **Phase 7 (reboot-persistence test)** remains.

**Q11 DONE (2026-07-30):** identity moved to an early `identity` role; estate remapped to 640–644. Two
bugs found + fixed live — `usermod` can't renumber a running user (stop-before-usermod, guarded) and
`when: ... is exists` checks the control host not the target (target-stat guard; caddy's dir had stayed
at the orphaned old gid). `StateDirectoryMode` set to 0700 estate-wide so systemd + identity agree.

## Review Log
- 2026-07-30: Pass 1+2 authored. Conventions verified against the live repo (bats = static + converge;
  shared `tests/helpers.bash`; roles list; caddy has no service.d; no CONTRACT.md). Phase 0 added to
  de-risk the four gated directives before templating. Exposure framed as a ratchet.
- 2026-07-30: **Phase 0 executed and scope broadened (user direction).** All directive probes resolved
  (caddy 1.9 skip-PrivateUsers, broker 1.5, telemetry 1.2). Added three audit passes: identity/uid-gid
  least privilege + portability (findings I1–I3), filesystem perms (F1–F2), and a dedicated certificate
  pass (C1 admin-API exposure, C2 duplicate key, C3 no-FDE, C4 backup invariant, C5 renewal freshness).
  Baseline redefined as a five-axis **least-privilege profile** realized deterministically via
  `sysusers.d`/`tmpfiles.d` (portability). Test-gate library gains a **negative/adversarial tier**
  (protection is enforced, not just configured). New certificate phase (2c) and decisions Q7–Q10.
