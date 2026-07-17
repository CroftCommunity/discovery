#!/usr/bin/env python3
"""stub.py — the shared contract stub for the appview-infra kit.

A stand-in for the real per-tenant service binary (and its own-data API
sidecar). It satisfies CONTRACT.md so the kit, the local rehearsal (D13), and
the fire-drill run with zero real binaries and zero credentials.

Stdlib only (http.server + sqlite3 + argparse): no build step, no network, runs
anywhere Python 3.11 does.

Roles:
  service mode (default):  serves /healthz and the tenant routes; owns and
                           writes the data dir per the passed data profile.
  api mode (--api):        read-only own-data sidecar (D10); opens SQLite ro.

The identity verifier is a STAND-IN for the atproto service-auth JWT verifier
proven in RUN-14 EXP-A. Same interface (token -> caller DID or None); swapped at
the seam when the real binary lands.
SPEC-DELTA[run15-stub-verifier | stand-in]
"""
from __future__ import annotations

import argparse
import json
import os
import sqlite3
import sys
import time
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from urllib.parse import urlparse, parse_qs


# --------------------------------------------------------------------------
# Verifier interface (the seam). Stub accepts "test:<did>" bearer tokens.
# --------------------------------------------------------------------------
class Verifier:
    """token -> caller DID, or None if unverifiable."""

    def verify(self, authorization: str | None) -> str | None:
        raise NotImplementedError


class StubVerifier(Verifier):
    """SPEC-DELTA[run15-stub-verifier | stand-in] for atproto service-auth."""

    def verify(self, authorization: str | None) -> str | None:
        if not authorization or not authorization.startswith("Bearer "):
            return None
        tok = authorization[len("Bearer "):].strip()
        # stand-in token shape: "test:<did>"
        if not tok.startswith("test:"):
            return None
        did = tok[len("test:"):]
        if not did.startswith("did:"):
            return None
        return did


# --------------------------------------------------------------------------
# Data profile: create/own the declared files under the data dir (§1.2).
# --------------------------------------------------------------------------
def init_data_profile(data_dir: str, canonical, disposable, blobs) -> None:
    os.makedirs(data_dir, exist_ok=True)
    for rel in list(canonical) + list(disposable):
        path = os.path.join(data_dir, rel)
        os.makedirs(os.path.dirname(path) or data_dir, exist_ok=True)
        conn = sqlite3.connect(path)
        try:
            conn.execute("PRAGMA journal_mode=WAL;")  # WAL: readers + one writer
            conn.commit()
        finally:
            conn.close()
    for rel in blobs:
        os.makedirs(os.path.join(data_dir, rel), exist_ok=True)
    # Own-data schema lives in the first canonical db (observation/grant-born).
    if canonical:
        conn = sqlite3.connect(os.path.join(data_dir, canonical[0]))
        try:
            conn.execute(
                "CREATE TABLE IF NOT EXISTS my_rows("
                "id INTEGER PRIMARY KEY, subject_did TEXT NOT NULL, "
                "payload TEXT, ts INTEGER)")
            conn.commit()
        finally:
            conn.close()


# --------------------------------------------------------------------------
# GroupStore (§1.4) — the fork-agnostic seam. Both write-path variants (D11)
# implement roster()/content(); the stub backs it with canonical tables in
# state.db seeded from a fixture. Roster + content are canonical (§1.2).
# --------------------------------------------------------------------------
class GroupStore:
    def roster(self, group):        # -> list[str] of member DIDs
        raise NotImplementedError

    def content(self, group, cursor):   # -> list[(seq, body)] with seq > cursor
        raise NotImplementedError


