#!/usr/bin/env bash
# link-check.sh — verify every relative markdown link in the kit resolves.
#
# Scans .md files (given paths/dirs, or the whole kit by default). For each
# inline link [text](target) it skips external (http, https, mailto), anchors
# (#...), and template placeholders, then resolves the target relative to the
# containing file and fails if it does not exist. Offline; no network.
set -euo pipefail

roots=("$@")
if [[ ${#roots[@]} -eq 0 ]]; then
  here="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
  roots=("$here")
fi

mapfile -t files < <(
  for r in "${roots[@]}"; do
    if [[ -f "$r" ]]; then echo "$r"
    else find "$r" -name '*.md' -not -path '*/.git/*' -not -path '*/.local/*'
    fi
  done | sort -u
)

fail=0
link_re='\[[^]]*\]\(([^)]+)\)'
for f in "${files[@]}"; do
  dir="$(dirname "$f")"
  while IFS= read -r line; do
    rest="$line"
    while [[ "$rest" =~ $link_re ]]; do
      target="${BASH_REMATCH[1]}"
      rest="${rest#*"${BASH_REMATCH[0]}"}"
      # skip external / anchor / placeholder links
      case "$target" in
        http://*|https://*|mailto:*|\#*|\<*|\$\{*) continue ;;
      esac
      target="${target%%#*}"          # strip trailing anchor
      target="${target%% *}"          # strip optional title
      [[ -z "$target" ]] && continue
      if [[ "$target" = /* ]]; then
        resolved="$target"            # absolute (rare) — check as-is
      else
        resolved="$dir/$target"
      fi
      if [[ ! -e "$resolved" ]]; then
        echo "BROKEN LINK: $f -> $target" >&2
        fail=1
      fi
    done
  done < "$f"
done

if [[ "$fail" -ne 0 ]]; then
  echo "link-check: FAIL" >&2
  exit 1
fi
echo "link-check: OK (${#files[@]} markdown file(s))"
