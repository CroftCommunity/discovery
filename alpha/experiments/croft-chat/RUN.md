# Running the integrated Drystone CLI

## Single node (interactive TUI)

```sh
RUST_LOG=croft_chat=debug cargo run -p croft-chat
```

Tab toggles focus (group tree ↔ message input); Up/Down move the tree cursor;
Enter selects a group/channel or sends; Esc / q (in the tree) / Ctrl-C quits.

## Headless ops (scripting / smoke)

```sh
croft-chat --store store.redb exec create-group        # prints the group id (hex)
croft-chat --store store.redb exec send <group_hex> "hello"
croft-chat --store store.redb exec timeline <group_hex>
croft-chat --store store.redb exec list
```

Each `exec` op opens the store, acts, and exits — so persistence is across
*process restarts*, which `tests/binary_smoke.rs` verifies.

## Two-node convergence over iroh-gossip (Milestone C gate)

Built behind the `iroh-it` feature (so default builds pull none of iroh's tree):

```sh
cargo build -p croft-chat --features iroh-it
```

The automated proof is the in-process test (two endpoints, real gossip). It runs
**direct-only over loopback** (`RelayChoice::LocalDirect`), so it is hermetic — no
relay, no Internet — and passes anywhere, including a sandbox or CI:

```sh
cargo test -p croft-chat --features iroh-it --test iroh_convergence -- --nocapture
```

### Same-host recipe (localhost testbed — no relay, no Internet)

Two separate `serve` **processes** on one host, converging over real iroh-gossip
across `127.0.0.1` with no relay dependency. Uses `croft-chat/localhost.toml`
(`relay_mode = "disabled"`), which selects the direct-only endpoint path. This is
the mode to use where Internet UDP / the n0 relays are unreachable.

```sh
BIN=target/debug/croft-chat; W=$(mktemp -d); T=croft-chat/localhost.toml

# Node 1 (creator): binds, publishes its addr, creates the group + enrolls local-2.
$BIN --store "$W/n1.redb" serve --topology "$T" --node local-1 \
  --addr-out "$W/n1.json" --create --send "from local-1" --run-seconds 26 &
until [ -s "$W/n1.json" ]; do sleep 0.5; done   # wait for the published address

# Node 2 (joiner): bootstraps from n1.json over loopback, sends, converges.
$BIN --store "$W/n2.redb" serve --topology "$T" --node local-2 \
  --addr-out "$W/n2.json" --peer "$W/n1.json" --send "from local-2" --run-seconds 22 &
wait
```

**The gate:** both processes print the **same** `fingerprint <hash> (pending 0,
settled)` and the same two-message timeline. Verified in this environment
(2026-07-13): both nodes → `fingerprint 503af2f0895c9b2d`, timeline
`1: from local-2` / `4: from local-1`, each logging `NeighborUp` for the other —
no relay, no Internet, entirely on loopback.

**What this unblocks:** multi-process convergence, fault injection (X2 — `SIGKILL`
a node then heal), and M1 fan-out (N local `serve` processes) all run here now.
Only X1 (real-NAT traversal) still needs the boxes, because a relay-holepunch path
cannot exist where Internet UDP is blocked.

### Cross-host recipe (the secroute boxes)

Topology is `stone-alpha.toml` (node names, seeds, reachability). The public box
`secroute-testing-one` is the bootstrap + group creator; the workstation
(`node4-workstation`, NAT) joins via the box's address. Only the creator's
`EndpointAddr` JSON needs exchanging — peers reach it, and the n0 relay covers the
NAT path. **SG opens UDP 2112 only**; iroh's QUIC/relay works within that.

**On `secroute-testing-one`** (creates the group, enrolls every topology node,
publishes its address, runs for 10 min):

```sh
croft-chat --store n1.redb serve \
  --topology stone-alpha.toml --node secroute-testing-one \
  --addr-out n1.json --create --run-seconds 600
# prints: group <hex>, addr written to n1.json, and a final `fingerprint <hash>`
```

**Copy `n1.json` to the workstation** (out-of-band — `scp`, paste, etc.):

```sh
scp secroute-testing-one:~/n1.json ./n1.json
```

**On the workstation** (bootstraps from `n1.json`, sends a message, converges):

```sh
croft-chat --store n4.redb serve \
  --topology stone-alpha.toml --node node4-workstation \
  --addr-out n4.json --peer n1.json --send "hello from workstation" \
  --run-seconds 120
# prints a final `fingerprint <hash>` and the converged timeline
```

**The gate:** the `fingerprint <hash>` printed on both nodes is **equal**, and
both timelines list the same messages. This is invariant I5 (order-insensitive
convergence) demonstrated over the real network path.

SSH note (driving the boxes): launch the long-running `serve` with the
detached-subshell pattern so it survives the SSH session, and redirect its output
to a file you fetch — never a top-level remote `&`. See the session memory
`ssh-driving-secroute-sandbox`.

## Four-node convergence (P19, Proof A)

Same as the two-node recipe, with three joiners. The creator
(`secroute-testing-one`) enrolls every topology principal on `--create`, so all
nodes are members. Each node bootstraps from the creator's `n1.json`:

```sh
# creator
croft-chat --store n1.redb serve --topology stone-alpha.toml \
  --node secroute-testing-one --addr-out n1.json --create --send "from node1" --run-seconds 120
# each of secroute-testing-two / node3 / node4-workstation (n1.json copied over):
croft-chat --store nX.redb serve --topology stone-alpha.toml \
  --node <name> --addr-out nX.json --peer n1.json --send "from <name>" --run-seconds 90
```

The gate: all reachable nodes print the **same** `fingerprint <hash>` and the
same four-message timeline. (node3 is internal-only; on the boxes it joins from
the VPC, the workstation pairs with node1/node2 via relay.)

### Verified locally (2026-06-27)

- **Two nodes:** `secroute-testing-one` (creator) + `node4-workstation` (joiner)
  over real iroh-gossip both printed `fingerprint 4c3099b76809a5ff` and the
  converged `1: hello from workstation`.
- **Four nodes:** all four `serve` processes (each sending a distinct message)
  converged to `fingerprint 9c10206e5b8ef5c3` with the same four-message
  timeline — the same code path the cross-host run uses.

The hard-stop-on-contradiction demo (P20) is exercised headlessly by
`tests/contradiction.rs` and surfaced in the TUI as a blocking banner.
