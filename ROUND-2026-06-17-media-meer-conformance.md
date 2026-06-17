# The 2026-06-17 round — media, the meer, and conformance (plain language)

date: 2026-06-17 · A plain-language summary of what we ran this round, what it means for Croft, and
what's still open. The hard evidence lives in `experiments/iroh/relay-lab-runs/E1{0,1,2}-*` and
`E9-meer-tier0-*`, `experiments/iroh/TEST-LOG.md`, the conformance crate under
`Proofs/lineage-groups/crates/conformance`, and the ledger/narrative in `discovery/crystallized/`.

## The one-paragraph version

We had a messaging spine that was already proven, and four big "we think this works but haven't run it"
gaps: real-time **voice/video** over our transport, the always-on **meer** (the blind superpeer), a
machine-checkable **conformance** contract, and some honesty boundaries in the faithful path. This round
we ran all four. The two genuine *technical unknowns* in media both came back favorable: iroh's transport
doesn't fight a media engine (with one named caveat we now understand exactly), and media encryption keyed
off our real group-crypto works end-to-end through a forwarder that can't read it. The meer's headline
milestone — a blind, always-on relay that serves your offline data while provably holding no keys — now
runs on real machines. And Croft now has the start of a real interoperability test suite, which also
caught a genuine spec-vs-code mismatch. Net: the media end-state went from "designed" to "de-risked," the
meer went from "designed" to "running," and the protocol went from "lives in our spikes" to "checkable."

## What we did, experiment by experiment

### E10 — Does voice/video survive our network? (the #1 unknown) → answered

**Plain version:** We sent a fake voice stream (the right size and rhythm of a real call, just without a
microphone) over iroh's "fire-and-forget" datagram channel between two cloud machines, and abused the
network — added delay, dropped packets, throttled bandwidth — to see if the call holds up.

**What we found:**
- Under packet loss and delay, **iroh gets out of the way**. A 5% loss showed up as 5% loss, 30% as 30%,
  the round-trip time stayed flat and tracked any delay we added exactly. In call terms: audio stays
  intelligible up to 30% loss with audible-but-graceful degradation — never a silent freeze. A real media
  engine would get clean, accurate signals to work with.
- The one catch — and we now understand it precisely: if you try to send **faster than the link allows**,
  iroh does *not* drop the excess to keep up. It queues it and delivers it late, so latency balloons to
  *seconds* (a call would become unusable), while the sender's API never reports an error. The fix is
  known and standard: the media engine must watch the round-trip-time and **turn its own bitrate down** —
  it can't rely on the network to do it. We confirmed every signal it needs to do that is available.

**Why it matters:** This was the single riskiest assumption in the whole "voice/video over Croft" plan.
It's now evidence, not hope, and the design's proposed approach is the right one.

### E12 — Can a relay forward your video without being able to watch it? → yes (green-real)

**Plain version:** This is the privacy heart of group calls. We want a relay (the "meer") to forward
everyone's encrypted media without ever being able to decrypt it — Discord's newer "DAVE" design does
this. We built it on **our real group-crypto** (the same MLS machinery our messaging uses) and tested the
four things that have to be true.

**What we found (all four pass):**
1. Each speaker gets their own encryption key derived from the group; an **outsider can't derive any key**.
2. With 10% packet loss and frames arriving out of order, the receiver still **decrypts every surviving
   frame** and **rejects replayed ones** — media tolerates loss the way messaging can't.
3. When someone is **removed from the group**, the keys roll over: the removed person's later frames
   **can't be decrypted** by anyone, but the frames received *before* removal **stay readable** (history
   isn't clawed back).
4. The **relay routed every frame using only the unencrypted headers and recovered zero plaintext.**

**Why it matters:** "Media encryption = message encryption plus loss-tolerance" is now real, not a
hand-wave. The blind group-call relay is keyed correctly off crypto we'd already proven.

### Meer P0+P1 — the blind, always-on superpeer → running on real machines (green-real)

**Plain version:** The meer is what makes Croft feel like Discord without being like Discord: an
always-on helper that holds your group's state so history is there when you rejoin and channels stay
in sync — but, unlike a Discord/Matrix server, it **can't read anything**. We built the headline version
("Tier-0 blind mirror") and ran it across two machines.

**What we found:**
- A member published **encrypted** updates; the meer stored them by fingerprint, holding no key.
- A second member who'd been "offline" connected, pulled the updates **through the blind meer**, and
  decrypted them locally — it **caught up** completely (5/5).
- The meer's own stats prove it: **zero payload keys held**; the only thing it learned is metadata
  (sizes, timestamps, the channel name). We surface exactly that, honestly.
- **Anti-entrenchment, demonstrated:** we exported the meer's encrypted store, loaded it into a *different*
  replacement meer, and the member re-homed and caught up identically. Losing a meer costs availability,
  never your data — so running a meer never becomes a position of power you can't walk away from.
- The admission gate works: a meer with an allow-list refused an unlisted peer at the door.

**Why it matters:** This is *the* differentiator over self-hosted Matrix (server sees plaintext + metadata)
and Discord (central, can un-blind). "Run your own infrastructure, with a guarantee neither competitor
offers" is now a running binary, not a paragraph.

### E11 — Livestream/broadcast: can a blind relay still fight abuse? → characterized

**Plain version:** Broadcast (a stage, a watch-party, a livestream) is the surface that gets apps pulled
for piracy/abuse. The worry: if the relay can't see content, how does it moderate? We proved the relay
*logic* for the three things that matter.

