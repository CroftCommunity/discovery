# Drystone transport and delivery: iroh-gossip over QUIC

`Status: reference, corroborates Part 2 §6 (gossip overlay, §6.10 HyParView + PlumTree) and §11.9 (delivery-as-race)`

`Substrate: iroh core 1.0.0 (FACTCHECK source of truth), iroh-gossip (separately versioned, pre-1.0)`

`Scope: how peer liveness is actually known, and why the delivery layer is pure transport`

---

## Overview

The spec states at Part 2 §6 that Drystone rides a gossip overlay, and at §11.9 that delivery
is a race: store-and-forward through a swarm where first-to-reach wins, with no per-recipient
acknowledgement. That raises a fair challenge. If gossip has no concept of verified delivery,
how does a node maintain a table of which peers are up or down, and where does that up/down
knowledge come from if not from message receipts?

The answer is a clean split between two planes and two layers. The gossip layer runs a
membership protocol (a control plane) that is separate from the best-effort broadcast of
application messages (a data plane); and the membership facts the control plane records do not
come from the data plane at all, they come from the QUIC transport underneath. Gossip is a
passenger. QUIC does the liveness tracking and only notifies gossip when a peer's connection
state changes. This document details that mechanism and closes with what it means to monitor
at scale.

A version note governs everything below. iroh core (Endpoint, Connection, Router, QUIC with
TLS 1.3, relay, key-based addressing) is wire-and-API-stable at 1.0.0, per the FACTCHECK
source of truth (do not re-verify). The iroh-gossip crate is separately versioned and still
pre-1.0, so the exact spelling of its event variants should be confirmed against the pinned
release. The load-bearing property (gossip carries no per-recipient ack) is version-stable and
does not depend on that spelling.

## Control plane versus data plane

iroh-gossip is not blind flooding. It runs a membership-management protocol, the HyParView plus
PlumTree pairing that Part 2 §6.10 names as the overlay's realization, and it surfaces state
through an event stream. Those events fall into two planes.

The control plane tracks the active neighbors in the local swarm view and their health. This is
where the peer table lives. Two events drive it:

- A neighbor-up event means a node was added to the local routing table (the membership layer).
  It is a statement about local topology, not about any message.

- A neighbor-down event means a node was removed from the local routing table, on timeout or
  disconnect. It is a status update on the local routing table, not a receipt and not a
  confirmation that any specific message reached anyone.

The data plane is the best-effort epidemic broadcast of application messages. There is one
event here:

- A received event means a gossip message arrived. This is the only event about the data
  payload, and it carries no notion of who else did or did not receive that message.

A fourth event, the lagged signal, is easy to misread as a delivery signal but is not one. It
means the local consumer's internal event buffer filled and events were dropped: it is local
flow control (backpressure), a fact about this node's own read speed, not about the network or
about any peer.

The exact variant names (NeighborUp, NeighborDown, Received, Lagged in the pre-1.0 crate)
should be confirmed against the pinned iroh-gossip release, since that crate has not reached its
1.0 wire-and-API freeze. The point that matters is stable regardless of spelling: the events
serve membership and local flow control, and the one data-plane event says nothing about
delivery to anyone else.

Neighbor-down has to exist even though there is no verified delivery, and understanding why
sharpens the whole picture. Without it, a node keeps trying to gossip to dead peers, wasting
bandwidth and CPU, and the spanning-tree and mesh topology decays because a parent cannot prune
a vanished child. Pruning dead neighbors is exactly what keeps the swarm connected and able to
broadcast. So neighbor-down is a membership-health signal that the overlay needs for its own
correctness, which is a different thing from a delivery confirmation.

## Where up and down knowledge comes from

Peer-table state cannot come from application ACKs, because gossip has none. It comes from the
transport layer. iroh-gossip rides on iroh-net, which uses QUIC, and QUIC is where liveness is
actually observed.

The connection, not the message, is what is tracked. Gossip is fire-and-forget for data, but it
does care whether the QUIC connection to a peer exists.

Liveness is observed with QUIC keep-alives, not a custom gossip ping. QUIC is a stateful
transport; when a path goes quiet it sends its own keep-alive frames so the connection does not
idle out, which also defeats NAT idle timeouts. iroh piggybacks on this mechanism for
efficiency and correctness rather than inventing a gossip-level ping.

The connection manager drives the membership events. A neighbor-up fires on a completed QUIC
handshake, when a valid authenticated socket exists. A neighbor-down fires on socket close: an
explicit connection-close, an idle timeout with no heartbeat, or a severed path that returns an
error.

The two layers are decoupled. A broadcast call just iterates the currently-active connections
the network layer provides; if a connection fails, the network layer updates the peer table and
gossip sees the neighbor set change. Gossip does not poll peers and does not confirm delivery.
QUIC does the liveness tracking and notifies gossip only on a state change.

## Why keep-alives are trustable: the crypto-durability point

A natural worry is that if a heartbeat can move a peer from down to up, an attacker could inject
a fake heartbeat. It cannot, and the reason is that trust is not re-established per keep-alive.

The expensive cryptographic work happens once, at connection setup. A TLS 1.3 handshake proves
the peer holds the private key for the dialed identity, derives session keys, and establishes a
secure pipe. Everything sent through that pipe afterward, including the tiny keep-alive frames,
is encrypted and authenticated under those session keys. An injected fake keep-alive lacks the
keys and is rejected.

So keep-alives are trustable and durable not because the crypto re-runs each time, but because
they reuse the verified trust established at the handshake. This buys two things at once:
efficiency, since a keep-alive is a standard tiny QUIC packet with no new handshake; and
durability, since a connection that migrates across networks keeps the same session, and
therefore the same peer-table entry, stable.

## QUIC in context

A few QUIC properties explain why the transport is a good fit for a peer table that must survive
real networks.

Identity over IP: iroh dials an Ed25519 public key (a node or endpoint identity), not an IP
address. QUIC finds a path and keeps it valid even as the physical network changes underneath.

NAT traversal: hole-punching is coordinated within the QUIC handshake, with an encrypted relay
fallback (roughly the tenth of restrictive networks that block direct paths) that is invisible
to the application.

Connection migration: a QUIC connection is keyed by a Connection ID, not by an IP, so a switch
from Wi-Fi to 5G does not drop the session. This is the property Drystone leans on so the gossip
peer table stays stable across a network change rather than churning up and down.

Stream multiplexing: independent streams inside one connection avoid TCP-style head-of-line
blocking, so a stalled file transfer does not freeze real-time gossip.

## What this establishes (and does not)

This establishes that Drystone's delivery layer is pure transport. Peer liveness is a QUIC fact
surfaced to the gossip membership layer, not a message-delivery guarantee; the peer table is
built from connection state, never from message ACKs, because there are none. Ordering is
carried cryptographically in the messages themselves, not inferred from arrival, and the swarm
race is simply first-to-reach-wins (Part 2 §11.9). The neighbor-up and neighbor-down events keep
the overlay's topology healthy; the received event is the only word about a payload; the lagged
signal is local backpressure.

It does not establish an application-level delivery guarantee, because gossip provides none by
design, and it does not turn membership into a consensus problem. The thing to monitor at scale
is commit liveness, not membership consensus (Part 2 §11.8): whether commits are making progress
through the swarm, not whether every node agrees on a single membership view. And it does not
pin the exact iroh-gossip event-enum spelling, which stays [confirm] against the pinned pre-1.0
release even as the no-per-recipient-ack property holds across versions.
