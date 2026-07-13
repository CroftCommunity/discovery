#!/usr/bin/env bash
# E10b — the rate-cap CC lever. Same rig as e10-orch, fixed at a 40kbit cap (below the ~84kbit
# wire rate of a 64kbps source), comparing the iroh-default ~1 MiB datagram send buffer (queue +
# pace = bufferbloat) against a small 8 KiB buffer (send_datagram errs = clean drop, bounded
# latency). Longer recv idle (12s) so we capture frames even when bloat delays them by seconds.
set -u
KEY=~/Downloads/chase-sandbox-one.pem
SSH="ssh -i $KEY -o StrictHostKeyChecking=no -o ServerAliveInterval=10"
N1=54.172.175.109; N2=34.207.146.151
N1_INT=172.31.43.122; N2_INT=172.31.19.13
DEV=ens5
BIN=/mnt/data/croft-iroh/relay-loadtest/rl-target/debug/relay-loadtest
RELAY_URL="https://$N1_INT:3343"
DUR=20
OUTDIR=/Users/cpettet/git/chasemp/CroftC/experiments/iroh/relay-lab-runs/E10-roq-netem-2026-06-17
mkdir -p "$OUTDIR"
RESULTS="$OUTDIR/ratecap-results.txt"; : > "$RESULTS"
log(){ echo "[$(date +%H:%M)] $*" | tee -a "$RESULTS"; }

tc_clear(){ $SSH ubuntu@$N1 "sudo tc qdisc del dev $DEV root 2>/dev/null; true" >/dev/null 2>&1; }
tc_rate(){ $SSH ubuntu@$N1 "sudo tc qdisc del dev $DEV root 2>/dev/null; \
    sudo tc qdisc add dev $DEV root handle 1: prio && \
    sudo tc qdisc add dev $DEV parent 1:3 handle 30: netem rate $1 && \
    sudo tc filter add dev $DEV protocol ip parent 1:0 prio 3 u32 match ip dst $N2_INT/32 flowid 1:3" >/dev/null 2>&1; }
cleanup(){ tc_clear; $SSH ubuntu@$N2 "sudo fuser -k 2114/udp 2>/dev/null; true" >/dev/null 2>&1; \
           $SSH ubuntu@$N1 "sudo fuser -k 3478/udp 2>/dev/null; true" >/dev/null 2>&1; }
trap cleanup EXIT
cleanup; sleep 1
$SSH ubuntu@$N1 "( setsid bash -c 'sleep 240; sudo tc qdisc del dev $DEV root 2>/dev/null' >/dev/null 2>&1 </dev/null & )"

log "relay on node-1"
$SSH ubuntu@$N1 "sudo fuser -k 3478/udp 2>/dev/null; \
  ( setsid $BIN relay --advertise-ip $N1_INT --https-port 3343 --quic-port 3478 --http-port 3340 --metrics-port 9091 >/tmp/e10relay.log 2>&1 </dev/null & )"
sleep 4

run(){ # name  extra-send-args...
  local name="$1"; shift
  log "=== $name (rate 40kbit) ==="
  tc_rate 40kbit; sleep 1
  $SSH ubuntu@$N2 "sudo fuser -k 2114/udp 2>/dev/null; \
    ( setsid $BIN roq-recv --bind 0.0.0.0:2114 --relay-url $RELAY_URL --quic-port 3478 --idle-ms 12000 >/tmp/e10recv.log 2>&1 </dev/null & )"
  sleep 4
  local RA; RA=$($SSH ubuntu@$N2 "grep -o 'ROQ_RECV_ADDR=.*' /tmp/e10recv.log | head -1 | cut -d= -f2-")
  if [ -z "$RA" ]; then log "FAIL: no recv addr"; return 1; fi
  echo "$RA" | $SSH ubuntu@$N1 "cat > /tmp/e10_ra.json"
  $SSH ubuntu@$N1 "$BIN roq-send --relay-url $RELAY_URL --quic-port 3478 --recv-addr @/tmp/e10_ra.json \
    --kbps 64 --frame-ms 20 --duration-secs $DUR $* 2>/dev/null" > "$OUTDIR/$name.send.json"
  sleep 14
  $SSH ubuntu@$N2 "sudo fuser -k 2114/udp 2>/dev/null; true" >/dev/null 2>&1; sleep 1
  $SSH ubuntu@$N2 "awk '/^\{/{f=1} f{print}' /tmp/e10recv.log" > "$OUTDIR/$name.recv.json"
}

run rate40-defaultbuf
run rate40-smallbuf --send-buf-bytes 8192

tc_clear
log "done; see $OUTDIR/rate40-*.{send,recv}.json"
