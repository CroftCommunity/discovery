# Validation methods — how the SSH-driven sandbox tests were run

This records *how* the three-node (plus NAT'd Mac) validation was driven over SSH, so future
sessions reproduce it without re-discovering the gotchas. Companion to the per-node transcripts.

## Topology (2026-06-15)

| node | role | public IP | VPC IP / AZ | network position |
|---|---|---|---|---|
| node-one | peer / fetcher | 54.172.175.109 | 172.31.43.122 / us-east-1c | same VPC `vpc-217f0f5c` |
| node-two | provider / broker | 34.207.146.151 | 172.31.19.13 / us-east-1b | same VPC |
| node-three | 3rd peer / broker (NEW) | 3.84.55.217 | 172.31.88.18 / us-east-1a | **same VPC** (not off-VPC) |
| node-four | NAT-traversal peer | this Mac | behind home/office NAT | **off-VPC, real NAT** |

All three AWS boxes share one VPC and one Security Group. node-three landing same-VPC means the
3rd AWS node alone cannot exercise NAT traversal — so the **Mac (node-four), behind a real NAT, is
the off-VPC peer** that forces hole-punch / relay. Its Rust is contained under `experiments/` with
project-local `CARGO_HOME` + `CARGO_TARGET_DIR` (no workstation pollution).

## SSH-driving gotchas (each cost a debugging cycle — use the working pattern)

1. **Run SSH/scp normally — do NOT disable the sandbox.** The sandbox allows network egress (only
   local filesystem writes are restricted), so `ssh`/`scp` work as-is. Disabling the sandbox forces a
   confirmation prompt on every call for no benefit.
2. **Top-level remote `&` kills the session (exit 255, no output).** `ssh host 'cmd &'` tears the
   connection. Working pattern: a **detached subshell inside a foreground session** —
   `( setsid CMD >log 2>&1 </dev/null & )` — survives cleanly. Verify with `ss -lun`.
3. **Tool-level background SSH also dies immediately (255, empty).** Don't background a long-lived
   listener at the tool level; launch it detached (pattern above) and read its logfile later.
4. **`pkill -f <pattern>` self-matches the SSH command line** (the pattern is in the remote argv) and
   suicides the shell → 255. Free resources another way: `sudo fuser -k 2112/udp`.
5. Long-lived *foreground* sessions are fine; add `-o ServerAliveInterval=10` for multi-second runs.

## UDP 2112 reachability test (the SG check)

iroh pins UDP 2112; the Security Group must allow it among all peers. Method used:

```sh
# Receiver (node-two): launch a detached python UDP sink on 2112, verify bound
ssh ... ubuntu@<rx> 'sudo fuser -k 2112/udp; sleep 1; \
  ( setsid python3 /tmp/udpsink.py >/tmp/sink.log 2>&1 </dev/null & ); \
  sleep 2; ss -lun | grep -w 2112'
# Senders (node-one, node-three): fire datagrams over the private path
ssh ... ubuntu@<sender> 'for i in 1 2 3; do echo "probe-$i" | nc -u -w1 <rx> 2112; done'
# Read result
ssh ... ubuntu@<rx> 'cat /tmp/sink.log'
```

`udpsink.py` binds `0.0.0.0:2112`, logs `RX from <ip>:<port> -> <payload>` per datagram, exits after
an idle timeout. **Result 2026-06-15:** node-one (172.31.43.122) and node-three (172.31.88.18) both
delivered 3/3 to node-two (172.31.19.13) → UDP 2112 open across the private path for all three;
the SG was extended to the new node-three.

## Provisioning state observed (2026-06-15)

- **node-two** — fully provisioned: repo `/mnt/data/alt.drive`, cargo on `/mnt/data` EBS, prior spike
  artifacts (`spike-store-provide`, `test-5g.bin`).
- **node-one** — repo dir was gone; re-cloned (`git@github.com:AltID/alt.drive.git` via
  `~/.ssh/id_secroute`). cargo + `target` + `test-5g-received.bin` survived on `/mnt/data`.
- **node-three** — bare new box, 128 GB root (no `/mnt/data` EBS — its root is big enough). Installed
  rustup stable; needed `build-essential` for the C toolchain (iroh native deps).

Transcript collection still excludes `~/.claude/.credentials.json` + MCP auth cache, and is
secret-scanned before commit (same rules as the existing per-node dirs).

## Build / orchestration mechanics learned 2026-06-16 (gossip + proof campaign)

- **cargo is not on the non-login-shell PATH on the boxes.** A plain `ssh box 'cargo …'` reports
  "command not found" even though rust is installed (`~/.cargo/bin`). Drive builds via
  `ssh box 'bash -lc "…"'` or prefix with `source $HOME/.cargo/env`. Observed: node-2 cargo 1.96.0.
- **Reachability (2026-06-16):** all three AWS boxes + the Mac up; capacities node-1 4c/15G
  (`/mnt/data`), node-2 2c/7.7G (`/mnt/data`), node-3 2c/3.8G (no `/mnt/data`), Mac 14c off-VPC/NAT.
- **The experiments/iroh checkout on the boxes is `/mnt/data/croft-iroh/iroh`** (warm 6.3G `target`).
  A **prebuilt gossip binary is already there:**
  `/mnt/data/croft-iroh/iroh/target/debug/altdrive-spike-gossip` — reuse it, no rebuild needed.
  The Mac builds into the contained env `CARGO_TARGET_DIR=.node4-target CARGO_HOME=.node4-cargo`
  under `experiments/iroh` (warm 5.2G; gossip spike incremental build ~14s).
- **The gossip spike runs `ROUNDS=18` (~36s) then exits** — a tight orchestration window. Pattern
  that works: start the origin detached (`nohup … >log 2>&1 &`), poll for its `self_out` addr file
  (written ~2–3s in, to a FILE not stdout → no block-buffer issue), `scp` the addr to the other
  node, start it fast to maximize the broadcast overlap. Both `SUMMARY senders_received={…}` lines
  are the verdict.
- **NAT path works via relay without inbound public UDP.** The spike uses `presets::N0` (n0 public
  relays); the off-VPC NAT'd Mac bootstraps from the box's `EndpointAddr` (relay URL + key, no direct
  IP) and exchanges bidirectionally over the relay. The 3343/3478 ingress gap only blocks
  *hole-punch* (the direct path), not the relay fallback. Versions: iroh `1.0.0-rc.1` / iroh-gossip
  `0.100.0` in experiments/iroh; relay-loadtest pins iroh `=1.0.0`.
- **pgrep/pkill self-match:** `pgrep -af <name>` matches the ssh argv running it (so a `|| echo none`
  guard won't fire). To actually stop a process use `sudo fuser -k 2112/udp` or a systemd unit;
  to check, filter out the pgrep pid.
