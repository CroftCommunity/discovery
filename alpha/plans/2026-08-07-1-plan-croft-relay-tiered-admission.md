# croft-relay: cap-gated calling with a metered introduction budget

- **Status:** Rewrite (2026-08-08), Pass 2 complete against the rewrite (2026-08-08), cap distribution
  settled out-of-band (2026-08-09). Supersedes the 2026-08-07 draft and its gap analysis — both
  preserved in the Review Log, not above it. **12 phases** in five milestones. Pass 3 (quality gates)
  pending; **no outstanding blocking questions.**
- **Supersedes in part:** ADR-0001's fork-vs-embed framing; **ADR-0004's entire enforcement
  mechanism** (rate-limiting is replaced by a byte budget with a clean disconnect).
- **Source dialogue:** `../seeds/transcripts/raw/croft-relay-tiered-admission-fork-vs-embed-2026-08-07.md`
- **Existing code:** `../experiments/croft-relay/` (RUN-CROFT-RELAY-01/02/03, PR #40);
  `CroftCommunity/connect` (the lexicon, the exchange page, the Android client)
- **Thread:** `beta/OPEN-THREADS.md` T62

---

## 1. What we are building, in one page

A relay does two jobs: it **introduces** two endpoints so they can connect directly, and it **carries**
their traffic when that direct connection can't be made. Croft's product splits exactly along that
seam:

> **Introduction is free. Carrying is what membership buys.**

Everyone who can call at all gets introduced. If the direct connection takes — roughly 90% of the time
— the relay drops out of the path and nothing is consumed. If it doesn't take, the relay is now
carrying a real call, and that is the service worth paying for.

Two independent gates, and keeping them independent is what makes the design coherent:

| Gate | Question | Who decides | Enforced by |
|---|---|---|---|
| **Cap** | May you call me at all? | the callee | `croft-admit`, at token-mint |
| **Membership** | Will we carry your traffic? | Croft (commercial) | the relay binary, per connection |

So the matrix is:

```
  no cap                    → cannot call (may be able to *request* a cap — see §5)
  cap, neither is a member  → introduced. Byte budget. Spent → connection dropped, cleanly.
  cap, either is a member   → carried. No budget.
```

"Either is a member" is the rule the owner set: a member's participation unlocks the tunnel whether
they are transmitting or receiving. It is decided at call setup by `croft-admit`, which is the only
component that sees both parties.

### 1.1 Precondition this plan does not solve (Pass 2)

**Both parties must be reachable for any of this to matter, and today that means both have the app in
the foreground.** The `connect` Android client is foreground-only by design — background reachability
needs a foreground service plus push-to-wake, which its README calls out as its own phase. Nothing in
this plan changes that.

This is stated here rather than buried in a phase because it bounds what the plan delivers: until it is
solved, calls connect only when both parties are already looking at the app.

**It is not a blocking question for this plan.** Background reachability is per-platform work with its
own constraints on each OS, tracked separately. Nothing here depends on it — cap distribution is
out-of-band (§5), so no phase needs to reach a user who isn't watching. Recorded so it is not
discovered later as a surprise, not because it gates anything below.

## 2. Problem Statement

`croft-relay` gates an iroh relay on atproto identity. RUN-01/02/03 built and proved the admission
core: a relay-agnostic `croft-admit` (DID-bound enrollment, deny-closed access check, EdDSA capability
tokens, 0 mutation survivors) and a `croft-relay-embed` adapter over `iroh-relay 1.0.3`'s public
`AccessControl` trait, verified against a real relay on localhost.

Four things were unresolved, and the first two were wrong rather than merely incomplete.

**(a) The enforcement mechanism was wrong.** The design throttled non-members until relayed media
became unusable. That is a soggy failure: the user gets a call that sounds broken rather than a call
that tells them why it cannot proceed. The owner's objection — "it feels like there would be an
escalation to tunnel and that's what we would want to stop, rather than trying to slow it down so it
fails" — is correct in spirit, though the mechanism is the reverse of the intuition: **the relay
carries the call from the first packet, and holepunch success is what removes it.** There is no
escalation event to refuse. What there is, is an ongoing flow that can be given a *budget* and cut off
cleanly when it is spent. See §3.

**(b) Tier enforcement was unreachable as specified.** `Access::Allow` is a unit variant and iroh's
rate limiter is installed on the raw stream *before* authentication, from a single service-wide
watcher. ADR-0004's stated fallback — "wrap the admitted connection in a `Bucket`" — cannot work:
`RateLimited` is `pub(crate)` and an embedder never touches the post-handshake stream. Verified in
source, not inferred.

**(c) Several foundations named in the plan do not exist.** No persistence, no runnable
`croft-admit` service, no real PDS or DID-document resolution (every test to date ran against an
in-memory fixture), and no per-user byte accounting available from iroh at all.

**(d) The coordination bucket was a guess.** A `SPEC-DELTA` placeholder, sized by reasoning. The budget
mechanism makes this far less sensitive — see §3 — but a number is still needed.

**Constraints carried in from the corpus and the owner's decisions:**

- The relay must not hold social-graph state, and must stay content-blind.
- Maintenance sanity is first-order: "as little and as discrete changes as possible."
- Authorization stays ours; authentication is delegated to atproto.
- CISS's native `id:` keypair identity is **not** to be extended outside CISS's scope.
- We would rather not manage storage for people we have no relationship with.
- The at-rest hashing mechanism is built and tested but **not relied upon** — key custody is unanswered.

## 3. Reasoning

### 3.1 Why a budget, not a speed limit

The relay cannot distinguish a holepunch coordination frame from audio — that is by design, it is what
content-blindness means, and we want to keep it. So content-based refusal is impossible. But *volume*
is a legitimate and honest proxy for "did the introduction take," because a successful introduction is
a small, bounded exchange and a carried call is not.

That gives two ways to use volume, and only one of them is good:

- **Throttle** (rejected): degrade until media fails. The user experiences a broken call. The system
  never says no; it just gets worse. And the number matters enormously — set it slightly wrong and you
  either permit media or break coordination.
- **Budget and drop** (chosen): allow a generous allowance, several times what an introduction needs.
  When it is spent, **disconnect**, and let the client surface "this call needs a membership." A clean
  no instead of a soggy maybe.

Budget-and-drop is also far less sensitive to calibration. An introduction is on the order of
kilobytes; a call is kilobytes *per second*. Any budget in between — and the gap spans two or three
orders of magnitude — separates them correctly. Throttling had to thread a needle; a budget only has to
land in a canyon.

**The budget is bytes, not seconds.** A non-member camping idle on their home relay so they can
*receive* calls costs us nothing and must keep working — that is the free introduction service. Only
sustained data burns budget. A time limit would evict idle campers, which is the opposite of the
intent.

**Self-cleaning property:** if the holepunch succeeds, the relay connection goes quiet, the budget is
never spent, and iroh's own actor exits after 60s idle. The mechanism only ever fires on the calls it
is meant to fire on.

### 3.2 Why the budget change deletes the tier switch

The previous design needed N relay instances with different rate limits behind a routing switch,
because iroh's rate limit is per-instance and cannot vary per connection. **A budget enforced in our
own layer has no such constraint.** One relay instance, no rate limit configured at all, and our
decorator drops connections that exceed their allowance.

This removes: the routing switch, the N-instance construction, per-tier `RelayService` wiring, the
"which room does a cross-tier pair meet in" problem, and the question of whether the switch could be
mistaken for the authorization point. It is the single largest simplification in this rewrite, and it
came directly from fixing the mechanism.

Retained from that design and still true: relays do not federate, and a client opens a connection per
relay it needs, so cross-relay calling works by the sender joining the receiver's relay.

### 3.3 Why our own binary, and why that is not a fork

Three distinct things, routinely conflated:

| | what it is | maintenance cost |
|---|---|---|
| Stock binary | n0's prebuilt release | none, but no customization possible |
| **Our binary** | our `main.rs` depending on `iroh-relay` from crates.io, **unmodified** | `cargo update` |
| Fork | a patch series carried on their source tree | rebase every release, forever |

We take the middle. Every line of relay code comes from the published crate; we wrap it. Nothing to
keep pristine because nothing is patched. This is also the only shape in which the byte counter can
exist, because it requires owning the connection before handing it to the relay — see §3.4.

**Rejected: the upstream accounting patch.** A per-connection accounting hook is a real gap in
iroh-relay and would be upstreamable. The owner's decision is to not attempt it now: build our own
wrapper first and see what it teaches us. Recorded because it remains a good future contribution — and
if it is ever attempted, it should emit **volume only** (`endpoint, bytes, duration`), never pairs.
`(src, dst)` accounting is a call-detail record, the exact artifact the design exists to avoid holding.

### 3.4 Why the byte counter is a decorator, not network plumbing

Not nftables, not eBPF, not a second process. Those sit below the layer where identity is known, and
the traffic is TLS regardless.

Our binary accepts the TCP connection, terminates TLS, and reads the HTTP request — at which point we
know which token was presented, therefore which member. Before handing the stream to iroh's service, we
wrap it in a struct implementing `AsyncRead`/`AsyncWrite` that increments a counter. On close we emit
`(member, bytes_in, bytes_out, duration)`.

It counts framing as well as payload, so it is an **upper bound**. That is fine for a budget and for
capacity planning, and it must never be described as billing.

### 3.5 Why caps live in the caller's own repo

A cap is a durable statement by the callee that a specific person may call them. Three homes were
considered:

- **A list in `croft-admit`** — private, but we hold a social graph and must store data for people we
  have no relationship with.
- **A public list in the callee's PDS** — sovereign, but publishes the call graph. Salted hashes do not
  help: atproto DIDs are enumerable from the firehose, so a public salt lets anyone test every DID.
- **An opaque-id record in the callee's PDS** (chosen). The record says "cap `a3f9` is valid." It names
  nobody. The caller holds the cap itself. `croft-admit` verifies by fetching the issuer's grant
  records; revocation is a record delete.

A `did:plc` user does not hold their own signing key — their PDS does — so they cannot sign a cap on
their device. But atproto signs **repo commits**, so a record in their repo is verifiably their
statement. The owner's ruling, which is the ecosystem's existing model rather than a new assumption:

> A record in a user's repo is that user's statement. Whether they hold the signing key or their PDS
> operator does is between them and their operator. We verify the signature and the DID binding; we do
> not model PDS trust.

**Blast radius, stated so the decision can be judged:** a hostile or compromised PDS could mint a call
grant in a user's name. What that buys is *relayed bandwidth in their name* — not access to any call,
because the call is end-to-end and the relay is blind. A bounded, financial-shaped harm, absorbable by
a quota. (Whether a user holds recovery keys and can undo a hostile PDS is the standard atproto
recovery question; our FACTCHECK docs do not cover it, and nothing here depends on the answer.)

**What this leaves us storing:** membership (a commercial relationship we necessarily have) and
accounting (our own costs). Neither is a social graph. Nothing about who-calls-whom persists anywhere.

### 3.6 Why one cap gate rather than two lists

Making a cap the precondition for calling at all collapses "who may ring me" and "who gets the good
pipe" into one artifact. There is no separate allowlist to maintain, leak, or synchronize.

The cost is that **discovery and reachability come apart**: the exchange page can find you by handle
but cannot dial you without a cap. That is a defensible stance for a calling app — findable, not
cold-callable — and §5 gives it a release valve.

### 3.7 Alternatives considered and rejected

| Alternative | Why rejected |
|---|---|
| Throttle non-members until media fails | Degraded call instead of an honest refusal; calibration-sensitive (§3.1) |
| Fork for a per-connection rate override | Unnecessary once the budget lives in our layer; permanent rebase cost |
| Pairwise policy in the relay's forwarding path | Deep fork, and installs social-graph state in the relay |
| N instances + tier switch | Deleted by the budget mechanism (§3.2) |
| Stock binary + HTTP hook as the destination | Can only answer yes/no: no budgets, no disconnect, no accounting (§6, Phase 4) |
| Route a proxy on destination endpoint id | Structurally impossible — destination is per-datagram, connections multiplexed |
| Upstream accounting patch as a dependency | Owner's call: build ours first. Kept as a future contribution, volume-only |
| Allowlist in `croft-admit` or public in the PDS | Holds/publishes the call graph (§3.5) |
| Time-based budget | Would evict idle campers, killing free reception (§3.1) |
| In-band cap requests via a `croft-admit` queue | Storage and a message queue for non-members — the two things we set out not to run (§5) |
| In-band cap requests via a record in the requester's repo | Makes "X wants to call Y" a public firehose fact; infrastructure-free but leaks the request graph (§5) |

## 4. Verified Assumptions

Read from pinned source. Anything not listed is unverified.

**iroh-relay 1.0.3**

| Assumption | Source |
|---|---|
| `Access` is `Allow` / `Deny { reason }`; `Allow` carries no rate limit | `src/server.rs:350` |
| Rate limiter wraps the raw stream pre-handshake, from one service-wide `watch::Sender` | `src/server/http_server.rs:856` |
| `on_connect` runs post-authentication, via `authorize_with` | `.../http_server.rs:876` |
| `RateLimited` is `pub(crate)`; `Bucket` is `pub` — no embedder-reachable wrap point inside the relay | `.../server/streams.rs:333`, `:363` |
| **`RelayServiceWithNotify` is `pub` in `pub mod http_server`, implements `Service<Request<Incoming>>`, and its docs carry an embedding example using `serve_connection(...).with_upgrades()`** — this is the seam the whole design rests on | `.../http_server.rs:714-745` |
| `RelayService::handle_connection(TcpStream, Option<TlsConfig>, Duration)` is public (the simpler path, which does its own TLS/HTTP and therefore cannot expose the header to us) | `.../http_server.rs:1013` |
| `RelayService::new` is public; `Handlers` is `pub` + `Default` + `DerefMut`; `KeyCache`, `Metrics`, `RelayService` exported | `.../http_server.rs:911,1130`; `src/server.rs:73`; `src/lib.rs:48` |
| `Clients::disconnect(endpoint_id, connection_id) -> bool` is public, reachable via `RelayService::clients()` | `src/server/clients.rs:181` |
| **Metrics are server-wide counters** (`bytes_sent`/`bytes_recv`), `#[non_exhaustive]`, **no per-endpoint dimension** — per-user accounting is not obtainable from iroh | `src/server/metrics.rs:9-24` |
| `RelayConfig.limits.client_rx` sets a per-instance rate limit; `Server::spawn(ServerConfig)` is the higher-level path | `src/server.rs:111-160`, `:691`, `:741` |
| Token rides `Authorization: Bearer` (native) or `?token=` (wasm) | `src/client.rs:157`; `src/http.rs:23` |
| Client-auth header is `x-iroh-relay-client-auth-v1`; TLS-exporter auth failure is normal and falls back to a challenge round-trip | `src/http.rs:18`; `src/protos/handshake.rs:447` |
| Relay path is `/relay`, constant — not a routing key | `src/http.rs:13` |
| **(Pass 2)** `Clients::disconnect(endpoint_id, None)` drops **every** connection for that endpoint; `Some(id)` targets one. `ConnectionId` is `pub` | `src/server/clients.rs:172-195`; `src/server.rs:175` |
| **(Pass 2)** `disconnect` is **asynchronous** — actors exit their run loop and unregister *after* the call returns, so byte overshoot between decision and close is expected | `src/server/clients.rs:179-181` (doc comment) |
| **(Pass 2)** `ClientRequest` exposes `endpoint_id()`, `connection_id()`, `auth_token()`, `uri()` as public accessors | `src/server.rs:186-235` |
| **(Pass 2)** The Service takes the websocket upgrade from the request inside `call` (`handle_relay_ws_upgrade`) — so the upgraded task receives the IO we wrapped. **Mechanism understood, not yet asserted by a test** (Phase 3 does that) | `src/server/http_server.rs:742-760` |

**iroh 1.0.3**

| Assumption | Source |
|---|---|
| Relays do not federate; one `ActiveRelayActor` per relay server; home-relay actor never exits; others exit after 60s idle | `src/socket/transports/relay/actor.rs:1-25`, `:65` |
| Send path is addressed by the destination's relay URL | `src/socket/transports/relay.rs:272` |
| `Endpoint::insert_relay` is public + async — runtime token swap without rebinding | `src/endpoint.rs:984` |
| `RelayConfig::with_auth_token` sets the bearer | `iroh-relay-1.0.3/src/relay_map.rs:266` |

**Our tree**

| Assumption | Source |
|---|---|
| `Registry` is `Mutex<HashMap<EndpointId, Did>>` — in-memory, no persistence, one direction only | `crates/croft-admit/src/registry.rs:17-50` |
| `croft-admit` has **no `[[bin]]`** — `access_router` exists but nothing serves it | `crates/croft-admit/Cargo.toml`; `src/lib.rs` |
| `PdsResolver` is a **trait with an in-memory fixture only**; no HTTP client dependency; the "production adapter" its doc comment promises is unbuilt | `crates/croft-admit/src/pds.rs:38-42` |
| `Did` is deliberately opaque — DID-document resolution is by design not in this crate | `crates/croft-admit/src/did.rs:1-11` |
| `ciss-auth` **does not resolve DIDs**; `ResolvedKeys` is defined there and must be supplied by the caller; no HTTP client | `CISS/crates/ciss-auth/src/lib.rs:127`; its `Cargo.toml` |
| `ciss-auth`'s service-auth JWT verifier takes the curve from the **resolved key, never the JWT header** | `CISS/crates/ciss-auth/src/service_jwt.rs:1-12` |
| Workspace dep rule: "add when a test forces it, not speculatively" | `experiments/croft-relay/Cargo.toml:16-18` |
| CI pins toolchain `1.94.1`; the experiment has **no** `rust-toolchain.toml` | `discovery/.github/workflows/smoke.yml:45-48` |
| `croft-relay` is in the CI smoke matrix | `smoke.yml:41` |
| Deployed relay is 1.0.0 via prebuilt musl tarball, pinned + checksummed, netns-isolated | `croft-stack/ansible/group_vars/all.yml:72-73`; `alpha/plans/croft-stack/06-iroh-relay.md:19` |

**`CroftCommunity/connect`**

| Assumption | Source |
|---|---|
| Lexicon `ing.croft.iroh.endpoint`, rkey `self`, fields `endpointId` (required) / `homeRelay` / `createdAt`; read via `getRecord` | `docs/contract.md` §1 |
| Deep link `croftcall://call?endpoint=&relay=&handle=&did=`; `endpoint` required | `docs/contract.md` §2 |
| Resolution pipeline: `resolveHandle` → plc.directory or `did:web` → `getRecord` | `docs/contract.md`, informative section |
| **Nothing is published yet** — the lexicon is a clean slate (owner-stated) | dialogue |

**Explicitly unverified, and named as such**

- **Holepunch failure rate ≈ 10%** — owner-supplied, recalled from prevailing figures, not measured by
  us. Accepted as a working assumption. Our own users' NAT distribution may differ.
- **Whether `Endpoint::insert_relay` with a changed auth token forces a relay reconnect.** The budget
  boundary depends on it (see Phase 4). Verify before designing around either answer.
- **Introduction byte cost across real NATs.** Phase 0.
- **(Pass 2) `com.atproto.repo.listRecords`** — the corpus FACTCHECK docs do not cover it, and
  `connect/docs/contract.md` evidences only `resolveHandle` and `getRecord` (which are working,
  deployed code). `listRecords` is this plan's addition and is asserted from general knowledge. Confirm
  against the atproto lexicon before Phase 10 builds the contract on it.
- **(Pass 2) That post-upgrade bytes traverse our `CountingStream`.** The mechanism is understood
  (hyper hands the original IO to the upgraded task) but untested. Phase 3 asserts it; the counting
  design fails silently if it does not hold.

## 5. Cap distribution: out-of-band only

A cap gates calling, so a stranger has no way in. That is intentional. The question was whether a
stranger needs an in-band way to *ask*. **Owner's decision (2026-08-09): out-of-band is the only model
we are pursuing.**

You hand someone a cap the way you would hand them your phone number — a link, a QR code, an email.
Nothing traverses our infrastructure, nothing queues anywhere, nothing about the request is published.

```
  callee's client                                   caller
        │  writes a grant record to its own repo
        │  (opaque cap id, names nobody)
        │
        │  produces a link / QR carrying the cap
        └───────────── out of band ───────────────▶ stores it
                     (message, email, in person)         │
                                                         ▼
                                       presents the cap at call setup
                                       croft-admit verifies it against
                                       the grant record in the callee's repo
```

Each user still publishes a **policy record** — `anyone | mutuals | nobody` — but its role is narrower
than the earlier design assumed: **it is advisory UI, not enforcement.** The cap is the gate. The
policy exists so the exchange page knows what to render, and so a user can say "don't bother asking"
without that statement disclosing anything about who they know. A caller holding a valid cap is
admitted regardless of what the policy says.

The page refuses identically for `nobody` and for "no such handle" — a refusal that leaks existence
would be a worse privacy failure than the allowlist this design avoided. `anyone` is built and tested
as the counter-case so we understand what enabling it means, and is not advertised.

**Rejected in-band alternatives, recorded because the second one looks free and is not:**

- **A request queue in `croft-admit`.** Storage and a message queue for people we have no relationship
  with — the two things this design set out not to run.
- **A request record in the *requester's* repo**, found the way Bluesky notifications are. Repos are
  write-authenticated by their owner, so a stranger cannot deposit anything in *your* repo — only in
  their own. Which makes the request **public**: "X wants to call Y" becomes a firehose fact. It
  requires no infrastructure from us and leaks the request graph in exchange, which is a bad trade for
  a system whose whole point is that the graph lives nowhere.

## 6. Documentation Impact

Scheduled into the phase that makes each reference stale.

- `experiments/croft-relay/docs/adr/0006-cap-gated-calling-and-budget.md` — **new**; the whole design. Phase 1.
- `experiments/croft-relay/docs/adr/0004-tier-buckets.md` — the **mechanism section is superseded**, not merely corrected: rate buckets are replaced by byte budgets with a clean disconnect. Retain its content-blindness argument, which still holds and is now load-bearing for why volume is the only honest signal. Phase 1.
- `experiments/croft-relay/docs/adr/0001-embed-vs-fork.md` — fork-vs-embed conclusion superseded-in-part; the `AccessControl` finding stands. Phase 1.
- `experiments/croft-relay/docs/adr/0003-token-format.md` — claims gain a **budget**; revocation story changes (expiry *and* record delete). Phase 8.
- `experiments/croft-relay/OPEN-QUESTIONS.md` — Q1 confirmed (JWT/EdDSA); Q3 superseded (no longer a rate cap); Q4 resolved (Path A is fallback, not first deploy); Q2 and Q5 open. Phase 1.
- `experiments/croft-relay/README.md` — the "build plan (in the run summary and the ADRs)" pointer is stale; the build plan was never filed until the 2026-08-07 transcript. Phase 1.
- `experiments/croft-relay/DESIGN.md` — tier-enforcement section rewritten to budgets. Phase 4.
- `experiments/croft-relay/crates/croft-admit/src/tier.rs` — `bucket_for` becomes `budget_for`; the `SPEC-DELTA` comment is rewritten around budget sizing. Phase 4.
- `beta/OPEN-THREADS.md` T62 — gates rewritten: budget sizing, deployment, and the `insert_relay` reconnect question. Phase 1.
- `alpha/experiments/EXPERIMENT-BACKLOG.md` §6j, `alpha/COHESION.md`, `alpha/seeds/transcripts/RAW-ARTIFACTS-MANIFEST.md`. Phase 1.
- `alpha/ROADMAP_TODO.md` + `alpha/plans/croft-stack/06-iroh-relay.md` — own musl artifact replaces the prebuilt tarball; 1.0.0 → 1.0.3. Phase 5.
- **`CroftCommunity/connect` `docs/contract.md`** — lexicon moves to per-device records; `getRecord` → `listRecords`; adds the request-policy record. Cross-repo. Phase 10.
- `.claude/CI-PATTERN.md` — grepped: no change needed; `croft-relay` is already in the smoke matrix.

## 7. Concurrency Map

```
Phase 0 (budget sizing — owner-gated on a second network) ─────┐ independent
Phase 1 (record + correct; docs + toolchain + manifest) ───────┘
                              │
                              │
   Milestone B (relay binary)  →  Milestone C (admission service)
   P2 → P3 → P4 → P5              P6 → P7 → P8
                                  (P8 consumes P3's usage records)
                              │
                    Milestone D (client + lexicon)
                    P10 (connect repo) → P11
                              │
                    Milestone E — P12
```

- **Phase 0 ∥ Phase 1.** Disjoint write-sets. Phase 1 writes the T62 gate *shape*; Phase 0 fills the
  constants. If run in parallel, Phase 1's placeholders must be explicitly marked.
  - *Re-entry (P0):* nothing written outside `experiments/croft-relay/{tests,evidence}/`; both hosts'
    harness ports released.
  - *Re-entry (P1):* `git diff --name-only` shows only `.md` plus the single manifest and toolchain
    edit.
- **Milestone B ∥ Milestone C — DISQUALIFIED (Pass 2). They run sequentially, B then C.**
  Two independent collisions, and the second is not fixable by hoisting:
  1. Both would write the workspace root `Cargo.toml` (B registers the new member crate, C adds
     dependencies). Hoisting both edits into Phase 1 would have fixed this — at the cost of the crate's
     "no speculative deps" rule.
  2. **Phase 4 (Milestone B) writes `crates/croft-admit/src/tier.rs`** — a Milestone C file — because
     the tier→budget rewrite lives there. No hoist resolves a phase writing another milestone's source.
  Since (2) settles it regardless, **the dep-rule question is moot** and the manifest hoist is dropped:
  each milestone adds its own dependencies when a test forces them, per the repo's stated rule.
  Sequencing B before C is also the right order on merit — Phase 8's quota work consumes the usage
  records Phase 3 emits.
- **Phase 10 is in a different repo** (`CroftCommunity/connect`) and shares no write-set with anything
  here, so it may start as soon as the lexicon shape is agreed — but Phase 11 needs both it and
  Milestone C.
- Everything else sequential.

---

## Milestone A — Ground truth and the record

### Phase 0: Discovery — size the introduction budget

**Goal:** Turn the budget from a guess into a measurement. Cheaper than the old calibration because
budget-and-drop only has to separate kilobytes from kilobytes-per-second.

**Discovery Exemption applies** — spike work, not TDD. The harness is `keep-as-fixture`; scripting is
`throwaway`.

**Changes:**
- [ ] Two `iroh` endpoints on genuinely separate networks. Instrument our side to count bytes per
      endpoint across a full introduction.
- [ ] Measure the **introduction byte cost** for a successful holepunch — distribution, not one sample.
- [ ] Measure it for a **failed** holepunch too: how many bytes accrue before the relay is clearly
      carrying a call. This is the number the budget must sit above.
- [ ] Establish a **carried-call byte rate** as an anchor. There is no media path in croft-call v0 (it
      does a `croft-call/0` hello exchange), so use a synthetic constant-bitrate stream at candidate
      audio rates and **record it as a proxy, explicitly not as a measured call**.
- [ ] Opportunistically record how often holepunch succeeded, as a sanity check against the ~10%
      working assumption. Not a rate — a sanity check, and labelled as such.

**Call chain / Wiring test:** n/a (discovery). The rig's own acceptance is that it produced **both** a
successful and a failed holepunch, measured. A rig that only ever succeeds has not characterized the
case the budget exists for.
**Depends on:** owner-supplied second network. Resource gate, not engineering.
**Read-set:** `crates/croft-relay-embed/tests/live_relay.rs`.
**Write-set:** `experiments/croft-relay/tests/`, `experiments/croft-relay/evidence/`.
**Shared-state contract:** Binds ports on two hosts; needs outbound UDP. No git operations; no writes
outside the experiment tree.
**Risks:** Reporting one NAT pair as a distribution. State sample counts, and refuse to average two
data points into a claim.
**Done when:**
1. **Behavioral:** The budget can be derived from recorded numbers rather than reasoning, with an
   explicit margin above the failed-holepunch figure and orders of magnitude below the carried-call
   anchor.
2. **Verification:** `evidence/budget-sizing.txt` holds the raw runs and sample counts.
**Validation:** Broad.

### Phase 1: Record the design and correct the record

**Goal:** The corpus states this design, and everything now known to be wrong is fixed, before code
depends on it.

**Changes:**
- [ ] Write ADR-0006: the two gates, budget-and-drop, our-binary-not-a-fork, the decorator, caps as
      opaque-id records, the PDS trust ruling and its blast radius, the request-policy flow, and what
      we store (membership and accounting only).
- [ ] Supersede ADR-0004's mechanism section; **retain** its content-blindness argument.
- [ ] Mark ADR-0001's fork-vs-embed conclusion superseded-in-part.
- [ ] OPEN-QUESTIONS: resolve Q1/Q3/Q4, leave Q2/Q5 with current status.
- [ ] Fix the stale build-plan pointer in the experiment README.
- [ ] Rewrite T62's gates.
- [ ] Backlog §6j, MASTER-INDEX, COHESION, RAW-ARTIFACTS-MANIFEST.
- [ ] Add `rust-toolchain.toml` pinning `1.94.1` to match CI — the experiment has none, which is the
      green-locally/red-in-CI failure `.claude/CI-PATTERN.md` names.
- [ ] Fix the stale MSRV comment in the workspace `Cargo.toml` (it claims `iroh-relay 1.0.0-rc.1`;
      it has been 1.0.3 since ADR-0005).
- [ ] *(Conditional — see Concurrency Map and Open Questions)* the hoisted root-manifest edit.

**Call chain:** n/a (docs plus one toolchain/manifest edit).
**Wiring test:** the repo's `build + broken-ref` gate passes on changed references. **Note it is
currently red on `main`** for unrelated `thinking-app-*` drift; check the failing paths before reading
red as this phase's failure.
**Depends on:** Phase 0 for constants only; structure lands first with marked placeholders.
**Read-set:** the ADR tree, `OPEN-THREADS.md`, `EXPERIMENT-BACKLOG.md`, `COHESION.md`.
**Write-set:** `experiments/croft-relay/docs/adr/*`, `{README,OPEN-QUESTIONS}.md`, `Cargo.toml`,
`rust-toolchain.toml` (new); `beta/OPEN-THREADS.md`; `alpha/experiments/EXPERIMENT-BACKLOG.md`;
`alpha/COHESION.md`; `alpha/seeds/transcripts/RAW-ARTIFACTS-MANIFEST.md`.
**Shared-state contract:** No mutable state beyond the write-set, except that the toolchain pin is
ambient for every later phase — re-run the existing suite once after adding it.
**Risks:** An ADR that records the decision without the rejected alternatives is not re-derivable
later, which is the whole point of writing it.
**Done when:**
1. **Behavioral:** A reader who has not seen the dialogue can reconstruct why throttling was replaced,
   why there is no fork and no switch, and what the PDS trust ruling does and does not assume.
2. **Verification:** `grep -rn "wrap the admitted connection" docs/` returns nothing; broken-ref gate
   clean on changed files.
**Validation:** Narrow.

---

## Milestone B — The relay binary

### Phase 2: Our binary, hosting the relay

**Goal:** Replace the stock binary with our own program, which depends on the unmodified crate. Feature
parity first, no new behaviour.

**Changes:**
- [ ] New crate `crates/croft-relay-bin`: accept loop, TLS, HTTP, dispatch to one
      `RelayServiceWithNotify` via `serve_connection(...).with_upgrades()` — the embedding pattern
      upstream documents.
- [ ] Wire `TokenAccess` (existing, from `croft-relay-embed`) as the `AccessControl`.
- [ ] TOML config: bind addrs, cert paths, verification pubkey. Every knob documented with its why.
- [ ] Preserve upstream's probe path so existing health checks keep working.

**Call chain:** `main()` → config → accept → TLS → parse request → `RelayServiceWithNotify` →
`/relay` upgrade → `TokenAccess::on_connect` → relay carries traffic.
**Wiring test:** A **real relay client** (not a unit stub) connects to the binary started from a config
file, is admitted on a valid token, and relays a datagram A→B. RED before the binary exists.
**Depends on:** Phase 1.
**Read-set:** `crates/croft-relay-embed/src/lib.rs`, `crates/croft-admit/src/{token,tier}.rs`.
**Write-set:** `crates/croft-relay-bin/*`.
**Shared-state contract:** Binds ports; tests use `:0` as the existing live-relay harness does.
**Risks:** Accidentally reimplementing parts of upstream's `main.rs` (ACME, metrics, probe path). Read
it, take only what is needed, record deliberate omissions in the config doc.
**Done when:**
1. **Behavioral:** `croft-relay --config relay.toml` serves a relay a stock iroh client can use, gated
   by our token.
2. **Verification:** `cargo test -p croft-relay-bin --test live_binary`.
**Validation:** Moderate — also run it by hand against a real iroh endpoint.

### Phase 3: The counting decorator (visibility, no enforcement)

**Goal:** Know what every connection costs, attributed to an identity. Measurement before enforcement,
deliberately: we want the numbers before we act on them.

**Changes:**
- [ ] `CountingStream<S>`: implements `AsyncRead`/`AsyncWrite`, increments byte counters, wraps the
      stream before it is handed to `serve_connection`.
- [ ] **(Pass 2 — corrected design.) Two decorators, not one.** The Pass-1 text said "the token is read
      at dispatch, before the wrap, so the association is available at wrap time." That is **wrong**:
      with `serve_connection(TokioIo::new(stream), service)`, hyper parses the request *after* we hand
      it the stream, so at wrap time the connection is anonymous. The association needs three
      participants:

      ```
      CountingStream        wraps the stream. Counts bytes. Knows no identity.
            │  Arc<ConnCounter>
            ▼
      our Service wrapper   sees Request<Incoming> → reads the bearer token.
            │               Records token → Arc<ConnCounter>. Delegates to
            │               RelayServiceWithNotify (upstream type; we cannot
            ▼               add fields to it, hence the wrapper).
      TokenAccess::on_connect
                            supplies endpoint_id() and connection_id() — both
                            public accessors on ClientRequest — plus auth_token().

      join key = the token, which all three see. Tokens are per-call and
      short-lived, so token → connection is 1:1 in practice.
      ```

      The result is `(endpoint_id, connection_id, bytes)` — exactly what Phase 4's `disconnect` needs.
- [ ] **Verify in-phase, do not assume:** that post-upgrade traffic still flows through `CountingStream`.
      The Service takes the upgrade from the request (`handle_relay_ws_upgrade`), and hyper hands the
      original IO to the upgraded task, so it should — but the entire counting design rests on it, so
      assert it with a test that pushes a known payload *after* the websocket is established.
- [ ] The authoritative verification stays in `on_connect`; the wrapper's token read is for attribution
      only and must never be treated as authorization.
- [ ] On close, emit `(subject, bytes_in, bytes_out, duration)`.
- [ ] Document that the count includes framing — an **upper bound**, suitable for budgets and capacity,
      never described as billing.

**Call chain:** accept → TLS → parse request (token known) → `CountingStream::wrap` →
`RelayServiceWithNotify` → … → close → usage record.
**Wiring test:** Two clients push known payload sizes through the running binary; the emitted records
attribute the right order-of-magnitude byte counts to the right subjects. Assert attribution and
ordering, not exact byte equality — framing overhead makes exactness a false precision.
**Depends on:** Phase 2.
**Read-set:** `crates/croft-relay-bin/src/main.rs`.
**Write-set:** `crates/croft-relay-bin/src/{counting,usage}.rs`, `src/main.rs`.
**Shared-state contract:** In-process counters only; nothing persisted in this phase.
**Risks:** The decorator sits in the hot path of every byte — keep it to counter increments and no
allocation. Second risk: the pre-verification token read becoming load-bearing for anything but
attribution.
**Done when:**
1. **Behavioral:** Every closed connection yields a usage record naming its subject.
2. **Verification:** `cargo test -p croft-relay-bin --test usage_accounting`.
**Validation:** Moderate.

### Phase 4: Budget enforcement and the clean drop

**Goal:** The product mechanism. A connection whose budget is spent is disconnected, cleanly, and its
token is not re-admitted.

**Changes:**
- [ ] `Tier` → **budget** rather than rate bucket: rewrite `tier.rs`'s `bucket_for` as `budget_for`,
      returning `Budget::Unlimited` (member-involved) or `Budget::Bytes(n)` (introduction only). Rewrite
      the `SPEC-DELTA` comment around Phase 0's sizing.
- [ ] Supervisor: when a counted connection exceeds its budget, call
      `RelayService::clients().disconnect(endpoint_id, connection_id)`. **(Pass 2:** passing `None` for
      the connection id drops *every* connection for that endpoint, which is simpler and is what we
      want — verified at `clients.rs:172-195`. `ConnectionId` is public if we ever need precision.**)**
- [ ] **(Pass 2) Treat the budget as a threshold, not a hard cap.** `disconnect` is documented as
      asynchronous — "each per-connection actor exits its run loop and unregisters itself after this
      call returns" — so bytes can still flow between the decision and the close. Overshoot is expected
      and acceptable; **the test must not assert an exact byte ceiling**, or it will be flaky for a
      reason that isn't a bug.
- [ ] **Spent-token refusal:** record the spent token id; `TokenAccess::on_connect` denies re-admission
      on it. Without this, iroh's automatic reconnect-with-backoff turns one drop into a flap.
- [ ] No `client_rx` rate limit configured on the service at all — the budget replaces it. Keep a
      coarse global limit only as an anti-hammering backstop, documented as such.
- [ ] **Verify first:** does `Endpoint::insert_relay` with a changed auth token force a reconnect? The
      budget boundary depends on the answer. If it reconnects, a sponsored upgrade naturally starts a
      fresh budget; if it does not, the supervisor must re-read the budget on an existing connection.
      Settle this with a probe **before** implementing, and record the result in Verified Assumptions.

**Call chain:** usage record crosses budget → supervisor → `Clients::disconnect` → client's
`ActiveRelayActor` retries → `on_connect` denies the spent token → client surfaces the limitation.
**Wiring test:** A non-member pair whose holepunch is forced to fail pushes past its budget and is
**disconnected**, and its immediate reconnect is **refused**; a member-involved pair on the same binary
pushes the same volume and is not disconnected. This single test is the product.
**Depends on:** Phase 3 and Phase 0 (for the number; a placeholder is acceptable to build against but
not to deploy).
**Read-set:** `crates/croft-admit/src/{tier,token}.rs`, `crates/croft-relay-bin/src/usage.rs`.
**Write-set:** `crates/croft-relay-bin/src/{supervisor,main}.rs`,
`crates/croft-admit/src/tier.rs`, `experiments/croft-relay/DESIGN.md`.
**Shared-state contract:** The spent-token set is process-local and lost on restart — acceptable
(tokens are short-lived) but state it, because "revocation survives restart" is a claim someone will
otherwise assume.
**Risks:** Dropping a connection whose holepunch *succeeded* would be invisible in testing (the direct
path carries on) but wasteful. Assert that a successful-holepunch pair never has its budget consumed.
Second: disconnect must be a clean close the client can distinguish from a network fault, or the app
cannot surface an honest reason.
**Done when:**
1. **Behavioral:** A non-member call that cannot holepunch ends with an honest "needs membership,"
   rather than degrading; a member-involved call is carried.
2. **Verification:** `cargo test -p croft-relay-bin --test budget_drop`.
**Validation:** Broad + mutation run on `tier.rs` and the supervisor (enforcement path — standing
no-unexplained-survivors policy applies).

### Phase 5: Deployment — our artifact replaces the tarball

**Goal:** `relay.croft.ing` runs our binary, pinned and checksummed as the upstream tarball was, without
loosening the netns/systemd posture.

**Changes:**
- [ ] CI job producing an `x86_64-unknown-linux-musl` release artifact with a published checksum.
- [ ] `croft-stack` relay role fetches our artifact instead of `relay_tarball_url`; checksum pinning,
      netns isolation, cgroup envelope, and DNAT ingress all unchanged.
- [ ] Effective bump 1.0.0 → 1.0.3 (three releases behind today).
- [ ] Keep the stock-binary + HTTP-hook path documented as the **fallback**: it can only answer yes/no,
      but it restores registered-only admission in minutes if our binary misbehaves.
- [ ] Update `06-iroh-relay.md`; add the ROADMAP_TODO E-item.

**Call chain:** CI tag → artifact + checksum → Ansible `get_url` + checksum → `/opt/iroh-relay/` →
systemd unit (unchanged shape) → netns.
**Wiring test:** A staging deploy where a real client is admitted on a real token and the unit reports
active with `systemd-analyze security` exposure no worse than the recorded 1.7.
**Depends on:** Phase 4.
**Read-set:** `croft-stack/ansible/roles/relay/tasks/main.yml`, `group_vars/all.yml`,
`alpha/plans/croft-stack/{06-iroh-relay,netns-isolation-plan}.md`.
**Write-set:** `croft-stack/ansible/roles/relay/*`, `group_vars/all.yml`, release workflow,
`alpha/plans/croft-stack/06-iroh-relay.md`, `alpha/ROADMAP_TODO.md`.
**Shared-state contract:** **Touches production** — deploys to a live host, restarts a live service,
changes an artifact URL. Requires owner authorization at execution time; does not run unattended.
**Risks:** Owning the release cadence becomes a standing obligation — say so in the ADR rather than
discovering it at the next CVE. The proven netns/hardening posture must not regress as a side effect of
changing how the binary arrives.
**Done when:**
1. **Behavioral:** `relay.croft.ing` serves from our binary; the netns Phase-0 result (5/5 direct, 5/5
   forced-relay) still holds.
2. **Verification:** the stack review's existing prove-commands, re-run.
**Validation:** Broad; staging before production.

---

## Milestone C — The admission service

### Phase 6: `croft-admit` as a running service, with the small store

**Goal:** Make the admission authority a program, with durable state for the two things that are
legitimately ours.

**Changes:**
- [ ] `[[bin]]` target: bind, serve the router, config for keys and store path.
- [ ] Durable store behind a narrow interface, holding **only**: membership, and accounting/quota
      counters. Explicitly **not**: allowlists, call policies, or any pair.
- [ ] Keep the in-memory implementation for tests.
- [ ] Fail loudly if the store path is unwritable — never silently fall back to in-memory.
- [ ] `IdIndex` seam over stored identifiers: `Transparent` (dev/test) and `Keyed`, both behaviourally
      tested, with the migration written now while the store is small.
      **(Pass 2 — the rationale was stale.)** The Pass-1 form was `HMAC(k, member ‖ counterpart)`,
      justified by stopping one person recurring as the same digest across many members' lists. **The
      rewrite deleted all the lists** — storage is membership and accounting, both keyed by member
      alone. So the key is `HMAC(k, member_did)`, and the surviving value is narrower but real: a leaked
      accounting store does not reveal our **member roster**, which is a customer list. If that is not
      worth a seam, the honest move is to drop `IdIndex` rather than keep it for a reason that no longer
      applies. **The ADR must state the security property is not claimed** in either mode
      until key custody is answered — otherwise "we hash the DIDs" gets read a year later as "they are
      protected" by someone who does not know the key sits in the same backup. In keyed mode an absent
      key is a hard error, never a default.

**Call chain:** `main()` → config → store open → router → bind → serve.
**Wiring test:** Start the binary against a tmp store, record a membership over HTTP, **restart the
process**, and confirm it survives. Restart is the assertion — an in-memory store passes every test
that does not restart.
**Depends on:** Phase 1.
**Read-set:** `crates/croft-admit/src/{registry,access,http_api,enroll}.rs`.
**Write-set:** `crates/croft-admit/src/{store,id_index,main}.rs`, `Cargo.toml`.
**Shared-state contract:** Binds a port, opens a store file. Tests use `:0` and tmp paths. Keyed mode
reads a key from the environment and must not proceed without it.
**Risks:** Scope creep back into holding a graph. The store's interface should make "add an allowlist"
awkward rather than easy. Second: store-engine choice has a long tail — keep the interface narrow so it
stays replaceable.
**Done when:**
1. **Behavioral:** Membership and accounting survive restart; nothing about relationships is stored.
2. **Verification:** `cargo test -p croft-admit --test persistence` (includes restart) and
   `--test id_index` (both impls plus migration).
**Validation:** Moderate + mutation run on `id_index`.

### Phase 7: Real atproto resolution

**Goal:** Talk to actual servers. Everything to date has run against fixtures.

**Why it is a phase and not a bullet:** `PdsResolver` is a trait with an in-memory fixture and no HTTP
client; `Did` is deliberately opaque so nothing resolves DID *documents*; and `ciss-auth` supplies a
verifier but **not** resolution — `ResolvedKeys` must be handed to it. These are **three** lookups, all
network-facing, all in an authentication path.

**Changes:**
- [ ] `handle → DID` via `com.atproto.identity.resolveHandle`.
- [ ] `DID → PDS` via **plc.directory** for `did:plc`, `.well-known/did.json` for `did:web`.
- [ ] `DID → document keys` producing `ciss_auth::ResolvedKeys`.
- [ ] Record reads: `com.atproto.repo.listRecords` for device records and for cap-grant records.
- [ ] **Cache with an explicit TTL**, since a plc.directory round-trip inside every call setup is both
      latency and an availability coupling.
- [ ] **Fail closed** on any resolver failure — owner's ruling — with a test asserting it, because
      "fail open under load" is the classic regression. Accept and document the consequence: a
      plc.directory outage stops new call setup for `did:plc` identities.
- [ ] Keep fixtures; no test may touch the live network.

**Call chain:** call setup → resolver (cache or network) → keys/records → verification.
**Wiring test:** End-to-end against a **local fixture HTTP server** (not a trait stub) serving
plc.directory-shaped and `listRecords`-shaped responses; plus an asserted deny on resolver timeout.
**Depends on:** Phase 6.
**Read-set:** `crates/croft-admit/src/{pds,did,enroll}.rs`, `CISS/crates/ciss-auth/src/lib.rs`.
**Write-set:** `crates/croft-admit/src/{pds_client,did_doc,cache}.rs`, `Cargo.toml`.
**Shared-state contract:** First outbound network dependency in this crate. Tests bind a local fixture
server on `:0`; **CI must have no egress expectation.** Cache is process-local with a TTL.
**Risks:** Under-scoping this a second time — handle resolution, DID methods, PDS discovery, document
parsing, and record listing are five failure surfaces. Cache invalidation on key rotation is a real
correctness question: state the TTL and its rationale rather than picking silently. Corpus rule: atproto
mechanics are **cited from the FACTCHECK docs, not re-verified**.
**Done when:**
1. **Behavioral:** A real handle resolves to real device records, and a real service-auth JWT verifies
   against its DID-resolved key.
2. **Verification:** `cargo test -p croft-admit --test resolution`, plus one manual run against the
   owner's test account recorded in `evidence/`.
**Validation:** Broad — network-facing, authentication path.

### Phase 8: Caps, and minting relay tokens with a budget

**Goal:** The authorization core. Verify a cap issued by a callee; mint the short-lived relay token
carrying tier and budget.

**Changes:**
- [ ] Cap format: an opaque id, plus whatever the holder presents to prove possession. The callee's repo
      holds a grant record naming **only** the opaque id.
- [ ] Verification: fetch the issuer's grant records (Phase 7, cached), confirm the presented cap
      matches a live record.
- [ ] **Revocation is both** — short expiry on the minted relay token, *and* deletion of the grant
      record to stop re-issue. Expiry alone would let a revoked cap keep minting until it lapsed;
      record-deletion alone would leave already-minted tokens live. Together, revocation takes effect
      within one token lifetime and cannot be renewed.
- [ ] `sponsorship_for(caller, callee, membership, now) -> Budget`: member-involved → unlimited;
      otherwise → introduction budget. Mint to **both** parties.
- [ ] Service-auth JWT (via `ciss-auth`) authenticates a member's instruction where one is needed;
      `ReplayGuard` for `jti`.
- [ ] Quota decrement from the usage records Phase 3 emits.
- [ ] Record the fate of `RegistryAccess` and the HTTP-hook path: **kept, labelled as the fallback**
      (Phase 5), not deleted.

**Call chain:** client → `/grantCall` → resolve → verify cap → `sponsorship_for` → mint → client swaps
its relay token.
**Wiring test:** A caller holding a valid cap is minted a token; the callee **deletes the grant record**;
the next mint attempt is refused. Deletion-then-refusal is the assertion — testing only the happy path
proves nothing about revocation.
**Depends on:** Phase 7, **and Phase 3 (Pass 2)** — the quota decrement consumes the usage records the
counting decorator emits. Pass 1 declared only Phase 7, hiding a cross-milestone dependency from
Milestone B into Milestone C.
**Read-set:** `crates/croft-admit/src/{token,tier,http_api}.rs`, `CISS/crates/ciss-auth/src/*`,
`crates/croft-relay-bin/src/usage.rs`.
**Write-set:** `crates/croft-admit/src/{cap,sponsorship,quota,token,http_api}.rs`.
**Shared-state contract:** Clock injected as the existing verifier does; no wall-clock reads outside the
edge. `ciss-auth` enters as a dependency (path or git — Open Questions).
**Risks:** Reimplementing JWT verification instead of reusing `ciss-auth` — it is the highest-risk
crypto surface and the existing code is reviewed. Second: an unenforced quota is worse than none;
assert exhaustion behaviour, not just accounting.
**Done when:**
1. **Behavioral:** Only a cap-holder can obtain a token; deleting the grant stops it; member
   involvement produces an unlimited budget and its absence an introduction budget.
2. **Verification:** `cargo test -p croft-admit --test caps`.
**Validation:** Broad + mutation run (authorization path).

*(Phase 9 — "the request-to-token flow" — was removed on 2026-08-09 when cap distribution was settled
as **out-of-band only**. With no in-band request, `croft-admit` never sees a request and has nothing to
build: the policy record is a lexicon concern and cap issuance is the callee's client writing to its
own repo. Both folded into Phase 10. See §5 and the Review Log.)*

---

## Milestone D — Client and lexicon

### Phase 10: Per-device lexicon, policy record, and cap distribution (in `CroftCommunity/connect`)

**Goal:** One identity, several devices, discoverable in one call — plus the two records that make
cap-gated calling work, and the client affordances that issue and redeem a cap.

**Changes — caps and policy (folded in from the removed Phase 9):**
- [ ] **Grant record** in the callee's own repo, carrying an **opaque cap id and nothing else**. It
      names no grantee, so the call graph is not published. Deleting the record is revocation.
- [ ] **Policy record** in the callee's own repo: `anyone | mutuals | nobody`. A small enum, not a
      list. **It is advisory UI, not enforcement** — the cap is the gate. The policy exists so the
      exchange page knows what to render, and so a user can say "don't bother asking" without that
      statement disclosing anything about who they know.
- [ ] Page behaviour per policy: `nobody` renders "not callable"; `mutuals` and `anyone` render an
      "ask them for an invite" affordance that explains the **out-of-band** step. The page must refuse
      identically whether the policy is `nobody` or the handle does not exist — a refusal that leaks
      existence is a worse privacy failure than the allowlist this design avoided.
- [ ] **Cap issuance in the callee's client:** write a grant record, and produce a shareable artifact
      (link or QR) carrying the cap. This is the whole distribution mechanism.
- [ ] **Cap redemption in the caller's client:** accept a pasted or scanned cap and store it against
      that callee.
- [ ] `anyone` is **built and tested as the counter-case** so we understand what enabling it means. Not
      advertised.

**Changes — devices:**
- [ ] Lexicon moves from rkey `self` to **one record per device, rkey = the device label** (`home`,
      `work`, `phone`), each carrying `endpointId`, optional `homeRelay`, `createdAt`, and a
      human label.
- [ ] Discovery becomes `com.atproto.repo.listRecords` — atproto's native "give me all of them" call,
      public and unauthenticated, so the exchange page stays backendless. Adding a device does not
      rewrite the others; removing one is a record delete.
- [ ] `docs/contract.md` first, then both halves' tests, then the implementations — the repo's own rule.
      The contract now covers **three** record shapes (device, grant, policy) plus the deep link.
- [ ] Update `web/resolver.js`, `web-tests/`, and `android/.../DeepLink.kt` accordingly.
- [ ] Deep link gains a device selector or the client tries devices in order — decide in the contract.

**Call chain:** page → `resolveHandle` → PDS → `listRecords` (devices + policy) → render → deep link →
app. Separately: callee's client → write grant record → share cap out-of-band → caller's client stores
it → presents it at call setup (Phase 8 verifies it).
**Wiring test:** Two assertions, both end to end. (a) A test account with **two** device records
resolves to both and the page produces a working link for each — two devices is the assertion, one
hides every bug this introduces. (b) A cap issued by one persona's client is redeemed by another's and
verifies against the grant record; deleting the record makes it stop verifying.
**Depends on:** Phase 8 for the verification side. The lexicon and page work may start as soon as the
shape is agreed.
**Read-set:** `connect/docs/contract.md`, `web/resolver.js`, `android/.../DeepLink.kt`.
**Write-set:** the same files plus the cap issue/redeem paths, in `CroftCommunity/connect`. **Different
repo** — separate PR, separate CI.
**Shared-state contract:** Writes records to the owner's test account. No shared state with this repo.
**Risks:** Nothing is published yet, so the lexicon change is free now and a migration later. Second:
the policy record is easy to mistake for enforcement — if any code path treats it as a gate, the cap
stops being the only gate and the design's simplicity is lost. Assert that a caller with a valid cap is
admitted **regardless** of the callee's policy setting.
**Done when:**
1. **Behavioral:** A two-device persona is resolvable and callable on either device, and a cap can be
   issued, shared out-of-band, redeemed, and revoked.
2. **Verification:** `npm test` in `connect` plus the Android unit tests against the new contract.
**Validation:** Moderate, plus a live run against the owner's test account.

### Phase 11: Client integration

**Goal:** The client obtains a token, applies it live, and surfaces budget exhaustion honestly.

**Changes:**
- [ ] Call setup: present cap → obtain relay token → apply via
      `Endpoint::insert_relay(url, RelayConfig::with_auth_token(t))`.
- [ ] Renewal for the call's duration.
- [ ] Surface a budget drop as **a tier limitation, not a bug** — the original build plan's app-facing
      note, and the entire point of choosing a clean drop over degradation.
- [ ] Do-not-disturb and a zero-cost blocklist. Owner's ranking: these matter more than eviction.

**Call chain:** app call action → `/grantCall` → token → `insert_relay` → dial → (budget spent) →
disconnect → honest UI.
**Wiring test:** A sponsored client swaps its token live and the **EndpointId is unchanged** across the
swap — that stability is what keeps the published records valid. Plus: a non-member call that cannot
holepunch shows the membership message rather than failing silently.
**Depends on:** Phase 4, Phase 8, Phase 10.
**Read-set:** `crates/croft-relay-bin/src/main.rs`, `crates/croft-admit/src/http_api.rs`,
`connect/android/*`.
**Write-set:** client integration crate (location gated on the repo-shape Open Question).
**Shared-state contract:** Holds a live iroh Endpoint; tests bind ephemeral ports.
**Risks:** A rebind creeping in would invalidate published records — assert EndpointId stability
explicitly.
**Done when:**
1. **Behavioral:** A sponsored call upgrades mid-session with no record change and no EndpointId change;
   an unsponsored one ends with a clear reason.
2. **Verification:** the end-to-end test above.
**Validation:** Broad.

---

## Milestone E — Hardening and operations

### Phase 12: Deny-path, metrics, fuzz, load

**Goal:** A relay that gates strangers will meet strangers.

**Changes:**
- [ ] Cheap rejects; no logging amplification on the deny path; per-source attempt limiting ahead of
      verification cost.
- [ ] Metrics: admissions by outcome, budgets exhausted, bytes by tier, spent-token refusals — **tier-
      level aggregates**, per OPEN-QUESTIONS Q5, unless explicitly overridden.
- [ ] Fuzz the token parser (network-facing), time-boxed.
- [ ] Load test: N clients, stable memory, correct budget behaviour, decorator overhead measured.

**Call chain:** deny path → counters; parser → fuzz harness.
**Wiring test:** Under load, denied connections allocate no per-attempt log lines and budget counters
move correctly end to end.
**Depends on:** Phase 11.
**Read-set:** all of Milestones B and C.
**Write-set:** `crates/croft-relay-bin/*`, `crates/croft-admit/src/metrics.rs`, fuzz targets.
**Shared-state contract:** Load tests bind many ports; staging only, never production.
**Risks:** Metric cardinality is a privacy decision, not only an ops one. Defaulting to endpoint-level
labels because they help debugging is the failure mode. Second: the decorator is in every byte's path —
measure its overhead rather than assuming it is free.
**Done when:**
1. **Behavioral:** Under load the deny path is cheap, budgets hold, dashboards render tier-level
   aggregates.
2. **Verification:** load harness output plus a clean time-boxed fuzz run.
**Validation:** Broad.

---

## Open Questions

- ~~**[Phase 1]** Hoist dependency entries to keep B and C parallel?~~ **(Pass 2: MOOT.)** Phase 4
  writes `crates/croft-admit/src/tier.rs`, a Milestone C file, so B ∥ C is disqualified by something no
  hoist can fix. B and C run sequentially and each adds its own deps per the repo's rule.
- ~~**[BLOCKING]** Reachability / request delivery transport.~~ **(RESOLVED 2026-08-09: out-of-band cap
  distribution only.)** With no in-band request there is nothing to deliver, so nothing in this plan
  needs to reach a user who isn't watching. Foreground reachability remains real per-platform work
  (§1.1) but gates none of this. Phase 9 was removed and its content folded into Phase 10.
- **[RECOMMENDED: PHASE-GATED — Phase 6] (Pass 2 — new)** Does `IdIndex` still earn its place? Its
  original rationale (breaking cross-list correlation) died with the lists. The surviving value is that
  a leaked accounting store doesn't expose the member roster. *Rationale: keep it for that reason, or
  drop it — but do not keep it for the reason written in Pass 1.*
- **[RECOMMENDED: PHASE-GATED — Phase 4]** Does `Endpoint::insert_relay` with a changed auth token force
  a relay reconnect? *Rationale: it determines whether a sponsored upgrade starts a fresh budget
  naturally or the supervisor must re-read budgets on live connections. Settle by probe before
  implementing; it is cheap to answer and expensive to assume.*
- **[RECOMMENDED: PHASE-GATED — Phase 7]** DID-document cache TTL, and the accepted consequence of
  fail-closed (a plc.directory outage stops new `did:plc` call setup). *Rationale: fail-closed is
  decided; the TTL trades outage exposure against serving a rotated key past its rotation.*
- **[RECOMMENDED: PHASE-GATED — Phase 8]** `ciss-auth` as a path dependency, a git dependency, or
  vendored. *Rationale: cross-repo coupling versus drift. Reimplementation is not on the table.*
- **[RECOMMENDED: PHASE-GATED — Phase 10]** Deep link with a device selector, or client tries devices in
  order? *Rationale: it is a contract change, and the contract is the source of truth for two
  codebases.*
- **[RECOMMENDED: PHASE-GATED — Phase 11]** Repo shape (OPEN-QUESTIONS Q2), now unavoidable because a
  client crate needs a home: does croft-relay graduate to a standalone repo, or stay under
  `discovery/alpha/experiments/`? *Rationale: the experiment tree is the wrong long-term home for a
  deployed production binary.*
- **[RECOMMENDED: PHASE-GATED — Phase 12]** OPEN-QUESTIONS Q5: tier-level aggregates only, or
  endpoint-level labels authorized? *Rationale: currently defaulted to tier-level; Phase 12 is where the
  default becomes a shipped choice.*
- **[RECOMMENDED: ADVISORY]** Store engine for Phase 6. *Rationale: keep the interface narrow so it
  stays replaceable; low stakes now, expensive after Milestone C builds on it.*
- **[RECOMMENDED: ADVISORY]** Owning the relay release cadence becomes a standing obligation once Phase
  5 lands — we are already three releases behind on a *prebuilt* binary. Who watches upstream releases?

## Review Log

### Pass 1 — 2026-08-07 (superseded)
Initial plan: split relay instances behind a tier-routing switch, sponsorship at mint time, JWT grant
flow, `IdIndex` seam, byte quota. Eleven phases in five milestones.

### Pass 2: Gap Analysis — 2026-08-08 (superseded by the rewrite, findings retained)
Verified the plan's claims against the code rather than its own logic. **Found:** `Registry` is
in-memory with no persistence and only one lookup direction; `croft-admit` has no binary; `PdsResolver`
is a fixture-only trait and the promised production adapter is unbuilt; `ciss-auth` does not resolve
DIDs; multi-device is unaddressed and touches the lexicon; no `rust-toolchain.toml` against a CI-pinned
toolchain; a stale MSRV comment; Phase 0 could not measure a media call that does not exist; the
throughput assertion was a flaky-test shape. **Concurrency:** the B ∥ C grouping was disqualified —
both write the workspace root `Cargo.toml`. **Confirmed:** the embedding seam
(`RelayServiceWithNotify`, `Handlers`, `KeyCache`, `Metrics`, `handle_connection`) is fully reachable —
the load-bearing feasibility claim held.

### Comprehensive rewrite — 2026-08-08

Triggered by the owner: "our starting plan got enough things flat out wrong that I want to review the
feasibility and plan in its entirety." Patching was stopped in favour of one coherent revision.

**Superseding changes:**

- **Enforcement mechanism replaced.** Rate-limiting non-members until media fails is out; a **byte
  budget with a clean disconnect** is in. Owner's objection — throttling-to-failure is ugly — was
  correct. Recorded alongside the mechanical correction it forced: the relay carries the call from the
  first packet, so there is no escalation event to refuse, only a flow to budget.
- **The tier switch is deleted.** With the budget enforced in our own layer, multiple relay instances
  with differing rate limits are unnecessary. Removed: routing switch, N-instance construction,
  cross-tier meeting-room problem, and the risk of the switch being mistaken for the authorization
  point. Largest simplification in this revision, and a direct consequence of fixing the mechanism.
- **Two gates made explicit.** Cap (may you call me — callee's decision) and membership (will we carry
  you — commercial). Previously entangled.
- **Caps live as opaque-id records in the callee's own repo.** Owner's ruling that a signed record in a
  user's PDS is that user's statement, regardless of who holds the key, with the risk on their side.
  Blast radius recorded: forged grants buy bandwidth, not conversations.
- **Storage shrank to membership and accounting.** Person→endpoint is a resolve-and-cache, not a table
  we own; policy lives in the user's repo. Phase 5a of the previous draft largely dissolved into
  Phase 6.
- **Byte accounting is ours.** iroh exposes only server-wide counters, so per-user accounting comes
  from a counting decorator in our own binary. The upstream accounting patch is **explicitly not
  attempted** (owner's call) but retained as a future contribution, volume-only and never pairs.
- **Our binary is not a fork** — stated as a table, because the conflation was driving the
  Path-A-as-destination reasoning. Path A demoted to a documented fallback that can only answer yes/no.
- **Per-device lexicon** (rkey = device label, `listRecords`), plus the request-policy record and the
  request-to-token flow, which the owner ruled must exist and must be the user's choice.
- **Revocation is both** expiry and grant-record deletion, with the reason each alone is insufficient
  spelled out.

**Retained from Pass 2 unchanged:** every verified-source finding; the toolchain pin; the MSRV fix;
the fixture-only resolution gap (now Phase 7); the `Cargo.toml` write-set collision and its conditional
resolution; the restart-based persistence assertion.

**Phase count:** 13 → 13, but substantially different: Phases 2–5 (relay binary) replace the old
switch-and-instances work; Phases 6–9 (admission) absorb the old 5/5a/6/6a/7/8; Phase 10 (lexicon, in a
different repo) is new.

### Pass 2: Gap Analysis (against the rewrite) — 2026-08-08

**Found:**

- **The counting decorator was designed wrong.** Pass 1 said "the token is read at dispatch, before the
  wrap, so the association is available at wrap time." False — with
  `serve_connection(TokioIo::new(stream), service)`, hyper parses the request *after* receiving the
  stream, so the connection is anonymous when we wrap it. Needs **two** decorators (a stream wrapper
  that counts, a Service wrapper that reads the token) joined by shared state, with `on_connect`
  supplying `endpoint_id()`/`connection_id()`. Join key is the token, which all three see. Phase 3
  rewritten with the diagram.
- **Phase 4 writes a Milestone C file.** `crates/croft-admit/src/tier.rs` — the tier→budget rewrite
  lives there. This is a second write-set collision beyond the root manifest, and unlike that one it
  cannot be hoisted away.
- **Phase 8 had an undeclared cross-milestone dependency** — quota decrement consumes the usage records
  Phase 3 emits, but only Phase 7 was declared.
- **The `IdIndex` rationale is stale.** `HMAC(k, member ‖ counterpart)` existed to break cross-list
  correlation; the rewrite deleted every list. Re-keyed to `member_did` with the narrower surviving
  justification (a leaked accounting store shouldn't expose the member roster), and flagged as
  drop-if-not-worth-it rather than kept on a dead reason.
- **Phase 9's delivery has no transport**, and behind it the plan never stated that the whole product
  currently requires both parties to have the app in the foreground. Added as §1.1 and raised to a
  BLOCKING open question — it bounds what any of this delivers.
- **`disconnect` is asynchronous**, so budget overshoot between decision and close is expected. The
  budget is a threshold, not a hard cap, and the test must not assert an exact ceiling or it will be
  flaky for a non-bug reason.
- **`listRecords` is unverified.** FACTCHECK doesn't cover it; `contract.md` evidences only
  `resolveHandle` and `getRecord`. It is this plan's addition, asserted from general knowledge, and
  Phase 10 builds the contract on it.
- **Post-upgrade byte flow through `CountingStream` is understood but untested** — the counting design
  fails silently if it doesn't hold, so Phase 3 now asserts it explicitly.

**Concurrency:**

- **Milestone B ∥ C disqualified outright** and moved to sequential (B then C), on the `tier.rs`
  collision rather than the manifest one. This **moots the dep-rule open question** and drops the
  hoisted-manifest item from Phase 1 — each milestone adds its own dependencies per the repo's stated
  rule. Sequencing B first is also correct on merit, since Phase 8 consumes Phase 3's output.
- Phase 0 ∥ Phase 1 confirmed unchanged.
- Phase 10 (different repo) confirmed independent.

**Changed:** §1.1 added; Phase 3's decorator design corrected with the three-way join; Phase 4 gained
the async-disconnect threshold caveat and the `None`-drops-all simplification; Phase 6's `IdIndex`
rationale rewritten; Phase 8's dependency and read-set corrected; Phase 9's delivery gap named;
Concurrency Map rewritten to sequential; five new Verified Assumption rows plus two new unverified
entries; one open question mooted, two added (one BLOCKING).

**Confirmed:**

- `Clients::disconnect(endpoint_id, None)` drops every connection for an endpoint — simpler than Pass 1
  assumed, and `ConnectionId` is public if precision is ever needed.
- `ClientRequest` exposes everything the join needs as public accessors.
- The embedding seam holds, as it did in the previous pass. Nothing found argues against
  budget-and-drop, against our-own-binary, or for reinstating the tier switch.
- The reasoning in §1 and §3 survives: every finding is missing or mis-specified *work*, not a wrong
  direction.

### Cap distribution settled — 2026-08-09

The Pass-2 BLOCKING question was poorly framed: it conflated **platform reachability** (per-OS work the
owner had already correctly scoped out) with a design question that was ours — *does a cap request need
an in-band path at all?* Re-asked properly, the owner's answer was immediate: **out-of-band is the only
model we are pursuing.**

**Changed:**

- §5 rewritten from "the request-to-token flow" to "cap distribution: out-of-band only," with the two
  rejected in-band shapes recorded — the `croft-admit` queue, and the requester's-repo record that
  looks infrastructure-free but makes "X wants to call Y" a public firehose fact. Both added to the
  alternatives table.
- **Phase 9 removed.** With no in-band request, `croft-admit` never sees one and had nothing to build:
  the policy record is a lexicon concern and cap issuance is the callee's client writing to its own
  repo. Both folded into Phase 10, which is now "per-device lexicon, policy record, and cap
  distribution." Phase count 13 → 12; Milestone C is P6–P8.
- **The policy record's role narrowed to advisory UI.** It tells the exchange page what to render; the
  cap is the only gate. Phase 10 now asserts that a valid cap is admitted **regardless** of the policy
  setting, so nobody can quietly turn the policy into a second gate.
- §1.1 downgraded from a blocking question to a stated precondition, with an explicit note that nothing
  below depends on it.
- The BLOCKING open question struck, marked resolved.

**Confirmed:** the identity-leak property survives — the grant record carries an opaque cap id and
names nobody, the policy record is an enum rather than a list, and the page refuses identically for
`nobody` and for a nonexistent handle.

**Next:** Pass 3 (quality gates — TDD ordering, diagnostic logging, validation calibration). No
outstanding blocking questions.
