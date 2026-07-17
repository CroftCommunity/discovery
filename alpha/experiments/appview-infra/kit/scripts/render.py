#!/usr/bin/env python3
"""render.py — the appview-infra generator.

Manifests (services/*.toml) + this generator are the single source of truth.
Everything under the out dir is a build product; hand-editing it is forbidden.

Emits, per manifest:
  systemd/<name>.service                 tenant service unit (hardened)
  systemd/<name>-api.service             own-data API sidecar (serve_api)
  systemd/<name>-snapshot.{service,timer} VACUUM INTO snapshot (api_mode snapshot)
  systemd/<name>-blob-<i>.{service,timer} rclone blob mirror per blob dir
  caddy/<fqdn>.caddy                     service vhost (HTTP-01)
  caddy/api.<fqdn>.caddy                 api vhost (serve_api)
and aggregate:
  litestream.yml                         one section per canonical sqlite
  backup-map.json                        state-taxonomy map for the D5 audit
  ports.json                             allocated ports
  GENERATED.md                           index of emitted files

Validates: port collisions across ALL service+api ports; canonical/disposable
overlap. Either aborts nonzero before writing anything.

Stdlib only (tomllib + string.Template). No secrets are ever written: bucket is
a non-secret render var (RENDER_BUCKET, staging default); endpoints/credentials
come from the runtime environment.
"""
from __future__ import annotations

import argparse
import json
import os
import string
import sys
import tomllib

BUCKET = os.environ.get("RENDER_BUCKET", "croft-appview-staging")
TMPL_DIR = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..",
                        "config-templates")


def die(msg: str) -> "NoReturn":  # noqa: F821
    sys.stderr.write(f"render: ERROR: {msg}\n")
    sys.exit(2)


def tmpl(name: str) -> string.Template:
    with open(os.path.join(TMPL_DIR, name), encoding="utf-8") as f:
        return string.Template(f.read())


def load_manifests(services_dir: str) -> list[dict]:
    out = []
    for fn in sorted(os.listdir(services_dir)):
        if not fn.endswith(".toml") or fn.startswith("_"):
            continue
        with open(os.path.join(services_dir, fn), "rb") as f:
            m = tomllib.load(f)
        for req in ("name", "fqdn", "port"):
            if req not in m:
                die(f"{fn}: missing required field '{req}'")
        m.setdefault("artifact", "stub")
        m.setdefault("serve_api", False)
        m.setdefault("api_mode", "shared-wal")
        m.setdefault("gated_groups", False)
        dp = m.setdefault("data_profile", {})
        dp.setdefault("canonical", [])
        dp.setdefault("disposable", [])
        dp.setdefault("blobs", [])
        dp.setdefault("blobs_immutable", [])
        if m["serve_api"] and "api_port" not in m:
            die(f"{fn}: serve_api=true requires api_port")
        if int(m["port"]) < 1024:
            die(f"{fn}: port {m['port']} must be >= 1024 (contract §5)")
        out.append(m)
    return out


def validate(manifests: list[dict]) -> None:
    # port collision across ALL service + api ports
    seen: dict[int, str] = {}
    for m in manifests:
        ports = [("port", int(m["port"]))]
        if m["serve_api"]:
            ports.append(("api_port", int(m["api_port"])))
        for kind, p in ports:
            if p in seen:
                die(f"port collision: {m['name']}.{kind}={p} already used by "
                    f"{seen[p]}")
            seen[p] = f"{m['name']}.{kind}"
    # canonical/disposable overlap
    for m in manifests:
        dp = m["data_profile"]
        overlap = set(dp["canonical"]) & set(dp["disposable"])
        if overlap:
            die(f"{m['name']}: paths classed as BOTH canonical and disposable "
                f"(overlap): {sorted(overlap)}")


