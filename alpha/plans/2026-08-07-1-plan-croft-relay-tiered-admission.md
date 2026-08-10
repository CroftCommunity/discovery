# croft-relay: cap-gated calling with a metered introduction budget

- **Status:** Rewrite (2026-08-08), Pass 2 complete against the rewrite (2026-08-08), cap distribution
  settled out-of-band (2026-08-09), **Pass 3 (quality gates) complete and all nine open questions closed
  with the owner (2026-08-09)**. Supersedes the 2026-08-07 draft and its gap analysis — both preserved
  in the Review Log, not above it. **12 phases** in five milestones. **No open questions. Ready for
  execution, starting with Phase 1's relocation to `croft-stack/relay/source/`.**
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
  no cap                    → cannot call (may ask for one out of band — see §5)
  cap, neither is a member  → introduced. Byte budget. Spent → connection dropped, cleanly.
  cap, either is a member   → carried when needed. No budget.
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

### 1.2 Plain English: user stories, storage, and end state (owner-reviewed 2026-08-10)

The §1 framing, retold at the level a non-engineer — or an engineer in a year — needs. Reviewed and
corrected by the owner; the corrections are folded in, not appended.

**What it is.** A phone system where nobody can cold-call you, nobody keeps a record of who talks to
whom, and the thing you pay for is honest: bandwidth, when a direct connection can't be made. atproto
answers "who are you," iroh answers "how do two devices connect," and `relay.croft.ing` covers the
~10–15% of pairs whose networks won't let them connect directly.

**The cap works like giving out your phone number.** Generate an invite — link or QR — and hand it
over however you like: text, email, in person. Nothing traverses our infrastructure. No cap, no call:
strangers can find your page, but they cannot ring you. Revocation is deleting one record from your
own repo.

**Membership means the relay carries you when needed** — it does not route you through the relay. A
member's calls holepunch at the same rate as anyone's; most are direct and free. Membership removes
the budget for whatever fraction can't be, in either direction: a member calling anyone, or anyone
calling a member.

**Where invite data lives, and what is public** — stated precisely, because the comfortable version
overstates it:

| Where | What | Visibility |
|---|---|---|
| Callee's atproto repo | grant record: opaque cap id + device scope | **public**, names nobody |
| Callee's atproto repo | policy record (`anyone\|mutuals\|nobody`), device records | **public**; an enum and endpoint ids |
| Caller's device | the cap secret itself | local, never published |
| Our servers | nothing about invites — the grant record is read at mint and forgotten | n/a |

So: an observer **can** see that you have issued N invites and which of your devices each reaches.
They **cannot** see who holds any of them — the secret half exists only on the invitee's device, and
verification is the presented secret matching the public opaque record. "Nobody can see who you've
invited" is true. "Nobody can see that you invite people" is not, and this plan does not claim it.

**User stories:**

- *Two friends, neither pays.* Sam texts Riley an invite; Riley calls; their networks holepunch; two
  hours of direct, encrypted conversation that cost us kilobytes. This is most calls — the free tier
  is genuinely useful, not a crippled teaser.
- *Same two, hotel wifi blocks the direct path.* The relay starts carrying real audio, the budget
  spends in seconds, the call drops cleanly: "this call needs a membership." A price tag, not a
  mystery failure. Either of them joining fixes it.
- *Jo is a member.* Everyone Jo talks to gets carried calls when carrying is needed, in either
  direction. One membership fixes calling for a whole family.
- *Alex is being harassed.* Alex deletes one record from their own repo; the harasser's invite dies
  within minutes without Alex asking us for anything — and nothing anywhere ever recorded that the
  harasser had been invited.
- *A stranger finds your page.* "Ask them for an invite," or "not callable" — the latter byte-identical
  to "no such handle," so the page cannot be used to probe who exists.
- *A phone and a desk.* Each device is its own record; each invite reaches the devices you chose at
  issuance. The plumber's invite reaches `work`; your partner's reaches everything. A caller can
  prefer a device, never widen the grant.

**End state.** Our own relay binary (stock `iroh-relay` wrapped, never forked) serving
`relay.croft.ing` with counting and budget enforcement; `croft-relay-admit` minting tokens, backed by
a private CISS instance; the exchange page and Android client on the per-device lexicon; invites
issued, redeemed, and revoked end to end. We store membership (peppered digests) and per-member byte
totals. Who calls whom, when, and what they say exists **nowhere** — not as policy, but as absence.

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
know which token was presented, therefore which member. On close we emit
`(member, bytes_in, bytes_out, duration)`.

**(Phase 3 correction, 2026-08-10.)** The original mechanism here — "wrap the stream in an
`AsyncRead`/`AsyncWrite` struct before handing it to iroh's service" — turned out to be
**impossible**: after the websocket upgrade the relay downcasts its IO to exactly
`TokioIo<MaybeTlsStream>`, whose variants are concrete `TcpStream` types, so any interposed type
breaks every relay connection at runtime. The built shape is the **loopback airlock**:
`CountingStream` wraps the *public* connection (where every byte of the connection's lifetime passes,
upgrade included) and a pump splices it over a loopback TCP hop to an internal listener whose
connections are genuine `TcpStream`s — upstream unmodified, downcast intact. The section's argument
survives unchanged: the counter still sits in our own process at the layer where identity is known,
which no nftables/eBPF/second-process design can reach. Full detail: the Review Log's Phase 3
course-correction and `croft-relay-bin/src/counting.rs`.

It counts framing as well as payload, so it is an **upper bound**. That is fine for a budget and for
capacity planning, and it must never be described as billing.

### 3.5 Why caps live in the callee's own repo

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

