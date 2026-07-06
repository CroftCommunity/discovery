# Delta Chat as comparative prior art: what it informs in our design

`Status: research finding`

`Purpose: assess whether Delta Chat's iroh + push architecture should influence the delivery spec`

`Companion to: 01-delivery-architecture.md`

---

## Verification legend

*Verified*: checked against a Delta Chat primary this round (their blog, their Rust API docs, their forum).

*Synthesis*: our own comparative reasoning, labeled as such.

**[confirm]**: not yet pinned to a primary.

---

## 1. The headline: Delta Chat is the inverse stack, which is exactly why it is useful

The single most important framing fact, because it governs which lessons transfer: **Delta Chat is email/SMTP-native, with iroh as a realtime add-on.** Their durable substrate is "chatmail" (SMTP servers); iroh provides ephemeral realtime channels on top. Delta Chat now establishes private Peer-to-Peer gossipping networks between users who start a webxdc app that uses the new joinRealtimeChannel() API. The realtime layer is explicitly ephemeral: ephemeral realtime application updates are only distributed between devices actively running a webxdc app that called the joinRealtime API.

Our stack is the reverse: MLS-over-iroh is the *substrate*, and durability/store-and-forward (the meer) is the thing we add. So Delta Chat is not a template; it is a **natural experiment in the opposite arrangement**, which makes its seams informative. Where they had to bolt realtime onto a durable email core, we are bolting durability onto a realtime P2P core. Their pain points are a map of the seam we are approaching from the other side.

*Synthesis.* This inversion is the lens for everything below. A choice that is forced for them (durability via email) is a choice we make differently (durability via the meer), and a choice that is a clean fit for us (MLS as the content plane) is something they do not have at the realtime layer at all.

---

## 2. What Delta Chat independently confirms for us (grounded)

These are points where their shipping implementation validates a claim we were carrying as design reasoning or `[confirm]`.

### 2.1 iroh-gossip needs an app-level sequence number, because gossip dedup is by content

Their actual implementation carries a per-message field whose documented purpose is: This is attached to every message to work around iroh_gossip deduplication. Their `Iroh` struct holds `sequence_numbers: Mutex<HashMap<TopicId, i32>>` with the comment Sequence numbers for gossip channels, and a `get_and_incr` method that issues them.

This is direct, shipping confirmation of two things we reasoned to:

- Our **content-hash dedup** (the race-both combinator, design §3.5) is real and necessary: iroh-gossip dedups on message identity, so two byte-identical messages collapse to one. Delta Chat had to actively *defeat* this for their realtime case (they want repeated identical payloads to count as distinct events), which is the mirror image of our situation (we *want* the dedup, for cross-path delivery). Either way, the dedup-by-content behavior is confirmed in a real deployment, not just in the proto docs.

- Our suspicion in **E1.1 / standalone-D-swarm** (that the application must layer its own sequence metadata because the gossip layer does not expose per-message ordering or hole-detection) is corroborated: Delta Chat adds its own `sequence_numbers` per topic precisely because the gossip layer does not give them a usable application-level sequence. *Synthesis from their source plus our E1.1 hypothesis.* This raises our prior that E1.1 will come back "application must layer its own metadata," and that standalone-D-swarm hole detection cannot lean on stock gossip.

### 2.2 The "no global discovery" connection mode exists and is used

Their `get_node_addr` is documented as returning an address without direct IP addresses ... guaranteed to have home relay URL set as it is the only way to reach the node without global discovery mechanisms. This confirms a real iroh deployment deliberately operating **without** the global discovery layer (no Pkarr/DHT lookup), reaching peers via a known relay instead. That is precisely the center-reduction posture our discovery sub-investigation cares about: it is possible to run iroh connectivity without the n0-operated discovery soft-center, paying for it with a relay hint instead. *Verified (their API docs).* Useful for our discovery-strategy mapping: "relay-as-discovery-aid, global lookup disabled" is not theoretical, it is their default for realtime.

### 2.3 iroh's transport gives forward secrecy independent of content E2E

