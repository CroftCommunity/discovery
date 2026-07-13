#!/usr/bin/env bash
# E10 — RoQ-under-netem: synthetic CBR media over iroh QUIC datagrams, the C1 probe.
# Relay on node-1; roq-recv on node-2; roq-send on node-1. Media flows node-1 -> node-2,
# and we shape ONLY node-1 -> node-2 egress (prio qdisc + u32 dst filter) so the Mac's SSH
# path is never shaped. Per condition: fresh recv, 20s sender, collect BOTH summaries.
# Conditions: baseline, +100ms delay, 5% loss, 30% loss, rate-cap below source (the
# "do the two controllers fight" detector). Watchdog clears tc no matter what.
set -u
KEY=~/Downloads/chase-sandbox-one.pem
SSH="ssh -i $KEY -o StrictHostKeyChecking=no -o ServerAliveInterval=10"
N1=54.172.175.109; N2=34.207.146.151
N1_INT=172.31.43.122; N2_INT=172.31.19.13
DEV=ens5
BIN=/mnt/data/croft-iroh/relay-loadtest/rl-target/debug/relay-loadtest
RELAY_URL="https://$N1_INT:3343"
KBPS=${KBPS:-64}; FRAME_MS=${FRAME_MS:-20}; DUR=${DUR:-20}
OUTDIR=/Users/cpettet/git/chasemp/CroftC/experiments/iroh/relay-lab-runs/E10-roq-netem-2026-06-17
mkdir -p "$OUTDIR"
RESULTS="$OUTDIR/results.txt"; : > "$RESULTS"
log(){ echo "[$(date +%H:%M)] $*" | tee -a "$RESULTS"; }

tc_clear(){ $SSH ubuntu@$N1 "sudo tc qdisc del dev $DEV root 2>/dev/null; true" >/dev/null 2>&1; }
tc_shape(){ # netem-args...
  $SSH ubuntu@$N1 "sudo tc qdisc del dev $DEV root 2>/dev/null; \
    sudo tc qdisc add dev $DEV root handle 1: prio && \
    sudo tc qdisc add dev $DEV parent 1:3 handle 30: netem $* && \
    sudo tc filter add dev $DEV protocol ip parent 1:0 prio 3 u32 match ip dst $N2_INT/32 flowid 1:3 && \
    echo shaped:$*" >/dev/null 2>&1
}
cleanup(){ tc_clear; $SSH ubuntu@$N2 "sudo fuser -k 2114/udp 2>/dev/null; true" >/dev/null 2>&1; \
           $SSH ubuntu@$N1 "sudo fuser -k 3478/udp 2>/dev/null; true" >/dev/null 2>&1; }
trap cleanup EXIT
cleanup; sleep 1

# watchdog: clear tc after 300s regardless
$SSH ubuntu@$N1 "( setsid bash -c 'sleep 300; sudo tc qdisc del dev $DEV root 2>/dev/null' >/dev/null 2>&1 </dev/null & )"

log "starting relay on node-1"
$SSH ubuntu@$N1 "sudo fuser -k 3478/udp 2>/dev/null; \
  ( setsid $BIN relay --advertise-ip $N1_INT --https-port 3343 --quic-port 3478 --http-port 3340 --metrics-port 9091 >/tmp/e10relay.log 2>&1 </dev/null & )"
sleep 4
$SSH ubuntu@$N1 "grep -o 'RELAY_READY.*' /tmp/e10relay.log | head -1" | tee -a "$RESULTS"

run_condition(){
  local name="$1"; shift
  local args="$*"
  log "=== condition: $name ($args) ==="
  if [ "$args" = "CLEAR" ]; then tc_clear; else tc_shape $args; fi
  sleep 1
  # fresh receiver on node-2
  $SSH ubuntu@$N2 "sudo fuser -k 2114/udp 2>/dev/null; \
    ( setsid $BIN roq-recv --bind 0.0.0.0:2114 --relay-url $RELAY_URL --quic-port 3478 --idle-ms 4000 >/tmp/e10recv.log 2>&1 </dev/null & )"
  sleep 4
  local RA
  RA=$($SSH ubuntu@$N2 "grep -o 'ROQ_RECV_ADDR=.*' /tmp/e10recv.log | head -1 | cut -d= -f2-")
  if [ -z "$RA" ]; then log "FAIL: no recv addr for $name"; $SSH ubuntu@$N2 "tail -5 /tmp/e10recv.log"; return 1; fi
  echo "$RA" | $SSH ubuntu@$N1 "cat > /tmp/e10_ra.json"
  # sender on node-1 (foreground), captures sender JSON
  $SSH ubuntu@$N1 "$BIN roq-send --relay-url $RELAY_URL --quic-port 3478 \
    --recv-addr @/tmp/e10_ra.json --kbps $KBPS --frame-ms $FRAME_MS --duration-secs $DUR 2>/tmp/e10send.err" \
    > "$OUTDIR/$name.send.json"
  sleep 5
  $SSH ubuntu@$N2 "sudo fuser -k 2114/udp 2>/dev/null; true" >/dev/null 2>&1
  sleep 1
  # the recv summary is the last JSON object in its log
  $SSH ubuntu@$N2 "awk '/^\{/{f=1} f{print}' /tmp/e10recv.log" > "$OUTDIR/$name.recv.json"
  # one-line digest
  python3 - "$OUTDIR/$name.send.json" "$OUTDIR/$name.recv.json" "$name" <<'PY' | tee -a "$RESULTS"
import json,sys
s=json.load(open(sys.argv[1]));
try: r=json.load(open(sys.argv[2]))
except Exception: r={}
name=sys.argv[3]
print(f"{name}: sent={s['sent']} send_err={s['send_errors']}(buf={s['err_buffer_full']},big={s['err_too_large']}) "
      f"achieved={s['achieved_kbps']:.1f}kbps rtt[min/mean/max]={s['rtt_ms_min']:.0f}/{s['rtt_ms_mean']:.0f}/{s['rtt_ms_max']:.0f}ms "
      f"max_dg={s['max_datagram_size']} | recv: got={r.get('received','?')} loss={r.get('loss_pct','?') if not isinstance(r.get('loss_pct'),float) else round(r['loss_pct'],1)}% "
      f"jitter={round(r['jitter_ms'],1) if isinstance(r.get('jitter_ms'),(int,float)) else '?'}ms gaps={r.get('gap_events','?')} maxburst={r.get('max_consecutive_loss','?')} goodput={round(r['goodput_kbps'],1) if isinstance(r.get('goodput_kbps'),(int,float)) else '?'}kbps")
PY
}

run_condition baseline CLEAR
run_condition delay100 delay 100ms
run_condition loss5    loss 5%
run_condition loss30   loss 30%
run_condition rate40   rate 40kbit
run_condition delay100-loss5 delay 100ms loss 5%

tc_clear
log "E10 done. artifacts in $OUTDIR"
