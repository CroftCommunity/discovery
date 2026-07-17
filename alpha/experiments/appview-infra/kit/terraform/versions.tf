terraform {
  required_version = ">= 1.6.0"

  required_providers {
    ovh = {
      source  = "ovh/ovh"
      version = "~> 1.5"
    }
  }

  # No remote state (non-goal §6): state stays local and is gitignored
  # (*.tfstate*). This is single-operator infra; a shared backend is scope
  # the runbook explicitly declines.
}

provider "ovh" {
  endpoint = var.ovh_endpoint
  # Credentials come from the environment (guardrail 4):
  #   OVH_APPLICATION_KEY / OVH_APPLICATION_SECRET / OVH_CONSUMER_KEY
  # Never set here.
}
