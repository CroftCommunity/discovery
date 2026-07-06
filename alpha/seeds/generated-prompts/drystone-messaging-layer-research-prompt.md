# Research brief: Drystone messaging-layer delivery models (MLS over iroh, relay, and DS)

## What I'm doing

I'm designing the messaging/delivery layer for Drystone, a peer-symmetric cooperative-governance
protocol. The substrate is settled: MLS (RFC 9420 / RFC 9750) for group key agreement and message
protection, and iroh (core 1.0 as of June 2026) for transport, discovery, and the gossip overlay.
I want to research, plan, and find a path forward for the delivery models that sit on top of this,
with special attention to the mobile/push problem and to a set of rights constraints that any design
must honor.

I want this to be a working session: research primary sources first, lay out the design space
honestly with tradeoffs, surface disagreements rather than paper over them, and help me converge on
a path. Do not hand me a confident synthesis built from memory. Where a claim is load-bearing,
ground it in a primary source (RFC, spec, iroh/iroh-gossip crate docs) before stating it.

## Grounding rules (important)

- Search and read primary sources BEFORE writing claims, not after. Standards/specs (RFCs, W3C)
  first, then papers, then official project docs and source repos. Blogs are corroboration only.

- iroh reached 1.0 (2026-06-15) with a wire-and-API stability guarantee for CORE (Endpoint /
  Connection / Router / ALPN / QUIC+TLS1.3 transport / relay / key-based addressing). But
  iroh-gossip is a SEPARATE pre-1.0 crate NOT covered by that guarantee, and discovery is split into
  separately-versioned crates (iroh-mainline-address-lookup, iroh-mdns-address-lookup). Treat
  core-stable facts as pinnable; treat gossip/discovery-crate internals as version-dependent and flag
  them. Verify current crate versions and APIs; don't assume.

- Mark every claim by epistemic status. If you can't source a specific (API name, guarantee, number),
  say so and search again rather than asserting it.

- Separate layers explicitly: what the MLS spec guarantees vs what an implementation chooses; what
  iroh core guarantees vs what a gossip/discovery crate happens to do; protocol vs deployment vs
  product.

- Plain-language intuition first, then the precise technical statement, then where the analogy breaks.
  I'm forming mental models for the first time here, so accuracy-before-fluency and
  clarity-before-completeness.

- Formatting: no em-dashes. Blank line between markdown bullets. No run-on sentences.

## Key architectural background you should hold

- Two identity planes, kept strictly separate:
  - PEER identity = iroh EndpointId (the public half of an Ed25519 keypair), the TLS identity of a
    connection. Authenticates a CHANNEL.
  - GROUP identity = an MLS leaf (membership in a scope at an epoch). Authorizes an ACTOR in a scope.
  - The seam: a verified channel from a non-member grants nothing; a member you can't currently reach
    is still a member. Reachability and membership fail independently. In iroh the remote EndpointId
    is known only AFTER the mTLS handshake, so the membership check is necessarily a later,
    application-layer step.

- Two encryption layers, different jobs:
  - Layer A (iroh QUIC/TLS 1.3): hop-by-hop, protects link metadata from a network observer,
    authenticates peer identity. A relay routes encrypted packets by EndpointId and cannot decode
    them; a meer you dial directly IS a TLS endpoint of your connection.
  - Layer B (MLS PrivateMessage): end-to-end content confidentiality/integrity/authenticity to the
    group epoch, plus forward secrecy and post-compromise security across epochs. Survives passing
    through untrusted intermediaries. This is what makes blind relays/meers/DS safe.
  - MLS guarantees do NOT depend on the transport (RFC 9750 §8); MLS holds even against a compromised
    Delivery Service.

- The DS (Delivery Service) in MLS terms: MLS assumes a trusted Authentication Service but a largely
  UNtrusted DS (RFC 9420 §3). A DS can censor (drop/delay) and observe (contact-graph metadata) but
  never forge or decrypt. Drystone splits the DS's two classic functions: it REFUSES the DS ordering
  role (ordering is a timestamp-free causal-cryptographic fold with forks first-class) and KEEPS only
  the store-and-forward function, embodied as the "meer" (a blind store-and-forward node that is never
  issued a key, stores byte-identical sealed PrivateMessage bytes, and is a revocable role held
  redundantly, never a structural dependency).

