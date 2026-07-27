# OVH box change log — auth-helper spike (revert record)

Box: `15.204.81.133` (`vps-e9655dff`) · user `debian` (passwordless sudo). Every mutation is logged
here so the box can be returned to baseline. This is a **throwaway spike**; teardown steps at the foot.

## Baseline (recorded 2026-07-24, before any change)

- OS: **Debian GNU/Linux 13 (trixie)**, kernel `6.12.86+deb13-amd64`, arch `amd64`. (Plan said Debian
  12; box is 13 — immaterial for the spike.)
- Listening: `22` (sshd), `53`/`5355` (systemd-resolved). Nothing else.
- Not installed: `node`, `npm`, `caddy`.
- No `/etc/caddy`. Custom systemd units: only distro defaults (networkd/resolved/timesyncd/sshd
  symlinks). Disk: 1.3G used of 99G.

## Changes applied

### C1 — apt install nodejs + caddy (2026-07-24)
- `sudo apt-get install -y nodejs caddy` → nodejs `20.19.2+dfsg-1+deb13u2`, caddy `2.6.2-12+b3`.
- Revert: `sudo apt-get purge -y nodejs caddy && sudo apt-get autoremove -y`.
- Side effect: caddy package created user `caddy`, unit `caddy.service` (enabled), `/etc/caddy/Caddyfile`.

### C2 — dedicated service user + install dir (2026-07-24)
- `useradd --system --no-create-home --home-dir /opt/auth-helper --shell /usr/sbin/nologin authhelper`.
- `/opt/auth-helper/dist` (root-owned, read-only to service), `/opt/auth-helper/data` (authhelper, 0700).
- Bundles `dist/server.mjs` + `dist/refresh-cli.mjs` copied to `/opt/auth-helper/dist/`.
- Revert: `sudo rm -rf /opt/auth-helper && sudo userdel authhelper`.

### C3 — systemd unit `auth-helper.service` (2026-07-24)
- `/etc/systemd/system/auth-helper.service` (see `deploy/auth-helper.service`). Runs as `authhelper`,
  listens `127.0.0.1:8001`, hardened (`ProtectSystem=strict`, empty caps, `ReadWritePaths=data`).
- `systemctl enable --now auth-helper.service`.
- Revert: `sudo systemctl disable --now auth-helper && sudo rm /etc/systemd/system/auth-helper.service && sudo systemctl daemon-reload`.

### C4 — Caddy reverse proxy + TLS (2026-07-24)
- Packaged `/etc/caddy/Caddyfile` backed up to `.orig`; replaced with a single `account.croft.ing`
  vhost reverse-proxying `127.0.0.1:8001` (see `deploy/Caddyfile`). Caddy auto-issued a Let's Encrypt
  cert (issuer `E2`, valid 2026-07-24 → 2026-10-22).
- Revert: `sudo cp /etc/caddy/Caddyfile.orig /etc/caddy/Caddyfile && sudo systemctl reload caddy`.

### C5 — daily unattended-refresh timer (2026-07-24)
- `/etc/systemd/system/auth-helper-refresh.{service,timer}` — a `daily` oneshot that runs
  `refresh-cli.mjs <did>` as `authhelper` and logs to `data/measurements.log`. Measures how long the
  confidential session survives unattended (Open decision 9 / Stage D long-run leg).
- Revert: `sudo systemctl disable --now auth-helper-refresh.timer && sudo rm /etc/systemd/system/auth-helper-refresh.{service,timer} && sudo systemctl daemon-reload`.

### C6 — stellin.app cross-origin demo pad + vhost (2026-07-24)
- DNS `stellin.app → 15.204.81.133` (owner-added). `/opt/stellin-pad/` (root-owned) holds `index.html`,
  `app.js` (browser bundle), `public-client-metadata.json` (the pad's OWN public client for the
  browser-only fallback). Second Caddy vhost `stellin.app` (file_server); Caddy auto-issued its cert.
- The helper (C3) gained CORS (allowlist `https://stellin.app`), a `return`-URL + opaque-ticket handoff,
  and a brokered `GET /api/whoami`. Redeployed `server.mjs`.
- Revert: `sudo rm -rf /opt/stellin-pad`; remove the `stellin.app {…}` block from `/etc/caddy/Caddyfile`
  (restore `deploy/Caddyfile`'s single-vhost form or `.orig`); `sudo systemctl reload caddy`.

### Secret material on the box (never leaves it)
- `/opt/auth-helper/data/assertion-key.jwk` (ES256 private, mode 0600) — signs client assertions.
- `/opt/auth-helper/data/store-key.bin` (AES-256, mode 0600) — encrypts pending/session records.
- Auto-generated on first boot. Never logged, never committed. Removed by the C2 revert.

## Teardown (return to baseline)

Run in order:
```
sudo systemctl disable --now auth-helper.service
sudo rm /etc/systemd/system/auth-helper.service && sudo systemctl daemon-reload
sudo cp /etc/caddy/Caddyfile.orig /etc/caddy/Caddyfile && sudo systemctl reload caddy
sudo rm -rf /opt/auth-helper && sudo userdel authhelper
sudo apt-get purge -y nodejs caddy && sudo apt-get autoremove -y
```
Then the box matches the recorded baseline. (Leave the DNS A record `account.croft.ing` — owner's.)
