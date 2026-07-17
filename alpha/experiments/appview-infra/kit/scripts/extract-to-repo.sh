#!/usr/bin/env bash
# extract-to-repo.sh <target-dir> [--commit <sha>]
#
# Produce the standalone CroftCommunity/appview-infra content from THIS kit/
# subtree: root = kit contents. A clean copy (history is noted, not carried —
# see docs/EXTRACTION.md for the git-subtree-split alternative). Corpus files
# (GROUPS.md, RUN-15 summary, spec notes, corpus-tests) live OUTSIDE kit/ and so
# are naturally excluded; a generated PROVENANCE.md points back to discovery.
#
# Idempotent for a given target (it clears and rewrites the target).
set -euo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
target="${1:-}"
commit=""
shift || true
while [[ $# -gt 0 ]]; do
  case "$1" in
    --commit) commit="$2"; shift 2 ;;
    *) echo "usage: extract-to-repo.sh <target-dir> [--commit <sha>]" >&2; exit 2 ;;
  esac
done
[[ -n "$target" ]] || { echo "usage: extract-to-repo.sh <target-dir> [--commit <sha>]" >&2; exit 2; }

# Resolve the source commit for provenance (best-effort; may be outside a repo).
if [[ -z "$commit" ]]; then
  commit="$(git -C "$KIT" rev-parse HEAD 2>/dev/null || echo UNKNOWN)"
fi

rm -rf "$target"
mkdir -p "$target"

# Copy the kit contents to the target root, excluding build/scratch noise.
# A tar pipe gives a clean, deterministic tree without needing rsync.
tar -C "$KIT" \
  --exclude='./.git' \
  --exclude='./.local' \
  --exclude='__pycache__' \
  --exclude='*.pyc' \
  --exclude='./drill/.work' \
  --exclude='*.tfstate' --exclude='*.tfstate.*' --exclude='./terraform/.terraform' \
  -cf - . | tar -C "$target" -xf -

# Generate PROVENANCE.md at the target root.
cat > "$target/PROVENANCE.md" <<EOF
# Provenance

This repository was **extracted from \`CroftCommunity/discovery\`** at commit
\`$commit\` by \`scripts/extract-to-repo.sh\` (RUN-15, the appview-infra hosting
kit).

The **design corpus stays in discovery** and is intentionally NOT copied here:

- \`GROUPS.md\` — the access-gated large-group tier brief (D11), incl. the
  Variant A vs B write-path fork analysis and the owner decision request.
- \`RUN-15-SUMMARY.md\` — the run summary (red/green table, grades, SPEC-DELTAs).
- the spec-facing note staged in discovery's
  \`beta/drystone-spec/proposed-changes-2026-07-experiment-reconciliation.md\`
  and \`beta/impl/experiments/drystone-reviews-and-experiments-log.md\`.

To trace a decision back to its design, read those in
\`CroftCommunity/discovery\` under \`alpha/experiments/appview-infra/\`.
EOF

echo "extracted kit -> $target (source commit $commit)"
echo "  root files: $(find "$target" -maxdepth 1 -type f | wc -l), dirs: $(find "$target" -maxdepth 1 -mindepth 1 -type d | wc -l)"
