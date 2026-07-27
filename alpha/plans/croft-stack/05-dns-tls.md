# Phase 5 — DNS + TLS

← [04-stub-bringup.md](04-stub-bringup.md) · [roadmap](README.md) · next →
[06-iroh-relay.md](06-iroh-relay.md)

**Status:** SCAFFOLD (fill on arrival) · **Depends-on:** Phase 4 (Caddy up on the box) ·
**Gate-out:** `https://<fqdn>/healthz` → `200 ok` with a valid cert for each fqdn we bring up.

---

## Problem

Each service needs a public fqdn resolving to the box, and TLS. No DNS automation — records are created
by hand (Porkbun); Caddy auto-issues certs on first request.

## Approach

Create A/AAAA per service fqdn pointing at the box; let Caddy's auto-HTTPS issue on first request.
Croft-domain services live under `croft.ing`; a pad on its own domain gets its cache subdomain under
*that* domain.

## Steps (sketch — fill on arrival)
1. A/AAAA for the stub fqdn and `account.croft.ing` (auth helper) and `relay.croft.ing` (relay) → box.
2. For arecipe's cache: A/AAAA for `cache.arecipe.app` → box (its own domain, same-site with the pad).
3. Confirm Caddy issues a valid cert per fqdn (`https://<fqdn>/healthz`).
4. Leave the `croft.ing` apex and the GitHub Pages pads untouched.

## TODO (decide on arrival)
- [ ] Final fqdn list once Q4 (stub) and Q5 (Stellin index name) land.
- [ ] Confirm arecipe.app DNS is reachable to us for the `cache.` subdomain.

## Risks & cautions
- ACME rate limits — do not thrash cert issuance during testing; use staging endpoints if iterating.
- `.app` is HSTS-preloaded (browser-forced HTTPS) — fine for `cache.arecipe.app` since Caddy serves
  real TLS, but note there is no HTTP fallback; `*.croft.ing` staging names are unaffected.

## Validation
Each fqdn: `https://<fqdn>/healthz` → `200 ok`, valid cert chain.

## References
Roadmap → naming scheme (Open decision 1); Caddy auto-HTTPS.
