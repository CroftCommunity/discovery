#!/usr/bin/env bash
# E6 — tc netem traffic shaping (orchestrated from the Mac across node-1 + node-2).
# Relay + responder on node-1; generator (relay-only passthrough) on node-2. Shape
# ONLY node-1 -> node-2 egress (prio qdisc + u32 dst filter) so the Mac's SSH is
# never shaped. Measure relay-path RTT + establish under: baseline, +100ms delay,
# 10% loss, 30% loss. Show delivery degrades VISIBLY (RTT/establish rise; QUIC
# rides loss) and never silently. A watchdog clears tc even if the run dies.
set -u
KEY=~/Downloads/chase-sandbox-one.pem
SSH="ssh -i $KEY -o StrictHostKeyChecking=no"
N1=54.172.175.109; N2=34.207.146.151
N1_INT=172.31.43.122; N2_INT=172.31.19.13
DEV=ens5
BIN=/mnt/data/croft-iroh/relay-loadtest/rl-target/debug/relay-loadtest
OUT=/Users/cpettet/git/chasemp/CroftC/experiments/iroh/relay-lab-runs/e6-results.txt
: > $OUT
log(){ echo "[$(date +%H:%M:%S)] $*" | tee -a $OUT; }

tc_clear(){ $SSH ubuntu@$N1 "sudo tc qdisc del dev $DEV root 2>/dev/null; true"; }
tc_shape(){ # netem-args...
  $SSH ubuntu@$N1 "sudo tc qdisc del dev $DEV root 2>/dev/null; \
    sudo tc qdisc add dev $DEV root handle 1: prio && \
    sudo tc qdisc add dev $DEV parent 1:3 handle 30: netem $* && \
    sudo tc filter add dev $DEV protocol ip parent 1:0 prio 3 u32 match ip dst $N2_INT/32 flowid 1:3 && \
    echo shaped:$*"
}

cleanup(){ tc_clear; $SSH ubuntu@$N1 "sudo fuser -k 2130/udp 3478/udp 2>/dev/null; true"; }
trap cleanup EXIT
cleanup; sleep 1

# watchdog on node-1: clear tc after 240s no matter what
$SSH ubuntu@$N1 "nohup bash -c 'sleep 240; sudo tc qdisc del dev $DEV root 2>/dev/null' >/dev/null 2>&1 &"

# relay + responder on node-1
log "starting relay + responder on node-1"
$SSH ubuntu@$N1 "sudo fuser -k 3478/udp 2130/udp 2>/dev/null; \
  nohup $BIN relay --advertise-ip $N1_INT --https-port 3343 --quic-port 3478 --http-port 3340 --metrics-port 9090 >/tmp/e6relay.log 2>&1 & \
  sleep 3; nohup $BIN responder --bind 0.0.0.0:2130 --relay-url https://$N1_INT:3343 --quic-port 3478 >/tmp/e6resp.log 2>&1 & sleep 4; \
  grep -o 'RESPONDER_ADDR=.*' /tmp/e6resp.log | head -1"
RA=$($SSH ubuntu@$N1 "grep -o 'RESPONDER_ADDR=.*' /tmp/e6resp.log | head -1 | cut -d= -f2-")
if [ -z "$RA" ]; then log "FAIL: no responder addr"; exit 1; fi
echo "$RA" > /tmp/e6_ra.json
$SSH ubuntu@$N2 "cat > /tmp/e6_ra.json" < /tmp/e6_ra.json
log "responder up"

parse_json(){ # reads JSON on stdin, prints one summary line
  python3 -c "import sys,json; d=json.load(sys.stdin); r=d['relay']; print(f\"established={d['established']} live_relay={d['live_relay']} failed={d['failed']} establish_ms={d['establish_ms']} relay_rtt_ms p50={r['rtt_ms_p50']:.1f} mean={r['rtt_ms_mean']:.1f} max={r['rtt_ms_max']:.1f}\")"
}

for cond in "baseline|CLEAR" "delay100|delay 100ms" "loss10|loss 10%" "loss30|loss 30%"; do
  name=${cond%%|*}; args=${cond#*|}
  if [ "$args" = "CLEAR" ]; then tc_clear >/dev/null; else tc_shape $args >/dev/null; fi
  sleep 2
  json=$($SSH ubuntu@$N2 "$BIN generate --relay-url https://$N1_INT:3343 --quic-port 3478 --responder-addr @/tmp/e6_ra.json --mode passthrough --count 12 --concurrency 6 --bytes 262144 --settle-ms 2500 --hold-secs 1 2>/dev/null")
  line=$(echo "$json" | parse_json 2>/dev/null || echo "PARSE_FAIL")
  log "=== $name ($args) === $line"
done
tc_clear >/dev/null
log "E6 done"
