# fed-shim — the fediverse-wire conformance shim

`Standing charter, drafted 2026-07-20 alongside the shim's landing. Standing header
— the graduation trigger is a NAMED condition (RUN-ATTEST-04 V10 convention):
when a real Mastodon instance (or a real GoToSocial or Pleroma) has round-tripped
against the shim over an attended live leg AND the observed byte-level
discrepancies (if any) are enumerated and either (a) reflected back into the
shim or (b) recorded as a firm gap in §3 below. This document stays in alpha
beside the crate until that trigger fires.`

## 0. Governing principle (read this first)

**fed-shim is a WIRE-CONFORMANCE surface, not a Mastodon replica.**

It models the byte-shape of a small, specific set of ActivityPub interactions
so that other Croft crates (starting with `ap-ambassador`) can be integration-
tested deterministically, in-env, without an outbound network. The shim's
promise is **strong on the wire it does model**, and **explicitly silent on
everything else**. What it does not model is *firm* — it is neither hedged
nor partially approximated; it is a documented gap that the attended live
leg (§4) has to close.

**Fidelity discipline (§1 rule set):**

1. Every wire behavior the shim produces or accepts is checked against a
   captured specimen from real Mastodon behavior (or against the referenced
   spec text where a specimen isn't available). No behavior is "roughly
   Mastodon-shaped" — it either matches, or it is out of scope.
2. When the shim cannot faithfully model a behavior (rate limits, retry
   queues, media pipelines, moderation, Sidekiq semantics, PostgreSQL
   consistency), it does not *approximate* the behavior; it either refuses
   to expose that surface at all, or exposes a clearly-marked stub that
   returns `Unimplemented` / `NotAShim` errors. Silent partial fidelity is
   the failure mode this charter is designed to prevent.
3. Every claimed conformance point cites its evidence: either a saved
   specimen path under `tests/specimens/` or a spec section reference.
4. The shim does not pretend to be a real fediverse instance. It is
   named `fed-shim`, its actor URLs live under a private `fed-shim.local`
   domain by default, and it never publishes an activity to the real
   Internet. Any test that requires a real Mastodon reaction to the
   shim's output is an attended live-leg concern, not a shim-side
   concern.

## 1. Scope — what the shim MODELS (each with a fidelity source)

For every listed capability, the shim's behavior is pinned to either
(a) a **saved specimen** captured from a real Mastodon instance (path listed),
or (b) a **spec citation** where the specimen was not obtainable in-env this
run.

| # | Capability | Fidelity source |
|---|---|---|
| 1 | Draft-cavage HTTP signatures, `rsa-sha256`, over the covered headers `(request-target) host date digest` | Mastodon HTTP-Signatures implementation (Mastodon `app/lib/request.rb`) + draft-cavage-http-signatures-12 §2.1–§2.3. Shim signs and verifies via `ap-ambassador::verify` — reused unchanged. |
| 2 | `Digest: SHA-256=<b64>` header over the raw body | draft-cavage §5. Shim computes/verifies via `sha2::Sha256` + `base64::STANDARD`. Reused from `ap-ambassador::verify::digest_matches`. |
| 3 | Follow activity JSON shape: `{"type":"Follow","id":<url>,"actor":<url>,"object":<url>}` | AP-1.0 §5.1 (Follow). Mastodon serialization observed in `mastodon/app/serializers/activitypub/follow_serializer.rb`. Specimen: `tests/specimens/mastodon-follow-observed-shape.md`. |
| 4 | Undo Follow shape: outer `{"type":"Undo",...,"object":{"type":"Follow",...}}` (nested Follow with its own `id`) | AP-1.0 §7.4. Mastodon `undo_follow_serializer.rb`. Specimen: `tests/specimens/mastodon-undo-follow-observed-shape.md`. |
| 5 | Delete(Actor) shape: `{"type":"Delete","actor":<url>,"object":<url>}` where `object` equals `actor` (account-delete) | AP-1.0 §5.7. Mastodon `delete_actor_serializer.rb`. Specimen: `tests/specimens/mastodon-delete-actor-observed-shape.md`. |
| 6 | Actor-document GET response: JSON-LD with `id`, `type: "Person"`, `preferredUsername`, `inbox`, `outbox`, `publicKey.{id,owner,publicKeyPem}` | AP-1.0 §4.1 + Mastodon `actor_serializer.rb`. RSA public key serialized as SPKI PEM (`-----BEGIN PUBLIC KEY-----`). Specimen: `tests/specimens/mastodon-actor-doc-observed-shape.md`. |
| 7 | Inbox POST accepts the four kinds above (Follow / Undo Follow / Delete) and only those; every other kind returns 501 with a JSON body naming the kind as unsupported | Firm non-goal: replies/likes/announces/updates are AP-OC-9 in the RUN-AP-01 charter — the shim REFUSES them explicitly rather than accepting-and-ignoring. |
| 8 | Content-Type discipline: outgoing bodies carry `application/activity+json` (not `application/ld+json`); inbox POST accepts either | RFC 7565 §7. Mastodon's own emit chooses `application/activity+json`; parsers accept both. |
| 9 | Deterministic construction: same input (actor keypair + activity fields + Date header) → byte-identical signed request output | Not a Mastodon fidelity claim — a **shim-specific determinism claim** so tests are reproducible. Mastodon itself uses a wall-clock `Date` and OS randomness for signature blinding; the shim takes both as caller parameters. |

**Non-goals inside the shim's scope (still §1, listed because they surface
often and would otherwise be mistaken for gaps):**

