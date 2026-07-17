#!/usr/bin/env bats
# D4 — the Terraform is canonically formatted and (where the provider registry
# is reachable) validates. Credential-free.

load helpers

@test "terraform/ is canonically formatted (fmt -check)" {
  run terraform -chdir="$KIT_ROOT/terraform" fmt -check -recursive
  if [ "$status" -ne 0 ]; then echo "$output"; fi
  [ "$status" -eq 0 ]
}

@test "required variables have NO guessed defaults (owner decisions)" {
  # plan_code, ovh_endpoint, datacenter, ssh_public_key are owner calls: they
  # must be declared without a default so terraform refuses to run unset.
  # Extract each variable's block (from its header to the closing brace at
  # column 0) and assert it has no 'default =' assignment.
  local f="$KIT_ROOT/terraform/variables.tf"
  for v in plan_code ovh_endpoint datacenter ssh_public_key; do
    run awk -v name="$v" '
      $0 ~ "^variable \"" name "\" \\{" {f=1}
      f {print}
      f && /^}/ {exit}
    ' "$f"
    [ "$status" -eq 0 ]
    [ -n "$output" ]
    # no default assignment inside the block
    if echo "$output" | grep -qE '^[[:space:]]*default[[:space:]]*='; then
      fail "owner-decision variable '$v' must not have a default"
    fi
  done
}

@test "provider is pinned and Debian 12 is the image" {
  grep -q 'source *= *"ovh/ovh"' "$KIT_ROOT/terraform/versions.tf"
  grep -Eq 'version *= *"[~>=0-9. ]+"' "$KIT_ROOT/terraform/versions.tf"
  grep -qi 'Debian 12' "$KIT_ROOT/terraform/"*.tf
}

@test "terraform validate runs when the registry is reachable, else BLOCKED" {
  run "$KIT_ROOT/scripts/terraform-check.sh"
  # the script is non-fatal on a blocked registry; it must still exit 0 and
  # say which path it took.
  [ "$status" -eq 0 ]
  [[ "$output" == *"validate: OK"* || "$output" == *"BLOCKED"* ]]
}
