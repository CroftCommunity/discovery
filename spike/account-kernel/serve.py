#!/usr/bin/env python3
"""K1 spike static server. Serves the harness for all hostnames from one process.

The browser requests app-a.localhost:8080, app-b.localhost:8080, kernel.localhost:8080
(all resolve to 127.0.0.1); content is served by path, the hostname supplies the origin.
No COOP/COEP needed: K1 uses only async OPFS + IndexedDB, not SharedArrayBuffer.
"""
import http.server
import os
import socketserver

PORT = 8080
os.chdir(os.path.dirname(os.path.abspath(__file__)))


class Handler(http.server.SimpleHTTPRequestHandler):
    def end_headers(self):
        self.send_header("Cache-Control", "no-store")
        super().end_headers()

    def log_message(self, fmt, *args):
        # keep the origin (Host) in the log so a run shows which subdomain hit what
        host = self.headers.get("Host", "?")
        super().log_message("%s %s", host, fmt % args)


socketserver.TCPServer.allow_reuse_address = True
with socketserver.TCPServer(("127.0.0.1", PORT), Handler) as httpd:
    print(f"K1 harness serving on 127.0.0.1:{PORT}")
    print(f"  app A : http://app-a.localhost:{PORT}/app/")
    print(f"  app B : http://app-b.localhost:{PORT}/app/")
    print(f"  kernel: http://kernel.localhost:{PORT}/kernel/")
    httpd.serve_forever()
