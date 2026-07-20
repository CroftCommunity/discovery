# RUN-STELLIN-INFRA-01: Stellin AppView production deployment kit (OVH + Litestream + R2)

`Status: runnable brief, 2026-07-17. Two phases: Phase 1 is unattended (build and validate the
complete deployment kit locally, no money spent, no credentials needed). Phase 2 is owner-gated
(real provisioning, bootstrap, and the fire drill; requires OVH and R2 credentials and an explicit
owner go). DNS is at Porkbun and stays there. TDD/checks-first per the standing directive
(2026-07-15): every deliverable gets its executable check committed red before the deliverable
itself, and the run summary shows red-to-green commit order.`

---

## 0. Owner pre-steps (before the session starts)

1. Create an empty repo for the kit: `CroftCommunity/stellin-infra` (suggested). If you prefer a
   different home, tell the session at start. Do NOT use the discovery repo; deployment IaC is not
   corpus material.

2. Decide the OVH region/endpoint before Phase 2: `ovh-eu`, `ovh-ca`, or `ovh-us`. Note the US
   subsidiary uses separate accounts. Phase 1 parameterizes this; no decision needed to start.

3. Nothing else. The service domain is NOT required yet: the kit uses a variable `service_fqdn`
   and the staging default `stellin-staging.croft.ing` until the stellin.app decision lands.

---

## 1. Context (read once)

The service being deployed is the Stellin AppView: one static Rust binary (product of the RUN-14
spike line in the discovery repo), SQLite storage, ingesting a filtered Jetstream over WSS and
serving XRPC over HTTPS. Its state model, decided 2026-07-17, drives everything here:

- `/var/lib/stellin/index.db`: disposable projection. Rebuilt from network backfill + tail.
  NEVER backed up. Its absence from every backup path below is intentional; do not "fix" it.

- `/var/lib/stellin/state.db`: canonical, small (firehose cursor, telemetry, grants). Continuously
  replicated by Litestream to Cloudflare R2.

- `/var/lib/stellin/blobs/`: sealed group ciphertext (content-blind store). Mirrored to the same
  R2 bucket by an rclone timer. Already ciphertext; safe on third-party storage.

Design rationale on record: provider-side backups fail with the provider (OVH's own docs state
Standard/Premium backups replicate within the same datacentre), so the canonical copy goes
off-provider to R2, whose zero egress makes restores free. OVH's included free Standard daily
backup is kept as an incidental extra, not relied on.

The binary itself may not exist yet in final form. The kit MUST NOT block on it: define a service
contract (a listening port, a `/healthz` endpoint returning 200, a `--data-dir` flag) and include
a tiny stub binary satisfying the contract so every check and the drill can run end to end. Wire
the real binary in by replacing one artifact path.

---

## 2. Guardrails

1. **Branch and commit discipline.** Work on `main` of the fresh stellin-infra repo (it is empty
   and unshared) or `claude/infra-01` if the owner has pushed anything. One commit per numbered
   deliverable, red check commit before green implementation commit, prefixes
   `test(infra):` / `feat(infra):`.

2. **No secrets in the repo, ever.** All credentials via environment (`OVH_APPLICATION_KEY`,
   `OVH_APPLICATION_SECRET`, `OVH_CONSUMER_KEY`, `AWS_ACCESS_KEY_ID`/`AWS_SECRET_ACCESS_KEY` for
   R2, `LITESTREAM_*`). `.gitignore` includes `*.tfstate*`, `.terraform/`, `*.env`. Terraform
   state stays local in Phase 2 (it contains service identifiers; owner may later move it to a
   backend). A pre-commit grep check for key-shaped strings is deliverable D1.

3. **Money gate.** `terraform apply`, and anything that orders an OVH service, is Phase 2 only and
   requires the owner's explicit go in-session. An OVH VPS is a MONTHLY purchase, not hourly.
   State costs before applying: primary box ~$4.20 to $4.54/mo entry tier (verify live price in
   the plan output), and the fire drill's second box is another monthly purchase unless the owner
   chooses the reinstall-in-place drill variant (section 5, step P2-6).

