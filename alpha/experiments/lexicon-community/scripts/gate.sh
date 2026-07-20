#!/usr/bin/env bash
# RUN-LEX-01 one-command gate: the whole spine, green or red in one shot.
#   ./scripts/gate.sh
set -euo pipefail
cd "$(dirname "$0")/.."

echo "== EXP-LEX-01/02/03/04 — Rust suite (red-first acceptance criteria) =="
cargo test --no-fail-fast

echo
echo "== EXP-LEX-01 — official @atproto/lexicon gate (validator of record) =="
if [ ! -d scripts/node_modules ]; then
  (cd scripts && npm install --no-audit --no-fund >/dev/null 2>&1)
fi
node scripts/gate.mjs

echo
echo "== EXP-LEX-03 — one-command attendance-attestation demo =="
cargo run --quiet --example demo_attendance

echo
echo "GATE COMPLETE"
