#!/usr/bin/env bats
# P15-local — the fire drill against a REAL S3 API (local MinIO standing in for
# R2), exercising the actual s3:// litestream replica + rclone s3 remote code
# paths the file:// drill (D13) does not. Skips cleanly where MinIO is absent.
#
# SPEC-DELTA[run15-s3-local | stand-in]: local MinIO stands in for Cloudflare R2
# (same S3 API); Phase 1.5 swaps endpoint + credentials to real R2.

load helpers

@test "drill scripts are shellcheck-clean (s3 paths included)" {
  run shellcheck "$KIT_ROOT/drill/fire-drill.sh" "$KIT_ROOT/drill/lib.sh" \
    "$KIT_ROOT/drill/local-up.sh"
  if [ "$status" -ne 0 ]; then echo "$output"; fi
  [ "$status" -eq 0 ]
}

@test "fire-drill --variant s3-local: destroy/restore against MinIO passes" {
  if ! command -v minio >/dev/null 2>&1; then skip "minio not installed"; fi
  if ! command -v rclone >/dev/null 2>&1; then skip "rclone not installed"; fi
  run "$KIT_ROOT/drill/fire-drill.sh" --variant s3-local
  echo "$output"
  [ "$status" -eq 0 ]
  [[ "$output" == *"target: s3"* ]]        # actually used the s3 path
  [[ "$output" == *"DRILL PASS"* ]]
  [[ "$output" == *"stellin-appview"* ]]
  [[ "$output" == *"croft-groups"* ]]
  [[ "$output" == *"canonical marker"* ]]
  [[ "$output" == *"blob marker"* ]]
}
