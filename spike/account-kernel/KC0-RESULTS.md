# KC0 results — single-origin sharing + SharedWorker coordination (validates ARCH-1)

date: 2026-07-22. Question: on ONE origin, do two same-origin top-level tabs share IndexedDB + OPFS, and
does a same-origin SharedWorker coordinate them — **on WebKit**? Runner: `run-kc0-playwright.cjs`; harness:
`kc0/`. This is the premise of the pivot-to-single-origin (ARCH-1). Plan:
`../../alpha/plans/2026-07-22-account-kernel-spike.md`.

## Runs (local single origin `http://localhost:8080`, two same-origin tabs)

| Engine | IndexedDB shared | OPFS shared | SharedWorker supported | SharedWorker coordinates |
|---|---|---|---|---|
| Chromium (build 1187) | **yes ✓** | **yes ✓** | yes | **yes ✓** |
| WebKit 26.5 | **yes ✓** | tooling-errored* | yes | **yes ✓** |

*OPFS write threw the same Playwright-WebKit `UnknownError` seen in K1 — on BOTH single-origin and
cross-origin, so it is a **Playwright-WebKit environment limitation, not a Safari-OPFS-sharing signal**.
Safari has shipped OPFS since 15.2/16; a shipping-Safari confirm would settle it, but it is not required
(see below).

## Conclusion

**ARCH-1 (single origin, path-based) is validated on WebKit for the load-bearing mechanisms:** two
same-origin tabs share IndexedDB, and a same-origin SharedWorker coordinates them. This is the exact
opposite of the cross-subdomain K1 FAIL — because same-origin storage is not subject to the cross-site
partitioning that bit us. So the shared-kernel vision works on every engine **if the first-party estate
lives on one origin** rather than across subdomains.

**Storage-backend implication:** use **IndexedDB as the cross-browser blockstore baseline** (shared on
WebKit here); treat **OPFS as a Chromium-confirmed optimization** (and pending a shipping-Safari check
under KC2). The `repo-mirror` should not hard-depend on OPFS.

**Coordination implication:** a same-origin **SharedWorker** works on WebKit for single-writer / change
fan-out; **Web-Locks + BroadcastChannel** remain the same-origin fallback where a SharedWorker is
unavailable. No cross-subdomain channel is needed in ARCH-1.

## What this leaves for the architecture decision

- **ARCH-1 is the recommendation:** kernel works on all engines, SSO is free (one origin), no BFF required
  for auth. Cost: first-party pads share the origin (no inter-pad isolation → untrusted/third-party pads
  MUST go on a separate registrable domain, sandboxed, postMessage-only), and deployment becomes one origin
  serving all first-party paths.
- **KC1 (cookie->BFF SSO) is only needed if ARCH-2 (subdomains) is chosen** — moot under ARCH-1.
- **KC2 (OPFS on real Safari)** is a nice-to-have confirm; IndexedDB baseline makes it non-blocking.
- **KC3 (PDS-as-coordinator latency)** still informs realtime-vs-eventual cross-device coherence in either
  ARCH; needs the test atproto account + a live PDS.
