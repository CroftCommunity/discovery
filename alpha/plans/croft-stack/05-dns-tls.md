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

## The records (by hand; the box is `A 15.204.81.133`, `AAAA 2604:2dc0:222::431` — confirmed post-reimage)

| fqdn | Zone | Added at | Status |
|---|---|---|---|
| `account.croft.ing` | croft.ing (Porkbun) | Phase 4/7 | **exists** (spike); re-point after reimage |
| `canary.croft.ing` | croft.ing (Porkbun) | Phase 4 | new |
| `relay.croft.ing` | croft.ing (Porkbun) | Phase 6 | new (same A record; relay's UDP/QUIC is same host) |
| `skylite-cache.croft.ing` | croft.ing (Porkbun) | Phase 8 | new |
| `cache.arecipe.app` | arecipe.app | Phase 8 | new (one record on arecipe.app's zone) |
| `index.stellin.app` | stellin.app | Phase 9 | new (`stellin.app` apex already → box from spike) |

Steps: add each record by hand as its service lands (not all at once); confirm Caddy auto-issues a valid
cert per fqdn (`https://<fqdn>/healthz`); leave the `croft.ing` apex and the GitHub Pages pads untouched.
**No DNS automation, no Porkbun API/token changes** — records only (Open decision 10).

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
