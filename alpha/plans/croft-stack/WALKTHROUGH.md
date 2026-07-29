# croft-stack — walkthrough & demo guide (Phases 0–7)

date: 2026-07-29 · a review of what is **built and live**, and **how to prove/demo each component**.
Roadmap: [README.md](README.md). Actuals (per-session commands): `croft-stack/sessions/`.

**One-line state:** a single OVH Debian-13 VPS (`vps-e9655dff`, `15.204.81.133`), converged idempotently
from the `CroftCommunity/croft-stack` repo, running a governed + telemetry-observed estate — the canary
tenant live over HTTPS, and an iroh dev/test relay. Reconstructible from the repo alone. Phases 0–6 done;
Phase 7 (auth broker) is planned + spike-proven, build pending.

The design ethos (unchanged): **every pad works with no server; the server is an optional accelerator**
(degrade-to-serverless, minimal held state, governed by default).

---

## How to reach the box

`ssh croft-vps` (from `~/.ssh/config`: `debian@vps-e9655dff.vps.ovh.us`, key `chase_ovh_vps`, passwordless
sudo, **key-only** — password auth is off). Reconstruct-from-scratch: `git clone
git@github-personal:CroftCommunity/croft-stack.git && cd croft-stack/ansible && ansible-playbook site.yml`.

---

## Phase 0 — model + manifests  ✅
**What:** the agreed service model + the concrete `services/*.toml` set (naming scheme, ports, modes).
Paper artifact, no box work. **Prove:** read [`00-model-and-manifests.md`](00-model-and-manifests.md) —
the locked decisions Q1–Q6 (role-based subdomains, canary, ports, repo name `croft-stack`).

## Phase 1 — extract the kit → `croft-stack`  ✅
**What:** the discovery `appview-infra/kit` extracted to the standalone production repo `croft-stack`,
renamed off `appview-infra`, self-verifying.
**Prove:**
```
git -C croft-stack log --oneline | tail   # 2ed8d19 init → extract → rename
cd croft-stack && brew install bats-core bash shellcheck 2>/dev/null; make check   # 12/13 green
#   (check-terraform + check-local-drill are toolchain-gated: need terraform/tofu + litestream)
grep -rn appview-infra croft-stack --include=*.md | grep -v experiments   # only discovery source refs
```

## Phase 2 — adopt the OVH box (OpenTofu, read-only)  ✅
**What:** the adopted VPS read via `data.ovh_vps` (no order); reproduce recipe captured
(`vps-2027-model3`, US, `us-west-or-2`); box reimaged clean Debian 13. OpenTofu owns only the resource
layer; DNS is manual (Porkbun); no R2 yet.
**Prove:** (needs the OVH creds in `croft-stack/.env`, `ovh-us`)
```
cd croft-stack/terraform && tofu init
tofu plan   # reads the box, place_order=false, only computes outputs (no changes):
            #   resolved_plan_code=vps-2027-model3, vps_ipv4=15.204.81.133
```

## Phase 3a — cgroup governance (data-driven)  ✅
**What:** every generated systemd unit is cgroup-governed by default — accounting always on (template),
limits as manifest `[limits]` data with governed-by-default fallbacks.
**Prove:**
```
cd croft-stack && bats tests/render.bats                 # 13/13; incl. governance + [limits]-override
make generate && make generate && git diff --quiet generated && echo "generate is idempotent"
grep -E 'MemoryMax|CPUQuota|MemoryAccounting' generated/systemd/canary.service
ssh croft-vps 'sudo systemctl show canary.service -p MemoryMax -p MemoryAccounting'  # 64M, yes
```

## Phase 3b — telemetry client (Python, stdlib, TDD)  ✅
**What:** `croft-stack/telemetry/` — a cgroup-v2 per-unit reader + `poll`/`show` CLI, SQLite time-series,
deployed as a 60s systemd timer. Zero pip deps.
**Prove:**
```
cd croft-stack/telemetry && uv run --with pytest pytest    # 32 pytest + (bats tests/*.bats) 6 bats
# on the box — the live sampler:
ssh croft-vps 'sudo systemctl start telemetry-poll.service; cd /opt/telemetry/current && \
  sudo python3 -m croft_telemetry.cli show canary.service --db /var/lib/telemetry/samples.db'
#   → rows with mem/cpu/pids; canary shows io_r/io_w populated (governance delegates the io controller)
```

