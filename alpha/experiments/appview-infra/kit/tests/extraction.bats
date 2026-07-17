#!/usr/bin/env bats
# D15 — the extraction that produces the standalone CroftCommunity/appview-infra
# from the kit/ subtree: root = kit contents, a generated PROVENANCE.md, corpus
# files excluded, and `make check` green inside the extracted tree standalone.

load helpers

@test "extract-to-repo.sh is shellcheck-clean" {
  run shellcheck "$KIT_ROOT/scripts/extract-to-repo.sh"
  if [ "$status" -ne 0 ]; then echo "$output"; fi
  [ "$status" -eq 0 ]
}

@test "extraction produces the expected tree with PROVENANCE.md and no corpus files" {
  T="$(mktemp -d)"
  run "$KIT_ROOT/scripts/extract-to-repo.sh" "$T/appview-infra"
  echo "$output"
  [ "$status" -eq 0 ]
  for e in Makefile CONTRACT.md scripts services generated terraform bootstrap \
           config-templates stub drill docs tests .github PROVENANCE.md \
           README.md .gitignore; do
    [ -e "$T/appview-infra/$e" ] || { echo "missing: $e"; false; }
  done
  # corpus stays in discovery — excluded from the extraction
  [ ! -e "$T/appview-infra/GROUPS.md" ]
  [ ! -e "$T/appview-infra/corpus-tests" ]
  [ ! -e "$T/appview-infra/RUN-15-SUMMARY.md" ]
  # PROVENANCE points back and names the corpus that stays
  grep -q 'CroftCommunity/discovery' "$T/appview-infra/PROVENANCE.md"
  grep -qi 'GROUPS.md' "$T/appview-infra/PROVENANCE.md"
  # README has a pointer-back section
  grep -qi 'discovery' "$T/appview-infra/README.md"
  rm -rf "$T"
}

@test "make check passes inside the extracted tree standalone" {
  T="$(mktemp -d)"
  "$KIT_ROOT/scripts/extract-to-repo.sh" "$T/appview-infra" >/dev/null
  run make -C "$T/appview-infra" check
  if [ "$status" -ne 0 ]; then echo "$output" | tail -30; fi
  [ "$status" -eq 0 ]
  rm -rf "$T"
}

@test "EXTRACTION.md documents the process and link-checks" {
  [ -f "$KIT_ROOT/docs/EXTRACTION.md" ]
  run "$KIT_ROOT/scripts/link-check.sh" "$KIT_ROOT/docs/EXTRACTION.md"
  [ "$status" -eq 0 ]
}
