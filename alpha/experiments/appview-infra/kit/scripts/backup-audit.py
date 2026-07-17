#!/usr/bin/env python3
"""backup-audit.py — the state taxonomy (§1.2) as executable law.

FAIL (exit 1) if:
  * any disposable path — including snapshot api serve/ dirs — is a backup
    target (appears in litestream.yml or an rclone unit), or
  * any canonical sqlite path lacks a litestream target, or
  * any blob dir lacks an rclone target.

Independent audit: reads the MANIFESTS for the intended classification and the
ACTUAL generated litestream.yml + rclone units for what is really backed up, so
a generator bug that mis-backs-up state is caught, not trusted away.
"""
from __future__ import annotations

import argparse
import glob
import os
import re
import sys
import tomllib


def load_manifests(services_dir):
    out = []
    for fn in sorted(glob.glob(os.path.join(services_dir, "*.toml"))):
        if os.path.basename(fn).startswith("_"):
            continue
        with open(fn, "rb") as f:
            m = tomllib.load(f)
        dp = m.get("data_profile", {})
        rec = {
            "name": m["name"],
            "canonical": list(dp.get("canonical", [])),
            "disposable": list(dp.get("disposable", [])),
            "blobs": list(dp.get("blobs", [])),
        }
        # snapshot api adds serve/ — derived, disposable, never backed up (§1.3)
        if m.get("serve_api") and m.get("api_mode") == "snapshot":
            rec["disposable"].append("serve/state.db")
        out.append(rec)
    return out


def litestream_db_paths(generated_dir):
    """Absolute db paths litestream actually replicates."""
    path = os.path.join(generated_dir, "litestream.yml")
    paths = set()
    if not os.path.exists(path):
        return paths
    with open(path, encoding="utf-8") as f:
        for line in f:
            m = re.match(r"\s*-\s*path:\s*(/\S+)", line)
            if m:
                paths.add(m.group(1))
    return paths


def rclone_sources(generated_dir):
    """Absolute source dirs rclone actually mirrors."""
    srcs = set()
    for unit in glob.glob(os.path.join(generated_dir, "systemd", "*-blob-*.service")):
        with open(unit, encoding="utf-8") as f:
            for line in f:
                m = re.search(r"ExecStart=\S*rclone\s+sync\s+(/\S+)", line)
                if m:
                    srcs.add(m.group(1).rstrip("/"))
    return srcs


def audit(services_dir, generated_dir):
    manifests = load_manifests(services_dir)
    ls_paths = litestream_db_paths(generated_dir)
    rc_srcs = rclone_sources(generated_dir)
    all_backup_targets = set(ls_paths)  # sqlite backups
    errors = []
    for m in manifests:
        dd = f"/var/lib/{m['name']}"
        for db in m["canonical"]:
            if f"{dd}/{db}" not in ls_paths:
                errors.append(
                    f"{m['name']}: canonical path '{db}' has NO litestream "
                    f"backup target (expected {dd}/{db} in litestream.yml)")
        for db in m["disposable"]:
            full = f"{dd}/{db}"
            if full in all_backup_targets:
                errors.append(
                    f"{m['name']}: disposable path '{db}' IS a litestream "
                    f"backup target — disposable state must NEVER be backed up")
            # a disposable dir (e.g. serve/) mirrored by rclone is also illegal
            base = full.rsplit("/", 1)[0] if "/" in db else full
            for src in rc_srcs:
                if src == full.rstrip("/") or src == base.rstrip("/"):
                    errors.append(
                        f"{m['name']}: disposable path '{db}' IS an rclone "
                        f"mirror target — disposable state must NEVER be backed up")
        for blobdir in m["blobs"]:
            full = f"{dd}/{blobdir}".rstrip("/")
            if full not in rc_srcs:
                errors.append(
                    f"{m['name']}: blob dir '{blobdir}' has NO rclone mirror "
                    f"target (expected {full})")
    return manifests, errors


def main():
    p = argparse.ArgumentParser(description="backup-audit — state taxonomy law")
    p.add_argument("--manifests-dir", default="services")
    p.add_argument("--generated-dir", default="generated")
    a = p.parse_args()
    manifests, errors = audit(a.manifests_dir, a.generated_dir)
    if errors:
        for e in errors:
            sys.stderr.write(f"backup-audit: FAIL: {e}\n")
        sys.stderr.write(f"backup-audit: {len(errors)} violation(s)\n")
        sys.exit(1)
    print(f"backup-audit: OK ({len(manifests)} service(s); taxonomy holds)")


if __name__ == "__main__":
    main()