## Phase 4 — Ansible converge (idempotent, no-lockout)  ✅
**What:** the in-box layer — Ansible converges base/firewall/ssh-hardening/caddy/deploy-user/tenants/
telemetry. `bootstrap.sh` retired. Reconstructs the box from the repo.
**Prove:**
```
cd croft-stack/ansible && ansible-playbook site.yml --syntax-check && ansible croft -m ping
ansible-playbook site.yml        # a converged box → changed=0 (idempotency)
ssh croft-vps 'systemctl is-active canary.service; sudo nft list ruleset | grep "policy drop"; \
  sudo sshd -T | grep -E "^passwordauthentication|^permitrootlogin"'   # active; drop; no; no
ssh croft-vps 'python3 -c "import urllib.request;print(urllib.request.urlopen(\"http://127.0.0.1:8100/healthz\").read().decode())"'  # ok
```

## Phase 5 — DNS + TLS (canary live over HTTPS)  ✅
**What:** `canary.croft.ing` A/AAAA → box; Caddy (upgraded to official v2.11.4) auto-issues a trusted
production LE cert.
**Prove:**
```
dig +short canary.croft.ing        # 15.204.81.133
curl -sS https://canary.croft.ing/healthz          # ok   (no -k → chain is trusted/prod)
echo | openssl s_client -connect canary.croft.ing:443 -servername canary.croft.ing 2>/dev/null \
  | openssl x509 -noout -issuer -dates              # issuer O=Let's Encrypt (not staging)
```

## Phase 6 — iroh-relay (dev/test relay)  ✅
**What:** off-the-shelf `iroh-relay` v1.0.0 (prebuilt, pinned+sha256), **mode A** — plain HTTP on
`127.0.0.1:8440`, Caddy fronts `relay.croft.ing`. No UDP, no `:443` conflict. Governed + telemetry-
sampled. (Public endpoint pending the `relay.croft.ing` DNS record.)
**Prove:**
```
ssh croft-vps 'systemctl is-active iroh-relay; /opt/iroh-relay/current/iroh-relay --version; \
  curl -s -o /dev/null -w "relay root=%{http_code}\n" http://127.0.0.1:8440/'   # active; 1.0.0; 200
cd croft-stack/relay && bats tests/test_relay_deploy.bats     # 5/5
# once relay.croft.ing DNS is added: curl https://relay.croft.ing/  → 200; point an iroh RelayUrl at it.
```

## Phase 7 — production auth-helper broker (Rust)  ⏳ planned + spike-proven
**What (mechanism proven, build pending):** the confidential-OAuth broker at `account.croft.ing` —
longer-lived brokered sessions for the pads, preferred-when-reachable, degrade-to-browser-only. The
**2026-07-24 spike proved the mechanism GO** (real login, server-side refresh, cross-domain brokered
`whoami` via an opaque ticket, clean fallback). The production build is a Rust rewrite (hardened,
multi-account) — plan: [auth-broker-plan.md](auth-broker-plan.md); spike record:
`discovery/spike/auth-helper/FINDINGS.md`.
**Prove (spike, still live):**
```
curl -s https://account.croft.ing/healthz          # the spike helper (throwaway) still running
# spike findings + the measured TTL table: discovery/spike/auth-helper/FINDINGS.md
```
**Prove (production, when built — per the plan):** `cargo test` (crypto/OAuth units + wiring), a live
login + server-side refresh against a test account, and a pad preferring the broker then falling back
cleanly when it's stopped.

---

## Cross-cutting proofs

- **Reproducible-from-repo:** a clean `git clone` + `ansible-playbook site.yml` reconstructs the box
  (get_url pins + checksums for the relay binary; generated units are committed + deterministic).
- **Idempotent:** re-running `ansible-playbook site.yml` → `changed=0`; `make generate` twice → identical.
- **Governed + observable:** every unit carries cgroup accounting/limits; the telemetry poller records
  per-unit usage (canary, relay, and every system.slice unit).
- **No lockout / hardened:** SSH is key-only, root login off, firewall default-drop (22/80/443 only).
- **Actuals logged:** every session's exact commands (by target LOCAL/OVH-API/BOX/GIT, secrets redacted)
  in `croft-stack/sessions/`.
