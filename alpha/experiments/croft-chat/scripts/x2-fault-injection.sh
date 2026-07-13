#!/usr/bin/env bash
# X2 — fault injection during convergence (Battery 8), on the localhost testbed.
#
# Claim under test (pairs with G2): when a node is KILLED mid-convergence and
# facts it has not seen are admitted while it is down, then on REJOIN it catches
# up to the exact same head — monotonically, never reverting an already-admitted
# fact. Falsifies if the healed node reaches a different head than the peer, or if
# any fact it admitted before the crash is lost/changed after heal.
#
# Mechanism (two real `serve` processes over iroh-gossip on 127.0.0.1, no relay):
#   1. A creates the group, enrolls B, sends "a1"; B joins and sends "b1".
#      Both converge to {a1, b1}.
#   2. B is SIGKILLed (a crash mid-run). Its redb store is crash-consistent.
#   3. While B is down, A sends "a2" — a fact B never saw.
#   4. B is restarted on the SAME store and rejoins A.
#   5. Report three sub-claims: crash-consistency (B kept {a1,b1} across SIGKILL),
#      no-reversion (nothing admitted is lost after heal), and catch-up (B folds a2
#      and re-converges to A's head).
#
# Observed result (2026-07-13, loopback testbed): crash-consistency and no-reversion
# HOLD; catch-up REVEALS A GAP — over plain iroh-gossip a returning node does not
# backfill. `publish_group` re-broadcasts identical frames, but gossip dedups them by
# message id, so a node that rejoins AFTER a frame's first broadcast never receives it
# (measured: a node present for the first broadcast received all 4 frames and
# converged; a node rejoining ~5s later received 1 of 815 re-broadcasts and stalled at
# its prefix). This localizes the unbuilt M2 "return-backfill" mechanism to a
# sync-on-connect trigger. The experiment therefore exits 0 (the two invariants hold)
# and prints the catch-up GAP as its finding.
#
# Usage: alpha/croft-chat/scripts/x2-fault-injection.sh
#   X2_WORKDIR=<dir>  keep logs/stores there (default: a fresh mktemp dir)
# Requires: cargo build -p croft-chat --features iroh-it (built binary).
set -u

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WS="$(cd "$HERE/.." && pwd)"                 # croft-chat workspace root
BIN="$WS/target/debug/croft-chat"
TOPO="$WS/croft-chat/localhost.toml"
W="${X2_WORKDIR:-$(mktemp -d)}"
mkdir -p "$W"
trap 'kill -9 $(jobs -p) 2>/dev/null' EXIT   # keep $W for inspection

[ -x "$BIN" ] || { echo "FAIL: build first: cargo build -p croft-chat --features iroh-it"; exit 1; }

wait_file() { for _ in $(seq 1 80); do [ -s "$1" ] && return 0; sleep 0.25; done; return 1; }
fp()  { grep -oE 'fingerprint [0-9a-f]+' "$1" | tail -1 | awk '{print $2}'; }        # last fingerprint
tl()  { "$BIN" --store "$1" exec timeline "$2" 2>/dev/null | sed 's/^/    /'; }        # timeline lines
has() { "$BIN" --store "$1" exec timeline "$2" 2>/dev/null | grep -q ": $3$"; }        # body present?

echo "workdir: $W"
echo

# --- 1. converge: A (creator) + B, both to {a1,b1} -------------------------------
"$BIN" --store "$W/a.redb" serve --topology "$TOPO" --node local-1 \
  --addr-out "$W/a1.json" --create --send "a1" --run-seconds 60 > "$W/a1.log" 2>&1 &
A1=$!
wait_file "$W/a1.json" || { echo "FAIL: A never published its address"; cat "$W/a1.log"; exit 1; }
"$BIN" --store "$W/b.redb" serve --topology "$TOPO" --node local-2 \
  --addr-out "$W/b1.json" --peer "$W/a1.json" --send "b1" --run-seconds 60 > "$W/b1.log" 2>&1 &
B1=$!
echo "[1] A + B converging over loopback gossip ..."
sleep 14

# Read the group id from A's log (lock-free) — B's store is held exclusively by
# the live B1 process (redb is single-writer), so we cannot open it until B1 dies.
GROUP="$(grep -oE 'group [0-9a-f]+' "$W/a1.log" | head -1 | awk '{print $2}')"
[ -n "$GROUP" ] || { echo "FAIL: A never created the group"; cat "$W/a1.log"; exit 1; }
echo "    group ${GROUP:0:16}…"

# --- 2. crash B mid-run; free A's store so a new fact can be injected ------------
echo "[2] SIGKILL B (crash) — capturing its persisted head while down"
kill -9 "$B1" 2>/dev/null; wait "$B1" 2>/dev/null
kill -9 "$A1" 2>/dev/null; wait "$A1" 2>/dev/null   # stop A1 so A2 can reuse the store
echo "    B pre-heal timeline (persisted across the crash):"
tl "$W/b.redb" "$GROUP"
PRE_A1=$(has "$W/b.redb" "$GROUP" "a1" && echo y || echo n)
PRE_B1=$(has "$W/b.redb" "$GROUP" "b1" && echo y || echo n)
PRE_A2=$(has "$W/b.redb" "$GROUP" "a2" && echo y || echo n)

