#!/usr/bin/env bash
# catalog-vps.sh — list LIVE OVH VPS plan codes and prices.
#
# VPS plan codes are undocumented and generation-specific; this is how the owner
# picks plan_code + datacenter in Phase 2 (P2-1). Read-only, no order, no spend.
# The public catalog endpoint needs no credentials.
#
# Usage: catalog-vps.sh [ovh-eu|ovh-ca|ovh-us]   (default ovh-eu)
set -euo pipefail

endpoint="${1:-ovh-eu}"
case "$endpoint" in
  ovh-eu) base="https://eu.api.ovh.com/1.0" ;;
  ovh-ca) base="https://ca.api.ovh.com/1.0" ;;
  ovh-us) base="https://api.us.ovhcloud.com/1.0" ;;  # separate subsidiary
  *) echo "unknown endpoint: $endpoint (want ovh-eu|ovh-ca|ovh-us)" >&2; exit 2 ;;
esac

# ovhSubsidiary drives the currency/prices shown.
sub="${OVH_SUBSIDIARY:-FR}"
url="$base/order/catalog/public/vps?ovhSubsidiary=$sub"

echo "# catalog: $url" >&2
json="$(curl -fsSL "$url")" || {
  echo "catalog fetch failed (endpoint $endpoint). Network/policy?" >&2
  exit 1
}

# Emit: planCode, its invoiceName, and the cheapest price (in the catalog's
# currency, converted from micro-units).
echo "$json" | jq -r '
  .plans[]
  | . as $p
  | ( [ $p.pricings[]?.price ] | min ) as $min
  | [ $p.planCode,
      ($p.invoiceName // "-"),
      (if $min == null then "-" else ($min/100000000 | tostring) end)
    ] | @tsv
' | sort | column -t -s $'\t' \
  || { echo "could not parse catalog JSON (schema drift?)" >&2; exit 1; }

echo
echo "# Pick a planCode + a datacenter for terraform (plan_code / datacenter)." >&2
echo "# Prices are catalog list prices in the ovhSubsidiary=$sub currency." >&2
