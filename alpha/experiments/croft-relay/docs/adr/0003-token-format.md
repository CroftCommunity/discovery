# ADR-0003: Signed per-endpoint token format and claim schema

- Status: Accepted (token default per plan §4; flag at review — see OPEN-QUESTIONS Q1)
- Date: 2026-08-02
- Phase: 2 (signed per-endpoint tokens)

## Context

Phase 2 replaces "the relay asks a database on every connection" with "the
relay verifies a signature and holds no state." The token is a capability
issued by the enrollment service after DID-control is proven; the relay admits
on signature alone, no network call in the attach path.

## Decision

### Format: JWT, EdDSA (ed25519)

Per the plan's default (§4). Boring, library-supported (`jsonwebtoken`),
auditable. `token.rs` holds the private signing key in `TokenIssuer` and only
the public key + trusted issuer string in `TokenVerifier` (no per-endpoint
state).

### Claim schema

| claim | meaning |
|---|---|
| `sub`  | hex `EndpointId` this token authorizes |
| `tier` | `coordination` \| `broker` (drives Phase 3 bucket) |
| `iss`  | enrollment service identifier |
| `iat`  | issued-at (unix seconds) |
| `exp`  | expiry (unix seconds) |

### Verification = three independent gates, all required

1. **Signature + algorithm** by the enrollment key (`jsonwebtoken::decode`
   with `Algorithm::EdDSA`). Covers wrong-issuer-key and tampering.
2. **Temporal validity** — `now <= exp + leeway` and `now >= iat - leeway`,
   with a symmetric clock-skew `leeway`.
3. **Bound id** — `sub` must equal the endpoint the relay *cryptographically
   authenticated* on this connection. This is the anti-replay hinge: a stolen
   token cannot be presented from a different endpoint.

A token without the enrollment key is inert; the key without a token admits
nothing; a valid token from the wrong endpoint is denied (`IdMismatch`).

### Determinism / clock injection

`jsonwebtoken`'s built-in `exp` validation is **disabled**; expiry is checked
against a caller-supplied `now`. Two reasons: (1) the deny matrix (expired,
clock-skew boundaries, future-dated) must be deterministic rather than
wall-clock-and-`sleep`; (2) expiry is exactly the logic the mutation gate must
not leave a survivor in, so it is explicit and local. The HTTP edge passes real
`SystemTime` seconds; the core never reads the clock.

### Revocation = short expiry + refusal to re-issue

There is no revocation list. Tokens are short-lived (minutes-to-an-hour, plan
§4) capabilities; to revoke, let the token expire and decline to re-issue.
Replay of a *still-valid* token by the *legitimate* endpoint is fine and
intended — tokens are capabilities, not nonces (tested:
`replay_within_expiry_by_legit_endpoint_admits`).

## Deny matrix (all tested — `tests/phase2_token.rs`)

| input | result |
|---|---|
| valid + matching id | admit (with tier) |
| valid + mismatched id | `IdMismatch` |
| expired (`now > exp + leeway`) | `Expired` |
| `now == exp + leeway` / `now == iat - leeway` | admit (boundary pinned) |
| future-dated beyond leeway | `NotYetValid` |
| wrong issuer key | `SignatureOrMalformed` |
| wrong `iss` claim | `WrongIssuer` |
| malformed / tampered | `SignatureOrMalformed` |
| replay within expiry, legit endpoint | admit |

`cargo mutants` leaves **no surviving mutant** in `token.rs`
(`evidence/mutants-full.txt`).

## Upstream shape

The upstream-candidate slice is a policy-free `signed_token` access mode:
"verify a bearer token against a configured ed25519 public key; require `sub`
== the authenticated endpoint id." No atproto, no tier, no Croft vocabulary —
those stay in `croft-admit`. `Tier` rides in our tokens as an extra claim the
generic verifier ignores.

## Flag for the human

Q1 in OPEN-QUESTIONS: confirm JWT/EdDSA vs the raw-ed25519-over-CBOR
alternative. Built to the JWT default; the three-gate structure is
format-independent if that changes.
