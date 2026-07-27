# auth-helper spike — Stage A flow spec (confidential atproto OAuth client)

date: 2026-07-24 · status: **Stage A gate artifact**. Every request/response field below is cited to
the atproto OAuth spec, a named RFC, or the working public-client code already in this repo. No field
is "likely follows this pattern": anything the spec extract did not pin exactly is listed under
**Verify-in-run** (§8), not asserted.

Plan: `../../alpha/plans/croft-stack/07-auth-helper.md` (roadmap: `../../alpha/plans/croft-stack/README.md`).
Findings (outcomes) will land in `FINDINGS.md`; this file is the grounding it is measured against.

## 0. What is proven vs. what this spike invents

- **Proven, reused verbatim** — the *public* client legs: identity resolution, PKCE, PAR, DPoP proofs,
  token exchange, rotating refresh. Live-verified in skylite and ported into
  `croft-pwa/src/atproto/oauth/{client,dpop,pkce,resolve,jose}.ts`. This spike does **not** reimplement
  these; it extends them.
- **Proven, reused** — DID→key resolution against the live PLC directory (secp256k1 **or** p256),
  `appview-validation/src/serviceauth.rs` P-A3 (RUN-14). Not on the OAuth login path, but it is the
  same identity substrate.
- **Invention needing live confirmation** — the *confidential*-client deltas: `private_key_jwt` client
  authentication, a hosted `jwks`, a signed **client-assertion** JWT presented at the authenticated
  endpoints, a server-held private key, and **server-side refresh with no browser** past the
  public-client TTL. `authserve.rs` explicitly named the interactive OAuth login leg a non-goal it
  could not run (`bin/authserve.rs:16-17`). That leg is exactly what this spike runs.

## 1. Identity resolution chain (unchanged from public client — cited to code)

Same for public and confidential clients; the client type only changes how we *authenticate to* the
endpoints this chain discovers.

```
handle ──resolveHandle──▶ DID ──resolvePds──▶ PDS
   com.atproto.identity.resolveHandle          DID doc `#atproto_pds` service endpoint
   (read.ts:57)                                 (read.ts resolvePds)
        │
        ▼
PDS /.well-known/oauth-protected-resource ──▶ authorization_servers[0]   (resolve.ts:39-46)
        │
        ▼
authServer /.well-known/oauth-authorization-server ──▶ AuthServerMeta    (resolve.ts:48-65)
   { issuer, authorization_endpoint, token_endpoint,
     pushed_authorization_request_endpoint }
```

The `issuer` field returned here is load-bearing for the confidential client: it is the **`aud` of the
client-assertion JWT** (§5). Cited: `croft-pwa/src/atproto/oauth/resolve.ts:9-14,48-65`;
`croft-pwa/src/atproto/read.ts:56-59`.

## 2. Public-client baseline (what we start from — cited to code)

`croft-pwa/client-metadata.json` today:

```json
{ "client_id": "https://croftcommunity.github.io/croft-pwa/client-metadata.json",
  "redirect_uris": ["https://croftcommunity.github.io/croft-pwa/atproto.html"],
  "scope": "atproto transition:generic",
  "grant_types": ["authorization_code", "refresh_token"],
  "response_types": ["code"],
  "token_endpoint_auth_method": "none",       ◀── PUBLIC: no client authentication
  "application_type": "web",
  "dpop_bound_access_tokens": true }
