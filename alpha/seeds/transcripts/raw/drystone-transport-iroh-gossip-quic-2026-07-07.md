# Raw transcript: iroh-gossip peer state, QUIC transport, and "no verified delivery", 2026-07-07

`Provenance caveat (PLAYBOOK §4): content-faithful cleaned paste, not a byte-pristine export. For
atproto/iroh/iOS facts the FACTCHECK SoT governs (iroh 1.0.0); the mechanism claims here are the design
reasoning behind Drystone's transport/delivery layer (Part 2 §6, the gossip overlay §6.10, and the
§11.9 delivery-as-race). One correction the user pressed: gossip has no application-level verified
delivery, so peer-table state cannot come from message ACKs; it comes from the transport layer.`

## The question (the user's challenge)

How are peer states used with iroh-gossip if gossip has no concept of verified delivery? And a skeptical
check on the `Event` enum names (NeighborUp, NeighborDown, Received, Lagged): "it would never have a
neighbor-down signal" if there is no delivery confirmation, "so how does it even know either way?"

## The mechanism (the reconciled answer)

**Control plane vs data plane.** iroh-gossip is not blind flooding; it runs membership-management
algorithms (HyParView + PlumTree, the §6.10 realization). Two planes:

- **Control plane (NeighborUp / NeighborDown):** tracks the active neighbors in the local swarm view and
  their health. This is where the peer table lives.

- **Data plane (Received):** the best-effort epidemic broadcast of application messages. No ACK, no retry,
  no guarantee every node receives a given message. `Lagged` is a local-consumer backpressure warning (the
  event buffer filled and events were dropped), not a delivery signal.

**The `Event` variants are accurate and serve membership, not delivery:**

- `NeighborUp` — a node was added to the local routing table (membership layer).

- `NeighborDown` — a node was removed from the local routing table (timeout or disconnect). It is a status
  update on the local routing table, NOT a receipt or confirmation that a specific message reached anyone.

- `Received` — a gossip message arrived (the only event about the data payload).

- `Lagged` — the internal event buffer is full; events were dropped (local flow control).

Why NeighborDown must exist even without verified delivery: without it, a node keeps trying to gossip to
dead peers (wasted bandwidth/CPU), and in the spanning-tree/mesh the topology decays (a "parent" cannot
prune a vanished "child"). Pruning dead neighbors is what keeps the swarm connected and able to broadcast.

## Where the "knowledge" of up/down comes from (the load-bearing correction)

Not from application ACKs. From the **transport layer**: iroh-gossip rides on iroh-net, which uses **QUIC**.

- **The connection, not the message, is what is tracked.** Gossip is fire-and-forget for data, but it does
  care whether the QUIC connection to a peer exists.

- **QUIC keep-alives (heartbeats), not a custom gossip ping.** QUIC is stateful; when the path goes quiet
  it sends its own keep-alive frames so the connection does not idle out (also defeats NAT idle timeouts).
  iroh piggybacks on this for efficiency and correctness rather than inventing a gossip-level ping.

- **NeighborUp/Down are driven by the connection manager:** NeighborUp on a completed QUIC handshake (a
  valid, authenticated socket exists); NeighborDown on socket close (explicit CONNECTION_CLOSE, idle
  timeout with no heartbeat, or a severed path returning an error).

- **Decoupling:** `gossip.broadcast(...)` just iterates the currently-active connections the network layer
  provides; if a connection fails, the network layer updates the peer table and gossip sees the neighbor
  set change. Gossip is a passenger on the transport layer; QUIC does the liveness tracking and only
  notifies gossip on state change.

## The crypto-durability point (the user's follow-up)

Trust is NOT re-established per keep-alive. The expensive cryptographic work (a **TLS 1.3 handshake**,
proving the peer holds the private key for the dialed identity) happens **once** at connection setup,
deriving session keys and a "secure pipe." Everything through that pipe, including the tiny keep-alive
frames, is encrypted and authenticated under those session keys, so an injected fake keep-alive lacks the
keys and is rejected. So keep-alives are "trustable and durable" not because crypto re-runs each time, but
because they reuse the verified trust established at the handshake. Result: efficiency (a keep-alive is a
standard tiny QUIC packet, no new handshake) plus durability (connection migration across networks keeps
the session and the peer-table entry stable).

## QUIC, in context (background the user asked for)

- **Identity over IP:** iroh dials an Ed25519 public key (a Node/Endpoint ID), not an IP; QUIC finds and
  keeps the path valid even as the physical network changes.

- **NAT traversal:** hole-punching coordinated within the QUIC handshake (QUIC-NAT-traversal), with an
  encrypted relay fallback (~10% of restrictive networks) invisible to the app.

- **Connection migration:** a QUIC connection is keyed by a Connection ID, not an IP, so a Wi-Fi->5G switch
  does not drop the session (the property Drystone leans on to keep gossip peer state stable). Multipath is
  possible.

- **Stream multiplexing:** independent streams inside one connection, no TCP-style head-of-line blocking (a
  stalled file transfer does not freeze real-time gossip).

- **QUIC background:** born at Google (Jim Roskind, ~2012) to cut latency, integrate TLS, and move
  transport logic out of the OS kernel into the application (fast iteration); standardized by the IETF
  (name is not an acronym anymore); rebuilds TCP's reliability on UDP with 0-RTT handshakes, mandatory
  encryption, connection migration, and multiplexing. Middlebox reality: because it is UDP, some networks
  block it, so browsers race a TCP and a QUIC connection and fall back to TCP transparently.

Design takeaway for Drystone: the delivery layer is pure transport (Part 2 §6, §11.9). Ordering is carried
cryptographically in the messages, not inferred from arrival; the store-and-forward/swarm race is
first-to-reach-wins; peer liveness is a QUIC-transport fact surfaced to the gossip membership layer, not a
message-delivery guarantee. "Commit liveness, not membership consensus" is the thing to monitor at scale
(§11.8).