- The shim does not model Mastodon's WebFinger. Actor documents are served
  directly at their `id` URL (as they are in AP-1.0 §3.4.1); a real-world
  federation flow that must resolve `@alice@example.social` to an actor URL
  happens outside the shim. This is a firm gap, see §3.

## 2. Public surface — the shim's API shape

```rust
// A shim actor: identity + key material, wire-fidelity to a Mastodon actor.
pub struct ShimActor { … }

// Sign and produce a Mastodon-shape inbox POST. Deterministic given (actor,
// date, body). Returns bytes ready to POST to the peer's inbox.
pub fn build_inbox_post(
    actor: &ShimActor,
    peer_inbox_url: &str,
    date: &str,
    body: Vec<u8>,
) -> InboxPost;

// Accept an inbox POST (the receiver side). Verifies HTTP signature and digest
// through ap-ambassador::verify::verify_ap_http_signature UNCHANGED. Returns
// the verified activity or a typed error.
pub fn accept_inbox_post<R: KeyResolver>(
    req: &SignedRequest,
    resolver: &R,
) -> Result<VerifiedActivity, VerifyError>;

// Serve the actor document JSON-LD for a given ShimActor. Byte-fidelity to
// Mastodon's actor_serializer.rb shape.
pub fn actor_document(actor: &ShimActor) -> Vec<u8>;

// Convenience: build a Follow / Undo Follow / Delete activity body from
// simple parameters. Mirror the JSON shapes Mastodon's own serializers emit.
pub fn build_follow(actor: &ShimActor, object_url: &str, activity_id: &str) -> Vec<u8>;
pub fn build_undo_follow(actor: &ShimActor, follow_id: &str, follow_object: &str, activity_id: &str) -> Vec<u8>;
pub fn build_delete_actor(actor: &ShimActor, activity_id: &str) -> Vec<u8>;
```

The shim is a **library** — no bin target, no async runtime, no HTTP server
(the sending side hands you bytes to POST; the receiving side takes a
`SignedRequest` and returns a verified activity or an error). This keeps the
surface small enough to audit, and keeps every test synchronous and
deterministic.

## 3. FIRM non-goals — the shim does NOT model these (attended live-leg territory)

These are gaps by design. Any test that needs them is out of shim scope and
is either done at the attended live leg (§4) or noted as a residual on the
RUN-AP-01 / RUN-AP-02 boards.

- **Persistence** — the shim keeps no state between invocations. Retry
  queues, delivery inboxes as durable stores, dedup by activity id: not
  modeled. If your test needs "the receiver has already seen this
  activity", set that up in your test's state, not in the shim.
- **Rate limiting, backoff, Sidekiq / job queues** — Mastodon's delivery
  worker retries with exponential backoff up to some ceiling and retires
  the peer after enough failures. The shim does not retry, does not back
  off, and does not maintain per-peer health state. RUN-AP-02's outbound
  delivery mechanics (`AP-OC-6`) design and test these in a separate
  layer.
- **HTTP transport** — the shim produces the BYTES of a signed request
  and CONSUMES the BYTES of a signed request. Actually opening a TCP
  socket, doing TLS, negotiating HTTP/2 keepalive, etc., is out of
  scope. RUN-AP-02 will wire the shim's bytes into an HTTP client for
  in-env integration.
- **WebFinger / handle resolution** — `@alice@example.social` → actor
  URL. The shim's actor URLs are known to callers directly. Real
  federation adds a WebFinger round-trip; the shim skips it.
- **Media proxy / blurhash / preview cards** — none.
- **The Mastodon REST API** (`/api/v1/*`) — the shim does not serve any
  of it. The shim is *federation*-shaped, not client-shaped.