class SqliteGroupStore(GroupStore):
    def __init__(self, db_path):
        self.db_path = db_path

    def _conn(self):
        return sqlite3.connect(self.db_path)

    def roster(self, group):
        conn = self._conn()
        try:
            return [r[0] for r in conn.execute(
                "SELECT member_did FROM roster WHERE group_id=?", (group,))]
        finally:
            conn.close()

    def content(self, group, cursor):
        conn = self._conn()
        try:
            return conn.execute(
                "SELECT seq, body FROM group_content WHERE group_id=? AND seq>? "
                "ORDER BY seq", (group, cursor)).fetchall()
        finally:
            conn.close()

    def remove_member(self, group, did):
        conn = self._conn()
        try:
            conn.execute("DELETE FROM roster WHERE group_id=? AND member_did=?",
                         (group, did))
            conn.commit()
        finally:
            conn.close()


def seed_groups(db_path: str, fixture_path: str) -> None:
    """Create the canonical roster/group_content tables and seed from a fixture
    (stub/rehearsal input; in production these are grant-born)."""
    with open(fixture_path, encoding="utf-8") as f:
        data = json.load(f)
    conn = sqlite3.connect(db_path)
    try:
        conn.execute("CREATE TABLE IF NOT EXISTS roster("
                     "group_id TEXT, member_did TEXT, "
                     "PRIMARY KEY(group_id, member_did))")
        conn.execute("CREATE TABLE IF NOT EXISTS group_content("
                     "group_id TEXT, seq INTEGER, body TEXT, "
                     "PRIMARY KEY(group_id, seq))")
        for gid, g in data.get("groups", {}).items():
            for did in g.get("roster", []):
                conn.execute("INSERT OR IGNORE INTO roster VALUES(?,?)", (gid, did))
            for row in g.get("content", []):
                conn.execute("INSERT OR IGNORE INTO group_content VALUES(?,?,?)",
                             (gid, row["seq"], row["body"]))
        conn.commit()
    finally:
        conn.close()


