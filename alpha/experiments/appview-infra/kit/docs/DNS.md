# DNS (Porkbun, manual v1)

Porkbun holds only address records for the staging fqdns. There is no DNS
automation in v1 (non-goal §6): the owner creates a handful of records by hand
from the Terraform outputs. Records point at the one VPS.

## Records to create (per service and per api)

After `terraform apply`, read the VPS addresses from the outputs (see
[`../terraform/outputs.tf`](../terraform/outputs.tf): `vps_ipv4`, `vps_all_ips`).
Create, for each fqdn, an `A` record to the IPv4 and an `AAAA` record to the
IPv6:

| fqdn                              | type   | value            |
|-----------------------------------|--------|------------------|
| `stellin-staging.croft.ing`       | A/AAAA | VPS IPv4 / IPv6  |
| `api.stellin-staging.croft.ing`   | A/AAAA | VPS IPv4 / IPv6  |
| `groups-staging.croft.ing`        | A/AAAA | VPS IPv4 / IPv6  |
| `api.groups-staging.croft.ing`    | A/AAAA | VPS IPv4 / IPv6  |

Every tenant with `serve_api = true` needs both its service fqdn and its
`api.<fqdn>`. The set is exactly the Caddy vhosts under
[`../generated/caddy/`](../generated/caddy) — one record pair per file.

## TLS: no DNS API needed

ACME is **HTTP-01** (Caddy default): the box proves control of each fqdn over
port 80, so certificates issue with only the A/AAAA records above. No DNS-01,
no Porkbun API token, no CAA gymnastics. This is why v1 stays manual — the DNS
provider only has to serve address records.

## Pending, not created: `_lexicon` TXTs

atproto app namespaces may later want `_lexicon.<domain>` TXT records (lexicon
resolution). These are **pending per-app namespace decisions** and are NOT
created here — they wait on the namespace choices for each app (stellin,
croft-groups). Listed so a future session creates them deliberately, not by
guesswork.

## If automation is ever wanted

Porkbun has a DNS API. If v2 ever wants record automation (many tenants, or
DNS-01 wildcards), it can be wired then — out of scope for v1, and called out
as a non-goal in `RUNBOOK.md` (D14) so it is not added reflexively.
