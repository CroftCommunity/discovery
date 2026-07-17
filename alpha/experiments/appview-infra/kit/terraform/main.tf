# appview-infra — one always-on OVH VPS, Debian 12, single-operator.
#
# SCOPE SPLIT (money gate, guardrail 6): discovery models plan RESOLUTION and
# reads; the actual paid ORDER is a Phase-2 step (P2-2), run from the extracted
# repo with explicit owner go and live prices shown first. The order resource is
# therefore kept as a clearly-marked block enabled only when var.place_order is
# set true — so nothing here can spend money by accident.
#
# terraform validate confirms the data-source schema when the provider registry
# is reachable; in discovery that registry is BLOCKED (github 403), so validate
# is reported BLOCKED and the schema is re-confirmed in Phase 2.
# SPEC-DELTA[run15-tf-validate | stand-in]

# --- plan resolution (free, read-only) --------------------------------------
data "ovh_order_cart" "cart" {
  ovh_subsidiary = var.ovh_subsidiary
}

data "ovh_order_cart_product_plan" "vps" {
  cart_id        = data.ovh_order_cart.cart.id
  price_capacity = "renew"
  product        = "vps"
  plan_code      = var.plan_code
}

# --- the paid order (Phase 2 only; place_order=false by default) -------------
# Standard daily backup is included free on current-generation VPS, so no backup
# option is ordered here. Premium 7-day backup is one additional plan block —
# an owner call, added in Phase 2 if wanted.
resource "ovh_order_cart_item_configuration" "os" {
  count = var.place_order ? 1 : 0

  cart_id = data.ovh_order_cart.cart.id
  item_id = ovh_order_cart_item.vps[0].id
  label   = "os"
  value   = var.os_image
}

resource "ovh_order_cart_item" "vps" {
  count = var.place_order ? 1 : 0

  cart_id      = data.ovh_order_cart.cart.id
  product_id   = data.ovh_order_cart_product_plan.vps.product_id
  plan_code    = var.plan_code
  pricing_mode = "default"
  duration     = data.ovh_order_cart_product_plan.vps.selected_price[0].duration
  quantity     = 1
}

# --- reading a delivered VPS for outputs (Phase 2, post-delivery) ------------
# Enabled once the VPS service name is known (var.vps_service_name). Keeps a
# fresh plan/apply from trying to read a VPS that does not exist yet.
data "ovh_vps" "this" {
  count        = var.vps_service_name == "" ? 0 : 1
  service_name = var.vps_service_name
}
