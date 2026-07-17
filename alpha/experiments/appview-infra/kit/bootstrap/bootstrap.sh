#!/usr/bin/env bash
# bootstrap.sh — bring a fresh Debian 12 VPS to a serving state, idempotently.
#
#   bootstrap.sh --plan   dry run: print every intended action, change nothing
#   bootstrap.sh --apply  do it (root; a fresh Debian 12 box)
#
# Idempotent: every apply step checks current state first, so a second --apply is
# a no-op exiting 0. In THIS environment we only exercise --plan (not a fresh
# Debian box, must not be mutated); real double-apply is Phase 2 (P2-3).
# SPEC-DELTA[run15-bootstrap-dryrun | stand-in]
#
# shellcheck disable=SC2016
#   Several `bash -c '...'` payloads are single-quoted on purpose: their `$vars`
#   (e.g. $f in the SSH step) must expand on the target at run time, not at
#   parse time here. Vars that must expand now use double quotes throughout.
set -euo pipefail

MODE=""
case "${1:-}" in
  --plan)  MODE=plan ;;
  --apply) MODE=apply ;;
  *) echo "usage: bootstrap.sh --plan|--apply" >&2; exit 2 ;;
esac

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
KIT="$(cd "$HERE/.." && pwd)"
# shellcheck source=/dev/null
source "$HERE/versions.env"

# do_or_plan "human description" -- cmd args...   (run in apply, print in plan)
# guard "human description" -- test_cmd... -- change_cmd...  (idempotent apply)
do_or_plan() {
  # $1 = description; rest after '--' = command
  local desc="$1"; shift
  [[ "$1" == "--" ]] && shift
  if [[ "$MODE" == plan ]]; then
    echo "PLAN: $desc"
  else
    echo "APPLY: $desc"
    "$@"
  fi
}
guard() {
  # guard "desc" -- <test-cmd...> -- <apply-cmd...>
  local desc="$1"; shift; shift   # drop leading --
  local test_cmd=() apply_cmd=() seen=0
  for a in "$@"; do
    if [[ "$a" == "--" && $seen -eq 0 ]]; then seen=1; continue; fi
    if [[ $seen -eq 0 ]]; then test_cmd+=("$a"); else apply_cmd+=("$a"); fi
  done
  if [[ "$MODE" == plan ]]; then
    echo "PLAN: $desc (skipped if already present)"
    return 0
  fi
  if "${test_cmd[@]}" >/dev/null 2>&1; then
    echo "SKIP: $desc (already present)"
  else
    echo "APPLY: $desc"
    "${apply_cmd[@]}"
  fi
}

echo "== appview-infra bootstrap ($MODE) =="
echo "== expected OS: $EXPECTED_OS; litestream $LITESTREAM_VERSION; rclone $RCLONE_VERSION =="

# --- 0. sanity (apply only) -------------------------------------------------
if [[ "$MODE" == apply ]]; then
  [[ "$(id -u)" -eq 0 ]] || { echo "must run as root" >&2; exit 1; }
fi

# --- 1. base packages -------------------------------------------------------
do_or_plan "apt-get update && install base (curl ca-certificates nftables unattended-upgrades sqlite3 python3 debian-keyring debian-archive-keyring apt-transport-https)" \
  -- bash -c 'apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y curl ca-certificates nftables unattended-upgrades sqlite3 python3 debian-keyring debian-archive-keyring apt-transport-https'

# --- 2. unattended-upgrades -------------------------------------------------
do_or_plan "enable unattended-upgrades (automatic security updates)" \
  -- bash -c 'dpkg-reconfigure -f noninteractive unattended-upgrades'

# --- 3. SSH hardening -------------------------------------------------------
do_or_plan "SSH hardening: PermitRootLogin no, PasswordAuthentication no, reload ssh" \
  -- bash -c '
    f=/etc/ssh/sshd_config.d/10-appview-hardening.conf
    printf "PermitRootLogin no\nPasswordAuthentication no\nKbdInteractiveAuthentication no\n" > "$f"
    systemctl reload ssh || systemctl reload sshd'

# --- 4. firewall (nftables 22/80/443) ---------------------------------------
do_or_plan "install nftables ruleset: default drop, allow established + 22 (ssh) + 80 (http) + 443 (https)" \
  -- bash -c '
    cat > /etc/nftables.conf <<'"'"'NFT'"'"'
#!/usr/sbin/nft -f
flush ruleset
table inet filter {
  chain input {
    type filter hook input priority 0; policy drop;
    ct state established,related accept
    iif "lo" accept
    ct state invalid drop
    ip protocol icmp accept
    ip6 nexthdr ipv6-icmp accept
    tcp dport 22 accept
    tcp dport 80 accept
    tcp dport 443 accept
  }
  chain forward { type filter hook forward priority 0; policy drop; }
  chain output  { type filter hook output priority 0; policy accept; }
}
NFT
    systemctl enable nftables
    nft -f /etc/nftables.conf'

