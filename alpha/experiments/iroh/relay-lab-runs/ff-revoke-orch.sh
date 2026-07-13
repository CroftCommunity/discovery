#!/usr/bin/env bash
# Workstream C — carry a REAL threshold-signed revoke SignedOp over the faithful iroh-gossip wire,
# verified with the green-real gov::meets_threshold_by_lineage on receipt (retires the MD-G5 MAC).
# Origin on node-1 writes its addr ~3s in; joiner on node-2 bootstraps off it. Both broadcast all
# vectors and adjudicate what they receive; per the known gossip gotcha we read the JOINER's SUMMARY.
set -u
KEY=~/Downloads/chase-sandbox-one.pem
SSH="ssh -i $KEY -o StrictHostKeyChecking=no -o ServerAliveInterval=10"
N1=54.172.175.109; N2=34.207.146.151
BIN=/mnt/data/croft-iroh/iroh/crates/altdrive-spike-faithful-sync/ff-target/debug/altdrive-spike-faithful-sync
GROUP=faithful-revoke-g1
OUTDIR=/Users/cpettet/git/chasemp/CroftC/experiments/iroh/relay-lab-runs/C-faithful-revoke-2026-06-17
mkdir -p "$OUTDIR"; RESULTS="$OUTDIR/results.txt"; : > "$RESULTS"
log(){ echo "[$(date +%H:%M)] $*" | tee -a "$RESULTS"; }

cleanup(){ $SSH ubuntu@$N1 "sudo fuser -k 2112/udp 2>/dev/null; true" >/dev/null 2>&1; \
           $SSH ubuntu@$N2 "sudo fuser -k 2112/udp 2>/dev/null; true" >/dev/null 2>&1; }
trap cleanup EXIT
cleanup; sleep 1

log "origin on node-1"
$SSH ubuntu@$N1 "sudo fuser -k 2112/udp 2>/dev/null; cd /tmp; ( setsid $BIN alice $GROUP /tmp/ff_n1.addr >/tmp/ff_n1.log 2>&1 </dev/null & )"
sleep 12
$SSH ubuntu@$N1 "test -f /tmp/ff_n1.addr && echo ADDR_OK || echo NO_ADDR" | tee -a "$RESULTS"
$SSH ubuntu@$N1 "cat /tmp/ff_n1.addr" > /tmp/ff_n1.addr.local
$SSH ubuntu@$N2 "cat > /tmp/ff_n1.addr" < /tmp/ff_n1.addr.local

log "joiner on node-2 (bootstraps off node-1)"
$SSH ubuntu@$N2 "sudo fuser -k 2112/udp 2>/dev/null; cd /tmp; ( setsid $BIN bob $GROUP /tmp/ff_n2.addr /tmp/ff_n1.addr >/tmp/ff_n2.log 2>&1 </dev/null & )"

log "running gossip rounds (~42s)…"
sleep 60
log "=== node-2 (joiner) SUMMARY ==="
$SSH ubuntu@$N2 "grep -E "SUMMARY|revoke-authority|vector=|Neighbor|error|group="  /tmp/ff_n2.log | tail -20" | tee "$OUTDIR/node2.log" | tee -a "$RESULTS"
log "=== node-1 (origin) SUMMARY (may be partial — gotcha) ==="
$SSH ubuntu@$N1 "grep -E 'SUMMARY|revoke-authority' /tmp/ff_n1.log | tail -8" | tee "$OUTDIR/node1.log" | tee -a "$RESULTS"
log "done; artifacts in $OUTDIR"
