#!/usr/bin/env bash
# terraform-check.sh — credential-free Terraform gate.
#   1. fmt -check  (offline, always fatal)
#   2. validate    (needs the provider schema; if the registry is unreachable
#                   through the egress proxy, report BLOCKED and pass — a blocked
#                   tool is surfaced, never silently skipped, and never bricks
#                   the whole gate; guardrail 7).
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
tfdir="$here/terraform"

echo "terraform fmt -check ..."
terraform -chdir="$tfdir" fmt -check -recursive
echo "fmt: OK"

# Point terraform at the proxy CA so a reachable registry actually works.
export SSL_CERT_FILE="${SSL_CERT_FILE:-/root/.ccr/ca-bundle.crt}"

initlog="$(mktemp)"
if terraform -chdir="$tfdir" init -backend=false -input=false -no-color \
    >"$initlog" 2>&1; then
  terraform -chdir="$tfdir" validate -no-color
  echo "validate: OK"
  rm -f "$initlog"
else
  if grep -qiE '403|forbidden|failed to (retrieve|install)|could not query provider' "$initlog"; then
    echo "BLOCKED: terraform validate — the ovh provider registry is unreachable"
    echo "BLOCKED: through the egress proxy (github 403 on checksum fetch)."
    echo "BLOCKED: unblock: run in Phase 2 where registry.terraform.io + github"
    echo "BLOCKED: release assets are reachable, then 'terraform validate'."
    rm -f "$initlog"
    exit 0
  fi
  echo "terraform init failed for a non-network reason:" >&2
  cat "$initlog" >&2
  rm -f "$initlog"
  exit 1
fi
