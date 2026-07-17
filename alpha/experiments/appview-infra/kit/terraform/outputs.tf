output "resolved_plan_code" {
  description = "Plan code resolved against the live catalog."
  value       = data.ovh_order_cart_product_plan.vps.plan_code
}

output "resolved_price" {
  description = "First resolved price for the plan (sanity vs catalog-vps.sh)."
  value       = try(data.ovh_order_cart_product_plan.vps.selected_price[0].price_in_ucents, null)
}

output "vps_service_name" {
  description = "Delivered VPS service name (null until read in Phase 2)."
  value       = var.vps_service_name == "" ? null : data.ovh_vps.this[0].service_name
}

output "vps_ipv4" {
  description = "Delivered VPS IPv4 (null until read in Phase 2)."
  value       = var.vps_service_name == "" ? null : try(data.ovh_vps.this[0].ips[0], null)
}

output "vps_all_ips" {
  description = "All IPs on the delivered VPS (null until read in Phase 2)."
  value       = var.vps_service_name == "" ? null : try(data.ovh_vps.this[0].ips, null)
}
