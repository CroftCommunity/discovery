#!/usr/bin/env bats
# D14 — the runbook: recovery, rotation, add-a-tenant, the escalation ladder,
# when-to-split, and the "why not" (non-goals) list. Checks: link-check + a
# section-coverage grep, plus shellcheck on the scripts the runbook references.

load helpers

R="$KIT_ROOT/docs/RUNBOOK.md"

@test "link-check passes across kit markdown (RUNBOOK included)" {
  run "$KIT_ROOT/scripts/link-check.sh"
  if [ "$status" -ne 0 ]; then echo "$output"; fi
  [ "$status" -eq 0 ]
}

@test "RUNBOOK covers box loss with recovery points" {
  grep -qi 'box loss' "$R"
  grep -qi 'seconds' "$R"        # canonical sqlite recovery point
  grep -qi '5 min' "$R"          # blobs recovery point
  grep -qi 'zero' "$R"           # disposables by construction
}

@test "RUNBOOK covers cursor loss, litestream alerting via journald, rotation" {
  grep -qi 'cursor loss' "$R"
  grep -qi 'journald' "$R"
  grep -qi 'rotat' "$R"
}

@test "RUNBOOK has ADD-A-TENANT in a small number of steps" {
  grep -qi 'ADD-A-TENANT' "$R"
  # numbered steps present
  grep -qE '^\s*[1-9]\.' "$R"
}

@test "RUNBOOK has the api escalation ladder with triggers" {
  grep -qi 'shared-wal' "$R"
  grep -qi 'snapshot' "$R"
  grep -qi 'second box' "$R"
}

@test "RUNBOOK has WHEN-TO-SPLIT a tenant to its own VPS" {
  grep -qi 'WHEN-TO-SPLIT' "$R"
}

@test "RUNBOOK 'why not' list names every non-goal (section 6)" {
  grep -qi 'why not' "$R"
  for term in 'HA' 'load balancer' 'container' 'orchestrat' 'monitoring' \
              'DNS automation' 'remote Terraform state' 'disposable' \
              'MLS' 'per-tenant VPS' 'discovery'; do
    grep -qi "$term" "$R" || { echo "missing non-goal: $term"; return 1; }
  done
}

@test "scripts referenced by the runbook are shellcheck-clean" {
  run bash -c 'shellcheck "$1"/scripts/*.sh "$1"/bootstrap/*.sh "$1"/drill/*.sh' _ "$KIT_ROOT"
  if [ "$status" -ne 0 ]; then echo "$output"; fi
  [ "$status" -eq 0 ]
}
