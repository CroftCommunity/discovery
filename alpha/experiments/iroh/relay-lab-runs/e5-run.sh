#!/usr/bin/env bash
# E5 — cgroup per-tenant group accounting + isolation (node-1, fat box).
# Two relay "tenants" each in its own transient systemd service (own cgroup with
# CPU+memory accounting). Drive ASYMMETRIC passthrough load and show (1) per-slice
# CPU/RSS accurately bills each tenant, (2) a CPU cap on one tenant isolates it
# without starving the other.
set -u
BIN=/mnt/data/croft-iroh/relay-loadtest/rl-target/debug/relay-loadtest
IP=172.31.43.122
CG=/sys/fs/cgroup/system.slice
log(){ echo "[$(date +%H:%M:%S)] $*"; }

cleanup(){
  sudo systemctl stop relayA.service relayB.service 2>/dev/null
  sudo fuser -k 2120/udp 2121/udp 2>/dev/null
  pkill -f 'relay-loadtest generate' 2>/dev/null
}
trap cleanup EXIT
cleanup; sleep 1

start_relay(){ # unit cpuquota https quic metrics
  local unit=$1 quota=$2 https=$3 quic=$4 metrics=$5
  sudo systemd-run --unit=$unit -p CPUAccounting=1 -p MemoryAccounting=1 -p CPUQuota=$quota \
    $BIN relay --advertise-ip $IP --https-port $https --quic-port $quic --http-port $((https-3)) --metrics-port $metrics \
    >/dev/null 2>&1
}

cpu_usec(){ awk '/usage_usec/{print $2}' $CG/$1.service/cpu.stat 2>/dev/null; }
rss(){ cat $CG/$1.service/memory.current 2>/dev/null; }

# ---- bring up two tenants (generously capped so neither caps during accounting) ----
log "starting relayA (CPUQuota=300%) + relayB (CPUQuota=300%)"
start_relay relayA 300% 3343 3478 9090
start_relay relayB 300% 3353 3488 9091
sleep 4

# responders, one homed per tenant relay
nohup $BIN responder --bind 0.0.0.0:2120 --relay-url https://$IP:3343 --quic-port 3478 >/tmp/respA.log 2>&1 &
nohup $BIN responder --bind 0.0.0.0:2121 --relay-url https://$IP:3353 --quic-port 3488 >/tmp/respB.log 2>&1 &
sleep 5
RA=$(grep -o 'RESPONDER_ADDR=.*' /tmp/respA.log | head -1 | cut -d= -f2-)
RB=$(grep -o 'RESPONDER_ADDR=.*' /tmp/respB.log | head -1 | cut -d= -f2-)
if [ -z "$RA" ] || [ -z "$RB" ]; then log "FAIL: responder addr missing"; cat /tmp/respA.log /tmp/respB.log; exit 1; fi
log "responders up"

# ---- RUN 1: asymmetric load, accounting fidelity ----
A0=$(cpu_usec relayA); B0=$(cpu_usec relayB)
log "RUN1 asymmetric: tenantA=40 conns x 32MiB (heavy), tenantB=4 conns x 1MiB (light)"
nohup $BIN generate --relay-url https://$IP:3343 --quic-port 3478 --responder-addr "$RA" --mode passthrough --count 40 --concurrency 16 --bytes 33554432 --hold-secs 2 >/tmp/genA.log 2>&1 &
GA=$!
nohup $BIN generate --relay-url https://$IP:3353 --quic-port 3488 --responder-addr "$RB" --mode passthrough --count 4 --concurrency 4 --bytes 1048576 --hold-secs 2 >/tmp/genB.log 2>&1 &
GB=$!
wait $GA $GB
sleep 1
A1=$(cpu_usec relayA); B1=$(cpu_usec relayB)
RA_RSS=$(rss relayA); RB_RSS=$(rss relayB)
log "RUN1 tenantA cpu_delta_usec=$((A1-A0)) rss=$RA_RSS"
log "RUN1 tenantB cpu_delta_usec=$((B1-B0)) rss=$RB_RSS"
echo "GENA: $(grep ESTABLISHED /tmp/genA.log)"
echo "GENB: $(grep ESTABLISHED /tmp/genB.log)"

# ---- RUN 2: isolation — cap tenantA to 50%, hammer BOTH equally, B must be unaffected ----
log "RUN2 isolation: set tenantA CPUQuota=50%, drive BOTH 30 conns x 32MiB"
sudo systemctl set-property --runtime relayA.service CPUQuota=50% 2>/dev/null
sleep 2
A2=$(cpu_usec relayA); B2=$(cpu_usec relayB)
T2=$(date +%s.%N)
nohup $BIN generate --relay-url https://$IP:3343 --quic-port 3478 --responder-addr "$RA" --mode passthrough --count 30 --concurrency 16 --bytes 33554432 --hold-secs 2 >/tmp/genA2.log 2>&1 &
GA2=$!
nohup $BIN generate --relay-url https://$IP:3353 --quic-port 3488 --responder-addr "$RB" --mode passthrough --count 30 --concurrency 16 --bytes 33554432 --hold-secs 2 >/tmp/genB2.log 2>&1 &
GB2=$!
wait $GA2 $GB2
T3=$(date +%s.%N)
A3=$(cpu_usec relayA); B3=$(cpu_usec relayB)
WALL=$(echo "$T3-$T2"|bc)
log "RUN2 wall=${WALL}s"
log "RUN2 tenantA(capped 50%) cpu_delta_usec=$((A3-A2)) cores=$(echo "scale=2;($A3-$A2)/1000000/$WALL"|bc)"
log "RUN2 tenantB(uncapped)   cpu_delta_usec=$((B3-B2)) cores=$(echo "scale=2;($B3-$B2)/1000000/$WALL"|bc)"
echo "GENA2: $(grep ESTABLISHED /tmp/genA2.log)"
echo "GENB2: $(grep ESTABLISHED /tmp/genB2.log)"
log "E5 done"