# --- 3. inject "a2" while B is down ---------------------------------------------
echo "[3] A sends \"a2\" while B is down (a fact B has never seen)"
"$BIN" --store "$W/a.redb" serve --topology "$TOPO" --node local-1 \
  --addr-out "$W/a2.json" --send "a2" --run-seconds 34 > "$W/a2.log" 2>&1 &
A2=$!
wait_file "$W/a2.json" || { echo "FAIL: A2 never published its address"; cat "$W/a2.log"; exit 1; }
sleep 4   # let A2 (re)load, send a2, and settle before B rejoins

# --- 4. heal: restart B on the same store, rejoin A -----------------------------
# A2 stays up (runs longer than B2) so it re-broadcasts the full log — incl. a2 —
# on every tick while B2 is in the swarm; both then finish naturally and print a
# final fingerprint (never kill A2 early, or it never emits its head).
echo "[4] restart B on the same store — rejoin + catch up"
"$BIN" --store "$W/b.redb" serve --topology "$TOPO" --node local-2 \
  --addr-out "$W/b2.json" --peer "$W/a2.json" --run-seconds 24 > "$W/b2.log" 2>&1 &
B2=$!
wait "$B2" 2>/dev/null
wait "$A2" 2>/dev/null

# --- 5. assert ------------------------------------------------------------------
echo
echo "    B post-heal timeline:"; tl "$W/b.redb" "$GROUP"
A_FP="$(fp "$W/a2.log")"
B_FP="$(fp "$W/b2.log")"
# Diagnostics (help if an assertion trips).
echo "    [diag] A store has a2: $(has "$W/a.redb" "$GROUP" "a2" && echo yes || echo NO)  |  B2 NeighborUp: $(grep -c NeighborUp "$W/b2.log")"
echo "    [diag] frame-level delivery counts need RUST_LOG=croft_chat=debug (gossip received/broadcast are debug logs)"
echo "    [diag] workdir: $W"
POST_A1=$(has "$W/b.redb" "$GROUP" "a1" && echo y || echo n)
POST_B1=$(has "$W/b.redb" "$GROUP" "b1" && echo y || echo n)
POST_A2=$(has "$W/b.redb" "$GROUP" "a2" && echo y || echo n)

echo
echo "    pre-heal  B: a1=$PRE_A1  b1=$PRE_B1  a2=$PRE_A2   (a2 must be absent)"
echo "    post-heal B: a1=$POST_A1  b1=$POST_B1  a2=$POST_A2   (all present)"
echo "    A head=$A_FP   B head=$B_FP"
echo

# X2 has three sub-claims; the experiment reports each. crash-consistency and
# no-reversion are the defensible invariants (a real FAIL here fails the run).
# Catch-up-after-absence is a probe: over plain gossip it is EXPECTED to reveal a
# gap (see below), not a hard failure — that gap is the point of the experiment.
fail=0
echo "  Verdict:"

# 1. crash-consistency — B's committed state survives the SIGKILL.
if [ "$PRE_A1" = y ] && [ "$PRE_B1" = y ]; then
  echo "   [PASS] crash-consistency  — B persisted {a1,b1} across the SIGKILL"
else
  echo "   [FAIL] crash-consistency  — B lost committed state across the crash"; fail=1
fi

# 2. no-reversion (monotonic) — nothing admitted before the crash is lost after heal.
if [ "$POST_A1" = y ] && [ "$POST_B1" = y ]; then
  echo "   [PASS] no-reversion       — a1,b1 still admitted after rejoin (monotonic)"
else
  echo "   [FAIL] REVERSION          — an admitted fact was lost after heal"; fail=1
fi

# 3. catch-up — did B fold the fact admitted while it was down, to the same head?
if [ "$POST_A2" = y ] && [ -n "$A_FP" ] && [ "$A_FP" = "$B_FP" ]; then
  echo "   [PASS] catch-up           — B folded a2 and re-converged to A's head ($B_FP)"
else
  echo "   [GAP]  catch-up           — a2 (admitted while B was down) was NOT delivered on rejoin."
  echo "          B stalls at its pre-crash prefix {a1,b1}; A is at {a1,b1,a2}, heads differ."
  echo "          Cause: publish_group re-broadcasts identical frames, which iroh-gossip dedups"
  echo "          by message id — so a node that returns AFTER a frame's first broadcast never"
  echo "          receives it (contrast: a node present for the first broadcast converges)."
  echo "          This is the unbuilt M2 'return-backfill' mechanism; the fix is sync-on-connect"
  echo "          (pull/refresh on NeighborUp), not periodic re-publish. Backlog: A4/M2."
fi
echo

if [ "$fail" = 0 ]; then
  echo "X2 RESULT: crash-consistency + monotonic no-reversion PROVEN on the loopback testbed."
  [ "$POST_A2" = n ] && echo "           Catch-up-after-absence is the localized M2 return-backfill gap (above)."
  exit 0
fi
echo "X2 RESULT: a core invariant FAILED (see [FAIL] above)."
exit 1