Their announcement notes the realtime layer inherits forward-secret end-to-end encryption ... Iroh uses QUIC on the networking layer which implements Forwa[rd secrecy]. This matches our Layer-A description (iroh QUIC/TLS 1.3 hop encryption). Note the seam, and it is one we already drew: their realtime FS comes from the *transport*, and their realtime identities are *ephemeral*: Delta Chat uses ephemeral cryptographic identities for any P2P messaging. When Delta Chat is closed or stopped by the operating system, then a new ephemeral identity will be created on the next start. They do **not** run MLS over the realtime channel. That is the gap our design fills: we put MLS PrivateMessage (Layer B) over iroh, getting content E2E and PCS to the group epoch, which their realtime layer does not have. So our two-layer model is doing something their realtime layer deliberately does not attempt.

---

## 3. The push design: their model vs ours (the most directly comparable piece)

### 3.1 What they do (grounded, with one inference)

For mobile, Delta Chat routes iOS push through a notification proxy: a community operator confirms when using a Chatmail Relay, push notifications are done by using the Apple Pushservices via notifications.delta.chat, with Android getting notifications by a different path (instant, per the same thread). So there is a dedicated push-relay host (`notifications.delta.chat`) bridging to APNs, distinct from the chatmail server that holds mail. *Verified that the host exists and bridges to APNs; the internal privacy design of that host was not pinned this round.* **[confirm: the exact token/heartbeat privacy design of notifications.delta.chat; their published rationale is that it should learn as little as possible, but I did not retrieve the server spec this round.]**

The webxdc-app-level notification ("it's your turn") is a separate, higher-layer mechanism: a new webxdc notification mechanism so that apps like Chess can cause a system-level notification between app users, reusing the existing push notification machinery of their host messengers without having to scream at Google or Apple compliance bureaucracy themselves.

### 3.2 What this informs in our design

*Synthesis.* Three takeaways, each mapping to a specific part of our spec.

- **A separate push-relay host, distinct from the store, is a shipping pattern, not a novelty.** Delta Chat keeps `notifications.delta.chat` separate from the chatmail/store server. That is exactly our **byte-free push-notify role kept separate from the meer** (design §5.1): they too split "the thing that pokes the phone" from "the thing that holds the mail." This validates our split-role instinct against a real deployment. Their push host bridges to APNs and holds device tokens; their mail lives elsewhere. We should cite this as prior-art corroboration that the byte-free push host is operationally realistic.

- **The push wakes the app; the content arrives over the durable channel.** Their push is a wake/signal; the actual message content is fetched over chatmail (SMTP) when the app wakes. This is identical in shape to our **wake-then-fetch, where the fetch drains the meer** (design §5.1, §9.2). Their entire production iOS experience is a working existence proof of "push is a wake, durability is elsewhere, and the device catches up on wake." That is the single most reassuring external validation of our default deployment: the model we propose is the model that already runs on hundreds of thousands of devices, just with SMTP where we put the meer.

- **Reusing the host messenger's push machinery so app developers never touch APNs/FCM** is a layering lesson for any future Drystone-hosted apps: the push credential and the Apple/Google compliance burden sit at the messenger/host layer, and higher-layer apps get a simple notify primitive. If Drystone ever grows an app layer (webxdc-like), the push-notify role should expose a similarly minimal notify API upward, never propagate APNs/FCM detail up the stack. *Synthesis.*

### 3.3 Where we deliberately differ, and why our position is defensible

- **They accept ephemeral realtime identities; we do not, because our realtime carries MLS.** Their realtime identity is regenerated each app start (§2.3 above). That is fine for ephemeral "it's your turn" pings but would be wrong for us, because our iroh channel carries MLS PrivateMessage tied to a durable group membership. Our EndpointId (peer identity, Layer A) is deliberately stable and separate from the MLS leaf (group identity, Layer B), per Part 2's two-identity-plane model. So we should *not* adopt their ephemeral-identity choice; it is correct for their use and wrong for ours, and the reason is the two-plane separation we already hold.

