# K1 spike — cross-subdomain unpartitioned storage (the account-kernel make-or-break)

Plan: `../../alpha/plans/2026-07-22-account-kernel-spike.md` (experiment K1). Thread:
`../../beta/OPEN-THREADS.md` T55. This is a throwaway measurement harness, not production code.

## What K1 asks

The account-kernel model puts one shared origin (`kernel.croft.ing`) inside every `*.croft.ing` app via an
embedded iframe, and assumes that iframe sees **one shared, unpartitioned** OPFS + IndexedDB no matter which
subdomain embeds it (because they are the same registrable site, `croft.ing`). If modern state partitioning
instead gives each embedding a **separate** partition, the one-mirror/one-outbox kernel fails as drawn.

**H1 (pass/fail):** a `kernel.<site>` iframe embedded under `app-a.<site>` and under `app-b.<site>` reads
the *same* storage. Write a nonce through the kernel from app-a; read it back through the kernel from app-b.
- **Shared value returned → H1 PASS** (same-site subdomain embeds share storage; the kernel model holds).
- **Empty / different → H1 FAIL** (storage is partitioned per top-level site; fall back to the G0
  postMessage-service topology).

## How to run (local first pass, `*.localhost`)

`*.localhost` resolves to loopback in modern browsers with no hosts edits, is a secure context (so OPFS +
`navigator.storage` work), and its registrable site is `localhost` (so `app-a.localhost`,
`app-b.localhost`, `kernel.localhost` are same-site — the local analogue of `*.croft.ing`).

1. Start the static server (serves all three hostnames from one process):
   `python3 serve.py`  (listens on 127.0.0.1:8080)
2. Open two tabs:
   - `http://app-a.localhost:8080/app/`  → click **Write nonce**
   - `http://app-b.localhost:8080/app/`  → click **Read nonce**
3. If app-b reads the nonce app-a wrote (for IndexedDB and/or OPFS), the storage is shared → H1 PASS for
   that store. The page prints the verdict; the console logs detail.

Also open `http://kernel.localhost:8080/kernel/` directly once to confirm the kernel page loads standalone.

## Caveat — why this is a first pass, not the verdict

Browsers may special-case `.localhost`, and state partitioning keys on the real registrable site (eTLD+1),
which `.localhost` is not a faithful stand-in for. A PASS here is necessary-not-sufficient; a FAIL here is a
strong early warning. The **authoritative K1 run is on real `kernel.croft.ing` + two `*.croft.ing` app
subdomains in a real browser** (Chrome / Firefox / Safari), which needs a credentialed/deploy environment.
The same harness deploys unchanged there: the app page derives the kernel origin by swapping its own first
hostname label for `kernel`, so `app-a.croft.ing` → `kernel.croft.ing` automatically.

Record outcomes in `K1-SPIKE-RESULTS.md`.

## Authoritative run (real browsers)

Chromium passed locally (`K1-SPIKE-RESULTS.md`). Two residual risks remain: **Safari** (strictest
partitioning) and **real registrable-site behaviour** (`.localhost` is not a faithful stand-in).

**Cheapest: test Safari locally (no repos, no DNS).** Safari does not auto-resolve `*.localhost`, so add a
hosts line once, then run in Safari:
```
echo '127.0.0.1 app-a.localhost app-b.localhost kernel.localhost' | sudo tee -a /etc/hosts
python3 serve.py
# Safari: open http://app-a.localhost:8080/app/ (Write), then http://app-b.localhost:8080/app/ (Read)
```

**Authoritative: real `*.croft.ing` on GitHub Pages (3 repos, single-level subdomains).** No sub-subdomains
needed - all three are the same registrable site `croft.ing`.
- `kernel-k1.croft.ing` serves `kernel/`; `appa-k1.croft.ing` and `appb-k1.croft.ing` each serve `app/`
  with an inline `window.KERNEL_ORIGIN = "https://kernel-k1.croft.ing"` (see `app/index.html`).
- Per subdomain: Porkbun `CNAME` -> `croftcommunity.github.io.`; repo Settings -> Pages custom domain +
  Enforce HTTPS (OPFS needs a secure context on a real domain).
- Run `appa-k1` (Write) then `appb-k1` (Read) in Chrome, Firefox, and Safari; log verdicts.

## Files

- `serve.py` — one static server for all hostnames (path-routed; hostname gives the origin).
- `kernel/` — the kernel iframe: a storage probe over postMessage (`whoami` / `write` / `read`) against
  both IndexedDB and OPFS.
- `app/` — a skin page that embeds the kernel iframe at `kernel.<site>` and drives the probe.
