#!/usr/bin/env bash
# RUN-09 Part 5 — repeated-run fan-out. Run the FANOUT-M1 sweep K times at N=2/4/8/16,
# capturing every node's metrics line per run for per-N spread (min/median/max).
set -uo pipefail
# Run from the croft-chat crate root (where scripts/ and fanout-data/ live).
cd "$(dirname "$0")/.."
BIN=${BIN:-target/debug/croft-chat}
export RUST_LOG=warn
K=${K:-5}
RUN_SECONDS=${RUN_SECONDS:-30}
DATA="$(pwd)/fanout-data"
TMP=${TMP:-$(mktemp -d)}
mkdir -p "$DATA"
CSV="${CSV:-$DATA/repeated-run09.csv}"
echo "run,N,node,live_sent,resync_sent,received,head_ms,converged_ms,fp_unique,fp_settled" > "$CSV"

for run in $(seq 1 "$K"); do
  for N in 2 4 8 16; do
    OUT="$TMP/fanout-r${run}-n${N}"
    BIN="$BIN" RUN_SECONDS="$RUN_SECONDS" OUT="$OUT" ./scripts/fanout-measure.sh "$N" > "$OUT.stdout" 2>&1
    # fingerprint distinctness/settled
    fp_unique=$(grep -h "^fingerprint" "$OUT"/n${N}-node*.log 2>/dev/null | awk '{print $2}' | sort -u | wc -l)
    fp_settled=$(grep -h "^fingerprint" "$OUT"/n${N}-node*.log 2>/dev/null | grep -c "settled")
    for i in $(seq 1 "$N"); do
      line=$(grep -h "^metrics" "$OUT/n${N}-node${i}.log" 2>/dev/null | tail -1)
      ls=$(echo "$line" | grep -oE "live_sent=[0-9]+" | cut -d= -f2)
      rs=$(echo "$line" | grep -oE "resync_sent=[0-9]+" | cut -d= -f2)
      rc=$(echo "$line" | grep -oE "received=[0-9]+" | cut -d= -f2)
      hm=$(echo "$line" | grep -oE "head_ms=[0-9]+" | cut -d= -f2)
      cm=$(echo "$line" | grep -oE "converged_ms=[0-9]+" | cut -d= -f2)
      echo "${run},${N},${i},${ls:-NA},${rs:-NA},${rc:-NA},${hm:-NA},${cm:-NA},${fp_unique:-NA},${fp_settled:-NA}" >> "$CSV"
    done
    echo "[run ${run}/${K} N=${N}] fp_unique=${fp_unique} fp_settled=${fp_settled}"
  done
done
echo "DONE-REPEATED-RUN csv=$CSV"
