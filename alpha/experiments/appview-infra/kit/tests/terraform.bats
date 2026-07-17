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
  for v in plan_code ovh_endpoint datacenter ssh_public_key; do
    run grep -A4 "variable \"$v\"" "$KIT_ROOT/terraform/variables.tf"
    [ "$status" -eq 0 ]
    # no 'default' line in the variable block
    run bash -c "grep -A6 'variable \"$v\"' '$KIT_ROOT/terraform/variables.tf' | grep -q 'default'"
    [ "$status" -ne 0 ]
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