**What we found:**
- **Lazy:** nothing is encoded or sent until someone actually watches (battery/privacy win — "nothing to
  fan out if nobody's watching").
- **Blind:** the relay forwards opaque frames, holds no key; viewers decrypt locally.
- **The abuse lever:** a blind relay *can* still moderate — by **scale and membership, from metadata
  alone**. It enforced a max-audience cap and a members-only rule, refusing over-capacity and non-member
  viewers **without reading a single frame**. The answer to "how do you moderate what you can't read" is:
  you limit *how many* and *who*, not *what*.

**Why it matters:** It shows Croft's broadcast story has an honest, non-surveillance abuse answer. (The
full video/codec/browser plumbing is deliberately deferred — it's assembly work, de-risked by n0's
shipping `iroh-live`, not a Croft unknown.)

### D — A conformance suite → built, 34/0 passing, and it caught a real bug

**Plain version:** A protocol is only "real" when a *second* implementation can be checked against it. We
built that suite — input→expected-output test vectors **generated by running our actual code** (never
hand-typed numbers) plus a runner that re-checks them.

**What we found:** 34 vectors pass, 0 fail, across the core invariants (genesis derivations, signatures,
the lineage-counted thresholds, revocation mechanics, the C1–C10 merge corpus). The "must-reject" cases
reject for real. And it **surfaced a genuine discrepancy**: our written protocol spec says genesis/topic
IDs use domain-tagged inputs, but the Rust code computes them plainly — the spec and one of the code
stacks disagree. That's exactly what a conformance suite is for.

## What it all means for us

- **The "voice/video/stage" end-state is de-risked.** The two real unknowns (transport congestion-control,
  and blind-relay media keying) both resolved favorably. What's left for media is *integration* (a real
  codec, the standard SFrame wire header, carrying it over the transport we've proven) and known *gated*
  items (browser reach, NAT hole-punching for direct calls) — not invention. **One firm design rule fell
  out of TC-CC3:** real-time media and bulk file transfers **must run on separate connections** — when we
  put a 20 MB transfer on the *same* iroh connection as a call under a 1 mbit link, the transfer starved
  the call (RTT 50 ms → 9.5 s). On separate flows the call is untouched. (This is the design's own C1
  recommendation, now proven mandatory rather than advisory.)
- **The meer is no longer a paper concept.** Its headline guarantee runs, and the "you can always leave /
  re-host" property that keeps it from becoming a power center is demonstrated, not just promised.
- **Croft is becoming a protocol, not just a prototype.** There's now a machine-checkable contract a second
  implementation can target, and it's already keeping the spec and the code honest.
- **A pleasant surprise:** the threshold "who's allowed to revoke someone" crypto we thought still needed
  building is **already real** in our governance code (real k-of-n signatures, counted by person not by
  device). The remaining work there is plumbing it over the wire, not new cryptography.

## The open questions (honest list)

1. **Media estimator — now demonstrated (E10c/TC-CC2), one caveat left.** We built a delay-based AIMD
   estimator and ran the realistic case (call starts healthy, link degrades mid-call): the adaptive
   sender backs off 64→8 kbps in under a second and **bounds RTT to ~1 s with a continuous lossless
   stream**, where the fixed sender bufferbloats unbounded past 7 s (dead call). So the RTT signal is
   *actionable*, not just present. What's left: it does **not** rescue the *join-an-already-saturated-
   link* case (the standing queue never drains); residual RTT is ~1 s not ~50 ms (a production engine
   would pace down faster / flush); and we still haven't run a true microphone-to-ear quality test
   (synthetic frames throughout) or a raw-UDP side-baseline.
2. **Direct (mesh) calls need NAT hole-punching**, which is still gated on opening public ingress. Until
   then all media is relay-routed (the louder-metadata case).
3. **str0m video maturity** is still unverified — we're audio-first on purpose; video is a later milestone.
4. **The meer's deeper roles** (bridging two namespaces, the double-enveloped Tier-1, the full Willow-style
   reconciliation, the actual media SFU/broadcast roles, and the trusted Tier-2) are still ahead. And
   **rogue-meer detection over time** — proving a meer stays blind and doesn't quietly log more — is an
   open design problem.
5. **Faithful messaging path:** real MLS key-distribution over the wire (proven for *media* in E12, not yet
   for the messaging crate), and carrying the real revoke-authority signature over the wire to retire the
   transport spike's placeholder MAC. Plus the co-sign-vs-vote ordering decision.
6. **The spec-vs-code derivation mismatch** the conformance suite found needs a decision: do the two
   stacks reconcile to the domain-tagged pre-images, or does the spec get corrected to match the code?
7. **Conformance coverage gaps:** the adversarial, visibility, and freshness vector categories aren't
   emitted yet (some live in the TypeScript model stack), and the revoke-authority vector is a declared
   placeholder pending #5.

## Where the evidence lives

- Runs + manifests: `experiments/iroh/relay-lab-runs/E10-roq-netem-2026-06-17/`,
  `E11-moq-lazy-2026-06-17/`, `E12-sframe-mls-2026-06-17/`, `E9-meer-tier0-2026-06-17/`.
- Code: `experiments/iroh/crates/relay-loadtest` (roq + meer subcommands),
  `crates/media-sframe-spike`, `crates/moq-lazy-spike`, `Proofs/lineage-groups/crates/conformance`.
- Detailed log: `experiments/iroh/TEST-LOG.md`. Lab spec (with MEASURED annotations):
  `experiments/iroh/RELAY-PLACEMENT-LAB-SPEC.md`.
- Status + story: `discovery/crystallized/proof-ledger.md` and `discovery/crystallized/test-narrative.md`.
