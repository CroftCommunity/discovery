# appview-infra — hosting kit for the Croft portfolio's small always-on services.
#
# The manifests in services/*.toml plus the generator (scripts/render.sh) are the
# single source of truth. Everything under generated/ is produced from them; do
# NOT hand-edit generated files. `make check` is the whole gate.
#
# This Makefile is written as the future root of CroftCommunity/appview-infra
# (D16 extracts the kit/ subtree). It has no dependency on the discovery repo.

SHELL := /bin/bash
BATS  ?= bats

# Every check sub-target is aggregated here. Each deliverable adds its own.
CHECKS := no-secrets stub render terraform backup-audit bootstrap deploy

.PHONY: check $(addprefix check-,$(CHECKS)) generate clean help

help:
	@echo "targets: check | generate | clean"
	@echo "check runs: $(CHECKS)"

check: $(addprefix check-,$(CHECKS))
	@echo "== make check: all sub-checks passed =="

# --- D1: hygiene ------------------------------------------------------------
check-no-secrets:
	@scripts/no-secrets-check.sh

# --- D2: contract stub ------------------------------------------------------
check-stub:
	@$(BATS) tests/stub_contract.bats

# --- D3: manifests + generator ----------------------------------------------
check-render:
	@$(BATS) tests/render.bats

# --- D4: terraform ----------------------------------------------------------
check-terraform:
	@$(BATS) tests/terraform.bats

# --- D5: backup-audit invariant ---------------------------------------------
check-backup-audit:
	@$(BATS) tests/backup_audit.bats

# --- D7: bootstrap ----------------------------------------------------------
check-bootstrap:
	@$(BATS) tests/bootstrap.bats

# --- D8: deploy pipeline ----------------------------------------------------
check-deploy:
	@$(BATS) tests/deploy.bats

# --- generation -------------------------------------------------------------
generate:
	@scripts/render.sh

clean:
	@echo "generated/ is a build product; regenerate with 'make generate'"
