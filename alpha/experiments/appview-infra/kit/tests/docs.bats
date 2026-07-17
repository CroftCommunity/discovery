#!/usr/bin/env bats
# D9 — DNS doc + the markdown link-checker (reused by D11/D14/D15 docs).

load helpers

@test "link-check passes across all kit markdown" {
  run "$KIT_ROOT/scripts/link-check.sh"
  if [ "$status" -ne 0 ]; then echo "$output"; fi
  [ "$status" -eq 0 ]
}

@test "link-check FAILS on a broken relative link (self-test)" {
  d="$(mktemp -d)"
  printf '# t\n[missing](./nope.md)\n' > "$d/a.md"
  run "$KIT_ROOT/scripts/link-check.sh" "$d"
  [ "$status" -ne 0 ]
  rm -rf "$d"
}

@test "DNS.md covers A/AAAA per service+api, HTTP-01, _lexicon pending, Porkbun API" {
  f="$KIT_ROOT/docs/DNS.md"
  grep -qi 'AAAA' "$f"
  grep -q 'stellin-staging.croft.ing' "$f"
  grep -q 'api.stellin-staging.croft.ing' "$f"
  grep -q 'groups-staging.croft.ing' "$f"
  grep -qi 'HTTP-01' "$f"
  grep -q '_lexicon' "$f"
  grep -qi 'Porkbun' "$f"
  grep -qi 'API' "$f"
}
