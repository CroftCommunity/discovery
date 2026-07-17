#!/usr/bin/env bash
# no-secrets-check.sh — guardrail 4 made executable.
#
# Fails (exit 1) if any tracked file in the kit carries a secret-shaped VALUE.
# We ban credential *values*, never the environment-variable NAMES that carry
# them (OVH_APPLICATION_KEY, AWS_ACCESS_KEY_ID, LITESTREAM_SECRET_ACCESS_KEY,
# ... are expected to appear in docs and configs as ${NAME} references).
#
# Runs standalone (no discovery repo, no network). Wired into `make check`.
set -euo pipefail

root="${1:-.}"
cd "$root"

# Files to scan: prefer git (fast, respects .gitignore); fall back to find so
# the check also works in an unpacked tarball.
if git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  mapfile -t files < <(git ls-files)
else
  mapfile -t files < <(find . -type f \
    -not -path './.git/*' -not -path './.terraform/*' \
    -not -path './generated/*' -not -path './.local/*' -not -path './drill/.work/*')
fi

# Never scan self (it contains the patterns) or lockfiles/binaries.
skip_re='(scripts/no-secrets-check\.sh|\.tfstate|\.terraform\.lock\.hcl|__pycache__|\.pyc$)'

# Secret-shaped VALUE patterns (extended regex). Each is a real credential
# shape, not a variable name.
patterns=(
  'AKIA[0-9A-Z]{16}'                                        # AWS access key id
  '-----BEGIN ([A-Z ]+ )?PRIVATE KEY-----'                 # PEM private key
  'aws_secret_access_key[[:space:]]*[=:][[:space:]]*["'"'"']?[A-Za-z0-9/+]{40}'
  '(application_secret|consumer_key)[[:space:]]*[=:][[:space:]]*["'"'"']?[0-9a-f]{32}'
  '(secret|password|passwd|api[_-]?key|token)[[:space:]]*[=:][[:space:]]*["'"'"'][A-Za-z0-9/+._-]{24,}["'"'"']'
)

# Allowlist: placeholders and documented non-secret defaults that would
# otherwise trip the generic assignment pattern.
allow_re='(\$\{|<|placeholder|example|CHANGE.?ME|changeit|redacted|xxxx|\.\.\.|env:|ENV\[|from the environment|via environment)'

fail=0
for f in "${files[@]}"; do
  [[ -f "$f" ]] || continue
  [[ "$f" =~ $skip_re ]] && continue
  # skip binary
  if grep -qI . "$f" 2>/dev/null; then :; else continue; fi
  for p in "${patterns[@]}"; do
    while IFS= read -r hit; do
      # strip "file:line:" to test the content against the allowlist
      content="${hit#*:}"; content="${content#*:}"
      if [[ "$content" =~ $allow_re ]]; then continue; fi
      echo "SECRET-SHAPED VALUE: $hit" >&2
      fail=1
    done < <(grep -nEI "$p" "$f" 2>/dev/null || true)
  done
done

if [[ "$fail" -ne 0 ]]; then
  echo "no-secrets-check: FAIL — remove the values above; use \${ENV_VAR} references." >&2
  exit 1
fi
echo "no-secrets-check: OK (${#files[@]} files scanned)"