**CISS (Pass 3 — read at the owner's direction, 2026-08-09)**

| Assumption | Source |
|---|---|
| **The self-assertion substrate is the "one mechanism for every customer-signed setting":** `kind` + optional `subkey` + strictly-monotonic `seq`, domain-separated preimage, pure verify choke point | `CISS/src/assertion.rs:1-30` |
| Two authorization models: **Model A** (`OwnerSigned`, an `id:` owner signs, key must derive the DID) and **Model C** (`ProviderAttested`, a `did:` owner authorizes the *action* by service-auth JWT and CISS counter-signs with a dedicated attestation key). Model C records the authorizing `jti` | `.../assertion.rs:19-27`, `:57-67` |
| Existing kinds: `policy`, `dial/ceiling`, `dial/account-mode`, `dial/period`, `dial/receipt-mode` | `src/policy.rs:27`; `src/dials.rs`; `src/server.rs:51-54` |
| **`ledger.rs` is a generic append-only, hash-linked, signed chain — one per actor, entries carry `kind`/`body`, `seq`/`prev_hash`, one or more signatures; `verify_entries` recomputes hashes, linkage, and signatures** | `src/ledger.rs:1-11` |
| `statements.rs` closes periods into co-signed, hash-chained balance-forward statements with a byte-day rent integrator — an edit to a historical figure breaks the chain at that link | `src/statements.rs:1-8` |
| **Persistence underneath is SQLite** (`Db::Memory` / `Db::File`), holding per-DID metering records | `src/server.rs:153-160` |
| **`Did` distinguishes two disjoint identity spaces at the type level** — `id:` (CISS-native, hash of a presented key, no resolution) and `did:*` (atproto, resolved to a signing key) — with a `WrongSpace` error stopping either plane accepting the other | `src/identifiers.rs:46-58`, `:100-110` |
| CISS is already deployed by `croft-stack` (the `tenants` role) | `croft-stack/ansible/roles/tenants/` |

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

- **Holepunch failure rate ≈ 10–15%** — owner-supplied (2026-08-09: "ten to fifteen, maybe twenty…
  we think it's around ten to twelve"), recalled from prevailing figures, not measured by us. Accepted
  as a working assumption, and recorded as a **range** rather than a point because that is how it was
  given. Our own users' NAT distribution may differ. Note the budget mechanism is insensitive to this
  number — it decides how *often* the budget path is exercised, never whether it is correct.
- ~~**Whether `Endpoint::insert_relay` with a changed auth token forces a relay reconnect.**~~
  **SETTLED BY PROBE (2026-08-10): NO RECONNECT.** The established connection persists with the old
  token; the swapped config is read only at the next connect. Empirical (in-process relay, admissions
  counted through our `TokenAccess`; `evidence/insert-relay-probe.txt` holds output + probe source)
  and consistent with source (`iroh-1.0.3 socket.rs:1236-1247` sends `RelayMapChange`, whose handler
  only re_stuns — `:1763-1782`). **Consequence:** a sponsored upgrade starts a fresh budget via
  **disconnect-to-upgrade** — drop the endpoint with the lever Phase 4 already builds; the client's
  relay actor auto-reconnects reading the new token from the swapped map. No live-connection budget
  re-reader needed.
- **Introduction byte cost across real NATs.** Phase 0.
- **(Pass 2) `com.atproto.repo.listRecords`** — the corpus FACTCHECK docs do not cover it, and
  `connect/docs/contract.md` evidences only `resolveHandle` and `getRecord` (which are working,
  deployed code). `listRecords` is this plan's addition and is asserted from general knowledge. Confirm
  against the atproto lexicon before Phase 10 builds the contract on it.
- ~~**(Pass 2) That post-upgrade bytes traverse our `CountingStream`.**~~ **REFUTED at source,
  Phase 3 start (2026-08-10) — and worse than "does not hold": the in-line decorator cannot exist at
  all.** After the websocket upgrade the relay **downcasts the IO to exactly
  `TokioIo<MaybeTlsStream>`** (`http_server.rs:95-96`; failure is a runtime error per `:321`), and
  `MaybeTlsStream` is `#[non_exhaustive]` with concrete variants — `Plain(TcpStream)`,
  `Tls(TlsStream<TcpStream>)` (`streams.rs:194-202`). Any interposed type, above or below, breaks
  every relay connection. Phase 3's design was revised to the loopback airlock (see the phase and the
  Review Log); the assumption this row replaced is now trivially true in the revised shape, because
  the airlock pump carries every byte of the connection's lifetime, upgrade included.

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
- **`CroftCommunity/connect` `docs/contract.md`** — lexicon moves to per-device records; `getRecord` → `listRecords`; adds the grant and policy records. Cross-repo. Phase 10.
- ~~`.claude/CI-PATTERN.md` — grepped: no change needed; `croft-relay` is already in the smoke
  matrix.~~ **(Pass 3: retired by the relocation.)** Superseded by the two CI items below.
- **(Pass 3)** `discovery/.github/workflows/smoke.yml` — **remove** the `croft-relay` matrix entry; the
  directory no longer exists there after the Phase 1 move. Phase 1.
- **(Pass 3)** `croft-stack/.github/workflows/` — **add a gate workflow** (build + test on
  `pull_request` and on push to `main`). croft-stack today has only `deploy-service.yml`, a
  `workflow_call` reusable deploy with no `pull_request` trigger — a notification, not a gate
  (`.claude/CI-PATTERN.md`). The gate must exist **before** the crates arrive, not after. Phase 1.
- **(Pass 3)** `croft-stack/README.md` + `CONTRACT.md` — record that `relay/source/` now holds the
  service's code alongside its existing `deploy/` and `tests/`. Phase 1.
- **(Pass 3)** `experiments/croft-relay/README.md` also gains the note that `phaseN_*.rs` test names
  refer to the superseded numbering (§8.2). Phase 1.
- **(Pass 3)** `croft-relay-bin`'s config doc records the log-level knob and the two operator greps;
  `06-iroh-relay.md` records the rollback procedure. Phases 2 and 5 respectively.
- **(Pass 3)** `CroftCommunity/connect` `docs/contract.md` also carries the client-side logging
  constraint (the page logs nothing about lookups; a cap secret never reaches a log or crash report),
  because it binds two codebases and neither will infer it. Phase 10.

## 7. Concurrency Map

```
Phase 1 (relocate to croft-stack + CI gate; record + correct)
                              │   (Pass 3: no longer ∥ with P0 — P1 moves
                              │    the directory P0 writes into)
                              ▼
Phase 0 (budget sizing — owner-gated on a second network)
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

- ~~**Phase 0 ∥ Phase 1.** Disjoint write-sets.~~ **(Pass 3: DISQUALIFIED by the relocation.)** Phase 1
  now **moves `experiments/croft-relay/` to `croft-stack/relay/source/`** (§8.4), and Phase 0 writes
  into `experiments/croft-relay/{tests,evidence}/` — the directory being moved. Running them
  concurrently means Phase 0 writing evidence into a tree that is disappearing underneath it, which git
  will not warn about because both operations are individually legitimate.
  **Sequential: Phase 1's move goes first, then Phase 0 writes to the new home.** This costs nothing —
  Phase 0 is gated on an owner-supplied second network and may be delayed indefinitely, so blocking the
  move on it would be the tail wagging the dog. Phase 1's remaining work (docs, T62, the toolchain pin)
  still has no dependency on Phase 0 beyond the budget constants, which stay marked as placeholders.
  - *Re-entry (P1):* the move is a single commit whose diff is pure renames plus the new CI gate;
    `discovery` shows only `.md` edits and the smoke-matrix removal; `croft-stack` builds green on its
    new gate before anything else proceeds.
  - *Re-entry (P0, now after the move):* nothing written outside `croft-stack/relay/source/{tests,
    evidence}/`; both hosts' harness ports released.
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
- **(Pass 3) Parallel candidate, flagged not adopted: Phase 10 alongside Milestone B.** Phase 10 lives
  in `CroftCommunity/connect`, shares no write-set with this repo, and depends on Phase 8 only for the
  *verification* half. Its lexicon, page, and cap-issuance work could run concurrently with Milestone B
  by a second person. Not restructured here because it is a resourcing decision, not a technical one —
  surfaced so it is a choice rather than an oversight.
  - *Re-entry (P10, if run in parallel):* `git -C connect status` clean apart from the contract and
    client paths; no records left on the owner's test account beyond the two fixture personas; this
    repo's `git status` untouched.

## 8. Cross-cutting: test conventions, logging, and checkpoints (Pass 3)

Three things every phase below inherits, hoisted here so they are not restated twelve times.

### 8.1 Test-first is the phase order, not a phase step

Each phase's **Changes** list is written implementation-first because that is how the work reads. It is
not the order it is executed in. **The wiring test named in each phase is written and watched fail
before any of that phase's production code exists.** Where a phase also names unit-level behaviours,
those are RED-first too, in the same phase — no phase defers a test for behaviour it introduces.

Two phases are exempt and say so explicitly: **Phase 0** (Discovery Exemption — spike work) and
**Phase 1** (documentation plus one toolchain edit; its gate is the broken-ref check, not a test).

### 8.2 Test naming, and a numbering collision to avoid

The existing tree names integration tests `phase1_access.rs`, `phase1_http.rs`, `phase2_token.rs`,
`phase3_tier.rs` — numbered against the **original** phase plan, which this document renumbers.
`phase3_tier.rs` in particular tests the old Phase 3 (rate buckets); this plan's Phase 3 is the counting
decorator. Continuing the numeric convention would produce two unrelated meanings for "phase 3" in the
same tree.

**Convention from here: topic names, not phase numbers** — `budget_drop.rs`, `usage_accounting.rs`,
`persistence.rs`, `caps.rs`, `resolution.rs`. Existing files are **not renamed** (they are cited in
evidence files and run summaries); Phase 1 adds a one-line note to the experiment README recording that
the `phaseN_` prefix refers to the superseded numbering.

### 8.3 Logging: the convention does not exist yet, and this plan needs it

**Spot-checked at Pass 3: the tree has no `tracing` or `log` dependency and no logging call anywhere.**
That was fine for a library spike. It is not fine for what this plan builds — a long-running network
daemon (Phase 2) whose central product behaviour is *silently dropping someone's connection* (Phase 4),
fronted by a second daemon making network calls in an authentication path (Phases 6–7). Without
logging, the first production question — "why did this call end?" — has no answer at all.

So **Phase 2 establishes the convention** (it is the phase that introduces a daemon), and every later
phase logs at the points named in its own Changes list. Metrics still land in Phase 12; **logging does
not wait for it.** Deferring all observability to a hardening phase is the same anti-pattern as a docs
phase at the end.

The convention:

| | |
|---|---|
| Crate | `tracing` + `tracing-subscriber`, structured fields (not formatted strings) |
| `ERROR` | We are broken: store unwritable, cert unreadable, bind failed |
| `WARN` | They are broken, or a policy fired: resolver timeout, budget exhausted, spent-token refusal |
| `INFO` | Lifecycle only: startup with resolved config, bind address, shutdown |
| `DEBUG` | Per-connection admit/deny with reason; per-connection close with byte totals |
| `TRACE` | Not used — the counting decorator is in every byte's path and must never log there |

**The privacy rule, which is not negotiable and belongs in ADR-0006:** logs may carry an
`endpoint_id`, a `connection_id`, a `cap_id`, and byte counts. Logs must **never** carry a token, a
cap secret, a DID *pair*, or anything from which who-called-whom can be reconstructed. A log line
naming both parties to a call is a call-detail record — the exact artifact §3.5 exists to avoid
holding, and it would arrive through the back door of a debugging convenience. Phase 12's deny-path
work re-checks this under load.

### 8.4 The relocation, and how to read every path in this plan

**Owner's ruling (2026-08-09): `croft-relay` moves out of `discovery/alpha/experiments/` and into
`croft-stack`, under the existing `relay/` directory.**

```
  croft-stack/
    ansible/roles/relay/     ← already there; stays put
    relay/
      deploy/                ← already there: systemd units, relay.toml, Caddy site
      tests/                 ← already there: bats deploy tests
      source/                ← NEW: the Rust workspace lands here
        croft-relay-admit/   ← renamed from croft-admit in the same move (owner, 2026-08-10)
        croft-relay-embed/
        croft-relay-bin/
```

**The rename rides the move.** `croft-admit` becomes **`croft-relay-admit`** — the crate is the relay's
admission authority, and the old name reads as a general-purpose admission service it is not. Phase 1
is the free moment: the crate is changing repos anyway, no new code has been written against the old
name, and one commit later every path is final. This document's paths say `croft-admit` below for the
same reason they say `experiments/croft-relay/` — they match the tree the passes verified against.
After Phase 1, read `croft-admit` as `croft-relay-admit` throughout.

The bundle is the service plus what deploys it. A standalone repo was the other candidate; it splits
those two apart again, which is the thing that is currently working.

**Move in Phase 1, not Phase 5 or 11.** Only two crates exist today and no production artifact ships
until Phase 5, so the move is at its cheapest now and gets monotonically more expensive as Milestones B
and C add crates. Moving later would also mean every phase writes to paths that are known in advance to
be wrong.

**Histories get mixed.** Accepted by the owner as unlikely to ever matter. Worth one line in the move
commit recording where the code came from, so the mixing is legible rather than mysterious.

**How to read this document's paths.** Every Read-set and Write-set below is written against the
*current* location (`experiments/croft-relay/…`). After the Phase 1 move, read them relative to
`croft-stack/relay/source/`. They were deliberately **not** rewritten in place: the paths as written
match the tree Pass 2 verified against, and rewriting them would have made every source citation in §4
unverifiable against anything.

**Two CI consequences, and the second invalidates a Pass-2 conclusion:**

1. `discovery`'s smoke matrix currently includes `croft-relay` (`smoke.yml:41`). After the move that
   entry is stale and must be removed, or CI builds a directory that no longer exists.
2. **`croft-stack` has no gate workflow** — its only workflow is `deploy-service.yml`, a
   `workflow_call` reusable deploy with no `pull_request` trigger. Per `.claude/CI-PATTERN.md` that is a
   notification, not a gate. Moving Rust source there **requires adding a gate** (build + test on
   `pull_request` and on push to `main`) before the code arrives, or these crates land somewhere with
   no PR checks at all. §6's earlier "CI-PATTERN: no change needed" was correct only while the code
   stayed in `discovery`; the move retires it.

### 8.5 Checkpoints: which phase broke?

Each phase's **Verification** command is its checkpoint, and they are cumulative — every phase re-runs
the suite that precedes it. Three are the load-bearing ones, in the sense that if the system is healthy
at these three points the phases between them are almost certainly fine:

```
  after P2   a stock iroh client relays a datagram through our binary
             → the embedding seam is real, not just reachable in source

  after P4   a non-member is dropped and refused re-admission; a member is not
             → the product mechanism works end to end

  after P8   a cap mints a token; deleting the grant record stops it
             → the authorization core works end to end
```

A failure after P5 or P11 that cannot be localized is diagnosed by re-running the checkpoint above it.

---

## Milestone A — Ground truth and the record

### Phase 0: Discovery — size the introduction budget

**Goal:** Turn the budget from a guess into a measurement. Cheaper than the old calibration because
budget-and-drop only has to separate kilobytes from kilobytes-per-second.

**Discovery Exemption applies** — spike work, not TDD. **Dispositions, per task:** the two-endpoint
harness is `keep-as-fixture` (Phase 4's forced-holepunch-failure test needs exactly this rig, and
rebuilding it there would be waste); the byte-counting instrumentation is `promote` — **Phase 3
re-implements it test-first as `CountingStream`**, and the Phase 0 version is a measuring tape, not a
draft of the production type; all scripting and one-off analysis is `throwaway`. No Phase 0 code reaches
`crates/` without going through Phase 3's RED-first cycle.

**(Pass 3) Could any of this be resolved during planning instead?** No — every task requires two hosts
on genuinely separate networks, which is the owner-supplied resource this phase is gated on. Nothing
here is answerable from source.

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
- [x] Write ADR-0006: the two gates, budget-and-drop, our-binary-not-a-fork, the decorator, caps as
      opaque-id records, the PDS trust ruling and its blast radius, out-of-band cap distribution, and
      what we store (membership and accounting only). **(Pass 3)** It also carries §8.3's **log privacy
      rule** — never a token, a cap secret, or a DID pair — because that rule is a design constraint
      with the same standing as content-blindness, and a convention that lives only in a plan is a
      convention that dies with the plan. **(Pass 3)** Same for the observability posture settled in
      Phase 12: aggregates-only metrics, identity confined to logs, ≥30-day retention on both, and *why*
      each — an ADR is what a future 2am debugging decision argues with.
- [x] Supersede ADR-0004's mechanism section; **retain** its content-blindness argument.
- [x] Mark ADR-0001's fork-vs-embed conclusion superseded-in-part.
- [x] OPEN-QUESTIONS: resolve Q1/Q3/Q4, leave Q2/Q5 with current status.
- [x] Fix the stale build-plan pointer in the experiment README, **and (Pass 3) add the one-line note
      that the existing `phaseN_*.rs` test names refer to the superseded numbering** (§8.2) — without
      it, "phase3_tier.rs" reads as this plan's Phase 3, which is the counting decorator.
- [x] Rewrite T62's gates.
- [x] Backlog §6j, MASTER-INDEX, COHESION, RAW-ARTIFACTS-MANIFEST.
- [x] **(Pass 3 — owner's ruling) Relocate the workspace to `croft-stack/relay/source/`** (§8.4), and
      do it **first in this phase**, so every later phase writes to its final home. Order matters:
      **add croft-stack's gate workflow before moving the code**, then move, then remove the
      `croft-relay` entry from `discovery`'s smoke matrix — never the reverse, or there is a window
      where the crates exist in a repo with no PR checks.
- [x] **(Owner, 2026-08-10) Rename `croft-admit` → `croft-relay-admit` in the same move** (§8.4):
      directory, crate name, and every internal reference. One commit with the move, before any new
      code exists against the old name. The gate proving the renamed workspace builds is the
      verification.
- [x] Add `rust-toolchain.toml` pinning `1.94.1` to match CI — the experiment has none, which is the
      green-locally/red-in-CI failure `.claude/CI-PATTERN.md` names. **(Pass 3)** After the move this
      pin must agree with croft-stack's new gate, not `discovery`'s smoke workflow.
- [x] Fix the stale MSRV comment in the workspace `Cargo.toml` (it claims `iroh-relay 1.0.0-rc.1`;
      it has been 1.0.3 since ADR-0005).

**Call chain:** n/a (docs plus one toolchain/manifest edit).
**Wiring test:** the repo's `build + broken-ref` gate passes on changed references. **Note it is
currently red on `main`** for unrelated `thinking-app-*` drift; check the failing paths before reading
red as this phase's failure.
**Depends on:** Phase 0 for constants only; structure lands first with marked placeholders.
**Read-set:** the ADR tree, `OPEN-THREADS.md`, `EXPERIMENT-BACKLOG.md`, `COHESION.md`.
**Write-set:** the relocated tree's `docs/adr/*`, `{README,OPEN-QUESTIONS}.md`, `Cargo.toml`,
`rust-toolchain.toml` (new); `beta/OPEN-THREADS.md`; `alpha/experiments/EXPERIMENT-BACKLOG.md`;
`alpha/COHESION.md`; `alpha/seeds/transcripts/RAW-ARTIFACTS-MANIFEST.md`. **(Pass 3) Plus, in
`croft-stack`:** the new gate workflow, `README.md`, `CONTRACT.md`, and the moved tree itself. **Plus,
in `discovery`:** the `croft-relay` removal from `.github/workflows/smoke.yml`. **This phase now spans
two repos and therefore two PRs** — croft-stack's lands first (gate, then code), discovery's second.
**Shared-state contract:** No mutable state beyond the write-set, except that the toolchain pin is
ambient for every later phase — re-run the existing suite once after adding it.
**Risks:** An ADR that records the decision without the rejected alternatives is not re-derivable
later, which is the whole point of writing it.
**Done when:**
1. **Behavioral:** A reader who has not seen the dialogue can reconstruct why throttling was replaced,
   why there is no fork and no switch, and what the PDS trust ruling does and does not assume.
2. **Verification:** `grep -rn "wrap the admitted connection" docs/` returns nothing; broken-ref gate
   clean on changed files; **(Pass 3)** croft-stack's new gate is green on the moved tree, and
   `discovery`'s smoke run no longer references `croft-relay`.
**Validation:** **(Pass 3) Narrow → Moderate.** It was a docs phase when Pass 2 rated it; it now
performs a cross-repo relocation and stands up a CI gate that did not exist. The docs half is still
narrow; the move half is the kind of change that is either completely right or obviously broken, and
the verification above is what tells them apart.

---

## Milestone B — The relay binary

### Phase 2: Our binary, hosting the relay

**Goal:** Replace the stock binary with our own program, which depends on the unmodified crate. Feature
parity first, no new behaviour.

**Changes:**
- [x] New crate `crates/croft-relay-bin`: accept loop, TLS, HTTP, dispatch to one
      `RelayServiceWithNotify` via `serve_connection(...).with_upgrades()` — the embedding pattern
      upstream documents.
- [x] Wire `TokenAccess` (existing, from `croft-relay-embed`) as the `AccessControl`.
- [ ] TOML config: bind addrs, cert paths, verification pubkey. Every knob documented with its why.
- [ ] Preserve upstream's probe path so existing health checks keep working.
- [x] **(Pass 3) Establish the logging convention** (§8.3): add `tracing` + `tracing-subscriber` — the
      first logging dependency in the tree — with `RUST_LOG`-style filtering. In this phase that means
      `INFO` on startup (bind address, cert path, **pubkey fingerprint not the key**), `ERROR` on the
      three fatal starts (bind failed, cert unreadable, config invalid), and `DEBUG` per connection
      carrying `endpoint_id` + `connection_id` + admit/deny with reason. **Deny reasons must be logged
      even though they are not returned to the client** — a relay that refuses strangers with an opaque
      error is one whose operators cannot tell a misconfigured client from an attack.
- [x] **(Pass 3) Fail loud on config:** an unparseable or incomplete config aborts at startup with the
      offending field named. No defaults substituted for a missing verification pubkey — a relay that
      silently starts unable to verify anything would deny every connection and look like a network
      fault.

**Call chain:** `main()` → config → accept → TLS → parse request → `RelayServiceWithNotify` →
`/relay` upgrade → `TokenAccess::on_connect` → relay carries traffic.
**Wiring test:** A **real relay client** (not a unit stub) connects to the binary started from a config
file, is admitted on a valid token, and relays a datagram A→B. RED before the binary exists.
**(Pass 3) Named behaviours, all RED-first, all in this phase:** valid token → admitted and datagram
delivered; **absent** token → denied; **malformed** token → denied; token valid but signed by a
different key → denied. The deny cases are three distinct paths through `TokenAccess::decide` and a
single happy-path test leaves all three unexercised through the binary. Plus: a config missing the
verification pubkey aborts with that field named, rather than starting.
**Depends on:** Phase 1.
**Read-set:** `crates/croft-relay-embed/src/lib.rs`, `crates/croft-admit/src/{token,tier}.rs`.
**Write-set:** `crates/croft-relay-bin/*`, workspace root `Cargo.toml` (new member + `tracing` deps).
**Shared-state contract:** **(Pass 3 — restated as invariants.)** Binds only ports obtained from `:0`;
never a fixed port. Writes no file outside its write-set and the test's own tmpdir. Performs no git
operation. Leaves no process running after the test binary exits — every spawned relay is owned by a
guard that kills it on drop.
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
- [x] `CountingStream<S>`: implements `AsyncRead`/`AsyncWrite`, increments byte counters, wraps the
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
- [x] **Verify in-phase, do not assume:** that post-upgrade traffic still flows through `CountingStream`.
      The Service takes the upgrade from the request (`handle_relay_ws_upgrade`), and hyper hands the
      original IO to the upgraded task, so it should — but the entire counting design rests on it, so
      assert it with a test that pushes a known payload *after* the websocket is established.
- [x] The authoritative verification stays in `on_connect`; the wrapper's token read is for attribution
      only and must never be treated as authorization.
- [x] On close, emit `(subject, bytes_in, bytes_out, duration)`.
- [x] Document that the count includes framing — an **upper bound**, suitable for budgets and capacity,
      never described as billing.
- [x] **(Pass 3) Logging, at exactly one point:** `DEBUG` on close, carrying `endpoint_id`,
      `connection_id`, byte totals, duration. **Nothing is logged inside `CountingStream` itself** —
      it sits in every byte's path, so a log call there is a performance bug and, at `TRACE`, a
      content-adjacent one. The usage record *is* the observability for this phase; the log line is its
      human-readable shadow. Per §8.3 the record names one endpoint, never a pair.
- [x] **(Pass 3) Diagnose the join failing, don't just fail it.** The three-way token join is the
      fragile part of this design. If a connection closes with a counter that was never associated with
      a token, emit `WARN` with the connection id — an unattributed connection is either a bug in the
      join or a client that never authenticated, and Phase 4 will silently fail to enforce against it.
      Assert this path: a connection that closes before presenting a token produces exactly this warning
      and no usage record, rather than a record attributed to nobody.

**Call chain:** accept → TLS → parse request (token known) → `CountingStream::wrap` →
`RelayServiceWithNotify` → … → close → usage record.
**Wiring test:** Two clients push known payload sizes through the running binary; the emitted records
attribute the right order-of-magnitude byte counts to the right subjects. Assert attribution and
ordering, not exact byte equality — framing overhead makes exactness a false precision.
**(Pass 3) Named behaviours, RED-first:** (a) a connection that pushes 10× another's produces a record
with 10× the bytes, ±framing — a *ratio* assertion, which is mutation-resistant where an absolute
threshold is not, since it fails if the counter is wired to the wrong stream, counts one direction, or
double-counts; (b) two concurrent connections produce two records with **no cross-attribution** — one
client's bytes never appear under the other's subject, which is the failure the shared-`Arc` join makes
possible; (c) bytes pushed **after** the websocket upgrade appear in the record (the §4 unverified
assumption — this is the test that settles it); (d) a connection closing without a token yields a `WARN`
and no record.
**Depends on:** Phase 2.
**Read-set:** `crates/croft-relay-bin/src/main.rs`.
**Write-set:** `crates/croft-relay-bin/src/{counting,usage}.rs`, `src/main.rs`.
**Shared-state contract:** **(Pass 3 — invariants.)** In-process counters only; writes no file and
opens no store. Binds only `:0` ports. Emits usage records to an in-process sink the test can read;
does not append to any path on disk in this phase.
**Risks:** The decorator sits in the hot path of every byte — keep it to counter increments and no
allocation. Second risk: the pre-verification token read becoming load-bearing for anything but
attribution.
**Done when:**
1. **Behavioral:** Every closed connection yields a usage record naming its subject.
2. **Verification:** `cargo test -p croft-relay-bin --test usage_accounting` — starts the binary and
   drives real clients through it; no test in this file may construct `CountingStream` directly, or the
   suite would prove the type works while the binary never calls it.
**Validation:** **(Pass 3) Moderate → Broad.** Pass 2 rated this Moderate as a measurement-only phase.
That undersells it: this phase is where the plan's single load-bearing unverified assumption gets
settled (post-upgrade bytes traversing our wrapper), and everything Phase 4 enforces rests on these
numbers being attributed correctly. If the join is subtly wrong, Phase 4 enforces against the wrong
person and the tests there still pass.

### Phase 4: Budget enforcement and the clean drop

**Goal:** The product mechanism. A connection whose budget is spent is disconnected, cleanly, and its
token is not re-admitted.

**Changes:**
- [x] `Tier` → **budget** rather than rate bucket: rewrite `tier.rs`'s `bucket_for` as `budget_for`,
      returning `Budget::Unlimited` (member-involved) or `Budget::Bytes(n)` (introduction only). Rewrite
      the `SPEC-DELTA` comment around Phase 0's sizing.
- [x] Supervisor: when a counted connection exceeds its budget, call
      `RelayService::clients().disconnect(endpoint_id, connection_id)`. **(Pass 2:** passing `None` for
      the connection id drops *every* connection for that endpoint, which is simpler and is what we
      want — verified at `clients.rs:172-195`. `ConnectionId` is public if we ever need precision.**)**
- [x] **(Pass 2) Treat the budget as a threshold, not a hard cap.** `disconnect` is documented as
      asynchronous — "each per-connection actor exits its run loop and unregisters itself after this
      call returns" — so bytes can still flow between the decision and the close. Overshoot is expected
      and acceptable; **the test must not assert an exact byte ceiling**, or it will be flaky for a
      reason that isn't a bug.
- [x] **Spent-token refusal:** record the spent token id; `TokenAccess::on_connect` denies re-admission
      on it. Without this, iroh's automatic reconnect-with-backoff turns one drop into a flap.
- [x] No `client_rx` rate limit configured on the service at all — the budget replaces it. Keep a
      coarse global limit only as an anti-hammering backstop, documented as such.
- [x] **(Pass 3) Log every drop at `WARN`, with the reason distinguishable from a fault.** A budget
      disconnect and a spent-token refusal are the two lines an operator will search for first, and they
      are the only evidence that the product mechanism fired at all. Fields: `endpoint_id`,
      `connection_id`, `bytes_counted`, `budget`, and a stable `reason` discriminant
      (`budget_exhausted` / `spent_token`). Per §8.3, one endpoint per line — never the pair, even
      though the supervisor could name both and it would be convenient during debugging. That
      convenience is how a call-detail record gets built by accident.
- [x] **(Pass 3) Log the near-miss too, at `DEBUG`:** a connection that closed having used most of its
      budget without exceeding it. When Phase 0's number turns out to be wrong, this is the line that
      shows it — and a budget that is silently near-missed by every real introduction is
      indistinguishable, in logs, from one that is comfortably right.
- [x] **Verify first:** does `Endpoint::insert_relay` with a changed auth token force a reconnect? The
      budget boundary depends on the answer. If it reconnects, a sponsored upgrade naturally starts a
      fresh budget; if it does not, the supervisor must re-read the budget on an existing connection.
      Settle this with a probe **before** implementing, and record the result in Verified Assumptions.

**Call chain:** usage record crosses budget → supervisor → `Clients::disconnect` → client's
`ActiveRelayActor` retries → `on_connect` denies the spent token → client surfaces the limitation.
**Wiring test:** A non-member pair whose holepunch is forced to fail pushes past its budget and is
**disconnected**, and its immediate reconnect is **refused**; a member-involved pair on the same binary
pushes the same volume and is not disconnected. This single test is the product.

**(Pass 3) Boundary cases, named — this is the phase where a single-point assertion would be worthless.**
`budget_for` is pure branching and the supervisor is a threshold comparison; both are exactly the shape
a one-line mutation survives. Assert, RED-first:

| Input | Expected |
|---|---|
| neither party a member | `Budget::Bytes(n)` |
| caller is a member | `Budget::Unlimited` |
| callee is a member | `Budget::Unlimited` |
| both are members | `Budget::Unlimited` |
| bytes well **under** budget | not disconnected |
| bytes **at** the budget | not disconnected (the budget is what you may spend, not the point of refusal) |
| bytes **over** budget | disconnected, within a generous tolerance — **never an exact ceiling** |
| a second connection on a spent token | refused at `on_connect` |
| a fresh token after a drop | admitted (the refusal is per-token, not a ban) |

The three member cases matter individually: "either is a member" is an `||`, and a test with only the
neither/both rows passes against `&&`. That mutation would ship a product that silently requires *both*
parties to pay.

**(Pass 3) One negative-space assertion, restated from Risks because it is easy to leave unwritten:** a
pair whose holepunch **succeeds** never has its budget consumed. Without it, the self-cleaning property
in §3.1 is an argument rather than a tested behaviour, and a regression there is invisible — the call
works, we just start charging budget against calls we never carried.
**Depends on:** Phase 3 and Phase 0 (for the number; a placeholder is acceptable to build against but
not to deploy).
**Read-set:** `crates/croft-admit/src/{tier,token}.rs`, `crates/croft-relay-bin/src/usage.rs`.
**Write-set:** `crates/croft-relay-bin/src/{supervisor,main}.rs`,
`crates/croft-admit/src/tier.rs`, `experiments/croft-relay/DESIGN.md`.
**Shared-state contract:** **(Pass 3 — invariants.)** The spent-token set is process-local and lost on
restart — acceptable (tokens are short-lived) but state it, because "revocation survives restart" is a
claim someone will otherwise assume. The supervisor writes nothing to disk, opens no store, and calls
`disconnect` only for endpoints whose usage records it has itself observed — it never enumerates
`clients()` and acts on connections it has no record for.
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
- [ ] **(Pass 3) Ship the logging as an operable thing, not just a compiled-in one:** log level set in
      the systemd unit (default `INFO`, `DEBUG` togglable without a rebuild), stdout to the journal so
      `journalctl -u` is the whole story, and **no log file inside the netns** to grow unbounded. Record
      in `06-iroh-relay.md` the two greps an operator runs first: `reason=budget_exhausted` and
      `reason=spent_token`.
- [ ] **(Pass 3 — owner, 2026-08-09) Turn on automated dependency PRs**, targeting croft-stack's gate:
      `iroh-relay` (ours to track from the moment we build the binary) and the git-pinned `ciss-auth`.
      This is the phase that takes on the release-cadence obligation, so it is the phase that arranges
      for the obligation to be met by a mechanism rather than by remembering. Record in the ADR that
      owning the cadence is a standing cost, not a one-time step.
- [ ] **(Pass 3) Write the rollback down before it is needed.** The stock-binary + HTTP-hook fallback is
      already named as the fallback; state the *procedure* — which Ansible variable reverts, how long it
      takes, and what capability is lost while reverted (budgets and accounting; admission still works).
      A fallback nobody has written the steps for is a hope.

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
- [ ] `[[bin]]` target: bind, serve the router, config for keys and the store backend (post-relocation:
      the private CISS instance's localhost URL and credentials — see below).
- [ ] Durable store behind a narrow interface, holding **only**: membership, and accounting/quota
      counters. Explicitly **not**: allowlists, call policies, or any pair.
- [ ] **(Pass 3 — owner's direction, 2026-08-09) The store is a CISS instance of our own, reached over
      localhost.** Not the customer-facing deployment: **our own private instance, on our host, exposed
      to nothing but this process.**

      ```
        our host
        ┌──────────────────────────────────────────┐
        │  croft-admit                             │
        │      │  HTTP over 127.0.0.1              │
        │      ▼                                   │
        │  CISS instance (ours, private)           │
        │      └─ SQLite + chained signed records  │
        └──────────────────────────────────────────┘
      ```

      **The reason is one persistence story, not two.** Backup, restore, failover, upgrade, and the
      operational wisdom around all of it accrue to a single mechanism we are already committed to
      running, instead of croft-admit inventing a second one that has to grow its own answers to the
      same questions. That argument stands on its own; the feature overlap below is corroboration, not
      the case.

      Corroborating findings from source: `ledger.rs` is a generic append-only, hash-linked, signed
      chain (one per actor, `kind`/`body`, `verify_entries` recomputing rather than trusting) — the
      "chain kind just to this end"; persistence underneath is **already SQLite**, so "SQLite vs CISS"
      was a false choice; and `Did` separates `id:` from `did:*` at the type level with a `WrongSpace`
      error, so keying on atproto DIDs **does not extend the `id:` identity outside its scope** (§2's
      standing constraint).

- [ ] **(Pass 3) Scoping note, so later readers do not re-derive the concerns Pass 3 raised and
      withdrew.** Three worries were raised against a *shared, customer-facing* CISS and do **not**
      apply to a private instance: that membership must live on the provider-signed side because a
      customer could otherwise assert their own membership (nobody else can reach this instance); that
      CISS in the call-setup path is an availability coupling needing caching and buffering (it is a
      localhost call to a process on the same host — if it is down, we are down); and that CISS's HA
      design must be located before depending on it (the point is that whatever it becomes, it becomes
      it once, for both). **If a shared instance is ever proposed, all three return.** One line in
      ADR-0006 to that effect; not a Phase 6 gate.
- [ ] Keep the in-memory implementation for tests, and keep the narrow interface **even though the
      engine is now decided** — its job was never engine-portability, it was to make "add an allowlist"
      awkward. CISS being a general store makes that discipline *more* necessary, not less: the engine
      will happily hold a graph if we ask it to.
- [ ] Fail loudly if the store is unreachable or refuses writes at startup — never silently fall back
      to in-memory. **(Free-form pass, 2026-08-10: reworded from "store path unwritable" — with CISS as
      the backend, the failure is a localhost connection or auth refusal, not a filesystem error. The
      principle is unchanged: a croft-admit that starts without durable storage is lying about every
      acknowledgment it sends.)**
- [ ] **(Free-form pass) The persistence test needs a CISS binary, which is a cross-repo test
      dependency.** The wiring test below restarts the store, so it must run a real CISS process against
      a tmp SQLite file — the in-memory store impl proves nothing about survival. Source it by the
      owner's standing dependency rule (ours → git pin): a pinned CISS release artifact or a build from
      the pinned commit, fetched in CI the same way `ciss-auth` is. Settle the mechanics at phase start;
      what is not acceptable is the test silently downgrading to the in-memory impl when no CISS binary
      is present — absent binary, the test **fails**, not skips.
- [ ] **(Pass 3) Logging:** `INFO` on startup naming the resolved store backend (the CISS URL, never
      its credentials) and the `IdIndex` mode in force — **the mode especially**, because `Transparent`
      in production is the failure this seam exists to prevent and it is otherwise indistinguishable
      from `Keyed` at runtime. `ERROR` on store-unreachable with the URL and the error class. `DEBUG`
      per admission decision with its outcome. Never log a stored identifier's pre-image in `Keyed`
      mode — logging the DID next to its digest defeats the seam entirely, and it is exactly what
      someone debugging a lookup mismatch will reach for.
- [ ] `IdIndex` seam over stored identifiers: `Transparent` (dev/test) and `Keyed`, both behaviourally
      tested, with the migration written now while the store is small.
      **(Pass 2 — the rationale was stale.)** The Pass-1 form was `HMAC(k, member ‖ counterpart)`,
      justified by stopping one person recurring as the same digest across many members' lists. **The
      rewrite deleted all the lists** — storage is membership and accounting, both keyed by member
      alone. So the key is `HMAC(k, member_did)`, and the surviving value is narrower but real: a leaked
      accounting store does not reveal our **member roster**, which is a customer list. If that is not
      worth a seam, the honest move is to drop `IdIndex` rather than keep it for a reason that no longer
      applies.
- [ ] **(Pass 3 — owner's ruling, 2026-08-09) `IdIndex` is kept, and key custody becomes its own dial.**
      The seam survives not because the old rationale was rescued but because the property it protects
      is worth having *once the key is treated as a credential rather than a config value*. A DID is
      such a strong identity proxy that the key guarding its digest deserves the handling a password
      gets. So: **settle a simple default now, and preserve the shape of the harder options while that
      is still cheap.**

      | Rung | Where the key lives | When |
      |---|---|---|
      | L0 `Transparent` | no key | dev and test only; never a production default |
      | **L1 `Keyed`, operator-supplied at startup** | a file or env var **outside the store's backup path** | **the default this plan ships** |
      | L2 `Keyed`, fetched at startup | a remote the host authenticates to; never at rest locally | later, if warranted |
      | L3 `Keyed`, external custody | KMS/HSM, key never in our process | later, if warranted |

      **L1 is the whole point and the one constraint that must not be fudged:** the key living outside
      the backup path is precisely what buys the leaked-backup property. A key sitting next to the store
      it protects is ceremony, and it is the default someone will reach for unless the plan says
      otherwise.

      **Preserving the shape cheaply** means one thing concretely: the key arrives through a narrow
      `KeySource` seam, so L2 and L3 are new implementations rather than a refactor of everything that
      touches the store. That is a small interface written once, not speculative machinery — it is the
      "cheap but clear" line, and anything beyond it (rotation orchestration, envelope encryption,
      multi-key eras) waits until a rung above L1 is actually chosen.
- [ ] **(Pass 3) What the hashing actually buys, stated without inflation.** We store the digest and
      compare by digest: the caller's DID arrives in hand at call time, so the pre-image never needs to
      be stored. Two properties follow, and the second is the one that is easy to get wrong.
      - **The salt does nothing; the pepper does all the work.** atproto DIDs are enumerable from the
        firehose, so a per-record salt only makes each guess cost one hash instead of zero — it does not
        stop enumeration. What stops it is a **secret** mixed into the digest, held outside the database
        it protects. That secret is exactly the L1 key above; there are not two mechanisms here, there
        is one, and it is handled like a password.
      - **Exact-match is the only query left.** No prefix scan, no readable roster, and every admin or
        debugging surface must go through the same digest. Free for "is this caller a member"; a real
        cost only if we later want a question we can no longer ask.
      - Rejected in passing (owner): keeping the map only in memory. It narrows exposure to the process
        image and costs a restart story — not worth it at this stage.
- [ ] **The ADR states what is and is not claimed at each rung.** At L1 the property is "a leaked store
      or backup does not reveal the member roster" — **not** "the roster is protected from someone with
      the host." Without that sentence, "we hash the DIDs" gets read a year later as the stronger claim
      by someone who does not know which rung is in force. In keyed mode an absent key is a hard error,
      never a silent fall back to `Transparent`.

**Call chain:** `main()` → config → store connect (CISS over localhost) → router → bind → serve.
**Wiring test:** Start a CISS instance against a tmp SQLite file, start the `croft-admit` binary
against it, record a membership over HTTP, **restart both processes**, and confirm it survives.
Restart is the assertion — an in-memory store passes every test that does not restart, and restarting
only croft-admit would prove CISS's durability rather than our wiring to it.
**(Pass 3) Named behaviours, RED-first:** membership survives restart; an accounting counter survives
restart **with its value**, not merely its existence; an unreachable store aborts startup rather
than degrading; `Keyed` mode with no key present is a hard error at startup, not a fallback to
`Transparent`; a value written under `Transparent` and migrated is retrievable under `Keyed` (the
migration is the part that is written once and run once, which is precisely when it is never tested);
lookups by an identifier that was never stored return absent rather than erroring.
**Depends on:** Phase 1.
**Read-set:** `crates/croft-admit/src/{registry,access,http_api,enroll}.rs`.
**Write-set:** `crates/croft-admit/src/{store,id_index,main}.rs`, `Cargo.toml`.
**Shared-state contract:** **(Pass 3 — invariants; free-form pass — restated for CISS.)** Binds only
`:0` ports. Talks to a store only at the URL given by config — never a discovered or default one — and
in tests that URL is always `127.0.0.1`. The test's CISS instance uses a tmp SQLite path and is owned
by a guard that kills it on drop. Reads exactly one environment variable (the `IdIndex` key) and
refuses to start in `Keyed` mode without it — it never falls back. Tests leave no file outside their
tmpdir and no process running after the test binary exits.
**Risks:** Scope creep back into holding a graph. The store's interface should make "add an allowlist"
awkward rather than easy — and CISS being a general store makes the discipline more necessary, not
less. Second (free-form pass): the CISS-binary test dependency is a build-order coupling CI must
actually satisfy; a skipped-not-failed persistence test would hollow this phase out silently.
**Done when:**
1. **Behavioral:** Membership and accounting survive restart; nothing about relationships is stored.
2. **Verification:** `cargo test -p croft-admit --test persistence` (starts the **binary**, not the
   router in-process — restart is only meaningful against a process) and `--test id_index` (both impls
   plus migration).
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
- [ ] **(Pass 3 — owner, 2026-08-09) Two ages, not one TTL.** A single number forces a choice between
      noticing a key rotation and surviving someone else's outage. Two do not:

      | Dial | Starting value | Behaviour past it |
      |---|---|---|
      | **Refresh age** | ~1h | refresh in the background on next use; **keep serving the cached entry** |
      | **Hard max-stale** | 24h | refuse — fail closed, per the ruling below |

      In normal operation entries are fresh within the hour, because the background refresh succeeds.
      The 24h figure is load-bearing **only** during an upstream outage, which is the case it exists
      for. Tune either independently.

      **Why a long max-stale is defensible here specifically:** a stale key means we would accept a
      signature from a key its owner has abandoned. In this system that buys **relayed bandwidth in
      their name** — not access to any call, because calls are end-to-end and the relay is blind (§3.5).
      A bandwidth-theft window, not an eavesdropping one. Were the key guarding message content, the
      numbers should be much shorter; state this reasoning in the ADR so the values are not later copied
      into a context where the premise does not hold.
- [ ] **Cache shape, since it is what makes the above work:** process-local, **bounded** (an unbounded
      identity cache is a memory-growth vector driven by strangers), single-flight on refresh so a
      thundering herd does not multiply one miss into hundreds of upstream calls, and **failures are
      never cached** — one plc.directory timeout must not lock a user out for the refresh age, let alone
      the max-stale.
- [ ] **Fail closed** on any resolver failure — owner's ruling — with a test asserting it, because
      "fail open under load" is the classic regression. Accept and document the consequence: a
      plc.directory outage stops new call setup for `did:plc` identities.
- [ ] Keep fixtures; no test may touch the live network.
- [ ] **(Pass 3) Logging — this is the phase where a production failure is otherwise undiagnosable.**
      Five lookups can fail (handle resolution, plc.directory, `did:web` fetch, document parsing, record
      listing) and they fail identically from the caller's view: the call does not connect. Each gets a
      `WARN` naming **which** lookup, the DID or handle involved, and the failure class (timeout /
      status / parse). Cache hits and misses at `DEBUG` with the key and remaining TTL — the two
      questions asked when the resolver "works but is slow" or "serves stale keys" are both answered by
      that line and by nothing else.
- [ ] **(Pass 3) Never log a resolved key or a JWT.** The DID and the lookup outcome, yes; the material,
      no. An authentication path that logs its inputs at `DEBUG` is one `RUST_LOG` change away from a
      credential leak in the journal.

**Call chain:** call setup → resolver (cache or network) → keys/records → verification.
**Wiring test:** End-to-end against a **local fixture HTTP server** (not a trait stub) serving
plc.directory-shaped and `listRecords`-shaped responses; plus an asserted deny on resolver timeout.
**(Pass 3) Named behaviours, RED-first — the fail-closed cases are the point, and each failure mode
needs its own row because they are five different code paths that must converge on one outcome:**
handle that does not resolve → deny; DID that resolves to no PDS → deny; PDS returning `500` → deny;
PDS returning **valid JSON of the wrong shape** → deny (not a panic, and not a partial parse that
proceeds); PDS that **hangs past the timeout** → deny, and within the timeout, not whenever the socket
eventually gives up; `did:web` whose `.well-known` is a redirect to another host → deny. Plus the cache:
a lookup **inside the refresh age** makes no network call; one **past the refresh age but inside
max-stale** serves the cached entry **and** triggers a refresh (assert both halves — serving stale
without refreshing, and refusing to serve while refreshing, are different bugs that a single assertion
misses); one **past max-stale** with the upstream unreachable **refuses**; a **failed** lookup is not
cached, or one plc.directory blip locks a user out; and **N concurrent misses for the same DID produce
one upstream call**, not N.
**Depends on:** Phase 6.
**Read-set:** `crates/croft-admit/src/{pds,did,enroll}.rs`, `CISS/crates/ciss-auth/src/lib.rs`.
**Write-set:** `crates/croft-admit/src/{pds_client,did_doc,cache}.rs`, `Cargo.toml`.
**Shared-state contract:** **(Pass 3 — invariants.)** First outbound network dependency in this crate.
No test resolves a hostname outside `127.0.0.1`; the fixture server binds `:0`; **CI makes no egress
whatsoever**, and the resolver's base URLs are injected by config so a test can point them at the
fixture rather than relying on a network being absent. **Clock is injected**, so the two cache ages are
tested by advancing time, never by sleeping. Cache is process-local, bounded, and never
written to disk. The one live run against the owner's test account is **manual and recorded in
`evidence/`** — it is not part of any `cargo test` invocation.
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
      holds a grant record naming the opaque id and its device scope — **never a grantee** (free-form
      pass: aligned with the device-scope ruling below; "only the opaque id" was the pre-scope wording).
- [ ] Verification: fetch the issuer's grant records (Phase 7, cached), confirm the presented cap
      matches a live record.
- [ ] **(Pass 3 — owner, 2026-08-09) Mint the device scope into the token.** The grant record carries
      the set of endpoint ids the cap reaches; `croft-admit` reads it and mints a token scoped to those
      endpoints. A caller-supplied device hint is checked **against** that scope, never trusted as it:
      a hint inside scope is honoured, a hint outside it is refused. This is the same discipline as the
      policy record — the caller-held artifact expresses preference, the callee's repo expresses
      authority.
- [ ] **Revocation is both** — short expiry on the minted relay token, *and* deletion of the grant
      record to stop re-issue. Expiry alone would let a revoked cap keep minting until it lapsed;
      record-deletion alone would leave already-minted tokens live. Together, revocation takes effect
      within one token lifetime and cannot be renewed.
- [ ] **(Free-form pass, 2026-08-10) The mint-time grant read must not ride Phase 7's identity cache,
      or the line above is false.** Phase 7 caches with a ~1h refresh age — sized for DID documents,
      where staleness buys bandwidth. A grant record served from that cache keeps a **deleted** grant
      minting for up to the refresh age, and "within one token lifetime" silently becomes "within cache
      age *plus* token lifetime." No test catches it, because fixtures do not age. So: the grant check
      at mint is **fresh or near-fresh** (no cache, or a cache age of seconds, decided in-phase and
      stated in the ADR next to the revocation claim), while identity resolution keeps Phase 7's two
      ages. One resolver, two cache policies, each sized to what staleness costs — and a test that
      deletes the grant and asserts the *next* mint refuses without any clock advance, which is exactly
      the assertion a cached read would fail.
- [ ] `sponsorship_for(caller, callee, membership, now) -> Budget`: member-involved → unlimited;
      otherwise → introduction budget. Mint to **both** parties.
- [ ] Service-auth JWT (via `ciss-auth`) authenticates a member's instruction where one is needed;
      `ReplayGuard` for `jti`.
- [ ] Quota decrement from the usage records Phase 3 emits.
- [ ] **(Free-form pass, 2026-08-10) Those records cross a process boundary this plan never wired.**
      Phase 3's sink is in-process in `croft-relay-bin`; Phase 4's supervisor reads it there — same
      binary, no gap. The quota decrement is in `croft-admit`, a different daemon. Settle the transport
      at phase start — the natural candidates are the relay binary POSTing usage to a localhost
      `croft-admit` endpoint, or writing it to the shared private CISS instance and letting croft-admit
      read it there — under two constraints that are not candidates: the record stays **volume-only,
      one endpoint per record, never a pair** (§3.3's rule for the never-attempted upstream patch binds
      our own wire just as hard), and a transport outage degrades **quota freshness, not calling** —
      the relay must never block a connection on croft-admit's availability. The wiring test asserts
      the decrement end to end across both processes, not against a hand-delivered record.
- [ ] Record the fate of `RegistryAccess` and the HTTP-hook path: **kept, labelled as the fallback**
      (Phase 5), not deleted.
- [ ] **(Pass 3) Update ADR-0003 in this phase** — claims gain a budget and revocation becomes
      expiry-*and*-record-deletion. §6 already schedules it here; it was missing from the write-set
      below, which is how a scheduled doc update silently becomes a docs-phase-at-the-end.
- [ ] **(Pass 3) Logging:** `WARN` on every mint refusal with a `reason` discriminant (`no_cap`,
      `cap_not_found`, `cap_revoked`, `jwt_invalid`, `replay`, `quota_exhausted`) — six refusals that
      look identical to the user and must not look identical to us. `INFO` on quota exhaustion for a
      member, which is a commercial event someone will need to answer for. `DEBUG` on a successful mint
      with the `cap_id` and the resulting budget class. **Never log the cap secret, the minted token, or
      the `(caller, callee)` pair** — the mint is the one place in the system that legitimately sees
      both parties, which makes it the one place a call-detail record could accidentally be written.
      §3.5's whole guarantee is destroyed by one convenient log line here.

**Call chain:** client → `/grantCall` → resolve → verify cap → `sponsorship_for` → mint → client swaps
its relay token.
**Wiring test:** A caller holding a valid cap is minted a token; the callee **deletes the grant record**;
the next mint attempt is refused. Deletion-then-refusal is the assertion — testing only the happy path
proves nothing about revocation. Driven over HTTP against the running `croft-admit` binary — the same
reason as Phase 6: a test that calls `sponsorship_for` directly proves the function, not the endpoint.

**(Pass 3) Boundary cases, named.** `sponsorship_for` has the same `||` hazard as Phase 4 and the same
four-row matrix applies (neither / caller / callee / both) — assert all four, at this layer too, since
the relay-side and mint-side membership rules are separate code that must agree. Then the refusal
surface, one row each, because they are six paths to one user-visible outcome:

| Presented | Expected |
|---|---|
| no cap | refused, `no_cap` |
| a cap that never existed | refused, `cap_not_found` |
| a cap whose grant record was deleted | refused, `cap_revoked` |
| a valid cap, invalid service-auth JWT | refused, `jwt_invalid` |
| a valid cap, replayed `jti` | refused, `replay` |
| a valid cap, member over quota | refused, `quota_exhausted` |
| a valid cap, everything in order | minted, with the budget class the matrix says |

**Revocation timing is a boundary, not a yes/no.** Assert that a token minted *before* the grant
deletion **stays valid until it expires** — that is the designed behaviour (§ "revocation is both"), and
a test that asserts immediate invalidation would be asserting a property the design deliberately does
not have. Assert the complementary half too: after deletion, no new token is issued. Together they are
the claim "revocation takes effect within one token lifetime"; separately, neither is.
**Depends on:** Phase 7, **and Phase 3 (Pass 2)** — the quota decrement consumes the usage records the
counting decorator emits. Pass 1 declared only Phase 7, hiding a cross-milestone dependency from
Milestone B into Milestone C.
**Read-set:** `crates/croft-admit/src/{token,tier,http_api}.rs`, `CISS/crates/ciss-auth/src/*`,
`crates/croft-relay-bin/src/usage.rs`.
**Write-set:** `crates/croft-admit/src/{cap,sponsorship,quota,token,http_api}.rs`,
**`experiments/croft-relay/docs/adr/0003-token-format.md` (Pass 3 — was scheduled in §6 but missing
here)**, `Cargo.toml`.
**Shared-state contract:** **(Pass 3 — invariants.)** Clock injected as the existing verifier does; no
wall-clock read outside the edge, so no test sleeps to reach an expiry. The `ReplayGuard` is
process-local and bounded — it never grows unbounded on `jti`s that have already expired. `ciss-auth`
enters as a **git dependency pinned to a commit** (Pass 3 — owner's standing rule); the pin is bumped
deliberately and never floated to a branch. No test reaches the network; caps and grant
records come from Phase 7's fixture server.
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
- [ ] **Grant record** in the callee's own repo, carrying an **opaque cap id and a device scope —
      nothing that names a person**. *(Free-form pass: "and nothing else" predated the device-scope
      ruling below; the two bullets now agree.)* It names no grantee, so the call graph is not
      published; the scope names only the callee's own endpoint ids, which `listRecords` already makes
      public. Deleting the record is revocation.
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
- [ ] **(Pass 3 — owner's ruling, 2026-08-09) One cap per callee, and the cap carries its own device
      scope. The link's device hint is not authoritative.**

      The insight that settles it: **a device is just an endpoint id.** Scope needs no new concept — it
      is a set of endpoint ids the cap reaches, and adding a device is adding another id.

      ```
        grant record (callee's repo, authoritative — signed by the repo commit)
            cap id: a3f9…                    ← opaque, names no grantee
            scope:  [endpoint_work]          ← which of my endpoints this cap reaches

        the cap (handed out, caller-held)     ← proves possession, carries no authority of its own
        the link's ?device= hint (caller-held) ← preference only; refused if outside scope
      ```

      "When I give you this thing, it is up to me where you can reach me" — the scope is chosen by the
      callee **at issuance**, which is the moment they already have the decision in mind. A caller
      cannot widen it, because the authoritative copy lives in the callee's repo and the caller's copies
      (cap, link) are just claims.

      **Why this does not re-introduce an allowlist.** The callee maintains no per-person state: they
      issue a cap with a scope and are done. The grant record still carries an **opaque cap id and no
      grantee**, so the call graph stays unpublished (§3.5). It now also names endpoint ids — which are
      **already public** via `listRecords`, so this discloses nothing new.
- [ ] Deep link's `?device=` is therefore **UI preference, not authorization** — the same relationship
      the policy record has to the cap. Client tries scoped devices in order when no hint is given.
      Settle the wire shape in `docs/contract.md` first, per that repo's rule.

**Call chain:** page → `resolveHandle` → PDS → `listRecords` (devices + policy) → render → deep link →
app. Separately: callee's client → write grant record → share cap out-of-band → caller's client stores
it → presents it at call setup (Phase 8 verifies it).
**Wiring test:** Two assertions, both end to end. (a) A test account with **two** device records
resolves to both and the page produces a working link for each — two devices is the assertion, one
hides every bug this introduces. (b) A cap issued by one persona's client is redeemed by another's and
verifies against the grant record; deleting the record makes it stop verifying.

**(Pass 3) Named behaviours, RED-first.** The policy enum is three-valued and the device list is
variable-length; both are single-point-assertion traps.

| Case | Expected |
|---|---|
| policy `nobody` | page renders "not callable" |
| policy `mutuals` | page renders the out-of-band invite affordance |
| policy `anyone` | same affordance (built, tested, unadvertised) |
| handle does not exist | **byte-identical refusal to `nobody`** — assert the equality directly, not each in isolation, or the leak returns the first time someone adds a helpful error message |
| policy record absent entirely | treated as `nobody`, not as `anyone` — the default must fail closed |
| 0 device records | page renders not-callable, no crash |
| 1 device record | link produced |
| 2 device records | **both** resolved, both linkable |
| a valid cap, policy `nobody` | **still admitted** — the cap is the only gate (the anti-second-gate assertion) |

**(Pass 3) Client-side logging is not the same problem.** The exchange page is a public, backendless
artifact: it must log **nothing** about who was looked up — not to a console, not to any analytics. The
Android client may log at debug for its own diagnosis, but a cap secret must never reach a log or a
crash report. State this in `contract.md`, since it constrains two codebases and neither one's authors
will infer it.

**Depends on:** Phase 8 for the verification side. The lexicon and page work may start as soon as the
shape is agreed.
**Read-set:** `connect/docs/contract.md`, `web/resolver.js`, `android/.../DeepLink.kt`.
**Write-set:** the same files plus the cap issue/redeem paths, in `CroftCommunity/connect`. **Different
repo** — separate PR, separate CI.
**Shared-state contract:** **(Pass 3 — invariants.)** Writes records only to the owner's two designated
fixture personas, never to a real account. Every record written by a test is deleted by that test, so a
re-run starts from the same state — a persona left with three device records makes the two-device
assertion pass for the wrong reason. Touches no path in this repo and performs no git operation here.
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
- [ ] **(Pass 3) The client must distinguish three endings and say so.** A budget drop, a network
      failure, and a callee declining are three different things that all present as "the call ended."
      The clean close chosen in Phase 4 exists precisely so the client can tell them apart — if the app
      shows one message for all three, the entire argument in §3.1 for budget-and-drop over throttling
      is forfeited at the last step. Assert each ending renders its own message.

**Call chain:** app call action → `/grantCall` → token → `insert_relay` → dial → (budget spent) →
disconnect → honest UI.
**Wiring test:** A sponsored client swaps its token live and the **EndpointId is unchanged** across the
swap — that stability is what keeps the published records valid. Plus: a non-member call that cannot
holepunch shows the membership message rather than failing silently.
**Depends on:** Phase 4, Phase 8, Phase 10.
**Read-set:** `crates/croft-relay-bin/src/main.rs`, `crates/croft-admit/src/http_api.rs`,
`connect/android/*`.
**Write-set:** client integration crate, a new member of the `croft-stack/relay/source/` workspace
(free-form pass: the repo-shape question this was gated on resolved to croft-stack on 2026-08-09).
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
      level aggregates**, per OPEN-QUESTIONS Q5.
- [ ] **(Pass 3 — owner's ruling, 2026-08-09) Aggregates only in metrics; identity only in logs; both
      retained at least 30 days. Document what and why in ADR-0006, not just here.**

      | | Cardinality | Retention | Carries identity |
      |---|---|---|---|
      | **Metrics** | tier-level labels only — never `endpoint_id` | **≥ 30 days** | no |
      | **Logs** | per-connection lines (§8.3) | **≥ 30 days** | `endpoint_id`, never a pair |

      **Why aggregates in metrics.** Two costs, different in kind. *Privacy:* a per-endpoint time
      series, kept for the monitoring system's whole retention and usually visible to everyone with a
      dashboard link, is a record of an individual's activity — one side of a call, not the pair, but
      the same slide toward holding records about people that §3.5 exists to avoid. *Cardinality:* one
      series per endpoint grows the metric count with the user base and melts the monitoring system at
      precisely the moment we are growing.

      **Why identity stays available in logs.** The debugging capability is real and we should not give
      it up — it just belongs where retention is bounded and access is narrower, rather than in a
      dashboard. §8.3's `DEBUG`/`WARN` lines already carry `endpoint_id` and `connection_id`.

      **Why write it down.** The failure mode is not a decision to surveil users. It is someone
      debugging at 2am who adds an endpoint label because it would help *right now*, and it stays. A
      recorded decision is what that person argues with.

      **Named honestly:** 30 days of identity-bearing logs is itself a deliberate choice, not a
      neutral default — it is a month in which who-connected-when is reconstructable from our logs for a
      single side of each call. Accepted as the working posture; revisit if the log content ever widens
      beyond one endpoint per line.
- [ ] Fuzz the token parser (network-facing), time-boxed.
- [ ] Load test: N clients, stable memory, correct budget behaviour, decorator overhead measured.
- [ ] **(Pass 3) Audit the logging this plan added, under load.** Every `WARN` introduced in Phases 2–8
      is on a path a stranger can trigger, which makes each one a log-amplification vector. Confirm the
      deny path allocates no per-attempt line at the default level, and that the per-attempt limiter
      sits **ahead of** verification cost rather than behind it.
- [ ] **(Pass 3) Grep the logging for the privacy rule, mechanically.** A test or CI check asserting no
      log call site takes both a caller and a callee identifier. §8.3 states the rule and ADR-0006
      records it, but a rule enforced only by review is one that survives until the first difficult
      debugging session.

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
- ~~**[Phase 6]** Does `IdIndex` still earn its place?~~ **(RESOLVED 2026-08-09 — owner.)** Kept, with
  key custody reframed as its own dial rather than an unanswered prerequisite. A DID is a strong enough
  identity proxy that the key deserves password-grade handling; ship **L1** (operator-supplied key,
  stored outside the store's backup path) as the simple default, and keep the shape of L2/L3 alive
  through a narrow `KeySource` seam while that is still cheap. The ADR must name which rung is in force
  and what it does *not* claim. See Phase 6.
- ~~**[CONFIRMED: PHASE-GATED — Phase 4]** Does `Endpoint::insert_relay` force a reconnect?~~
  **(SETTLED BY PROBE, 2026-08-10: no.)** See §4's row and `evidence/insert-relay-probe.txt`.
  Sponsored upgrades use disconnect-to-upgrade; no live-connection budget re-reader.
- **[CONFIRMED: PHASE-GATED — Phase 7]** DID-document cache TTL. **(Owner, 2026-08-09: phase-gated
  confirmed; shape settled, values provisional.)** Split into a **~1h refresh age** (background refresh,
  keep serving) and a **24h hard max-stale** (refuse), so noticing a rotation and surviving an upstream
  outage stop competing. Defensible because a stale key buys relayed bandwidth, not call content.
  Remaining for Phase 7: confirm both numbers against real plc.directory behaviour, and implement the
  cache shape (bounded, single-flight, failures never cached).
- ~~**[Phase 8]** `ciss-auth` as a path dependency, a git dependency, or vendored.~~ **(RESOLVED
  2026-08-09 — owner.)** **Git dependency pinned to a commit**, per the owner's standing rule: *if the
  dependency is ours, pin it by git; if it is not ours, bundle it and add CI checks.* A path dep only
  resolves under a fixed on-disk layout, so CI (which already runs `croft-relay` in the smoke matrix)
  and any single-repo clone could not build. Vendoring is the one option where a CISS **security** fix
  silently fails to reach an authentication path. The residual cost — someone must do the bumping — is
  the same standing obligation as watching upstream iroh releases.
- ~~**[Phase 10]** Deep link with a device selector, or client tries devices in order?~~ **(RESOLVED
  2026-08-09 — owner.)** **One cap per callee, carrying its own device scope**, because a device is just
  an endpoint id — scope needs no new concept. The callee chooses at issuance where a given cap may
  reach them; the scope lives in the grant record (authoritative) and is minted into the token; the
  link's `?device=` is a non-authoritative hint, honoured inside scope and refused outside it. No
  per-person state on the callee's side, and the grant record still names no grantee. Remaining for
  Phase 10: the wire shape, settled in `docs/contract.md` before either half is built.
- ~~**[Phase 11]** Repo shape (OPEN-QUESTIONS Q2).~~ **(RESOLVED 2026-08-09 — owner.)** **Moves into
  `croft-stack`, under the existing `relay/` directory, as `relay/source/`.** Not a standalone repo:
  the cohesive bundle is the service together with what deploys it, and `croft-stack/relay/` already
  holds `deploy/` (systemd units, `relay.toml`, the Caddy site) and `tests/` (bats) — the code is the
  missing third. Root `ansible/roles/relay/` stays where it is. Histories get mixed by the move;
  accepted as unlikely to matter. **Move in Phase 1**, before any code is written against the old
  paths — see §9.
- ~~**[Phase 12]** OPEN-QUESTIONS Q5: tier-level aggregates only, or endpoint-level labels?~~
  **(RESOLVED 2026-08-09 — owner.)** **Tier-level aggregates only in metrics; identity confined to logs;
  both retained at least 30 days.** Reasoning recorded in ADR-0006 rather than only in the plan, because
  the thing it has to survive is a 2am debugging decision. Endpoint-level metric labels are refused on
  two independent grounds — a long-retention, widely-visible record of individual activity, and
  cardinality that grows with the user base.
- ~~**[ADVISORY]** Store engine for Phase 6.~~ **(RESOLVED 2026-08-09 — owner.)** **A private CISS
  instance of our own, over localhost** — not the customer-facing deployment. The reason is *one
  persistence story instead of two*: backup, restore, failover, and the ops wisdom around them accrue to
  a single mechanism we are already committed to running. Corroborating: CISS is SQLite underneath (so
  "SQLite vs CISS" was a false choice), `ledger.rs` is the chained signed store both our records want,
  and `Did` keeps atproto and CISS-native `id:` identities disjoint at the type level, so this does not
  extend `id:` outside its scope. Concerns Pass 3 raised against a *shared* instance were withdrawn as
  inapplicable to a private one — see Phase 6's scoping note.
- ~~**[ADVISORY]** Who watches upstream releases once Phase 5 makes the release cadence ours?~~
  **(RESOLVED 2026-08-09 — owner.)** **Automated dependency PRs against croft-stack's new gate**, not a
  person on a schedule. Being three releases behind on a *prebuilt* binary is the evidence that manual
  noticing does not happen. It covers **two** standing obligations with one mechanism: `iroh-relay`
  from crates.io, and the git-pinned `ciss-auth` (Phase 8) — both pinned dependencies in paths where a
  missed security fix matters. Cheap specifically because the gate from §8.4 lands in Phase 1 anyway:
  an automated bump PR is worth little without a gate to prove the bump is safe. **Phase 5 checklist
  item**, since Phase 5 is where the obligation is taken on.

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

### Pass 3: Quality Gates — 2026-08-09

Applied additively; no phase restructured, no reasoning rewritten. The plan's shape is unchanged from
the post-rewrite version.

**TDD ordering:**
- Added **§8.1**: the Changes lists read implementation-first but are executed test-first, with Phases 0
  and 1 named as the only exemptions. Previously this was implied by the per-phase Wiring test and
  nowhere stated.
- Added **named behaviours, RED-first** to Phases 2, 3, 6, 7, 8, and 10. Pass 2 left several phases with
  a wiring test and no unit-level specificity, which is how a phase ships a stub that satisfies one
  end-to-end assertion.
- **Mutation resistance — the largest single finding.** Phase 4's `budget_for` and Phase 8's
  `sponsorship_for` both implement "either party is a member," an `||` that a neither/both test pair
  passes against `&&`. That mutation ships a product silently requiring *both* parties to pay. Both
  phases now carry the full four-row membership matrix, plus budget boundary rows (under / at / over)
  and Phase 8's six-row refusal-reason surface.
- Phase 8 gained the **revocation-timing pair**: a token minted before deletion stays valid until
  expiry (the designed behaviour), *and* no new token issues after. Asserting only one of these asserts
  a property the design deliberately does not have.
- Phase 3's byte assertion re-specified as a **ratio**, which survives framing overhead while still
  failing on the wrong stream, one-direction counting, or double counting.
- Added **§8.2**: the existing `phaseN_*.rs` tests are numbered against the superseded plan —
  `phase3_tier.rs` tests the old Phase 3 while this plan's Phase 3 is the counting decorator. New tests
  use topic names; existing files are not renamed (they are cited in evidence), and Phase 1 records why.

**Observability:**
- **Spot-check finding: the tree has no `tracing` or `log` dependency and no logging call anywhere.**
  Acceptable for a library spike; not for two long-running daemons whose central product behaviour is
  silently dropping a connection. Every phase in Milestones B and C was going to build on a convention
  that did not exist.
- Added **§8.3**: the convention (crate, level semantics, and an explicit "`TRACE` unused" so nobody
  logs in the byte path), established in **Phase 2** rather than deferred — metrics still land in Phase
  12, logging does not wait for it. All-observability-at-the-end is the same anti-pattern as a docs
  phase at the end.
- Added the **log privacy rule** and routed it into ADR-0006 (Phase 1) and a mechanical CI check (Phase
  12): never a token, a cap secret, or a **DID pair**. Phase 8's mint is the one component that
  legitimately sees both parties, which makes it the one place a call-detail record could be written by
  a debugging convenience — destroying §3.5's guarantee through the back door.
- Per-phase logging added where a failure would otherwise be undiagnosable: Phase 3's unattributed-
  connection `WARN` (the three-way join failing silently), Phase 4's drop lines with a stable `reason`
  discriminant **plus a near-miss `DEBUG`** (the only signal that Phase 0's number was wrong), Phase 6's
  `IdIndex`-mode-at-startup (`Transparent` in production is otherwise invisible), Phase 7's
  which-of-five-lookups-failed, Phase 8's six refusal reasons.
- Phase 5 makes the logging **operable**: journal-only, level in the unit file, and the two greps an
  operator runs first recorded in `06-iroh-relay.md`.

**Debugging readiness:**
- Added the checkpoints section (§8.4 at the time; **now §8.5**, after the relocation section was
  inserted): three load-bearing checkpoints (after P2, P4, P8), so a later failure is localized by
  re-running the checkpoint above it rather than bisecting a milestone.
- Phase 5 gained a **written rollback procedure** — which variable reverts, how long, and what
  capability is lost while reverted. The stock-binary fallback was named but its steps were not.
- Phase 2 gained fail-loud config validation, so a relay that cannot verify anything aborts rather than
  starting and denying every connection in a way that reads as a network fault.

**Validation calibration:**
- **Phase 3: Moderate → Broad.** Rated as measurement-only, but it settles the plan's one load-bearing
  unverified assumption (post-upgrade bytes traversing `CountingStream`) and everything Phase 4 enforces
  depends on its attribution being right. A subtly wrong join makes Phase 4 enforce against the wrong
  person while Phase 4's own tests pass.
- **Verification commands re-pointed through entry points.** Phase 6's persistence test must start the
  binary (restart is meaningless in-process); Phase 8's must drive HTTP against the running service;
  Phase 3's must not construct `CountingStream` directly, or it proves the type works while the binary
  never calls it.
- All other phase validations confirmed proportionate. Phase 1 stays Narrow (docs plus one toolchain
  edit); Phase 5 stays Broad with staging-before-production.

**Concurrency honesty:**
- Map accounts for all 12 phases. Write-sets re-checked after Pass 3's edits: Phase 2 now also writes
  the workspace root `Cargo.toml` (new member plus the tracing deps) and Phase 8 now also writes
  ADR-0003 — **neither creates a new collision**, since B and C are already sequential.
- **Shared-state contracts converted from mechanisms to invariants** in Phases 2, 3, 4, 6, 7, 8, and 10.
  "Tests use `:0`" is a mechanism; "binds only ports obtained from `:0`, never a fixed port; leaves no
  process running after the test binary exits" is an invariant. Phase 7's is the one that mattered most:
  "CI must have no egress expectation" became "no test resolves a hostname outside `127.0.0.1`, base
  URLs are injected by config," which is checkable rather than aspirational.
- **New parallel candidate flagged, not adopted:** Phase 10 could run alongside Milestone B — different
  repo, disjoint write-set, and only its verification half needs Phase 8. Left sequential because it is
  a resourcing decision; surfaced with its own re-entry checks so it is a choice, not an oversight.

**Discovery (Phase 0):**
- **Dispositions were missing entirely** — a plan defect, since the Discovery Exemption exists precisely
  to bound what unTDD'd code may become. Now declared per task: harness `keep-as-fixture` (Phase 4's
  forced-failure test needs the same rig), byte instrumentation **`promote`** with Phase 3 named as the
  phase that re-implements it test-first, everything else `throwaway`. No Phase 0 code reaches `crates/`
  without a RED-first cycle.
- Checked whether any Phase 0 task could be resolved during planning: no. Every one needs two hosts on
  separate networks — the resource this phase is gated on.

**Coherence:**
- The plan still solves §2's four problems and no scope crept in; Pass 3 added no new capability, only
  tests, logging, and checkpoints around existing ones.
- One consistency fix: Phase 1's ADR-0006 bullet still said "the request-policy flow," which the
  out-of-band decision replaced. Now "out-of-band cap distribution."

**Documentation impact:**
- ADR-0003 was scheduled in §6 for Phase 8 but **absent from Phase 8's write-set** — exactly how a
  scheduled doc update becomes an end-of-plan docs phase. Added.
- Four Pass-3 doc updates added, each in the phase that triggers it: the README test-naming note (P1),
  the config doc's log knob (P2), the rollback procedure (P5), and `contract.md`'s client-side logging
  constraint (P10).

**Confirmed ready:** Yes — see the walk-through below, which closed every open question.

### Pass 3 walk-through: all nine open questions closed — 2026-08-09

Walked one at a time with the owner. **Two were deferred as-is, seven were decided**, and three of the
decisions changed the plan's structure rather than filling in a blank.

| # | Question | Outcome |
|---|---|---|
| 1 | `insert_relay` reconnect (P4) | **Phase-gated, unchanged.** Probe first; either answer is workable |
| 2 | Does `IdIndex` earn its place? (P6) | **Kept** — key custody reframed as its own dial (L0–L3), ship **L1** behind a `KeySource` seam |
| 3 | Store engine (P6) | **A private CISS instance over localhost** — one persistence story instead of two |
| 4 | DID cache TTL (P7) | **Two ages, not one:** ~1h refresh, 24h hard max-stale |
| 5 | `ciss-auth` dependency shape (P8) | **Git pin**, per a standing rule now recorded: ours → git pin, not ours → vendor + CI checks |
| 6 | Device selector (P10) | **One cap per callee, carrying its own device scope** — a device is just an endpoint id |
| 7 | Repo shape (P11) | **Moves to `croft-stack/relay/source/`, in Phase 1** — service bundled with what deploys it |
| 8 | Metric cardinality (P12) | **Aggregates only; identity in logs; ≥30-day retention on both** |
| 9 | Upstream release cadence | **Automated dependency PRs** against the new gate, covering `iroh-relay` and `ciss-auth` |

**Structural consequences, none of which existed before the walk-through:**

- **(#7) Phase 1 is now a two-repo, two-PR phase** and its validation rises Narrow → Moderate. It
  relocates the workspace and must **add a CI gate to `croft-stack` first** — that repo's only workflow
  is `deploy-service.yml`, a `workflow_call` deploy with no `pull_request` trigger, which
  `.claude/CI-PATTERN.md` classifies as a notification rather than a gate. Order is gate → move →
  remove `discovery`'s smoke entry; any other order leaves the crates in a repo with no PR checks.
  This also **retired §6's "CI-PATTERN: no change needed"**, which held only while the code stayed in
  `discovery`.
- **(#7) Phase 0 ∥ Phase 1 is disqualified.** Phase 1 moves the directory Phase 0 writes evidence into.
  Both operations are individually legitimate, so git gives no warning. Phase 1's move goes first,
  which costs nothing since Phase 0 waits on an owner-supplied second network.
- **(#3 → #2) The `IdIndex` decision is provisional on where the roster actually sits.** Settled by the
  clarification that CISS here is a *private* instance, not the customer-facing one — which withdrew
  three concerns Pass 3 had raised against a shared deployment, recorded in Phase 6 so they are not
  re-derived. What survives is narrower and honest: the **pepper** does the work (DIDs are enumerable,
  so a salt alone buys nothing), and exact-match becomes the only query available.
- **(#6) The device scope is authoritative in the grant record, advisory in the link** — the same
  caller-holds-preference / callee-holds-authority split the policy record already has. Notably this
  does *not* re-introduce an allowlist: the callee keeps no per-person state, and the grant record still
  names no grantee.
- **(#9 ← #5) Two standing obligations, one mechanism.** The git pin from #5 has the same failure shape
  as owning the relay release cadence, and the automation from #9 covers both — cheap only because
  #7's gate lands in Phase 1 regardless.

**Also updated:** the holepunch working assumption is now recorded as a **range** (10–15%) rather than a
point, matching how it was given, with a note that the figure decides how often the budget path is
exercised and never whether it is correct.

**No open questions remain.** The plan is ready for execution.

### Free-form coherence pass — 2026-08-10

A full end-to-end read after the question walk-through, checking the phases against each other rather
than each in isolation — the amendment-heavy history (rewrite, two Pass 2s, Pass 3, nine resolutions)
made stale cross-references the likely defect class, and that is what was found. The design itself
survived the re-read: the mechanism, the verified seams, and the two-gate structure required no change.

**Two substantive gaps, both invisible to any single-phase review:**

- **The Phase 7 cache broke Phase 8's revocation claim.** The identity cache's ~1h refresh age, applied
  to the mint-time grant read, turns "revocation within one token lifetime" into "within cache age plus
  token lifetime" — and no test would catch it, because fixtures do not age. Fixed: the grant check at
  mint is fresh or near-fresh (one resolver, two cache policies, each sized to what staleness costs),
  with a no-clock-advance deletion-then-refusal test that a cached read would fail.
- **Usage records cross a process boundary nobody wired.** Phase 3's sink is in-process in the relay
  binary — fine for Phase 4, same process — but Phase 8's quota decrement runs in `croft-admit`.
  Transport now named as an explicit in-phase decision with two fixed constraints: volume-only and
  never a pair; and a transport outage degrades quota freshness, never calling.

**Contradictions left behind by later decisions, reconciled:**

- Phase 6 still described the pre-CISS store throughout — "store path," tmp-store wiring test,
  filesystem invariants. Reworded end to end for CISS-over-localhost, including the previously
  unstated **cross-repo test dependency** (the persistence test needs a real CISS binary, sourced by
  the standing dependency rule; absent binary the test fails, never skips) and the both-processes
  restart assertion.
- The grant record was "an opaque cap id and nothing else" in two places, twenty lines from the
  device-scope ruling that added a scope to it. Both aligned: cap id + device scope, never a grantee.
- Phase 11's write-set still gated the client crate's location on the resolved repo-shape question.
- §3.5's heading said "caller's own repo"; the body and the whole design say callee's.
- Removed Phase 1's leftover conditional manifest-hoist bullet (mooted by Pass 2); "request-policy
  record" → grant + policy records in §6; §1's matrix now says "ask out of band"; the Pass 3 log's
  checkpoint reference renumbered to §8.5.

**Confirmed sound on re-read, for the record:** the B→C sequencing and its two collisions; the
three-way token join and its Phase 3 assertions; the budget boundary rows; the fail-closed resolver
matrix; the relocation ordering (gate → move → smoke-matrix removal); and that every §4 source citation
still points at text this plan actually relies on.

**Verdict: correct, viable, and ready to execute** — Phase 1 first, and Phase 8's two new bullets are
in-phase decisions, not new open questions.

### Plain-English review with the owner — 2026-08-10

The full design was retold in plain English with user stories before starting Phase 1; the owner
reviewed it and made three calls, now folded in as **§1.2**:

- **"Carried" corrected to "carried when needed."** Membership does not route calls through the relay;
  a member's calls holepunch at the same rate as anyone's. Membership removes the budget for the
  fraction that can't. The §1 matrix now says so.
- **The storage/visibility claim stated precisely.** The grant record is public-but-opaque, so an
  observer can see *that* you issue invites (their count, their device scopes) but never *to whom*.
  The comfortable phrasing — "nobody can look anywhere" — overstated it; §1.2 now carries the honest
  table of what lives where and what is public.
- **`croft-admit` renamed `croft-relay-admit`**, riding the Phase 1 move: the crate is the relay's
  admission authority, and the old name read as a general-purpose service it is not. Paths in this
  document keep the old name below Phase 1 for the same reason they keep the old repo paths — they
  match what the passes verified. Read `croft-admit` as `croft-relay-admit` after Phase 1.

### Phase 1 executed — 2026-08-10

Both repos, in the plan's order: **gate → move → smoke removal → record.**

- **croft-stack** (branch `relay-source-move`, four commits): a pre-existing red bats assertion
  fixed first (the cert path moved to tmpfs and the test still said `/etc` — the gate must not be
  born red on drift that predates it); the gate workflow (`gate.yml`: relay/source Rust +
  relay/tests bats, on every PR and push to `main`, trivially green in the pre-move window); the
  move itself with the `croft-admit` → `croft-relay-admit` rename riding it, plus
  `rust-toolchain.toml` (1.94.1) and the MSRV-comment fix; then the record — **ADR-0006**,
  ADR-0004's mechanism superseded with the unreachable-fallback correction in place (content-
  blindness retained), ADR-0001 superseded-in-part, README rewritten for the current design,
  croft-stack README + CONTRACT.md scoping notes (the workspace is not a kit tenant).
- **discovery** (on `main`): tree removed, smoke matrix entry replaced with a pointer, **T62
  rewritten** (new gates: budget sizing / relay binary end-to-end / deploy / the `insert_relay`
  probe; the five owner-calls noted resolved), backlog §6j banner, MASTER-INDEX + RUN-01-summary
  relocation notes, manifest row updated. COHESION needed nothing — its croft-relay reference is a
  wiki-link to a lab note, not a path into the moved tree.
- **Deviation from the phase text, correct on arrival:** the OPEN-QUESTIONS bullet said "resolve
  Q1/Q3/Q4, leave Q2/Q5" — written before the 2026-08-09 walk-through resolved Q2 (repo shape) and
  Q5 (metric cardinality). All five are closed in the file, each citing its ruling.
- **Verification:** cargo test **46/46**, clippy clean, bats **8/8** post-move;
  `grep -rn "wrap the admitted connection" docs/` returns nothing; discovery's smoke no longer
  references croft-relay. Local rustc is Homebrew (ignores the toolchain pin) — the pin is
  enforced by the gate, which is the agreement that matters.
- **Still open in this phase:** nothing. Phase 0 (budget sizing) is next and remains gated on the
  owner-supplied second network; Milestone B (Phase 2, our binary) can start independently of it.

### Phase 2 executed — 2026-08-10

Branch `relay-bin` in croft-stack (stacked on the Phase-1 PR), commit `2c1424f`. RED-first per §8.1:
`tests/live_binary.rs` written against a stub `main` and watched fail (3 of 4, each for the right
reason — the fourth, unparseable-config, passed trivially against a stub that exits nonzero, which is
the weakest assertion in the file and is noted as such), then implemented to green.

- **The binary:** `croft-relay --config relay.toml`. Accept loop → `MaybeTlsStream::Plain` →
  `http1::serve_connection(...).with_upgrades()` → `RelayServiceWithNotify` — the embedding pattern
  upstream's own docs carry, all types public as §4 verified. `TokenAccess` wired via a `LoggedAccess`
  adapter (admit/deny + reason at DEBUG; verification untouched). No `client_rx` rate limit, per
  Phase 4's design.
- **Named behaviours, all through the spawned process:** valid token → admitted, datagram relayed
  A→B intact; absent / malformed / wrong-key tokens → each denied at the handshake; config missing
  `verification_pubkey_hex` → aborts naming the field; unparseable config → aborts.
- **§8.3 established:** first `tracing` dependency in the tree. Logs to stderr; stdout carries
  exactly one contract line, `listening on <addr>`, which the wiring test and later the deploy's
  readiness check parse. INFO startup logs the pubkey **fingerprint**, never the key.
- **Stated deviation:** no `[tls]` config section. The phase text names cert paths; shipping TLS
  here would ship untested production code (no test forces it — the staging wiring test in Phase 5
  is where real certificates exist). `deny_unknown_fields` makes a premature `[tls]` block a loud
  abort. Recorded here so Phase 5 picks it up as scope, not as a surprise.
- **Verification:** workspace 12 suites green (4 through the running binary), clippy clean,
  `cargo fmt --check` clean. Hand-run against a real iroh endpoint (the phase's Moderate validation)
  remains open until one is convenient; the relay-client legs cover the protocol path meanwhile.

### Phase 3 course-correction: the in-line decorator cannot exist — 2026-08-10

The phase's own "verify in-phase, do not assume" bullet fired **before implementation**, at the
source level. `Upgraded::downcast::<TokioIo<MaybeTlsStream>>()` (`http_server.rs:96`) recovers the
concrete IO type after the websocket upgrade; `MaybeTlsStream` is `#[non_exhaustive]` over concrete
`TcpStream` variants. So the Pass-2 design — wrap the stream, hand it to `serve_connection` — breaks
the downcast and with it every relay connection. Not "untested": impossible.

**Revised design — the loopback airlock.** The public connection never reaches the relay directly:

```
public accept → CountingStream<TcpStream> ──pump──▶ loopback TCP ──▶ internal accept
                (counts every byte,                                    │ port map → ConnState
                 connection lifetime,                                  ▼
                 upgrade included)                     serve_connection(MaybeTlsStream::Plain(tcp))
                                                       — a REAL TcpStream: downcast intact
```

- `CountingStream` survives as planned, on the **public** side, where every byte of the connection's
  lifetime passes — the refuted assumption becomes trivially true.
- The relay side is untouched upstream code over a genuine `TcpStream`.
- **The join gains a fourth participant** but keeps the token as its key: the pump binds its loopback
  client socket **before** connecting (port known → `ConnState` registered → no accept race), the
  internal accept resolves peer-port → `ConnState`, the per-connection Service wrapper reads the
  token, and `on_connect` supplies `endpoint_id`/`connection_id`.
- **Bypass guard:** the internal listener drops any connection whose peer port is not in the map —
  a local process cannot skip the meter. (In production the netns already isolates the listener;
  the guard makes the property hold everywhere, not just there.)
- **Cost, named:** one extra user-space copy and a loopback socket pair per connection, on the
  relay's data path. Accepted: it is the price of counting without forking, and Phase 12 already
  measures decorator overhead. The upstream accounting hook remains the future contribution that
  would delete this cost.
- **Usage records** are emitted as one structured line on the dedicated `usage` tracing target at
  connection close — the §8.3 "human-readable shadow" is, for now, also the machine-readable surface
  the wiring test parses; Phase 8 settles the real transport.

### Phase 3 executed — 2026-08-10

Commit `536cb7a` on `relay-bin` (PR #6). The course-correction entry above records the refutation
that opened the phase; this records the build.

- **RED honestly:** the wiring test failed against the Phase-2 binary (no records emitted), then
  drove the airlock green. `CountingStream` has exact-count unit tests at the duplex level; the
  wiring assertions are the plan's named set — the 10× ratio measured **9.5×**, no
  cross-attribution, record ≥ post-upgrade payload, and the unattributed-WARN-no-record path.
- **The join completes on deny as well as admit** — a denied handshake still has an authenticated
  endpoint worth attributing, so even a rejected connection's bytes land under a subject.
- **One field bug, fixed in the binary not the parser:** `tracing` emitted ANSI color codes to a
  piped stderr, so `bytes_in=` was never a literal substring of the record line. `with_ansi(false)` —
  stderr is a record surface and a journal, not a terminal. The kind of bug only a
  through-the-binary test catches.
- **Validation (Broad, as Pass 3 re-rated it):** 13 suites green, clippy zero warnings, fmt clean;
  the formerly load-bearing unverified assumption is now true **by construction** (the airlock pump
  carries the connection's whole lifetime) and asserted anyway.
- **Carried forward to Phase 12:** the airlock adds one user-space copy and a loopback socket pair
  per connection on the data path — the overhead measurement Phase 12 already owns now has a
  concrete thing to measure.

### Phase 4 executed — 2026-08-10

Commit `dd5cff9` on `relay-bin` (PR #6). The phase opened with its gate: **the `insert_relay` probe**
(scratchpad, full-iroh dep deliberately kept out of the workspace; evidence + probe source preserved
in `evidence/insert-relay-probe.txt`). **Verdict: no reconnect** — the established connection persists
with the old token; the swapped config is read only at the next connect. Consequence adopted:
**sponsored upgrades are disconnect-to-upgrade**, the same lever this phase builds; no live-connection
budget re-reader exists or is needed.

- **`tier.rs` rewritten:** `bucket_for` → `budget_for`, `RateBucket` → `Budget`
  (`Unlimited | Bytes`), SPEC-DELTA moved to the introduction-budget placeholder (256 KiB — build
  against, never deploy; Phase 0 measures). The exceeded decision is a method with the boundary rows
  as unit tests: under / **at** (not exceeded — the budget is what you may spend) / one-over /
  unlimited-never / zero-budget. **Mutation run: 8 mutants, 7 caught, 1 unviable, zero survivors** —
  the `>`→`>=` mutant died to exactly the at-the-budget row, which is the mutation-resistance
  argument §8.1 made, demonstrated.
- **`phase3_tier.rs` deleted, `tests/budget.rs` replaces it** (topic-named, §8.2): the old file
  tested the superseded rate-bucket shape and could not survive the API it pinned being removed.
- **The supervisor:** 100ms scan; over-budget → `dropped` flag (idempotent while the async disconnect
  completes), token → spent set, WARN `reason=budget_exhausted` with `bytes_counted` + `budget` (one
  endpoint per line), `Clients::disconnect(endpoint, None)`. **Spent-token refusal runs first in
  `on_connect`, before verification**, so iroh's auto-reconnect cannot flap. Near-miss DEBUG within
  20% of budget — the line that shows when Phase 0's number is wrong.
- **RED honestly:** `budget_drop.rs` failed against the unenforcing Phase-3 binary on exactly the
  enforcement path (the two negative tests passed trivially, as expected — their value is guarding
  against over-enforcement now that it exists). Green through the spawned process: dropped → refused
  on the spent token → admitted on a fresh one; Broker survives the volume that kills Coordination;
  under-budget left alone.
- **Membership-matrix note, stated honestly:** at the relay layer "either party is a member" has
  already been decided at mint and arrives as a tier. The four-row matrix is asserted where the `||`
  lives — `sponsorship_for`, Phase 8. This phase's wiring asserts Broker survives what kills
  Coordination on the same binary; the caller-only/callee-only rows are mint-side.
- **DESIGN.md** tier-enforcement section rewritten to budgets (the phase's doc-impact item).
- **Validation:** 14 suites green, clippy zero warnings, fmt clean, mutation clean on `tier.rs`.
  Remaining from the phase's list: nothing.

### Phase 6 in execution — chunk 1 done, 2026-08-10

Cross-repo pin semantics noted on **both sides** first (owner's ask): CISS's README carries a
"downstream consumers" table (who pins what, drift needs a deliberate bump, CHANGELOG the surfaces
consumers use); `relay/source`'s README carries the mirror. Both pushed.

**Chunk 1 (`61d6ff9`): `IdIndex` at L1 + the narrow store.** RED-first; mutation run on the two new
files: 13 caught, 5 unviable, **zero missed** — the two initial survivors (Pepper's Debug-redaction
body; a `> 0` micro-guard in `migrate`) were killed by a redaction pin and by deleting the guard,
not explained away. Keyed mode's absent-key path is a hard error naming the source; the migration is
tested at birth including plaintext-is-gone; the store trait cannot express a pair.

**Chunk 2 (next):** the `ciss` git pin (lib for Model-A assertion signing, binary for tests), the
`CissStore` impl over `/{did}/assertion/{kind}/{subkey}` with croft-relay-admit as a CISS tenant
(`kind=member|usage`, `subkey=digest`), the `[[bin]]` + router + §8.3 logging, and the
both-processes-restart persistence wiring test (fails, never skips, without the binary).
