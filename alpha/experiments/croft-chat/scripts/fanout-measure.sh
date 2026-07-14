#!/usr/bin/env bash
# A4 / M1 fan-out measurement (EXP-1).
#
# Brings up N local `serve` processes that converge over REAL iroh-gossip on
# loopback (no relay, no Internet) and captures, per node:
#   - convergence latency  (`converged_ms`: time to fold the full N-message
#     timeline with nothing pending)
#   - gossip message counts (`live_sent` / `resync_sent` / `received`)
#   - the settled fingerprint (all nodes must match => converged)
#
# Topology (N nodes, seeds 11..10+N) is generated per run. Node local-1 is the
# creator (enrolls every node + sends); local-2..N bootstrap from its address and
# each send one message, so the converged timeline length is exactly N.
#
# Usage:  BIN=target/debug/croft-chat ./scripts/fanout-measure.sh 2 4 8
set -uo pipefail

BIN=${BIN:-target/debug/croft-chat}
RUN_SECONDS=${RUN_SECONDS:-30}
OUT=${OUT:-/tmp/fanout}
export RUST_LOG=${RUST_LOG:-warn}
mkdir -p "$OUT"

run_n() {
  local N=$1
  local W; W=$(mktemp -d)
  local T="$W/topology.toml"
  echo 'relay_mode = "disabled"' > "$T"
  for i in $(seq 1 "$N"); do
    printf '\n[[node]]\nname = "local-%d"\nhost = "127.0.0.1"\nport = 0\nseed = %d\npublic = true\n' \
      "$i" "$((10 + i))" >> "$T"
  done

  # Creator: binds, publishes its address, creates the group, enrolls every node.
  "$BIN" --store "$W/n1.redb" serve --topology "$T" --node local-1 \
    --addr-out "$W/n1.json" --create --send "from local-1" \
    --expect-msgs "$N" --run-seconds "$RUN_SECONDS" > "$OUT/n${N}-node1.log" 2>&1 &
  # Wait for the creator's published address before starting joiners.
  local waited=0
  until [ -s "$W/n1.json" ]; do
    sleep 0.2; waited=$((waited + 1))
    [ "$waited" -gt 100 ] && { echo "N=$N: creator never published addr"; return 1; }
  done

  # Joiners: bootstrap from the creator's address, each send one message.
  for i in $(seq 2 "$N"); do
    "$BIN" --store "$W/n${i}.redb" serve --topology "$T" --node "local-${i}" \
      --addr-out "$W/n${i}.json" --peer "$W/n1.json" --send "from local-${i}" \
      --expect-msgs "$N" --run-seconds "$RUN_SECONDS" > "$OUT/n${N}-node${i}.log" 2>&1 &
  done
  wait

  echo "===================== N=$N ====================="
  for i in $(seq 1 "$N"); do
    printf 'node%-2d ' "$i"
    grep -h "^metrics" "$OUT/n${N}-node${i}.log" | tail -1
  done
  echo "-- fingerprints (must all match) --"
  grep -h "^fingerprint" "$OUT/n${N}"-node*.log | sort | uniq -c
  rm -rf "$W"
}

for N in "$@"; do run_n "$N"; done