def exec_start(m: dict, data_dir: str, api: bool) -> str:
    """Build the ExecStart for the tenant service or its api sidecar."""
    listen = f"127.0.0.1:{m['api_port']}" if api else f"127.0.0.1:{m['port']}"
    dp = m["data_profile"]
    if m["artifact"] == "stub":
        base = f"/usr/bin/env python3 /opt/{m['name']}/current/stub.py"
        parts = [base, f"--data-dir {data_dir}", f"--listen {listen}"]
        if api:
            parts.append("--api")
            parts.append(f"--api-mode {m['api_mode']}")
        for c in dp["canonical"]:
            parts.append(f"--canonical {c}")
        if not api:
            for d in dp["disposable"]:
                parts.append(f"--disposable {d}")
            for b in dp["blobs"]:
                parts.append(f"--blobs {b}")
            if m["gated_groups"]:
                parts.append("--gated-groups")
        return " ".join(parts)
    # real binary: manages its own layout; just point it at data-dir + listen
    binname = m["name"] if not api else f"{m['name']}-api"
    return f"/opt/{m['name']}/current/{binname} --data-dir {data_dir} --listen {listen}"


def render_service(m, out, t_service):
    data_dir = f"/var/lib/{m['name']}"
    content = t_service.substitute(
        name=m["name"], user=m["name"], data_dir=data_dir,
        exec_start=exec_start(m, data_dir, api=False),
    )
    write(out, f"systemd/{m['name']}.service", content)


def render_api(m, out, t_api, t_snap_s, t_snap_t):
    if not m["serve_api"]:
        return
    data_dir = f"/var/lib/{m['name']}"
    content = t_api.substitute(
        name=m["name"], user=m["name"], api_user=f"{m['name']}-api",
        api_mode=m["api_mode"], data_dir=data_dir,
        api_exec_start=exec_start(m, data_dir, api=True),
        cpu_quota="40%", io_class="idle",
    )
    write(out, f"systemd/{m['name']}-api.service", content)
    if m["api_mode"] == "snapshot":
        canonical0 = m["data_profile"]["canonical"][0] if m["data_profile"]["canonical"] else "state.db"
        write(out, f"systemd/{m['name']}-snapshot.service",
              t_snap_s.substitute(name=m["name"], user=m["name"],
                                  data_dir=data_dir, canonical0=canonical0))
        write(out, f"systemd/{m['name']}-snapshot.timer",
              t_snap_t.substitute(name=m["name"], snapshot_interval="5min"))


def render_caddy(m, out, t_vhost):
    write(out, f"caddy/{m['fqdn']}.caddy",
          t_vhost.substitute(fqdn=m["fqdn"], port=m["port"]))
    if m["serve_api"]:
        write(out, f"caddy/api.{m['fqdn']}.caddy",
              t_vhost.substitute(fqdn=f"api.{m['fqdn']}", port=m["api_port"]))


def blob_prefix(m, blobdir):  # D6: one bucket, per-service prefix
    return f"{m['name']}/{blobdir.rstrip('/')}"


def state_path(m, dbfile):    # D6: one bucket, per-service prefix (<svc>/<stem>)
    stem = dbfile[:-3] if dbfile.endswith(".db") else dbfile
    return f"{m['name']}/{stem}"


def render_blobs(m, out, t_rc_s, t_rc_t):
    data_dir = f"/var/lib/{m['name']}"
    immutable = set(m["data_profile"]["blobs_immutable"])
    for i, blobdir in enumerate(m["data_profile"]["blobs"]):
        is_cas = blobdir in immutable
        flagged = "" if is_cas else (
            "# FLAGGED: mutable blob dir (not content-addressed); rclone runs in\n"
            "# plain sync mode. Overwrites/deletes propagate to R2.\n")
        content = t_rc_s.substitute(
            name=m["name"], user=m["name"], data_dir=data_dir,
            blobdir=blobdir,
            blob_remote_path=f"{BUCKET}/{blob_prefix(m, blobdir)}",
            cas_desc="content-addressed" if is_cas else "mutable",
            immutable_flag="--immutable" if is_cas else "",
            flagged_comment=flagged,
        )
        write(out, f"systemd/{m['name']}-blob-{i}.service", content)
        write(out, f"systemd/{m['name']}-blob-{i}.timer",
              t_rc_t.substitute(name=m["name"], blobdir=blobdir))