- **Their durability is the email server; ours is the meer, and ours is blind.** Chatmail/SMTP holds their durable messages, and a chatmail operator sees SMTP envelope metadata in the ordinary email way. Our meer is a blind store-and-forward of sealed MLS bytes, which is a stronger confidentiality posture for the durable store than their SMTP core. So our durability layer is a privacy improvement over theirs, not a copy. *Synthesis.*

---

## 4. The one genuinely new idea their design surfaces for us

*Synthesis.* The webxdc realtime channel is a **non-durable, ephemeral, members-actively-present broadcast** with a 128 KB per-update limit: a specified 128KB size limit for ephemeral application updates ... only distributed between devices actively running a webxdc app. This is, in our vocabulary, a pure **D-self + live-gossip** mode with explicitly *no* durability and *no* presence wake, used for live, both-present interaction (a shared game, a live cursor).

We had folded this into D-self / swarm-supplement, but Delta Chat treating it as a named, first-class, deliberately-ephemeral channel with its own size budget suggests we should **name the live-ephemeral case explicitly** as a distinct selector mode, rather than leaving it implicit inside D-self. It is the "both present, no durability wanted, low-latency interaction" cell, and it is genuinely different from "deliver a durable message." Recommend adding it to the design doc as a named selector mode (call it **live-ephemeral**: direct-carriage path, no durability source engaged, no presence wake), with the note that it is the right mode for realtime-interaction payloads and the wrong mode for messages that must survive. This is a small refinement, not a structural change, and it is the one place their design adds a category we had blurred. (Adopted: the design doc §8.1 now carries the payload classes, including the live-durable and intrinsic-ephemeral distinction this motivated.)

---

## 5. What does NOT transfer, stated to avoid over-learning

- **Their SMTP federation model.** It is their durability and discovery backbone; we deliberately do not have an email layer. Their relay-runs-on-every-chatmail-server pattern (an Iroh Relay which typically runs on every chatmail server, mirroring the existing e-mail federation) is an artifact of bootstrapping on email infrastructure. Our relay/meer/push hosts are resource-asymmetry roles, not federated mail servers; we should not import the "every server runs a relay" framing.

- **Ephemeral identity** (covered in §3.3): wrong for our MLS-bearing channel.

- **No MLS at the realtime layer:** their realtime channel has no group-epoch content E2E; do not treat their realtime security as a model for ours, ours is stronger by design.

---

## 6. Net effect on the spec

Small and confirmatory, not structural. Concretely:

- **No structural change (at the time of this finding).** The delivery-plane model, the durability sources, the byte-free push host, and wake-then-fetch all survive and are *corroborated* by a real deployment running the same shapes. (Later note: the plane model was subsequently refined from two planes to three, splitting carriage out from durability, and D-swarm became C-swarm; that refinement was internal to Drystone and does not affect the Delta Chat corroboration recorded here, which concerns the store-plus-push-plus-gossip shapes, not the plane count.)

- **Raise confidence on two `[confirm]` items.** The gossip-dedup-by-content behavior (supports our race-both dedup and our E1.1 hypothesis that the app must supply its own sequence metadata) is confirmed by their shipping `sequence_numbers` workaround. The byte-free-push-host-separate-from-store pattern is confirmed by `notifications.delta.chat` being distinct from chatmail.

- **One naming refinement.** Add **live-ephemeral** as a named selector mode (D-self, no durability source, no wake) for realtime-interaction payloads, which their webxdc realtime channel shows is worth distinguishing from durable delivery.

- **One new `[confirm]`.** The internal privacy/threat-model design of `notifications.delta.chat` (heartbeat/token scheme) was not pinned this round and is worth retrieving before we finalize our push-host metadata-minimization section, since it is the closest real-world analog to our byte-free push host and may have lessons (or pitfalls) for exactly the device-token-to-EndpointId binding cost we flagged as irreducible.

- **A reassurance worth stating in the pitches.** Our default mobile model (push wakes, durable store catches you up) is not speculative: it is the model Delta Chat runs in production on hundreds of thousands of devices, with SMTP where we put the blind meer. That is a strong, honest point for the technical pitch.
