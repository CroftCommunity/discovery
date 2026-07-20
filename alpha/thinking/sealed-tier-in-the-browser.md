# The sealed tier reaches the browser (no overlay bridge)

author: ISaT / Product Security

date: 2026-07-20

status: design thinking — grounded in RUN-19 (`Modeled`; component/wasm-node grade, digest-attested);
the §A.8 revision + custody posture are staged `needs-call` (the owner's)

relationship: a deployment-layer finding about the sealed (real-E2EE) tier; upgrades GROUPS.md §A.8's
deferred-browser sentence. Sibling to `realtime-media-over-iroh.md` (the media floor) and
`local-first-as-design-imperative.md` (why the browser matters); the PWA lineage is skylite / pdsview.

---

## 1. The thesis

A browser can be a **full sealed-tier client** — real MLS end-to-end encryption, the operator
content-blind — with **no overlay bridge and no native app**. The old plan (§A.8) deferred the browser's
sealed story to "WASM MLS plus a relay bridge." The bridge is now unnecessary, and the reason is a clean
one: the bridge was only ever for **peer reachability**, and a sealed-tier browser client does not need
to reach peers. It needs to reach the **content-blind Delivery Service** — a public, certificated
endpoint — which is exactly what a browser can connect to directly.

## 2. Why the timing turned good (two facts changed)

- **OpenMLS compiles to the browser.** `wasm32-unknown-unknown` is a CI-built target with a `js` feature,
  and the RustCrypto provider is pure Rust — nothing in the crypto stack fights the page. (Honest
  asterisk: OpenMLS CI *builds* but does not *test* wasm — which is exactly the evidence gap RUN-19
  closed with a green suite under the wasm module.)
- **WebTransport reached cross-browser Baseline (March 2026).** QUIC streams/datagrams from a browser to
  a certificated endpoint are now a promise you can make to users, not an A/B test. The historic reason
  every honest answer was "not yet" was Safari; that reason is gone.

So the shape is: **MLS in WASM in the page → ciphertext over QUIC/WebTransport → the content-blind DS →
offer-gating exactly as proven → E2EE intact, because the DS was blind by construction all along.** The
overlay (iroh) remains the *native* peers' transport and loads only for sealed steward governance; the
browser's leg is web-native end to end. (evidence: RUN-19 P1–P5 green red-first; P2 cross-build interop
goldens — wasm-sealed ciphertext unsealed natively and vice-versa, byte-identical group state; P4 real
QUIC through the blind DS; P5 two wasm members over the blinded path)

## 3. Why it matters (the sovereignty story)

This is the difference between "real E2EE is for people who can install a native app" and "real E2EE is
for anyone with a browser." The audience Croft courts hardest — kids and grandparents on a tended PWA
(skylite), a plot-tender opening pdsview, a family on arecipe — gets the sealed tier **with no app-store
gatekeeper standing between them and their own encryption**. It is the local-first imperative reaching the
one runtime everyone already has. The browser stops being the degraded tier; it becomes a first-class
sealed client.

## 4. The honest costs (custody-shaped, not transport-shaped)

The transport question is answered; what remains is custody, and it must be stated plainly:

- **Keys live in wasm memory → XSS is the threat model.** Not the network, not the DS — the page's own
  script surface. The bounding answers: device-key delegation limits the blast radius, and
  revocation-by-attestation-deletion is the remedy. (evidence: RUN-19 P3 state-at-rest via the provider's
  AEAD; SIGKILL-the-host resume into the next epoch)
- **Browser storage is evictable.** iOS especially will evict. Eviction means **rejoin-via-Welcome**,
  blind to the gap — which is consistent with the history-honesty doctrine (a gap is named, not silently
  papered over), *not* a data-loss surprise. (evidence: RUN-19 P3 eviction drill — no self-restore.)
  This is also where the account-recovery predicate (I9) meets the browser: RUN-19 shows the boundary and
  stops; **no recovery path is built here** (see the E2EE-recovery research + I9).
- **MLS state is single-writer → multi-tab needs a leader lock.** Two tabs writing the same group state
  corrupt it; one tab must hold the lock.
- **WebTransport / the W3C spec is still a Working Draft.** Pin the server and browser library versions
  together (RUN-19 used `wtransport =0.7.1` server+client from one revision) until it stabilizes.

## 5. What this establishes (and does not)

Establishes that the sealed tier's browser shape is feasible and demonstrated end to end at
component/wasm-node grade — MLS in wasm, ciphertext over real WebTransport to a content-blind DS, no iroh
bridge — so §A.8's browser leg upgrades from "deferred, needs a bridge" to "web-native." It does **not**
ratify the custody posture (drafted `needs-call`, nothing landed), does **not** build the browser
key-recovery path (I9, still the open predicate), and is **not** a live-relay/production claim — the run
is loopback-localhost QUIC with named Node-hosted stand-ins (one headless-Chrome attempt then tag; a
native Rust client speaking the identical WebTransport wire), digest-attested because the audit container
has no toolchain. The §A.8 revision + the custody-posture DRAFT are the owner's next call.