4. **Checks-first, honestly scoped.** Infra "tests" here are: `terraform fmt -check` +
   `terraform validate`, `shellcheck` on all scripts, `caddy validate` on the Caddyfile,
   `litestream` config parse, `systemd-analyze verify` on unit files, and bats (or plain sh
   assert) tests for script behavior. Install tools via the proxy; if one cannot be installed,
   mark that check BLOCKED in the summary rather than skipping silently. The full-system
   acceptance test is the Phase 2 fire drill; Phase 1 greens are component-level and the summary
   must say so.

5. **Stop rules.** Time-box stuck installs/builds at ~30 min. Design decisions are the owner's:
   if a step requires choosing region, drill variant, Premium backup, or spending money, surface
   the options and pause. Do not invent scope beyond the deliverables list.

---

## 3. Phase 1 deliverables (unattended, in order)

Each Dn is: red check commit, then implementation commit.

**D1. Repo skeleton and secret hygiene.**
`Makefile` with a `check` target that runs every checker below; `.gitignore` per guardrail 2; a
`scripts/no-secrets-check.sh` grepping staged files for credential-shaped strings, wired into
`make check`. Red state: `make check` exists and fails because nothing else does yet.

**D2. Service contract and stub.**
`CONTRACT.md` defining: binary name `stellin-appview`, flags `--data-dir` and `--listen`,
`/healthz` returning 200 when ingest cursor is loaded, data files exactly as in section 1.
`stub/` containing a ~40-line Rust (or shell + busybox httpd if Rust is unavailable) stub
honoring the contract. Check: a bats test that starts the stub, curls `/healthz`, asserts the
data-dir files it creates, kills it.

**D3. Terraform module.**
`terraform/` with: ovh provider pinned; `variables.tf` (`ovh_endpoint`, `datacenter`,
`plan_code`, `ssh_public_key`, `service_fqdn` defaulting to `stellin-staging.croft.ing`);
`vps.tf` using the `ovh_vps` resource with the plan structure the provider requires (plan code +
configuration labels for datacenter and OS `Debian 12`); outputs for IPv4/IPv6 and service name.
Include `scripts/catalog-vps.sh`: an authenticated GET against `/order/catalog/public/vps` on the
chosen endpoint that lists current plan codes and prices, because codes are not documented and
change by generation; its output is how the owner picks `plan_code` in Phase 2. Do NOT hardcode a
guessed plan code as default; leave it required. Standard daily backup is included free on
current-generation VPS, so no backup option is ordered; note in comments that Premium 7-day is a
single additional plan block if ever wanted. Check: `terraform fmt -check` + `terraform validate`
(validate needs no credentials).

**D4. Bootstrap script.**
`bootstrap/bootstrap.sh`, idempotent, run over SSH as root on a fresh Debian 12 VPS. It must:
create user `stellin` (no shell login, no sudo); write nftables rules allowing 22/80/443 inbound
and drop else; enable unattended-upgrades; disable SSH password auth and root password login;
install Caddy from its official apt repo; install Litestream and rclone from release artifacts
(pin versions in a `versions.env`); create `/var/lib/stellin/{blobs}` owned by `stellin`; install
all config from `config/` (D5); `systemctl enable --now` the units. Idempotent means a second run
changes nothing and exits 0. Checks: shellcheck; a bats test running the script twice against a
chroot-or-container if available in the environment, else a dry-run mode (`--plan`) whose output
is asserted, with the limitation recorded.

**D5. Runtime configuration.**
`config/` containing:

- `stellin-appview.service`: `User=stellin`, `StateDirectory=stellin`, `Restart=always`,
  `RestartSec=2`, hardening (`NoNewPrivileges=yes`, `ProtectSystem=strict`,
  `ReadWritePaths=/var/lib/stellin`, `PrivateTmp=yes`), `ExecStart` per CONTRACT.md.

- `litestream.service` + `litestream.yml`: replicate ONLY `/var/lib/stellin/state.db` to
  `s3://<bucket>/state` with the R2 endpoint URL variable, sync interval 1s, snapshot interval
  24h, retention 168h. A comment states why index.db is absent (section 1) so future editors do
  not add it.

- `blobs-sync.service` + `blobs-sync.timer`: rclone sync `/var/lib/stellin/blobs` to
  `:s3:<bucket>/blobs` every 5 minutes, `--immutable` if blob naming permits (blobs are
  content-addressed ciphertext; flag if the assumption fails).