- Gossip overlay = HyParView (membership; DSN 2007) + PlumTree (eager/lazy broadcast trees; "Epidemic
  Broadcast Trees," SRDS 2007), as implemented by iroh-gossip. Best-effort and probabilistic: it
  repairs transient breaks in a live tree but offers nothing to a node that was entirely offline. So
  gossip cannot be the durability layer; offline durability is the meer's job.

- Returning-member catch-up: a member reports a (G, D) cursor (last-held governance-commit position
  and dataplane position), drains what it missed, verifies forward. Safety property: an incomplete
  catch-up can only UNDER-authorize (act on less), never mis-authorize. Incompleteness is always safe.

## The delivery-style taxonomy I want to investigate

I think there are at least four delivery styles, possibly more. Pressure-test this taxonomy, merge or
split as the evidence warrants, and for each give: how it works, what it guarantees, offline-durability
behavior, metadata/exposure profile, efficiency, failure modes, and which rights constraints it
stresses.

1. Direct dial: iroh direct connection + MLS. Two online peers, hole-punched QUIC, PrivateMessage
   end-to-end. No intermediary holds bytes.

2. MLS over gossip: eventual delivery via the iroh-gossip overlay, with possibly some offline
   durability. Question: how much durability, really, given gossip repairs only live-tree breaks?

3. MLS with a DS / meer: store-and-forward for offline members, with possible efficiency gains (fan-out
   from one well-connected node rather than N direct dials). Blind store-and-forward only.

4. Opportunistic / adaptive: MLS over iroh that blends direct, gossip, and DS/meer modes, adapting to
   connectivity. Picks the best available path and degrades gracefully.

For each, I care about: does it preserve the two-plane separation; can the intermediary be taken
offline or removed from the loop without loss of function or standing; what exactly degrades when it's
gone.

## IP-to-Endpoint translation strategies (a specific sub-investigation)

iroh dials EndpointId (a key), not an IP. I want the discovery/translation strategies mapped out:
DNS/Pkarr (n0-operated, a soft center for observation), Pkarr-on-mainline-DHT (center-free, global),
mDNS (center-free, local/airgapped), and direct EndpointAddr with cached dialing hints. For each:
who can observe or withhold lookups, integrity model (records are self-signed by the endpoint key),
latency/reliability tradeoffs, and what it costs to remove the center. Also: how these interact with
each delivery style above (e.g. does a DS/meer need stable discovery; can a relay double as a discovery
aid).

## The mobile / push-notification problem (the hard part)

This is the part I most need to think through. The facts and constraints:

- On mobile, especially iOS, a backgrounded app cannot hold a live connection. Delivering a message
  requires a push notification (APNs for iOS, FCM for Android) to wake the app. Push requires a host
  with the resources and the credentials to send it.

- Open questions I want investigated, with primary sources where possible:
  - Can a push be triggered by a NON-MLS-group-member iroh-gossip node? (i.e. a node that participates
    in the overlay and can SEE that a sealed message exists for an EndpointId, without being a group
    member who can read it.) What would such a node need: APNs/FCM credentials, a device-token
    registry, awareness of which EndpointId maps to which push token?
  - Can DS-aligned functionality (the meer) BE the push trigger, since it already holds sealed messages
    for offline members and is already a delivery path / already aware that messages exist? In how many
    of the four delivery styles is "something already knows a message is waiting" true? (I think at
    least two: gossip-node-aware and DS/meer-aware.)
  - What is the minimal trusted/resourced role here? I want a "push-resource host" that can be a blind
    intermediary: it knows "EndpointId X has a waiting sealed message, send X a push," but it cannot
    read content (Layer B) and ideally learns as little contact-graph metadata as possible. Map the
    metadata it unavoidably learns (device token <-> EndpointId binding is inherently identifying).
  - APNs/FCM specifics that matter: payload size limits, whether an encrypted/opaque payload can ride
    the push or whether it's just a wake signal, silent/background push reliability and throttling on
    iOS, what Apple/Google actually allow. Pull current primary docs.

- The design stance I'm starting from (challenge it if it's wrong):
  - Adapting the architecture to be functional on a specific platform is FINE, as long as it (a) does
    not compromise the rights imperatives, and (b) CAN take the resource host offline and/or remove it
    from the loop. Platform-specific accommodation is acceptable; platform-induced structural
    dependency is not.
  - The push-resource host is a resource-asymmetry role, like the meer and relay: a convenience held
    by whoever has the resources (an always-on device, a willing well-resourced member), revocable and
    ideally redundant, never a structural requirement and never an authority.
  - When push cannot be sent (iOS background restrictions, a power-sensitive device that disallows
    background work), the affected device lives with a degraded mode (it catches up on next foreground
    via the (G, D) cursor). That degradation is acceptable and expected. I frame this as an
    unequal-resources issue: the device that withholds the resource accepts the lesser mode.

## Rights imperatives any design MUST satisfy (the non-negotiables)

These come from Drystone Part 1. A delivery design that violates these is wrong no matter how efficient.

- Tenure, voice, exit (the rights floor): no delivery role may become a gate on a member's standing,
  ability to participate, or ability to leave with history.

- No structural dependency on any single non-member intermediary (relay, meer, gossip node, push host).
  The no-helper path must stay real and exercised. Removing the helper costs convenience, never
  function or standing.

- Blind intermediaries only: any non-member role (relay, meer, push host) sees ciphertext and routing
  metadata at most, never content. Map the metadata each role unavoidably learns and treat that as the
  cost to minimize.

- Incompleteness must be safe: any catch-up or adaptive path that misses messages may under-authorize,
  never mis-authorize.

## What I want out of this session

1. A pressure-tested delivery-style taxonomy (the 4 above, refined), each with guarantees, durability,
   exposure, efficiency, failure modes, and rights-stress.

2. A clear answer, grounded in APNs/FCM and iroh-gossip primaries, on whether a non-member gossip node
   and/or a DS/meer can serve as the push-resource host, and what each would have to know and hold.

3. A minimal-role definition for the push-resource host that fits the blind-intermediary,
   revocable-redundant-helper pattern, with its unavoidable-metadata cost stated honestly.

4. The IP-to-Endpoint discovery strategies mapped against the delivery styles.

5. A recommended adaptive/opportunistic model that picks paths by connectivity and degrades correctly
   on mobile/iOS, with the degradation modes named.

6. An explicit list of what stays [confirm] against pinned crate/spec versions (iroh-gossip internals,
   APNs/FCM current limits, Pkarr record model) so I know the residue before any implementation.

Start by confirming the current state of: iroh core 1.0 transport/relay APIs, iroh-gossip's current
crate version and whether a non-subscribed node can observe message presence for a topic, and the
current APNs and FCM constraints on background/silent push and payload. Then lay out the design space.
