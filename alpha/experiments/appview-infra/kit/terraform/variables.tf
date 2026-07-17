# Owner-decision variables carry NO default (guardrail 5): Terraform must refuse
# to run until the owner supplies them, so nothing is silently guessed.

variable "ovh_endpoint" {
  description = "OVH API endpoint. ovh-us is a SEPARATE subsidiary/account."
  type        = string
  validation {
    condition     = contains(["ovh-eu", "ovh-ca", "ovh-us"], var.ovh_endpoint)
    error_message = "ovh_endpoint must be one of: ovh-eu, ovh-ca, ovh-us."
  }
}

variable "ovh_subsidiary" {
  description = "OVH subsidiary for the order cart (e.g. FR, CA, US). Owner call."
  type        = string
}

variable "datacenter" {
  description = "Target datacenter/region code (owner picks from the catalog)."
  type        = string
}

variable "plan_code" {
  description = <<-EOT
    VPS plan code from the LIVE catalog (scripts/catalog-vps.sh). Codes are
    undocumented and generation-specific — there is deliberately no default;
    the owner picks one from live prices in Phase 2.
  EOT
  type        = string
}

variable "ssh_public_key" {
  description = "SSH public key installed on the VPS for the operator."
  type        = string
}

variable "os_image" {
  description = "Base image. Debian 12 is the target; kept as a var for clarity."
  type        = string
  default     = "Debian 12"
}

variable "service_display_name" {
  description = "Human label for the ordered VPS."
  type        = string
  default     = "croft-appview"
}

variable "place_order" {
  description = <<-EOT
    Money gate (guardrail 6). false in discovery/Phase 1 — only plan resolution
    runs, nothing is purchased. Set true in Phase 2 with explicit owner go.
  EOT
  type        = bool
  default     = false
}

variable "vps_service_name" {
  description = <<-EOT
    Delivered VPS service name, known only after the order completes. Empty
    means "do not read a VPS yet" (outputs stay null). Set in Phase 2.
  EOT
  type        = string
  default     = ""
}