# --- 5. Caddy from the official apt repo ------------------------------------
guard "add Caddy official apt repo + install caddy" \
  -- test -x /usr/bin/caddy \
  -- bash -c '
    curl -1sLf "https://dl.cloudsmith.io/public/caddy/stable/gpg.key" | gpg --dearmor -o /usr/share/keyrings/caddy-stable-archive-keyring.gpg
    curl -1sLf "https://dl.cloudsmith.io/public/caddy/stable/debian.deb.txt" > /etc/apt/sources.list.d/caddy-stable.list
    apt-get update && apt-get install -y caddy'

# --- 6. Litestream (pinned .deb) --------------------------------------------
guard "install litestream $LITESTREAM_VERSION (pinned)" \
  -- test -x /usr/bin/litestream \
  -- bash -c "curl -fsSL -o /tmp/litestream.deb https://github.com/benbjohnson/litestream/releases/download/v${LITESTREAM_VERSION}/litestream-v${LITESTREAM_VERSION}-linux-amd64.deb && dpkg -i /tmp/litestream.deb"

# --- 7. rclone (pinned) -----------------------------------------------------
guard "install rclone $RCLONE_VERSION (pinned)" \
  -- test -x /usr/bin/rclone \
  -- bash -c "curl -fsSL -o /tmp/rclone.deb https://downloads.rclone.org/v${RCLONE_VERSION}/rclone-v${RCLONE_VERSION}-linux-amd64.deb && dpkg -i /tmp/rclone.deb"

# --- 8. per-manifest users --------------------------------------------------
# A dedicated system user per service, plus a read-only <name>-api user in the
# service's group (so the sidecar can read StateDirectory 0750 files).
while IFS=$'\t' read -r name user api_user _port _apiport _artifact _apimode; do
  [[ -z "$name" ]] && continue
  guard "create system user '$user' (service)" \
    -- id -u "$user" -- useradd --system --home "/var/lib/$name" --shell /usr/sbin/nologin "$user"
  if [[ "$api_user" != "-" ]]; then
    guard "create system user '$api_user' (api sidecar), add to group '$user'" \
      -- id -u "$api_user" -- useradd --system --home "/var/lib/$name" --shell /usr/sbin/nologin --groups "$user" "$api_user"
  fi
done < <(python3 "$KIT/scripts/list-services.py" "$KIT/services")

# --- 9. deploy user (D8 receives releases via forced-command rsync) ---------
guard "create 'deploy' user (forced-command rsync target, D8)" \
  -- id -u deploy -- useradd --system --home /home/deploy --create-home --shell /bin/bash deploy

# --- 10. install generated/ configs ----------------------------------------
do_or_plan "install generated systemd units -> /etc/systemd/system/, caddy vhosts -> /etc/caddy/conf.d/, litestream.yml -> /etc/litestream.yml" \
  -- bash -c "
    install -d /etc/caddy/conf.d
    cp $KIT/generated/systemd/*.service $KIT/generated/systemd/*.timer /etc/systemd/system/ 2>/dev/null || true
    cp $KIT/generated/caddy/*.caddy /etc/caddy/conf.d/
    grep -q 'import conf.d/\\*.caddy' /etc/caddy/Caddyfile 2>/dev/null || echo 'import conf.d/*.caddy' >> /etc/caddy/Caddyfile
    cp $KIT/generated/litestream.yml /etc/litestream.yml
    systemctl daemon-reload"

# --- 11. enable + start units ----------------------------------------------
do_or_plan "systemctl enable --now: each <name>.service, <name>-api.service, litestream, blob + snapshot timers, caddy" \
  -- bash -c "
    systemctl enable --now caddy
    while IFS=\$'\t' read -r name user api_user _p _ap _art _am; do
      [[ -z \"\$name\" ]] && continue
      systemctl enable --now \"\$name.service\"
      [[ \"\$api_user\" != '-' ]] && systemctl enable --now \"\$name-api.service\"
      for t in $KIT/generated/systemd/\$name-blob-*.timer $KIT/generated/systemd/\$name-snapshot.timer; do
        [[ -e \"\$t\" ]] && systemctl enable --now \"\$(basename \"\$t\")\"
      done
      systemctl enable litestream || true
    done < <(python3 $KIT/scripts/list-services.py $KIT/services)"

echo "== bootstrap ($MODE) complete =="
[[ "$MODE" == plan ]] && echo "== plan only; nothing changed =="
exit 0
