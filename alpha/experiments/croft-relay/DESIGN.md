# croft-relay — design

## The shape of the system

```
  caller app                  enrollment service            relay (iroh-relay)
 ┌──────────┐   handle→DID    ┌───────────────┐   token    ┌────────────────┐
 │ resolve  │───→ PDS record  │  croft-admit  │  (JWT/Ed)  │ AccessControl  │
 │ EndpointId ─────────────→  │  verify DID   │───────────→│  on_connect →  │
 │  dial    │                 │  control,     │            │  Allow / Deny  │
 └────┬─────┘                 │  mint token   │            └───────┬────────┘
      │ attach (token)        └───────────────┘                    │ Bucket
      └──────────────────────────────────────────────────────────→│ per tier
```

- **Identity** lives in atproto. `handle → DID → PDS record
  (ing.croft.iroh.endpoint, rkey self) → EndpointId`.
- **Transport** is iroh. The relay carries first contact; holepunch upgrades to
  direct QUIC when possible.
- **`croft-admit`** (this crate) is the app-side authority: it proves
  DID-control at enrollment, mints capability tokens, and maps a token's tier
  to a rate bucket. It is relay-agnostic; the relay reaches it either via the
  HTTP hook (Phase 1) or via an embedded `AccessControl` adapter (Phase 2).

## Trust model (why each gate exists)

1. **Enrollment binds identity to endpoint.** Only the DID owner can write the
   PDS record naming an EndpointId, so a matching record proves control.
   Deny-closed: no proof, no binding (ADR-0002).
2. **The relay authenticates the endpoint key.** By the time admission runs,
   the connecting EndpointId is cryptographically proven, not asserted
   (verified in iroh-relay source, ADR-0001).
3. **The token binds a capability to that endpoint.** `sub` == the
   authenticated id is checked at verify time, so a stolen token is worthless
   from any other endpoint (ADR-0003). Signature binds it to our enrollment
   key; expiry bounds its lifetime; there is no revocation list because there
   is no long-lived token to revoke.
4. **The tier bucket meters volume, never content.** The relay cannot see
   inside encrypted frames and must not — so coordination-vs-media is split by
   rate, the only honest proxy (ADR-0004).

## What the relay never does

- It never learns call content (end-to-end encrypted above it).
- It never holds social-graph state (mutuals-only etc. is app-layer, above the
  relay, and out of scope for v1).
- It never inspects `(src, dst)` in the forwarding path (deferred by design).

## Module map

| module | role | phase |
|---|---|---|
| `endpoint_id` | 32-byte ed25519 id; hex wire form | all |
| `did` | atproto DID newtype + validation | 1 |
| `pds` | fetch `ing.croft.iroh.endpoint`; `PdsResolver` trait | 1 |
| `registry` | `EndpointId -> Did` bindings | 1 |
| `enroll` | prove DID-control, bind (deny-closed) | 1 |
| `access` | pure access decision (deny-closed) | 1 |
| `http_api` | axum `/access` service (the relay's hook contract) | 1 |
| `token` | JWT/EdDSA mint + three-gate verify | 2 |
| `tier` | tier -> `RateBucket` mapping | 3 |

## Determinism as a testing choice

The core reads no clock and no randomness: `now` is injected into mint/verify,
endpoint ids come from explicit bytes, PDS answers come from a resolver. This is
what lets the deny matrix (expiry, clock-skew boundaries) and the mutation gate
be deterministic. The only wall-clock/random reads live at the edges (the HTTP
handler; test keygen).
