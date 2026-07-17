#!/usr/bin/env bats
# D16 — the run summary and registers are present and consistent.

DIR="$(cd "$BATS_TEST_DIRNAME/.." && pwd)"
EXP="$DIR/.."            # alpha/experiments
ROOT="$EXP/../.."        # discovery root

@test "appview-infra README (experiment-style summary) exists" {
  [ -f "$DIR/README.md" ]
  grep -qi 'goal' "$DIR/README.md"
  grep -qi 'result' "$DIR/README.md"
}

@test "RUN-15-SUMMARY covers the required sections" {
  S="$EXP/RUN-15-SUMMARY.md"
  [ -f "$S" ]
  grep -qi 'red' "$S" && grep -qi 'green' "$S"     # commit table
  grep -qi 'component' "$S"                        # grade vocabulary
  grep -qi 'local-rehearsal' "$S"
  grep -qi 'BLOCKED' "$S"                          # blocked items
  grep -q 'SPEC-DELTA' "$S"                        # spec-delta rows
  grep -qi 'decision' "$S"                         # D11 decision request restated
  grep -qi 'Phase 1.5' "$S"
  grep -qi 'Phase 2' "$S"
}

@test "RUN-15-SUMMARY names every deliverable D1..D15" {
  S="$EXP/RUN-15-SUMMARY.md"
  for d in D1 D2 D3 D4 D5 D6 D7 D8 D9 D10 D11 D12 D13 D14 D15; do
    grep -q "\b$d\b" "$S" || { echo "missing $d"; return 1; }
  done
}

@test "all six RUN-15 SPEC-DELTAs are in the divergence register" {
  R="$EXP/SPEC-DIVERGENCE-REGISTER.md"
  for id in run15-stub-verifier run15-local-root run15-tf-validate \
            run15-bootstrap-dryrun run15-sandbox-unshare run15-usermode; do
    grep -q "$id" "$R" || { echo "missing register row: $id"; return 1; }
  done
}

@test "EXPERIMENT-BACKLOG and MASTER-INDEX have RUN-15 rows" {
  grep -q 'RUN-15' "$EXP/EXPERIMENT-BACKLOG.md"
  grep -q 'RUN-15' "$EXP/MASTER-INDEX.md"
}
