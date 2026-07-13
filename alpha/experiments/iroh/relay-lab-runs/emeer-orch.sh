#!/usr/bin/env bash
# Meer P0+P1 == "E9 Tier-0, made real": an always-on BLIND Tier-0 message mirror.
# relay + meer A on node-1; members + replacement meer B on node-2.
# Proves: (1) a member publishes ENCRYPTED blobs the meer stores by digest, holding no key;
# (2) an offline/behind member syncs through the blind meer and CONVERGES (decrypts locally);
# (3) the meer's own stats show meer_payload_keys_held=0 (blindness, asserted+logged);
# (4) anti-entrenchment: export meer A's encrypted store, import into a replacement meer B, and the
#     member re-homes to B and converges identically (state portability = materially reversible);
# (5) admission: a meer with an allowlist DENIES a non-listed peer.
set -u
KEY=~/Downloads/chase-sandbox-one.pem
SSH="ssh -i $KEY -o StrictHostKeyChecking=no -o ServerAliveInterval=10"
N1=54.172.175.109; N2=34.207.146.151; N1_INT=172.31.43.122; N2_INT=172.31.19.13
BIN=/mnt/data/croft-iroh/relay-loadtest/rl-target/debug/relay-loadtest
RELAY_URL="https://$N1_INT:3343"
OUTDIR=/Users/cpettet/git/chasemp/CroftC/experiments/iroh/relay-lab-runs/E9-meer-tier0-2026-06-17
mkdir -p "$OUTDIR"; RESULTS="$OUTDIR/results.txt"; : > "$RESULTS"
log(){ echo "[$(date +%H:%M)] $*" | tee -a "$RESULTS"; }

cleanup(){ $SSH ubuntu@$N1 "sudo fuser -k 3478/udp 2120/udp 2>/dev/null; true" >/dev/null 2>&1; \
           $SSH ubuntu@$N2 "sudo fuser -k 2121/udp 2>/dev/null; true" >/dev/null 2>&1; }
trap cleanup EXIT
cleanup; sleep 1

log "relay + meer A on node-1"
$SSH ubuntu@$N1 "sudo fuser -k 3478/udp 2120/udp 2>/dev/null; \
  ( setsid $BIN relay --advertise-ip $N1_INT --https-port 3343 --quic-port 3478 --http-port 3340 --metrics-port 9092 >/tmp/mrelay.log 2>&1 </dev/null & )"
sleep 4
$SSH ubuntu@$N1 "( setsid $BIN meer --bind 0.0.0.0:2120 --relay-url $RELAY_URL --quic-port 3478 >/tmp/meerA.log 2>&1 </dev/null & )"
sleep 4
MA=$($SSH ubuntu@$N1 "grep -o 'MEER_READY addr=.*' /tmp/meerA.log | head -1 | sed 's/MEER_READY addr=//'")
if [ -z "$MA" ]; then log "FAIL: no meer A addr"; $SSH ubuntu@$N1 "tail -5 /tmp/meerA.log"; exit 1; fi
echo "$MA" | $SSH ubuntu@$N2 "cat > /tmp/meerA.json"
log "meer A online"

mem(){ $SSH ubuntu@$N2 "$BIN meer-member --relay-url $RELAY_URL --quic-port 3478 $* 2>/dev/null"; }

log "step 1: member-1 publishes 5 encrypted blobs (meer never sees the key)"
mem --meer-addr @/tmp/meerA.json --action publish --count 5 --namespace household-v1 | tee "$OUTDIR/publish.json" | tee -a "$RESULTS"

log "step 2: member-2 (offline-until-now) syncs through the blind meer and converges"
mem --meer-addr @/tmp/meerA.json --action sync --expect 5 | tee "$OUTDIR/sync.json" | tee -a "$RESULTS"

log "step 3: blindness proof — meer A stats"
mem --meer-addr @/tmp/meerA.json --action stats | tee "$OUTDIR/statsA.json" | tee -a "$RESULTS"

log "step 4: anti-entrenchment — export meer A store, import into replacement meer B, re-home + converge"
mem --meer-addr @/tmp/meerA.json --action export --file /tmp/meer-export.json | tee -a "$RESULTS"
$SSH ubuntu@$N2 "sudo fuser -k 2121/udp 2>/dev/null; ( setsid $BIN meer --bind 0.0.0.0:2121 --relay-url $RELAY_URL --quic-port 3478 >/tmp/meerB.log 2>&1 </dev/null & )"
sleep 4
MB=$($SSH ubuntu@$N2 "grep -o 'MEER_READY addr=.*' /tmp/meerB.log | head -1 | sed 's/MEER_READY addr=//'")
if [ -z "$MB" ]; then log "FAIL: no meer B addr"; $SSH ubuntu@$N2 "tail -5 /tmp/meerB.log"; exit 1; fi
echo "$MB" | $SSH ubuntu@$N2 "cat > /tmp/meerB.json"
mem --meer-addr @/tmp/meerB.json --action import --file /tmp/meer-export.json | tee -a "$RESULTS"
log "step 4b: member re-homes to meer B and syncs"
mem --meer-addr @/tmp/meerB.json --action sync --expect 5 | tee "$OUTDIR/sync-rehomed.json" | tee -a "$RESULTS"

log "step 5: admission — meer C with an allowlist denies a non-listed peer"
$SSH ubuntu@$N1 "sudo fuser -k 2122/udp 2>/dev/null; ( setsid $BIN meer --bind 0.0.0.0:2122 --relay-url $RELAY_URL --quic-port 3478 --allow deadbeefdeadbeef >/tmp/meerC.log 2>&1 </dev/null & )"
sleep 4
MC=$($SSH ubuntu@$N1 "grep -o 'MEER_READY addr=.*' /tmp/meerC.log | head -1 | sed 's/MEER_READY addr=//'")
echo "$MC" | $SSH ubuntu@$N2 "cat > /tmp/meerC.json"
# this publish should fail (admission denied); capture the error, don't abort
mem --meer-addr @/tmp/meerC.json --action publish --count 1 2>&1 | tee "$OUTDIR/admission-attempt.txt" | tee -a "$RESULTS" || true
sleep 1
$SSH ubuntu@$N1 "grep -c 'admission DENIED' /tmp/meerC.log" | sed 's/^/meerC admission-denied log lines: /' | tee -a "$RESULTS"

log "meer P0+P1 done. artifacts in $OUTDIR"
