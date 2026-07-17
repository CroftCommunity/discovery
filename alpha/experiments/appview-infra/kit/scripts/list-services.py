#!/usr/bin/env python3
"""list-services.py — emit a stable, shell-consumable digest of the manifests.

One TSV line per service:
  name <TAB> user <TAB> api_user <TAB> port <TAB> api_port <TAB> artifact <TAB> api_mode
api_user/api_port are "-" when serve_api is false. Used by bootstrap.sh and the
local rehearsal so they never re-parse TOML in shell.
"""
from __future__ import annotations

import glob
import os
import sys
import tomllib


def main():
    services_dir = sys.argv[1] if len(sys.argv) > 1 else "services"
    rows = []
    for fn in sorted(glob.glob(os.path.join(services_dir, "*.toml"))):
        if os.path.basename(fn).startswith("_"):
            continue
        with open(fn, "rb") as f:
            m = tomllib.load(f)
        name = m["name"]
        serve_api = m.get("serve_api", False)
        rows.append("\t".join([
            name,
            name,
            f"{name}-api" if serve_api else "-",
            str(m["port"]),
            str(m["api_port"]) if serve_api else "-",
            m.get("artifact", "stub"),
            m.get("api_mode", "shared-wal") if serve_api else "-",
        ]))
    sys.stdout.write("\n".join(rows) + ("\n" if rows else ""))


if __name__ == "__main__":
    main()