# --------------------------------------------------------------------------
# HTTP handler with a small route table. Routes are registered by role.
# --------------------------------------------------------------------------
class App:
    def __init__(self, cfg):
        self.cfg = cfg
        self.verifier = StubVerifier()
        self.routes = {}  # (method, path) -> handler(app, req, qs, caller)
        self.authed = set()  # paths requiring a verified caller
        self.group_store = None
        if not cfg.api and cfg.gated_groups:
            self.group_store = SqliteGroupStore(
                os.path.join(cfg.data_dir, cfg.canonical[0]))
        self.register_core()
        if cfg.api:
            self.register_api()
        else:
            self.register_service()

    # -- route registration --------------------------------------------------
    def route(self, method, path, fn, authed=False):
        self.routes[(method, path)] = fn
        if authed:
            self.authed.add((method, path))

    def register_core(self):
        self.route("GET", "/healthz", self._healthz)

    def register_service(self):
        self.route("GET", "/xrpc/app.stub.echo", self._echo, authed=True)
        # own-data WRITE path (the service, not the api, writes canonical state)
        self.route("POST", "/xrpc/app.stub.recordMyRow", self._record, authed=True)
        if self.group_store is not None:
            # gated large-group serving (§1.4). Verifier is stubbed here; real
            # atproto service-auth verification is RUN-14 EXP-A, swapped in with
            # the real binary.
            self.route("GET", "/xrpc/app.stub.getGroupContent",
                       self._group_content, authed=True)
            self.route("POST", "/xrpc/app.stub.removeRosterMember",
                       self._remove_member, authed=True)

    def register_api(self):
        # own-data READ path (§1.3): self-scoping, paginated export, timeout.
        self.route("GET", "/xrpc/app.stub.getMyRows", self._get_my_rows, authed=True)
        self.route("GET", "/xrpc/app.stub.export", self._export, authed=True)
        self.route("GET", "/xrpc/app.stub.slowQuery", self._slow, authed=True)

    # -- core route handlers -------------------------------------------------
    def _healthz(self, req, qs, caller):
        self.text(req, 200, "ok")

    def _echo(self, req, qs, caller):
        msg = (qs.get("msg") or [""])[0]
        self.json(req, 200, {"caller": caller, "msg": msg})

    # -- own-data write (service) --------------------------------------------
    def _canonical_db(self):
        return os.path.join(self.cfg.data_dir, self.cfg.canonical[0])

    def _record(self, req, qs, caller):
        payload = (qs.get("payload") or [""])[0]
        conn = sqlite3.connect(self._canonical_db())
        try:
            cur = conn.execute(
                "INSERT INTO my_rows(subject_did, payload, ts) "
                "VALUES(?, ?, ?)", (caller, payload, int(time.time())))
            conn.commit()
            rowid = cur.lastrowid
        finally:
            conn.close()
        self.json(req, 200, {"id": rowid, "subject_did": caller})

    # -- gated large-group serving (§1.4) ------------------------------------
    def _group_content(self, req, qs, caller):
        group = (qs.get("group") or [""])[0]
        try:
            cursor = int((qs.get("cursor") or ["0"])[0])
        except ValueError:
            cursor = 0
        # roster gate: verified caller must be a current member. Non-member and
        # nonexistent-group are one indistinguishable 403 (no existence leak).
        if caller not in self.group_store.roster(group):
            self.text(req, 403, "forbidden")
            return
        rows = self.group_store.content(group, cursor)
        # The server READS by design in this tier — the honest posture is the
        # feature; the offering-vs-reading distinction (EXP-B) does not apply.
        self.json(req, 200, {"group": group,
                             "content": [{"seq": r[0], "body": r[1]} for r in rows]})

    def _remove_member(self, req, qs, caller):
        # A test/admin affordance; real admission is governed (RUN-14 A2 seam).
        group = (qs.get("group") or [""])[0]
        did = (qs.get("did") or [""])[0]
        self.group_store.remove_member(group, did)
        self.json(req, 200, {"removed": did, "group": group})

    # -- own-data read (api, read-only) --------------------------------------
    def _ro_db_path(self):
        if self.cfg.api_mode == "snapshot":
            return os.path.join(self.cfg.data_dir, "serve", "state.db")
        return self._canonical_db()

    def _ro_conn(self):
        # OS-incapable of writing is provided by the unit (ReadOnlyPaths); here
        # the connection itself is opened read-only (mode=ro) too.
        uri = f"file:{self._ro_db_path()}?mode=ro"
        return sqlite3.connect(uri, uri=True, timeout=1.0)

    def _query(self, conn, sql, params=()):
        """Run a read with a per-statement wall-clock timeout (progress handler
        aborts a query that outlives the budget, so a slow read cannot pin the
        WAL). Raises sqlite3.OperationalError on timeout."""
        budget = self.cfg.stmt_timeout_ms / 1000.0
        start = time.monotonic()
        conn.set_progress_handler(
            lambda: 1 if (time.monotonic() - start) > budget else 0, 2000)
        try:
            return conn.execute(sql, params).fetchall()
        finally:
            conn.set_progress_handler(None, 0)

    def _get_my_rows(self, req, qs, caller):
        conn = self._ro_conn()
        try:
            rows = self._query(
                conn,
                "SELECT id, payload, ts FROM my_rows WHERE subject_did = ? "
                "ORDER BY id", (caller,))
        finally:
            conn.close()
        self.json(req, 200, {"subject_did": caller,
                             "rows": [{"id": r[0], "payload": r[1], "ts": r[2]}
                                      for r in rows]})

    def _export(self, req, qs, caller):
        try:
            cursor = int((qs.get("cursor") or ["0"])[0])
        except ValueError:
            cursor = 0
        limit = self.cfg.page_size
        conn = self._ro_conn()
        try:
            rows = self._query(
                conn,
                "SELECT id, payload FROM my_rows WHERE subject_did = ? "
                "AND id > ? ORDER BY id LIMIT ?", (caller, cursor, limit))
        finally:
            conn.close()
        next_cursor = rows[-1][0] if len(rows) == limit else None
        self.json(req, 200, {
            "subject_did": caller,
            "rows": [{"id": r[0], "payload": r[1]} for r in rows],
            "next_cursor": next_cursor,
        })

    def _slow(self, req, qs, caller):
        conn = self._ro_conn()
        try:
            self._query(conn,
                        "WITH RECURSIVE c(x) AS (SELECT 1 UNION ALL "
                        "SELECT x + 1 FROM c) SELECT count(*) FROM c")
            self.json(req, 200, {"ok": True})  # should never reach
        except sqlite3.OperationalError:
            self.text(req, 503, "statement timeout")
        finally:
            conn.close()

    # -- response helpers ----------------------------------------------------
    @staticmethod
    def text(req, code, body):
        data = body.encode()
        req.send_response(code)
        req.send_header("Content-Type", "text/plain")
        req.send_header("Content-Length", str(len(data)))
        req.end_headers()
        req.wfile.write(data)

    @staticmethod
    def json(req, code, obj):
        data = json.dumps(obj).encode()
        req.send_response(code)
        req.send_header("Content-Type", "application/json")
        req.send_header("Content-Length", str(len(data)))
        req.end_headers()
        req.wfile.write(data)