```

The public flow (`client.ts`): `beginAuthorization` runs a DPoP-signed PAR (`client.ts:124-138`),
returns the authorize URL carrying `client_id` + `request_uri` (`client.ts:144-146`);
`completeAuthorization` exchanges the code at the token endpoint with a DPoP proof + `code_verifier`
(`client.ts:175-186`); `refresh` posts `grant_type=refresh_token` with the rotating refresh token
(`client.ts:214-239`). **No `client_assertion` anywhere** — auth method is `none`.

## 3. Confidential-client deltas (each cited to the atproto OAuth spec)

Source: atproto OAuth spec, https://atproto.com/specs/oauth (fetched 2026-07-24).

| # | Field / behavior | Public (today) | Confidential (this spike) | Citation |
|---|---|---|---|---|
| D1 | `token_endpoint_auth_method` | `"none"` | **`"private_key_jwt"`** (required for confidential) | spec: *"confidential clients must set this to `private_key_jwt`"* |
| D2 | public key publication | — | **`jwks`** (inline JWK array) **or** `jwks_uri`, **not both** | spec: *"Either this field or the `jwks_uri` field must be provided for confidential clients, but not both"* |
| D3 | key algorithm | ES256 (DPoP) | **ES256** for the client-assertion signing key | spec: *"Clients and Authorization Servers currently must support the `ES256` cryptographic system"* |
| D4 | token/PAR requests | DPoP proof only | DPoP proof **+** a `client_assertion` (§5) | spec: private_key_jwt + RFC 7523 |
| D5 | private key | none | **server-held**, `Zeroize` material, never logged/committed/serialized in clear | plan "secrets addendum"; task constraint |
| D6 | `redirect_uris` | GitHub Pages URL | **HTTPS URL under the helper's control** (OVH `/callback`) | spec: *"redirect_uri is a HTTPS URL … must match one of the URIs declared in the client metadata"* |
| D7 | `client_id` | hosted metadata URL | **hosted metadata URL over HTTPS**, no port (except localhost) | spec: *"client_id must be a fully-qualified web URL from which the client-metadata JSON document can be fetched"* |
| D8 | refresh-token TTL | ≤ 2 weeks | individual refresh token ≤ **180 days**; **overall session lifetime may be unlimited** | spec: *"for confidential clients, the overall session lifetime may be unlimited"* / *"Individual refresh tokens should have a lifetime limited to 180 days"* |
| D9 | PAR | required | **still required** (all client types) | spec: *"Pushed Authorization Requests (PAR) are required for all client types"* |
| D10 | DPoP | required | **still required** — assertion is *in addition to* DPoP, not instead of | spec: `dpop_bound_access_tokens: true`; RFC 9449 |

**The value claim this spike must confirm (D8):** a public client's session dies at ~2 weeks; a
confidential client's session lifetime "may be unlimited" with 180-day refresh tokens. Whether a given
PDS/entryway *actually* issues those lifetimes is a server policy — measuring the real numbers on the
test account's PDS is Stage D, and it is what retires Open decision 9 (which currently has NO FACTCHECK
citation for the "~2 week" number).

## 4. Confidential client-metadata.json (target shape)

```json
{ "client_id": "https://<helper-fqdn>/client-metadata.json",
  "client_name": "Croft Auth Helper (spike)",
  "client_uri": "https://<helper-fqdn>/",
  "redirect_uris": ["https://<helper-fqdn>/callback"],
  "scope": "atproto transition:generic",
  "grant_types": ["authorization_code", "refresh_token"],
  "response_types": ["code"],
  "token_endpoint_auth_method": "private_key_jwt",   ◀── D1
  "application_type": "web",
  "dpop_bound_access_tokens": true,                   ◀── D10
  "jwks_uri": "https://<helper-fqdn>/jwks.json" }     ◀── D2 (jwks_uri form)
```

`jwks.json` publishes only the **public** half of the ES256 signing key:

```json
{ "keys": [ { "kty": "EC", "crv": "P-256", "x": "…", "y": "…",
              "use": "sig", "alg": "ES256", "kid": "<kid>" } ] }