- **Streaming** (`/api/v1/streaming`) — none.
- **Moderation / instance blocking / domain blocks** — none. The shim
  will accept a signed request from any keyId its resolver knows,
  regardless of hostname.
- **Actual Ruby / PostgreSQL / Redis behavior** — the shim is Rust and
  in-memory. Any Mastodon behavior that depends on the underlying tech
  stack (queue-processing order, PG-serialization anomalies, Ruby
  timezone quirks in Date headers, etc.) is a firm gap. This is the
  point of the §0 rule "the shim does not approximate what it can't
  faithfully model."

**Grounding note (fidelity honesty).** The shim's byte-level fidelity is
strongest at the specimen-anchored points (§1 rows 1–6). The specimens
themselves live under `tests/specimens/` and are the SINGLE SOURCE OF
TRUTH for wire shape; if a byte-level discrepancy against a real
Mastodon instance is later observed at the attended live leg (§4), the
resolution is to (a) update the specimen (recording the observation),
(b) update the shim to match, and (c) name what changed in the run
summary that observed the discrepancy. The specimens are dated and
versioned — the shim does not silently drift.

## 4. Attended live leg (recorded, not built)

Two shapes for the attended live leg:

1. **shim ↔ real Mastodon** — an attended session with a disposable Mastodon
   test instance (see: Mastodon `docker-compose.yml` on a real host).
   Predictions: outbound signed Follow from shim → Mastodon accepts and
   returns 202; inbound signed Follow from Mastodon → shim's
   `accept_inbox_post` returns `Ok(VerifiedActivity)` with the parsed
   `Follow`. Byte-level: the specimen in `tests/specimens/` matches the
   observed Mastodon output; any diff records as a specimen update.

2. **shim ↔ GoToSocial** — GoToSocial is a single-binary AP server with a
   smaller dep footprint than Mastodon and (per its own docs) commits to
   API compatibility with Mastodon at the federation surface. Same
   predictions.

Neither shape runs in this ephemeral in-env session; both are recorded as
attended runs.

## 5. Kinship

- **`ap-ambassador`** — uses fed-shim as its integration-test surface.
  Wire verification is ap-ambassador's `verify_ap_http_signature`,
  reused UNCHANGED (path-dep). Canonical dag-cbor / receipt-record
  types are also ap-ambassador's. fed-shim adds ONLY the wire-shape
  glue Mastodon uses (JSON activity bodies, PEM public key
  serialization, actor-document JSON-LD).
- **Sibling `xmtp-shim`** — the equivalent for the XMTP direction, if
  RUN-FS-01 (xmtp-ambassador) needs the same discipline. Not built here.
- **RUN-AP-01** — the fixture-leg tests (`ap-ambassador/tests/p1_verify.rs`)
  drive `verify_ap_http_signature` with fixture headers. A shim-leg
  companion test set (`ap-ambassador/tests/shim_roundtrip.rs`, landing
  with fed-shim) drives the SAME verify function with fed-shim's
  signed-request output. Grade stays `Modeled`; the SHIM is a more
  realistic wire source than the fixture path.
- **RUN-AP-02** (outbound delivery mechanics, AP-OC-6) — will consume
  fed-shim's `build_inbox_post` output as the delivery layer's input,
  then wrap in a retry queue / job runner / etc. Fed-shim is the
  wire-source; RUN-AP-02 is the delivery-plane.

## 6. Declared stand-ins

- **Fixture RSA keypairs** — deterministic-seed keys via `rand_chacha`
  seeded from `blake3(seed_id)`. Real actor keys in a live leg are
  server-generated (typically 2048-bit RSA). The shim generates
  1024-bit keys for test speed; the verify surface is size-agnostic
  (§1 row 1). Declared here so the grade honesty is visible.
- **Static `Date` header** — the shim takes the Date header value as a
  parameter, so tests can pass any fixed value. Real Mastodon uses
  wall-clock. The verify path checks Date's presence but not its
  value against a clock (there is no clock skew tolerance to
  configure); a real deployment would gate on freshness at a layer
  above.
- **In-memory `KeyResolver`** — same shape as `ap-ambassador::verify::
  KeyResolver`. The shim does not fetch actor documents over the
  network; test setup pre-populates the resolver with the shim
  actors' keys. Live-leg territory replaces this with a real HTTP
  fetcher.

## 7. Grade

This run's grade for the shim itself is `Modeled` — the byte-wire is
specimen-anchored, but real-Mastodon round-trip is the attended live
leg (§4). Downstream tests that use the shim inherit `Modeled` at
their own layer; the SHIM does not upgrade a downstream `Modeled` to
`Verified` on its own (that upgrade is the attended live leg's job).
