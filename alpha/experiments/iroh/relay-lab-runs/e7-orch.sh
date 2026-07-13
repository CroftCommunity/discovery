#!/usr/bin/env bash
# E7 — placement churn / split-brain window (orchestrated from the Mac).
# relay-A on node-1, relay-B on node-2. A single "mobile peer" (responder with a
# PINNED secret => stable endpoint id) is re-homed A->B mid-test. A connector
# resolves the peer by bare id through the controller-assigned relay (E2Connect).
# Shows: (1) pre-churn assign=A connects via A; (2) stale assign=A after the move
# FAILS (no relay mesh — the partition window); (3) post-churn assign=B connects
# via B (churn converges, no endpoint stranded).
set -u
KEY=~/Downloads/chase-sandbox-one.pem
SSH="ssh -i $KEY -o StrictHostKeyChecking=no"
N1=54.172.175.109; N2=34.207.146.151
A_INT=172.31.43.122; B_INT=172.31.19.13
A_URL="https://$A_INT:3343"; B_URL="https://$B_INT:3343"
BIN=/mnt/data/croft-iroh/relay-loadtest/rl-target/debug/relay-loadtest
SECRET=abababababababababababababababababababababababababababababababab
OUT=/Users/cpettet/git/chasemp/CroftC/experiments/iroh/relay-lab-runs/e7-results.txt
: > $OUT
log(){ echo "[$(date +%H:%M:%S)] $*" | tee -a $OUT; }

cleanup(){
  $SSH ubuntu@$N1 "sudo fuser -k 3478/udp 2140/udp 3340/tcp 3343/tcp 2>/dev/null; true"
  $SSH ubuntu@$N2 "sudo fuser -k 3478/udp 2140/udp 3340/tcp 3343/tcp 2>/dev/null; true"
}
trap cleanup EXIT
cleanup; sleep 1

log "start relay-A (node-1) + relay-B (node-2)"
$SSH ubuntu@$N1 "nohup $BIN relay --advertise-ip $A_INT --http-port 3340 --https-port 3343 --quic-port 3478 --metrics-port 9090 >/tmp/e7relayA.log 2>&1 & echo relayA \$!"
$SSH ubuntu@$N2 "nohup $BIN relay --advertise-ip $B_INT --http-port 3340 --https-port 3343 --quic-port 3478 --metrics-port 9090 >/tmp/e7relayB.log 2>&1 & echo relayB \$!"
sleep 4

log "home the mobile peer on relay-A (node-1), pinned secret => stable id"
$SSH ubuntu@$N1 "nohup $BIN responder --bind 0.0.0.0:2140 --relay-url $A_URL --quic-port 3478 --secret $SECRET >/tmp/e7peer.log 2>&1 & echo peer \$!"
sleep 5
PID=$($SSH ubuntu@$N1 "grep -o 'RESPONDER_ADDR=.*' /tmp/e7peer.log | head -1 | cut -d= -f2- | python3 -c 'import sys,json;print(json.load(sys.stdin)[\"id\"])'")
if [ -z "$PID" ]; then log "FAIL: no peer id"; $SSH ubuntu@$N1 "cat /tmp/e7peer.log"; exit 1; fi
log "peer id=$PID"

conn(){ # assign_url label
  $SSH ubuntu@$N1 "$BIN e2-connect --peer-id $PID --assign-relay $1 --relays $A_URL,$B_URL --quic-port 3478 --timeout-secs 12 2>/dev/null" \
    | python3 -c "import sys,json; d=json.load(sys.stdin); print(f\"connected={d['connected']} relay_used={d.get('relay_used')} err={d.get('error')}\")"
}

log "=== PRE-CHURN: assign=A (correct) ==="
log "  $(conn $A_URL preA)"

log "=== CHURN: re-home peer A->B (kill on node-1, restart on node-2, same id) ==="
$SSH ubuntu@$N1 "sudo fuser -k 2140/udp 2>/dev/null; true"
sleep 2
$SSH ubuntu@$N2 "nohup $BIN responder --bind 0.0.0.0:2140 --relay-url $B_URL --quic-port 3478 --secret $SECRET >/tmp/e7peer.log 2>&1 & echo peer-on-B \$!"
sleep 5
PID2=$($SSH ubuntu@$N2 "grep -o 'RESPONDER_ADDR=.*' /tmp/e7peer.log | head -1 | cut -d= -f2- | python3 -c 'import sys,json;print(json.load(sys.stdin)[\"id\"])'")
log "  re-homed peer id=$PID2 (same as before: $([ \"$PID\" = \"$PID2\" ] && echo YES || echo NO))"

log "=== WINDOW: stale assign=A after the move (expect FAIL — peer not on A, no relay mesh) ==="
log "  $(conn $A_URL staleA)"

log "=== POST-CHURN: assign=B (correct, converged) ==="
log "  $(conn $B_URL postB)"
log "E7 done"