```

`kid` must match the client-assertion JWT header `kid` (§5) so the AS selects the right key.
`use`/`alg`/`kid` presence is standard JWKS but was **not** pinned by the spec extract → Verify-in-run
§8-a.

## 5. The client-assertion JWT (the core new artifact)

Presented at every endpoint that authenticates the client. Type and shape per **RFC 7523 §2.2 / §3**
(JWT client authentication, `private_key_jwt`), audience per the atproto spec.

Form parameters added to the POST body (alongside the existing grant params):

```
client_assertion_type = urn:ietf:params:oauth:client-assertion-type:jwt-bearer   (RFC 7523 §2.2)
client_assertion      = <the signed JWT below>
```

JWT header:

```json
{ "typ": "JWT", "alg": "ES256", "kid": "<kid matching jwks.json>" }
```

JWT claims:

| claim | value | source |
|---|---|---|
| `iss` | the `client_id` (metadata URL) | RFC 7523 §3: for private_key_jwt, iss = sub = client_id |
| `sub` | the `client_id` (metadata URL) | RFC 7523 §3 |
| `aud` | the authorization server's **`issuer`** (from §1 metadata) | atproto spec: *"The `aud` claim … must be the Authorization Server's `issuer`"* |
| `jti` | unique per assertion (replay prevention) | atproto spec: *"Must include `jti` … for replay prevention"* |
| `iat` | issued-at, unix seconds | atproto spec: *"Must include `iat`"* |
| `exp` | short expiry (recommend iat+60s) | RFC 7523 §3 requires `exp`; atproto extract did not restate → Verify-in-run §8-b |

Signed with the **server-held private key** whose public half is in `jwks.json`. This is a *different*
key from the per-session DPoP key: the DPoP key proves possession of the token (RFC 9449, unchanged,
`dpop.ts`); the assertion key proves *which client* is calling (RFC 7523, new). Both appear on the same
token request.

## 6. End-to-end sequence (confidential deltas marked ◀)

```
                         HUMAN opens authorize URL in a browser, authorizes, PDS redirects to /callback
                                                        │
 helper backend (OVH, holds private key)                │ browser
 ───────────────────────────────────────                │
 1. resolve handle→DID→PDS→authServer meta      §1       │
 2. PKCE verifier+challenge (S256)              pkce.ts   │
 3. generate per-session DPoP key               dpop.ts   │
 4. PAR  POST authorization_request_endpoint             │
      body: client_id,response_type=code,redirect_uri,   │
            scope,state,code_challenge(S256),login_hint   │
      + DPoP proof header                        client.ts:124-138
      + client_assertion(_type)      ◀ D4/§5   (PAR auth — Verify-in-run §8-c)
      ← { request_uri }
 5. build authorize URL: client_id + request_uri  client.ts:144-146
    ───────────────────────────────────────────────────▶ PRINT URL, STOP (Stage C gate)
                                                        HUMAN authorizes ─▶ redirect
 6. /callback receives { code, state }  ◀ D6 (our redirect_uri)
    verify state == pending.state              client.ts:171
 7. token exchange  POST token_endpoint
      body: grant_type=authorization_code, code,
            redirect_uri, client_id, code_verifier   client.ts:175-186
      + DPoP proof header (use_dpop_nonce retry)   client.ts:100-108
      + client_assertion(_type)      ◀ D4/§5
      ← { access_token, refresh_token, expires_in, sub, token_type=DPoP }
      verify sub == DID                          client.ts:192-193
 8. store session (encrypted at rest)  ◀ D5      Stage C gate: live DPoP-bound session
 ───────────────────────────────────────────────────────────────────────────────────
 9. SERVER-SIDE refresh (NO browser)  ◀ the thing being proven
      POST token_endpoint
      body: grant_type=refresh_token, refresh_token, client_id   client.ts:220-225
      + DPoP proof (carry dpopNonce)             client.ts:225
      + client_assertion(_type)      ◀ D4/§5
      ← { access_token, refresh_token', expires_in }  (refresh token ROTATES — single use)
      replace stored session                     client.ts:232-238  Stage D gate: measured TTLs
```

Response field names (`access_token`, `refresh_token`, `expires_in`, `sub`, `request_uri`, `error`,
`DPoP-Nonce` header, `use_dpop_nonce`) are all confirmed against the working public-client code
(`client.ts:139,187,192,201,206,101,104`), which is live-verified. The confidential path adds the two
`client_assertion*` form fields and nothing else to the request bodies.

## 7. Where the private key lives (secrets discipline — D5)

- Generated once (ES256, P-256). Private half → an env var **or** a mode-0600 file **outside the repo**
  on the OVH box. Public half → `jwks.json` served by the helper.
- Never logged, never printed, never committed, never serialized in the clear. Zeroize-equivalent
  handling in memory. (Task constraint + plan secrets addendum.)
- Session store (access+refresh tokens, DPoP private JWK) encrypted at rest.

## 8. Verify-in-run items (spec did not pin these exactly — do NOT assume)

- **§8-a — JWKS member requirements.** `use:"sig"`, `alg:"ES256"`, `kid` presence/format not restated
  by the atproto extract. Publish them (standard JWKS) and confirm the AS accepts the assertion; if it
  rejects on a missing/duplicate member, record it.
- **§8-b — assertion `exp`.** RFC 7523 §3 requires `exp`; atproto extract did not restate a max skew.
  Use iat+60s; confirm the live AS accepts it (some reject assertions with too-long `exp`).
- **§8-c — client authentication at the PAR endpoint.** The spec extract states client auth for the
  *token* endpoint; whether the *PAR* endpoint also demands the `client_assertion` for confidential
  clients is not explicitly quoted. RFC 9126 implies yes. Confirm live: try PAR with the assertion; if
  the AS 401s without it or 400s with it, record which.
- **§8-d — real refresh-token TTL + rotation.** D8 gives spec *ceilings* (180d / unlimited session),
  not what the test account's PDS actually issues. Measure `expires_in`, whether `refresh_token`
  rotates on every refresh (public client already rotates — `client.ts:213`), and how long an
  unattended session survives. This is the Stage D deliverable.
- **§8-e — public-vs-confidential delta on the SAME account.** Log the same account with the public
  croft-pwa client and measure its TTLs, to produce the real delta table (Stage D).

## 9. Citations index

- atproto OAuth spec — https://atproto.com/specs/oauth (fetched 2026-07-24): D1, D2, D3, D6, D7, D8,
  D9; assertion `aud`, `jti`, `iat`.
- RFC 7523 (JWT client auth / assertion) — `client_assertion_type`, `iss=sub=client_id`, `exp`.
- RFC 9449 (DPoP) — D10, proof shape (`dpop.ts`).
- RFC 9126 (PAR) — §8-c.
- Working public-client code (live-verified via skylite): `croft-pwa/src/atproto/oauth/*.ts`,
  `croft-pwa/src/atproto/read.ts`, `croft-pwa/client-metadata.json`.
- DID→key resolution proven live: `appview-validation/src/serviceauth.rs` (P-A3, RUN-14).
- OAuth-login-leg non-goal on record: `appview-validation/src/bin/authserve.rs:16-17`.
</content>
</invoke>
