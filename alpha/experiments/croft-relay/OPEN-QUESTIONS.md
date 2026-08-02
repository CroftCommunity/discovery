# croft-relay — open questions for the human

Per the build plan §7. Defaults were chosen so this run did not block; each is
flagged here for a decision. Status tokens: DEFAULTED (built to the default),
UNDECIDED (needs a call before the dependent phase).

## Q1 — Token format. DEFAULTED: JWT / EdDSA (ed25519).

Built to the plan default: JWT, EdDSA, claims `sub`/`tier`/`iss`/`iat`/`exp`
(ADR-0003). The three-gate verification (signature / expiry / bound-id) is
format-independent, so the alternative (raw ed25519 over canonical CBOR) is a
contained swap if you prefer to drop the JWT dependency. **Decision needed
before Phase 2 hardening / upstream packaging.**

## Q2 — Repo shape. DEFAULTED: iroh pinned by tag, core is relay-agnostic.

This spike lives in `discovery/alpha/experiments/croft-relay/` (house
convention). The built crate `croft-admit` has **no** iroh dependency; the
recon depended on `iroh-relay 1.0.0-rc.1` read from the crates.io registry
(ADR-0001). If/when this graduates to a standalone `CroftCommunity/croft-relay`
repo, the only iroh dependency is the thin `AccessControl` adapter. **Confirm
the standalone-repo intent, or keep it folded here.**

## Q3 — Coordination-tier product stance. DEFAULTED: hard cap (recommended).

The plan recommends, and this build assumes, a hard cap that starves sustained
relayed media on the coordination tier (a failed holepunch there is a *tier
limitation*, to be surfaced by the app as such — not a bug). The alternative
(allow a trickle) is a config change to the coordination bucket, not a code
change. **Confirm hard-cap, or specify the trickle allowance.**

## Q4 — Ship Phase 1 to relay.croft.ing before Phase 2? UNDECIDED.

Phase 1 (registered-only via the stock relay's HTTP hook + `croft-admit`'s
`/access` endpoint) works against the stock upstream binary with zero relay
changes and could deploy now. Phase 2 (stateless signed tokens) supersedes the
per-connection database call. **Deploy Phase 1 first, or hold for tokens?**
Note: the live end-to-end proof (relay + two endpoints + holepunch) has NOT
been run here — see the README "Deferred" section — so any deploy should follow
that verification.

## Q5 — Metrics label cardinality. DEFAULTED: tier-level aggregates only.

The plan assumes tier-level aggregates (admissions by outcome/tier, active
connections by tier, bytes by tier, saturation events) rather than
per-`EndpointId` labels — a cardinality and privacy trade. Phase 4 (metrics) is
not built yet; recording the default so it is a conscious choice when it is.
**Confirm tier-level only, or authorize endpoint-level labels.**

## Also surfaced during recon (not in §7)

- **Header-name discrepancy (decided in-code):** iroh-relay 1.0.0-rc.1 sends
  `X-Iroh-NodeId` on the HTTP hook, though its own doc-comment says
  `X-Iroh-Endpoint-Id`. `croft-admit` reads the real header and accepts the
  documented alias (ADR-0001, `http_api.rs`). No decision needed; noted so a
  future iroh rename is a known quantity.