def render_litestream(manifests, out):
    lines = [
        "# GENERATED by scripts/render.sh — do not hand-edit.",
        "# Credentials come from the environment (LITESTREAM_ACCESS_KEY_ID /",
        "# LITESTREAM_SECRET_ACCESS_KEY); never stored here (guardrail 4).",
        "# One bucket, per-service prefix (<service>/<db>) — see docs/BUCKETS.md.",
        "dbs:",
    ]
    for m in manifests:
        data_dir = f"/var/lib/{m['name']}"
        for db in m["data_profile"]["canonical"]:
            lines += [
                f"  - path: {data_dir}/{db}",
                "    replicas:",
                "      - type: s3",
                "        endpoint: ${LITESTREAM_ENDPOINT}",
                f"        bucket: {BUCKET}",
                f"        path: {state_path(m, db)}",
                "        sync-interval: 1s",
                "        snapshot-interval: 24h",
                "        retention: 168h",
            ]
    write(out, "litestream.yml", "\n".join(lines) + "\n")


def build_backup_map(manifests) -> dict:
    m_map = {}
    for m in manifests:
        dp = m["data_profile"]
        entry = {"canonical": [], "disposable": [], "blobs": []}
        for db in dp["canonical"]:
            entry["canonical"].append(
                {"path": db, "backup": "litestream",
                 "replica": f"{BUCKET}/{state_path(m, db)}"})
        for db in dp["disposable"]:
            entry["disposable"].append({"path": db, "backup": None})
        # snapshot-mode api adds serve/ — DERIVED, disposable, never backed up
        if m["serve_api"] and m["api_mode"] == "snapshot":
            entry["disposable"].append({"path": "serve/state.db", "backup": None})
        for blobdir in dp["blobs"]:
            entry["blobs"].append(
                {"dir": blobdir,
                 "immutable": blobdir in dp["blobs_immutable"],
                 "backup": "rclone",
                 "replica": f"{BUCKET}/{blob_prefix(m, blobdir)}"})
        m_map[m["name"]] = entry
    return m_map


def build_ports(manifests) -> dict:
    p = {}
    for m in manifests:
        p[m["name"]] = {"port": m["port"]}
        if m["serve_api"]:
            p[m["name"]]["api_port"] = m["api_port"]
    return p


def write(out_dir: str, rel: str, content: str) -> None:
    path = os.path.join(out_dir, rel)
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as f:
        f.write(content)


def render_all(services_dir, out_dir):
    manifests = load_manifests(services_dir)
    validate(manifests)
    t_service = tmpl("service.service.tmpl")
    t_api = tmpl("api.service.tmpl")
    t_snap_s = tmpl("snapshot.service.tmpl")
    t_snap_t = tmpl("snapshot.timer.tmpl")
    t_vhost = tmpl("caddy-vhost.tmpl")
    t_rc_s = tmpl("rclone.service.tmpl")
    t_rc_t = tmpl("rclone.timer.tmpl")
    for m in manifests:
        render_service(m, out_dir, t_service)
        render_api(m, out_dir, t_api, t_snap_s, t_snap_t)
        render_caddy(m, out_dir, t_vhost)
        render_blobs(m, out_dir, t_rc_s, t_rc_t)
    render_litestream(manifests, out_dir)
    write(out_dir, "backup-map.json",
          json.dumps(build_backup_map(manifests), indent=2) + "\n")
    write(out_dir, "ports.json",
          json.dumps(build_ports(manifests), indent=2) + "\n")
    # index
    files = []
    for root, _, fns in os.walk(out_dir):
        for fn in fns:
            if fn == "GENERATED.md":
                continue
            files.append(os.path.relpath(os.path.join(root, fn), out_dir))
    idx = ["# GENERATED files (build product — do not hand-edit)", "",
           f"Bucket (render var): `{BUCKET}`", "",
           f"Services: {', '.join(m['name'] for m in manifests)}", ""]
    idx += [f"- `{f}`" for f in sorted(files)]
    write(out_dir, "GENERATED.md", "\n".join(idx) + "\n")
    print(f"render: {len(manifests)} manifest(s) -> {out_dir} "
          f"({len(files)} files)")


def main():
    p = argparse.ArgumentParser(description="appview-infra generator")
    p.add_argument("--services-dir", default="services")
    p.add_argument("--out-dir", default="generated")
    a = p.parse_args()
    render_all(a.services_dir, a.out_dir)


if __name__ == "__main__":
    main()