class Handler(BaseHTTPRequestHandler):
    app: App = None  # set on the server class

    def log_message(self, *a):  # quiet
        pass

    def _dispatch(self, method):
        parsed = urlparse(self.path)
        qs = parse_qs(parsed.query)
        key = (method, parsed.path)
        fn = self.app.routes.get(key)
        if fn is None:
            App.text(self, 404, "not found")
            return
        caller = None
        if key in self.app.authed:
            caller = self.app.verifier.verify(self.headers.get("Authorization"))
            if caller is None:
                App.text(self, 401, "unauthorized")
                return
        try:
            fn(self, qs, caller)
        except BrokenPipeError:
            pass
        except Exception as e:  # noqa: BLE001 — stub: surface as 500
            App.text(self, 500, f"error: {e}")

    def do_GET(self):
        self._dispatch("GET")

    def do_POST(self):
        self._dispatch("POST")


def serve(cfg):
    if not cfg.api:
        init_data_profile(cfg.data_dir, cfg.canonical, cfg.disposable, cfg.blobs)
        if cfg.gated_groups and cfg.group_fixture:
            seed_groups(os.path.join(cfg.data_dir, cfg.canonical[0]),
                        cfg.group_fixture)
    app = App(cfg)
    host, _, port = cfg.listen.rpartition(":")
    server = ThreadingHTTPServer((host, int(port)), Handler)
    server.RequestHandlerClass.app = app
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        pass


def parse_args(argv):
    p = argparse.ArgumentParser(description="appview-infra contract stub")
    p.add_argument("--data-dir", required=True)
    p.add_argument("--listen", required=True, help="host:port, port >= 1024")
    p.add_argument("--canonical", action="append", default=[])
    p.add_argument("--disposable", action="append", default=[])
    p.add_argument("--blobs", action="append", default=[])
    p.add_argument("--api", action="store_true", help="own-data API sidecar mode")
    p.add_argument("--api-mode", choices=["shared-wal", "snapshot"],
                   default="shared-wal")
    p.add_argument("--stmt-timeout-ms", type=int, default=250)
    p.add_argument("--page-size", type=int, default=100)
    p.add_argument("--gated-groups", action="store_true",
                   help="enable roster-gated large-group serving (§1.4)")
    p.add_argument("--group-fixture", default=None,
                   help="stub/rehearsal roster+content fixture (JSON)")
    cfg = p.parse_args(argv)
    _, _, port = cfg.listen.rpartition(":")
    if int(port) < 1024:
        p.error("--listen port must be >= 1024 (contract §5)")
    if os.geteuid() == 0 and os.environ.get("STUB_ALLOW_ROOT") != "1":
        # contract §4: no root. Allow override only for constrained test harness.
        sys.stderr.write("refusing to run as root (contract §4); "
                         "set STUB_ALLOW_ROOT=1 only in a sandboxed test\n")
        sys.exit(3)
    return cfg


def main(argv=None):
    cfg = parse_args(sys.argv[1:] if argv is None else argv)
    serve(cfg)


if __name__ == "__main__":
    main()
