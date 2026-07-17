#!/usr/bin/env bats
# D8 — a reusable deploy workflow (workflow_call) plus a server-side receive
# script that only ever stages/activates a KNOWN service and restarts that one
# unit. No arbitrary command execution through the deploy key.

load helpers

RECV() { "$KIT_ROOT/scripts/deploy-receive.sh" "$@"; }

setup() {
  ROOT="$(mktemp -d)"
  RLOG="$ROOT/restart.log"
  export DEPLOY_ROOT="$ROOT/opt"
  export DEPLOY_SERVICES="svc-a svc-b"
  # fake restart: record the unit(s) asked to restart, restart nothing real
  export RESTART_CMD="$ROOT/fake-restart.sh"
  cat > "$RESTART_CMD" <<EOF
#!/usr/bin/env bash
echo "\$1" >> "$RLOG"
EOF
  chmod +x "$RESTART_CMD"
  mkdir -p "$DEPLOY_ROOT/svc-a/incoming"
  echo "artifact-v1" > "$DEPLOY_ROOT/svc-a/incoming/binary"
}
teardown() { rm -rf "$ROOT"; }

@test "workflow passes actionlint" {
  if ! command -v actionlint >/dev/null 2>&1; then skip "actionlint not installed"; fi
  run actionlint "$KIT_ROOT/.github/workflows/deploy-service.yml"
  if [ "$status" -ne 0 ]; then echo "$output"; fi
  [ "$status" -eq 0 ]
}

@test "deploy-receive.sh is shellcheck-clean" {
  run shellcheck "$KIT_ROOT/scripts/deploy-receive.sh"
  if [ "$status" -ne 0 ]; then echo "$output"; fi
  [ "$status" -eq 0 ]
}

@test "unknown service is rejected" {
  run RECV activate not-a-service
  [ "$status" -ne 0 ]
  [[ "$output" == *"unknown"* || "$output" == *"not allowed"* ]]
}

@test "arbitrary command is rejected" {
  run RECV "rm -rf /"
  [ "$status" -ne 0 ]
}

@test "activate does an atomic swap into releases/ and points current at it" {
  run RECV activate svc-a
  [ "$status" -eq 0 ]
  [ -L "$DEPLOY_ROOT/svc-a/current" ]
  target="$(readlink "$DEPLOY_ROOT/svc-a/current")"
  [ -f "$target/binary" ]
  run cat "$DEPLOY_ROOT/svc-a/current/binary"
  [ "$output" = "artifact-v1" ]
}

@test "activate restarts ONLY that service unit" {
  RECV activate svc-a
  run cat "$RLOG"
  [ "$output" = "svc-a.service" ]
}