- `Caddyfile`: `{$SERVICE_FQDN}` reverse_proxy to the contract port, HTTP-01 ACME (no DNS plugin
  and no Porkbun API needed; Porkbun only holds the A/AAAA records).

Checks: `systemd-analyze verify` on each unit, `caddy validate`, litestream config parse.

**D6. Deploy pipeline.**
`.github/workflows/deploy.yml`: on release tag, build (or fetch) the binary, rsync to
`/opt/stellin/stellin-appview.new` over a deploy key, atomically move + `systemctl restart
stellin-appview`. Server side: a dedicated `deploy` user whose authorized_keys forces the rsync
command. Check: `actionlint` if installable; else YAML parse + a bats test of the server-side
receive script.

**D7. DNS instructions (Porkbun, manual v1).**
`docs/DNS.md`: the exact A/AAAA records to create at Porkbun for `service_fqdn` pointing at the
Terraform outputs; a note that ACME is HTTP-01 so no DNS automation is required; the future
`_lexicon` TXT record documented as pending the stellin.app namespace decision, NOT created now;
one line noting Porkbun has an API if automation is ever wanted. Check: file exists and is
referenced from the runbook (link check in `make check`).

**D8. Runbook and drill script.**
`docs/RUNBOOK.md` covering: box loss (terraform re-apply, bootstrap, `litestream restore`
state.db, rclone copy blobs back, start service, index rebuilds via backfill + tail; expected
recovery point: seconds for state.db, up to 5 min for blobs, zero for index by construction);
cursor loss (delete cursor row, service backfills); Litestream failure (journald alert, restart,
verify generations); credential rotation (OVH keys, R2 token, deploy key). Plus
`drill/fire-drill.sh` implementing Phase 2 step P2-6 end to end with assertions, and its
`--variant second-box|reinstall` switch. Checks: shellcheck; link check; a bats test of the
drill script's assertion logic against the stub.

**D9. Phase 1 summary.**
`RUN-SUMMARY.md`: red/green commit table per deliverable, checks run and their tool versions,
anything BLOCKED with the one-line unblock, and the exact Phase 2 command sequence for the owner.

---

## 4. Phase 2 (owner-gated; may be a later session)

Owner supplies: OVH API credentials for the chosen endpoint, an R2 bucket (suggested name
`stellin-state`) + bucket-scoped token, and the go for spend.

- **P2-1.** `scripts/catalog-vps.sh` → owner picks `plan_code` and `datacenter` from live output;
  record price in the summary.

- **P2-2.** `terraform plan` → show the owner → `terraform apply` on go.

- **P2-3.** `bootstrap/bootstrap.sh` against the new box; run it TWICE; second run must be a
  no-op (the idempotence check, now against reality).

- **P2-4.** Owner creates the Porkbun A/AAAA records per `docs/DNS.md`; verify ACME issuance and
  `https://<fqdn>/healthz` 200 with the stub.

- **P2-5.** Verify the backup plane: write a marker row into state.db via sqlite3, wait one sync,
  `litestream restore` to a temp path on the same box, assert the marker is present; drop a
  marker blob, wait for the timer, assert it in R2 via rclone ls.

- **P2-6. The fire drill (the run's acceptance test).** Variant chosen by owner:
  `second-box` (another monthly VPS purchase, true provider-loss rehearsal, then destroy) or
  `reinstall` (OVH reinstall of the same VPS; free; rehearses everything except surviving-machine
  assumptions). Script asserts: fresh box + bootstrap + restore + blobs back + service up +
  `/healthz` 200 + marker row present. PASS is these assertions green; FALSIFY is any restore
  assertion failing, and that halts go-live until understood.

- **P2-7.** Final summary update: prices paid, drill variant and result, recovery-point numbers
  observed, and the single follow-up list (swap stub for real binary; `_lexicon` TXT when the
  namespace lands; Premium backup yes/no).

---

## 5. Explicit non-goals

No HA, no load balancer, no container runtime, no monitoring stack beyond `/healthz` + an
external pinger (owner sets that up in a UI; note the URL in the runbook), no DNS automation, no
remote Terraform state, no index.db backup ever. Each is one sentence in the runbook's "why not"
list so future sessions do not helpfully add them.
