#!/usr/bin/env bash
# deploy-receive.sh — the deploy user's forced command (authorized_keys
# command="/opt/appview-infra/deploy-receive.sh"). It is the ONLY thing the
# deploy key can run, so it constrains every operation:
#
#   rsync --server ... /opt/<service>/incoming/   stage a build (dest validated)
#   activate <service>                            atomic swap + restart THAT unit
#
# Anything else is rejected. A leaked deploy key can therefore only push and
# activate builds for a known service — never run arbitrary commands, never
# touch another service, never restart anything but the one unit.
#
# Env (tests override): DEPLOY_ROOT (default /opt), RESTART_CMD (default
# "systemctl restart"), DEPLOY_SERVICES (space-separated allowlist; falls back
# to directories under DEPLOY_ROOT).
set -euo pipefail

DEPLOY_ROOT="${DEPLOY_ROOT:-/opt}"
RESTART_CMD="${RESTART_CMD:-systemctl restart}"

is_allowed() {
  local s="$1"
  [[ "$s" =~ ^[a-z0-9][a-z0-9-]*$ ]] || return 1
  if [[ -n "${DEPLOY_SERVICES:-}" ]]; then
    local a
    for a in $DEPLOY_SERVICES; do [[ "$a" == "$s" ]] && return 0; done
    return 1
  fi
  [[ -d "$DEPLOY_ROOT/$s" ]]
}

# Command comes from the forced-command context if present, else argv.
if [[ -n "${SSH_ORIGINAL_COMMAND:-}" ]]; then
  read -r -a cmd <<< "$SSH_ORIGINAL_COMMAND"
else
  cmd=("$@")
fi
verb="${cmd[0]:-}"

case "$verb" in
  rsync)
    # Only permit rsync --server writing into an allowed service's incoming/.
    dest="${cmd[${#cmd[@]}-1]}"
    svc=""
    if [[ "$dest" =~ ^"$DEPLOY_ROOT"/([a-z0-9-]+)/incoming/?$ ]]; then
      svc="${BASH_REMATCH[1]}"
    fi
    if [[ -z "$svc" ]] || ! is_allowed "$svc"; then
      echo "deploy-receive: rsync dest not allowed: $dest" >&2
      exit 3
    fi
    exec rsync "${cmd[@]:1}"
    ;;

  activate)
    svc="${cmd[1]:-}"
    if ! is_allowed "$svc"; then
      echo "deploy-receive: unknown / not allowed service: $svc" >&2
      exit 3
    fi
    base="$DEPLOY_ROOT/$svc"
    if [[ ! -d "$base/incoming" ]]; then
      echo "deploy-receive: nothing staged for $svc" >&2
      exit 4
    fi
    ts="$(date +%Y%m%d%H%M%S)"
    rel="$base/releases/$ts"
    mkdir -p "$base/releases"
    mv "$base/incoming" "$rel"
    mkdir -p "$base/incoming"
    ln -sfn "$rel" "$base/current"        # atomic activation
    # restart ONLY this unit
    $RESTART_CMD "$svc.service"
    echo "deploy-receive: activated $svc -> $rel"
    ;;

  *)
    echo "deploy-receive: rejected. Only 'rsync' (staging) and " \
         "'activate <service>' are permitted." >&2
    exit 5
    ;;
esac
